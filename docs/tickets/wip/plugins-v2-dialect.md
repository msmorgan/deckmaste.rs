---
needs: []
---
**The v2 RON dialect: what a card may write.** Rulings (user, 2026-09-07)
after reading the first canon cards, which spell
`Simple(symbol: Specific(color: Of(color: Green)))` where v1 wrote `Green`
and the Lean bench writes `pip .green`. Five parts, in this order; one
claim, since each depends on the one before.

1. **Injections embed.** Every mirror constructor with exactly one field
   whose type is another syntax type (`ManaSymbol::Simple`,
   `SimpleManaSymbol::Specific`, `ColorOrColorless::Of`, `ColorTerm::Lit`,
   and the rest; enumerate mechanically from the mirror) carries
   `#[macro_ron(embed)]`, and single-field newtypes `#[serde(transparent)]`,
   so `Red` at a mana position reads through the chain. The mechanism is
   v1's (`crates/deckmaste_semantics/src/mana.rs` explains why embed rather
   than `serde(untagged)`); the rule is the user's from v1 and is not
   loosened. Where two injections in one enum would both accept the same
   text, the reader refuses with both names rather than guessing; report each
   such pair.
2. **Positional application, names optional.** Lean writes
   `.simple (.specific (.of .green))` and `exile x (agent := .you)`. The
   reader accepts positional arguments mapped by the constructor's declared
   field order, and named arguments in any position after them, for every
   struct variant; the mirror keeps its struct fields (binder names are the
   contract with Lean, and the drift test reads them). The field-order
   inventory the drift test already builds is the reader's table.
3. **Literal leaves.** Lean marks `Amount.lit` with `semantic_literal`; the
   reader reads a bare numeral there. Mark the mana pip and generic-symbol
   leaves the same way on both sides so `[2, Green, Blue]` is a mana cost.
4. **The helper macro layer.** Port every non-keyword `semantic_macro` in
   `lean/Semantics/Macros.lean` (about 330 of 401) to declarations under
   `plugins_v2/builtin/macros/<family>/`, one file per macro, so a card can
   write what the Lean bench writes (`target creature`, `graveyard`,
   `thisPermanent`, `sequentially`, …). Keyword families keep their
   declaration metas; helpers get a plain meta with `name`, `kinds`,
   `params`, `body`, no spelling or grammar. Lean's named defaults become
   explicit parameters (no default slots). This is ADR §6's direction:
   `Macros.lean` is later generated from these.
5. **Macro-only cards.** Lean's `spelled` refuses a card whose term holds a
   raw constructor outside `semantic_literal` leaves and macro parameters
   (`Authoring.Form.onlyMacros`). The v2 reader enforces the same on card
   text before expansion, with the same refusal shape (list the raw names).
   Then convert every `plugins_v2/canon` card and every family body by
   load-and-reserialise through the new dialect, so nothing is rewritten by
   hand, and `lean-check` proves every card.

0. **Unknown fields are refused.** The reader silently drops a field a
   constructor does not declare: Fading and Impending wrote `amount:` for a
   `quantity` field and read as a count-less removal; retired arguments
   (`conferral:`) passed unnoticed. Every reader path refuses an unknown
   field with the constructor and field named; a test writes one and
   asserts the refusal. Lands first, before the conversion below.

Also: rename `plugins_v2/builtin/macros/stubs/<family>` to
`plugins_v2/builtin/macros/<family>` and re-point `read_builtin_v2`,
`facts.rs`, `gate.rs`, and the tests; the files are no longer stubs.

Record the dialect in `docs/decisions/semantics-v2.md` §11 (what a card may
write) and §12 (what a macro may write). Standard constraints apply; the
gate closure includes construction_core, english_v2, semantics_v2, xtask.
