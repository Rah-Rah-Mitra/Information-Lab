---
type: content
title: "Foundations of Geographic Information Systems"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-19T01:20:27.367514800+00:00
summary: "This note explores the fundamental components of GIS, including the distinction between spatial and attribute data, the mechanics of map projections, and the essential role of metadata. It provides a framework for understanding how real-world geographic features are abstracted into digital formats for analysis."
tags:
  - gis
  - cartography
  - spatial-data
  - geospatial-science
entities:
  - "[[Geographic Information System]]"
  - "[[Spatial Data]]"
  - "[[Attribute Data]]"
  - "[[Map Projection]]"
  - "[[Map Scale]]"
  - "[[Coordinate System]]"
  - "[[Metadata]]"
  - "[[Map Abstraction]]"
  - "[[Raster Data Model]]"
  - "[[Vector Data Model]]"
relationships:
  - source: "Geographic Information System"
    target: "Spatial Data"
    description: "integrates"
  - source: "Geographic Information System"
    target: "Attribute Data"
    description: "integrates"
  - source: "Geographic Information System"
    target: "Map Projection"
    description: "utilizes"
  - source: "Geographic Information System"
    target: "Map Scale"
    description: "utilizes"
  - source: "Geographic Information System"
    target: "Coordinate System"
    description: "utilizes"
  - source: "Geographic Information System"
    target: "Metadata"
    description: "requires"
  - source: "Map Projection"
    target: "Spatial Data"
    description: "transforms"
  - source: "Map Abstraction"
    target: "Spatial Data"
    description: "defines"
  - source: "Map Abstraction"
    target: "Map Scale"
    description: "involves"
  - source: "Map Abstraction"
    target: "Map Projection"
    description: "involves"
---

# Foundations of Geographic Information Systems

*This note explores the fundamental components of GIS, including the distinction between spatial and attribute data, the mechanics of map projections, and the essential role of metadata. It provides a framework for understanding how real-world geographic features are abstracted into digital formats for analysis.*

This note provides an overview of the fundamental concepts required to understand and operate within a [[Geographic Information System]] (GIS).

## Concept
A [[Geographic Information System]] is a multi-faceted entity comprising software, hardware, networks, and people, used to maintain, analyze, and share geographic data. A core requirement of any GIS is the integration of two distinct types of data: [[Spatial Data]] and [[Attribute Data]].

[[Spatial Data]] refers to the real-world geographic objects and their locations (e.g., streets, lakes, or coordinates), whereas [[Attribute Data]] describes the non-geographic traits or characteristics of those objects (e.g., a building's population or a forest's tree density).

To represent the three-dimensional earth on a two-dimensional surface, GIS practitioners use [[Map Projection]] techniques. A [[Map Projection]] is a mathematical formula that translates latitude and longitude into $x$ and $y$ coordinates on a plane. Because this transformation is from a 3D spheroid to a 2D surface, all projections inevitably introduce distortions in distance, angle, or area.

Effective mapping also requires managing [[Map Scale]], which is the factor of reduction used to fit the real world onto a map. A [[Map Scale]] can be expressed as a [[Representative Fraction]] (e.g., 1:10,000), where the numerator is map distance and the denominator is ground distance.

To define unique positions on the earth, a [[Coordinate System]] is used. The most common is the [[Geographic Coordinate System]] (GCS), which uses latitude and longitude measured in degrees relative to the equator and prime meridian. A [[Datum]] specifies the orientation and origin of these lines relative to the center of the earth.

[[Map Abstraction]] is the process of explicitly defining and representing real-world features in a digital environment. This involves deciding which features to include, how to simplify them, and how to symbolize them using geometric forms like [[Point]], [[Line]], or [[Polygon]].

Finally, [[Metadata]] (data about data) is essential for ensuring the quality, reliability, and interoperability of datasets. [[Geospatial Metadata]] specifically documents the geographic extent, projection, and attribute definitions of a spatial dataset.

## Relationships
- [[Geographic Information System]] integrates [[Spatial Data]]
- [[Geographic Information System]] integrates [[Attribute Data]]
- [[Geographic Information System]] utilizes [[Map Projection]]
- [[Geographic Information System]] utilizes [[Map Scale]]
- [[Geographic Information System]] utilizes [[Coordinate System]]
- [[Geographic Information System]] requires [[Metadata]]
- [[Map Projection]] transforms [[Spatial Data]]
- [[Map Abstraction]] defines [[Spatial Data]]
- [[Map Abstraction]] involves [[Map Scale]]
- [[Map Abstraction]] involves [[Map Projection]]
