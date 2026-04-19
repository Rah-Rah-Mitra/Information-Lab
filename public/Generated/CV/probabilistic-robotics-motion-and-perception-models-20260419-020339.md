---
type: content
title: "Probabilistic Robotics Motion and Perception Models"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:03:39.859897600+00:00
summary: "This note covers the fundamental probabilistic models for robot motion and sensor perception. It details how velocity and odometry-based motion models handle uncertainty, and how beam models approximate sensor measurement likelihoods using mixture densities. These models are are essential for state estimation in mobile robotics."
tags:
  - probabilistic-robotics
  - motion-models
  - sensor-perception
  - state-estimation
  - particle-filters
entities:
  - "[[Probabilistic Robotics]]"
  - "[[Particle Filter]]"
  - "[[Velocity Motion Model]]"
  - "[[Velocity]]"
  - "[[Rotational Velocity]]"
  - "[[Odometry Motion Model]]"
  - "[[Odometry]]"
  - "[[Kinematic Configuration]]"
  - "[[Pose]]"
  - "[[Measurement Model]]"
  - "[[Beam Model]]"
  - "[[Occupancy Map]]"
  - "[[Gaussian Distribution]]"
  - "[[Exponential Distribution]]"
  - "[[Triangular Distribution]]"
  - "[[Mixture Density]]"
relationships:
  - source: "Particle Filter"
    target: "Velocity Motion Model"
    description: "utilizes"
  - source: "Particle Filter"
    target: "Odometry Motion Model"
    description: "uses"
  - source: "Velocity Motion Model"
    target: "Velocity"
    description: "depends on"
  - source: "Velocity Motion Model"
    target: "Rotational Velocity"
    description: "depends on"
  - source: "Odometry Motion Model"
    target: "Odometry"
    description: "uses"
  - source: "Odometry Motion Model"
    target: "Pose"
    description: "updates"
  - source: "Measurement Model"
    target: "Beam Model"
    description: "is implemented via"
  - source: "Measurement Model"
    target: "Occupancy Map"
    description: "is conditioned on"
  - source: "Beam Model"
    target: "Mixture Density"
    description: "uses"
  - source: "Mixture Density"
    target: "Gaussian Distribution"
    description: "includes"
  - source: "Mixture Density"
    target: "Exponential Distribution"
    description: "includes"
  - source: "Mixture Density"
    target: "Triangular Distribution"
    description: "includes"
---

# Probabilistic Robotics Motion and Perception Models

*This note covers the fundamental probabilistic models for robot motion and sensor perception. It details how velocity and odometry-based motion models handle uncertainty, and how beam models approximate sensor measurement likelihoods using mixture densities. These models are are essential for state estimation in mobile robotics.*

This note explores the mathematical foundations of motion and perception in [[Probabilistic Robotics]].

## Concept
In probabilistic robotics, the goal is to estimate the robot's state (typically its [[Pose]]) from noisy control inputs and sensor measurements. This is achieved through two primary components: motion models and measurement models.

### Motion Models
Motion models describe the state transition probability $p(x_t | u_t, x_{t-1})$. There are two main types discussed:

1. **[[Velocity Motion Model]]**: This model assumes control via [[Velocity]] (translational) and [[Rotational Velocity]] ($\omega_t$). To avoid degeneracy in the 3D pose space, a third noise parameter for a "final rotation" $\hat{\gamma}$ is added. The model is derived from circular trajectories with radius $r = v / \omega$.

2. **[[Odometry Motion Model]]**: This model uses [[Odometry]] measurements rather than commanded velocities. It decomposes relative motion into a rotation, a translation, and another rotation. This is often more accurate than velocity models because it avoids the mismatch between controllers and physical motion.

### Measurement Models
Measurement models describe the sensor formation process $p(z_t | x_t, m)$. A common approach for range finders is the [[Beam Model]], which uses a [[Mixture Density]] to represent the likelihood of a measurement $z_t$. This mixture typically includes:
- [[Gaussian Distribution]] for local measurement noise.
- [[Exponential Distribution]] for unexpected objects (e.g., people).
- A point-mass distribution for sensor failures (max-range measurements).
- A uniform distribution for unexplainable measurements.

These models are often conditioned on an [[Occupancy Map]] $m$, which provides information about traversable vs. occupied space.

## Relationships
- [[Particle Filter]] utilizes [[Velocity Motion Model]]
- [[Particle Filter]] uses [[Odometry Motion Model]]
- [[Velocity Motion Model]] depends on [[Velocity]]
- [[Velocity Motion Model]] depends on [[Rotational Velocity]]
- [[Odometry Motion Model]] uses [[Odometry]]
- [[Odometry Motion Model]] updates [[Pose]]
- [[Measurement Model]] is implemented via [[Beam Model]]
- [[Measurement Model]] is conditioned on [[Occupancy Map]]
- [[Beam Model]] uses [[Mixture Density]]
- [[Mixture Density]] includes [[Gaussian Distribution]]
- [[Mixture Density]] includes [[Exponential Distribution]]
- [[Mixture Density]] includes [[Triangular Distribution]]

