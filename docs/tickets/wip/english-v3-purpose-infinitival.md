---
needs: []
---
# Read the purpose *to*-infinitival Adjunct: mana spent to cast this spell

## Why

The ungoverned purpose infinitival never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), the recon buckets
"spend / spent" and "purpose infinitival (pay/spend/tap … to <verb>)" read
together touch **483** unread faces and are the sole cause on **287**: 154 of
those contain *spent to cast*, 108 *only to* (*Spend this mana only to cast
…*), 25 other. Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Pay {1} to draw a card." 0; "You may pay {2} to draw
a card." 0; "Sacrifice a creature to draw a card." 0; "Draw a card to gain 1
life." 0; "Spend this mana to cast a creature spell." 0, while "Spend this
mana." 1; "You gain life equal to the amount of mana spent to cast this
spell." 0, while "… equal to the amount of mana spent." 1.

## Goal

A *to*-infinitival without a governing preposition reads as a purpose Adjunct
of the clause (*Pay {1} to draw a card*) and of the reduced passive in an NP
Postmodifier (*the amount of mana spent to cast this spell*, *each color of
mana spent to cast it*), with its missing Subject left to the grammar's
existing control treatment. With `english-v3-focusing-only` landed, *Spend this
mana only to cast …* composes through its focusing Modifier; measure those
faces, and if focusing-only has not landed, record them as owed there.

## Analysis

The ungoverned infinitival is the most frequent type of purpose adjunct (*We
left early to miss the rush-hour*); ungoverned infinitivals are always
positive, a negative needing a governing preposition (CGEL, Ch. 8, §12.2,
p. 728, [9i]). Purpose elements are usually adjuncts (p. 728). The existing
`ToInfinitive` Construction is a Complement; whether the Adjunct reuses its
internal structure is for the landing to settle and record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Molten Note: "Molten Note deals damage to target creature equal to the amount
  of mana spent to cast this spell." (control without *to cast this spell* 1)
- Skyrider Elf: "Converge — This creature enters with a +1/+1 counter on it for
  each color of mana spent to cast it."
- Infuse with the Elements: "Converge — Put X +1/+1 counters on target creature,
  where X is the number of colors of mana spent to cast this spell."
- Scaled Nurturer: "When you spend this mana to cast a Dragon creature spell,
  you gain 2 life." (control "When you spend this mana, you gain 2 life." 1)
- Angel of Jubilation: "Players can't pay life or sacrifice creatures to cast
  spells or activate abilities." (control without the infinitival 1)

Composition check, not a gain claim before focusing-only: Piracy, "Spend this
mana only to cast spells."

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
   Reading. The infinitival read as an Object or catenative Complement
   of a verb that does not select one, or as a relative Postmodifier of the
   noun, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Finite passive *was spent* with a measured mass Subject (*If at least four
  mana was spent to cast it*): "If three or more mana was spent, draw a card."
  is 0 without any infinitival; note those faces.
- *Spend … on X* (*Spend only black mana on X.*), cost-additional quoted
  costs (Drought), and *in order to*.
- Focusing *only* itself: `english-v3-focusing-only`.

## Record requirements

In addition to the standard record: the attachment of the purpose Adjunct per
host (clause, reduced passive Postmodifier) and its relation to
`ToInfinitive`; the *only to* faces measured with or without focusing-only on
the base; Reading counts of each witness; before/after counts stamped with
change ids; timings as integer ns and ns/B with host load and worker count.


## STOP and resolution (2026-10-08)

The initial crate tests exposed two cost-scalar contract regressions and the
causative-have negative control admitting a possessive Reading plus purpose.
Cost now declares PurposeAdjunctLicence=No under its existing project analysis.
The causative test remains unchanged and must retain zero Readings.

The orchestrator ruling (2026-10-08) excludes purpose Adjuncts on stative
possessive predicates in Oracle English, through declared licensing, with
corpus attestation checked before extending the exclusion. This resolves the
STOP between the test and the broader general-English analysis. The ruling's
agentive/dynamic restriction is a **project ruling**. CGEL Ch. 8 §12.2,
pp. 727–729 does not state a blanket stative prohibition: p. 729 [13v] admits
possessive *has* with purpose explained by intended design/building. The passage
backs the positive ungoverned infinitival and non-obligatory syntactic control;
it does not back Oracle's exclusion. No test was removed or re-spelled.

## Landing record

### Provenance and census

Measured feature change **`zxumsproyqltrnoqxvxwkqzruxyzturu`, covered 22,087**.
Claim parent: `ttkxuvuouuospwttspkrzvulrkrvnrqq`. Its grammar, runtime and
lexical sources match the complete census measured on
**`qsrqtwknwxqtrpyrmknwollqqrrnssyo`, covered 21,916**;
`baseline-source-proof.json` records zero source-hash differences. The
claim-parent comparison uses that census, not the older recon in the ticket's
Why section. Both censuses use the same pinned input SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`
and the same 32,828 supported face identities and analyzed source bytes.

The finished feature's **one full run**, without incremental reuse:

```sh
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-purpose-infinitival/after.json
```

| Census | Before: qsrqtwkn, covered 21,916 | After: zxumspro, covered 22,087 |
| --- | ---: | ---: |
| Supported faces | 32,828 | 32,828 |
| No Reading | 10,912 | 10,741 |
| One Reading | 7,516 | 7,540 |
| Multiple Readings | 14,400 | 14,547 |
| Exact Readings | 943,065 | 984,376 |
| Newly covered | — | 171 |
| Lost faces / Reading-count decreases | — | 0 / 0 |
| Limited / failed / undetermined | 0 / 0 / 0 | 0 / 0 / 0 |
| Validation issues / internal failures / duplicate Readings | 0 / 0 / 0 | 0 / 0 / 0 |

`comparison.json` compares every identity's count and `readings_sha256`.
**Every previously covered face retains exactly its count and digest**;
all 32,657 faces outside the gain set have unchanged digests. No removed
Reading, retirement, re-coverage obligation or unexplained loss exists.
The 171 gains comprise 24 unique and 147 multiple-Reading faces. No existing
unique face became ambiguous. V3 retains all grammatical Readings; there is
no destructive specificity selection or unresolved-tie gate to report.
The additional alternatives concern purpose attachment to a dynamic predicate
versus its embedded predicate, and coordination scope, rather than two names
for one constituency.

Evidence is under `target/english-v3/english-v3-purpose-infinitival/`:
`before.json`, `after.json`, `comparison.json`, `gain-analyses.json`,
`spot-checks-landing.json`, `stative-scan.json`, `share-scan.json`,
`inventory-parent.json`, `inventory-final.json`, and the verification logs.
V3 has no coverage lock: the complete census supplies its covered count.
These figures and inventories are stamped with the measured change and its
covered count above; they are provenance, not targets or gates.

### Analysis and structural evidence

`PurposeInfinitival` is the positive ungoverned Clause used as an Adjunct.
It reuses `ToInfinitive` with `BarePredicate`, in a separate Category from
selected `InfinitiveComplement` and `HollowInfinitivalComplement`. Neither
selected Complement route is superseded or removed. `PurposePredicate`
attaches it to finite and secondary predicates, including the bare Passive
inside a participial NP Postmodifier; object-gap predicate instances preserve
the same distinction. The focusing construction composes with that Clause:
*only* modifies the purpose constituent, not the main verb or its Object.

The full census validates declaration admission, lexical ownership/context,
byte-exact realization, and construction/word traversal identity for all
984,376 Readings. The independent construction test checks the reverse
constructed-value roundtrip law, exact parse-set equality and both traversals
for the purpose Clause, its passive host and its focused form. Corpus
validation alone is not a proof of linguistic correctness or the reverse law;
the witness tests and tree reviews supply that evidence.

There is one new permission requirement,
`head.PurposeAdjunctLicence = Yes`, reading a declared feature. New
word/identity-naming licensing checkers: **0**. The licence is carried through
modifiers, gaps and coordination (both conjuncts must license it); auxiliary
predicates inherit the licence of their selected predicate. Source loading
succeeds, with no new load error or diagnostic. The **87 inherited source
mapping/morphology diagnostics are byte-identical to the parent**, named in
`source-diagnostics.json` and both censuses; they are not silently described
as zero. Their remaining source work belongs to
[english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md).
The v3 census does not emit the retired parser's permitted-checker total;
none is invented here, and the derived gate is not run.

### Stative-host ruling and corpus scan

The STOP and its orchestrator resolution are recorded above. CGEL Ch. 8
§12.2 pp. 727–729 supports the ungoverned positive infinitival and the
possibility of interpreting its missing Subject from a participant or
situation. Its possessive example on p. 729 [13v] is explained through the
intended design/building; the passage does **not** impose Oracle's blanket
stative restriction. The **orchestrator ruling of 2026-10-08** supplies that
Oracle-register restriction, motivated by the agentive/dynamic situation
whose purpose is explained.

The supported-corpus scan finds **0 possessive-have purpose attachments**.
It reviews 78 sentence/semicolon-segment *have/has … to <declared verb form>*
candidates (56 *have*, 22 *has*), separating quoted granted abilities,
causative clauses, independent/subordinate clauses, purpose inside an NP,
and the homographic *to target* Preposition Phrase. The scan covers all
32,828 supported faces. *Remain/remains* has 23 candidates and **0** such
attachments. Copular *be*, *own* and *control* also have **0** stative purpose
uses; the expanded named candidate inventory is in `stative-scan.json`.
*Share* has nine surface candidates, all separate clauses, prepositional
attachments or purpose inside a different predicate, and **0** stative
purpose uses (`share-scan.json`). Applying all exclusions loses **0 faces**
and removes **0 parent Readings**, as the full census establishes.

The declarations deny stative permission to Have, Be, BeNegative,
BeContracted, Remain, Own, Control and Share. The Have declaration also denies
outer attachment on its causative frames, for which the scan likewise finds
no attested purpose-host use; a purpose inside the selected dynamic predicate
remains possible. Perfect and participial auxiliary uses inherit their
selected predicate's licence, preserving dynamic *have sacrificed* and
*was spent* purpose uses. Cost denies permission under its pre-existing
cost-scalar project analysis. No grammar guard names these lexemes.

### Witnesses and reviewed gain trees

All six document witnesses had zero Readings on the claim parent. After:
Molten Note **46**, Skyrider Elf **66**, Infuse with the Elements **20**,
Scaled Nurturer **1**, Angel of Jubilation **69**, Piracy **7**. The test
fragments retain the expected purpose constituent in every Reading. The
*only to* inventory contains **163 faces**: covered **0 → 92**, with the
already-landed focusing implementation on both measured trees. This is a
full-corpus substring inventory, distinct from the original recon bucket.

The following **13 newly covered faces** were inspected against the ticket's
defect list. Each has a genuine purpose Clause with a saturated BarePredicate;
it is neither an unselected Object/catenative Complement nor an infinitival
relative NP Postmodifier. In the passive NP cases the purpose modifies the
embedded predicate. Sample fingerprints and exact reviewed subtrees are in
`spot-checks-landing.json`. Full-census Reading counts match the selector
reports, which retain trees; the full report itself retains zero samples.

| Face | Readings | Reviewed purpose host and analysis |
| --- | ---: | --- |
| Guidelight Optimizer | 3 | Spend; focused purpose with cast/activate coordination alternatives |
| Karfell Harbinger | 18 | Spend; focused purpose with foretell/cast alternatives |
| Soldevi Machinist | 1 | Spend; only to activate abilities of artifacts |
| Interplanar Beacon | 2 | Spend; only to cast planeswalker spells |
| Ancient Ziggurat | 1 | Spend; only to cast a creature spell |
| Piracy | 7 | Spend; only to cast spells |
| Scaled Nurturer | 1 | finite Spend in the when-Clause; to cast a Dragon creature spell |
| Angel of Jubilation | 69 | Sacrifice and coordinated Pay/Sacrifice hosts; cast/activate alternatives |
| Molten Note | 46 | bare Passive Spend in the amount-of-mana NP Postmodifier |
| Infuse with the Elements | 20 | bare Passive Spend in the where-supplement NP |
| Skyrider Elf | 66 | bare Passive Spend in the each-color-of-mana NP |
| Gluntch, the Bestower | 4 | Choose a player; to draw / to create, with matrix Object as possible antecedent |
| Swallow Whole | 6 | bare Passive Tap in the creature NP Postmodifier; to pay the additional cost |

Swallow Whole also retains grammatically distinct higher dynamic-Put
attachments. Those Readings are disclosed rather than destructively selected
away: the current retained-Reading contract admits grammatical game-semantic
nonsense. The reviewed witness uses the intended bare-Passive Tap attachment.
The same distinction applies where a higher dynamic predicate and an embedded
predicate both admit purpose. The stative exclusions are the explicit ruling,
not an inference that every semantically unusual Reading should be pruned.

### Deviations and additions

- Added one schema, PurposePredicate, with four instances; added one
  PurposeInfinitival instance of ToInfinitive. The distinct adjunct function
  is the ticket's requested construction, not a replacement selected frame.
- Added one focusing instance for PurposeInfinitival. This composes the
  already-landed focusing construction with the attested Clause constituent;
  the previous PP/AdvP citation is not claimed to establish Clause focusing.
- Added finite and bare object-gap hosts beyond the listed witnesses so
  purpose in relative predicates keeps structural gap identity rather than
  a second lexical route. Existing tests and all parent digests remain intact.
- Added declared Cost permission and propagated licensing to repair the two
  cost-scalar regressions. Its hollow Complement remains required; both
  lexicon and grammar retain only their intended routes.
- Applied the ruling to copular Be (including negative/contracted spellings),
  Remain, Own, Control and Share, beyond the possessive-Have negative control.
  Each stative family has zero purpose attestation and the combined exclusions
  lose zero faces and zero parent Readings. Auxiliary permission follows the
  selected predicate, so these data declarations do not suppress dynamic
  passive/perfect purpose hosts.
- Added the twelve purpose integration tests, under tests/suite and registered
  in tests/main. Assurance counts: **3 restored** existing regressions;
  **0 existing tests re-spelled**, **0 ignored**, **12 added**, **0 removed**.
  The new auxiliary test initially used an unsupported mana-symbol Subject;
  that document remains explicitly zero, while its finite predicate is tested
  independently. No Subject-support feature was added to make it pass.
- Filled two glossary gaps: Purpose Infinitival and Purpose Adjunct Licence,
  with CGEL support for the former and project-ruling provenance for the latter.
- No superseded purpose route existed on either side. No selected lexical
  frame was added or deleted, and no selected Complement declaration was
  left unreachable by a purpose replacement.

Measured declarations on zxumspro, covered 22,087: **202 ordinary
constructions + 50 schemas = 252**, **335 instance rows**, **712 compiled
productions**, **147 Categories**, **3,503 nonblank declaration lines**.
Parent qsrqtwkn, covered 21,916: 202 ordinary constructions + 49 schemas =
251, 329 instance rows, 706 productions, 146 Categories, 3,414 nonblank lines.
The existing tree already exceeds the decision's eventual economy ceilings;
this landing adds one schema for a grammatical function distinct from selected
Complements and does not treat those ceilings as a fitted coverage gate.
Observed construction-name inventory: **212** on the final census.

### Verification and performance advisory

- `cargo test -p deckmaste_english_v3`: **406 passed**, zero failed/ignored;
  unit/doc suites have zero tests. See `tests-landing.log`.
- `cargo test -p deckmaste_lexical_source`: **23 unit + 33 integration passed**,
  zero failed/ignored. See `lexical-landing.log`.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  passed. See `clippy-landing.log`.
- `cargo xtask cite check --list-noncompliant`: empty; `cargo xtask cite check`:
  **16,333 checked, 0 stale**; `jj --no-pager diff --git | cargo xtask cite audit --diff`:
  **0 changed CR sites**. No CR citations were introduced or blessed.
- Selector iteration used explicit identities only; the last pre-full selector
  has 2,462 faces, 171 gains, zero losses/decreases/issues. No xtask tests,
  Lean checks or derived gate were run.

Performance advisory, integer ns, **12 workers** on both measured trees:

| Tree stamp | Setup ns | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1/5/15 min) |
| --- | ---: | ---: | ---: | --- |
| qsrqtwknwxqtrpyrmknwollqqrrnssyo, covered 21,916 | 2263243075 | 136415517053 | 532155 | [15.26708984375, 13.7666015625, 11.27001953125] |
| zxumsproyqltrnoqxvxwkqzruxyzturu, covered 22,087 | 2360945390 | 125368987248 | 385487 | [1.86181640625, 4.623046875, 6.13671875] |

Final coverage-command wall, including setup, is **127729932638 ns**;
corpus wall alone is **125368987248 ns**. Both exceed the historical
quiet-host ceiling of **16260000000 ns**. This is an advisory on the measured
host and workload, not a gate or a quiet-host equivalence claim. Parent
command wall including setup is **138678760128 ns**. Thread CPU is reported
per checked text byte, not wall time divided by corpus bytes.

Lexical inventories (final zxumspro, covered 22,087; parent qsrqtwkn, covered
21,916): **38,392 spellings**, **1,139 named homographs**, **1,092 case-folded
homographs**, **23 form literals**, **0 form-literal/vocabulary overlaps**.
Every named spelling-owner map, homograph list and literal list is identical
before/after, recorded in inventory-parent.json and inventory-final.json;
the overlap list is **[]**. Lexical inventory SHA-256 changes from
`b0b0b7a98db683ad92f7c61e8c98791fbced1d61b5e04882b685d9bf8b57018f`
to `05876f19fd5d2be92f55ce5eddaa4c51d5f113218e9728f27814c9f45ba9b09f`
because of declared permissions, not new vocabulary or surface forms.

### Remaining out-of-scope faces

Measured mass Subjects remain unsupported independently of the infinitival.
The following **36 still-unread identities** match the declared measured-mass
pattern; their sources and IDs are in measured-mass-residuals.json. They are
not losses and are routed to
[english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md):

- The Emperor of Palamecia — `042c01ec-9f1f-432b-bea9-d64d822cd91e#face:0`.
- Blazing Bomb — `088acd15-029b-4091-960a-d5060a364ca9#card`.
- Rally for the Throne — `0a557383-596b-4b8f-b81f-21706fef7b63#card`.
- Cauldron's Gift — `152bbb62-195b-4de1-90c6-10dd953f8c4b#card`.
- Outmuscle — `185d84f4-6095-46ef-917f-3d662a86aa81#card`.
- Vantress Paladin — `1965ca0e-1357-4437-82d3-1a78353ab5f1#card`.
- Tellah, Great Sage — `1a080b26-c766-4f7c-9198-4974d2133f75#card`.
- Slaying Fire — `1e94e647-9150-4b22-aa9f-d195f64fb20a#card`.
- Sahagin — `24310f49-2156-4c0d-b3b1-40ca3182474f#card`.
- Searing Barrage — `26c17025-1123-46d9-a8bd-edbad01c4b6e#card`.
- Tackle Artist — `2e80d632-cf84-4562-8c93-5850cceb5bec#card`.
- Ardenvale Paladin — `30393cb0-f461-4899-8aae-609bc8378f36#card`.
- Muse Seeker — `31f3d37f-e613-427f-b34c-cc87611ea078#card`.
- Foreboding Fruit — `33a162f3-e781-494f-ab8f-12a84801a694#card`.
- Garenbrig Paladin — `33fdbff6-0e89-4f02-b429-9b17869134cd#card`.
- Deluge Virtuoso — `416c70ab-f136-43e0-b5d5-9af8d4211c01#card`.
- Thunderdrum Soloist — `43b5794a-4ed6-471d-bc45-22fac486895d#card`.
- Colorstorm Stallion — `47ee6837-2e2f-4344-ab9f-6b8887874f77#card`.
- Molten-Core Maestro — `485233bf-dc4b-4f31-8dfb-5054aee70c08#card`.
- Spectacular Skywhale — `6a6b19db-cdeb-4922-8642-5d872f90e7a1#card`.
- Locthwain Paladin — `6fea3c95-9663-4857-983e-eaf522efa1a8#card`.
- Elemental Mascot — `882f30e4-4ce1-4581-8d2d-d763aa13befe#card`.
- Sundering Stroke — `9c348bc7-e6e1-40a2-9798-6d2c4a2d8e72#card`.
- Unexplained Vision — `9f693483-f4df-497a-a6c7-fc269fc60ea8#card`.
- The Prima Vista — `a09d9754-a321-4018-a237-e613e915b642#card`.
- Raggadragga, Goreguts Boss — `a7641ee0-9c0d-4201-9827-a775ed03797c#card`.
- Ultros, Obnoxious Octopus — `b6f4b524-1081-4263-b7e4-9aee93131b31#card`.
- Paragon of Modernity — `c1c62704-1f20-48cf-8588-47e7fca85f7b#card`.
- Desecrate Reality — `c9cd57f6-1f69-4ea1-9672-ca944759c7e8#card`.
- Turn into a Pumpkin — `cfda6f65-3755-40e5-8061-a21941c4b1bd#card`.
- Exhibition Tidecaller — `d32c9439-4c97-412b-83cc-6d9acb48be3c#card`.
- Silverflame Ritual — `d511573b-35c7-4934-aa46-906e14101664#card`.
- Prompto Argentum — `e13d7600-0924-42eb-9d65-aa18548624a7#card`.
- Embereth Paladin — `eec31fe0-a633-4424-84d2-90fe88016310#card`.
- Expressive Firedancer — `f1812d69-c016-4bc3-b899-ed47f5e893e5#card`.
- Once and Future — `f666c38d-e601-4aa9-b140-4b6607bb12dd#card`.

*Spend … on X*, Drought's quoted additional costs and *in order to* remain
outside this feature. No claimed gain is a replacement for those obligations.

### Every newly covered identity

All entries were **0 → the listed count**. The representative retained
Reading is specified by its purpose host; exact fingerprints, lexical owners
and attachment ancestry are in gain-analyses.json. Every listed Reading's
count and digest agree with the full census. Focusing applies to the purpose
Clause, and a bare Passive in an NP Postmodifier remains the NP's embedded
predicate rather than an infinitival relative modifier of its noun.

| Newly covered face / identity | Readings | Representative retained analysis |
| --- | ---: | --- |
| Aberrant Manawurm — `a0657586-0404-4bb0-a92d-25f040d560c1#card` | 24 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Abstract Paintmage — `bc899d5c-0a91-4f19-a3e4-3078ae966110#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Abundant Countryside — `e3eb6f90-ccfc-41e7-bff6-0b378226bc7e#card` | 7 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Ally Encampment — `9d293b69-12b7-4b50-a0a7-c4f493dee30b#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Ancient Ziggurat — `0baabe39-72ae-47bd-a095-cbf7eb8a6361#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Angel of Jubilation — `36c2fc4c-7d8b-47c2-91c3-e356df60b219#card` | 69 | Purpose Adjunct of SelectedPredicate lexeme:keyword_action/sacrifice |
| Animal Attendant — `787e6cc6-d97a-4229-97d1-ec2377d138d3#card` | 24 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Arcane Omens — `863cbd12-da20-456e-8698-06fba4d9bd5c#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Asmoranomardicadaistinaculdacar — `1f74e26f-5cf3-4b0e-bcc6-f858599474fc#card` | 108 | Purpose Adjunct of BareAuxiliaryPredicate core-verb:May |
| Automated Artificer — `600e1093-5619-49b3-b193-bd263e4fde33#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Battery Bearer — `de6fd93e-16d8-4855-a655-1cc0755c5d15#card` | 18 | Purpose Adjunct of BareAuxiliaryPredicate core-verb:Cant |
| Beastcaller Savant — `e28227eb-b3b5-42fa-b597-1fefd2a70186#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Biophagus — `7c06a368-3306-48e0-825e-a8fdc81fa343#card` | 24 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Blessing of Frost — `240d650b-97ea-40ad-8c1b-fe1ea52d861f#card` | 161 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Blood on the Snow — `45f386ba-ad62-4c0e-b6ad-ae02ca8f2bec#card` | 23 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Boommobile — `cb3edb86-ddd5-461c-98b5-ffb4df437a39#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Brilliant Spectrum — `88ed66fc-7068-44a7-b00f-3be22fb500cb#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Bucolic Ranch — `8f5902bf-4bc4-4d0c-84ea-a425307a4eb2#card` | 10 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Cargo Ship — `93e336e3-2be7-4581-8ab3-9bea422d81c3#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Cavern of Souls — `89ca686a-7c72-4d8f-9290-e89635624a83#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Chamber Sentry — `65f8e726-be50-49ee-96fc-d3a2080d7b8c#card` | 99 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Chandra's Embercat — `e75bec5c-538d-44ce-a6c7-85580d04fac8#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Cormela, Glamour Thief — `3cd9bac9-0abc-43d8-9f84-94f965f7a2e0#card` | 16 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Corrupted Crossroads — `6276a985-7630-476d-a94f-6c6adc88f6c4#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Crucible of the Spirit Dragon — `ecfbebc9-7fc7-474e-8c59-8ede800e082e#card` | 18 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Cryptic Trilobite — `28c0fd21-a88a-4a40-853f-2c1810517bc3#card` | 5 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Crystalline Crawler — `5ea3e672-ae14-49a4-badb-f713497dd126#card` | 66 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Dalakos, Crafter of Wonders — `211814a5-195d-41c6-b96c-4c26b2cbb33c#card` | 6 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Delighted Halfling — `f9d3b046-0b95-4103-a630-4b3fb88bb60b#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Desolation of Smaug — `c9634afc-4a5b-4cf6-b63d-0ff9909dd5a7#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Devour Intellect — `abf8264e-d020-4939-8092-83469b1a2244#card` | 24 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Dire Fleet Daredevil — `7a3e50a5-c163-4c41-b62d-d52c233c55b7#card` | 288 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Dyadrine, Synthesis Amalgam — `618a2272-62e0-4d0e-834d-7b81719eba12#card` | 171 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Eldrazi Temple — `7fab8d65-af51-47d3-8f10-2676bf6e8ba3#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Elfhame Druid — `519fccdb-fde4-42cb-b2ca-35fc36cf984a#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Exert Influence — `feb9cce1-6c3c-4e04-93ee-d646a888bc29#card` | 30 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Flamebraider — `c552bd73-ce75-40fd-9d80-9e90b6f45274#card` | 12 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Freestrider Commando — `09117016-5fd1-4590-9867-aad65d3097e3#card` | 30 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Fíli and Kíli, Joyous — `392f6cd3-3895-42b3-8bb9-6b0c6a689a29#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Galazeth Prismari — `f9217532-f7cf-4436-b476-72d97a3147dc#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Gallifrey Council Chamber — `26e4b49e-77e7-41d9-94c5-924669a82591#card` | 12 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Geosurge — `6ec2794f-db1a-4448-a75e-ed7aff565378#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Gideon's Memorial — `c8f08704-7330-4fa9-b456-2e96aa252a2c#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Gilanra, Caller of Wirewood — `4bda947d-493f-47a0-b0d2-516a6ef6b2c0#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend |
| Glinting Creeper — `0e252f94-7731-45e1-bdbb-1bd11441577e#card` | 2112 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Gluntch, the Bestower — `0222dc7c-459b-4909-a037-72b2eb248599#card` | 4 | Purpose Adjunct of SelectedPredicate core-verb:Choose |
| Gnarlroot Trapper — `d3cc22ea-2ccf-409b-b471-51adb63c7fda#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Grand Architect — `7af1d7f9-058a-4ccb-96b7-fc391b615c09#card` | 54 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Graven Lore — `0d9df234-520c-40aa-a37d-070181cf695c#card` | 2 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Great Hall of the Citadel — `b3e28bcf-0ed0-4406-b615-68ddc55b349a#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Guidelight Optimizer — `045488b3-0206-4dcd-91ab-59ec542f06c0#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Gwenna, Eyes of Gaea — `7d95e72a-461e-4993-a190-847466a4b17c#card` | 216 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Haven of the Spirit Dragon — `acc9c16a-5e72-43bd-87e1-56a16aa892f5#card` | 33 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Herd Heirloom — `78b6dd40-c182-4037-a4d3-9fd012b2c584#card` | 6 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Hermitic Herbalist — `9928773d-3f35-4b0e-965d-4233e04398fb#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Hired Hexblade — `f3a0f155-05d8-465c-b0ef-35aa12e93013#card` | 6 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Hogaak, Arisen Necropolis — `97acd977-164d-49d5-b3a1-72e0d75c9016#card` | 8 | Purpose Adjunct of BareAuxiliaryPredicate core-verb:Cant |
| Humble Naturalist — `ff3a69a7-a244-4958-9e8b-d153942c1ba6#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Hydraulic Helper — `503928cd-1abd-44cd-92c3-6dc19bb07f78#card` | 3 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Hydro-Channeler — `95630a83-3699-4340-b3e5-17e81db1b82f#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Infuse with the Elements — `beea5db2-1771-417f-9a31-028b3d31aa66#card` | 20 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Interplanar Beacon — `073169f2-da3a-4a93-8c01-b3fd8558d225#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Intrepid Stablemaster — `a825bdea-3ea2-46b2-93b9-21586643aab0#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Ixalli's Lorekeeper — `a3ce0fa9-8742-4e2c-a8a0-00fc009f998a#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Jade Orb of Dragonkind — `5eef3d70-ef16-4722-8c3d-21a0311597bd#card` | 12 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend |
| Jaded Sell-Sword — `f666a990-bd9c-4160-9770-4758fd40ca92#card` | 8 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| James, Wandering Dad — `8c90b558-de07-4689-9427-da68d823c2e8#face:0` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Jasmine Boreal of the Seven — `138201c0-556d-4c76-b383-90fc9dad0fe0#card` | 320 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Jasmine Dragon Tea Shop — `d9a24444-289f-473f-9985-8df275257555#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Jetmir's Fixer — `f87df7d4-99e2-4340-9d0a-b96c397555db#card` | 12 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Kaleidoscorch — `b02bd695-ec7f-47be-b797-e2b4b309cc2a#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Karfell Harbinger — `0507a9d2-cbad-4a7b-b888-fbd5ac1ed02f#card` | 18 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Karolina Dean, Runaway — `50b5faff-acaa-426b-9f3e-1005c0392e6c#card` | 60 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Kurbis, Harvest Celebrant — `991fab9c-8554-4e64-a933-6cc2ffa4edb1#card` | 16359 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Lapis Orb of Dragonkind — `c61ab8e1-ef23-465d-bcd3-f771434988b2#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend |
| Leyline Immersion — `aa6c4079-72e1-4b2e-bd26-9399298c6105#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Lidless Gaze — `287aae84-b098-456e-b9cb-8eac51c36360#card` | 24 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Lilypad Village — `5bb06e6f-e3af-4caa-b66d-77248ad46b61#card` | 7 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Lord of the Forsaken — `3a25d1e2-63d8-4a93-b0ce-f9307736b3f9#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Maelstrom of the Spirit Dragon — `49e9fba7-8465-4bbb-95db-73a7e149f494#card` | 7 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Magmablood Archaic — `c6a5adc8-ee57-421e-8235-0b3160f44568#card` | 3696 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Mana Sculpt — `35e2f82e-7ca3-4a92-9134-b7999eef5337#card` | 126 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Manaform Hellkite — `f426aa1a-e1dc-4563-b450-2e23d6bd980b#card` | 72 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Master of Dark Rites — `a65250f6-55e2-47ed-a355-d690047db6f2#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Mastermind Plum — `4605d8b3-b714-4717-8ff3-155fabfc1999#card` | 120 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Mech Hangar — `abc04775-171d-41f3-83ea-4b4eb72723d5#card` | 8 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Memory of Toshiro — `0a708566-2994-47b3-8d89-0547cc115e7c#face:1` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Mishra's Workshop — `ba284fe6-bb29-455c-8321-9714a0cdc05e#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Molten Note — `8c179b8f-3a83-49d6-9e7f-9e17c293ccbb#card` | 46 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Mudflat Village — `aeeab1df-0b8b-4bc4-a5f9-aac413449bec#card` | 12 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Myr Reservoir — `82b327c6-5cd9-4bee-bef4-860522bded9c#card` | 15 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Nix — `fe1a9af0-1774-4d54-b59f-f42dc8afcbe9#card` | 5 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Oaken Siren — `a06a35a9-a23e-4a0a-90bc-84e7a52d2cef#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Oakhollow Village — `177b7fe4-8565-4631-b4d9-8b2b4282f3ac#card` | 6 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Obsidian Obelisk — `92a5e50d-8477-4654-9c1b-219e4fe3c5ab#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Omen Hawker — `215fd0b4-5817-4335-a0ed-1694e44bde41#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Open the Omenpaths — `9ac01998-6e26-4ba5-b66c-614ec831caef#card` | 14 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Painful Truths — `58a2d15f-1b4b-4303-b524-8cc0d1b3e7d2#card` | 20 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Pelargir Survivor — `ce0b3c22-3f77-4a6c-a126-9ed3c06b4482#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Pillar of Origins — `9e62f30d-3130-416e-b459-48b5f71f8be9#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Pillar of the Paruns — `677b8ce7-f922-4ee3-b311-f199da9b352b#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Piracy — `131acf69-fa5c-4ce7-9e39-2677af4f8907#card` | 7 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Plaza of Heroes — `9c58d241-4d9f-4b46-b8ee-f4587f9acfd6#card` | 14 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Power Depot — `64687880-03f9-4f38-985b-1027c797e33f#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Price of Loyalty — `f25b97f9-8ca2-40f7-9d55-47fa8bb2054e#card` | 48 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Primal Beyond — `541744d9-449d-420a-a5a1-2fffba18450f#card` | 18 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Primal Wellspring — `8e4d0da0-c7d8-4a20-9bfd-02c1331a7a49#face:1` | 52 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Prismatic Ending — `2cb98ca9-d7bb-416b-a17e-ee5f8e4d78f2#card` | 30 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Prossh, Skyraider of Kher — `868882d2-ed4e-4171-a17c-478a341080fb#card` | 40 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Pyromancer's Goggles — `f76bcbfe-483f-4e63-8425-76feca1abf3e#card` | 78 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Quistis Trepe — `821e8648-222c-4b33-a8bd-e8bfff7dcd9e#card` | 216 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Radiant Epicure — `cdea4344-24b6-4659-a830-c51d4b348af5#card` | 20 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Radiant Flames — `cfc8372f-13b6-4628-84c7-066881b287f5#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Rakdos, the Muscle — `e2c141af-925e-42bd-8d83-e1a71f4c9794#card` | 720 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Rancorous Archaic — `6a68fcaf-b1e8-451d-8e1c-b47db55c83cc#card` | 66 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Renowned Weaponsmith — `40c1375b-1d15-4525-894a-16b7a0ac02b4#card` | 33 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Resonating Lute — `9263287d-f782-4318-9ab8-e2e5f8107f5e#card` | 7 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Roiling Vortex — `34f83b7a-45a7-40a8-abcb-31d7aae4cea1#card` | 8 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| Roilmage's Trick — `73eaf40a-abc6-486a-958d-374ed6111173#card` | 20 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Scaled Nurturer — `13b96709-0e88-476b-9485-956e682bb818#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend |
| Secluded Courtyard — `79ba18fd-f184-43c1-86df-56ee18ce806c#card` | 18 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Shadow the Hedgehog — `0c12abcc-6ada-43b3-8750-9cb6ce2b0785#card` | 16 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Shrine of the Forsaken Gods — `8ea46945-d5ab-4209-b473-4769e7b8b962#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Skyrider Elf — `c71f19be-aabc-44ef-a62b-57522b3a2c98#card` | 66 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Sliver Hive — `e7286688-ffbe-4d25-ad55-27990f005368#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Slobad, Iron Goblin — `72062fb6-0329-424d-936a-882b638d6d57#card` | 6 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Smaug, Wicked Worm — `20535126-f811-4386-bdce-d73f30691724#card` | 96 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Smokebraider — `e970f2df-90cb-45b8-8ae5-71b762908925#card` | 12 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Snarl Song — `3b6fb13b-f126-4697-90a1-64b14ffcecd6#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Soldevi Machinist — `06600efe-1718-4cc1-b1c0-59c82e2efd73#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Somberwald Sage — `f249db15-09fa-4a1c-9461-9ae803c219b6#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Springmantle Cleric — `83865eee-709a-4a34-94f1-9b818f31323f#card` | 33 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Steel Exemplar — `ab3b72e3-9a49-4c95-8016-22fa02ad7663#card` | 16 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Steelswarm Operator — `770b2097-5178-42d2-be6c-393b4bfe3727#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Summon: Esper Valigarmanda — `cecd2f49-8ccb-472d-9cf1-fd0f4d93546d#card` | 3915 | Purpose Adjunct of BareAuxiliaryPredicate core-verb:Can |
| Sunken Citadel — `508189e1-9cef-4f9c-8ff1-078c99a0f603#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Sunken Palace — `c098c507-5154-423a-a70b-f6dfd4959cf6#card` | 20 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend |
| Swallow Whole — `54d4c632-1e7c-42f1-938c-9b4bd9b9a9b9#card` | 6 | Purpose Adjunct of bare passive lexeme:keyword_action/tap, in the NP Postmodifier |
| Sweep the Skies — `7596fe49-c9cf-4415-ac1b-156cbbf9604e#card` | 1208 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Séance Board — `8331d2ff-3729-4014-8dcb-278644d2632a#card` | 6 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Tablet of Discovery — `19cf5798-4600-4a66-b1c7-de77bde157d0#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Tajuru Stalwart — `ac87cc33-4a71-471b-917d-6a92721ea0b9#card` | 66 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Taster of Wares — `020aae46-cdba-4c37-9c54-283727c09994#card` | 672 | Purpose Adjunct of BareAuxiliaryPredicate core-verb:Can |
| The Mightstone and Weakstone — `c396db03-bf11-4e20-b630-4f9aa8fd78da#card` | 6 | Purpose Adjunct of ParticipialAuxiliaryPredicate core-verb:Be |
| The Ringhart Crest — `490ac985-79b5-438d-a2e7-f47683b29683#face:1` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| The Seedcore — `249fdd3e-376c-4ec2-a612-4353e0e61ee2#card` | 4 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Thieving Varmint — `b9f3cf9f-24c7-43ed-9ba1-f4570df3d09e#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Thran Turbine — `6c31f06d-bf02-48a7-9c17-be2a226b8eae#card` | 3 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Throne of Makindi — `7e8198e9-0f3b-420b-ab09-74f13f4fd548#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Together as One — `234f1c4b-dd0c-4c4e-b3c1-3b9ccfc42226#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Toph, Greatest Earthbender — `3741206f-8ef5-4b8f-ae4c-df3ab0a17683#card` | 12 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Tournament Grounds — `a3ed5d7f-c554-4d46-86e5-7bb8df2693f6#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Transcendent Archaic — `fdd4b3a9-83ce-41bf-82e2-7808657e2c09#card` | 10 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Turtle Lair — `eb002bbc-08df-4bf0-bea3-46494ad261b6#card` | 10 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Unclaimed Territory — `584b15f2-6ae9-413a-8b8d-9244dbea4878#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Uncover the Moon-Letters — `4c67359d-5590-4eb0-827e-b411967ee238#card` | 4 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Unified Front — `1f514b02-7827-4087-a6b7-bd6a3f520a64#card` | 452 | Purpose Adjunct of SelectedPredicate lexeme:keyword_action/create |
| Unravel — `1525cbc1-0d95-4f58-99fb-136e54c4d486#card` | 2 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Untaidake, the Cloud Keeper — `362f25a6-01ff-4c53-be52-c6346a9b0065#card` | 1 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Vazi, Keen Negotiator — `7446c013-63b0-402b-b8b9-dd25ccc553df#card` | 48 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Vedalken Engineer — `b36accb4-cde6-4b1d-b549-8c975cfa999d#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Vexing Bauble — `4514777d-0589-4631-978b-ff244167c176#card` | 2 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the selected passive predicate in the finite Clause |
| Vodalian Arcanist — `1d07856b-d9ef-418c-bd06-3888dbcaddf4#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Voldaren Estate — `fb0c0426-f1a6-4e52-9242-627786d3119a#card` | 3 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Volshe Tideturner — `eab16fd1-735c-411f-a23a-0b93131c2eb2#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| White Lotus Hideout — `2cdbfda3-98fc-4108-b551-c7049168924e#card` | 2 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Wildgrowth Archaic — `4469ff35-54ec-4ff5-bc19-3808ae0f711b#card` | 7920 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Woodland Wanderer — `d5f3c711-edfa-4105-9844-e3f444942a03#card` | 66 | Purpose Adjunct of PrepositionPredicate core-verb:Enter |
| Woodland Weavemaster — `28ffddca-a3c4-467d-a2dc-2c5e62393e11#card` | 9 | Purpose Adjunct of SelectedPredicate lexeme:Verb/Spend; only focuses the Purpose Infinitival |
| Wurmquake — `0eeb9a62-1b36-42f5-8820-3331fa5ca3a6#card` | 16 | Purpose Adjunct of bare passive lexeme:Verb/Spend, in the NP Postmodifier |
| Yasharn, Implacable Earth — `6dc23410-4d8f-4c8a-92b4-a8b3fae1af4e#card` | 345 | Purpose Adjunct of Coordination core-verb:Pay |
