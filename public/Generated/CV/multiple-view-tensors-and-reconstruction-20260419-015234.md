---
type: content
title: "Multiple View Tensors and Reconstruction"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:52:34.277541+00:00
summary: "Multiple view tensors like the trifocal and quadrifocal tensors encapsulate the geometric relationships between points and lines across multiple camera views. These tensors facilitate projective reconstruction and are computed using methods such as bundle adjustment and the factorization algorithm. The study of these tensors is central to the multi-view geometry of multi-view systems."
tags:
  - computer-vision
  - multi-view-geometry
  - tensors
  - projective-reconstruction
  - trifocal-tensor
entities:
  - "[[Trifocal Tensor]]"
  - "[[Quadrifocal Tensor]]"
  - "[[Fundamental Matrix]]"
  - "[[Bundle Adjustment]]"
  - "[[Factorization Algorithm]]"
  - "[[Trifocal Point Relations]]"
  - "[[Trifocal Line Relations]]"
  - "[[Quadrilinear Relations]]"
  - "[[Sampson Distance]]"
  - "[[Levenberg-Marquardt Algorithm]]"
relationships:
  - source: "Trifocal Tensor"
    target: "Fundamental Matrix"
    description: "generalises"
  - source: "Trifocal Tensor"
    target: "Trifocal Point Relations"
    description: "encodes"
  - source: "Trifocal Tensor"
    target: "Trifocal Line Relations"
    description: "encodes"
  - source: "Trifocal Tensor"
    target: "Bundle Adjustment"
    description: "estimated via"
  - source: "Quadrifocal Tensor"
    target: "Trifocal Tensor"
    description: "generalises"
  - source: "Trifocal Tensor"
    target: "Quadrilinear Relations"
    description: "governs"
  - source: "Trifocal Tensor"
    target: "Sampson Distance"
    description: "approximated by"
  - source: "Trifocal Tensor"
    target: "Levenberg-Marquardt Algorithm"
    description: "optimized using"
  - source: "Trifocal Tensor"
    target: "Factorization Algorithm"
    description: "is computed via"
  - source: "Trifocal Tensor"
    target: "Trifocal Point Relations"
    description: "governs"
  - source: "Trifocal Tensor"
    target: "Trifocal Line Relations"
    description: "governs"
---

# Multiple View Tensors and Reconstruction

*Multiple view tensors like the trifocal and quadrifocal tensors encapsulate the geometric relationships between points and lines across multiple camera views. These tensors facilitate projective reconstruction and are computed using methods such as bundle adjustment and the factorization algorithm. The study of these tensors is central to the multi-view geometry of multi-view systems.*

This note explores the geometric constraints and computational methods for estimating multiple view tensors, primarily the [[Trifocal Tensor]] and [[Quadrifocal Tensor]].

## Concept
Multiple view geometry relies on the existence of mathematical structures that relate image coordinates across views. While the [[Fundamental Matrix]] relates two views, the [[Trifocal Tensor]] relates three. It is a 3-way tensor that encapsulates the projective structure of three camera matrices. 

### Trifocal Point Relations
For a point correspondence across three views, the [[Trifocal Tensor]] provides trilinear relations. Given camera matrices $A, B, C$, the relation is expressed as:

$$ q_{ri}^{j} T_{j}^{uv} x_i x_j x_k = 0 $$

This relation ensures that the 3D point $X$ projects correctly to the image points $x_i, x_j, x_k$.

### Trifocal Line Relations
The tensor also relates lines in three views. For example, a point-line-line correspondence can be represented as:

$$ l_i^q r l_j^p w l_k^m T_{p}^{qr} = 0 $$

This captures the incidence of a 3D line with planes back-projected from image lines.

### Quadrilinear Relations
The [[Quadrifocal Tensor]] generalizes these properties to four views. It governs [[Quadrilinear Relations]] such as:

$$ l_p^q r l_q^s l_r^p l_s^q Q_{pqrs} = 0 $$

This relation holds for four lines in four images if they are the image of a common 3D line.

## Estimation Methods

### Bundle Adjustment
[[Bundle Adjustment]] is a non-linear optimization problem that minimizes the reprojection error across all views. It is formulated as:

$$ 	ext{min}_j 	ext{dist}(x_{ij}, P_i X_j) $$ 

This is typically solved using the [[Levenberg-Marquardt Algorithm]], which is a robust iterative method for non-linear least squares.

### Factorization Algorithm
For affine cameras, the [[Factorization Algorithm]] provides a closed-form solution via Singular Value Decomposition (SVD). It seeks a rank-3 matrix $W$ that is closest to the measurement matrix in Frobenius norm.

## Relationships
- [[Trifocal Tensor]] generalises [[Fundamental Matrix]]
- [[Trifocal Tensor]] encodes [[Trifocal Point Relations]]
- [[Trifocal Tensor]] encodes [[Trifocal Line Relations]]
- [[Trifocal Tensor]] estimated via [[Bundle Adjustment]]
- [[Quadrifocal Tensor]] generalises [[Trifocal Tensor]]
- [[Trifocal Tensor]] governs [[Quadrilinear Relations]]
- [[Trifocal Tensor]] approximated by [[Sampson Distance]]
- [[Trifocal Tensor]] optimized using [[Levenberg-Marquardt Algorithm]]
- [[Trifocal Tensor]] is computed via [[Factorization Algorithm]]
