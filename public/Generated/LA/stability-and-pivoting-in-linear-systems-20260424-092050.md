---
type: content
title: "Stability and Pivoting in Linear Systems"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:20:50.495622100+00:00
summary: "This note explores the numerical stability of various algorithms for solving linear least squares and square systems of equations. It compares the performance of stability-preserving methods like SVD and Householder triangularization against unstable methods like the normal equations and Gaussian elimination without pivoting. The core concept is that pivoting is essential to prevent division by zero and control rounding errors."
tags:
  - linear-algebra
  - numerical-analysis
  - stability-analysis
  - gaussian-elimination
  - lu-factorization
entities:
  - "[[Least Squares Problem]]"
  - "[[Gaussian Elimination]]"
  - "[[LU Factorization]]"
  - "[[Partial Pivoting]]"
  - "[[Singular Value Decomposition]]"
  - "[[Householder Triangularization]]"
  - "[[Normal Equations]]"
  - "[[Backward Stability]]"
  - "[[Permutation Matrix]]"
  - "[[Condition Number]]"
relationships:
  - source: "Gaussian Elimination"
    target: "LU Factorization"
    description: "produces"
  - source: "Gaussian Elimination"
    target: "Partial Pivoting"
    description: "requires"
  - source: "Gaussian Elimination"
    target: "Backward Stability"
    description: "lacks"
  - source: "LU Factorization"
    target: "Partial Pivoting"
    description: "is stabilized by"
  - source: "Singular Value Decomposition"
    target: "Least Squares Problem"
    description: "solves"
  - source: "Normal Equations"
    target: "Least Squares Problem"
    description: "solves"
  - source: "Householder Triangularization"
    target: "Least Squares Problem"
    description: "solves"
  - source: "Partial Pivoting"
    target: "Permutation Matrix"
    description: "uses"
  - source: "Gaussian Elimination"
    target: "Permutation Matrix"
    description: "uses"
  - source: "Normal Equations"
    target: "Backward Stability"
    description: "lacks"
  - source: "Singular Value Decomposition"
    target: "Backward Stability"
    description: "possesses"
  - source: "Householder Triangularization"
    target: "Backward Stability"
    description: "possesses"
  - source: "Least Squares Problem"
    target: "Condition Number"
    description: "is sensitive to"
---

# Stability and Pivoting in Linear Systems

*This note explores the numerical stability of various algorithms for solving linear least squares and square systems of equations. It compares the performance of stability-preserving methods like SVD and Householder triangularization against unstable methods like the normal equations and Gaussian elimination without pivoting. The core concept is that pivoting is essential to prevent division by zero and control rounding errors.*

This note examines the numerical stability of algorithms used to solve linear systems, specifically focusing on the distinction between backward stability and general stability. 

## Concept

In numerical linear algebra, the goal is to solve the following problems efficiently and accurately: 

1. [[Least Squares Problem]]
2. [[Gaussian Elimination]]
3. Square systems of the form \(Ax = b\).

### Least Squares Stability

For the [[Least Squares Problem]], several algorithms are compared. The [[Singular Value Decomposition]] (SVD) is the most accurate and is [[Backward Stability]] is guaranteed. [[Householder Triangularization]] is also [[Backward Stability]] is guaranteed. The [[Normal Equations]] approach, which involves forming \(A^T A x = A^T b\), is notoriously unstable for ill-conditioned problems, as its behavior is governed by the more severe [[Condition Number]] of the matrix \(A^T A\), which is the square of the condition number of \(A\). 

$$ A^T A x = A^T b $$

This formula models the normal equations for a least squares solution. 

### Gaussian Elimination and LU Factorization

[[Gaussian Elimination]] is the standard method for solving square systems. In its pure form, it transforms a matrix into an upper-triangular matrix \(U\) through a sequence of elementary transformations. This process results in the [[LU Factorization]] [[LU Factorization]] is defined as: 

$$ A = LU $$

Where \(L\) is unit lower-triangular and \(U\) is upper-triangular. Without pivoting, Gaussian elimination is susceptible to failure due to division by zero or extreme rounding errors. 

### Pivoting Strategies

To ensure stability, [[Partial Pivoting]] is employed. [[Partial Pivoting]] involves interchanging rows to ensure the largest available element in a column is used as the pivot. This process can be represented using a [[Permutation Matrix]] (\(P\), where \ enough information to be solved via the [[LU Factorization]] of the permuted matrix: 

$$ PA = LU $$

This factorization is universal for any square matrix, whether singular or nonsingular. 

## Relationships
- [[Gaussian Elimination]] produces [[LU Factorization]]
- [[Gaussian Elimination]] requires [[Partial Pivoting]] stable
- [[Singular Value Decomposition]] solves [[Least Squares Problem]]
- [[Normal Equations]] requires [[Condition Number]] is high
- [[Householder Triangularization]] solves [[Least Squares Problem]]
- [[Partial Pivoting]] uses [[Permutation Matrix]]
- [[Gaussian Elimination]] uses [[Permutation Matrix]]
- [[LU Factorization]] is stabilized by [[Partial Pivoting]]
- [[Gaussian [[Least Squares Problem]] is sensitive to [[Condition Number]]
- [[Singular Value Decomposition]] is [[Backward Stability]]
- [[Sing|Gaussian Elimination]] is not [[Backward Stability]]
- [[Householder Triangularization]] is [[Backward Stability]]
- [[Partial Pivoting]] is used to stabilize [[Gaussian Elimination]]
