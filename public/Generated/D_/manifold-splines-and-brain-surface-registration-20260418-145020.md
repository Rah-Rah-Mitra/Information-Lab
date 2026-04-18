---
type: content
title: "Manifold Splines and Brain Surface Registration"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:50:20.568843700+00:00
summary: "An exploration of manifold splines for arbitrary topology modeling and the application of conformal geometry to brain surface parameterization and registration."
tags:
  - gis
  - computer-vision
  - differential-geometry
  - medical-imaging
  - splines
entities:
  - "[[Manifold Spline]]"
  - "[[Manifold T-spline]]"
  - "[[Manifold Triangular B-spline]]"
  - "[[Extraordinary Point]]"
  - "[[Affine Atlas]]"
  - "[[Ricci Flow]]"
  - "[[Conformal Parameterization]]"
  - "[[Brain Surface Registration]]"
  - "[[Harmonic Map]]"
  - "[[Hyperbolic Harmonic Map]]"
  - "[[Poincaré Disk]]"
  - "[[Klein Model]]"
relationships:
  - source: "Manifold Spline"
    target: "Affine Atlas"
    description: "depends on"
  - source: "Manifold T-spline"
    target: "Manifold Spline"
    description: "is a type of"
  - source: "Manifold Triangular B-spline"
    target: "Manifold Spline"
    description: "is a type of"
  - source: "Extraordinary Point"
    target: "Manifold Spline"
    description: "appears as a singularity in"
  - source: "Ricci Flow"
    target: "Affine Atlas"
    description: "is used to compute"
  - source: "Conformal Parameterization"
    target: "Brain Surface Registration"
    description: "facilitates"
  - source: "Harmonic Map"
    target: "Brain Surface Registration"
    description: "is used for"
  - source: "Hyperbolic Harmonic Map"
    target: "Harmonic Map"
    description: "generalizes"
  - source: "Hyperbolic Harmonic Map"
    target: "Poincaré Disk"
    description: "is computed on"
  - source: "Hyperbolic Harmonic Map"
    target: "Klein Model"
    description: "utilizes"
---

# Manifold Splines and Brain Surface Registration

*An exploration of manifold splines for arbitrary topology modeling and the application of conformal geometry to brain surface parameterization and registration.*

A [[Manifold Spline]] is a generalization of planar splines to surfaces of arbitrary topology, utilizing an [[Affine Atlas]] to define basis functions across coordinate charts.

## Manifold Spline Types

### Manifold Triangular B-splines
[[Manifold Triangular B-spline]]s are used to model surfaces with various genus, often employing [[Ricci Flow]] to minimize the number of [[Extraordinary Point]]s (singularities) in the affine structure.

### Manifold T-splines
[[Manifold T-spline]]s generalize NURBS by using a T-junction mechanism to reduce superfluous control points and improve local refinement. They require the domain manifold faces to be rectangular.

## Handling Singularities

Because of topological obstructions, [[Extraordinary Point]]s are inevitable for closed manifolds other than tori. In practice, these are handled by removing the singularity and filling the resulting hole with a spline surface that minimizes thin-plate energy.

## Brain Surface Analysis

In medical imaging, [[Conformal Parameterization]] is used to map complex brain cortical surfaces to canonical spaces (sphere, plane, or hyperbolic space) to facilitate [[Brain Surface Registration]].

### Registration Methods
- **Harmonic Maps**: [[Harmonic Map]]s are used to register surfaces by solving Laplace's equation on the parameter domain, providing a numerically stable diffeomorphism for convex domains.
- **Hyperbolic Harmonic Maps**: To handle high-genus surfaces or those with negative Euler numbers, [[Hyperbolic Harmonic Map]]s are employed. This process involves:
    1. Topology optimization via landmark cuts.
    2. Computing a hyperbolic metric using [[Ricci Flow]].
    3. Embedding the surface into the [[Poincaré Disk]] and converting to the [[Klein Model]] for convex hexagon mapping.
    4. Applying non-linear heat diffusion to achieve a global harmonic diffeomorphism.

## Relationships
- Manifold Spline depends on Affine Atlas
- Manifold T-spline is a type of Manifold Spline
- Manifold Triangular B-spline is a type of Manifold Spline
- Extraordinary Point appears as a singularity in Manifold Spline
- Ricci Flow is used to compute Affine Atlas
- Conformal Parameterization facilitates Brain Surface Registration
- Harmonic Map is used for Brain Surface Registration
- Hyperbolic Harmonic Map generalizes Harmonic Map
- Hyperbolic Harmonic Map is computed on Poincaré Disk
- Hyperbolic Harmonic Map utilizes Klein Model
