---
type: content
title: "Krylov Subspace Projection Methods"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-19T04:59:15.470836100+00:00
summary: "This note covers various iterative projection methods for solving linear systems, including FOM, GMRES, and Lanczos-based algorithms. It details the mathematical relationships between these methods, such as the convergence properties and the relationship between FOM and GMRES residuals. It also discusses practical considerations like restarting and truncation to manage memory and computational costs."
tags:
  - linear-algebra
  - iterative-methods
  - krylov-subspace
  - numerical-linear-algebra
entities:
  - "[[Krylov Subspace]]"
  - "[[Full Orthogonalization Method]]"
  - "[[GMRES]]"
  - "[[Full Orthogonalization Method]]"
  - "[[Lanczos Algorithm]]"
  - "[[Conjugate Gradient]]"
  - "[[Arnoldi Process]]"
  - "[[Incomplete Orthogonalization Method]]"
  - "[[Incomplete Orthogonalization Method]]"
  - "[[Direct Incomplete Orthogonalization Method]]"
  - "[[Quasi-GMRES]]"
  - "[[Lanczos Biorthogonalization]]"
relationships:
  - source: "Full Orthogonalization Method"
    target: "Krylov Subspace"
    description: "is a projection method onto"
  - source: "GMRES"
    target: "Krylov Subspace"
    description: "is a projection method onto"
  - source: "Lanczos Algorithm"
    target: "Krylov Subspace"
    description: "uses"
  - source: "Arnoldi Process"
    target: "Krylov Subspace"
    description: "generates basis for"
  - source: "Conjugate Gradient"
    target: "Krylov Subspace"
    description: "is a projection method onto"
  - source: "Incomplete Orthogonalization Method"
    target: "Krylov Subspace"
    description: "uses truncated basis from"
  - source: "GMRES"
    target: "Full Orthogonalization Method"
    description: "relates to"
  - source: "Lanczos Algorithm"
    target: "Conjugate Gradient"
    description: "is a simplification of Arnoldi for symmetric matrices, leading to"
  - source: "Lanczos Biorthogonalization"
    target: "Krylov Subspace"
    description: "uses biorthogonal bases for"
  - source: "Direct Incomplete Orthogonalization Method"
    target: "Incomplete Orthogonalization Method"
    description: "is a progressive version of"
---

# Krylov Subspace Projection Methods

*This note covers various iterative projection methods for solving linear systems, including FOM, GMRES, and Lanczos-based algorithms. It details the mathematical relationships between these methods, such as the convergence properties and the relationship between FOM and GMRES residuals. It also discusses practical considerations like restarting and truncation to manage memory and computational costs.*

This note explores the various iterative methods used to solve linear systems by projecting onto a [[Krylov Subspace]].

## Concept
An iterative method based on the [[Krylov Subspace]] $\mathcal{K}_m(A, r_0)$ is defined as an orthogonal projection method that seeks an approximate solution $x_m$ within an affine space. For the [[Full Orthogonalization Method]] (FOM), the approximate solution is found by imposing the Galerkin condition.

### FOM and GMRES
The [[Full Orthogonalization Method]] (FOM) and the [[GMRES]] (Generalized Minimal Residual) method are closely related. While FOM uses the Galerkin condition, [[GMRES]] minimizes the 2-norm of the residual over the Krylov subspace. 

One significant relationship between the two is the residual norm relationship: 

$$ \|r_m^{GMRES} \| = \|r_m^{FOM} \| \cdot |\gamma_m| $$

where $\gamma_m$ is a scalar related to the rotation matrices used in the QR factorization of the Hessenberg matrix. 

### Lanczos and Conjugate Gradient
When the matrix $A$ is symmetric, the [[Arnoldi Process]] simplifies into the [[Lanczos Algorithm]]. This leads to a three-term recurrence, which is much more memory-efficient than the full Arnoldi process. 

For symmetric positive definite matrices, the [[Conjugate Gradient]] algorithm is the most prominent method. It can be viewed as a a variation of the direct version of the incomplete orthogonalization method (DIOM) and is mathematically equivalent to FOM in the symmetric case.

### Incomplete Orthogonalization
To manage the memory and computational costs of full orthogonalization, methods like the [[Incomplete Orthogonalization Method]] (IOM) and its progressive version, the [[Direct Incomplete Orthogonalization Method]] (DIOM), are used. These methods truncate the orthogonalization process, which can lead to approximate minimization of the residual norm.

### Biorthogonalization
For non-symmetric matrices, the [[Lanczos Biorthogonalization]] algorithm builds a pair of biorthogonal bases for two subspaces, rather than a single orthogonal basis. This is a useful extension for non-symmetric systems.

## Relationships
- [[Full Orthogonalization Method]] is a projection method onto [[Krylov Subspace]]
- [[GMRES]] is a projection method onto [[Krylov Subspace]]
- [[Lanczos Algorithm]] uses [[Krylov Subspace]]
- [[Arnoldi Process]] generates basis for [[Krylov Subspace]]
- [[Conjugate Gradient]] is a projection method onto [[Krylov Subspace]]
- [[Incomplete Orthogonalization Method]] uses truncated basis from [[Krylov Subspace]]
- [[GMRES]] relates to [[Full Orthogonalization Method]]
- [[Lanczos Algorithm]] is a simplification of Arnoldi for symmetric matrices, leading to [[Conjugate Gradient]]
- [[Lanczos Biorthogonalization]] uses biorthogonal bases for [[Krylov Subspace]]
- [[Direct Incomplete Orthogonalization Method]] is a progressive version of [[Incomplete Orthogonalization Method]]
