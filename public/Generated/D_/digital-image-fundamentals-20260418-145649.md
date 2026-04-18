---
type: content
title: "Digital Image Fundamentals"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:56:49.595081100+00:00
summary: "An overview of the electromagnetic spectrum, image sensing, sampling, quantization, and the mathematical representation of digital images."
tags:
  - computer-vision
  - digital-image-processing
  - signal-processing
entities:
  - "[[Electromagnetic Spectrum]]"
  - "[[Photon]]"
  - "[[Visible Spectrum]]"
  - "[[Monochromatic Light]]"
  - "[[Gray Level]]"
  - "[[Gray Scale]]"
  - "[[Radiance]]"
  - "[[Luminance]]"
  - "[[Brightness]]"
  - "[[Image Sensing]]"
  - "[[Photodiode]]"
  - "[[Sensor Strip]]"
  - "[[Sensor Array]]"
  - "[[Charge-Coupled Device]]"
  - "[[Image Formation Model]]"
  - "[[Illumination Component]]"
  - "[[Reflectance Component]]"
  - "[[Sampling]]"
  - "[[Quantization]]"
  - "[[Pixel]]"
  - "[[Spatial Domain]]"
  - "[[Dynamic Range]]"
  - "[[Image Contrast]]"
  - "[[Spatial Resolution]]"
  - "[[Intensity Resolution]]"
  - "[[False Contouring]]"
  - "[[Isopreference Curve]]"
  - "[[Image Interpolation]]"
  - "[[Nearest Neighbor Interpolation]]"
  - "[[Bilinear Interpolation]]"
  - "[[Bicubic Interpolation]]"
  - "[[Neighborhood]]"
  - "[[4-Neighbors]]"
  - "[[8-Neighbors]]"
  - "[[Adjacency]]"
  - "[[m-Adjacency]]"
  - "[[Connected Component]]"
  - "[[Region]]"
  - "[[Boundary]]"
  - "[[Edge]]"
  - "[[Euclidean Distance]]"
  - "[[City-Block Distance]]"
  - "[[Chessboard Distance]]"
  - "[[Elementwise Operation]]"
  - "[[Linear Operator]]"
relationships:
  - source: "Electromagnetic Spectrum"
    target: "Visible Spectrum"
    description: "contains"
  - source: "Visible Spectrum"
    target: "Monochromatic Light"
    description: "can be simplified to"
  - source: "Monochromatic Light"
    target: "Gray Level"
    description: "is characterized by"
  - source: "Image Sensing"
    target: "Photodiode"
    description: "can be implemented using"
  - source: "Image Sensing"
    target: "Sensor Strip"
    description: "can be implemented using"
  - source: "Image Sensing"
    target: "Sensor Array"
    description: "can be implemented using"
  - source: "Sensor Array"
    target: "Charge-Coupled Device"
    description: "often uses"
  - source: "Image Formation Model"
    target: "Illumination Component"
    description: "consists of"
  - source: "Image Formation Model"
    target: "Reflectance Component"
    description: "consists of"
  - source: "Sampling"
    target: "Pixel"
    description: "creates"
  - source: "Quantization"
    target: "Pixel"
    description: "assigns intensity to"
  - source: "Spatial Resolution"
    target: "Sampling"
    description: "depends on"
  - source: "Intensity Resolution"
    target: "Quantization"
    description: "depends on"
  - source: "Image Interpolation"
    target: "Nearest Neighbor Interpolation"
    description: "includes"
  - source: "Image Interpolation"
    target: "Bilinear Interpolation"
    description: "includes"
---

# Digital Image Fundamentals

*An overview of the electromagnetic spectrum, image sensing, sampling, quantization, and the mathematical representation of digital images.*

Digital image fundamentals encompass the physical nature of light, the process of converting energy into a digital format, and the mathematical tools used to represent and manipulate images.

## Light and the Electromagnetic Spectrum

Light is a form of electromagnetic radiation that can be sensed by the eye. The [[Electromagnetic Spectrum]] is characterized by wavelength, frequency, and energy, where energy is carried by massless particles called [[Photon]]s. The [[Visible Spectrum]] is a narrow band of this spectrum, typically ranging from 0.43 $\mu$m (violet) to 0.79 $\mu$m (red).

Light that lacks color is termed [[Monochromatic Light]], and its only attribute is intensity, often referred to as a [[Gray Level]]. The range of these values is the [[Gray Scale]]. To describe chromatic light, three quantities are used: [[Radiance]] (total energy flow), [[Luminance]] (perceived energy), and [[Brightness]] (a subjective descriptor of perception).

## Image Sensing and Acquisition

[[Image Sensing]] involves transforming incident energy into a voltage. This can be achieved through various arrangements:
- **Single Sensing Element**: Such as a [[Photodiode]], which produces a voltage proportional to light intensity.
- **Sensor Strip**: An in-line array of sensors used in flatbed scanners or airborne imaging.
- **Sensor Array**: A 2-D grid of elements, commonly implemented as a [[Charge-Coupled Device]] (CCD) array in digital cameras.

## Image Formation Model

An image can be modeled as a two-dimensional function $f(x, y)$. The [[Image Formation Model]] describes this as the product of two components: the [[Illumination Component]] $i(x, y)$ and the [[Reflectance Component]] $r(x, y)$:
$$f(x, y) = i(x, y)r(x, y)$$

## Digitization: Sampling and Quantization

To convert a continuous image into a digital one, two processes are required:
1. [[Sampling]]: The digitization of coordinate values.
2. [[Quantization]]: The digitization of amplitude (intensity) values.

These processes result in the creation of a [[Pixel]] (picture element). The area spanned by the coordinates of an image is the [[Spatial Domain]].

### Resolution and Quality
- [[Spatial Resolution]] is the smallest discernible detail, often measured in dots per inch (dpi).
- [[Intensity Resolution]] is the smallest discernible change in intensity level, typically measured in bits (e.g., 8-bit resolution).
- Insufficient intensity levels in smooth areas can lead to [[False Contouring]].
- The interaction between spatial and intensity resolution is often visualized using [[Isopreference Curve]]s.

## Image Interpolation

[[Image Interpolation]] is used for resizing, rotating, or correcting images by estimating values at unknown locations:
- [[Nearest Neighbor Interpolation]]: Assigns the value of the closest pixel; simple but prone to distortion.
- [[Bilinear Interpolation]]: Uses the four nearest neighbors to estimate intensity.
- [[Bicubic Interpolation]]: Uses the sixteen nearest neighbors, preserving finer detail and serving as a commercial standard.

## Pixel Relationships

Pixels are analyzed based on their [[Neighborhood]]. A pixel $p$ has [[4-Neighbors]] (horizontal and vertical) and [[8-Neighbors]] (which include diagonal neighbors).

### Adjacency and Connectivity
[[Adjacency]] defines how pixels are related based on their intensity values. Types include 4-adjacency, 8-adjacency, and [[m-Adjacency]] (mixed adjacency), the latter of which is used to eliminate ambiguities in 8-adjacency.

Pixels are [[Connected Component]]s if a path exists between them. A [[Region]] is a connected set of pixels. The [[Boundary]] of a region is the set of pixels in the region adjacent to the background. This differs from an [[Edge]], which is a local intensity discontinuity.

### Distance Measures
Common metrics for measuring distance between pixels include:
- [[Euclidean Distance]]: The straight-line distance.
- [[City-Block Distance]]: Also known as $D_4$ distance.
- [[Chessboard Distance]]: Also known as $D_8$ distance.

## Mathematical Tools

Image processing often employs [[Elementwise Operation]]s, where operations are performed on corresponding pixel pairs, as opposed to matrix products. A [[Linear Operator]] is defined by its properties of additivity and homogeneity.

## Relationships

- [[Electromagnetic Spectrum]] contains [[Visible Spectrum]].
- [[Visible Spectrum]] can be simplified to [[Monochromatic Light]].
- [[Monochromatic Light]] is characterized by [[Gray Level]].
- [[Image Sensing]] can be implemented using [[Photodiode]], [[Sensor Strip]], or [[Sensor Array]].
- [[Sensor Array]] often uses [[Charge-Coupled Device]].
- [[Image Formation Model]] consists of [[Illumination Component]] and [[Reflectance Component]].
- [[Sampling]] creates [[Pixel]].
- [[Quantization]] assigns intensity to [[Pixel]].
- [[Spatial Resolution]] depends on [[Sampling]].
- [[Intensity Resolution]] depends on [[Quantization]].
- [[Image Interpolation]] includes [[Nearest Neighbor Interpolation]], [[Bilinear Interpolation]], and [[Bicubic Interpolation]].
