---
type: content
title: "Active-Set and Interior-Point Methods for Convex QP"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:09:22.104645100+00:00
summary: "Active-set methods iteratively update a working set of constraints to find the optimal solution of a convex quadratic program. Interior-point methods approach the solution via a central path of perturbed KKT conditions. Both are fundamental for large-scale optimization."
tags:
  - operations-research
  - quadratic-programming
  - optimization-algorithms
  - convex-optimization
entities:
  - "[[Active-Set Method]]"
  - "[[Interior-Point Method]]"
  - "[[Convex Quadratic Programming]]"
  - "[[Convex Optimization]]"
  - "[[KKT Conditions]]"
  - "[[Lagrange Multipliers]]"
  - "[[Lagrange Multipliers]]"
  - "[[Working Set]]"
  - "[[Central Path]]"
  - "[[Reduced Hessian]]"
relationships:
  - source: "Active-Set Method"
    target: "Convex Quadratic Programming"
    description: "solves"
  - source: "Active-Set Method"
    target: "Working Set"
    description: "uses"
  - source: "Interior-Point Method"
    target: "Convex Quadratic Programming"
    description: "solves"
  - source: "Interior-Point Method"
    target: "Central Path"
    description: "follows"
  - source: "Active-Set Method"
    target: "KKT Conditions"
    description: "satisfies"
  - source: "Interior-Point Method"
    target: "KKT Conditions"
    description: "satisfies"
  - source: "Working Set"
    target: "KKT Conditions"
    description: "relates to"
  - source: "Central Path"
    target: "KKT Conditions"
    description: "is defined by"
  - source: "Reduced Hessian"
    target: "Active-Set Method"
    description: "is updated in"
---

# Active-Set and Interior-Point Methods for Convex QP

*Active-set methods iteratively update a working set of constraints to find the optimal solution of a convex quadratic program. Interior-point methods approach the solution via a central path of perturbed KKT conditions. Both are fundamental for large-scale optimization.*

This note explores the two primary algorithmic families for solving [[Convex Quadratic Programming]] problems: [[Active-Set Method]] and [[Interior-Point Method]].

## Concept

### Active-Set Methods
An [[Active-Set Method]] solves a quadratic program by maintaining a subset of constraints, known as the [[Working Set]], which are treated as equalities. The algorithm iterates by either adding a constraint to the working set (if a step is blocked) or removing a constraint (if its [[Lagrange Multipliers]] are negative). 

Given an iterate \(x_k\), the algorithm solves a subproblem to find a step \(p_k\): 

$$ \min_{p} \frac{1}{2} p^T G p + g^T p \quad \text{subject to} \quad a_i^T p = 0, \text{ for } i \in W_k $$

This equation models the step direction that minimizes the objective while staying on the current active constraints. 

If the step length \(α_k\) is less than 1, a new constraint is added to the working set. If the step length is 1, the algorithm checks the [[Lagrange Multipliers]] \(λ_i\) for the constraints in the working set. If any \(λ_i < 0\), that constraint is removed from the [[Working Set]].

### Interior-Point Methods
An [[Interior-Point Method]] approaches the solution of a [[Convex Quadratic Programming]] problem by following the [[Central Path]], a trajectory of solutions to perturbed [[KKT Conditions]]. 

By introducing a slack vector \(y\) and a complementarity measure \(μ = \lambda^T y$, the the method solves the perturbed system:\n\n$$ Gx + A^T \lambda = c \n A x + y = b \n Y \lambda e = \sigma 	ext{μ} e $$\n\nThis system models the central path where the complementarity between primal and dual variables is controlled by the parameter \(σ\). As \(σ \to 0$, the path converges to the optimal solution.

### Computational Efficiency
In [[Active-Set Method]] implementations, efficiency is often achieved by updating factorizations (such as QR or Cholesky) of the [[Reduced Hessian]] or the KKT matrix, rather than recomputing them from scratch. This is particularly useful when the working set changes by only one index per iteration.

## Relationships
- [[Active-Set Method]] solves [[Convex Quadratic Programming]]
- [[Active-Set Method]] uses [[Working Set]]
- [[Interior-Point Method]] solves [[Convex Quadratic Programming]]
- [[Interior-Point Method]] follows [[Central Path]]
- [[Active-Set Method]] satisfies [[KKT Conditions]]
- [[Interior-Point Method]] satisfies [[KKT Conditions]]
- [[Working Set]] relates to [[KKT Conditions]]
- [[Central Path]] is defined by [[KKT Conditions]]
- [[Reduced Hessian]] is updated in [[Active-Set Method]]
