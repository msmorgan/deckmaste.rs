---
needs: []
---
# Re-align the effect basis and the verb vocabulary, or record why the divergence pays

The one axis the v1/v2 comparison graded **DIFFERENT_FOR_NO_REASON**. Two
questions, one region: how effects are split into types, and whether the
keyword-action verb name is a closed enum or a declared atom.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 11 (2026-08-24). Standing authority on the effect boundary:
[effect-atom-independence.md](../../decisions/effect-atom-independence.md).
Delta only: the comparison found the tag idea, its well-formedness check and its
rules rationale identical on both sides, and located the divergence in the verb
vocabulary rather than in the constructor basis.

## From the v1 comparison (2026-08-24)

> **Crate.** Two types: `OneShotEffect` (23 variants, `effect.rs:45`) for
> wrappers/frames and `Action` (41 variants, `action.rs:148`) for verbs, joined
> by `OneShotEffect::Act(Action)`. Keyword actions are tagged:
> `Action::Composite { name: VerbName, body: Arc<OneShotEffect> }`
> (`action.rs:287`).
>
> **Workbench.** One type: `Effect : Bindings -> Type`, 53 constructors
> (`Experimental.idr:3208`), verbs and frames together. Tagging is split in two:
> `Composite : (v : VerbName) -> (e : Effect bs) -> {auto 0 ok : TagBody v e} ->
> {auto 0 na : NonAgentive v}` and `Does : (subj : Noun bs Player) -> (v :
> VerbName) -> (e : Effect (nomIntro subj)) -> {auto 0 tb : TagBody v e}`
> (`Experimental.idr:3347,3349`).
>
> **Assessment.** The tag idea, the `TagBody` well-formedness check […] are
> identical on both sides […]. The agentive/non-agentive split (`Does` vs
> `Composite`) is a genuine addition and matches `semantics-v2.md:§7`'s
> agentive-verb rule. The one-type-vs-two split is a wash: `Act(Action)` is pure
> noise in the crate, but the crate's boundary is what
> `docs/decisions/effect-atom-independence.md` names. Where the two really
> diverge with no payoff is the **verb vocabulary**: the crate's `VerbName` is
> an open `Ident` newtype (`event.rs:171`) gated by entailment-table membership,
> so a new keyword action is a data row; the workbench's is a closed 8-member
> enum `Destroy | Sacrifice | Exile | Discard | Mill | Scry | Surveil | Put`
> (`Words.idr:688`) with three total tables keyed on it (`verbAgentive`,
> `verbMoves`, `Eq VerbName`). That is a divergence from an already-solved
> extensibility problem […]. Candidate to re-align: make `VerbName` a declared
> atom.

Both sides verified in tree: `Words.idr:686-688` is the closed eight, and
`event.rs:163-171` documents the crate's own trade explicitly — "typo-safety is
no longer the type's job (any bareword parses) but the GATE's".

## The deliverable

Either re-align, or write the divergence down as intended. Not both, and not
silence.

- **The verb vocabulary.** Decide whether `VerbName` becomes a declared atom
  with a membership gate (the crate's shape, the shape `semantics-v2.md:§6` says
  the macro layer should eventually mirror), or stays a closed enum with the
  totality tables re-decided per addition. If it stays closed, say what the
  closing rule is — the eight are not rules-closed the way `CardType` is, so
  "the corpus attests eight" is a count and not a rule, and the reason has to be
  written where `VerbName` is defined.
- **The effect basis.** The one-type/two-type split is a wash on its own, but
  `effect-atom-independence.md` names the crate's boundary. Say whether v2's
  single `Effect` type owes that ADR anything at lowering time, or whether the
  ADR's scope to *engine* atoms already settles it. The comparison's own
  adversarial note on axis 1 flags the same seam.
- **Do not** copy the crate's `Act(Action)` join back; the report calls it pure
  noise and nothing here argues for it.

## Sibling case, owned elsewhere

`Keyword` is the same closed-enum-vs-declaration question at a second site (33
hand-maintained members against the crate's 63 declared `.ron` macros, comparison
axis 15), and `semantics-v2.md:§6` already records it as a deliberate stand-in.
Its residues belong to
[workbench-keyword-parameters-and-attachment](workbench-keyword-parameters-and-attachment.md).
Whatever this round decides for `VerbName` should be the same decision, or the
difference should be stated.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental/Words.idr` (`VerbName`,
`verbAgentive`, `verbMoves`, the `Eq` instance),
`idris/src/Experimental.idr` (`Effect`, `Composite`, `Does`, `TagBody`,
`NonAgentive`, and every table keyed on `VerbName`),
`idris/src/Experimental/Macros.idr` (the verbs' spelling side), the pin modules
`idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate — this is a decision about the
workbench's own shape, not a port.

## Acceptance

- The verb-vocabulary verdict is recorded where `VerbName` is defined, with its
  reason; a closed enum that survives says what closes it.
- If the atom lands, the three total tables become data keyed by the atom and no
  new addition requires re-deciding totality.
- `TagBody`, `NonAgentive` and the `Does`/`Composite` split are preserved
  whichever way the vocabulary goes — the agentive rule is the genuine addition
  and is not traded away for the crate's shape.
- The effect basis question is answered against `effect-atom-independence.md`
  by name; `Act(Action)` is not reintroduced.
- The `Keyword` sibling gets the same answer or an explicit difference.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
