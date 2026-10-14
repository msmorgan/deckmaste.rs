---
needs: [english-v4-stratum-6-coordination-and-rnr]
---
# Stratum 7: Discourse sequencing, replacement effects, and the corpus long tail

Implement **Stratum 7** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6):

- **Replacement and Prevention Effects:** `If <Clause> would <VP>, <Clause>
  instead` and `Prevent the next N damage that would be dealt ...`.
- **Discourse Sequencing and Pro-Forms:** `Then ...`, `Otherwise, ...`, `If you
  do, ...`, `If you don't, ...`, `When you do, ...`, `Repeat this process ...`,
  and auxiliary stranding / `do so`.
- **Anaphora, Supplements, and Quoted Grants:** Reflexive/reciprocal pronouns
  (`itself`, `themselves`, `each other`), `with` / `without` counter and ability
  supplements (`enters with two +1/+1 counters on it`), and embedded quoted
  abilities (`gains "<Document>"` and `"<Ability>," "<Ability>," and "<Ability>"`).
- **Closing the Corpus Tail Under the Long-Tail Rule:** Cover the remaining
  Stratum 7 families without introducing policing flags (`...Use`, `...Licence`,
  `...Present`). If any outlier card's wording cannot be parsed by a clean
  general schema/production or a catalog-declared lexical idiom without a
  policing flag, **STOP and surface the witness to the user** to decide between a
  catalog idiom, a general production, or an explicit documented deferral
  (`unread`).

## Gate

- 100% of non-deferred Vintage-legal Oracle corpus units across Strata 0–7 parse
  as **read** (0 `known-wrong-only`) and satisfy both round-trip laws
  (`v is an admitted Reading of s => render(v) = s` and
  `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Full linguistic judgment suite passes.
- Zero CR-cataloged words appear directly in the grammar data file, and zero
  policing flags exist.

Standard constraints apply.
