---
type: content
title: "Aliasing and Sampling in Digital Images"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:00:29.384185200+00:00
summary: "Explores the effects of under-sampling in 2-D images, including spatial and temporal aliasing, jaggies, and the role of anti-aliasing filters."
tags:
  - computer-vision
  - digital-image-processing
  - sampling-theory
  - signal-processing
entities:
  - "[[Aliasing]]"
  - "[[Spatial Aliasing]]"
  - "[[Temporal Aliasing]]"
  - "[[Sampling Rate]]"
  - "[[Sampling Theorem]]"
  - "[[Jaggies]]"
  - "[[Anti-aliasing Filtering]]"
  - "[[Moiré Patterns]]"
  - "[[Lowpass Filter]]"
  - "[[Discrete Fourier Transform]]"
  - "[[Zero Padding]]"
  - "[[Wraparound Error]]"
relationships:
  - source: "Aliasing"
    target: "Sampling Rate"
    description: "is caused by an insufficient"
  - source: "Spatial Aliasing"
    target: "Aliasing"
    description: "is a manifestation of"
  - source: "Temporal Aliasing"
    target: "Aliasing"
    description: "is a manifestation of"
  - source: "Sampling Theorem"
    target: "Aliasing"
    description: "defines the limits to avoid"
  - source: "Jaggies"
    target: "Spatial Aliasing"
    description: "is a common visual artifact of"
  - source: "Anti-aliasing Filtering"
    target: "Aliasing"
    description: "reduces the effects of"
  - source: "Anti-aliasing Filtering"
    target: "Lowpass Filter"
    description: "is typically implemented as a"
  - source: "Moiré Patterns"
    target: "Aliasing"
    description: "are a general form of"
  - source: "Discrete Fourier Transform"
    target: "Wraparound Error"
    description: "introduces periodicity that can cause"
  - source: "Zero Padding"
    target: "Wraparound Error"
    description: "is used to mitigate"
---

# Aliasing and Sampling in Digital Images

*Explores the effects of under-sampling in 2-D images, including spatial and temporal aliasing, jaggies, and the role of anti-aliasing filters.*

Aliasing occurs when a continuous function is sampled at a rate insufficient to capture its highest frequency components, leading to the introduction of spurious low-frequency artifacts.

## Spatial and Temporal Aliasing

In digital images, [[Aliasing]] manifests in two primary forms: [[Spatial Aliasing]] and [[Temporal Aliasing]]. Spatial aliasing is caused by under-sampling the spatial frequencies of an image and is most visible in repetitive patterns. Temporal aliasing occurs in sequences of dynamic images (such as movies) when the frame rate is too low relative to the speed of motion, often resulting in the "wagon wheel" effect.

## Visual Artifacts

Under-sampling leads to several distinct visual artifacts:
- **Jaggies**: Edge distortion appearing as jagged lines, common in images with strong line or edge content.
- **Moiré Patterns**: Secondary visual phenomena produced by superimposing two gratings of approximately equal spacing or when sampling periodic components whose spacing is comparable to the [[Sampling Rate]].

## Mitigation and the Sampling Theorem

According to the [[Sampling Theorem]], no information is lost if a band-limited function is sampled at rates greater than twice the highest frequency content. To prevent aliasing, [[Anti-aliasing Filtering]] must be performed at the "front-end" before sampling. This is typically achieved using a [[Lowpass Filter]] to attenuate high frequencies (e.g., by slightly defocusing the lens).

## Computational Considerations

When using the [[Discrete Fourier Transform]] (DFT) for filtering, the implied periodicity of the DFT can lead to [[Wraparound Error]], where the edges of the image interfere with each other. This is corrected by [[Zero Padding]], which involves appending zeros to the image arrays to ensure the periodic repetitions do not overlap during convolution.
