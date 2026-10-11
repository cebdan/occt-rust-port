# Stage 1 Execution Plan

This repository is currently at Stage 1: Geometry Foundation.

## Current objective
Build a reliable 3D geometry core that can be extended safely by the Topology Agent.

## Required outputs
- `Point3`
- `Vector3`
- `Matrix3`
- `Transform3`
- `Plane`
- `Box3`
- `Sphere3`
- numeric tests for distance, dot/cross products, transforms, and volumes

## Definition of done
1. The geometry module compiles without warnings in release builds.
2. Unit tests cover normal and edge cases.
3. The API is explicit and minimal.
4. The next stage can use this module without redesign.
