---
type: content
title: "Numerical Discretization Methods for Partial Differential Equations"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T08:29:58.633621500+00:00
summary: "This note covers the fundamental discretization techniques used to solve partial differential equations, including finite difference, finite element, and finite volume methods. It explains how continuous operators are discretized into algebraic systems, focusing on the stability, accuracy, and sparsity of the resulting matrices. The comparison between these methods highlights the trade-world trade-offs between order of accuracy and numerical stability."
tags:
  - numerical-analysis
  - partial-differential-equations
  - computational-fluid-dynamics
  - finite-element-method
  - finite-difference-method
  - finite-volume-method
entities:
  - "[[Finite Difference Method]]"
  - "[[Finite Element Method]]"
  - "[[Finite Volume Method]]"
  - "[[Partial Differential Equations]]"
  - "[[Laplacean Operator]]"
  - "[[Upwind Scheme]]"
  - "[[Green's Formula]]"
  - "[[Divergence Theorem]]"
  - "[[Control Volume]]"
  - "[[Element Stiffness Matrix]]"
  - "[[Assembly Process]]"
relationships:
  - source: "Finite Difference Method"
    target: "Laplacean Operator"
    description: "approximates"
  - source: "Finite Element Method"
    target: "Green's Formula"
    description: "utilizes"
  - source: "Finite Volume Method"
    target: "Control Volume"
    description: "is based on"
  - source: "Upwind Scheme"
    target: "Finite Difference Method"
    description: "improves stability of"
  - source: "Finite Element Method"
    target: "Laplacean Operator"
    description: "solves"
  - source: "Element Stiffness Matrix"
    target: "Finite Element Method"
    description: "is a component of"
  - source: "Finite Volume Method"
    target: "Laplacean Operator"
    description: "approximates"
  - source: "Green's Formula"
    target: "Divergence Theorem"
    description: "is a generalization of"
  - source: "Finite Element Method"
    target: "Assembly Process"
    description: "requires"
  - source: "Finite Volume Method"
    target: "Upwind Scheme"
    description: "can incorporate"
  - source: "Finite Volume Method"
    target: "Control Volume"
    description: "depends on"
  - source: "Finite Element Method"
    target: "Divergence Theorem"
    description: "uses"
  - source: "Finite Volume Method"
    target: "Upwind Scheme"
    description: "uses"
  - source: "Finite_Difference_Method"
    target: "Upwind_Scheme"
    description: "can be stabilized by"
  - source: "Finite_Element_Method"
    target: "Element_Stiffness_Matrix"
    description: "produces"
  - source: "Finite_Element_Method"
    target: "Finite_Volume_Method"
    description: "is compared to"
---

# Numerical Discretization Methods for Partial Differential Equations

*This note covers the fundamental discretization techniques used to solve partial differential equations, including finite difference, finite element, and finite volume methods. It explains how continuous operators are discretized into algebraic systems, focusing on the stability, accuracy, and sparsity of the resulting matrices. The comparison between these methods highlights the trade-world trade-offs between order of accuracy and numerical stability.*

Numerical discretization is the process of transforming continuous [[Partial Differential Equations]] into discrete algebraic systems. This note explores three primary methods: [[Finite Difference Method]], [[Finite Element Method]], and [[Finite Volume Method]].

## Concept

### Finite Difference Method

[[Finite Difference Method]] approximates derivatives using local point-based stencils. For a first derivative, a forward difference is given by: 

$$ \frac{\delta z}{\delta x} \approx \frac{z_{i+1} - z_i}{\Delta x} $$

This is a first-order accurate approximation. A centered difference for the second derivative is given by: 

$$ \frac{\delta^2 z}{\delta x^2} \approx \frac{z_{i+1} - 2z_i + z_{i-1}}{(\Delta x)^2} $$

This is a second-order accurate approximation. To avoid oscillations in convection-diffusion problems, an [[Upwind Scheme]] is used to replace the central difference with a one-sided difference based on the sign of the velocity. This improves stability by ensuring the matrix is a diagonally dominant M-matrix, though it reduces accuracy to first order.

### Finite Element Method

[[Finite Element Method]] is based on the weak formulation of a problem, where the integral of the product of a function and a test function is considered. This approach relies on [[Green's Formula]] and the [[Divergence Theorem]] to transform integrals. 

[[Green's Formula]] is expressed as: 

$$ \int_{\Omega} \nabla z \cdot \nabla y \, d\mathbf{x} = -\int_{\partial\Omega} z \frac{\partial y}{\partial \boldsymbol{\n}} \, d\boldsymbol{\s} + \int_{\Omega} \Delta z \cdot y \, d\boldsymbol{\x} $$

In the [[Finite Element Method]], the domain is divided into a subspace of functions that are piecewise linear and continuous. The resulting system of equations is formed by the [[Element Stiffness Matrix]] for each element, and the following [[Assembly Process]] is process to sum the continuous domain into a global stiffness matrix. 

$$ M = \A_e \in \mathcal{E} \{ M_e \} $$

### Finite Volume Method

[[Finite Volume Method]] is based on the principle of conservation laws. It integrates the flux across the boundaries of a [[Control Volume]], typically an elementary triangle or cell. The approximation for a flux across an edge is given by: 

$$ \int_{\partial V} \boldsymbol{\F_x} \text{ d}\boldsymbol{s} = \sum_{e \in \partial V} \bar{\F_x} L_e $$

In [[Finite Volume Method]], the unknowns are often associated with the cell centers (cell-centered) or cell vertices (cell-vertex). A common technique to handle high gradients is to use an [[Upwind Scheme]], which ensures the stability of the unique solution by maintaining diagonal dominance in the resulting linear system.

## Relationships

- [[Finite Difference Method]] approximates [[Laplacean Operator]]
- [[Finite Element Method]] utilizes [[Green's Formula]]
- [[Finite Volume Method]] is based on [[Control Volume]]
- [[Upwind Scheme]] improves stability of [[Finite Difference Method]]
- [[Finite Element Method]] solves [[Laplacean Operator]]
- [[Element Stiffness Matrix]] is a component of [[Finite Element Method]]
- [[Finite Volume Method]] approximates [[Laplacean Operator]]
- [[Green's Formula]] is a generalization of [[Divergence Theorem]]
- [[Finite Element Method]] requires [[Assembly Process]]
- [[Finite Volume Method]] can incorporate [[Upwind Scheme]]
- [[Finite Volume Method]] depends on [[Control Volume]]
- [[Finite Element Method]] uses [[Divergence Theorem]]
- [[Finite Volume Method]] uses [[Upwind Scheme]]
- [[Finite_Difference_Method]] can be stabilized by [[Upwind_Scheme]]
- [[Finite_Element_Method]] produces [[Element_Stiffness_Matrix]]
- [[Finite_Element_Method]] compares to [[Finite_Volume_Method]]
