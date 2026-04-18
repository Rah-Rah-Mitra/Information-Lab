---
type: content
title: "Structure from Motion"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:51:15.634882900+00:00
summary: "A process for simultaneously estimating 3D point structure and camera pose from multiple images using sparse feature correspondences."
tags:
  - computer-vision
  - 3d-reconstruction
  - geometry
  - photogrammetry
entities:
  - "[[Structure from Motion]]"
  - "[[Triangulation]]"
  - "[[Epipolar Geometry]]"
  - "[[Essential Matrix]]"
  - "[[Fundamental Matrix]]"
  - "[[Bundle Adjustment]]"
  - "[[Factorization]]"
  - "[[Self-Calibration]]"
  - "[[Chirality]]"
  - "[[Bas-Relief Ambiguity]]"
  - "[[Gauge Ambiguity]]"
  - "[[Simultaneous Localization and Mapping]]"
relationships:
  - source: "Structure from Motion"
    target: "Triangulation"
    description: "uses"
  - source: "Structure from Motion"
    target: "Epipolar Geometry"
    description: "depends on"
  - source: "Structure from Motion"
    target: "Bundle Adjustment"
    description: "is refined by"
  - source: "Structure from Motion"
    target: "Factorization"
    description: "can be solved via"
  - source: "Epipolar Geometry"
    target: "Essential Matrix"
    description: "is encoded by"
  - source: "Epipolar Geometry"
    target: "Fundamental Matrix"
    description: "is encoded by in uncalibrated cases"
  - source: "Bundle Adjustment"
    target: "Triangulation"
    description: "iteratively optimizes"
  - source: "Self-Calibration"
    target: "Fundamental Matrix"
    description: "recovers intrinsics from"
  - source: "Structure from Motion"
    target: "Chirality"
    description: "must satisfy"
  - source: "Structure from Motion"
    target: "Bas-Relief Ambiguity"
    description: "is subject to"
  - source: "Structure from Motion"
    target: "Gauge Ambiguity"
    description: "is subject to"
  - source: "Structure from Motion"
    target: "Simultaneous Localization and Mapping"
    description: "is closely related to"
---

# Structure from Motion

*A process for simultaneously estimating 3D point structure and camera pose from multiple images using sparse feature correspondences.*

[[Structure from Motion]] is the simultaneous recovery of 3D scene structure (point positions) and camera pose (rotation and translation) from image correspondences across multiple views.

## Core Techniques

### Triangulation
[[Triangulation]] is the process of estimating a 3D point's location given its 2D projections in multiple images and known camera positions. It is often solved as a least-squares problem by finding the point closest to all corresponding optical rays.

### Epipolar Geometry
[[Epipolar Geometry]] describes the geometric constraints between two views. In calibrated cameras, this is represented by the [[Essential Matrix]], which relates corresponding points via the epipolar constraint. For uncalibrated cameras, the [[Fundamental Matrix]] is used instead, mapping a point in one image to an epipolar line in the other.

### Factorization
[[Factorization]] allows for the simultaneous recovery of structure and motion from extended feature tracks, typically using Singular Value Decomposition (SVD) under orthographic or weak-perspective approximations.

### Bundle Adjustment
[[Bundle Adjustment]] is the most accurate method for refining structure and motion. It involves the non-linear minimization of reprojection errors—the distance between the observed 2D feature and the projected 3D point—across all cameras and points.

## Calibration and Constraints

### Self-Calibration
[[Self-Calibration]] (or auto-calibration) is the process of converting a projective reconstruction into a metric one by recovering unknown camera intrinsic parameters from the image correspondences themselves.

### Geometric Constraints
Reconstructions must satisfy [[Chirality]], meaning the reconstructed 3D points must lie in front of all cameras. Failure to resolve this can lead to depth reversals.

## Ambiguities and Uncertainty

### Common Ambiguities
- [[Bas-Relief Ambiguity]]: An uncertainty in the relative depth of the scene and the extent of camera motion.
- [[Gauge Ambiguity]]: A 7-degree-of-freedom indeterminacy regarding the global scale, position, and orientation of the reconstruction.

## Applications

Structure from Motion is fundamental to [[Simultaneous Localization and Mapping]] (SLAM), augmented reality, and the reconstruction of 3D models from large-scale unstructured image collections (e.g., Internet photos).

## Relationships
- Structure from Motion uses [[Triangulation]].
- Structure from Motion depends on [[Epipolar Geometry]].
- Structure from Motion is refined by [[Bundle Adjustment]].
- Structure from Motion can be solved via [[Factorization]].
- Epipolar Geometry is encoded by the [[Essential Matrix]].
- Epipolar Geometry is encoded by the [[Fundamental Matrix]] in uncalibrated cases.
- Bundle Adjustment iteratively optimizes [[Triangulation]].
- Self-Calibration recovers intrinsics from the [[Fundamental Matrix]].
- Structure from Motion must satisfy [[Chirality]].
- Structure from Motion is subject to [[Bas-Relief Ambiguity]].
- Structure from Motion is subject to [[Gauge Ambiguity]].
- Structure from Motion is closely related to [[Simultaneous Localization and Mapping]].
