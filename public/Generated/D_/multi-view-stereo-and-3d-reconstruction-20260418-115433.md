---
type: content
title: "Multi-View Stereo and 3D Reconstruction"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:54:33.061236700+00:00
summary: "An overview of techniques for reconstructing 3D object models from multiple images, including volumetric, surface-based, and model-based approaches."
tags:
  - computer-vision
  - 3d-reconstruction
  - multi-view-stereo
  - photogrammetry
entities:
  - "[[Multi-View Stereo]]"
  - "[[3D Object Model]]"
  - "[[Scene Flow]]"
  - "[[Voxel Representation]]"
  - "[[Level Set]]"
  - "[[Polygonal Mesh]]"
  - "[[Depth Map]]"
  - "[[Photoconsistency Measure]]"
  - "[[Geometric Visibility Model]]"
  - "[[Visual Hull]]"
  - "[[Space Carving]]"
  - "[[Voxel Coloring]]"
  - "[[Iterated Closest Point]]"
  - "[[Signed Distance Function]]"
  - "[[Implicit Surface]]"
  - "[[Principal Component Analysis]]"
  - "[[Shape From Shading]]"
  - "[[Photometric Stereo]]"
  - "[[Shape From Texture]]"
  - "[[Shape From Focus]]"
relationships:
  - source: "Multi-View Stereo"
    target: "3D Object Model"
    description: "reconstructs"
  - source: "Multi-View Stereo"
    target: "Scene Flow"
    description: "is closely related to"
  - source: "Multi-View Stereo"
    target: "Voxel Representation"
    description: "uses"
  - source: "Multi-View Stereo"
    target: "Level Set"
    description: "uses"
  - source: "Multi-View Stereo"
    target: "Polygonal Mesh"
    description: "uses"
  - source: "Multi-View Stereo"
    target: "Depth Map"
    description: "uses"
  - source: "Multi-View Stereo"
    target: "Photoconsistency Measure"
    description: "depends on"
  - source: "Multi-View Stereo"
    target: "Geometric Visibility Model"
    description: "employs"
  - source: "Multi-View Stereo"
    target: "Visual Hull"
    description: "can be initialized by"
  - source: "Space Carving"
    target: "Voxel Representation"
    description: "operates on"
  - source: "Voxel Coloring"
    target: "Voxel Representation"
    description: "is a technique for"
  - source: "Iterated Closest Point"
    target: "3D Object Model"
    description: "aligns partial"
  - source: "Iterated Closest Point"
    target: "Signed Distance Function"
    description: "can use for efficiency"
  - source: "Implicit Surface"
    target: "Signed Distance Function"
    description: "is often represented by"
  - source: "Principal Component Analysis"
    target: "3D Object Model"
    description: "parameterizes morphable"
---

# Multi-View Stereo and 3D Reconstruction

*An overview of techniques for reconstructing 3D object models from multiple images, including volumetric, surface-based, and model-based approaches.*

[[Multi-View Stereo]] is the process of reconstructing a complete [[3D Object Model]] from a collection of images taken from known camera viewpoints. A closely related task is [[Scene Flow]], which involves recovering the 3D shape and full 3D motion of every surface point in a dynamic scene.

## Reconstruction Representations

Multi-view stereo algorithms utilize various scene representations to model 3D geometry:
- **[[Voxel Representation]]**: A uniform grid of 3D voxels, often reconstructed using carving or optimization techniques.
- **[[Level Set]]**: A representation that encodes the signed distance to the surface on a uniform grid, allowing for finer detail than binary occupancy maps.
- **[[Polygonal Mesh]]**: The standard representation in computer graphics, which readily supports the computation of visibility and occlusions.
- **[[Depth Map]]**: Multiple depth maps can be computed independently and then merged into a coherent 3D model.

## Core Components of Stereo Algorithms

Reconstruction depends on several key design choices:
- **[[Photoconsistency Measure]]**: Similarity measures used to compare pixel values across different images, computed either in space or image space.
- **[[Geometric Visibility Model]]**: A principled way to reason about visibility and occlusions by predicting which surface pixels are visible in each image based on the current state of the model.

## Volumetric and Silhouette-Based Approaches

Volumetric reconstruction often employs [[Voxel Coloring]] or [[Space Carving]], where the 3D voxel grid is iteratively carved away. Another approach is the construction of a [[Visual Hull]], which is a 3D volumetric model reconstructed from the intersection of binary silhouettes projected into 3D space.

## Range Data and Surface Fitting

For data acquired via active rangefinding, the [[Iterated Closest Point]] (ICP) algorithm is widely used for the registration and alignment of partial 3D surface models. These models are often integrated using a [[Signed Distance Function]] to create a smooth, merged surface. This approach is closely linked to the use of [[Implicit Surface]] representations, where an indicator function defines the inside and outside of an object.

## Other Shape Cues

Beyond stereo, other cues are used to infer shape:
- **[[Shape From Shading]]**: Recovering surface shape from intensity variations caused by the angle between surface orientation and illumination.
- **[[Photometric Stereo]]**: Using multiple light sources to recover surface normals and albedo.
- **[[Shape From Texture]]**: Inferring local surface orientation from the foreshortening of regular patterns.
- **[[Shape From Focus]]**: Estimating depth based on the amount of blur (defocus) in images captured at different focal lengths.

## Model-Based Reconstruction

When prior knowledge of the object is available, specialized models can be used. For example, [[Principal Component Analysis]] can be applied to a collection of 3D scanned faces to create morphable models that can be fit to single images.

## Relationships

- Multi-View Stereo reconstructs 3D Object Model
- Multi-View Stereo is closely related to Scene Flow
- Multi-View Stereo uses Voxel Representation
- Multi-View Stereo uses Level Set
- Multi-View Stereo uses Polygonal Mesh
- Multi-View Stereo uses Depth Map
- Multi-View Stereo depends on Photoconsistency Measure
- Multi-View Stereo employs Geometric Visibility Model
- Multi-View Stereo can be initialized by Visual Hull
- Space Carving operates on Voxel Representation
- Voxel Coloring is a technique for Voxel Representation
- Iterated Closest Point aligns partial 3D Object Model
- Iterated Closest Point can use Signed Distance Function for efficiency
- Implicit Surface is often represented by Signed Distance Function
- Principal Component Analysis parameterizes morphable 3D Object Model
