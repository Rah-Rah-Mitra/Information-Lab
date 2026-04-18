---
type: content
title: "Matrix Decompositions and Least Squares"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:56:51.766270+00:00
summary: "An overview of essential linear algebra techniques including SVD, eigenvalue decomposition, and various least squares methods for computer vision."
tags:
  - linear-algebra
  - computer-vision
  - numerical-methods
  - optimization
entities:
  - "[[Singular Value Decomposition]]"
  - "[[Eigenvalue Decomposition]]"
  - "[[QR Factorization]]"
  - "[[Cholesky Factorization]]"
  - "[[Linear Least Squares]]"
  - "[[Total Least Squares]]"
  - "[[Non-linear Least Squares]]"
  - "[[Principal Component Analysis]]"
  - "[[Conjugate Gradient]]"
  - "[[Levenberg-Marquardt Algorithm]]"
  - "[[Maximum Likelihood Estimation]]"
  - "[[Mahalanobis Distance]]"
relationships:
  - source: "Singular Value Decomposition"
    target: "Principal Component Analysis"
    description: "is used to implement"
  - source: "Eigenvalue Decomposition"
    target: "Principal Component Analysis"
    description: "is the basis for"
  - source: "Cholesky Factorization"
    target: "Linear Least Squares"
    description: "is a preferred method for solving"
  - source: "QR Factorization"
    target: "Linear Least Squares"
    description: "provides a more stable solution for"
  - source: "Total Least Squares"
    target: "Singular Value Decomposition"
    description: "is solved using"
  - source: "Non-linear Least Squares"
    target: "Levenberg-Marquardt Algorithm"
    description: "is often solved via"
  - source: "Conjugate Gradient"
    target: "Linear Least Squares"
    description: "is an iterative technique for solving"
  - source: "Maximum Likelihood Estimation"
    target: "Linear Least Squares"
    description: "reduces to when noise is Gaussian"
  - source: "Mahalanobis Distance"
    target: "Eigenvalue Decomposition"
    description: "is visualized using"
---

# Matrix Decompositions and Least Squares

*An overview of essential linear algebra techniques including SVD, eigenvalue decomposition, and various least squares methods for computer vision.*

Matrix decompositions and least squares fitting are fundamental numerical tools used to solve inverse problems in computer vision, such as camera calibration and structure from motion.

## Matrix Decompositions

Several types of matrix factorizations are used to improve numerical stability and understand the structure of data:

- [[Singular Value Decomposition]] (SVD): Decomposes any real-valued matrix into orthonormal matrices and a diagonal matrix of singular values. It provides the best possible least squares approximation to the original matrix when truncated.
- [[Eigenvalue Decomposition]]: Used for symmetric matrices to find principal directions of variation. When applied to a covariance matrix, this process is known as [[Principal Component Analysis]].
- [[QR Factorization]]: Decomposes a matrix into an orthonormal matrix and an upper triangular matrix. It is often used to solve poorly conditioned least squares problems and as a basis for computing SVD.
- [[Cholesky Factorization]]: Applied to symmetric positive definite matrices to convert them into a product of lower and upper triangular matrices. It is computationally efficient for solving normal equations in linear least squares.

## Least Squares Methods

Least squares fitting minimizes a squared distance objective function to estimate unknown parameters.

### Linear Least Squares
[[Linear Least Squares]] is used when the function is linear in the unknown parameters. The optimal estimate is typically found by solving the normal equations, often using [[Cholesky Factorization]] or [[QR Factorization]] for better stability.

### Total Least Squares
[[Total Least Squares]] (TLS) is used when there is uncertainty in all measurement directions (errors-in-variables model). The solution is found by identifying the eigenvector associated with the smallest eigenvalue of the data matrix, which is equivalent to finding the last right singular vector via [[Singular Value Decomposition]].

### Non-linear Least Squares
[[Non-linear Least Squares]] occurs when the measurement equations are not linear in the unknown parameters. These problems are solved iteratively by re-linearizing around the current estimate using the Jacobian. The [[Levenberg-Marquardt Algorithm]] is a common approach that combines the Gauss-Newton method with adaptive damping to ensure convergence.

## Iterative Techniques

For very large systems where direct factorization is prohibitively expensive, iterative methods are used:

- [[Conjugate Gradient]]: An iterative solver that takes downhill steps conjugate to each other, offering faster convergence than standard gradient descent.
- Preconditioning: A change of basis used to reduce the condition number of the matrix, thereby accelerating the convergence of iterative solvers.

## Statistical Foundation

From a probabilistic perspective, [[Maximum Likelihood Estimation]] seeks the parameter value that maximizes the likelihood of the observed data. If the measurement noise is multivariate Gaussian, the negative log-likelihood is proportional to the squared [[Mahalanobis Distance]], and the problem reduces to a weighted least squares problem.

## Relationships

- [[Singular Value Decomposition]] is used to implement [[Principal Component Analysis]].
- [[Eigenvalue Decomposition]] is the basis for [[Principal Component Analysis]].
- [[Cholesky Factorization]] is a preferred method for solving [[Linear Least Squares]].
- [[QR Factorization]] provides a more stable solution for [[Linear Least Squares]].
- [[Total Least Squares]] is solved using [[Singular Value Decomposition]].
- [[Non-linear Least Squares]] is often solved via [[Levenberg-Marquardt Algorithm]].
- [[Conjugate Gradient]] is an iterative technique for solving [[Linear Least Squares]].
- [[Maximum Likelihood Estimation]] reduces to [[Linear Least Squares]] when noise is Gaussian.
