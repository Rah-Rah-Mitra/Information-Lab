---
type: content
title: "Geometric Primitives and Transformations"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:46:35.595698700+00:00
summary: "An overview of 2D and 3D geometric primitives and the hierarchy of coordinate transformations used to project 3D scenes into 2D images."
tags:
  - computer-vision
  - geometry
  - linear-algebra
  - image-formation
entities:
  - "[[2D Point]]"
  - "[[2D Line]]"
  - "[[2D Conic]]"
  - "[[3D Point]]"
  - "[[3D Plane]]"
  - "[[3D Line]]"
  - "[[3D Quadric]]"
  - "[[Homogeneous Coordinates]]"
  - "[[Translation]]"
  - "[[Euclidean Transformation]]"
  - "[[Similarity Transform]]"
  - "[[Affine Transform]]"
  - "[[Projective Transformation]]"
  - "[[Homography]]"
  - "[[Calibration Matrix]]"
  - "[[Camera Matrix]]"
  - "[[Radial Distortion]]"
relationships:
  - source: "Homogeneous Coordinates"
    target: "2D Point"
    description: "represents"
  - source: "Homogeneous Coordinates"
    target: "2D Line"
    description: "represents"
  - source: "Homogeneous Coordinates"
    target: "3D Point"
    description: "represents"
  - source: "Homogeneous Coordinates"
    target: "3D Plane"
    description: "represents"
  - source: "Euclidean Transformation"
    target: "Translation"
    description: "generalizes"
  - source: "Similarity Transform"
    target: "Euclidean Transformation"
    description: "generalizes"
  - source: "Affine Transform"
    target: "Similarity Transform"
    description: "generalizes"
  - source: "Projective Transformation"
    target: "Affine Transform"
    description: "generalizes"
  - source: "Homography"
    target: "Projective Transformation"
    description: "is a type of"
  - source: "Calibration Matrix"
    target: "Camera Matrix"
    description: "is a component of"
  - source: "Camera Matrix"
    target: "Projective Transformation"
    description: "implements"
  - source: "Radial Distortion"
    target: "Projective Transformation"
    description: "deviates from"
---

# Geometric Primitives and Transformations

*An overview of 2D and 3D geometric primitives and the hierarchy of coordinate transformations used to project 3D scenes into 2D images.*

Geometric primitives and transformations provide the mathematical foundation for describing 3D scene geometry and its projection into 2D image coordinates.

## Geometric Primitives

Basic building blocks for 3D shape include [[2D Point]]s, [[2D Line]]s, and [[3D Point]]s. To simplify calculations, these are often represented using [[Homogeneous Coordinates]], where a point is denoted by a vector and scale-equivalent vectors are considered identical. 

In 2D, [[2D Conic]] sections are used for camera calibration, while in 3D, [[3D Plane]]s and [[3D Line]]s are fundamental. [[3D Quadric]] surfaces serve as the 3D analogue to conics, representing shapes like spheres and cylinders.

## Coordinate Transformations

Transformations are organized in a nested hierarchy where each more complex group preserves fewer properties than the one above it:

- **[[Translation]]**: The simplest shift in position.
- **[[Euclidean Transformation]]**: Also known as rigid body motion; preserves lengths and orientation.
- **[[Similarity Transform]]**: Preserves angles between lines.
- **[[Affine Transform]]**: Preserves parallelism.
- **[[Projective Transformation]]**: Also known as a [[Homography]]; preserves only straight lines.

## Camera Projection

The process of mapping 3D world coordinates to 2D pixels is handled by the [[Camera Matrix]], which is the product of the [[Calibration Matrix]] (intrinsics) and the extrinsic parameters (rotation and translation). While the ideal pinhole model assumes a linear projection, real-world lenses often introduce [[Radial Distortion]], which causes straight lines to appear curved in the image.

## Relationships

- [[Homogeneous Coordinates]] represents [[2D Point]]
- [[Homogeneous Coordinates]] represents [[2D Line]]
- [[Homogeneous Coordinates]] represents [[3D Point]]
- [[Homogeneous Coordinates]] represents [[3D Plane]]
- [[Euclidean Transformation]] generalizes [[Translation]]
- [[Similarity Transform]] generalizes [[Euclidean Transformation]]
- [[Affine Transform]] generalizes [[Similarity Transform]]
- [[Projective Transformation]] generalizes [[Affine Transform]]
- [[Homography]] is a type of [[Projective Transformation]]
- [[Calibration Matrix]] is a component of [[Camera Matrix]]
- [[Camera Matrix]] implements [[Projective Transformation]]
- [[Radial Distortion]] deviates from [[Projective Transformation]]
