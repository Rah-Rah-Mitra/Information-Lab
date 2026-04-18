---
type: content
title: "Camera Calibration from Single View"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:18:09.592147300+00:00
summary: "Explains how to determine the camera calibration matrix K from scene and internal constraints using the image of the absolute conic."
tags:
  - computer-vision
  - camera-calibration
  - single-view-geometry
  - projective-geometry
entities:
  - "[[Camera Calibration Matrix]]"
  - "[[Image of the Absolute Conic]]"
  - "[[Vanishing Point]]"
  - "[[Vanishing Line]]"
  - "[[Homography]]"
  - "[[Zero Skew]]"
  - "[[Square Pixels]]"
  - "[[Cholesky Factorization]]"
  - "[[SVD]]"
  - "[[Calibrating Conic]]"
  - "[[Principal Point]]"
  - "[[Focal Length]]"
relationships:
  - source: "Camera Calibration Matrix"
    target: "Image of the Absolute Conic"
    description: "is derived from"
  - source: "Vanishing Point"
    target: "Image of the Absolute Conic"
    description: "provides linear constraints on"
  - source: "Vanishing Line"
    target: "Image of the Absolute Conic"
    description: "provides linear constraints on"
  - source: "Homography"
    target: "Image of the Absolute Conic"
    description: "provides linear constraints on"
  - source: "Zero Skew"
    target: "Image of the Absolute Conic"
    description: "imposes internal constraints on"
  - source: "Square Pixels"
    target: "Image of the Absolute Conic"
    description: "imposes internal constraints on"
  - source: "SVD"
    target: "Image of the Absolute Conic"
    description: "is used to solve for"
  - source: "Cholesky Factorization"
    target: "Camera Calibration Matrix"
    description: "is used to decompose the IAC into"
  - source: "Calibrating Conic"
    target: "Camera Calibration Matrix"
    description: "visualizes the parameters of"
  - source: "Principal Point"
    target: "Camera Calibration Matrix"
    description: "is a parameter of"
  - source: "Focal Length"
    target: "Camera Calibration Matrix"
    description: "is a parameter of"
---

# Camera Calibration from Single View

*Explains how to determine the camera calibration matrix K from scene and internal constraints using the image of the absolute conic.*

Camera calibration from a single view involves estimating the [[Camera Calibration Matrix]] (K) by determining the [[Image of the Absolute Conic]] (IAC, denoted as $\omega$).

## Constraints on the IAC

The IAC is an imaginary conic that provides a bridge between the image plane and the 3D world. Various constraints can be used to solve for its entries:

### Scene Constraints
- **Orthogonality**: [[Vanishing Point]]s corresponding to orthogonal lines in the scene provide linear constraints on $\omega$.
- **Metric Planes**: A metric plane imaged with a known [[Homography]] provides constraints on the IAC.
- **Vanishing Lines**: [[Vanishing Line]]s corresponding to orthogonal planes provide further constraints.

### Internal Constraints
- **[[Zero Skew]]**: The condition that the image axes are orthogonal, which simplifies the entries of $\omega$.
- **[[Square Pixels]]**: The condition that the pixel aspect ratio is unity, providing additional linear constraints.

## Estimation Procedure

To compute the calibration, the constraints are represented as linear equations in the form $Aw = 0$, where $w$ is a 6-vector of the distinct entries of $\omega$. This system is typically solved using [[SVD]] to find the null vector. Once $\omega$ is determined, the [[Camera Calibration Matrix]] is recovered via [[Cholesky Factorization]] and matrix inversion.

## The Calibrating Conic

The [[Calibrating Conic]] is a visible conic used for visualization and geometric construction. It is the image of a cone with a 45-degree apex angle whose axis is the principal axis of the camera. The [[Principal Point]] of the camera is the center of this conic, and the [[Focal Length]] can be read from its scale.

## Relationships
- [[Camera Calibration Matrix]] is derived from [[Image of the Absolute Conic]].
- [[Vanishing Point]] provides linear constraints on [[Image of the Absolute Conic]].
- [[Vanishing Line]] provides linear constraints on [[Image of the Absolute Conic]].
- [[Homography]] provides linear constraints on [[Image of the Absolute Conic]].
- [[Zero Skew]] imposes internal constraints on [[Image of the Absolute Conic]].
- [[Square Pixels]] imposes internal constraints on [[Image of the Absolute Conic]].
- [[SVD]] is used to solve for [[Image of the Absolute Conic]].
- [[Cholesky Factorization]] is used to decompose the IAC into [[Camera Calibration Matrix]].
- [[Calibrating Conic]] visualizes the parameters of [[Camera Calibration Matrix]].
- [[Principal Point]] is a parameter of [[Camera Calibration Matrix]].
- [[Focal Length]] is a parameter of [[Camera Calibration Matrix]].
