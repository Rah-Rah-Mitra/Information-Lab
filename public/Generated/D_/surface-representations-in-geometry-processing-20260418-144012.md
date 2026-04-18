---
type: content
title: "Surface Representations in Geometry Processing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:40:12.295931300+00:00
summary: "An overview of parametric and implicit surface representations, their mathematical foundations, and their relative strengths in geometry processing."
tags:
  - computer-vision
  - geometry-processing
  - mathematical-graphics
  - surface-modeling
entities:
  - "[[Parametric Surface]]"
  - "[[Implicit Surface]]"
  - "[[Triangle Mesh]]"
  - "[[Spline Surface]]"
  - "[[Subdivision Surface]]"
  - "[[2-Manifold]]"
  - "[[Signed Distance Function]]"
  - "[[Barycentric Parameterization]]"
  - "[[Euler Formula]]"
  - "[[h-refinement]]"
  - "[[p-refinement]]"
relationships:
  - source: "Parametric Surface"
    target: "Triangle Mesh"
    description: "is a piecewise linear instance of"
  - source: "Parametric Surface"
    target: "Spline Surface"
    description: "includes"
  - source: "Parametric Surface"
    target: "Subdivision Surface"
    description: "includes"
  - source: "Implicit Surface"
    target: "Signed Distance Function"
    description: "is often represented by"
  - source: "Triangle Mesh"
    target: "2-Manifold"
    description: "is ideally"
  - source: "Triangle Mesh"
    target: "Barycentric Parameterization"
    description: "uses for local mapping"
  - source: "Triangle Mesh"
    target: "Euler Formula"
    description: "follows the topology of"
  - source: "Triangle Mesh"
    target: "h-refinement"
    description: "improves accuracy via"
  - source: "Triangle Mesh"
    target: "p-refinement"
    description: "improves accuracy via"
---

# Surface Representations in Geometry Processing

*An overview of parametric and implicit surface representations, their mathematical foundations, and their relative strengths in geometry processing.*

[[Surface Representations in Geometry Processing]] are mathematical models used to describe the 2D boundary of 3D solid objects, categorized primarily into parametric and implicit forms.

## Parametric Surfaces

A [[Parametric Surface]] is defined by a vector-valued function that maps a 2D parameter domain to a 3D surface. These representations are highly efficient for surface evaluation and finding geodesic neighborhoods.

### Common Types
- **[[Spline Surface]]**: Standard in CAD systems (e.g., NURBS), these use piecewise polynomial or rational B-spline basis functions. They are often constrained by rectangular topological structures.
- **[[Subdivision Surface]]**: A generalization of splines that can represent arbitrary topology through repeated refinement of a coarse control mesh.
- **[[Triangle Mesh]]**: The most widely used standard in geometry processing. It is a piecewise linear representation where each triangle is defined via [[Barycentric Parameterization]].

### Approximation and Refinement
To improve the accuracy of a surface approximation, two methods are used:
- **[[h-refinement]]**: Reducing the size of individual segments (increasing the number of elements). This is generally preferred in geometry processing.
- **[[p-refinement]]**: Increasing the degree of the polynomial used for the approximation.

## Implicit Surfaces

An [[Implicit Surface]] (or volumetric representation) is defined as the zero level set of a scalar-valued function. This form is particularly robust for spatial queries (inside/outside tests) and Boolean operations used in constructive solid geometry.

### Signed Distance Function
The most natural implicit representation is the [[Signed Distance Function]], which maps every point in space to its distance from the surface, with the sign indicating whether the point is inside or outside the solid.

## Topological Properties

### Manifoldness
A surface is considered a [[2-Manifold]] if it is locally homeomorphic to a disk. In the context of a [[Triangle Mesh]], this means it contains no non-manifold edges (edges shared by more than two faces) or non-manifold vertices.

### Mesh Statistics
For a closed, connected mesh, the [[Euler Formula]] relates the number of vertices (V), edges (E), and faces (F) to the genus (g) of the surface: $V - E + F = 2(1 - g)$. For large meshes with small genus, this implies that the number of triangles is approximately $2V$ and the number of edges is approximately $3V$.

## Relationships
- [[Parametric Surface]] is a piecewise linear instance of [[Triangle Mesh]]
- [[Parametric Surface]] includes [[Spline Surface]]
- [[Parametric Surface]] includes [[Subdivision Surface]]
- [[Implicit Surface]] is often represented by [[Signed Distance Function]]
- [[Triangle Mesh]] is ideally [[2-Manifold]]
- [[Triangle Mesh]] uses [[Barycentric Parameterization]] for local mapping
- [[Triangle Mesh]] follows the topology of [[Euler Formula]]
- [[Triangle Mesh]] improves accuracy via [[h-refinement]]
- [[Triangle Mesh]] improves accuracy via [[p-refinement]]
