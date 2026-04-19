---
type: content
title: "Two-View Geometry and Homographies"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:51:43.500167500+00:00
summary: "This note explores the relationship between epipolar geometry and plane-induced homographies in both projective and affine camera models. It details how scene planes induce specific transformations between views and how these transformations can be used to optimally estimate fundamental matrices and 3D structures. The discussion covers geometric error minimization, polynomial solutions for triangulation, and reconstruction ambiguities like the bas-relief effect."
tags:
  - computer-vision
  - projective-geometry
  - epipolar-geometry
  - homographies
  - affine-geometry
entities:
  - "[[Epipolar Geometry]]"
  - "[[Homography]]"
  - "[[Fundamental Matrix]]"
  - "[[Triangulation]]"
  - "[[Sampson Error]]"
  - "[[Affine Camera]]"
  - "[[Scene Plane]]"
  - "[[Bas-Relief Ambiguity]]"
  - "[[Plane-Induced Parallax]]"
  - "[[Projective Reconstruction]]"
relationships:
  - source: "Homography"
    target: "Epipolar Geometry"
    description: "is compatible with"
  - source: "Homography"
    target: "Scene Plane"
    description: "is induced by"
  - source: "Epipolar Geometry"
    target: "Fundamental Matrix"
    description: "is represented by"
  - source: "Triangulation"
    target: "Sampson Error"
    description: "uses"
  - source: "Triangulation"
    target: "Epipolar Geometry"
    description: "depends on"
  - source: "Homography"
    target: "Bas-Relief Ambiguity"
    description: "exhibits"
  - source: "Scene Plane"
    target: "Plane-Induced Parallax"
    description: "generates"
  - source: "Fundamental Matrix"
    target: "Epipolar Geometry"
    description: "encodes"
  - source: "Affine Camera"
    target: "Epipolar Geometry"
    description: "simplifies"
  - source: "Projective Reconstruction"
    target: "Homography"
    description: "is upgraded to affine via"
---

# Two-View Geometry and Homographies

*This note explores the relationship between epipolar geometry and plane-induced homographies in both projective and affine camera models. It details how scene planes induce specific transformations between views and how these transformations can be used to optimally estimate fundamental matrices and 3D structures. The discussion covers geometric error minimization, polynomial solutions for triangulation, and reconstruction ambiguities like the bas-relief effect.*

This note covers the mathematical foundations of two-view geometry, focusing on the interplay between epipolar constraints and planar transformations.

## Concept
In two-view geometry, the relationship between points in two images is governed by [[Epipolar Geometry]]. For perspective cameras, this is captured by the [[Fundamental Matrix]] which satisfies the equation \(x'^T F x = 0\). When points lie on a [[Scene Plane]], they are related by a [[Homography]], a projective transformation that maps points from one view to the other.

### Geometric Error and Triangulation
To recover 3D structure from noisy correspondences, one must minimize a geometric error cost function. A common first-order approximation is the [[Sampson Error]], which provides a computationally efficient correction to measured points. For optimal triangulation, the text describes a non-iterative method that reduces the problem to finding the roots of a sixth-degree polynomial.

$$ d^2 = d_x^2 + d_{x'}^2 $$

The formula above models the squared Euclidean distance between points satisfying the epipolar constraint.

### Homographies and Compatibility
A [[Homography]] is said to be compatible with a [[Fundamental Matrix]] if it satisfies the condition that the matrix \(F H$ is skew-symmetric. This compatibility ensures that the homography is indeed induced by a physical plane in the scene.

### Affine Geometry
Under an [[Affine Camera]] model, the epipolar geometry simplifies significantly: all epipolar lines are parallel, and the epipoles lie at infinity. The [[Affine Fundamental Matrix]] is a rank-2 matrix with only four degrees of freedom. In this case, [[Triangulation]] and reconstruction are more linear and computationally efficient.

### Reconstruction Ambiguities
Even with perfect calibration, two-view reconstruction is subject to ambiguities. The [[Bas-Relief Ambiguity]] (or depth-turn ambiguity) occurs when the rotation of a camera about the normal to the epipolar planes is confounded with the depth of the object, meaning only their product can be determined. Similarly, the [[Necker Reversal]] ambiguity arises from the mirror-image rotation under parallel projection.

## Relationships
- [[Homography]] is compatible with [[Epipolar Geometry]]
- [[Homography]] is induced by a [[Scene Plane]]
- [[Epipolar Geometry]] is represented by the [[Fundamental Matrix]]
- [[Triangulation]] uses [[Sampson Error]]
- [[Triangulation]] depends on [[Epipolar Geometry]]
- [[Homography]] exhibits [[Bas-Relief Ambiguity]]
- [[Scene Plane]] generates [[Plane-Induced Parallax]]
- [[Fundamental Matrix]] encodes [[Epipolar Geometry]]
- [[Affine Camera]] simplifies [[Epipolar Geometry]]
- [[Projective Reconstruction]] is upgraded to affine via [[Homography]]
