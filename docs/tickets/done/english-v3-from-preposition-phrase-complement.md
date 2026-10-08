---
needs: []
---
# Read *from* with a Preposition Phrase Complement: a land card from among them

## Why

*From among* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *from among* touches **460** unread faces and
is the sole cause on **268** (recon bucket "from among"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Choose a creature card from among them." 0; "Choose
a creature card from them." 1; "Choose a creature card among them." 1; "You may
put a land card from among them into your hand." 0, while "… from them into
your hand." 1.

This ticket takes over from `english-v3-systemic-residuals` the Cavalier of
Thorns / Genesis Ultimatum item ("`among them` has one PP Reading but `from
among them` … none. Consume a declared PP argument for From; the current
PrepositionComplement inventory has only None/NounPhrase") and the remaining
From-among sources of Tracker's Instincts, Pieces of the Puzzle and Kruphix's
Insight from the library-position reconciliation.

## Goal

*from* takes a PP Complement headed by *among* (*from among them*, *from among
the cards milled this way*, *from among permanents you control*), wherever its
NP Complement already reads: as a Postmodifier of the selected NP and as the
source Complement of verbs whose frames declare one. Add a PP value to the
preposition-complement inventory and license it by a declared feature of the
governing preposition and of the complement's head; no word-named pairing.

## Analysis

A preposition can take a PP complement (*The magician emerged from behind the
curtain*; CGEL, Ch. 7, §2, p. 617). *From* takes as complement an NP or a PP
headed by a locative preposition (*from behind the wall*, *from under the
bridge*; Ch. 8, §4, p. 687). That passage's examples do not include *among*;
classifying *among* as eligible is the landing's decision, recorded through a
declared feature, not a CGEL claim.

## Project analysis

Claimant decision under this ticket's Goal: *among* is eligible as the head
of a PP Complement to source-marking *from*. The lexicon declares that use;
the grammar reads the governing Complement licence and the internal PP's
head licence. CGEL, Ch. 7 §2.4, p. 617 establishes PP complementation, and
Ch. 8 §4.3, p. 687 describes *from* with locative PPs. Neither passage
establishes the *among* classification adopted here.

The ticket's prohibition on an NP attachment when the verb declares a source
is implemented by a Source-free Object requirement in the affected lexical
frame slots. The complete source PP fills Source Complement. The global
From Modifier licence remains for hosts such as Choose and Put whose source
frames are deferred; no deferred function licence is pruned here. The
frame-category projection reads Source Modifier Present in every admission
path, including coordinated Complement segments, without a verb identity guard.

The Complement inventory is a set so one head can license NP and PP
Complements. Intransitive use has the explicit member `Intransitive`;
`None` is the declaration reader's empty-set spelling and cannot name a
positive set member. The former `None` inventory route is retired in both
the grammar and the three affected lexical declarations. NP, predicative NP,
PP and intransitive admission all read membership in the same inventory.

Tomakul Scrapsmith permits the participial Postmodifier *milled this way*
on the selected artifact-card nominal or on the internal plural *cards*
nominal. Both grammatical Readings are retained; the nested source remains
one complete *from* PP in each. Return's selected-source depictive route
re-spells Yore-Tiller Nephilim's existing exact-value test, preserving its
Object, source, destination, and coordinated depictive outcome.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Ostrich-Horse: "You may put a land card from among them into your hand."
- Satyr Wayfinder: "You may put a land card from among them into your hand."
- Animal Magnetism: "An opponent chooses a creature card from among them."
  (control "An opponent chooses a creature card." 1)
- Seek the Wilds: "You may reveal a creature or land card from among them and
  put it into your hand."
- Tomakul Scrapsmith: "You may put an artifact card from among the cards milled
  this way into your hand."

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
   Reading. *among them* read as a separate Adjunct beside a bare
   *from*, or the PP attached to the wrong NP when a verb frame declares the
   source, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *from exile*, *from anywhere* and the other deferred From source frames
  (from-M, from-A): owned by `english-v3-systemic-residuals` /
  `english-v3-selected-preposition-nominal-licensing` re-application.
- *Remove … counters from among artifacts you control*: count it if it reads,
  do not build a Remove frame here.

## Landing record

### PROVE: parent equivalence, retained Readings and structural laws

The one full, non-incremental census is `after.json`, measured at
`pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**, against the claim-parent-equivalent
`vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307** census in `before.json`.
The claim parent at claim time was `ulkovlxyoqtsrqtuquupuvyvyyqpqntu`: its only
changes then from the measured baseline were two ticket status moves. All **1,697** census
source hashes are equivalent; `claim-parent-equivalence.json` records the
status-only delta. No baseline full run was repeated in this ticket.

The **32,828** durable face identities and their raw/analyzed source hashes
are unchanged. Face-by-face counts and `readings_sha256` give **181 gains,
zero lost faces, 1,609 previously covered faces with fewer Readings, 306
same-count representation changes, and 30,732 unchanged count/digest pairs**.
There are zero increases on previously covered faces. Every decrease and
representation change contains Return or Remove; all decreases contain Return.
`comparison.json` retains both fingerprints and source text for every changed
identity. The complete decrease list below names and justifies each identity;
none is unexplained, no re-coverage is owed, and no regression is accepted.
The decreases retire **135,016** old Readings. Gains contribute **73,118**
Readings, giving **1,000,364 → 938,466** checked Readings overall.

Every counted Reading passes declaration admission, lexical ownership/context,
byte-exact realization, and construction/word traversal identity. There are
**zero issues, internal failures, cyclic derivations, duplicates, failed or
limited faces**. All enumerations are complete. The corpus command does not
prove linguistic correctness or the independently constructed-value roundtrip;
the independent PP and Return values, altered-feature negative oracles,
coordinated-tail negative oracle, and re-spelled exact-value tests provide that
second law and the source-attachment evidence. The selected source path of
every gain is retained in `gain-analysis-ledger.json`.

The **471-face From-among** and **4,992-face Return/Remove/intransitive**
selectors have zero count, digest or analyzed-source mismatches against this
full run (`subset-full-equivalence.json`). The full run used exactly
`cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-from-preposition-phrase-complement/after.json`.
No other full census, xtask tests, Lean checks or derived gate ran.

New word-named licensing guards: **zero**, by diff audit. The new guards read
Complement inventory, PP Complement Licence, Source Complement Use, Source
Marker, Source Modifier Present, function licence and declared frame signature.
Workspace lexical loading has zero fatal errors. Its **87** source diagnostics
and **nine** unsupported frame signatures are byte-for-byte unchanged in the
inventory report. V3 does not emit the retired licensing-checker total; no
replacement checker or derived gate is introduced.

### DISCLOSE: selected analyses, witnesses and residual ownership

All figures in this section belong to `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**,
compared to `vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307**.
The new inventory member is **PrepositionPhrase**, licensed for From by
`PrepositionPhraseComplementLicence = Source` and by the internal PP head's
`SourceComplementUse = Yes`. Source Marker identifies the complete source PP
for a selected Source Complement. NP Postmodifier, inherited VP/Clause Adjunct,
and verb-selected source functions all retain the complete nested
**From [Among NP]** constituent. The inherited Adjunct functions belong to
the deferred hosts; they are not new source frames. From has no intransitive
membership, so a bare From beside a separate Among PP is not admitted.

The five requested localized witnesses change from zero Readings to:

| Witness | Localized Document Readings | Full-face Readings |
| --- | ---: | ---: |
| Ostrich-Horse | 2 | 2 |
| Satyr Wayfinder | 2 | 6 |
| Animal Magnetism | 4 | 12 |
| Seek the Wilds | 36 | 108 |
| Tomakul Scrapsmith | 5 | 5 |

The extra Bag of Devouring VP test requires the complete PP in Return's
selected Source slot in every retained Reading. Independent values also check
NP ownership and frame requirements through coordinated tails. Avatar Destiny
remains covered: **2,160 → 1,170** Readings, with its coordinated tails and
controller outcome retained. Tomakul's exact observed source set contains
*among the cards* and *among the cards milled this way*, preserving both
participial attachments described in the project analysis.

Manual selected-value spot checks covered these **14** newly covered faces:
Ostrich-Horse, Satyr Wayfinder, Animal Magnetism, Seek the Wilds, Tomakul
Scrapsmith, Cavalier of Thorns, Tracker's Instincts, Pieces of the Puzzle,
Hopeful Initiate, Tekuthal, Inquiry Dominus, Retribution of the Ancients,
Novijen Sages, Overseer of Vault 76, and Iron Spider, Stark Upgrade. The first
eight select an NP Postmodifier in the retained value; the six Remove faces
select a complete Source Complement. Remove still has **one** lexical frame;
no Remove frame was added. Every gain is listed below with its actual retained
function, source head identities and selected fingerprint in the evidence.

Cavalier of Thorns (**12**), Tracker's Instincts (**6**) and Pieces of the
Puzzle (**6**) discharge their From-among whole-face residuals. Genesis
Ultimatum, Kruphix's Insight, Midnight Tilling, Bag of Devouring and Tayam,
Luminous Enigma remain uncovered as full faces. Kruphix's localized source
unit has **two** Readings while its source-free whole instruction control has
**zero**, localizing an independent residual; Midnight's permanent-card
source-free control likewise remains blocked. These faces and the remaining
**284** unread From-among faces stay routed to the live
`english-v3-systemic-residuals` ticket, with no whole-face gain asserted.
The deferred From-M/From-A frame re-application remains there too; the
completed nominal-licensing ticket is provenance, not a live obligation sink.

The selection census is **7,540 one / 14,767 multiple → 8,120 one /
14,368 multiple**. Two gains are unique and 179 are multiple; 578 previously
multiple faces become unique by the declared route retirement. Specificity
resolution is not an admission stage. All other grammatical Readings remain.

### Implementation and analysis

`PrepositionComplement` is a set with NP, predicative NP, PP and intransitive
membership. From retains NP membership and gains PP membership; the PP
licence requires a Source Complement Use head, declared on Among. Of retains
its prior unrestricted PP complementation through this same inventory. The
superseded separate PP flag and intransitive `None` spelling are retired on
both declaration and lexical sides.

Return and Remove now select the complete Source Complement PP. Remove
retains its one frame. Return's existing optional-source route is retired;
source-present, source-absent and source-absent/controller-present layouts
retain the original outcomes without duplicate absent-source frame choices.
Source-free Object is a feature-constrained Category projection. The shared
compiler applies it during recognition, independently constructed value
admission and coordinated tail admission. A source inside a separate embedded
NP or Clause does not mark the containing Object head. From's global function
licences remain for the deferred hosts named in the ticket.

CGEL Ch. 7 §2.4, p. 617 backs PP complementation; Ch. 8 §4.3, p. 687 backs
From with locative PPs. Among eligibility is the claimant's project analysis,
recorded above, and is not attributed to either passage. Tomakul retains both
grammatical participial attachments. The selected-source depictive instance
preserves Yore-Tiller's exact Object/source/destination/depictive test outcome.

### Deviations and additions

- Added Source Complement and a selected-source depictive schema, its two
  instances, and two static frame signatures. The latter are signature
  classifiers, not extra lexical frames or alternative constituencies.
- Added general feature requirements to Frame Category projections and shared
  their admission with coordinated Frame segments, to prevent the forbidden
  source-on-Object attachment without an identity guard.
- Re-spelled the seven Remove witnesses, the independent Sun Droplet value and
  invalid-marker check, Raise Dead, and Yore-Tiller: 10 existing English tests and one lexical-source orientation test (11 total). Added
  9 tests in the shared integration suite; restored 0 deleted tests, ignored
  0, removed 0. Exact values, lexical traversal and outcomes are retained.
- Added eight Oracle-English glossary entries. No Game Model terminology
  changed. No glossary gap remains.
- The extra Midnight Tilling test is localized to its source PP because the
  source-free `return a permanent card to your hand` control also fails on
  the independent permanent noun-Premodifier gap; the face is not claimed as
  a gain. An overstrict new Tomakul assertion was corrected to preserve both
  grammatical participial attachments, with an exact observed-source set.
- The suite exposed and fixed set membership checks for existing predicative
  and intransitive prepositions, the source flag through noun premodification,
  and the selected-source depictive route. No existing test was removed or
  weakened. No contradiction with a recorded ruling arose; no STOP remains.

### REPORT: declarations, inventories, checks and performance

At `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**: **205 ordinary constructions +
51 schemas = 256**, **337 instance rows**, **796 compiled Productions**,
**149 Categories**, **3,668 nonblank declaration lines**. The measured parent
`vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307** has **204 + 50 = 254**,
**335 rows**, **717 Productions**, **148 Categories**, **3,530 nonblank lines**.
The additional selected-frame depictive instances account for expanded
Productions beyond their declaration rows. Observed construction names are
**216 → 218**: SourceComplement and SelectedSourceDepictivePredicate are new.
These are provenance counts, not an economy or coverage gate.

Both stamped trees have **38,404 spellings, 1,139 named homographs, 1,092
case-folded homographs, 23 form literals, zero literal/vocabulary overlaps**.
Complete named inventories are `spellings.json`, `homographs.json`,
`case-folded-homographs.json`, `form-literals.json`, and
`form-literal-vocabulary-overlaps.json`. All five named inventories are
structurally identical before and after. No spelling, lexical owner or form
is added or removed. The lexical fingerprint changes only for the declared
features and frame layouts recorded above.

- `cargo test -p deckmaste_english_v3`: **427 passed**, zero failed/ignored;
  final rerun after compiler helper extraction and the test allocation fix.
- `cargo test -p deckmaste_lexical_source`: **23 unit + 34 integration passed**,
  zero failed/ignored; the intransitive feature assertion is re-spelled.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  **passed**. The shared compiler is checked as a dependency; its seam is
  exercised by the English integration suite. No extra crate test scope ran.
- Formatting passed for the two test crates and shared compiler Rust files;
  new runtime-template blocks are formatted without rewriting the template.
- `cargo xtask cite check --list-noncompliant`: **zero noncompliant strings**;
  `cargo xtask cite check`: **zero stale**. No new CR citation needs blessing.
  The piped diff citation audit is recorded in `cite-audit.log`.

Performance advisory, integer ns and **12 workers**:

| Tree / covered | Setup ns | Corpus wall ns | Checked-text thread CPU ns/B | Host load at start (1/5/15 min) |
| --- | ---: | ---: | ---: | --- |
| `vqoxvykpqkoupsorotsrnuurwvwvwnov` / 22,307 | 4162111218 | 99031374696 | 402729 | [10.681640625, 8.5498046875, 7.46826171875] |
| `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / 22,488 | 2276800773 | 101737986666 | 411796 | [1.03173828125, 2.09814453125, 1.9404296875] |

The feature command wall is **107352113161 ns**, measured directly
around the foreground Cargo invocation including serialization (`full-timing.json`).
Host load at completion is **[12.2919921875, 5.94189453125, 3.34765625]**, with **12 workers**.
Both corpus timings and the feature command wall exceed the
**16,260,000,000 ns** quiet-host advisory ceiling. No quiet-host compliance
claim is made. These measurements are provenance, never a fitted gate.

All evidence lives under `target/english-v3/english-v3-from-preposition-phrase-complement/`.
The deprecated covered-lock/derived-gate mechanism is not invoked; the full
census covered count supplies the tree stamp above.

Mandatory Kata refresh completed after closing the feature. The refreshed
trunk includes `twsoltks` and `tquwunwx` for the Lean checker/documentation
landing; none of the **1,697** measured census sources changed. The exact
post-refresh hash comparison is `post-refresh-source-equivalence.json`.
The measured grammar/lexicon and corpus remain byte-identical, so prior tests,
clippy and census evidence are retained, with no post-refresh re-census and
no trunk-inherited Reading decrease to attribute.

### Every newly covered identity and retained analysis

All rows are `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**, against
`vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307**. Each selected value retains
From [Among NP], with the actual function shown. The complete internal NP
lexical-owner list, source text, construction path and selected fingerprint
are in `gain-analysis-ledger.json`; all grammatical Readings are retained.

| Face | Durable identity | Readings | Retained From [Among NP] function |
| --- | --- | ---: | --- |
| Aang, at the Crossroads | `96e5d4a1-e59f-4140-823f-a17e15ee5d8d#face:0` | 675 | NP Postmodifier |
| Acclaimed Contender | `35df179a-c0e6-4ac1-a861-e6e9b4d1614d#card` | 72 | NP Postmodifier |
| Adaptive Omnitool | `e91240bc-4a13-4131-9379-5ddac6555f4a#card` | 96 | NP Postmodifier |
| Adventure Awaits | `70235727-e330-46fa-b057-ddbb55cd1ac3#card` | 96 | NP Postmodifier |
| Adventurous Impulse | `5bf49f39-b3d5-4148-8dc2-b5b5011769d2#card` | 108 | NP Postmodifier |
| Aethermage's Touch | `15692698-ef57-4672-bf76-5fe4a00c693a#card` | 12 | NP Postmodifier |
| Ainok Wayfarer | `8134f3fd-197a-4b44-a370-311cc955dea3#card` | 2 | NP Postmodifier |
| Airlift Chaplain | `719aeb50-c8cf-4cd8-b74c-d2cef408b52e#card` | 7 | NP Postmodifier |
| Ajani's Influence | `2a3e3f3b-1f74-4ac7-b5ac-7c3cb9cc38a6#card` | 48 | VP Adjunct |
| Ancient Stirrings | `82f18e7d-5c42-47c4-8e74-3fccc9b7b1f0#card` | 48 | VP Adjunct |
| Animal Magnetism | `a1bdf7f1-e3f7-4a13-9cf0-a2394df6cc74#card` | 12 | NP Postmodifier |
| Apex of Power | `d06eedb4-bd5a-4864-9505-a570a745d165#card` | 30 | VP Adjunct |
| Arcane Infusion | `d5251d47-11de-41f7-9965-1f921e12d927#card` | 108 | VP Adjunct |
| Arcanist's Owl | `4bd38337-fb79-4809-848a-308c682825d1#card` | 108 | VP Adjunct |
| Armored Skyhunter | `b4dbbf56-d7df-4183-bb48-cfc6b4d0468f#card` | 30 | NP Postmodifier |
| Ashe, Princess of Dalmasca | `db75577d-1001-4088-999f-40684218a07f#card` | 48 | NP Postmodifier |
| Astor, Bearer of Blades | `196962f0-6aff-4198-9e0c-eaf42aeaf147#card` | 108 | NP Postmodifier |
| Augur of Bolas | `be5d6779-ecc9-44b9-8c4d-97a243059d55#card` | 108 | NP Postmodifier |
| Basic Conjuration | `547ab16c-b8d1-4357-9219-7cabde80f5ba#card` | 48 | VP Adjunct |
| Belisarius Cawl | `6b336228-92c0-4f14-a479-1f97aaeae018#card` | 1536 | NP Postmodifier |
| Benefaction of Rhonas | `02da2cbf-50d6-4c37-8a9f-40db1f7954be#card` | 6 | NP Postmodifier |
| Bind to Life | `3a97bc8d-bd81-468e-9277-eba4fac11ee0#face:1` | 2 | NP Postmodifier |
| Blanchwood Prowler | `8d1a367a-542e-4b94-b466-c922c5fb717b#card` | 5 | NP Postmodifier |
| Blossom Prancer | `69e92c4f-df05-4297-a453-8e4644c31a90#card` | 216 | VP Adjunct |
| Boiling Rock Rioter | `5193172c-9024-409e-bd9e-0387971d65fe#card` | 462 | NP Postmodifier |
| Boromir, Gondor's Hope | `5bb5e4e5-c3b4-4f67-83e1-bde9ce9f18c4#card` | 216 | NP Postmodifier |
| Bounty of Skemfar | `e4bfe005-7c4e-4178-ae41-a28eb2352ad1#card` | 12 | NP Postmodifier |
| Brazen Upstart | `e0a9c4be-74fe-4984-9028-60380db1575b#card` | 48 | NP Postmodifier |
| Brightwood Tracker | `f8d44858-1d21-43fd-b193-59ddf7c53785#card` | 48 | VP Adjunct |
| Bucky Barnes, Eager Ally | `55bc5d06-7dd1-4403-af32-0e10d8bbe327#card` | 108 | NP Postmodifier |
| Call to the Kindred | `c2fb8d4c-b973-42a0-9e9c-7abd5cc2c22e#card` | 492 | VP Adjunct |
| Cantankerous Keepers | `77c97dad-17cc-4a28-a48b-907c044d9034#card` | 12 | NP Postmodifier |
| Cartographer's Survey | `e05029b4-552d-4691-a5c4-885284fb1636#card` | 12 | NP Postmodifier |
| Casey Jones, Jury-Rig Justiciar | `b4c4fdb9-e034-49c0-9f7f-07e8cae19eb2#card` | 192 | NP Postmodifier |
| Cataclysmic Gearhulk | `2e4865c5-a590-4820-81ec-926014ddb8c4#card` | 5 | VP Adjunct |
| Cavalier of Thorns | `96b909ab-55f1-4cde-a675-2504de3da772#card` | 12 | NP Postmodifier |
| Chiss-Goria, Forge Tyrant | `3f4d1ae1-d5c6-4afb-94c8-8e0ae4891501#card` | 84 | VP Adjunct |
| Choco, Seeker of Paradise | `7979bf83-5091-4e62-8885-ca1833a97306#card` | 90 | NP Postmodifier |
| Colfenor's Plans | `ad786230-4045-46af-bc89-078d45891e5b#card` | 2250 | VP Adjunct |
| Collected Company | `0bbcb563-09b3-499d-a9cc-3a6d7501dd81#card` | 9 | NP Postmodifier |
| Commune with Beavers | `9108fffb-90f4-444e-8d3e-46a3d4ff3681#card` | 108 | VP Adjunct |
| Commune with Dinosaurs | `27ca27f0-95a5-4292-b30b-1db3e9d639f8#card` | 108 | NP Postmodifier |
| Commune with Nature | `d4ed4260-5f38-4b55-ba6e-9f54f04d2360#card` | 48 | NP Postmodifier |
| Commune with Spirits | `68f8ed23-024e-414f-bbb2-ab73fbe21b69#card` | 108 | NP Postmodifier |
| Commune with the Gods | `6042a806-6e6a-4f1d-9563-5932a17f32f9#card` | 15 | NP Postmodifier |
| Conjurer's Mantle | `21b520c6-c94a-475a-a824-dd96394ea55b#card` | 420 | VP Adjunct |
| Contagious Vorrac | `1b6da140-0d91-4fae-a3ee-33e6808acf34#card` | 96 | NP Postmodifier |
| Courageous Outrider | `00915480-ffa8-4930-a158-b1c1f75b7876#card` | 48 | VP Adjunct |
| Cowabunga! | `a1ddf985-1646-468c-b1a4-ab6bb8e9dd7c#card` | 108 | VP Adjunct |
| Dawnhand Dissident | `2cabf7a4-17af-45c7-b314-a84f5dc0d070#card` | 7191 | NP Postmodifier, selected Source Complement |
| Definitely Not a Turtle | `3d8dfd57-7d26-4a3b-a9c3-33077c85b3db#card` | 72 | VP Adjunct |
| Depala, Pilot Exemplar | `cc913f28-18ef-485e-8d2b-ae0115523020#card` | 72 | NP Postmodifier |
| Deploy the Gatewatch | `e17ea7eb-362e-4c61-97cd-537f3921bced#card` | 6 | NP Postmodifier |
| Director Nick Fury | `04b907bf-c80c-4e92-9fea-26dc3d1aaf2a#card` | 384 | VP Adjunct |
| Doomskar Warrior | `7ac00312-8963-4fee-8561-1a54e533adbb#card` | 540 | VP Adjunct |
| Dragonologist | `f6791df0-a37b-44dc-8bf3-40ac2304d188#card` | 216 | NP Postmodifier |
| Dáin's Company | `ae03782f-8f05-4ce2-a049-005b1b688faf#card` | 216 | VP Adjunct |
| Eclipsed Boggart | `9cc3b91b-ad70-4b01-b718-fda3819f84d0#card` | 108 | NP Postmodifier |
| Eclipsed Elf | `7c89ef4d-b375-4644-ac8b-034effdbe98c#card` | 108 | NP Postmodifier |
| Eclipsed Flamekin | `48c641df-d511-4e35-b623-9d1fdb5240da#card` | 108 | VP Adjunct |
| Eclipsed Kithkin | `e6227840-6e3e-48c7-b4a4-42cfd8f14b68#card` | 108 | NP Postmodifier |
| Eclipsed Merrow | `a3a00b62-f07e-44ef-8d97-22a3e7a06f72#card` | 108 | VP Adjunct |
| Eerie Gravestone | `cbf5bd0a-20ef-4dce-9f70-13339d438ff7#card` | 2 | NP Postmodifier |
| Eivor, Wolf-Kissed | `cb8289e9-1e59-41f1-a994-6e34d2da2da2#card` | 12 | NP Postmodifier |
| Elvish Rejuvenator | `71d14755-b81d-4aec-b94d-1637884a5844#card` | 12 | NP Postmodifier |
| Explosion of Riches | `ffd469b4-413d-49f5-a873-e3f654240b31#card` | 22 | Clause Adjunct |
| Faerie Mechanist | `fcbe14c4-1b0d-486f-bb9c-0fed452668cb#card` | 48 | VP Adjunct |
| Fertile Thicket | `7ba580c9-f933-43d9-b03d-a349faa6c641#card` | 270 | NP Postmodifier |
| Floriferous Vinewall | `26685121-ab1f-4366-840b-c5a80508cde6#card` | 48 | VP Adjunct |
| Frontier Seeker | `466cf331-2d9c-4ebc-84f6-30c890e1b637#card` | 48 | VP Adjunct |
| Getaway Barrel | `7ddbf891-7f80-44e9-b94c-70c1da0b91ed#card` | 27 | NP Postmodifier |
| Gilgamesh, Master-at-Arms | `c9432e87-38f6-4889-8f07-ae7cd045e532#card` | 18 | NP Postmodifier |
| Gleam of Death | `2802069f-201e-43a7-b5d3-43a95951a2ec#face:1` | 4 | NP Postmodifier |
| Glint-Nest Crane | `1983478a-cedb-4053-adab-ac5f1a163296#card` | 48 | NP Postmodifier |
| Grisly Salvage | `a89ebf63-63c9-42cd-98b4-ec852cd9b0a8#card` | 15 | NP Postmodifier |
| Growing Rites of Itlimoc | `ea9c459a-6047-43aa-968f-a582be4000e8#face:0` | 48 | NP Postmodifier |
| Guru Pathik | `4dcd7b78-ce5a-43eb-a16d-3b9ea411795d#card` | 864 | NP Postmodifier |
| Harald, King of Skemfar | `0089b07c-7f47-4a9a-9fb7-cc38ec9d7bcd#card` | 108 | NP Postmodifier |
| Harper Recruiter | `9ecbc7de-9399-42f4-9c39-b9255080f48c#card` | 48 | VP Adjunct |
| Hedonist's Trove | `9d5f712a-d46c-4d7e-8fea-45fdadb458df#card` | 1587 | VP Adjunct |
| Heroes' Podium | `85292bfb-32d7-449e-9ebb-5be828e8ecf6#card` | 720 | VP Adjunct |
| Hopeful Initiate | `317169b8-9014-48e6-862b-ca21b706846e#card` | 1 | selected Source Complement |
| Horn of the Mark | `9b836c32-84f0-41ae-b7d9-67b92c743c60#card` | 48 | NP Postmodifier |
| Ian Malcolm, Chaotician | `57ff1665-0a7b-4d98-bd8f-cc07cf04f701#card` | 780 | VP Adjunct |
| Icon of Ancestry | `2b4973cc-83b0-4986-a19e-b4252706527e#card` | 132 | NP Postmodifier |
| Ignis Scientia | `23320d55-80cb-4da3-a53c-8c75abdc8c8f#card` | 72 | NP Postmodifier |
| In the Presence of Ages | `142a960c-4d8e-4b3b-a0db-435c22df8f92#card` | 6 | NP Postmodifier |
| Incubation | `363edb16-009f-4cc2-99e3-2b15db84e7af#face:0` | 48 | NP Postmodifier |
| Industrial Advancement | `7a0aa9ea-610e-4910-b66d-fdac69e88adf#card` | 6 | NP Postmodifier |
| Ingenious Smith | `f3988b96-9e00-445c-b3e1-6b2df9b13dac#card` | 48 | NP Postmodifier |
| Intrepid Paleontologist | `d6026309-aae9-43e7-938b-69db7af7139c#card` | 6588 | NP Postmodifier |
| Iron Spider, Stark Upgrade | `e123fd7d-ace9-48a4-9510-eedcc837d8e8#card` | 5 | selected Source Complement |
| Jet, Rebel Leader | `b9ca8ead-c267-4170-a6c8-04498330c82b#card` | 18 | NP Postmodifier |
| Kaalia, Zenith Seeker | `6307ec34-bef6-4ac6-a363-f9bef0d03b3b#card` | 96 | NP Postmodifier |
| Kayla's Reconstruction | `43bae4b3-57e8-4d9a-9d1b-9805b0e0fff5#card` | 9 | NP Postmodifier |
| Kheru Mind-Eater | `dfea918b-5702-4c12-ba36-e6aa4ace09c7#card` | 3375 | VP Adjunct |
| Knickknack Ouphe | `55adf2f0-f023-4f41-ae9d-3c002e9e6e8d#card` | 90 | NP Postmodifier |
| Kolaghan Warmonger | `4c7e6520-d179-4590-afdc-92eb2c0740a6#card` | 48 | NP Postmodifier |
| Kolvori, God of Kinship | `490ac985-79b5-438d-a2e7-f47683b29683#face:0` | 60 | NP Postmodifier |
| Kylox's Voltstrider | `e7b746c8-1b32-42ed-8328-4e16274209d8#card` | 3216 | NP Postmodifier |
| Liliana's Spoils | `751da0fa-0ab1-4b8c-81bb-d86a0a280ac4#card` | 48 | VP Adjunct |
| Living Phone | `da554d71-1f13-49ae-9eb2-c778a23df08b#card` | 123 | VP Adjunct |
| Lord of the Void | `626b85f8-8889-46e6-9659-460bac79ae9b#card` | 252 | NP Postmodifier |
| Lupinflower Village | `b6c7c708-5212-4100-b954-b77855b27915#card` | 108 | NP Postmodifier |
| Master Piandao | `e294d9d2-df91-4410-85d3-8ffb13fb2285#card` | 108 | NP Postmodifier |
| Mayael the Anima | `7f546d54-584d-4bec-8fbb-1ea2f8ab277e#card` | 9 | NP Postmodifier |
| Meletis Astronomer | `6593a7c1-fb48-4e53-8e6a-339ad2cc4489#card` | 192 | VP Adjunct |
| Memorial to Unity | `a74494ef-aa35-4830-9b4c-47bff5270efc#card` | 48 | NP Postmodifier |
| Militia Bugler | `edb6ae9e-a41c-4d82-97bf-4be00e98364c#card` | 123 | VP Adjunct |
| Muzzio, Visionary Architect | `fc2768a5-621b-48de-9160-4dbaeddb3de1#card` | 30 | NP Postmodifier |
| Mycoid Maze | `740aa9d9-91a9-431e-8bf9-1344e5273e27#face:1` | 48 | NP Postmodifier |
| Nessian Game Warden | `b03397e5-56af-47c0-ab9a-746a0c99c568#card` | 96 | VP Adjunct |
| Nessian Wanderer | `45703364-660a-4e73-a467-5ee47d936c1a#card` | 96 | NP Postmodifier |
| Nightveil Specter | `4ce83f8c-b27e-4e5b-b19b-76b66aaa904b#card` | 600 | Clause Adjunct |
| Novijen Sages | `6d7f3139-9776-43bb-ae12-12ed4f5c3a51#card` | 1 | selected Source Complement |
| Ojer Kaslem, Deepest Growth | `eda11077-b2ce-408b-b982-def2da8fe599#face:0` | 300 | NP Postmodifier |
| Orb of Dragonkind | `d9c36ed5-629c-4709-bff5-2f10fcde608a#card` | 576 | VP Adjunct |
| Ostrich-Horse | `2b6cb199-1564-4214-b24f-624dc962c907#card` | 2 | NP Postmodifier |
| Overseer of Vault 76 | `84f32a1d-3861-418e-a3c2-7e731a2cd55c#card` | 14 | selected Source Complement |
| Owlbear Cub | `0c92432d-ac76-47d2-af7a-fdffd4b546a2#card` | 6 | NP Postmodifier |
| Peer Through Depths | `a61f5100-aa38-4996-b23c-3ce4ed25ed09#card` | 108 | NP Postmodifier |
| Pictures of Spider-Man | `4fd74a8c-a4e7-4988-ae02-9a73ab6ec89e#card` | 48 | NP Postmodifier |
| Pieces of the Puzzle | `7b599b4d-95bc-4ed2-a167-84a697944747#card` | 6 | NP Postmodifier |
| Planar Atlas | `4f14bf38-13ec-461d-8a1b-340843b5d95d#card` | 216 | NP Postmodifier |
| Planar Genesis | `b368121b-9984-4a01-8d35-d1e4a8a87539#card` | 12 | NP Postmodifier |
| Pulsar Squadron Ace | `8019e879-cfea-40a2-ac0d-86b47ec02b48#card` | 96 | VP Adjunct |
| Quandrix Apprentice | `92116bbd-c7ad-4033-8d98-3ea09fea8ae7#card` | 384 | VP Adjunct |
| Quilled Greatwurm | `01635d21-aeda-4754-9d94-68fcd0cd6556#card` | 61 | selected Source Complement |
| Rampant Frogantua | `a08393b5-603b-41c4-aa03-2ae9c74884a7#card` | 36 | NP Postmodifier |
| Ravenous Gigamole | `68f4e3e5-2f0d-4d2d-84f6-d1e824a0dcf0#card` | 5 | NP Postmodifier |
| Reality Heist | `569da5d3-6af2-4021-a464-b30ebaaf6fa0#card` | 96 | VP Adjunct |
| Recruitment Officer | `97dfc025-afc2-42d8-a20a-f29d60754a2e#card` | 123 | VP Adjunct |
| Relentless Pursuit | `d39c378b-f0a2-400f-af40-a632e24b6d72#card` | 6 | NP Postmodifier |
| Retribution of the Ancients | `550b8bee-1773-427b-b300-aaceff243e11#card` | 2 | selected Source Complement |
| Rick Jones, Destined Sidekick | `4bccfa67-0c6f-42a1-b8d4-0842a8230d1e#card` | 5 | NP Postmodifier |
| Rip, Spawn Hunter | `c027f369-e445-46a8-94ad-93396321295a#card` | 576 | NP Postmodifier |
| Rosecot Knight | `b81a9cf5-4059-45fb-a4e1-f2fe6aa3c841#card` | 216 | VP Adjunct |
| Saheeli's Directive | `1aee3a93-d064-4d10-9fe6-879a1f342c1f#card` | 18 | NP Postmodifier |
| Sanwell, Avenger Ace | `4884ad6c-2290-44fc-ab9e-6e6b75b2d69a#card` | 156 | NP Postmodifier |
| Sarkhan's Dragonfire | `2a08d878-7816-4d0a-9532-4f2d3c8eee90#card` | 48 | NP Postmodifier |
| Satyr Wayfinder | `ead4c01a-e2ae-49c9-95d9-651d81005907#card` | 6 | NP Postmodifier |
| Scout the Borders | `08781f2a-0d43-495c-aac8-7ff7a2e5752f#card` | 15 | NP Postmodifier |
| Seek the Wilds | `3865d219-44b7-49ce-ab26-b833acb39765#card` | 108 | NP Postmodifier |
| Seismic Sense | `9d586c74-b1dd-4fcb-a24a-c703d250fa63#card` | 216 | VP Adjunct |
| Serpent's Soul-Jar | `5d451958-25f3-4e70-8be2-c832f60f39e9#card` | 52 | VP Adjunct |
| Shared Fate | `e23bf5fe-e494-473e-8502-606106ee1820#card` | 1500 | VP Adjunct |
| Shigeki, Jukai Visionary | `cbad3570-5417-42e6-b3ef-f42194098314#card` | 24 | NP Postmodifier |
| Sigarda, Champion of Light | `24919141-82d2-42ad-963c-e53511a868b3#card` | 240 | NP Postmodifier |
| Silhana Wayfinder | `dc858fc1-d97a-4cc5-af31-2c33d9639e1b#card` | 108 | VP Adjunct |
| Silundi Vision | `b0182ca0-f353-4012-9121-6f4ac9f7a046#face:0` | 108 | NP Postmodifier |
| Silvan Rally | `f6771d32-395e-4832-b33d-cbc12dff1516#face:1` | 4 | NP Postmodifier |
| Silverback Elder | `b1a0ae56-77aa-43ad-9aa8-dc1ab779e9d9#card` | 24 | NP Postmodifier |
| Skalla Wolf | `22da0659-7cd0-4277-b6d2-7d954dcd7729#card` | 48 | NP Postmodifier |
| Sludge Titan | `94dd397e-0127-4c79-b9d9-22ce62ccdfe7#card` | 2 | NP Postmodifier |
| Speak Secrets | `34d34f2c-06ff-4acf-8f2f-ff867774b777#face:1` | 10 | NP Postmodifier |
| Spirit of Resilience | `ddc85b23-113a-46e9-ab09-fae9312c72fa#card` | 315 | VP Adjunct |
| Squad Rallier | `e0b15374-b4f6-48e0-8477-4eb5d5c2a938#card` | 123 | NP Postmodifier |
| Star Charter | `578ddb51-e128-43e5-a115-471f0cae1222#card` | 123 | VP Adjunct |
| Staunch Crewmate | `8ab49974-3f06-4b6b-a416-d6f872b3887f#card` | 108 | NP Postmodifier |
| Stoneforge Acolyte | `c3c40f93-7e10-4139-b9e0-b6dfff8b8038#card` | 192 | NP Postmodifier |
| Sumala Woodshaper | `5a44062b-f1db-4122-8570-1241a16cb895#card` | 108 | NP Postmodifier |
| Szarekh, the Silent King | `9a63f950-2e8a-4617-896c-1d0af538dbc7#card` | 43 | NP Postmodifier |
| Tapping at the Window | `d5b80d04-1650-4b5d-9f9b-091ecd17b6e8#card` | 48 | NP Postmodifier |
| Tekuthal, Inquiry Dominus | `4716ab91-30e6-4c63-8389-a9db8f9414d8#card` | 18 | selected Source Complement |
| The Fifteenth Doctor | `90aa8344-ffc7-4a1a-a02a-80eceb1babe5#card` | 36 | NP Postmodifier |
| Theater of Horrors | `b391a4ce-52ce-4a8c-bbe5-6053951ee1d4#card` | 448 | NP Postmodifier |
| Through the Forest Gate | `acfe54b3-10e3-4fdb-b874-d39fde96c40a#card` | 36 | NP Postmodifier |
| Tomakul Scrapsmith | `d1559911-6621-4fd6-8b3e-16e82baf849e#card` | 5 | NP Postmodifier |
| Town Greeter | `9dc61ef6-da22-42a8-b348-dfd5722deadf#card` | 4 | NP Postmodifier |
| Tracker's Instincts | `1bc7c38b-9571-4395-be90-922f92816987#card` | 6 | NP Postmodifier |
| Turntimber Symbiosis | `403b59f3-7ade-4bc2-a3e6-de0c3c700f18#face:0` | 72 | NP Postmodifier |
| Uba Mask | `766c644c-04fe-4b01-93dd-09a50d78d01f#card` | 606 | VP Adjunct |
| Ureni of the Unwritten | `99c2d3ef-e5b4-48cd-b3f5-de9b02c7c36a#card` | 18 | NP Postmodifier |
| Vessel of Nascency | `e33dec01-c114-436a-befc-dab4e806c10b#card` | 15 | NP Postmodifier |
| Vivien's Arkbow | `efe3204d-2013-47a1-ad68-adfbb2d0be8f#card` | 9 | NP Postmodifier |
| Vivien's Invocation | `5e2b30d2-81e9-40cd-86dd-3ec5dd06ced0#card` | 72 | NP Postmodifier |
| Warm Welcome | `53807ded-5bbb-44a1-9c22-ffdd4b8523e0#card` | 48 | NP Postmodifier |
| Web of Life and Destiny | `33d556c5-1bb4-49bb-88c2-627311eb8c9b#card` | 6 | NP Postmodifier |
| Whiskervale Forerunner | `861e427f-cbb4-4ed5-8356-747ac638ac34#card` | 25920 | Clause Adjunct |
| Winota, Joiner of Forces | `02598a35-1b11-4abe-89cd-fc44c8c36c15#card` | 36 | NP Postmodifier |
| Zimone's Experiment | `5faa9e54-1cc4-48c0-bbfa-a495087c36d6#card` | 768 | NP Postmodifier |

### Every previously covered identity with fewer Readings

All **1,609** rows are `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**, against
`vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307**. Classification: **wrong
analysis retired**, with no re-coverage owed. The retired routes are the old
optional-source Return frame's absent-source branch duplicating the dedicated
destination frame, and source-marked PP Postmodifiers on the Object when the
verb selects that source. The complete source PP is retained in the selected
slot; controller and depictive outcomes are preserved. These are removed
Readings, not silent losses or unreported regressions. The per-face census
provides aggregate counts/digests, rather than individual removed-value trees;
the route justification is shared below, and the source text is retained in
`comparison.json`. No decrease is attributed to an unmeasured trunk change.

| Face | Durable identity | Before → after | Classification and justification |
| --- | --- | ---: | --- |
| Abduction | `d0e1904e-1a37-41f6-8582-b9ea794bb886#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Abiding Grace | `f4e7bc58-336d-4670-84cb-a46ca1f48671#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Abnormal Endurance | `edbf1b87-2d1e-47e6-a04e-a2b1646af7d9#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Absorb Identity | `4489f362-9b7e-4dc0-a459-a990a487454e#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Abuelo's Awakening | `2b22cf44-7b03-4919-9812-6796bf193484#card` | 4800 → 600 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Abuelo, Ancestral Echo | `eaa9baac-72f2-4bde-9146-860878d9860b#card` | 48 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Abzan Devotee | `3d02ee2f-b7bd-40b0-b9be-2420991b2be0#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Academy Journeymage | `b400700c-1d82-4721-a166-56f88ba6ad19#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Accursed Witch | `80b07882-e144-4815-8b6f-04b3ab343d97#face:0` | 60 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aclazotz, Deepest Betrayal | `fcdfe9d5-2743-4d3e-ab57-bf0f96beaa15#face:0` | 152 → 80 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Acrobatic Maneuver | `1aa1cf50-ee79-4bef-826b-5f1a9e1aa1a5#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Active Volcano | `205ffa76-ff8e-4405-b21a-42358a9883c2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Adarkar Valkyrie | `2932baae-bdf1-455a-9ac4-23ba1f83e7e1#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Admonition Angel | `78a7bb30-fc60-410c-b086-862b5521c1aa#card` | 72 → 40 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Adun Oakenshield | `aefd49a4-ecec-42cb-b882-e5db7f81eff8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Advanced Stitchwing | `7e5d2801-873f-4318-9c23-9a73e00e31a4#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aegis Automaton | `727f4734-53cc-4279-ba51-dfa26c4ceb1d#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Adept | `e076067c-77b7-4a37-a62d-51434392dd53#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Channeler | `fb220f46-f8b8-4804-baa4-e7d50b4871f7#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Gale | `b21d6482-1b3a-47e6-98a5-3067f5f3818b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Membrane | `5efea57b-937d-46ac-999e-490316e0f820#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Mutation | `6697fe5b-90ac-4321-aa2f-cdc6ec283cb4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Spellbomb | `4b033a0a-c1ae-44d7-9662-72cbbfda024b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Tide | `2fbf95b4-bcf4-4b5e-b5dc-0294f2b48d3e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aether Tradewinds | `3a0dd318-9523-4740-b0aa-a911032e3851#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aetherize | `7c779721-cd1b-4696-9ae9-68ccc284ed2a#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aetherling | `b980bf11-9e12-4ee9-83c1-45706d9ae344#card` | 320 → 180 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aetherplasm | `0cd99210-8d34-4112-8e41-7e8c5d2a95b9#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aethersnipe | `3e0be003-4f42-4917-b85d-46baf2afd734#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aftermath Analyst | `374e54d8-8e73-4268-9dde-c28c77bbbf32#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Agoraphobia | `b268fe15-2860-4730-865e-eed7f308fc45#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Air-Cult Elemental | `6ec2d957-c6d9-469f-8190-8b621c490238#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Airbender Ascension | `c4d2fdf9-637d-4e97-ae33-45f2d27bf8cd#card` | 216 → 116 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ajani, Nacatl Pariah | `2588f348-d7a3-46c8-9ace-dca53ed5ef99#face:0` | 6664 → 3528 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Alesha, Who Smiles at Death | `6969a7e2-6866-4001-a139-24b3be13deae#card` | 6 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Alexi, Zephyr Mage | `3f60de36-ed63-4d08-a012-fc16e91da46d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| All Aboard | `788d3cfa-7706-4728-9c48-cf7bc963d002#face:1` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| All Suns' Dawn | `a97d9a72-2e47-491b-b1db-9829a9abbbd7#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Alley Evasion | `b59c7c62-ce2f-4062-9790-566191aded69#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ally Encampment | `9d293b69-12b7-4b50-a0a7-c4f493dee30b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Along the Crooked Way | `f4437cb3-85f7-49b5-bb41-6a948549d987#card` | 30 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Altanak, the Thrice-Called | `6591f243-f94c-4a83-bb16-1fb838d9eb14#card` | 35 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Alter Fate | `2db17cfc-0947-407f-bd53-d64a48eec314#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ambrosia Whiteheart | `2bcc9f11-5b12-433e-9680-0f4b18aa521c#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ambush Krotiq | `e53f5f82-a8e0-44b1-a3be-162a0724755d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Amugaba | `73a1552e-bbfa-445e-8306-b347354c0364#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Anarchist | `82b9ad93-df75-4417-9746-798db8fe6ba0#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ancestral Statue | `da69fbf7-c894-41dd-85ec-217de23a362f#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angel of Condemnation | `2317a33f-665e-4cbc-bb5c-16b912ac2eef#card` | 64 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angel of Flight Alabaster | `9b6a74fb-c48f-4d85-b4b3-96eb80b9efc7#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angel of Glory's Rise | `c6a606af-ea08-4cd9-bfd9-60d76e232e85#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angelic Destiny | `8ba36878-0816-44c7-b543-8ebfc28c2ecd#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angelic Renewal | `f86bcdde-8b1f-4efa-a01d-39e09dc826d0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angelic Shield | `05b020fd-21be-495d-ae45-7de3b1224e6d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Angler Drake | `620d7146-b5b3-487c-a24b-f36b10f48df4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Animate Dead | `c0d8fef4-65f4-4769-982d-b397d2b7e977#card` | 144 → 84 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Another Chance | `df9cdc11-cb92-46ce-a83d-eb26e3cd004a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Anti-Venom, Horrifying Healer | `3c7bafe9-80cd-48d0-bcae-e7910c9fb83b#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Anurid Brushhopper | `bdecf59c-f8bd-4003-9058-685b70c08837#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Anzrag's Rampage | `ce690a21-8117-420e-be00-cd6d9923a812#card` | 528 → 264 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aphetto Dredging | `6cb8c627-6b2f-48a7-b2d9-8772defea560#card` | 98 → 33 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Apprentice Necromancer | `d1a6aa5f-2ad6-4bc7-82f0-4262f66af730#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Arcanis the Omnipotent | `8a7183cc-161c-444d-a889-a17519c8061b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archaeomancer | `a91a3266-cadd-47a0-9b20-160307f14c07#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archaeomender | `5a4553fc-f945-4094-b231-2d378ed6609e#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archenemy's Charm | `edcb0248-e7c9-4df9-8dba-982048654fd2#card` | 28 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archon of Falling Stars | `38e0a1d0-2a4c-4510-9610-abea7fa1de7c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archpriest of Shadows | `7b03910a-2ca7-40cb-9c71-1c2cc856a287#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Archwing Dragon | `a0a706b2-b237-4183-82fa-2140a23b89e3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Arclight Phoenix | `a80ea391-54d1-4b61-95a3-f8deb4cbe842#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ardent Elementalist | `1edd4f50-05ee-43e0-a288-c26666910c8b#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Argent Sphinx | `7220c391-8d0e-4f43-94fc-ff77ffa2ee8d#card` | 48 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Argivian Archaeologist | `4b889ec0-6130-4e31-bb02-03fdabd28bee#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Argivian Find | `0b6c74a7-d7d3-4c05-84b1-bcb580bb476f#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Argivian Restoration | `d887919a-68d5-455d-977c-4f1f0b4b3141#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Arm with Aether | `e6479195-cbdf-4d2b-9f45-5fd0a6ab4a77#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Arrester's Admonition | `ccfb612a-762d-41a7-bc3d-268c6abf8e85#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Artisan of Kozilek | `19409704-09c4-4a4b-a5a7-f95120b425db#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ascend from Avernus | `451e8ece-7389-4d52-8bc9-450cb5e53e6a#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ascent of the Worthy | `d748bad4-dd4c-4553-9fcc-e462260f6ff3#card` | 5040 → 1008 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ashcoat of the Shadow Swarm | `9cc69ea5-42a2-4306-ac6d-cff5adb20bcb#card` | 84 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Asmodeus the Archfiend | `1e4032f6-9741-4845-9325-781c9e489172#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aspect of Mongoose | `dcc55415-216e-4e18-83d6-43f301359f84#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Astral Drift | `7aa5db50-5057-45fd-bce9-fb4a5e874263#card` | 54 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Astral Slide | `70f14b10-8129-4249-a267-8b4be5d9befa#card` | 27 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Attunement | `5ea46ccf-5e36-4889-bf64-00d4f327e9b2#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Atzal, Cave of Eternity | `7d6ccd0b-df16-40b2-930b-bcde0b6ef73f#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Atzocan Seer | `affced7f-797d-4e50-8a24-2da14de44e43#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Auntie's Snitch | `d2964a68-36d9-4b05-8f34-540969b410bf#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Auramancer | `bd5eb181-6a69-4dc2-93a0-fa000291bc3d#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Auriok Salvagers | `db2f3a06-f75e-4689-b147-9820aebb30e2#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Auriok Survivors | `103b92c7-8f8e-4924-8d7e-fc0a01486e86#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aurora Phoenix | `a98cc4d0-a322-4142-aee0-83f39b137616#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Auroral Procession | `511d3de9-076b-4811-bddf-00623f98993f#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Avatar Destiny | `ecb81888-589b-4486-a444-e0c26384854e#card` | 2160 → 1170 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aven Augur | `e68d1b86-3996-4bbf-9086-a76a7f9ce2a4#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aven Fogbringer | `3fb3d4b3-1034-449c-864b-633e7f39d73f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aven Surveyor | `57831c07-6915-4579-b3e1-de93a3664444#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Avengers Quinjet | `905e4bd1-f5f2-4691-b13f-ce2628d26a38#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Aviary Mechanic | `b47d37ee-2bb2-4ca8-b245-94d2cc4b6684#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Awaken the Honored Dead | `5f694917-f434-4459-bc86-caf9ac1c80e7#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ayara, Furnace Queen | `f9ef90b9-a2b4-4a5b-92fb-2268ea7d709f#face:1` | 32 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Azorius Chancery | `189fc8f4-17ac-4f1d-82c8-8401445bdaf4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Azusa's Many Journeys | `02c60b8b-f665-4279-a00f-ae03991df4a6#face:0` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Back for More | `670df8be-21cd-49d8-92e3-d7cc38b256d6#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bag of Holding | `63c04040-e109-494e-baa6-c639a6c9a996#card` | 32 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bala Ged Recovery | `d2075f58-b0e9-4e85-b7e6-0523a27a1d5b#face:0` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Balduvian Atrocity | `0dcd13ab-37f9-4d66-8227-1e72d920f297#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Baloth Null | `55a0a40c-cb40-469d-9bf3-9c33b378f4f7#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Balshan Griffin | `0ef74c49-0825-4201-b23b-dc6c6bbadacb#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bane Alley Broker | `4993e373-4b76-4582-a789-6a3123e7ca89#card` | 360 → 180 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Banish into Fable | `1f18e63d-ab1f-426c-afd5-ce89433394ca#card` | 9240 → 4620 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Banishing Betrayal | `6b633e58-ea1b-4ed0-addb-6dfd93645ae4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Banishing Knack | `6f23621e-eceb-402e-a8d3-8e2b734762b3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Barrin, Master Wizard | `8c6d6684-1943-4af3-98da-951604193911#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Barrin, Tolarian Archmage | `f98f7cb7-90ca-41e7-9f1d-8054931ecf0e#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Barrow Witches | `9d4b34cc-0452-42a4-90e4-2a48e7ffadc0#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Batterskull | `d12e5ce0-5705-4c35-9a93-b883db52c80c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Befriending the Moths | `2ee5f5ad-2f16-40d9-831a-2aefece31b36#face:0` | 66 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Behold the Sinister Six! | `6fa27f8a-bade-460f-853a-abc1c05c946a#card` | 27 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bespoke Bō | `679480aa-f9ad-4a6c-bd79-db9f9f75433d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bestial Bloodline | `a3ae6694-6314-42e2-bc12-d0e32d49b347#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Biblioplex Kraken | `a1bdba37-f034-412f-8395-adbaed95ad11#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bigfin Bouncer | `4e473298-0b67-42b8-90a8-3bcdd73da473#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bind Spirit | `4af03564-1d1c-4582-9248-cfe9551bad30#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Biolume Egg | `7af3876e-5297-46e6-936e-6c8344f4b6ac#face:0` | 43 → 23 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bishop of Rebirth | `05058594-608f-4046-bf44-b736a7072f0a#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bladewing's Thrall | `f068a0f3-48bc-4d5d-be79-c23c0be11c27#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blessed Ghoul | `fe99d90e-4809-4310-a3f7-ceb540511d55#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blink of an Eye | `51acd176-e84d-42c2-acb2-cceeb28a9fec#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blinking Spirit | `a26f6958-9bc8-46f8-8ca7-7f55098e1291#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blizzard Specter | `b6f4148d-8511-46b3-93ec-2f5eb1e4ffad#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blizzard Strix | `c687af9b-2159-4965-a068-ce4db1ce801c#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood Beckoning | `67a48e3f-2388-42a9-a8b1-97c08761f807#card` | 35 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood Clock | `ef987ddb-25f6-4895-8f83-045254745ea4#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood Fountain | `abfa5753-8c02-4e2f-8f98-b0636907293a#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood Spatter Analysis | `aa929252-5dcc-4c9b-9e7c-61d0bef98d6d#card` | 180 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood Speaker | `2659c0ea-9ae0-4be3-b9f7-5a38230815ad#card` | 36 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood for Bones | `fc7439ec-32e3-47db-9ff4-6eb84cb33d89#card` | 50 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blood on the Snow | `45f386ba-ad62-4c0e-b6ad-ae02ca8f2bec#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bloodfeather Phoenix | `f4490b09-dc55-47ed-a573-4ae0b95725b1#card` | 60 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bloodline Bidding | `bdb9e933-f679-4930-b669-0d123db09d16#card` | 16 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bloodline Necromancer | `f2a54e85-dd8b-462c-a811-c9553dc349a5#card` | 16 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bloodsoaked Champion | `cad09970-14c8-4d80-82fe-6c855efb0191#card` | 30 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blossoming Tortoise | `5e1bf23b-7fb0-45ff-8544-fce9fa3eba00#card` | 420 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Blur | `00b36996-43c6-42a5-892c-c7c8864cf973#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bob, Reluctant HYDRA Agent | `9caa490e-dc28-4069-9ffb-4b1afd8fdd54#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Body Snatcher | `e04688e0-672b-407f-a2c9-fd7292926483#card` | 45 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boggart Birth Rite | `030e9e3e-ce78-4c3b-bd83-969c07c70459#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bond of Insight | `3a877718-ff90-4d6e-9f95-47b06adad7f8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bond of Revival | `62cca25f-3bf7-4b67-97d5-280bf2544722#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bone Dragon | `51926430-b98a-424d-9347-c36938132825#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bonebind Orator | `c79cf0e5-52dd-4af3-ab06-6cdaab526c23#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boneyard Scourge | `eef40ab2-9e17-4e8d-9e58-1a82ab871b5b#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boomerang | `dc4a4996-108a-4aac-850f-2d9f76403446#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boomerang Basics | `acd881eb-f3b4-4174-9fc2-e3c7c213886b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boros Garrison | `8fa3ac81-3dfe-4565-be99-5554f7597b4b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bortuk Bonerattle | `d965e8c2-34b1-42eb-9647-59951d9fc06b#card` | 1344 → 672 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Boseiju Reaches Skyward | `ec08aeb3-bba7-4982-9160-68d25bd411d6#face:0` | 462 → 252 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bottled Cloister | `1fd5e5e7-f23d-4891-86c4-268195ea09fc#card` | 216 → 108 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bounce Off | `d9c29e6e-5949-4599-bf74-b9539e753f76#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brackish Blunder | `4ad34f85-a185-458a-a7a1-eb60f7efce5b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brackish Trudge | `c775d3ef-2184-4425-8c8e-2bbc49687f76#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brago, King Eternal | `fcb0c7db-bd07-4c25-b7f8-b6c207e4f6be#card` | 444 → 240 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bramble Familiar | `68f36dfa-bbd2-4860-b09b-37e35136be72#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Breath of Life | `30d9e200-b944-43ff-89b8-a550a788ae03#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brilliant Halo | `8f68f6d5-1953-4ccd-8af0-7d0a69139556#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brilliant Restoration | `9584a8ae-2aba-42b8-8983-0467d6bd5698#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brinelin, the Moon Kraken | `1f00edd6-4321-48a1-9053-77c77b922aeb#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Broken Fall | `79791c7f-dd33-406a-8c2e-6b722abd5161#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brood of Cockroaches | `7a22d694-29d1-4e7b-8fca-d7008aede489#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Brotherhood Outcast | `79d0cdc1-2e27-48c5-8840-430490068f68#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bruna, the Fading Light | `bce7e45d-b2c5-43aa-91a6-3a3c24f30850#card` | 32 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Bull Elephant | `4ed04ece-64d2-41a4-bf36-b993cbbf2aac#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Buried Ruin | `3644f316-f9a3-46c9-9b1e-747f86cf4ead#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cabal Surgeon | `5528d0f1-4774-419f-ba1e-b2abdbce8735#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cache Raiders | `57947037-91b2-4058-be22-158d3257317e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cactuar | `3e61d7c4-263f-4077-bdbc-d79020f2a479#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cadaver Imp | `7265d768-b79f-44eb-9873-d834e55f6463#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cage of Hands | `18ffd8ad-1556-450d-8bc1-8103a3d0404e#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Calamitous Tide | `d605d9d1-c01a-4ef6-89f9-06fae8cd57e6#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Call a Surprise Witness | `9038a457-ce01-4f6d-9e2b-5220f08b4290#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Call to Heel | `8ef3dd68-f6a1-43b9-8837-40b326019b1b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Call to Mind | `72e2b883-a284-4ab6-8174-1ca289141a49#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Call to the Netherworld | `a28ef7a5-4b47-4429-838b-a64a7c0701d0#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Callous Dismissal | `9219b7d5-642d-4470-a7c4-f3e56bfae8c7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Campus Renovation | `d34a3b58-a905-4e90-b35c-9f5c21149b04#card` | 48 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Can't Stay Away | `69ae219e-bf97-4626-9b16-7901f51f0343#card` | 24 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Capsize | `77637eff-2963-4402-88f3-ca346f762fc8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Capsizing Wave | `d60e14b4-f750-40cc-85b5-17c87ae8a584#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Captivating Gyre | `41011f09-ad8a-4eea-aa52-d20a219bc94c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Carrion Cruiser | `98a61659-9882-417c-9c16-16ebb26f9037#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Carrion Thrash | `89599a64-19fc-47ad-a116-3ce0a10fa546#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cartographer | `50d33663-5d83-4e3c-9ed7-074fe29a9fc8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cass, Hand of Vengeance | `f9b6b1db-37c6-43d4-b762-6c9770da5964#card` | 80 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cauldron Dance | `e9784eb4-96f2-409e-a7b5-eb6478644943#card` | 160 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cauldron Familiar | `a7dc2e62-1c50-4ed7-b71f-2d782a447a5e#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cavalier of Dawn | `7c520355-1ab8-4d6d-9a29-0f63d6b60024#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cavalier of Night | `27c2b8ce-17b9-4f6c-8444-3bdc636dd413#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cavern Harpy | `81c40bd8-b989-41e8-9527-eae8fb87311d#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Celestine, the Living Saint | `6503ed78-6499-47f6-856b-ee1003ce4a4f#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cemetery Recruitment | `07792d72-d5db-41b9-871c-e720e12ce659#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cessation | `a8b3a35e-0172-4f7b-b50f-571e59d7ca5f#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chain of Vapor | `2167ac25-d042-4b48-b770-8b94acc1a965#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chakra Meditation | `12c57ebe-35a7-4cb3-936b-99c148a3cfed#card` | 960 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chamber Sentry | `65f8e726-be50-49ee-96fc-d3a2080d7b8c#card` | 99 → 33 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Champion of Stray Souls | `4edbaac4-dcb3-40b4-abd7-ff2683bd366d#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Champion's Victory | `8df38d95-1707-4f83-9d64-41f0be2a3b3b#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chandra's Phoenix | `65ddd04c-7207-4f6c-ae1b-c5e02a520a51#card` | 360 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Changing Loyalty | `cdfa9de4-7d7d-4638-8ac4-793ece6372dd#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Charming Prince | `c48d844c-3976-4fa5-8e0d-3f0e535e7619#card` | 48 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Charnel Serenade | `54c3d8ba-ee7c-4f15-8ee3-e5daf860b428#card` | 700 → 140 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Charnelhoard Wurm | `4a430fa3-e693-424b-9981-d7d8193445e3#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chronosavant | `969c9864-f78b-46b2-b35b-c9adaf355c2d#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Chulane, Teller of Tales | `ebf7ce9b-9e5e-4557-9e28-76556997f0ee#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Churning Eddy | `774e53b2-5aec-43e1-b4fe-8812b8430401#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cid, Freeflier Pilot | `d3bdfb25-dfb2-4df4-b502-ef0909a39f86#card` | 336 → 56 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Circle of the Land Druid | `b6cc351e-6afd-4427-998d-e06e430cd0e6#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Claim | `16da9e4b-c6ee-4e70-8653-31ca147f3b01#face:0` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Clattering Augur | `d88f249f-5bf1-448a-91d1-f81538dfd475#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Clay Revenant | `c183186b-d92c-45a2-92d1-42e0df2b441f#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cleansing Meditation | `658bccf8-fe73-4d6a-b37b-7a58034e5e5d#card` | 64 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Clear the Stage | `77b6a3db-c098-4b06-bc7f-136eef892d30#card` | 28 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cleaving Reaper | `70c268af-3fb4-42b8-a87a-d06d33c1c641#card` | 42 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cloud Cover | `a4cd8dd1-adb9-4c4c-bda9-6f6b6c167702#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cloudshift | `6879f5ce-7a1b-4606-bad1-885779b0d456#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Clutch of Currents | `2c6658c3-8c0e-46c9-8127-bc5b77eb0dab#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Clutch of the Undercity | `316cae8b-31b0-4b76-8b72-8122005e1b4e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Coastal Breach | `e42b6bd7-8fdf-4452-85cc-9aed790ff682#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Coastal Drake | `d4220231-e9ac-4390-8c7b-979549b26d33#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Codex Shredder | `ef7b11ab-24e7-4e7c-91a7-920bade6e60b#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Coffin Puppets | `e49771ab-cabb-4a5f-aef3-a0ac8147e6bf#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cold Storage | `e23bf6bd-bc97-425d-902a-c6b606e82b65#card` | 35 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Colfenor's Urn | `920c4df3-9517-44cd-8304-28b1ef69d60e#card` | 99 → 54 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Colossal Skyturtle | `871e4c4e-8105-4cb9-b1cd-53c5f8e64494#card` | 24 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Come Back Wrong | `2a9783c5-084b-4cd3-b7fa-ae9d7fed51d1#card` | 36 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Command of Unsummoning | `fd57dfa5-f29d-4b79-8748-c34a73efb7f0#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Compelling Deterrence | `fb0c9048-73ee-434b-a018-dbfbe32e56f5#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Complex Automaton | `e4951b4e-ba68-495c-8062-e89250d02cf3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Conciliator's Duelist | `d6ea0e58-7b14-425b-9135-57e149c0e47e#card` | 320 → 180 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Concoct | `6e07e7ae-1bb7-4831-adb9-a5e71b7d79e3#face:1` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Confounding Conundrum | `f86b9a01-62fc-4120-8914-5054ed48d37e#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Conjurer's Closet | `cd1eda60-53e4-44d0-9b2c-7a57395e291f#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Conqueror's Foothold | `88b18901-50cd-461c-b1bc-be900210be8e#face:1` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Conqueror's Galleon | `88b18901-50cd-461c-b1bc-be900210be8e#face:0` | 72 → 39 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Consign | `8fe7436b-2467-486e-8a2d-7cbf21a65da9#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Consign to Dream | `624bdbfb-b611-43b6-a3c5-cc8b11dbfaae#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Consuming Vortex | `863bb8d8-4571-4775-87e3-d6f3ec88c835#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Contempt | `811c8364-fafb-49c8-a5bf-7b6cdfceb475#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Convenient Target | `2f214dcf-9601-4ccb-bdac-2f9b9f6dff99#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Conviction | `4374b6b1-e691-4823-802d-8de8530aa461#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Coral Atoll | `3f347ebf-e0d2-4ae0-ad84-df7a460404e0#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cormela, Glamour Thief | `3cd9bac9-0abc-43d8-9f84-94f965f7a2e0#card` | 16 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Corpse Churn | `e2aeac0b-af74-4c47-9ac1-51fbf869c95f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Corpse Cur | `8f59e4d9-67ba-4dc6-ae3e-183e63673318#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Corpse Dance | `59b02ca7-e7bd-4820-ac32-87ea1a8c64c0#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Corpse Hauler | `c2ba1c3b-08c7-4835-81f4-2f98843fb7b8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Corroding Dragonstorm | `4c55ca3b-1254-4d44-96ac-1e1cadaf49f5#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cosmic Intervention | `cddccc2a-a76e-48b3-b4dd-dfeab89e1619#card` | 432 → 243 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Counterintelligence | `0c4e64d1-cdcf-4e96-8bf4-ed1414f6ad62#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Courier Bat | `d8ff239c-cd71-4cd2-9e96-792a116989ca#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cowardice | `2692dd66-c200-4d0b-9427-30deaa2342ab#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crackleburr | `59a6cb4e-1646-42b5-b482-da641c0f7f6f#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crashing Tide | `4c3f87d7-a632-4b8c-9052-b4055ea88771#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crawl from the Cellar | `16c99713-f19e-4b25-9b03-6a8d8203adfe#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cresting Mosasaurus | `bf245433-8e3b-47d7-b628-3cdae93c384d#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crosis's Catacombs | `e9a7dede-3968-4b0e-a707-419d46a6fec9#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crosis's Charm | `e59d70a2-40ac-45b0-995d-65b9caa290c8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crovax, Ascendant Hero | `0cbf6ad8-9bf1-455d-bd74-9c142e17acb0#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crown of Flames | `f14fb965-5ebc-427a-a5b8-149f54c64171#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crown of Skemfar | `db961314-fe78-4362-902f-aa0dbf6c487d#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cruel Revival | `836d074a-8888-42a0-8dce-11ffce00d2f5#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cruel Ultimatum | `01294ac9-1a06-4ee8-b3b1-db1f07c9d94e#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crypt Angel | `665f3e52-c536-481b-8b29-fe68f08d1fdf#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cryptek | `27a4b633-5a62-4d8c-8cc6-44959c311de9#card` | 42 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cryptic Coat | `73b8adea-1786-44fc-b190-bf240b07f5b6#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cryptic Command | `a3e51a35-09df-4189-b131-08a21e6a557d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Crystal Chimes | `fbff7452-1da4-4cd0-8247-a625e4b4da2b#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cult Conscript | `e5762815-5fc3-4ae7-abee-69f27c5e4e90#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cunning Evasion | `3e56606d-562e-4c77-9769-213997fea28e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cunning Geysermage | `746768bc-9abc-495f-ad00-4b58497dce0d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Curfew | `6ff1e793-1569-4887-b6bb-d9855c5e8a26#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Curious Cadaver | `cc5370e0-a0e6-4958-a6f2-093a3868b575#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Custodi Soulcaller | `dc25bdde-1c73-4c61-99d8-b772aae278cd#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cut the Earthly Bond | `127ce189-2758-4789-9e02-4aa71d013431#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cut the Tethers | `9af9b8de-eff6-48da-979b-4125e7965a11#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cyclonic Rift | `d75b9c82-1b49-4c3e-a1b5-aeef57d6644b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Cyclopean Snare | `6f5a86bf-caa7-48c9-a59f-b13fb483a9d4#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Daretti, Rocketeer Engineer | `0eb1d539-6301-46b0-903e-487eda34253f#card` | 50 → 25 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Darigaaz Reincarnated | `c2f42c67-ff88-4248-bfa5-79a18c6473d5#card` | 560 → 280 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Darigaaz's Caldera | `19b58ec9-bb88-4193-8ea8-c8f09ceec1ed#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Darigaaz's Charm | `7ee01801-5e42-4916-a359-64e9975090a7#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dark Intimations | `228cb63f-0b93-4be5-94e5-4eaae9112eeb#card` | 2688 → 448 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Darklight Phoenix | `e4b51c2d-7e56-435b-89d6-85e33df65879#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Darting Merfolk | `2f33e684-a4e4-4528-9a20-3df902bc2cde#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dauntless Avenger | `bfa60dd1-fcc8-4126-ab7a-4695c74f5bce#face:1` | 3 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dawn | `7bc3f92f-68a2-4934-afc4-89f6d0e8cf98#face:1` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dawn Evangel | `4d6e38f6-480b-4f81-a5d4-342fc45e39b0#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dawn of the Dead | `309e61f5-1fc7-4481-8d3a-9b35d335c99f#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dawnbreak Reclaimer | `6fa7229d-18b6-43d8-aa57-a56cdb815767#card` | 639 → 355 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Daydream | `094c4486-6cae-4268-850e-ff0082e5ad10#card` | 276 → 152 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dead Revels | `4354eb0a-28c5-4f3f-8e78-fd7eec230c7b#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deadeye Rig-Hauler | `1355424e-9909-41dc-8d43-b8d21b0919bf#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deadly Plot | `6178715c-870d-4710-b758-66e080804ee3#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deadwood Treefolk | `b7efcb42-aa52-4d13-8c7c-b2db2dd51afd#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Death | `d476f4b3-db63-4756-ab76-4b35f63c2825#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Death Denied | `94f23a93-cd98-4ee7-9e62-9b7b49ace6fd#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Death Tyrant | `fa944633-e078-41bb-9c7d-4f6b669c27a5#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Death's Duet | `28d24add-da3a-4697-8fd2-5106c9225c0e#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deathless Ancient | `990dde55-8ee3-41ad-874b-60864ec28ff4#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deathless Knight | `f96f1a94-6226-4e1f-a7fb-a43a276b3c3a#card` | 24 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deathly Ride | `fab8d954-807b-426f-8057-99a5d2fec618#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deathpact Angel | `ad0f50d5-7b40-4fd8-8d13-4a0493398f53#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Decoy Gambit | `94d9878e-9786-438c-8b33-c1dff730b748#card` | 44 → 22 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deeproot Wayfinder | `3ec767f2-40ba-46c9-b7cd-5821986dda0c#card` | 42 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Defossilize | `d620de15-8141-4877-9330-8328a3fa85e1#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Defy Death | `dbf9c3e8-35dd-4b80-bd23-f43e21cfd991#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deliver | `6343aa78-2df6-41f3-9366-4933f9e6a93e#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deliver Unto Evil | `9e73f938-0aa4-4639-a97a-f7e68f645623#card` | 56 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dematerialize | `d2fbad7d-d05a-4321-9aff-a93bd102a3f4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Demigod of Revenge | `602ce676-380d-4c3c-b97a-9234ccdc2904#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Demonic Gifts | `162bcc9d-6edf-4b3b-83e6-2c933e2a5967#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Demonic Vigor | `00cb93a8-5d33-4c6a-9e2a-4b3560f76f3f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Denizen of the Deep | `fc1bb5b9-5e36-4154-b683-e62f4f66f2b6#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deny Reality | `f9e31993-bfe1-46fb-8a32-418e97f9cfe1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Depart the Realm | `ae069ba4-8ca6-4eda-8f5d-fc82072d7d40#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deprive | `1065eceb-9150-4ea4-bd6d-acaabda14e00#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Depths of Desire | `caa0c20c-594b-4ffd-87de-60bf15d8c3be#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Deputy of Acquittals | `3cbb5045-8566-4279-b7d3-3e599b11ccc5#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Despoiler of Souls | `cabb0cf3-124a-4f5f-9ca7-2d45a72159ca#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Despondency | `a68ac1ba-f51e-446c-abf0-7a2bd4d65a8c#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Devastation Tide | `4245ee98-2d4c-49d1-8d07-80760cae2bf9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Devoted Crop-Mate | `fcde4643-a212-4a9e-94c3-0a0a95841bc6#card` | 24 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Devour in Flames | `ea1fc580-a640-4f49-8f29-405c2830eca8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Diabolic Servitude | `e275cbfb-3d25-4189-84f0-3162eadc7d65#card` | 540 → 54 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dig Up the Body | `144e3d90-8ed8-4c21-941d-fc59cec042a2#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dimensional Infiltrator | `b93732e4-7aeb-4c37-a415-2b0ff46e2056#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dimir Aqueduct | `378a1d57-e2f1-4b84-9692-1564602e9e99#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dinrova Horror | `b1eb7a92-3055-426e-9716-cc6d2a69f0af#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Diregraf Rebirth | `1920254f-0f65-4426-aabc-f3807476b67b#card` | 30 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dirty | `eba21e3b-e2b2-4e0b-82e3-f0849943fd89#face:1` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disappear | `039e2e39-8f7f-43f2-8448-de35ef7b8dc1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disappearing Act | `20393369-fdee-45ea-8584-451f38b1c002#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disentomb | `e4ab986d-662c-4879-b6f1-6887a2ec4d7c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dispersal | `8f1c1bd6-8420-4de2-81f4-5faa419da301#face:1` | 110 → 55 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dispersal Technician | `47a66451-8a4e-4fc6-8685-96e1feb8f997#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disperse | `9d8fe3f8-5027-4f9e-999e-e277caa96478#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dispersing Orb | `dabc4ba1-3f90-4cea-a737-d41537cce729#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Displace | `334e2147-5944-4558-b2bc-2f2bfb96dfa5#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Displacement Wave | `2b3fc609-7b19-441a-ab7e-58ccd70d955b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Displacer Beast | `7f24c0b7-1d39-48b6-969f-ee97fdf99305#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Displacer Kitten | `09ef446c-a13d-49d9-a94c-cd5f5a2d440b#card` | 222 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disruptor of Currents | `e76ae81f-729c-4def-ae83-6870a696cbe8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dissipation Field | `35426427-4268-4c8f-9fe1-270f2ce43d97#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Distinguished Conjurer | `6df57d67-2fd9-4e7a-b67b-f361fc30e496#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Distorting Wake | `e5387fc7-c3b3-4933-bffa-580ad793c7ab#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disturbed Burial | `cdbaa21d-5896-4b8b-9a08-f0839ccfddfc#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Disturbing Plot | `3efecb6c-7c21-4b3b-9612-e3916f91c680#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Divergent Equation | `84681f12-2978-4f62-ab89-b579033c5192#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Divide by Zero | `7b1c1a0c-f443-4eb5-a430-9eea886361ca#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Djinn of the Fountain | `dcf28f91-1bf4-48be-a3d0-e061f443fd9e#card` | 128 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Doctor Jane Foster | `b1c8fddb-d686-4b4a-8aff-d79ea747801c#card` | 48 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dogged Detective | `c5a1516a-5e01-48c5-88c7-2178f4b2ad24#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dogmeat, Ever Loyal | `5232fcc0-5dba-4569-8e99-c0b2c230e9f0#card` | 36 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Don & Leo, Problem Solvers | `a4ae71c8-ea1b-4fa4-9e47-cf00ab60c5ee#card` | 36 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Doombot Harbinger | `37042a3e-c377-4282-80f9-068e4955f8a1#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Doomed Necromancer | `155422a0-a0cd-4399-8ed9-fa68ac2c80a6#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Doomsday Specter | `5811effb-5010-44b9-bb97-9d7eb358327f#card` | 168 → 84 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dormant Volcano | `38ba1956-5505-4a7a-b6af-e75715b1401f#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dose of Dawnglow | `f852bc33-e2f8-43be-9e6e-7badd1f894ef#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dovin's Acuity | `898e480d-c44e-4080-910a-f53e4ee471e1#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dowsing Shaman | `c9cb34df-75b1-4dda-8f22-daadf59c5332#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Draconic Destiny | `f034137e-c19f-48af-a2f2-f767bf87a275#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drafna, Founder of Lat-Nam | `ed2e8718-4b20-4310-96ce-05a29a24ef32#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drag Under | `021d8596-782a-4cfd-8b70-ac5bb8a5aff1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Breath | `3535daf2-7dce-4fda-92ce-ad2a1ee95197#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Fangs | `0131d2ac-d677-45fd-9729-a5428139cb5b#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Mask | `fce29dd8-23b1-465c-a977-526f716dc8f6#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Scales | `04628d92-bd5e-4aff-830d-7b302bd6459e#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Shadow | `c9708a12-96b5-43c1-ac2d-d6dc6b03beb7#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dragon Wings | `85271333-af96-447a-8a16-9fadf0eaedb2#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drake Familiar | `5d144825-594a-454b-9276-fe92e87c19c0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dramatic Rescue | `a5386844-ea24-48ee-9337-ae10bcfddfb9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drana, the Last Bloodchief | `3ae3b901-4d36-40e2-b861-26209fa1e823#card` | 1728 → 864 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Draugr Recruiter | `f854ea0d-aae3-4b6a-9767-6ac1788f2190#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dread Slaver | `8110f092-41b0-4e53-a7d7-f7e3cbc4a52a#card` | 540 → 300 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dream Eater | `4ec84bb3-796a-4074-ac9a-1bd277158896#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dream Stalker | `6e67551b-1f6e-4ef4-9baf-520607b8cb61#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Driver of the Dead | `7c179e54-3beb-4761-bddf-41fa98b082db#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dromar's Cavern | `d8b57707-796d-4488-8f91-65bb75bc6281#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dromar, the Banisher | `35ad7f82-a1b1-4aa7-a8b0-3f99ca2cff52#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drown in Shapelessness | `a3874282-9887-4b53-b80e-564ac99cca23#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Drownyard Temple | `c30f9be4-c274-4ad0-b5d7-7d3421aa4277#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Druidic Ritual | `79edf527-2de1-433a-9cac-c91f461b1ebf#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dryad's Revival | `8a69a0f0-5f0f-4098-ab6d-be3233af5cd3#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dune Drifter | `b32649cb-bf5c-45fa-86f0-8bf4e8555f06#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Durable Coilbug | `a421582a-f673-4f43-beb5-28ee785da11e#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dust Elemental | `4d08e822-6535-44b7-a100-e5e1e8f5c3e3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dust Stalker | `69cd25d2-206c-4572-95e7-d66973c94358#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dutiful Attendant | `e9195f10-8cc2-482a-9e28-0be9a37099b4#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dutiful Griffin | `f91ff377-a997-4f24-926c-76edcbf91a6c#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Dutiful Return | `c8f958da-3a31-4064-9a78-66e9b65b85ab#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Déjà Vu | `7408b9c5-7266-4627-be4e-b691cf5c622c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Earthquake Dragon | `1903200d-0350-48a9-ac7c-b851f7df647f#card` | 15 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eccentric Farmer | `b51a852c-ccde-4d23-a3f3-a002f7eacdab#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Edea, Possessed Sorceress | `7beb7505-b765-486d-a205-78325922a10f#card` | 440 → 242 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Edgar's Awakening | `2c04371a-af69-4e42-8cda-096cf8312b5e#card` | 25 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Edgar, Charmed Groom | `96cb97ca-4ce0-4d33-86fb-4db4ce1956be#face:0` | 180 → 96 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Edgewall Inn | `4dfd33e8-7e30-493b-8564-f7df5f0257aa#card` | 7 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eerie Interlude | `0634091a-a74c-4cea-b6d1-7324a725554a#card` | 48 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eerie Soultender | `a3e4d14b-818a-4a4b-8462-1de9ad844941#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eiganjo Free-Riders | `0e381b7e-2d32-4a9a-a327-e9dd1236dc1b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eject | `9a18fb98-796a-43b1-8157-e3afca53a1c0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Elder Owyn Lyons | `19788405-7141-475e-94ee-d0c89488f50f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Elderfang Ritualist | `29c1f80e-f7e2-425e-a225-e88351fcbaab#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eldrazi Displacer | `9b1f552a-bddc-4fcb-ac67-b4a65b2f48ba#card` | 36 → 22 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eldrazi Ravager | `25a73cfc-a40d-47ea-a7b1-61c7b62a9e6c#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Elrond, Moon-Reader | `3f4d6f91-95ad-4687-8899-5a21a0abb49e#card` | 64 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Elspeth Conquers Death | `40c88392-2aa7-4e12-86a4-8d680219e670#card` | 1152 → 192 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Elven Cache | `8a922366-ee2a-4ddb-a904-683dcf4f191a#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Emancipation Angel | `3834d3b0-9b66-465a-a8dc-22875a819fd9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Emerge from the Cocoon | `fed2127c-7174-4863-ad6a-1847c96bbd3f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Emergency Weld | `86b0e978-5b4e-4bb7-bfda-be899c632d8e#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Emeria, the Sky Ruin | `cc999cf2-c99b-4911-8c52-6cc4a99fcc7b#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Emiel the Blessed | `b11c250c-f191-4c52-ba02-a9176f163447#card` | 132 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Empty the Catacombs | `49e0af9d-eb76-4ad6-b60a-430ae353728c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Encroaching Dragonstorm | `1e95c273-7fec-4f8d-8be9-e21ad4b93717#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Endless Cockroaches | `9f31dbb1-c350-46f8-bd1d-9f23a073d2f1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Endless Sands | `c4033d97-769f-4811-8b11-f85b8817b7a2#card` | 45 → 25 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enduring Courage | `b3de29cb-7d7d-4566-b5bf-37e571bd5d78#card` | 52 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enduring Curiosity | `9d2460c3-8eeb-4f35-b6f6-748c478664c7#card` | 39 → 21 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enduring Innocence | `98a389f4-2905-47f3-b60e-3d4afb3e5cb0#card` | 65 → 35 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enduring Tenacity | `98e698ae-1a69-469c-9cfb-0e3fedeb71d4#card` | 13 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enduring Vitality | `3577c47e-76d3-4659-b922-31c4b74be3a0#card` | 13 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enigma Eidolon | `4489a526-0e07-4d37-9e83-628f7eb256c6#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enigma Thief | `d39a1e12-bb6b-4e7a-96a4-5dab98084716#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Enslaved Horror | `f7920a2c-d14a-4ed6-8341-96751322f4e7#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Entomber Exarch | `e820296a-81b0-401f-959c-7aed8abefce1#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Entreat the Dead | `2de6c3d9-1759-40a2-99c6-8cbe17b4bcdd#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Entropic Eidolon | `be5f788a-8a2e-447c-9e65-f9fc197ddfc9#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Entry Denied | `52fd9309-b849-419a-8d19-5882c00a72b4#face:1` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ephara's Dispersal | `1e0bb57a-f5c7-45e9-b5b9-5bb3ec930108#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ephara's Enlightenment | `da848568-1c66-4ce1-87f0-a35c317f2c61#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ephemerate | `0fd57894-b917-41c8-a394-360d1d31b236#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ephemeron | `e4f28144-71c8-4393-b383-047529fe8878#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Equilibrium | `cbc052e7-817f-43b0-b8e3-96b1644d798b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Era of Enlightenment | `a6d646e8-0d84-489b-a46a-3bd5b1141864#face:0` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Erebos's Titan | `1341b64e-1f8f-4613-abea-a337cd0169de#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Erratic Portal | `4a85cd7e-625b-4043-b7f1-999def26eb36#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Escape Artist | `74f4d505-e114-47be-be27-a7949e8abdc7#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Escape Protocol | `e875b468-b475-4573-adf6-312ca4ab6994#card` | 74 → 40 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Escape Routes | `65260087-48b5-4968-8584-7cc561bb312c#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Esper Terra | `54ca5904-9099-497a-9684-101656025487#face:1` | 56 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Esperzoa | `c9a188f9-2e65-4a66-826f-0150f25730af#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Essence Flux | `64824ae5-efab-4b55-9d3c-b9c690bad857#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Essence Fracture | `31a49fb9-1081-46c3-a352-0444869afdc9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Essence Reliquary | `528039f3-e8e6-499d-add4-43cf2d731b65#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eternal Taskmaster | `a7c000a8-1864-4bb2-80fa-a61d74dbd966#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eternal Witness | `30b24e8e-3b0e-4d8e-90f3-f66eb7c1858c#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ethereal Forager | `a3f3a5b5-b931-4961-828c-501a33e5a0a0#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Etherium-Horn Sorcerer | `01afb7a1-6427-4bb6-935a-613e9bda9de3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Evacuation | `fdd94383-b573-439a-8e1c-925af887c5a6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ever After | `86be501c-6f05-4ce0-adb0-7adb135bfb93#card` | 420 → 60 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Everglades | `ef3b8b0c-cea7-4bae-934c-9c65fd64245d#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Evershrike | `9f148f1f-53c5-477d-ba91-a2415db65adf#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Evil Reawakened | `264f944b-386b-4378-9764-4a8bde246809#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Evolution Charm | `5f73fd4e-0d24-4daa-8d82-73c0cf7f801d#card` | 70 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Excava, the Risen Past | `77216636-f76d-4c5e-b1ac-ab1844753f05#card` | 20650 → 3150 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Excavation Elephant | `45fa04ee-3b5c-45b3-8c09-bda08cca3630#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Exclusion Mage | `fd28fe55-36cd-4242-8dd4-edb58fbb9895#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Exosuit Savior | `8b13ec4d-812a-48b0-8a64-6847a694a03a#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Experimental Overload | `7851e4c8-020c-4a3d-88ed-0c97609cdfed#card` | 48 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Explosive Getaway | `c844cc98-b8b9-447d-92ee-64f4a4906e7c#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Extraction Specialist | `4164034a-5e59-4e40-a150-2c1000b0bd0d#card` | 56 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Extus, Oriq Overlord | `0b299983-9f0f-404a-acd1-8f142572b1f1#face:0` | 56 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Eye of Nowhere | `27ffdf11-bb7f-40bf-94c4-6bfbc41668e5#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fable of the Mirror-Breaker | `c0957e5e-c71b-439c-931c-9f55d2f76ace#face:0` | 132 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fading Hope | `64883ef2-c536-42e2-b52f-cfd1984fcdfd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fae of Wishes | `c0abbed2-d213-47ef-8d6c-4a21efb9a55f#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Faerie Impostor | `0d78ba2c-4de0-47d9-b1b9-13dc9de8844b#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Failed Fording | `fe4bbe82-1d6b-4f3d-8cc0-f6df22d9e9fb#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Failure | `ed8184f8-f6a7-4f97-9f4e-bf72077af754#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Faith & Grief | `4f4358cb-59df-46d9-be27-69929f5a615c#face:1` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fake Your Own Death | `ad01df89-29fe-44c7-a133-91425f8ff09c#card` | 33 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Falkenrath Forebear | `940080ce-c504-48c8-a763-326fef907217#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fall of the Thran | `341803d9-fd29-4721-81bf-ae9b7f1c01d2#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fallen Ideal | `4b8183e6-4ff2-4254-9ee0-0ebbf8cef4a3#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| False Defeat | `8b119579-38e1-4509-9dcb-019d5a160ac2#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| False Demise | `50ffa9f0-fb3b-4de4-b463-6d7ee8b5e93a#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Familiar's Ruse | `3b6d773b-e2f7-4e74-9e77-f2e0d369a31e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fang, Roku's Companion | `6665bfe2-942c-4b3d-a976-776fe00c99f4#card` | 396 → 216 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fantastic Bounce | `71d09e00-e526-4899-beb3-0ee6d9b87f44#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fantastic Elasticity | `67af9477-82b2-459e-8c09-b9a93a1d1c94#card` | 24 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Far | `e4b1ef6a-6f05-472e-aaed-1c24dd6605c7#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fates' Reversal | `8cd88e87-6585-437a-bcad-b494fffc0544#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fateshaper Aspirant | `bb3996b6-b68d-4ea1-b3dd-1600f8da2f1c#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fear of Isolation | `0ee28549-5909-4372-8376-34010b7b6654#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fearsome Awakening | `b6e87f78-d4dd-44b2-bf70-92436ce583c9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Feasting Troll King | `6f0c6876-dcae-46e5-8c62-6929733c3525#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Feign Death | `f718e507-296b-4f22-842b-5fb91322069b#card` | 248 → 145 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Felidar Guardian | `33869ba6-13e5-4e48-8963-92f7965648fb#card` | 104 → 56 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ferocity of the Hunt | `747a1b36-eb4f-44e6-9e5a-bb1a0259a471#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Field of Reality | `4e16f537-8c36-4bd5-9ae6-0877ccfd983e#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fiery Mantle | `1b553ed4-12eb-4a53-8e1f-d47ec92b1d65#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fight On! | `4ab42daa-8726-4973-947d-1d6c32f0ba61#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Find | `212af747-2ac1-4d33-9456-8b3cc8988f5d#face:0` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Firemane Angel | `837e4452-3002-41e3-890b-3d54b469e7b9#card` | 12 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Firewing Phoenix | `326d9b5c-ac04-45b5-9a8c-a9be7d8cca1b#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| First Responder | `8f5aed57-d0f3-4fa8-826c-0d94fa143f08#card` | 40 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fix What's Broken | `7f8f46c0-889a-4c9a-9bfa-2b43db608d55#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flash Flood | `c2b5d847-f52a-43e3-a5d2-d53cae3d8936#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleetfoot Panther | `d99e8f94-4a8a-42a1-b5d0-2b08cacb8676#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleeting Aven | `7215b1d9-eb8b-41bc-a7e2-1930d9cd5365#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleeting Effigy | `d6d9b5d2-0ab6-4637-a468-5149b7c7d0cf#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleeting Image | `623f3efb-5431-42d4-98aa-9caac40e4e83#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleeting Spirit | `bc79eb3c-33e2-4820-a59c-92a6271fd725#card` | 64 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fleshless Gladiator | `8e33d6e5-30a2-48b4-826c-09aa75bbba27#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flicker of Fate | `d5b9d250-92f5-40d8-b962-4ae4adb466c8#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flickerform | `e5345c28-7046-4ff3-a5d6-eeb7a0fb230b#card` | 242 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flickering Hound | `162421d2-8761-437f-bed9-578b61c96f1f#card` | 222 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flickering Spirit | `ee679e57-4f0b-4ad4-9bfd-f331f4a5fba5#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flickerwisp | `b23a3d30-6b8e-4aad-890f-db0c3af43ace#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flock Impostor | `6522070b-7002-44fd-8deb-832fe67ef9a9#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flood of Recollection | `02393093-e970-40a0-848b-9cd758450cbb#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Floodbringer | `6c8bba8b-44de-4bbf-bf6a-3045e7313995#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Flooded Shoreline | `01fd889f-1839-44b9-b936-0e62c8f8a730#card` | 4 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Floodtide Serpent | `1031bc78-7eca-44aa-9ea8-47fa67b4ce62#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Floodwaters | `e89cce0e-4fd7-40fa-8a55-b07d72826956#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Floral Evoker | `4da17e9e-df3e-4c46-a0cf-baafbe67186f#card` | 14 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Font of Return | `b8805aa6-fa5f-4c70-ad90-8502611bef5a#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fool's Demise | `d719b761-a35d-4cec-b3fe-f0fc64278677#card` | 54 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Footsteps of the Goryo | `d0e8d9f0-86e9-46b7-ba6f-2976ff9148d6#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Force Away | `4770ec88-bc7b-457e-8f67-ae46e2cf4caa#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Forced Worship | `ea6b1e8e-b6e7-4fce-bfb7-7adf66b9f240#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Forcemage Advocate | `c8c5cd28-4f27-424d-92bc-e7d6f15a5f9f#card` | 5 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fortitude | `5017c07d-8ad5-4ee2-91d3-974811cdff75#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Forum Filibuster | `6a9cc11b-a223-46f0-bf58-c1e08ad1253d#card` | 128 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Foul Familiar | `90767f52-fa6d-4ef3-ae54-d8e3881971c1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Freyalise's Charm | `e0d638d2-2c5b-415c-95e7-74bf8d88794f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| From the Rubble | `76d07c99-8c6b-4c2c-a0f6-47541999015b#card` | 224 → 84 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fumble | `cbf68ddd-596f-4b2d-b93c-3b7976ba4d37#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Fungal Fortitude | `cf73d2e7-584f-41d0-8fc7-50d886dcf9fe#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Furious Forebear | `4fb1d33f-a87d-46cf-a73e-882c3dbcb26f#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Galecaster Colossus | `d8379d8f-df17-4473-938d-7ac198d17980#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Galepowder Mage | `b2457982-6c28-47e9-9b32-9db359eb0ac5#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Galestrike | `3006d992-2714-4264-a4c3-953b67dc7347#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Galian Beast | `f40e6bbf-1fed-4ec5-869d-18c2dd396d14#face:1` | 3 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gallia, Tragic Host | `58285b70-0d13-4723-b5a4-91fb6f9ccf03#card` | 141 → 57 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gallifrey Stands | `52680471-a407-418b-9e74-c32308322772#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gandalf, Shadow's Foe | `6760d7d1-098e-4131-8824-53e5427d047b#card` | 35280 → 21756 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gangrenous Goliath | `103c6037-18ff-44d8-84a1-fd6bdb3040c5#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Garenbrig Growth | `31af78dd-e962-4d9a-b696-f3048be03486#face:1` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gathering of Darkness | `85e1556d-c4da-4979-a5db-4ce3842c7797#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Geistwave | `14d5ed90-0aa7-48f0-9017-83912b9e89d1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| General Leo Cristophe | `088166b8-5b0b-4ba9-9b0d-358ae523f336#card` | 45 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Generous Revival | `f8601f5f-0f7c-4689-9d15-137f1e82c49c#card` | 96 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Genesis | `15afdd88-05fc-4335-8a55-c6ee702f3bef#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Geothermal Kami | `fbf8fdf5-0d7d-41a3-a6b9-27bb7a644c20#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Geralf's Masterpiece | `dacab848-89ca-46cc-b796-21fc2c218e78#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Get Out | `b2f3c27e-b646-4449-901a-8b809cdfa84e#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Geth's Summons | `c345be5e-243d-44c8-a7c7-280bdb235223#card` | 336 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghastly Remains | `0eafe734-19f2-492d-bfc2-d0c75ece45b2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghen, Arcanum Weaver | `6ac59552-9562-492c-aea4-4d64bad58e3e#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghitu Amplifier | `a965898b-9c68-46cb-82a5-a1e11397f68f#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghitu Chronicler | `ab8a00c2-dbb0-48c3-9d13-ad6f1e2f3a18#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghitu Firebreathing | `893cdd4f-0b22-4ddc-98a3-5d202564ffa5#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghost Council of Orzhova | `2ba16e4d-2de7-4906-801a-4ba2b90b5050#card` | 32 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghostly Flicker | `ad0070db-6454-41cb-861f-8f5b8fc2a3b8#card` | 72 → 40 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghostly Wings | `b07d8095-c5e2-4bf8-b055-af97ce5df4a4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghostway | `c8981af1-b0aa-4c19-8520-a7fb59018bc8#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghoulcaller's Chant | `b685fff3-f1b4-42d5-9b7b-e0885bcc92d2#card` | 35 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghoulish Impetus | `276eccd6-4a76-46e8-b604-94472e618115#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ghoulsteed | `dbc34afa-16bc-41f2-b74d-29bf2bfe24a9#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gift of Immortality | `2c11c72f-5b04-47d8-bdf8-8d9d197ebc1b#card` | 88 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gigapede | `9be31785-2975-424f-aada-25ee2a0bf6d8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gilded Assault Cart | `73da9297-4c7c-4388-820f-1189b5aa0b98#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gilraen, Dúnedain Protector | `27e83bae-0ffe-4cc7-a6c0-cbca25f2ba4e#card` | 6270 → 1860 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gix's Command | `9270eb03-4600-40cc-83a5-ea987d5d4978#card` | 570 → 114 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gixian Puppeteer | `9d6a9a37-a245-4503-baeb-9488553798ab#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glen Elendra Pranksters | `4b02df52-d6c0-4735-b2b8-d637cb41f0ba#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glimmerpoint Stag | `9620c8a2-63a8-4151-b4fd-a15db191a7b0#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glimpse the Core | `fa44fa48-d270-4f0f-b672-d63e1c1942ce#card` | 98 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glint Hawk | `5d24525f-dbac-434b-bda7-ff8af55245ff#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glissa's Retriever | `d099be2e-1879-4bb0-a27d-a386308387c0#card` | 160 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glissa, the Traitor | `cafacba9-195c-44ea-ba80-3e489274759c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glistening Oil | `645a4dee-a4e0-4850-a781-522aa718827f#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glitch Interpreter | `cf2c507a-41b4-457b-bbe8-edcdfcc681ac#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glitterfang | `226dca3d-9da3-4c3f-89d0-67264fd19c51#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Glowing Anemone | `52305f9d-2bfd-4b35-92b3-c08cdd758cde#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Goblin Engineer | `c1d6cce8-085f-42cb-8b0c-b6fbbf88b16a#card` | 176 → 44 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Golbez, Crystal Collector | `533d5bf9-0912-451f-a3f4-3842b0a75b1d#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Golden Guardian | `58afb897-4d57-4b53-a5c3-b532cb3d5180#face:0` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Golgari Guildmage | `46dad873-11f5-4269-8ec2-f126419fbd7d#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Golgari Rot Farm | `1b301478-b14f-4ef8-94e6-9647d582eabe#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gone | `86790a10-c564-44b7-a177-1e66ab9afe77#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gone Fishing | `dba1d231-6b03-44ad-ab8c-be0c8fe1b6db#face:1` | 88 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Goryo's Vengeance | `3ea3c681-f0ad-45a2-bc25-2813efd8d560#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Graceful Reprieve | `76cef1af-ff7e-4eab-9be6-81ba6049d222#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Graceful Restoration | `59bfeacc-f80b-49fc-8a35-f7e1ad905161#card` | 900 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grapple with the Past | `d14c633d-977a-4f78-85c2-88582b3c3670#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grasping Current | `a10a4bb2-c6fa-482b-9c8b-e64cbb8424ba#card` | 30 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grave Betrayal | `892c8fd1-9553-4ced-84ec-9fc35c314c76#card` | 8190 → 4650 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grave Exchange | `cc7d902e-c86e-4771-ba66-2f63e847cff1#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grave Sifter | `07a26388-773e-4614-a3de-f133c6694d59#card` | 12 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gravedig | `c77c3618-bc12-46a8-a4ef-073e359ff3f9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gravedigger | `1a2030cc-d7ee-4059-b2d7-fb95ea8e267b#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Graveshifter | `1eebc02a-9a64-49b0-8322-c5ce671457ce#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gravewaker | `7ceec1c7-c1f9-4d47-a203-6ee98c5d30d2#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Graveyard Shift | `da8a93c6-66e2-492d-8894-397ba9ec5a87#card` | 70 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grazilaxx, Illithid Scholar | `d22ff377-d282-4a28-9dce-96f25913dc96#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Greasefang, Okiba Boss | `0129ffe2-389f-4e05-8a04-96461bac39a5#card` | 20 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Greenwarden of Murasa | `2facb1b3-4522-4610-a0ab-29ac53ca7fcd#card` | 9 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Griffin Dreamfinder | `ca5d5cbf-fc4f-43dd-a2af-767f277dfef9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grim Harvest | `231ca538-b277-4c12-979f-57640a0e87f0#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grim Reaper, Lethal Legionnaire | `edf429a8-6a4d-4d2a-b0ca-c0c72a0698a3#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grist, Voracious Larva | `196f8f13-d4df-4740-8746-8fed3bc71840#face:0` | 392 → 208 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Grixis Charm | `387ae7be-1cb5-4ae8-bb63-edf4941945a9#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Groffskithur | `32a8ca5f-0170-4f5d-97cd-63e2b74fd653#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Groundskeeper | `f2c19fba-9b06-461f-bb5e-e827ba41204d#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gruul Turf | `657243dd-e479-4f4b-99d2-09b55d833a35#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Guardian of Ghirapur | `2553ab57-ceb6-4654-a59d-bc61735da908#card` | 64 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Guildless Commons | `ee723c7c-ec9f-4ffb-8f36-cd7637eb1fae#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gust of Wind | `7cfc14ce-9940-4c61-9daa-2c6dbf1a80ad#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Gutterbones | `76ae3779-c821-4cd3-8d0f-8c42d1206ce4#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hakim, Loreweaver | `43e99cba-a6e3-4abc-a92b-aaf5839c72d2#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hakka, Whispering Raven | `0a69c3dd-52a3-4ef6-b53f-b0780e0dc5f8#face:1` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hallowed Ground | `ee8fd844-4551-4e44-9740-00d2e76eb337#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hallowed Respite | `2d72454f-3d87-4acc-9f46-b8c579f44af8#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hammer of Bogardan | `3089bdd7-87e6-4d0e-ad98-c32e6f861c30#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hana Kami | `7ac433a7-c107-46de-b36b-d7419c9ec565#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hanabi Blast | `8e47b355-98fe-452a-a81e-b901c1f2cb6f#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hanna, Ship's Navigator | `3b008c71-e2a0-4f56-83d3-655c5dd58e0e#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Harnessed Snubhorn | `e1dd9dca-cb5f-46b4-b16d-a61ae651d653#card` | 36 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Haru-Onna | `fead5048-3ce5-46d9-9c04-0a52f1554030#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Harvest Hand | `ffd3c928-af42-4d69-8d22-a4c72f93b1c7#face:0` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Harvest Wurm | `cd539212-ef7b-4f78-8d62-c42289aedc51#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Haunt of the Dead Marshes | `0b683611-e46f-4105-b336-014d1556c8de#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Haunted Dead | `03259490-a789-4f96-8c5b-b410425bdd02#card` | 32 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Haunting Voyage | `334c284e-7bc3-4c18-9f8d-4d846f80008e#card` | 432 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Haven of the Spirit Dragon | `acc9c16a-5e72-43bd-87e1-56a16aa892f5#card` | 33 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Havengul Skaab | `8fda6c20-04dc-4b60-9313-896a032ee5cb#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hazel's Nocturne | `2a74a055-ff7f-434b-8dfb-d9fb72579cee#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Healing Technique | `1b99c32d-1d02-410e-84bf-9db2ca05f576#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Heidar, Rimewind Master | `58ac4cce-506e-4d6e-a19c-0e875fbfcd81#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hell's Caretaker | `2f0d797e-d897-453d-92b6-a90e1a548dc5#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hellcat, Undying Vigilante | `4c27ba61-6812-4dd7-8efc-65435a8df7b5#card` | 138 → 76 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Helping Hand | `170a5dbc-6a43-4715-9501-178b1eb7b08c#card` | 11 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Helvault | `fa3d8a6f-dc5b-4890-a806-f785a41660c7#card` | 54 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hero of the Dunes | `eb6f936e-8dba-49da-84fc-de240b11b8e4#card` | 46 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Heroic Return | `4b7a8c05-15cd-452e-b9fc-83800e81793d#card` | 360 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hibernation | `633b541b-e37f-451a-be73-3398e61e0f75#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hibernation Sliver | `edb8a6d6-9e25-4048-a842-e854eece42c2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hidetsugu Consumes All | `5b8be856-c08d-4356-925d-fce843565189#face:0` | 88 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hikari, Twilight Guardian | `98a89d5c-871c-4da6-a502-9fdb7bdfccff#card` | 108 → 60 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Honest Rutstein | `439bf6d1-bd65-474b-a70c-7fdd446de4bb#card` | 40 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hoodwink | `a2fe10b8-857e-4881-85ef-24a0f96a3d79#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hormagaunt Horde | `cc965557-f69e-4689-8791-d3618e4ffbfa#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Horned Kavu | `14817050-9f2d-453f-b36b-e6429fa29165#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hotshot Investigators | `288fcf5c-f5aa-4f62-9dd8-ba295df48f7a#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hoverguard Sweepers | `853ef032-a2f7-4e6b-a210-03e96855d062#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hubris | `95fbd573-c04c-408a-a8c3-f7e1ad5ddac0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hurkyl's Recall | `ed1e5d24-c8a8-48fe-a88f-1003ad432477#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Hypervolt Grasp | `269a2b53-52d7-4032-ba62-7b7742e7a159#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Icebreaker Kraken | `01590ff3-34fa-47aa-b5da-f0b8c1b031c6#card` | 384 → 96 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Iceridge Serpent | `53f07529-c212-43e4-b5b6-086efeadda39#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Icewind Stalwart | `9ad12c75-e97d-404a-bfa9-26ff1d0bb506#card` | 111 → 60 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ichorid | `5261ba70-b641-461d-9d9f-fce17764ad67#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ignorant Bliss | `9cf12838-64fe-49c9-9181-e0f6118c0bf6#card` | 30 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ill-Gotten Gains | `c88eff74-def5-42ea-873c-b3b479f6fe18#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Illusionist's Stratagem | `1225bee2-c829-4a93-9f75-3495277c23bc#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Immortal Phoenix | `1d8bd391-b133-4c8b-95ad-7f7ab99137c7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Immortal Servitude | `c946be2c-1e8f-46bc-ab75-861926d4aa63#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Imperial Recovery Unit | `18ad0dd1-2d0e-46b4-8ab1-ece07180267f#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Iname, Life Aspect | `cb806239-32bf-4850-be65-239444c347a5#card` | 18 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Incarnation Technique | `0ab9539d-3461-4936-95cd-4c229308c4a4#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Indoctrination Attendant | `9587bdb3-fb6a-47c0-898d-404bf60fcb6b#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Infernal Vessel | `0e13c283-f462-441f-a8a5-0c75a59ff1d9#card` | 840 → 460 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Infuse with Vitality | `6799217a-030c-4c1b-9de3-99725c6ef4cf#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ingenuity Engine | `d1da2c33-2be4-4065-8e5c-613a069d2013#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Inspiration from Beyond | `be87e410-4302-43ee-b880-56eedf2b3648#card` | 24 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Into Thin Air | `06df923f-916e-49f4-960a-7b3fb108c704#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Into the Roil | `c2898bbd-82a4-4d26-b6ee-169b0ebe71b4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Into the Void | `4d5a970b-dd36-4871-a2aa-36ba21917cdf#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Inundate | `0390a3dc-386c-402f-a89c-b94867bfb0a2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Invasion of Xerex | `8b5add91-2e82-4430-bfc5-f4a6366dbbdb#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Invasive Species | `9c4c478c-6f0c-430a-91eb-4125a73d03a7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Inventive Iteration | `e711a213-2a6b-4b34-a788-95b6c0b7ba48#face:0` | 220 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ironclad Slayer | `964e20e2-2947-4d8a-a0a2-b8b5fde27ff8#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| It Doesn't Add Up | `dcfc8c59-d032-414d-9da6-125ebcb1366c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ivory Gargoyle | `e8cc9f83-4e4c-4820-9a7a-f82f728e9767#card` | 43 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Izzet Boilerworks | `1cb9d94a-3039-4f2e-8fcc-6996f9a45f74#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Izzet Chronarch | `1da438f3-db1c-4713-a60c-e078f31d809c#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jace's Ruse | `59cbed39-a07e-491e-a95a-adafe19eb652#card` | 64 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jace, Vryn's Prodigy | `594f6881-c059-46f8-aa4e-7151d502de73#face:0` | 784 → 416 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jackalope Herd | `bcc8ce61-a881-42be-ab2c-64351d17f9f8#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jarad, Golgari Lich Lord | `87e65e36-9483-49fe-b644-2caca092107f#card` | 42 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jecht, Reluctant Guardian | `b6cebf8d-37e1-42de-b3f1-1c174ab17ac8#face:0` | 204 → 108 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jeskai Barricade | `7a2fa0a6-1324-4714-8896-6bb97640578a#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jeskai Revelation | `61b6b1d1-4350-41c1-ac47-835b2831f24a#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jilt | `382bc554-d170-4c19-82dd-829f254e855e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Journey to Eternity | `7d6ccd0b-df16-40b2-930b-bcde0b6ef73f#face:0` | 255 → 79 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Journey to the Oracle | `ca79cd3f-13ba-4d11-b885-48a504ae69aa#face:1` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jugan Defends the Temple | `e70ac420-8540-4a69-820f-ae3e0274ca1a#face:0` | 132 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jungle Basin | `f922f90a-b1a2-4630-9266-40726ca89f74#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Jungle Creeper | `acc091c0-3891-41b9-b5f6-7537fe9493fe#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Junk Diver | `08c595da-9305-42a9-b72f-5ccc546edc01#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Just the Wind | `761ebd6a-8f53-46fa-b3cd-16616b773007#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Justiciar's Portal | `0214805d-d207-42da-a25f-2a8e1990904e#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kaijin of the Vanishing Touch | `cd8bab2c-b04f-43a2-9786-df5f7411bf16#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kami of Industry | `b308fa9a-3f69-4e59-9d4b-8007d9a2b1f1#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kami of Restless Shadows | `7e770c08-910e-4394-88c5-79c874750b6a#card` | 40 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kami of Transience | `7baffe00-78a4-4675-9c74-29eb3882bd01#card` | 30 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kami of Twisted Reflection | `f23bc7a3-325b-4b37-a4d8-19f3b260ca3d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Karakas | `59119143-c0fa-49dd-adf0-e2fd3029c48b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kardur's Vicious Return | `6ca1d5ca-cc97-4783-9c0a-f47097f35ba8#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Karmic Guide | `8c31fec9-e4b3-4761-990e-7be38eb05604#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Karoo | `d4e875d9-2245-470d-aa2f-1dfe66ce2d15#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kazandu Stomper | `121f6c96-2911-4ee3-b14e-d3b143eb14c3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kederekt Leviathan | `edf459c0-53e4-47e8-8098-105656f9b49c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Keeper of the Nine Gales | `39c4357b-6093-486b-8131-041a3f89af75#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kemuri-Onna | `54db986e-cf8a-4d2c-8ce9-13da11b11464#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Keymaster Rogue | `b8c0c102-f0ef-4ab6-9b1a-ee82ee159c54#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Khalni Gem | `b0a96e47-91da-43ee-93dd-f866721029c6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Killian's Confidence | `c17f0e65-e5a3-4113-b782-0b192ab52046#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kiri-Onna | `e09661d4-858b-43a8-b6a4-ba76f7a69777#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kirri, Talented Sprout | `f03bc648-08e4-4ecd-b339-ff72b00367c2#card` | 216 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kishla Trawlers | `09277185-fccd-4747-aa79-f569c581022a#card` | 72 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Knights' Charge | `b4014b7c-aa1d-4feb-ab7a-4921ad150f9a#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kogla, the Titan Ape | `8b21062e-97b0-4967-96df-30e8309f4fba#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kolaghan's Command | `45f1e957-09f0-4d46-8e32-238f26060a87#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kor Skyfisher | `054a0193-861e-4644-ab0a-8aa33407abc8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Koya, Death from Above | `e7cd87d4-028c-49d9-9416-69f57e7dfec5#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Krasis Incubation | `66e2e816-5197-4151-a6c0-352c34c2eec7#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kraul Swarm | `ac97d8fc-15fa-4a60-96d3-4c09deafa83a#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kraven's Last Hunt | `e74efcf2-d77f-4fb5-9027-77e462ec829e#card` | 640 → 128 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kroxa and Kunoros | `56679f06-982b-4f33-9e48-ecc64c802393#card` | 25 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kuldotha Phoenix | `884260cf-b3f3-43a4-82b6-4f6a0b19cc75#card` | 24 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Kykar, Zephyr Awakener | `0156c94b-e64c-49de-a46c-a203c5e05e98#card` | 448 → 252 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lamplight Phoenix | `c6f7efad-381c-45ba-9976-159b7e78f7ed#card` | 12 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lantern Spirit | `cf590bb7-2ea4-446a-9ca2-1e0706c2c41e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Late to Dinner | `31bf199b-dfb1-428e-96a5-eb25104e2b43#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Launch | `9e3b373f-05f5-44df-9568-bd1f8039aa54#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lava Zombie | `2232c38e-49eb-4823-a325-2342a952db72#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leashling | `8460abd9-ab5d-4a7e-8132-185b4a310190#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leave | `dff5d16f-3a41-4873-8188-076e77f18c75#face:0` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leave in the Dust | `7bb843b6-c55b-4796-b1a7-2fff7b05c276#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Legion's Initiative | `7b23bceb-1b82-4c9c-b7c1-8e8ef4643fb0#card` | 276 → 152 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leonin Squire | `fa19c9d7-fe93-4e93-a5ef-9af9c6cd1a5e#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leshrac's Sigil | `98388eba-31ab-4302-be9e-ea67ea2b8c1a#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lethal Protection | `49a7c053-81e9-47b7-af5c-c62cd0389ed4#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Leyline Phantom | `af642b8d-a085-4a07-8a1d-ca385d4700f7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Liberate | `f17e6c85-4813-4b0f-a0e9-d295822e9bb5#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lich-Knights' Conquest | `b0e533dd-baf2-482a-b9e1-872eb5e47439#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lie in Wait | `4dc0831a-0eb1-4f7d-9591-6e670e180116#card` | 25 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Life from the Loam | `ac8fba34-512c-4f24-999a-ab72f1ce4acb#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Life of Toshiro Umezawa | `0a708566-2994-47b3-8d89-0547cc115e7c#face:0` | 88 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lifeline | `f475b636-eb7f-4dd7-bc87-a1af07064ffe#card` | 80 → 44 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Light the Way | `83917116-d2d8-4701-a79f-af17486706ae#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lim-Dûl the Necromancer | `820052b3-d217-4b30-acc4-fa5af7465e9e#card` | 45 → 25 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Linessa, Zephyr Mage | `64337db7-a66b-42c5-89fd-c9dfc203c0dd#card` | 40 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Live or Die | `dc367b23-d1f4-4c5c-b300-a44a726e3f74#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Living Lightning | `1f4738db-be2e-4d02-ad0c-1b74dd6bbfdc#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Living Tsunami | `7260657a-c1cd-4081-8ab5-46d604dccca8#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Living Twister | `206f7eaf-1e0b-48ea-9c40-38902e109e4f#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Llawan, Cephalid Empress | `55f9a9a2-2842-4555-8b54-03aebc8d6fb8#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Long River Lurker | `c65c4d01-29da-47e6-993c-a265cf1f198c#card` | 220 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Long Road Home | `a8f53018-c496-41f3-bc3f-3c4703b9e4e1#card` | 138 → 76 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Loran, Disciple of History | `9d1f232c-e904-46f6-a4b7-24ca2ede711c#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lord of the Undead | `7714af0e-41d9-4609-967f-27233b46055f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lost Monarch of Ifnir | `626a979f-bed7-4cb1-9ab7-c3f2bbe37781#card` | 600 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lost in the Mist | `98014ad2-7a2b-45f5-808c-edd4d0b2c597#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lost in the Spirit World | `a37adc18-ea11-4972-8d04-936cf3fe2613#card` | 200 → 100 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lotus-Eye Mystics | `efb11270-4b5a-4c52-862f-1d24c6b95d73#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Loyal Cathar | `bf28a2ed-cad7-48a7-9dec-06e43af73e9c#face:0` | 31 → 17 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Loyal Gryff | `f5c07218-c8bf-4990-a366-f3c065d0b30c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lumengrid Drake | `0700d889-d235-49fe-8b92-698d486b2539#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Luminous Broodmoth | `28c7c816-07e7-42fb-923c-bf149ba28b38#card` | 276 → 152 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lychguard | `e82daa7b-d9fd-48c4-b5b2-f3b1f6ef1a24#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Lynde, Cheerful Tormentor | `dc386f6c-bd4a-462a-8d42-0e7a552a4f17#card` | 30 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Macabre Waltz | `cf75dbd8-b65a-48d9-b96b-4afb43f336d1#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Machine Over Matter | `37069c66-c17a-40ac-80f4-14ef49d18008#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Magma Phoenix | `9c6edb99-2625-4d80-a2e8-1b6039c0a960#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Magmatic Sprinter | `85d025d9-158d-4bf1-b3b2-361f3fe59663#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Magnetic Snuffler | `8f85088c-256d-4925-a4dd-aafcc2632282#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Malakir Rebirth | `a731e87b-8d99-4b64-8ee3-8e540d652366#face:0` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Malevolent Awakening | `522fc178-d11a-4919-83b0-e58453e3a160#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Man-o'-War | `67a3541c-8408-40c8-b44f-90035b860f57#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mana Bloom | `7c699180-d18e-4599-acd1-27b951d82136#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mana Breach | `221d0305-94db-4791-9dc7-89ee96c7fcd6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mantle of the Ancients | `7424560f-557f-4bc9-a3e7-eb890c73aaa3#card` | 96 → 42 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Marang River Regent | `099b8d2c-b482-48dd-b6bc-dedf546d66c5#face:0` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| March of the Drowned | `e9e55768-ed62-4998-9922-dbcdeab12aee#card` | 35 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| March of the Returned | `53a40369-0e62-4ad2-a6bd-65a5ff0da65e#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Marchesa, the Black Rose | `17a59d3d-9e01-48cd-bb4a-3eaaa077751c#card` | 210 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mark of Eviction | `d9193641-a7f4-40d3-ac62-15015acece4e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mark of Fury | `1c350891-8810-4c14-b8ea-84486864637b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Marsh Crocodile | `1d6f3934-d8f1-4758-b87a-8df8d7dc492c#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Marwyn, the Preserver | `eb03f6c1-c94f-4f09-b2e9-a55ce5533653#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Masked Admirers | `ea97fc76-3a32-414c-aa12-87be1a0eb9f3#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Master Skald | `34bccfc0-453d-4d1a-8046-70f9c3065e88#card` | 72 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Master Transmuter | `da46786e-28df-4638-ab3d-121011d2f150#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Master of Death | `3a16bdda-8499-47e4-b870-b9576f4c1a83#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mausoleum Turnkey | `d4a61685-7149-4d14-bc17-85b060945687#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Maze's End | `49479778-c4c0-43ba-a7b7-45f00d067462#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Meanders Guide | `8229954b-e1ef-4503-8443-2356e2a87663#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Meloku the Clouded Mirror | `3b96b8c3-b9c8-4712-a3b1-243a67cc013a#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Memorial to Folly | `2bc38f14-0314-4351-8138-e2b8bf041404#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Meneldor, Swift Savior | `ef9a5bd1-6ce2-4570-9d42-89d997e2fc6f#card` | 186 → 102 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Merchant of Many Hats | `23820b45-2f3a-4b6e-99ea-d84caccb7e37#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Metalhead | `7df45a2a-75bc-4343-8dad-3a642efd13ee#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Metallurgic Summonings | `7d5149c6-0e7e-438c-a554-5235c85e2426#card` | 40 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Metalwork Colossus | `64ffe529-ba53-48ec-8af1-b046fbd1f0cc#card` | 21 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Metamorphosis Fanatic | `017aa9b3-a8ea-4588-9c50-e914a7d8e4ee#card` | 98 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Metathran Aerostat | `093daf01-d83b-4d49-a6b3-ec9a799b9582#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Michelangelo, On the Scene | `e0e3feb8-2add-41c9-839c-587af6ca8f93#card` | 550 → 275 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Michiko's Reign of Truth | `94794f7c-62e7-43c8-ba7f-ac5c0bc9a712#face:0` | 132 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Midnight Recovery | `7904025c-7120-4362-abe9-a9f5fb64447f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Midnight Scavengers | `1c29ae09-cc93-4a2a-b432-3c78ae01c1ed#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mina and Denn, Wildborn | `e091718c-0c26-46d9-b694-f95effbac3e5#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mine Excavation | `9bd5dc7a-11b0-48eb-bf52-2956dd3989ab#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Minion's Return | `6cbe76c6-2b64-4389-b5f9-0b221cd92312#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Miraculous Recovery | `cea73a4b-e10a-4517-a92b-24a08321009f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mirkwood Elk | `5b29c26f-c0f0-4703-83db-3a249ebc5792#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mirkwood Nurturer | `66630640-9605-44da-afdf-eb9d458d056f#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mirozel | `c35111ce-4325-4547-b954-08275a268060#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mischievous Pup | `e75b187d-0e3b-4d94-a940-8b7e4e7ed1ca#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Misery Charm | `46aa8dca-2a7d-4bff-9323-5f6933ca95f2#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mist Raven | `12654457-3aae-4046-a976-8e41c3f78927#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mistblade Shinobi | `16856e22-6f9e-4f8e-b4ee-622a7c2ee453#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mistbreath Elder | `bf9221f5-a4b2-4074-901c-0fa3c2bda80c#card` | 4 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mistmeadow Witch | `38e274e2-bd04-48de-a1df-44f0ee987ba8#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mistmoon Griffin | `7fe9d30f-c778-4769-a3ae-3cb48cd23e07#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mnemonic Wall | `f2933616-211b-4d3f-8064-2be61c26079a#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Molderhulk | `a6fa840a-817c-4e84-9dd3-d28a011f2214#card` | 120 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Molten Firebird | `e91ed165-b86f-45e8-95b4-bce79da4ab5a#card` | 43 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Molting Skin | `8262dbb3-7abb-4de8-9a1d-edee71906a10#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Momentary Blink | `a3ec6b5d-08ec-4ae0-b1db-c4b87a1849c7#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Monk Idealist | `1f47ff5a-dd93-46de-a1ca-b412631218bf#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Moonbow Illusionist | `411137a5-7054-42bc-a731-88a15e5e6b9d#card` | 30 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Moonlit Scavengers | `8403ea46-e6ac-458c-9c17-4c8a81f21659#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Moonsnare Specialist | `ba858b68-ae93-4e1c-8a3a-3e4ac97282e8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Morbid Plunder | `1bb7009b-aeb0-4545-be4c-c8c5d723bcf5#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Morcant's Loyalist | `b2c30683-cc7f-4341-add1-b08cb9b93eeb#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Morgue Theft | `0b8a508b-20c0-4a87-a73a-08af72c39a0a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Moriok Scavenger | `fdccdd9f-6bf9-4339-ad7a-e9dd8f771b22#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Morningtide's Light | `a25bcf65-4417-45e9-a4b1-94e0ea78af63#card` | 28 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mortus Strider | `15af3bd0-022b-472e-b21e-29b925b7692b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Moseo, Vein's New Dean | `c2e2f21a-0338-44d7-96a5-5a277f0ec419#card` | 600 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mourning | `20602d11-c0ae-47ce-a7fd-998b12480e18#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mtenda Griffin | `273f0353-4936-491b-82da-6d4d7f3259a6#card` | 40 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mudflat Village | `aeeab1df-0b8b-4bc4-a5f9-aac413449bec#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Multani, Yavimaya's Avatar | `4b8bf64b-4800-45ff-81c6-2857f34999b5#card` | 84 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Murasa Rootgrazer | `ca7964a1-b4ce-412b-bcae-e0dfbb95389a#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Myr Reservoir | `82b327c6-5cd9-4bee-bef4-860522bded9c#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Myr Retriever | `d07d3be3-f69d-4484-8467-cffd43871788#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Myr Servitor | `b6a50858-a013-4703-a9d7-5b69ef9bd9ae#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mystic Retrieval | `b7cfbfa1-dd38-47ff-9138-fa6b247afc11#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Mystifying Maze | `58bd67a8-1833-4827-aa33-1c141568f481#card` | 36 → 22 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Narrow Escape | `c96803dc-b69b-4fc3-a39b-1aad6d800e86#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Narset's Reversal | `d55f6c70-321f-4fb4-bd33-0850ae1a7c36#card` | 40 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Naya Charm | `20cc0f3e-de69-4e5f-88b3-ae7e6c6b5996#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necrogen Communion | `ff58459a-0015-4012-86c0-580da4e02676#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necromancer's Magemark | `dffa7c06-096a-47f8-9645-1ba7306ba5b0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necromantic Selection | `b48a149c-aead-43b2-a99e-c964d6d28aa0#card` | 1080 → 600 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necromantic Thirst | `494895db-3da1-4a19-b7a5-ea47817f4e4f#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necrosavant | `6732a587-e3cc-4c30-9267-f8949dd8724a#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necroskitter | `536f7c92-2f1e-49e7-98de-47382a9488f1#card` | 55 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Necrotic Plague | `9d637892-26d7-46f1-bd7d-89bda523f649#card` | 5 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nephalia Smuggler | `f96a2aa6-3259-4bb6-92cd-4d17215d04d0#card` | 36 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nether Traitor | `6f16c3ac-a9b8-47e6-b18b-ae37c74d44a0#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Neurok Prodigy | `910860ce-d617-48c1-b44d-b1d95cdd006f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Neurok Replica | `de8cdbd4-d8f7-4ce8-a69b-6d2020beff63#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Neva, Stalked by Nightmares | `ba5895e1-f12b-43c7-b47c-376588d9d2d2#card` | 108 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nezahal, Primal Tide | `c45e96cb-2539-4e00-9fff-782a182c2f4b#card` | 31 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Niambi, Esteemed Speaker | `91c9dbd8-a8d8-4a4e-8408-3ad92e974d11#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nightscape Master | `e5f9bd6b-0aa6-4f61-b101-ec194af6d632#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nikko-Onna | `f5cf3f74-e497-46d0-acfb-3b57ee27a314#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Niko, Light of Hope | `f153e369-1d5b-4d8b-a2f6-d93b849cabb5#card` | 96 → 54 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nim Devourer | `89075fbe-cd8a-4054-abab-fcaa2aedddfb#card` | 24 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nimraiser Paladin | `c9daff24-1af8-4133-bff1-e0e4d1042db4#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nissa, Vastwood Seer | `35754a21-9fba-4370-a254-292918a777ba#face:0` | 24640 → 13024 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| No One Left Behind | `afe8fd01-f0c7-46a5-9e9c-e8b9599cd19a#card` | 75 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nobody | `eddab60f-2eac-4f20-934c-3d9307f885c3#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Noggle Bridgebreaker | `d7e127e4-56fd-4116-9c1a-421beab58e5c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Norin the Wary | `b8b51a29-a2be-4b97-a73c-ea512d3d5622#card` | 32 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Northampton Farm | `e05f1a43-16ce-4e88-b0ad-2202efb25516#card` | 350 → 100 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nostalgic Dreams | `fabbc474-682c-4565-861c-d41de05abb8f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nucklavee | `09c2d504-f2e1-4799-b948-c7b9502d5088#card` | 49 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nullmage Advocate | `558568a6-5e22-482f-9096-e76682965c21#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nullpriest of Oblivion | `c1111228-ef88-4b64-91e0-66ee32a1f5e4#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nyla, Shirshu Sleuth | `3492cee8-b854-449a-ada4-a4e8a4333d87#card` | 72 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Nyx Weaver | `6f8bf968-0571-4582-8c04-9791e44d5df0#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| O-Kagachi Made Manifest | `4d23b22b-31a5-4eec-b894-2dd5a7138c5a#face:1` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oathkeeper, Takeno's Daisho | `7d58b478-a64b-46e9-b62a-08c6c1ca7c13#card` | 63 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Obelisk of Undoing | `299ea6dd-79eb-4c25-a05d-ff6fcad663cf#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oboro Breezecaller | `257ee9a0-5ee0-49c4-8619-e75e1134d7ac#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oboro Envoy | `be70c6e8-6f9f-49fb-ab40-c6ce0ec2077c#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oboro, Palace in the Clouds | `645fb11b-d684-4bec-8532-8fa97e8f7b28#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Obscura Interceptor | `b0870f91-c544-446a-8f7b-a8d03fdb0a2a#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Obsessive Stitcher | `e8210145-a6ac-4f84-a74c-a5799c5e9434#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Obzedat, Ghost Council | `6c3df703-e96a-4ee4-b449-b8e1e3460bd3#card` | 54 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Odunos River Trawler | `0282b5c0-e792-4099-a3fa-329eda11fb44#card` | 49 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ojer Pakpatiq, Deepest Epoch | `34ef174e-1b3d-43d5-9f72-3d35befbdd7f#face:0` | 1908 → 996 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oji, the Exquisite Blade | `7c4c556c-b5e3-493a-a7b0-38c55b7809ac#card` | 888 → 480 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ojutai Exemplars | `4405db13-7b36-4dc2-9a5d-1d31d568abf5#card` | 144 → 88 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ojutai's Command | `c1fafe93-8fd2-44c1-a4a0-5c9ecee550cc#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Okiba Reckoner Raid | `54257132-c67a-4c19-bada-08e7025c2929#face:0` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Okiba Salvage | `142409ce-1187-4554-bc07-6432c4d22dbc#card` | 36 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Old One Eye | `86ef2ceb-b7e6-4451-92ec-8528ea191d3a#card` | 24 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Olivia, Crimson Bride | `8922a91e-3d15-4351-8d77-e0d6bc4de82e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oltec Archaeologists | `b19c4bbf-258f-4436-8ef9-fc905c283776#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Omen of Fire | `5935e9b3-b9b9-413c-ba17-103f5792131a#card` | 54 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Omen of the Dead | `0aa9ee99-69a8-452b-a0d5-001384bdff3c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Onakke Oathkeeper | `080df9eb-c26b-494a-96b4-d7e47e501866#card` | 330 → 66 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| One Last Job | `55cba59c-932c-4295-87dc-9179fbdfbd53#card` | 1440 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oni of Wild Places | `f551093d-40b7-43ac-8023-4cfbb6c646da#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Open the Vaults | `1e9c473e-bd65-4e1f-b2ab-cac58dc581c9#card` | 18 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Optimus Prime, Hero | `199c4a1a-2082-417c-a174-f479d7334d15#face:0` | 30 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Order of Whiteclay | `16c46eaf-7a31-4957-ab7c-ff07fbbd9ddc#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Orzhov Basilica | `aa00ae0b-7c0f-427e-8102-ce0e2a6af5df#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Orzhov Charm | `26f8626a-ce64-450c-bda5-833df2fbb4fb#card` | 16 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Otawara, Soaring City | `e9b6a394-691c-425a-9307-76d8edc7375e#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Otherworldly Journey | `4a080242-412d-4968-ad57-2667dab977f5#card` | 138 → 76 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Out of the Way | `9300874a-32a9-4869-b3e1-1b996403df4e#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ovalchase Daredevil | `a33961bb-fff0-4f4a-aa35-f904dbf2b075#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Overlord of the Balemurk | `652e87af-7cf0-407f-9fb7-1a630fb8dd47#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Oversold Cemetery | `ed4cd3a1-688b-4c05-948d-39d3336e00c0#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ovinomancer | `98026d30-a29f-4590-968f-b1b36926a275#card` | 24 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pain 101 | `e9975888-c75f-471e-a70d-17449e92226d#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Paleoloth | `627de3d8-ad5e-4f9b-b83c-476cc98fea9b#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Palinchron | `0023888e-7bec-43e0-8dee-d1a4eb94b372#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pangosaur | `0b350f23-9150-40c7-8ac7-86adfdb8ba52#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Part the Veil | `c8ca2523-0511-4415-b85e-7834d3cb1d0c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Patriarch's Bidding | `25fc3bc2-d852-4f52-9adb-c7e35c06f3af#card` | 43 → 17 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pearl Lake Ancient | `b7ae125c-955b-4cfd-9aeb-e6572c0d4d3d#card` | 4 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Peel from Reality | `65dc05d0-f885-4fe5-9a23-d6b75185c179#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Peerless Ropemaster | `d7aaffed-a143-49d6-a516-b9bc8b503aee#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Perigee Beckoner | `e0a35a44-e56d-434e-8043-43d5031b2e84#card` | 36 → 22 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Perilous Voyage | `0ab090db-f8fc-42ff-95db-e9370796db97#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Persist | `367d4cf2-270f-4236-af10-7d5e8ea8c9fe#card` | 63 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Persistent Nightmare | `99d42ae8-8cee-449b-812b-98a81a593077#face:1` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Persistent Specimen | `5f4ad578-f2c4-47f6-8080-8fbc17383f6f#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Personify | `e1708ad9-35d4-4243-92ca-ddf9d129aa32#card` | 154 → 84 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Petals of Insight | `1eb88d87-e024-47fe-bd63-0eac69c106f6#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Petrahydrox | `545c516f-85a4-4a59-8cfa-03da27121a57#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Petrified Field | `c4bc5bc4-e589-42c5-91fa-2ebc96448e85#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Petty Theft | `c7b044c3-3cfa-407e-bf20-2875e8e04b7b#face:1` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phantasmagorian | `248e0dfa-0bcb-402c-badf-ca59f02792f7#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phantom Whelp | `9b020aeb-7b27-43f2-9767-d93dae3d102a#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phantom Wings | `9da74f0f-143d-4db2-81eb-c7873744a2d0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pharika's Mender | `00771a33-7443-43f9-a61e-1737b62b1f81#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phelddagrif | `d0e99535-0ea0-4aa5-99f5-6c5255b27c38#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phelia, Exuberant Shepherd | `5d86a59a-ba1f-45f7-b829-dca6f9f3e624#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phoenix Down | `8cc7e418-cc5e-4aef-9757-874223fccd74#card` | 11 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Photon, Lady of Light | `2d0aa801-1227-4226-8c27-3bdd441857aa#card` | 222 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phyrexian Delver | `a13cbac0-4c76-4970-b61e-5f4e020ee95c#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phyrexian Missionary | `be04f54f-59fa-4f0a-b8b5-790d568932b6#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phyrexian Reclamation | `647ca69e-cc01-4b2b-b376-bee2a98331e8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Phytotitan | `2541e0cd-2cc1-4e47-94e3-030c1cb1a6bd#card` | 49 → 29 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pilgrim of the Ages | `f67084bb-07f8-420a-ad6e-9ab9842d3ee6#card` | 66 → 22 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pillardrop Rescuer | `5cf9002d-6c04-420b-8c3b-0a17f674bb09#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pinnacle Monk | `f3d48efa-910a-4872-a5b1-a353c5dbce99#face:0` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pit Keeper | `e1c5bfae-07cb-4194-8a86-201503e17085#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Plague Reaver | `eef50ef8-958b-4f5c-b261-16926d9fe672#card` | 32 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Planar Birth | `7a9d13b3-3823-463a-acd9-8b7a9d5e121f#card` | 38 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Planar Guide | `75701981-b9fc-471f-a97f-e897bb3dfb77#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Planar Incision | `a959a91a-3e1d-4bd4-9e14-5ee977540146#card` | 276 → 152 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Planar Overlay | `a7be1624-04cc-4ed5-8b05-63b633893dee#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Port of Karfell | `f500a8a4-6135-448c-9116-bd2695a72229#card` | 14 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Portal of Sanctuary | `c30491b5-08fe-4694-9ac8-53be7b4b92ce#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Portcullis | `6230c179-0263-4fa4-b0ff-f2cb621de147#card` | 48 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Possessed Skaab | `a42b567d-6bfc-49e7-8f91-917e2bb3046c#card` | 36 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Postmortem Lunge | `aacd1aae-1bf0-43e4-99f4-c6d738af812c#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Postmortem Professor | `0859d16e-2a1f-4b48-8687-99af8a8bdb38#card` | 42 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pothole Mole | `8a8bc537-b69f-441d-a10e-d7f17f2f75bb#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Praetor's Counsel | `83d8eda7-b8d2-479a-b1ec-de56833a3182#card` | 27 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pre-War Formalwear | `c5782f70-05c6-4df9-812d-7ecbe2dbaaff#card` | 24 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Presumed Dead | `a2d3e3f0-7399-4eb9-a0cf-831710c3d431#card` | 27 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Primal Witchstalker | `c21fafaf-4510-4752-a501-606ae68e6699#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Prismari Charm | `c3c25a2c-f71c-4cb7-9117-0bcda9248bfd#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Prison Break | `9c6a6332-37f0-4d7d-a86f-478cf0eeec91#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Prized Amalgam | `8499b3ec-9a61-47a4-a880-e8230b41c81b#card` | 44 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Proclamation of Rebirth | `e1e5592c-3605-4069-89d0-b462dec83516#card` | 480 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Proctor's Gaze | `a7fe7621-9bb1-4d0d-8b50-c179c685d79d#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Profane Command | `6f9ecfc4-85bd-4171-9031-dbe0e39e0135#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Professor Zei, Anthropologist | `59a10335-72df-44a5-a514-5bac392cd282#card` | 24 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Project Deathlok Soldier | `d89aa87a-d59a-41e2-b92f-15c9bc6d3d13#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Promise of Tomorrow | `26c22894-10e5-48f1-a32a-a7f1f3f7e12e#card` | 92 → 50 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Protomatter Powder | `22d57bc8-e473-47c5-8e48-c0fb98e66948#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Prowling Geistcatcher | `175db9f1-0188-4640-9b54-6b461c5440aa#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Psychic Miasma | `43d69e13-27df-43a9-b610-6628dc02e3b4#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Psychic Pickpocket | `60ead8a5-11fb-4eeb-99bf-0caaf1de1731#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pull from the Deep | `1c0a72dc-4756-4d60-b6b4-4c2c69699efd#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pull from the Grave | `c5373026-8b43-42a7-aa69-a1b0f1619795#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pulse of Murasa | `d5b446a4-a761-42f0-acf0-ea0a1a9538d6#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pulsemage Advocate | `06e4355b-02d4-4238-a6e6-b03a5773346c#card` | 25 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Punishing Fire | `1890ef1b-38e5-432d-b040-f33cea7814e7#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Puppet Master | `42c7e933-f169-444b-917d-b4dca918d989#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Purple Pentapus | `37d2e83d-f5fd-48d2-b091-3c9167522530#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Push the Limit | `62e54c6d-d775-4597-bd4d-dc38460b33ec#card` | 80 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pyre Zombie | `accb82be-4f90-4d9a-bafd-f8de3622b3a3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pyretic Rebirth | `a38ace5b-34f5-4105-b6ee-dedb86037729#card` | 84 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pyrewild Shaman | `a1e71473-3eba-4d29-87e2-7f14acab2ccd#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pyroclastic Hellion | `b2151d51-39f6-49cc-a5cc-8202dd028336#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Pyrrhic Revival | `e8daf155-1fc9-4a0e-a7b5-69a50bc23576#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quandrix Command | `9c0f7cdc-9095-40ae-bf83-75a5ffda72b9#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quantum Entanglement | `4b072fbb-5e61-4009-bcb1-d493eb4a1be4#card` | 37 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quarry Beetle | `fff66029-24bb-4354-a650-99a205fd168f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Queen's Bay Paladin | `60b18d94-8ae5-4508-b4db-18f92c2d39fd#card` | 98 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quickling | `109899e1-abd7-4582-8b67-b0852755bf75#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quicksilver Behemoth | `799f31fc-3b6f-419a-8e70-b41c8e97f000#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quicksilver Geyser | `6ee7b00b-b875-48e2-92f2-889c154b0f82#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quicksilver Wall | `82372776-bedd-4893-92ad-f1272f9c3250#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quillmane Baku | `736168bb-0a5c-4b29-9f82-1319cd112873#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Quirion Ranger | `3ecaefc8-ead2-47a3-a7ea-b030faab65a7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Radiant Grace | `93cddbad-92ef-4b30-a0e7-981aee714bce#face:0` | 15 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ragamuffin Raptor | `71b81f3f-6092-4819-8529-e27bccfcf088#card` | 16 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Raise Dead | `cbc9c731-181a-4f00-a7b0-eb7e56eac2ea#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Raise Dead | `fd01229f-1c4c-4ad1-bc6f-4023f24d8c71#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Raise the Draugr | `8727284f-e9a6-49ea-9fb8-1076883ceecf#card` | 165 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Raise the Palisade | `f55a3781-fe33-4301-9bb5-6a54b9c13c4f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Raise the Past | `a69a24d0-ca58-4a44-8af1-a3bd1608d2f9#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rakdos Carnarium | `0a023964-2905-4928-9c3e-dc63e6ebd218#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rakdos Joins Up | `6a47865c-8fa2-4cb2-aebf-8009c065395f#card` | 900 → 180 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ral's Dispersal | `a81b7629-9db9-4351-8d0e-c45a7a6167d4#card` | 64 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rally the Ancestors | `9d612686-2d7f-49a3-b9c4-1e99a30134f6#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rancor | `9d2d6479-531c-4ce1-b52b-00e36fa63b64#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Random Encounter | `324493fc-a205-4177-aa94-cf4672844c81#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ravos, Soultender | `7880576b-8ad9-4382-97c5-7a631106a35e#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Razing Snidd | `bbcd2045-cf51-4346-a561-ebae8ffdc181#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Razor Boomerang | `f2649e48-4252-43f3-bd1e-c4c6fcb0aa3a#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Razor Hippogriff | `d121108e-f0bc-469b-bf94-e5e5308014a2#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Razorfin Abolisher | `38a2ded4-77c5-4d2b-b411-7dca8108aa02#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Razorlash Transmogrant | `580996d4-db9a-4b57-a0e1-a2dbb524121a#card` | 81 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reach of Branches | `951b59c5-6d5a-4fca-b3ae-e783814e7e7a#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Read the Tides | `e5c4ed40-5069-4606-81f2-59dcfa1345bd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reality Strobe | `2097ac9e-0f9b-4cd0-8537-2518195e4549#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reap | `7fcb8472-1e77-4dcd-b18f-8abf8806da08#card` | 60 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reaping the Graves | `80c663a5-2794-4748-b47a-34352fb2112f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reassembling Skeleton | `9dbc3530-b278-4c8d-b2cc-a09dfac9d5e5#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reborn Hero | `5562b4bd-1485-47d4-a087-d93dafc37272#card` | 36 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reborn Hope | `683d8deb-2ab4-4c65-9a23-f4e733f06ba2#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rebuild | `496999a7-a82a-45ea-9c4e-1b6aabc781cd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recalibrate | `e6331b3a-81a8-48e8-8a28-237d381f9062#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recall | `4158f397-4ee4-4f53-90f2-3b64bfe4f9b8#card` | 60 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recantation | `02269353-2949-4599-b5a3-f20fdb501285#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recoil | `1579bf86-4400-4467-88e8-1c5ba1209804#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recollect | `5b38fafb-3999-46dc-928b-1677625c1943#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recommission | `e43e8c00-f1e7-49d2-bb27-f1c16d85cc22#card` | 552 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reconstruct History | `97fad58a-8ee4-4f01-a47a-5ddc9ed845af#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reconstruction | `ad8fb78b-5ca5-4ef1-8c68-ee57d1e32fec#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Recover | `f4361602-63da-430b-adbb-36d74e987aa6#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Redeem the Lost | `3c5e269e-1baa-4b40-8e0f-1763ac85dc6b#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Redress Fate | `e8ff6780-5f22-42fa-a7c1-7406e8203063#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Redtooth Vanguard | `9e459e48-d2bb-4c4e-9a87-58efb56cddd1#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reduce to Dreams | `85bac6ad-b63a-4068-a813-ad685966e171#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Refurbish | `e4e61fe3-27a4-4149-bdfb-67dcfc8eb32c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Regress | `2fa763ad-4d94-4c3a-a099-13ac22c09ce4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Regrowth | `e5152e8d-9f55-493d-a0fc-753cc72ee299#face:1` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Regrowth | `e6e4a8bd-5c40-4654-8de1-0da9afed90fd#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reincarnation | `d6bf5e22-8d33-43a9-8824-435068e0a87a#card` | 109 → 39 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reinforced Ronin | `5e891927-b6cb-40ae-8270-0e72820fd7f5#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reins of the Vinesteed | `db616096-ce15-4c32-b8ff-0fda4dd3b0de#card` | 60 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rekindling Phoenix | `f34be4b8-126d-4b5c-8ebb-23b5e5370012#card` | 90 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Relearn | `13cb7d87-ff12-4742-8f54-f3175536db40#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Release the Ants | `b420e429-9dec-4993-a45f-3458795e5cbe#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Relentless Dead | `7404c078-228b-4296-bf1f-62f57bf832d9#card` | 48 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Relive the Past | `dcc51bcb-a61e-471f-8a01-7fcbc069fcba#card` | 99 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Remove | `a1ed0f7e-56c3-4866-89c4-77b81a952955#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Renet, Temporal Apprentice | `6c754808-86a6-43d3-8aeb-96ec19f7396c#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Repair and Recharge | `4f42517b-e62e-4b8c-8c56-cb32fb62c478#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Repeal | `b1f2b26c-91b2-4f3c-9f48-6429ec3fde48#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Repeating Barrage | `a08c599e-ee75-4d2b-ae9b-0987c233dd17#card` | 30 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Replenish | `09b7ea91-2620-4c45-af3c-60bbeab89966#face:1` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Replenish | `523ae937-5535-490d-96ea-07f331b5e5ad#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reprieve | `f0449cbd-855c-4c20-a1f5-f76a395d8d39#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Repulse | `6f7ef1d1-c441-41ba-b7d6-6b7727ffcf04#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Repulsor Bots | `facc26d5-bb53-4f91-9b8c-73bf4dd44fcd#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescind | `3f487c8e-a923-4031-b0a7-04f778544fe0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescue | `f0ae687a-7222-4521-a514-ba10373fed7e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescue Girl, First Responder | `df6fc0b4-333b-4f6c-804c-ba6c8e29c88e#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescue from the Underworld | `7217ea53-3685-48d6-9df1-58a551f8c979#card` | 48 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescue the Foal | `e71863a7-0de1-4ab5-95e8-c39e6810d899#face:1` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescue, Pepper Potts | `54917578-9493-499e-b200-51d5cf7978ff#card` | 32 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescuer Chwinga | `be1f0d72-14b5-41be-a37a-7abeb52ec0e5#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rescuer Sphinx | `d4c4ad11-7e71-4ad0-aee7-58ac7b1b38f8#card` | 36 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Research the Deep | `02f35851-c1be-4162-a6f8-7897325555a7#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Resounding Wave | `7e607fc7-ec22-4249-8936-70a026f9996d#card` | 4 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Resourceful Return | `952ef791-5c89-4965-9f34-0228acb18bc4#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restart Sequence | `c415bfdd-3d42-4f94-9f93-7e310becbc8b#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restless Dreams | `65f1dd09-0640-47f8-8bfd-7aceaa5646a6#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restock | `8b641cd3-bb08-4f2e-9dad-24f58a9c32e1#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restoration Angel | `dfbd3afc-9905-4cff-a4f4-df08a4d0a7fa#card` | 88 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restoration Gearsmith | `447c7fe0-6ec7-4163-9d9c-f08f0a4c7109#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restoration Specialist | `36f71d49-0f64-4b5d-b1ee-bb0af5e9db9a#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restorative Burst | `78780f37-f215-4ae1-adce-6255b0146873#face:1` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Restore the Peace | `aeca0661-8b71-4085-81e9-bb0884fc9831#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Resurgent Belief | `0fcfef04-a954-4423-82b0-c70dc64a8fbc#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Resurrection | `837417d8-8260-486d-a3ed-3b5711eaf34a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Resurrection Orb | `b410f294-cbb6-4016-981f-808fd9b7bbae#card` | 27 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Retched Wretch | `0d3b6e27-2f8b-42e0-9490-7b134e84bb6b#card` | 66 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Retract | `31cefc9d-cc13-4b32-a1f8-edbcaf35aa3b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Retraction Helix | `2707f9f4-2b80-47ac-af0a-99fc53af94bf#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Retrofitted Transmogrant | `058dae67-807e-4542-ae51-f2b1216ffaed#card` | 47 → 19 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Return Upon the Tide | `b0bd6135-4257-4f71-bdfe-71fb3f7a36a5#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Return from Extinction | `3937a96c-3be1-465d-98b6-1db67d2215e6#card` | 165 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Return to Action | `9c002b4d-bb6c-45a5-88e1-1df2958dff9f#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Return to Battle | `cf6676b7-b40a-46bd-b166-bd3cbb49d80d#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Return to the Ranks | `0d4ee553-f206-4e70-a7b2-a3670440aee7#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Returned Pastcaller | `58b9f933-7b13-4335-996c-4722ab9e08b2#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reveillark | `1be13ede-98f8-497e-800c-03e5802932b3#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Revival | `836ecd67-6487-4f3d-9e08-013746be829a#face:0` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Revive | `51991d28-137e-4690-a674-d852b904bd1a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Revive the Fallen | `2f857ef6-c1e5-4b85-b3fb-8163be4895b5#card` | 30 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Revoke Demise | `83f0b573-58a0-457d-80e5-6e0b690e5f4e#card` | 75 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Revolutionist | `d00607b1-77ce-4b89-ad04-763d0d390c5c#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rewrite History | `f91c860e-3feb-4a5a-b7ef-4e7d38d61c75#card` | 77 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Reya Dawnbringer | `08d1a5d1-141d-48e3-ac22-c2529191cb13#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rhythmic Water Vortex | `373c6cd0-0bf7-4dab-bc18-7ecf5f015e10#card` | 60 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Riddlemaster Sphinx | `29f61caf-1aba-40cd-aff6-c3685a0997d7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Riders of the Mark | `c7031047-5109-482f-b2e9-14800b9279bf#card` | 56 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rienne, Angel of Rebirth | `1ef2415f-f88f-41fb-b084-ea52631a60c1#card` | 36 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Riftwing Cloudskate | `ae2e6ffd-fb93-4c47-88e7-e84460eefbe5#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rimekin Recluse | `ad99e430-e13f-4819-a2ee-bf3332ef517d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ringskipper | `8e353b5f-0692-430e-9558-59818275b13b#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rinoa, Angel Wing | `47b29cf9-8ef3-4b6a-a9a3-3ab822c5dea4#card` | 16192 → 9592 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Riptide Chimera | `718d72c4-ad37-49b6-860a-0b921c0de978#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Riptide Laboratory | `444d50dd-a44a-42db-bbf6-d0978e3bd6a3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rise | `814f06aa-b48e-4c7f-b4cd-595d89176d0e#face:0` | 15 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rise Again | `4600e968-fe3a-446a-a306-bd10e6763e0d#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rise from the Wreck | `8eeb83b4-dca1-497e-a6b7-0485807aeef3#card` | 21 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rite of Undoing | `9dbfa026-e364-4111-a03a-e9b1693bc7b7#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rite of the Moth | `a7fea839-d7d7-48b4-aeaf-133767532e32#card` | 45 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rith's Grove | `e13289e5-370b-435b-a38e-cf57c3078cec#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ritual of Restoration | `7cfd0320-0b1c-4ef4-8e17-600cbcbddd6f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| River's Rebuke | `c52cfb41-18f3-4e73-b5e7-d75baf74e578#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roadside Blowout | `03008c7d-c738-473d-a968-1922b2899d1a#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roaming Ghostlight | `0d5eb0b9-9b08-4d26-b141-a9d612a680a6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roar of Reclamation | `a4372c4c-3577-43ca-a801-1f2bd1488a2f#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roaring Primadox | `2a098296-6a64-48ba-af40-5b68bae06bec#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rob the Hoard | `56b0cb64-b886-413e-bb20-392bf12d3926#face:1` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rodolf Duskbringer | `26a716b2-a2e2-405b-accd-df572416d8bf#card` | 96 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rofellos's Gift | `7c309759-e607-4bad-b615-f7c673e508ac#card` | 175 → 35 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rogues' Gallery | `7ea1b0e8-a577-47e2-b4cb-a6ca973742d0#card` | 21 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roiling Dragonstorm | `6badc583-fabb-4d63-b70c-e7380acafc23#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roiling Waters | `77fef82d-e8d0-444b-8d76-68e9b533f38b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roll-Roll-Roll-Roll | `aa913409-7a7a-4e74-82ff-7ff8195585ad#card` | 81 → 45 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rona's Vortex | `21989fad-9672-46d4-b7d5-101906e7aed2#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roon of the Hidden Realm | `fd336830-4a11-42b8-9fc7-d7526f569124#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rootha, Mercurial Artist | `3735559f-efe5-43ad-9b3e-3ef127c0ceda#card` | 50 → 25 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Roots of Wisdom | `8f6af612-a4a1-4165-ae4d-ff6353cbaf8d#card` | 32 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rootwater Diver | `4145c153-1ee3-4583-8a60-8471959656eb#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Rot Hulk | `36117e99-a2bc-46f9-8e17-bd2e2a9c31f4#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ruin Ghost | `169705c3-32c1-4628-b108-c37ca5f27e24#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| S.H.I.E.L.D. Flying Car | `12d51534-a98d-487b-9ba9-c10861dff992#card` | 32 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Safe Haven | `8f7f4061-8998-40e9-b980-c63f27e7a5cd#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Saffi Eriksdotter | `84ba3e2c-ea96-470b-92c6-b937dc87549b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sage's Knowledge | `b8ffd53b-5b10-4866-83fe-630eee58c514#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sage-Eye Avengers | `97a1b49e-d6bc-46ba-88c1-991057e64170#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Said | `5d1d76e0-049c-4a33-96ec-8e60deed16d5#face:0` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Saltskitter | `831e9f71-9664-4dbb-94f5-935d7cf95d77#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvage Scout | `502e991c-f4de-4095-adac-cb0c25e44a7c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvage Scuttler | `01a6f8e6-439b-4de4-9f0a-21a035fd849e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvager of Secrets | `3e400e13-6cae-45e6-b5cc-5b5f9d74dec5#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvaging Station | `3501ed98-473c-4a17-8b5f-1e0c00ee75da#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvation Engine | `7903666e-7803-4cfc-9ed9-482a6b577c96#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Salvation Swan | `0f68ae67-1671-4281-8b6b-3e52fdd4915f#card` | 1386 → 770 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sanctum Gargoyle | `6ee9802a-7a44-49c7-93d9-f75c4d42e71e#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sandstorm Eidolon | `84fe1c59-3f7b-4c1c-9bc4-78a185cfcc0c#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sanitarium Skeleton | `d09b0648-e324-4a16-b78c-d1849177c04f#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Saprazzan Bailiff | `1f861517-ac53-4218-b62e-35e971098b6d#card` | 25 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Saprazzan Raider | `ca011fb1-fdc6-4eec-9344-c247339292ab#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Savage Knuckleblade | `5a4fabfe-a641-49b7-8134-9d9edeb8bf1b#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Saving Grasp | `f7a4458a-7e8a-46bb-b62d-991c606c7a15#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Savior of the Small | `15f2085f-d5e6-4bc6-870f-00fcaccd981b#card` | 128 → 32 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sawtooth Loon | `b917e472-2550-4381-b756-45487c2c187a#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scapegoat | `04eb4fc0-0cc5-44f1-af5a-44579125d5c2#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scarab of the Unseen | `5f085af7-ce66-4019-9516-d1650ee7b6e0#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scarecrone | `f45716dc-c613-4b14-865d-419395fb02f6#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scaretiller | `e4b32e47-5484-4ee9-bac3-4ba37558a56a#card` | 28 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scholar of the Ages | `1bb1ad32-9ce5-4f14-a408-e142ff30c679#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scourge of Fleets | `57308fc8-8915-4211-a5fd-9363caea9ab9#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scrapheap Scrounger | `d309e1e9-26e6-4a10-a3f4-352c7ba3f2ff#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scrappy Bruiser | `2976f47b-9626-4934-bcee-7ee7f57944c3#card` | 16 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Screams from Within | `a3b3b9ae-1e48-481d-80a5-7e2a26cf2f50#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scribe of the Mindful | `019a3229-dd1c-4f70-a8fb-e33dbc54df39#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scrivener | `27c37a8d-777a-400d-b096-bde195228ea8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scrollshift | `9f762b9f-7525-4ff8-bc1d-fbab66fd4013#card` | 66 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scrounge for Eternity | `71982b17-1d9a-4f98-b8b5-d232e3a8cf1d#card` | 23 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scryb Ranger | `d1961110-575b-4a1b-9cee-db0e1f0fdbc1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Scythe of the Wretched | `9dc26b9f-3b02-46f2-88f8-eb6dffa04ee5#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sea Drake | `44d1029f-604e-44d6-89f6-bd18344f5d79#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sea God's Revenge | `8ab71a49-ce81-44f7-b940-50e7aba64df9#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sea God's Scorn | `d2497a5e-ff2b-4024-9b14-634304d583fe#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Seal of Removal | `f0801029-bcf7-4bdb-84bf-e88dcaa9dc03#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Seasons Past | `c46738a4-e0d4-41dc-a20d-4800f00cea62#card` | 32 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sedraxis Alchemist | `56f3f1b4-c36a-41f3-95a1-45e9d7e74cf9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Seedling Charm | `b8d66cd6-15db-41ff-bae3-60f1aad3ea23#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Seeds of Renewal | `48d63b86-8ddd-4306-960f-98490d1d80b9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sekki, Seasons' Guide | `1b92501c-80fe-4ae7-834e-72f57ba41a6a#card` | 45 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Select for Inspection | `cfee4ba9-3cf2-4e8b-a4a7-21974828cd54#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Selective Snare | `85442273-9596-47bf-9b40-6d48fd7183a6#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Selenia, Dark Angel | `3db1ac09-e35a-4f50-97c4-f65dbc28daef#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Selesnya Sanctuary | `00ef1c55-dea1-4564-bd57-66de86cba4df#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Selkie Hedge-Mage | `1f1c369a-3209-405b-9880-0e10b86e5698#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Semester's End | `3aeba3c2-5517-4870-8802-29b0fa70ffae#card` | 6545 → 3570 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Separatist Voidmage | `d9fabfa8-b930-428d-af4e-640fb310a5b2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Serum Snare | `fbc7350c-d392-44bf-92f7-8dd4743a73b4#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shackles | `05861ff2-ec85-43d1-b641-c5d2da819c03#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shade's Form | `682a6d77-e8cd-4bf2-b777-7793101e0fd3#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shallow Grave | `9843d8a6-bcbf-40c2-8c41-316e97db9c62#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shambling Cie'th | `d3f4aa87-a89e-43f8-abc3-712e6bf08889#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shard Phoenix | `3b629793-0efb-4d60-9d82-38b7b400abfd#card` | 18 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sharuum the Hegemon | `037e7fc9-3aa6-484c-a2c8-43009e45f1d8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shattered Crypt | `77bef290-7229-420a-a5c6-8c1b6fea63df#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shatterskull Charger | `571f9d80-9c37-474e-9524-400e7a238e2c#card` | 54 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sheoldred's Restoration | `df587772-516f-4ae8-aefe-e6fc63f8b365#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shi'ar Soldier | `f20ec8e2-d6e4-4707-9b86-107dbc09ac63#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shieldmage Advocate | `57188b3d-567c-4dff-8b93-7cc7a47894be#card` | 120 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shifty Doppelganger | `b9be00a5-448f-4c90-b9ca-b30bcdfe95f2#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shilgengar, Sire of Famine | `71ac4d8c-5b58-4ea8-996b-f0289717c4bb#card` | 1620 → 324 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shimmering Wings | `7d892d61-5a69-4500-9fb2-7cf50728aedd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shimmerwing Chimera | `21c768fa-4f1a-4a1a-abb9-e69e17426233#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shipwreck Dowser | `dc3073fc-dc23-426e-b867-fee5dd99fe25#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shivan Phoenix | `49ca8516-624b-4aa8-8e13-d145590cdfda#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shivan Wurm | `fc988ce3-0cad-41a5-8f13-656caca732df#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shreds of Sanity | `6eaa343d-0097-4f05-80ab-deb016d4e15e#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Shrieking Drake | `2a8e14be-6f93-428d-a204-e2399dc93797#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sibsig Muckdraggers | `423c1079-bdba-42fc-8732-59cb83ebafc3#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sidar Jabari of Zhalfir | `b229f314-a5e5-41a3-a8c6-217a3c5a61c3#card` | 1344 → 192 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sigardian Savior | `14bc9e20-a6ad-4eca-8c5c-5459ce078d9a#card` | 30 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sigil of Sleep | `51b47053-d169-4627-982f-f29b8630d75f#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sigil of the New Dawn | `cb029c0f-b08a-4952-b236-8166b9f17119#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silent Departure | `c41641f0-1abf-4776-9dec-1882f2d9badd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silent Sentinel | `26396b80-e5b7-44f5-916c-56a9215a7a49#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silumgar's Command | `7052fbd6-94df-41a9-a382-641701086804#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silver Drake | `ec7eb7d8-92b2-4442-856e-a9790301c3b3#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silver Surfer, Cosmic Voyager | `3ce1f550-d044-4cb1-991b-e98449f74507#card` | 256 → 144 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Silverquill Command | `6075b999-75c5-41c8-898d-ee612ba79a92#card` | 24 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Simic Charm | `c767aa2e-0a95-401f-8e36-d90a87648eab#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Simic Growth Chamber | `046f5783-cc7b-416a-8cf6-2bcef9c2cc1a#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sink into Stupor | `bcc6eece-75ea-494c-b33a-d4477d504e0b#face:0` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Siren's Ruse | `58a55d14-b03b-4648-b168-ad35ff088b19#card` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sister Hospitaller | `06c39c1c-0e34-4ef0-bd06-23245cb9cc59#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skarrgan Firebird | `42a96276-fddc-435e-bd47-04f9c6001ec7#card` | 15 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skeleton Crew | `2536750c-3c4f-4e00-b517-680ad4fd2ecc#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skull Collector | `42ec62b1-c899-4ebd-b786-28edf76ce8e0#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skull of Orm | `67601399-089b-44fe-82db-c319729c75bf#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skullwinder | `d3d8dd6e-a80b-4063-a542-ff67ef8dad4f#card` | 27 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skybind | `13d34f21-f0ce-4e59-ac3e-6635bf7292d3#card` | 32 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skyblade's Boon | `a03a9ddb-595a-4dff-b7cb-7ad60655f1ee#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skymark Roc | `b7564801-e35c-4d5c-afa9-99b33b35309c#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skyskipper Duo | `a50db36c-948b-4132-a5e5-84aab1f42612#card` | 48 → 27 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Skywing Aven | `afc3a65e-7a7a-4bd9-bca4-aa762b9bbbcb#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Slab Hammer | `f353948c-b5f9-4840-a89a-19206dde9d47#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sleeper's Guile | `eaaa3e41-dbf6-43ae-b65e-e1502ba4fa0b#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sliptide Serpent | `25ae99c5-83b7-428d-b049-8ad40e250696#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Slow Motion | `151388a4-a24b-4089-a323-e7c1d5e6490a#card` | 90 → 45 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sluggishness | `39eb4062-68a0-4f64-a9cc-79be87fd8173#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Slumbering Walker | `6d972501-4174-4bf0-a26b-2edef31b3a0c#card` | 40 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Smile at Death | `9228f04c-506f-4453-a335-e66876b8ce8d#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Smoke Shroud | `a621c5f6-c1c6-4bc9-88b1-f459efa07a39#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Snap | `ac914d98-221e-426c-8a50-342896b15f9e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sneaky Snacker | `f255492e-4e12-48b4-9c19-8a2a2780a1b7#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Snow Hound | `2fe8512f-826c-400d-ba6d-1de4f315b617#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Cloudskater | `ec6a09d2-d90a-4c3b-940b-659e471baf88#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Mindsweeper | `2ef6f4d6-f755-4402-8339-2a9b6c86f96c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Mirror-Guard | `a3caf864-a9d4-4854-9fc3-408354ed1fe6#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Mirror-Mage | `03e1cdd5-b687-4dc0-967e-244b70f5ed15#card` | 4 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Rainshaper | `63c5f42b-13d0-4391-b033-4ab2f4e3152b#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soratami Savant | `33cbb650-a756-4e06-8b18-6cc97b2340b4#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soul Collector | `7b7b3a34-13ff-45d4-ad0d-ac4bad9d0581#card` | 18 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soul Salvage | `60b6bd93-3eb3-4b81-bc4d-1e3351ee2e64#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soul Stair Expedition | `24b11ede-26c8-40ed-b874-c0f6b21572c3#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soul Strings | `fac310c7-c1e5-42f9-8140-17472be21d0d#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soul of Innistrad | `b5ba8155-520b-4053-be31-2744b77fb862#card` | 98 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soulherder | `92019547-f6db-4ea6-8356-d0a90ace5662#card` | 208 → 112 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soulless Revival | `5f00dcc4-38e7-4dde-96d4-f8d4c08b334a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Soulquake | `0b9a477b-f2dd-47b2-8493-eb63f44a2688#card` | 10 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spaceshift | `5c63a244-75d7-4372-baf4-7a3526f240c8#card` | 276 → 152 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sparkcaster | `84c912e8-6a92-47f7-88b6-e777bd0bb424#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spawnbed Protector | `256a7f54-0e8e-4d22-a0f7-ff2830e8884e#card` | 54 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Species Gorger | `0e741c79-5ea4-457d-a7d3-b11154459246#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spectral Deluge | `bb9bb65e-504f-4edd-8d47-d0efa03e7d17#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spectral Shepherd | `d28b7c07-ef65-4187-a15e-2180bdc593d1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spectral Steel | `95f74c0d-3f89-4944-997e-1bdc408bae71#card` | 32 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spellkeeper Weird | `91e09399-3f09-4323-8533-51536ecb7869#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spellpyre Phoenix | `a86710da-e498-4ca1-832c-8ba4abd6fa46#card` | 129 → 15 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spellweaver Duo | `1fd4204f-f627-4919-9d48-79b172f4560e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spider-Byte, Web Warden | `845472ca-3fcb-4bcd-953c-96eb8d914af9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spider-Man 2099, Miguel O'Hara | `c0280d66-699f-4139-828b-a0f79ae2d20d#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spine of Ish Sah | `02f062f5-8012-4440-ac12-49fc49822106#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spirit Loop | `6edf5767-10d3-40b0-bfce-daa8633888f6#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spit Flame | `f63278e0-3c67-421d-87ce-67725e5e74df#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Splash Portal | `29b537f5-4d9f-40ed-b38b-e8faa2b5af7d#card` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Splendid Reclamation | `13fe5e46-77a6-45d8-ac0b-c3d740eccf86#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spreading Algae | `a08189f2-1643-496a-8a91-74864528b4db#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Spurnmage Advocate | `0477b923-9f32-4e7f-ac08-c715cca819a4#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Squee's Embrace | `095f8dac-15b8-4c28-ae33-6dc71152bcc7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Squee, Goblin Nabob | `0ef760f4-bbec-4735-8d60-fc24d973452a#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stampeding Serow | `7e605a8c-d6d4-410a-aca4-52576d08d81c#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stampeding Wildebeests | `c270c770-91ed-4540-b281-c4b60b943b34#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Steam Clean | `12c71296-1243-49e6-a196-8b016a7b635c#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Steel Leaf Paladin | `15e0250e-a23b-499a-a8d2-24fc13084c28#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Steel Sabotage | `0e71fc56-0332-4e5e-8658-9257cc0d8a1d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Steelshaper Apprentice | `bf325b3a-0b28-4660-8b51-4334b59a9034#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stern Dismissal | `319d5cf8-ab72-401e-b04e-c13a7a6c1aac#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stern Proctor | `5120bfdc-7201-420d-85a1-eeb4aba6632b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stickytongue Sentinel | `899f0931-4e9b-4fa8-9f6d-d13a7de05334#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stingscourger | `3e8d1051-1739-4a10-9cef-78f7333b4979#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stir the Grave | `296dcc57-5651-4cb1-b89c-fcdd3f5311bd#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stitch Together | `bc3d5911-3580-4132-9daf-2826495b5739#card` | 300 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stitchwing Skaab | `0b40e537-d385-450d-93a9-b60a25bfe508#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stockpiling Celebrant | `171150fb-dbe1-4a07-a7ac-5f45762a26b0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stoic Builder | `cca94004-fe8e-404d-9391-ad33d0a126fb#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stolen by the Fae | `5bafd0e0-8bf8-4e06-a54a-81cc6cc4bdb7#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stonecloaker | `b2d597f4-162f-4adc-9dc9-6d44815b7a68#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Storm Cauldron | `5a51b168-02d1-4eb4-8ccc-614ab6f6cffa#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Storm Sculptor | `e43eb15e-e910-4165-b9df-25cbe7054e0a#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Storm of Souls | `7022ae9a-2589-493b-a050-a230fd02f80b#card` | 70 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stormfront Riders | `9df318b9-16c6-42f1-9f81-6f73ca01e5c1#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Stormwatch Eagle | `640b67d8-75a4-44d4-a2f5-7323bad4e1a8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Strands of Night | `0e8ae1f7-8926-414a-8434-db0b0f877505#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| String of Disappearances | `23985249-c54e-4bfa-b69a-512422a46a77#card` | 14 → 7 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Strongarm Thug | `a4d46c1b-7742-4ac8-9794-b669ee4d2cc1#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Struggle for Sanity | `5c0f63ad-4bb1-479e-9760-5421265da40b#card` | 72 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sudden Reclamation | `c34dec60-031a-4cd3-b2e5-db7a02a34d96#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Summon Undead | `c86ff0e3-5d6e-4981-a2da-8da70b9daeb0#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Summon the School | `00909d51-3da8-41eb-a535-0c325fedc59f#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Summon: Leviathan | `b38d1811-64d7-4a35-a86c-3291eecd8391#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Summon: Titan | `66ce0c60-f406-4ece-915f-675da4e8b16d#card` | 28 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sun Ce, Young Conquerer | `93b0185c-b86f-4137-bc6d-96212eee1708#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sun Clasp | `cb8f5ee4-285a-4c37-adcb-514b3e4bdad3#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Suncrusher | `4e81ead0-df8b-4d9d-b1cb-d1d9dc503d4c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sunder | `fddc93a2-912e-402a-9d0e-34f778e659ea#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sunderflock | `c01b221f-3993-43ab-8cde-b42fb344e2a7#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sunken Hope | `1820b6e9-83fd-40b9-953e-ba1a8ae69614#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sunpearl Kirin | `f17b9356-a329-4f89-b690-afc2eaff4977#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sunscape Master | `e3be8e19-cec1-43ae-9e22-d7cb374570bb#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Super Mutant Scavenger | `2972b9c8-d9b3-4bb3-b9c5-9aa70fb1ac71#card` | 16 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Supernatural Stamina | `65756e6d-9825-4ed6-91c4-79f1e3bd4ae5#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Supplant Form | `7e49f689-996a-477e-a327-32b69fbc0d85#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Suppress | `b39651ad-1c05-45f8-b036-536c4d7324c0#card` | 50 → 25 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Surgespanner | `3f15349a-6ac6-4091-95a3-ae9f593e4650#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Surging Aether | `060b1a76-22a4-48a5-a353-4d0501367956#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Surprise Deployment | `61a1748c-e772-4338-9edd-720cd5d2fc12#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Surrakar Banisher | `d48fd4b7-b60c-4932-abed-a8df1184db25#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sutina, Speaker of the Tajuru | `91f410c5-4313-462f-8d21-da63988d919f#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sweep Away | `c6901a89-ac7d-4797-a027-97bb1cf42e60#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Swift Warkite | `0df3444c-ae29-4fcc-bf46-4edf52b90e30#card` | 28 → 14 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sword of Light and Shadow | `01c8a554-45a6-46ca-b40a-af0fd4529e40#card` | 42 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sword of the Meek | `215c287d-56a5-46da-b49e-8524b6d320a4#card` | 24 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sword of the Realms | `acfc3e07-757f-4ccf-a65e-bee07ce87f40#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Sylvan Hierophant | `e98361d9-1e3a-464e-bac6-75314fc49f32#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Symbol of Unsummoning | `c44f1a81-269b-4f05-8ff2-e7ce19a93937#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Synod Sanctum | `643ddf81-bbd3-4398-b5a7-1169c6e2b081#card` | 14 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Take It Back | `9357fc7f-c9d0-4ce2-a6dd-0ff1f24bfc56#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Takenuma, Abandoned Mire | `ac2dd694-d2f1-4025-8400-12332bdc882a#card` | 288 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tales of Master Seshiro | `096fbb51-be73-4edc-a051-770a12c09005#face:0` | 88 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Talons of Wildwood | `2ff309e3-90e0-40e7-a277-3df057f8f198#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tamiyo, Inquisitive Student | `1ed9e351-0000-4b75-8ecb-d15bd88df68a#face:0` | 588 → 312 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tasigur, the Golden Fang | `837e3fde-241f-4826-ad04-f146fdcf2278#card` | 36 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tatsumasa, the Dragon's Fang | `ab739c7e-246a-45ac-bbec-ba44d044dd03#card` | 128 → 72 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tazeem Raptor | `3111f8b7-36af-4390-af99-5b8976b2f241#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tazeem Roilmage | `55a277b4-575f-463b-913c-7857acad20be#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Teacher's Pest | `12397dbf-99a9-40c3-aa7c-76d09baad008#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Teachings of the Kirin | `cb7edb5e-3a90-41c7-ae52-dc564086bf22#face:0` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Teeming Dragonstorm | `2154cc68-41a4-4e88-9fe0-e0150dc0be45#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Teleportation Circle | `c786a0aa-d86f-42c3-a7ea-5cb6ab72b5ec#card` | 111 → 60 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Temporal Adept | `fd9b5462-6578-4bf6-8026-23eaf5af3eee#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Temporal Fissure | `8a9525f4-74a6-44f0-a1b1-d57937d7783b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Temporal Machinations | `f0adab66-c3fe-4989-b7c8-ab7bdc7f0fd1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tempt with Immortality | `06e1c0fa-767c-4204-972f-d98f770d85f3#card` | 250 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Temur Sabertooth | `8bf8e846-9d10-4dbc-a379-df37bb9fb6d4#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tenacious Dead | `9ae376d5-c68f-4a4b-851c-ebfd385b86dd#card` | 18 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Terra, Herald of Hope | `25d40d28-1b46-4055-959a-45719040ed30#card` | 264 → 96 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Terror of Towashi | `c4b6dd8f-cdef-413a-a820-ec47e168b3bd#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tethmos High Priest | `f922150c-8f79-412b-a611-59064e3e3cea#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Teval, the Balanced Scale | `c8cbf0ec-ec98-4cb3-8068-60e92bbd740d#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Thalakos Scout | `c8877da5-7008-4b47-955e-d205220c91e6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Thawing Glaciers | `c6792d9f-8b74-43c4-814f-ba4adab2fdea#card` | 56 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Binding of the Titans | `f0435065-a8ca-4b4d-a7da-0ef41749118f#card` | 96 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Eagles Are Coming! | `169a4a77-12ff-4997-9550-bdcd456aa2f7#card` | 1110 → 555 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Falcon, Airship Restored | `9f4093a6-83f1-4c1d-8b03-8fe15647fb11#card` | 120 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Fall of Lord Konda | `6ba8e099-5871-4ee3-8a65-06fd8ccec340#face:0` | 176 → 96 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Great Work | `4b1da9aa-a30c-44b8-a10e-f9dc3fe70b6f#face:1` | 72 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Kami War | `4d23b22b-31a5-4eec-b894-2dd5a7138c5a#face:0` | 88 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Last Ronin | `e1865bec-1744-40c2-9031-7d6363be5333#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Legend of Kuruk | `201b8936-18f4-4881-a1d4-c308cf3832bf#face:0` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Legend of Kyoshi | `e41e8754-028c-4399-b530-de9e134d04b0#face:0` | 2376 → 1296 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Legend of Roku | `2f26f270-26d9-46d3-957f-19f52d51eb03#face:0` | 66 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Locust God | `e025a714-02da-4b0c-8021-cf3e8dc9b19e#card` | 96 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Long Reach of Night | `8b1e453a-35a6-4dff-bd37-498f34d0470e#face:0` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Mending of Dominaria | `9b0b8f00-6367-4dc9-b33e-680470b3a36c#card` | 50 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Mind Stone | `b175e826-09e8-4fae-9f2e-b902f95b282d#card` | 296 → 160 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Mirari Conjecture | `56528234-46c0-4fba-972a-0c8011996f8d#card` | 500 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Modern Age | `a8995df6-99a7-41a7-bd3e-f34643b27947#face:0` | 44 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Mountain-king's Return | `32ad5b3e-92c0-45be-b2e4-6f1794552f36#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Revelations of Ezio | `c7fe1f05-f412-4991-98bd-f6a4b8b62501#card` | 336 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Scorpion God | `aff223d4-6338-460d-a4d5-d45f522864dc#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Shattered States Era | `99473461-3c6f-4f7f-8ba2-d6c090ddb567#face:0` | 352 → 192 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Soul Stone | `92cfba68-12f6-4f97-9187-0f6a39656a0f#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The True Scriptures | `97652492-7906-4d79-983c-fa1dc1239eba#face:1` | 72 → 36 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Underworld Cookbook | `37d5c720-d090-44bc-91df-0a7bab79a03e#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| The Unspeakable | `64e413c9-96e5-43c3-a182-84d53fef4327#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Theoretical Necromancer | `6c5ed756-b999-49f7-9a52-fb8196202f0f#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| This Town Ain't Big Enough | `2550c2ae-834d-46e7-9ab4-d7bf7bfa6a78#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Thopter Assembly | `007c110d-4364-41e5-a8bc-6ac05ecbbd00#card` | 82 → 41 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Thunderbolts Conspiracy | `d90a5213-3363-4cd1-97d6-4fc057f78b2a#card` | 552 → 304 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tiana, Ship's Caretaker | `80f8fb4c-cc6d-4b41-abb3-471dc96d4b2a#card` | 48 → 24 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tidecaller Mentor | `e75e40a5-a9ae-4789-96c1-0e19d1ce59c5#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tidespout Tyrant | `331b44e4-cf08-44f4-8afd-a7374e5dc5ed#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tilling Treefolk | `8a011d0d-da29-4287-b50c-12746abfa38c#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Time Elemental | `d89075a3-4413-4796-a055-eef51fddb7f3#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Time Wipe | `36c78a5f-0148-4596-a346-f8e35037b694#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Time of Ice | `93e0fd89-8a0d-4d97-aa22-aafe83d53515#card` | 24 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Timeless Witness | `a52a06aa-56fc-4845-bbcd-a54f16369b34#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Timely Hordemate | `9ace6510-7876-48e4-9cae-1c24f2229e03#card` | 32 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Timid Drake | `aa8c5467-190a-4c6a-872c-56b8f8d01a2b#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Titan's Revenge | `0a0be326-643f-449f-a7ff-13b5ae569f20#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Titania, Protector of Argoth | `d0ade00d-a496-441d-9b7e-7dc033d3292c#card` | 30 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Together Forever | `36b19ec0-d581-4213-bfae-1d7808a2f60d#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tolarian Geyser | `d2240ce8-0bd6-4c4f-a73e-fb0a2af5bfac#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tolarian Sentinel | `bef15918-c0e7-4f6d-a589-f91f3fa11ef1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tombstone, Career Criminal | `6ad2cbb6-2291-420c-bfc8-ed3f079b59bf#card` | 40 → 8 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Toph, Hardheaded Teacher | `8c1c0117-6fc0-43c5-a4c8-6472de6503d9#card` | 72 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Toralf's Hammer | `fffcb71f-e802-436e-917d-eaa6607ab74f#face:1` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tortured Existence | `fa9dfcf7-069c-4441-9ea0-133de47ecada#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Touch the Spirit Realm | `52f0dcbd-ccf3-4525-8e55-2d38b4991834#card` | 192 → 108 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trade Routes | `120fa67e-e5e0-4c23-9a78-d6171d357aee#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tradewind Rider | `d0314da0-da24-4f6e-b1f6-0d50617bcaaa#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trading Post | `63788566-e25a-44bb-bb55-197e1b93b3e8#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tragic Lesson | `516d1ac4-e831-44b6-8266-131dde7fe802#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tragic Poet | `97bdfbe0-7048-4157-8a58-5b17410750ab#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trash for Treasure | `e69279f6-50a1-4db5-83be-30a2ea7fc6a9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Traveling Plague | `371e8e29-f569-4b23-bae4-cf89e5b09fd3#card` | 25 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Treasure Hunter | `71ae3bc0-28da-4e91-bffd-a49fe545b7a9#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Treasured Find | `e99d6c5c-b029-4cd1-bd7a-6f8ea91eb4a7#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Treasury Thrull | `34161ec4-5d99-45e6-b43c-c8d28958660a#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Treva's Ruins | `7b2c7758-2b89-49ff-8838-8dc9880c7209#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trial of Ambition | `bf614953-bd62-4000-a960-e2c26e1f7bb0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trial of Knowledge | `6155908a-0079-42c7-ae81-326a6faccb89#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trial of Solidarity | `cbcc2577-7556-4174-83ff-9e00611412b3#card` | 22 → 11 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trial of Strength | `b272cb56-dcba-479e-9fba-5b31e616a95f#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trial of Zeal | `0798eb13-83f1-4b7c-b537-83a9da2915a6#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tribute to Horobi | `81622cf6-3021-4ad4-933d-a7e3d8ac817f#face:0` | 22 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Triton Cavalry | `0fd11de4-2dfd-4345-b3cb-d7d65e11c6d9#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Triumphant Reckoning | `0d3e0b38-a9ff-489b-8c00-4ca89deb4869#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trusted Advisor | `430e4e74-33ee-49f2-aef5-6d09079c5eb7#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trusty Boomerang | `3c032e74-69c6-48ca-95d4-51aa3d894c14#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trusty Packbeast | `12c7289f-da53-403a-a607-227f43c7e171#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Trusty Retriever | `06a2a50d-f54f-4eb7-9cf8-ce37d802794a#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tune Up | `aaa14260-2537-4668-8517-c825118777cc#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tunnel Rats | `0a8e5c48-885d-4ea8-9c68-a3e9213d4289#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Turbulent Dreams | `83134570-debe-4bc0-9656-2a16801a3592#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Turn to Mist | `631f04d6-5693-4414-a378-fd839c3b6f6d#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Turtles in Time | `803971e6-67f3-4fda-aeb6-69ef8ea4b54e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tymaret, the Murder King | `0a029c61-b1de-427b-a8e8-36f3a23f001b#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Tyrant's Scorn | `afb278f1-0e21-4f6d-96bb-c34aa3ef96b0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ugin's Binding | `ed622e71-f348-4e46-8fb9-05aae430983a#card` | 480 → 120 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Umbilicus | `c9132e72-d016-4cb7-8662-a16b7d932bef#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unauthorized Exit | `3d145811-adb2-46a4-b173-bf8152e5de48#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unbreakable Bond | `36cfdbd6-f971-4ee5-aa1b-fa3ddad5b4b1#card` | 45 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unburial Rites | `d48e1545-7997-45ed-83a1-aee45b3d3d20#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unbury | `e2928d3c-b64a-443e-a1c1-fd204bb098ba#card` | 165 → 12 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unconventional Tactics | `95ee46ec-4b60-4fea-a09b-d15c083916b3#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undead Butler | `426fa8e8-b0f9-4b6c-b22b-04ec2073187a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undead Gladiator | `4da2425d-2118-4116-855d-2ef80693a7b7#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undergrowth Recon | `3fbe46fa-88da-439a-994e-c8f1791332a8#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undertaker | `a8ffbf6d-44e9-44e4-846c-4f894078130e#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undiscovered Paradise | `76c33d54-ce55-400e-bec5-79d33a5a20fb#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undo | `b446c3bd-3c7d-4d59-ba59-3dd80818465e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undying Malice | `8b061702-1bc3-45aa-a758-446b25034801#card` | 248 → 145 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Undying Rage | `622d783d-4201-4769-8a0f-442b358d8bf1#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unearth | `4e0b647a-6e1a-421b-bca9-8d5e346bda80#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unexplained Disappearance | `18d16129-da25-4120-a343-64ce962abba2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unfinished Business | `800cfd70-6a97-40eb-8dc0-40b762e4db76#card` | 70 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unhallowed Pact | `da1dd850-9118-42b9-8846-e6bd5042775c#card` | 9 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unholy Indenture | `df8fe368-00eb-42fa-89d6-794fc61f8f8c#card` | 110 → 62 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unliving Legionnaire | `0edaf433-4ded-4841-bc86-136780c0767f#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unmake the Graves | `d9b99c58-8e2e-496c-8288-eda934f49162#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unnerving Grasp | `7a335332-87ff-42dc-a547-c84295dd8138#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unquenchable Fury | `1c69a41e-122a-42c1-98d4-56035d68cfc8#card` | 18 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unseal the Necropolis | `b3ea71d2-6298-452a-9c54-7c880b7e9cc0#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unshakable Tail | `3d28e89d-d5e2-46b1-8dd2-d808b78f08fe#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unsubstantiate | `c387f46f-ad91-4964-8dc6-acefe537fe3c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unsummon | `837182db-1bf3-4a2c-bd01-1af9d9873561#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Unwind History | `e7b78acd-5288-4528-97a8-a5b43de90bf5#face:1` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Upheaval | `7cafc972-a6f5-4cac-a3d3-8a3ae36ffb1e#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Urborg Emissary | `f30943f1-473f-4fa4-b6a7-3dfbd1995ba9#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Urborg Uprising | `882d3be7-1c0f-4d2b-8f2e-32369488ef82#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Ureni's Rebuff | `c0b7e989-8194-484c-93aa-a44df0764a77#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Usher to Safety | `6d611800-5b0e-40ca-82a2-5b294a1ebf6d#face:1` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Uyo, Silent Prophet | `93da1e63-54d6-4b05-af91-f13e7e111176#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vacuumelt | `eb421a1b-147f-422b-bb8d-483a182a4096#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vampire Charmseeker | `993bc179-4e24-4161-866f-5e146cbdd2f5#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vampire Soulcaller | `bbea8ade-a641-4132-b306-f39fe2f14311#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vanish into Memory | `d3beb624-f589-4ed5-97a0-11ad61481250#card` | 11 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vapor Snag | `a1408bae-ed5f-4837-a1a7-438dc30a2786#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vapor Snare | `29ae5540-5fdf-48be-86ba-b4db0fe19f6c#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vault 13: Dweller's Journey | `586fb987-cd77-420f-978c-2ce140bcdcfb#card` | 1248 → 728 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vedalken Mastermind | `1ba85537-6dc2-459e-8850-619b41c0a881#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Veilborn Ghoul | `8381b04a-666b-4e33-b277-6db39fdfe905#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Veinwitch Coven | `ab478ac9-af59-4df1-afef-5e9806c06643#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Venerable Warsinger | `e286d1aa-bb74-4568-98b5-a693ac7a369d#card` | 120 → 30 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vengeful Rebirth | `522297b1-493a-4c62-a82a-3017c44183e8#card` | 30 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vengeful Regrowth | `c4d05f64-d1bb-4725-81ba-baf737f3066c#card` | 80 → 16 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vengeful Strangler | `ffc1fe11-0ecb-4497-a266-0817c79477e9#face:0` | 65 → 34 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vengevine | `7ed52301-81ea-4e7f-b985-cfab0593cae4#card` | 144 → 48 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Venser, Shaper Savant | `0f41cefc-d6ff-4db7-ba35-502b7e081de1#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Verdant Eidolon | `a3acc388-4015-42e4-88ff-b35a91134d12#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Verdant Rebirth | `c35540ad-9ab1-4be9-b97d-603813394a26#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vesperlark | `b801a701-15ce-4792-a3d7-d632e7bd9f72#card` | 8 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vexing Scuttler | `2a5605a8-606e-4866-9473-7a2240ac6b8c#card` | 24 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Viashino Cutthroat | `6dc1305a-d6ca-4280-ba5d-5ed91f78ced0#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Viashino Sandscout | `f8d5a65c-382b-4277-9c08-5dec9e395b02#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Viashino Sandsprinter | `6560517d-41c0-4c9c-b9d5-3ecfcb2b99fd#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Viashino Sandstalker | `cbfccfc0-3117-4ca0-b7ce-8e2d67ec49ab#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Victimize | `240e85d3-e495-4877-8609-4b4056c402f7#card` | 15 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| View from Above | `3a128587-c751-4b27-b608-d4e47646b80e#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vindictive Triumph | `d6b478d1-5015-49ba-b1aa-cbc2b08107a6#card` | 28 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vineweft | `f4a2d9eb-7f2b-4ee6-b38e-cf6e3a837b49#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Viscerid Armor | `87b4cbec-3521-4bdb-bf01-64ddb526b655#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vivid Revival | `b5d856a3-0305-4c3b-b447-99570d6e8195#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vivien's Jaguar | `32412a41-2929-4417-90bc-21c5659eeb72#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vizier of Deferment | `008d5896-6fc9-4aaa-8c6f-a44c2feb98bb#card` | 400 → 225 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Void Snare | `06e2ce8e-7341-4448-8625-cb1574eb179d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Void Squall | `e8ca5c7e-7d8a-4e86-91ee-2d3576f9fd5d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Voidmage Husher | `cdb1f944-a5ac-4683-a4eb-eebb82feea8d#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Voidwalk | `6706ebe4-4f96-4788-a934-f9b9aac5746f#card` | 16 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Voidwielder | `6686b897-64a9-4482-9563-57c88bc83d71#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Voidwing Hybrid | `0dab592c-a230-408d-a383-044e561dbe5d#card` | 3 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Volcanic Vision | `fe3b4c74-2bf8-464a-8f55-9de401a8c4b9#card` | 108 → 18 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Voyage's End | `1c2829b7-f73f-4023-95f3-9cd40b4125d7#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Vulpine Harvester | `cc5e3a21-4911-49f8-8e3c-ab23812c4004#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wail of War | `fc0abdf3-3a88-4cb2-ba79-35fcaebbf7d2#card` | 14 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wake the Dead | `69bf79c3-87d1-4244-8d58-989307c91a12#card` | 42 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wake the Past | `34143cfa-aa48-481c-af42-1240af174b80#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Walker of Secret Ways | `451a3f52-9810-4137-93b1-9594e7727c29#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wall of Junk | `22b0de2d-47a8-4269-ba21-4bc2b4b10b40#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wall of Nets | `cd112962-b297-403c-bede-c8b587f379b3#card` | 108 → 60 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wall of Tears | `5825d718-6a32-4f4c-9b56-1a50051cfe28#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wander in Death | `f25c5451-58fa-4916-8342-ae88c5e406cc#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wanderwine Farewell | `7e1da4a1-5922-4a2e-8bbc-25fbcbf20935#card` | 142 → 71 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Warren Pilferers | `9f90dcd1-e372-40c5-9de2-81392b0add06#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wash Out | `54748cb1-d92a-4212-ad76-417ee79b5ef1#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Waterfront Bouncer | `b53a8cde-494c-43d0-8d08-7bdfe2e84064#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Waterspout Djinn | `050dac46-9ba0-4b8a-b61b-1c7ec6f3723a#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Waterspout Elemental | `f86b8c03-9f1e-4d42-ac45-907c46e9e1c0#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Waterwhirl | `77636154-962f-4ca8-85e7-c0c9c0c3cd2d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wave Goodbye | `06ef46c6-00ba-40e6-b866-d0095ab83749#card` | 4 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wave of Rats | `ccb0f59b-7947-4563-9e16-5d23246f4232#card` | 65 → 35 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wayward Guide-Beast | `c752e001-56cc-46dc-acb3-3fca6c0bdd2e#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Weatherseed Totem | `5d19c68d-6eb8-4b34-9f31-c26931e59670#card` | 168 → 84 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Weatherseed Treefolk | `4e9b878b-80a2-433b-941b-92d854e83d58#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Welcome to . . . | `a4b37d16-95b3-4143-a0b2-ad9f2aba91f8#face:0` | 16632 → 9072 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whip Silk | `ce2c420b-e569-4715-b65b-4a097700799d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whirlpool Whelm | `7f527852-bf7d-43a3-be0b-784c73ed88aa#card` | 40 → 20 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whisper, Blood Liturgist | `496ab82e-24a9-4a74-ba4b-992c7309b44a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| White Widow, Free Agent | `4f0c055f-f614-44b0-9374-c53541f89185#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whitemane Lion | `e8d6084b-9b72-438e-a30a-851b888f3e4d#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whiteout | `9e8de3e1-1159-460f-99b7-1043e5814597#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Whoosh! | `855353e2-b686-482a-b987-c6ae456f088c#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wildest Dreams | `0a25a72e-5603-4b50-ac20-36b286cc1a2b#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wildwood Escort | `ce807ee3-a27c-4f3b-92a4-37faaca42aa1#card` | 36 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wildwood Rebirth | `7a4ddb8f-08ad-4bf9-82f6-72b073dd859a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Winds of Rebuke | `6840a63d-d627-4a18-afa9-018406767642#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Windscouter | `58232c49-f880-41ef-b808-62a6e804c715#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wingspan Stride | `e75cee5d-ab02-4227-b36e-61c5d2f564a8#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Winter Eladrin | `48ae84e7-125d-4018-afc1-cd78448d1a13#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Winter Soldier, Icy Assassin | `6f4c94b6-f829-4646-a574-083997b78dfb#card` | 162 → 54 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wipe Away | `ad9b6737-22c4-45cc-976b-32743ed48462#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wirewood Symbiote | `67a52a74-9474-4f4e-8785-6ff54078a8ca#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wisdom of Ages | `d9722ec3-aa5e-4c69-8ff0-cef1b1839fa4#card` | 45 → 9 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wispweaver Angel | `f8a88521-80e5-419b-ab4e-f4b3aba8afa0#card` | 104 → 56 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Witch Hunter | `5fc1f04a-2c39-4b3a-b62e-6e30c3d8a583#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Witch of the Moors | `7ad97dd9-1342-4ed4-bea9-1bb21d748e04#card` | 21 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Withdraw | `626882c5-15a8-43ea-ab08-33a7e949c88c#card` | 12 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wizard Mentor | `a1fb1b77-a49f-4447-bbe0-156407f192e2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Woebearer | `5ed93d71-3f62-49d4-a9ca-5728c44b0633#card` | 15 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wolfbat | `adc47a25-daab-49c7-93a5-5bbf2bc20cf9#card` | 84 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wonderscape Sage | `99994c36-21ec-4a28-b850-d3f2ba8b7f19#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wondrous Revival | `19cab267-3c51-4322-b1af-22941bbbdadf#card` | 9 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Woodland Guidance | `09e84398-7782-48d0-8411-9055d253395c#card` | 9 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Word of Undoing | `2597a1ee-fe9e-44b5-a5e3-d34332c6c123#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Workshop Assistant | `319ee221-bace-41e9-b386-6b528bd780c1#card` | 7 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| World Breaker | `deeb7e22-b207-42de-b9d4-f790403d52a9#card` | 6 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| World Shaper | `3c075bb6-1831-4521-bd8d-4ed2825ae796#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wort, Boggart Auntie | `6f477e2e-220e-48d9-9236-9564478e796a#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Writ of Return | `47735ef2-fc4c-4634-83ab-715b1e86da0c#card` | 7 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Wydwen, the Biting Gale | `7891ec87-8d1c-42fc-b00a-4c64ee5edbff#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Y'shtola Rhul | `a6a7bf77-0560-4572-a826-3bc9df1f78d1#card` | 444 → 240 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yarok's Wavecrasher | `30868d69-2ba3-4373-9d47-4bc8978f35d1#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yathan Roadwatcher | `0c3d86b6-5dcd-4016-9b4a-d4f66b1e963f#card` | 16 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yomiji, Who Bars the Way | `438be2fa-5f8d-42ff-894a-671ba4221196#card` | 12 → 6 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yore-Tiller Nephilim | `7e9d61d3-6ba7-45ac-a55c-a2f36595c5d2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Young Necromancer | `17455491-c295-4130-99b4-1b5c6ac1cd25#card` | 25 → 5 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yuki-Onna | `8f867a2c-f17b-4608-bca7-5b49d1387bc7#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Yuna, Hope of Spira | `3dee768a-612c-4960-bad2-2fe5b83a70c3#card` | 196 → 28 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zara, Renegade Recruiter | `cd2720c2-522c-4fdc-9cef-9c5ce250fe7b#card` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zealous Lorecaster | `f82265fd-e772-4347-a27f-37b00ccd416b#card` | 12 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zell Dincht | `f207ce73-e4cd-4dfc-b3b9-5704f5148959#card` | 20 → 10 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zephyr Sentinel | `dcddd143-a7d9-4a38-a2da-dd6bd2e48205#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zephyr Spirit | `ff640343-6980-4668-a41f-0f01b0bbc0f2#card` | 2 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zodiac Dragon | `e5c2cf0f-c1c0-4fd6-9b5f-7d4aa25246a7#card` | 6 → 3 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zombie Apocalypse | `8241277d-654f-4985-9d49-a22c1e59eec2#card` | 20 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zombify | `bb95db4d-5017-4121-bf79-d68476602d8c#card` | 5 → 1 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zoological Study | `6d4e7943-1fdf-4085-8af0-658badbd0cb8#face:1` | 8 → 4 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |
| Zuko's Conviction | `f034e8a3-82f7-4c70-93ea-0bf2688f66e1#card` | 10 → 2 | Wrong analysis retired: duplicate absent-source / source-on-Object Return routes |

### Review notes (2026-10-08)

- **181 gained vs 268 sole-cause.** All 181 gains contain *from among*. On the
  claim-parent census **460** unread faces contained *from among*, and **279**
  of them remain unread after this landing (460 − 181). The record's **284**
  remaining figure came from the broader 471-face From-among selector, not
  the 460-face *from among* set. The 268 sole-cause figure was a recon
  estimate; the gap to 181 is faces whose other blockers surface once the PP
  reads.
- **Residue re-routed.** Genesis Ultimatum, Kruphix's Insight (independent
  residual in its source-free whole instruction; source unit reads 2),
  Midnight Tilling (permanent noun-Premodifier gap), Bag of Devouring, Tayam,
  Luminous Enigma and the ~279 remaining *from among* faces go back to
  `english-v3-systemic-residuals`, whose take-over markers are reversed there.
- *From under* / *from behind* (CGEL, Ch. 8 §4, p. 687: a locative PP as the
  complement of *from*) are not licensed, by design: only *among* carries the
  declared PP-Complement head licence, and no corpus face in this landing
  required the others.
