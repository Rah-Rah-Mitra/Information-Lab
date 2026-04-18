---
type: content
title: "Fundamentals of Geographic Information Systems"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T11:35:28.416535700+00:00
summary: "An introduction to GIS, covering spatial thinking, core geographic concepts, map types, and the integration of spatial and attribute data."
tags:
  - gis
  - geography
  - cartography
  - spatial-analysis
entities:
  - "[[Geographic Information System]]"
  - "[[Spatial Thinking]]"
  - "[[Mental Map]]"
  - "[[Geographic Question]]"
  - "[[Location]]"
  - "[[Direction]]"
  - "[[Distance]]"
  - "[[Space]]"
  - "[[Navigation]]"
  - "[[Map Scale]]"
  - "[[Coordinate System]]"
  - "[[Map Projection]]"
  - "[[Map Abstraction]]"
  - "[[Map Generalization]]"
  - "[[Spatial Data]]"
  - "[[Attribute Data]]"
relationships:
  - source: "Geographic Information System"
    target: "Spatial Thinking"
    description: "facilitates"
  - source: "Geographic Information System"
    target: "Geographic Question"
    description: "helps answer"
  - source: "Geographic Information System"
    target: "Spatial Data"
    description: "integrates"
  - source: "Geographic Information System"
    target: "Attribute Data"
    description: "integrates"
  - source: "Spatial Thinking"
    target: "Mental Map"
    description: "utilizes"
  - source: "Location"
    target: "Coordinate System"
    description: "defined by"
  - source: "Location"
    target: "Map Projection"
    description: "transformed by"
  - source: "Map Scale"
    target: "Map Abstraction"
    description: "influences"
  - source: "Map Projection"
    target: "Map Abstraction"
    description: "influences"
  - source: "Map Abstraction"
    target: "Map Generalization"
    description: "includes"
  - source: "Map Abstraction"
    target: "Spatial Data"
    description: "defines"
  - source: "Navigation"
    target: "Mental Map"
    description: "relies on"
  - source: "Navigation"
    target: "Location"
    description: "requires awareness of"
  - source: "Navigation"
    target: "Direction"
    description: "requires awareness of"
  - source: "Navigation"
    target: "Distance"
    description: "requires awareness of"
---

# Fundamentals of Geographic Information Systems

*An introduction to GIS, covering spatial thinking, core geographic concepts, map types, and the integration of spatial and attribute data.*

A [[Geographic Information System]] (GIS) is a specialized information technology that integrates spatial and attribute data to organize, analyze, visualize, and share geographic information.

## Spatial Thinking and Inquiry

[[Spatial Thinking]] is the process of relating to the environment and understanding the world by answering the question of "where." This process often begins with [[Mental Map]]s—psychological tools stored in the brain that reflect an individual's geographic knowledge and spatial awareness.

To leverage GIS, one must be able to formulate a [[Geographic Question]]. These questions typically fall into five categories: location, distribution, association, interaction, and change.

## Core Geographic Concepts

Several fundamental concepts form the framework for GIS:

- **[[Location]]**: A position on the earth's surface, described in nominal (by name), absolute (using a reference system like latitude and longitude), or relative terms.
- **[[Direction]]**: The position of something relative to a benchmark (e.g., egocentric, landmark, true north, magnetic north, or grid north).
- **[[Distance]]**: The degree of separation between locations, measured either nominally or absolutely using standard metrics.
- **[[Space]]**: An abstract concept denoting a general geographic area of interest, including [[Topological Space]] which focuses on connectivity and relationships.
- **[[Navigation]]**: Destination-oriented movement through space, which relies on landmark, route, and survey knowledge.

## Map Anatomy and Abstraction

Maps are representations of the world that can be reference, thematic, or dynamic. The transition from the real world to a map involves [[Map Abstraction]], which requires explicit and consistent definitions of features.

### Technical Constraints

- **[[Map Scale]]**: The factor of reduction used to fit the world onto a map. GIS is considered multiscalar rather than scaleless.
- **[[Coordinate System]]**: Frameworks used to define unique positions, such as the [[Geographic Coordinate System]] (GCS) based on a spheroid.
- **[[Map Projection]]**: Mathematical formulas used to transform the 3D earth into a 2D plane, inevitably introducing distortions in distance, direction, or area.

### Representation and Generalization

Geographic features are represented using three basic geometric forms: points, lines, and polygons. [[Map Generalization]] is the process of resolving conflicts caused by limited space or excessive detail through simplification, masking, selection, or exaggeration.

## Data Integration in GIS

At its core, a GIS integrates two primary types of data:

- **[[Spatial Data]]**: Information regarding real-world geographic objects and their locations.
- **[[Attribute Data]]**: Traits or characteristics of those objects (e.g., the population of a city).

## Relationships

- [[Geographic Information System]] facilitates [[Spatial Thinking]].
- [[Geographic Information System]] helps answer [[Geographic Question]]s.
- [[Geographic Information System]] integrates [[Spatial Data]] and [[Attribute Data]].
- [[Spatial Thinking]] utilizes [[Mental Map]]s.
- [[Location]] is defined by a [[Coordinate System]].
- [[Location]] is transformed by a [[Map Projection]].
- [[Map Scale]] and [[Map Projection]] influence [[Map Abstraction]].
- [[Map Abstraction]] includes [[Map Generalization]] and defines [[Spatial Data]].
- [[Navigation]] relies on [[Mental Map]]s and requires awareness of [[Location]], [[Direction]], and [[Distance]].
