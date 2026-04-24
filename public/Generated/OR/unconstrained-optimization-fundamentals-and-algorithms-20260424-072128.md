---
type: content
title: "Unconstrained Optimization Fundamentals and Algorithms"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:21:28.582026600+00:00
summary: "This note covers the necessary and sufficient conditions for local minimizers in smooth unconstrained optimization, alongside a comparison of primary algorithmic strategies. It details the differences between line search and trust-region methods, as well as the impact of scaling on convergence. Understanding these concepts is essential for selecting the appropriate optimization algorithm for a specific problem."
tags:
  - operations-research
  - optimization-algorithms
  - mathematical-optimization
  - calculus-foundations
entities:
  - "[[Unconstrained Optimization]]"
  - "[[Local Minimizer]]"
  - "[[Stationary Point]]"
  - "[[Taylor's Theorem]]"
  - "[[Convex Function]]"
  - "[[Line Search]]"
  - "[[Trust Region]]"
  - "[[Newton Direction]]"
  - "[[Newton's Method]]"
  - "[[Quasi-Newton Method]]"
  - "[[Steepest Descent]]"
  - "[[Conjugate Gradient Method]]"
  - "[[Hessian Matrix]]"
  - "[[Scaling]]"
  - "[[Scale Invariance]]"
relationships:
  - source: "Unconstrained Optimization"
    target: "Local Minimizer"
    description: "seeks"
  - source: "Unconstrained Optimization"
    target: "Stationary Point"
    description: "identifies"
  - source: "Local Minimizer"
    target: "Stationary Point"
    description: "must be a"
  - source: "Taylor's Theorem"
    target: "Local Minimizer"
    description: "used to derive conditions for"
  - source: "Convex Function"
    target: "Local Minimizer"
    description: "ensures local is"
  - source: "Local Minimizer"
    target: "Hessian Matrix"
    description: "characterized by"
  - source: "Newton's Method"
    target: "Hessian Matrix"
    description: "uses"
  - source: "Newton\\_and_Newton's Method"
    target: "Hessian Matrix"
    description: "requires"
  - source: "Newton's Method"
    target: "Newton Direction"
    description: "produces"
  - source: "Quasi-Newton Method"
    target: "Hessian Matrix"
    description: "approximates"
  - source: "Steepest Descent"
    target: "Gradient"
    description: "uses"
  - source: "Line Search"
    target: "Newton Direction"
    description: "can use"
  - source: "Trust Region"
    target: "Newton Direction"
    description: "can use"
  - source: "Scaling"
    target: "Unconstrained Optimization"
    description: "affects performance of"
  - source: "Scale Invariance"
    target: "Scaling"
    description: "is a property of robust algorithms against"
  - source: "Conjugate Gradient Method"
    target: "Unconstrained Optimization"
    description: "is an algorithm for"
  - source: "Newton's Method"
    target: "Scale Invariance"
    description: "is often"
  - source: "Newton's Method"
    target: "Scale Invariance"
    description: "is often"
  - source: "Newton's Method"
    target: "Scale Invariance"
    description: "is often"
---

# Unconstrained Optimization Fundamentals and Algorithms

*This note covers the necessary and sufficient conditions for local minimizers in smooth unconstrained optimization, alongside a comparison of primary algorithmic strategies. It details the differences between line search and trust-region methods, as well as the impact of scaling on convergence. Understanding these concepts is essential for selecting the appropriate optimization algorithm for a specific problem.*

This note explores the foundational principles of [[Unconstrained Optimization]] and the primary algorithmic strategies used to solve such problems.

## Concept
In [[Unconstrained Optimization]], we seek a point $x$ that minimizes a function $f(x)$. For smooth functions, the identification of a local minimizer is governed by the first and second-order necessary and sufficient conditions, which are heavily reliant on [[Taylor's Theorem]].

### Optimality Conditions

- **First-Order Necessary Condition**: If $x$ is a [[Local Minimizer]], a continuously differentiable function $f$ must satisfy $\nabla f(x) = 0$. A point where the gradient vanishes is called a [[Stationary Point]].
- **Second-Order Necessary Condition**: If $x$ is a local minimizer, the [[Hessian Matrix]] $\nabla^2 f(x)$ must be positive semidefiniteness.
- **Second-Order Sufficient Condition**: If the [[Hessian Matrix]] $\nabla^2 f(x)$ is positive definite, then $x$ is a strict local minimizer.

When the objective function is a [[Convex Function]], any local minimizer is also a global minimizer.

$$ \nabla^2 f(x) \text{ is positive definite} \implies x \text{ is a strict local minimizer} $$

### Algorithmic Strategies

Most unconstrained optimization algorithms generate a scale of iterates $x_k$ to find a minimizer. There are two fundamental strategies:

1. **[[Line Search]]**: The algorithm chooses a search direction $p_k$ and then identifies an appropriate step length $\alpha_k$ to minimize $f(x_k + \\text{alpha}_k p_k)$. 
   - The [[Steepest Descent]] direction is the most intuitive, defined by $p_k = -\nabla f(x_k) / \|\nabla f(x_k)\|$. It is computationally efficient but can be slow on difficult problems.
   - The [[Newton Direction]] is derived from a second-order Taylor expansion and provides a more reliable, and often quadratic, rate of convergence. It is defined by $p_k = -[\nabla^2 f(x_k)]^{-1} \nabla f(x_k)$.
   - [[Quasi-Newton Method]]s use an approximation $B_k$ to the Hessian to avoid the expensive computation of the exact Hessian.
   - [[Conjugate Gradient Method]]s provide ans alternative that is effective for large-scale problems without requiring matrix storage.

2. **[[Trust Region]]**: Instead of choosing a direction first, the algorithm constructs a a model function $m_k$ (often quadratic) and restricts the search for a minimizer to a region around $x_key$ where the model is behavior is predictable. The search is conducted within a a trust-region radius $\delta_k$.

### Scaling and Robustness

[[Scaling]] is a critical factor in problem formulation. A [[Poorly Scaled]] problem is one where changes in $x$ in one direction produce much larger variations in $f(x)$ than in others, leading to elongated contours. This makes algorithms like [[Steepest Descent]] highly sensitive to performance degradation. In contrast, [[Newton's Method]] is often unaffected by scaling due to its use of the second-order information in the [[Hessian Matrix]]. Algorithms that aim for [[Scale Invariance]] in all aspects of their design are considered more robust.

## Relationships
- [[Unconstrained Optimization]] seeks [[Local Minimizer]]
- [[Local Minimizer]] must be a [[Stationary Point]]
- [[Taylor's Theorem]] used to derive conditions for [[Local Minimizer]]
- [[Convex Function]] ensures local is [[Local Minimizer]]
- [[Local Minimizer]] characterized by [[Hessian Matrix]]
- [[Newton's Method]] uses [[Hessian Matrix]]
- [[Newton's Method]] produces [[Newton Direction]]
- [[Quasi-Newton Method]] approximates [[Hessian Matrix]]
- [[Steepest Descent]] uses [[Gradient]]
- [[Line Search]] can use [[Newton Direction]]
- [[Trust Region]] can use [[Newton Direction]]
- [[Conjugate Gradient Method]] is an algorithm for [[Unconstrained Optimization]]
- [[Scaling]] affects performance of [[Unconstrained Optimization]]
- [[Scale Invariance]] is a property of robust algorithms against [[Scaling]]
