---
type: content
title: "3D Reconstruction and Shape Inference Techniques"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:36:17.070744800+00:00
summary: "This note explores various methods for recovering 3D geometry from 2D images, ranging from passive stereo matching to active scanning. It covers the mathematical principles of shape from X, shape from silhouettes, and volumetric reconstruction. These techniques are fundamental to robotics, digital heritage, and computer graphics."
tags:
  - computer-vision
  - 3d-reconstruction
  - stereo-vision
  - shape-from-x
entities:
  - "[[3D Reconstruction]]"
  - "[[Shape from Shading]]"
  - "[[Photometric Stereo]]"
  - "[[Shape from Texture]]"
  - "[[Shape from Focus]]"
  - "[[3D Scanning]]"
  - "[[Multi-view Stereo]]"
  - "[[Epipolar Plane Image]]"
  - "[[Visual Hull]]"
  - "[[Monocular Depth Estimation]]"
relationships:
  - source: "3D Reconstruction"
    target: "Shape from Shading"
    description: "utilizes"
  - source: "3D Reconstruction"
    target: "Multi-view Stereo"
    description: "includes"
  - source: "3D Reconstruction"
    target: "3D Scanning"
    description: "includes"
  - source: "Multi-view Stereo"
    target: "Epipolar Plane Image"
    description: "uses"
  - source: "3D Reconstruction"
    target: "Shape from Texture"
    description: "utilizes"
  - source: "3D Reconstruction"
    target: "Shape from Focus"
    description: "description"
  - source: "3D Reconstruction"
    target: "Visual Hull"
    description: "produces"
  - source: "3D Reconstruction"
    target: "Monocular Depth Estimation"
    description: "is_related_to"
  - source: "Shape from Shading"
    target: "Photometric Stereo"
    description: "generalises_to"
  - source: "Shape from Shading"
    target: "3D Reconstruction"
    description: "is_a_type_of"
  - source: "Shape from Texture"
    target: "3D Reconstruction"
    description: "is_a_type_of"
  - source: "Shape from Focus"
    target: "3D Reconstruction"
    description: "is_a_type_of"
  - source: "3D Scanning"
    target: "3D Reconstruction"
    description: "is_a_type_of"
  - source: "3D Reconstruction"
    target: "Monocular Depth Estimation"
    description: "is_related_to"
---

# 3D Reconstruction and Shape Inference Techniques

*This note explores various methods for recovering 3D geometry from 2D images, ranging from passive stereo matching to active scanning. It covers the mathematical principles of shape from X, shape from silhouettes, and volumetric reconstruction. These techniques are fundamental to robotics, digital heritage, and computer graphics.*

This note summarizes the diverse methodologies for 3D shape acquisition and reconstruction from visual and active sensing data.

## Concept
[[3D Reconstruction]] is the process of recovering 3D geometry from 2D observations. This can be achieved through several distinct paradigms:

### Passive Cues (Shape from X)
Methods in the [[Shape from X]] category rely on intrinsic image properties to infer shape:
- [[Shape from Shading]] uses intensity variations caused by the angle between surface normals and light sources to estimate orientation. For diffuse surfaces, the irradiance equation is modeled as:
$$ I(x, y) = R(p, q) ho(p, q) ight. $$ 
This equation models the relationship between surface reflectance and local orientation.
- [[Shape from Texture]] exploits the foreshortening of regular patterns to infer local surface orientation.
- [[Shape from Focus]] (or depth from defocus) uses the amount of blur to estimate depth, where the amount of defocus increases as the surface moves away from the focus plane.

### Active Sensing
[[3D Scanning]] involves projecting known patterns (e.g., light stripes, speckle patterns, or structured light) to create synthetic texture or depth information:
- **Structured Light**: Uses deformations of projected patterns (like a single light stripe or a checkerboard) to infer 3D shape via triangulation.
- **Time-of-Flight (ToF)**: Measures the time delay of light signals to estimate absolute distance.
- **RGB-D Cameras**: Devices like the Microsoft Kinect use infrared speckle patterns to provide dense depth maps.

### Multi-View and Volumetric Approaches
- [[Multi-view Stereo]] leverages multiple images from different viewpoints to improve robustness and accuracy. A key visualization tool is the [[Epipolar Plane Image]] (EPI), which represents the 4D light field as a 2D slice, where the motion of objects at different depths is visible as slopes.
- [[Visual Hull]] construction involves intersecting the binary silhouettes of an object from multiple views to find the maximal volume consistent with the views.
- [[Monocular Depth Estimation]] attempts to infer depth from a single image, often using deep neural networks to 'hallucinate' depth based on learned priors.

## Relationships
- [[3D Reconstruction]] utilizes [[Shape from Shading]], [[Shape from Texture]], [[Shape from Focus]], [[3D Scanning]], and [[Multi-view Stereo]].
- [[Shape from Shading]] generalizes to [[Photometric Stereo]], which uses multiple light sources to recover both albedo and surface normals.
- [[Shape from Shading]] is a type of [[3D Reconstruction]].
- [[Shape from Texture]] is a type of [[3D Reconstruction]].
- [[Shape from Focus]] is a type of [[3D Reconstruction]].
- [[3D Scanning]] is a type of [[3D Reconstruction]].
- [[Multi-view Stereo]] uses [[Epipolar Plane Image]].
- [[3D Reconstruction]] is related to [[Monocular Depth Estimation]].
- [[3D Reconstruction]] produces [[Visual Hull]].
