---
needs: []
---
# Probe what `spelled` accepts, then cut authoring boilerplate it allows

Standard constraints apply. Load the `lean4:*` skills and the Lean LSP tools before the probe.

## Governing principle (owner, 2026-10-21)

- Printed wording lives on Templates, which carry their spellings as text templates in the DSL
  this workbench prototypes. It does not live in Core AST nodes (the semantic expression types in
  `Words.lean`, `Events.lean` and `Abilities.lean`) or in Deed stamps.
- A distinction belongs in the Core AST only if the CR or the checker's anaphora resolution
  distinguishes it.
- Separate Templates keep distinct printed wordings even when they expand to the same tree.

Boilerplate is neither wording nor rules meaning. It is noise the authored form carries for Lean's
sake. This ticket only removes it; no tree changes.

## Ruling (owner, 2026-10-21)

1. **Probe first.** Write a Lean probe that establishes what `spelled` (`Check/Card.lean:390-417`)
   and `Authoring.inspect` (`Authoring.lean:427`) accept, for each of:
   - **`Option` coercions.** Core's `α → Option α` coercion lets `untilEndOfTurn` stand for
     `(some untilEndOfTurn)`. A leading-dot name cannot use it: `.combat` resolves against
     `Option`'s namespace before any coercion applies, so `some .combat` stays required (recorded
     in [lean-v3-deal-damage-kind](../done/lean-v3-deal-damage-kind.md), "Spelling of a qualified
     kind").
   - **`Coe` instances between semantic sorts:** `List ManaSymbol` (`ManaCost`) → `Cost` (for
     `mana [...]`) and `Action` → `Cost` (for `perform (...)`).
   - **An `OfNat Amount` instance**, preferred over `Coe Nat Amount`, so authors write `3` for
     `.lit 3`.

   For each, record two things. First, whether `inspect` accepts the elaborated term or refuses it
   as raw or concealed semantic syntax. Second, whether the retained `Authoring.Form` still names
   the same Templates and literals.
2. **Then reduce only what the probe shows is safe**, from the targets below.
3. **If the probe shows coercions are refused**, or their measured cost is real, this ticket says
   so in its landing and narrows to defaulted parameters and renames.

## Coercions against authoring-surface Principle 3 (owner ruling, 2026-10-21)

[lean-v3-authoring-surface](lean-v3-authoring-surface.md) Governing Principle 3 says literal and
enum coercions belong to the Rust DSL, and that "Lean retains explicit `.lit`". Its reason is
elaboration and `by decide` cost: typeclass search, lost expected-type propagation in list
literals, and projection unfolding in `Authoring.inspect` and the kernel.

**Ruling.** Run the probe and measure that cost. Principle 3 wins only where the cost is real.
- Measure the domain `Coe` instances separately from `OfNat Amount`. The domain instances are
  `Subtype`/`CardType` and the other leaf constants → `Predicate`, `List ManaSymbol` → `Cost`,
  and `Action` → `Cost`.
- For each, time elaboration (`lake build SemanticsCards`) and the kernel check of a few large
  `spelled` cards, before and after, on the same machine.
- Each instance's sweep goes ahead only where its measured cost is negligible.
- Amend Principle 3 only once the probe's result is in. The amendment states which coercions Lean
  now takes and cites the measurements.

**Pointers, checked 2026-10-21 (the ruling's lead, corrected where it differs).**
- `spelled`'s elaborator (`Check/Card.lean:401-417`) calls `Authoring.inspect` at `:404`.
  `inspect` itself is in `Authoring.lean:427`. It has no `Coe` handling of its own; it reads the
  term after `instantiateMVars` (`Authoring.lean:348`). A `Coe` coercion is expanded by Lean's
  elaborator before `inspect` sees the term, so a domain coercion should reach `inspect` as the
  ordinary Template call it unfolds to. The probe confirms this. Seven Template-valued `Coe`
  instances already pass `spelled`: `NounPhrase → Subject` (`Templates.lean:143`),
  `Ability → KeywordTerm` (`:491`), the two `Duration` lookback coercions (`:2084,2086`),
  `CounterKind → CounterKindSource` (`:2129`), `String → NameSource` (`:2208`) and
  `Condition → AsThough` (`:2264`). They are the baseline the new instances are measured against.
- A numeral is different. It elaborates to `OfNat.ofNat` applied to the instance, which nothing
  unfolds before `inspect`. Whether `inspect` accepts it, or needs the instance registered as an
  authored declaration, is the probe's first `OfNat` question.
- `Card.check`'s refusals are computed with `evalExpr` (`Check/Card.lean:411`). The `ok` field is
  still a kernel `decide` proof (`mkDecideProof valid`, `:416`), and so is `onlyAuthored`
  (`:409`). The kernel cost is not zero. An expanded coercion leaves the kernel term the same as
  the hand-written form, so its cost should be in elaboration, which is what to measure.

## Targets (counted 2026-10-21; `lean/Semantics/Cards/` unless noted)

- **`(some …)` arguments.** 394 in Cards, 204 in Definitions, 101 in `Templates.lean`. A
  leading-dot argument keeps its `some`.
- **Cost wrappers.** 235 `mana [` and 91 `perform (` in Cards. 111 of them (70 `mana`, 41
  `perform`) are members of a `compound [...]`, where a coercion would let the member be written
  bare.
- **`.lit` literals.** 1,174 `.lit` in Cards (1,152 of them `(.lit N)`), 91 in Definitions, 31 in
  `Templates.lean`, 933 in Proofs.
- **Card envelopes.** Every `spelled <| .singleFaced { characteristics := { name := …, cost :=
  some […], types := […], text := […] } }` repeats the same record nesting. A defaulted card
  Template could drop it, provided `spelled` still inspects every field.
- **The `atLeast` collision.** Three spellings name three different things:
  - the `Quantity` Template `atLeast` (`Templates.lean:158`, "N or more"), 40 bare uses in Cards;
  - `Comparator.atLeast` (`Words.lean:51`), 44 `.atLeast` uses in Cards;
  - `LevelRange.atLeast` (`Card.lean:25`), 3 uses: `levelBand` `Cards/Faces.lean:572,582,591`.

  Rename so each name means one thing. This needs no coercion, so it lands whatever the probe
  shows. Propose the names in the landing. A rename changes no tree.

## Overlaps

This ticket touches nearly every card file. Land it after the other lean-v3 cleanup tickets
(the move, damage, untap, trigger and AST-merge tickets) to keep the rebases mechanical.
[lean-v3-proofs-and-docs-alignment](lean-v3-proofs-and-docs-alignment.md) rewrites proofs too.
Whichever lands second rebases.

## Tests

- The probe's results are pinned as theorems or `#guard`s in `Proofs/Authoring.lean`. Each
  accepted form is checked through `spelled`, and each refused form through the elaborator error,
  using the existing refusal-pin pattern.
- After the sweep, every `ok…` theorem and every `spelled` card stays green under
  `./lean/scripts/build` (`--wfail`).
- The `Authoring.Form` of a swept card names the same Templates and literals as before the sweep.
  Pin that for one card from each swept target.
