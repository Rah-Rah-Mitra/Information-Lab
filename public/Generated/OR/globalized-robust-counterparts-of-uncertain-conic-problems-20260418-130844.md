---
type: content
title: "Globalized Robust Counterparts of Uncertain Conic Problems"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:08:44.604272800+00:00
summary: "Explores the definition, tractability, and safe approximations of Globalized Robust Counterparts (GRCs) for uncertain conic problems, including applications to dynamical systems."
tags:
  - gis
  - robust-optimization
  - conic-programming
  - linear-matrix-inequalities
  - convex-optimization
entities:
  - "[[Globalized Robust Counterpart]]"
  - "[[Uncertain Conic Problem]]"
  - "[[Safe Tractable Approximation]]"
  - "[[Recessive Cone]]"
  - "[[Tightness Factor]]"
  - "[[Nesterov's Theorem]]"
  - "[[Talagrand Inequality]]"
  - "[[Robust Nonexpansiveness]]"
  - "[[Linear Matrix Inequality]]"
  - "[[Support Vector Machine]]"
  - "[[Hinge Loss Function]]"
  - "[[Minkowski Function]]"
relationships:
  - source: "Globalized Robust Counterpart"
    target: "Uncertain Conic Problem"
    description: "is a robust extension of"
  - source: "Safe Tractable Approximation"
    target: "Globalized Robust Counterpart"
    description: "provides a computationally feasible version of"
  - source: "Tightness Factor"
    target: "Safe Tractable Approximation"
    description: "quantifies the accuracy of"
  - source: "Recessive Cone"
    target: "Globalized Robust Counterpart"
    description: "is used to decompose the feasibility of"
  - source: "Nesterov's Theorem"
    target: "Safe Tractable Approximation"
    description: "provides an efficiently computable upper bound for"
  - source: "Talagrand Inequality"
    target: "Globalized Robust Counterpart"
    description: "is used to prove theorems related to"
  - source: "Robust Nonexpansiveness"
    target: "Linear Matrix Inequality"
    description: "is certified by a system of"
  - source: "Support Vector Machine"
    target: "Hinge Loss Function"
    description: "minimizes a regularized version of"
  - source: "Minkowski Function"
    target: "Globalized Robust Counterpart"
    description: "characterizes the uncertainty set in"
  - source: "Linear Matrix Inequality"
    target: "Safe Tractable Approximation"
    description: "often forms the basis of"
---

# Globalized Robust Counterparts of Uncertain Conic Problems

*Explores the definition, tractability, and safe approximations of Globalized Robust Counterparts (GRCs) for uncertain conic problems, including applications to dynamical systems.*

A [[Globalized Robust Counterpart]] (GRC) is a multi-dimensional extension of robust feasibility that accounts for global sensitivity to perturbations in [[Uncertain Conic Problem]]s.

## Definition and Structure

For an uncertain convex constraint, a solution is robust feasible with global sensitivity $\alpha$ if the distance from the perturbed constraint to the target set $\mathbf{Q}$ does not exceed $\alpha$ times the distance of the perturbation from its normal range. This allows for a more flexible treatment of uncertainty than standard robust counterparts.

## Tractability and Approximations

Because GRCs can be computationally intractable, the focus shifts to finding a [[Safe Tractable Approximation]]. Such an approximation is "safe" if its feasible set is contained within the feasible set of the GRC, and "tractable" if it can be represented by efficiently computable constraints, such as [[Linear Matrix Inequality]] (LMI) systems.

### Tightness and Bounds

The accuracy of these approximations is measured by a [[Tightness Factor]] $\vartheta$. [[Nesterov's Theorem]] is frequently employed to provide tight, efficiently computable upper bounds on intractable quadratic forms over unit boxes, facilitating the transition from intractable problems to tractable approximations.

## Decomposition and the Recessive Cone

Proposition 11.3.3 establishes that feasibility for a GRC is equivalent to a system of semi-infinite constraints. A key component of this analysis is the [[Recessive Cone]], which consists of all rays emanating from a point in a closed convex set that remain within the set. The distance to the recessive cone is used to characterize the behavior of the GRC under large perturbations.

## Applications

### Robust Nonexpansiveness

In the analysis of linear dynamical systems, [[Robust Nonexpansiveness]] is used to ensure that the $L_2$ norm of the output does not exceed the $L_2$ norm of the input. This property is certified by the existence of a solution to a specific system of LMIs. GRC-based analysis often provides more optimistic (less conservative) results than standard robust counterpart analysis.

### Robust Classification

In machine learning, [[Support Vector Machine]] (SVM) formulations can be viewed through the lens of robust optimization. The goal is often to minimize the worst-case [[Hinge Loss Function]] under various uncertainty models (spherical, box, or coupled). The use of a [[Minkowski Function]] allows for a generalized description of the uncertainty sets affecting the data matrix.

## Relationships

- [[Globalized Robust Counterpart]] is a robust extension of [[Uncertain Conic Problem]].
- [[Safe Tractable Approximation]] provides a computationally feasible version of [[Globalized Robust Counterpart]].
- [[Tightness Factor]] quantifies the accuracy of [[Safe Tractable Approximation]].
- [[Recessive Cone]] is used to decompose the feasibility of [[Globalized Robust Counterpart]].
- [[Nesterov's Theorem]] provides an efficiently computable upper bound for [[Safe Tractable Approximation]].
- [[Talagrand Inequality]] is used to prove theorems related to [[Globalized Robust Counterpart]].
- [[Robust Nonexpansiveness]] is certified by a system of [[Linear Matrix Inequality]].
- [[Support Vector Machine]] minimizes a regularized version of [[Hinge Loss Function]].
- [[Minkowski Function]] characterizes the uncertainty set in [[Globalized Robust Counterpart]].
- [[Linear Matrix Inequality]] often forms the basis of [[Safe Tractable Approximation]].
