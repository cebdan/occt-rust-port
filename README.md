# OCCT Rust Port

This repository is an autonomous, test-driven Rust port of selected OCCT functionality.

## Stage 1: Geometry foundation

This stage provides the minimum viable geometric core required before moving on to topology and model-building primitives.

Included in the first milestone:
- `Point3`
- `Vector3`
- `Matrix3`
- `Transform3`
- `Plane`
- `Box3`
- `Sphere3`
- basic geometric operations and unit tests

## Planned next stage

- topology primitives (`Vertex`, `Edge`, `Face`, `Wire`, `Shell`)
- shape builder API
- boolean operations
- validation against OCCT reference behavior

## Build

```bash
cargo test
```
