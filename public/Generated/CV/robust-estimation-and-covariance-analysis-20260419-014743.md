---
type: content
title: "Robust Estimation and Covariance Analysis"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:47:43.141738+00:00
summary: "This note covers robust estimation techniques like RANSAC and error analysis for 2D homographies, including covariance propagation. It explains how to distinguish inliers from outliers and how to quantify uncertainty in estimated transformations. The methods provide bounds for performance evaluation using Maximum Likelihood Estimates."
tags:
  - computer-vision
  - robust-estimation
  - homography-estimation
  - error-analysis
  - statistics
entities:
  - "[[RANSAC]]"
  - "[[Homography]]"
  - "[[Maximum Likelihood Estimate]]"
  - "[[Sampson Error]]"
  - "[[Covariance Matrix]]"
  - "[[Levenberg-Marquardt Algorithm]]"
  - "[[Inliers]]"
  - "[[Outliers]]"
  - "[[Gold Standard Algorithm]]"
  - "[[DLT Algorithm]]"
relationships:
  - source: "RANSAC"
    target: "Inliers"
    description: "identifies"
  - source: "RANSAC"
    target: "Outliers"
    description: "separates"
  - source: "Homography"
    target: "Maximum Likelihood Estimate"
    description: "estimated via"
  - source: "Sampson Error"
    target: "Homography"
    description: "approximates error for"
  - source: "Covariance Matrix"
    target: "Homography"
    description: "quantifies uncertainty of"
  - source: "Levenberg-Marquardt Algorithm"
    target: "Maximum Likelihood Estimate"
    description: "minimizes cost for"
  - source: "Gold Standard Algorithm"
    target: "Homography"
    description: "preferred method for"
  - source: "DLT Algorithm"
    target: "Homography"
    description: "provides initialization for"
---

# Robust Estimation and Covariance Analysis

*This note covers robust estimation techniques like RANSAC and error analysis for 2D homographies, including covariance propagation. It explains how to distinguish inliers from outliers and how to quantify uncertainty in estimated transformations. The methods provide bounds for performance evaluation using Maximum Likelihood Estimates.*

This note explores the mathematical frameworks for robustly estimating transformations and quantifying the uncertainty of those estimates in computer vision tasks.

## Concept
In many practical scenarios, point correspondences between images contain [[Outliers]] (mismatches) that do not follow a Gaussian error distribution. [[Robust Estimation]] techniques are required to separate these from [[Inliers]] (valid points). The [[RANSAC]] (RANdom SAmple Consensus) algorithm is a primary method for this, iteratively selecting minimal subsets of data to find a model that achieves the highest consensus among inliers.

For 2D [[Homography]] estimation, the minimal subset consists of four correspondences. The algorithm uses a distance threshold to classify points as inliers or outliers. Once the consensus set is is identified, a [[Maximum Likelihood Estimate]] (MLE) is performed on the inliers to obtain an optimal fit.

### Error Metrics
Two common approximations for geometric error are used:
1. [[Sampson Error]]: An approximation to the reprojection error that is more computationally tractable, requiring minimization over only 9 parameters.
2. [[Gold Standard Algorithm]]: A method that minimizes the true geometric error by solving for both the homography and the subsidiary points simultaneously.

### Uncertainty and Covariance
To assess the accuracy of a computed [[Homography]], one can compute its [[Covariance Matrix]] $\Sigma_{\mathbf{h}}$. This matrix captures how errors in measurement propagate to the parameters of the transformation. For a non-linear mapping $f$, the covariance can be approximated using the Jacobian $J$:

$$ \Sigma_{\mathbf{h}} \approx J^T (J \Sigma J^T)^{-1} J $$ 

This formula represents the backward propagation of covariance from the measurement space to the parameter space, particularly in the case of over-parametrized models where constraints (like $||\mathbf{h}||=1$) are applied to ensure a unique solution.

## Relationships
- [[RANSAC]] identifies [[Inliers]]
- [[RANSAC]] separates [[Outliers]]
- [[Homography]] is estimated via [[Maximum Likelihood Estimate]]
- [[Sampson Error]] approximates error for [[Homography]]
- [[Covariance Matrix]] quantifies uncertainty of [[Homography]]
- [[Levenberg-Marquardt Algorithm]] minimizes cost for [[Maximum Likelihood Estimate]]
- [[Gold Standard Algorithm]] is preferred method for [[Homography]]
- [[DLT Algorithm]] provides initialization for [[Homography]]
