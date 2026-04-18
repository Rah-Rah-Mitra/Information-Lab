---
type: content
title: "Global Transformation-Invariant Shape Descriptors"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:50:53.896816600+00:00
summary: "An overview of conformal geometry tools, including spherical harmonics, Teichmüller space, and Wasserstein distance, for invariant brain surface analysis."
tags:
  - gis
  - medical-imaging
  - conformal-geometry
  - shape-analysis
  - differential-geometry
entities:
  - "[[Spherical Harmonic Shape Descriptor]]"
  - "[[Teichmüller Space]]"
  - "[[Conformal Module]]"
  - "[[Conformal Welding]]"
  - "[[Riemannian Optimal Mass Transport]]"
  - "[[Riemannian Wasserstein Distance]]"
  - "[[Hyperbolic Wasserstein Distance]]"
  - "[[Spherical Wasserstein Distance]]"
  - "[[Poincaré Disk]]"
  - "[[Geodesic Power Voronoi Diagram]]"
  - "[[Laplace–Beltrami Operator]]"
  - "[[Möbius Transformation]]"
relationships:
  - source: "Spherical Harmonic Shape Descriptor"
    target: "Laplace–Beltrami Operator"
    description: "is based on eigenfunctions of the"
  - source: "Teichmüller Space"
    target: "Conformal Module"
    description: "is parameterized by"
  - source: "Conformal Welding"
    target: "Teichmüller Space"
    description: "generalizes 2D shape space to 3D"
  - source: "Riemannian Optimal Mass Transport"
    target: "Riemannian Wasserstein Distance"
    description: "induces the"
  - source: "Riemannian Wasserstein Distance"
    target: "Spherical Wasserstein Distance"
    description: "is implemented as"
  - source: "Riemannian Wasserstein Distance"
    target: "Hyperbolic Wasserstein Distance"
    description: "is implemented as"
  - source: "Hyperbolic Wasserstein Distance"
    target: "Poincaré Disk"
    description: "is computed on the"
  - source: "Riemannian Optimal Mass Transport"
    target: "Geodesic Power Voronoi Diagram"
    description: "is computed using a"
  - source: "Conformal Module"
    target: "Möbius Transformation"
    description: "is normalized by"
---

# Global Transformation-Invariant Shape Descriptors

*An overview of conformal geometry tools, including spherical harmonics, Teichmüller space, and Wasserstein distance, for invariant brain surface analysis.*

[[Global Transformation-Invariant Shape Descriptors]] are mathematical tools used to analyze the geometry of brain surfaces in a way that is independent of the surface's orientation, position, or scale.

## Spherical Harmonic Analysis

A [[Spherical Harmonic Shape Descriptor]] represents a brain surface by conformally mapping it to a sphere. The surface is decomposed into a linear combination of spherical harmonics, which are eigenfunctions of the [[Laplace–Beltrami Operator]]. By filtering high-frequency coefficients, the geometry can be smoothed and compressed, while low-frequency coefficients are used to match surfaces and compute similarity.

## Teichmüller Space and Conformal Welding

[[Teichmüller Space]] is a manifold consisting of conformal equivalence classes of Riemann surfaces. For genus-zero surfaces with boundaries, the space is parameterized by the [[Conformal Module]], which describes the centers and radii of circle domains after a [[Möbius Transformation]] normalization.

[[Conformal Welding]] is a process where a surface is sliced into connected components, each mapped to a circle domain. The resulting signature, combining conformal modules and diffeomorphisms of the curves, uniquely determines the shape up to a conformal automorphism.

## Riemannian Wasserstein Distance

[[Riemannian Optimal Mass Transport]] (OMT) generalizes the transport of probability measures from Euclidean to Riemannian manifolds. The total transportation cost induced by the OMT map defines the [[Riemannian Wasserstein Distance]].

### Spherical and Hyperbolic Variants
- **[[Spherical Wasserstein Distance]]**: Computed by mapping surfaces to a unit sphere and using a [[Geodesic Power Voronoi Diagram]] to find the optimal transport map.
- **[[Hyperbolic Wasserstein Distance]]**: Computed by mapping multiply connected surfaces (sliced along landmark curves) onto the [[Poincaré Disk]] using hyperbolic Ricci flow.

## Relationships
- [[Spherical Harmonic Shape Descriptor]] is based on eigenfunctions of the [[Laplace–Beltrami Operator]].
- [[Teichmüller Space]] is parameterized by [[Conformal Module]].
- [[Conformal Welding]] generalizes 2D shape space to 3D [[Teichmüller Space]].
- [[Riemannian Optimal Mass Transport]] induces the [[Riemannian Wasserstein Distance]].
- [[Riemannian Wasserstein Distance]] is implemented as [[Spherical Wasserstein Distance]] and [[Hyperbolic Wasserstein Distance]].
- [[Hyperbolic Wasserstein Distance]] is computed on the [[Poincaré Disk]].
- [[Riemannian Optimal Mass Transport]] is computed using a [[Geodesic Power Voronoi Diagram]].
- [[Conformal Module]] is normalized by [[Möbius Transformation]].
