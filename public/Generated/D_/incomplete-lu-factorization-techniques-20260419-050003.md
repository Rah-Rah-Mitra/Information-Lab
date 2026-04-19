---
type: content
title: "Incomplete LU Factorization Techniques"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-19T05:00:03.804729700+00:00
summary: "Incomplete LU (ILU) factorization is a method for constructing approximate inverse matrices to serve as preconditioning for iterative solvers. It provides a different trade of-off between memory and convergence speed by controlling the through levels of fill-in. It is widely used for large sparse matrices arising from PDE discretization. "
tags:
  - linear-algebra
  - sparse-matrices
  - preconditioning
  - incomplete-lu
  - iterative-methods
entities:
  - "[[Incomplete LU Factorization]]"
  - "[[ILU(0)]]"
  - "[[ILU(1)]]"
  - "[[ILU(k)]]"
  - "[[ILUT]]"
  - "[[ILUTP]]"
  - "[[Level of Fill]]"
  - "[[Preconditioning]]"
  - "[[GMRES]]"
  - "[[Sparse Skyline Format]]"
  - "[[Minimal Residual Algorithm]]"
relationships:
  - source: "Incomplete LU Factorization"
    target: "Preconditioning"
    description: "serves as"
  - source: "ILU(0)"
    target: "Incomplete LU Factorization"
    description: "is a special case of"
  - source: "ILU(1)"
    target: "Incomplete LU Factorization"
    description: "is a variant of"
  - source: "ILU(k)"
    target: "Incomplete LU Factorization"
    description: "is a generalization of"
  - source: "source"
    target: "ILU(k)"
    description: "depends on"
  - source: "ILU(k)"
    target: "Level of Fill"
    description: "uses"
  - source: "ILU(threshold)"
    target: "Incomplete LU Factorization"
    description: "is a threshold-based variant of"
  - source: "ILU(k)"
    target: "Preconditioning"
    description: "is used for"
  - source: "ILU(k)"
    target: "GMRES"
    description: "is used to precondition"
  - source: "Incomplete LU Factorization"
    target: "Sparse Skyline Format"
    description: "can be implemented using"
  - source: "ILU(k)"
    target: "Minimal Residual Algorithm"
    description: "is related to"
  - source: "ILU(k)"
    target: "ILU(k)"
    description: "is improved by"
  - source: "ILU(k)"
    target: "ILU(k)"
    description: "is improved by"
  - source: "ILU(k)"
    target: "ILU(k)"
    description: "is improved by"
---

# Incomplete LU Factorization Techniques

*Incomplete LU (ILU) factorization is a method for constructing approximate inverse matrices to serve as preconditioning for iterative solvers. It provides a different trade of-off between memory and convergence speed by controlling the through levels of fill-in. It is widely used for large sparse matrices arising from PDE discretization.*

This note explores the various forms of [[Incomplete LU Factorization]] used to improve the convergence of iterative solvers for large sparse linear systems.

## Concept
[[Incomplete LU Factorization]] (ILU) is a technique where a sparse lower triangular matrix $L$ and an upper triangular matrix $U$ are computed such that their product $LU$ approximates the original matrix $A$. Unlike full LU decomposition, ILU allows for the control of "fill-in" elements—nonzero entries that appear in the product $LU$ but are not present in the original matrix $A$. This is achieved by dropping elements based on either a fixed zero pattern or a magnitude threshold.

### Variants of ILU

#### 1. Pattern-Based ILU
These methods use a predefined zero pattern to decide which elements to keep.

- **[[ILU(0)]]**: This is the simplest form, where the zero pattern of the factors $L$ and $U$ is exactly the same as the zero pattern of the original matrix $A$. It is computationally inexpensive but may provide a crude approximation.
- **[[ILU(1)]]**: This variant allows for the first order of fill-in, keeping elements that appear in the product of the [[ILU(0)]] factors. 
- **[[ILU(k)]]**: A generalization where elements are kept based on a defined [[Level of Fill]] (denoted by $k$). The higher the $k$, the more accurate the factorization, but at a higher computational and memory cost.

#### 2. Threshold-Based ILU
These methods drop elements based on their numerical magnitude rather than just their position.

- **[[ILUT]]**: This method uses a dual dropping strategy. It drops elements that fall below a relative tolerance $	au$ and also limits the number of elements per row to a specific count $lfil$. This is highly effective for general sparse matrices but can be unstable if the matrix is indefinite.
- **[[ILUTP]]**: An extension of [[ILUT]] that incorporates column pivoting to improve stability and prevent zero pivots or exponential growth of entries.

#### 3. Specialized Formats and Strategies

- **[[Sparse Skyline Format]]**: A method for storing matrices in a way that can exploit symmetry and sparsity, particularly useful for symmetric matrices in the context of [[Incomplete LU Factorization]].
- **[[Minimal Residual Algorithm]]**: A technique used to minimize the residual norm, which can be used to construct approximate inverses directly.

## Relationships
- [[Incomplete LU Factorization]] serves as [[Preconditioning]]
- [[ILU(0)]] is a special case of [[Incomplete LU Factorization]]
- [[ILU(1)]] is a variant of [[Incomplete LU Factorization]]
- [[ILU(k)]] is a generalization of [[Incomplete LU Factorization]]
- [[ILU(k)]] depends on [[Level of Fill]]
- [[IL(k)]] is used for [[Preconditioning]]
- [[ILU(k)]] is used to precondition [[GMRES]]
- [[Incomplete LU Factorization]] can be implemented using [[Sparse Skyline Format]]
- [[ILU(k)]] is related to [[Minimal Residual Algorithm]]
