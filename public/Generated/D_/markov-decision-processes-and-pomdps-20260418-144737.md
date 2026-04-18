---
type: content
title: "Markov Decision Processes and POMDPs"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:47:37.295084600+00:00
summary: "An overview of probabilistic planning and control using Markov Decision Processes (MDPs) and Partially Observable Markov Decision Processes (POMDPs)."
tags:
  - robotics
  - probabilistic-planning
  - control-theory
  - mdp
  - pomdp
entities:
  - "[[Markov Decision Process]]"
  - "[[Partially Observable Markov Decision Process]]"
  - "[[Value Iteration]]"
  - "[[Bellman Equation]]"
  - "[[Control Policy]]"
  - "[[Payoff Function]]"
  - "[[Belief Space]]"
  - "[[Discount Factor]]"
  - "[[Planning Horizon]]"
  - "[[Backup Step]]"
relationships:
  - source: "Markov Decision Process"
    target: "Value Iteration"
    description: "is solved using"
  - source: "Markov Decision Process"
    target: "Control Policy"
    description: "generates a"
  - source: "Partially Observable Markov Decision Process"
    target: "Markov Decision Process"
    description: "generalizes"
  - source: "Partially Observable Markov Decision Process"
    target: "Belief Space"
    description: "operates within"
  - source: "Value Iteration"
    target: "Bellman Equation"
    description: "is based on"
  - source: "Value Iteration"
    target: "Backup Step"
    description: "utilizes a"
  - source: "Control Policy"
    target: "Payoff Function"
    description: "optimizes the"
  - source: "Value Iteration"
    target: "Planning Horizon"
    description: "depends on the"
  - source: "Value Iteration"
    target: "Discount Factor"
    description: "incorporates a"
---

# Markov Decision Processes and POMDPs

*An overview of probabilistic planning and control using Markov Decision Processes (MDPs) and Partially Observable Markov Decision Processes (POMDPs).*

Probabilistic planning and control focus on selecting optimal actions under uncertainty, primarily through the frameworks of [[Markov Decision Process]] (MDP) and [[Partially Observable Markov Decision Process]] (POMDP).

## Markov Decision Processes

A [[Markov Decision Process]] is a mathematical framework for modeling decision-making where outcomes are partly random and partly under the control of a decision-maker. In MDPs, the state of the environment is assumed to be fully observable.

### Value Iteration and the Bellman Equation

To find the optimal [[Control Policy]]—a mapping from states to actions—the [[Value Iteration]] algorithm is used. This process recursively calculates the utility of actions relative to a [[Payoff Function]], which balances goal achievement against operational costs. The equilibrium state of this recursion is defined by the [[Bellman Equation]].

### Key Parameters

- **[[Planning Horizon]]**: The time window over which the robot optimizes its cumulative payoff. This can be greedy (T=1), finite, or infinite.
- **[[Discount Factor]]**: A parameter $\gamma \in [0, 1]$ used in infinite-horizon cases to ensure the cumulative payoff remains finite by exponentially discounting future rewards.

## Partially Observable Markov Decision Processes

A [[Partially Observable Markov Decision Process]] generalizes the MDP by removing the assumption of full observability. In a POMDP, the robot cannot sense the state directly and must instead maintain a posterior distribution over possible states, known as the [[Belief Space]].

### Planning in Belief Space

Value iteration in a POMDP is performed over the [[Belief Space]] rather than the state space. The resulting value function is continuous, piecewise linear, and convex. A critical component of the POMDP update is the [[Backup Step]], which projects the value function backward in time to determine the optimal action based on current knowledge.

## Relationships
- [[Markov Decision Process]] is solved using [[Value Iteration]].
- [[Markov Decision Process]] generates a [[Control Policy]].
- [[Partially Observable Markov Decision Process]] generalizes [[Markov Decision Process]].
- [[Partially Observable Markov Decision Process]] operates within [[Belief Space]].
- [[Value Iteration]] is based on the [[Bellman Equation]].
- [[Value Iteration]] utilizes a [[Backup Step]].
- [[Control Policy]] optimizes the [[Payoff Function]].
- [[Value Iteration]] depends on the [[Planning Horizon]].
- [[Value Iteration]] incorporates a [[Discount Factor]].
