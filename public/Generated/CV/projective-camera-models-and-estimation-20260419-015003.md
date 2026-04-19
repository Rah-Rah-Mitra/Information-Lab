---
type: content
title: "Projective Camera Models and Estimation"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:50:03.976436300+00:00
summary: "Projective camera models describe the mathematical mapping from 3D world space to 2D image space using a projection matrix. This note covers the taxonomy of camera models, from pinhole to affine, and the numerical methods for estimating these matrices from point correspondences. It also addresses the necessary corrections for radial distortion."
tags:
  - computer-vision
  - camera-calibration
  - projective-geometry
  - matrix-decomposition
entities:
  - "[[Projective Camera]]"
  - "[[Camera Matrix]]"
  - "[[Pinhole Camera Model]]"
  - "[[Camera Calibration Matrix]]"
  - "[[Camera Calibration]]"
  - "[[Camera Calibration Matrix]]"
  - "[[Camera Matrix]]"
  - "[[Affine Camera]]"
  - "[[Radial Distortion]]"
  - "[[Direct Linear Transformation]]"
relationships:
  - source: "Projective Camera"
    target: "Camera Matrix"
    description: "is represented by"
  - source: "Camera Matrix"
    target: "Pinhole Camera Model"
    description: "specializes to"
  - source: "Camera Matrix"
    target: "Affine Camera"
    description: "specializes to"
  - source: "Camera Matrix"
    target: "Camera Calibration Matrix"
    description: "decomposes into"
  - source: "Camera Matrix"
    target: "Radial Distortion"
    description: "is corrected for"
  - source: "Camera Matrix"
    target: "Direct Linear Transformation"
    description: "is estimated via"
---

# Projective Camera Models and Estimation

*Projective camera models describe the mathematical mapping from 3D world space to 2D image space using a projection matrix. This note covers the taxonomy of camera models, from pinhole to affine, and the numerical methods for estimating these matrices from point correspondences. It also addresses the necessary corrections for radial distortion.*

This note explores the mathematical framework of [[Projective Camera]] models, which map 3D world points to 2D image points through a projection matrix. 

## Concept
A [[Projective Camera]] is defined by a $3 \times 4$ matrix $P$ that maps homogeneous 3-space coordinates $X$ to homogeneous 2-space coordinates $x$ via the linear mapping $x = PX$. 

### Finite Projective Cameras
For a [[Pinhole Camera Model]], the matrix $P$ can be decomposed into internal and external parameters. The [[Camera Calibration Matrix]] $K$ (or internal orientation) contains parameters like focal length, principal point, and skew. The external orientation is defined by a rotation matrix $R$ and a translation vector $C$, such that $P = K[R | t]$ where $t = -RC$. 

$$ P = K [R | t] $$

This decomposition allows for the separation of intrinsic properties (focal length, aspect ratio) from extrinsic properties (camera position and orientation in the world).

### Affine Cameras
An [[Affine Camera]] is a specialization where the camera centre lies on the plane at infinity. This is characterized by a last row of $(0, 0, 0, 1)$ in the matrix $P$, or more generally, a singular $3 \times 3$ submatrix $M$. Affine cameras are useful for modeling parallel projection, where parallel world lines are mapped to parallel image lines.

### Estimation and Calibration
[[Camera Calibration]] is the process of determining the matrix $P$ or its components. The [[Direct Linear Transformation]] (DLT) algorithm is a standard method for estimating $P$ from a set of point correspondences. For a more accurate result, the "Gold Standard" algorithm minimizes geometric error using iterative techniques like Levenberg-Marquardt.

$$ \min_{P} \sum_{i} d(x_i, PX_i)^2 $$

### Radial Distortion
Real-world lenses often deviate from the ideal pinhole model due to [[Radial Distortion]], which causes straight lines to appear curved. This is modeled by a distortion factor $L(r)$ that acts on the radial distance $r$ from the principal point. Correcting this distortion is a prerequisite for accurate camera calibration.

## Relationships
- [[Projective Camera]] is represented by [[Camera Matrix]]
- [[Camera Matrix]] specializes to [[Pinhole Camera Model]]
- [[Camera Matrix]] specializes to [[Affine Camera]]
- [[Camera Matrix]] decomposes into [[Camera Calibration Matrix]]
- [[Camera Matrix]] is corrected for [[Radial Distortion]]
- [[Camera Matrix]] is estimated via [[Direct Linear Transformation]]
