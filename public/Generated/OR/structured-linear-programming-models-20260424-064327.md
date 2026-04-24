---
type: content
title: "Structured Linear Programming Models"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:43:27.616967600+00:00
summary: "Structured linear programming models exploit the inherent organization of large-scale mathematical programming problems to improve computational efficiency. These models often feature block angular or staircase structures, which can be solved using decomposition algorithms. Such structures are important for multiplant, multi-product, and multi-period models."
tags:
  - operations-research
  - linear-programming
  - mathematical-programming
  - structured-models
entities:
  - "[[Structured Linear Programming Models]]"
  - "[[Block Angular Structure]]"
  - "[[Staircase Structure]]"
  - "[[Stochastic Programming]]"
  - "[[Dantzig-Wolfe Decomposition]]"
  - "[[Decomposition by Pricing]]"
  - "[[Multiplant Models]]"
  - "[[Multi-period Models]]"
  - "[[Common Rows]]"
  - "[[Submodels]]"
relationships:
  - source: "Structured Linear Programming Models"
    target: "Block Angular Structure"
    description: "exhibit"
  - source: "Structured Linear Programming Models"
    target: "Staircase Structure"
    description: "exhibit"
  - source: "Structured Linear Programming Models"
    target: "Stochastic Programming"
    description: "is a special case of"
  - source: "Structured Linear Programming Models structured"
    target: "Block Angular Structure"
    description: "often features"
  - source: "Block Angular Structure"
    target: "Common Rows"
    description: "contains"
  - source: "Block Angular Structure"
    target: "Submodels"
    description: "comprises"
  - source: "Block Angular Structure"
    target: "Multiplant Models"
    description: "arises in"
  - source: "Block Angular Structure"
    target: "Multi-period Models"
    description: "arises in"
  - source: ": "
    target: "Staircase Structure"
    description: "arises in"
  - source: "Staircase Structure"
    target: "Multi-period Models"
    description: "arises in"
  - source: "Dantzig-Wolfe Decomposition"
    target: "Block Angular Structure"
    description: "solves"
  - source: "Decomposition by Pricing"
    target: "Block Angular Structure"
    description: "solves"
  - source: "Decomposition by Pricing"
    target: "Dantzig-Wolfe Decomposition"
    description: "is a form of"
  - source: "Stochastic Programming"
    target: "Structured Linear Programming Models"
    description: "is a special case of"
  - source: "Submodels"
    target: "Block Angular Structure"
    description: "are components of"
---

# Structured Linear Programming Models

*Structured linear programming models exploit the inherent organization of large-scale mathematical programming problems to improve computational efficiency. These models often feature block angular or staircase structures, which can be solved using decomposition algorithms. Such structures are important for multiplant, multi-product, and multi-period models.*

This note explores the organization and solution methods for large-scale mathematical programming models that exhibit specific mathematical structures. 

## Concept

Large-scale [[Structured Linear Programming Models]] are often not monolithic; instead, they are arise from the combining of smaller, more manageable subproblems. These structures allow for specialized computational techniques that can exploit the organization to solve massive problems that would otherwise be computationally intractable. 

### Block Angular Structure

A common structure is the [[Block Angular Structure]], which is characterized by a set of [[Submodels]] (represented by blocks of coefficients) that are linked together by [[Common Rows]] (constraints that apply to all subproblems, such as shared resources like raw materials). 

$$ \begin{pmatrix} A_0 & 0 & \dots & 0 \ A_1 & B_1 & \dots & 0 \ A_2 & 0 & \dots & B_2 \ \vdots & \vdots & \ddots & \vdots \ A_n & 0 & \dots & B_n \ \end{pmatrix} $$ 

The matrix above represents a general block angular structure where $A_0$ represents the common rows and $bing$ represents the submodel blocks. 

This structure is prevalent in [[Multiplant Models]], where different factories share a single resource, and in [[Multi-period Models]], where decisions made in one period are time-staged. In multi-period models, equality constraints link consecutive time periods, creating a staircase or block angular structure. 

### Stochastic Programming

[[Stochastic Programming]] is a special type of linear program that models uncertainty by representing data as a series of weighted probabilities rather than a single point estimate. It is can be viewed as a special case of robust optimization. 

### Decomposition Methods

To solve large structured models, one can use [[Decomposition]] techniques. [[Dantzig-Wolfe Decomposition]] is a prominent example of [[Decomposition by Pricing]], which uses 'internal prices' (dual variables) to value shared resources and incorporates them into the subproblems. This allows the subproblems to be optimized relative to the overall benefit of the total model. 

By using a convex linear combination of the vertices of the subproblems, the total problem can be reformulated into a new set of variables, a process known as giving a modal formulation to the subproblems. 

## Relationships

- [[Structured Linear Programming Models]] exhibit [[Block Angular Structure]] and [[Staircase Structure]].
- [[Block Angular Structure]] contains [[Common Rows]] and [[Submodels]].
- [[Block Angular Structure]] arises in [[Multiplant Models]] and [[Multi-period Models]].
- [[Staircase Structure]] arises in [[Multi-period Models]].
- [[Stochastic Programming]] is a special case of [[Structured Linear Programming Models]].
- [[Dantzig-Wolfe Decomposition]] solves [[Block Angular Structure]].
- [[Decomposition by Pricing]] is a form of [[Dantzig-Wolfe Decomposition]].
- [[Submodels]] are components of [[Block Angular Structure]]

