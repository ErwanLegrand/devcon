# Plan: Architecture Review & STRIDE Threat Modelling

## Phase 1: Architecture Review

- [ ] Task: Read and map the full source structure
    - [ ] Read all files under `src/` to build a complete picture of modules, types, and call flows
    - [ ] Diagram (in text) the data flow: CLI → Settings → Config → build_provider → Devcontainer → Provider → container engine
    - [ ] Document module responsibilities and boundary definitions

- [ ] Task: Evaluate architecture concerns
    - [ ] Module structure: devcontainers/ vs provider/ boundary
    - [ ] Provider trait design: abstraction level, leaky abstractions
    - [ ] Config parsing pipeline: parse → validate → dispatch
    - [ ] Error propagation strategy (std::io::Error throughout, swallowed errors)
    - [ ] Lifecycle hook ordering and failure semantics
    - [ ] Settings/config layering

## Phase 2: STRIDE Threat Modelling

- [ ] Task: Enumerate threats for each STRIDE category
    - [ ] Spoofing — identity/trust assumptions
    - [ ] Tampering — integrity of config, temp files, images
    - [ ] Repudiation — auditability of container operations
    - [ ] Information Disclosure — SSH socket, env vars, logs
    - [ ] Denial of Service — resource exhaustion, infinite loops
    - [ ] Elevation of Privilege — DooD socket, setuid, volume mounts

- [ ] Task: Rate each threat and propose mitigations
    - [ ] Assign likelihood (High/Med/Low) and impact (High/Med/Low) to each threat
    - [ ] Write a concrete mitigation for each
    - [ ] Group mitigations into implementable follow-up tracks

## Phase 3: Report

- [ ] Task: Write the full report to `conductor/tracks/arch_review_stride_20260309/report.md`
    - [ ] Architecture findings section (severity-rated)
    - [ ] STRIDE threat table
    - [ ] Prioritised action list with suggested track IDs
    - [ ] Commit: `docs(conductor): add architecture review and STRIDE threat model report`

- [ ] Task: Conductor - User Manual Verification 'Architecture Review & STRIDE Report' (Protocol in workflow.md)
