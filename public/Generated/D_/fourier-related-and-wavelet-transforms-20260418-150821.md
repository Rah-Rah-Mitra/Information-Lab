---
type: content
title: "Fourier-Related and Wavelet Transforms"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:08:21.442328700+00:00
summary: "An overview of discrete transforms including the DCT, DST, WHT, Slant, Haar, and the Discrete Wavelet Transform (DWT) for signal and image processing."
tags:
  - computer-vision
  - signal-processing
  - linear-algebra
  - image-compression
entities:
  - "[[Discrete Fourier Transform]]"
  - "[[Discrete Hartley Transform]]"
  - "[[Discrete Cosine Transform]]"
  - "[[Discrete Sine Transform]]"
  - "[[Walsh-Hadamard Transform]]"
  - "[[Slant Transform]]"
  - "[[Haar Transform]]"
  - "[[Discrete Wavelet Transform]]"
  - "[[Fast Wavelet Transform]]"
  - "[[Multi-Resolution Analysis]]"
  - "[[Scaling Function]]"
  - "[[Mother Wavelet Function]]"
relationships:
  - source: "Discrete Cosine Transform"
    target: "Discrete Fourier Transform"
    description: "is a variant of"
  - source: "Discrete Sine Transform"
    target: "Discrete Fourier Transform"
    description: "is a variant of"
  - source: "Walsh-Hadamard Transform"
    target: "Discrete Fourier Transform"
    description: "is a non-sinusoidal alternative to"
  - source: "Discrete Wavelet Transform"
    target: "Multi-Resolution Analysis"
    description: "is based on"
  - source: "Discrete Wavelet Transform"
    target: "Scaling Function"
    description: "uses"
  - source: "Discrete Wavelet Transform"
    target: "Mother Wavelet Function"
    description: "uses"
  - source: "Fast Wavelet Transform"
    target: "Discrete Wavelet Transform"
    description: "is a computationally efficient implementation of"
  - source: "Haar Transform"
    target: "Discrete Wavelet Transform"
    description: "is the simplest case of"
  - source: "Discrete Cosine Transform"
    target: "Slant Transform"
    description: "shares basis function similarities with"
---

# Fourier-Related and Wavelet Transforms

*An overview of discrete transforms including the DCT, DST, WHT, Slant, Haar, and the Discrete Wavelet Transform (DWT) for signal and image processing.*

Fourier-related and wavelet transforms are mathematical tools used to decompose functions or images into a set of basis functions to analyze frequency, sequency, or multi-resolution characteristics.

## Fourier-Related Transforms

Several transforms are related to the [[Discrete Fourier Transform]] (DFT) but offer different symmetry assumptions or computational advantages:

- **[[Discrete Cosine Transform]] (DCT)**: Assumes 2N-point periodicity and even symmetry, which minimizes boundary discontinuities compared to the DFT. This makes it highly effective for image compression.
- **[[Discrete Sine Transform]] (DST)**: Assumes 2N-point periodicity and odd symmetry. Unlike the DCT, it has no DC component (average value is zero).
- **[[Discrete Hartley Transform]]**: A real-valued transform that avoids complex numbers while maintaining similar spectral properties to the DFT.

## Non-Sinusoidal Transforms

- **[[Walsh-Hadamard Transform]] (WHT)**: Decomposes functions into rectangular basis functions. It uses the concept of "sequency" (the number of sign changes per row) instead of frequency.
- **[[Slant Transform]]**: Specifically designed to represent linearly increasing or decreasing intensity values efficiently, making it useful for images with gradients.
- **[[Haar Transform]]**: The oldest and simplest orthonormal wavelet transform, using rectangular-shaped basis functions.

## Discrete Wavelet Transform (DWT)

The [[Discrete Wavelet Transform]] provides a multi-resolution representation of a signal. It is governed by [[Multi-Resolution Analysis]] (MRA), which defines a nested sequence of function spaces.

### Core Components
- **[[Scaling Function]]**: Also known as the father wavelet, it captures the low-frequency approximation of the signal.
- **[[Mother Wavelet Function]]**: Captures the high-frequency details (differences) between adjacent resolution levels.

### Implementation
- **[[Fast Wavelet Transform]] (FWT)**: Implemented using a filter bank of lowpass (approximation) and highpass (detail) filters followed by downsampling. This reduces computational complexity to $O(N)$.
- **2-D DWT**: Extended to images by applying the FWT to rows and then to the resulting columns, producing approximation, horizontal, vertical, and diagonal detail sub-images.

## Relationships

- [[Discrete Cosine Transform]] is a variant of [[Discrete Fourier Transform]].
- [[Discrete Sine Transform]] is a variant of [[Discrete Fourier Transform]].
- [[Walsh-Hadamard Transform]] is a non-sinusoidal alternative to [[Discrete Fourier Transform]].
- [[Discrete Wavelet Transform]] is based on [[Multi-Resolution Analysis]].
- [[Discrete Wavelet Transform]] uses [[Scaling Function]] and [[Mother Wavelet Function]].
- [[Fast Wavelet Transform]] is a computationally efficient implementation of [[Discrete Wavelet Transform]].
- [[Haar Transform]] is the simplest case of [[Discrete Wavelet Transform]].
