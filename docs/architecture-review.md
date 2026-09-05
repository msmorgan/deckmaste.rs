# Architecture Review — 2026-06-18

> Status reviewed 2026-07-14. This document preserves the findings from the
> original review and records what has changed since then. Ticket paths below
> are current; completed work lives under `docs/tickets/done/`.

## Overview

A comprehensive review of the `deckmaste.rs` engine and taxonomy was performed
to identify systemic issues, false assumptions, and technical debt. Four of its
six findings have since been implemented. The two remaining findings are open
design investigations, not known correctness regressions on the implemented
card slice.

## Key Findings

### 1. Dynamic Counts in Layers

**Status: RESOLVED**

The review found that the layer engine evaluated only `Count::Literal`, which
made characteristic-defining abilities and other dynamic power/toughness
effects resolve incorrectly. The layer evaluator now handles the broader count
language against the working derived-characteristics map, including object
counts and derived statistics. Regression coverage includes a dynamic CDA and a
dynamic layer-7c pump.

- **Completed ticket:** `done/engine-layers-dynamic-counts`
- **Implementation:** `crates/deckmaste_engine/src/layer.rs::eval_count`

### 2. Subtype Modification in Layer 4

**Status: RESOLVED**

The review found that subtype modifications were a no-op because modifications
carried subtype names while derived characteristics stored full `Subtype`
values. Layer 4 now resolves names through the loaded subtype registry, applies
set/add/remove operations, and carries type- or subtype-conferred rules into the
derived ability list. Tests cover registry resolution, additive changes,
deduplication, removal, and conferred abilities.

- **Completed ticket:** `done/layers-layer-4-subtypes`
- **Implementation:** the `Modification::Subtypes` arm and
  `fold_conferred_abilities` in `crates/deckmaste_engine/src/layer.rs`

Two narrower cases remain explicitly deferred in the implementation:
`BecomeBasicLandType` and the every-creature-type expansion used by changeling.
Those are separate grammar/registry concerns rather than the original subtype
modification no-op.

### 3. Static-Ability Gathering Fixpoint

**Status: RESOLVED**

The original engine gathered continuous effects only from printed abilities to
break a recursion cycle. It now performs bounded whole-pass iteration: the first
pass gathers printed abilities, later passes gather from the preceding derived
view, and derivation repeats from base until the effect set stabilizes. Tests
cover granted static abilities, self-referential grants, and multi-object
convergence.

- **Completed ticket:** `done/engine-layers-fixpoint`
- **Implementation:** `GameState::layers` in
  `crates/deckmaste_engine/src/layer.rs`

### 4. Macro Composition Opportunities

**Status: RESOLVED FOR AMASS; ONGOING AS A DESIGN POLICY**

Amass is now a composite `Instruction` macro rather than a new engine
primitive. Its definition composes the existing condition, token creation,
choice, counter, and continuous-modification vocabulary and is checked by a
structural expansion test. Other keyword actions should continue to be assessed
individually under the repository's keyword policy; the completion of Amass
does not imply that every composite keyword has been authored.

- **Completed ticket:** `done/macro-amass`
- **Definition:** `plugins/builtin/macros/action/Amass.ron`

### 5. Loop Detection and Game-State Equality (UD-11)

**Status: OPEN INVESTIGATION**

The engine still has no mandatory-loop monitor for the draw prescribed by
[CR#104.4b]. Implementing one requires an explicit equivalence relation over
game states or event sequences. The rules require loop recognition but do not
define which state components are relevant to equality, so this remains a real
engine-design decision rather than a straightforward missing match arm.

- **Open ticket:** `maybe/rules-state-equality`
- **Dependency:** the mtg-rules `UD-11` game-state-equality decision

### 6. LKI Snapshot Technical Debt (UD-9)

**Status: OPEN INVESTIGATION**

The field-enumerated `LkiSnapshot` remains efficient but fragile: each new
last-known-information consumer may require another field and population path.
The open question is whether to retain that minimal representation or snapshot
a broader object record while preserving garbage-collection freedom and
avoiding live references.

- **Open ticket:** `maybe/engine-lki-robustness`
- **Dependency:** the mtg-rules `UD-9` snapshot-boundary decision

## Current Priorities

1. Keep the resolved layer behavior covered as the count and subtype
   vocabularies expand, especially the explicitly deferred basic-land-type and
   changeling cases.
2. Continue expressing composite mechanics as macros where the existing action
   and effect vocabulary is sufficient.
3. Resolve UD-11 before implementing mandatory-loop detection, and revisit the
   LKI representation when a concrete consumer exposes a missing snapshot
   property.
