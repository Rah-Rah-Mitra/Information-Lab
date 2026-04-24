---
type: content
title: "Automatic Differentiation and Derivative-Free Optimization"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:33:09.408898500+00:00
summary: "Automatic differentiation (AD) provides efficient ways to compute gradients, Jacobians, and Hessians using computational graphs. Derivative-free optimization (DFO) methods, such as model-based trust-region approaches, are used when derivatives are unavailable or noisy. The two techniques are often complementary in large-scale optimization."
tags:
  - optimization
  - automatic-differentiation
  - derivative-free-optimization
  - numerical-analysis
entities:
  - "[[Automatic Differentiation]]"
  - "[[Computational Graph]]"
  - "[[Forward Mode]]"
  - "[[Reverse Mode]]"
  - "[[Jacobian]]"
  - "[[Hessian]]"
  - "[[Derivative-Free Optimization]]"
  - "[[Derivative-Free Optimization Model-Based Methods]]"
  - "[[Trust-Region Methods]]"
  - "[[Finite Differences]]"
relationships:
  - source: "Automatic Differentiation"
    target: "Computational Graph"
    description: "utilizes"
  - source: "Automatic "
    target: "Forward Mode"
    description: "includes"
  - source: "Automatic Differentiation"
    target: "Reverse Mode"
    description: "includes"
  - source: "Automatic Differentiation"
    target: "Jacobian"
    description: "computes"
  - source: "Automatic Differentiation"
    target: "Hessian"
    description: "computes"
  - source: "Derivative-Free Optimization"
    target: "Finite Differences"
    description: "relates to"
  - source: "Derivative-Free Optimization"
    target: "Derivative-Free Optimization Model-Based Methods"
    description: "includes"
  - source: "Derivative-Free Optimization Model-Based Methods"
    target: "Trust-Region Methods"
    description: "uses"
---

# Automatic Differentiation and Derivative-Free Optimization

*Automatic differentiation (AD) provides efficient ways to compute gradients, Jacobians, and Hessians using computational graphs. Derivative-free optimization (DFO) methods, such as model-based trust-region approaches, are used when derivatives are unavailable or noisy. The two techniques are often complementary in large-scale optimization.*

This note explores the mechanisms of [[Automatic Differentiation]] and [[Derivative-Free Optimization]] for calculating derivatives and optimizing functions. 

## Concept

### Automatic Differentiation

[[Automatic Differentiation]] (AD) is a technique for obtaining exact derivatives by propagating derivatives through a [[Computational Graph]] representing the function's evaluation. AD is typically implemented in two modes: 

1. [[Forward Mode]]: This mode computes derivatives alongside the function evaluation during a single forward sweep. It is highly efficient for a function $f: \mathbb{R}^n \to \mathbb{R}$ where $n$ is small, but its cost scales with the number of inputs. 

2. [[Jacobian-Transpose-Vector Product]] (Reverse Mode): The [[Reverse Mode]] of AD is particularly powerful for functions with many inputs and few outputs (eg. $f: \mathbb{R}^n \to \mathbb{R}$), as it provides the gradient in a single reverse sweep. 

For vector functions $r: \{1, \text{m} \to \text{R}^m$, AD can compute the [[Jacobian]] $J(x)$ or the [[Hessian]] $\nabla^2 f(x)$. The [[Forward Mode]] is suitable for computing Jacobian-vector products, $J(x)p$, while the [[Reverse Mode]] allows for efficient computation of the Jacobian-transpose-vector products, $J(x)^T q$. 

For second-order derivatives, the [[Hessian]] can be computed using forward-mode propagation of first and second derivatives, or by combining forward and reverse modes. The complexity of thes is often managed via [[Checkpointing]] [[Checkpointing]] is a A technique to reduce storage requirements of the computational graph by performing partial sweeps. 

### Derivative-Free Optimization

[[Derivative-Free Optimization]] (DFO) is used when derivatives are not available, are too expensive to compute, or are noisy. 

- [[Finite Differences]]: This approach approximates the derivatives using function evaluations at nearby points. However, it is can be unreliable in the presence of [[Noise]] (e.g., from stochastic simulations or numerical solvers). The error in a centered finite-difference approximation is bounded by the noise level $\eta(x)$ and the step size $h$. 

- [[Model-Based Methods]]: These methods, such as [[Derivative-Free Optimization Model-Based Methods]], construct a local quadratic model $m_k(x)$ of the objective function $f(x)$ at the current iterate $x_k$ using a set of interpolation points $Y$. 

$$ m_k(x) = c + g^T(x - x_k) + \frac{1}{2}(x - x_k)^T G (x - x_k) $$ 

This model is used within a [[Trust-Region Methods]] framework to find the next iterate. The model-based approach is more robust to noise than finite differences. 

## Relationships

- [[Automatic Differentiation]] utilizes [[Computational Graph]]
- [[Automatic Differentiation]] includes [[Forward Mode]] and [[Reverse Mode]]
- [[Automatic Differentiation]] computes [[Jacobian]] and [[Hessian]]
- [[Derivative-Free Optimization]] relates to [[Finite Differences finite-difference]] 
- [[Derivative-Free Optimization]] includes [[Derivative-Free Optimization Model-Based Methods]] 
- [[Derivative-Free Optimization Model-Based Methods]] uses [[Trust-Region Methods]]
