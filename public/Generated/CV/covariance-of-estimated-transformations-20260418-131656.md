---
type: content
title: "Covariance of Estimated Transformations"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-18T13:16:56.388708200+00:00
summary: "Analysis of the propagation of covariance from measurement space to parameter space for affine and non-linear transformations, including over-parametrized cases."
tags:
  - computer-vision
  - statistics
  - linear-algebra
  - error-analysis
entities:
  - "[[Covariance Matrix]]"
  - "[[Maximum Likelihood Estimate]]"
  - "[[Jacobian Matrix]]"
  - "[[Affine Mapping]]"
  - "[[Mahalanobis Distance]]"
  - "[[Backward Transport of Covariance]]"
  - "[[Over-parametrization]]"
  - "[[Homography Matrix]]"
  - "[[Essential Parameters]]"
  - "[[Monte Carlo Estimation]]"
relationships:
  - source: "Backward Transport of Covariance"
    target: "Covariance Matrix"
    description: "computes the"
  - source: "Backward Transport of Covariance"
    target: "Maximum Likelihood Estimate"
    description: "is used to find the covariance of the"
  - source: "Backward Transport of Covariance"
    target: "Jacobian Matrix"
    description: "depends on the"
  - source: "Backward Transport of Covariance"
    target: "Affine Mapping"
    description: "has a specific form for"
  - source: "Maximum Likelihood Estimate"
    target: "Mahalanobis Distance"
    description: "minimizes the"
  - source: "Over-parametrization"
    target: "Essential Parameters"
    description: "defines the number of"
  - source: "Over-parametrization"
    target: "Homography Matrix"
    description: "occurs when estimating a"
  - source: "Monte Carlo Estimation"
    target: "Covariance Matrix"
    description: "provides a statistical estimate of the"
  - source: "Jacobian Matrix"
    target: "Covariance Matrix"
    description: "is used to propagate the"
---

# Covariance of Estimated Transformations

*Analysis of the propagation of covariance from measurement space to parameter space for affine and non-linear transformations, including over-parametrized cases.*

The covariance of an estimated transformation describes the uncertainty in the parameters of a mapping from a parameter space to a measurement space.

## Backward Propagation of Covariance

When a differentiable mapping $f$ exists from a parameter space $\mathbb{R}^M$ to a measurement space $\mathbb{R}^N$, the covariance of the probability distribution in the measurement space can be propagated backward to compute the [[Covariance Matrix]] for the [[Maximum Likelihood Estimate]] (MLE) of the parameters.

### Affine Case
For an [[Affine Mapping]] of the form $f(\mathbf{P}) = f(\mathbf{P}_0) + \mathbf{J}\mathbf{P}$, where $\mathbf{J}$ is the [[Jacobian Matrix]], the covariance matrix $\Sigma_{\mathbf{P}}$ is given by:
$$\Sigma_{\mathbf{P}} = (\mathbf{J}^T \Sigma_{\mathbf{X}}^{-1} \mathbf{J})^{-1}$$
where $\Sigma_{\mathbf{X}}$ is the covariance in the measurement space.

### Non-linear Case
In the non-linear case, the mapping is approximated by an affine function using a first-order Taylor expansion. The covariance is then estimated using the Jacobian evaluated at the point $\mathbf{P}$.

## Over-parametrization

[[Over-parametrization]] occurs when the mapping $f$ is not locally one-to-one, meaning the Jacobian does not have full rank $M$. The number of independent parameters is referred to as the [[Essential Parameters]] ($d < M$). 

In such cases, the [[Covariance Matrix]] is singular because variance in directions orthogonal to the constraint surface is zero. For a [[Homography Matrix]], which has 8 degrees of freedom but is often represented by a 9-vector, a constraint (such as $\mathbf{P}^T\mathbf{P} = 1$) is typically imposed to resolve this.

## Estimation Methods

### Analytical Method
Analytical estimation relies on the assumption of linearity (local flatness of the surface) and that the estimation method is the [[Maximum Likelihood Estimate]]. It utilizes the [[Mahalanobis Distance]] to find the closest point on the surface to the measurement vector.

### Monte Carlo Estimation
[[Monte Carlo Estimation]] is a general, simulation-based approach used when the linearity assumption is invalid or when the estimation method is not the MLE. It involves adding noise to a known transformation and statistically computing the covariance from multiple trials.

## Relationships
- [[Backward Transport of Covariance]] computes the [[Covariance Matrix]].
- [[Backward Transport of Covariance]] is used to find the covariance of the [[Maximum Likelihood Estimate]].
- [[Backward Transport of Covariance]] depends on the [[Jacobian Matrix]].
- [[Backward Transport of Covariance]] has a specific form for [[Affine Mapping]].
- [[Maximum Likelihood Estimate]] minimizes the [[Mahalanobis Distance]].
- [[Over-parametrization]] defines the number of [[Essential Parameters]].
- [[Over-parametrization]] occurs when estimating a [[Homography Matrix]].
- [[Monte Carlo Estimation]] provides a statistical estimate of the [[Covariance Matrix]].
- [[Jacobian Matrix]] is used to propagate the [[Covariance Matrix]].
