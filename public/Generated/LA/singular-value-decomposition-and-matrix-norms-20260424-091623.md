---
type: content
title: "Singular Value Decomposition and Matrix Norms"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:16:23.838856+00:00
summary: "The Singular Value Decomposition (SVD) factorises a matrix into orthonormal bases and scaling factors, providing a geometric interpretation of linear maps. This note covers the relationship between matrix norms, vector norms, and the SVD, including the properties of invariant norms under unitary multiplication. It is a fundamental tool for understanding rank, range, and null space."
tags:
  - linear-algebra
  - matrix-decomposition
  - singular-value-decomposition
  - matrix-norms
entities:
  - "[[Singular Value Decomposition]]"
  - "[[Matrix Norms]]"
  - "[[Vector Norms]]"
  - "[[Unitary Matrix]]"
  - "[[Eigenvalue Decomposition]]"
  - "[[Frobenius Norm]]"
  - "[[Matrix 2-Norm]]"
  - "[[Hyperellipse]]"
  - "[[Left Singular Vectors]]"
  - "[[Right Singular Vectors]]"
  - "[[Singular Values]]"
relationships:
  - source: "Singular Value Decomposition"
    target: "Singular Values"
    description: "produces"
  - source: "Singular Value Decomposition"
    target: "Left Singular Vectors"
    description: "provides"
  - source: "Singular Value Decomposition"
    target: "Right Singular Vectors"
    description: "provides"
  - source: "Singular Value Decomposition"
    target: "Hyperellipse"
    description: "describes geometry of"
  - source: "Matrix Norms"
    target: "Vector Norms"
    description: "induced by"
  - source: "Matrix 2-Norm"
    target: "Singular Values"
    description: "equals maximum"
  - source: "Singular Value Decomposition"
    target: "Eigenvalue Decomposition"
    description: "contrasted with"
  - source: "Singular Value Decomposition"
    target: "Frobenius Norm"
    description: "relates to"
  - source: "Singular Value Decomposition"
    target: "Unitary Matrix"
    description: "uses"
  - source: "Singular Value Decomposition"
    target: "Hyperellipse"
    description: "describes geometry of"
---

# Singular Value Decomposition and Matrix Norms

*The Singular Value Decomposition (SVD) factorises a matrix into orthonormal bases and scaling factors, providing a geometric interpretation of linear maps. This note covers the relationship between matrix norms, vector norms, and the SVD, including the properties of invariant norms under unitary multiplication. It is a fundamental tool for understanding rank, range, and null space.*

This note explores the fundamental decomposition of a matrix and the various ways to measure its size through norms. 

## Concept

The [[Singular Value Decomposition]] (SVD) is a matrix factorization that decomposes an arbitrary $m \times n$ matrix $A$ into three components: a unitary matrix $U$, a diagonal matrix $\Sigma$ (often denoted as $E$ in this text), and a unitary matrix $V^*$ (the adjoint of $V$). The full SVD is expressed as:

$$ A = U E V^* $$

where $U \in \mathbb{C}^{m \times m}$ and $V \in \mathbb{C}^{n \times n}$ are unitary, and $E$ is an $m \times n$ diagonal matrix with non-negative entries $\sigma_1 \ge \ \sigma_2 \ge \dots \ge \ \sigma_p \ge  0$ in non-increasing order. These diagonal entries, $\sigma_i$, are the [[Singular Values]].

Geometrically, the SVD reveals that the image of the unit sphere under a linear map $A$ is a [[Hyperellipse]]. The [[Left Singular Vectors]] (the columns of $U$) and [[Right Singular Vectors]] (the columns of $V$) provide the orthonormal bases for the range and domain spaces, respectively. Specifically, the mapping $A$ acts on the right singular vectors $v_j$ such that $A v_j = \sigma_j u_j$.

## Matrix Norms

[[Matrix Norms]] are measures of the "size" of a matrix, which can be can be categorized into induced norms and general matrix norms. 

### Induced Matrix Norms

An induced matrix norm is defined by the how a matrix acts as an operator between normed domain and range spaces. Given vector norms $\| \cdot \|_{(n)}$ and $\| \cdot \|_{(m)}$, the induced norm $\| A \|_{(m,n)}$ is the smallest number $C$ such that:

$$ \| A x \|_{(m)} \le C \| x \|_{(n)} \ {\textstyle \sup_{x \neq 0} \frac{\| A x \|_{(m)}}{\| x \|_{(n)}} } $$

This represents the maximum factor by which $A$ can "stretch" a vector. For the most common case, the [[Matrix 2-Norm]], the value is equal to the largest singular value:

$$ \| A \|_2 = \sigma_1 $$

This norm is invariant under multiplication by a [[Unitary Matrix]] $Q$, such that $\| Q A \|_2 = \ \| A \|_2$ and $\| A Q \|_2 = \ \| A \|_2$.

### General Matrix Norms

Some matrix norms are not induced by vector norms. The most prominent is the [[Frobenius Norm]], defined as:

$$ \| A \|_F = \sqrt{\sum_{i=1}^m \sum_{j=1}^n |a_{ij}|^2} = \sqrt{\text{tr}(A^* A)} $$

This norm is also invariant under unitary multiplication. 

## Relationships

- [[Singular Value Decomposition]] produces [[Singular Values]].
- [[Singular Value Decomposition]] provides [[Left Singular Vectors]] and [[Right Singular Vectors]].
- [[Singular Value Decomposition]] describes the geometry of a [[Hyperellipse]].
- [[Matrix Norms]] are induced by [[Vector Norms]].
- [[Matrix 2-Norm]] equals the maximum of [[Singular Values]].
- [[Singular Value Decomposition]] is contrasted with [[Eigenvalue Decomposition]].
- [[Singular Value Decomposition]] relates to the [[Frobenius Norm]].
- [[Singular Value Decomposition]] uses [[Unitary Matrix]].
- [[Singular Value Decomposition]] describes the geometry of a [[Hyperellipse]].
