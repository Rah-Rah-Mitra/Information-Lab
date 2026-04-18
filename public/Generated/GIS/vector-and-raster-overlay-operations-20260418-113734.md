---
type: content
title: "Vector and Raster Overlay Operations"
source: "GIS"
index_parent: "Sources/GIS.md"
created: 2026-04-18T11:37:34.461823400+00:00
summary: "An overview of spatial overlay techniques for vector and raster data, including Boolean operations, neighborhood analysis, and surface interpolation."
tags:
  - gis
  - spatial-analysis
  - vector-data
  - raster-data
  - geoprocessing
entities:
  - "[[Vector Overlay]]"
  - "[[Raster Overlay]]"
  - "[[Boolean Operators]]"
  - "[[Union]]"
  - "[[Intersection]]"
  - "[[Symmetrical Difference]]"
  - "[[Identity]]"
  - "[[Clip]]"
  - "[[Erase]]"
  - "[[Split]]"
  - "[[Spatial Join]]"
  - "[[Reclassification]]"
  - "[[Neighborhood Operations]]"
  - "[[Zonal Operations]]"
  - "[[Global Operations]]"
  - "[[Spatial Interpolation]]"
  - "[[Thiessen Polygons]]"
  - "[[Digital Elevation Model]]"
relationships:
  - source: "Vector Overlay"
    target: "Boolean Operators"
    description: "employs"
  - source: "Union"
    target: "Vector Overlay"
    description: "is a type of"
  - source: "Intersection"
    target: "Vector Overlay"
    description: "is a type of"
  - source: "Symmetrical Difference"
    target: "Vector Overlay"
    description: "is a type of"
  - source: "Identity"
    target: "Vector Overlay"
    description: "is a type of"
  - source: "Clip"
    target: "Vector Overlay"
    description: "is a geoprocessing option for"
  - source: "Erase"
    target: "Vector Overlay"
    description: "is a geoprocessing option for"
  - source: "Split"
    target: "Vector Overlay"
    description: "is a geoprocessing option for"
  - source: "Spatial Join"
    target: "Vector Overlay"
    description: "is a hybrid of attribute and"
  - source: "Raster Overlay"
    target: "Boolean Operators"
    description: "can employ"
  - source: "Reclassification"
    target: "Raster Overlay"
    description: "often serves as input for"
  - source: "Neighborhood Operations"
    target: "Raster Overlay"
    description: "is a scale of"
  - source: "Zonal Operations"
    target: "Raster Overlay"
    description: "is a scale of"
  - source: "Global Operations"
    target: "Raster Overlay"
    description: "is a scale of"
  - source: "Spatial Interpolation"
    target: "Digital Elevation Model"
    description: "is used to create"
---

# Vector and Raster Overlay Operations

*An overview of spatial overlay techniques for vector and raster data, including Boolean operations, neighborhood analysis, and surface interpolation.*

Vector and Raster Overlay operations are spatial analysis techniques used to combine multiple thematic maps to create new datasets. These operations allow GIS analysts to integrate spatial and attribute information from different layers based on their relative locations.

## Vector Overlay

[[Vector Overlay]] involves placing two or more vector layers on top of one another. Common models include point-in-polygon, line-in-polygon, and polygon-in-polygon. These operations often rely on [[Boolean Operators]] such as AND, OR, and XOR to determine the output.

### Common Vector Operations
- **[[Union]]**: Employs the OR operator to preserve all features and attributes from both input polygon layers.
- **[[Intersection]]**: Employs the AND operator; the output covers the spatial extent of the overlay and contains features from both layers.
- **[[Symmetrical Difference]]**: Employs the XOR operator to produce areas common to only one of the feature datasets.
- **[[Identity]]**: Creates an output with the spatial extent of the input layer but includes attributes from the identity (overlay) layer.

### Other Geoprocessing Options
- **[[Clip]]**: Extracts features from an input layer that fall within the spatial extent of a clip layer, without carrying over the clip layer's attributes.
- **[[Erase]]**: The opposite of clipping; it preserves only those areas outside the extent of the erase layer.
- **[[Split]]**: Divides an input layer into multiple layers based on a polygon split layer.
- **[[Spatial Join]]**: A hybrid operation that appends attributes from a source layer to a destination layer based on proximity or containment.

## Raster Overlay

[[Raster Overlay]] is generally less computationally intensive than vector overlay. It requires that all input rasters be coregistered, cover identical areas, and maintain equal resolution.

### Raster Analysis Types
- **Mathematical Overlay**: Performs user-specified mathematical transformations on aligned cells.
- **Boolean Overlay**: Uses AND, OR, and XOR to combine information into a binary output.
- **Relational Overlay**: Uses operators like < or > to evaluate conditions.

### Scales of Raster Analysis
- **[[Reclassification]]**: The process of assigning new class values to pixels based on original values, often as a first step in analysis.
- **[[Neighborhood Operations]]**: Uses moving windows (kernels) to calculate new cell values based on surrounding cells (e.g., for smoothing or edge enhancement).
- **[[Zonal Operations]]**: Analyzes groups of cells of similar value (zones) to measure geometry or summarize values.
- **[[Global Operations]]**: Treats the entire raster extent as a single zone to determine overall statistics.

## Surface Analysis

[[Spatial Interpolation]] is used to estimate values at unsampled locations based on nearby measurements, following Tobler's first law of geography. This is essential for creating a [[Digital Elevation Model]] (DEM).

### Interpolation Methods
- **Thiessen Polygons**: Also known as Voronoi polygons, these create a vector surface by defining the sphere of influence around each point.
- **Spline**: Forces a smoothed curve through known points.
- **Inverse Distance Weighting (IDW)**: Weights proximal values inversely to their distance from the target.
- **Trend Surface**: Fits a multivariate statistical regression model to the data.

## Relationships
- [[Vector Overlay]] employs [[Boolean Operators]].
- [[Union]], [[Intersection]], [[Symmetrical Difference]], and [[Identity]] are types of [[Vector Overlay]].
- [[Clip]], [[Erase]], and [[Split]] are geoprocessing options for [[Vector Overlay]].
- [[Spatial Join]] is a hybrid of attribute and [[Vector Overlay]].
- [[Raster Overlay]] can employ [[Boolean Operators]].
- [[Reclassification]] often serves as input for [[Raster Overlay]].
- [[Neighborhood Operations]], [[Zonal Operations]], and [[Global Operations]] are scales of [[Raster Overlay]].
- [[Spatial Interpolation]] is used to create [[Digital Elevation Model]].
