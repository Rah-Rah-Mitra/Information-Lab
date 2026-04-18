---
type: content
title: "Panoramic Image Stitching and Alignment"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:52:33.463416900+00:00
summary: "An overview of motion models, global alignment techniques like bundle adjustment, and compositing methods for creating seamless panoramic images."
tags:
  - computer-vision
  - image-stitching
  - geometry
  - optimization
entities:
  - "[[Panoramic Image Stitching]]"
  - "[[Motion Models]]"
  - "[[Homography]]"
  - "[[Bundle Adjustment]]"
  - "[[RANSAC]]"
  - "[[Cylindrical Coordinates]]"
  - "[[Spherical Coordinates]]"
  - "[[3D Rotation Matrix]]"
  - "[[Laplacian Pyramid Blending]]"
  - "[[Gradient Domain Blending]]"
  - "[[Poisson Equation]]"
  - "[[Markov Random Field]]"
relationships:
  - source: "Panoramic Image Stitching"
    target: "Motion Models"
    description: "requires"
  - source: "Motion Models"
    target: "Homography"
    description: "includes"
  - source: "Panoramic Image Stitching"
    target: "Bundle Adjustment"
    description: "uses for global alignment"
  - source: "Bundle Adjustment"
    target: "3D Rotation Matrix"
    description: "optimizes"
  - source: "Homography"
    target: "RANSAC"
    description: "is often estimated using"
  - source: "Panoramic Image Stitching"
    target: "Cylindrical Coordinates"
    description: "may use for compositing"
  - source: "Panoramic Image Stitching"
    target: "Spherical Coordinates"
    description: "may use for compositing"
  - source: "Panoramic Image Stitching"
    target: "Laplacian Pyramid Blending"
    description: "uses for seamless compositing"
  - source: "Panoramic Image Stitching"
    target: "Gradient Domain Blending"
    description: "uses for seamless compositing"
  - source: "Gradient Domain Blending"
    target: "Poisson Equation"
    description: "solves"
  - source: "Panoramic Image Stitching"
    target: "Markov Random Field"
    description: "uses for optimal seam selection"
---

# Panoramic Image Stitching and Alignment

*An overview of motion models, global alignment techniques like bundle adjustment, and compositing methods for creating seamless panoramic images.*

[[Panoramic Image Stitching]] is the process of combining multiple overlapping images into a single wide-field-of-view composite. This requires establishing mathematical relationships to map pixel coordinates between images, choosing a compositing surface, and blending the results to remove visible seams.

## Motion Models
To align images, various [[Motion Models]] are used depending on the camera movement. A [[Homography]] is an 8-parameter model describing the deformation of a planar surface viewed from different positions or the result of a pure camera rotation. For simpler cases, 2D rigid transforms or affine models may suffice. When a camera undergoes pure rotation, the mapping depends only on the [[3D Rotation Matrix]] and focal lengths, making the estimation more stable than a general homography.

## Global Alignment
While pairwise alignment can be performed using [[RANSAC]] to find inliers, this often leads to accumulated error. [[Bundle Adjustment]] is used to simultaneously align all images by minimizing a global energy function, typically using non-linear least squares to refine rotation matrices and focal lengths.

## Compositing Surfaces
Depending on the field of view, images are warped onto different surfaces:
- **Flat**: A perspective projection where straight lines remain straight, suitable for small fields of view.
- **Cylindrical Coordinates**: Used for horizontally panning cameras, allowing alignment via pure translation.
- **Spherical Coordinates**: Used for full-sphere or hemisphere views.

## Blending and Seam Selection
To create a seamless result, the system must handle exposure differences and ghosting:
- **Seam Selection**: Optimal seams can be found by minimizing a [[Markov Random Field]] energy, which penalizes differences in labelings between adjacent pixels to avoid cutting through moving objects.
- **Feathering**: A simple weighted average based on distance maps.
- **[[Laplacian Pyramid Blending]]**: A frequency-adaptive approach that uses different transition widths for different frequency bands.
- **[[Gradient Domain Blending]]**: Instead of copying pixels, this method copies gradients and solves a [[Poisson Equation]] to reconstruct the image, effectively masking boundary differences.

## Relationships
- [[Panoramic Image Stitching]] requires [[Motion Models]].
- [[Motion Models]] includes [[Homography]].
- [[Panoramic Image Stitching]] uses [[Bundle Adjustment]] for global alignment.
- [[Bundle Adjustment]] optimizes [[3D Rotation Matrix]].
- [[Homography]] is often estimated using [[RANSAC]].
- [[Panoramic Image Stitching]] may use [[Cylindrical Coordinates]] or [[Spherical Coordinates]] for compositing.
- [[Panoramic Image Stitching]] uses [[Laplacian Pyramid Blending]] or [[Gradient Domain Blending]] for seamless compositing.
- [[Gradient Domain Blending]] solves [[Poisson Equation]].
- [[Panoramic Image Stitching]] uses [[Markov Random Field]] for optimal seam selection.
