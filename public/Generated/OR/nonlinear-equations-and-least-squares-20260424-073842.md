---
type: content
title: "Nonlinear Equations and Least Squares"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:38:42.034877100+00:00
summary: "This note explores the relationship between solving nonlinear equations and nonlinear least-squares problems. It details Newton's method and its inexact variants for finding roots, as well as orthogonal distance regression for handling errors-in-variables. It highlights the convergence properties of local algorithms."
tags:
  - nonlinear-optimization
  - numerical-methods
  - nonlinear-equations
  - least-squares
entities:
  - "[[Nonlinear Equations]]"
  - "[[Nonlinear Least-Squares]]"
  - "[[Newton's Method]]"
  - "[[Gauss-Newton Method]]"
  - "[[Levenberg-Marquardt Algorithm]]"
  - "[[Orthogonal Distance Regression]]"
  - "[[Orthogonal Distance]]"
  - "[[Jacobian Matrix]]"
  - "[[Inexact Newton Method]]"
  - "[[Nondegenerate Solution]]"
relationships:
  - source: "Newton's Method"
    target: "Nonlinear Equations"
    description: "solves"
  - source: "Newton's Method"
    target: "Nonlinear Least-Squares"
    description: "is equivalent to"
  - source: "Gauss-Newton Method"
    target: "Nonlinear Least-Squares"
    description: "is used for"
  - source: "Source: 559f11d9335667b48ba481e28780d9345a16335ffddab2b44d0ca2bd9933835f (pages 283-284)"
    target: "Levenberg-Marquardt Algorithm"
    description: "is discussed in"
  - source: "Orthogonal Distance Regression"
    target: "Nonlinear Least-Squares"
    description: "is a type of"
  - source: "Jacobian Matrix"
    target: "Nonlinear Equations"
    description: "characterizes"
  - source: "Inexact Newton Method"
    target: "Newton's Method"
    description: "is a variant of"
  - source: "Nonlinear Equations"
    target: "Nondegenerate Solution"
    description: "requires"
  - source: "Levenberg-Marquardt Algorithm"
    target: "Nonlinear Least-Squares"
    description: "is used for"
---

# Nonlinear Equations and Least Squares

*This note explores the relationship between solving nonlinear equations and nonlinear least-squares problems. It details Newton's method and its inexact variants for finding roots, as well as orthogonal distance regression for handling errors-in-variables. It highlights the convergence properties of local algorithms.*

This note covers the fundamental methods for solving [[Nonlinear Equations]] and [[Nonlinear Least-Squares]] problems, emphasizing the local convergence of iterative algorithms.

## Concept

In [[Nonlinear Equations]], the goal is to find a root where a vector function \(r(x) = 0\). In contrast, [[Nonlinear Least-Squares]] aims to minimize the sum of squares of the residuals, \(f(x) = \frac{1}{2} \sum_{i=1}^m r_i(x)^2\). While these problems are often related—for instance, the [[Gauss-Newton Method]] and [[Levenberg-Marquardt Algorithm]] are used for least-squares—the distinction is critical when the number of equations equals the number of variables.

### Newton's Method for Nonlinear Equations

[[Newton's Method]] is a primary local algorithm for finding roots. It uses a linear model of the function based on the first derivative (the [[Jacobian Matrix]]). The step \(p_k\) is found by solving the Newton equations:

$$ J(x_k) p_k = -r(x_k) $$

This equation models the function linearly at the current iterate. When the root is a [[Nondegenerate Solution]] (meaning the [[Jacobian Matrix]] is nonsingular at the root), [[Newton's Method]] exhibits rapid local convergence: Q-superlinear if the Jacobian is continuous, and Q-quadratic if the Jacobian is Lipschitz continuous.

### Inexact Newton Methods

[[Inexact Newton Method]]s are used when solving the Newton equations exactly is too expensive. Instead of solving for \(p_k\) exactly, they satisfy a condition involving a forcing sequence \(\eta_k\):

$$ J(x_k) p_k = -r(x_k) + v_k, \text{ where } \|v_k\| \text{ is small.} $$
nThis approach allows for efficient computation using iterative solvers like GMRES.

### Orthogonal Distance Regression

[[Orthogonal Distance Regression]] (ODR) is an "errors-in-variables" model. Unlike standard least-squares, which assumes errors only in the observations (ordinates), ODR accounts for errors in both the independent and dependent variables by minimizing the [[Orthogonal Distance]] between the data points and the model curve.

## Relationships

- [[Newton's Method]] solves [[Nonlinear Equations]].
- [[Newton's Method]] is equivalent to [[Gauss-Newton Method]] in the case of zero residuals.
- [[Inexact Newton Method]] is a variant of [[Newton's Method]].
- [[Orthogonal Distance Regression]] is a type of [[Nonlinear Least-Squares]].
- [[Jacobian Matrix]] characterizes [[Nonlinear Equations]].
- [[Levenberg-Marquardt Algorithm]] is used for [[Nonlinear Least-Squares]].
