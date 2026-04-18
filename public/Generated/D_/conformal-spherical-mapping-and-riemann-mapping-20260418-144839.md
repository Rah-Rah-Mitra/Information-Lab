---
type: content
title: "Conformal Spherical Mapping and Riemann Mapping"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:48:39.467260400+00:00
summary: "Explains the computation of conformal maps from topological spheres and disks to unit spheres and disks using harmonic maps and Möbius transformations."
tags:
  - gis
  - differential-geometry
  - conformal-mapping
  - computer-vision
entities:
  - "[[Harmonic Map]]"
  - "[[Conformal Map]]"
  - "[[Topological Sphere]]"
  - "[[Unit Sphere]]"
  - "[[Möbius Transformation]]"
  - "[[Zero Mass-Center Constraint]]"
  - "[[Gauss Map]]"
  - "[[Riemann Mapping Theorem]]"
  - "[[Topological Disk]]"
  - "[[Unit Planar Disk]]"
  - "[[Stereographic Projection]]"
relationships:
  - source: "Harmonic Map"
    target: "Conformal Map"
    description: "is a type of"
  - source: "Conformal Map"
    target: "Topological Sphere"
    description: "maps from"
  - source: "Conformal Map"
    target: "Unit Sphere"
    description: "maps to"
  - source: "Möbius Transformation"
    target: "Unit Sphere"
    description: "transforms"
  - source: "Zero Mass-Center Constraint"
    target: "Harmonic Map"
    description: "ensures uniqueness of"
  - source: "Gauss Map"
    target: "Harmonic Map"
    description: "provides initial condition for"
  - source: "Riemann Mapping Theorem"
    target: "Topological Disk"
    description: "claims existence of conformal mapping for"
  - source: "Riemann Mapping Theorem"
    target: "Unit Planar Disk"
    description: "claims existence of conformal mapping to"
  - source: "Stereographic Projection"
    target: "Unit Sphere"
    description: "maps from"
  - source: "Stereographic Projection"
    target: "Unit Planar Disk"
    description: "maps to"
---

# Conformal Spherical Mapping and Riemann Mapping

*Explains the computation of conformal maps from topological spheres and disks to unit spheres and disks using harmonic maps and Möbius transformations.*

A [[Conformal Map]] is a mapping that preserves angles, and in the context of closed genus zero surfaces, [[Harmonic Map]]s between such surfaces are conformal.

## Conformal Spherical Mapping

To compute a conformal map from a [[Topological Sphere]] to a [[Unit Sphere]], an algorithm first constructs a degree-one map and evolves it to minimize harmonic energy via a nonlinear heat diffusion process. Because solutions are not unique, they are related by a [[Möbius Transformation]] of the unit sphere.

To ensure convergence and uniqueness, a [[Zero Mass-Center Constraint]] is applied, requiring the mass center of the mapping to coincide with the sphere center. The [[Gauss Map]] is typically used as the initial condition for this process.

## Riemann Mapping

The [[Riemann Mapping Theorem]] states that any simply connected domain (that is not the entire complex plane) can be mapped biholomorphically to a [[Unit Planar Disk]].

To compute this mapping for a [[Topological Disk]], the surface is first double-covered to form a closed genus zero surface, which is then mapped to a [[Unit Sphere]] using the spherical conformal mapping algorithm. A [[Möbius Transformation]] is then used to map the boundary to the equator, and a [[Stereographic Projection]] maps the lower hemisphere conformally onto the unit disk.

## Relationships

- [[Harmonic Map]] is a type of [[Conformal Map]]
- [[Conformal Map]] maps from [[Topological Sphere]] to [[Unit Sphere]]
- [[Möbius Transformation]] transforms [[Unit Sphere]]
- [[Zero Mass-Center Constraint]] ensures uniqueness of [[Harmonic Map]]
- [[Gauss Map]] provides initial condition for [[Harmonic Map]]
- [[Riemann Mapping Theorem]] claims existence of conformal mapping for [[Topological Disk]] to [[Unit Planar Disk]]
- [[Stereographic Projection]] maps from [[Unit Sphere]] to [[Unit Planar Disk]]
