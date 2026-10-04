---
needs: []
---
# Compose Oracle coordination over independently verified phrase types

Verify non-coordinated constituents before composing their coordinations.
Cover nominal, noun-phrase, adjective, adverb, preposition, quantity, notation,
keyword, quoted, selected-complement, finite and secondary-predicate and clause
functions. Preserve category and function distinctions, grammatical agreement,
selected frames, voice and overt-head correlations. Include shared complements,
relative gaps, correlatives and independently licensed unlike-category uses.

Oracle flat lists of three or more coordinates require the Oxford comma.
Preserve binary punctuation and genuine layered readings. Keyword lines and
activation-cost sequences have their own comma conventions. Correct inherited
serial alternatives that admit missing final commas without weakening other
regressions. Independently construct complete admitted values and negative
values; preserve both roundtrip laws and complete grammatical readings.

Standard constraints apply. All corpus runs use at least eight workers. Record
any newly admitted wrong reading or unexplained coverage loss as a STOP and
resolve it before landing. Disclose scope and unsupported grammatical primitives
rather than using opaque fragments or game-semantic filtering. English Lean
requirements are retired by the current lexical ADR.

## Landing record

### PROVE

No silent loss: all 4,083 previously covered identities remain covered, and no
previously covered identity has a decreased exact Reading count. Newly covered:
908 identities, listed below with the cheapest retained analysis, its cost and
complete structural fingerprint. Covered total: 4,991 of 32,828.

Complete corpus enumeration validates all 36,721 Readings against declaration
admission, byte-exact realization of the declared reminder-stripped input,
lexical ownership, construction traversal and lexical traversal. Zero issues,
internal failures, duplicate derivations, cyclic derivations, limited enumerations
or undetermined results. All 32,828 Type Lines retain exactly one Reading.
Independent constructed values additionally prove realize-then-parse membership,
exact value sets, selected frames and ordered traversal rather than only a
surface or discriminant check. Raw Oracle source and its digest remain in each
census record; reminder stripping is the existing declared input policy.

The reverse-dependency gate is
`cargo test -p deckmaste_construction_v3_core -p deckmaste_lexical -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The final gate passes, including the existing whole-corpus and Lean integration
checks. Formatting and citation checks pass; citation audit reports zero noncompliant sites and zero stale
citations. No CR citation was added or changed.

Tests added: 42; removed: 0; restored: 0; re-spelled existing tests: 1; newly
ignored: 0. The existing serial-coordination test preserves its witnesses,
re-spells three flat-list positives with Oxford commas, adds the three omitted-
comma negatives, and now compares the complete independent two-value set for
plural Nominal versus complete Noun Phrase coordination. Its former singleton
expectation became obsolete when Nominal coordination was deliberately added;
the original Noun Phrase structure remains in the exact expected set.

No word/card/construction-named admission guard was added. Selection reads
declared grammatical features and exact independently owned frame signatures.
No opaque source fragment or game-semantic target filter was introduced.
The current v3 census emits no legacy permitted-licensing-checker total or
English coverage-lock field; those old pipeline fields are not fabricated here.
Lexical source loading and all independent lexical-value checks pass.

### DISCLOSE

Base verification preceded coordination application for the ordinary phrase
families, 16 selected verbal signatures and selected preposition heads. Missing
bases discovered during composition were implemented and independently checked:
possessive-determiner NPs, explicitly selected With/Without keyword PPs, usable
adverbs and selected to-infinitives. Possessor agreement remains distinct from
referent agreement; PP and adverb host permissions intersect under coordination.

Ordinary binary, Oxford serial and layered binary coordination compose over
Nominal, Noun Phrase, adjective, adverb, preposition, Cardinal, Amount, scalar/
pair measures, mana, keywords, quotes, infinitives, frequency phrases, predicates
and clauses. Shared verbal complements preserve 16 exact selected frame classes;
shared preposition heads preserve NP selection and both host-permission flags.
Object-gap coordination and correlative object gaps preserve transitive selection
and concord. Unlike AP/NP predicative coordination is licensed by common function,
while homogeneous compositions retain their existing category analyses.
Selected finite-clause complements use independently valid Finite Clause children.

Correlative markers retain lexical identity and pair Both/And, Either/Or and
Neither/Nor. Both is binary only. Clause-initial Both/Neither are excluded;
Either is supported for clauses and selected finite-clause complements.
Correlative adjectives have explicit predicative/postpositive hosts and do not
license malformed pre-head placement. Standalone Nor remains excluded without
an independently declared negative or correlative construction. AndOr is a
lexically owned Coordinator; it is not a correlated Or marker.

All grammatical Readings remain retained. The One/Multiple census appears below;
there is no destructive specificity selection or legacy specificity-resolved
count in this pipeline. New constructions keep the default cost 1; the existing
high-cost Flavor Word preference remains intact. All 3,864 retained trees from
Balefire Liege, Ashenmoor Liege and Luxknight Breacher were audited. No definite
invalid coordination distribution was found. Diagnostic-only normalization of
declared lexical morphology leaves 27, 45 and 114 structural attachment shapes,
respectively; actual Reading counts remain 3,456, 180 and 228. Every Reading on
each card ties at cost 106, 88 and 40. Broad versus close attachment can therefore
remain an unintended cheapest sample; game meaning does not authorize erasing
these grammatical alternatives. Attachment ranking remains an explicit obligation.

Deviations and additions:

- Added 280 constructions and 45 categories to cover the requested phrase and
  function families. Every added construction is named below; none was deleted.
- Added possessive NP, selected keyword PP, selected infinitive and adverb bases
  when independent composition exposed missing primitives. Their tests check the
  uncoordinated bases before their coordinated hosts.
- Authored KeywordComplement permission on With/Without explicitly; NP permission
  never implies permission to consume a Keyword Phrase.
- Widened declared lexical Word spelling to permit an internal slash only between
  alphabetic neighbors. This supports AndOr and general declared slash compounds;
  leading/trailing/doubled/numeric slash declarations remain rejected. Numeral slash
  notation remains independent. New source owners: Coordinator/Nor and Coordinator/AndOr.
- Generated materialization, Clone and structural ordering now dispatch through
  separate per-rule/per-constructor helpers. The expanded grammar exposed stack
  overflows in normal workers. Materialization dispatch dropped from 2,103,560 to
  11,352 bytes; recursive Clone/Ord no longer reserve grammar-wide temporary space.
  The existing normal-worker nested-document test passes unchanged. One independent
  trait regression preserves variant, form, lexical, optional/repeated-child order,
  Clone identity and all pairwise relations across 12 constructed values.
- Added the glossary terms Correlative Coordination and Shared Complement.
  Their definitions are linguistic; no invented CR authority was added.
- The 34 authentic shared-preposition witnesses are mutate reminder wording
  containing over-or-under placement. Reminders are stripped by this parser's
  existing policy, so these witnesses establish grammatical structure, not a
  claim of 34 new full-card gains. Keyword lines and activation-cost commas retain
  their separate conventions.

STOPs and resolutions: inherited missing-Oxford serial alternatives were retired
under the user's explicit policy; the old positives were re-spelled rather than
removed. The declared-word contract initially excluded AndOr; the general
alphabetic slash rule resolves it with independent lexical negatives. Normal-worker
materialization and recursive structural-trait stack overflows were fixed in the
compiler, with no enlarged stacks or weakened tests. No unresolved invalid-reading
or coverage-regression STOP remains.

Scope still owed to english-v3-systemic-residuals: mixed-number/countability shared
Nominals; propagation of placement constraints through layered ordinary/correlative
adjectives; displaced correlatives; negative standalone Nor; compound out-of and
preposition gerund complements; true clause gapping and suspended word parts.
Correlative adjective isolation deliberately does not license an outer ordinary
AP coordinator around an already correlative AP. Existing complete Noun Phrase
coordination still handles independently licensed mixed NP constituents.
Coordinated corresponding multi-slot amount/recipient or destination frame segments
remain with english-v3-frame-coordination; this batch's shared-head complements
are not a claim to discharge its 85 inherited obligations. Corpus text-body gaps
remain with english-v3-corpus-long-tail/systemic-residuals. The sibling keyword-label
WIP is untouched. No unresolved glossary gap remains for this landing.

### REPORT

Measured 2026-10-04 on grammar change tvmpxymwzptlwnvqsrynyorkqtsykrmo,
report child urwtwmuxvzsrmrworpsnqkwkzmtwpnts, covered 4,991.
Kata refresh before final checks was a no-op. Source SHA-256:
49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb. Lexical inventory SHA-256:
de810adc0d9241abce10d8f2453845a50fba58dc9b6d099722ed91df2e4a41f9. Construction count: 178 before, 458 after;
Category count: 65 before, 110 after. No English coverage-lock field exists
in this v3 pipeline; CR-lock coverage is unrelated.

| Census | Before | After |
| --- | ---: | ---: |
| No Reading | 28,745 | 27,837 |
| One Reading | 2,885 | 3,127 |
| Multiple Readings | 1,198 | 1,864 |
| Exact Readings | 13,014 | 36,721 |
| Covered faces | 4,083 | 4,991 |
| Normalized vocabulary-gap faces | 3,138 | 3,137 |
| Case-folded missing spellings | 1,193 | 1,192 |

The lexical manifest now has 34,834 owners, including 1,224 noncatalog owners,
120 verbs, 551 nouns and 44 adjectives. All 43,114 independent lexical values
pass. Raw lexical analysis includes reminders: 9,171 unknown occurrences across
1,279 spellings. The removed case-folded missing spelling is nor. New exact-word
homograph inventory: none; Nor and AndOr have distinct declared owners. Existing
homographs and form-literal/vocabulary overlaps are retained. New ordinary-word
form literals and new form-literal/vocabulary overlaps: none; coordinators and
markers are lexical fields, while separators are punctuation. No verb ownership
or morphology declaration was moved out of its macro.

Text: 75.724 seconds debug corpus wall, 1,563,590 ns/B checked-text thread CPU, 24 workers, host load [10.11083984375, 9.712890625, 8.57568359375].
Type Line: 7.950 seconds debug corpus wall, 172,255 ns/B checked-text thread CPU, 24 workers, host load [7.85693359375, 9.26904296875, 8.42724609375].
These runs overlapped the reverse-dependency gate on a loaded host. The text
run exceeds the 16.26-second quiet-host advisory; this is reported performance,
not an optimized cutover claim or a grammatical admission gate.

Reproduction artifacts are local inspection files, not source dependencies:
/tmp/coordination-final-text.json, /tmp/coordination-final-types.json,
/tmp/coordination-lexical-final.json, /tmp/coordination-ambiguity-audit.json and
/tmp/coordination-final-gate.log. Complete text/type runs use --all --workers 24
without a reading limit; text retains one cheapest checked sample per face.

### Added constructions

```text
AdjectivalCorrelativeComplement
AdjectivePhraseCoordination
AdjectivePhraseSeriesContinuation
AdjectivePhraseSeriesEnd
Adverb
AdverbPhraseCoordination
AdverbPhraseSeriesContinuation
AdverbPhraseSeriesEnd
AmountCoordination
AmountSeriesContinuation
AmountSeriesEnd
BareObjectGapCoordination
BareObjectGapSeriesContinuation
BareObjectGapSeriesEnd
BothAdjectives
BothAdverbPhraseCoordination
BothAmountCoordination
BothBareObjectGapCoordination
BothCardinalCoordination
BothFiniteObjectGapCoordination
BothFinitePredicateCoordination
BothFiniteSelectedHeadCoordination
BothInfinitiveComplementCoordination
BothKeywordPhraseCoordination
BothManaPhraseCoordination
BothMeasurePhraseCoordination
BothNounPhraseCoordination
BothPrepositionPhraseCoordination
BothQuotedTextCoordination
BothSecondarySelectedHeadCoordination
BothSecondaryVerbPhraseCoordination
CardinalCoordination
CardinalSeriesContinuation
CardinalSeriesEnd
CoordinatedClauseComplementPreposition
CoordinatedFiniteClauseCoordination
CorrelativeAdjectiveSeriesContinuation
CorrelativeAdjectiveSeriesEnd
CorrelativeAdverbPhraseSeriesContinuation
CorrelativeAdverbPhraseSeriesEnd
CorrelativeAmountSeriesContinuation
CorrelativeAmountSeriesEnd
CorrelativeBareObjectGapSeriesContinuation
CorrelativeBareObjectGapSeriesEnd
CorrelativeCardinalSeriesContinuation
CorrelativeCardinalSeriesEnd
CorrelativeClauseSeriesContinuation
CorrelativeClauseSeriesEnd
CorrelativeFiniteClauseSeriesContinuation
CorrelativeFiniteClauseSeriesEnd
CorrelativeFiniteObjectGapSeriesContinuation
CorrelativeFiniteObjectGapSeriesEnd
CorrelativeFinitePredicateSeriesContinuation
CorrelativeFinitePredicateSeriesEnd
CorrelativeFiniteSelectedHeadSeriesContinuation
CorrelativeFiniteSelectedHeadSeriesEnd
CorrelativeInfinitiveComplementSeriesContinuation
CorrelativeInfinitiveComplementSeriesEnd
CorrelativeKeywordPhraseSeriesContinuation
CorrelativeKeywordPhraseSeriesEnd
CorrelativeManaPhraseSeriesContinuation
CorrelativeManaPhraseSeriesEnd
CorrelativeMeasurePhraseSeriesContinuation
CorrelativeMeasurePhraseSeriesEnd
CorrelativeNounPhraseSeriesContinuation
CorrelativeNounPhraseSeriesEnd
CorrelativePostpositiveNominal
CorrelativePrepositionPhraseSeriesContinuation
CorrelativePrepositionPhraseSeriesEnd
CorrelativeQuotedTextSeriesContinuation
CorrelativeQuotedTextSeriesEnd
CorrelativeSecondarySelectedHeadSeriesContinuation
CorrelativeSecondarySelectedHeadSeriesEnd
CorrelativeSecondaryVerbPhraseSeriesContinuation
CorrelativeSecondaryVerbPhraseSeriesEnd
EitherAdjectives
EitherAdverbPhraseCoordination
EitherAmountCoordination
EitherBareObjectGapCoordination
EitherCardinalCoordination
EitherClauseCoordination
EitherCoordinatedFiniteClause
EitherFiniteObjectGapCoordination
EitherFinitePredicateCoordination
EitherFiniteSelectedHeadCoordination
EitherInfinitiveComplementCoordination
EitherKeywordPhraseCoordination
EitherManaPhraseCoordination
EitherMeasurePhraseCoordination
EitherNounPhraseCoordination
EitherPrepositionPhraseCoordination
EitherQuotedTextCoordination
EitherSecondarySelectedHeadCoordination
EitherSecondaryVerbPhraseCoordination
FiniteAdverb
FiniteClauseSeriesContinuation
FiniteClauseSeriesEnd
FiniteInfinitive
FiniteObjectGapSeriesContinuation
FiniteObjectGapSeriesEnd
FiniteSelectedAmountHead
FiniteSelectedAuxiliaryBareHead
FiniteSelectedAuxiliaryParticipleHead
FiniteSelectedAuxiliaryPerfectHead
FiniteSelectedCardinalHead
FiniteSelectedHeadCoordination
FiniteSelectedHeadSeriesContinuation
FiniteSelectedHeadSeriesEnd
FiniteSelectedInfinitiveHead
FiniteSelectedKeywordHead
FiniteSelectedLocativeHead
FiniteSelectedManaHead
FiniteSelectedMeasureHead
FiniteSelectedObjectEqualityHead
FiniteSelectedObjectHead
FiniteSelectedObjectNameHead
FiniteSelectedPredicativeHead
FiniteSelectedQuotedHead
FiniteSelectedSlashMeasureHead
FiniteSharedAmountComplement
FiniteSharedAuxiliaryBareComplement
FiniteSharedAuxiliaryParticipleComplement
FiniteSharedAuxiliaryPerfectComplement
FiniteSharedCardinalComplement
FiniteSharedInfinitiveComplement
FiniteSharedKeywordComplement
FiniteSharedLocativeComplement
FiniteSharedManaComplement
FiniteSharedMeasureComplement
FiniteSharedObjectComplement
FiniteSharedObjectEqualityComplement
FiniteSharedObjectNameComplement
FiniteSharedPredicativeComplement
FiniteSharedQuotedComplement
FiniteSharedSlashMeasureComplement
FrequencyPhraseCoordination
FrequencyPhraseSeriesContinuation
FrequencyPhraseSeriesEnd
InfinitiveComplementCoordination
InfinitiveComplementSeriesContinuation
InfinitiveComplementSeriesEnd
InitialAdverb
KeywordComplementPreposition
KeywordPhraseCoordination
KeywordPhraseSeriesContinuation
KeywordPhraseSeriesEnd
ManaPhraseCoordination
ManaPhraseSeriesContinuation
ManaPhraseSeriesEnd
MeasurePhraseCoordination
MeasurePhraseSeriesContinuation
MeasurePhraseSeriesEnd
NeitherAdjectives
NeitherAdverbPhraseCoordination
NeitherAmountCoordination
NeitherBareObjectGapCoordination
NeitherCardinalCoordination
NeitherFiniteObjectGapCoordination
NeitherFinitePredicateCoordination
NeitherFiniteSelectedHeadCoordination
NeitherInfinitiveComplementCoordination
NeitherKeywordPhraseCoordination
NeitherManaPhraseCoordination
NeitherMeasurePhraseCoordination
NeitherNounPhraseCoordination
NeitherPrepositionPhraseCoordination
NeitherQuotedTextCoordination
NeitherSecondarySelectedHeadCoordination
NeitherSecondaryVerbPhraseCoordination
NominalCoordination
NominalSeriesContinuation
NominalSeriesEnd
PossessiveNounPhrase
PredicativeComplementSeriesContinuation
PredicativeComplementSeriesEnd
PrepositionPhraseCoordination
PrepositionPhraseSeriesContinuation
PrepositionPhraseSeriesEnd
QuotedTextCoordination
QuotedTextSeriesContinuation
QuotedTextSeriesEnd
SecondaryAdverb
SecondaryInfinitive
SecondarySelectedAmountHead
SecondarySelectedAuxiliaryBareHead
SecondarySelectedAuxiliaryParticipleHead
SecondarySelectedAuxiliaryPerfectHead
SecondarySelectedCardinalHead
SecondarySelectedHeadCoordination
SecondarySelectedHeadSeriesContinuation
SecondarySelectedHeadSeriesEnd
SecondarySelectedInfinitiveHead
SecondarySelectedKeywordHead
SecondarySelectedLocativeHead
SecondarySelectedManaHead
SecondarySelectedMeasureHead
SecondarySelectedObjectEqualityHead
SecondarySelectedObjectHead
SecondarySelectedObjectNameHead
SecondarySelectedPredicativeHead
SecondarySelectedQuotedHead
SecondarySelectedSlashMeasureHead
SecondarySharedAmountComplement
SecondarySharedAuxiliaryBareComplement
SecondarySharedAuxiliaryParticipleComplement
SecondarySharedAuxiliaryPerfectComplement
SecondarySharedCardinalComplement
SecondarySharedInfinitiveComplement
SecondarySharedKeywordComplement
SecondarySharedLocativeComplement
SecondarySharedManaComplement
SecondarySharedMeasureComplement
SecondarySharedObjectComplement
SecondarySharedObjectEqualityComplement
SecondarySharedObjectNameComplement
SecondarySharedPredicativeComplement
SecondarySharedQuotedComplement
SecondarySharedSlashMeasureComplement
SelectedPrepositionHead
SelectedPrepositionHeadCoordination
SelectedPrepositionHeadSeriesContinuation
SelectedPrepositionHeadSeriesEnd
SerialAdjectivePhrase
SerialAdverbPhrase
SerialAmount
SerialBareObjectGap
SerialCardinal
SerialCoordinatedFiniteClause
SerialEitherAdjectives
SerialEitherAdverbPhraseCoordination
SerialEitherAmountCoordination
SerialEitherBareObjectGapCoordination
SerialEitherCardinalCoordination
SerialEitherClauseCoordination
SerialEitherCoordinatedFiniteClause
SerialEitherFiniteObjectGapCoordination
SerialEitherFinitePredicateCoordination
SerialEitherFiniteSelectedHeadCoordination
SerialEitherInfinitiveComplementCoordination
SerialEitherKeywordPhraseCoordination
SerialEitherManaPhraseCoordination
SerialEitherMeasurePhraseCoordination
SerialEitherNounPhraseCoordination
SerialEitherPrepositionPhraseCoordination
SerialEitherQuotedTextCoordination
SerialEitherSecondarySelectedHeadCoordination
SerialEitherSecondaryVerbPhraseCoordination
SerialFiniteObjectGap
SerialFiniteSelectedHead
SerialFrequencyPhrase
SerialInfinitiveComplement
SerialKeywordPhrase
SerialManaPhrase
SerialMeasurePhrase
SerialNeitherAdjectives
SerialNeitherAdverbPhraseCoordination
SerialNeitherAmountCoordination
SerialNeitherBareObjectGapCoordination
SerialNeitherCardinalCoordination
SerialNeitherFiniteObjectGapCoordination
SerialNeitherFinitePredicateCoordination
SerialNeitherFiniteSelectedHeadCoordination
SerialNeitherInfinitiveComplementCoordination
SerialNeitherKeywordPhraseCoordination
SerialNeitherManaPhraseCoordination
SerialNeitherMeasurePhraseCoordination
SerialNeitherNounPhraseCoordination
SerialNeitherPrepositionPhraseCoordination
SerialNeitherQuotedTextCoordination
SerialNeitherSecondarySelectedHeadCoordination
SerialNeitherSecondaryVerbPhraseCoordination
SerialNominal
SerialPrepositionPhrase
SerialQuotedText
SerialSecondarySelectedHead
SerialSelectedPrepositionHead
SerialUnlikePredicative
SharedPrepositionComplement
ToInfinitive
UnlikePredicativeCoordination
```

### Newly covered identities and selected analyses

Each selected analysis lists every new construction occurring in its cheapest
retained tree, followed by the full structural fingerprint. The census retains
and validates all other Readings as well.

| Face | Identity | Readings | Cost | New constructions in selected analysis | Fingerprint |
| --- | --- | ---: | ---: | --- | --- |
| Abyssal Gatekeeper | 4954181c-cb46-4cac-adc3-2c8b7693a211#card | 1 | 26 | PossessiveNounPhrase | 80325001a2dbb887a2b117c1b7462fdbe6caae4ffc77e8558d8d7fc40f2d1316 |
| Abyssal Gorestalker | 465cda20-c8f3-4a7c-a250-875877bce21d#card | 1 | 27 | PossessiveNounPhrase | 341472a858227cfe5474f5bf35d21d54edd3bf96010b9339bafc196733f223ee |
| Abyssal Persecutor | 7282f643-191b-41be-9f6f-82360c915d6d#card | 4 | 34 | PossessiveNounPhrase | 32eb20acd0113978d234fa6f6a16b76c1951d4fe24b128861838e480deba2f40 |
| Abzan Advantage | e7b2f844-2ba2-4465-bb80-358a4d915f36#card | 1 | 26 | PossessiveNounPhrase | 8434a652b059350f06f25a73d912ac9af8b3045843e79c2d81393f2f2c2a8e19 |
| Abzan Banner | 46535f8e-1bcd-4588-ac6c-a4bc89c379c8#card | 1 | 48 | ManaPhraseSeriesEnd, SerialManaPhrase | 5a2f5b442b33f2a6d502b6e884acc8173e7848f5c86e98c5f17905e8d85494d5 |
| Accursed Duneyard | 48edc348-93f6-4dce-9cc4-7244d76b6f4a#card | 1 | 45 | NominalSeriesContinuation, NominalSeriesEnd, SerialNominal | a02fa003174bc81bb965ee3ba767134d5f535723ca87fc44a031304d889c363f |
| Acidic Slime | 21f45043-5419-4019-8b6c-e5294bd5f549#card | 1 | 25 | NominalSeriesEnd, SerialNominal | 42ff823b8d9ee147aa0edba186f4579e1bbaf7079bcce478560435f4d5747897 |
| Acolyte of Aclazotz | 324d2aa8-6dea-4b0d-898d-1e52ed187565#card | 2 | 36 | NominalCoordination | 235ed0a41d3a6684057e0ceb62782d774f1a2e7d75e3be3d8c9cb3ec85d0666f |
| Act of Authority | c12b8e31-5702-4d3c-80c9-56c87e50679d#card | 32 | 78 | NominalCoordination, PossessiveNounPhrase | 0dbf815b2859223562273bb620b3d38fa73f7ffd0bfc424ad7408ef0c6386259 |
| Adarkar Wastes | d5ad26cc-2bdb-46b7-b8bf-dd099d5fa09b#card | 4 | 47 | ManaPhraseCoordination | 04ef2a2c140e017ef3604edfed29575a7df36ff49fa1441ebc986d7c69f9e88a |
| Addle | 3745aea4-4455-4859-9772-bcfd14e4067b#card | 10 | 53 | PossessiveNounPhrase | 12350b7c3d2a52352032910b18e0aabcca886005b01e58dc02ecde99427a8fc3 |
| Aerial Predation | 6bac1495-16cf-4e1e-9126-482f85322c78#card | 6 | 25 | KeywordComplementPreposition | 5a9ca853fe43c3658cb2a4f115a0e6948fdf0f11fc88bb40c24298b85cc9513a |
| Agent 13, Sharon Carter | ddf7201a-6672-416d-be6b-202dafbeccab#card | 2 | 22 | Adverb, FiniteAdverb | 881f9e45302e693d07c7c71249491fa2b121c4eeef4546e61e2dd8e9370e2b73 |
| Agent of the Fates | 45f88a6a-ac8a-4a53-a96b-f09ca33c79ba#card | 4 | 38 | PossessiveNounPhrase | 322d77288d9ee3268d78b40735d8cff18361834c3ae60deff421f50ce14724b6 |
| Ahriman | 37d1d507-25dc-4cd0-b51e-2f1e9b24832c#card | 1 | 29 | NominalCoordination | 0fd3f9f7d1c511d1c8e427048d8fd9711edd7bb1131ba4b1cbe55c9c8d21ecb4 |
| Aim for the Head | b191ae0c-7d54-459f-abe3-aab5d792e514#card | 3 | 39 | PossessiveNounPhrase | 7178c885db219bdd21b1343a0717bd77b97ec5f29232ff9bae6d3047c7986a96 |
| Air Servant | 49638d96-e83e-4aab-bafb-d6183a6e3ff6#card | 3 | 22 | KeywordComplementPreposition | 0e144bba7f44c01c7f93c1e199d347c3af333869b3f1df68651eff0476211715 |
| Airship Crash | 72a0e0f1-e8be-441a-bcaf-029fb1bd1fd2#card | 4 | 23 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | 4ad6f8a348cc970ebd3041b36dad9a7e6787ca8a0b128bfb447fb6a832312a07 |
| Ajani's Mantra | 2f47661e-a107-4d98-8773-0a068a63df49#card | 4 | 26 | PossessiveNounPhrase | 11afe1c9f1f84c25c2259e43785f6244aa7e7a1a7be3b89197b0b86e48855439 |
| Akki Blizzard-Herder | 16fe1f04-9bbf-43a3-ab14-395fdf4a3a43#card | 1 | 26 | PossessiveNounPhrase | 76fd86152e16bc471b6990dd1613ddd6e77249e9bec8f4eb8366060a70dace60 |
| Akki Ronin | 4b4c76e1-c480-4d10-93d4-483ef2debc4b#card | 32 | 49 | Adverb, FiniteAdverb, NominalCoordination | 0676505c180f6f91191a5af3a750f2383b351c7ba7bf78e7b4818fc683f2672a |
| Akki Scrapchomper | baea7cc5-15ec-4229-ad8b-d4b34c2e21e8#card | 1 | 29 | NominalCoordination | 1223e5b6d19430ae499a4883c826b43b611eb80b30449c40ab931faf66b170f2 |
| Al Bhed Salvagers | 43f81514-e812-4e17-9b1a-704c136d8ffd#card | 16 | 40 | NominalCoordination | 02d1fc8c0eb78cdc03dd10c77e990b807891df33f0bd1e0c7ef47c7cf6acface |
| Alania's Pathmaker | ce36a450-68a6-4519-a9f0-0e3b772054eb#card | 16 | 51 | PossessiveNounPhrase | 0186adca84221bdad7f5b53cdb97accb2abac600e2d34cc6192af82552fb5f55 |
| Alchor's Tomb | 61473d8e-45f1-4753-918d-04918a466031#card | 4 | 33 | PossessiveNounPhrase | 58d01294f40118389330970aad56201df0747c0e00f18bc0f4e114185f98cdb5 |
| Alert Heedbonder | 683a06b6-af77-440f-81b9-8958597a7324#card | 72 | 39 | KeywordComplementPreposition, PossessiveNounPhrase | 01a3482f718819281c530a8c75deea3d66ffd19403c6c2e265f76954ae2258d1 |
| Altar's Light | fa9b6be2-b88c-4302-b7e2-faf25a60bcb9#card | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| An-Havva Township | 40ae17be-9998-4ee4-9d95-82a08895405f#card | 1 | 53 | ManaPhraseCoordination | ddbd7cec6ec73f3dc769cddf93a325794ba2613daa2162204646b20242d179f9 |
| Ancestor's Chosen | fc2ccab7-cab1-4463-b73d-898070136d74#card | 36 | 33 | PossessiveNounPhrase | 016daeae9667242ed5496cc5ef978d75adac76175ceddd19bbce557bcefda918 |
| Ancestral Tribute | 380ea3b6-8eb4-466a-8680-ce5cfed9b8c4#card | 20 | 31 | PossessiveNounPhrase | 0b04644c133f4f8c5dce76366ccdb2ab010f9085bae734926a158d301dad4a76 |
| Angelic Benediction | 6b19377c-f178-49cb-95f8-fd3451acc7cf#card | 4 | 33 | Adverb, FiniteAdverb | 4c4031172c4d77922eed93b575778bf574b1a12068f8a5fcb1cf4af9eebeff0c |
| Angelic Edict | 36de8401-ee4e-413f-91ad-06924e39c857#card | 1 | 13 | NominalCoordination | a85941a512e6cc6f2daeac0598c6f65d356b428f6fa099c42358187a66940a8f |
| Angrath's Rampage | 850a8369-94ab-43b6-8cfe-286e3623b19d#card | 1 | 65 | PossessiveNounPhrase | 913c1fdafe83616642e65780825eb19db00e18f21ad4871822bfc27f768ab5d1 |
| Aphetto Alchemist | 867aa0b7-b814-4d33-a677-f6368c8e93c0#card | 1 | 22 | NominalCoordination | b833fcb930fe1896dd53468f882c44ac711a5e23587ef0aa6b6f4fd6d91fa331 |
| Apocalypse | 82c8f5dd-563d-4fd0-bd43-7ee2001d3777#card | 4 | 22 | PossessiveNounPhrase | 1e0efbc125174053d2b30616cb3c534e4363ea0b6fa03a722ed6e56de794b78b |
| Appetite for the Unnatural | f041b37e-25af-4aff-b7e3-f07a4d5c6f9a#card | 2 | 24 | NominalCoordination | 16ce78133daa2e11cbc3d9fc506809a6723a3f8ee682f7c8317c05393af89afd |
| Arc of Fortune | 67c603cc-ad66-4a1c-8386-5901c9c01bfb#face:1 | 2 | 23 | PossessiveNounPhrase | a1f40250d35f74f6e676180a22f81ed2ddfbba8a5ce22a65e04edbee648fbff1 |
| Archdemon of Greed | 18932bb9-e054-4ad6-80c8-d2f177312da0#face:1 | 112 | 56 | PossessiveNounPhrase | 00358169825bf5e18841093f6441f619bc0c9a849a65a71a1c9d5d83610e61f2 |
| Archivist of Oghma | 08b13e1f-27ca-40a8-b5ed-88ac933d24bf#card | 6 | 31 | PossessiveNounPhrase | b0b210b980474169c02e0c941f784300ff62417d08bb4e86660f4996fbb18122 |
| Archon of Cruelty | aa1a6646-c1e6-4bff-9092-43ee3e137914#card | 8 | 59 | NominalCoordination, PossessiveNounPhrase | 2671819302ed8abe7d244da0aa5d04324c50c569de09605c22adf46d1a5c5235 |
| Armament Corps | 3b93427c-5b40-4119-96ae-7dce3ca2bbb9#card | 26 | 39 | CardinalCoordination | 1f78898f04db528afd333d02ab9c8bbc5d19eed08b93f4bd0e9edd89ab20a9c9 |
| Armament Dragon | 23c75386-8011-4ee4-97e2-eaafeb20b788#card | 26 | 44 | CardinalSeriesEnd, SerialCardinal | 00c6f2fc0065075305604f34b88c0f3de5e3244bb2373e6042536730bf8b83f3 |
| Artisan's Sorrow | 115fd5ac-460b-49a3-83ed-9d56a7aecb14#card | 1 | 21 | NominalCoordination | 8ed934a652376ac3881d5eafedb9f9692a3b99775dbb83419af4d77ac02a656d |
| Ascendant Acolyte | 8b9918c3-1135-4c63-991f-f0a38340a33b#card | 4,752 | 86 | PossessiveNounPhrase | 0004c7f50108a0b76a5a0a049bab271c06b2646da2b507a3dd01e2cbbd4de90a |
| Ashen Firebeast | 7246e3a0-f8b7-4c1b-ae75-a1eb8990a728#card | 5 | 27 | KeywordComplementPreposition | 4b6dfb3a0d2948f9d87edfabfe93d181ec9a9d45f907e581f452c838d6acad49 |
| Ashenmoor Liege | f779754a-3de9-4621-b190-cf32949da0e8#card | 180 | 88 | NominalCoordination | 01732e0af7c3c6a68cbd5948e4878984ab9facc811f459257fc28f289519ab83 |
| Assassin's Strike | 1ad6339d-6206-49bf-bf97-05e6d3bcaab7#card | 1 | 23 | PossessiveNounPhrase | 87652313e8cd7b61bd76332fbc37186b0a0395f8ecb50e37cecb000e4654e5ac |
| Atraxa's Fall | 22677153-bd87-4ff8-af92-ced4da2aac6c#card | 4 | 20 | KeywordComplementPreposition, NominalSeriesContinuation, NominalSeriesEnd, SerialNominal | 10637a1ebb1ccddb81bf5015f7e32c57f9a428c0ec14f0dba3c8b0a56d2f14b9 |
| Atraxa, Praetors' Voice | 7e6b9b59-cd68-4e3c-827b-38833c92d6eb#card | 2 | 27 | PossessiveNounPhrase | a36b9dc834f72b1293ae09a8adb761de3d0140ed7bd934914251bf9634ecaa96 |
| Aura Shards | 8d03d050-391c-4311-8c42-4ee632d40fdc#card | 4 | 30 | NominalCoordination | 5c11830e1591d42d342ee30aa4785e7c31ca94cd793a907ad06b44c360127c83 |
| Avacynian Missionaries | 5b356a01-900c-4fe5-93bd-45629528c484#face:0 | 2 | 29 | PossessiveNounPhrase | 920936142f8fdfc5e77eade51bbdc0fec88cb305d06abcbc813bcc88cc523ecc |
| Avaricious Dragon | 9be9d534-470d-463a-ba2f-9817a6723692#card | 8 | 47 | PossessiveNounPhrase | 1ebcfdc2c6d5d48f47924276c5c5d98ba83179df3adcd79276897c2c726def9b |
| Aven Gagglemaster | 8c601628-b1ed-467b-ba92-8572343147f8#card | 36 | 36 | KeywordComplementPreposition | 0142ee5615d2743c7f0011bfb837567095e923660e8fb1b508f509746c949dcc |
| Aven Windreader | 8406b724-a824-4905-83e0-a6c02d7f72e3#card | 2 | 29 | PossessiveNounPhrase | 7525b44ff4e94a373247b2b76160370425b69b160788f3b074c632b48c88935e |
| Away | e4b1ef6a-6f05-472e-aaed-1c24dd6605c7#face:1 | 1 | 22 | PossessiveNounPhrase | d5c01a88e64cc2eac7c3e5a977414c51d0f93219b8c50f17d92f557035c94e9e |
| Aysen Abbey | 6ff85e73-bf7a-4a9c-80ef-6ce76656fab7#card | 1 | 53 | ManaPhraseCoordination | c49a2491a2f3dce472b5566bdb9a64b3ef6613ab6548713a09858107b8950356 |
| Azorius Cluestone | 27e04c41-f42c-4d60-8a71-ec2d7c326f64#card | 1 | 44 | ManaPhraseCoordination | 5613393cbdeac81a3cae7e70cf4dcc4213367ed94ddabf397d6355586c5b1ac2 |
| Azorius Skyguard | bbfdb5eb-2140-4d50-a667-1b4ac75d03a5#card | 2 | 27 | PossessiveNounPhrase | 1ae6bebc8b7cab16e2d19fd52e0ef8d42e65f607bffba5101e0089a4b9f0ba06 |
| Balefire Liege | ee35ed62-d5a2-4e65-a7eb-fcdcb3665532#card | 3,456 | 106 | NominalCoordination | 00174d8c1dd56e70c6986945dc4e782c95d97fde9758cf33ad7417bd50b0625b |
| Baleful Stare | 4135131d-4653-4767-ad1d-68c9bf393c3a#card | 40 | 38 | NominalCoordination, PossessiveNounPhrase | 0505112f658213c91c9331a5dc6971431a9392aea6adec5fcc9a7a7e629a1c4a |
| Bamboo Grove Archer | 657f6318-aa09-46f1-ab69-896b17a48950#card | 3 | 32 | KeywordComplementPreposition | b9f2fc026e95519d976abddcad8126ad18ff0516ef148a6cc36178fc7e6c3428 |
| Banishing Coils | 67dec976-bcf5-4995-8da1-cd570862d3cf#face:1 | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| Barter in Blood | 9167998d-5cac-47d7-99f2-f38122f7b8e7#card | 1 | 20 | PossessiveNounPhrase | c953e336bed7816b13cec1dd8067cde024d840bd341d87d0641f71ad76aca899 |
| Basalt Monolith | 6b8cf2a0-b045-4d91-9d91-c602d40c6237#card | 12 | 50 | PossessiveNounPhrase | 3141bbf50e332fcfce27b5fbf0393011523d5a1b6cc195d7108a3343d88d3252 |
| Battered Golem | 066dfefd-eef5-4c81-8e8a-310bac01674f#card | 24 | 42 | PossessiveNounPhrase | 0af7ac99380c1a18b0573536316c6bd93c076c2491cfc5f21267e6a31502db8e |
| Battle Mammoth | 7a6db509-7ef4-483c-b8a0-62a4f3e16ef9#card | 20 | 55 | NominalCoordination | 14f02a5fd3170bbf2d2ee947de92daee00471be0a8b530a92697b3b490216c98 |
| Battlefield Forge | 6b75b94e-83b7-457e-ac41-7ca90b5a59aa#card | 4 | 47 | ManaPhraseCoordination | 16a7792615cf04977ffdaab448e51ac207c73040a1a4ab07f50221884f6a96c3 |
| Bazaar Trader | 21ad7067-3210-42d6-9009-0784b2834227#card | 6 | 32 | NominalSeriesEnd, SerialNominal | 63ed9515deda8923d79ae532e5a2737de0575cf87c7b35ad7fcdc9961ea5a70c |
| Bear Down | 8a31c49f-5b3f-4f84-a08e-58b30a6bff7c#face:1 | 1 | 13 | NominalCoordination | bc97d4e47a24480cb6c1f41fdf441d4cc53efea11b4f2764b359754e0e2e37df |
| Bedevil | bceecc64-96f1-4e7b-8904-0aef90377764#card | 1 | 15 | NominalSeriesEnd, SerialNominal | 2d50f6418e934c021b8a5adff5368a573df10d468c3d0f1f79624a17569d625b |
| Benalish Lancer | b5adbd37-8d98-4c65-b42d-cd9db94e7222#card | 18 | 44 | KeywordComplementPreposition, PrepositionPhraseCoordination | 0f9f6951c721dfc9970f00a3289b3e896f1c87090797a4b8b8eb58e05d870a5b |
| Benalish Sleeper | 14c3692d-16a9-4ef0-98cd-60d15e4aa31f#card | 1 | 39 | PossessiveNounPhrase | 03cbcb707769c4b33a2f8e6abc020732596b1c9602e8c76dbd67bfe521289efe |
| Bereavement | fae834ee-2b2e-4f68-b4a4-81592cbecefb#card | 1 | 23 | PossessiveNounPhrase | 86b40e20709854d58ae12059a018c68bcc11d092838999c17e9d337a83c52d45 |
| Binding Mummy | 5adc9d0d-3393-4d91-be85-1f9dd344698b#card | 4 | 30 | NominalCoordination | 068fcaf72cd84917250410f4b09f4f4fc9c54a981a1cb9a25b71aa8557749749 |
| Blessed Light | 918b3860-e96c-45b5-b0e6-e82cad9f304d#card | 1 | 13 | NominalCoordination | a85941a512e6cc6f2daeac0598c6f65d356b428f6fa099c42358187a66940a8f |
| Blighted Fen | b8f3da11-7c8f-4846-98a6-204bfd8d572b#card | 1 | 49 | PossessiveNounPhrase | a0cf3b393451a0767bdb60c924417b764cda08e679a03f0ffe043649935c2763 |
| Blood Burglar | 9c975fda-471a-4d41-9bb6-ea42edc6a0e6#card | 2 | 17 | PossessiveNounPhrase | 60341d0912fb107449137db9502c629b4f394c223585a347e1352fad985e6ee3 |
| Bloodfire Dwarf | ff42551a-a08e-4d0c-a5b1-d1c1bbff4915#card | 5 | 32 | KeywordComplementPreposition | 0a1945d7b4b2b03c9979e6786cd6cfafb377f28b81dd4ecbe9852ada8e40f52a |
| Bloodgift Demon | e64b184d-9746-4723-a928-d459b5c3ee6c#card | 2 | 32 | PossessiveNounPhrase | 9965a9fc81de9004c7d77020d7ed20620c1b7f58ad1fa6b690c429fc13d8e98e |
| Bloodstone Cameo | 1ce6ae30-33c3-4f05-9286-69b0871b1c2d#card | 1 | 17 | ManaPhraseCoordination | 6eb9764492a26add2bdb2e449294d61d5c5e491df1d03de66ab2c6bf707949f6 |
| Blossoming Calm | 489a60f1-83f8-465b-918f-7d63d4f76d14#card | 16 | 32 | PossessiveNounPhrase | 08d8d61a5ddc5d1fece32567f60fd446dd631f32cb50250aa10ba97390237401 |
| Boggart Forager | cef9e0d3-5e28-4c5f-88fc-0d78af5a4db1#card | 1 | 25 | PossessiveNounPhrase | 294056632e2c7e8530f4dace8a5a95b1045b3962a1ce9da00cd6e7da39973890 |
| Boilerbilges Ripper | 0cbe2a00-4303-4ade-98b5-4172d97f15fb#card | 24 | 50 | NominalCoordination | 03258b71f87d745d907124c3584f9a7768ed39c4a9d51e2884cfec1eeb58b6c2 |
| Bola Slinger | 6ff995f9-080d-4316-8fbe-39bf195db91b#card | 2 | 30 | NominalCoordination | 62ef3bc22f180c3752e0a9cbb6d6dd4011a3663b41907f53a1372bc06a6ce0c7 |
| Bonded Construct | c7556be3-649a-4994-8aaa-a0f258856682#card | 2 | 16 | Adverb, FiniteAdverb | 3f033595abe4a76d5449a0d319c2d28aea1d470fc8861bca32ad0f7e09826e25 |
| Bonded Horncrest | bf71c210-8fef-4eda-a943-8c52cbae6159#card | 4 | 18 | Adverb, SecondaryAdverb | 7e3bab0b1502dac18f9404ee792ed91d467b7f13797407bc0f6d5d2d95e44397 |
| Bontu's Last Reckoning | c010d833-1873-4132-b4ff-8d78b3d7c2d7#card | 24 | 35 | PossessiveNounPhrase | 05067e3cb1802a0c2e9b7c41112efc0acce91ce6bcbec84504da6812308595ef |
| Boros Cluestone | f6c3b420-9aca-4fab-b7bb-1814bd2d93f6#card | 1 | 44 | ManaPhraseCoordination | 002164953beea3c9c589fe5ddc69c258c0b5f34f621a7c3f0cad71898ab20d33 |
| Bounding Krasis | c6736bac-314b-48af-8722-c16a0d66affa#card | 2 | 29 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 0247704c6755588900f55290005a593e90697fbf53d7d9b28fbd8acb08bbef3f |
| Bower Passage | f91ef713-5289-4901-9291-0a5afe6f997b#card | 2 | 25 | KeywordComplementPreposition | 48ada23d67f6afccae0de92b3029751c75645d5e310490e3bd19ab00cd9ecbb8 |
| Brainbite | fef94125-aa8d-4147-a609-1e990961bde2#card | 6 | 52 | PossessiveNounPhrase | 2f9fb3011ce0ccd4abbe7165e4e1f25d7e425e39674dc98de0ad2c7cb59cc594 |
| Bramble Wurm | db4f67a9-6b47-4abe-9367-718da9e0609e#card | 16 | 55 | PossessiveNounPhrase | 25f4508cf6e3ceb6ebdcdcc92741c05f02a9491779ecff0928242b12e072f74d |
| Break Asunder | d2c53737-c265-46e3-a779-52c6b4f82d7d#card | 1 | 18 | NominalCoordination | b0d4b7cd982e362568f79e3e3aee2bcd2954e110cb495f8105671defc0c875ef |
| Broken Dam | 75fffc1b-db65-4c6b-9503-db50fccb837c#card | 4 | 18 | CardinalCoordination, KeywordComplementPreposition | 0828979fcad8776eebb3f025980d9dd98655b7566a3bef921b891e4b5fb865bc |
| Broken Wings | 5e316864-d55c-496f-8f46-773567896864#card | 4 | 18 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | 35cdbbf4063433aacffab009817dd78f4a995fc8d9971e0e0ca499bc7eae9495 |
| Brushland | 5eb8b497-ec9a-4a89-ad29-1ec3ca82da7c#card | 4 | 47 | ManaPhraseCoordination | 359998bc6a3b14402aaaa09670e7f56886257914132f1d72a5bea05806ebaa34 |
| Burning Vengeance | ef1fb174-9a34-43d0-8caf-9c1938147a12#card | 48 | 33 | PossessiveNounPhrase | 1294ef3d0515842e8402e8badc35d8e4fc402c7004d4af971c451c627f85f606 |
| Butcher of Malakir | a85197ab-dc94-4b72-9716-8dbdbbe90ff8#card | 2 | 37 | PossessiveNounPhrase | 3f14885131980ee457ebf4e52abeb5858c1364eb4f61423dc16b60e1871df506 |
| Cadaverous Bloom | fbb0f73b-5e30-4632-99c1-e49582e41f8d#card | 4 | 27 | ManaPhraseCoordination, PossessiveNounPhrase | b29ad904e18d11b0b2be1e8f9823070c9d368a9c2fcb59da6dbdbf350ee555bb |
| Capashen Unicorn | 52793400-bc83-402a-9609-928a4d3ac812#card | 1 | 29 | NominalCoordination | 13b1d70332b65b1a7cf9c7c88b8e364e4fd89a1edbf8a831040d8b453d726aeb |
| Captain of the Mists | a7b6cd8b-1bed-4742-8c97-8be4edbfc8ee#card | 4 | 51 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 059d8373cfc162dcbb0eee831c1a2c7350d0cad25bae98eb2f0bb1fee0f5b3e0 |
| Careless Celebrant | 56afad48-c5e5-4fbe-8889-15bcab234c31#card | 6 | 33 | NominalCoordination | 73548a1b832792c9d5036b3ce4f23413f7ff268720f5052a3a1c2f8c811bed34 |
| Carnophage | 9ac77cfd-7d78-4c8a-bffc-17649a852114#card | 16 | 30 | PossessiveNounPhrase | 0eda49facafed49f848d7859848680220a956950b7591721c291b8572431a9d3 |
| Castle Sengir | c7f0251a-9341-4ff2-8b15-31c06eb4f2e7#card | 1 | 53 | ManaPhraseCoordination | b6c1d6eefd4e549693cd5188f7ae4de3fa8cc59aa8fa4d511087ce22f0d8cd4c |
| Cat-Owl | ba6840a6-b6fd-4b91-ac1a-a61b7b15849a#card | 1 | 23 | NominalCoordination | 2351ebd86a63f07cfc06018e051deb71b7cf19ae55d1e44d44d30a492c597885 |
| Cathar Commando | 774dce79-67e0-4820-8013-c7a7347993ce#card | 1 | 27 | NominalCoordination | 4098a3fee54295a85b3b9057a5d6e3b814d8706ec75b8ea7afc0c738c7a895db |
| Caustic Caterpillar | 45d35128-76e6-43f9-8d23-41f7506c3a71#card | 1 | 25 | NominalCoordination | 4cab56f4215336dc2405f3c8d5bcfeb8dd9029371f19c98e85fd9fe2a4e70f42 |
| Cautious Survivor | 28eb696f-a42c-4444-8139-dfd62029e020#card | 16 | 36 | PossessiveNounPhrase | b66e11541f9d24b8becc5a47969a6b147f34bb1d09a2ffc99b46a41eb0dcd6bc |
| Cavalry Master | 7083dedd-b246-4fe0-b1ca-00c490ed9c6b#card | 6 | 25 | KeywordComplementPreposition | 00b1b4a821224bdecee3f46000453f50c20a37b1374c02c162a5f3cb77197a13 |
| Caves of Koilos | 33de01e9-ce5a-42d4-afcb-343cd54a6d80#card | 4 | 47 | ManaPhraseCoordination | 15443e52f2c2faf12fa7655504ef99d41dbea2bab47bf28d92b5b7e73cc44b91 |
| Celestial Purge | ec1f6188-2516-46ac-8a03-7b7285b23a62#card | 1 | 15 | AdjectivePhraseCoordination | 474ccf19148d351efb31ea57667c7462eeaee053e0118f0a644428beed4c8e0a |
| Centaur Archer | d48eb6ee-d4ca-4bc7-8d7b-0e6261b88ae5#card | 5 | 26 | KeywordComplementPreposition | 3599d0b6cdd97afdc0796d4b5b04bd558a2594cf8a8c9fe33d9bbb12fe18ca07 |
| Cephalid Aristocrat | 7a0514d0-db8f-402d-abf1-cddda87405cf#card | 2 | 30 | NominalCoordination | 7f4c8f0c5feb260d356270e55011a8e471c4dee9266ceab8ef6c33072cf2ca7f |
| Cephalid Retainer | 9a0a926c-59ab-4bd3-a238-13988bdd96a5#card | 3 | 19 | KeywordComplementPreposition | 466e61cfbb0a26cfe127dfde1151b2eb602a5e9ba1bc054d49f91b949a05b37e |
| Certain Death | 1d4b8a31-6500-4e3d-bcaf-f6f184d7743e#card | 2 | 32 | PossessiveNounPhrase | 9e887671931f6461bd791ad6b15d1b9d5c405f80bf43d46df07818f88aa38181 |
| Chainer's Edict | 7975cb9d-d37e-44f8-9614-86d2ad453c35#card | 1 | 26 | PossessiveNounPhrase | 31d1452cb0255b091ce3a684714a1e21b3b8b550ddde0dd8cca3c993729e3edc |
| Chandra's Magmutt | a2c4537e-70a6-416d-9e8b-f549ad535147#card | 2 | 25 | NominalCoordination | 868538f6bee688310f10c8f801ede5e081181f0b87e32d3e2500d52f8fe211ac |
| Chaoslace | 08842aa3-f923-46e9-a106-f542331e9cc1#card | 1 | 15 | NominalCoordination | 150d12ef350ce0a61af6e586f81a51f82ae8a82da946cd5893ced741c6701cc4 |
| Chart a Course | 05878e49-93ad-4144-9c50-a0bb86126c2e#card | 8 | 32 | Adverb, InitialAdverb | 069e65d186ffc67402ad81ce2a015046029819df04d6388f66c21826f59c2a45 |
| Chase Stein, Runaway | 161ff063-b784-43c7-b608-f78be3daa5b2#card | 16 | 55 | PossessiveNounPhrase | 0d9cd2ff0e1c78daed6aa101b4e1fc6d65fc406b46f1aa7292bddcbdbc1e44af |
| Child of Gaea | d4e8aa93-c0d1-49b7-bbf4-35a155048774#card | 16 | 49 | PossessiveNounPhrase | 291bd27eed481d52b9cecbeef9bdc3af738801a03d3e8838eb31fce9db71e587 |
| Chimil, the Inner Sun | 119d9671-61ba-4629-89b1-f94bdec5cb74#card | 4 | 41 | PossessiveNounPhrase | 46a9ebabc95674867543e7bcfe34615b6053963c1b04fb7373b77e63f3c1aa66 |
| Cinder Giant | 3a8306ae-3162-4105-95f4-dacd10c7c051#card | 24 | 36 | PossessiveNounPhrase | 1998108f37c95bf2435b625eeecac7a204c1ca12534d1277dfbdc46725dc1006 |
| Cinder Hellion | 9e1ad977-2814-4336-93ec-57b7736a629f#card | 3 | 30 | NominalCoordination | 4651253d0c1d5d588b5889cacf45e2242a8775beda2dbd6addde0da9b0a28ebb |
| Cinder Marsh | 6f8cc374-e76c-4bfa-bf20-28dea0bfefbe#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 02c37b8955e04c0dbefb1671aa34aa7c3a4e91fbd9a2020d8918af054c4347b4 |
| Cinder Pyromancer | 47bfee3e-34b4-4b89-aaf8-f74e0c3d3c62#card | 16 | 52 | NominalCoordination | 038e81fe453fa26e03671553c0377479dc26883dff13c9aca4d077dbd7b35a98 |
| Circle of Flame | c31801e6-2444-4158-8e19-2afe751d7cff#card | 12 | 39 | KeywordComplementPreposition | 3e4747fa85a1b794fd71a9d3eb24072bd36b971fd1f6d452151be386e919dc71 |
| Citizen's Arrest | 6f554418-9ee6-4a03-b81e-f93d5c70873e#card | 9 | 36 | NominalCoordination | 25db48bf7b4912664c956cd511bcfde45651b2c0a5dcd4864bb5339934438d72 |
| Civic Gardener | 9eb7f301-b9b6-48c3-9dad-e7df9cc15355#card | 1 | 20 | NominalCoordination | d2a7d458fdca94270c3bf30529f2274b1c5b6728c58ce9506dab7cf25a242532 |
| Clear a Path | 05a1575c-8d08-4845-8ae0-85e0aba55637#card | 3 | 14 | KeywordComplementPreposition | 520df4a6c42fa18fff6f0f27bbbd83964de9f3231430a76b27c86b0baede7158 |
| Clear the Mind | 141c33be-7a25-4981-b62c-efee8e8ec91d#card | 3 | 30 | PossessiveNounPhrase | 1e7818088624bb48d7875130f440f951e021c1b433539587c23fb0bde71c046f |
| Clip Wings | cefa28e2-6a95-4aca-86f4-b0b741f61939#card | 4 | 22 | KeywordComplementPreposition, PossessiveNounPhrase | 004fa3bad4282509f07e8b2d9f0e3750991b6fec862afe6516b672d60fbc17d1 |
| Clockwork Percussionist | 3b74c8ac-9c19-45b0-935f-8630b8da4543#card | 48 | 53 | PossessiveNounPhrase | 13a3defcbab4e9699a59304f1687a5d23c259f37bca1668031963891db9e4c32 |
| Cloistered Youth | 7552a9b4-b82f-491f-ad52-e271cf730211#face:0 | 4 | 26 | PossessiveNounPhrase | 8656aaf34c18e08de0b7cefd5b9f4ddc49f2a0284cffb2007c47263dee89a3da |
| Cloudcrest Lake | 8df14d53-472c-416e-93c6-6c0b7f9b614e#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 09f446cbbc4b58e4632f4a2fdf81112da5b21b21bdd3a46fd22d27f4201e0425 |
| Cloudthresher | ab0e2aa0-1ddd-4a4c-b7cd-4063caaa8bcd#card | 3 | 44 | KeywordComplementPreposition | 482b71cf2a7daa211ccfe4f8622fb8bd391fe2efabef6160d7da3540ba7d2016 |
| Coal Stoker | 7f3f7fb0-da18-467b-bf57-2ded458fda4d#card | 8 | 32 | PossessiveNounPhrase | 12c966d3687099edd0e6efa96708c17890fcda7ef85b867cf1b89bf9a8765ea0 |
| Coercion | 68413337-ddb9-46f8-8c8e-f3d2d672c652#card | 6 | 41 | PossessiveNounPhrase | 0b078754b9355ae2fee5d2e29b8b82089313cf5b2b479d7ee6c71fb88946533d |
| Coliseum Behemoth | 076cf4f1-b3d4-4175-b976-938ad851cc0e#card | 1 | 42 | NominalCoordination | 7a07d541665435686c7286839ff791a073579951e1019a30880b5fa766691b05 |
| Compleated Huntmaster | 65b8bb9b-91f9-44e2-aa6e-e82e3760a4d9#card | 1 | 26 | NominalCoordination | 8552f6313662a35cabf5a79bfc19c2ee2ce96d651da6fedaf8426f2a163959ab |
| Conclave Naturalists | cb5de977-d420-41cc-85f0-6ffee8bbeff1#card | 2 | 25 | NominalCoordination | 1182f563cb58f7369ac258af89bf2d710fa4421d228f040a07fb502ff91602ca |
| Conjured Currency | 427ba80d-cfdf-4651-a1eb-89a2125a9217#card | 24 | 41 | NeitherFiniteObjectGapCoordination, PossessiveNounPhrase | 05a003ea2a137d7204ea81667e25682910be6bcaad4086bbd66bebc689c57cab |
| Consulate Crackdown | 128f7e5a-17e0-4069-a185-c9f874c7d234#card | 8 | 34 | PossessiveNounPhrase | 158998bd7430875913703775c231323985268f5b43357d64fa1110c8c5dc8cb8 |
| Consuming Sepulcher | 883a4180-9ede-4249-a4b8-3a29c998fb63#face:1 | 8 | 33 | PossessiveNounPhrase | 430a51da9f038927551d9ecc53a6e9ea46edb0df5950f2088c403b47e1be7877 |
| Conversion | a24e05fb-dffb-4400-b4ca-22fdde45e7a7#card | 16 | 45 | PossessiveNounPhrase | 15e1efc4438772c1e431d3a5fb44b58420ef4d3b30ad9743987f578678fb1cfa |
| Corridor Monitor | 07b6ee55-a77e-4ba7-9ab7-8a8375c7acc9#card | 4 | 25 | NominalCoordination | 0de91792b58600d3df8c26dcb7254cb31fca2da65a10e10325e1bf3db0069d50 |
| Corrupted Shapeshifter | 5c4cbf1c-95ee-4555-8351-ea962e17ae1f#card | 108 | 62 | KeywordComplementPreposition, PossessiveNounPhrase | 05d85eb6b8ffe2b588acc67e9cce4507a68cb981230e497ff78869a6c465b640 |
| Cosmic Larva | 97428d25-d850-4a98-8852-9440e06a9091#card | 16 | 34 | PossessiveNounPhrase | 0fbbb6d0dbdcca8c61ff6bba73e8a4553e28c581fec9f2c424e71e0fedd6d0f7 |
| Council's Deliberation | 3280488d-e94b-4d53-b279-c32ed711d4bc#card | 384 | 64 | PossessiveNounPhrase | 0013c075b5690207322716143ec2721ec624d2606d9e754a0d200736ff58b04e |
| Countermand | 43402a6c-9fba-4a80-90b3-14bac05c574b#card | 1 | 24 | PossessiveNounPhrase | c8e59d6fe3d58264a69f1fbbfe2f0ad054aa8ea205a071f1fd771474ef48bbc4 |
| Crack in Time | a3099143-0146-435f-8d29-cd364aefbc2b#card | 32 | 50 | PossessiveNounPhrase, PrepositionPhraseCoordination | 1866d9d6f04522ad164c912ef3c73dcb143d1df79a21baadcbff81134929cf56 |
| Crack the Earth | eb107601-f4ff-4504-9e3f-3de63b0d9e6b#card | 1 | 19 | PossessiveNounPhrase | 1f69879426902aa13a45674ee087149d67e950493f05b515bd7a15fe1bcebb73 |
| Cranial Archive | 260f95c5-4058-4627-981f-98b9eafc2265#card | 3 | 39 | PossessiveNounPhrase | 28ec5473c9d5704a6df7ebb54ca4f12850364434270dc8f74c43a7222c5c5e29 |
| Craven Hulk | 3cd13364-3ee8-4a00-986a-489b0ccf90c7#card | 2 | 16 | Adverb, SecondaryAdverb | a97e19ec9816ebdcb14b1a732184e4936e367d7c0ed71fc2247fba5f4cdf0b4b |
| Creeping Mold | 59180e94-ccdf-4d9f-9a4a-fe55497d0d63#card | 1 | 15 | NominalSeriesEnd, SerialNominal | 1f644aaa99011a3ac8b1618fb2c9958e269fca6bccf72b1ec3c936b19396fe86 |
| Crimson Operative | d8863742-6c1c-415a-ad27-7def6d119c6a#card | 16 | 54 | PossessiveNounPhrase | 072dd5a65d8ec1f51cbee0ed96d1feaeac70fd68c9ea0660c42e15cde472f426 |
| Crosswinds | 68d7fafd-15f4-46fe-8233-09c3c873d1a1#card | 1 | 19 | KeywordComplementPreposition | 46d7793578fff777d1c71de04a8a6595e21210c97e5db7724954983610e00306 |
| Crucible of Worlds | 33c722cf-b4bf-431f-aefd-ee96241a7fbf#card | 16 | 21 | PossessiveNounPhrase | 08317b437d0cd2064872bf5abc693c38eb130b8f8cfc44ad9377c27b5fd88284 |
| Cruel Celebrant | 3ee78cfc-0e9e-4737-a7e2-b42f94228040#card | 16 | 40 | NominalCoordination | 28106f1702125b19a73751891bde8285b8a7dc12e88a0b928f958d2d7cfcf971 |
| Cruel Edict | 10c585c4-bf5b-4d8f-94a9-e9a5036a688f#card | 1 | 19 | PossessiveNounPhrase | 4d8c702835d78403302ba5ddec30142b4b33bd4aa090d8a8fc9cbeb286514339 |
| Crumbling Ashes | 2e4030eb-c74f-4f81-8d35-8f1d080b15cf#card | 36 | 36 | PossessiveNounPhrase | 04199f5de521388685f295fb92564466bed6087f97f72ef76d7543e6bbf47113 |
| Crushing Canopy | 618cd1bc-4422-441b-906f-1a209277be93#card | 3 | 33 | KeywordComplementPreposition | 0a77764f8b2e0e70bc02a5054f73dc27c6d39226a43e7b31396492078be8213f |
| Crushing Vines | aebe0ba5-70b2-406b-b53b-03b12a95c2c2#card | 3 | 33 | KeywordComplementPreposition | 493bc6b58ee4819eec3c3debf62c2c3aa07c88d6542a6e63ce725ef2c441d591 |
| Crypt of the Eternals | cc78776b-822b-4f11-8982-0805a25a9d36#card | 2 | 58 | ManaPhraseSeriesEnd, SerialManaPhrase | 193e88f855bfba52060683403a108708fa5365de58a95b39d78694dc7f22e985 |
| Cumber Stone | 49199997-65cd-4cb9-bdd6-74c93c1f6b5a#card | 2 | 22 | PossessiveNounPhrase | 2d0270248a3d2e64cb1c5dcea235fff9b628116cd71bacc070d24ad96284b905 |
| Cunning Lethemancer | d696aba9-ef3e-4f81-821a-c1df51431b8a#card | 2 | 24 | PossessiveNounPhrase | c32b54535eb4f8f864648fd058c6b45f2911f8075fa1aabc1e4dc67b43aee1c0 |
| Daemogoth Woe-Eater | 290323d7-d8cb-4706-87d9-b920a4e2f680#card | 32 | 62 | PossessiveNounPhrase | 04d2a1d82d8730443d70e746a841f7854df0193ddc0e943fa183fed0ca4d5b5c |
| Dagger Caster | f1352a81-6fd5-4c4d-ada4-b81d3a9f74c0#card | 12 | 39 | PossessiveNounPhrase | 1b2c665778f15e8ae84db9a5b668bce93a4c917cb3965ecda7d158181db67ad2 |
| Daggersail Aeronaut | 7c0ef3b7-b368-4a5b-8cbe-bf1f47610f74#card | 2 | 17 | PossessiveNounPhrase | 00f3ee8b23727d692d147c96dd2cb94c0ec86611071745579e90a259405a6d89 |
| Dampening Pulse | fe3c81b8-9bca-4760-b677-02bfa7e6137b#card | 2 | 22 | PossessiveNounPhrase | 2d0270248a3d2e64cb1c5dcea235fff9b628116cd71bacc070d24ad96284b905 |
| Darba | c64c0cb5-5a8c-4a2f-a0fc-8b080aaf90bb#card | 16 | 30 | PossessiveNounPhrase | 092e1c75809a1b23fd0327feaba8903b84faa7b60a3b31be221d4abe71e5000f |
| Daring Demolition | 8c8a3962-42a5-4070-8719-00d7483c7dbe#card | 1 | 13 | NominalCoordination | eedea1cb737dd130ee76a101cdf4f7f3389b133152c8c58c2812429a544b67e1 |
| Dauthi Cutthroat | 5fdde593-54cb-4186-974b-df4383f7efe2#card | 3 | 26 | KeywordComplementPreposition | 4ddfdf061ad29138894c0fe77e4ec4881ac6c248bb5f18caf30f8eeae8975016 |
| Dead Drop | 09f3d60c-34ee-41ec-a047-fe2140b11950#card | 1 | 23 | PossessiveNounPhrase | b6644a2e3660a1c7ae7ec480168179e62c3a37ba9cf1edbe9492804caeb5a0c2 |
| Deathlace | fb80aaba-352a-4b58-8db2-1e02d542819c#card | 1 | 15 | NominalCoordination | a49225717d54a2eb92e1dfd1c6c5f54dc3a265f8ad95de3b500db67acc8cd02e |
| Deathmark | cf09b0af-3cf1-4486-8f65-8cfd2410314a#card | 1 | 15 | AdjectivePhraseCoordination | b8333a505fa25174caf2f2f6ea1a4c379967edf080a655700e0c7dec7ca9e588 |
| Deepchannel Duelist | c892d28c-4a78-4a7f-b8d5-c3663b819c5e#card | 64 | 49 | PossessiveNounPhrase | 1b385c159dccadd88970ceb18185f3ad353c9baeb8cc223a75d2f73a5cae0df9 |
| Deface | 390290b3-77cc-4180-8e6c-9cb808f6ac7a#card | 3 | 33 | KeywordComplementPreposition | 88ab69dfd2568443679ada9f9af391ab2f7eb57e34dd517b10bf43f54a7c5897 |
| Defend the Celestus | c079a636-cfd4-49cd-bcd4-54375b3fa355#card | 22 | 34 | CardinalSeriesEnd, SerialCardinal | 05b4de0441687a0645dea389867cd0526a77aff7a46158891850b282835828f8 |
| Defenestrate | caf840e8-c1d3-4570-8542-432a765c09ff#card | 3 | 14 | KeywordComplementPreposition | 66bb7ca33e0d09ad7608c1ae36e4f7f57ebec42871ef3c11e38a4b24119bc7ab |
| Defiant Survivor | 7b62c53a-46a2-4c57-9eab-0c6706795b4e#card | 8 | 31 | PossessiveNounPhrase | 4af6b3cd3d4c38910f24e674b360a0761635e15f94188932d087c840109691fa |
| Deglamer | 049d2e18-8fa9-44d9-8618-f675592cb624#card | 2 | 29 | NominalCoordination, PossessiveNounPhrase | 42617e6aac5711e87e3b3bf9e04ee809bd1803945521f0ebb27051e2390abbec |
| Deluge | cd271e61-0135-484f-ae02-5aaca57c1124#card | 3 | 14 | KeywordComplementPreposition | 13e2bc8abd47906a9a4725044ebdcec02e207657da6fd6d6fd9110cbe8741f01 |
| Demanding Dragon | c8f1e3bd-bbf0-4750-b054-03553fc61850#card | 12 | 43 | PossessiveNounPhrase | 3fddbd828fb0f2f13d0d58a9daf40a224a9cbfec0841bc44fc8476f7ed592033 |
| Demolish | 08c6d2fd-d9ae-4f95-bb9d-7e16b1039814#card | 1 | 13 | NominalCoordination | cabe4a78ae86e4063d0bf611d0f25ab197368cff34283b0b2044fa93515eb159 |
| Demon's Disciple | d5a33091-a348-4b13-8dbd-79ab0ad99afe#card | 2 | 28 | NominalCoordination, PossessiveNounPhrase | 58065b0a98b37a36b1e620b55ba15fa195ca16cbca07dd611d559f5cde84123e |
| Demonic Lore | 47a9df32-7588-4430-a40e-56a9041e0360#card | 72 | 52 | PossessiveNounPhrase | 079bd91a7b48b412eee24baad57ee18e2a151783849b1770a4c8bf3504febf26 |
| Desecrated Earth | 3c9242fd-915b-4f6d-b4ee-830c9d80c9e8#card | 1 | 23 | PossessiveNounPhrase | a8b5c1f802d1c6c6d07a6ee1ab02fd3209545c6becd276a06c0e3d1da02c331b |
| Desecration Plague | 131d55d1-7429-4940-b7d2-d759aabff47b#card | 1 | 13 | NominalCoordination | 0e8c68ac859abbd89fa0b5984406d75648fb24604dba0f4cd88f29c3f00a1d65 |
| Despoil | 77fa18fa-5dba-4e4c-9b24-9bfa9b68779f#card | 1 | 23 | PossessiveNounPhrase | 8a55b2cb600c6359fd413e1034b15bc1ed4b8516a05a114a45ef3f81742190dc |
| Destructive Digger | 56787f7d-d9e9-42f2-a419-31f15e6e436e#card | 1 | 28 | NominalCoordination | cdbbec1f7d46a16ff335d06ec29ab73f8cec2dd6270b6508498ce0e8b769112e |
| Destructive Tampering | e6ae536a-95c0-495d-98b8-200b3564dd79#card | 1 | 39 | KeywordComplementPreposition | 94138e7ed22ba83f4fd30f6458ac7fc1bec094995b465c15f73100cdb2bb1762 |
| Detention Chariot | 0ea2c5ad-2982-4041-8c9d-a1d31ced11aa#card | 9 | 45 | NominalCoordination | 03ab119c263e9535b3c02eefe10e21dde65ba290b93dc9d4308e8383626785f3 |
| Devoted Grafkeeper | 0a154fb2-9f23-4c22-baee-728492385d6d#face:0 | 32 | 57 | PossessiveNounPhrase | 07e41d1892e993b1a2cd2aa2157435864b2dbeb40a9668b93213622d6b586584 |
| Devouring Strossus | 8f2ab0a8-9d60-493b-a284-9ed7fa07e5ea#card | 2 | 44 | PossessiveNounPhrase | 2cdfd5aff751c20f91a222ae357d9fe4d73bf3780e45ed01cee643b6e09a9bcd |
| Devouring Sugarmaw | c8bc28ea-5630-49d5-963a-5789fa83f3c7#face:0 | 16 | 52 | NominalSeriesEnd, PossessiveNounPhrase, SerialNominal | 014702d3c7086359ef0902f2b76900414202d863dc8650428c5c112f4eeaf1d2 |
| Devout Witness | 83647f5b-2b0f-4f9d-83c5-75c52460f35b#card | 1 | 29 | NominalCoordination | 73b7e7ab916d541d4ba32fcb4f38a93b2d14741248053cb3f32eb9485e7d2299 |
| Diabolic Edict | 058917c1-21ab-488a-9f9c-591c55f3c596#card | 1 | 19 | PossessiveNounPhrase | 9f98c2d33c9a284e37bfc9be81eb6785202c950e0d92e5b4a2546daf6177c196 |
| Dictate of Erebos | 7c777a41-e40a-4b40-96bf-8ddd5c12924c#card | 2 | 34 | PossessiveNounPhrase | a9106ddde4c4a80ebad6ee74105d802bea87cea79ca772e3b96e896b213b9f73 |
| Didn't Say Please | b90bc464-a95a-42e4-9d9a-4b0882eb57ba#card | 1 | 24 | PossessiveNounPhrase | 6aea81d4701e511e5f13334b3097003ba08595bfcfc2eaad1ce9f8b326e468e9 |
| Dimir Cluestone | 62f834c7-4765-4b6f-812c-7ce14e13ea0d#card | 1 | 44 | ManaPhraseCoordination | 3d78f24ccc99c46dac4ede7a314eff693efe42f41661de4d1d3f1f750fd19c45 |
| Diplomatic Escort | 55cc14d0-11ed-444d-884a-cfb692ba632b#card | 2 | 34 | NominalCoordination | 0bc98c625112ba8d511561fb0bfc23d7b896add3170478fdf87c3d9644bfcd35 |
| Disenchant | a7e97fa9-4b72-4548-b854-5be5f18a6f1a#card | 1 | 13 | NominalCoordination | bc97d4e47a24480cb6c1f41fdf441d4cc53efea11b4f2764b359754e0e2e37df |
| Dismal Failure | 1feeb881-ebe6-4afb-ba31-3d116d8ab7a1#card | 1 | 23 | PossessiveNounPhrase | 6f04a40821d7a5f972c9c0c8cff2caf66b9963c7f4f7b02408ff674a72259931 |
| Dismantling Blow | 300cba5a-adbb-4852-be06-ac00d9d6fd37#card | 1 | 38 | NominalCoordination | 74ca0399b73d85cf083043bfdfc6c68bacd1f4c243db55dede046ffe9c4dcd3e |
| Dispeller's Capsule | 91fc862d-fdde-45ea-a05d-30d38e7a735c#card | 1 | 29 | NominalCoordination | 8369398665aaec3fed25473bb083f6ecb81bfeeb46c103722f5838a1ac412c78 |
| Disturbing Mirth | 7010bfbb-8aea-473b-a9a5-27488de13cc5#card | 16 | 60 | NominalCoordination | 12bfa0c000c32de9eef22e05f6b4d2be377c78123800ee6d43b45ee48baec1c1 |
| Dockside Chef | fed12a16-8920-403c-be63-0601a9d864b0#card | 1 | 25 | NominalCoordination | 4af145ce8bb3afaf019e9428ed9042434581fe953189108764763118fc8fef6f |
| Doctor Doom, Unrivaled | cb3dd50b-ce78-4727-b174-06f99fc5ec74#card | 32 | 52 | Adverb, InitialAdverb, PossessiveNounPhrase | 09e3e0542154757b6367676a229e5e6466826b099e8fae3e16f5ebf0017f1e97 |
| Dogged Pursuit | 987f8c13-0164-4536-b505-d9fa799222fb#card | 8 | 33 | PossessiveNounPhrase | 024f2bcf5900f8a4f97e32068f348bb3e6b9bae5100011d36e09ca0ab51cec72 |
| Dragon Appeasement | d3cb9b08-16d2-4574-afb5-068239dbd152#card | 8 | 36 | PossessiveNounPhrase | 2739fd34c85c206865eed5c221856ca3438c80b2855ba097125e6f397ab7cc79 |
| Dragon's Desire | caef715c-3ca6-4f56-9dd5-aebce9cd0621#card | 4 | 21 | PossessiveNounPhrase | 0ddb791edd01690e8db56848cbe528e6c09aa7faf580b3666906762ab807c4e5 |
| Dragonlord Dromoka | 82cdd612-3322-4372-810f-1ff106ea8e6a#card | 16 | 44 | PossessiveNounPhrase | 351cd86d951f172161711362dd1e4e4d15f4ae4ef0f0bc9aa5769df790490f1d |
| Drake-Skull Cameo | 8fbdec25-4222-4b96-aeea-82a4b5b8b80e#card | 1 | 17 | ManaPhraseCoordination | 95875089384c14f2f821661b2262fbc9b296a3e409b8a20233afb8275f66a30a |
| Drana's Emissary | ef49b53d-e5b6-423a-88f0-51a7b2021c46#card | 8 | 36 | PossessiveNounPhrase | 01f4faab270f6db55a1507d90239b56e409705a7475e9f95b8a87a0822ee3bf3 |
| Dread Slag | 4fd87eb8-624b-48f4-b6f0-ee2e09f91387#card | 10 | 29 | PossessiveNounPhrase | 026c01c3de9586cc39e1c4c2bab6313f8ebdf8902a76646e3341cd48317192b2 |
| Dreadbore | d685799f-cc1a-40d6-9df6-d05b8b1f5b13#card | 1 | 13 | NominalCoordination | 9381f96ed17c2802eb05677c0ed02d066b0ff020d2e6011d9b0da634adc35dab |
| Dream Fracture | c5843d13-855f-41dd-ac13-3c7f7e18bb39#card | 1 | 34 | PossessiveNounPhrase | bce7790e3ed5a0b401b500d72594df43966e5ed6f65879735432fe0752289afe |
| Dredge | efc9498a-b740-44de-aef3-01a1fa4ee49a#card | 1 | 24 | NominalCoordination | 1d5c69fb0e783b3b6c7e3029420f5b77e8b4b1e3ff415994bfe6d835e5f9b6d7 |
| Dreg Recycler | 556c4dad-002e-41a8-8ba3-8b4eda70232f#card | 2 | 36 | NominalCoordination | 3f20ee4ff5cf49c98825c86c877b56281d13dbf03d6089854cdc4f50e4432d47 |
| Drifter il-Dal | dbe8ad46-d862-4ff6-bf17-238fb8341075#card | 16 | 32 | PossessiveNounPhrase | 153d76d0865922d841e4490ad264bb33f34d9772a347e571c2c79da66962b0ea |
| Dromoka Dunecaster | 5ac44e0a-782f-49ee-b263-2fb65359faaa#card | 3 | 23 | KeywordComplementPreposition | 0941b3f16f5191bb9e207c10104d9bfa9ec43770a40d6ce1500707495df44f4b |
| Druid of the Anima | 8d7933c4-4c74-492f-a9cd-50b339a4b29e#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | cb7397c9691adbe280718ba4eb685de7e7f757391eb09747295dd33ef7fc3e9e |
| Duelist of Deep Faith | 30d69e88-2bf6-43b8-8678-2cf37f6a5cf5#card | 2 | 21 | PossessiveNounPhrase | 0d317b5f7b266709224fbb808c80e074c2a6beabc08e58cc626ca2e92251454d |
| Duskwalker | 689f8696-dd13-4e1e-bdbc-3794c524f423#card | 18 | 44 | KeywordComplementPreposition, PrepositionPhraseCoordination | 0983dcd207364a072b4df1ec0b952c31ee7c31afca9d2dddb144a915b1b4bf9e |
| East-Mark Cavalier | 884d08bf-7e97-40cc-864e-445ae0bd1646#card | 2 | 31 | NominalCoordination | 21dec3a0b7a8454017dfab9d75f2f755f11d63267759d02c9e77dd50a7d9ff4d |
| Eat to Extinction | bf5f348e-d748-48e2-b483-740109e71176#card | 1 | 20 | NominalCoordination | 0141eade7b2aca4eb7298d38be2ff1bdd85ac36c17668636965c18124464bf79 |
| Elaborate Firecannon | 6e61ae7b-f2e8-421c-870b-a48c08d07503#card | 384 | 89 | PossessiveNounPhrase | 0363f418c5573265d5a3486c0889cb31feb9476e3e890c9c899fc1e091b528e3 |
| Elder Druid | a440a4f7-843f-4186-8ca9-df99f6aa321e#card | 2 | 32 | NominalSeriesEnd, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement, SerialNominal | aa1db0265bfa5f2403d002b6fa1987a13f30bdd4db0621b4b503b74df5c45374 |
| Elder Spawn | 363b2213-4c70-4cb0-b895-81393c1082a7#card | 384 | 67 | PossessiveNounPhrase | 0074a7cfe8d8dcb0b71778daa0a69f8b138b780d3595aae22969515e7c274caf |
| Eldrazi Monument | c7ee1b21-d02e-49ef-95d6-d6dbf2f886ae#card | 32 | 64 | KeywordPhraseCoordination, PossessiveNounPhrase | 006c09cad9d050f1d8f3b5ebde05897c1dab79554820eda596bc811eb4d5cc9b |
| Elesh Norn, Grand Cenobite | 958d71ff-c9f7-46f0-96ca-79e7f4d65a16#card | 8 | 48 | PossessiveNounPhrase | 0c592655368145570ad3eba511cceca15f07e2ac790507ec5cd0cf4075573e04 |
| Elkin Bottle | 1e381361-7404-4654-bfb4-5ea65f6e4dce#card | 16 | 52 | PossessiveNounPhrase | 036070d73df31dc84887eea92a69cdc9410cd7e1bcb96494a07bd8b59985bbc3 |
| Elven Rite | 755dbf29-7484-444f-b5df-29ec4b4bbc82#card | 4 | 27 | CardinalCoordination | 60dda7d6c23e8d5b36c166208f3979b9e6179d8d10f57d1d73fb2c74058c9b5b |
| Elvish Skysweeper | f557697a-4c1b-44f6-a989-4879bbe26eb5#card | 3 | 26 | KeywordComplementPreposition | 0f4957287360368e87373938a4905d4f037889f8b5b91f8746e52744189fcc8a |
| Elvish Spirit Guide | 6b0e23cf-7d68-4329-86db-7adc26abd86b#card | 4 | 22 | PossessiveNounPhrase | 2175a7394093770c480508d8726b76e511ff6d31ae3a73b9462981d62ef2d15e |
| Ember Beast | 58edb282-a124-47bc-b123-1c34b6804f48#card | 4 | 18 | Adverb, SecondaryAdverb | 7e3bab0b1502dac18f9404ee792ed91d467b7f13797407bc0f6d5d2d95e44397 |
| Empty City Ruse | b0391ea4-e37d-48c9-91df-79aefe376bcc#card | 1 | 21 | PossessiveNounPhrase | fa2b01e28f8bb874413512ea29e208a232b169005e325f93148f31eba5b535fe |
| Empyrean Eagle | 270d14b2-07bc-46bc-918f-658102265ccf#card | 6 | 29 | KeywordComplementPreposition | 408002378d53652f608325414e374012221f1ac088f6e58e4b77904f3c525fda |
| Enchanted Evening | c56ea5ce-c8de-46c2-a8dd-bdd41467a28c#card | 9 | 27 | PossessiveNounPhrase | 1d205a9a7e246d1a99dad48bf6fa3d1f509364172ce8383de40f71e343cf4772 |
| Endless Wurm | ce4d714c-c22b-47be-a372-a3139c4e5b1c#card | 16 | 33 | PossessiveNounPhrase | 18237c5bfc31418bc11fe518e897de93da789e8a6b4ecf6dfb019b4f78d121fd |
| Eon Hub | 84342991-b925-4064-a88f-b6d60fd931e7#card | 1 | 14 | PossessiveNounPhrase | b69a1cc6a382c3b700b55751c6d3327a476d31c286a38ba7570643e0ebd8fa1e |
| Ertai, the Corrupted | 36934bd0-b275-4222-926c-b5a74cf0967d#card | 1 | 28 | NominalCoordination | 4d31c6ff6f5e16ddd710a78294808efd721323fc98fa71c3e5d72d01aeb7d7dd |
| Esper Sojourners | 0c59db70-9cb4-4edf-ba6e-12ffa7c06b7b#card | 4 | 41 | PrepositionPhraseCoordination, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 24eece274ddb5c3d5452c20d460bb8088f421ec1479821bf8a071f68f15a1e3e |
| Ethersworn Adjudicator | ca2a7669-4932-4a78-8a6c-8558b7c05ba5#card | 1 | 42 | NominalCoordination | 9a54e94d45fb12499240a95ee22500618189df4c0e4e3221845b9bf8d0d70c89 |
| Etherwrought Page | f1c87913-9374-41d7-ae70-444d427290b4#card | 4 | 54 | PossessiveNounPhrase | 1371ad41cbfedd43633c3abf2ee34d00667b0709f7befb7512e14675731b9080 |
| Ever-Watching Threshold | c6639b52-b51d-46e7-a83c-c4621b1cecaa#card | 4 | 34 |  | 04219f96b47b63051b4332eb54894df3329a7244fd97361dab6c2677377f0192 |
| Exhaustion | 0e7b9caf-8285-4386-98bc-9a809827f447#card | 18 | 29 | NominalCoordination, PossessiveNounPhrase | 11b2663ed4c7b9031f916c48df479d11b24f6a967a2642027c335d3059dca35b |
| Expose to Daylight | c8195fcd-1858-4f23-aa73-b654d25035aa#card | 1 | 21 | NominalCoordination | 3553966e17bfd22ac5078492b79d42ffcd581c3af339542bff9aae7210d67f64 |
| Fade into Antiquity | 3bd3156f-dbc7-48f7-9e50-1f2b8b227f97#card | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| Faerie Dreamthief | d6e68414-80cb-4e08-ae84-b084a3f4b6de#card | 16 | 58 | PossessiveNounPhrase | 077a45b96d742442b6e306a24ecfb8b73eeaae1a3f021ca6f79651d800138260 |
| Faerie Squadron | 109f7a7d-dc08-4c8f-8065-93c620a9194e#card | 18 | 44 | KeywordComplementPreposition, PrepositionPhraseCoordination | 156c98b11b0eb922474be897a1856da0969711133ccd62694938b40b6fee46db |
| False Memories | d95dd8cc-ce7d-4d43-afbd-efaa2d3c7a35#card | 8 | 39 | PossessiveNounPhrase | 094e564be7ec9eb24e88dccc8947efd25e70e699fc5a1e6e24383ac56d61d4ce |
| False Peace | 7962db58-dbd9-4b94-8a21-a1625da4c384#card | 1 | 21 | PossessiveNounPhrase | 4a6830d81cc1993d2d4d2b2c62d03c171288641eb9b849e908fe046bacbe918b |
| Falter | d4b50749-a016-4aff-8d70-a1707cabf57b#card | 1 | 20 | KeywordComplementPreposition | 9794bdd1b02ddc40b70c691ec972bd65b26cb16e43744fb297ee0cb5561ee1c2 |
| Famished Paladin | f80df9c4-f682-4a70-81fe-03c7c5c256e1#card | 24 | 39 | PossessiveNounPhrase | 315d0726072d870a2397ce70e70764c1bcb558634fb720c3b5a7f1a072d3f051 |
| Fate Forgotten | 7f591bfb-a241-4980-9cb3-fb5af3355f6b#card | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| Fateful Absence | 35bba442-1aec-4d33-b502-4c580d61644b#card | 1 | 22 | NominalCoordination, PossessiveNounPhrase | 94304d917a925135f5010b5876a587d46926787a1a70c56e785058258f28d57f |
| Fatestitcher | 55765934-07be-4f03-a2e8-8292c0b601ae#card | 2 | 29 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 190775b0b2bdd8337ae78024131013deb4c7b51c920710e0ab42dfbfd77fa934 |
| Fathom Feeder | 0767fb65-a621-494b-97a4-f23a4c8b4bec#card | 2 | 45 | PossessiveNounPhrase | 9cbe631840a7cc2deb5bb7a8a6d70dab67aec560e703be06f117731d47249923 |
| Fatigue | 0a88dcb6-a391-408f-8bfc-7b5b2cc34267#card | 1 | 16 | PossessiveNounPhrase | 56dba1e2bae8f68ce645e4123b568cbff5ae30d3fcaecadb7d8d3772327d9ff9 |
| Favorable Winds | 2361ca87-6352-4ba3-8d91-b3d71242914d#card | 2 | 24 | KeywordComplementPreposition | be76acc64d54d3a81ddcc5c36f4ac81a020e02542eb9b3bfa3b4d272c1a553d4 |
| Fear of Impostors | 628ae0e4-5d54-48a1-a24c-b9c1d862df9d#card | 1 | 30 | PossessiveNounPhrase | 153aa7e4b22f5a22879e7d772a3c4513eba58bbd098a42f710b74259c287c5f2 |
| Feast of Worms | 905889fb-a123-4f7a-afc6-b4c6ccac9105#card | 1 | 37 | PossessiveNounPhrase | 61c7b2ba4eb28be4081fcb2ac50cf45193b5386a2b75b194309c3ccc651ccb47 |
| Feed the Serpent | ad404b46-dee7-41bb-969d-35b4cbaadece#card | 1 | 13 | NominalCoordination | ee58122193302e40345baae7e5d5a17a1a41f867e4c432391e4ef71c08fd8ef0 |
| Feisty Spikeling | fcf796fa-43ae-43d1-a124-776338bc3542#card | 2 | 20 | PossessiveNounPhrase | 8fb4797e9271bb22a2cfad35be58009f0a79c765f5f5d284c24306e060b4041c |
| Feldon's Cane | 9b884dfd-59f4-45c0-bf1e-6ad9f5b58895#card | 12 | 27 | PossessiveNounPhrase | 178539d922398779f0ba28f8a18e728751cd475134ebdcb3cf13aa508fb95176 |
| Fell the Profane | 053a69d8-2b5e-4f14-8b02-ca405891dc4a#face:0 | 1 | 13 | NominalCoordination | 9381f96ed17c2802eb05677c0ed02d066b0ff020d2e6011d9b0da634adc35dab |
| Fervent Paincaster | 1d569df1-23cf-4e01-8ef9-a1a8b815f11e#card | 4 | 54 | NominalCoordination | 1161b9e0667a19cfeacb96223e60896a2a0dc38a6f9e0955c99257cf1b9fb03c |
| Festering Evil | 0581b047-ffa1-42cc-a175-b7bf928e279e#card | 12 | 65 | PossessiveNounPhrase | 07fcfb9a368e71144a9dd831d8e2e2c4a38139067b924ba28c59755f6e5e6d9e |
| Fiendslayer Paladin | 3eb02993-b2fc-4185-aa67-08c8f133ce42#card | 6 | 39 | AdjectivePhraseCoordination, PossessiveNounPhrase | 0e235ea01357811bdf5a62e84d5a47eefb4685a0e408cfa7ed21af6eb0ce4824 |
| Fiery Islet | 026f4a4b-eedd-44e1-9d37-ca4fb8d6db98#card | 1 | 50 | ManaPhraseCoordination | 5b2c178ef60e9c9b9e3cd33749f5ea1cbc8d50861cb30f4f7c5235318ffba838 |
| Filigree Fracture | e2376660-20f2-4474-b6fd-d270e0e6a31e#card | 1 | 33 | AdjectivePhraseCoordination, NominalCoordination | 4d126da8ba3a51597e0ec5b1df1487fab1c1dff1b12d8cc8b072397ebea8d928 |
| Finishing Blow | 3c8a6c58-f4af-4809-af86-a931313fa71e#card | 1 | 13 | NominalCoordination | 9381f96ed17c2802eb05677c0ed02d066b0ff020d2e6011d9b0da634adc35dab |
| Fire Ants | c23543b7-f637-4a7c-9b72-ec411cd5488e#card | 7 | 28 | KeywordComplementPreposition | 1576bae73babbc856439059d5078cf9b842ec13dd87f656a291142e9e0ec05f0 |
| Fire of Orthanc | 407021e2-4072-4961-95be-f9309c341677#card | 1 | 31 | KeywordComplementPreposition, NominalCoordination | 50020db0ac7d18a1dd85e70cd6c2cce11deddefb90026802b6f089c58dec14cc |
| Fireblade Artist | 52630eaf-995e-4d08-be99-888a38b598b2#card | 48 | 56 | NominalCoordination, PossessiveNounPhrase | 0776802169eb70974cb00fb2ebe9b96833f688c1521b408fcd9ea1bf27cf8f46 |
| Fissure | c8b1e9f3-b014-4e57-b278-6d84a7e88b23#card | 1 | 27 | NominalCoordination | d6eb6c81d23217b2bd5d424bd1503b2b708670b731dad6dc81046a7527288f72 |
| Flame Blitz | 817eb91a-e3b4-40c3-9a6e-e4d0387e0f4f#card | 6 | 34 | PossessiveNounPhrase | 06b9b2fc5aecb221c95354b010d24493812ad990d1ae117f98ba94093882dd56 |
| Flamewave Invoker | ee17706a-edc0-4301-8330-1da539679c7a#card | 2 | 26 | NominalCoordination | 6aa8ae9e9a1fda188870a6397b15ab93aa12ba4bad3de1c7d1c901765916f6e3 |
| Flashfreeze | eaf98e03-729b-4145-b2af-c910c415c15d#card | 1 | 15 | AdjectivePhraseCoordination | 5a8cb5ed2fa36176d4441d6b015f3299f405de2070f258466cf642138c3d6529 |
| Fledgling Djinn | 2c73ef77-ae58-401f-8747-538c4cd075d0#card | 12 | 31 | PossessiveNounPhrase | 35a9147277b68e804ce83b1e5b02eed2da48ec07052a255405a877ef5f8f91a2 |
| Fleshbag Marauder | 4b1bf05e-753e-4350-a913-894cf3cecc0c#card | 1 | 26 | PossessiveNounPhrase | cc803752ed14d81f99e1a23e922edc69a3ea7f7c366ff23e0745ad54a3b6cb1e |
| Flood | e8eb2abb-daf9-43e0-b909-d96b679f71c2#card | 3 | 19 | KeywordComplementPreposition | 466e61cfbb0a26cfe127dfde1151b2eb602a5e9ba1bc054d49f91b949a05b37e |
| Fodder Tosser | feb34b5a-d51e-42d7-90c6-c5c84ff00c4e#card | 2 | 32 | NominalCoordination | 9a329aeb6604b5e3e5ee432f7290406229b6fc0e67be3dfe1dfa6a41b1aaad7e |
| Font of Ire | 88d4c611-3211-41b2-8453-6210f8394a0c#card | 2 | 32 | NominalCoordination | 5a38dab21ae2a6a62d3b00611db556a4a55d5625caf3e18378ef0b35554efe1d |
| For the Common Good | de6e4aa2-23f1-46e5-a061-05522944e0c6#card | 960 | 74 | Adverb, InitialAdverb, PossessiveNounPhrase | 00288976bc8a46b4af3cf60af4998c5e8abcb380e56788ed724959d791e37821 |
| Force of Nature | e3c4c27d-f263-4c69-a4fe-2928136ff68b#card | 48 | 42 | PossessiveNounPhrase | 006adeaaa034fb9aa639da9b54118d19880dd2f97f6bb246063e2e09b7ed541b |
| Forsake the Worldly | f60fb81a-969a-476c-a227-5231bbed4ad4#card | 1 | 18 | NominalCoordination | 4fe5ffdbcd24de0edcb7452aa0e21b435c5b46d590e9bceef16174fdecfffd4b |
| Foundation Breaker | 8b7a3613-f1bd-4262-9b11-b631167c2c2d#card | 2 | 31 | NominalCoordination | 14b53ee5e73879bb97464d4703e65ab6971a6a0136e6b0a4930c96e4e47ed71f |
| Fountain of Renewal | df024c55-c008-48ec-a1a5-02ce336e3de6#card | 4 | 45 | PossessiveNounPhrase | 247df3ea6843a77773ee8775e14072b0e323c50d81c9c1a26cbcafb61f5c743e |
| Four Knocks | 64fe83b1-5aae-4433-82c3-a11c8c83137e#card | 8 | 27 | PossessiveNounPhrase | 22543cd32a5efff1690a41df9410bf0291cf19de7421d04093f1871374d939dd |
| Fracture | f21d0319-0509-4ac1-b6e3-10955a26fd7a#card | 1 | 15 | NominalSeriesEnd, SerialNominal | 49ebbd36c6e60f5950b997bccb406d24b515f97bd36b51e72bec92fd62eff9fc |
| Fresh-Faced Recruit | 14319020-faf2-49c4-a808-c39650203383#card | 2 | 17 | PossessiveNounPhrase | 672b05a1ed111dcb80fc5ef288923ccbb459c39ea019b642ee551ec5bc9ccd16 |
| Frost Walker | 54a22814-c0d5-4637-9aa9-a6bc96401914#card | 2 | 28 | NominalCoordination | 654baf11b835cf0f0f883b9a3cac772160c30191bcb951279db0ab75077fb3ca |
| Galvanic Relay | 973c684c-a396-4e40-8aaa-365bc64a6f9a#card | 16 | 42 | PossessiveNounPhrase | 06d36d3957eba12fb51d68bb287fbb465f70bcf7fc063d7959e31cbdcb9ca36d |
| Gastal Blockbuster | 3b601721-90ca-45d3-9a3a-19c719799bfa#card | 8 | 48 | NominalCoordination | 09ce03cd9f481437fc1ae49bf1dbaffc2ff7036fa53361037541ee4fe5d0fcc9 |
| Gatekeeper of Malakir | 6781f8ae-2a86-4e3d-bc43-48809c9d6c26#card | 1 | 39 | PossessiveNounPhrase | f6a5064e7567ae5c5c46bf8582b1c560507483c277c3bfa45500a31565e3c2e2 |
| Gaze in Wonder | 70a702fe-1dd6-4d7b-a85a-45744ecad741#face:1 | 1 | 15 | CardinalCoordination | e9bd0e0666db4fc6f1991132492722ecf82ebc9dc081190ca1aa193b25162cc3 |
| Geist of the Archives | 345964ea-12b6-44be-b1a6-50178768b8f4#card | 2 | 23 | PossessiveNounPhrase | 56bc5b0003b40347075fe871f402da404c0932f77a8c8d21d7f4cf90325e9ccc |
| Gerrard's Wisdom | 3e30e8f2-f437-4620-a3bc-dd9c29ae6570#card | 20 | 23 | PossessiveNounPhrase | 08cd86f06f1944552f1a7da5978439ebd8cbd2d8e6742b03c3086dae79e3d323 |
| Geth's Verdict | 0f575f18-4606-4b85-9a90-07f8d8d46d06#card | 1 | 24 | PossessiveNounPhrase | eb12d8dd2d90daaca70b01efbcf898ed97d8bb8c4b34ae02ada8abb9f4b2245e |
| Ghostly Flame | 6a461ca3-7923-4df4-9f15-4072545da117#card | 6 | 28 | AdjectivePhraseCoordination, NominalCoordination | 18f4863c428d26da282c59cee4594084819e910578c3e86124bd5b3fe9366084 |
| Giant Ankheg | ca905672-50c2-4e19-9d08-2d3f8e3debfd#card | 4 | 31 | KeywordPhraseCoordination | 169c51241a80d549a3c57c7064409ef43e1f74ac997bb1d2e3906d2adb75d15d |
| Giant Turtle | 9297c0a6-1a8e-4e6e-99d6-f0877b2ec46c#card | 18 | 27 | PossessiveNounPhrase | 04162a84e05132874533d605aae01614eaed8fd2058a9ab1bd067fe55102b998 |
| Glaciers | a4a9c2bb-e6d0-4665-8b2f-e90c78784303#card | 16 | 45 | PossessiveNounPhrase | 00ab60e016e67f6e8d516a90c8ddb5e08d3aaf7d8396fd2e787e3a3d9c9ae101 |
| Gleeful Sabotage | 25fe48be-95c4-4011-8c60-4628fb5ecfcb#card | 1 | 16 | NominalCoordination | b2750cc5dd7f8090b7c6b67740c7ea21d983af8b74a94d69d40104be9ffce74c |
| Glissa's Scorn | f30f17f0-bd2d-4f10-9739-111917acc100#card | 1 | 23 | PossessiveNounPhrase | 92bac5d60c32e9cb0945f62ec63e7e028d715140757ecfb510df3af9072d33bd |
| Glistening Goremonger | f810e185-4498-483b-ab37-0c583dae0eaf#face:1 | 2 | 28 | NominalCoordination, PossessiveNounPhrase | 355c56eacbffe258ce513d02345e2ddf69a61b98d8d1d32149a13c264581a7c2 |
| Gloomlance | 2aa040d2-35a1-4b1d-9542-395e1b8a9fd8#card | 1 | 34 | AdjectivePhraseCoordination, PossessiveNounPhrase | 69251cbed1a94b0e6011223df659ba6fcb11985c64a0c6947fb85ddcb6d3b79d |
| Glóin the Mighty | 6505a53b-00db-44c4-8eaa-09a1a01451a4#face:0 | 8 | 23 | PossessiveNounPhrase | 170fdc16aa501a21fa9e13be703a5207eade4d185b0a964137e08283c1db5f29 |
| Gnat Alley Creeper | 17f930a9-64ee-46f9-8a7c-8877e532c94f#card | 14 | 25 | KeywordComplementPreposition | 0515d9102b3bf9024e1c710c968bab239ecde0baff3b68cf1195372bf532146b |
| Goblin Chainwhirler | 0342b085-be20-475a-bc38-59b9cb6012e8#card | 6 | 38 | NominalCoordination | 09505028cd13e905f1e0d48f0bee4ac6a9e697671a3937c9f44bb1aa376c21ec |
| Goblin Fireslinger | 396564be-3747-49dd-98dc-afb81a375f18#card | 2 | 25 | NominalCoordination | 868538f6bee688310f10c8f801ede5e081181f0b87e32d3e2500d52f8fe211ac |
| Goblin Locksmith | d692adcf-8ab5-4208-85d1-c75285a85c59#card | 1 | 27 | KeywordComplementPreposition | 25308b2c4fb1faac12aabaa9e9398ed9833054079896dbe44ae6fda30af77570 |
| Goblin Researcher | c8762dfa-a027-4e1c-a2fa-bfdab1de102a#card | 16 | 54 | PossessiveNounPhrase | 0a0b42433aee21230fc525f4c3e0366a94521fc2262e9b20b017b74d93cff3fe |
| Goblin Sharpshooter | d81285b7-a718-411a-8be3-ecc0cfe0bcb0#card | 24 | 60 | PossessiveNounPhrase | 041322a7e5a3076b7921f5a347335aca91127f9be3b14a9f93f0a803eeb62e53 |
| God-Pharaoh's Faithful | 3f86e6c6-fca5-43e9-88cc-4546cdd5f0ec#card | 8 | 28 | AdjectivePhraseSeriesEnd, SerialAdjectivePhrase | 6c2a19c2f49669d73028b26f95293b1379b2ff86f4fd4f53b098e1048dd37e6b |
| Golgari Cluestone | e917ad3d-df8e-442b-918c-3aa7a10f3cc4#card | 1 | 44 | ManaPhraseCoordination | d53e80035615b69093eb2b0a15c437611c074549cf695db94a6116e1a088a8fa |
| Goretusk Firebeast | 853ce64f-50b4-49a6-9aa3-f07b11cd8b69#card | 3 | 27 | NominalCoordination | 15096ba40b86db4cd15e3d2e41d5153332cbc41c97d22f879e3558897854926b |
| Gossamer Phantasm | 5ed23ed2-be2f-4958-affe-429f58e9dca5#card | 2 | 31 | NominalCoordination | 01523c6dc0cb1ac2c0a41b2838b7444d236da1c95b369377d697640ae80490ce |
| Grabby Giant | e6f59807-eaf6-4889-8c89-2ac915afefff#face:0 | 1 | 28 | NominalCoordination | 7af8c8f468809b20705efd92392e8bb9ac505261d1d349e87b5b0d5d4227cddd |
| Grafted Skullcap | 7694887a-2f3b-4ef1-b598-f3323f018ef4#card | 8 | 44 | PossessiveNounPhrase | 230add7a2f72ec1421c063fff039bbd39b40b1c09fa024d2fea4a4f2a559d589 |
| Grand Abolisher | c749f23c-40c0-4159-b84c-a70cbb062c14#card | 64 | 36 | NominalSeriesEnd, PossessiveNounPhrase, SerialNominal | 234063eeeef752addfe3ed562b8bcebffa2ff4030909076c81e21439528fe5a1 |
| Grapeshot Catapult | 23f73983-0337-4464-8817-5f7596d65b38#card | 5 | 26 | KeywordComplementPreposition | 3599d0b6cdd97afdc0796d4b5b04bd558a2594cf8a8c9fe33d9bbb12fe18ca07 |
| Grapple with Death | dced5cab-1438-4264-b44c-54dc37ec73d3#card | 2 | 24 | NominalCoordination | 87702c8b0e08a9142c0d86103f4ed6ee50a571bd1cd4989ae102a02804c87e4e |
| Grave Consequences | 435c8d98-9fa3-40d8-9b7c-32474099e08e#card | 45 | 62 | Adverb, InitialAdverb, PossessiveNounPhrase | 0483c58b7060f44d24f8bd63f16c148b646aba0403cf924f9d48af7b04111b80 |
| Grave Pact | 6f4ac4a4-53ec-4bc9-8f5c-d4b801d867b2#card | 2 | 33 | PossessiveNounPhrase | 58d865433f3399b597f2638ebdd228f447c5429eda91be048a3fd7a210514e0a |
| Gravestorm | c815d510-73c9-4a8b-9947-f478a2f5297d#card | 40 | 55 | PossessiveNounPhrase | 12c25fb058e2cdb3f5097f8f3168b82c32dc05a3f41e341da2c804800bfc8398 |
| Gravitational Shift | 0411aead-9f77-435d-a343-635f23221cf7#card | 1 | 38 | KeywordComplementPreposition | f12f44d873e14b4907b37065910bfa3da50a808db0ac6d0382def45876fb97d5 |
| Grim Lavamancer | 37445e06-88a1-4e2e-a432-383736c9b977#card | 8 | 40 | PossessiveNounPhrase | 3da6430022f6a146726617e7087e9a24cf492133d09d03abf5b34535c83ba004 |
| Grim Monolith | 229d6627-1292-4ae1-8849-b0f956fa6540#card | 12 | 50 | PossessiveNounPhrase | 442286e76a8ccf449de4880584406e2c47dcf7d62c3028ea59abc916cee25c6d |
| Grim Strider | b9dcdf5b-0fdb-4d8e-8ddf-08e44f67018d#card | 10 | 26 | PossessiveNounPhrase | 05f4d892e3ff188e9beb4c100c6561c32457e54fa3ba763bda75c6acf1b6bbc0 |
| Grinning Demon | 3db0207d-d80b-4915-b157-6a3e5b3a154e#card | 4 | 30 | PossessiveNounPhrase | 1284b93d1ce8de569e5333c98c827961f120495e2448ce44f7ea0d6283c2825d |
| Grip of Amnesia | 0144d9bb-d3cb-4c23-b133-6653d7c3a76d#card | 12 | 37 | PossessiveNounPhrase | 075c96ead210b9fef74b03f107bcb6aa8444eb485412c64a121c48281f8af7bf |
| Grixis Sojourners | 21f507fd-6a26-455c-879a-f838e0aa2a43#card | 20 | 43 | PrepositionPhraseCoordination | 0292b9a411c92ead36bad0c71f932f858e57c83bbe304a6667dde9ea9bef07a4 |
| Ground Rift | c1f7d451-ca70-403d-81bf-8822876d1a54#card | 1 | 23 | KeywordComplementPreposition | 69a5f077bc90031dea25505fce6ce5550ff42430b503511322bce8d249123c69 |
| Grove of the Burnwillows | d33c3fbb-8306-4c2d-b0dd-88f12639da94#card | 1 | 43 | ManaPhraseCoordination | ea453ff4f57bb3f00a588d576e40ffbca3b343682b73000356fe8f685c43e9b6 |
| Growing Ranks | bec6fb31-60a3-432f-ba36-aebf172f8b27#card | 2 | 18 | PossessiveNounPhrase | 63d9c3bc87ff39140c1a097e8d42a966912e498def23b3f85dcfde27ca050be4 |
| Gruul Cluestone | 80911151-1e72-4d10-b00a-9fcff2dd131a#card | 1 | 44 | ManaPhraseCoordination | cb6469289d44ab3f920b5a9925313418a474e05f5436b467ff9f22db5ee32bae |
| Gruul Spellbreaker | 827b3ed0-09b3-4d89-8b81-dd89cc002782#card | 4 | 25 | PossessiveNounPhrase | 2341dd6cb6db052360b6ca24794bcd2ace4b4407a7fa4ff3d01fd8688b40b54a |
| Guardian Naga | 67dec976-bcf5-4995-8da1-cd570862d3cf#face:0 | 14 | 33 | PossessiveNounPhrase | 08717daa61c6e6ceadf5f9742da10ae37d2e29f1f6494d0fac96fd06979daafc |
| Gundabad Opportunist | 61773534-8da8-48fa-b51a-73c0a9a751c4#card | 16 | 51 | PossessiveNounPhrase | 0186adca84221bdad7f5b53cdb97accb2abac600e2d34cc6192af82552fb5f55 |
| Gutsplitter Gang | 322c1c8f-ef8a-4a9f-b9ce-927720bad1d6#card | 128 | 45 | PossessiveNounPhrase | 0053ad1450a94a6ba696a67c4d92b56eb2a54221541cd4df6ec56918b5fe2e0c |
| Gutwrencher Oni | 73af8406-4447-41b2-9cec-1b471759f09c#card | 16 | 36 | PossessiveNounPhrase | 0051b96089210c63b4c060c8ccb17cea0acbbfe4286dc21e008cc8a1d86fd37d |
| HYDRA Assault Robot | a2558476-5506-4a09-93fc-b8ab33c98985#card | 12 | 33 | NominalCoordination | 057c3cbf1a55d2636b0ad3602867e1d46ed04b6bcbcb7477e585c8d9d21b1f4b |
| HYDRA Infiltration | 27692079-a517-4d38-b452-7409980b5912#card | 8 | 59 | Adverb, FiniteAdverb | 10ea8276c9a393a709f79573a1c2b815884ae7ddbe67d49b5a3a934f0d868a33 |
| Hammerfist Giant | ce916c92-350c-4546-882b-555dc450413d#card | 2 | 29 | KeywordComplementPreposition | 02e9e4d339b97b5b91d435b37df1da7f6dc2dd336364748e2796fec8e9608757 |
| Hammerheim Deadeye | a6962ccc-9883-48ca-9a5f-aeb68e66fd97#card | 4 | 27 | KeywordComplementPreposition | 14cd0f17ff4cf0a984cc3b57eb65ae329fca585ec945ce296f9ae7259c8265fc |
| Harbinger of the Hunt | 3256029f-6558-4fcc-9fa1-74ceba3e5c92#card | 35 | 59 | KeywordComplementPreposition | 124447a0e6e3820f79ac5cdaf3d86d95be4cc9bdf1eafc5313ca8bd97ec9919a |
| Hardy Veteran | fc112282-bc40-4b91-ac5f-ae49db1dabb8#card | 2 | 21 | PossessiveNounPhrase | 2b972701ff431f4b691aa23069aaea601257b72fc4bee97862f15dfff1e12f5d |
| Harrier Griffin | 65dc3191-0eb2-4c10-bf2c-ee0e24888870#card | 2 | 24 | PossessiveNounPhrase | 43e585842bc7febc5b96edb7503b1ef9f8f3faaceee2770baaa0bd236dcd62f9 |
| Haunter of Nightveil | 496c852b-ec62-4320-afbb-e19f534954c9#card | 2 | 22 | PossessiveNounPhrase | 2d0270248a3d2e64cb1c5dcea235fff9b628116cd71bacc070d24ad96284b905 |
| Heartwood Giant | b7d7418d-222b-4ae0-b0d9-9ea71d6ad800#card | 2 | 32 | NominalCoordination | 2c5676f51e3bad8f97b4ff636b2bbe811b50463161768a959a586f42cb625344 |
| Heightened Awareness | a69a0720-35fb-450d-8d48-8e51a14ffa49#card | 4 | 41 | PossessiveNounPhrase | 1bdcdc509ee4c3d2d6e2668d196075eb399d4e164cfe2f44e06facba86552941 |
| Herald of Eternal Dawn | 4080f7e3-06d3-4d3d-9929-4e826cb66713#card | 4 | 35 | PossessiveNounPhrase | 1a9f341b7227f1035513a3a7dfbd38808ed762b95e8a967ec8fb202dff5a711b |
| Hero's Downfall | 03df6a57-37c9-46d3-83b3-4a6240100714#card | 1 | 13 | NominalCoordination | 9381f96ed17c2802eb05677c0ed02d066b0ff020d2e6011d9b0da634adc35dab |
| Hissing Miasma | e257d8e0-06e9-433d-a750-1962db399388#card | 2 | 23 | PossessiveNounPhrase | 041b11e7bf0364580bcd07c791dfe63be0345d8fab62661c862c207279c90f47 |
| Honden of Cleansing Fire | 15ee7e1d-b13e-4fd0-bc41-345b6ed34f53#card | 24 | 33 | PossessiveNounPhrase | 0a4e400f6740d11536131d938b90e05d83f0f7fc4241a04e6c1aa0f056018025 |
| Honden of Night's Reach | 3776d11b-18fb-4451-a8f8-bda4c7c23776#card | 20 | 34 | PossessiveNounPhrase | 10f62dfd57326ea97fcf6da92adb09df56b52b7dd5bd934de15ffdf09b9d0526 |
| Honden of Seeing Winds | 45beca46-480c-4771-ba46-78b6ac7d5c2c#card | 20 | 31 | PossessiveNounPhrase | 072c7f430955004fe9d26b661b1cb192bc78a48935abe735d768453af3ce5d5b |
| Honorable Scout | 16345f75-f33e-4a31-adb1-d3563358fcec#card | 12 | 35 | AdjectivePhraseCoordination | 015d361e38c406320d18562bfb659b68521d9989dad13809e16e0cf409c58b4e |
| Hooded Blightfang | 342eb666-838c-43d0-bf52-2f704f2a80e3#card | 32 | 75 | KeywordComplementPreposition | 01b4cf379efd42187cc1a6c22dc80d3687bfdb519b2a66e0302f37a2f3285dd8 |
| Hookblade Veteran | bad2385b-6981-46fa-a012-d3992c3bc118#card | 2 | 17 | PossessiveNounPhrase | 00f3ee8b23727d692d147c96dd2cb94c0ec86611071745579e90a259405a6d89 |
| Horizon Canopy | 262a5d83-506c-4781-9bc9-1a2b5d83955c#card | 1 | 50 | ManaPhraseCoordination | 06e6bfa6f53031e5bdc8e44c23f108a5b2e319e1d70ac5ffc606161ef861021e |
| Horobi, Death's Wail | 44e8d51e-0885-49df-a77c-cf4a88e41bc1#card | 2 | 32 | NominalCoordination | 48ba86a20605f7f0536c876a0d1b203221a9e32af9816b9ae58060ce5ec74118 |
| Howlpack Wolf | a2fb7d03-d6bf-4716-9ca4-bd7dbcf9ae66#card | 6 | 25 | NominalCoordination | 17a54c3a5f865aadb4e46dda32532c6edbe702837162f44a103e6b8041dac5ba |
| Hulking Raptor | 9f2e7533-cfc1-4dd8-a9b8-09cdc712ef2f#card | 8 | 28 | PossessiveNounPhrase | 05c5e0bca4005e3f7861dd2d3f5178d92f37d0d8704aa8e5f03991ef1f6b47a9 |
| Hungry Mist | dedfbd92-4f12-423c-9d56-167753f03bff#card | 16 | 30 | PossessiveNounPhrase | 092e1c75809a1b23fd0327feaba8903b84faa7b60a3b31be221d4abe71e5000f |
| Hurloon Shaman | 9fc74fc9-bb02-4d9e-bb8e-f44f219d8946#card | 1 | 26 | PossessiveNounPhrase | 76fd86152e16bc471b6990dd1613ddd6e77249e9bec8f4eb8366060a70dace60 |
| Hyperion Blacksmith | bafad165-c2aa-45d1-8f99-05e752450dfd#card | 2 | 29 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 1f8deffdca5eeb1342f6cf47161f73c244a11163f9439f75e2469a644398412f |
| Iceberg Titan | 3feedc63-173e-4713-8e96-8f1c9576055f#face:1 | 2 | 28 | NominalCoordination, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 0df0f740d3d346d36f1ba0e59b8dd664b8539b15c92c26198d2a4f916573ccfe |
| Icebind Pillar | 18050051-63f6-4091-bf67-7a0701a0791d#card | 1 | 21 | NominalCoordination | ca719af863d9b9cd603b30ec9d44be198b5c5fbe202e4721b927b6d681ffa26f |
| Icefall | 30baca42-50e2-45f8-8f99-8e2f9795b4da#card | 1 | 19 | NominalCoordination | 19fa0af9bb7f65254cfad58dc220f7f7c610b8038771ad620306e2bb2c9e4258 |
| Icy Manipulator | 3608f1f7-8dc5-4dd1-ae91-c830e1de9529#card | 1 | 23 | NominalSeriesEnd, SerialNominal | d4fd1a2eb47f4d9c74d72c82e1a4e74448c9badf3db16e529eb28843ae1cfd92 |
| Ifh-Bíff Efreet | e503a4f2-a785-4e7a-89a7-a9b24fb98831#card | 2 | 47 | KeywordComplementPreposition | 23718929699f3a5e529343f9182d087f4f0ecc415c30e8a100f48a20a9d500e6 |
| Ignoble Hierarch | c8de43a3-ebd3-4000-b343-a6ffed11d34d#card | 1 | 23 | ManaPhraseSeriesEnd, SerialManaPhrase | e2f2e14cc48bd2085123b60e475888e989427eb4494f39320a006f1aa1479def |
| Ill-Gotten Inheritance | 84e8ca5e-730e-468b-943c-7c6358ade95c#card | 80 | 77 | PossessiveNounPhrase | 0050057317b99fa1ca3c53e6ad44238149c2560a89011003c2e9e9f3f2285f8d |
| Illuminated Folio | 0440f85f-8128-4399-9a3d-51f39e196c96#card | 2 | 38 | PossessiveNounPhrase | 2986c2dc3a3bc1d5e3071fb4cc237ebc60e9fc7c668de562cfa2665ac435528e |
| Illusionary Servant | 28e99c5f-a78a-4323-b5e9-1b43303acaf6#card | 2 | 31 | NominalCoordination | 01523c6dc0cb1ac2c0a41b2838b7444d236da1c95b369377d697640ae80490ce |
| Imperial Subduer | bedf1dd5-b133-455b-bc3d-c7b1f469d91f#card | 8 | 33 | Adverb, FiniteAdverb, NominalCoordination | 28de1248f5983abd70639f75f7a5b2ff637d0b58540e759ea5c4bd4f3dfcd0db |
| Indatha Crystal | 3617c4ec-dc70-4fa8-94f3-fbc48abcaeea#card | 1 | 25 | ManaPhraseSeriesEnd, SerialManaPhrase | 37b2b6b966e994460e551f7a27e66a904b18ec0434e9e97e07826b57a3b82e32 |
| Indrik Stomphowler | de47a1e8-9c69-4af6-9d72-1bdd41352b32#card | 1 | 20 | NominalCoordination | 2d0ffbc9ea448c5215db5c89209a07f8be11aef1209ca35cfc5218443848ae1e |
| Indulgent Tormentor | 8b202c63-c961-4590-958f-d17e76610ab5#card | 8 | 44 | PossessiveNounPhrase | 0be3a0713791b3351995a066e71dd8fa123f58f2b07d14fe8d7bc48f63eb90be |
| Infernal Sovereign | b08cb220-34fe-4c54-b654-64a3c24dba44#card | 64 | 52 | PossessiveNounPhrase | 022c32c6baafc2f16e53ae667c3b2c8c40224ff399a8381414e4e8ccbb764361 |
| Inferno Hellion | 68c88a4c-8c2b-4e6f-b835-7e269eb073c8#card | 8 | 43 | PossessiveNounPhrase | 511c312203f602e26237857a1d3347ebb78fa86a7329f665fdfb7f85cb69a8ce |
| Inner Fire | 27348fe5-21d4-40f7-9feb-bba2c2321008#card | 10 | 20 | PossessiveNounPhrase | 41a5963fa02fda23342340b10d0aa175107ef1293090b291f479fad3d1130183 |
| Innocent Blood | 6791ec3c-c397-4087-8c8c-84d3797df415#card | 1 | 19 | PossessiveNounPhrase | 33ddb84fe4415099da4c40a24a99d49295532ed62d61b4a87de74d69f9490e15 |
| Innocent Traveler | 7950a7ff-7c2e-40fe-b64a-324f4fbcf528#face:0 | 4 | 50 | PossessiveNounPhrase | 0c1a3d0a1a82a0149462bb35ac8c590bd6c0710500daa3c3198b1438435fb788 |
| Inspired Insurgent | 5863c67b-a47c-4466-8bd1-210ab26e0d3f#card | 1 | 24 | NominalCoordination | b3c24c793478a6cf883287fe4f6f921881ae37b9d181aebe3bfcb3cbe55499bf |
| Inspiring Paladin | 95efbb81-2418-42ed-ae34-5759efff8253#card | 24 | 54 | PossessiveNounPhrase | 1185bc98681c007506ca9b70cd2938a5cfef4559237afb0f850123109e0b0b9c |
| Invasion of Azgol | 1abcdddd-0022-4dfe-8c15-eb1fa86de614#face:0 | 2 | 33 | NominalCoordination, PossessiveNounPhrase | 4062879e90df876f2e304c2227977c34e0b86aa9ca466005ca472a1be733d923 |
| Invasion of New Capenna | 530dc119-e266-4525-abe9-7fab599e9de2#face:0 | 16 | 50 | NominalCoordination | 056152b1746f52d5e822079e56891b2c9154ab9ce902a9368c8cf82c9340f193 |
| Invoke the Divine | 87bd0fdb-c5b9-46ea-9858-1a870960351f#card | 2 | 24 | NominalCoordination | 2094532db5c9e799cdb6d1996a4cc636bc60f6ea8f96e949160dfc634db28d55 |
| Invoke the Winds | 721c3255-953c-49cd-a5dd-852c3d09eae4#card | 1 | 26 | NominalCoordination | 1808c31e7f84e161fa9624bf2d5e5febb17b1ffd2ae098bad9a99e5f0308a7e3 |
| Iona's Judgment | abbfbe50-4caf-48d0-b8f1-f2c9212bcd2f#card | 1 | 13 | NominalCoordination | a85941a512e6cc6f2daeac0598c6f65d356b428f6fa099c42358187a66940a8f |
| Ironwright's Cleansing | 599faf5b-88fd-4373-b6ce-6047a67b30ea#card | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| Isolation Zone | 7c1890cf-36db-4720-a472-9158bdc0d8aa#card | 9 | 36 | NominalCoordination | 207c6427a8664f75eb5c4ddcc47993895e6c9d05b2be5d61e7ce91c71c098749 |
| Izzet Cluestone | 3c76bb90-92fb-428b-8e94-def4cf4b6f2d#card | 1 | 44 | ManaPhraseCoordination | e53e7ce9d95501e212912734d9aa9ca980f3bc952733142f39f816dc45a403c8 |
| Jackal Familiar | e4d5d666-fd55-4c1a-89c8-2348288f6b30#card | 4 | 18 | Adverb, SecondaryAdverb | 7e3bab0b1502dac18f9404ee792ed91d467b7f13797407bc0f6d5d2d95e44397 |
| Jeskai Banner | fda10ef2-f431-4d9b-96a4-4db4c98d2b0c#card | 1 | 48 | ManaPhraseSeriesEnd, SerialManaPhrase | a9e4c6e6ca452346d5c2ae7780ff3e36a1852b797730cf76192c028be8072d0d |
| Jinxed Idol | 89b1eab9-4d24-4367-99f7-263349f69c3a#card | 12 | 54 | PossessiveNounPhrase | 21af70db4a7aa60ea7b753988035fc0b00eeeab6835e0f8a8626b10c8b88168a |
| Junún Efreet | afda663e-c5f7-4182-86f7-d95d71793717#card | 16 | 33 | PossessiveNounPhrase | 1ac8b2f5362b9b0ecd8d24b3ffaae8e801e6cc5f3f76928864ab780077a1da3b |
| Juzám Djinn | 4e81596c-9225-43d1-bd35-798212144f2c#card | 12 | 28 | PossessiveNounPhrase | 1fdd447da30af62a9874061c77343e295234288437b49ef2f569c598f5293bef |
| Kami of the Tended Garden | e2770ebf-24e3-4787-956b-ca01aaa50a6f#card | 16 | 33 | PossessiveNounPhrase | 048fa1e4d2ab7883463cfd6a88471093bfa714945a976af1edf1557fe254dede |
| Karplusan Forest | bd912666-f37f-4767-af6f-9e6d0fcccacf#card | 4 | 47 | ManaPhraseCoordination | 3fb680cb9a0aa1568667ca5571c7e7750162a665c55e9c30a2b783bd1614da03 |
| Karplusan Strider | f2074c37-6e76-4460-9666-010c7f38ae66#card | 1 | 27 | AdjectivePhraseCoordination | 1e9fdb4ba6759142b209bf038c032a8d80fea3d263195de5c18a0edcc054e200 |
| Kavu Titan | df166d0e-f944-404d-83b9-8cf53e220774#card | 18 | 44 | KeywordComplementPreposition, PrepositionPhraseCoordination | 15ba349db4f7f601d7e7472b6d85c4328f8ae00771e697feacf20c502caec8a2 |
| Kefnet's Last Word | 67cf6ba1-55ad-4008-ba1f-96c363a96b16#card | 24 | 44 | NominalSeriesEnd, PossessiveNounPhrase, SerialNominal | 3077f704f2fe0595f1363e8a01b9b1a574aa86330ea3be0281aa41ce343a9204 |
| Keldon Champion | 1600b361-9298-49f0-8925-4c1aad363e1b#card | 3 | 37 | NominalCoordination | 035bb7206b4d0fb456475ed700b009ec7b477168d1b440e39aa773de145b2870 |
| Keldon Marauders | e05b850f-24cd-42b9-bc4f-0f05f9aea679#card | 3 | 36 | NominalCoordination | 3922f40d43b47bc69c9d2c50bffc2539b0b07deda24098267108541be6aef555 |
| Ketria Crystal | 3ee5d7b6-0263-4239-a5fb-de1e1a220c55#card | 1 | 25 | ManaPhraseSeriesEnd, SerialManaPhrase | f463019b26ecde8cd513a00926b2d62cc09434076113b4b64a5fbfd7b732e47a |
| Kezzerdrix | 1523671c-0cf4-442c-a5f8-c74ae31016e6#card | 32 | 41 | PossessiveNounPhrase | 0231527997b3504524017b747a8e4f9c345e7ec90e14d0e0d618af29d61af330 |
| Killing Wave | 69d6b906-5461-4eba-8667-dbb8c0ce3fcb#card | 3 | 27 | PossessiveNounPhrase | 2d25de2e22a98e8b33c21afdef09e1cbab490d14444b2c17bb2d24684ca11931 |
| King Suleiman | 97f548b1-6398-4dc7-b5eb-da5a3f24ddb7#card | 1 | 17 | NominalCoordination | a2d9cfca2e06ebe2511281a738b97f3e54cb3d1ae5877f4611957bf15a241393 |
| Kingpin's Enforcers | 342df67f-44d8-4b0a-b1d7-4241966d0598#card | 1 | 28 | NominalCoordination | e35143ad614e6925792ee17f18ec464aa61b55d5b2f12563a832ab9211839657 |
| Kithkin Zealot | 7445122d-a943-493c-b7b6-3699cc07db0e#card | 12 | 35 | AdjectivePhraseCoordination | 09bd1869cc72b296ebf1db8674df8c1c843df6db4d9d9cb286208136ca43055c |
| Kor Sanctifiers | 55d8ac5b-622c-42a9-83dc-f2c1c6c2506f#card | 1 | 33 | NominalCoordination | 6c25d3899d0e49d6be51ec5ac2e5988f41fe2110c9e1bef17d22a435b02d7225 |
| Koskun Keep | 184c5a0d-7654-4421-86e4-7f04bcf49494#card | 1 | 53 | ManaPhraseCoordination | 77a64d1e0999948311869bf5acedce1498c81931411854c46d97af3ef466a978 |
| Krark-Clan Shaman | 198d1792-d638-4760-a476-7ec3af495610#card | 5 | 29 | KeywordComplementPreposition | 735ebd0f03aee712142b5b649cc6d8bbe20d09db536ed86d773effce79d90aa4 |
| Krosan Cloudscraper | a0ff742b-f709-43ac-84d2-dc00c723bad6#card | 16 | 37 | PossessiveNounPhrase | 078ade55b4d521d9c83acba1a86f46d54ca021cabb7c21bf401baab37ff711d1 |
| Krosan Grip | 3e39224c-72ce-4ecc-aa17-12c071ea1f3e#card | 1 | 16 | NominalCoordination | 6e8498467e09709308311cffbec3254cd31e2349d82c22320fe56cea1ab3db8c |
| Kulrath Knight | f3d1b2e9-6ede-416d-937b-2063ea278d2f#card | 12 | 37 | PossessiveNounPhrase | 040ca70de733eef37ed46df225b6de3dfdde23f995a2312435d5d1891f0534f6 |
| Kwende, Pride of Femeref | 3adebbbf-39fe-4b42-b719-2f8edb009693#card | 2 | 23 | KeywordComplementPreposition | 467d90a933f8a9e960c51a5281039e08b690e7bc25ec5442018e88111471e375 |
| Lantern-Lit Graveyard | 73a39a1b-2fb7-4328-8718-18569ae28e9e#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 02c37b8955e04c0dbefb1671aa34aa7c3a4e91fbd9a2020d8918af054c4347b4 |
| Lava Flow | 91c0a76e-3992-437f-b85a-97b0b4adbb84#card | 1 | 13 | NominalCoordination | 41f720c7a984a12e81be5dab1993e45d7cd9c083f858e73ef12a07bf797ca767 |
| Learn from the Past | f325fd7a-dcae-4892-adf5-de9893a23ac5#card | 3 | 30 | PossessiveNounPhrase | 1e7818088624bb48d7875130f440f951e021c1b433539587c23fb0bde71c046f |
| Leech Fanatic | 17bcd92b-1826-4a36-9048-49e5e65a12fd#card | 2 | 17 | PossessiveNounPhrase | 60341d0912fb107449137db9502c629b4f394c223585a347e1352fad985e6ee3 |
| Lethal Vapors | 9fe9650d-6c9b-4988-9ea4-79ba0d55a77b#card | 4 | 60 | PossessiveNounPhrase | 12ca9bc2a56bd0e001ae702cc1e1be41f73c6e5179db64db4bcde8d4cd8c901f |
| Leveler | ba1d34ab-7529-4f47-a5e7-7d8f66d7b901#card | 8 | 23 | PossessiveNounPhrase | 1a0d3037f67e8ec725a166861ab84834ca34e92df171b4d9a6988ae2fbca034a |
| Lifelace | eec1de80-4b3d-481d-a235-c299e0381830#card | 1 | 15 | NominalCoordination | 1e46608235e8ee47d5403c46b3c58d3ce30efaaa3bf55ea98af4918df630d3f2 |
| Lithophage | b4eb3d7e-a234-4ed5-8611-fcb818686fcf#card | 16 | 30 | PossessiveNounPhrase | 02e528b2193f7989e94be7c20df3a04b78ef56336f0fe6296099148f6d1b0b0f |
| Living Library | 81f022a7-fd19-4340-b8a0-6b11ab3e9d91#card | 4 | 46 | NominalCoordination, PossessiveNounPhrase | 23dc1497017dc5932183fdb6d67d5de299e184e63596e13cd2a0736d809959cf |
| Llanowar Wastes | 32116127-cf96-4a1b-8896-a1ebc087b597#card | 4 | 47 | ManaPhraseCoordination | 11e216a5f93519692db96481baf38525d00ff423620a8d996312e07bb93fb3be |
| Loaming Shaman | cf7352bf-3971-4bb7-88b9-65ab9a219289#card | 47 | 36 | PossessiveNounPhrase | 1bc8d77faa8cd4b7c7bb813c9851c9b1e754ae709612639c9dd3217310c4753b |
| Loyal Pegasus | 3b7b862a-7edd-4e96-b4b9-f7113b4c93f9#card | 4 | 21 | Adverb, SecondaryAdverb | 500b91e3933cc7840cf28992ae222b58968d461330b33020cd3da144c8c45b83 |
| Lullmage's Familiar | 2a56fe0b-f8a9-499c-bce1-cec6cb3b66f0#card | 8 | 41 | ManaPhraseCoordination | 380b0efeebfdca8851efa71064242fec8eb6aeb594fccde92ff59821a7534582 |
| Lupari Shield | 6c370299-9803-4cb5-97ba-c17831797b54#face:1 | 8 | 24 | PossessiveNounPhrase | 2cad66c8d078760220381c2d245eca2c82117db74826454bb9d3a7f5e0b09c00 |
| Lurking Roper | 61f3e964-6bea-4543-992e-3e36404bf628#card | 24 | 39 | PossessiveNounPhrase | 315d0726072d870a2397ce70e70764c1bcb558634fb720c3b5a7f1a072d3f051 |
| Luxknight Breacher | e0a508b5-0896-4620-a93e-60b958829703#card | 228 | 40 | NominalCoordination | 010ce0a7599a659cca1b7c4906bdbcd0b4d787d735a48c8e57e85b424f9e4d72 |
| Mage Hunters' Onslaught | 22195e0e-a426-48d6-b100-480c7e2bd5cc#card | 1 | 37 | NominalCoordination, PossessiveNounPhrase | 775a287733f827e071d7a8946bb5cac7599a70d8a56e94ddaed02b2c0264fdf7 |
| Magma Vein | 9ae2bf71-c443-4814-b7c3-b3b82d595983#card | 5 | 33 | KeywordComplementPreposition | 0e06f699c574634338ecde0599a99c5a52266a6e4b0366d3431b62ce4f1a4256 |
| Magmatic Chasm | 6cfd2cd8-a86e-48fe-a85a-9a3444b5030c#card | 1 | 20 | KeywordComplementPreposition | 9794bdd1b02ddc40b70c691ec972bd65b26cb16e43744fb297ee0cb5561ee1c2 |
| Magus of the Moat | a035f865-b4db-4b9f-bccc-8572acd2033c#card | 1 | 17 | KeywordComplementPreposition | d80b0d4a9dc21c6b52d9d4e577af6d8b2e3f83061b0a59f1bc5e2b0c170ff581 |
| Makeshift Munitions | 2421bec6-7647-4684-be61-d1aa951c6b4f#card | 2 | 32 | NominalCoordination | 6a03d2e9cc1456ab172c879be20fe3fa22f37b8e16d287ce3d00741ec9ed46b4 |
| Malicious Advice | 14c1adf8-950b-4f64-9c84-956a623448f4#card | 4 | 28 | NominalSeriesEnd, SerialNominal | 8b58ace5fdab8d901c6c0e6c8b9c02045a3e78741eecfd9318e16894454c78c1 |
| Mana Vapors | 8e082c95-cdc4-4f29-9915-05f83298f14d#card | 6 | 27 | PossessiveNounPhrase | 2db7e36d68f6bdd549fe41c28d811f9613bc6aa4b84b348f85e7834136391b81 |
| Maraleaf Pixie | 013e4281-4e3e-47c2-ac9a-f570668bcb16#card | 1 | 20 | ManaPhraseCoordination | 7e6881516a5c4a94baabec73ffe3018ebc842ba05c22a5adc94e2272d35d942b |
| Marauding Looter | 427bf4eb-3dfd-4747-be92-4a4a3d96a228#card | 32 | 53 | PossessiveNounPhrase | 00bf7a0f2dd3be6c3a5c8c37bcf8aeb51c7ec14cbb608bb054e317fa81a876f2 |
| Mardu Banner | 9805b7b3-086a-4c91-810f-52c8f8f95f6a#card | 1 | 48 | ManaPhraseSeriesEnd, SerialManaPhrase | fa1cce0ecdacb812f7b682937b2d424f36266383ffa0b960162f61cde2ccceb2 |
| Marshdrinker Giant | be109ed5-9e3e-40b3-9bf0-7caf7cdbb958#card | 2 | 26 | NominalCoordination | 3556252021cd9c56a0453961389d37971f2c40c14018a206006bdc14814742d2 |
| Martial Law | dfcf9f38-eb26-4033-b7c0-45ed29417aa5#card | 2 | 27 | PossessiveNounPhrase | 47a4302fc97ae6fead53747f7fe19607ece526b3c5252bb83b90e3a9f2498191 |
| Martyr of Ashes | 7f00bc45-7c65-4455-9db0-58bd79bcdb4b#card | 30 | 48 | KeywordComplementPreposition, PossessiveNounPhrase | 09bd6e42fe7e3f38512bc6ff69ee8df33a3e4d3cd24c80c5e93b67f39b6bb619 |
| Marwyn, the Clearcutter | aff4e43d-cafa-47de-8e39-011f9254a26a#card | 1 | 28 | NominalCoordination | e61fbe4cb4f18dbcdcb826df26a982c9889fc32230da3b2fe36f79be61fb3ded |
| Mass Manipulation | 68e87559-7cae-4dcf-b7cd-a61f19bcd840#card | 3 | 20 | NominalCoordination | f64f16caf844bcd0fe272c29f6046081fdcb1e853f293c95b5237b0658476573 |
| Massacre Girl, Known Killer | ccb39f4a-2684-4eb7-9066-aad81ae08962#card | 2 | 55 | PossessiveNounPhrase | 13053a207d6150db49d63c328d2511e5ea88d68fb6b7ad66da6a63923db08ef9 |
| Master of the Feast | 01dc86f3-5ebb-4b10-bf68-8fc9f232c724#card | 2 | 27 | PossessiveNounPhrase | 598908488b49105516333fb4baaeb468ec21d987a1d410ca0bdf455e52260330 |
| Masticore | 621476a4-44bf-4f55-adb1-880cdd213f38#card | 32 | 68 | PossessiveNounPhrase | 00d6ed13924fe917f1a1017835564bfd6a60cad415d28384a17eb55dd008789d |
| Meditate | f79f84c9-2023-4348-863b-a3b4bc382904#card | 4 | 25 | PossessiveNounPhrase | 0893b2c0c6f21879c6db46a7a0f98032ee446391d154a606d25744d361f53b28 |
| Memnarch | 52d8e70b-7e94-4e17-811f-70afe39ff9bd#card | 9 | 54 | PossessiveNounPhrase | 22e7c5a23d68e14e908ad4ac3e69ce6a428c1d5c9c681b253543b114a07a04c3 |
| Memorial Team Leader | c71d3775-0c71-4e37-979d-35e319f357f0#card | 8 | 34 | PossessiveNounPhrase | 1dfe2ff5eb8c9e14265ba8ed86dfbdeab7d3919f087cf4c20b677786ea586cbf |
| Merciless Executioner | c3c45d50-9038-41df-bb2f-9bc40071845b#card | 1 | 26 | PossessiveNounPhrase | cc803752ed14d81f99e1a23e922edc69a3ea7f7c366ff23e0745ad54a3b6cb1e |
| Merrow Bonegnawer | dad36dc0-39aa-46f9-819b-cad7a578c037#card | 24 | 50 | PossessiveNounPhrase | 0081d9bc5273da22eab523338a39aaf131bb716a56d4f5beaaf2f76c10e3b96d |
| Merrow Commerce | a931b609-1486-447b-9479-61a45d64f2ef#card | 4 | 26 | PossessiveNounPhrase | 39adc54ef2fd593816b3b8d607358b98d67d3ce27a7ed170d8386f363054e486 |
| Michiko Konda, Truth Seeker | fbe190eb-fd4b-40e7-9a17-48c96b60f653#card | 4 | 39 | PossessiveNounPhrase | 3aa7b37f1e10b1eea98ec52c53fc269a7a4d66c94dde0c6e30c9f6c9f9331f31 |
| Militia Rallier | d5377597-3454-4636-8563-7e6a5544e36a#card | 2 | 34 | Adverb, FiniteAdverb | 4597476c322dd11ed3a779a6fbe2b5f303803a95d027a12af383cc059bd19aeb |
| Mind Games | ea43372f-b0a7-4d1b-be3e-a4a738b06d59#card | 1 | 21 | NominalSeriesEnd, SerialNominal | c01394e9b4b2c7700f30710e294aaa7de8ffa821a48fb012903a7ca01072ff96 |
| Mind Over Matter | 656afde2-cdf3-4907-a0fc-7ac93f5d3e03#card | 2 | 30 | NominalSeriesEnd, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement, SerialNominal | 4a1d0c573fd86b0242e6d9fdc8bee33df1d2ff468b8a059126ccb9ee47673ac0 |
| Mindslicer | 76c229d2-4bdd-4520-97dd-afb40f8c65ec#card | 1 | 21 | PossessiveNounPhrase | 4fa48ceb08a16760bbf3da97ae0c32e175c30b74ea3560c6a0f8160f59354b87 |
| Mire in Misery | da1d1f64-e239-4b92-9eb9-fe9ea06aa18a#card | 2 | 21 | NominalCoordination, PossessiveNounPhrase | 0ff4c17c9001ed267e5219ee6528da714186b148016b08cd94b8a62074131056 |
| Mirkwood Bats | 0636b6c3-0662-420a-b30d-f0a14e7c512d#card | 2 | 29 | FiniteSelectedHeadCoordination, FiniteSelectedObjectHead, FiniteSharedObjectComplement | 22c86847431e865aa5e1d6006603dece190d4b4195b6438ff85ee393e08b65f9 |
| Misfortune's Gain | 01b778d7-0965-48ab-a81f-e29f8a233d07#card | 1 | 23 | PossessiveNounPhrase | 1c14c5bd8d1cc981584379669f7a9d0e85b85836a7f8e80603e478937aa09afc |
| Misguided Rage | f5c77355-96dc-46af-a7de-f547a1368ff1#card | 1 | 19 | PossessiveNounPhrase | e75dac07837d0843a1fb4748b250abf4599198b049c3d4a9551bab41b6061b10 |
| Mizzium Meddler | 48a909b6-e6ee-4148-8b50-b35f11bc065f#card | 26 | 38 | NominalCoordination | 144c83fe819f1a363aa189417a28c7b6c201dce1221a3bca75750f907fb33740 |
| Mnemonic Nexus | 6fe302f2-5594-478c-a66b-89b4b96e3611#card | 3 | 19 | PossessiveNounPhrase | 73da5fd7a8e575f35e600eb935408a41160bb441bca67aa27e87d00cc590a28a |
| Moat | 42208fea-8c24-451f-861d-6d70c0a7a502#card | 1 | 17 | KeywordComplementPreposition | d80b0d4a9dc21c6b52d9d4e577af6d8b2e3f83061b0a59f1bc5e2b0c170ff581 |
| Mobile Garrison | 14ef5aa4-5f61-4fbd-9a8d-35920c40ec25#card | 6 | 30 | NominalCoordination | 023b4ce19db36e466b70efcda1c15fb0553550efb43e9e512f7ed90c90d973ab |
| Mockery of Nature | 5da5240e-5d87-494f-aec9-64b7a3f0d935#card | 8 | 33 | NominalCoordination | 0c125810078d524ac20e41c4b04988c851e72446a1316912f8d3b69831baf75f |
| Mogg Bombers | 69812cd2-9885-45ed-9d79-658597bfeb8b#card | 7 | 34 | NominalCoordination | 02b0e3ec00b3a1277a89cbce223031725b6601cdb83a8496133f1a83fc90a99d |
| Mogg Flunkies | b92fc9c1-7264-4894-90b8-a193039489c5#card | 4 | 18 | Adverb, SecondaryAdverb | 7e3bab0b1502dac18f9404ee792ed91d467b7f13797407bc0f6d5d2d95e44397 |
| Mogg Hollows | 1745fd57-467c-45f9-a46e-b9a2af87ec87#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 18e5047a1b80d1530ef656a5a925cc2e887e3aca5147379a259e59832040fc96 |
| Monk Gyatso | ff92fa60-f0fe-496e-8155-d9d6f5af651b#card | 8 | 39 | NominalCoordination | 28f61d93fc6020a15ce0379684a7bf829a0ada3714997977c98152add201761d |
| Monomania | 29c6935e-ef64-4298-af74-9b18eed93136#card | 2 | 24 | PossessiveNounPhrase | 56e04f0467744659e93bf328f74b9ed9dd82365e916d56b6d5aaefe1dc91c277 |
| Moonlace | 52d4dc8a-2799-4b4e-9c20-f14bb75c2279#card | 1 | 15 | NominalCoordination | eec2b3ef68a62dc54f7dcebc93064976be089cb86c04c9a00dbc7b9dda5af723 |
| Moonstone, Harsh Mistress | aeeb50f2-8e02-4d40-a1dd-28b1c5c85c7e#card | 640 | 67 | PossessiveNounPhrase | 0154beff277c44974192346ffb3c36f0403d9410b0fca51bd995494e81973651 |
| Morality Shift | 82e36ba9-9652-4594-8660-86c766b47709#card | 4 | 24 | Adverb, InitialAdverb, NominalCoordination, PossessiveNounPhrase | 3532ff4c76b67ed23dd47c004f05d06ac2417e9ded4ee4790a53d2330442d3fc |
| Morkrut Necropod | ed3990cd-fa67-427b-bbe4-a2b1942e9be2#card | 1 | 25 | NominalCoordination | be279eefbc48447f4ebf21cbf3c1e8378f95d2fe80923649ceecbcf79a270cce |
| Moroii | 418be12f-b40a-4018-86c1-98c9000a999a#card | 4 | 26 | PossessiveNounPhrase | 38dfabefd17f2676f5e04779ccf038e6c92afe30cb809df75d3892bee574b446 |
| Mortify | faa01ed1-ccfa-4e58-951f-cd81f9068027#card | 1 | 13 | NominalCoordination | a9348ccfd4aa003a1a6b7deed01216de0229106622bafe4999e531effd164794 |
| Mouth of the Storm | 832fbb1f-0ff5-4d6c-b1ee-2231923a6f64#card | 12 | 44 | PossessiveNounPhrase | 000fb236854c5f2a3f5c7995121f32804f3aa589f5b9453a7a2b00b8f6c04a74 |
| Myr Custodian | 43923f24-7e95-4934-9a5c-8d9a5e06ddea#card | 1 | 33 | Adverb, InitialAdverb | 218c942b93e14ee2d9de158888b6fc4e9d5ef0dc77b2d505eee8057ea9fcde9b |
| Myr Mindservant | c0ea83ab-f62e-43ee-b7ad-ddbbc8916632#card | 2 | 19 | PossessiveNounPhrase | 95f5ac0e3b6723737def17f695a9460865518b725ca0a468981e3ea7aa518c73 |
| Mystic Redaction | c2e54e13-dad5-4d01-850f-a1735eea54ae#card | 4 | 44 | PossessiveNounPhrase | 55bfb0531aacf9248dc158b3968892103e0a80899a7c435dbd5000dfec0aab4f |
| Natural End | 05b5c0fb-84b2-4a32-bf66-179c4dccb3df#card | 2 | 24 | NominalCoordination | 359b5dfd49d36bfe73715a480ae60fb4f13f14db83c194fe6abccf351a510d7a |
| Natural Reclamation | 98266582-12fd-4a7b-a9bb-e9cc4370ed28#card | 1 | 16 | NominalCoordination | 6cf59c0fd947be456689ec134e9441a62db6e43cf3d438d739ee685022d5c6af |
| Naturalize | bdb3ca68-ec1f-4e16-81cc-d23f8f52c728#card | 1 | 13 | NominalCoordination | bc97d4e47a24480cb6c1f41fdf441d4cc53efea11b4f2764b359754e0e2e37df |
| Nature's Chant | 46e4ace2-e8bf-4b72-81de-286215886a10#card | 1 | 13 | NominalCoordination | bc97d4e47a24480cb6c1f41fdf441d4cc53efea11b4f2764b359754e0e2e37df |
| Nature's Claim | 6d4e558e-9109-4918-a082-fdcbaffd516b#card | 1 | 25 | NominalCoordination, PossessiveNounPhrase | 8360ff218ad00cd5b8035c47ac2d866147e73ffd36eba109284221c59e02c553 |
| Navigator's Ruin | cd830c46-3444-488d-ae5b-0a2533158a13#card | 4 | 35 | PossessiveNounPhrase | 563f3b5d76c56f330c65f908b174ca5864d47e8cb28f83f206064b929046363c |
| Nettle Sentinel | 5c367b7d-3cb4-45db-b836-dad953efce15#card | 96 | 46 | PossessiveNounPhrase | 029bf65dd37e7a20d99ef3d12e15ea62128260766c22426c8f2c71c2750aa667 |
| Nettletooth Djinn | f1d300b6-f9cf-40a9-8520-d78cd7d813cf#card | 12 | 28 | PossessiveNounPhrase | 1fdd447da30af62a9874061c77343e295234288437b49ef2f569c598f5293bef |
| Never | bf276236-18a6-4a48-b47d-5227832d26e6#face:0 | 1 | 13 | NominalCoordination | 9381f96ed17c2802eb05677c0ed02d066b0ff020d2e6011d9b0da634adc35dab |
| Niblis of the Breath | 10391bcb-0932-49a0-8d47-e06e3c10dd68#card | 2 | 30 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | ab3315948bc7c282ff517053068a1f82bd880ccd24b9562ed44eba343e111af0 |
| Nightmare Void | 9a53ef0d-8e29-4128-9cc4-fbdd3e4fdb84#card | 6 | 45 | PossessiveNounPhrase | 52b5287dda3171ad4a7be98525719816c84733a6b99cbccf8a61ce14a508d25a |
| Nim Abomination | b0713b1e-544d-4549-9371-61ac7f6c2a6a#card | 4 | 33 | PossessiveNounPhrase | 3c7ffbb6c4fab19993a97dd9295cc6c0555024f863d42296f22c8bab4c066a0b |
| Noble Hierarch | 98aa9424-5912-4bd6-9300-b3972a31d8af#card | 1 | 23 | ManaPhraseSeriesEnd, SerialManaPhrase | a9f2ea4bdeb3364ce0aba6d2b2706855f825ea8bc06ff40e620fa3a1d1fb28b4 |
| Null Group Biological Assets | 49598fd5-2aad-4974-b499-38b60fc371b6#card | 16 | 57 | PossessiveNounPhrase | 1337917616dc4a7e62aae40b1d5e74166790b6871f43b438aed734d567f1bc33 |
| Nurturing Peatland | 8ed932ff-986c-4592-ad70-53b3fac80d69#card | 1 | 50 | ManaPhraseCoordination | 11e38f62621984a188cc6147b9190d4559693560a34f04a6f857f09c7f67b2ad |
| Nyx-Fleece Ram | 5a20113c-7ea8-4edf-af9f-148ccd326a88#card | 4 | 23 | PossessiveNounPhrase | 10100a152f05cd08961af8655fa8e0766a02cc571d75a5d80d4040b2708b5841 |
| Obelisk of Bant | 3f96dfd3-975a-4595-95c1-d2f04a86cb37#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | 897a46abb2fb256497aa0796ca14551c31774453366dd6786e2cbc09e1dc5a3f |
| Obelisk of Esper | 9da963f6-734b-4d3c-a3be-dd5cca2c7b19#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | fafa4aef1fbda1c6ed35b70cc205e953aec4b2a38e41bf495c21e8fb68d2cce2 |
| Obelisk of Grixis | e193ae18-91a0-4870-b8a1-f8dedf744708#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | ae7b688373e44124d66ed9ee961763d0f7ce235aab3ce9f1449348d6ef69f293 |
| Obelisk of Jund | eb0be178-55f9-4743-8128-34bc86f134ec#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | 8bed60ab59c497efaa7e7dda5ba6a5a8fee3d63ad1acd7fc05cb06c8a92c757f |
| Obelisk of Naya | 02e81bb8-ba80-4483-9b9f-973dcae0fc9d#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | cb7397c9691adbe280718ba4eb685de7e7f757391eb09747295dd33ef7fc3e9e |
| Ogre Gatecrasher | 0b88bc94-99c4-4be5-b474-a7dad68057b3#card | 4 | 21 | KeywordComplementPreposition | 142501875a3aea2dfdca227b8222e7b46cbcd65b8a04313eaac7579bce7830b3 |
| Olog-hai Crusher | fb6f04bc-3514-49b2-9d17-f80980d996a8#card | 6 | 28 | NominalCoordination | 337894834fe2fc107e9cc9d9e9387efb66abfbebfb76eeb8070d9036622a93bd |
| Onakke Javelineer | e834b281-279f-4df3-9c0e-764502b809c5#card | 2 | 28 | NominalCoordination | 30923f46dfbca7871bd2d394d05f9a8f656b74613fe38c3ec559620d7139a502 |
| One with Nothing | ba995901-bb32-45ed-8af8-f98784b5a9b0#card | 2 | 11 | PossessiveNounPhrase | bef81baa45c260bf6e21627157fbfb510b129c68c02081e9b400ff260caca41a |
| One-Eyed Scarecrow | 87b61c1f-18e3-446a-a732-cff40adc5e05#card | 2 | 28 | KeywordComplementPreposition, PossessiveNounPhrase | 937b10c647d7e23e78254e5bf9003c8d8526ff627aa9e3034023ff2c5b8fc53b |
| Oran-Rief Recluse | 043fa53c-8a46-4fbd-836f-2403c62e0121#card | 5 | 38 | KeywordComplementPreposition | 14df24463506ab3ad062fc54d3dc70049f9a7510e1e929ec693cdb2fdc8c1774 |
| Orcish Hellraiser | dd21cbaa-7537-414b-b720-3c7d6fc7c93d#card | 3 | 32 | NominalCoordination | 73d92d71c7a14f76f3d63f2571ba43d31feb952513b652b42137ea52953d5057 |
| Organ Grinder | a9f279a5-c35f-4703-a583-050f2dffb28e#card | 4 | 31 | PossessiveNounPhrase | 084cf6b85a1bae50c0a9b97c2b381d241fea3e0463cec55189d67ff9f1419b78 |
| Orzhov Cluestone | 09650a76-05c4-40a0-b861-d146b3ba87d6#card | 1 | 44 | ManaPhraseCoordination | b776a05cfaa41130c923fad18541d42614ce62cf084ccc9dd6436522a093f1c1 |
| Outland Liberator | 9545f126-062f-4410-a362-e16255a128d6#face:0 | 1 | 27 | NominalCoordination | e29c095ac53ad98cafeb752d23628a86c23af2db288ab5740e0fa7792dc68ff1 |
| Overgrown Battlement | 585f62dc-4461-42f1-a3a4-b19a1e550d2d#card | 10 | 30 | KeywordComplementPreposition | 10599e6ad06f0b5f921e63ea870303d698fd9d46a6531937047494a4eaeb5369 |
| Overwhelmed Apprentice | e68d6459-2f32-4320-a062-6d33f85348f9#card | 2 | 34 | Adverb, InitialAdverb | 62e60e6f3d7a9c4933e6d1970faf68de134b85b7221270d97c3c290f1964e810 |
| Pacification Array | b2c14195-2747-490e-99ef-5b71048ff51a#card | 1 | 21 | NominalCoordination | bc8af072bdfa132ba19b1a9078a855bb3dcf369ddd65efd65734a4c0f09f0126 |
| Painwracker Oni | 4c033502-2005-4e13-96b6-3f8c0a4b6a81#card | 16 | 36 | PossessiveNounPhrase | 0c0b0aad5018446862765afee73aea29fa5734e1bae5f3cc54105bc7be874295 |
| Paradigm Shift | 7179c2c5-c59c-4793-a3b5-28d26d755157#card | 96 | 32 | Adverb, InitialAdverb, PossessiveNounPhrase | 01890b9f881d8327fc3a88fdc98a413b7932365855b492d8cf9144532c6315ad |
| Patched Plaything | 0b7e950a-89e1-4ec4-94af-5af2f1252b12#card | 368 | 43 | PossessiveNounPhrase | 00559aeec1b5bb58779d6f1a013c3cab1dfb16e477895a808dba9a7b62b23e55 |
| Path of Peace | b7593cf8-4dcb-473b-a2ef-180fffe66738#card | 1 | 23 | PossessiveNounPhrase | 1c14c5bd8d1cc981584379669f7a9d0e85b85836a7f8e80603e478937aa09afc |
| Perennial Behemoth | ee4b7aca-31dd-4369-8e85-b300892b9b41#card | 16 | 27 | PossessiveNounPhrase | 24ee4e6624025a08a25f6e2fa566bb82c0e164d32e3a33e5738414d9808449ae |
| Perimeter Captain | 05608055-d97a-4c8f-833d-47b3dd1ea255#card | 4 | 34 | KeywordComplementPreposition | 6d4989e4f8a614e7c1e2898ec8625122badc9ea6d5cc131e6c15d72e78eccf69 |
| Perish the Thought | 8955996a-76c9-496a-9238-5fd89bcdaae4#card | 18 | 46 | PossessiveNounPhrase | 05d78142599e2348ae616f03042fb96856b117f796b7a4a1f4d862d2603750ff |
| Perpetual Timepiece | 17b4778b-82b1-4845-ad08-00f3ff66877b#card | 136 | 53 | PossessiveNounPhrase | 03b867bdf07413bfcb6e486b9ab418cc06e0f76dc16eaada94e70ceb4a69119d |
| Perplex | 904f9e0c-cb7f-4611-8bf7-2e408e8ab237#card | 3 | 28 | PossessiveNounPhrase | 253380118d5d4f75f476947b5b61ad6890bdf7732fbb5cb40649e5ca1d548068 |
| Persecute | 676c4cfa-f929-43aa-95cc-df81a596c2a2#card | 1 | 33 | PossessiveNounPhrase | 4376cc503ac7598633c1bb481888c5e1a16e37324a35d105f0b5a1951e3544de |
| Personal Sanctuary | 20963cad-1c13-44a1-901a-bfc9930089e6#card | 28 | 29 | PossessiveNounPhrase | 0112e3e61382fdc6d345d34ed64890f6c82e900a6eb8cd11871a60b60a703718 |
| Pestermite | 83cf4235-eb4e-4c55-839a-f2740d41ed5e#card | 2 | 32 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 1d93e20d4e8d96427d8c471bd929c69a0c8b94ee0ab4736c1d4856942d99a440 |
| Phantasmal Abomination | f4925e05-1332-4466-b2ea-9f6f4ccf7db4#card | 2 | 31 | NominalCoordination | 6e48917166e363877542f5b784e55080a0535848f7bd8b5637b959c843bd5db1 |
| Phantasmal Bear | 780da643-8d14-4f1a-a298-5b7454e9c4df#card | 2 | 28 | NominalCoordination | 654baf11b835cf0f0f883b9a3cac772160c30191bcb951279db0ab75077fb3ca |
| Phantasmal Dragon | 7f32c581-4d4a-4873-8e4b-b95e13790481#card | 2 | 31 | NominalCoordination | 01523c6dc0cb1ac2c0a41b2838b7444d236da1c95b369377d697640ae80490ce |
| Phantasmal Dreadmaw | cbde4b9d-1549-4892-b8ca-38256a6ac1ba#card | 2 | 31 | NominalCoordination | 084378eff9364982c287fbe11c581a7051abf6e37e0250fb606808bf561a50b0 |
| Phantasmal Forces | 06a158c6-7e36-49f8-a8e0-a7b7df5fd7ed#card | 16 | 32 | PossessiveNounPhrase | 12241d89886760656937714a7bc1d18454f95dd36d28b5351d21f4d5fd24a2fe |
| Phantasmal Shieldback | 8e514bc9-7c8c-4257-9dbc-19a78f61df05#card | 2 | 46 | NominalCoordination | 38361871651571e48a13983cfcf70dd9a7c0d465e04246276eb747325fcebaad |
| Phantom Beast | 13ca7c14-5db2-4d17-912a-fdeabcdcd9f8#card | 2 | 28 | NominalCoordination | 654baf11b835cf0f0f883b9a3cac772160c30191bcb951279db0ab75077fb3ca |
| Pharika's Libation | 32c84176-cf11-4010-b1cc-ed570a8c3e54#card | 1 | 46 | PossessiveNounPhrase | 2825d57a4126966c6136d4533e141000e4ca31558306fc37dea78cde46ac5d4e |
| Phyrexian Arena | ee579a32-a048-4335-b966-231ba731cdea#card | 12 | 28 | PossessiveNounPhrase | 37c7f21f92f60f3c29b6732f2b6852298719bbb36bf67f251b2893b851fc389a |
| Phyrexian Dragon Engine | 022b42a6-ee0f-4080-b973-c1b68938c3be#card | 32 | 56 | PossessiveNounPhrase | 0fd79afdbd144bd04d651e3f2008959cb0624ae4e19a6d26565c3148096fe910 |
| Phyrexian Ironfoot | 83b4acd0-751d-4882-a32d-6c089bc652ee#card | 12 | 35 | PossessiveNounPhrase | 3f862530b80dcc33585f90320b21fa19fdf9f2ff0a383a2267910ea4e3294da4 |
| Pick Your Poison | 9af4a832-d634-47aa-91ed-79d44fe08864#card | 1 | 68 | KeywordComplementPreposition, PossessiveNounPhrase | fa279cad6eaffd032d482aaaac0e327a91ce751e9d8f38b5fa7f41beac064f86 |
| Pile On | 361b0d7f-1e43-45c5-92c2-92baacaf326f#card | 1 | 23 | NominalCoordination | 29d9737d65ee5a90b1eea56b33cd96f3f7d512fb974b38d119de73ecfce1b3ba |
| Pillage | 0b137853-7cb9-424b-8285-12938991eafb#card | 1 | 27 | NominalCoordination | abfdb90d8085ccd90ebfbf5ec4f2fb08cb68158b39b6a2bb4dcd1049b9c3aba6 |
| Pinecrest Ridge | d8ef7c7b-0201-4978-ac73-fd376a19830f#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 18e5047a1b80d1530ef656a5a925cc2e887e3aca5147379a259e59832040fc96 |
| Pinion Feast | 9812bdf9-98b6-4b4d-8381-a7bbb25069d8#card | 3 | 21 | KeywordComplementPreposition | 8411177cd6b9df9eae0c4d747e7e6a101c1669ae1afad2f8d47b979bbb20e86b |
| Pit Spawn | f536acf3-ff8e-48da-8509-00e7ce4a1678#card | 32 | 59 | PossessiveNounPhrase | 02d9cf7bb91c08c944a7acd5db1900182d7189a69f030ff12a6bcaca638a2b65 |
| Pitiless Horde | d5726b60-60f6-4481-bc76-759fbb55bb49#card | 4 | 30 | PossessiveNounPhrase | 43adc5cf86eae6be9548447e4d16ac589afae61f370b9c9be26e550c4df009e7 |
| Plague Spitter | 5feedfb0-30e6-400d-9e28-d541ea1aa14e#card | 18 | 60 | PossessiveNounPhrase | 0956fa68d623c4e2937d5c21b9ad63997d3844ed4abec9139e817238c7c677d4 |
| Platinum Angel | b148578c-c0bf-4785-b97c-4b6f83028008#card | 4 | 32 | PossessiveNounPhrase | 9f1f4fb3a51c62cf115324e261bf0041bc604b84ece12c799029975165256f95 |
| Plummet | 85bde6ac-3dd4-4946-8b57-24f57e3eae2b#card | 3 | 14 | KeywordComplementPreposition | 22b6ddd29f72e754cf855043218718920f5fd06d4fe2ae7bb85be5e8ded8fa07 |
| Poisonbelly Ogre | 38872730-0fd1-414e-8015-29d1bc649913#card | 1 | 21 | PossessiveNounPhrase | 70dbd38b8580e661ab85ecd531c3dfa29f4093e5353ebf2a848b8a7d144a892a |
| Pompous Gadabout | 0361b9b7-9097-4aae-aa06-c63a5fb47a76#card | 8 | 48 | PossessiveNounPhrase | 29c7c46f9dce614f7103961a557acc2f5732834a9f251da53f61e8ee45b04320 |
| Portcullis Vine | a217f386-11b5-48f0-b173-3b0700ba295f#card | 2 | 32 | KeywordComplementPreposition | 0f60ecfb928d6bed1e9bc121d4838c9ecddd2139f178852c8a34ae27e9652146 |
| Portrait of Michiko | 94794f7c-62e7-43c8-ba7f-ac5c0bc9a712#face:1 | 8 | 28 | NominalCoordination | 0c61ed85c516d4f6607d385af52f15c277e70faee258fe44d84a1d5dadc48b72 |
| Pouncing Kavu | 5f9a968d-cc23-46d3-b28f-3779dd89c531#card | 18 | 47 | KeywordComplementPreposition, PrepositionPhraseCoordination | 00e4771d0cdc4b37ec6d19e11aeeec1b95a0d2a7175503ccd9c9485365af9e2e |
| Pouncing Lynx | d8d92f97-d54c-4954-8324-b9c0a9b96ec2#card | 2 | 17 | PossessiveNounPhrase | 672b05a1ed111dcb80fc5ef288923ccbb459c39ea019b642ee551ec5bc9ccd16 |
| Pouncing Wurm | 3e299484-32e9-4646-9032-062d08a70bb9#card | 18 | 44 | KeywordComplementPreposition, PrepositionPhraseCoordination | 032c76a2645549477aed3a02ae1adb2253a270cf03d8f030ea6b86bf333bd640 |
| Precise Redaction | c251b676-0e98-4047-bed1-d72c83aa8da0#card | 1 | 15 | AdjectivePhraseCoordination | 1ce8b4894febc60ced9be99c7799f9c7654388ac8e891993e642f4ce59b8873f |
| Presence of the Wise | 6728064e-1d4e-4d4f-a0ab-127c23478f3b#card | 20 | 23 | PossessiveNounPhrase | 08cd86f06f1944552f1a7da5978439ebd8cbd2d8e6742b03c3086dae79e3d323 |
| Prickleboar | a023deaa-c393-44f9-9c9c-6bb9aae05c1a#card | 2 | 24 | PossessiveNounPhrase | a1e299aec80d203136c305ce89c47ff8ddbc1a29c5505c44f7613c4241f811a2 |
| Primal Plasma | 540c5225-845e-4498-baae-0fbb836c1f38#card | 44 | 56 | KeywordComplementPreposition, PossessiveNounPhrase | 01ba3f09afa5055d796e61cf6ce0dddf16289c825740f2443b444e0daf46b9af |
| Prismatic Lace | d4a3c9a5-3c91-4135-a19d-98eb5bcf5e00#card | 2 | 23 | PossessiveNounPhrase | 36c48c39e33f66a422ba2fe5d898a0f52a3ad4e8a78cb853951147a5b8fac2f9 |
| Prison Realm | 808a8491-2966-4388-8590-a46ba147f65a#card | 9 | 53 | NominalCoordination | 1bcb130495612f255b20e750dae41d0afc01961373e42a18d99d72a66495e601 |
| Prophesied End | 45f4d057-5134-40a2-9590-ddba73a65582#card | 1 | 32 | PossessiveNounPhrase | f08d79a38d955624216e8e036c48495ce5115bca4e78a9dfe997781acfa8f3ba |
| Prowling Pangolin | 92042cad-09f5-464f-85ad-baad27e6d1eb#card | 2 | 48 | PossessiveNounPhrase | 5ff3b772e88dafb04cfe09bb291629a08c00fa5e4494c441f5142e4c9567ba8c |
| Psychic Strike | 17694111-5623-48b3-8c54-c383418e8d2b#card | 1 | 24 | PossessiveNounPhrase | eb9299069229d98b554a19a80947668799b0413752894519226e16493da42bc2 |
| Psychic Whorl | 3df4eb80-e8dd-4a9e-bd8a-b4b23fb9de6c#card | 2 | 33 | Adverb, InitialAdverb | 105242c60a6d3b1c8443b8c826787b051b394345fcc4c8c2ffcd47aea1c87aaa |
| Pulling Teeth | 9c966ba3-3408-4bdd-a4cf-97c9183e1115#card | 4 | 46 | Adverb, InitialAdverb | 2232d6ed91d5c4d5c33c437c1cc55691e087c12b1c7257dd4cdc34240adea9bd |
| Punish Ignorance | 0fc56124-bdcd-4f4f-886f-0fafcf8003bc#card | 2 | 32 | PossessiveNounPhrase | 5c581348ac7aa838f64bc1da2f0414ae856a48ce941713ec167f8b359044bd79 |
| Puppet Strings | eb810d5c-903e-4ab4-874c-e49a686ca367#card | 2 | 27 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | b2f46bbd085bd4f8c5f6a57ee57645214a0b63be8a4983167b31774f1bcd8893 |
| Puppeteer | b6597ae6-ae32-4409-8ba9-1015b6ffe2d2#card | 2 | 27 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 44cb2d46f77e1ee277f1015cae1200a574fa6e78517de90a4b33b7589f40f7f0 |
| Purelace | 3773001a-8868-49ec-a406-298cf72359c2#card | 1 | 15 | NominalCoordination | 02201df3cdbc6db8095b0e230870f6439a46b18577745bb06aa58154ec811219 |
| Putrefaction | 8c4545f8-10b6-4a62-9bce-8c9ffa8b8471#card | 1 | 28 | AdjectivePhraseCoordination | d77255dd8cba93b30ec051faa946420349ceea23aed0f04186e82a3bf3d83207 |
| Putrefy | 9b271430-f53d-42d6-a547-2f286dd9bcb6#card | 1 | 27 | NominalCoordination | 6e0c341d225feb97acb1b489c891b87dbf3ae67eb27d93aef8d3d2e06f0cf9d7 |
| Pygmy Kavu | 17407b73-0dbe-4a71-84ec-3aaca9651f5b#card | 18 | 31 | PossessiveNounPhrase | 1fba241744c5b4ac6005ece60a0e17313cb5ae5f7aeb4869ba02e3571544dac9 |
| Qarsi High Priest | 3bcb5ad4-818e-4f10-aaff-188d713b160b#card | 4 | 34 | PossessiveNounPhrase | 757eb0863fb2204f870c807fb345b8881325163d00b13f687c134a37034e0f83 |
| Qasali Pridemage | 10821e37-8a83-4d8e-a12f-249a10933a22#card | 1 | 27 | NominalCoordination | 10ccd6059339a2d1d521189721bbc6bfd6ab496cd9b330d550967e87765bec3b |
| Qasali Slingers | f763d118-3183-4604-a50c-7c3933e2185e#card | 4 | 36 | NominalCoordination | 3298355d48c26141985d1b25eb0c0619c2da863d7d28b748a8c33256b60b7c0d |
| Radiant Purge | b61718ad-7102-4ed2-b884-733bc46df715#card | 2 | 17 | NominalCoordination | 33cf652e4c1e85c30c2955ed30fab50e01ea17d5ce11374412cbac0e29f4b28d |
| Rage Thrower | f9697a59-46e2-4612-b30b-d041a38fdd30#card | 3 | 28 | NominalCoordination | 56270c593414ef0a2b46dec2ac235d93624c83fc6baab76d4c0c9384b7341008 |
| Raging Kronch | c05c2eae-1a9c-4ce7-9c76-22e40781c1a9#card | 2 | 16 | Adverb, FiniteAdverb | 3f033595abe4a76d5449a0d319c2d28aea1d470fc8861bca32ad0f7e09826e25 |
| Raiders' Wake | d5fd3060-358c-4a5b-aee9-1afcf6c506ba#card | 4 | 58 | PossessiveNounPhrase | 0bed40149c18a6eeab31f06b2c07acab3800ab8a76e89673c3afd7666f26e79f |
| Rakdos Cluestone | b4f0bda0-1051-4ab8-b254-891f427b33ad#card | 1 | 44 | ManaPhraseCoordination | 3aecbf0cd5426b2b668463abeb02cb88c1d4ff0742789d8c0dc0b6a317aa46de |
| Raking Canopy | 75f381a6-49f9-4caf-a96a-91c75640b9b0#card | 6 | 30 | KeywordComplementPreposition | 20ab1bb7bfeb19d6e584c3dc0f697d75e9c970cdc08c2f056e17bc44cf3ea314 |
| Rambunctious Mutt | 48a53d6e-0320-4b9c-b692-6d0fc00e46cc#card | 2 | 26 | NominalCoordination | 50e476d7c4e6bc85bf28b240b6a6d3972ce5e6c6ea18eef09efc11fc2e306a17 |
| Ramunap Excavator | 4f819ba4-52ef-4fdd-8e4c-5ae3b2f44db5#card | 16 | 21 | PossessiveNounPhrase | 08317b437d0cd2064872bf5abc693c38eb130b8f8cfc44ad9377c27b5fd88284 |
| Ransom Note | 49c0aac5-d650-4ada-a194-3bf77f104bc1#card | 4 | 75 | PossessiveNounPhrase | 0430bd2918b22348c76f68b79162c507a296f90d9429aca8f6a01d95d99f2547 |
| Raphael's Technique | f58a5f8b-254c-4206-bdbe-55a23798ed2d#card | 2 | 29 | PossessiveNounPhrase | 5886d98078b665fe7a62cca7beb97e2e65b8bb5b0eaaaf93041ff09b3a50b617 |
| Ratcatcher Trainee | ecc91e38-90fa-4d89-b262-d5f36dce5be4#face:0 | 2 | 17 | PossessiveNounPhrase | 672b05a1ed111dcb80fc5ef288923ccbb459c39ea019b642ee551ec5bc9ccd16 |
| Rats of Rath | 75458175-eeb1-4a6b-93d9-c744c93b3759#card | 4 | 24 | NominalSeriesEnd, SerialNominal | 995dd05475611552a849c334e8d568a60675068adf42b7125f789e0af9e0a8a2 |
| Raugrin Crystal | ae4a8470-4517-42c7-a7f6-ef7635e10fe5#card | 1 | 25 | ManaPhraseSeriesEnd, SerialManaPhrase | 1fb9e8c2c8e2e4fda8f8a4034e2def0de6020946a9ecafd950e9f27a9c8d6b01 |
| Ravenous Giant | 46c7a552-4a98-4e1a-8651-8fb0e7153aec#card | 12 | 28 | PossessiveNounPhrase | 1fdd447da30af62a9874061c77343e295234288437b49ef2f569c598f5293bef |
| Ray of Distortion | 1c838441-4713-49dd-97e6-4fb7155c36c8#card | 1 | 20 | NominalCoordination | e306e38983b7eaf7530fb9c97603a58fdc6319399318afc5cc597246e25a081d |
| Razorkin Needlehead | a78f981a-bf8a-42a4-b171-d655cc2cc1a2#card | 12 | 45 | PossessiveNounPhrase | 17df009a74231476e1bdd3cf7ed801a8f7a8506f7049e1567a89e6aebc0abb50 |
| Razortip Whip | 44cf7fd5-721f-41f4-ae78-cf93afeff5f1#card | 2 | 29 | NominalCoordination | aeb69bc816efc1b86bde1c913cd80d33c16057b6d0b135ffc0ae1746acb1bfab |
| Reactor Raid | 4e34a49d-f031-48ac-a458-97b79124b76c#face:1 | 8 | 36 | NominalCoordination | 0a2f077ad21b7f7f8efa4eed8f6ea802fcb6eadaedf9452e55b1f7e10ca242a4 |
| Reality Shift | 70dc830e-d05b-4fc7-88dd-879e140b3fbf#card | 2 | 30 | PossessiveNounPhrase | 910914c58e65f3a8a4ee6f4b98a21eee6f0fe3fa22b79f7554d28756f9497709 |
| Reality Smasher | 17c651c7-da4e-45e9-9a64-fad3e873123d#card | 8 | 48 | PossessiveNounPhrase | 0e426d8cc19f70128fa6acaf78c78c9571e39db6c5f0fa144dad96c6f665916d |
| Reaver Drone | 6dd887be-484b-4ecc-8c02-8cce7e43ec89#card | 24 | 37 | PossessiveNounPhrase | 00cc9f7204db9047bd086fed22fb80d9940055065771e4951486b250059cbdee |
| Reclaiming Vines | 3ec3d99d-2c49-469a-81d5-f1609b6e49ca#card | 1 | 15 | NominalSeriesEnd, SerialNominal | 1f644aaa99011a3ac8b1618fb2c9958e269fca6bccf72b1ec3c936b19396fe86 |
| Reclamation Sage | 032ec6e2-6cc3-4a97-9cc7-3233f5e11904#card | 2 | 25 | NominalCoordination | 1182f563cb58f7369ac258af89bf2d710fa4421d228f040a07fb502ff91602ca |
| Rejoinder | d97de156-5a6d-4dc6-953e-d4bad63cbad2#face:1 | 2 | 30 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 52853571f36cfdfd36a501d6a0f3058524b6d73242b20ff97b95d7f32581b398 |
| Release | a7df5dc0-2564-4288-bc53-59affd896f79#face:1 | 1 | 34 | PossessiveNounPhrase | 32361117c022840c11336c33289f4c75a3758f7ac4b6a7ef5df026123cd75590 |
| Relic of Progenitus | 6a9c3401-570c-41cb-a605-75dfa57d6ab7#card | 3 | 54 | PossessiveNounPhrase | 14b4cac2134bdae2ecfb899ad7b9e7a23ad89dfd07298bed5ac69e829b286492 |
| Reliquary Monk | a149ac31-b792-4fbe-903a-788bbdfd5e97#card | 1 | 20 | NominalCoordination | e14d23bf98e400074f6e0e020bcc1c586c9478a9200f8715894273f42035f4b6 |
| Reminisce | 296c4d2b-e41c-416b-807e-5b4db638ec57#card | 3 | 19 | PossessiveNounPhrase | 2770e9797a23f9c6f8010bd57825679ad601c555d8f7e9e0a383e5f8b1ab7171 |
| Renounce the Guilds | c854f9e8-6a1c-4515-9912-3847978bf5d5#card | 2 | 21 | PossessiveNounPhrase | 3d3cc496c5f0546f6533f878ea84c86ad22a36e3a4fa03a587144cdbc86ef8c4 |
| Retreat to Coralhelm | c450f6b7-bb2a-4b8c-aec7-d27555137b63#card | 4 | 50 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 718387ae16a1958560501c0025bdde22a8366a6e7d97adb02651c8e72c554374 |
| Return to the Earth | 722f2965-a1ec-4b97-a4bd-b99364f58b1e#card | 4 | 18 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | 35cdbbf4063433aacffab009817dd78f4a995fc8d9971e0e0ca499bc7eae9495 |
| Revoke Existence | 289e1a7c-b301-4395-baf8-e9a1c6d38aab#card | 1 | 13 | NominalCoordination | 7f788fb32a0d13dd260f5803f8af6f1e9d25b81f5cb9a916729a6a684d9c9569 |
| Ring of Gix | e01fe9e9-a76c-4909-924c-a0cba403f975#card | 1 | 28 | NominalSeriesEnd, SerialNominal | 711f1e94ec4565574ed9e29c7f042544c58456a666844d7de1b201a970767a2c |
| Rite of Harmony | af94fd87-ea37-49eb-b5e6-0b1846f23859#card | 4 | 36 | NominalCoordination | 033139e1cf170274c1d78d044dff942369f4aa2f245e0f00d09acc0a730bae26 |
| Rockcaster Platoon | 3bc82914-1524-4d57-8b38-9788867a4046#card | 2 | 30 | KeywordComplementPreposition | b07074a099ab304017a16e48830ad61ca35a02619d76aff6e6e019b34ecbe858 |
| Root Greevil | e43b25be-fe67-4eaa-80df-85718f38f904#card | 4 | 37 | PossessiveNounPhrase | 02d3453ed9c4ea4ae72b2c567bde06a83991559e0d4a3d7acd839daed0031e71 |
| Root Out | bec03edc-261a-4b02-be37-2006f657e905#card | 1 | 21 | NominalCoordination | 2aab6bfbfd8f1de0ea00162bc723089284da779f5488b0861ac4d4735df14fca |
| Rootwater Depths | 2d28c83a-7415-4eb0-95a6-6245f2169d17#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 0ecab5b948d8c8e9938179fc960ff68e176864a838f6b314c467d34a8ed01d76 |
| Rotting Giant | 09bf80cc-1112-4f4c-9431-92d741bf2230#card | 48 | 33 | PossessiveNounPhrase | 07a9a7abeceff2d2b1f328a7b1c6f20fa0e703d0a574230a9fd40ecad82387f7 |
| Rotting Regisaur | acdbaa99-18c9-4aa1-8797-69837ca7d8f5#card | 2 | 21 | PossessiveNounPhrase | 75231ba32c6a2b3617c0147cd7377d003085dc587ff5f6eca8b7fd5605e23746 |
| Royal Herbalist | b3baf498-3be7-4a22-8fed-806d8d8ac748#card | 8 | 31 | PossessiveNounPhrase | 19b92c2e0ef5152ad89d3204b7821422373391149eccbcb3e7995a30d1da19a6 |
| Rubble Rouser | f35aa497-bd0f-4642-8713-820b9f014da5#card | 384 | 91 | PossessiveNounPhrase | 00306ce9d3baf7349bb77ebec2c384f2241fad30ca0d318afbf47653ca94ec70 |
| Rumbling Slum | 77275fc5-5f70-4d7f-938e-f32b5c19c532#card | 6 | 29 | PossessiveNounPhrase | 192688f9840b91e9488d13ddc59df1af11b96ae65384bf6fb462cb73f83136c3 |
| Run Afoul | 5462cb38-7cd7-46e8-a273-35d0d1af45b4#card | 4 | 22 | KeywordComplementPreposition, PossessiveNounPhrase | 5d301d471fb86cd45ce3e9ca8942e11e032ce02af908e0addfa2b34ab955e40c |
| Rust Elemental | 21af6b08-1dd1-4e7f-be7b-cc8e70c578c7#card | 32 | 50 | PossessiveNounPhrase | 1d3b4404d836956bb4615a9a28beecfe67a43cdc11e0ebcbab97fbea2bdee0b6 |
| Ruthless Deathfang | 69c22788-cb90-4100-a64d-891124082dd4#card | 2 | 31 | PossessiveNounPhrase | 38aed7cad0eff525812ec8f61cd2bafe4b5fbe456f40196adf19abd41ceef761 |
| Ruthless Negotiation | a82ea8f2-e576-46cd-aa08-9c98d6029db4#card | 6 | 49 | PossessiveNounPhrase | 5348de2a602bc8db7256b33045f2c753a26ad5606e8c196ec5820704d0208c5d |
| Sacred Knight | fe8f5cc0-5849-437f-8d1f-cb191f2fc638#card | 4 | 26 | AdjectivePhraseCoordination | 49b908d44c9dcd7596cb193d6ec38f1d50dac70564758a729557269d891ebc97 |
| Sanctify | c804ac8e-7427-43c6-b496-3ee6de192f03#card | 2 | 24 | NominalCoordination | 359b5dfd49d36bfe73715a480ae60fb4f13f14db83c194fe6abccf351a510d7a |
| Sangrophage | 5d03e6a7-81ea-47d3-add7-8dfdbbfec6d3#card | 16 | 30 | PossessiveNounPhrase | 13be6915b89b033caeed4aa988b466f9b1cf22eb3db036de1c06a506465fb7e6 |
| Savai Crystal | cd0c1165-673d-4936-8254-deedd0ebbe97#card | 1 | 25 | ManaPhraseSeriesEnd, SerialManaPhrase | 961e13808fb37afff57f7b11e172285d18bf60b2301608d13be939233fac4fcd |
| Savra, Queen of the Golgari | 641db975-cf94-44c6-a391-f540a952e0d8#card | 64 | 81 | PossessiveNounPhrase | 06c5bfb1450485faf0607d3904a0cd5310261c8e339b0de30a8d26e521e4d9ff |
| Scalding Devil | 3bb1536f-06cc-433f-aeeb-afd528f9af03#card | 2 | 26 | NominalCoordination | 9ab8b95d245e6b38321894db1edf976d32d93c5e198472c0a5b44155ffe4209b |
| Scattershot Archer | a38ef622-d793-4867-a818-5c27982608b7#card | 5 | 26 | KeywordComplementPreposition | 47df0dfe47d9b2cb93b34a4a9a3d6994a9ecb09c07d86d540724eee8b723d0e9 |
| Scorched Rusalka | dd05fd8e-f25e-4b9e-b162-afea1b248282#card | 2 | 32 | NominalCoordination | 5a41759af17390a307a6ec38da5e5113c415be33d434b0f9a257069cfbb203e5 |
| Scourge of Geier Reach | 1e02f371-5dce-415a-8563-1f93c35401d5#card | 4 | 27 | PossessiveNounPhrase | 0d7247b24debc9dcb317728f4665236dcabf00be784d26163176eb14e901764b |
| Scourge of Kher Ridges | a3ced75d-eb1b-4a56-bbe0-49e8b906f729#card | 35 | 59 | KeywordComplementPreposition | 13291642ab4ec9c575f64df183ebce534b85f75d1fd480735e30939c1cd14b17 |
| Scourge of Numai | 540d1878-7afe-48a5-8b40-3dd1b06a024a#card | 24 | 35 | PossessiveNounPhrase | 1336a9a1905097caa20ac4cf2d43db7634dcae0949498800964503d9edcd2613 |
| Scrabbling Claws | 4538ec2c-867e-412b-b6e3-a57f8e29ba84#card | 9 | 59 | PossessiveNounPhrase | 176971fc163c495cb59ad9348cae2c3602f70c7124d94ac9f3d034b7ad5fdb19 |
| Scrap Compactor | 7f113875-a411-463b-8b4d-91d1dee95c2a#card | 2 | 61 | NominalCoordination | 6d260aba4ea3a8eefe02881a9984fbcfe7de06a4b940830ee9ba4fc78c79e1b8 |
| Scrawling Crawler | 1d60c80d-9a95-4051-bbf7-02ca51ee1be2#card | 2 | 48 | PossessiveNounPhrase | 0592a26474629c866265e7fb08d6f376910300d1a70808e1b63bd9efbd99ed1b |
| Screams of the Damned | 831bb507-aaf0-4e6a-98d9-f010f96aff96#card | 8 | 39 | PossessiveNounPhrase | 00916f5c64a2c4f1ba8d37ab2e1f7624cdfd09571246bca0f3b7f352e75f9d75 |
| Scroll of Fate | c15c13d2-8db9-4801-a1f0-f087f2551c92#card | 6 | 20 | PossessiveNounPhrase | 5809036f25fff9833fcc19203ccfd38b28bb5187059f7348960a74eaf7a80fd3 |
| Seal of Cleansing | a75dbe70-7e3e-446f-9a76-9fbb414f2e7c#card | 1 | 20 | NominalCoordination | 6c87b5e6cadfe8c2b016e3730ebd53abcebf0d71a8f3bd2056d47025982fb7c6 |
| Seal of Primordium | f14dbb39-c9f9-4f64-b22a-38dba28f5b1e#card | 1 | 20 | NominalCoordination | 6c87b5e6cadfe8c2b016e3730ebd53abcebf0d71a8f3bd2056d47025982fb7c6 |
| Seashell Cameo | 383f6020-c26d-43e8-bb07-566886626d74#card | 1 | 17 | ManaPhraseCoordination | c62f46f68b3939ceaba2804969674d32f6e56531d8c519115ff7c873fd3beeba |
| Secrets of the Dead | b9720c63-5aa7-4fb8-8d71-8f55d8b1337b#card | 16 | 25 | PossessiveNounPhrase | 1a2fe080db5b7b4f90270c568b0b0bd4388b5944eb7af467e8148a37f3a96689 |
| Seismic Elemental | 61ec78da-979c-45cf-8be6-12aa24dc833a#card | 1 | 27 | KeywordComplementPreposition | 2c245c25a7bac17cfcab761576c7aa1d5719bb9515898fc2d6553a53d0d87df1 |
| Seismic Stomp | a1d02a70-2543-45c6-a9a1-c1941f2f68f3#card | 1 | 20 | KeywordComplementPreposition | 9794bdd1b02ddc40b70c691ec972bd65b26cb16e43744fb297ee0cb5561ee1c2 |
| Selesnya Cluestone | 792d4afd-b3de-4be2-b68a-03a5be7b9627#card | 1 | 44 | ManaPhraseCoordination | 828846df699a4afaaa264f0946110710dc558c218fdc524b7e3fca997824a2c4 |
| Self-Inflicted Wound | 28956db0-985c-402d-abae-1075c68b97e7#card | 4 | 43 | AdjectivePhraseCoordination, PossessiveNounPhrase | 05ea37fad75b68684c1186bb4a3de484dec0b38e56bd348e5984ba0e0e5d34fc |
| Serendib Efreet | dd520d83-297a-487f-b8f3-4997bbc056e0#card | 12 | 31 | PossessiveNounPhrase | 35a9147277b68e804ce83b1e5b02eed2da48ec07052a255405a877ef5f8f91a2 |
| Serenity | f4f751c0-02dc-45a2-a4b0-980592509944#card | 4 | 37 | NominalCoordination, PossessiveNounPhrase | 01554e7382359e2bd5690babc6f43b265a7de554d8d20f9f5b51652ab9b57478 |
| Serra Aviary | 7a06483b-e71c-4d11-86ea-3b48e2a9cdaf#card | 1 | 19 | KeywordComplementPreposition | 0dbadb489ccb2edae5670c02c09763d4f7fa961aa16a094311bcaf05133df775 |
| Sewer-veillance Cam | 25c83d45-581f-48ba-b6bb-feb66e8a65c7#card | 2 | 58 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 2517f2d72da9015e0d9c0ddf328fce6be1ec233fa8a384489ae03003216bea6d |
| Shapers' Sanctuary | 15bd8fa1-0858-4029-9973-e24c86e45af3#card | 20 | 45 | NominalCoordination | 029bd8ecb8d35bb5531fabe6aafacd8053733a0c6e22cbcde948b6c6f5ef2e80 |
| Shapesharer | 400c9fd8-c307-4b27-af6a-73e100717881#card | 8 | 35 | PossessiveNounPhrase | 1a173c40493693542434f64cf74068676119e588b8acaa8304374aa1f1e2dfe5 |
| Shattered Acolyte | 57f58d52-0014-4cac-94fc-668063cc147a#card | 1 | 27 | NominalCoordination | d99176d9a62940308a263493e208324e41d606dfdcbb966812f262084ec05ac5 |
| Shattered Wings | 709b9f76-0ea5-456c-b1db-2cf517370fde#card | 4 | 25 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | 1820ab6bd03f1bea84ba1083db1352c2ac856246be28c0ebfc6290724a195f84 |
| Shattered Yard | 4122720b-cad9-4ebb-b458-be71411c21e3#face:1 | 6 | 29 | PossessiveNounPhrase | 25a27d6912af1f32819986da6cfde8f1eb38711b10b9c81e76ba0018162dc7e3 |
| Shield Mare | e07b6988-9ee7-4f20-8aa5-7dfa87ff507b#card | 80 | 63 | NominalCoordination | 00177a0c983d37fa9d45682562100a3c59129b4091b5f454f8af70ec4a813273 |
| Shivan Reef | 0fe16212-66c3-4e45-a641-7391e9b2e304#card | 4 | 47 | ManaPhraseCoordination | 2e9a3b457c91dcb4b7dca7feef474229744fd8937753c3a11d8aa0e7910caf84 |
| Shoot Down | 2a89feeb-9e66-4539-bd75-7d9d53f52d49#card | 4 | 18 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | 026921bdc9d771d53835c669bc53ea1efdda9568c59d137c75cf83223bff384c |
| Shower of Arrows | efcbca5a-743c-4b16-a7cf-b07335b31ced#card | 4 | 26 | KeywordComplementPreposition, NominalSeriesEnd, SerialNominal | a364b8eaba6d9f730519ae05cf737a24c42852fa00423ab17e91b9372c624d61 |
| Shrewd Negotiation | d36fea45-d8aa-469a-bcfc-5c0e4a0e807b#card | 28 | 32 | NominalCoordination | 00b970fdd5f61efd0436a874c509b3ec052d82912ee050768de45f964e466331 |
| Silent Clearing | fd45063f-c83c-431c-9104-f139c497ec0d#card | 1 | 50 | ManaPhraseCoordination | e87fb35d64ca4bd29589c0c2f81a88ac669ac35e1b5a2bd26b4186ab61afc59e |
| Silklash Spider | 2d0c73da-ed8c-44fc-abd9-7c344b1f890d#card | 5 | 31 | KeywordComplementPreposition | 36b71114595558973ca16534cbcb74dab29cd1fa71add258f4856bfbd4f96f0d |
| Simian Spirit Guide | 44e0ffa3-8915-4c1f-8f1a-4aeea1365f07#card | 4 | 22 | PossessiveNounPhrase | 14e148d10664be94bc8ee9da4437eef675c8de62b0ea27cac513b8488e17c595 |
| Simic Cluestone | 0b65050d-a9cd-4b6c-9114-af89bfa6682f#card | 1 | 44 | ManaPhraseCoordination | 922d145bb9c7e8f722494c3f0538ef1365c671554c001555df8c031ff7c66181 |
| Simplify | 853cac98-34dc-471f-af87-1a94b0022b67#card | 1 | 19 | PossessiveNounPhrase | fa365c2bf0128fc687ad55f5f9a14980ce19839304b56ae88b5bb321b13b2679 |
| Sinister Gnarlbark | fcd2142e-ba84-4693-9b29-ecc43feba2b8#card | 6 | 24 | PossessiveNounPhrase | 745b293f43d995e24229fa674175203199a22e98a6e13a0766ca2a2c7a984209 |
| Sip of Hemlock | 861096be-dbb3-429e-820f-d82f6ed2aa24#card | 1 | 23 | PossessiveNounPhrase | 7caf15499c33cfa34571157766cca4de901f1ef034804bfbac7194ddf4c6e21a |
| Sire of Insanity | b55d8cd6-037d-4d7c-90ec-51249d34b02e#card | 1 | 24 | PossessiveNounPhrase | 124947c7d00446ee01ae30ea018af0390508a0c2f2a533454ece990b6be2d145 |
| Siren Stormtamer | 59c7496b-19a2-478b-b28f-6f153c2458ae#card | 28 | 40 | NominalCoordination | 1580a8cca1633221c696b72a4cc5231b2f2a241620c0dd157ce2f6baa7f5c8d2 |
| Sizzling Changeling | 60ad7aaa-8ce6-4449-8e8d-4ccf925010db#card | 16 | 54 | PossessiveNounPhrase | 0c8b6c0d521f226a443cb087dc935f33be026b05a043e92b80c45f806599e0e9 |
| Skophos Reaver | 4bc0bdd5-32da-46df-b624-29a709efc676#card | 2 | 27 | PossessiveNounPhrase | 0b9a2f977859b6881d3acb9c75ed912f5505000a4747dd8389ff652e490797ae |
| Skulking Fugitive | 4178ae56-2449-4617-aff8-f59d8fb96c66#card | 2 | 28 | NominalCoordination | 654baf11b835cf0f0f883b9a3cac772160c30191bcb951279db0ab75077fb3ca |
| Skulking Ghost | cd97f8b4-e751-4b52-987b-a442aef57809#card | 2 | 31 | NominalCoordination | 01523c6dc0cb1ac2c0a41b2838b7444d236da1c95b369377d697640ae80490ce |
| Skulking Knight | 11e4c731-4212-4aad-891f-fe066ed0436f#card | 2 | 31 | NominalCoordination | 6319fea7dc11a9fd669fe6368f0f3866874ba3c5fc1fe5d906a58adb290b76c5 |
| Skull Prophet | 39b51f62-8421-42d1-86d3-74fd6e6f31a2#card | 1 | 33 | ManaPhraseCoordination | 37b3dc3fb23a26d871cca40fafc1540fbbaaba788ad089d6af654e13c1c84d16 |
| Skullcap Snail | 6420e8a0-3ef4-4f95-bb6a-12409eef4d48#card | 4 | 26 | PossessiveNounPhrase | 77129d98a19e0ce0495b3b0a8d391642baf0613b644ea2f1417fc536b15a6447 |
| Skyscythe Engulfer | b7cc831c-c1a1-4ea4-ae78-5be2b6173407#card | 14 | 30 | KeywordComplementPreposition | 197e595c314060c2e19662a624049a4b9044434541af9621e68d44a375b55eb2 |
| Skyshroud Elf | 84a666a0-e453-4b34-815c-48dbfc9b4f4b#card | 1 | 31 | ManaPhraseCoordination | f078f858f5e2ff0156fb22b2917352487cc9117f88ae64343e3a37b8a08a39bc |
| Skyway Sniper | 8d907825-25f9-4b25-9fac-e0a207ed1797#card | 5 | 30 | KeywordComplementPreposition | 182afa3421b2aa8cb334ac9861111399bd63e672042c0ab4572c583ed34a1948 |
| Slagdrill Scrapper | 7f1edf4c-46a1-426d-8ce9-757a4c27367d#card | 1 | 28 | NominalCoordination | 0cc6f80b9e284ef4a9afe2b66a02c1e3126aad2e1f0f394436fefd7c40b61938 |
| Slate of Ancestry | a07483b4-c04f-42a4-b979-8b77c11fa8f5#card | 16 | 36 | PossessiveNounPhrase | 00f70ffc30c5bc34c5d3a88651f6430510accb8968d405978312acb0004f6fcc |
| Slayer of the Wicked | d93299d4-3aaf-48fb-8227-26203963c8b9#card | 2 | 27 | NominalSeriesEnd, SerialNominal | 101472e3b0aa887acd19d45a8859d1d5770f93b8953831e3d88684cdb58fe63c |
| Sleeper Agent | 627c9546-85b0-4b4d-93c6-e2ca8c3adf4f#card | 12 | 53 | PossessiveNounPhrase | 14b43800b96d4744f431a53c42d359d14c30beb91e6511a8c9be5dc5f7ad0f06 |
| Slice in Twain | a96d0065-b8c1-4843-8752-dce60bf83545#card | 1 | 24 | NominalCoordination | 78365c315f90ee39aee7007c0a1afac414a06cc10c3e8df8ba13832bbb18de05 |
| Slum Reaper | 0cf742c1-feb5-4108-9976-37fa58528212#card | 1 | 26 | PossessiveNounPhrase | cc803752ed14d81f99e1a23e922edc69a3ea7f7c366ff23e0745ad54a3b6cb1e |
| Sly Instigator | 8c2dd8dc-8623-4518-b3fd-dcb9f647fa98#card | 2 | 47 | PossessiveNounPhrase | 1b1174e531724b20b10f82d2eeea9b7aaa1275d7fd1a33c0da408f00f923047f |
| Smog Elemental | 8e51a07d-8618-4908-8798-bd28bc9b5bbb#card | 2 | 28 | KeywordComplementPreposition, PossessiveNounPhrase | 077284ff177316ed1b9778d8b06e478166a77456aa0cd85867afc2cbe4e349bf |
| Smothering Abomination | c3db28e5-6162-4642-a1c9-a7ace0737950#card | 4 | 47 | PossessiveNounPhrase | 37664730b9a9438d060697ec6387604c0a61bc4ca3c026b4daf3870cfc6e13f1 |
| Social Snub | b284913e-3530-4a7e-8c75-efb18788dad0#card | 64 | 74 | PossessiveNounPhrase | 03ea847a7c600be004b12f0d7f9ed6810775bf395450069da2f4c7d033679ee3 |
| Sokka's Charge | dcc29e59-341c-4175-aa54-f535e2c3e355#card | 4 | 24 | KeywordPhraseCoordination, PossessiveNounPhrase | 109219833a950f5e9fb5e04ccdd3a9adb5f77545b23df1f0a06dd369e332ecbc |
| Soldier of Fortune | 8b2367b3-3a8d-442f-8d58-6e4c03fa5a46#card | 1 | 22 | PossessiveNounPhrase | a4469f28a06e4a3e00ef063780d95c453ddb56ae52a3636c6c3b72fe41c16c32 |
| Solemn Offering | aff51c27-fb61-41b3-a7ea-7092cd280c63#card | 2 | 24 | NominalCoordination | 2094532db5c9e799cdb6d1996a4cc636bc60f6ea8f96e949160dfc634db28d55 |
| Solitary Confinement | d41ab41d-07c8-4f6d-be5e-7aec9f5f6365#card | 768 | 76 | PossessiveNounPhrase | 00d616e139a6712f7a8c98612d4c6995fd69ef4b28c025b0dea6b2c802f32c4e |
| Sophic Centaur | 5e8e443b-2102-4d91-b50c-1679dc893ef7#card | 20 | 40 | PossessiveNounPhrase | 0c5b2f9d079584be77dc826a3cd2fc1de5df685ad1a2888896fad1f2fd88db93 |
| Sorcerer of the Fang | c1d5a631-a963-43a4-a5c0-1ba23bd88d0f#card | 2 | 30 | NominalCoordination | 89ff35f9dd371803d44498e2cb176dbac7221f79e7f7d082a74b088c67c72aa4 |
| Soul Summons | 0f5b79ca-9f80-420b-a6c5-bb2a9a95c7e7#card | 4 | 18 | PossessiveNounPhrase | 07d96e0a0111214e686557ae10c475dc0586c8c3e1c0e3eb6f67787f83658a42 |
| Soul of Ravnica | eb014442-5935-4333-8078-ee4896dce653#card | 3,136 | 79 | PossessiveNounPhrase | 0014406398ea307a7de6a0a307fc92cc1ef4acdfa18d894cf0998d593e3465e4 |
| Spark Reaper | 894176e3-af9c-41ef-9f35-59dfdda3c17f#card | 4 | 31 | NominalCoordination | 5610c35ff24c91dafa72d12e09f7d7a502cfdf72d0553e4cefe3943c6c50b155 |
| Sphinx Sovereign | 41907508-9ffc-4e01-969a-4f9c35cdf548#card | 12 | 50 | Adverb, InitialAdverb, PossessiveNounPhrase | 2d8a198aaef8ede1031a61da1417e6940d3228d371aa71529bfd5f3c350b78ff |
| Sphinx's Insight | 01f675c3-d1c4-45c8-8a2e-75f535db3025#card | 64 | 40 | PossessiveNounPhrase | 0158c54a12d3eee67f8e226ffd86d84f40cbf08f620677f0c5769c629267d826 |
| Spin Out | c1b6795a-1ac6-4edb-bcee-4db1f1fbe8ea#card | 1 | 13 | NominalCoordination | eedea1cb737dd130ee76a101cdf4f7f3389b133152c8c58c2812429a544b67e1 |
| Spindrift Drake | 53599f89-1598-4d3d-8aa7-75b397b258eb#card | 16 | 32 | PossessiveNounPhrase | 12241d89886760656937714a7bc1d18454f95dd36d28b5351d21f4d5fd24a2fe |
| Spirit of the Spires | bf10ce45-c6d6-461e-ab46-06b4205d31fb#card | 6 | 29 | KeywordComplementPreposition | 5706873c4584ebd018c34271d62584f07dfe3230aec5e3f1f049389b2449de17 |
| Spiteful Bully | 410acdbc-3a00-49db-a712-ad970d24b363#card | 12 | 34 | PossessiveNounPhrase | 01ea0dfb1a8ae02f3d3053814231b99e8c3f55786f0cd2b57b824453c51158fc |
| Spiteful Prankster | 58cbac5c-d58e-4794-87f3-9563fce1b53f#card | 6 | 45 | NominalCoordination, PossessiveNounPhrase | 18b9517611924fdfa202359b050ec665296f46448c841cfd975e7b2a69c3105c |
| Spitting Spider | 24ddcf3c-a4f3-4373-9483-5ce6e50d6150#card | 5 | 32 | KeywordComplementPreposition | 00a5acda0e47361ac61616e56878002b4a3ac9d7b20e5f7940562ab0af551a6c |
| Splendid Agony | e9958344-13d5-4f7b-acc8-904e5d99b90e#card | 4 | 27 | CardinalCoordination | 14f06f5c900fc6f6efff3ebc8c43a62d9305a6821463f2e0d66ae2087a84adbb |
| Splitting Headache | 3a5937de-5957-47a9-9bd4-0dc7f8b833ef#card | 18 | 64 | PossessiveNounPhrase | 291fb4dee5b03b3320af6654d247ae63c11402f35ed028e34fb6559f021c386e |
| Sporeback Wolf | 6fd5f773-e53e-4a7d-9093-dddd9b2e741f#card | 2 | 21 | PossessiveNounPhrase | 2b972701ff431f4b691aa23069aaea601257b72fc4bee97862f15dfff1e12f5d |
| Spreading Rot | 3c1b95bd-a2fc-47cd-8940-e4af03bd3ce2#card | 1 | 23 | PossessiveNounPhrase | 8a55b2cb600c6359fd413e1034b15bc1ed4b8516a05a114a45ef3f81742190dc |
| Spring Cleaning | 9b3b81cf-f020-4e1e-95c7-db44ca153dc1#card | 8 | 43 | PossessiveNounPhrase | 1ec3300867a7d68cca9b491973ff8d177d9d2efcd4b818686ee48f01afdb2d58 |
| Springsage Ritual | 14945b80-10ec-4205-98df-cc3e119ca57c#card | 2 | 24 | NominalCoordination | 2094532db5c9e799cdb6d1996a4cc636bc60f6ea8f96e949160dfc634db28d55 |
| Squallmonger | 50bf5af6-50d7-4dea-936e-508e24db0a03#card | 2 | 44 | KeywordComplementPreposition | 5f7f9b09c47aff9d9af84004008b5fab3260d085171095e657ea26e8637a7cbc |
| Staff of Nin | 48d9f488-4ebb-4fdb-a568-6930fc2f5416#card | 4 | 44 | PossessiveNounPhrase | 4c55b4253c229ded5767a91175f7d9cfb405c2801a1012a70847da26d7fef97d |
| Staff of the Death Magus | c3fda16e-fd56-434e-91ac-561ae0482f43#card | 16 | 35 | CoordinatedClauseComplementPreposition, CoordinatedFiniteClauseCoordination | 02c1b581a3793aa10646cae0e7a476eba523bef7c2f4f85d01e4d5ea4ce66e3e |
| Staff of the Flame Magus | fc71710a-8294-4370-89da-7ab1c2b38d19#card | 16 | 35 | CoordinatedClauseComplementPreposition, CoordinatedFiniteClauseCoordination | 169da1df38dfe864ff7ff232b933eaf6d9c22abd0110b07c463f4a6ac7fd03a3 |
| Staff of the Mind Magus | 1466687f-cfe1-4e6e-ad8e-dca5e8e4c06d#card | 16 | 35 | CoordinatedClauseComplementPreposition, CoordinatedFiniteClauseCoordination | 111c0f21cd4f77f06759b109643687abb0c5622f189eb53bd745e5bf5ff91c7f |
| Staff of the Sun Magus | bd8bc927-e50a-4fab-9803-0602a841c7c2#card | 16 | 35 | CoordinatedClauseComplementPreposition, CoordinatedFiniteClauseCoordination | 1c5945bd4b61ca0e8db8f7a6cfc51787f8dc2b7459ddd69adcb9ba46f8a625ef |
| Staff of the Wild Magus | 6bdff38d-0a52-42e5-86ea-edd7029b44f6#card | 16 | 35 | CoordinatedClauseComplementPreposition, CoordinatedFiniteClauseCoordination | 256b084ae7ba55bd5ba98232148b59c7850e021599d96e7bfdf02b6b32ac9a61 |
| Stalwart Shield-Bearers | 55aafc25-76c5-4621-941b-754bd8f13e64#card | 6 | 29 | KeywordComplementPreposition | 3010f136567818e16c94c9b8fa41fa581fb876843a26a9824c23eee57a45c72a |
| Stasis | a8cf1379-0195-4e11-b994-481ef1284245#card | 32 | 43 | PossessiveNounPhrase | 0e37a08e8093a7c40b619e39c477b13320befa46a30a8a9d1cea99a68772baf8 |
| Statue | df7d4964-35a2-42f7-a4f5-05122f78cbba#face:1 | 1 | 15 | NominalSeriesEnd, SerialNominal | c76139b5c29acd671f383c33b2b8e5238a456f8d27f4b401ca1a99ea277131a5 |
| Stensia Bloodhall | 8220c5fa-28dc-40d0-a38a-d8eefc2795d6#card | 2 | 45 | NominalCoordination | 092d9eba1a85b4433f1d6d4fc8fd3a8d219ce4766c8e23cb3923506dfe5db749 |
| Stingerback Terror | 5cc6ca73-c38f-4ef8-a2c9-4f0342eebb73#card | 10 | 37 | PossessiveNounPhrase | 157a12b33a6093eecdd1ededa079459c1f30d2f7d40855cc9c6ad9442157aba1 |
| Stingerfling Spider | 056770e2-8ab7-424d-a2d8-7be6d6cc73f1#card | 10 | 29 | KeywordComplementPreposition | 4203e908c30845114afa63f15397bac853099a3bfaddc6f96c893e5fe5e06a1a |
| Stone Spirit | 63bc0a02-0d6c-4a00-8b17-1b024f830219#card | 14 | 25 | KeywordComplementPreposition | 0515d9102b3bf9024e1c710c968bab239ecde0baff3b68cf1195372bf532146b |
| Stonehorn Dignitary | 65368569-68bf-4498-9979-d2935942ca49#card | 1 | 23 | PossessiveNounPhrase | f171081ba2778fd78e0e9fd99fe91e0e0fa7d533144ab020dd89c12834f2c44f |
| Stonybrook Angler | d29d28c8-4ba9-4fcc-b0d7-5d7b3de89b82#card | 2 | 28 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 50d1c649d94d0a68b446d8cddaff9c08ada792c9affcec25c0ce8e189eb623c8 |
| Storm Fleet Arsonist | 36081fbb-413f-4924-a536-82add34e38be#card | 2 | 36 | PossessiveNounPhrase | 9bf3befde9e2b232e8971a4fdab3c152066a5cd9bbeb2e91b691639ce2b082c2 |
| Storm Front | 3c8044b7-3999-4825-8448-0a1a325f1b48#card | 3 | 19 | KeywordComplementPreposition | 0c75c4149d6d3f2c75bfc4b18e675b573b2e6b2cd217b36bed98b99462da311a |
| Stormfist Crusader | 0d3bebc6-662a-4cd4-ad1e-aeed0c5e04f6#card | 2 | 32 | PossessiveNounPhrase | 711cd41b5bdccbd4bcabd6812dff05f50c82ded863f8d30d0635edfbaa608f9c |
| Strategic Betrayal | 0127c13f-7b0f-44c3-8b35-97f306c5f239#card | 1 | 22 | PossessiveNounPhrase | 45eee51bcc882f8af989d1b2fc3a2d368a83085d8f562b6dc97aefc35d61c4d9 |
| Street Riot | 526db754-abb6-450c-bead-81dd1e1c347c#card | 12 | 29 | PossessiveNounPhrase | 1e3f613a2f1ad0c5c91278866595efa32365aa46fb6f9b37a6c51c40ad20579d |
| Subjugator Angel | 88b1e06c-5899-4e83-8204-e2c32c0c6aff#card | 2 | 27 | PossessiveNounPhrase | a4261525a084530d295466a2b5a06af0abb9c2713b1cdb4fd3475e3360782ec2 |
| Subterranean Shambler | d06be0a6-0d45-4e95-b132-88ce80dd7cb6#card | 9 | 39 | KeywordComplementPreposition | 27187bec7ec509e301c946563b2aa6eedc47253a7456e2f9b129fdf4ad56e8e2 |
| Sudden Edict | b95f704d-96b9-437e-8c49-aba874139e12#card | 1 | 22 | PossessiveNounPhrase | 7c6d5e031357657ef8bb29f4598145ef7da5f57ed04f8e3e1ec798e39568908d |
| Suleiman's Legacy | 9888d67d-fcad-427f-8ba7-b6d421b3d88e#card | 2 | 67 | NominalCoordination | d87d338926f03857103aa05d7be19db5f5fe6378b57ef62abddf58c0fb4c7d9e |
| Sulfurous Springs | f5c38c01-4a40-469f-91a0-7479daf4e8e7#card | 4 | 47 | ManaPhraseCoordination | 4c18e6a0dca3750609cd0b6d7a43260e9bb509233f613c65117a7367f224009c |
| Sultai Ascendancy | 06e92ab3-c781-4f32-9ada-09a388047067#card | 2 | 19 | PossessiveNounPhrase | 48a969216f014da632ea9af7c160d0075a6a1a5c1e3ddfe49bbee66465db28a1 |
| Sultai Banner | 93b139a5-2678-46a5-b8eb-45344ee3290a#card | 1 | 48 | ManaPhraseSeriesEnd, SerialManaPhrase | 106db8eaa5684d37401198cda9079ab14c6dcf4e65ffd70b6ae085c971958203 |
| Sultai Emissary | 0274f286-46a1-4028-878a-88c2146a895c#card | 4 | 25 | PossessiveNounPhrase | 60dc674abc32fc7b123431746748f6bc307c0ab9367e8fcc4b9024079687ef71 |
| Sunbaked Canyon | f97fd068-b83a-4621-bf8c-cc96e880ce90#card | 1 | 50 | ManaPhraseCoordination | 94bce7ff5e55a9c366256253cfd247c356f2da6ccb2cc92a925a4927f9a2c942 |
| Sunder from Within | 09f75f28-09b9-4a6e-a1b0-671208d274d3#card | 1 | 13 | NominalCoordination | cabe4a78ae86e4063d0bf611d0f25ab197368cff34283b0b2044fa93515eb159 |
| Sundering Vitae | 132c8a62-00bc-4b89-b775-099d8909f200#card | 1 | 16 | NominalCoordination | e3ca8ab9c55dd1f9b837b732aa9e3d4484723a3fae7a7a0936c43808edec5a12 |
| Sunken City | 17841133-c526-42a3-a1d2-a874e9bb7138#card | 16 | 48 | PossessiveNounPhrase | 0f4319782fa8b54be56ec5ac9981e6c1a06165e58c3768d6f7ea1928253fde15 |
| Sunscorched Desert | 256b8c23-589e-429d-9e6e-433d55079eb4#card | 3 | 41 | NominalCoordination | 0b7a111e87711a67c526e504ae4f73929dbb1f9673496036e9c85b464c2a68db |
| Sunset Strikemaster | d81ade13-e7c7-4fe3-bb0c-5007ed9f1198#card | 5 | 51 | KeywordComplementPreposition | 5b82afcada5746da22ac6204d68268e4ddebb3ebc49cbbb5ebaff0b8a9bf7398 |
| Sureshot Sower | 5bbc9d7d-91bd-4a58-8d19-f7be650720b7#card | 3 | 29 | KeywordComplementPreposition | af3044a386cddc1143a023d9898853b16a92559c41476f2d0311f3c94b304003 |
| Survive | 0b1f2522-960c-405f-9622-4cadbc7ba545#face:1 | 3 | 22 | PossessiveNounPhrase | 24a48e27b9212d2607e3fd20d744c3244fe7aa05ce98b3ccbee6c1ae794af8f2 |
| Survivor of Korlis | ca7a25f4-c280-48b8-9e04-b15f4c13e569#card | 4 | 30 | PossessiveNounPhrase | 146dcf27f536c495fdeaabf774c55c79a20a8c70d9bd726f54f1b8f850b153cb |
| Susurian Voidborn | 4f754f11-3b6d-4a4e-86d7-7495e475ef74#card | 16 | 45 | NominalCoordination | 0018aa47ee21574185a0120a0390913f05a335a1765477d5eaff995371ea8266 |
| Swarmyard | 4b508087-99da-4eb1-8b12-29162f2ec85d#card | 1 | 35 | NominalSeriesContinuation, NominalSeriesEnd, SerialNominal | 5611d7a84e3a31572e681fbca28f9ad3925c6b320ada62cfe8a782ca2530e89a |
| Swift End | 1080c5b5-6651-4c6a-93e6-099fbe389e26#face:1 | 2 | 24 | NominalCoordination | 20e6416b8cb9d3225ab4f535e94c69ca480a09e9447f0fc0458dccc7cd4f3acc |
| Sylvok Replica | 97818ed4-3590-4a3b-92ce-716f9cfdc78e#card | 1 | 24 | NominalCoordination | b2be503651d03c2fa9232e0002fc4f2bbe72d27cfe707bbb5f5df5d15ee8af2f |
| Tainted Aether | a61ceda1-5993-479e-945f-15753eeb7049#card | 2 | 28 | NominalCoordination, PossessiveNounPhrase | 447b39280b001fcc66acee339ca76c5448e8633af336b9ac4cc13a5a6ecbe800 |
| Talisman of Conviction | 6c326439-5620-4ec6-a56a-fe9c3d5d2a46#card | 4 | 47 | ManaPhraseCoordination | 240a60c23438c94c0cc45ecf366af1207da7da564fdf8c301b956b63a971b64c |
| Talisman of Creativity | 14d2979d-5728-42d7-a027-0eb1f754655d#card | 4 | 47 | ManaPhraseCoordination | 356e8135cd3d2b339d69e74097d34b7b1d244680fa9eec8d3f72b522e6c0e128 |
| Talisman of Curiosity | 8c34b089-aad1-476e-958a-3077bf1bbb51#card | 4 | 47 | ManaPhraseCoordination | 32ce4e52b5b8b877607b261157006cfff56a8a57a72f0ba76c8e91db8592994a |
| Talisman of Dominance | 4c0a0448-b9d6-43a0-8549-64066dac63f0#card | 4 | 47 | ManaPhraseCoordination | 29841ecc4e7159b7f210c5179949fdf5537f030ab3b0d359edfa26e653adf3f8 |
| Talisman of Hierarchy | b693c3de-2eaf-4850-b405-e79d00adefda#card | 4 | 47 | ManaPhraseCoordination | 07132787d75b9533db79b350d01650360c8eeae2a57fdca9d616796e24619217 |
| Talisman of Impulse | f2ccc9e8-8e92-4f8c-8728-8c748630e0dd#card | 4 | 47 | ManaPhraseCoordination | 4dfd38050f2cf620cf845e69c0aeb58e861caddf0fc3b783f7eaa04fa82ece6b |
| Talisman of Indulgence | 1d9aeaaa-66f6-41cb-9bac-162d6fd8662c#card | 4 | 47 | ManaPhraseCoordination | 1bda236d51c09ff8310463e54ee8d9e7920c88114c16d7e685caa81a9a65c697 |
| Talisman of Progress | 00e35322-1a9a-41e3-9ce1-359c8eaa3bc7#card | 4 | 47 | ManaPhraseCoordination | 1c200edd3a796da3cad0c5e297a910728527aed48d53a373a2ff9e6302f5700d |
| Talisman of Resilience | 42b8aa14-10bc-4bd6-88d9-4bb287eadd19#card | 4 | 47 | ManaPhraseCoordination | 1c94806151a91f1b2500d9fc153f3f32d5beb7fc4e80560d134196b02e28925e |
| Talisman of Unity | e5fcc5d7-6a60-4a5b-9d02-6c30041a95b9#card | 4 | 47 | ManaPhraseCoordination | 3880dd77d0be7641e2990a638cfdb740dd75625d8963bc2c0988b2193845e45c |
| Taoist Mystic | e705ec38-b8f7-4f99-8cb9-8448dc3e50ec#card | 14 | 25 | KeywordComplementPreposition | 01998458826a801c62544af24254cdfb759ec98194e2efe0fcf7b2a22fb0ae01 |
| Tar Pit Warrior | 05a7ca83-e820-433f-b9e9-151e817d3708#card | 2 | 28 | NominalCoordination | 654baf11b835cf0f0f883b9a3cac772160c30191bcb951279db0ab75077fb3ca |
| Taunt from the Rampart | 9138a768-d01d-4603-9bae-d3bff6cd2f61#card | 4 | 36 | PossessiveNounPhrase | 33c0a63cb827b2728a430943af72160df5daab886a0bdce5d35a1cc4869ac03b |
| Teardrop Kami | 6f47a67e-1fcd-4f3f-81d0-1a6868606390#card | 2 | 26 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 6166df36eace0cd43ff89a870d3dad6f76f565e260fe63be969688ea713278aa |
| Tectonic Break | 4a3c6d0e-6d77-4434-95f4-737879de026f#card | 1 | 20 | PossessiveNounPhrase | f40a8e699e8947e73e984adf503a4cb697081341c83f44ac9023f0ef6d54a2eb |
| Tectonic Instability | 0e68ee36-9649-490a-8483-fc8bdc23f161#card | 1 | 24 | PossessiveNounPhrase | f4988c8a6bb1016fc29d41d9d5f6d5453aa38ff2ddfecdd2e1aa727e9e010407 |
| Tectonic Rift | b94fcdd2-7e50-439c-b5eb-aaa9ec7dee08#card | 1 | 29 | KeywordComplementPreposition | 5ee8f134b55122d9b46d8be3980ce1e7a00f8678463eadd167e0b5e67d0db1cb |
| Telim'Tor's Darts | 0c978da4-9caa-4b4b-be52-889677962ec9#card | 2 | 29 | NominalCoordination | b34866cf0b044299e58e5021ed61e4078c199828762aa80be677a3f2cb156e67 |
| Temporal Cascade | c65dc9cb-3aad-4d15-a8c0-c0c142058d45#card | 4 | 49 | NominalCoordination, PossessiveNounPhrase | 2dca33d55593e4be7dab867788602ac4aba6db6b91e19859a361d2219a2a9f4f |
| Temur Banner | 8a35f23d-f59a-4677-9736-035c60c22a2b#card | 1 | 48 | ManaPhraseSeriesEnd, SerialManaPhrase | 6731ac9f6f0863621e1402861d6effb40cee509344667740b15de512f5cf8550 |
| Territorial Dispute | a1785817-f17b-471b-a63b-866e7972df1f#card | 16 | 47 | PossessiveNounPhrase | 26405a19b76dcc6f1bbd2d6a58037f6a5152a5d237d3d78628363fbce1332417 |
| Tethered Skirge | 42bbef66-967b-4922-8b2e-e57d1b8244bb#card | 4 | 34 | NominalCoordination | a23fb76eb3d86704a734367755c45831a17f8f74ffdf6434628dd9a93ff5001a |
| Thalakos Lowlands | 5a54d6a3-b1d0-42fe-9531-604b34d197f1#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 09f446cbbc4b58e4632f4a2fdf81112da5b21b21bdd3a46fd22d27f4201e0425 |
| Thassa's Ire | fb3b365b-eb8c-4695-bfd6-88060ad20741#card | 2 | 24 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | a31c5039c88567b097754d78e131d99561d121267406cf50201eee87cfc90c3c |
| The Arkenstone | 09f09d0a-6054-4545-b81e-0295773cc5f0#face:0 | 4 | 42 | PossessiveNounPhrase | 4c079d10a00911838e2d7dc459be57e97ab23731bab53a5e0dedad8e82b5bef6 |
| The Squadron Sinister | 3b7352e6-6508-491c-9935-426a6eebb429#card | 8 | 40 | KeywordPhraseCoordination | 0419c1abb0cdbe17577e9bfeb1e661e2bbc447e5fe4214a289ea131204082959 |
| Think Tank | c13b9ee5-cf4f-47bc-a6e8-1dbf98df3d9c#card | 2 | 19 | PossessiveNounPhrase | 35de7d1a324f1b3373147ce7046175da3ff09e7097a1aa4e6ba41951f70c6dce |
| Thopter Arrest | 9b9cc771-3988-41af-a49e-f3bb7a8e397c#card | 9 | 36 | NominalCoordination | 1323a09c6232bcddf18f005787ac7860a14452c546b0809d7a14dfbc7d5ecd64 |
| Thornado | 7476a14a-611e-4f81-b603-fd600e88568f#card | 3 | 20 | KeywordComplementPreposition | 5f1857754d148fc4ed60111ea776fc37357050db29b3ad7a25dd9ad1eef80169 |
| Thought Collapse | 6a265af8-448d-402b-b728-b7ed64521842#card | 1 | 24 | PossessiveNounPhrase | 6aea81d4701e511e5f13334b3097003ba08595bfcfc2eaad1ce9f8b326e468e9 |
| Thought Harvester | 13f9a845-d0e9-4790-99e0-b857204ad1da#card | 8 | 38 | PossessiveNounPhrase | 08dc47a0f1ddde50ebc698d109af76297a37e954c60430281f8a52e67cf8ba43 |
| Thoughtcutter Agent | 06ada754-da78-4eda-8be7-3ac4f06115c4#card | 1 | 28 | PossessiveNounPhrase | 57880115232b48af1f8bdd680399d4e93cb6d467c995122853d856552dba6875 |
| Thoughtlace | 6452b6a6-6235-46a3-a712-a26592450438#card | 1 | 15 | NominalCoordination | 3bf11fa23b9810f8e7e8afec4f112ed3a7b2d782e882ea4771b376cf3f88a44c |
| Thoughts of Ruin | bb9e1d12-ebcc-4a8a-baef-9f69638a51cd#card | 28 | 29 | PossessiveNounPhrase | 02dd4c995b9ef9c57b26e2940139b6c5bcc2708da52f4cbf1263c7cd57d14072 |
| Thoughtweft Gambit | ddf8d3d2-b0f9-43bc-9c9e-cb08fdedcc72#card | 8 | 27 | PossessiveNounPhrase | 66bd1210c366bd3ab87f75701e1491896fe6697f829b466131bb437a076ddae2 |
| Thran Foundry | 88079144-8d8e-4f98-9bee-3762540c94ca#card | 3 | 34 | PossessiveNounPhrase | 3ad0ef0caed2874e469276c12a29ecad79144a66407f9a2333e8df101ca94bdf |
| Thrashing Brontodon | 60bc63dc-ac9f-4a2f-aef5-c90d0aa31553#card | 1 | 24 | NominalCoordination | b3c24c793478a6cf883287fe4f6f921881ae37b9d181aebe3bfcb3cbe55499bf |
| Thraxodemon | 01cf689c-5a64-4ecf-b45d-3f385c9b212b#card | 1 | 28 | NominalCoordination | 4051c08d97ac5033bb1c5caf26a37bb845f591fb9737b52640ceb4f0d349cf50 |
| Thrilling Discovery | c5f5a234-c751-4976-a3e1-fbd52b3255c9#card | 16 | 48 | Adverb, InitialAdverb | 16f4d25660dc9a209b5f92055c3a3837a47c10d106b72563ef734881780cdfc6 |
| Throw a Line | dc69a130-967a-4d9f-992f-db7fad2fcd6d#face:1 | 4 | 27 | CardinalCoordination | 60dda7d6c23e8d5b36c166208f3979b9e6179d8d10f57d1d73fb2c74058c9b5b |
| Thunder Dragon | 70d9579f-c8a6-4406-afe1-9291fe5ee736#card | 9 | 31 | KeywordComplementPreposition | 13eb3ea1370b52295adbd1f6b556d3da8604043e18536d4b01362d44095b5e4b |
| Thunderbreak Regent | bb04f927-0348-4a14-9e74-381f18083f6a#card | 30 | 51 | NominalCoordination | 01a74d89b68da7a8bd3e2c8238854420e5810870d74db4101b8f82cd1302f457 |
| Thunderclap Wyvern | 15276d3b-a117-44bf-87c3-c17e032e4a26#card | 6 | 32 | KeywordComplementPreposition | 973c288ee0489af41d565662719da9fa8e9c02f664b79e107ba106ed841497b0 |
| Thundermaw Hellkite | e958144e-01bc-46ff-8f56-ddf2eb098630#card | 6 | 49 | KeywordComplementPreposition, PossessiveNounPhrase | 0a26a2ea5baf7c0dbe27c57059a21a765952d86da0c33040e22bd2722f317cc0 |
| Tidal Force | 1b25e262-e2df-4768-b55e-1b7b8d3ee993#card | 2 | 29 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 7743d8fc2ff7ef062869e91906b144d0ad8f21bc1585e90ce07147a1bbc6d132 |
| Tideforce Elemental | bda87ca2-91d0-48fa-bb82-a3396f0438ac#card | 8 | 57 | SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement | 3b2395a2113b6fb86d4d74710c9fcfc4ef3d5595fe0171c4f62be16301df0c60 |
| Tigereye Cameo | d2be289e-e560-405d-9728-d8a4ee9cbf56#card | 1 | 17 | ManaPhraseCoordination | b6381cf244e214cc1d8c4c20181b4bf4ff1591aaa1e56742f0368515a0b73b81 |
| Tolarian Serpent | 684e18bf-1390-4d12-8e7a-e8562db64dbb#card | 2 | 22 | PossessiveNounPhrase | 5198f495d96c429ad7499bc596cb140356a348d89f1b8896b7f970664b47fc27 |
| Trade the Helm | c478fb5c-9fde-400d-a2cd-f8c4fa6b957a#card | 24 | 39 | NominalCoordination | 014d9d8da9cd74c42d63989617d2c393b050982853e04924c75f53958bee4554 |
| Tranquil Garden | d9dfef08-b824-4d56-a0e9-3dcefb7e4612#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 0525aca441acf0e88b85a3c0299b1f2df8d7ac4d993d577a14f6c0edb834ab0f |
| Trapped in the Screen | 20f2fc0e-b1d4-4f80-9c7a-a9fb25fef7b8#card | 9 | 43 | NominalSeriesEnd, SerialNominal | 00aa8bd4b398eb16d442dae8c16262b8bca7fc262ce0bcea8de34ba67e64d0a2 |
| Treacherous Blessing | 17f573c4-3ed4-453e-bfbe-4e42d54633e5#card | 16 | 69 | NominalCoordination | 04da266bc6217659fb9d508417f8066e364f064fdb8c286adb0238364888db27 |
| Treasure Nabber | 20a6d576-a5b2-43b4-8be9-75be001e567d#card | 32 | 43 | PossessiveNounPhrase | 147031de8446f4c9963da310b940d06cd49302033ff975f28da3540720cb9260 |
| Tremble | 0fa0f562-e66f-4630-890a-9a4ae24fd6c0#card | 1 | 19 | PossessiveNounPhrase | b21a72cf943238a533f8709e4a5247074d426a5362b891adc8f5358621b1fda0 |
| Tresserhorn Skyknight | df344469-1177-4168-91af-3621b2a65182#card | 110 | 36 | KeywordComplementPreposition | 00d56b0a493a84fbeedf3f42c1bc601999ca41c7fd1d86bbcf52dadd31e6a67a |
| Tribute to the Wild | ba0fc2ca-7084-454a-9d4e-5cbca945a7ca#card | 2 | 21 | NominalCoordination, PossessiveNounPhrase | 391d4df3dcd5830244283471d577e556b8df597d5f4ef51498309effc4df5fa7 |
| Trickster Mage | d5a9fd97-535c-4aa4-9777-6cf8a413b6ec#card | 2 | 38 | NominalSeriesEnd, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement, SerialNominal | 853f7712b3a11bb3bb57046f31f0acee5f962f94bfe38fc09faf69c4e90ee6a1 |
| Trip Wire | 5751df7a-aa9a-4a27-b7ab-445ec4bcbdee#card | 3 | 14 | KeywordComplementPreposition | 0bf0f1e7232ff20fa9190ff864bd56efa8bb70f53d6a9a12d7fedc1546dd576a |
| Triumphant Adventurer | 6aac0ae4-d9a7-4c40-a39e-229186c84d74#card | 2 | 35 | PossessiveNounPhrase | 5fda5c0673581c5839a5098a839ef03326a0fe539e37fe1a62dbddce09ecff17 |
| Troll-Horn Cameo | 98e042de-05f4-4e2e-b12f-375b905e6600#card | 1 | 17 | ManaPhraseCoordination | bce4667f7da69a01d93f18ef59ea16c4957735064480dd11aaa758832f0cbfea |
| Troublesome Spirit | 601d9bc3-ca75-43de-beea-ab3f0a49f543#card | 4 | 29 | PossessiveNounPhrase | 164ce75aa33be5a4dd1cfb5cdff59eace24bac94d057bde633a12d2a7e55989f |
| True Conviction | fc299c1c-50f3-492a-b6b8-a3664bb72ab7#card | 2 | 19 | KeywordPhraseCoordination | 95ce6c8f894b57a2f659f10b978cd4a7e9034b8f6bf04f0724b39a1b221de363 |
| True Love's Kiss | 9506aba2-5e1d-48c5-ace8-4beb824ad8af#card | 1 | 24 | NominalCoordination | dcdc2f5b5224a999aaddc8920a67a1153bc5c55360a2ee78c4a37194a0f802ac |
| True Polymorph | 8d24e181-74c1-4913-a18a-59378ec7d9e7#card | 2 | 25 | NominalCoordination | 105317a8a7bd39fdf9f7f77f63ccfe6b79167936ea4be1879e1e5c165aa339dd |
| Trumpeting Carnosaur | f2ef8bda-373d-4387-900d-0f1b6ccf72e9#card | 2 | 51 | NominalCoordination | b3f061559d58f88c10cc7397031e63abd2ee9de56a2184616409bb9386b0088a |
| Trusty Companion | bd8c633c-ba99-47ac-a1d1-9e15aa948933#card | 2 | 19 | Adverb, SecondaryAdverb | 43a97f9fc79beb76df5b289f813ca50e92a96836c21645a3886c07e3a6d5f164 |
| Twiddle | 773ad2ef-5acc-49ea-8d85-056330e87039#card | 2 | 23 | NominalSeriesEnd, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement, SerialNominal | cdd0112c9f879b0d1d5d607a0286c1c40c6a8b1a77791ca813e759c8dd0c1089 |
| Twitch | 321fd970-a259-448d-bd98-e85e386174f9#card | 2 | 34 | NominalSeriesEnd, SecondarySelectedHeadCoordination, SecondarySelectedObjectHead, SecondarySharedObjectComplement, SerialNominal | 0b20d49e1f6679df07e77278fec8821347851dafb9f9db72a2a5a85e32bac6a0 |
| Tyrannize | 27da0af7-85d3-42aa-9426-b86c5e464f81#card | 3 | 23 | PossessiveNounPhrase | 44926d5d16c24af1241e8dfb28c14e04f54f1f5a5fc3323245f6efa23bcfaf47 |
| Ulvenwald Behemoth | 90379825-1894-4b79-bf72-445175e97ffb#face:1 | 8 | 33 | KeywordPhraseCoordination | 638ee6c668513c8f3d53da6d9a2ad7cff4b50b6e69ee3ed03300b1c6104c5dff |
| Umbral Collar Zealot | 12db6263-75c2-442f-a1a5-7af7915f8f9f#card | 1 | 18 | NominalCoordination | 4367f6013de7971fb7fd43994ae76bae41dd9fff0fe8e977f9e8927cc96db4a6 |
| Undead Slayer | 08fc069e-7fdd-4726-aa08-a96bbd2bcaa3#card | 1 | 23 | NominalSeriesEnd, SerialNominal | abb2f0ce7efc5a1e6bbebacfdcc7bfebce07addab2b88a7a1ff95b6a733bc842 |
| Undercity Eliminator | 93cb9c7e-d6d3-4b1e-8bcf-2a4a1d95a549#card | 8 | 48 | NominalCoordination | 06dbe34641d884e858f638fe9ceab4bf7436f47fc766416c3b1d69acefe9f1a7 |
| Underground River | 857febd9-cdd7-4f8e-a852-d88084b0cfbc#card | 4 | 47 | ManaPhraseCoordination | 336da34d04b7c4d6e499ba2104935b8cfead341361fa42483bb8ec9c81cecfb7 |
| Undergrowth Leopard | 511fa8a3-0a65-47fc-9f3b-57eaef2698cd#card | 1 | 27 | NominalCoordination | 14253404dd5e413acd8d269a7ff786e413b6fbce20df0344c1de157ed18f9880 |
| Undermine | 6bcfe240-c165-49f2-a90a-be05c05498a1#card | 1 | 23 | PossessiveNounPhrase | 661e5729d0473826b6387667d88809683337acba0f3d0d05d634c16abbb1227a |
| Unholy Annex | bd388ad9-a47b-4b0b-b94a-8e4343cd3de5#face:0 | 32 | 64 | Adverb, InitialAdverb, PossessiveNounPhrase | 08207ddc396c3782e622e957a810992999b455ec98fe6d5a0573640ff930b8c2 |
| Unholy Fiend | 7552a9b4-b82f-491f-ad52-e271cf730211#face:1 | 4 | 23 | PossessiveNounPhrase | 555f45dba70c31bf9b9d76b39b187ec49fab138b3536e6fa8a82b57156ebf088 |
| Unravel the Aether | 128e0bd1-ed1d-4f53-8dae-83ee95f6fa2a#card | 2 | 29 | NominalCoordination, PossessiveNounPhrase | 42617e6aac5711e87e3b3bf9e04ee809bd1803945521f0ebb27051e2390abbec |
| Unscrupulous Agent | 3e7879d5-62ea-4c9a-9fc0-659d70f3a8e1#card | 4 | 26 | PossessiveNounPhrase | 77129d98a19e0ce0495b3b0a8d391642baf0613b644ea2f1417fc536b15a6447 |
| Untimely Malfunction | 2c49d24a-98a4-43c2-bb59-1ef13d4214c2#card | 9 | 65 | CardinalCoordination, NominalCoordination | 054c9fb0bb77c0db998d213230360c8a74ea57817778eb1480e1e43aa509bb32 |
| Unwanted Remake | 90aa9113-5148-486b-a1c6-b4230e6d3e41#card | 1 | 20 | PossessiveNounPhrase | f1a07795a1b1f31d14c764c5f95b3f83657b7c1dcf6e42069d8054afcbee4697 |
| Unwilling Ingredient | 40d483fb-0dc7-413e-87f9-8d7cb7883fca#card | 16 | 42 | PossessiveNounPhrase | 035aaf9ee528a4865a0ba3108247587707870500076f7781bca3f4296f04d144 |
| Urborg Elf | 9c382817-5c62-44ff-8f29-d283e65712a1#card | 1 | 20 | ManaPhraseSeriesEnd, SerialManaPhrase | a25a850ff3bb9dc06572462a8de8f880d165206577f47b3438e8994c43e01779 |
| Urgent Exorcism | 2f87ebc6-68b0-4121-bfaf-b412b077d75b#card | 1 | 13 | NominalCoordination | d4f6de074cb6691ab745dfd1c2bbcabca1e88c056cf47471e601540db626bdd8 |
| Vault Guardsman | 0068f8c1-d1a2-4f7b-b39f-963acb2c023b#card | 9 | 39 | NominalCoordination | 0247ace42a64aec2505b842db553236d366f801856b581aa47ceb12c6a8d2da7 |
| Vec Townships | b0a4680f-9707-431c-b5d5-7d4424783602#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 0525aca441acf0e88b85a3c0299b1f2df8d7ac4d993d577a14f6c0edb834ab0f |
| Vectis Dominator | 1ed63322-f877-4f1c-90bd-e1adb3668d6d#card | 3 | 25 | PossessiveNounPhrase | 81da90016fcf0fe64b95e74dd9873fb9db77ce4e643100db344526c90d92a341 |
| Viashino Pyromancer | 5e78e6e0-60f0-4c3c-b8dd-bd673f5152b8#card | 3 | 27 | NominalCoordination | 29074c37292cf975b75e3fadff24b42a338e4f611cc2c7a28e85834e7a57cfee |
| Victorious Destruction | fa5d6c02-53b2-4144-87cb-e223b529ba7d#card | 1 | 25 | NominalCoordination, PossessiveNounPhrase | baa7776ebf8167875d4a6cc17277e3cb25c898047e983584dd2149cad2162e39 |
| Violent Impact | d48d9e93-72a2-48f2-8164-8032615ee217#card | 1 | 18 | NominalCoordination | 7c9cb52575e26f303c8d7b6cb9614c387a635bbf94ddbfb0e56b1bc600f693c3 |
| Viridian Scout | c5b4bc25-0f32-4e1d-b6fe-f122140f2207#card | 5 | 33 | KeywordComplementPreposition | 2ec884270581edf621b0fd8ad9c5c5f531398448c15a38ef1319cef9b6947d0b |
| Viridian Zealot | 9410ae41-b6db-47c6-ae0a-4e33aad1d3ce#card | 1 | 25 | NominalCoordination | 4cab56f4215336dc2405f3c8d5bcfeb8dd9029371f19c98e85fd9fe2a4e70f42 |
| Vision of the Unspeakable | 464372ef-0b16-4b8d-ba6f-fbf5c905c479#face:1 | 10 | 31 | PossessiveNounPhrase | 1676e44b28ef410a4113e568e3140f8dbdb2164461c96c1c68238f0f0da5f3f4 |
| Voice of Victory | b12cbda1-ac09-49eb-96b3-a4df27bc37ea#card | 16 | 26 | PossessiveNounPhrase | 12556f0212a36cb2a83d058864381454da0f55ac34d18c7d421c25bd52845c7f |
| Volcanic Rambler | 97f929ce-3eb3-4040-972f-0b21d5f2f61f#card | 2 | 26 | NominalCoordination | 9ab8b95d245e6b38321894db1edf976d32d93c5e198472c0a5b44155ffe4209b |
| Volcanic Submersion | 37985fef-696e-4c6b-9a60-29c5051b5a25#card | 1 | 18 | NominalCoordination | 7c9cb52575e26f303c8d7b6cb9614c387a635bbf94ddbfb0e56b1bc600f693c3 |
| Voltaic Servant | d0c2703b-bd74-41ac-b516-c46c654de1bb#card | 2 | 21 | PossessiveNounPhrase | 4ea21d7c40726dace4c8ec5c3dc6b852163ec530cd0a2444f630537bc3546abe |
| Voracious Varmint | bd785a7e-df65-48e6-843b-7831f3c1e5d3#card | 1 | 27 | NominalCoordination | 14253404dd5e413acd8d269a7ff786e413b6fbce20df0344c1de157ed18f9880 |
| Vraska's Contempt | 38ddbe0b-07ef-4cdf-ad6c-cd8b8ba6d206#card | 2 | 24 | NominalCoordination | e8cff789e60012d3148ee4c512d7d9cfba0ffb17cb03e61f2c1819ec0aa4453d |
| Vulshok Replica | da8e2236-88ae-4f30-884b-b961231fba8f#card | 2 | 32 | NominalCoordination | 345e25c7e91a36b03c3ceac8edadc844c66ac4dada985814ccf2698b28d7582a |
| Walk-In Closet | 52e77cc3-f8e9-4a20-811b-fe1e46a96ad7#face:0 | 16 | 21 | PossessiveNounPhrase | 08317b437d0cd2064872bf5abc693c38eb130b8f8cfc44ad9377c27b5fd88284 |
| Wallop | e939ef5e-ebe1-48cd-9e9f-eb3fd516165c#card | 4 | 18 | AdjectivePhraseCoordination, KeywordComplementPreposition | 50277b2a8dce28256e6bb715acd4553fd812afc3daf896922ad1d5f981dc64f5 |
| Warden of the Woods | c8c6924e-9af8-4212-be28-b0b4eaa4c7d8#card | 10 | 44 | NominalCoordination | 21e23b4a239c5bf523abe6ba466dd3db4c42d3e8ebde3df976d98ab2d242b9f6 |
| Warehouse Thief | ddf34a38-29d5-4bf3-9b08-e7c9579a4200#card | 16 | 61 | NominalCoordination, PossessiveNounPhrase | 16f1d3f41f82c9eb0365980c021a3dedf8a3a1d50735d3534b822294f864b18c |
| Warmonger | bae92332-1a0b-476b-8719-190e8d8cc03a#card | 2 | 44 | KeywordComplementPreposition | 23d8a0974928b1a06e973d18b8c016fac21282b77acc67ce46edc3eea6cb0453 |
| Warrior's Resolve | e3f68aff-9347-452a-8f26-b344016a275d#card | 72 | 68 | PossessiveNounPhrase | 0301ed0e4598b0fe768c31f7a018983db5cb5be23c306718f38f7c36c5bfce4c |
| Wasteland Raider | 29439669-c9be-46f3-a62f-16de51fefe88#card | 1 | 31 | PossessiveNounPhrase | 6876e4b481e287126c7195737752265547d2323dd6318381df9dbee6b6585717 |
| Watchers of the Dead | 9ac46f46-3099-4ad9-ae58-bf25d7270e87#card | 2 | 32 | PossessiveNounPhrase | 26e1286b0d77caf15a74d5d9007e17340d710cc5785d460b4ee4af15f476bb9a |
| Waterlogged Grove | 70fa2eba-565e-4fed-adc9-7f5d9fcbf1fa#card | 1 | 50 | ManaPhraseCoordination | cbb5bc514a03add4cc8f3f6a125909271fbfba9493354e598a26b5479bdfe6e1 |
| Waterveil Cavern | 3debfa0d-9945-4a85-a714-7c3d3d74de4e#card | 12 | 50 | ManaPhraseCoordination, PossessiveNounPhrase | 0ecab5b948d8c8e9938179fc960ff68e176864a838f6b314c467d34a8ed01d76 |
| Web Shot | e0b5b3af-af89-4b91-8b55-947ec095d6f2#face:1 | 3 | 14 | KeywordComplementPreposition | 22b6ddd29f72e754cf855043218718920f5fd06d4fe2ae7bb85be5e8ded8fa07 |
| Wedding Crasher | ac3d07cd-88c4-4e22-9d86-f92d34f406d4#face:1 | 4 | 31 | NominalCoordination | 4ed176015af5761fee76222ac57de3329d45ba4ed402490fe4b51f2769f59778 |
| Weftstalker Ardent | 926d52a5-4db1-46ce-9567-17c28bf56ae7#card | 12 | 38 | NominalCoordination | 02e1daf7c2923912ced0cb708cbdf02d35dcb8a3db621702adb0caa0b85d46f4 |
| Whipstitched Zombie | c498f0ff-812a-4106-834b-f43a0a7e0bac#card | 16 | 29 | PossessiveNounPhrase | 01a11e81e9d6f9b4c9d32ca879a25dc41d6d82865c8256454a86ba8406a67155 |
| Whirlwind | d3946df7-cb24-47c0-a8b0-1bdf95d99926#card | 3 | 14 | KeywordComplementPreposition | 3ba5476cd2daa62e0ac8f54ae7005e047676138af108df351afc193ea30a3c82 |
| Wild Leotau | d5277fe0-b707-41bb-9ff9-62f8be249dd2#card | 16 | 29 | PossessiveNounPhrase | 08efd21b823c5ea0b7550df5d8712c5e5f552944c0410eb8a4233b301ce35f2e |
| Wilderness Reclamation | 6f856f99-4cb4-479d-958d-964220965ed6#card | 4 | 26 | PossessiveNounPhrase | 2b563544fc56baf0e45a3c7f074d95032448ec1874b7b80bda21b40356250b2e |
| Wildwood Geist | 9fb4aa2d-6414-4176-9070-e5230e100c6a#card | 2 | 21 | PossessiveNounPhrase | 1c1c57d61d4b83fb5bfe855f37db384158fb9994bb964ee94c4a60582c50590a |
| Wilt | 77b6c3bb-bd05-4547-8f3e-9c50879973f1#card | 1 | 18 | NominalCoordination | b0d4b7cd982e362568f79e3e3aee2bcd2954e110cb495f8105671defc0c875ef |
| Windreader Sphinx | 5e038a3a-fbab-46c2-8773-1327d953da16#card | 2 | 29 | KeywordComplementPreposition | 34a1eb2f983bf4faf08b1f6cd65ef65d885fe4072260e65ac4567904ebe93cbf |
| Windstorm Drake | 16977ebd-6384-4c28-b9c3-448563a07807#card | 6 | 29 | KeywordComplementPreposition | 0489b486914e21bb6730789fd4e74b790e068fc701e991de7327dff00f6a00b0 |
| Wing Snare | 61ce6ec1-79d8-4dc9-ab2d-1623ed1459c1#card | 3 | 14 | KeywordComplementPreposition | 22b6ddd29f72e754cf855043218718920f5fd06d4fe2ae7bb85be5e8ded8fa07 |
| Wit's End | 97f11aed-7a5c-4a14-8c2c-5c230c1f05b4#card | 1 | 14 | PossessiveNounPhrase | 0c0cb1f955d807054171c490ff485b627380f907ee90cad61f03a0c2bec490cd |
| Witch Enchanter | 0355249a-8e4e-41db-9cea-1b901faffbe6#face:0 | 2 | 26 | NominalCoordination | 50e476d7c4e6bc85bf28b240b6a6d3972ce5e6c6ea18eef09efc11fc2e306a17 |
| Withering Gaze | 9225016d-adfd-43c6-99cd-d41a7e0d35d6#card | 40 | 38 | NominalCoordination, PossessiveNounPhrase | 02c8508c30aa7210bd695058306a69d13d1530e0920f2adab222f72a8c0403cc |
| Withering Torment | ffce81c5-1b58-4882-a4e7-6f8d7cb170de#card | 2 | 24 | NominalCoordination | 03a6a07464a02f5b7dfc1718385fd9b4f080bd951229a53bc592a44ac4dbac62 |
| Witness the End | 340d2261-9454-4ab0-827c-49c7e76c7781#card | 2 | 28 | PossessiveNounPhrase | 0d61e0282e6901e8873d3b53acbc8298913a7f649ecdfa8c708339e5a5a25797 |
| Wizards' School | 51635f96-af1d-4b33-9118-432495aaf07f#card | 1 | 53 | ManaPhraseCoordination | 302bf6da9b2444e13e948958fb833f19f07a2694e6bc2da223251f76e491d7ff |
| Wojek Bodyguard | 8de57b66-cb97-44e4-8c0d-8ecf3202c299#card | 4 | 21 | Adverb, SecondaryAdverb | 0c95e8c8129d705c2839b497b454af7cdebf2393ee6a67215667809b9b8ddaba |
| Wreak Havoc | b1d2519e-7340-4ea4-a62e-cf0740d4e57a#card | 1 | 30 | NominalCoordination | 5861af4b063ec7a5f84ed87d9e7ac3c1df845f38dc17f758b061dd31fa7becdf |
| Wrecking Ball | 0a1b0910-3b0c-4aa3-b47f-8eb069122743#card | 1 | 13 | NominalCoordination | 41f720c7a984a12e81be5dab1993e45d7cd9c083f858e73ef12a07bf797ca767 |
| Wurmskin Forger | c94332ac-982a-4c0f-85d4-43e950ac3069#card | 5 | 36 | CardinalSeriesEnd, SerialCardinal | 914cf3eefa74e6f1f18d75026082f3dbad05245823766202ced6888e67e40ffa |
| Yavimaya Coast | 40b36bc6-c185-4bda-99e7-0118953c2c97#card | 4 | 47 | ManaPhraseCoordination | 13541375a936d01538717880d749ef736c3dc14ce7c9c79b8258703a9704c5ab |
| Yawgmoth Demon | 6c54fc14-2af8-46e8-a4dc-b2a0a88ef2e1#card | 224 | 62 | PossessiveNounPhrase | 011597520c70de463ae500b6b49ef880925a8e6909c921f265074e45d2fdabf5 |
| Yawgmoth's Bargain | f7f76f39-a0de-4bda-86b6-0f291892fcec#card | 2 | 29 | PossessiveNounPhrase | 10809d40db417613de521ebf6d849ed234f8f46fa5b2d416536df38badf20392 |
| Yawning Fissure | 634ec9d3-24c9-4090-a087-2624b6d5bf5b#card | 1 | 19 | PossessiveNounPhrase | f8ff1cc3bc12c20de882b4d05a79bafe1686b61d1af20750b7b1ddf52a565ff5 |
| Zagoth Crystal | 87edb566-8929-4961-9c03-d3657a8c6feb#card | 1 | 25 | ManaPhraseSeriesEnd, SerialManaPhrase | 8acb24b74e01e066b8ac9bdf1fb9d76c7735c80ed5676055fa5a67c7d5507fa3 |
| Zamriel, Seraph of Steel | acb0fa6a-89c8-4957-881e-c61659e86f97#card | 8 | 27 | PossessiveNounPhrase | 0774ef39262ef4f8573e2d56fffe20c9b7f4591f4a7866ce6a8a96fdc6395050 |
| Zealot of the God-Pharaoh | 2911d52f-7faf-455a-acdf-77269c9ee8ce#card | 2 | 26 | NominalCoordination | 0223a498900ff1dfb567ba2b3fc5c26a64bbee7dd4d4369bee3ac91a356429fb |
