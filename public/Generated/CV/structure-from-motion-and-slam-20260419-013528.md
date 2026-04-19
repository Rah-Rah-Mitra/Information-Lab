---
type: content
title: "Structure From Motion And SLAM"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:35:28.929094400+00:00
summary: "Structure from motion (SfM) is the simultaneous recovery of 3D structure and camera pose from image correspondences. It is used for large-scale 3D reconstruction, augmented reality, and autonomous navigation. The field is evolving from sparse, two-frame techniques to robust, multi-frame bundle adjustment and real-time SLAM."
tags:
  - computer-vision
  - 3d-reconstruction
  - slam
  - structure-from-motion
  - optimization
entities:
  - "[[Structure From Motion]]"
  - "[[Simultaneous Localization And Mapping]]"
  - "[[Bundle Adjustment]]"
  - "[[Essential Matrix]]"
  - "[[Fundamental Matrix]]"
  - "[[Triangulation]]"
  - "[[Epipolar Geometry]]"
  - "[[Cheirality]]"
  - "[[Pose Estimation]]"
  - "[[Camera Calibration]]"
relationships:
  - source: "Structure From Motion"
    target: "Bundle Adjustment"
    description: "is optimized via"
  - source: "Structure From Motion"
    target: "Essential Matrix"
    description: "uses"
  - source: "Structure From Motion"
    target: "Fundamental Matrix"
    description: "uses"
  - source: "Structure From Motion"
    target: "Triangulation"
    description: "requires"
  - source: "Structure From Motion"
    target: "Simultaneous Localization And Mapping"
    description: "is related to"
  - source: "Simultaneous Localization And Mapping"
    target: "Bundle Adjustment"
    description: "incorporates"
  - source: "Essential Matrix"
    target: "Epipolar Geometry"
    description: "encodes"
  - source: "Structure From Motion"
    target: "Pose Estimation"
    description: "involves"
  - source: "Structure From Motion"
    target: "Camera Calibration"
    description: "requires"
  - source: "Structure From Motion"
    target: "Cheirality"
    description: "must satisfy"
---

# Structure From Motion And SLAM

*Structure from motion (SfM) is the simultaneous recovery of 3D structure and camera pose from image correspondences. It is used for large-scale 3D reconstruction, augmented reality, and autonomous navigation. The field is evolving from sparse, two-frame techniques to robust, multi-frame bundle adjustment and real-time SLAM.*

[[Structure From Motion]] is the process of simultaneously recovering 3D structure and camera motion from a set of image correspondences. It serves as the foundation for many applications, including 3D modeling, augmented reality, and autonomous navigation.

## Concept

### Triangulation
To reconstruct 3D points, one must perform [[Triangulation]], which determines a point's 3D position from corresponding 2D image locations and known camera poses. A common approach is to find the 3D point that minimizes the distance to all optical rays originating from the camera centers.

$$ \min_{p} \sum_{k} || p - q_{c,k} ||^2 $$

The above formula represents the minimization of squared distances to rays for a point $p$.

### Epipolar Geometry and the Essential Matrix
In two-frame reconstruction, the relationship between views is governed by [[Epipolar Geometry]]. For calibrated cameras, this is captured by the [[Essential Matrix]] $E$, which relates a 2D point in one image to an epipolar line in the second.

$$ E = [t]_{	imes} R $$

This equation defines the essential matrix as the cross product of the translation vector $t$ and the rotation matrix $R$.

For uncalibrated cameras, the relationship is described by the [[Fundamental Matrix]] $F$, which maps image coordinates directly without needing the intrinsic calibration matrices $K$.

$$ F = K^{-T} E K^{-1} $$

This formula relates the fundamental matrix to the essential matrix via the camera intrinsics.

### Optimization and Bundle Adjustment
For multi-frame reconstruction, the la most accurate method is [[Bundle Adjustment]], which performs a robust non-linear minimization of reprojection errors across all cameras and points.

$$ \min_{p, R, t, K} \sum_{i,j} || x_{ij} - f(p_i, R_j, t_j, K_j) ||^2 $$

This represents the minimization of the reprojection error for points $p_i$ seen by cameras $j$.

[[Simultaneous Localization And Mapping]] (SLAM) is a closely related field that focuses on real-time, online estimation of both robot pose and map structure, often used in autonomous navigation and mobile robotics.

### Constraints and Ambiguities
Reconstructions must satisfy [[Cheirality]], the requirement that reconstructed points lie in front of the cameras. Additionally, [[Pose Estimation]] often requires [[Camera Calibration]] to resolve scale and orientation ambiguities.

## Relationships
- [[Structure From Motion]] is optimized via [[Bundle Adjustment]]
- [[Structure From Motion]] uses [[Essential Matrix]]
- [[Structure From Motion]] uses [[Fundamental Matrix]]
- [[Structure From Motion]] requires [[Triangulation]]
- [[Structure From Motion]] is related to [[Simultaneous Localization And Mapping]]
- [[Simultaneous Localization And Mapping]] incorporates [[Bundle Adjustment]]
- [[Essential Matrix]] encodes [[Epipolar Geometry]]
- [[Structure From Motion]] involves [[Pose Estimation]]
- [[Structure From Motion]] requires [[Camera Calibration]]
- [[Structure From Motion]] must satisfy [[Cheirality]]
