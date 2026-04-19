---
type: content
title: "Image-Based Rendering and Novel View Synthesis"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:39:01.881806500+00:00
summary: "Image-based rendering (IBR) encompasses a spectrum of techniques used to synthesize new views from existing images or video. These methods range from classic 3D texture-mapped models to advanced light field and neural rendering approaches. The choice of technique depends on the balance between input data availability and the desired visual fidelity."
tags:
  - computer-vision
  - image-based-rendering
  - novel-view-synthesis
  - light-fields
  - 3d-photography
entities:
  - "[[Image-Based Rendering]]"
  - "[[Novel View Synthesis]]"
  - "[[View Interpolation]]"
  - "[[Light Field]]"
  - "[[Lumigraph]]"
  - "[[Layered Depth Images]]"
  - "[[Multiplane Images]]"
  - "[[Environment Mattes]]"
  - "[[Video-Based Rendering]]"
  - "[[Neural Rendering]]"
relationships:
  - source: "Image-Based Rendering"
    target: "Novel View Synthesis"
    description: "enables"
  - source: "View Interpolation"
    target: "Image-Based Rendering"
    description: "is a type of"
  - source: "Light Field"
    target: "Image-Based Rendering"
    description: "is a representation for"
  - source: "Lumigraph"
    target: "Light Field"
    description: "is a specialized"
  - source: "Layered Depth Images"
    target: "Image-Based Rendering"
    description: "is a representation for"
  - source: "Multiplane Images"
    target: "Layered Depth Images"
    description: "is a variant of"
  - source: "Environment Mattes"
    target: "Image-Based Rendering"
    description: "is a technique for"
  - source: "Video-Based Rendering"
    target: "Image-Based Rendering"
    description: "is an extension to"
  - source: "Neural Rendering"
    target: "Image-Based Rendering"
    description: "is a modern approach to"
---

# Image-Based Rendering and Novel View Synthesis

*Image-based rendering (IBR) encompasses a spectrum of techniques used to synthesize new views from existing images or video. These methods range from classic 3D texture-mapped models to advanced light field and neural rendering approaches. The choice of technique depends on the balance between input data availability and the desired visual fidelity.*

[[Image-Based Rendering]] is a broad field of computer vision focused on synthesizing new views of a scene from a collection of existing images or video. It sits on a continuum between explicit 3D geometric modeling and pure sampled ray-based representations.

## Concept

### View Interpolation
[[View Interpolation]] is the process of generating intermediate views between two or more reference images. This is often achieved by warping source images using precomputed depth maps and then blending them. For example, in the [[Chen and Williams 1993]] approach, depth maps are represented as quadtrees for efficiency. To resolve occlusion conflicts during warping, techniques like z-buffering or back-to-front ordering are used.

### Light Fields and Lumigraphs
A [[Light Field]] represents the 4D space of all possible light rays in a scene. This can be parameterized using two planes (the $s,t$ and $u,v$ planes) such that any ray is described by a $(s, t, u, v)$ coordinate. A [[Lumigraph]] extends this by incorporating 3D geometry to improve rendering quality and reduce ghosting.

$$ s, t, u, v $$

This equation represents the 4D coordinates of a light ray passing through two planes.

### Layered Representations
To handle occlusions and depth discontinuities, several layered representations are used:
- [[Layered Depth Images]] (LDI): Stores multiple color-depth values at each pixel.
- [[Multiplane Images]] (MPI): A discretization of the viewing frustum into fronto-parallel planes, each containing RGBA values.
- [[Sprites with Depth]]: Uses flat, alpha-matted geometry to represent distant objects.

### Environment Mattes
[[Environment Mattes]] are used to model how refractive or reflective objects interact with their surroundings. Instead of modeling the 4D space of rays, an environment matte captures the 4D mapping of a pixel's position and its reflection/refraction angle $(\phi, \theta)$ to the environment.

### Video-Based Rendering
[[Video-Based Rendering]] extends IBR to temporal sequences. This includes [[Video Textures]], where short clips are are re-arranged to create infinite loops, and [[Free-Viewpoint Video]], which uses synchronized multi-camera rigs to allow users to move through a 3D scene.

## Relationships
- [[Image-Based Rendering]] enables [[Novel View Synthesis]]
- [[View Interpolation]] is a type of [[Image-Based Rendering]]
- [[Light Field]] is a representation for [[Image-Based Rendering]]
- [[Lumigraph]] is a specialized [[Light Field]]
- [[Layered Depth Images]] is a variant of [[Multiplane Images]]
- [[Environment Mattes]] is a technique for [[Image-Based Rendering]]
- [[Video-Based Rendering]] is an extension to [[Image-Based Rendering]]
- [[Neural Rendering]] is a modern approach to [[Image-Based Rendering]]
