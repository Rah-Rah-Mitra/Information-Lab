---
type: content
title: "Probabilistic Measurement Models for Range Finders"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:46:33.369478+00:00
summary: "An overview of beam-based, likelihood field, and feature-based measurement models for robot range sensors, including ML estimation and EKF localization."
tags:
  - gis
  - robotics
  - probabilistic-robotics
  - sensor-models
  - maximum-likelihood
entities:
  - "[[Beam Model]]"
  - "[[Likelihood Field]]"
  - "[[Feature-Based Model]]"
  - "[[Maximum Likelihood Estimator]]"
  - "[[Expectation Maximization]]"
  - "[[Extended Kalman Filter]]"
  - "[[Markov Localization]]"
  - "[[Data Association]]"
  - "[[Landmark]]"
  - "[[Map Matching]]"
  - "[[Occupancy Grid Map]]"
relationships:
  - source: "Beam Model"
    target: "Maximum Likelihood Estimator"
    description: "parameters are learned via"
  - source: "Maximum Likelihood Estimator"
    target: "Expectation Maximization"
    description: "is implemented using"
  - source: "Likelihood Field"
    target: "Beam Model"
    description: "provides a smoother alternative to"
  - source: "Map Matching"
    target: "Likelihood Field"
    description: "is an alternative to"
  - source: "Feature-Based Model"
    target: "Landmark"
    description: "extracts and tracks"
  - source: "Feature-Based Model"
    target: "Data Association"
    description: "requires solving the"
  - source: "Extended Kalman Filter"
    target: "Markov Localization"
    description: "is a special case of"
  - source: "Extended Kalman Filter"
    target: "Feature-Based Model"
    description: "often utilizes"
  - source: "Beam Model"
    target: "Occupancy Grid Map"
    description: "performs ray casting against"
  - source: "Likelihood Field"
    target: "Occupancy Grid Map"
    description: "projects endpoints into"
---

# Probabilistic Measurement Models for Range Finders

*An overview of beam-based, likelihood field, and feature-based measurement models for robot range sensors, including ML estimation and EKF localization.*

Probabilistic measurement models describe the likelihood of a sensor reading given a robot's pose and a map of the environment.

## Beam-Based Models
The [[Beam Model]] uses ray casting to determine the probability of a range measurement. It typically employs a mixture model to account for different noise sources: hits (Gaussian), short readings (unexpected obstacles), max-range readings, and random noise. The intrinsic parameters of this model are often learned using a [[Maximum Likelihood Estimator]], which is implemented via the [[Expectation Maximization]] algorithm to handle latent variables (the cause of each measurement).

## Likelihood Field Models
The [[Likelihood Field]] is an ad-hoc alternative that avoids expensive ray casting by projecting sensor endpoints into the map and calculating the distance to the nearest obstacle. This results in a smoother probability distribution in pose space compared to the beam model, though it ignores free-space information and occlusions. It is often used with an [[Occupancy Grid Map]].

## Correlation and Feature-Based Models
[[Map Matching]] correlates local maps generated from scans with a global map. While efficient, it lacks a physical generative model. 

Alternatively, the [[Feature-Based Model]] reduces high-dimensional raw data into a small set of [[Landmark]] features (e.g., lines, corners). This significantly reduces computational complexity but may sacrifice sufficient statistics. A critical challenge here is [[Data Association]], the process of correctly matching observed features to landmarks in the map.

## Localization Integration
These models are integrated into localization frameworks. [[Markov Localization]] provides a general Bayesian framework for global localization. The [[Extended Kalman Filter]] (EKF) is a specific implementation that represents the belief as a Gaussian, linearizing the motion and measurement models to update the robot's pose estimate.

## Relationships
- [[Beam Model]] parameters are learned via [[Maximum Likelihood Estimator]].
- [[Maximum Likelihood Estimator]] is implemented using [[Expectation Maximization]].
- [[Likelihood Field]] provides a smoother alternative to [[Beam Model]].
- [[Map Matching]] is an alternative to [[Likelihood Field]].
- [[Feature-Based Model]] extracts and tracks [[Landmark]] features.
- [[Feature-Based Model]] requires solving the [[Data Association]] problem.
- [[Extended Kalman Filter]] is a special case of [[Markov Localization]].
- [[Extended Kalman Filter]] often utilizes [[Feature-Based Model]].
- [[Beam Model]] performs ray casting against [[Occupancy Grid Map]].
- [[Likelihood Field]] projects endpoints into [[Occupancy Grid Map]].
