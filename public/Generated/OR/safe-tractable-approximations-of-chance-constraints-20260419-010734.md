---
type: content
title: "Safe Tractable Approximations of Chance Constraints"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:07:34.162897800+00:00
summary: "This note explores methods for constructing safe, computationally tractable convex approximations for scalar chance constraints under various uncertainty models. It details Lagrange relaxation techniques, Bernstein-based bounds, and robust counterpart approaches for both purely stochastic and combined uncertainty scenarios. These methods allow for decision-makers to guarantee service levels while maintaining computational efficiency."
tags:
  - operations-research
  - optimization-under-uncertainty
  - chance-constraints
  - convex-optimization
  - robust-optimization
entities:
  - "[[Chance Constraints]]"
  - "[[Lagrange Relaxation]]"
  - "[[Bernstein Approximation]]"
  - "[[Robust Counterpart]]"
  - "[[Ambiguous Chance Constraint]]"
  - "[[Convex Optimization]]"
  - "[[Convex Approximation]]"
  - "[[Scalar Chance Constraint]]"
  - "[[Semidefinite Relaxation]]"
relationships:
  - source: "Lagrange Relaxation"
    target: "Chance Constraints"
    description: "provides approximations for"
  - source: "Bernstein Approximation"
    target: "Chance Constraints"
    description: "bounds"
  - source: "Robust Counterpart"
    target: "Chance Constraints"
    description: "approximates"
  - source: "Convex Optimization"
    target: "Chance Constraints"
    description: "enables tractable"
  - source: "Lagrange Relaxation"
    target: "Convex Approximation"
    description: "generates"
  - source: "Ambiguous Chance Constraint"
    target: "Chance Constraints"
    description: "is a type of"
  - source: "Semidefinite Relaxation"
    target: "Chance Constraints"
    description: "is a specific form of"
---

# Safe Tractable Approximations of Chance Constraints

*This note explores methods for constructing safe, computationally tractable convex approximations for scalar chance constraints under various uncertainty models. It details Lagrange relaxation techniques, Bernstein-based bounds, and robust counterpart approaches for both purely stochastic and combined uncertainty scenarios. These methods allow for decision-makers to guarantee service levels while maintaining computational efficiency.*

This note details the derivation and application of safe, tractable convex approximations for [[Chance Constraints]], which are constraints requiring a certain probability of satisfaction. 

## Concept
In many optimization problems, a constraint of the form $\text{Prob}(\text{condition}) \geq 1 - \epsilon$ is difficult to compute directly. The text discusses several strategies to ensure these constraints are satisfied for all possible distributions within an uncertainty set. 

### Lagrange Relaxation
One primary method is [[Lagrange Relaxation]], which is used to transform a difficult chance constraint into a system of Linear Matrix Inequalities (LMIs). For example, Theorem 4.5.2 shows how a piecewise linear convex function's expectation can be bounded using an inf-sup optimization problem involving parameters $\alpha, beta, 	ext{ and } \lambda$. 

$$ 	ext{F}(z) 	riangleq 	ext{inf} 	ext{sup} 	ext{max} [\alpha_j b z + u_j] 	ext{ s.t. } 	extstyle\[\sum 	ext{alpha}_j = 1\] $$ 
This formula models the upper bound on the expectation of a piecewise linear convex function. 

### Bernstein and Robust Counterpart Approaches
The text also explores the [[Bernstein Approximation]] scheme, particularly for Gaussian perturbations. Theorem 4.5.9 provides a safe approximation for quadratically perturbed constraints using a function $F(beta, W, w)$ derived from the Bernstein scheme. 

$$ 	ext{ln } 	ext{E} \{\exp(\xi) \} = F(W, w) $$ 
This models the log-expectation of the exponential of a quadratic form. 

For more complex scenarios involving both deterministic and stochastic uncertainty, the [[Robust Counterpart]] approach is is used. The text introduces a [[Mixed Uncertainty Model]] where the perturbation $\zeta$ is a sum of a deterministic component $\xi$ and a random component $\eta$. 

### Applications and Numerical Results
The text provides several numerical illustrations, including portfolio selection and ATM cash loading, to demonstrate that these approximations are often significantly more accurate (less conservative) than the standard [[Tschebyshev Inequality]] or simple engineering bounds. 

## Relationships
- [[Lagrange Relaxation]] provides approximations for [[Chance Constraints]]
- [[Bernstein Approximation]] provides bounds for [[Chance Constraints]]
- [[Robust Counterpart]] approximates [[Chance Constraints]]
- [[Convex Optimization]] enables tractable [[Chance Constraints]]
- [[Lagrange Relaxation]] generates [[Convex Approximation]]
- [[Ambiguous Chance Constraint]] is a type of [[Chance Constraints]]
- [[Semidefinite Relaxation]] is a specific form of [[Chance Constraints]]
