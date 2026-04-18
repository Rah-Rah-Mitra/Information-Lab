---
type: content
title: "Conic Programming and Duality"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:12:14.161379900+00:00
summary: "An overview of conic optimization, focusing on Euclidean spaces, regular cones, conic duality, and the computational tractability of convex programs."
tags:
  - gis
  - optimization
  - convex-programming
  - linear-algebra
  - mathematics
entities:
  - "[[Euclidean Space]]"
  - "[[Inner Product]]"
  - "[[Conic Programming]]"
  - "[[Regular Cone]]"
  - "[[Dual Cone]]"
  - "[[Lorentz Cone]]"
  - "[[Semidefinite Cone]]"
  - "[[Conic Duality Theorem]]"
  - "[[Weak Duality]]"
  - "[[Strong Duality]]"
  - "[[Polynomial Time Solution Algorithm]]"
  - "[[Ellipsoid Method]]"
relationships:
  - source: "Conic Programming"
    target: "Euclidean Space"
    description: "is defined over"
  - source: "Conic Programming"
    target: "Regular Cone"
    description: "utilizes"
  - source: "Regular Cone"
    target: "Dual Cone"
    description: "has a"
  - source: "Lorentz Cone"
    target: "Regular Cone"
    description: "is an example of"
  - source: "Semidefinite Cone"
    target: "Regular Cone"
    description: "is an example of"
  - source: "Conic Duality Theorem"
    target: "Weak Duality"
    description: "includes"
  - source: "Conic Duality Theorem"
    target: "Strong Duality"
    description: "includes"
  - source: "Polynomial Time Solution Algorithm"
    target: "Conic Programming"
    description: "solves"
  - source: "Ellipsoid Method"
    target: "Polynomial Time Solution Algorithm"
    description: "is an instance of"
  - source: "Euclidean Space"
    target: "Inner Product"
    description: "is equipped with"
---

# Conic Programming and Duality

*An overview of conic optimization, focusing on Euclidean spaces, regular cones, conic duality, and the computational tractability of convex programs.*

[[Conic Programming]] is a framework for optimization problems where the decision vectors reside in a [[Euclidean Space]] and constraints are defined by membership in [[Regular Cone]]s.

## Euclidean Spaces and Cones

A [[Euclidean Space]] is a finite-dimensional linear space over reals equipped with an [[Inner Product]]. A subset $K$ of such a space is a cone if it is a convex set comprised of rays emanating from the origin. A cone is considered a [[Regular Cone]] if it is closed, pointed, and possesses a nonempty interior.

Every regular cone $K$ has a corresponding [[Dual Cone]] $K^*$, defined as the set of all vectors that have a non-negative inner product with all vectors in $K$. Notable examples of regular cones include the [[Lorentz Cone]] (used in second-order cone programming) and the [[Semidefinite Cone]] (used in semidefinite programming).

## Conic Duality

[[Conic Programming]] involves a primal problem (P) and a dual problem (D). The [[Conic Duality Theorem]] establishes the relationship between them:

- **[[Weak Duality]]**: The optimal value of the dual problem always provides a lower bound for the optimal value of the primal problem.
- **[[Strong Duality]]**: Under conditions of strict feasibility, the primal and dual optimal values are equal, and both problems are solvable.

## Computational Tractability

A generic convex optimization problem is considered computationally tractable if it admits a [[Polynomial Time Solution Algorithm]]. This requires the problem to be polynomially computable, exhibit polynomial growth, and have polynomially bounded feasible sets.

One such algorithm is the [[Ellipsoid Method]], which can find an $\epsilon$-solution to a convex problem using a separation oracle and a first-order oracle in polynomial time relative to the problem size and accuracy digits.
