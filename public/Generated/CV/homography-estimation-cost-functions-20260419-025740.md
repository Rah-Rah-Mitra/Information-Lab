---
type: content
title: "Homography Estimation Cost Functions"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:57:40.312078+00:00
summary: "This note explores various cost functions used to estimate 2D homographies, ranging from algebraic to geometric and statistical approaches. It details the differences between algebraic distance, transfer error, symmetric transfer error, reprojection error, and Sampson error. The choice of function impacts computational complexity, statistical optimality, and invariance to coordinate transformations."
tags:
  - computer-vision
  - homography-estimation
  - optimization
  - projective-geometry
entities:
  - "[[Homography]]"
  - "[[Algebraic Distance]]"
  - "[[Geometric Distance]]"
  - "[[Transfer Error]]"
  - "[[Symmetric Transfer Error]]"
  - "[[Reprojection Error]]"
  - "[[Sampson Error]]"
  - "[[Maximum Likelihood Estimation]]"
  - "[[DLT Algorithm]]"
  - "[[Degeneracy]]"
  - "[[Similarity Transformation]]"
  - "[[Mahalanobis Distance]]"
relationships:
  - source: "Homography"
    target: "Algebraic Distance"
    description: "can be estimated using"
  - source: "Homography"
    target: "Geometric Distance"
    description: "can be estimated using"
  - source: "Homography"
    target: "Sampson Error"
    description: "can be estimated using"
  - source: "Homography"
    target: "DLT Algorithm"
    description: "is estimated by"
  - source: "Homography"
    target: "Maximum Likelihood Estimation"
    description: "is estimated via"
  - source: "Homography"
    target: "Degeneracy"
    description: "is subject to"
  - source: "Geometric Distance"
    target: "Transfer Error"
    description: "specialises to"
  - source: "Geometric Distance"
    target: "Symmetric Transfer Error"
    description: "specialises to"
  - source: "Geometric Distance"
    target: "Reprojection Error"
    description: "specialises to"
  - source: "Sampson Error"
    target: "Geometric Distance"
    description: "approximates"
  - source: "DLT Algorithm"
    target: "Algebraic Distance"
    description: "minimises"
  - source: "Maximum Likelihood Estimation"
    target: "Geometric Distance"
    description: "is equivalent to minimizing"
  - source: "Similarity Transformation"
    target: "Homography"
    description: "affects invariance of"
  - source: "Mahalanobis Distance"
    target: "Maximum Likelihood Estimation"
    description: "is used in general Gaussian cases for"
---

# Homography Estimation Cost Functions

*This note explores various cost functions used to estimate 2D homographies, ranging from algebraic to geometric and statistical approaches. It details the differences between algebraic distance, transfer error, symmetric transfer error, reprojection error, and Sampson error. The choice of function impacts computational complexity, statistical optimality, and invariance to coordinate transformations.*

This note details the various mathematical frameworks used to estimate a [[Homography]] from point correspondences.

## Concept
A [[Homography]] is a projective transformation that maps points from one image plane to another. Estimating this transformation requires defining a cost function to minimize the error between measured and estimated correspondences. The choice of cost function determines the algorithm's computational efficiency, statistical properties, and invariance to coordinate systems.

### Algebraic Distance
[[Algebraic Distance]] is the norm of the residual vector in the Direct Linear Transformation (DLT) algorithm. For a point correspondence \(x_i \leftrightarrow x'_i\), the condition is expressed as \(x'_i H x_i = 0\). The algebraic error is defined as:

$$ d_{alg}^2 = ||A_i h||^2 $$

This method is computationally cheap and provides a linear solution, but it lacks direct geometric or statistical meaning. However, with proper normalization, it provides excellent results.

### Geometric Distance
[[Geometric Distance]] refers to errors measured in image coordinates. Unlike algebraic distance, these are physically meaningful. Several types exist:

- **[[Transfer Error]]**: The Euclidean distance in the second image between the measured point \(x'_i\) and the mapped point \(H x_i\). It is defined as:
  $$ d(x'_i, H x_i) = ||x'_i - H x_i|| $$
- **[[Symmetric Transfer Error]]**: Sums the transfer errors in both images, considering both forward and backward transformations:
  $$ d_{sym} = d(x'_i, H x_i) + d(x'_i, H^{-1} x'_i) $$
- **[[Reprojection Error]]**: A more complex error function that seeks to minimize the distance between measured points and a set of perfectly matched points that satisfy the homography. This is equivalent to finding the closest point on a variety in 4D space.

### Sampson Error
The [[Sampson Error]] is a first-order approximation to the geometric error. It provides a middle ground between the simplicity of the algebraic error and the complexity of the non-linear geometric error. For a single point pair, it is given by:

$$ d_{Sampson}^2 = (C(X)^T J J^T C(X))^{-1} $$ 

where $J$ is the partial-derivative matrix. It is often used as a starting point for non-linear optimization.

### Statistical Optimality
[[Maximum Likelihood Estimation]] (MLE) is the framework for finding the optimal estimate under a specific noise model. If image measurement errors follow a zero-mean isotropic Gaussian distribution, the MLE is equivalent to minimizing the geometric error (transfer error). In the general case with a covariance matrix 
$\Sigma$, the minimization of the [[Mahalanobis Distance]] is required.

### Degeneracy and Invariance
[[Degeneracy]] occurs when a configuration of points or lines does not uniquely determine a homography (e.g., when points are collinear). The [[DLT Algorithm]] is not invariant to similarity transformations unless the data is normalized. [[Similarity Transformation]] invariance is a key property of geometric error minimization, which is inherently more robust to coordinate changes than the unnormalized DLT.

## Relationships
- [[Homography]] is estimated via [[Maximum Likelihood Estimation]]
- [[Homography]] is estimated using [[Algebraic Distance]]
- [[Homography]] is estimated using [[Geometric Distance]]
- [[Homography]] is estimated using [[Sampson Error]]
- [[Geometric Distance]] specialises to [[Transfer Error]]
- [[Geometric Distance]] specialises to [[Symmetric Transfer Error]]
- [[Geometric Distance]] specialises to [[Reprojection Error]]
- [[Sampson Error]] approximates [[Geometric Distance]]
- [[DLT Algorithm]] minimises [[Algebraic Distance]]
- [[Maximum Likelihood Estimation]] is equivalent to minimizing [[Geometric Distance]]
- [[Homography]] is subject to [[Degeneracy]]
- [[Similarity Transformation]] affects invariance of [[Homography]]
- [[Mahalanobis Distance]] is used in general Gaussian cases for [[Maximum Likelihood Estimation]]
