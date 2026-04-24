---
type: content
title: "Krylov Subspace Projection Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T08:38:10.278441600+00:00
summary: "Krylov subspace methods are iterative techniques for solving large, sparse linear systems by projecting the problem onto a subspace. These methods include GMRES, Conjugate Gradient, and Lanczos algorithms, which differ in their choice of basis and orthogonality conditions. They form a unifying framework for scientific computing."
tags:
  - linear-algebra
  - iterative-methods
  - krylov-subspace
  - numerical-linear-algebra
entities:
  - "[[Krylov Subspace]]"
  - "[[GMRES]]"
  - "[[Conjugate Gradient]]"
  - "[[Lanczos Algorithm]]"
  - "[[Orthogonal Projection]]"
  - "[[Conjugate Residual]]"
  - "[[Arnoldi Process]]"
  - "[[Petrov-Galerkin Conditions]]"
relationships:
  - source: "Krylov Subspace"
    target: "GMRES"
    description: "is the basis for"
  - source: "Krylov Subspace"
    target: "Conjugate Gradient"
    description: "is the basis for"
  - source: "Krylov Subspace"
    target: "Lanczos Algorithm"
    description: "is the basis for"
  - source: "Krylov Subspace"
    target: "Arnoldi Process"
    description: "is used in"
  - source: "GMRES"
    target: "Orthogonal Projection"
    description: "uses"
  - source: "Conjugate Gradient"
    target: "Orthogonal Projection"
    description: "uses"
  - source: "Conjugate Residual"
    target: "Orthogonal Projection"
    description: "uses"
  - source: "Arnoldi Process"
    target: "Lanczos Algorithm"
    description: "simplifies to"
  - source: "Petrov-Galerkin Conditions"
    target: "Krylov Subspace"
    description: "defines constraints on"
  - source: "Conjugate Residual"
    target: "Conjugate Gradient"
    description: "is related to"
---

# Krylov Subspace Projection Methods

*Krylov subspace methods are iterative techniques for solving large, sparse linear systems by projecting the problem onto a subspace. These methods include GMRES, Conjugate Gradient, and Lanczos algorithms, which differ in their choice of basis and orthogonality conditions. They form a unifying framework for scientific computing.*

This note explores the mathematical framework of projection methods used to solve linear systems of the form \(A\mathbf{x} = \mathbf{b}\) by searching for an approximate solution within a subspace.

## Concept
Projection methods seek an approximate solution \(x_k\) in a subspace \(K_k\) by imposing constraints on the residual vector \(r_k = b - Ax_k\). These constraints are typically defined by the [[Petrov-Galerkin Conditions]], which require the residual to be orthogonal to a specific subspace. If the search subspace and the constraint subspace are identical, the method is an [[Orthogonal Projection]].

In the context of iterative solvers, these methods are often applied to the [[Krylov Subspace]], defined as: 

$$ K_k(A, r_0) = \\text{span} \{ r_0, Ar_0, A^2r_0, \dots, A^{k-1}r_0 \} $$

This subspace provides the foundation for several major algorithms:

1. **[[GMRES]]**: Uses an orthogonal basis for the Krylov subspace, minimizing the residual norm over the affine space. It is a highly robust method but requires storing all previous basis vectors.
2. **[[Conjugate Gradient]]**: A realization of an orthogonal projection technique specifically for [[Symmetric Positive Definite]] matrices. It uses a sequence of vectors that are $A$-orthogonal (or conjugate) to ensure efficient convergence.
3. **[[Lanczos Algorithm]]**: A simplification of the [[Arnoldi Process]] for symmetric matrices. Instead of a full orthogonalization, it uses a three-term recurrence, which significantly reduces memory requirements.
4. **[[Conjugate Residual]]**: A method where the residual vectors are orthogonal and the search directions are conjugate, providing a structure similar to the Conjugate Gradient method but for Hermitian systems.

## Relationships
- [[Krylov Subspace]] is the basis for [[GMRES]], [[Conjugate Gradient]], and [[Lanczos Algorithm]].
- [[Arnoldi Process]] simplifies to [[Lanczos Algorithm]] when the matrix is symmetric.
- [[GMRES]] uses [[Orthogonal Projection]].
- [[Conjugate Gradient]] and [[Conjugate Residual]] use [[Orthogonal Projection]].
- [[Petrov-Galerkin Conditions]] define constraints on the [[Krylov Subspace]] projection.

