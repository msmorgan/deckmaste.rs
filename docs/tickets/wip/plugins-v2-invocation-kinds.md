---
needs: []
---
**A `plugins_v2` declaration must be invocable at the semantic position its
body occupies.** Two family landings (2026-09-06) found it is not:

- `Adamant(<ability>)` at an `Ability` slot does not resolve. A declaration
  carries exactly one kind, its family (`AbilityWord`, `KeywordAction`,
  `KeywordAbility`), `deckmaste_construction_core::macro_def::normalized_kind`
  refuses a second, and `deckmaste_semantics_v2`'s reader registers the macro
  under that family kind. Only families whose name coincides with a syntax
  type (`TurnPart`, `CounterKind`, `Subtype`) are invocable today. The
  ability-word canon cards spell `ItalicHead(...)` directly as a result.
- A `TurnPart` declaration whose name equals its body's constructor
  (`Upkeep` → `Upkeep`) is refused at register time as a self-invocation:
  `macro_ron::set::check_cycles` exempts an identity macro only when the
  kind's variant list contains the name, and `semantics_v2::ron::kinds()`
  never registers `TurnPart::kind()`. Eight turn-part declarations are
  blocked on it (their intended bodies sit in each file's STOP comment).

Fix, keeping the file as the shared contract (`semantics-v2.md` §11) and
kinds as one-per-`SupportsMacros`-enum (§12):

- Each meta-declaration emits `kinds: [<Family>, <SemanticKind>]`
  (`KeywordAction` → `Instruction`, `KeywordAbility` → `Ability`,
  `AbilityWord` → `Ability`; the coinciding families stay single). Update
  `plugins_v2/builtin/macros/meta/*.ron`; the stub files need no change.
- `construction_core` reads the family kind it knows and tolerates further
  kinds; `semantics_v2`'s reader registers each declaration under every
  kind that names a `SupportsMacros` enum and ignores family-only kinds.
- `semantics_v2::ron::kinds()` registers every `SupportsMacros` enum's kind,
  `TurnPart` included, so the identity-macro exemption sees its variants.
- Re-point the ability-word canon cards to invoke their macros
  (`Landfall(...)`, `Raid(...)`, `Threshold(...)`), and add a
  `plugins_v2_declarations` test that invokes one declaration of each
  family at its semantic position and expands it. `lean-check
  plugins_v2/canon` proves every card.

Standard constraints apply. Gate closure: construction_core, english_v2,
semantics_v2, xtask.
