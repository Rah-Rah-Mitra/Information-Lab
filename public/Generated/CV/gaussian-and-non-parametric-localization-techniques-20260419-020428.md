---
type: content
title: "Gaussian And Non-Parametric Localization Techniques"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:04:28.600828100+00:00
summary: "This note explores various probabilistic localization methods for mobile robots, ranging from unimodal Gaussian filters like EKF and UKF to non-parametric approaches like Grid and Monte Carlo localization. These techniques address different levels of uncertainty, such as position tracking, global localization, and the kidnapped robot problem. The choice of method depends on the trade-off between computational efficiency and the ability to represent complex, multi-modal distributions."
tags:
  - robotics
  - localization
  - probabilistic-filtering
  - ekf
  - ukf
  - monte-carlo-localization
  - grid-localization
entities:
  - "[[Extended Kalman Filter]]"
  - "[[Unscented Kalman Filter]]"
  - "[[Multi-Hypothesis Tracking]]"
  - "[[Monte Carlo Localization]]"
  - "[[Grid Localization]]"
  - "[[Maximum Likelihood Correspondence]]"
  - "[[Kullback-Leibler Divergence]]"
  - "[[Mixture Monte Carlo Localization]]"
  - "[[Data Association]]"
  - "[[Gaussian Mixture]]"
relationships:
  - source: "Extended Kalman Filter"
    target: "Gaussian Mixture"
    description: "can be extended to"
  - source: "Multi-Hypothesis Tracking"
    target: "Gaussian Mixture"
    description: "uses"
  - source: "Monte Carlo Localization"
    target: "Gaussian Mixture"
    description: "approximates"
  - source: "Monte Carlo Localization"
    target: "Data Association"
    description: "addresses"
  - source: "Relationship"
    target: "Data Association"
    description: "is a core challenge in"
  - source: "Maximum Likelihood Correspondence"
    target: "Data Association"
    description: "is a type of"
  - source: "Multi-Hypothesis Tracking"
    target: "Extended Kalman Filter"
    description: "is an extension of"
  - source: "Mixture Monte Carlo Localization"
    target: "Monte Carlo Localization"
    description: "is a variant of"
  - source: "Monte Carlo Localization"
    target: "Kullback-Leibler Divergence"
    description: "uses"
  - source: "Grid Localization"
    target: "Monte Carlo Localization"
    description: "is an alternative to"
---

# Gaussian And Non-Parametric Localization Techniques

*This note explores various probabilistic localization methods for mobile robots, ranging from unimodal Gaussian filters like EKF and UKF to non-parametric approaches like Grid and Monte Carlo localization. These techniques address different levels of uncertainty, such as position tracking, global localization, and the kidnapped robot problem. The choice of method depends on the trade-off between computational efficiency and the ability to represent complex, multi-modal distributions.*

This note provides an overview of the primary probabilistic localization algorithms used in mobile robotics, categorized by their parametric (Gaussian) and non-parametric (approaches that represent beliefs as particles or grids). 

## Concept

### Gaussian Filters

[[Extended Kalman Filter]] (EKF) is a widely used feature-based localization algorithm. It linearizes motion and measurement models using Taylor expansion, which can lead to errors in highly non-linear systems. The EKF update involves a prediction step (using a motion model) and a correction step (using the innovation vector and the Kalman gain). 

[[Unscented Kalman Filter]] (UKF) provides a more accurate linearization by using the unscented transform. Instead of computing derivatives, it represents Gaussians by a sigma-point representation, which captures the mean and covariance more accurately, especially in highly non-linear scenarios. 

[[Multi-Hypothesis Tracking]] (MHT) extends the EKF to handle uncertainty in [[Data Association]] (the problem of which observed feature corresponds to which landmark in the map). MHT represents the posterior belief as a [[Gaussian Mixture]], where each component (or "track") is a Gaussian with its own mean and weight. This allows the robot to represent multiple plausible hypotheses about its location. 

### Non-Parametric Filters

[[Monte Carlo Localization]] (MCL) is a non-parametric approach that uses a set of particles to represent the posterior belief. Unlike Gaussian filters, MCL can represent complex, multi-modal distributions, and is capable of solving the global localization and the kidnapped robot problem. 

[[Grid Localization]] is a different non-parametric approach that approximates the posterior using a histogram filter over a a grid decomposition of the space. It is more accurate with a fine-grained metric grid, but computationally expensive. 

### Advanced MCL Variants

[[Mixture Monte Carlo Localization]] (or Mixture MCL) also improves the upon standard MCL by incorporating a fraction of the particles generated from the measurement model. This makes it more robust to the total localization failure and the kidnapped robot problem. 

[[Kullback-Leibler Divergence]] (KLD) is used in [[KLD-sampling]] to adapt theively the number of particles in MCL. By monitoring the statistical error between the sample-based approximation and the true posterior, the algorithm can dynamically adjust the number of particles to maintain a certain accuracy bound. 

## Relationships
- [[Extended Kalman Filter]] can be extended to [[Gaussian Mixture]] models via [[Multi-Hypothesis Tracking]].
- [[Employed in feature-based maps.]]
- [[Unscented Kalman Filter]] provides improved linearization compared to [[Extended Kalman Filter]].
- [[Multi-Hypothesis Tracking]] uses [[Gaussian Mixture]] to represent the belief. 
- [[Monte Carlo Localization]] is a non-parametric, a an alternative to [[Grid Localization]].
- [[Monte Carlo Localization]] is a robust solution to [[Data Association]] challenges. 
- [[Mixture Monte Carlo Localization]] is a variant of [[Monte Carlo Localization Monte Carlo Localization]].
- [[KLD-sampling]] uses [[Kullback-Leibler Divergence]] to adapt particle counts in [[Monte Carlo Localization]].
