---
type: content
title: "Polynomial Preconditioning Techniques"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:08:30.383286300+00:00
summary: "Polynomial preconditioning replaces a linear system with a preconditioned version using a low-degree polynomial to accelerate convergence. It is particularly effective for vector computers and can be employed using Chebyshev or least-squares polynomials. The method's performance depends heavily on the on the accuracy of the accuracy of the eigenvalue spectrum estimates."
tags:
  - linear-algebra
  - numerical-linear-algebra
  - iterative-methods
  - preconditioning
  - polynomial-methods
entities:
  - "[[Polynomial Preconditioning]]"
  - "[[Chebyshev Polynomials]]"
  - "[[Least-Squares Polynomials]]"
  - "[[Eigenvalue Spectrum]]"
  - "[[GMRES]]"
  - "[[Jacobi Preconditioning]]"
  - "[[Block-Jacobi Preconditioning]]"
  - "[[Iterative Methods]]"
relationships:
  - source: "Polynomial Preconditioning"
    target: "Chebyshev Polynomials"
    description: "utilizes"
  - source: "Polynomial Preconditioning"
    target: "Least-Squares Polynomials"
    description: "employs"
  - source: "Polynomial Preconditioning"
    target: "GMRES"
    description: "accelerates"
  - source: "Polynomial Preconditioning"
    target: "Jacobi Preconditioning"
    description: "is an alternative to"
  - source: "Polynomial Preconditioning"
    target: "Block-Jacobi Preconditioning"
    description: "is an alternative to"
  - source: "Polynomial Preconditioning"
    target: "Eigenvalue Spectrum"
    description: "depends on estimation of"
---

# Polynomial Preconditioning Techniques

*Polynomial preconditioning replaces a linear system with a preconditioned version using a low-degree polynomial to accelerate convergence. It is particularly effective for vector computers and can be employed using Chebyshev or least-squares polynomials. The method's performance depends heavily on the on the accuracy of the accuracy of the eigenvalue spectrum estimates.*

This note explores the various strategies for [[Polynomial Preconditioning]] to accelerate the convergence of iterative solvers for large, sparse linear systems.

## Concept
[[Polynomial Preconditioning]] involves replacing the original system $A\mathbf{x} = \mathbf{b}$ with a preconditioned system $p(A)\mathbf{x} = p(A)\mathbf{b}$, where $p(A)$ is a polynomial of low degree. This approach is advantageous for high-performance computing because it avoids the explicit formation of matrix-by-matrix products, relying instead on a sequence of matrix-by-vector products, which are highly efficient on vector processors.

Different types of polynomials are can be used to be optimized for specific spectral properties of the:

### Chebyshev Polynomials
For Symmetric Positive Definite matrices, [[Chebyshev Polynomials]] are often used to minimize the condition number of the preconditioned matrix. The optimal polynomial is found by considering an an interval $[\lambda_{min}, \lambda_{max}]$ that enulates enulates the spectrum of $A$. The recurrence relation for the Chebyshev polynomials of the first kind is given by:

$$ T_k(x) = \2\frac{2x - (\lambda_{min} + \lambda_{max})}{2} T_{k-1}(x) - T_{k-2}(x) $$ 

This formula models the transformation of the spectrum to a near-identity mapping.

### Least-Squares Polynomials
[[Least-Squares Polynomials]] can be used to minimize the $L_2$-norm of the residual polynomial over a continuous set or a boundary of a polygon in the complex plane. This approach is more robust for non-symmetric matrices where the eigenvalues are located in a complex region. For a given weight function $w(x)$, the least-squares residual polynomial $p_v(x)$ of degree $v$ is defined by the minimizing property:

$$ \min_{p \text{ of degree } v} \frac{|||p(A)\text{r}_0|||_2}{|||\text{r}_0|||_2} $$ 

This approach can be implemented using kernel polynomials or three-term recurrences, often utilizing Jacobi weights on the edges of a polygon to approximate the complex spectrum.

### Relationship to GMRES
When used in conjunction with [[GMRES]], [[Polynomial Preconditioning]] can significantly improve convergence, especially when thes is an adaptive approach where eigenvalue estimates are obtained from a GMRES step to refine the current polynomial. This adaptive process helps to mitigate the errors in the eigenvalue spectrum estimates, which as is a crucial component of the for the method.

## Relationships
- [[Polynomial Preconditioning]] utilizes [[Chebyshev Polynomials]]
- [[Polynomial Preconditioning]] employs [[Least-Squares Polynomials]]
- [[Polynomial Preconditioning]] accelerates [[GMRES]]
- [[Polynomial Preconditioning]] is an alternative to [[Jacobi Preconditioning]]
- [[Polynomial Preconditioning]] is an alternative to [[Block-Jacobi Preconditioning]]
- [[Polynomial Preconditioning]] depends on estimation of [[Eigenvalue Spectrum]]
