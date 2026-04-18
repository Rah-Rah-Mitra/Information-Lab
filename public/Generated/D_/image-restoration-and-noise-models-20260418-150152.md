---
type: content
title: "Image Restoration and Noise Models"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:01:52.045850800+00:00
summary: "An overview of image restoration techniques, focusing on additive noise models and spatial and frequency domain filtering for image denoising."
tags:
  - computer-vision
  - image-processing
  - noise-reduction
  - spatial-filtering
  - frequency-domain
entities:
  - "[[Image Restoration]]"
  - "[[Image Degradation]]"
  - "[[Additive Noise]]"
  - "[[Gaussian Noise]]"
  - "[[Rayleigh Noise]]"
  - "[[Erlang Noise]]"
  - "[[Exponential Noise]]"
  - "[[Uniform Noise]]"
  - "[[Salt-and-Pepper Noise]]"
  - "[[Periodic Noise]]"
  - "[[Arithmetic Mean Filter]]"
  - "[[Geometric Mean Filter]]"
  - "[[Harmonic Mean Filter]]"
  - "[[Contraharmonic Mean Filter]]"
  - "[[Median Filter]]"
  - "[[Max Filter]]"
  - "[[Min Filter]]"
  - "[[Midpoint Filter]]"
  - "[[Alpha-Trimmed Mean Filter]]"
  - "[[Adaptive Local Noise Reduction Filter]]"
  - "[[Adaptive Median Filter]]"
  - "[[Notch Filter]]"
relationships:
  - source: "Image Restoration"
    target: "Image Degradation"
    description: "attempts to reverse"
  - source: "Image Restoration"
    target: "Additive Noise"
    description: "addresses"
  - source: "Additive Noise"
    target: "Gaussian Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Rayleigh Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Erlang Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Exponential Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Uniform Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Salt-and-Pepper Noise"
    description: "includes"
  - source: "Additive Noise"
    target: "Periodic Noise"
    description: "includes"
  - source: "Image Restoration"
    target: "Arithmetic Mean Filter"
    description: "uses"
  - source: "Arithmetic Mean Filter"
    target: "Additive Noise"
    description: "reduces"
  - source: "Median Filter"
    target: "Salt-and-Pepper Noise"
    description: "is effective against"
  - source: "Adaptive Median Filter"
    target: "Salt-and-Pepper Noise"
    description: "handles high densities of"
  - source: "Notch Filter"
    target: "Periodic Noise"
    description: "removes"
---

# Image Restoration and Noise Models

*An overview of image restoration techniques, focusing on additive noise models and spatial and frequency domain filtering for image denoising.*

[[Image Restoration]] is an objective process that attempts to recover an original image that has been degraded by using a priori knowledge of the [[Image Degradation]] phenomenon. Unlike image enhancement, which is subjective, restoration focuses on modeling the degradation and applying the inverse process.

## Noise Models

[[Additive Noise]] is a primary component of image degradation, often modeled as a random variable characterized by a probability density function (PDF). Common types include:

- [[Gaussian Noise]]: Arises from electronic circuit noise and sensor noise due to poor illumination or high temperature.
- [[Rayleigh Noise]]: Useful for modeling skewed histograms, often found in range imaging.
- [[Erlang Noise]]: Also known as Gamma noise, applied in laser imaging.
- [[Exponential Noise]]: A special case of Erlang noise with a parameter of 1.
- [[Uniform Noise]]: Often used as a basis for random number generators in simulations.
- [[Salt-and-Pepper Noise]]: Also known as impulse noise, characterized by sudden spikes of maximum or minimum intensity.
- [[Periodic Noise]]: Arises from electrical or electromechanical interference, appearing as concentrated bursts of energy in the Fourier transform.

## Spatial Filtering for Restoration

Spatial filters are used to estimate the original image when only additive random noise is present. These include mean filters and order-statistic filters:

### Mean Filters
- [[Arithmetic Mean Filter]]: A simple box filter that smooths local variations by averaging pixels.
- [[Geometric Mean Filter]]: Tends to lose less image detail than the arithmetic mean filter while providing similar smoothing.
- [[Harmonic Mean Filter]]: Works well for salt noise but fails for pepper noise.
- [[Contraharmonic Mean Filter]]: Well-suited for eliminating salt-and-pepper noise; the sign of the order $Q$ determines whether salt or pepper noise is removed.

### Order-Statistic Filters
- [[Median Filter]]: Replaces a pixel with the median of its neighborhood; highly effective against impulse noise.
- [[Max Filter]]: Useful for finding the brightest points or eroding dark regions; reduces pepper noise.
- [[Min Filter]]: Useful for finding the darkest points or eroding light regions; reduces salt noise.
- [[Midpoint Filter]]: Computes the midpoint between the maximum and minimum values; works best for random noise like Gaussian or uniform noise.
- [[Alpha-Trimmed Mean Filter]]: Deletes the $d$ lowest and highest values before averaging; useful for mixed noise types.

## Adaptive Filtering

Adaptive filters change their behavior based on the local statistical characteristics of the image:

- [[Adaptive Local Noise Reduction Filter]]: Uses local variance and noise variance to preserve edges while smoothing flat areas.
- [[Adaptive Median Filter]]: Increases the neighborhood size dynamically to remove impulse noise while preserving detail and reducing distortion.

## Frequency Domain Restoration

[[Periodic Noise]] is most effectively reduced using a [[Notch Filter]], which isolates and removes specific frequency spikes in the Fourier spectrum without introducing the blurring associated with lowpass filtering.
