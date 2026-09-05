# lean — Semantics and English workbenches in Lean 4

`English` is the Oracle English grammar design model beside `Semantics`.
It uses grammatical and realization relations, with checked structural and
surface witnesses; it does not require an executable parser or renderer.
See the [grammar design](../docs/english-grammar-design.md) for scope and open
decisions. Run `./scripts/build English` for its warning-free gate. The default
build includes both workbenches. The remaining sections describe `Semantics`
and its port-specific conventions.

The semantics-target grammar as a Lean 4 workbench, begun as a port of
`../idris/src/Experimental.idr` and its family: the grammar's constructors as
plain inductives, and the checkers (anaphora, zone coherence, obligations) as
functions over that syntax. This is the ordinary Lean shape — `Lean.Expr` is
untyped syntax and the type checker is a function — and it is closer to "check
the RON" than the Idris model was: the AST here is the RON's shape. The Idris
remains the reference for *which* obligations must hold; the Lean is shaped on
its own merits.

    lake build          # everything (./scripts/build adds --wfail)

## Status

Ported: the six grammar layers as syntax (`Words`, `Events`, `Phrase`,
`Triggers`, `Abilities`, `Card`), the checker for all of them (`Check/*`, every
obligation `Experimental/*.idr` put in a constructor type), the subset of
`Macros` the bench and the pin suites use, the whole `Cards` bench (see
[Cards](#cards)), and all fourteen `Proofs.<Family>` pin suites as `decide`
theorems (`theorem okX : … = []`, `theorem badX : … = [.reason]`), with the
Idris names and sentences kept. Three Idris pins are not ported, each named in
its module docstring: two Planechase sentences and one Idris type error.

Since the port the syntax has been reshaped (2026-09-04): a constructor stays
only if the checker attaches something to it that its expansion would not
carry, and the expansion is what the sentence means. So "permanent",
"colorless", "historic", "spell", "attacking", and "regenerates" are macros
over `inZone`, `colorCount`, `or`, `inCombat`, and `verbedEvent`; the combat
predicates and events are one `inCombat`/`combat` each over a
`CombatRelation`; `statOf` takes a `ProjAxis`; `arith` takes an `ArithOp`;
`or` joins kinds ("creature or player") where `joined` used to; designations
are labels (`DesignationLabel := String`) whose facts live in a checker table
(`Check/Words`), so a keyword's expansion can bring its own; and
`Characteristics` is flat and is exactly [CR#109.3]'s list, with
`power`/`toughness`/`loyalty`/`defense : Option Amount` (`none` is the printed
`*` a characteristic-defining ability fills), wrapped by the two records that
carry what a role adds to it: a `CardFace` is a printed face — its
characteristics and the choices its text announces as it enters — and a
`CharacteristicBundle` is a characteristics set as an effect writes it, with
the token qualities an effect can add [CR#111.3]. Planechase and Archenemy are
not ported.

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

The Idris grammar is inductive-recursive: constructor types call functions
(`nomIntro`, `detOk`, `zonesOk`, `instrIntro`) defined over the types being
declared. Lean has no induction-recursion, so the port splits every layer:

1. **Syntax** (`Semantics/*.lean`): the constructors, unindexed.
2. **Attributes** (`Check/Words`, `Check/Phrase`, `Check/Triggers`,
   `Check/Abilities`): every function the Idris declared beside its syntax —
   the antecedent stack (`Bindings`), what a phrase introduces
   (`NounPhrase.introduced`, `Amount.intro`, `Instruction.profile`), its kind,
   number, zone, and type. They take the stack as an argument and pass the
   shifted stack into each field the Idris typed at the shifted index.
   Checker-only vocabulary (`CardClass`, `FaceSide`, the designation facts
   table) lives here too, not in the syntax.
3. **Rules** (`Check/PhraseRules`, `Check/Triggers`, `Check/AbilityRules`,
   `Check/Card`): one clause per Idris `{auto 0 … : …}` obligation, as a
   `Refusal` named after it. A check returns *every* refusal, not the first.
4. **Entry point**: `Card.check : Card → List Refusal`, and `Spelled`, a card
   with `card.check = []`. `spelled <| .singleFaced { … }` finds that proof
   by `decide`, so writing a card runs the checker the way writing one ran
   the Idris elaborator.

`Kind` is inferred rather than indexed: `Predicate.kind?`/`NounPhrase.kind?`
give the kind a phrase fixes, and a context that expects a kind refuses a
`kindMismatch`. A disjunction whose disjuncts name different kinds is a
joined phrase ("a creature or player"), each disjunct checked at its own
kind.

The [checker contracts](CONTRACTS.md) record field contexts, enclosed scopes,
and the enforcing checks or replacements for the former datatype indices.

## Pins

A pin is a `decide` theorem naming the one refusal:

    /-- "a creature two target opponents control" -/
    theorem badControlledByGroup :
        Predicate.check .object []
          (.hasPossessor .controller (.described (.target (exactly 2)) .opponent))
          = [.soleHolder] := by decide

and its twin is the same statement `= []`. Because a check lists every
refusal, `= [r]` states that `r` is the *only* obligation failing, which is
what a non-vacuous Idris pin claimed by elaborating the rest of the term. The
VERIFY.md discipline (twin beside pin, same constructor at the same slot) is
unchanged, and every suite keeps the Idris names and sentences: `Proofs/<Family>.lean`
is `Proofs/<Family>.idr` clause for clause. `native_decide` is the lever for a
suite that outgrows `decide`; none has: the largest, `Proofs/Faces` (129
theorems), builds in about three seconds.

## Cards

`Cards/<Family>.lean` is `Experimental/Cards/<Family>.idr` card for card: the
fifteen families hold the 815 printed cards of the Idris bench (2026-09-05),
each a `Spelled` — `spelled <| .singleFaced { characteristics := { … } }`
finds `card.check = []` by `decide` at the definition, so a card that stops
checking stops defining, as it stopped elaborating in Idris — and every
phrase-level bench item beside its card as a plain definition with an
`ok…` theorem. The Idris identifiers and oracle-text docstrings are kept. The
`SemanticsCards` library holds them so `lake build Semantics` stays the inner
loop; `scripts/build` builds all three Semantics libraries and English.

Not ported, each named in its family's module docstring: the Planechase items
(`ichorElixirPlanarDice`, `fracturedPowerstonePlanarRoll`,
`missyChaosBranch`) and the two printed-`*` boxes (`shapeshifterBox`,
`tarmogoyfBox`: the slot is `none`, the characteristic-defining ability sets
it).

A card that refuses in Lean is a missing macro, a missing construct, or a
checker regression, never a card to drop. The port surfaced no missing
constructs, 184 macros (all ports of `Macros.idr` names), and eight
checker readings that had flattened an Idris obligation, each restored to the
Idris reading with every pin verdict unchanged: event bindings threaded
through casts, combat, attachment, damage, targeting, activation, payment,
mana, door and attack-with events; `getsPt` reading "it" through the Idris
`itsOther` window; `StaticSpec.definedSlots` descending into `andAlso`; the
box law no longer demanding power and toughness printed together; a joined
disjunction seeding one half per kind (`Predicate.seedTy` on `.or`), so
"target player or planeswalker" reads as a planeswalker and "any target"
stays untyped; `GameEvent.intro`/`.after` reading a caused event in the
causing's own bindings (`causingIntro`); and `agentRef` re-spelling an agent
that introduces no binding as itself.

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

- The library is `Semantics`: every grammar type lives directly in
  `namespace Semantics` (one grammar is one vocabulary, and Idris `Subtype`
  would otherwise collide with Lean's), and the definition collections take
  their module path as namespace: `Semantics.Macros`, `Semantics.Cards`,
  `Semantics.Proofs.<Family>`, with the pin shape in `Semantics.Proofs`.
- Types are UpperCamelCase; constructors and functions lowerCamelCase in the
  type's namespace. The Idris anti-collision suffixes are gone:
  `PlayerW` → `NounWord.player`, `TapC` → `StatusCat.tap`,
  `OwnerAx` → `PossessorAxis.owner`, `ObjectP` is not ported, and macro
  names carry no suffix either (`battlefield`, not `battlefieldZ`). Write
  `.player` where the expected type is known, `NounWord.player` otherwise.
  Never `open` the syntax namespaces: `Predicate.and`/`.or`/`.not` would
  shadow core. Field and argument names are descriptive words (`subject`,
  `amount`, `player`), never abbreviations.
- Lean keywords as names take a trailing underscore: `at_`, `if_`, `repeat_`,
  `exists_`, `unless_`, `until_`, `by_`, `from_`, `while_`, `end_`. The one
  exception is `Cost.perform` for the Idris `Do`, which is a word.
- A macro is written bare (`creature`), a constructor with a leading dot
  (`.hasType`); the bench opens `Semantics.Macros` and never qualifies a macro.
- No indices. The Idris `bs` (antecedent stack) and `Kind` indices are gone;
  they come back as functions over the syntax with the checkers. Idris families
  indexed only to pick constructors (`StatusVal : StatusCat → Type`,
  `ChoiceDomain : ChoiceSort → Type`) are flat, with a function
  (`Status.category`) recovering the index.
- In the syntax, every `{auto 0 … : …}` obligation is dropped (it becomes a
  rule in `Check/*`). There is no `TypeLine`: `Characteristics` carries
  `supertypes`, `types`, and `subtypes` flat, as [CR#109.3] lists them, and a
  frame that shares a line (`sharedLineSplit`) shares a `CardFace`. A frame
  that prints only part of a set names the part: a `LevelBand` is a range,
  text, and a power/toughness box [CR#711.2a,711.2b], and a `PrototypeFrame`
  is a mana cost and a power/toughness box [CR#718.1].
- Every type derives `Repr` and `BEq` (`DecidableEq` where it can: it does not
  derive for the nested mutual block in `Phrase`).
- `[CR#…]` citations carry over into docstrings.

## Idris → Lean phrasebook

| Idris | Lean |
| --- | --- |
| `%default total` | the default |
| `%unbound_implicits off` | `autoImplicit = false` (lakefile) |
| `import public` | plain `import` (Lean imports are transitive) |
| `public export` | the default |
| `data … where` | `inductive … where`, `deriving Repr, BEq` |
| `record … constructor MkFoo` | `structure Foo`, built with `⟨…⟩` or `{ … }` |
| `Maybe`/`Just`/`Nothing` | `Option`/`some`/`none` |
| `(a, b)` | `a × b` |
| `\|\|\|` docstring | `/-- … -/` |
| `k \/ k'` | `Kind.join k k'` |
| `Foo bs k` (indexed family) | `Foo` |
| `{auto 0 ok : So (f x)}` | a rule `refuse (f x) .reason` in `Check/*Rules` |
| `Unspellable T (\ok => term)` … `Oh impossible` | `theorem bad… : X.check … term = [.reason] := by decide` |
| a twin `ok… : T = term` | `theorem ok… : X.check … term = [] := by decide` |
| a bench card | `def c : Spelled := spelled <| .singleFaced { characteristics := { name := …, … } }` |
