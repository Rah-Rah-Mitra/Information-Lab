---
type: content
title: "Machine Learning Paradigms and Techniques"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:29:40.471387700+00:00
summary: "This note covers the fundamental paradigms of supervised, unsupervised, and semi-supervised learning, alongside core techniques like SVMs and deep neural networks. It explains how different learning objectives, such as classification and regression, are achieved through various optimization and regularization strategies. The text highlights the importance of loss functions and normalization in training robust models."
tags:
  - machine-learning
  - computer-vision
  - supervised-learning
  - unsupervised-learning
  - deep-learning
entities:
  - "[[Supervised Learning]]"
  - "[[Unsupervised Learning]]"
  - "[[Semi-supervised Learning]]"
  - "[[Classification]]"
  - "[[Regression]]"
  - "[[Support Vector Machines]]"
  - "[[Logistic Regression]]"
  - "[[Deep Neural Networks]]"
  - "[[Empirical Risk Minimization]]"
  - "[[Loss Function]]"
  - "[[Regularization]]"
  - "[[Batch Normalization]]"
  - "[[Principal Component Analysis]]"
  - "[[K-means]]"
  - "[[Decision Trees]]"
  - "[[Manifold Learning]]"
relationships:
  - source: "Supervised Learning"
    target: "Classification"
    description: "includes"
  - source: "Supervised Learning"
    target: "Regression"
    description: "includes"
  - source: "Supervised Learning"
    target: "Logistic Regression"
    description: "uses"
  - source: "Supervised Learning"
    target: "Support Vector Machines"
    description: "uses"
  - source: "Supervised Learning"
    target: "Decision Trees"
    description: "uses"
  - source: "Unsupervised Learning"
    target: "Principal Component Analysis"
    description: "uses"
  - source: "Unsupervised Learning"
    target: "K-means"
    description: "uses"
  - source: "Unsupervised Learning"
    target: "Manifold Learning"
    description: "uses"
  - source: "Deep Neural Networks"
    target: "Regularization"
    description: "requires"
  - source: "Deep Neural Networks"
    target: "Batch Normalization"
    description: "uses"
  - source: "Deep Neural Networks"
    target: "Loss Function"
    description: "optimizes"
  - source: "Supervised Learning"
    target: "Empirical Risk Minimization"
    description: "relies on"
  - source: "Semi-supervised Learning"
    target: "Supervised Learning"
    description: "is a subset of"
  - source: "Support Vector Machines"
    target: "Regularization"
    description: "uses"
  - source: "Logistic Regression"
    target: "Loss Function"
    description: "uses"
---

# Machine Learning Paradigms and Techniques

*This note covers the fundamental paradigms of supervised, unsupervised, and semi-supervised learning, alongside core techniques like SVMs and deep neural networks. It explains how different learning objectives, such as classification and regression, are achieved through various optimization and regularization strategies. The text highlights the importance of loss functions and normalization in training robust models.*

This note provides an overview of the primary machine learning paradigms and the specific algorithms used to solve classification and regression tasks.

## Concept

Machine learning is broadly categorized into three main paradigms based on the availability of labels: [[Supervised Learning]], [[Unsupervised Learning]], and [[Semi-supervised Learning]].

### Supervised Learning
[[Supervised Learning]] involves feeding pairs of (input, label) into an algorithm to estimate model parameters that maximize agreement with the target outputs. This can be split into two main tasks:
- [[Classification]]: Predicting discrete class membership from a set of classes.
- [[Regression]]: Predicting continuous, potentially vector-valued targets.

Common supervised techniques include:
- [[Logistic Regression]]: A discriminative model that uses a linear projection followed by a logistic sigmoid function to obtain probabilities.
- [[Support Vector Machines]]: A maximum margin classifier that finds a decision surface to maximize the distance to the nearest training examples (support vectors).
- [[Decision Trees]]: A sequence of simple operations that split training samples into more specific distributions.

To optimize these models, we use [[Empirical Risk Minimization]], where the expected risk is estimated using training data. The objective is to minimize a [[Loss Function]], such as the cross-entropy loss for classification or the squared error for regression.

$$ E(\mathbf{w}, b) = \sum_{i} t_i \log p(x_i) $$ 

This formula represents the multi-class cross-entropy loss used in neural networks.

### Unsupervised Learning
[[Unsupervised Learning]] aims to characterize data patterns without explicit labels. Key techniques include:
- [[K-means]]: An iterative algorithm that assigns samples to the nearest cluster center.
- [[Principal Component Analysis]]: A dimensionality reduction technique that uses eigenvalue decomposition of the covariance matrix to find a lower-rank approximation of data.
- [[Manifold Learning]]: A non-linear dimensionality reduction technique used when data resides on a little-dimensional manifold rather than a global linear subspace.

### Deep Neural Networks
[[Deep Neural Networks]] are feedforward computation graphs of interconnected units (neurons) that learn internal representations end-to-end. They rely heavily on [[Regularization]] (such as [[Dropout]] or [[Weight Decay]]) and [[Batch Normalization]] to prevent overfitting and improve convergence.

$$ y = 	ext{softmax}(	ext{score}) $$ 

This formula shows the conversion of real-valued activations to class likelihoods via the softmax function.

## Relationships
- [[Supervised Learning]] includes [[Classification]] and [[Regression]]
- [[Supervised Learning]] uses [[Logistic Regression]], [[Support Vector Machines]], and [[Decision Trees]]
- [[Unsupervised Learning]] uses [[Principal Component Analysis]], [[K-means]], and [[Manifold Learning]]
- [[Deep Neural Networks]] requires [[Regularization]] and [[Batch Normalization]]
- [[Deep Neural Networks]] optimizes [[Loss Function]]
