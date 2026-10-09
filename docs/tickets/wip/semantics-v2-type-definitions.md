---
needs: []
---
**A `Type` declaration's body is read by v2.** Routed on 2026-10-08 from
`plugins-v2-keyword-body-defects`, whose type half could not land: the three
type declarations that confer something (`creature`, `land` and `instant` under
`plugins_v2/builtin/macros/types/`) still write raw v1-shaped conferrals
inside `TypeDef(…)`, and nothing in v2 reads them. Design-bearing (Lean first);
standard constraints apply.

## Why it is blocked today

- `Type` is a loader tag in v2 (`DECLARATION_KINDS` doc in
  `crates/deckmaste_semantics_v2/src/ron.rs`): v2's card-type position is
  `CardType`, and no ticket has reconciled the two names.
- Lean `Definition` (`lean/Semantics/Rules.lean`) has no type constructor and
  the mirror has no `TypeDef`, so `cargo xtask expansions` skips all ten
  `types/` declarations ("no kind of [Type] is a v2 syntax type, and the body
  (`TypeDef`) is not a registry node"). A re-spelling of their conferrals
  would have no expansion proof.

## The change

1. Decide the v2 shape of a card type's Registry Definition (name, whether it
   is a permanent type, its conferrals), Lean first, and mirror it.
2. Make `Type` declarations read through it, and add its head to
   `DEFINITION_HEADS` in `crates/xtask/src/expansions.rs` so `types/` prints.
3. Then re-spell the conferrals over helpers that expand to the same term:
   creature's Combatant role [CR#508.1a], land's play permission
   [CR#305.1], and the instant cast permission [CR#304.1]. A re-spelling
   leaves every expanded term identical.
