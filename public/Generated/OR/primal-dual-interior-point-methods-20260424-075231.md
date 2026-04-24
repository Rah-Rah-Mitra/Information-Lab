---
type: content
title: "Primal-Dual Interior-Point Methods"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:52:31.639215+00:00
summary: "Primal-dual interior-point methods solve linear programming problems by following a central path toward an optimal solution. These methods use Newton-like steps to balance reducing the duality gap and maintaining centrality. Practical implementations often use predictor-corrector enhancements to accelerate convergence."
tags:
  - operations-research
  - linear-programming
  - optimization
  - interior-point-methods
entities:
  - "[[Primal-Dual Interior-Point Methods]]"
  - "[[Central Path]]"
  - "[[Duality Measure]]"
  - "[[Predictor-Corrector Algorithm]]"
  - "[[Long-Step Path-Following]]"
  - "[[KKT Conditions]]"
  - "[[Normal-Equations Form]]"
  - "[[Mehrotra's Predictor-Corrector]]"
  - "[[Linear Programming]]"
  - "[[Newton Step]]"
relationships:
  - source: "Primal-Dual Interior-Point Methods"
    target: "Central Path"
    description: "follows"
  - source: "Primal-Dual Interior-Point Methods"
    target: "Duality Measure"
    description: "reduces"
  - source: "Primal-Dual Interior-Point Methods"
    target: "KKT Conditions"
    description: "satisfies"
  - source: "Predictor-Corrector Algorithm"
    target: "Primal-Dual Interior-Point Methods"
    description: "is a practical enhancement of"
  - source: "Long-Step Path-Following"
    target: "Primal-Dual Interior-Point Methods"
    description: "is a type of"
  - source: "Mehrotra's Predictor-Corrector"
    target: "Predictor-Corrector Algorithm"
    description: "is a specific instance of"
  - source: "Newton Step"
    target: "Primal-Dual Interior-Point Methods"
    description: "is the basis of"
  - source: "Normal-Equations Form"
    target: "Primal-Dual Interior-Point Methods"
    description: "is a computationally efficient reformulation of the step equations for"
  - source: "Linear Programming"
    target: "Primal-Dual Interior-Point Methods"
    description: "is the primary application of"
---

# Primal-Dual Interior-Point Methods

*Primal-dual interior-point methods solve linear programming problems by following a central path toward an optimal solution. These methods use Newton-like steps to balance reducing the duality gap and maintaining centrality. Practical implementations often use predictor-corrector enhancements to accelerate convergence.*

[[Primal-Dual Interior-Point Methods]] are a class of algorithms used to solve [[Linear Programming]] problems by iteratively approaching the optimal solution while staying within a neighborhood of the [[Central Path]].

## Concept
In [[Primal-Dual Interior-Point Methods]], the goal is to achieve a small [[Duality Measure]] $\mu$, defined as:

$$ \mu = \frac{x^T s}{n} $$

This measure represents the average complementarity gap. As $\mu$ approaches zero, the iterates $(x, \lambda, s)$ approach a primal-dual optimal solution that satisfies the [[KKT Conditions]].

Path-following algorithms restrict iterates to a neighborhood of the central path to ensure that nontrivial steps can be taken. Two common neighborhoods are defined:

1. The $\mathcal{N}_2(\theta)$ neighborhood: 
   $$ \mathcal{N}_2(\theta) = \left\{ (x, \lambda, s) \in \{x, \lambda, s > 0\} \mid \| X S e \| \text{ is close to } \mu e \right\} $$
2. The $\mathcal{N}_{\gamma}$ neighborhood: 
   $$ \mathcal{N}_{\gamma} = \left\{ (x, \lambda, s) \in \text{feasible region} \text{ s.t. } x_i s_i \text{ is close to } \mu \right\} $$

## Algorithmic Variants

### Long-Step Path-Following
[[Long-Step Path-Following]] algorithms use a wide neighborhood $\mathcal{N}_{\gamma}$ to allow for larger steps. They rely on a centering parameter $\sigma$ to balance the duality gap reduction and centrality. The complexity of these methods is typically $O(n \log(1/\epsilon))$ iterations.

### Predictor-Corrector Methods
Practical implementations, such as [[Mehrotra's Predictor-Corrector]], often use a [[Newton Step]] to compute an affine-scaling (predictor) step, followed by a corrector step to compensate for linearization error. This approach can lead to superlinear convergence.

## Computational Structure
To solve the linear systems required for each iteration, the algorithm often reformulates the step equations into the [[Normal-Equations Form]]:

$$ A^T D A \Delta \lambda = A^T D (r_c - A x) $$

This formulation is highly efficient for sparse matrices, as it is often solved using direct sparse Cholesky factorization. The matrix $A^T D A$ is the core computational bottleneck.

## Relationships
- [[Primal-Dual Interior-Point Methods]] follows [[Central Path]]
- [[Primal-Dual Interior-Point Methods]] reduces [[Duality Measure]]
- [[Primal-Dual Interior-Point Methods]] satisfies [[KKT Conditions]]
- [[Predictor-Corrector Algorithm]] is a practical enhancement of [[Primal-Dual Interior-Point Methods]]
- [[Long-Step Path-Following]] is a type of [[Primal-Dual Interior-Point Methods]]
- [[Mehrotra's Predictor-Corrector]] is a specific instance of [[Predictor-Corrector Algorithm]]
- [[Newton Step]] is the basis of [[Primal-Dual Interior-Point Methods]]
- [[Normal-Equations Form]] is a computationally efficient reformulation of the step equations for [[Primal-Dual Interior-Point Methods]]
- [[Linear Programming]] is the primary application of [[Primal-Dual Interior-Point Methods]]
