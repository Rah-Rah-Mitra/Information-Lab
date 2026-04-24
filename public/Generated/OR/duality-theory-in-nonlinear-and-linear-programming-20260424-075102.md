---
type: content
title: "Duality Theory in Nonlinear and Linear Programming"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:51:02.679037300+00:00
summary: "Duality theory establishes a relationship between a primal optimization problem and an alternative dual problem, providing lower bounds and algorithmic motivation. In linear programming, strong duality ensures that the optimal objective values of the primal and dual are identical. This theory is also used for sensitivity analysis, providing insight into how perturbations in constraints affect the optimal value."
tags:
  - operations-research
  - optimization-theory
  - duality
  - linear-programming
  - nonlinear-programming
entities:
  - "[[Duality Theory]]"
  - "[[Primal Problem]]"
  - "[[Lagrangian Function]]"
  - "[[Dual Problem]]"
  - "[[Dual Objective Function]]"
  - "[[Karush-Kuhn-Tucker Conditions]]"
  - "[[Weak Duality]]"
  - "[[Strong Duality]]"
  - "[[Sensitivity Analysis]]"
  - "[[Linear Programming]]"
  - "[[Wolfe Dual]]"
relationships:
  - source: "Duality Theory"
    target: "Primal Problem"
    description: "relates to"
  - source: "Duality Theory"
    target: "Dual Problem"
    description: "relates to"
  - source: "Dual Problem"
    target: "Dual Objective Function"
    description: "is defined by"
  - source: "Primal Problem"
    target: "Lagrangian Function"
    description: "is associated with"
  - source: "Primal Problem"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Dual Problem"
    target: "Karush-Kuhn-Tucker Conditions"
    description: "satisfies"
  - source: "Weak Duality"
    target: "Dual Problem"
    description: "provides a lower bound via"
  - source: "Strong Duality"
    target: "target"
    description: "ensures equality of objectives between primal and dual"
  - source: "Sensitivity Analysis"
    target: "Lagrangian Function"
    description: "uses multipliers from"
  - source: "Linear Programming"
    target: "Duality Theory"
    description: "is a special case of"
  - source: "Wolfe Dual"
    target: "Dual Problem"
    description: "is a form of"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Lagrangian Function"
    description: "involves"
  - source: "Dual Problem"
    target: "Primal Problem"
    description: "is symmetric to"
---

# Duality Theory in Nonlinear and Linear Programming

*Duality theory establishes a relationship between a primal optimization problem and an alternative dual problem, providing lower bounds and algorithmic motivation. In linear programming, strong duality ensures that the optimal objective values of the primal and dual are identical. This theory is also used for sensitivity analysis, providing insight into how perturbations in constraints affect the optimal value.*

This note explores the mathematical framework of [[Duality Theory]], which connects an optimization problem (the [[Primal Problem]]) to an alternative formulation (the [[Dual Problem]]). This relationship is fundamental to both nonlinear and linear programming.

## Concept
In nonlinear programming, given a primal problem of the form

$$ \min f(x) \text{ subject to } c_i(x) \geq 0, \quad i=1, \textdots, m $$
	he Lagrangian function is defined as

$$ L(x, \lambda) = f(x) - \sum_{i=1}^m \lambda_i c_i(x) $$

The [[Dual Objective Function]] $q(\lambda)$ is then defined as the infimum of the Lagrangian over $x$:

$$ q(\lambda) = \inf_x L(x, \lambda) $$ 

This function $q(\lambda)$ is always concave, and its domain is convex. A key result is [[Weak Duality]], which states that for any primal feasible $x$ and any $\ $\lambda \geq 0$, we have $q(\lambda) \leq f(x)$. This implies the dual objective provides a lower bound on the primal optimal value.\n\nIn [[Linear Programming]], the relationship is even stronger. Under the [[Strong Duality]] theorem, if either the primal or dual has a finite solution, their optimal objective values are equal.\n\n## Karush-Kuhn-Tucker Conditions\nThe optimality of a solution is characterized by the [[Karush-Kuhn-Tucker Conditions]] (KKT), which for the linear case include complementarity: \n\n$$ x_i s_i = 0, 	ext{ for all } i=1, 	extdots, m $$\n\nThese conditions ensure that the relationship between the primal and dual variables is maintained at optimality.\n\n## Sensitivity Analysis\nThe optimal Lagrange multipliers $\lambda$ provide insight into [[Sensitivity Analysis]]. Specifically, the change in the optimal objective value due to a small perturbation in the constraint vector $b$ is given by:\n\n$$ \Delta f(x^*) \approx \lambda_j \cdot \Delta b_j $$\n\nThis shows that the multipliers represent the marginal value of the constraints.\n\n## Relationships\n- [[Duality Theory]] relates to [[Primal Problem]]\n- [[Duality Theory]] relates to Dual Problem\n- [[Dual Problem]] is defined by [[Dual Objective Function]]\n- [[Primal Problem]] is associated with [[Lagrangian Function]]\n- [[Primal Problem]] satisfies [[Karush-Kuhn-Tucker Conditions]]\n- [[Dual Problem]] satisfies [[Karush-Kuhn-Tucker Conditions]]\n- [[Weak Duality]] provides a lower bound via [[Dual Problem]]\n- [[Strong Duality]] ensures equality of objectives between primal and dual\n- [[Sensitivity Analysis]] uses multipliers from [[Lagrangian Function]]\n- [[Linear Programming]] is a special case of [[Duality Theory]]\n- [[Wolfe Dual]] is a form of [[Dual Problem]]\n- [[Karush-Kuhn-Tucker Conditions]] involves [[Lagrangian Function]]\n- [[Dual Problem]] is symmetric to [[Primal Problem]]
