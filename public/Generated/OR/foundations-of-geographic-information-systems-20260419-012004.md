---
type: content
title: "Foundations of Geographic Information Systems"
source: "OR"
index_parent: "Sources/OR.md"
created: 2026-04-19T01:20:04.746357200+00:00
summary: "This note explores the fundamental concepts of geography and the core components of Geographic Information Systems (GIS). It details how spatial thinking, mental maps, and geographic questions drive the utility of GIS in analyzing location, distribution, and change. The concepts of location, direction, distance, and space form the essential framework for spatial analysis."
tags:
  - gis
  - spatial-thinking
  - geography-fundamentals
entities:
  - "[[Geographic Information Systems]]"
  - "[[Mental Maps]]"
  - "[[Spatial Thinking]]"
  - "[[Location]]"
  - "[[Direction]]"
  - "[[Distance]]"
  - "[[Topological Space]]"
  - "[[Global Positioning System]]"
relationships:
  - source: "Geographic Information Systems"
    target: "Spatial Thinking"
    description: "requires"
  - source: "Geographic Information Systems"
    target: "Location"
    description: "analyzes"
  - source: "Geographic Information Systems"
    target: "Direction"
    description: "utilizes"
  - source: "Co-location"
    target: "Location"
    description: "is a type of"
  - source: "Global Positioning System"
    target: "Location"
    description: "determines"
  - source: "Mental Maps"
    target: "Spatial Thinking"
    description: "reflects"
  - source: "Topological Space"
    target: "Geographic Information Systems"
    description: "is a type of space studied in"
---

# Foundations of Geographic Information Systems

*This note explores the fundamental concepts of geography and the core components of Geographic Information Systems (GIS). It details how spatial thinking, mental maps, and geographic questions drive the utility of GIS in analyzing location, distribution, and change. The concepts of location, direction, distance, and space form the essential framework for spatial analysis.*

This note provides an overview of the fundamental principles that underpin [[Geographic Information Systems]] (GIS) and the human approach to spatial reasoning.

## Concept
[[Geographic Information Systems]] are information technologies used to organize, analyze, visualize, and share geographic data. They are used to answer the "where" question by integrating various data sources into maps. Central to the GIS framework is the concept of [[Spatial Thinking]], which involves the ability to recognize and relate to the local environment through spatial awareness.

Humans use [[Mental Maps]]—psychological tools stored in the brain—to navigate and understand their surroundings. These maps are unique to individuals and serve as approximations of local geographic knowledge. To move beyond mental maps, GIS users must learn to ask effective geographic questions, which typically fall into five categories: location, distribution, association, interaction, and change.

Key geographic concepts include:
- [[Location]]: A position on the Earth's surface. This can be defined as nominal (by name), absolute (using coordinates like latitude and longitude), or relative (in relation to other places).
- [[Direction]]: The position of something relative to another, often measured from a benchmark such as true north, magnetic north, or grid north.
- [[Distance]]: The degree of separation between locations, which can be measured nominally or through absolute metrics like the Euclidean distance formula:

$$ D = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} $$

This formula calculates the distance between two points on a planar surface.
- [[Space]]: An abstract concept describing geographic areas. [[Topological Space]] is a specific type of space concerned with the connectivity and relationships between locations, such as the connections in a subway network.
- [[Global Positioning System]]: A constellation of satellites used to triangulate and determine absolute locations on the Earth's surface.

## Relationships
- [[Geographic Information Systems]] requires [[Spatial Thinking]]
- [[Geographic Information Systems]] analyzes [[Location]]
- [[Global Positioning System]] determines [[Location]]
- [[Mental Maps]] reflects [[Spatial Thinking]]
- [[Topological Space]] is a type of space studied in [[Geographic Information Systems]]
