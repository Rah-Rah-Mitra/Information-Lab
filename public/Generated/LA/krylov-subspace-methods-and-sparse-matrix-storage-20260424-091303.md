---
type: content
title: "Krylov Subspace Methods and Sparse Matrix Storage"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:13:03.645551+00:00
summary: "Krylov subspace methods are iterative solvers used for large, sparse linear systems. They rely on efficient matrix-vector products and are often enhanced by preconditioning to accelerate convergence. The note covers the relationship between subspace methods and various sparse matrix storage formats."
tags:
  - linear-algebra
  - numerical-linear-algebra
  - sparse-matrices
  - iterative-solvers
entities:
  - "[[Krylov Subspace Methods]]"
  - "[[Sparse Matrix]]"
  - "[[Preconditioning]]"
  - "[[Preconditioner]]"
  - "[[Matrix-Vector Product]]"
  - "[[GMRES]]"
  - "[[Conjugate Gradient Algorithm]]"
  - "[[Sparse Matrix Storage Format]]"
relationships:
  - source: "Krylov Subspace Methods"
    target: "Preconditioning"
    description: "benefit from"
  - source: "Krylov Subspace Methods"
    target: "Sparse Matrix"
    description: "solve"
  - source: "Krylov Subspace Methods"
    target: "Matrix-Vector Product"
    description: "rely on"
  - source: "Krylov Subspace Methods"
    target: "GMRES"
    description: "includes"
  - source: "Krylov Subspace Methods"
    target: "Conjugate Gradient Algorithm"
    description: "includes"
  - source: "Krylov Subspace Methods"
    target: "Preconditioner"
    description: "use"
  - source: "Krylov Subspace Methods"
    target: "Sparse Matrix Storage Format"
    description: "require efficient"
  - source: "Preconditioner"
    target: "Preconditioning"
    description: "enables"
  - source: "GMRES"
    target: "Preconditioner"
    description: "can be preconditioned"
  - source: "Conjugate Gradient Algorithm"
    target: "Preconditioner"
    description: "algorithm can be preconditioned"
  - source: "Sparse Matrix Storage Format"
    target: "Sparse Matrix"
    description: "represents"
  - source: "Matrix-Vector Product"
    target: "Sparse Matrix"
    description: "is performed on"
---

# Krylov Subspace Methods and Sparse Matrix Storage

*Krylov subspace methods are iterative solvers used for large, sparse linear systems. They rely on efficient matrix-vector products and are often enhanced by preconditioning to accelerate convergence. The note covers the relationship between subspace methods and various sparse matrix storage formats.*

This note explores the fundamental components of iterative solvers for large-scale linear systems, specifically focusing on [[Krylov Subspace Methods]].

## Concept
[[Krylov Subspace Methods]] are iterative techniques used to find approximate solutions to linear systems of the form $Ax = b$, where $A$ is a large, sparse matrix. These methods generate a sequence of subspace-based approximants. Common examples include the [[Conjugate Gradient Algorithm]] (for symmetric positive definite systems) and [[GMRES]] (Generalized Minimal Residual method, for non-symmetric systems).

Efficiency in these methods is heavily dependent on the ability to perform fast [[Matrix-Vector Product]] operations. For large-scale problems, the matrix $A$ is typically stored using a [[Sparse Matrix Storage Format]] such as [[Compressed Sparse Row]] (CSR) or [[Compressed Sparse Column]] (CSC). These formats optimize memory and computation by only storing non-zero elements.

Preconditioning is a critical component used to accelerate the convergence of these iterative methods. A [[Preconditioner]] $M$ is applied to transform the system into $M^{-1}Ax = M^{-1}b$ (left preconditioning) or $AM^{-1}y = b$ (right preconditioning) to improve the condition number of the system. The effectiveness of a a single [[Preconditioner]] can be significantly enhanced by techniques like [[Incomplete LU Factorization]] (ILU) or [[Polynomial Preconditioning]].

## Relationships
- [[Krylov Subspace Methods]] benefit from [[Preconditioning]].
- [[Krylov Subspace Methods]] solve [[Sparse Matrix]] systems.
- [[Krylov Subspace Methods]] rely on [[Matrix-Vector Product]].
- [[Krylov Subspace Methods]] includes [[GMRES]].
- [[Kry|lov Subspace Methods]] includes [[Conjugate Gradient Algorithm]].
- [[Klov Subspace Methods]] includes [[Preconditioner]].
- [[Krylov Subspace Methods]] require efficient [[Sparse Matrix Storage Format]].
- [[Preconditioner]] enables [[Preconditioning]].
- [[GMRES]] can be be preconditioned with a [[Preconditioner]].
- [[Conjugate Gradient Algorithm]] Compressed Sparse Row (CSR) and Compressed Sparse Column (CSC) formats are used to represent [[Sparse Matrix]] elements.
- [[Sparse Matrix Storage Format]] represents [[Sparse Matrix]].
- [[Matrix-Vector Product]] is performed on [[Sparse Matrix]].
