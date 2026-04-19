---
type: content
title: "Fundamental Matrix Estimation and Epipolar Geometry"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:51:19.743957400+00:00
summary: "The fundamental matrix is a $3 	imes 3$ matrix that encapsulates the 2-view epipolar geometry between two uncalibrated cameras. Estimation methods range from simple linear 8-point algorithms to robust Maximum Likelihood estimates using geometric distance. Understanding its singularity and the covariance of its epipoles is crucial for reliable point matching and image rectification."
tags:
  - computer-vision
  - epipolar-geometry
  - fundamental-matrix
  - multiple-view-geometry
entities:
  - "[[Fundamental Matrix]]"
  - "[[Epipolar Geometry]]"
  - "[[Epipole]]"
  - "[[8-Point Algorithm]]"
  - "[[Epipolar Line]]"
  - "[[Maximum Likelihood Estimation]]"
  - "[[Sampson Distance]]"
  - "[[Image Rectification]]"
  - "[[RANSAC]]"
  - "[[Absolute Conic]]"
relationships:
  - source: "Fundamental Matrix"
    target: "Epipolar Geometry"
    description: "defines"
  - source: "Fundamental Matrix"
    target: "Epipole"
    description: "generates"
  - source: "Fundamental Matrix"
    target: "Epipolar Line"
    description: "produces"
  - source: "Fundamental Matrix"
    target: "8-Point Algorithm"
    description: "estimated via"
  - source: "Fundamental Matrix"
    target: "Maximum Likelihood Estimation"
    description: "optimized via"
  - source: "Fundamental Matrix"
    target: "Sampson Distance"
    description: "approximated by"
  - source: "Fundamental Matrix"
    target: "Image Rectification"
    description: "enables"
  - source: "Fundamental Matrix"
    target: "RANSAC"
    description: "robustly estimated using"
  - source: "Fundamental Matrix"
    target: "Absolute Conic"
    description: "related to through calibration-based reconstruction"
---

# Fundamental Matrix Estimation and Epipolar Geometry

*The fundamental matrix is a $3 imes 3$ matrix that encapsulates the 2-view epipolar geometry between two uncalibrated cameras. Estimation methods range from simple linear 8-point algorithms to robust Maximum Likelihood estimates using geometric distance. Understanding its singularity and the covariance of its epipoles is crucial for reliable point matching and image rectification.*

This note explores the mathematical foundations and estimation techniques for the [[Fundamental Matrix]], which is central to multiple-view geometry.

## Concept
The [[Fundamental Matrix]] $F$ is a $3 	imes 3$ matrix that relates matching points $x$ and $x'$ in two images such that they satisfy the epipolar constraint:

$$ x'^T F x = 0 $$

This equation ensures that a point in the first image projects to an epipolar line in the second image. The matrix $F$ has rank 2 and is singular, meaning its determinant is zero. The null-spaces of $F$ and $F^T$ represent the two [[Epipole]]s in the respective images.

## Estimation Methods

### Linear Methods
One of the simplest approaches is the [[8-Point Algorithm]], which uses a set of at least 8 point correspondences to solve a set of linear equations. To ensure stability, the [[Normalized 8-Point Algorithm]] is recommended, which involves translating and scaling the points so the centroid is at the origin and the RMS distance is $\sqrt{2}$.

### Non-linear and Robust Methods
For higher accuracy, one can minimize the [[Sampson Distance]], which provides a first-order approximation to the geometric error. The "Gold Standard" method involves [[Maximum Likelihood Estimation]] (MLE) to minimize the reprojection error, which is the geometric distance between the measured points and their estimated true correspondences. This is often implemented using the [[RANSAC]] algorithm to handle outliers and provide a robust initial estimate.

## Epipolar Geometry and Rectification
The [[Epipolar Line]] $l' = Fx$ is the line in the second image that contains the corresponding point. The uncertainty in the estimation of $F$ can be characterized by the covariance matrix, which defines an [[Epipolar Envelope]] (a hyperbola or conic) within which a certain fraction of the most likely epipolar lines lie. 

[[Image Rectification]] is the process of resampling images so that the epipolar lines are horizontal and parallel to the x-axis. This simplifies stereo matching by reducing the search to a 1D horizontal search. This process is achieved by applying projective transformations to both images to map the epipoles to infinity.

## Relationships
- [[Fundamental Matrix]] defines [[Epipolar Geometry]]
- [[Fundamental Matrix]] produces [[Epipole]]
- [[Fundamental Matrix]] produces [[Epipolar Line]]
- [[Fundamental Matrix]] estimated via [[8-Point Algorithm]]
- [[Fundamental Matrix]] optimized via [[Maximum Likelihood Estimation]]
- [[Fundamental Matrix]] approximated by [[Sampson Distance]]
- [[Fundamental Matrix]] enables [[Image Rectification]]
- [[Fundamental Matrix]] robustly estimated using [[RANSAC]]
- [[Fundamental Matrix]] related to [[Absolute Conic]] through calibration-based reconstruction
