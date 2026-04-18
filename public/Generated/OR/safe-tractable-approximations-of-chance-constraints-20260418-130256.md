---
type: content
title: "Safe Tractable Approximations of Chance Constraints"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-18T13:02:56.667944+00:00
summary: "Explores methods for approximating scalar chance constraints using Robust Counterparts, Bernstein schemes, and Conditional Value at Risk."
tags:
  - statistics
  - optimization
  - robust-optimization
  - chance-constraints
  - linear-algebra
entities:
  - "[[Chance Constraint]]"
  - "[[Robust Counterpart]]"
  - "[[Safe Tractable Approximation]]"
  - "[[Bernstein Approximation]]"
  - "[[Conditional Value at Risk]]"
  - "[[Scenario Approximation]]"
  - "[[Globalized Robust Counterpart]]"
  - "[[Majorization Theorem]]"
  - "[[Moment-Generating Function]]"
  - "[[Uncertain Linear Optimization]]"
relationships:
  - source: "Safe Tractable Approximation"
    target: "Chance Constraint"
    description: "approximates"
  - source: "Robust Counterpart"
    target: "Safe Tractable Approximation"
    description: "represents"
  - source: "Bernstein Approximation"
    target: "Safe Tractable Approximation"
    description: "is a type of"
  - source: "Conditional Value at Risk"
    target: "Safe Tractable Approximation"
    description: "provides the least conservative form of"
  - source: "Scenario Approximation"
    target: "Chance Constraint"
    description: "approximates via sampling"
  - source: "Globalized Robust Counterpart"
    target: "Robust Counterpart"
    description: "generalizes"
  - source: "Majorization Theorem"
    target: "Chance Constraint"
    description: "provides bounds for"
  - source: "Moment-Generating Function"
    target: "Bernstein Approximation"
    description: "is used to construct"
  - source: "Uncertain Linear Optimization"
    target: "Chance Constraint"
    description: "incorporates"
---

# Safe Tractable Approximations of Chance Constraints

*Explores methods for approximating scalar chance constraints using Robust Counterparts, Bernstein schemes, and Conditional Value at Risk.*

A [[Safe Tractable Approximation]] is a computationally efficient convex system that ensures a [[Chance Constraint]] is satisfied with a probability of at least $1-\epsilon$, without requiring simulation.

## Approximation Methods

### Scenario Approximation
The [[Scenario Approximation]] is a sampling-based approach where $N$ independent realizations of the uncertainty are generated. While general, it is not necessarily safe and becomes impractical when the risk level $\epsilon$ is very small, as the required sample size $N$ grows significantly.

### Robust Counterpart Representation
Many safe approximations can be represented as a [[Robust Counterpart]], where the original chance constraint is replaced by a requirement that the solution be robustly feasible for an "artificial" uncertainty set $\mathcal{Z}$. A safe approximation is called "normal" if it inherits the conic and closed properties of the true feasible set.

### Bernstein Approximation
The [[Bernstein Approximation]] utilizes the [[Moment-Generating Function]] of the random perturbations. By finding a convex upper bound $\Phi(w)$ on the logarithmic moment-generating function, one can construct a compact convex uncertainty set $\mathcal{Z}$ that yields a safe normal approximation.

### Conditional Value at Risk (CVaR)
[[Conditional Value at Risk]] (CVaR) provides the least conservative approximation among generating-function-based schemes. While theoretically optimal, it is often computationally intractable unless the random variables are supported on a finite set.

## Advanced Concepts

### Globalized Robust Counterpart
The [[Globalized Robust Counterpart]] extends the standard [[Robust Counterpart]] by allowing "soft" constraints. It ensures the constraint is satisfied within a normal range $\mathcal{Z}$ and bounds the violation linearly based on the distance from $\mathcal{Z}$ when perturbations fall outside this range, controlled by a global sensitivity parameter $\alpha$.

### Majorization and Monotone Dominance
The [[Majorization Theorem]] allows for the bounding of probabilities for independent random variables with symmetric and unimodal distributions. If one set of variables is "more diffuse" (monotone dominance) than another, the probability of falling into a symmetric closed convex set is lower for the more diffuse variables.

## Relationships
- [[Safe Tractable Approximation]] approximates [[Chance Constraint]]
- [[Robust Counterpart]] represents [[Safe Tractable Approximation]]
- [[Bernstein Approximation]] is a type of [[Safe Tractable Approximation]]
- [[Conditional Value at Risk]] provides the least conservative form of [[Safe Tractable Approximation]]
- [[Scenario Approximation]] approximates [[Chance Constraint]] via sampling
- [[Globalized Robust Counterpart]] generalizes [[Robust Counterpart]]
- [[Majorization Theorem]] provides bounds for [[Chance Constraint]]
- [[Moment-Generating Function]] is used to construct [[Bernstein Approximation]]
- [[Uncertain Linear Optimization]] incorporates [[Chance Constraint]]
