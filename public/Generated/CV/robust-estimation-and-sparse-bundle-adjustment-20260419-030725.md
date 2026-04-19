---
type: content
title: "Robust Estimation and Sparse Bundle Adjustment"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T03:07:25.820047700+00:00
summary: "This note covers the application of sparse Levenberg-Marquardt to bundle adjustment and the use of robust cost functions to mitigate outlier influence. It also discusses the mathematical properties of parameterization and sparse matrix decomposition for efficient large-scale optimization. These techniques are essential for reliable structure-from-motion and structure-and-motion estimation."
tags:
  - computer-vision
  - optimization
  - robust-estimation
  - sparse-methods
entities:
  - "[[Bundle Adjustment]]"
  - "[[Levenberg-Marquardt Algorithm]]"
  - "[[Robust Cost Functions]]"
  - "[[Sparse Matrix]]"
  - "[[Parameterization]]"
  - "[[, ]]"
relationships:
  - source: "Bundle Adjustment"
    target: "Levenberg-Marquardt Algorithm"
    description: "utilizes"
  - source: "Bundle Adjustment"
    target: "Robust Cost Functions"
    description: "requires"
  - source: "Levenberg-Marquardt Algorithm"
    target: "Sparse Matrix"
    description: "exploits"
  - source: "Parameterization"
    target: "Robust Cost Functions"
    description: "influences"
---

# Robust Estimation and Sparse Bundle Adjustment

*This note covers the application of sparse Levenberg-Marquardt to bundle adjustment and the use of robust cost functions to mitigate outlier influence. It also discusses the mathematical properties of parameterization and sparse matrix decomposition for efficient large-scale optimization. These techniques are essential for reliable structure-from-motion and structure-and-motion estimation.*

This note explores the optimization techniques used in large-scale reconstruction and estimation problems, particularly in computer vision and robotics.

## Concept
[[Bundle Adjustment]] is the simultaneous estimation of multiple camera parameters and 3D points to compute projective structure. In large-scale scenarios, the problem is often solved using the [[Levenberg-Marquardt Algorithm]], which can be optimized by exploiting the [[Sparse Matrix]] structure of the Jacobian and normal equations. This sparseness often arises from the banded structure of point tracks over time.

For robust estimation in the presence of outliers, [[Robust Cost Functions]] are employed. These functions modify the standard least-squares cost to reduce the influence of weight of distant measurements. Common examples include:
- [[L1 Cost Function]]: A convex, asymptotically linear function that finds the median.
- [[Huber Cost Function]]: A hybrid between L1 and squared error, providing a smooth transition.
- [[Cauchy Cost Function]]: A non-convex function with heavier tails, useful for de-emphasizing outliers.
- [[Squared Error]]: The standard quadratic cost, highly susceptible to outliers.

Efficient computation of these large systems requires specialized [[Parameterization]] to avoid singularities and gauge freedoms. For instance, rotations can be parametrized by angle-axis or quaternions, and unit vectors can be constrained using local parametrizations on the n-sphere.

## Relationships
- [[Bundle Adjustment]] utilizes [[Levenberg-Marquardt Algorithm]]
- [[Bundle Adjustment]] requires [[Robust Cost Functions]]
- [[Levenberg-Marquardt Algorithm]] exploits [[Sparse Matrix]]
- [[Parameterization]] influences [[Robust Cost Functions]]
