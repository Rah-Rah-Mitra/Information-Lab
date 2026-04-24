---
type: content
title: "Quadratic Programming Fundamentals and Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:55:20.349759800+00:00
summary: "Quadratic programming involves minimizing a quadratic objective function subject to linear constraints. This note covers the mathematical structure of convex and nonconvex QPs, the KKT conditions, and various direct and iterative solution methods. The choice of method depends on the problem scale and constraint structure."
tags:
  - operations-research
  - optimization
  - quadratic-programming
  - mathematical-programming
entities:
  - "[[Quadratic Programming]]"
  - "[[Convex Quadratic Programming]]"
  - "[[Equality-Constrained Quadratic Programming]]"
  - "[[Karush-Kuhn-Tucker Conditions]]"
  - "[[KKT Matrix]]"
  - "[[Portfolio Optimization]]"
  - "[[Covariance Matrix]]"
  - "[[Null-Space Method]]"
  - "[[Schur-Complement Method]]"
  - "[[Active-Set Methods]]"
  - "[[Interior-Point Methods]]"
  - "[[Reduced Hessian]]"
relationships:
  - source: "Quadratic Programming"
    target: "Convex Quadratic Programming"
    description: "includes"
  - source: "Quadratic Programming"
    target: "Equality-Constrained Quadratic Programming"
    description: "specialises to"
  - source: "Quadratic Programming"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Quadratic Programming"
    target: "Portfolio Optimization"
    description: "applied in"
  - source: "Equality-Constrained Quadratic Programming"
    target: "KKT Matrix"
    description: "leads to"
  - source: "Equality-Constrained Quadratic Programming"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "governed by"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "KKT Matrix"
    description: "defines"
  - source: "Equality-Constrained Quadratic Programming"
    target: "Schur-Complement Method"
    description: "solved via"
  - source: "Equality-Constrained Quadratic Programming"
    target: "Null-Space Method"
    description: "solved via"
  - source: "Equality-Constrained Quadratic Programming"
    target: "KKT Matrix"
    description: "solved via"
  - source: "Null-Space Method"
    target: "Reduced Hessian"
    description: "uses"
  - source: "Active-Set Methods"
    target: "Quadratic Programming"
    description: "solves"
  - source: "Interior-Point Methods"
    target: "Quadratic Programming"
    description: "solves"
  - source: "Schur-Complement Method"
    target: "Reduced Hessian"
    description: "depends on"
  - source: "Portfolio Optimization"
    target: "Covariance Matrix"
    description: "uses"
---

# Quadratic Programming Fundamentals and Methods

*Quadratic programming involves minimizing a quadratic objective function subject to linear constraints. This note covers the mathematical structure of convex and nonconvex QPs, the KKT conditions, and various direct and iterative solution methods. The choice of method depends on the problem scale and constraint structure.*

[[Quadratic Programming]] is the optimization of a quadratic objective function subject to linear constraints. A general QP is stated as:

$$ \min_{x} \frac{1}{2} x^T G x + c^T x \quad \text{subject to} \quad Gx \le b, \ Ax = b $$

Where $G$ is a symmetric matrix. If $G$ is positive semidefinite, the problem is a [[Convex Quadratic Programming]] problem, which is generally easier to solve than nonconvex versions.

## Concept

### Equality-Constrained QPs
In the case of [[Equality-Constrained Quadratic Programming]], the first-order necessary conditions for optimality are given by the [[Karush-Kuhn-Tucker Conditions]]. These conditions result in a system of equations known as the [[KKT Matrix]] system:

$$ \begin{pmatrix} G & A^T \\ A & 0 \end{pmatrix} \begin{pmatrix} x \\ \lambda \end{pmatrix} = \begin{pmatrix} -c \\ b \end{pmatrix} $$

This matrix is typically indefinite. Direct solution methods include the [[Schur-Complement Method]], which uses the matrix $A G^{-1} A^T$ as a pivot, and the [[Null-Space Method]], which decouples the system using a basis for the null space of $A$.

### Portfolio Optimization
An application of [[Quadratic Programming]] is [[Portfolio Optimization]], where an investor seeks to balance expected return and risk. The risk is modeled using a [[Covariance Matrix]] $G$, where the elements are $\rho_{ij} \sigma_i \sigma_j$. The objective function combines expected return and variance using a risk tolerance parameter $\kappa$.

### Solution Methods
For large-scale problems, iterative methods like the [[Projected CG Method]] are used. These latter methods often rely on the [[Reduced Hessian]] $Z^T G Z$, where $Z$ is a null-space basis. [[Active-Set Methods]] are effective for small-to-medium problems and manage the optimal set of active constraints, while [[Interior-Point Methods]] are better suited for large-scale problems.

## Relationships
- [[Quadratic Programming]] includes [[Convex Quadratic Programming]]
- [[Quadratic Programming]] specialises to [[Equality-Constrained Quadratic Programming]]
- [[Quadratic Programming]] satisfies [[Karush-Kuhn-Tucker Conditions]]
- [[Quadratic Programming]] is applied in [[Portfolio Optimization]]
- [[Equality-Constrained Quadratic Programming]] leads to [[KKT Matrix]]
- [[Equality-Constrained Quadratic Programming]] is governed by [[Karush-Kuhn-Tucker Conditions]]
- [[Equality-Constrained Quadratic Programming]] is solved via [[Schur-Complement Method]]
- [[Equality-Constrained Quadratic Programming]] is solved via [[Null-Space Method]]
- [[Equality-Constrained Quadratic Programming]] is solved via [[KKT Matrix]]
- [[Null-Space Method]] uses [[Reduced Hessian]]
- [[Active-Set Methods]] solves [[Quadratic Programming]]
- [[Interior-Point Methods]] solves [[Quadratic Programming]]
- [[Schur-Complement Method]] depends on [[Reduced Hessian]]
- [[Portfolio Optimization]] uses [[Covariance Matrix]]
