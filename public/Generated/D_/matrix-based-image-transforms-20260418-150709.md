---
type: content
title: "Matrix-Based Image Transforms"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:07:09.962315800+00:00
summary: "An exploration of linear image transforms using matrix operations, covering orthogonal, unitary, and biorthonormal bases for 1-D and 2-D signals."
tags:
  - computer-vision
  - linear-algebra
  - image-processing
  - signal-processing
entities:
  - "[[Matrix-Based Transform]]"
  - "[[Orthogonal Transform]]"
  - "[[Unitary Transform]]"
  - "[[Biorthonormal Basis]]"
  - "[[Transformation Matrix]]"
  - "[[Inner Product Space]]"
  - "[[Basis Image]]"
  - "[[Heisenberg Box]]"
  - "[[Heisenberg-Gabor Inequality]]"
  - "[[Discrete Fourier Transform]]"
  - "[[Discrete Hartley Transform]]"
  - "[[Mother Wavelet]]"
relationships:
  - source: "Matrix-Based Transform"
    target: "Transformation Matrix"
    description: "is implemented using a"
  - source: "Orthogonal Transform"
    target: "Matrix-Based Transform"
    description: "is a type of"
  - source: "Unitary Transform"
    target: "Matrix-Based Transform"
    description: "is a type of"
  - source: "Orthogonal Transform"
    target: "Inner Product Space"
    description: "preserves inner products within an"
  - source: "Unitary Transform"
    target: "Inner Product Space"
    description: "preserves inner products within an"
  - source: "Biorthonormal Basis"
    target: "Matrix-Based Transform"
    description: "can be used to define a"
  - source: "Matrix-Based Transform"
    target: "Basis Image"
    description: "can be visualized via"
  - source: "Discrete Fourier Transform"
    target: "Matrix-Based Transform"
    description: "is an example of a"
  - source: "Discrete Hartley Transform"
    target: "Matrix-Based Transform"
    description: "is an example of a"
  - source: "Mother Wavelet"
    target: "Heisenberg Box"
    description: "generates basis functions localized in a"
  - source: "Heisenberg Box"
    target: "Heisenberg-Gabor Inequality"
    description: "has a minimum area defined by the"
---

# Matrix-Based Image Transforms

*An exploration of linear image transforms using matrix operations, covering orthogonal, unitary, and biorthonormal bases for 1-D and 2-D signals.*

A [[Matrix-Based Transform]] is a linear expansion that represents a function or image as a weighted sum of basis vectors, computed efficiently using matrix multiplication.

## Transform Types

### Orthogonal and Unitary Transforms
An [[Orthogonal Transform]] uses real orthonormal basis vectors, meaning the [[Transformation Matrix]] is orthogonal (its transpose is its inverse). These transforms preserve distances and angles between vectors. A [[Unitary Transform]] is the generalization of an orthogonal transform for complex-valued basis vectors, where the inverse of the transformation matrix is its conjugate transpose.

### Biorthonormal Bases
In a [[Biorthonormal Basis]], the expansion functions and their duals are not necessarily orthonormal themselves, but they satisfy a biorthogonality relationship. Unlike orthogonal transforms, biorthonormal transforms do not necessarily preserve inner products.

## 2-D Transforms and Basis Images
For 2-D images, a separable transform can be computed by pre- and post-multiplying the image matrix by the transformation matrix. The resulting inverse transformation kernels are known as [[Basis Image]]s, which provide a visual representation of the 2-D basis functions.

## Time-Frequency Localization

### The Heisenberg Box
Basis functions can be analyzed in the time-frequency plane. The energy of a basis function is concentrated in a rectangular region called a [[Heisenberg Box]] (or cell). The area of this box is constrained by the [[Heisenberg-Gabor Inequality]], which states that a function cannot be perfectly localized in both time and frequency simultaneously.

### Wavelets
A [[Mother Wavelet]] is a real, square-integrable function used to generate a basis of scaled and shifted versions of itself. Wavelet-based transforms provide both frequency and temporal information, effectively acting as a "musical score" for the signal.

## Examples
- [[Discrete Fourier Transform]]: A complex-valued transform using orthonormal complex exponentials.
- [[Discrete Hartley Transform]]: A real-valued transform that avoids complex numbers while maintaining similar frequency resolution to the Fourier transform.

## Relationships
- Matrix-Based Transform is implemented using a Transformation Matrix
- Orthogonal Transform is a type of Matrix-Based Transform
- Unitary Transform is a type of Matrix-Based Transform
- Orthogonal Transform preserves inner products within an Inner Product Space
- Unitary Transform preserves inner products within an Inner Product Space
- Biorthonormal Basis can be used to define a Matrix-Based Transform
- Matrix-Based Transform can be visualized via Basis Image
- Discrete Fourier Transform is an example of a Matrix-Based Transform
- Discrete Hartley Transform is an example of a Matrix-Based Transform
- Mother Wavelet generates basis functions localized in a Heisenberg Box
- Heisenberg Box has a minimum area defined by the Heisenberg-Gabor Inequality
