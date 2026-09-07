---
needs: [lean-constructor-collapse]
---
**Promote `docs/decisions/semantics-v2.md` from draft by amending it with how
the layer lands as Rust.** Decisions settled with the user on 2026-09-06;
the ADR records them, it does not reopen them. Also update
`docs/decisions/README.md`, the `## Crate fates` section of `CLAUDE.md`, and
the workbench-succession banner in the ADR.

Sections to add:

- **Rust representation.** `crates/deckmaste_semantics_v2` mirrors the Lean
  syntax (`lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card}.lean`)
  constructor-for-constructor and field-for-field. Lean is the spec: a Lean
  change is a Rust change, never the reverse. The crate does no law checking;
  the Lean gate is the only checker, and `deckmaste_lowering_v2` may fail to
  lower a card that breaks a law without going out of its way to validate.
- **Plugin format.** `plugins_v2/` is the v2 format; `plugins/builtin_v2`
  moves to `plugins_v2/builtin` (symlink left behind until english_v2 and
  xtask paths move). One declaration file per macro carries `name`, `params`,
  `spelling`, `grammar`, and a semantic `body`: english_v2 reads the spelling
  and grammar, semantics_v2 reads the params and body, the file is the shared
  contract and neither crate depends on the other for it. Today's bodyless
  declarations (keyword actions, keyword abilities, ability words, turn
  parts) grow bodies per family; no new bodyless declarations. Cards, tokens,
  and the three rules tables (state-based actions, conferrals, damage
  results) are further kinds in the same tree. `plugins_v2/canon` is
  hand-authored RON first; English translation from `OracleText` is a later
  crate.
- **Macro system.** `macro_ron` unchanged: it stays a dumb expander with
  `MacroDef<Metadata>` opaque metadata. Kinds are one per `SupportsMacros`
  enum, existing only to disambiguate same-name macros at different usage
  sites. Macros invoke other macros; no self-recursion. Lean's
  `MacroParameters` classes are a proof device, not the kind set.
- **Gate.** The `lean-card-soundness-gate` emitter writes fully expanded
  terms as untracked generated Lean; `Macros.lean` plays no part in the gate.
  Generating `Macros.lean` from the declarations is parked
  (`lean-macros-from-ron`). The hand-written `Cards/` bench is a stand-in:
  as each card lands in `plugins_v2/canon`, the emitted term supersedes its
  hand spelling (`lean-hand-bench-retirement`), and canon cards without Lean
  versions get them by emission.
- **Parity and cutover.** v1 retires when card translation reaches parity,
  measured by `semantics-v2-parity`: per-card comparison of v2 and v1
  lowerings, v1 as a starting point not an oracle, disagreements adjudicated
  by hand, then the whole-game slow tests. At cutover
  `deckmaste_semantics_v2` takes the `deckmaste_semantics` name and
  `plugins_v2/` becomes `plugins/`. Deletion set, to be enumerated from
  `cargo metadata` with differences from this list reported:
  `deckmaste_semantics`, `deckmaste_lowering`, `deckmaste_legacy_render`,
  `deckmaste_plugin` (`deck.rs` and `provenance.rs` rehomed, `energy.rs`
  dropped as vestigial, verify the `deckmaste_migrations` call site goes with
  it), `idris/`, `plugins/builtin`, `plugins/canon`, and the splice
  machinery of `deckmaste_spelling` already slated by
  `english-v2-rewrite.md`. `deckmaste_migrations` sheds legacy_render and
  survives.
- **Registries.** The `plugins_v2/builtin` declarations are the sole v2
  registries and already feed `Facts.lean`; v1's registries under
  `plugins/builtin/macros` are frozen until deletion.

Leave §3's joined-kind payload flagged open; it is a design item for
`semantics-v2-crate`, not a blocker on promotion.
