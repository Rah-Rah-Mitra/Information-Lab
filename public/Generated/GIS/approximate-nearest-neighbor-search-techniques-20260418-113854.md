---
type: content
title: "Approximate Nearest Neighbor Search Techniques"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T11:38:54.923271600+00:00
summary: "An overview of graph-based and quantization-based methods for efficient high-dimensional vector similarity search in vector databases."
tags:
  - gis
  - computer-vision
  - vector-databases
  - approximate-nearest-neighbor
  - quantization
entities:
  - "[[Approximate Nearest Neighbor Search]]"
  - "[[Navigable Small World]]"
  - "[[Hierarchical Navigable Small World]]"
  - "[[Vector Quantization]]"
  - "[[Inverted File Index]]"
  - "[[Product Quantization]]"
  - "[[Optimized Product Quantization]]"
  - "[[Scalable Nearest Neighbor]]"
  - "[[Online Product Quantization]]"
  - "[[Inverted File Product Quantization]]"
  - "[[Voronoi Region]]"
  - "[[S2 Geometry Library]]"
relationships:
  - source: "Approximate Nearest Neighbor Search"
    target: "Navigable Small World"
    description: "is implemented by"
  - source: "Approximate Nearest Neighbor Search"
    target: "Vector Quantization"
    description: "is implemented by"
  - source: "Hierarchical Navigable Small World"
    target: "Navigable Small World"
    description: "generalizes"
  - source: "Inverted File Index"
    target: "Voronoi Region"
    description: "uses"
  - source: "Inverted File Product Quantization"
    target: "Inverted File Index"
    description: "combines"
  - source: "Inverted File Product Quantization"
    target: "Product Quantization"
    description: "combines"
  - source: "Optimized Product Quantization"
    target: "Product Quantization"
    description: "is a variation of"
  - source: "Online Product Quantization"
    target: "Product Quantization"
    description: "is a variation of"
  - source: "Scalable Nearest Neighbor"
    target: "Optimized Product Quantization"
    description: "optimizes"
  - source: "Scalable Nearest Neighbor"
    target: "Hierarchical Navigable Small World"
    description: "integrates"
  - source: "S2 Geometry Library"
    target: "Approximate Nearest Neighbor Search"
    description: "supports"
---

# Approximate Nearest Neighbor Search Techniques

*An overview of graph-based and quantization-based methods for efficient high-dimensional vector similarity search in vector databases.*

[[Approximate Nearest Neighbor Search]] (ANNS) is a set of techniques used to reduce the computational complexity of finding the most similar vectors in high-dimensional spaces, which is essential for the performance of vector databases.

## Graph-Based Methods

Graph-based methods build a network of vectors where edges represent similarity. [[Navigable Small World]] (NSW) creates a graph by connecting vectors to nearest neighbors and adding random long-range links to create shortcuts for faster traversal using a greedy heuristic.

[[Hierarchical Navigable Small World]] (HNSW) improves upon NSW by adding multiple layers of graphs with different scales and densities. Search starts at the highest layer (coarsest) and moves downward, allowing the algorithm to "jump over" large portions of irrelevant data.

## Quantization-Based Methods

[[Vector Quantization]] maps high-dimensional vectors to a low-precision representation using a finite set of codewords. This process involves an encoder that maps an input vector to the nearest codeword in a codebook, and a decoder that maps the index back to the codeword.

### Partitioning and Compression

[[Inverted File Index]] (IVF) enhances search efficiency by partitioning the vector space into [[Voronoi Region]]s using clustering (e.g., K-means). Search is then restricted to the closest regions to the query vector.

[[Product Quantization]] (PQ) compresses vectors by dividing them into sub-vectors and applying clustering to each sub-vector independently. This results in a compact code consisting of centroid indices.

### Advanced Quantization Variations

- [[Optimized Product Quantization]] (OPQ) reduces quantization distortion by optimizing the space decomposition and codebooks, often using random orthogonal rotations.
- [[Online Product Quantization]] (O-PQ) allows for the dynamic update of codebooks and codes in real-time to handle data streams without offline retraining.
- [[Scalable Nearest Neighbor]] (ScaNN) optimizes OPQ and integrates it with HNSW or PQN, utilizing anisotropic loss functions to better fit the local structure of the data.

### Hybrid Approaches

[[Inverted File Product Quantization]] (IVFPQ) combines the partitioning capabilities of IVF with the compression of PQ. In some implementations, such as FAISS, PQ is applied to the residuals (the difference between a point and its cluster center) rather than the original vectors to reduce approximation errors.

## Geospatial Indexing

The [[S2 Geometry Library]] provides a hierarchical geospatial index that partitions the Earth into cells, preserving spatial locality and enabling efficient geometric operations, which supports the underlying infrastructure for geospatial vector search.

## Relationships

- Approximate Nearest Neighbor Search is implemented by Navigable Small World
- Approximate Nearest Neighbor Search is implemented by Vector Quantization
- Hierarchical Navigable Small World generalizes Navigable Small World
- Inverted File Index uses Voronoi Region
- Inverted File Product Quantization combines Inverted File Index
- Inverted File Product Quantization combines Product Quantization
- Optimized Product Quantization is a variation of Product Quantization
- Online Product Quantization is a variation of Product Quantization
- Scalable Nearest Neighbor optimizes Optimized Product Quantization
- Scalable Nearest Neighbor integrates Hierarchical Navigable Small World
- S2 Geometry Library supports Approximate Nearest Neighbor Search
