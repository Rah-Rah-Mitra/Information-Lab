---
type: content
title: "Probabilistic Robot Motion Models"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:45:57.909333900+00:00
summary: "An overview of probabilistic motion models for mobile robots, including velocity, odometry, and map-based models to handle actuation uncertainty."
tags:
  - gis
  - robotics
  - probabilistic-robotics
  - state-estimation
  - kinematics
entities:
  - "[[Probabilistic Motion Model]]"
  - "[[Velocity Motion Model]]"
  - "[[Odometry Motion Model]]"
  - "[[Map-Based Motion Model]]"
  - "[[Particle Filter]]"
  - "[[Robot Pose]]"
  - "[[Configuration Space]]"
  - "[[Kinematics]]"
  - "[[Sampling]]"
  - "[[Low Variance Sampler]]"
  - "[[Particle Deprivation]]"
  - "[[Sampling Bias]]"
relationships:
  - source: "Probabilistic Motion Model"
    target: "Robot Pose"
    description: "estimates the distribution of"
  - source: "Velocity Motion Model"
    target: "Probabilistic Motion Model"
    description: "is a type of"
  - source: "Odometry Motion Model"
    target: "Probabilistic Motion Model"
    description: "is a type of"
  - source: "Map-Based Motion Model"
    target: "Probabilistic Motion Model"
    description: "is a type of"
  - source: "Map-Based Motion Model"
    target: "Configuration Space"
    description: "constrains poses to the free space of"
  - source: "Particle Filter"
    target: "Probabilistic Motion Model"
    description: "utilizes sampling from"
  - source: "Low Variance Sampler"
    target: "Particle Filter"
    description: "reduces sampling variance in"
  - source: "Particle Deprivation"
    target: "Particle Filter"
    description: "is a failure mode of"
  - source: "Sampling Bias"
    target: "Particle Filter"
    description: "is a systematic error in"
  - source: "Kinematics"
    target: "Robot Pose"
    description: "describes the effect of control on"
---

# Probabilistic Robot Motion Models

*An overview of probabilistic motion models for mobile robots, including velocity, odometry, and map-based models to handle actuation uncertainty.*

A [[Probabilistic Motion Model]] is a conditional probability density that describes the posterior distribution over a robot's [[Robot Pose]] after executing a specific motion command, accounting for uncertainty in actuation.

## Motion Model Types

### Velocity Motion Model
The [[Velocity Motion Model]] assumes control via translational and rotational velocities. To avoid degeneracy in the posterior distribution, the model incorporates three noise parameters: one for translational velocity, one for rotational velocity, and a third for a "final rotation" upon arrival at the destination pose.

### Odometry Motion Model
The [[Odometry Motion Model]] uses relative motion information (rotation, translation, rotation) measured by internal encoders rather than commanded velocities. It is generally more accurate than the velocity model but is only available retrospectively, making it less suitable for motion planning.

### Map-Based Motion Model
The [[Map-Based Motion Model]] incorporates environmental knowledge (e.g., occupancy maps) to refine the motion estimate. It ensures that the resulting [[Robot Pose]] is consistent with the map by assigning zero probability to poses that would place the robot in occupied space, effectively mapping obstacles from the workspace to the [[Configuration Space]].

## Implementation in Particle Filters

[[Particle Filter]] implementations typically rely on [[Sampling]] rather than closed-form density calculations because forward simulation of the physical motion model is computationally simpler than inverting it.

### Variance and Error Reduction
To improve the efficiency of the [[Particle Filter]], a [[Low Variance Sampler]] can be used to systematically cover the sample space and reduce the variance of the estimate. 

### Common Failure Modes
- **[[Sampling Bias]]**: A systematic error introduced when using a finite number of particles, where the resampling step can deterministically accept samples regardless of their importance weights.
- **[[Particle Deprivation]]**: Occurs when no particles exist in the vicinity of the true state, often due to an unlucky series of random samples or an insufficient number of particles relative to the state space dimensionality.

## Relationships
- [[Probabilistic Motion Model]] regulates [[Robot Pose]]
- [[Velocity Motion Model]] is a type of [[Probabilistic Motion Model]]
- [[Odometry Motion Model]] is a type of [[Probabilistic Motion Model]]
- [[Map-Based Motion Model]] is a type of [[Probabilistic Motion Model]]
- [[Map-Based Motion Model]] depends on [[Configuration Space]]
- [[Particle Filter]] depends on [[Probabilistic Motion Model]]
- [[Low Variance Sampler]] regulates [[Particle Filter]]
- [[Particle Deprivation]] depends on [[Particle Filter]]
- [[Sampling Bias]] depends on [[Particle Filter]]
- [[Kinematics]] regulates [[Robot Pose]]
