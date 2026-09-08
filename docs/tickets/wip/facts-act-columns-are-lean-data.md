---
needs: []
---
**`ACTION_OVERLAY` is hand-written Lean living in Rust.** Finding recorded by
`semantics-v2-definition-bodies`, whose per-column pass found no derivation and
no declaration home for twelve of `ActFacts`' thirteen columns.

`lean/Semantics/Check/Words.lean` already hand-writes two of the checker's three
deed-fact sources in Lean itself: `coreDeedFacts : CoreDeed → ActFacts` (15
rows) and `abilityDeedFacts : List (KeywordLabel × ActFacts)` (3 rows). The
third, `actFacts : List (KeywordActionLabel × ActFacts)`, is generated into
`Check/Facts.lean` from `crates/xtask/src/facts/action_overlay.rs` — 65 rows of
Lean record syntax written inside Rust string literals, where no Lean tooling
reads them and `decide` never sees them until the generator has run.

The columns are the checker's own data, not the registry's: `agentRole` (45
rows) and `patientRole` (15) are the gate a deontic clause reads, authored as a
bench sentence spells its deed; `stepwise`, `intransitive`, `feature`,
`counterfactual`, `rides`, `plays`, `bounded` and `opponentsLibrary` are the
same. `participle` (8 rows) is NOT the declaration's English participle: five
keyword actions declare `grammar: Verb(participle: …)`, and the two sets
disagree in both directions (`cast` and `activate` declare one and carry no
`ActFacts` participle; `destroy`, `discard`, `mill`, `tap` and `untap` carry one
and declare none). `dest` (13 rows) is the one column the `facts labels` report
already calls a CR fact authored from the [CR#701] entry.

So move the 65 rows into `Check/Words.lean` beside `abilityDeedFacts`, as
`actFacts` written in Lean; drop `actFacts` from the generated module and from
`facts.rs`; keep the generator's two-way check that every keyword-action
declaration has a row and every row a declaration. `Facts.lean` shrinks by 65
rows and the checker gains a hand-written table it can be read and proved
against. `distinctActLabels actFacts` in `Proofs/Tables.lean` keeps working.

The same question stands for the `KeywordFacts` gate columns (`regime` 53 rows,
`functionsOnStack` 47, `onInstantOrSorceryCard` 66, `paidCost` 64,
`onPermanentCard` 13, `wantsModes` 4, and the six `extra` argument-schema
variants), but those are ALSO read by the frozen Idris reference generator, so
they cannot move until it retires (`semantics-v1-cutover`). Decide the two
separately. Standard constraints apply.

## Landing record

Stamped on `nwkywupv` (`nwkywupvvxzrozvmmvxrmmwpvsrkkxor`); `plugins_v2/canon`
118 cards, `plugins_v2/testing` 2 cards, 65 keyword-action declarations, 65
`actFacts` rows — every one of those counts unchanged by this landing.

### PROVE

- **No silent loss.** All 65 `actFacts` rows moved unchanged: the generated
  block and the hand-written table are identical up to whitespace (compared
  with whitespace collapsed; the only edit is wrapping long rows to the file's
  100-column width, as `abilityDeedFacts` beside it is wrapped). No row was
  added, dropped, reordered or re-valued, and no `[CR#…]` citation stood inside
  the overlay to carry over. No keyword action stopped being covered:
  `cargo xtask lean-check plugins_v2/canon` proves 118/118 and
  `plugins_v2/testing` 2/2, unchanged.
- **Structural laws.** `cargo xtask facts check` reports both generated modules
  (`lean/Semantics/Check/Facts.lean`, `idris/src/Experimental/FactsGen.idr`) up
  to date, so the committed `Facts.lean` is exactly what the generator writes;
  `lake build` completes 80 jobs; `distinctActLabels actFacts` still holds
  (`Semantics.Proofs.Tables.actLabelsDistinct`); `deckmaste_semantics_v2`'s
  `lean_drift` suite (the Lean/Rust mirror law) is green.
- **The two-way check is not vacuous.** Probed with `lake env lean` on a
  scratch file: `["Activate", "Nonesuch"].all (fun l => (actFacts.find? (·.1 ==
  l)).isSome) = true` and `actFacts.all (fun row => ["Activate"].contains
  row.1) = true` are each *refuted* by `decide`, while the same statement over
  the real lists is proved. A declaration without a row, and a row without a
  declaration, each fail the Lean build.
- **No word-naming.** The two new theorems name no card, lexeme or verb — they
  quantify over the generated label list and the table. The generator's new
  `action_label` reads a declaration's own name and names no action.

### DISCLOSE

- **The table.** `actFacts : List (KeywordActionLabel × ActFacts)` is now
  hand-written in `lean/Semantics/Check/Words.lean`, immediately after
  `abilityDeedFacts` and before `distinctActLabels`, with a doc comment saying
  why the columns live there and naming the pin.
- **The generated module.** `Facts.lean` no longer carries `actFacts`. In its
  place, at the same position, the generator writes
  `def keywordActionLabels : List KeywordActionLabel` — every keyword-action
  declaration's checker label, in declaration order.
- **The label derivation (a decision this ticket had to take).** The retired
  overlay carried each Lean label as its own key and matched it to a
  declaration through `normalize`, so with the overlay gone nothing supplied
  the labels. `keywordActionLabels` derives each from the declaration's
  camelCase `name`, read as the words it spells with each capitalized:
  `collectEvidence` → `"Collect Evidence"`, `theRingTemptsYou` → `"The Ring
  Tempts You"`. Checked against all 65 labels: exact, in both this derivation
  and the alternative (title-casing the declared `spelling`). The declaration
  `name` was chosen because it is the declaration's identity under the dialect
  case rule, while `spelling` is a surface that may legitimately drift from it.
  A derivation that ever disagrees with the table is caught by the Lean pin, not
  silently accepted.
- **The check's new home.** `lean/Semantics/Proofs/Tables.lean`:
  `Semantics.Proofs.Tables.everyDeclaredActionHasFacts` (every declared keyword
  action has a row) and `Semantics.Proofs.Tables.everyActRowIsDeclared` (every
  row names a declared keyword action), both `by decide`, both order- and
  multiplicity-independent — the same two directions the generator used to
  check of its overlay, with the same two failure messages' meanings. No
  `native_decide`, no xtask text-scraping of Lean; the fallback the ticket
  fenced as a STOP was not needed.
- **What was deleted.** `crates/xtask/src/facts/action_overlay.rs` (143 lines,
  the 65 rows of Lean written in Rust string literals), its `mod` line in
  `crates/xtask/src/facts.rs`, `lean.rs`'s `action_rows` and its
  `ACTION_OVERLAY`/`normalize` imports.
- **What was NOT deleted, and why (`facts.rs` mirror fields).** *None* — no
  field in `crates/deckmaste_semantics_v2/src/facts.rs` existed only to type
  the `actFacts` emission. The emission never used the Rust types at all: the
  overlay was raw Lean text, and `ActFacts`/`DeedRole`/`ReferentSort`/
  `EntityDomain`/`ObjectClass`/`PremiseSort`/`DeedFeature` had (and still have)
  no Rust reader. They cannot be dropped from `facts.rs` on their own either:
  `tests/lean_drift.rs` pins `Check/FactTypes.lean` against `facts.rs` in both
  directions, so deleting them in Rust requires moving the same seven
  declarations out of `FactTypes.lean` — a change to which Lean modules the
  mirror contract covers, which belongs to `docs/decisions/semantics-v2.md`
  §10 rather than to a generator ticket. Routed to
  `docs/tickets/planned/facts-actfacts-leaves-the-registry-mirror.md`.
  `SubtypeFacts`, `CounterFacts`, `DesignationFacts` and `KeywordFacts` keep
  every column they had; nothing they need was touched.
- **Deviations and additions.** (1) The two Lean theorems above, and a sentence
  in `Tables.lean`'s module doc saying why an act pin beyond distinctness is
  there — the ticket asks for the surviving two-way check and this is it.
  (2) `action_label`/`action_labels` in `lean.rs`, replacing `action_rows` —
  forced by the label question above. (3) A duplicate-label guard was kept on
  the new path ("two keyword actions share the checker label X"), preserving
  the retired `duplicate action overlay key` guarantee. (4) Module docs in
  `facts.rs` and `facts/lean.rs` updated to stop describing an overlay that no
  longer exists. (5) Two planned tickets minted (participle, mirror). Nothing
  else was added or removed.
- **STOPs.** None. The one the ticket fenced (xtask scraping labels out of
  Lean) was not reached; the Lean-side proof is clean. The `facts.rs` question
  above was resolved as "delete nothing", disclosed rather than acted on,
  because acting would have crossed a recorded mirror contract.
- **Residue routed.** The `participle` disagreement the ticket documents is
  carried over untouched (declarations `activate`, `cast`, `exile`,
  `sacrifice`, `regenerate` declare `grammar: Verb(participle: …)`; `ActFacts`
  carries eight participles: destroyed, discarded, exiled, milled, regenerated,
  sacrificed, tapped, untapped — verified against the declarations rather than
  taken from the ticket) →
  `docs/tickets/planned/facts-participle-column-reconciled.md`.
- **Glossary.** No gap. `docs/contexts/game-model/CONTEXT.md` defines **Keyword
  Action** ([CR#701.1]) and says a Keyword Action "is named by its declared
  label, never by a constructor" — which is what `keywordActionLabels` emits,
  so the generated list is the glossary's own shape rather than a new notion.
- **Assurance counts.** Restored 0; re-spelled 1
  (`new_action_requires_a_checker_overlay` →
  `new_action_reaches_the_lean_label_list`: same card — a keyword-action
  declaration added with no checker columns — and the same asserted outcome,
  now spelled against the surviving check, asserting the new declaration
  reaches `keywordActionLabels` where the Lean pin refuses it); ignored 0;
  added 2 (the two Lean theorems); removed 0.

### REPORT

- `Facts.lean`: 359 lines before, 359 after (the 65 row lines became 65 label
  lines); 31,750 bytes before, 28,039 after (−3,711). `Check/Words.lean`:
  1,528 → 1,632 lines (+104: 6 doc lines and the 98-line table).
  `Proofs/Tables.lean`: 49 → 64. `action_overlay.rs`: 143 lines deleted.
- Gate: `cargo xtask gate --changed` derives `cargo test -p xtask`; run with
  `--run`, green — 487 passed / 0 failed / 1 ignored (pre-existing), plus
  12, 1, 4, 5 and 0 in the integration and doc targets. Also run outside the
  derived closure, as this ticket's Lean oracle:
  `cargo test -p deckmaste_semantics_v2 --test corpus --test lean_drift`
  (6 and 4 passed). `cargo fmt --all`; `cargo clippy -p xtask --all-targets`
  clean. `cargo xtask cite check --list-noncompliant` 0; `cite check` 15,854
  citations, 0 stale; `jj diff --git | cargo xtask cite audit --diff` audited
  2 sites (both `[CR#701.1]`, both claims about keyword actions), no `bless`
  needed — [CR#701.1] was already registered.
- Performance advisory: the english coverage command is not in this lane (no
  `deckmaste_english_v2` or corpus change), so its 16.26 s ceiling and per-byte
  telemetry do not apply. Measured instead, on a host also running sibling
  workspaces: `lake build` 80 jobs from a warm cache, with
  `Semantics.Proofs.Tables` itself at 13 s (it now carries two 65-label
  `decide`s beside the six distinctness ones); `lean-check plugins_v2/canon`
  33.2 s for 118 cards, `plugins_v2/testing` 1.1 s for 2; `cargo test -p xtask`
  36.8 s for the unit target.
