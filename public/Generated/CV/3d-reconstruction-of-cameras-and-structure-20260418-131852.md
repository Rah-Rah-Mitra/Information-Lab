---
type: content
title: "3D Reconstruction of Cameras and Structure"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:18:52.510123300+00:00
summary: "An overview of the process of reconstructing 3D scene structure and camera poses from two uncalibrated images, progressing from projective to metric reconstruction."
tags:
  - computer-vision
  - geometry
  - 3d-reconstruction
  - epipolar-geometry
entities:
  - "[[Projective Reconstruction]]"
  - "[[Affine Reconstruction]]"
  - "[[Metric Reconstruction]]"
  - "[[Fundamental Matrix]]"
  - "[[Absolute Conic]]"
  - "[[Plane at Infinity]]"
  - "[[Similarity Transformation]]"
  - "[[Projective Transformation]]"
  - "[[Affine Transformation]]"
  - "[[Image of the Absolute Conic]]"
  - "[[Ground Control Points]]"
  - "[[8-Point Algorithm]]"
  - "[[Sampson Distance]]"
  - "[[Gold Standard Algorithm]]"
relationships:
  - source: "Projective Reconstruction"
    target: "Projective Transformation"
    description: "is determined up to a"
  - source: "Projective Reconstruction"
    target: "Fundamental Matrix"
    description: "is computed using the"
  - source: "Affine Reconstruction"
    target: "Plane at Infinity"
    description: "requires identification of the"
  - source: "Affine Reconstruction"
    target: "Affine Transformation"
    description: "is determined up to an"
  - source: "Metric Reconstruction"
    target: "Absolute Conic"
    description: "requires identification of the"
  - source: "Metric Reconstruction"
    target: "Similarity Transformation"
    description: "is determined up to a"
  - source: "Metric Reconstruction"
    target: "Metric Reconstruction"
    description: "generalizes"
  - source: "Image of the Absolute Conic"
    target: "Absolute Conic"
    description: "is the projection of the"
  - source: "Ground Control Points"
    target: "Metric Reconstruction"
    description: "allows direct computation of"
  - source: "8-Point Algorithm"
    target: "Fundamental Matrix"
    description: "estimates the"
  - source: "Sampson Distance"
    target: "Fundamental Matrix"
    description: "provides a first-order approximation for estimating the"
  - source: "Gold Standard Algorithm"
    target: "Fundamental Matrix"
    description: "computes the Maximum Likelihood estimate of the"
---

# 3D Reconstruction of Cameras and Structure

*An overview of the process of reconstructing 3D scene structure and camera poses from two uncalibrated images, progressing from projective to metric reconstruction.*

3D reconstruction is the process of recovering the 3D structure of a scene and the poses of the cameras that captured it from a set of 2D images.

## Reconstruction Ambiguities

Depending on the available information about camera calibration and scene geometry, reconstruction is possible up to different levels of ambiguity:

- **[[Projective Reconstruction]]**: When cameras are uncalibrated, the reconstruction is determined up to a [[Projective Transformation]]. This is the most basic form of reconstruction and can be achieved using only point correspondences and the [[Fundamental Matrix]].
- **[[Affine Reconstruction]]**: This occurs when the [[Plane at Infinity]] is identified. The resulting reconstruction is determined up to an [[Affine Transformation]].
- **[[Metric Reconstruction]]**: Also known as Euclidean reconstruction, this is achieved by identifying the [[Absolute Conic]]. It is determined up to a [[Similarity Transformation]], meaning metric properties like angles and length ratios are preserved.

## Stratified Reconstruction Process

In a stratified approach, a reconstruction is refined progressively:

1. **Projective to Affine**: The [[Plane at Infinity]] is located (e.g., via parallel scene lines or translational motion), and a homography is applied to map it to the canonical plane at infinity.
2. **Affine to Metric**: The [[Absolute Conic]] is identified, often by analyzing the [[Image of the Absolute Conic]] (IAC) in one or more views. This allows the transformation of the affine reconstruction into a metric one.

## Direct Metric Reconstruction

Alternatively, [[Metric Reconstruction]] can be achieved directly if [[Ground Control Points]] (points with known 3D Euclidean coordinates) are provided. A homography can then be computed to map the projective reconstruction directly to the true Euclidean world frame.

## Estimating the Fundamental Matrix

The [[Fundamental Matrix]] is central to projective reconstruction. Several algorithms are used for its estimation:

- **[[8-Point Algorithm]]**: A simple linear method that solves for the matrix using at least 8 point correspondences, typically requiring normalization for stability.
- **[[Sampson Distance]]**: A first-order approximation to the geometric reprojection error used to refine the matrix estimate.
- **[[Gold Standard Algorithm]]**: A non-linear minimization process that finds the Maximum Likelihood estimate by minimizing the actual geometric distance (reprojection error).

## Relationships

- [[Projective Reconstruction]] is determined up to a [[Projective Transformation]].
- [[Projective Reconstruction]] is computed using the [[Fundamental Matrix]].
- [[Affine Reconstruction]] requires identification of the [[Plane at Infinity]].
- [[Affine Reconstruction]] is determined up to an [[Affine Transformation]].
- [[Metric Reconstruction]] requires identification of the [[Absolute Conic]].
- [[Metric Reconstruction]] is determined up to a [[Similarity Transformation]].
- [[Image of the Absolute Conic]] is the projection of the [[Absolute Conic]].
- [[Ground Control Points]] allows direct computation of [[Metric Reconstruction]].
- [[8-Point Algorithm]] estimates the [[Fundamental Matrix]].
- [[Sampson Distance]] provides a first-order approximation for estimating the [[Fundamental Matrix]].
- [[Gold Standard Algorithm]] computes the Maximum Likelihood estimate of the [[Fundamental Matrix]].
