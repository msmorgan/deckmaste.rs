---
needs: []
---
Unify every CR-701 keyword action — Scry, Surveil, Destroy, Mill, Draw (Discard
later) — behind ONE parameterized, closed, idris-sourced atom family wrapped by a
single present-tense `Act` event. The engine holds zero bespoke per-verb
`Action`/`PlayerAction` variants and zero per-verb intent events; keyword-action
NAMES are bareword closed atoms (not quoted, not a hand-maintained core enum);
the deontic/replacement/trigger/provenance layers all key on the one `Act` atom.
[CR#701.8a,121.1,701.17a,701.22b]

## The insight (evidence)

Keyword-action names are a CLOSED grammar vocabulary, not open data, and the
engine already models the guard side by the CAUSE of a zone change, not bespoke
intent events:

- **Closed atom namespace.** Idris models these as closed sum types —
  `data KeywordActionSpec = Scry | Surveil | Mill | Fight` (`idris/src/Core.idr:2177`),
  `data KeywordSpec = Flying | … | Indestructible | Ward Cost | Protection Quality`
  (parameterized on the ability side) — and the re-emit bridge REJECTS any name
  outside them (`idris_emit.rs:1571` gap-errors on an unknown `Composite` name).
  Contrast the explicitly-OPEN `CounterKind` (`Core.idr:245`). The same name
  namespace is ALREADY bareword on its reference side (`KeywordRef(Ident)` reads
  `Has(Flying)` bare, `keyword.rs:50`); `Composite(name: "Ward")` quoting is an
  inconsistency wart.
- **Guards already key on cause, not intent-type.** `EventFilter::ZoneChange`
  carries `cause` (`event.rs:264`); the closed verb vocab is `entailments.ron`
  (9 verbs, emitted from idris `EmitTables.idr`). `ZoneWillChange` is itself the
  replaceable pre-commit form and carries `cause`. So the bespoke
  `GameEvent::WillDestroy` is a redundant second intent layer.
- `Action::Destroy(Reference)`, `PlayerAction::Draw/Mill` are bespoke per-verb
  variants where Scry/Surveil are already macro-over-`Composite` — the
  special-casing the macro layer exists to kill ([[minimal-primitives-keyword-macros]]).

## Design — one parameterized atom, one `Act` event

### The keyword-action atom (closed, parameterized, bareword, idris-sourced)

Parameterize the idris `KeywordActionSpec` to carry each verb's principal
argument, consistent with the already-parameterized `KeywordSpec` (`Ward Cost`,
`Protection Quality`):

```
KeywordAction = Destroy Reference | Draw Reference        -- single-object atoms
              | Mill Ref Count | Scry Ref Count | Surveil Ref Count  -- who + count
              | Fight Reference Reference | …
  -- Draw is SINGLE (no count): DrawN = Repeat(n, Draw(who)) [CR#121.2].
  -- Mill/Scry keep a count (batch / look-N-at-once), NOT Repeat.
```

Authored positionally as a **bareword** atom (one shared bare-ident newtype for
the whole keyword-action-name namespace — `Composite`/`Act`/`Cause`/`KeywordDecl`
names — mirroring `KeywordRef`/`CostTag`; fixes the `"Ward"` quoting wart). The
atom subsumes the old bespoke `Action::Destroy(Reference)` — that IS the
`Destroy(Reference)` arm, not a variant to keep. Typo-safety: the re-emit gate
rejects unknown atoms; add an `entailments.ron`-membership check. Retire the
`CauseVerb` enum into this namespace.

### The `Act` composite — ONE dual-facet event, applied atomically

A keyword action is `Composite(tag, body)` (the atom's principal arg lives in
the tag: `Destroy(Ref(This))`, `Scry(You, 2)`). It surfaces as ONE present-tense
`GameEvent::Act` that is **simultaneously matchable on two facets** and goes
through cant→replace→apply **as one unit** — NO unwrap step, NO second event
below it. The asymmetry the old two-step model created (destroy's realization
sat *below* its `Act` as a separate `ZoneWillChange`; scry's sat *above* it) is
gone: every verb is the same dual-facet composite.

- **Tag facet.** The keyword-action NAME + its participant. `Act(Destroy(_))`,
  `Act(Scry(You))`, `Cant(Act(Destroy(Ref(This))))` (indestructible), regen,
  "whenever you scry/mill/destroy" all bite here.
- **Body facet.** The composite's *canonical* realized shape, derivable from the
  tag **before** the body runs (`Destroy(x)`→`ZoneChange(Battlefield→Graveyard,
  x)`; `Mill(who,n)`→top-`n` `→Graveyard`; `Scry`→within-library reorder, no
  →Graveyard). "When it dies" (`ZoneChange(_→Graveyard)`) and Rest-in-Peace
  ("if a card would go to a graveyard, exile instead") bite here — on the SAME
  fact.

`FactView::of(Act{…})` populates **both** the tag fields (`act_name`, `actor`,
`object`) **and** the body-shape zone fields, so the one shared evaluator
(`replacement_watches` → `FactView` → `eval`) matches whichever facet a filter
names. The `eval` arms key on **facet present**, not strict `kind ==`:
`EventFilter::Act(pat)` checks `act_name`; `EventFilter::ZoneChange{…}` checks
the zone-shape — and a destroy composite carries both, so both fire.

**Atomic apply.** A surviving `Act` COMMITS its body directly (`Act(Destroy)`
apply *is* the Battlefield→Graveyard commit — `apply(Act(Destroy(x))) ==
apply(Move(x,Gy))`, tagged Destroy), not a `schedule_evolution` to a separate
replaceable `ZoneWillChange`. Realizations, all as the body facet:
- `Destroy(x)` → `→Graveyard` [CR#701.8a]. `Mill(who,n)` → top-`n` (clamped,
  [CR#701.17b]) batch `→Graveyard`. `Scry(who,n)` → look + within-library
  reorder. `Surveil`/`Fateseal` likewise.
- `Draw` is the ATOMIC single-object lane WITH Destroy, with a PURE-DATA body:
  the atom is a SINGLE card draw `Draw(Reference)` (no count — [CR#121.2]
  "individual card draws"), body `With(TopOfLibrary(1), Move(That, Hand))`.
  `Act(Draw)` is a single-object dual-facet composite that RETIRES `WillDraw`
  fully (as `Act(Destroy)` retired `WillDestroy`) — no card-intent, no
  front-loaded `ZoneWillChange`; on an empty library the binder yields nothing
  and the move no-ops. Multiplicity is a macro: `DrawN(who, n) = Repeat(n,
  Draw(who))` (`OneShotEffect::Repeat(Count, …)` exists, effect.rs). A single
  `Draw(who)` ALWAYS emits `Act(Draw)` (one ATTEMPT — the rules key on it), even
  from an empty library; the "no-op ⇒ no `Act`" rule fires only on a NULL
  instruction (`Repeat(0, …)`), never on an attempt that found no card.
  **Draw-from-empty is authored, not baked in** ([[rules-sba-subsystem]]): a
  reaction rule sets the persistent drew-from-empty flag on `Act(Draw)`-over-an-
  empty-library [CR#120.3,104.3c], and the existing loss `SbaRule` keys on that
  flag [CR#704.5b]. STAGE-4 design point to validate: whether the rules layer can
  set that flag reactively (fully authored) or the flag-set stays a one-line
  intrinsic bit of `Act(Draw)` apply (body still pure data either way). Preserve
  face-down-during-cast [CR#121.8] and the "without the word draw" distinction
  [CR#121.5]. NOTE: this decomposition is Draw-specific — Mill is SIMULTANEOUS
  ([CR#701.17a], "the milled cards" group [CR#701.17d]), so `MillN ≠ Repeat(n,
  Mill(1))`; Mill keeps a count and realizes as a batch/group `→Graveyard` move.
  Scry likewise keeps its count.
- **No-op ⇒ no `Act`.** A body that does nothing performs no keyword action, so
  the composite never surfaces (scry 0, a fully-canted destroy — [CR#701.22b]).
  Trigger fires post-commit so arrangement precedes it ([CR#701.22d]).

### Guards / replacements / SBAs

- Indestructible [CR#702.12b]: `StaticEffect::CantHappen(Act(Destroy(Ref(This))))`
  — reads like the card. (`CantHappen` takes an `EventFilter`; `Deontic::Cant` is
  for Attack/Block-style `DeonticAction`s and does NOT apply.) Ignores toughness-0
  ([CR#704.5f]) and sacrifice ([CR#701.21a]) — not the Destroy atom. Covers the
  effect destroy AND the lethal / deathtouch SBAs, which now PERFORM the Destroy
  keyword action (`Act(Destroy(x))`, `Agency::StateBasedAction`) instead of
  `WillDestroy` (`sba.rs:248`) — census "destroy immunity (SBA + Destroy)".
- "Creatures can't be destroyed" (global) = `CantHappen(Act(Destroy(_)))`.
- Regeneration [CR#701.19a]: `Replaces(Act(Destroy(Ref(This))) → tap + heal +
  remove-from-combat)` (was watching `WillDestroy`, `replace_registry.rs:270`);
  "can't be regenerated" rider unchanged [CR#701.19c]. idris already models both
  as `CantHappen`/`Replaces (MkEventQuery [Destroy] [Patient (SameAs This)])`
  (`Macros.idr:171,186`).
- Preserve cant→replace→apply order ([[engine-replacements]], `step.rs:1247`) so
  an indestructible creature never consumes its regen shield [CR#702.12b].
- **One replace step, both facets ([CR#616.1]).** Because the composite is a
  single event, regeneration (tag facet, `Act(Destroy)`) and a graveyard-move
  replacement (body facet, Rest-in-Peace's `→Graveyard`) are gathered into the
  SAME [CR#616] applicable-set and the affected player orders them — fixing the
  old two-event split (regen on `Act`, RiP on the downstream `ZoneWillChange` =
  two replace moments that could misorder). Dual-facet matching in the shared
  evaluator gives this for free.

### Provenance & result-side triggers

- "destroyed/milled this way" = the result zone-moves tagged with THIS `Act`
  instance's atom (generalizes the existing "milled this way" Noting
  product-groups; 1004 "this way" corpus hits vs ~1 standalone destroy≠dies
  trigger). The `cause` on a result `ZoneChange` = the producing `Act`'s atom,
  same bareword namespace [CR#701.8b,701.17c].
- "when it dies" = any Battlefield→Graveyard (shape), unchanged [CR#700.4].

### Surface — `Composite(tag, body)`, body is canonical stored data

Destroy stops being a bespoke `Action::Destroy` verb: it is authored (via macro,
so the author still writes "Destroy target creature") as
`Composite(Destroy(x), Move(x, Graveyard))` — the body IS the zone move, as
DATA, exactly like scry's stored reorder body. The engine holds no per-verb
`Action::Destroy`/`WillDestroy` and no hardcoded `Act(Destroy)`→`ZoneWillChange`
apply arm; the body-facet match-shape is read by **descending into the stored
body's head zone-change(s)** (`Move(_,Graveyard)`→`→Graveyard`; scry's reorder →
within-library, no graveyard) — "matches the expanded body." RON stays positional
bareword: `Act(Destroy(Ref(This)))`, `Act(Scry(2))`, `Act(Mill(3))`.

## Blast radius (churn irrelevant — state scope)

- **idris**: parameterize `KeywordActionSpec` (`Scry Nat | Destroy Reference |
  Mill Nat | Draw Nat | …`); verify re-emit soundness — this is the gate's turf
  ([[idris-probe-soundness-gate]]); if a verb genuinely resists parameterization,
  report rather than force.
- **core types**: one shared bare-ident newtype for the keyword-action-name
  namespace (`Composite.name`, `Act`, `Cause.verb`, `KeywordDecl.name`); remove
  `Action::Destroy`, `PlayerAction::Draw/Mill`, `GameEvent::WillDestroy`, the
  `CauseVerb` enum + `as_str` + `pub use`.
- **engine**: dual-facet `Act` — `FactView::of` carries tag + body-shape (descend
  the stored body), `eval` arms match facet-present not `kind ==`, apply commits
  the body atomically (no `ZoneWillChange` unwrap); one [CR#616] replace step over
  both facets; retarget indestructible + regen + lethal/deathtouch SBAs to
  `Act(Destroy(...))`; the ~8 `CauseVerb::*` construction sites → the newtype.
- **grammar**: parse⇄render⇄idris round-trip for the atoms
  ([[parse-via-macros-design-settled]], RENDER arms required
  [[canon-card-needs-renderer-for-fidelity]]) — `parsers/effect.rs`,
  `render/effect.rs:648,975,1446-1488`, `idris_emit.rs:1510`.
- re-encode existing `Composite(name: "…")` corpus to bareword-positional;
  regenerate `plugins/wizards`; re-bless fidelity (render text unaffected).

## Non-goals

- `Discard`/`WillDiscard` — shaped only; next atom once a fixture forces it.
- `ZoneWillChange` stays for PLAIN (non-keyword) moves — bounce, tuck, a bare
  `Move`. What changes: a keyword action no longer emits a SEPARATE replaceable
  `ZoneWillChange` below its `Act`; the dual-facet composite commits the tagged
  zone change directly on apply. The committed post-fact (`ZoneChanged`, tag
  carried) is unchanged.

## Verification

TDD off existing coverage — behavior identical, MUST stay green: `action.rs`
destroy/draw/mill/indestructible (`:805,:863,:951,:972,:1013,:1096,:1122,:1186`);
`sba.rs` lethal/deathtouch/indestructible/toughness-zero (`:579,:596,:1393,:1693,
:1725,:1818,:1843,:1860`); `replace_registry.rs:337`; the Stage-1 scry tests +
`cant_act_suppresses_composite_body`. Add: a destroy and a scry through the SAME
`Act` path; `Act(Destroy(_))` canted by indestructible while a toughness-0 move
is not; `entailments.ron`-membership rejection of an unknown atom.
`no_dead_grammar`, fidelity, keywords, builtin/canon load, idris re-emit,
`cargo xtask cite check` all green; `cite audit --diff` the touched cites.

Deps (all `done/`): engine-replacements, engine-resolve-actions,
macro-keyword-actions, engine-cause-constructors, engine-trigger-events,
migrate-damage-sbas-to-rules.

## Progress

- **Stage 1 (done, green — 529+213):** collapsed `KeywordActionPerformed`→ one
  present-tense `GameEvent::Act`; added `EventFilter::Act`; `Composite` resolve
  emits it with the schedule-time cant gate; post-completion ordering
  ([CR#701.22d]) preserved.
- **Stage 2 (done, green — 529+213):** parameterized bareword atom + `CauseVerb`
  retirement (→ `VerbName`); Destroy semantic migration — `WillDestroy` removed,
  indestructible/regen/lethal-SBA retargeted to `Act(Destroy)`. DEVIATION carried:
  `Act(Destroy)` still *unwraps* to a separate `ZoneWillChange` on apply, and a
  thin `Action::Destroy` emitter remains — superseded by Stage 2b.
- **Stage 2b (done, green — 213+533+cards, idris 5/5) — dual-facet atomic
  composite.** `GameEvent::Act` gained `from`/`to` (body facet, derived from the
  stored `Move` body by `composite_body_head_move_to`, not a per-verb table);
  `FactView::of(Act)` carries both facets; the `eval` `ZoneChange` arm matches a
  shaped `Act` in the **Replacement lane only** (so Rest-in-Peace's `→Graveyard`
  gathers with regeneration in ONE [CR#616.1] step, while dies TRIGGERS still
  fire on the committed `ZoneChanged` — no double-fire); the move-verb `Act`
  apply COMMITS the body via `apply_zone_will_change` (single replace
  opportunity, no downstream `ZoneWillChange`). Destroy authored as
  `Composite(Destroy(x), Move(x, Graveyard))` via `Action::destroy` ctor + a
  `Destroy` macro; `Action::Destroy` removed; render/idris route through the
  `Composite` arms (fidelity unchanged, idris emits `Composite (Destroy r)
  (moveAttacking r Graveyard)` — typechecks). DEVIATIONS: the idris
  `Core.idr` `Action.Destroy` constructor is now unused (harmless; emit never
  produces it) — cleanup deferred; canon `Collective Resistance`/`Do or Die`
  keep PRE-EXISTING emitter gaps (`EventFilter::Act`, `SeparatePiles`),
  untouched by this stage. Wizards regen + broad corpus re-encode stays Stage
  3–5 (no committed corpus card needed re-encoding — the `Destroy` macro
  renders identically, no fidelity re-bless).
- **Stages 3–5:** Mill, Draw (both `Composite(tag, body)`, same dual-facet path),
  then corpus re-encode + wizards regen + suites.
