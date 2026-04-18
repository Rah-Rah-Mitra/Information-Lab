---
title: "GIS Project Management Principles"
created: 2026-04-18T02:32:48.516847800+00:00
summary: "An overview of GIS project management, covering PMBOK process groups, knowledge areas, common tools like Gantt and PERT charts, and technical challenges."
tags:
  - gis
  - project-management
  - pmbok
  - cartography
entities:
  - "[[Project Management]]"
  - "[[PMBOK Guide]]"
  - "[[Project Manager]]"
  - "[[Initiation]]"
  - "[[Planning]]"
  - "[[Executing]]"
  - "[[Monitoring and Controlling]]"
  - "[[Closing]]"
  - "[[Project Integration Management]]"
  - "[[Project Scope Management]]"
  - "[[Project Time Management]]"
  - "[[Project Cost Management]]"
  - "[[Project Quality Management]]"
  - "[[Project Human Resource Management]]"
  - "[[Project Communication Management]]"
  - "[[Project Risk Management]]"
  - "[[Project Procurement Management]]"
  - "[[Gantt Chart]]"
  - "[[PERT Chart]]"
  - "[[Critical Path]]"
  - "[[Computer-Aided Design]]"
  - "[[Georeferencing]]"
  - "[[Grid-to-Ground Transformation]]"
  - "[[Scale Factor]]"
  - "[[Elevation Factor]]"
relationships:
  - source: "Project Management"
    target: "PMBOK Guide"
    description: "follows standards defined in"
  - source: "Project Manager"
    target: "Project Management"
    description: "implements"
  - source: "Project Management"
    target: "Initiation"
    description: "includes process group"
  - source: "Project Management"
    target: "Planning"
    description: "includes process group"
  - source: "Project Management"
    target: "Executing"
    description: "includes process group"
  - source: "Project Management"
    target: "Monitoring and Controlling"
    description: "includes process group"
  - source: "Project Management"
    target: "Closing"
    description: "includes process group"
  - source: "Project Management"
    target: "Project Integration Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Scope Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Time Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Cost Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Quality Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Human Resource Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Communication Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Risk Management"
    description: "includes knowledge area"
  - source: "Project Management"
    target: "Project Procurement Management"
    description: "includes knowledge area"
  - source: "Gantt Chart"
    target: "Project Time Management"
    description: "used for tracking in"
  - source: "PERT Chart"
    target: "Project Time Management"
    description: "used for tracking in"
  - source: "PERT Chart"
    target: "Critical Path"
    description: "identifies"
  - source: "Georeferencing"
    target: "Computer-Aided Design"
    description: "aligns data from"
  - source: "Grid-to-Ground Transformation"
    target: "Scale Factor"
    description: "utilizes"
  - source: "Grid-to-Ground Transformation"
    target: "Elevation Factor"
    description: "utilizes"
---

# GIS Project Management Principles

*An overview of GIS project management, covering PMBOK process groups, knowledge areas, common tools like Gantt and PERT charts, and technical challenges.*

[[Project Management]] is the application of knowledge, skills, tools, and techniques to project activities to meet stakeholder expectations, particularly challenging in GIS due to high storage and performance requirements.

## PMBOK Process Groups

The [[PMBOK Guide]] defines five overlapping process groups that structure the project lifecycle:
- **Initiation**: Defines and authorizes the project or phase, resulting in a project charter.
- **Planning**: An iterative process (often called "rolling wave planning") to define scope, resources, risks, and costs.
- **Executing**: The actual performance of the work defined in the planning phase.
- **Monitoring and Controlling**: Concurrent processes that observe the project and correct problems to ensure goals are met.
- **Closing**: The formal termination of activities and hand-off of deliverables.

## Project Management Knowledge Areas

Project managers must be proficient in nine key knowledge areas:
1. **[[Project Integration Management]]**: Coordinating various activities into a coherent whole.
2. **[[Project Scope Management]]**: Defining what work is required and what should be excluded.
3. **[[Project Time Management]]**: Analyzing time constraints and developing schedules.
4. **[[Project Cost Management]]**: Budgeting and maintaining financial constraints.
5. **[[Project Quality Management]]**: Satisfying quality standards through planning and assurance.
6. **[[Project Human Resource Management]]**: Acquisition and oversight of team members.
7. **[[Project Communication Management]]**: Maintaining open lines of communication with stakeholders.
8. **[[Project Risk Management]]**: Identifying and mitigating potential risks.
9. **[[Project Procurement Management]]**: Acquiring products or services from outside the project team.

## Tools and Techniques

### Scheduling
Project managers use specific tools to track progress:
- **[[Gantt Chart]]**: A bar chart focusing on start and completion dates and task dependencies, best for small, linear projects.
- **[[PERT Chart]]**: Focuses on project events and the [[Critical Path]] (the longest potential duration sequence), preferred for very large projects.

### Technical Integration
- **[[Computer-Aided Design]] (CAD)**: Graphics-based mapping often used in engineering. Unlike GIS, CAD historically lacked attribute links, though "smart" features are emerging.
- **[[Georeferencing]]**: The process of aligning local CAD data (often using local coordinate systems) with georeferenced GIS datasets.

### Grid-to-Ground Transformation
To move from in-program "grid" measurements to real-world "ground" measurements, managers must account for:
- **[[Scale Factor]]**: Corrects scale error associated with the move from 3D earth to 2D grid plane.
- **[[Elevation Factor]]**: Corrects elevation error as project site altitude increases.

## Relationships
- [[Project Management]] follows standards defined in [[PMBOK Guide]].
- [[Project Manager]] implements [[Project Management]].
- [[Project Management]] includes process groups: [[Initiation]], [[Planning]], [[Executing]], [[Monitoring and Controlling]], and [[Closing]].
- [[Project Management]] includes knowledge areas: [[Project Integration Management]], [[Project Scope Management]], [[Project Time Management]], [[Project Cost Management]], [[Project Quality Management]], [[Project Human Resource Management]], [[Project Communication Management]], [[Project Risk Management]], and [[Project Procurement Management]].
- [[Gantt Chart]] and [[PERT Chart]] are used for tracking in [[Project Time Management]].
- [[PERT Chart]] identifies the [[Critical Path]].
- [[Georeferencing]] aligns data from [[Computer-Aided Design]].
- [[Grid-to-Ground Transformation]] utilizes [[Scale Factor]] and [[Elevation Factor]].
