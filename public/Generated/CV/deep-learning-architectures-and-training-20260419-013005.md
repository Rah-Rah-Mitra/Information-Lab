---
type: content
title: "Deep Learning Architectures and Training"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T01:30:05.633135300+00:00
summary: "This note covers the fundamental components of deep neural networks, including convolutional layers, recurrent structures, and optimization strategies. It explains how backpropagation and stochastic gradient descent enable training, while also discussing modern advancements like transformers and self-supervised learning. The content provides a technical overview of the mechanisms used to achieve high performance in computer vision tasks."
tags:
  - computer-vision
  - deep-learning
  - neural-networks
  - optimization
  - convolutional-neural-networks
entities:
  - "[[Deep Neural Networks]]"
  - "[[Backpropagation]]"
  - "[[Stochastic Gradient Descent]]"
  - "[[Convolutional Neural Networks]]"
  - "[[Recurrent Neural Networks]]"
  - "[[Self-Supervised Learning]]"
  - "[[Transformers]]"
  - "[[Weight Initialization]]"
  - "[[Adam Optimizer]]"
  - "[[Residual Networks]]"
relationships:
  - source: "Deep Neural Networks"
    target: "Backpropagation"
    description: "trained via"
  - source: "Deep Neural Networks"
    target: "Stochastic Gradient Descent"
    description: "optimized using"
  - source: "Deep Neural Networks"
    target: "Convolutional Neural Networks"
    description: "includes"
  - source: "Deep Neural Networks"
    target: "Recurrent Neural Networks"
    description: "includes"
  - source: "Convolutional Neural Networks"
    target: "Residual Networks"
    description: "specialises to"
  - source: "Convolutional Neural Networks"
    target: "Adam Optimizer"
    description: "uses"
  - source: "Backpropagation"
    target: "Stochastic Gradient Descent"
    description: "provides gradients for"
  - source: "Self-Supervised Learning"
    target: "Deep Neural Networks"
    description: "pre-trains"
  - source: "Self-Supervised Learning self-supervised learning"
    target: "Deep Neural Networks"
    description: "pre-trains"
  - source: "Transformers"
    target: "Deep Neural Networks"
    description: "is a type of"
  - source: "Recurrent Neural Networks"
    target: "Transformers"
    description: "preceded by"
  - source: "Weight Initialization"
    target: "Deep Neural Networks"
    description: "required for"
  - source: "Adam Optimizer"
    target: "Deep Neural Networks"
    description: "optimizes"
  - source: "Residual Networks"
    target: "Convolutional Neural Networks"
    description: "is a variant of"
---

# Deep Learning Architectures and Training

*This note covers the fundamental components of deep neural networks, including convolutional layers, recurrent structures, and optimization strategies. It explains how backpropagation and stochastic gradient descent enable training, while also discussing modern advancements like transformers and self-supervised learning. The content provides a technical overview of the mechanisms used to achieve high performance in computer vision tasks.*

This note provides a technical overview of the core mechanisms and architectures of [[Deep Neural Networks]].

## Concept
[[Deep Neural Networks]] are multi-layered computational structures designed to learn hierarchical representations from data. Their training relies on the fundamental process of [[Backpropagation]], which uses the chain rule to compute the derivatives of the loss function with respect to the weights and biases. This process is essential for iteratively updating parameters to minimize error.

### Weight Initialization
Before training begins, [[Weight Initialization]] is necessary to prevent vanishing or exploding gradients. For example, [[He Initialization]] is specifically designed for layers using the ReLU activation function to maintain stable activation variance across layers.

### Optimization
To update weights, practitioners use [[Stochastic Gradient Descent]] (SGD) and its variants. While standard gradient descent is computationally expensive for large datasets, [[Stochastic Gradient Descent]] uses minibatches to provide noisy but efficient updates. Advanced optimizers like the [[Adam Optimizer]] (which combines momentum and adaptive learning rates) are currently the most popular for deep learning.

## Convolutional Architectures
[[Convolutional Neural Networks]] (CNNs) are specialized for image processing, using trainable multi-layer convolutions that exploit local connectivity and weight sharing. 

### Residual Learning
To enable the training of much deeper networks, [[Residual Networks]] (ResNets) introduce skip connections that allow gradients to flow more easily through the layers. This innovation was crucial for the performance of the나머지

## Sequence Modeling
[[Recurrent Neural Networks]] (RNNs) are designed for sequential data, such as video or text, by passing information from one time step to the next via a hidden state. More advanced architectures like [[Transformers]] incorporate attention mechanisms to allow the parallel processing of entire sequences, which can be more efficient and powerful than traditional sequential models.

## Self-Supervised Learning
[[Self-Supervised Learning]] is a growing field where models are pre-trained on unlabeled data using pretext tasks (e.g., enough to predict a missing part of an image) to learn meaningful representations. This can then be followed by fine-tuning on a downstream task.

## Relationships
- [[Deep Neural Networks]] trained via [[Backpropagation]]
- [[Deep Neural Networks]] optimized using [[Stochastic Gradient Descent]]
- [[Convolutional Neural Networks]] includes [[Residual Networks]]
- [[Convolutional Neural Networks]] uses [[Adam Optimizer]]
- [[Backpropagation]] provides gradients for [[Stochastic Gradient Descent]]
- [[Self-Supervised Learning]] pre-trains [[Deep Neural Networks]]
- [[Transformers]] is a type of [[Deep Neural Networks]]
- [[Weight Initialization]] required for [[Deep Neural Networks]]
- [[Adam Optimizer]] optimizes [[Deep Neural Networks]]
- [[Residual Networks]] is a variant of [[Convolutional Neural Networks]]
