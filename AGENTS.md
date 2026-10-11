# Project Agents

This repository uses role-based autonomous agents for incremental porting of OCCT features to Rust.

Each agent owns a well-defined scope, passes artifacts between phases, and is validated by tests before handoff.

## Agent list

1. Geometry Agent
2. Topology Agent
3. Model Builder Agent
4. Boolean Ops Agent
5. Test & Validation Agent
6. Integration Agent
7. Benchmark & Performance Agent

Each agent has:
- responsibility
- input artifacts
- output artifacts
- acceptance criteria
- escalation policy

## Execution flow

1. Geometry Agent creates primitives and mathematical foundations.
2. Topology Agent builds shape/container abstractions on top of geometry.
3. Model Builder Agent creates primitive solids and construction APIs.
4. Boolean Ops Agent implements set operations and topological validation.
5. Test & Validation Agent verifies results against reference behavior.
6. Integration Agent ensures external API compatibility and ergonomics.
7. Benchmark Agent measures performance, memory, and numerical stability.

## Handoff contract

Every agent must leave behind:
- source code
- tests
- documentation for the module
- known limitations
- references to the original OCCT behavior being mirrored

No agent may move to the next stage without passing tests.
