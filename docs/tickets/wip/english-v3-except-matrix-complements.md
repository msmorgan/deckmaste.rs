---
needs: []
---
# Read the matrix-licensed Complements of *except*: can't be blocked except by Walls

## Why

*except* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *except* touches **268** unread faces and is the
sole cause on **152** (recon bucket "except"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Slivers can't be blocked by Slivers." 1; "Slivers
can't be blocked except by Slivers." 0; "Create a token that's a copy of target
creature you control." 1; "… , except it isn't legendary." 0; "Each player
sacrifices all lands they control." 1; "… except for three." 0.

## Goal

The preposition *except* heads a PP Adjunct whose Complement is the category
the matrix licenses: a PP (*except by creatures with flying or reach*), a *for*
PP (*except for three*) and a declarative content clause without *that*
(*except its name is Sunfrill Imitator and it has this ability*, *except it's
an artifact in addition to its other types*). Matrix licensing reads declared
features of the host, not of a named verb.

## Analysis

*Except* takes as its complement something licensed by features of the clause
containing it: NP, bare role NP, PP, AdjP, AdvP, infinitivals,
gerund-participial and content clauses are all attested (CGEL, Ch. 7, §5.1(g),
p. 642, [21]); CGEL calls these **matrix-licensed complements**. *Except* takes
either a *for* complement or an object (Ch. 7, §5.1(c), p. 640, [15i]). Only the PP, *for*
PP and declarative content clause are attested in the witnesses; add no other
complement category (pruning rule).

## Project analysis

The ticket's Goal requests declared host licensing for the Oracle nominal
and copy-description exceptions. This is project analysis, not a claim that
CGEL §5.1(g)'s directly-admissible matrix counterpart supplies either
“sacrifices … for three” or “Create … it isn't legendary”. CGEL §5.1(c),
p. 640 independently licenses a selected *for* PP under *except*. The passive
PP use follows the matrix-licensed principle together with the existing
internalised passive Complement distribution.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Evil Eye of Orms-by-Gore: "This creature can't be blocked except by Walls."
- Shifting Sliver: "Slivers can't be blocked except by Slivers." (control
  without *except* 1)
- Seeker: "Enchanted creature can't be blocked except by artifact creatures
  and/or white creatures."
- Quantum Misalignment: "Create a token that's a copy of target creature you
  control, except it isn't legendary."
- Keldon Firebombers: "When this creature enters, each player sacrifices all
  lands they control except for three."

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
   Reading. The content clause read as a Postmodifier of a noun, or
   *except* attached inside the preceding NP where the matrix licenses it, is a
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

- *except* governing a bare NP after *all … except* where no witness needs it.
- *but*, *save*, *including*.

## Landing record

### PROVE

Original claim-parent evidence is the full census stamped
`owmovnyynvrxuqynvupomwvtrqnmuutx` / **22,853 covered**. All 1,697 source
hashes match the original claim tree (`baseline-equivalence.json`). Refresh
imported gerund-complement work `lsxnwqlq` and the coordinated-property
follow-up `rwvlplnl`; its parent census is stamped `zpmurnquqlpkunplvxmymyztmkxlxypw` /
**23,005 covered**, with all 1,697 source hashes matching the
refreshed parent (`refreshed-parent-proof.json`). The finished feature census
is stamped `wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / **23,117 covered**.

All 32,828 supported faces are compared by identity, source hash,
Reading count and `readings_sha256`. Against the original claim parent:
**264 gains, zero lost faces, zero Reading decreases**.
Against the refreshed parent: **112 exception gains, zero
lost faces, zero Reading decreases, zero count or digest changes on previously
covered faces**. The 152 inherited gains are attributed to
`lsxnwqlq`; no decreases were imported. `comparison.json`,
`feature-comparison.json` and `trunk-comparison.json` retain every changed
identity. No re-coverage obligation is created.

| Census | Original claim parent | Refreshed parent | Finished feature |
| --- | ---: | ---: | ---: |
| No Reading | 9,975 | 9,823 | 9,711 |
| Unique Reading | 8,124 | 8,126 | 8,146 |
| Multiple Readings | 14,729 | 14,879 | 14,971 |
| Checked Readings | 999,360 | 1,131,832 | 1,144,241 |

Every counted Reading passes admission, lexical ownership/context, byte-exact
realization and construction/word traversal identity. There are zero limits,
failed enumerations, validation issues, internal failures, duplicate Readings
or cyclic derivations. The independent passive construction test verifies
whole-value parse/render equality. All grammatical
Readings remain; destructive selection and specificity arbitration are absent.
The selector agrees with the full census for every selected identity
(`full-validation.json`).

Exactly one full census ran on the finished feature:

```sh
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-except-matrix-complements/after.json
```

Iteration used `--text-contains except`; no other `--all` run, xtask tests,
Lean checks or derived gate ran. Forbidden word/card/lexeme/construction-named
licensing guards: **zero** by diff review; guards read declared features.
Environment loading has zero fatal errors. Unsupported frames and unmapped
sources remain unchanged, with complete named inventories below. V3 does not
emit the retired coverage-lock or licensing-checker totals; no legacy totals
are substituted.

### DISCLOSE

The outer PP keeps its Complement constituent. An internalised PP requires
an unfilled Bare Passive host (`ParticipialUse = BarePassive`,
`InternalisedComplementPresent = No`). A nominal exception requires
`MatrixComplementLicence = Nominal`; a declarative content clause requires
`MatrixComplementLicence = Content` and comma attachment. PP exports preserve
`MatrixComplementRequirement`; coordination agrees on that requirement and
intersects host licences. Generic adjunct routes require no matrix requirement.
No grammar guard names a lexical identity.

CGEL Ch. 7 §5.1(g), pp. 641–643 supplies the matrix-licensed general account;
§5.1(c), p. 640 supports the selected *for* PP. The nominal and copy-description
host permissions are project analysis requested by this ticket's Goal, not
CGEL's speech-verb example. This distinction is recorded in Project analysis
above and in the glossary.

Lexical owners with new host permissions: Create, Copy, Become (Content),
Sacrifice, Destroy, Return, Deal, Have (Nominal). Except declares matrix-governor
and content-clause distribution; For declares nominal-exception availability.
Every owner already exists in the supported corpus. No lexical owner, spelling
or frame is added or retired; verbs.ron and plugin definitions/bodies are untouched.

The existing bare-NP Except route is outside this ticket and is not superseded
by PP/content-clause complements. Removing it would lose Teferi's Ageless
Insight, Hullbreacher, Inspire Awe, Orcish Bowmasters, Chains of Mephistopheles
and Notion Thief. Its original lexicon declaration and grammar route remain,
with unchanged Reading counts and digests (`bare-np-retention.json`). No new
bare-NP distribution is added. ClauseComplementPreposition and its coordinated
counterpart are reused for content clauses; separate candidate matrix-clause
constructions were removed rather than retaining parallel constituency labels.
No final route is superseded or left unreachable on either side.

| Witness | Before | Finished full-face Readings | Analysis |
| --- | ---: | ---: | --- |
| Evil Eye of Orms-by-Gore | 0 | 2 | Except → By PP; unfilled passive Block host |
| Shifting Sliver | 0 | 1 | Except → By PP; unfilled passive Block host |
| Seeker | 0 | 3 | Except → By PP with coordinated NPs; unfilled passive Block host |
| Quantum Misalignment | 0 | 5 | Except → declarative content clause; Create host |
| Keldon Firebombers | 0 | 1 | Except → For PP → fused-head Cardinal; Sacrifice host |

The control “Slivers can't be blocked by Slivers.” retains all **nine Readings**;
the ticket's older probe counted admitted roots, a different measure. Every
passive witness Reading places the exception on the embedded passive host.
The content and nominal tests reject attachment inside the preceding NP; a
licence-removal negative rejects the unlicensed copy-description clause.

Ten newly covered sampled trees were inspected: Evil Eye of Orms-by-Gore,
Shifting Sliver, Seeker, Quantum Misalignment, Keldon Firebombers, Sunfrill
Imitator, Croaking Counterpart, Mageta the Lion, Whelming Wave and Pathrazer
of Ulamog. Their complete trees, lexical paths and Reading IDs are retained
in `spot-checks.json`, sampled selector reports and spot-check text files.
Refresh also enables Deluxe Dragster through the combination of its passive
exception and the imported Without gerund route; its inspected full tree is
`deluxe-dragster-spot.json`. The original ten audited Reading sets are
unchanged after refresh (`spot-refresh-proof.json`).
The last five respectively use Become with coordinated content clauses,
Create with a content clause, Destroy with a For PP, Return with a For PP,
and Block with a By PP. No inspected exception is a noun Postmodifier.

Deviations and additions:

- Three ordinary Constructions and one shared schema are added:
  MatrixComplementPredicate (embedded passive attachment),
  MatrixComplementPreposition (PP Complement constituent),
  QuantitativeExceptionPreposition (Cardinal inside the selected For PP),
  LicensedMatrixComplementPredicate (declared nominal/content hosts).
  The numeric construction is necessary for “for three” and does not grant
  independent purpose, duration or nominal-modifier distribution.
- Generic clause-Complement constructions gain requirement propagation;
  existing host and coordination policies propagate declared licences.
  These are required to preserve scope through auxiliary and catenative hosts.
- Refresh combines the imported complement-function restriction with matrix
  requirements; the new PPs explicitly export its neutral value.
- Eight integration tests are added under tests/suite and registered in main.
  Assurance counts: restored 0, re-spelled 0, ignored 0, added 8, removed 0.
- Glossary gaps are filled: Matrix-licensed Complement, Matrix Complement
  Licence, Matrix Complement Requirement, Matrix Complement Governor and
  Nominal Exception Complement Use. Project terms are marked as such.

No unresolved STOP or recorded-ruling contradiction remains. An initial
citation reading error was corrected against the actual CGEL text; the
correct original “either” statement was retained. No new CR citation is
introduced or changed, so no citation registration is required.

Every newly covered identity is named below with its selected exception
analysis. Inherited gains follow in a separate table with their gerund
analysis; all grammatical alternatives are retained.

| New exception face / identity | Readings | Analysis |
| --- | ---: | --- |
| Absorbing Man (`ec8e310f-e871-498f-8d80-c3617c5e1433#card`) | 696 | Except → content clause; declared content host |
| Aggressive Biomancy (`9080a7bb-2b8b-42af-9f5b-a9f374ecb801#card`) | 10 | Except → content clause; declared content host |
| Amrou Seekers (`396efb56-7a54-4d91-b2ce-7f3c62c3238b#card`) | 3 | Except → PP; embedded passive host |
| Anikthea, Hand of Erebos (`991fd651-0cba-41ee-855a-d01a1a7c8f60#card`) | 714 | Except → content clause; declared content host |
| Applied Geometry (`88fc179c-055b-47c2-b830-1fd2b1377147#card`) | 84 | Except → content clause; declared content host |
| Ardyn, the Usurper (`9ae13026-960a-4d31-b775-d47209a1e313#card`) | 12 | Except → content clause; declared content host |
| Artisan of Forms (`3e8abec7-1e18-460a-9d88-30969755ebd2#card`) | 4 | Except → content clause; declared content host |
| Astral Dragon (`a20e2e1a-90d8-4073-96e9-efce86f0178e#card`) | 50 | Except → content clause; declared content host |
| Benthic Anomaly (`c12e2563-4ad4-4637-b9ae-72f23cca31d8#card`) | 32 | Except → content clause; declared content host |
| Canopy Cover (`5b84101e-7e23-437d-835c-409bc061ecbb#card`) | 30 | Except → PP; embedded passive host |
| Cemetery Puca (`7e93ea61-bc16-4d22-a5e5-e6ba0f7ae14f#card`) | 1 | Except → content clause; declared content host |
| Cloak of Invisibility (`68b06917-7f83-47d4-a466-4e6698cae56d#card`) | 1 | Except → PP; embedded passive host |
| Croaking Counterpart (`142b2f24-49f6-4ef8-9731-9ae2e513f796#card`) | 2 | Except → content clause; declared content host |
| Cryptoplasm (`e7bef318-2720-45bc-b663-aafd724bc511#card`) | 1 | Except → content clause; declared content host |
| Crystalline Resonance (`93255c68-2ed1-499b-82c6-672823451ce9#card`) | 4 | Except → content clause; declared content host |
| Cursed Mirror (`4d67e2a7-4aa7-44cc-853b-500d7aac046d#card`) | 9 | Except → content clause; declared content host |
| Cyclone Summoner (`2088e791-9ce5-4e97-a4ce-f51adaa3f42b#card`) | 4 | Except → For PP; declared nominal host |
| Deathcult Rogue (`fa8ae8ac-4ad2-4cda-9f6b-4ec634526658#card`) | 1 | Except → PP; embedded passive host |
| Deluxe Dragster (`56ba8fe5-8693-4783-8406-9ced99de0b2a#card`) | 540 | Except → PP; embedded passive host |
| Departed Deckhand (`a620a765-97ba-4687-acfd-4dec7da75d9f#card`) | 2 | Except → PP; embedded passive host |
| Dimir Doppelganger (`1916f120-4418-404a-aed5-b95ebf60d3a3#card`) | 4 | Except → content clause; declared content host |
| Donal, Herald of Wings (`1c9f2b6e-a0eb-493e-8c43-3a14f6d95a53#card`) | 152 | Except → content clause; declared content host |
| Double Major (`ece44a82-dcf0-4439-bdd9-a09c99a6f159#card`) | 6 | Except → content clause; declared content host |
| Dread Charge (`6b7933b3-821a-4d64-9a10-af8a69fa008e#card`) | 2 | Except → PP; embedded passive host |
| Dread Warlock (`5791710d-c718-4f64-9c14-3668154716f1#card`) | 1 | Except → PP; embedded passive host |
| Echoing Equation (`9110339d-72ba-4132-801f-cd2fd738b71d#face:1`) | 4 | Except → content clause; declared content host |
| Electroduplicate (`112f2b3f-32e7-40b1-b80e-0b99184a840c#card`) | 5 | Except → content clause; declared content host |
| Elven Riders (`9a6ff511-3c8f-446e-8cad-b8997a753958#card`) | 11 | Except → PP; embedded passive host |
| Evil Eye of Orms-by-Gore (`3f79780a-accd-4782-94a8-a6e71fb3ada7#card`) | 2 | Except → PP; embedded passive host |
| Face Yourself (`3c0fe71c-bee1-41d3-aeb5-1723028eedde#card`) | 2 | Except → content clause; declared content host |
| Feldon of the Third Path (`be094704-5232-4b10-beeb-aa9bbfae063d#card`) | 100 | Except → content clause; declared content host |
| Felhide Spiritbinder (`91e3f057-c24f-42b5-8525-74b44bdefc83#card`) | 48 | Except → content clause; declared content host |
| Firefright Mage (`9a0bb238-f832-4b37-97ae-520899798e7a#card`) | 3 | Except → PP; embedded passive host |
| Flame Sweep (`0b34f471-40da-4c7a-81df-245fc3d14092#card`) | 4 | Except → For PP; declared nominal host |
| Gingerbrute (`10b8d4c7-7553-4d76-b643-d98b80701e13#card`) | 5 | Except → PP; embedded passive host |
| God-Pharaoh's Gift (`9f2d2fda-f098-4375-8fe9-63f929e3928c#card`) | 24 | Except → content clause; declared content host |
| Gorilla Berserkers (`6373c66d-edfc-44cb-af2a-2273e0ba53e0#card`) | 1 | Except → PP; embedded passive host |
| Hashaton, Scarab's Fist (`db266661-f783-4907-9e52-6963eec05431#card`) | 5 | Except → content clause; declared content host |
| Heat Shimmer (`5c80d39e-f3af-4d24-bca0-19f0bd0f5082#card`) | 2 | Except → content clause; declared content host |
| Helm of the Host (`83b43aba-bf9c-4da2-967d-9daa632e97d2#card`) | 2 | Except → content clause; declared content host |
| Hexmark Destroyer (`b3329240-c812-45ea-a7f8-4d79cd4ed515#card`) | 1 | Except → PP; embedded passive host |
| Hour of Eternity (`6fbaac3b-37ec-4815-a4ea-544c547e90b1#card`) | 10 | Except → content clause; declared content host |
| Impossible Man (`9be1d4b1-064d-4051-aafc-ca48def28e3b#card`) | 4 | Except → content clause; declared content host |
| Infiltrator's Magemark (`df82b67f-f685-4278-a108-f3689a3a121b#card`) | 6 | Except → PP; embedded passive host |
| Invisibility (`de26b0c6-dfb7-45a8-9d7f-f8d45522d675#card`) | 1 | Except → PP; embedded passive host |
| Irenicus's Vile Duplication (`a3faf698-1dac-45ef-9c43-c8cead489ffc#card`) | 10 | Except → content clause; declared content host |
| Irma, Part-Time Mutant (`deda54da-4fcc-4da3-9337-7768a2db3283#card`) | 12 | Except → content clause; declared content host |
| Iron Man, Bleeding Edge (`fc6e170c-820c-48bb-95c7-ee79fba634de#card`) | 4 | Except → content clause; declared content host |
| Joven's Tools (`b1935140-b038-4a81-98f7-28d2c137124a#card`) | 1 | Except → PP; embedded passive host |
| Keldon Firebombers (`d5e02fa5-827e-414b-acb5-932de4bb3ee3#card`) | 1 | Except → For PP; declared nominal host |
| Kiki-Jiki, Mirror Breaker (`a34b7416-cfe3-4a1e-a8c1-a3056b747519#card`) | 14 | Except → content clause; declared content host |
| Lazav, the Multifarious (`c14bcef2-6d49-4430-86d0-e5a87ba442d9#card`) | 672 | Except → content clause; declared content host |
| Mageta the Lion (`cd170b1c-879b-414e-9f0e-5e7fb099e7bb#card`) | 1 | Except → For PP; declared nominal host |
| Manta Ray (`d5129531-e4b6-454e-9c67-dae925c8f2ee#card`) | 3 | Except → PP; embedded passive host |
| Midnight Pathlighter (`e49228d6-6488-4cbc-aa53-303afd3a549b#card`) | 3 | Except → PP; embedded passive host |
| Mirror of the Forebears (`9eaeb54b-73a8-43cc-b4d8-ed8b5eb19646#card`) | 18 | Except → content clause; declared content host |
| Mizzium Transreliquat (`a1f73421-cda5-4029-9bcf-f2baee179e5c#card`) | 4 | Except → content clause; declared content host |
| Molten Duplication (`92b91910-9b65-4421-8eef-a33793161089#card`) | 280 | Except → content clause; declared content host |
| Muddle, the Ever-Changing (`9a8f6e5e-6cde-49df-861c-a26cb09c9b12#card`) | 72 | Except → content clause; declared content host |
| Multiversal Recruitment (`8c65d5bd-5d90-4e84-8251-cfef21dd6332#card`) | 5 | Except → content clause; declared content host |
| Nanogene Conversion (`c06d1127-a269-453f-9c06-e9c424998eb7#card`) | 3 | Except → content clause; declared content host |
| Nick Valentine, Private Eye (`5663288a-7300-4e1b-8fb0-43c467709fe5#card`) | 2 | Except → PP; embedded passive host |
| Noggle Bandit (`3e7707df-54e3-4307-9695-2de4e3edcf89#card`) | 5 | Except → PP; embedded passive host |
| Orchard Spirit (`b6eb56a7-4dfe-4d51-a1df-55f892c1b014#card`) | 5 | Except → PP; embedded passive host |
| Pathrazer of Ulamog (`4bf95747-4572-49b8-b892-87fe7f910252#card`) | 1 | Except → PP; embedded passive host |
| Phyrexian Colossus (`caab555c-7852-418a-bdc3-f908af7e0932#card`) | 6 | Except → PP; embedded passive host |
| Protean Thaumaturge (`148331cc-24de-481f-9131-c3089d1ff7ce#card`) | 2 | Except → content clause; declared content host |
| Prowler's Helm (`603ef53f-6191-4229-b85f-adad6f4503ce#card`) | 1 | Except → PP; embedded passive host |
| Prowling Nightstalker (`6279078a-5c77-4a9e-8e9b-e99310363b6f#card`) | 1 | Except → PP; embedded passive host |
| Quantum Misalignment (`197582b9-4c86-41a3-ad0c-789a0db8e087#card`) | 5 | Except → content clause; declared content host |
| Rampaging Ceratops (`91d01119-9b3e-4629-8cf0-bbca2c19a7cc#card`) | 1 | Except → PP; embedded passive host |
| Rebuild the City (`2aac0ddc-1005-4049-a337-fe86e092b461#card`) | 28 | Except → content clause; declared content host |
| Reflection of Kiki-Jiki (`c0957e5e-c71b-439c-931c-9f55d2f76ace#face:1`) | 18 | Except → content clause; declared content host |
| Relentless X-ATM092 (`50994df7-300d-4e30-897a-0974dc0820ee#card`) | 19 | Except → PP; embedded passive host |
| Resilient Roadrunner (`78c7c73f-847d-43bf-9e02-cf85357453f4#card`) | 10 | Except → PP; embedded passive host |
| Rime Transfusion (`d7ff0525-8617-4e0f-92db-660f67d706ca#card`) | 1 | Except → PP; embedded passive host |
| Saheeli, the Sun's Brilliance (`5aa76c5d-8046-42ce-b61d-cc9b4806c11a#card`) | 160 | Except → content clause; declared content host |
| Sarkhan, Soul Aflame (`bbcae463-ba05-4cfe-9c25-13f2e64698c7#card`) | 1,040 | Except → content clause; declared content host |
| Scourglass (`78541a40-c27a-49c0-aa3e-5a48b97065f0#card`) | 4 | Except → For PP; declared nominal host |
| Seeker (`ca00eb17-e5c3-42c8-a665-431f5f95b67f#card`) | 3 | Except → PP; embedded passive host |
| Serpent of Yawning Depths (`d31c6043-e005-4efe-9aa7-d82cb122eebe#card`) | 6 | Except → PP; embedded passive host |
| Shadow Kin (`7ecc506e-bed0-4ae2-9d2f-5143ef63f62c#card`) | 21 | Except → content clause; declared content host |
| Shifting Sliver (`6a262f53-2eea-45d0-8189-9222d66f1b52#card`) | 1 | Except → PP; embedded passive host |
| Signal Pest (`724f3d2f-0513-4efd-961f-56a7ea7b5731#card`) | 5 | Except → PP; embedded passive host |
| Silhana Ledgewalker (`5b15dcac-38dc-4957-bc01-40153ea07559#card`) | 5 | Except → PP; embedded passive host |
| Skirk Shaman (`78d5ce34-f160-4925-a4ce-25a3546d97c7#card`) | 3 | Except → PP; embedded passive host |
| Slinn Voda, the Rising Deep (`753d98d3-d534-42c1-b728-a4f14c7f29ec#card`) | 2 | Except → For PP; declared nominal host |
| Sonorous Howlbonder (`89645420-1a14-46bf-b27d-966553a95463#card`) | 2 | Except → PP; embedded passive host |
| Speed, Young Avenger (`2fb0f2d2-ed1f-4181-91f2-684a71c34998#card`) | 12 | Except → PP; embedded passive host |
| Spire Tracer (`9b38ab81-7ee7-4d89-878b-3507e44836ca#card`) | 5 | Except → PP; embedded passive host |
| Splinter Twin (`2df3187b-4b91-4de9-9b26-d48a30700cec#card`) | 4 | Except → content clause; declared content host |
| Storm of Saruman (`cf5f4860-e805-46a3-9352-a2c583e33403#card`) | 20 | Except → content clause; declared content host |
| Sunfrill Imitator (`2f618a4a-8b76-4c87-baa7-fb6a17f6f51c#card`) | 9 | Except → content clause; declared content host |
| Séance (`a375b935-c02b-44bc-a942-37c5de58b1e4#card`) | 144 | Except → content clause; declared content host |
| Taskmaster, Mercenary Mimic (`dcd97730-b793-4c0c-b158-625ac3656b4a#card`) | 640 | Except → content clause; declared content host |
| Tawnos, the Toymaker (`48cce741-cfbd-4ed9-a7ac-38880867a286#card`) | 52 | Except → content clause; declared content host |
| Tempestra, Dame of Games (`12401ad1-b5c9-43dc-9389-04aaf5db8b26#card`) | 14 | Except → content clause; declared content host |
| The Argent Etchings (`650ed75c-20b0-45b8-a4d4-813e0369aaf1#face:1`) | 96 | Except → For PP; declared nominal host |
| The Balrog, Durin's Bane (`d113af33-d75d-44e4-a8dc-31f8984edc42#card`) | 48 | Except → PP; embedded passive host |
| The Cloning of Shredder (`bd768e8f-8f04-487e-85e9-2828d6fd8745#card`) | 6,048 | Except → content clause; declared content host |
| The Ever-Changing 'Dane (`bda0468f-1c72-4d3d-ad94-15d446c75313#card`) | 1 | Except → content clause; declared content host |
| The Scarab God (`c75e55f2-fd6c-4816-9d96-21eeb8369aff#card`) | 96 | Except → content clause; declared content host |
| Thespian's Stage (`b01e698b-608a-4fc7-8073-b01d044743ec#card`) | 1 | Except → content clause; declared content host |
| Tin Street Dodger (`fde3c8df-cc57-4e64-b78b-301e103d8487#card`) | 5 | Except → PP; embedded passive host |
| Treetop Bracers (`ba71eb8a-f9f8-4608-88d8-963a1770d8d4#card`) | 6 | Except → PP; embedded passive host |
| Treetop Rangers (`e4606bc2-faaa-4dd8-9109-55ce3440cf5b#card`) | 5 | Except → PP; embedded passive host |
| Treetop Scout (`f62d4910-4547-43f6-909c-fc6a86ecc922#card`) | 5 | Except → PP; embedded passive host |
| Underworld Cerberus (`a36c2b76-d595-42ba-b8c2-bb8f02639981#card`) | 8 | Except → PP; embedded passive host |
| Unstable Shapeshifter (`6a71deb3-6659-47ea-8687-136cf75fa381#card`) | 1 | Except → content clause; declared content host |
| Urza, Prince of Kroog (`6a448862-e8d8-4a08-8e3d-5460afa29b4d#card`) | 120 | Except → content clause; declared content host |
| Varchild's Crusader (`45351d9d-453f-4245-84c0-053eb8aca345#card`) | 2 | Except → PP; embedded passive host |
| Whelming Wave (`e510eaaf-6497-480f-baa8-f4796b5f1086#card`) | 2 | Except → For PP; declared nominal host |

| Inherited face / identity (`lsxnwqlq`) | Readings | Analysis |
| --- | ---: | --- |
| Arcane Bombardment (`1e36a42f-58e2-49bc-8c14-7f75e1d8ef98#card`) | 2,160 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Arcane Heist (`839748e7-ccb4-421e-9626-b6d8be9390ab#card`) | 126 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Assert Authority (`085bc7be-bd44-40af-8e8f-a1a8006fe22c#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| Audacious Swap (`40b0b93d-4075-42d6-bcd8-b2f9910cc7d2#card`) | 16 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Baral's Expertise (`a1cdcb8c-1e12-4d39-aa4a-db678a80df9c#card`) | 38 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Bre of Clan Stoutarm (`59384481-3ca0-49dd-b94f-d3680b336f1e#card`) | 6,944 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Breaching Dragonstorm (`5370ee80-0354-454f-a16e-ec133e432ecb#card`) | 207 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Caravan Vigil (`925a3ded-b7bc-4384-8684-79c20c59ab12#card`) | 266 | Compound PP → Of → gerund-participial Complement |
| Cho-Arrim Legate (`780c9cb2-9d98-4789-87e0-cb81db477204#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Daring Waverider (`e4d52559-7624-48b2-96ba-e52ada6c507a#card`) | 378 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Deep Gnome Terramancer (`d4e3440d-4e34-40d7-8a42-c673225c0332#card`) | 44 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Deepwood Legate (`414eb407-dd39-41d2-bf25-a7a77ff35cb4#card`) | 8 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Deny Existence (`b665a64b-4772-42dd-9fd2-fd8598e689ba#card`) | 6 | Compound PP → Of → gerund-participial Complement |
| Deny the Divine (`0146de73-29bf-415a-b450-11074553f715#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| Descendants' Path (`ba13cbdb-8043-4c1b-8a85-084c86886ac8#card`) | 280 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Devious Cover-Up (`feb221fb-59bf-4671-a53f-1bbe8e9c2ca9#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| Diluvian Primordial (`f21ce158-2925-4658-ab7b-b73718d31965#card`) | 300 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Dissipate (`4b2e9aa9-6f91-41de-84b6-e9a89be06a53#card`) | 6 | Compound PP → Of → gerund-participial Complement |
| Diviner of Mist (`abf4dcd8-5176-4f4a-b7c1-5ce24ff181bf#card`) | 312 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Djinn of Wishes (`a34c3012-f787-4998-93c7-89c4decaa1ba#card`) | 30 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Dour Port-Mage (`cf58e309-00e8-438e-813e-2e1c1002db23#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Dracogenesis (`9b06f2bf-146d-4b30-b467-e16f1a942253#card`) | 3 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Dream Harvest (`70b9497f-0dd6-4b66-9675-57c7041fe3c0#card`) | 1,584 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Efreet Flamepainter (`fb5e8b8c-bad2-45bb-abb4-2ed454525749#card`) | 360 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Electrodominance (`4a59d86e-405d-4c70-b2a5-c64f5cecfd9e#card`) | 19 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Emergent Ultimatum (`0eecdfb3-3b05-4051-a660-060ff6df80ef#card`) | 78 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Epic Experiment (`08bc10fd-8901-4836-ad4d-a867e1bcba77#card`) | 93 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Epistolary Librarian (`cb4f65b0-2841-42bf-8e18-d04a3b9f1a76#card`) | 34 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Etali, Primal Storm (`078def07-ae5d-4591-8db6-d156834aab97#card`) | 516 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Evercoat Ursine (`65c32889-3d70-449f-af76-05385c78c5a9#card`) | 30 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Extract Brain (`bb0eb9c2-c26d-452d-9e02-fc0bec2655c4#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Eye of the Storm (`01f8db27-1191-488e-b0d5-172821613c15#card`) | 232 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Faerie Trickery (`bbe370ba-412b-4052-89d9-2d0ae4928118#card`) | 6 | Compound PP → Of → gerund-participial Complement |
| Fallen Shinobi (`4fc5f871-a809-4a4a-adfb-eb4cd52837c8#card`) | 36 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Feather, the Redeemed (`aa219936-661b-4ccb-8741-78b70cff2b1a#card`) | 756 | Compound PP → Of → gerund-participial Complement |
| Fevered Suspicion (`51545f6e-d0e3-4b4c-bc73-c731f85e26b0#card`) | 192 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Flawless Forgery (`0e3cb267-5704-4708-94c8-e2fbce4978af#card`) | 42 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Founding the Third Path (`0eff910a-712d-4652-8b3b-9eddf63eb891#card`) | 693 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Galvanoth (`0b4fa1ba-9597-4db1-a77c-975ed839f57f#card`) | 36 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Gix, Yawgmoth Praetor (`928d977e-cff0-4e0e-83bb-16d73a754f35#card`) | 1,368 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Glorious Purpose (`182baad0-ec6e-4e93-9dfc-7c80b37005de#card`) | 252 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Goblin Dark-Dwellers (`bab0ab8d-74d2-49a3-b258-b93d19925d99#card`) | 378 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Golos, Tireless Pilgrim (`efe9c3e0-7315-408e-9903-9d25a681039b#card`) | 660 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Great Intelligence's Plan (`cb9abc41-07f8-4a06-b7b9-9dd6be526e42#card`) | 14 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Halo Forager (`210d1077-6e60-4f06-a8a4-10b842979ac5#card`) | 378 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Hazoret's Undying Fury (`af4d3489-6bba-4538-928c-50c9a7073813#card`) | 372 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Hellcarver Demon (`90514aa5-84b6-4be7-b6d0-b05529c97140#card`) | 3,294 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Herald of Amity (`6c25c6b5-d98c-4eb4-aacd-b6960a8779a9#card`) | 216 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Hidetsugu and Kairi (`18591d9e-c9f5-4ae8-aee1-8580bc8fa600#card`) | 48 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Horde of Notions (`a7656948-4d7d-47f7-b8a1-fc9716155e37#card`) | 12 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Horribly Awry (`9cac23a1-a0b3-490c-aea2-dc2928f6dc9e#card`) | 24 | Compound PP → Of → gerund-participial Complement |
| Idol of Endurance (`6b946317-8a66-41b7-b902-308784d69b90#card`) | 7,750 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Ignite the Future (`915aa871-4c5e-4799-9a07-d7b855df4e7a#card`) | 42 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Imperial Cosmographer (`b2aa052c-08b7-4334-9ea7-e78da57ff9ea#card`) | 1 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Improvisation Capstone (`fd4f0315-567a-4cc5-bc7e-88a9d2cee910#card`) | 876 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Impulsivity (`02b6900b-219f-4832-8f93-6ef27ee76c0c#card`) | 120 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Invasion of Alara (`8faa4330-07bb-4b65-810a-bbce29d3cd42#face:0`) | 816 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Invoke Calamity (`5b3f041e-ad4a-47ea-bdc4-1be2353f2e18#card`) | 93 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Izzet Chemister (`e4a3d6f3-36ba-4e6d-a254-351bc8886f20#card`) | 162 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Jace's Mindseeker (`8ebf337d-eac0-40de-978c-3a8fc3bbe03c#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Jeleva, Nephalia's Scourge (`a014f283-c531-415c-ac00-e6773ea5d64d#card`) | 2,460 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Jetsam (`b0c28b2b-a2dd-4b76-bb1c-cec55a0a6784#face:1`) | 54 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Kaho, Minamo Historian (`395a7bd6-7c2d-448e-842c-ca53256d7008#card`) | 1,140 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Kari Zev's Expertise (`e0de0121-6185-45c5-9d88-a415b07347e0#card`) | 190 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Kefka, Dancing Mad (`fe5690d3-547b-4ce9-8e94-77fdc0e9c5c6#card`) | 4,392 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Knowledge Exploitation (`61ad3299-474a-400f-a1c1-b86024aca3e8#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Knowledge Pool (`cb53bd72-dee8-46fa-b0f9-1eee3a99379a#card`) | 2,004 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Kotis, the Fangkeeper (`2eb2f6a2-e31a-4d3e-93ce-100394899014#card`) | 1,116 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Kyren Legate (`41b46aa5-6091-4bc4-8d17-d2c6b5fe1c5e#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Leaf-Crowned Elder (`9416fd24-15a2-46d0-8b5f-55f5e7b65465#card`) | 64 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Liquify (`dc9ae094-139f-4f28-9d4e-4d6def765744#card`) | 18 | Compound PP → Of → gerund-participial Complement |
| Lobelia, Defender of Bag End (`0f2e3e2e-9857-4be5-9bf0-62da1cc147fc#card`) | 3,344 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Maelstrom Archangel (`ac50ef98-1791-4dd4-9c1f-8cea7db7ade5#card`) | 42 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Massacre (`ce1866e9-28de-4bcc-9999-11958e6c13a6#card`) | 8 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Memory Plunder (`36b67014-0507-4976-ad52-a2a36676197d#card`) | 42 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mind's Desire (`b3739b5a-5731-4c64-a244-815a363b0d5c#card`) | 8 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mind's Dilation (`d4b5e09b-9ca2-4b51-a4f8-080741810df8#card`) | 16 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mindclaw Shaman (`018985f5-6a67-4f2f-855c-efa7254ad3f8#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mindleech Mass (`facc2d6f-3f28-4b87-b643-44fdf939a2b5#card`) | 42 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mizzix's Mastery (`40362fe0-a1a9-4d76-8c35-eac474b91af5#card`) | 90 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mnemonic Deluge (`56de4cdc-4db0-48ec-8a16-f66086507233#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mogg Salvage (`0923ad85-2ea4-4463-8f71-b7d0a162b486#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Mysterious Stranger (`4cd2e1c2-5f14-4ac9-9c88-899af6ad9f80#card`) | 4,680 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Nahiri, Forged in Fury (`87966759-206a-4a2d-bf78-86f727c29972#card`) | 80 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Narset, Enlightened Master (`aeaf7768-ed67-487d-8506-e667babb88ba#card`) | 162 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| No Escape (`7c6e0198-edd8-42b8-ba1b-549e1713d188#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| No More Lies (`ead1ee6a-e0da-46d5-89e8-31e8c0270bc2#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| Omen Machine (`dbf05c40-7274-4aaa-94d7-1f28cbe15e09#card`) | 36 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Omniscience (`730e39e6-c61d-48b5-8827-bfd952bf1be7#card`) | 9 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Omnispell Adept (`b575449b-591d-4357-92b7-55137fc7966a#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Oracle's Vault (`220e3298-1663-4731-b50f-247076e1328b#card`) | 112 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Panoptic Mirror (`505cb78d-b292-4d16-b3f2-110164b4cc93#card`) | 1,728 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Patrician's Scorn (`0dc9acfb-3f8b-4a34-b44c-5cc62689904b#card`) | 8 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Perception Bobblehead (`f0bcdc43-c43d-4ce7-a283-b8dcab4fc497#card`) | 114 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Plargg and Nassari (`9e769a65-7aa5-484d-9493-badb485b5964#card`) | 5,428 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Power Pack (`3d5c98ff-fcf1-42bd-9533-2648791a4f45#card`) | 2,664 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Powerbalance (`8be368dc-6212-4627-989e-077eea78f30d#card`) | 28 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Refreshing Rain (`c6f69168-71c2-41a2-ab95-cfd7baf0ba68#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Reject (`198fed4c-297e-41c0-a172-e471c7401fbb#card`) | 24 | Compound PP → Of → gerund-participial Complement |
| Renegade Bull (`61747805-0c34-4beb-8cbf-97bac7a998d0#card`) | 1,188 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Reversal of Fortune (`938ba053-35f6-4424-a978-bf915709572c#card`) | 44 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Rishkar's Expertise (`97407cd0-2bd2-4074-94d3-4ec3d243fa78#card`) | 171 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| River Song's Diary (`60a63eb4-b0f7-4930-9bf6-afd42261d4d0#card`) | 3,024 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Rod of Absorption (`d32345c1-ed20-4bfc-a5fa-6ac7b99542ec#card`) | 6,858 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Rona, Tolarian Obliterator (`cf5bd3a4-3899-4c25-ad1f-f1778c1d9c60#face:1`) | 1,080 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Rushwood Legate (`f3623759-bf3a-4846-9ef7-1b732ad4355f#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Saprazzan Legate (`d20b62fc-e248-4c84-87ac-7319b92531d4#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Scarlet Witch, Chaotic Avenger (`8c72d54e-97a6-408e-ae77-18d3c96b3bd8#card`) | 40,590 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Scholar of the Lost Trove (`5f1e9098-f554-4505-974b-cef4b4b7b23d#card`) | 600 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Seifer Almasy (`7088a901-0489-41c3-9f2f-633939514de8#card`) | 3,402 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Shell of the Last Kappa (`a8b24b38-019c-45e7-a5f4-a1bc3014f7a2#card`) | 270 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Shiko, Paragon of the Way (`d3bdd3f2-9e38-436a-93a3-f87d9f4d4f86#card`) | 60 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Silent-Blade Oni (`4f515250-0f1f-4fb7-aa75-b703b81ccd4a#card`) | 27 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sins of the Past (`9059a940-ac28-4322-a11b-af1d107b2edd#card`) | 120 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sivvi's Ruse (`c5edfaae-3ecd-41ad-970f-8cecb09bf1e4#card`) | 88 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Solstice Revelations (`310c1280-dee8-426f-8440-6132a634a1e0#card`) | 576 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sorcerous Squall (`a986d83d-22f1-45c3-bc2d-f5b210c539fb#card`) | 180 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Spawnsire of Ulamog (`b90d2f4d-b4ea-40af-aea0-1ab4234ab80f#card`) | 104 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Spell Shrivel (`bfaf376f-90f9-45d9-bfbe-dd84a2a4688b#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| Spellshift (`640fc475-5708-4ee6-a1a9-8f0722e22b48#card`) | 252 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Spelltwine (`307e68e0-5095-474c-af02-bfcb2e463763#card`) | 1,005 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Spellweaver Volute (`3d92e5ce-9d30-4203-9f84-6c36117440cf#card`) | 204 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sram's Expertise (`5cd17347-5558-46f9-b8bb-ae5deeb1f229#card`) | 19 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Stolen Goods (`9294d4d3-0df0-4070-9261-06f15b758a0d#card`) | 64 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Storm of Memories (`6f8ca795-d6fd-4e3e-911c-c621a942acbb#card`) | 369 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Submerge (`99427ebe-c00d-4206-84ca-9764f6e952c6#card`) | 4 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sunbird's Invocation (`10d482fd-e034-4187-b148-51fe15885360#card`) | 708 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sunforger (`d1421070-a3cc-4af3-bf91-0f97372f4161#card`) | 99 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Surtland Elementalist (`b1a3b2ab-d9f8-4a0a-b2e2-aaa9989b2e41#card`) | 480 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Sword of Once and Future (`1e0cf860-6be6-4b4b-9a67-0c19b6018ae1#card`) | 1,134 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Syncopate (`318f7f70-e374-40ef-8afb-3389c10461d8#card`) | 12 | Compound PP → Of → gerund-participial Complement |
| The Dawning Archaic (`a3e10b9b-9349-4b44-a46c-c825293dbd05#card`) | 3,240 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| The Five Doctors (`dbdd93d2-f2c6-43f9-93e6-2185c7df83bc#card`) | 27 | Compound PP → Of → gerund-participial Complement |
| The Great Synthesis (`b70c3adb-64ba-49ea-bcf6-6406a58cb453#face:1`) | 432 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Three Tree Scribe (`7d2a7741-1dff-4fac-92d8-abea24f23f97#card`) | 1 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Thunderblade Charge (`b4f31434-b7c4-489b-9385-70ee0cc3125e#card`) | 12 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Torrential Gearhulk (`a6395447-677d-4c39-8eda-2d57e527c94e#card`) | 54 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Transcendent Dragon (`e83a629e-2d74-48e2-ad4d-f390067cc51a#card`) | 88 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Transforming Flourish (`9fb7e7ca-ad2f-4677-859c-7a9adf8fb5fd#card`) | 1,068 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Unexpected Results (`048bdc4a-4d26-420c-88ed-23a073c30fff#card`) | 32 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Urza, Lord High Artificer (`e87906d2-db1a-4e19-b910-adb4eb339945#card`) | 384 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Victor Timely, Wily Tycoon (`2ad77eb7-9466-432f-87b4-e80e4b66e143#card`) | 378 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Villainous Wealth (`f2fae95d-074b-4c0c-99cd-f9385a424c05#card`) | 93 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Void Shatter (`3d9f854e-1ee5-4aa1-a33b-d3ae08f15dd0#card`) | 6 | Compound PP → Of → gerund-participial Complement |
| Wanda's Vision (`1448eadb-e8b1-4e8f-9697-3bb919085dab#card`) | 276 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Weftwalking (`08af6751-5848-47fb-95f5-57215e899460#card`) | 128 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Wild Evocation (`9cf1af53-ab9f-4cce-bdb5-2c13b805523d#card`) | 108 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Wildfire Devils (`b446b485-c10b-4175-bb49-16ac2f9e7718#card`) | 81 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Wildfire Eternal (`195736a9-d8b0-4c74-abe9-0289aa83c67d#card`) | 80 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Wondrous Crucible (`d0e80414-abde-4f5e-9539-3efc83aa507f#card`) | 78 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| World War Hulk (`8c4da760-8913-46ca-b00c-bba6d877d482#card`) | 64 | Without PP → gerund-participial Complement; VP/clause Adjunct |
| Yahenni's Expertise (`692babfa-9b74-415e-a560-abae4ccb006b#card`) | 38 | Without PP → gerund-participial Complement; VP/clause Adjunct |

### REPORT

At the stamped original and finished trees: **206 → 209 ordinary
Constructions**, **52 → 53 shared schemas**, **151 → 151
Categories**, **800 → 807 compiled Productions**, and
**3,755 → 4,001 nonblank declaration lines**. The refreshed
parent already contains 3,831 nonblank lines. Ordinary Constructions plus
schemas are **258 → 262**. The validated covered
count is **22,853 → 23,117**, of which 112
faces are this feature's gains. The legacy lock covered field is retired.

Inventories are unchanged: **38,404 spellings**,
**1,139 named homographs**,
**1,092 case-folded homographs**,
**23 form literals**, and **zero** literal/vocabulary
overlaps. Complete named lists are `homographs.json`,
`case-folded-homographs.json`, `form-literals.json` and
`form-literal-vocabulary-overlaps.json`; `inventory-comparison.json` also
retains unsupported-frame and unmapped-source comparisons. Corpus SHA-256 is
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb` in all compared reports.

Performance advisory, integer nanoseconds; 12 workers in each run:

| Tree / covered | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1 / 5 / 15 minute) |
| --- | ---: | ---: | --- |
| `owmovnyynvrxuqynvupomwvtrqnmuutx` / 22,853 | 117,029,599,723 | 473,013 ns/B | `[2.0771484375, 5.73046875, 6.20068359375]` |
| `zpmurnquqlpkunplvxmymyztmkxlxypw` / 23,005 | 130,576,279,486 | 468,576 ns/B | `[1.70263671875, 2.47412109375, 4.13720703125]` |
| `wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / 23,117 | 129,644,321,130 | 518,410 ns/B | `[1.7509765625, 3.63525390625, 4.23193359375]` |

The measured wall times exceed the 16,260,000,000 ns quiet-host advisory
ceiling. Host load and worker count above bound the comparison; this is
provenance, not a performance gate. Evidence remains ignored under
`target/english-v3/english-v3-except-matrix-complements/` and is preserved in
the coordinator before workspace retirement.

Validation: `cargo test -p deckmaste_english_v3` and
`cargo test -p deckmaste_lexical_source` pass: **462 English integration tests
and 57 lexical-source tests**, zero failures and zero ignored
tests. `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests
-- -D warnings` passes. Both crates were checked again after refresh changed
their consumed grammar/lexicon. `checks.json`, `refreshed-checks.json` and their
logs retain outcomes and integer command-wall timings. Changed Rust files are
formatted. No xtask tests or Lean checks ran.
