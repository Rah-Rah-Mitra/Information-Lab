---
type: content
title: "Projection Methods for Linear Systems"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T08:33:19.629717500+00:00
summary: "Projection methods approximate solutions to linear systems by restricting the search to a subspace. These techniques include orthogonal and oblique projections, which are can be categorized into residual and residual-norm minimization or error projection methods. Krylov subspace methods, such as the Arnoldi process, building an orthonormal basis for the subspace, construct polynomial approximations. polynomial approximations. "
tags:
  - linear-algebra
  - numerical-linear-algebra
  - projection-methods
  - krylov-subspace-methods
  - optimization
entities:
  - "[[Projection Methods]]"
  - "[[Orthogonal Projection]]"
  - "[[Oblique Projection]]"
  - "[[Petrov-Galerkin Condition]]"
  - "[[Krylov Subspace]]"
  - "[[Galerkin Condition]]"
  - "[[Symmetric Positive Definite Matrix]]"
  - "[[Arnoldi Process]]"
  - "[[Hessenberg Matrix]]"
  - "[[Minimal Residual Method]]"
  - "[[Error Projection Methods]]"
  - "[[Residual Projection Methods]]"
relationships:
  - source: "Projection Methods"
    target: "Orthogonal Projection"
    description: "utilize"
  - source: "Projection Methods"
    target: "Oblique Projection"
    description: "utilize"
  - source: "Projection Methods"
    target: "Krylov Subspace"
    description: "use"
  - source: "Projection Methods"
    target: "Petrov-Galerkin Condition"
    description: "satisfy"
  - source: "Projection Methods"
    target: "Arnoldi Process"
    description: "implemented-via"
  - source: "Arnoldi Process"
    target: "Krylov Subspace"
    description: "builds-basis-for"
  - source: "Arnoldi Process"
    target: "Hessenberg Matrix"
    description: "produces"
  - source: "Orthogonal Projection"
    target: "Galerkin Condition"
    description: "satisfies"
  - source: "Oblique Projection"
    target: "target-of-Petrov-Galerkin-Condition"
    description: "defined-by"
  - source: "Error Projection Methods"
    target: "Projection Methods"
    description: "is-a-type-of"
  - source: "Residual Projection Methods"
    target: "Projection Methods"
    description: "is-a-type-of"
---

# Projection Methods for Linear Systems

*Projection methods approximate solutions to linear systems by restricting the search to a subspace. These techniques include orthogonal and oblique projections, which are can be categorized into residual and residual-norm minimization or error projection methods. Krylov subspace methods, such as the Arnoldi process, building an orthonormal basis for the subspace, construct polynomial approximations. polynomial approximations.*

This note explores the theoretical foundations and implementation of [[Projection Methods]] for solving linear systems of the form \(Ax = b\). These methods seek an approximate solution \(x_k\) within an affine subspace, imposing specific constraints to ensure the quality of the approximation.

## Concept

[[Projection Methods]] can be categorized based on the constraints applied to the subspace. If the search is restricted to an orthogonal subspace, the method satisfies the [[Galerkin Condition]], which requires the residual vector \(r_k = b - Ax_k\) to be orthogonal to the subspace. In the case of an [[Oblique Projection]], an approximate solution is found by requiring the orthogonality of the residual to a different subspace, which is defined by the [[Petrov-Galerkin Condition]].

### Residual vs. Error Projection

Depending on whether the method minimizes the \(L_2\)-norm of the residual or the error, the methods are classified as:

1. **[[Residual Projection Methods]]**: These methods minimize \(r_k = b - Ax_k\) over the subspace. For example, the [[Steepest Descent]] algorithm is a type of residual projection method where the subspace is one-dimensional.
2. **[[Error Projection Methods]]**: These methods minimize the error \(e_k = x - x_k\) with respect to an $[\text{A}]$-inner product, certain cases where theA matrix is a [[Symmetric Positive Definite Matrix]] is required. In these cases, the error is the orthogonal projection of the initial error onto the subspace.

## Krylov Subspace Methods

Many modern iterative solvers are [[Krylov Subspace Methods]], which use a subspace of the form: 

$$ \n\mathcal{K}_k(A, r_0) = \text{span} \{ r_0, Ar_0, A^2r_0, \dots, A^{k-1}r_0 \} \n$$

These methods construct polynomial approximations of the solution. The [[Arnoldi Process]] is a key algorithm for building an orthonormal basis for these subspaces, especially for non-Hermitian matrices. The the Arnoldi process produces a [[Hessenberg Matrix]] $H_k$ through a relation:

$$ \n\mathcal{K}_k(A, r_0) = Q_k H_k \n$$

In the event of a "lucky breakdown," the algorithm stops because the subspace becomes invariant under $A$, meaning the exact solution is contained within the subspace, and the resulting approximation is exact.

## Relationships

- [[Projection Methods]] utilize [[Orthogonal Projection]] and [[Oblique Projection]].
- [[Projection Methods]] use [[Krylov Subspace Methods]] as a subspace choice.
- [[Orthogonal Projection Methods]] satisfy the [[Galerkin Condition]].
- [[Oblique Projection Methods]] satisfy the [[Petrov-Galerkin Condition]].
- [[Arnoldi Process]] builds a basis for [[Krylov Subspace Methods]].
- [[Arnoldi Process]] produces a [[Hessenberg Matrix]].
- [[Residual Projection Methods]] is a type of of [[Projection Methods]].
- [[Error Projection Methods]] respect the [[Error Projection Methods]].
- [[Steepest Descent]] is a type of of [[Projection Methods]].

