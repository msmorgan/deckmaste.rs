# lean — a Lean 4 port of the Idris workbench

A port of `../idris/src/Experimental.idr` and its family to Lean 4, as **syntax
first**: the grammar's constructors as plain inductives, with the checkers
(anaphora, zone coherence, obligations) to be re-added as functions over that
syntax later. This is the ordinary Lean shape — `Lean.Expr` is untyped syntax
and the type checker is a function — and it is closer to "check the RON" than
the Idris model was: the AST here is the RON's shape.

    lake build          # everything (./scripts/build adds --wfail)

## Status

Ported: the six grammar layers as syntax (`Words`, `Events`, `Phrase`,
`Triggers`, `Effect`, `Card`), the checker for all of them (`Check/*`, every
obligation `Experimental/*.idr` put in a constructor type), the subset of
`Macros` the bench and the pin suites use, a `Cards` bench of seven cards,
and the `ProofsDescription` pin suite. The other thirteen `Proofs<Family>`
suites follow the same recipe.

## The checker

The Idris grammar is inductive-recursive: constructor types call functions
(`nomIntro`, `detOk`, `zonesOk`, `instrIntro`) defined over the types being
declared. Lean has no induction-recursion, so the port splits every layer:

1. **Syntax** (`Experimental/*.lean`): the constructors, unindexed.
2. **Attributes** (`Check/Words`, `Check/Phrase`, `Check/Triggers`,
   `Check/Effect`): every function the Idris declared beside its syntax —
   the antecedent stack (`Bindings`), what a phrase introduces
   (`Noun.delta`, `Amount.intro`, `Instruction.profile`), its kind, number,
   zone, and type. They take the stack as an argument and pass the shifted
   stack into each field the Idris typed at the shifted index.
3. **Rules** (`Check/PhraseRules`, `Check/Triggers`, `Check/EffectRules`,
   `Check/Card`): one clause per Idris `{auto 0 … : …}` obligation, as a
   `Refusal` named after it. A check returns *every* refusal, not the first.
4. **Entry point**: `Card.check : Card → List Refusal`, and `Spelled`, a card
   with `card.check = []`. `spelled <| card …` finds that proof by `decide`,
   so writing a card runs the checker the way writing one ran the Idris
   elaborator.

`Kind` is inferred rather than indexed: `Predicate.kind?`/`Noun.kind?` give
the kind a phrase fixes, and a context that expects a kind refuses a
`kindMismatch`.

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
unchanged; `Proofs/Description.lean` is `ProofsDescription.idr` clause for
clause, and runs in about two seconds.

## Structural recursion is load-bearing

`decide` asks the kernel to evaluate the checker, and the kernel cannot unfold
well-founded recursion. Lean falls back to well-founded recursion *silently*,
so every recursive definition in `Check/*` carries `termination_by structural`
and a build error, not a slow proof, is what a non-structural definition
produces. The traps, all hit once:

- a call on the *same* argument (`sliceTy d g` calling `Noun.ty g`): pass the
  computed attribute in instead (`sliceTyOf d (Noun.ty bs g)`);
- a projection of a structure argument matched as a variable (`t.abilities`):
  match the constructor (`⟨pt, _, _, abilities, _, quals⟩`);
- a standalone function recursing through a nested list (`flatten`): make it
  a `mutual` pair with the per-element function.

`#print axioms f` showing `Quot.sound` means `f` fell back.

## Conventions

- Everything lives in `namespace Mtg` (Idris `Subtype` would otherwise collide
  with Lean's).
- Types are UpperCamelCase; constructors and functions lowerCamelCase in the
  type's namespace. The Idris anti-collision suffixes are gone:
  `PlayerW` → `NounWord.player`, `TapC` → `StatusCat.tap`,
  `OwnerAx` → `PossessorAxis.owner`, `ObjectP` is not ported. Write `.player`
  where the expected type is known, `NounWord.player` otherwise. Never `open`
  the syntax namespaces: `Predicate.and`/`.or`/`.not` would shadow core.
- Lean keywords as constructor names are renamed rather than escaped:
  `TriggerWord.At` → `atTime`, `Condition.Exists` → `thereIs`, `Cost.Do` →
  `action`, `Instruction.If` → `ifThen`, `Instruction.Repeat` →
  `repeatProcess`, `Macros.unless` → `unlessPays`. Fields named `by`, `from`,
  `as`, `while` get a trailing underscore.
- No indices. The Idris `bs` (antecedent stack) and `Kind` indices are gone;
  they come back as functions over the syntax with the checkers. Idris families
  indexed only to pick constructors (`StatusVal : StatusCat → Type`,
  `ChoiceDomain : ChoiceSort → Type`) are flat, with a function
  (`Status.category`) recovering the index.
- In the syntax, every `{auto 0 … : …}` obligation is dropped (it becomes a
  rule in `Check/*`); every explicit field is kept,
  same order, same name where the name was not a suffix. One deliberate
  reshaping: supertypes live on `TypeLine` [CR#205.1] instead of beside it in
  `Characteristics`, `TokenChars`, and `Card.sharedLineSplit`.
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
| `Unspellable T (\ok => term)` … `Oh impossible` | `theorem bad : X.check … term = [.reason] := by decide` |
| a twin `ok… : T = term` | `theorem ok… : X.check … term = [] := by decide` |
| a bench card | `def c : Spelled := spelled <| card …` |
