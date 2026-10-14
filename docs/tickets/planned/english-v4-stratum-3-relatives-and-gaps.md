---
needs: [english-v4-stratum-2-valency-passives-and-nonfinites]
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
