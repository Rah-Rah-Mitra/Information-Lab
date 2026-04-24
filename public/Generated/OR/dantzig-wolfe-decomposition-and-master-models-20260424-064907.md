---
type: content
title: "Dantzig-Wolfe Decomposition and Master Models"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T06:49:07.581012300+00:00
summary: "Dantzig-Wolfe decomposition is a method for solving large-scale structured linear programming models by breaking them into a submodels and a master model. It utilizes internal prices (shadow prices) and vertex proposals to vertex-based solutions from submodels to iteratively than the full model. This approach is efficient for block-angular structures and block-diagonal structures."
tags:
  - operations-research
  - linear-programming
  - decomposition-methods
  - mathematical-programming
entities:
  - "[[Dantzig-Wolfe Decomposition]]"
  - "[[Master Model]]"
  - "[[Restricted Master Model]]"
  - "[[Submodels]]"
  - "[[Shadow Prices]]"
  - "[[Proposals]]"
  - "[[Linear Programming]]"
  - "[[Block-Angular Structure]]"
relationships:
  - source: "Dantzig-Wolfe Decomposition"
    target: "Master Model"
    description: "utilizes"
  - source: "Dantzig-Wolfe Decomposition"
    target: "Submodels"
    description: "decomposes into"
  - source: "Dantzig-Wolfe Decomposition"
    target: "Shadow Prices"
    description: "uses"
  - source: "Master Model"
    target: "Proposals"
    description: "incorporates"
  - source: "Master Model"
    target: "Restricted Master Model"
    description: "specialises to"
  - source: "Submodels"
    target: "Proposals"
    description: "generate"
  - source: "Submodels"
    target: "Shadow Prices"
    description: "provide"
  - source: "Master-Model"
    target: "Shadow Prices"
    description: "determines"
---

# Dantzig-Wolfe Decomposition and Master Models

*Dantzig-Wolfe decomposition is a method for solving large-scale structured linear programming models by breaking them into a submodels and a master model. It utilizes internal prices (shadow prices) and vertex proposals to vertex-based solutions from submodels to iteratively than the full model. This approach is efficient for block-angular structures and block-diagonal structures.*

The process of decomposition in mathematical programming, specifically through the techniques associated with the [[Dantzig-Wolfe Decomposition]], is a method for solving large-scale structured models by avoiding the direct solution of a massive, monolithic model. This is particularly useful in organizations with decentralized planning needs or models with a block-angular structure.

## Concept
In a decomposition scheme, the problem is split into a 

### Submodels
[[Submodels]] represent the individual components or plants of a system, containing the technological details and specific constraints relevant to only that subproblem. For example, in a multiplant model, a submodel might contain the specific production capacities and processing times for a single factory.

### Master Model
[[Master Model]] refers to the overall organizational model that manages common resources and common rows. Instead of containing all the technological detail, it uses a simple convexity constraint to represent the subproblems. The because of the scale, a 

### Restricted Master Model
[[Restricted Master Model]] is a truncated version of the master model that only includes a subset of the possible [[Proposals]] (vertex solutions from submodels) to make the computation manageable. 

Optimization of the [[Master Model]] produces [[Shadow Prices]] (also known as marginal values) for the common resources. These prices are then passed back to the submodels as internal charges, which in turn cause the submodels to reformulate their objective functions and generate new [[Proposals]]. This cycle continues until no new useful proposals are added to the model.

$$ \text{Profit}_A = 1.12x_1 + 3.18x_2 $$ 

This equation represents a reformulated objective function for a submodel after being charged an internal price for a resource.

## Relationships
- [[Dantzig-Wolfe Decomposition]] utilizes [[Master Model]]
- [[Dantzig-Wolfe Decomposition]] decomposes into [[Submodels]]
- [[Dantzig-Wolfe Decomposition]] uses [[Shadow Prices]]
- [[Master Model]] incorporates [[Proposals]]
- [[Master Model]] specialises to [[Restricted Master Model]]
- [[Submodels]] generate [[Proposals]]
- [[Submodels]] generate [[Shadow Prices]]
- [[Master Model]] determines [[Shadow Prices]]
