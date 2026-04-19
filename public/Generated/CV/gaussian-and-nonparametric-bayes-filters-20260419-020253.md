---
type: content
title: "Gaussian And Nonparametric Bayes Filters"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:02:53.788534200+00:00
summary: "This note compares Gaussian-based filters like the Kalman and Extended Kalman filters with nonparametric approaches such as particle filters. It explains the mathematical transitions from linear to nonlinear state estimation and the representational differences between parametric and nonparametric methods. The discussion highlights the trade-offs between computational efficiency and the ability to represent multimodal distributions."
tags:
  - robotics
  - state-estimation
  - gaussian-filters
  - nonparametric-filters
  - bayes-filters
entities:
  - "[[Kalman Filter]]"
  - "[[Extended Kalman Filter]]"
  - "[[Unscented Kalman Filter]]"
  - "[[Information Filter]]"
  - "[[Particle Filter]]"
  - "[[Gaussian Distribution]]"
  - "[[Taylor Expansion]]"
  - "[[Unscented Transform]]"
  - "[[Jacobian]]"
  - "[[Nonparametric Filter]]"
  - "[[Bayes Filter]]"
  - "[[Canonical Parameterization]]"
relationships:
  - source: "Kalman Filter"
    target: "Gaussian Distribution"
    description: "assumes"
  - source: "Extended Kalman Filter"
    target: "Taylor Expansion"
    description: "uses"
  - source: "Extended Kalman Filter"
    target: "Jacobian"
    description: "requires"
  - source: "Unscented Kalman Filter"
    target: "Unscented Transform"
    description: "uses"
  - source: "Unscented Kalman Filter"
    target: "Jacobian"
    description: "avoids"
  - source: "Information Filter"
    target: "Canonical Parameterization"
    description: "uses"
  - source: "Particle Filter"
    target: "Nonparametric Filter"
    description: "is a type of"
  - source: "Bayes Filter"
    target: "Gaussian Distribution"
    description: "can be represented by"
---

# Gaussian And Nonparametric Bayes Filters

*This note compares Gaussian-based filters like the Kalman and Extended Kalman filters with nonparametric approaches such as particle filters. It explains the mathematical transitions from linear to nonlinear state estimation and the representational differences between parametric and nonparametric methods. The discussion highlights the trade-offs between computational efficiency and the ability to represent multimodal distributions.*

This note explores the various implementations of the Bayes filter for state estimation, ranging from parametric Gaussian models to nonparametric sample-based methods.

## Concept

A [[Bayes Filter]] is a recursive algorithm used to estimate the state of a system. When the system is linear and the noise is Gaussian, the [[Kalman Filter]] provides an optimal solution by representing the belief as a [[Gaussian Distribution]] using its mean and covariance.

### Gaussian Filters

#### The Kalman Filter
In a linear Gaussian system, the [[Kalman Filter]] updates its belief using the moments parameterization (mean and covariance). The measurement update involves the [[Kalman Gain]], which determines how much the new measurement affects the current estimate.

#### The Extended Kalman Filter
When the system is nonlinear, the [[Kalman Filter]] is no longer optimal. The [[Extended Kalman Filter]] (EKF) addresses this by applying a [[Taylor Expansion]] to linearize the nonlinear functions $g$ and $h$ around the current mean. This process requires calculating the [[Jacobian]] matrix, which represents the local slope of the function.

$$ rac{\partial g(u, x_{t-1})}{\partial x_{t-1}} = G_t $$

The accuracy of the EKF depends on the degree of local nonlinearity and the uncertainty of the state. High uncertainty can lead to significant errors because the linearization is only valid in a small neighborhood of the mean.

#### The Unscented Kalman Filter
As an alternative to Taylor-based linearization, the [[Unscented Kalman Filter]] (UKF) uses the [[Unscented Transform]]. Instead of approximating the function, it selects a set of deterministic [[Sigma Points]] and passes them through the nonlinear function. This method is often more accurate than the EKF because it captures the first two terms of the Taylor expansion.

#### The Information Filter
The [[Information Filter]] is the dual of the Kalman filter. It represents the Gaussian belief using the [[Canonical Parameterization]], which consists of an [[Information Matrix]] $\Omega$ and an [[Information Vector]] $\xi$. This representation is particularly useful in multi-robot systems where information can be integrated through simple addition.

### Nonparametric Filters

[[Nonparametric Filters]], such as the [[Particle Filter]], do not assume a fixed functional form for the posterior. Instead, they represent the belief using a set of random samples called [[Particles]].

#### The Particle Filter
The [[Particle Filter]] uses importance sampling to approximate the posterior. Each particle is assigned an [[Importance Factor]] (or weight) based on its likelihood under the current measurement. The algorithm involves a [[Resampling]] step, which refocuses the particles into regions of high probability, effectively implementing a "survival of the fittest" mechanism.

$$ w_m^t = rac{p(z_t | x_t^m) p(x_t^m | u_t, x_{t-1}^m)}{q(x_t^m | z_t, u_t, x_{t-1}^m)} $$

## Relationships
- [[Kalman Filter]] assumes [[Gaussian Distribution]]
- [[Extended Kalman Filter]] uses [[Taylor Expansion]]
- [[Extended Kalman Filter]] requires [[Jacobian]]
- [[Unscented Kalman Filter]] uses [[Unscented Transform]]
- [[Unscented Kalman Filter]] avoids [[Jacobian]]
- [[Information Filter]] uses [[Canonical Parameterization]]
- [[Particle Filter]] is a type of [[Nonparametric Filter]]
- [[Bayes Filter]] can be represented by [[Gaussian Distribution]]
