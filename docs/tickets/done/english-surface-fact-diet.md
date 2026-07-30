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
  `PredicateObjectCoordination`, `ClauseAttachment`,
  `DependentAttachment`, and `ExceptionConjunct` (added 2026-07-23 by
  the copy-exception round; its Oxford lists make the arity-derivation
  question concrete). Determine per field whether the comma is derivable
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
- `QuotedAbility.terminal_period` (added 2026-07-23 by the
  quoted-coordination round): whether the interior's period sits inside
  the quote. Hypothesis: derivable from position — the quote closes its
  host sentence ⟺ the period is interior. If the corpus proves the
  derivation exact, delete the field and derive at render.
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

## Completion

**14 fields deleted, 11 kept with their refutations recorded on the field, 3
split out as parser-defect tickets**, plus one dead variant pair removed.

### Method correction

This ticket's stated procedure — "delete, derive, run `--require-clean`" — has
the steps in the wrong order, and following it literally wastes a round. A
*stored* bit round-trips clean no matter what it holds, because the renderer
replays it. The procedure that works is to **swap the derivation into the
renderer with the field still in place and populated**, then run
`roundtrip --list`: every face where derived and stored disagree mismatches, and
the gate names it. Only then delete. Every measurement below was taken that way.

The corollary matters more than the mechanics: a disagreement does not mean the
derivation is wrong. It can equally mean the *tree* is wrong and the stored bit
was laundering it. Read the witnesses before concluding.

### Deleted

`QuotedAbility.terminal_period` — derived from position. The renderer already
walked each sentence's AST tail to decide the sentence period; that walk now
returns the terminal quote node instead of a bool, and the quote that recognizes
itself keeps its interior period.

`Sentence.initial_uppercase` and `ActivatedAbility.effect_initial_uppercase` —
position decides. The two lowercase witnesses are data defects, not language
(Sphinx Summoner's snapshot prints a period where the live card has a comma;
Necratog's prints a lowercase word after the cost colon), so
`normalize_sentence_case` rewrites the position at the input boundary and the
gate compares in the normalized domain — the move `normalize_roll_row_dashes`
already made for `RollRangeDash`. Both defects had additionally been baked into
the test suite as if they were language; those tests now assert the derived rule.

The stored `IndefiniteArticle` — derived from the renderer's existing
initial-sound machinery.

Five `comma` fields (`ModifierCoordination`, `AdjectivePhraseCoordination`,
`PredicateObjectCoordination`, `ExceptionConjunct`, `RestrictionCoordination`) —
`conjunction.is_none() || rest.len() >= 2`, exact. See
`english-derived-serial-comma`.

Five single-valued fields the earlier list never named, found by the sweep:
`QuotedAbility.closed` (hardcoded `true` at every construction site — a fossil
with a plausible doc comment), `KeywordCostTerminal`, both `KeywordCost`
separators, and `KeywordArgument::Recovered.separator`. Also removed:
`ModalFrame::Preamble` and `ModalPreambleSeparator`, zero construction sites.

### Kept, refutation documented on the field

`QuotedAbility.initial_uppercase` (Takklemaggot vs Master of the Hunt — same
structure, opposite casing); `CoordinationJunction.comma`,
`SetExceptionNounPhrase.comma`, `Attachment<T>.comma`; `ComparativeWord`
(minimal pair `total power 2 or more` vs `8 or greater`); the three contraction
records; `ModalHeaderSuffix`; `KeywordListSeparator`;
`KeywordArgument::Named.separator`.

Two findings worth carrying forward:

- **The contraction hypothesis in this ticket is falsified.** Construction kind
  does not predict contraction (161 mismatches / 731 render errors for the
  voice-based rule; 1617 / 162 for unconditional copular). The witnesses show
  why — `this creature is attacking`, `power and toughness are each equal to`,
  `X is the greatest toughness` all have full noun-phrase subjects. The real
  driver looks like **subject pronominality**: only pronoun subjects contract.
  That is not exact either (`it's` 1132 vs `it is` 73), so the bits stay, but
  the next attempt should start there rather than from construction kind.
  Negation is confirmed to need no storage: zero occurrences of `cannot` /
  `does not` / `do not` / `is not` / `did not` / `are not` across all 31,685
  supported faces.
- **Corpus-clean is not the same as correct.** A per-keyword rule for
  `KeywordArgument::Named.separator` (`Partner` and `Modular`) reaches zero
  mismatches on the corpus, but the grammar has a test whose name asserts the
  shape is deliberately keyword-*independent*, with a fixture (`Partner a Food`)
  the rule would break. The corpus attests what was printed; the test encodes
  what the grammar is for. Where they disagree, the derivation is unsafe.

### Split out

`NounPhraseCoordination.comma` (39) and `NominalPhraseCoordination.comma` (1) to
`english-coordination-comma-defects` — deriving them exposes a wrong tree, not a
wrong rule: the Arrest/Pacifism aura family stores `can't attack or [block, and
its activated abilities]`, a verb conjoined with a noun phrase. Rendered English
is unaffected, so no fidelity gate ever failed, but every consumer of the tree
sees it. `ClauseCoordination.comma` (281) to
`english-clause-coordination-comma-rule` — the count model inverts at
full-clause level.

### Coverage caveat

The systematic sweep enumerated all `pub struct` / `pub enum` definitions in
`syntax/{ability,clause,phrase}.rs` exhaustively (13 fields/families).
**`word.rs`, `word/*.rs` and `numeral.rs` were not enumerated** — they sit
outside `syntax/` and were only sampled. `Numeral` (digit vs word) and
`RelativeMarker` were traced to their population mechanism but **not measured**;
no derivation was attempted for either, and `RelativeMarker`'s animacy
hypothesis was never taken to a corpus count. Those three are the known residue.

Round-trip 31685/31685 clean, 2937 tests passing across 50 binaries, recovery
census byte-identical at every step, fidelity 0 failing.
