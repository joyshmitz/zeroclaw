# SOP Necessity Research (2026-02-19)

## Scope

Critical technical assessment of whether a SOP layer is justified for an industrial automation agent stack, and where LLM-based autonomy is acceptable vs unsafe.

## Facts In Favor Of SOP

- OSHA PSM requires written operating procedures for startup, normal operation, temporary operations, emergency shutdown, emergency operations, and normal shutdown.
- EPA RMP requires operating procedures and safe work practices for covered processes.
- ISA-106 exists specifically for procedural automation in process industries (startup/shutdown/abnormal situations), which validates SOP-driven orchestration as an engineering pattern.
- Incident history (for example BP Texas City, analyzed by CSB) shows procedure quality, training, and execution discipline directly affect risk.
- Human reliability in abnormal operations is limited; formalized procedure execution and check-gated automation reduce dependence on operator memory.
- NIST OT guidance emphasizes deterministic behavior, availability, and safe fallback; an explicit SOP layer can improve traceability and control boundaries.

## Facts Against Naive SOP+LLM Autonomy

- AI/LLM behavior is non-deterministic by design; identical input can produce different outputs. This conflicts with safety-critical control expectations.
- Government OT guidance (CISA/NSA/FBI/ACSC) warns against uncontrolled active AI control and recommends safe fallback to conventional automation/manual modes.
- High-risk control decisions should remain bounded by deterministic control systems (PLC/SIS), with AI in supervisory roles unless strict controls are proven.
- Hallucination/error behavior in state-of-the-art LLMs is non-zero; this is unacceptable as a sole decision layer for irreversible actions.
- SOP itself becomes a liability if change management, review cadence, training, and ownership are weak.

## Evidence From Current `feat/sop-foundation` Implementation

- SOP trigger `condition` fields are fully implemented and evaluated deterministically in runtime trigger matching via `evaluate_condition()`.
  - `src/sop/engine.rs` (`trigger_matches` function)
  - `src/sop/condition.rs` (evaluation logic)
- `SopAuditLogger` is implemented and wired into all SOP tool execution paths (execute, advance, approve).
  - `src/sop/audit.rs`
- Approval-timeout logic (`check_approval_timeouts`) is scheduled in the agent interactive runtime loop.
  - `src/agent/loop_.rs`

## Brutal Conclusion

- SOP as a deterministic orchestration, governance, and audit layer is technically justified for industrial operations.
- LLM direct control in safety-critical paths is not technically justified without hard deterministic guardrails, independent safety layers, and fail-safe design.
- Without those controls, this is not robust industrial automation; it is an operational risk multiplier.

## Sources

- OSHA PSM (29 CFR 1910.119): https://www.law.cornell.edu/cfr/text/29/1910.119
- EPA RMP (40 CFR 68.69): https://www.ecfr.gov/current/title-40/chapter-I/subchapter-C/part-68/subpart-D/section-68.69
- NIST SP 800-82r3: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-82r3.pdf
- CISA/NSA/FBI/ACSC AI Data Security in OT (2025): https://www.cisa.gov/sites/default/files/2025-03/Joint-Guidance-AI-Data-Security-SOTI.pdf
- CISA principles for secure AI deployment: https://www.cisa.gov/resources-tools/resources/principles-ai-software-development-and-deployment
- OpenAI o1 system card: https://openai.com/index/openai-o1-system-card/
- UK HSE control systems context: https://www.hse.gov.uk/comah/sragtech/techmeascontsys.htm
- ISA procedural automation context: https://www.isa.org/standards-and-publications/isa-publications/intech-magazine/2015/july-august/departments/procedural-automation-for-the-process-industries
- CSB BP Texas City: https://www.csb.gov/bp-america-refinery-explosion/ and https://www.csb.gov/chemical-safety-board-releases-new-safety-video-on-the-2005-bp-texas-city-refinery-disaster/
