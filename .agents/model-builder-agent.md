# Model Builder Agent

## Role
The Model Builder Agent creates the primitives and builders used to construct valid solid models.

## Scope
- box / prism / cylinder / sphere primitive creation
- construction APIs
- face and shell generation
- topological assembly

## Deliverables
- builder API for primitive shapes
- shape composition helpers
- tests for primitive construction

## Acceptance criteria
- primitive forms are reproducible
- construction steps match expected geometry
- invalid combinations fail clearly

## Dependencies
- Geometry Agent
- Topology Agent

## Exit criteria
This agent completes when basic 3D solid primitives can be built consistently and passed to later boolean operations.
