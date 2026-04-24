---
type: content
title: "Fundamentals of Constrained Optimization Algorithms"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T08:00:50.008842400+00:00
summary: "This note covers the taxonomy of algorithms for solving nonlinear constrained optimization problems, including penalty, augmented Lagrangian, SQP, and interior-point methods. It also explores variable elimination techniques and the mechanisms for step acceptance via merit functions and filters. The text highlights the combinatorial difficulty of managing inequality constraints."
tags:
  - operations-research
  - nonlinear-optimization
  - constrained-optimization
  - optimization-algorithms
entities:
  - "[[Nonlinear Programming]]"
  - "[[Penalty Function]]"
  - "[[Augmented Lagrangian Method]]"
  - "[[Sequential Quadratic Programming]]"
  - "[[Interior-Point Method]]"
  - "[[Active-Set Method]]"
  - "[[Combinatorial Difficulty]]"
  - "[[Merit Function]]"
  - "[[Exact Penalty Function]]"
  - "[[Variable Elimination]]"
  - "[[Null Space]]"
  - "[[Filter Method]]"
relationships:
  - source: "Nonlinear Programming"
    target: "Penalty Function"
    description: "can be solved using"
  - source: "Nonlinear Programming"
    target: "Augmented Lagrangian Method"
    description: "can be solved using"
  - source: "Nonlinear Programming"
    target: "Sequential Quadratic Programming"
    description: "can be solved using"
  - source: "Nonlinear Programming"
    target: "Interior-Point Method"
    description: "can be solved using"
  - source: "Nonlinear Programming"
    target: "Active-Set Method"
    description: "can be solved using"
  - source: "Nonlinear Programming"
    target: "Merit Function"
    description: "requires assessment via"
  - source: "Nonitieral Programming"
    target: "Filter Method"
    description: "balances goals via"
  - source: "Active-Set Method"
    target: "Combinatorial Difficulty"
    description: "suffers from"
  - source: "Variable Elimination"
    target: "Null Space"
    description: "utilizes"
  - source: "Merit Function"
    target: "Exact Penalty Function"
    description: "includes"
  - source: "Augmented Lagrangian Method"
    target: "Merit Function"
    description: "is a type of"
  - source: "Penalty Function"
    target: "Merit Function"
    description: "is a type of"
---

# Fundamentals of Constrained Optimization Algorithms

*This note covers the taxonomy of algorithms for solving nonlinear constrained optimization problems, including penalty, augmented Lagrangian, SQP, and interior-point methods. It also explores variable elimination techniques and the mechanisms for step acceptance via merit functions and filters. The text highlights the combinatorial difficulty of managing inequality constraints.*

This note provides an overview of the fundamental algorithmic approaches to solving [[Nonlinear Programming]] problems, which involve minimizing an objective function subject to constraints. 

## Concept
[[Nonlinear Programming]] is defined as the minimization of a smooth, real-valued function $f(x)$ subject to equality and inequality constraints $c_i(x) = 0$ and $c_i(x) \nless 0$. 

Several major classes of algorithmic approaches exist:

1. [[Penalty Function]] methods: These combine the objective and constraints into a single unconstrained function. A quadratic penalty function is defined as:
$$ \varphi(x) = f(x) + \mu \sum_{i \in E} c_i(x)^2 $$
This function uses a penalty parameter $\mu$ to penalize constraint violation.

2. [[Augmented Lagrangian Method]]s: These improve upon penalty functions by combining the properties of the Lagrangian function and the quadratic penalty function. For equality constraints, the augmented Lagrangian is:
$$ L_A(x, \lambda, \mu) = f(x) - \lambda^T c(x) + \frac{\mu}{2} \c(x)^T \c(x) $$ 
This approach avoids some of the drawbacks of simple penalty functions.

3. [[Sequential Quadratic Programming]] (SQP): These methods model the nonlinear problem as a quadratic programming subproblem at each iterate. The subproblem objective is an approximation of the change in the Lagrangian function.

4. [[Interior-Point Method]]s: These are often viewed as [[Barrier Method]]s that generate steps by solving a problem with a logarithmic barrier function:
$$ \min f(x) - \\mu \sum_{i=1}^m \log(s_i) \text{ subject to } c(x) - s = 0 $$ 
These methods avoid the [[Combinatorial Difficulty]] of managing inequality constraints by staying away from the boundary of the feasible region.

5. [[Active-Set Method]]s: These methods make a guess of the optimal active set $W$ and solve the problem by imposing equality constraints for the constraints in $W$. They face the [[Combinatorial Difficulty]] because the number of possible working sets grows exponentially with the number of inequality constraints.

## Variable Elimination

When dealing with linear equality constraints $Ax = b$, one can use [[Variable Elimination]] to simplify the problem. 

### Simple Elimination
Using a basis matrix $B$ and a permutation matrix $P$, the constraints can be rewritten to express basic variables $x_B$ in terms of non-basic variables $x_N$. Any feasible point can be expressed as:
$$ x_B = B^{-1}(b - N x_N) $$ 
This method is a form of [[Coordinate Relaxation]] and can lead to numerical instability if the basis is poorly chosen.

### General Reduction Strategies
A more robust approach uses an orthonormal basis for the [[Null Space]] of $A$. An orthogonal matrix $Q$ from a QR factorization of $A$ can be used to construct matrices $Y$ and $Z$ such that $Z$ is a basis for the [[Null Space]] of $A$. This allows the rewriting of the problem as an unconstrained problem in terms of $Z$: 
$$ \text{min } f(x) = f(Y x_Z + Y^{\perp} b) $$ 
This method is numerically more stable than simple elimination.

## Step Acceptance Mechanisms

To balance the competing goals of minimizing the objective function and satisfying constraints, algorithms use [[Merit Function]]s or [[Filter Method]]s.

### Merit Functions
An [[Exact Merit Function]] is a function $\varphi(x, \mu)$ that, for a sufficiently large $\mu$, ensures that any local solution of the nonlinear program is a local minimizer of the function. A common choice is the $\ell_1$ penalty function:
$$ \varphi(x, \mu) = f(x) + \mu \text{max}(0, c(x)) $$ 

### Filter Methods
A [[Filter Method]] treats the two goals as separate objectives in a multiobjective optimization framework. It accepts a trial step if the pair $(f(x), h(x))$ is not dominated by a previous step, where $h(x)$ is a measure of infeasibility. 

## Relationships
- [[Nonlinear Programming]] is solved via [[Penalty Function]], [[Augmented Lagrangian Method]], [[Augmented Lagrangian Method]], [[Sequential Quadratic Programming]], [[Interior-Point Method]], and [[Active-Set Method]].
- [[Active-Set Method]] suffers from [[Combinatorial Difficulty]].
- [[Variable Elimination]] uses [[Null Space]] to improve stability.
- [[Merit Function]] is a type of [[Exact Penalty Function]].
- [[Filter Method]] is a separate approach to the ability to assess step quality via [[Merit Function]].
