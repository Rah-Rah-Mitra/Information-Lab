---
type: content
title: "Mathematical Programming Problem Formulations"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:12:04.684309500+00:00
summary: "This note explores various real-world applications of mathematical programming, including oil blending, mining, and economic planning. It demonstrates how complex industrial and logistical problems can be translated into objective functions, variables, and constraints. The text highlights the methods used to solve these models, such as integer programming and network flow algorithms."
tags:
  - operations-research
  - mathematical-programming
  - optimization-models
entities:
  - "[[Mathematical Programming]]"
  - "[[Integer Programming]]"
  - "[[Linear Programming]]"
  - "[[Objective Function]]"
  - "[[Constraint]]"
  - "[[Totally Unimodular Matrix]]"
  - "[[Branch and Bound]]"
  - "[[Goal Programming]]"
  - "[[Leontief Model]]"
  - "[[Quadratic Assignment Problem]]"
relationships:
  - source: "Mathematical Programming"
    target: "Integer Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Linear Programming"
    description: "utilizes"
  - source: "Mathematical Programming"
    target: "Objective Function"
    description: "requires"
  - source: "Mathematical Programming"
    target: "Constraint"
    description: "requires"
  - source: "Integer Programming"
    target: "Branch and Bound"
    description: "solved via"
  - source: "Linear Programming"
    target: "Totally Unimodular Matrix"
    description: "guarantees integer solutions via"
  - source: "Goal Programming"
    target: "Objective Function"
    description: "uses specialized"
  - source: "Leontief Model"
    target: "Linear Programming"
    description: "is a type of"
  - source: "Quadratic Assignment Problem"
    target: "Integer Programming"
    description: "is often linearized into"
  - source: "Integer Programming"
    target: "Branch and Bound"
    description: "described as a strategy for"
---

# Mathematical Programming Problem Formulations

*This note explores various real-world applications of mathematical programming, including oil blending, mining, and economic planning. It demonstrates how complex industrial and logistical problems can be translated into objective functions, variables, and constraints. The text highlights the methods used to solve these models, such as integer programming and network flow algorithms.*

This note provides an overview of various case studies in [[Mathematical Programming]] used to model complex industrial and logistical systems.

## Concept
[[Mathematical Programming]] is the process of defining an objective function to be maximized or minimized, subject to certain constraints. The text provides several distinct examples of how this is applied in different domains:

### 1. Industrial Blending and Production
In the oil industry, blending problems involve determining the optimal quantities of different components to meet specific quality requirements (e.g., octane number) and demand. These are modeled using [[Linear Programming]] or [[Integer Programming]] if discrete decisions are involved.

### 2. Mining and Combinatorial Problems
Problems such as mining operations or the arrangement of balls in a cube involve combinatorial complexity. These are often addressed using [[Integer Programming]] and the [[Branch and Bound]] method to navigate the search tree efficiently.

### 3. Economic and Resource Planning
Models like the [[Leontief Model]] for economic planning or farm planning models involve managing resources over multiple time periods. These models often require [[Integer Programming]] to represent discrete decisions, such as whether to work a mine or build a depot.

### 4. Optimization of Constraints
Techniques like the method by Bradley et al. (1974) are used to simplify or optimize a single 0–1 [[Constraint]] that is too large. This involves identifying 'roofs' and 'ceilings' to find tighter, more equivalent constraints.

### 5. Network Flow and Distribution
Logistical problems, such as distribution from factories to depots to customers, can be viewed as minimum cost flow problems. If the model possesses the property of a [[Totally Unimodular Matrix]], a continuous [[Linear Programming]] solution will automatically be integer, which is significantly more efficient than using [[Integer Programming]].

## Relationships
- [[Mathematical Programming]] includes [[Integer Programming]]
- [[Mathematical Programming]] utilizes [[Linear Programming]]
- [[Mathematical Programming]] requires [[Objective Function]]
- [[Mathematical Programming]] requires [[Constraint]]
- [[Integer Programming]] is solved via [[Branch and Bound]]
- [[Linear Programming]] guarantees integer solutions via [[Totally Unimodular Matrix]]
- [[Goal Programming]] uses specialized [[Objective Function]]
- [[Leontief Model]] is a type of [[Linear Programming]]
- [[Quadratic Assignment Problem]] is often linearized into [[Integer Programming]]
- [[Integer Programming]] is described as a strategy for
