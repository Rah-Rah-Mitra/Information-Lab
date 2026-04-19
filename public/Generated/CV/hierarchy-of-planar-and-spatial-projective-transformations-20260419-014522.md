---
type: content
title: "Hierarchy of Planar and Spatial Projective Transformations"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:45:22.279298600+00:00
summary: "This note details the hierarchy of geometric transformations in 2D and 3D, ranging from Euclidean isometries to general projective transformations. It explains how each level of the hierarchy restricts invariants like distance, angle, and parallelism. The text also covers the representation of lines, planes, and quadrics using homogeneous coordinates and null-spaces."
tags:
  - projective-geometry
  - linear-algebra
  - computer-vision
  - geometry-hierarchy
  - homogeneous-coordinates
entities:
  - "[[Projective Transformation]]"
  - "[[Euclidean Transformation]]"
  - "[[Affine Transformation]]"
  - "[[Similarity Transformation]]"
  - "[[Isometry]]"
  - "[[Projective Plane]]"
  - "[[Projective Space]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Homogeneous Matrix]]"
  - "[[Circular Points]]"
  - "[[Dual Conic]]"
  - "[[Quadric]]"
  - "[[Plucker Coordinates]]"
  - "[[Plucker Matrix]]"
  - "[[Plucker Line]]"
  - "[[Klein Quadric]]"
  - "[[Twisted Cubic]]"
relationships:
  - source: "Projective Transformation"
    target: "Affine Transformation"
    description: "generalises"
  - source: "Projective Transformation"
    target: "Similarity Transformation"
    description: "generalises"
  - source: "Affine Transformation"
    target: "Similarity Transformation"
    description: "generalises"
  - source: "Similarity Transformation"
    target: "Euclidean Transformation"
    description: "generalises"
  - source: "Euclidean Transformation"
    target: "Isometry"
    description: "is a type of"
  - source: "Projective Transformation"
    target: "Homogeneous Coordinates"
    description: "represented by"
  - source: "Projective Transformation"
    target: "Projective Plane"
    description: "acts on"
  - source: "Projective Transformation"
    target: "Projective Space"
    description: "acts on"
  - source: "Similarity Transformation"
    target: "Circular Points"
    description: "fixes"
  - source: "Affine Transformation"
    target: "Homogeneous Coordinates"
    description: "uses"
  - source: "Projective Transformation"
    target: "Plucker Coordinates"
    description: "uses for lines in 3D"
  - source: "Projective Transformation"
    target: "Quadric"
    description: "transforms"
  - source: "Projective Transformation"
    target: "Twisted Cubic"
    description: "transforms"
  - source: "Projective Transformation"
    target: "Klein Quadric"
    description: "defines"
  - source: "Plucker Matrix"
    target: "Plucker Coordinates"
    description: "contains"
  - source: "Euclidean Transformation"
    target: "Homogeneous Coordinates"
    description: "uses"
---

# Hierarchy of Planar and Spatial Projective Transformations

*This note details the hierarchy of geometric transformations in 2D and 3D, ranging from Euclidean isometries to general projective transformations. It explains how each level of the hierarchy restricts invariants like distance, angle, and parallelism. The text also covers the representation of lines, planes, and quadrics using homogeneous coordinates and null-spaces.*

This note explores the hierarchy of geometric transformations and the mathematical entities they manipulate in both 2D and 3D projective geometry.

## Concept
Geometric transformations can be categorized into a hierarchy of increasing generality. In the 2D [[Projective Plane]], a [[Projective Transformation]] is the most general non-singular linear transformation of [[Homogeneous Coordinates]].

### The 2D Hierarchy
1. **[[Isometry]]**: Preserves Euclidean distance. A [[Euclidean Transformation]] (orientation-preserving isometry) is a composition of rotation and translation.
2. **[[Similarity Transformation]]**: An isometry composed with isotropic scaling. It preserves shape and is characterized by the [[Circular Points]] being fixed.
3. **[[Affine Transformation]]**: A non-singular linear transformation followed by a translation. It preserves parallelism and the ratio of lengths on parallel lines, but not angles.
4. **[[Projective Transformation]]**: The most general form, which can map the line at infinity to a finite line, and thus models vanishing points.

### The 3D Hierarchy and Representations
In [[Projective Space]], transformations act on 4-vectors. Lines and planes are represented using [[Homogeneous Coordinates]].

#### Lines in 3D
Lines in 3D have 4 degrees of freedom. They can be represented via [[Plucker Coordinates]] or a [[Plucker Matrix]]. A [[Plucker Matrix]] is a 4x4 skew-symmetric matrix whose non-zero elements are the [[Plucker Coordinates]]. These lines satisfy the constraint of the [[Klein Quadric]], which is the surface in projective space where all valid Plucker coordinates reside.

#### Quadrics and Conics
A [[Quadric]] is a surface defined by a quadratic form. In 3D, a [[Quadric]] is represented by a symmetric 4x4 matrix. The transformation of a quadric under a point transformation is given by:

$$ 
\tilde{Q} = H^{-T} Q H^{-1} 
$$

This equation models how a quadric surface transforms under a projective mapping.

#### Twisted Cubics
A [[Twisted Cubic]] is a 3D analogue of a 2D conic, defined as a parametric curve in projective space. It is a unique curve that can be passed through six points in general position.

## Relationships
- [[Projective Transformation]] generalises [[Affine Transformation]]
- [[Affine Transformation]] generalises [[Similarity Transformation]]
- [[Similarity Transformation]] generalises [[Euclidean Transformation]]
- [[Euclidean Transformation]] is a type of [[Isometry]]
- [[Projective Transformation]] acts on [[Projective Plane]]
- [[Projective Transformation]] acts on [[Projective Space]]
- [[Projective Transformation]] uses [[Homogeneous Coordinates]]
- [[Similarity Transformation]] fixes [[Circular Points]]
- [[Projective Transformation]] uses [[Plucker Coordinates]]
- [[Projective Transformation]] transforms [[Quadric]]
- [[Projective Transformation]] transforms [[Twisted Cubic]]
- [[Plucker Matrix]] contains [[Plucker Coordinates]]
- [[Plcker Line]] is defined by [[Plucker Coordinates]]
- [[Projective Transformation]] defines [[Klein Quadric]]
- [[Projective Transformation]] uses [[Homogeneous Coordinates]]
