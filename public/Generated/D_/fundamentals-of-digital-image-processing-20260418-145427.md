---
type: content
title: "Fundamentals of Digital Image Processing"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:54:27.986349200+00:00
summary: "An introduction to the scope, history, and applications of digital image processing, including its relationship to computer vision and image analysis."
tags:
  - computer-vision
  - digital-image-processing
  - medical-imaging
  - image-analysis
entities:
  - "[[Digital Image Processing]]"
  - "[[Digital Image]]"
  - "[[Pixel]]"
  - "[[Image Analysis]]"
  - "[[Computer Vision]]"
  - "[[Artificial Intelligence]]"
  - "[[Low-Level Processes]]"
  - "[[Mid-Level Processes]]"
  - "[[High-Level Processes]]"
  - "[[Computerized Axial Tomography]]"
  - "[[Electromagnetic Spectrum]]"
  - "[[Photon]]"
relationships:
  - source: "Digital Image Processing"
    target: "Digital Image"
    description: "processes"
  - source: "Digital Image"
    target: "Pixel"
    description: "is composed of"
  - source: "Image Analysis"
    target: "Digital Image Processing"
    description: "overlaps with"
  - source: "Computer Vision"
    target: "Digital Image Processing"
    description: "extends from"
  - source: "Computer Vision"
    target: "Artificial Intelligence"
    description: "is a branch of"
  - source: "Digital Image Processing"
    target: "Low-Level Processes"
    description: "includes"
  - source: "Digital Image Processing"
    target: "Mid-Level Processes"
    description: "includes"
  - source: "Digital Image Processing"
    target: "High-Level Processes"
    description: "includes"
  - source: "Computerized Axial Tomography"
    target: "Digital Image Processing"
    description: "is an application of"
  - source: "Digital Image"
    target: "Electromagnetic Spectrum"
    description: "is often generated from"
  - source: "Electromagnetic Spectrum"
    target: "Photon"
    description: "consists of"
---

# Fundamentals of Digital Image Processing

*An introduction to the scope, history, and applications of digital image processing, including its relationship to computer vision and image analysis.*

[[Digital Image Processing]] is the use of digital computers to process images that are represented as finite, discrete quantities of intensity values.

## Core Concepts

An image is defined as a two-dimensional function $f(x, y)$, where $x$ and $y$ are spatial coordinates and the amplitude $f$ is the intensity or gray level. A [[Digital Image]] is specifically one where the coordinates and intensity values are all finite and discrete. The basic elements of a digital image are called [[Pixel]]s (picture elements).

## The Processing Continuum

There is a a continuum of computerized processes ranging from basic image manipulation to cognitive emulation:

- **[[Low-Level Processes]]**: Primitive operations where both inputs and outputs are images (e.g., noise reduction, contrast enhancement).
- **[[Mid-Level Processes]]**: Tasks where inputs are images but outputs are attributes extracted from those images (e.g., [[Image Analysis]], segmentation, and object recognition).
- **[[High-Level Processes]]**: Cognitive functions that make sense of recognized objects, often falling into the domain of [[Computer Vision]].

[[Computer Vision]] is a branch of [[Artificial Intelligence]] aimed at emulating human vision to make inferences and take actions based on visual inputs.

## Applications and Technology

Digital image processing is applied across various fields, most notably in medical imaging and space exploration. A key example is [[Computerized Axial Tomography]] (CAT/CT), which uses X-ray sources and detectors to construct 3D renditions of the interior of an object.

Most imaging systems rely on the [[Electromagnetic Spectrum]], utilizing waves or [[Photon]]s of varying energy levels—from gamma rays and X-rays to visible light and radio waves—to generate image data.

## Relationships

- Digital Image Processing processes [[Digital Image]]
- Digital Image is composed of [[Pixel]]
- Image Analysis overlaps with [[Digital Image Processing]]
- Computer Vision extends from [[Digital Image Processing]]
- Computer Vision is a branch of [[Artificial Intelligence]]
- Digital Image Processing includes [[Low-Level Processes]]
- Digital Image Processing includes [[Mid-Level Processes]]
- Digital Image Processing includes [[High-Level Processes]]
- Computerized Axial Tomography is an application of [[Digital Image Processing]]
- Digital Image is often generated from [[Electromagnetic Spectrum]]
- Electromagnetic Spectrum consists of [[Photon]]
