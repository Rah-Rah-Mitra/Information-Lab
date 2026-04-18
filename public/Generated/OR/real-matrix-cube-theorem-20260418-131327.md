---
type: content
title: "Real Matrix Cube Theorem"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:13:27.421238200+00:00
summary: "A theorem providing tractable approximations for uncertain linear matrix inequalities with structured norm-bounded uncertainty in the real case."
tags:
  - linear-algebra
  - optimization
  - robust-optimization
  - matrix-theory
entities:
  - "[[Real Matrix Cube Theorem]]"
  - "[[Linear Matrix Inequality]]"
  - "[[Structured Norm-Bounded Uncertainty]]"
  - "[[Robust Counterpart]]"
  - "[[S-Lemma]]"
  - "[[LMI]]"
  - "[[Tractable Approximation]]"
relationships:
  - source: "Real Matrix Cube Theorem"
    target: "Linear Matrix Inequality"
    description: "provides tractable approximations for"
  - source: "Real Matrix Cube Theorem"
    target: "Structured Norm-Bounded Uncertainty"
    description: "addresses"
  - source: "Real Matrix Cube Theorem"
    target: "Tractable Approximation"
    description: "establishes the existence of"
  - source: "Real Matrix Cube Theorem"
    target: "Robust Counterpart"
    description: "analyzes the gap between the original and its"
  - source: "Linear Matrix Inequality"
    target: "LMI"
    description: "is abbreviated as"
---

# Real Matrix Cube Theorem

*A theorem providing tractable approximations for uncertain linear matrix inequalities with structured norm-bounded uncertainty in the real case.*

The [[Real Matrix Cube Theorem]] is a fundamental result in robust optimization used to determine whether a [[Linear Matrix Inequality]] (LMI) remains valid under [[Structured Norm-Bounded Uncertainty]]. It provides a [[Tractable Approximation]] of the [[Robust Counterpart]] of an uncertain LMI, allowing the problem to be solved using semidefinite programming.

## Core Problem
The theorem addresses a parametric family of "matrix boxes" defined by perturbations $\rho$ and structured blocks $\Theta$. The goal is to check whether the predicate $A(\rho)$—the condition that the matrix remains positive semidefinite for all allowable perturbations—is valid.

## Tractability and the Gap
One of the primary contributions of the [[Real Matrix Cube Theorem]] is the identification of a stronger predicate $B(\rho)$ that is computationally tractable. The validity of $B(\rho)$ is equivalent to the solvability of a system of LMIs in matrix variables $Y$ and real variables $\lambda$.

There exists a "gap" between the original predicate $A(\rho)$ and the tractable predicate $B(\rho)$. This gap can be bounded solely in terms of the maximal rank of the scalar perturbations. Specifically, a universal function $\vartheta_R$ is used to bound the relationship: if $B(\rho)$ is not valid, then $A(\vartheta_R \rho)$ is not valid.

## Relationship to Other Theorems
The proof of the real case mirrors the complex case counterpart, with evident simplifications. It relies on tools such as the [[S-Lemma]] to handle quadratic constraints and ensure the existence of a solution within the specified uncertainty sets.

## Relationships
- [[Real Matrix Cube Theorem]] provides tractable approximations for [[Linear Matrix Inequality]].
- [[Real Matrix Cube Theorem]] addresses [[Structured Norm-Bounded Uncertainty]].
- [[Real Matrix Cube Theorem]] establishes the existence of [[Tractable Approximation]].
- [[Real Matrix Cube Theorem]] analyzes the gap between the original and its [[Robust Counterpart]].
