---
type: content
title: "EKF SLAM vs GraphSLAM Algorithms"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:05:22.473132800+00:00
summary: "This note compares the Extended Kalman Filter (EKF) SLAM and GraphSLAM approaches to the simultaneous localization and mapping problem. EKF SLAM is an incremental, proactive algorithm that maintains a posterior over the momentary pose, while GraphSLAM is a lazy, graph-based optimization approach that solves the full SLAM problem. The choice between them involves trade-offs in computational complexity, data association robustness, and map scale."
tags:
  - robotics
  - slam
  - kalman-filter
  - graph-based-slam
  - optimization
entities:
  - "[[EKF SLAM]]"
  - "[[GraphSLAM]]"
  - "[[Simultaneous Localization and Mapping]]"
  - "[[Extended Kalman Filter]]"
  - "[[Maximum Likelihood Correspondence]]"
  - "[[Information Matrix]]"
  - "[[Information Vector]]"
  - "[[Data Association]]"
  - "[[Full SLAM Problem]]"
  - "[[Sparse Graph]]"
relationships:
  - source: "EKF SLAM"
    target: "Simultaneous Localization and Mapping"
    description: "solves"
  - source: "GraphSLAM"
    target: "Simultaneous Localization and Mapping"
    description: "solves"
  - source: "GraphFSLAM"
    target: "Information Matrix"
    description: "uses"
  - source: "EKF SLAM"
    target: "Extended Kalman Filter"
    description: "applies"
  - source: "GraphSLAM"
    target: "Sparse Graph"
    description: "extracts"
  - source: "GraphSLAM"
    target: "Full SLAM Problem"
    description: "addresses"
  - source: "EKF SLAM"
    target: "Data Association"
    description: "relies on"
  - source: "GraphSLAM"
    target: "Data Association"
    description: "improves"
  - source: "GraphSLAM"
    target: "Information Matrix"
    description: "produces"
  - source: "GraphSLAM"
    target: "Information Vector"
    description: "produces"
---

# EKF SLAM vs GraphSLAM Algorithms

*This note compares the Extended Kalman Filter (EKF) SLAM and GraphSLAM approaches to the simultaneous localization and mapping problem. EKF SLAM is an incremental, proactive algorithm that maintains a posterior over the momentary pose, while GraphSLAM is a lazy, graph-based optimization approach that solves the full SLAM problem. The choice between them involves trade-offs in computational complexity, data association robustness, and map scale.*

This note explores the two primary paradigms for solving the [[Simultaneous Localization and Mapping]] (SLAM) problem: the incremental [[EKF SLAM]] and the graph-based [[GraphSLAM]].

## Concept

[[Simultaneous Localization and Mapping]] is the concurrent task of a robot acquiring a map of its environment while simultaneously localizing itself relative to that map. The text distinguishes between the online problem (momentary pose) and the global problem (all poses).

### EKF SLAM
[[EKF SLAM]] is an incremental approach that applies the [[Extended Kalman Filter]] to the online SLAM problem. It is characterized as "proactive" because it resolves new information immediately into an improved estimate of the state. However, it suffers from a significant limitation: its update complexity is quadratic in the number of landmarks, which limits it to sparse maps with fewer than 1,000 features. Furthermore, its data association is often brittle because it relies on an incremental [[Maximum Likelihood Correspondence]] estimator, which cannot revise past decisions.

### GraphSLAM
In contrast, [[GraphSLAM]] is a "lazy" algorithm that solves the [[Full SLAM Problem]] by representing information as a [[Sparse Graph]] of soft constraints. Instead of updating a covariance matrix immediately, it accumulates information into a graph where nodes are robot poses and edges are motion or measurement constraints. This approach allows for much larger maps and more robust [[Data Association]] because decisions can be undone or revised during the optimization phase.

GraphSLAM represents the information in an information-theoretic form using an [[Information Matrix]] and an [[Information Vector]]. The relationship between the variables is captured by the sum of nonlinear quadratic constraints, which is minimized via nonlinear least squares. The reduction of the graph is achieved through a process similar to variable elimination, which transforms the graph into a smaller one defined only over robot poses.

$$ \Omega \text{ is the information matrix, } \xi 	ext{ is the information vector} $$

This expression represents the quadratic form of the negative log-posterior in the information form.

## Relationships
- [[EKF SLAM]] solves [[Simultaneous Localization and Mapping]]
- [[GraphSLAM]] solves [[Simultaneous Localization and Mapping]]
- [[GraphSLAM]] produces [[Information Matrix]]
- [[GraphSLAM]] produces [[Information Vector]]
- [[GraphSLAM]] uses [[Sparse Graph]]
- [[GraphSLAM]] addresses [[Full SLAM Problem]]
- [[EKF SLAM]] applies [[Extended Kalman Filter]]
- [[EKF SLAM]] relies on [[Data Association]]
- [[GraphSLAM]] improves [[Data Association]]
