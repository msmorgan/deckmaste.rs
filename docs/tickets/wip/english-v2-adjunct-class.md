---
needs: [english-v2-genitive-possessor-licensing, english-v2-locative-licence-data]
---
One predicate-adjunct class (A3) consuming PPs and adverbials — temporal,
locative, manner, purpose, duration as values of one class. The at_phrase
temporal residue from the dissolution lands here as ordinary adjunct PPs.
Expected unlock: postposed "at the beginning of the next end step".

Baselines (locative re-issue review): 16,337 selected / 384 constructions /
collisions 60 / lock `c73055d0…`. Acceptance wording: lock diff -0 rows
(never "net").

Owns the review's HIGH: the per-X adjunct slots must be REPLACED by the one
class, not inherited — inventory them explicitly and delete each as the
general adjunct lands: `adjunct: opt ExistentialDomain` on
positive_object_gap_relative (guarded only by AdjunctCapable; beats the NP
postmodifier on specificity), the duration slot on the contracted-perfect
relative, the reduced-passive duration/manner slots,
`existential_finite_clause.domain`, and postscalar_prepositional_
qualified_noun_phrase's unexplained `is OnComplement` narrowing. Low
attachment is a CHECKED property, not emergent: Seedborn Muse's "Untap
all permanents you control during each other player's untap step." must
attach the PP to the predicate (untap … during …), never to the relative
clause — a probe with the winner pinned by structure, not by ordinal.
RULING for this ticket: `for each <NP>` is a predicate-level distributive
adjunct, never an NP postmodifier ("Draw a card for each creature you
control." attaches to draw). `UnrestrictedComplement` (for/after/before)
must stop licensing NP postmodification off Relationality (H2 residue) —
these prepositions attach only as predicate adjuncts. Census shift
quantified AND its mechanism narrated in the record.

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.

## Landing record

Measured on change `onpzrwsz` with 16,385 covered lock identities.

- Coverage: 16,337 -> 16,385 selected and covered units; parse failures
  16,304 -> 16,256; lock diff **+48/-0 rows**. The schema-4 lock retains source
  fingerprint `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and moved from SHA-256
  `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`
  to `25f3bd5f47db192653a09fd8834d86654515a9ae551b1b9e4ff0e327a706ae27`.
- Selection census: unique 10,843 -> 10,962; specificity-resolved 5,494 ->
  5,423; exception-resolved 0 -> 0; unresolved ties 0 -> 0; internal
  failures 0 -> 0; exception uses 0 -> 0. The +119 unique/-71
  specificity shift is the mechanism of the +48 selection gain: removing
  nominal `for`/`after`/`before` attachment and the per-adverbial predicate
  brackets eliminates lower-attachment rivals, while the shared adjunct
  admits new preposed and postposed predicate parses. Seedborn Muse's
  `control` relative is checked out as an adjunct host, leaving only the
  outer `untap ... during ...` structure. `for each` now follows the same
  predicate route, including above a cost comparison.
- Construction declarations: 384 -> 388. Literal/lexicon collisions: 60 ->
  59; deleting the private `for each` cost-basis spelling removes the one
  collision.
- Gates: `cargo fmt --all -- --check`; strict all-target clippy for
  `deckmaste_construction_core`, `deckmaste_english_v2`, and `xtask`;
  `cargo test -p deckmaste_english_v2`; `cargo xtask english_v2 ambiguity
  --require-resolved --json`; and post-bless `cargo xtask english_v2 coverage
  --check` all exited 0. The corpus gates emitted only their busy-host common-
  path performance warning. A foreground `cargo test --workspace` was also
  attempted; it reached the unchanged `deckmaste_construction` compiled-
  consumer fixture and failed because that parent-tree fixture lacks
  `ParserEnvironment::declaration_noun_features`. No file in that crate is in
  this change. The required Kata refresh produced the same failure on a second
  foreground attempt; the full workspace gate must be repeated after
  integration.
- Assurance census: restored 0; re-spelled 11 existing test functions;
  ignored 0; added 1; removed 0. The added structural test pins Seedborn
  Muse, the next-end-step `at` unlock, ordinary `for each`, and cost-comparison
  `for each` without ordinals.
- STOPs: none. The ambiguity gate remained at zero ties throughout the final
  measurement.

### Replaced inventory

- Deleted the purpose, duration, state-duration, manner, frequency, and
  prepositional predicate wrapper families. Their adverbial values now enter
  the single `PredicateAdjunct` sum, and their postposed hosts return the
  single `PredicateAdjunctPredicate` class.
- Replaced the two preposed-PP and two preposed-duration constructions with
  one clause and one predicate construction consuming `PredicateAdjunct`.
- Removed the positive object-gap relative's optional `ExistentialDomain`
  slot. A shared-adjunct relative host is a separate checked construction;
  the core `control` head is rejected there, which pins Seedborn's high
  attachment structurally rather than by candidate ordinal.
- Replaced the contracted-perfect duration, reduced-passive duration/manner,
  and existential domain slots with the shared adjunct value. Deleted
  `ExistentialDomain` and retained only an existential host adapter that
  checks the shared value is prepositional.
- Deleted `ForEachCostBasis` from the cost-comparison codec, core-verb frame,
  AST, and visitor. Cost comparisons accept `for each` only through the
  shared predicate-adjunct class.
- Deleted the postscalar `OnComplement` narrowing and made
  `UnrestrictedComplement` categorically false for nominal postmodification;
  `for`, `after`, and `before` are available only through predicate adjuncts.

### Deviations and additions

- Twelve old constructions were deleted and sixteen shared-value or
  shared-host constructions were added, for the disclosed net +4. The
  additional host adapters cover verb phrases, cost comparisons, action
  restrictions, alternative-payment predicates, stacked transitive
  duration+PP sequences, passive durations, existentials, and checked
  positive object-gap relatives. They are compatibility points into one
  adjunct class, not adverbial-specific wrapper families, and are required
  for the +48/-0 lock result.
- The design source was only the v1 taxonomy document named by the ticket;
  no v1 code, type, or feature vocabulary was reused.
