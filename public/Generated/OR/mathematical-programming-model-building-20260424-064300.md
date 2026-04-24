---
type: content
title: "Mathematical Programming Model Building"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:43:00.290748900+00:00
summary: "Mathematical programming model building involves selecting appropriate objective functions and constraint types to represent real-world problems accurately. It covers techniques for handling multiple objectives, non-linear objectives, and various constraint structures like hard, soft, and chance constraints. Effective modeling is a key driver of efficiency and both solution quality and solution interpretation."
tags:
  - operations-research
  - mathematical-programming
  - linear-programming
  - model-building
entities:
  - "[[Mathematical Programming]]"
  - "[[Multiple Objectives]]"
  - "[[Goal Programming]]"
  - "[[Minimax Objectives]]"
  - "[[Ratio Objectives]]"
  - "[[Hard Constraints]]"
  - "[[Hard Constraints]]"
  - "[[Soft Constraints]]"
  - "[[Chance Constraints]]"
  - "[[Generalized Upper Bounds]]"
  - "[[Shadow Price]]"
  - "[[Linear Programming]]"
relationships:
  - source: "Mathematical Programming"
    target: "Multiple Objectives"
    description: "addresses"
  - source: "Multiple Objectives"
    target: "Goal Programming"
    description: "can be handled via"
  - source: "Multiple Objectives"
    target: "Minimax Objectives"
    description: "can be handled via"
  - source: "Multiple Objectives"
    target: "Ratio Objectives"
    description: "can be handled via"
  - source: "Hard Constraints"
    target: "Mathematical Programming"
    description: "is a type of constraint in"
  - source: "Soft Constraints"
    target: "Mathematical Programming"
    description: "is a type of constraint in"
  - source: "Soft Constraints"
    target: "Goal Programming"
    description: "is used in"
  - source: "Soft Constraints"
    target: "Minimax Objectives"
    description: "is related to"
  - source: "Chance Constraints"
    target: "Mathematical Programming"
    description: "is a type of constraint in"
  - source: "Generalized Upper Bounds"
    target: "Mathematical Programming"
    description: "is a type of constraint in"
  - source: "Shadow Price"
    target: "Linear Programming"
    description: "is associated with"
  - source: "Linear Programming"
    target: "Multiple Objectives"
    description: "is used to solve"
  - source: "Ratio Objectives"
    target: "Data Envelopment Analysis"
    description: "arises in"
---

# Mathematical Programming Model Building

*Mathematical programming model building involves selecting appropriate objective functions and constraint types to represent real-world problems accurately. It covers techniques for handling multiple objectives, non-linear objectives, and various constraint structures like hard, soft, and chance constraints. Effective modeling is a key driver of efficiency and both solution quality and solution interpretation.*

This note explores the principles and techniques of building effective [[Mathematical Programming]] models, particularly within the context of [[Linear Programming]].

## Concept
Model building is the process of translating a real-world problem into a mathematical structure. This involves defining an objective function (what to optimize) and constraints (the rules or limits). A single model might require handling [[Multiple Objectives]], which can be approached by treating all but one as a constraint, using a weighted linear combination, or employing [[Goal Programming]].

### Objective Types
- **Minimax Objectives**: These aim to minimize the maximum value of a set of expressions, often used in equity or bottleneck problems. It can be formulated as a standard LP by introducing an variable $z$ such that $a_i x ≤ z$ for all $i$.
- **Ratio Objectives**: These involve maximizing or minimizing a ratio of variables, such as in [[Data Envelopment Analysis]] (DEA). While non-linear, they can often be transformed into linear programs using the substitution $w_j = x_j / t$ and $b \sum w_j = 1$.
- **Non-optimizable Objectives**: Sometimes the goal is simply to find a feasible solution that satisfies complex constraints, rather than optimizing a specific value.

### Constraint Types
Constraints define the feasible region. They can be categorized as follows:
- **Hard Constraints**: These are absolute limits that cannot be violated (e.g., technological limits).
- **Soft Constraints**: These can be violated at a certain cost, often modeled by adding slack or surplus variables to the objective function. This approach is often used in [[Goal Programming]].
- **Chance Constraints**: These specify that a constraint must be satisfied with a given probability $\beta$, written as $P(a_j x ≤ b) ≥ \beta$.
- **Generalized Upper Bounds (GUBs)**: A set of variables whose sum is bounded by a constant $b$, written as $\sum x_j ≤ b$. These are computationally efficient in the simplex algorithm.

### Model Quality and Efficiency
Effective model building aims for ease of understanding, error detection, and computational efficiency. Techniques like [[Modal Formulations]] include reducing the number of constraints by representing activities at extreme modes of operation using variables $\lambda_i$. 

Redundancy in constraints can be identified through [[Shadow Price]] analysis; a constraint with a zero shadow price is non-binding and may be considered redundant, though it may be worth keeping for future data changes.

## Relationships
- [[Mathematical Programming]] addresses [[Multiple Objectives]]
- [[Multiple Objectives]] can be handled via [[Goal Programming]]
- [[Multiple Objectives]] can be handled via [[Minimax Objectives]]
- [[Multiple Objectives]] can be handled via [[Ratio Objectives]]
- [[Hard Constraints]] is a type of constraint in [[Mathematical Programming]]
- [[Soft Constraints]] is a type of constraint in [[Mathematical Programming]]
- [[Soft Constraints]] is used in [[Goal Programming]]
- [[Soft Constraints]] is related to [[Minimax Objectives]]
- [[Chance Constraints]] is a type of constraint in [[Mathematical Programming]]
- [[Generalized Upper Bounds]] is a type of constraint in [[Mathematical Programming]]
- [[Shadow Price]] is associated with [[Linear Programming]]
- [[Linear Programming]] is used to solve [[Multiple Objectives]]
- [[Ratio Objectives]] arises in [[Data Envelopment Analysis]]
