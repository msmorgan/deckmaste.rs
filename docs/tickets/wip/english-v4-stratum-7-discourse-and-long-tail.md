---
needs: [english-v4-s7-then-initial, english-v4-s7-then-coordination, english-v4-s7-quoted-abilities, english-v4-s7-do-so-and-the-same, english-v4-s7-stranded-auxiliaries, english-v4-s7-named-token-apposition, english-v4-s7-instead-tail, english-v4-s7-mentions-and-votes, english-v4-s7-cleave-brackets, english-v4-s7-gapping-and-ellipsis, english-v4-s7-quantifiers-and-anaphora, english-v4-s7-counter-kind-specifiers, english-v4-s7-with-small-clauses, english-v4-s7-game-procedure-idioms, english-v4-s7-selection-idioms, english-v4-s7-preposed-clause-coordination, english-v4-s7-gapping-remnants, english-v4-s7-gapping-remnants-rest, english-v4-s7-long-tail-quick-wins, english-v4-s7-long-tail-quantifiers, english-v4-s7-long-tail-quotes-and-mentions, english-v4-s7-long-tail-pp-coordination, english-v4-s7-long-tail-nonfinite-clauses, english-v4-s7-long-tail-edge-cases, english-v4-s7-cleave-transparent-brackets, english-v4-s7-heavy-object-shift, english-v4-s7-unlike-cluster-coordination, english-v4-s7-nonparallel-object-coordination, english-v4-s7-cited-for-complement, english-v4-s7-supplement-between-vp-conjuncts, english-v4-s7-postpositive-sum-total]
---
# Stratum 7: Discourse sequencing, replacement effects, and the corpus long tail

Umbrella for **Stratum 7** in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6). The work lives in the slice tickets listed in `needs:`, each named `english-v4-s7-<slug>`; their scopes, grounding, witnesses and judgments are in those tickets, not here. This ticket carries no work of its own and closes when all of them have landed.

The Stratum gate holds when the last slice lands:

- 100% of non-deferred Vintage-legal Oracle corpus units across Strata 0-7 parse as **read** (0 `known-wrong-only`) and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s` and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- The full linguistic judgment suite passes: every judgment's positive assertions hold, and nothing ungrammatical is admitted. No judgment asserts rank (`docs/decisions/english-v4.md` §4.4, principle 4).
- The landing acceptance of §4.4, principle 5, holds for every landing: no face is lost, `cargo xtask english-v4 strata --check` passes, and no real card's forest explodes.
- Zero CR-cataloged words appear directly in the grammar data file, and zero policing flags exist.

Progress: `COMPLETED_STRATUM` is 7 (`english-v4-s7-raise-stratum`). Every Stratum-7 judgment is enforced, and the units still unread are deferred under the open slices in `deferred.txt`; each slice removes its deferrals as it lands.

Standard constraints apply.
