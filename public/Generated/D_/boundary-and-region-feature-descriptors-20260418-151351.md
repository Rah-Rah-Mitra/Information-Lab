---
type: content
title: "Boundary and Region Feature Descriptors"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:13:51.765988700+00:00
summary: "An overview of descriptors for characterizing digital boundaries and regions, including shape numbers, Fourier descriptors, and statistical moments."
tags:
  - computer-vision
  - feature-extraction
  - image-processing
  - geometry
entities:
  - "[[Boundary Feature Descriptors]]"
  - "[[Region Feature Descriptors]]"
  - "[[Shape Number]]"
  - "[[Fourier Descriptors]]"
  - "[[Statistical Moments]]"
  - "[[Eccentricity]]"
  - "[[Compactness]]"
  - "[[Circularity]]"
  - "[[Euler Number]]"
  - "[[Co-occurrence Matrix]]"
  - "[[Moment Invariants]]"
  - "[[Principal Component Analysis]]"
  - "[[Harris-Stephens Corner Detector]]"
relationships:
  - source: "Boundary Feature Descriptors"
    target: "Shape Number"
    description: "includes"
  - source: "Boundary Feature Descriptors"
    target: "Fourier Descriptors"
    description: "includes"
  - source: "Boundary Feature Descriptors"
    target: "Statistical Moments"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Eccentricity"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Compactness"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Circularity"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Euler Number"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Co-occurrence Matrix"
    description: "includes"
  - source: "Region Feature Descriptors"
    target: "Moment Invariants"
    description: "includes"
  - source: "Principal Component Analysis"
    target: "Region Feature Descriptors"
    description: "normalizes"
  - source: "Harris-Stephens Corner Detector"
    target: "Boundary Feature Descriptors"
    description: "detects features of"
---

# Boundary and Region Feature Descriptors

*An overview of descriptors for characterizing digital boundaries and regions, including shape numbers, Fourier descriptors, and statistical moments.*

Feature descriptors are used to quantify the properties of digital boundaries and regions to facilitate image pattern classification.

## Boundary Feature Descriptors

[[Boundary Feature Descriptors]] are used to describe the shape and morphology of a curve. Simple descriptors include length, diameter, and the major and minor axes.

### Shape Numbers

A [[Shape Number]] is derived from a Freeman chain-coded boundary and is defined as the first difference of smallest magnitude. It is independent of the starting point and insensitive to rotation in increments of 90 degrees.

### Fourier Descriptors

[[Fourier Descriptors]] represent a boundary as a sequence of complex numbers and apply the Discrete Fourier Transform (DFT). Low-frequency components determine the overall shape, while high-frequency components account for fine detail. These descriptors can be made insensitive to translation, rotation, and scale changes.

### Statistical Moments

[[Statistical Moments]] can be applied to 1-D renditions of boundaries, such as signatures, to measure spread, symmetry, and other shape characteristics.

## Region Feature Descriptors

[[Region Feature Descriptors]] characterize the interior of a segmented area. Basic descriptors include area, perimeter, and the following dimensionless measures:

- [[Compactness]]: The ratio of perimeter squared over area.
- [[Circularity]]: A measure of roundness, closely related to compactness.
- [[Eccentricity]]: The ratio of the distance between foci to the length of the major axis for an approximating ellipse.

### Topological Descriptors

[[Euler Number]] is a topological property defined as the number of connected components minus the number of holes. It remains unaffected by rubber-sheet distortions.

### Texture Descriptors

Texture can be described using statistical approaches (intensity histograms) or spectral approaches (Fourier spectrum). A [[Co-occurrence Matrix]] (or gray-level co-occurrence matrix) captures the relative positions of pixels to describe spatial relationships, from which descriptors like contrast, correlation, and homogeneity are derived.

### Moment Invariants

[[Moment Invariants]] are a set of 2-D moments that are invariant to translation, scale change, mirroring, and rotation.

## Normalization and Detection

[[Principal Component Analysis]] (PCA) is used to normalize regions or boundaries for variations in size, translation, and rotation by aligning the data with the eigenvectors of the covariance matrix.

The [[Harris-Stephens Corner Detector]] identifies corners by analyzing the eigenvalues of a matrix formed from image derivatives in a local patch. Two large eigenvalues indicate a corner, while one large and one small eigenvalue indicate an edge.

## Relationships

- Boundary Feature Descriptors includes Shape Number
- Boundary Feature Descriptors includes Fourier Descriptors
- Boundary Feature Descriptors includes Statistical Moments
- Region Feature Descriptors includes Eccentricity
- Region Feature Descriptors includes Compactness
- Region Feature Descriptors includes Circularity
- Region Feature Descriptors includes Euler Number
- Region Feature Descriptors includes Co-occurrence Matrix
- Region Feature Descriptors includes Moment Invariants
- Principal Component Analysis normalizes Region Feature Descriptors
- Harris-Stephens Corner Detector detects features of Boundary Feature Descriptors
