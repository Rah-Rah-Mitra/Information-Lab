---
type: content
title: "Linear System Solvers for Mesh Processing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:44:15.158170100+00:00
summary: "A comparison of iterative and direct linear system solvers, including Cholesky and LU factorization, for Laplacian and bi-Laplacian systems."
tags:
  - gis
  - linear-algebra
  - numerical-analysis
  - computer-vision
entities:
  - "[[Linear System Solver]]"
  - "[[Cholesky Factorization]]"
  - "[[LU Factorization]]"
  - "[[Conjugate Gradients]]"
  - "[[Multigrid Method]]"
  - "[[Bi-Conjugate Gradient]]"
  - "[[Generalized Minimal Residual]]"
  - "[[Stabilized Bi-Conjugate Gradients]]"
  - "[[Nested Dissection]]"
  - "[[Laplacian System]]"
  - "[[Bi-Laplacian System]]"
relationships:
  - source: "Linear System Solver"
    target: "Cholesky Factorization"
    description: "includes"
  - source: "Linear System Solver"
    target: "LU Factorization"
    description: "includes"
  - source: "Linear System Solver"
    target: "Conjugate Gradients"
    description: "includes"
  - source: "Linear System Solver"
    target: "Multigrid Method"
    description: "includes"
  - source: "LU Factorization"
    target: "Bi-Laplacian System"
    description: "may fail on large"
  - source: "Cholesky Factorization"
    target: "Bi-Laplacian System"
    description: "works robustly on"
  - source: "Conjugate Gradients"
    target: "Bi-Laplacian System"
    description: "may fail to converge on large"
  - source: "Multigrid Method"
    target: "Bi-Laplacian System"
    description: "converges robustly on"
  - source: "Nested Dissection"
    target: "Cholesky Factorization"
    description: "is used for matrix reordering in"
  - source: "Stabilized Bi-Conjugate Gradients"
    target: "Bi-Conjugate Gradient"
    description: "is a mixture of"
  - source: "Stabilized Bi-Conjugate Gradients"
    target: "Generalized Minimal Residual"
    description: "is a mixture of"
---

# Linear System Solvers for Mesh Processing

*A comparison of iterative and direct linear system solvers, including Cholesky and LU factorization, for Laplacian and bi-Laplacian systems.*

A [[Linear System Solver]] is a numerical method used to solve systems of linear equations, which in mesh processing often take the form of [[Laplacian System]] or [[Bi-Laplacian System]] matrices.

## Direct vs Iterative Solvers

Linear solvers are generally categorized into direct methods and iterative methods. Direct solvers, such as [[Cholesky Factorization]] and [[LU Factorization]], compute the exact solution up to numerical round-off errors. Iterative solvers, such as [[Conjugate Gradients]] and the [[Multigrid Method]], approach the solution incrementally and can be stopped once a sufficient error threshold is reached.

### Direct Solvers
- **Cholesky Factorization**: Highly efficient for symmetric positive definite matrices. It is numerically robust and allows for significant optimization. It often employs [[Nested Dissection]] for matrix reordering to reduce fill-in.
- **LU Factorization**: Used for general non-symmetric matrices. Unlike Cholesky, it requires pivoting (row and column permutations) to guarantee numerical stability, which complicates the factorization process and creates a trade-off between stability and fill-in minimization.

### Iterative Solvers
- **Conjugate Gradients**: An efficient iterative method for symmetric positive definite systems. However, it can break down for very large, ill-conditioned systems without effective preconditioning.
- **Multigrid Method**: Highly efficient and scales linearly with system complexity. It is particularly robust for higher-order systems like the bi-Laplacian.
- **Non-Symmetric Solvers**: For non-symmetric matrices, alternatives include the [[Bi-Conjugate Gradient]] (BiCG), the [[Generalized Minimal Residual]] (GMRES) method, and the [[Stabilized Bi-Conjugate Gradients]] (BiCGStab), which balances the efficiency of BiCG with the smooth convergence of GMRES.

## Performance Comparison

In the context of Laplacian systems, sparse direct solvers (Cholesky and LU) are often superior to multigrid methods when the system must be solved for multiple right-hand sides, as the pre-computation of the factorization is performed only once.

For [[Bi-Laplacian System]] cases, the condition number is squared and sparsity decreases, making iterative solvers like Conjugate Gradients more likely to fail. In these scenarios, the Multigrid Method and Cholesky Factorization remain robust, while LU Factorization may fail for larger systems due to the requirements of pivoting.

## Relationships
- [[Linear System Solver]] includes [[Cholesky Factorization]]
- [[Linear System Solver]] includes [[LU Factorization]]
- [[Linear System Solver]] includes [[Conjugate Gradients]]
- [[Linear System Solver]] includes [[Multigrid Method]]
- [[LU Factorization]] may fail on large [[Bi-Laplacian System]]
- [[Cholesky Factorization]] works robustly on [[Bi-Laplacian System]]
- [[Conjugate Gradients]] may fail to converge on large [[Bi-Laplacian System]]
- [[Multigrid Method]] converges robustly on [[Bi-Laplacian System]]
- [[Nested Dissection]] is used for matrix reordering in [[Cholesky Factorization]]
- [[Stabilized Bi-Conjugate Gradients]] is a mixture of [[Bi-Conjugate Gradient]] and [[Generalized Minimal Residual]]
