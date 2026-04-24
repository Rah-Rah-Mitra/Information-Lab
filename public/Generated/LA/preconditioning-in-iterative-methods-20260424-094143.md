---
type: content
title: "Preconditioning In Iterative Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:41:43.981408100+00:00
summary: "Preconditioning is a technique used to improve the convergence rate of iterative solvers for linear systems and eigenvalue problems. It involves transforming the original problem into an equivalent one with more favorable spectral properties. This method is widely used in domain decomposition and sparse matrix computations."
tags:
  - numerical-linear-algebra
  - iterative-methods
  - preconditioning
entities:
  - "[[Preconditioning]]"
  - "[[Iterative Methods]]"
  - "[[Linear Systems]]"
  - "[[Linear Eigenvalue Problems]]"
  - "[[Incomplete Factorization]]"
  - "[[Domain Decomposition]]"
  - "[[Jacobi-Davidson Algorithm]]"
  - "[[Krylov Subspace Methods]]"
relationships:
  - source: "Preconditioning"
    target: "Iterative Methods"
    description: "accelerates"
  - source: "Preconditioning"
    target: "Linear Systems"
    description: "improves convergence of"
  - source: "Preconditioning"
    target: "Linear Eigenvalue Problems"
    description: "improves convergence of"
  - source: "Preconditioning"
    target: "Incomplete Factorization"
    description: "utilises"
  - source: "Preconditioning"
    target: "Domain Decomposition"
    description: "is applied in"
  - source: "Preconditioning"
    target: "Jacobi-Davidson Algorithm"
    description: "is enhanced by"
  - source: "Preconditioning"
    target: "Krytextic Subspace Methods"
    description: "is a core component of"
---

# Preconditioning In Iterative Methods

*Preconditioning is a technique used to improve the convergence rate of iterative solvers for linear systems and eigenvalue problems. It involves transforming the original problem into an equivalent one with more favorable spectral properties. This method is widely used in domain decomposition and sparse matrix computations.*

[[Preconditioning]] is a fundamental technique in [[Iterative Methods]] used to improve the convergence rate of solvers for both [[Linear Systems]] and [[Linear Eigenvalue Problems]]. The core objective is to transform the original system into an equivalent one that possesses more favorable spectral properties, such as a smaller condition number or more clustered eigenvalues.

## Concept
In the context of solving $Ax = b$, where $A$ is a large, sparse matrix, standard [[Iterative Methods]] like [[Krylov Subspace Methods]] often suffer from slow convergence if the matrix is is ill-conditioned. [[Preconditioning]] involves applying a transformation $M^{-1}$ such that the resulting system $M^{-1}Ax = M^{-1}b$ has a better convergence profile. 

Common approaches to [[Preconditioning]] include:
- [[Incomplete Factorization]] (e.g., ILU or ICCG): Approximating the $LU$ or Cholesky factorization to provide a dense-like preconditioner.
- [[Domain Decomposition]]: Partitioning the problem into smaller sub-problems to facilitate parallel computation and improved convergence.
- [[Jacobi-Davidson Algorithm]]: A modern approach specifically designed for eigenvalue problems, which uses a subspace projection to find eigenvalues and eigenvectors.

Other specialized techniques include [[Polynomial Acceleration Devices]] and [[Shift-and-Invert Arnoldi Methods]] which leverage polynomial properties to speed up iterations.

## Relationships
- [[Preconditioning]] accelerates [[Iterative Methods]]
- [[Preconditioning]] improves convergence of [[Linear Systems]]
- [[Preconditioning]] improves convergence of [[Linear Eigenvalue Problems]]
- [[Preconditioning]] and [[Incomplete Factorization]] are related via the use of approximate factorizations as preconditioners.
- [[Preconditioning]] is applied in [[Domain Decomposition]]
- [[Preconditioning]] is enhanced by [[Jacobi-Davidson Algorithm]]
- [[Preconditioning]] is a core component of [[Krylov Subspace Methods]]
