---
type: content
title: "Sequential Quadratic Programming And Interior Point Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:06:30.638621800+00:00
summary: "Sequential Quadratic Programming (SQP) and Interior Point Methods (IPM) are two primary frameworks for solving nonlinear programming problems. SQP focuses on iterative refinement of quadratic subproblems, while IPMs use barrier functions to guide iterates toward the solution. Both methods rely on efficient linear algebra and merit functions for step acceptance."
tags:
  - nonlinear-optimization
  - optimization-algorithms
  - mathematical-programming
entities:
  - "[[Sequential Quadratic Programming]]"
  - "[[Interior Point Methods]]"
  - "[[Quadratic Subproblem]]"
  - "[[Lagrangian Hessian]]"
  - "[[Merit Function]]"
  - "[[Merit Function]]"
  - "[[Barrier Parameter]]"
  - "[[Quasi-Newton Approximation]]"
  - "[[Primal-Dual System]]"
  - "[[KKT Conditions]]"
  - "[[Maratos Effect]]"
  - "[[Damped BFGS Updating]]"
  - "[[Reduced-Hessian Quasi-Newton Approximations]]"
  - "[[Least-Squares Multipliers]]"
relationships:
  - source: "Sequential Quadratic Programming"
    target: "Quadratic Subproblem"
    description: "solves iterative"
  - source: "Sequential Quadratic Programming"
    target: "Lagrangian Hessian"
    description: "uses"
  - source: "Sequential Quadratic Programming"
    target: "Merit Function"
    description: "uses"
  - source: "Sequential Quadratic Programming"
    target: "Quasi-Newton Approximation"
    description: "employs"
  - source: "Sequential Quadratic Programming"
    target: "Maratos Effect"
    description: "suffers from"
  - source: "Sequential Quadratic Programming"
    target: "Damped BFGS Updating"
    description: "implements"
  - source: "Sequential Quadratic Programming"
    target: "Reduced-Hessian Quasi-Newton Approximations"
    description: "utilizes"
  - source: "Sequential Quadratic Programming"
    target: "Least-Squares Multipliers"
    description: "calculates"
  - source: "Interior Point Methods"
    target: "Barrier Parameter"
    description: "updates"
  - source: "Interior Point Methods"
    target: "Primal-Dual System"
    description: "solves"
  - source: "Interior Point Methods"
    target: "KKT Conditions"
    description: "satisfies"
  - source: "Interior Point Methods"
    target: "Barrier Parameter"
    description: "uses"
  - source: "Interior Point Methods"
    target: "Primal-Dual System"
    description: "relies on"
---

# Sequential Quadratic Programming And Interior Point Methods

*Sequential Quadratic Programming (SQP) and Interior Point Methods (IPM) are two primary frameworks for solving nonlinear programming problems. SQP focuses on iterative refinement of quadratic subproblems, while IPMs use barrier functions to guide iterates toward the solution. Both methods rely on efficient linear algebra and merit functions for step acceptance.*

This note explores the algorithmic development of two major classes of nonlinear optimization: [[Sequential Quadratic Programming]] and [[Interior Point Methods]].

## Concept

### Sequential Quadratic Programming
[[Sequential Quadratic Programming]] (SQP) is an iterative method for solving nonlinear programming problems by solving a sequence of [[Quadratic Subproblem]]s. Each subproblem is a quadratic model of the objective function and a linear approximation of the constraints. The effectiveness of the method depends on the heavily used [[Lagrangian Hessian]] and the choice of step-acceptance mechanisms.

In the case where the second derivatives are difficult to compute, [[Quasi-Newton Approximation]]s such as BFGS or SR1 are used to approximate the [[Lagrangian Hessian]]. To ensure positive definiteness in the presence of indefinite Hessians, [[Damped BFGS Updating]] is employed to ensure the update remains well-defined and positive definite.

One significant challenge in SQP is the [[Maratos Effect]], where a step that decreases the objective and constraints might still increase the merit function. To mitigate this, [[Second-Order Correction]] steps are used to improve the accuracy of the linear constraint approximations.

For large-scale problems, [[Reduced-Hessian Quasi-Newton Approximations]] are often preferred. These methods focus on approximating only the part of the Hessian that affects the component of the step in the null space of the constraints, which is more likely to be positive definite.

### Interior Point Methods

[[Interior Point Methods]] (IPM) solve nonlinear programs by incorporating inequality constraints into the objective function via a [[Barrier Parameter]] $\mu$ using a barrier function. As the parameter $\mu$ approaches zero, the iterates are guided toward the solution. The sequence of [[Barrier Parameter]] values must be carefully managed; strategies like the Fiacco-McCormick approach or adaptive strategies are used to update $\mu$.

Modern practical IPMs often solve a [[Primal-Dual System]] that arises from the KKT conditions. This system is typically solved using symmetric indefinite factorizations or iterative methods. The success of the of the [[Primal-Dual System]] depends on the ability to handle ill-conditioning that arises as $\mu \to 0$.

## Relationships

- [[Sequential Quadratic Programming]] solves iterative [[Quadratic Subproblem]]
- [[Sequential Quadratic Programming]] uses [[Lagrangian Hessian]]
- [[Sequential Quadratic Programming]] uses [[Merit Function]]
- [[Sequential Quadratic Programming]] employs [[Quasi-Newton Approximation]]
- [[Sequential Quadratic Programming]] suffers from [[Maratos Effect]]
- [[Sequential Quadratic Programming]] implements [[Damped BFGS Updating]]
- [[Sequential Quadratic Programming]] utilizes [[Reduced-Hessian Quasi-Newton Approximations]]
- [[Sequential Quadratic Programming]] calculates [[Least-Squares Multipliers]]
- [[Interior Point Methods]] updates [[Barrier Parameter]]
- [[Interior Point Methods]] solves [[Primal-Dual System]]
- [[Interior Point Methods]] satisfies [[KKT Conditions]]
- [[Interior Point Methods]] relies on [[Primal-Dual System]]
