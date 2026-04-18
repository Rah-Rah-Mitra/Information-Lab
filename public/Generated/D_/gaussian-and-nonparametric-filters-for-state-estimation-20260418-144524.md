---
type: content
title: "Gaussian and Nonparametric Filters for State Estimation"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:45:24.278644200+00:00
summary: "An overview of Bayesian filters for state estimation, covering Gaussian filters like EKF, UKF, and Information Filters, and nonparametric methods like Particle Filters."
tags:
  - gis
  - robotics
  - statistics
  - state-estimation
  - bayesian-filtering
entities:
  - "[[Bayes Filter]]"
  - "[[Kalman Filter]]"
  - "[[Extended Kalman Filter]]"
  - "[[Unscented Kalman Filter]]"
  - "[[Information Filter]]"
  - "[[Extended Information Filter]]"
  - "[[Particle Filter]]"
  - "[[Histogram Filter]]"
  - "[[Binary Bayes Filter]]"
  - "[[Taylor Expansion]]"
  - "[[Unscented Transform]]"
  - "[[Jacobian]]"
  - "[[Canonical Parameterization]]"
  - "[[Importance Sampling]]"
relationships:
  - source: "Extended Kalman Filter"
    target: "Kalman Filter"
    description: "generalizes"
  - source: "Extended Kalman Filter"
    target: "Taylor Expansion"
    description: "utilizes for linearization"
  - source: "Extended Kalman Filter"
    target: "Jacobian"
    description: "computes"
  - source: "Unscented Kalman Filter"
    target: "Unscented Transform"
    description: "utilizes for linearization"
  - source: "Information Filter"
    target: "Kalman Filter"
    description: "is dual to"
  - source: "Information Filter"
    target: "Canonical Parameterization"
    description: "uses for representation"
  - source: "Extended Information Filter"
    target: "Information Filter"
    description: "extends to nonlinear case"
  - source: "Particle Filter"
    target: "Importance Sampling"
    description: "implements via"
  - source: "Histogram Filter"
    target: "Bayes Filter"
    description: "is a nonparametric implementation of"
  - source: "Binary Bayes Filter"
    target: "Bayes Filter"
    description: "is a specialized version of"
  - source: "Particle Filter"
    target: "Bayes Filter"
    description: "is a nonparametric implementation of"
---

# Gaussian and Nonparametric Filters for State Estimation

*An overview of Bayesian filters for state estimation, covering Gaussian filters like EKF, UKF, and Information Filters, and nonparametric methods like Particle Filters.*

A [[Bayes Filter]] is a recursive algorithm used to estimate the state of a system over time, often implemented using either parametric Gaussian representations or nonparametric sample-based methods.

## Gaussian Filters

Gaussian filters represent the posterior belief as a multivariate Gaussian distribution, characterized by a mean and covariance.

### Kalman Filter and its Extensions
The [[Kalman Filter]] is the optimal estimator for linear Gaussian systems. However, most robotics problems are nonlinear, necessitating approximations:

- **[[Extended Kalman Filter]] (EKF)**: Linearizes nonlinear state transitions and measurement functions using a first-order [[Taylor Expansion]] evaluated at the current mean. This process requires the computation of a [[Jacobian]] matrix.
- **[[Unscented Kalman Filter]] (UKF)**: Uses the [[Unscented Transform]] to deterministically pick "sigma points" that are passed through the nonlinear function, capturing the mean and covariance more accurately than the EKF without requiring Jacobians.

### Information Filters
The [[Information Filter]] is the dual of the Kalman Filter. Instead of moments (mean, covariance), it uses a [[Canonical Parameterization]] consisting of an information matrix (precision matrix) and an information vector. This representation makes measurement updates additive and computationally efficient, particularly in decentralized multi-robot systems. The [[Extended Information Filter]] (EIF) applies the same linearization principles as the EKF to the information filter framework.

## Nonparametric Filters

Nonparametric filters do not assume a fixed functional form for the posterior, allowing them to represent multimodal beliefs.

### Histogram and Binary Filters
- **[[Histogram Filter]]**: Decomposes the state space into finite bins (regions) and assigns a probability to each. This can be implemented via static grids or dynamic [[Density Trees]].
- **[[Binary Bayes Filter]]**: Used for static binary states (e.g., whether a door is open), often implemented using a log-odds ratio to avoid numerical truncation.

### Particle Filters
The [[Particle Filter]] approximates the posterior using a set of random samples called particles. It employs [[Importance Sampling]] to refocus the particle set toward regions of high posterior probability through a process of weighting and resampling (survival of the fittest).

## Relationships
- [[Extended Kalman Filter]] generalizes [[Kalman Filter]]
- [[Extended Kalman Filter]] utilizes [[Taylor Expansion]] for linearization
- [[Extended Kalman Filter]] computes [[Jacobian]]
- [[Unscented Kalman Filter]] utilizes [[Unscented Transform]] for linearization
- [[Information Filter]] is dual to [[Kalman Filter]]
- [[Information Filter]] uses [[Canonical Parameterization]] for representation
- [[Extended Information Filter]] extends [[Information Filter]] to nonlinear case
- [[Particle Filter]] implements via [[Importance Sampling]]
- [[Histogram Filter]] is a nonparametric implementation of [[Bayes Filter]]
- [[Binary Bayes Filter]] is a specialized version of [[Bayes Filter]]
- [[Particle Filter]] is a nonparametric implementation of [[Bayes Filter]]
