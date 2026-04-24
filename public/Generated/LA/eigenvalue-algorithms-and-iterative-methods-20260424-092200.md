---
type: content
title: "Eigenvalue Algorithms and Iterative Methods"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:22:00.671555800+00:00
summary: "This note covers fundamental iterative methods for computing eigenvalues and eigenvectors, including power iteration, inverse iteration, and the QR algorithm. These methods are essential for large-scale matrix computations where direct methods are computationally prohibitive. The note highlights the relationship between the convergence rates and eigenvalue separation."
tags:
  - numerical-linear-algebra
  - eigenvalue-algorithms
  - iterative-methods
entities:
  - "[[Eigenvalue Algorithms]]"
  - "[[Power Iteration]]"
  - "[[Inverse Iteration]]"
  - "[[Rayleigh Quotient]]"
  - "[[QR Algorithm]]"
  - "[[Simultaneous Iteration]]"
  - "[[Pseudospectrum]]"
  - "[[Bauer-Fike Theorem]]"
relationships:
  - source: "Power Iteration"
    target: "Eigenvalue Algorithms"
    description: "is a basic"
  - source: "Inverse Iteration"
    target: "Eigenvalue Algorithms"
    description: "is an accelerated"
  - source: "Rayleigh Quotient"
    target: "Eigenvalue Algorithms"
    description: "improves"
  - source: "QR Algorithm"
    target: "Eigenvalue Algorithms"
    description: "is a sophisticated"
  - source: "Simultaneous Iteration"
    target: "target_not_found"
    description: "is equivalent to"
  - source: "Simultaneous Iteration"
    target: "QR Algorithm"
    description: "is equivalent to"
---

# Eigenvalue Algorithms and Iterative Methods

*This note covers fundamental iterative methods for computing eigenvalues and eigenvectors, including power iteration, inverse iteration, and the QR algorithm. These methods are essential for large-scale matrix computations where direct methods are computationally prohibitive. The note highlights the relationship between the convergence rates and eigenvalue separation.*

This note explores the mathematical foundations and algorithmic implementations of various iterative methods used to find eigenvalues and eigenvectors.

## Concept
Iterative methods are used when direct methods are too expensive or when matrices are large. The primary goal is to find a set of vectors and scalars that satisfy the equation $A v = \lambda v$.

### Power Iteration
[[Power Iteration]] is the simplest method, starting with an initial vector $v^{(0)}$ and repeatedly applying the matrix $A$ and normalizing. It converges to the eigenvector corresponding to the largest eigenvalue in absolute value. 

$$ v^{(k)} = \frac{A^k v^{(0)}}{\|v^{(k)}\|} $$

The convergence rate is linear and depends on the ratio of the magnitude of the largest two eigenvalues: $\left| \frac{\lambda_1}{\lambda_2} \right|$.

### Inverse Iteration
[[Inverse Iteration]] accelerates the convergence by applying the process to $(A - \rho I)^{-1}$, where $\rho$ is an estimate of an eigenvalue. This method is effectively a form of shifted power iteration. 

$$ (A - \rho I) w = v^{(k-1)} $$ 

If $\rho$ is close to an eigenvalue, the convergence is extremely rapid.

### Rayleigh Quotient
The [[Rayleigh Quotient]] $r(x)$ provides a natural eigenvalue estimate from an eigenvector estimate $x$. For a real symmetric matrix $A$, it is defined as:

$$ r(x) = \\frac{x^T A x}{x^T x} $$ 

At an eigenvector, the gradient of the Rayleigh quotient is zero, and it is a quadratically accurate estimate of the eigenvalue.

### Rayleigh Quotient Iteration
[[Rayleigh Quotient Iteration]] combines the ideas of inverse iteration and the Rayleigh quotient to achieve cubic convergence. It uses the current Rayleigh quotient as the shift in the next step of the            

$$ A(k) = R(k)Q(k) + \rho(k)I $$

This method is highly efficient for finding specific eigenvalues.

### QR Algorithm
[[QR Algorithm]] is a central method in numerical linear algebra. The "pure" version involves repeatedly taking the QR factorization of $A$ and recombining the factors in reverse order. 

$$ A^{(k)} = R^{(k)}Q^{(k)} \n$$ 

In practice, the [[Simultaneous Iteration]] (or block power iteration) is used to find the basis for the subspace spanned by the eigenvectors. The [[QR Algorithm]] is equivalent to simultaneous iteration applied to the identity matrix. 

### Pseudospectrum
While eigenvalues are the most important, the [[Pseudospectrum]] describes the behavior of a matrix under small perturbations. The $\epsilon$-pseudospectrum is the $\epsilon$-norm pseudospectrum, defined as the set of numbers $z$ such that $z$ is an eigenvalue of $A + \triangle A$ for some $\triangle A$ with $\text{norm}(\triangle A) < \text{epsilon}$.

## Relationships
- [[Power Iteration]] is a basic [[Eigenvalue Algorithms]]
- [[Power Iteration]] converges to the largest eigenvalue
- [[Inverse Iteration]] is an accelerated [[Eigenvalue Algorithms]]
- [[Rayleigh Quotient]] improves [[Inverse Iteration]]
- [[Rayleigh Quotient Iteration]] is a sophisticated [[Eigenvalue Algorithms]]
- [[QR Algorithm]] is equivalent to [[Simultaneous Iteration]]
