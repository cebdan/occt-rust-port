# Boolean Ops Agent

## Role
The Boolean Ops Agent implements set operations on solids and their topological representations.

## Scope
- union, intersection, difference
- face/edge trimming rules
- invalid-geometry handling
- boolean result validation

## Deliverables
- boolean operation module
- result validation helpers
- tests comparing expected solid behavior

## Acceptance criteria
- all supported operations produce valid topology or an explicit failure result
- degenerate intersections are handled gracefully
- failure modes are deterministic and test-covered

## Dependencies
- Model Builder Agent
- Test & Validation Agent

## Exit criteria
The Boolean Ops Agent completes only when the library can build a small set of valid, test-covered CAD operations.
