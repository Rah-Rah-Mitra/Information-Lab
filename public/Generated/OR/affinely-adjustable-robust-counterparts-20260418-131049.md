---
type: content
title: "Affinely Adjustable Robust Counterparts"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:10:49.435645500+00:00
summary: "Exploration of Affinely Adjustable Robust Counterparts (AARC) for uncertain optimization, focusing on tractability, decision rules, and control applications."
tags:
  - gis
  - optimization
  - robust-optimization
  - control-theory
  - linear-programming
entities:
  - "[[Affinely Adjustable Robust Counterpart]]"
  - "[[Adjustable Robust Counterpart]]"
  - "[[Robust Counterpart]]"
  - "[[Fixed Recourse]]"
  - "[[Affine Decision Rules]]"
  - "[[Quadratic Decision Rules]]"
  - "[[Purified-Output-Based Representation]]"
  - "[[Semidefinite Program]]"
  - "[[S-Lemma]]"
  - "[[Approximate S-Lemma]]"
  - "[[Linear Optimization]]"
  - "[[Uncertainty Set]]"
relationships:
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Adjustable Robust Counterpart"
    description: "is a specific form of"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Robust Counterpart"
    description: "generalizes"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Affine Decision Rules"
    description: "utilizes"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Fixed Recourse"
    description: "is computationally tractable under"
  - source: "Affinely Adjustable Robust Counterpart"
    target: "Linear Optimization"
    description: "is applied to"
  - source: "Quadratic Decision Rules"
    target: "Affinely Adjustable Robust Counterpart"
    description: "can be linearized into"
  - source: "Purified-Output-Based Representation"
    target: "Affine Decision Rules"
    description: "re-parameterizes"
  - source: "Semidefinite Program"
    target: "Affinely Adjustable Robust Counterpart"
    description: "provides a tractable reformulation for"
  - source: "S-Lemma"
    target: "Semidefinite Program"
    description: "is used to derive"
  - source: "Approximate S-Lemma"
    target: "Affinely Adjustable Robust Counterpart"
    description: "provides tight approximations for"
  - source: "Uncertainty Set"
    target: "Affinely Adjustable Robust Counterpart"
    description: "defines the range of"
---

# Affinely Adjustable Robust Counterparts

*Exploration of Affinely Adjustable Robust Counterparts (AARC) for uncertain optimization, focusing on tractability, decision rules, and control applications.*

The [[Affinely Adjustable Robust Counterpart]] (AARC) is a method for solving uncertain optimization problems where decision variables are allowed to depend on the uncertain data via affine functions, known as [[Affine Decision Rules]].

## Tractability and Fixed Recourse

For an uncertain [[Linear Optimization]] problem, the AARC is computationally tractable provided the problem has [[Fixed Recourse]]—meaning the uncertainty only affects the constant terms in the constraints. When fixed recourse is present, the AARC can often be reformulated as a [[Semidefinite Program]] or a standard linear program, depending on the structure of the [[Uncertainty Set]].

## Decision Rule Extensions

While affine rules are common, [[Quadratic Decision Rules]] can be employed to reduce conservatism. These can be handled by augmenting the data vector with pairwise products of the original entries, effectively transforming the quadratic rule into an affine rule over an extended data space. However, this transformation may result in an [[Uncertainty Set]] that is computationally intractable unless specific structures (like ellipsoidal sets) are used.

## Control Theory Applications

In the synthesis of linear controllers, the AARC framework is applied via the [[Purified-Output-Based Representation]]. This approach re-parameterizes affine control laws so that the state-control trajectory becomes bi-affine in the controller parameters and the uncertain data. This transformation ensures that the robust design of finite-horizon controllers remains a tractable [[Linear Optimization]] problem with fixed recourse.

## Theoretical Foundations

The tractability of these counterparts often relies on the [[S-Lemma]] for ellipsoidal uncertainty or the [[Approximate S-Lemma]] for intersections of ellipsoids, which allow for the derivation of tight safe tractable approximations of the robust constraints.

## Relationships
- [[Affinely Adjustable Robust Counterpart]] is a specific form of [[Adjustable Robust Counterpart]].
- [[Affinely Adjustable Robust Counterpart]] generalizes [[Robust Counterpart]].
- [[Affinely Adjustable Robust Counterpart]] utilizes [[Affine Decision Rules]].
- [[Affinely Adjustable Robust Counterpart]] is computationally tractable under [[Fixed Recourse]].
- [[Affinely Adjustable Robust Counterpart]] is applied to [[Linear Optimization]].
- [[Quadratic Decision Rules]] can be linearized into [[Affinely Adjustable Robust Counterpart]].
- [[Purified-Output-Based Representation]] re-parameterizes [[Affine Decision Rules]].
- [[Semidefinite Program]] provides a tractable reformulation for [[Affinely Adjustable Robust Counterpart]].
- [[S-Lemma]] is used to derive [[Semidefinite Program]].
- [[Approximate S-Lemma]] provides tight approximations for [[Affinely Adjustable Robust Counterpart]].
- [[Uncertainty Set]] defines the range of [[Affinely Adjustable Robust Counterpart]].
