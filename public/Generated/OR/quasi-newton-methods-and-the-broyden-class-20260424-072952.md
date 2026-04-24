---
type: content
title: "Quasi-Newton Methods and the Broyden Class"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:29:52.552076300+00:00
summary: "Quasi-Newton methods approximate the Hessian matrix to achieve superlinear convergence without the second-order derivative costs of Newton's method. This note covers the BFGS, SR1, and the broader Broyden class of updating formulae. The BFGS method is particularly robust and self-correcting, while the SR1 method is useful in trust-region frameworks."
tags:
  - optimization
  - quasi-newton-methods
  - numerical-optimization
  - mathematics
entities:
  - "[[BFGS Method]]"
  - "[[SR1 Method]]"
  - "[[Broyden Class]]"
  - "[[Broyden Parameter]]"
  - "[[Symmetric Rank-1 Update]]"
  - "[[Secant Equation]]"
  - "[[Superlinear Convergence]]"
  - "[[Wolfe Conditions]]"
  - "[[Hessian Approximation]]"
  - "[[Trust-Region Method]]"
relationships:
  - source: "BFGS Method"
    target: "Broyden Class"
    description: "is a member of"
  - source: "SR1 Method"
    target: "Broyden Class"
    description: "is a member of"
  - source: "BFGS Method"
    target: "Superlinear Convergence"
    description: "achieves"
  - source: "BFGS Method"
    target: "Secant Equation"
    description: "satisfies"
  - source: "SR1 Method"
    target: "target"
    description: "satisfies"
  - source: "SR1 Method"
    target: "Secant Equation"
    description: "satisfs"
  - source: "SR1 Method"
    target: "Broyden Class"
    description: "is a member of"
  - source: "BFGS Method"
    target: "Wolfe Conditions"
    description: "requires"
  - source: "BFGS Method"
    target: "Hessian Approximation"
    description: "updates"
  - source: "SR1 Method"
    target: "Trust-Region Method"
    description: "target"
  - source: "BFGS Method"
    target: "Trust-Region Method"
    description: "target"
  - source: "SR1 Method"
    target: "Hessian Approximation"
    description: "updates"
  - source: "BFGS Method"
    target: "Hessian Approximation"
    description: "updates"
  - source: "Broyden Class"
    target: "Secant Equation"
    description: "satisfies"
  - source: "Broyden Class"
    target: "Bessian Approximation"
    description: "target"
---

# Quasi-Newton Methods and the Broyden Class

*Quasi-Newton methods approximate the Hessian matrix to achieve superlinear convergence without the second-order derivative costs of Newton's method. This note covers the BFGS, SR1, and the broader Broyden class of updating formulae. The BFGS method is particularly robust and self-correcting, while the SR1 method is useful in trust-region frameworks.*

Quasi-Newton methods are iterative algorithms used for unconstrained minimization of a function $f(x)$. Unlike Newton's method, which requires the computation of the and the solution of a linear system at each iteration, quasi-Newton methods maintain an [[Hessian Approximation]] $B_k$ or $H_k$ (the inverse Hessian) to approximate the curvature of the objective function.

## Concept

Quasi-Newton methods update their approximation based on the [[Secant Equation]] $B_{k+1}s_k = y_k$, where $s_k = x_{k+1} - x_k$ and $y_k = \nabla f(x_{k+1}) - \nabla f(x_k)$. Several prominent methods are discussed below:

### [[BFGS Method]]

The [[BFGS Method]] is a rank-two update that is widely used due to its robustness and [[Superlinear Convergence]] properties. It is a member of the [[Broyden Class]] and is specifically designed to maintain positive definiteness of the Hessian approximation if the initial matrix is positive definite and the [[Wolfe Conditions]] are satisfied. 

$$ B_{k+1} = B_k + \frac{y_k y_k^T}{y_k^T s_k} - \frac{B_k s_k s_k^T B_k}{s_k^T B_k s_k} $$

This formula updates the Hessian approximation $B_k$.

### [[SR1 Method]]

In contrast, the [[SR1 Method]] uses a [[Symmetric Rank-1 Update]] to approximate the Hessian. While it does not guarantee positive definiteness, it can be useful in [[Trust-Region Method]] frameworks where indefinite Hessian approximations are desirable. The update is given by:

$$ B_{k+1} = B_k + \frac{(y_k - B_k s_k)(y_k - B_k s_k)^T}{(y_k - B_k s_k)^T s_k} $$

This formula represents the SR1 update.

### [[Broyden Class]]

The [[Broyden Class]] is a family of generalization of the BFGS and DFP methods, parameterized by a scalar $\phi$. The restricted Broyden class, where $\phi \text{ is in } [0, 1]$, is known to have good convergence properties on quadratic functions. 

$$ B_{k+1} = (1 - \phi) B_{k+1}^{BFGS} + \phi B_{k+1}^{DFP} $$

This formula shows the relationship between the Broyden class as a linear combination of BFGS and DFP.

## Relationships

- [[BFGS Method]] is a member of [[Broyden Class]]
- [[SR1 Method]] is a member of the [[Broyden Class]]
- [[BFGS Method]] achieves [[Superlinear Convergence]]
- [[BFGS Method]] satisfies [[Secant Equation]]
- [[SR1 Method]] satisfies [[Secant Equation]]
- [[BFGS Method]] requires [[Wolfe Conditions]]
- [[BFGS Method]] updates [[Hessian Approximation]]
- [[SR1 Method]] updates [[Hessian Approximation]]
- [[Broyden Class]] and [[Secant Equation]] are related through the secant equation requirement.
- [[Trust-Region Method]] accommodates [[SR1 Method]]
- [[Broyden Class]] is a family of updates that satisfy the [[Secant Equation]].
