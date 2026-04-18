---
type: content
title: "Optimal Marching of Autonomous Networked Robots"
source: "D:"
index_parent: "Sources/D_.md"
created: 2026-04-18T14:53:08.160450700+00:00
summary: "A framework for redeploying autonomous networked robots between fields of interest while maximizing local connectivity and ensuring global connectivity."
tags:
  - gis
  - robotics
  - wireless-sensor-networks
  - computational-geometry
entities:
  - "[[Autonomous Networked Robots]]"
  - "[[Field of Interest]]"
  - "[[Total Stable Link Ratio]]"
  - "[[Global Connectivity]]"
  - "[[Minimum Cost Bipartite Matching Problem]]"
  - "[[Harmonic Map]]"
  - "[[Diffeomorphism]]"
  - "[[Centroidal Voronoi Diagram]]"
  - "[[Lloyd Algorithm]]"
  - "[[Optimal Marching Problem]]"
relationships:
  - source: "Autonomous Networked Robots"
    target: "Field of Interest"
    description: "explore and monitor"
  - source: "Optimal Marching Problem"
    target: "Total Stable Link Ratio"
    description: "aims to maximize"
  - source: "Optimal Marching Problem"
    target: "Global Connectivity"
    description: "requires as a constraint"
  - source: "Minimum Cost Bipartite Matching Problem"
    target: "Optimal Marching Problem"
    description: "solves for minimum moving distance but contradicts stable link ratio"
  - source: "Harmonic Map"
    target: "Diffeomorphism"
    description: "implements as a least stretched"
  - source: "Harmonic Map"
    target: "Optimal Marching Problem"
    description: "provides an approximated solution for"
  - source: "Centroidal Voronoi Diagram"
    target: "Autonomous Networked Robots"
    description: "determines optimal coverage positions for"
  - source: "Lloyd Algorithm"
    target: "Centroidal Voronoi Diagram"
    description: "computes"
---

# Optimal Marching of Autonomous Networked Robots

*A framework for redeploying autonomous networked robots between fields of interest while maximizing local connectivity and ensuring global connectivity.*

[[Autonomous Networked Robots]] (ANRs) are mobile robotic systems that coordinate to explore or monitor a [[Field of Interest]] (FoI). The challenge of redeploying these robots from one FoI to another is termed the [[Optimal Marching Problem]], which seeks to maximize the [[Total Stable Link Ratio]]—the proportion of local communication links preserved during transition—while ensuring [[Global Connectivity]] (the existence of a path to the network boundary for all robots).

## Relocation Trade-offs

Minimizing the total moving distance during relocation can be modeled as a [[Minimum Cost Bipartite Matching Problem]]. However, this objective contradicts the maximization of the [[Total Stable Link Ratio]]. In general cases, local connectivity cannot be fully preserved during relocation.

## Approximated Solution via Harmonic Mapping

To maintain local connectivities, a least stretched [[Diffeomorphism]] is required to map the connectivity graph of the robots from the current FoI to the target FoI. This is approximated using a [[Harmonic Map]]:
1. A triangulation is extracted from the connectivity graph.
2. The triangulation and the target FoI are both mapped to a unit disk.
3. An optimal rotation angle is found to maximize the [[Total Stable Link Ratio]].
4. Robots follow the induced harmonic map to redeploy.

## Final Coverage Optimization

Once redeployed, robots perform a minor local adjustment to reach optimal coverage positions. This is achieved by computing a [[Centroidal Voronoi Diagram]], where each robot (site) is located at the mass centroid of its Voronoi region. The [[Lloyd Algorithm]] is used iteratively to converge to this optimal layout, typically forming an equilateral triangulation.

## Relationships
- [[Autonomous Networked Robots]] explore and monitor [[Field of Interest]].
- [[Optimal Marching Problem]] aims to maximize [[Total Stable Link Ratio]] and requires [[Global Connectivity]].
- [[Minimum Cost Bipartite Matching Problem]] solves for minimum moving distance but contradicts [[Total Stable Link Ratio]].
- [[Harmonic Map]] implements a least stretched [[Diffeomorphism]] to solve the [[Optimal Marching Problem]].
- [[Centroidal Voronoi Diagram]] determines optimal coverage positions for [[Autonomous Networked Robots]].
- [[Lloyd Algorithm]] computes [[Centroidal Voronoi Diagram]].
