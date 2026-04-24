---
type: content
title: "Mathematical Programming Model Building"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-24T07:20:51.264525900+00:00
summary: "This note explores various complex real-world applications of mathematical programming, including refinery optimization, mining, and economic planning. It demonstrates how to translate physical and operational constraints into linear and multi-period mathematical models. The text focuses on the method of model building rather than specific solutions."
tags:
  - operations-research
  - mathematical-programming
  - model-building
  - linear-programming
entities:
  - "[[Mathematical Programming]]"
  - "[[Mathematical Programming Model]]"
  - "[[Refinery Optimization]]"
  - "[[Mining]]"
  - "[[Economic Planning]]"
  - "[[Multi-period Problem]]"
  - "[[Linear Programming]]"
  - "[[Constraint Modeling]]"
relationships:
  - source: "Mathematical Programming"
    target: "Mathematical Programming Model"
    description: "uses"
  - source: "Mathematical Programming"
    target: "Linear Programming"
    description: "specialises to"
  - source: "Mathematical Programming"
    target: "Constraint Modeling"
    description: "requires"
  - source: "Mathematical Programming Model"
    target: "Multi-period Problem"
    description: "generalises to"
  - source: "Refinery Optimization"
    target: "Mathematical Programming Model"
    description: "is an example of"
  - source: "Mining"
    target: "Mathematical Programming Model"
    description: "is an example of"
  - source: "Economic Planning"
    target: "mapping to a model"
    description: "is an example of of"
  - source: "Economic Planning"
    target: "Mathematical Programming Model"
    description: "is an example of"
  - source: "Mining"
    target: "Mathematical Programming Model"
    description: "is an example of"
  - source: "Multi-period Problem"
    target: "Mathematical Programming Model"
    description: "is an example of a type of"
  - source: "Constraint Modeling"
    target: "Mathematical Programming Model never to invent facts not supported by the sources. If two sections disagree, prefer the longer/more specific source and note the disagreement in the body.  "
    description: "is a component of"
  - source: "Constraint "
    target: "Mathematical Programming Model"
    description: "is a component of"
---

# Mathematical Programming Model Building

*This note explores various complex real-world applications of mathematical programming, including refinery optimization, mining, and economic planning. It demonstrates how to translate physical and operational constraints into linear and multi-period mathematical models. The text focuses on the method of model building rather than specific solutions.*

This note explores the methodology of [[Mathematical Programming]] as applied to various industrial and economic systems through thes construction of a [[Mathematical Programming Model]].

## Concept
[[Mathematical Programming]] is the process of selecting an optimal course of action among a set of alternatives by satisfying a set of constraints. The provided text presents several case studies that illustrate the different types of constraints and variables that can be- 

### Refinery Optimization
In a [[Refinery Optimization]] problem, the objective is to maximize profit by blending different crude oils through various processes like distillation, reforming, and cracking. Constraints include availability of crude oil, capacity of the limitations of the processes, and product quality (e.g., octane number or vapour pressure). Quality constraints are often modeled as linear combinations of volume. For example, in blending, the following constraint might bethought of as: 

$$ \text{Quality} = \frac{\text{Quantity}_1 \times \text{Quality}_1 + \text{Quantity}_2 \times \text{Quality}_2}{\text{Quantity}_1 + \text{Quantity}_2} \text{ is not linear, but can be re-expressed as: } \n\n$$ 	ext{Quality} 	imes (	ext{Quantity}_1 + 	ext{Quantity}_2) = 	ext{Quantity}_1 	imes 	ext{Quality}_1 + 	ext{Quantity}_2 	imes 	ext{Quality}_2 $$ \n\nThis ensures the dimensionally correct linear form. \n\n### Mining and Resource Extraction\nProblems in [[Mining]] and [[Opencast Mining]] involve deciding which mines or blocks to be extracted to maximize revenue minus cost, while considering physical constraints like the angle of slip and the existence of \n\n### Economic Planning\nIn [[Economic Planning]], models often deal with multiple industries and the| (e.0. coal, steel, and transport) and the time lag between investment in capacity and production. These models must account for stocks, stocks, and productive capacity. \n\n### Multi-period Problems\nIn a [[Multi-period Problem]], decisions made in the current period must account for future periods. This is often modeled using linking constraints that connect the variables of one period to the next. For example, the relationship between stock at the end of the month $t$ is the stock at the end of the month $t+1$ is: \n\n$$ S_{t} + B_{t} - U_{t} = S_{t+1} $$ \n\nwhere $B_t$ is the amount bought, $U_t$ is the amount used, and $S_{t+1}$ is the ability to the amount stored. \n\n## Relationships\n- [[Mathematical Programming]] uses [[Mathematical Programming Model]]\n- [[Mathematical Programming Model]] produces [[Linear Programming]]\n- [[Mathematical Programming Model]] is applied to [[Refinery Optimization]], [[Mining]], and [[Economic Planning]]\n- [[Multi-period Problem]] is a type of [[Mathematical Programming Model]]\n- [[Constraint Modeling]] is a component of [[Mathematical Programming Model]]
