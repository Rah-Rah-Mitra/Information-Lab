---
type: content
title: "Translational and Parametric Image Alignment"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:51:56.426516100+00:00
summary: "An overview of techniques for aligning images using translational shifts, parametric motion models, and optical flow, including robust error metrics and hierarchical search."
tags:
  - computer-vision
  - image-registration
  - motion-estimation
  - optical-flow
entities:
  - "[[Translational Alignment]]"
  - "[[Sum of Squared Differences]]"
  - "[[Sum of Absolute Differences]]"
  - "[[Normalized Cross-Correlation]]"
  - "[[Hierarchical Motion Estimation]]"
  - "[[Fourier-Based Alignment]]"
  - "[[Phase Correlation]]"
  - "[[Lucas-Kanade Algorithm]]"
  - "[[Aperture Problem]]"
  - "[[Parametric Motion]]"
  - "[[Inverse Compositional Algorithm]]"
  - "[[Spline-Based Motion]]"
  - "[[Optical Flow]]"
relationships:
  - source: "Translational Alignment"
    target: "Sum of Squared Differences"
    description: "often uses as a least squares solution"
  - source: "Translational Alignment"
    target: "Sum of Absolute Differences"
    description: "uses as a robust error metric"
  - source: "Translational Alignment"
    target: "Normalized Cross-Correlation"
    description: "uses to handle relative scales and offsets"
  - source: "Hierarchical Motion Estimation"
    target: "Translational Alignment"
    description: "accelerates the search process for"
  - source: "Fourier-Based Alignment"
    target: "Translational Alignment"
    description: "provides an alternative approach for"
  - source: "Phase Correlation"
    target: "Fourier-Based Alignment"
    description: "is a variant of"
  - source: "Lucas-Kanade Algorithm"
    target: "Translational Alignment"
    description: "provides an incremental refinement step for"
  - source: "Lucas-Kanade Algorithm"
    target: "Aperture Problem"
    description: "can be poorly conditioned due to"
  - source: "Parametric Motion"
    target: "Lucas-Kanade Algorithm"
    description: "generalizes the incremental update rule of"
  - source: "Inverse Compositional Algorithm"
    target: "Parametric Motion"
    description: "is an efficient implementation for estimating"
  - source: "Spline-Based Motion"
    target: "Parametric Motion"
    description: "is a hybrid between general optical flow and"
  - source: "Optical Flow"
    target: "Translational Alignment"
    description: "is the most general version of"
  - source: "Optical Flow"
    target: "Lucas-Kanade Algorithm"
    description: "often employs for sub-pixel estimates"
---

# Translational and Parametric Image Alignment

*An overview of techniques for aligning images using translational shifts, parametric motion models, and optical flow, including robust error metrics and hierarchical search.*

[[Translational Alignment]] is the process of establishing a correspondence between two images by shifting one relative to the other to minimize a cost function.

## Error Metrics for Alignment

To find the optimal displacement, various error metrics are used to measure the difference between a template image and a target image:

- [[Sum of Squared Differences]] (SSD): A least squares solution that is optimal under Gaussian noise but sensitive to outliers.
- [[Sum of Absolute Differences]] (SAD): An L1 norm that is more robust to outliers and faster to compute, though not differentiable at the origin.
- [[Normalized Cross-Correlation]] (NCC): A metric that maximizes the product of aligned images, normalized to be invariant to linear bias and gain (exposure) differences.

## Search and Optimization Strategies

### Hierarchical Motion Estimation
[[Hierarchical Motion Estimation]] uses an image pyramid to accelerate the search. A coarse-to-fine strategy is employed where the best displacement is first found at a low resolution and then used to initialize a local search at finer levels.

### Fourier-Based Alignment
[[Fourier-Based Alignment]] leverages the property that a shift in the spatial domain corresponds to a linear phase shift in the frequency domain. This allows the use of the Fast Fourier Transform (FFT) to compute correlations more efficiently than a full spatial search. [[Phase Correlation]] is a specific variant that whitens the spectrum to produce a single impulse at the correct displacement.

### Incremental Refinement
For sub-pixel accuracy, the [[Lucas-Kanade Algorithm]] performs gradient descent on the energy function using a Taylor series expansion of the image. This approach solves a system of normal equations to iteratively refine the displacement.

## Advanced Motion Models

### Parametric Motion
[[Parametric Motion]] extends alignment beyond simple translation to models like affine or projective transforms. The [[Inverse Compositional Algorithm]] is a preferred implementation for these models because it allows the pre-computation of the Hessian matrix and steepest descent images, significantly reducing the per-iteration cost.

### Spline-Based Motion
[[Spline-Based Motion]] represents the motion field as a 2D spline controlled by a set of control vertices. This provides a middle ground between the rigid constraints of parametric models and the high dimensionality of per-pixel flow.

### Optical Flow
[[Optical Flow]] is the most general form of motion estimation, computing an independent motion vector for every pixel. Because this is an underconstrained problem, it is typically solved using either local patch-based methods (like Lucas-Kanade) or global regularization methods (such as the Horn-Schunck model).

## Challenges in Alignment

A primary challenge in local motion estimation is the [[Aperture Problem]], where the lack of texture in a patch (e.g., a single edge) makes the motion component along the edge unrecoverable, resulting in a rank-deficient Hessian matrix.

## Relationships
- [[Translational Alignment]] often uses [[Sum of Squared Differences]] as a least squares solution.
- [[Translational Alignment]] uses [[Sum of Absolute Differences]] as a robust error metric.
- [[Translational Alignment]] uses [[Normalized Cross-Correlation]] to handle relative scales and offsets.
- [[Hierarchical Motion Estimation]] accelerates the search process for [[Translational Alignment]].
- [[Fourier-Based Alignment]] provides an alternative approach for [[Translational Alignment]].
- [[Phase Correlation]] is a variant of [[Fourier-Based Alignment]].
- [[Lucas-Kanade Algorithm]] provides an incremental refinement step for [[Translational Alignment]].
- [[Lucas-Kanade Algorithm]] can be poorly conditioned due to the [[Aperture Problem]].
- [[Parametric Motion]] generalizes the incremental update rule of [[Lucas-Kanade Algorithm]].
- [[Inverse Compositional Algorithm]] is an efficient implementation for estimating [[Parametric Motion]].
- [[Spline-Based Motion]] is a hybrid between general [[Optical Flow]] and [[Parametric Motion]].
- [[Optical Flow]] is the most general version of [[Translational Alignment]].
- [[Optical Flow]] often employs [[Lucas-Kanade Algorithm]] for sub-pixel estimates.
