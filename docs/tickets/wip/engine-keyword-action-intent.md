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
KeywordAction = Destroy Reference | Draw Count | Mill Count
              | Scry Count | Surveil Count | Fight Reference | …
```

Authored positionally as a **bareword** atom (one shared bare-ident newtype for
the whole keyword-action-name namespace — `Composite`/`Act`/`Cause`/`KeywordDecl`
names — mirroring `KeywordRef`/`CostTag`; fixes the `"Ward"` quoting wart). The
atom subsumes the old bespoke `Action::Destroy(Reference)` — that IS the
`Destroy(Reference)` arm, not a variant to keep. Typo-safety: the re-emit gate
rejects unknown atoms; add an `entailments.ron`-membership check. Retire the
`CauseVerb` enum into this namespace.

### The `Act` event — present tense, one event, three roles

`GameEvent::Act(KeywordAction)` (present tense; NOT a pre/post `WillAct`/`Acted`
pair — a surviving `Act` in the log IS the "it happened" fact). Resolving a
keyword action:

1. **Guard/replace point.** Emit `Act(<atom>)`. `CantHappen`/`Replaces` bite it
   by matching the atom. If suppressed/replaced, the realization never runs.
2. **Unwrap on apply.** A surviving `Act` realizes per-atom, mirroring today's
   `WillDestroy → ZoneWillChange`:
   - `Act(Destroy(x))` → `ZoneWillChange(Battlefield→Graveyard)` tagged with the
     Destroy atom [CR#701.8a]. (`apply(Act(Destroy(x))) == apply(Move(x,Gy))`.)
   - `Act(Mill(n))` → top-`n` (clamped, [CR#701.17b]) → Graveyard batch.
   - `Act(Draw(n))` → per-card, one at a time [CR#121.2]: library non-empty →
     move top to Hand + bump `CardsDrawn`; empty → `DrewFromEmpty`
     [CR#121.4,704.5b]. (The empty branch is why Draw keeps a card-intent — it
     can't front-load a `ZoneWillChange{object}` with no object. Preserve
     face-down-during-cast [CR#121.8] and the "without the word draw"
     distinction [CR#121.5].)
   - `Act(Scry(n))` → look + reorder (moves within the library).
3. **Trigger fact.** The surviving `Act` is what "whenever you scry/mill/draw"
   reads. Gated on the atom actually acting (scry 0 / fully-replaced destroy →
   no `Act`, [CR#701.22b]). Fires post-completion so arrangement precedes it
   ([CR#701.22d]); the guard CHECK runs at schedule-time on the `Act` value,
   decoupled from where the fact lands (proven in Stage 1).

`EventFilter::Act(<atom-pattern>)` matches by the atom: `Act(Destroy(Ref(This)))`
(this only), `Act(Destroy(_))` (any destroy), `Act(Scry(_))` (any scry). The
patient is the atom's own argument — there is NO separate `on` field and NO
patient-default fork.

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

### Provenance & result-side triggers

- "destroyed/milled this way" = the result zone-moves tagged with THIS `Act`
  instance's atom (generalizes the existing "milled this way" Noting
  product-groups; 1004 "this way" corpus hits vs ~1 standalone destroy≠dies
  trigger). The `cause` on a result `ZoneChange` = the producing `Act`'s atom,
  same bareword namespace [CR#701.8b,701.17c].
- "when it dies" = any Battlefield→Graveyard (shape), unchanged [CR#700.4].

### Surface — positional bareword atoms, no named params

Card-facing text is unchanged ("Destroy target creature", "Scry 2"). RON is
positional bareword throughout: `Act(Destroy(Ref(This)))`, `Act(Scry(2))`,
`Act(Mill(3))` — no `name:`/`on:` binders.

## Blast radius (churn irrelevant — state scope)

- **idris**: parameterize `KeywordActionSpec` (`Scry Nat | Destroy Reference |
  Mill Nat | Draw Nat | …`); verify re-emit soundness — this is the gate's turf
  ([[idris-probe-soundness-gate]]); if a verb genuinely resists parameterization,
  report rather than force.
- **core types**: one shared bare-ident newtype for the keyword-action-name
  namespace (`Composite.name`, `Act`, `Cause.verb`, `KeywordDecl.name`); remove
  `Action::Destroy`, `PlayerAction::Draw/Mill`, `GameEvent::WillDestroy`, the
  `CauseVerb` enum + `as_str` + `pub use`.
- **engine**: `Act` resolve/apply per-atom realization; retarget indestructible +
  regen + lethal/deathtouch SBAs to `Act(Destroy(...))`; the ~8 `CauseVerb::*`
  construction sites → the newtype.
- **grammar**: parse⇄render⇄idris round-trip for the atoms
  ([[parse-via-macros-design-settled]], RENDER arms required
  [[canon-card-needs-renderer-for-fidelity]]) — `parsers/effect.rs`,
  `render/effect.rs:648,975,1446-1488`, `idris_emit.rs:1510`.
- re-encode existing `Composite(name: "…")` corpus to bareword-positional;
  regenerate `plugins/wizards`; re-bless fidelity (render text unaffected).

## Non-goals

- `Discard`/`WillDiscard` — shaped only; next atom once a fixture forces it.
- Do NOT fold `ZoneWillChange` away — it stays the committed result; `Act` sits
  above it. One `Act` atom for ALL keyword actions, not a per-verb event.

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
- **Stage 2 (next):** the parameterized bareword atom + `CauseVerb` retirement +
  the full Destroy migration (macro→atom, remove `Action::Destroy`/`WillDestroy`,
  retarget indestructible/regen/SBAs, round-trip).
- **Stages 3–5:** Mill, Draw, then corpus re-encode + wizards regen + suites.
