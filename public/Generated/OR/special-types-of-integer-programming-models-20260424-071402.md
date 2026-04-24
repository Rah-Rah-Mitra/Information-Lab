---
type: content
title: "Special Types of Integer Programming Models"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:14:02.399563700+00:00
summary: "This note covers several fundamental classes of integer programming (IP) models, including set covering, set packing, set partitioning, knapsack, and travelling salesman problems. These models are critical for combinatorial optimization and combinatorial structure in practical applications like aircrew scheduling and vehicle routing. Understanding their structures allows for model builders to exploit specific algorithmic properties and constraint simplification techniques."
tags:
  - operations-research
  - integer-programming
  - combinatorial-optimization
  - mathematical-programming
entities:
  - "[[Integer Programming]]"
  - "[[Set Covering Problem]]"
  - "[[Set Packing Problem]]"
  - "[[Set Partitioning Problem]]"
  - "[[Knapsack Problem]]"
  - "[[Travelling Salesman Problem]]"
  - "[[Disjunctive Constraint]]"
  - "[[Convex Hull]]"
  - "[[Facet]]"
  - "[[Linear Programming Relaxation]]"
relationships:
  - source: "Integer Programming"
    target: "Set Covering Problem"
    description: "includes"
  - source: "Integer Programming"
    target: "Set Packing Problem"
    description: "includes"
  - source: "Integer Programming"
    target: "Set Partitioning Problem"
    description: "includes"
  - source: "Integer Programming"
    target: "Knapsack Problem"
    description: "includes"
  - source: "Integer Programming"
    target: "Travelling Salesman Problem"
    description: "includes"
  - source: "Set Covering Problem"
    target: "Linear Programming Relaxation"
    description: "relates to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is dual to"
  - source: "Set Partitioning Problem"
    target: "Set Packing Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "target"
    description: "is dual to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
  - source: "Set Packing Problem"
    target: "Set Partitioning Problem"
    description: "is equivalent to"
---

# Special Types of Integer Programming Models

*This note covers several fundamental classes of integer programming (IP) models, including set covering, set packing, set partitioning, knapsack, and travelling salesman problems. These models are critical for combinatorial optimization and combinatorial structure in practical applications like aircrew scheduling and vehicle routing. Understanding their structures allows for model builders to exploit specific algorithmic properties and constraint simplification techniques.*

This note explores several specialized classes of [[Integer Programming]] models that arise frequently in combinatorial optimization.

## Concept

In [[Integer Programming]], many practical problems can be categorized into specific structural forms. These forms often allow for the use of specialized algorithms or constraint simplification techniques.

### Set Covering Problem

A [[Set Covering Problem]] is a minimization problem where the goal is to cover all members of a set $S$ using a collection of subsets at minimum cost. It is characterized by the following properties:
- Minimization objective.
- All constraints are '$\\ge$' type.
- All RHS coefficients are 1.
- All matrix coefficients are 0 or 1.

$$ \text{Minimize } \sum \delta_i \quad \text{subject to } \sum_{i \text{ in cover}} \text{member } j \text{ is covered} \ge 1 $$

This models scenarios like aircrew scheduling, where 'legs' must be covered by 'rosters'.

### Set Packing Problem

A [[Set Packing Problem]] is a maximization problem where the goal is to 'pack' as many members of a set $S$ into a collection of subsets without any overlap. It is characterized by:
- Maximization objective.
- All constraints are '$\\le$' type.
- All RHS coefficients are 1.
- All matrix coefficients are 0 or 1.

$$ \text{Maximize } \sum \delta_i \quad \text{subject to } \sum_{i \text{ in pack}} \text{member } j \text{ is included} \textle 1 $$

An interesting property is that the [[Set Packing Problem]] is the dual of the [[Set Covering Problem]] in terms of its [[Linear Programming Relaxation]].

### Set Partitioning Problem

A [[Set Partitioning Problem]] textually combines covering and packing, requiring that each member of $S$ be in exactly one subset. This is modeled using equality constraints:

$$ \text{subject to } \sum_{i \text{ in partition}} \text{member } j \text{ is included} = 1 $$

[[Set Partitioning Problem]] is equivalent to [[Set Packing Problem]] via the introduction of slack variables, and is often used in aircrew scheduling and political districting.

### Knapsack Problem

A [[Knapsack Problem]] is an [[Integer Programming]] model with a single constraint. It can be take the form:

$$ \text{Maximize } \sum p_{\gamma} \gamma \quad \text{subject to } \sum a_{\gamma} \gamma \le b $$

This models the hiker's knapsack, where items have values $p_{\gamma}$ and weights $a_{\gamma}$ within a capacity $b$. It is often used in column generation techniques.

### Travelling Salesman Problem

A [[Travelling Salesman Problem]] is a minimization problem to find the shortest tour through a set of cities. A common IP formulation involves 0-1 variables $\delta_{ij}$ representing whether the edge $(i, j)$ is in the tour:

$$ \text{Minimize } \sum c_{ij} \text{ where } \text{tour is valid} $$

Constraints ensure thats that exactly one city is visited after city $i$ and exactly one city is visited before city $j$.

## Relationships
- [[Integer Programming]] includes [[Set Covering Problem]], [[Set Packing Problem]], [[Set Partitioning Problem]], [[Knapsack Problem]], and [[Travelling Salesman Problem]].
- [[Set Packing Problem]] is dual to [[Set Covering Problem]].
- [[Set Partitioning Problem]] is equivalent to [[Set Packing Problem]].
- [[Set Packing Problem]] and [[Set Partitioning Problem]] is dual to [[Set Covering Problem]].
