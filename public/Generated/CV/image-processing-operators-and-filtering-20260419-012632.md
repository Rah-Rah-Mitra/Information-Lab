---
type: content
title: "Image Processing Operators and Filtering"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:26:32.461275700+00:00
summary: "Image processing involves various operators that transform pixel values to improve image quality or prepare data for analysis. These range from simple point operators like brightness adjustments to complex neighborhood operators like convolution and non-linear filters. Understanding these mechanisms is crucial for compensating for artifacts introduced by compression or sensor noise."
tags:
  - computer-vision
  - image-processing
  - signal-processing
  - filtering
  - mathematical-morphology
entities:
  - "[[Image Processing]]"
  - "[[Point Operators]]"
  - "[[Neighborhood Operators]]"
  - "[[Linear Filtering]]"
  - "[[Convolution]]"
  - "[[Non-linear Filtering]]"
  - "[[Bilateral Filtering]]"
  - "[[Fourier Transform]]"
  - "[[Morphological Operations]]"
  - "[[Histogram Equalization]]"
relationships:
  - source: "Image Processing"
    target: "Point Operators"
    description: "includes"
  - source: "Image Processing"
    target: "Neighborhood Operators"
    description: "includes"
  - source: " "
    target: "Point Operators"
    description: "is a type of"
  - source: "Neighborhood Operators"
    target: "Linear Filtering"
    description: "includes"
  - source: "Neighborhood Operators"
    target: "Non-linear Filtering"
    description: "includes"
  - source: "Linear Filtering"
    target: "Convolution"
    description: "uses"
  - source: "Non-linear Filtering"
    target: "Bilateral Filtering"
    description: "is an example of"
  - source: "Linear Filtering"
    target: "Fourier Transform"
    description: "analyzed by"
  - source: "Morphological Operations"
    target: "Image Processing"
    description: "is a type of"
  - source: "Histogram Equalization"
    target: "Point Operators"
    description: "is a type of"
---

# Image Processing Operators and Filtering

*Image processing involves various operators that transform pixel values to improve image quality or prepare data for analysis. These range from simple point operators like brightness adjustments to complex neighborhood operators like convolution and non-linear filters. Understanding these mechanisms is crucial for compensating for artifacts introduced by compression or sensor noise.*

[[Image Processing]] is the first stage in most computer vision algorithms, used to preprocess images for tasks like exposure correction, noise reduction, and sharpening. It can be categorized into several classes of operators based on their spatial extent.

## Concept

### Point Operators
[[Point Operators]] manipulate each pixel independently of its neighbors. Common examples include:
- **Brightness and Contrast**: Adjusting the gain and bias of a signal.
- **Gamma Correction**: Removing non-linear mappings between input radiance and quantized pixel values.
- **Histogram Equalization**: A global process that maps intensities to a flat distribution to improve contrast.

$$ g(\mathbf{x}) = a f(\mathbf{x}) + b $$

This equation models simple brightness and contrast adjustments where $a$ is the gain and $b$ is the bias.

### Neighborhood Operators
[[Neighborhood Operators]] use a collection of pixel values in the vicinity of a given pixel to determine its output. These are divided into:

#### Linear Filtering
[[Linear Filtering]] involves fixed weighted combinations of pixels. The most common form is [[Convolution]], which is a linear shift-invariant (LSI) operator. 

$$ g(i, j) = \sum_{k,l} f(i+k, j+l) h(k, l) $$

This formula represents the convolution of an image $f$ with a kernel $h$.

#### Non-linear Filtering
[[Non-linear Filtering]] can outperform linear filters in specific scenarios, such as edge-preserving smoothing. Examples include:
- **Median Filtering**: Effective for removing shot noise by selecting the median value in a neighborhood.
- **Bilateral Filtering**: An edge-preserving filter that uses both a domain kernel (spatial distance) and a range kernel (intensity similarity).

$$ w_{i,j,k,l} = \exp\left(-\frac{(i-k)^2 + (j-l)^2}{2\sigma_d^2}\right) \exp\left(-\frac{(f_{i,j} - f_{k,l})^2}{2\sigma_r^2}\right) $$ 

This expression defines the bilateral weight as a product of a domain and range kernel.

### Frequency Domain Analysis
The [[Fourier Transform]] allows for analyzing the frequency characteristics of filters. For instance, the frequency response of a filter can be determined by passing a sinusoid through it and observing the resulting magnitude and phase.

## Relationships
- [[Image Processing]] includes [[Point Operators]]
- [[Image Processing]] includes [[Neighborhood Operators]]
- [[Point Operators]] includes [[Histogram Equalization]]
- [[Neighborhood Operators]] includes [[Linear Filtering]]
- [[Neighborhood Operators]] includes [[Non-linear Filtering]]
- [[Linear Filtering]] uses [[Convolution]]
- [[Non-linear Filtering]] includes [[Bilateral Filtering]]
- [[Linear Filtering]] is analyzed by [[Fourier Transform]]
