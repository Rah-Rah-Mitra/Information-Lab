---
type: content
title: "Integer Programming Model Building"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:52:13.686614700+00:00
summary: "Integer programming involves using discrete variables to represent indivisible quantities, decisions, or logical conditions. It provides a framework for modeling complex non-linearities and logical relationships through 0-1 variables and special ordered sets. This approach allows for the transformation of complex combinatorial problems into structured mathematical programming models."
tags:
  - operations-research
  - integer-programming
  - mathematical-programming
  - combinatorial-optimization
entities:
  - "[[Integer Programming]]"
  - "[[Linear Programming Relaxation]]"
  - "[[Branch and Bound]]"
  - "[[Cutting Planes]]"
  - "[[Indicator Variables]]"
  - "[[Zero-One Variables]]"
  - "[[Special Ordered Sets]]"
  - "[[Boolean Algebra]]"
  - "[[Logical Conditions]]"
  - "[[Logical Connectives]]"
relationships:
  - source: "Integer Programming"
    target: "Linear Programming Relaxation"
    description: "is solved via"
  - source: "Integer Programming"
    target: "Branch and Bound"
    description: "uses"
  - source: "Integer_Programming"
    target: "Cutting Planes"
    description: "uses"
  - source: "Integer Programming"
    target: "Indicator Variables"
    description: "utilizes"
  - source: "Integer Programming"
    target: "Zero-One Variables"
    description: "utilizes"
  - source: "Integer Programming"
    target: "Special Ordered Sets"
    description: "utilizes"
  - source: "Integer Programming"
    target: "Boolean Algebra"
    description: "uses"
  - source: "Integer Programming"
    target: "Logical Conditions"
    description: "models"
  - source: "Indicator Variables"
    target: "Zero-One Variables"
    description: "is a type of"
  - source: "Zero-One Variables"
    target: "Boolean Algebra"
    description: "is modeled using"
  - source: "Special Ordered Sets"
    target: "Integer Programming"
    description: "is a specialized set of variables in"
  - source: "Logical Conditions"
    target: "Zero-One Variables"
    description: "are represented by"
---

# Integer Programming Model Building

*Integer programming involves using discrete variables to represent indivisible quantities, decisions, or logical conditions. It provides a framework for modeling complex non-linearities and logical relationships through 0-1 variables and special ordered sets. This approach allows for the transformation of complex combinatorial problems into structured mathematical programming models.*

This note explores the methodologies and structural components of building [[Integer Programming]] models, particularly focusing on how discrete variables are used to represent real-world constraints and decisions.

## Concept
[[Integer Programming]] (IP) is an extension of [[Linear Programming]] that imposes integrality requirements on a subset of variables. Unlike continuous variables, [[Integer Programming]] models often rely on [[Zero-One Variables]] (or binary variables) to represent decisions, indivisible quantities, or logical states. 

### Types of Discrete Variables
1. **Indivisible Quantities**: Representing items like cars or people.
2. **Decision Variables**: Using [[Zero-One Variables]] to indicate whether a decision is made (e.g., $\delta = 1$ if a depot is built).
3. **Indicator Variables**: These are used to [[Logical Conditions]] connect continuous variables to discrete states. For example, to force a variable $x$ to be zero when an indicator $\delta = 0$, one can use the constraint:

$$ x \le M\delta $$

where $M$ is a sufficiently large constant (the 'Big M' method). This constraint ensures that if $\delta = 0$, then $x = 0$. 

### Solving Integer Programs
Solving [[Integer Programming]] is computationally more difficult than solving [[Linear Programming]]. Common approaches include:
- **[[Branch and Bound]]**: The most successful general method for solving practical MIP problems. It involves solving an [[LP Relaxation]] (the continuous optimum) and performing a tree search.
- **[[Cutting Planes]]**: A method that adds extra constraints to the [[LP Relaxation]] to systematically narrow the feasible region toward integer solutions.
- **[[Branch and Cut]]**: A powerful combination of both methods.

### Logical Modeling with Boolean Algebra
Modellers can use [[Boolean Algebra]] to represent complex [[Logical Conditions]] using [[Zero-One Variables]]. For example, the proposition 'If $A$ or $B$ is manufactured, then $C$ must be manufactured' can be expressed through linear constraints. This process involves converting logical connectives (AND, OR, NOT, IMPLIES, IFF) to linear inequalities. 

### Special Ordered Sets (SOS)
[[Special Ordered Sets]] are specialized variable structures that provide computational advantages. 
- **[[SOS1]]**: A set of where exactly one variable must be non-zero.
- **[[SOS2]]**: A set of where at most two adjacent variables can be non-zero, which is useful for modeling piecewise linear approximations of non-linear functions.

## Relationships
- [[Integer Programming]] is solved via [[Linear Programming Relaxation]], [[Branch and Bound]], and [[Cutting Planes]].
- [[Indicator Variables]] are a type of of [[Zero-One Variables]].
- [[Zero-One Variables]] are modeled using [[Boolean Algebra]] to represent [[Logical Conditions]].
- [[Special Ordered Sets]] is a specialized set of variables in [[Integer Programming]].

