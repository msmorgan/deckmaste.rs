---
needs: []
---
# kw-evolve — Evolve [CR#702.100a] (+ Bloodthirst [CR#702.54a] sibling) keyword macros

Author two counter-related keyword macros under `plugins/builtin/macros/keyword/`.

## Evolve [CR#702.100a]

"Whenever a creature you control enters, if that creature's power is greater
than this creature's power and/or that creature's toughness is greater than
this creature's toughness, put a +1/+1 counter on this creature."

Pieces (all already representable — no core addition needed):
- ETB trigger binding the entering creature as `ThatObject`:
  `Enters(AllOf([Creature, Not(Ref(This)), ControlledBy(Ref(You))]))`.
- Intervening-if [CR#603.4] cross-object stat comparison: `Condition::Compare`
  takes two full `Count`s, so `OneOf([Compare(StatOf(ThatObject, Power),
  Greater, StatOf(This, Power)), Compare(StatOf(ThatObject, Toughness),
  Greater, StatOf(This, Toughness))])`.
- `PutCounters(This, P1P1Counter, 1)`.

## Bloodthirst N [CR#702.54a]

"If an opponent was dealt damage this turn, this permanent enters with N +1/+1
counters on it." Conditional enters-with-counters:
`AsEnters(If(condition: Happened(event: Performed(verb: "DealDamage", on:
OpponentOf(Ref(You))), within: ThisTurn), then: PutCounters(This, P1P1Counter,
Param(0))))`. The "an opponent was dealt damage this turn" history condition is
already engine-executable (`Happened` scans history reusing the trigger
event-matcher; `DealDamage` matches `GameEvent::DamageDealt` with `on` = the
live recipient; `Window::ThisTurn` is a history-lookback window).

Bloodthirst X [CR#702.54b] (X = total damage to opponents) is a separate
special form, not authored here.

## Verify

Builtin `Plugin::load` + Evolve/Bloodthirst rows in
`every_builtin_keyword_macro_expands`. No core grammar additions — both gaps
the task flagged (cross-object stat compare; opponent-damage-this-turn history)
are already representable and engine-executable.
