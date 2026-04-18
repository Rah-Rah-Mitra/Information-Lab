---
type: content
title: "S2Vec Geospatial Embedding Framework"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T11:39:29.227049300+00:00
summary: "A self-supervised framework using S2 Geometry and Masked Autoencoding to learn task-agnostic embeddings of the built environment."
tags:
  - gis
  - geoai
  - self-supervised-learning
  - embeddings
  - computer-vision
entities:
  - "[[S2Vec]]"
  - "[[S2 Geometry Library]]"
  - "[[Masked Autoencoding]]"
  - "[[Built Environment]]"
  - "[[Vision Transformer]]"
  - "[[Geospatial Embeddings]]"
  - "[[Multimodal Fusion]]"
  - "[[S2 Cell]]"
  - "[[Feature Vector]]"
  - "[[SATCLIP]]"
  - "[[GEOCLIP]]"
  - "[[RS-MaMMUT]]"
relationships:
  - source: "S2Vec"
    target: "S2 Geometry Library"
    description: "leverages for spatial partitioning"
  - source: "S2Vec"
    target: "Masked Autoencoding"
    description: "uses for learning representations"
  - source: "S2Vec"
    target: "Geospatial Embeddings"
    description: "generates"
  - source: "S2Vec"
    target: "Built Environment"
    description: "encodes features of"
  - source: "Masked Autoencoding"
    target: "Vision Transformer"
    description: "is based on"
  - source: "S2Vec"
    target: "S2 Cell"
    description: "partitions geographic areas into"
  - source: "S2 Cell"
    target: "Feature Vector"
    description: "is represented by a"
  - source: "S2Vec"
    target: "Multimodal Fusion"
    description: "can be combined via"
  - source: "Multimodal Fusion"
    target: "SATCLIP"
    description: "integrates with"
  - source: "Multimodal Fusion"
    target: "GEOCLIP"
    description: "integrates with"
  - source: "Multimodal Fusion"
    target: "RS-MaMMUT"
    description: "integrates with"
---

# S2Vec Geospatial Embedding Framework

*A self-supervised framework using S2 Geometry and Masked Autoencoding to learn task-agnostic embeddings of the built environment.*

S2Vec is a scalable self-supervised framework designed to learn geospatial embeddings that capture the characteristics of the built environment. It transforms geographic data into a format suitable for computer vision architectures by leveraging the [[S2 Geometry Library]] to partition landmasses into hierarchical [[S2 Cell]] units.

## Methodology

### Feature Extraction and Rasterization
Each [[S2 Cell]] is associated with a [[Feature Vector]] consisting of a histogram of counts for place-of-interest categories and road network features. To apply image-based learning, S2Vec organizes these cells into patches and rasterizes them into images, where each pixel represents a feature vector of a child cell. This process preserves spatial continuity and local context.

### Learning with Masked Autoencoding
S2Vec employs [[Masked Autoencoding]] (MAE), a self-supervised strategy based on the [[Vision Transformer]]. The process involves:
- **Random Masking**: A subset of patches is randomly removed from the image.
- **Encoding**: A transformer encoder processes the remaining unmasked patches.
- **Decoding**: A decoder predicts the missing patches based on the encoded context.
- **Self-Supervision**: The model is trained to minimize the difference between original and reconstructed patches.

After training, the patch encoder is used to generate the final [[Geospatial Embeddings]] for individual cells.

## Multimodal Integration

To enhance performance, S2Vec embeddings can be combined with image-based embeddings from other models such as [[SATCLIP]], [[GEOCLIP]], or [[RS-MaMMUT]] through [[Multimodal Fusion]]. Three fusion strategies are explored: concatenation, weighted addition, and projection and addition (proj-add), with the latter generally performing best.

## Evaluation and Results

S2Vec demonstrates competitive performance in predicting socioeconomic indicators like housing prices, population density, and median income. It is particularly effective in zero-shot geographic adaptation, where it outperforms several image-only models when tested on unseen regions, suggesting that encoding the [[Built Environment]] features provides a more transferable representation than raw imagery alone.

## Relationships
- S2Vec leverages [[S2 Geometry Library]] for spatial partitioning.
- S2Vec uses [[Masked Autoencoding]] for learning representations.
- S2Vec generates [[Geospatial Embeddings]].
- S2Vec encodes features of the [[Built Environment]].
- [[Masked Autoencoding]] is based on the [[Vision Transformer]].
- S2Vec partitions geographic areas into [[S2 Cell]] units.
- [[S2 Cell]] is represented by a [[Feature Vector]].
- S2Vec can be combined via [[Multimodal Fusion]] with [[SATCLIP]], [[GEOCLIP]], and [[RS-MaMMUT]].
