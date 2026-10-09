---
needs: []
---
# Read an equality property value: each nonland permanent with mana value equal to the number of charge counters on this artifact.

## Why

*with power less than X* reads since `english-v3-postpositive-comparative-property`
landed, but *with power equal to X* does not. Not a wave-5 recon bucket; the
comparative-property review counted it. `cargo xtask english-v3 grep -i
'\bwith (power|toughness|mana value) equal to\b'` finds **36** faces, all
unread on census `qsupksky` (32,828 supported faces, 25,474 covered, 7,354
unread). On 21 of them the face's only failing unit at `qsupksky` contains
the phrase and no *plus*; 10 faces have *equal to 1 plus …* and also need
`english-v3-arithmetic-nominal-operand`. Surface counts, not gain
forecasts.

Probes (`--readings 1`): "Its power is equal to its toughness." 1; "… with
mana value less than the number of charge counters on this artifact." 1;
"… with mana value equal to the number of charge counters on this
artifact." 0.

## Goal

After a property noun (*power*, *toughness*, *mana value*), the equality AdjP
*equal to X* reads as the property value, through the same route as the
ordering comparatives, with byte-exact realisation.

## Analysis

CGEL Ch. 6 §3.3(c), p. 552, [46ii] (*a leader younger than you*): postpositive
AdjPs follow an ordinary noun head; Ch. 6 §2.1, pp. 528–529, for the
postpositive function. That backs the function only (the citation the
ordering landing used).

Existing route: `MeasuredAttribute`
(`crates/deckmaste_english_v3/src/declarations.rs:4342`,
`[head: Nominal, quantity: PropertyValue]`) ← `ScalarMeasureValue` instance
row `(PropertyValue, ScalarValue, ScalarPropertyValueKind)` (`:4300`) ←
`ComparativeScalarValue` (`:4309`, `[value: ComparativeAdjectivePhrase]`),
which today requires `value.ComparisonMarker = Ordering` (`:4312`).
*equal to* is a `ComparativeGovernorHead` with `ComparisonMarker = Equality`
(*to* declares it, `lexicon/core.ron:225`); coordinated *equal to or greater
than* already reads because `coordinated_comparison_marker`
(`declarations.rs:4427`) yields `Ordering`.

Orchestrator rulings 2026-10-09:

- R13a. Same route as R5a (2026-10-08): admit the Equality marker through the
  existing `ComparativeScalarValue` / `PropertyValue` route by relaxing the
  `ComparisonMarker` requirement (a table over the declared feature, or the
  requirement dropped if both values are licensed); no new Construction, no
  new NP postmodifier.
- R13b. The predicative "… is equal to X" must keep its claim-parent Reading
  count; *damage equal to its power* and other non-property heads must not
  gain a Reading through `MeasuredAttribute` (its `PropertyValuePosition`
  licence already gates the head; verify by probe, do not add a word guard).
- R13c. *equal to or greater than* keeps exactly its current Readings; the
  Equality admission must not create a second bracketing of the coordinated
  governor.
- R13d. *with power equal to that card's power and toughness equal to its
  toughness* (Soul Separator) composes through the landed coordinated
  property-value route; record it in spot checks if it reads.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control
(*equal to* → *less than*) 1. (D) = `--category Document`.

- Master Symmetrist: "Whenever a creature you control with power equal to its
  toughness attacks, it gains trample until end of turn."
- Chalice of the Void: "Whenever a player casts a spell with mana value equal
  to the number of charge counters on this artifact, counter that spell."
- Sanctum Prelate: "Noncreature spells with mana value equal to the chosen
  number can't be cast."
- Ratchet Bomb (D): "{T}, Sacrifice this artifact: Destroy each nonland
  permanent with mana value equal to the number of charge counters on this
  artifact."
- Aether Vial (D): "{T}: You may put a creature card with mana value equal to
  the number of charge counters on this artifact from your hand onto the
  battlefield."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bequal to\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261009`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *equal to X* read as a postmodifier of the whole NP rather than as the property value, *than*/*to X* attached outside the AdjP, or a non-property head gaining the value, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *whose power is equal to X* (relative-clause gap).
- The arithmetic half of *equal to 1 plus …*
  (`english-v3-arithmetic-nominal-operand`).

## Landing record

Six items, each figure stamped with the change id it was measured on (ruling
2026-10-08):

1. **Census delta**: covered faces and total Readings before/after on the
   bounded manifest; lost faces by name with classification, or "none".
2. **Witnesses**: each witness above with its Reading count after.
3. **Spot checks**: ten newly covered faces by name with the selected analysis.
4. **Deviations and STOPs**: constructions, frames or tests added or deleted
   beyond the ticket's letter, with justification; every STOP and its
   resolution; any glossary gap in `docs/contexts/oracle-english/CONTEXT.md`.
5. **Tests**: the `cargo test -p` / clippy commands and outcomes; counts
   restored / re-spelled / ignored-with-blocker / added / removed.
6. **Timing**: wall time of the bounded census as one figure with worker count
   (6), plus peak RSS from `/usr/bin/time -v`.
