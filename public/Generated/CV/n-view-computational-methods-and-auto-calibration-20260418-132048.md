---
type: content
title: "N-View Computational Methods and Auto-Calibration"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:20:48.074043200+00:00
summary: "An overview of computational techniques for projective and affine reconstruction from multiple views, including factorization algorithms and auto-calibration."
tags:
  - computer-vision
  - projective-geometry
  - multi-view-geometry
  - reconstruction
entities:
  - "[[Bundle Adjustment]]"
  - "[[Projective Reconstruction]]"
  - "[[Affine Reconstruction]]"
  - "[[Factorization Algorithm]]"
  - "[[Non-Rigid Factorization]]"
  - "[[Auto-Calibration]]"
  - "[[Absolute Dual Quadric]]"
  - "[[Dual Image of the Absolute Conic]]"
  - "[[Image of the Absolute Conic]]"
  - "[[Rectifying Homography]]"
  - "[[Trifocal Tensor]]"
  - "[[Quadrifocal Tensor]]"
relationships:
  - source: "Bundle Adjustment"
    target: "Projective Reconstruction"
    description: "is used as a final step to refine"
  - source: "Factorization Algorithm"
    target: "Affine Reconstruction"
    description: "provides an optimal method for computing"
  - source: "Non-Rigid Factorization"
    target: "Factorization Algorithm"
    description: "is a modification of"
  - source: "Auto-Calibration"
    target: "Projective Reconstruction"
    description: "transforms into a metric reconstruction"
  - source: "Auto-Calibration"
    target: "Rectifying Homography"
    description: "determines the"
  - source: "Absolute Dual Quadric"
    target: "Auto-Calibration"
    description: "is used as a framework for"
  - source: "Absolute Dual Quadric"
    target: "Dual Image of the Absolute Conic"
    description: "projects to the"
  - source: "Dual Image of the Absolute Conic"
    target: "Auto-Calibration"
    description: "provides linear constraints for"
  - source: "Image of the Absolute Conic"
    target: "Auto-Calibration"
    description: "provides constraints for"
  - source: "Trifocal Tensor"
    target: "Projective Reconstruction"
    description: "can be used to initialize"
  - source: "Quadrifocal Tensor"
    target: "Projective Reconstruction"
    description: "can be used to initialize"
---

# N-View Computational Methods and Auto-Calibration

*An overview of computational techniques for projective and affine reconstruction from multiple views, including factorization algorithms and auto-calibration.*

Computational methods for N-view reconstruction focus on estimating the 3D structure and camera parameters from multiple image correspondences.

## Projective and Affine Reconstruction

[[Projective Reconstruction]] is the most general form of recovery where the result is determined up to a 3D projective transformation. [[Bundle Adjustment]] is the standard non-linear optimization technique used to minimize reprojection error, though it requires a good initialization and can be computationally expensive for large datasets.

[[Affine Reconstruction]] is a specialized case where the reconstruction is determined up to an affine transformation. The [[Factorization Algorithm]] (Tomasi-Kanade) provides a Maximum Likelihood estimate for affine reconstruction by decomposing a measurement matrix using Singular Value Decomposition (SVD) to separate motion and structure.

## Non-Rigid Reconstruction

[[Non-Rigid Factorization]] extends the factorization approach to deforming objects by modeling the scene as a linear combination of basis shapes. This allows the recovery of both the motion and the deformation basis, though recovering the specific camera motion is more complex than in the rigid case.

## Auto-Calibration

[[Auto-Calibration]] is the process of determining internal camera parameters from uncalibrated images to upgrade a projective reconstruction to a metric one. This is achieved by finding a [[Rectifying Homography]] that transforms the projective cameras into a metric form.

### The Absolute Dual Quadric

A primary framework for auto-calibration is the use of the [[Absolute Dual Quadric]], a degenerate dual quadric that encodes the plane at infinity and the absolute conic. The [[Absolute Dual Quadric]] projects to the [[Dual Image of the Absolute Conic]] (DIAC) in each view. By imposing constraints on the internal parameters (such as known principal point or zero skew), linear or non-linear equations can be derived to estimate the quadric and subsequently the rectifying transformation.

## Multi-View Tensors

Constraints from [[Trifocal Tensor]] and [[Quadrifocal Tensor]] relationships are used to handle point and line correspondences across three and four views, respectively. These tensors provide the closed-form mathematical foundations for initializing reconstructions before refining them with bundle adjustment.

## Relationships

- Bundle Adjustment is used as a final step to refine Projective Reconstruction.
- Factorization Algorithm provides an optimal method for computing Affine Reconstruction.
- Non-Rigid Factorization is a modification of Factorization Algorithm.
- Auto-Calibration transforms into a metric reconstruction Projective Reconstruction.
- Auto-Calibration determines the Rectifying Homography.
- Absolute Dual Quadric is used as a framework for Auto-Calibration.
- Absolute Dual Quadric projects to the Dual Image of the Absolute Conic.
- Dual Image of the Absolute Conic provides linear constraints for Auto-Calibration.
- Image of the Absolute Conic provides constraints for Auto-Calibration.
- Trifocal Tensor can be used to initialize Projective Reconstruction.
- Quadrifocal Tensor can be used to initialize Projective Reconstruction.
