---
type: content
title: "Estimation of 2D Projective Transformations"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:16:19.765256100+00:00
summary: "An overview of algorithms for estimating 2D homographies, covering DLT, normalization, iterative minimization, and robust estimation via RANSAC."
tags:
  - computer-vision
  - geometry
  - linear-algebra
  - statistics
entities:
  - "[[2D Homography]]"
  - "[[Direct Linear Transformation]]"
  - "[[Maximum Likelihood Estimation]]"
  - "[[Mahalanobis Distance]]"
  - "[[RANSAC]]"
  - "[[Sampson Approximation]]"
  - "[[Levenberg-Marquardt Algorithm]]"
  - "[[Reprojection Error]]"
  - "[[Symmetric Transfer Error]]"
  - "[[Data Normalization]]"
  - "[[Isotropic Scaling]]"
  - "[[Gold Standard Algorithm]]"
relationships:
  - source: "Direct Linear Transformation"
    target: "2D Homography"
    description: "estimates"
  - source: "Data Normalization"
    target: "Direct Linear Transformation"
    description: "improves accuracy of"
  - source: "Isotropic Scaling"
    target: "Data Normalization"
    description: "is a method of"
  - source: "Maximum Likelihood Estimation"
    target: "2D Homography"
    description: "provides optimal estimate of"
  - source: "Reprojection Error"
    target: "Maximum Likelihood Estimation"
    description: "is a cost function for"
  - source: "Symmetric Transfer Error"
    target: "Maximum Likelihood Estimation"
    description: "is a cost function for"
  - source: "Sampson Approximation"
    target: "Reprojection Error"
    description: "approximates"
  - source: "Levenberg-Marquardt Algorithm"
    target: "Maximum Likelihood Estimation"
    description: "is used to minimize"
  - source: "RANSAC"
    target: "2D Homography"
    description: "robustly estimates"
  - source: "Gold Standard Algorithm"
    target: "Reprojection Error"
    description: "minimizes"
  - source: "Mahalanobis Distance"
    target: "Maximum Likelihood Estimation"
    description: "generalizes distance for"
---

# Estimation of 2D Projective Transformations

*An overview of algorithms for estimating 2D homographies, covering DLT, normalization, iterative minimization, and robust estimation via RANSAC.*

The estimation of a [[2D Homography]] is the process of determining the projective transformation matrix that maps points from one image to another.

## Estimation Methods

### Linear Estimation
The [[Direct Linear Transformation]] (DLT) is a fundamental linear algorithm for estimating homographies. However, the basic DLT is not invariant to similarity transformations, meaning the result depends on the the choice of coordinate origin and scale. To resolve this, [[Data Normalization]] is essential. This typically involves [[Isotropic Scaling]], where points are translated so their centroid is at the origin and scaled so the average distance from the origin is $\sqrt{2}$.

### Non-Linear and Optimal Estimation
[[Maximum Likelihood Estimation]] (MLE) provides the theoretically optimal estimate by minimizing a geometric cost function. Common cost functions include:
- [[Symmetric Transfer Error]]: The sum of distances between points and their transformations in both images.
- [[Reprojection Error]]: The most accurate but computationally expensive measure, as it requires simultaneous minimization over the transformation and the corrected point positions.
- [[Sampson Approximation]]: A first-order approximation to the reprojection error that allows minimization over only the homography parameters rather than all point coordinates.

The [[Levenberg-Marquardt Algorithm]] is typically used to iteratively minimize these non-linear cost functions. The [[Gold Standard Algorithm]] specifically refers to the process of minimizing the full reprojection error.

## Robust Estimation
In practical scenarios, point correspondences often contain outliers (mismatches). [[RANSAC]] (Random Sample Consensus) is used to partition the data into inliers and outliers. It works by randomly selecting a minimal subset of points (four for a homography), instantiating a model, and counting the number of points that fall within a distance threshold (the consensus set).

## Error Analysis
When using Gaussian noise, the [[Mahalanobis Distance]] is used to generalize the cost function for non-isotropic error distributions. The performance of an estimator can be evaluated by comparing its residual error (the distance between measured and estimated values) and estimation error (the distance between estimated and true values) against the theoretical bounds of the MLE.

## Relationships
- [[Direct Linear Transformation]] estimates [[2D Homography]].
- [[Data Normalization]] improves accuracy of [[Direct Linear Transformation]].
- [[Isotropic Scaling]] is a method of [[Data Normalization]].
- [[Maximum Likelihood Estimation]] provides optimal estimate of [[2D Homography]].
- [[Reprojection Error]] is a cost function for [[Maximum Likelihood Estimation]].
- [[Symmetric Transfer Error]] is a cost function for [[Maximum Likelihood Estimation]].
- [[Sampson Approximation]] approximates [[Reprojection Error]].
- [[Levenberg-Marquardt Algorithm]] is used to minimize [[Maximum Likelihood Estimation]].
- [[RANSAC]] robustly estimates [[2D Homography]].
- [[Gold Standard Algorithm]] minimizes [[Reprojection Error]].
- [[Mahalanobis Distance]] generalizes distance for [[Maximum Likelihood Estimation]].
