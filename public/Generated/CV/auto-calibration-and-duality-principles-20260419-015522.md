---
type: content
title: "Auto-Calibration and Duality Principles"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:55:22.898813+00:00
summary: "Auto-calibration is the process of determining camera internal parameters from image sequences without prior calibration. This note covers stratified auto-calibration, specialized motions like rotating cameras and planar motion, and the Carlsson-Weinshall duality principle for dualizing reconstruction algorithms. These methods allow for metric reconstruction from projective structures."
tags:
  - computer-vision
  - auto-calibration
  - projective-geometry
  - camera-calibration
  - duality
entities:
  - "[[Auto-Calibration]]"
  - "[[Metric Reconstruction]]"
  - "[[Projective Reconstruction]]"
  - "[[Infinite Homography]]"
  - "[[Carlsson-Weinshall Duality]]"
  - "[[Reduced Fundamental Matrix]]"
  - "[[Reduced Trifocal Tensor]]"
  - "[[Planar Motion]]"
  - "[[Rotating Camera]]"
  - "[[Plane at Infinity]]"
relationships:
  - source: "Auto-Calibration"
    target: "Metric Reconstruction"
    description: "enables"
  - source: "Auto-Calibration"
    target: "Projective Reconstruction"
    description: "refines"
  - source: "Infinite Homography"
    target: "Auto-Calibration"
    description: "provides constraints for"
  - source: "Carlsson-Weinshall Duality"
    target: "Auto-Calibration"
    description: "generalises"
  - source: "Planar Motion"
    target: "Auto-Calibration"
    description: "is a special case of"
  - source: "Rotating Camera"
    target: "Auto-Calibration"
    description: "is a special case of"
  - source: "Reduced Fundamental Matrix"
    target: "Auto-Calibration"
    description: "is used in dual"
  - source: "Reduced Trifocal Tensor"
    target: "Auto-Calibration"
    description: "is used in dual"
  - source: "Plane at Infinity"
    target: "Metric Reconstruction"
    description: "is recovered via"
---

# Auto-Calibration and Duality Principles

*Auto-calibration is the process of determining camera internal parameters from image sequences without prior calibration. This note covers stratified auto-calibration, specialized motions like rotating cameras and planar motion, and the Carlsson-Weinshall duality principle for dualizing reconstruction algorithms. These methods allow for metric reconstruction from projective structures.*

This note explores the mechanisms of [[Auto-Calibration]], the process of determining camera internal parameters from image sequences. Auto-calibration is typically performed as a stratified process, moving from a projective reconstruction to a metric reconstruction.

## Concept
In [[Auto-Calibration]], one aims to recover the camera matrix $K$ from the motion of the camera. A common approach is the stratified algorithm, which uses the [[Infinite Homography]] relation to impose constraints on the internal parameters. For example, the relation is given by:

$$ H = K R K^{-1} $$

This relation allows for the estimation of the camera conic $\omega$ through the time-varying or constant internal parameters. Once the camera conic is estimated, a metric reconstruction is achieved via Cholesky decomposition of $\omega$.

## Specialized Motion Scenarios
### Rotating Cameras
When a camera rotates about its centre without translating, the calibration problem is mathematically identical to the affine-to-metric step in stratified reconstruction. The [[Infinite Homography]] can be computed directly from image pairs, and the internal parameters can be recovered using the following constraints:

$$ \omega^T H \omega = 0 $$

This is particularly useful for pan-tilt surveillance cameras or hand-held camcorders.

### Planar Motion
In [[Planar Motion]], the camera moves in a plane (e.g., a vehicle on a ground plane). This motion is a provides a set of fixed image points, such as the [[Apex]] and the [[Horizon Line]]. The The [[Plane at Infinity]] can be determined by identifying these fixed points, which allows for the transition from an affine to a metric reconstruction.

## Duality and Reduced Representations
[[Carlsson-Weinshall Duality]] provides a powerful framework for dualizing any projective reconstruction algorithm. It allows for the interchange of the roles of points and cameras. For instance, a reconstruction problem involving $n$ views of $m$ points can be dualized into a problem involving $m$ views of $n$ points.

This duality is often implemented using [[Reduced Fundamental Matrix]] and [[Reduced Trifocal Tensor]] representations. A [[Reduced Fundamental Matrix]] is a fundamental matrix that satisfies specific linear constraints imposed by synthetic point correspondences, allowing for minimal-case solutions (e.g., 6 points in 3 views). A [[Reduced Trifocal Tensor]] is similarly defined to be consistent with a set of four basis points, making it suitable for dualized reconstruction from 7 points in 2 views.

## Relationships
- [[Auto-Calibration]] enables [[Metric Reconstruction]]
- [[Auto-Calibration]] refines [[Projective Reconstruction]]
- [[Infinite Homography]] provides constraints for [[Auto-Calibration]]
- [[Carlsson-Weinshall Duality]] generalises [[Auto-Calibration]]
- [[Planar Motion]] is a special case of [[Auto-Calibration]]
- [[Rotating Camera]] is aIs a special case of [[Auto-Calibration]]
- [[Reduced Fundamental Matrix]] is used in dual [[Auto-Calibration]]
- [[Reduced Trifocal Tensor]] is used in dual [[Auto-Calibration]]
- [[Plane at Infinity]] is recovered via [[Auto-Calibration]]
