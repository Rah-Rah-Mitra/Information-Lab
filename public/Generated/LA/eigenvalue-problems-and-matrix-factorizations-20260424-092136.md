---
type: content
title: "Eigenvalue Problems and Matrix Factorizations"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:21:36.458919700+00:00
summary: "This note explores the fundamental properties of eigenvalues and eigenvectors, including multiplicities and stability. It details various matrix factorizations like Schur, diagonalization, and unitary diagonalization, which serve as the basis for modern eigenvalue algorithms. The discussion highlights the necessity of iterative methods due to the mathematical impossibility of finding exact roots for high-degree polynomials."
tags:
  - linear-algebra
  - matrix-factorization
  - eigenvalue-problems
  - numerical-analysis
entities:
  - "[[Eigenvalue Decomposition]]"
  - "[[Eigenvalue]]"
  - "[[Eigenvector]]"
  - "[[Geometric Multiplicity]]"
  - "[[Algebraic Multiplicity]]"
  - "[[Characteristic Polynomial]]"
  - "[[Similarity Transformation]]"
  - "[[Schur Factorization]]"
  - "[[Diagonalizable Matrix]]"
  - "[[Hessenberg Form]]"
  - "[[Tridiagonal Form]]"
  - "[[Normal Matrix]]"
relationships:
  - source: "Eigenvalue Decomposition"
    target: "Eigenvalue"
    description: "involves"
  - source: "Eigenvalue Decomposition"
    target: "Eigenvector"
    description: "produces"
  - source: "Eigenvalue Decomposition"
    target: "Diagonalizable Matrix"
    description: "requires"
  - source: "Characteristic Polynomial"
    target: "Eigenvalue"
    description: "defines"
  - source: "Similarity Transformation"
    target: "Eigenvalue"
    description: "preserves"
  - source: "Similarity Transformation"
    target: "Algebraic Multiplicity"
    description: "preserves"
  - source: "Similarityness Transformation"
    target: "Geometric Multiplicity"
    description: "preserves"
  - source: "Schur Factorization"
    target: "Eigenvalue"
    description: "reveals"
  - source: "Normal Matrix"
    target: "Eigenvalue Decomposition"
    description: "enables"
  - source: "Hessenberg Form"
    target: "Eigenvalue Problems"
    description: "is a preliminary reduction for"
  - source: "Tridiagonal Form"
    target: "Eigenvalue Problems"
    description: "is a preliminary reduction for"
---

# Eigenvalue Problems and Matrix Factorizations

*This note explores the fundamental properties of eigenvalues and eigenvectors, including multiplicities and stability. It details various matrix factorizations like Schur, diagonalization, and unitary diagonalization, which serve as the basis for modern eigenvalue algorithms. The discussion highlights the necessity of iterative methods due to the mathematical impossibility of finding exact roots for high-degree polynomials.*

This note covers the fundamental concepts of eigenvalues and eigenvectors and the various factorizations used to compute them in numerical linear algebra.

## Concept
An [[Eigenvalue]] is a scalar Λ such that for a non-zero vector $x$, $Ax = \lambda x$. The set of all eigenvectors corresponding to a single eigenvalue, together with the zero vector, forms an [[Eigenspace]], which is an invariant subspace. The dimension of this eigenspace is the [[Geometric Multiplicity]] of the eigenvalue. The [[Characteristic Polynomial]] $p_A(z) = \\det(zI - A)$ is a monic polynomial whose roots are the eigenvalues of the matrix $A$.

An eigenvalue's [[Algebraic Multiplicity]] is its multiplicity as a root of the characteristic polynomial. It is always at least as great as its [[Geometric Multiplicity]]. If the algebraic and geometric multiplicities are equal for all eigenvalues, the matrix is called nondefective or [[Diagonalizable Matrix]].

### Matrix Factorizations

There are several eigenvalue-revealing factorizations:
1. [[Eigenvalue Decomposition]] $A = X Λ X^{-1}$ exists if and only if $A$ is nondefective.
2. [[Unitary Diagonalization]] $A = Q Λ Q^*$ exists if and only if $A$ is a [[Normal Matrix]] (where $A^*A = AA^*$). This includes Hermitian matrices.
3. [[Schur Factorization]] $A = QTQ^*$ always exists for any square matrix, where $Q$ is unitary and $T$ is upper-triangular. The eigenvalues appear on the diagonal of $T$.

## Algorithms and Numerical Stability

Because of the impossibility result (Abel-Ruffini theorem) that no general formula exists for roots of polynomials of degree 5 or higher, eigenvalue solvers must be iterative. Most modern algorithms use a two-phase approach:

Phase 1: A direct reduction to a structured form, such as [[Hessenberg Form]] or [[Tridiagonal Form]] (for Hermitian matrices).
Phase 2: An iterative process (like the QR algorithm) to converge to a triangular or diagonal form.

Reducing a matrix to [[Hessenberg Form]] via Householder reflections is a backward stable process, meaning the computed result is the exact eigenvalue problem of a slightly perturbed matrix.

## Relationships
- [[Eigenvalue Decomposition]] involves [[Eigenvalue]] and [[Eigenvector]]
- [[Eigenvalue Decomposition]] produces [[Eigenvector]]
- [[Eigenvalue Decomposition]] requires [[Diagonalizable Matrix]]
- [[Characteristic Polynomial]] defines [[Eigenvalue]]
- [[Similarity Transformation]] preserves [[Eigenvalue]]
- [[Similarity Transformation]] preserves [[Algebraic Multiplicity]]
- [[Similarity Transformation]] preserves [[Geometric Multiplicity]]
- [[Schur Factorization]] reveals [[Eigenvalue]]
- [[Normal Matrix]] enables [[Eigenvalue Decomposition]]
- [[Hessenberg Form]] is a preliminary reduction for [[Eigenvalue Problems]]
- [[Tridiagonal Form]] is a preliminary reduction for [[Eigenvalue Problems]]
