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
  node's kind.
- Comma records: the `comma: bool` fields on `ClauseCoordination`,
  `PredicateObjectCoordination`, `ClauseAttachment`, and
  `DependentAttachment`. Determine per field whether the comma is derivable
  from list arity and attachment position; delete where the corpus proves
  the derivation exact, keep (documented) only where two supported faces
  genuinely differ on the bit alone.
- Then the systematic sweep: enumerate EVERY AST field populated from a
  surface observation (casing, spacing, punctuation, glyph records) and
  witness-count each — how many supported faces differ solely on that bit;
  classify the witnesses as language / typo / stale datum and treat
  accordingly.

The round-trip gate is the decision procedure throughout: delete, derive,
run `--require-clean`; a failure names the exact counterexample. Every
deletion is representation-only (recovery census byte-identical). Standard
constraints apply.
