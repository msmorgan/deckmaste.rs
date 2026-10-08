---
needs: []
---
# Read *if* with a verbless Complement: attacks each combat if able

## Why

*If able* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *if able / able to* touches **283** unread faces
and is the sole cause on **221** (recon bucket "if able / able to"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Ares attacks each combat if able." 0, while "Ares
attacks each combat." 1; "Jurin must be blocked if able." 0, while "Jurin must
be blocked." 1; "Target creature blocks this turn if able." 0; "Target creature
attacks this turn if possible." 0 with *possible* a vocabulary gap; "… if you
control a Zombie." 1.

This ticket takes over from `english-v3-systemic-residuals` the item "Reduced
if-able hosts."

## Goal

*if* takes a verbless clause consisting of the Adjective *able* as its
Complement, in clause-final Adjunct position, in finite
clauses with active and passive (*must be blocked*) predicates and under
coordinated predicates (*attacks or blocks this turn if able*). The
Adjective's unexpressed infinitival Complement is not realised.

## Analysis

*If*, *when*(*ever*), *where*(*ver*) and marginally *unless* take *necessary* and
*possible* as complements; the missing subject's interpretation derives from a
proposition in the matrix (CGEL, Ch. 14, §10(b), p. 1267, [4]). The passage
names only *necessary* and *possible*. Admitting *able* is attested (283
faces) but not stated by CGEL: the landing records it as an attestation-based
project decision carried by a declared Adjective feature, and STOPs for a
ruling if it needs more than that. *Possible* is not attested in the failing
units and is not added (pruning rule).

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Ares, God of War: "Ares attacks each combat if able."
- Jurin, Leading the Charge: "Jurin must be blocked if able."
- Tangle Angler: "{G}: Target creature blocks this creature this turn if able."
- Alluring Siren: "{T}: Target creature an opponent controls attacks you this
  turn if able."
- Hustle: "Target creature attacks or blocks this turn if able."

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
   Reading. *able* read as a depictive or Postmodifier of the Subject,
   or *if able* attached inside an NP, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Postpositive *able to* AdjPs (*All creatures with flying able to block this
  creature do so.*, Talruum Piper) and *do so*.
- *if possible*, *if necessary*: unattested in the failing units.

## Landing record

Completed 2026-10-08. Before: `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / covered **22,488**. After: `oqrvnvwrsoxvnzwkqmmnsktssltmomlt` / covered **22,669**. These stamps apply to every before/after figure below.
The claim parent is `oxomnkov`; its 1,697 census-source hashes exactly match
the complete census at the before stamp. `baseline-proof.json` records that
reuse. Both censuses use the same pinned input digest, supported identities,
and reminder-stripped analyzed sources. The implementation is
`swzpsuqukvvowmzwmrpwknszopmtxrto`; the measured after tree includes that
implementation. The pre-census refresh was a no-op.

### Analysis and authority

The Category of the Complement is **Verbless Clause**. Its **Predicate** is
the lexical Adjective *able*, with its Subject, verb and infinitival
Complement unexpressed. The adjective retains its declared infinitival frame
choice in the Reading. `VerblessPredicateUse = Yes` licenses this use;
`VerblessClauseComplement = Yes` licenses the preposition's Clause Complement.
The resulting PP has clause-final **Adjunct** function, including attachment
to finite predicates and embedded passive predicates. Coordination retains
its licensed scope alternatives. No generic Adjective Phrase, depictive,
NP Modifier, locative Complement or preposed Adjunct use is added.

CGEL, Ch. 14 §10(b), p. 1267 backs reducible and verbless Clause Complements,
and its [4] discusses *necessary* and *possible*. The extension to *able*
and this register's clause-final restriction are the attestation-based
project decision in this ticket's Analysis and Goal. They are not attributed
to CGEL. The claim-parent corpus has 252 analyzed faces containing *if able*,
all unread; the historical recon's broader *if able / able to* bucket remains
historical evidence. No vocabulary, form or overt infinitival frame was added
or removed. The existing finite-Clause route and the overt *able to* frame
are distinct from the new route; neither is superseded, so there is no route
to retire on either the grammar or lexicon side.

### PROVE: identities, structural laws and licensing

Face-by-face comparison covers **32,828** supported identities. There are
**181 gained faces**, **zero lost faces**, **zero removed Readings**, and
**zero count or digest changes on previously covered faces**. Every unchanged
face retains both its count and `readings_sha256`. `comparison.json` names
all changed identities and their before/after counts and digests.
There are no inherited Reading decreases to attribute to trunk.

| Census | Before | After |
| --- | ---: | ---: |
| No Reading | 10,340 | 10,159 |
| Unique Reading | 8,120 | 8,120 |
| Multiple Readings | 14,368 | 14,549 |
| Covered faces | 22,488 | 22,669 |
| Checked Readings | 938,466 | 956,021 |

The full run completely enumerates all supported faces: **zero limited or
failed enumerations, zero validation issues, zero internal failures, zero
duplicate Readings and zero cyclic derivations**. Every counted Reading passes
declaration admission, lexical ownership/context, byte-exact realization,
and construction/word traversal identity checks. The independently authored
conditional PP test checks admission, exact realization, whole-value
parse/render equality and lexical leaf order, including the adjective's frame
identity. Disabling either lexical permission or Adjunct function permission
rejects both parsing and independently constructed admission.

The full run was exactly:

```sh
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-verbless-if-complement/after.json
```

This was the **only full census**. Iteration used the explicit `--text-contains
'if able'` selector, selecting **344** raw faces. Its counts and digests match
the full run for every selected face (`subset-full-equivalence.json`).
No xtask tests, Lean checks, derived gate or post-refresh re-census ran.

New word-named licensing guards: **zero**, by diff review. The new guards read
the two declared lexical permissions and the declared function licence.
Lexical/Grammar Environment loading reports **zero fatal errors**; the
**87** source diagnostics and **nine** unsupported frame signatures are
identical to the parent (`inventory-comparison.json`). V3 does not emit the
retired licensing-checker total; no replacement checker was introduced.

### DISCLOSE: retained analyses and witnesses

All 181 newly covered identities are listed below with their Reading counts.
Their common new analysis is the PP headed by *if*, with the licensed
Verbless Clause Complement and its adjectival Predicate *able*. The PP uses
VP or clause Adjunct attachment; all admitted scope alternatives are retained.
The unique/multiple census above replaces the superseded specificity-selection
accounting. Existing covered faces keep their complete Reading sets.

| Witness | Localized witness Readings, before → after | Full-face Readings, before → after |
| --- | ---: | ---: |
| Ares, God of War | 0 → 4 | 0 → 8 |
| Jurin, Leading the Charge | 0 → 7 | 0 → 42 |
| Tangle Angler | 0 → 2 | 0 → 2 |
| Alluring Siren | 0 → 2 | 0 → 2 |
| Hustle | 0 → 10 | 0 → 10 |

Twelve additional newly covered faces were spot-checked from actual sampled
trees. Each has the common conditional PP analysis above, with the immediate
host a `PrepositionPredicate` (VP Adjunct) or `ClausalPreposition` (clause
Adjunct). Outer triggered/conditional clauses retain their own independent
initial PP. The samples preserve the adjective's lexical identity and frame;
none reads it as a depictive, Subject Postmodifier or direct NP-internal PP.
The complete inspected trees and host paths are in `spotchecks.json`.

| Spot-check | Full-face Readings | Conditional analysis |
| --- | ---: | --- |
| Phyrexian Juggernaut | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Reckless Cohort | 6 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Ashen Monstrosity | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Impetuous Devils | 6 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Bloodrock Cyclops | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Nacatl Hunt-Pride | 12 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Lust for War | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Culling Mark | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Legion Warboss | 8 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Magitek Scythe | 30 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Xantcha, Sleeper Agent | 8 | *if* + Verbless Clause (*able*); VP/clause Adjunct |
| Instigator | 4 | *if* + Verbless Clause (*able*); VP/clause Adjunct |

The residual item **Reduced if-able hosts**, taken over from
`english-v3-systemic-residuals`, is discharged by this construction. This
landing does not cover postpositive *able to*, *do so*, *if possible* or
*if necessary*; those remain outside this ticket's scope.

**Deviations and additions**:

- Added `VerblessClause` and `VerblessClauseComplementPreposition`, one
  Category and two lexical permission features. A clause without an overt
  verb is grammatically distinct from the existing finite-Clause Complement;
  the two routes do not duplicate a constituency on the same string.
- Added three companion tests beyond the five named witnesses: independently
  constructed roundtrip/traversal, lexical/function permission rejection,
  and exclusion of generic adjectival, nominal, locative and preposed uses.
  These verify the structural and invalid-Reading requirements.
- Added the glossary entries **Verbless Clause**, **Verbless Predicate Use**
  and **Verbless Clause Complement**, with their source/decision attribution.
  No glossary gap remains for the landing's new terms.
- Test counts: **restored 0, re-spelled 0, ignored 0, added 8, removed 0**.
  No existing test was deleted or weakened. The first positive run exposed
  an omitted frame identity in the test's authored expected value; the
  corrected complete value passes with the implementation unchanged.
- No contradiction with a recorded ruling arose. No STOP was required or
  remains unresolved. No plugin body, compiler or parser runtime changed.

### REPORT: inventories, checks and performance

At the before/after stamps above: **205 → 207 ordinary Constructions**,
**51 → 51 schemas**, **337 → 337 instance rows**, **796 → 798 compiled
Productions**, **149 → 150 Categories**, and **3,668 → 3,694 nonblank declaration
lines**. Ordinary Constructions plus schemas total **256 → 258**. Observed
Construction names are **218 → 220**; the new names are `VerblessClause` and
`VerblessClauseComplementPreposition`. These are provenance counts. The active
v3 record uses the census's covered count; the retired derived gate is absent.

The stamped trees have identical named inventories: **38,404 spellings,
1,139 homographs, 1,092 case-folded homographs, 23 form literals, zero
literal/vocabulary overlaps**. Full named lists are `spellings.json`,
`homographs.json`, `case-folded-homographs.json`, `form-literals.json`, and
`form-literal-vocabulary-overlaps.json`. `inventory-comparison.json` proves
that all five inventories and the existing diagnostics are unchanged.

- `cargo test -p deckmaste_english_v3`: **435 passed**, zero failed/ignored.
- `cargo test -p deckmaste_lexical_source`: **23 unit + 34 integration passed**,
  zero failed/ignored.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  **passed**.
- `cargo fmt -p deckmaste_english_v3 -p deckmaste_lexical_source -- --check`:
  **passed**.

- `cargo xtask cite check --list-noncompliant`: **zero noncompliant strings**.
- `cargo xtask cite check`: **16,311 citations checked, zero stale**.
- `jj --no-pager diff --git --from oxomnkov | cargo xtask cite audit --diff`:
  **passed**, zero changed Comprehensive Rules citation sites. No new rule
  citation requires blessing.

Performance advisory, with **12 workers** in both censuses:

| Stamped tree / covered | Host load (1 / 5 / 15 min) | Setup wall | Corpus wall | Checked-text thread CPU |
| --- | --- | ---: | ---: | ---: |
| `pkqkpttklumvrvyrqznxmuttmtnkmqnk` / 22,488 | 1.03173828125 / 2.09814453125 / 1.9404296875 | 2,276,800,773 ns | 101,737,986,666 ns | 411,796 ns/B |
| `oqrvnvwrsoxvnzwkqmmnsktssltmomlt` / 22,669 | 0.712890625 / 3.2890625 / 3.72607421875 | 2,258,207,185 ns | 101,746,289,340 ns | 421,106 ns/B |

Both corpus wall measurements exceed the quiet-host advisory ceiling of
**16,260,000,000 ns**. The after command's foreground wall time, including
Cargo and report emission, is **107,182,573,656 ns**. Host loads accompany
these observations; they are performance provenance.

All evidence lives under `target/english-v3/english-v3-verbless-if-complement/`.
It is preserved in the coordinator's ignored target directory before workspace
retirement. The final refresh log and post-refresh
source-equivalence proof accompany that evidence before integration.

### Newly covered identities

Every row starts with zero Readings on the before stamp and uses the common
conditional analysis disclosed above on the after stamp. `comparison.json`
also records each durable identity and complete analyzed source.

| Face | After Readings |
| --- | ---: |
| Academic Dispute | 16 |
| Aggravate | 11 |
| Akoum Firebird | 8 |
| Alexios, Deimos of Kosmos | 8 |
| Alluring Siren | 2 |
| Alpine Guide | 176 |
| Amarant Coral | 24 |
| Angler Turtle | 4 |
| Anje's Ravager | 12 |
| Anzrag, the Quake-Mole | 46 |
| Ares, God of War | 8 |
| Ashen Monstrosity | 4 |
| Auriok Siege Sled | 4 |
| Avalanche Tusker | 3 |
| Avatar of Slaughter | 14 |
| Barricade Breaker | 4 |
| Basandra, Battle Seraph | 12 |
| Battle-Mad Ronin | 4 |
| Berserkers of Blood Ridge | 4 |
| Bident of Thassa | 12 |
| Bloodcrazed Neonate | 12 |
| Bloodrock Cyclops | 4 |
| Bloodshed Fever | 4 |
| Boiling Blood | 4 |
| Boros Battleshaper | 236 |
| Bullwhip | 4 |
| Burning-Tree Bloodscale | 4 |
| Canopy Stalker | 28 |
| Captured by the Consulate | 4 |
| Chemister's Trick | 8 |
| Compelled Duel | 23 |
| Courtly Provocateur | 16 |
| Crashing Boars | 4 |
| Crazed Goblin | 4 |
| Culling Mark | 4 |
| Curse of the Nightly Hunt | 4 |
| Dauthi Slayer | 4 |
| Deadly Allure | 23 |
| Deathbellow Raider | 4 |
| Dulcet Sirens | 2 |
| Emberwilde Caliph | 4 |
| Emergent Growth | 23 |
| Enlarge | 42 |
| Fear of Being Hunted | 7 |
| Feral Contest | 2 |
| Fibrous Entangler | 14 |
| Flameborn Hellion | 4 |
| Flamewake Phoenix | 16 |
| Frontline Rebel | 4 |
| Fumiko the Lowblood | 4 |
| Furor of the Bitten | 8 |
| Furygale Flocking | 5,670 |
| Gaea's Protector | 7 |
| Galvanic Juggernaut | 24 |
| Giant Ambush Beetle | 12 |
| Goblin Assault | 32 |
| Goblin Brigand | 4 |
| Goblin Diplomats | 4 |
| Goblin Fire Fiend | 14 |
| Goblin Rabblemaster | 576 |
| Goblin Spymaster | 40 |
| Goldenhide Ox | 38 |
| Grand Melee | 16 |
| Grappling Hook | 12 |
| Grisly Anglerfish | 4 |
| Guise of Fire | 8 |
| Heckling Fiends | 4 |
| Hellraiser Goblin | 14 |
| Howlsquad Heavy | 32 |
| Hulk, Always Angry | 64 |
| Hulk, Brutal Brawler | 128 |
| Hunt Down | 2 |
| Hustle | 10 |
| Imaginary Threats | 8 |
| Impending Doom | 8 |
| Impetuous Devils | 6 |
| Impetuous Sunchaser | 4 |
| Imps' Taunt | 4 |
| Incite | 8 |
| Incite War | 8 |
| Incited Rabble | 8 |
| Inescapable Brute | 7 |
| Infectious Bloodlust | 216 |
| Insatiable Gorgers | 4 |
| Instigator | 4 |
| Into the Fray | 4 |
| Iron Golem | 10 |
| Irresistible Prey | 14 |
| Joraga Invocation | 23 |
| Juggernaut | 36 |
| Jurin, Leading the Charge | 42 |
| Kardur, Doomscourge | 156 |
| Khârn the Betrayer | 20 |
| King Harald's Revenge | 14 |
| Knight Rampager | 24 |
| Kookus | 28 |
| Kuldotha Ringleader | 4 |
| Legion Warboss | 8 |
| Loathsome Catoblepas | 42 |
| Lust for War | 4 |
| Mage-Ring Bully | 4 |
| Magitek Scythe | 30 |
| Manticore Eternal | 4 |
| Maraleaf Rider | 2 |
| Marauding Maulhorn | 6 |
| Mark for Death | 24 |
| Matsu-Tribe Decoy | 108 |
| Maximum Carnage | 8 |
| Mishra's Juggernaut | 4 |
| Mogis's Warhound | 32 |
| Monstrous Carabid | 4 |
| Monstrous Step | 4 |
| Nacatl Hunt-Pride | 12 |
| Nettling Curse | 4 |
| Oathsworn Knight | 720 |
| Oracle en-Vec | 320 |
| Otarian Juggernaut | 396 |
| Phyrexian Juggernaut | 4 |
| Phyrexian Snowcrusher | 8 |
| Predatory Impetus | 5 |
| Predatory Rampage | 8 |
| Primordial Ooze | 24 |
| Provoke | 4 |
| Public Enemy | 2 |
| Rage Nimbus | 4 |
| Rampant Elephant | 2 |
| Ramroller | 8 |
| Ravener | 3 |
| Razorgrass Screen | 4 |
| Reckless Brute | 4 |
| Reckless Cohort | 6 |
| Red Herring | 4 |
| Relentless Raptor | 10 |
| Rimehorn Aurochs | 12 |
| Riot Piker | 4 |
| Riveteers Decoy | 7 |
| Rubblebelt Recluse | 4 |
| Ruhan of the Fomori | 6 |
| Ruric Thar, the Unbowed | 36 |
| Sabertooth Alley Cat | 8 |
| Satyr Piper | 14 |
| Shessra, Death's Whisper | 6 |
| Shipwreck Singer | 8 |
| Silver Surfer, Galactus's Herald | 1,296 |
| Sizzling Soloist | 1,728 |
| Skeletal Swarming | 18 |
| Skin Invasion | 24 |
| Slayer's Cleaver | 34 |
| Spirespine | 32 |
| Sprinting Warbrute | 4 |
| Suicidal Charge | 8 |
| Tangle Angler | 2 |
| Tattermunge Maniac | 4 |
| Taunt | 3 |
| Tectonic Fiend | 4 |
| Thantis, the Warweaver | 4 |
| The Akroan War | 36 |
| The Brothers' War | 6 |
| Thran War Machine | 4 |
| Timely Interference | 12 |
| Tormentor's Trident | 8 |
| Toski, Bearer of Secrets | 12 |
| Tower Above | 3 |
| Trench Behemoth | 48 |
| Trumpeting Armodon | 2 |
| Turntimber Basilisk | 24 |
| Ulamog's Crusher | 4 |
| Uncontrollable Anger | 8 |
| Urborg Drake | 4 |
| Utvara Scalper | 4 |
| Valley Dasher | 4 |
| Vengeant Earth | 3,780 |
| Viashino Bey | 3 |
| Vinebred Brawler | 42 |
| Walking Desecration | 12 |
| War's Toll | 6 |
| Warmonger Hellkite | 8 |
| Watchdog | 8 |
| Wrathful Jailbreaker | 4 |
| Xantcha, Sleeper Agent | 8 |
| Zurgo Helmsmasher | 128 |
