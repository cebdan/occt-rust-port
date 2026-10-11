# OCCT Rust Port

This repository is an autonomous, test-driven port of selected Open Cascade Technology (OCCT) functionality to Rust.

The project is intentionally structured in stages:

1. Geometry primitives (`gp`-style foundation)
2. Topological objects (`TopoDS`-style foundation)
3. Building solid primitives
4. Boolean operations
5. Mesh and conversion routines
6. Validation against reference OCCT behavior

## Current status

This repository currently contains the initial project scaffold and a minimal geometry foundation for future development.

## Goals

- Provide a Rust-native API for the subset of OCCT needed by the target application
- Preserve functional equivalence where possible
- Keep behavior verifiable via unit tests
- Decompose the port into autonomous, incremental milestones

## Architecture

```text
src/
  lib.rs
  core/
  geom/
  topo/
```

## Build

```bash
cargo test
```
