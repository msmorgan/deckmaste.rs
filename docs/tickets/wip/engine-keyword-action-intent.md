---
needs: []
---
Collapse the bespoke removal/draw/mill machinery into the ONE keyword-action
primitive the engine already has (`Action::Composite{name, body}`). Every CR-701
keyword action — Scry, Surveil, Destroy, Mill (and the shaped-later Discard) —
becomes a macro over that single primitive; the engine holds zero per-verb
`Action`/`PlayerAction` variants and zero per-verb intent events. The name rides
the emitted zone-change as its `cause`, and the deontic / replacement / trigger
layer keys on that cause — which the filter language already speaks
[CR#701.8a,121.1,701.17a,701.22b].

## The insight

Destroy/Draw/Mill are "named zone moves". The engine already models the guard
side of that entirely in terms of the CAUSE of a zone change, not bespoke intent
events:

- `EventFilter::ZoneChange { what, from, to, cause }` (`event.rs:264`) — the
  `cause` field narrows by cause triple: "'destroyed' admits exactly two causes
  ([CR#701.8b]); 'sacrificed' is never destruction ([CR#701.21a]); omitted = any
  cause ('dies', [CR#700.4])". There is **no** `WillDestroy`/`WillDraw` variant in
  `EventFilter` — guards already watch "a Bf→Gy change *caused by Destroy*".
- `EventFilter::Drawn { who, amount }` (`event.rs:307`) — draw is already a
  first-class filterable fact, per-card ([CR#121.2]).
- `ZoneWillChange` is itself the replaceable pre-commit intent (`step.rs`
  apply commits it) and already carries `cause`.

So the bespoke `GameEvent::WillDestroy` (`event.rs:154`) is a *redundant second
intent layer* above `ZoneWillChange` — it exists only to give indestructible /
regeneration a "destroy" hook, but `ZoneWillChange{cause: Destroy}` is that hook.
And `Action::Destroy` / `PlayerAction::Draw` / `PlayerAction::Mill` are per-verb
engine variants where Scry/Surveil are already just macros over `Composite` —
the exact special-casing the macro layer exists to eliminate
([[minimal-primitives-keyword-macros]], [[keyword-authoring]]).

## Design — one primitive, macros on top

### Sole keyword-action primitive + one named intent

`Action::Composite { name: Ident, body: OneShotEffect }` stays as the ONLY
keyword-action verb (`action.rs:234`). Resolving it:

1. emits a guardable/replaceable **`GameEvent::KeywordAction { name, patient }`**
   intent (the named-action moment — new; the "WillAct" layer). If a deontic
   `Cant` or a replacement bites it, the action is suppressed/replaced and the
   body never runs.
2. otherwise runs `body`, producing the RESULT events (a `ZoneWillChange` carrying
   `cause: name`, `CardsDrawn`, …).
3. then (gated on the body actually acting, [CR#701.22b]) emits the post-fact
   `KeywordActionPerformed{name}` for "whenever you scry/mill/draw" triggers —
   unchanged from today.

The verb thus rides TWO layers by design: the **name** on the pre-intent
(`KeywordAction`) is what guards/replacements bite; the **cause** on the result
zone-change is what triggers read. This mirrors the CR — indestructible acts on
the destroy *attempt* ([CR#702.12b]) while "dies" fires on the *resulting move*
([CR#700.4]).

### Destroy / Mill — pure named zone-change bodies

`destroy(x)` and `mill(n)` become macros expanding to `Composite`:

- `destroy(x)` → `Composite{ name:"Destroy", body: Move(x, Graveyard) }` where the
  emitted `ZoneWillChange` carries `cause: Destroy` [CR#701.8a,701.8b]. No
  `WillDestroy`.
- `mill(n)` → `Composite{ name:"Mill", body: MoveGroup(top-n, Graveyard) }`,
  `cause: Mill`, clamped to library size [CR#701.17a,701.17b]. Replaces the
  inline `PlayerAction::Mill` batch and gives Mill its first-class name (so
  "milled this way" reads `KeywordActionPerformed{"Mill"}` / the `Mill` cause,
  not only Noting product-groups) [CR#701.17c].

### Draw — same primitive, a richer body

Draw can't front-load a `ZoneWillChange{object: top-card}`: an empty library has
no object to name, yet empty→loss must still fire [CR#121.4,704.5b]. So the
`draw(n)` macro → `Composite{ name:"Draw", body: <draw-body> }` whose body keeps a
draw-intent: for each of `n` (one at a time, [CR#121.2]) check the library — a
card present → move top to hand (`cause: Draw`, bump `CardsDrawn`); empty →
`DrewFromEmpty`. This is the Draw keyword action's DEFINITION (as reorder is
Scry's), living in the body — NOT a bespoke `Action::Draw`. Preserve
face-down-during-cast [CR#121.8] and the "without the word draw" distinction
[CR#121.5] (carried by `cause: Draw` vs a plain move) exactly as today.

### Guards / replacements — point at the NAMED action

The deontic/replacement layer watches the `KeywordAction` pre-intent by name, not
the result zone-change. A new `EventFilter::KeywordAction { name, patient }`
variant lets `Cant`/replacements reference it.

- Indestructible [CR#702.12b]: `Cant(KeywordAction(Destroy, patient: This))` —
  reads exactly like the card ("can't be destroyed"). Naturally ignores the
  toughness-0 move ([CR#704.5f]) and sacrifice ([CR#701.21a]) — they are not the
  Destroy keyword action, so NO cause-predicate is needed. Must still cover BOTH
  the effect destroy AND the lethal-damage / deathtouch SBAs: those now perform
  the Destroy keyword action (emit `KeywordAction(Destroy)` with
  `Agency::StateBasedAction`) instead of the bespoke `WillDestroy` (`sba.rs:248`)
  — census "destroy immunity (SBA + Destroy)".
- Regeneration [CR#701.19a]: floating replacement watching
  `KeywordAction(Destroy)` (was watching `WillDestroy`, `replace_registry.rs:270`);
  "can't be regenerated" rider unchanged [CR#701.19c].
- Preserve cant→replace→apply order ([[engine-replacements]], `step.rs:1247`) so
  an indestructible creature never consumes its regen shield [CR#702.12b].
- Empty-draw loss SBA unchanged (reads `DrewFromEmpty`).

### Result-side triggers — unchanged, cause-keyed

"When it dies" watches any Bf→Gy ([CR#700.4]); "destroyed"/"milled"/"drawn"
narrow by `cause` / read `Drawn` — the existing `EventFilter::ZoneChange{cause}`
and `EventFilter::Drawn`, untouched.

## Blast radius

- `action.rs`: remove `Action::Destroy`, `PlayerAction::Draw`, `PlayerAction::Mill`.
- `event.rs`: add `GameEvent::KeywordAction{name, patient}` (pre-intent) +
  `EventFilter::KeywordAction{name, patient}`; remove `GameEvent::WillDestroy`.
  `step.rs`: the `Composite` resolve emits `KeywordAction` then the body's result
  events; keep the draw library-check/empty→loss logic as the Draw body's
  behavior (its status as a top-level `WillDraw` folds under the Draw composite).
- Macros (`deckmaste_cards/src/macros.rs`): add/point `destroy`/`draw`/`mill` at
  `Composite` (mill/scry pattern already exists from macro-keyword-actions).
- Parse⇄render⇄idris round-trip ([[parse-via-macros-design-settled]]): rewire
  `parsers/effect.rs`, `render/effect.rs:648,975,1446-1488`, `idris_emit.rs:1510`
  to the macro form; a new/changed grammar needs its RENDER arm
  ([[canon-card-needs-renderer-for-fidelity]]).
- `replace_registry.rs` / `sba.rs`: retarget indestructible + regen from
  `WillDestroy` to `KeywordAction(Destroy)`; the lethal / deathtouch SBAs perform
  the Destroy keyword action (emit `KeywordAction(Destroy)`,
  `Agency::StateBasedAction`) instead of `WillDestroy`.
- Regenerate wizards; run the cards suite once.

## Non-goals

- `Discard`/`WillDiscard` — shaped only, not built here (slots in as the next
  macro once a fixture forces it, exactly as it's shaped today).
- No card-authoring-surface change: `Destroy(Target(0))` / `Draw(...)` / `Mill(...)`
  still read identically ([[authored-surface-ergonomics-rulings]] governs the
  surface, not the engine enum).
- Do NOT fold `ZoneWillChange` away — it stays the committed result event. The
  new `KeywordAction` intent sits ABOVE it (the named-action moment), it does not
  replace it. One named intent for ALL keyword actions, not a per-verb event.

## Verification

TDD off existing coverage — behavior is identical, so these MUST stay green:
`action.rs` destroy/draw/mill/indestructible (`:805,:863,:951,:972,:1013,:1096,
:1122,:1186`); `sba.rs` lethal/deathtouch/indestructible/toughness-zero
(`:579,:596,:1393,:1693,:1725,:1818,:1843,:1860`); `replace_registry.rs:337`.
Add: a destroy and a scry proven to flow through the SAME `Composite` path; a
`ZoneChange{cause: Destroy}` canted by indestructible while a toughness-0 move is
not. `cargo xtask cite check` clean; `cite audit --diff` the touched CR cites.
Cards suite once (new emit shape → wizards regen).

Deps (all in `done/`): engine-replacements, engine-resolve-actions,
macro-keyword-actions, engine-cause-constructors, engine-trigger-events,
migrate-damage-sbas-to-rules.
