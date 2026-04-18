---
type: content
title: "Scale-Invariant Feature Transform"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:15:02.431601400+00:00
summary: "A complex, heuristic algorithm for extracting invariant features from images, robust to scale, rotation, illumination, and viewpoint changes."
tags:
  - computer-vision
  - feature-extraction
  - image-processing
  - sift
entities:
  - "[[Scale-Invariant Feature Transform]]"
  - "[[Keypoints]]"
  - "[[Scale Space]]"
  - "[[Gaussian Kernel]]"
  - "[[Octave]]"
  - "[[Difference of Gaussians]]"
  - "[[Laplacian of a Gaussian]]"
  - "[[Hessian Matrix]]"
  - "[[Keypoint Descriptor]]"
  - "[[Harris-Stephens Corner Detector]]"
  - "[[Maximally Stable Extremal Regions]]"
relationships:
  - source: "Scale-Invariant Feature Transform"
    target: "Scale Space"
    description: "uses to find scale-invariant locations"
  - source: "Scale-Invariant Feature Transform"
    target: "Keypoints"
    description: "extracts as invariant features"
  - source: "Scale Space"
    target: "Gaussian Kernel"
    description: "is implemented using"
  - source: "Scale Space"
    target: "Octave"
    description: "is subdivided into"
  - source: "Scale-Invariant Feature Transform"
    target: "Difference of Gaussians"
    description: "detects extrema in"
  - source: "Difference of Gaussians"
    target: "Laplacian of a Gaussian"
    description: "approximates"
  - source: "Scale-Invariant Feature Transform"
    target: "Hessian Matrix"
    description: "uses to eliminate edge responses"
  - source: "Scale-Invariant Feature Transform"
    target: "Keypoint Descriptor"
    description: "computes to identify matches"
  - source: "Scale-Invariant Feature Transform"
    target: "Harris-Stephens Corner Detector"
    description: "is more robust than"
  - source: "Scale-Invariant Feature Transform"
    target: "Maximally Stable Extremal Regions"
    description: "is more robust than"
---

# Scale-Invariant Feature Transform

*A complex, heuristic algorithm for extracting invariant features from images, robust to scale, rotation, illumination, and viewpoint changes.*

The [[Scale-Invariant Feature Transform]] (SIFT) is a heuristic algorithm used to extract invariant features, known as [[Keypoints]], from an image. These features are robust to changes in scale, rotation, illumination, and 3-D viewpoint.

## Scale Space Construction

To achieve scale invariance, SIFT uses a [[Scale Space]], which is a multi-scale representation of an image. This is implemented by convolving the image with [[Gaussian Kernel]]s of varying standard deviations. The scale space is organized into [[Octave]]s, where each octave corresponds to a doubling of the scale parameter $\sigma$.

## Keypoint Detection

Initial keypoint locations are found by detecting extrema in the [[Difference of Gaussians]] (DoG) of two adjacent scale-space images. The DoG is an approximation to the [[Laplacian of a Gaussian]] (LoG). To refine these locations, SIFT uses a Taylor series expansion to achieve subpixel accuracy.

### Filtering Keypoints

Not all detected extrema are stable. SIFT rejects keypoints with low contrast and those that lie along edges. To eliminate edge responses, the [[Hessian Matrix]] of the DoG is computed, and the ratio of its principal curvatures is checked; if the ratio exceeds a threshold (typically 10), the keypoint is discarded.

## Orientation Assignment

To achieve rotation invariance, each keypoint is assigned a consistent orientation based on local image gradients. A histogram of orientations is formed, and the highest peak determines the dominant direction.

## Keypoint Descriptors

A [[Keypoint Descriptor]] is a 128-dimensional feature vector computed for a local region around the keypoint. It is based on gradient magnitudes and orientations, normalized to unit length to reduce the effects of illumination changes.

## Comparison to Other Methods

While the [[Harris-Stephens Corner Detector]] and [[Maximally Stable Extremal Regions]] (MSERs) are effective for images with similar scale and orientation, SIFT is required when dealing with significant scale changes, rotation, and viewpoint variations.

## Relationships
- [[Scale-Invariant Feature Transform]] uses [[Scale Space]] to find scale-invariant locations.
- [[Scale-Invariant Feature Transform]] extracts [[Keypoints]] as invariant features.
- [[Scale Space]] is implemented using [[Gaussian Kernel]]s.
- [[Scale Space]] is subdivided into [[Octave]]s.
- [[Scale-Invariant Feature Transform]] detects extrema in the [[Difference of Gaussians]].
- [[Difference of Gaussians]] approximates the [[Laplacian of a Gaussian]].
- [[Scale-Invariant Feature Transform]] uses the [[Hessian Matrix]] to eliminate edge responses.
- [[Scale-Invariant Feature Transform]] computes [[Keypoint Descriptor]]s to identify matches.
