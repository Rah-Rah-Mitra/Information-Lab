---
type: content
title: "Statistical Inference and Robust Estimation"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:39:45.059701+00:00
summary: "This note explores the relationship between likelihood functions, Bayesian inference, and robust estimation techniques. It explains how maximum likelihood estimation and MAP estimation are used to find optimal state estimates in the presence of noise. The text also introduces robust statistics to handle outliers through M-estimators and iterative re-weighted least squares."
tags:
  - computer-vision
  - statistical-inference
  - robust-statistics
  - bayesian-inference
  - optimization
entities:
  - "[[Maximum Likelihood Estimation]]"
  - "[[Bayesian Inference]]"
  - "[[Maximum A Posteriori]]"
  - "[[M-Estimator]]"
  - "[[Markov Random Fields]]"
  - "[[Mahalanobis Distance]]"
  - "[[Iteratively Re-weighted Least Squares]]"
  - "[[Cramer-Rao Lower Bound]]"
  - "[[Gaussian Noise]]"
  - "[[Robust Statistics]]"
relationships:
  - source: "Maximum Likelihood Estimation"
    target: "Gaussian Noise"
    description: "assumes"
  - source: "Maximum Likelihood Estimation"
    target: "Maximum A Posteriori"
    description: "is a special case of"
  - source: "Maximum A Posteriori"
    target: "Bayesian Inference"
    description: "uses"
  - source: "M-Estimator"
    target: "Robust Statistics"
    description: "is a tool of"
  - source: "Markov Random Fields"
    target: "Bayesian Inference"
    description: "provides"
  - source: "Iteratively Re-weighted Least Squares"
    target: "M-Estimator"
    description: "solves"
  - source: "Cramer-Rao Lower Bound"
    target: "Maximum Likelihood Estimation"
    description: "provides a bound for"
---

# Statistical Inference and Robust Estimation

*This note explores the relationship between likelihood functions, Bayesian inference, and robust estimation techniques. It explains how maximum likelihood estimation and MAP estimation are used to find optimal state estimates in the presence of noise. The text also introduces robust statistics to handle outliers through M-estimators and iterative re-weighted least squares.*

This note covers the fundamental principles of statistical inference used in computer vision to estimate unknown states from noisy measurements.

## Concept
In many vision tasks, we observe noisy measurements $\mathbf{y}$ and wish to infer the state $\mathbf{x}$. The [[Maximum Likelihood Estimation]] (MLE) approach seeks the value of $\mathbf{x}$ that maximizes the likelihood $L(p(\mathbf{yp|x}))$. When the noise is modeled as [[Gaussian Noise]] with covariance $\mathbf{\Sigma}$, the negative log-likelihood becomes a quadratic cost function, leading to the [[Least Squares]] problem.

$$ \mathcal{E}_{\text{log}L} = \sum_{i} \frac{1}{2} (\mathbf{\tilde{y}}_i - \mathbf{f(x)})^T \mathbf{\Sigma}_i^{-1} (\mathbf{\tilde{y}}_i - \mathbf{f(x)}) $$ 

This quadratic cost is minimized to find the MLE. However, if the data contains outliers, a quadratic penalty is too sensitive. [[Robust Statistics]] provides alternatives through the use of an [[M-Estimator]], which replaces the squared residual with a robust penalty function $\rho(r)$. 

$$ \mathcal{E}_{\text{p}} = \sum_{i} \rho(r_i) $$

Finding the minimum of such functions is often achieved via [[Iteratively Re-weighted Least Squares]] (IRLS), which alternates between computing weights and solving weighted least squares problems.

In a [[Bayesian Inference]] framework, we incorporate a prior distribution $p(\mathbf{x})$ to obtain the [[Maximum A Posteriori]] (MAP) estimate. The MAP estimate minimizes the sum of the data energy and the prior energy:

$$ \mathcal{E}_{\text{dp}} = \mathcal{E}_{\text{dp}} + \mathcal{E}_{\text{p}} $$

For gridded data like images, [[Markov Random Fields]] (MRFs) are a common way to encode such priors, using interaction potentials between neighboring pixels. Finally, the [[Cramer-Rao Lower Bound]] provides a theoretical limit on the precision of any unbiased estimator, often estimated via the inverse of the Hessian matrix in the vicinity of the optimal solution.

## Relationships
- [[Maximum Likelihood Estimation]] assumes [[Gaussian Noise]]
- [[Maximum Likelihood Estimation]] is a special case of [[Maximum A Posteriori]]
- [[Maximum A Posteriori]] uses [[Bayesian Inference]]
- [[M-Estimator]] is a tool of [[Robust Statistics]]
- [[Markov Random Fields]] provides [[Bayesian Inference]]
- [[Iteratively Re-weighted Least Squares]] solves [[M-Estimator]]
- [[Cramer-Rao Lower Bound]] provides a bound for [[Maximum Likelihood Estimation]]
