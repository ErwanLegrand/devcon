# Plan: Architecture Review & STRIDE Threat Modelling

## Phase 1: Architecture Review

- [x] Task: Read and map the full source structure
    - [x] Read all files under `src/` to build a complete picture of modules, types, and call flows
    - [x] Diagram (in text) the data flow: CLI → Settings → Config → build_provider → Devcontainer → Provider → container engine
    - [x] Document module responsibilities and boundary definitions

- [x] Task: Evaluate architecture concerns
    - [x] Module structure: devcontainers/ vs provider/ boundary
    - [x] Provider trait design: abstraction level, leaky abstractions
    - [x] Config parsing pipeline: parse → validate → dispatch
    - [x] Error propagation strategy (std::io::Error throughout, swallowed errors)
    - [x] Lifecycle hook ordering and failure semantics
    - [x] Settings/config layering

## Phase 2: STRIDE Threat Modelling

- [x] Task: Enumerate threats for each STRIDE category
    - [x] Spoofing — identity/trust assumptions
    - [x] Tampering — integrity of config, temp files, images
    - [x] Repudiation — auditability of container operations
    - [x] Information Disclosure — SSH socket, env vars, logs
    - [x] Denial of Service — resource exhaustion, infinite loops
    - [x] Elevation of Privilege — DooD socket, setuid, volume mounts

- [x] Task: Rate each threat and propose mitigations
    - [x] Assigned likelihood and impact ratings to all 20 threats
    - [x] Proposed mitigations for each
    - [x] Grouped mitigations into implementable follow-up tracks

## Phase 3: Report

- [x] Task: Write the full report to `conductor/tracks/arch_review_stride_20260309/report.md`
    - [x] 9 architecture findings (High/Medium/Low)
    - [x] 20 STRIDE threats enumerated with ratings
    - [x] Prioritised action list with suggested track IDs
    - [x] Commit: `docs(conductor): add architecture review and STRIDE threat model report` [72e18ef]

- [ ] Task: Conductor - User Manual Verification 'Architecture Review & STRIDE Report' (Protocol in workflow.md)
