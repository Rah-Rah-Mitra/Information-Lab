---
type: content
title: "Multivariate Calculus and Optimization Fundamentals"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:22:17.832454700+00:00
summary: "This note covers the fundamental principles of multivariate calculus, including gradients, gradients of vector-valued functions, gradients of composite functions, chain rule, and the Jacobian, and how they relate to root-finding and root-finding algorithms. It also covers the source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and sourcema"
tags:
  - mathematics
  - multivariate-calculus
  - ]
  - optimization-theory
  - numerical-analysis
entities:
  - "[[Gradient]]"
  - "[[Jacobian]]"
  - "[[Hessian]]"
  - "[[Chain Rule]]"
  - "[[Implicit Function Theorem]]"
  - "[[Directional Derivative]]"
  - "[[Mean Value Theorem]]"
  - "[[Newton's Method]]"
  - "[[Secant Method]]"
  - "[[Order Notation]]"
  - "[[Taylor's Theorem]]"
relationships:
  - source: "Gradient"
    target: "Jacob    "
    description: "is part of"
  - source: "Jacobian"
    target: "Jacobian"
    description: "is part of"
  - source: "Jacobian"
    target: "Hessian"
    description: "is part of"
  - source: "Chain Rule"
    target: "Differentiability"
    description: "is used to compute"
  - source: "Newton's Method"
    target: "Jacobian"
    description: "uses"
  - source: "Jacobian"
    target: "Hessian"
    description: "relates to"
  - source: "Implicit Function Theorem"
    target: "Jacobian"
    description: "requires nonsingular"
  - source: "Secant Method"
    target: "Jacobian"
    description: "approximates"
  - source: "Newton's Method"
    target: "Jacobian"
    description: "target is used in"
  - source: "Mean Value Theorem"
    target: "Jacobian["
    description: "generalises to"
  - source: "Taylor's Theorem"
    target: "Jacobian["
    description: "is a form of"
  - source: "Directional Derivative"
    target: "Gradient"
    description: "is related to via"
  - source: "Hessian"
    target: "Gradient"
    description: "is the derivative of"
  - source: "Order Notation"
    target: "Convergence"
    description: "describes"
  - source: "Newton'Decision Method"
    target: "Jacobian"
    description: "uses"
---

# Multivariate Calculus and Optimization Fundamentals

*This note covers the fundamental principles of multivariate calculus, including gradients, gradients of vector-valued functions, gradients of composite functions, chain rule, and the Jacobian, and how they relate to root-finding and root-finding algorithms. It also covers the source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and source material and sourcema*

This note provides a foundational overview of multivariate calculus and its application to optimization. 

## Concept

In multivariate calculus, the [[Gradient]] is the vector of partial derivatives of a function $f: \mathbb{R}^n \to \mathbb{R}$. It is defined as: 

$$ \nabla f(x) = [\frac{\partial f}{\partial x_1}, \dots, \frac{\partial f}{\partial x_n}]^T $$
	ext{This formula represents the vector of first-order partial derivatives.}

For vector-valued functions $f: \mathbb{R}^n \to \text{R}^m$, the matrix of first-order partial derivatives is the [[Jacobian]] matrix, denoted by $J(x)$. The $(i, j)$ entry is $\frac{\partial f_i}{\partial x_j}$. 

If a function is twice differentiable, the matrix of second-order partial derivatives is the [[Hessian]] matrix. For a scalar function, the Hessian is symmetric. 

[[Chain Rule]] is used to compute the derivatives of composite functions $h(t) = f(x(t))$. 

[[Directional Derivative]] is defined as the limit: 

$$ \text{D}_p f(x) = \lim_{\Delta \to 0} \frac{f(x + \Delta p) - f(x)}{\Delta} $$ 

This formula models the rate of change of a function in a specific direction $p$. 

[[Mean Value Theorem]] for multivariate functions states that for a vector $p$, there exists $\alpha \text{ in } (0, 1)$ such that: 

$$ f(x + p) - f(x) = \nabla f(x + \alpha p) \text{ } p $$ 

This formula relates the change in function value to the gradient at an intermediate point. 

[[Implicit Function Theorem]] states that if a function $h(z, t)$ is continuously differentiable and the Jacobian with respect to $z$ is nonsingular at a point, then $z$ can be uniquely defined as a continuous function of $t$. 
ooting-finding methods like [[Newton's Method]] and [[Secant Method]] are used to find roots of equations. Newton's method uses the tangent line to approximate the function, and the Secant method approximates the Hessian or Jacobian. 

[[Order Notation]] is used to describe the convergence behavior of and sequences. We use $O$, $o$, and $\Theta$ to        

$$ \eta_k = O(\nu_k) \text{ if there is a constant } C \text{ such that } |\eta_s| \le C|\nu_k| \text{ for all } k \text{ sufficiently large.} $$ 

This formula defines the Big-O notation for sequences. 

## Relationships
- [[Gradient]] is part of [[Jacobian]]
- [[Jacobian]] is part of [[Hessian]]

