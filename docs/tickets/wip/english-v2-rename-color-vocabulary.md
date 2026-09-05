---
needs: []
---
**`vocab ColorWord` replaces a false name.** Its eight members combine the five Color
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

Measured on change `ysqkktxtsuzv` after a changed `kata refresh`, with lock
covered 19,564. The harmony repair helper then reported `workspace is clean`.

PROVE. Renamed the eight-member vocabulary to `ColorWord`, its five
`constructions.rs` consumers, the public AST re-export, the generated visitor
re-export, and the exact lexical-provenance assertion. The eight members and
all construction bodies, scanners, selection rules, and lexical licences are
otherwise unchanged. The report-mode coverage check left
`english-v2-coverage.lock` byte-unchanged: 19,564 -> 19,564 covered, with no
loss or gain entries (+0/-0). Coverage reports 0 selected-uncovered units, 0
roundtrip mismatches, 0 ownership failures, 866,873/866,873 construction
visits, 302,117/302,117 leaf visits, and 0 traversal failures. Full roundtrip
is clean for all 19,564 accepted units. Full ambiguity reports 0 unresolved
ties and 0 internal failures.

The refreshed parent-tip and feature ambiguity reports were diffed by identity
over `status`, `internal_kind`, and the complete selection `decision`. The
per-unit selection diff is empty: 0 losses, 0 gains, 0 changed construction
paths, 0 changed specificity vectors, and 0 changed resolution kinds across
32,641 units. Construction declarations remain 394 -> 394. The coverage
inventory reports 21 permitted and 0 forbidden licensing checkers; the
environment and closure tests report no load error or word-naming guard.

The reverse-dependency closure printed and ran:

```text
cargo test -p deckmaste_english_v2 -p xtask
cargo clippy -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
```

Representative positive artifacts from that run:

```text
test result: ok. 146 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 112 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 468 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.18s
```

Citation gates for the glossary change report `0 non-compliant
citation-looking string(s)`, `checked 14486 citations against cr.txt (eff.
2026-08-07); 0 stale`, and `audited 3 citation site(s)`; the audit text matches
each claim.

DISCLOSE. Selection census is unchanged: 19,564 selected / 15,510 unique /
4,054 specificity-resolved before and after. The specificity share did not
rise, so there is no construction pair to name. No identity became covered and
no identity stopped being covered. The raw ambiguity JSON differs only in four
parse-failure expectation messages: Become the Pilot, Firemane Commando,
Shield Broker, and Subjugate the Hobbits now say `color word` where the parent
said `color`. Their failure spans, statuses, and complete selection decisions
are unchanged; this is the requested vocabulary display rename, not a parsing
or selection change.

Glossary gap: the Oracle-English glossary had no term for one of these eight
Word Forms. This landing adds **Color Word**, with the required CR grounding,
and uses its identifier spelling `ColorWord`. Deviations and additions: the
ticket's measured source blast radius omitted `visit.rs`; compilation exposed
the vocabulary visitor re-export, so it is necessarily re-spelled
`walk_color_word`. Outside `deckmaste_english_v2`, the glossary, this ticket,
the fog routing note, and the predecessor ticket's source-name references are
updated because the ticket requires a whole-workspace old-name sweep. No
construction or test was added or deleted. Assurance counts: restored 0;
re-spelled 1 (the existing lexical-provenance assertion); ignored 0; added 0;
removed 0. STOPs: none.

REPORT. Coverage-lock identities: 19,564 -> 19,564; construction declarations:
394 -> 394. Licensed vocabulary/lexicon homographs remain exactly
`AttributiveAdjective::Untap` beside the declared keyword-action verb `Untap`,
and `TargetingMarker::Target` beside `CommonNoun::Target`. Form-literal /
vocabulary overlaps remain exactly: `additional` in `additional_cost`; `to` in
`up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and
`the` in `positional_partitive`.

Performance advisory: the authoritative refreshed-tree coverage check used 8
workers and took 130.771689063 s at 172,884 ns/B, against the 16.26 s quiet-host
ceiling, with host load 19.82 / 24.14 / 21.51. Concurrent-process count is not
visible from the sandbox; the reviewer stamps contention. The ceiling excess
is advisory, not fitted to.
