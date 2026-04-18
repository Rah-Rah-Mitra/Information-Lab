---
type: content
title: "Stereo Correspondence and Depth Estimation"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:53:46.676016400+00:00
summary: "An overview of stereo matching techniques, from epipolar geometry and rectification to dense correspondence and global optimization for 3D reconstruction."
tags:
  - computer-vision
  - stereo-vision
  - 3d-reconstruction
  - epipolar-geometry
entities:
  - "[[Stereo Correspondence]]"
  - "[[Epipolar Geometry]]"
  - "[[Rectification]]"
  - "[[Plane Sweep]]"
  - "[[Disparity Map]]"
  - "[[Dense Correspondence]]"
  - "[[Sparse Correspondence]]"
  - "[[Global Optimization]]"
  - "[[Local Methods]]"
  - "[[Disparity Space Image]]"
  - "[[Multi-view Stereo]]"
  - "[[Epipolar Plane Image]]"
relationships:
  - source: "Stereo Correspondence"
    target: "Epipolar Geometry"
    description: "is constrained by"
  - source: "Rectification"
    target: "Epipolar Geometry"
    description: "simplifies"
  - source: "Plane Sweep"
    target: "Disparity Map"
    description: "is an alternative method to generate"
  - source: "Stereo Correspondence"
    target: "Dense Correspondence"
    description: "can be performed as"
  - source: "Stereo Correspondence"
    target: "Sparse Correspondence"
    description: "can be performed as"
  - source: "Dense Correspondence"
    target: "Disparity Map"
    description: "aims to produce a"
  - source: "Local Methods"
    target: "Disparity Space Image"
    description: "aggregate costs within a"
  - source: "Global Optimization"
    target: "Dense Correspondence"
    description: "is used to solve"
  - source: "Multi-view Stereo"
    target: "Disparity Map"
    description: "improves the accuracy of"
  - source: "Multi-view Stereo"
    target: "Epipolar Plane Image"
    description: "is visualized using"
---

# Stereo Correspondence and Depth Estimation

*An overview of stereo matching techniques, from epipolar geometry and rectification to dense correspondence and global optimization for 3D reconstruction.*

[[Stereo Correspondence]] is the process of establishing correspondences between pixels in two or more images to recover depth information and build 3D models of a scene.

## Epipolar Geometry and Rectification

[[Epipolar Geometry]] provides the geometric constraints that reduce the search for corresponding pixels from a 2D area to a 1D [[Epipolar Plane Image]] (or epipolar line). A pixel in one image projects to an epipolar line in the other, bounded by the epipole.

[[Rectification]] is the process of warping images so that corresponding epipolar lines are coincident with horizontal scanlines. This simplifies the matching process, as the search for correspondences is reduced to a horizontal shift, leading to a simple inverse relationship between 3D depth and disparity.

## Correspondence Strategies

### Sparse Correspondence
[[Sparse Correspondence]] focuses on matching a limited set of high-certainty features, such as interest points or [[Sparse Correspondence]] profile curves (occluding contours). This approach is computationally efficient and useful for untextured surfaces where only boundaries are visible.

### Dense Correspondence
[[Dense Correspondence]] aims to estimate a [[Disparity Map]] for every pixel in the image. This is more challenging due to textureless regions and occlusions. The process typically involves matching cost computation, cost aggregation, disparity optimization, and refinement.

## Estimation Methods

### Local Methods
[[Local Methods]] use a window-based approach to aggregate matching costs. They often operate on a [[Disparity Space Image]] (DSI), where the winning disparity is chosen via a "winner-take-all" (WTA) optimization. Advanced versions use shiftable windows or adaptive weights to better handle depth discontinuities.

### Global Optimization
[[Global Optimization]] treats stereo matching as an energy minimization problem. It combines a data term (matching cost) with a smoothness term to encourage piecewise smooth disparity maps. Techniques include Markov Random Fields (MRFs), graph cuts, and dynamic programming.

### Plane Sweep
[[Plane Sweep]] is an alternative to rectification where a set of planes is swept through the scene, and images are re-projected onto these planes to measure photoconsistency.

## Multi-view Stereo

[[Multi-view Stereo]] extends pairwise matching to multiple images, which significantly improves depth map quality and occlusion reasoning. It often utilizes the [[Epipolar Plane Image]] to reason about the relationship between different depth layers and can involve simultaneous estimation of multiple depth maps for consistency.

## Relationships
- [[Stereo Correspondence]] is constrained by [[Epipolar Geometry]]
- [[Rectification]] simplifies [[Epipolar Geometry]]
- [[Plane Sweep]] is an alternative method to generate [[Disparity Map]]
- [[Stereo Correspondence]] can be performed as [[Dense Correspondence]]
- [[Stereo Correspondence]] can be performed as [[Sparse Correspondence]]
- [[Dense Correspondence]] aims to produce a [[Disparity Map]]
- [[Local Methods]] aggregate costs within a [[Disparity Space Image]]
- [[Global Optimization]] is used to solve [[Dense Correspondence]]
- [[Multi-view Stereo]] improves the accuracy of [[Disparity Map]]
- [[Multi-view Stereo]] is visualized using [[Epipolar Plane Image]]
