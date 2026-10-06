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
