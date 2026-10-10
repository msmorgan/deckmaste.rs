---
needs: []
---
# Declare the negative formation of *combat*: Whenever an opponent is dealt noncombat damage, …

A data ticket. No `declarations.rs` edit is expected.

## Why

*noncombat* is not a word of the lexicon: NP "noncombat damage" 0, "excess
noncombat damage" 0, "a noncombat phase" 0, beside "combat damage" 1, "excess
combat damage" 1, "a noncreature spell" 1. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): failing token `noncombat` 37, key
`at:dealt noncombat` 9. `cargo xtask english-v3 grep -i '\bnoncombat\b'`
finds **41** faces, all unread; **37** have that line as their only failing
line. Every one is *noncombat damage* (`'\bnoncombat damage\b'` 41 faces;
1,120 faces contain *combat damage*).

## Goal

*noncombat* reads as the negative-prefix form of the noun *combat* in noun
Premodifier function wherever *combat* premodifies today (*noncombat damage*,
*excess noncombat damage*, *all noncombat damage that would be dealt …*).

## Analysis

No new grammar and no CGEL claim beyond what
`docs/decisions/english-lexical-analysis.md` already settled (amendment
2026-10-04: "Noun premodification is a function of the existing noun, not a
duplicate Adjective Lexeme. Negative noun Lexemes retain their source noun's
declared inflectional paradigm"; ruling 2026-10-08,
`english-v3-negative-common-noun-premodifier`: "a common noun may declare the
negative formation when the corpus attests the negative form"). The corpus
attests it on 41 faces.

Nearest existing declaration: `negative_noun_formations:
{"lexeme:CommonNoun/Token": Joined}` in
`crates/deckmaste_lexical_source/lexicon/core.ron`, consumed by
`add_negative_noun_formations`
(`crates/deckmaste_lexical_source/src/native.rs`), which finds the owner by
lexeme id among all loaded lexemes, requires a Noun with a Singular form, and
mints the negative lexeme with `NounPremodifier: Yes`. *combat* is
plugin-owned: `lexeme:turn_part/combat`
(`plugins_v2/builtin/macros/turn_parts/combat.ron`, `grammar: Noun(singular:
"combat")`). It reads as Premodifier of *damage* today through
`NounPremodifiedNominal`.

Orchestrator rulings 2026-10-10:

- NC1. Declare the formation as data: add `"lexeme:turn_part/combat": Joined`
  to `negative_noun_formations`. The plugin definition keeps owning the noun;
  the negative lexeme's Provenance names that source declaration, as for
  *nontoken*. No Adjective *noncombat* and no core noun *combat* is minted.
- NC2. If the owner ruling for plugin terms (V3 of
  `docs/tickets/done/english-v3-vocabulary-batch-1.md`) is read as requiring
  the formation on the owning definition's `grammar:` row instead, that is a
  schema question for the orchestrator: STOP with the one-line data change
  measured, do not add a field to the `Noun` row unasked.
- NC3. The minted lexeme must take the same Premodifier route as *combat*
  (`licensed_noun_premodifier` over the head's declared permission). If
  `NounPremodifier: Yes` does not select *damage* the way *combat*'s own
  value does, copy the source noun's declared value in the loader (a general
  rule: the negative retains its source's premodifier class), never a test on
  the word.
- NC4. One analysis per string: *noncombat damage* reads once, as negative
  noun Premodifier + head. Hyphenated *non-combat* is unattested (0 faces);
  do not declare it.
- NC5. Other uses of the negative (*a noncombat phase*) are unattested (all
  41 faces are *noncombat damage*); they may read as a consequence and are
  not blocked. Newly covered faces are gains; no Reading-count change on a
  covered face is expected (the word occurs on none). Report any change on
  the 1,120 *combat damage* faces: it would be a defect.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Chandra's Spitfire: "Whenever an opponent is dealt noncombat damage, this
  creature gets +3/+0 until end of turn."
- Niv-Mizzet, Visionary: "Whenever a source you control deals noncombat
  damage to an opponent, you draw that many cards." (probed 0 with "draw a
  card"; control with *combat damage* 3)
- Mark of Asylum: "Prevent all noncombat damage that would be dealt to
  creatures you control." (probed 0 as "… dealt to you."; control with
  *combat damage* 5)
- Wildfire Elemental: "Whenever an opponent is dealt noncombat damage,
  creatures you control get +1/+0 until end of turn."
- Chandra's Pyreling: "Whenever a source you control deals noncombat damage
  to an opponent, this creature gets +1/+0 and gains double strike until end
  of turn."

Controls that must not change: NP "combat damage" 1; NP "excess combat
damage" 1; NP "a noncreature spell" 1; Wall of Essence "Whenever this
creature is dealt combat damage, you gain that much life." 1 (census);
Coastal Piracy "Whenever a creature you control deals combat damage to an
opponent, you may draw a card." 3 (census); Jaheira's Respite "Prevent all
combat damage that would be dealt this turn." 10 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\bnoncombat\b|\bcombat damage\b|\bnontoken\b

(1,476 faces) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost, zero changed among covered faces. `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end;
lexicon data loads at runtime, so NC1 needs no grammar rebuild. Tests assert
the role (the negative noun lexeme, its Provenance, its Premodifier function
over *damage*), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- The 4 faces with a second failing line (Taii Wakeen, Chandra's Incinerator,
  Aether Revolt, Artist's Talent) and lines blocked independently (*that much
  damage plus 1*, key `at:many plus`; *4 or more noncombat damage*).
- Negative formations of other plugin nouns.

## Landing record

1. **Census delta** (`mzkvyunrqtrw`, clean claim-parent state before / edited
   state after; claim `vlmxklpr`). Bounded manifest: the Method's grep pattern
   found 1,476 faces; union with 300 covered controls (seed 20261008) is 1,765
   faces. Covered 1,524 → 1,555; total Readings 74,106 → 75,146. Official
   `english-v3 diff --lines`: gained 31, lost none, decreases 0, increases 0,
   digest-only 0, unchanged 1,734; issues 0. Every previously covered face,
   including every covered *combat damage* and *nontoken* face, is unchanged.
   Evidence: `target/english-v3/english-v3-noncombat-negative-noun/`:
   `manifest.json`, `before.json`, `after.json`, `diff.json`. Post-refresh
   (`lnskowyvyylt`): covered 1,524 → 1,556, Readings 74,106 → 75,167;
   gained 32, lost/decreased/increased/digest-only 0, unchanged 1,733.
   `refresh-diff.json` isolates Felix Five-Boots (0 → 21) from trunk's
   cause-catenative change `nzmrrqky`; this ticket's gain remains 31.
   The refresh incorporated `nzmrrqky` and `tlpwnxqn`; every previously
   covered face is still unchanged. Final accepted refresh incorporated
   kind-of-counter change `wkooyukozlpw`: covered 1,524 → 1,558, Readings
   74,106 → 75,325; gained 34, lost/decreased/increased/digest-only 0,
   unchanged 1,731. `final-refresh-diff.json` isolates Blitzball Stadium
   (0 → 151) and Skyship Plunderer (0 → 7) from that trunk change; this
   ticket still gains 31 faces. All 1,765 selected faces were re-censused
   with 6 workers and samples-per-face 0; issues 0. The subsequent accepted
   refresh incorporated temporal-supplement change `mpvousxzotns`;
   `temporal-refresh-diff.json` reports all 1,765 faces unchanged, including
   every covered face. The final claim-parent totals remain 34 gains
   (31 this ticket, 3 trunk), 0 losses/decreases/increases/digest-only.
   Integration was then refused because default moved. The retry refresh
   incorporated preposed-duration change `rtzolrtxrrru`;
   `duration-refresh-diff.json` isolates Aerial Extortionist (0 → 9) and
   Dream-Thief’s Bandana (0 → 217), with every previously covered face
   unchanged. Final totals: covered 1,524 → 1,560, Readings 74,106 → 75,551;
   gained 36 (31 this ticket, 5 trunk), lost/decreased/increased/digest-only
   0, unchanged 1,729; issues 0.
2. **Witnesses** (`mzkvyunrqtrw`). Every ticket witness re-probed before the
   edit at 0; none dropped. Sentence Readings after / positive-control
   Readings: Chandra's Spitfire 1/1; Niv-Mizzet, Visionary 3/3; Mark of Asylum
   12/12; Wildfire Elemental 1/1; Chandra's Pyreling 6/6. Whole-face census
   Readings respectively 2, 5, 12, 1, 6 (other lines contribute to the sum).
   NP *noncombat damage*, *excess noncombat damage*, *a noncombat phase*:
   0 → 1 each. Controls unchanged: *combat damage* 1, *excess combat damage*
   1, *a noncreature spell* 1; Wall of Essence 1, Coastal Piracy 3,
   Jaheira's Respite 10. W9-A preserves the controls' independent ambiguity.
3. **Spot checks** (`mzkvyunrqtrw`). Chandra's Spitfire, Niv-Mizzet,
   Visionary, Mark of Asylum, Wildfire Elemental, Chandra's Pyreling,
   Asgardian Inspiration, Blessed Sanctuary, Grisly Sigil, Soul-Scar Mage,
   Solphim, Mayhem Dominus: each selected Reading has
   `lexeme:turn_part/combat/non` as noun Premodifier over *damage*, using
   `NounPremodifier` / `NounPremodifiedNominal`, and realizes byte-exactly.
   Unintended-analysis gains found: 0.
4. **Deviations and STOPs** (`mzkvyunrqtrw`). NC1 is data-only; no loader,
   grammar, plugin schema, or glossary change. NC6 accepts the plugin owner's
   formation in `negative_noun_formations`; NC3 needs no loader adjustment
   because *combat* already declares `NounPremodifier: Yes`. No superseded
   route exists to retire. Hyphenated *non-combat*: grep 0 faces; undeclared.
   Seven role/provenance/roundtrip tests registered once in `suite_6`.
   Chandra's Incinerator gains coverage because its independent blocker was
   already resolved on the claim parent; ten *noncombat* faces remain unread
   for other material. No STOP or glossary gap.
5. **Tests** (`mzkvyunrqtrw`). `cargo test -p deckmaste_english_v3`: PASS,
   908 tests; `cargo test -p deckmaste_lexical_source`: PASS, 84 tests.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`: PASS after
   removing a redundant clone of a Copy identifier in the new test helper;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: PASS.
   Restored 0 / re-spelled 0 / ignored 0 / added 7 / removed 0; no removals
   to justify. After refresh (`lnskowyvyylt`), `cargo build -p xtask`: PASS;
   the bounded manifest re-census: PASS, 0 issues; both full `cargo test -p`
   commands above: PASS (English 916; lexical-source 84); both exact clippy
   commands above: PASS. Final accepted refresh: `cargo build -p xtask`
   PASS; the same bounded manifest re-census PASS; both full test commands
   PASS (English 921; lexical-source 84); both exact clippy commands PASS.
   Earlier English totals corrected from the saved logs (908 then 916).
   The subsequent temporal refresh also passed `cargo build -p xtask`,
   the same bounded re-census, both full test commands (English 925;
   lexical-source 84), and both exact clippy commands. Check logs use
   `final-` and `temporal-` prefixes in the evidence directory. The duration
   retry refresh likewise passed `cargo build -p xtask`, the bounded
   re-census, both full test commands (English 929; lexical-source 84),
   and both exact clippy commands; logs use the `duration-` prefix.
   Citation checks: no noncompliant or stale citations;
   diff audit
   has no citations to register or verify. No Lean or xtask test changes.
6. **Timing** (`mzkvyunrqtrw`). Main pre-refresh AFTER bounded census: 157610 ms wall
   under `/usr/bin/time -v`, 6 workers, samples-per-face 0. No full-corpus run.
