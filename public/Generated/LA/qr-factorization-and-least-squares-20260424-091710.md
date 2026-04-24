---
type: content
title: "QR Factorization and Least Squares"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:17:10.219675300+00:00
summary: "QR factorization decomposes a matrix into an orthogonal matrix Q and an upper-triangular matrix R. This process is fundamental to solving overdetermined systems via least squares, providing numerically stable alternatives to the normal equations. The text compares Gram-Schmidt orthogonalization and Householder triangularization as primary methods."
tags:
  - linear-algebra
  - numerical-analysis
  - qr-factorization
  - least-squares
  - matrix-decomposition
entities:
  - "[[QR Factorization]]"
  - "[[Gram-Schmidt Orthogonalization]]"
  - "[[Modified Gram-Schmidt]]"
  - "[[Classical Gram-Schmidt]]"
  - "[[Householder Triangularization]]"
  - "[[Least Squares Problem]]"
  - "[[Householder Reflector]]"
  - "[[Orthogonal Projection]]"
  - "[[Vandermonde Matrix]]"
  - "[[Machine Epsilon]]"
relationships:
  - source: "QR Factorization"
    target: "Gram-Schmidt Orthogonalization"
    description: "can be computed via"
  - source: "QR Factorization"
    target: "Householder Triangularization"
    description: "is computed via"
  - source: "QR Factorization"
    target: "Least Squares Problem"
    description: "solves"
  - source: "Gram-Schmidt Orthogonalization"
    target: "Modified Gram-Schmidt"
    description: "is a more stable version of"
  - source: "Gram-Schmidt Orthogonalization"
    target: "Classical Gram-Schmidt"
    description: "is a less stable version of"
  - source: "Householder Triangularization"
    target: "Householder Reflector"
    description: "uses"
  - source: "Least Squares Problem"
    target: "Orthogonal Projection"
    description: "is solved via"
  - source: "Householder Triangularization"
    target: "QR Factorization"
    description: "produces"
  - source: "Gram-Schmidt Orthogonalization"
    target: "QR Factorization"
    description: "produces"
  - source: "Householder Reflector"
    target: "QR Factorization"
    description: "is a component of"
---

# QR Factorization and Least Squares

*QR factorization decomposes a matrix into an orthogonal matrix Q and an upper-triangular matrix R. This process is fundamental to solving overdetermined systems via least squares, providing numerically stable alternatives to the normal equations. The text compares Gram-Schmidt orthogonalization and Householder triangularization as primary methods.*

This note explores the methods of [[QR Factorization]] and their application to solving [[Least Squares Problem]]s.

## Concept
[[QR Factorization]] is the decomposition of a matrix $A$ into an orthogonal matrix $Q$ and an upper-triangular matrix $R$, such that $A = QR$. This decomposition is used to find the orthonormal basis for the column space of $A$, which is essential for solving overdetermined systems.

### Gram-Schmidt Methods
There are two primary versions of the Gram-Schmidt process: [[Classical Gram-Schmidt]] and [[Modified Gram-Schmidt]]. While both aim to produce the same result, they differ significantly in numerical stability. [[Classical Gram-Schmidt]] is often unstable because it is prone to loss of orthogonality, especially when the matrix is near rank-deficient. [[Modified Gram-Schmidt]] provides better stability by updating the columns iteratively, though it still suffers from loss of orthogonality in floating-point arithmetic.

$$ r_{jj} = \text{size of the projection at step } j $$

The text notes that the stability of [[Modified Gram-Schmidt]] is superior to [[Classical Gram-Schmidt]] because it maintains the size of the diagonal elements $r_{jj}$ closer to the singular values of the matrix, whereas the classical version's $r_{jj}$ values drop precipitously due to rounding errors.

### Householder Triangularization
[[Householder Triangularization]] is a more robust method for computing [[QR Factorization]] than Gram-Schmidt. It is an "orthogonal triangularization" process that uses a [[Householder Reflector]] to introduce zeros below the diagonal in a column-by-column fashion. Each [[Householder Reflector]] is a unitary matrix $Q_k$ that reflects a vector across a hyperplane.

$$ Q_k = \begin{pmatrix} 1 & \dots & 0 \ 0 & F \ \end{pmatrix} $$

This method is is more stable because it is less sensitive to rounding errors. The total operation count for Householder orthogonalization is approximately $4(2mn^2 - n^3)/3$ flops.

### Least Squares
In an overdetermined system $Ax = b$ where $m > n$, the [[Least Squares Problem]] is to find a vector $x$ that minimizes the 2-norm of the residual $r = b - Ax$. Geometrically, this occurs when the residual $r$ is orthogonal to the range of $A$, meaning $A^*r = 0$.

$$ \|r\| = \|b - Ax\|_2 \), which is minimized when $r \perp \text \text{range}(A) $$\n\nThis condition is satisfied by using [[Orthogonal Projection]] onto the range of $A$ to find the closest point in the subspace.\n\n## Relationships\n- [[QR Factorization]] produces [[Gram-Schmidt Orthogonalization]] and [[Householder Triangularization]]\n- [[Gram-Schmidt Orthogonalization]] produces [[QR Factorization]]\n- [[Modified Gram-Schmidt]] is a more stable version of [[Classical Gram-Schmidt]]\n- [[Householder Triangularization]] uses [[Householder Reflector]]\n- [[Least Squares Problem]] is solved via [[Orthogonal Projection]]
