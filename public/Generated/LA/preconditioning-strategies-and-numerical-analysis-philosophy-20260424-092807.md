---
type: content
title: "Preconditioning Strategies and Numerical Analysis Philosophy"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:28:07.019465400+00:00
summary: "This note explores various preconditioning techniques for iterative solvers and the philosophical definition of numerical analysis. It highlights how preconditioning transforms intractable problems into rapidly convergent ones. The text also argues that numerical analysis is the study of algorithms for continuous mathematics."
tags:
  - numerical-linear-algebra
  - preconditioning
  - numerical-analysis
  - iterative-methods
entities:
  - "[[Preconditioning]]"
  - "[[Iterative Methods]]"
  - "[[Multigrid Methods]]"
  - "[[Krylov Subspace Iteration]]"
  - "[[Numerical Analysis]]"
  - "[[Continuous Mathematics]]"
  - "[[Gaussian Elimination]]"
  - "[[Singular Value Decomposition]]"
  - "[[Least Squares Problems]]"
  - "[[Polynomial Preconditioning]]"
relationships:
  - source: "Preconditioning"
    target: "Iterative Methods"
    description: "improves convergence of"
  - source: "Preconditioning"
    target: "Krylov Subspace Iteration"
    description: "is applied to"
  - source: "Multigrid Methods"
    target: "Preconditioning"
    description: "is a form of"
  - source: "Numerical Analysis"
    target: "Continuous Mathematics"
    description: "studies algorithms for"
  - source: "Gaussian Elimination"
    target: "Preconditioning"
    description: "can serve as"
  - source: "Singular Value Decomposition"
    target: "Numerical Analysis"
    description: "is a fundamental tool in"
---

# Preconditioning Strategies and Numerical Analysis Philosophy

*This note explores various preconditioning techniques for iterative solvers and the philosophical definition of numerical analysis. It highlights how preconditioning transforms intractable problems into rapidly convergent ones. The text also argues that numerical analysis is the study of algorithms for continuous mathematics.*

## Concept

[[Preconditioning]] is the process of transforming a linear system or eigenvalue problem into a form that is more amenable to iterative solvers. The goal is to improve the convergence rate of algorithms like [[Krylov Subspace Iteration]].

Various strategies for preconditioning include:

- **Local Approximations:** Using block-diagonal or [[Domain Decomposition]] methods to capture local effects while ignoring long-range connections.
- **Multilevel Approaches:** [[Multigrid Methods]] use a sequence of coarser grids to treat different frequency components of the error.
- **Operator Splitting:** Approximating a complex operator by splitting it into simpler, more manageable parts (e.g.,[[ADI]] or alternating direction implicit methods).
- **Polynomial Preconditioning:** Approximating the inverse of the matrix using a matrix polynomial $p(A)$ such that $p(A)A$ has better spectral properties.
- **Classical Iterations:** Using one or more steps of methods like Jacobi or SSOR as a preconditioner.

[[Numerical Analysis]] is more than just the study of rounding errors; it is the study of algorithms for [[Continuous Mathematics]]. While rounding errors are a fundamental concern, the deeper mission is to find rapidly convergent algorithms for problems involving real or complex variables.

## Relationships
- [[Preconditioning]] improves convergence of [[Iterative Methods]].
- [[Multigrid Methods]] is a form of [[Preconditioning]].
- [[Numerical Analysis]] studies algorithms for [[Continuous Mathematics]].
- [[Gaussian Elimination]] can serve as a preconditioner.
- [[Singular Value Decomposition]] is a fundamental tool in [[Numerical Analysis]].
