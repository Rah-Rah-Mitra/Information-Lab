---
type: content
title: "QR Factorization and Gram-Schmidt Orthogonalization"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:16:44.797450600+00:00
summary: "QR factorization decomposes a matrix into an orthonormal basis and an upper-triangular matrix. This process is fundamental to solving linear systems and is implemented via the Gram-Schmidt process. The method's stability depends on the varying implementations of the classical and classical-modified versions."
tags:
  - linear-algebra
  - matrix-factorization
  - numerical-linear-algebra
  - orthogonalization
entities:
  - "[[QR Factorization]]"
  - "[[Gram-Schmidt Orthogonalization]]"
  - "[[Orthonormal Basis]]"
  - "[[Upper-Triangular Matrix]]"
  - "[[Classical Gram-Schmidt]]"
  - "[[Modified Gram-Schmidt]]"
  - "[[Orthogonal Projector]]"
  - "[[Singular Value Decomposition]]"
relationships:
  - source: "QR Factorization"
    target: "Orthonormal Basis"
    description: "produces"
  - source: "QR Factorization"
    target: "Upper-Triangular Matrix"
    description: "uses"
  - source: "Gram-Schmidt Orthogonalization"
    target: "QR Factorization"
    description: "computes"
  - source: "Gram-Schmidt Orthogonalization"
    target: "Classical Gram-Schmidt"
    description: "includes"
  - source: "Gram-Schmidt Orthogonalization"
    target: "Modified Gram-Schmidt"
    description: "includes"
  - source: "Orthogonal Projector"
    target: "QR Factorization"
    description: "underlies"
  - source: "Singular Value Decomposition"
    target: "QR Factorization"
    description: "provides basis for"
---

# QR Factorization and Gram-Schmidt Orthogonalization

*QR factorization decomposes a matrix into an orthonormal basis and an upper-triangular matrix. This process is fundamental to solving linear systems and is implemented via the Gram-Schmidt process. The method's stability depends on the varying implementations of the classical and classical-modified versions.*

This note explores the decomposition of a matrix into orthogonal and triangular components, a process central to mathematical stability and numerical computation.

## Concept
[[QR Factorization]] is a decomposition of an $m \times n$ matrix $A$ into a product $A = QR$, where $Q$ is an $m \times n$ matrix with orthonormal columns and $R$ is an $n \times n$ upper-triangular matrix. This factorization is essential for solving linear systems $Ax = b$ by transforming them into $Rx = Q^*b$, which is significantly easier to solve due to the $R$ matrix's triangular structure.

In the reduced form, the columns of $Q$ span the successive subspaces spanned by the columns of $A$. For a full QR factorization, $Q$ is expanded to an $m \times m$ unitary matrix, and $R$ is padded with rows of zeros.

## Orthogonalization via Gram-Schmidt
To construct the $Q$ matrix, one employs the [[Gram-Schmidt Orthogonalization]] process. The two primary variants are:

1. [[Classical Gram-Schmidt]]: This method is mathematically equivalent to the [[Modified Gram-Schmidt]] algorithm but is numerically unstable due to rounding errors.
2. [[Modified Gram-Schmidt]]: This algorithm improves stability by performing successive projections of the remaining vectors onto the orthogonal complement of the newly found orthonormal basis vectors.

The [[Gram-Schmidt Orthogonalization]] process can be expressed using [[Orthogonal Projector]]s. For a vector $v$, the projection onto the space orthogonal to the first $j-1$ vectors is given by:

$$ P_j a_j = \frac{v_j}{\|v_j\|} $$ 

where $v_j$ is the vector obtained by subtracting the projections of $a_j$ onto the existing basis vectors.

## Relationship to SVD
The [[Singular Value Decomposition]] (SVD) provides a geometric context for these decompositions. While QR factorization focuses on the orthonormalization of columns, the SVD provides a more fundamental decomposition into scaling directions and singular values, which can be used to derive properties of the orthonormal bases used in QR.

## Relationships
- [[QR Factorization]] produces [[Orthonormal Basis]]
- [[QR Factorization]] uses [[Upper-Triangular Matrix]]
- [[Gram-Schmidt Orthogonalization]] computes [[QR Factorization]]
- [[Gram-Schmidt Orthogonalization]] includes [[Classical Gram-Schmidt]]
- [[Gram-Schmidt Orthogonalization]] includes [[Modified Gram-Schmidt]]
- [[Orthogonal Projector]] underlies [[QR Factorization]]
- [[Singular Value Decomposition]] provides basis for [[QR Factorization]]
