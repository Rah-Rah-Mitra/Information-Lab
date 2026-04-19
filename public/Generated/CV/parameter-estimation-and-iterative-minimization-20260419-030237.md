---
type: content
title: "Parameter Estimation and Iterative Minimization"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T03:02:37.565076400+00:00
summary: "This note covers the fundamental principles of parameter estimation, including bias, variance, and the Cramer-Rao lower bound. It also details iterative methods like Gauss-Newton and Levenberg-Marquardt, specifically focusing on sparse implementations for large-scale reconstruction problems. These concepts are essential for robustly estimating camera parameters and 3D points from noisy image measurements."
tags:
  - computer-vision
  - optimization
  - statistics
  - parameter-estimation
  - iterative-methods
entities:
  - "[[Parameter Estimation]]"
  - "[[Bias]]"
  - "[[Variance]]"
  - "[[Cramer-Rao Lower Bound]]"
  - "[[Maximum Likelihood Estimate]]"
  - "[[Maximum A Posteriori Estimate]]"
  - "[[Fisher Information Matrix]]"
  - "[[Fisher Score]]"
  - "[[Levenberg-Marquardt Iteration]]"
  - "[[Gauss-Newton Method]]"
  - "[[Singular Value Decomposition]]"
  - "[[Jacobian Matrix]]"
relationships:
  - source: "Parameter Estimation"
    target: "Bias"
    description: "is subject to"
  - source: "Parameter Estimation"
    target: "Variance"
    description: "is characterized by"
  - source: "Parameter Estimation"
    target: "Maximum Likelihood Estimate"
    description: "uses"
  - source: "Parameter Estimation"
    target: "Maximum A Posteriori Estimate"
    description: "uses"
  - source: "Cramer-Rao Lower Bound"
    target: "Fisher Information Matrix"
    description: "depends on"
  - source: "Levenberg-Marquardt Iteration"
    target: "Gauss-Newton Method"
    description: "is a variation of"
  - source: "Levenberg-Marquardt Iteration"
    target: "Jacobian Matrix"
    description: "requires"
  - source: "Levenberg-Marquardt Iteration"
    target: "Singular Value Decomposition"
    description: "uses"
  - source: "Gauss-Newton Method"
    target: "Jacobian Matrix"
    description: "approximates the Hessian with"
  - source: "Levenberg-Marquardt Iteration"
    target: "Fisher Information Matrix"
    description: "not directly related in text, but contextually relevant"
  - source: "Singular Value Decomposition"
    target: "Jacobian Matrix"
    description: "used in solving"
  - source: "Maximum Likelihood Estimate"
    target: "Bias"
    description: "can be biased"
  - source: "Maximum A Posteriori Estimate"
    target: "Bias"
    description: "can be biased"
  - source: "Fisher Information Matrix"
    target: "Fisher Score"
    description: "is defined by"
  - source: "Variance"
    target: "Cramer-Rao Lower Bound"
    description: "is bounded by"
---

# Parameter Estimation and Iterative Minimization

*This note covers the fundamental principles of parameter estimation, including bias, variance, and the Cramer-Rao lower bound. It also details iterative methods like Gauss-Newton and Levenberg-Marquardt, specifically focusing on sparse implementations for large-scale reconstruction problems. These concepts are essential for robustly estimating camera parameters and 3D points from noisy image measurements.*

This note explores the mathematical foundations of parameter estimation and the iterative optimization techniques used to solve complex geometric estimation problems.

## Concept
[[Parameter Estimation]] involves finding the best-fitting parameters for a model given a set of measurements. The quality of an estimator is primarily assessed through its [[Bias]] and [[Variance]]. [[Bias]] represents the systematic error in the estimate, which can be highly dependent on the parametrization of the model (e.g., in projective geometry). [[Variance]] measures the variability of the estimates across repeated trials. The [[Cramer-Rao Lower Bound]] provides a theoretical minimum variance for any unbiased estimator, and it is defined using the [[Fisher Information Matrix]], which is derived from the [[Fisher Score]].

In the context of Bayesian estimation, the [[Maximum Likelihood Estimate]] (MLE) and [[Maximum A Posteriori Estimate]] (MAP) are two common approaches. The MLE focuses on the maximum of the likelihood function, while the MAP estimate incorporates a prior distribution. Note that the MAP estimate is sensitive to the parametrization of the parameter space.

To solve non-linear least-squares problems, iterative methods are used. The [[Gauss-Newton Method]] is a common approach that approximates the Hessian matrix of the cost function using the [[Jacobian Matrix]] of the residuals. The [[Levenberg-Marquardt Iteration]] acts as a hybrid between Gauss-Newton and [[Gradient Descent]] (though not explicitly detailed in the note, it but is mentioned as a concept). The LM algorithm transitions between these as the $\lambda$ parameter is adjusted, providing stability when the far-from-minimum regions are are difficult to optimize. For large-scale problems like 3D reconstruction, a [[Singular Value Decomposition]] (SVD) is used to efficiently solve the normal equations, and sparse implementations of the LM algorithm are able to leverage the Jacobian's block structure to achieve linear complexity in the number of points.

## Relationships
- [[Parameter Estimation]] is subject to [[Bias]]
- [[Parameter Estimation]] is characterized by [[Variance]]
- [[Parameter Estimation]] uses [[Maximum Likelihood Estimate]]
- [[Parameter Estimation]] uses [[Maximum A Posteriori Estimate]]
- [[Cramer-Rao Lower Bound]] is bounded by [[Variance]]
- [[Cramer-Rao Lower Bound]] depends on [[Fisher Information Matrix]]
- [[Fisher Information Matrix]] is defined by [[Fisher Score]]
- [[Gauss-Newton Method]] uses [[Jacobian Matrix]]
- [[Levenberg-Marquardt Iteration]] is a variation of [[Gauss-Newton Method]]
- [[Levenberg-Marquardt Iteration]] requires [[Jacobian Matrix]]
- [[Levenberg-Marquardt Iteration]] uses [[Singular Value Decomposition]]
- [[Singular Value Decomposition]] is used in solving [[Jacobian Matrix]]
