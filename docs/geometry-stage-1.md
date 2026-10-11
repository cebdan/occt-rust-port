# Geometry Agent - Stage 1

This stage establishes the numerical and geometric core needed before shape topology can be modeled.

## Implemented primitives

- `Point3`
- `Vector3`
- `Matrix3`
- `Transform3`
- `Plane`
- `Box3`
- `Sphere3`
- `Line3`

## Behavioral guarantees

- elementary geometric math is deterministic
- zero vectors are handled safely
- transform operations preserve expected geometry
- tests cover common cases and degenerate path

## Exit criteria for Stage 1

The geometry layer is complete when all tests pass and the resulting primitives can act as the foundation for topology modeling in Stage 2.
