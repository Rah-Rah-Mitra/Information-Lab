---
type: content
title: "Large-Scale Unconstrained Optimization Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:30:43.841036500+00:00
summary: "This note covers limited-memory quasi-Newton methods and techniques for handling partially separable functions in large-scale optimization. It details the L-BFGS algorithm, its compact representation, and how to structural properties of structural properties of the function can be used to exploit the efficient updating of element Hessians."
tags:
  - operations-research
  - optimization
  - quasi-newton
  - large-scale-optimization
entities:
  - "[[L-BFGS]]"
  - "[[BFGS Method]]"
  - "[[Lanczos Method]]"
  - "[[Nonlinear Conjugate Gradient]]"
  - "[[Partially Separable Functions]]"
  - "[[SR1]]"
  - "[[Compact Representation]]"
  - "[[Hessian Matrix]]"
  - "[[Wolfe Conditions]]"
  - "[[Trust-Region Methods]]"
relationships:
  - source: "L-BFGS"
    target: "BFGS Method"
    description: "is a limited-memory version of"
  - source: "L-BFGS"
    target: "Nonlinear Conjugate Gradient"
    description: "is related to"
  - source: "L-BFGS"
    target: "Compact Representation"
    description: "uses"
  - source: "L-BFGS"
    target: "Wolfe Conditions"
    description: "requires"
  - source: "L-BFGS"
    target: "Hessian Matrix"
    description: "approximates"
  - source: "L-BFGS"
    target: "Trust-Region Methods"
    description: "can be used in"
  - source: "L-BFGS"
    target: "Partially Separable Functions"
    description: "can be be improved by exploiting structure of"
  - source: "SR1"
    target: "Hessian Matrix"
    description: "approximates"
  - source: "Lanczos Method"
    target: "Hessian Matrix"
    description: "can be used to solve systems involving"
  - source: "Compact Representation"
    target: "L-BFGS"
    description: "enables efficient implementation of"
---

# Large-Scale Unconstrained Optimization Methods

*This note covers limited-memory quasi-Newton methods and techniques for handling partially separable functions in large-scale optimization. It details the L-BFGS algorithm, its compact representation, and how to structural properties of structural properties of the function can be used to exploit the efficient updating of element Hessians.*

This note explores various strategies for efficient large-scale unconstrained optimization, particularly focusing on quasi-Newton methods that avoid the full computation or storage of the Hessian matrix.

## Concept
In large-scale optimization, the [[Hessian Matrix]] is often too large to store or compute. [[L-BFGS]] (Limited-memory BFGS) is a memory-efficient approach that approximates the inverse Hessian using only the most recent curvature information from a few vector pairs \[(s_i, y_i)\] from the last \[$m$\] iterations. 

Instead of storing a full \[$n \times n$\] matrix, [[L-BFGS]] maintains a compact set of vector pairs. The product \[$H_k f$ \] is computed efficiently via the [[L-BFGS Two-Loop Recursion]] (Algorithm 7.4), which requires only \[$4mn$ \] multiplications. 

One effective way to implement this is through a [[Compact Representation]], which expresses the corrections to the basic matrix as an outer product of long, narrow matrices and a small \[$m \times m$\] matrix. This representation allows for efficient updates and matrix-vector products, making it [[L-BFGS]] a practical choice for large problems where the true Hessian is not sparse.

## Partially Separable Functions
Many large-scale problems involve [[Partially Separable Functions]], where the objective function can be decomposed into a sum of simpler element functions: 

$$ f(x) = \sum_{i=1}^{n_e} f_i(x) \\ $$ 

Each element function $f_i$ depends on only a few components of $x$. By maintaining quasi-Newton approximations to each element Hessian separately and then summing them, one can achieve much more efficient and accurate updates than standard [[L-BFGS]]. 

This technique is particularly useful when the function is partially separable, as it allows the algorithm to exploit the structure of the element Hessians, which are often small and dense, to build a complete approximation of the full Hessian. This approach is used in software packages like [[LANCELOT]].

## Relationships
- [[L-BFGS]] is a limited-memory version of the [[BFGS Method]].
- [[L-BFGS]] is related to [[Nonlinear Conjugate Gradient]] methods.
- [[L-BFGS]] uses [[Compact Representation]] for efficiency.
- [[L-BFGS]] can be used in [[Trust-Region Methods]].
- [[L-BFGS]] approximates the [[Hessian Matrix]].
- [[SR1]] is an alternative quasi-Newton update formula.
- [[Lanczos Method]] can be used to solve systems involving the Hessian.
- [[Partially Separable Functions]] can be be improved by exploiting structure of [[L-BFGS]].
