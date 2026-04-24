---
type: content
title: "Incomplete LU Factorization"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:04:47.800394600+00:00
summary: "Incomplete LU factorization is a technique used to approximate the original linear system matrix through sparse lower and upper triangular matrices. It is a fundamental component of preconditioning for iterative solvers like GMRES, used to improve convergence rates. The method involves controlling the fill-in elements to manage the trade-off between accuracy and computational cost."
tags:
  - linear-algebra
  - sparse-matrices
  - iterative-solvers
  - preconditioning
entities:
  - "[[Incomplete LU Factorization]]"
  - "[[Preconditioning]]"
  - "[[Krylov Subspace Methods]]"
  - "[[GMRES]]"
  - "[[ILU(0)]]"
  - "[[Level of Fill]]"
  - "[[Fill-in Elements]]"
  - "[[Fill-in Elements]]"
  - "[[Gaussian Elimination]]"
  - "[[Sparse Matrix]]"
relationships:
  - source: "Incomplete LU Factorization"
    target: "Preconditioning"
    description: "is a form of"
  - source: "Incomplete LU Factorization"
    target: "Krylov Subspace Methods"
    description: "improves convergence of"
  - source: "Incomplete LU Factorization"
    target: "ILU(0)"
    description: "specialises to"
  - source: "Incomplete LU Factorization"
    target: "Gaussian Elimination"
    description: "is derived from"
  - source: "Incomplete LU Factorization"
    target: "Fill-in Elements"
    description: "manages"
  - source: "Incomplete LU(0)"
    target: "Preconditioning"
    description: "is a type of"
  - source: "Incomplete LU(0)"
    target: "Sparse Matrix"
    description: "applied to"
  - source: "Incomplete LU(0)"
    target: "Level of Fill"
    description: "uses"
  - source: "Incomplete LU(ization)"
    target: "GMRES"
    description: "used with"
  - source: "Incomplete LU(ization)"
    target: "Fill-in Elements"
    description: "controls"
  - source: "Incomplete|_0"
    target: "Sparse Matrix"
    description: "applied to"
  - source: "Incomplete LU(0)"
    target: "Gaussian Elimination"
    description: "uses"
  - source: "Incomplete LU(0)"
    target: "Level of Fill"
    description: "uses"
  - source: "Incomplete LU(0)"
    target: "Fill-in Elements"
    description: "controls"
  - source: "Incomplete LU(0)"
    target: "GMRES"
    description: "used with"
---

# Incomplete LU Factorization

*Incomplete LU factorization is a technique used to approximate the original linear system matrix through sparse lower and upper triangular matrices. It is a fundamental component of preconditioning for iterative solvers like GMRES, used to improve convergence rates. The method involves controlling the fill-in elements to manage the trade-off between accuracy and computational cost.*

This note describes the concept of [[Incomplete LU Factorization]] as a method for approximating a sparse matrix $A$ through a decomposition of the form $A \approx LU$, where $L$ is a unit lower triangular matrix and $U$ is an upper triangular matrix. This process is a core component of [[Preconditioning]] for solving large sparse linear systems using [[Krylov Subspace Methods]] like [[GMRES]].

## Concept
In a standard LU factorization, the matrix $A$ is decomposed into $L$ and $U$ factors. However, for sparse matrices, this process often introduces many non-zero elements in positions where $A$ was zero, known as [[Fill-in Elements]]. To manage the computational cost and memory, [[Incomplete LU Factorization]] is performed by dropping certain elements during the elimination process based on a predetermined zero pattern. This results in a trade of-off between the accuracy of the approximation and the computational cost of the preprocessing step.

One of the simplest forms is [[ILU(0)]], where the zero pattern of the factors $L$ and $U$ is forced to be precisely the same as the zero pattern of the original matrix $A$. This is computationally inexpensive but may result in a crude approximation.

More advanced variants like [[ILU(p)]] use the concept of [[Level of Fill]] to decide which elements to keep. A [[Level of Fill]] is a attributed to each element during [[Gaussian Elimination]], and elements are kept if their level of fill does not exceed a threshold $p$. This allows for a more accurate factorization that captures more significant fill-in elements.

$$ A \approx LU $$

This formula represents the approximate decomposition of the original matrix $A$ into sparse factors $L$ and $U$.

## Relationships
- [[Incomplete LU Factorization]] is a form of [[Preconditioning]]
- [[Incomplete LU Factorization]] improves convergence of [[Krylov Subspace Methods]]
- [[Incomplete LU Factorization]] is derived from [[Gaussian Elimination]]
- [[Incomplete LU Factorization]] manages [[Fill-in Elements]]
- [[ILU(0)]] is a type of [[Incomplete LU Factorization]]
- [[ILU(0)]] uses [[Level of Fill]]
- [[ILU(0)]] is applied to [[Sparse Matrix]]
- [[ILU(0)]] is used with [[GMRES]]
- [[ILU(p)]] uses [[Level of Fill]]
- [[Incomplete LU Factorization]] controls [[Fill-in Elements]]
