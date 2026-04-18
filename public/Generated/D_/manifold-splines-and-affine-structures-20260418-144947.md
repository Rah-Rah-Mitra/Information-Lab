---
type: content
title: "Manifold Splines and Affine Structures"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:49:47.487997800+00:00
summary: "An exploration of manifold splines for modeling complex surfaces, utilizing affine structures computed via holomorphic 1-forms, discrete Ricci flow, and polycube maps."
tags:
  - computer-vision
  - differential-geometry
  - geometric-modeling
  - splines
entities:
  - "[[Manifold Spline]]"
  - "[[Affine Structure]]"
  - "[[Affine Atlas]]"
  - "[[Triangular B-Spline]]"
  - "[[Discrete Ricci Flow]]"
  - "[[Holomorphic 1-form]]"
  - "[[Polycube Map]]"
  - "[[Conformal Module]]"
  - "[[Teichmüller Space]]"
  - "[[Euler Number]]"
  - "[[Singular Point]]"
relationships:
  - source: "Manifold Spline"
    target: "Affine Structure"
    description: "requires for construction"
  - source: "Affine Structure"
    target: "Affine Atlas"
    description: "is defined by"
  - source: "Manifold Spline"
    target: "Triangular B-Spline"
    description: "can be implemented as"
  - source: "Affine Atlas"
    target: "Discrete Ricci Flow"
    description: "can be computed using"
  - source: "Affine Atlas"
    target: "Holomorphic 1-form"
    description: "can be computed using"
  - source: "Affine Atlas"
    target: "Polycube Map"
    description: "can be computed using"
  - source: "Discrete Ricci Flow"
    target: "Euler Number"
    description: "uses to determine target curvatures"
  - source: "Affine Atlas"
    target: "Singular Point"
    description: "contains at locations of topological obstruction"
  - source: "Conformal Module"
    target: "Teichmüller Space"
    description: "acts as coordinates for"
---

# Manifold Splines and Affine Structures

*An exploration of manifold splines for modeling complex surfaces, utilizing affine structures computed via holomorphic 1-forms, discrete Ricci flow, and polycube maps.*

A [[Manifold Spline]] is a geometric construction that glues local spline patches across a domain manifold such that the surface evaluation is independent of the choice of local charts.

## Theoretical Foundation

To ensure that the evaluation of a spline is independent of the chart selection, the transition functions between overlapping charts must be affine. This requirement means that the domain manifold must possess an [[Affine Structure]], which is formally described by an [[Affine Atlas]].

### Topological Obstructions

According to characteristic class theory, general closed 2-manifolds do not naturally possess an affine atlas. The existence of such a structure is limited by the [[Euler Number]] of the surface. For closed surfaces, the only one admitting a global affine atlas is of genus one. To construct manifold splines on other surfaces, a finite number of [[Singular Point]]s must be removed from the manifold to convert it into an affine manifold.

## Computing Affine Structures

Several algorithms are used to compute the affine structure of an oriented 2-manifold:

- **[[Holomorphic 1-form]]**: Induces an affine structure with a fixed number of extraordinary points based on the genus and number of boundaries.
- **[[Discrete Ricci Flow]]**: A powerful tool for computing the affine atlas by deforming the surface based on target curvatures. It allows users to specify the number and location of singularities and can balance area and angle distortion.
- **[[Polycube Map]]**: Minimizes both angular and area distortion by mapping the original model to a polycube, where the corners of the polycube become the singularities.

## Manifold Triangular B-Splines

[[Triangular B-Spline]]s are generalized to manifolds by utilizing the parametric affine invariance of the planar scheme. The construction involves:
1. Defining the domain manifold.
2. Computing the affine structure.
3. Constructing planar spline patches on each chart, which are then automatically glued via affine transition functions.
4. Modifying control points through a least-squares fairing process to eliminate "knot-line" curvature concentrations.

## Relation to Conformal Geometry

In the broader context of shape analysis, the [[Conformal Module]] (the centers and radii of inner boundary circles in a circle domain) provides a unique representation of multiply-connected domains. These modules can be treated as coordinates within the [[Teichmüller Space]], allowing for the classification and indexing of surfaces with the same topology.
