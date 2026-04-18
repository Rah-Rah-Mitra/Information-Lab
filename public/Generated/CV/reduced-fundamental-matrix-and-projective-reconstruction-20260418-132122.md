---
type: content
title: "Reduced Fundamental Matrix and Projective Reconstruction"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:21:22.324483300+00:00
summary: "An exploration of the reduced fundamental matrix for efficient reconstruction from minimal point correspondences across multiple views."
tags:
  - computer-vision
  - projective-geometry
  - structure-from-motion
  - multi-view-geometry
entities:
  - "[[Reduced Fundamental Matrix]]"
  - "[[Fundamental Matrix]]"
  - "[[Projective Reconstruction]]"
  - "[[Reduced Trifocal Tensor]]"
  - "[[Trifocal Tensor]]"
  - "[[Camera Matrix]]"
  - "[[DLT Algorithm]]"
  - "[[Dual Fundamental Matrix]]"
  - "[[Projective Transformation]]"
  - "[[Least-Squares Solution]]"
relationships:
  - source: "Reduced Fundamental Matrix"
    target: "Fundamental Matrix"
    description: "is a specialized form of"
  - source: "Reduced Fundamental Matrix"
    target: "Projective Reconstruction"
    description: "is used to compute"
  - source: "Reduced Trifocal Tensor"
    target: "Trifocal Tensor"
    description: "is a specialized form of"
  - source: "Reduced Trifocal Tensor"
    target: "Projective Reconstruction"
    description: "is used to compute"
  - source: "Projective Reconstruction"
    target: "Camera Matrix"
    description: "determines"
  - source: "DLT Algorithm"
    target: "Camera Matrix"
    description: "is used to estimate"
  - source: "Dual Fundamental Matrix"
    target: "Reduced Fundamental Matrix"
    description: "expresses relationships between points in the same image for"
  - source: "Projective Transformation"
    target: "Reduced Fundamental Matrix"
    description: "is used to map points to a canonical basis for"
  - source: "Least-Squares Solution"
    target: "Reduced Fundamental Matrix"
    description: "is used to estimate parameters of"
---

# Reduced Fundamental Matrix and Projective Reconstruction

*An exploration of the reduced fundamental matrix for efficient reconstruction from minimal point correspondences across multiple views.*

A [[Reduced Fundamental Matrix]] is a specialized [[Fundamental Matrix]] that satisfies specific linear constraints derived from four point correspondences, allowing it to be computed from a smaller number of points (linearly from four or non-linearly from three).

## Computation and Properties

The [[Reduced Fundamental Matrix]] is parameterized to automatically satisfy constraints from synthetic correspondences, reducing the number of unknowns. When more than four correspondences are available, a [[Least-Squares Solution]] is typically employed to find the parameters. 

In the context of multi-view geometry, the [[Dual Fundamental Matrix]] is used to express relationships between points within the same image, contrasting with the standard [[Fundamental Matrix]] which encodes relationships between points in separate images.

## Application to Projective Reconstruction

The [[Reduced Fundamental Matrix]] is a key component in solving the [[Projective Reconstruction]] problem for minimal cases, such as six points in three views. The general process involves:
1. Applying a [[Projective Transformation]] to map points to a canonical basis.
2. Solving for the [[Reduced Fundamental Matrix]] parameters.
3. Extracting the [[Camera Matrix]] parameters.
4. Completing the reconstruction in the original measurement domain, often using the [[DLT Algorithm]] for final camera calibration.

## Extension to Trifocal Tensors

Similar to the fundamental matrix, a [[Reduced Trifocal Tensor]] is a [[Trifocal Tensor]] that satisfies linear constraints imposed by synthetic point correspondences. It is used for reconstruction from seven points in $n$ views. Unlike the reduced fundamental matrix, the reduced trifocal tensor does not have a simple parametrization, requiring a constrained least-squares approach to solve for its entries while ensuring synthetic constraints are satisfied exactly.

## Relationships
- [[Reduced Fundamental Matrix]] is a specialized form of [[Fundamental Matrix]]
- [[Reduced Fundamental Matrix]] is used to compute [[Projective Reconstruction]]
- [[Reduced Trifocal Tensor]] is a specialized form of [[Trifocal Tensor]]
- [[Reduced Trifocal Tensor]] is used to compute [[Projective Reconstruction]]
- [[Projective Reconstruction]] determines [[Camera Matrix]]
- [[DLT Algorithm]] is used to estimate [[Camera Matrix]]
- [[Dual Fundamental Matrix]] expresses relationships between points in the same image for [[Reduced Fundamental Matrix]]
- [[Projective Transformation]] is used to map points to a canonical basis for [[Reduced Fundamental Matrix]]
- [[Least-Squares Solution]] is used to estimate parameters of [[Reduced Fundamental Matrix]]
