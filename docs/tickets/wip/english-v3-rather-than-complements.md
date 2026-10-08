---
needs: []
---
# Read *rather than* with matrix-matched Complements: rather than pay this spell's mana cost

## Why

*rather than* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *rather than* touches **189** unread faces
and is the sole cause on **125** (recon bucket "rather than"). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may pay {W}{U}{B}{R}{G}." 1; "You may pay
{W}{U}{B}{R}{G} rather than pay this spell's mana cost." 0; "Draw a card
rather than gain 1 life." 0; "It assigns combat damage equal to its
toughness." 1; "… equal to its toughness rather than its power." 0.

## Goal

*rather* governs a *than* Complement whose form matches the compared matrix
constituent: a bare infinitival after a bare-infinitival VP (*may pay {U}
rather than pay this spell's mana cost*), a gerund-participial after a gerund
(*by paying life … rather than paying its mana cost*), and an NP after an NP
(*equal to its toughness rather than its power*). The matching is read from
declared features of the two constituents.

## Analysis

*Rather* is a comparative governor taking *than* (CGEL, Ch. 13, §1, p. 1104,
[15iii]). An infinitival is permitted after *than* when the matrix licenses a
complement of the same form (*I'd rather stay at home than go out in this
weather*; Ch. 14, §8.4(b), p. 1263, [27i]). The NP case and the gerund case
are attested; their structure beyond what those passages say is for the
landing to record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Bringer of the White Dawn: "You may pay {W}{U}{B}{R}{G} rather than pay this
  spell's mana cost." (control without the *than* phrase 1)
- Lethargy Trap: "If three or more creatures are attacking, you may pay {U}
  rather than pay this spell's mana cost."
- Abolish: "You may discard a Plains card rather than pay this spell's mana
  cost."
- Gauntlets of Light: "Enchanted creature gets +0/+2 and assigns combat damage
  equal to its toughness rather than its power."
- Access Maze: "Once during each of your turns, you may cast a spell from your
  hand by paying life equal to its mana value rather than paying its mana
  cost."

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
   Reading. A Complement whose form does not match the compared
   constituent, or *than* read as the comparative of an adjective, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *would rather*, *prefer … to*; unattested.
- *Once during each of your turns* if it fails on its own: note it.


## Landing record

### PROVE

The claim parent **`xynkyyvk`** is source-equivalent to the full census
`wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / **23,117 covered**: all **1,697** recorded source hashes match
(`baseline-proof.json`, `before.json`). The finished feature's single full
census is `pvmvwtuqnywqnvvnppssxyqwlwxvunqp` / **23,216 covered**. The pre-census refresh changed **zero**
measured sources (`pre-census-refresh-proof.json`).

All **32,828** identities are compared face by face by source hash,
`checked_readings`, and `readings_sha256`: **99 gains, zero lost faces,
zero removed Readings, zero count or digest changes on previously covered
faces** (`comparison.json`). All 208 selector results have exactly the
full run's Reading counts and digests. The full run does no incremental reuse.

All faces enumerate completely: **32,828 complete,
0 limited, 0 failed,
0 undetermined**. **1,147,462**
Readings pass byte-exact realization, lexical ownership, construction and
leaf traversal identity, and reconstruction/admission validation; issues,
internal failures and duplicate Readings are **zero**. Independent composed
values exercise the reverse roundtrip and exact node/word traversal for all
three forms in `independent_comparisons_roundtrip_with_traversal_identity`.

Forbidden word-naming admission guards: **zero**, by diff review. The new
guards read `MatrixComparisonUse`, `ComparisonMarker`,
`PrepositionFunctionLicence`, `ComparativeComplementForm`, and the existing
Inflectional Form. Source-load diagnostics (87 unmapped strings) and
unsupported-frame diagnostics are byte-identical to the parent inventories.
V3 does not emit the retired coverage-lock or licensing-checker totals;
no legacy total or derived gate is manufactured.

### DISCLOSE

#### Analysis and route retirement

The structure is `[matrix [rather [than Complement]]]`: an Adverb governor
selects an expanded comparative Complement headed by a Preposition.
`MatrixComparisonUse = Yes` declares the governor's selection;
its `ComparisonMarker = Ordering` agrees with the selected marker's feature.
The expanded Complement exports `ComparativeComplementForm`. The nominal
host's form is declared by the `NounPhrase` schema instance, which requires
`Nominal`; the nonfinite instance reads `head.form` and matches only
`Plain ↔ BareInfinitival` or `GerundParticiple ↔ GerundParticipial`.
No lexeme or Construction identity decides admission.

CGEL, Ch. 13 §1.3, p. 1104 [15iii] lists *rather* as a comparative governor.
CGEL, Ch. 14 §8.4(b), p. 1263 [27i] describes the matching infinitival form
in *would rather*. Applying matching to postposed Oracle *rather than*,
including nominal and gerund cases, is this ticket's **project analysis**,
not a claim made by those CGEL passages.

The superseded standalone-adverb route is retired on both sides: the
lexical declaration removes its old `VPFinalAdjunct` / `ClauseInitialAdjunct`
fields and declares comparative selection; `Adverb` excludes heads with
`MatrixComparisonUse = Yes`. The Adverb Lexeme remains attested and reached
through the comparative governor. Ordinary adjectival comparative
Complements retain their existing route; the new selected expanded
Complements cannot function as free Adjuncts. Mismatch tests reject
mixed nominal/verbal forms, mixed plain/gerund forms, finite forms, a wrong
marker, a standalone governor and an adjectival analysis.

Witness **units** before → after: Bringer of the White Dawn **0 → 1**;
Lethargy Trap **0 → 1**; Abolish **0 → 1**; Gauntlets of Light **0 → 4**;
Access Maze's means Adjunct **0 → 1**. Full faces have **1, 2, 1, 4, 0**
Readings respectively on `pvmvwtuqnywqnvvnppssxyqwlwxvunqp` / **23,216 covered**. Access Maze's full face and
its control with the comparison removed both remain unread because of
*Once during each of your turns* (`access-maze-control.json`); the ticket
explicitly excludes that independent initial-frequency gap.

#### Reading census

| Measured tree / covered | No Readings | Unique | Multiple |
| --- | ---: | ---: | ---: |
| `wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / **23,117 covered** | 9,711 | 8,146 | 14,971 |
| `pvmvwtuqnywqnvvnppssxyqwlwxvunqp` / **23,216 covered** | 9,612 | 8,166 | 15,050 |

The 99 gains add **20 unique** and **79 multiple-Reading** faces, with
**3,221** added Readings. No existing unique face becomes multiple. V3
retains all grammatical Readings; there is no specificity-resolved selection
census. The alternatives include nominal comparison attachment to the
inner NP or the containing Object NP, and verbal attachment to a final
Conjunct or its coordinated VP. They keep the same matched Complement form.
The **MatchedComparativeConstituent / NominalComparativeComplement**
nesting and the **MatchedComparativeConstituent / Coordination** attachment
choices are disclosed grammatical ambiguities, not duplicate labels for one
constituency. The separate nominal and nonfinite expanded-Complement routes
are disjoint; the reverse-roundtrip tests preserve exact value comparisons.

The selector contains 208 raw faces, 185 after reminder stripping. It yields
99 gains: **82 bare infinitival, 9 gerund-participial, 8 nominal**. Eighty-six
faces with an active comparison remain unread; 7 other raw matches are unread
for independent text outside the stripped comparison. The historical
125-sole / 189-touched figures are forecasts on the ticket's older tree,
not a gain target. No gain is credited merely for recognizing a local unit.

#### Reviewed selected analyses

The following **15** newly covered faces were reviewed by exact sampled
tree (`spot-checks.json`, `selected-comparisons.json`, `subset.json`). Each
sample has the governor, selected Preposition, and matrix-matched
Complement; the full run confirms the same complete Reading-set fingerprint.

| Face | Full-face Readings | Selected matrix → Complement |
| --- | ---: | --- |
| Abolish | 1 | bare infinitival: `discard a Plains card` → `pay this spell's mana cost` |
| Anrakyr the Traveller | 30 | gerund-participial: `paying life equal to its mana value` → `paying its mana cost` |
| Bismuth Mindrender | 414 | gerund-participial: `paying life equal to the spell's mana value` → `paying its mana cost` |
| Bringer of the White Dawn | 1 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Contagion | 20 | bare infinitival: `pay 1 life and exile a black card from your hand` → `pay this spell's mana cost` |
| Daze | 2 | bare infinitival: `return an Island you control to its owner's hand` → `pay this spell's mana cost` |
| Force of Will | 10 | bare infinitival: `exile a blue card from your hand` → `pay this spell's mana cost` |
| Gauntlets of Light | 4 | nominal: `its toughness` → `its power` |
| Gush | 1 | bare infinitival: `return two Islands you control to their owner's hand` → `pay this spell's mana cost` |
| Lethargy Trap | 2 | bare infinitival: `pay {U}` → `pay this spell's mana cost` |
| Mogg Alarm | 1 | bare infinitival: `sacrifice two Mountains` → `pay this spell's mana cost` |
| Outbreak | 2 | bare infinitival: `discard a Swamp card` → `pay this spell's mana cost` |
| Snuff Out | 1 | bare infinitival: `pay 4 life` → `pay this spell's mana cost` |
| Squee, Dubious Monarch | 315 | gerund-participial: `exiling four other cards from your graveyard` → `paying its mana cost` |
| Thunderclap | 1 | bare infinitival: `sacrifice a Mountain` → `pay this spell's mana cost` |

#### Every newly covered identity

Each row names its selected analysis on the finished census tree above;
all selected expanded Complements use the *rather* Adverb governor and
*than* Preposition. Sample IDs and exact structures are retained in
`selected-comparisons.json`; all complete sets are validated in `after.json`.

| Face / identity | Readings | Selected matrix → Complement |
| --- | ---: | --- |
| Abolish — `d6d7faa8-ecc9-408b-9c46-a0f62c74f567#card` | 1 | bare infinitival: `discard a Plains card` → `pay this spell's mana cost` |
| Admiral's Order — `ae4fe5e0-408d-4398-84d7-a901b836df2a#card` | 4 | bare infinitival: `pay {U}` → `pay this spell's mana cost` |
| Angelic Favor — `37162c6e-ac31-4386-9d1e-76aa66070b35#card` | 56 | bare infinitival: `tap an untapped creature you control` → `pay this spell's mana cost` |
| Anrakyr the Traveller — `32476743-c9d4-49ca-bec2-0669c215841b#card` | 30 | gerund-participial: `paying life equal to its mana value` → `paying its mana cost` |
| Archive Trap — `d412b175-2365-4b2c-a8f5-3e4e9d06280d#card` | 1 | bare infinitival: `pay {0}` → `pay this spell's mana cost` |
| Baloth Cage Trap — `1221bef9-5947-4c32-a74b-b60279bb79bf#card` | 5 | bare infinitival: `pay {1}{G}` → `pay this spell's mana cost` |
| Bark of Doran — `db3ba239-2239-4e9d-ad70-cff32ed24578#card` | 4 | nominal: `its toughness` → `its power` |
| Belligerent Brontodon — `fa2d8440-a44a-44fe-8624-0e8b4be53f44#card` | 4 | nominal: `combat damage equal to its toughness` → `its power` |
| Bill the Pony — `e461ef8d-59ca-440a-8773-da1de3b6b195#card` | 4 | nominal: `its toughness` → `its power` |
| Bismuth Mindrender — `12b859f8-36f3-43cd-b537-6e07857b247e#card` | 414 | gerund-participial: `paying life equal to the spell's mana value` → `paying its mana cost` |
| Blasphemous Edict — `6cf50e21-0f60-4e9a-b910-ebe1bad2a29e#card` | 18 | bare infinitival: `pay {B}` → `pay this spell's mana cost if there are thirteen or more creatures` |
| Blazing Shoal — `ccba143d-ed9b-4044-bdad-f010a0e38e47#card` | 12 | bare infinitival: `exile a red card with mana value X from your hand` → `pay this spell's mana cost` |
| Blue Mage's Cane — `6290233b-b015-4909-8aba-bf8d65cc37b5#card` | 180 | gerund-participial: `paying {3}` → `paying its mana cost` |
| Bounty of the Hunt — `77a43c24-398f-42c6-a0d3-b28b33a0ce12#card` | 36 | bare infinitival: `exile a green card from your hand` → `pay this spell's mana cost` |
| Bringer of the Black Dawn — `a9f98ad5-e8d4-4142-a19f-3368b85d1c95#card` | 43 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Bringer of the Blue Dawn — `7d4676fa-4bfe-43b4-9241-c2a33214852a#card` | 1 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Bringer of the Green Dawn — `f0548af1-d1cc-44d1-b5a0-f013563f29de#card` | 1 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Bringer of the Red Dawn — `2ac11645-1290-4026-af19-e47e59c277b2#card` | 40 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Bringer of the White Dawn — `d6ee95a9-e518-49fd-bc11-02453c3220cd#card` | 1 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay this spell's mana cost` |
| Cave-In — `7d937265-02f5-4f2c-9fef-c5930e13b3be#card` | 3 | bare infinitival: `exile a red card from your hand` → `pay this spell's mana cost` |
| Cobra Trap — `f3953f48-01c0-4647-9e4a-d70708d01e33#card` | 16 | bare infinitival: `pay {G}` → `pay this spell's mana cost` |
| Commandeer — `ad7d854b-d303-40eb-acd6-03a8023e05e7#card` | 15 | bare infinitival: `exile two blue cards from your hand` → `pay this spell's mana cost` |
| Conspiracy Unraveler — `e4541d02-2860-46ef-b630-1d6307b60766#card` | 12 | bare infinitival: `collect evidence 10` → `pay the mana cost for spells you cast` |
| Contagion — `433e86ce-df04-4182-bb63-ad2607d67dbd#card` | 20 | bare infinitival: `pay 1 life and exile a black card from your hand` → `pay this spell's mana cost` |
| Crash — `562af5da-1080-46d2-8a38-71f06b7e8457#card` | 1 | bare infinitival: `sacrifice a Mountain` → `pay this spell's mana cost` |
| Dark Triumph — `885630a8-c43b-4804-acd6-14a7c7321d73#card` | 2 | bare infinitival: `sacrifice a creature` → `pay this spell's mana cost` |
| Daze — `70486bee-6ee7-41ea-b834-8caf4699302b#card` | 2 | bare infinitival: `return an Island you control to its owner's hand` → `pay this spell's mana cost` |
| Delraich — `38f9e766-a2ad-437b-a38c-d6f3ffd261fb#card` | 1 | bare infinitival: `sacrifice three black creatures` → `pay this spell's mana cost` |
| Demon of Death's Gate — `be465805-1a37-4212-acf1-5dfec890b338#card` | 3 | bare infinitival: `sacrifice three black creatures` → `pay this spell's mana cost` |
| Disrupting Shoal — `2912d93d-ed51-41ef-b64f-3cb6581c09e5#card` | 12 | bare infinitival: `exile a blue card with mana value X from your hand` → `pay this spell's mana cost` |
| Doran, the Siege Tower — `a46d96d9-8e59-4777-8208-7730b9e33240#card` | 4 | nominal: `its toughness` → `its power` |
| Downhill Charge — `8018419c-ea19-499b-a33a-7c20b35bce7b#card` | 4 | bare infinitival: `sacrifice a Mountain` → `pay this spell's mana cost` |
| Ensnare — `df197279-ddb2-4237-8bb1-8df41d3bed9e#card` | 1 | bare infinitival: `return two Islands you control to their owner's hand` → `pay this spell's mana cost` |
| Fieldmist Borderpost — `a292ca5b-2472-4b02-a26b-d1b5b717a34b#card` | 6 | bare infinitival: `return a basic land you control to its owner's hand` → `pay this spell's mana cost` |
| Fireblast — `9dd7f27a-e862-47d7-9158-034cf4d353b8#card` | 1 | bare infinitival: `sacrifice two Mountains` → `pay this spell's mana cost` |
| Firewild Borderpost — `312a5a75-9b68-4e4b-97fe-3ab644e3e493#card` | 6 | bare infinitival: `return a basic land you control to its owner's hand` → `pay this spell's mana cost` |
| Fist of Suns — `6fd5e591-fae8-4128-a4d3-a848c8a8ffda#card` | 12 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay the mana cost` |
| Foil — `7dde4eb6-e9d7-4259-abc2-3af738e0f00f#card` | 1 | bare infinitival: `discard an Island card and another card` → `pay this spell's mana cost` |
| Force of Will — `956381ba-6d37-4a8a-846c-bad79222dbee#card` | 10 | bare infinitival: `exile a blue card from your hand` → `pay this spell's mana cost` |
| Gauntlets of Light — `e5741c85-cd09-4ccb-ab39-3f3a62f98939#card` | 4 | nominal: `its toughness` → `its power` |
| Gavi, Nest Warden — `716a3bb8-04bd-41f5-a568-131f175e00a6#card` | 112 | bare infinitival: `pay {0}` → `pay the cycling cost of the first card you cycle` |
| Gush — `16d8ee99-8ec8-429f-9ba7-818a74b6f910#card` | 1 | bare infinitival: `return two Islands you control to their owner's hand` → `pay this spell's mana cost` |
| Inferno Trap — `83a71000-942b-4472-9521-0e1070a3343b#card` | 9 | bare infinitival: `pay {R}` → `pay this spell's mana cost` |
| Inside Information — `66961bcb-f588-47a7-b3f3-12f4ac674477#card` | 12 | bare infinitival: `pay life equal to its mana value` → `pay its mana cost` |
| Jadzi, Oracle of Arcavios — `ca79cd3f-13ba-4d11-b885-48a504ae69aa#face:0` | 128 | gerund-participial: `paying {1}` → `paying its mana cost` |
| Jodah, Archmage Eternal — `8be4745e-36d8-430f-945e-c8a7fde9b4f6#card` | 12 | bare infinitival: `pay {W}{U}{B}{R}{G}` → `pay the mana cost for spells you cast` |
| Kentaro, the Smiling Cat — `bfaf20a7-a11c-4f57-8eb6-b03b5223704d#card` | 22 | bare infinitival: `pay {X}` → `pay the mana cost for Samurai spells you cast` |
| Lashknife — `98d790c0-985f-44a4-b247-0951beea7637#card` | 2 | bare infinitival: `tap an untapped creature you control` → `pay this spell's mana cost` |
| Lavaball Trap — `a9cdfb6c-6087-4b0d-b8a5-175f3546b23a#card` | 5 | bare infinitival: `pay {3}{R}{R}` → `pay this spell's mana cost` |
| Lethargy Trap — `7356319d-1485-426b-80a7-9700d6684ac9#card` | 2 | bare infinitival: `pay {U}` → `pay this spell's mana cost` |
| Mind Swords — `4af20f8c-2ba0-49d5-a64e-d3668cec30cd#card` | 3 | bare infinitival: `sacrifice a creature` → `pay this spell's mana cost` |
| Mindbreak Trap — `48712d36-5c13-4ded-87a4-68106da4a5cd#card` | 1 | bare infinitival: `pay {0}` → `pay this spell's mana cost` |
| Mine Collapse — `83df0f44-86ca-4adb-9f3a-da1ba6dd7a6d#card` | 1 | bare infinitival: `sacrifice a Mountain` → `pay this spell's mana cost` |
| Misdirection — `c39e5fb0-6de3-4105-ad3c-0ecb8951a1d5#card` | 12 | bare infinitival: `exile a blue card from your hand` → `pay this spell's mana cost` |
| Mistvein Borderpost — `1499af11-274f-4f8d-b0b6-20e1693ebab7#card` | 6 | bare infinitival: `pay {1} and return a basic land you control to its owner's hand` → `pay this spell's mana cost` |
| Mogg Alarm — `1c2117d5-4600-489c-be5b-dc0ed69b30ca#card` | 1 | bare infinitival: `sacrifice two Mountains` → `pay this spell's mana cost` |
| Needlebite Trap — `c8fedb0e-27d5-4fae-8058-3e9666a9661d#card` | 1 | bare infinitival: `pay {B}` → `pay this spell's mana cost` |
| Nemesis Trap — `1c6f95fd-f8d0-4d22-aa21-29e5c70b2f85#card` | 4 | bare infinitival: `pay {B}{B}` → `pay this spell's mana cost` |
| Nourishing Shoal — `2aa0e0e5-cb6d-4518-8eff-d29f935486e0#card` | 6 | bare infinitival: `exile a green card with mana value X from your hand` → `pay this spell's mana cost` |
| Outbreak — `05079cee-b190-42d7-b59f-130cb545cab7#card` | 2 | bare infinitival: `discard a Swamp card` → `pay this spell's mana cost` |
| Permafrost Trap — `f037b319-e559-42ad-bcfe-d43d07ce05e7#card` | 30 | bare infinitival: `pay {U}` → `pay this spell's mana cost` |
| Pulverize — `d403d1f3-5a92-4f4c-aff2-cff6a0b3b1ec#card` | 1 | bare infinitival: `sacrifice two Mountains` → `pay this spell's mana cost` |
| Raffine's Guidance — `ad3bf726-e0c2-4038-a66e-9a9dff439563#card` | 9 | gerund-participial: `paying {2}{W}` → `paying its mana cost` |
| Ramosian Rally — `2731c237-eda1-4b74-a294-502d574f4435#card` | 4 | bare infinitival: `tap an untapped creature you control` → `pay this spell's mana cost` |
| Reverent Mantra — `b4f7b1eb-6cd4-40f9-b259-3c39291edf27#card` | 12 | bare infinitival: `exile a white card from your hand` → `pay this spell's mana cost` |
| Ricochet Trap — `15df3467-c67e-4ae2-9e6f-ffb6ad288a5a#card` | 4 | bare infinitival: `pay {R}` → `pay this spell's mana cost` |
| Rooftop Storm — `6835bf6d-7197-481b-ac26-276ce363b4ad#card` | 32 | bare infinitival: `pay {0}` → `pay the mana cost` |
| Rouse — `475099d9-4743-420a-ada8-3932f5ef9e82#card` | 2 | bare infinitival: `pay 2 life` → `pay this spell's mana cost` |
| Runeflare Trap — `259efd92-416e-42d9-b501-ac1338fd35fc#card` | 75 | bare infinitival: `pay {R}` → `pay this spell's mana cost` |
| Runeforge Champion — `2705a288-3b9d-4452-a320-28b86331aa09#card` | 594 | bare infinitival: `pay {1}` → `pay the mana cost` |
| Salvage Titan — `823cd586-92db-4e44-ba1f-394b2404cafc#card` | 3 | bare infinitival: `sacrifice three artifacts` → `pay this spell's mana cost` |
| Scourge of Nel Toth — `32972d7a-c7c6-437c-8e7c-6c7d56b130e4#card` | 18 | gerund-participial: `paying {B}{B} and sacrificing two creatures` → `paying its mana cost` |
| Sephara, Sky's Blade — `ac8952c9-bd30-48cd-b964-7e6271f9cbc8#card` | 115 | bare infinitival: `pay {W} and tap four untapped creatures you control with flying` → `pay this spell's mana cost` |
| Sickening Shoal — `2da8e9d6-e8a8-46a5-a478-3e81098d2a08#card` | 12 | bare infinitival: `exile a black card with mana value X from your hand` → `pay this spell's mana cost` |
| Sivvi's Valor — `e02164ba-34f8-4a5f-a05b-dd3ef3f8ceae#card` | 18 | bare infinitival: `tap an untapped creature you control` → `pay this spell's mana cost` |
| Slingbow Trap — `66ce0ae5-7000-4e4a-891f-f571f0e58cd7#card` | 8 | bare infinitival: `pay {G}` → `pay this spell's mana cost` |
| Snapback — `e88de17c-086d-4e2a-b5b2-2f9f57ca7c0f#card` | 3 | bare infinitival: `exile a blue card from your hand` → `pay this spell's mana cost` |
| Snuff Out — `324824cb-f938-401c-b9b5-d8908b431ef0#card` | 1 | bare infinitival: `pay 4 life` → `pay this spell's mana cost` |
| Solid Footing — `edc91e1a-a305-4c14-9d85-077a410528fa#card` | 4 | nominal: `its toughness` → `its power` |
| Soul Spike — `be979bfc-998d-43d5-a7a4-44548d9a14c4#card` | 3 | bare infinitival: `exile two black cards from your hand` → `pay this spell's mana cost` |
| Spinning Darkness — `69eeee95-cc69-41d1-833b-8895ef72df82#card` | 4 | bare infinitival: `exile the top three black cards of your graveyard` → `pay this spell's mana cost` |
| Squee, Dubious Monarch — `124a5dd7-c4f8-479c-9eb0-669aca01d045#card` | 315 | gerund-participial: `exiling four other cards from your graveyard` → `paying its mana cost` |
| Stalwart Valkyrie — `b6b4c459-9c8b-4b95-8706-05d972b8daea#card` | 10 | bare infinitival: `pay {1}{W} and exile a creature card from your graveyard` → `pay this spell's mana cost` |
| Streetwise Negotiator — `74b631c5-9420-496e-8845-3b050d3d5340#card` | 4 | nominal: `combat damage equal to its toughness` → `its power` |
| Summoning Trap — `2faa8dc1-0370-458c-9aef-44f81a9b5cd5#card` | 144 | bare infinitival: `pay {0}` → `pay this spell's mana cost` |
| Sunscour — `155f14dd-1516-4b9f-ae39-0a6172c69a66#card` | 3 | bare infinitival: `exile two white cards from your hand` → `pay this spell's mana cost` |
| The Infamous Cruelclaw — `e229d55c-9513-437d-bd67-015ac46aaa18#card` | 207 | gerund-participial: `discarding a card` → `paying its mana cost` |
| The Lady of Otaria — `1ea4f882-6872-4d9e-9532-5ba2bf848d00#card` | 90 | bare infinitival: `tap three untapped Dwarves you control` → `pay this spell's mana cost` |
| Thunderclap — `02f5ceb4-8b9c-4107-972b-6eec365c4a6c#card` | 1 | bare infinitival: `sacrifice a Mountain` → `pay this spell's mana cost` |
| Thwart — `615a149a-2869-4b18-bbbe-9ec1825049d7#card` | 1 | bare infinitival: `return three Islands you control to their owner's hand` → `pay this spell's mana cost` |
| Tidal Bore — `513ad32a-2887-4825-adad-86a87cf1d2cb#card` | 1 | bare infinitival: `return an Island you control to its owner's hand` → `pay this spell's mana cost` |
| Treefolk Umbra — `fa321366-9bc1-48c7-873a-0b12a026ce49#card` | 4 | nominal: `combat damage equal to its toughness` → `its power` |
| Unmask — `1a3a274d-41da-47db-ad41-5720f36a7963#card` | 12 | bare infinitival: `exile a black card from your hand` → `pay this spell's mana cost` |
| Veinfire Borderpost — `3f65d1f8-3e46-4e3d-b087-5496a4106e4e#card` | 6 | bare infinitival: `return a basic land you control to its owner's hand` → `pay this spell's mana cost` |
| Whiplash Trap — `1bd26692-c030-4493-883e-2d26b3a1d025#card` | 5 | bare infinitival: `pay {U}` → `pay this spell's mana cost` |
| Wildfield Borderpost — `b8141489-00fa-4aba-85d0-4c883196fece#card` | 6 | bare infinitival: `pay {1} and return a basic land you control to its owner's hand` → `pay this spell's mana cost` |
| Worldheart Phoenix — `67c50af9-bb8e-40b6-8b25-c209784b6865#card` | 81 | gerund-participial: `paying {W}{U}{B}{R}{G}` → `paying its mana cost` |
| Xander's Pact — `3df11e5c-19aa-441c-a889-6b40eb3fdabf#card` | 40 | bare infinitival: `pay life equal to that spell's mana value` → `pay its mana cost` |
| Zahid, Djinn of the Lamp — `89b07037-64df-4e66-acd8-87d97df61e3a#card` | 6 | bare infinitival: `pay {3}{U} and tap an untapped artifact you control` → `pay this spell's mana cost` |

#### Deviations and additions

- Added `NominalMatrixComparativeComplement` and
  `NonfiniteMatrixComparativeComplement`: distinct nominal and nonfinite
  Complement shapes keep matching separate from adjectival comparisons.
- Added `MatrixComparativePhrase`: preserves governor and selected expanded
  Complement constituency.
- Added one shared `MatchedComparativeConstituent` schema with NP and
  nonfinite VP instances: matches the two forms and preserves host features.
- Added **6** integration tests; **0** restored, **0** re-spelled,
  **0** removed, **0** newly ignored. No existing assertion is weakened.
- Added glossary definitions for **Comparative Complement Form** and
  **Matrix Comparison Use**. These are linguistic/project grammatical terms;
  no Comprehensive Rules citation is applicable.
- No vocabulary, plugin-body or xtask tooling change. Evidence helpers remain
  ignored under `target/`. No additional Construction or test is deleted.
- No STOP or ruling contradiction arose. Compiler/test API mistakes were
  fixed within this ticket before census; no failed verification is bypassed.

### REPORT

Counts are stamped `wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / **23,117 covered** → `pvmvwtuqnywqnvvnppssxyqwlwxvunqp` / **23,216 covered**:
**209 → 212 ordinary Constructions**, **53 → 54 shared schemas**,
**262 → 266 ordinary Constructions plus schemas**, **151 → 153 Categories**,
**807 → 812 compiled Productions**, and
**4,001 → 4,063 nonblank declaration lines**. The validated covered count
is **23,117 → 23,216**; the legacy lock `covered` field is retired.

Lexical inventories are unchanged on those stamped trees:
**38,404 spellings**, **1,139 named homographs**, **1,092 case-folded
homographs**, **23 form literals**, **0 literal/vocabulary overlaps**.
Complete named lists: `homographs.json`, `case-folded-homographs.json`,
`form-literals.json`, `form-literal-vocabulary-overlaps.json` (the last is
**{}**). `inventory-comparison.json` proves their equality with the parent.

Performance advisory: **12 workers**, integer nanoseconds in both rows.

| Measured tree / covered | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1 / 5 / 15 minute) |
| --- | ---: | ---: | --- |
| `wuvkkwmvzsmrokrvssqrltwuzmovqkkz` / **23,117 covered** | 129,644,321,130 | 518,410 ns/B | `[1.7509765625, 3.63525390625, 4.23193359375]` |
| `pvmvwtuqnywqnvvnppssxyqwlwxvunqp` / **23,216 covered** | 124,815,861,419 | 509,959 ns/B | `[8.59521484375, 7.40966796875, 6.9755859375]` |

The finished run's setup is **2,316,004,720 ns**, and the full cargo
command wall time is **130,591,540,915 ns**, with launch host load
`[8.59521484375, 7.40966796875, 6.9755859375]`. Corpus wall time exceeds the **16,260,000,000 ns**
quiet-host advisory ceiling; this is provenance, not a gate.

Exactly one full run was used on the finished feature, with no incremental
reuse:

```text
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-rather-than-complements/after.json
```

Iteration used only the explicit *rather than* selector and fragment probes.
Validation passed:

```text
cargo test -p deckmaste_english_v3
cargo test -p deckmaste_lexical_source
cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings
cargo xtask cite check --list-noncompliant
cargo xtask cite check
```

Tests: **468** English integration tests and **57** lexical-source tests,
zero failed and zero ignored. Clippy passes with warnings denied. Citation
checks report **0 noncompliant / 0 stale**; no Comprehensive Rules citation
was added or changed. Changed Rust files are formatted. No xtask tests,
Lean checks, derived gate or full re-census ran. Evidence is preserved
under the coordinator's ignored
`target/english-v3/english-v3-rather-than-complements/` before retirement.
