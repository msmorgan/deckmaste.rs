# Semantics workbench in Lean 4

The semantics workbench models the card language's syntax and checks its
structural obligations. `Semantics/*.lean` contains plain inductives;
`Semantics/Check/*` computes bindings, kinds, zones, and refusals over that
syntax. `Card.check` returns every refusal, and a `Spelled` card carries a
kernel-checked proof that its refusal list is empty. Runtime game execution
remains in Rust.

Run `lean/scripts/build` from the repository root, or `./scripts/build` from
this directory. It builds the syntax, checker, macros, printed-card bench,
and pin suites with warnings treated as failures. For the
semantics inner loop, run `lake build Semantics` from this directory.

Lean is the active workbench. The [succession decision](../docs/decisions/lean-is-the-workbench.md)
records the frozen Idris reference and migration history. Rust card re-emission
into Lean remains tracked by
[lean-card-soundness-gate](../docs/tickets/planned/lean-card-soundness-gate.md);
a successful workbench build does not claim that gate is implemented.

## Syntax and macros

The six syntax layers are `Words`, `Events`, `Phrase`, `Triggers`, `Abilities`,
and `Card`. A constructor earns its place through an obligation or structural
distinction its expansion would not carry. Common phrasings live in `Macros`:
for example, "permanent", "colorless", "historic", "spell", "attacking", and
"regenerates" expand into shared predicates and events.

Instructions and their macros use dictionary forms (`exile`, `mill`, `gainLife`);
static-spec constructors use nouns (`modification`, `abilityGrant`, `replacement`).
Event constructors retain their predication forms. An explicit instruction agent
is a trailing named argument. `exile thisPermanent` omits the agent;
`exile thisPermanent (agent := NounPhrase.you)` supplies one. Omission preserves
`none` for optional agent slots; required player slots default to `.you`.
`choose` also accepts `(disclosure := .secretly)`.

Combat predicates and events carry a `CombatRelation`; `statOf` takes a
`ProjAxis`; arithmetic takes an `ArithOp`; `Predicate.or` joins alternatives
with their own kinds. Designations are open labels whose declared scope and
conferrers feed the checker tables. `Characteristics` is flat and follows
[CR#109.3]. Printed stat slots use `Option Amount`: `none` represents a printed
`*`, with a characteristic-defining ability supplying its value. `CardFace`
adds what a printed face carries; `CharacteristicBundle` describes what an
effect writes, including token qualities [CR#111.3].

## Registry facts

`cargo xtask facts generate` writes `Semantics/Check/Facts.lean` from the
builtin_v2 registry declarations and xtask's checker-column overlays.
`cargo xtask facts check` rejects stale Lean or reference Idris output.
`FactTypes.lean` owns the shared data columns; `Check/Words` and
`Check/Keywords` interpret them. Designation conferrers are declared on
`DesignationDecl` and generate lists, including all sectors and both doors.
The table-integrity pins remain part of `lean/scripts/build`.

## Numbers

The only numbers the game uses are integers [CR#107.1], so `Amount.lit`
carries an `Int` and a printed negative face is spellable — Spinal Parasite's
−1/−1, Char-Rumbler's −1 power — while a written `*` stays `none`. What
[CR#107.1b] adds is that most positions clamp: a calculation that determines
the result of an effect yields zero rather than a negative, except where the
effect sets, doubles, or triples a life total or a creature's power and
toughness. So every position that consumes an `Amount` declares its regime as
data the engine reads, in `Instruction.numberSlots`, `StaticSpec.numberSlots`,
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
3. **Rules** (`Check/PhraseRules`, `Check/Triggers`, `Check/AbilityRules`,
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
    Instruction.check [] (.choose none .you .openly none) = [.choiceClause] := by decide
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

A refused card identifies a missing macro, a missing construct, or a checker
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
  their module path, such as `Semantics.Macros` and `Semantics.Proofs.Anaphora`.
- Types use UpperCamelCase; constructors and functions use lowerCamelCase in
  the type's namespace. Fields and arguments use descriptive words.
- Lean keywords used as names take a trailing underscore, such as `if_`,
  `from_`, and `return_`.
- Write macros bare (`creature`) and constructors with a leading dot
  (`.hasType`). The bench opens `Semantics.Macros`; it does not open syntax
  namespaces whose constructors would shadow core names.
- Syntax is unindexed. Contexts and inferred attributes are explicit inputs
  and results of the checker. Sort-specific constructors retain the
  distinctions needed to check their fields.
- Types derive `Repr` and `BEq`, and `DecidableEq` where derivation supports
  their recursive shape. Rules-bearing docstrings use bracketed Comprehensive Rules citations.
