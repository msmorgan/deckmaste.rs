---
needs: []
---
# Open rulings from the child-name and Category-fission audit

**Needs an owner ruling before any implementation.** These are the rulings
`english-v4-child-name-control-and-category-fission-audit` left open. Its
Findings section has the evidence, line numbers and probe measurements (P1–P4)
this ticket refers to. Once the owner rules, each ruled fix becomes its own
ticket, or is implemented here if it is small.

## R1. Gates the Ranking Key cannot absorb (policy)

`docs/decisions/english-v4.md` §4.4 item 1 says that a Reading English grammar
admits is kept, and the Ranking Key orders it. Some gates keep out Readings
that English admits but that the structural Ranking Key cannot order. If such
a gate retires, the Readings either tie with the endorsed one or rank ahead of
it, and §4.3 forbids lexical plausibility as a tie-breaker.

The gates:
- PeriodNounPhrase and Determined's two `ExcludeTogether`s. Retiring them
  puts the Object Reading first (*can't block [this turn]*).
- Unmodified's `Exclude(head.ObliqueKind, Gerund)`. Probe P4 measured it: in
  about 25 units the gerund Reading ties the endorsed one in the cheapest
  tier.
- The `Manner` Excludes (NonNumericComplement, SubjectPredicate,
  RelativeSubjectPredicate).
- The Verb Valence's `internalised` Excludes of `Symbol` and `Quantity`.
- PresentClause.
- Possessor and PossessorNominals (phrasal genitives).
- The *non-*-only asyndetic premodifier chain (AttributeChain,
  NegativeAttribute and the three Negative* duplicates).
- RelativeTemporalNominal.
- The FreeChoice Excludes (Determined, DeterminedModifiedQuantity).

Options:
- (a) Keep each gate as an admission rule, with a comment stating the
  justification. §4.4 item 1 then reads "unless the Ranking Key cannot order
  it", and the ADR records that exception.
- (b) Retire each gate and accept cheapest-tier ties.
- (c) Add a structural Ranking Key component that orders these Readings
  without lexical plausibility.

**Recommendation: (a) for now, gate by gate.**
- Keep a gate only where a probe shows that retiring it loses a
  cheapest-tier singleton.
- Retire it (b) where the cheapest tier survives.
- Revisit (c) only if a general structural principle covers several of these
  gates. None is known yet.

## R2. Head renames

Three Productions name their semantic head something else, so that implicit
head agreement does not apply: DistributiveCoordinationDetermined (`nominal`),
TransparentQuantity (`quantity`) and TemporalQuantity (`quantity`). Their
Readings have no head lexeme, and they are outside the head line.

Options:
- (B-a) Keep the renames, with comments and the reserved-name contract
  (`english-v4-reserved-child-names-contract`).
- (B-b) Add a per-Production opt-out from head agreement, such as
  `unagreed: ["Number"]`, so the child can be named `head`. This is a new
  `ProductionDecl` field, which the freeze forbids even though it suppresses
  nothing. It would also let TargetingMarker, QuantifiedNominal,
  ModifiedQuantifiedNominal and perhaps PartitiveHead go.
- (B-c) A general rule: an explicit `Agree` of a mother feature overrides
  implicit head agreement on it. Probe P2 measured it. It loosens the
  partitives (+264 Readings in 19 units, singleton tiers 15,824 → 15,821), so
  it is not a refactor.

**Recommendation: B-a.** Reconsider B-b only if the owner relaxes the
`ProductionDecl` freeze for non-suppressing declarations.

## R3. SubjectInitialClause

The Category restates four Clause Productions: SubjectPredicate,
CliticSubjectPredicate, FloatedEach and FloatedAll. It also restates
AdjunctGap as RelativeAdjunctGap. It does this so that no preposed dependent
comes before a bare relative's Subject (CGEL, Ch. 12 §3.5.5, p. 1055).

It is over-broad. It also shuts out coordinated bare relatives: *each creature
you control and an opponent owns* has 0 Readings.

Options:
- (C-c) Move the four base Productions to lhs SubjectInitialClause and add one
  unary lift `Clause → SubjectInitialClause`. That removes 3 Productions and
  all duplication. Clause Readings gain a unary node.
- (C-a) Add `Coordination(category: "SubjectInitialClause", …)` as coverage.
  Probe P3 measured it: no face changes, and the new wrong Readings rank
  behind the endorsed ones.
- (C-b) Retire the Category, and key bare relatives on the existing
  `Summary::fronted` bit. That needs a new `ProductionDecl` list (frozen), and
  `fronted` would have to see through unary heads.

**Recommendation: C-c, then C-a.**

## R4. PartitiveNounPhrase

PartitiveNounPhrase keeps PP postmodifiers off *each/any/all of X*, while
PartitiveHead allows them on *two of X*. That is inconsistent, and English
admits both.

Option: merge PartitiveNounPhrase into PartitiveHead, and let Right
Association rank the inner attachment first. That removes one Category and one
Production. Risk is low to medium: PartitiveHead's DeterminerPhrase heads
would start agreeing on DeterminativeDistribution.

**Recommendation: merge, measured.** Stop if a cheapest-tier singleton is lost.

## R5. Load-report leftovers

1. **Privative features.** VerbKind `Lexical` and AdjectivalPassive `Derived`
   have no holder: every reader excludes the marked value. May
   english-lexical-analysis §3 admit a one-valued privative feature, with the
   unmarked value dropped?
   **Recommendation: yes.** Absence of the marked value is the unmarked value,
   and the checker accepts a one-valued feature whose value has two or more
   holders.
2. **Interrogative *how*.** Is it a closed function word, which would make the
   `InterrogativeAdverb` class a Closed Word?
   **Recommendation: yes.** It is CGEL's closed interrogative class (Ch. 10
   §7, p. 902).
3. **DeterminativeDistribution `SingularOrQuantified` and
   `IndefiniteArticle`.** Should one analysis cover CGEL's class (*another*, *a
   further*, *an additional*; Ch. 5 §3.4, p. 353)?
   **Recommendation: yes, as a follow-up ticket.** It keeps *another day*
   (Ch. 8 §6, p. 698).
4. **`FreeChoice`.** Turn the Requires into `Word("Any")`, and treat the
   Excludes under R1.
   **Recommendation: yes.** First check that *any turn* survives the fallback
   distribution.
5. **`Targeting`.** Should "this determiner ends in the Targeting Marker" be
   the TargetingMarker Category rather than a one-holder value?
   **Recommendation: decide together with R2.** B-b would dissolve it.
6. **`PrepositionMarking`.** Should the consumers take the determinative and
   its PP as siblings (`Selected(.., ["Of"])`, as SynesisDistributivePartitive
   does), and the tunnel go?
   **Recommendation: yes, as a measured restructure.**

## Side findings without a ticket

- **ParticipialSupplement passes a Gap through a comma.** It declares
  `transparent: ["head"]`, so a Gap passes through a comma-set
  post-dependent. The engine's adjunct schemas close that Gap (CGEL, Ch. 12
  §4, p. 1058). Measure which Readings depend on it, then drop the
  transparency, or keep it with a stated reason.
- **FloatedEachSubjectGap lacks the Irrealis exclusion** that SubjectGap has.
  Align the two, or state why they differ.

## Coverage question: Acclaimed Contender

The ranking-residuals work found that the analysis *[Knight, Aura, Equipment,
or legendary artifact] card* is not in Acclaimed Contender's forest: a serial
coordination of noun premodifiers in which the last conjunct carries its own
premodifier, under one head *card*. Find out which rule keeps it out of the
chart (premodifier coordination, the conjunct Category, or a unary
restriction), and whether adding it is new coverage under §4.4 item 3.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
