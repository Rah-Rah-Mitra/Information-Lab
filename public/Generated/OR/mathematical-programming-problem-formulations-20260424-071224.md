---
type: content
title: "Mathematical Programming Problem Formulations"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:12:24.552417500+00:00
summary: "This note covers various practical applications of mathematical programming, including agricultural pricing, efficiency analysis, and logistics. It demonstrates how to translate real-world constraints and objectives into formal mathematical models, such as quadratic and stochastic programming. The examples illustrate the trade-offs between linear approximations and linear programming relaxations."
tags:
  - operations-research
  - mathematical-programming
  - optimization-models
entities:
  - "[[Mathematical Programming]]"
  - "[[Quadratic Programming]]"
  - "[[Stochastic Programming]]"
  - "[[Data Envelopment Analysis]]"
  - "[[Linear Programming Relaxation]]"
  - "[[Integer Programming]]"
  - "[[Traveling Salesman Problem]]"
  - "[[Vehicle Routing Problem]]"
  - "[[Convexity]]"
  - "[[Separable Programming]]"
relationships:
  - source: "Mathematical Programming"
    target: "Quadratic Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Stochastic Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Integer Programming"
    description: "includes"
  - source: "Mathematical Programming"
    target: "Data Envelopment Analysis"
    description: "includes"
  - source: "Quadratic Programming"
    target: "Convexity"
    description: "requires"
  - source: "Integer Programming"
    target: "Traveling Salesman Problem"
    description: "is used in"
  - source: "Traveling Salesman Problem"
    target: "Linear Programming Relaxation"
    description: "uses"
  - source: "Vehicle Routing Problem"
    target: "Traveling Salesman Problem"
    description: "is an extension of"
  - source: "Separable Programming"
    target: "Quadratic Programming"
    description: "approximates"
  - source: "Stochastic Programming"
    target: "Integer Programming"
    description: "uses"
---

# Mathematical Programming Problem Formulations

*This note covers various practical applications of mathematical programming, including agricultural pricing, efficiency analysis, and logistics. It demonstrates how to translate real-world constraints and objectives into formal mathematical models, such as quadratic and stochastic programming. The examples illustrate the trade-offs between linear approximations and linear programming relaxations.*

This note explores the practical application of [[Mathematical Programming]] to solve complex decision-making problems across diverse industries. Mathematical programming involves constructing objective functions and constraints that represent real-world limitations and goals.

## Concept

Mathematical programming is the broad field of optimization where one seeks to maximize or minimize a certain quantity subject to constraints. The text provides several specific types of models:

1. [[Quadratic Programming]]: These models involve quadratic terms in the objective function. For example, in the agricultural pricing problem, the objective function includes terms like $p_i p_j$, which necessitates specialized algorithms or transformations into a [[Separable Programming]] form to find a global optimum.

2. [[Stochastic Programming]]: This approach is used when parameters are uncertain. In the yield management problem, a three-period model is built to maximize expected yield by considering multiple scenarios and their probabilities. This is a form of recourse, where decisions in the first period are adjusted based on the outcomes of the observed scenarios in subsequent periods.

3. [[Data Envelopment Analysis]]: Used for efficiency analysis, this technique (often referred to as DEA) seeks to find a mixture of existing units (e.g., garages) to establish an efficiency frontier. An efficiency number $1/w$ is derived by maximizing a weight $w$ such that the combined inputs of a mixture do not exceed the inputs of the target unit, while outputs are maximized.

4. [[Integer Programming]]: Many problems, such as the milk collection (a variant of the TSP) and lost baggage distribution (a variant of the [[Vehicle Routing Problem]]), require discrete decisions, often represented by 0–1 variables. These models are often solved using a [[Linear Programming Relaxation]] to find an initial bound or to simplify the computation, though subtour elimination constraints must often be added to ensure a valid tour.

## Relationships

- [[Mathematical Programming]] includes [[Quadratic Programming]], [[Stochastic Programming]], [[Integer Programming]], and [[Data Envelopment Analysis]].
- [[Quadratic Programming]] can be approximated by [[Separable Programming]].
- [[Traveling Salesman Problem]] is a component of the [[Vehicle Routing Problem]].
- [[Integer Programming]] models often utilize [[Linear Programming Relaxation]] for computational tractability.
