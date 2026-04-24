---
type: content
title: "Graph Partitioning and Domain Decomposition"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:12:41.127729200+00:00
summary: "Graph partitioning techniques like Recursive Graph Bisection (RGB) and level set expansion methods are used to divide large networks into subdomains. These methods aim to minimize cut-edges while maintaining balanced domain sizes, which is critical for parallel processing and domain decomposition in numerical simulations. Heuristic improvements, such as using multiple centers for expansion, help create more compact, and more efficient subdomains."
tags:
  - graph-theory
  - domain-decomposition
  - parallel-computing
  - numerical-methods
  - graph-partitioning
entities:
  - "[[Graph Partitioning]]"
  - "[[Recursive Graph Bisection]]"
  - "[[Level Set Expansion]]"
  - "[[Domain Decomposition]]"
  - "[[Cut-Edges]]"
  - "[[Pseudo-Peripheral Node]]"
  - "[[Recursive Spectral Bisection]]"
relationships:
  - source: "Graph Partitioning"
    target: "Domain Decomposition"
    description: "enables"
  - source: "Recursive Graph Bisection"
    target: "Graph Partitioning"
    description: "is a type of"
  - source: "Level Set Expansion"
    target: "Graph Partitioning"
    description: "is a technique for"
  - source: "Recursive Spectral Bisection"
    target: "Graph Partitioning"
    description: "is a type of"
  - source: "Pseudo-Peripheral Node"
    target: "Level Set Expansion"
    description: "is used to find starting points for"
---

# Graph Partitioning and Domain Decomposition

*Graph partitioning techniques like Recursive Graph Bisection (RGB) and level set expansion methods are used to divide large networks into subdomains. These methods aim to minimize cut-edges while maintaining balanced domain sizes, which is critical for parallel processing and domain decomposition in numerical simulations. Heuristic improvements, such as using multiple centers for expansion, help create more compact, and more efficient subdomains.*

## Concept
[[Graph Partitioning]] is the process of dividing a graph into several subgraphs, often to facilitate parallel computing or to manage large-scale systems. The primary goal is to achieve a balance between two competing qualities: minimizing the number of [[Cut-Edges]] (the edges that connect different subdomains) and ensuring that the subdomains have roughly equal sizes.

In the context of [[Domain Decomposition]], these partitions serve as the subdomains for numerical solvers. Common heuristic approaches include:

- **[[Recursive Graph Bisection]] (RGB):** This method repeatedly divides a graph into two parts. A common approach involves finding a [[Pseudo-Peripheral Node]] to initiate a level set traversal, which helps define the boundaries of the subgraphs.
- **[[Level Set Expansion]]**: This method can improve upon simple bisection by using multiple expansion centers (or sites) to grow subdomains. This process can be used to smooth boundaries and create more compact, circular shapes, which reduces the number of cut-edges.

While simple bisection methods can result in elongated or twisted shapes, more advanced techniques like [[Recursive Spectral Bisection]] are noted for providing high-quality partitions, though they are more computationally expensive.

## Relationships
- [[Graph Partitioning]] enables [[Domain Decomposition]]
- [[Recursive Graph Bisection]] is a type of [[Graph Partitioning]]
- [[Level Set Expansion]] is a technique for [[Graph Partitioning]]
- [[Recursive Spectral Bisection]] is a type of [[Graph Partitioning]]
- [[Pseudo-Peripheral Node]] is used to find starting points for [[Level Set Expansion]]
