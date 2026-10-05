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


## Restricted Oracle host repairs (2026-10-05)

The next implementation batch adds declared keyword parameter/separator/order
metadata and typed quality, subject, amount-cost, quality-cost, ability,
condition and prototype hosts. Ordinary binary/Oxford coordination schemas are
reused for keyword quality PPs. Keyword payloads keep their declared marker,
number, separator and ownership constraints; unsupported bound landwalk stems
remain explicit rather than accepting arbitrary nominal stems.

Temporal/manner NP adjuncts use declared noun and determiner classes. Bare
end-of-turn PPs are separately licensed; bare count nouns do not become general
Subjects or Objects. Finite preterite relatives use existing finite agreement.
Who keeps its pronoun analysis. Past-participial postnominals use bare-passive
structure, including recipient-first deal with its retained damage Object.
Selected put/look/search/deal frames retain their complete marker signatures.
Card-name catalog entries project to singular NPs without changing catalog
morphology. Equipped, enchanted and fortified bare singular NPs are explicitly
licensed Oracle-register uses; other participial adjectives gain no such rule.
Singular NP genitives preserve the head's number independently of the possessor.

CGEL authority: temporal NPs Ch8 §6.3 p698; this way Ch8 §2.1 p671;
selected bare count uses Ch5 §8.5 pp409–410; relative who Ch12 §3.5.6
pp1056–1057; bare passives Ch14 §9 pp1264–1265; recipient-first passive
Ch16 §10.1.2 pp1432–1433; genitives Ch5 §16.3 pp467–468 and §16.5.1
pp472–473. These analyses inform the grammar; Oracle attestation and declared
lexical licenses restrict its supported distribution.

All 73 CounterKind declarations now supply owned compound nouns from one
recipe, preserving singular/plural head paradigms and the original fixed-term
owners. Numeric compounds use validated measured structure. The counter head
rejects the former syntactic slash-premodifier analysis. Whole counter NPs can
coordinate; the modifier-sharing charge-and-loyalty-counters analysis rejects.
CGEL compound diagnostics: Ch5 §14.4 pp448–451; head plural morphology:
Ch18 §4.1.7 p1594. Ordinary English alternatives do not expand Oracle licenses.

Independent expected-value tests cover actual keyword payloads, temporal and
manner adjuncts, restricted status NPs, Seedborn Muse genitives, selected frames
and retained-object passives, Lightning Bolt/Rift Bolt names, Coalition Relic
counter nouns and Flycatcher Giraffid counter-NP coordination. Original article
fixtures are preserved: typed feature defaults repair absent custom lexical
properties, while invalid authored values still reject.

The first complete intermediate census adds 3,193 covered identities with zero
lost covered identities (6,186 → 9,379). Its 98,778 admitted Readings all
roundtrip and preserve traversal. This is intermediate evidence; the later
selected-frame, name and compound changes require final reconciliation.
Generic PP attachment remains an independent grammatical-correctness audit;
passing roundtrip laws does not discharge it. Reduced if-able hosts, bound
landwalk stems, flavor heads, ordered shared frame segments and the full
identity-level residual classification remain open.


The independent attachment audit confirms a complement-function defect in
Avacyn, Guardian Angel: a recipient to-PP can attach above would/be/dealt as
an adjunct rather than belonging to the selected deal frame. Its intermediate
41,209 Readings are 203² attachment alternatives across two similar abilities,
not protection-keyword ambiguity. NP attachments remain distinct until a
syntactic analysis disproves them; semantic oddness alone is insufficient.
To/into/onto adjunct permissions need coordinated repair with selected-frame
breadth. Diligent Zookeeper's limiting to a maximum of 10 is a real independent
adjunct and must receive its own restricted license. Other destination hosts
currently relying on generic adjunct admission need frame reconciliation before
changing those permissions. Preserve this defect as a cutover blocker; the
roundtrip census does not certify these readings' grammatical correctness.


Catalog nickname attestation now excludes full titles and other complete
card-title mentions: Captain Sisay does not invent Captain, and Nissa's Chosen
does not invent a Nissa Revane self-reference. Genuine independent shortened
and possessive references remain variants of their original owner. The prior
Nissa lexical-source positive fixture is replaced with actual Dina usage;
actual-source negative witnesses and independently supplied values check this
precision repair. This changes lexical spelling availability, not grammar costs.

Corpus diagnostics now test sample cost/identity priority before building the
large pretty-printed sample payload. Root fingerprints reuse the already
validated materialization trace. Every candidate still undergoes complete
admission, byte-exact realization, traversal and duplicate validation; sample
retention does not cap Reading enumeration. A regression verifies late cheaper
samples, deterministic identity ties, rejected expensive payloads and zero limit.


The one-Reading diagnostic examines all 32,828 identities but deliberately does
not establish complete reading counts. It finds 4,397 newly covered identities
and 54 lost identities relative to the verified 6,186-covered baseline. All
54 losses contain indefinite numeric counter compounds: removing the old
syntactic slash analysis exposed absent plus/minus pronunciation onsets in the
new lexical recipe. STOP: these are regressions, not permitted count changes.
The correction must declare compound-stem onset in the macro, preserve both
noun forms and prove independently supplied a +1/+1 and a -1/-1 counter values.
Do not use this capped diagnostic as final coverage or ambiguity evidence.


STOP resolution: explicit compound onset restores 53 affected faces, with all
2,785 retained Readings checked exactly and zero issues. Lightning Serpent's
remaining +1/+0 gap is restored by the native declared compound class: eleven
additional attested numeric stems reuse the same head-paradigm factory, for
84 owned counter compounds total. Signed zero retains its exact Oracle spelling;
validation separates the authored sign from canonical unsigned magnitude.
Independent exact NP tests cover Ebon Praetor, Takklemaggot, Greater Werewolf
and Jabari's Influence. Lightning Serpent now parses as a complete face with
zero issues. No syntactic counter slash-premodifier analysis is reinstated.
Frankenstein's Monster's shared numeric tail remains a separate restricted
coordination obligation; adding its +2/+0 stem does not claim that host solved.

The broad gate's three facts-test failures occurred during concurrent macro
metadata changes: the binary embedded the prior CounterKind signature but read
the new declaration files. A rebuilt facts suite passes all 17 tests. No
fixture was weakened for those failures; the frozen-tree gate must pass anew.


## Selected complement coordination and PP licensing

The PP audit checks 1,366 previously covered marker-bearing faces under a
lexicon-only to/into/onto adjunct-permission correction. Exactly 37 lose all
Readings; all require parallel object+recipient clusters (Arc Trail, Char,
Fireslinger and related witnesses), with no into/onto losses. One reusable
selected-tail host now checks the complete governing verb frame and requires
coordination. Its primitive tail keeps its two NP functions separate; binary
and Oxford-serial forms reuse existing coordination schemas and require equal
markers. The direct uncoordinated host stays separate, avoiding duplicate ASTs.
CGEL Ch15 §4.3 pp1341–1343 identifies right nonce-constituent coordination,
explicitly NP+to-PP sequences under give; this is not verb gapping or ellipsis.

Ordinary to/into/onto PPs no longer license independent VP/Clause adjuncts.
Selected verb complements and nominal PP modifiers remain available; the
unsupported limiting maximum PP remains explicit. Three new independent tests
cover full Arc Trail/Char sentences and Fireslinger's Activated Ability, reject
mixed/wrong frames and the uncoordinated cluster host, preserve Lightning Bolt's
direct selected AST, and reject recipient lifting above dealt/be/would in an
actual Avacyn constituent. The Avacyn census decreases from 41,209 intermediate
Readings to 33,856, all checked/exact; the remaining nominal/agent attachments
require further independent analysis. Reducing this count is not its rationale.

The subsequent one-Reading diagnostic finds 5,063 newly covered identities and
zero lost covered identities relative to the verified baseline. It is not a
complete ambiguity census. The full English suite passes 147 tests across 35
suites, zero ignored. The refreshed unlimited census remains required.


## Verified coverage batch record

Measured tree ovrpzmymkzkpvquyrypvokmxkzrrzlsl (empty child of uvymuyqo),
lock covered count 11,249. Input SHA-256 remains
49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb;
lexical inventory SHA-256 is
d261c3ea60c7b9f32c8b8b8b8972f3a1a15663cf831670e3d1270dd7a31cdd6e.
The final refreshed, complete 24-worker census covers all 32,828 supported
faces: 21,579 no, 5,898 one, 5,351 multiple. It adds 5,063 covered identities
relative to the verified 6,186-covered baseline and loses none. All 110,285
Readings pass declaration admission, lexical ownership/context, byte-exact
realization and construction/word traversal. Zero limited or failed enumerations,
duplicates, internal failures or validation issues. Costs rank samples only;
no admitted Reading is pruned or capped.

Every gained identity, exact count and selected sample fingerprint/cost/
construction list is recorded in /tmp/systemic-hosts-final-reconciliation.json;
the complete census is /tmp/systemic-hosts-final.json. There are 5,661 changed
face reading counts, including 598 previously covered faces. Retirement of the
syntactic numeric-counter analysis, unattested catalog aliases and lifted goal
adjuncts is intentional and independently checked; new selected-frame and
counter-compound structures replace those paths. These count changes are not
proof that all remaining alternative attachments are correct.

Corpus wall time is 235.015 s, with 960,452 ns/B checked-text thread CPU,
24 workers and host load 3.59/3.83/3.64. This exceeds the 16.26 s advisory.
Avacyn, Guardian Angel still has 33,856 Readings; its 0.099 s chart and
150.638 s complete validation show that structural enumeration/validation,
not chart admission, dominates this face. Keep performance and independent
attachment/function audits as cutover blockers; do not conflate a roundtrip
census with a linguistic-correctness audit.

All 20 named migration obligations are reconciled against the current snapshot:
Boldwyr Heavyweights (18), Ashen-Skin Zubera (8), Ballista Wielder (18) and
Absorbing Man and Titania (2) have whole-face Readings; sixteen remain without
one. Independent expected values now witness finite searched, temporal died,
retained-object bare passive dealt and selected destination frames. Independent
whole expected-set coverage remains owed for the auxiliary Object Gap witness;
parsing alone does not discharge its ledger obligation. The named report is
/tmp/systemic-named-register-final.json. Remaining label/body, ordered-destination,
only-if/only-during, comparison, flavor, Type Line ordering and document cases
retain their existing owners and structural obligations. This broad cause audit
is still open and must not be automatically completed by integrating the batch.

Validation: cargo xtask gate --changed --run passes 1,220 tests across 91 suites,
with one existing on-demand live-corpus test ignored. The full English suite
passes 147 tests across 35 suites. Compiler equivalence and independently
supplied-value laws pass. cargo fmt --all --check and cargo xtask cite check
pass; 15,871 citations checked, zero stale. The batch adds seven English test files. The compiler/runtime suite passes
58 tests, covering typed guards, defaults, frame declarations, header bindings
and the shared grammar cache; the new numeric article tests fix an observed
regression. No existing English fixture is removed,
ignored or re-spelled. The one lexical-source Nissa alias fixture is replaced
with attested Dina, with new independent negative/possessive cases recording why.
