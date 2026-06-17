---
needs: []
---
Data types for data-driven play strategies, expressed in the card RON language.
A strategy is RON *data*, not a macro (macros have no control flow): its
"sensing" half reuses the existing `Condition`/`Filter`/`Count`/`Reference`
vocabulary verbatim, and its branching is an ordered rule list the evaluator
walks — the same way the engine already walks `Vec<Ability>` / orders triggers.

Define and register as macroable kinds, alongside cards in
`crates/deckmaste_cards/src/macros.rs`:

- `Strategy { name: String, rules: Vec<Rule> }`
- `Rule { when: Condition, prefer: Preference }` — ordered; first applicable +
  legal wins. `when` reuses core `Condition` (default = an `Always` macro).
- `Preference` — the one genuinely new node family (the "choose a play" half;
  cards never choose their controller's plays, so it has no card analog). Its
  arguments reuse Filter/Count/Reference:
  `Pass | Concede | Play(Selector) | Cast(Selector, target: TargetPolicy?) |
   Activate(Selector, target: TargetPolicy?) | Attack(Selector) |
   Block(BlockPolicy) | Discard(Selector)`
- `Selector { pick: Extremum, by: Count, among: Filter? }` — the workhorse:
  argmin/argmax of a `Count` over the legal set; `among` narrows it (default =
  all legal). `Extremum = Min | Max | First`. `TargetPolicy = Selector`.
- `BlockPolicy` — coarse enum for v1 (block-all / no-blocks / chump-biggest);
  grows later.

Scope: pure data types + parse/render round-trip tests + macro registration. No
evaluator here (that is `strategy-evaluator-core`). Foundation ticket for the
strategy-engine epic (v1).

## Status: DONE

`crates/deckmaste_cards/src/strategy.rs` (10 TDD tests; full crate suite,
clippy, and `cargo build --workspace` all green):

- All six data types — `Extremum`, `Selector`, `BlockPolicy`, `Preference`,
  `Rule`, `Strategy` — parse from RON and round-trip (serialize → parse =
  identity). `Selector` embeds core `Count`/`Filter` verbatim.
- **Sensing-position macro reuse verified end-to-end**: a `Condition` macro at
  `when:` expands through `MacroSet::read_str::<Strategy>` despite sitting deep
  inside plain-serde `Strategy`/`Rule`. So `Condition`/`Count`/`Filter` macros
  work for free at `when:`/`by:`/`among:` — no extra machinery on the new
  structs. Enables strategy-guide vocabulary (`Always`, `BehindOnBoard`,
  deck-specific predicates) as macros.
- **`Preference` is a registered macroable kind**: `#[derive(SupportsMacros)]`
  + `Expanded(Expansion<Preference>)`, registered in `macros.rs` `kinds()`. So
  the choose-a-play vocabulary (`prefer: AttackAll`, `Mulligan`, …) can be
  authored as macros that expand to literal preference variants. The
  `SupportsMacros` derive forwards the `#[serde(default, skip_serializing_if)]`
  attrs on the `Cast`/`Activate` struct-variant fields onto its generated
  helper structs, so the round-trip stays correct. `Extremum`/`Selector`/
  `BlockPolicy` gained `#[derive(Expand)]` (required by `Preference`'s
  generated `Expand`); `Rule`/`Strategy` deliberately did not (nothing expands
  into them).
