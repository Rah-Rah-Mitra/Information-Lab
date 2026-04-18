---
type: content
title: "Parametrization of the n-Sphere"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:23:51.232643+00:00
summary: "A method for constraining parameter vectors to lie on a unit n-sphere using local parametrizations and Householder matrices to reduce computational cost."
tags:
  - computer-vision
  - linear-algebra
  - optimization
  - geometry
entities:
  - "[[n-Sphere]]"
  - "[[Householder Matrix]]"
  - "[[Local Parametrization]]"
  - "[[Jacobian Matrix]]"
  - "[[Euclidean Bundle Adjustment]]"
  - "[[Unit Vector]]"
  - "[[Cost Function]]"
relationships:
  - source: "Local Parametrization"
    target: "n-Sphere"
    description: "provides a mapping onto"
  - source: "Householder Matrix"
    target: "Unit Vector"
    description: "transforms a vector to lie along the coordinate axis"
  - source: "Local Parametrization"
    target: "Jacobian Matrix"
    description: "is used to compute the"
  - source: "Euclidean Bundle Adjustment"
    target: "n-Sphere"
    description: "requires parameters to lie on a"
  - source: "Cost Function"
    target: "Jacobian Matrix"
    description: "is differentiated to produce a"
  - source: "Local Parametrization"
    target: "Cost Function"
    description: "constrains the parameters of a"
---

# Parametrization of the n-Sphere

*A method for constraining parameter vectors to lie on a unit n-sphere using local parametrizations and Householder matrices to reduce computational cost.*

The [[n-Sphere]] parametrization is a technique used in geometric optimization to ensure that a parameter vector remains a [[Unit Vector]] of length one, avoiding over-parametrization.

## Local Parametrization Methods
To parametrize a sphere of dimension $n$ (consisting of $n+1$ vectors of unit length), a [[Local Parametrization]] is used. This is a map from $\mathbb{R}^n$ to the sphere that is well-behaved near the origin. Two common choices for this map $f(y)$ include:
1. A mapping that parametrizes half the sphere.
2. A mapping using the $\text{sinc}$ function, which parametrizes the entire sphere with a single singularity at $y = 2\pi$.

## Integration with Householder Matrices
To apply this locally around a specific point $x$ on the sphere, a [[Householder Matrix]] $H$ is employed to transform $x$ to the coordinate axis $(0, \dots, 0, 1)$. The composite map $y \mapsto H f(y)$ then provides a local parametrization in the neighborhood of $x$.

## Application in Optimization
In problems such as [[Euclidean Bundle Adjustment]], a [[Cost Function]] is often defined for all values in $\mathbb{R}^{n+1}$, but the minimization must be constrained to the sphere. By using local parametrizations, the [[Jacobian Matrix]] of the cost function with respect to the minimal parameter set $y$ can be computed efficiently by multiplying the full Jacobian by the first $n$ columns of the Householder matrix.

## Relationships
- Local Parametrization provides a mapping onto the n-Sphere.
- Householder Matrix transforms a vector to lie along the coordinate axis for a Unit Vector.
- Local Parametrization is used to compute the Jacobian Matrix.
- Euclidean Bundle Adjustment requires parameters to lie on an n-Sphere.
- Cost Function is differentiated to produce a Jacobian Matrix.
- Local Parametrization constrains the parameters of a Cost Function.
