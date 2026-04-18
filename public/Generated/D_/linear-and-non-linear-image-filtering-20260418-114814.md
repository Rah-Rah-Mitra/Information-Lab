---
type: content
title: "Linear and Non-Linear Image Filtering"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:48:14.467872600+00:00
summary: "An overview of image filtering techniques, including linear separable filters, non-linear edge-preserving filters, and multi-resolution pyramid decompositions."
tags:
  - computer-vision
  - image-processing
  - signal-processing
  - linear-algebra
entities:
  - "[[Linear Filtering]]"
  - "[[Separable Filter]]"
  - "[[Singular Value Decomposition]]"
  - "[[Box Filter]]"
  - "[[Bilinear Kernel]]"
  - "[[Gaussian Kernel]]"
  - "[[Sobel Operator]]"
  - "[[Laplacian Operator]]"
  - "[[Laplacian of Gaussian]]"
  - "[[Band-Pass Filter]]"
  - "[[Steerable Filter]]"
  - "[[Summed Area Table]]"
  - "[[Recursive Filtering]]"
  - "[[Non-Linear Filtering]]"
  - "[[Median Filter]]"
  - "[[Bilateral Filter]]"
  - "[[Anisotropic Diffusion]]"
  - "[[Morphological Operations]]"
  - "[[Distance Transform]]"
  - "[[Connected Components]]"
  - "[[Fourier Transform]]"
  - "[[Fast Fourier Transform]]"
  - "[[Wiener Filter]]"
  - "[[Discrete Cosine Transform]]"
  - "[[Image Pyramid]]"
  - "[[Gaussian Pyramid]]"
  - "[[Laplacian Pyramid]]"
  - "[[Wavelets]]"
  - "[[Steerable Pyramid]]"
relationships:
  - source: "Linear Filtering"
    target: "Separable Filter"
    description: "can be implemented as a"
  - source: "Separable Filter"
    target: "Singular Value Decomposition"
    description: "can be analyzed using"
  - source: "Linear Filtering"
    target: "Box Filter"
    description: "includes the"
  - source: "Linear Filtering"
    target: "Gaussian Kernel"
    description: "includes the"
  - source: "Linear Filtering"
    target: "Sobel Operator"
    description: "includes the"
  - source: "Linear Filtering"
    target: "Laplacian Operator"
    description: "includes the"
  - source: "Laplacian of Gaussian"
    target: "Band-Pass Filter"
    description: "is a type of"
  - source: "Steerable Filter"
    target: "Band-Pass Filter"
    description: "is a type of"
  - source: "Summed Area Table"
    target: "Box Filter"
    description: "optimizes the computation of"
  - source: "Recursive Filtering"
    target: "Linear Filtering"
    description: "is a method for"
  - source: "Non-Linear Filtering"
    target: "Median Filter"
    description: "includes the"
  - source: "Non-Linear Filtering"
    target: "Bilateral Filter"
    description: "includes the"
  - source: "Bilateral Filter"
    target: "Anisotropic Diffusion"
    description: "is closely related to"
  - source: "Fourier Transform"
    target: "Fast Fourier Transform"
    description: "is efficiently computed by"
  - source: "Image Pyramid"
    target: "Gaussian Pyramid"
    description: "includes the"
  - source: "Image Pyramid"
    target: "Laplacian Pyramid"
    description: "includes the"
  - source: "Wavelets"
    target: "Steerable Pyramid"
    description: "is a related multi-resolution approach to"
---

# Linear and Non-Linear Image Filtering

*An overview of image filtering techniques, including linear separable filters, non-linear edge-preserving filters, and multi-resolution pyramid decompositions.*

Image filtering involves the application of a neighborhood operator to an image to achieve effects such as blurring, sharpening, or edge detection. 

## Linear Filtering

[[Linear Filtering]] is the process of computing each output pixel as a weighted summation of input pixels. A [[Separable Filter]] is a 2D kernel that can be decomposed into the outer product of two 1D kernels, significantly reducing computational cost. The separability of a kernel can be determined using [[Singular Value Decomposition]].

Common linear filters include:
- [[Box Filter]]: A simple moving average that averages pixels in a window.
- [[Bilinear Kernel]]: A piecewise linear "tent" function.
- [[Gaussian Kernel]]: A smooth low-pass filter often used for noise reduction.
- [[Sobel Operator]]: A separable combination of a central difference and a box filter used for edge extraction.
- [[Laplacian Operator]]: A second-order derivative operator used for corner detection.
- [[Laplacian of Gaussian]]: A [[Band-Pass Filter]] that smooths an image with a Gaussian and then takes the Laplacian.

To optimize the computation of box filters, a [[Summed Area Table]] (or integral image) can be precomputed. Additionally, [[Recursive Filtering]] (Infinite Impulse Response) can be used to implement large-extent smoothing kernels efficiently.

## Non-Linear Filtering

[[Non-Linear Filtering]] is used when linear methods fail, such as removing shot noise or preserving edges. 
- [[Median Filter]]: Selects the median value from a neighborhood, effectively removing outlier noise while preserving edges better than Gaussian blurring.
- [[Bilateral Filter]]: An edge-preserving filter that weights pixels based on both spatial distance (domain kernel) and intensity similarity (range kernel).
- [[Anisotropic Diffusion]]: An iterative smoothing process where the diffusion coefficient is a function of the image gradient, preventing smoothing across edges.

## Binary Image Operations

Beyond grayscale filtering, [[Morphological Operations]] (such as dilation and erosion) are used to process binary images. The [[Distance Transform]] computes the distance from each pixel to the nearest background pixel, often used in skeletonization. [[Connected Components]] are regions of adjacent pixels with the same label, used for object segmentation.

## Frequency Domain Analysis

The [[Fourier Transform]] allows for the analysis of an image's frequency content. The [[Fast Fourier Transform]] provides an efficient algorithm for this computation. 
- [[Wiener Filter]]: An optimum restoration filter based on the power spectrum of the signal and noise.
- [[Discrete Cosine Transform]]: A variant of the Fourier transform used extensively in image and video compression (e.g., JPEG).

## Multi-Resolution Representations

An [[Image Pyramid]] is a hierarchy of images at different resolutions. 
- [[Gaussian Pyramid]]: Created by repeatedly blurring and downsampling.
- [[Laplacian Pyramid]]: Stores the difference between a level and the reconstructed version of the level below it, allowing for perfect reconstruction and applications like image blending.
- [[Wavelets]]: Filters that localize a signal in both space and frequency. Unlike pyramids, they can provide a tight frame (non-overcomplete representation).
- [[Steerable Pyramid]]: An overcomplete, orientationally selective representation that avoids aliasing.

## Relationships

- Linear Filtering can be implemented as a Separable Filter
- Separable Filter can be analyzed using Singular Value Decomposition
- Linear Filtering includes the Box Filter
- Linear Filtering includes the Gaussian Kernel
- Linear Filtering includes the Sobel Operator
- Linear Filtering includes the Laplacian Operator
- Laplacian of Gaussian is a type of Band-Pass Filter
- Steerable Filter is a type of Band-Pass Filter
- Summed Area Table optimizes the computation of Box Filter
- Recursive Filtering is a method for Linear Filtering
- Non-Linear Filtering includes the Median Filter
- Non-Linear Filtering includes the Bilateral Filter
- Bilateral Filter is closely related to Anisotropic Diffusion
- Fourier Transform is efficiently computed by Fast Fourier Transform
- Image Pyramid includes the Gaussian Pyramid
- Image Pyramid includes the Laplacian Pyramid
- Wavelets is a related multi-resolution approach to Steerable Pyramid
