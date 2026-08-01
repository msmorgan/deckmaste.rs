---
needs: []
---
**A current-Modern-scoped structural-recovery gate — make "no structural
recovery" declarable for the corpus slice the project actually targets.** The
ticket census already defines its universe as Modern-legal names
(`docs/tickets/README.md`), and `english-structural-recovery-zero` re-scoped
the corpus-wide bar to ≥90% once family rounds hit the singleton ceiling. The
residue partition (`english-structural-recovery-long-tail`, "Format scope of
the residue") shows current Modern keeps 1860 of the 3111 failing names /
2065 of 3477 spans, with the same composition-machinery profile as the full
tail. A Modern-scoped census makes progress against the census universe
directly measurable, and zero declarable, without waiting on Unfinity and
Commander-only text.

Design (three small pieces, no derived-schema change):

1. **Scope fixture.** A generated name list of currently-Modern-legal cards,
   derived at generation time from `data/mtgjson/AtomicCards.json`
   (`.legalities.modern == "Legal"`, name-level; faces resolve through the
   corpus rows' `face`→`name` mapping). The fixture header records the
   MTGJSON snapshot date (`Meta.json`); regeneration is deliberate, never
   implicit — gate membership must not drift under a data refresh.
2. **`--scope` filter on the english instruments.** `cargo xtask english
   recovery|unknown --scope modern` filters supported faces through the
   fixture after the `supported` cut, so census counts, dumps, and
   `--require-complete` all take the scope.
3. **The gate.** `cargo xtask english recovery --require-complete --scope
   modern` becomes the declarable zero; the corpus-wide ≥90% bar is unchanged
   and stays primary until this one is green.

Decisions taken here so they don't reopen: "Modern" means
`legalities.modern == "Legal"` — the 13 currently-failing Modern-banned names
are out of scope (cardpool-vs-legal can be revisited when a consumer needs
it). An ever-Standard variant is deliberately NOT built: historical legality
is not queryable (MTGJSON legalities are current-only), so it would need a
set-type proxy plus an inception-era adjudication of 89 names, for only +257
names of scope; the measurement lives in the long-tail ticket if that ever
becomes worth doing. `format-deck-validation`'s derived-schema `Legalities`
extension is NOT a prerequisite — the fixture reads mtgjson directly; if that
ticket lands later, generation can switch to derived data.

Standard constraints apply.
