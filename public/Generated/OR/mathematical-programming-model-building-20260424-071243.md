---
type: content
title: "Mathematical Programming Model Building"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:12:43.388750400+00:00
summary: "This note covers various practical applications and solution techniques for mathematical programming, including refinery optimization, mining, and efficiency analysis. It demonstrates how different objectives and objectives with recourse, with recourse, and minimax-style objectives can lead to different operational decisions. The text also explores the sensitivity analysis of Sensitivity Analysis and Sensitivity Analysis through shadow prices and ranges of optimality. Sensitivity Analysis is used to evaluate the how much a the cost or capacity of a system is allowed to change without altering thethoughtfulness of the optimal solution. "
tags:
  - operations-research
  - mathematical-programming
  - optimization-models
  - sensitivity-analysis
entities:
  - "[[Mathematical Programming]]"
  - "[[Optimization]]"
  - "[[Sensitivity Analysis]]"
  - "[[Mixed Integer Programming]]"
  - "[[Stochastic Programming]]"
  - "[[Shadow Prices]]"
  - "[[Minimax Objective]]"
  - "[[Efficiency Analysis]]"
relationships:
  - source: "Mathematical Programming"
    target: "Optimization"
    description: "is a form of"
  - source: "Mathematical Programming"
    target: "Mixed Integer Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Stochastic Programming"
    description: "includes"
  - source: "Mixed Integer Programming"
    target: "Shadow Prices"
    description: "provides"
  - source: "Optimization"
    target: "Sensitivity Analysis"
    description: "utilizes"
  - source: "Sensitivity Analysis"
    target: "Shadow Prices"
    description: "uses"
  - source: "Efficiency Analysis"
    target: "Mathematical Programming"
    description: "applies"
---

# Mathematical Programming Model Building

*This note covers various practical applications and solution techniques for mathematical programming, including refinery optimization, mining, and efficiency analysis. It demonstrates how different objectives and objectives with recourse, with recourse, and minimax-style objectives can lead to different operational decisions. The text also explores the sensitivity analysis of Sensitivity Analysis and Sensitivity Analysis through shadow prices and ranges of optimality. Sensitivity Analysis is used to evaluate the how much a the cost or capacity of a system is allowed to change without altering thethoughtfulness of the optimal solution.*

This note explores the practical application of [[Mathematical Programming]] across diverse industrial sectors, demonstrating how model formulation and objective selection drive decision-making.

## Concept
[[Mathematical Programming]] involves constructing mathematical models to optimize specific objectives, such as maximizing profit or minimizing cost, subject to constraints. The text provides several case studies:

- **Refinery Optimization**: Using [[Mixed Integer Programming]] to determine optimal production levels and variable values.
- **Mining and Farm Planning**: Multi-period models that manage resources over time, often involving continuity constraints.
- **Economic Planning**: Demonstrating how different objectives (e.g., maximizing capacity vs. maximizing manpower) lead to divergent optimal strategies.
- **Curve Fitting**: Comparing different regression techniques, such as minimizing the sum of absolute deviations versus minimizing the maximum absolute deviation (a [[Minimax Objective]]).
- **Efficiency Analysis**: Using [[Efficiency Analysis]] to evaluate the performance of different entities (e.g., garages) by comparing their input-output ratios. This often involves using dual values as weightings to maximize the ratio of outputs to inputs.
- **Yield Management**: Applying [[Stochastic Programming]] with recourse to manage uncertainty in demand, such as in airline seat pricing.

### Sensitivity Analysis
[[Sensitivity Analysis]] is used to evaluate how changes in parameters (like costs or capacities) affect the optimal solution. A key tool in this process is [[Shadow Prices]], which represent the marginal value of a constraint. For example, in a power generation model, the shadow price on a demand constraint indicates the marginal cost of production per hour.

$$ \text{Shadow Price} = \frac{\text{Change in Objective Value}}{\text{Change in Right-Hand Side}} $$

This relationship allows planners to understand the range within which a parameter can vary without changing the fundamental structure of the optimal solution.

## Relationships
- [[Mathematical Programming]] is a form of [[Optimization]]
- [[Mathematical Programming]] includes [[Mixed Integer Programming]] and [[Stochastic Programming]]
- [[Mixed Integer Programming]] provides [[Shadow Prices]]
- [[Optimization]] utilizes [[Sensitivity Analysis]]
- [[Sensitivity Analysis]] uses [[Shadow Prices]]
- [[Efficiency Analysis]] applies [[Mathematical Programming]]
