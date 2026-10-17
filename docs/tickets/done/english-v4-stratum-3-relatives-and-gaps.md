---
needs: [english-v4-stratum-2-cleanup]
---
# Stratum 3: Relative clauses and frame-derived slash gaps

Implement **Stratum 3** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§4.1, §4.2, §6.1):

- **Subject Relatives:** `that`, `who`, `which` with subject gap (`creatures
  that entered this turn`, `a player who controls an artifact`).
- **Reduced Participial Postmodifiers:** Present-participial (`creatures
  attacking you`, `a spell targeting ~`) and past-participial (`cards exiled
  with ~`) postnominal clauses.
- **Compiler-Derived Object-Gap and Stranded-Preposition Relatives:** Relative
  clauses with `that`/`whom`/`which` or bare (contact) relative clauses (`all
  permanents you control`, `the card you are putting into your hand`, `each
  creature ~ dealt damage to this turn`), derived uniformly across all verb
  forms (finite, plain, progressive, perfect, passive) from active verb and
  preposition frames.
- **Possessive, Pied-Piped, Free, and Adverbial Relatives:** `whose` relatives,
  `with which` / `for which` pied-piped relatives, free relatives (`whatever`,
  `whichever`, `what`), and temporal/locative adverbial relatives (`the next
  time ...`).

## Gate

- 100% of corpus units in Strata 0–3 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–3 judgment suite passes, including the `[[Seedborn Muse]]` attachment
  ambiguity witness (`"Untap all permanents you control during each other
  player's untap step."` retaining both matrix and relative-clause attachments).

Standard constraints apply.

## Deferral

By owner decision ten Stratum 3 units are deferred to
`english-v4-stratum-3-residual` (deictic *this game*, relative-clause
extraposition, and four one-off frames); the census lists them as regressions
until it lands.

Census delta (`vkyunppu` → this landing): faces read 9,499 → 9,926; total Readings 86,439 → 94,937; lost faces: Coat of Arms, Samite Elder (Stratum 5), Ghostly Possession, Heart of Light, Sandskin (Stratum 6), each read only through the removed fused demonstrative *that*. Strata 0–3: 62,551/62,561 units read, 0 known-wrong-only, 10 unread (deferred), 0 law failures.
Judgment set: seed and cumulative Strata 0–3 pass (1,582 enforced, 1,418 pending); tests and clippy pass for deckmaste_syntax, deckmaste_lexical_source_v4, deckmaste_english_v4 and xtask; citation checks pass.
