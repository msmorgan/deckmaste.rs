---
needs: []
---
# Prune eight unattested overgenerations

Priority: HIGH. Besides retiring Readings Oracle text does not attest, item g
deletes two Constructions. That lowers the construction count, which stands at
273 named declarations (central-adverb landing record, `qtyomtqqylrm`) against
the ADR's 250 ceiling (`docs/decisions/english-lexical-analysis.md`,
"Construction economy").

## Why

This is a hygiene ticket and adds no Construction. Each item is an
overgeneration, meaning a Reading that parses on a string the supported corpus
does not attest. The items come from the reviews of the signed-numeral,
licence-hygiene, coordinated-subject-agreement, floated-*each* and
central-adverb landings. Expected coverage gain is 0, and zero lost faces are
required. The deliverable is that the negative probes below read 0 while every
attested Reading is kept.

Line numbers refer to `qsupksky`. Probe counts marked "probed" were measured
at minting on the counter-kind-nouns feature tree, one landing before
`qsupksky`. Counts marked "review" come from the review that found the item
and were not re-probed at minting. Probe both kinds before changing anything.

## Items

### a. ASCII hyphen-minus reads as a loyalty sign

`crates/deckmaste_english_v3/src/declarations.rs:1836–1840`:

```
// Oracle document notation, under the signed-numeral-cost project analysis.
construction SignedScalarCost: CostComponent {
    form [value: ScalarComponent];
    require value.ScalarSign = Present;
}
```

`NegativeScalar` (`:3925–3930`) exports `ScalarSign = Present` for both
spellings:

```
form ["-", value: UnsignedScalar];
form ["−", value: UnsignedScalar];
```

`ScalarSign` is `{ Absent, Present }` (`:185`). The loyalty Cost therefore
accepts the ASCII hyphen. Supported Oracle text spells loyalty costs with
U+2212 only: 579 supported lines begin with `−` (audit of
`data/scryfall/oracle-cards.jsonl` at minting). The only hyphen loyalty costs
in the snapshot are on two Vintage `not_legal`, `set_type: funny` cards:
Invasion Specialist (`"-3: …"` inside a granted quote) and Byode, Inverse Sun
(`-X:`). The hyphen form is still needed elsewhere, because P/T modifiers use
it (*gets -3/-0*, *a -1/-1 counter*, thousands of tokens).

Fix through a declared sign-spelling distinction. Two options: split
`ScalarSign` into `Absent | Plus | Hyphen | Minus`, or add a separate
sign-spelling feature that `NegativeScalar` exports per form. Either way,
`SignedScalarCost` requires `Plus | Minus`, and every other `ScalarComponent`
consumer accepts any sign as it does today. Use a per-form export or a table.
Add no new Construction.

- Negative (probed): "-2: Draw a card." 1 → 0.
- Controls: "−2: Draw a card." stays 1; "+1: Draw a card." stays 1;
  attested hyphen P/T modifiers such as "gets -3/-0" keep their digests.

### b. A signed or zero cost combines inside a comma cost list

`Cost` is `form [first: CostComponent, rest: repeat(CostContinuation, "")]`
(`:1819–1821`), and `CostContinuation` is `[", ", component: CostComponent]`
(`:1823–1825`). `SignedScalarCost` (`:1837`) and `ZeroScalarCost`
(`:1842–1844`) are `CostComponent`s, so they combine with other components in
either order. No supported Oracle cost puts a signed or `0` loyalty cost in a
list: 0 matches for `, [−+]N:` / `[−+]N, …:` / `}, 0:` at minting.

Make both loyalty forms whole-Cost forms, i.e. `Cost` Constructions (or `Cost`
rows) rather than `CostComponent`s. This moves two Constructions between
categories and adds none.

- Negatives (probed): "{1}, −2: Draw a card." 1 → 0; "−2, {T}: Draw a
  card." 1 → 0; "{1}, 0: Draw a card." 1 → 0.
- Controls: "0: Draw a card." stays 1; "−2: Draw a card." stays 1.

### c. A second *by* Adjunct stacks on a filled passive host

"Slivers can't be blocked except by Slivers by Walls." reads 4 (probed). All
four Readings keep *except by Slivers* as `MatrixComplementPredicate` over the
Bare Passive and attach *by Walls* as follows:

1. as a `ClausalPreposition` dependent of the Clause;
2. through `PrepositionPredicate` at the `FinitePredicate` row;
3. through `PrepositionPredicate` at the `SecondaryVerbPhrase` row, over the
   *be* Secondary Verb Phrase that contains the passive;
4. through `PrepositionPredicate` at the `SecondaryVerbPhrase` row, directly
   over the filled passive.

To scope the problem, "Slivers can't be blocked by Walls by Slivers." reads 23
(probed) and "Slivers can't be blocked by Walls." reads 9 (probed). The
supported corpus has no two agent *by* phrases on one passive.

Mechanism: `PrepositionPredicate` (schema `:1457–1464`; instance rows
`:3579–3586`) puts the `SecondaryVerbPhrase` row through
`SecondaryPrepositionProjection` (`:949–956`). That policy already exports
`InternalisedComplementPresent = Yes` after a *by* PP: *by* declares
`InternalisedComplementMarker = Yes` (`core.ron:216`), and the table
`internalised_complement_presence` is at `:616–619`. The licence-hygiene
landing made a filled host reject a later matrix Complement this way. Nothing
yet rejects a later *by* Adjunct.

Required fix: the `SecondaryVerbPhrase` row of `PrepositionPredicate` rejects
`modifier.InternalisedComplementMarker = Yes` when
`head.InternalisedComplementPresent = Yes`. Use a declared-feature table, for
example a Selection table over the two features that is `No` only for
`(Yes, Yes)`. This retires Reading 4.

Readings 1–3 attach *by Walls* above the passive, at sites where
`InternalisedComplementPresent` is not projected: the *be* chain's policies
export `No` (`:912`, `:920`), and `FinitePredicate` and `Clause` do not carry
the feature. These are the same higher-attachment Readings that the single-*by*
string "Slivers can't be blocked by Walls." already has. Retire them only if a
declared-feature projection through the auxiliary chain loses zero faces.
Otherwise leave them and record the residual count.

- Negative (probed): "Slivers can't be blocked except by Slivers by Walls."
  4 → at most 3 (Reading 4 retired). Report "Slivers can't be blocked by Walls
  by Slivers." before and after (23 probed).
- Controls: "Slivers can't be blocked except by Slivers." stays 1; "Slivers
  can't be blocked by Walls." stays 9; Shifting Sliver keeps its digest.

### d. A relative clause on a coordinated Nominal agrees plural even under *each*

`SubjectRelativeNominal` (`:3656–3665`) requires
`nominal_relative_agreement(head.NominalAgreementNumber, head.number,
relative.number) = Yes` (table `:561–564`). An additive Nominal coordination
exports `NominalAgreementNumber = Plural` through `NominalConcord`
(`:1153–1155`, table `coordinated_nominal_agreement` `:551–555`). The table
then admits only a plural relative. The singular-selecting Determiner applies
later, at `DeterminedNounPhrase`
(`quantificational_number(…, determiner.DeterminerAgreementNumber)`,
`:2780–2782`; rows `:683–691`, owner ruling 2026-10-08). As a result,
"Each creature and planeswalker that are tapped gains flying." reads (review),
while the singular relative does not. The row came from the
coordinated-subject-agreement landing (`szlklvns`). By default a relative
clause's Subject takes its person and number from its antecedent (CGEL Ch. 5
§18.1(a), [5], p. 500). Under the owner's ruling, the antecedent's number is
singular under a singular-selecting Determiner.

Fix: the relative's agreement follows the NP's resolved number. The relative
must agree singular when the Nominal is determined by a Determiner declaring
`DeterminerAgreementNumber = Singular`, and plural otherwise. Use declared
features only, for example a relative-number feature exported by the Nominal
and checked against the resolved number at `DeterminedNounPhrase`.

- Corpus: no supported face has *each/every X and Y that/who* followed by a
  finite verb (0 matches at minting), so neither form is attested.
- Negative (review): "Each creature and planeswalker that are tapped gains
  flying." → 0.
- Control: "Each creature and planeswalker they control" (2 Readings on the
  agreement landing) stays unchanged. Its relative has an Object gap and no
  agreeing verb.

### e. NP-level *each X and each Y* agrees plural

CGEL Ch. 5 §18.4(a), [28], p. 508 (page-checked at minting): "coordinations of
NPs containing distributive *each* or *every* take singular verbs", e.g.
*Each dog and each cat has/\*have to be registered*. The comment at
`:683–685` already cites this passage for Nominal coordination under a single
Determiner. NP-level coordination still resolves plural through the coordinated
NP agreement path. Supported witness: Spore Cloud, "Each attacking creature and
each blocking creature doesn't untap during its controller's next untap step."
(the only sentence-initial *each X and each Y* Subject with an agreeing verb in
the corpus at minting).

Fix: NP coordination whose coordinates are all headed by a singular-selecting
Determiner (`DeterminerAgreementNumber = Singular`) resolves singular. Export
that Determiner property from the NP and read it in the NP coordination
agreement policy. Do not name *each* or *every*.

- Probe first: Spore Cloud's sentence (expected 0 today if the item is right),
  and "Each creature and each player gains flying." / "Each creature and each
  player gain flying."
- After: the singular verb reads and the plural verb reads 0. Any gain is
  Spore Cloud only and is disclosed as such. Additive NP coordination without
  *each*/*every* keeps plural agreement.

### f. The cardinal *one* route ignores `DeterminerAgreementNumber`

*one* has two routes:

- `vocab:CardinalDeterminative/One` (`core.ron:176`,
  `DeterminerAgreementNumber: Singular`) reaches `DeterminedNounPhrase` and
  resolves singular through `quantificational_number`;
- the Numeral *one* reaches `Cardinal` (`:3742–3746`) →
  `CardinalDeterminer` (`:3850–3854`) → `QuantifiedNounPhrase`
  (`:3874–3883`). That route uses `NounPhraseHeadAgreement` (`:982–990`,
  `number = nominal_agreement_number(head.NominalAgreementNumber,
  head.number)`), so an additive Nominal under *one* resolves plural.

The same word therefore agrees differently depending on the route. Fix: give
`QuantitativeDeterminer` (category `:331`) a declared agreement number,
singular for a singular cardinal, and have `QuantifiedNounPhrase` resolve
number through it the way `DeterminedNounPhrase` does.

- Corpus: *one target creature or planeswalker that …* (Lich's Relic, Feline
  Sovereign, The Theorist) uses alternative coordination, which is unaffected.
- Probe first: "One creature and planeswalker you control gain flying." The
  plural verb should read 0 after through both routes.

### g. Retire the quantified participial and perfect auxiliary predicates

`QuantifiedParticipialAuxiliaryPredicate` (`:2145–2152`) and
`QuantifiedPerfectAuxiliaryPredicate` (`:2153–2160`) have zero attested
faces. In the supported corpus at minting, every auxiliary-plus-floated-*each*
token is copular: 164 × *are each equal*, 1 × *are each 2*. *are/were each*
+ participle and *have/has each* + participle occur 0 times. (*have each
other*, *have each opponent* and *has each activated* are not floated
quantifiers.) Under the ADR pruning rule ("Pruning rule for licensed functions
and attachments"), delete both Constructions if the bounded census loses zero
faces.

Also remove `"AuxiliaryUse": "Yes"` from `core-verb:BeNegative` and
`core-verb:BeContracted` (`core.ron:80–81`). The corpus has 0 *aren't each*,
*isn't each* and *'re each*. Keep it on `core-verb:Be` (`:79`). Its only
consumer is `QuantifiedAuxiliaryHead` (`:2116–2123`, `require
head.AuxiliaryUse = Yes`).

- Negatives (review): "They are each dealt 2 damage." → 0; "They have each
  drawn a card." → 0; probe "They aren't each 2." and "They're each 2." → 0.
- Controls: Angry Mob ("… are each 2.") and Lord of Extinction ("… are each
  equal to …") keep their digests.
- No test names either Construction (grep at minting). Re-spell any test that
  exercises them; delete none.

### h. Make the central Adjunct licence position-specific

*still* and *also* both declare a single `"CentralAdjunctLicence": "Yes"`
(`core.ron:921,928`; feature `declarations.rs:249`). Four sites read it:

- `CentralAdverbHead` (`:462–473`): a pre-verbal form before a lexical verb,
  and a post-copula form after a verb that declares `CentralAdjunctPosition =
  PostAuxiliary` (`core.ron:79–81`);
- `CentralAdverbComplement` (`:1427–1440`): after an auxiliary or modal,
  before its Bare, Participial or Perfect Complement;
- `CentralAdverbImperative` (`:2087–2091`): the imperative.

Supported-corpus attestation at minting (reminder text removed):

| Adverb | after copula *be* | pre-verbal | after modal/auxiliary | imperative |
| --- | ---: | ---: | ---: | ---: |
| *still* | 152 | 1 (Gilded Drake) | 0 | 0 |
| *also* | 5 | ~50 | 1 (Parnesse) | 3 (Dark Dabbling, Finale of Glory, Beast Mode) |

Two of the post-*be* tokens are followed by a participle: *still exiled*
(Dragonhawk, Fate's Tempest) and *also goaded* (Hot Pursuit). Whichever route
reads them today must keep reading them.

Replace the boolean with a declared per-adverb licence set over these
positions, for example `CentralAdjunctLicence { None, Preverbal, PostCopula,
PostAuxiliary, Imperative }` as a set-valued declaration, read with the
set-membership table patterns that the enter-as-copy follow-up added. Each
consumer then requires its own position. *still* declares `PostCopula |
Preverbal`, and *also* declares all four.

- Negatives (review): "You may still cast it." → 0; "Still destroy target
  creature." → 0.
- Controls: the central-adverb witnesses (`central_adverb_position.rs`,
  including Parnesse's "may also copy that spell" at `:170` and the
  imperative *also regenerate* at `:207`) keep their Readings, and Gilded
  Drake's "This ability still resolves …" keeps its digest.
- Docs edit in the same landing: add the per-adverb attestation line above to
  item 4 (Deviations) of the landing record in
  `docs/tickets/done/english-v3-central-adverb-position.md`. That ticket's
  Goal says each adverb's positions are declared lexical properties, and its
  Method 5 calls an unattested position a defect.
- Update the **Central Adjunct Licence** glossary entry
  (`docs/contexts/oracle-english/CONTEXT.md:968`) to say that the licence is
  per position.

## Method

Standard constraints apply. Deltas:

1. Verification is a bounded census of every face the touched Constructions
   and lexemes can reach, plus 300 random covered faces with the seed
   recorded. Compare it with `cargo xtask english-v3 diff` against the claim
   parent on the same manifest. Expected gain is 0, except a possible Spore
   Cloud gain from item e. Zero lost faces are allowed. A covered face's count
   may drop only where an overgeneration named above is retired, and each such
   face is listed by name with the Reading it lost.
2. Probe first: confirm that each negative reads at least 1 before the change,
   write it as a test, and run the tests once at the end.
3. Run tests and clippy on the crates touched (`deckmaste_english_v3`, and
   `deckmaste_lexical_source` because `core.ron` changes). No Lean check,
   because no plugin changes.
4. No guard may name a lexeme, construction, verb, noun, preposition or card.
   Read declared features only. Re-spell tests and never delete them.
5. The landing record is the six items in CLAUDE.md. Additionally, give the
   construction count before and after using the same counting method, and
   state the method.

## Landing record

1. **Census delta** (pre-refresh claim parent `kqqrpppy` → feature `okwzvwqr`): The manifest unions 16,955 supported faces matched by `:| by | and | each | still | also ` with 300 covered controls selected by stock diff using seed 20261008 (147 outside that surface), for 17,102 faces. Covered faces 12,651 → 12,652; total Readings 1,209,660 → 1,209,720. Stock diff reports lost 0, gained 1 (Spore Cloud), decreases 0, increases 0, digest-only 127, unchanged 16,974, source changes 0. Every digest-only face contains the reshaped SignedScalarCost or ZeroScalarCost. Lost faces: none. Conditional refresh recheck on the same manifest (`okwzvwqr` → `mwnqnrox`): covered faces 12,652 → 12,653; Readings 1,209,720 → 1,209,721; gained 1 (Marang River Skeleton, 0 → 1, from the megamorph declaration introduced by trunk `nwwlspuy`), lost/decreases/increases/digest-only/source changes all 0; unchanged 17,101. Every counted Reading passed declaration admission, lexical ownership/context, byte-exact realization and node/word traversal comparison; no limited enumeration or internal failures. The independent-value roundtrip law is exercised by the retained and re-spelled tests.
2. **Witnesses** (`kqqrpppy` → `okwzvwqr`, complete probes): `-2: Draw a card.` 1 → 0; `{1}, −2: Draw a card.`, `−2, {T}: Draw a card.`, `{1}, 0: Draw a card.` each 1 → 0. Unicode-minus, plus and zero controls each remain 1. Double-*by* with *except* 4 → 3; ordinary double-*by* 23 → 19; single-*by* controls remain 1 and 9. Coordinated *each … that are* 2 → 0; Object-gap *each creature and planeswalker they control* remains 2. Spore Cloud's sentence 0 → 6, full face 0 → 60; *Each creature and each player gains* 0 → 1, *gain* 1 → 0. Cardinal *One … gain* 2 → 0. *They are each dealt* and *They have each drawn* each 1 → 0; contracted/negative *be* review negatives already read 0 and were skipped. *You may still cast it* and *Still destroy target creature* each 1 → 0. Full-face controls after (`okwzvwqr`): Shifting Sliver 1; Dragonhawk 336; Hot Pursuit 14; Lord of Extinction 4; Parnesse 75; Dark Dabbling 12; Vastwood Animist 10; Thelonite Druid 4; Verdant Touch 2; Aven Warcraft 32; Might of the Meek 6; Lich’s Relic 3; Feline Sovereign 72. Angry Mob, Gilded Drake, Finale of Glory and Beast Mode remain pre-existing unread full faces (0); no gain is claimed for them. The retained central fragment tests passed. Shifting Sliver, Dragonhawk, Hot Pursuit, Lord of Extinction, Parnesse and Dark Dabbling retain their corpus digests.
3. **Spot checks** (`okwzvwqr`): Only Spore Cloud is newly covered, so the requested ten checks comprise that gain and nine retained faces: Spore Cloud (60), distributive NP coordination with singular *doesn't*; Shifting Sliver (1), filled passive with *except by* Matrix Complement; Ob Nixilis, the Hate-Twisted (1), Unicode-minus whole Cost; Dragonhawk, Fate's Tempest (336), post-*be* *still exiled* Participial Complement; Lord of Extinction (4), copular floated *each equal*; Parnesse, the Subtle Brush (75), post-auxiliary *also copy*; Jace Beleren (1), whole signed loyalty Costs; Vastwood Animist (10), post-copula *still a land*; Thelonite Druid (4), plural post-copula *still lands*; Dark Dabbling (12), imperative *also regenerate*. Each inspected Reading realizes byte-exactly; probe trees are under `target/english-v3/english-v3-overgeneration-prune/spot-*.json`.
4. **Deviations and STOPs** (`okwzvwqr`): Added no Construction or frame. Retired the two quantified participial/perfect auxiliary Constructions; signed and zero loyalty forms moved from CostComponent to whole Cost. Declaration count method: count `construction` and `schema` declaration lines separately in `declarations.rs`, then add: 225 + 57 = 282 (`kqqrpppy`) → 223 + 57 = 280 (`okwzvwqr`); the ticket's 273 used a different count. The declaration reader now accepts per-form exports to distinguish hyphen from Unicode minus without splitting NegativeScalar or changing hyphen P/T trees. New declared Subject Relative Number defers coordinated antecedent agreement until NP resolution; quantitative determiner agreement is forwarded to NP coordination too. Higher *by* attachments remain residual (3 and 19); only the specified filled passive-host route was retired, with no unverified higher-chain projection. BeNegative/BeContracted AuxiliaryUse declarations remain because both review negatives already read 0, following the user's skip ruling. Central licences became position sets; the passive Complement's post-*be* site retains Dragonhawk and Hot Pursuit under the ticket's project classification, without claiming CGEL establishes that classification. Updated the glossary's per-position licence and added Subject Relative Number; no remaining glossary gap. Added the required per-adverb corpus attestation to the central-adverb landing. Baseline STOP: inherited counter-kind evidence was explicitly stale; resolved by read-only `jj run --ignore-changes` on the claim parent and the same bounded manifest, reusing the completed after run. The first parent attempt stopped before analysis on a missing shared Scryfall fixture; linking that fixture allowed the bounded parent run. Its coordinator-derived report stamp was corrected from `mqrlzszkuszn` to the actual `JJ_CHANGE_ID` supplied by `jj run` (`kqqrpppy`); the original report and the correction provenance remain under target, with no measurement data changed. All 300 sampled controls were confirmed covered on that parent. Supported-corpus grep for the ticket’s literal alternative-relative wording attests Lich’s Relic, The True Scriptures and Tempted by the Oriq; the ticket’s other named examples do not attest that literal wording, and no rule depends on those names. Comparison uses stock `cargo xtask english-v3 diff`; no custom comparator. No superseded lexical central licence remains. Refresh incorporated landwalk stem-class derivation (`wtnuopxm`) and face-down/special-actions keyword declarations (`nwwlspuy`); retained all prior feature evidence and performed only the required bounded same-manifest recheck plus the affected English/lexical-source checks. No frames or tests deleted.
5. **Tests** (`okwzvwqr`): `cargo test -p deckmaste_english_v3` passed (585); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed. `cargo test -p deckmaste_lexical_source` passed (26 unit + 35 integration); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. `cargo test -p deckmaste_construction_v3_core` passed (30); `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings` passed. After refresh (`mwnqnrox`), repeated `cargo test -p deckmaste_english_v3` passed (585), `cargo test -p deckmaste_lexical_source` passed (27 unit + 35 integration), and both corresponding clippy commands above passed. The additional lexical-source unit test belongs to trunk `wtnuopxm`; the feature’s test deltas below exclude trunk additions. Construction-v3-core and its dependencies were unchanged by refresh, so its prior checks were retained. An added core test initially expected the wrong missing-export diagnostic; corrected that expectation and re-ran the core suite. Restored 0; re-spelled 5 (per-form undeclared-export rejection, empty central licence, independent signed Cost/traversal, independent zero Cost, malformed/unsigned Cost admission); added 7 (five English integration tests, two reader tests); ignored 0; removed 0. Formatting check passed. No xtask tests or Lean checks.
6. **Timing** (`okwzvwqr`): The pre-refresh bounded census took 145.622 seconds wall time with 12 workers; no full-corpus census was run.
