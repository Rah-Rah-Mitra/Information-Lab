---
type: content
title: "Image Alignment and Stitching Techniques"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:33:39.696400600+00:00
summary: "Image alignment and stitching involve estimating geometric transformations to map features between images for seamless compositing. These techniques range from simple 2D transformations to complex spherically projected panoramas. They are essential for digital maps, satellite imagery, and smartphone camera features."
tags:
  - computer-vision
  - image-stitching
  - image-registration
  - geometric-transformation
  - panoramas
entities:
  - "[[Image Alignment]]"
  - "[[Image Stitching]]"
  - "[[Homography]]"
  - "[[Least Squares]]"
  - "[[RANSAC]]"
  - "[[Mean Shift]]"
  - "[[Panoramas]]"
  - "[[Cylindrical Coordinates]]"
  - "[[Spherical Coordinates]]"
  - "[[Non-linear Least Squares]]"
  - "[[3D Alignment]]"
  - "[[Parametric Motion Models]]"
relationships:
  - source: "Image Alignment"
    target: "Image Stitching"
    description: "is a prerequisite for"
  - source: "Image Stitching"
    target: "Homography"
    description: "uses"
  - source: "Image Alignment"
    target: "Least Squares"
    description: "utilizes"
  - source: "Image Alignment"
    target: "RANSAC"
    description: "requires robustness via"
  - source: "Image Alignment"
    target: "Non-linear Least Squares"
    description: "uses for complex motion"
  - source: "Image Stitching"
    target: "Panoramas"
    description: "produces"
  - source: "Image Stitching"
    target: "Cylindrical Coordinates"
    description: "can use"
  - source: "Image Stitching"
    target: "Spherical Coordinates"
    description: "can use"
  - source: "Image Stitching"
    target: "Parametric Motion Models"
    description: "relies on"
  - source: "Image Alignment"
    target: "Mean Shift"
    description: "can be used for segmentation-based alignment"
  - source: "Image Alignment"
    target: "3D Alignment"
    description: "generalises to"
---

# Image Alignment and Stitching Techniques

*Image alignment and stitching involve estimating geometric transformations to map features between images for seamless compositing. These techniques range from simple 2D transformations to complex spherically projected panoramas. They are essential for digital maps, satellite imagery, and smartphone camera features.*

This note explores the methodologies for aligning and stitching multiple images into a single, coherent view.

## Concept
[[Image Alignment]] is the process of estimating the motion between sets of matched 2D or 3D points to ensure geometric consistency. This is often achieved through [[Parametric Motion Models]], which range from simple 2D translations to complex [[Homography]] matrices. For many applications, [[Least Squares]] is the standard method for parameter estimation, minimizing the squared residuals between sensed and predicted feature locations.

In cases where the outliers are present, [[RANSAC]] (RANdom SAmple Consensus) is employed to find a robust inlier set. For non-linear relationships, such as rigid rotation, [[Non-linear Least Squares]] is used, often via iterative methods like the Levenberg–Marquardt algorithm.

[[Image Stitching]] is the application of these alignment techniques to create [[Panoramas]]. Depending on the camera motion, different projection surfaces are used. For example, [[Cylindrical Coordinates]] are suitable for leveled cameras rotating around a vertical axis, while [[Spherical Coordinates]] are used for full-sphere or hemispherical views.

## Mathematical Formulations

### Homography
For a planar perspective motion, the mapping between pixels is described by a [[Homography]] matrix $\mathbf{H}$:

$$ \mathbf{x}' = \mathbf{H} \mathbf{x} $$

This matrix describes the mapping between pixels in two images viewing a common plane.

### Least Squares Residuals
The objective is to minimize the sum of squared residuals:

$$ \sum_{i} \left\| \mathbf{x}_i - \mathbf{f}(\mathbf{x}_i, \mathbf{p}) \right\|^2 $$
