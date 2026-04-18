---
type: content
title: "Intensity Transformations and Spatial Filtering"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:58:13.581643400+00:00
summary: "An overview of intensity transformation functions, histogram processing techniques, and the mechanics of linear spatial filtering for image enhancement."
tags:
  - computer-vision
  - image-processing
  - digital-signal-processing
  - linear-algebra
entities:
  - "[[Log Transformation]]"
  - "[[Power-Law Transformation]]"
  - "[[Gamma Correction]]"
  - "[[Piecewise Linear Transformation]]"
  - "[[Contrast Stretching]]"
  - "[[Intensity-Level Slicing]]"
  - "[[Bit-Plane Slicing]]"
  - "[[Image Histogram]]"
  - "[[Histogram Equalization]]"
  - "[[Histogram Matching]]"
  - "[[Linear Spatial Filter]]"
  - "[[Spatial Correlation]]"
  - "[[Spatial Convolution]]"
  - "[[Separable Filter Kernel]]"
relationships:
  - source: "Log Transformation"
    target: "Image Histogram"
    description: "compresses the dynamic range of"
  - source: "Power-Law Transformation"
    target: "Gamma Correction"
    description: "is used for"
  - source: "Piecewise Linear Transformation"
    target: "Contrast Stretching"
    description: "generalizes"
  - source: "Piecewise Linear Transformation"
    target: "Intensity-Level Slicing"
    description: "generalizes"
  - source: "Image Histogram"
    target: "Histogram Equalization"
    description: "is the basis for"
  - source: "Image Histogram"
    target: "Histogram Matching"
    description: "is the basis for"
  - source: "Histogram Equalization"
    target: "Histogram Matching"
    description: "is an intermediate step in"
  - source: "Linear Spatial Filter"
    target: "Spatial Correlation"
    description: "implements"
  - source: "Linear Spatial Filter"
    target: "Spatial Convolution"
    description: "implements"
  - source: "Spatial Correlation"
    target: "Spatial Convolution"
    description: "differs from by kernel rotation"
  - source: "Separable Filter Kernel"
    target: "Linear Spatial Filter"
    description: "optimizes the computation of"
---

# Intensity Transformations and Spatial Filtering

*An overview of intensity transformation functions, histogram processing techniques, and the mechanics of linear spatial filtering for image enhancement.*

Intensity transformations and spatial filtering are fundamental techniques used to enhance the visual quality of images by modifying pixel intensity values.

## Intensity Transformation Functions

Intensity transformations map input pixels to output pixels based on a specific function. Common types include:

- [[Log Transformation]]: Used to expand the values of dark pixels while compressing higher-level values, effectively compressing the dynamic range. This is particularly useful for displaying the Fourier spectrum.
- [[Power-Law Transformation]]: Defined by the form $s = c r^\gamma$. When $\gamma < 1$, it maps a narrow range of dark input values into a wider range of output values. This is the basis for [[Gamma Correction]], which corrects the power-law response of display devices like CRTs.
- [[Piecewise Linear Transformation]]: These functions can be arbitrarily complex and are used for specific enhancements such as [[Contrast Stretching]] (expanding the intensity range to span the full scale) and [[Intensity-Level Slicing]] (highlighting a specific range of intensities).
- [[Bit-Plane Slicing]]: An analysis technique that decomposes an image into its constituent binary planes to evaluate the relative importance of each bit.

## Histogram Processing

An [[Image Histogram]] represents the distribution of intensity levels in an image. Manipulation of this histogram is a key tool for enhancement:

- [[Histogram Equalization]]: An automatic process that seeks to generate an output image with a uniform histogram, thereby increasing global contrast.
- [[Histogram Matching]]: Also known as histogram specification, this method allows a user to specify a desired histogram shape for the output image.

## Spatial Filtering

A [[Linear Spatial Filter]] modifies an image by replacing each pixel value with a function of its neighbors using a filter kernel.

### Correlation and Convolution

- [[Spatial Correlation]]: The process of moving a kernel over an image and computing the sum of products at each location.
- [[Spatial Convolution]]: Identical to correlation, but the kernel is pre-rotated by 180 degrees. Convolution is commutative and associative, making it a foundation of linear system theory.

### Separable Kernels

A [[Separable Filter Kernel]] is a matrix that can be expressed as the outer product of two vectors. Convolving a separable kernel with an image is computationally more efficient because it can be implemented as two successive 1-D convolutions.

## Relationships
- [[Log Transformation]] compresses the dynamic range of [[Image Histogram]].
- [[Power-Law Transformation]] is used for [[Gamma Correction]].
- [[Piecewise Linear Transformation]] generalizes [[Contrast Stretching]] and [[Intensity-Level Slicing]].
- [[Image Histogram]] is the basis for [[Histogram Equalization]] and [[Histogram Matching]].
- [[Histogram Equalization]] is an intermediate step in [[Histogram Matching]].
- [[Linear Spatial Filter]] implements [[Spatial Correlation]] and [[Spatial Convolution]].
- [[Spatial Correlation]] differs from [[Spatial Convolution]] by kernel rotation.
- [[Separable Filter Kernel]] optimizes the computation of [[Linear Spatial Filter]].
