---
type: content
title: "Krylov Subspace Methods for Nonsymmetric Systems"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-19T04:59:39.768078800+00:00
summary: "This note covers various Krylov subspace methods, including Lanczos-based algorithms like BCG, QMR, and CGS, as well as GMRES variants. It explains how these methods handle nonsymmetric matrices and discusses the advantages and stability of different preconditioning strategies. The text also introduces flexible GMRES (FGMRES) for cases where the preconditioner varies per step."
tags:
  - linear-algebra
  - iterative-methods
  - krylov-subspace
  - numerical-linear-algebra
entities:
  - "[[Lanczos Algorithm]]"
  - "[[Biconjugate Gradient]]"
  - "[[Quasi-Minimal Residual]]"
  - "[[Conjugate Gradient Squared]]"
  - "[[GMRES]]"
  - "[[Biconjugate Gradient Stabilized]]"
  - "[[Conjugate Gradient Normal Equations]]"
  - "[[Conjugate Gradient Normal Error]]"
  - "[[Flexible GMRES]]"
relationships:
  - source: "Lanczos Algorithm"
    target: "Biconjugate Gradient"
    description: "is the basis for"
  - source: "Lanczos Algorithm"
    target: "Quasi-Minimal Residual"
    description: "leads to"
  - source: "Lanczos Algorithm"
    target: "Conjugate Gradient Squared"
    description: "leads to"
  - source: "Biconjugate Gradient"
    target: "Conjugate Gradient Squared"
    description: "is related to"
  - source: "Biconjugate Gradient"
    target: "Biconjugate Gradient Stabilized"
    description: "is improved by"
  - source: "Conjugate Gradient Squared"
    target: "Biconjugate Gradient Stabilized"
    description: "is stabilized by"
  - source: "Conjugate Gradient Normal Equations"
    target: "Lanczos Algorithm"
    description: "applies to"
  - source: "Conjugate Gradient Normal Error"
    target: "Lanczos Algorithm"
    description: "description of"
  - source: "GMRES"
    target: "Flexible GMRES"
    description: "is generalized to"
  - source: "Quasi-Minimal Residual"
    target: "GMRES"
    description: "is an alternative to"
---

# Krylov Subspace Methods for Nonsymmetric Systems

*This note covers various Krylov subspace methods, including Lanczos-based algorithms like BCG, QMR, and CGS, as well as GMRES variants. It explains how these methods handle nonsymmetric matrices and discusses the advantages and stability of different preconditioning strategies. The text also introduces flexible GMRES (FGMRES) for cases where the preconditioner varies per step.*

This note explores the iterative methods used to solve nonsymmetric linear systems using Krylov subspace techniques.

## Concept
Krylov subspace methods aim to find an approximate solution to a linear system $Ax = b$ by searching within the subspace $\mathcal{K}_m(A, r_0) = \text{span}\{r_0, Ar_0, A^2r_0, \dots, A^{m-1}r_0\}$. For nonsymmetric matrices, standard methods like the Conjugate Gradient (CG) algorithm must be be replaced by biorthogonalization or minimization-based approaches.

### Lanczos-Based Methods
[[Lanczos Algorithm]] is a fundamental process for generating a biorthogonal sequence of vectors. From this, several algorithms are derived:
- [[Biconjugate Gradient]] (BCG): Solves the system by projecting onto the Krylov subspace while maintaining biorthogonality between the primary and dual sequences. It requires matrix-vector products with both $A$ and $A^T$.
- [[Quasi-Minimal Residual]] (QMR): An alternative to GMRES that uses the Lanczos process to obtain a least-squares solution from the Krylov subspace. It is often more stable than BCG.
- [[Conjugate Gradient Squared]] (CGS): A method that squares the residual polynomial to accelerate convergence, but it is prone to significant rounding errors and irregular convergence.
- [[Biconjugate Gradient Stabilized]] (BiCGSTAB): A variation of CGS designed to "smooth" the convergence behavior by introducing a stabilizing polynomial.

### GMRES and Preconditioning
[[GMRES]] (Generalized Minimal Residual) minimizes the residual norm over the Krylov subspace. For nonsymmetric systems, preconditioning is essential to improve convergence. 

Preconditioning can be applied in three ways:
1. **Left Preconditioning**: $M^{-1}Ax = M^{-1}b$.
2. **Right Preconditioning**: $AxM^{-1} = bM^{-1}$.
3. **Split Preconditioning**: $M_1^{-1}AM_2^{-1}x = M_1^{-1}bM_2^{-1}$.

While left and right preconditioning have identical spectra, they differ in the optimality properties: left preconditioned GMRES minimizes the preconditioned residual norm, whereas right preconditioned GMRES minimizes the original residual norm.

[[Flexible GMRES]] (FGMRES) is a specialized variant that allows the preconditioner $M$ to change at each step, which is only possible with the right preconditioned formulation.

### Normal Equations Approaches
For systems where $A$ is difficult to handle, one can solve the normal equations $A^TAx = A^Tb$. This leads to methods like [[Conjugate Gradient Normal Equations]] (CGNR) and [[Conjugate Gradient Normal Error]] (CGNE), also known as Craig's method. 
- [[CGNR]] minimizes the residual norm $||r_m||_2$.
- [[CGNE]] minimizes the error norm $||e_m||_2$.

## Relationships
- [[Lanczos Algorithm]] leads to [[Biconjugate Gradient]]
- [[Lanczos Algorithm]] leads to [[Quasi-Minimal Residual]]
- [[Lanczos Algorithm]] leads to [[Conjugate Gradient Squared]]
- [[Biconjugate Gradient]] is related to [[Conjugate Gradient Squared]]
- [[Biconjugate Gradient]] is stabilized by [[Biconjugate Gradient Stabilized]]
- [[Conjugate Gradient Normal Equations]] applies to [[Lanczos Algorithm]]
- [[Conjugate Gradient Normal Error]] is a description of [[Lanczos Algorithm]]
- [[GMRES]] is generalized to [[Flexible GMRES]]
- [[Quasi-Minimal Residual]] is an alternative to [[GMRES]]
