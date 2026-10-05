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
