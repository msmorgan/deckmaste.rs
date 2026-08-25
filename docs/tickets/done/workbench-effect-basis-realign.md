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

## As-landed

Decision round; no vocabulary and no bench lines were minted. Two docstrings
carry the verdicts, one at each site the ticket names.

### The verb vocabulary: `VerbName` stays a closed enum, bounded by [CR#701]

Recorded at `Experimental/Words.idr` on `data VerbName`.

The round opened by testing whether [CR#701] — the CR's own closed enumeration
of keyword actions — is the closing rule, the way [CR#205.2a] closes `CardType`
and [CR#207.2c] closes `AbilityWordName`. **It is the bound, not the
population, and it does not cover all eight.**

- Seven members are keyword actions [CR#701] defines:
  [CR#701.8,701.9,701.13,701.17,701.21,701.22,701.25].
- `Put` is not a keyword action, and the CR says so itself: [CR#701.1] holds
  that a verb the rules do not keyword uses "the standard English definition".
  The tree already shows the consequence — `PutB` is the one `TagBody` arm that
  constrains nothing, admitting every `Move`, where every other arm states its
  rule's expansion.

So the steer's mechanism holds with one amendment. [CR#701] bounds what may
ever be a member; it cannot be *ported* the way the two precedent catalogs are,
because a card type and an ability word carry names only ([CR#207.2c]: "no
special rules meaning and no individual entries") while a keyword action
carries its expansion, and `TagBody` states that expansion as a type. That is
also why the crate's shape buys nothing here: an atom plus a
`So (isKnownVerb …)` gate purchases "a new keyword action is a data row", and
on this side a row can never be data — its expansion is rules content and is
not derivable. What it would spend is real: `Eq`-based `TagBody` dispatch, and
the coverage the compiler checks on four total tables rather than three
(`verbAgentive`, `verbMoves`, `verbedMarkingOk`, `Eq VerbName` — the comparison
missed `verbedMarkingOk`), each of which would become a partial lookup needing
a fallback the `{default` ban forbids.

**Extension story (the three-plus-one total tables).** Adding a verb is one
`TagBody` arm stating the rule's expansion, plus the row each of the four
tables demands. Nothing re-decides totality *policy* — the compiler names the
missing rows by construction, which is the property the atom would trade away,
not a tax the atom would remove.

Nothing minted this round. The ledgers' named verb blockers — transform
(`workbench-multiface-cards`), search
(`workbench-pins-refuse-rules-impossibility-only`), coin flip
(`workbench-cost-and-payment-residues`) — are
not cheap 701 rows: each needs a new `Effect` body before a tag has anything to
name, and coin flip is not a keyword action at all. The extension story covers
them.

`TagBody`, `NonAgentive` and the `Does`/`Composite` split are untouched.

### The effect basis: the ADR's scope settles it, and owns one lowering rule

Recorded at `Experimental.idr` on `data Effect`.

`docs/decisions/effect-atom-independence.md` is scoped to *engine* atoms — one
"depends only on its literal arguments and explicitly bound references" and
"does not infer meaning from its parent". On that scope the one-type/two-type
count is not an ADR question, and the `Bindings` index is the ADR's
explicit-reference channel promoted to a type index, so a context-reading
constructor here is the sanctioned form rather than an exception to it.

The ADR does own one obligation at lowering: `Composite` and `Does` must emit
their **body** as the atom and the verb as a name, never an atom that reads its
tag to learn what it does — that is the parent-inference the ADR forbids.
`TagBody` makes the obligation free, since the body is already the rule's whole
expansion and the tag is discardable. `Act(Action)` is not reintroduced.

### The `Keyword` sibling: same verdict, and the reason does NOT transfer

Stated here only; the residues stay with
[workbench-keyword-parameters-and-attachment](workbench-keyword-parameters-and-attachment.md),
which this round did not touch.

`Keyword` stays closed too, and its closing rule is cleaner than `VerbName`'s:
[CR#702] is the CR's own enumeration of keyword abilities, with no `Put`-shaped
exception, so the steer's mechanism applies to `Keyword` *better* than to the
site it was proposed for.

But the argument that makes closure free for `VerbName` is absent here. No GADT
is indexed on `Keyword`; its tables (`keywordParamShape`, `keywordParamless`,
`keywordCounterOk`, `keywordStackRegime`, `keywordCardOk`) are flat data rows,
and a member carries no per-keyword typed obligation. `Keyword` is therefore
the genuine declaration candidate of the two — the crate's 63 `.ron` macros are
data because a keyword ability *can* be data — and whether to convert it is a
live question for the sibling ticket rather than one settled by this round.

### Gates

- `cd idris && scripts/build` — 19/19.
- `Experimental/Cards.idr` implicit binds `{x = …}`: **0**. `{default` across
  every `Experimental*` module: **0**.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 16,611 citations, **0 stale**.
- `cargo xtask cite bless` — 1,477 rules; `cr-citations.lock` unchanged (every
  rule cited was already registered).
- `jj diff --git | cargo xtask cite audit --diff` — every site read against the
  rule text: 6 in `Words.idr`, the rest restatements of the same rules in this
  ticket plus [CR#702]. [CR#701.1] is the load-bearing one and it argues *for*
  the claim: it is the rule that puts non-keyworded verbs outside the keyword
  set, which is exactly `Put`'s status.
- Bench: 435 cards, 441 pins — unchanged. Nothing minted, nothing lost.

### Ledger

- **`verbedMarkingOk`'s `Put` rows rest on a corpus observation.** The two
  `False` rows carry "No line marks a placement patient by the bare
  participle", which is a count, though the same comment also gives a
  structural reason (the marking names the destination too). Not a pin, so
  `measurements-live-in-pins.md` does not condemn it outright, but the row
  would read better on the structural half alone. Left as found; out of this
  round's scope.
- **`badSubjectlessPut` sits on the `Put` exception.** `ProofsG.idr` pins
  subjectless `Composite Put` as unspellable because the agentive names its
  subject. That pin is the one place the unmarked verb's oddity is already
  written down; if `Put` ever earns a rules-shaped justification beyond
  [CR#701.1]'s contrast, that pin is where to check it still holds.
- **`Keyword`'s conversion question is now sharp and unclaimed.** Per the
  section above, `Keyword` is the site where the crate's declared shape would
  actually buy something. It belongs to
  `workbench-keyword-parameters-and-attachment` and is stated, not taken.
