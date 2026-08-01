---
needs: [core-pay-player-action]
design: true
---
**Make the macro layer the only author-facing vocabulary in card files: every
core primitive gets a (mostly generated) wrapping macro, and raw enum-variant
spellings stop parsing in card/token authoring positions — legal only inside
macro definition bodies.** This decouples the core Rust/Idris ident namespace
from the author-facing vocabulary: core variants get named for engine clarity
alone (retiring `action.rs:355-361`'s standing rule "keep any new bare
variant's name distinct from every macro name"), and future author-name
collisions (a newly printed keyword colliding with an existing name) become
data-side macro renames instead of Rust+Idris+emitter+frames churn.
Motivating scars: `DrawCard` is named to dodge the `Draw(N)` macro
(`action.rs:355-361` documents the shadowing mechanism); the
action-role-reshape spec carried a plan-time ident-collision sweep obligation
(its §16.3) that this policy retires for future types; `Nominate` was
rejected partly on future-keyword-space grounds.

Recon-verified premises (2026-08-01; file:line as of that date):

- Canon already stores macro CALLS unexpanded (`Draw(1)`,
  `plugins/canon/cards/Elvish Visionary.ron:15`), expanded at load
  (`MacroSet::read_str` → `EnumIntercept::visit_enum`,
  `crates/macro_ron/src/expand.rs:1519-1588`); `remembers_expansion()` kinds
  round-trip the INVOCATION, never the baked value
  (`macro_ron/src/expansion.rs:116-155`) — macro-preserving storage
  machinery already exists.
- Resolution is native-variant-first with SILENT shadowing: a macro named
  like a variant of its kind is simply dead — no diagnostic at registration
  (`expand.rs:1547-1565`; the `set.rs:622-650` validation chain never
  consults variant names) — and flatten/embed chains widen the collision
  surface beyond the immediate kind
  (`macro_ron_derive/src/generate.rs:615-619`).
- Macro bodies parse in the SAME namespace as card files — no separate body
  grammar; only `Ctx.frame` (`expand.rs:144-152`) distinguishes body
  re-reads from root reads at runtime. File-kind provenance lives one level
  up in `Plugin::load_onto`'s per-directory loaders
  (`deckmaste_cards/src/plugin.rs:147-338`: MACROS_DIR registration vs
  `card()`/`token()`/rules loaders) — the natural enforcement hook.
- Generation recombines existing parts: the wizards stamp-out loop
  (`deckmaste_migrations/src/stubs/subtypes.rs:21-85`; the `CreatureType`
  meta-macro pattern) + `xtask map enums`' syn-based variant reflection
  (`xtask/src/map.rs:65-119`); arity mirrors into `params:` with positional
  `Param` forwarding. No new reflection capability needed.
- Macro defs already carry author/render surface fields (`template:`,
  `frames:` — `plugins/builtin/macros/action/Draw.ron:24-51`), so generated
  identity macros are a natural home for per-verb surface text.

## Scope

- The parse-position rule: card/token (scope of rules files TBD, design Q1)
  authoring reads REJECT raw core-variant spellings at read time — a
  restricted read mode threaded from the per-directory loaders into
  `macro_ron`, rejecting inside `deserialize_enum` where the ident
  dispatches. (The riders-battlefield-only precedent's post-parse
  idris-gate style is too late for this rule — no value tree should exist.)
- Generated identity macros: one per authoring-reachable primitive variant,
  wizards-style (generated directory, wipe-first, never hand-edited);
  hand-authored macros keep owning the interesting names.
- Registration-time shadowing diagnostic: registering a macro whose name a
  reachable variant already owns becomes an ERROR, killing the
  silent-dead-macro footgun — worth landing even ahead of the full policy.
- Canon migration: re-spell residual raw-primitive usages in card files
  (expected small post-reshape).
- Docs: authoring guide; promote the settled policy to `docs/decisions/`.

## Design questions (the design gate)

1. Scope boundary: cards + tokens only, or also `rules/` data files?
   (Rules/fixtures are engine-adjacent IR — primitives there may be a
   feature, not a leak.)
2. Enforcement carrier: a read-mode flag on `MacroSet::read_str` vs a second
   restricted kind-set view; whether `Ctx.frame` participates or the split
   stays purely loader-level.
3. Identity-macro coverage where a hand macro already owns the good name
   (`Draw` macro vs `DrawCard` primitive): generated wrapper omitted, or
   coverage asserted some other way?
4. Does this unlock renaming `DrawCard`→`Draw` core-side? (Card-file
   shadowing dies with the ban, but macro BODIES still parse both
   namespaces, so the bare-variant-shadows-macro hazard survives inside
   bodies; the reshape deletes the `By` embed link while `Act`'s flatten
   remains.) Ride the policy or stay a follow-up?
5. One source of truth for per-verb surface text: identity macros'
   `template:`/`frames:` fields vs the frames catalog.

## Gates

Standard constraints apply. Specific: canon + workspace suites green;
`cargo xtask idris-check plugins/canon` no regressions; a negative fixture
proving a raw-primitive card file rejects with a useful error;
identity-macro generation deterministic (wipe-first regen, clean diff); zero
behavior change to expansion of existing canon.

## Sequencing

After `core-pay-player-action` (the `needs:`): identity macros generate once
over the final role-honest shapes, and the reshape's canon re-spelling isn't
churned twice.

Related: the action-role-reshape spec's §16.3 naming criterion (retired for
future types by this policy); `core-remove-default-args` (the same
no-implicit-surface philosophy); the macro-frames effort (the read-path
counterpart: frames render structures, macros spell them).
