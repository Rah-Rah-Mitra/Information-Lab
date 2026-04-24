---
type: content
title: "Krylov Subspace Iterative Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:25:22.002810400+00:00
summary: "Krylov subspace methods like Arnoldi and Lanczos iterations are used to find eigenvalues and solve linear systems. Their convergence is fundamentally linked to polynomial approximation problems on the spectrum of a matrix. The efficiency of these methods depends on the eigenvalue distribution and matrix conditioning."
tags:
  - linear-algebra
  - iterative-methods
  - krylov-subspace
  - eigenvalue-problems
  - numerical-linear-algebra
entities:
  - "[[Arnoldi Iteration]]"
  - "[[Lanczos Iteration]]"
  - "[[GMRES]]"
  - "[[Krylov Subspace]]"
  - "[[Ritz Values]]"
  - "[[Polynomial Approximation]]"
  - "[[Eigenvalue Distribution]]"
  - "[[Eigenvalue]]"
  - "[[Matrix]]"
  - "[[Linear System]]"
relationships:
  - source: "Arnoldi Iteration"
    target: "Krylov Subspace"
    description: "operates on"
  - source: "Arnoldi Iteration"
    target: "Ritz Values"
    description: "produces"
  - source: "Lanczos Iteration"
    target: "Krylov Subspace"
    description: "operates on"
  - source: "Lanczos Iteration"
    target: "Ritz Values"
    description: "produces"
  - source: "Lanczos Iteration"
    target: "Arnoldi Iteration"
    description: "specialises to"
  - source: "GMRES"
    target: "Krylov Subspace"
    description: "uses"
  - source: "GMRES"
    target: "Ritz Values"
    description: "approximates"
  - source: "GMRES"
    target: "Arnoldi Iteration"
    description: "uses"
  - source: "Ritz Values"
    target: "Polynomial Approximation"
    description: "are roots of"
  - source: "Polynomial Approximation"
    target: "Eigenvalue Distribution"
    description: "governs convergence via"
  - source: "Arnoldi Iteration"
    target: "Eigenvalue"
    description: "locates"
  - source: "Lanczos Iteration"
    target: "Eigenvalue"
    description: "locates"
  - source: "GMRES"
    target: "Linear System"
    description: "solves"
  - source: "Krylov Subspace"
    target: "Eigenvalue"
    description: "contains information about"
---

# Krylov Subspace Iterative Methods

*Krylov subspace methods like Arnoldi and Lanczos iterations are used to find eigenvalues and solve linear systems. Their convergence is fundamentally linked to polynomial approximation problems on the spectrum of a matrix. The efficiency of these methods depends on the eigenvalue distribution and matrix conditioning.*

This note explores the mechanics and convergence of Krylov subspace methods, specifically the Arnoldi and Lanczos iterations, and the GMRES algorithm for solving linear systems.

## Concept

Krylov subspace methods are iterative processes that build a subspace spanned by the vectors $b, Ab, A^2b, \dots, A^{n-1}b$. This subspace is denoted as $\mathcal{K}_n(A, b) = \text{span}\{b, Ab, \dots, A^{n-1}b\}$. The primary goal is to either find approximate [[Eigenvalue]]s of a matrix $A$ or to solve a linear system $Ax = b$.

### Arnoldi Iteration

[[Arnoldi Iteration]] is a general method for non-Hermitian matrices. It uses the Arnoldi process to construct an orthonormal basis for the Krylov subspace. An orthonormal basis $Q_n$ is produced, basis vectors $q_1, q_2, \dots, q_n$ basis vectors $q_1, q1, q_2, \text    importantly, the Arnoldi polynomial $p_n$ is the polynomial of degree $n$ that minimizes the norm of the residual, which is related to finding thes roots of the Hessenberg matrix $H_n$. The [[Ritz Values]] are the eigenvalues of $H_n$, and they serve as approximate eigenvalues of $A$. \n\n$$ H_n = Q_n^* A Q_n $$\n\nThis matrix $H_n$ represents the projection of $A$ onto theMatrix basis. The [[Arnoldi Iteration]] [[Ritz Values]] typically converge to the extreme eigenvalues of $A$ first, convergence is often geometric. convergence is governed by the polynomial approximation problem: find $p_n \in 	ext{P}_n$ (monic polynomials of degree $n$) monic polynomials of degree $n$ such that $\|p_n(A)b\| = 	ext{minimum}$. \n\n### GMRES\n\n[[GMRES]] (Generalized Minimal Residual) is used to solve $Ax = b$. At each step $n$, it finds an approximate solution $x_n \in \mathcal{K}_n$ that minimizes the residual norm $r_n = b - Ax_n$. \n\n$$ \|r_n\| = \min_{p 
 \in \text{P}_{n,0}} \|p(A)b\| $$\n\nThis minimization problem is essentially a polynomial approximation problem where the polynomials are normalized such that $p(0) = 1$. The convergence of [[GMRES]] depends on the heavily on the location of the eigenvalues of $A$ and the condition number of $A$. If the eigenvalues surround the origin, convergence can stagnate. \n\n### Lanczos Iteration\n\n[[Lanczos Iteration]] is the specialized case of the Arnoldi iteration for Hermitian (or real symmetric) matrices. Because the Hessenberg matrix $H_n$ is both symmetric and Hessenberg, it is tridiagonal. This leads to a three-term recurrence relation, making each step much cheaper than the Arnoldi process.\n\n$$ A q_n = \alpha_n q_n + \beta_{n-1} q_{n-1} + \beta_n q_{n+1} $$\n\nThis recurrence allows the [[Lanczos Iteration]] to efficiently find [[Eigenvalue]]s. The [[Ritz Values]] produced by Lanczos are the roots of the Lanczos polynomial, which is small on the spectrum of $A$. The convergence to outliers is often faster, a phenomenon explained by the 
