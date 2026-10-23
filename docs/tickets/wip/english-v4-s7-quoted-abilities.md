---
needs: []
---
# Stratum 7: Quoted granted abilities

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read quoted granted abilities:
- after *has*, *have*, *gain(s)* and *escapes with*, and as a Nominal postmodifier with *with* (`english-v4.md` §4.2, host Nominal only);
- coordinated with keywords and with each other (*has reach and "..."*; *"A," "B," and "C."*);
- in serial predicate lists (*gets +1/+0, has "...," and is a Wizard*);
- inside emblems (*You get an emblem with "..."*);
- nested single quotes;
- a quoted body that is a whole Line.

Today the frames already declare `QuotedAbility`, but no Production builds it.

Engine work: the quote delimiters, and Oracle's punctuation inside the closing quote (the sentence's period and the serial comma sit inside the quote). That punctuation is derived, never stored, so that round-trip law 1 holds. The quoted span re-enters the grammar's root Category. The engine must not name `QuotedAbility` (principle 6); the grammar declares the sub-root.

## Grounding

- CGEL, Ch. 20 §6, pp. 1753–1755.
- `docs/oracle-style-guide.md` §3, "Quotation marks and apostrophes"; §12, "Granting and removing abilities".
- [CR#113.1a].

## Witness cards

- Magma Sliver
- Food Fight
- Dungeon Delver
- Ozox, the Clattering King (*with "..." and "..."*)
- Harold and Bob, First Numens (nested quotes)

## Estimated faces

About 1031 faces (an estimate from the 2026-10-23 census, not a measured figure).
67 more faces also need `english-v4-s7-then-coordination`.

## Engine

Yes (certain).

## Pending judgments this slice makes hold

`granted_ability_quotes.txt:2–7,9` (line 8 is a negative and must keep holding); `quoted_ability_coordination.txt:2–11`; `articles.txt:21`; `grammar.txt:100–101`; `irrealis_scalar_difference.txt:4`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
