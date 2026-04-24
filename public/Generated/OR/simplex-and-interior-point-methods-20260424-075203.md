---
type: content
title: "Simplex and Interior-Point Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:52:03.584380300+00:00
summary: "This note compares the simplex method and interior-point methods for solving linear programming problems. The simplex method traverses the boundary of the feasible polytope via vertices, while interior-point methods approach the solution through the interior of the feasible region. The choice between them depends on the theoretical complexity and worst-case performance of Klee-Minty cubes."
tags:
  - linear-programming
  - optimization-algorithms
  - simplex-method
  - interior-point-methods
entities:
  - "[[Simplex Method]]"
  - "[[Dual Simplex Method]]"
  - "[[Interior-Point Methods]]"
  - "[[Primal-Dual Interior-Point Methods]]"
  - "[[Karush-Kuhn-Tucker Conditions]]"
  - "[[Presolving]]"
  - "[[Central Path]]"
  - "[[Duality Measure]]"
  - "[[Active Set Methods]]"
relationships:
  - source: "Simplex Method"
    target: "Active Set Methods"
    description: "is a member of the class of"
  - source: "Simplex Method"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Central Path"
    description: "use"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Duality Measure"
    description: "utiliesizes"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Duality-Measure"
    description: "reduces"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Central-Path"
    description: "follows"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Simplex Method"
    target: "Dual Simplex Method"
    description: "is related to"
  - source: "Dual Simplex Method"
    target: "Simplex Method"
    description: "is a variant of"
  - source: "Presolving"
    target: "Simplex Method"
    description: "precedes"
  - source: "Simplex Method"
    target: "Primal-Dual Interior-Point Methods"
    description: "is compared to"
  - source: "Simplex Method"
    target: "Duality Measure"
    description: "is not based on"
  - source: "Simplex Method"
    target: "Central Path"
    description: "is not based on"
  - source: "Simplex Method"
    target: "Presolving"
    description: "is often preceded by"
---

# Simplex and Interior-Point Methods

*This note compares the simplex method and interior-point methods for solving linear programming problems. The simplex method traverses the boundary of the feasible polytope via vertices, while interior-point methods approach the solution through the interior of the feasible region. The choice between them depends on the theoretical complexity and worst-case performance of Klee-Minty cubes.*

This note explores the fundamental algorithms used to solve linear programming problems, specifically comparing the simplex method and interior-point methods.

## Concept

In linear programming, the objective is to minimize a linear function subject to equality and inequality constraints. Two primary approaches exist:

1. [[Simplex Method]]:
   This method works by traversing the boundary of the feasible polytope, testing a sequence of vertices (basic feasible solutions) basic feasible solutions) in turn until an optimal vertex is found. Geometrically, it moves along the edges of the polytope boundary.

2. [[Interior-Point Methods]]:
   Unlike the simplex method, simplex method, it approaches the solution through the interior of the feasible region. These methods are the focus of the next chapter, specifically [[Primal-Dual Interior-Point Methods]].

### Optimality Conditions

Both methods are characterized by the [[Karush-Kuhn-Tucker Conditions]] (KKT conditions), which define the optimality of a solution $(x, \lambda, s)$. A solution is optimal if it satisfies:

$$ A^T \lambda + s = c, \quad Ax = b, \quad x_i s_i = 0, \text{ for } i=1, \text{ ..., } n, \text{ and } x, s \ge 0 $$

This system of equations and inequalities represents the core of the optimality criteria for linear programming. The [[Primal-Dual Interior-Point Methods]] use Newton's method to solve this system while maintaining the strict positivity of $x$ and $s$ (i.e., $x, s > 0$) at every iteration. This property is known as being an "interior-point" method.

### Key Components of Primal-Dual Methods

[[Primal-Dual Interior-Point Methods]] use several key concepts to be effective:

- [[Duality Measure]] ($\mu$): This is the average value of the pairwise products $x_i s_i$. It is defined as:
  $$ \mu = \frac{1}{n} \sum_{i=1}^n x_i s_i $$
  The goal of the goal is to reduce $\mu$ to zero. 

- [[Central Path]] ($\mathcal{C}$): An arc of strictly feasible points that guides the algorithms toward the solution. It is parametrized by a $\tau > 0$ and satisfies the conditions:
  $$ A^T \lambda + s = c, \text    Ax = b, \text    x_i s_i = \tau, \text    x, s > 0 $$
  As $\tau \to 0$, the central path converges to a primal-dual solution.

- [[Centering Parameter]] ($\sigma$): Used to control the step length and direction in Newton steps. A centering step ($\sigma=1$) moves the iterate toward the central path, while an affine scaling step ($\sigma=0$) aims directly at reducing the duality measure.

### Presolving

[[Presolving]] is a preprocessing step used to reduce the problem size before passing it to a solver. Techniques include identifying row singletons, free column singletons, and dominated constraints. This can significantly reduce the computational load on the simplex method or interior-point methods.

## Relationships

- [[Simplex Method]] is a member of the class of [[Active Set Methods]].
- [[Simplex Method]] satisfies [[Karush-Kuhn-Tucker Conditions]].
- [[Primal-Dual Interior-Point Methods]] satisfies [[Karush-Kuhn-Tucker Conditions]].
- [[Primal-Dual Interior-Point Methods]] use [[Duality Measure]].
- [[Primal-Dual Interior-Point Methods]] uses [[Central Path]] to guide the search.
- [[Simplex Method]] is compared to [[Primal-Dual Interior-Point Methods]].
- [[Presolving]] precedes [[Simplex Method]].
- [[Dual Simplex Method]] is a variant of [[Simplex Method]].
