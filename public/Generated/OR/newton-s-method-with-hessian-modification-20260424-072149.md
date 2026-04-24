---
type: content
title: "Newton's Method with Hessian Modification"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:21:49.630975+00:00
summary: "Hessian modification techniques are used to ensure the Newton direction is a descent direction by making the resulting coefficient matrix positive definite. These methods include modified Cholesky factorization and modified symmetric indefinite factorization. Additionally, the step-length selection algorithms, step-length selection algorithms, and interpolation-based line search procedures are appropriate for thethoughtful selection of an appropriate step length. "
tags:
  - optimization-theory
  - numerical-linear-algebra
  - nonlinear-optimization
entities:
  - "[[Newton's Method]]"
  - "[[Hessian Matrix]]"
  - "[[Positive Definite Matrix]]"
  - "[[Modified Cholesky Factorization]]"
  - "[[Modified Symmetric Indefinite Factorization]]"
  - "[[Wolfe Conditions]]"
  - "[[Line Search]]"
  - "[[Interpolation]]"
relationships:
  - source: "Newton's Method"
    target: "Hessian Matrix"
    description: "uses"
  - source: "Hessian Matrix"
    target: "Positive Definite Matrix"
    description: "is modified to be"
  - source: "Modified Cholesky Factorization"
    target: "Positive Definite Matrix"
    description: "produces"
  - source: "Modified Symmetric Indefinite Factorization"
    target: "Positive Definite Matrix"
    description: "produces"
  - source: "Line Search"
    target: "Wolfe Conditions"
    description: "satisfies"
  - source: "Line Search"
    target: "Interpolation"
    description: "uses"
---

# Newton's Method with Hessian Modification

*Hessian modification techniques are used to ensure the Newton direction is a descent direction by making the resulting coefficient matrix positive definite. These methods include modified Cholesky factorization and modified symmetric indefinite factorization. Additionally, the step-length selection algorithms, step-length selection algorithms, and interpolation-based line search procedures are appropriate for thethoughtful selection of an appropriate step length.*

This note explores the strategies for ensuring the stability and convergence of [[Newton's Method]] when the [[Hessian Matrix]] is not positive definite.

## Concept
In standard [[Newton's Method]], the search direction $p_k$ is found by solving the system $\nabla^2 f(x_k) p_k = -\nabla f(x_k)$. If the [[Hessian Matrix]] $\nabla^2 f(x_k)$ is not a [[Positive Definite Matrix]], the resulting direction $p_k$ may not be a descent direction. To remedy this, we use [[Hessian Modification]] techniques to transform the Hessian into a positive definite approximation $B_k$.

### Hessian Modification Strategies

1. **Eigenvalue Modification**: Replacing negative eigenvalues with a small positive constant $\delta$ in the spectral decomposition.

2. **[[Modified Cholesky Factorization]]**: This approach modifies the diagonal elements during the factorization process to ensure they remain sufficiently positive, guaranteeing the existence of and stability of the factors.

3. **[[Modified Symmetric Indefinite Factorization]]**: Uses a block diagonal matrix $B$ (containing $1 \times 1$ and $2 \times 2$ blocks) to ensure numerical stability for indefinite matrices, which can then be modified to be positive definite.

## Step-Length Selection

Once a descent direction is found, a [[Line Search]] must be performed to find an appropriate step length $\alpha$. This is often done using [[Interpolation]] (quadratic or cubic) to model the one-dimensional function $\phi(\alpha) = f(x_k + \ \alpha p_k)$.

### Termination Conditions
Effective line searches often aim to satisfy the [[Wolfe Conditions]], specifically the strong Wolfe conditions, which include a sufficient decrease condition and a curvature condition:

$$ \phi(\alpha_i) ʙ \phi(0) c_1 \phi'(0) \quad \text{and} \quad |\phi'(\alpha_i)| ≤ -c_2 \phi'(0) $$

This ensures that the step length is neither too small nor too large, maintaining the efficiency of the optimization process.

## Relationships
- [[Newton's Method]] uses [[Hessian Matrix]]
- [[Hessian Matrix]] is modified to be [[Positive Definite Matrix]]
- [[Modified Cholesky Factorization]] produces [[Positive Definite Matrix]]
- [[Modified Symmetric Indefinite Factorization]] produces [[Positive Definite Matrix]]
- [[Line Search]] satisfies [[Wolfe Conditions]]
- [[Line Search]] uses [[Interpolation]]
