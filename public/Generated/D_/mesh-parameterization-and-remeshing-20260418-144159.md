---
type: content
title: "Mesh Parameterization and Remeshing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:41:59.412576800+00:00
summary: "An overview of techniques for attaching coordinate systems to 3D surfaces and improving mesh quality through isotropic and quad-dominant remeshing."
tags:
  - gis
  - computer-vision
  - geometry-processing
  - mesh-parameterization
  - remeshing
entities:
  - "[[Mesh Parameterization]]"
  - "[[Barycentric Mapping]]"
  - "[[Conformal Mapping]]"
  - "[[Least Squares Conformal Maps]]"
  - "[[Cauchy-Riemann Equations]]"
  - "[[Dirichlet Energy]]"
  - "[[Angle-Based Flattening]]"
  - "[[Distortion Analysis]]"
  - "[[Remeshing]]"
  - "[[Isotropic Remeshing]]"
  - "[[Quad-Dominant Remeshing]]"
  - "[[Centroidal Voronoi Tessellation]]"
  - "[[Delaunay Triangulation]]"
  - "[[Voronoi Diagram]]"
relationships:
  - source: "Mesh Parameterization"
    target: "Barycentric Mapping"
    description: "is implemented via"
  - source: "Mesh Parameterization"
    target: "Conformal Mapping"
    description: "is implemented via"
  - source: "Conformal Mapping"
    target: "Least Squares Conformal Maps"
    description: "is realized by"
  - source: "Conformal Mapping"
    target: "Cauchy-Riemann Equations"
    description: "is characterized by"
  - source: "Conformal Mapping"
    target: "Dirichlet Energy"
    description: "minimizes"
  - source: "Mesh Parameterization"
    target: "Angle-Based Flattening"
    description: "is implemented via"
  - source: "Mesh Parameterization"
    target: "Distortion Analysis"
    description: "is evaluated using"
  - source: "Remeshing"
    target: "Isotropic Remeshing"
    description: "includes"
  - source: "Remeshing"
    target: "Quad-Dominant Remeshing"
    description: "includes"
  - source: "Isotropic Remeshing"
    target: "Centroidal Voronoi Tessellation"
    description: "utilizes"
  - source: "Isotropic Remeshing"
    target: "Delaunay Triangulation"
    description: "utilizes"
  - source: "Isotropic Remeshing"
    target: "Voronoi Diagram"
    description: "utilizes"
  - source: "Quad-Dominant Remeshing"
    target: "Mesh Parameterization"
    description: "often relies on"
---

# Mesh Parameterization and Remeshing

*An overview of techniques for attaching coordinate systems to 3D surfaces and improving mesh quality through isotropic and quad-dominant remeshing.*

[[Mesh Parameterization]] is the process of attaching a geometric coordinate system to a 3D object, effectively creating a one-to-one correspondence between a 3D surface and a 2D domain.

## Parameterization Methods

### Barycentric Mapping
[[Barycentric Mapping]] is a widely used method based on Tutte's theorem, which ensures a valid parameterization without self-intersections if boundary vertices are fixed to a convex polygon and internal vertices are convex combinations of their neighbors.

### Conformal Mapping
[[Conformal Mapping]] focuses on preserving angles, where the anisotropy ellipse at every point is a circle. This is formally characterized by the [[Cauchy-Riemann Equations]].
- **Least Squares Conformal Maps (LSCM)**: [[Least Squares Conformal Maps]] minimize a conformal energy to find a parameterization that is as conformal as possible.
- **Energy Relations**: Conformal maps are related to harmonic functions, and the minimization of [[Dirichlet Energy]] is often used to achieve these mappings.

### Angle-Based Flattening
[[Angle-Based Flattening]] (ABF) is a geometric method that reformulates the parameterization problem in terms of angles rather than coordinates, minimizing the deviation of 2D angles from their 3D counterparts.

## Distortion Analysis
[[Distortion Analysis]] provides the metric tools to evaluate how a signal is deformed during mapping. Key types include:
- **Conformal**: Angle-preserving.
- **Equiareal**: Area-preserving.
- **Isometric**: Both angle and area preserving (only possible for developable surfaces).

## Remeshing
[[Remeshing]] is the process of improving mesh quality by redistributing vertices and edges while approximating the original surface.

### Isotropic Remeshing
[[Isotropic Remeshing]] aims for elements that are locally uniform in all directions (e.g., equilateral triangles). Techniques include:
- **Greedy Approaches**: Using [[Delaunay Triangulation]] refinement and filtering.
- **Variational Approaches**: Utilizing [[Centroidal Voronoi Tessellation]] (CVT) and Lloyd relaxation to evenly distribute points.
- **Incremental Approaches**: Iteratively splitting long edges, collapsing short edges, and equalizing valences.

### Quad-Dominant Remeshing
[[Quad-Dominant Remeshing]] produces meshes consisting primarily of quadrangles, which are better for capturing symmetries and principal curvature directions. This often involves tracing lines of curvatures on the surface.

## Relationships
- [[Mesh Parameterization]] is implemented via [[Barycentric Mapping]], [[Conformal Mapping]], and [[Angle-Based Flattening]].
- [[Mesh Parameterization]] is evaluated using [[Distortion Analysis]].
- [[Remeshing]] includes [[Isotropic Remeshing]] and [[Quad-Dominant Remeshing]].
- [[Isotropic Remeshing]] utilizes [[Voronoi Diagram]] and [[Delaunay Triangulation]].
- [[Isotropic Remeshing]] utilizes [[Centroidal Voronoi Tessellation]].
- [[Quad-Dominant Remeshing]] often relies on [[Mesh Parameterization]].
