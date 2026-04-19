---
type: content
title: "Manhattan-World Stereo and Structure Priors"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:44:29.306696700+00:00
summary: "Manhattan-world stereo uses structure priors to reconstruct piecewise planar surfaces in multi-view stereo. It improves robustness in textureless or non-Lambertian environments by replacing depthmaps with planemaps. This approach is often extended with image classification to handle non-planar structures like vegetation."
tags:
  - computer-vision
  - multi-view-stereo
  - structure-priors
  - mrf
  - planar-reconstruction
entities:
  - "[[Manhattan-World Stereo]]"
  - "[[Multi-View Stereo]]"
  - "[[Markov Random Field]]"
  - "[[Planemap]]"
  - "[[Structure Priors]]"
  - "[[Piecewise Planar Structure]]"
  - "[[Image Classification]]"
  - "[[Non-Lambertian Surfaces]]"
relationships:
  - source: "Manhattan-World Stereo"
    target: "Multi-View Stereo"
    description: "is an extension of"
  - source: "Manhattan-World Stereo"
    target: "Markov Random Field"
    description: "uses"
  - source: "Manhattan-World Stereo"
    target: "Planemap"
    description: "produces"
  - source: "Manhattan-World Stereo"
    target: "Structure Priors"
    description: "exploits"
  - source: "Manhattan-World Stereo"
    target: "Piecewise Planar Structure"
    description: "assumes"
  - source: "Image Classification"
    target: "Manhattan-World Stereo"
    description: "improves"
  - source: "Manhattan-World Stereo"
    target: "Non-Lambertian Surfaces"
    description: "struggles with"
  - source: "Planemap"
    target: "Multi-View Stereo"
    description: "replaces depthmap in"
---

# Manhattan-World Stereo and Structure Priors

*Manhattan-world stereo uses structure priors to reconstruct piecewise planar surfaces in multi-view stereo. It improves robustness in textureless or non-Lambertian environments by replacing depthmaps with planemaps. This approach is often extended with image classification to handle non-planar structures like vegetation.*

This note explores the transition from standard depthmap-based reconstruction to planemap-based reconstruction using structure priors.

## Concept
In standard [[Multi-View Stereo]] (MVS), the goal is to estimate a dense depthmap. However, in environments with many orthogonal surfaces (like architecture), [[Manhattan-World Stereo]] provides a more robust alternative. Instead of assigning a depth value to each pixel, the algorithm assigns a plane ID, resulting in a [[Planemap]].

[[Manhattan-World Stereo]] assumes a [[Piecewise Planar Structure]] where surfaces are aligned with dominant orthogonal directions. The reconstruction is formulated as a [[Markov Random Field]] (MRF) energy minimization problem. The energy function consists of two main terms:

1. **Data Term** (Φ): Measures visibility inconsistency between a hypothesized plane and the reconstructed 3D points. It penalizes cases where the plane would occlude points or where points reside in the 'free-space' in front of the plane.

2. **Smoothness Term** (Έ): Enforces spatial consistency. It penalizes depth discrepancies between neighbors but is down-weighted at image edges where discontinuities are expected.

$$ Έ(k_p, k_q) = Έ_{DE}(k_p, k_q) Έ_E(k_p, k_q) $$

This formula models the smoothness penalty as a product of depth discrepancy and edge presence.

To handle non-architectural elements like trees or bushes, [[Image Classification]] can be used to label regions as planar or non-planar. This modifies the data term to favor non-planar labels in vegetation, preventing the forced planarity that would otherwise cause errors in those regions.

While these methods are highly effective for architectural scenes, they still face challenges with [[Non-Lambertian Surfaces]] (e.g., specular or transparent objects) which violate the fundamental assumption of consistent photometric appearance across views.

## Relationships
- [[Manhattan-World Stereo]] is an extension of [[Multi-View Stereo]]
- [[Manhattan-World Stereo]] uses [[Markov Random Field]]
- [[Manhattan-World Stereo]] produces [[Planemap]]
- [[Manhattan-World Stereo]] exploits [[Structure Priors]]
- [[Manhattan-World Stereo]] assumes [[Piecewise Planar Structure]]
- [[Image Classification]] improves [[Manhattan-World Stereo]]
- [[Manhattan-World Stereo]] struggles with [[Non-Lambertian Surfaces]]
- [[Planemap]] replaces depthmap in [[Multi-View Stereo]]
