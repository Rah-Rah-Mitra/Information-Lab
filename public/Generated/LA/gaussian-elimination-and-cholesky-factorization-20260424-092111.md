---
type: content
title: "Gaussian Elimination and Cholesky Factorization"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:21:11.884469+00:00
summary: "This note explores the stability of Gaussian elimination with partial pivoting and the efficient decomposition of Hermitian positive definite matrices via Cholesky factorization. It contrasts the theoretical backward stability of Gaussian elimination with its practical reliability and explains why Cholesky factorization is inherently stable and more efficient."
tags:
  - numerical-linear-algebra
  - matrix-factorization
  - stability-analysis
entities:
  - "[[Gaussian Elimination]]"
  - "[[Partial Pivoting]]"
  - "[[Cholesky Factorization]]"
  - "[[Hermitian Positive Definite Matrix]]"
  - "[[Growth Factor]]"
  - "[[Backward Stability]]"
  - "[[Eigenvalue Problem]]"
relationships:
  - source: "Gaussian Elimination"
    target: "Partial Pivoting"
    description: "uses"
  - source: "Gaussian Elimination"
    target: "Growth Factor"
    description: "characterized by"
  - source: "Cholesky Factorization"
    target: "Hermitian Positive Definite Matrix"
    description: "decomposes"
  - source: "Cholesky Factorization"
    target: "Backward Stability"
    description: "possesses"
  - source: "Gaussian Elimination"
    target: "Backward Stability"
    description: "is theoretically"
  - source: "Growth Factor"
    target: "Gaussian Elimination"
    description: "determines stability of"
---

# Gaussian Elimination and Cholesky Factorization

*This note explores the stability of Gaussian elimination with partial pivoting and the efficient decomposition of Hermitian positive definite matrices via Cholesky factorization. It contrasts the theoretical backward stability of Gaussian elimination with its practical reliability and explains why Cholesky factorization is inherently stable and more efficient.*

This note covers the fundamental matrix factorizations used in solving linear systems, focusing on the stability of Gaussian elimination and the efficiency of Cholesky factorization.

## Concept

### Gaussian Elimination with Partial Pivoting
[[Gaussian Elimination]] is the standard method for reducing a square matrix to upper-triangular form. To improve stability, [[Partial Pivoting]] is employed to ensure that the entries of the lower-triangular factor $L$ are bounded. In the case of [[Complete Pivoting]], [[Gaussian Elimination]] [[Partial Pivoting]] is modified to perform both row and column permutations, resulting in a factorization $PAQ = LU$. 

Stability in [[Gaussian Elimination]] is governed by the [[Growth Factor]], which is the ratio of the largest element in the resulting upper-triangular matrix $U$ to the largest element in the original matrix $A$. It is defined as:

$$ \rho = \frac{\max_{i,j} |u_{ij}|}{\max_{i,j} |a_{ij}|} $$ 

This formula models the ratio of amplification of entries during the reduction process.

While [[Gaussian Elimination]] with [[Partial Pivoting]] is technically [[Backward Stability]] is defined as being backward stable if the computed factors satisfy $(A + \text{SA}) = O(\text{E}_{\text{machine}} \|A\|)$, the algorithm is considered unstable for certain specific matrices where the growth factor $\rho$ becomes exponentially large (e.g., $\rho = 2^{m-1}$). However, in practice, [[Gaussian Elimination]] with [[Partial Pivoting]] is extremely reliable because such matrices are are vanishingly rare in real-world applications.

### Cholesky Factorization

For a [[Hermitian Positive Definite Matrix]], the [[Cholesky Factorization]] is a more efficient and inherently stable alternative to Gaussian elimination. A matrix $A$ is [[Hermitian Positive Definite Matrix]] if it is Hermitian ($A = A^*$) and $x^*Ax > 0$ for all $x \neq 0$. 

[[Cholesky Factorization]] decomposes the matrix into $A = R^*R$, where $R$ is an upper-triangular matrix. This process is exploits the symmetry of the matrix and requires only $\frac{1}{3}m^3$ flops, which is half the work of standard Gaussian elimination. 

Unlike Gaussian elimination, [[Cholesky Factorization]] is always stable because the entries of $R$ are bounded by the square root of the norm of $A$. 

$$ A = R^*R $$

This formula represents the decomposition of a Hermitian positive definite matrix into its triangular factors.

## Relationships
- [[Gaussian Elimination]] uses [[Partial Pivoting]]
- [[Gaussian Elimination]] is characterized by [[Growth Factor]]
- [[Cholesky Factorization]] decomposes [[Hermitian Positive Definite Matrix]]
- [[Cholesky Factorization]] possesses [[Backward Stability]]
- [[Gaussian Elimination]] is theoretically [[Backward Stability]]
- [[Growth Factor]] determines stability of [[Gaussian Elimination]]
