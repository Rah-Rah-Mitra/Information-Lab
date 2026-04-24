---
type: content
title: "Sequential Quadratic Programming Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:06:03.821185400+00:00
summary: "Sequential Quadratic Programming (SQP) is an iterative method for solving nonlinearly constrained optimization problems by solving a sequence of quadratic subproblems. It is highly effective for problems with significant nonlinearities in the constraints and can be implemented via IQP or EQP approaches. The method's efficiency depends on the ability to correctly identify the optimal active set."
tags:
  - optimization
  - nonlinear-programming
  - sequential-quadratic-programming
  - mathematical-programming
entities:
  - "[[Sequential Quadratic Programming]]"
  - "[[Quadratic Programming Subproblem]]"
  - "[[Inequality-Constrained Quadratic Program]]"
  - "[[Equality-Constrained Quadratic Program]]"
  - "[[Equality-Constrained Optimization]]"
  - "[[Lagrange Multiplier]]"
  - "[[Active Set]]"
  - "[[Inequality-Constrained Quadratic Program]]"
  - "[[IQP Approach]]"
  - "[[EQP Approach]]"
relationships:
  - source: "Sequential Quadratic Programming"
    target: "Quadratic Programming Subproblem"
    description: "solves a sequence of"
  - source: "Sequential Quadratic Programming"
    target: "Equality-Constrained Optimization"
    description: "applies to"
  - source: "Sequential Quadratic Programming"
    target: "Lagrange Multiplier"
    description: "estimates"
  - source: "Sequential Quadratic Programming"
    target: "Active Set"
    description: "identifies"
  - source: "Sequential Quadratic Programming"
    target: "IQP Approach"
    description: "implements via"
  - source: "Sequential Programming"
    target: "EQP Approach"
    description: "implements via"
  - source: "IQP Approach"
    target: "Inequality-Constrained Quadratic Program"
    description: "solves"
  - source: "EQP Approach"
    target: "Equality-Constrained Quadratic Program"
    description: "solves"
  - source: "Quadratic Programming Subproblem"
    target: "Lagrange Multiplier"
    description: "provides"
  - source: " "
    target: "Active Set"
    description: "determines"
---

# Sequential Quadratic Programming Methods

*Sequential Quadratic Programming (SQP) is an iterative method for solving nonlinearly constrained optimization problems by solving a sequence of quadratic subproblems. It is highly effective for problems with significant nonlinearities in the constraints and can be implemented via IQP or EQP approaches. The method's efficiency depends on the ability to correctly identify the optimal active set.*

[[Sequential Quadratic Programming]] is a powerful iterative framework for solving nonlinear programming problems, particularly those involving significant nonlinearities in the constraints. The method generates a step by solving a sequence of quadratic subproblems that approximate the local geometry of the objective function and the constraints.

## Concept
At each iteration $k$, the [[Sequential Quadratic Programming]] algorithm models the nonlinear problem at the current iterate $x_k$ by constructing a quadratic model of the Lagrangian and a linear approximation of the constraints. For equality-constrained problems, the subproblem takes the form:

$$ \min_{p} \frac{1}{2} p^T \nabla_{xx}^2 L(x_k, \lambda_k) p + \nabla f(x_k)^T p \quad \text{subject to} \quad A(x_k)p + c(x_k) = 0 $$

This subproblem is a [[Quadratic Programming Subproblem]]. If the constraints are inequalities, the [[Inequality-Constrained Quadratic Program]] (IQP) approach solves the full inequality-constrained subproblem at each step, using the solution's active set as a guess for the optimal active set. Conversely, the [[Equality-Constrained Quadratic Program]] (EQP) approach selects a subset of constraints (the working set) and solves only equality-constrained subproblems, which is often more computationally efficient for large-scale problems.

## Implementation Strategies
There are two primary implementation strategies for [[Sequential Quadratic Programming]]:
1. **IQP Approach**: Solves an [[Inequality-Constrained Quadratic Program]] at every iteration. While robust, it can be be expensive for large problems.
2. **EQP Approach**: Decouples the computation of the ability to identify the optimal [[Active Set]]. The working set is updated based on [[Lagrange Multiplier]] estimates or auxiliary subproblems.

## Relationships
- [[Sequential Quadratic Programming]] solves a sequence of [[Quadratic Programming Subproblem]]
- [[Sequential Quadratic Programming]] applies to [[Equality-Constrained Optimization]]
- [[Sequential Quadratic Programming]] estimates [[Lagrange Multiplier]]
- [[Sequential Quadratic Programming]] identifies [[Active Set]]
- [[Sequential Quadratic Programming]] implements via [[IQP Approach]]
- [[Sequential Quadratic Programming]] implements via [[EQP Approach]]
- [[IQP Approach]] solves [[Inequality-Constrained Quadratic Program]]
- [[EQP Approach]] solves [[Equality-Constrained Quadratic Program]]
- [[Quadratic Programming Sub Programming]] provides [[Lagrange Multiplier]]

