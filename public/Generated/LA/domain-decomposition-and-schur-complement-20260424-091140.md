---
type: content
title: "Domain Decomposition and Schur Complement"
source: "LA"
index_parent: "Sources/LA.md"
created: 2026-04-24T09:11:40.178266700+00:00
summary: "Domain decomposition methods partition a global problem into smaller subproblems to exploit parallelism and simplify geometry. The Schur complement approach uses block-Gaussian elimination to solve for interface variables, effectively decoupling the subdomains. This technique is a cornerstone of modern parallel computing for solving partial differential equations."
tags:
  - linear-algebra
  - domain-decomposition
  - numerical-methods
  - parallel-computing
  - schur-complement
entities:
  - "[[Domain Decomposition]]"
  - "[[Schur Complement]]"
  - "[[Schur Complement Matrix]]"
  - "[[Schwarz Alternating Procedures]]"
  - "[[Block-Gaussian Elimination]]"
  - "[[Symmetric Positive Definite]]"
  - "[[Dirichlet Boundary Conditions]]"
  - "[[Neumann Boundary Conditions]]"
  - "[[Partial Differential Equations]]"
  - "[[Finite Element Method]]"
relationships:
  - source: "Domain Decomposition"
    target: "Schur Complement"
    description: "utilizes"
  - source: "Domain Decomposition"
    target: "Schwarz Alternating Procedures"
    description: "implements via"
  - source: "Schur Complement Matrix"
    target: "Block-Gaussian Elimination"
    description: "is derived via"
  - source: "Schwarz Alternating Procedures"
    target: "Domain Decomposition"
    description: "is a type of"
  - source: "Schur Complement Matrix"
    target: "Symmetric Positive Definite"
    description: "inherits property of"
  - source: "Schur Complement Matrix"
    target: "Partial Differential Equations"
    description: "solplements for"
  - source: "Schur Complement Matrix"
    target: "Block-Gaussian Elimination"
    description: "depends on"
  - source: "Schur Complement Matrix"
    target: "Dirichlet Boundary Conditions"
    description: "handles via"
  - source: "Schur Complement Matrix"
    target: "Neumann Boundary Conditions"
    description: "handles via"
  - source: "Schur Complement Matrix"
    target: "Finite Element Method"
    description: "is used in"
---

# Domain Decomposition and Schur Complement

*Domain decomposition methods partition a global problem into smaller subproblems to exploit parallelism and simplify geometry. The Schur complement approach uses block-Gaussian elimination to solve for interface variables, effectively decoupling the subdomains. This technique is a cornerstone of modern parallel computing for solving partial differential equations.*

This note explores the principles of [[Domain Decomposition]] and the mathematical framework of the [[Schur Complement]].

## Concept
[[Domain Decomposition]] is a divide-and-conquer strategy used to solve [[Partial Differential Equations]] (PDEs) by partitioning a global domain into smaller, more manageable subdomains. This can be done through vertex-based, edge-based, or element-based partitionings. The primary goal is to exploit parallelism and simplify the geometry of the subproblems.

When a global linear system is partitioned into interior and interface nodes, the system can be expressed as:

$$ \begin{pmatrix} A_{ii} & A_{i\backslash} \ A_{\backslash i} & A_{\backslash\backslash} \end{pmatrix} \begin{pmatrix} x_i \ x_{\backslash} \end{pmatrix} = \begin{pmatrix} b_i \ b_{\backslash} \end{pmatrix} $$

By applying [[Block-Gaussian Elimination]], [[Schur Complement]] is the reduced system that solves for the interface variables $x_{\backslash}$:

$$ S = A_{\backslash\backslash} - A_{\backslash i} A_{ii}^{-1} A_{i\backslash} $$

If the original matrix $A$ is [[Symmetric Positive Definite]], then the Schur complement $S$ is also [[Symmetric Positive Definite]]. This property is crucial for iterative methods like the Conjugate Gradient algorithm to be applied to the reduced system.

## Schwarz Alternating Procedures
One approach to handling interface values is through [[Schwarz Alternating Procedures]], which can be either multiplicative (reminiscent of block Gauss-Seidel) or additive (reminiscent of block Jacobi). The these procedures often involve overlapping subdomains, where the following occurs:

- **Multiplicative Schwarz**: Updates boundary conditions from the most recent subdomain solutions sequentially.
- **Additive Schwarz**: Updates all subdomains simultaneously.

In the context of discretized [[Partial Differential Equations]], these procedures can be used to effectively manage [[Dirichlet Boundary Conditions]] or [[Neumann Boundary Conditions]] at the interfaces.

## Relationships
- [[Domain Decomposition]] utilizes [[Schur Complement]]
- [[Domain Decomposition]] implements via [[Schwarz Alternating Procedures]]
- [[Schur Complement Matrix]] is derived via [[Block-Gaussian Elimination]]
- [[Schwarz Alternating Procedures]] is a type of [[Domain Decomposition]]
- [[Schur Complement Matrix]] inherits property of [[Symmetric Positive Definite]]
- [[Schur Complement Matrix]] solves for [[Partial Differential Equations]]
- [[Schur Complement Matrix]] depends on on [[Block-Gaussian Elimination]]
- [[Schur Complement Matrix]] handles via [[Dirichlet Boundary Conditions]]
- [[Schur Complement Matrix]] Matrix handles via [[Neumann Boundary Conditions]]
- [[Schur Complement Matrix]] is used in [[Finite Element Method]]
