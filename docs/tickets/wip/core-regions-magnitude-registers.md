---
needs: []
---
**A magnitude produced at runtime has no register, so nothing can read
it.** Routed from `core-regions-test-restoration`'s landing, 2026-09-02;
two restored tests are `#[ignore]`d on it. Standard constraints apply.

The discourse stage deleted the state-global magnitude register and its
occurrence-amount writer, and lowering pins a magnitude to a definition
only when the amount is statically known. So a count produced while
resolving reaches no destination: a coin-flip or die-roll tally in
`crates/deckmaste_engine/src/resolve/player_action.rs`, and a zone-change
batch's card count in `crates/deckmaste_engine/src/step/mod.rs`, which
also lost the entailment table's amount column.

Give a runtime-produced magnitude a definition the way a runtime-produced
object already gets one: the instruction that fixes the amount writes a
number register, and the later read is an ordinary register read. This is
the number half of the same move the discourse stage made for objects.

## Gates

Both ignored tests un-ignore and pass. A card that flips coins and then
reads the number of heads resolves correctly. Standard suites green.

## Landing record

- Construction count: runtime-magnitude-producing action families 0 → 3
  (`FlipCoins`, `RollDice`, multi-card `Discard`); `WorkItem` +1
  (`WriteMagnitude`), `Progress` +1 (`MagnitudeWritten`), and no `Action`
  variant added.
- Coverage and lock state: both blocker tests restored; lowered card-shaped
  coin and discard read-through witnesses added; `cr-citations.lock`
  unchanged.
- Assurance: restored 2
  (`flip_and_roll_batches_fix_the_magnitude_anaphor`,
  `discard_batch_fixes_the_magnitude_anaphor_to_its_card_count`); re-spelled
  0; ignored blockers 0; added 4
  (`a_coin_tally_defines_the_register_that_many_reads`,
  `dice_and_discard_tallies_lower_as_number_producers`,
  `a_lowered_coin_card_reads_its_number_of_heads`,
  `a_lowered_discard_card_reads_cards_actually_discarded`); removed 0.
- Positive artifacts: core/lowering/engine/plugin test suites green; clippy
  green for all four crates and all targets with warnings denied; runtime
  witnesses cover uncalled heads, called wins, summed dice, and committed
  discard zone changes. The post-refresh `cargo test --workspace` reaches an
  unrelated `deckmaste_construction` trybuild failure: four expected-stderr
  snapshots now receive pre-existing `parser-metrics` cfg warnings; the same
  `cargo test -p deckmaste_construction --test compile_fail` failure reproduces
  unchanged on `default`.
- Deviations and additions: core discard-body inspection now recognizes the
  normalized direct `Each(Random(...))` shape as well as the authored
  sequential shape, because lowering removes a singleton sequence before
  runtime classification.
- STOPs: none.
