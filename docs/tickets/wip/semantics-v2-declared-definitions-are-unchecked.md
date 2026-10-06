---
needs: []
---
**No gate checks a Registry Definition, and the checker refuses the +1/+1
counter's conferral for demanding the battlefield.** Two findings routed by
the landing record of `semantics-v2-counter-kind-is-a-name` (2026-10-05),
decided by the owner the same day. Standard constraints apply. Related:
`semantics-v2-counter-kind-is-a-name` (the landing that found both),
`semantics-v2-designation-is-a-name` (the designation definitions the gate
also covers).

## The finding

`cargo xtask lean-check` emits and proves CARDS only. Nothing emits the
`Definition` node a registry declaration in `plugins_v2/builtin/macros/`
carries, so the Lean `Definition.check` (`lean/Semantics/Check/Rules.lean`)
runs only over the hand-written pins in `lean/Semantics/Proofs/Rules.lean`.

Evidence: the +1/+1 and -1/-1 counter definitions are refused today.
`Proofs/Rules.lean` pins `p1p1CounterDefinitionZone` and
`m1m1CounterDefinitionZone` assert `Definition.check … = [.zoneIs
.battlefield]`: a stat change's subject must be a permanent
(`StaticSpec.check`), and the definition's subject is the counter's bearer.
The retired count-multiplied spelling was refused the same way, twice
(`p1p1CounterCountMultipliedTwin`), so the refusal predates the landing.

## (a) Decided: a definition's conferral is about its bearer wherever it is

A definition's conferral is about its bearer wherever the bearer is, and the
checker stops demanding the battlefield for conferrals inside a definition.
The owner, 2026-10-05: "since when can +1/+1 counters be put on
non-permanents? and since when would that do anything? This might be one of
those things that lean is being overzealous about; if an invalid instruction
is asked, nothing happens as per the CR", and, after the rule was quoted,
"counters can go on exiled cards too. and players."

The basis, read from `data/rules/cr.txt` (rule numbers inside quotations are
written in citation form):

- A +X/+Y counter works off the battlefield too: [CR#122.1a] "A +X/+Y counter
  on a creature or on a creature card in a zone other than the battlefield,
  where X and Y are numbers, adds X to that object’s power and Y to that
  object’s toughness. Similarly, -X/-Y counters subtract from power and
  toughness. See [CR#613.4c]."
- Putting counters is a battlefield matter: [CR#122.6] "Some spells and
  abilities refer to counters being put on an object. This refers to putting
  counters on that object while it’s on the battlefield and also to an object
  that’s given counters as it enters the battlefield." An instruction that
  cannot be carried out does only what it can [CR#609.3]: "If an effect
  attempts to do something impossible, it does only as much as possible." So
  the instruction is where the zone matters, not the definition.
- Counters sit on exiled cards: suspend exiles a card "with N time counters
  on it" [CR#702.62a], and a suspended card is one "in the exile zone, has
  suspend, and has a time counter on it" [CR#702.62b].
- Counters sit on players: a counter is "a marker placed on an object or
  player" [CR#122.1]; poison counters [CR#122.1f]; an energy counter is one a
  player removes "from themselves" to pay {E} [CR#107.14].

The work: change the definition check (`Definition.check`, and
`StaticSpec.check` where it is reached from a definition) so a conferral
inside a definition is checked without the battlefield demand. The pins
`p1p1CounterDefinitionZone` and `m1m1CounterDefinitionZone` are re-spelled to
assert `= []` (same definitions, the decided outcome); stat changes outside
definitions keep the demand and their pins.

## (b) Decided: build the gate over every Registry Definition

Owner: "sure". After (a) makes the +1/+1 definition pass, emit every Registry
Definition as Lean and prove `Definition.check = []` for each, beside the card
gate. Coverage, counted from `cargo xtask expansions` on the counter landing's
tree: 554 definitions — 73 counters, 462 subtypes, 19 designations (in 18
declaration files). Its first run must count and name every definition it
refuses; none is waved through or left out unnamed.

## (c) Closed: per-counter conferrals are right for shield, stun and finality

The ruling of 2026-10-05 (`docs/decisions/semantics-v2.md` §7) says a counter
definition states what ONE counter confers and the model applies it once per
counter held. Three counter rules say one or more counters create a single
effect [CR#122.1c,122.1d,122.1h]. That is not a conflict, and nothing needs a
second reading of `confers` or a flag: each of the three confers a
replacement (or prevention) effect, and N identical such effects behave as
one.

The owner: "they confer a replacement effect, and iirc with the way
replacement effect stacking works this functions correctly for free. check me
on this." Checked against `data/rules/cr.txt`; it holds:

> [CR#616.1] If two or more replacement and/or prevention effects are
> attempting to modify the way an event affects an object or player, the
> affected object’s controller (or its owner if it has no controller) or the
> affected player chooses one to apply, following the steps listed below. If
> two or more players have to make these choices at the same time, choices
> are made in APNAP order (see [CR#101.4]).

> [CR#614.5] A replacement effect doesn’t invoke itself repeatedly; it gets
> only one opportunity to affect an event or any modified events that may
> replace that event.

With N shield counters there are N identical effects; the controller applies
one [CR#616.1], it replaces the event, and that effect gets no second
opportunity [CR#614.5]. The process then repeats over only the effects that
"would now be applicable" [CR#616.1f], and the others are not: the modified
event (remove a shield counter, prevent the damage, exile instead, stay
tapped) is no longer the destruction, damage, graveyard move or untap they
replace. So one counter is removed per event, as [CR#122.1c,122.1d] print it,
and a permanent with finality counters is exiled once [CR#122.1h].

`shieldCounter`, `stunCounter` and `finalityCounter` confer nothing today
(`confers` empty); when they gain their conferral, it is written per counter
like every other kind.

## Landing record

The series, oldest first: `kqxvmnkz` (the zone rule and its pins),
`wlkwrwok` (the `definition-check` gate, its emitter, reader and tests),
`zwrqlwyl` (docs), and this record. Each stage was gated on its own tree. No
English grammar, lexicon, corpus input or `plugins_v2/` data changed, so there
is no coverage figure to stamp: the lock and the corpus are untouched.

**(a) The rule.** `Conferral.checkProperty` (`lean/Semantics/Check/Rules.lean`),
reached from `Conferral.check`'s `.property` arm, so it applies to every
conferral a `Definition` holds (counters' `confers`, subtypes' `rules`) and to
nothing else:

```lean
def Conferral.checkProperty : StaticSpec → List Refusal
  | .ptModification .this p t =>
      NounPhrase.check (some .object) [] .this ++ Amount.check (selfSubjIntro [] .this) p.amount ++
        Amount.check (Delta.introduced (selfSubjIntro [] .this) p ++ selfSubjIntro [] .this) t.amount
  | spec => StaticSpec.check [] spec
```

That is `StaticSpec.check`'s `ptModification` arm with the one
`zoneIsCheck … .battlefield` line left out, for the bearer (`this`) only.
`StaticSpec.check` and everything else a card is checked by are untouched
(`Check/AbilityRules.lean` has no diff), so a card's "gets +1/+1" still
demands the battlefield. The narrowing is the smallest that passes the two
counters: the bearer only, and only the two-delta `ptModification` that
`boost` writes. A definition's single-stat `modification` on its bearer, and
a stat change on any other subject, keep the demand; that is why
`p1p1CounterCountMultipliedTwin` keeps its two refusals (see Deviations and
Open questions). Basis as the ticket quotes it:
[CR#122.1a,122.6,609.3,702.62a,702.62b,122.1].

**(b) The gate.** `cargo xtask definition-check [PLUGIN_DIR]` (default
`plugins_v2/builtin`; `crates/xtask/src/definition_check.rs`). It reads every
declaration of the three registry kinds through `facts`' readers (a new
`registry_definitions` in `crates/xtask/src/facts/lean.rs`: counters and
subtypes through the plugin's macros so the conferral helpers expand,
designations through `designations::designation_definitions`, one per listed
member), emits `lean/GeneratedDefinitions/{Counters,Subtypes,Designations}.lean`
under a `GeneratedDefinitions.lean` root (a `GeneratedDefinitions` lake
library beside `Generated`, outside `defaultTargets`, both paths gitignored),
and builds each definition as `def`, guarded `#eval Definition.check`, and
`theorem … = [] := by decide`. Build and attribution are `lean-check`'s own
(`build` now takes its lake target; `attribute`, `EmittedModule`,
`PluginReport` are crate-visible), so a refused definition is named with its
refusal list and a lake failure that names none is a gate defect. Each run
clears its own generated sources and their `.lake/build` outputs first. The
emitter gained `render_definitions_module` and `definition_ident`
(`crates/deckmaste_semantics_v2/src/lean_emit.rs`), one `render_items` body
shared with `render_module`; the card module's text is byte-identical.

Names are the declaration's name for counters (`p1p1Counter`),
`<category>/<name>` for subtypes (`creature/elf`), and the label for
designations, `<declaration>/<label>` for members (`dayNight/day`).

**A kind the checker has no rule for** cannot arise silently, and the gate
has no skip list: Lean's `Definition.check` is an exhaustive match (a new
constructor without an arm does not build); the Rust side sorts each node
by an exhaustive match on `Definition` (`Family::of`), so a new constructor
does not compile until it is given a family; and a declaration of a registry
kind whose body does not read as its family's constructor fails the read,
naming it. A plugin with no registry definition at all fails the run.

Name alternatives for the owner: `definition-check` (chosen, after
`lean-check`), `registry-check`, `lean-definitions`, or a
`lean-check --definitions` flag.

**First run, and the refused list.** On `wlkwrwok` (the zone rule in place):
554 definitions, 73 counters 73/73, 462 subtypes 462/462, 19 designations
19/19; **refused: none**. The same command run once against the stage-1 tree
with only `Conferral.check`'s `.property` arm put back to `StaticSpec.check []
spec` (the checker as it was; restored at once, never committed) refused
exactly two, both for the zone rule this ticket decides: `p1p1Counter`
`[zoneIs battlefield]` and `m1m1Counter` `[zoneIs battlefield]`; 71/73
counters, 462/462 subtypes, 19/19 designations. No other definition is
refused, so no follow-up ticket was minted.

**Pins.** Re-spelled to the decided outcome (the one outcome change the
ticket authorises): `Rules.p1p1CounterDefinitionZone` and
`Rules.m1m1CounterDefinitionZone`, `[.zoneIs .battlefield]` → `[]`.
Re-spelled, outcome unchanged: `Rules.p1p1CounterCountMultipliedTwin`, from
`= Definition.check p1p1CounterDefinition ++ [.zoneIs .battlefield]` (which
evaluated to two `zoneIs` refusals and would now read one) to the literal
`[.zoneIs .battlefield, .zoneIs .battlefield]`; doc comment updated. Added:
`Rules.cardStatChangeOffBattlefieldZone`, `StaticSpec.check []
(.ptModification .this (.up (.lit 1)) (.up (.lit 1))) = [.zoneIs
.battlefield]`, the narrowing proven to be the definition's alone. No other
pin's outcome changed; none removed.

**Tests.** Added 4, removed 0, ignored 0: in `crates/xtask/tests/lean_check.rs`
(the target that runs `lean-check`, sharing its lock)
`every_builtin_registry_definition_proves`,
`a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`
(a copy of the whole builtin registry plus a counter conferring a spell
ability; the gate reports 73/74 and `REFUSED lawlessCounter:
[Semantics.Refusal.grantable]`), and
`a_lake_that_fails_without_naming_a_definition_stops_the_definition_gate`;
in `lean_emit` `a_definitions_module_carries_a_def_and_a_decide_theorem_per_definition`.
Existing tests untouched except the module doc of `tests/lean_check.rs`.

**Docs.** `lean/CONTRACTS.md` (Rules tables: the stale "refuses the +1/+1 and
-1/-1 definitions, which no gate runs" replaced by the rule and the gate);
`lean/README.md` (a "registry definition gate" section beside the card gate);
`docs/guided_tour.md` and ADR §13 (one sentence each naming the command
beside `lean-check`). ADR §7: no sentence became false, unchanged. `CLAUDE.md`
unchanged: it names no Lean gate (`lean-check` is named as a gate in
`lean/README.md`, `docs/guided_tour.md`, `docs/keyword-policy.md` and the
workbench ADR), so the sentence went beside it there. Glossary: no gap; the
Registry Definition entry already reads right.

**Deviations and additions.** (1) The narrowing covers `ptModification` on
the bearer only; the ticket's wording ("a conferral inside a definition is
checked without the battlefield demand") is wider. Narrowing `modification
this` too would have changed `p1p1CounterCountMultipliedTwin`'s outcome (two
refusals → none), which the brief makes a STOP, and no registry definition
needs it (the gate refuses nothing). (2) `cardStatChangeOffBattlefieldZone`,
the narrowing pin. (3) A separate `GeneratedDefinitions` lake library rather
than a module inside `Generated`, because `lean-check` deletes `Generated/`
wholesale on every run. (4) `lean_check.rs` internals made crate-visible, no
behaviour change. (5) The ADR §13 and guided-tour sentences.

**STOPs.** None. The zone narrowing was stated without touching how cards are
checked, and the gate's first run refused nothing beyond the zone rule.

**Open questions for the owner.** Whether a definition's single-stat
`modification` (and `ptSwitch`) on its bearer should be narrowed the same way;
nothing declares one today. And the gate's name.

**Proof.**
- `lean/scripts/build` on every stage: Build completed successfully (82 jobs).
- `cargo xtask lean-check` on every stage: canon 127/127, testing 6/6.
- `cargo xtask definition-check` on `wlkwrwok` and `zwrqlwyl`: 554
  definitions (73 counters, 462 subtypes, 19 designations), all proved,
  none refused (about 26s).
- `cargo xtask facts check`: up to date on every stage.
- Gate: `cargo xtask gate --changed` derived nothing on `kqxvmnkz` (Lean
  only: "No workspace crates are affected"; the Lean build, `lean-check` and
  `cargo test -p xtask --test lean_check` stood in); `cargo test -p
  deckmaste_semantics_v2 -p xtask` on `wlkwrwok`, run with `--no-fail-fast`:
  464 passed, 0 failed, 1 ignored (pre-existing; `lean_check` 7/7);
  `cargo test -p deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask` on `zwrqlwyl` (docs are read by the declaration readers), with
  `--no-fail-fast`: 1224 passed, 0 failed, 1 ignored.
- `cargo clippy -p xtask -p deckmaste_semantics_v2 --all-targets`: clean.
- `cargo xtask cite check`: 0 stale; `--list-noncompliant`: 0; nothing
  blessed (every rule cited was already in the lock); `cite audit --diff`
  read on each stage.

**After `kata refresh`** (onto `uwktopko`'s line): came in
`plugins-v2-helper-parameter-order-residue` (`wrtqnlum` and its record:
fifteen builtin helpers take required parameters first, callers positional;
no registry declaration touched) and the `english-v3-census-tractability`
claim. No conflicts (`trunk()..@ & conflicts()` empty). On the refreshed tree:
`lean/scripts/build` 82 jobs, success; `lean-check` canon 127/127, testing
6/6; `definition-check` 554 (73/73 counters, 462/462 subtypes, 19/19
designations), none refused; `facts check` up to date; `gate --changed`
derived `cargo test -p deckmaste_semantics_v2 -p xtask`, and the wider
six-crate command of the docs stage (a superset) ran with `--no-fail-fast`:
1224 passed, 0 failed, 1 ignored; cite checks 0 stale, 0 noncompliant;
`kanban check` OK.
