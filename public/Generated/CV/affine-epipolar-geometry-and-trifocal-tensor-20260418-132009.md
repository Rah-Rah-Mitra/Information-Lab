---
type: content
title: "Affine Epipolar Geometry and Trifocal Tensor"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:20:09.159947+00:00
summary: "An exploration of affine epipolar geometry, the affine fundamental matrix, and the trifocal tensor for three-view geometric relations."
tags:
  - computer-vision
  - epipolar-geometry
  - multi-view-geometry
  - affine-projection
entities:
  - "[[Affine Epipolar Geometry]]"
  - "[[Affine Fundamental Matrix]]"
  - "[[Epipole]]"
  - "[[Epipolar Line]]"
  - "[[Trifocal Tensor]]"
  - "[[Trilinearities]]"
  - "[[Point Transfer]]"
  - "[[Line Transfer]]"
  - "[[Trifocal Plane]]"
  - "[[Bas-Relief Ambiguity]]"
  - "[[Necker Reversal]]"
  - "[[Sampson Error]]"
relationships:
  - source: "Affine Epipolar Geometry"
    target: "Affine Fundamental Matrix"
    description: "is characterized by the"
  - source: "Affine Fundamental Matrix"
    target: "Epipole"
    description: "encodes the position of"
  - source: "Affine Fundamental Matrix"
    target: "Epipolar Line"
    description: "defines the"
  - source: "Trifocal Tensor"
    target: "Trilinearities"
    description: "expresses through"
  - source: "Trifocal Tensor"
    target: "Point Transfer"
    description: "enables"
  - source: "Trifocal Tensor"
    target: "Line Transfer"
    description: "enables"
  - source: "Point Transfer"
    target: "Trifocal Plane"
    description: "becomes degenerate on the"
  - source: "Affine Epipolar Geometry"
    target: "Bas-Relief Ambiguity"
    description: "suffers from"
  - source: "Affine Epipolar Geometry"
    target: "Necker Reversal"
    description: "suffers from"
  - source: "Affine Fundamental Matrix"
    target: "Sampson Error"
    description: "is estimated using"
---

# Affine Epipolar Geometry and Trifocal Tensor

*An exploration of affine epipolar geometry, the affine fundamental matrix, and the trifocal tensor for three-view geometric relations.*

[[Affine Epipolar Geometry]] is the study of the geometric constraints between two images acquired under affine imaging conditions, where parallel projection is assumed.

## Affine Fundamental Matrix

The [[Affine Fundamental Matrix]] is a rank-2 homogeneous matrix with 4 degrees of freedom. It relates corresponding points in two affine views such that for any pair of matching points $\mathbf{x}$ and $\mathbf{x}'$, the relation $\mathbf{x}^T F \mathbf{x}' = 0$ holds. 

### Properties
- **Epipoles**: The [[Epipole]] in the first view is the right null-vector of $F$. In affine geometry, all [[Epipolar Line]]s are parallel.
- **Epipolar Lines**: The epipolar line in the second view corresponding to a point $\mathbf{x}$ in the first is given by $\mathbf{l}' = F \mathbf{x}$.

## Estimation and Ambiguities

Estimation of the affine fundamental matrix can be performed via a linear algorithm (requiring 4 point correspondences) or the "Gold Standard" algorithm, which minimizes geometric image distances. A key advantage of the affine form is that the [[Sampson Error]] is identical to the geometric error.

### Motion Ambiguities
Under parallel projection, two primary ambiguities arise:
1. **[[Necker Reversal]]**: A reflection ambiguity where an object and its mirror image rotating in opposite senses generate the same image.
2. **[[Bas-Relief Ambiguity]]**: A depth-turn ambiguity where the product of the rotation angle and depth is stable, but the individual parameters are confounded.

## The Trifocal Tensor

While the fundamental matrix handles two views, the [[Trifocal Tensor]] encapsulates the projective geometric relations between three views independent of scene structure. It consists of three $3 \times 3$ matrices and has 18 independent degrees of freedom.

### Trilinearities and Transfer
The tensor defines [[Trilinearities]], which are linear relations in the coordinates of image elements (points and lines) across three views. These are used for:
- **[[Point Transfer]]**: Determining the position of a point in a third view given its correspondence in the first two. Unlike epipolar transfer, which fails for points on the [[Trifocal Plane]] (the plane defined by the three camera centres), trifocal tensor transfer is generally robust.
- **[[Line Transfer]]**: Computing the image of a 3D line in one view given its images in the other two.

## Relationships
- Affine Epipolar Geometry is characterized by the Affine Fundamental Matrix.
- Affine Fundamental Matrix encodes the position of the Epipole and defines the Epipolar Line.
- Trifocal Tensor expresses through Trilinearities.
- Trifocal Tensor enables Point Transfer and Line Transfer.
- Point Transfer becomes degenerate on the Trifocal Plane.
- Affine Epipolar Geometry suffers from the Bas-Relief Ambiguity and Necker Reversal.
- Affine Fundamental Matrix is estimated using the Sampson Error.
