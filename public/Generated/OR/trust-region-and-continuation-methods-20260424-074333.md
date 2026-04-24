---
type: content
title: "Trust-Region and Continuation Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:43:33.007352300+00:00
summary: "Trust-region methods and continuation methods provide robust alternatives to standard Newton-based solvers for nonlinear equations. Trust-region approaches use a local model to restrict step size, while continuation methods transform easy problems into difficult ones via a homotopy map. Both methods aim to overcome local minima and divergence issues common in local search."
tags:
  - nonlinear-equations
  - optimization-methods
  - numerical-analysis
entities:
  - "[[Trust-Region Method]]"
  - "[[Dogleg Method]]"
  - "[[Continuation Method]]"
  - "[[Homotopy Map]]"
  - "[[Lagrange Multiplier]]"
  - "[[Nonlinear Equations]]"
  - "[[Lagrange Function]]"
  - "[[Cauchy Point]]"
relationships:
  - source: "Trust-Region Method"
    target: "Dogleg Method"
    description: "specialises to"
  - source: "Dogleg Method"
    target: "Cauchy Point"
    description: "uses"
  - source: "Continuation Method"
    target: "Homotopy Map"
    description: "utilises"
  - source: "Lagrange Multiplier"
    target: "Lagrange Function"
    description: "is a parameter in"
  - source: "Lagrange Function"
    target: "Nonlinear Equations"
    description: "characterises solutions for"
  - source: "Trust-Region Method"
    target: "Nonlinear Equations"
    description: "solves"
---

# Trust-Region and Continuation Methods

*Trust-region methods and continuation methods provide robust alternatives to standard Newton-based solvers for nonlinear equations. Trust-region approaches use a local model to restrict step size, while continuation methods transform easy problems into difficult ones via a homotopy map. Both methods aim to overcome local minima and divergence issues common in local search.*

This note explores practical methods for solving [[Nonlinear Equations]], focusing on trust-region and continuation strategies.

## Concept

### Trust-Region Methods
[[Trust-Region Method]]s are iterative frameworks that solve a subproblem by minimizing a local model within a specified radius. The step $p_k$ is found by solving:

$$ \min m_k(p) \text{ subject to } \|p\| \le \\delta_k $$

This radius $\delta_k$ is adjusted based on the ratio $\rho_k$ of actual to predicted reduction:

$$ \rho_k = \frac{r(x_k) - r(x_k + p_k)}{m_k(0) - m_k(p_k)} $$

This ratio determines whether the step is accepted and if the radius should be expanded or contracted. A special case is the [[Dogleg Method]], which constructs an approximate solution by following a piecewise linear path from the origin to the [[Cauchy Point]] and then to the unconstrained minimizer.

### Continuation Methods
[[Continuation Method]]s address the shortcomings of Newton-based methods, such as convergence to local minima or divergence. They use a [[Homotopy Map]] $H(x, \\lambda)$ to gradually transform an easy problem into the original one:

$$ H(x, \\lambda) = (1 - \lambda)r(x - a) + \lambda r(x) $$

As $\lambda$ increases from 0 to 1, the algorithm follows the [[Zero Path]] (the trajectory of points where $H(x, \\lambda) = 0$). Practical implementations use predictor-corrector procedures to trace this path, often using the tangent vector to the path.

### Constrained Optimization and the Lagrangian
In the context of [[Nonlinear Equations]] and optimization, the [[Lagrange Function]] (or Lagrangian) is used to characterize solutions. For a single equality constraint $c(x) = 0$, the condition for a solution $x^*$ is that the gradients of the objective function $f(x)$ and the constraint $c(x)$ are parallel:

$$ \nabla f(x^*) = \lambda \nabla c(x^*) $$

Here, $\lambda$ is the [[Lagrange Multiplier]]. The [[Lagrange Function]] is defined as:

$$ L(x, \\lambda) = f(x) + \lambda c(x) $$

Stationary points of this function ($\nabla L(x, \\lambda) = 0$) correspond to solutions to the equality-constrained problem.

## Relationships
- [[Trust-Region Method]] specialises to [[Dogleg Method]]
- [[Dogleg Method]] uses [[Cauchy Point]]
- [[Continuation Method]] utilises [[Homotopy Map]]
- [[Lagrange Multiplier]] is a parameter in [[Lagrange Function]]
- [[Lagrange Function]] characterises solutions for [[Nonlinear Equations]]
- [[Trust-Region Method]] solves [[Nonlinear Equations]]
