---
type: content
title: "Projective Geometry and 3D Transformations"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:52:32.753466400+00:00
summary: "This note explores the hierarchy of 3D transformations, from Euclidean to projective, and the geometric invariants that define them. It specifically details the screw decomposition of Euclidean motion and the fundamental role of the absolute conic and absolute dual quadric in defining metric properties."
tags:
  - computer-vision
  - projective-geometry
  - 3d-transformations
  - linear-algebra
entities:
  - "[[Projective Geometry]]"
  - "[[Euclidean Transformation]]"
  - "[[Screw Decomposition]]"
  - "[[Absolute Conic]]"
  - "[[Absolute Dual Quadric]]"
  - "[[Similarity Transformation]]"
  - "[[Plane at Infinity]]"
  - "[[Homography]]"
relationships:
  - source: "Euclidean Transformation"
    target: "Screw Decomposition"
    description: "can be decomposed via"
  - source: "Projective Geometry"
    target: "Absolute Conic"
    description: "contains"
  - source: "Absolute Conic"
    target: "Plane at Infinity"
    description: "lies on"
  - source: "Absolute Dual Quadric"
    target: "Absolute Conic"
    description: "is the dual of"
  - source: "Similarity Transformation"
    target: "Absolute Conic"
    description: "fixes"
  - source: "Similarity Transformation"
    target: "Absolute Dual Quadric"
    description: "fixes"
  - source: "Homography"
    target: "Projective Geometry"
    description: "is a type of transformation in"
  - source: "Euclidean Transformation"
    target: "Similarity Transformation"
    description: "is a specialization of"
---

# Projective Geometry and 3D Transformations

*This note explores the hierarchy of 3D transformations, from Euclidean to projective, and the geometric invariants that define them. It specifically details the screw decomposition of Euclidean motion and the fundamental role of the absolute conic and absolute dual quadric in defining metric properties.*

This note covers the mathematical foundations of transformations in 3D space, specifically within the context of [[Projective Geometry]].

## Concept
Transformations in 3D space are categorized by their degrees of freedom (dof) and the invariants they preserve. A hierarchy exists where higher-level transformations (like projective) contain all the properties of lower-level ones (like Euclidean).

### The Hierarchy of Transformations
- **Projective**: 15 dof. Preserves intersection and tangency.
- **Affine**: 12 dof. Preserves parallelism, volume ratios, and centroids.
- **Similarity**: 7 dof. Preserves the absolute conic and angles.
- **Euclidean**: 6 dof. Preserves volume and absolute conic.

### Screw Decomposition
Any [[Euclidean Transformation]] can be decomposed into a rotation about a screw axis and a translation along that same axis. This is known as the [[Screw Decomposition]].

$$ 	ext{Any particular translation and rotation is equivalent to a rotation about a screw axis together with a translation along the screw axis.} $$

This result states that any rigid body motion can be represented as a combination of rotation and translation along a single axis.

### Metric Invariants
In projective space, metric properties (like angles and lengths) are not inherently defined. They are recovered through specific geometric entities:

1. **[[Plane at Infinity]]**: A geometric representation of the 3 degrees of freedom required to specify affine properties. It is a fixed plane under any affine transformation.
2. **[[Absolute Conic]]**: A conic on the plane at infinity that represents the 5 additional degrees of freedom required for metric properties. It is a [[Similarity Transformation]] fixes this conic.

$$ 	ext{The absolute conic } Ω 	ext{ is a fixed conic under the projective transformation } H 	ext{ if, and only if, } H 	ext{ is a similarity transformation.} $$

This formula defines the condition under which a projective transformation preserves metric structure.

3. **[[Absolute Dual Quadric]]**: The dual of the absolute conic, which is a degenerate quadric in 3-space. It is used to represent the 8 degrees of freedom required for metric properties in a projective frame. It is also fixed by a [[Similarity Transformation]].

$$ 	ext{The angle between two planes } π_1 	ext{ and } π_2 	ext{ is given by: } $$
$$ 	ext{T}π_1 	ext{Q} π_2 / (	ext{T}π_1 	ext{Q} π_2)^{1/2} 	ext{T}π_2 	ext{Q} π_2^{1/2} $$ 

This expression calculates the angle between two planes using the quadric Q.

## Relationships
- [[Euclidean Transformation]] can be decomposed via [[Screw Decomposition]]
- [[Projective Geometry]] contains [[Absolute Conic]]
- [[Absolute Conic]] lies on [[Plane at Infinity]]
- [[Absolute Dual Quadric]] is the dual of [[Absolute Conic]]
- [[Similarity Transformation]] fixes [[Absolute Conic]]
- [[Similarity Transformation]] fixes [[Absolute Dual Quadric]]
- [[Homography]] is a type of transformation in [[Projective Geometry]]
- [[Euclidean Transformation]] is a specialization of [[Similarity Transformation]]
