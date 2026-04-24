---
type: content
title: "Lanczos Iteration and Conjugate Gradients"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:32:32.173975200+00:00
summary: "The Lanczos iteration and Conjugate Gradients (CG) are Krylov subspace methods used for eigenvalue problems and linear systems, respectively. They leverage three-term recurrence relations to achieve efficiency, though Lanczos is susceptible to loss of orthogonality and 'ghost' eigenvalues. CG is an optimal iterative process that minimizes the A-norm of the error over a single-dimensional subspace at each step."
tags:
  - linear-algebra
  - iterative-methods
  - krylov-subspace
  - optimization
entities:
  - "[[Lanczos Iteration]]"
  - "[[Conjugate Gradients]]"
  - "[[Krylov Subspace]]"
  - "[[Ghost Eigenvalues]]"
  - "[[Jacobi Matrix]]"
  - "[[Gauss Quadrature]]"
  - "[[Legendre Polynomials]]"
  - "[[A-norm]]"
  - "[[Ritz Values]]"
relationships:
  - source: "Lanczos Iteration"
    target: "Krylov Subspace"
    description: "operates on"
  - source: "Lanczos Iteration"
    target: "Ghost Eigenvalues"
    description: "produces"
  - source: "Lanczos Iteration"
    target: "Ritz Values"
    description: "approximates"
  - source: "Conjugate Gradients"
    target: "Krylov Subspace"
    description: "operates on"
  - source: "Conjugate Gradients"
    target: "A-norm"
    description: "minimizes"
  - source: "Jacobi Matrix"
    target: "Legendre Polynomials"
    description: "relates to"
  - source: "Gauss Quadrature"
    target: "Legendre Polynomials"
    description: "uses zeros of"
  - source: "Lanczos Iteration"
    target: "Jacobi Matrix"
    description: "generates"
  - source: "Ritz Values"
    target: "Lanczos Iteration"
    description: "are produced by"
---

# Lanczos Iteration and Conjugate Gradients

*The Lanczos iteration and Conjugate Gradients (CG) are Krylov subspace methods used for eigenvalue problems and linear systems, respectively. They leverage three-term recurrence relations to achieve efficiency, though Lanczos is susceptible to loss of orthogonality and 'ghost' eigenvalues. CG is an optimal iterative process that minimizes the A-norm of the error over a single-dimensional subspace at each step.*

This note explores the connection between Krylov subspace iterations, orthogonal polynomials, and optimal iterative solvers. 

## Concept

Krylov subspace methods, such as the [[Lanczos Iteration]] and [[Conjugate Gradients]], are iterative processes that build a sequence of subspaces 

$$ K_n = (b, Ab, \dots, A^{n-1}b) $$

where the matrix $A$ is symmetric. The [[Lanczos Iteration]] is primarily used to find eigenvalues, where the approximate eigenvalues, known as [[Ritz Values]], approximate the extreme eigenvalues of the spectrum. However, in finite-precision arithmetic, the [[Lanczos Iteration]] suffers from a loss of orthogonality in the basis vectors, which leads to the emergence of [[Ghost Eigenvalues]]—spurious copies of true eigenvalues that do not correspond to the actual multiplicities of the matrix. 

In the continuous limit, the [[Lanczos Iteration]] is equivalent to the construction of [[Orthogonal Polynomials]]. For example, when applied to the operator of pointwise multiplication by $x$ on $L^2[-1, 1]$, it generates the sequence of [[Legendre Polynomials]]. The zeros of these polynomials are the nodes for [[Gauss Quadrature]], a highly efficient numerical integration method. The relationship is formalized via the [[Jacobi Matrix]], a symmetric tridiagonal matrix whose eigenvalues are the zeros of the orthogonal polynomials. 

$$ T_n = \begin{pmatrix} a_1 & \beta_1 & 0 \ & a_2 & \beta_2 \\ & & \ddots & \\ & & & a_n \end{pmatrix} $$

This tridiagonal structure is central to both the [[Lanczos Iteration]] and the construction of [[Gauss Quadrature]]. 

[[Conjugate Gradients]] is the 'original' Krylov subspace method, designed to solve symmetric positive definite systems $Ax = b$. It is an optimal process that minimizes the [[A-norm]] of the error $e_n = x - x_n$ at each step $n$ over the subspace $K_n$. Specifically, it minimizes 

$$ \|e_n\|_A = \sqrt{e_n^T A e_n} $$

This minimality property makes [[Conjugate Gradients]] exceptionally powerful for large, sparse systems. The iteration can be viewed as a nonlinear optimization algorithm that minimizes a quadratic function $w(x) = \frac{1}{2}(x^T Ax - x^T b)$ at each step. 

## Relationships
- [[Lanczos Iteration]] operates on [[Krylov Subspace]]
- [[Lanczos Iteration]] produces [[Ghost Eigenvalues]]
- [[Lanczos Iteration]] approximates [[Ritz Values]]
- [[Conjugate Gradients]] operates on [[Krylov Subspace]]
- [[Conjugate Gradients]] minimizes [[A-norm]]
- [[Jacobi Matrix]] relates to [[Legendre Polynomials]]
- [[Gauss Quadrature]] uses zeros of [[Legendre Polynomials]]
- [[Lanczos Iteration]] generates [[Jacobi Matrix]]
- [[Ritz Values]] are produced by [[Lanczos Iteration]]
