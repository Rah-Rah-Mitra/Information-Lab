---
type: content
title: "Parallel Preconditioning and Domain Decomposition"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-19T05:01:10.838684300+00:00
summary: "This note explores various parallel preconditioning techniques, including polynomial, multicoloring, and domain decomposition methods. These strategies aim to enhance computational efficiency by increasing parallelism and reducing the convergence time of iterative solvers. The text specifically details the Schur complement approach within domain decomposition."
tags:
  - parallel-computing
  - numerical-linear-algebra
  - preconditioning
  - domain-decomposition
  - iterative-methods
entities:
  - "[[Parallel Preconditioning]]"
  - "[[Polynomial Preconditioning]]"
  - "[[Polynomial Preconditioning]]"
  - "[[Multicoloring]]"
  - "[[Multicoloring]]"
  - "[[Domain Decomposition]]"
  - "[[Schur Complement]]"
  - "[[Chebyshev Polynomials]]"
  - "[[Red-Black Ordering]]"
  - "[[ILU(0)]]"
  - "[[Jacobi Preconditioning]]"
  - "[[Schwarz Alternating Procedures]]"
relationships:
  - source: "Parallel Preconditioning"
    target: "Polynomial Preconditioning"
    description: "includes"
  - source: "Parallel Preconditioning"
    target: "Multicoloring"
    description: "includes"
  - source: "Parallel Preging"
    target: "Domain Decomposition"
    description: "includes"
  - source: "Polynomial Preconditioning"
    target: "Chebyshev Polynomials"
    description: "uses"
  - source: "Multicoloring"
    target: "Red-Black Ordering"
    description: "specialises to"
  - source: "Domain Decomposition"
    target: "Schur Complement"
    description: "utilizes"
  - source: "Parallel Preconditioning"
    target: "Jacobi Preconditioning"
    description: "includes"
---

# Parallel Preconditioning and Domain Decomposition

*This note explores various parallel preconditioning techniques, including polynomial, multicoloring, and domain decomposition methods. These strategies aim to enhance computational efficiency by increasing parallelism and reducing the convergence time of iterative solvers. The text specifically details the Schur complement approach within domain decomposition.*

This note covers the conceptual substance of parallel preconditioning and domain decomposition techniques used to solve large-scale linear systems in parallel environments.

## Concept
[[Parallel Preconditioning]] is a set of strategies designed to improve the convergence of iterative solvers by increasing the degree of parallelism and exploiting the local architecture of processors. The text discusses several distinct categories:

### 1. Polynomial Preconditioning
[[Polynomial Preconditioning]] replaces the original system with a preconditioned system using a polynomial $P(A)$ of low degree. This is highly effective on vector computers. 

One popular choice is [[Chebyshev Polynomials]], which are designed to minimize the condition number of the preconditioned matrix by making its spectrum as close to the identity as possible. The recurrence relation for these polynomials is given by:

$$ p_{k+1}(x) = 2rac{x - c}{h} p_k(x) - p_{k-1}(x) $$

This formula models the iterative update of the residual vector using the polynomial expansion.

### 2. Multicoloring
[[Multicoloring]] is a graph-theoretic approach used to increase parallelism in relaxation techniques like SOR or ILU. By coloring the nodes of an adjacency graph such that no two adjacent nodes share the same color, all unknowns associated with a same color can be computed simultaneously. 

A common example is [[Red-Black Ordering]], which is a 2-color case used for 5-point finite difference grids. This reordering results in a block-diagonal structure in the matrix, allowing for highly parallel updates.

### 3. Domain Decomposition
[[Domain Decomposition]] methods follow a divide-and-conquer principle, partitioning a large problem into smaller subdomains. These methods can be either vertex-based, edge-based, or element-based. 

One key mechanism is the use of the [[Schur Complement]], which is used in substructuring methods to eliminate interior unknowns and reduce the system to a single system involving only interface variables. The Schur complement matrix $S$ is defined as:

$$ S = A_{22} - A_{21} A_{11}^{-1} A_{12} $$ 

This formula represents the reduction of the global system to an interface-based system.

Other techniques mentioned include [[Jacobi Preconditioning]] and [[Schwarz Alternating Procedures]], which are used to handle the coupling between subdomains.

## Relationships
- [[Parallel Preconditioning]] includes [[Polynomial Preconditioning]]
- [[Parallel Preconditioning]] includes [[Multicoloring]]
- [[Parallel Preconditioning]] includes [[Domain Decomposition]]
- [[Polynomial Preconditioning]] uses [[Chebyshev Polynomials]]
- [[Multicoloring]] specialises to [[Red-Black Ordering]]
- [[Domain Decomposition]] utilizes [[Schur Complement]]
- [[Parallel Preconditioning]] includes [[Jacobi Preconditioning]]
