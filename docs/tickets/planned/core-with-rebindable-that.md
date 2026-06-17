---
needs: [engine-replacements]
---
`Effect::With(selection, effect)` — a binding combinator that resolves
`selection`, binds the result to the **rebindable `That`** (`ThatObject`/
`ThatPlayer`), and runs `effect` with it in scope; distributive when the
selection is plural ("those creatures *each* deal damage to `Target(0)`").
Principle (settled with the user): **`This` always denotes the ability's
source and never rebinds** — JavaScript-style moving `this` is banned. The
moving role is `That`.

Shape (user-given):

```
With(
    selection: Choose(2, AllOf([ControlledBy(You), Creature])),
    effect: Targeted(targets: [Player], effect: DealDamage(That, Target(0))),
)
```

Scope:
- Add `Effect::With { selection: Selection, effect: Box<Effect> }` (core) +
  its resolver (bind `That`, run `effect`, distribute over a plural binding).
- **Retire `Selection::Each` and `Effect::ForEach`** in favor of `With`
  (iteration leaves the selection layer for an explicit effect-level binder;
  `ForEach(over: F, …)` ≡ `With(selection: Filter(F), …)`). Migrate the ~7
  existing `Each`/`ForEach` call sites (`rg 'Selection::Each|Effect::ForEach'`).
- **Generalize the regeneration-shield `That`-capture** built in
  `engine-replacements`: a floating `ReplacementInstance` already remembers its
  `subject` as the captured `That` and its body reads `ThatObject`; rework
  `Regenerate` (and other "the next time …" shields) to express that capture
  through `With` rather than the bespoke `CreateReplacement.subject` field.

Open questions to settle first:
- **Multiple simultaneous `That`s?** Find effects that need more than one live
  binding at once (nested `With`, "exchange A and B", "for each X, for each
  Y"). If any exist, a single rebindable `That` slot is insufficient and
  named bindings are required — that is [[engine-bound-references]]
  (`Reference::Bound(Ident)`, today a `todo!` in `eval_reference`). If none do,
  the single `That` subsumes `Each`/`ForEach`/`ThatObject` as the user expects.
- **Align with `Filter::Where(Condition)` / `Subject`** (the user's unmerged
  precedent: `Where` binds a `Subject` reference for condition-scoped
  selections). `With`/`That` and `Where`/`Subject` should share one binding
  model, not two.
