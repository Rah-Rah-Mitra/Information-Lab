---
type: content
title: "The Trifocal Tensor and Three-View Geometry"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:52:07.757787900+00:00
summary: "The trifocal tensor encapsulates all projective geometric relations between three views that are independent of scene structure. It enables point and line transfer across views and provides a more stable estimation of camera geometry than pairwise fundamental matrices. The tensor is defined by 18 degrees of freedom and satisfies specific trilinear incidence relations."
tags:
  - computer-vision
  - multiple-view-geometry
  - tensor-notation
  - epipolar-geometry
entities:
  - "[[Trifocal Tensor]]"
  - "[[Fundamental Matrix]]"
  - "[[Epipolar Geometry]]"
  - "[[Epipole]]"
  - "[[Trilinearities]]"
  - "[[Camera Matrices]]"
  - "[[Homography]]"
  - "[[Bas-Relief Ambiguity]]"
  - "[[Projective Transformation]]"
  - "[[Trifocal Plane]]"
relationships:
  - source: "Trifocal Tensor"
    target: "Fundamental Matrix"
    description: "generalises"
  - source: "Trifocal Tensor"
    target: "Epipole"
    description: "determines"
  - source: "Trifocal Tensor"
    target: "Trilinearities"
    description: "encodes"
  - source: "Trifocal Tensor"
    target: "Camera Matrices"
    description: "determines"
  - source: "Trifocal Tensor"
    target: "Homography"
    description: "induces"
  - source: "Trifocal Tensor"
    target: "Trifocal Plane"
    description: "relates to"
  - source: "Trifocal Tensor"
    target: "Epipolar Geometry"
    description: "extends"
  - source: "Trifocal Tensor"
    target: "Bas-Relief Ambiguity"
    description: "resolves"
  - source: "Trifocal Tensor"
    target: "Projective Transformation"
    description: "is invariant to"
---

# The Trifocal Tensor and Three-View Geometry

*The trifocal tensor encapsulates all projective geometric relations between three views that are independent of scene structure. It enables point and line transfer across views and provides a more stable estimation of camera geometry than pairwise fundamental matrices. The tensor is defined by 18 degrees of freedom and satisfies specific trilinear incidence relations.*

This note explores the geometric and algebraic properties of the [[Trifocal Tensor]], a central object in three-view geometry that extends the principles of [[Epipolar Geometry]] found in two-view systems.

## Concept
The [[Trifocal Tensor]] is a valency-3 tensor with 18 degrees of freedom that captures all projective geometric relations between three views, independent of scene structure. Unlike the [[Fundamental Matrix]] [[F]] which only relates two views, the trifocal tensor provides a mechanism for transferring points and lines across three views. It is represented by a homogeneous $3 	imes 3 	imes 3$ array of 27 elements.

In tensor notation, the basic incidence relation is expressed as:

$$ T^{ijk} l_i l_j l_k = 0 $$

This equation represents the trilinear relationship between corresponding lines in three views. The tensor is defined by the three matrices $T_i^{jk}$ which are can be extracted from the camera matrices $P_i$ using the following relationship:

$$ T^{ijk} = a^i b^j a^k b^k 	ext{ (simplified form)} $$

## Point and Line Transfer
One of the primary advantages of the trifocal tensor is its ability to perform point and line transfer. A line in the second view induces a [[Homography]] between the first and third views. This homography $H$ is given by:

$$ x_{13} = H x_1, 	ext{ where } H = [T^{1jk} l_j] $$ 

This mechanism allows for a point $x_1$ in the first view to be mapped to its corresponding point $x_3$ in the third view using a single line $l_2$ in the second view. This is significantly more robust than [[Epipolar Transfer]], which fails for any point on the [[Trifocal Plane]] (the plane defined by the three camera centres) and becomes inaccurate near it.
n
## Algebraic Properties and Constraints
The [[Trifocal Tensor]] must satisfy several internal constraints to be geometrically valid. While it has 27 entries, it only has 18 independent parameters. This means the elements must satisfy 8 independent algebraic constraints. 

Failure to satisfy these constraints leads to numerical instability, such as when computing [[Epipole]]s from the tensor. The [[Epipole]] $e$ in the second and third images can be computed as the common intersection of the lines represented by the left and right null-vectors of the matrices $T_i^{jk}$.

## Relationships
- [[Trifocal Tensor]] generalises [[Fundamental Matrix]]
- [[Trifocal Tensor]] determines [[Epipole]]
- [[Trifocal Tensor]] encodes [[Tr_linearities]]
- [[Trifocal Tensor]] determines [[Camera Matrices]]
- [[Trifocal Tensor]] induces [[Homography]]
- [[Trifocal Tensor]] relates to [[Trifocal Plane]]
- [[Trifocal Tensor]] extends [[Epipolar Geometry]]
- [[Trifocal Tensor]] is invariant to [[Projective Transformation]]
