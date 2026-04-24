---
type: content
title: "Convergence and Preconditioning of Iterative Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:37:26.329420600+00:00
summary: "This note explores the convergence rates of the Conjugate Gradient (CG) method and its variants, including biorthogonalization methods like BCG. It explains how the condition number and eigenvalue distribution influence convergence speed and how preconditioning can be drastically improve performance. The discussion covers the transition from symmetric positive definite systems to nonsymmetric systems using CGN and biorthogonalization."
tags:
  - numerical-linear-algebra
  - iterative-methods
  - conjugate-gradient
  - preconditioning
  - krylov-subspace-methods
entities:
  - "[[Conjugate Gradient]]"
  - "[[Condition Number]]"
  - "[[Biconjugate Gradients]]"
  - "[[CGN]]"
  - "[[GMRES]]"
  - "[[Biorthogonalization]]"
  - "[[Preconditioning]]"
  - "[[Krylov Subspace]]"
  - "[[Chebyshev Polynomials]]"
  - "[[Singular Value Decomposition]]"
relationships:
  - source: "Conjugate Gradient"
    target: "Condition Number"
    description: "convergence rate depends on"
  - source: "Conjugate Gradient"
    target: "Krylov Subspace"
    description: "is an iteration in"
  - source: "Biconjugate Gradients"
    target: "Krylov Subspace"
    description: "operates in"
  - source: "CGN"
    target: "Singular Value Decomposition"
    description: "convergence depends on"
  - source: "Preconditioning"
    target: "Conjugate Gradient"
    description: "improves convergence of"
  - source: "Biorthogonalization"
    target: "Biconjugate Gradients"
    description: "underlies"
  - source: "Conjugate Gradient"
    target: "Chebyshev Polynomials"
    description: "uses"
  - source: "CGN"
    target: "Condition Number"
    description: "is sensitive to the square of"
  - source: "Biorthogonalization"
    target: "Krylov Subspace"
    description: "generates"
  - source: "Preconditioning"
    target: "Condition Number"
    description: "aims to reduce"
  - source: "Biconjugate Gradients"
    target: "GMRES"
    description: "is an alternative to"
---

# Convergence and Preconditioning of Iterative Methods

*This note explores the convergence rates of the Conjugate Gradient (CG) method and its variants, including biorthogonalization methods like BCG. It explains how the condition number and eigenvalue distribution influence convergence speed and how preconditioning can be drastically improve performance. The discussion covers the transition from symmetric positive definite systems to nonsymmetric systems using CGN and biorthogonalization.*

This note summarizes the convergence properties and preconditioning strategies for various Krylov subspace iterative methods used to solve linear systems. 

## Concept

Iterative methods like the [[Conjugate Gradient]] method are used to solve large, sparse systems of the form \(Ax = b\). The speed at which these methods converge is heavily influenced by the distribution of the spectrum (eigenvalues and singular values) of the matrix \(A\). 

For the [[Conjugate Gradient]] method, which is applied to symmetric positive definite matrices, the convergence rate is determined by the [[Condition Number]] of the matrix, specifically the ratio of the extreme eigenvalues. Theorem 38.5 states that the error in the A-norm satisfies:

$$ \|e_n\|_A < 2 \left( \frac{\sqrt{\kappa} - 1}{\sqrt{\kappa} + 1} \right)^n \|e_0\|_A $$

This formula shows that a larger [[Condition Number]] leads to slower convergence. The method can also be accelerated by using [[Chebyshev Polynomials]] to minimize the residual norm over the Krylov subspace. 

For nonsymmetric systems, one must move beyond standard CG. [[CGN]] (CG applied to the normal equations) solves the system by applying CG to \(A^*Ax = A^*b\). While this method is robust, it suffers from the "squaring of the condition number," as the convergence depends on the singular values of \(A\), and the condition number of \(A^*A$ is $\kappa^2$. \n\n[[Biorthogonalization]] methods, such as [[Biconjugate Gradients]] (BCG), are based on a three-term recurrence relation, which allows for low storage and computational cost per step. Unlike [[GMRES]], which minimizes the 2-norm of the residual, [[Biconjugate Gradients]] enforces an orthogonality condition between two sequences of residuals and search directions. This can lead to erratic convergence, as seen in the examples where the signs of entries are randomized. \n\n[[Biorthogonalization]] methods generate vectors in two different [[Krylov Subspace]]s: \n\n$$ V_n \in (v_1, Av_1, \dots, Av_{n-1}v_1) \quad \text{and} \quad W_n \in (w_1, A^*w_1, \dots, A^{*n-1}w_1) $$\n\nThis process can lead to a breakdown if an inner product becomes zero. \n\n[[Preconditioning]] is the essential process of transforming the original system \(Ax = b\) into a more favorable system, such as \(M^{-1}Ax = M^{-1}b\). A good [[Preconditioning]] strategy aims to make the \(M^{-1}A$ matrix have clustered eigenvalues or a condition number close to 1. Common techniques include [[Diagonal Scaling or Jacobi]], and [[Incomplete Cholesky]] factorization. 

## Relationships

- [[Conjugate Gradient]] convergence rate depends on [[Condition Number]].
- [[Conjugate Gradient]] is an iteration in [[Krylov Subspace]].
- [[Biconjugate Gradients]] operates in [[Krylov Subspace]].
- [[CGN]] convergence depends on [[Singular Value Decomposition]]."]) 
