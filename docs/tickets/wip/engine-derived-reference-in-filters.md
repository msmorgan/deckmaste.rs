---
needs: []
---
**A derived reference inside a predicate (`Ref(ControllerOf(…))`) trips the
frameless matcher's invariant assert.** Observed while writing
`core-regions-witness-fixtures`. Standard constraints apply.

## The gap

`crates/deckmaste_engine/src/target.rs`'s `matches_with_activation` handles
`Predicate::Ref(Reference::Reg(_))` and the `This`/`You` anchors, then ends in

```rust
Predicate::Ref(r) => { debug_assert!(false, "engine invariant violated: …"); false }
```

Every DERIVED reference lands there: `ControllerOf`, `OwnerOf`,
`AttachHostOf`, `Coalesce`, `Single`. The ADR keeps those nestable as pure
expressions evaluated at the read site (law 4), so a filter is a legitimate
read site — but the matcher holds only a watcher, never a resolving frame, and
in a test build the assert aborts rather than degrading.

Two witness fixtures reached it and were re-spelled around it:
`ControlledBy(Ref(ControllerOf(Target(0))))` became
`ControlledBy(Controls(Ref(Target(0))))`, and a history read's
`Damage(to: Ref(ControllerOf(It)))` became `Damage(to: Controls(Ref(It)))`.
Both alternatives are faithful, so nothing is blocked today — but the natural
spelling of "controlled by the same player as X" aborts the process, and the
relation-predicate detour is not discoverable.

## Acceptance

A filter naming a derived reference resolves it (threading the frame the way
`resolve_count` already does) or is refused at LOAD with a diagnostic naming
the card. A `debug_assert!(false)` reachable from authored card text is the
thing to remove.

## Landing record

**Localization: held.** Reproduced first, before any edit: a filter naming
`Ref(ControllerOf(Reg(0)))` panicked at `target.rs`'s
`debug_assert!(false, "engine invariant violated: …")`, exactly as the ticket
says. All five derived `Reference` shapes fell through to it; only the
`AttachHostOf(This)`-shaped special case above it was handled.

**What was fixed.** `target::matches_with_activation`'s catch-all
`Predicate::Ref(r)` arm now resolves the reference and matches iff it lands on
this candidate. `resolve_watcher_reference` — previously a two-arm helper for
`Predicate::Adjacent` — became `resolve_frameless_reference`, which walks the
derivation itself: `ControllerOf`, `OwnerOf`, `AttachHostOf`, `OpponentOf`,
`Coalesce` recurse onto their inner reference, whose base case is the register
read (activation product first, then the carrier fallbacks the bare `Reg` arm
already took). `Single` and `Source` are not register derivations — the first
needs `resolve::eval_selection_set`'s `Frame`, the second is the set-valued
deal-time damage binding read only inside `Matches(Source, …)` — so they read
"no match", the same never-crash fizzle `Predicate::Adjacent` takes.

**Why resolve rather than refuse at LOAD.** The acceptance offers either. Law 4
of `core-explicit-regions` makes derived references pure nestable expressions
"evaluated at the read site", introducing no binding — so a load refusal would
contradict the ADR, and the first disjunct is the only ADR-compatible one. No
STOP: the ticket's disjunction contains the ruling-compatible option, so there
is no contradiction to stop on.

**Why the derivation is walked here rather than delegated to
`resolve::eval_reference`.** That evaluator reads the DERIVED controller
through `state.layers()`, and this matcher is reachable from inside a layer
rebuild (`layer::condition_predicate_matches` delegates player proxies to it),
where re-entering `layers()` would recurse — the same hazard `layer.rs`'s own
`resolve_source_relative` seam documents. The stored controller read is the one
the sibling `Predicate::Relation(ControlledBy)` arm already uses, so the two
spellings of "controlled by the same player as X" agree by construction; a new
test asserts that agreement over every object in a fixture. Both share the
pre-existing narrowness that a control-changing effect (layer 2) is not seen.

**Rule citations needed.** `[CR#110.2]` (every permanent has a controller — the
derivation `ControllerOf` performs), `[CR#109.4]` (only stack/battlefield
objects have one, so a player proxy fizzles), `[CR#108.3]` (owner),
`[CR#301.5,303.4]` (attach host), `[CR#102.2,102.3]` (opponent, two-player and
multiplayer), `[CR#613.1b]` (the layer the unseen control change applies in).
All were already in `cr-citations.lock`; nothing was blessed, so the lock is
byte-identical before and after. Two citations were corrected during the
`cite audit --diff` read rather than shipped: `[CR#109.5]` (which defines what
"you" means, not what an object's controller is) was replaced by `[CR#110.2]`,
and an `[CR#611.2c]` copied from a neighbouring doc comment was dropped as
off-topic — it governs when a continuous effect's affected set is fixed, which
is not the claim it was attached to.

### Assurance counts

- **added: 8** — six unit tests in `target.rs`
  (`derived_controller_ref_in_filter_resolves_against_the_carrier`,
  `derived_controller_ref_agrees_with_the_relation_spelling`,
  `derived_owner_ref_in_filter_resolves`,
  `derived_ref_over_a_player_proxy_fizzles`,
  `coalesce_ref_in_filter_takes_the_first_resolvable`,
  `unresolvable_ref_in_filter_fizzles_instead_of_asserting`), plus a second
  case each inside the two witness fixtures the ticket names, which now run for
  BOTH spellings (`run_away_together_refuses_two_targets_under_one_controller`
  and `steel_hellkite_destroys_by_announced_x_and_who_it_damaged` each loop over
  the relation-predicate detour and the derived reference).
- **re-spelled: 0** — the two witness fixtures were NOT converted. Their
  relation-predicate spelling is still asserted; the derived spelling was added
  beside it, so no coverage was traded away.
- **removed: 0**
- **restored: 0**
- **ignored with blockers: 0 added** — the tree's existing `#[ignore]`s are
  untouched.
- **ignores closed: 0** — none of them was blocked on this.

### Deviations and additions

- `Predicate::Adjacent` now shares the widened resolver, so it additionally
  resolves a register that has a live activation product (previously only the
  intrinsic source/controller parameters answered, and everything else read
  `false`). Resolving more references correctly is the direction the ADR points;
  Death Spark, the only corpus user, reads `This` and is unaffected.
- The stale "AUDITED invariant (engine-candidate-frame-context) … no live call
  path reaches this arm" comment above the `Ref(Reg)` arm was replaced: it
  described a hazard that no longer exists and named a `debug_assert!` that is
  gone.
- Two `todo!`s in the same function that name `engine-frameless-carrier-threading`
  (a done ticket) were deliberately left alone — that ticket removed the callers
  that could reach them, so they are unreachable markers, not this ticket's
  target.
- The LKI sibling `trigger::filter_matches_snapshot_with_activation` still
  `todo!`s on a derived reference. That is the already-live, already-owned seam
  `engine-snapshot-predicate-breadth`, named in the panic message; no new
  routing was needed.
- No `deckmaste_core` change, so no `plugins/wizards` regeneration was required;
  no construction was added or deleted, and the coverage lock is unchanged.

### Gate artifacts

```
cargo test -p deckmaste_core -p deckmaste_lowering
  test result: ok. 53 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 738 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p deckmaste_engine
  test result: ok. 746 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
  test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
  test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 17 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
  test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p deckmaste_plugin
  test result: ok. 103 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo clippy -p deckmaste_core -p deckmaste_lowering -p deckmaste_engine \
  -p deckmaste_plugin --all-targets   →  clean, no warnings

cargo xtask cite check --list-noncompliant
  0 non-compliant citation-looking string(s)
cargo xtask cite check
  checked 17669 citations against cr.txt (eff. 2026-08-07); 0 stale

cargo xtask idris-check plugins/canon --differential
  plugins/canon: certifier/resolver differential — 68 agreed sound,
  0 agreed unsound, 12 skipped (no certifier verdict)
  differential OK: 0 disagreements
```

No STOP was taken. No ledger residue: the one adjacent gap found (the LKI
snapshot matcher's derived-reference `todo!`) is already owned by the live
`engine-snapshot-predicate-breadth` ticket.
