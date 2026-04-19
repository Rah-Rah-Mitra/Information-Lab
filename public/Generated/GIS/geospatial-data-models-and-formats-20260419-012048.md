---
type: content
title: "Geospatial Data Models and Formats"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-19T01:20:48.596660400+00:00
summary: "This note explores the fundamental data models—raster and vector—and the various file formats used to store geospatial information. It explains how these models differ in structural complexity, precision, and analytical utility. The discussion also covers database management and the capture of primary and secondary data."
tags:
  - gis
  - geospatial-data
  - data-models
  - remote-sensing
  - database-management
entities:
  - "[[Raster Data Model]]"
  - "[[Vector Data Model]]"
  - "[[Topology]]"
  - "[[Spatial Resolution]]"
  - "[[Spatial Join]]"
  - "[[Relational Database Management System]]"
  - "[[Shapefile]]"
  - "[[Triangulated Irregular Network]]"
  - "[[Satellite Imagery]]"
  - "[[Geodatabase]]"
relationships:
  - source: "Raster Data Model"
    target: "Spatial Resolution"
    description: "is characterized by"
  - source: "Vector Data Model"
    target: "Topology"
    description: "can incorporate"
  - source: "Vector Data Model"
    target: "Triangulated Irregular Network"
    description: "includes"
  - source: "Relational Database Management System"
    target: "Shapefile"
    description: "is distinct from"
  - source: "Satellite Imagery"
    target: "Raster Data Model"
    description: "is a form of"
  - source: "Geodatabase"
    target: "Vector Data Model"
    description: "supports"
  - source: "Geodatabase"
    target: "Raster Data Model"
    description: "supports"
---

# Geospatial Data Models and Formats

*This note explores the fundamental data models—raster and vector—and the various file formats used to store geospatial information. It explains how these models differ in structural complexity, precision, and analytical utility. The discussion also covers database management and the capture of primary and secondary data.*

Geospatial information is represented through two primary paradigms: the [[Raster Data Model]] and the [[Vector Data Model]].

## Concept

The ### Raster Data Model

The [[Raster Data Model]] consists of a grid-based system of contiguous, equally sized cells or pixels. Each cell carries a single value representing a characteristic of a spatial phenomenon. The precision of this model is governed by its [[Spatial Resolution]], which is the area covered by each pixel. Coarse resolution leads to loss of information, while fine resolution increases computational requirements.

Common encoding methods include cell-by-cell, run-length, and quad-tree encoding. [[Satellite Imagery]] and aerial photography are primary examples of this model, often used for contextual background or heads-up digitizing.

### Vector Data Model

The [[Vector Data Model]] represents space using discrete geometric entities: [[Point]], [[Line]], and [[Polygon]]. Unlike the raster model, vector data can provide much higher precision and is often more aesthetically pleasing. 

Vector structures can be simple, such as the [[Spaghetti Data Model]], which lacks explicit [[Topology]], or complex, such as the [[Topological Data Model]]. In a [[Topological Data Model]], the system explicitly encodes relationships between neighboring features, such as connectivity, area definition, and contiguity. This allows for efficient network analysis and error detection (e.g., identifying [[Sliver]] polygons or [[Dangling Node]] errors).

One sophisticated vector structure for representing surfaces is the [[Triangulated Irregular Network]] (TIN), which uses non-overlapping triangles to model elevation. This is more efficient than a raster-based elevation model in areas with simple relief.

## Data Management and Formats

Geospatial data is often managed within a [[Relational Database Management System]] (RDBMS). In an RDBMS, tables are linked via primary and foreign keys to reduce redundancy. Advanced hybrid formats like the [[Geodatabase]] can store both vector and raster datasets in a single file.

Common file formats include:
- **Vector**: [[Shapefile]], [[ArcInfo Coverage]], and [[TIGER/Line]] files.
- **Raster**: [[JPEG]], [[TIFF]], [[USGS DEM]], and [[DTED]].
- **Hybrid**: [[Geodatabase]] and [[KML]].

## Relationships
- [[Raster Data Model]] is characterized by [[Spatial Resolution]]
- [[Vector Data Model]] can incorporate [[Topology]]
- [[Vector Data Model]] includes [[Triangulated Irregular Network]]
- [[Relational Database Management System]] is distinct from [[Shapefile]]
- [[Satellite Imagery]] is a form of [[Raster Data Model]]
- [[Geodatabase]] supports [[Vector Data Model]]
- [[Geodatabase]] supports [[Raster Data Model]]
