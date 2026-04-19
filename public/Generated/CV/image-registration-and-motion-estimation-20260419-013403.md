---
type: content
title: "Image Registration And Motion Estimation"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:34:03.643539500+00:00
summary: "Image registration involves aligning multiple images to a consistent coordinate system through various motion models and optimization techniques. It is essential for tasks like panoramic stitching, video stabilization, and medical imaging. Techniques range from simple translational alignment to complex bundle adjustment and gradient domain blending."
tags:
  - computer-vision
  - image-registration
  - motion-estimation
  - optimization
entities:
  - "[[Image Registration]]"
  - "[[Motion Estimation]]"
  - "[[Bundle Adjustment]]"
  - "[[Translational Alignment]]"
  - "[[Optical Flow]]"
  - "[[Gradient Domain Blending]]"
  - "[[Bundle Adjustment]]"
  - "[[Panoramas]]"
  - "[[RANSAC]]"
  - "[[Lucas-Kanade Method]]"
relationships:
  - source: "Image Registration"
    target: "Bundle Adjustment"
    description: "utilizes"
  - source: "Image Registration"
    target: "Translational Alignment"
    description: "includes"
  - source: "Image Registration"
    target: "Optical Flow"
    description: "specialises to"
  - source: "Motion Estimation"
    target: "Bundle Adjustment"
    description: "requires"
  - source: "Motion Estimation"
    target: "Translational Alignment"
    description: "involves"
  - source: "Bundle Adjustment"
    target: "Panoramas"
    description: "enables"
  - source: "Bundle Adjustment"
    target: "RANSAC"
    description: "uses"
  - source: "Gradient Domain Blending"
    target: "Image Registration"
    description: "is a compositing technique for"
  - source: "Translational Alignment"
    target: "Lucas-Kanade Method"
    description: "is refined by"
  - source: "Optical Flow"
    target: "Motion Estimation"
    description: "is a form of"
---

# Image Registration And Motion Estimation

*Image registration involves aligning multiple images to a consistent coordinate system through various motion models and optimization techniques. It is essential for tasks like panoramic stitching, video stabilization, and medical imaging. Techniques range from simple translational alignment to complex bundle adjustment and gradient domain blending.*

This note explores the fundamental principles of [[Image Registration]] and [[Motion Estimation]] used to achieve visual consistency across multiple views.

## Concept
[[Image Registration]] is the process of finding a globally consistent set of alignment parameters to minimize misregistration between images. This can be achieved through various methods, depending on the complexity of the motion model.

### Global Alignment and Bundle Adjustment
For large-scale applications like creating [[Panoramas]], a simple pairwise alignment is often insufficient due to accumulated error. [[Bundle Adjustment]] is a more robust alternative that simultaneously adjusts pose parameters (such as rotation matrices and focal lengths) and 3D point locations for a large collection of overlapping images. This process minimizes a global energy function, often using a least-squares framework.

$$ E_{BA} = \sum_{i,j} \left( \hat{\mathbf{x}}_{ij} - \mathbf{x}_{ij} \right)^2 $$ 

This formula represents the error in 3D projected ray directions or point locations during bundle adjustment.

### Translational Alignment
[[Translational Alignment]] is the simplest form of motion estimation, involving shifting one image relative to another. A common metric is the Sum of Squared Differences (SSD), which assumes a brightness constancy constraint: that corresponding pixel values remain the same across images.

$$ E_{SSD} = \\sum_{i} [I(\mathbf{x} + \mathbf{u}) - I(\mathbf{x})]^2 $$ 

This equation models the error in the sum of squared differences for a given displacement $\mathbf{u}$.

To achieve sub-pixel precision, the [[Lucas-Kanade Method]] uses a Taylor series expansion of the image function to perform gradient descent on the SSD energy function.

$$ E_{LK}^{SSD} \approx \sum_{i} [2 e_i 
abla I(\mathbf{x} + \{u\}) + 
abla I(\{u\})^T 
abla I(\{u	ext{-})]^2 $$ 

This formula describes the incremental refinement of the displacement using the image gradient (Jacobian).

### Compositing and Blending
Once images are registered, they must be blended to create a seamless composite. [[Gradient Domain Blending]] is an advanced technique that copies gradients rather than pixel values, which helps in matching exposure and lighting conditions. This is often solved using the Poisson equation to ensure smooth transitions at the seams.

## Relationships
- [[Image Registration]] utilizes [[Bundle Adjustment]]
- [[Image Registration]] includes [[Translational Alignment]]
- [[Image Registration]] specialises to [[Optical Flow]]
- [[Motion Estimation]] requires [[Bundle Adjustment]]
- [[Motion Estimation]] involves [[Translational Alignment]]
- [[Bundle Adjustment]] enables [[Panoramas]]
- [[Bundle Adjustment]] uses [[RANSAC]]
- [[Gradient Domain Blending]] is a compositing technique for [[Image Registration]]
- [[Translational Alignment]] is refined by [[Lucas-Kanade Method]]
- [[Optical Flow]] is a form of [[Motion Estimation]]
