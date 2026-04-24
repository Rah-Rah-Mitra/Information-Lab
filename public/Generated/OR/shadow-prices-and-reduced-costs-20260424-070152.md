---
type: content
title: "Shadow Prices and Reduced Costs"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:01:52.333869300+00:00
summary: "Shadow prices represent the marginal value of resources in a linear programming model, indicating how much the objective function improves with small changes to constraints. Reduced costs quantify the amount by which a product's profit contribution is insufficient to cover its imputed resource costs. Together, they provide critical economic insights into resource valuation and product viability."
tags:
  - operations-research
  - linear-programming
  - sensitivity-analysis
  - duality-theory
entities:
  - "[[Shadow Prices]]"
  - "[[Reduced Costs]]"
  - "[[Duality Theorem]]"
  - "[[Linear Programming]]"
  - "[[Primal Model]]"
  - "[[Dual Model]]"
  - "[[Right-hand Side Ranges]]"
  - "[[Opportunity Costs]]"
relationships:
  - source: "Shadow Prices"
    target: "Dual Model"
    description: "arise from"
  - source: "Shadow Prices"
    target: "Primal Model"
    description: "provide marginal value for"
  - source: "Shadow Prices"
    target: "Opportunity Costs"
    description: "represent"
  - source: "Reduced Costs"
    target: "Linear Programming"
    description: "quantify profit deficit in"
  - source: "Duality Theorem"
    target: "Shadow Prices"
    description: "underlies"
  - source: "Reduced Costs"
    target: "Shadow Prices"
    description: "relates to"
  - source: "Shadow Prices"
    target: "Right-hand Side Ranges"
    description: "valid within"
  - source: "Dual Model"
    target: "Primal Model"
    description: "is the dual of"
---

# Shadow Prices and Reduced Costs

*Shadow prices represent the marginal value of resources in a linear programming model, indicating how much the objective function improves with small changes to constraints. Reduced costs quantify the amount by which a product's profit contribution is insufficient to cover its imputed resource costs. Together, they provide critical economic insights into resource valuation and product viability.*

This note explores the economic interpretation of the optimal solutions to [[Linear Programming]] models through the lens of duality.

## Concept
In [[Linear Programming]], the relationship between the [[Primal Model]] and the [[Dual Model]] is governed by the [[Duality Theorem]]. This theorem ensures that the total value of the factory (the optimal objective value) is identical in both models. The dual variables, often referred to as [[Shadow Prices]], represent the marginal value of each resource constraint. For instance, if a constraint represents grinding capacity, the shadow price indicates the rate at which the total profit would increase per unit increase in that capacity.

$$ \text{Shadow Price} = \frac{\partial (\text{Objective Value})}{\partial (\text{RHS Coefficient})} $$

This interpretation is valid only within specific [[Right-hand Side Ranges]]. Outside these ranges, the marginal value of the resource may change unpredictably.

[[Shadow Prices]] are essentially a form of of [[Opportunity Costs]]. They represent the cost of not being able to use resources for their most profitable alternative uses. 

[[Reduced Costs]] provide a different but vital piece of information regarding product viability. For a a product that is not produced in the optimal solution (i.e., its variable is zero), the [[Reduced Costs]] represent the amount by which its unit profit contribution is insufficient to cover its imputed resource costs. If a product's profit is £125 less than its imputed costs, its reduced cost is £125; to make it worth producing, its unit price would need to increase by that amount.

## Relationships
- [[Shadow Prices]] arise from the [[Dual Model]]
- [[Shadow Prices]] provide marginal value for the [[Primal Model]]
- [[Shadow Prices]] represent [[Opportunity Costs]]
- [[Reduced Costs]] quantify profit deficit in [[Linear Programming]]
- [[Duality Theorem]] underlies [[Shadow Prices]]
- [[Reduced Costs]] relates to [[Shadow Prices]]
- [[Shadow Prices]] valid within [[Right-hand Side Ranges]]
- [[Dual Model]] is the dual of [[Primal Model]]
