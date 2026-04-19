---
type: content
title: "Iterative Methods and Projection Techniques"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-19T04:58:52.216593900+00:00
summary: "This note covers fundamental iterative methods for solving linear systems, including Jacobi, Gauss-Seidel, and SOR, alongside projection-based methods like Krylov subspace techniques. It explains the convergence criteria, the role of preconditioning, and the mathematical foundations of orthogonal and oblique projections. The content also details the Arnoldi process for constructing orthonormal bases in Krylov subspaces."
tags:
  - numerical-linear-algebra
  - iterative-methods
  - projection-methods
  - krylov-subspace
  - linear-systems
entities:
  - "[[Jacobi Iteration]]"
  - "[[Gauss-Seidel Iteration]]"
  - "[[Successive Over-Relaxation]]"
  - "[[Krylov Subspace]]"
  - "[[Arnoldi Process]]"
  - "[[Projection Methods]]"
  - "[[Preconditioning]]"
  - "[[Symmetric Positive Definite]]"
  - "[[Gershgorin Discs]]"
  - "[[Petrov-Galerkin Conditions]]"
relationships:
  - source: "Jacobi Iteration"
    target: "Gauss-Seidel Iteration"
    description: "is a simpler form of"
  - source: "Gauss-Seidel Iteration"
    target: "Successive Over-Relaxation"
    description: "is a special case of"
  - source: "Successive Over-Relaxation"
    target: "Preconditioning"
    description: "can be viewed as"
  - source: "Krylov Subspace"
    target: "Arnoldi Process"
    description: "is constructed by"
  - source: "Projection Methods"
    target: "Petrov-Galerkin Conditions"
    description: "satisfy"
  - source: "Jacobi Iteration"
    target: "Preconditioning"
    description: "uses a diagonal"
  - source: "Gauss-Seidel Iteration"
    target: "Preconditioning"
    description: "uses a lower triangular part of"
  - source: "Gershgorin Discs"
    target: "Jacobi Iteration"
    description: "provides convergence bounds for"
---

# Iterative Methods and Projection Techniques

*This note covers fundamental iterative methods for solving linear systems, including Jacobi, Gauss-Seidel, and SOR, alongside projection-based methods like Krylov subspace techniques. It explains the convergence criteria, the role of preconditioning, and the mathematical foundations of orthogonal and oblique projections. The content also details the Arnoldi process for constructing orthonormal bases in Krylov subspaces.*

This note explores the mathematical frameworks used to solve large-scale linear systems through iterative and projection-based approaches.

## Concept
Iterative methods solve the system \(Ax = b\) by generating a sequence of approximations \(x^{(i)}\) that converge to the true solution. Basic methods include the [[Jacobi Iteration]], which updates components based on the previous iterate, and the [[Gauss-Seidel Iteration]], which uses updated components immediately to accelerate convergence.

[[Successive Over-Relaxation]] (SOR) generalizes these by introducing an acceleration parameter \(\omega\). The convergence of these methods is often analyzed using [[Gershgorin Discs]], which provide bounds on the eigenvalues of the matrix, ensuring that the spectral radius of the iteration matrix is less than unity.

## Projection Methods
[[Projection Methods]] seek an approximate solution within a specific subspace. If the subspace is the same for both the approximation and the constraints, the method is is an orthogonal projection, often called the [[Galerkin Conditions]]. If the subspaces differ, it is an oblique projection, governed by the [[Petrov-Galerkin Conditions]].

[[Krylov Subspace]] methods, such as those using the [[Arnoldi Process]], build an orthonormal basis for the subspace \(K_n(A, r_0) = \\text{span} \{r_0, Ar_0, A^2r_0, 
\dots, A^{n-1}r_0\}\). The [[Arnoldi Process]] is used to transform a dense matrix into a Hessenberg form, providing an efficient way to approximate eigenvalues and solve linear systems.

## Convergence and Preconditioning
Many iterative methods are equivalent to a fixed-point iteration on a a [[Preconditioning]] matrix. For example, the Jacobi method uses the diagonal of \(A\) as a preconditioner, while Gauss-Seidel uses its lower triangular part. A method is considered successful if the spectral radius of its iteration matrix is less than one.

## Relationships
- [[Jacobi Iteration]] is a simpler form of [[Gauss-Seidel Iteration]]
- [[Gauss-Seidel Iteration]] is a special case of [[Successive Over-Relaxation]]
- [[Successive Over-Relaxation]] can be viewed as [[Preconditioning]]
- [[Krylov Subspace]] is constructed by the [[Arnoldi Process]]
- [[Projection Methods]] satisfy [[Petrov-Galerkin Conditions]]
- [[Jacobi Iteration]] uses a diagonal [[Preconditioning]] matrix
- [[Gauss-Seidel Iteration]] uses a lower triangular part of [[Preconditioning]] matrix
- [[Gershgorin Discs]] provides convergence bounds for [[Jacobi Iteration]]
