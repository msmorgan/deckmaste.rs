---
needs: [lowering-crate]
---
**Repoint loading and migration at the authoring grammar: parse
`deckmaste_authoring` types, lower to core for everything downstream.**
Design:
`docs/decisions/authoring-spelling-lowering.md`
(§11). Zero behavior change — the engine receives byte-identical core
values.

## Scope

- `Plugin::card`/`token`, the per-directory loaders (including the
  `rules/{sba,grant,damage}` tables — they are authored containers too),
  and the migrations graduation/parser paths construct authored terms and
  call `lower`. The loader grows a DUAL-RESULT contract (authored term AND
  lowered core value; provenance erased exactly at `lower`) so spelling
  and the Idris emitter keep the authored form while the engine receives
  core.
- **Typed-reader inventory** (spec §4): validation, fidelity, the canon
  comparison path, and the Idris bulk emitter currently `read_str` typed
  `Card`/`Token` values directly, bypassing `Plugin::card` — all route
  through the one restricted read API, with a grep gate forbidding
  remaining direct `read_str::<Card|Token>` calls. Engine strategy RON is
  explicitly OUTSIDE the program (plain core serde; spec §3/§4 table).
- Provenance stays put: the lowering map's `Expansion` arms remain
  identities here, so the lowered core is byte-identical to today's and
  every consumer of invocation provenance keeps working untouched. The
  erasure arms, the per-variant mapping-test edits they force (spec §16.3),
  and the ~97 dead `Expanded` match sites in `deckmaste_engine` land with
  their compensation in `runtime-prose-link` (spec §12, sequenced
  2026-08-02).
- The dual result is a RETAINABLE pair: `Deck::resolve` stops collapsing to
  bare `Arc<Card>` so the authored half survives to the game-composition
  layer. Delimitation vs `runtime-prose-link`: the pair shaping and the
  `Deck::resolve` non-collapse are THIS ticket's; the `CardId` companion
  table, the ability provenance index, the erasure arms, and the
  renderer/fidelity move-and-repoint are that ticket's.
- `deckmaste_core::strategy` moves to `deckmaste_engine::strategy`
  (owner-settled 2026-08-02): strategy RON is engine configuration read by
  plain core serde (spec §3/§4) and `StrategyEvaluator::from_ron` already
  reads it raw, so `Preference`'s macroable-kind registration
  (`core::ron::kinds`, the plugin `param_types` row, the
  `deckmaste_frames::guard` row) is vestigial and comes out with it —
  nothing in `plugins/**` declares `kinds: [Preference]`.
- Frames guard-constant reads retarget to authoring kinds. Delimitation vs
  `spelling-crate-rename`: THIS ticket retargets the loader-supplied
  MacroSet/guard-constant read path; the frames crate's own internal type
  table is the rename ticket's (the two are unordered; each owns its
  side).

## Gates

Standard constraints apply. `lower(authored_read(src)) == core_read(src)`
byte-identical over canon, builtin, and wizards; full workspace suites;
`cargo xtask idris-check plugins/canon` no regressions; fidelity PASS; the
authoring and core writers agree byte-for-byte on every canon/builtin/wizards
file, so an invocation survives write-back through either grammar
(`deckmaste_plugin/tests/corpus_identity.rs`); no direct
`read_str::<Card|Token>` outside the one restricted read API — the migration
oracle in that same file reads through BOTH grammars by design, but always at
a generic parameter, so it is invisible to the gate's matcher rather than
exempted by it, and it retires with `core-demacro`.
