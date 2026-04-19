---
type: content
title: "Multi-View Photo-consistency Measures"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:43:44.599934500+00:00
summary: "Multi-view photo-consistency measures the agreement between input photographs to estimate 3D geometry. It serves as the primary signal for optimization in multi-view stereo (MVS) algorithms. Different measures offer varying trade-offs between robustness to illumination changes and computational efficiency."
tags:
  - computer-vision
  - multi-view-stereo
  - 3d-reconstruction
  - photometric-consistency
entities:
  - "[[Multi-View Stereo]]"
  - "[[Photo-consistency]]"
  - "[[Epipolar Geometry]]"
  - "[[Normalized Cross Correlation]]"
  - "[[Sum of Squared Differences]]"
  - "[[Census Transform]]"
  - "[[Mutual Information]]"
  - "[[Sum of Absolute Differences]]"
  - "[[Structure from Motion]]"
  - "[[Space Carving]]"
relationships:
  - source: "Multi-View Stereo"
    target: "Photo-consistency"
    description: "uses"
  - source: "Multi-View Stereo"
    target: "Structure from Motion"
    description: "relies on"
  - source: "Photo-consistency"
    target: "Epipolar Geometry"
    description: "is constrained by"
  - source: "Normalized Cross Correlation"
    target: "Photo-consistency"
    description: "is a type of"
  - source: "Sum of Squared Differences"
    target: "Photo-consistency"
    description: "is a type of"
  - source: "Census Transform"
    target: "Photo-consistency"
    description: "is a type of"
  - source: "Mutual Information"
    target: "Photo-consistency"
    description: "is a type of"
  - source: "Sum of Absolute Differences"
    target: "Photo-consistency"
    description: "is a type of"
  - source: "Space Carving"
    target: "Photo-consistency"
    description: "addresses visibility in"
  - source: "Structure from Motion"
    target: "Multi-View Stereo"
    description: "provides camera parameters for"
---

# Multi-View Photo-consistency Measures

*Multi-view photo-consistency measures the agreement between input photographs to estimate 3D geometry. It serves as the primary signal for optimization in multi-view stereo (MVS) algorithms. Different measures offer varying trade-offs between robustness to illumination changes and computational efficiency.*

This note explores the concept of photo-consistency as the core optimization signal in [[Multi-View Stereo]] (MVS) pipelines.

## Concept
[[Multi-View Stereo]] aims to reconstruct 3D geometry from a set of images by maximizing the agreement between them, a concept known as [[Photo-consistency]]. Given a set of $N$ images and a 3D point $p$, the photo-consistency $C_{ij}$ between two images $I_i$ and $I_j$ is defined as:

$$ C_{ij}(p) = ho(I_i(\Omega(\pi_i(p))), I_j(\Omega(\pi_j(p)))) $$

This formula models the similarity between image intensities sampled within a support domain $\Omega$ around the projected point $\pi(p)$. The choice of the similarity measure $\rho$ determines the robustness and accuracy of the algorithm.

Common measures include:
- [[Normalized Cross Correlation]] (NCC): Invariant to gain and bias, making it ideal for varying illumination.
- [[Sum of Squared Differences]] (SSD): Computationally efficient but sensitive to outliers.
- [[Sum of Absolute Differences]] (SAD): More robust to outliers than SSD due to its $L_1$ norm.
- [[Census Transform]]: Uses relative pixel ordering, providing robustness at depth boundaries.
- [[Mutual Information]]: Highly invariant to complex transformations but often requires larger domains.

In MVS, the matching problem is simplified from a 2D search to a 1D search along the epipolar line, a constraint provided by [[Epipolar Geometry]].

## Visibility and Space Carving
Computing photo-consistency requires knowing which images see a specific 3D point. This creates a dependency loop with the geometry itself. [[Space Carving]] is a seminal approach used to resolve this by iteratively removing voxels that fail the consistency test, using an ordinal visibility constraint to ensure occluders are processed before occluded points.

## Relationships
- [[Multi-View Stereo]] uses [[Photo-consistency]]
- [[Multi-View Stereo]] relies on [[Structure from Motion]]
- [[Photo-consistency]] is constrained by [[Epipolar Geometry]]
- [[Normalized Cross Correlation]] is a type of [[Photo-consistency]]
- [[Sum of Squared Differences]] is a type of [[Photo-consistency]]
- [[Census Transform]] is a type of [[Photo-consistency]]
- [[Mutual Information]] is a type of [[Photo-consistency]]
- [[Sum of Absolute Differences]] is a type of [[Photo-consistency]]
- [[Space Carving]] addresses visibility in [[Photo-consistency]]
- [[Structure from Motion]] provides camera parameters for [[Multi-View Stereo]]
