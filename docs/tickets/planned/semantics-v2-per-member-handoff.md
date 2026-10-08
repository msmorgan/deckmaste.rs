---
needs: [lean-drt-anaphora-refactor]
---
**A handoff that distributes an instruction over the members of a bound
group, each member's own performer acting.** Routed 2026-10-08 from
`plugins-v2-airbend-reads-its-own-stamp` (its STOP). Standard constraints
apply.

[CR#701.65a]: "For each card exiled this way, for as long as it remains
exiled, its owner may cast it by paying {2} rather than paying its mana cost."
Each exiled card's own owner gets a permission for that card.

## What the model has today

- `each` (`plugins_v2/builtin/macros/determiners/each.ron`) takes a
  Predicate, so it cannot range over a group a body already bound.
- `eachOf` (`nouns/eachOf.ron`, Lean `NounPhrase.eachOf`) refuses a group
  whose number is one (`refuse (g.plur == .many) .plural`), so a keyword
  body whose parameter may be one or several cannot use it.
- `ownerOf(<group>)` is one `PossessorOf` performer for the whole group:
  `act(ownerOf(Param(0)), …)` hands the body to "their owner" once.

## Candidate shapes (not chosen)

- A `forEach(members, act(ownerOf(it), …))`-style instruction: iterate the
  bound group, with a member pronoun inside. The name `forEach` is already an
  amount macro (`amounts/forEach.ron`), so it needs its own name.
- `each` (or `eachOf`) accepting a bound group of either number, so
  `act(ownerOf(each(<group>)), …)` distributes.
- Nearest existing device: the Lean group-handoff `.many` profile from the
  actor-handoff landing (`semantics-v2-actor-handoff`), where a distributive
  performer ("each opponent") has each member perform the whole body.

## Pins

- Two exiled cards with different owners: each owner gets the permission for
  their own card only.
- One exiled card: the permission is unchanged from today's.

## Until it lands

Airbend's body (`keyword_actions/airbend.ron`) is the singular reminder's,
"Exile it. While it's exiled, its owner may cast it for {2} rather than its
mana cost.": since the 2026-10-08 ruling its later mentions are `it`, so it is
exact for one exiled object and refused for several (`it` agrees with one;
pin `airbendItRefusesSeveral`). The per-card loop is the plural reminder's,
"Exile them. While each one is exiled, its owner may cast it …", and serves
both numbers once it checks.

## STOP (2026-10-08): the singular case waits on anaphora resolution

The shape needs no new node. The per-member handoff is the existing loop
`doForEach` (Lean `Instruction.doForEach`) over the cards exiled this way,
with the handoff inside it. Since the 2026-10-08 ruling (§7, "A moved
referent is read back by anaphora") the loop's group is the exiled cards, not
the parameter: the exile makes them new objects [CR#400.7,400.7j], so nothing
after it mentions `Param(0)`. The group is the description "each card exiled
this way", `allOf(and([card, happenedTo(verbedEvent(exile, relative(Object)),
ThisWay)]))`, which is the rule's own wording [CR#701.65a]:
`doForEach(allOf(and([card, happenedTo(verbedEvent(exile, relative(Object)),
ThisWay)])), act(ownerOf(it), establish(…it…)))`. A description has no fixed
number, so neither the `.plural` refusal that stops `eachOf` nor the number
agreement that stops `theVerbed(exile, Many, card)` ("the exiled cards") for
one object arises: measured, that group is refused `[anaphor (verbed (action
"exile")) many 0]` for "Airbend target creature." whatever the resolution
rule, since a stamped pronoun fixes its number.
(The 2026-10-08 first draft looped over `allFromAmong(card, Param(0))`,
connive's device in `instructions/discardThenForEachNonland.ron`; the ruling
retires that read of a moved parameter.)

It checks for several objects (pin 1). For one object it is refused
`[anaphor bare one 2]` three times: every "it" in the loop reaches both the
loop's card and the exiled object, which are the same object. The strict rule
the checker keeps until `lean-drt-anaphora-refactor` lands refuses that tie
(§7, ruling 2026-10-07); `that(card)` ties the same way. Resolving it is a
reference-resolution change the ruling assigns to that ticket, so this
landing did not make it. Landing the RON body now would refuse the Airbend
Probe ("Airbend target creature."). Recommended routing: `needs:
[lean-drt-anaphora-refactor]`; that ticket now carries the pin to flip.

## Landing record (draft, STOP open)

Measured on change `zkltxqtw` over the claim `xmwpslwu`.

1. **Census delta**: none; no English grammar, lexicon or card text changed,
   so no census was run. Lost faces: none.
2. **Witnesses**: pin 1, two exiled cards with different owners:
   `okAirbendByCardUpToTwo` checks `[]` (each card's permission handed to
   that card's owner). Pin 2, one exiled card:
   `airbendByCardOneTiesUnderStrictRule` records the three-way tie (STOP
   above); the shipped airbend body is unchanged, so its permission is too.
3. **Spot checks**: none; no face is newly covered.
4. **Deviations and STOPs**: the STOP above. Added two Lean pins in
   `lean/Semantics/Proofs/BindingIdentity.lean`; airbend.ron changed in its
   STOP comment only (its expansion is unchanged); a routing note added to
   `lean-drt-anaphora-refactor`. No glossary gap.
5. **Tests** (on `zpuwumkt`, after a no-op `kata refresh`): `cargo test -p
   deckmaste_semantics_v2 -p xtask`: all pass except
   `lean_check::a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`,
   which fails identically on the claim `xmwpslwu` without this work
   (pre-existing: the lawless counter's definition now proves, so
   `expect_err` panics at `crates/xtask/tests/suite/lean_check.rs:201`).
   `cargo clippy -p deckmaste_semantics_v2 -p xtask --tests -- -D warnings`:
   clean. `lake build --wfail`: 88 jobs, clean. `cargo xtask lean check`:
   canon 138/138, testing 8/8. `cargo xtask lean definitions`: 553/553.
   `cite check --list-noncompliant`: 0; `cite check`: 0 stale; `cite audit
   --diff`: 3 sites, each [CR#701.65a], read against the rule. Restored 0,
   re-spelled 0, ignored 0, added 2 (Lean pins), removed 0.
6. **Timing**: no census was run.
