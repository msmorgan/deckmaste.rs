---
needs: [builtin-v2-flavor-word-generator-onsets]
---
Flavor-word onset-override inventory: pin every closure branch and align the
closure rule with the card-name template (generator-onsets landing review M1,
L1). `authored_onset_overrides` in `crates/xtask/src/english_v2/flavor_words.rs`
hard-fails on missing, stale, and duplicate entries, but only `missing` has a
test; deleting the stale or duplicate branch leaves every gate green. Add one
test per branch, modelled on the card-name inventory test beside it
(`english_v2.rs`, the `CARD_NAME_ONSET_OVERRIDES` closure test). The card-name
inventory enforces two-way equality per the rewrite ADR (a surface the recipe
can classify may not carry an override); the flavor-word inventory currently
permits a redundant override. Adopt two-way equality here too so both
inventories share one closure rule, and extend the ADR sentence's scope from
"card-name rows" to both authored override inventories — that extension is
pre-ruled (coordinator, 2026-09-04): the claimant appends the dated amendment
in the same landing; it is not a STOP. Zero grammar change;
generated stubs byte-unchanged. Standard constraints apply.
