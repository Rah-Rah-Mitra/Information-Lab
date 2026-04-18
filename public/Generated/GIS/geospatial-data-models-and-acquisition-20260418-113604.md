---
type: content
title: "Geospatial Data Models and Acquisition"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T11:36:04.564957300+00:00
summary: "An overview of raster and vector data models, the distinction between spatial and attribute data, and methodologies for primary and secondary data acquisition."
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
  - "[[Database Management System]]"
relationships:
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
  - source: "Data"
    target: "Information"
    description: "becomes when put into context"
  - source: "Geographic Data"
    target: "Attribute Data"
    description: "complements"
  - source: "Secondary Data"
    target: "Metadata"
    description: "often accompanied by"
  - source: "Primary Data"
    target: "Geographic Data"
    description: "is a method of collecting"
  - source: "Secondary Data"
    target: "Geographic Data"
    description: "is a method of obtaining"
  - source: "Vector Data Model"
    target: "Database Management System"
    description: "links spatial features to"
  - source: "Raster Data Model"
    target: "Database Management System"
    description: "can reference for attribute tables"
---

# Geospatial Data Models and Acquisition

*An overview of raster and vector data models, the distinction between spatial and attribute data, and methodologies for primary and secondary data acquisition.*

Geospatial data management involves the distinction between raw facts and processed knowledge, as well as the selection of appropriate models to represent the earth's surface.

## Data and Information
[[Data]] refer to raw facts, measurements, or characteristics of an object. When these data are situated within analytical frameworks or used to answer questions, they become [[Information]].

### Spatial vs. Attribute Data
In a GIS environment, a critical distinction is made between [[Geographic Data]] (or spatial data), which define the location of an object on the earth's surface, and [[Attribute Data]], which describe the nongeographic traits and characteristics of that object.

## Geospatial Data Models
[[Data]] are represented in a computer using specific rules and constructs known as data models.

### Raster Data Model
The [[Raster Data Model]] consists of a grid-based system of contiguous, equally sized pixels. Each cell carries a single value representing a spatial phenomenon. A key characteristic of this model is [[Spatial Resolution]], which is determined by the size of the square pixel; a finer resolution results in higher detail but larger file sizes.

### Vector Data Model
The [[Vector Data Model]] represents space using points, lines, and polygons defined by X, Y coordinate pairs. Unlike the raster model, it is not quantized into grid cells. Vector data can be structured as a "spaghetti model" (no inherent structure) or a [[Topology]]-based model. [[Topology]] is a set of rules that model relationships between neighboring features, such as connectivity, area definition, and contiguity.

## Data Acquisition
Acquiring data is often the most time-consuming part of a GIS project. Data is categorized by its source:

- **[[Primary Data]]**: Data collected directly or on a firsthand basis (e.g., via GPS or total stations).
- **[[Secondary Data]]**: Data collected by another party (e.g., government census data or existing digital maps).

### Metadata and Standards
[[Metadata]] are "data about data," providing essential documentation regarding the contents, quality, and origin of a dataset. This is especially critical for [[Secondary Data]] to ensure transparency and promote data sharing.

### Data Management
Both models often interact with a [[Database Management System]] to store and organize the associated [[Attribute Data]].

## Relationships
- [[Geographic Data]] can be represented by [[Raster Data Model]]
- [[Geographic Data]] can be represented by [[Vector Data Model]]
- [[Raster Data Model]] is characterized by [[Spatial Resolution]]
- [[Vector Data Model]] can incorporate [[Topology]]
- [[Data]] becomes [[Information]] when put into context
- [[Geographic Data]] complements [[Attribute Data]]
- [[Secondary Data]] often accompanied by [[Metadata]]
- [[Primary Data]] is a method of collecting [[Geographic Data]]
- [[Secondary Data]] is a method of obtaining [[Geographic Data]]
- [[Vector Data Model]] links spatial features to [[Database Management System]]
- [[Raster Data Model]] can reference [[Database Management System]]
