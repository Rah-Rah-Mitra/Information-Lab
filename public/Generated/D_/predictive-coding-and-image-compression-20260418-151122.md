---
type: content
title: "Predictive Coding and Image Compression"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T15:11:22.762697800+00:00
summary: "An overview of predictive coding techniques, including lossless and lossy variants, motion compensation for video, and their application in standards like JPEG."
tags:
  - computer-vision
  - image-compression
  - predictive-coding
  - video-compression
  - signal-processing
entities:
  - "[[Predictive Coding]]"
  - "[[Lossless Predictive Coding]]"
  - "[[Lossy Predictive Coding]]"
  - "[[Prediction Residual]]"
  - "[[Motion Compensation]]"
  - "[[Motion Vector]]"
  - "[[I-frame]]"
  - "[[P-frame]]"
  - "[[B-frame]]"
  - "[[Differential Pulse Code Modulation]]"
  - "[[Delta Modulation]]"
  - "[[Lloyd-Max Quantizer]]"
  - "[[JPEG]]"
  - "[[JPEG-2000]]"
  - "[[Discrete Cosine Transform]]"
relationships:
  - source: "Predictive Coding"
    target: "Lossless Predictive Coding"
    description: "includes"
  - source: "Predictive Coding"
    target: "Lossy Predictive Coding"
    description: "includes"
  - source: "Lossy Predictive Coding"
    target: "Prediction Residual"
    description: "generates"
  - source: "Motion Compensation"
    target: "Motion Vector"
    description: "uses"
  - source: "Motion Compensation"
    target: "P-frame"
    description: "creates"
  - source: "Motion Compensation"
    target: "B-frame"
    description: "description: "
  - source: "Differential Pulse Code Modulation"
    target: "Predictive Coding"
    description: "is a form of"
  - source: "Delta Modulation"
    target: "Lossy Predictive Coding"
    description: "is a simple form of"
  - source: "Lloyd-Max Quantizer"
    target: "Lossy Predictive Coding"
    description: "optimizes error in"
  - source: "JPEG"
    target: "Discrete Cosine Transform"
    description: "uses"
  - source: "JPEG-2000"
    target: "Predictive Coding"
    description: "incorporates"
---

# Predictive Coding and Image Compression

*An overview of predictive coding techniques, including lossless and lossy variants, motion compensation for video, and their application in standards like JPEG.*

Predictive coding is a compression technique that reduces redundancy by encoding only the difference between an actual pixel value and its predicted value.

## Lossless Predictive Coding
Lossless predictive coding uses an identical predictor in both the encoder and decoder to generate an anticipated value based on past samples. The difference between the actual sample and the predicted value, known as the [[Prediction Residual]], is then encoded using a variable-length code. This process removes spatial redundancy in images or temporal redundancy in video sequences.

## Lossy Predictive Coding
Lossy predictive coding introduces a quantizer between the symbol encoder and the point where the prediction error is formed. This prevents error buildup at the decoder's output by placing the predictor within a feedback loop. A common example is [[Delta Modulation]], where the predictor and quantizer are simplified to a 1-bit fixed-length code, though this can lead to distortions such as slope overload and granular noise.

### Optimal Prediction and Quantization
To minimize mean-squared prediction error, [[Differential Pulse Code Modulation]] (DPCM) is often employed. The optimal predictor is typically a linear combination of previous samples. For quantization, the [[Lloyd-Max Quantizer]] is used to design decision and reconstruction levels that minimize mean-squared quantization error for a given probability density function, such as a Laplacian PDF.

## Motion Compensation in Video
In video compression, [[Motion Compensation]] is used to reduce temporal redundancy by tracking object movement between frames. The video frame is divided into macroblocks, and the movement of each macroblock relative to a reference frame is encoded as a [[Motion Vector]].

### Frame Types
- **I-frames (Independent frames)**: Compressed without a prediction residual, resembling [[JPEG]] images and serving as random access points.
- **P-frames (Predictive frames)**: Based on a forward prediction from a previous frame.
- **B-frames (Bidirectional frames)**: Based on both previous and subsequent frames, requiring the codestream to be reordered for decoding.

## Comparison with Transform Coding
While predictive coding focuses on differences, transform coding (like the [[Discrete Cosine Transform]] used in [[JPEG]]) decorrelates pixels. [[JPEG-2000]] extends these capabilities by using wavelet-based coding, which avoids the blocking artifacts associated with DCT-based approximations at high compression ratios.
