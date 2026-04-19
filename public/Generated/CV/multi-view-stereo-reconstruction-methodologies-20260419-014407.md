---
type: content
title: "Multi-view Stereo Reconstruction Methodologies"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:44:07.244805200+00:00
summary: "Multi-view stereo (MVS) algorithms reconstruct 3D geometry from multiple images by evaluating photo-consistency. These methods range from depthmap-based to point-cloud and volumetric fusion approaches, each offering different trade-offs in scalability, noise handling, and application suitability."
tags:
  - computer-vision
  - mvs
  - 3d-reconstruction
  - photometric-consistency
  - optimization
entities:
  - "[[Multi-view Stereo]]"
  - "[[Photo-consistency]]"
  - "[[Depthmap Reconstruction]]"
  - "[[Point-cloud Reconstruction]]"
  - "[[Volumetric Data Fusion]]"
  - "[[Markov Random Field]]"
  - "[[Mesh Refinement]]"
  - "[[Structure Priors]]"
relationships:
  - source: "Multi-view Stereo"
    target: "Photo-consistency"
    description: "relies on"
  - source: "Multi-view Stereo"
    target: "Depthmap Reconstruction"
    description: "implements via"
  - source: "Multi-view Stereo"
    target: "Point-cloud Reconstruction"
    description: "implements via"
  - source: "Multi-view Stereo"
    target: "Volumetric Data Fusion"
    description: "implements via"
  - source: "Depthmap Reconstruction"
    target: "Photo-consistency"
    description: "uses"
  - source: "Point-cloud Reconstruction"
    target: "Photo-consistency"
    description: "uses"
  - source: "Volumetric Data Fusion"
    target: "Photo-consistency"
    description: "uses"
  - source: "Depthmap Reconstruction"
    target: "Markov Random Field"
    description: "formulated as"
  - source: "Mesh Refinement"
    target: "Multi-view Stereo"
    description: "refines"
  - source: "Structure Priors"
    target: "Multi-view Stereo"
    description: "improves"
  - source: "Markov Random Field"
    target: "Photo-consistency"
    description: "regularizes using"
---

# Multi-view Stereo Reconstruction Methodologies

*Multi-view stereo (MVS) algorithms reconstruct 3D geometry from multiple images by evaluating photo-consistency. These methods range from depthmap-based to point-cloud and volumetric fusion approaches, each offering different trade-offs in scalability, noise handling, and application suitability.*

[[Multi-view Stereo]] (MVS) is the process of reconstructing 3D geometry from a collection of images by evaluating the consistency of appearance across different views.

## Concept
At the core of MVS is [[Photo-consistency]], which measures how well a 3D point or surface element matches across multiple images. Common strategies for evaluating this consistency include Normalized Cross Correlation (NCC) and Sum of Squared Differences (SSD).

### Reconstruction Paradigms
Modern MVS algorithms are typically categorized by their output scene representation:

1. **[[Depthmap Reconstruction]]**: Reconstructs a 2D array of 3D points for each input image. This is highly scalable and flexible, often using a "Winner-takes-all" strategy or [[Markov Random Field]] (MRF) optimization to enforce spatial consistency.
2. **[[Point-cloud Reconstruction]]**: Reconstructs a single 3D model using all input images simultaneously. This approach, often uses oriented patches to represent local surface geometry (center and normal), making it more robust to noise and occlusions than simple depthmaps.
3. **[[Volumetric Data Fusion]]**: Accumulates 3D evidence into a voxel grid or tetrahedralization to extract a surface model (e.g., via Marching Cubes or Graph-Cuts). This is a powerful way to merge diverse measurements like depthmaps or point clouds into a single clean mesh.

## Optimization and Refinement
Once an initial geometry is obtained, [[Mesh Refinement]] is often employed to polish the model. This involves minimizing an energy function consisting of a data term (photometric consistency) and a regularization term (smoothness). Techniques like the Mesh Laplacian or Bi-Laplacian are used to enforce smoothness.

For challenging scenes (e.g., textureless or highly non-Lambertian surfaces), [[Structure Priors]] are integrated into the MVS framework to act as smart interpolators or smoothers, leveraging structural regularities like planarity.

## Relationships
- [[Multi-view Stereo]] relies on [[Photo-consistency]]
- [[Multi-view Stereo]] implements via [[Depthmap Reconstruction]], [[Point-cloud Reconstruction]], and [[Volumetric Data Fusion]]
- [[Depthmap Reconstruction]] uses [[Photo-consistency]]
- [[Point-cloud Reconstruction]] uses [[Photo-consistency]]
- [[Volumetric Data Fusion]] uses [[Photo-consistency]]
- [[Depthmap Reconstruction]] is formulated as [[Markov Random Field]]
- [[Mesh Refinement]] refines [[Multi-view Stereo]]
- [[Structure Priors]] improves [[Multi-view Stereo]]
- [[Markov Random Field]] regularizes using [[Photo-consistency]]
