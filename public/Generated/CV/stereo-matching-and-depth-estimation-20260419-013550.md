---
type: content
title: "Stereo Matching And Depth Estimation"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:35:50.443318800+00:00
summary: "Stereo matching is the process of finding pixel correspondences between multiple images to build 3D models. It utilizes epipolar geometry to constrain search spaces and employs various optimization techniques like local window-based or global energy minimization. The field has evolved from sparse feature-based methods to dense per-pixel depth maps, increasingly leveraging deep neural networks."
tags:
  - computer-vision
  - stereo-vision
  - depth-estimation
  - epipolar-geometry
  - optimization
entities:
  - "[[Stereo Matching]]"
  - "[[Epipolar Geometry]]"
  - "[[Disparity Map]]"
  - "[[Disparity Space Image]]"
  - "[[Rectification]]"
  - "[[Global Optimization]]"
  - "[[Local Methods]]"
  - "[[Dense Correspondence]]"
  - "[[Dynamic Programming]]"
  - "[[Similarity Measures]]"
  - "[[Deep Neural Networks]]"
relationships:
  - source: "Stereo Matching"
    target: "Epipolar Geometry"
    description: "uses"
  - source: "Stereo Matching"
    target: "Disparity Map"
    description: "produces"
  - source: "Stereo Matching"
    target: "Dense Correspondence"
    description: "seeks"
  - source: "Epipolar Geometry"
    target: "Rectification"
    description: "enables"
  - source: "Disparity Space Image"
    target: "Stereo Matching"
    description: "represents"
  - source: "Global Optimization"
    target: "Stereo Matching"
    description: "improves"
  - source: "Local Methods"
    target: "Stereo Matching"
    description: "improves"
  - source: "Deep Neural Networks"
    target: "Stereo Matching"
    description: "enhances"
---

# Stereo Matching And Depth Estimation

*Stereo matching is the process of finding pixel correspondences between multiple images to build 3D models. It utilizes epipolar geometry to constrain search spaces and employs various optimization techniques like local window-based or global energy minimization. The field has evolved from sparse feature-based methods to dense per-pixel depth maps, increasingly leveraging deep neural networks.*

[[Stereo Matching]] is the fundamental process of taking two or more images and building a 3D model of the scene by finding matching pixels and converting their 2D positions into 3D depths. This process is essential for applications ranging from robotic navigation to augmented reality.

## Concept

### Epipolar Geometry
To reduce the search space for correspondences, [[Epipolar Geometry]] is used. A pixel in one image projects to an epipolar line in the other, constraining the search to a 1D line rather than a 2D area. [[Rectification]] is a common technique used to pre-warp images so that corresponding epipolar lines are coincident horizontal scanlines, simplifying the matching process.

$$ d = \frac{fB}{Z} $$

This formula relates disparity \(d\) to depth \(Z\), focal length \(f\), and baseline \(B\).

### Disparity Space
After rectification, similarity is measured at various disparities, and results are stored in a [[Disparity Space Image]] (DSI). A DSI represents the cost of matching pixels at different disparity hypotheses, allowing for the processes of [[Local Methods]] or [[Global Optimization]] to find the best surface.

### Correspondence Strategies

#### Dense Correspondence
Unlike early sparse feature-based methods, modern algorithms focus on [[Dense Correspondence]], aiming to assign a depth value to every pixel. This is achieved through several algorithmic building blocks: matching cost computation, cost aggregation, disparity computation, and refinement.

#### Local Methods
[[Local Methods]] use a window-based approach, aggregating matching costs over a support region in the DSI. These methods often perform a "winner-take-all" optimization, which is highly efficient but can struggle with depth discontinuities.

#### Global Optimization
[[Global Optimization]] methods aim to find a solution that minimizes a global energy function, often incorporating smoothness constraints. Techniques like [[Dynamic Programming]] can find global minima for scanline-based optimization, while [[Segmentation-based Techniques]] use image regions to guide the disparity assignment.

#### Deep Learning
Recently, [[Deep Neural Networks]] have significantly impacted the field, providing end-to-end learning capabilities for stereo matching pipelines.

## Relationships
- [[Stereo Matching]] uses [[Epipolar Geometry]]
- [[Stereo Matching]] produces [[Disparity Map]]
- [[Stereo Matching]] seeks [[Dense Correspondence]]
- [[Epipolar Geometry]] enables [[Rectification]]
- [[Disparity Space Image]] represents [[Stereo Matching]]
- [[Global Optimization]] improves [[Stereo Matching]]
- [[Local Methods]] improves [[Stereo Matching]]
- [[Deep Neural Networks]] enhances [[Stereo Matching]]
