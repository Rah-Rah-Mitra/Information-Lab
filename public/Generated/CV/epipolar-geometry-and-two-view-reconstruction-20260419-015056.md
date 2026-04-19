---
type: content
title: "Epipolar Geometry and Two-View Reconstruction"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:50:56.847426+00:00
summary: "Epipolar geometry describes the intrinsic projective relationship between two camera views, encapsulated by the fundamental matrix. This relationship enables the 3D reconstruction of scene structure and camera poses from image correspondences, progressing from projective to metric reconstructions through stratified approaches."
tags:
  - computer-vision
  - epipolar-geometry
  - two-view-geometry
  - projective-reconstruction
  - fundamental-matrix
entities:
  - "[[Epipolar Geometry]]"
  - "[[Fundamental Matrix]]"
  - "[[Essential Matrix]]"
  - "[[Epipole]]"
  - "[[Epipolar Line]]"
  - "[[Projective Reconstruction]]"
  - "[[Metric Reconstruction]]"
  - "[[Absolute Conic]]"
  - "[[Camera Matrix]]"
  - "[[Homography]]"
  - "[[Pure Translation]]"
  - "[[Pure Planar Motion]]"
relationships:
  - source: "Epipolar Geometry"
    target: "Fundamental Matrix"
    description: "is encapsulated by"
  - source: "Fundamental Matrix"
    target: "Epipole"
    description: "contains information about"
  - source: "Fundamental Matrix"
    target: "Epipolar Line"
    description: "defines"
  - source: "Fundamental Matrix"
    target: "Essential Matrix"
    description: "generalises"
  - source: "Epipole"
    target: "Epipolar Line"
    description: "is the intersection point of all"
  - source: "Projective Reconstruction"
    target: "Fundamental Matrix"
    description: "is computed using"
  - source: "Metric Reconstruction"
    target: "Absolute Conic"
    description: "requires identification of"
  - source: "Pure Translation"
    target: "Fundamental Matrix"
    description: "results in a special form of"
  - source: "Pure Planar Motion"
    target: "Fundamental Matrix"
    description: "results in a special form of"
  - source: "Essential Matrix"
    target: "Fundamental Matrix"
    description: "is a special case of"
  - source: "Camera Matrix"
    target: "Fundamental Matrix"
    description: "determines"
  - source: "Homography"
    target: "Epipolar Line"
    description: "relates pencils of"
  - source: "Projective Reconstruction"
    target: "Metric Reconstruction"
    description: "is the first step in a stratified approach towards"
---

# Epipolar Geometry and Two-View Reconstruction

*Epipolar geometry describes the intrinsic projective relationship between two camera views, encapsulated by the fundamental matrix. This relationship enables the 3D reconstruction of scene structure and camera poses from image correspondences, progressing from projective to metric reconstructions through stratified approaches.*

This note explores the geometric constraints and algebraic representations of two-view geometry, specifically focusing on the relationship between image points in different views.

## Concept
[[Epipolar Geometry]] is the intrinsic projective geometry between two views, which is independent of scene structure and depends only on the camera's internal parameters and relative pose. It is mathematically represented by the [[Fundamental Matrix]] $F$, a $3 \times 3$ rank-2 matrix that satisfies the epipolar constraint for corresponding points $x$ and $x'$:

$$ x'^T F x = 0 $$

This constraint ensures that a point in the first image corresponds to a line in the second image, known as the [[Epipolar Line]]. The intersection of the baseline (the line joining the camera centers) with the image planes defines the points called [[Epipole]]s $e$ and $e'$.

### The Fundamental and Essential Matrices
While the [[Fundamental Matrix]] $F$ is used for uncalibrated cameras, the [[Essential Matrix]] $E$ is its specialization for calibrated cameras using normalized image coordinates. The relationship is given by:

$$ F = K' E K^{-1} $$

The [[Essential Matrix]] has a specific structure: its two singular values are equal and the third is zero. This allows for the 4-fold ambiguity in retrieving camera matrices from $E$, whereas the [[Fundamental Matrix]] only determines camera matrices up to a projective transformation.

### Special Motions and Reconstruction
Certain camera motions result in special forms of the [[Fundamental Matrix]]. For example, [[Pure Translation]] results in a skew-symmetric matrix, and [[Pure Planar Motion]] reduces the degrees of freedom from 7 to 6. 

[[Projective Reconstruction]] is the first stage of a stratified approach to 3D reconstruction. By computing $F$ from correspondences, one can recover the scene and cameras up to a projective transformation. To move towards a [[Metric Reconstruction]], one must identify the [[Absolute Conic]] $\Omega$, which is the planar conic lying in the plane at infinity. Identifying the absolute conic allows for the transformation of a projective reconstruction into a metric one, which preserves angles and length ratios.

## Relationships
- [[Epipolar Geometry]] is encapsulated by [[Fundamental Matrix]]
- [[Fundamental Matrix]] contains information about [[Epipole]]
- [[Fundamental Matrix]] defines [[Epipolar Line]]
- [[Fundamental Matrix]] generalises [[Essential Matrix]]
- [[Epipole]] is the intersection point of all [[Epipolar Line]]s
- [[Projective Reconstruction]] is computed using [[Fundamental Matrix]]
- [[Metric Reconstruction]] requires identification of [[Absolute Conic]]
- [[Pure Translation]] results in a special form of [[Fundamental Matrix]]
- [[Pure Planar Motion]] results in a special form of [[Fundamental Matrix]]
- [[Essential Matrix]] is a special case of [[Fundamental Matrix]]
- [[Camera Matrix]] determines [[Fundamental Matrix]]
- [[Homography]] relates pencils of [[Epipolar Line]]s
- [[Projective Reconstruction]] is the first step in a stratified approach towards [[Metric Reconstruction]]
