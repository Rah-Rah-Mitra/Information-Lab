---
type: content
title: "Safe Tractable Approximations of Chance Constraints"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:04:59.965844700+00:00
summary: "This note explores methods for approximating stochastic chance constraints with computationally tractable convex constraints. It focuses on translating probabilistic knowledge into 'uncertain-but-bounded' models using safe approximations that guarantee feasibility. The approach is particularly useful when only partial information about a distribution is known."
tags:
  - operations-research
  - stochastic-optimization
  - chance-constraints
  - robust-optimization
entities:
  - "[[Chance Constraints]]"
  - "[[Robust Counterpart]]"
  - "[[Safe Convex Approximation]]"
  - "[[Uncertainty Set]]"
  - "[[Ambiguous Chance Constraints]]"
  - "[[Gaussian Random Vector]]"
  - "[[Budgeted Uncertainty]]"
  - "[[Conic Quadratic Constraint]]"
relationships:
  - source: "Chance Constraints"
    target: "Robust Counterpart"
    description: "are approximated by"
  - source: "Safe Convex Approximation"
    target: "Chance Constraints"
    description: "guarantees feasibility for"
  - source: "Robust Counterpart"
    target: "Uncertainty Set"
    description: "depends on"
  - source: "Ambiguous Chance Constraints"
    target: "Chance Constraints"
    description: "is a generalization of"
  - source: "Budgeted Uncertainty"
    target: "Robust Counterpart"
    description: "leads to a tractable"
  - source: "Conic Quadratic Constraint"
    target: "Robust Counterpart"
    description: "forms a type of"
---

# Safe Tractable Approximations of Chance Constraints

*This note explores methods for approximating stochastic chance constraints with computationally tractable convex constraints. It focuses on translating probabilistic knowledge into 'uncertain-but-bounded' models using safe approximations that guarantee feasibility. The approach is particularly useful when only partial information about a distribution is known.*

This note details the construction of safe, tractable approximations for [[Chance Constraints]], which are constraints that must be satisfied with a certain probability level $1-\epsilon$. 

## Concept
In stochastic optimization, a [[Chance Constraint]] is expressed as: 

$$ \text{Prob}_{\zeta \sim P} (a^T x + 	ext{perturbation} \leq b) \geq 1-\epsilon $$

Directly solving these is often NP-hard or non-convex. A [[Safe Convex Approximation]] is a system of convex constraints that ensures any solution satisfying the approximation is also feasible for the original chance constraint. 

### Uncertainty Models

1. **Uncertain-but-bounded**: Data is assumed to belong to a deterministic [[Uncertainty Set]] $Z$. The [[Robust Counterpart]] (RC) is the set of solutions feasible for all $\zeta \in Z$. 

2. **Stochastic**: Data follows a distribution $P$. If $P$ is unknown but belongs to a family $\mathcal{P}$, we face [[Ambiguous Chance Constraints]].

### Specific Approximations

- **[[Budgeted Uncertainty]]**: Uses a budget $\gamma$ to limit the number of simultaneous deviations. This leads to a tractable linear programming reformulation.
- **[[Conic Quadratic Constraint]]**: Often arises when using ellipsoidal or ball-shaped uncertainty sets, providing a probabilistic guarantee via a parameter $\Omega$.

## Relationships
- [[Chance Constraints]] are approximated by [[Robust Counterpart]]
- [[Safe Convex Approximation]] guarantees feasibility for [[Chance Constraints]]
- [[Robust Counterpart]] depends on [[Uncertainty Set]]
- [[Ambiguous Chance Constraints]] is a generalization of [[Chance Constraints]]
- [[Budgeted Uncertainty]] leads to a tractable [[Robust Counterpart]]
- [[Conic Quadratic Constraint]] forms a type of [[Robust Counterpart]]
