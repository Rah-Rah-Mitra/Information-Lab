---
type: content
title: "Image-Based Rendering Techniques"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:55:10.565592100+00:00
summary: "An overview of methods for synthesizing novel views of scenes using multiple images or videos, ranging from view interpolation to light fields and video-based rendering."
tags:
  - computer-vision
  - image-based-rendering
  - graphics
  - 3d-reconstruction
entities:
  - "[[Image-Based Rendering]]"
  - "[[View Interpolation]]"
  - "[[View-Dependent Texture Maps]]"
  - "[[Layered Depth Image]]"
  - "[[Light Field]]"
  - "[[Lumigraph]]"
  - "[[Unstructured Lumigraph]]"
  - "[[Surface Light Field]]"
  - "[[Environment Matte]]"
  - "[[Video-Based Rendering]]"
  - "[[Video Textures]]"
  - "[[Photo Tourism]]"
relationships:
  - source: "Image-Based Rendering"
    target: "View Interpolation"
    description: "includes"
  - source: "Image-Based Rendering"
    target: "View-Dependent Texture Maps"
    description: "includes"
  - source: "Image-Based Rendering"
    target: "Layered Depth Image"
    description: "includes"
  - source: "Image-Based Rendering"
    target: "Light Field"
    description: "includes"
  - source: "Image-Based Rendering"
    target: "Video-Based Rendering"
    description: "includes"
  - source: "View Interpolation"
    target: "Layered Depth Image"
    description: "is improved by"
  - source: "Light Field"
    target: "Lumigraph"
    description: "is extended by"
  - source: "Lumigraph"
    target: "Unstructured Lumigraph"
    description: "has a variant called"
  - source: "Light Field"
    target: "Surface Light Field"
    description: "can be re-parameterized as"
  - source: "Image-Based Rendering"
    target: "Environment Matte"
    description: "includes"
  - source: "Video-Based Rendering"
    target: "Video Textures"
    description: "includes"
  - source: "Photo Tourism"
    target: "View Interpolation"
    description: "applies"
---

# Image-Based Rendering Techniques

*An overview of methods for synthesizing novel views of scenes using multiple images or videos, ranging from view interpolation to light fields and video-based rendering.*

[[Image-Based Rendering]] is the combination of 3D reconstruction techniques from computer vision and computer graphics rendering to create interactive, photo-realistic experiences from multiple views of a scene.

## Core Techniques

### View Interpolation and Texture Mapping
[[View Interpolation]] generates novel views by combining pairs of reference images with pre-computed depth maps, using warping and blending to create a seamless transition. A closely related approach is [[View-Dependent Texture Maps]], where a single 3D model is used, but the texture source changes based on the virtual camera's position, blending images based on the angle between the virtual and source views.

### Depth-Based Representations
To address the holes and cracks that appear during warping in standard view interpolation, [[Layered Depth Image]] (LDI) stores multiple depth and color values at each pixel. Alternatively, objects can be organized into layers or sprites, often referred to as "plane plus parallax" representations in computer vision.

### Light Fields and Lumigraphs
A [[Light Field]] is a 4D representation of all possible rays in a scene. The [[Lumigraph]] extends this by incorporating 3D geometry to improve resampling and reduce ghosting. When images are acquired irregularly, the [[Unstructured Lumigraph]] renders directly from the original images using fidelity criteria like epipole consistency and angular deviation. To further compress this data, a [[Surface Light Field]] re-parameterizes the light field to lie on the object's surface, storing a "Lumisphere" of rays for each surface point.

### Environment Mattes
An [[Environment Matte]] models how a refractive or reflective object interacts with its environment by mapping pixels to a 4D distribution of light rays, allowing the object to be placed in front of novel backgrounds while preserving realistic refractive interplay.

## Video-Based Rendering

[[Video-Based Rendering]] applies similar principles to video sequences. This includes [[Video Textures]], where short clips are re-arranged into infinite-length animations by matching frame triplets to preserve motion continuity. Other applications include 3D video (free-viewpoint video) and interactive video-based walkthroughs of large environments.

## Applications

[[Photo Tourism]] is a prominent application that uses structure from motion to allow users to navigate through large collections of casually acquired photographs using 3D planar proxies.

## Relationships
- [[Image-Based Rendering]] includes [[View Interpolation]]
- [[Image-Based Rendering]] includes [[View-Dependent Texture Maps]]
- [[Image-Based Rendering]] includes [[Layered Depth Image]]
- [[Image-Based Rendering]] includes [[Light Field]]
- [[Image-Based Rendering]] includes [[Video-Based Rendering]]
- [[View Interpolation]] is improved by [[Layered Depth Image]]
- [[Light Field]] is extended by [[Lumigraph]]
- [[Lumigraph]] has a variant called [[Unstructured Lumigraph]]
- [[Light Field]] can be re-parameterized as [[Surface Light Field]]
- [[Image-Based Rendering]] includes [[Environment Matte]]
- [[Video-Based Rendering]] includes [[Video Textures]]
- [[Photo Tourism]] applies [[View Interpolation]]
