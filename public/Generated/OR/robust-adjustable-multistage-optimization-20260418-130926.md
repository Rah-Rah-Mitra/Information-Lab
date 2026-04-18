---
type: content
title: "Robust Adjustable Multistage Optimization"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:09:26.272090800+00:00
summary: "An exploration of Adjustable Robust Counterparts (ARC) and Affinely Adjustable Robust Counterparts (AARC) for multi-stage decision making under uncertainty."
tags:
  - optimization
  - robust-optimization
  - multistage-decision-making
  - convex-analysis
entities:
  - "[[Adjustable Robust Counterpart]]"
  - "[[Affinely Adjustable Robust Counterpart]]"
  - "[[Robust Counterpart]]"
  - "[[Fixed Recourse]]"
  - "[[Information Base]]"
  - "[[Decision Rule]]"
  - "[[Wait-and-See Decisions]]"
  - "[[Analysis Variables]]"
  - "[[Scenario-Generated Uncertainty Set]]"
  - "[[Constraint-Wise Uncertainty]]"
relationships:
  - source: "Adjustable Robust Counterpart"
    target: "Robust Counterpart"
    description: "is an extension of"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Adjustable Robust Counterpart"
    description: "is a restricted version of"
  - source: "Adjustable Robust Counterpart"
    target: "Decision Rule"
    description: "optimizes over"
  - source: "Decision Rule"
    target: "Information Base"
    description: "depends on"
  - source: "Adjustable Robust Counterpart"
    target: "Wait-and-See Decisions"
    description: "models"
  - source: "Adjustable Robust Counterpart"
    target: "Analysis Variables"
    description: "allows adjustability for"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Fixed Recourse"
    description: "is computationally tractable under"
  - source: "Adjustable Robust Counterpart"
    target: "Scenario-Generated Uncertainty Set"
    description: "is tractable with"
  - source: "Adjustable Robust Counterpart"
    target: "Constraint-Wise Uncertainty"
    description: "is equivalent to Robust Counterpart under"
---

# Robust Adjustable Multistage Optimization

*An exploration of Adjustable Robust Counterparts (ARC) and Affinely Adjustable Robust Counterparts (AARC) for multi-stage decision making under uncertainty.*

The [[Adjustable Robust Counterpart]] (ARC) is a framework for multi-stage decision making where some decision variables are allowed to adjust based on a portion of the revealed uncertain data, rather than being fixed "here and now."

## Motivation for Adjustability

In standard [[Robust Counterpart]] (RC) models, all decisions must be made before the uncertainty is revealed. However, in many practical scenarios, this is too conservative. Adjustability arises from two primary sources:

- **[[Analysis Variables]]**: Slack variables used for mathematical conversion (e.g., in Linear Optimization) that do not represent actual decisions and can thus depend on the data.
- **[[Wait-and-See Decisions]]**: Decisions made sequentially over time, where later actions can be informed by data revealed in earlier stages.

## The ARC Framework

In an ARC, decision variables are replaced by [[Decision Rule]]s, which are functions $X_j(P_j \zeta)$ where $P_j$ is the [[Information Base]] (the specific portion of the data $\zeta$ available to the decision maker). The goal is to find decision rules that ensure feasibility for all $\zeta$ in the uncertainty set while minimizing the objective.

## Computational Challenges and AARC

General ARCs are typically infinite-dimensional and computationally intractable because they optimize over functions. To address this, the [[Affinely Adjustable Robust Counterpart]] (AARC) restricts decision rules to be affine functions of the data.

### Fixed Recourse

A critical condition for the tractability of the AARC is [[Fixed Recourse]]. A problem has fixed recourse if the coefficients of every adjustable variable in the objective and constraints are certain (independent of the uncertainty $\zeta$). Under fixed recourse, the AARC remains a semi-infinite conic problem, making it as tractable as the standard RC.

## Special Tractable Cases

Certain structures allow the ARC to be solved efficiently without complex approximations:

- **[[Scenario-Generated Uncertainty Set]]**: When the uncertainty set is the convex hull of finite scenarios and the problem has fixed recourse, the ARC can be reduced to a tractable conic problem.
- **[[Constraint-Wise Uncertainty]]**: If uncertainty is split into independent blocks per constraint and the decision space is compact, the ARC is equivalent to the non-adjustable RC.

## Relationships

- [[Adjustable Robust Counterpart]] is an extension of [[Robust Counterpart]].
- [[Affinely Adjustable Robust Counterpart]] is a restricted version of [[Adjustable Robust Counterpart]].
- [[Adjustable Robust Counterpart]] optimizes over [[Decision Rule]]s.
- [[Decision Rule]] depends on [[Information Base]].
- [[Adjustable Robust Counterpart]] models [[Wait-and-See Decisions]].
- [[Adjustable Robust Counterpart]] allows adjustability for [[Analysis Variables]].
- [[Affinely Adjustable Robust Counterpart]] is computationally tractable under [[Fixed Recourse]].
- [[Adjustable Robust Counterpart]] is tractable with [[Scenario-Generated Uncertainty Set]].
- [[Adjustable Robust Counterpart]] is equivalent to [[Robust Counterpart]] under [[Constraint-Wise Uncertainty]].
