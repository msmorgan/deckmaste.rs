---
needs: [english-v4-s7-quoted-abilities]
---
# Stratum 7: Bare mentions and vote options

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read bare mentions selected by a head's frame:
- vote options (*votes for sickness or psychosis*, *If guilty gets more votes*, *each strength vote*), [CR#701.38a]; the five Path of the ... vote-option faces;
- *choose left or right*, *choose friend or foe*, *choose land or nonland*, *choose artifact, creature, or land*;
- *other than Wall* and *can't be Wall*;
- text-changing *replacing all instances of one color word with another* ([CR#612.1]; 10 faces profiled in by Stratum 6);
- quoted mentions, such as *the "legend rule"*, *"bands with other" abilities* and *"Cascade, cascade."* (14 more faces, which need the quoted-abilities slice; this is why it is a dependency);
- *with a name originally printed in the Antiquities expansion*, plus the vocabulary gaps for the expansion and dungeon names (Acererak, Sail into the West, Travel Through Caradhras).

A mention is selected by a head's frame, never free: the type, subtype and color words mentioned are CR-cataloged, and a free mention grows every forest. Vocabulary lives in the lexical catalogs (`deckmaste_lexical_source_v4`), not in grammar declarations.

## Grounding

- CGEL, Ch. 5 §8.2, p. 401.
- `docs/oracle-style-guide.md` §7, "Choosing a type or name"; §12, "Randomness, dice, coins, and voting".
- [CR#701.38a], [CR#612.1].

## Witness cards

- Bite of the Black Rose
- Mystic Barrier
- Cloud Key
- Stenn, Paranoid Partisan
- Alter Reality
- Travel Through Caradhras

## Estimated faces

About 48 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar and lexicon.

## Pending judgments this slice makes hold

`vocabulary_batch_2.txt:96–97` (vocabulary gaps).

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
