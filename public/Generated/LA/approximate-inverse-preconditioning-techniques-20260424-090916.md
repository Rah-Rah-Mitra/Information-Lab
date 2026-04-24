---
type: content
title: "Approximate Inverse Preconditioning Techniques"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:09:16.143188800+00:00
summary: "Approximate inverse techniques seek to find a sparse matrix that minimizes a residual norm, providing a robust alternative to incomplete LU factorizations. These methods include global and column-oriented minimization of the Frobenius norm, and factored approaches that seek unit triangular factors. These techniques are especially useful for indefinite or poorly conditioned matrices where standard ILU often fails."
tags:
  - linear-algebra
  - numerical-linear-algebra
  - preconditioning
  - sparse-matrices
  - iterative-solvers
entities:
  - "[[Approximate Inverse]]"
  - "[[Incomplete LU Factorization]]"
  - "[[Frobenius Norm]]"
  - "[[GMRES]]"
  - "[[Minimal Residual Algorithm]]"
  - "[[Column-Oriented Minimization]]"
  - "[[Global Minimization]]"
  - "[[Global Minimal Residual Algorithm]]"
  - "[[Inverse LU Factors]]"
  - "[[Incomplete Cholesky Factorization]]"
  - "[[, ]]"
relationships:
  - source: "Approximate Inverse"
    target: "Incomplete LU Factorization"
    description: "replaces or improves upon"
  - source: "Approximate Inverse"
    target: "Frobenius Norm"
    description: "minimizes"
  - source: "Co-oriented Minimization"
    target: "Approximate Inverse"
    description: "is a type of"
  - source: "Global Minimization"
    target: "Approximate Inverse"
    description: "is a type of"
  - source: "Global Minimal Residual Algorithm"
    target: "Approximate Inverse"
    description: "computes"
  - source: "Inverse LU Factors"
    target: "Approximate Inverse"
    description: "components of"
  - source: "Incomplete Cholesky Factorization"
    target: "Incomplete LU Factorization"
    description: "is a symmetric version of"
  - source: "GMRES"
    target: "Approximate Inverse"
    description: "can be used to solve subproblems in"
  - source: "Minimal Residual Algorithm"
    target: "Approximate Inverse"
    description: "can be used to solve subproblems in"
---

# Approximate Inverse Preconditioning Techniques

*Approximate inverse techniques seek to find a sparse matrix that minimizes a residual norm, providing a robust alternative to incomplete LU factorizations. These methods include global and column-oriented minimization of the Frobenius norm, and factored approaches that seek unit triangular factors. These techniques are especially useful for indefinite or poorly conditioned matrices where standard ILU often fails.*

This note explores various methods for constructing [[Approximate Inverse]] preconditioners, which serve as a robust alternative to [[Incomplete LU Factorization]] when standard methods fail due to zero pivots or instability.

## Concept
An [[Approximate Inverse]] is a sparse matrix $M$ designed to approximate the identity $M \approx A^{-1}$ for a given system $Ax=b$. Unlike standard ILU, which factorizes $A$ into $L$ and $U$ factors, approximate inverse methods often minimize a residual norm, such as the [[Frobenius Norm]] of the residual matrix $R = I - AM$. 

One primary objective is to minimize the residual norm in the Frobenius sense:

$$ \min_{M} \| I - AM \|_F $$

This can be approached via [[Global Minimization]] or [[Column-Oriented Minimization]].

## Global Minimization
In [[Global Minimization]], the entire matrix $M$ is treated as an unknown and updated using descent-type methods. The [[Global Minimal Residual Algorithm]] is a specific implementation where the search direction is chosen to minimize the residual norm. This algorithm exhibits quadratic convergence at the limit when no numerical dropping is applied.

## Column-Oriented Minimization
[[Column-Oriented Minimization]] approaches the problem by minimizing the objective function for each column of $M$ independently. This is highly suitable for parallel computing. Each column is computed by solving approximate linear systems, often using iterative solvers like [[GMRES]] or the [[Minimal Residual Algorithm]] in sparse-sparse mode.

## Factored Approaches
Another technique involves seeking [[Inverse LU Factors]], which are unit lower and upper triangular matrices $L$ and $U$ such that $LU \approx A^{-1}$. This approach is more mathematically structured and can be particularly effective for symmetric matrices, where it can yield the [[Incomplete Cholesky Factorization]] if the original matrix is symmetric and positive definite.

## Summary of Convergence
While standard ILU can be unstable for indefinite matrices, [[Approximate Inverse]] methods provide a more direct approximation of the inverse. The convergence of these methods, particularly the global MR algorithm, is often superlinear or quadratic depending on the implementation and the specific properties of the such as diagonal dominance.

## Relationships
- [[Approximate Inverse]] replaces or improves upon [[Incomplete LU Factorization]]
- [[Approximate Inverse]] minimizes [[Frobenius Norm]]
- [[Column-Oriented Minimization]] is a type of [[Approximate Inverse]]
- [[Global Minimization]] is atypeof [[Approximate Inverse]]
- [[Global Minimal Residual Algorithm]] computes [[Approximate Inverse]]
- [[Inverse LU Factors]] are components of [[Approximate Inverse]]
- [[Incomplete Cholesky Factorization]] is a symmetric version of [[Incomplete LU Factorization]]
- [[GMRES]] can be used to solve subproblems in [[Approximate Inverse]]
- [[Minimal Residual Algorithm]] can be used to solve subproblems in [[Approximate Inverse]]
