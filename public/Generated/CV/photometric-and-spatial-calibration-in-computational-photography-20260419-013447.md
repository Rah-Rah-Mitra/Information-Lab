---
type: content
title: "Photometric and Spatial Calibration in Computational Photography"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:34:47.319051+00:00
summary: "Photometric calibration characterizes the mapping between light irradiance and digital pixel values, while spatial calibration addresses lens-induced blur and distortion. These processes are essential for high dynamic range imaging, super-resolution, and seamless image compositing. Understanding these camera-specific responses allows for for the reconstruction of high-fidelity images from multiple exposures or degraded inputs."
tags:
  - computer-vision
  - computational-photography
  - camera-calibration
  - image-processing
entities:
  - "[[Photometric Calibration]]"
  - "[[Radiometric Response Function]]"
  - "[[High Dynamic Range Imaging]]"
  - "[[Point Spread Function]]"
  - "[[]]"
  - "[[Vignetting]]"
  - "[[Tone Mapping]]"
  - "[[]]"
  - "[[Image Matting]]"
  - "[[Super-resolution]]"
  - "[[Bayer Pattern]]"
relationships:
  - source: "Photometric Calibration"
    target: "Radiometric Response Function"
    description: "involves characterizing"
  - source: "Photometric Calibration"
    target: "Vignetting"
    description: "addresses"
  - source: "Photometric Calibration"
    target: "High Dynamic Range Imaging"
    description: "enables"
  - source: "Photometric Calibration"
    target: "Tone Mapping"
    description: "requires"
  - source: "Photometric Calibration"
    target: "Image Matting"
    description: "is distinct from"
  - source: "High Dynamic Range Imaging"
    target: "Radiometric Response Function"
    description: "uses"
  - source: "High Dynamic Range Imaging"
    target: "Tone Mapping"
    description: "requires"
  - source: "Point Spread Function"
    target: "Vignetting"
    description: "is related to"
  - source: "Super-resolution"
    target: "Point Spread Function"
    description: "requires estimation of"
  - source: "Super-resolution"
    target: "Bayer Pattern"
    description: "often involves"
  - source: "Image Matting"
    target: "Tone Mapping"
    description: "is a different task from"
---

# Photometric and Spatial Calibration in Computational Photography

*Photometric calibration characterizes the mapping between light irradiance and digital pixel values, while spatial calibration addresses lens-induced blur and distortion. These processes are essential for high dynamic range imaging, super-resolution, and seamless image compositing. Understanding these camera-specific responses allows for for the reconstruction of high-fidelity images from multiple exposures or degraded inputs.*

This note explores the fundamental calibration processes required to transform raw sensor data into high-quality digital images through [[Photometric Calibration]] and spatial response estimation.

## Concept
[[Photometric Calibration]] is the process of characterizing the functions that map incoming irradiance into pixel values and the amount of noise present. It consists of several key components:

### Radiometric Response
The [[Radiometric Response Function]] maps photons arriving at the lens into digital values. This can be calibrated using an integrating sphere or by taking multiple exposures of the same scene to estimate the response curve and irradiance simultaneously.

### Vignetting and Noise
[[Vignetting]] refers to the darkening of pixel values near the periphery of an image, often caused by wide-angle lenses. [[Noise Level Estimation]] is also a critical part of photometric calibration, characterizing how noise is injected as a function of pixel value.

### Spatial Response
[[Point Spread Function]] (PSF) characterizes the optical blur induced by the lens and sensor. Estimating the PSF is essential for applications like [[Super-resolution]] and deblurring. A common method involves using a calibration pattern with edges at various orientations to solve a a least squares problem:

$$ K = \arg \min_{K} \| B - D I K \|_2^2 $$

This equation models the relationship between the sensed image $B$ and the predicted sharp image $I$ convolved with kernel $K$.

## High Dynamic Range Imaging
[[High Dynamic Range Imaging]] (HDR) aims to capture a wider range of radiance than a single exposure allows. This is achieved by: 
1. Estimating the [[Radiometric Response Function]].
2. Estimating a radiance map by blending pixels from bracketed exposures.
3. Performing [[Tone Mapping]] to compress the high dynamic range back into a displayable gamut.

[[Tone Mapping]] can be performed using global curves or local operators like [[Bilateral Filtering]] to avoid halos.

## Image Matting and Super-resolution
[[Image Matting]] is the process of cutting a foreground object out of an image to create a soft opacity channel $\alpha$. [[Super-resolution]] involves combining multiple images to recover higher spatial resolution and less noise. Both processes often deal with the complexities of the [[Bayer Pattern]] in [[Color Image Demosaicing]].

## Relationships
- [[Photometric Calibration]] involves characterizing the [[Radiometric Response Function]].
- [[Photometric Calibration]] addresses [[Vignetting]].
- [[Photometric Calibration]] enables [[High Dynamic Range Imaging]].
- [[Photometric Calibration]] requires [[Tone Mapping]].
- [[High Dynamic Range Imaging]] uses [[Radiometric Response Function]].
- [[High Dynamic Range Imaging]] requires [[Tone Mapping]].
- [[Point Spread Function]] is related to [[Vignetting]].
- [[Super-resolution]] requires estimation of [[Point Spread Function]].
- [[Super-resolution]] often involves [[Bayer Pattern]].
- [[Image Matting]] is a different task from [[Photometric Calibration]].
