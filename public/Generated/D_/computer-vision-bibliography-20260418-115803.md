---
type: content
title: "Computer Vision Bibliography"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:58:03.613487300+00:00
summary: "A comprehensive list of academic references covering algorithms and applications in computer vision, including stereo vision, image segmentation, and object recognition."
tags:
  - computer-vision
  - bibliography
  - academic-references
entities:
  - "[[Computer Vision]]"
  - "[[Stereo Vision]]"
  - "[[Image Segmentation]]"
  - "[[Object Recognition]]"
  - "[[Markov Random Fields]]"
  - "[[Graph Cuts]]"
  - "[[Camera Calibration]]"
  - "[[Multi-View Stereo]]"
  - "[[Image Registration]]"
  - "[[Belief Propagation]]"
relationships:
  - source: "Computer Vision"
    target: "Stereo Vision"
    description: "includes"
  - source: "Computer Vision"
    target: "Image Segmentation"
    description: "includes"
  - source: "Computer Vision"
    target: "Object Recognition"
    description: "includes"
  - source: "Markov Random Fields"
    target: "Image Segmentation"
    description: "used for"
  - source: "Graph Cuts"
    target: "Image Segmentation"
    description: "used for"
  - source: "Graph Cuts"
    target: "Stereo Vision"
    description: "used for"
  - source: "Camera Calibration"
    target: "Multi-View Stereo"
    description: "is required for"
  - source: "Multi-View Stereo"
    target: "Stereo Vision"
    description: "generalizes"
  - source: "Image Registration"
    target: "Computer Vision"
    description: "is a task in"
  - source: "Belief Propagation"
    target: "Markov Random Fields"
    description: "is used to solve"
  - source: "Belief Propagation"
    target: "Stereo Vision"
    description: "is used for"
---

# Computer Vision Bibliography

*A comprehensive list of academic references covering algorithms and applications in computer vision, including stereo vision, image segmentation, and object recognition.*

This bibliography provides a comprehensive collection of references for [[Computer Vision]], focusing on various algorithms and applications.

## Core Topics

### Stereo and 3D Reconstruction
Research in [[Stereo Vision]] and [[Multi-View Stereo]] often involves [[Camera Calibration]] to ensure accurate depth mapping and surface reconstruction. Techniques such as [[Belief Propagation]] and [[Graph Cuts]] are frequently employed to solve correspondence problems.

### Image Analysis and Segmentation
[[Image Segmentation]] is a central task, often modeled using [[Markov Random Fields]] to incorporate spatial constraints. Optimization methods like [[Graph Cuts]] are used to find efficient solutions for these models.

### Object Recognition and Registration
[[Object Recognition]] involves identifying categories and parts of objects within images, while [[Image Registration]] focuses on aligning different views of the same scene.

## Relationships
- [[Computer Vision]] includes [[Stereo Vision]]
- [[Computer Vision]] includes [[Image Segmentation]]
- [[Computer Vision]] includes [[Object Recognition]]
- [[Markov Random Fields]] used for [[Image Segmentation]]
- [[Graph Cuts]] used for [[Image Segmentation]]
- [[Graph Cuts]] used for [[Stereo Vision]]
- [[Camera Calibration]] is required for [[Multi-View Stereo]]
- [[Multi-View Stereo]] generalizes [[Stereo Vision]]
- [[Image Registration]] is a task in [[Computer Vision]]
- [[Belief Propagation]] is used to solve [[Markov Random Fields]]
- [[Belief Propagation]] is used for [[Stereo Vision]]
