# Geometry Agent

## Role
The Geometry Agent owns the foundational math layer required by the OCCT port.

## Scope
- 3D points and vectors
- matrices and transforms
- coordinate space conventions
- plane and bounding primitives
- numeric stability rules
- geometry helper functions

## Deliverables
- `src/geom/geometry.rs` and related modules
- unit tests for each primitive and operation
- documentation for numeric conventions

## Acceptance criteria
- all geometry tests pass
- no panic on degenerate inputs where behavior is defined
- distance, normalization, and transforms are numerically stable
- APIs are small, explicit, and deterministic

## Dependencies
- Core math conventions
- Test harness

## Exit criteria
The Geometry Agent completes when:
- `Point3`, `Vector3`, `Matrix3`, `Transform3` are stable
- plane and box/sphere primitives exist
- tests cover common and edge-case inputs
- the next agent can build topology on top of this layer
