---
type: content
title: "Index of Computer Vision Concepts"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:24:09.065033600+00:00
summary: "A comprehensive index of key terms and topics in computer vision, including projective geometry, trifocal tensors, and triangulation."
tags:
  - computer-vision
  - geometry
  - index
entities:
  - "[[Projective Geometry]]"
  - "[[Trifocal Tensor]]"
  - "[[Triangulation]]"
  - "[[Singular Value Decomposition]]"
  - "[[Vanishing Points]]"
  - "[[Vanishing Lines]]"
  - "[[Stereo Rectification]]"
  - "[[Twisted Cubic]]"
relationships:
  - source: "Projective Geometry"
    target: "Vanishing Points"
    description: "defines the framework for"
  - source: "Projective Geometry"
    target: "Vanishing Lines"
    description: "defines the framework for"
  - source: "Projective Geometry"
    target: "Twisted Cubic"
    description: "includes the concept of"
  - source: "Trifocal Tensor"
    target: "Projective Geometry"
    description: "is a tool within"
  - source: "Triangulation"
    target: "Singular Value Decomposition"
    description: "often utilizes for numerical solution"
  - source: "Stereo Rectification"
    target: "Projective Geometry"
    description: "applies transformations from"
  - source: "Trifocal Tensor"
    target: "Triangulation"
    description: "can be used to facilitate"
---

# Index of Computer Vision Concepts

*A comprehensive index of key terms and topics in computer vision, including projective geometry, trifocal tensors, and triangulation.*

This document serves as an index of key concepts in [[Projective Geometry]] and computer vision. It lists various mathematical tools and [[Singular Value Decomposition]] for solving linear systems, and geometric constructions such as [[Vanishing Points]] and [[Vanishing Lines]].

## Key Concepts

### Projective Geometry
[[Projective Geometry]] provides the foundation for understanding how 3D scenes are projected onto 2D images, encompassing concepts like the [[Twisted Cubic]] and the behavior of points at infinity.

### Multi-View Geometry
Concepts such as the [[Trifocal Tensor]] are used to describe the relations between three views of a scene, while [[Triangulation]] is the process of determining a 3D point from its projections in multiple images.

### Stereo Vision
[[Stereo Rectification]] is used to align images to simplify the correspondence search, often relying on the principles of projective transformations.

## Relationships

- Projective Geometry defines the framework for Vanishing Points
- Projective Geometry defines the framework for Vanishing Lines
- Projective Geometry includes the concept of Twisted Cubic
- Trifocal Tensor is a tool within Projective Geometry
- Triangulation often utilizes for numerical solution Singular Value Decomposition
- Stereo Rectification applies transformations from Projective Geometry
- Trifocal Tensor can be used to facilitate Triangulation
