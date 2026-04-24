---
type: research
title: "Derivation of Euler-Lagrange Equations with Holonomic Constraints"
created: 2026-04-24T06:53:17.697103100+00:00
---

# Derivation of Euler-Lagrange Equations with Holonomic Constraints

*A step-by-step derivation of the Euler-Lagrange equations for a system with holonomic constraints using the method of Lagrange multipliers.*

# Derivation: Euler-Lagrange with Holonomic Constraints

## Overview
This derivation focuses on the extension of the Euler-Lagrange equations to systems where the coordinates are not independent, but are subject to a holonomic constraint of the form $f(q_1, q_2, \dots, q_n, t) = 0$.

## The Action Integral
For a system with $n$ generalized coordinates $q_i$ and a Lagrangian $L(q, \dot{q}, t)$, the action $S$ is defined as:

$$
S = \int_{t_1}^{t_2} L(q, \dot{q}, t) dt
$$

## Holonomic Constraints
A holonomic constraint is expressed as:

$$
{f(q_i, t) = 0}
$$

Where $f$ is a function of the coordinates and time. For a multiple constraints $m$ constraints, we have $f_k(q_i, t) = 0$ for $k = 1, \dots, m$.

## Derivation using Lagrange Multipliers
To find the extremum of the action $S$ subject to the constraints $f_k = 0$, we use the method of Lagrange multipliers. We modify the action by adding the constraint terms:

$$
S' = \int_{t_1}^{t_2} [L(q, \dot{q}, t) + \sum_{k=1}^m \lambda_k(t) f_k(q, t)] dt
$$

### Step 1: Variation of the Action
We take the variation $\delta S' = 0$:

$$
\delta S' = \int_{t_1}^{t_2} \sum_{i=1}^n \left( \frac{\partial L}{\partial q_i} \delta q_i + \frac{\partial L}{\partial \dot{q}_i} \delta \dot{q}_i + \sum_{k=1}^m \lambda_k \frac{\partial f_k}{\partial q_i} \delta q_i \right) dt = 0
$$

### Step 2: Integration by Parts
Integrating the term involving $\delta \dot{q}_i$ by parts:

$$
\int_{t_1}^{t_2} \frac{\partial L}{\partial \dot{q}_i} \delta \dot{q}_i dt = \left[ \frac{\partial L}{\partial \dot{q}_i} \delta q_i \right]_{t_1}^{t_2} - \int_{t_1}^{t_2} \frac{d}{dt} \left( \frac{\partial L}{\partial \dot{q}_i} \right) \delta q_i dt
$$

Since the variations $\delta q_i$ vanish at the endpoints $t_1$ and $t_2$, the boundary term is zero.

### Step 3: The Modified Euler-Lagrange Equation
Combining the terms and factoring out $\delta q_i$:

$$
\int_{t_1}^{t_2} \sum_{i=1}^n \left( \frac{\partial L}{\partial q_i} - \frac{d}{dt} \frac{\partial L}{\partial \dot{q}_i} + \sum_{k=1}^m \lambda_k \frac{\partial f_k}{\partial q_i} \right) \delta q_i dt = 0

$$

Since the $\delta q_i$ are no longer independent due to the constraint $f_k(q, t) = 0$, the $\lambda_k(t)$ are chosen such that the term in the parentheses is zero for each $i$:

$$
\frac{d}{dt} \left( \frac{\partial L}{\partial \dot{q}_i} \right) - \frac{\partial L}{\partial q_i} = \sum_{k=1}^m \lambda_k \frac{\partial f_k}{\partial q_i}
$$

## Observed Facts
- The method of Lagrange multipliers is the standard approach for handling holonomic constraints in classical mechanics.
- The constraint forces are represented by the term $\sum \lambda_k \frac{\partial f_k}{\partial q_i}$.

## Hypotheses
- If the constraints are non-holonomic, the method of Lagrange multipliers can be extended, but the only way to solve the equations is typically via the d'Alembert principle.

## Next Checks
- Verify the physical interpretation of $\lambda_k$ as the magnitude of the constraint force.
- Explore the application of this derivation to a pendulum system to demonstrate the practical use of the constraints.

