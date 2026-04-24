---
type: content
title: "Nonlinear Equation Solvers and Constrained Optimization"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:47:59.437921700+00:00
summary: "This note covers quasi-Newton methods like Broyden's method, tensor methods, and the geometric foundations of constrained optimization. It explains how iterative solvers approximate the Jacobian to avoid expensive computations and how tangent cones define the local geometry of feasible sets. These concepts are fundamental for solving large-scale nonlinear systems and optimization problems with constraints."
tags:
  - nonlinear-equations
  - optimization-theory
  - numerical-methods
  - quasi-newton-methods
  - constrained-optimization
entities:
  - "[[Broyden's Method]]"
  - "[[Newton's Method]]"
  - "[[Tensor Methods]]"
  - "[[Merit Functions]]"
  - "[[Tangent Cone]]"
  - "[[Linearized Feasible Direction Set]]"
  - "[[Lagrangian Function]]"
  - "[[Complementarity Condition]]"
  - "[[Constraint Qualifications]]"
  - "[[Jacobian Matrix]]"
relationships:
  - source: "Broyden's Method"
    target: "Newton's Method"
    description: "is a quasi-Newton alternative to"
  - source: "Broyden's Method"
    target: "Jacobian Matrix"
    description: "approximates"
  - source: "Newton's Method"
    target: "Jacobian Matrix"
    description: "uses exact"
  - source: "Tensor Methods"
    target: "Jacob ability Matrix"
    description: "augment with higher-order terms to improve"
  - source: "Merit Functions"
    target: "Newton's Method"
    description: "guide convergence of"
  - source: "Tangent Cone"
    target: "Linearized Feasible Direction Set"
    description: "is geometrically related to"
  - source: "Lagrangian Function"
    target: "Complementarity Condition"
    description: "incorporates"
  - source: "Constraint Qualifications"
    target: "Tangent Cone"
    description: "ensure similarity between the tangent cone and the linearized feasible direction set"
---

# Nonlinear Equation Solvers and Constrained Optimization

*This note covers quasi-Newton methods like Broyden's method, tensor methods, and the geometric foundations of constrained optimization. It explains how iterative solvers approximate the Jacobian to avoid expensive computations and how tangent cones define the local geometry of feasible sets. These concepts are fundamental for solving large-scale nonlinear systems and optimization problems with constraints.*

This note explores the local algorithms used to solve nonlinear equations and the geometric principles governing constrained optimization.

## Concept

### Quasi-Newton Methods
[[Broyden's Method]] is a prominent secant method that avoids the direct calculation of the [[Jacobian Matrix]] at every iteration. Instead, it constructs an approximation, denoted as $B_k$, which is updated using the [[Broyden Update]] formula:

$$ B_{k+1} = B_k + \frac{(y_k - B_k s_k) s_k^T}{s_k^T s_k} $$

This update ensures that the approximation mimics the behavior of the true Jacobian along the direction of the step taken. While [[Newton's Method]] provides Q-quadratic convergence, [[Broyden's Method]] typically exhibits Q-superlinear convergence, making it a practical choice for large-scale systems where the exact Jacobian is too expensive to compute.

### Tensor Methods
[[Tensor Methods]] aim to capture higher-order nonlinear behavior by augmenting the linear model used in Newton's method with a tensor term $T_k$. The model function takes the form:

$$ \hat{M}_k(p) = r(x_k) + J(x_k)p + \frac{1}{2} T_k(p, p) $$

This approach is particularly useful for handling degenerate roots where the Jacobian is singular or near-singular.

### Merit Functions and Global Convergence
To ensure convergence from points far from a solution, [[Merit Functions]] are employed. A common choice is the sum-of-squares merit function:

$$ f(x) = \\frac{1}{2} \|r(x)\|^2_2 $$

Algorithms use these functions to determine if a step provides sufficient progress toward a root. However, the presence of local minima in the merit function that are not roots can cause algorithms to stall.

### Geometry of Constrained Optimization
In constrained optimization, the local geometry of the feasible set is characterized by the [[Tangent Cone]] $T(x)$. A vector $d$ is a tangent to the set at $x$ if there is a feasible sequence $z_k \to x$ such that the direction of the step is the limit of the normalized steps.

Conversely, the [[Linearized Feasible Direction Set]] $F(x)$ is defined algebraically via the linear approximation of the constraints. A [[Constraint Qualifications]] are necessary to ensure that the linearized feasible set $F(x)$ accurately reflects the actual geometry of the tangent cone $T(x)$. If these conditions fail, the linearized approximation may be exclude valid directions, leading to failure in optimization algorithms.

## Relationships
- [[Broyden's Method]] is a quasi-Newton alternative to [[Newton's Method]].
- [[Broyden's Method]] approximates the [[Jacobian Matrix]].
- [[Newton's Method]] uses exact [[Jacobian Matrix]].
- [[Tensor Methods]] augment the [[Jacobian Matrix]] with higher-order terms to improve convergence.
- [[Merit Functions]] guide convergence of [[Newton's Method]].
- [[Tangent Cone]] is geometrically related to the [[Linearized Feasible Direction Set]].
- [[Lagrangian Function]] incorporates the [[Complementarity Condition]].
- [[Constraint Qualifications]] ensure similarity between the [[Tangent Cone]] and the [[Linearized Feasible Direction Set]].
