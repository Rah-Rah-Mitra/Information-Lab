---
type: content
title: "Nonlinear Conjugate Gradient and Quasi-Newton Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:39:10.727118900+00:00
summary: "This note covers the theoretical foundations and variants of nonlinear conjugate gradient (CG) methods and the fundamental principles of quasi-Newton methods. It explains how CG methods use search directions to minimize functions, while quasi-Newton methods build an approximate Hessian to build a more efficient search direction. Both approaches aim to achieve superlinear convergence by avoiding the direct computation of second derivatives."
tags:
  - optimization
  - nonlinear-optimization
  - conjugate-gradient
  - quasi-newton
  - numerical-methods
entities:
  - "[[Nonlinear Conjugate Gradient]]"
  - "[[Polak-Ribiere Method]]"
  - "[[Fletcher-Reeves Method]]"
  - "[[Fletcher-Powell Method]]"
  - "[[BFGS Method]]"
  - "[[Quasi-Newton Methods]]"
  - "[[Strong Wolfe Conditions]]"
  - "[[Secant Equation]]"
  - "[[Hessian Approximation]]"
relationships:
  - source: "Nonlinear Conjugate Gradient"
    target: "Strong Wolfe Conditions"
    description: "requires"
  - source: "Polak-Ribiere Method"
    target: "Nonlinear Conjugate Gradient"
    description: "is a variant of"
  - source: "Fletcher-Reeves Method"
    target: "Nonlinear Conjugate Gradient"
    description: "is a variant of"
  - source: "BFGS Method"
    target: "Quasi-Newton Methods"
    description: "is a type of"
  - source: "BFGS Method"
    target: "Secant Equation"
    description: "satisfies"
  - source: "Quasi-Newton Methods"
    target: "Hessian Approximation"
    description: "constructs an"
  - source: "Polak-Ribiere Method"
    target: "Strong Wolfe Conditions"
    description: "ensures descent"
  - source: "Fletcher-Reeves Method"
    target: "Strong Wolfe Conditions"
    description: "requires"
---

# Nonlinear Conjugate Gradient and Quasi-Newton Methods

*This note covers the theoretical foundations and variants of nonlinear conjugate gradient (CG) methods and the fundamental principles of quasi-Newton methods. It explains how CG methods use search directions to minimize functions, while quasi-Newton methods build an approximate Hessian to build a more efficient search direction. Both approaches aim to achieve superlinear convergence by avoiding the direct computation of second derivatives.*

This note explores the mechanisms of nonlinear conjugate gradient and quasi-Newton optimization methods, focusing on their ability to achieve efficient convergence without explicit second-order information.

## Concept

### Nonlinear Conjugate Gradient Methods

[[Nonlinear Conjugate Gradient]] methods are iterative descent algorithms that generate search directions $p_k$ by combining the current gradient and previous search directions. Unlike the linear CG method, which is optimal for quadratic functions, nonlinear CG methods must manage the convergence properties of their search directions to ensure they remain descent directions.

To ensure that $p_k$ is a descent direction, the step length $\alpha_k$ must satisfy the [[Strong Wolfe Conditions]] (5.43). These conditions are defined as:

$$ \begin{aligned} f(x_{k+1}) \leq f(x_k) + c_1 \alpha_k p_k^T \nabla f(x_k) \\ |\nabla f(x_{k+1})^T p_k| \leq c_2 |\nabla f(x_k)^T p_k| \end{aligned} $$

This ensures that the gradient at the next iterate is sufficiently reduced in magnitude along the direction $p_k$.

Several variants of the [[Nonlinear Conjugate Gradient]] method exist, differing primarily in the choice of the parameter $\beta_k$. The [[Fletcher-Reeves Method]] uses a formula for $\beta_k$ that is identical to the linear case for quadratic functions. However, the [[Polak-Ribiere Method]] is often more robust in practice, as it essentially performs a restart when it encounters a bad direction. This is expressed by the setting $\beta_k = \text{max}(0, \beta_k^{PR})$ in the formula (5.45).

### Quasi-Newton Methods

[[Quasi-Newton Methods]] aim to approximate the second-order curvature of the objective function by constructing a [[Hessian Approximation]] $B_k$ or its inverse $H_k$. Instead of recomputing the Hessian from scratch, these methods apply a rank-two modification to the existing approximation based on the most recent step information.

The fundamental requirement for any quasi-Newton update is the [[Secant Equation]]:

$$ B_{k+1} s_k = y_k $$

where $s_k = x_{k+1} - x_k$ and $y_k = \nabla f(x_{k+1}) - \nabla f(x_k)$. This equation ensures that the approximation $B_{k+1}$ captures the curvature information from the latest step. For the method to be successful, the curvature condition $s_k^T y_k > 0$ must be satisfied, which is guaranteed if the step length $\alpha_k$ satisfies the Wolfe conditions.

Among these methods, the [[BFGS Method]] is considered the most effective. It is is derived by imposing conditions on the inverse Hessian approximation $H_k$ to ensure it is symmetric and positive definite, and it satisfies the secant equation. The [[DFP Method]] (Davidon-Fletcher-Powell) is an [[Fletcher-Powell Method]] an earlier variant that was also highly effective, but it was eventually superseded by the [[BFGS Method]].

## Relationships

- [[Nonlinear Conjugate Gradient]] requires [[Strong Wolfe Conditions]]
- [[Polak-Ribiere Method]] is a variant of [[Nonlinear Conjugate Gradient]]
- [[Fletcher-Reeves Method]] is a variant of [[Nonlinear Conjugate Gradient]]
- [[BFGS Method]] is a type of [[Quasi-Newton Methods]]
- [[BFGS Method]] satisfies [[Secant Equation]]
- [[Quasi-Newton Methods]] constructs a [[Hessian Approximation]]
- [[Polak-Ribiere Method]] ensures descent via [[Strong Wolfe Conditions]]
- [[Fletcher-Reeves Method]] requires [[Strong Wolfe Conditions]]
