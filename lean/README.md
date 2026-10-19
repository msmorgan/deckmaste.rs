# Semantics workbench in Lean 4

The semantics workbench models the card language's syntax and checks its
structural obligations. `Semantics/*.lean` contains plain inductives;
`Semantics/Check/*` computes bindings, kinds, zones, and refusals over that
syntax. `Card.check` returns every refusal, and a `Spelled` card carries a
kernel-checked proof that its refusal list is empty. Runtime game execution
remains in Rust.

Run `lean/scripts/build` from the repository root, or `./scripts/build` from
this directory. It builds the syntax, checker, templates and definitions, printed-card bench,
and pin suites with warnings treated as failures. For the
semantics inner loop, run `lake build Semantics` from this directory.

Lean is the active workbench. The [succession decision](../docs/decisions/lean-is-the-workbench.md)
records the frozen Idris reference and migration history.

## Workbench gate

`cargo xtask lean check` runs `lake build --wfail` in `lean/`, the same full
workbench gate as `lean/scripts/build`. It builds the active syntax, checker,
templates and definitions, card bench and proofs, and propagates build failures.

The obsolete `plugins_v2` card/definition emitters and macro generator have
been retired. `lean definitions` and `lean macros generate|check|align` are no
longer commands. The ported v3 declarations now live in `Semantics/Templates.lean` and
`Semantics/Definitions.lean`, maintained in Lean while the replacement DSL is
designed; v2 RON output must not regenerate them. Existing v2 readers elsewhere remain until their consumers
are cut over.

### Retired tooling tests

The following tests retired with their implementations. The native Lean
proofs and card bench continue to run; the new command tests cover successful
builds, failed builds and failure to launch Lake.

- `lean_macros` (unit): `every_param_type_has_a_lean_reader`, `every_syntax_kind_has_a_lean_reader`, `the_committed_module_matches_the_declarations`, `a_hole_is_the_whole_argument`.
- `lean_check` (unit): `a_plugin_directory_names_a_lean_module_component`, `a_lean_diagnostic_keeps_its_position_severity_and_whole_message`, `a_warning_is_a_diagnostic_too_because_the_build_treats_it_as_one`, `a_diagnostic_lands_on_the_card_whose_block_it_sits_in`, `a_card_that_fails_to_elaborate_records_the_failing_head`, `a_diagnostic_above_the_first_card_is_a_gate_defect`, `a_diagnostic_in_a_file_no_card_owns_is_a_gate_defect`, `a_lake_failure_that_names_no_card_is_a_gate_defect`, `a_module_that_was_neither_built_nor_diagnosed_is_a_gate_defect`, `a_report_fails_iff_any_card_did_not_prove`.
- `definition_check` (unit): `emit_keeps_the_build_cache`.
- `lean_check` (integration): `the_testing_plugin_proves_every_card`, `a_card_that_breaks_a_card_law_is_singled_out_and_the_gate_fails`, `a_lake_that_fails_without_naming_a_card_stops_the_gate`, `a_real_run_recovers_after_a_stubbed_lake_failure`, `every_builtin_registry_definition_proves`, `a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`, `a_lake_that_fails_without_naming_a_definition_stops_the_definition_gate`.

The macro tests checked v2 reader parity, parameter-hole extraction and exact
v2 regeneration. The card/definition tests checked the retired v2 emitter,
its diagnostic attribution and generated-file handling. These contracts no
longer apply to the native v3 workbench.

## Syntax, templates and definitions

The syntax modules are `Words`, `Events`, `Abilities`, `Card`, and `Rules`;
`Primitives` wraps their constructors, `Templates` holds the transparent phrasings, and
`Definitions` the rules-defined vocabulary (keyword actions and abilities, counters, tokens and
designations), followed by the few templates written over it. A constructor earns its place through an obligation or structural
distinction its expansion would not carry. Common phrasings live in `Templates`:
for example, "permanent", "colorless", "historic", "spell" and "attacking" expand into
shared predicates.

Actions and their templates and definitions use dictionary forms (`exile`, `mill`, `gainLife`);
static-spec constructors use nouns (`modification`, `abilityGrant`, `replacement`).
Event constructors retain their predication forms. Action composition uses `sequentially` and `simultaneously`; fixed repetition
lives in `Repetition.fixed`. No action writes its performer: it is
`actor`, the controller unless an enclosing `act performer body` hands it to
another player or a permanent; "target player draws a card" is
`act (target .anyPlayer) (draw (.lit 1))`. A choice's chooser and an added
turn's taker are the actor too ("target player chooses a creature" is
`act (target .anyPlayer) (choose (a creature))`). `choose` accepts
`(disclosure := .secretly)`.

Combat predicates and events carry a `CombatRelation`; `statOf` takes a
`ProjAxis`; arithmetic takes an `ArithOp`; `Predicate.or` joins alternatives
with their own kinds. Designations are open labels whose declared scope
feeds the checker tables. `Characteristics` is flat and follows
[CR#109.3]. Printed stat slots use `Option Amount`: `none` represents a printed
`*`, with a characteristic-defining ability supplying its value. `CardFace`
adds what a printed face carries; `CharacteristicBundle` describes what an
effect writes, including token qualities [CR#111.3].

## Rules tables

`Rules.lean` is the rules-as-data a plugin authors beside its cards: an
`SbaRule` [CR#704.1], a `ConferralRule` [CR#306.5b], a `DamageResultRule`
[CR#120.3], and a `PredefinedToken` catalog entry [CR#111.10]. Each row is
scoped by a `Predicate` instead of printed on a face, and `Check/Rules`
checks it with the same functions a card's text obeys — the row's own
obligations are only what the scope adds: the kind it may bind, that a
state-based action's effect is untargeted, that a conferred ability is one an
effect could grant, and that a damage result's counter is a declared one whose
holder is the kind the recipient binds. A conferral is an ordinary `Ability`
and a catalog entry an ordinary `CharacteristicBundle`, checked through
`TokenSpec.written`; neither is a twin type. The pins are `Proofs/Rules`, whose
bench items are the rows `plugins/builtin/rules` and `plugins/builtin/tokens`
write.

## Registry facts

`cargo xtask facts generate` writes `Semantics/Check/Facts.lean` from the
builtin_v2 registry declarations and xtask's checker-column overlays.
`cargo xtask facts check` rejects stale Lean or reference Idris output.
`FactTypes.lean` owns the shared data columns; `Check/Words` and
`Check/Keywords` interpret them. A designation declaration generates one table
row per label it names, so an enum-valued one generates all its members.
The table-integrity pins remain part of `lean/scripts/build`.

## Numbers

The only numbers the game uses are integers [CR#107.1], so `Amount.lit`
carries an `Int` and a printed negative face is spellable — Spinal Parasite's
−1/−1, Char-Rumbler's −1 power — while a written `*` stays `none`. What
[CR#107.1b] adds is that most positions clamp: a calculation that determines
the result of an effect yields zero rather than a negative, except where the
effect sets, doubles, or triples a life total or a creature's power and
toughness. So every position that consumes an `Amount` declares its regime as
data the engine reads, in `Action.numberSlots`, `StaticSpec.numberSlots`,
`Cost.numberSlots`, and `GameEvent.numberSlots`: `clamped` for damage, life
gain and loss, draws, counts, costs, choices, and the letter X; `signed` for
an effect that sets a value, for exchanges of values, and for the raw reads
and comparisons inside an `Amount`, which are calculations and not effects.
Nothing here is a refusal — "gains −1 life" means gain 0, and that meaning is
the slot's declared regime. The pins are `Proofs/Numbers`.

Present power, toughness, loyalty, and defense in a printed box must be
`Amount.lit`; a nonliteral receives `cardBox`, including on shared-line halves,
level bands, and prototype frames. Effect-written characteristics keep their
context-aware expression checks. `Parity` is the even/odd number vocabulary,
also used when a deck condition tests mana values.

## The checker

1. **Syntax** (`Semantics/*.lean`) supplies the card-language terms.
2. **Attributes** (`Check/Words`, `Check/Phrase`, `Check/Triggers`,
   `Check/Abilities`) compute bindings and projections such as kind, number,
   zone and type. Field contexts determine which earlier introductions each
   child can read.
3. **Rules** (`Check/Phrase`, `Check/AbilityRules`,
   `Check/Card`) check those projections and return named `Refusal` values.
   Laws read structural or declared features; labels are table keys.
4. **Entry point**: `Card.check : Card → List Refusal`. A `Spelled` value
   combines a card with a proof of `card.check = []`; `spelled` asks `decide`
   to construct that proof.

`Predicate.kind?` and `NounPhrase.kind?` infer kinds. A context expecting a
kind refuses a `kindMismatch` when they disagree. Each disjunct of a joined
phrase is checked at its own kind. The [checker contracts](CONTRACTS.md)
record field contexts, enclosed scopes, and the checks or planned redesigns
that enforce them.

## Pins

A positive pin proves that the exact refusal list is empty; its negative twin
proves the exact expected list. For example, `Proofs/Anaphora.lean` contains:

```lean
theorem badChooseYou :
    Action.check [] (.choose none .you .openly none) = [.choiceClause] := by decide
```

`okX` / `badX` names and card sentences are retained when a term is re-spelled.
Changing the syntax does not authorize weakening the asserted result or
removing its twin. `Proofs/Tables.lean` also checks table uniqueness and
conferral boundaries. `decide` evaluates these laws in the kernel.

## Cards

`Semantics/Cards/<Family>.lean` contains the printed-card bench. Each card is a
`Spelled` value, so a card that stops checking stops defining. Phrase-level
bench items have their own named acceptance or refusal theorems beside them.
`SemanticsCards` builds the card families; `SemanticsProofs` builds the pin
suites. The full build includes both.

A refused card identifies a missing template or definition, a missing construct, or a checker
regression. Keep its sentence and assertion while repairing that boundary.
The bench follows the repository's Vintage-playable scope; out-of-scope
formats are not reserved in the syntax.

## Structural recursion is load-bearing

`decide` asks the kernel to evaluate the checker, and the kernel cannot unfold
well-founded recursion. Lean falls back to well-founded recursion *silently*,
so every recursive definition in `Check/*` carries `termination_by structural`
and a build error, not a slow proof, is what a non-structural definition
produces. The traps, all hit once:

- a call on the *same* argument (`sliceTy d g` calling `NounPhrase.ty g`):
  pass the computed attribute in instead (`sliceTyOf d (NounPhrase.ty bs g)`);
- a projection of a structure argument matched as a variable (`c.text`):
  match the constructor (`⟨_, _, _, _, _, _, _, text, qualities, …⟩`);
- a standalone function recursing through a nested list (`flatten`): make it
  a `mutual` pair with the per-element function.

`#print axioms f` showing `Quot.sound` means `f` fell back.

## Conventions

- Syntax types live in `namespace Semantics`; definition collections use
  their module path, such as `Semantics.Templates` and `Semantics.Proofs.Anaphora`.
- Types use UpperCamelCase; constructors and functions use lowerCamelCase in
  the type's namespace. Fields and arguments use descriptive words.
- Lean keywords used as names take a trailing underscore, such as `if_`,
  `from_`, and `return_`.
- Write templates and definitions bare (`creature`) and constructors with a leading dot
  (`.hasType`). The bench opens `Semantics.Templates` and `Semantics.Definitions`; it does not open syntax
  namespaces whose constructors would shadow core names.
- Syntax is unindexed. Contexts and inferred attributes are explicit inputs
  and results of the checker. Sort-specific constructors retain the
  distinctions needed to check their fields.
- Types derive `Repr` and `BEq`, and `DecidableEq` where derivation supports
  their recursive shape. Rules-bearing docstrings use bracketed Comprehensive Rules citations.
