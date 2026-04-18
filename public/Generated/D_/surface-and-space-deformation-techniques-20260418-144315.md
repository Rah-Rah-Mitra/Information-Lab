---
type: content
title: "Surface and Space Deformation Techniques"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:43:15.383180+00:00
summary: "An overview of linear and nonlinear methods for deforming 3D surfaces and ambient space, including shell-based, differential, and freeform deformation."
tags:
  - computer-vision
  - geometry-processing
  - linear-algebra
  - surface-deformation
entities:
  - "[[Surface Deformation]]"
  - "[[Shell-Based Deformation]]"
  - "[[Multi-Scale Deformation]]"
  - "[[Differential Coordinates]]"
  - "[[Gradient-Based Deformation]]"
  - "[[Laplacian-Based Deformation]]"
  - "[[Freeform Deformation]]"
  - "[[Lattice-Based Freeform Deformation]]"
  - "[[Cage-Based Freeform Deformation]]"
  - "[[Radial Basis Functions]]"
  - "[[Cholesky Factorization]]"
  - "[[Laplace Matrix]]"
relationships:
  - source: "Surface Deformation"
    target: "Shell-Based Deformation"
    description: "includes"
  - source: "Surface Deformation"
    target: "Differential Coordinates"
    description: "includes"
  - source: "Shell-Based Deformation"
    target: "Multi-Scale Deformation"
    description: "is enhanced by"
  - source: "Differential Coordinates"
    target: "Gradient-Based Deformation"
    description: "includes"
  - source: "Differential Coordinates"
    target: "Laplacian-Based Deformation"
    description: "includes"
  - source: "Surface Deformation"
    target: "Freeform Deformation"
    description: "contrasts with"
  - source: "Freeform Deformation"
    target: "Lattice-Based Freeform Deformation"
    description: "includes"
  - source: "Freeform Deformation"
    target: "Cage-Based Freeform Deformation"
    description: "includes"
  - source: "Freeform Deformation"
    target: "Radial Basis Functions"
    description: "includes"
  - source: "Shell-Based Deformation"
    target: "Laplace Matrix"
    description: "solves using"
  - source: "Laplacian-Based Deformation"
    target: "Laplace Matrix"
    description: "solves using"
  - source: "Freeform Deformation"
    target: "Cholesky Factorization"
    description: "may utilize for linear systems"
---

# Surface and Space Deformation Techniques

*An overview of linear and nonlinear methods for deforming 3D surfaces and ambient space, including shell-based, differential, and freeform deformation.*

[[Surface Deformation]] refers to the process of modifying the geometry of a 3D object while maintaining certain fairness or physical properties. These techniques are generally divided into surface-based and space-based approaches.

## Surface-Based Deformation

Surface-based methods compute a deformation field directly on the surface by minimizing quadratic energy functionals.

### Shell-Based Deformation
[[Shell-Based Deformation]] models the surface as a physical skin that resists stretching and bending. This is achieved by minimizing an elastic thin-shell energy functional. To maintain interactive rates, the nonlinear energy is often linearized, which can lead to artifacts during large rotations.

### Multi-Scale Deformation
To preserve fine-scale details during deformation, [[Multi-Scale Deformation]] decomposes the surface into low-frequency global shapes and high-frequency details. The global shape is deformed, and details are reconstructed using displacement vectors or normal displacements.

### Differential Coordinates
Methods using [[Differential Coordinates]] modify surface properties rather than spatial coordinates. 
- [[Gradient-Based Deformation]] manipulates face gradients and solves a Poisson system to reconstruct the surface.
- [[Laplacian-Based Deformation]] manipulates per-vertex Laplacians, resulting in a bi-Laplacian system. This method is generally more robust for rotations than shell-based approaches.

## Space-Based Deformation

Unlike surface-based methods, [[Freeform Deformation]] warps the ambient space containing the object, making it independent of the surface representation.

### Lattice-Based and Cage-Based FFD
- [[Lattice-Based Freeform Deformation]] uses a trivariate tensor-product spline function defined over a regular control grid.
- [[Cage-Based Freeform Deformation]] generalizes this by using an arbitrary coarse triangle mesh (a cage) and generalized barycentric coordinates to define the deformation.

### Radial Basis Functions
[[Radial Basis Functions]] (RBFs) provide a way to interpolate scattered displacement constraints by superposing radially symmetric kernels, ensuring a smooth and fair interpolation of the space.

## Numerical Implementation

Many of these methods require solving large, sparse linear systems. The [[Laplace Matrix]] is central to many surface-based techniques. For symmetric positive definite systems, [[Cholesky Factorization]] is often used as a direct solver, with reordering heuristics like the Reverse Cuthill-McKee algorithm used to minimize fill-in and maintain sparsity.

## Relationships
- Surface Deformation includes Shell-Based Deformation
- Surface Deformation includes Differential Coordinates
- Shell-Based Deformation is enhanced by Multi-Scale Deformation
- Differential Coordinates includes Gradient-Based Deformation
- Differential Coordinates includes Laplacian-Based Deformation
- Surface Deformation contrasts with Freeform Deformation
- Freeform Deformation includes Lattice-Based Freeform Deformation
- Freeform Deformation includes Cage-Based Freeform Deformation
- Freeform Deformation includes Radial Basis Functions
- Shell-Based Deformation solves using Laplace Matrix
- Laplacian-Based Deformation solves using Laplace Matrix
- Freeform Deformation may utilize for linear systems Cholesky Factorization
