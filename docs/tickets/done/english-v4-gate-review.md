---
needs: []
---
# Gate review: keep a gate only if it is a formation rule (R1)

From `english-v4-child-name-control-and-category-fission-audit` (Findings C,
D and G). The owner ruled R1 in round 9.

## Owner's ruling (round 9, R1)

"Keep a gate only if it enforces a true grammatical/register formation rule
(e.g. AttributeChain/NegativeAttribute banning uncoordinated stacked
non-negative attributes; RelativeTemporalNominal licensing when-less relatives
on temporal nouns, OSG §11). Retire a gate that only suppresses a syntactically
well-formed English parse semantics will reject by type (PeriodNounPhrase as
object + Determined's two ExcludeTogethers, Unmodified's Gerund exclusion,
Manner excludes, PresentClause syncretic preterite, FreeChoice excludes, and
the internalised Symbol/Quantity excludes and Possessor gates unless they are
formation rules), subject only to safety against pathological forest explosion
on real cards. Record the kept set in ADR §4.4 as the closed list. For
AttributeChain, check whether the Negative* duplicates on
ModifiedQuantifiedNominal / ModifiedPartitive / ModifiedTargetedNominal can
share AttributeChain."

The test is `docs/decisions/english-v4.md` §4.4, item 10. Ranking plays no
part in it. A retired gate's Readings may tie with the endorsed one, or rank
ahead of it (items 6 and 7).

## The gates

The evidence is the audit's. Line numbers are from `english.ron` at this
ticket's minting and drift. "Owner" gives the classification the ruling
states.

| # | Gate | Keeps out | Audit evidence | Owner |
|---|---|---|---|---|
| 1 | PeriodNounPhrase (`:164`; DeicticPeriod `:1082`, DistributivePeriod `:1084`) and Determined's two `ExcludeTogether`s (`:1078–1079`) | A deictic or distributive period Noun Phrase as a general Object (*can't block [this turn]*) | DeicticPeriod and DistributivePeriod restate Determined. The Object is semantically anomalous, not ungrammatical (CGEL, Ch. 4 §1.2, p. 224). With the gate retired, the Object Reading ranks first: an Object costs 0, TemporalAdjunct costs 2. | retire |
| 2 | Unmodified's `Exclude("head.ObliqueKind", "Gerund")` (`:1152`) | A gerund Complement of *of* inside a Nominal (*the number of [attacking creatures]*) | Probe P4: about 25 units double, and the gerund Reading ties the Noun Phrase Reading (both 0/-8/36). `gerund_preposition_complements.txt:13` fails its `!`. | retire |
| 3 | The `Manner` Excludes: NonNumericComplement (`:433`), SubjectPredicate (`:548`), RelativeSubjectPredicate (`:1253`) | *way* (NounKind `Manner`; CGEL, Ch. 8 §2, p. 671) as a complement Noun Phrase or a Subject | The value is justified. Its three Excludes only exclude (Finding G, J list). | retire |
| 4 | The Verb Valence's `internalised` Excludes of `Symbol` and `Quantity` (`:1374`) | *{2}* in *reduced by {2}* read as a passive agent | NounKind `Symbol` has this Exclude as its only reader. The `Quantity` Exclude is the same kind (Finding G, F list). | retire unless a formation rule |
| 5 | PresentClause (`:277`; PresentComplementClause `:1333`; the Slot Kind mapping `:380`) | A preterite Clause Complement of *whenever*, *while*, *until* and *where* | Declared honestly in the lexicon (`docs/oracle-style-guide.md` §1, "Voice and tense"). Mapping the slot to FiniteClause adds preterite/present ties on syncretic verbs (*cast*). | retire |
| 6 | Possessor and PossessorNominals (`:186`, `:189`; Productions `:1172–1186`; the Coordination `:1484`) | A phrasal genitive (*the King of England's daughter*; CGEL, Ch. 5 §16.6, p. 479) | 7 Productions restate Determined, GenitiveDetermined, MaximumDetermination, BarePlural, ProperName and BareSingularTerm over BareNominal. Retiring takes −2 Categories and −7 Productions. The Readings it adds are unmeasured. | retire unless a formation rule |
| 7 | AttributeChain, NegativeAttribute and NegativeNounPremodifier, NegativeParticiple, NegativeAdjectivePhrase (`:240`, `:243–246`; Productions `:1053–1063`) | An asyndetic premodifier chain other than a *non-* chain | The register rule is `docs/oracle-style-guide.md` §6, "Determiners and modifier order": "If independent negative modifiers stack before one head, separate them with commas". The `!Attributes` witnesses in `asyndetic_premodifier_coordination.txt` rest on it. The three Negative* Categories restate PrefixedNounPremodifier `:1037`, the Premodification `/prefix` draft (`:1365`) and NegativeAdjective `:441`. | keep (the owner's example) |
| 8 | RelativeTemporalNominal (`:172`; RelativeTime `:753`, RelativeTimePostmodifier `:755`) | Other postmodifiers of a temporal Noun Phrase Adjunct | Coverage plus a gate. RelativeTime ≈ NominalPostmodifier; RelativeTimePostmodifier ≈ TemporalPostmodifier. | keep (the owner's example: *when*-less relatives on temporal nouns, OSG §11) |
| 9 | The FreeChoice Excludes: Determined (`:1077`) and DeterminedModifiedQuantity (`:798`) | *any* under those Productions | Probably exclusion-only (Finding G, T list). | retire |

## Change

For each gate, in this order:

1. **Classify it under the test.**
   - A kept gate gets a comment in `english.ron` that states the formation
     rule and cites CGEL or `docs/oracle-style-guide.md`.
   - A retired gate is deleted, together with every Category and Production
     that only restates another phrase for it. Examples are DeicticPeriod and
     DistributivePeriod, and the seven Possessor Productions.
   - If the evidence contradicts the owner's classification, STOP and
     surface it. A gate marked "retire" may turn out to enforce a formation
     rule. A kept example may turn out to suppress a well-formed parse.
2. **Rewrite the judgments it pinned.** A `!` assertion or a Reading count
   that only witnessed a retired gate's suppression of well-formed English
   asserts something wrong (§4.4, item 6). Rewrite it to assert the intended
   analysis positively, and name each in the landing.
   `gerund_preposition_complements.txt:13` is the known one. A `!` on a
   genuinely ungrammatical form stays.
3. **Measure forest-size safety.** Measure each retirement on its own, on the
   whole corpus: faces, total Readings, Surplus, the largest per-unit
   Derivation count, and the units that cross the judgment evaluator's
   4096-derivation limit. If a retirement makes a real card's forest explode,
   keep the gate, with that measurement as its justification. If it is a
   judgment call, STOP and surface the witnesses.
4. **Fill the closed list.** Fill the "Kept gates (closed list)" placeholder
   in `docs/decisions/english-v4.md` §4.4, item 10. Each entry names the gate
   and its formation rule with the citation, or the forest-size measurement
   that keeps it.

## The AttributeChain de-duplication check

Check whether the Negative* duplicates can share AttributeChain, rather than
restate the base Productions. If they can, fold them. If they cannot, say why
in the AttributeChain comment.

The ruling names ModifiedQuantifiedNominal, ModifiedPartitive and
ModifiedTargetedNominal. At this ticket's minting, `english.ron` has no
ModifiedPartitive or ModifiedTargetedNominal. ModifiedQuantifiedNominal
(`:170`) takes no Attributes. The duplicates the audit found are the three
Negative* Categories in row 7. Start from them, and STOP if the owner's names
point somewhere else.

## Not in this ticket

- FreeChoice's Requires become `Word("Any")` in `english-v4-free-choice-any`,
  which follows this ticket.
- No gate is added, and no gate is kept for its effect on ranking.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on the refreshed tvwxssow → xmzmyptl): faces read 28,801 → 28,801; total Readings 238,531 → 245,825; Surplus 102,458 → 106,859; lost faces: none.
Judgment set: 3,052 enforced and 206 pending, all holding; ranking tally S/A/P/F 708/232/11/93 on the first base (a statistic, §4.4 item 7).
