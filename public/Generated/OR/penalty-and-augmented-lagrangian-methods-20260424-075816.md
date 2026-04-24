---
type: content
title: "Penalty And Augmented Lagrangian Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:58:16.151177700+00:00
summary: "Penalty and augmented Lagrangian methods are iterative techniques used to solve equality-constrained optimization problems by transforming them into a sequence of unconstrained or simpler subproblems. The quadratic penalty method uses a squared violation term, while the augmented Lagrangian method incorporates explicit Lagrange multiplier estimates to improve convergence and reduce ill-conditioning. Both methods rely on the most effective selection of and updating the penalty parameter."
tags:
  - operations-research
  - optimization-methods
  - constrained-optimization
  - nonlinear-programming
entities:
  - "[[Penalty And Augmented Lagrangian Methods]]"
  - "[[Quadratic Penalty Method]]"
  - "[[Augmented Lagrangian Method]]"
  - "[[Lagrange Multipliers]]"
  - "[[KKT Conditions]]"
  - "[[Ill-Conditioning]]"
  - "[[Nonsmooth Penalty Functions]]"
  - "[[Equality-Constrained Optimization]]"
relationships:
  - source: "Penalty And Augmented Lagrangian Methods"
    target: "Quadratic Penalty Method"
    description: "includes"
  - source: "Penalty And Augmented Lagrangian Methods"
    target: "Augmented Lagrangian Method"
    description: "comprises"
  - source: "Quadratic Penalty Method"
    target: "Lagrange Multipliers"
    description: "approximates"
  - source: "Augmented Lagrangian Method"
    target: "Lagrange Multipliers"
    description: "incorporates"
  - source: "Augmented Lagrangian Method"
    target: "KKT Conditions"
    description: "satisfies"
  - source: "Quadratic Penalty Method"
    target: "KKT Conditions"
    description: "converges to"
  - source: "Quadratic Penalty Method"
    target: "Ill-Conditioning"
    description: "suffers from"
  - source: "Augmented Lagrangian Method"
    target: "Ill-Conditioning"
    description: "reduces"
  - source: "Nonsmooth Penalty Functions"
    target: "Equality-Constrained Optimization"
    description: "solves"
  - source: "Equality-Constrained Optimization"
    target: "KKT Conditions"
    description: "governed by"
  - source: "Lagrange Multipliers"
    target: "KKT Conditions"
    description: "part of"
---

# Penalty And Augmented Lagrangian Methods

*Penalty and augmented Lagrangian methods are iterative techniques used to solve equality-constrained optimization problems by transforming them into a sequence of unconstrained or simpler subproblems. The quadratic penalty method uses a squared violation term, while the augmented Lagrangian method incorporates explicit Lagrange multiplier estimates to improve convergence and reduce ill-conditioning. Both methods rely on the most effective selection of and updating the penalty parameter.*

This note explores the mechanisms and convergence properties of penalty and augmented Lagrangian methods for solving [[Equality-Constrained Optimization]] problems.

## Concept
In [[Equality-Constrained Optimization]], the goal is to minimize an objective function $f(x)$ subject to $c_i(x) = 0$ for $i \in I$. Both [[Penalty And Augmented Lagrangian Methods]] are strategies to solve this by creating a sequence of subproblems.

### Quadratic Penalty Method

The [[Quadratic Penalty Method]] uses a penalty function $Q(x; \mu) = f(x) + \frac{\mu}{2} \sum_{i \in I} c_i(x)^2$. This method transforms the constrained problem into an unconstrained one. As $\mu \to \text{infinity}$, the minimizer $x_k$ of $Q(x; \mu_k)$ approaches the global solution. However, a significant drawback is [[Ill-Conditioning]]. The Hessian of the penalty function is given by:

$$ Q_{xx}(x; \mu) = f_{xx}(x) + \mu \\sum_{i \in I} c_i(x) c_{ii}(x) + \mu A(x)^T A(x) $$

This matrix becomes increasingly ill-conditioned as $\mu$ grows, because the term $\mu A(x)^T A(x)$ has eigenvalues of order $\mu$, while others remain constant. This makes the Newton step calculation numerically unstable.

### Augmented Lagrangian Method

To mitigate the ill-conditioning, the [[Augmented Lagrangian Method]] (also known as the method of multipliers) method incorporates explicit estimates of the [[Lagrange Multipliers]] $\lambda_i$ into the objective function. The augmented Lagrangian function is defined as:

$$ L_A(x, \\lambda; \mu) = f(x) + \\sum_{i \in I} \lambda_i c_i(x) + \frac{\mu}{2} \sum_{i \in I} c_i(x)^2 $$

Unlike the [[Quadratic Penalty Method]], the [[Augmented Lagrangian Method]] allows for convergence to the solution without requiring $\mu$ to go to infinity, which significantly reduces the numerical instability. The update rule for the multipliers is:

$$ \lambda_i^{k+1} = \lambda_i^k + \mu_k c_i(x_k) $$

This method is designed to satisfy the [[KKT Conditions]] for the equality-constrained problem.

### Nonsmooth Penalty Functions

Some approaches use [[Nonsmooth Penalty Functions]], such as the $L_1$ penalty function:

$$ \phi(x; \mu) = f(x) + \\mu \sum_{i \in I} |c_i(x)| $$

These functions are are nonsmooth at the boundary of the feasible region. While they can be [[Equality-Constrained Optimization]] solutions can be exact for certain $\mu$, they are more difficult to minimize using standard smooth optimization techniques.

## Relationships
- [[Penalty And Augmented Lagrangian Methods]] includes [[Quadratic Penalty Method]]
- [[Penalty And Augmented Lagrangian Methods]] comprises [[Augmented Lagrangian Method]]
- [[Quadratic Penalty Method]] approximates [[Lagrange Multipliers]]
- [[Augmented Lagrangian Method]] incorporates [[Lagrange Multipliers]]
- [[Augmented Lagrangian Method]] satisfies [[KKT Conditions]]
- [[Quadratic Penalty Method]] converges to [[KKT Conditions]]
- [[Quadratic Penalty Method]] suffers from [[Ill-Conditioning]]
- [[Augmented Lagrangian Method]] reduces [[Ill-Conditioning]]
- [[Nonsmooth Penalty Functions]] solves [[Equality-Constrained Optimization]]
- [[Equality-Constrained Optimization]] governs [[KKT Conditions]]
- [[Lagrange Multipliers]] are part of [[KKT Conditions]]
