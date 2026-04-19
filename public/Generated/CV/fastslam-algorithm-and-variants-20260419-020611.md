---
type: content
title: "FastSLAM Algorithm and Variants"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:06:11.082969800+00:00
summary: "FastSLAM is a particle filter-based approach to SLAM that exploits the conditional independence of map features given a robot path. It provides efficient online and offline solutions by maintaining separate local Gaussians for each feature. The algorithm scales logarithmically with map size when implemented with tree structures."
tags:
  - robotics
  - slam
  - particle-filters
  - fastslam
  - probabilistic-robotics
entities:
  - "[[FastSLAM]]"
  - "[[FastSLAM 1.0]]"
  - "[[FastSLAM 2.0]]"
  - "[[Particle Filter]]"
  - "[[Extended Kalman Filter]]"
  - "[[Conditional Independence]]"
  - "[[Data Association]]"
  - "[[Occupancy Grid Map]]"
  - "[[Loop Closure]]"
  - "[[Rao-Blackwellized Particle Filter]]"
relationships:
  - source: "FastSLAM"
    target: "Particle Filter"
    description: "uses"
  - source: "FastSLAM"
    target: "Conditional Independence"
    description: "exploits"
  - source: "FastSLAM 1.0"
    target: "Extended Kalman Filter"
    description: "uses"
  - source: "FastSLAM 2.0"
    target: "FastSLAM 1.0"
    description: "improves"
  - source: "FastSLAM"
    target: "Data Association"
    description: "handles"
  - source: "FastSLAM"
    target: "Occupancy Grid Map"
    description: "supports"
  - source: "FastSLAM"
    target: "Loop Closure"
    description: "enables"
---

# FastSLAM Algorithm and Variants

*FastSLAM is a particle filter-based approach to SLAM that exploits the conditional independence of map features given a robot path. It provides efficient online and offline solutions by maintaining separate local Gaussians for each feature. The algorithm scales logarithmically with map size when implemented with tree structures.*

## Concept
[[FastSLAM]] is a probabilistic SLAM algorithm that utilizes a [[Particle Filter]] to estimate the robot's trajectory while simultaneously maintaining a map. The core mathematical insight is that the full SLAM posterior can be factored because map features are [[Conditional Independence]] given the robot's path. This allows the [[FastSLAM]] algorithm to represent the map as a collection of independent local Gaussians, one for each feature, rather than a single high-dimensional joint Gaussian.

## FastSLAM 1.0 vs. 2.0
[[FastSLAM 1.0]] is the original version which samples new robot poses based solely on the motion model. While simple, it can be inefficient if the robot's sensors are much more accurate than its motion model. [[FastSLAM 2.0]] improves this by incorporating the current measurement into the pose sampling process, creating a more efficient proposal distribution.

$$ p(x_t | x_{t-1}, u_t, z_t, c_{1:t}) 
eq p(x_t | x_{t-1}, u_t) $$

The formula above illustrates the difference in the proposal distribution between the two versions.

## Data Association and Map Management
Unlike Gaussian-based SLAM algorithms like the [[Extended Kalman Filter]], [[FastSLAM]] can handle [[Data Association]] on a per-particle basis. This is because each particle maintains its own local data association decisions, allowing the filter to represent multiple hypotheses simultaneously. This makes the algorithm highly robust to incorrect associations. 

For efficient implementation, map features can be organized into a [[Binary Tree]] structure. This allows for logarithmic time complexity in updates and memory usage, as only the path to the modified feature needs to be updated when a new particle is generated.

## Grid-based FastSLAM
[[FastSLAM]] can also be extended to [[Occupancy Grid Map]] representations. In this variant, instead of modeling features as Gaussians, the algorithm uses occupancy grid mapping to represent the environment, which is suitable for arbitrary environments where landmarks are not predefined.

## Loop Closure
[[Loop Closure]] is a critical challenge in SLAM. In [[FastSLAM]], the ability to close a loop depends on the diversity of the particles. While the [[FastSLAM]] algorithm is efficient, it can lose long-range correlations due to the resampling process, which can lead to difficulties in large-scale loop closure compared to [[Extended Kalman Filter]] or [[GraphSLAM]].

## Relationships
- [[FastSLAM]] uses a [[Particle Filter]]
- [[FastSLAM]] exploits [[Conditional Independence]]
- [[FastSLAM 1.0]] uses [[Extended Kalman Filter]]
- [[FastSLAM 2.0]] improves [[FastSLAM 1.0]]
- [[FastSLAM]] handles [[Data Association]]
- [[FastSLAM]] supports [[Occupancy Grid Map]]
- [[FastSLAM]] enables [[Loop Closure]]
