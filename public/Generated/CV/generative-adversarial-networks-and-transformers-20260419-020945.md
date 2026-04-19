---
type: content
title: "Generative Adversarial Networks and Transformers"
source: "CV"
index_parent: "Sources/CV.md"
created: 2026-04-19T02:09:45.673015500+00:00
summary: "Generative Adversarial Networks (GANs) and Transformers are powerful deep learning architectures used for image synthesis and sequence modeling. GANs utilize a competitive game between a generator and a discriminator to produce realistic images, while Transformers leverage self-attention to capture global context. Both represent significant shifts from traditional convolutional inductive biases toward more flexible, data-driven representations."
tags:
  - deep-learning
  - generative-models
  - transformers
  - computer-vision
entities:
  - "[[Generative Adversarial Networks]]"
  - "[[Transformer]]"
  - "[[Self-Attention]]"
  - "[[Vision Transformer]]"
  - "[[Discriminator]]"
  - "[[Generator]]"
  - "[[Convolutional Neural Networks]]"
  - "[[Variational Autoencoders]]"
  - "[[Image-to-Image Translation]]"
  - "[[Cycle-Consistent Adversarial Network]]"
relationships:
  - source: "Generative Adversarial Networks"
    target: "Generator"
    description: "comprises"
  - source: "Generative Adversarial Networks"
    target: "Discriminator"
    description: "comprises"
  - source: "Generative Adversarial Networks"
    target: "Image-to-Image Translation"
    description: "enables"
  - source: "Transformer"
    target: "Self-Attention"
    description: "uses"
  - source: "Transformer"
    target: "Vision Transformer"
    description: "specialises to"
  - source: "Vision Transformer"
    target: "Convolutional Neural Networks"
    description: "competes with"
  - source: "Variational Autoencoders"
    target: "Generative Adversarial Networks"
    description: "is a different type of"
  - source: "Image-to-Image Translation"
    target: "Cycle-Consistent Adversarial Network"
    description: "uses"
  - source: "Generator"
    target: "Discriminator"
    description: "competes with"
---

# Generative Adversarial Networks and Transformers

*Generative Adversarial Networks (GANs) and Transformers are powerful deep learning architectures used for image synthesis and sequence modeling. GANs utilize a competitive game between a generator and a discriminator to produce realistic images, while Transformers leverage self-attention to capture global context. Both represent significant shifts from traditional convolutional inductive biases toward more flexible, data-driven representations.*

This note explores the evolution of deep learning architectures from local convolutional biases to global attention-based models and generative adversarial frameworks.

## Concept

### Generative Adversarial Networks
[[Generative Adversarial Networks]] (GANs) are a class of generative models that learn through an adversarial game between two networks: a [[Generator]] and a [[Discriminator]]. The [[Generator]] aims to create synthetic images that are indistinguishable from real data, while the [[Discriminator]] attempts to classify them as real or fake. The joint loss function for a standard GAN is expressed as:

$$ \mathcal{L}_{GAN} = \log D(\mathbf{x}) + \log(1 - D(G(\mathbf{z}))) $$

This equation models the minimax game where the discriminator maximizes its ability to distinguish real from fake, and the generator minimizes the probability of the discriminator being correct.

### Transformers and Self-Attention
Unlike [[Convolutional Neural Networks]] (CNNs), which are constrained by spatial locality, the [[Transformer]] architecture relies on [[Self-Attention]] to attend to all parts of an input sequence simultaneously. In the context of vision, the [[Vision Transformer]] (ViT) applies this by dividing an image into patches, which are then flattened and processed as tokens. This allows the model to capture long-range dependencies that are often missed by the local receptive fields of convolutions.

Self-attention is computed as a weighted sum of values, where weights are derived from the pairwise distances between a query and a set of keys:

$$ \mathbf{y}_i = \text{softmax}\left(\frac{\mathbf{q}_i \mathbf{K}^T}{\sqrt{D}}ight) \mathbf{V} $$

This scaled dot-product attention mechanism ensures that the variance of the dot product does not increase with the embedding dimension $D$, preventing vanishing gradients.

### Generative Alternatives
[[Variational Autoencoders]] (VAEs) provide an alternative generative approach by learning a latent distribution rather than a single deterministic mapping. While VAEs are more stable, GANs are often preferred for high-fidelity photorealistic synthesis. Furthermore, [[Image-to-Image Translation]] tasks, such as those performed by [[Cycle-Consistent Adversarial Network]] (CycleGAN), allow for the mapping between different domains (e.g., from sketches to photos) without requiring paired training data.

## Relationships
- [[Generative Adversarial Networks]] comprises [[Generator]]
- [[Generative Adversarial Networks]] comprises [[Discriminator]]
- [[Generative Adversarial Networks]] enables [[Image-to-Image Translation]]
- [[Transformer]] uses [[Self-Attention]]
- [[Transformer]] specialises to [[Vision Transformer]]
- [[Vision Transformer]] competes with [[Convolutional Neural Networks]]
- [[Variational Autoencoders]] is a different type of [[Generative Adversarial Networks]]
- [[Image-to-Image Translation]] uses [[Cycle-Consistent Adversarial Network]]
- [[Generator]] competes with [[Discriminator]]
