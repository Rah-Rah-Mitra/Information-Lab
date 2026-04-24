---
type: content
title: "Quadratic Programming and Penalty Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:57:45.985129200+00:00
summary: "This note covers various numerical methods for solving quadratic programming problems, including interior-point, gradient projection, and penalty methods. It contrasts active-set and active-set methods and discusses the limitations of the and the ill-conditioning of the Hessian in penalty methods. The note provides a understanding of the convergence and implementation considerations for large-scale problems."
tags:
  - optimization
  - quadratic-programming
  - numerical-methods
  - mathematics
entities:
  - "[[Quadratic Programming]]"
  - "[[Interior-Point Method]]"
  - "[[Predictor-Corrector Algorithm]]"
  - "[[Gradient Projection Method]]"
  - "[[Cauchy Point]]"
  - "[[Quadratic Penalty Method]]"
  - "[[Quadratic Penalty Function]]"
  - "[[Active-Set Method]]"
  - "[[Convex Quadratic Programming]]"
  - "[[Lagrange Multipliers]]"
relationships:
  - source: "Quadratic Programming"
    target: "Interior-Point Method"
    description: "solved by"
  - source: "Quadratic Programming"
    target: "Gradient Projection Method"
    description: "solved by"
  - source: "Quadratic Programming"
    target: "Active-Set Method"
    description: "solved by"
  - source: "Quadratic Programming"
    target: "Quadratic Penalty Method"
    description: "solved by"
  - source: "Interior-Point Method"
    target: "Predictor-Corrector Algorithm"
    description: "uses"
  - source: "Gradient Projection Method"
    target: "Cauchy Point"
    description: "computes"
  - source: "Quadratic Penalty Method"
    target: "Quadratic Penalty Function"
    description: "uses"
  - source: "Active-Set Method"
    target: "Quadratic Programming"
    description: "solves"
  - source: "Lagrange Multipliers"
    target: "Quadratic Programming"
    description: "associated with"
---

# Quadratic Programming and Penalty Methods

*This note covers various numerical methods for solving quadratic programming problems, including interior-point, gradient projection, and penalty methods. It contrasts active-set and active-set methods and discusses the limitations of the and the ill-conditioning of the Hessian in penalty methods. The note provides a understanding of the convergence and implementation considerations for large-scale problems.*

This note explores several numerical approaches for solving [[Quadratic Programming]] problems, which involve minimizing a quadratic objective function subject to constraints. 

## Concept

### [[Quadratic Programming]]
[[Quadratic Programming]] is the optimization of a quadratic function subject to constraints. A common form is:

$$ \min_{x} \frac{1}{2} x^T G x + c^T x \quad \text{subject to} \quad Ax = b, \ Gx \leq c \text{ or } l \leq x \leq u $$

where $G$ is a symmetric matrix. 

### [[Interior-Point Method]]
An [[Interior-Point Method]] is a class of algorithms that approach the solution from the interior of the feasible region. The most popular version for [[Convex Quadratic Programming]] is based on [[Predictor-Corrector Algorithm]] (Mehrotra's), which uses an affine scaling step followed by a corrector step to improve the step. 

### [[Active-Set Method]]
An [[Active-Set Method]] is an iterative method where the algorithm maintains a set of constraints that are currently active (at the boundary). These methods are typically more efficient for small to medium-sized problems or when a "warm start" is available. In contrast, [[Interior-Point Method]]s are often more efficient for very large-scale problems. 

### [[Gradient Projection Method]]
The [[Gradient Projection Method]] is an active-set style method that is particularly efficient for bound-constrained problems. It involves two stages: finding a [[Cauchy Point]] and then performing a subspace minimization. 

In the first stage, the [[Cauchy Point]] is found by searching along the steepest descent direction and "bending" the path whenever a bound is encountered. The path is defined by the 

$$ x(t) = P(x - tg) $$

where $P$ is the projection operator. The [[Cauchy Point]] is the first local minimizer along this piecewise-linear path. 

### [[Quadratic Penalty Method]]
The [[Quadratic Penalty Method]] is a technique for handling constraints by adding a penalty term to the objective function. The 

[[Quadratic Penalty Function]] is defined as:

$$ Q(x; \mu) = f(x) + \frac{\mu}{2} \sum_{i \in E} (c_i(x))^2 $$ 

where $\mu$ is the penalty parameter. As $\mu \to \infty$, the minimizer of the penalty function approaches the solution of the constrained problem. However, a major drawback is the나 ill-conditioning of the Hessian of the penalty function as $\mu$ becomes large, which can make unconstrained minimization difficult. 

## Relationships

- [[Quadratic Programming]] is solved by [[Interior-Point Method]], [[Gradient Projection Method]], and [[Active-Set Method]].
- [[Interior-Point Method]] uses [[Predictor-Corretor Algorithm]].
- [[Gradient Projection Method]] computes [[Cauchy Point]].
- [[Quadratic Penalty Method]] uses [[Quadratic Penalty Function]].
- [[Lagrange Multipliers]] are associated with [[Quadratic Programming]].
