---
type: content
title: "Safe Tractable Approximations of Chance Constraints"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:14:33.366080400+00:00
summary: "This note explores various methods for approximating scalar chance constraints with tractable convex constraints. It covers the Robust Counterpart (RC) approach, the Bernstein approximation, and the Conditional Value at Risk (CVaR) based methods. The text highlights the trade-offs between computational complexity and conservatism."
tags:
  - operations-research
  - optimization
  - probability-theory
  - robust-optimization
entities:
  - "[[Chance Constraint]]"
  - "[[Robust Counterpart]]"
  - "[[Bernstein Approximation]]"
  - "[[Conditional Value at Risk]]"
  - "[[Globalized Robust Counterpart]]"
  - "[[Uncertainty Set]]"
  - "[[Conic Programming]]"
  - "[[Majorization]]"
relationships:
  - source: "Robust Counterpart"
    target: "Chance Constraint"
    description: "approximates"
  - source: "Bernstein Approximation"
    target: "Chance Constraint"
    description: "approximates"
  - source: "Conditional Value at Risk"
    target: "Chance Constraint"
    description: "approximates"
  - source: "Globalized Robust Counterpart"
    target: "Chance Constraint"
    description: "generalises"
  - source: "Uncertainty Set"
    target: "Robust Counterpart"
    description: "defines"
  - source: "Conic Programming"
    target: "Robust Counterpart"
    description: "underlies"
---

# Safe Tractable Approximations of Chance Constraints

*This note explores various methods for approximating scalar chance constraints with tractable convex constraints. It covers the Robust Counterpart (RC) approach, the Bernstein approximation, and the Conditional Value at Risk (CVaR) based methods. The text highlights the trade-offs between computational complexity and conservatism.*

This note details the mathematical frameworks used to approximate [[Chance Constraint]] problems, which involve ensuring a constraint is satisfied with a certain probability. 

## Concept
A [[Chance Constraint]] is a requirement that a random constraint is satisfied with a probability of at least $1-\epsilon$. Formally, for a randomly perturbed linear inequality, it is expressed as:

$$ \text{Prob} \{ x^T \zeta \leq b \} \geq 1 - \epsilon $$

Because evaluating this probability directly is often computationally difficult, researchers use [[Safe Tractable Approximations]].

### Robust Counterpart (RC)
A [[Robust Counterpart]] is an approximation where the constraint is required to be satisfied for all realizations of the data within a specified [[Uncertainty Set]] $\mathcal{Z}$. This is a common approach in robust optimization, but it can be overly conservative if the data falls outside $\mathcal{Z}$.

### Globalized Robust Counterpart (GRC)
The [[Globalized Robust Counterpart]] (GRC) extends the standard RC by allowing for controlled deterioration of the constraint when the data falls outside the nominal uncertainty set. It introduces a 'global sensitivity' parameter $\alpha$ to bound the violation based on the distance from the set.

### Bernstein Approximation
The [[Bernstein Approximation]] is a scheme that uses the moment-generating function of the random perturbations to create a safe convex approximation. It is particularly useful when the perturbations are independent and their moment-generating functions are efficiently computable.

### Conditional Value at Risk (CVaR)
nThe [[Conditional Value at Risk]] (CVaR) approach provides a bound on the probability of a random variable exceeding a certain threshold. It is recognized as the least conservative among the generating-function-based approximations discussed. The relationship between the CVaR approximation and the chance constraint is given by:

$$ \text{CVaR}_{\epsilon}(\xi) = \inf_{a 
eq 0} a + \frac{1}{\epsilon} \mathbb{E} \{\max[|a + \xi|, 0] \} $$

### Majorization
The concept of [[Majorization]] is used to bound probabilities by replacing random variables with 'more diffused' ones. This is useful for symmetric and unimodal distributions, where a more diffused distribution can provide a safe upper bound on the probability of violation.

## Relationships
- [[Robust Counterpart]] approximates [[Chance Constraint]]
- [[Bernstein Approximation]] approximates [[Chance Constraint]]
- [[Conditional Value at Risk]] approximates [[Chance Constraint]]
- [[Globalized Robust Counterpart]] generalises [[Robust Counterpart]]
- [[Uncertainty Set]] defines [[Robust Counterpart]]
- [[Conic Programming]] underlies [[Robust Counterpart]]
