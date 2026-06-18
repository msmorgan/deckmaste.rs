# Architecture Review — 2026-06-18

## Overview
A comprehensive review of the `deckmaste.rs` engine and taxonomy was performed to identify systemic issues, false assumptions, and technical debt.

## Key Findings

### 1. The "Literalism" Bottleneck in Layers
**Status: CRITICAL**
The layer engine (`layer.rs`) currently only evaluates `Count::Literal`. This is a significant false assumption that breaks:
*   **CDAs (Characteristic-Defining Abilities):** Tarmogoyf, Nightmare, and other P/T-setting abilities resolve to 0.
*   **Dynamic Pumps:** "Gets +1/+1 for each [X]" resolves to +0/+0.
*   **Layer 7 Resolution:** The sub-layer system (7a-7d) is largely non-functional for any non-fixed values.
*   **Ticket:** `critical/engine-layers-dynamic-counts`

### 2. Subtype Modification Bug (Layer 4)
**Status: CRITICAL**
Modification of subtypes is currently a no-op stub because the `Modification` enum carries `Ident` (strings), but derived `Characteristics` expect `Subtype` structs. This prevents any effect from adding or changing subtypes correctly (e.g., Changeling, basic land type changes).
*   **Ticket:** `critical/layers-layer-4-subtypes`

### 3. Static Ability Gathering (No Fixpoint)
**Status: PLANNED**
The engine currently only gathers static abilities from *printed* characteristics to avoid recursion. This prevents "nested lords" or granted static abilities from functioning. A fixpoint iteration is required to correctly re-gather abilities after Layer 6 has applied.
*   **Ticket:** `planned/engine-layers-fixpoint`

### 4. Macro Composition Opportunities
**Status: PLANNED**
Several keyword actions (e.g., `Amass`, `Explore`) are being treated as primitives or are unimplemented. These should be decomposed into compositions of smaller representable operations using the `macro-modification-bundle` infrastructure.
*   **Ticket:** `planned/macro-amass`

### 5. Loop Detection & Game State Equality (UD-11)
**Status: MAYBE**
The engine lacks a way to detect mandatory loops ([CR#104.4b]). This requires a formal definition of `GameState` equality or a stable event-sequence hash.
*   **Ticket:** `maybe/rules-state-equality`

### 6. LKI Snapshot Technical Debt (UD-9)
**Status: MAYBE**
The field-enumerated `LkiSnapshot` is efficient but fragile. As the engine grows, this manual enumeration will become a maintenance bottleneck.
*   **Ticket:** `maybe/engine-lki-robustness`

## Action Plan
1.  **Phase 1 (Immediate):** Fix `eval_count` and Layer 4 subtypes. These are foundational blockers for ~30% of card mechanics.
2.  **Phase 2 (Intermediate):** Implement fixpoint iteration for layers and begin graduating composite macros like `Amass`.
3.  **Phase 3 (Long-term):** Address UD-11 and refactor LKI snapshots for better long-term maintainability.
