---
type: content
title: "Incomplete LU Factorization and ILUT"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:08:51.517259700+00:00
summary: "Incomplete LU (ILU) factorization is a technique for constructing approximate LU decompositions to serve as preconditioners for iterative solvers. The ILUT algorithm specifically uses threshold-based dropping rules to control fill-in and memory usage. These methods are significantly more efficient for large sparse matrices than direct solvers."
tags:
  - linear-algebra
  - sparse-matrices
  - preconditioning
  - iterative-methods
entities:
  - "[[Incomplete LU Factorization]]"
  - "[[ILUT]]"
  - "[[GMRES]]"
  - "[[ILU(0)]]"
  - "[[MILU]]"
  - "[[Diagonal Dominance]]"
  - "[[Krylov Subspace Methods]]"
relationships:
  - source: "ILUT"
    target: "Incomplete LU Factorization"
    description: "is a variant of"
  - source: "ILUT"
    target: "GMRES"
    description: "is used as a preconditioner for"
  - source: "ILU(0)"
    target: "Incomplete LU Factorization"
    description: "is a specific case of"
  - source: "MILU"
    target: "Incomplete LU Factorization"
    description: "is a variant of"
  - source: "MILU"
    target: "Diagonal Dominance"
    description: "exploits"
  - source: "Incomplete LU Factorization"
    target: "GMRES"
    description: "provides preconditioning for"
---

# Incomplete LU Factorization and ILUT

*Incomplete LU (ILU) factorization is a technique for constructing approximate LU decompositions to serve as preconditioners for iterative solvers. The ILUT algorithm specifically uses threshold-based dropping rules to control fill-in and memory usage. These methods are significantly more efficient for large sparse matrices than direct solvers.*

This note explores the construction and application of [[Incomplete LU Factorization]] as a preconditioner for iterative linear solvers.

## Concept
[[Incomplete LU Factorization]] aims to approximate the decomposition of a matrix $A$ into lower and upper triangular matrices $L$ and $U$ such that $A \approx LU$. Unlike full Gaussian elimination, which produces significant fill-in, incomplete methods drop certain elements to maintain sparsity. 

### ILU(0)
[[ILU(0)]] is the simplest form, where the sparsity pattern of $L$ and $U$ is restricted to be identical to the original sparsity pattern of $A$. This is achieved by dropping any element that falls outside the original structure of the matrix.

### ILUT (Incomplete LU with Threshold)
[[ILUT]] is a more advanced variant that uses a dual-dropping strategy to control memory and computational cost. It employs a 
1. **Relative Tolerance Threshold**: Elements are dropped if their magnitude is below a certain threshold relative to the row norm.
2. **Fill-in Control**: A parameter $lfil$ is used to keep only the largest $lfil$ elements in each row to prevent excessive memory usage.

$$ |a_{ij}| < \tau \cdot \|a_{i,:} \| $$ 

The text notes that for [[ILUT]], the existence of a valid factorization is guaranteed for [[Diagonal Dominance]] matrices under specific modification strategies (e.g., ensuring the largest element in the original row is not dropped).

### MILU (Modified ILU)
[[MILU]] (Modified Incomplete LU) attempts to compensate for the discarded entries by adding their sum to the diagonal of $L$ or $U$. This strategy is particularly effective for problems arising from Partial Differential Equations (PDEs), as it ensures the row sums of the preconditioned system are preserved, effectively capturing the discretization of constant functions.

### Application to Krylov Subspace Methods
These incomplete factorizations are frequently used to precondition [[GMRES]] (Generalized Minimal Residual method). The effectiveness of [[ILUT]] is highly dependent on the $lfil$ parameter; increasing $lfil$ improves the convergence rate (reducing the number of iterations) but increases the preprocessing time required to build the preconditioner. For large sparse matrices, there is a critical balance between the cost of amortization over multiple linear systems versus the cost of a single solve.

## Relationships
- [[ILUT]] is a variant of [[Incomplete LU Factorization]]
- [[ILU(0)]] is a specific case of [[Incomplete LU Factorization]]
- [[MILU]] is a variant of [[Incomplete LU Factorization]]
- [[ILUT]] is used as a preconditioner for [[GMRES]]
- [[Incomplete LU Factorization]] provides preconditioning for [[GMRES]]
