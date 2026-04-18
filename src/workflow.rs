//! Workflow control primitives — ADK-style Sequential / Parallel / Loop
//! patterns implemented natively in tokio. No framework dependency.
//!
//! A [`Workflow`] is any async function-like object that transforms an input
//! into an output under an [`AgentCtx`]. The three combinators in this module
//! compose workflows deterministically:
//!
//! * [`Sequential`] — run `a`, feed its output into `b`.
//! * [`Parallel`]   — run `a` and `b` concurrently, return both results.
//! * [`Loop`]       — iterate `step` until the [`Terminator`] says stop or
//!   `max_iters` is reached. State is threaded through.
//!
//! The orchestrator uses these to express the pipeline's control-flow shape
//! (Sequential for ingest, Parallel for research tick, Loop inside the
//! BridgeFinder's propose → search → critique cycle) without pulling in a
//! full agent framework.

use async_trait::async_trait;

use crate::{agents::AgentCtx, error::AppResult};

#[async_trait]
#[allow(dead_code)]
pub trait Workflow: Send + Sync {
    type In: Send;
    type Out: Send;
    async fn run(&self, ctx: &AgentCtx, input: Self::In) -> AppResult<Self::Out>;
}

/// Deterministic "run A, then run B with A's output".
#[allow(dead_code)]
pub struct Sequential<A, B>(pub A, pub B);

#[async_trait]
impl<A, B> Workflow for Sequential<A, B>
where
    A: Workflow,
    B: Workflow<In = A::Out>,
    A::Out: Send,
{
    type In = A::In;
    type Out = B::Out;

    async fn run(&self, ctx: &AgentCtx, input: Self::In) -> AppResult<Self::Out> {
        let mid = self.0.run(ctx, input).await?;
        self.1.run(ctx, mid).await
    }
}

/// Fan out to two workflows concurrently under `tokio::join!`. Each arm
/// receives a clone of the input. Fair scheduling is left to the admission
/// layer (`Limiter::admit`) — nothing here is semaphore-aware, so both arms
/// would otherwise race the global governor.
#[allow(dead_code)]
pub struct Parallel<A, B>(pub A, pub B);

#[async_trait]
impl<A, B> Workflow for Parallel<A, B>
where
    A: Workflow,
    B: Workflow<In = A::In>,
    A::In: Clone,
{
    type In = A::In;
    type Out = (A::Out, B::Out);

    async fn run(&self, ctx: &AgentCtx, input: Self::In) -> AppResult<Self::Out> {
        let input_b = input.clone();
        let (ra, rb) = tokio::join!(self.0.run(ctx, input), self.1.run(ctx, input_b));
        Ok((ra?, rb?))
    }
}

/// Iterate `step` up to `max_iters` times, feeding the previous state into
/// the next iteration. Stops early when `terminator` returns true OR the
/// step returns `ControlFlow::Stop`.
#[allow(dead_code)]
pub struct Loop<W, T> {
    pub step: W,
    pub terminator: T,
    pub max_iters: u8,
}

#[allow(dead_code)]
pub trait Terminator<S>: Send + Sync {
    fn should_stop(&self, iters: u8, state: &S) -> bool;
}

#[async_trait]
impl<W, T, S> Workflow for Loop<W, T>
where
    W: Workflow<In = (u8, S), Out = S>,
    T: Terminator<S>,
    S: Send + Clone + 'static,
{
    type In = S;
    type Out = (u8, S);

    async fn run(&self, ctx: &AgentCtx, input: Self::In) -> AppResult<Self::Out> {
        let mut state = input;
        let mut iters = 0u8;
        while iters < self.max_iters {
            iters += 1;
            state = self.step.run(ctx, (iters, state)).await?;
            if self.terminator.should_stop(iters, &state) {
                break;
            }
        }
        Ok((iters, state))
    }
}
