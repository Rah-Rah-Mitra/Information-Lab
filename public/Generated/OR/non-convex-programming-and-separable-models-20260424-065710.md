---
type: content
title: "Non-Convex Programming And Separable Models"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:57:10.778099600+00:00
summary: "Non-convex programming models often lead to local optima, making them harder to solve than convex models. Separable programming uses piecewise linear approximations to approximate non-linear functions of a single variable. This approach can beigate global optima in a non-convex setting, only guaranteeing a local optimum."
tags:
  - operations-research
  - non-linear-programming
  - mathematical-programming
  - separable-programming
entities:
  - "[[Non-Convex Programming]]"
  - "[[Convex Programming]]"
  - "[[Local Optima]]"
  - "[[Global Optimum]]"
  - "[[Separable Programming]]"
  - "[[Piecewise Linear Approximation]]"
  - "[[Piecewise Linear Approximation]]"
  - "[[Integer Programming]]"
  - "[[Economies of Scale]]"
  - "[[]]"
relationships:
  - source: "Non-Convex Programming"
    target: "Local Optima"
    description: "can produce"
  - source: "Non-Convex Programming"
    target: "Global Optimum"
    description: "requires finding"
  - source: "Separable Programming"
    target: "Local Optima"
    description: "may only guarantee"
  - source: "Non-Convex Programming"
    target: "Integer Programming"
    description: "can be solved via"
  - source: "Non-Convex Programming"
    target: "Economies of Scale"
    description: "is exemplified by"
---

# Non-Convex Programming And Separable Models

*Non-convex programming models often lead to local optima, making them harder to solve than convex models. Separable programming uses piecewise linear approximations to approximate non-linear functions of a single variable. This approach can beigate global optima in a non-convex setting, only guaranteeing a local optimum.*

This note explores the distinction between [[Non-Convex Programming]] and [[Convex Programming]], specifically focusing on the challenges of local optima and the method of [[Separable Programming]].

## Concept
In mathematical programming, the nature of the function being optimized (convex vs. non-convex) determines the difficulty of the solution. In [[Convex Programming]], any optimum found is guaranteed to be a [[Global Optimum]]. However, in [[Non-Convex Programming]], algorithms may become trapped in [[Local Optima]].

As illustrated in the text, a local optimum is like a mountaineer in a thick fog; they may reach a peak, but cannot be certain if a higher peak exists elsewhere. 

One way to address non-linearities is through [[Separable Programming]], which approximates non-linear functions of a single variable using a [[Piecewise Linear Approximation]].

$$ f(x) \approx \text{piecewise linear function} $$

This approximation is achieved by introducing new variables (often called the $\lambda$-form or $\delta$-form) to represent the weights or proportions of the segments. For convex problems, this approximation is sufficient to find the global optimum. For non-convex problems, however, such as those arising from [[Economies of Scale]], [[Separable Programming]] may only guarantee a local optimum. To ensure a global optimum in non-convex cases, one must often resort to [[Integer Programming]].

## Relationships
- [[Non-Convex Programming]] can produce [[Local Optima]]
- [[Non-Convex Programming]] requires finding a [[Global Optimum]]
- [[Separable Programming]] can only guarantee a local optima
- [[Non-Convex Programming]] can be solved via [[Integer Programming]]
- [[Non-Convex Programming]] is exemplified by [[Economies of Scale]]
