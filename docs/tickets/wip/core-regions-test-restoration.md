---
needs: []
---
**Restore the assurance the discourse stage deleted.** The stage-2 landing
(`core: complete discourse regions`, 2026-09-02) removed 163 `#[test]`
functions and added 1. Its suites are green because the tests that could
fail are gone. Standard constraints apply.

## What was deleted

132 recovered by name from the diff, spread over 25 files. The heaviest:
`engine/src/resolve/effect.rs` 22, `lowering/src/effect.rs` 19,
`engine/src/resolve/action.rs` 13, `engine/tests/replace_registry.rs` 12,
`engine/tests/payment.rs` 11, `engine/src/resolve/query.rs` 10,
`engine/tests/payment_replay.rs` 8. The files survive; individual tests
were removed from them.

Most are unrelated to the discourse channel this stage rewrote. Scry
(`scry_fact_records_after_the_arrange`,
`scry_reposition_keeps_id_and_fires_no_zone_change`,
`scry_zero_fires_no_event_but_nonzero_does`,
`scry_arrange_surfaces_only_for_multi_card_piles`,
`scry_two_both_top_round_trip`), emblems
(`get_emblem_emits_emblem_created_for_the_actor`), replacement
(`redirected_discard_still_records_the_name_fact`), deontic
(`cant_act_suppresses_composite_body`), coin flips and dice
(`uncalled_flip_emits_batch_and_fixes_that_many_to_heads`,
`dice_roll_emits_per_die_and_fixes_that_many_to_sum`,
`called_flip_surfaces_call_and_scores_won`,
`multi_coin_called_flip_pauses_per_coin`) and the whole payment-replay
family are behaviour this stage did not touch.

Two named casualties matter beyond their family. `unbound_reference_
degrades_to_null_not_panic` was the fixture pinning ADR law 10, the
fizzle-not-panic contract. `blink_exiles_and_returns_the_target_in_one_
resolution` was the exile-and-return fixture stage 2's own ticket
required, deleted together with its `inline_blink` helper.

## Scope

Recover the deleted set from the pre-landing tree (the claim commit
`kata: claim core-regions-discourse` is the parent) and triage each by
one rule:

- **Subject still exists** (scry, emblems, replacement, payment, deontic,
  coin/dice mechanics, fizzle-not-panic): restore the test. If it fails,
  the landing broke behaviour and the fix is in the code.
- **Subject was retired by design** (a `Binder`, `With`, `It`/`That`,
  `ThatMany` spelling): re-spell it against the region form, same card,
  same asserted outcome. A magnitude test whose subject is now a def is a
  re-spelling, not a deletion.
- **Genuinely unrecoverable**: `#[ignore]` naming the exact blocker, and
  list it in the landing record. This is the rare case, not the default.

Then add the seven fixtures stage 2's ticket named and did not deliver:
exile-and-return (target param, then a move-product def read by the
return); an insertion negative (a producing clause inserted between binder
and mention re-resolves at lowering, never silently rebinds); nested
`Where`-in-`Pick` reading both candidates; a multi-sentence anaphora card;
a `Let`-pinned "that many" not re-evaluated after the state changes; a
chooser nested under another chooser, `Random`, or `AmongNoted` resolving
its own picks; and a two-magnitude card plus an `Each`-over-players "that
much" reading the per-element amount.

`lowering/src/region.rs` carries the whole resolver in ~692 lines with no
tests of its own; it gets direct coverage here.

## Gates

Report tests restored, re-spelled, ignored, and added, with every ignored
one naming its blocker. Engine, core, lowering, and plugin suites green
with `plugins/wizards` regenerated and `crates/*/build.rs` touched, and
the `ignored` count read on every `test result:` line rather than `ok`
alone. A restored test that fails is a code fix, never a re-deletion.
