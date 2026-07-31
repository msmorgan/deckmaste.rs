---
needs: []
---
**The same keyword ability parses one way at top level and another inside a
quote.** Found while retesting `QuotedAbility.initial_uppercase` after round
`kwbandsother` (2026-07-30).

Animate Dead, verified with `cargo xtask english inspect`:

| position | text | parse |
|---|---|---|
| top level | `Enchant creature card in a graveyard` | `Keyword(Enchant)`, argument `Qualified(…)` |
| inside a quote | `"enchant creature card in a graveyard"` | `Paragraph` — an ordinary clause |

Same keyword, same phrasal argument, same catalog entry (`Enchant` is
long-catalogued — unlike `bands with other`, this needs no new registration).
Only the position differs, and the quoted one loses its keyword reading.

Witnesses: **Animate Dead**, **Dance of the Dead**, **Necromancy** — all the
`enchant <quality>` Aura-rewrite family, where the quote grants a replacement
enchant line (`becomes an Aura with "enchant creature put onto the battlefield
with this Aura."`).

## Why this matters beyond the tree

It is the last thing standing between the diet and deleting
`QuotedAbility.initial_uppercase`. Retested after `kwbandsother`, the node-kind
derivation (`keyword or recovered or flavor-header interiors are reproduced
verbatim and must not be re-cased; everything else takes the capital`) now
mismatches only **4** faces, down from 12 before the banding fix:

- **Animate Dead, Dance of the Dead, Necromancy** — this bug. They *should* be
  keyword interiors, which the derivation already predicts correctly.
- **Ogre Marauder** — `it gains "this creature can't be blocked" until end of
  turn`. A genuinely parsed sentence interior, printed lowercase, against
  Takklemaggot's `gains "At the beginning of that player's upkeep, …"`, a
  genuinely parsed sentence interior printed capitalized. That pair is a real
  refutation and survives this ticket.

So fixing this alone does **not** free the field — it reduces the refutation to
a single honest witness pair. Whether one witness pair is enough to keep a
stored bit is then a judgement call worth revisiting, and the field's doc
comment should be updated either way.

Rejected while measuring: casing does **not** track whether the quote closes its
host sentence. Deriving `capitalize == terminal_period` (a bit the renderer
already computes) scores **21** mismatches, worse than the 4 above. Do not
retry it.

## Leads

Both paths eventually parse a keyword line, so the divergence is in how the
quoted fragment reaches that grammar. `parse_quoted_ability_fragment` re-lexes
the interior and calls `parse_ability` on it; compare what the top-level
paragraph path passes for the argument gates (`bare_object`, `carries_from`,
`in_list`) against what the quoted path passes. Initial casing is a red herring
— `bands with other` matches its `"Bands with other"` canonical from a lowercase
surface, so the keyword-ability case policy is already lenient.

## Verify

Animate Dead, Dance of the Dead and Necromancy parse their quoted interiors as
`Keyword(Enchant)` with a `Qualified` argument, matching the top-level line.
Round-trip stays 31685 clean. The recovery census should be unchanged — these
faces parse cleanly today (wrongly), so this is a tree fix, not a recovery fix.
Then re-run the casing derivation described above and record the new count.
Standard constraints apply.
