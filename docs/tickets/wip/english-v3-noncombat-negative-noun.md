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

To be written by the claimant (six items, `docs/tickets/README.md`).
