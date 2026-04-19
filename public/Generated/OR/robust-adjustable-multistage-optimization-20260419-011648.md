---
type: content
title: "Robust Adjustable Multistage Optimization"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:16:48.989243300+00:00
summary: "This note explores the tractability and design of robust adjustable counterparts (RCs) for uncertain linear optimization problems. It details how nonlinear decision rules, such as quadratic rules, can be processed via semidefinite programming (SDP) and purified-output-based (POB) representations in control theory."
tags:
  - operations-research
  - robust-optimization
  - control-theory
  - semidefinite-programming
  - multistage-optimization
entities:
  - "[[Robust Adjustable Counterpart]]"
  - "[[Decision Rules]]"
  - "[[Semidefinite Programming]]"
  - "[[Affinely Adjustable Robust Counterpart]]"
  - "[[Decision Rules]]"
  - "[[Purified-Output-Based Representation]]"
  - "[[Linear Optimization]]"
  - "[[Quadratic Decision Rules]]"
  - "[[Ellipsoidal Uncertainty Set]]"
  - "[[Fixed Recourse]]"
relationships:
  - source: "Robust Adjustable Counterpart"
    target: "Decision Rules"
    description: "utilizes"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Decision Rules"
    description: "uses"
  - source: "Decision Rules"
    target: "Semidefinite Programming"
    description: "reformulated via"
  - source: "Quadratic Decision Rules"
    target: "Semidefinite Programming"
    description: "can be represented by"
  - source: "Purified-Output-Based Representation"
    target: "Decision Rules"
    description: "enables tractable design of"
  - source: "Ellipsoidal Uncertainty Set"
    target: "Robust Adjustable Counterpart"
    description: "ensures tractability of"
  - source: "Fixed Recourse"
    target: "Robust Adjustable Counterpart"
    description: "simplifies"
  - source: "Quadratic Decision Rules"
    target: "Ellipsoidal Uncertainty Set"
    description: "applied to"
---

# Robust Adjustable Multistage Optimization

*This note explores the tractability and design of robust adjustable counterparts (RCs) for uncertain linear optimization problems. It details how nonlinear decision rules, such as quadratic rules, can be processed via semidefinite programming (SDP) and purified-output-based (POB) representations in control theory.*

This note covers the methodology for handling uncertainty in multistage decision-making processes through robust optimization techniques.

## Concept
In multistage optimization, the [[Robust Adjustable Counterpart]] (ARC) seeks to find decisions that remain feasible for all realizations of uncertainty within a set $Z$. A central challenge is the tractability of the [[Decision Rules]] used to parameterize these decisions. While [[Affinely Adjustable Robust Counterpart]] (AARC) is often tractable, more complex rules like [[Quadratic Decision Rules]] can provide better performance at the cost of higher complexity.

### Decision Rule Types
1. **Affine Rules**: Decisions are affine functions of the data. These are generally tractable for [[Linear Optimization]] problems with [[Fixed Recourse]].
2. **Quadratic Rules**: Decisions are are quadratic functions of the data. The text demonstrates that for an [[Ellipsoidal Uncertainty Set]], the associated ARC can be represented via [[Semidefinite Programming]] (SDP).

### Control Theory Application
In the context of linear control, the [[Purified-Output-Based Representation]] (POB) allows for the synthesis of robust controllers. By re-parameterizing affine control laws, the state-control trajectory becomes bi-affine in the controller parameters, making the problem computationally tractable via SDP reformulations.

## Relationships
- [[Robust Adjustable Counterpart]] utilizes [[Decision Rules]]
- [[Affinely Adjustable Robust Counterpart]] uses [[Decision Rules]]
- [[Decision Rules]] can be reformulated via [[Semidefinite Programming]]
- [[Quadratic Decision Rules]] can be represented by [[Semidefinite Programming]]
- [[Purified-Output-Based Representation]] enables tractable design of [[Decision Rules]]
- [[Ellipsoidal Uncertainty Set]] ensures tractability of [[Robust Adjustable Counterpart]]
- [[Fixed Recourse]] simplifies [[Robust Adjustable Counterpart]]
- [[Quadratic Decision Rules]] are applied to [[Ellipsoidal Uncertainty Set]]
