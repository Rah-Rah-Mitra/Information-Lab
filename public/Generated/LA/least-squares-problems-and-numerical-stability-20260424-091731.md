---
type: content
title: "Least Squares Problems and Numerical Stability"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:17:31.456621900+00:00
summary: "Least squares problems aim to minimize the norm of the residual in linear systems, often solved via normal equations, QR factorization, or SVD. The accuracy of these solutions depends heavily on the condition number of the matrix and the problem's sensitivity to perturbations. Numerical stability is further influenced by the relative errors introduced by floating point arithmetic."
tags:
  - linear-algebra
  - numerical-analysis
  - least-squares
  - conditioning
  - floating-point
entities:
  - "[[Least Squares Problem]]"
  - "[[Normal Equations]]"
  - "[[QR Factorization]]"
  - "[[Singular Value Decomposition]]"
  - "[[Condition Number]]"
  - "[[Pseudoinverse]]"
  - "[[Floating Point Arithmetic]]"
  - "[[Machine Epsilon]]"
relationships:
  - source: "Least Squares Problem"
    target: "Normal Equations"
    description: "can be solved via"
  - source: "Least Squares Problem"
    target: "QR Factorization"
    description: "is solved via"
  - source: "Least Squares Problem"
    target: "Singular Value Decomposition"
    description: "is solved via"
  - source: "Least Squares Problem"
    target: "Pseudoinverse"
    description: "uses"
  - source: "Condition Number"
    target: "Least Squares Problem"
    description: "measures sensitivity of"
  - source: "Floating Point Arithmetic"
    target: "Condition Number"
    description: "introduces error in"
  - source: "Machine Epsilon"
    target: "Floating Point Arithmetic"
    description: "characterizes resolution of"
---

# Least Squares Problems and Numerical Stability

*Least squares problems aim to minimize the norm of the residual in linear systems, often solved via normal equations, QR factorization, or SVD. The accuracy of these solutions depends heavily on the condition number of the matrix and the problem's sensitivity to perturbations. Numerical stability is further influenced by the relative errors introduced by floating point arithmetic.*

This note explores the mathematical formulation and numerical stability of least squares problems and the condition of mathematical problems. 

## Concept

A [[Least Squares Problem]] is defined as finding a vector $x$ that minimizes the norm of the residual $r = b - Ax$, typically in the 2-norm. For a full-rank matrix $A \to C^{m \times n}$ with $m \ge n$, the solution is unique and can be expressed using the [[Pseudoinverse]] $A^+ = (A^*A)^{-1}A^*$. 

Several algorithms are used to solve these problems:

1. **Normal Equations**: Solving $A^*Ax = A^*b$. This method is efficient but can be be numerically unstable if $A$ is ill-conditioned. 
2. **[[QR Factorization]]**: Decomposing $A = QR$ to solve $Rx = Q^*b$. This is the "modern classical" method and is more stable than the normal equations.
3. **[[Singular Value Decomposition]] (SVD)**: Decomposing $A = U\text{E}V^*$ to solve the diagonal system. This is the most robust method, especially when $A$ is close to rank-deficient.

## Conditioning and Stability

[[Condition Number]] is a measure of how sensitive a problem is to small perturbations in the input data. For a matrix $A$, the relative condition number is defined as: 

$$ \text{cond}(A) = \|A\| \|A^{-1}\\| $$ 

This value determines how many digits of accuracy are lost during computation. An [[ill-conditioned]] problem is one where the condition number is large, meaning small changes in $x$ or $b$ can lead to large changes in the solution $x$. 

[[Floating Point Arithmetic]] is the system used by computers to represent real numbers using a finite number of bits. The precision of this system is characterized by [[Machine Epsilon]] $\epsilon_{\text{machine}}$, which represents the smallest relative error introduced by any elementary arithmetic operation. 

$$ \text{fl}(x) = x(1 + c), \text{ where } |c| < \epsilon_{\text{machine}} $$ 

This formula models the relationship between the real number $x$ and its closest floating point approximation. 

## Relationships
- [[Least Squares Problem]] can be solved via [[Normal Equations]], [[QR Factorization]], and [[Singular Value Decomposition]].
- [[Least Squares Problem]] uses [[Pseudoinverse]].
- [[Condition Number]] measures sensitivity of [[Least Squares Problem]].
- [[Floating Point Arithmetic]] introduces error in [[Least Squares Problem]].
- [[Machine Epsilon]] characterizes resolution of [[Floating Point Arithmetic]].
