---
needs: [english-v2-closed-class-single-owner, english-v2-require-through-optional-role, english-v2-verb-frame-vocabulary]
---
**Move verb-selected prepositions into declared Verb Frame data.** Under the
[`Verb Frame` and `Complement` definitions](../../contexts/oracle-english/CONTEXT.md), the selected
preposition is part of what the lexical schema licenses; `valency` is not a
catch-all name for the schema, its instantiated phrase, and its realization.

Replace the 20 hardwired `to`/`from`/`on`/`into`/`for`/`onto`/`at` form
literals with vocabulary `Preposition` claims in selected-complement positions.
The core-verb seed and Keyword Action grammar contributions declare the marker,
and the frame construction consumes it. Close the optional-slot workaround at
the same seam. The `form_literal_vocab_overlaps` count decreases by these 20
with zero coverage change; standard constraints apply.

## Landing record

Measured on refreshed change `lxnpvmzupswmpkxkvwplmunzsovquzsq` with
16,824 covered lock identities. The refreshed-parent lock and measured lock
both have 49,474 lines and SHA-256
`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`.

| gate | claimed parent | refreshed parent | measured tree |
| --- | ---: | ---: | ---: |
| selected and covered units | 16,771 | 16,824 | 16,824 |
| ordinary parse failures | 15,870 | 15,817 | 15,817 |
| unique selections | 11,515 | 11,527 | 11,527 |
| specificity-resolved selections | 5,256 | 5,297 | 5,297 |
| unresolved ties | 0 | 0 | 0 |
| construction declarations | 395 | 397 | 397 |
| licensed vocabulary/lexicon homographs | 2 | 2 | 2 |
| form-literal/vocabulary overlaps | 25 | 25 | 5 |
| coverage-lock identities | 16,771 | 16,824 | 16,824 |

- Implementation: forms can now consume a fixed vocabulary value as
  `lex(Preposition::Variant)`. `VerbFrame` data carries the corresponding
  `Lex` atom, and its optional form carries `OptionalLex`; the latter replaces
  the generic optional source-role workaround without making literals
  optional. The core-verb seed and the Attach, Exchange, Search, Shuffle, and
  Vote Keyword Action contributions declare their markers. Rendering,
  ownership, visitation, and frame-key matching all consume those declared
  atoms. Preposition classes remain the authority validated by the existing
  frame constructions.
- Coverage and ownership: against both the claimed and refreshed parents,
  this change has +0/-0 lock identities and no lock-byte change. Newly covered
  identities: none. The refreshed tree has 16,824 selected and covered units;
  selected-uncovered units, internal failures, exception resolutions,
  exception uses, round-trip mismatches, ownership failures, gaps, overlaps,
  synthetic claims, provenance-plan mismatches, and traversal failures are all
  zero. In the change-isolated pre-refresh measurement, the 20 live
  selected-complement collision entries moved from form ownership to
  vocabulary ownership: form claims/bytes changed 123,464/236,235 ->
  115,545/209,097 and vocabulary claims/bytes changed 54,289/221,495 ->
  62,208/248,633; total claims and bytes remained exactly 359,147 and
  1,523,802. The refreshed tree's absolute totals are 361,023 claims and
  1,531,738 bytes. The twentieth live overlap was the selected `to` in
  `ScalarEquality`; accounting for it makes the measured ceiling decrease
  exactly 25 -> 5.
- Selection census: the change-isolated full row comparison preserved all
  32,641 identities, statuses, selected renderings, resolution classes,
  survivors, candidate ordinals, and construction paths. After refresh, the
  parent and measured tree are both exactly 11,527 unique and 5,297
  specificity-resolved selections, with zero ties, zero exceptions, and
  15,817 parse failures. Fixed vocabulary markers re-spell affected positions
  from literal to typed lexical specificity without changing a candidate set,
  survivor, attachment, or winner. The +53 selected/covered units and two
  added constructions between claim and refresh belong to the crossed
  targeting-marker landing, not this change.
- Positive artifacts after refresh: `cargo fmt --all -- --check`; strict
  clippy for `deckmaste_construction_core`, `deckmaste_english_v2`, and
  `xtask`; `cargo test -p deckmaste_english_v2 -p xtask`; `cargo test
  --workspace`; coverage `--check`, ambiguity `--require-resolved`, and
  round-trip `--require-clean`, each with `--workers 8`; and `cargo xtask cite
  check` all exited zero. Citation freshness reports `checked 14162 citations
  against cr.txt (eff. 2026-08-07); 0 stale`; the post-refresh noncompliant
  list is empty. Before refresh, that list contained the two known loose
  ability-taxonomy citations plus one already-corrected loose cleanup-step
  citation from the stale parent; no file outside this ticket's scope was
  edited.
- Performance advisory after refresh: with 8 workers, coverage took
  85.011973249 s at 110,460 integer thread-CPU ns/B under host load
  11.47/11.46/13.98; ambiguity took 86.541924506 s at 123,085 ns/B under load
  11.23/11.59/13.80; round trip took 85.400588523 s at 108,902 ns/B under load
  11.38/11.35/13.50. The worker-capped measurements exceeded the 16.26 s
  ceiling under load. Concurrent sibling process count is unavailable in the
  sandbox and requires the reviewer contention stamp.
- Assurance census: restored 0; re-spelled 12 existing test functions;
  ignored with blockers 0; added 0; removed 0. The re-spelled assertions cover
  fixed and optional vocabulary tail parsing, normalized frame keys, plugin
  frame data, exact core frames, typed ownership and specificity, and the
  lowered collision ceiling.
- Deviations and additions: no construction or test function was added or
  deleted. No dominance edge, exception entry, narrowed form, row-specific
  licence, or identity-specific guard was added. The implementation follows
  both 2026-09-03 rulings: the declared selected role retains preemption, and
  no adjunct licence is restored. The first refresh produced two textual
  conflicts in nominal witness rows: harmony combined the concurrent
  targeting-marker construction paths with this change's typed-lexical
  specificity. The harmony conflict list is empty, and the refresh retry was
  a no-op. A later final refresh crossed the unrelated
  `engine-bare-scopes-and-notes-map` coordinator change without conflict; the
  complete gate set was rerun and produced the final figures above.
- STOPs: none. No coverage drop, selection change, attachment misselection,
  tie, negative-oracle admission, ticket/ruling contradiction, glossary gap,
  or decision request was found.
