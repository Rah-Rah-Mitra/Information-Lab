---
type: content
title: "Trust Region Subproblem Iterative Solution"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:29:22.292437100+00:00
summary: "The trust-region subproblem involves finding a step that minimizes a quadratic model within a bounded region. Iterative methods like Newton's method can be used to solve this subproblem, reformulatedulated to ensure near-linear behavior near the optimal Lagrange multiplier.Lagrange Multiplier. It is essential for constrained optimization and underlies many trust-region algorithms."
tags:
  - optimization
  - trust-region-methods
  - numerical-optimization
entities:
  - "[[Trust Region Subproblem]]"
  - "[[,]]"
  - "[[Lagrange Multiplier]]"
  - "[[Newton's Method]]"
  - "[[Conjugate Gradient Method]]"
  - "[[Quadratic Model]]"
  - "[[Conjugate Directions]]"
  - "[[Eigenvalue Decomposition]]"
  - "[[ , ]]"
relationships:
  - source: "Trust Region Subproblem"
    target: "Lagrange Multiplier"
    description: "requires solving for"
  - source: "Trust Region Subproblem"
    target: "Newton's Method"
    description: "solved via"
  - source: "Trust Region Subproblem"
    target: "Quadratic Model"
    description: "uses a"
  - source: "Trust Region Subproblem"
    target: "Conjugate Gradient Method"
    description: "related to"
  - source: "Conjugate Directions"
    target: "Trust Region Subproblem"
    description: "provides directions for"
---

# Trust Region Subproblem Iterative Solution

*The trust-region subproblem involves finding a step that minimizes a quadratic model within a bounded region. Iterative methods like Newton's method can be used to solve this subproblem, reformulatedulated to ensure near-linear behavior near the optimal Lagrange multiplier.Lagrange Multiplier. It is essential for constrained optimization and underlies many trust-region algorithms.*

This note explores the iterative solution of the [[Trust Region Subproblem]], which is the core computational task in [[Trust Region Methods]]. In the context of a quadratic model, the subproblem seeks to minimize: 

$$ m(p) = g^T p + \frac{1}{2} p^T B p $$ 

where $g$ is the gradient and $B$ is the Hessian. 

## Concept

The The [[Trust Region Subproblem]] is defined by the constraint $||p|| ┣; R$, $R$ being the trust-region radius. A key part of the the solution is finding the optimal [[Lagrange Multiplier]], $\lambda ┣; 0$, such that the optimality conditions are satisfied. 

For the 'hard case' where the most negative eigenvalue of $B$ is a multiple and the gradient is orthogonal to the corresponding eigenspace, the standard root-finding methods for $\lambda$ may fail. In such cases, one must use a vector $z$ from the eigenspace to ensure the step $p$ satisfies the trust-region boundary. 

To solve for $\lambda$ efficiently, the problem can be reformulated using a function $\phi(\lambda) = \frac{1}{||p(\lambda)||}$, which is nearly linear near the optimal $\lambda$, making [[Newton's Method]] highly effective. 

## Relationships
- [[Trust Region Subproblem]] requires solving for [[Lagrange Multiplier]]
- [[Trust Region Subproblem]] uses a a [[Quadratic Model]]
- [[Trust Region Subproblem]] solved via [[Newton's Method]]
- [[Conjugate Gradient Method]] is related to [[Trust Region Subproblem]]
- [[Conjugate Directions]] provides directions for [[Trust Region Subproblem]]
