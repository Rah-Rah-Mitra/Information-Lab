---
type: content
title: "Karush-Kuhn-Tucker Optimality Conditions"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:48:35.028383200+00:00
summary: "The Karush-Kuhn-Tucker (KKT) conditions provide first-order necessary conditions for a point to be a local minimizer in constrained optimization. They relate the gradient of the objective function to the gradients of the active constraints via Lagrange multipliers. These conditions are fundamental to the foundation of most modern optimization algorithms."
tags:
  - operations-research
  - constrained-optimization
  - mathematical-programming
  - optimization-theory
entities:
  - "[[Karush-Kuhn-Tucker Conditions]]"
  - "[[Lagrange Multiplier]]"
  - "[[Linear Independence Constraint Qualification]]"
  - "[[Strict Complementarity]]"
  - "[[Critical Cone]]"
  - "[[Lagrangian Function]]"
  - "[[Farkas' Lemma]]"
  - "[[Mangasarian-Fromovitz Constraint Qualification]]"
  - "[[Second-Order Necessary Conditions]]"
  - "[[Second-Order Sufficient Conditions]]"
  - "[[Normal Cone]]"
  - "[[Tangent Cone]]"
relationships:
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Lagrange Multiplier"
    description: "utilize"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Lagrangian Function"
    description: "derived from"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Linear Independence Constraint Qualification"
    description: "requires"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Strict Complementarity"
    description: "relates to"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Critical Cone"
    description: "defines"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Farkas' Lemma"
    description: "proven using"
  - source: "Karue-Kuhn-Tucker Conditions"
    target: "Second-Order Necessary Conditions"
    description: "forms basis for"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Second-Order Sufficient Conditions"
    description: "forms basis for"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Normal Cone"
    description: "relates to"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Mangasarian-Fromovitz Constraint Qualification"
    description: "compared to"
  - source: "Karush-Kuhn-Tucker Conditions"
    target: "Tangent Cone"
    description: "relates to"
---

# Karush-Kuhn-Tucker Optimality Conditions

*The Karush-Kuhn-Tucker (KKT) conditions provide first-order necessary conditions for a point to be a local minimizer in constrained optimization. They relate the gradient of the objective function to the gradients of the active constraints via Lagrange multipliers. These conditions are fundamental to the foundation of most modern optimization algorithms.*

This note explores the first and second-order optimality conditions for constrained optimization problems, specifically focusing on the Karush-Kuhn-Tucker (KKT) conditions.

## Concept
In constrained optimization, we seek to minimize an objective function $f(x)$ subject to equality and inequality constraints $c_i(x) = 0$ and $c_i(x) \nless 0$. A point $x$ is a local minimizer if it satisfies the [[Karush-Kuhn-Tucker Conditions]]. These conditions are necessary for optimality, first-order conditions involving the gradients of the objective and constraints.

The [[Lagrangian Function]] is defined as:

$$ L(x, \lambda) = f(x) + \sum_{i \in E} \lambda_i c_i(x) + \sum_{i \in I} \lambda_i c_i(x) $$

This function combines the objective and constraints into a single expression. The KKT conditions at a local solution $x$ require that the gradient of the Lagrangian is zero:

$$ \nabla_x L(x, \lambda) = 0 $$

And that the constraints are satisfied, and that the complementarity conditions hold:

$$ c_i(x) = 0, \quad \lambda_i = 0 \text{ for } i \notin A(x) \quad \text{and} \n\lambda_i c_i(x) = 0 \text{ for all } i \text{ (complementarity)} $$

For these conditions to hold, a [[Linear Independence Constraint Qualification]] (LICQ) is typically required. [[LICQ]] holds if the set of active constraint gradients $\nabla c_i(x)$ is linearly independent. If LICQ holds, the [[Lagrange Multiplier]] is unique.

## Second-Order Conditions
While first-order KKT conditions identify candidates for optimality, they cannot distinguish between a local minimum, a local maximum, well or a saddle point. [[Second-Order Necessary Conditions]] require that the Hessian of the Lagrangian has non-negative curvature along the directions in the [[Critical Cone]] $C(x, \0) = \{w \mid \nabla c_i(x)^T w = 0, \text\{i \in A(x) \mid \lambda_i = 0\}$.

$$ \nabla_{xx}^2 L(x, \lambda) w^T w \ge 0 \quad \text{for all } w \in C(x, \lambda) $$

Conversely, [[Second-Order Sufficient Conditions]] provide a sufficient condition for a point to be a strict local solution. This involves a strict inequality for the Hessian of the Lagrangian along the critical cone directions.

$$ \nabla_{xx}^2 L(x, \lambda) w^T w > 0 \quad \text{for all } w \neq 0, w \text{ in } C(x, \lambda) \) \n\n## Geometric and Alternative Viewpoints\n\n### Constraint Qualifications\n\nThere are several [[Constraint Qualifications]] (CQs) easily compared. [[Mangasarian-Fromovitz Constraint Qualification]] (MFCQ) is a weaker condition than LICQ, ensuring the boundedness of the set of Lagrange multipliers. \n\n### Geometric Viewpoint\n\nGeometrically, the first-order optimality conditions can be expressed using the [[Tangent Cone]] and the [[Normal Cone]]. At a local minimizer $x$, the gradient of the objective function must be orthogonal to the tangent cone, and thes is a normal vector to the set. A vector $v$ is in the [[Normal Cone]] $N(x)$ if it makes an angle of at least $\pi/2$ with every tangent vector $w 	ext{ in } T(x)$.\n\n$$ \nabla f(x) 	ext{ is in } N(x) $$ \n\n## Relationships\n- [[Karush-Kuhn-Tucker Conditions]] produces [[Lagrange Multiplier]]\n- [[Karush-Kuhn-Tucker Conditions]] requires [[Linear Independence Constraint Qualification]]\n- [[Karush-Kuhn-Tucker Conditions]] relates to [[Strict Complementarity]]\n- [[Karush-Kuhn-Tucker Conditions]] relates to [[Critical Cone]]\n- [[Karush-Kuhn-Tucker Conditions]] is proven using [[Farkas' Lemma]]\n- [[Karush-Kuhn-Tucker Conditions]] forms basis for [[Second-Order Necessary Conditions]]\n- [[Karush-Kuhn-Tucker Conditions]] forms basis for [[Second-Order Sufficient Conditions]]\n- [[Karush-Kuhn-Tucker Conditions]] relates to [[Normal Cone]]\n- [[Karush-Kuhn-Tucker-Conditions]] relates to [[Tangent Cone]]\n- [[Karush-Kuhn-Tucker Conditions]] relates to [[Mangasarian-Fromovitz Constraint Qualification]]
