---
type: content
title: "Integer Programming Applications And Case Studies"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:12:59.154487+00:00
summary: "This note explores various real-world applications of integer programming and combinatorial optimization, ranging from car rental logistics, car repair capacity, and vehicle routing, and protein folding. Protein folding models use integer programming to match hydrophobic acids. matching hydrophobic acids with folds. It highlights the importance of capacity constraints and subtour elimination constraints in complex logistics and biological modeling. "
tags:
  - operations-research
  - integer-programming
  - combinatorial-optimization
  - case-studies
entities:
  - "[[Integer Programming]]"
  - "[[Combinatorial Optimization]]"
  - "[[Subtour Elimination Constraints]]"
  - "[[Car Rental Logistics]]"
  - "[[Vehicle Routing Problem]]"
  - "[[Protein Folding]]"
  - "[[Shadow Prices]]"
  - "[[Capacity Constraints]]"
relationships:
  - source: "Integer Programming"
    target: "Car Rental Logistics"
    description: "applied to"
  - source: "Integer Programming"
    target: "Vehicle Routing Problem"
    description: "solves"
  - source: "Integer Programming"
    target: "Protein Folding"
    description: "models"
  - source: "Subtour Elimination Constraints"
    target: "Vehicle Routing Problem"
    description: "prevents subtours in"
  - source: "Capacity Constraints"
    target: "Car Rental Logistics"
    description: "limits"
  - source: "Shadow Prices"
    target: "Capacity Constraints"
    description: "reflects value of"
---

# Integer Programming Applications And Case Studies

*This note explores various real-world applications of integer programming and combinatorial optimization, ranging from car rental logistics, car repair capacity, and vehicle routing, and protein folding. Protein folding models use integer programming to match hydrophobic acids. matching hydrophobic acids with folds. It highlights the importance of capacity constraints and subtour elimination constraints in complex logistics and biological modeling.*

This note summarizes several case studies demonstrating the application of [[Integer Programming]] and [[Combinatorial Optimization]] to complex logistical and biological systems.

## Concept
[[Integer Programming]] is a subfield of mathematical programming where some or all variables are restricted to integer values. It is used to solve problems involving discrete decisions, discrete objects, or counts, such as the number of cars in a depot, the number of vans, or the specific configuration of of a protein structure.

In the case of [[Car Rental Logistics]], [[Integer Programming]] [[Integer Programming]] is used to optimize the fleet size and repair capacity. The text describes a scenario where repair capacity is a limiting factor, which is reflected in high [[Shadow Prices]] on the constraints. Increasing repair capacity allows for an expansion of the fleet and increased profit.

In the [[Vehicle Routing Problem]], specifically in the context of [[Lost Baggage Distribution]], lost baggage distribution is a model of a vehicle routing problem. Solving the model without [[Subtour Elimination Constraints]] leads to unacceptable subtours. A total of 31 subtour elimination constraints were required to ensure a valid route.

In [[Protein Folding]], [[Integer Programming]] is used to match hydrophobic acids with folds. The model used 471 nodes to solve, matching eight hydrophobic acids with folds between specific acid indices.

## Relationships
- [[Integer Programming]] applied to [[Car Rental Logistics]]
- [[Integer Programming]] applied to domain of [[Combinatorial Optimization]]
- [[Integer Programming]] models [[Protein Folding]]
- [[Subtour Elimination Constraints]] prevents subtours in [[Vehicle Routing Problem]]
- [[Capacity Constraints]] limits [[Car Rental Logistics]]
- [[Shadow Prices]] reflects value of [[Capacity Constraints]]
