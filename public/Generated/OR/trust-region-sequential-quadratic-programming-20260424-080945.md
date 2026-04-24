---
type: content
title: "Trust-Region Sequential Quadratic Programming"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:09:45.353414700+00:00
summary: "Trust-region SQP methods incorporate a trust-region constraint to manage step size and ensure global convergence, even when the Hessian is indefinite. They address potential inconsistencies between linearized constraints and the trust region by using relaxation, penalty, or filter methods. This approach provides more robust convergence properties compared to standard SQP."
tags:
  - operations-research
  - optimization
  - sequential-quadratic-programming
  - trust-region-methods
entities:
  - "[[Trust-Region SQP Methods]]"
  - "[[Sequential Quadratic Programming]]"
  - "[[Hessian Matrix]]"
  - "[[Maratos Effect]]"
  - "[[Relaxation Method]]"
  - "[[Relaxation Vector]]"
  - "[[Penalty Method]]"
  - "[[Filter Method]]"
  - "[[Sequential Linear-Quadratic Programming]]"
  - "[[Convergence Rate]]"
relationships:
  - source: "Trust-Region SQP Methods"
    target: "Sequential Quadratic Programming"
    description: "is a variant of"
  - source: "Trust-Region SQP Methods"
    target: "Hessian Matrix"
    description: "manages"
  - source: "Trust-Region SQP Methods"
    target: "Maratos Effect"
    description: "mitigates"
  - source: "Trust-Region SQP Methods"
    target: "Relaxation Method"
    description: "uses"
  - source: "Trust-Region SQP Methods"
    target: "Penalty Method"
    description: "uses"
  - source: "Trust-Region SQP Methods"
    target: "Filter Method"
    description: "uses"
  - source: "Trust-Region SQP Methods"
    target: "Sequential Linear-Quadratic Programming"
    description: "is compared to"
  - source: "Trust-Region SQP Methods"
    target: "Convergence Rate"
    description: "improves"
---

# Trust-Region Sequential Quadratic Programming

*Trust-region SQP methods incorporate a trust-region constraint to manage step size and ensure global convergence, even when the Hessian is indefinite. They address potential inconsistencies between linearized constraints and the trust region by using relaxation, penalty, or filter methods. This approach provides more robust convergence properties compared to standard SQP.*

[[Trust-Region SQP Methods]] are specialized versions of [[Sequential Quadratic Programming]] designed to handle the complexities of nonlinear programming, particularly when the Hessian matrix is indefinite or when constraints are inconsistent. Unlike standard SQP, which may struggle with global convergence or singularity, trust-region approaches provide a mechanism for enforcing global convergence and controlling step quality.

## Concept
In a trust-region SQP framework, the subproblem involves minimizing a quadratic model of the objective function subject to linearized constraints and a trust-region constraint. The model is typically formulated as:

$$ \min_{p} \frac{1}{2} p^T \nabla^2_{xx} f f^T p + \nabla f^T p \quad \text{subject to} \quad c(x) + A p = 0, \text{ and } \|p\| \le \Delta_k \n $$

This constraint, $||p|| \le \Delta_k$, defines the region where the model is trusted to accurately reflect the true function behavior. A significant challenge arises when the linearized constraints and the trust-region constraint are incompatible, meaning no step $p$ exists within the trust region that satisfies the constraints. To resolve this, several strategies are employed:

1. **[[Relaxation Method]]s**: These methods use a [[Relaxation Vector]] to adjust the linearized constraints, aiming for feasibility rather than exact satisfaction at every step.
2. **[[Penalty Method]]s**: These methods incorporate constraint violations into the objective function via a penalty parameter $\mu$.
3. **[[Filter Method]]s**: These methods use a merit function to accept or reject steps based on a balance between feasibility and objective reduction.

Additionally, [[Trust-Region SQP Methods]] can be used to mitigate the [[Maratos Effect]], where a step that improves the objective and feasibility might be rejected because it increases the merit function. A second-order correction can be added to prevent this.

[[Sequential Linear-Quadratic Programming]] (SLQP) is an alternative that attempts to reduce computational cost by solving a linear program (LP) first to identify a working set, followed by an equality-constrained quadratic programming (EQP) phase. This is more scalable for large-scale problems than general SQP.

## Relationships
- [[Trust-Region SQP Methods]] is a variant of [[Sequential Quadratic Programming]]
- [[Trust-Region SQP Methods]] manages [[Hessian Matrix]]
- [[Trust-Region SQP Methods]] mitigates [[Maratos Effect]]
- [[Trust-Region SQP Methods]] uses [[Relaxation Method]]
- [[Trust-Region SQP Methods]] uses [[Penalty Method]]
- [[Trust-Region SQP Methods]] uses [[Filter Method]]
- [[Trust-Region SQP Methods]] is compared to [[Sequential Linear-Quadratic Programming]]
- [[Trust-Region SQP Methods]] improves [[Convergence Rate]]
