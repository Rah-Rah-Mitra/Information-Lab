---
title: "Geospatial Data Models and Acquisition"
created: 2026-04-18T02:30:42.158745400+00:00
summary: "An overview of raster and vector data models, their properties, and the methodologies for primary and secondary geospatial data acquisition."
tags:
  - gis
  - geospatial-data
  - data-models
  - data-acquisition
entities:
  - "[[Data]]"
  - "[[Information]]"
  - "[[Geographic Data]]"
  - "[[Attribute Data]]"
  - "[[Raster Data Model]]"
  - "[[Vector Data Model]]"
  - "[[Spatial Resolution]]"
  - "[[Metadata]]"
  - "[[Primary Data]]"
  - "[[Secondary Data]]"
  - "[[Topology]]"
  - "[[Spaghetti Data Model]]"
  - "[[Topological Data Model]]"
  - "[[Satellite Imagery]]"
  - "[[Aerial Photography]]"
  - "[[Measurement Scale]]"
relationships:
  - source: "Data"
    target: "Information"
    description: "is processed into"
  - source: "Geographic Data"
    target: "Raster Data Model"
    description: "can be represented by"
  - source: "Geographic Data"
    target: "Vector Data Model"
    description: "can be represented by"
  - source: "Raster Data Model"
    target: "Spatial Resolution"
    description: "is characterized by"
  - source: "Vector Data Model"
    target: "Topology"
    description: "can incorporate"
  - source: "Vector Data Model"
    target: "Spaghetti Data Model"
    description: "includes the simple"
  - source: "Vector Data Model"
    target: "Topological Data Model"
    description: "includes the explicit"
  - source: "Topology"
    target: "Topological Data Model"
    description: "is the foundation of"
  - source: "Primary Data"
    target: "Geographic Data"
    description: "is a method of collecting"
  - source: "Secondary Data"
    target: "Geographic Data"
    description: "is a method of obtaining"
  - source: "Satellite Imagery"
    target: "Raster Data Model"
    description: "is an example of"
  - source: "Aerial Photography"
    target: "Raster Data Model"
    description: "is an example of"
  - source: "Metadata"
    target: "Geographic Data"
    description: "provides documentation for"
  - source: "Measurement Scale"
    target: "Attribute Data"
    description: "categorizes the complexity of"
---

# Geospatial Data Models and Acquisition

*An overview of raster and vector data models, their properties, and the methodologies for primary and secondary geospatial data acquisition.*

[[Geospatial Data Models and Acquisition]] is the study of how geographic information is structured, stored, and collected for use in a Geographic Information System (GIS).

## Data and Information
[[Data]] refer to raw facts, measurements, or characteristics of objects, while [[Information]] is the knowledge of value obtained through the collection, interpretation, and analysis of that data. In GIS, a critical distinction is made between [[Geographic Data]] (spatial data that defines the location of an object on Earth) and [[Attribute Data]] (non-geographic traits and characteristics of that object).

## Geospatial Data Models
To represent the real world in a computer, GIS uses two primary data models:

### Raster Data Model
The [[Raster Data Model]] represents space as a grid-based system of contiguous cells (pixels). Each cell carries a single value representing a spatial phenomenon. A key property is [[Spatial Resolution]], which is determined by the size of the square pixel; a finer resolution results in more detail but larger file sizes.

### Vector Data Model
The [[Vector Data Model]] uses points, lines, and polygons defined by X, Y coordinate pairs. It is generally more aesthetically pleasing and compact than raster data. Vector structures vary by their use of [[Topology]]—the set of rules modeling relationships between neighboring features:
- **[[Spaghetti Data Model]]**: The simplest structure where features are strings of coordinates with no inherent spatial relationships.
- **[[Topological Data Model]]**: Explicitly encodes connectivity, area definition, and contiguity, allowing for efficient network analysis and error detection.

## Data Acquisition
Geospatial data is acquired through two main methodologies:

### Primary Data
[[Primary Data]] are collected directly via firsthand observation or measurement, such as using GPS units, total stations, or remotely sensed sources like [[Satellite Imagery]] and [[Aerial Photography]].

### Secondary Data
[[Secondary Data]] are obtained from existing sources, such as government databases or paper maps. The process of converting non-digital sources into digital files is known as digitization, which can be performed via tablet digitizing, heads-up digitizing, or automated vectorization.

## Data Management
Effective data use requires [[Metadata]] (data about data) to ensure transparency and standards. Additionally, [[Attribute Data]] are categorized by a [[Measurement Scale]] (nominal, ordinal, interval, or ratio) to determine the level of complexity and the types of arithmetic operations possible.

## Relationships
- [[Data]] is processed into [[Information]].
- [[Geographic Data]] can be represented by the [[Raster Data Model]] or [[Vector Data Model]].
- [[Raster Data Model]] is characterized by [[Spatial Resolution]].
- [[Vector Data Model]] can incorporate [[Topology]].
- [[Vector Data Model]] includes the [[Spaghetti Data Model]] and [[Topological Data Model]].
- [[Topology]] is the foundation of the [[Topological Data Model]].
- [[Primary Data]] and [[Secondary Data]] are methods of collecting [[Geographic Data]].
- [[Satellite Imagery]] and [[Aerial Photography]] are examples of the [[Raster Data Model]].
- [[Metadata]] provides documentation for [[Geographic Data]].
- [[Measurement Scale]] categorizes the complexity of [[Attribute Data]].
