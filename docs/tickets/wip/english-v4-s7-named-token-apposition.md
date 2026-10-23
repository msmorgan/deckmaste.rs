---
needs: []
---
# Stratum 7: Named-token apposition

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read a supplementary appositive Noun Phrase after a token's Introduced Name (*create Voja, Friend to Elves, a legendary 3/3 ... token*). The name itself may contain a comma. The appositive keeps its *with* postmodifiers.

## Grounding

- CGEL, Ch. 15 §5, p. 1357 (Apposition).
- CGEL, Ch. 5 §14.3, pp. 447–448.
- `docs/oracle-style-guide.md` §12, "Tokens" (appositive form required for a named token).

## Witness cards

- Rinoa Heartilly
- Minsc, Beloved Ranger
- Stangg
- Tamiyo, Compleated Sage
- Tolsimir, Friend to Wolves

## Estimated faces

About 27 faces (an estimate from the 2026-10-23 census, not a measured figure).
12 more faces also need `english-v4-s7-quoted-abilities`.

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`named_token_apposition.txt:2–6`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
