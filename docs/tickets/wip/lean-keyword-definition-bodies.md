---
needs: []
---
**A keyword ability's body is its definition; widen the Lean law to admit
it.** Ruling (user, 2026-09-06, resolving the contradiction found while
briefing `semantics-v2-macro-bodies-keyword-abilities`): the v2 `KeywordAbility`
macro for each keyword carries the keyword's rules definition as the
`Ability.keyword … (body : Option Ability)` payload, ported from the 55
definitional bodies under `plugins/builtin/macros/keyword/` (Flying is
`Static(Cant(Block(on: This, by: Not(Or([Has(Flying), Has(Reach)])))))`
[CR#702.9b]). Today `keywordBodyFits` (`lean/Semantics/Check/Abilities.lean`)
admits only `none`, or a triggered body on a keyword whose facts say
`bodied` with a matching stack regime; `badBodyOnBodilessKeyword`
(`Proofs/Faces.lean`) pins that a body on Flying is refused. Both must
change so the definition is admitted and the gate we now run
(`cargo xtask lean-check`) proves cards that invoke the widened macros.

Decide and land, standard constraints applying:

- **The new law.** A body is optional (a card may write bare "flying"); when
  present its regime must agree with the keyword's declared regime from
  `Facts.lean` (a static keyword takes a static body, a triggered one a
  triggered body whose event regime matches, an activated one an activated
  body). Decide whether a keyword may carry more than one ability (v1's
  `Composite` allowed a list; Lean's slot is one `Ability`) and, if so,
  what shape holds them. Decide what the `bodied` facts column now means
  (it meant "reminder text is printed"); retire it or redefine it in
  `crates/xtask/src/facts/` overlays and regenerate, `facts check` clean.
- **Pins.** Re-spell `badBodyOnBodilessKeyword` against the new law (a
  regime mismatch, e.g. a triggered body on Flying, refused as
  `keywordBodyFits`), keep `okRenownWithRenownExpansion` and its siblings,
  add a positive pin for a static definition on a static keyword.
- **A worked definition.** Add `flying` to `Macros.lean` carrying its
  definition, with the `StaticSpec` shape "can't be blocked except by
  creatures with flying and/or reach". If no `StaticSpec` constructor can
  express a block-legality restriction of that form, that is a modelling
  extension inside this ticket's remit; add it with its CR citation and
  pins, and say so in the landing record. Port one more definition of each
  regime (a triggered keyword and an activated keyword) to prove the law
  across regimes; the remaining ports are
  `semantics-v2-macro-bodies-keyword-abilities`'s.
- Mirror the changes into `crates/deckmaste_semantics_v2` if any syntax
  type changes (drift test), and note in `docs/decisions/semantics-v2.md`
  §6 that keyword ability macros carry definitions.

## Landing record

Measured on `mlwumovr` (`docs: keyword ability macros carry definitions`),
196 keyword facts rows, 46 of them declaring a definition.

### PROVE

- **Structural laws.** `lean/scripts/build` (all 77 targets: syntax, checker,
  macros, printed-card bench, pin suites, `--wfail`) completes successfully.
  `cargo xtask facts check` reports both generated tables up to date.
  `cargo xtask lean-check plugins_v2/testing` still proves 2/2 cards and the
  baseline is unchanged. `cargo xtask gate --changed` derived
  `cargo test -p xtask`; that suite is green (492 + 13 + 1 + 1 + 4 + 1 passed,
  0 failed, 1 pre-existing ignored). `cargo fmt --all` leaves no changes;
  `cargo clippy -p xtask --all-targets` is clean.
- **No word-naming.** The widened law reads two declared columns —
  `KeywordFacts.definition` and `KeywordFacts.regime` — and the written body's
  own constructor. No branch in `Check/` matches a keyword label, a lexeme or
  a card identity. The keyword names in `crates/xtask/src/facts.rs` are the
  declaration table, which is what that overlay is.
- **Citations.** `cargo xtask cite check --list-noncompliant` is empty;
  `cargo xtask cite check` reports 15,108 citations, 0 stale. Every citation
  site in the feature diff was read against its rule text with
  `jj diff --git | cargo xtask cite audit --diff` (18 sites). One was
  withdrawn on that read: an earlier draft of `keywordBodyFits`'s docstring
  cited [CR#113.6] for the stack-regime agreement, but [CR#113.6] governs the
  zone an ability functions in — the `functionsOnStack` column, not the
  `regime` one. No rule was new to `cr-citations.lock`, so no `cite bless`.

### DISCLOSE

**Decision 1 — the law.** `keywordBodyFits` now admits `none`, or an ability
whose category is one the keyword's registry row declares; a triggered body
additionally keeps the old stack-regime agreement
(`keywordStackRegime k == bodyEventRegime ev`). Category comes from a new
`Ability.category : Ability → Option AbilityCategory`, which is `none` for
`italicHead`, `alsoForKeywords`, `spell`, `keyword`, `mayBeginOnBattlefield`
and `thatAbility` — a word-headed or extended line is a card's decoration of
an ability, not a category, so those stay refused.

**Decision 2 — one body, not a list.** `Ability.keyword`'s slot stays
`Option Ability`. Definitions that are several abilities of the *same*
category are held by conjunctions that already exist: two statics by
`StaticSpec.conjunction` (shadow [CR#702.28b] and, on the v1 side, flashback),
several steps of one trigger by `Instruction.sequentially` (as
`renownExpansion` already does). See the STOP below for the cases that leaves.

**Decision 3 — `bodied` is redefined, not retired.** The Boolean
`KeywordFacts.bodied` ("the CR entry defines one triggered ability with a
quoted expansion") becomes `definition : List AbilityCategory`, and
`keywordBodied` becomes `keywordDefinition`. The migration is exact rather
than a reclassification: every one of the 44 rows that carried
`bodied := true` now carries `definition := [.triggered]`, which is what the
old column asserted. Two rows are added from their own CR entries: flying
`[.static]` [CR#702.9b] and cycling `[.activated]` [CR#702.29a]. The other
150 rows carry an empty list — "the workbench has not declared this keyword's
definition yet" — and their terms therefore still carry no body, exactly as
before this landing. `AbilityCategory` (`static | triggered | activated`) is
new in `Check/FactTypes.lean` [CR#113.3].

*Idris reference output is byte-identical.* The Idris table has no
`AbilityCategory`, so its emitter now writes `bodied := True` exactly when the
definition list contains `.triggered`. `idris/src/Experimental/FactsGen.idr`
does not appear in this landing's diff.

**Decision 4 — no `StaticSpec` extension was needed.** The ticket allowed
adding a block-legality constructor for flying. None was added: "can't be
blocked except by creatures with flying and/or reach" [CR#702.9b] spells as
`StaticSpec.deonticRule` with `Compulsion.forbid`, `[.core .block]`,
`Role.patient` and a `DeonticPatient.counterpart` naming creatures without
either keyword — the same shape the bench's Battlefront Krushok already uses
for menace's cardinality bound, with `Predicate.hasKeyword` supplying the
blocker filter. The v1 RON was read as reference only; the Lean term is
written in Lean's own constructors.

**Decision 5 — no Rust mirror change.** `AbilityCategory` and
`KeywordFacts.definition` live in `lean/Semantics/Check/`, which the
`deckmaste_semantics_v2` drift test does not mirror (it pairs only `Words`,
`Events`, `Phrase`, `Triggers`, `Abilities`, `Card`). No syntax type changed:
`Ability.keyword`'s arity and field types are untouched. The crate is not
edited in this landing.

**Pins** (`lean/Semantics/Proofs/Faces.lean`). Re-spelled 1, added 6,
restored 0, ignored 0, removed 0.

- Re-spelled: `badBodyOnBodilessKeyword` → `badTriggeredBodyOnStaticKeyword`.
  Same card sentence ("Flying (When this creature deals combat damage to a
  player, …)"), same asserted refusal list (`[.keywordBodyFits "Flying"]`), new
  reason: flying's definition is static, so a triggered body is the wrong
  category. The old name is retained in the docstring as provenance, because
  the subject it named — a keyword that admits no body at all — is what this
  ticket retires.
- Added positives, one per category: `okFlyingWithFlyingDefinition`,
  `okBushidoWithBushidoDefinition`, `okCyclingWithCyclingDefinition`.
- Added negatives, one per crossing not already pinned:
  `badStaticBodyOnTriggeredKeyword` (flying's body on renown),
  `badActivatedBodyOnStaticKeyword` (cycling's body on flying),
  `badActivatedBodyOnTriggeredKeyword` (cycling's body on bushido).
- Kept unchanged and still green: `okRenownWithRenownExpansion`,
  `badRenownWithStormExpansion`, `okCumulativeUpkeepOnPermanent`,
  `badCumulativeUpkeepOnSpell`.

**Macros** (`lean/Semantics/Macros.lean`). `flyingExpansion` / `flying`,
`bushidoExpansion` / `bushido`, `cyclingExpansion` / `cycling`, following the
`renownExpansion` / `renown` naming already in the file. "Expansion" is kept
rather than renamed to "definition" because it is the repo's existing word for
a keyword ability's rules meaning (game-model glossary, *Designation
Conferrer*), and renaming would split the vocabulary across the three
expansions already there.

**Deviations and additions beyond the ticket's letter.**

- Three negative pins beyond the ticket's one re-spelling, so that each of the
  three categories has a refused crossing as well as an admitted body.
- The game-model glossary gains an **Ability Category** entry [CR#113.3], with
  an `_Avoid_` line pointing away from the existing Lean `AbilityClass` type
  (which names grant/removal families, a different classification). Required
  by `CLAUDE.md`'s terminology rule: the landing needed a term for the
  static/triggered/activated axis and the glossary named only the members.
- Two comment blocks in `crates/xtask/src/facts.rs` were reworded where they
  explained the old `bodied` column, including the decayed [CR#702.147a] note,
  which now records *why* its definition stays undeclared.

**STOP — a multi-category definition has no shape.** Several keywords'
definitions are one static ability *and* one triggered ability: decayed
[CR#702.147a] and modular [CR#702.43a] say so in as many words, and v1's
composites for evoke, graft and offspring each pair a static component with a
triggered one. Reconfigure's is two activated abilities [CR#702.151a], which
has the same problem for a different reason. One `Option Ability` slot cannot
hold either pair, and neither can any conjunction that exists today —
`StaticSpec.conjunction` joins static specs only, and `Ability` has no
conjunction constructor (a `List Ability` is card text, `Characteristics.text`).
Of the eleven v1 keyword composites carrying more than one component ability,
six (convoke, enchant, flashback, infect, protection, shadow) are all-static
and so already spellable through `StaticSpec.conjunction`; the other five
(evoke, graft, modular, offspring, reconfigure) are not.

Resolution: not decided here. The slot stays `Option Ability`, and this
landing declares no definition for a mixed-category keyword, so nothing is
mis-admitted — the law is already written against a *list* of declared
categories, so a keyword declared `[.static, .triggered]` would admit either
half today and would need only the slot's arity to change, not the law. The
choice between widening the slot to `List Ability` and adding an ability
conjunction belongs with the porting evidence, so it routes to
`semantics-v2-macro-bodies-keyword-abilities`, which ports the remaining 52
definitions and will hold every mixed case at once.

**Glossary gaps.** One, amended above (*Ability Category*). No term the
landing needed is left undefined.

### REPORT

Provenance, not a gate; measured on `mlwumovr`.

- `lean/scripts/build`, warm incremental after the `Check/Abilities.lean`
  docstring edit (which invalidates the checker and everything downstream —
  74 of 77 targets rebuilt): 1m50s wall, 7m41s user, on a host under load
  average 26.9 with three sibling feature workspaces building concurrently.
  An earlier run of the same script on the same tree, at load average ~10,
  took 1m15s wall / 3m25s user. The workbench build is not otherwise
  instrumented per byte.
- `cargo test -p xtask`: 39.8s for the main suite on the same loaded host.
- `cargo xtask lean-check plugins_v2/testing`: 0.3s, 1 plugin, 2 cards.
- Keyword facts: 196 rows, 46 declaring a definition (44 `.triggered`,
  1 `.static`, 1 `.activated`), 150 undeclared.
