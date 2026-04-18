---
type: content
title: "Projective Geometry and Transformations of 2D"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:14:55.530404300+00:00
summary: "An exploration of 2D projective geometry, including homogeneous coordinates, projective transformations, and the recovery of affine and metric properties."
tags:
  - computer-vision
  - projective-geometry
  - linear-algebra
  - geometry
entities:
  - "[[Projective Geometry]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Projective Plane]]"
  - "[[Projective Transformation]]"
  - "[[Homography]]"
  - "[[Ideal Point]]"
  - "[[Line at Infinity]]"
  - "[[Duality Principle]]"
  - "[[Conic]]"
  - "[[Dual Conic]]"
  - "[[Affine Transformation]]"
  - "[[Similarity Transformation]]"
  - "[[Euclidean Transformation]]"
  - "[[Circular Points]]"
  - "[[Cross Ratio]]"
  - "[[Pole-Polar Relationship]]"
relationships:
  - source: "Projective Geometry"
    target: "Projective Plane"
    description: "studies the properties of"
  - source: "Projective Plane"
    target: "Homogeneous Coordinates"
    description: "is represented using"
  - source: "Projective Transformation"
    target: "Homography"
    description: "is synonymous with"
  - source: "Projective Transformation"
    target: "Projective Plane"
    description: "maps points and lines within"
  - source: "Ideal Point"
    target: "Line at Infinity"
    description: "is a point on"
  - source: "Duality Principle"
    target: "Projective Geometry"
    description: "is a fundamental principle of"
  - source: "Conic"
    target: "Dual Conic"
    description: "has a dual representation as a"
  - source: "Projective Transformation"
    target: "Affine Transformation"
    description: "generalizes"
  - source: "Affine Transformation"
    target: "Similarity Transformation"
    description: "generalizes"
  - source: "Similarity Transformation"
    target: "Euclidean Transformation"
    description: "generalizes"
  - source: "Circular Points"
    target: "Similarity Transformation"
    description: "are fixed points under"
  - source: "Cross Ratio"
    target: "Projective Transformation"
    description: "is invariant under"
  - source: "Pole-Polar Relationship"
    target: "Conic"
    description: "defines a relationship between points and lines via a"
  - source: "Line at Infinity"
    target: "Affine Transformation"
    description: "is a fixed line under"
---

# Projective Geometry and Transformations of 2D

*An exploration of 2D projective geometry, including homogeneous coordinates, projective transformations, and the recovery of affine and metric properties.*

Projective geometry is the study of geometric properties that remain invariant under a group of transformations known as projectivities.

## Homogeneous Representation

To facilitate computation, points and lines in the [[Projective Plane]] are represented using [[Homogeneous Coordinates]]. A point $(x, y)$ in $\mathbb{R}^2$ is represented as a 3-vector $(x, y, 1)^T$. Similarly, a line $ax + by + c = 0$ is represented by the vector $\mathbf{l} = (a, b, c)^T$. 

Two vectors are considered equivalent if they differ only by a non-zero scale factor. This equivalence class forms the projective space $\mathbb{P}^2$.

### Incidence and Intersection

A point $\mathbf{x}$ lies on a line $\mathbf{l}$ if and only if their inner product is zero: $\mathbf{x}^T\mathbf{l} = 0$. The intersection of two lines $\mathbf{l}_1$ and $\mathbf{l}_2$ is given by the cross product $\mathbf{x} = \mathbf{l}_1 \times \mathbf{l}_2$.

## Projective Transformations

A [[Projective Transformation]] (also called a [[Homography]] or collineation) is an invertible mapping from the projective plane to itself that preserves collinearity. Algebraically, it is represented by a non-singular $3 \times 3$ matrix $\mathbf{H}$ such that $\mathbf{x}' = \mathbf{H}\mathbf{x}$.

## The Hierarchy of Transformations

Projective transformations form a group, and several important subgroups exist, creating a hierarchy of increasing specialization:

1. **[[Projective Transformation]]**: The most general, with 8 degrees of freedom. Preserves the [[Cross Ratio]].
2. **[[Affine Transformation]]**: A subgroup that fixes the [[Line at Infinity]]. It preserves parallelism and ratios of lengths on parallel lines.
3. **[[Similarity Transformation]]**: A subgroup of affine transformations that preserves angles and ratios of lengths. It fixes the [[Circular Points]].
4. **[[Euclidean Transformation]]**: The most specialized, preserving Euclidean distance, area, and angles.

## Ideal Points and the Line at Infinity

Points with a homogeneous coordinate $x_3 = 0$ are called [[Ideal Point]]s (or points at infinity). The set of all such points forms the [[Line at Infinity]], denoted by $\mathbf{l}_\infty = (0, 0, 1)^T$. In the projective plane, parallel lines meet at an ideal point on this line.

## Conics and Duality

A [[Conic]] is a curve described by a second-degree equation, represented by a symmetric $3 \times 3$ matrix $\mathbf{C}$. A [[Dual Conic]] represents the envelope of tangent lines to the point conic.

### Duality Principle

The [[Duality Principle]] states that to any theorem of 2D projective geometry, there corresponds a dual theorem obtained by interchanging the roles of points and lines.

### Polarity

The [[Pole-Polar Relationship]] defines a mapping where a point $\mathbf{x}$ (the pole) and a conic $\mathbf{C}$ define a line $\mathbf{l} = \mathbf{C}\mathbf{x}$ (the polar).

## Relationships

- [[Projective Geometry]] studies the properties of the [[Projective Plane]].
- The [[Projective Plane]] is represented using [[Homogeneous Coordinates]].
- A [[Projective Transformation]] is synonymous with a [[Homography]].
- A [[Projective Transformation]] maps points and lines within the [[Projective Plane]].
- An [[Ideal Point]] is a point on the [[Line at Infinity]].
- The [[Duality Principle]] is a fundamental principle of [[Projective Geometry]].
- A [[Conic]] has a dual representation as a [[Dual Conic]].
- A [[Projective Transformation]] generalizes an [[Affine Transformation]].
- An [[Affine Transformation]] generalizes a [[Similarity Transformation]].
- A [[Similarity Transformation]] generalizes a [[Euclidean Transformation]].
- [[Circular Points]] are fixed points under a [[Similarity Transformation]].
- The [[Cross Ratio]] is invariant under a [[Projective Transformation]].
- The [[Pole-Polar Relationship]] defines a relationship between points and lines via a [[Conic]].
- The [[Line at Infinity]] is a fixed line under an [[Affine Transformation]].
