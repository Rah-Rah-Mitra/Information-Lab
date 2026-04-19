---
type: content
title: "Markov Decision Processes and POMDPs"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:06:32.495477100+00:00
summary: "Markov Decision Processes (MDPs) and Partially Observable Markov Decision Processes (POMDPs) provide mathematical frameworks for probabilistic planning and control under uncertainty. MDPs assume full state observability, while POMDPs extend this to cases where the state is only estimated via noisy measurements. These frameworks allow robots to optimize cumulative payoffs through value iteration and belief-space planning."
tags:
  - robotics
  - probabilistic-planning
  - decision-processes
  - control-theory
  - stochastic-systems
entities:
  - "[[Markov Decision Processes]]"
  - "[[Partially Observable Markov Decision Processes]]"
  - "[[Value Iteration]]"
  - "[[Belief Space]]"
  - "[[Control Policy]]"
  - "[[Payoff Function]]"
  - "[[Bellman Equation]]"
  - "[[Discount Factor]]"
relationships:
  - source: "Markov Decision Processes"
    target: "Value Iteration"
    description: "solved using"
  - source: "Partially Observable Markov Decision Processes"
    target: "Value Iteration"
    description: "generalises to"
  - source: "Markov Decision Processes"
    target: "Control Policy"
    description: "produces"
  - source: "Partially Observable Markov Decision Processes"
    target: "Belief Space"
    description: "operates in"
  - source: "Value Iteration"
    target: "Bellman Equation"
    description: "satisfies"
  - source: "Control Policy"
    target: "Payoff Function"
    description: "maximises"
  - source: "Value Iteration"
    target: "Discount Factor"
    description: "uses"
---

# Markov Decision Processes and POMDPs

*Markov Decision Processes (MDPs) and Partially Observable Markov Decision Processes (POMDPs) provide mathematical frameworks for probabilistic planning and control under uncertainty. MDPs assume full state observability, while POMDPs extend this to cases where the state is only estimated via noisy measurements. These frameworks allow robots to optimize cumulative payoffs through value iteration and belief-space planning.*

This note explores the mathematical frameworks for probabilistic planning and control, specifically focusing on Markov Decision Processes and their partially observable extensions.

## Concept
In robotics, action selection must account for uncertainty in both action effects (stochasticity) and perception (measurement noise). [[Markov Decision Processes]] (MDPs) model environments where the state is fully observable, meaning the robot knows its exact state at all times. The goal is to to find an optimal [[Control Policy]] ♀, which is a mapping from states to actions that maximizes the expected cumulative [[Payoff Function]] over a planning horizon.

In the more realistic case of [[Partially Observable Markov Decision Processes]] (POMDPs), the state is not directly observable. Instead, the robot must maintain a [[Belief Space]], which is the space of all posterior probability distributions over possible world states. Planning in a POMDP occurs in this belief space, where the robot actively gathers information to reduce uncertainty.

## Mathematical Foundations

### MDP Framework
An MDP is defined by its state space, action space, and transition model. The objective is to maximize the expected cumulative payoff, often adjusted by a [[Discount Factor]] ∓ to ensure convergence in infinite-horizon problems. The optimal value function for an MDP satisfies the [[Bellman Equation]]:

$$ V(x) = r(x, u) + γ 	ext{max}_u 	ext{E} [V(x')] $$

This equation allows for the [[Value Iteration]] algorithm to recursively approximate the optimal value function by performing a "backup step" that propagates information through the state space.

### POMDP Framework
In a POMDP, the robot operates on a belief state $b$. The value function is computed over the belief space rather than the state space. For finite worlds, the optimal value function is continuous, piecewise linear, and convex. This property allows the value function to be represented as a maximum of a set of linear functions (alpha-vectors):

$$ V(b) = 	ext{max}_i [v_i 	ext{.} b] $$

where $v_i$ represents the parameters of the $i$-th linear function. The complexity of POMDPs arises from the "explosion" of these linear pieces during the update step, necessitating pruning algorithms to maintain computational tractability.

## Relationships
- [[Markov Decision Processes]] are solved using [[Value Iteration]].
- [[Partially Observable Markov Decision Processes]] generalise [[Markov Decision Processes]] to include measurement uncertainty.
- [[Value Iteration]] satisfies the [[Bellman Equation]].
- [[Control Policy]] is designed to maximise the [[Payoff Function]].
- [[Partially Observable Markov Decision Processes]] operate within a [[Belief Space]].
