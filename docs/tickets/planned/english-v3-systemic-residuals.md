---
needs: [english-v3-frame-coordination, english-v3-then-sequencing, english-v3-lexical-gaps-batch-1, english-v3-granted-ability-quotes, english-v3-clitic-contractions, english-v3-library-position, english-v3-fixed-cost-phrases, english-v3-comparative-quantity-determiners, english-v3-comparative-complements, english-v3-postpositive-comparative-determinatives, english-v3-scalar-property-values, english-v3-copular-scalar-location, english-v3-as-long-as, english-v3-instead-replacement, english-v3-cost-scalar-complement, english-v3-remove-from-complement, english-v3-that-degree-quantifiers, english-v3-combat-interval-noun, english-v3-where-variable-clause, english-v3-each-of-partitive, english-v3-causative-have, english-v3-focusing-only, english-v3-once-frequency, english-v3-become-adjectival-passive]
---
# Reconcile systemic residuals before production cutover

The implementation is staged through `english-v3-lexical-measures` →
`english-v3-article-variants` → `english-v3-keyword-labels` →
`english-v3-frame-coordination`. This ticket owns the final cause audit,
remaining obligation routing and evidence for the cutover decision. It is no
longer an omnibus grammar implementation ticket.

Reclassify every remaining no-Reading identity and every unmet structural
obligation after those batches. Preserve the identity-level reconciliation in
ignored report artifacts. Distinguish a missing Lexical Analysis, missing
Production, failed feature/frame constraint, missing sharing/discharge, invalid
Reading, and compiler/parser/internal failure; absence of a Reading alone
establishes none of these causes. Select focused discriminating witnesses for
each proposed cause, then mint bounded implementation tickets for remaining
systemic repairs and make them prerequisites of `english-v3-production-cutover`.
Do not mark a cause long-tail merely because its whole sentence is uncommon.

The baseline's 6,396 initially unclassified failures remain this ticket's
responsibility. So do residual extraction, tense, passive-temporal and scope
obligations, flat serial coordination, within-group Type Line ordering, and
richer document forms. Test positive/negative structural witnesses for each
before assigning an implementation owner. Concrete starting points are:

- Shadowborn Demon: existential *there are* host; its NP reads after
  `english-v3-comparative-quantity-determiners`. No implementation ticket owns
  existential *there*; retain this host obligation here for cause audit and
  bounded-ticket routing.
- Ashen-Skin Zubera and Boldwyr Heavyweights: finite preterite in relatives,
  with temporal Adjuncts preserved.
- Aggravate and Ballista Wielder: reduced passive, not finite preterite.
- Assassin's Blade and the ten other passive-temporal identities in the
  [named register](../../english-grammar-migration-obligations.md#named-identity-obligations):
  a temporal NP must not become a passive Object.
- Absorbing Man and Titania: auxiliary Object Gap; Infectious Curse: both
  relatives and the ordinary comparative-cost host.
- Phantasmal Form: flat predicate list; Boros Battleshaper: shared auxiliary
  scope; Council Guardian: attachment to the appropriate nominal constituent.

Reconcile every section of the migration register and activation obligation
report, including all 20 named register entries, 85 frame-coordination gains,
196 flavor identities, attachment/scope sets and preterite hosts. Parsing
without the required structure does not discharge an obligation. Keep remaining
body failures after successful label/keyword/lexical work explicit. Every
remaining identity needs a live owner and structural requirement; genuinely
local cases go to `english-v3-corpus-long-tail`. Systemic repairs block cutover.

Corroborated baseline, tree `kkmxslkn`: 134 Constructions, 52 Categories; Type
Lines 32,641 One Reading; rules text 31,027 No / 1,167 One / 447 Multiple,
zero internal failures. The One count includes 356 absent-text/empty documents.
Overlapping observed groups are articles 19,516, slash notation 10,803,
unknown words 8,468, and unconsumed keyword headings 1,783. Group membership is
not proof of sole cause. The initial declaration list contains unsupported
entries; the supported-face filter controls scope. No literal `*/*` occurs in
this rules-text baseline; it is not an established corpus-repair witness.

A focused corroboration on `zumvoowx` reproduced the nine frame cases' 64–736
Readings. Removing only the grouped/ungrouped-small-numeral distinction from
those diagnostic trees leaves 16–92 structures: e.g. Fireslinger 128→32 and
Spicy Oatmeal Pizza 736→92. This diagnostic quotient is not an admission policy
or proof that the remainder is grammatical. The activation audit reported no
confirmed invalidity in its representative sample; classify individual
structures before suppressing them. The intended shared-frame Reading remains
an unmet requirement regardless of parse count.

Reproduce baseline reports using the activation landing's commands and current
supported snapshot; old ignored artifacts are optional evidence, never required
ticket inputs. Focused corroboration uses `cargo xtask english-v3 --data
<subset.json> --output <report.json> --samples-per-face 1024 --workers 24` with
the nine frame witnesses and no reading limit. Synthetic probes separate
article admission (`Draw a card.` versus `Draw the card.`), capitalization
(`draw the card.`), and numeral duplication (`Gain 1 life.` has two Readings,
`Gain 1,000 life.` one, `Gain 1000 life.` none at baseline).

Acceptance follows [V3 sequence and evidence](../../decisions/english-lexical-analysis.md#v3-sequence-and-evidence)
and [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity).
Report complete no/one/multiple census, identity-level gains/losses and their
analyses, invalid-Reading audit, both roundtrip laws, traversal, lexical gaps,
zero internal failures and correlated packed-forest metrics. Deferred rejects
are not Readings. Run the full corpus once on the refreshed final tree after
subset iteration, with complete enumeration and explicit worker count/load.
The baseline's 8.66 s used one worker at load 6.55/5.55/3.54 and mostly failed
early; 35,923 ns/B covers parsed text. It does not establish future throughput.
Measure expanded coverage and justify operational corpus-runtime and forest
bounds for cutover against the 16.26 s advisory; linear extrapolation is only
an estimate. Stamp results with the measured change and applicable lock count.

Keep all grammatical Readings, generated bidirectional declarations and the
Earley/packed admission contract. V2 remains untouched until cutover. STOP and
report rather than use destructive preference, word-named guards, opaque
leaves, eager AST products or a fixed coverage percentage as the handoff test.
Standard constraints apply.

## Progress reconciliation (2026-10-05)

The lexical-measures and article-variants prerequisites are done. Recent work
also supplies lexicalized mana value/cost and face up/down, class-owned type
noun/modifier and negation entries, singular-only you, ordinary binary/Oxford
serial/correlative coordination, selected-head sharing, and corrected ellipsis
and passive/perfect selection. These resolve portions of the implementation
ledger, not this final audit's acceptance conditions.

The coordination-schema landing preserves coverage while reducing 462 named
constructions to 182. The attached construction-feature-economy work further
shares agreement contracts and removes proven constant or unused feature
propagation; it adds no coverage and discharges no missing grammatical host.

The complete current census examines 32,828 supported faces: 26,642 no Reading,
4,246 one, 1,940 multiple; 6,186 covered and 28,799 checked/exact Readings. All
20 named migration witnesses have known spellings, but 19 still have no
whole-face Reading. Absorbing Man and Titania has two, with the auxiliary
Object Gap shape present; an independent complete expected-set regression
remains owed. Known spelling does not prove the needed lexical analysis.

Keep the following obligations open:

- Ordered frame segments: several selected heads sharing one complement do
  not implement one head with several ordered amount/recipient segments.
  Lower Fireslinger ambiguity does not establish that structure. The existing
  frame-coordination ticket owns this repair and depends on keyword-labels.
- Keyword parameters and labels: the keyword-labels ticket remains the owner;
  quality, subject and compound parameter hosts require implementation. The
  historical claim of a flavor-word fallback is not supported by the current
  declarations and cannot discharge the 196-identity obligation.
- Temporal NP adjuncts and preterite relatives; reduced passive postnominals
  retaining an Object; the eleven passive-temporal witnesses; relative and
  comparative-cost host breadth; attachment, scope, Type Line within-group
  ordering and document obligations still require discriminating witnesses.

The old 32,641-face activation snapshot differs from the current snapshot.
Reconcile the 6,396 unclassified, 85 frame and 196 flavor ledgers by identity;
count subtraction is not a cause audit. Do not close this ticket or declare
these residuals long-tail from the present sample. The current DRY pass and
its verification belong to the attached feature-economy ticket.

## Batch records and what they leave open

- Remove source omission (2026-10-07):
  [english-v3-remove-from-complement](../done/english-v3-remove-from-complement.md)
  replaces Remove's unsupported source slot with a selected From-marked NP
  Complement. The bare-Object frame remains owed here: Red Ward attests source
  omission, but admitting the bare frame before the existing From Modifier /
  Adjunct licensing deferrals are discharged introduces wrong source attachments
  beside the selected Reading. Coordinate its admission with those prerequisite
  repairs; do not implement a word-named attachment guard. This is an
  implementation deferral, not a CGEL requirement for an overt source.

- *Remove X from combat* (2026-10-07, from the
  `english-v3-remove-from-complement` review): about 24 attested lines stay
  unread (Observed Stasis; Gustcloak Runner, Cavalier, Savior and Skirmisher;
  Reconnaissance; Illusionist's Gambit; False Orders; Sorrow's Path; Gollum;
  Ydwen Efreet). `turn_part/combat` is a bare interval nominal only inside
  temporal PPs and is not a NounPhrase, so `MarkedRole(From, NounPhrase)`
  cannot take it. Owned here: it needs *combat* admitted as a bare NP
  Complement of *from*, or a Remove frame with a bare interval Complement,
  read from declared features, never a word-named guard.
- Remove passives and pro-forms (2026-10-07, same review): passive
  "is removed from" (Protean Hydra; Chandra, Fire Artisan; Benalish
  Commander; Magma Pummeler; Immard; Jinxed Choker) and the pro-form
  "remove one from it" stay unread. Owned here.
- Unattested measured-amount `ScalarDenotation` exports (2026-10-07, from the
  `english-v3-copular-scalar-location` review): `CardinalMeasuredNominal`,
  `CardinalMeasuredNounPhrase` and `MeasuredNounPhrase` export
  `ScalarDenotation = Yes`, but no gained face uses a measured-amount Subject
  ("Two damage is 3." reads 0 corpus faces). Owed removal under the pruning
  rule, measured.
- *During* Interval overgeneration (2026-10-07, from the
  `english-v3-combat-interval-noun` review): adding the Interval class to
  *During* lets "During turn, draw a card." read (ungrammatical; 0 corpus
  faces), because noun and preposition agree on one shared class with no
  pairwise restriction. Owed: restrict the bare-interval pairing by declared
  feature, measured.

- Instead landing follow-up (2026-10-07): *instead of* with a
  gerund-participial Complement remains unread on **53** supported in-scope
  faces, including Deny the Divine, Reject, Storm Herald and Loxodon Smiter
  ("Exile it instead of putting it into your graveyard."). This ticket owns
  the missing selected gerund-participial Complement beneath *of* inside the
  compound PP; the nominal Complement route is insufficient. The named census
  is retained with the [Instead post-landing fix](../done/english-v3-instead-replacement.md#post-landing-fix-2026-10-07).
- Clitic landing follow-up (2026-10-06): perfect *have* with an Object Gap in
  relative clauses ("each spell you've/you have cast this turn", about **56**
  unread faces) is not read as the perfect. Murmuration's clitic sentence has
  **0** admitted roots and the full form **1**; full enumeration is **0/9**
  Readings, all lacking the perfect auxiliary. The inspected full-form Reading
  is a pre-existing wrong analysis: lexical *have* with a finite Object Gap
  plus a passive depictive Adjunct. Retire it when the perfect analysis lands.
- Clitic surface pruning residue (2026-10-06): the `'m` and `'d` surfaces of
  `BeContracted` and `HaveContracted`, respectively, in `core.ron` have **zero**
  supported-corpus tokens and are owed deletion. The dead-lexeme audit worked
  at Lexeme level; these are pruning-rule residue at surface level.
- Auxiliary stranding exclusion (2026-10-07):
  [english-v3-instead-replacement](../done/english-v3-instead-replacement.md)
  lands the declared auxiliary licence excluding stranded *be*, *have* and
  modals, while preserving do-support and *can't*. This is the user ruling of
  2026-10-06 applied by the orchestrator resolution of 2026-10-07, not a CGEL
  restriction. The named Reading retirement census in that landing resolves
  the former `english-v3-ellipsis-recoverability` maybe-ticket by measurement;
  it is moved to done. Contextual recoverability is not implemented. General
  residual host causes remain here; this exclusion closes the old full-form
  stranding review rather than deferring its wrong Readings again.

- `mana of any color/type that <clause>` (for example, "Add one mana of any
  color that land could produce") is not read: 31 faces, 0 covered. The relative
  clause on *color*/*type* inside the add-mana Nominal is owed here.
- Orchestrator resolution (2026-10-06), fixed-cost phrases: *same … as* remains
  with the existing comparative residual here. Its landing owns the non-scalar
  equality comparative *as* Complement licensed by *same*, not an Adjunct
  licence (CGEL, Ch. 13 §1.1, p. 1101).
- Orchestrator resolution (2026-10-06), fixed-cost phrases: *Activate only as a
  sorcery* remains with the existing only-restriction residual here. Its landing
  widens the position licence for clause-final predicative *as*, measured; the
  fixed-cost landing licenses only the preposed, comma-separated Adjunct.
  **Taken over (2026-10-07)** by `english-v3-focusing-only`, including the
  deferred clause-final predicative-*as* widening.

The landing records of the batches run under this ticket (restricted Oracle
hosts, selected complement coordination and PP licensing, the verified coverage
batch, the keyword-label handoff, participial uses and participial composition)
are in [English v3 residual batch records](../../english-v3-residual-batch-records.md).
They are provenance, not specification. This list of what they leave open was
extracted from them on 2026-10-05; where it and a record disagree, the record
is right and this list needs fixing.

- Selected-preposition licensing deferrals (2026-10-06): prerequisites below
  are owned here; `english-v3-selected-preposition-nominal-licensing` re-applies
  each exclusion after the replacement analysis lands and attribution passes.
- **with-A deferred**: consume Enter's declared
  `Preposition(With), Object, Preposition(On), Complement` frame by reconciling
  `Object`/`FrameComplement`; consume Exile's declared resultative frame and
  declare/consume Return/Put resultative slots where selected. Preserve genuine
  circumstantial With Adjuncts. Gravetiller Wurm, Flycatcher Giraffid and
  Marketback Walker are witnesses. Re-application owner:
  `english-v3-selected-preposition-nominal-licensing`.
- **from-M deferred**: consume Put's legacy source-bearing frames and declare
  consumable From source slots for Exile, Discard, Reveal, Choose and Cast.
  Disposal Mummy, Game Trail and Gix's Caress witness missing source frames;
  Return's optional source/controller frame is now consumable. Re-application
  owner: `english-v3-selected-preposition-nominal-licensing`.
- **from-A deferred**: declare/consume Enter's From source (Phyrexian Dragon
  Engine, Dredging Claw, Triarch Praetorian) and Cast's pronominal-object source
  (Furnace Dragon, Wakening Sun's Avatar); consume adjective Different's From
  valence (Jason Bright, Glowing Prophet). Preserve genuine source Adjuncts.
  Re-application owner: `english-v3-selected-preposition-nominal-licensing`.
- **to-M deferred**: consume Addition's To Complement and fixed `in addition
  to` composition (Don Andres, Neurok Transmuter, Blanket of Night); consume
  Attach's existing Object-To-Object frame in reduced passives and Deal's
  recipient gap under perfect auxiliaries (The Fallen). Re-application owner:
  `english-v3-selected-preposition-nominal-licensing`.
- **in-A deferred**: compose fixed `in addition to` with outer Adjunct scope
  (Indigo Faerie); no selected In frame for Become is implied. Re-application
  owner: `english-v3-selected-preposition-nominal-licensing`.
- **for-M deferred**: compose Block's existing Object NP frame with the
  distributive For-each Adjunct and reduced passive Attach-To (Kemba's Legion),
  without inventing a selected For frame for Block. Re-application owner:
  `english-v3-selected-preposition-nominal-licensing`.
- **until-M deferred**: compose Exile's existing Object NP frame through
  coordinated Objects with same-name comparisons and an outer duration Adjunct
  (Deputy of Detention, Banishment); no selected Until frame is implied.
  Re-application owner: `english-v3-selected-preposition-nominal-licensing`.
- Re-application of the seven deferred Preposition Function Licence exclusions
  (with-A, from-M, from-A, to-M, in-A, for-M, until-M) is owned here: when a
  prerequisite frame or composition lands, re-run the attribution check from the
  done licensing ticket and apply the exclusion only if it loses zero faces.
- Comparative *same ... as*: retiring as-M left "with the same name as that
  permanent" (Deputy of Detention, Banishment) with no NP-internal analysis; the
  remaining Readings misparse *as* as taking a clause. *As* needs a licensed
  comparative Complement of *same*/*such* (CGEL treats *as* here as the complement
  marker of a comparative construction; cite Ch. 13 when the owner consults it). A
  gap, not a regression: no correct Reading existed before.
- Cavalier of Thorns / Genesis Ultimatum destination composition, measured by
  `english-v3-selected-preposition-nominal-licensing`: `among them` has one PP
  Reading but `from among them` and both source-bearing Object NP fragments
  have none. Consume a declared PP argument for From; the current
  PrepositionComplement inventory has only None/NounPhrase. This is the
  inherited PP-complement gap, not a lost face or a verb-frame licence failure.
- Reduced if-able hosts.
- Bound landwalk stems: the suffix keeps its lexical metadata but needs a
  declared compatible stem class before a generic bound-quality consumer can
  admit it.
- 137 of the 196 inherited flavor identities: 112 whose probed ability body has
  no Reading, 25 with a readable body but an unresolved whole-face host.
- Diligent Zookeeper's limiting "to a maximum of 10": a real independent Adjunct
  that needs its own restricted license.
- Frankenstein's Monster's shared numeric counter tail (see the counter-kind
  entry in `docs/tickets/fog.md`).
- Sixteen of the twenty named migration witnesses still have no whole-face
  Reading; the auxiliary Object Gap witness (Absorbing Man and Titania) parses
  but is owed an independent whole expected-set test.
- Rooting Moloch's intended keyword-name modification.
- Return's legacy optional source/controller frame was corrected and made
  consumable by `english-v3-selected-preposition-nominal-licensing`; Put's
  legacy source-bearing forms remain open under the deferrals above.
- The 19 verb frame shapes / 20 assignments the generic frame consumer
  (`english-v3-generic-frame-consumption`, done) originally reported as
  unsupported; see
  the `### Unsupported inventory and owners` table in
  [that ticket](../done/english-v3-generic-frame-consumption.md). The unreconciled
  slot categories on that landing were legacy `Object` role labels,
  `FrameComplement`,
  `ReplacementMarker`, `MassNoun`, `DistributionPhrase`, `GrantedAbility`,
  `VerbPhrase`, `ManaAmount`/`ComparisonDirection`/`ControlledCostAction`, and
  `ResultativeComplement`; each needed typed reconciliation before a consumer
  could
  admit it.
  Landed 2026-10-06: the Gain/Have `Complement(GrantedAbility)` row by
  `english-v3-granted-ability-quotes` (including the quoted Complement and
  keyword/quotation Coordination); the Look row and the two Put rows with
  `Preposition(On), Complement(FrameComplement)` by
  `english-v3-library-position` (selected NP, Locative Complement and manner
  tail, with passive/Object Gap support). Discharged by
  `english-v3-comparative-complements`: Deal's two `ScalarEquality` rows were
  retired and replaced with nominal-internal comparison and the postposed
  comparative frame. Still owned: the
  `ReplacementMarker` slot of Deal's first row by `english-v3-instead-replacement`
  (its `MassNoun`/`DistributionPhrase` slots stay here).
  Taken over 2026-10-07 by wave-2 owners: the Cost row
  (`ManaAmount`/`ComparisonDirection`/`ControlledCostAction`) by
  `english-v3-cost-scalar-complement`; the Remove row's `FrameComplement` slot
  by `english-v3-remove-from-complement` (discharged 2026-10-07 by that
  landing; see the Remove batch record above); the Have
  `Complement(Object), Complement(VerbPhrase)` row by
  `english-v3-causative-have`. The other `FrameComplement`, `Object`-role and
  `ResultativeComplement` rows stay here.
- The exile resultative frame (`[ObjectNounPhrase, Role("ResultativeComplement")]`
  in `plugins_v2/builtin/macros/keyword_actions/exile.ron`) is declared but has no
  consumer, so attested "exile ... face down" sentences (Duplicity, The Foretold
  Soldier, Moonring Mirror, Lobelia, Defender of Bag End) cannot get the
  resultative analysis until one exists. CGEL Ch. 4 §5.3 treats a verb-licensed
  resultative as a Complement, which is why the frame was kept.
- The wider depictive hosts left open by the participial-composition batch.
- Copular scalar location is discharged by `english-v3-copular-scalar-location`
  on `lxupmwno` (covered 19,205): all six pinned faces read. The synthetic
  "The number of cards in your hand is three." has two Readings (its two PP
  attachments); the plural-*are* probe has none. Scalar variable Subjects,
  including Magma Sliver's *X*, remain owned here. *Is equal to* and *is less
  than* are be + comparative AdjP and belong to `english-v3-comparative-complements`.
- Compound/split lexical ambiguity: "untap step" is read both as one compound
  lexeme and as *untap* + *step*, doubling every Reading of Seedborn Muse (the selected-preposition
  landing re-spells its attachment set to two and retires the nominal third). Decide whether declared game-term compounds should
  suppress the split analysis; CGEL Ch. 5 §14.4 (compound nouns) is the authority
  to consult.
- Tooling: `cargo xtask english-v3 probe` does not print Noun Phrase Agreement
  features, so plural concord had to be confirmed behaviourally (plural verb
  parses, singular does not) in the 2026-10-06 reviews. Not a grammar cause; noted
  so an owner can add it.
- Label/body, ordered-destination, only-if/only-during, comparison, Type Line
  ordering and document cases keep their existing owners. Scalar comparison was
  split by construction on 2026-10-06: comparative quantity Determiners
  (*three or more*, *one or more*, *more than one*) by
  `english-v3-comparative-quantity-determiners`; comparative governors with
  *to*/*than* Complements (*equal to*, *less than*, *other than*) by
  `english-v3-comparative-complements`; property values (*with power 2 or
  less*, *with mana value X*) by `english-v3-scalar-property-values`; copular
  scalar location by `english-v3-copular-scalar-location`. Spikeshell
  Harrier's *below 1* sentence is not a comparative and stays here. Only-if/only-during
  is **taken over (2026-10-07)** by `english-v3-focusing-only` (focusing *only* on
  the clause-final Adjunct); its *only once each turn* frequency case by
  `english-v3-once-frequency`.

- Frame-coordination reconciliation (2026-10-06): 42 inherited no-Reading faces
  remain owned here for exact structural cause discrimination: Commune with
  Evil; Strategic Planning; Sunflare Shaman; Drakuseth, Maw of Flames; Beast
  Hunt; Vigean Intuition; Mulch; Assembled Alphas; Discerning Taste; Tracker's
  Instincts; Murmurs from Beyond; Maestros Charm; Confounding Riddle; Chandra,
  Flame's Fury; The Brothers' War; Winding Way; Sultai Soothsayer; Forbidden
  Alchemy; Taigam, Sidisi's Hand; Resentful Revelation; Neonate's Rush;
  Scattered Thoughts; Tamiyo, Collector of Tales; Pieces of the Puzzle; Firja,
  Judge of Valor; Trick Shot; Ancestral Memories; Self-Destruct; Tropical Storm;
  Ransack the Lab; Organ Hoarder; Judgment Bolt; Sparksmith; Burn the Accursed;
  Borborygmos Enraged; Shadow Guildmage; Testament Bearer; Rakshasa's Bargain;
  Psionic Sliver; Bitter Revelation; Kruphix's Insight; Glimpse the Future.
  Durable identities, exact source and measurements:
  `/tmp/frame-coordination-inherited-reconciliation.json`; exhaustive final
  Reading evidence: `/tmp/frame-coordination-inherited-final.json`. Whole-face
  failure alone does not establish a frame-coordination cause.
  Ownership split (2026-10-06, measured on `wlvwtnppyovn`): the 29 faces whose
  first failing unit is a library position (every name above except the 13
  below) are owned by `english-v3-library-position`. Burn the Accursed (only
  failing unit an *instead* sentence) is owned by
  `english-v3-instead-replacement`. Psionic Sliver now reads: its quote landed
  in `english-v3-granted-ability-quotes` (2026-10-06), and its inner ordered
  damage/recipient segments were already supported on that landing's base.
  It no longer remains a residual. The other 11 stay here: Sunflare Shaman,
  Drakuseth, Assembled Alphas, Chandra, The
  Brothers' War, Trick Shot, Self-Destruct, Tropical Storm, Judgment Bolt and
  Sparksmith fail on Deal's ordered amount/recipient segments; Neonate's Rush
  fails on cost reduction (**taken over 2026-10-07** by
  `english-v3-cost-scalar-complement`). The earlier attribution of no `, then` failures
  among these 42 is superseded by the library-position landing's current
  discriminating probes, as reconciled below.

  Library-position reconciliation (2026-10-06, `xltvxkxk`, 15,997 covered):
  20 of its 29 inherited faces already read on the measured base; 21 read
  finally, with Shadow Guildmage the gain. The
  [landing record](../done/english-v3-library-position.md#landing-record)
  lists every face, its Reading count and the next failing cause. Remaining
  From-among sources on Tracker's Instincts, Pieces of the Puzzle and
  Kruphix's Insight stay here, along with Winding Way's bare singular type
  choice. Comma-then reconciliation (2026-10-06): Organ Hoarder is a
  gain in `english-v3-then-sequencing` (0 → 9 full-face Readings). Vigean
  Intuition's and Tamiyo, Collector of Tales' comma-then links now read as
  well, but each face remains unread on its bare *Choose a card type.* /
  *Choose a nonland card name.* unit respectively. Those bare choice units
  are owned here explicitly; the comma links are discharged. This supersedes
  the earlier remaining-failure annotation for these three faces. Discerning
  Taste is discharged by `english-v3-comparative-complements`: its comparison
  and superlative composition now yield 72 whole-face Readings on `kowrwwym`
  (covered 18,646). Library-position parsing
  does not discharge other sharing/scope obligations. Telling Time's retained
  NP-coordination Reading does not establish its intended Complement Cluster;
  that structural obligation remains here.

- Nested single-quote abilities inside a double-quoted granted ability
  (2026-10-06, retained from `english-v3-granted-ability-quotes`): Urza's Saga;
  Nesting Dragon; Reef Worm; Koth of the Hammer; Arlinn Kord; Mu Yanling;
  Liliana of the Dark Realms; Old-Growth Troll; Huatli; Preston Garvey; Harold
  and Bob; Toggo; Teferi's Talent. Arlinn Kord and Huatli refer to their back
  faces Arlinn, Embraced by the Moon and Roar of the Fifth People. These 13
  supported faces have no whole-face Reading because QuotedText has only
  double-quote forms; none is mis-analysed.

- *Combat phase* duplicate analysis (2026-10-07, from the
  `english-v3-lexical-gaps-batch-1` record): Moment of Silence, Stonehorn
  Dignitary, False Peace and Empty City Ruse gained a redundant Reading in that
  landing, the composite *combat* (Premodifier) + *phase* beside the atomic
  `lexeme:turn_part/combatPhase`. Two labels for one constituency is a spurious
  duplicate; retire one route, measured (Reading counts of the four faces
  before/after). `english-v3-combat-interval-noun` must not add a third.
- `BareGenitiveHost` (2026-10-07): the feature on eight nouns in `core.ron`
  stands in for a productive morphological property. The bare genitive is
  obligatory with plural nouns ending in *s*, regular or irregular (CGEL,
  Ch. 18, §4.2, pp. 1595–1596, [35i]), so other *·s'* plurals fail. Generalize from the plural form when a case arises,
  retiring the per-noun feature.
- `Adverb` now exports `DurationUse` (2026-10-07, from the
  `english-v3-as-long-as` landing), so bare "for long" reads although it has
  0 supported-corpus attestations. Harmless reach; a pruning-rule candidate.
- The 738 *as long as* faces still unread after `english-v3-as-long-as` are
  owed to their other blockers, routed here; that landing's record lists
  samples. They are not claimed as covered.

The records also name two cutover blockers that now have their own owners: the
cost of the complete corpus run (english-v3-census-tractability) and the Avacyn
complement-function defect (english-v3-by-complement-functions).

## Architecture probe follow-ups (2026-10-05)

Bounded owners from the exact-span exploration, ordered for implementation:

1. english-v3-generic-frame-consumption: consume ordered typed lexical Frame Slots;
   Amass's declared NP-plus-Amount frame has no consumer despite readable arguments.
2. english-v3-scalar-cardinals: compose “up to” quantities with the shared NP system.
3. english-v3-number-transparent-concord: retain the nominal head while propagating
   quantificational number's obligatory plural Agreement from its Of Complement.
4. english-v3-targeting-projection-duplication: audit the two bare-target routes
   without discarding distinct targeting functions or modifier scopes.

english-v3-attachment-domain-audit (done) records the measured Avacyn
decomposition and hands the passive By function gap to
english-v3-by-complement-functions and the ellipsis question to
english-v3-ellipsis-recoverability. english-v3-scope-preserving-sequence-schemas
(done) closed with a negative result. english-v3-packed-preferred-readings owns
Preference and lazy preferred access without changing complete enumeration;
english-v3-census-tractability owns the cost of the complete corpus run. Speculative flattening or attachment bans
are not accepted repairs. No 80-percent coverage forecast or raw occurrence count
is a promised gain. Preserve cause classification and actual identity reconciliation
here after each implementation. The three demonstrated composition defects are
explicit production-cutover prerequisites; the broader audit remains open.

- Postpositive comparative Determinatives after measured NPs (*at least 10
  life more than your starting life total*) are owned by
  `english-v3-postpositive-comparative-determinatives`, split during the
  adjective-complement landing. CGEL Ch. 5 §14.2(a), p. 445 identifies their
  category and post-head distribution; they must not become Adjective
  homographs. The eight supported witnesses and host gaps are named there.
