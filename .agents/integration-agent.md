# Integration Agent

## Role
The Integration Agent ensures the Rust port remains usable from external applications.

## Scope
- public Rust API design
- ergonomics and naming consistency
- re-export patterns
- compatibility layer and examples
- external integration tests

## Deliverables
- stable public crate interface
- usage examples
- migration guidance for C++ users

## Acceptance criteria
- public API is coherent and documented
- examples compile cleanly
- there is a path from the internal modules to end-user usage

## Dependencies
- all functional agents

## Exit criteria
The Integration Agent completes when the crate exposes usable, coherent Rust APIs that are testable from user code.
