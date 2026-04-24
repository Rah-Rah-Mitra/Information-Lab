---
type: content
title: "Stability and Conditioning of Least Squares"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:24:26.648312500+00:00
summary: "This note explores the backward stability of Householder triangularization and the sensitivity of least squares solutions to perturbations in data. It distinguishes between algorithm instability and algorithm ill-conditioning, demonstrating how condition numbers govern the error amplification. The analysis provides quantitative bounds for the sensitivity of the solution vector and the projection."
tags:
  - numerical-linear-algebra
  - least-squares
  - stability-analysis
  - conditioning
entities:
  - "[[Least Squares Problem]]"
  - "[[Backward Stability]]"
  - "[[Householder Triangularization]]"
  - "[[Condition Number]]"
  - "[[Conditioning of Least Squares]]"
  - "[[QR Factorization]]"
  - "[[Back Substitution]]"
  - "[[Orthogonal Projector]]"
relationships:
  - source: "Householder Triangularization"
    target: "QR Factorization"
    description: "is a method for"
  - source: "QR Factorization"
    target: "Least Squares Problem"
    description: "solves"
  - source: "Back Substitution"
    target: "Least Squares Problem"
    description: "is used in solving"
  - source: "Condition Number"
    target: "Least Squares Problem"
    description: "measures sensitivity of"
  - source: "Conditioning of Least Squares"
    target: "Least Squares Problem"
    description: "analyzes"
  - source: "Householder Triangularization"
    target: "Backward Stability"
    description: "is shown to be"
  - source: "Back Substitution"
    target: "Backward Stability"
    description: "is shown to be"
  - source: "Orthogonal Projector"
    target: "Least Squares Problem"
    description: "characterizes the projection in"
---

# Stability and Conditioning of Least Squares

*This note explores the backward stability of Householder triangularization and the sensitivity of least squares solutions to perturbations in data. It distinguishes between algorithm instability and algorithm ill-conditioning, demonstrating how condition numbers govern the error amplification. The analysis provides quantitative bounds for the sensitivity of the solution vector and the projection.*

This note examines the numerical stability and conditioning of the least squares problem, specifically focusing on the Householder triangularization method and the stability of back substitution.

## Concept
A [[Least Squares Problem]] is defined as finding a vector $x \in \mathbb{C}^n$ that minimizes the norm of the residual, $||b - Ax||$, for a given $A \in \mathbb{C}^{m \times n}$ and $b \in \mathbb{C}^m$. The sensitivity of this problem to perturbations in the matrix $A$ and the vector $b$ is characterized by the [[Condition Number]] of $A$, denoted $K(A)$. 

In the context of solving via [[QR Factorization]], where $A = QR$, the process involves [[Householder Triangularization]], which is a [[Backward Stability]] property of the algorithm. An algorithm is [[Backward Stability]] if the computed result is the exact solution to a slightly perturbed problem. For example, [[Back Substitution]] is shown to be backward stable, meaning the computed solution $x$ satisfies $(R + \delta R)x = b$ for some small $\delta R$ such that $|\delta R| \le M \epsilon_{\text{machine}} |R|$.

## Stability and Conditioning

### Householder Triangularization
In the context of the [[Least Squares Problem]], [[Householder Triangularization]] is a standard method for computing the QR factorization. The text demonstrates that while thes solution might appear inaccurate due to high condition numbers, the inaccuracy is often a result of ill-conditioning rather than instability. 

$$ \min_{x \in \\mathbb{C}^n} ||b - Ax|| $$

This equation models the minimization of the residual norm in the least squares problem.

### Conditioning of Least Squares
The [[Conditioning of Least Squares]] refers to the sensitivity of the analysis of the solution $x$ and the orthogonal projection $y = Ax$. The sensitivity of $y$ to perturbations in $b$ is governed by $1/\cos \theta$, where $\theta$ is the angle between $b$ and the range of $A$. The the sensitivity of $x$ to $x$ is more complex and involves both the condition number $K(A)$ and the term $\tan \theta$. 

Theorem 18.1 provides the following bounds for the relative condition numbers of $y$ and $x$ in the 2-norm:

$$ \kappa_b(y) = \frac{1}{\cos \theta}, \quad \kappa_b(x) = \frac{K(A)}{\cos \theta} $$ 

These formulas describe the sensitivity of the projection $y$ and the solution vector $x$ with respect to perturbations in the vector $b$.

For perturbations in $A$, the sensitivity of $x$ is bounded by:

$$ \kappa_A(x) \approx K(A) + K(A)^2 \tan \theta $$

This bound shows that how much the error is amplified by the term $K(A)^2 \tan \theta$ depends on the heavily ill-conditioned basis, and the sensitivity of the increases with the angle $\theta$.

## Relationships
- [[Householder Triangularization]] is a method for [[QR Factorization]]
- [[QR Factorization]] is used to solve [[Least Squares Problem]]
- [[Back Substitution]] is shown to be [[Backward Stability]]
- [[Condition Number]] measures sensitivity of [[Least Squares Problem]]
- [[Conditioning of Least Squares]] analyzes [[Least Squares Problem]]
- [[Householder Triangularization]] is shown to be [[Backward Stability]]
- [[Back Substitution]] is shown to be [[Backward Stability]]
- [[Orthogonal Projector]] characterizes the projection in [[Least Squares Problem]]
