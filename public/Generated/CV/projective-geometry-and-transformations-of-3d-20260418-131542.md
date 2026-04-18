---
type: content
title: "Projective Geometry and Transformations of 3D"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:15:42.277307500+00:00
summary: "An exploration of projective 3-space, including the representation of points, planes, lines, and quadrics, and the hierarchy of 3D projective transformations."
tags:
  - gis
  - projective-geometry
  - computer-vision
  - linear-algebra
entities:
  - "[[Projective 3-Space]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Projective Transformation]]"
  - "[[Plane]]"
  - "[[Line]]"
  - "[[Quadric]]"
  - "[[Dual Quadric]]"
  - "[[Plucker Matrix]]"
  - "[[Plucker Line Coordinates]]"
  - "[[Klein Quadric]]"
  - "[[Twisted Cubic]]"
  - "[[Plane at Infinity]]"
  - "[[Absolute Conic]]"
  - "[[Absolute Dual Quadric]]"
  - "[[Screw Decomposition]]"
relationships:
  - source: "Projective Transformation"
    target: "Projective 3-Space"
    description: "acts on"
  - source: "Homogeneous Coordinates"
    target: "Projective 3-Space"
    description: "represent points in"
  - source: "Plane"
    target: "Projective 3-Space"
    description: "is an entity in"
  - source: "Line"
    target: "Projective 3-Space"
    description: "is an entity in"
  - source: "Quadric"
    target: "Projective 3-Space"
    description: "is a surface in"
  - source: "Dual Quadric"
    target: "Quadric"
    description: "is the dual of"
  - source: "Plucker Matrix"
    target: "Line"
    description: "represents"
  - source: "Plucker Line Coordinates"
    target: "Plucker Matrix"
    description: "are the non-zero elements of"
  - source: "Plucker Line Coordinates"
    target: "Klein Quadric"
    description: "satisfy the equation of"
  - source: "Twisted Cubic"
    target: "Projective 3-Space"
    description: "is a curve in"
  - source: "Plane at Infinity"
    target: "Projective 3-Space"
    description: "is a special plane in"
  - source: "Absolute Conic"
    target: "Plane at Infinity"
    description: "is a conic on"
  - source: "Absolute Dual Quadric"
    target: "Absolute Conic"
    description: "is the dual of"
  - source: "Screw Decomposition"
    target: "Projective Transformation"
    description: "is a decomposition of a Euclidean specialization of"
---

# Projective Geometry and Transformations of 3D

*An exploration of projective 3-space, including the representation of points, planes, lines, and quadrics, and the hierarchy of 3D projective transformations.*

Projective 3-space is a generalization of the projective plane where Euclidean 3-space is augmented with a plane at infinity to handle ideal points and parallel entities.

## Points and Transformations

Points in [[Projective 3-Space]] are represented using [[Homogeneous Coordinates]] as 4-vectors. A [[Projective Transformation]] is a linear transformation on these vectors represented by a non-singular 4x4 matrix, preserving incidence relations and collineations.

## Planes, Lines, and Quadrics

### Planes
A [[Plane]] in 3-space is represented by a 4-vector. Three points in general position uniquely define a plane, and the intersection of three planes defines a unique point.

### Lines
A [[Line]] can be defined as the join of two points or the intersection of two planes. Lines are often represented using a [[Plucker Matrix]] (a skew-symmetric 4x4 matrix) or [[Plucker Line Coordinates]] (a homogeneous 6-vector). The constraint on these coordinates defines the [[Klein Quadric]], a co-dimension 1 surface in 5-space.

### Quadrics
A [[Quadric]] is a surface defined by a symmetric 4x4 matrix. The [[Dual Quadric]] represents the envelope of tangent planes to the point quadric. Quadrics are classified by their rank and signature, ranging from spheres and hyperboloids to degenerate forms like cones.

### Twisted Cubics
A [[Twisted Cubic]] is a 3D analogue of a 2D conic, defined as a parametric curve. Six points in general position uniquely define a twisted cubic.

## Metric Properties and Special Entities

### The Plane at Infinity
The [[Plane at Infinity]] is a fixed plane under any affine transformation. It allows for the identification of parallelism: parallel planes intersect in a line on this plane, and parallel lines intersect at a point on it.

### The Absolute Conic
The [[Absolute Conic]] is a conic of purely imaginary points residing on the [[Plane at Infinity]]. It is fixed under any similarity transformation and is used to define orthogonality and angles in a projective frame.

### The Absolute Dual Quadric
The [[Absolute Dual Quadric]] is a degenerate dual quadric consisting of the planes tangent to the [[Absolute Conic]]. It provides a single geometric object to represent the 8 degrees of freedom required for metric properties in a projective coordinate frame.

## Euclidean Motion

Any Euclidean transformation (rotation and translation) can be reduced via [[Screw Decomposition]] to a rotation about a screw axis combined with a translation along that same axis.

## Relationships
- [[Projective Transformation]] acts on [[Projective 3-Space]]
- [[Homogeneous Coordinates]] represent points in [[Projective 3-Space]]
- [[Plane]] is an entity in [[Projective 3-Space]]
- [[Line]] is an entity in [[Projective 3-Space]]
- [[Quadric]] is a surface in [[Projective 3-Space]]
- [[Dual Quadric]] is the dual of [[Quadric]]
- [[Plucker Matrix]] represents [[Line]]
- [[Plucker Line Coordinates]] are the non-zero elements of [[Plucker Matrix]]
- [[Plucker Line Coordinates]] satisfy the equation of [[Klein Quadric]]
- [[Twisted Cubic]] is a curve in [[Projective 3-Space]]
- [[Plane at Infinity]] is a special plane in [[Projective 3-Space]]
- [[Absolute Conic]] is a conic on [[Plane at Infinity]]
- [[Absolute Dual Quadric]] is the dual of [[Absolute Conic]]
- [[Screw Decomposition]] is a decomposition of a Euclidean specialization of [[Projective Transformation]]
