---
type: content
title: "Discrete Differential Geometry for Triangle Meshes"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:41:04.562104800+00:00
summary: "An overview of discrete differential operators, including the Laplace-Beltrami operator and curvature measures, applied to polygonal triangle meshes."
tags:
  - gis
  - computer-vision
  - differential-geometry
  - geometry-processing
  - triangle-meshes
entities:
  - "[[Laplace-Beltrami Operator]]"
  - "[[Mean Curvature]]"
  - "[[Gaussian Curvature]]"
  - "[[First Fundamental Form]]"
  - "[[Second Fundamental Form]]"
  - "[[Principal Curvatures]]"
  - "[[Cotangent Formula]]"
  - "[[Uniform Laplacian]]"
  - "[[Manifold Harmonics]]"
  - "[[Diffusion Flow]]"
  - "[[Surface Fairing]]"
  - "[[Voronoi Cell]]"
relationships:
  - source: "Laplace-Beltrami Operator"
    target: "Mean Curvature"
    description: "evaluates to the mean curvature normal when applied to coordinate functions"
  - source: "First Fundamental Form"
    target: "Laplace-Beltrami Operator"
    description: "determines the intrinsic metric used by the"
  - source: "Second Fundamental Form"
    target: "Principal Curvatures"
    description: "is used to define"
  - source: "Cotangent Formula"
    target: "Laplace-Beltrami Operator"
    description: "provides a widely used discretization of the"
  - source: "Uniform Laplacian"
    target: "Laplace-Beltrami Operator"
    description: "provides a simple but geometry-independent discretization of the"
  - source: "Manifold Harmonics"
    target: "Laplace-Beltrami Operator"
    description: "are the eigenfunctions of the"
  - source: "Diffusion Flow"
    target: "Laplace-Beltrami Operator"
    description: "is modeled by the heat equation using the"
  - source: "Surface Fairing"
    target: "Laplace-Beltrami Operator"
    description: "minimizes fairness energies by solving systems involving the"
  - source: "Voronoi Cell"
    target: "Laplace-Beltrami Operator"
    description: "serves as a local averaging region for the discretization of the"
  - source: "Principal Curvatures"
    target: "Mean Curvature"
    description: "their average defines the"
  - source: "Principal Curvatures"
    target: "Gaussian Curvature"
    description: "their product defines the"
---

# Discrete Differential Geometry for Triangle Meshes

*An overview of discrete differential operators, including the Laplace-Beltrami operator and curvature measures, applied to polygonal triangle meshes.*

Discrete differential geometry provides the tools to approximate smooth surface properties on piecewise linear polygonal meshes. By interpreting triangle meshes as approximations of smooth manifolds, operators like the [[Laplace-Beltrami Operator]] can be computed directly from mesh connectivity and geometry.

## Discrete Differential Operators

### The Laplace-Beltrami Operator
The [[Laplace-Beltrami Operator]] is a fundamental intrinsic operator that generalizes the Laplacian to manifold surfaces. In the discrete setting, two primary discretizations are used:
- **[[Uniform Laplacian]]**: A simple graph-based approach that depends only on connectivity, making it unsuitable for non-uniform meshes but useful for isotropic remeshing.
- **[[Cotangent Formula]]**: A more accurate discretization based on finite element/volume methods that accounts for the geometry of the mesh via the cotangents of angles opposite to an edge.

### Curvature Measures
Surface curvature is characterized by the [[Principal Curvatures]] ($\kappa_1, \kappa_2$), which are the extremal values of normal curvature. From these, two key scalars are derived:
- **[[Mean Curvature]]**: The average of the principal curvatures ($H = \frac{\kappa_1 + \kappa_2}{2}$).
- **[[Gaussian Curvature]]**: The product of the principal curvatures ($K = \kappa_1 \kappa_2$).

Discrete approximations of these are computed using local averaging regions, such as the [[Voronoi Cell]] or barycentric cells. The discrete Gaussian curvature is often derived from the Gauss-Bonnet theorem as the angle deficit at a vertex.

## Metric Properties

The [[First Fundamental Form]] (or metric tensor) encodes how distances, angles, and areas are transformed from parameter space to the surface. The [[Second Fundamental Form]] describes the local embedding of the surface in $\mathbb{R}^3$ and is used to determine the principal curvatures.

## Applications in Geometry Processing

### Spectral Analysis
[[Manifold Harmonics]] are the eigenfunctions of the [[Laplace-Beltrami Operator]]. They provide a generalized Fourier transform for meshes, allowing for ideal low-pass filtering and signal processing on arbitrary geometries.

### Smoothing and Fairing
- **[[Diffusion Flow]]**: A time-dependent process (heat equation) that removes high-frequency noise by evolving the surface according to the [[Laplace-Beltrami Operator]].
- **[[Surface Fairing]]**: The process of computing the "simplest" shape by minimizing fairness energies (e.g., membrane energy or thin-plate energy), which results in solving linear systems involving the Laplacian or bi-Laplacian operators.

## Relationships
- [[Laplace-Beltrami Operator]] evaluates to the mean curvature normal when applied to coordinate functions.
- [[First Fundamental Form]] determines the intrinsic metric used by the [[Laplace-Beltrami Operator]].
- [[Second Fundamental Form]] is used to define [[Principal Curvatures]].
- [[Cotangent Formula]] provides a widely used discretization of the [[Laplace-Beltrami Operator]].
- [[Uniform Laplacian]] provides a simple but geometry-independent discretization of the [[Laplace-Beltrami Operator]].
- [[Manifold Harmonics]] are the eigenfunctions of the [[Laplace-Beltrami Operator]].
- [[Diffusion Flow]] is modeled by the heat equation using the [[Laplace-Beltrami Operator]].
- [[Surface Fairing]] minimizes fairness energies by solving systems involving the [[Laplace-Beltrami Operator]].
- [[Voronoi Cell]] serves as a local averaging region for the discretization of the [[Laplace-Beltrami Operator]].
- [[Principal Curvatures]] their average defines the [[Mean Curvature]] and their product defines the [[Gaussian Curvature]].
