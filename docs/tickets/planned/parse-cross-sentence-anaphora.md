---
needs: [core-with-rebindable-that]
design: true
---
Parse multi-sentence effects whose follow-up sentences refer back to the
previous sentence's object: `Target creature gets +2/+2 until end of turn.
Untap it.`, `Exile target creature. Its controller loses 2 life.`,
`~ deals 3 damage to target creature. ~ deals 3 damage to that creature's
controller.`, `Then discard a card.` Today the first sentence parses and the
follow-up fails alone (probe 2026-07-16: 1,445 one-away cards fail on exactly
one sentence; the anaphor-headed follow-ups — `it` / `its` / `that <noun>` /
`then` — are the dominant shapes at ~1,110 cards).

**Why design-gated.** The emission needs a settled binding story: the parse
must hoist the first sentence's patient into a binder (`With`/`That`) so the
follow-up's `It`/`That` anaphor resolves — which shape (rebindable `That`,
`With` chaining, `Sequentially` + provenance) is a design call that
`core-with-rebindable-that` opens. Possessive anaphora (`its controller`,
`that creature's controller`) additionally needs the controller-of-That
reference form. Sub-shapes worth staging: (1) bare `Then <effect>.`
sequencing, (2) `<verb> it/that creature.` direct anaphors, (3) possessives.

Largest single lever in the corpus: multi-sentence clauses touch ~6,000
one-away cards; this ticket's staged shapes cover ~1,110.

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask idris-check` on graduated anaphora cards (anaphora soundness is
exactly what the Idris gate exists for).
