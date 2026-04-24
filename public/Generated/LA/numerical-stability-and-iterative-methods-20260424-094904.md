---
type: content
title: "Numerical Stability and Iterative Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:49:04.568675600+00:00
summary: "This note covers the fundamental concepts of numerical stability, backward error analysis, and the evolution of iterative methods for large-scale linear systems. It highlights the relationship between conditioning and stability, and the transition from direct methods to Krylov subspace methods like Lanczos and GMRES. The text emphasizes the statistical nature of practical stability in Gaussian elimination."
tags:
  - numerical-analysis
  - linear-algebra
  - stability-analysis
  - iterative-methods
  - krylov-subspace-methods
entities:
  - "[[Numerical Stability]]"
  - "[[Backward Error Analysis]]"
  - "[[Condition Number]]"
  - "[[Gaussian Elimination]]"
  - "[[Krylov Subspace Methods]]"
  - "[[Lanczos Iteration]]"
  - "[[Arnoldi Iteration]]"
  - "[[GMRES]]"
  - "[[Conjugate Gradients]]"
  - "[[QR Algorithm]]"
relationships:
  - source: "Numerical Stability"
    target: "Backward Error Analysis"
    description: "is analyzed via"
  - source: "Gaussian Elimination"
    target: "Numerical Stability"
    description: "is evaluated for"
  - source: "Krylov Subspace Methods"
    target: "Lanczos Iteration"
    description: "includes"
  - source: "Krylov Subspace Methods"
    target: "Arnoldi Iteration"
    description: "includes"
  - source: "Krylov Subspace Methods"
    target: "GMRES"
    description: "includes"
  - source: "Krylov Subspace Methods"
    target: "Conjugate Gradients"
    description: "includes"
  - source: "Lanczos Iteration"
    target: "Krylov Subspace Methods"
    description: "is a type of"
  - source: "Arnoldi Iteration"
    target: "Krylov Subspace Methods"
    description: "is a type of"
  - source: "GMRES"
    target: "Krylov Subspace Methods"
    description: "description of"
  - source: "Conjugate Gradients"
    target: "Krylov Subspace Methods"
    description: "is a type of"
  - source: "QR Algorithm"
    target: "Numerical Stability"
    description: "is evaluated for"
  - source: "Condition Number"
    target: "Numerical Stability"
    description: "is related to"
---

# Numerical Stability and Iterative Methods

*This note covers the fundamental concepts of numerical stability, backward error analysis, and the evolution of iterative methods for large-scale linear systems. It highlights the relationship between conditioning and stability, and the transition from direct methods to Krylov subspace methods like Lanczos and GMRES. The text emphasizes the statistical nature of practical stability in Gaussian elimination.*

This note provides an overview of the core principles of numerical stability and the development of iterative methods in scientific computing.

## Concept

[[Numerical Stability]] refers to the degree to which an algorithm's output is affected by small perturbations in the input or rounding errors during computation. A central tool for assessing this is [[Backward Error Analysis]], which seeks to find a solution that is the exact solution to a slightly perturbed problem. This approach is particularly powerful in matrix factorizations like the Householder triangularization, where individual factors might appear inaccurate, but their product remains highly accurate.

The [[Condition Number]] of a problem is a measure of how sensitive a single problem is to perturbations. While [[Gaussian Elimination]] is theoretically predicted to be unstable due to exponentially compounding rounding errors, computational experience and statistical evidence suggest that it is often stable in practice, a phenomenon linked to the size of the growth factor.

For large-scale systems, direct methods like [[QR Algorithm]] for eigenvalue problems or Gaussian elimination become computationally expensive. This leads to the use of [[Krylov Subspace Methods]], which project the problem onto low-dimensional subspaces to approximate solutions. Key algorithms within this family include:

- [[Lanczos Iteration]]: A method for symmetric matrices that builds a basis for the Krylov subspace.
- [[Arnoldi Iteration]]: A more general version of the [[Lanczos Iteration]] that works for non-symmetric matrices.
- [[GMRES]]: A method proposed by Saad and Schultz for solving linear systems using Krylov subspaces.
- [[Conjugate Gradients]]: A [[Lanczos Iteration]] related method used for solving symmetric positive definite systems.

## Relationships

- [[Numerical Stability]] is analyzed via [[Backward Error Analysis]].
- [[Gaussian Elimination]] is evaluated for [[Numerical Stability]].
- [[Krylov Subspace Methods]] includes [[Lanczos Iteration]], [[Arnoldi Iteration]], [[GMRES]], and [[Conjugate Gradients]].
- [[QR Algorithm]] is evaluated for [[Numerical Stability]].
- [[Condition Number]] is related to [[Numerical Stability]].
