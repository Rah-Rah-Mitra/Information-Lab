---
type: content
title: "Geometric Primitives and Coordinate Transformations"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:43:25.208389300+00:00
summary: "This note covers the fundamental geometric primitives (points, lines, planes) and the hierarchy of coordinate transformations used in computer vision. It explains how 2D and 3D entities are represented using homogeneous coordinates and how various transformations (translation, rotation, scaling, affine, and projective) preserve specific geometric properties. Understanding these structures is essential for tasks like image alignment, 3D reconstruction, and camera calibration."
tags:
  - computer-vision
  - geometry
  - coordinate-transformations
  - linear-algebra
entities:
  - "[[Geometric Primitives]]"
  - "[[Homogeneous Coordinates]]"
  - "[[2D Transformations]]"
  - "[[3D Transformations]]"
  - "[[Rotation Matrix]]"
  - "[[Translation]]"
  - "[[Affine Transformation]]"
  - "[[Projective Transformation]]"
  - "[[Unit Quaternions]]"
  - "[[Rodrigues' Formula]]"
  - "[[Plucker Coordinates]]"
  - "[[Euclidean Transformation]]"
relationships:
  - source: "Geometric Primitives"
    target: "Homogeneous Coordinates"
    description: "are represented using"
  - source: "2D Transformations"
    target: "Geometric Primitives"
    description: "act upon"
  - source: "3D Transformations"
    target: "Geometric Primitives"
    description: "act upon"
  - source: "Rotation Matrix"
    target: "Euclidean Transformation"
    description: "is a component of"
  - source: "Unit Quaternions"
    target: "Rotation Matrix"
    description: "can represent a"
  - source: "Rodrigues' Formula"
    target: "Rotation Matrix"
    description: "generates a"
  - source: "Projective Transformation"
    target: "Affine Transformation"
    description: "generalises"
  - source: "Projective Transformation"
    target: "Euclidean Transformation"
    description: "generalises"
  - source: "Affine Transformation"
    target: "Euclidean Transformation"
    description: "generalises"
  - source: "Euclidean Transformation"
    target: "Rotation Matrix"
    description: "includes"
  - source: "Euclidean Transformation"
    target: "Translation"
    description: "includes"
  - source: "Unit Quaternions"
    target: "Rotation Matrix"
    description: "provides an alternative to"
  - source: "Plucker Coordinates"
    target: "3D Lines"
    description: "parameterize"
---

# Geometric Primitives and Coordinate Transformations

*This note covers the fundamental geometric primitives (points, lines, planes) and the hierarchy of coordinate transformations used in computer vision. It explains how 2D and 3D entities are represented using homogeneous coordinates and how various transformations (translation, rotation, scaling, affine, and projective) preserve specific geometric properties. Understanding these structures is essential for tasks like image alignment, 3D reconstruction, and camera calibration.*

This note provides a foundational overview of the geometric primitives and the mathematical frameworks for coordinate transformations used in computer vision.

## Concept

To describe the geometry of a scene, one must define [[Geometric Primitives]] such as points, lines, and planes. These primitives are often represented using [[Homogeneous Coordinates]], which allow for a unified treatment of points at infinity and simplifies the composition of transformations through matrix multiplication.

### 2D Primitives and Transformations

In 2D, a point can be represented as a vector $\mathbf{x} = (x, y)^T$. Using homogeneous coordinates, this becomes $\tilde{\mathbf{x}} = (\tilde{x}, y, w)^T$, where the inhomogeneous point is recovered by dividing by the last element $w$.

$$ \tilde{\x} = (\tilde{x}, y, w)^T = w \bar{\x} = w(x, y, 1)^T $$

The hierarchy of [[2D Transformations]] ranges from simple [[Translation]] to complex [[Projective Transformation]] (also known as a homography). 

- [[Translation]]: $\mathbf{x}' = \mathbf{x} + \	extbf{t}$
- [[Euclidean Transformation]] (Rigid Body Motion): $\mathbf{x}' = \mathbf{R}\mathbf{x} + \mathbf{t}$, where $\mathbf{R}$ is an orthonormal [[Rotation Matrix]].
- [[Affine Transformation]]: Preserves parallelism.
- [[Projective Transformation]]: Preserves straight lines.

### 3D Primitives and Transformations

[[3D Transformations]] follow a similar hierarchy. A 3D [[Euclidean Transformation]] (or $SE(3)$) is defined by a rotation and a translation.

$$ \mathbf{x}' = \mathbf{R}\mathbf{x} + \mathbf{c} $$

This note specifically addresses the parameterization of the [[Rotation Matrix]] in 3D, which is more complex than in 2D.

#### Rotation Parameterizations

1. **Axis/Angle (Exponential Twist):** A rotation can be represented by an axis $\hat{\{n\}}$ and an angle $\theta$. [[Rodrigues' Formula]] provides a direct way to compute the rotation matrix from these parameters:

$$ \mathbf{R}(\hat{\n}, \theta) = \mathbf{I} + \sin\theta [\hat{\n}] + (1 - \cos\theta) [\hat{\n}]^2 $$

This formula maps the rotation vector $\boldsymbol{\w}$ to the matrix $\R$.

2. **[[Unit Quaternions]]:** A 4-vector $\q = (x, y, z, w)$ that lives on a unit sphere. [[Unit Quaternions]] are highly popular for pose interpolation because they provide a continuous and unique representation (up to antipodal sign ambiguity).

3. **[[Plucker Coordinates]]:** A minimal representation for 3D lines, using six independent entries in a skew-symmetric matrix.

## Relationships
- [[Geometric Primitives]] are represented using [[Homogeneous Coordinates]]
- [[2D Transformations]] act upon [[Geometric Primitives]]
- [[3D Transformations]] act upon [[Geometric Primitives]]
- [[Rotation Matrix]] is a component of [[Euclidean Transformation]]
- [[Unit Quaternions]] can represent a [[Rotation Matrix]]
- [[Rodrigues' Formula]] generates a [[Rotation Matrix]]
- [[Projective Transformation]] generalises [[Affine Transformation]]
- [[Projective Transformation]] generalises [[Euclidean Transformation]]
- [[Affine Transformation]] generalises [[Euclidean Transformation]]
- [[Euclidean Transformation]] includes [[Rotation Matrix]]
- [[Euclidean Transformation]] includes [[Translation]]
- [[Unit Quaternions]] provides an alternative to [[Rotation Matrix]]
- [[Plucker Coordinates]] parameterize 3D Lines
