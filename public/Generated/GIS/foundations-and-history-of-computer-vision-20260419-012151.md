---
type: content
title: "Foundations and History of Computer Vision"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-19T01:21:51.691390200+00:00
summary: "Computer vision involves recovering 3D structure and appearance from 2D imagery through physics-based, statistical, and engineering approaches. The field has evolved from early edge-based scene understanding to modern deep learning and SLAM. It serves diverse applications ranging from medical imaging to autonomous vehicles."
tags:
  - computer-vision
  - history-of-science
  - image-processing
  - machine-learning
entities:
  - "[[Computer Vision]]"
  - "[[Image Formation]]"
  - "[[Structure From Motion]]"
  - "[[Deep Learning]]"
  - "[[Simultaneous Localization and Mapping]]"
  - "[[Statistical Modeling]]"
  - "[[Physics-Based Vision]]"
  - "[[Inverse Problem]]"
relationships:
  - source: "Computer Vision"
    target: "Inverse Problem"
    description: "is framed as an"
  - source: "Computer Vision"
    target: "Image Formation"
    description: "seeks to invert"
  - source: "Computer Vision"
    target: "Structure From Motion"
    description: "utilizes"
  - source: "Computer Vision"
    target: "Deep Learning"
    description: "incorporates"
  - source: "Computer Vision"
    target: "Simultaneous Localization and Mapping"
    description: "employs"
  - source: "Computer Vision"
    target: "Statistical Modeling"
    description: "uses"
  - source: "Computer Vision"
    target: "Physics-Based Vision"
    description: "relies on"
  - source: "Structure From Motion"
    target: "Computer Vision"
    description: "is a subfield of"
---

# Foundations and History of Computer Vision

*Computer vision involves recovering 3D structure and appearance from 2D imagery through physics-based, statistical, and engineering approaches. The field has evolved from early edge-based scene understanding to modern deep learning and SLAM. It serves diverse applications ranging from medical imaging to autonomous vehicles.*

[[Computer Vision]] is the field of developing mathematical techniques to recover the three-dimensional shape and appearance of objects from two-dimensional imagery. It is fundamentally an [[Inverse Problem]], where the goal is to describe the world seen in images by inverting the process of [[Image Formation]].

## Concept
Because vision is an inverse problem, researchers must use various frameworks to disambiguate potential solutions. These include:
- **Scientific Approach:** Building detailed models of the physics of the system (radiometry, optics, and sensor design) to understand how light interacts with a scene.
- **Statistical Approach:** Using probabilistic models to quantify uncertainty and the likelihood of unknowns, often through [[Statistical Modeling]] or [[Bayesian Modeling]].
- **Engineering Approach:** Developing robust and efficient algorithms that work in real-world conditions.
- **Data-driven Approach:** Using large datasets to learn model parameters, a method that has become central through [[Deep Learning]].

Historically, the field has transitioned through several eras. In the 1970s, research focused on extracting edges and topological structures. The 1980s saw the rise of [[Image Pyramids]], [[Shape From Shading]], and variational optimization. The 1990s introduced significant advances in [[Structure From Motion]] and [[Simultaneous Localization and Mapping]] (SLAM), as well as more sophisticated [[Image Segmentation]] techniques.

## Relationships
- [[Computer Vision]] is framed as an [[Inverse Problem]]
- [[Computer Vision]] seeks to invert [[Image Formation]]
- [[Computer Vision]] utilizes [[Structure From Motion]]
- [[Computer Vision]] incorporates [[Deep Learning]]
- [[Computer Vision]] employs [[Simultaneous Localization and Mapping]]
- [[Computer Vision]] uses [[Statistical Modeling]]
- [[Computer Vision]] relies on [[Physics-Based Vision]]
