# Test & Validation Agent

## Role
The Test & Validation Agent owns the correctness suite and regression safety net.

## Scope
- unit tests
- golden datasets
- numerical tolerance rules
- regression checks against reference behavior
- validation logs and failure triage

## Deliverables
- `tests/` suite
- validation report format
- acceptance criteria for each stage

## Acceptance criteria
- all tests pass for the current stage
- edge-case regressions are recorded
- numerical tolerances are explicit
- failures identify the exact stage and cause

## Dependencies
- all other agents

## Exit criteria
This agent is complete when every stage has explicit pass/fail gates and clear regression coverage.
