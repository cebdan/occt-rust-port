# Topology Agent

## Role
The Topology Agent defines the OCCT-like shape hierarchy and relationship model.

## Scope
- vertices, edges, faces, wires, shells, solids
- shape IDs and ownership
- topology relationships and adjacency
- traversal helpers

## Deliverables
- `src/topo/` module tree
- shape representation types
- tests for relation consistency

## Acceptance criteria
- shape graph is deterministic
- orientation and basic adjacency rules are well-defined
- impossible states are rejected or encoded explicitly
- tests verify basic topology invariants

## Dependencies
- Geometry Agent output

## Exit criteria
The Topology Agent completes when the system can represent a minimal CAD-like shape graph and the next stage can compose solids from it.
