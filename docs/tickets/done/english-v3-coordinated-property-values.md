---
needs: []
---
# Read coordinated and premodified property values: base power and toughness 0/2

## Why

*Base power and toughness 4/4* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *base power* touches **224**
unread faces and is the sole cause on **178** (recon bucket "base power and
toughness"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Enchanted creature has power 0." 1; "Enchanted
creature has power 1 and toughness 2." 1; "Enchanted creature has power and
toughness 0/2." 0; "Enchanted creature has base power 0." 0; "Enchanted
creature has base power and toughness 0/2." 0; "Create a creature token with
power and toughness 2/2." 0; "Each creature with base power 2 gets +1/+1." 0;
"Its base power is 2." 1.

## Goal

The scalar property value NP takes (a) a premodified property Nominal (*base
power 0*) and (b) a coordinated property Nominal whose value is a slash pair
matching the coordinates in order (*power and toughness 0/2*, *base power and
toughness X/X*), in the hosts where *power 2* already reads (Object of *have*,
*with* PP Postmodifier, *becomes a Bird with …*). A slash pair after a single
property noun (*power 0/2*) and a single value after the coordination
(*power and toughness 2*) do not read.

## Analysis

The structure of *power 2* is the project ruling recorded in
[english-v3-scalar-property-values](../done/english-v3-scalar-property-values.md#project-ruling-scalar-property-values)
(2026-10-07): a singular scalar property noun followed by a value forms an NP
with the value as its selected Complement. That ruling and its CGEL notes say
nothing about premodified or coordinated property heads or slash-pair values;
this ticket extends the ruling and the landing records the extension as a
project decision, not a CGEL citation. Reuse the existing `SlashPair` value and
the ordinary Coordination schemas; no parallel property frame.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Reduce in Stature: "Enchanted creature has base power and toughness 0/2."
- Illusory Wrappings: "Enchanted creature has base power and toughness 0/2."
- Godhead of Awe: "Other creatures have base power and toughness 1/1."
  (control "Other creatures have power 1." 1)
- Admiral Brass, Unsinkable: "It has base power and toughness 4/4."
- Glamer Gifter: "Until end of turn, that creature has base power and toughness
  4/4 and gains all creature types."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The slash pair read as a Premodifier or as an Object of
   *have* apart from the property Nominal, or *base* read as a separate NP, is a
   defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.

## Out of scope

- *where X is …* continuations beyond the value (Candlekeep Inspiration):
  existing variable-*where* route; measure only.
- *Switch … power and toughness*, *base power* as a Subject of *be* (already
  reads).

## Landing record

In addition to the standard record: the extension of the scalar-property-value
ruling as recorded; how the slash pair is matched to the coordinates (declared
features); Reading counts of each witness and of the two negatives; before/after
counts stamped with change ids; timings as integer ns and ns/B with host load
and worker count.

### Project ruling: coordinated property values

Landing decision (2026-10-08), extending the
[scalar-property-values project ruling](../done/english-v3-scalar-property-values.md#project-ruling-scalar-property-values).
The value Complement is selected by a property Nominal, including ordinary
Premodification and Coordination. A single property selects a Scalar Value;
an additive Coordination of first and second property positions selects a
Slash Pair whose components correspond to those coordinates in surface order.
Premodification preserves that selection. The pair may therefore follow
*power and toughness* or *base power and toughness*, while a pair following
one property or a scalar following the two coordinates is not admitted.

Property Value Position is declared on each lexical property: first for
*power*, second for *toughness*, single for *mana value*. The ordinary
Coordination schema composes first + second under an additive Coordinator
into the pair position; reversed, repeated, alternative and longer
Coordinations do not produce that position. The existing `SlashPair` retains
its scalar components, including variables, and the existing
`MeasuredAttribute` consumes the Nominal and its selected Property Value.
`MeasurePosition::After` is retired on both the lexicon and grammar sides;
`MeasurePosition::Before` remains the distinct preceding-measure licence.
No parallel property frame or Coordination construction is introduced.
`SlashMeasure` becomes a shared schema for the existing Measure Phrase and
new Property Value projections. The scalar Property Value projection reuses
`ScalarMeasureValue`; `ScalarValue` itself remains scalar-only, including
in the existing copular frames. The property-Complement licence does not
discharge a Nominal's unresolved Determiner Requirement; the bare property
NP requires that requirement to be absent.

This is a project ruling authorized by this ticket's Goal and Analysis.
No CGEL passage is claimed to establish either the following-value
Complement or its ordered slash-pair extension.


### Landing record — 2026-10-08

Standard constraints apply. The before census is measured on `oqrvnvwr` /
covered **22,669**, source-equivalent to claim parent `vllryyok` and claim
`vlnpkroz`: all 1,697 recorded runtime source hashes agree. The finished
feature census is stamped **`owmovnyy` / covered 22,853**. Its input digest
and all 32,828 face identities and source digests match the before census.
Only `declarations.rs` and `core.ron` change in the runtime source inventory.
All figures below use these before/after stamps unless explicitly noted.

#### PROVE — no silent loss and structural laws

Coverage rises **22,669 → 22,853: 184 newly covered faces, zero lost faces**.
All previously covered faces retain their exact Reading count: zero
increases and zero decreases. Total complete Readings rise
**956,021 → 999,360**, with all **43,339** additions on newly covered faces.
`comparison.json` names every changed identity with its before/after count
and `readings_sha256`. No removed Reading or re-coverage debt is hidden.

The 624 previously covered faces using `MeasuredAttribute` are structurally
re-spelled: the lexical noun becomes the Head of a Nominal and the old
Scalar Value gains a Property Value projection. Their fingerprints change;
their Reading counts do not. The scalar projection preserves the complete
old value subtree and lexical identities; the noun projection preserves its
Word Form, Countability and lexical identity. No second property route is
retained. The same 624 identities pass the explicit retention subset before
the full census, and every checked subset agrees with the full run in count
and digest (`subset-full-equivalence.json`).

The full run enumerates and validates all 999,360 Readings with zero issues,
internal failures, duplicate Readings, failed or limited enumerations and
undetermined faces. Byte-exact realization, lexical ownership, construction
traversal and leaf traversal checks pass. Independent construction tests
preserve both roundtrip laws and exact node/leaf identity for numeric and
variable Slash Pairs, their ordered property Nominal, Premodification and
Property Value NP. Independently constructed mismatched values fail both
admission and realization.

Word-named licensing guards added: **zero**. Every new guard reads declared
features. Lexical loading and GrammarEnvironment construction succeed; the
existing environment load-error tests pass. The nine inherited unsupported
frame declarations and unmapped-source inventory are unchanged. V3 does not
emit the legacy coverage lock or permitted-licensing-checker total; the
covered figures here are the complete retained-Reading census.

Assurance: **added 8; re-spelled 3; restored 0; removed 0; new ignores 0**.
The five initial tests fail on the baseline for the missing property
constituents. Existing value assertions remain intact. Re-spelled tests are
`property_values_are_dependents_of_the_property_noun`,
`independently_constructed_property_values_preserve_both_roundtrip_laws`, and
`slash_pairs_preserve_ordered_components_and_explicit_signs`: same surfaces,
values and outcomes, using the replacement Head/projection shapes.

#### DISCLOSE — readings, analyses and scope

The complete census is **8,120 unique / 14,549 multiple / 10,159 unread**
before and **8,124 unique / 14,729 multiple / 9,975 unread** after. There is
no specificity arbitration; all grammatical Readings survive. Construction
Cost overrides are unchanged and were not fitted to corpus results.

`newly-covered-readings.md` and `selected-analysis-audit.json` name **all 184**
newly covered identities, complete Reading counts, preferred Reading ids,
costs and property subtrees. All preferred analyses were checked: **141
paired and 57 scalar property occurrences**, including existing scalar
constituents inside newly covered texts. The value is the selected Complement
of its property Nominal. The pair is neither a Premodifier nor a separate
Object of *have*, and *base* is an Adjective inside that Nominal. Ordinary
Premodification also admits attested *total power*, *total toughness* and
*total mana value*; Chariot of the Sun supplies *base toughness 1*.

Representative newly covered faces and their preferred property analysis:

| Face | Complete Readings | Property Nominal / selected value |
|---|---:|---|
| Reduce in Stature | 2 | Coordination with *base* on the first Conjunct / SlashMeasure |
| Illusory Wrappings | 2 | Coordination with *base* on the first Conjunct / SlashMeasure |
| Godhead of Awe | 2 | Premodified coordinated Nominal / SlashMeasure |
| Admiral Brass, Unsinkable | 80 | Premodified coordinated Nominal / SlashMeasure |
| Glamer Gifter | 2 | Coordination with *base* on the first Conjunct / SlashMeasure |
| Warden of the First Tree | 48 | Premodified coordinated Nominal / SlashMeasure |
| Square Up | 4 | Premodified coordinated Nominal / SlashMeasure |
| Ensoul Artifact | 14 | Premodified coordinated Nominal / SlashMeasure |
| Marsh Flitter | 4 | Coordination with *base* on the first Conjunct / SlashMeasure |
| Wings of Velis Vel | 2 | Coordination with *base* on the first Conjunct / SlashMeasure |
| Werewolf Pack Leader | 8 | Premodified Nominals / scalar *total power* and paired *base power and toughness* |
| Protean Hulk | 76 | Premodified *total mana value* / ScalarMeasureValue |
| Chariot of the Sun | 1 | Premodified *base toughness* / ScalarMeasureValue |
| Owlbear Shepherd | 1 | Premodified *total power* / ScalarMeasureValue |

Each quoted witness unit has **2** Readings, including Admiral Brass's short
*It has …* unit; its whole face has 80. The two required negative Documents,
*Enchanted creature has power 0/2.* and *Enchanted creature has power and
toughness 2.*, each have **0**. The existing *has power 1 and toughness 2*
control retains **1**. At Noun Phrase root, *base power 0* has **1** and
*base power and toughness X/X* has **2** (`probe-counts.json`).

The two *base power and toughness* structures preserve genuine Modifier
scope: *base* can modify the coordinated Nominal or its first Conjunct.
The Slash Pair always corresponds to the first/second property positions
in order. Reversed, alternative, repeated and longer coordinates, a pair
following one property, and a scalar following the pair are rejected.
The *with* PP Postmodifier and *become* host are independently witnessed by
Warden of the First Tree. Existing outer attachment alternatives remain
licensed. Candlekeep Inspiration remains at **0**, measured without widening
its out-of-scope continuation.

The [project ruling above](#project-ruling-coordinated-property-values)
records the scalar-property extension. No CGEL passage is attributed the
following-value or slash-pair decision. `MeasurePosition::After` is retired
from both the grammar domain/table and all three lexical property licences;
the preceding-measure licence remains. There is no unreachable old route.

Deviations and additions:

- Refined `MeasuredAttribute` to take an ordinary Nominal and a shape-matched
  Property Value. Added Property Value Position, propagated it through
  existing Nominal policies, and composed it through the ordinary
  Coordination schema. Added no parallel property frame or named Construction.
- Shared `SlashMeasure` between Measure Phrase and Property Value; reused
  `ScalarMeasureValue` for the scalar projection. `ScalarValue` and the
  existing copular frames remain scalar-only. The old signed-pair test is
  re-spelled against the shared schema.
- The generic Premodification extension also covers attested *total* and
  *base toughness*, without new words, frames or modifier recipes. The full
  census discloses 42 gains beyond the initial target-string subset; all
  were subsequently sampled with an explicit identity selector.
- Added an outer-Determiner rejection test after review exposed the invalid
  iteration Reading *one power 1*. The declared Determiner Requirement now
  prevents a property Complement from licensing a required outer Determiner.
  The refinement changes zero supported target-subset Readings. No invalid
  gain is accepted and no word-named guard is used.
- Added eight tests covering the five witnesses, ordered shape negatives,
  the PP/*become* host, independent values and the Determiner requirement.
  Re-spelled three existing tests without weakening comparisons. Added
  glossary entries for Slash Pair, Property Value and Property Value Position,
  and updated Scalar Property Value and Measure Position. No glossary gap
  remains. No Comprehensive Rules citation changed.

STOPs: none. No ticket/ruling contradiction arose. The first attempted red
build overlapped provisioning's incremental-cache cleanup; it was rerun after
provisioning completed, then produced the five intended test failures.

#### REPORT — declarations, inventories and performance

Declaration inventory: **207 ordinary + 51 shared = 258** before;
**206 ordinary + 52 shared = 258** after. Categories **150 → 151**;
compiled Productions **798 → 800**; nonblank declaration lines
**3,694 → 3,755**. The inherited total already exceeds the ultimate
250-Construction ceiling; this ticket adds no named Construction. The
2,800-line final breadth ceiling remains applicable.

The complete named inventories are in `inventories-before.json` and
`inventories-after.json`: **38,404 spellings**, **1,139 homographs** and
**1,092 case-folded homographs**, all byte-for-byte equal before/after.
The **23 form literals** are unchanged and the form-literal/vocabulary
overlap inventories are empty. No lexical owner or Word Form is added or
retired. `inventory-comparison.json` records their equality.

Performance advisory, with host load as one/five/fifteen minutes:

| Tree / covered | Workers | Host load | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| Before `oqrvnvwr` / 22,669 | 12 | `0.712890625/3.2890625/3.72607421875` | 101,746,289,340 | 421,106 ns/B |
| After `owmovnyy` / 22,853 | 12 | `2.0771484375/5.73046875/6.20068359375` | 117,029,599,723 | 473,013 ns/B |

Both exceed the inherited quiet-host ceiling of **16,260,000,000 ns**.
The after run adds 43,339 validated Readings and runs on a busier host;
these figures do not isolate the change's cost. This is an advisory, never
an admission gate. The full cargo command's elapsed wall time is
**122,649,070,020 ns** (`full-command-timing.json`).

#### Verification and evidence

Exactly one full census was run on the finished feature:

```text
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-coordinated-property-values/after.json
```

Iteration used explicit target-string selectors (454 raw-source matches),
the 624-identity retention manifest, and an explicit 42-identity selector for
the additional gains. There was no iteration on `--all`. The before census
was reused with its 1,697-source equivalence proof rather than re-enumerated.
All subset counts and fingerprints match the full run.

Required commands pass:

```text
cargo test -p deckmaste_english_v3
cargo test -p deckmaste_lexical_source
cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings
cargo clippy -p deckmaste_english_v3 --tests -- -D warnings
```

The final English run passes **443** tests; lexical source passes **57**
(23 unit and 34 integration). All have zero failures and zero ignores.
The final English clippy follows the Determiner fix; the lexical-source
checks remain applicable because its consumed source did not change again.
Changed Rust files pass nightly rustfmt. No xtask tests, Lean checks or
retired derived gate were run. Citation checks report **0 noncompliant**
strings and **0 stale** across 16,311 sites; the piped diff audit selects
zero new citation sites, so no blessing is needed.

Evidence is entirely under the workspace's ignored
`target/english-v3/english-v3-coordinated-property-values/`, retained in the
coordinator's same ignored directory before workspace retirement. No evidence
snapshot, per-task verifier or process artifact is tracked or embedded in a
crate. Refresh verification is appended below before integration.

#### Refresh verification

Kata refresh exits 0 and moves the claim onto `vqxrosrx`. The imported changes
are a separate gerund-preposition claim plus documentation in
`semantics-v2.md`, `lean/CONTRACTS.md` and the completed Lean sweep ticket.
The card-frame ruling concerns Semantics and does not contradict this
independent English property analysis. No grammar, lexicon, plugin definition
or runtime source changes: all **1,697** measured source hashes still agree
(`post-refresh-source-equivalence.json`).

The before stamp remains `oqrvnvwr` / covered 22,669, source-equivalent to
the refreshed parent `vqxrosrx`; the after stamp remains `owmovnyy` / covered
22,853. Trunk-attributed face or Reading-count decreases: **none**. There
are no conflicts or ledger residues to route. Per the binding verification
budget, no post-refresh census, tests or Lean checks are needed for these
documentation-only imports; all measured results are preserved.


### Post-landing fix (2026-10-08)

Follow-up to the post-landing HIGH grammar finding. Orchestrator ruling
(2026-10-08): only ordinary adjectival Premodification and the already
licensed ordered additive Coordination preserve Property Value Position.
Participial and noun Premodification are unattested and remain excluded;
Postmodification, relative and named Nominals do not preserve this licence.
This clarifies the project ruling above and is not a CGEL claim.

`NominalHeadProperties` and `PostmodifiedNominalProperties` now export
`PropertyValuePosition = None`. Ordinary adjectival `PremodifiedNominal`
exports its Head's position only when the Modifier has ordinary Adjective
Origin; a participial-origin Modifier clears it. Unmodified lexical Nouns and
`NominalConcord` retain their existing licensing. Cardinal and targeted
Nominals also reset the position; their Determiner Requirement and Targeting
already prevent their direct use by `MeasuredAttribute`.

The compiler requires exactly one export per feature and does not permit
policy overrides or nested policy uses. Consequently the common exports
formerly inside `NominalHeadProperties` are factored into
`NominalHeadFeatures`, used alongside the reset policy by its former consumers.
Ordinary adjectival Premodification uses those common exports with its own
position composition. No word-named admission guard, lexical identity, Construction, Category,
Production, frame or form literal is added or retired. One declared feature,
Adjective Origin, distinguishes ordinary from participial-origin adjectives.
The glossary now states the ordinary-adjective restriction explicitly.

The first package run exposes an additional route: the declared participial
adjective class derives a distinct Adjective for *tapped*, which survives the
Nominal policy correction. Fix within scope: that class now declares its
Adjective Origin once in `core.ron`; the lexical-source reader preserves
class-declared features on each derived adjective. The existing derivation
unit test is re-spelled to check the complete expected Lexeme including those
features, preserving its morphology, ownership and absence of verbal features.
Ordinary Adjective projection exports the origin; Coordination retains a
participial origin if any Conjunct has it. Nominal Premodification reads that
feature when composing Property Value Position. This is a grammatical origin
distinction, not a guard naming a verb or modifier. No predicate body changes.

The new suite test rejects all nine review probes at Noun Phrase root:
*power with flying 2*; *power of that creature 2*; *power that it has 2*;
*power named Bob 2*; *power among creatures you control 2*;
*base power of that creature and toughness 0/2*;
*power and toughness of it 0/2*; *tapped power 2*; *creature power 2*.
It also rejects *untapped power 2*, *blocked power 2*, *tapped and base power
2*, *base tapped power 2*, and *tapped base power 2*. Royal Assassin's attested
*target tapped creature* stays admitted with its Adjective Phrase.
These are wrong analyses deliberately retired, rather than corpus coverage
losses. Existing authentic scalar and slash-pair assertions and independent
roundtrip comparisons remain intact. The duplicated Reduce in Stature /
Illusory Wrappings loop now checks their identical sentence once, retaining
the same complete constituent assertions and both card witnesses.
Assurance: added **1**, restored **0**, re-spelled **1**, removed **0**, new
ignores **0**; one existing test's duplicate invocation is removed.

#### Reconciliation of the 178 estimate and 184 gains

The historical wave-3 manifest names **178** sole-cause faces in the
surface-pattern bucket *base power and toughness*. Its actual census stamp is
`mlksrxum` / covered **21,486** (the ticket quotes the later `pvmzprsz` claim).
The pattern is *base power*, not a proof that the following Slash Pair was
the only grammatical blocker. The copied original bucket report, identity
manifest, failing units and provenance are in `historical-recon/` under this
follow-up's target evidence; no historical census is rerun.

Face-by-face reconciliation against `oqrvnvwr` / covered **22,669** and
`owmovnyy` / covered **22,853** gives **134 gained + 44 still unread = 178**;
**zero** were already covered at the coordinated-property claim and **zero**
identities are missing. The original **184** gains comprise those **134**,
**8** additional target-string gains, and **42** gains outside the target
selector: **134 + 8 = 142**, then **142 + 42 = 184**. The eight are
Werewolf Pack Leader; Living Conundrum; Crater Elemental; The Antiquities War; Graaz, Unstoppable Juggernaut; Targ Nar, Demon-Fang Gnoll; Doc Ock, Sinister Scientist; Bramblefort Fink.

The 42 additional gains are **27 total power**, **13 total mana value**,
**1 total toughness** (Orysa, Tide Choreographer), and **1 base toughness 1**
(Chariot of the Sun). Scalar *base power N* also belongs to the original
target selector; it is not an additional selector gain. Complete identity
lists and sources are in `gain-groups.json` and
`sole-cause-reconciliation.json`.

All 44 historical sole-cause faces still unread are named here:

Darksteel Mutation; Sovereign Okinec Ahau; Tenth District Hero; Timber Paladin; Honest Work; Aettir and Priwen; Baird, Argivian Recruiter; Eccentric Apprentice; Halfdane; Kutzil, Malamet Exemplar; Candlekeep Inspiration; Goddric, Cloaked Reveler; Mirkwood Meditator; Arni Brokenbrow; Bogardan Dragonheart; Graven Dominator; Opalescence; Figure of Fable; Storvald, Frost Giant Jarl; Inkfathom Witch; Angelic Aberration; Sword of the Squeak; Master of Winds; Creepy Puppeteer; Azure Beastbinder; Katsumasa, the Animator; Vincent's Limit Break; Unruly Krasis; Behind the Mask; Flexible Waterbender; Starfield of Nyx; Wild Shape; Fractalize; Galion, Elvenking's Butler; Trench Gorger; Witness Protection; The Skullspore Nexus; Figure of Destiny; Zur, Eternal Schemer; Brine Hag; Porcelain Gallery; Secret Identity; Sauron, Dino Devotee; Shape Stealer.

All retain their original zero Reading count in the target selector. They
are existing residuals, not removed Readings or regressions. Remaining cause
audit and routing belongs to the live
[systemic-residuals ticket](../planned/english-v3-systemic-residuals.md);
this follow-up does not infer sole grammatical cause from the surface bucket
or widen admission to meet its estimate. Candlekeep Inspiration's continuation
remains explicitly out of scope. Many listed sources include property
comparisons, *each equal to* values, copular *become* values, mixed property
and ability coordination, or separate keyword/cost restrictions; the manifest
records each source and historical failing units for discriminating review.

The **75-row** `coordinated_property_value_position` table is recorded as a
compaction candidate in the planned
[licence-plumbing-economy ticket](../planned/english-v3-licence-plumbing-economy.md#compaction-candidate-2026-10-08).
Its full domain must survive compaction: omitting a default row would reject
ordinary Nominal Coordination. This fix preserves that table.

#### Verification, evidence and refresh

Verification is restricted to the three requested selectors, compared
face by face with the original finished census `owmovnyy` / covered **22,853**:
the **454** raw-source target-string faces, **624** re-spelled retained faces,
and **184** gained faces. No full census is run. Follow-up evidence lives in
`target/english-v3/english-v3-coordprop-followup/` and is retained in the
coordinator before retiring the workspace. The retained and gained manifests form an **808-face** union, exactly all
covered faces containing `MeasuredAttribute` in the original full census
(`selector-scope-proof.json`); no previously admitted property-value face
is omitted. The previous whole-corpus totals are reference measurements,
not a new census of this follow-up.

Deviations and additions beyond the requested policy fix: a common head
feature policy and the Adjective Origin feature with two composition tables
are necessary because policy overrides are unavailable and the first package
run exposes the participial-adjective route. Cardinal and targeted Nominals
explicitly clear the unused licence. Five additional negatives and Royal
Assassin's positive control cover origin propagation. The existing lexical
class derivation test is re-spelled with complete expected values. No test
is removed or weakened. Adjective Origin was the glossary gap; it is now
defined. Declaration lines rise **3,755 → 3,786**, with the inherited final
breadth ceiling unchanged; **258** Construction/schema names remain. The
original named spelling, homograph and form-literal inventories remain
applicable: the class still derives the same three lexical identities and
Word Forms, now with an additional declared grammatical origin.

An intermediate queued package run uses the old reader after the new class
data was written and therefore fails at loading the `features` field. That
stale source/data combination is superseded by a full rebuild of the frozen
finished feature; its test result is not landing evidence. The tests that
exposed the original exclusion defect and every existing test remain active.
Results and refresh provenance follow below.


Measured follow-up stamp: **`rwvlplnl`**, compared with the accepted full
census **`owmovnyy` / covered 22,853**. Runtime differences against that tree
are limited to `declarations.rs`, `core.ron` and the lexical-source class
reader `native.rs`; `runtime-source-comparison.json` names them.

All three selectors pass with **zero count changes, zero Reading digest
changes, zero source digest changes and zero validation issues**. No face or
Reading is lost, added or altered within the selectors. Each enumeration is
complete; there are zero duplicate Readings and internal failures. Every
counted Reading passes declaration admission, lexical ownership, byte-exact
realization, construction traversal and leaf traversal. The package tests
retain independent construction checks for both roundtrip laws.

| Selector | Faces | Unread / unique / multiple | Complete Readings |
|---|---:|---|---:|
| target-subset | 454 | 294 / 3 / 157 | 44414 |
| retained | 624 | 0 / 12 / 612 | 108871 |
| gained | 184 | 0 / 4 / 180 | 43339 |

New Reading removal from covered supported faces: **zero**. The wrong
fragment analyses retired are the named negative probes above; all 14 have
zero Readings. Word-named licensing guards added: **zero**. GrammarEnvironment
loading and its existing error tests pass. No new specificity arbitration or
Construction Cost changes occur; the original selected analyses are retained
byte-for-byte by their fingerprints. No STOP requiring a new ruling arose;
the orchestrator's explicit exclusion resolves the review finding.

Required tests pass: `cargo test -p deckmaste_english_v3` (**444**), and,
because `core.ron` changed, `cargo test -p deckmaste_lexical_source`
(**57**, 23 unit + 34 integration). All have zero failures and zero ignores.
Strict clippy passes:
`cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`.
Changed Rust files pass nightly rustfmt. No xtask tests, Lean checks, full
census, derived gate or new xtask tooling is run. No CR citation changes.

Selector commands (all with `--workers 12 --samples-per-face 0`):
`cargo xtask english-v3 --text-contains 'base power' --text-contains 'power and toughness'`,
`cargo xtask english-v3 --identity-manifest target/english-v3/english-v3-coordinated-property-values/retained-manifest.json`,
and `cargo xtask english-v3 --identity-manifest target/english-v3/english-v3-coordprop-followup/gained-manifest.json`.
Each writes its named JSON report in this follow-up evidence directory;
`commands-complete.json` retains exact arguments, exit codes and integer ns
command timings. `selector-comparison.json` retains every aggregate and the
empty per-face change lists. Original gains, their preferred analyses and
named lexical inventories remain in the original landing evidence directory.

Performance advisory for `rwvlplnl`, using the reference covered count
22,853 for provenance (no new whole-corpus timing or count). Host load is
one/five/fifteen minutes:

| Selector | Workers | Host load | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| target-subset | 12 | `[7.44873046875, 8.55517578125, 7.09619140625]` | 13,783,623,480 | 1,080,651 ns/B |
| retained | 12 | `[6.24072265625, 8.2333984375, 7.013671875]` | 40,591,752,271 | 1,285,639 ns/B |
| gained | 12 | `[15.30517578125, 10.2724609375, 7.75732421875]` | 22,747,021,617 | 1,523,148 ns/B |

These are selector timings, not a new measurement against the full-corpus
quiet-host ceiling of 16,260,000,000 ns. The previous full-corpus advisory
remains in the original record. The first selector command includes the
xtask build; its command wall ns is distinct from the corpus wall ns.


Kata refresh succeeds without conflicts onto **`wvsqvplp`**. It imports the
`uyuykrtn` landing-record hygiene change and a separate gerund-complement
claim. All **1,697** recorded runtime source hashes still agree with the
measured follow-up (`refresh-source-comparison.json`); no grammar, lexicon
or class-reader change is imported. Trunk-attributed face or Reading-count
changes: **none**. No conflict-triggered retest or selector rerun is needed;
no post-refresh census is run. The source and subset results above are
preserved. The feature is closed and reviewed before integration.
