---
type: content
title: "Probabilistic Robot Localization and Mapping"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:05:00.062450400+00:00
summary: "This note covers the core principles of probabilistic robot localization, including Monte Carlo Localization and Occupancy Grid Mapping, as well as the Simultaneous Localization and Mapping (SLAM) problem. It explains how robots can estimate their pose and environment structure from noisy sensor data. The text also details techniques for handling dynamic environments and managing the complexity of the scale of the map."
tags:
  - robotics
  - probabilistic-filtering
  - localization
  - mapping
  - slam
entities:
  - "[[Monte Carlo Localization]]"
  - "[[KLD-sampling]]"
  - "[[Occupancy Grid Mapping]]"
  - "[[Occupancy Grid]]"
  - "[[Occupancy Probability]]"
  - "[[Kullback-Leibler Divergence]]"
  - "[[Dynamic Environments]]"
  - "[[State Augmentation]]"
  - "[[Outlier Rejection]]"
  - "[[Simultaneous Localization and Mapping]]"
  - "[[Extended Kalman Filter]]"
  - "[[Extended Kalman Filter SLAM]]"
  - "[[Landmark]]"
  - "[[Combined State Vector]]"
  - "[[Maximum A Posteriori Occupancy Mapping]]"
  - "[[Inverse Sensor Model]]"
  - "[[Forward Measurement Model]]"
  - "[[Binary Bayes Filter]]"
relationships:
  - source: "Monte Carlo Localization"
    target: "KLD-sampling"
    description: "uses"
  - source: "Monte Carlo Localization"
    target: "Occupancy Grid Mapping"
    description: "is compared to"
  - source: "Monte Carlo Localization"
    target: "KLD-KL-divergence"
    description: "error measured by"
  - source: "Monte Carlo Localization"
    target: "Dynamic Environments"
    description: "is challenged by"
  - source: "Monte Carlo Localization"
    target: "Simultaneous Localization and Mapping"
    description: "is a component of"
  - source: "Monte Carlo Localization"
    target: "Extended Kalman Filter SLAM"
    description: "is an alternative to"
  - source: "Occupancy Grid Mapping"
    target: "Occupancy Grid"
    description: "uses"
  - source: "Occupancy Grid Mapping"
    target: "Occupancy Probability"
    description: "estimates"
  - source: "Occupancy Grid Mapping"
    target: "Inverse Sensor Model"
    description: "relies on"
  - source: "Occupancy Grid Mapping"
    target: "Maximum A Posteriori Occupancy Mapping"
    description: "is improved by"
  - source: "Occupancy Grid Mapping"
    target: "Binary Bayes Filter"
    description: "is an adaptation of"
  - source: "Occupancy Grid Mapping"
    target: "Dynamic Environments"
    description: "is affected by"
  - source: "Simultaneous Localization and Mapping"
    target: "Extended Kalman Filter SLAM"
    description: "uses"
  - source: "Simultaneous Localization and Mapping"
    target: "Landmark"
    description: "incorporates"
  - source: "Simultaneous Localization and Mapping"
    target: "Combined State Vector"
    description: "requires"
  - source: "Extended Kalman Filter SLAM"
    target: "Landmark"
    description: "estimates"
  - source: "Extended Kalman Filter SLAM"
    target: "Combined State Vector"
    description: "uses"
---

# Probabilistic Robot Localization and Mapping

*This note covers the core principles of probabilistic robot localization, including Monte Carlo Localization and Occupancy Grid Mapping, as well as the Simultaneous Localization and Mapping (SLAM) problem. It explains how robots can estimate their pose and environment structure from noisy sensor data. The text also details techniques for handling dynamic environments and managing the complexity of the scale of the map.*

This note explores the probabilistic frameworks used for robot localization and mapping, focusing on how robots manage uncertainty in their environment. 

## Concept

### Monte Carlo Localization
[[Monte Carlo Localization]] (MCL) is a particle-filter-based approach to estimating a robot's pose. It represents the posterior distribution of the robot's pose using a set of particles. A key efficiency improvement is [[KLD-sampling]], which adapts the number of particles used based on the complexity of the belief. This is achieved by monitoring the number of non-empty bins in a histogram, which serves as a proxy for the belief's volume. The approximation error of MCL is often measured using the [[Kullback-Leibler Divergence]].

### Localization in Dynamic Environments
Standard localization algorithms often assume a static world (the Markov assumption). In [[Dynamic Environments]], where moving objects like people create unmodeled dynamics, algorithms may fail. Two main strategies exist: [[State Augmentation]], which includes the hidden states of dynamic objects in the filter, and [[Outlier Rejection]], which pre-processes sensor measurements to discard those likely caused by unmodeled entities. For example, in range-finding, measurements that are "surprisingly short" are often rejected.

### Occupancy Grid Mapping
[[Occupancy Grid Mapping]] partitions the environment into a grid of cells, each with a binary occupancy state. The algorithm estimates the [[Occupancy Probability]] for each cell using a [[Binary Bayes Filter]]. This is often implemented using the [[Inverse Sensor Model]], which reasons from sensor effects to causes. To improve accuracy and handle dependencies between cells, [[Maximum A Posteriori Occupancy Mapping]] can be used to find the mode of the posterior, which provides a more consistent map than the standard factorial approach.

### Simultaneous Localization and Mapping (SLAM)
[[Simultaneous Localization and Mapping]] (SLAM) is the problem of estimating both the robot's pose and the map of the environment concurrently. This is significantly more difficult than pure localization or mapping because both are unknown. 

One prominent approach is [[Extended Kalman Filter SLAM]] (EKF SLAM), which uses a feature-based map of [[Landmark]] locations. The state vector is augmented to include both the robot pose and the coordinates of all landmarks, forming a [[Combined State Vector]]. This approach uses the [[Extended Kalman Filter]] to maintain a Gaussian belief over this combined state. While efficient for small numbers of landmarks, it scales quadratically with the number of features.

## Relationships
- [[Monte Carlo Localization]] uses [[KLD-sampling]]
- [[Monte Carlo Localization]] error is measured by [[Kullback-Leibler Divergence]]
- [[Monte Carlo Localization]] is challenged by [[Dynamic Environments]]
- [[Occupancy Grid Mapping]] uses [[Occupancy Grid]]
- [[Occupancy Grid Mapping]] estimates [[Occupancy Probability]]
- [[Occupancy Grid Mapping]] relies on [[Inverse Sensor Model]]
- [[Occupancy Grid Mapping]] is improved by [[Maximum A Posteriori Occupancy Mapping]]
- [[Occupancy Grid Mapping]] is an adaptation of [[Binary Bayes Filter]]
- [[Simultaneous Localization and Mapping]] uses [[Extended Kalman Filter SLAM]]
- [[Simultaneous Localization and Mapping]] incorporates [[Landmark]]
- [[Extended Kalman Filter SLAM]] estimates [[Landmark]]
- [[Extended Kalman Filter SLAM]] uses [[Combined State Vector]]
