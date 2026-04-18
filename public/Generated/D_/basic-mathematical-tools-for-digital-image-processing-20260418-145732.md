---
type: content
title: "Basic Mathematical Tools for Digital Image Processing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:57:32.817300100+00:00
summary: "An overview of the fundamental arithmetic, set, logical, and spatial operations used to manipulate digital images in the spatial and transform domains."
tags:
  - computer-vision
  - digital-image-processing
  - linear-algebra
  - set-theory
entities:
  - "[[Digital Image Processing]]"
  - "[[Image Addition]]"
  - "[[Image Subtraction]]"
  - "[[Image Multiplication]]"
  - "[[Image Division]]"
  - "[[Set Operations]]"
  - "[[Logical Operations]]"
  - "[[Spatial Operations]]"
  - "[[Intensity Transformation]]"
  - "[[Neighborhood Processing]]"
  - "[[Geometric Transformation]]"
  - "[[Affine Transformation]]"
  - "[[Image Registration]]"
  - "[[Image Transform]]"
  - "[[Fourier Transform]]"
  - "[[Spatial Domain]]"
  - "[[Transform Domain]]"
relationships:
  - source: "Digital Image Processing"
    target: "Image Addition"
    description: "utilizes"
  - source: "Digital Image Processing"
    target: "Image Subtraction"
    description: "utilizes"
  - source: "Digital Image Processing"
    target: "Image Multiplication"
    description: "description: utilizes"
  - source: "Digital Image Processing"
    target: "Image Division"
    description: "utilizes"
  - source: "Digital Image Processing"
    target: "Set Operations"
    description: "utilizes"
  - source: "Digital Image Processing"
    target: "Logical Operations"
    description: "utilizes"
  - source: "Digital Image Processing"
    target: "Spatial Operations"
    description: "utilizes"
  - source: " "
    target: "Intensity Transformation"
    description: "includes"
  - source: "Spatial Operations"
    target: "Neighborhood Processing"
    description: "includes"
  - source: "Spatial Operations"
    target: "Geometric Transformation"
    description: "includes"
  - source: "Geometric Transformation"
    target: "Affine Transformation"
    description: "generalizes"
  - source: "Image Registration"
    target: "Geometric Transformation"
    description: "depends on"
  - source: "Image Transform"
    target: "Fourier Transform"
    description: "example of"
  - source: "Image Transform"
    target: "Transform Domain"
    description: "operates in"
  - source: "Spatial Operations"
    target: "Spatial Domain"
    description: "operates in"
---

# Basic Mathematical Tools for Digital Image Processing

*An overview of the fundamental arithmetic, set, logical, and spatial operations used to manipulate digital images in the spatial and transform domains.*

[[Digital Image Processing]] involves the application of various mathematical tools to manipulate image data. These tools are categorized by whether they operate in the [[Spatial Domain]] or a [[Transform Domain]].

## Arithmetic Operations

Image arithmetic allows for the enhancement of specific features or the reduction of noise.

- **[[Image Addition]]**: Used frequently for noise reduction. By averaging $K$ noisy images of the same scene, the variance of the noise is reduced by a factor of $K$, causing the result to approach the noiseless image.
- **[[Image Subtraction]]**: Used to enhance differences between images. Applications include mask mode radiography and digital subtraction angiography, where a mask image is subtracted from live images to highlight contrast medium propagation.
- **[[Image Multiplication]] and [[Image Division]]**: Primarily used for shading correction (dividing the sensed image by an estimated shading function) and masking (multiplying by a binary region of interest mask).

## Set and Logical Operations

[[Set Operations]] (union, intersection, complement, and difference) are applied to binary images where sets represent regions of foreground pixels. For grayscale images, union and intersection are defined as the maximum and minimum of corresponding pixel pairs, respectively.

[[Logical Operations]] (AND, OR, NOT, XOR) are elementwise operators used on binary images. These operators are functionally complete, meaning they can be used to construct any other logical operator.

## Spatial Operations

[[Spatial Operations]] are performed directly on the pixels of an image and are divided into three categories:

1. **[[Intensity Transformation]]**: Single-pixel operations that alter intensity using a mapping function $s = T(r)$. Examples include contrast stretching, thresholding, and producing image negatives.
2. **[[Neighborhood Processing]]**: Operations where the output pixel value is determined by a specified operation on a neighborhood of pixels in the input image (e.g., local averaging for blurring).
3. **[[Geometric Transformation]]**: Modifying the spatial arrangement of pixels. This includes [[Affine Transformation]] (scaling, rotation, translation, and shearing), which preserves points, straight lines, and planes.

## Image Registration and Transforms

[[Image Registration]] is the process of aligning two or more images of the same scene using tie points (control points) to estimate the necessary geometric transformation.

An [[Image Transform]] moves the image from the spatial domain to the transform domain. A [[Fourier Transform]] is a critical example, moving the image into the frequency domain to allow for the removal of periodic interference via filtering.

## Relationships

- Digital Image Processing utilizes [[Image Addition]], [[Image Subtraction]], [[Image Multiplication]], and [[Image Division]].
- Digital Image Processing utilizes [[Set Operations]] and [[Logical Operations]].
- Digital Image Processing utilizes [[Spatial Operations]].
- Spatial Operations operate in the [[Spatial Domain]].
- Spatial Operations include [[Intensity Transformation]], [[Neighborhood Processing]], and [[Geometric Transformation]].
- Geometric Transformation generalizes [[Affine Transformation]].
- Image Registration depends on [[Geometric Transformation]].
- Image Transform operates in the [[Transform Domain]] and includes the [[Fourier Transform]].
