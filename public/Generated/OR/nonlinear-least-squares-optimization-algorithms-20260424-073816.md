---
type: content
title: "Nonlinear Least-Squares Optimization Algorithms"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:38:16.296227200+00:00
summary: "Nonlinear least-squares optimization involves minimizing the sum of squared residuals to find optimal parameters for a model. Key algorithms include the Gauss-Newton and Levenberg-Marquardt methods, which exploit the special structure of the Hessian to improve efficiency and robustness. These methods are are essential for data-fitting and model-parameter estimation in various scientific disciplines."
tags:
  - optimization
  - nonlinear-least-squares
  - numerical-optimization
  - mathematical-modeling
entities:
  - "[[Nonlinear Least-Squares]]"
  - "[[Gauss-Newton Method]]"
  - "[[Jacobian Matrix]]"
  - "[[Hessian Matrix]]"
  - "[[Levenberg-Marquardt Method]]"
  - "[[Residual Vector]]"
  - "[[Trust-Region Method]]"
  - "[[Normal Equations]]"
  - "[[Singular-Value Decomposition]]"
  - "[[Linear Least-Squares]]"
relationships:
  - source: "Gauss-Newton Method"
    target: "Nonlinear Least-Squares"
    description: "solves"
  - source: "Gauss-Newton Method"
    target: "Jacobian Matrix"
    description: "approximates Hessian using"
  - source: "Gauss-Newton Method"
    target: "Hessian Matrix"
    description: "uses an approximation of"
  - source: "source_not_found"
    target: "Levenberg-Marquardt Method"
    description: "is a trust-region variant of"
  - source: "Levenberg-Marquardt Method"
    target: "Nonlinear Least-Squares"
    description: "solves"
  - source: "Levenberg-Marquardt Method"
    target: "Trust-Region Method"
    description: "utilizes"
  - source: "Levenberg-Marquardt Method"
    target: "Gauss-Marquardt Method"
    description: "is a variant of"
  - source: "Levenberg-Marquardt Method"
    target: "Jacobian Matrix"
    description: "uses"
  - source: "Levenberg-Marquardt Method"
    target: "Hessian Matrix"
    description: "approximates"
  - source: "Jacobian Matrix"
    target: "Nonlinear Least-Squares"
    description: "is a component of"
  - source: "Jacobian Matrix"
    target: "Hessian Matrix"
    description: "is used to construct"
  - source: "Normal Equations"
    target: "Nonlinear Least-Marquardt Method"
    description: "is related to"
  - source: "Levenberg-Marquardt Method"
    target: "Normal $\\text{I}$"
    description: "incorporates"
  - source: "Singular-Value Decomposition"
    target: "Gauss-Newton Method"
    description: "provides robustness for"
  - source: "Linear Least-Squares"
    target: "Nonlinear Least-Squares"
    description: "is a special case of"
---

# Nonlinear Least-Squares Optimization Algorithms

*Nonlinear least-squares optimization involves minimizing the sum of squared residuals to find optimal parameters for a model. Key algorithms include the Gauss-Newton and Levenberg-Marquardt methods, which exploit the special structure of the Hessian to improve efficiency and robustness. These methods are are essential for data-fitting and model-parameter estimation in various scientific disciplines.*

[[Nonlinear Least-Squares]] optimization is a fundamental technique used to minimize the sum of squared discrepancies between a model and observed data. The objective function typically takes the form: 

$$ f(x) = \frac{1}{2} \textstyle \sum_{j=1}^{m} [r_j(x)]^2 $$ 

This formula represents the sum of squares of the residuals, where each $r_j(x)$ is a residual function. 

## Concept

In [[Nonlinear Least-Squares]], the goal is to find the parameter vector $x$ that minimizes $f(x)$. The derivatives of $f(x)$ can be expressed using the [[Jacobian Matrix]] $J(x)$ and the [[Hessian Matrix]] $\nabla^2 f(x)$. The [[Jacobian Matrix]] is the $m \times n$ matrix of first partial derivatives of the residuals, defined as: 

$$ J(x)_{ij} = \frac{\partial r_j(x)}{\partial x_i} $$ 

This matrix captures the first-order sensitivity of the residuals to the parameter changes. 

The Hessian of the objective function is given by: 

$$ \nabla^2 f(x) = 2 J(x)^T J(x) + 2 \textstyle \sum_{j=1}^{m} r_j(x) \nabla^2 r_j(x) $$ 

This expression shows that the Hessian consists of a two-part structure: a term involving the Jacobian, and a second-order term involving the second derivatives of the residuals. 

## Algorithms

### [[Gauss-Newton Method]]

The [[Gauss-Newton Method]] is a modified Newton's method that approximates the Hessian by omitting the second-order term in the Hessian expression above. It solves the following system to find the search direction $p_k$: 

$$ J(x_k)^T J(x_k) p_k = -J(x_k)^T r(x_k) $$ 

This approximation is effective when the residuals $r_j(x)$ are small or when the Jacobian is nearly affine. The [[Gauss-Newton Method]] provides a descent direction for $f(x)$ as long as the $J(x)$ has full rank. 

### [[Levenberg-Marquardt Method]]

The [[Levenberg-Marquardt Method]] addresses the weaknesses of the Gauss-Newton method, particularly when the Jacobian is rank-deficient or nearly so. It uses a [[Trust-Region Method]] approach, specifically a spherical trust region, to solve the subproblem: 

$$ \textstyle \text{min}_{p} \frac{1}{2} ||r(x+p)||^2 \textstyle \text{subject to } ||p|| \textstyle \text{leq} \delta $$ 

This method effectively interpolates between the Gauss-Newton direction and the steepest descent direction. The solution $p$ satisfies: 

$$ (J(x)^T J(x) + \textstyle \text{lambda} I) p = -J(x)^T r(x) $$ 

This formula models the relationship between the Jacobian and a regularized Hessian approximation. 

## Relationships

- [[Gauss-Newton Method]] solves [[Nonlinear Least-Squares]]
- [[Gauss-Newton Method]] approximates [[Hessian Matrix]] using [[Jacobian Matrix]]
- [[Gauss-Newton Method]] uses an approximation of [[Hessian Matrix]]
- [[Levenberg-Marquardt Method]] is a trust-region variant of [[Gauss-Newton Method]]
- [[Levenberg-Marquardt Method]] solves [[Nonlinear Least-Squares]]
- [[Levenberg-Marquardt Method]] utilizes [[Trust-Region Method]]

