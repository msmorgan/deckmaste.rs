---
needs: []
---
**`ActFacts.participle` and the declarations' `grammar: Verb(participle: …)`
disagree in both directions.** Left standing by
`facts-act-columns-are-lean-data`, which moved the `actFacts` table into
`lean/Semantics/Check/Words.lean` and carried both sets over as they were.

`participle` (8 rows) is NOT the declaration's English participle: five keyword
actions declare `grammar: Verb(participle: …)`, and the two sets disagree in
both directions (`cast` and `activate` declare one and carry no `ActFacts`
participle; `destroy`, `discard`, `mill`, `tap` and `untap` carry one and
declare none).

Decide whether the checker column is a second, deliberately different fact —
in which case say so where it is defined — or the declaration's own participle
read late, in which case the column comes from the declaration and the five
disagreements are each resolved by name. Standard constraints apply.

## Landing record

Stamped on `nvlpwwyy` (`nvlpwwyypqukmlxsmtwtqmmpnxsumzst`); `plugins_v2/canon`
118 cards, `plugins_v2/testing` 2 cards, 65 keyword-action declarations, 65
`actFacts` rows, 8 `ActFacts` participle rows, 5 declared
`grammar: Verb(participle: ...)` -- every one of those counts unchanged by this
landing. The english_v2 coverage lock is not in this lane (no
`deckmaste_english_v2`, `core_verbs.ron` or corpus change), so no lock
`covered` count or construction count applies.

### The decision

**The checker column is a second, deliberately different fact.** It is read, so
it is not a dead column: the disposition is the doc sentence at its definition
and nothing else. The five disagreements are not resolved, because they are not
disagreements between two spellings of one fact -- the two sets answer
different questions and there is no reason they should coincide.

- **What reads it.** `ActFacts.participle` is read in exactly one place,
  `Semantics.participleOf` (`lean/Semantics/Check/Words.lean:309`,
  `deedFacts v >>= (·.participle)`). `participleOf` has two readers:
  `Semantics.actNamesParticiple` (`Check/Words.lean:320`) and the proof
  `Semantics.Proofs.Anaphora.copyParticipleUnwritten`
  (`lean/Semantics/Proofs/Anaphora.lean:441`, `participleOf (.core .copy) =
  none`, `by decide`). `actNamesParticiple` in turn has exactly one reader:
  `Semantics.verbedVoiceOk` (`lean/Semantics/Check/EventContext.lean:96`),
  which licenses an agentless passive event clause --
  `| none, what => what.isSome && (actNamesParticiple v || actIntransitiveOf v)`.
  So the column answers: *may this deed head a passive event clause with no
  agent written* ("when a creature is destroyed").
- **What the declaration column is.** `grammar: Verb(participle: ...)` on a
  `KeywordAction` declaration is English realization data: the surface a
  participle leaf spells. Omitted, it is derived from the bare surface by
  `english_participle`
  (`crates/deckmaste_construction_core/src/macro_def.rs:2582`), which is
  `format!("{bare}ed")`. The five declarations that write one (`activate`,
  `cast`, `exile`, `regenerate`, `sacrifice`) are five whose derived surface
  would be wrong: `activateed`, `casted`, `exileed`, `regenerateed`,
  `sacrificeed`. The column exists to correct a spelling, and says nothing
  about passive event clauses.
- **Why the two sets legitimately differ.** A deed may head a passive event
  clause without its declaration needing a spelling override (`destroy`,
  `discard`, `mill`, `tap`, `untap` -- five of the eight), and a declaration
  may need a spelling override for a deed no printed event clause names in the
  passive (`cast`, `activate` -- two of the five). The three that appear in
  both (`exile`, `regenerate`, `sacrifice`) agree on the string, which is what
  a coincidence of two independent facts looks like, not evidence of one fact
  written twice. Deriving either column from the other would be wrong in both
  directions: taking the checker column from the declarations would drop
  `destroy`/`discard`/`mill`/`tap`/`untap` from `verbedVoiceOk` and admit
  `cast`/`activate` to it; taking the declarations from the checker column
  would leave `activate` and `cast` spelling themselves `activateed` and
  `casted`.

### PROVE

- **No silent loss.** Nothing stopped being covered: no declaration, table row,
  type, function or theorem was changed, moved or deleted. The only edit in the
  tree is four added comment lines. `cargo xtask lean-check plugins_v2/canon`
  proves 118/118 and `plugins_v2/testing` 2/2, unchanged; `lake build`
  completes 80 jobs.
- **Structural laws.** `cargo xtask facts check` reports both generated modules
  (`lean/Semantics/Check/Facts.lean`, `idris/src/Experimental/FactsGen.idr`) up
  to date, so no generator output moved. `deckmaste_semantics_v2`'s
  `lean_drift` suite (the Lean/Rust mirror law, both directions) is green --
  the drift scanner strips block, doc and line comments before reading
  declarations (`strip_comments`, `tests/lean_drift.rs`), so the added doc
  comment is invisible to it by construction, not by luck. `corpus` (byte-exact
  declaration and card read/write round-trip) green.
- **No word-naming.** The added text is a doc comment; it introduces no guard.
  It names the participle `destroyed` and the declaration key
  `grammar: Verb(participle: ...)` as prose describing a data column, and no
  code reads a lexeme, construction, verb, noun, preposition or card identity
  as a result.

### DISCLOSE

- **What was added.** Four comment lines, one doc comment, on
  `ActFacts.participle` in `lean/Semantics/Check/FactTypes.lean` (172 -> 176
  lines). No other file in the tree is touched. No generator change, as the
  ticket disposition requires.
- **The eight rows the column carries**, unchanged and listed for the record --
  `Destroy` "destroyed", `Discard` "discarded", `Exile` "exiled", `Mill`
  "milled", `Regenerate` "regenerated", `Sacrifice` "sacrificed", `Tap`
  "tapped", `Untap` "untapped" (`Check/Words.lean`, `actFacts`). No
  `coreDeedFacts` or `abilityDeedFacts` row carries a participle.
- **Observed, not acted on: only the column's presence is consumed today.**
  Every reader goes through `actNamesParticiple`, which is
  `(participleOf v).isSome`, and the one proof asserts `= none`; no Lean or
  Rust code reads the eight strings themselves. The strings are the fact's
  content -- they say *which* participle names the deed, which is what makes
  the column checkable against printed text -- and the ticket's disposition
  fences resolution ("resolve nothing else"), so nothing was changed. Recorded
  here rather than routed: it is an observation about the column's current
  readers, not a defect and not deferred work.
- **Observed, outside this ticket: `tap` and `untap` derive a wrong declaration
  participle.** With no `participle:` declared, `english_participle` yields
  `taped` and `untaped`. That is an English realization defect in
  `plugins_v2/builtin/macros/keyword_actions/{tap,untap}.ron`, wholly separate
  from the checker column this ticket is about (the checker's `tapped` /
  `untapped` are correct). Not fixed here, because touching a declaration's
  `grammar` is outside the disposition and the change belongs to the english
  lane. Reported to the coordinator for routing.
- **Deviations and additions.** None. The ticket asked for a decision and, on
  the "second, deliberately different fact" branch, for one doc sentence at the
  definition; that is exactly what was written. Nothing was added or deleted
  beyond it.
- **STOPs.** None.
- **Glossary.** No gap. `docs/contexts/oracle-english/CONTEXT.md` and
  `docs/contexts/game-model/CONTEXT.md` between them already carry every term
  the record needs; the doc sentence introduces no new one.
- **Assurance counts.** Restored 0; re-spelled 0; ignored 0; added 0; removed
  0. No test's subject changed.

### REPORT

- `lean/Semantics/Check/FactTypes.lean`: 172 -> 176 lines. No other file
  changed.
- Gate: `cargo xtask gate --changed` reports "No workspace crates are affected
  by the changed paths" -- the derived closure for a `lean/`-only change is
  empty, because the gate reads changed paths against `cargo metadata` and does
  not know that `tests/lean_drift.rs` pulls `Check/FactTypes.lean` in through
  `include_str!`. Recorded as provenance, not fitted to: this landing therefore
  ran the ticket's stated oracles by hand rather than an empty gate --
  `cargo test -p deckmaste_semantics_v2 --test lean_drift --test corpus`
  (4 and 6 passed, 0 failed), `lake build` (80 jobs, "Build completed
  successfully"), `cargo xtask lean-check plugins_v2/canon` (118/118) and
  `plugins_v2/testing` (2/2), `cargo xtask facts check` (both modules up to
  date). `cargo fmt --all` clean; no Rust changed, so no clippy target moved.
  `cargo xtask cite check --list-noncompliant` 0 non-compliant; `cite check`
  15,853 citations, 0 stale -- the code tree, measured before this record was
  appended; `jj diff --git | cargo xtask cite audit --diff` audited 0 sites
  (the code diff adds no citation), no `bless` needed.
- Performance advisory: the english coverage command is not in this lane, so
  its 16.26 s quiet-host ceiling and per-byte thread-CPU telemetry do not
  apply. Measured instead, on a host also running sibling workspaces:
  `lake build` 80 jobs from a warm cache; `lean-check plugins_v2/canon` 33.0 s
  for 118 cards, `plugins_v2/testing` 1.1 s for 2;
  `cargo test -p deckmaste_semantics_v2 --test lean_drift --test corpus`
  0.02 s and 0.32 s.
