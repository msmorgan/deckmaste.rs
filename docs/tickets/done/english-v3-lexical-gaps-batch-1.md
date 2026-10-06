---
needs: [english-v3-dead-lexeme-audit]
---
# Close six small composition gaps around existing lexemes

## Why

An unread-face reconnaissance on change `wlvwtnppyovn` (32,828 supported
faces, 13,716 covered, 19,112 unread) found six frequent surfaces that never
read. Every word in them already has a Lexical Analysis: the corpus report
lists none of them as unknown words. Each gap is a missing composition for an
existing lexeme or notation, not a missing word. Counts are "faces touched /
faces where it is the only recognised cause"; they are surface-bucket counts,
not gain forecasts.

| Item | Touched / sole | Lexical analysis today | Probe (admitted roots) |
|---|---|---|---|
| *combat* premodifying *damage* | 1,120 / 396 | `lexeme:turn_part/combat` (noun) + `lexeme:CommonNoun/Damage` | "Whenever this creature deals combat damage to a player, draw a card." 0; without *combat*, 1 |
| *defending player* | 245 / 98 | `lexeme:Verb/Defend` GerundParticiple + `lexeme:CommonNoun/Player` | "Whenever this creature attacks, defending player loses 1 life." 0; *target player*, 1 |
| plural (bare) genitive *owners'*, *controllers'* | 205 / 97 | `lexeme:CommonNoun/Owner` Plural + `vocab:Genitive/Sibilant` | "Return all permanents to their owners' hands." 0; *owner's*, 1 |
| hybrid and Phyrexian mana symbols | 203 / 75 | `vocab:FixedCostSymbol/HybridRedGreen`, `…/PhyrexianRed` exist; `{2/U}` has no lexical match | "{R/G}: This creature gains flying until end of turn." 0; `{R}`, 1; "you may pay {R/W}." 0; `{R}`, 1 |
| *at random* | 476 / 63 | `vocab:Preposition/At` + `vocab:Adjective/Random` | "When this creature enters, discard a card at random." 0; without it, 1 |
| colour-negative *nonblack*, *nonred*, … | 119 / 87 (every failing unit contains one) | `vocab:NegativePrefix/Non` + `vocab:ColorWord/*` | "Destroy target nonblack creature." 0 (likewise nonred, nonwhite, nongreen, nonblue); *nonartifact*, *nonland*: 1 |

No covered face contains a non- colour adjective, while 320 covered faces
contain *non-* + a card type. Attested symbol spellings in supported faces:
ten two-colour hybrids (`{W/B}` 47 occurrences … `{W/U}` 18), five Phyrexian
(`{W/P}`, `{U/P}`, `{B/P}`, `{R/P}`, `{G/P}`), and the monocolored hybrids
`{2/U}`, `{2/B}`, `{2/R}`, `{2/G}` (one occurrence each). `{2/W}` is not
attested and must not be added (pruning rule).

## Goal

Each item reads in its attested hosts, through declared features. One item
must not be fixed by widening another.

## Analysis

*Combat* in *combat damage* is a noun used as an attributive modifier. It stays
a noun and forms a composite nominal with *damage*, not a compound noun:
CGEL, Ch. 6, §2.4.1, p. 537, [27]; Ch. 5, §14.4, pp. 448–449. The glossary's
Premodifier entry already records that type nouns license this function by
declaration. *combat* needs the same declared availability, not an Adjective
analysis. *Defending* in *defending player* is a gerund-participial
Premodifier, like *attacking creature* (the Premodifier entry, CGEL, Ch. 6,
§2.4.3, pp. 541–542). Its lexical declaration must record that distribution.
*Owners'* is the bare genitive of a plural noun ending in *s*, written as a
bare apostrophe (CGEL, Ch. 18, §4.2, p. 1595, [35]). It must compose like
*owner's* with plural Number on the possessor. *At random* is a
Preposition + Adjective idiom of the *at first*, *at last* type (CGEL, Ch. 7,
§3.2, p. 626, [23]). It is a PP whose Preposition Function Licence must admit
the clause Adjunct function it has in *discard a card at random*. *Non-* is a
negative prefix (CGEL, Ch. 19, §5.5, p. 1687). It already composes with type
nouns but not with Color Words. The Affix's declared host classes are the
place to fix that.

## Witnesses

- Fell Flagship: "Whenever this Vehicle deals combat damage to a player, that
  player discards a card."
- Odious Witch: "Whenever this creature attacks, defending player loses 1 life
  and you gain 1 life."
- Upheaval: "Return all permanents to their owners' hands."
- Vexing Shusher: "{R/G}: Target spell can't be countered." (Phyrexian:
  Immolating Souleater, "{R/P}: This creature gets +1/+0 until end of turn.")
- Spellgorger Barbarian: "When this creature enters, discard a card at random."
- Doom Blade: "Destroy target nonblack creature."

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Lands after `english-v3-dead-lexeme-audit`, which deletes zero-token
   lexemes; re-run the probes above on the refreshed tree first and drop any
   item that already reads.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/lexical-gaps-1-before.json` on the claim parent,
   stamped with its change id.
3. Land a series of logical commits, one per item. Write each witness as a
   test first, then iterate on `--face-id` selectors for that item, and verify
   on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading, covering every item. A
   negative or wrong analysis that starts parsing (for example *combat* read as
   an Adjective, or *owners'* read as *owners* + stray mark) is a defect, not
   a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Only attested spellings may be added (pruning rule): add `{2/U}`,
   `{2/B}`, `{2/R}`, `{2/G}`, not `{2/W}`.
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Other *non-* compositions that already read.
- Cost-modification sentences ("costs {1} less") and Phyrexian payment rules;
  this ticket only makes the symbols read where a plain symbol reads.

## Landing record

In addition to the standard record: per item, before/after covered and touched
faces stamped with change ids; the declared feature each fix reads; newly
covered faces by item; timings as integer ns and ns/B with host load and
worker count.


### PROVE — complete census and assurance

Completed all six items as separate logical changes: `uqkkxwtt` (combat),
`lxuqzwmp` (defending), `tkyspouq` (genitives), `ynrztrun` (symbols),
`tmrymrmy` (at random), and `zxpzklny` (colour negatives).
Witnesses were written first and observed red, then made green through lexical
features and bidirectional declarations. Iteration used explicit face selectors;
final verification used the complete supported corpus.

Measurement stamps for every census, inventory, timing and table below:
**B** = `uqkkxwttpmownrwnmxumwpoxmuxqyyxw`, covered **13,716**, measured
before that change's lexical amendment; **A** =
`msklolyynsqnlutlxrmqvqzkkulonswm`, covered **14,425**. B's consumed grammar
and lexical data equal the claim parent's: the intervening claim and initial
witness changed no consumed grammar. The exact baseline lexical hash is
`d81c3e20a1cb37af46058a5bda5f1f75022cafc0e5984ae05e802e40d057ad9f`;
A's is `d144b07a9542744dd1ec729cdf3ac162a26995518bc602953fc8ca6c056fcea2`.
Both read input SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
These stamps identify measured trees, independently of subsequent description,
record-only changes, refresh or integration.

`cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output
target/english-v3/lexical-gaps-1-before.json` and the corresponding
`lexical-gaps-1-after.json` run examine the same 32,828 supported identities and
unchanged analyzed inputs. Coverage is 13,716 → 14,425 (**+709**), unread
19,112 → 18,403. Lost covered identities: **empty**. Faces with fewer retained
Readings: **empty**. No retirement or re-coverage obligation arises.
All enumerations complete: 32,828 in both runs; failed, limited, undetermined,
validation issues and internal Reading failures are all zero.
Checked exact Readings: 150,324 → 204,973.

Every counted Reading passes declaration Admission, lexical ownership/context,
byte-exact realization and Construction/Word traversal comparison. The corpus
command validates analyzed rules text (balanced reminder text stripped), not a
claim to parse reminder contents. Independently constructed values in the six
composition tests additionally prove the value-to-text-to-value law, including
plural Number, noun versus adjective identity, verbal Form/frame, apostrophe
variants, symbol order and adjective-complement PP identity. No grammatical
alternatives are discarded; multiple Readings are retained under the current
English lexical-analysis decision.

No added Admission guard names a word, lexical identity, card, Construction or
specific verb/preposition. Review of the production diff finds **zero** forbidden
word-naming guards. Lexical owner declarations and test witnesses name their
subjects, as required; Admission reads declared features. V3 lexical loading and
GrammarEnv construction succeed with zero errors. V3 does not have the retired
`environment.rs` or legacy `coverage` licensing-checker report; its permitted
checker total is **not emitted**, and is not invented here. No legacy coverage
lock is changed; `covered` above is the complete v3 census count, not a claim
about that retired lock's count.

Assurance: **restored 0 / re-spelled 6 / newly ignored 0 / added 7 / removed 0**.
The six new composition tests cover each item, and the lexical-source test
checks negative formation provenance and exact paradigms. Existing value tests
retain all values and outcomes with `GenitiveNounPhrase` replacing the old
singular-only constructor: `spikeshell_active_extent_is_selected_by_its_lexical_frame`,
`independently_constructed_seedborn_muse_relative_retains_internal_pp` (via its
fixture), `authentic_full_genitive_possessor_preserves_internal_structure`,
`maximum_quantity_preserves_finite_agreement_and_targeted_genitives`, and
`targeted_genitives_preserve_the_possessor_and_coordinated_head`.
`independent_mana_values_preserve_symbol_sequence` re-spells its unlicensed
symbol negative from newly licensed `{W/U}` to unattested `{W/U/P}`; complete
positive values for all 19 licensed symbols are independently tested.
No test is deleted or weakened. One inherited xtask ignore remains:
`macros::templates::tests::macro_schema_census_count_matches_21`, whose existing
attribute says it cross-checks the live corpus on demand.

`cargo xtask gate --changed --from wkmskuxy --run --clippy` derives:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The test command passes **698 tests**, zero failures, one inherited ignore,
including all seven Lean gate integration tests. The combined invocation first
exited nonzero at strict lint for a needless Box replacement in a new test.
After replacing its contents in place, the six composition tests pass again
and the exact derived strict Clippy command exits zero. That test also verifies
its negative's child possessor admits before rejecting the incorrect outer
genitive. Production behavior is unchanged by this test-only correction.
Final `cargo fmt --all -- --check` passes. `cargo xtask cite check
--list-noncompliant` reports zero non-compliant strings; `cargo xtask cite
check` checks 16,059 sites with zero stale. The piped full-feature diff audit
reports zero changed Comprehensive Rules citation sites; no bless is needed.

### DISCLOSE — analyses, counts and additions

The following per-item table is stamped B / 13,716 and A / 14,425. Touched
counts use case-insensitive exact surfaces on the same corpus: `combat damage`,
`defending player`, bare plural genitives of the eight declared hosts, the 19
symbol spellings, exact `at random`, and the five joined colour negatives.
These explicit selectors differ from the ticket's broader reconnaissance
buckets (in particular its 476 random-related faces and two-host genitive
forecast); they are not gain forecasts or a change of census denominator.

| Item | Touched B / A | Covered B / A | Newly covered | Declared feature read |
|---|---:|---:|---:|---|
| Combat damage | 1,120 / 1,120 | 0 / 352 | 352 | `NounPremodifier = Yes`; singular noun modifier, mass noun head retained |
| Defending player | 245 / 245 | 0 / 96 | 96 | `BareSingularUse = Yes`, existing `verbal_premodifier` Form/frame table |
| Bare plural genitives | 212 / 212 | 0 / 84 | 84 | `BareGenitiveHost`, plural Number, Clitic `HostEnding = PluralS`, Genitive Function |
| Hybrid/Phyrexian symbols | 203 / 203 | 0 / 82 | 82 | `SymbolUse = Cost`, `ManaSymbolUse = Yes`, opaque notation structure |
| At random | 184 / 184 | 0 / 48 | 48 | shared `AdjectiveComplementClass = Manner`, unframed adjective, preposition Function Licence |
| Colour negatives | 119 / 119 | 0 / 63 | 63 | existing `negative_prefix_join` declared `adjective_classes` hosts |

Sixteen gained faces need two fixes, so the 725 bucket gains have union 709.
Selected-analysis combinations: C 338, D 87, G 82, S 81, R 46, N 59;
C+D 9, G+N 2, C+N 2, C+R 2, C+S 1. Unattributed gains: zero.

Selection census (B / 13,716 → A / 14,425): unique 6,421 → 6,528;
multiple 7,295 → 7,897; undetermined 0 → 0. V3 reports complete Reading
multiplicity, not the retired unique/specificity-resolved selection census.
No specificity winner suppresses a Reading. Construction Cost ranks the
representatives below only for presentation.

Four previously covered identities retain their old Readings and gain six
compositional readings. All ten resulting trees were inspected:

| Identity | Card | B / A Readings | New analysis |
|---|---|---:|---|
| `43af5041-a4bd-4b61-b1a1-bdea916afc59#card` | Moment of Silence | 1 / 2 | singular noun *combat* premodifies *phase/phases* |
| `65368569-68bf-4498-9979-d2935942ca49#card` | Stonehorn Dignitary | 1 / 2 | singular noun *combat* premodifies *phase/phases* |
| `7962db58-dbd9-4b94-8a21-a1625da4c384#card` | False Peace | 1 / 3 | singular noun *combat* premodifies *phase/phases* |
| `b0391ea4-e37d-48c9-91df-79aefe376bcc#card` | Empty City Ruse | 1 / 3 | singular noun *combat* premodifies *phase/phases* |

The atomic `lexeme:turn_part/combatPhase` noun remains. The composite retains
`lexeme:turn_part/combat` as a singular noun Premodifier and
`lexeme:CommonNoun/Phase` with the head's singular/plural Number. False Peace
and Empty City Ruse additionally preserve the two grammatical of-PP attachment
sites (`NounPremodifiedNominal` / `PostmodifiedNominal`). Fewer Construction
applications keep the atomic reading cheaper; both analyses remain available.
This is disclosed additional grammar, not a newly covered face.

Every gained face's representative was checked against its source and lexical
identities, Forms/features, and Construction tree. The complete 709-face
selector also enumerates all its Readings with zero validation issues; one
representative per face is recorded below. These checks do not claim an
independent linguistic audit of every alternative in the whole corpus.
Representative Reading fingerprints are SHA-256 of generated Debug in this
measured tree; they are diagnostic identifiers, not a stable wire format.

Analysis legend for the spotlight and complete gain list:

- **C**: `NounPremodifiedNominal(NounPremodifier(combat: noun, Singular), damage: mass noun)`. *combat* is never an Adjective.
- **D**: `BareParticipialStatusNounPhrase(Defend: GerundParticiple, intransitive frame, player: singular count noun)`; determined occurrences retain the ordinary `ParticipialPremodifier` route.
- **G**: `GenitiveNounPhrase(possessor: Common-case NP with plural host, marker: Genitive/Sibilant, head: Nominal)`; the bare apostrophe is a retained Clitic, not ignored punctuation.
- **S**: `NamedCostSymbol` or `NamedManaSymbol` under the existing cost/mana composition, owning the exact declared hybrid or Phyrexian symbol identity.
- **R**: `AdjectiveComplementPreposition(At: Preposition, Random: unframed Adjective)` used according to At's declared Function Licence; no noun-postmodifier or locative licence is invented.
- **N**: `Adjective(vocab:ColorWord/<colour>/non: Invariant)` generated through the joined negative-prefix recipe, retaining base lexical source/provenance.

Twelve named spotlights (A / 14,425; the full Reading identity is in the gain
list):

| Card | Analysis | Complete retained Readings |
|---|---|---:|
| Fell Flagship | C | 3 |
| Scroll Thief | C | 3 |
| Odious Witch | D | 2 |
| Upheaval | G | 2 |
| Hibernation | G | 2 |
| Vexing Shusher | S | 1 |
| Immolating Souleater | S | 2 |
| Spellgorger Barbarian | R | 3 |
| Hymn to Tourach | R | 2 |
| Doom Blade | N | 1 |
| Crovax, Ascendant Hero | N | 4 |
| Inundate | G+N | 2 |

**Deviations and additions.** Two Constructions are added for genuine
grammatical distinctions: `BareParticipialStatusNounPhrase` keeps a verbal
participial modifier and Oracle-register bare singular NP, whereas the existing
`BareStatusNounPhrase` consumes an Adjective; `AdjectiveComplementPreposition`
keeps an adjective Complement distinct from NP, clause and gerund Complements.
One Construction is renamed/generalized: `SingularGenitiveNounPhrase` →
`GenitiveNounPhrase`; the singular branch is preserved unchanged in outcome.
Bare-genitive declarations additionally cover attested plural hosts *opponents*,
*players*, *cards*, *permanents*, *creatures*, *planeswalkers*, alongside *owners*
and *controllers*. Right-edge host eligibility is propagated through
premodification and cleared for postmodification, relatives, names and
coordination; the test independently rejects inheritance from an earlier head.
Four new native opaque symbol declarations supply the attested monocolored
hybrids; `{2/W}` is absent. Five new derived adjective identities supply the
colour negatives through existing formation machinery; no runtime morphology
code or positive lexical identity is replaced. Seven tests are added as listed
above; none removed. No plugin bodies, v1 grammar, xtask commands/flags, tracked
fixtures or tooling are changed. The baseline was measured on the initial
witness tree with claim-parent consumed sources, rather than editing back to
the claim parent; its exact change id and lexical hash are disclosed above.

Draft new-test negatives were corrected on evidence before landing:
*defending players* is a grammatical bare plural, and the text *their owners
of cards' hands* can have a genitive inside its of-PP. The latter is checked
with an independently constructed wrong outer-genitive value instead.
Existing tests are preserved. No STOP or recorded-ruling contradiction arose.
The missing glossary term **Bare Genitive** is added to the owning Oracle English
glossary with CGEL Ch. 18, §4.2, p. 1595; no Game Model term or CR citation is
introduced or changed.

### REPORT — provenance and performance advisory

All figures here are stamped B / 13,716 → A / 14,425. Ordinary Constructions
179 → 181; shared schemas 44 → 44; combined named declarations 223 → 225;
Categories 135 → 135; static Productions 545 → 547; compiled Productions
611 → 613. Declaration lines 2,814 → 2,869. The ultimate 250-Construction
ceiling still has headroom; the ultimate 2,800-line economy target was already
exceeded and is now exceeded by 69 lines. Consolidation/economy evidence is
routed to live `english-v3-systemic-residuals`, without hiding lines or deleting
Readings to meet a count. These inventories are provenance, not fitted gates.

Performance is an advisory, **not a quiet-host pass**. B: setup 5,894,004,369 ns,
corpus wall 27,260,311,679 ns, checked-text thread CPU **155,475 ns/B**;
12 workers, host load (1/5/15-minute) 2.884765625 / 5.06103515625 / 4.626953125.
B's total cargo-command wall was not captured. A: setup 7,739,506,446 ns,
corpus wall 63,067,080,045 ns, cargo-command wall **73,662,732,383 ns**,
checked-text thread CPU **280,887 ns/B**; 12 workers, host load
3.2275390625 / 2.3076171875 / 2.53857421875. A overlapped dependency-gate
compilation; no causal slowdown ratio is inferred from these loaded runs.
Both corpus walls exceed the 16,260,000,000 ns quiet-host advisory ceiling.
The additional Readings are retained; a quiet-host performance remeasurement
and cutover affordability follow-up are routed to live
`english-v3-systemic-residuals` (which owns the cutover advisory assessment),
with prior `english-v3-census-tractability` measurements as reference only.

Homograph inventory (120 named surfaces, unchanged B → A):

`'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `X`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `instead`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’s`, `∞`.


Form-literal/vocabulary overlap inventory: **empty**, unchanged B → A.
Machine-readable reports, traces, review manifests and logs remain in ignored
`target/english-v3/` and `/tmp/english-v3-lexical-gaps-1/`; none are source data
or gate authority. The following identity/analysis list is the required human
landing disclosure, not an executable evidence fixture.

### Every newly covered identity and representative analysis

All rows are stamped A / 14,425; every listed identity was unread in B / 13,716.
Codes expand to the selected grammatical analysis above. Cost and retained
Reading count describe the representative and complete enumeration respectively.

| Card | Face identity | Analysis | Reading SHA-256 | Cost | Readings |
|---|---|---|---|---:|---:|
| Aberrant | `46b5f12e-9fe5-44f6-b895-ee8c33205521#card` | C | `afd2a6b95e6374885c20129d93355e77044b173ccd529f3e4f4fe74ddbfdf03c` | 140 | 6 |
| Abomination of Gudul | `3d98af5f-7a0b-4a5a-b3e4-f3c9d150c993#card` | C | `89b20c102883adc408aa891555306344c0a845bcba048300794e0f10f8fb2a01` | 59 | 3 |
| Abomination, Terrifying Titan | `72101185-4336-480a-b4d7-f8ceb0007a51#card` | S | `23fd3fc8c1dd1b7065daf961bd41cc196d630825669b33221548f39e51a0fa5f` | 48 | 12 |
| Abyssal Nightstalker | `10733767-1c97-4e2e-b02b-54bf346f6583#card` | D | `ff8f321b55eac544ab02f2706fa39208a98abd999ad7d30e306c0f9afc58960c` | 26 | 1 |
| Academy Raider | `75131d75-0703-44d0-b503-35190be8e66f#card` | C | `da6340d2d6a68ebe02701f8b27edc03ab103d36cc529f45e90f2b4932ac0d354` | 51 | 3 |
| Acquisition Octopus | `234ff22f-2ff1-4a73-a7c5-e9c53557c4c6#card` | C | `df841591a621de955103efdc8c4750223707f6173cf86d9cb2b1d1bb9273b9b6` | 34 | 6 |
| Aether Gale | `b21d6482-1b3a-47e6-98a5-3067f5f3818b#card` | G | `7b964be6df78e3cfefbce235527908501938ff67a30d9a729cf4c1d260caa761` | 20 | 2 |
| Aether Rift | `2577ef8e-d85e-47af-ac34-629f31992751#card` | R | `05118ec07008598a67f47a956eda4844d532bc8e7fe5bc8e333d9e77bb1572f2` | 61 | 9 |
| Aether Tradewinds | `3a0dd318-9523-4740-b0aa-a911032e3851#card` | G | `b75481659199d19eddefe24c1a2f84eb8ba94c8078ca41f5266501a0bef089d0` | 30 | 2 |
| Agate-Blade Assassin | `381a3e8e-71dd-48e4-ab62-53478bde4a14#card` | D | `660951aaa9ba86812753d8ffeed5eca0b91924d4a3bf5b877bd9dae208b41a1d` | 30 | 2 |
| Ajani's Aid | `b3779fbe-7701-433b-afad-bebd6096a3c8#card` | C | `33dd802e9d51cf36ae7346d5b622bbc1dc8fb460da8bef15ba38513ece4140f1` | 95 | 252 |
| Akki Underminer | `8358f65a-7572-430d-b4f0-4fba7a372bd9#card` | C | `93d5a36baa1efabfe469cf258ee3e365a8e5092e93fa8a1bf193b10bdd4278d9` | 34 | 3 |
| Alexi, Zephyr Mage | `3f60de36-ed63-4d08-a012-fc16e91da46d#card` | G | `58c3958f72e342db33bc34cf3bb9ec7da74621fc2752d4c30ac44b15e963bbb8` | 35 | 2 |
| Amok | `2ddbbe65-f928-4b8e-8c4c-a9ce82e2594d#card` | R | `ed6bbeaf7a4ca27a2640401c1a447c5eb6cd1297d2dc0499852b05d4809e2325` | 27 | 1 |
| An-Zerrin Ruins | `f805b100-49b8-4d23-a768-e8de8bb0daa3#card` | G | `2ffab31562f49f13d6bb27a1565df8eef6e47ee78e15ef19ad67c6af697ae2d3` | 48 | 6 |
| Angelsong | `fe7bad80-f853-4af9-82e2-5ccf8038d93b#card` | C | `00cb10be5e6d3b8dcda415c79cecb61da1187b3965c3260d22c87be1c3aa0658` | 30 | 28 |
| Annihilate | `28f55673-7aa0-497e-8434-aa70df1b5e04#card` | N | `64c701047ebc4f56a02cc3a71d1a2f8110a537612734ec0cf8b29ac964790347` | 38 | 1 |
| Ant-Man, Reformed Rogue | `c51c429d-d8d7-40d6-9760-72c26acaed42#card` | C | `00001aad03573b3d49dd3ec8bc6a1444f00fe150c2749abd35d69e6fe9441a69` | 104 | 12288 |
| Apathy | `09ac5adf-f722-417e-b0fc-7ed0d8c9abd6#card` | R | `01ccf15b4a28fde3a5f40d69a443073a249954a5df42fec97fc83792f699ac59` | 79 | 120 |
| Arcbound Slith | `4fa113da-44aa-4c84-8f63-b1cad63da926#card` | C | `4399f2d81f9ab6e6a94b587cfb25aceb87b46ceef4c65a2c4e1cec0d7e422560` | 32 | 3 |
| Archpriest of Shadows | `7b03910a-2ca7-40cb-9c71-1c2cc856a287#card` | C | `f68e066f915d433c319da158a215f73ff545da2ee2f3f644ee0d28aef8c46be0` | 43 | 15 |
| Arcus Acolyte | `628f2deb-519e-4810-a7e5-e31316e9a4e0#card` | S | `2e1b8b80024d115a4fb9ed53f0af89cdfc1cd065e5c681fd6f66e91d5c53edf9` | 40 | 14 |
| Arena of the Ancients | `12584f75-77c1-4b6b-ae9d-1882792b6b94#card` | G | `381bf92d095605791287e5e4638ce31481a4b954887a6eb9b76f9024c1d3e35b` | 43 | 6 |
| Armored Galleon | `637a10e8-4384-49a0-ad78-03da8930811e#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Armored Transport | `0caae56e-5995-4f1b-b735-80580a372707#card` | C | `04ec0da3bebcf1980d862f4aea6b94735134fbc84434f2b1b926ccdb14abcb90` | 34 | 86 |
| Ascendant Evincar | `e32c0de7-132d-4a39-85fb-da0ed1132878#card` | N | `156b4ddb6fe87878dace8790164dc343cc2d38dfeccc9305fcab7dac0f475750` | 41 | 1 |
| Ashling, the Extinguisher | `e89e102f-e3bc-4c16-bb27-09db83cec5ec#card` | C | `131f493c19cbae1df1b68f413cfeff0e0401e403c2208cb4ba1fb76ba5e46483` | 44 | 15 |
| Attentive Skywarden | `896213bb-28a0-45a4-abbd-35af51f1d9aa#card` | C | `5e07712222eaa4a9b80b221dd095c5a7c3fccb2b893e5e4a687f18052d705da4` | 42 | 9 |
| Attrition | `a85ee0cf-39ce-49f2-874a-f6f54ca1775f#card` | N | `b872f769a36613da327f0155261ce36f9d5c75883ab5ad2ffe68ebae63360aa7` | 24 | 1 |
| Auntie's Snitch | `d2964a68-36d9-4b05-8f34-540969b410bf#card` | C | `0775740247be66e59fb939d6d477bb8174032cff63a1883887f59a11928a0faa` | 73 | 36 |
| Azorius Locket | `ca13dda2-fbb0-4b6b-9126-3c8df04dee2b#card` | S | `22abba612fa145b8ec1fe6b04db3195f27c842b85c6e089521f11a5e82de560c` | 47 | 1 |
| Azorius Ploy | `b9b58c3a-5a76-4b6a-884b-74f54f0ca53c#card` | C | `051cbc5bf3d1bde2fbb70ff048e3c7c6388c5f066dfb49cc116b2b2e24acad65` | 51 | 186 |
| Back to Basics | `05c2dec2-d2f7-4036-b91f-4fccba10a8bb#card` | G | `08b139c0964138df9725ece611af3009c94d51c7c32084f512bd731ac2cdc93b` | 23 | 6 |
| Balduvian Horde | `fd2a94d3-901d-4b46-aa9b-a30ed6dea6bd#card` | R | `04dad4710c942404528c5b3f3cfb431a6d6b8350ad50f96b2b648bdd3b05b3b4` | 28 | 9 |
| Bane of Bala Ged | `38839894-1706-4e11-8310-5ea8dd8866d9#card` | D | `65246e04090164ff5094f2d7a417b03ab3326dd3df029126d60e7f7799fa3b15` | 27 | 1 |
| Banshee of the Dread Choir | `282b4dbf-ec00-48ba-8010-73bee489fa7a#card` | C | `67351edcca1f4c3d0af2f0baf2b78e477e02a30e9f3319ff3371a5e42df0812b` | 32 | 3 |
| Banshee's Blade | `81a770e9-adea-4799-9fed-f10e97e7e901#card` | C | `001ebedbfec7ebaad02e3c72ba763b4f78b297e84aac756dc80b9ab871d561b3` | 57 | 5 |
| Basilica Stalker | `e335485a-80fc-40d4-8be6-7eabf2bab571#card` | C | `5087466c7cf638d9d207d578590f2e2db9b1caf7618982b93d1d8a543c55dd2d` | 41 | 9 |
| Battle Frenzy | `0fca46ec-ec1a-4e6c-bb0e-cdb834b38c92#card` | N | `02e1887bd7c549dc75de6b7b8152aa4fc42872449f92c020515d9faaf1c737ac` | 56 | 16 |
| Beacon Hawk | `edf7c98a-c2ed-4ba2-ac08-e83db0d968cb#card` | C | `8104f7ca0ed47f0ce78026086ffa8d3e64814709e95196e8d978bc1463913cdf` | 59 | 6 |
| Beamtown Beatstick | `035e33ab-adfe-4831-bdbc-52f4ddc7ffe8#card` | C | `0167c9d6b9e400addfd59cc1ae0773a42b624f0bdc8045f73ca4ddc045196c07` | 54 | 3 |
| Bearer of Overwhelming Truths | `69ca69c7-403b-4b5d-be08-d9dce8ce410e#face:1` | C | `2eaaebe2e6f8ed84ba0cc2eef281914b3cc661c8682fa057fddb54283db159a4` | 26 | 3 |
| Befoul | `c573c59a-5a79-4fe1-a6d8-5e0153b5c059#card` | N | `34dfcde28162320a76a3601259adbbd9f05f66bbd8ff27d6b1b186e557365791` | 29 | 1 |
| Bejeweled Warg | `051ff7e0-dd00-4467-8796-a5d1c21934ed#card` | C | `0d6cdb099b7c1291a386532bfbc377aa2069cf90c73b9716375c134d2b7e7b4d` | 58 | 3 |
| Belligerent Guest | `417e798d-9ca6-4cd0-adb4-f61d3becb201#card` | C | `781f8d332f064713ad99ea3b641e8771983452c8526b3dc894dbfabcdc8ccf62` | 31 | 3 |
| Black Cat | `10edf7e0-d6be-4510-819f-8834bedfba41#card` | R | `15f0e2376fb309377e13320c4010473de17c0efc82d7bffdea89c1551fc140dd` | 23 | 3 |
| Black Panther, Hope Enduring | `fe00278d-4385-4e3f-919c-12a794fd86f5#face:1` | C | `0b9635bb31bd879b56192c28fa913b7f9818c1f4445627341b6e8ebe469ef738` | 55 | 162 |
| Black Widow, Deadly Hunter | `12a7bf3c-5194-48ad-be0b-553b8ad67671#card` | C | `75d9f2ae808ac28131f9e215a7f1a315f76ec6116145a1b2080f1cd298b5bfb2` | 44 | 18 |
| Bladegriff Prototype | `9fff90b6-98ef-4f78-b08d-7e46dfc03403#card` | C | `068eb14feda2de383b93e8c1890449d557fdde357c7e813b6a4bc86384464f4b` | 48 | 42 |
| Bladewing, Deathless Tyrant | `cc4bc5d7-ed37-4a80-a5e7-eb68c255cbef#card` | C | `07bc757700cb38f0388d33fe2a58b346e0b59f65b9a1531edf385f46f52c4ca0` | 62 | 120 |
| Blazing Specter | `4c046368-5899-42ef-9972-3a3a6c8fe3fd#card` | C | `c015e3181e91844ce89937c040cab5d6d16283d5276da14c08f04cfe7e60dd64` | 34 | 3 |
| Blessed Ghoul | `fe99d90e-4809-4310-a3f7-ceb540511d55#card` | S | `0c089b2ba9c4cb561f3d1a09cee0eb38d42a2fdd355fd504feac2cc0d49d5ec6` | 25 | 3 |
| Blessed Respite | `8c888122-9d05-4641-931c-83f771ba9d09#card` | C | `3eeca0974dd9188e57c35f3956b457509e2c796ecc92149ef9179b554bd78763` | 40 | 28 |
| Blind Zealot | `30c7e304-ecfe-44c1-88d1-f507df63c080#card` | C | `1e8506e541f2d06e3b6b8ad25645075d38a1ea1b088f90e96a6bae7d470885a1` | 56 | 3 |
| Blinding Angel | `cb479ce1-59e8-4531-9a21-f7c97d334817#card` | C | `1bb53da827d5a7e5b0015781b60200ab83df111c993dbbf98bd6335987ffb2ff` | 34 | 6 |
| Blinding Light | `6b315dc3-c330-4b30-b6ad-4da12ccf6ca3#card` | N | `2c6829d4357f9f9e2a062b5cf75bc01defe00214244b7c5c196f15c476c557d0` | 13 | 1 |
| Blinding Souleater | `f5f330a5-14a0-4bfe-ba46-4616692b7bd3#card` | S | `f9fa91d1226581b09409a88f1438ebfc07c09accea481e8646a7efece4f51bf7` | 19 | 1 |
| Blistering Dieflyn | `4cd6ad78-7441-4dd7-b486-d9575e8e0947#card` | S | `222d680816aa49ca516271d1eed78b798231fa25fa705abdd76cf045db24fc95` | 28 | 2 |
| Blizzard Specter | `b6f4148d-8511-46b3-93ec-2f5eb1e4ffad#card` | C | `6d94f8bbdc3a39690039141e36aac4b9dbca49136a86ff3a1e8ba5534bdba5d6` | 64 | 6 |
| Bloated Contaminator | `090018e0-4dcb-4b3c-b4e0-7ba62de0484d#card` | C | `c9cd4fd256272c636f7f73807cc611d73618f7838be6e23b6a4c2ad6408f1ce6` | 30 | 3 |
| Bloodmad Vampire | `c8a20fc7-025b-403a-b893-ff5efcbdc9d8#card` | C | `d243ae90d7f9c720bebca2e3c0b089a30820c965167ad96786e3f757999072b8` | 34 | 3 |
| Bloodvial Purveyor | `7292272d-6511-466b-aadd-c4fe7ed8df09#card` | D | `16bd41d583c91eabb67ec67884757d3895c3ae0887aa4e6b1a294e0b8cbbd4b8` | 71 | 12 |
| Blunt the Assault | `dccd0533-fda0-41ff-b540-c4f1bb32fc95#card` | C | `019f94d8c4d85ea8729c3d83a800ec6429855818e524bb38067b3ec14cc986ac` | 46 | 140 |
| Bog Serpent | `9a9877b5-9f75-4c83-b11b-f006aecb075b#card` | D | `17df3c3726e6151baba0f4c09491c83f8bf2d1e3cdcf56bae346656da5025dd4` | 44 | 3 |
| Bomat Courier | `a88f3881-4f43-4473-965f-f5c24ae5752f#card` | G | `09b8b8bbbd8ad32477cf269062bc704a193d230ffd0ca1e21f9e3dd7a3fc9da4` | 71 | 12 |
| Bone Dancer | `13b5c9ef-d0e3-453d-ae5e-8de4f310cc38#card` | C+D | `0046624704e6fa4fec408805aa3f5424d9aadb24daa92ed0c14cb343d77dcb9d` | 72 | 30 |
| Borborygmos | `9ab0820b-4f5f-47b3-8fa9-43f802aed275#card` | C | `e5dd2153c135fa726aff75863dfc809c9da705b51809d50cb8d2b9f2243e01f2` | 37 | 3 |
| Boros Locket | `b044b4fd-d504-4a11-80bf-eb8fb0b6e21e#card` | S | `a04db80fe042fc7385a67290be0055cd378cba90bea966314d54deebcf8a66d1` | 47 | 1 |
| Bottomless Pit | `91e6fb47-59e4-4616-b8dd-3a7e30070074#card` | R | `04a43dee58f9d05d883d6964c7ca5672414396c8f57b55514d21ff288a5b8b29` | 28 | 6 |
| Bounty Hunter | `2fd1cfc5-fbcd-4d31-ba8b-759d8c41b435#card` | N | `40b8addfbf8fed82c339b285aac3097f901cfd953de3b079a17aa93e5e6edcc0` | 44 | 9 |
| Branded Brawlers | `e975a8b3-f817-42f3-84da-b03ebf95cf25#card` | D | `104ef0f14b8cda08224a295869b853960bbb936097fe959eb3b495cbf194ac36` | 51 | 9 |
| Brawn, Amadeus Cho | `19910a90-d334-4d99-89f2-cd81cea3c824#card` | S | `02d4160fa80c219fd297f28cfd9975e8793e4f3ebe7d31f390a094aec04d8367` | 51 | 40 |
| Breaching Leviathan | `7d249b16-33ef-463e-9e5d-0b84d434f093#card` | G+N | `089736808bda5e924d6c36e531814578747cfdbbe6191a810de7cb34a9de907c` | 54 | 12 |
| Bred for the Hunt | `fc7cda6f-7e5e-4a56-8d67-ffdea7edf269#card` | C | `1cd96f641c4da294f19e77e471ed65d1846a34d80b0958f07f536b32f4d80f1f` | 45 | 15 |
| Brood Sliver | `6b1d178f-9713-4dcf-920b-ddf2c94cc427#card` | C | `e6504d1c0bbe447f4963c29e77a3e900dd8f2893fbb573b422d199269faca86d` | 44 | 3 |
| Broodbirth Viper | `a8455828-9c61-4ef9-a5f1-1f0cc6fbf446#card` | C | `c6bc98def7fd47506f712261ea30dc27a0fb8028ebae5b505bdbac3e8612df6a` | 34 | 3 |
| Cabal Executioner | `b7817579-df4d-41db-bb90-2aabac58f299#card` | C | `202c865bc14f9a93f1c4188154ede2316ccc6106ca3b965eabf1ee1371363448` | 41 | 3 |
| Cabal Slaver | `22a189a9-e755-4cbb-b419-5ba2e5acbe1b#card` | C | `309b422a0c83a6ccfa1eb79bda0a02001f58c5ece47de300b86fa245253a7cc9` | 29 | 3 |
| Cabaretti Initiate | `5e0d877c-6ce3-462b-9746-7fc921c93be9#card` | S | `154cdd0b790c49efd41110f5ebb3ba3a0667f5c18273c9291d57b21ed4b8d846` | 22 | 2 |
| Cadira, Caller of the Small | `fcf22321-2f55-41ca-b8e1-4792540ba3ee#card` | C | `e4c60d26f9fd11421210a9931f3be7731622d6572426e1885880e48361c66b98` | 51 | 3 |
| Call to Serve | `bb4be6b2-507c-4d7c-bea4-010670dce3da#card` | N | `1186bea45c7fda0fa57a49075f4a83ddd005369ea28dd5c881a9f990d5b9802f` | 43 | 5 |
| Campaign of Vengeance | `627e9668-5e1c-4ab6-83f4-612a42d0afce#card` | D | `8a4508cbae207d2a876ffac96f6602a0965af05ae30e1c312b3810afad155cc4` | 35 | 2 |
| Canyon Drake | `b5b46c99-1cc1-465a-a831-3fd6665ef560#card` | R | `59d4c96cc26b03e8fc0db48e6bddbe4f9d018a040eb28e0dc971eedf6394510c` | 37 | 2 |
| Capricious Efreet | `9abd2286-23e9-49cd-be53-39423890f35c#card` | R | `267949f1b583b34ad799b3bfebbb5c3cabf810479d771511f6a21ca8804fd1f1` | 57 | 12 |
| Captain America's Shield | `8255a797-ddd3-4db0-a3f5-dd8f38c3b908#card` | D | `0ef0c52600cc8f68d80b08ac3d34fb2f6a9e5504bdb363cfb3c3ef94d563129c` | 51 | 1 |
| Captivating Gyre | `41011f09-ad8a-4eea-aa52-d20a219bc94c#card` | G | `25033944f7ad1d644e3ba928cfa3404641b1fb461640922b3a31a67187d7e438` | 20 | 2 |
| Carrion Rats | `1aa6b31b-cd30-4741-abc6-2ef72e445630#card` | C | `1b0b5e9a827216e17bbe299d008345f75bc1de4735af4628bbb3776d7d3ce175` | 57 | 5 |
| Carrion Wurm | `ed0cf504-c365-4486-92bc-c329b83b99d0#card` | C | `72f7cbe86cef60c337cc96e7804219d35a2541d7b35e8d9816674717d9fa7811` | 58 | 5 |
| Cascade Bluffs | `f1603384-4361-49c9-98aa-7785fc3504c4#card` | S | `130fdb2894707fb59849c51503d84d23c47ec34477bdd47b94c8228dd1312ddc` | 41 | 1 |
| Casey Jones, Vigilante | `bc163b5a-0b1a-4257-8c07-5c7d5284015f#card` | R | `0278936091627b3fe41da36622e4eac62fc21638ffd2f11440e3f6e386cffcd2` | 43 | 12 |
| Caustic Wasps | `fea997a4-3ed1-4f83-b16a-56939f4b2211#card` | C | `26948ee8869e67188c6fcd5e1ae6d644510089bd8c4767dadb5fc3f740682302` | 40 | 3 |
| Chilling Apparition | `13cfc92c-142a-4016-b86f-ace6bda1a69c#card` | C | `e2112b41d689247627deb8f924701faf6ea168ef9882df3f95dd56d24fde16bb` | 44 | 3 |
| Chime of Night | `09b41783-0583-4486-99e8-3f8a11ae37fd#card` | N | `1119302ed2e2e0cec20779186fb10eaeb27f04b29331cb02615342f0f87f73a9` | 35 | 3 |
| Choke | `057fa60b-10b0-4612-be0d-157076c82241#card` | G | `18c6394a103e50e27025573178c0b1f7f588bc9e055e28a3acc0262539495093` | 21 | 6 |
| Churning Eddy | `774e53b2-5aec-43e1-b4fe-8812b8430401#card` | G | `ae5f61bda43cb081f9dd3f040120312b421ad3ba488a2b02a1236d0679bbfdc8` | 19 | 2 |
| Cinderheart Giant | `13efba9f-aedd-4ec0-a387-60c09b887341#card` | R | `0ef6ccf0d7ce326861625a97647e7dd4c70ff30a9c239bde151b5931d1d75d1c` | 36 | 4 |
| Cindering Cutthroat | `46e8bec0-1a09-4e6e-b78a-7ba161686ca9#card` | S | `0c351d3fc4ee9c5a8ef52f7f20146f3dd56890c51d50b1521130d26ec5d2def9` | 55 | 18 |
| Coastal Breach | `e42b6bd7-8fdf-4452-85cc-9aed790ff682#card` | G | `0bd20aa608766e23e4c415f86af1f0ef8220ccc6b103ae478f485cc32c93b17c` | 21 | 2 |
| Coastal Piracy | `8a05ec32-7b0c-4f23-a4f7-413301c2a70a#card` | C | `e25042f1f48159743c021bdb11a265fffe5540abab221087eb35ba88631159ed` | 36 | 3 |
| Coastline Marauders | `c93bef6e-b676-43df-9f0f-026aaff25604#card` | D | `207e2e915747c0fe00f21bcd87c863ebdca62641131f2d1eee29c937de1d72ee` | 48 | 6 |
| Coiling Stalker | `54dc1c02-2d71-465b-a285-d6d11113991c#card` | C | `0da5d84d72de457cff3310c5bf038d4481dce8f91f9e900743d55f0603c27517` | 53 | 30 |
| Commencement of Festivities | `56ea8f6b-a765-44e7-bf0e-4d4a2405684b#card` | C | `011698becb83663913a0ac964f51d4205c199cef6ca92a3ac51838283ac2c1cd` | 28 | 31 |
| Connecting the Dots | `a29e2c70-8909-465b-9cec-1a7037250313#card` | G | `190ea33691ebc557dbe290c97109f76494aec345b90d71f17a43d522bb26070a` | 74 | 12 |
| Contested War Zone | `ed73de2b-d7f4-48d9-9be2-aa9d111b7aa7#card` | C | `ac5899a986c6d8bb12e2b6759f1902a9aa39f94715f0f88d43317c2d7959e8d9` | 80 | 6 |
| Coral Helm | `aa2970c8-f2ea-4e06-8b8f-ec89af0012a0#card` | R | `6c63c5f5f83eda3b33ca91bd8ff27b38f2d749bbdd10d0247e085b792a9e54c1` | 34 | 2 |
| Corsairs of Umbar | `edc74fb9-a368-4f73-8147-f318a32a3d06#card` | C | `52e5584c0c90afd863332f3b5ddb316b6088f3ea478fa83cc91389c94d427fe4` | 57 | 9 |
| Counterintelligence | `0c4e64d1-cdcf-4e96-8bf4-ed1414f6ad62#card` | G | `ca47bcc57a1978fc08780aab95b69080ae56008c092679bdad9c9075202531b4` | 20 | 2 |
| Coveted Peacock | `e6ad5e92-c1ab-4c91-95ca-1af295e71b23#card` | D | `2e88c0e8abcaef455b67b5bb5c1f6f6336590db99319b27edc370d92bef8d2c6` | 32 | 1 |
| Crackleburr | `59a6cb4e-1646-42b5-b482-da641c0f7f6f#card` | S | `018d6c100d4e8c4224c7bb9b48659fbe5dd342250d05575d913c96e9bf7a0c64` | 85 | 18 |
| Cradle to Grave | `a86f9b53-ff44-4b08-a8e9-beef73595de1#card` | N | `229a21f00e2105299fd64e6647b0bfc0f080808e95966a65e4a4898d7f592cf4` | 19 | 6 |
| Creakwood Ghoul | `94c47804-bb64-4a91-85c5-9241c39685aa#card` | S | `089daf1744fd5b49ded027e926e2f92e601a6504bdeee5561769388e1fffd160` | 32 | 3 |
| Crimson Caravaneer | `ab3360e5-6a60-4806-bbc2-b7b26c119856#card` | C | `e8a3106025bfa051fadce0b7bd33ec95d329ca71b9d64b11cab90479e4a47471` | 33 | 3 |
| Crosis's Charm | `e59d70a2-40ac-45b0-995d-65b9caa290c8#card` | N | `1b9230ce74ea2f64d6b8b9ca00082e6bf8ce0b409b2e136cfce01399cee7f4ca` | 62 | 2 |
| Crovax, Ascendant Hero | `0cbf6ad8-9bf1-455d-bd74-9c142e17acb0#card` | N | `86bbdea6d6557de0d94019ff9877b35853327d93596107d2f38a208e12c58ba2` | 61 | 4 |
| Curiosity Crafter | `0be907b2-54f9-4c8c-b6af-7c0093892e42#card` | C | `0e3cab6b574eafbd6b8cea32982613195911871f568d9c940f3fe3dbcbf252d0` | 51 | 6 |
| Curious Altisaur | `80686908-abb5-4728-a5f6-71baca27f467#card` | C | `bb06a7b5f4e0848809d338b28501fcdd067feff7c7af68f8e5d99e388a4efda7` | 36 | 3 |
| Curse of Marit Lage | `6fde4790-4186-4786-aa1a-b6a53bfadc1c#card` | G | `88279a84db94cb8cdfb56f8dde1b37308607074f162f58bf32686f26cc5c1021` | 39 | 6 |
| Curse of Stalked Prey | `7971a2a6-f89f-46a5-9008-96cfa49de41a#card` | C | `6484f3f80e9588b8a26d033ee32fb6c7b518fb00ef420eb77f201ac5137a2038` | 33 | 3 |
| Dakmor Lancer | `00efb113-ba5a-4749-b42b-5693b980f26f#card` | N | `fa64169a5fae1a42df3431f6957c662193314d666e18ad6e1ed3f5b2afb460e8` | 20 | 1 |
| Dandân | `88929373-b2c8-4a81-a809-fed87fd5b0d7#card` | D | `10b2efb4f9d744f6c2690297bbcb22c612bdc8b2d7c3de68b2e6c0e9579371bf` | 44 | 3 |
| Daring Saboteur | `d36f64a0-53f7-4c56-9f40-a7927a5c2608#card` | C | `8c349a22ed925309b7387eaf343710aecbf487d77567ce0d43f37b9250cedd45` | 73 | 9 |
| Dark Banishing | `0df450be-9bd2-45be-ae59-fa8d19f6a391#card` | N | `542c5a137275c91f3f3151c28c05f7416fd3ef3886237388e08cf813ee280c64` | 27 | 1 |
| Dark Hatchling | `36633b43-855b-4620-8afb-70c39fc07280#card` | N | `bd76ddbde18acdfb86e2a85ef2812955796ec19ef0c60ee178a130cfd78f1ae7` | 37 | 1 |
| Dark Offering | `102356a9-0de5-4034-b121-034da5deb4f2#card` | N | `97309140adbdc2d8e0b91f0f2b8e8aad3cf37b402e7b820e376f852e00232109` | 24 | 1 |
| Dark Withering | `ac2720d2-b010-4fa9-8c94-b13680b75aac#card` | N | `306542888a3cb6f4c0b121dc270d0510ee31921db10c7e5c7494f565266255c2` | 18 | 1 |
| Darkness | `932708ae-9d5c-4561-aa8b-0d2222d37fdc#card` | C | `08f4c88b0b9b363cddd4375ea8afe786e30b39b7c3e038888daf52ca42a4a053` | 25 | 28 |
| Dauthi Mindripper | `ecae5eba-8407-4c71-aeae-ab10fb1e5226#card` | D | `edd810bfa4489dbd67cbc7239d71547f97edcd79f37f1e1837a2a3506c4a7b45` | 51 | 1 |
| Dawn Charm | `a8f5cfa7-4956-4182-8e44-acf3493239f0#card` | C | `083e9cdb248b1c376eed0cfbccb245513b4d3e1f22c76dc0373f4e2447e38da6` | 60 | 28 |
| Dawning Purist | `9e55cde0-16e6-4d32-8a37-1ef3dc812971#card` | C | `1ee3845fcc122166a866c7c8b283589938f484d9c4597ee4588756887d57cb3d` | 43 | 3 |
| Dawnstrider | `d2783a37-b6de-4184-b094-e1a23f185a94#card` | C | `0bd9ecfc9ca0f06c08f4460a66a964342133e5db1e6b3bb75976591f9ce851a9` | 40 | 28 |
| Deadshot Minotaur | `3992369a-2158-4cf8-b7e5-be517b91c1f5#card` | S | `263bcf9f674020cafdabb965624effd17951768f3da5869a744bf2f83ab1d929` | 31 | 4 |
| Death Rattle | `ae327976-1026-4318-8846-f6b0cac373a7#card` | N | `fad64b7936e4864308ae0d85a02362b4f5534e8e8df0ad1647f901ab97aa5242` | 30 | 1 |
| Deep-Sea Serpent | `7fbb98cc-585c-4184-97f5-9b3d3ebdb1e5#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Deepfathom Skulker | `a09a4556-8913-495c-8768-e05413c48f0b#card` | C | `ad912c38f2932dd630361de2b28a708e7a8f4a1b87e4ae955407b69e21b79124` | 64 | 9 |
| Defend the Hearth | `1dd64972-5713-4c25-b7a5-a35a56886ef4#card` | C | `011698becb83663913a0ac964f51d4205c199cef6ca92a3ac51838283ac2c1cd` | 28 | 31 |
| Demon of Loathing | `2e55fc03-779c-4a71-beb7-aa0ade738c24#card` | C | `dd4dd0465abeccaa4afd9224474d7fa7d9224dc8bdea65df0465bfd4456866dc` | 39 | 3 |
| Demonic Torment | `2172b724-9004-488e-88d3-a5fc48c50e41#card` | C | `0116a7e36261c9e5f949314d57f2e7bf0aa9c77191e8097c85572214e49f367e` | 45 | 36 |
| Derevi, Empyrial Tactician | `afa49a09-146f-4439-850e-dd1938c93cef#card` | C | `2d5733966394949d0b70f85f251c6c04cadf477f73a15bd58c96ef3c351ad94c` | 75 | 9 |
| Destructive Urge | `5ab62b09-1135-4d22-866b-3a0d64228bf9#card` | C | `24f664470f012171c86c0a68aadd6e9b95807c39b52c617c54cbe02620c6724c` | 38 | 3 |
| Devastation Tide | `4245ee98-2d4c-49d1-8d07-80760cae2bf9#card` | G | `38b674b3309f8714f0ad5add5eb04a8023a513a0d9d7823c83e07ea564d077b4` | 24 | 2 |
| Dimir Cutpurse | `56c1a548-c80c-432b-bf92-d4ef6117b298#card` | C | `3273bc56fc81126cafb2493bb507d0ade5362726aa59fe07516ba2de976b3f8e` | 38 | 6 |
| Dimir Locket | `3ae4b679-2e3c-4f60-bcf4-b279917cdc54#card` | S | `60bbafe581ad00993d8563c847e11ec67075977c01ab6d51ae20425fc4f24ffa` | 47 | 1 |
| Disappear | `039e2e39-8f7f-43f2-8448-de35ef7b8dc1#card` | G | `7dc5c2afd5ce48d1dfa5e8871be4570a2ff89f4b8414653446fd3759cc601737` | 27 | 2 |
| Distorting Wake | `e5387fc7-c3b3-4933-bffa-580ad793c7ab#card` | G | `93f5d30fe5593b9c5e94d0bcfd1a32f6c571c98afd11cfae4c08be484a2b8fb6` | 20 | 2 |
| Distracting Geist | `2fcd5779-7234-49e3-b3c5-ebc07db74462#face:0` | D | `fea85ee8ed17e19be90eb50cebdab294ac818621b6d57899511b98e47dcbbce6` | 30 | 1 |
| Dokuchi Silencer | `d377b28f-c887-4937-9f14-74484b612551#card` | C | `e37438a8e43e60a5f08d78a4bd27bc77f1decfe2255884c09b91e7cbfd041d76` | 64 | 6 |
| Dolmen Gate | `d53d000f-c92f-4d70-ae60-8be3791da8a2#card` | C | `53d25c9a82b04cb0dec147e310360d3445b2c71354eb35447f55748c5c953f14` | 32 | 88 |
| Don & Leo, Problem Solvers | `a4ae71c8-ea1b-4fa4-9e47-cf00ab60c5ee#card` | G | `41890cf8ada0ceba34fc3fceb55a45bceb214508add0955f6ea8bc988798129b` | 63 | 36 |
| Doom Blade | `59e7f2ae-4535-4191-98be-3e65b6b2befa#card` | N | `2fb8cbaf77324f72a84d1befc9caa9c6d7ed0fc12700494503db6e46632586fc` | 13 | 1 |
| Doomsday Specter | `5811effb-5010-44b9-bb97-9d7eb358327f#card` | C | `2e0324631b9639d1eb62d6209ef4dd76ca1185a7bd3994e07a441377c6bcd83a` | 84 | 168 |
| Dowsing Dagger | `df34a6ad-ae1c-4470-8c9e-49815bba1973#face:0` | C | `7d0f7935e2bb874f9843f7e295f065cfe13e1e31375008ae4db5b6ffd28bf04f` | 89 | 24 |
| Draconian Cylix | `0880b36a-6141-4955-b532-cf88daca0869#card` | R | `23308ea58b8a53c313006dbb5a2605ba6fcfe6150256f79dfd8724fb217e3d25` | 28 | 1 |
| Dragon Turtle | `a3270e64-38d2-49e5-8a1e-a5e81205776b#card` | G | `02373db5d3de693c4e9f425e5be05b154d29f15498a270270b9246ccce539c3d` | 153 | 12 |
| Drana, Liberator of Malakir | `89b24b5f-d837-4274-8877-8ff7dc2708ba#card` | C | `09c1716fd0411c07f733fb3fe11fc362c946798b9e4249bff344f1e64a04265c` | 41 | 18 |
| Drana, the Last Bloodchief | `3ae3b901-4d36-40e2-b861-26209fa1e823#card` | D | `000a6cbcf046041adef3b09e52093aade8e393cb27060f5ce884629587663a03` | 83 | 1728 |
| Dreamwinder | `eb70d548-9769-4569-ae79-cddc0e623aef#card` | D | `07ce51496abe5be5f60bf24ac9d41d148607a8ef339fa3e63264ebf8fe6d8d3f` | 55 | 9 |
| Dregs of Sorrow | `8126e60b-cf3d-4bf0-bfb5-f1e914d3e20e#card` | N | `1c8f64f899f5a1f16fbb2d3fb9a2f4cb75b1039ae79b247143dc978194825696` | 25 | 1 |
| Drider | `1db76752-31ed-4a16-986a-d0852ba7d812#card` | C | `16f7f8470eb8c16d0982a59573647670c7c138168269dd3674dde3329af60381` | 46 | 24 |
| Drinker of Sorrow | `6208534d-e7c7-43ef-b8b2-fea1d82459e8#card` | C | `394c2f37f1254cc4976d7adea722ae67d34fdfc7f8b2abc93633c9b33f608685` | 37 | 1 |
| Dripping Dead | `bf0c85d7-16a0-46a5-a3a6-5591849bef60#card` | C | `80ee79f7b3a2a734d3470efb1608e79179c0746dec8cb17613cc44a7b2fcfdca` | 54 | 3 |
| Druid's Deliverance | `fde7645a-5f02-4d5f-b38c-8390f325899e#card` | C | `3340c41931a7113c6fe70abfd6b414fe86f20d1e4aa5143f7a10f003776f0ab5` | 33 | 31 |
| Duergar Cave-Guard | `79d7950b-92c1-4431-a69e-26544eb0c7f0#card` | S | `8a055a78c0c9a6136077936e58216b6a54ff1285b40593963c8e85773ed43cd6` | 28 | 2 |
| Duergar Mine-Captain | `3ac276ca-91d5-4c7d-a355-e56c67ac3ba9#card` | S | `5780840b98e94e9e94a4bdb1c632243ca1e98aa2f2b0f8921fd4b8cbe0663366` | 32 | 2 |
| Dune-Brood Nephilim | `634bd800-8caa-47ae-8b70-2c66baf9a355#card` | C | `013cdc13080526a600782c22cd9018723705636d1fa782cd3d922a96d8761f7f` | 48 | 69 |
| Dwarven Patrol | `4369adc8-d8e0-4d51-b2a9-7e4a8dfd9f8f#card` | N | `1b32d1fced79ce861a93147337b475452dc1b83ae7ab3f41e42ccb470e1d7130` | 41 | 12 |
| Dwarven Strike Force | `66dd0e68-f5ec-45e5-991f-d588aa726387#card` | R | `0fe5759378fbfd85fb468789e49f50fabbfda1bb4593f1da1d8cf0908030e315` | 28 | 2 |
| Eager Trufflesnout | `7f2ba562-8d5e-4915-967e-54916c3c5876#card` | C | `4e7667336fa41c0e0b90ebce53a4db5446ce06c654d35a6ae602edb2b204e026` | 31 | 3 |
| Edric, Spymaster of Trest | `9a1de7e4-9930-4db3-a8f3-d146d0abf38b#card` | C | `ce0b1c4898c9cf40a465875bee977882aad355db42ff2a2086ac34ba3f50f619` | 36 | 3 |
| Elemental Masterpiece | `e9f5f87d-a485-4e69-9beb-49a51230bd53#card` | S | `0b36ca02b190c9bb7a9c605ebe05de380fab54b3b53e204552756e4a1a23db34` | 51 | 1 |
| Elite Headhunter | `fad0118b-d046-4ee7-87d9-e6799f70a7a8#card` | S | `67c713b9805836f3825300403a5ad0cfe7e30271b9c72b98582a2bffe94d1706` | 38 | 1 |
| Elite Scaleguard | `b90af42d-edb6-4654-b7dc-10f0f272c883#card` | D | `18e8ae4484ccb20fdbd5a7f3bdc0be33f433c65f42ec6b666f8603522d831c73` | 55 | 5 |
| Elvish Hexhunter | `f8f450c1-f405-4ccc-aef3-f0186d1f5056#card` | S | `5ee689ea817010d127b00218e33b8164a17eac12f0bd414d98a4b127ec89588d` | 26 | 1 |
| Embargo | `edadd0bf-15c8-4e9c-810e-2edfd77a9d01#card` | G | `8166ffc9c7a68b3b1bc129ebc187baa7fce89335ab6652d53abc22dc46149813` | 46 | 6 |
| Emissary of Despair | `4efa1654-c822-4477-ad24-f379ed7ecb11#card` | C | `411b2858a0a58db2dac88bc894eec652c6d08f359d9378932e2a8e2364fabe9a` | 42 | 9 |
| Emissary of Hope | `62a6dbdb-70fc-43ab-8e63-367a609ecdeb#card` | C | `1d8254c1d70de22f7f8a24d93a15ca828681fc587c2d5c47b8bae5825ecf4b64` | 42 | 9 |
| Emperor's Vanguard | `5007425e-a626-46f7-ab7d-777af97ee724#card` | C | `ecef6c928931303da4b36d5ffa75579acf5197fa50ab8a69bc5e55991435eab2` | 25 | 3 |
| Enchanted Being | `c98b725e-ca16-4576-bf53-653d4028d861#card` | C | `13b0985b5734a6b4501e04806911ac6ccb9b903414fde4cb20f1804469054f44` | 32 | 58 |
| Encircling Fissure | `39d7569e-62b0-4f37-813a-0985632c66c9#card` | C | `0449275c1cd185990a483504d3641f50c417973ed07d310464c16bea0c211c81` | 44 | 167 |
| Enemy of Enlightenment | `783f059f-90d0-4148-898b-9e694b22ac0a#card` | G | `1de3e707d2bc7b6370697cb2502512f3539c7a2ebe950cf2481e5e057fdb7011` | 55 | 5 |
| Energy Storm | `567d3de7-8d56-4b6d-a59a-f8674172f595#card` | G | `0567c41231c0d582456aa619ac97994556538de24425f96255f10ee8c7ffa722` | 58 | 84 |
| Erdwal Ripper | `6a207599-188f-49fc-8932-1974fc716f0b#card` | C | `8e39123501f9f65b5f040babdcb0382e45462cbb0cfaf0434d439b862fe78eb0` | 31 | 3 |
| Erithizon | `45645a97-f41d-47fb-b943-293ffc18fc82#card` | D | `4d853e85a88a7d3fb98bd8b5639bb4717caff57932191571b65cb80d4c146a11` | 28 | 2 |
| Essence Filter | `b9f63388-d1ac-4895-84c5-e5b1db187463#card` | N | `84159d06b909429c7306390fea1a2dfbecd94acbc626bff6691cd1a95baf9e9d` | 16 | 1 |
| Essence Fracture | `31a49fb9-1081-46c3-a352-0444869afdc9#card` | G | `0b7b7da8eea4e586a144d110b736be45a88239d8b24a1606d23419e11dc868d0` | 24 | 2 |
| Ethereal Whiskergill | `3d3e6dc0-2aed-4afa-bfa9-59d04ade9bee#card` | D | `06ec86c03debb422ade22e318ddfc3997fb9f3301383f0debc67a04fe5315d00` | 27 | 3 |
| Evacuation | `fdd94383-b573-439a-8e1c-925af887c5a6#card` | G | `ec217cf75ff220761a2cd1a4361b234296157d59d77e0fb97443bbec4588e466` | 16 | 2 |
| Everdawn Champion | `964e065f-380d-491b-9225-244c5f9edda7#card` | C | `0bfa0a2efc184c56cfbd6d282c349cd2247e24e9276e8d0401a66185c1bd9d37` | 25 | 19 |
| Excess | `abcb5b99-bf5e-402f-97a2-abb1ab39dd72#face:1` | C | `009f72b3529b7630d90614d9db7a1a61cfaa50a6ce5c815c5795c0a7473e3d55` | 40 | 100 |
| Executioner's Capsule | `c9cd266c-7ecf-4beb-b9da-69b88f33abd3#card` | N | `01bb424c14dacad1152042ebbce3b6fea706837311a2c903c0b515ded91e3079` | 29 | 1 |
| Exsanguinator Cavalry | `a867be6f-46ce-4a12-a827-9429bd94243f#card` | C | `2362cba62a01b8d73426b35ce05187bc0dc11bbeca7f57c99150b5d821e49d52` | 46 | 9 |
| Eye Collector | `553c2006-e123-406e-ab4a-1d1f67c7468c#card` | C | `9dcba106f4036a1a57af36be88b6686629fd28a1231aae46718c619a06753333` | 32 | 3 |
| Ezio, Blade of Vengeance | `2127a245-d664-45cb-8ed1-0024de1a10df#card` | C | `8be5c744cc7c4ceaeaa5b845f75e6c660d5e346edf27aec5bb97920b25a7b37d` | 34 | 3 |
| Falkenrath Forebear | `940080ce-c504-48c8-a763-326fef907217#card` | C | `c05d1b7f861011812215fb2047ab638f2de95de7a86769c11a99caaecfee2148` | 76 | 9 |
| Falkenrath Marauders | `f819b134-24cd-4863-a46a-0a068e34ca9e#card` | C | `46659820277644223241e6b7742077621fc5894fb52fdf3e6eed6b1be8e8e79f` | 35 | 3 |
| Falkenrath Perforator | `6e991808-fea4-46a8-9e87-7b5917f33ce8#card` | D | `367d8e763da361ecc60aa986ef353aece8d69d0253734b085d5cde3876f081f1` | 23 | 1 |
| Fear of the Dark | `9285cdf1-ad25-4aee-a865-1bf0cc905183#card` | D | `0a2bf175e6b9b02ea4506b533da51f9f7e39651a20ebfea2c0479ca8e6406909` | 37 | 4 |
| Feint | `1bb8fe05-abb3-40a8-9e80-5d99ed0e4284#card` | C | `0327a56cadb361c664879df861a65dc6b47991575ea4dc9dacce00fed07466c0` | 53 | 213 |
| Fell Flagship | `e5afa88a-8bde-47fe-a890-3759f807360f#card` | C | `3ce15ddd420bf3938e90b349cf382d094d8005620ad7a28acd1db441896488e6` | 54 | 3 |
| Fend Off | `fa8f3827-8cd9-4896-ab0c-26fecacceb40#card` | C | `003584a7e7f94ed191b4bca359789d226710144e284f5223f5fb0459063d3901` | 35 | 58 |
| Festergloom | `a653ac26-e9c9-4319-88f6-c83cc64d3bdb#card` | N | `c2b0d87be4ad1393abb8448a70bef0c943ed9376fd8cd0b9e05a9c1f422849d9` | 23 | 2 |
| Fetid Heath | `42bf259d-4bb9-49c3-b4ec-223dca62f4d6#card` | S | `c6b5c22d067f0ecf9d7953cc2e9b4904eeaa2dec8127223fc208b907cd896c8f` | 41 | 1 |
| Fiend Binder | `060c7999-ab81-4e37-af65-7cd219b693c8#card` | D | `9cfc2d8d4e3e4f025cbf2dcedf30960736a8d6be7270e6d2bd0ba137e22f3b6e` | 24 | 1 |
| Fire-Lit Thicket | `d99a1d9a-7721-4331-bf22-1c6ee0bd825a#card` | S | `ae57ee8c7c2fbf9ec6670518017563f8591999bf85abecae4f02de50795b5f01` | 41 | 1 |
| Fireborn Knight | `b77695da-86fb-4675-b2d3-51732ad0325f#card` | S | `08c2eba9ca2abe238f9c119611925747324691eb08c00100e5a5953ba9a202a5` | 31 | 2 |
| Flamespeaker's Will | `3abc09a9-3baa-45ff-b048-53e42d4fa9f2#card` | C | `bf290999c2f76a6e919abca98cddfd0117be08ebd1ea034cf516690d2afd036b` | 73 | 3 |
| Flaxen Intruder | `bacedc99-46d9-4757-8a27-8df77d7c2f02#face:0` | C | `dc07fa8a617b18304c5e405ae1d7fe39926975607a8102d76b9b011ffbecce1c` | 49 | 3 |
| Fleeting Flight | `6336401f-4e2d-4ebe-8c5b-24aa6f516abf#card` | C | `10b661b3330c4395f856696f24d588525d7002145e8b9140eaa0ebe6e97c1b47` | 53 | 62 |
| Flickerform | `e5345c28-7046-4ff3-a5d6-eeb7a0fb230b#card` | G | `78baec5d7b380ea49a7fcd9a1792f99f096d499160158b9040bde079f016e33a` | 94 | 242 |
| Flooded Grove | `dc974eb4-72b9-4213-887b-8ee684b93420#card` | S | `248abb5dbc68f9f4d75822ba93aca688fb7f2351d154d1743ed2a350e9a9ca93` | 41 | 1 |
| Floodpits Drowner | `43c69d19-c06c-470a-bdba-a0335ff2d655#card` | G | `522954b153f4c656c7efa5d55ec948232554bf1ad9dbe1e771c02751801cd3b2` | 74 | 6 |
| Floodwaters | `e89cce0e-4fd7-40fa-8a55-b07d72826956#card` | G | `314cd802d9eb3a360df35087a7d2349c90fb9b25b86c9754751e9a5d6419b384` | 25 | 2 |
| Floral Spuzzem | `994de451-14f9-466f-a56e-da052b4666e5#card` | C+D | `aaae034dc3d621b58a4e2543fd146736f99c657b8d0c391fd32b7978e0fc1fa5` | 59 | 1 |
| Fog | `27e9db49-7af7-4bef-ad4c-bf5dfb92030d#card` | C | `08f4c88b0b9b363cddd4375ea8afe786e30b39b7c3e038888daf52ca42a4a053` | 25 | 28 |
| Foot Ninjas | `306652a8-9a9c-4b33-96d7-b59234d148e9#card` | S | `1652d655da8e2e4e657a312f64bcab9ab9f11dfef4ab9f642952fd218fe2f7b1` | 26 | 1 |
| Foxfire Oak | `925775c6-b6d2-4793-a363-bd5aca871aa8#card` | S | `052dc2d0a8e719b6e3a8ce2cc572f56caa548616919ec72329746bbdf94e1ecd` | 27 | 2 |
| Frazzle | `55c4df81-9513-424b-aa58-10190797bba7#card` | N | `421db993b54d72028da8db915ba14fd54b945494233fc610ea665ac991225fc4` | 13 | 1 |
| Frenetic Ogre | `6eb4b649-c578-4b29-8da1-7c757412bec0#card` | R | `b8aa65dbc41f6d82963ddab0c3ae2640396bf956f923b0990bf9f6d9579999b6` | 34 | 2 |
| Frenzied Baloth | `6a4ef075-9254-4d36-9572-622833fae54e#card` | C | `14287091d99d0352087917a3f20746969414b617d4ebd815db6a475e6edd09aa` | 65 | 2 |
| Frenzied Trapbreaker | `9545f126-062f-4410-a362-e16255a128d6#face:1` | D | `12c0bc5830a45fe95154dae4d60d32231e70b511e596f5eae0020ef6d10940be` | 53 | 2 |
| Freyalise's Radiance | `d7f28d00-2ec3-4d4e-b3a6-89d33c66da28#card` | G | `91222c7528cc57c1da323a82a39c13757c86221985ad4b614d1c7fde36fd9e9f` | 28 | 6 |
| Frostbite Pyromental | `9a104f73-597b-49e3-8088-13db31f61900#card` | C | `d4200ac745c7c52130c1b4fed226c7fec707cffaeefb923dab4462e3f4aaefec` | 53 | 3 |
| Frostburn Weird | `f667b4b6-c97f-47fa-9146-c5c1f3f47e92#card` | S | `a4467e6aa27faf47aca95a6279bb0fe8f432ef2c8248e0298cca72473ecbc26f` | 25 | 2 |
| Furnace Scamp | `d0868a52-5b19-4429-acf1-399f3ae8308c#card` | C | `7e3f42bfdd8a59cbf95835f7ee5e38945c13169396087422db1166baec2e1186` | 53 | 3 |
| Gaea's Revenge | `26f6cc1e-d85e-45a3-a1ed-7f09d7cfc7df#card` | N | `0fa7ea9640558217f09d60796dedd8f8281c3cb4175dfe1fdcd6b109331d4770` | 54 | 22 |
| Game Preserve | `409e0a43-0e85-47ea-9d40-de0860b12952#card` | G | `1b36e450944aa449471f62cbdc404369a8df3df60c24235848fad36f4e1b792f` | 68 | 8 |
| Garza Zol, Plague Queen | `5b48e926-fc0e-403c-a47b-1f1d96e2eaf7#card` | C | `6a1385e56205416de4c27e553c56484005a166c830f356f572a6c52c6ef2d2e6` | 70 | 6 |
| Gateway Sneak | `66c227f2-0e74-43e2-ab24-3866d15c5eef#card` | C | `4cd6d344759ea1a14b63ff10d1122e242b703a4089d1662ec13fc1376eff4eb2` | 58 | 9 |
| General's Kabuto | `cf17889e-9201-4af7-bc2a-9539b04fea15#card` | C | `0dcb5d9a049bc9e1935ebab7c82defc9930c2e531103b2f195a9af8e21583099` | 42 | 19 |
| Ghastlord of Fugue | `fce163aa-c5d0-4147-8566-512321c0be8e#card` | C | `026bcc33e5c0ba11fff44b3f9bee0b62526819b6334b54e0f7c91630b2a315df` | 73 | 9 |
| Ghostly Visit | `73b6694a-47d4-417a-af9b-470da14b5b0e#card` | N | `2fb8cbaf77324f72a84d1befc9caa9c6d7ed0fc12700494503db6e46632586fc` | 13 | 1 |
| Giant's Skewer | `aaa8377e-21de-4c63-a26a-c25d986a272d#card` | C | `4fa523f0bfd99ca3ab3787c2e39d5f02cd9039f3fd1d8d1efffe6c4bbbaa3b85` | 49 | 3 |
| Gimli of the Glittering Caves | `f6747295-d21d-40d7-bc70-baa4a37ae668#card` | C | `0dcea054f9f06f379c141cf5ba2f2e2c688ea548f2c51e6a5ff73943230ce9c3` | 59 | 54 |
| Gitaxian Mindstinger | `80ecb069-36e2-490d-9f6c-ef553c00e997#face:1` | C | `3ac43615db597060e7a2bf733f8f2141ef454660c7db92b7e1ad34aac9eccd8d` | 31 | 3 |
| Glacial Crevasses | `88e0551a-ada0-41d9-b5c3-39257ce56c3e#card` | C | `12cfc546d58bcad39bf1e51212e046c244fda9d9f3965228f5a0180541ccf36e` | 34 | 28 |
| Glassdust Hulk | `a1379c25-6daf-4efa-a517-6e6e25c31875#card` | S | `730697ef4d36573924731ce5467190099c8cf32cbd1beedc5adb327c94a7cc13` | 49 | 4 |
| Goblin Racketeer | `9336a62c-f2f9-45a8-bf69-86060aa0ce59#card` | D | `c28925097aa82adfc67c0ced6906e247cc08b92728554887a1b9f95a49678f73` | 29 | 1 |
| Goblin Rock Sled | `ada3247e-ec5e-499d-bc15-1d9dd80a59ae#card` | D | `0222f5c5beb8a89e2a23f300fc376d461c1e86b4dcbe94a0086a072dd20b0938` | 59 | 96 |
| Goblin Test Pilot | `4760bb87-56e9-47eb-b5ec-6fa4dd19156e#card` | R | `1a658045085b0bb3b5a0cd0bdd5b16a2ff22b205ab556f1d8f039ac034ca0250` | 28 | 3 |
| Goblin Vandal | `50bebbb9-01b7-4eb6-8efd-307c9bcd2517#card` | C+D | `1ece8171c947b0b4306f22072bec7146abe72f95fd41145898461158e0d5b996` | 65 | 2 |
| Godhunter Octopus | `930b48b8-dbd1-4109-9eaa-7d7e9a04fa5b#card` | D | `33ff6f5de3087b34436e8c54c730117156ff6f488450181406b28bba66978e02` | 29 | 3 |
| Goldbug, Humanity's Ally | `d1960749-c11e-4ac4-97cb-f37d7875d479#face:0` | C | `06116f6070d64d200dbe039f6bdd31fbd4eb31d7b7db9e9ebd93d3657f730269` | 63 | 704 |
| Goldlust Triad | `86d76078-9af3-45a7-90d6-72946afdc78b#card` | C | `1617dc14110f06ee329426292f6f34a2a840bdb5eac20f44e69e268a091518b3` | 34 | 3 |
| Goldvein Pick | `c1624d10-8838-4af8-aea1-a96c0fe6fd6b#card` | C | `8e72d96775a500d9aa8ae537348357c488268b25f7bf767c7f673cc8af192cf5` | 49 | 3 |
| Golgari Locket | `dba6e24e-7875-46b9-b51d-4cd71568ecfa#card` | S | `f9d132669e7429bf917b479830ec52aaa920ac2176bc3a15c9a155fdf06dd6cf` | 47 | 1 |
| Gorbag of Minas Morgul | `e77e97d7-58ff-4454-ba49-66182e550f12#card` | C | `36952672648966ad83f77759bffec8538089fd7779256a4d83222f6fb56a6812` | 75 | 6 |
| Gorilla Pack | `f2c8814b-581b-483b-a7ae-d3d7b962aec1#card` | D | `68552b6518502f35fd2bb9cfa591f292858a030fd3b4ec551997f50792c6f43f` | 44 | 3 |
| Grateful Apparition | `f1b3e0fc-1dc3-4eba-bd54-557f036f5ea9#card` | C | `9946645695701cdd3ad072c094aa99564174bb458ff9e883ae8bc7cecf147f22` | 28 | 3 |
| Grave Peril | `f5a74cee-6b17-4ed4-bc05-94fac53f87d7#card` | N | `205ae583c13dfcc26685ffdcc2a56f5a256b8bef9eda3d9f043a8cea4f013b0d` | 37 | 1 |
| Graven Abomination | `2590801e-0de0-4933-be2d-96a1d3cb4585#card` | D | `352ffd5c54142dfa3fbeabc8c5b156c05123ed861d1c60d32740f46728b53eed` | 25 | 5 |
| Graven Cairns | `5004b84a-33b7-4f6f-b2c2-7086b9087535#card` | S | `74f8a1e4d5e75e098e5a5df4bb475f8b7b0675875363ad691ab1454cbac65914` | 41 | 1 |
| Greater Harvester | `1e8f17ab-4616-44de-adfd-010882cfcd42#card` | C | `76aecf48fc02aadfc77d2330a92e350e35dc1970f51d2bf1c711d36fd6c815d9` | 56 | 3 |
| Greatsword of Tyr | `81a602b6-a4f9-4534-a84e-2fa7556c2309#card` | D | `82a78eb1b7188527447649ff6925fb261f47aa566ac5580aacce071380176396` | 40 | 6 |
| Greel, Mind Raker | `2b84a10e-4c5c-407f-8b5b-a23d402f0529#card` | R | `8971253c1a7b2875fab95a2e0a4b179d5173489fcc4418fb6ed699547da45787` | 34 | 2 |
| Grotag Night-Runner | `623e261c-d4b9-4dea-930d-e0573a1deaee#card` | C | `31b07d5abc67446e79fa4d25e3c4e87d7139467e7b6dd651c1ce316288e93822` | 50 | 12 |
| Grotesque Hybrid | `c37e69a7-5596-4dce-a3c9-41f899e05f24#card` | C | `8adbe5e9eb364501fdcbcc0d0d857322de62f3cf2ed41af4ff82fc54206d31af` | 73 | 6 |
| Gruul Locket | `7018ccd0-96e6-458f-b412-197edcb2bafd#card` | S | `0cde8721ace7250dc63d910bdefcaf1966d7538de9d01978c9f14e353df730c2` | 47 | 1 |
| Guard Dogs | `490af644-2e85-49c7-af63-d5f0a9babff8#card` | C | `20096ac61a8c3804e59389d1055ce1a930b7fcbc662710830ebbcfb6c27f70f2` | 58 | 101 |
| Guard Gomazoa | `7c565975-aebc-4599-ac32-5594c718e2cb#card` | C | `8ccb0dff188ba49a90bb7d126004a9a8342d66940160a1d4da27ec83098b964c` | 30 | 19 |
| Guild Thief | `bf061d7e-46eb-4037-968c-3cfc5ca94552#card` | C | `705da6b19b48797c584e280bf9986ca37e3cf07b5c59b4cc336c09504687798c` | 153 | 9 |
| Guildpact Informant | `330f97ee-6bb2-4a4b-a8bc-411659dc4260#card` | C | `9946645695701cdd3ad072c094aa99564174bb458ff9e883ae8bc7cecf147f22` | 28 | 3 |
| Guiltfeeder | `0285bd20-f49e-48f7-8c8c-960f9fbe4d34#card` | D | `203bd66c56aaa5129da8a5f79e679c576eed92bd8697ce8b69e739c7569c318e` | 39 | 9 |
| Hammer of Ruin | `78ba3363-0fc8-42a9-b586-69d0795ca65e#card` | C | `9ccfa5151611d799eb1e5930707b050df6b6849e714afc7a05f09edd7391f077` | 58 | 9 |
| Hammerhead Shark | `ce176172-6c7b-40b0-a6d0-68e9c32b3402#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Hand of Death | `dc45b2e3-272b-479b-8e3b-36eead606a3a#card` | N | `2fb8cbaf77324f72a84d1befc9caa9c6d7ed0fc12700494503db6e46632586fc` | 13 | 1 |
| Harbor Guardian | `d72b69aa-fb61-441e-9b62-4534d7a32a69#card` | D | `6e16c52803064668931693eb553eb48d58c9f53e05105af8d18e30cf779221d4` | 27 | 1 |
| Harmless Assault | `bfcc3a16-1ca8-4112-a10e-d4e52d3daa8d#card` | C | `070015bd75d45f5b0ecc5e7fe0e14a43952bc670ea7d6702db16442bc43f657c` | 31 | 204 |
| Harnessed Snubhorn | `e1dd9dca-cb5f-46b4-b16d-a61ae651d653#card` | C | `1558e34b6928eadacb393dde240233991e1f7d50ea7f8dd9abd2f1abf692d9e6` | 39 | 36 |
| Haunted Cadaver | `7f46839f-175d-4292-8057-c3e0da206075#card` | C | `7a27862a3ed77a0c82609e316dd5be0820362639b895818cd3a583c61734aa6b` | 57 | 3 |
| Havengul Vampire | `616b3ee0-4678-4562-a21a-5f2d31b9964c#card` | C | `3db7080a1a378bfe381f62e6cf86d7c8320e964823e5892eb823629e8f4febae` | 49 | 3 |
| Haze Frog | `d4cbf16d-6da8-4018-8398-8e263ef6c69e#card` | C | `1a7ee06e9887c52ee2405eec0f6ad39ff12f9677f17b6f4d22cdcedcf63547ba` | 35 | 6 |
| Haze of Pollen | `f0140f00-5414-493b-962d-33312d49f6ea#card` | C | `0877ebfed78e94b7b229052da00d4f74452009599b08336d8d33a2d24fc3b5cd` | 30 | 28 |
| Headhunter | `83578de7-5488-4952-af47-8fe966897210#card` | C | `05741d63597226f0523a5359906f19fe242b8e9a9402614454430a77eca15a6a` | 34 | 3 |
| Heart-Piercer Bow | `ebe9dad7-d74d-4fa8-b17a-88ff79209379#card` | D | `ac68a746c354442f4c90cdf149bb5dfe42541b371237a3302cef2e803f652aaf` | 35 | 1 |
| Heat Wave | `d8f08699-6210-41c3-bfa0-c4b3bcbbaf1c#card` | N | `090d24b5d61742f1c22427be973a08c9713ed001fe0ec7f2419bf0bed829df5b` | 75 | 33 |
| Heirs of Stromkirk | `7149de06-d676-47bc-a50f-e1058d89c890#card` | C | `2cfc55102a1f43832d262c93067639641f637ea22a1a1fd9d9fd2ebf4c2cf296` | 31 | 3 |
| Hell-Bent Raider | `ee8ab29b-0749-463a-abbd-eb0b9013aec8#card` | R | `9d0cf2fb532a64c7b23156121f75853281e56a1b10ced08651b2385a75cec055` | 34 | 2 |
| Hellkite Whelp | `a9b1dd4a-2eaf-4448-a767-da705d3ec32b#card` | D | `c70949502364abc1a2deec337feebbbecae96f029fd04313607b7c933eabc6d1` | 32 | 1 |
| Helvault | `fa3d8a6f-dc5b-4890-a806-f785a41660c7#card` | G | `0f2b73b9779ec7c9a14ba72f2e2ef3c9bebd43207374a446d7d440d4abe5607e` | 92 | 54 |
| Henchfiend of Ukor | `91b5688d-4dd6-477b-998f-bdc7612de37a#card` | S | `24a1a62400991545f659dc71b1c0b520df75e9e734b1c60e802cfb4c86043bf7` | 34 | 2 |
| Hibernation | `633b541b-e37f-451a-be73-3398e61e0f75#card` | G | `4299c35435edbbf951d129c131e32daf91f7cc5c2633042efac2aef86a625721` | 18 | 2 |
| Hideous End | `f1eafb69-e7c0-4a22-9e73-6bc32168a7c5#card` | N | `b4d8109e54cb56e6d07f1963558e883b5c12b7220404b7578830917841775072` | 25 | 1 |
| Hindervines | `8d13fabc-1e0f-41f2-8873-9f44b68f7e43#card` | C | `002fb1ca93cc189e7083726e660395755882c9ead697195917cc45d1138861b2` | 39 | 1618 |
| Hixus, Prison Warden | `02bff648-fbad-4d61-ab09-93993a5fda58#card` | C | `338e3ae5324846078d63ce42b884e6b89ceea331103c642a1b2fe336eb467efc` | 48 | 30 |
| Hoard Hauler | `2578e8e4-a93c-4904-9037-92a1529c2c67#card` | C | `28f05c3c40387bd5c233f273e0712319b372a4f33ae79c8e928029007306f96c` | 45 | 24 |
| Hoard Robber | `1a04f70c-288d-4010-a6d5-24e9778b9b45#card` | C | `d967dde9f0f376f3bab8e2aff161d88945945283f624af16076de807a31f17c0` | 28 | 3 |
| Hokori, Dust Drinker | `d2aaf485-8c74-4a27-b80b-f600c20d8b9c#card` | G | `00eecf0cc4e0373898224769ff2782cd70df949e9660c58c57a03384aaae0384` | 52 | 12 |
| Holy Avenger | `86df1de7-967a-4420-847f-7a77d0217a15#card` | C | `172e82c465578cfc8152785a193a43a479120b4b57f6a51bec6298d92e58753f` | 60 | 2 |
| Holy Day | `98423a34-f044-4811-b288-56981d604b6e#card` | C | `08f4c88b0b9b363cddd4375ea8afe786e30b39b7c3e038888daf52ca42a4a053` | 25 | 28 |
| Holy Light | `8f7dd328-b367-4546-ba79-1efc1e97f2ab#card` | N | `2225a3943a624d9871b2e1b3a7b677c8005bd871ceced7afd2f24871f154ec76` | 23 | 2 |
| Hope of Ghirapur | `1a20e805-c80b-4710-917b-706d29ec395e#card` | C | `181669ce97249e182a0e8d6243286630a6a6fa73da8398ea7cd76b6a7f415703` | 55 | 9 |
| Horn of Deafening | `50a1c14a-003f-424b-bb8e-2e2d51465a90#card` | C | `0699984f4cfa3640849b1182cf7df51101276434e9cbaada4c4c0a7b54662e37` | 38 | 58 |
| Hoverguard Sweepers | `853ef032-a2f7-4e6b-a210-03e96855d062#card` | G | `56672a422cf85f80b47cf078b37b41140afc114441d8d71bdd7cb9940fd59b1c` | 35 | 2 |
| Hubris | `95fbd573-c04c-408a-a8c3-f7e1ad5ddac0#card` | G | `896121231fa5265cc9e26be0a1602766e95e3e07ae81b6bfc842e7c14136f3b8` | 23 | 2 |
| Hunter's Ambush | `1f085891-0d99-46a6-8f09-b84f44d701f8#card` | C+N | `1384a92d323476e594a7ae4e31c03261c22a08b08782eb27e570db180aae773f` | 32 | 58 |
| Hymn to Tourach | `992d9a72-d34d-47d7-96fb-37f0efb515ef#card` | R | `946a19dce157a050b163a386cfbdc2d5507eed28f53fb4bf5bb1abb0fbd2bc19` | 17 | 2 |
| Hypnotic Specter | `759af941-f6a3-4726-91f2-9b1e4e55ea71#card` | R | `5a880f773e96bc98e849db9803658b7e922d9e7106a20e0b02cc539d8dda66d0` | 32 | 6 |
| Hystrodon | `17191fa9-e956-4f46-b541-465790125b6b#card` | C | `2acae94431914f5ae8d070d7fc35b5bd6239c29940e1c53e5a69bfaaaf3785c5` | 41 | 3 |
| Immolating Souleater | `fb260f5a-b937-435b-ab18-dcc57d6845df#card` | S | `4cc47688269da4cec3563644f8502305fb93c6378076172a2e3fb26929b20796` | 25 | 2 |
| Impaler Shrike | `db98ee99-d4e5-4623-8446-5cce5d11dca0#card` | C | `0d7e8bfb2308c416c3fa1f41a0f06c95516023a9852e130324728995e1fb2949` | 51 | 3 |
| Impromptu Raid | `f46fe37f-c552-44e6-9657-ff63dbbd0ccf#card` | S | `a29c91e732ef523621a1591de4c367748b0544dddf983cf176d678647741e07d` | 90 | 4 |
| Incinerator of the Guilty | `9b7d037f-3269-4e3b-9744-a977acb69c67#card` | C | `369910cfe0c91d37bdfe500daeb530404463dbd1aea67e6cd2b47a7e170a6a3c` | 66 | 6 |
| Ingenious Infiltrator | `354defd2-f63f-4a48-9fd4-2526b3878a84#card` | C | `5a2f3139c6d8c55e615b32dc0af6732d3ad2461b61da003c6ba815e57bdf7417` | 37 | 3 |
| Initiate's Companion | `f3264224-43a4-4c6b-b363-2d147687dfc1#card` | C | `2adf162899ae544ae11951ac4e1528d9b8a040a01cd46840073902024a14970f` | 28 | 3 |
| Ink-Eyes, Servant of Oni | `5d520476-740a-4005-801e-472b24fa6497#card` | C | `15c3ec4d2384b62cab9bcf610ac94d65720df8ace9b01c8a7f088d51d060d7ff` | 71 | 60 |
| Insatiable Souleater | `f81c0ddc-cd3c-42c0-a7d9-0057b69a5df1#card` | S | `77322ffc539bd56f8652a9b1f29f807de8a6a1a4985c4a8422c46628fcc118b3` | 21 | 2 |
| Inspire Awe | `9ca539cd-9876-4c13-b220-d553c17f2378#card` | C | `0013b4cd5877d46617840bd370ba9ed38f204bb079dd92e2f6c1e067172290e5` | 60 | 17778 |
| Into the Void | `4d5a970b-dd36-4871-a2aa-36ba21917cdf#card` | G | `24720e57c5b22c9840e1a8c7ce76163c7192d634d8b9cf75c23cfcf1b39d419e` | 20 | 2 |
| Intruder Alarm | `1e943e04-e213-4781-b1a7-935aad8790e1#card` | G | `0a1ce98159434317a6ddafd9d54f6224f5d800a19a07fc79b6e6b1ccdfce5e16` | 39 | 6 |
| Inundate | `0390a3dc-386c-402f-a89c-b94867bfb0a2#card` | G+N | `2fff6c7ebc989e0641e7553b67190a98b710cdf77b819b97984ead09fd867726` | 18 | 2 |
| Ironscale Hydra | `cef72b9b-91b1-47ff-ab6e-91d3c548e98b#card` | C | `11b292db35692fd56a191c861ffbb5acaa2b7443f88785e5f48975dd7e686fc7` | 37 | 9 |
| Island Fish Jasconius | `bb217f12-532f-4833-a27a-99e290aa47d0#card` | D | `07be858b34abbc9fc738e26ee1df1712c51d5a9e00054aa4f115c83b0c165833` | 107 | 18 |
| Ivora, Insatiable Heir | `d847542b-7522-43ec-91b0-f0aba5d7dddd#card` | C | `d6353e5f5d3997eb5245012495f8b40ed4a7211255f6b22ff6def59fcefb9f4c` | 60 | 3 |
| Ivory Giant | `38132f50-9e2d-433b-9f2f-9bd0a7925b68#card` | N | `66f9b49849470d7d2e84d64cd0125038e047b534757aa2acf50463fa28968050` | 27 | 1 |
| Izzet Keyrune | `8b8c0285-424f-4a82-bfb6-17752cfa71dd#card` | C | `f31e33b66dcec6d90a801def3659ab66f44b557a864170c85a242370cfe3aae6` | 104 | 3 |
| Izzet Locket | `7ee07546-2c2e-4679-b671-7ea509b1d3ea#card` | S | `c0fb649d7a89290ab4a45417856fcd3e42f0eeb8cdab65657516678d31ac63cc` | 47 | 1 |
| Jackdaw | `ac161100-46b7-4f0f-a72b-84aaa45980ad#card` | C | `09d46352e40f259f857cc7a2ab8482cf6e8df465354bc37ef9d4b1f6518e00e1` | 62 | 15 |
| Jangling Automaton | `b6f67539-9f76-42b2-96a8-575f27ba928e#card` | D | `0d7ccb31cf397421d93835dfb3576b62feb0afe9dc2ede3297526ebfdb7ff8ff` | 24 | 1 |
| Jenny, Generated Anomaly | `fbddb6a4-f817-4f3d-bf75-63a98e71954b#card` | C | `f99eaf5e32945307cabd64395fdaefd3ab39344318ce8250b7ee323ee9caea0e` | 28 | 3 |
| Jeskai Elder | `dfbd5ad1-0f68-4219-95c4-98f674f321be#card` | C | `18383f0a647a81ab047457d767ed0ca26093185511ef0f18829fbebb64a1348f` | 51 | 3 |
| Jeskai Shrinekeeper | `bb50b4f4-f0da-4283-821b-6d74bd20752f#card` | C | `a3c8e18d304d4c54070e64642f231aa8674d599d2ed71f0cd798e12a28592341` | 38 | 9 |
| Jhessian Thief | `b917c3aa-2d54-49e0-924a-dfd02eaf4b2d#card` | C | `4e4fdea56c55fe3c08de9be4e1fe99841f73f4a929dc73db69ae7782c35fb2d8` | 29 | 3 |
| Kami of False Hope | `49984248-4800-4f46-933a-fedd970ae910#card` | C | `45b2b91e1c3f3afd4b19c681bcee273603f741cbc41acb6e566aff8afbf25af0` | 32 | 28 |
| Kashi-Tribe Elite | `dac9bbfc-62e9-42a9-8ad6-2530ba47b990#card` | C | `13e36272ff46eeef91b563c06c990b3360bcb237265c26da02971c3eba38a11e` | 63 | 108 |
| Kashi-Tribe Reaver | `e602f06e-64ff-4d1b-813c-6612ee514ea6#card` | C | `04a36aa3282e4800aabba03968713a13507d07ce8232a56a02ee2e19811e289a` | 60 | 54 |
| Kashi-Tribe Warriors | `4920492e-c210-4d99-ace9-184d65e697f5#card` | C | `1141c96ccb392c40ff9b685ead7f8c33b9c910938723ef7131ae196d0591b057` | 44 | 54 |
| Kathari Bomber | `1ad39f85-799a-42ae-852c-9117d689fe65#card` | C | `0debb31c6fb4acb4ef49cfb3b66a45638fe359bf0d9ec497aa694ecff9e3afef` | 54 | 9 |
| Kaya's Guile | `a6becd5f-d279-4df8-81d9-deeb1b513c95#card` | G | `0bd2efb9a41b32daf13407a871c6862c55d37456695c187663f43698db403609` | 86 | 7 |
| Kederekt Leviathan | `edf459c0-53e4-47e8-8098-105656f9b49c#card` | G | `75493198dcee706638926e4c362f3e4a99bc866c43c6353eaaab1b702c2a8c0d` | 33 | 2 |
| Keeper of Tresserhorn | `ce976cb2-2b69-4d00-918c-d4900244ac86#card` | C+D | `7330ba5173ae4b5f0c6544974b29cb7e2a2c6e822341835f7c2c8f7c3cd51317` | 40 | 2 |
| Kels, Fight Fixer | `fe2e27a9-437e-4d22-aa22-982978793cff#card` | S | `431458f150f433f9b30593431a6caf6e2b7e04382062a04e7c3e82eb1d418cb2` | 72 | 2 |
| Killmonger, Ruthless Usurper | `aff21bd1-87b0-409c-bfad-08f3690ef220#card` | C+D | `015b3610605bfdda40539b9f364c1ebd338c95e00af2910f00ec2ac1d2742e02` | 86 | 144 |
| Knight-Captain of Eos | `611f1714-a8bb-4ce4-a810-4d26fe64e358#card` | C | `10415cfc4e1b5a6d909f0d50ec3e0a4df352fbd66e5811336b3ae09450a2a4f3` | 67 | 28 |
| Kogla, the Titan Ape | `8b21062e-97b0-4967-96df-30e8309f4fba#card` | D | `0bd232a3a4ef9916b6033762d16e426746f20d3eb6a2e8c09896573be1bd725f` | 97 | 16 |
| Komainu Battle Armor | `5e4d69de-1876-49a8-8ec6-a891d4a84ccf#card` | C | `5432308526e8867abd374819e41510fb4571b144ae1521a2bc150cc85aacaef8` | 62 | 6 |
| Kor Haven | `276cece9-f9f2-46e6-ae76-daddaa2fb9ab#card` | C | `0a15e3c75e986ebc520a4a01cce1507b9da3dc8a93df7a04f6a03de85225390a` | 55 | 58 |
| Kukemssa Pirates | `280148c5-3a2f-48f7-a147-4578f2068073#card` | C+D | `39e855f99fecb65780d95ba99a330b023e69c306d55a928744d772973c9ac38e` | 64 | 2 |
| Kukemssa Serpent | `49e3ac82-7c22-4dec-b61a-b581148c0419#card` | D | `225fe6fbc4c8c5fff33332f11661f171a4ae845353cc69de8b64c1daff03cba1` | 81 | 9 |
| Lady Evangela | `8800d672-424b-4a7b-886f-7eb9d7a56cfe#card` | C | `1db6c7fe2739a3902f2e408b413a53714549884dcda0f463c4f12989825fd3a5` | 39 | 58 |
| Larceny | `f356087e-e1d2-4c25-aa1d-215d09535e7c#card` | C | `05a4ddb03d40775277a35669b2a8d655d01f77eca1bd1ca9d04c7e3ec2130be1` | 34 | 3 |
| Lashwrithe | `066ba81e-b32d-4808-97ab-0b5f52d557e4#card` | S | `3b48e036ceb1ea69d66faddcfa7ea6b68606e40178554b50ad9100b2ba9ad882` | 35 | 2 |
| Latulla's Orders | `2f3c66db-efc1-4571-a4db-371a3d532615#card` | C+D | `8e5510fb715dab03d9265505b3d407ecebfc5fcbcc98966c7499e8c34a1d6969` | 44 | 3 |
| Lavacore Elemental | `70b6fde2-f759-4478-926c-b5601b7d903b#card` | C | `8238c13849132dc98839f20b18cd0837d712f1ffa87d64e7528af3795ed2b3d6` | 38 | 3 |
| Leeching Sliver | `f104b361-8000-4eda-b530-1502528e09f9#card` | D | `74c7c260cd66f989254bf874d06d9a675fb8d54f5d1ce080552f5e7fb55bd82b` | 26 | 1 |
| Leori, Sparktouched Hunter | `ff94e2d5-8ba1-4f6c-916d-58235a60743e#card` | C | `05f72842c4826ee17eaffe4b29698279c9b55845735a18c2a44c3247b13ec434` | 87 | 30 |
| Leyline Phantom | `af642b8d-a085-4a07-8a1d-ca385d4700f7#card` | C | `692e351cc58309886c2729389a1ea7352a7e724f7d7e0b49797c5287722d75ed` | 27 | 2 |
| Lightning Skelemental | `7c3f2b3d-1517-4818-9675-189949175623#card` | C | `1e1713b47a57cdb1bdf0ff6651e1d65584a34dcb03365ffb728d3b551f1f47d8` | 56 | 3 |
| Lightwielder Paladin | `ad62e9d6-61bc-485f-a9b8-2ef9ba98a4b0#card` | C | `0828c7cad4089317a22792c90db601b3114926a49b5c60767862a726c776a764` | 44 | 6 |
| Liliana's Reaver | `97332d4e-a55c-4e4f-9040-bf426a7c8e39#card` | C | `28bef3c9c164122089b7e2710ad40371d7d8b91ebfd663c5ae1ff37052638a27` | 55 | 6 |
| Llawan, Cephalid Empress | `55f9a9a2-2842-4555-8b54-03aebc8d6fb8#card` | G | `138d42581e8c7b17ffadc36f4856c81e2e72d91dbcb893dfb5bddb8488260fb3` | 52 | 4 |
| Loafing Giant | `038dce4c-f754-45ef-98a4-4e16f931a65c#card` | C | `2103b02f19eca5cae196e3bc8c0d9f174508045b4481f3a78b67bcebfec1cd23` | 56 | 12 |
| Loch Korrigan | `f281c47f-3587-4a3f-8726-ec59568fef94#card` | S | `719efca97db824d42ae9d7bffaa1a532315a8274dd2f9ae7d9a3a02400ee0090` | 25 | 2 |
| Long River Lurker | `c65c4d01-29da-47e6-993c-a265cf1f198c#card` | C | `004abb3a61c5993c0050555c4e5c7713541d6553adc74a643934e77c548e54b3` | 110 | 132 |
| Lull | `d53f4c24-8deb-485a-b03a-945211fdf10d#card` | C | `00cb10be5e6d3b8dcda415c79cecb61da1187b3965c3260d22c87be1c3aa0658` | 30 | 28 |
| Lurking Green Dragon | `262e1cf6-61c8-444d-a71b-050eebcaf932#card` | D | `11737e741e8ca88473f7bf352a9afeca38a53b9b75e0be148674c50713230f83` | 30 | 12 |
| Mage il-Vec | `adea7db6-634c-4c5d-be40-264b4acffc53#card` | R | `6b42fe585c92856f6f071142ed6ebbb933b9623f8c885a3b36a6378ec03b625f` | 30 | 1 |
| Mage-Ring Responder | `1d84fb81-2de5-4be1-9638-9ae28e3d867a#card` | D | `8917259cc03d55468f9ed08923851496e4eec6c6cd4a24d9c30add4ec2a2f6a0` | 63 | 6 |
| Malice | `b829de81-04cd-498e-9f31-074f23e2a488#face:1` | N | `542c5a137275c91f3f3151c28c05f7416fd3ef3886237388e08cf813ee280c64` | 27 | 1 |
| Malicious Affliction | `18f22cc4-e10b-4af0-9c1f-d7d15dbcce63#card` | N | `2b67d878f0b7c064f63f9aa92f04bfb412cd40459b39d928be62b7ecdfff5081` | 64 | 32 |
| Marchesa's Infiltrator | `9f05fab1-398a-4bd1-824d-6b1373dc9a94#card` | C | `a607737086eda3eb58f5c30e87409edd483a980e4eb42b18cc433ff0b0f6a3e6` | 29 | 3 |
| Marcus, Mutant Mayor | `c9db8f36-2c72-403e-b2b7-7cdfba5feaa4#card` | C | `07ad5ce02b17ac715a1e0380b00ac7cc5631c169ef86c98f577aa3c64d2eb50e` | 69 | 36 |
| Mark of Eviction | `d9193641-a7f4-40d3-ac62-15015acece4e#card` | G | `273d764a74b26a9a6b60286ef2ea8b6ca6b41db4aacdf835ab84929c0468fb9c` | 38 | 2 |
| Markov Blademaster | `774e474a-f2a1-4b62-8cfe-40ad509f9062#card` | C | `98bb9d417630ebd9af4f93c36ecd76d760c9898a432845565360197e28eb5f2b` | 31 | 3 |
| Mask of Memory | `d6b2c998-a226-426c-a40d-6e6007041bfe#card` | C | `42aaee8308153e87e5a9f07f16d176ec9b1257c586cd13c5a37d2642536783fc` | 54 | 3 |
| Mask of Riddles | `a50c55a7-4e7e-40f6-b76c-1e32598dc9b6#card` | C | `708772131fa7179d4db510eda1981ab591772e53bab8171a624f00e48a094da2` | 48 | 3 |
| Mass Calcify | `3ab3996f-aa3f-4041-8634-8e197d51f108#card` | N | `a03f50cc434a38b25683a25704e24ea94ae9d162aa836fa18ba7325b6b1b8c44` | 13 | 1 |
| Master of Diversion | `3402aa73-5e63-4158-b114-6f1d42ec8992#card` | D | `9cfc2d8d4e3e4f025cbf2dcedf30960736a8d6be7270e6d2bd0ba137e22f3b6e` | 24 | 1 |
| Matsu-Tribe Birdstalker | `4c90ca5c-551c-4da5-9482-62521b1ae17c#card` | C | `1af216cd0488eae073b34f1dc6dca16fd81e199d942cc53848904efa3dad380b` | 65 | 108 |
| Mephitic Ooze | `230e0bf3-3f17-4968-8723-3fb6a0909816#card` | C | `3549d095bf3d937129d6992d3dc526a18ffc44c6777057222a647c3d93be8252` | 67 | 6 |
| Mercadia's Downfall | `2c1201b2-5036-4e68-b36d-cdaf54b4993d#card` | D | `3cbf5eaa178aff9adb042df9dcdbf449af956cfb1c77457e7a503bf2afd5b440` | 36 | 6 |
| Merchant Ship | `69556f6c-c05b-4902-bac7-012f0ed81b75#card` | D | `5278f4fc3b4f942515ab0345caf127603f7f626a4c63fda130ba822ae46b36e9` | 69 | 3 |
| Mercurial Kite | `92f2a27d-3ec6-445c-9962-96b099f793ee#card` | C | `b0d16d077db06f48acac257e17263a01a54f5a2261e625b603e551bbb5c8a37c` | 50 | 18 |
| Merrow Grimeblotter | `4e9a63d2-b48b-4740-95ba-7966ada95baf#card` | S | `10e012b11be49395721de5b3dc2bf70ca5d29f167c7cce03a07d7e32e3edfb24` | 30 | 2 |
| Meteor Storm | `03f96c23-de0b-4b85-81f8-febc850aa621#card` | R | `5cb25bc78267f2662641b872abedc415d152c7956af9df10e3e04af0a0e91e3e` | 33 | 1 |
| Michelangelo, Improviser | `1939679b-ea2e-4a75-b3ce-d7af68437484#card` | C | `491ae95bfe4d52d61e2c55eb38f290b73e7c0535d8a73ad81aab65cd8085e6d0` | 53 | 30 |
| Midnight Banshee | `dd8b6067-a97f-402f-bba1-70ebc6ee1599#card` | N | `57b5b198067b234a77f9d02a46769adebea30e17760bf36a0983d8fafe465885` | 29 | 1 |
| Mind Knives | `f9be5566-c3df-44cc-9de4-29510c8c245f#card` | R | `ab7d2573cdcf0b5740df4d2177e393cc03a36b0a712ca90f1ae8869f9a7c0101` | 16 | 2 |
| Mind Shatter | `26809033-a823-4251-887e-0410a545f993#card` | R | `3fd6a182cfe7ee99dddcb72f1e5d853094ed4ef75b9be475163a3a10f813d103` | 17 | 2 |
| Mind Twist | `78f9c223-9982-4282-a496-a6f892f0a5bf#card` | R | `3fd6a182cfe7ee99dddcb72f1e5d853094ed4ef75b9be475163a3a10f813d103` | 17 | 2 |
| Mindscour Dragon | `c5d326a9-c047-4f77-9f0c-b30daf268f15#card` | C | `5ffd1dc7ffa6f09e6e2064531631ccc93d9a976b6c2cfaccd6da6b6066210aa9` | 33 | 3 |
| Mindstab Thrull | `43abe1f3-7b92-411a-bf93-e158f949685c#card` | D | `fee2e9cb55ff87c7a70cfaf5ea55fc7a3393aa686607eeb9e76ef8a8dfbcc88c` | 48 | 1 |
| Mindwrack Liege | `c24b3135-5f23-461c-9e57-de747ca8978a#card` | S | `1e1f65e3ed71e2a240b6b059c62cefe32e2d824f7fee0f6522fe467a04d949ff` | 87 | 27 |
| Minotaur Explorer | `22374ec2-61b3-46eb-82ed-93bbb42403c4#card` | R | `04dad4710c942404528c5b3f3cfb431a6d6b8350ad50f96b2b648bdd3b05b3b4` | 28 | 9 |
| Mirri the Cursed | `19d6a20b-4641-4c79-ae70-ed1ad5085472#card` | C | `38ebf9faa1043123eff13581caae183436f47532035c036b11f3487b6f0fdfee` | 36 | 12 |
| Mirror Sheen | `d6494704-54b2-4c80-acb4-184c9815c5f3#card` | S | `03abbd4918032c4d428c35335716456a5f0705d3335af63e9e55077e2df52f02` | 47 | 25 |
| Mischievous Catgeist | `4d0a0027-53b3-45a1-8736-f0ac86b19342#face:0` | C | `a993031ead306dd431c2cf35edc2561efbfd6960e62f8a619a76303a4f3bd8d4` | 32 | 3 |
| Misfortune Teller | `bd1ccc6f-d281-413e-889e-1e022c3f2a99#card` | C | `178566e0710684c35bc859dfab6b12ad7e651668252bc6ece5a14886cdb09bdd` | 105 | 20 |
| Mistblade Shinobi | `16856e22-6f9e-4f8e-b4ee-622a7c2ee453#card` | C | `76fa70e36e79fe9a5d2520aebf9dc3a8cb4293dc7e5c7457698de4fc9d295f5d` | 47 | 6 |
| Moltensteel Dragon | `dc2d67bb-29b3-43d7-8ebf-2100c5287bbf#card` | S | `773e8bd4efd80ef6c53e465a325e271d881e5a8a3e29baa367723fbdfd2b9fd3` | 28 | 2 |
| Moment's Peace | `b6d22228-a45e-4296-9ae2-1649e04b1c53#card` | C | `01c4ab40cd3b3e4709c04e69d5eca74110f74fbe42ef0f9bd96d4e487ba9550b` | 31 | 28 |
| Mondo Gecko | `fa3ae120-756d-4c52-91f9-235767539d67#card` | C | `025b6608b6e24b292ea5c637dd30ffd826bd49b7ab9c8aeb0bf1a8096d7cdc5d` | 85 | 132 |
| Moon-Circuit Hacker | `7cd8b017-8d59-4b55-bb9a-8d8e492ef1ac#card` | C | `0237eb4d39722bebf1bc4ea52eb95cfc4f155cac3f279d8742d429b74ca42aff` | 63 | 21 |
| Moonblade Shinobi | `046e25ee-c96d-4cff-93be-f7e3379d713c#card` | C | `01123af745e5596cf3a1812ee2ac4a6cf1c8276c081b21391f96a8ec4cbab46a` | 47 | 24 |
| Moria Marauder | `9f0027e3-815b-4965-9ef4-3c4814c62cf3#card` | C | `178b80685dd321657043a42267c37b40c3d2359e5bfe5f93f11cea5ec1bf7d35` | 60 | 24 |
| Morningtide's Light | `a25bcf65-4417-45e9-a4b1-94e0ea78af63#card` | G | `09e80bd757164a960b18db08c203396fcb94dd16d52368a6f840cf87a0b669b7` | 88 | 84 |
| Mortal Obstinacy | `ea29c489-bd04-4f36-8b59-bccaac153152#card` | C | `9476bb2debed81c95b7244d9e19f1420621269064dd201af2655f54d7f613b5a` | 73 | 3 |
| Mtenda Lion | `af029853-cfb0-403b-af51-141ba02ae2e4#card` | C+D | `059c3d6d25df0068edff5c8e12aa0afbaac96ff18f99b26bf32076d7a1aeb1aa` | 60 | 58 |
| Murderous Spoils | `e911f9d9-da04-4997-a8d7-449b566cd1ce#card` | N | `f1e4a07c8e98c39bf8ac797185681b3818b545cf18345b1f7b84bc6c8998bb82` | 51 | 5 |
| Mystery Key | `6a1e0dfd-2dd1-4372-966e-e28b48ae0115#card` | C | `cce5f077598c3ab720d4b3741b6c93d4bfc3e994b5da25798628925ee94442b9` | 49 | 3 |
| Mystic Gate | `e9f5feb2-2c1a-46ce-885a-4f378d7d10af#card` | S | `28100bd0e539321c29b483523a0984437847652a56fe5fc0c34450528ba9d453` | 41 | 1 |
| Nautiloid Ship | `613a8774-165e-4cf6-ad43-124f9ffc9980#card` | C | `049e1f902a86323141577f3e562acca8b847edd0bef072a38404f8cd1771e13a` | 75 | 75 |
| Nazahn, Revered Bladesmith | `a3a6c024-43de-4f5c-a1d6-c48a0c96a43b#card` | D | `8b371205bc76d0226ac5b25bc66975c4f8c46b1ac44338acec386cfcc763655a` | 111 | 24 |
| Necrite | `e6ab8a04-e225-4ff0-bb8b-c80d0e6ce50c#card` | D | `e966d419c137f7029ddd2562c9f07205e20c5e90cbacafd31339f79130e190ce` | 64 | 1 |
| Necromantic Thirst | `494895db-3da1-4a19-b7a5-ea47817f4e4f#card` | C | `4a5ec3bbf98a7f15f4d201e1f7007ed415622b03a420e23f5eb0f55c0acf4c9c` | 43 | 15 |
| Nefarox, Overlord of Grixis | `0632f0f9-8d4f-4e94-8b53-1275b7f21a57#card` | D | `1e342f890368545d6b5873429971e37dc4277e70239b06fbe7b70a3169c1bb57` | 34 | 1 |
| Nemesis of Reason | `b1d70845-869b-45f7-b4bf-1549bac51d58#card` | D | `bb6850b41934db282820aa3cc3cefa5600f68c0c4b8eaa4057f6e23a5c550c23` | 22 | 1 |
| Neurok Commando | `48936751-429b-4c40-88f5-72acde6278cc#card` | C | `5683c1233444fba59dd9b3cb0f16c328622b4415d52543eb86ab1e3c3df9e82d` | 34 | 3 |
| Nine-Tail White Fox | `51f4eb1f-abf2-425a-b78e-cafac177da89#card` | C | `78f48467ebaf88c5900cb922a91ee7468f76993ea9689308dda1fb570fab5241` | 26 | 3 |
| Ninja of the Deep Hours | `1f3c2b00-0000-4ae1-9650-9553accac52e#card` | C | `03da8f7e3cae4dc9c0844d1f865ea7caf72f9817af5c9c4c3d8b15ea2c9c3701` | 37 | 3 |
| Notorious Assassin | `455138dd-4f76-4b40-9715-f1b1d61f8f23#card` | N | `c7aff31b24daef720608bca4ffac6540f81ac377b41c90871d0e208d6ab2e73c` | 43 | 1 |
| O-Kagachi, Vengeful Kami | `300715a0-4f95-4212-9e13-558434c9d1a4#card` | C | `ddb29c94fea1966288f1063f74a42cad3f73455391d2ef1751249b4df2837c82` | 55 | 6 |
| Obscura Initiate | `bdc3a563-86e0-4457-b00b-85c03e62a930#card` | S | `9fb63a7b9729f97cc20964ff8f44c1df0b7da393aa15b88a62a61d36aa1a21a5` | 25 | 2 |
| Odious Trow | `0e9864e2-1bb4-4527-b2af-62433a90b669#card` | S | `5c624a22b20645b59adb131cd73666393892fc0ecc3e323739cb76a7945d4c81` | 16 | 1 |
| Odious Witch | `9ad999fe-889a-426a-86b3-c0c311b82041#face:1` | D | `660951aaa9ba86812753d8ffeed5eca0b91924d4a3bf5b877bd9dae208b41a1d` | 30 | 2 |
| Ogre Marauder | `d9dafa31-41ea-4f1c-bc63-ec84b01d57fa#card` | D | `127fad87b5b139a288ab4e0db76e504d8455e5e29a50366f71a51bed8233ee16` | 50 | 6 |
| Ogre Shaman | `04c7bb20-6e40-4e79-a0ec-ced920d3491e#card` | R | `9655aa77cb80bf0ab4221bbc26409d8091a9098f07fb8a31b7a10dc81c45f5b4` | 30 | 1 |
| Ohran Frostfang | `b99ada26-9a61-4175-9fb8-15a106960220#card` | C | `2814f894742b6cfd7cd07b85dcd861a4b070963f9873dc636dfe32c734753cd0` | 50 | 6 |
| Oketra's Avenger | `8f064160-3afe-408a-85b4-b335eae8571c#card` | C | `0cf0825d515142a75132a488c890fbb6baf68b330def6c211cef4e0a3c990f02` | 55 | 93 |
| Okiba-Gang Shinobi | `efe3b58f-6f83-45d5-beb1-7dde1cae9dd7#card` | C | `da3b51159e2195c74ca062c83c0eed39f8ae50713ec3de4529c5ec2536994a5d` | 36 | 3 |
| Omen of Fire | `5935e9b3-b9b9-413c-ba17-103f5792131a#card` | G | `02cdad58d2baea35eb1c889d88ff52252a71775aee62a98c504dfdf488f3cedf` | 50 | 54 |
| Oona's Blackguard | `b72cedc2-73b3-4e7f-8b58-8db532a69f6e#card` | C | `052c33f9da75b03b5bc2ea13ffb67da0ce7878a19f0ee9d06152122810c970c8` | 77 | 315 |
| Open the Vaults | `1e9c473e-bd65-4e1f-b2ab-cac58dc581c9#card` | G | `a382cbc4fe1a3e4a260f152b41f5fbb4fd5408e3417736f23dec5ffe0e68f1e5` | 26 | 18 |
| Ophidian | `953bdefa-4639-4524-af5c-5055d1289f40#card` | C | `d0f7b1c3d356f5c18aea555b28eb48bf9217eca452b3194b5b115497ee48f4c1` | 53 | 1 |
| Optimus Prime, Autobot Leader | `199c4a1a-2082-417c-a174-f479d7334d15#face:1` | C | `2522bb04c0bc2ff9d4fd2974fc2ec2c9dd0a2c91b789e3baf266e929d5aad1a1` | 66 | 20 |
| Orcrist, Goblin-cleaver | `cc65821a-1893-4087-b461-35cf9fd26c71#card` | C | `3b5c026ed8626507e6d82996ab0e4ab20887447c7f40b59c1832c9975131f0c9` | 78 | 33 |
| Orochi Ranger | `6119fe8e-e7e2-4cc1-a44d-fe95a230db80#card` | C | `1141c96ccb392c40ff9b685ead7f8c33b9c910938723ef7131ae196d0591b057` | 44 | 54 |
| Oroku Saki, Shredder Rising | `d14f620c-0fbd-485c-9fe4-34949e742147#card` | C | `91e614bfc0ed4bc95b36ceb7b7fa332df5222777297a2361b02e1d4f603400de` | 39 | 9 |
| Orzhov Locket | `8aaf248e-66cf-447b-b062-9aa2ab1ec864#card` | S | `83845f50afa62d7e747fd98372ed4333c04e442c932730bf92258a7af4a70b35` | 47 | 1 |
| Orzhov Racketeers | `4f6c9cee-5af7-4b00-a2a4-2770dab0a2dc#card` | C | `59f8e4ed995f696d1cea07aa3f477d91f326a28a3e0f36ab558ca006ec8fbb03` | 33 | 3 |
| Pack Leader | `3701ed34-a97c-4d7b-a15a-4faec02ef24b#card` | C | `04b1d7f677387e3ebd381e134f8db9e0c9651b9b90b3a3bcc27dae86a8208ff3` | 65 | 296 |
| Parapet Watchers | `c2ffce33-ab1a-40f1-959b-31583219ed1a#card` | S | `86054e9032000006f610822ec69ccab19aab491c380f32fe0b19aeede657ceec` | 25 | 2 |
| Pardic Lancer | `1bb2b090-19a4-40d9-823a-671f6eaca9f9#card` | R | `14685dad224a4022a4f1483f4aaea759cc5d82a566ac6c70adc0fb5d6c0d04df` | 33 | 3 |
| Pardic Swordsmith | `946d770f-0a59-415b-b334-8f7353b96046#card` | R | `db7b015f07b96507e56d80cef655adb2b1a181edd805c043fceb8260ba998eda` | 34 | 2 |
| Pause for Reflection | `a7f266b5-7258-4eaa-b447-66460ec38505#card` | C | `17dbd574ed91173afc99d05c5799717dcd6d54360d94f90c4005f4c0d054683a` | 28 | 28 |
| Peel from Reality | `65dc05d0-f885-4fe5-9a23-d6b75185c179#card` | G | `17fb426008d6690a5025b6e2b08e23047631ff74aef31defe7d42f2dd6d10bfc` | 30 | 2 |
| Permeating Mass | `5e719329-612e-4ad5-ac95-6eb5328738ab#card` | C | `b40e911e13141a26d21250e5b2df4b042a983987c22ee0fce51319f71678351f` | 35 | 3 |
| Persistent Nightmare | `99d42ae8-8cee-449b-812b-98a81a593077#face:1` | C | `60a2231f35f3ba4c42cf666127a1624814e06eda843e56b81e32a215bc3c1980` | 33 | 6 |
| Pestilent Souleater | `c4779671-2a01-4eef-847c-ede36091fc4a#card` | S | `538f090f946fd67def039b5b4b5eba29a118ea0d571d12e64e87f9cac86a37c4` | 21 | 2 |
| Phage the Untouchable | `09d01406-b529-4c3b-900e-d8c889a4c400#card` | C | `0e81480396854d23827f768d511594f341ee2cdeed341a5d744a7f86984fd3c0` | 105 | 18 |
| Phyrexian Scriptures | `11173ad3-c007-478f-bce0-d756eac07ccb#card` | G | `2a5ac3eb6cb7a621f6696d21f372be3e40935364f79dcfb75891cf553bb6fbcf` | 369 | 4 |
| Pillaging Horde | `ad07cb55-cc4f-40be-b16a-bd3d3ca94249#card` | R | `04dad4710c942404528c5b3f3cfb431a6d6b8350ad50f96b2b648bdd3b05b3b4` | 28 | 9 |
| Pirate Ship | `c6b3f924-806d-47d3-b044-72b48470196c#card` | D | `3d5db6aab855fd47e02bf82e144413f109c9ac2f541dfdb7c0282d7d12ae1e9b` | 65 | 3 |
| Plague Spores | `f79f5cb7-d238-41ab-85f0-eb21cc6bf561#card` | N | `c47768f248558f90dc482805180410e5aad82d7f2ce170b0a873fd50e680f361` | 30 | 1 |
| Planar Birth | `7a9d13b3-3823-463a-acd9-8b7a9d5e121f#card` | G | `7894b40d526d73f5e20228a58f945e45ba348102a4bf10d8cbc0d409b49e6c1f` | 28 | 38 |
| Planar Guide | `75701981-b9fc-471f-a97f-e897bb3dfb77#card` | G | `1d6ce2accf31ea48676741017d27fcd213e5af2c8b3db5b2199692c6e245ac6b` | 52 | 9 |
| Planar Overlay | `a7be1624-04cc-4ed5-8b05-63b633893dee#card` | G | `549eceafdb995ab2b7ca91316ae0fff683ae61335f714a9198da12b4c878c423` | 42 | 2 |
| Plant Tadpoles | `299ffd3a-29cf-4667-b191-dd30d09f799c#face:1` | G | `42fca400b05c33bd0df02084687f94f53aa2663a627126530afee4be6c4b942f` | 33 | 6 |
| Poison Arrow | `6d920c99-8a85-4b20-97c2-7aa0c79ae70a#card` | N | `97309140adbdc2d8e0b91f0f2b8e8aad3cf37b402e7b820e376f852e00232109` | 24 | 1 |
| Pollen Lullaby | `1f64d70d-ea38-4419-be91-8b68aab3401e#card` | C | `0ca4dfbcd25f1a35c447d3ba8c3a79b8e814817912e67863cf8392ea51813d7c` | 67 | 672 |
| Precinct Captain | `3c75234b-e367-46da-8454-be4315f9b9de#card` | C | `ddd36b5af3ef214bfc2356aaa43ebcb81c3ec16fcd3bf6610ebf57d1746d5133` | 41 | 3 |
| Prophetic Flamespeaker | `b290a59a-5752-46bb-90be-9792b57bd467#card` | C | `35e1be312931bfd9155dc00b5339d1344a23d9be116955ae12acf48b194c9f27` | 54 | 12 |
| Prying Blade | `5fe86083-1009-4de2-b186-f817ad1cc4e4#card` | C | `dec83b3d7a261ffab4529f9bfbcfcc98db79d259bba73b15596131c782be5d87` | 49 | 3 |
| Psychic Frog | `e157ad1e-be35-47f3-92db-01799f8fb6a5#card` | C | `66c998cd0c31a99b2af8a556b04720ae8f003e956a1a8eff7900a42da30037cc` | 79 | 12 |
| Puresight Merrow | `012c665c-24b1-49d6-b575-2b4ee1c3a833#card` | S | `0d554198b513c808842bbb5136a52e09574b3d7fa015c4343904f2ba8f2e0eae` | 40 | 2 |
| Pus Kami | `9b2471d4-2821-4220-becc-94bebfdc456f#card` | N | `9b99df48f91e955748439bb43635192a487fe71f7bfd65a43f5abda22c5edbb2` | 28 | 1 |
| Pyromania | `2c89cf19-8dc6-422e-bbf8-4dd5a399c630#card` | R | `d14410b1af5ef55f0a0e0885358a393fe950d26e8e27d9e97f2dc0737eebb2b1` | 59 | 1 |
| Queen of Ice | `f5511802-56fe-4922-83d2-ae723e1beab7#face:0` | C | `284341ccfff824cfc48a340211671a9e7228e21a46a4f04b13a2741d07823370` | 46 | 18 |
| Quicksilver Geyser | `6ee7b00b-b875-48e2-92f2-889c154b0f82#card` | G | `131884cff47b2349aec08c4a9991264c4af2d207e9ece8aef3d5eb9b157511eb` | 22 | 2 |
| Radiant Kavu | `ab46b957-5206-4390-a2f6-aba2e8d1debe#card` | C | `1154465ee823a2b97bc84afc84aaded36f255eceadb8127f2b3369d25542d7ce` | 35 | 18 |
| Ragavan, Nimble Pilferer | `37108cd4-bbab-4ce3-9ed6-f60e8422e703#card` | C | `099d41c96eb27ddd3ff14d20951312de55f450fb3c3ac60f5ba037fcdb1e70f0` | 67 | 36 |
| Raiders' Spoils | `45fd14b9-d66a-452d-add4-36f512fff9c2#card` | C | `a53db383ff470a2bf2d6adde3bb8cb6b5d50e20d4796ed5e39fe07cba25acddf` | 74 | 3 |
| Raise the Palisade | `f55a3781-fe33-4301-9bb5-6a54b9c13c4f#card` | G | `3baea094841e586f084805006a417ab07794fa0357f225e13870c135fd1e7f0e` | 37 | 6 |
| Rakdos Locket | `ab784412-0722-4501-9aa0-fd68d307f6fd#card` | S | `e275190dec06e8087702ca5c987262aecc6486c642f2376603be5117b9bbb938` | 47 | 1 |
| Rakdos Ringleader | `f8ce8573-7c87-4f47-a0d4-14bd335f2c1d#card` | C+R | `2fc5d44deb73497517ba7a211a66dbd860b54534a1bf17c6e9d559f2e40af9dd` | 49 | 9 |
| Rakish Heir | `3318c391-7aa1-4550-88a0-b29d8dca28b7#card` | C | `77d0cba3e75ed42e65d72755a389330a5b6b2db9a0d1b955f536c96672f7eb5d` | 33 | 3 |
| Rakshasa Debaser | `db286eef-f0df-4f7c-8e06-957447166950#card` | D | `12a936da5a3c9a6417259e95fd66c54aac63eaf3c15722af0bb7ffb1ace3a8b1` | 42 | 16 |
| Rankle, Master of Pranks | `b8619990-9dc2-4fcc-bc7e-457b77cd2a8e#card` | C | `3d294df0b0907c51c3f1c2934c2370bd364a4457e3c4a914ffb7a46c1d19ca4f` | 82 | 3 |
| Ray Fillet, Wave Warrior | `206913aa-4d73-4465-8311-8054a22c0743#card` | C | `029453d7cfc11ecf62fdd9f2104f0aac1177713902095f35133beaffd4a7d591` | 46 | 15 |
| Read the Tides | `e5c4ed40-5069-4606-81f2-59dcfa1345bd#card` | G | `6799bf7e9733dd1634b1484459b350750e92eef17bf70116e3ae6fe50428f567` | 40 | 2 |
| Reaper's Talisman | `bc2ea8c8-5360-4256-91c5-d821b5e59c0f#card` | D | `2e498c8492add762998fc1fb47b08cea4c4bdd51931531611749842ce940e169` | 60 | 6 |
| Rebuild | `496999a7-a82a-45ea-9c4e-1b6aabc781cd#card` | G | `5400407968c8424ab4926086637a12237b73a121fb1fba9eda21c189ce5dce2c` | 21 | 2 |
| Reckless Spite | `a684df3a-5441-4daa-86d1-c47a91b35e6a#card` | N | `ae82c41e499a56273c58d8a6ca42c8022ad67bf3afe7cc19a8f18aa04dc5e25c` | 26 | 1 |
| Reconnaissance Mission | `11bf211c-95a3-4e7f-bd08-13f979d54e73#card` | C | `fa09bc7632a4c7182b84b724805d2c4e535d596037d592f364dd3f2e7bb05041` | 41 | 3 |
| Red Cliffs Armada | `063884e0-1f5e-4be9-930b-e73895b2fa41#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Redcap Melee | `1b78aa71-b554-45dc-a737-52cd74c3e7f1#card` | N | `0472355c266e609deae5773f6bd66c2221390df3d27d6c56b20758794aa64f9d` | 48 | 2 |
| Reduce to Dreams | `85bac6ad-b63a-4068-a813-ad685966e171#card` | G | `56d0d91609fd3a6a6c43e178608d8b021c0fae7b769fd4d2925537a2a182f2f2` | 18 | 4 |
| Research Thief | `ba5055ce-6809-47a4-bee6-fb97a5b60f8a#card` | C | `b6fab7b4a786b40025803fdb2a5de5fe58ea4a2d8158e1bb49987d496cd9b644` | 39 | 6 |
| Resistance Fighter | `a95a2d1c-70a9-4eb2-ae30-a07fab26a9c6#card` | C | `34598452720c35f02e46e287a38e2e321e743dfbe172c2c10f40c3598f663a8d` | 30 | 6 |
| Resolute Rider | `2f8b2bca-991c-4dee-829b-c1cc7deaadfb#card` | S | `07959dd0eaddc759ceddb40cde9e61853181bc67bb296fc31454686e3594e872` | 45 | 4 |
| Resounding Scream | `287d5a5d-e477-4d5b-815a-d83c73f4792d#card` | R | `07a085b887d31f034f8566b8d8b78f26bb41378f696bbb48456c0b2bfe797d85` | 50 | 6 |
| Resounding Wave | `7e607fc7-ec22-4249-8936-70a026f9996d#card` | G | `3ace460f38bf8fbd64ba5f0bf67b7a885009bef3181f9907749b8825bd0343bb` | 51 | 4 |
| Respite | `46137f00-f7f5-430a-857c-ff2dbf2d2579#card` | C | `02d51f2ddfdb29246981ea781fb20224badd014b074c25ecb55bc61ad1f82f09` | 43 | 56 |
| Restless Apparition | `fbd6dda7-ccee-4640-805d-78a9d60f45ef#card` | S | `454ea833b8bc4c8de3f041c5d41bdfc655ec1056e5d15dae2b589c3f34d35d42` | 30 | 2 |
| Restrain | `5aa66cb0-86b9-4e85-9a17-df78416d682d#card` | C | `0883f2bf6f6220af70ce468b9d639b37ce1351c474bda646eb29933496f4e7b3` | 43 | 58 |
| Riftburst Hellion | `e121f9cb-ed25-4be1-9822-b68cce2f2623#card` | S | `ee303b5c6f9e53daa7396aaebca86f918dad304575cefdd634ae05479f8b3b4d` | 10 | 1 |
| Rimescale Dragon | `3f8182e5-4df5-4beb-a3b7-387ed1e86c99#card` | G | `366f5fbde432f43de295f5d62d0a4250cfa225591a8677c464918a56555a61b3` | 56 | 24 |
| Riptide Entrancer | `d0bf59a2-347c-4c3e-85cf-b27a2af77192#card` | C | `416d913c85bfb012aff414fba4f434ba57759b5d68288600dcd5bbac0a1923fb` | 64 | 6 |
| Riptide Pilferer | `d3e40d26-1687-4cb4-9ff8-4c4421f09e53#card` | C | `a3f96286beb9f7e7bfb8ee5d719d0841e6d1d806b42419d5041c944b5259c0f2` | 34 | 3 |
| Rise | `814f06aa-b48e-4c7f-b4cd-595d89176d0e#face:0` | G | `5f36b7c403965cd70fa3b214b4b554f0fb86d94251b01198a615739a9d1d87c6` | 29 | 15 |
| Rising Waters | `d4cbf380-1a05-48ea-9187-de931b00866d#card` | G | `00eecf0cc4e0373898224769ff2782cd70df949e9660c58c57a03384aaae0384` | 52 | 12 |
| Rite of Undoing | `9dbfa026-e364-4111-a03a-e9b1693bc7b7#card` | G | `03876237ee33dfab89561cb21499c82885dce8d7dbb8a2b32f728d94f0bc0ab5` | 37 | 8 |
| Rites of Initiation | `77aa591d-efb5-48b5-98aa-5c8912ede11b#card` | R | `17d0955a88cc28e9444fa3efe16940afa2bcf36717cd2ad4e1517fec013eff51` | 52 | 8 |
| Riveteers Initiate | `c756d786-9aff-4d2e-9c06-6ae01d5fca2e#card` | S | `f15df1bc4d6324ece4f22d7d2023bc555b68bb339bb05578a917f575ee7ecc03` | 22 | 2 |
| Roadkill Rodney | `cd8761ef-b57b-456b-aa12-374ac825087e#card` | C | `20972713c8675db73a1a3edf77b210b6fe341d4f0f3e341caba53872ba587580` | 36 | 3 |
| Rogue's Gloves | `308d7868-49f9-47a3-a7b1-4b0332d610f1#card` | C | `f595574218c91703d5d2a61c65477da1b4102a631bd33d0803c0ae3f4e86ee46` | 36 | 3 |
| Roiling Waters | `77fef82d-e8d0-444b-8d76-68e9b533f38b#card` | G | `08a8ac35709320af6267da15f9c98d99bbeaecb0bd204e1f603561fca03dfe87` | 39 | 4 |
| Ronom Serpent | `ff35e480-8ea2-47bb-bb2a-24cefe9c2139#card` | D | `2dae390c907b3a5f4d46fa66d4ed666d162cf1164d811ab9b374c18ec99ef9e9` | 48 | 3 |
| Rooftop Saboteurs | `462fa2a4-651b-4fc1-b73b-1bd4508c8f75#face:1` | C | `f2789b5b224fdeff3cf8a01c9e9c9f698d3dc30c7563fdf21ac288fa76480769` | 31 | 3 |
| Root Cage | `14c3a43c-cdb1-45d1-8eba-8e6d16bd7643#card` | G | `02ea31cb839b28d99607111288073d8dcbad9feee19977463b3ca237bb591672` | 21 | 6 |
| Root Snare | `cbf743a5-123f-4747-826d-7f8bb929a52c#card` | C | `08f4c88b0b9b363cddd4375ea8afe786e30b39b7c3e038888daf52ca42a4a053` | 25 | 28 |
| Rugged Prairie | `8e7641e1-e814-4d5a-9cb3-71ad2f4ceee8#card` | S | `13856102caf916a9e3d90b43b55690eb7597a70094524b966df9684ffd433646` | 41 | 1 |
| Rune-Cervin Rider | `5c6956ad-8031-4708-82ed-75491755d2a4#card` | S | `2fcf731180cb2b705183dee2a53fe1443d0afcb34a32b1040180fcad984ca163` | 29 | 2 |
| Rustmouth Ogre | `0a64b683-7017-44cb-923f-b512fe3886f6#card` | C | `e8fdf8e9d3f2ed1a5290c1a54540ed0599e8054ba6b495d656d4ba8449a35f03` | 37 | 3 |
| Rysorian Badger | `cf9fc317-3184-4999-84ef-945a8842096e#card` | C+D | `027ecc34787552ef8e71b545b0d3e0b46d709a465d0f950e0c4d4a0f549714da` | 85 | 49 |
| Safeguard | `e310c3ab-a729-404d-944f-9b477258495c#card` | C | `0c696bef1341961331954289a5141dac9e978f718b6ad226c2e04ceaed9b1d62` | 35 | 58 |
| Saltblast | `a6a9776b-87f4-4a08-8074-d49fbbe417ae#card` | N | `fa870811406d90f6c6a2400b6de810cf8e5cc75f345c907b9ea572637c4aa35e` | 13 | 1 |
| Samut, Vizier of Naktamun | `178b3fdf-dbee-4245-a811-b21ff8a24f66#card` | C | `1dda01c362d8009cb3a0734b1a55acfdcc6be72f688de2d81501d096a5382ac6` | 48 | 6 |
| Sanity Gnawers | `e82308d5-29ee-48eb-9d1c-f24897ef191c#card` | R | `40de533f4e609333ba47de0acf35c7dd71175a920323c85ce02c156d78339526` | 23 | 3 |
| Saprazzan Bailiff | `1f861517-ac53-4218-b62e-35e971098b6d#card` | G | `0f96ce90e1808193be0b22c3138a769d23abae95644f959a9ba4fa64412145be` | 60 | 25 |
| Scab-Clan Giant | `1c2c2329-fdae-4894-a43f-41fd91cfd2a8#card` | R | `0ae6dc181075c31ba85f3f0208709c3a05a00ea5fc4f557f3ee50b03dedd0f45` | 30 | 4 |
| Scarab of the Unseen | `5f085af7-ce66-4019-9516-d1650ee7b6e0#card` | G | `0c1ca11c530f0d8a397d976610e19818b77506c283db9cad9559726ca48a6673` | 60 | 16 |
| Scarlet Spider, Kaine | `4ec8a5ea-b750-4711-8843-2bc98f4ce060#card` | S | `3b79c7bdd608a1b1b81242daedf306402bf6a6bbeb866ceae44a1fdd7f9a9efc` | 50 | 2 |
| Scion of Calamity | `75fddffb-68f4-4b65-bf3b-4e2ada62a522#card` | C | `4741398bad5ecc8c401b409cb1c643a4fc123d8a9f6dc6c3929300fa3ce4ff53` | 37 | 6 |
| Scion of Darkness | `bd940cab-d23a-44bf-ab96-8aa1dcd3f0ff#card` | C | `04e41bf1f7ca20ee20b248f15c015441209b820671c66bd7a4a29d55e56bc026` | 56 | 60 |
| Scream Puff | `b46ddf4b-7ed8-49c4-8828-f0fbbc023c1f#card` | C | `ddb1aa81f0f59349c6f3db2a269c2792e341392eed8f6258137417bc92aa1bba` | 31 | 3 |
| Scroll Thief | `637c5583-4683-4ae4-8b4e-f5da42a772c7#card` | C | `78f48467ebaf88c5900cb922a91ee7468f76993ea9689308dda1fb570fab5241` | 26 | 3 |
| Scurry of Squirrels | `3beed054-a099-4887-b1bd-39cf9e514e94#card` | C | `293d57e5b0553d38953acd9f26551dad79abdf703664a73244cd3a80b315ff14` | 39 | 3 |
| Sea God's Revenge | `8ab71a49-ce81-44f7-b940-50e7aba64df9#card` | G | `7333bd11eb16ed8d18a110ad4860405f2efebee278cc4558abc90f618ed88726` | 34 | 4 |
| Sea God's Scorn | `d2497a5e-ff2b-4024-9b14-634304d583fe#card` | G | `1429961926fc6e36b7bd27d9ed3b99707b01906498170b493cf2159d46fe7669` | 22 | 4 |
| Sea Monster | `21b07ce7-b4f9-438c-8c09-e624557d62d2#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Sea Serpent | `c16495fc-784d-4bac-9a68-ed437008df73#card` | D | `10b2efb4f9d744f6c2690297bbcb22c612bdc8b2d7c3de68b2e6c0e9579371bf` | 44 | 3 |
| Sea-Dasher Octopus | `0b718d21-6873-4613-ad15-660d30527fcd#card` | C | `50a6684366b4dd0174e0d504038c0085e5ec19a89548c0b36fc1293d5aa88184` | 35 | 3 |
| Seafaring Werewolf | `da412474-1da8-409b-bd7b-87cbfc239fb0#face:1` | C | `eb07901d0097cb35765552eff141728783db42d2d0cd7eab5e3a32f35ff07ae9` | 46 | 3 |
| Seafloor Oracle | `f9f719cb-7778-4109-bbd3-4504ee1b5f49#card` | C | `a271ef497becae36efd72f86a6f5e347f1bfb4b0f217ea9d095cc87a7e9701b9` | 31 | 3 |
| Seal of Doom | `71d54ba3-91fe-4a14-8b99-c284b4c75013#card` | N | `cc7cc216f2f987979ee757522acc8086946b491a8e6004eb0e389be8b67d7cd6` | 34 | 1 |
| Sedraxis Specter | `eccc1230-df5a-44a0-859b-9e1e0d176dff#card` | C | `507119fff6094ad4beaf03bb76139b7b5d0ef3f15ebac888009b40c5d5b3ca39` | 38 | 3 |
| Selesnya Locket | `9ecd98d5-8d66-4cb0-bada-23dfa0faacc0#card` | S | `813c829673de5965450efaa6f8ce58839a68f0d70fbae29032462eb64829015a` | 47 | 1 |
| Seraph of the Sword | `84ee74e9-b3e3-4f47-8b3c-5c1a70f5cad6#card` | C | `55dd6ede8286cf2224ab4c9546fe6d363bc6174293abcc0868543939a91fe9cb` | 28 | 19 |
| Serene Remembrance | `f980c8b4-4cd7-42d2-82d5-96e3275b2647#card` | G | `1a9cedde3bc604e07e8bc60d97f6a124391311ff67210f8cfcaee551cd3ca251` | 30 | 2 |
| Serene Sunset | `e2ef027d-0ef7-4c8b-a594-3918290b02d9#card` | C | `58fd05096d8626bce894fea147de7d3afd7ba9af4bbc527e43b7658e2ad6ac87` | 25 | 6 |
| Serpent Assassin | `3bc7d3a7-ddb4-4eaa-882e-404d8f2926fb#card` | N | `e5d7489a9d694d3a40202d4e3be7ecc6a04690fd92255fa3f352bdd9cf9ca996` | 25 | 1 |
| Seshiro the Anointed | `54fc373e-157f-4157-b7dd-74d689f4c0f9#card` | C | `31b06e5ae47981dac87b75a4303b2ece8e42143bd092d1435bba59fc501bb281` | 61 | 9 |
| Shadow Stinger | `214562b1-fdf0-4d5d-93f0-594b31b80bb3#card` | C | `42e35e8c0480e01d06e651eb14659ae3c37fbd6e294a14cae5ceeb6213e92b7c` | 61 | 12 |
| Shadowmage Infiltrator | `3a6f886b-2043-47e9-9c0f-f7913a6fa67d#card` | C | `4eb281863bf11c326dd3e49c63f820467c88e53397765dcfcd14bc3bc26fd464` | 34 | 3 |
| Shady Informant | `5cf3727b-eb7b-414e-982c-3acf4c05ddf1#card` | S | `a3cc21b1ae106e79a563906048e66d14f84880e3349e1bd42b436fe9e687c8d6` | 30 | 1 |
| Sharding Sphinx | `9ebc0144-fc45-4de7-b3d8-ef8cf2e211ae#card` | C | `08c4f81f21a8ed7957fdc353ad482cff26112cfdcde1a5d479c4c66e0826467b` | 58 | 60 |
| Shark Shredder, Killer Clone | `dabd0867-ee3f-40cb-8314-b4a3966f8b24#card` | C | `06b05f98a020f279c1f0081744bf5b6982958dd0487745143ce0d660c1e2f913` | 73 | 72 |
| She-Hulk, Attorney-at-Law | `db99d52b-bbe6-4a48-8f2c-bf77912dc0a9#card` | S | `085198eae3b3ceb444a6b316ed2e98b23df609d286403a9a3901ecbe555e1619` | 52 | 48 |
| Shisato, Whispering Hunter | `8658cd14-6826-4308-b6ce-fcfd8f2e17ed#card` | C | `0a1eb8b7b3cc0d84c66f8fa613f1241e0989cf4bb754fdc043d253793929ee38` | 52 | 6 |
| Shivan Emissary | `1b885600-def1-4b2c-a6cb-02df34d58a1b#card` | N | `42f181630823c5c4d49101018eee853f4f09116ecb2e1c8de1f5af26034df962` | 48 | 1 |
| Shockmaw Dragon | `e0754b8e-fd3c-4207-9031-a6323e1b66d5#card` | C | `85f4b8c7dcb9bc2756834de0fda7b9da39197d11d98460184d0b1203a1cd297a` | 40 | 3 |
| Shoreline Salvager | `02c08d0b-0d76-4533-a7e9-d40c7c4b3450#card` | C | `1b43fbc4fa92f378e79a0f565cd9341ae320e50f86ff2b4635d3f23028a05944` | 40 | 3 |
| Shortcut Seeker | `27cfa916-282d-476c-8ee9-ca7f0d84f551#card` | C | `597e81996baab3fc11782f7690fcb4f0cd13b0a00fc58fc01f9b047f969d502d` | 23 | 3 |
| Shriekgeist | `e8dd4341-e7e5-49f7-b173-e4c3195e3334#card` | C | `6765b07ca2539d0a0986eb4280aacbed703d92dacf33a38aa4f1c1f996496861` | 33 | 3 |
| Shrieking Specter | `f6adfb5f-a21c-4e86-bd16-a9c1e355af34#card` | D | `f237b45d9ef7031d89a4391687e66c14435396ebd9894cb2c08cf764fc8695fd` | 24 | 1 |
| Shu Yun, the Silent Tempest | `7b22f31c-9caa-4afd-8c44-f88346bfa79c#card` | S | `2cff08b13496233809609ace2dbfe1aed8b867acac2d1de55015487b13d845bb` | 53 | 6 |
| Sibilant Spirit | `9798d307-a44a-4f44-94c7-03e499b94962#card` | D | `fc3cd2c807afa903447e89421b2adf05e0eb498717a7a1707be351375114d9c8` | 27 | 1 |
| Sidar Jabari | `cb637dfd-4022-43a4-8632-77e614436f38#card` | D | `8ba291c099b71cc583f69532f39f14d30533b1d09295b7d6d78b6a71aa6c0e26` | 27 | 2 |
| Siege Dragon | `206539a6-0e60-4986-bc57-c2b89e08b9f2#card` | D | `9b81169cceeb0419967c37b86429ebdf9183df6196e1e3442370645a80d342c2` | 69 | 1 |
| Sigil of the Nayan Gods | `92017087-18f4-46ee-8808-5959d31d0011#card` | S | `73017e1908005fa115c8f076ed1d69f00a7c0f0be75f1ab772650db53ae5c7ea` | 35 | 2 |
| Signal the Clans | `fd68ae3c-52cd-4b20-b015-bd31378d972e#card` | R | `3e34afc41f0b91bb6b07a40fe07cb18baf59b54c1fb66214719907fa8818401c` | 72 | 84 |
| Silas Renn, Seeker Adept | `0dc78b1e-581e-4f28-bf35-e2082553d6a9#card` | C | `1c59a73dcd6f83f32484adb142e39b1e5a7756cbae4f69292912bc556b727ac6` | 56 | 30 |
| Silent Skimmer | `92198967-e89b-4213-a0fa-71361c8420dc#card` | D | `9dca324fb5f60462dc5332e62f97922b4067a603f731a7c7978b4ae409b36f68` | 27 | 1 |
| Silent Specter | `49e70569-b0bc-4705-927e-eb1504925210#card` | C | `f4d2f968a1d9aaefe5639c2b3dc368e6b320081cd02f40ae9330a5c351bc49bd` | 40 | 3 |
| Silent Submersible | `43257a85-2cbe-4931-9755-106808ab124a#card` | C | `c8fd0ff11a8b91ebf715c204ada0e421c39ca2be74c80c6eaa91effe8ffbe018` | 32 | 3 |
| Silkbind Faerie | `fbf4ce62-26f1-4886-952a-efdc1f4946b6#card` | S | `3201383a7dc62ca3604140a0afd1e62cfc8ab8d7986c723c3e413f6eade281c5` | 23 | 1 |
| Silumgar, the Drifting Death | `b26c91d9-87de-45f7-a24c-bbe0cef3a35c#card` | D | `3540c4124294f7b7314b2b27e611241e10429f5578fe3179beff425b2cbbf6d1` | 44 | 3 |
| Simic Locket | `c567fed8-df00-4579-901e-8754079105bf#card` | S | `9607292e8d9406df2860a9a315b14c9a936737f14fd8348ecc6f94591e92b265` | 47 | 1 |
| Skeleton Key | `328834f6-1f49-417a-a2d6-d430b8dca177#card` | C | `248b74c5042b223947d2c7540b4670ead76c439ac32f0883050b7eaf27b644f8` | 65 | 3 |
| Skrelv, Defector Mite | `20053847-6623-493c-8cdb-a69cda3b1577#card` | S | `01dcc7427479c995dddf8d6c7f68753e3b4e294e0a601abe4ba987aa94b9c31e` | 93 | 90 |
| Skullknocker Ogre | `980a0d4d-b070-46c1-be8d-31bdda0d1c33#card` | R | `102c3aae54e17d6622443a4ce9a32b41bf5025e495c966bdeac88353c817e739` | 49 | 6 |
| Skullsnatcher | `5326381e-e5e7-4c3e-9565-4837b3ffaec0#card` | C | `156ac60ac855af2f5f9b34fc0cc7deb14aa0fef00baca639a36f9dfb6a3b644c` | 42 | 15 |
| Sleeper's Robe | `d82783d9-9bfe-4fcd-a7d2-6e2477f99edc#card` | C | `ee345b4c0ccb2958f6e12a2cc4efd121117a431386871091c938af2982082b5e` | 47 | 3 |
| Slipstream Eel | `74e6dd0f-2866-4d45-a214-8b09b837bc02#card` | D | `9c302cfbd8e93abc6c15e99bb23f521a0335ac804fc2b66bd31f9eecae649a76` | 30 | 3 |
| Slipstream Serpent | `aa1152cb-255f-43fa-81f5-430304ce4d98#card` | D | `4501030e2f6bdbed3cf3a4c7d549a668a49f6f9234c207a17fbd70bf3d772698` | 50 | 3 |
| Slith Ascendant | `72f5095f-b4ba-45ba-82e5-9a22bddd544b#card` | C | `668ab2fbaafa5b55db00dcf989959f5b75db464c8ead219731bd255eae797c38` | 31 | 3 |
| Slith Bloodletter | `fbad6038-93ed-4a28-b1f0-ada54608758b#card` | C | `182861c4e8fbd7b7d7539a088314d23d3f4ed350e0ac8f2adb77be40117edfaf` | 44 | 3 |
| Slith Firewalker | `ea1af948-686d-4b40-ad4e-cb48a0258269#card` | C | `8e39123501f9f65b5f040babdcb0382e45462cbb0cfaf0434d439b862fe78eb0` | 31 | 3 |
| Slith Predator | `fd9c984e-daa2-414c-bb90-ecc41aaa896d#card` | C | `242f33f535ee25ec744a349ad9ce3b97cbc666e9f35c9fd04fb1d30c40cac8c9` | 31 | 3 |
| Snooping Page | `f07c07d7-00f1-44a5-b51f-0a22cc51fd2b#card` | C | `01d7ee8482a84918ca574e73a511acbafd65fec0b1f4cdacffb2d95a7625d430` | 73 | 540 |
| Songstitcher | `eec72bcf-ccbc-43fc-8bdd-7bf4faa1fad7#card` | C | `016cb8f1fff7aa924816f3cbacd779915014359967bfae09fe6feea0fe78da2d` | 40 | 513 |
| Soot Imp | `c867e31a-90ea-4d20-80db-3977ec68b16b#card` | N | `36abff9c26c8f928a9ceac77b5c3ecd02b51772ec3024eb6e248418c5236ee9d` | 29 | 1 |
| Soul Seizer | `98950aaf-d0b8-420b-9e6a-c7d07d0f8588#face:0` | C | `985d9477b43d00e2a4231075b157095a943dc574f297672157c90044cae1a923` | 58 | 3 |
| Soul Shred | `2894ef5e-738b-4ece-b143-6662d9453295#card` | N | `9463da022c056bbece5f557a755d7d2c85936ac9996fcc062cdef2a01560c7ca` | 30 | 1 |
| Soulknife Spy | `813c0654-7e3b-435b-b7a5-148f0c36a4be#card` | C | `78f48467ebaf88c5900cb922a91ee7468f76993ea9689308dda1fb570fab5241` | 26 | 3 |
| Soulquake | `0b9a477b-f2dd-47b2-8493-eb63f44a2688#card` | G | `333290865c078f16675d79d6f9811ea33023e443c801b13aaa6116eebb4dd9d0` | 31 | 10 |
| Spawning Kraken | `f0357833-80b5-40ba-874a-bd22ea6e4e46#card` | C | `2bafc5be97af728b3b25a44cdbea3bedb83f2a536e2a5bfd906f3939373a1315` | 49 | 6 |
| Specter's Shriek | `8c9164b5-10ed-4cad-af33-7d4b5d3d6bf2#card` | N | `046293cbac711e42425f1895cb2d2f1512e69918367029c30e7497abde76fff9` | 83 | 40 |
| Specter's Shroud | `dbc2b0f9-73e8-41b7-9d64-07f86fbc5d6c#card` | C | `c9c72fbaa4f953790e47f3fde3c957cc5b1c9ab295620ce67843fa8c88d37a3d` | 50 | 3 |
| Specter's Wail | `af32ce11-c026-4135-b1e3-10c6aa15c72c#card` | R | `6fd5043146a5e7631789b246f9742415de3a44843fd4d5cf43efa0f176467b69` | 16 | 2 |
| Spectral Force | `271bfde0-6a6f-4bc8-ac0e-d2eff6784463#card` | D | `668557d560106e98dfcc4185f04bf12b7d637d24c671f428939c6d7033e7273b` | 42 | 10 |
| Spellgorger Barbarian | `9c0fd5ad-fbba-4598-80cf-6540888378cb#card` | R | `05349ac3b2e3b5efe0d7abd4457e025b8cb84ee8f8f5a4d120965793e1d71558` | 41 | 3 |
| Spellskite | `e0aa6ce0-ca31-433b-ac6c-32b8675cdb71#card` | S | `87f6f0c719aaa86e8216256022a43134b5b0ee91af0b124d58e9a6b5e1556c2f` | 27 | 5 |
| Sphere Grid | `f4770345-f15e-4e31-b6ef-4c2608df9644#card` | C | `19feddeb3cdb5d2f7ba016255d2e38f234495a2f4764c565fd22f425300994c9` | 162 | 15 |
| Spiteful Returned | `e7e60afe-5ddf-406b-86e1-e82e2a660432#card` | D | `d1c83728c00e175e8517be6da064fa8f214d56688d7756183a838d5c5c01c1cc` | 46 | 2 |
| Spore Frog | `97db6c39-e690-49b6-93a6-e51b8dfad10b#card` | C | `45b2b91e1c3f3afd4b19c681bcee273603f741cbc41acb6e566aff8afbf25af0` | 32 | 28 |
| Spring Splasher | `f1136a22-b202-4688-a44c-a9979c91261a#card` | D | `925e2c6f74241a058670693343519c6a0d328e6a8f25dbc6c1bcdf00eb9d86d5` | 34 | 3 |
| Star-Crowned Stag | `a375b739-6a6c-4e07-9b1c-a304f84ea2cc#card` | D | `9cfc2d8d4e3e4f025cbf2dcedf30960736a8d6be7270e6d2bd0ba137e22f3b6e` | 24 | 1 |
| Stealer of Secrets | `8f9535f2-f00c-40d3-9997-d1c0ece15f9f#card` | C | `78f48467ebaf88c5900cb922a91ee7468f76993ea9689308dda1fb570fab5241` | 26 | 3 |
| Steam Frigate | `2e00598c-4f06-45aa-87a7-c63b5e8e92f3#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Stensia Masquerade | `88f5d146-420b-417c-bcf1-fe0e78312a74#card` | C | `7faa940004d1ec0885067197cb5db4f8f982952e123999cc4534f1d0bf81cee7` | 58 | 6 |
| Stillmoon Cavalier | `9c430b5a-83b6-4f4f-b1d3-5737e1f61947#card` | S | `42caf6d0727ae3507191f85d24a0fb1e79c1fba1d41fd45d1b81bbc33ab2fb1b` | 78 | 8 |
| Stinkweed Imp | `e005bc76-4985-4ec6-b9f6-cf6d0d9f5df4#card` | C | `74a915e8f91e96a7c62faadd9ceecc49ef67588d7b133a7cf5833d1d3c4a2871` | 33 | 3 |
| Stonespeaker Crystal | `a494fcee-6885-434c-aad5-6f83640c4472#card` | G | `a9bb57584fe218bffa2b8ead33e06604c021c602060074d725a9753a37256729` | 58 | 1 |
| Storm, Force of Nature | `7c47876d-143f-4e4c-b151-90c099e452a1#card` | C | `032332b85842924e055673a81cf1a1081d3c34d606e705e8cd451caa0f11c98d` | 146 | 180 |
| Stormbind | `78f50668-36fc-4911-84f7-93667436b0c7#card` | R | `404dc9ef9a8bd83c3eeb4a4f56a4ea27a92bd3bc2748d6141446317ed9e3909e` | 30 | 1 |
| Strafe | `6ac7990b-b86e-4725-974c-d6affa2af2f9#card` | N | `0bb69d73ed2414a40cb72e589024f6911145338e84771008d6595f03d01fc65d` | 19 | 1 |
| Strategic Intervention | `b8d019fa-3893-453e-a46d-aa53fc3f2f0e#card` | D | `14f0eaafca11c1953ec052ffea4e187ca386b8d222688a10ecae7a1cd0f9e841` | 53 | 6 |
| Strax, Sontaran Nurse | `ffde2558-2168-4b40-a31a-86d160bdb289#card` | R | `347880407d018280de93ac5be8d212054c96a9cb5d10c0618adb7a9091f16fb8` | 287 | 8 |
| Stream Hopper | `153cd93c-7bba-4482-893c-55a602c4439b#card` | S | `3d2aba2f5a69833c5d2b99a21fb6397f08dd7ee4d06c7709fce3f80ab00753ec` | 21 | 2 |
| Stream of Acid | `1b9c9083-62c3-46fd-a2a0-6de7de88a3c3#card` | N | `042f0efa600e94f6a27901e3b42bcf84eb13be19663b3079314f2ab7845a800a` | 15 | 1 |
| Stromkirk Noble | `c55909d3-1f67-4a4a-9d53-8513d6cf96d8#card` | C | `92368157783bec5e119e4ddfc3debf8e05892e4911a102247438d2b8de6c312b` | 50 | 15 |
| Stromkirk Occultist | `3bdd140b-1406-4d07-8f35-78f2ba92de64#card` | C | `006d8da3f1825689e1700ac2ead88a563061241f9c6e231d64ba47a2e62432ed` | 61 | 6 |
| Stromkirk Patrol | `5c5fbb54-f0e3-4afe-9158-ff9eadaa338d#card` | C | `f1f33d957ce31791fa34bbc8d391d27377957413c961d56b70b0f48f800954b8` | 28 | 3 |
| Stronghold Assassin | `35826db0-b6ff-47b5-be16-c5b878d9e842#card` | N | `6419323a36d02a3c1b4a85c6ac0957e96af184b3efbae336b82e8e38338e3f88` | 24 | 1 |
| Stronghold Rats | `e91053ae-682a-4bbe-8394-fd65e626bacc#card` | C | `175145807381e3edf70f9acf55caecf9f9769ac714b43b0afe924c6e6dcd2394` | 32 | 3 |
| Sudden Storm | `32e915fb-3836-4bed-93b1-61c9c3951ad1#card` | G | `69415577799d13deef474240b6b543b263006a1c6bd3ab88f079b30fb8857885` | 44 | 6 |
| Sunder | `fddc93a2-912e-402a-9d0e-34f778e659ea#card` | G | `092a0b2eb205a4419de24fbc904747d1662587cf2eb4220d72a3264d09d9d6ee` | 16 | 2 |
| Sunken Ruins | `e6415ffb-8b7a-41c3-bedf-0d4112b7b795#card` | S | `7c75787f3c130249bc031301da3625edae2d8f0df5a30d503fe5a3c232972b73` | 41 | 1 |
| Sunlance | `3c8cc7e2-7bff-46e0-bb5b-1e721a56a3ef#card` | N | `3da796f388e55ccba7928ced572969dccccfa01f6c13241c57716ac299703f09` | 19 | 1 |
| Sunstone | `300ad0f7-92a7-450d-a9de-b658836fe3cb#card` | C | `27a702073492f927a286d1c36ac353c3a8e1f32d3077bca2e52bd79d85e85bba` | 38 | 28 |
| Sure-Footed Infiltrator | `e6108735-8370-4611-ad59-d7da7fd0323f#card` | C | `0c75e717b5d4b49ecdb0bd33f7ad293740ebda6d3eecf311f0ffb4102812822c` | 60 | 18 |
| Suture Spirit | `4b2eba61-3937-4dc2-ad0a-e51fff8ecb80#card` | S | `f09c44d209bac69efea21b8cb3e71f29b99c4fde05e8edc522aa0a1a9d0c2bab` | 20 | 1 |
| Swathcutter Giant | `fc16a2ce-a570-454f-8498-f9c604c0d68b#card` | D | `a61daa1083e34f9c1b2ee42fd1c221cca6246437b85180729681129f74309af3` | 32 | 1 |
| Swirling Spriggan | `3b0cea17-9baf-43c7-9f01-44f255e9d687#card` | S | `2f500d04922b7b31c67894a236860548b9d370c245fd74f3819fd3eb89838499` | 38 | 4 |
| Sword of Body and Mind | `fac42229-4f5f-4d04-85dd-5031d4e435aa#card` | C | `109d23a05a6448e0207e52d0060eaa1f4e5ac04811082b8cfafb7a13a466a1f3` | 82 | 6 |
| Sword of Feast and Famine | `d0901053-6de0-46d0-9ee3-8d40510236c1#card` | C | `4b1ef69dee9afdb57179ee0b114429153612d8230d7d8af7c0da8cc24756555c` | 74 | 6 |
| Sword of Fire and Ice | `2ccdc60a-49a9-44b9-a7af-0ebf18b26785#card` | C | `1d2b4b70db1e68b0c989d07f68376ea621263ebe7f5e47cfa1ecc9e34b32737b` | 72 | 6 |
| Sword of Light and Shadow | `01c8a554-45a6-46ca-b40a-af0fd4529e40#card` | C | `19b9249f0ce53344400fef4f3ca8a6545d4bebf0f6d684b2be19351fedb8200d` | 83 | 42 |
| Sword of Sinew and Steel | `ccab4509-f189-4610-9597-547e7f6b0775#card` | C | `7aa7eb3af77f5b51625b0f6bb11ddaf931da19707fd95a0078b75ab54ea1148a` | 68 | 3 |
| Synapse Sliver | `b76f123d-7b78-42a1-a66e-0dee6f773687#card` | C | `a0cdf661d30b04be10c47e7caf4acf33859636a1696e956f7fb94b64c1dfec13` | 32 | 3 |
| Taeko, the Patient Avalanche | `3d3eb043-8ce6-461a-b8bc-67f0ba6cd580#card` | S | `6a4c66e91565dc6d9d166f23fd762a7d59368d7d96045d5c05193d308e1645d9` | 105 | 12 |
| Talonrend | `85c6290e-2bf2-4666-a606-326a8209ace6#card` | S | `9156de9678317c7aab776d20945a6e7b875eef19d4a9decebb8c7dc313191e84` | 28 | 2 |
| Tangle | `f627e125-15af-4e53-b34e-82b60e4ec87b#card` | C | `0358f694821e2164f05b2b2f842597629b56f1c01bffa5287f32d8d3f540e4df` | 50 | 168 |
| Tanglesap | `a533df83-782f-4f77-a0be-312ae56f6447#card` | C | `0009f25fe1387aa0b61f40d4d33fcb7b9946d498d4679ae57d79ac4d1bbf9d7b` | 33 | 411 |
| Tariel, Reckoner of Souls | `9c00b503-ae5e-4ce3-ad6a-92b541b2e3cd#card` | R | `2486782507d259d3d8f3731c41a64173c1e82051ed950ef51963a7c4c2a429e2` | 48 | 9 |
| Teysa, Envoy of Ghosts | `98b283dc-96a6-45a3-8a3f-11458497f358#card` | C | `1701f64c4708a924153416d1b2f885250653b1a118ba0d843d8c4750db1cabfc` | 59 | 42 |
| Thassa's Emissary | `fcb97d06-e0b8-4a13-ab7c-1304ccb7ace2#card` | C | `f773cdd160c08f77dc95d57321328abc43cb3659d5668da59bf42b8ed6c98ad1` | 51 | 6 |
| The Beast, Deathless Prince | `ca187570-3634-4296-92b2-1fcd99d150e3#card` | C | `0006d6ebc8fbf7c9e683de7a3c07eba7595bcf9670da0e64de662d12bf98e09c` | 108 | 1800 |
| The Death of Gwen Stacy | `2624b5c5-ba59-4e20-8e66-e463268366e4#card` | G | `1687ec773eeb5e3fb87e1762f47f895aa395369f5d5f1ea107047783cca60f85` | 364 | 1 |
| The Falcon, Airship Restored | `9f4093a6-83f1-4c1d-8b03-8fe15647fb11#card` | C | `17ebe40e3cb78f5caa3bdaf5eb664b445cc9cd9b370d40604fcd089203a28173` | 86 | 120 |
| The Nipton Lottery | `8af798a9-2198-4f39-aa6d-5420a0f1203c#card` | R | `01d536320605bceeadca68dbb2ae667d6954434c0d1314e092917149c41ee842` | 69 | 16 |
| The Thanos-Copter | `e58e94dc-9602-48db-9dd9-9d3051c33a76#card` | C | `00255fd61ca407782df370e9ec875498e2ed0ef28b76305ffdfbcd3d9228bd82` | 74 | 15 |
| The Unspeakable | `64e413c9-96e5-43c3-a182-84d53fef4327#card` | C | `1f80071f907c627c734062edf7ec3c325a6171ebc6501f3c8bc43527840f3539` | 44 | 15 |
| The Wasp, Winsome Avenger | `4b706959-ea38-4808-870a-7c35d83f7ab2#card` | D | `134067d78a6d114ab802db2395d252d40369bd615e603d9e4c3dc4acc58f5461` | 54 | 12 |
| Thelon's Curse | `f6880f71-a05a-4752-ba33-ffbc450606bf#card` | G | `00b850e15d364b22ecea8cb2cb955be8f262b93b34755af5a0c1a20beb269fb5` | 98 | 960 |
| Thirsting Axe | `e024870c-af13-4ef3-be27-cdd84dab7d7d#card` | C | `0260496607bdf7e90dace1d7291ab16259f4ef07329e1a8a3899c9de76a4e686` | 62 | 8 |
| Thorin, Company's Leader | `7926b4c1-9a93-4193-ae6f-e75bbcd762ec#card` | C | `6d080c80014a6a4b2cb2c3f63305c054c3db6ee52d10dbc973ec5e06774bcb17` | 62 | 6 |
| Thraximundar | `9e0e4217-fefe-48dd-9153-032460192b19#card` | D | `f358703f0de3679a210fe5bba0c1a436ee2bb011c20f1bca68065b1293391ee3` | 58 | 1 |
| Throat Slitter | `56f782b7-d46c-4aa1-ae16-66d6401c72ae#card` | C+N | `30564fcfca5de52bf96c6d9d1f5dfc0cfc9c773d31f5d7f64b10cc83c21a9e70` | 40 | 6 |
| Thrummingbird | `eac94269-4baa-4b8e-a0fd-d6b227d1cde3#card` | C | `d16c2976fc0a4c6ebeb46a9f0a5ce80c67efd1f103753aa61618ebe88899eb01` | 26 | 3 |
| Thrun, Breaker of Silence | `789b7af5-ac15-40b6-b5b7-f3fcdcfb52e1#card` | N | `04797127f00fcf6cd2d4a84a8ad3c2ad4bbb19a5ae3773f7a46ecb2d52f5e8a4` | 83 | 452 |
| Thunder Lasso | `d98cc18b-6ebb-4f55-9d0d-b8fbe9a92bc1#card` | D | `f90aaef89bf32db08604972fa9b6e20a430bb386a2c7a4b23541c9eb4dd5d2d5` | 70 | 1 |
| Thunderwolf Cavalry | `3b36df26-6a41-4714-95a8-fd02e2fc845f#card` | C | `3b8278ee77850d1ad2a10f25ee1cf4b5dcfb050b706540a04b02a9abb17e234b` | 139 | 6 |
| Tiger Shark, Abyssal Hunter | `286be139-1ddf-4b04-86b2-cf17a06c262a#card` | S | `6eb803f60360fa99de8f79a80eca5f1e36678b00a167e1bf5a96c5db00f7cade` | 44 | 3 |
| Time Beetle | `247a0c13-c611-4c00-bedf-ecf7ea6bb956#card` | C | `3bfe76c2116b14bdf54f027fdfa6f0bac81abaa20e7f3bd348c7e11273fc21cc` | 126 | 3 |
| Tourach, Dread Cantor | `0c952586-e522-4982-bebd-96f1db73053d#card` | R | `40391eebec54150892af514bbe94133aa2641e5da078652eb69a4b4382d73373` | 68 | 4 |
| Tovolar, the Midnight Scourge | `45d49831-548a-4a0e-9a18-9f7397913895#face:1` | C | `0b58691b44b665e08a38e81e9b3a7a46c4b2de6159b3f20a1c4a38bbbb67ff96` | 73 | 36 |
| Toxin Sliver | `96ccad44-7912-4fdb-bb2d-9d16cc3cd68a#card` | C | `f48912a3c5a9d6d9f4ac86f4ec9d4144c3f924a67da9baf1079c52c1b3502aa6` | 40 | 3 |
| Trespassing Souleater | `4d19b4d1-49d1-44e9-a453-dcb08b5fa6e4#card` | S | `207d465b84fb7ca8a24315da93137c46099652704a97deb3b7cac9175494a38d` | 24 | 3 |
| Trostani, Three Whispers | `5c90b951-826e-4595-aacc-958904f3f794#card` | S | `403dc509350b04cf1ab8e420282680c18c1c74f01235fb1826b5393a16723e66` | 65 | 8 |
| Trygon Predator | `c744b5f4-fbcf-48b8-9d60-5e9c6ac297e0#card` | C | `872631f358864b709a94af0266be85f958e5fdba65c25af303a4e7da47d84bba` | 42 | 6 |
| Twilight Mire | `db623754-e078-4030-ba07-818803c348a8#card` | S | `1f396c1e812047f473496a660064365d136eaf98f280dc220f52b52eb20fc1cb` | 41 | 1 |
| Tyranid Harridan | `01432542-2ea7-45bc-bc90-151516faee0e#card` | C | `0c11fbb28b27f40630e3456e57bdc4e73d3b66ab7fe8a6b9faf0def6a987adca` | 158 | 27 |
| Tzaangor Shaman | `1d55a1c5-fd0b-44d2-9e7c-550052fef262#card` | C | `004aa950f3cf435c92de34cde338cdfe509d3bf4ad7678c7dcc85e8db5e10c53` | 172 | 4200 |
| Ultimecia, Temporal Threat | `5a8e7e35-e9ea-4808-8aa7-7580a23b9d9d#card` | C | `320dac8ec06e19b79fe7c8d30e51686acc6902a5bd9f71ceaa13d98b29cc9508` | 55 | 6 |
| Undercover Crocodelf | `3ba223b7-743f-4f99-836e-869778fb2e99#card` | C+S | `b0d7eda337572aa7075daf92908086c2966c8d600aebd8bd72b7a69d979f9af1` | 30 | 3 |
| Undo | `b446c3bd-3c7d-4d59-ba59-3dd80818465e#card` | G | `19e73492999744688e68b30dfa9149540fdb6d706116f43e44d5c8563dced0a5` | 18 | 2 |
| Uninvited Geist | `5c975dbb-aabb-43fe-877c-c3950019a584#face:0` | C | `ff0e2830c652b6b9bbe3bd5374f0cc450ef8f1b3a44b6f2a1fec7d23fa17e3b2` | 28 | 3 |
| Upheaval | `7cafc972-a6f5-4cac-a3d3-8a3ae36ffb1e#card` | G | `40c7de8deab65195dcdafbca15fe5a38b6f99f68d8e6c42a73809546bbd6f27a` | 16 | 2 |
| Urgoros, the Empty One | `43e7b834-d3a9-4297-bf79-2dab5507bebe#card` | C+R | `00e712f0dd8f485393633e8c9b6dda56e69349cef985c8171561b64a29563231` | 54 | 9 |
| Urza's Bauble | `17dbbca3-ac1c-4d4e-9618-3e66ac3ccd24#card` | R | `02e1380cde0c2e95dcd5a65d25db5b597a83d4caf4e2f6597b1c83bb40f94323` | 56 | 12 |
| Vanguard Suppressor | `b7c48f87-bdef-4cef-a1c4-5b4d0b7a26cd#card` | C | `eb663d9c6b7a028939a31ed2b01dcdbe497f1dd0ef791f5712dc86196a51f8c7` | 134 | 3 |
| Veteran Brawlers | `77a963a3-473a-4c34-bc46-8ef016a92c0c#card` | D | `104ef0f14b8cda08224a295869b853960bbb936097fe959eb3b495cbf194ac36` | 51 | 9 |
| Vexing Shusher | `a20a7cf8-2075-47ad-9229-36264b112e61#card` | S | `eea94a6041e95c505ae1e07c0076081bde5c80dd5aabdd32f084cd6738287b8a` | 38 | 1 |
| Violet Pall | `bf6ea336-6a4d-4653-acdd-f09af0b9f0e3#card` | N | `14108d6406f7a447d997c7e49f9985c669c045f65978b2db4b3ef20e72aa102e` | 39 | 8 |
| Vodalian Knights | `f6daa28f-e5ce-440c-8dc2-b36f59ae0d4f#card` | D | `136751bcccad846a46171682f3fd2a6c1b577264494e530bb97abb81c592a563` | 68 | 6 |
| Vodalian Serpent | `c39c1604-3bae-454d-9985-85101e51ec6e#card` | D | `036975f39484cbf45721b419f4d23b04eb77a281356d3a9e8e82d0bccbdaf203` | 58 | 27 |
| Voracious Cobra | `5b2e1105-5af0-4f34-bfe9-15a2f6881edd#card` | C | `f80afd82120709caf421db89c31f1e94ccaa790cd20e142f83cb0363a58d9e92` | 29 | 3 |
| Vraska Joins Up | `c91b0dd5-4c63-49a5-95bf-c342b6ff2076#card` | C | `04635e980a0c6d1603788e805e0ed59789f8fc0c5c33487c184370252efe7485` | 59 | 6 |
| Wakka, Devoted Guardian | `fbd37284-35ff-4409-b6d5-f813c06c4931#card` | C | `3b5b718e95cfaf877945e8f69f84689c6aa4da3be830665d81debc2f69c31029` | 196 | 72 |
| Wandering Champion | `54af9e23-459e-470d-9b0e-e91a0fa45c98#card` | C | `867d1b4e3849fb785e7c3b2eec370c6a11b73687d0c177e1903f09c8a62c0966` | 61 | 3 |
| Wanderwine Farewell | `7e1da4a1-5922-4a2e-8bbc-25fbcbf20935#card` | G | `00006190ee4303fbf9ca6757abb5f7321c24299f2e0b6c833fcd9f6e6c3ac85b` | 74 | 142 |
| Warning | `c5ba0f0f-65c5-4ffa-987a-f320b401ec8f#card` | C | `0d595a49a70a3ee4fe14cea3b76335ef35c76244db1b1cf464b1a7e791c03b19` | 32 | 58 |
| Wash Out | `54748cb1-d92a-4212-ad76-417ee79b5ef1#card` | G | `175cb5d229d944a04dd8920814978d5635d77a97435aa9ba3e4be75793700e53` | 26 | 4 |
| Wasitora, Nekoru Queen | `13fc4168-fcc2-4011-804f-211d5d86b7dd#card` | C | `08f88e65e173ce5cdcae1132ff57159b92b9d265220a2fcd78630827a0f41f51` | 80 | 27 |
| Waterspout Elemental | `f86b8c03-9f1e-4d42-ac45-907c46e9e1c0#card` | G | `6f13b0fe1dd8006402e40bab318d3e5fe69aafde51a4106f7f31d72907cb25d7` | 52 | 6 |
| Waterwhirl | `77636154-962f-4ca8-85e7-c0c9c0c3cd2d#card` | G | `24720e57c5b22c9840e1a8c7ce76163c7192d634d8b9cf75c23cfcf1b39d419e` | 20 | 2 |
| Wave of Rats | `ccb0f59b-7947-4563-9e16-5d23246f4232#card` | C | `32afe507dcaef523c712cebad52c3b9b28fdbeb3047ab9c37a7e537bde145d28` | 51 | 65 |
| Wayward Guide-Beast | `c752e001-56cc-46dc-acb3-3fca6c0bdd2e#card` | C | `5ad1c50c3289fea3ab3b8397e6ce30eb925559d20f66cf9aef0ad017b31bcbe7` | 41 | 6 |
| Weeping Angel | `1a89c623-c9c3-443c-bbd3-834767b6bdb8#card` | C | `0e4e8db32fceedfa8ec08fdf08daaf8455795d9b409fb6a62b8ac968215949a7` | 83 | 24 |
| Whimwader | `21d9ce2c-eb6a-4f43-a79b-0b99b3dc4a00#card` | D | `a7c1f480fefc55bf0123062a0a7d48fe3fd123f50222a8983d9fa504f4e24eaf` | 26 | 3 |
| Whispering Specter | `18d270bf-d0d3-4fac-a8d2-5f920e789bb5#card` | C | `5b7fcf993bd6a8303d75f517a563a15bfcd29de87d61ff04eb88d8be1660f7bd` | 66 | 15 |
| White Widow, Yelena Belova | `7aff7cb6-aa56-47ae-8813-75d850917320#card` | C | `22cfcb2940c7a723ad752349f9030e079c89650b08c54ad6b9475cb928db4cc9` | 39 | 6 |
| Wicked Pact | `39e21a5a-b278-478a-854c-17695c0f6246#card` | N | `ae82c41e499a56273c58d8a6ca42c8022ad67bf3afe7cc19a8f18aa04dc5e25c` | 26 | 1 |
| Wight of Precinct Six | `6397c046-4c59-4f0b-9b44-2a804eb95edf#card` | G | `345401027c46ae4a6270f8293b60b0dd4a595c255762d9a4069c42ea942d4fe6` | 30 | 7 |
| Wild Swing | `92d07c7c-be30-4bcc-a632-99986fc5315c#card` | R | `1f399fdef294e82501f2ccfe55600c006162ad30131013ac5825604ff2f6e2b0` | 29 | 2 |
| Willie Lumpkin, Postman | `b08633e9-7b8f-4f69-a3b7-8d982b17574c#card` | C | `01c0eafae0ab0a08cf51ca248e7c9fa9e1755dbfef5f89fb9b9b587f1c921cf6` | 95 | 30 |
| Windrider Patrol | `dea0f359-19a5-4aef-8c96-574eaa748a31#card` | C | `15d7c624443114dfd1f81dbc03591e4fe8b847a4631c8d681e553da92f6b73b6` | 28 | 3 |
| Witch Hunt | `e86bd38f-7804-449d-af29-21e96a56ab30#card` | R | `a11b0bfb1cd90973cae02c64f599447dc0157e4c15dd90f814f173dfb6875e47` | 76 | 1 |
| Witch-king, Bringer of Ruin | `afcb21bb-aeb3-4024-9ce1-68ba7fb64b2d#card` | D | `125208ff19e2c06b235fe2cedced3384f02baef33fbb22a49c4cbc76f4eb2689` | 41 | 24 |
| Woebearer | `5ed93d71-3f62-49d4-a9ca-5728c44b0633#card` | C | `2eb5e2af6fe431f6a0c8c986b9faa92bae7cb9340bb00cf2e623f29cd19214c2` | 42 | 15 |
| Wooded Bastion | `61b85077-64aa-4bcc-890d-2d88da9543c0#card` | S | `9f0ab0c1aa689caac7af431d31e6691af5a6923db1b71b9e062ca5df6d87e613` | 41 | 1 |
| Word of Undoing | `2597a1ee-fe9e-44b5-a5e3-d34332c6c123#card` | G | `37c07456d6c4ecd50eefbed7bbfaa485ce577783df2c3ebd9383f63f3e0447bd` | 30 | 6 |
| Worlds Within Worlds | `e25062ba-7176-4d59-ac3e-26d19b26f02a#card` | G | `044a085d104e73870757a8615c20ad7a80fbd2827ac3a254df216cb91fc05d88` | 71 | 3 |
| Wrath of Marit Lage | `740cafc6-9521-403a-ab5f-11582bf55af0#card` | G | `1a9f1269ffeb09097e43de407c8e5d89e3b122c318cb02df2333d228af679bab` | 43 | 6 |
| Wu Warship | `f184e860-05c3-43cf-a625-ab53427406c5#card` | D | `1079c9e1dcd51604ef72d33bf37315aaa8650ad924cf4b9578030dbfbf8e4066` | 24 | 3 |
| Xantid Swarm | `13ba9ef8-2010-4d3f-8c62-85b7c5620031#card` | D | `3daa271773cfece5d50647adfd62a65316b74ef5da1ebc7205f8e761a8252530` | 30 | 2 |
| Yare | `f0f24c6d-80fb-4e99-a74d-6ed9d349ed3d#card` | D | `2e84d6d863378b5d4197930a5d61be99bb841b211610a3bff681bc107cbd1c52` | 50 | 4 |
| Yathan Tombguard | `2321b4fc-6b73-4d1a-84cc-13524adb0899#card` | C | `026fd7f0f750ae51a7c8110848442a563f9e60f5509aa7185d999e029e7cfbcb` | 54 | 30 |
| Yidris, Maelstrom Wielder | `9efe8aff-9a7d-4397-b5fd-c1a0fad7c15f#card` | C | `41b5309cd07a7b63aeebd4902b69aa5d3e0316e7db19444ad0e7a7bd4ff1990e` | 46 | 12 |
| Yuan-Ti Fang-Blade | `8ef06396-861d-4599-9976-6e5c04575456#card` | C | `064cae34ba2fbc1e993412f12a5127227be0a19aed44f9ce6b586287519b63ca` | 26 | 3 |
| Zara, Renegade Recruiter | `cd2720c2-522c-4fdc-9cef-9c5ce250fe7b#card` | D | `0fff9ad9777c6128ef270a1a39ac29f0cd0ffaf31596b2ba22a63d62c6b9ad7c` | 94 | 8 |
| Zephyr Winder | `13c4437e-48a9-4c51-9b76-9e83e3ec362f#card` | C | `576893c2cfc0f852934d36830331578c536e79aac4cfe408166b29d098da3ed7` | 33 | 3 |
| Zeriam, Golden Wind | `fa1dc21d-f7f6-425f-a4ae-2b0f143091e3#card` | C | `113f19ac9ef2990e5f9513c7cc3158cc4610da64f06a81f56d7853252ad89d5b` | 49 | 24 |
| Zhou Yu, Chief Commander | `0b4742b7-e769-4354-beaf-6b4d18768ec1#card` | D | `38bbccfbfb86bb3711bb07f0c3bf93e85d928da282328ae4186fc443f556171c` | 24 | 3 |
| Zombie Assassin | `7d8af783-e73f-49ce-bcdb-ba55e1650481#card` | N | `46f4ff1e46c977cee9fb17b94d6ca1b76c1ae4e4d45b85525693ed6da22f10c0` | 47 | 3 |
| Zombie Cannibal | `baab3245-6616-4a4a-a168-3215a7878073#card` | C | `0618ea44c59756eedb329ecd39821fd4bd9b60087ada2331316361c69c518251` | 38 | 18 |
| Zopandrel, Hunger Dominus | `b168e4e6-f572-4d9f-b98f-95b2611354cb#card` | S | `0364e9811de10ab5687a8c441651084528ed8b71904c26fe725ecae33ab4618d` | 70 | 29 |
| Zuko, Seeking Honor | `6e737c52-cbe0-493f-ae46-7bf068e04882#card` | C | `0c825abca95cf6702d680690bf6736a9092d789508defd03819c2266cb8b5b5d` | 60 | 72 |
