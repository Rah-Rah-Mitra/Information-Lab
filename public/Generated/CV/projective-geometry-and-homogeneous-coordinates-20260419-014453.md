---
type: content
title: "Projective Geometry and Homogeneous Coordinates"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:44:53.017970700+00:00
summary: "Projective geometry extends Euclidean space by adding points at infinity to handle parallel lines and perspective transformations. It uses homogeneous coordinates to represent points and lines as vectors, enabling linear matrix operations for non-linear mappings. This framework is fundamental for computer vision tasks like camera calibration and 3D reconstruction."
tags:
  - computer-vision
  - projective-geometry
  - linear-algebra
  - homogeneous-coordinates
entities:
  - "[[Projective Geometry]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Euclidean Geometry]]"
  - "[[Projective Transformation]]"
  - "[[Ideal Points]]"
  - "[[Line at Infinity]]"
  - "[[Camera Matrix]]"
  - "[[Fundamental Matrix]]"
  - "[[Trifocal Tensor]]"
  - "[[Absolute Conic]]"
relationships:
  - source: "Projective Geometry"
    target: "Homogeneous Coordinates"
    description: "utilizes"
  - source: "Projective Geometry"
    target: "Euclidean Geometry"
    description: "generalises"
  - source: "Projective Geometry"
    target: "Projective Transformation"
    description: "is invariant under"
  - source: "Projective Transformation"
    target: "Homogeneous Coordinates"
    description: "represented by"
  - source: "Homogeneous Coordinates"
    target: "Ideal Points"
    description: "represent"
  - source: "Homogeneous Coordinates"
    target: "Line at Infinity"
    description: "defines"
  - source: "Projective Geometry"
    target: "Camera Matrix"
    description: "underpins"
  - source: "Projective Geometry"
    target: "Fundamental Matrix"
    description: "relates"
  - source: "Projective Geometry"
    target: "Trifocal Tensor"
    description: "relates"
  - source: "Projective Geometry"
    target: "Absolute Conic"
    description: "relates to"
---

# Projective Geometry and Homogeneous Coordinates

*Projective geometry extends Euclidean space by adding points at infinity to handle parallel lines and perspective transformations. It uses homogeneous coordinates to represent points and lines as vectors, enabling linear matrix operations for non-linear mappings. This framework is fundamental for computer vision tasks like camera calibration and 3D reconstruction.*

[[Projective Geometry]] provides a mathematical framework for modeling the geometric distortions that occur during perspective imaging, such as when a 2D image is captured from a 3D scene. Unlike [[Euclidean Geometry]], which requires special cases for parallel lines, projective geometry treats all points as equal by incorporating [[Ideal Points]] (points at infinity) into the space.

## Concept
In [[Projective Geometry]], points and lines are represented using [[Homogeneous Coordinates]]. A point in 2D Euclidean space \(x, y\) is represented as a 3-vector \(x = [x, y, 1]^T\). This allows non-linear mappings, like perspective projection, to be represented as linear matrix transformations.

$$ x = \begin{bmatrix} x \\ y \\ 1 \end{bmatrix} $$

Points with a final coordinate of zero, such as \(x = [x, y, 0]^T\), are [[Ideal Points]], which correspond to the points where parallel lines meet in the [[Line at Infinity]].

## Projective Transformations
An invertible mapping that preserves collinearity is a [[Projective Transformation]] (also known as a homography). It is represented by a non-singular 3x3 matrix \(H\) acting on homogeneous coordinates:

$$ x' = Hx $$

This transformation has eight degrees of freedom and preserves all projective properties of the geometry.

## Multi-View Geometry
In computer vision, the geometry of multiple views is essential for reconstructing 3D structure from 2D images. The [[Camera Matrix]] represents the imaging process, mapping 3D points to 2D image points. For two views, the [[Fundamental Matrix]] encapsulates the complete projective geometry of the camera pair, while the [[Trifocal Tensor]] relates correspondences across three views.

In 3D Euclidean space, the [[Absolute Conic]] is a key entity that determines the metric structure. Knowing the [[Absolute Conic]] is equivalent to knowing the camera calibration, as it allows for the ability to measure angles and lengths in a reconstructed scene.

## Relationships
- [[Projective Geometry]] utilizes [[Homogeneous Coordinates]]
- [[Projective Geometry]] generalises [[Euclidean Geometry]]
- [[Projective Geometry]] is invariant under [[Projective Transformation]]
- [[Projective Transformation]] is represented by [[Homogeneous Coordinates]]
- [[Homogeneous Coordinates]] represent [[Ideal Points]]
- [[Homogeneous Coordinates]] define [[Line at Infinity]]
- [[Projective Geometry]] underpins [[Camera Matrix]]
- [[Projective Geometry]] relates to [[Fundamental Matrix]]
- [[Projective Geometry]] relates to [[Trifocal Tensor]]
- [[Projective Geometry]] relates to [[Absolute Conic]]
