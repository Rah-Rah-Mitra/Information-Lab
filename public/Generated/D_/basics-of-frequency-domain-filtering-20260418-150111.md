---
type: content
title: "Basics of Frequency Domain Filtering"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:01:11.383072500+00:00
summary: "An overview of the process, tools, and types of filters used to perform image smoothing and sharpening in the frequency domain using the DFT."
tags:
  - computer-vision
  - digital-image-processing
  - frequency-domain
  - fourier-transform
entities:
  - "[[Discrete Fourier Transform]]"
  - "[[Filter Transfer Function]]"
  - "[[Zero-Phase-Shift Filter]]"
  - "[[Ideal Lowpass Filter]]"
  - "[[Gaussian Lowpass Filter]]"
  - "[[Butterworth Lowpass Filter]]"
  - "[[Ideal Highpass Filter]]"
  - "[[Gaussian Highpass Filter]]"
  - "[[Butterworth Highpass Filter]]"
  - "[[Homomorphic Filtering]]"
  - "[[Notch Filter]]"
  - "[[Fast Fourier Transform]]"
relationships:
  - source: "Discrete Fourier Transform"
    target: "Filter Transfer Function"
    description: "is used to compute the image spectrum for multiplication by a"
  - source: "Zero-Phase-Shift Filter"
    target: "Filter Transfer Function"
    description: "is a type of"
  - source: "Ideal Lowpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Gaussian Lowpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Butterworth Lowpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Ideal Highpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Gaussian Highpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Butterworth Highpass Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Homomorphic Filtering"
    target: "Zero-Phase-Shift Filter"
    description: "generalizes the concept of"
  - source: "Notch Filter"
    target: "Zero-Phase-Shift Filter"
    description: "is a type of"
  - source: "Fast Fourier Transform"
    target: "Discrete Fourier Transform"
    description: "is an efficient algorithm to compute the"
---

# Basics of Frequency Domain Filtering

*An overview of the process, tools, and types of filters used to perform image smoothing and sharpening in the frequency domain using the DFT.*

Filtering in the frequency domain involves the elementwise multiplication of an image's [[Discrete Fourier Transform]] (DFT) by a [[Filter Transfer Function]]. This process allows for the manipulation of specific frequency components to achieve effects like smoothing or sharpening.

## Filtering Process

The standard procedure for frequency domain filtering involves several steps:
1. Padding the input image to avoid wraparound error.
2. Centering the Fourier transform by multiplying the image by $(-1)^{x+y}$.
3. Computing the [[Discrete Fourier Transform]].
4. Constructing a real, symmetric [[Filter Transfer Function]] of the same size as the padded image.
5. Performing elementwise multiplication between the DFT and the filter.
6. Computing the Inverse DFT (IDFT) and extracting the original image region.

## Zero-Phase-Shift Filters

Filters that affect the real and imaginary parts of the DFT equally, thereby having no effect on the phase angle, are known as [[Zero-Phase-Shift Filter]]s. These are the primary filters used in image processing to avoid undesirable spatial distortions.

## Lowpass and Highpass Filtering

### Lowpass Filters
Lowpass filters attenuate high-frequency components to achieve image smoothing (blurring). Common types include:
- [[Ideal Lowpass Filter]]: Passes all frequencies within a radius $D_0$ and cuts off all others. It often causes ringing artifacts in the spatial domain.
- [[Gaussian Lowpass Filter]]: Provides a smooth transition and is guaranteed to produce no ringing.
- [[Butterworth Lowpass Filter]]: A compromise between the ideal and Gaussian filters, where the filter order $n$ controls the sharpness of the transition.

### Highpass Filters
Highpass filters attenuate low-frequency components to enhance edges and fine details (sharpening). They can be derived by subtracting a lowpass filter from a constant (1). Common types include:
- [[Ideal Highpass Filter]]
- [[Gaussian Highpass Filter]]
- [[Butterworth Highpass Filter]]

## Specialized Filtering Techniques

### Homomorphic Filtering
[[Homomorphic Filtering]] is used for simultaneous dynamic range compression and contrast enhancement. It operates on the logarithm of the image to separate the illumination (low frequency) and reflectance (high frequency) components.

### Notch Filtering
[[Notch Filter]]s are used to reject or pass frequencies in a predefined neighborhood of the frequency rectangle, making them ideal for removing periodic interference or moiré patterns.

## Computational Efficiency

Because the brute-force computation of the DFT is computationally expensive, the [[Fast Fourier Transform]] (FFT) is used to reduce the complexity from $O(N^2)$ to $O(N \log N)$, making frequency domain filtering practical for large images.

## Relationships
- [[Discrete Fourier Transform]] is used to compute the image spectrum for multiplication by a [[Filter Transfer Function]].
- [[Zero-Phase-Shift Filter]] is a type of [[Filter Transfer Function]].
- [[Ideal Lowpass Filter]], [[Gaussian Lowpass Filter]], and [[Butterworth Lowpass Filter]] are types of [[Zero-Phase-Shift Filter]].
- [[Ideal Highpass Filter]], [[Gaussian Highpass Filter]], and [[Butterworth Highpass Filter]] are types of [[Zero-Phase-Shift Filter]].
- [[Homomorphic Filtering]] generalizes the concept of [[Zero-Phase-Shift Filter]].
- [[Notch Filter]] is a type of [[Zero-Phase-Shift Filter]].
- [[Fast Fourier Transform]] is an efficient algorithm to compute the [[Discrete Fourier Transform]].
