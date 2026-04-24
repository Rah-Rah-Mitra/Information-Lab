---
type: content
title: "Optimization Problem Classifications"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:16:25.534638300+00:00
summary: "Optimization problems are categorized by the nature of their objective functions, constraints, and the presence of uncertainty. These classifications guide the selection of appropriate mathematical programming techniques, such as linear, nonlinear, or integer programming. Understanding these distinctions is essential for ensuring algorithmic convergence and finding global versus local solutions."
tags:
  - mathematical-programming
  - optimization-theory
  - decision-science
entities:
  - "[[Optimization Problem]]"
  - "[[Linear Programming]]"
  - "[[Continuous Optimization]]"
  - "[[Discrete Optimization]]"
  - "[[Integer Programming]]"
  - "[[Mixed Integer Programming]]"
  - "[[Nonlinear Programming]]"
  - "[[Convex Optimization]]"
  - "[[Convex Set]]"
  - "[[Convex Function]]"
  - "[[Global Minimizer]]"
  - "[[Local Minimizer]]"
relationships:
  - source: "Optimization Problem"
    target: "Linear Programming"
    description: "includes"
  - source: "Optimization Problem"
    target: "Continuous Optimization"
    description: "includes"
  - source: "Optimization Problem"
    target: "Discrete Optimization"
    description: "includes"
  - source: "Discrete Optimization"
    target: "Integer Programming"
    description: "specialises to"
  - source: "Discrete Optimization"
    target: "Mixed Integer Programming"
    description: "specialises to"
  - source: "Optimization Problem"
    target: "Nonlinear Programming"
    description: "includes"
  - source: "Optimization Problem"
    target: "Convex Optimization"
    description: "includes"
  - source: "Convex Set"
    target: "Convex Function"
    description: "defines domain for"
  - source: "Global Minimizer"
    target: "Optimization Problem"
    description: "is a solution to"
  - source: "Local Minimizer"
    target: "Optimization Problem"
    description: "is a solution to"
---

# Optimization Problem Classifications

*Optimization problems are categorized by the nature of their objective functions, constraints, and the presence of uncertainty. These classifications guide the selection of appropriate mathematical programming techniques, such as linear, nonlinear, or integer programming. Understanding these distinctions is essential for ensuring algorithmic convergence and finding global versus local solutions.*

## Concept
[[Optimization Problem]]s are mathematical formulations used to find the best possible value of a (usually minimum or maximum) of an objective function subject to certain constraints. They are broadly classified based on several dimensions: the nature of the variables, the smoothness of the functions, and the presence of uncertainty.

### Continuous vs. Discrete Optimization
[[Continuous Optimization]] involves variables that can take any real value from an uncountably infinite set, whereas [[Discrete Optimization]] involves variables drawn from a finite or countable set (e.g., integers or permutations). 

[[Integer Programming]] is a specific type of discrete optimization where variables are restricted to the set of integers \( \mathbb{Z} \). If some variables are continuous and others are integers, the problem is termed a [[Mixed Integer Programming]] (MIP) problem.

### Linear vs. Nonlinear Programming
[[Linear Programming]] occurs when both the objective function and all constraints are linear functions of the variables. In contrast, [[Nonlinear Programming]] arises when at least one of the objective or constraint functions is nonlinear.

### Convexity in Optimization
[[Convex Optimization]] is a special class of problems where the objective function is convex and the feasible region is a convex set. A [[Convex Set]] \( S \) is defined such that for any two points \( x, y \in S \), the line segment connecting them lies entirely within \( S \): 

$$ \alpha x + (1 - \alpha)y \in S \ \ \forall \alpha \in [0, 1] $$

This formula defines the property of a convex set.

A [[Convex Function]] \( f \) is defined on a convex set \( S \) if for all \( x, y \in S \) and \(alpha \in [0, 1] \): 

$$ f(\alpha x + (1 - \alpha)y) \le \alpha f(x) + (1 - \alpha)f(y) $$ 

This inequality defines the convexity of a function.

In [[Convex Optimization]], any local solution is guaranteed to be a global solution, which significantly simplifies the search for an optimal point.

### Local vs. Global Solutions
In many nonlinear problems, algorithms may find a [[Local Minimizer]], which is a point that achieves the smallest value of the function within a specific neighborhood, but not necessarily the lowest value across the entire domain. A [[Global Minimizer]] is a point where the function attains its absolute minimum value over the entire feasible set.

## Relationships
- [[Optimization Problem]] includes [[Linear Programming]], [[Continuous Optimization]], [[Discrete Optimization]], and [[Nonlinear Programming]].
- [[Discrete Optimization]] specialises to [[Integer Programming]] and [[Mixed Integer Programming]].
- [[Convex Optimization]] requires a [[Convex Set]] and a [[Convex Function]].
- [[Local Minimizer]] and [[Global Minimizer]] are types of solutions to an [[Optimization Problem]].
