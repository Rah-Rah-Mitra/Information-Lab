---
type: content
title: "Iterative Methods For Solving Linear Systems"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T08:37:48.431215500+00:00
summary: "Iterative methods solve linear systems by refining approximate solutions through successive updates. These techniques, including Jacobi, Gauss-Seidel, and SOR, are categorized by their how they split the matrix and update components. Convergence depends on the spectral radius of the iteration matrix and is influenced by matrix properties like diagonal dominance and Property A."
tags:
  - linear-algebra
  - numerical-methods
  - iterative-solvers
  - partial-differential-equations
entities:
  - "[[Iterative Methods]]"
  - "[[Jacobi Iteration]]"
  - "[[Jacobi Iteration Matrix]]"
  - "[[Gauss-Seidel Iteration]]"
  - "[[Successive Over Relaxation]]"
  - "[[Symmetric Successive Over Relaxation]]"
  - "[[Block Jacobi Iteration]]"
  - "[[Property A]]"
  - "[[Consistently Ordered Matrix]]"
  - "[[Gershgorin Discs]]"
  - "[[Spectral Radius]]"
  - "[[Richardson's Iteration]]"
relationships:
  - source: "Iterative Methods"
    target: "Jacobi Iteration"
    description: "includes"
  - source: "Iterative Methods"
    target: "Gauss-Seidel Iteration"
    description: "includes"
  - source: "Iterative Methods"
    target: "Successive Over Relaxation"
    description: "generalises"
  - source: "Jacobi Iteration"
    target: "Jacobi Iteration Matrix"
    description: "is defined by"
  - source: "Gauss-Seidel Iteration"
    target: "Jacobi Iteration Matrix"
    description: "uses"
  - source: "Successive Over Relaxation"
    target: "description of a relaxation method based on a splitting of A"
    description: "depends on"
  - source: "Block Jacobi Iteration"
    target: "Iterative Methods"
    description: "is a block variant of"
  - source: "Property A"
    target: "Consistently Ordered Matrix"
    description: "is satisfied by"
  - source: "Gershgorin Discs"
    target: "Iterative Methods"
    description: "helps bound eigenvalues for"
  - source: "Spectral Radius"
    target: "Iterative Methods"
    description: "determines convergence of"
  - source: "Richardson's Iteration"
    target: "Iterative Methods"
    description: "is an example of"
  - source: "Successive Over Relaxation"
    target: "Symmetric Successive Over Relaxation"
    description: "can be extended to"
---

# Iterative Methods For Solving Linear Systems

*Iterative methods solve linear systems by refining approximate solutions through successive updates. These techniques, including Jacobi, Gauss-Seidel, and SOR, are categorized by their how they split the matrix and update components. Convergence depends on the spectral radius of the iteration matrix and is influenced by matrix properties like diagonal dominance and Property A.*

This note explores the fundamental iterative methods used to solve linear systems of the form \(Ax = b\), where \(A\) is a coefficient matrix, \(b\) is the right-hand side vector, and \(x\) is the vector of unknowns. 

## Concept

Iterative methods approach a solution by generating a sequence of iterates \(x^{(m)}\), where each step involves modifying components of the vector to reduce the residual vector \(r^{(m)} = b - Ax^{(m)}\). 

### Point Relaxation Methods

Common point relaxation methods include the [[Jacobi Iteration]], which updates each component independently using the previous iterate, and the [[Gauss-Seidel Iteration]], which uses updated components immediately within the same step. 

Both methods can be expressed as a fixed-point iteration on a preconditioned system: 

$$ x^{(m+1)} = Z^{-1} r^{(m)} $$

where \(Z\) is the preconditioning matrix. For the [[Jacobi Iteration]], the preconditioning matrix is the diagonal of \(A\). 

[[Successive Over Relaxation]] (SOR) generalizes Gauss-Seidel by introducing an acceleration parameter \(\omega\). The [[Symmetric Successive Over Relaxation]] (SSOR) method is a two-sweep process involving a forward and backward SOR step. 

### Block Relaxation Methods

[[Block Jacobi Iteration]] and block Gauss-Seidel are generalizations where whole subvectors are updated at once. This is particularly useful in parallel computing and for solving systems arising from finite difference discretizations of PDEs. 

### Convergence and Matrix Properties

Convergence is determined by the [[Spectral Radius]] of the [[Jacobi Iteration Matrix]] (or the iteration matrix in general). An iteration converges if the spectral radius is less than unity: 

$$ \rho(M) < 1 $$

Several matrix properties facilitate convergence or provide bounds on eigenvalues, such as [[Gershgorin Discs]], which provide a location for eigenvalues in the complex plane. 

[[Property A]] and [[Consistently Ordered Matrix]] are specific structural properties that which allow for a relationship between the eigenvalues of SOR and Jacobi methods. For a [[Consistently Ordered Matrix]], the eigenvalues of the SOR iteration matrix are related to the through the following equation: 

$$ \lambda + \n\omega - 1 $$ 

where \	ext{constant} \) is the eigenvalue of the Jacobi matrix. 

[[Richardson's Iteration]] is a simple example of a relaxation method where a scalar parameter is used to accelerate convergence. 

## Relationships

- [[Iterative Methods]] includes [[Jacobi Iteration]], [[Gauss-Seidel Iteration]], and [[Successive Over Relaxation]].
- [[Gauss-Seidel Iteration]] uses the [[Jacobi Iteration Matrix]] to update iterates.
- [[Successive Over Relaxation]] can be extended to [[Symmetric Successive Over Relaxation]].
- [[Block Jacobi Iteration]] is a block variant of [[Iterative Methods]].
- [[Property A]] is satisfied by [[Consistently Ordered Matrix]].
- [[Gershgorin Discs]] helps bound eigenvalues for [[Iterative Methods]].
- [[Spectral Radius]] determines convergence of [[Iterative Methods]].
- [[Richardson's Iteration]] is an example of [[Iterative Methods]].
