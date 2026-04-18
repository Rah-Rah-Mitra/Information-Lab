---
type: content
title: "Photometric Calibration and High Dynamic Range Imaging"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:53:10.411561100+00:00
summary: "An overview of camera calibration techniques for photometric response, noise level functions, vignetting, and the creation of high dynamic range images."
tags:
  - computer-vision
  - photometry
  - camera-calibration
  - hdr-imaging
  - image-processing
entities:
  - "[[Photometric Calibration]]"
  - "[[Camera Response Function]]"
  - "[[Noise Level Function]]"
  - "[[Vignetting]]"
  - "[[Point Spread Function]]"
  - "[[High Dynamic Range Imaging]]"
  - "[[Radiance Map]]"
  - "[[Tone Mapping]]"
  - "[[Bilateral Filtering]]"
  - "[[Image Matting]]"
  - "[[Super-resolution]]"
  - "[[Demosaicing]]"
relationships:
  - source: "Photometric Calibration"
    target: "Camera Response Function"
    description: "estimates"
  - source: "Photometric Calibration"
    target: "Noise Level Function"
    description: "estimates"
  - source: "Photometric Calibration"
    target: "Vignetting"
    description: "corrects"
  - source: "Photometric Calibration"
    target: "Point Spread Function"
    description: "calibrates"
  - source: "High Dynamic Range Imaging"
    target: "Radiance Map"
    description: "produces"
  - source: "High Dynamic Range Imaging"
    target: "Camera Response Function"
    description: "depends on"
  - source: "Radiance Map"
    target: "Tone Mapping"
    description: "is processed by"
  - source: "Tone Mapping"
    target: "Bilateral Filtering"
    description: "uses for edge-preserving smoothing"
  - source: "Image Matting"
    target: "Super-resolution"
    description: "shares optimization techniques with"
  - source: "Super-resolution"
    target: "Demosaicing"
    description: "generalizes"
---

# Photometric Calibration and High Dynamic Range Imaging

*An overview of camera calibration techniques for photometric response, noise level functions, vignetting, and the creation of high dynamic range images.*

[[Photometric Calibration]] is the process of characterizing an imaging system's response to light to ensure that captured pixel values accurately represent the scene's radiance.

## Camera Response and Noise

Calibration typically involves estimating the [[Camera Response Function]], which maps incoming irradiance to pixel values. This can be achieved using calibration charts or by taking multiple exposures of the same scene. Additionally, the [[Noise Level Function]] is estimated to characterize the amount of noise injected as a function of pixel value, often by fitting a lower envelope to the spatial or temporal variance of measurements.

## Optical Artifacts

[[Vignetting]] refers to the darkening of image corners, common in wide-angle lenses. It can be calibrated using an integrating sphere or by analyzing radial intensity variations in a single image. The [[Point Spread Function]] (PSF) encodes the optical blur convolved with the image; it is estimated using patterns of edges or point light sources to enable applications like de-blurring and super-resolution.

## High Dynamic Range Imaging

[[High Dynamic Range Imaging]] (HDR) allows for the capture of scenes with a radiance range exceeding the sensor's capacity. The process involves three main stages:
1. Estimating the [[Camera Response Function]] from aligned images of different exposures.
2. Creating a [[Radiance Map]] by blending pixels from these exposures.
3. Applying [[Tone Mapping]] to compress the HDR image back into a displayable 8-bit gamut.

Local tone mapping often employs [[Bilateral Filtering]] to reduce halos by performing edge-preserving smoothing on the log-luminance base layer.

## Advanced Image Reconstruction

[[Super-resolution]] combines multiple low-resolution images to produce a high-resolution composite, often utilizing Bayesian inference and image priors. A specialized case of this is [[Demosaicing]], which interpolates samples from a color filter array (CFA), such as the Bayer pattern, into a full-color RGB image.

[[Image Matting]] is the process of estimating a soft opacity channel (alpha matte) and foreground colors to cut an object out of a background, often using trimaps or global optimization techniques like the matting Laplacian.

## Relationships

- [[Photometric Calibration]] estimates [[Camera Response Function]]
- [[Photometric Calibration]] estimates [[Noise Level Function]]
- [[Photometric Calibration]] corrects [[Vignetting]]
- [[Photometric Calibration]] calibrates [[Point Spread Function]]
- [[High Dynamic Range Imaging]] produces [[Radiance Map]]
- [[High Dynamic Range Imaging]] depends on [[Camera Response Function]]
- [[Radiance Map]] is processed by [[Tone Mapping]]
- [[Tone Mapping]] uses for edge-preserving smoothing [[Bilateral Filtering]]
- [[Image Matting]] shares optimization techniques with [[Super-resolution]]
- [[Super-resolution]] generalizes [[Demosaicing]]
