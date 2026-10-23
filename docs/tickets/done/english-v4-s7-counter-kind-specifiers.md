---
needs: []
---
# Stratum 7: Counter-kind specifiers

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Expose the specifier of a compound counter lexeme without splitting the lexeme (`docs/decisions/english-lexical-analysis.md` §2, multiword terms). This covers:
- coordinated counter kinds before *counter*: *your choice of a +1/+1, first strike, or vigilance counter* (Me, the Immortal; Owen Grady; Denry Klin; The Night of the Doctor; Reluctant Role Model; 2 more faces also need the stranded-auxiliaries slice);
- counter kinds listed as a choice (Crystalline Giant);
- a Preposition Phrase complement extraposed past a relative clause (*Equip abilities you activate of other Equipment*, Bladehold War-Whip), which rides along.

## Grounding

- [CR#122.1].
- `docs/oracle-style-guide.md` §12, "Counters".

## Witness cards

- Me, the Immortal
- Owen Grady
- Crystalline Giant
- Bladehold War-Whip

## Estimated faces

About 7 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

Possibly (lexical source or engine).

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

1. **Census delta** (`tszlxwsn` against claim parent `rymsmopq`): faces read 32,574 → 32,583 (+9: Me, the Immortal; Owen Grady, Raptor Trainer; Reluctant Role Model; The Night of the Doctor; Denry Klin, Editor in Chief; Crystalline Giant; Bladehold War-Whip; Assaultron Dominator; T-45 Power Armor); total Readings 315,929 → 315,959; lost faces: none.
2. **Judgment set**: `cargo test -p deckmaste_english_v4` passes; the ten new entries in `judgments/counter_kind_specifiers.txt` hold.
