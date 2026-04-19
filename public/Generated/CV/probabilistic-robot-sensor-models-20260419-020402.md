---
type: content
title: "Probabilistic Robot Sensor Models"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:04:02.767814400+00:00
summary: "This note explores various probabilistic models used to represent sensor measurements in robotics, ranging from raw beam-based models to feature-based landmark models. It details the mathematical mechanisms for estimating intrinsic parameters and the trade-offs between physical realism, computational efficiency, and smoothness in different approaches."
tags:
  - robotics
  - sensor-models
  - probabilistic-robotics
  - localization
  - estimation-theory
entities:
  - "[[Beam Model]]"
  - "[[Likelihood Field]]"
  - "[[Map Matching]]"
  - "[[Feature-Based Model]]"
  - "[[Maximum Likelihood Estimation]]"
  - "[[Expectation Maximization]]"
  - "[[Extended Kalman Filter]]"
  - "[[Landmark]]"
  - "[[Correspondence Variable]]"
  - "[[Markov Localization]]"
relationships:
  - source: "Beam Model"
    target: "Maximum Likelihood Estimation"
    description: "parameters are estimated via"
  - source: "Maximum Likelihood Estimation"
    target: "Expectation Maximization"
    description: "is an instance of"
  - source: "Beam Model"
    target: "Expectation Maximization"
    description: "uses"
  - source: "Likelihood Field"
    target: "Beam Model"
    description: "is an alternative to"
  - source: "Feature-Based Model"
    target: "Landmark"
    description: "relies on"
  - source: "Extended Kalman Filter"
    target: "Feature-Based Model"
    description: "is applied to"
  - source: "Extended Kalman Filter"
    target: "Correspondence Variable"
    description: "requires"
  - source: "Markov Localization"
    target: "Beam Model"
    description: "can use"
  - source: "Landmark"
    target: "Correspondence Variable"
    description: "is associated with via"
---

# Probabilistic Robot Sensor Models

*This note explores various probabilistic models used to represent sensor measurements in robotics, ranging from raw beam-based models to feature-based landmark models. It details the mathematical mechanisms for estimating intrinsic parameters and the trade-offs between physical realism, computational efficiency, and smoothness in different approaches.*

This note summarizes the various probabilistic measurement models used in robotics to relate sensor readings to the robot's pose and the environment map.

## Concept

### [[Beam Model]]
The [[Beam Model]] is a generative model that uses ray casting to determine the expected range of a sensor beam. It models sensor noise through a mixture of four distributions: a hit (Gaussian), a short reading (due to obstacles), a max-range reading, and a random reading (uniform). The intrinsic parameters of this model, such as the standard deviation of the hit distribution, are often estimated using [[Maximum Likelihood Estimation]] (MLE). Because the mixture components are not known a priori, the [[Expectation Maximization]] (EM) algorithm is used to iteratively estimate both the latent error types and the model parameters.

$$ p(z | x, m, \Theta) = p_{hit}(z | x, m, \Theta) + p_{short}(z | x, m, \Theta) + p_{max}(z | x, m, \Theta) + p_{rand}(z | x, m, \Theta) $$

The the [[Beam Model]] is highly physically realistic but suffers from a lack of smoothness and high computational cost due to the required ray casting.

### [[Likelihood Field]]
The [[Likelihood Field]] is an ad hoc alternative to the beam model. Instead of ray casting, it projects the endpoints of sensor scans into a global coordinate space and models the probability of a measurement based on the Euclidean distance to the nearest obstacle in the map. This approach yields much smoother distributions and is more computationally efficient, as it can be reduced to a 2-D table lookup. However, it ignores free-space information and can be susceptible to errors if the robot "sees through walls."

### [[Map Matching]]
[[Map Matching]] involves comparing a local map generated from recent sensor scans with a global map. The similarity is measured using a correlation function, which is more efficient than the beam model but lacks a clear physical explanation and can be highly non-smooth.

### [[Feature-Based Model]]
In a [[Feature-Based Model]], the robot extracts a small number of high-dimensional features (such as lines or corners) from raw measurements. These features are often represented as [[Landmark]]s, which consist of range, bearing, and a signature. This approach significantly reduces computational complexity but sacrifices information, making the [[Data Association Problem]] (the problem of identifying which landmark corresponds to which measurement) more difficult.

## Relationships
- [[Beam Model]] parameters are estimated via [[Maximum Likelihood Estimation]]
- [[Maximum Likelihood Estimation]] is an instance of [[Expectation Maximization]]
- [[Likelihood Field]] is an alternative to [[Beam Model]]
- [[Feature-Based Model]] relies on [[Landmark]]s
- [[Extended Kalman Filter]] is applied to [[Feature-Based Model]]s
- [[Markov Localization]] can use [[Beam Model]]s
