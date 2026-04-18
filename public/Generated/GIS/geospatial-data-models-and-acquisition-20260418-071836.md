---
type: content
title: "Geospatial Data Models and Acquisition"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T07:18:36.823782300+00:00
summary: "An overview of raster and vector data models, their structural differences, and the methodologies for primary and secondary geospatial data acquisition."
tags:
  - gis
  - geospatial-data
  - data-models
  - remote-sensing
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
  - "[[Satellite Imagery]]"
  - "[[Aerial Photography]]"
  - "[[Measurement Scale]]"
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
  - source: "Primary Data"
    target: "Geographic Data"
    description: "is a method of collecting"
  - source: "Secondary Data"
    target: "Geographic Data"
    description: "is a method of collecting"
  - source: "Satellite Imagery"
    target: "Raster Data Model"
    description: "is an implementation of"
  - source: "Aerial Photography"
    target: "Raster Data Model"
    description: "is an implementation of"
  - source: "Metadata"
    target: "Data"
    description: "provides information about"
  - source: "Measurement Scale"
    target: "Attribute Data"
    description: "categorizes the complexity of"
---

# Geospatial Data Models and Acquisition

*An overview of raster and vector data models, their structural differences, and the methodologies for primary and secondary geospatial data acquisition.*

Geospatial data management involves the distinction between raw facts and the knowledge derived from them, as well as the selection of appropriate models to represent the earth's surface.

## Data and Information
[[Data]] refer to raw facts, measurements, or characteristics of objects. When these data are put into context or analyzed to obtain insights, they become [[Information]]. In a GIS environment, a critical distinction is made between [[Geographic Data]] (spatial data that defines the location of an object on the earth's surface) and [[Attribute Data]] (nongeographic traits and characteristics of that object).

## Geospatial Data Models
To represent geographic space in a computer, two primary data models are used:

### Raster Data Model
The [[Raster Data Model]] consists of a grid of equally sized pixels. Each cell carries a single value representing a spatial phenomenon. The accuracy of this model is defined by its [[Spatial Resolution]], which is the area covered by a single pixel. Common implementations include [[Satellite Imagery]] and [[Aerial Photography]].

### Vector Data Model
The [[Vector Data Model]] uses points, lines, and polygons defined by X, Y coordinate pairs. Unlike the raster model, it can be topologically explicit. [[Topology]] is a set of rules that model the relationships between neighboring features, such as connectivity, area definition, and contiguity.

## Data Acquisition
Data can be acquired through two main methodologies:

- **[[Primary Data]]**: Collected directly via firsthand observation or measurement (e.g., GPS surveying or remote sensing).
- **[[Secondary Data]]**: Obtained from existing digital or hard-copy sources (e.g., government databases or digitizing paper maps).

## Data Management and Quality
[[Metadata]] (data about data) is essential for the transparency and integration of datasets, particularly [[Geospatial Metadata]] which documents digital resources like GIS files. Furthermore, [[Attribute Data]] is often categorized by a [[Measurement Scale]] (nominal, ordinal, interval, or ratio) to determine the level of complexity and the types of arithmetic operations possible.
