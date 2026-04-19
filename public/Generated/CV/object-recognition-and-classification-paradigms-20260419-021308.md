---
type: content
title: "Object Recognition And Classification Paradigms"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:13:08.504817+00:00
summary: "This note explores the evolution of computer vision tasks from instance recognition to generic category classification and object detection. It details various algorithmic approaches including feature-based methods, part-based models, and deep learning. The transition from handcrafted features to deep neural networks has significantly improved performance across all these tasks."
tags:
  - computer-vision
  - image-classification
  - object-detection
  - machine-learning
entities:
  - "[[Instance Recognition]]"
  - "[[Image Classification]]"
  - "[[Object Detection]]"
  - "[[Bag Of Words]]"
  - "[[Part-Based Models]]"
  - "[[Active Appearance Models]]"
  - "[[Deep Neural Networks]]"
  - "[[DeepFace]]"
  - "[[Histogram Of Oriented Gradients]]"
  - "[[Intersection Over Union]]"
relationships:
  - source: "Instance Recognition"
    target: "Image Classification"
    description: "differs from"
  - source: "Image Classification"
    target: "Bag Of Words"
    description: "can use"
  - source: "Image Classification"
    target: "Part-Based Models"
    description: "can use"
  - source: "Image Classification"
    target: "Deep Neural Networks"
    description: "is revolutionized by"
  - source: "Image Classification"
    target: "Active Appearance Models"
    description: "is a specialized form of"
  - source: "Object Detection"
    target: "Deep Neural Networks"
    description: "is revolutionized by"
  - source: "Object Detection"
    target: "Histogram Of Oriented Gradients"
    description: "uses"
  - source: "Object Detection"
    target: "Intersection Over Union"
    description: "is evaluated by"
  - source: "Part-Based Models"
    target: "Deep Neural Networks"
    description: "is being replaced by"
  - source: "Part-Based Models"
    target: "Active Appearance Models"
    description: "is a type of"
  - source: "Object Detection"
    target: "Instance Recognition"
    description: "is a more general version of"
---

# Object Recognition And Classification Paradigms

*This note explores the evolution of computer vision tasks from instance recognition to generic category classification and object detection. It details various algorithmic approaches including feature-based methods, part-based models, and deep learning. The transition from handcrafted features to deep neural networks has significantly improved performance across all these tasks.*

This note summarizes the core paradigms of computer vision for recognizing and localizing objects in images.

## Concept

Computer vision tasks are broadly categorized into [[Instance Recognition]], which focuses on identifying specific known objects (e.g., a particular car), and [[Image Classification]], which aims to categorize images into generic classes (eg, "cat", "car"). [[Object Detection]] goes a step further by both localizing the object with a bounding box and labeling it. 

### Feature-Based Approaches

Early methods for [[Image Classification]] relied on handcrafted features. One such method is the [[Bag Of Words]] approach, which represents images as unordered collections of feature descriptors (like SIFT) quantized into a visual vocabulary. 

[[Part-Based Models]] represent objects by their constituent parts and their geometric relationships. A classic example is [[Active Appearance Models]], which model both shape and texture variations. These models are often used in face recognition to align faces into a canonical pose.

### Deep Learning Revolution

Modern systems rely heavily on [[Deep Neural Networks]]. For instance, the [[DeepFace]] system uses deep convolutional networks for both detection and classification, achieving significant gains in accuracy. Deep networks have also become the primary driver for the success of modern [[Object Detection]] systems.

### Evaluation Metrics

To evaluate [[Object Detection]], the [[Intersection Over Union]] (IoU) is used to measure the overlap between a predicted bounding box and the ground truth. 

$$ IoU(B_{pr}, B_{gt}) = \frac{Area(B_{pr} \cap B_{gt})}{Area(B_{pr} \ \cup B_{gt})} $$

This formula calculates the ratio of the intersection area to the union area of the two bounding boxes.

## Relationships
- [[Instance Recognition]] differs from [[Image Classification]]
- [[Image Classification]] can use [[Bag Of Words]]
- [[Image Classification]] can use [[Part-Based Models]]
- [[Image Classification]] is revolutionized by [[Deep Neural Networks]]
- [[Image Classification]] is a specialized form of [[Active Appearance Models]]
- [[Object Detection]] is revolutionized by [[Deep Neural Networks]]
- [[Object Detection]] uses [[Histogram Of Oriented Gradients]]
- [[Object Detection]] is evaluated by [[Intersection Over Union]]
- [[Part-Based Models]] are being replaced by [[Deep Neural Networks]]
- [[Part-Based Models]] is a type of [[Active Appearance Models]]
- [[Object Detection]] is a more general version of [[Instance Recognition]]
