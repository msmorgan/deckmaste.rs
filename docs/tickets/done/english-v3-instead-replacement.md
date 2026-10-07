---
needs: []
---
# Read "instead" in replacement and alternative effects

## Why

*Instead* never reads in Oracle replacement text. On change `wlvwtnppyovn`
(32,828 supported faces, 13,716 covered, 19,112 unread) it occurs in a failing
unit of 979 unread faces and is the only recognised cause on 288. These are
surface-bucket counts, not gain forecasts. The *would* host is not the problem:
"If that creature would die this turn, exile it." has one admitted root, and
"… exile it instead." has none; "Exile it instead." alone has none.

Two lexemes spell *instead*: `vocab:Preposition/Instead` (core.ron) and
`vocab:ReplacementMarker/Instead` (vocabulary.rs). Deal's first frame in
`verbs.ron` ends `OptionalRole("ReplacementMarker")`. This ticket takes over
that ReplacementMarker slot from the Deal row of the unsupported-inventory
table (done `english-v3-generic-frame-consumption`, routed to
`english-v3-systemic-residuals`). The row's `MassNoun` and `DistributionPhrase`
slots stay with the residuals ticket. It also takes over Burn the Accursed, one
of the 42 inherited frame-coordination faces in that ticket, whose only failing
unit is an *instead* sentence.

## Goal

*Instead* reads clause-finally ("…, exile it instead."), clause-initially
("…, instead any number of target creatures you control gain indestructible
…"), and with its Complement (*instead of X*), with one lexical analysis. The
duplicate lexeme is either retired or justified by a distinct declared
function, recorded in the landing. Any Reading the corpus cannot disambiguate
stays: clause-final *instead* attaches to the main clause and may also attach
to a coordinated predicate.

## Analysis

*Instead* is a Compound Preposition: *in* + *stead* have coalesced into one
word that takes an *of* Complement (CGEL, Ch. 7, §3.1, pp. 622–623; glossary
Compound Preposition). Its Complement is optional, and when it is omitted,
*of* drops too (CGEL, Ch. 7, §2.4, p. 616, [32v], [33]). Bare *instead*
therefore heads a PP with no overt Complement, like *out*, not an Adverb. In
the replacement sentences it functions as a connective Adjunct of contrast:
CGEL lists *instead* among the connective adjuncts of addition and comparison
(CGEL, Ch. 8, §19, p. 778, [10]). Its Preposition Function Licence must admit
the clause Adjunct function. A lexical verb frame should select it only where
the corpus requires a Complement reading.

Ruling (user, relayed with this ticket's brief, 2026-10-06): stranded *be*,
*have* and modals are not attested Oracle style ("If it would be, instead …"
does not occur). Add no ellipsis licence for them. Do-support (*if you do*)
and *can't* are attested and keep their existing analyses. If the
implementation seems to need a stranded-auxiliary Reading, STOP and report.

## Witnesses

- Forbidden Crypt: "If you would draw a card, return a card from your
  graveyard to your hand instead." (its other unit, "If a card would be put
  into your graveyard from anywhere, exile that card instead.", also needs
  *from anywhere*)
- Burn the Accursed: "If that creature would die this turn, exile it instead."
- Soldevi Excavations: "If this land would enter, sacrifice an untapped Island
  instead."
- Increasing Savagery: "If this spell was cast from a graveyard, put ten
  +1/+1 counters on that creature instead." (non-*would* host)
- Divine Resilience: "If this spell was kicked, instead any number of target
  creatures you control gain indestructible until end of turn."
  (clause-initial)
- Reality Twist: "If tapped for mana, Plains produce {R}, Swamps produce {G},
  Mountains produce {W}, and Forests produce {B} instead of any other type."
  (*instead of*)

Except for Forbidden Crypt, each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/instead-before.json` on the claim parent,
   stamped with its change id.
2. Write the witnesses as tests first, each with its *instead*-less twin where
   one reads.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *instead* attached inside an
   NP, or any stranded-auxiliary gap, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule). Deleting the duplicate lexeme follows the same rule.
8. Gate: `cargo xtask gate --changed --from <claim> --run`.
9. A CGEL citation may back only what the cited passage itself says; a project
   or orchestrator ruling is cited as a ruling, never attributed to CGEL.
10. Retire a superseded route on both the lexicon and the grammar side; do not
    leave unreachable declarations.

## Out of scope

- Ellipsis recoverability in general (`english-v3-ellipsis-recoverability`,
  maybe).
- Host-clause causes such as *double that damage* (Fire Servant), *from
  anywhere*, *rather than* (189 touched / 70 sole).

## Landing record

### Post-refresh record

Kata refresh incorporated the coordinator's completed
[english-v3-then-sequencing](../done/english-v3-then-sequencing.md) and conflicted
only on the PrepositionPhraseSeries/AdverbPhraseSeries declaration boundary.
The resolution keeps both this item's ClauseInitialAdjunct property and the
coordinator's UnmarkedConjunctLicence. The conflict revset is empty and retrying
refresh exits zero. Both grammar and lexicon changes from the coordinator are
retained; no inherited test or declaration is removed.

Current measured tree `pmpwnuprtqpszolnywwkykrznqqysozk`, covered **17,556**:
`instead-refresh.json`, inventory SHA-256
`339d46e07fe00728495ad4272e0b17e5423e4986ef7830e5cd92ced0ebeb2f9f`.
All 32,828 faces are completely enumerated and all **343,616** Readings pass
admission, ownership, realization and traversal checks. Issues, internal
failures, cycles and duplicate Readings remain zero. Selection census remains
7,122 unique / 10,434 multiple; 234 gains and zero lost faces remain unchanged.
The fresh complete-Reading audit of all 234 gains has zero bad Readings.
Construction inventory remains 192 ordinary + 44 schemas = 236, with
598 static / 666 compiled productions and the same named homograph and
empty literal-overlap inventories as below.

Against the original claim-parent census (17,322 covered), **202 previously
covered faces have decreased Reading counts**: the same 201 stranding
retirements named in PROVE below, plus this one inherited decrease:

| Face / identity | Readings before → after | Retired Reading / authority |
|---|---:|---|
| Cryptic Annelid / `32ebc862-3bf6-4754-b7b8-8bb73f1651cf#card` | 13 → 11 | Two InitialAdverb/Adverb-Then analyses on a ClauseSeries middle member, already retired by the then-sequencing orchestrator resolution of 2026-10-06. |

STOP resolution (orchestrator, 2026-10-07): Cryptic Annelid 13 → 11 is inherited from trunk changes `stwlrsrz`/`xxknlzyp`, not produced by this ticket, and the standing rule compares against the refreshed base or attributes inherited changes by change id without stopping, so this STOP is resolved.

Current performance advisory, stamped `pmpwnupr` / 17,556 covered, 12 workers:
53,149,616,479 ns corpus wall, **266,780 ns/B** checked-text thread CPU,
host load 4.22607421875 / 2.21826171875 / 1.51904296875. Setup is
6,434,320,010 ns. This loaded-host run exceeds the 16,260,000,000 ns quiet-host
ceiling and remains an advisory. The original pre-refresh measurements below
are retained as provenance and do not supersede this current census.
Post-conflict validation: the same derived command documented below passes
(exit zero): **763 passed, zero failed, one unchanged ignored census test**.
`instead-refresh-gate.log` includes all seven successful Lean checks. Clippy
for the four derived packages with `--all-targets` and changed-file nightly
formatting pass. From the workspace root, citation checks report zero
noncompliant strings and zero stale among 16,113 citations; the piped diff
audit selects zero changed CR citation sites. The prior validation record
below remains provenance for the pre-refresh tree.

The following PROVE/DISCLOSE/REPORT sections record the pre-refresh measured
tree, distinguished by its inventory hash. Their classification and counts
for this item's own changes remain valid; this post-refresh record is the
current census authority and records the additional inherited Reading decrease.

### PROVE

Claim parent `pyoyosmvznzsulkxzytxsusstulvqwpq`: 17,322 covered.
Baseline measured in test-only child `pmpwnuprtqpszolnywwkykrznqqysozk`:
17,322 covered. Its consumed lexicon and grammar were byte-identical to the
claim parent, verified against `jj file show`; source copies and
`instead-baseline-provenance.json` retain that evidence. Final measured tree
`pmpwnuprtqpszolnywwkykrznqqysozk`: 17,556 covered. Both report stamps and
inventory hashes distinguish the measured before/after trees.

Against the claim parent: **234 gained faces, 201 previously covered faces
with decreased Reading counts, zero lost faces**. Every decrease is
**wrong analysis retired** under the dated user/orchestrator ruling below.
The complete named identity/count/classification and retired-Reading evidence
is [instead-reading-retirements.md](../../../target/english-v3/english-v3-instead-replacement/instead-reading-retirements.md).
`instead-resolved-comparison.json` lists all 201 decreases and empty lost-face
and unattributed-change lists. No valid Reading is intentionally retired,
and no re-coverage obligation or regression is created.

The pre-exclusion grammar independently enumerated every covered candidate
Reading and identified 47,346 Readings with an unlicensed omitted auxiliary
Complement: 36,020 on the 201 previously covered faces and 11,326 on 18 gained
faces. For every one of all 32,828 faces, its predicted retained count equals
the final census exactly; no unrelated decrease or increase occurred. Named
full retired Reading representatives and every retired diagnostic identity
are retained in `instead-exclusion-prediction.json`, the named report above,
and `retired-reading-identities-*.jsonl`. The 18 gains retain their valid
alternatives; their Reading count changes are also individually listed.
Final valid gains contribute 12,391 Readings. Complete Readings reconcile as
367,247 − 36,020 + 12,391 = 343,618.

Named decrease record, measured on `pmpwnupr` / 17,556 covered against the
claim-parent grammar `pyoyosmv` / 17,322 covered. Every row is classified
**wrong analysis retired**: only the omitted auxiliary Complement of the
named head(s) is excluded by the dated ruling. Complete retired Reading
representatives are in the linked evidence; all retained counts are validated.

| Face / identity | Readings before → after | Retired auxiliary head(s) |
|---|---:|---|
| Agitator Ant / `abe377f8-35f0-4dea-a681-91d31a36815c#card` | 10 → 6 | Have |
| Al-abara's Carpet / `0f5b0c77-1e3d-46a1-ae0e-03ed79196cd9#card` | 298 → 166 | Be |
| Angelsong / `fe7bad80-f853-4af9-82e2-5ccf8038d93b#card` | 28 → 10 | Be |
| Anger / `eaabd151-2160-4bff-82c0-3fa88659be98#card` | 3 → 1 | Be |
| Argothian Pixies / `bbf183bc-d502-4432-8202-f29f60c08396#card` | 100 → 40 | Be |
| Argothian Treefolk / `f3aaef18-dc32-40d6-b48c-f957aa31247f#card` | 20 → 8 | Be |
| Armament of Nyx / `1a4d16eb-343e-41ca-9363-90dc79da4769#card` | 34 → 14 | Be |
| Armored Transport / `0caae56e-5995-4f1b-b735-80580a372707#card` | 86 → 30 | Be |
| Artifact Ward / `9bd3a4bb-cc12-4e5f-a33f-77ab0c7788db#card` | 500 → 200 | Be |
| Astral Drift / `7aa5db50-5057-45fd-bce9-fb4a5e874263#card` | 324 → 54 | Be |
| Auntie's Snitch / `d2964a68-36d9-4b05-8f34-540969b410bf#card` | 36 → 12 | Be |
| Avacyn, Guardian Angel / `6da0b6d2-3c5e-49ef-9264-076269c04744#card` | 9,801 → 3,025 | Be |
| Awakened Skyclave / `4b1874af-4ea5-4e22-a4d4-e718d75fe95e#face:1` | 15 → 5 | Be |
| Azorius Ploy / `b9b58c3a-5a76-4b6a-884b-74f54f0ca53c#card` | 186 → 66 | Be |
| Black Panther, Hope Enduring / `fe00278d-4385-4e3f-919c-12a794fd86f5#face:1` | 162 → 54 | Be |
| Blessed Reincarnation / `ca29588c-f117-418e-be9e-fa2ee89862ca#card` | 46 → 16 | Be |
| Blessed Respite / `8c888122-9d05-4641-931c-83f771ba9d09#card` | 28 → 10 | Be |
| Blinding Fog / `50555471-a03c-4735-a6ce-6ae96e27d7c0#card` | 22 → 10 | Be |
| Blinding Powder / `75e13f7f-aded-405c-97ce-124fa0cb1929#card` | 31 → 11 | Be |
| Bloodfire Enforcers / `5e4facd9-d416-4ade-ba71-6da32b12a891#card` | 12 → 2 | Be |
| Blunt the Assault / `dccd0533-fda0-41ff-b540-c4f1bb32fc95#card` | 140 → 50 | Be |
| Boneyard Scourge / `eef40ab2-9e17-4e8d-9e58-1a82ab871b5b#card` | 15 → 3 | Be |
| Brawn / `00876e98-d062-4a12-85e6-86a2b20cf867#card` | 3 → 1 | Be |
| Bronze Horse / `a7887b24-977d-4a40-bb30-4bf467e5dba6#card` | 26 → 10 | Be |
| Bubble Matrix / `50261dca-bfeb-40b3-95b6-9057491ef3ad#card` | 6 → 2 | Be |
| Call to the Grave / `db5a4c25-5ae5-4a04-be79-bdee39b9152c#card` | 12 → 4 | Be |
| Cass, Hand of Vengeance / `f9b6b1db-37c6-43d4-b762-6c9770da5964#card` | 1,720 → 80 | Be |
| Champion Lancer / `2681bfb5-127b-4046-85ac-72be584999cb#card` | 20 → 8 | Be |
| Cho-Manno, Revolutionary / `91af5e35-b3b8-43ce-b1ea-997ed74e4ad2#card` | 6 → 2 | Be |
| Commencement of Festivities / `56ea8f6b-a765-44e7-bf0e-4d4a2405684b#card` | 31 → 11 | Be |
| Covert Cutpurse / `5d0b8dc6-f4b6-4650-805d-4240d4a4ab82#face:0` | 7 → 3 | Be |
| Crippling Fear / `e7a7ba65-ad14-41d5-889b-9bbeae9ab3f7#card` | 6 → 2 | BeNegative |
| Crushing Pain / `34424168-1353-46a9-9e60-d09a344c3573#card` | 7 → 3 | Be |
| Cut Short / `428d131c-b0fa-4e96-9c02-e2e6714357b9#card` | 4 → 2 | Be |
| Darkness / `932708ae-9d5c-4561-aa8b-0d2222d37fdc#card` | 28 → 10 | Be |
| Dawn Charm / `a8f5cfa7-4956-4182-8e44-acf3493239f0#card` | 28 → 10 | Be |
| Dawn Elemental / `307421ed-aa59-4518-a0eb-689c60d7b709#card` | 6 → 2 | Be |
| Dawnstrider / `d2783a37-b6de-4184-b094-e1a23f185a94#card` | 28 → 10 | Be |
| Dearly Departed / `539be0f3-0863-4d44-87ee-5912dd003475#card` | 72 → 24 | Be |
| Defang / `5a6c81b8-71f0-468b-85ad-d87e9a712ecf#card` | 14 → 6 | Be |
| Defend the Hearth / `1dd64972-5713-4c25-b7a5-a35a56886ef4#card` | 31 → 11 | Be |
| Demonic Torment / `2172b724-9004-488e-88d3-a5fc48c50e41#card` | 36 → 12 | Be |
| Divine Light / `924bf3bb-9f06-4a2c-a17f-3fe11551bcf2#card` | 46 → 22 | Be |
| Djeru's Resolve / `c48ce123-bb07-4221-b3b3-0a458e101ca1#card` | 11 → 5 | Be |
| Dolmen Gate / `d53d000f-c92f-4d70-ae60-8be3791da8a2#card` | 88 → 24 | Be |
| Downwind Ambusher / `1e0a2709-2cd2-4954-81ca-60654ab03d7c#card` | 14 → 6 | Be |
| Druid's Deliverance / `fde7645a-5f02-4d5f-b38c-8390f325899e#card` | 31 → 11 | Be |
| Dune Chanter / `7811b50d-af76-43cd-8753-43cf65fbefa3#card` | 1,320 → 216 | BeNegative |
| Edgar Markov / `41e2790d-49f5-4e98-b8d9-04179f47f13a#card` | 864 → 288 | Be |
| Eerie Interference / `35ba0e70-da7c-4ed9-b8ec-7dd5c5ce110e#card` | 119 → 47 | Be |
| Emmara Tandris / `38a71429-e673-4de4-80a8-f59e5cf98acc#card` | 18 → 6 | Be |
| Enchanted Being / `c98b725e-ca16-4576-bf53-653d4028d861#card` | 58 → 18 | Be |
| Encircling Fissure / `39d7569e-62b0-4f37-813a-0985632c66c9#card` | 167 → 75 | Be |
| Endure / `f4cd32f0-d6aa-4497-b8db-a0bf4c3c31de#card` | 41 → 15 | Be |
| Energy Storm / `567d3de7-8d56-4b6d-a59a-f8674172f595#card` | 84 → 36 | Be |
| Errant, Street Artist / `23e6df25-90db-42a4-8658-e592e67be15e#card` | 15 → 5 | BeNegative |
| Ethereal Haze / `0cdbd9e7-4941-46ac-99d8-ea227181bdf8#card` | 24 → 14 | Be |
| Ethersworn Shieldmage / `ee988017-fc7e-4d8a-8f5c-0a7e57a8d050#card` | 11 → 5 | Be |
| Everdawn Champion / `964e065f-380d-491b-9225-244c5f9edda7#card` | 19 → 5 | Be |
| Excruciator / `9911c2f8-fafb-4979-898e-e86b726e3538#card` | 8 → 4 | Be |
| Fangs of Kalonia / `8d9bd4cc-564b-4fdf-9462-b4bb30583642#card` | 187 → 132 | Have |
| Fatal Blow / `16774611-c004-4383-b4e1-4a43a9b1f4aa#card` | 7 → 3 | Be |
| Fathom Fleet Cutthroat / `e90e3f58-1197-41c1-81c5-008e946eb539#card` | 7 → 3 | Be |
| Favored Hoplite / `145a15f9-89a9-4eb7-9924-e768f17e7e68#card` | 156 → 68 | Be |
| Feint / `1bb8fe05-abb3-40a8-9e80-5d99ed0e4284#card` | 213 → 97 | Be |
| Fend Off / `fa8f3827-8cd9-4896-ab0c-26fecacceb40#card` | 58 → 28 | Be |
| Final-Sting Faerie / `299765d8-72b9-4ada-a607-efb36aa0f5db#card` | 7 → 3 | Be |
| Fleeting Flight / `6336401f-4e2d-4ebe-8c5b-24aa6f516abf#card` | 62 → 22 | Be |
| Fog / `27e9db49-7af7-4bef-ad4c-bf5dfb92030d#card` | 28 → 10 | Be |
| Forfend / `59f8f2c4-d30d-411c-9fac-a5a3219bb124#card` | 11 → 5 | Be |
| Fumble / `cbf68ddd-596f-4b2d-b93c-3b7976ba4d37#card` | 88 → 24 | Be |
| Furious Forebear / `4fb1d33f-a87d-46cf-a73e-882c3dbcb26f#card` | 15 → 3 | Be |
| Furious Spinesplitter / `f4964f74-07c0-4edc-984d-57722fb16961#card` | 33 → 13 | Be |
| General's Kabuto / `cf17889e-9201-4af7-bc2a-9539b04fea15#card` | 19 → 5 | Be |
| Genesis / `15afdd88-05fc-4335-8a55-c6ee702f3bef#card` | 15 → 5 | Be |
| Ghastly Remains / `0eafe734-19f2-492d-bfc2-d0c75ece45b2#card` | 6 → 2 | Be |
| Gigapede / `9be31785-2975-424f-aada-25ee2a0bf6d8#card` | 6 → 2 | Be |
| Glacial Crevasses / `88e0551a-ada0-41d9-b5c3-39257ce56c3e#card` | 28 → 10 | Be |
| Goka the Unjust / `5b18d2fd-bc65-49a2-b812-c217f125571d#face:1` | 7 → 3 | Be |
| Goldbug, Humanity's Ally / `d1960749-c11e-4ac4-97cb-f37d7875d479#face:0` | 704 → 192 | Be |
| Guard Gomazoa / `7c565975-aebc-4599-ac32-5594c718e2cb#card` | 19 → 5 | Be |
| Guardian Naga / `67dec976-bcf5-4995-8da1-cd570862d3cf#face:0` | 6 → 2 | Be |
| Harmless Assault / `bfcc3a16-1ca8-4112-a10e-d4e52d3daa8d#card` | 204 → 84 | Be |
| Harsh Mercy / `dcd65c9b-4aaa-42af-869f-de179ae57c9f#card` | 13 → 5 | BeNegative |
| Harvestguard Alseids / `ad88bf37-bbb4-46f0-9994-1863c7c31a2a#card` | 22 → 10 | Be |
| Hateful Eidolon / `a3b39590-34b7-4fd0-877f-748e6df30e88#card` | 20 → 6 | Be |
| Haze of Pollen / `f0140f00-5414-493b-962d-33312d49f6ea#card` | 28 → 10 | Be |
| Heroism / `74716642-fc8c-4f62-a556-154ed0b4af0e#card` | 218 → 116 | Be |
| Hidden Retreat / `a116329a-343e-4f10-a122-38bf8b5ac2c8#card` | 48 → 28 | Be |
| Hindervines / `8d13fabc-1e0f-41f2-8873-9f44b68f7e43#card` | 1,618 → 830 | Be |
| Holy Day / `98423a34-f044-4811-b288-56981d604b6e#card` | 28 → 10 | Be |
| Hooded Assassin / `a3f76c99-b608-425e-83ca-675acb7c2693#card` | 7 → 3 | Be |
| Hope of Ghirapur / `1a20e805-c80b-4710-917b-706d29ec395e#card` | 9 → 5 | Be |
| Horn of Deafening / `50a1c14a-003f-424b-bb8e-2e2d51465a90#card` | 58 → 28 | Be |
| Hunter's Ambush / `1f085891-0d99-46a6-8f09-b84f44d701f8#card` | 58 → 28 | Be |
| Indestructible Aura / `e10e8d56-bba6-412d-970e-c24969f32b5b#card` | 11 → 5 | Be |
| Initiate of Blood / `5b18d2fd-bc65-49a2-b812-c217f125571d#face:0` | 7 → 3 | Be |
| Inspire Awe / `9ca539cd-9876-4c13-b220-d553c17f2378#card` | 17,778 → 2,638 | Be |
| Inviolability / `3f336198-2425-447a-88b4-9be21ef7e90a#card` | 6 → 2 | Be |
| Jarl of the Forsaken / `294c1fed-9473-4fbe-b10e-71a7a432b98d#card` | 25 → 9 | Be |
| Kami of False Hope / `49984248-4800-4f46-933a-fedd970ae910#card` | 28 → 10 | Be |
| Kindred Dominance / `ccaa44f2-96be-44e2-884f-c31baa3908d5#card` | 3 → 1 | BeNegative |
| Kindred Judgment / `b5dce42a-a769-4d7d-b29e-8722f79d4092#card` | 3 → 1 | BeNegative |
| Knight-Captain of Eos / `611f1714-a8bb-4ce4-a810-4d26fe64e358#card` | 28 → 10 | Be |
| Kor Haven / `276cece9-f9f2-46e6-ae76-daddaa2fb9ab#card` | 58 → 28 | Be |
| Lady Evangela / `8800d672-424b-4a7b-886f-7eb9d7a56cfe#card` | 58 → 28 | Be |
| Leap of Faith / `ecf0b5ad-5989-4b99-9552-c5a24504f983#card` | 22 → 10 | Be |
| Lifeline / `f475b636-eb7f-4dd7-bc87-a1af07064ffe#card` | 240 → 80 | Be |
| Light of Sanction / `c34cf404-729d-46ef-8661-1095e5581766#card` | 70 → 26 | Be |
| Lithomancer's Focus / `a8a27a87-fc4c-44b5-9ba8-505bbf42164e#card` | 66 → 34 | Be |
| Lull / `d53f4c24-8deb-485a-b03a-945211fdf10d#card` | 28 → 10 | Be |
| Lurking Deadeye / `d1dd27eb-ba7b-4d5d-a36c-3f350df2ad51#card` | 7 → 3 | Be |
| Manticore / `fe35b55e-52ae-428e-b27e-817ee78b5d8c#card` | 7 → 3 | Be |
| Master of Death / `3a16bdda-8499-47e4-b870-b9576f4c1a83#card` | 6 → 2 | Be |
| Mind Funeral / `cf5103c1-2590-4aed-b3ae-8a570f797738#card` | 46 → 16 | Be |
| Mirrodin Avenged / `ec83f398-31af-484d-b6d2-8c7add142260#card` | 7 → 3 | Be |
| Mirror-Mad Phantasm / `fd26127d-6807-41e3-9dcf-20ef257d71d3#card` | 118 → 46 | Be |
| Moment's Peace / `b6d22228-a45e-4296-9ae2-1649e04b1c53#card` | 28 → 10 | Be |
| Morningtide's Light / `a25bcf65-4417-45e9-a4b1-94e0ea78af63#card` | 84 → 28 | Be |
| Mtenda Lion / `af029853-cfb0-403b-af51-141ba02ae2e4#card` | 58 → 28 | Be |
| Murderous Spoils / `e911f9d9-da04-4997-a8d7-449b566cd1ce#card` | 5 → 1 | Be |
| Mutational Advantage / `2daa5b89-e772-4a2b-ad52-bbbf148c7b2f#card` | 110 → 50 | Be |
| Muzzle / `bdf497b5-166f-46a7-8e18-ae0b8f97768c#card` | 14 → 6 | Be |
| Myr Servitor / `b6a50858-a013-4703-a9d7-5b69ef9bd9ae#card` | 9 → 3 | Be |
| Needle Drop / `8622a2b2-fe07-41bd-bef3-e60731e0d701#card` | 7 → 3 | Be |
| Nezumi Graverobber / `1373fa8d-0a39-4451-a482-ce2071438dc8#face:0` | 12 → 4 | Be |
| Ogre Siegebreaker / `1432db6a-9466-48f6-9011-60728c437608#card` | 7 → 3 | Be |
| Oketra's Avenger / `8f064160-3afe-408a-85b4-b335eae8571c#card` | 93 → 33 | Be |
| Old Fat Spider Can't See Me / `e4402d21-bb7f-4777-bc8d-b24bf2faccb0#card` | 76 → 40 | Be |
| Oloro, Ageless Ascetic / `620ff5f2-7d3f-467f-943d-3b62c2135023#card` | 6 → 2 | Be |
| Opportunist / `5b5524f7-3c68-4c7b-be1b-745ffcad8006#card` | 7 → 3 | Be |
| Oriss, Samite Guardian / `8d6e0dab-400a-4761-8343-92c7eb7e8735#card` | 132 → 60 | Be |
| Pack Leader / `3701ed34-a97c-4d7b-a15a-4faec02ef24b#card` | 296 → 120 | Be |
| Painter's Servant / `baf93873-35e6-4bf2-bdcc-78a5206422fb#card` | 105 → 21 | BeNegative |
| Parhelion II / `24d22bcb-8a77-4c47-a508-6f4bc093c1d0#card` | 85 → 15 | Be |
| Pause for Reflection / `a7f266b5-7258-4eaa-b447-66460ec38505#card` | 28 → 10 | Be |
| Personal Sanctuary / `20963cad-1c13-44a1-901a-bfc9930089e6#card` | 6 → 2 | Be |
| Pestilence / `dafe63ef-f3d6-45e7-877a-573da92ba85e#card` | 3 → 1 | Be |
| Pollen Lullaby / `1f64d70d-ea38-4419-be91-8b68aab3401e#card` | 672 → 240 | Be |
| Prismatic Ward / `ad5e2cb0-00d7-4dad-b20e-23bd41ada398#card` | 28 → 12 | Be |
| Pyre Zombie / `accb82be-4f90-4d9a-bafd-f8de3622b3a3#card` | 6 → 2 | Be |
| Pyrohemia / `9ac57a10-3402-4656-9079-f713884cde35#card` | 3 → 1 | Be |
| Qutrub Forayer / `500ccb61-83b0-4b1f-ada6-65850ed5a825#card` | 28 → 12 | Be |
| Raiding Party / `1d9b8859-14e3-4bbe-ad92-916b25a37b31#card` | 2,552 → 1,144 | BeNegative |
| Raise the Palisade / `f55a3781-fe33-4301-9bb5-6a54b9c13c4f#card` | 6 → 2 | BeNegative |
| Redeem / `c2032f88-a000-4ed2-a7e5-61d29723d45e#card` | 19 → 9 | Be |
| Repel the Abominable / `ed198c68-c219-469f-b133-737b060cfbfc#card` | 41 → 21 | Be |
| Rescue Retriever / `86f9897f-84aa-4c8d-8335-cfdb2f565311#card` | 48 → 16 | Be |
| Respite / `46137f00-f7f5-430a-857c-ff2dbf2d2579#card` | 56 → 20 | Be |
| Restrain / `5aa66cb0-86b9-4e85-9a17-df78416d682d#card` | 58 → 28 | Be |
| Revealing Wind / `905a6d6c-5ef8-4afa-89bb-9d85a854743f#card` | 56 → 20 | Be |
| Riftstone Portal / `8d7e05ba-5406-4d5e-bb8f-a4a6f3b0eaa7#card` | 3 → 1 | Be |
| Riot Control / `a189e20f-1072-4608-9e50-da2a9d7a7ee3#card` | 22 → 10 | Be |
| Rooftop Assassin / `f3baef8c-f8d7-4cb2-b5cc-b889a991e4e8#card` | 7 → 3 | Be |
| Root Snare / `cbf743a5-123f-4747-826d-7f8bb929a52c#card` | 28 → 10 | Be |
| Rune-Tail's Essence / `710f1f8a-e5f7-4889-9f12-2611f42d5c73#face:1` | 12 → 4 | Be |
| Safe Passage / `dfa459a1-b065-4488-88d3-4da388261b52#card` | 41 → 15 | Be |
| Safeguard / `e310c3ab-a729-404d-944f-9b477258495c#card` | 58 → 28 | Be |
| Scarecrow / `58616e15-531f-4680-8294-35ab7e962c96#card` | 116 → 66 | Be |
| Seraph of the Sword / `84ee74e9-b3e3-4f47-8b3c-5c1a70f5cad6#card` | 19 → 5 | Be |
| Sevinne, the Chronoclasm / `869c9bc4-2b21-40c3-8b1c-c83269c856d0#card` | 1,560 → 520 | Be |
| Shielded Passage / `a6933789-bb4f-4d45-975e-402b8e0b0e68#card` | 11 → 5 | Be |
| Shieldmage Advocate / `57188b3d-567c-4dff-8b93-7cc7a47894be#card` | 220 → 120 | Be |
| Skyclave Shade / `2345b243-ed0d-4556-8017-d533db1fbc74#card` | 162 → 54 | Be |
| Solitary Confinement / `d41ab41d-07c8-4f6d-be5e-7aec9f5f6365#card` | 18 → 6 | Be |
| Songstitcher / `eec72bcf-ccbc-43fc-8bdd-7bf4faa1fad7#card` | 513 → 235 | Be |
| Spectacular Showdown / `b6f61b77-d571-4508-a5e8-435b4e097f3a#card` | 19 → 14 | Have |
| Spirit of Resistance / `b075b645-2e5a-4d91-9440-02db4e6f5d08#card` | 6 → 2 | Be |
| Spore Frog / `97db6c39-e690-49b6-93a6-e51b8dfad10b#card` | 28 → 10 | Be |
| Stingblade Assassin / `7938112a-3832-42b4-b123-27a42c06a290#card` | 7 → 3 | Be |
| Stonewise Fortifier / `ecfa6791-938a-4159-92bf-ff2b0ce69523#card` | 27 → 15 | Be |
| Summon: Alexander / `b291d046-7649-4a05-98c4-224ecaece912#face:1` | 29 → 11 | Be |
| Sunstone / `300ad0f7-92a7-450d-a9de-b658836fe3cb#card` | 28 → 10 | Be |
| Swift Demise / `7eaf3fd0-e141-42c2-874d-b471f4a94ad2#card` | 7 → 3 | Be |
| Tangle / `f627e125-15af-4e53-b34e-82b60e4ec87b#card` | 168 → 60 | Be |
| Tanglesap / `a533df83-782f-4f77-a0be-312ae56f6447#card` | 411 → 193 | Be |
| Telekinesis / `1b9dd2b6-d14d-4c1e-9885-00ab2c0bf8da#card` | 348 → 168 | Be |
| Temporal Isolation / `cb5ab978-d9b1-4f2f-bf3a-5e28e96b9e2e#card` | 14 → 6 | Be |
| The Girl in the Fireplace / `39068bf1-cae4-4ac9-82ef-4e81bf59446e#card` | 900 → 300 | Be |
| Thwart the Enemy / `5799baed-3457-4bf2-adf3-239a41dcc1c8#card` | 60 → 32 | Be |
| Time to Reflect / `7ce28a3c-5f93-4434-918e-1e74f9fc71d8#card` | 24 → 14 | Be |
| Trained Pronghorn / `22528380-4f47-4bd9-a1d4-79f700cba4f5#card` | 11 → 5 | Be |
| Tresserhorn Skyknight / `df344469-1177-4168-91af-3621b2a65182#card` | 75 → 35 | Be |
| Uncle Istvan / `44e855a4-95f9-4515-a868-a8ad7a2db751#card` | 20 → 8 | Be |
| Unsparing Boltcaster / `0a56a33b-033f-4398-8dc1-e531be059710#card` | 7 → 3 | Be |
| Valor / `b2ac84e3-cc3c-49c6-918b-a407ef1ee06c#card` | 3 → 1 | Be |
| Vengeful Firebrand / `920e21c4-0116-416e-a375-bb887cb7b44a#card` | 24 → 4 | Be |
| Vengeful Pharaoh / `7e465cfe-d2ee-4420-87f4-945728f58f87#card` | 12 → 4 | Be |
| Vraska's Finisher / `5df16529-6f1d-4978-b364-97d91abac441#card` | 25 → 9 | Be |
| Wall of Putrid Flesh / `e31d4be7-cd24-4287-b8ca-66f8612731a6#card` | 20 → 8 | Be |
| Warning / `c5ba0f0f-65c5-4ffa-987a-f320b401ec8f#card` | 58 → 28 | Be |
| Well-Laid Plans / `50343947-d127-4479-9c1b-8f5040a97afb#card` | 47 → 23 | Be |
| Wellgabber Apothecary / `10cdb0af-a206-4eec-ba9b-634f6ec583d8#card` | 33 → 15 | Be |
| Windwright Mage / `2f051ee4-9f2e-414d-b763-5fd78f2d4e1b#card` | 12 → 2 | Be |
| Winnow / `1b2ad355-3f1e-4c8b-93ee-848934646b23#card` | 12 → 2 | Be |
| Wirecat / `57828973-df3d-4288-988d-147a3016120e#card` | 53 → 7 | Be |
| Witch's Mist / `4327225e-140f-45a4-a56f-dcf472f6c062#card` | 7 → 3 | Be |
| Wonder / `232284f7-c623-4895-9ab9-8b1a39926830#card` | 3 → 1 | Be |
| You Are Already Dead / `c1157d79-38ea-4a88-b346-7b12614ff78a#card` | 7 → 3 | Be |
| You Cannot Pass! / `8e94ccdb-5978-440b-b7cd-11c7ca6b2c98#card` | 24 → 14 | Be |
| Zack Fair / `e84d146a-1c67-49df-b062-622794e45c42#card` | 60 → 20 | Be |

The 18 newly gained faces affected by the same exclusion retain the following
valid Readings; their before counts here are the pre-exclusion Instead
candidate, not the claim-parent census (which covered none of these faces).

| Face / identity | Candidate → final Readings | Retired auxiliary head(s) |
|---|---:|---|
| Ascent of the Worthy / `d748bad4-dd4c-4553-9fcc-e462260f6ff3#card` | 15,120 → 5,040 | Be |
| Crafty Cutpurse / `2ef53e18-0baf-4eee-b671-4c19b464cbfb#card` | 80 → 60 | Be |
| Empyrial Archangel / `3a105959-dfce-4202-b37f-ae635dfcb30b#card` | 18 → 6 | Be |
| Gideon's Sacrifice / `1af63a5e-2bec-4f8d-a373-d9ce43a7d242#card` | 276 → 132 | Be |
| Heroic Sacrifice / `03c47f1c-02a7-428c-a126-9e85325ebc71#card` | 288 → 96 | Be |
| Kor Chant / `5b3f6817-5d7a-4d83-ad1d-df75b4e1970b#card` | 564 → 300 | Be |
| Kor Dirge / `44ae1c25-8622-4a58-92e0-12d76f084718#card` | 564 → 300 | Be |
| Martyrs of Korlis / `7ca54a23-f8eb-4982-b4ee-7392e2f2a1b3#card` | 64 → 32 | Be |
| Oracle's Attendants / `f4bc6674-8586-4dd9-ad3a-87ba704fda7a#card` | 66 → 42 | Be |
| Palisade Giant / `f38fe1e9-8997-4b63-9109-7513034bac88#card` | 54 → 18 | Be |
| Pariah / `2c5c8250-1860-42a1-a335-071f54830d37#card` | 18 → 6 | Be |
| Pariah's Shield / `f2103ab8-a183-4db8-98dd-4146217b5125#card` | 18 → 6 | Be |
| Reverberation / `d3088e1d-62c9-4478-9ef7-fc3c9e5cfadb#card` | 57 → 33 | Be |
| Saving Grace / `50ddbea3-7ef4-4f6a-83c9-0c3ea1dfa3c9#card` | 184 → 88 | Be |
| Shimian Night Stalker / `09b9e6fd-7a61-4ed4-a121-61b64fbf03f4#card` | 33 → 21 | Be |
| Treacherous Link / `b7902113-9ddf-4d81-a76a-b54b8a36c154#card` | 18 → 6 | Be |
| Turn the Tables / `bd5ad7c3-4477-4278-9875-d2ffbb0e8089#card` | 36 → 18 | Be |
| With Great Power . . . / `dfedb968-f27c-4117-aff6-da707dd43e82#card` | 108 → 36 | Be |

The lost-face list is `[]`; thus there is no sole-wrong-Reading coverage loss
to classify. The six previously covered compound-PP faces preserve their
counts while acquiring the nested Of constituent: Lapse of Certainty 6 → 6,
Memory Lapse 6 → 6, Hinder 72 → 72, Spell Crumple 24 → 24, Remand 6 → 6,
Desertion 36 → 36. Their full identities are retained in the comparison.

Every one of the 343,618 counted final Readings passes declaration admission,
lexical ownership/context, byte-exact realization and construction/leaf
traversal identity against materialization traces. Enumeration is complete
for all 32,828 faces: zero limits, issues, cyclic derivations, duplicate
Readings or internal failures. Independently authored bare Instead and
Library of Leng values prove value → realization → retained equal Reading;
the bare value also compares complete node and lexical traversal. Library
of Leng preserves its exact-value assertion against the replacement shape.
Independent lexical validation successfully checks 42,540 declared values.

The fresh Pariah regression passes. Its Instead-less twin loses its four
malformed Readings and retains the exact two good trees (checked against the
pre-exclusion probe); its two legitimate PP attachment sites remain. Do-support
`If you do, draw a card.` and Roots of Wisdom's full attested *can't* context
remain positive controls. Stranded be/have/modal negatives and the mixed
licensed/unlicensed shared-head negative remain rejected.

Forbidden word/card/lexeme/construction-named admission guards added: zero.
New constraints read declared licences, realization and marker features.
Source/environment loading succeeds with zero errors; the unchanged 87
unmapped plugin declarations are disclosed residuals, not load errors.
The v3 authority is `deckmaste_lexical_source::load_workspace` and generated
feature checks. It has no legacy `environment.rs`, emitted legacy
permitted-licensing-checker total or active legacy coverage lock; those are
not fabricated or obtained from the retired coverage pipeline.

### DISCLOSE

| Measured tree / covered | Unread | Unique | Multiple | Complete Readings |
|---|---:|---:|---:|---:|
| `pmpwnupr`, claim-parent `pyoyosmv` grammar / 17,322 | 15,506 | 7,111 | 10,211 | 367,247 |
| `pmpwnupr`, final / 17,556 | 15,272 | 7,122 | 10,434 | 343,618 |

Specificity-resolved selections: zero before and after. All grammatical
Readings survive; no destructive selection or construction preference is
introduced. Clause-final and predicate-final attachments coexist, including
attachment to a coordinated predicate or its final predicate.

Every gained identity, face name, selected analysis and complete Reading
count is in [instead-landing-identities.md](../../../target/english-v3/english-v3-instead-replacement/instead-landing-identities.md).
`instead-resolved-selected-audit.json` retains sources, fingerprints and
owner paths. All 234 representatives were inspected. The full-Reading gain
audit (`instead-resolved-gain-audit.json`) checks all alternatives, finding
zero nominal Instead attachment and zero stranded unlicensed auxiliary.
Selected examples, stamped by the final tree/covered count above:

| Face | Selected connective attachment | Readings |
|---|---|---:|
| Topography Tracker | clause-initial PP | 3 |
| Lotus Vale | clause-final PP | 3 |
| Burn the Accursed | clause-final PP | 6 |
| Soldevi Excavations | clause-final PP | 3 |
| Increasing Savagery | clause-final PP | 6 |
| Divine Resilience | clause-initial PP | 24 |
| Scorching Dragonfire | clause-final PP | 6 |
| Bilbo, Fellow Conspirator | clause-initial PP | 1 |
| Ascent of the Worthy | clause-final PP | 5040 |
| Lofty Denial | predicate-final PP | 36 |
| Pariah | predicate-final PP | 6 |
| Crafty Cutpurse | clause-final PP | 60 |

Witness-unit counts (not whole-face counts), complete with zero issues.
All six quoted witnesses had zero Readings before:

| Witness | Final with Instead | Twin before | Twin after |
|---|---:|---:|---:|
| Forbidden Crypt, first replacement sentence | 9 | 3 | 3 |
| Burn the Accursed | 6 | 2 | 2 |
| Soldevi Excavations | 3 | 1 | 1 |
| Increasing Savagery | 6 | 2 | 2 |
| Divine Resilience | 12 | 9 | 9 |
| Reality Twist, full sentence | 0 | 0 | 0 |

Reality Twist's attested `instead of any other type` constituent reads with
its full AccusativePhrase complement. Its full sentence and twin independently
lack Produce (four unknown occurrences) and the reduced `If tapped for mana`
host. These causes and Forbidden Crypt's other `from anywhere` unit remain
with `english-v3-systemic-residuals`; no wrong host analysis is counted as a
gain. The quoted Forbidden Crypt first sentence reads while the full face
remains unread.

The surviving sole lexical analysis is `vocab:Preposition/Instead`, an
intransitive Compound Preposition with an optional selected Of PP. CGEL
Ch. 7 §3.1 pp. 622–623 supports its coalesced compound status; Ch. 7 §2.4
pp. 616–617 supports the full PP Complement and omission of that PP, including
its head. Ch. 8 §19 p. 778 lists it as a connective adjunct. The lexical
ClauseInitialAdjunct licence and its unpunctuated Oracle use are project
analyses grounded in Divine Resilience, not additional CGEL claims.

Retired on both sides: `vocab:ReplacementMarker/Instead`, its vocabulary
category mapping, Deal's unconsumed OptionalRole slot, the flattened lexical
marker field of CompoundPrepositionPhrase, and Of's obsolete
CompoundComplementMarker property. Compound selection now reads the existing
NominalComplementMarker through a declared table. Deal's ReplacementMarker
slot is retired, not consumed as a verb Complement: Instead is a connective
Adjunct. MassNoun and DistributionPhrase remain with systemic residuals.
No ReplacementMarker declaration remains.

STOP and resolution (dated): the initial full-Reading audit exposed 11,326
stranded-Be Readings on 18 gains. Implementation stopped because general
ellipsis lay outside the ticket and the maybe-ticket required a dialogue.
The **orchestrator resolution of 2026-10-07**, applying the **user ruling of
2026-10-06**, extended this item's scope: “reject stranded be, have and modal
ellipsis; preserve do-support” and *can't*. Ground, quoted from the user:
“it's 'would be dealt' not damage [that would be] [dealt to ...]”. The relayed
survey reports 1,556 do-support faces and 30 *can't* faces; those are ruling
provenance, not this landing's survey. Contextual recoverability is explicitly
not implemented. This is a user/project ruling, never attributed to CGEL.
The STOP is resolved by the declared licence and the complete attribution
measurement above; no unresolved STOP remains.

Deviations and additions:

- Add ordinary PrepositionComplementPreposition to represent Of with an
  internal PP Complement. The declaration contract fixes field categories
  across forms; the separate constructor preserves the complete selected PP
  and existing nominal complementation.
- Reshape CompoundPrepositionPhrase, propagate ClauseInitialAdjunct through
  PPs and coordination with default No, and add the feature-licensed
  unpunctuated initial form. Extra Wheel of Sun and Moon / Darksteel Colossus
  consequent witnesses verify coordinated predicates and retained scopes.
- Under the explicit scope extension, declare Auxiliary Ellipsis Licence on
  do-support and negative can only, enforce it for direct and shared auxiliary
  heads, and propagate the conjunction of licences through head coordination.
  Retire unreachable OmittedGerundParticiple, OmittedPastParticiple,
  ProgressiveEllipsis, PassiveEllipsis and PerfectEllipsis. Overt participial
  and perfect Complements remain; BareEllipsis/OmittedPlain remain licensed.
- Re-spell the Library of Leng expected value and the existing participial
  omission negative test against the replacement shape. Preserve its authored
  admission exclusions, additionally check absence of both empty complement
  categories, and include Have under the resolved ruling. The new Pariah twin
  retains both good attachment values; added do/*can't* positives and
  be/have/modal/shared-head negatives verify the scope extension.
- Update the owning glossary's Ellipsis definition and add Auxiliary Ellipsis
  Licence with its avoid-line, explicitly crediting the dated ruling. This
  closes the one glossary gap exposed by the extended landing.
- Move `english-v3-ellipsis-recoverability` from maybe to done with its two-line
  record pointing here, and record this exclusion's measured resolution in
  planned `english-v3-systemic-residuals`. No contextual-discharge algorithm,
  plugin body or unrelated grammar frame is changed.

Assurance counts: restored 0, re-spelled 2, added 7, newly ignored 0, removed 0.
The pre-existing ignored gate test
`macros::templates::tests::macro_schema_census_count_matches_21` retains its
blocker: on-demand cross-check of the live corpus against the census.

### REPORT

Corpus SHA-256 in both full reports:
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
Baseline inventory SHA-256:
`50a975d9d292bd934306fe90df2485e942ae298fe8630f8d0884ec89b2399210`.
Final inventory SHA-256:
`be4f999c2d3a8ab499939573257d8a4fa7c3c26cb69daea424783e447f35263a`.
`instead-resolved.json` and `instead-resolved-comparison.json` are the final
authority; the earlier candidate reports retain the STOP's provenance.

Construction inventory: before 196 ordinary + 44 schemas = 240; after
192 + 44 = 236 (one addition, five retirements). Static productions
601 → 598; lexically compiled 669 → 666. Legacy lock `covered` is not a v3
metric; the stamped validated corpus covered counts are 17,322 → 17,556.

Declared-surface homographs 397 → 395; case-folded 360 → 359. Removed named
homographs: `instead`, `Instead` (declared), `instead` (case-folded); former
owners were Preposition/Instead and ReplacementMarker/Instead. The complete
named lists and owners are in [instead-homographs.md](../../../target/english-v3/english-v3-instead-replacement/instead-homographs.md).
Form-literal/vocabulary overlap inventory is `[]` before and after. The seven
changed lexical owners are Instead, Of, Deal, Do, DoNegative, Didnt and Cant;
the sole removed owner is ReplacementMarker/Instead, with zero added owners.

Performance advisory, integer nanoseconds; 12 workers in both full runs:

| Measured tree / covered | Corpus wall ns | Thread CPU ns/B | Host load (1 / 5 / 15 minute) |
|---|---:|---:|---|
| `pmpwnupr, claim-parent pyoyosmv grammar` / 17,322 | 62,340,411,506 | 312,146 ns/B | 8.49072265625 / 11.56005859375 / 10.60693359375 |
| `pmpwnupr, final` / 17,556 | 54,774,767,251 | 272,176 ns/B | 7.60986328125 / 5.5029296875 / 6.087890625 |

Both exceed the 16,260,000,000 ns quiet-host ceiling on loaded hosts. This is
an advisory, not a fitted gate or a quiet-host comparison. Separate setup:
before 8,189,922,478 ns; after 11,210,327,704 ns.
All census, evidence and scratch audit files are under this workspace's
ignored `target/english-v3/`; retirement retains them in the coordinator's
ignored `target/english-v3/english-v3-instead-replacement/`. No evidence is
tracked or embedded into source crates.

Validation: focused Instead/participial/preposition tests passed (24 tests).
Clippy for the four derived packages with `--all-targets` passed. The derived
command is `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`,
run with `cargo xtask gate --changed --from pyoyosmv --run` and
`CARGO_TARGET_DIR=target/instead-build`. The derived gate passed (exit 0): 759 passed, zero failed, one unchanged
ignored census test with its existing blocker above. Focused tests additionally
passed on the final test source. Changed-file nightly formatting passed.
From the workspace root, `cargo xtask cite check --list-noncompliant` reports
zero noncompliant strings and `cargo xtask cite check` checks 16,112 citations
with zero stale. `jj diff --git | cargo xtask cite audit --diff` audits zero
changed CR citation sites; no blessing is needed.
