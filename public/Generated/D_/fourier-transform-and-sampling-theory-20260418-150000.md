---
type: content
title: "Fourier Transform and Sampling Theory"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:00:00.662885300+00:00
summary: "An exploration of the Fourier transform, the sampling theorem, and the relationship between spatial and frequency domains in signal and image processing."
tags:
  - computer-vision
  - signal-processing
  - mathematics
  - frequency-domain
entities:
  - "[[Fourier Transform]]"
  - "[[Fourier Series]]"
  - "[[Sampling Theorem]]"
  - "[[Nyquist Rate]]"
  - "[[Aliasing]]"
  - "[[Discrete Fourier Transform]]"
  - "[[Convolution Theorem]]"
  - "[[Sinc Function]]"
  - "[[Impulse Train]]"
  - "[[Lowpass Filter]]"
  - "[[Frequency Domain]]"
  - "[[Spatial Domain]]"
relationships:
  - source: "Fourier Transform"
    target: "Frequency Domain"
    description: "maps functions from the spatial domain to the"
  - source: "Fourier Series"
    target: "Fourier Transform"
    description: "is a special case of for periodic functions"
  - source: "Sampling Theorem"
    target: "Nyquist Rate"
    description: "defines the minimum sampling rate as the"
  - source: "Sampling Theorem"
    target: "Aliasing"
    description: "prevents when sampling rate exceeds the Nyquist rate"
  - source: "Discrete Fourier Transform"
    target: "Fourier Transform"
    description: "is the discrete version of the"
  - source: "Convolution Theorem"
    target: "Fourier Transform"
    description: "relates convolution in the spatial domain to multiplication in the frequency domain"
  - source: "Sinc Function"
    target: "Fourier Transform"
    description: "is the transform of a box function"
  - source: "Impulse Train"
    target: "Fourier Transform"
    description: "transforms into another impulse train in the frequency domain"
  - source: "Lowpass Filter"
    target: "Sampling Theorem"
    description: "is used as a reconstruction filter to recover band-limited functions"
  - source: "Frequency Domain"
    target: "Spatial Domain"
    description: "is the dual of the"
---

# Fourier Transform and Sampling Theory

*An exploration of the Fourier transform, the sampling theorem, and the relationship between spatial and frequency domains in signal and image processing.*

The [[Fourier Transform]] is a mathematical tool used to represent a function as a sum of sinusoidal components, effectively mapping a signal from the [[Spatial Domain]] (or time domain) to the [[Frequency Domain]].

## Fourier Series and Transform

For periodic functions, the [[Fourier Series]] expresses the function as a weighted sum of sines and cosines. The [[Fourier Transform]] generalizes this to non-periodic functions, allowing for the complete reconstruction of the original signal via an inverse process without loss of information.

## Sampling and the Sampling Theorem

Sampling is the process of converting a continuous function into a sequence of discrete values. The [[Sampling Theorem]] states that a continuous, band-limited function can be recovered perfectly if it is sampled at a rate exceeding twice its highest frequency content. This minimum required rate is known as the [[Nyquist Rate]].

### Aliasing

When a signal is sampled below the Nyquist rate, [[Aliasing]] occurs. This phenomenon causes different continuous signals to become indistinguishable after sampling, where a high-frequency signal masquerades as a lower-frequency one. To mitigate this, anti-aliasing involves applying a [[Lowpass Filter]] before sampling to attenuate frequencies above the Nyquist limit.

## Discrete Fourier Transform (DFT)

The [[Discrete Fourier Transform]] is the application of Fourier analysis to finite sets of discrete samples. It is characterized by the fact that both the forward and inverse transforms are infinitely periodic. The DFT allows for efficient computation of frequency components in digital systems.

## Convolution Theorem

The [[Convolution Theorem]] is a fundamental result stating that convolution in the [[Spatial Domain]] is equivalent to point-wise multiplication in the [[Frequency Domain]], and vice versa. This theorem forms the basis for most frequency-domain filtering techniques.

## Mathematical Tools

- **Impulses**: A unit impulse (Dirac delta) has the sifting property, which extracts the value of a function at a specific point during integration.
- **Impulse Train**: A periodic sequence of impulses that, when transformed, results in another impulse train in the frequency domain.
- **Sinc Function**: The [[Fourier Transform]] of a rectangular (box) function is a [[Sinc Function]], demonstrating an inverse relationship between the width of the spatial function and the spread of its spectrum.

## Relationships
- [[Fourier Transform]] maps functions from the [[Spatial Domain]] to the [[Frequency Domain]].
- [[Sampling Theorem]] defines the [[Nyquist Rate]] and prevents [[Aliasing]].
- [[Discrete Fourier Transform]] is the discrete version of the [[Fourier Transform]].
- [[Convolution Theorem]] relates spatial convolution to frequency multiplication.
