---
needs: []
---
**Remove the conferrer bookkeeping around designations.** Ruling (user,
2026-09-07): which keyword confers a designation is not a fact worth
tracking; a keyword's definition body grants the designation, and that is
the whole fact ([CR#701.37a] for monstrosity). Today the same fact is kept
three times and checked for agreement: `Instruction.gainDesignation` carries
a `Conferral` tag (`instructed` / `byKeyword k` / `byDeed d`,
`lean/Semantics/Words.lean`), the keyword and deed facts rows carry a
`confers` column (`Check/FactTypes.lean`, `Facts.lean`), and each
`DesignationDecl` under `plugins_v2/builtin/macros/stubs/designations/`
declares `conferrers`, which xtask inverts into the `confers` columns
through v1's `deckmaste_semantics::DesignationConferrer`
(`crates/xtask/src/facts/lean.rs`). `conferralOk` (`Check/Keywords.lean`)
compares them.

Remove all of it, standard constraints applying:

- `Conferral` and the argument on `gainDesignation` go; every macro and
  card that passes one is re-spelled (`Macros.lean` keyword expansions,
  `Cards/`, `Proofs/`); the Rust mirror follows (drift test).
- `confers` leaves the keyword-ability, keyword-action, and deed facts
  (`FactTypes.lean`, the overlays, `coreDeedConferrals`); `conferrers`
  leaves `DesignationDecl` and every declaration file; xtask's facts
  generator drops the `deckmaste_semantics` import (it must not depend on
  a deletion-bound crate for v2 data). Regenerate; `facts check` clean. The
  Idris reference table's column is emitted as before from nothing, or the
  reference emitter is told the column is gone; say which.
- `conferralOk` reduces to the surviving rules-meaningful read: an
  instructed grant needs a `checked` (effectful) designation, per the
  designation's own row. Pins that asserted a conferrer mismatch are
  re-spelled against that law; positives keep proving.
- `docs/decisions/conferrals-come-from-registries.md` gets a dated note:
  registries carry what a designation is, not who confers it.

## Landing record

Measured on change `yplxmxxo` (`b20c9e4f`), 24-core host, load average 16.5.

### PROVE

**No silent loss.** Nothing stopped being covered without a named replacement.
Every identity that lost its old spelling is listed under DISCLOSE with the
theorem that now carries its surviving content. Three tests were removed; each
is named and justified below (wrong analysis retired, not a regression). No
coverage lock is in play for this lane (`facts check` is the generated-data
gate; it is clean).

**Structural laws.** `lean/scripts/build` (`lake build --wfail`, the full gate:
syntax, checker, bench, every pin suite) completes with 80/80 jobs and zero
warnings from a deleted `.lake/build`: **141 s wall**, load average 16.5, 24
cores. `cargo test -p deckmaste_semantics_v2 --test lean_drift` 4/4 green, so
the Rust mirror matches the Lean syntax declaration-for-declaration after the
constructor lost its argument. `cargo xtask facts check` reports both generated
modules up to date. `cargo xtask lean-check` proves 24/24 canon and 2/2 testing
cards (`Card.check = []`), 4.5 s; no baseline.

**No word-naming.** The surviving guard reads a declared feature and nothing
else: `refuse d.checked (.designationChecked d)`, where `DesignationLabel.checked`
is `(label.facts.map (·.effectful)).getD false` — a lookup in the generated
`designationTable`. The removed code was the opposite: `conferralOk` matched a
`Conferral` tag naming a `KeywordLabel` or a `Deed` and compared it against a
generated `confers` list. No lexeme, construction, verb, noun, preposition or
card identity is named by any guard this landing leaves behind.

### DISCLOSE

**What the four ticket bullets did.**

1. `Conferral` (`lean/Semantics/Words.lean`) and the `conferral` argument on
   `Instruction.gainDesignation` (`lean/Semantics/Abilities.lean`) are gone,
   with the Rust mirror (`crates/deckmaste_semantics_v2/src/words.rs`,
   `abilities.rs`) following in the same landing. Seven term sites re-spelled:
   `Macros.lean` ×2 (`makeMonstrous`, the Renown expansion), `Cards/Anaphora`,
   `Cards/Cost`, `Cards/Counters`, `Cards/Trigger` ×2.
2. `confers` left `ActFacts` and `KeywordFacts` (`Check/FactTypes.lean`);
   `coreDeedConferrals` is no longer generated, so `coreDeedFacts` and the
   private `coreDeedRuleFacts` it wrapped collapsed into one public
   `coreDeedFacts` (`Check/Words.lean`). `conferrers` left `DesignationDecl`
   and all twelve declaration files under
   `plugins_v2/builtin/macros/stubs/designations/`.
   `cargo xtask facts generate` re-emitted both modules; `facts check` clean.
3. `conferralOk` is gone; `Instruction.check`'s `.gainDesignation` arm now
   refuses on `d.checked`, the same read its `.setGameDesignation` sibling
   already made.
4. `docs/decisions/conferrals-come-from-registries.md` carries a dated
   amendment (2026-09-07) with its [CR#701.37a] justification.

**The Idris reference table's column — which.** Neither. The generated Idris
module `idris/src/Experimental/FactsGen.idr` never carried a `confers` column;
only the Lean module did. The generator emits it byte-identically before and
after (`facts check`: "is up to date"), so the reference emitter needed no
change and nothing is emitted from nothing. The hand-written Idris reference
`idris/src/Experimental/Words.idr` still declares the historical
`conferredDesignation` / `GivingWarrant` family; it is frozen historical
evidence under the workbench-succession note and was left alone.
`lean/CONTRACTS.md`'s `GivingWarrant` row now records the retirement instead of
the old correspondence.

**Assurance counts.**

- Restored: 0.
- Re-spelled: 14 sites. Seven card/macro terms (above). Seven pins:
  `Proofs/Zone.lean` — `okGoadOnBattlefield`, `badGoadInGraveyard` (tag
  dropped, asserted refusal lists unchanged); `Proofs/Static.lean` —
  `okBecomesMonarch`, `badGoadedPlayer` (tag dropped),
  `okMonstrousByDeed` → `okBecomesMonstrous`,
  `okEnduringStoryByStoried` → `okGetsAnEnduringStory` (same card, same `= []`,
  new spelling), `badMonstrousByKeyword` → `badGainsCommander`.
- Consolidated: `Proofs/Tables.lean`'s five conferrer pins became four, none
  deleted without its surviving content re-asserted:
  `spaceSculptorConfersEverySector` → `everySectorLabelIsAnEffectfulDesignation`
  (the enum designation still reaches the table under all three members);
  `unlockConfersEitherDoor` **and** `coreFactsIncludeTheirDeclaredConferrals`
  → `bothDoorLabelsAreDeclaredHalves` (both asserted that the two door labels
  are exactly `["left half unlocked", "right half unlocked"]`; that list is now
  read off the surviving `half` column, in order);
  `storiedDoesNotConferCitysBlessing` → `aRuleOnlyDesignationIsNotEffectful`;
  `unlockDoesNotConferSector` → `anUndeclaredSectorIsNotADesignation`.
- Ignored with blockers: 0.
- Added: 0 pins beyond the re-spellings and the consolidation.
- Removed: 3, each justified.
  - `crates/xtask/src/facts/lean.rs::declaration_conferrer_changes_reach_the_generated_keyword_row`
    and `::undeclared_conferrers_are_rejected` — both drive the retired
    `conferrers` field. The surviving property they also touched ("a
    declaration edit reaches the generated row") stays covered by
    `declared_parameter_shapes_are_read_structurally` and
    `counter_scope_is_read_from_the_declaration` in the same module.
  - `crates/deckmaste_lowering/src/designation.rs::declaration_conferrers_survive_lowering`
    (the whole `conferrer_tests` module) — it lowered a field that no longer
    exists on either side.

**Positives keep proving.** Every designation a keyword ability or keyword
action confers is `effectful := true` in `designationTable`, so tightening the
instructed-grant law to apply unconditionally refuses nothing that used to
pass: `okBecomesMonstrous` and `okGetsAnEnduringStory` prove `= []` exactly as
their tagged predecessors did, and all 26 bench cards still prove.

**Negative oracles.** None. `badGainsCommander` is a genuine refusal: the
commander designation is the table's only `effectful := false` row, and it is
`heldByCard`, so the instruction draws three refusals — scope, holder, and
`designationChecked`. The doc comment says which one is the ticket's law. No
pin was weakened to a `matches!`-style check; every negative still asserts the
exact refusal list.

**Deviations and additions.**

1. The ticket points at `crates/deckmaste_construction_core/src/macro_def.rs`
   for `DesignationDecl` ("if it types `conferrers`"). It does not:
   construction_core reads declaration bodies as opaque RON, and
   `DesignationDecl` / `DesignationConferrer` live in `deckmaste_semantics`
   (authored layer) and `deckmaste_core` (lowered). `conferrers` was dropped
   from both, together with `DesignationConferrer` — which existed only to type
   that field — its `Lower` impl, the two `lib.rs` re-exports, and the
   `minimal_designation_decl` fixture. `macro_def.rs` untouched. No v1
   declaration file used the field (v1's only designations, `Monarch` and
   `Commander`, never declared it), so no v1 data moved.
2. `conferralOk` deleted rather than reduced to a one-line alias for
   `DesignationLabel.checked`. Reducing it to an alias would name nothing the
   checker does not already say; the surviving read is inlined at the
   `.gainDesignation` arm, identical to `.setGameDesignation`'s.
3. `coreDeedFacts` / `coreDeedRuleFacts` collapsed to one definition (the
   wrapper existed only to splice `coreDeedConferrals` in).
4. `action_rows` in the facts generator now emits `{}` for a row with no
   columns. Before this landing every such row carried a `confers` list; the
   naive output would have been `{  }`.
5. `docs/contexts/game-model/CONTEXT.md`'s **Designation Conferrer** entry
   retired — the term now names nothing in the model. **Conferral** stays: it
   is the unrelated rules-defined ability conferral (`ConferralRule`), which
   this landing does not touch. `docs/keyword-policy.md` §14 no longer claims
   the checker reads declared conferrals.

**STOPs.**

1. **The facts generator cannot drop its `deckmaste_semantics` import** (ticket
   bullet 2). Only two of its six items were conferrer-related
   (`DesignationConferrer`, `CoreDeed` — the latter used solely by the
   `coreDeedConferrals` emitter); both are gone, along with the `core_deed`
   and `conferrals` helpers and `string_list`. The rest — `DesignationDecl`,
   `DesignationDef`, `DesignationScope`, `DesignationShape`, `CounterScope`,
   `Bearing`, `ron::options` — are how the generator deserializes v2
   declaration **bodies**: designation definitions and, for the counter-kind
   `dedicated` column, the full v1 `Bearing` tree. Shedding them means either
   duplicating those schemas in xtask (drift risk against the authored
   grammar) or relocating the declaration-body types into a v2 crate — a
   separate decision, not this ticket's. **Resolution:** delivered the
   conferrer half, left the dependency, did not expand scope. Owed to a
   follow-up ticket.
2. **Pre-existing red inside this landing's gate closure.**
   `cargo test -p deckmaste_plugin --test read_api_gate` fails on the claim
   base commit `66d4a51b`, unmodified by this landing:
   `typed_card_reads_go_through_the_restricted_api` flags
   `crates/xtask/tests/plugins_v2_declarations.rs:175`
   (`let written_out: Card = prelude.macros.read_str(…)`). Confirmed by running
   the test on a working copy at the claim parent, where it fails identically.
   `deckmaste_plugin` is in this landing's closure only because
   `deckmaste_core` / `deckmaste_semantics` lost the `conferrers` field.
   **Resolution:** reported, not fixed and not integrated around — the assurance
   rule forbids deleting the test that found it.

**Glossary gaps.** None the landing needed. One term was retired (deviation 5).

### REPORT

- Gate closure, from `cargo xtask gate --changed`:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_core -p deckmaste_card -p deckmaste_english_v2 -p deckmaste_semantics -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon -p deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui -p deckmaste -p xtask`.
  All suites green except the pre-existing `read_api_gate` failure above.
- `lean/scripts/build`: 141 s wall from a cleared `.lake/build`, 80 jobs, zero
  warnings, load average 16.5 on 24 cores.
- `cargo xtask lean-check`: 2 plugins, 26 cards, 4.5 s.
- `cargo xtask facts check`: both generated modules up to date.
- `cargo xtask cite check`: 15,307 citations, 0 stale.
  `cite check --list-noncompliant`: 0. One new citation site
  (`[CR#701.37a]`, already in the lock, no `bless` needed) read through
  `jj diff --git | cargo xtask cite audit --diff`: the rule is the monstrosity
  definition body, which is exactly the claim it supports.
- `cargo fmt --all` and `cargo clippy --all-targets` on the changed crates
  (`xtask`, `deckmaste_semantics_v2`, `deckmaste_core`, `deckmaste_semantics`,
  `deckmaste_lowering`): clean.
- Diffstat: 43 files, +303 / −351 (the landing record itself is 195 of
  those insertions; the code, data and prose change is net −243 lines).

### Note for integrate

The sibling ticket `semantics-v2-macro-bodies-keyword-abilities` is writing
keyword definition bodies in parallel and may pass a `Conferral` in the bodies
it writes. Those bodies need the same re-spell at integrate: drop the tag
argument from every `.gainDesignation` call. `Conferral` no longer exists, so
the conflict will surface as a Lean build error, not a silent merge.
