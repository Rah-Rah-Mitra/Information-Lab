---
type: content
title: "Conjugate Gradient Method and Preconditioning"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:26:35.763632400+00:00
summary: "The Conjugate Gradient (CG) method is an iterative algorithm for solving linear systems and minimizing quadratic forms. It achieves rapid convergence by using conjugate search directions rather than orthogonal ones. Preconditioning techniques are used to transform the system to improve the eigenvalue distribution, thereby accelerating convergence."
tags:
  - optimization
  - numerical-linear-algebra
  - conjugate-gradient
  - preconditioning
  - linear-systems
entities:
  - "[[Conjugate Gradient Method]]"
  - "[[Krylov Subspace]]"
  - "[[Preconditioning]]"
  - "[[Eigenvalue Distribution]]"
  - "[[Eigenvalue]]"
  - "[[Expanding Subspace Minimization]]"
  - "[[Fletcher-Reeves Method]]"
  - "[[Fletcher-Reeves Method]]"
  - "[[Residual]]"
  - "[[Search Direction]]"
relationships:
  - source: "Conjugate Gradient Method"
    target: "Krylov Subspace"
    description: "generates iterates in"
  - source: "Conjugate Gradient Method"
    target: "Preconditioning"
    description: "is accelerated by"
  - source: "Conjugate Gradient Method"
    target: "Expanding Subspace Minimization"
    description: "exhibits"
  - source: "Conjugate Gradient Method"
    target: "Residual"
    description: "uses"
  - source: "Conjugate Gradient Method"
    target: "Search Direction"
    description: "produces"
  - source: "Conjugate Gradient Method"
    target: "Fletcher-Reeves Method"
    description: "is generalized to nonlinear case by"
  - source: "Preconditioning"
    target: "Eigenvalue Distribution"
    description: "improves"
  - source: "Preeconditoning"
    target: "Conjugate Gradient Method"
    description: "accelerates"
---

# Conjugate Gradient Method and Preconditioning

*The Conjugate Gradient (CG) method is an iterative algorithm for solving linear systems and minimizing quadratic forms. It achieves rapid convergence by using conjugate search directions rather than orthogonal ones. Preconditioning techniques are used to transform the system to improve the eigenvalue distribution, thereby accelerating convergence.*

The [[Conjugate Gradient Method]] is an iterative method used to solve linear systems of the form \(Ax = b\) or, equivalently, to minimize a strictly convex quadratic function \( \phi(x) = \\frac{1}{2}x^T Ax - b^T x \). Unlike the steepest descent method, which can suffer from slow convergence, the CG method uses a set of search directions that are conjugate with respect to theA matrix, meaning \( p_i^T A p_j = 0 \) for \( i \neq j \). This property ensures that the algorithm terminates in at most \( n \) steps in exact arithmetic.

## Concept

The ### Expanding Subspace Minimization property is a fundamental characteristic of the CG method. As described in Theorem 5.2, the algorithm minimizes the quadratic function over the subspace spanned by the previous search directions. Specifically, after \( k \) steps, the iterate \( x_k \) is the minimizer of \( \phi(x) \) over the set \( span \{ p_0, p_1, \dots, p_{k-1} \) \). This property is closely linked to the [[Krylov Subspace]], defined as:

$$ K_k(r_0) = span \{ r_0, Ar_0, \dots, A^{k-1}r_0 \} $$

Theorem 5.3 establishes that the search directions and the residuals are contained within this Krylov subspace. This optimality property means that the CG method is highly efficient for large-scale problems where the matrix \(A\) is sparse.

## Preconditioning

To accelerate the convergence of the [[Conjugate Gradient Method]], one can use [[Preconditioning]]. This involves a change of variables from \( x \) to \( \hat{x} = C^{-1}x \), which transforms the system to improve the [[Eigenvalue Distribution]] of the matrix. The goal is to \( Preconditioning \) is to choose a matrix \( C \) such that the eigenvalues of the transformed matrix \( C^{-T}AC \) is more favorable (e.g., clustered or having a smaller condition number). 

Common techniques include [[Incomplete Cholesky]] factorization, which approximates the Cholesky factor to maintain sparsity. 

## Nonlinear Variants

For general nonlinear functions, the [[Fletcher-Reeves Method]] provides a nonlinear extension of the conjugate gradient method. It replaces the exact line search with an approximate one and uses the gradient of the function instead of the residual. 

## Relationships

- [[Conjugate Gradient Method]] generates iterates in [[Krylov Subspace]].
- [[Conjugate Gradient Method]] is accelerated by [[Preconditioning]].
- [[Conjugate Gradient Method]] exhibits [[Expanding Subspace Minimization]].
- [[Conjugate Gradient Method]] uses [[Residual]] and [[Search Direction]].
- [[Preconditioning]] improves [[Eigenvalue Distribution]].
- [[Fletcher-Reeves Method]] generalizes [[Conjugate Gradient Method]].
