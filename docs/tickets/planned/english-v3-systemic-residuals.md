---
needs: [english-v3-frame-coordination]
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
  (`english-v3-generic-frame-consumption`, done) still reports as unsupported; see
  the `### Unsupported inventory and owners` table in
  [that ticket](../done/english-v3-generic-frame-consumption.md). The unreconciled
  slot categories are legacy `Object` role labels, `FrameComplement`,
  `ReplacementMarker`, `MassNoun`, `DistributionPhrase`, `GrantedAbility`,
  `VerbPhrase`, `ManaAmount`/`ComparisonDirection`/`ControlledCostAction`, and
  `ResultativeComplement`; each needs typed reconciliation before a consumer can
  admit it.
- The exile resultative frame (`[ObjectNounPhrase, Role("ResultativeComplement")]`
  in `plugins_v2/builtin/macros/keyword_actions/exile.ron`) is declared but has no
  consumer, so attested "exile ... face down" sentences (Duplicity, The Foretold
  Soldier, Moonring Mirror, Lobelia, Defender of Bag End) cannot get the
  resultative analysis until one exists. CGEL Ch. 4 §5.3 treats a verb-licensed
  resultative as a Complement, which is why the frame was kept.
- The wider depictive hosts left open by the participial-composition batch.
- Copular scalar location: "The number of cards in your hand is three." parses in
  neither number on 2026-10-06 (the *becomes* version does). CGEL Ch. 8 §5.4,
  pp. 693-694 treats *be* + NP as scalar location; a copula gap, not a concord one.
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
  ordering and document cases keep their existing owners.

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
