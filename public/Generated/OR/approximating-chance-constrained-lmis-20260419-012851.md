---
type: content
title: "Approximating Chance Constrained LMIs"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:28:51.979759500+00:00
summary: "This note explores safe, tractable approximations for chance-constrained Linear Matrix Inequalities (LMIs) under various random perturbation models. It details approximation schemes for bounded and Gaussian noise, including the use of the Randomized r-procedure for post-optimality verification. The methods aim to balance computational tractability with reliability in uncertain optimization problems."
tags:
  - optimization
  - stochastic-optimization
  - linear-matrix-inequalities
  - robust-control
entities:
  - "[[Chance Constrained LMI]]"
  - "[[Random Perturbations]]"
  - "[[Gaussian Case]]"
  - "[[Bounded Case]]"
  - "[[Randomized r-procedure]]"
  - "[[Conic Quadratic Inequality]]"
  - "[[Schur Complement Lemma]]"
  - "[[Talagrand Inequality]]"
  - "[[Gaussian Majorization]]"
  - "[[Linear Matrix Inequality]]"
relationships:
  - source: "Chance Constrained LMI"
    target: "Random Perturbations"
    description: "subject to"
  - source: "Chance Constrained LMI"
    target: "Linear Matrix Inequality"
    description: "is a type of"
  - source: "Chance Constrained LMI"
    target: "Gaussian Case"
    description: "approximated in"
  - source: "Chance Constrained LMI"
    target: "Bounded Case"
    description: "approximated in"
  - source: "Chance Constrained LMI"
    target: "Randomized r-procedure"
    description: "verified by"
  - source: "Randomized r-procedure"
    target: "Random Perturbations"
    description: "handles"
  - source: "Schur Complement Lemma"
    target: "Chance Constrained LMI"
    description: "used to reformulate"
  - source: "Gaussian Majorization"
    target: "Gaussian Case"
    description: "extends to"
  - source: "Talagrand Inequality"
    target: "Random Perturbations"
    description: "provides bounds for"
---

# Approximating Chance Constrained LMIs

*This note explores safe, tractable approximations for chance-constrained Linear Matrix Inequalities (LMIs) under various random perturbation models. It details approximation schemes for bounded and Gaussian noise, including the use of the Randomized r-procedure for post-optimality verification. The methods aim to balance computational tractability with reliability in uncertain optimization problems.*

This note discusses the derivation and application of safe, tractable approximations for [[Chance Constrained LMI]] problems where constraints are subject to [[Random Perturbations]].

## Concept
A [[Chance Constrained LMI]] is a semidefinite program where the constraints must hold with a specified probability (e.g., $\text{Prob} \{ A(y) + \zeta A(y) \preceq 0 \} \geq 1 - \epsilon$). The goal is to approximate these probabilistic constraints with deterministic, convex constraints that are a "safe" approximation, meaning they guarantee the feasibility of the original chance constraint with high probability.

Depending on the distribution of the random variables $\zeta$, two primary cases are considered:

1. **[[Bounded Case]]** (Assumption A.I): The random variables are bounded, e.g., $|\zeta_i| \leq 1$.
2. **[[Gaussian Case]]** (Assumption A.II): The random variables follow a Gaussian distribution $\zeta 	ext{ i.s.d. } N(0, I)$.

For the [[Bounded Case]], the text describes an approximation scheme using the Schur Complement Lemma and specific LMI-representable conditions. For the [[Gaussian Case]], the approximation involves error functions and inverse error functions to handle the tail behavior of the random variables.

## Approximation Schemes

### Randomized r-procedure
The The [[Randomized r-procedure]] is a simulation-based method used for post-optimality analysis to verify if a solution $y$ obtained from an approximation is indeed feasible for the original chance constraint. It uses a sample size $N$ and a reliability tolerance $\delta$ to infer a a lower bound on the feasibility radius $ho$.

### Gaussian Majorization
The [[Gaussian Majorization]] technique is used when the random variables do not strictly fit the bounded or Gaussian assumptions but are symmetrically and unimodally distributed. By finding a Gaussian majorant, one can treat the problem as a case of the [[Gaussian Case]].

## Mathematical Foundations

Key tools used in the derivation include the [[Schur Complement Lemma]], which is used to transform constraints into LMI form, and the [[Talagrand Inequality]], which provides concentration bounds for the random matrices.

$$ 	ext{Prob} \{ \| \zeta B \| \leq \Upsilon \} \geq \chi $$

This relation is a central part of the the following theorems regarding sufficient conditions for the validity of the chance constraint.

## Relationships
- [[Chance Constrained LMI]] subject to [[Random Perturbations]]
- [[Chance Constrained LMI]] approximated in [[Bounded Case]] and [[Gaussian Case]]
- [[Randomized r-procedure]] verifies [[Chance Constrained LMI]]
- [[Gaussian Majorization]] extends to the [[Gaussian Case]]
- [[Schur Complement Lemma]] used to reformulate [[Chance Constrained LMI]]
- [[Talagrand Inequality]] provides bounds for [[Random Perturbations]]
