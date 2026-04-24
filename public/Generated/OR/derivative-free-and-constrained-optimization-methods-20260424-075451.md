---
type: content
title: "Derivative-Free and Constrained Optimization Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:54:51.332540800+00:00
summary: "This note covers several optimization techniques including conjugate-direction methods, Nelder-Mead simplex, and filter methods for constrained problems. It highlights the Maratos effect, where good steps are rejected by merit functions, and discusses remedies like second-order corrections and nonmonotone strategies. These methods are essential for when gradients are unavailable or constraints must be managed efficiently."
tags:
  - optimization
  - derivative-free-optimization
  - constrained-optimization
  - numerical-methods
entities:
  - "[[Conjugate-Direction Method]]"
  - "[[Nelder-Mead Method]]"
  - "[[Implicit Filtering]]"
  - "[[Filter Method]]"
  - "[[Maratos Effect]]"
  - "[[Second-Order Correction]]"
  - "[[Nonmonotone Strategy]]"
  - "[[Merit Function]]"
relationships:
  - source: "Conjugate-Direction Method"
    target: "Derivative-Free Optimization"
    description: "is a type of"
  - source: "Nelder-Mead Method"
    target: "Derivative-Free Optimization"
    description: "is a type of"
  - source: "Implicit Filtering"
    target: "Derivative-Free Optimization"
    description: "is a type of"
  - source: "Maratos Effect"
    target: "Filter Method"
    description: "affects"
  - source: "Maratos Effect"
    target: "Merit Function"
    description: "affects"
  - source: "Second-Order Correction"
    target: "Maratos Effect"
    description: "remedies"
  - source: "Nonmonotone Strategy"
    target: "Maratos Effect"
    description: "remedies"
---

# Derivative-Free and Constrained Optimization Methods

*This note covers several optimization techniques including conjugate-direction methods, Nelder-Mead simplex, and filter methods for constrained problems. It highlights the Maratos effect, where good steps are rejected by merit functions, and discusses remedies like second-order corrections and nonmonotone strategies. These methods are essential for when gradients are unavailable or constraints must be managed efficiently.*

This note explores various optimization algorithms used when derivatives are unavailable or when constraints are present.

## Concept

### Derivative-Free Optimization (DFO)
Several methods are discussed that do not require explicit gradient information. 

[[Conjugate-Direction Method]] is a technique for minimizing quadratic functions by performing one-dimensional minimizations along a set of conjugate directions. It uses the [[Parallel Subspace Property]] to generate new directions. 

[[Nelder-Mead Method]] is a simplex-based method that maintains a set of $n+1$ points in $\mathbb{R}^n$ whose convex hull forms a simplex. It iteratively updates the simplex through reflection, expansion, contraction, or shrinkage to find a minimum. 

[[Implicit Filtering]] uses a finite difference estimate of the gradient, where the noise level is systematically decreased to maintain accuracy. 

### Constrained Optimization

When constraints are present, managing the trade-off between objective function reduction and constraint satisfaction is critical. 

[[Filter Method]] uses a list of pairs $(f(x), h(x))$ representing the objective and constraint violation. An iterate is acceptable if its pair is not dominated by any other pair in the filter. 

[[Merit Function]] approaches use a single scalar function that combines objective and constraint terms. However, these might suffer from the [[Maratos Effect]], where a step that would lead to superlinear convergence is rejected because it increases both the objective and the constraint violation. 

To combat the Maratos effect, two main strategies are used:
1. [[Second-Order Correction]] involves adding a term to the step that accounts for the second-order information of the constraints, effectively reducing constraint violation.
2. [[Nonmonotone Strategy]] (such as the [[Watchdog Strategy]]) allows the merit function to increase temporarily on certain iterations to avoid rejecting good steps.

## Relationships
- [[Conjugate-Direction Method]] is a type of [[Derivative-Free Optimization]]
- [[Nelder-Mead Method]] is a type of [[Derivative-Free Optimization]]
- [[Implicit Filtering]] is a type of [[Derivative-Free Optimization]]
- [[Maratos Effect]] affects [[Filter Method]] and [[Merit Function]]
- [[Second-for-Order Correction]] remedies [[Maratos Effect]]
- [[Nonmonotone Strategy]] remedies [[Maratos Effect]]
