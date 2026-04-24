---
type: content
title: "Inexact Newton Methods and Convergence Analysis"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:30:20.043508500+00:00
summary: "Inexact Newton methods approximate the Newton step by solving the Hessian-based linear system iteratively, often using Conjugate Gradient (CG) techniques. These methods achieve superlinear or quadratic convergence depending on the forcing sequence used to control theness of the solution. They are particularly effective for large-scale problems where explicit Hessian storage or factorization is highly expensive."
tags:
  - optimization
  - numerical-methods
  - large-scale-optimization
  - newton-methods
  - quasi-newton
entities:
  - "[[Inexact Newton Methods]]"
  - "[[BFGS Method]]"
  - "[[SR1 Method]]"
  - "[[Conjugate Gradient Method]]"
  - "[[Trust-Region Newton-CG Method]]"
  - "[[Trust-Region Newton-Lanczos Method]]"
  - "[[Forcing Sequence]]"
  - "[[Superlinear Convergence]]"
  - "[[Quadratic Convergence]]"
  - "[[Hessian]]"
  - "[[Cauchy Point]]"
  - "[[Steihaug's Method]]"
relationships:
  - source: "Inexact Newton Methods"
    target: "Conjugate Gradient Method"
    description: "uses"
  - source: "Inexact Newton Methods"
    target: "Superlinear Convergence"
    description: "achieves"
  - source: "Inexact Newton Methods"
    target: "Quadratic Convergence"
    description: "achieves"
  - source: "Inexact Newton Methods"
    target: "Forcing Sequence"
    description: "controlled by"
  - source: "Trust-Region Newton-CG Method"
    target: "Steihaug's Method"
    description: "implements"
  - source: "Trust-Region Newton-CG Method"
    target: "Conjugate Gradient Method"
    description: "uses"
  - source: "Trust-Region Newton-CG Method"
    target: "Cauchy Point"
    description: "guarantees reduction at least as good as"
  - source: "BFGS Method"
    target: "Superlinear Convergence"
    description: "exhibits"
  - source: "SR1 Method"
    target: "Superlinear Convergence"
    description: "exhibits"
---

# Inexact Newton Methods and Convergence Analysis

*Inexact Newton methods approximate the Newton step by solving the Hessian-based linear system iteratively, often using Conjugate Gradient (CG) techniques. These methods achieve superlinear or quadratic convergence depending on the forcing sequence used to control theness of the solution. They are particularly effective for large-scale problems where explicit Hessian storage or factorization is highly expensive.*

This note explores the convergence properties and implementation strategies of [[Inexact Newton Methods]], which are used to solve large-scale unconstrained optimization problems by approximating the Newton step rather than computing a direct factorization of the Hessian. 

## Concept
In a standard Newton method, the search direction $p_k$ is found by solving the 

$$ \nabla^2 f(x_k) p_k = -\nabla f(x_k) $$

In the [[Inexact Newton Methods]] framework, this system is solved approximately using iterative methods like the [[Conjugate Gradient Method]]. The quality of the approximation is governed by a [[Forcing Sequence]] $\eta_k$, where the residual $r_k$ of the step satisfies:

$$ r_k = \nabla f(x_k) + \nabla^2 f(x_k) p_k, \quad \|r_k\| \le \eta_k \,|\nabla f(x_k)| $$

### Convergence Rates
- **[[Superlinear Convergence]]**: Occurs when the [[Forcing Sequence]] $\eta_k \to 0$ as $k \to \infty$. This ensures that the gradient norm decreases sufficiently to achieve a superlinear rate.
- **[[Quadratic Convergence]]**: If the [[Hessian]] $\nabla^2 f(x_k)$ is Lipschitz continuous and the forcing sequence is chosen such that $\eta_k = O(\|\nabla f(x_k)\|)$, the method achieves a quadratic rate of convergence.

## Quasi-Newton Methods
The text compares these to [[Ququasi-Newton Methods]] such as the [[BFGS Method]] and the [[SR1 Method]]. While [[BFGS Method]] is known for its robust [[Superlinear Convergence]], the [[SR1 Method]] is less well-understood but can achieve superlinear convergence in trust-region frameworks under specific conditions. 

## Trust-Region Approaches
For large-scale problems, the [[Trust-Region Newton-CG Method]] (specifically [[Steihaug's Method]]) is used to solve the trust-region subproblem. [[Steihaug's Method]] is an adaptation of the Conjugate Gradient method that terminates when it encounters a direction of negative curvature or hits the trust-region boundary. This method ensures that the step is at least as good as the [[Cauchy Point]], providing global convergence guarantees. 

## Implementation Details
In [[Hessian-free]] implementations, the [[Hessian]] is never explicitly stored; instead, Hessian-vector products $\nabla^2 f(x) d$ are computed using finite differencing or automatic differentiation. 

$$ \nabla^2 f(x) d \approx \frac{\nabla f(x + hd) - \nabla f(x)}{h} $$

This approximation is used within the iterative solvers to maintain efficiency in large-scale settings.

## Relationships
- [[Inexact Newton Methods]] uses [[Conjugate Gradient Method]]
- [[Inexact Newton Methods]] achieves [[Superlinear Convergence]]
- [[Inexact Newton Methods]] achieves [[Quadratic Convergence]]
- [[Inexact Newton Methods]] is controlled by [[Forcing Sequence]]
- [[Trust-Region Newton-CG Method]] implements [[Steihaug's Method]]
- [[Trust-Region Newton-CG Method]] uses [[Conjugate Gradient Method]]
- [[Trust-Region Newton-CG Method]] guarantees reduction at least as good as [[Cauchy Point]]
- [[BFGS Method]] exhibits [[Superlinear Convergence]]
- [[SR1 Method]] exhibits [[Superlinear Convergence]]
