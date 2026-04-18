---
type: content
title: "Metric-Driven N-RoSy Fields Design"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:49:10.344161700+00:00
summary: "An algorithm for designing smooth vector fields on surfaces using flat cone metrics, parallel transport, and holonomy compensation."
tags:
  - gis
  - computer-vision
  - differential-geometry
  - vector-fields
  - mesh-processing
entities:
  - "[[N-RoSy Field]]"
  - "[[Flat Cone Metric]]"
  - "[[Parallel Transport]]"
  - "[[Holonomy]]"
  - "[[Relative Rotation]]"
  - "[[Turning Number]]"
  - "[[Discrete Euclidean Ricci Flow]]"
  - "[[Harmonic 1-form]]"
  - "[[Birkhoff Curve Shortening]]"
  - "[[Hyperbolic Uniformization Metric]]"
  - "[[Fuchsian Group]]"
  - "[[Teichmüller Space]]"
relationships:
  - source: "N-RoSy Field"
    target: "Flat Cone Metric"
    description: "is designed using"
  - source: "Flat Cone Metric"
    target: "Parallel Transport"
    description: "enables"
  - source: "Parallel Transport"
    target: "Holonomy"
    description: "defines"
  - source: "Holonomy"
    target: "Relative Rotation"
    description: "is subtracted from absolute rotation to find"
  - source: "Relative Rotation"
    target: "Turning Number"
    description: "is equivalent to"
  - source: "Flat Cone Metric"
    target: "Discrete Euclidean Ricci Flow"
    description: "is computed via"
  - source: "N-RoSy Field"
    target: "Harmonic 1-form"
    description: "is smoothed using"
  - source: "Birkhoff Curve Shortening"
    target: "Hyperbolic Uniformization Metric"
    description: "converges under"
  - source: "Hyperbolic Uniformization Metric"
    target: "Fuchsian Group"
    description: "is used to compute"
  - source: "Fuchsian Group"
    target: "Teichmüller Space"
    description: "provides coordinates for"
---

# Metric-Driven N-RoSy Fields Design

*An algorithm for designing smooth vector fields on surfaces using flat cone metrics, parallel transport, and holonomy compensation.*

An [[N-RoSy Field]] is a vector field on a surface with local rotational symmetry that is invariant under rotations of an integer multiple of $2\pi/N$.

## Metric-Driven Design

The design of these fields relies on the construction of a [[Flat Cone Metric]], where curvatures are zero everywhere except at a few discrete cone singularities. The algorithm uses [[Parallel Transport]] to construct a parallel vector field; however, transporting a vector along a closed loop often results in a rotation relative to the original vector, known as the [[Holonomy]] of the loop.

To ensure the field is smooth, the algorithm calculates the [[Relative Rotation]], which is the difference between the absolute rotation of the field and the holonomy of the loop. This relative rotation is equivalent to the [[Turning Number]]. For a smooth N-RoSy field, the turning number along any loop must be an integer multiple of $2\pi/N$.

## Computational Pipeline

1. **Metric Computation**: A [[Flat Cone Metric]] is computed using [[Discrete Euclidean Ricci Flow]] based on user-specified singularity positions and indices.
2. **Holonomy Compensation**: If the parallel field has jumps (discontinuities), they are eliminated via:
    - **Rotation-based compensation**: Using a [[Harmonic 1-form]] to adjust the absolute rotation.
    - **Metric-based compensation**: Deforming the surface to modify the holonomy.
3. **Editing**: Users can interactively modify the field's orientation and magnitude using harmonic functions.

## Related Geometric Concepts

For surfaces with non-trivial topology (genus > 1), a [[Hyperbolic Uniformization Metric]] can be used to find unique closed geodesics in each homotopy class. This process often involves the [[Fuchsian Group]], which consists of Möbius transformations acting on the universal covering space (the Poincaré disk). These geometric invariants are fundamental to defining coordinates in [[Teichmüller Space]], which classifies surfaces based on conformal equivalence.

To find the shortest homotopic cycles, the [[Birkhoff Curve Shortening]] method is employed, which iteratively replaces arcs with geodesics, converging to the unique homotopy geodesic under a hyperbolic metric.

## Relationships
- [[N-RoSy Field]] is designed using [[Flat Cone Metric]]
- [[Flat Cone Metric]] enables [[Parallel Transport]]
- [[Parallel Transport]] defines [[Holonomy]]
- [[Holonomy]] is subtracted from absolute rotation to find [[Relative Rotation]]
- [[Relative Rotation]] is equivalent to [[Turning Number]]
- [[Flat Cone Metric]] is computed via [[Discrete Euclidean Ricci Flow]]
- [[N-RoSy Field]] is smoothed using [[Harmonic 1-form]]
- [[Birkhoff Curve Shortening]] converges under [[Hyperbolic Uniformization Metric]]
- [[Hyperbolic Uniformization Metric]] is used to compute [[Fuchsian Group]]
- [[Fuchsian Group]] provides coordinates for [[Teichmüller Space]]
