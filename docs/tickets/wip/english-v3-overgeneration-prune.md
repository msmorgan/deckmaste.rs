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
