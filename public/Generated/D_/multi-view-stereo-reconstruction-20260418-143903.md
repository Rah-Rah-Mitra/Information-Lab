---
type: content
title: "Multi-View Stereo Reconstruction"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:39:03.109777200+00:00
summary: "An overview of multi-view stereo (MVS) techniques for reconstructing detailed 3D geometry from multiple photographs using photo-consistency and SfM."
tags:
  - computer-vision
  - 3d-reconstruction
  - photogrammetry
  - mvs
entities:
  - "[[Multi-View Stereo]]"
  - "[[Structure from Motion]]"
  - "[[Photo-consistency]]"
  - "[[Bundle Adjustment]]"
  - "[[Pinhole Camera Model]]"
  - "[[Epipolar Geometry]]"
  - "[[Normalized Cross Correlation]]"
  - "[[Sum of Squared Differences]]"
  - "[[Sum of Absolute Differences]]"
  - "[[Census Transform]]"
  - "[[Mutual Information]]"
  - "[[Markov Random Field]]"
  - "[[Depthmap Reconstruction]]"
relationships:
  - source: "Multi-View Stereo"
    target: "Photo-consistency"
    description: "optimizes"
  - source: "Multi-View Stereo"
    target: "Structure from Motion"
    description: "depends on for camera parameters"
  - source: "Structure from Motion"
    target: "Bundle Adjustment"
    description: "uses for refinement"
  - source: "Multi-View Stereo"
    target: "Epipolar Geometry"
    description: "utilizes to simplify matching to 1D search"
  - source: "Multi-View Stereo"
    target: "Pinhole Camera Model"
    description: "requires for projection"
  - source: "Photo-consistency"
    target: "Normalized Cross Correlation"
    description: "is measured by"
  - source: "Photo-consistency"
    target: "Sum of Squared Differences"
    description: "is measured by"
  - source: "Photo-consistency"
    target: "Sum of Absolute Differences"
    description: "is measured by"
  - source: "Photo-consistency"
    target: "Census Transform"
    description: "is measured by"
  - source: "Photo-consistency"
    target: "Mutual Information"
    description: "is measured by"
  - source: "Depthmap Reconstruction"
    target: "Photo-consistency"
    description: "uses to estimate depth"
  - source: "Depthmap Reconstruction"
    target: "Markov Random Field"
    description: "uses for spatial consistency"
  - source: "Multi-View Stereo"
    target: "Depthmap Reconstruction"
    description: "implements via"
---

# Multi-View Stereo Reconstruction

*An overview of multi-view stereo (MVS) techniques for reconstructing detailed 3D geometry from multiple photographs using photo-consistency and SfM.*

[[Multi-View Stereo]] (MVS) is a group of computer vision techniques used to reconstruct highly detailed 3D geometry from a set of photographs by utilizing stereo correspondence across more than two images.

## Overview of the MVS Pipeline

An MVS pipeline typically consists of three main stages: imagery collection, estimation of camera parameters, and 3D geometry reconstruction. A critical prerequisite for MVS is the accurate estimation of camera parameters, which is usually provided by [[Structure from Motion]] (SfM) algorithms. SfM computes the camera pose (location and orientation) and intrinsic properties (such as focal length) using a [[Pinhole Camera Model]]. To refine these parameters and minimize reprojection error, [[Bundle Adjustment]] is employed.

## Photo-consistency

At the core of MVS is the concept of [[Photo-consistency]], which measures the agreement between different photographs of the same 3D point. This is framed as an optimization problem where photo-consistency is maximized as a function of geometry, viewpoints, and materials.

### Similarity Measures

Different measures are used to compute photo-consistency, each with different trade-offs regarding invariance to lighting and material changes:

- [[Normalized Cross Correlation]]: Highly accurate and invariant to gain and bias changes, but fails on textureless or repetitive surfaces.
- [[Sum of Squared Differences]]: Optimal if noise is additive Gaussian, but sensitive to outliers.
- [[Sum of Absolute Differences]]: More robust to outliers than SSD, often used in real-time applications.
- [[Census Transform]]: Invariant to gain and bias; more robust than NCC around depth boundaries.
- [[Mutual Information]]: Highly invariant to complex bijective mappings, often used in multi-modal imagery, though at the cost of accuracy.

## Geometry Reconstruction

Matching pixels across images is simplified from a 2D search to a 1D search using [[Epipolar Geometry]], which restricts the search for corresponding pixels to a line.

### Depthmap Reconstruction

[[Depthmap Reconstruction]] is a popular approach due to its scalability. It involves estimating a depth value for each pixel in a reference image. Simple strategies include "Winner-Takes-All," where the depth with the highest photo-consistency score is chosen. To handle noise and occlusions, more robust functions or [[Markov Random Field]] (MRF) formulations are used. MRF enforces spatial consistency by assuming neighboring pixels have similar depth values, treating the reconstruction as a combinatorial optimization problem.

## Relationships

- [[Multi-View Stereo]] optimizes [[Photo-consistency]].
- [[Multi-View Stereo]] depends on [[Structure from Motion]] for camera parameters.
- [[Structure from Motion]] uses [[Bundle Adjustment]] for refinement.
- [[Multi-View Stereo]] utilizes [[Epipolar Geometry]] to simplify matching to 1D search.
- [[Multi-View Stereo]] requires [[Pinhole Camera Model]] for projection.
- [[Photo-consistency]] is measured by [[Normalized Cross Correlation]], [[Sum of Squared Differences]], [[Sum of Absolute Differences]], [[Census Transform]], and [[Mutual Information]].
- [[Depthmap Reconstruction]] uses [[Photo-consistency]] to estimate depth.
- [[Depthmap Reconstruction]] uses [[Markov Random Field]] for spatial consistency.
- [[Multi-View Stereo]] implements via [[Depthmap Reconstruction]].
