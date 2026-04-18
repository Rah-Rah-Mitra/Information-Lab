---
type: content
title: "Iterative Estimation and Least-Squares Minimization"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:22:35.701024700+00:00
summary: "An overview of numerical algorithms for solving linear systems and non-linear parameter estimation using SVD, Levenberg-Marquardt, and robust cost functions."
tags:
  - computer-vision
  - numerical-analysis
  - linear-algebra
  - optimization
entities:
  - "[[Singular Value Decomposition]]"
  - "[[Least-Squares Solution]]"
  - "[[Pseudo-Inverse]]"
  - "[[Normal Equations]]"
  - "[[Newton Iteration]]"
  - "[[Gauss-Newton Method]]"
  - "[[Gradient Descent]]"
  - "[[Levenberg-Marquardt Iteration]]"
  - "[[Hessian Matrix]]"
  - "[[Jacobian Matrix]]"
  - "[[Bundle Adjustment]]"
  - "[[Robust Cost Functions]]"
relationships:
  - source: "Least-Squares Solution"
    target: "Singular Value Decomposition"
    description: "is conveniently found using"
  - source: "Pseudo-Inverse"
    target: "Singular Value Decomposition"
    description: "is defined via"
  - source: "Normal Equations"
    target: "Least-Squares Solution"
    description: "provide an alternative method for finding"
  - source: "Newton Iteration"
    target: "Hessian Matrix"
    description: "uses the second-order derivatives of"
  - source: "Gauss-Newton Method"
    target: "Newton Iteration"
    description: "is a simplification of that approximates the"
  - source: "Gauss-Newton Method"
    target: "Jacobian Matrix"
    description: "approximates the Hessian using the"
  - source: "Gradient Descent"
    target: "Newton Iteration"
    description: "is a first-order alternative to"
  - source: "Levenberg-Marquardt Iteration"
    target: "Gauss-Newton Method"
    description: "is a hybrid between and"
  - source: "Levenberg-Marquardt Iteration"
    target: "Gradient Descent"
    description: "is a hybrid between and"
  - source: "Bundle Adjustment"
    target: "Levenberg-Marquardt Iteration"
    description: "often employs a sparse version of"
  - source: "Robust Cost Functions"
    target: "Levenberg-Marquardt Iteration"
    description: "are used to mitigate outliers in"
---

# Iterative Estimation and Least-Squares Minimization

*An overview of numerical algorithms for solving linear systems and non-linear parameter estimation using SVD, Levenberg-Marquardt, and robust cost functions.*

Iterative estimation and least-squares minimization are numerical techniques used to solve linear and non-linear systems of equations, particularly in computer vision tasks like camera resection and 3D reconstruction.

## Linear Least-Squares and SVD

For an over-determined system $Ax = b$, the [[Least-Squares Solution]] seeks a vector $x$ that minimizes the norm $||Ax - b||$. This is efficiently computed using the [[Singular Value Decomposition]] (SVD), where the solution can be expressed via the [[Pseudo-Inverse]] of the matrix $A$. Alternatively, the problem can be solved using the [[Normal Equations]], which transform the system into a square $n \times n$ set of equations $A^T A x = A^T b$.

## Non-Linear Parameter Estimation

When the functional relation is non-linear, iterative methods are used to refine an initial estimate $P_0$.

### Newton and Gauss-Newton Methods
[[Newton Iteration]] minimizes a cost function by using the [[Hessian Matrix]] (second derivatives) and the gradient. The [[Gauss-Newton Method]] simplifies this by approximating the Hessian using the product of the [[Jacobian Matrix]] $J^T J$, which is particularly effective when the function is nearly linear.

### Gradient Descent and Levenberg-Marquardt
[[Gradient Descent]] moves iteratively in the direction of the negative gradient. The [[Levenberg-Marquardt Iteration]] acts as a hybrid, transitioning between Gauss-Newton (for rapid convergence near the minimum) and Gradient Descent (to guarantee a decrease in cost when far from the minimum) by augmenting the normal equations with a damping factor $\lambda I$.

## Advanced Applications

### Bundle Adjustment
[[Bundle Adjustment]] is the simultaneous estimation of camera parameters and 3D point positions. Because the [[Jacobian Matrix]] in these problems has a sparse block structure, a sparse version of the Levenberg-Marquardt algorithm is used to reduce computational complexity from $O(N^3)$ to linear in the number of points.

### Robust Estimation
To handle outliers, [[Robust Cost Functions]] are employed instead of simple squared error. These include:
- **Convex functions**: Such as the Huber and Pseudo-Huber functions, which are robust to outliers and guarantee a single global minimum.
- **Non-convex functions**: Such as the Cauchy and Blake-Zisserman functions, which diminish the weight of outliers but may introduce local minima.

## Relationships
- Least-Squares Solution depends on Singular Value Decomposition
- Pseudo-Inverse is defined via Singular Value Decomposition
- Normal Equations provide an alternative method for finding Least-Squares Solution
- Newton Iteration uses the Hessian Matrix
- Gauss-Newton Method is a simplification of Newton Iteration
- Gauss-Newton Method approximates the Hessian using the Jacobian Matrix
- Gradient Descent is a first-order alternative to Newton Iteration
- Levenberg-Marquardt Iteration is a hybrid between Gauss-Newton Method and Gradient Descent
- Bundle Adjustment often employs a sparse version of Levenberg-Marquardt Iteration
- Robust Cost Functions are used to mitigate outliers in Levenberg-Marquardt Iteration
