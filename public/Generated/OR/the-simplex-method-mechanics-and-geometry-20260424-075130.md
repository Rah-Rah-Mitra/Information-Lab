---
type: content
title: "The Simplex Method Mechanics and Geometry"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:51:30.298881300+00:00
summary: "The simplex method is an iterative algorithm for solving linear programming problems by traversing the vertices of a feasible polytope. It utilizes basis matrices to navigate between adjacent vertices to optimize an objective function. The method's efficiency depends on pricing strategies and numerical stability in linear algebra updates."
tags:
  - operations-research
  - linear-programming
  - simplex-method
  - optimization
entities:
  - "[[Simplex Method]]"
  - "[[Basic Feasible Point]]"
  - "[[Basis Matrix]]"
  - "[[Feasible Polytope]]"
  - "[[Degeneracy]]"
  - "[[Pricing]]"
  - "[[LU Factorization]]"
  - "[[Two-Phase Approach]]"
  - "[[Phase-I Problem]]"
  - "[[Phase-II Problem]]"
  - "[[Steepest Edge Rule]]"
  - "[[Reduced Costs]]"
relationships:
  - source: "Simplex Method"
    target: "Feasible Polytope"
    description: "traverses vertices of"
  - source: "Simplex Method"
    target: "Basic Feasible Point"
    description: "iterates through"
  - source: "Simplex Method"
    target: "Basis Matrix"
    description: "uses"
  - source: "Simplex Method"
    target: "Two-Phase Approach"
    description: "implements via"
  - source: "Simplex Method"
    target: "Degeneracy"
    description: "is affected by"
  - source: "Simplex Method"
    target: "Pricing"
    description: "utilizes"
  - source: "Simplex Method"
    target: "LU Factorization"
    description: "maintains via"
  - source: "Simplex Method"
    target: "Reduced Costs"
    description: "calculates"
  - source: "Basic Feasible Point"
    target: "Feasible Polytope"
    description: "corresponds to vertices of"
  - source: "Two-Phase Approach"
    target: "Phase-I Problem"
    description: "consists of"
  - source: "Two-Phase Approach"
    target: "Phase-II Problem"
    description: "consists of"
  - source: "Phase-Phase I Problem"
    target: "Simplex Method"
    description: "solves via"
  - source: "Phase-II Problem"
    target: "Simplex Method"
    description: "solves via"
  - source: "Pricing"
    target: "Reduced Costs"
    description: "determines"
  - source: "Degeneracy"
    target: "Simplex Method"
    description: "can cause cycling in"
---

# The Simplex Method Mechanics and Geometry

*The simplex method is an iterative algorithm for solving linear programming problems by traversing the vertices of a feasible polytope. It utilizes basis matrices to navigate between adjacent vertices to optimize an objective function. The method's efficiency depends on pricing strategies and numerical stability in linear algebra updates.*

The [[Simplex Method]] is an iterative optimization algorithm used to solve linear programming problems of the form \( \min c^T x \) subject to \( Ax = b, x \ge 0 \). It moves along the edges of a [[Feasible Polytope]] to find an optimal solution.

## Concept
At each iteration, the algorithm maintains a [[Basis Matrix]] $B$, which is a nonsingular submatrix of $A$. A solution is defined by the components of $x$ corresponding to the indices in $B$. The set of all [[Basic Feasible Point]]s is equivalent to the geometrically recognized vertices of the polytope. 

$$ B x_B = b $$

This equation defines the basic solution for the current basis. A point is a [[Basic Feasible Point]] if $x_B \text{ is non-negative and } x_N = 0$. 

## Geometry and Degeneracy
All [[Basic Feasible Point]]s are vertices of the [[Feasible Polytope]]. A basis is considered to [[Degeneracy]] if one of its basic variables is zero, which can lead to [[Degenerate Steps]] where the objective function does not strictly decrease. This phenomenon can lead to [[Cycling]], where the algorithm returns to a previous basis, potentially preventing convergence.

## Implementation Details
To improve efficiency, implementations often use [[LU Factorization]] of the basis matrix $B$ to solve linear systems rather than computing $B^{-1}$ directly. 

$$ L U = B $$

This represents the decomposition of the factors used for triangular substitution.

### Pricing and Selection
Choosing the entering index $q$ is a function of [[Pricing]]. During [[Pricing]], the algorithm calculates the [[Reduced Costs]] $s_N$ for nonbasic variables. 

$$ s_N^T = c_N^T - c_B^T B^{-1} N $$

An entering index is chosen based on rules like the Dantzig rule or the [[Steepest Edge Rule]], which seeks to largest decrease in the objective function per unit distance moved along the edge.

### The Two-Phase Approach
When an initial [[Basic Feasible Point]] is not readily available, the [[Two-Phase Approach]] is used. In [[Phase-I Problem]], an auxiliary problem is constructed to find a feasibility. In [[Phase-II Problem]], the original objective is optimized using the solution from Phase I.

## Relationships
- [[Simplex Method]] traverses vertices of [[Feasible Polytope]]
- [[Simplex Method]] iterates through [[Basic Feasible Point]]
- [[Simplex Method]] uses [[Basis Matrix]]
- [[Simplex Method]] is affected by [[Degeneracy]]
- [[Simplex Method]] utilizes [[Pricing]]
- [[Simplex Method]] maintains via [[LU Factorization]]
- [[Simplex Method]] implements via [[Two-Phase Two-Phase Approach]]
- [[Basic Feasible Point]] corresponds to vertices of [[Feasible Polytope]]
- [[Pricing]] determines [[Reduced Costs]]
- [[Two-Phase Approach]] consists of [[Phase-I Problem]] and [[Phase-II Problem]]
