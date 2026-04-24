---
type: content
title: "Line Search Methods and Convergence Rates"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:24:11.061446200+00:00
summary: "Line search methods optimize functions by choosing a search direction and an appropriate step length. These methods, including steepest descent, steepest descent, and Newton's method, exhibit different convergence rates, ranging from linear, linear, and superlinear, to quadratic. The Wolfe and Goldstein conditions are termination conditions used to unctionally ensure sufficient decrease and prevent unacceptably short steps."
tags:
  - optimization
  - numerical-methods
  - mathematical-optimization
entities:
  - "[[Line Search Methods]]"
  - "[[Steepest Descent]]"
  - "[[coefficients]]"
  - "[[Newton's Method]]"
  - "[[Wolfe Conditions]]"
  - "[[Goldstein Conditions]]"
  - "[[Convergence Rate]]"
  - "[[Quasi-Newton Methods]]"
  - "[[Zoutendijk's Condition]]"
  - "[[Armijo Condition]]"
  - "[[Curvature Condition]]"
  - "[[Backtracking Line Search]]"
relationships:
  - source: "Line Search Methods"
    target: "Steepness Descent"
    description: "includes"
  - source: "Line Search Methods"
    target: "Newton's Method"
    description: "uses"
  - source: "Line Search Methods"
    target: "Quasi-Newton Methods"
    description: "includes"
  - source: "Line Search Methods"
    target: "Wolfe Conditions"
    description: "utilizes"
  - source: "Line Search Methods"
    target: "Goldstein Conditions"
    description: "utilizes"
  - source: "Wolfe Conditions"
    target: "Armijo Condition"
    description: "consists of"
  - source: "Wolfe Conditions"
    target: "Curvature Condition"
    description: "consists of"
  - source: "Wolfe Conditions"
    target: "Zoutendijk's Condition"
    description: "implies"
  - source: "Quasi-Newton Methods"
    target: "Newton's Method"
    description: "approximates"
  - source: "Superlinear Convergence"
    target: "Wolfe Conditions"
    description: "depends on"
---

# Line Search Methods and Convergence Rates

*Line search methods optimize functions by choosing a search direction and an appropriate step length. These methods, including steepest descent, steepest descent, and Newton's method, exhibit different convergence rates, ranging from linear, linear, and superlinear, to quadratic. The Wolfe and Goldstein conditions are termination conditions used to unctionally ensure sufficient decrease and prevent unacceptably short steps.*

This note explores the mechanics and convergence properties of [[Line Search Methods]].

## Concept
[[Line Search Methods]] follow the iteration formula:

$$ x_{k+1} = x_k + \alpha_k p_k $$

where $\alpha_k$ is the [[Step Length]] and $p_k$ is the search direction. The goal is to find an $\alpha_k$ that provides a substantial reduction in the objective function $f$, while minimizing computational cost. 

### Termination Conditions
To ensure convergence, algorithms use specific termination conditions to identify acceptable step lengths. 

#### The Wolfe Conditions
[[Wolfe Conditions]] are a popular set of inexact line search conditions. They consist of two parts:
1. [[Armijo Condition]] (Sufficient Decrease): 

$$ \nabla f(x_k)^T p_k \geq c_1 (f(x_{k+1}) - f(x_k)) $$

This ensures the reduction is proportional to the directional derivative. 

2. [[Curvature Condition]]: 

$$ \nabla f(x_{k+1})^T p_k \text{ is greater than } c_2 \text{ times the initial slope} $$

Together, these conditions prevent unacceptably short steps. 

#### The Goldstein Conditions
[[Goldstein Conditions]] also ensure sufficient decrease but are often used in Newton-type methods rather than quasi-Newton methods. 

### Convergence Rates
Different algorithms exhibit different [[Convergence Rate]]s. 

- [[Steepest Descent]] exhibits linear convergence, especially when the problem is ill-conditioned. 
- [[Newton's Method]] provides quadratic convergence, provided the Hessian is positive definite and the starting point is sufficiently close to the solution. 
- [[Quasi-Newton Methods]] can achieve superlinear convergence if the search direction approximates the Newton direction well enough. 

## Relationships
- [[Line Search Methods]] includes [[Steepest Descent]], [[Newton's Method]], and [[Quasi-Newton Methods]].
- [[Wolfe Conditions]] consists of [[Armijo Condition]] and [[Curvature Condition]].
- [[Wolfe Conditions]] implies [[Zoutendijk's Condition]].
- [[Quasi-Newton Methods]] approximates [[Newton's Method]].
- [[Step Length]] is a key parameter in [[Line Search Methods]].
