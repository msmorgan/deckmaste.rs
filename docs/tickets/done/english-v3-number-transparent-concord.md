---
needs: [english-v3-scalar-cardinals]
---
# Propagate agreement through number-transparent quantificational nouns

Fix the feature propagation for “any number of target creatures”. Keep “number”
as the syntactic nominal head and its Of Complement as a distinct constituent;
derive NP Agreement from the plural oblique in its number-transparent use.
Do not disable Agreement, globally make number plural, or lexicalize the entire
phrase as an unanalyzed Determinative Phrase.

Pinned witness: Sway of Illusion's “Any number of target creatures become the
color of your choice”. The subject has two Noun Phrase Readings, both only
Singular/Third, and the finite predicate independently parses; their combined
Finite Clause has zero Readings. Eerie Interlude supplies “any number of target
creatures you control”, also currently only Singular/Third despite recognition.
CGEL Ch. 5 section 3.3, pp. 349–352 (especially p. 350) explicitly includes “any
number of people”; section 18.2, pp. 501–504 (especially p. 502) requires the plural
override in this quantificational use. Consult full sections before implementation.

Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered
13,226). The 149 raw any-number-target surface occurrences overlap other causes
and include reminder text; neither they nor constituent success predict face gains.

Express the distinction through an appropriate declared grammatical property and
composition policy, not a guard naming number or a card. Restrict initial breadth
to attested Oracle vocabulary/uses; do not add lot/couple mechanisms merely because
ordinary English supports them. Preserve ordinary singular uses and nominal scope.
Record any newly needed grammatical term in the owning glossary with CGEL authority.

Acceptance: independently construct the intended Sway of Illusion Clause and
contrasting attested ordinary-number uses; verify obligatory plural concord in the
quantificational use and both roundtrip laws. Report exact Reading sets, lexical
ownership, identity-level corpus changes and remaining causes. Scratch reports in
/tmp/english-v3-np-probes are optional. Standard constraints apply.

## Sequencing

The `needs:` edge is sequencing, not a technical dependency: these grammar
tickets touch overlapping structures and are worked one at a time. Write the
pinned witnesses as tests before implementing.
The landing compares corpus Reading identities before and after on its own tree.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It drafted glossary entries for Number Transparency and Oblique, which
this ticket owns and should write afresh from CGEL. Read it for ideas if useful. Do not rebase onto
it, and treat every claim in its ticket notes as unverified.

## Landing record

Completed 2026-10-05. Standard constraints apply. Baseline declaration tree
`yvwxwoxu`, covered 13,594; implementation `uxztsmxn`, covered
13,596. Both report headers name `uxztsmxn`: the before run used its
then-empty tree, whose declarations were those of claim parent `yvwxwoxu`.
The before/after lexical inventory digests distinguish those measured states:
`efdb16a23340706a4967063224c01f298231e173f8f9a2a2dfa088a067d025a9` and
`3529edcda18b9a654fead1641ec76614ca3c449d2066a0147d337956b031be07`.
All corpus numbers below refer to these states; V3 emits no coverage lock, so covered means the complete face census.

### PROVE — retained identities and structural laws

Both trees completely enumerate all 32,828 supported faces on the same stripped
parser inputs. Covered-face losses: 0. Of 157,573 baseline Reading identities,
157,539 survive byte-for-byte in generated structural identity; 34 invalid
Readings are retired below, and 36 newly admitted Readings produce 157,575 total.
There are no regressions, unexplained losses or re-coverage obligations. The
comparison uses SHA-256 of complete Reading Debug, with no normalization.
External diagnostic exporters and snapshots remain in `/tmp`; no process
verifier or evidence blob entered a crate.

Every counted Reading passes declaration admission, byte-exact realization,
lexical ownership/context and construction/word traversal identity. Internal,
admission, realization, ownership, duplicate-Reading and traversal issues: 0.
Independent Sway Clause, subject, Eerie subjects, ordinary-number constituents,
predicate and Grave Sifter scope values check both roundtrip laws and exact
construction/leaf equality. Invalid independent singular-concord values fail
both admission and realization. Targeting, third-person concord, nominal scope,
Countability licensing and the original nominal noun head are preserved.

The new determiner licensing guard reads declared `DeterminerUse`, Number and
Countability; concord reads declared `QuantificationalDeterminer`,
`NumberTransparency` and `ObliqueNumber`. Word-named licensing guards added: 0.
Lexical source loading and GrammarEnvironment construction succeed without
workspace load errors. V3 emits no legacy permitted-licensing-checker total;
that retired statistic is not reported as zero or substituted for this audit.

All 34 removals have a singular NP Oblique where this quantificational use
requires plural. Baseline trees were independently replayed and inspected;
Equipment's unchanged singular/plural spelling does not excuse a singular
Reading. The coordinated cases improperly close the quantificational phrase
at singular Aura or instant before a higher Coordination. These are wrong
analyses retired, not coverage regressions; their remaining grammatical
Readings still cover every affected face. Hash prefixes below identify the
removed Readings in the full scratch identity manifest.

| Face identity | Card | Before → after Readings | Invalid Oblique | Retired Reading hashes |
|---|---|---:|---|---|
| `6f2ba142-5e7c-4709-ae03-e028cb2a9872#card` | Super-Soldier Serum | 56 → 40 | singular Equipment | `0cdd2dc83180`, `2855c6af8c21`, `2ee40b9e3cc5`, `44c6c1e1ab3e`, `4a1a58544570`, `5771f5aa5912`, `5be4b88edd45`, `85204ac99fb7`, `a06dff47854a`, `a3104b66d40c`, `b99d33a49406`, `be356362ab8d`, `cbbeaa157cb1`, `cd7b7a54f9fd`, `d456d9c32ef4`, `ffbd721d5d91` |
| `7424560f-557f-4bc9-a3e7-eb890c73aaa3#card` | Mantle of the Ancients | 72 → 60 | singular Aura | `08fcea7e3517`, `1082819aac0b`, `2f8d1566121d`, `498d0f99e90d`, `5caa9e590b2a`, `67466f4bb8b2`, `9132431bdc30`, `b0045e2588e5`, `b8a21cbe0f97`, `c675644f935b`, `f9707b7af418`, `f9f77343b456` |
| `99140891-face-4015-aacd-1309e87d8f9f#card` | Display of Power | 20 → 15 | singular instant | `03fe78ab22e7`, `4bc7a0052cbb`, `9af0edd53a2b`, `bc303a678dc7`, `c6e8191ba9ba` |
| `fa3335be-80b1-49d7-9809-5bef13be79f4#card` | Armory Automaton | 3 → 2 | singular Equipment | `551a95c51a12` |

### DISCLOSE — analyses, census and remaining causes

No new Construction or lexical owner was added. The noun's declared Number
and ordinary determiner/Countability licensing remain separate from NP
Agreement. Only the declared initial combination of quantificational *any*,
the existing eligible noun and a plural Of Complement supplies the obligatory
plural override. A missing Of Complement retains ordinary head concord;
a singular expressed Oblique is rejected in this use. Other determiners and
ordinary singular *the number* retain their existing behavior. No lot/couple,
collective, elliptical-oblique or additional quantificational-noun machinery
is included in this initial breadth.

PP summaries retain the Oblique's Number, Nominal projections carry its
eligibility/dependency through modification, and NP composition chooses its
Agreement. Nominal and PP coordination summaries retain these properties,
including the existing Coordination Kind in serial tails. Other PP categories
export no Oblique. An existing first Of Complement remains the concord source
when a later PP is attached; it is not overwritten by a subsequent modifier.

Exact pinned sets on `uxztsmxn` / covered 13,596:

- Sway subject: exactly 2 Readings before and after, using TargetNounPhrase or
  BarePlural(TargetedNominal) inside the separate Of Complement. Its nominal
  head is singular *number* in both; NP concord changes from Singular/Third
  to Plural/Third.
- Sway's constituent Clause ending at *choice*: 0 → exactly 2 Readings, the
  independently constructed subject alternatives with plural/third-person
  *become* and the nominal predicative Complement. Singular *becomes* admits 0.
  The complete two-line Document has 8 Readings across those subject
  alternatives and retained temporal-PP attachments.
- Eerie subject: exactly 5 Readings. Two attach the Object Relative Clause to
  the outer Nominal; three attach it within the Oblique, preserving the two
  Targeting realizations and placement around TargetedNominal. All five now
  have Plural/Third concord. Its already-covered complete face keeps all 90
  Reading identities.
- Magma Sliver's *the number of Slivers* and Pain's Reward's *any number*
  remain independently admitted singular constituents. An Of Complement on
  the ordinary noun *color* also retains head concord.
- Grave Sifter keeps all 5 complete-face Readings. Its independent high
  attachment of *of that type*, alongside the low attachment, preserves the
  first Oblique's plural concord.

Newly covered identities, with all admitted analyses reviewed:

| Face identity | Card | Readings | Analysis |
|---|---|---:|---|
| `9a7e2298-8855-43a7-8cb1-4b31e4058c3f#card` | Sway of Illusion | 8 | Singular nominal noun head; plural target-creature Oblique; obligatory plural/third-person finite predicate; nominal *the color of your choice* Complement; retained temporal PP scopes and final imperative. |
| `23bf0648-7097-41df-a539-1d36ac42cf9c#card` | Depthshaker Titan | 28 | Initial temporal Clause; singular nominal noun head with plural target noncreature-artifact Oblique and retained Object Relative Clause scopes; plural/third-person finite *become* with slash-premodified nominal Complement; subsequent imperative and independent artifact-creature predication retained. |

All 36 new Reading trees and their owned words were inspected, including
plural/third-person *become* in each. They are grammatical Oracle analyses,
not negative witnesses or a label-fallback coverage gain. Optional preference
never decides admission; no Construction Cost changed.

| Census | `yvwxwoxu` / covered 13,594 | `uxztsmxn` / covered 13,596 |
|---|---:|---:|
| No Reading | 19,234 | 19,232 |
| Unique Reading | 6,647 | 6,647 |
| Multiple Readings | 6,947 | 6,949 |
| Retained Readings | 157,573 | 157,575 |
| Undetermined / validation issues | 0 / 0 | 0 / 0 |

The inherited unique/specificity-resolved census is superseded by retained
Readings; there is no destructive specificity selection or pair to report.

On `uxztsmxn` / covered 13,596, 522 supported stripped-text faces contain
*any number*: 59 are covered and 463 still have no complete Document Reading.
Of 172 containing *any number of target*, 21 are covered. These are overlapping
surface cohorts, not promised gains or grammatical cause counts. Among the
463 residual faces, 105 have vocabulary gaps; the remaining 358 need further
grammatical diagnosis. They remain routed to `english-v3-systemic-residuals`.
The ticket does not infer whole-face success from constituent recognition.

Confirmed independent remaining causes: Colossal Heroics' distributive *each*
between the subject and finite predicate yields 0 Clause Readings, while
*get +2/+2 until end of turn* independently has 5 finite-predicate Readings.
Magma Sliver's scalar subject *X* has 0 Noun Phrase Readings despite recognition
in scalar Categories. Pain's Reward still has vocabulary gaps including bid,
bidding, bidder, high, stands and start. These witnesses remain residuals on
the measured tree; their ordinary-number constituents are fixed neither by
inventing a whole-face gain nor by weakening Agreement.

Deviations and additions:

- Constructions added/deleted: 0/0; refinements use existing noun, PP, NP and
  coordination schemas, plus general declared feature tables and policies.
- Added the independent Grave Sifter attachment witness after a preliminary
  implementation overwrote the first Oblique with a later modifier. Its
  failing test caught the regression; the composition fix restores both
  removed preliminary Readings. That intermediate defect was never landed.
- Tests: restored 0, re-spelled 0, added 6, removed 0, newly ignored 0.
  Existing ignored tests/documentation examples retain their blockers.
- Added Number Transparency and Oblique to the owning glossary from full
  CGEL Ch. 5 §§3.3 and 18.2, pp. 349–352 and 501–504. No glossary gap remains.
- STOPs: none; no contradiction with a recorded ruling was resolved silently.
  Unrelated formatting-only edits were restored before the final review.

### REPORT — provenance and performance advisory

Both measured declaration states have 170 ordinary Constructions, 44 shared
schemas (214 named Construction families), 308 schema category instances,
530 static Productions and 586 environment-compiled Productions. The final
declaration has 2,692 physical lines. No breadth count increased. The V3
covered count above is provenance; no retired-parser coverage lock was changed.

The complete homograph inventory is unchanged on both measured states:
`'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.

Form-literal/vocabulary overlap inventory: empty on both measured states.
No newly declared surface or lexical owner changed either inventory.

| Tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `yvwxwoxu` / 13,594 | 8 | 23.77/14.32/12.91 | 306,626,947,192 | 1,059,974 ns/B |
| `uxztsmxn` / 13,596 | 8 | 33.93/26.87/19.35 | 417,683,806,606 | 1,148,250 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling. Wall time
rises from 306,627 ms to 417,684 ms; per-byte thread CPU rises from
1,059,974 ns/B to 1,148,250 ns/B. Different host load, overlapping local checks
and the expanded grammatical summaries limit attribution. This advisory routes
to `english-v3-census-tractability`; it is not a gate or a fitted target.

Verification:

```text
cargo xtask gate --changed --clippy --run
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
cargo xtask english-v3 --all --workers 8 --samples-per-face 0 --output /tmp/number-transparent-final.json
```

The derived test closure passes: 658 passed, 0 failed, 2 pre-existing
ignored across 73 test/doc-test suites. Strict clippy passes with warnings
denied. Scoped nightly formatting checks pass on both changed Rust files.
Citation checks report 0 noncompliant strings and 0 stale citations; the piped
diff audit has no new Comprehensive Rules sites, so no blessing is needed.

Scratch provenance: `/tmp/number-transparent-before.json`,
`/tmp/number-transparent-final.json`, `/tmp/number-before-identities.jsonl`,
`/tmp/number-final-identities.jsonl`, `/tmp/number-transparent-delta.json`,
`/tmp/number-baseline-audit.jsonl`, `/tmp/number-transparent-gains.json`,
`/tmp/number-before-inventory.json`, `/tmp/number-final-inventory.json`,
`/tmp/number-transparent-subset.json`, `/tmp/number-transparent-grave-sifter.json`,
probe JSON files, red/green logs, and final gate/citation/format logs.
