---
type: content
title: "Sensitivity Analysis and Non-linear Programming"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:51:30.993583400+00:00
summary: "This note explores the sensitivity of linear programming solutions to changes in parameters and introduces the fundamental distinction between convex and non-convex non-linear models. It explains how shadow prices, ranges, and marginal rates of substitution describe the impact of parameter changes. The transition to non-linear models is motivated by increasing or decreasing returns to scale and returns to demand elasticity."
tags:
  - operations-research
  - linear-programming
  - sensitivity-analysis
  - non-linear-programming
  - mathematical-programming
entities:
  - "[[Sensitivity Analysis]]"
  - "[[Shadow Price]]"
  - "[[Objective Range]]"
  - "[[Right-Hand Side Range]]"
  - "[[Marginal Rates of Substitution]]"
  - "[[Parametric Programming]]"
  - "[[Convex Programming]]"
  - "[[Non-linear Programming]]"
  - "[[Returns to Scale]]"
  - "[[Price Elasticity of Demand]]"
relationships:
  - source: "Sensitivity Analysis"
    target: "Shadow Price"
    description: "utilizes"
  - source: "Sensitivity Analysis"
    target: "Objective Range"
    description: "investigates"
  - source: "Sensitivity Analysis"
    target: "Right-Hand Side Range"
    description: "investigates"
  - source: "Sensitivity Analysis"
    target: "Marginal Rates of Substitution"
    description: "calculates"
  - source: "Sensitivity Analysis"
    target: "Parametric Programming"
    description: "extends via"
  - source: "Convex Programming"
    target: "Non-linear Programming"
    description: "is a subset of"
  - source: "Non-linear Programming"
    target: "Returns to Scale"
    description: "arises from"
  - source: "Non-linear Programming"
    target: "Price Elasticity of Demand"
    description: "is modeled by"
---

# Sensitivity Analysis and Non-linear Programming

*This note explores the sensitivity of linear programming solutions to changes in parameters and introduces the fundamental distinction between convex and non-convex non-linear models. It explains how shadow prices, ranges, and marginal rates of substitution describe the impact of parameter changes. The transition to non-linear models is motivated by increasing or decreasing returns to scale and returns to demand elasticity.*

This note covers the core concepts of sensitivity analysis in linear programming and the transition into non-linear mathematical programming.

## Concept

### Sensitivity Analysis
In [[Sensitivity Analysis]], we investigate how changes in the objective function coefficients or right-hand side (RHS) constraints affect the optimal solution. 

For RHS coefficients, the [[Shadow Price]] represents the rate of change in the objective value per unit change in the constraint's RHS. The effect of this change is valid within a specific [[Right-Hand Side Range]]. Within this permitted range, the [[Shadow Price]] remains constant, but the values of the variables in the optimal solution may change. The relative rates at which these variables change are described by the [[Marginal Rates of Substitution]].

If a constraint is binding and its RHS is changed within its range, the values of the variables change. If the constraint is non-binding, the values of the variables remain unchanged, and the shadow price is zero.

### Objective Ranges
[[Objective Range]] refers to the range of values for an objective coefficient within which the optimal solution (the decision variables' values) remains unchanged. A key distinction is that within an [[Objective Range]], the optimal objective value will change, but the decision variables themselves will not. This is a stronger result than the RHS ranging case.

### Parametric Programming
n[[Parametric Programming]] provides a method for investigating the effects of radical changes or multiple simultaneous changes in parameters, which often fall outside the standard ranges. It allows for the tracking of the optimal solution as a parameter 	heta 

$$ \theta $$ 

varies. This is particularly useful for expansion or contraction of resources.

### Non-linear Programming
[[Non-linear Programming]] occurs when the objective function or constraints are not linear. This often arises from [[Returns to Scale]] (e.g., increasing or decreasing returns to scale) or from [[Price Elasticity of Demand]]. Price elasticity can be modeled as:

$$ E_x = \frac{\text{percentage change in quantity of } x \text{ demanded}}{\text{percentage change in } p(x)} $$

This leads to a non-linear relationship between price and quantity. 

[[Convex Programming]] is a special class of non-linear programming where the objective function is being minimized and thes is a convex function over a convex feasible region. A function $f(x)$ is convex if the set of points where $y \ge f(x)$ forms a convex region. 

## Relationships
- [[Sensitivity Analysis]] utilizes [[Shadow Price]]
- [[Sensitivity Analysis]] investigates [[Objective Range]]
- [[Sensitivity Analysis]] investigates [[Right-Hand Side Range]]
- [[Sensitivity Analysis]] investigates [[Marginal Rates of Substitution]]
- [[Sensitivity Analysis]] extends via [[Parametric Programming]]
- [[Convex Programming]] is a subset of [[Non-linear Programming]]
- [[Non-linear Programming]] arises from [[Returns to Scale]]
- [[Non-linear Programming]] is modeled by [[Price Elasticity of Demand]]
