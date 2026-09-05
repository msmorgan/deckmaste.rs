---
needs: []
---
**`vocab Color` is a false name.** Its eight members combine the five Color
values ([CR#105.1]) with three color-property adjectives, and [CR#105.4] holds
that neither *multicolored* nor *colorless* is a color. Rename the vocabulary
and all five declaration consumers plus the `ast.rs` use so the Oracle-English
name does not claim that every member is a Game Model Color.

Pinned scope: a behaviour-preserving vocabulary rename within
`deckmaste_english_v2`; no member, construction, scanner, selection rule, or
lexical licensing change. Choose the replacement name against the Oracle
English glossary before implementation and record any glossary amendment it
needs.

Routed by `english-v2-tail-color-property-adjectives` on 2026-09-05. The
measured blast radius is five sites in `constructions.rs` plus `ast.rs`, with
nothing outside `deckmaste_english_v2`.

## Landing record

Measured on change `ptmoouqsvxpq`, the last commit in this landing carrying a
source change, after a `kata refresh` that took trunk's
`english-v2-affinity-quality-surface` landing; lock `covered` 19,622. The
review commit above it edits only this file.

PROVE. Renamed the eight-member vocabulary to `ColorWord`, its four
`constructions.rs` consumers, the public AST re-export, the generated visitor
re-export, and the exact lexical-provenance assertion. The eight members and
all construction bodies, scanners, selection rules, and lexical licences are
otherwise unchanged. The report-mode coverage check left
`english-v2-coverage.lock` byte-unchanged: 19,622 -> 19,622 covered, with no
loss or gain entries (+0/-0), so no identity became covered and none stopped
being covered. Coverage reports 0 selected-uncovered units, 0 roundtrip
mismatches over all 19,622 accepted units, 0 ownership failures,
869,121/869,121 construction visits, 303,174/303,174 leaf visits, and 0
traversal failures. Full ambiguity reports 0 unresolved ties and 0 internal
failures. The coverage inventory reports 23 permitted and 0 forbidden
licensing checkers; no guard names a lexeme, construction, verb, noun,
preposition, or card, and `environment.rs` reports no load error.

Selection neutrality, measured on the final tree with identical flags on both
sides (`cargo xtask english_v2 ambiguity --json --workers 8`, neither side
passing `--require-resolved`): the parent side is this feature's fork point,
change `ztszxmotttws`, checked out in-workspace; the feature side is change
`ptmoouqsvxpq`. Both reports carry the same 32,641 unit ids. Diffed by
identity over `status`, `internal_kind`, and the complete selection
`decision`, the per-unit diff is empty: 0 losses, 0 gains, 0 changed
construction paths, 0 changed specificity vectors, and 0 changed resolution
kinds. Construction declarations remain 395 -> 395.

The reverse-dependency closure printed by `cargo xtask gate --changed
--clippy` and run:

```text
cargo test -p deckmaste_english_v2 -p xtask
cargo clippy -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
```

Representative positive artifacts from that run:

```text
test result: ok. 146 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 468 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
test result: ok. 112 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Clippy is clean for both crates, and `cargo fmt --all` leaves the tree
unchanged. Citation gates report `0 non-compliant citation-looking string(s)`
and `checked 14497 citations against cr.txt (eff. 2026-08-07); 0 stale`; the
`cite audit --diff` text was read against each claim.

DISCLOSE. Selection census, both sides under identical flags: 19,622 selected
/ 15,560 unique / 4,062 specificity-resolved, before and after. The
specificity share did not rise, so there is no construction pair to name. No
identity became covered, so there is no newly covered analysis to disclose,
and no identity stopped being covered. The raw ambiguity JSON differs only in
four parse-failure expectation messages: Become the Pilot, Firemane Commando,
Shield Broker, and Subjugate the Hobbits now read `expected color word, ...`
where the fork point read `expected color, ...`. Their failure spans,
statuses, and complete selection decisions are unchanged; this is the
requested vocabulary display rename, not a parsing or selection change.

Glossary gap: the Oracle-English glossary had no term for this class of Word
Forms. This landing adds **Color Word**, grounded in [CR#105.1] for the five
Colors, [CR#105.2a..105.2c] for the three color-property adjectives, and
[CR#105.4] for the fact that *multicolored* and *colorless* are not
themselves Colors, with an `_Avoid_` line naming the false term *Color* for
this Category. `CONTEXT-MAP.md` needs no change: it routes to this glossary
and names no color term.

Deviations and additions: the ticket's measured blast radius omitted
`visit.rs`; compilation exposed the generated vocabulary visitor re-export, so
it is necessarily re-spelled `walk_color_word`. `docs/tickets/fog.md`'s routed
residue note is updated to record the rename, matching how that section
records other landed families. Nothing else outside `deckmaste_english_v2` and
the glossary is touched: the completed ticket
`docs/tickets/done/english-v2-tail-color-property-adjectives.md` keeps the
old name throughout, because its text is the stamped record of what that
landing measured. No construction or test was added or deleted. Assurance
counts: restored 0; re-spelled 1 (the existing lexical-provenance assertion);
ignored 0; added 0; removed 0. STOPs: none.

REPORT. Coverage-lock identities: 19,622 -> 19,622; construction
declarations: 395 -> 395. Licensed vocabulary/lexicon homographs remain
exactly `AttributiveAdjective::Untap` beside the declared keyword-action verb
`Untap`, and `TargetingMarker::Target` beside `CommonNoun::Target`.
Form-literal / vocabulary overlaps remain exactly: `additional` in
`additional_cost`; `to` in `up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and
`the` in `positional_partitive`.

Performance advisory, all on the final tree at 8 workers, against the 16.26s
quiet-host ceiling. Coverage check: 113 s at 144,521 ns/B, host load 7.58 /
9.17 / 10.28. Feature-side ambiguity: 106 s at 126,384 ns/B, host load 5.23 /
7.47 / 10.35. Fork-point ambiguity: 111 s at 124,872 ns/B, host load 7.43 /
8.45 / 11.21. True contention during these runs, stamped by the reviewer
because the executor sandbox cannot see it: 3 concurrent codex executors plus
1 other Opus reviewer. The ceiling excess is advisory, not fitted to.

### Review corrections

- HIGH: the parked tree failed `cargo xtask cite check --list-noncompliant`,
  which read this record's own bare-decimal coverage wall time as a rule
  number, while the record asserted that gate reported `0 non-compliant
  citation-looking string(s)`. Every time figure is now an integer with a
  unit, and the gate is green on the landed tree.
- MEDIUM: the executor's whole-workspace old-name sweep rewrote the completed
  ticket `docs/tickets/done/english-v2-tail-color-property-adjectives.md`,
  retro-fitting its stamped landing record to a symbol that did not exist when
  it was measured and turning its statement of the defect into a
  self-contradiction (`so vocab ColorWord names a colour system while holding
  a colour-property class`). That file is restored to its landed text; this
  ticket's scope is the code name, not the history.
- MEDIUM: the record was stamped on change `ysqkktxtsuzv` with lock `covered`
  19,564, a tree trunk had already moved past. Every number above is
  re-measured and re-stamped on the landed tree, including the corrected
  construction count (395, not 394) and licensing-checker total (23, not 21).
- MEDIUM: the selection-neutrality baseline is re-taken against the feature's
  fork point checked out in-workspace, with identical flags on both sides,
  rather than a parent tip that drifts as siblings integrate.
- LOW: the glossary entry grounded "the last three state an object's color
  property" on [CR#105.4] alone, which says only that *multicolored* and
  *colorless* are not colors. The claim is now carried by
  [CR#105.2a..105.2c], which define monocolored, multicolored, and colorless
  objects.
- LOW: this ticket's own problem statement had been rewritten to
  `vocab ColorWord replaces a false name`, erasing the defect it was minted
  for. It is restored to the as-minted wording.
