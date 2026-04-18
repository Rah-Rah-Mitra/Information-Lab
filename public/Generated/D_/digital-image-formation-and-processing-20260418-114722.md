---
type: content
title: "Digital Image Formation and Processing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T11:47:22.462513400+00:00
summary: "An overview of the photometric image formation process, digital sensor characteristics, color spaces, and fundamental image processing operators."
tags:
  - computer-vision
  - image-processing
  - optics
  - digital-sensors
entities:
  - "[[Photometric Image Formation]]"
  - "[[Depth of Field]]"
  - "[[F-number]]"
  - "[[Chromatic Aberration]]"
  - "[[Vignetting]]"
  - "[[Fundamental Radiometric Relation]]"
  - "[[Digital Image Sensor]]"
  - "[[CCD]]"
  - "[[CMOS]]"
  - "[[Shutter Speed]]"
  - "[[Sampling Pitch]]"
  - "[[Fill Factor]]"
  - "[[Analog Gain]]"
  - "[[Sensor Noise]]"
  - "[[Analog to Digital Conversion]]"
  - "[[Aliasing]]"
  - "[[Nyquist Frequency]]"
  - "[[Point Spread Function]]"
  - "[[Modulation Transfer Function]]"
  - "[[Color Space]]"
  - "[[RGB]]"
  - "[[CIE XYZ]]"
  - "[[L*a*b* Color Space]]"
  - "[[Bayer Pattern]]"
  - "[[Demosaicing]]"
  - "[[Gamma Correction]]"
  - "[[YCbCr]]"
  - "[[HSV]]"
  - "[[Image Compression]]"
  - "[[Discrete Cosine Transform]]"
  - "[[Local Operator]]"
  - "[[Neighborhood Operator]]"
  - "[[Linear Filter]]"
  - "[[Convolution]]"
  - "[[Histogram Equalization]]"
relationships:
  - source: "Photometric Image Formation"
    target: "Depth of Field"
    description: "determines"
  - source: "Depth of Field"
    target: "F-number"
    description: "depends on"
  - source: "Photometric Image Formation"
    target: "Vignetting"
    description: "includes"
  - source: "Vignetting"
    target: "Fundamental Radiometric Relation"
    description: "is described by"
  - source: "Digital Image Sensor"
    target: "CCD"
    description: "implemented as"
  - source: "Digital Image Sensor"
    target: "CMOS"
    description: "implemented as"
  - source: "Digital Image Sensor"
    target: "Sensor Noise"
    description: "introduces"
  - source: "Analog to Digital Conversion"
    target: "Sensor Noise"
    description: "is affected by"
  - source: "Aliasing"
    target: "Nyquist Frequency"
    description: "occurs above"
  - source: "Point Spread Function"
    target: "Modulation Transfer Function"
    description: "Fourier transform yields"
  - source: "Color Space"
    target: "RGB"
    description: "includes"
  - source: "Color Space"
    target: "CIE XYZ"
    description: "includes"
  - source: "Color Space"
    target: "L*a*b* Color Space"
    description: "includes"
  - source: "Bayer Pattern"
    target: "Demosaicing"
    description: "requires"
  - source: "Local Operator"
    target: "Histogram Equalization"
    description: "example of"
---

# Digital Image Formation and Processing

*An overview of the photometric image formation process, digital sensor characteristics, color spaces, and fundamental image processing operators.*

Digital image formation is the process by which a 3D scene is projected onto a 2D sensor, involving the interaction of lighting, optics, and electronic sensing. 

## Photometric Image Formation

Optical systems are subject to various imperfections. [[Depth of Field]] is the allowable depth variation in a scene that limits the circle of confusion to an acceptable size, and it is a function of the focus distance and the [[F-number]] (the ratio of focal length to aperture diameter). Lenses also suffer from [[Chromatic Aberration]], where different wavelengths focus at different distances, and [[Vignetting]], the tendency for image brightness to fall off toward the edges. The relationship between scene radiance and the irradiance reaching the sensor is defined by the [[Fundamental Radiometric Relation]].

## Digital Image Sensors

Modern cameras use a [[Digital Image Sensor]], typically implemented as either a [[CCD]] (Charge Coupled Device) or [[CMOS]] (Complementary Metal Oxide on Silicon). Key performance factors include [[Shutter Speed]], [[Sampling Pitch]], and the [[Fill Factor]] (the active sensing area as a fraction of the total area). The signal is boosted by [[Analog Gain]] before undergoing [[Analog to Digital Conversion]]. Throughout this process, [[Sensor Noise]] is introduced from various sources such as shot noise and quantization noise.

## Sampling and Aliasing

When a continuous light field is sampled by a sensor, [[Aliasing]] occurs if the signal contains frequencies above the [[Nyquist Frequency]] (half the sampling frequency). The response of a pixel to a point light source is described by the [[Point Spread Function]], and its Fourier transform is the [[Modulation Transfer Function]], which can be used to estimate the amount of aliasing.

## Color Representation

Digital cameras capture color using a [[Color Space]]. The [[RGB]] space is common but not perceptually uniform. The [[CIE XYZ]] space was developed to represent all perceivable colors, and the [[L*a*b* Color Space]] provides a more perceptually uniform mapping. Most cameras use a [[Bayer Pattern]] filter array, requiring [[Demosaicing]] to interpolate missing color values. To compensate for the non-linear response of display devices, [[Gamma Correction]] is applied to the luminance values.

Other common spaces include [[YCbCr]], used in image compression, and [[HSV]] (Hue, Saturation, Value).

## Image Processing Operators

Image processing involves mapping pixel values using various operators. A [[Local Operator]] (or point process) manipulates each pixel independently, with [[Histogram Equalization]] being a primary example for contrast enhancement. A [[Neighborhood Operator]] uses a local window of pixels to determine the output, often implemented as a [[Linear Filter]]. The most common linear filter is [[Convolution]], which is a linear shift-invariant operation used for blurring or sharpening.

## Relationships

- Photometric Image Formation determines Depth of Field
- Depth of Field depends on F-number
- Photometric Image Formation includes Vignetting
- Vignetting is described by Fundamental Radiometric Relation
- Digital Image Sensor implemented as CCD
- Digital Image Sensor implemented as CMOS
- Digital Image Sensor introduces Sensor Noise
- Analog to Digital Conversion is affected by Sensor Noise
- Aliasing occurs above Nyquist Frequency
- Point Spread Function Fourier transform yields Modulation Transfer Function
- Color Space includes RGB
- Color Space includes CIE XYZ
- Color Space includes L*a*b* Color Space
- Bayer Pattern requires Demosaicing
- Local Operator example of Histogram Equalization
