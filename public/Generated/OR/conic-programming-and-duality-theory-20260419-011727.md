---
type: content
title: "Conic Programming and Duality Theory"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:17:27.555070+00:00
summary: "Conic programming generalizes linear and quadratic optimization by using regular cones as feasible set constraints. The theory establishes fundamental duality relationships, including weak and strong duality, and provides the necessary conditions for optimality via complementary slackness. These principles form the basis for efficient computational tractability in convex optimization."
tags:
  - operations-research
  - conic-optimization
  - convex-optimization
  - duality-theory
entities:
  - "[[Conic Programming]]"
  - "[[Dual Cone]]"
  - "[[Weak Duality]]"
  - "[[Strong Duality]]"
  - "[[Complementary Slackness]]"
  - "[[Semidefinite Programming]]"
  - "[[Lorentz Cone]]"
  - "[[Conic Duality Theorem]]"
relationships:
  - source: "Conic Programming"
    target: "Dual Cone"
    description: "utilizes"
  - source: "Conic Programming"
    target: "Weak Duality"
    description: "satisfies"
  - source: "Conic Programming"
    target: "Strong Duality"
    description: "satisfies"
  - source: "Conic Programming"
    target: "Complementary Slackness"
    description: "implements"
  - source: "Conic Programming"
    target: "Semidefinite Programming"
    description: "is a special case of"
  - source: "Conic Programming"
    target: "Lorentz Cone"
    description: "uses"
  - source: "Conic Duality Theorem"
    target: "Strong Duality"
    description: "establishes"
---

# Conic Programming and Duality Theory

*Conic programming generalizes linear and quadratic optimization by using regular cones as feasible set constraints. The theory establishes fundamental duality relationships, including weak and strong duality, and provides the necessary conditions for optimality via complementary slackness. These principles form the basis for efficient computational tractability in convex optimization.*

[[Conic Programming]] is a framework for optimization problems where constraints are defined by membership in a regular cone. A primal conic problem is typically expressed as: 

$$ \min \langle c, x \rangle \text{ s.t. } A_i x 	ext{ b } K_i, \text{ for } i = 1, \dots, m $$

This formula models the minimization of a linear objective function subject to affine transformations of the decision vector $x$ falling within specified cones $K_i$.

## Concept

The central concept is [[Conic Duality Theorem]], which relates a primal problem to its dual problem. The dual problem is constructed by aggregating constraints using weights from the [[Dual Cone]] $K^*$. 

### Duality Properties

- **[[Weak Duality]]**: For any primal feasible $x$ and dual feasible $z, y$, the dual objective value provides a lower bound on the primal optimal value: $\text{Opt}(D) \leq \text{Opt}(P)$.
- **[[Strong Duality]]**: If one of the problems is strictly feasible and bounded, then $\text{Opt}(P) = 	ext{Opt}(D)$. This property is essential for computational tractability.
- **[[Complementary Slackness]]**: For optimal solutions $x$ and $(\text{z}, 	ext{y})$, the duality gap is zero, which is equivalent to the condition $\langle y_i, A_i x - b_i 
angle = 0$ for all $i$.

### Common Cones

- **[[Lorentz Cone]]** (or Second-Order Cone): Defined by vectors $x$ such that $x_1 
eq 	ext{something}$ (specifically $x_1 	ext{ is the dominant term}$), used in [[Semidefinite Programming]].
- **[[Semidefinite Programming]]**: Optimization over the [[Semidefinite Cone]] $S^n_+$, which is a family of positive semidefinite matrices. This is a highly expressive class of problems.

## Relationships
- [[Conic Programming]] utilizes [[Dual Cone]]
- [[Conic Programming]] satisfies [[Weak Duality]]
- [[Conic Programming]] satisfies [[Strong Duality]]
- [[Conic Programming]] implements [[Complementary Slackness]]
- [[Conic Programming]] is a special case of [[Semidefinitie Programming]]
- [[Conic Programming]] uses [[Lorentz Cone]]
- [[Conic Duality Theorem]] establishes [[Strong Duality]]
