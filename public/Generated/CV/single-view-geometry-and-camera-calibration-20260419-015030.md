---
type: content
title: "Single View Geometry and Camera Calibration"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:50:30.996798200+00:00
summary: "This note explores the projective geometry of how cameras map 3D entities like points, lines, conics, and quadrics into 2D images. It details the mechanisms of camera calibration using the image of the absolute conic and the image of the absolute conic to recover internal parameters. The text also covers vanishing points and lines as tools for metric reconstruction and metric properties from single views."
tags:
  - computer-vision
  - projective-geometry
  - camera-calibration
  - single-view-geometry
entities:
  - "[[Single View Geometry]]"
  - "[[Camera Calibration]]"
  - "[[Camera Matrix]]"
  - "[[Image of the Absolute Conic]]"
  - "[[Vanishing Point]]"
  - "[[Timestamp]]"
  - "[[Vanishing Line]]"
  - "[[Quadric]]"
  - "[[Contour Generator]]"
  - "[[Contour Apparent]]"
  - "[[Plucker Line Representation]]"
  - "[[Calibrating Conic]]"
relationships:
  - source: "Single View Geometry"
    target: "Camera Matrix"
    description: "is modeled by"
  - source: "Camera Matrix"
    target: "Vanishing Point"
    description: "produces"
  - source: "Camera Matrix"
    target: "Vanishing Line"
    description: "produces"
  - source: "Camera Matrix"
    target: "Image of the Absolute Conic"
    description: "determines"
  - source: "Vanishing Point"
    target: "Single View Geometry"
    description: "is a feature of"
  - source: "Vanishing Line"
    target: "Single View Geometry"
    description: "is a feature of"
  - source: "Quadric"
    target: "Contour Generator"
    description: "has a"
  - source: "Contour Generator"
    target: "Contour Apparent"
    description: "defines"
  - source: "Image of the Absolute Conic"
    target: "Camera Calibration"
    description: "enables"
  - source: "Calibrating Conic"
    target: "Camera Calibration"
    description: "is used for"
---

# Single View Geometry and Camera Calibration

*This note explores the projective geometry of how cameras map 3D entities like points, lines, conics, and quadrics into 2D images. It details the mechanisms of camera calibration using the image of the absolute conic and the image of the absolute conic to recover internal parameters. The text also covers vanishing points and lines as tools for metric reconstruction and metric properties from single views.*

This note covers the fundamental principles of [[Single View Geometry]] and the mathematical framework for [[Camera Calibration]].

## Concept
In perspective imaging, a camera is represented by a [[Camera Matrix]] $P$, which maps 3D points to 2D image points. The geometry of how various entities project is central to understanding how we can recover 3D structure or camera parameters from a single image.

### Projection of Planes and Lines
When a scene plane is aligned with the world coordinate frame, the mapping between points on that plane and their image is a [[Planar Homography]] $H$. For a point $X$ on the plane $\pi$, the image $x$ is given by:

$$ x = Hx $$

This shows that the most general transformation between a scene plane and an image plane is a plane projective transformation.

Lines in 3-space project to lines in the image. A line $L$ in 3-space is imaged as a line $l$ in the image. Using the [[Plucker Line Representation]], a line can be represented by a 6-vector $L$. The [[Line Projection Matrix]] $\tilde{P}$ maps these coordinates to the image line $l$:

$$ l = 	ilde{P}L $$

### Vanishing Points and Lines
[[Vanishing Point]]s are the images of points at infinity. A line with direction $d$ in 3-space has a vanishing point $v$ given by:

$$ v = Kd $$nThis represents the intersection of the image plane with a a ray parallel to the world line through the camera centre.

[[Vanishing Line]]s are the images of parallel planes. A plane $\pi$ with normal $n$ has a vanishing line $l$ given by:

$$ l = Kn \nThis line partitions the image and represents the orientation of the scene plane.

### Conics and Quadrics
For a [[Quadric]] $Q$, the [[Contour Generator]] $\header{\Gamma}$ is the set of points on the surface where the imaging rays are tangent. The [[Contour Apparent]] $\gamma$ is the image of this generator. For a quadric, the contour generator is a plane conic, and the image (the apparent contour) is also a conic.

The relationship between the tangent planes and the camera is captured by the [[Calibrating Conic]] $\mathcal{C}$, which is an affine transformation of a unit circle. The [[Image of the Absolute Conic]] $oldsymbol{\omega}$ allows for the measurement of angles and orthogonality in the image, which is directly related to the camera's internal calibration matrix $K$.

## Relationships
- [[Single View Geometry]] is modeled by [[Camera Matrix]]
- [[Camera Matrix]] produces [[Vanishing Point]]
- [[Camera Matrix]] produces [[Vanishing Line]]
- [[Camera Matrix]] determines [[Image of the Absolute Conic]]
- [[Vanishing Point]] is a feature of [[Single View Geometry]]
- [[Vanishing Line]] is a feature of [[Single View Geometry]]
- [[Quadric]] has a [[Contour Generator]]
- [[Contour Generator]] defines [[Contour Apparent]]
- [[Image of the Absolute Conic]] enables [[Camera Calibration]]
- [[Calibrating Conic]] is used for [[Camera Calibration]]
