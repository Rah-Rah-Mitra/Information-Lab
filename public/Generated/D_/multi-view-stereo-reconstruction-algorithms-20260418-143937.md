---
type: content
title: "Multi-View Stereo Reconstruction Algorithms"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:39:37.359603+00:00
summary: "An overview of depthmap and point-cloud reconstruction techniques, volumetric data fusion, and mesh refinement in multi-view stereo systems."
tags:
  - computer-vision
  - mvs
  - 3d-reconstruction
  - mrf
  - geometry-processing
entities:
  - "[[Multi-View Stereo]]"
  - "[[Depthmap Reconstruction]]"
  - "[[Point-cloud Reconstruction]]"
  - "[[Markov Random Field]]"
  - "[[Photo-consistency]]"
  - "[[Plane Sweeping Stereo]]"
  - "[[Volumetric Data Fusion]]"
  - "[[Graph-Cuts]]"
  - "[[Delaunay Tetrahedralization]]"
  - "[[Mesh Refinement]]"
  - "[[Structure Priors]]"
  - "[[Manhattan-world Stereo]]"
relationships:
  - source: "Multi-View Stereo"
    target: "Depthmap Reconstruction"
    description: "utilizes"
  - source: "Multi-View Stereo"
    target: "Point-cloud Reconstruction"
    description: "utilizes"
  - source: "Depthmap Reconstruction"
    target: "Markov Random Field"
    description: "is formulated as a"
  - source: "Depthmap Reconstruction"
    target: "Photo-consistency"
    description: "depends on"
  - source: "Plane Sweeping Stereo"
    target: "Depthmap Reconstruction"
    description: "is a real-time method for"
  - source: "Point-cloud Reconstruction"
    target: "Volumetric Data Fusion"
    description: "provides input for"
  - source: "Volumetric Data Fusion"
    target: "Graph-Cuts"
    description: "is often solved using"
  - source: "Volumetric Data Fusion"
    target: "Delaunay Tetrahedralization"
    description: "uses for space discretization"
  - source: "Multi-View Stereo"
    target: "Mesh Refinement"
    description: "employs as a final polishing step"
  - source: "Mesh Refinement"
    target: "Photo-consistency"
    description: "optimizes based on"
  - source: "Multi-View Stereo"
    target: "Structure Priors"
    description: "incorporates to handle textureless surfaces"
  - source: "Manhattan-world Stereo"
    target: "Structure Priors"
    description: "is an implementation of"
---

# Multi-View Stereo Reconstruction Algorithms

*An overview of depthmap and point-cloud reconstruction techniques, volumetric data fusion, and mesh refinement in multi-view stereo systems.*

[[Multi-View Stereo]] (MVS) is the process of reconstructing a dense 3D geometry from a set of images by exploiting photometric consistency across multiple views.

## Depthmap Reconstruction
Depthmap reconstruction focuses on estimating the depth of each pixel in a reference image. A common approach is to formulate the problem as a [[Markov Random Field]] (MRF), where unary costs are derived from [[Photo-consistency]] (often using Normalized Cross Correlation or NCC) and pairwise costs enforce spatial smoothness. To handle occlusion boundaries and low-confidence regions, an "unknown" state is often introduced into the label set.

### Real-Time Approaches
[[Plane Sweeping Stereo]] allows for real-time execution by sweeping parallel planes through the scene and projecting images via homographies, typically implemented on GPUs to achieve high performance.

## Point-cloud Reconstruction
Unlike depthmaps, [[Point-cloud Reconstruction]] grows a set of oriented patches (local tangent planes) across the scene. Algorithms like PMVS use a greedy expansion strategy, iterating between initial feature matching, patch expansion, and filtering to ensure global visibility consistency.

## Volumetric Data Fusion
To merge multiple depthmaps or point clouds into a global 3D model, [[Volumetric Data Fusion]] is used. This often involves discretizing the 3D space into a voxel grid or using [[Delaunay Tetrahedralization]] to create an adaptive grid based on reconstructed points. The problem is then formulated as a binary segmentation task (interior vs. exterior) and solved using [[Graph-Cuts]] to find the optimal surface boundary.

## Mesh Refinement
Once an initial mesh is extracted, [[Mesh Refinement]] is used to polish the surface. This process involves moving vertices to minimize an energy function consisting of a photometric consistency term, a regularization term (often using Mesh Laplacian or Bi-Laplacian operators), and optionally a silhouette consistency term.

## Structure Priors
Standard MVS often fails on textureless or non-Lambertian surfaces. [[Structure Priors]] are used to interpolate holes or smooth noise by assuming geometric regularities. For example, [[Manhattan-world Stereo]] assumes the scene consists of piecewise planar surfaces aligned with three dominant orthogonal axes, transforming the problem from a depthmap to a "planemap" where labels represent specific plane hypotheses.
