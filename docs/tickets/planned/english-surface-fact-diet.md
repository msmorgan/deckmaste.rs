---
needs: [english-structural-recovery-zero]
---
**Delete the remaining stored-surface-fact fields ("lexical syntax cheats")
from the English AST.** Successor to the 2026-07-23 structure diet
(`RollRangeDash`, `SentenceEnding`), deferred out of that campaign.

- Casing flags: `Sentence.initial_uppercase`,
  `ActivatedAbility.effect_initial_uppercase`, and the quoted-ability
  `initial_uppercase`. Verified 2026-07-23 on the supported corpus: the
  sentence flag has one lowercase witness (Sphinx Summoner `. then shuffle`
  — already fixed in live oracle; stale snapshot datum) and the effect flag
  has one (Necratog `: this creature gets` — a live oracle typo; normalize
  the position, inert on errata); line-initial lowercase is zero. Quoted
  casing is structural, not lexical: quoted sentence abilities capitalize
  while quoted keyword lines (`"bands with other legendary creatures"`) and
  phrase references (`"legend rule"`) do not — derive it from the quoted
  node's kind. (Rejected alternative: an explicit casing-exception wrapper
  preserving Necratog's raw bytes — that mints structure from a single typo
  witness, and raw-typo fidelity is a non-goal: the gate compares in the
  normalized canonical-template domain.)
- Comma records: the `comma: bool` fields on `ClauseCoordination`,
  `PredicateObjectCoordination`, `ClauseAttachment`, and
  `DependentAttachment`. Determine per field whether the comma is derivable
  from list arity and attachment position; delete where the corpus proves
  the derivation exact, keep (documented) only where two supported faces
  genuinely differ on the bit alone.
- Contraction records: `first_auxiliary_contracted_with_subject`,
  `Copula.contracted_with_subject`, and `ExistentialForm::ContractedIs`.
  NOT trivially derivable — measured 2026-07-23 on the supported corpus:
  `it's` 1132 vs `it is` 73, `there's` 9 vs `there is` 67 (opposite
  majorities), `you're` 67 vs `you are` 1. Refined hypothesis to test:
  contraction is a function of construction kind (copular and subordinate
  positions contract; passive `it is` and sentence-initial existential
  `There is` do not). Negation contraction needs no storage at all
  (`cannot` / `does not` / `is not`: zero supported occurrences).
- `IndefiniteArticle` (a/an): candidate-derivable from the following word's
  initial sound; the renderer already carries initial-sound logic from the
  `non-` polarity work. Witness test: any same-sound minimal pair.
- `ComparativeWord`: direction (`more`/`greater` vs `fewer`/`less`) is
  semantic and stays. Only the within-direction lexical choice is
  candidate-derivable from the head's class — and the count/mass story is
  already dubious (`5 or more damage`, `X or more life` put `more` on mass
  heads). Requires a minimal-pair search; interacts with the
  `AdjectiveComparison` metadata machinery.
- Then the systematic sweep: enumerate EVERY AST field populated from a
  surface observation (casing, spacing, punctuation, glyph records) and
  witness-count each — how many supported faces differ solely on that bit;
  classify the witnesses as language / typo / stale datum and treat
  accordingly.

The round-trip gate is the decision procedure throughout: delete, derive,
run `--require-clean`; a failure names the exact counterexample. Every
deletion is representation-only (recovery census byte-identical). Standard
constraints apply.
