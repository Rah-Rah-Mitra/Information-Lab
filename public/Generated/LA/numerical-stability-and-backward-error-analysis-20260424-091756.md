---
type: content
title: "Numerical Stability and Backward Error Analysis"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:17:56.058984200+00:00
summary: "Numerical stability characterizes how rounding errors in floating point arithmetic affect the accuracy of computed results. This note distinguishes between stability and backward stability, explaining how backward stable algorithms provide the right answer to a nearly right question. It also introduces the concept of backward error analysis as a method for quantifying these effects."
tags:
  - numerical-linear-algebra
  - numerical-analysis
  - floating-point-arithmetic
  - stability-analysis
entities:
  - "[[Numerical Stability]]"
  - "[[Backward Stability]]"
  - "[[Backward Error Analysis]]"
  - "[[Condition Number]]"
  - "[[Floating Point Arithmetic]]"
  - "[[Householder Triangularization]]"
  - "[[QR Factorization]]"
  - "[[Relative Error]]"
relationships:
  - source: "Numerical Stability"
    target: "Floating Point Arithmetic"
    description: "is affected by"
  - source: "Backward Stability"
    target: "Numerical Stability"
    description: "is a stronger form of"
  - source: "Backward Error Analysis"
    target: "Backward Stability"
    description: "is used to analyze"
  - source: "Backward Stability"
    target: "Condition Number"
    description: "accuracy depends on"
  - source: "Householder Triangularization"
    target: "QR Factorization"
    description: "is a method for"
  - source: "Householder Triangularization"
    target: "Backward Stability"
    description: "is an example of"
---

# Numerical Stability and Backward Error Analysis

*Numerical stability characterizes how rounding errors in floating point arithmetic affect the accuracy of computed results. This note distinguishes between stability and backward stability, explaining how backward stable algorithms provide the right answer to a nearly right question. It also introduces the concept of backward error analysis as a method for quantifying these effects.*

This note explores the fundamental concepts of numerical stability and how algorithms are evaluated in the context of floating point arithmetic.

## Concept
In numerical analysis, a problem is defined as a function $f: X \to Y$. An algorithm is a map between the same spaces. Because digital computers are discrete and use [[Floating Point Arithmetic]], the computed result $\tilde{f}(x)$ is inevitably affected by rounding errors. [[Numerical Stability]] is the standard way of characterizing whether an algorithm provides a reliable result despite these errors.

### Accuracy and Error Measures
We distinguish between the absolute error and the [[Relative Error]], which is the standard measure in this context:

$$ \frac{\|\tilde{f}(x) - f(x)\|}{\|f(x)\|} = \mathcal{O}(\epsilon_{\text{machine}}) $$

An algorithm is considered accurate if its relative error is on the order of the machine epsilon $\epsilon_{\text{machine}}$.

### Stability vs. Backward Stability
While accuracy is a desirable goal, it is often impossible if the problem itself is ill-conditioned. This leads to two distinct definitions:

1. **[[Numerical Stability]]**: An algorithm is stable if it gives nearly the right answer to nearly the right question. Mathematically, this means the relative error is bounded by the [[Condition Number]] $\kappa(x)$ and the machine epsilon:

$$ \frac{\|\tilde{f}(x) - f(x)\|}{\|f(x)\|} = \\mathcal{O}(\epsilon_{\text{machine}} \cdot \kappa(x)) $$

2. **[[Backward Stability]]**: A stronger condition where the algorithm gives exactly the right answer to nearly the right question. This is defined as:

$$ \frac{\|\tilde{f}(x) - f(x)\|}{\|x\|\| = \mathcal{O}(\epsilon_{\text{machine}}) \text{ for some } \tilde{x} \text{ such that } f(\tilde{x}) = \tilde{f}(x) $$

In other words, a backward stable algorithm produces a result that is the exact solution to a slightly perturbed input.

### Backward Error Analysis
[[Backward Error Analysis]] is the process of investigating the condition of the problem and the stability of the algorithm to determine final accuracy. It is more effective than forward error analysis because it captures the global property of the condition number, which is often invisible at the level of individual operations.

### Example: Householder Triangularization
[[Householder Triangularization]] is a backward stable algorithm used to compute the [[QR Factorization]] of a matrix. Although the individual factors $Q$ and $R$ might have large forward errors due to an ill-conditioned problem, their product $QR$ remains highly accurate. This is demonstrated by the theorem:

$$ \tilde{Q}\tilde{R} = (A + \Delta A) \text{ where } \Delta A = \mathcal{O}(\epsilon_{\text{machine}}) \|A\| $$

This property ensures thats that the algorithm is suitable for solving systems of equations via QR factorization.

## Relationships
- [[Numerical Stability]] is affected by [[Floating Point Arithmetic]].
- [[Backward Stability]] is a stronger form of [[Numerical Stability]].
- [[Backward Error Analysis]] is used to analyze [[Backward Stability]].
- [[Backward Stability]] accuracy depends on [[Condition Number]].
- [[Householder Triangularization]] is a method for [[QR Factorization]].
- [[Householder Triangularization]] is an example of [[Backward Stability]].
