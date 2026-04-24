---
type: content
title: "Mathematical Programming Problem Formulations"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:08:30.968292800+00:00
summary: "This note explores various applications of mathematical programming, including logistics, biology, and manufacturing. It demonstrates how real-world constraints and objectives can be translated into formal mathematical models. The examples range from complexity levels from simple linear programming to complex integer programming."
tags:
  - operations-research
  - mathematical-programming
  - optimization-models
entities:
  - "[[Mathematical Programming]]"
  - "[[Linear Programming]]"
  - "[[Integer Programming]]"
  - "[[Data Envelopment Analysis]]"
  - "[[Branch and Bound]]"
  - "[[Revised Simplex Algorithm]]"
  - "[[Separable Programming]]"
relationships:
  - source: "Mathematical Programming"
    target: "Linear Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Integer Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Data Envelopment Analysis"
    description: "applies to"
  - source: "Integer Programming"
    target: "Branch and Bound"
    description: "solved by"
  - source: "Linear Programming"
    target: "Revised Simplex Algorithm"
    description: "solved by"
  - source: "Separable Programming"
    target: "Revised Simplex Algorithm"
    description: "solved by"
---

# Mathematical Programming Problem Formulations

*This note explores various applications of mathematical programming, including logistics, biology, and manufacturing. It demonstrates how real-world constraints and objectives can be translated into formal mathematical models. The examples range from complexity levels from simple linear programming to complex integer programming.*

This note provides an overview of diverse applications of [[Mathematical Programming]] used to model complex real-world systems.

## Concept
[[Mathematical Programming]] is the process of selecting the best possible solution from a set of available alternatives by optimizing an objective function subject to constraints. The text provides several case studies illustrating different mathematical structures:

- **Logistics and Distribution**: Problems involving minimizing costs in factory-to-depot-to-customer supply chains, or optimizing tanker lorry routes for milk collection.
- **Biological Modeling**: Using [[Integer Programming]] to model protein folding and protein comparison via contact maps. This involves finding the largest isomorphic subgraphs while preserving amino acid ordering.
- **Efficiency Analysis**: Using [[Data Envelopment Analysis]] to evaluate the efficiency of various service providers (e.g., garages) by comparing inputs (staff, space) against outputs (profit, sales).
- **Manufacturing and Blending**: Using [[Linear Programming]] to solve blending problems, such as determining optimal prices for dairy products or food manufacture.

## Algorithms
Common algorithms used to solve these models include:
- [[Revised Simplex Algorithm]] for [[Linear Programming]] problems.
- [[Branch and Bound]] for [[Integer Programming]] problems.
- The separable extension to the [[Revised Simplex Algorithm]] for [[Separable Programming]] problems.

## Relationships
- [[Mathematical Programming]] includes [[Linear Programming]], [[Integer Programming]], and [[Data Envelopment Analysis]].
- [[Integer Programming]] is solved by [[Branch and Bound]].
- [[Linear Programming]] is solved by [[Revised Simplex Algorithm]].
- [[Separable Programming]] is solved by [[Revised Simplex Algorithm]].
