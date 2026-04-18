---
type: content
title: "Introduction to Multiple View Geometry"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:14:04.917422700+00:00
summary: "An overview of the geometric relations between images of 3D objects, focusing on projective geometry, camera projections, and scene reconstruction."
tags:
  - computer-vision
  - projective-geometry
  - camera-calibration
  - 3d-reconstruction
entities:
  - "[[Projective Geometry]]"
  - "[[Projective Transformation]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Euclidean Geometry]]"
  - "[[Affine Geometry]]"
  - "[[Absolute Conic]]"
  - "[[Camera Matrix]]"
  - "[[Fundamental Matrix]]"
  - "[[Trifocal Tensor]]"
  - "[[Projective Reconstruction]]"
  - "[[Euclidean Reconstruction]]"
  - "[[Auto-calibration]]"
relationships:
  - source: "Projective Geometry"
    target: "Projective Transformation"
    description: "is the framework for"
  - source: "Projective Transformation"
    target: "Homogeneous Coordinates"
    description: "is represented using"
  - source: "Affine Geometry"
    target: "Projective Geometry"
    description: "is a specialization of"
  - source: "Euclidean Geometry"
    target: "Affine Geometry"
    description: "is a specialization of"
  - source: "Euclidean Geometry"
    target: "Absolute Conic"
    description: "is defined by the specification of the"
  - source: "Camera Matrix"
    target: "Projective Geometry"
    description: "represents a mapping within"
  - source: "Fundamental Matrix"
    target: "Projective Reconstruction"
    description: "is the basic tool for"
  - source: "Trifocal Tensor"
    target: "Projective Reconstruction"
    description: "enables reconstruction from three views"
  - source: "Projective Reconstruction"
    target: "Euclidean Reconstruction"
    description: "is the first step toward"
  - source: "Auto-calibration"
    target: "Euclidean Reconstruction"
    description: "allows for"
  - source: "Auto-calibration"
    target: "Absolute Conic"
    description: "attempts to determine the position of the"
---

# Introduction to Multiple View Geometry

*An overview of the geometric relations between images of 3D objects, focusing on projective geometry, camera projections, and scene reconstruction.*

Multiple View Geometry is the study of the geometric relations between images of 3D objects and the process of reconstructing 3D structure from these images.

## Projective Framework

[[Projective Geometry]] provides a unified way to represent the 3D world and its 2D images. A central tool in this framework is the use of [[Homogeneous Coordinates]], which allow non-linear perspective projections to be represented as linear matrix equations and treat points at infinity as ordinary points.

A [[Projective Transformation]] is a mapping that preserves straight lines but not lengths, angles, or ratios of distances. Other geometries are specializations of this: [[Affine Geometry]] is obtained by designating a specific line or plane at infinity, and [[Euclidean Geometry]] is further specialized by identifying two circular points on that line or the [[Absolute Conic]] in 3D space.

## Camera Projections

Image formation is modeled as a central projection from a 3D projective space to a 2D projective plane. This process is represented by a 3x4 [[Camera Matrix]] P, which maps a 3D point to its image coordinates. If the camera is calibrated, the image of the absolute conic (IAC) is known, allowing for the measurement of angles and the determination of the Euclidean structure of a scene.

## Scene Reconstruction

Reconstructing a 3D scene from multiple images typically begins with [[Projective Reconstruction]], where the structure is recovered up to a projective ambiguity. For two views, the [[Fundamental Matrix]] encapsulates the relative projective geometry and is used to perform triangulation.

For three views, the [[Trifocal Tensor]] relates corresponding points and lines across the images, providing greater stability and allowing for the lause of line correspondences.

To move from a projective to a [[Euclidean Reconstruction]], the internal calibration of the cameras must be known. [[Auto-calibration]] techniques attempt to recover this calibration by leveraging constraints such as the assumption of identical cameras or square pixels to locate the absolute conic.

## Relationships

- [[Projective Geometry]] is the framework for [[Projective Transformation]].
- [[Projective Transformation]] is represented using [[Homogeneous Coordinates]].
- [[Affine Geometry]] is a specialization of [[Projective Geometry]].
- [[Euclidean Geometry]] is a specialization of [[Affine Geometry]].
- [[Euclidean Geometry]] is defined by the specification of the [[Absolute Conic]].
- [[Camera Matrix]] represents a mapping within [[Projective Geometry]].
- [[Fundamental Matrix]] is the basic tool for [[Projective Reconstruction]].
- [[Trifocal Tensor]] enables reconstruction from three views in [[Projective Reconstruction]].
- [[Projective Reconstruction]] is the first step toward [[Euclidean Reconstruction]].
- [[Auto-calibration]] allows for [[Euclidean Reconstruction]].
- [[Auto-calibration]] attempts to determine the position of the [[Absolute Conic]].
