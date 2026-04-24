---
type: content
title: "Matrix Canonical Forms and Spectral Properties"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:28:14.875498700+00:00
summary: "This note explores the fundamental canonical forms of square matrices, including the Jordan and Schur forms, and examines the relationship between their spectra and matrix properties. It details how normality and Hermiticity define specific structural constraints on eigenvectors and eigenvalues. These concepts are essential for understanding the convergence of iterative methods and the stability of linear systems."
tags:
  - linear-algebra
  - matrix-theory
  - spectral-analysis
  - canonical-forms
entities:
  - "[[Matrix Canonical Forms]]"
  - "[[Jordan Form]]"
  - "[[Schur Form]]"
  - "[[Schur Vectors]]"
  - "[[Normal Matrix]]"
  - "[[Hermitian Matrix]]"
  - "[[Eigenvalue]]"
  - "[[Eigenvector]]"
  - "[[Spectral Radius]]"
  - "[[Rayleigh Quotient]]"
  - "[[Min-Max Theorem]]"
relationships:
  - source: "Matrix Canonical Forms"
    target: "Jordan Form"
    description: "includes"
  - source: "Matrix Canonical Forms"
    target: "Schur Form"
    description: "provides"
  - source: "Jordan Form"
    target: "Eigenvalue"
    description: "is associated with"
  - source: "Schur Form"
    target: "Schur Vectors"
    description: "is composed of"
  - source: "Normal Matrix"
    target: "Schur Form"
    description: "reduces to diagonal form via"
  - source: "Normal Matrix"
    target: "Eigenvector"
    description: "possesses orthonormal"
  - source: "Hermitian Matrix"
    target: "Normal Matrix"
    description: "is a special case of"
  - source: "Rayleigh Quotient"
    target: "Hermitian Matrix"
    description: "characterizes eigenvalues of"
  - source: "Hermitian Matrix"
    target: "Min-Max Theorem"
    description: "satisfies"
  - source: "Spectral Radius"
    target: "Eigenvalue"
    description: "is the maximum modulus of"
  - source: "Normal Matrix"
    target: "Eigenvalue"
    description: "has real eigenvalues if"
  - source: "Normal Matrix"
    target: "Eigenvalue"
    description: "is unitarily similar to diagonal via"
  - source: "Schur Form"
    target: "Eigenvalue"
    description: "contains eigenvalues on diagonal"
---

# Matrix Canonical Forms and Spectral Properties

*This note explores the fundamental canonical forms of square matrices, including the Jordan and Schur forms, and examines the relationship between their spectra and matrix properties. It details how normality and Hermiticity define specific structural constraints on eigenvectors and eigenvalues. These concepts are essential for understanding the convergence of iterative methods and the stability of linear systems.*

This note summarizes the structural decomposition of square matrices into canonical forms and the properties of their spectra. 

## Concept
Matrix decomposition involves transforming a matrix into a simpler, more structured form that preserves its eigenvalues. This is achieved through [[Matrix Canonical Forms]] such as the [[Schur Form]] and the [[Jordan Form]].

### Schur Decomposition
Any square matrix $A$ is unitarily similar to an upper triangular matrix $T$ through the $A = QTQ^*$ transformation, where $Q$ is a unitary matrix. This is known as the [[Schur Form]].

$$ A = QTQ^* $$

The vectors in $Q$ are referred to as [[Schur Vectors]]. For real matrices, the quasi-Schur form (or real Schur form) allows for $2 \times 2$ blocks on the diagonal to account for complex conjugate pairs of eigenvalues without requiring complex arithmetic.

### Jordan Canonical Form
When a matrix is not diagonalizable, it can be reduced to a block diagonal matrix consisting of [[Jordan Form]] blocks. Each block is associated with a distinct [[Eigenvalue]] $\lambda_i$ and has a structure where the diagonal contains $\lambda_i$ and the super-diagonal contains 1s.

$$ J_i = \begin{pmatrix} \lambda_i & 1 \ & \ddots & 1 \ & & \lambda_i \end{pmatrix} $$

The size of the block is determined by the index of the eigenvalue.

### Normality and Hermiticity
A [[Normal Matrix]] is defined by the property $A^*A = AA^*$. Such matrices are unitarily similar to a diagonal matrix and possess an orthonormal basis of [[Eigenvector]]s. A [[Hermitian Matrix]] is a special case of a normal matrix where all eigenvalues are real. 

For a [[Hermitian Matrix]], the eigenvalues can be characterized by the [[Min-Max Theorem]], which relates the eigenvalues to the ability to minimize or maximize the Rayleigh quotient over specific subspaces.

$$ \lambda_k = \min_{\dim(V_k)=k} \max_{x \neq 0, x \in V_k} \frac{x^*Ax}{x^*x} $$

### Spectral Properties
The [[Spectral Radius]], denoted $\rho(A)$, is the maximum modulus of the eigenvalues. The convergence of the sequence $A^k$ to zero is governed by the $\rho(A) < 1$ condition.

## Relationships
- [[Matrix Canonical Forms]] includes [[Jordan Form]]
- [[Matrix Canonical Forms]] provides [[Schur Form]]
- [[Jordan Form]] is associated with [[Eigenvalue]]
- [[Schur Form]] is composed of [[Schur Vectors]]
- [[Normal Matrix]] reduces to diagonal form via [[Schur Form]]
- [[Normal Matrix]] possesses orthonormal [[Eigenvector]]s
- [[Hermitian Matrix]] is a special case of [[Normal Matrix]]
- [[Rayleigh Quotient]] characterizes eigenvalues of [[Hermitian Matrix]]
- [[Hermitian Matrix]] satisfies [[Min-Max Theorem]]
- [[Spectral Radius]] is the maximum modulus of [[Eigenvalue]]
- [[Normal Matrix]] has real eigenvalues if [[Eigenvalue]] is real
- [[Normal Matrix]] is unitarily similar to diagonal via [[Schur Form]]
- [[Schur Form]] contains [[Eigenvalue]]s on diagonal
