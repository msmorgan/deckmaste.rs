---
needs: []
---
# Stratum 7: *Instead of* complements and the replacement tail

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read the replacement-effect forms where *instead* still fails:
- *instead of* with a Preposition Phrase or Gerund complement (*... instead of into that player's graveyard*; *Instead of drafting ...*);
- *may instead*;
- the *reduce [it] to N / to less than N* frame of *reduce*, which also fails without *instead*;
- *that many dice plus one*, *that many times plus an additional time*;
- the remaining replacement effects where *instead* fails.

## Grounding

- [CR#614.1a].
- `docs/oracle-style-guide.md` §13, "Would and instead".
- CGEL, Ch. 7 (complex prepositions); verify the section while drafting.

## Witness cards

- Remand
- Memory Lapse
- Worship
- Moonlit Meditation
- Twinning Staff
- Eye for an Eye

## Estimated faces

About 33 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar and lexicon.

## Pending judgments this slice makes hold

`preposition_function_licences.txt:3`; `instead_replacement.txt:18`; `next_time_relative_adjunct.txt:19`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

1. **Census delta** (measured on `zwpmlwww` → `sumoowlk`): faces read 30,365 → 30,389;
   total Readings 277,002 → 278,082; Surplus 116,303 → 116,513; 67 units changed (24 newly
   read, 43 grew, none shrank); lost faces: none.
2. **Judgment set** (`sumoowlk`): 3,087 enforced and 249 pending, all enforced holding; the
   three named entries and the 13 added ones hold.
