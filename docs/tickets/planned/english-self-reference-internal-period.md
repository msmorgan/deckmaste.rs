---
needs: []
---
**A self-reference nickname containing a period is severed by the sentence
splitter.** `split_sentences` cuts at every top-level `Period` token with no
knowledge of the face's own name, so a nickname like `U.S.Agent` or `Ms. Marvel`
is torn into fragments before the chart ever sees it.

Found when the Marvel set went vintage-legal and entered the supported corpus
(2026-08-04), breaking the structural round-trip gate.

`U.S.Agent, John Walker`, verified with `cargo xtask english inspect`:

| source | parse |
|---|---|
| `When U.S.Agent enters,` (trigger condition) | `ThisCard(AbbreviatedName)` — correct |
| `Attach it to U.S.Agent.` (effect paragraph) | three sentences: a clause `Attach it to <opaque "U">`, then `Recovered("S.")`, then `Recovered("Agent.")` |

The fragments rejoin through `join_words`' plain `.join(" ")`, so the card
renders `Attach it to U. S. Agent.` and fails the round trip.

## Why full names are safe and nicknames are not

`collapse_full_names` fuses full-name occurrences into one `FullSelfReference`
token *before* splitting, which is what keeps `Aang, A Lot to Learn` from
dividing at its comma. Nicknames are deliberately left for the chart to
disambiguate — a nickname can collide with ordinary vocabulary (`Sliver` from
`Sliver Queen` is also a creature type), so eager collapse is wrong. But the
chart runs *after* the sentence splitter has physically severed the token run.
Commas survive that ordering because their split is structural and recoverable;
periods do not.

`Black Waltz No. 3` confirms the discriminator: its text spells the **full**
name, gets collapsed, and parses correctly.

## Blast radius is wider than the failing gate

Only `U.S.Agent` fails the round trip, because it is the one case where a
truncated fragment happened to parse into a clause and got space-joined on the
way out. The others are equally mis-parsed and round-trip anyway — a `Recovered`
node stores an exact source span and renders verbatim:

- `Ms. Marvel, Kamala Khan` — `Recovered("Until end of turn, Ms.")`, a sentence
  severed mid-nickname.
- `M.O.D.O.K.` — `Recovered("M.O.D.O.K. Connives.")`, the fragment treated as
  sentence-initial. Masked twice: `normalized_rules_text` normalizes sentence
  case on both sides of the comparison, so the spurious capital never shows.

Witnesses: **U.S.Agent, John Walker**, **Ms. Marvel, Kamala Khan**,
**Ms. Marvel, Elastic Ally**, **M.O.D.O.K.**, **J. Jonah Jameson**,
**Mr. Foxglove**. More arrive with every period-bearing legendary printed.

## Shape of the fix

Suppress the sentence boundary at periods **strictly interior** to a candidate
nickname token-run match, leaving the actual disambiguation to the chart. Do not
extend `collapse_full_names` to nicknames — that discards the ambiguity
resolution it was designed to preserve.

A name-final period stays a terminal: it does double duty as the sentence
terminator, which the renderer already models in
`clause_ends_with_terminated_self_reference`. [CR#201.5c] licenses treating the
shortened form as the full name, so both forms must survive splitting intact.

Assert the fix on parse **shape**, not the round trip — the round-trip gate
demonstrably cannot see the masked cases above.
