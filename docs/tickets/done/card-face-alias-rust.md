---
needs: []
---
**Split `CardFace` from `Characteristics` on the Rust side.** Residue of
`workbench-card-face-record` (2026-09-04), which made the split in the
workbench and stopped at the Rust alias `pub type CardFace =
Characteristics` (`crates/deckmaste_card/src/card.rs`): 749 references
across 11 crates plus the serde shape of every card RON. The ruling stands
— a face has characteristics [CR#109.3] and possibly layout-specific data;
alternative characteristics (flip, adventure [CR#710.1,715.2]) are not
faces — so core, card and lowering should refuse a face where alternative
characteristics are meant, the way the workbench pin
`badFaceAsFlipAlternative` does.

Fix: `struct CardFace { characteristics: Characteristics, … }` in
`deckmaste_card` (or wherever the drift guard says the type belongs), the
flip/adventure alternatives typed `Characteristics`, a serde shape that keeps
every card RON loading (a `#[serde(transparent)]` or migration step —
decide from the guard), the drift guard re-run, a Rust test mirroring the
workbench pin. `deckmaste_semantics` untouched.

Size: M–L. Done when: no alias remains; the alternatives cannot be passed a
face; every card RON loads; `cargo test --workspace` green. Standard
constraints apply.

## As landed

- **`pub struct CardFace { pub characteristics: Characteristics }`**
  (`crates/deckmaste_card/src/card.rs`) — one slot, nothing else, because a
  single-faced card's layout adds nothing today. `pub type CardFace =
  Characteristics` is gone; no alias replaces it in either direction.
- **Serde shape: `#[serde(transparent)]`.** The face adds no data, so a face
  serialises exactly as its `Characteristics` and every RON written against the
  pre-split shape still reads. No migration step. Guarded by a new unit test,
  `deckmaste_card::card::tests::a_face_is_serde_transparent_over_its_characteristics`,
  which reads `Normal((name: "Forest", types: []))` and writes it back.
  (Card RON under `plugins/`/`data/` is read as `deckmaste_semantics::Card`
  and lowered, so the core-side shape is not what those files parse against;
  transparency keeps it byte-identical regardless.)
- **`impl From<Characteristics> for CardFace`** is the constructor wrap used at
  every face-building site.
- **The alternatives stay `Characteristics`**: `Card::Flip { normal: CardFace,
  alternative: Characteristics }` [CR#710.1] and `Card::Adventurer { normal:
  CardFace, adventure: Characteristics }` [CR#715.2] — unchanged in declared
  type, but now genuinely a different type from a face.
- **The pin is a compile error, not a deserialisation error** — the type system
  gives it directly: a `CardFace` in an `alternative`/`adventure` slot is
  `E0308`. Spelled as a `compile_fail` doctest on `CardFace` mirroring the
  workbench's `badFaceAsFlipAlternative`, with a passing companion doctest
  showing the correct spelling.
- **Lowering re-pointed**: `impl Lower for deckmaste_semantics::CardFace` now
  targets `deckmaste_card::Characteristics`, and `Lower for Card` wraps the
  three face slots (`Normal`, `DoubleFaced`, `Split`) plus each `normal` with
  `.into()`. The grammar's `TwoFaced { front, back }` packs alternatives into
  `back`, so a semantic `CardFace` is not always a face; lowering already
  decided which form each is, and now says so in the types.
- **749-reference sweep**: 244 struct literals rewritten to
  `CardFace::from(Characteristics { … })` (with `..CardFace::default()` →
  `..Characteristics::default()`) by script, then 193 field accesses given
  `.characteristics` from compiler diagnostics, over `deckmaste_engine`,
  `deckmaste_lowering`, `deckmaste_plugin`, `deckmaste_noncanon` and
  `deckmaste_tui`. The non-mechanical sites were read and changed by hand:
  `deckmaste_plugin::plugin::validate_card_regions`,
  `deckmaste_plugin::validate::lint_all_card_faces` and
  `deckmaste_plugin::tests::corpus_identity::core_abilities` each collect
  `Vec<&Characteristics>` over faces **and** alternatives, so they now project
  `&face.characteristics` and pass the alternative through unchanged.
- **`deckmaste_semantics` untouched**, as is `idris/src/Semantics.idr`. So is
  every `deckmaste_semantics::CardFace` consumer (`deckmaste_legacy_render`,
  `xtask`'s english/macros modules, `macro_ron`) and
  `deckmaste_migrations::TodoCardFace` — different types with the same name.
- **Guards re-run**: `cargo xtask validate` over every plugin, and
  `cargo xtask idris-check plugins/canon --differential` (the
  certifier/resolver differential that ties the mirror to lowering).

## Landing record

Measured on change `rxqvokpo` (this landing's tree).

- `cargo check --workspace --all-targets`: `Finished 'dev' profile
  [unoptimized + debuginfo] target(s) in 3.27s` — 0 errors.
- `cargo fmt --check`: clean, no diff.
- `cargo clippy --workspace --all-targets`: exit 0, **11 warnings, all
  pre-existing** — `deckmaste_lowering/src/ability.rs:28` (`items after a test
  module`) and 10 × `this pattern is unneeded as the '..' pattern can match
  that element` in `deckmaste_lowering/src/card.rs`. Same count, same lint,
  same two files as before the round; **0 new**.
- `cargo test --workspace`: exit 0, **6117 passed, 0 failed, 6 ignored** across
  128 test binaries.
- `cargo xtask validate`: `plugins/builtin: 12 valid, 0 todos skipped, 0
  invalid, 0 canon mismatch(es)`; `plugins/canon: 80 valid, … 0 canon
  mismatch(es)`; `plugins/testing: 39 valid, … 0 canon mismatch(es)`;
  `plugins/demo: 0 valid, …`. `plugins/builtin_v2` fails to load
  (`macros/stubs/ability_words/Adamant.ron`: `Expected unit`) — **pre-existing
  and unrelated**: the failure is in macro-stub expansion, and this round
  changes no file in `macro_ron`, `deckmaste_semantics` or `deckmaste_english`.
- `cargo xtask idris-check plugins/canon --differential`: `certifier/resolver
  differential — 68 agreed sound, 0 agreed unsound, 12 skipped (no certifier
  verdict)`; `differential OK: 0 disagreements`.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14210 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless`: `blessed 1420 rules at cr_date 2026-08-07` — no
  diff to `cr-citations.lock`; every cited rule was already registered.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`: `audited 8
  citation site(s)`, each rule read against its claim. [CR#712.1] (a
  double-faced card has a card face on each side) and [CR#709.1] (split cards
  have two card faces on a single card) back the face doc's enumeration;
  [CR#710.1] ("Additional alternative characteristics appear upside down")
  backs both the pin's claim and lowering's comment; [CR#715.2] ("the text in
  the inset frame … defines alternative characteristics") backs the Adventure
  half; [CR#707.2] (copiable values are those derived from printed text) sits
  on the pre-existing copy-model comments the sweep reflowed.
- Assurance counts: **restored 0; re-spelled 11** — the 4
  `deckmaste_card::card::tests` witnesses (now built through a `face()` helper)
  and the 7 `deckmaste_lowering::card::tests` arms (`lowers_card_face` retargets
  to `Characteristics`; `lowers_card_normal`, the two double-faced arms,
  `lowers_card_split`, `lowers_card_flip` and `lowers_card_adventurer` assert
  the nested face shape). Every other test kept its subject and its asserted
  outcome and changed spelling only (the 244 constructor wraps and 193 field
  accesses above). **ignored 0; added 3** — the `compile_fail` pin doctest, its
  passing companion, and the serde-transparency unit test. **removed 0.**
- Pin non-vacuity probed: with `alternative: face` replaced by `alternative:
  Characteristics::default()`, the `compile_fail` block compiles and the
  doctest fails with `Test compiled successfully, but it's marked
  'compile_fail'`. Restored afterwards; both doctests pass.
- Deviations and additions:
  - `impl From<Characteristics> for CardFace` — the constructor wrap the
    749-site sweep needed; the ticket names the wrap but not the spelling.
  - `Lower for deckmaste_semantics::CardFace` retargeted to
    `deckmaste_card::Characteristics` rather than `CardFace`. Forced: `Lower`
    has one `Target` per source type, and the same semantic `CardFace` lowers
    to a face in three `Card` arms and to alternative characteristics in two.
  - Added `a_face_is_serde_transparent_over_its_characteristics` beyond the
    ticket's letter, so the `#[serde(transparent)]` decision has a test rather
    than a comment.
  - Three stale type references in comments corrected to `Characteristics`
    (`deckmaste_core::ability`, `deckmaste_core::copy`,
    `deckmaste_engine::copy`) — they named `CardFace::abilities` and
    "`CopiableValues` mirrors `CardFace`", paths that no longer resolve.
- No STOP taken.
