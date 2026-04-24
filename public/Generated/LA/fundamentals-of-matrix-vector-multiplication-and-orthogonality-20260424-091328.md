---
type: content
title: "Fundamentals of Matrix-Vector Multiplication and Orthogonality"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:13:28.904588200+00:00
summary: "This note covers the fundamental operations of matrix-vector multiplication, the properties of the concept of range and nullspace, and the orthogonal decomposition of a vector. It provides the mathematical basis for more advanced numerical linear algebra algorithms. The focus is on interpreting matrix-vector products as linear combinations of columns."
tags:
  - linear-algebra
  - matrix-vector-multiplication
  - orthogonality
  - numerical-linear-algebra
entities:
  - "[[Matrix-Vector Multiplication]]"
  - "[[Range]]"
  - "[[Nullspace]]"
  - "[[Orthogonality]]"
  - "[[Unitary Matrix]]"
  - "[[Inner Product]]"
  - "[[Unitary Matrix]]"
  - "[[Column Space]]"
  - "[[Rank]]"
  - "[[Adjoint]]"
  - "[[Vandermonde Matrix]]"
  - "[[Singular Value Decomposition]]"
relationships:
  - source: "Matrix-Vector Multiplication"
    target: "Column Space"
    description: "defines"
  - source: "Matrix-Vector Multiplication"
    target: "Range"
    description: "produces"
  - source: "Matrix-Vector Multiplication"
    target: "Nullspace"
    description: "depends on"
  - source: "Orthogonality"
    target: "Unitary Matrix"
    description: "characterizes"
  - source: "Unitary Matrix"
    target: "Orthogonality"
    description: "possesses"
  - source: "Inner Product"
    target: "Orthogonality"
    description: "defines"
  - source: "Rank"
    target: "Column Space"
    description: "is the dimension of"
  - source: "Adjoint"
    target: "Inner Product"
    description: "relates to"
  - source: "Matrix-Vector Multiplication"
    target: "Rank"
    description: "relates to"
---

# Fundamentals of Matrix-Vector Multiplication and Orthogonality

*This note covers the fundamental operations of matrix-vector multiplication, the properties of the concept of range and nullspace, and the orthogonal decomposition of a vector. It provides the mathematical basis for more advanced numerical linear algebra algorithms. The focus is on interpreting matrix-vector products as linear combinations of columns.*

This note explores the fundamental mathematical structures of [[Matrix-Vector Multiplication]], [[Orthogonality]], and the properties of matrices and vectors in complex and real spaces. 

## Concept

At its core, [[Matrix-Vector Multiplication]] is more than a just a calculation; it is a statement about how a matrix acts on a vector. While the standard formula is 

$$ b_i = \sum_{j=1}^n a_{ij}x_j, i=1, \dots, m $$ 

this represents the m-dimensional vector $b$ as a linear combination of the columns of $A$. Specifically, if $a_i$ denotes the jth column of $A$, then 

$$ b = Ax = \sum_{j=1}^n x_j a_j $$ 

This interpretation is essential for understanding numerical algorithms. For example, the [[Vandermonde Matrix]] is a linear map from vectors of polynomial coefficients to sampled polynomial values. 

## Range and Nullspace

For a matrix $A$, the [[Range]] (also known as the [[Column Space]]) is the set of vectors that can be expressed as $Ax$ for some $x$. 

$$ \text{range}(A) = \span{a_1, \dots, a_n} $$ 

Conversely, the [[Nullspace]] of $A$, written $\text{null}(A)$, is the set of vectors $x$ such that $Ax = 0$. 

## Rank and Invertibility

The [[Rank]] of a matrix is the dimension of its [[Column Space]]. A matrix is of full rank if it has the maximal possible rank. A square nonsingular matrix is [[Invertible]], meaning it has a unique inverse $A^{-1}$ such that $AA^{-1} = A^{-1}A = I$. 

Multiplication by $A^{-1}$ can be viewed as a continuous [[Change of Basis]] operation, where $A^{-1}b$ is the vector of coefficients for the expansion of $b$ in the basis of the columns of $A$. 

## Orthogonality and Inner Products

An [[Inner Product]] of two vectors $x$ and $y$ is defined as 

$$ x^*y = \sum_{j=1}^m x_i \bar{y}_i $$ 

Two vectors are [[Orthogonality]] if $x^*y = 0$. A set of vectors is [[Orthonormal]] if they are pairwise orthogonal and each has a unit length. 

[[Unitary Matrix]]s are square matrices $Q$ such that $Q^*Q = I$. The columns of a [[Unitary Matrix]] form an orthonormal basis for the space. 

## Relationships

- [[Matrix-Vector Multiplication]] defines [[Column Space]]
- [[Matrix-Vector Multiplication]] produces [[Range]]
- [[Matrix-Vector Multiplication]] depends on [[Nullspace]]
- [[Orthogonality]] characterizes [[Unitary Matrix]]
- [[Unitary Matrix]] possesses [[Orthogonality]]
- [[Inner Product]] relates to [[Orthogonality]]
- [[Rank]] is the dimension of [[Column Space]]
- [[Adjoint]] relates to [[Inner Product]]
- [[Matrix-Vector Multiplication]] relates to [[Rank]]
