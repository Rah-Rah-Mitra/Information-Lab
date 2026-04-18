---
type: content
title: "Neural Networks and Deep Learning"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:16:35.866682800+00:00
summary: "An overview of neural network architectures, from the basic perceptron and its training algorithms to multilayer feedforward networks and convolutional neural networks."
tags:
  - computer-vision
  - deep-learning
  - neural-networks
  - pattern-recognition
entities:
  - "[[Perceptron]]"
  - "[[Linear Decision Function]]"
  - "[[Hyperplane]]"
  - "[[Perceptron Training Algorithm]]"
  - "[[Perceptron Convergence Theorem]]"
  - "[[Least-Mean-Squared-Error Algorithm]]"
  - "[[XOR Classification Problem]]"
  - "[[Artificial Neuron]]"
  - "[[Activation Function]]"
  - "[[Sigmoid Function]]"
  - "[[Rectifier Linear Unit]]"
  - "[[Feedforward Neural Network]]"
  - "[[Hidden Layer]]"
  - "[[Deep Neural Network]]"
  - "[[Backpropagation]]"
  - "[[Convolutional Neural Network]]"
  - "[[LeNet Architecture]]"
relationships:
  - source: "Perceptron"
    target: "Linear Decision Function"
    description: "implements a"
  - source: "Linear Decision Function"
    target: "Hyperplane"
    description: "is defined by a"
  - source: "Perceptron Training Algorithm"
    target: "Perceptron"
    description: "trains a"
  - source: "Perceptron Convergence Theorem"
    target: "Perceptron Training Algorithm"
    description: "guarantees convergence of"
  - source: "Least-Mean-Squared-Error Algorithm"
    target: "Perceptron"
    description: "provides an alternative training method for"
  - source: "XOR Classification Problem"
    target: "Perceptron"
    description: "demonstrates the limitations of a single"
  - source: "Artificial Neuron"
    target: "Activation Function"
    description: "uses an"
  - source: "Feedforward Neural Network"
    target: "Artificial Neuron"
    description: "is composed of"
  - source: "Feedforward Neural Network"
    target: "Hidden Layer"
    description: "contains"
  - source: "Deep Neural Network"
    target: "Feedforward Neural Network"
    description: "is a type of"
  - source: "Backpropagation"
    target: "Feedforward Neural Network"
    description: "is used to train"
  - source: "Convolutional Neural Network"
    target: "Feedforward Neural Network"
    description: "is a specialized type of"
  - source: "Convolutional Neural Network"
    target: "LeNet Architecture"
    description: "is exemplified by the"
  - source: "Sigmoid Function"
    target: "Activation Function"
    description: "is a type of"
  - source: "Rectifier Linear Unit"
    target: "Activation Function"
    description: "is a type of"
---

# Neural Networks and Deep Learning

*An overview of neural network architectures, from the basic perceptron and its training algorithms to multilayer feedforward networks and convolutional neural networks.*

Neural networks are computing systems inspired by biological neurons, designed to recognize patterns and classify data through layers of interconnected processing units.

## The Perceptron

A [[Perceptron]] is the simplest form of a neural network unit that learns a linear boundary between two linearly separable classes. It implements a [[Linear Decision Function]], which in n-dimensions is represented by a [[Hyperplane]].

### Training the Perceptron
The [[Perceptron Training Algorithm]] iteratively adjusts weights and bias to find a separating hyperplane. The [[Perceptron Convergence Theorem]] ensures that if the classes are linearly separable, the algorithm will converge in a finite number of steps. For non-separable classes, the [[Least-Mean-Squared-Error Algorithm]] (LMSE) can be used to minimize the mean squared error between desired and actual responses.

### Limitations
The [[XOR Classification Problem]] illustrates that a single perceptron cannot solve problems where the data is not linearly separable, necessitating the use of multilayer architectures.

## Multilayer Feedforward Neural Networks

A [[Feedforward Neural Network]] consists of layers of [[Artificial Neuron]] units. Unlike the perceptron, which uses a hard threshold, an artificial neuron uses a smooth [[Activation Function]] to produce its output.

### Neuron Models and Activations
Common activation functions include the [[Sigmoid Function]], the hyperbolic tangent, and the [[Rectifier Linear Unit]] (ReLU), the latter of which often outperforms others in deep architectures.

### Network Architecture
Networks are organized into an input layer, one or more [[Hidden Layer]] units, and an output layer. A network with two or more hidden layers is termed a [[Deep Neural Network]].

## Training and Operation

### Forward Pass
In a forward pass, input vectors are propagated through the network. Each neuron computes a sum-of-products of its inputs and weights, adds a bias, and applies its activation function. This process is efficiently implemented using matrix multiplications.

### Backpropagation
[[Backpropagation]] is the primary method for training these networks. It uses the chain rule from calculus to propagate the output error backward through the network, updating weights and biases via gradient descent to minimize a cost function.

## Convolutional Neural Networks

[[Convolutional Neural Network]] (CNN) architectures, such as the [[LeNet Architecture]], are designed specifically for image data. Unlike fully connected networks, CNNs learn 2-D features directly from raw images using convolution operations over spatial neighborhoods, making them highly effective for complex image recognition tasks.

## Relationships
- [[Perceptron]] implements a [[Linear Decision Function]].
- [[Linear Decision Function]] is defined by a [[Hyperplane]].
- [[Perceptron Training Algorithm]] trains a [[Perceptron]].
- [[Perceptron Convergence Theorem]] guarantees convergence of [[Perceptron Training Algorithm]].
- [[Least-Mean-Squared-Error Algorithm]] provides an alternative training method for [[Perceptron]].
- [[XOR Classification Problem]] demonstrates the limitations of a single [[Perceptron]].
- [[Artificial Neuron]] uses an [[Activation Function]].
- [[Feedforward Neural Network]] is composed of [[Artificial Neuron]].
- [[Feedforward Neural Network]] contains [[Hidden Layer]].
- [[Deep Neural Network]] is a type of [[Feedforward Neural Network]].
- [[Backpropagation]] is used to train [[Feedforward Neural Network]].
- [[Convolutional Neural Network]] is a specialized type of [[Feedforward Neural Network]].
- [[Convolutional Neural Network]] is exemplified by the [[LeNet Architecture]].
- [[Sigmoid Function]] is a type of [[Activation Function]].
- [[Rectifier Linear Unit]] is a type of [[Activation Function]].
