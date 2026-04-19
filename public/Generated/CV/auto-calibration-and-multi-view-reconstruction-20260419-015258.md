---
type: content
title: "Auto-Calibration and Multi-View Reconstruction"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:52:58.064014600+00:00
summary: "Auto-calibration is the process of determining internal camera parameters from uncalibrated image sequences to enable metric reconstruction. It utilizes constraints on the absolute conic and absolute dual quadric to bridge the gap between projective and metric geometry. The method is highly effective for recovering structure and motion from video sequences through iterative refinement."
tags:
  - computer-vision
  - camera-calibration
  - multi-view-geometry
  - projective-reconstruction
entities:
  - "[[Auto-Calibration]]"
  - "[[Metric Reconstruction]]"
  - "[[Projective Reconstruction]]"
  - "[[Absolute Dual Quadric]]"
  - "[[Absolute Conic]]"
  - "[[Absolute Dual Conic]]"
  - "[[Kruppa Equations]]"
  - "[[Plane at Infinity]]"
  - "[[Infinite Homography]]"
  - "[[Non-rigid Factorization]]"
relationships:
  - source: "Auto-Calibration"
    target: "Metric Reconstruction"
    description: "enables"
  - source: "Auto-Calibration"
    target: "Projective Reconstruction"
    description: "transforms"
  - source: "Auto-Calibration"
    target: "Absolute Dual Quadric"
    description: "uses"
  - source: "Auto-Calibration"
    target: "Absolute Conic"
    description: "uses"
  - source: "Auto-Calibration"
    target: "Kruppa Equations"
    description: "employs"
  - source: "Auto-Calibration"
    target: "Plane at Infinity"
    description: "requires"
  - source: "Auto-Calibration"
    target: "Infinite Homography"
    description: "utilizes"
  - source: "Auto-Calibration"
    target: "Non-rigid Factorization"
    description: "relates to"
  - source: "Absolute Dual Quadric"
    target: "Absolute Conic"
    description: "represents"
  - source: "Plane at Infinity"
    target: "Infinite Homography"
    description: "induces"
---

# Auto-Calibration and Multi-View Reconstruction

*Auto-calibration is the process of determining internal camera parameters from uncalibrated image sequences to enable metric reconstruction. It utilizes constraints on the absolute conic and absolute dual quadric to bridge the gap between projective and metric geometry. The method is highly effective for recovering structure and motion from video sequences through iterative refinement.*

This note explores the computational methods for recovering camera parameters and scene structure from multiple views, specifically focusing on the transition from projective to metric reconstruction.

## Concept
[[Auto-Calibration]] is the process of determining internal camera parameters (the calibration matrix $K$) directly from uncalibrated image sequences. This is essential for transforming a [[Projective Reconstruction]] into a [[Metric Reconstruction]]. In a projective frame, camera matrices $P$ are related to metric matrices $M$ by a homography $H$ such that $P = MH$. The goal of auto-calibration is to find this $H$ to recover the metric properties of the scale, rotation, and translation.

### The Absolute Conic and Dual Quadric
Central to auto-calibration are the [[Absolute Conic]] (IAC) and the [[Absolute Dual Quadric]] (DIAC). The [[Absolute Dual Quadric]] $Q$ is a degenerate dual quadric that encodes the camera's internal parameters. In a projective frame, it is represented by a symmetric $4 	imes 4$ matrix of rank 3. Its image projection is the [[Absolute Dual Conic]] $\omega$, which is the dual image of the absolute conic. The relationship is given by:

$$ \omega = P^T Q P $$

This equation allows constraints on the internal parameters to be transferred to constraints on $Q$. Once $Q$ is estimated, the rectifying homography $H$ can be extracted via the decomposition $Q = HIH^T$.

### The Kruppa Equations
An alternative approach involves the [[Kruppa Equations]], which are two-view constraints based on the correspondence of epipolar tangent lines to a conic. These equations are quadratic in the elements of the image of the absolute conic $\omega$. They are expressed as:

$$ \omega'^T e F \omega F e \omega = 0 $$

where $e$ and $e'$ are the epipoles and $F$ is the fundamental matrix. While the Kruppa equations provide a way to estimate $\omega$ directly, they are often considered weaker than methods using the absolute dual quadric because they do not enforce the degeneracy of $Q$.

### Non-Rigid Factorization
In scenes where objects deform, [[Non-rigid Factorization]] is used. This method models deformation as a linear combination of basis shapes, allowing the recovery of structure and motion even when the rigid scene assumption is violated. This is achieved by decomposing a measurement matrix $W$ into a motion matrix $M$ and a structure matrix $B$.

## Relationships
- [[Auto-Calibration]] transforms [[Projective Reconstruction]] to [[Metric Reconstruction]]
- [[Auto-Calibration]] uses [[Absolute Dual Quadric]] to find [[Infinite Homography]]
- [[Kruppa Equations]] are constraints on the [[Absolute Dual Conic]]
- [[Plane at Infinity]] induces [[Infinite Homography]]
