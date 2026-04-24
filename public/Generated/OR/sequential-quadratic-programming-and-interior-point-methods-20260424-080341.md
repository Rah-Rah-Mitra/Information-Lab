---
type: content
title: "Sequential Quadratic Programming And Interior-Point Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:03:41.565320200+00:00
summary: "This note covers the fundamental mechanisms of Sequential Quadratic Programming (SQP) and Interior-Point Methods for nonlinear programming. It explains how SQP uses quadratic subproblems to approximate the local geometry, while Interior-Point methods follow a central path via barrier functions. Both are powerful tools for large-scale optimization."
tags:
  - optimization
  - nonlinear-programming
  - mathematical-optimization
entities:
  - "[[Sequential Quadratic Programming]]"
  - "[[Interior-Point Methods]]"
  - "[[Quadratic Subproblem]]"
  - "[[Barrier Function]]"
  - "[[KKT Conditions]]"
  - "[[Lagrangian]]"
  - "[[Primal-Dual System]]"
  - "[[, ]]"
relationships:
  - source: "Sequential Quadratic Programming"
    target: "KKT Conditions"
    description: "approximates"
  - source: "Sequential Quadratic Programming"
    target: "Quadratic Subproblem"
    description: "solves a sequence of"
  - source: "Interior-Point Methods"
    target: "Barrier Function"
    description: "utilizes"
  - source: "Interior-Point Methods"
    target: "Primal-Dual System"
    description: "solves a"
  - source: "KKT Conditions"
    target: "Lagrangian"
    description: "involves"
  - source: "Primal-Dual System"
    target: "KKT Conditions"
    description: "is a reformulation of the perturbed"
  - source: "Quadratic Subproblem"
    target: "KKT Conditions"
    description: "seeks to satisfy"
  - source: "Barrier Function"
    target: "KKT Conditions"
    description: "approaches via"
  - source: "Lagrangian"
    target: "KKT Conditions"
    description: "defines the"
  - source: "Interior-Point Methods"
    target: "KKT Conditions"
    description: "converges to a point satisfying"
---

# Sequential Quadratic Programming And Interior-Point Methods

*This note covers the fundamental mechanisms of Sequential Quadratic Programming (SQP) and Interior-Point Methods for nonlinear programming. It explains how SQP uses quadratic subproblems to approximate the local geometry, while Interior-Point methods follow a central path via barrier functions. Both are powerful tools for large-scale optimization.*

This note explores the two primary paradigms for solving large-scale nonlinear programming problems: [[Sequential Quadratic Programming]] and [[Interior-Point Methods]].

## Concept

### Sequential Quadratic Programming (SQP)
[[Sequential Quadratic Programming]] is an iterative method that models the nonlinear program by solving a sequence of [[Quadratic Subproblem]]s. At each iteration, a quadratic approximation of the Lagrangian is used to define the objective, while the linearized constraints define the feasible region. The goal is to find a point that satisfies the [[KKT Conditions]].

An SQP step is typically defined by the subproblem:

$$ \min_{p} \frac{1}{2} p^T B_k p + \nabla f(x_k)^T p \quad \text{subject to} \quad c(x_k) + \nabla c(x_k)^T p \text{ (linearized constraints)} $$

where $B_k$ is an approximation of the Hessian of the [[Lagrangian]].

### Interior-Point Methods
[[Interior-Point Methods]] can be interpreted through two lenses: the homotopy (continuation) approach and the barrier approach. In the homotopy approach, one solves the perturbed [[KKT Conditions]] for a sequence of positive parameters $\mu_k$ that converges to zero. This trajectory is known as the [[Central Path]].

In the barrier approach, the problem is transformed by adding a logarithmic barrier term to the objective function:

$$ \min_{x} f(x) - \mu \sum_{i=1}^{m} \log(s_i) \quad \text{subject to} \quad c(x) - s = 0, \quad s \text0 \end{)$$ 

This approach forces the variables to remain in the interior of the feasible region. Modern methods often solve the [[Primal-Dual System]], which is a Newton-like system derived from the perturbed KKT conditions, to find the search direction.

## Relationships

- [[Sequential Quadratic Programming]] approximates [[KKT Conditions]]
- [[Sequential Quadratic Programming]] solves a sequence of [[Quadratic Subproblem]]s
- [[Interior-Point Methods]] utilizes [[Barrier Function]]
- [[Interior-Point Methods]] solves a [[Primal-Dual System]]
- [[KKT Conditions]] involves [[Lagrangian]]
- [[Primal-Dual System]] is a reformulation of the perturbed [[KKT Conditions]]
- [[Quadratic Subproblem]] seeks to satisfy [[KKT Conditions]]
- [[Barrier Function]] approaches [[KKT Conditions]] via
- [[Lagrangian]] defines the [[KKT Conditions]]
- [[Interior-Point Methods]] converges to a point satisfying [[KKT Conditions]]
