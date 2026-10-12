---
needs: []
---
# Tests, tooling and glossary nits

Priority: LOW.

## Why

Hygiene ticket; no new Construction, no coverage change expected. Small
findings from the reviews of the derived-punctuation-forms, same-as and
enter-as-copy landings. Line numbers are those of `qsupksky`.

## Items

### a. Text-level negatives beside the form-index negatives

The derived-punctuation landing made the form-index negatives pass trivially:
they set `form = 1` on a Reading and assert that admission fails, which no
longer exercises the surface.

- `crates/deckmaste_english_v3/tests/suite/clitic_contractions.rs:251–263`
  (`obsolete`/`joined` with `*form = 1`) has no text-level twin. Add one:
  `common::readings("it 's tapped", Category::FiniteClause)` and
  "that 's blue" (as a `SubjectRelativeClause`, beside the existing `"that's"`
  negative at `:272`) both read 0.
- `then_sequencing.rs:158–166` (`obsolete_form`, `form: 1`) already has its
  text-level twin at `:171`: "Draw a card then discard a card." reads 0
  (`Category::Document`). No change there beyond keeping it.

### b. A coordinator declaring neither separator realises a bare space

`separator coordination_separator(CommaCoordination)`
(`crates/deckmaste_english_v3/src/declarations.rs:1369–1371`) maps `No → " "`,
`Yes → ", "`, and schema `Coordination` (`:1372–1376`) never reads
`BareCoordination` (`:256`, default `Yes`; `CommaCoordination` `:257`, default
`No`). `ClauseCoordination` (`:2239–2245`) does read it. A coordinator
declaring `BareCoordination: No` and `CommaCoordination: No` would silently
realise `" "` through `Coordination`. Today only `vocab:Coordinator/Then`
(`core.ron:878–879`) departs from the defaults (`No`/`Yes`).

Fix: make the invalid pair fail at load or admission. The construction
language has no `!=` comparison today (no use in `declarations.rs`), so use a
Selection table over `(BareCoordination, CommaCoordination)` that admits
`(Yes, No)` and `(No, Yes)` and `require` it in `Coordination`, or reject the pair at lexicon load. Test: a lexicon fixture declaring both `No` fails.

### c. DSL keyword `separator` vs glossary term **Selected Literal**

The glossary (`docs/contexts/oracle-english/CONTEXT.md:172–177`) defines
**Selected Literal** for a fixed surface expression determined by declared
features; the construction language spells the declaration `separator`
(`declarations.rs:1369`). Per CLAUDE.md, never reword an entry to fit an
identifier: either rename the keyword to match the term (`selected_literal`
or similar, across `deckmaste_construction_v3_core` and every declaration
use), or, if the keyword names a narrower thing (a Selected Literal used as a
separator), add one sentence to the glossary entry naming the keyword as its
declaration form. Choose one and record why.

### d. `english-v3 diff --manifest` omits digest-only faces

`crates/xtask/src/english_v3/diff.rs:303–313`: `manifest()` collects
`gained`, `lost`, `decreases` and `increases` but not `digest_only`
(`Comparison` field `:132`), and the flag's doc (`:31–32`) says so. A reviewer
of the same-as landing added the 12 digest-only faces by hand. Include
`digest_only` by default, update the doc comment, and extend
`manifest_holds_every_count_change_and_seeded_controls` (`:489`) to assert a
digest-only face is present. No new flag.

### e. Unlike-function complement coordination licensing: route only

Already recorded as a design note at
`docs/tickets/planned/grammar-coverage-resume.md:686–697` ("each
coordinate's frame signature for the shared head", Master Biomancer's
*enters with … and as a Mutant*, gated today by coordinate licences per the
owner's 2026-10-08 bounded ruling, `znsuzsus`). Add one sentence there with
the shape: each coordinate retains its candidate frame signatures; the shared
head checks each against its declared frames. Route it to a future core
ticket (to be minted by the orchestrator; not minted here). Do not implement.

## Method

Standard constraints apply. Deltas:

1. Verification: items b and c touch grammar declarations, so run a bounded
   census of every face containing a coordinator plus 300 random covered
   faces, compared with `cargo xtask english-v3 diff` against the claim
   parent on the same manifest: expected gain 0, zero lost faces, zero count
   changes, zero digest changes. Items a, d and e need no census.
2. Probe-first: confirm each new negative reads 0 today and its positive twin
   reads ≥ 1; write the tests; run them once at the end.
3. Tests and clippy on the crates touched (`deckmaste_english_v3`,
   `deckmaste_construction_v3_core` if the keyword is renamed, `xtask` for
   item d). No Lean.
4. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted.
5. Record = the two-line note in `docs/tickets/README.md`.
