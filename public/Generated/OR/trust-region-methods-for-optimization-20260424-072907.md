---
type: content
title: "Trust-Region Methods for Optimization"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:29:07.459937200+00:00
summary: "Trust-region methods approximate the minimizer of an objective function within a local region where a quadratic model is considered reliable. They provide global convergence by choosing step lengths and directions simultaneously, offering astrengthening of line search methods in complex landscapes. The approach relies on a significantly improved agreement between the model and the actual function reduction."
tags:
  - operations-research
  - optimization-methods
  - nonlinear-optimization
entities:
  - "[[Trust-Region Methods]]"
  - "[[Line Search Methods]]"
  - "[[Quadratic Model]]"
  - "[[Cauchy Point]]"
  - "[[Dogleg Method]]"
  - "[[Two-Dimensional Subspace Minimization]]"
  - "[[Two-Dimensional Subspace Minimization]]"
  - "[[Trust-Region Subproblem]]"
  - "[[Global Convergence]]"
  - "[[Actual Reduction]]"
  - "[[Predicted Reduction]]"
  - "[[Hessian Matrix]]"
relationships:
  - source: "Trust-Region Methods"
    target: "Line Search Methods"
    description: "differ from"
  - source: "Trust-Region Methods"
    target: "Quadratic Model"
    description: "uses"
  - source: "Trust-Region Methods"
    target: "Trust-Region Subproblem"
    description: "solves a sequence of"
  - source: "Trust-Region Methods"
    target: "Cauchy Point"
    description: "uses as a baseline for"
  - source: "Trust-Region Methods"
    target: "Global Convergence"
    description: "achieves"
  - source: "Trust-Region Methods"
    target: "Dogleg Method"
    description: "includes"
  - source: "Trust-Region Methods"
    target: "Two-Dimensional Subspace Minimization"
    description: "includes"
  - source: "Trust-Region Methods"
    target: "Hessian Matrix"
    description: "relies on"
  - source: "Cauchy Point"
    target: "Trust-Region Subproblem"
    description: "approximate solution to"
  - source: "Dogleg Method"
    target: "Trust-Region Subproblem"
    description: "approximate solution to"
  - source: "Two-Dimensional Subspace Minimization"
    target: "Trust-Region Subproblem"
    description: "approximate solution to"
  - source: "Trust-Region Methods"
    target: "Actual Reduction"
    description: "monitors"
  - source: "Trust-Region Methods"
    target: "Predicted Reduction"
    description: "monitors"
  - source: "Trust-Region Methods"
    target: "Global Convergence"
    description: "depends on"
  - source: "Trust-Region Methods"
    target: "Hessian Matrix"
    description: "depends on"
---

# Trust-Region Methods for Optimization

*Trust-region methods approximate the minimizer of an objective function within a local region where a quadratic model is considered reliable. They provide global convergence by choosing step lengths and directions simultaneously, offering astrengthening of line search methods in complex landscapes. The approach relies on a significantly improved agreement between the model and the actual function reduction.*

[[Trust-Region Methods]] are iterative optimization techniques that define a local region around the current iterate where a quadratic model of the objective function is assumed to be an adequate representation. Unlike [[Line Search Methods]], which first determine a search direction and then a step length, trust-region methods determine both simultaneously by finding the approximate minimizer of the model within the specified radius.

## Concept
Given an objective function $f(x)$, a [[Quadratic Model]] $m_k$ is constructed using the first and second-order Taylor-series expansion information around the current iterate $x_k$. The model is typically defined as:

$$ m_k(p) = f(x_k) + g_k^T p + \frac{1}{2} p^T B_k p $$

This model represents the approximation of the function $f$ near $x_k$, where $g_k$ is the gradient and $B_k$ is an approximation of the [[Hessian Matrix]]. The algorithm proceeds by solving the [[Trust-Region Subproblem]]:

$$ \text{min } m_k(p) \text{ s.t. } ||p|| \text{ } \text{in } \text{R} $$ 

where $\Delta_k$ is the trust-region radius. The effectiveness of step is evaluated by the ratio $\rho_k$ of the [[Actual Reduction]] to the [[Predicted Reduction]]:

$$ \rho_k = \frac{f(x_k) - f(x_k + p_k)}{m_k(0) - m_k(p_k)} $$

If $\rho_k$ is close to 1, the model is reliable and the radius $\Delta_k$ may be increased. If $\rho_k$ is small or negative, the radius is reduced.

## Approximate Solutions
To ensure [[Global Convergence]], algorithms often seek approximate solutions to the subproblem rather than exact ones. Several strategies exist:

- **[[Cauchy Point]]**: The minimizer of the model along the steepest descent direction within the trust region. It provides a sufficient reduction to guarantee convergence.
- **[[Dogleg Method]]**: An approach suitable for positive definite $B_k$, which approximates the curved trajectory of the exact solution with a path consisting of two line segments (from the origin to the Cauchy point, and from the Cauchy point to the Newton step).
- **[[Two-Dimensional Subspace Minimization]]**: A more sophisticated method that expands the search to a two-dimensional subspace spanned by the gradient and the Newton direction, providing better reduction than the dogleg method.

## Relationships
- [[Trust-Region Methods]] differ from [[Line Search Methods]]
- [[Trust-Region Methods]] uses [[Quadratic Model]]
- [[Trust-Region Methods]] solves a sequence of [[Trust-Region Subproblem]]
- [[Trust-Region Methods]] uses [[Cauchy Point]] as a baseline for
- [[Trust-Region Methods]] achieves [[Global Convergence]]
- [[Trust-Region Methods]] includes [[Dogleg Method]]
- [[Trust-Region Methods]] includes [[Two-Dimensional Subspace Minimization]]
- [[Trust-Region Methods]] relies on [[Hessian Matrix]]
- [[Cauchy Point]] is an approximate solution to [[Trust-Region Subproblem]]
- [[Dogleg Method]] is an approximate solution to [[Trust-Region Subspace Minimization]]
- [[Dogleg Method]] is an approximate solution to [[Trust-Region Subproblem]]
- [[Two-Dimensional Subspace Minimization]] is an approximate solution to [[Trust-Region Subproblem]]
- [[Trust-Region Methods]] monitors [[Actual Reduction]]
- [[Trust-Region Methods]] monitors [[Predicted Reduction]]
- [[Trust-Region Methods]] achieves [[Global Convergence]] via [[Hessian Matrix]]
