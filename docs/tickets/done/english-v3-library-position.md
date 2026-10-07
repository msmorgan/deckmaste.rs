---
needs: []
---
# Read library positions with Look, Reveal, Exile and Put

## Why

Library-position phrases never read: *the top N cards of your library*, *the
top card of …*, *on top of …*, *on the bottom of …*, *the rest*, *in any
order*, *in a random order*. On change `wlvwtnppyovn` (32,828 supported faces,
13,716 covered, 19,112 unread) they occur in a failing unit of 1,304 unread
faces and are the only recognised cause on 298. These are surface-bucket
counts, not gain forecasts. Probes on that change ("Look at the top five cards
of your library.", "Exile the top two cards of your library.", "Put target
nonland permanent on top of its owner's library.") each have zero admitted
roots. `vocab:Adjective/Top` and `vocab:Adjective/Bottom` exist, and the
lexicon's `frame_markers` already map `in` and `order`.

This ticket takes over from `english-v3-systemic-residuals`:

- the Look row `Preposition(At), Complement(Object)` and the two Put rows
  carrying `Preposition(On), Complement(FrameComplement)` (one ending
  `Preposition(In), Complement(ArbitraryDeterminer), CommonNoun(Order)`) of the
  unsupported-inventory table in the done `english-v3-generic-frame-consumption`;
- 29 of the 42 inherited frame-coordination faces, which fail first on a
  library position: Commune with Evil, Strategic Planning, Beast Hunt, Vigean
  Intuition, Mulch, Discerning Taste, Tracker's Instincts, Murmurs from Beyond,
  Maestros Charm, Confounding Riddle, Winding Way, Sultai Soothsayer, Forbidden
  Alchemy, Taigam, Sidisi's Hand, Resentful Revelation, Scattered Thoughts,
  Tamiyo, Collector of Tales, Pieces of the Puzzle, Firja, Judge of Valor,
  Ancestral Memories, Ransack the Lab, Organ Hoarder, Borborygmos Enraged,
  Shadow Guildmage, Testament Bearer, Rakshasa's Bargain, Bitter Revelation,
  Kruphix's Insight, Glimpse the Future. Their identities and earlier evidence
  are in `/tmp/frame-coordination-inherited-reconciliation.json` if it still
  exists. Re-measure; do not depend on it.

## Goal

Library positions read as ordinary Noun Phrases and Preposition Phrases
selected by Look, Reveal, Exile, Put (and Mill/Manifest where they already
take an NP), with the existing typed frame slots reconciled rather than
bypassed. "Put … on top of / on the bottom of …" consumes Put's declared
destination slot. "in any order" / "in a random order" consumes Put's declared
order tail.

## Analysis

In *the top two cards of your library*, *top* is an attributive Modifier of the
Nominal *cards* and *of your library* is NP-internal. The ADR licenses *of*
only NP-internally, as Modifier or Complement of a noun
(`docs/decisions/english-lexical-analysis.md`, Preposition Function Licence
amendment; CGEL, Ch. 5, §14.2, p. 446, [14i]). *On top of X* is a
Prep + N + Prep + X idiom of the *in front of X* type (CGEL, Ch. 7, §3.1,
pp. 618–623; *on top of* is discussed on p. 621). Choose between the
right-branching and layered-head structures by CGEL's tests (pp. 621–623), and
do not lexicalise *on top of* as one Compound Preposition without that
evidence. *On the bottom of X* is a regular PP whose NP has the noun *bottom*
as head with an *of* Complement. *In any order* is a manner PP (CGEL, Ch. 8,
§2.1, p. 671, "NPs and PPs"); with Put it is the declared frame tail, not a
free Adjunct, because Put's frame selects it.

## Witnesses

- Commune with Evil: "Look at the top four cards of your library."
- Reckless Impulse: "Exile the top two cards of your library."
- Totally Lost: "Put target nonland permanent on top of its owner's library."
- Hide: "Put target artifact or enchantment on the bottom of its owner's
  library."
- Winding Way: "Reveal the top four cards of your library." (its other failing
  unit, "Choose creature or land.", stays out of scope)
- Psychic Surgery: "Whenever an opponent shuffles their library, you may look
  at the top two cards of that library." / "Then put the rest on top of that
  library in any order."

The first four witnesses each have only the quoted failing unit.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/library-position-before.json` on the claim
   parent, stamped with its change id.
2. Write the witnesses as tests first. Assert the slot each phrase fills, not
   just recognition.
3. Iterate on `--face-id` selectors (the witnesses and the 29 inherited faces);
   verify on `--all` at the end. Report, by name, which of the 29 now read and
   the next failing cause of each that does not.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *of your library* read as a
   clause Adjunct, or *in any order* as an Adjunct of Look, is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *from among them* and other From-source slots (deferred in
  `english-v3-systemic-residuals`; owner of re-application:
  `english-v3-selected-preposition-nominal-licensing`).
- Ordinal positions ("third from the top", Enigma Sphinx, Lost Hours) and
  "their choice of the top or bottom of their library" (Uncharted Voyage).
  Record whether they read afterwards; do not build for them.
- `, then` links (owned by `english-v3-then-sequencing`).

## Landing record

Completed 2026-10-06 on `xltvxkxkvsyknxnkrztqqkqpqsvzzvrp` (`xltvxkxk`).
All final numbers below describe that tree with **15,997 covered faces**;
baseline numbers describe the same claimant change with **15,792 covered**.
The baseline consumed grammar and lexical declarations were identical to claim
parent `nmtxxkuv`; the report names the claimant because the initial witness
had already been added there. The ticket's historical 13,716 baseline is
superseded by this measurement, not a gain attributable to this landing.

### PROVE

Across the same 32,828 supported faces: **205 gains, zero losses**. No identity
stops being covered, so no retirement or re-coverage debt is created.
`library-position-final-delta.json` names every gained identity. Intermediate
`library-position-after.json` (`xltvxkxk`, 15,983 covered) is superseded:
it found 14 passive regressions
which were repaired before landing. The repaired identities are Political
Triumph, Blossombind, Construct a Cosmic Cube, Claim the Kingdom, Fairgrounds
Trumpeter, Shang-Chi and the Ten Rings, Melira's Keepers, Fathom Mage, Tatterkite,
Agitator Ant, Blightbeetle, Darksteel Angel, Flourishing Defenses, and Wakka,
Devoted Guardian. Their Object Gap/passive destination now consumes the
Object-Locative frame through `SelectedGapLocativePredicate`.

Every one of the **245,342 final Readings** passes declaration admission,
lexical ownership/context, byte-exact realization, and construction/leaf
traversal identity against independent materialization traces. Enumeration is
complete on every face; issues, internal failures, cyclic derivations,
duplicates and limited/failed enumerations are zero. The independently authored
`on top of that library` value proves the converse roundtrip law and complete
Reading-set equality, with exact node and word traversal equality. Selected
slot witnesses inspect every retained Reading, including the negative Look
order-tail case. Bare position nouns cannot become bare count Subjects or
Objects. The current ADR requires retention of every grammatical alternative
and supersedes the old zero-tie/destructive-selection prescription.
Post-landing review found ten faces had lost their complement-cluster Reading;
the follow-up below restores them.

Forbidden word-, lexeme-, card- or construction-named admission guards added:
**zero**. Guards use declared features and frames. Source/environment loading
succeeds with zero errors; the lexical export independently checks **42,536**
values. The active v3 command has no legacy `environment.rs`, coverage lock or
emitted legacy permitted-licensing-checker total. Those obsolete counters are
not fabricated or borrowed from the retired parser. The v3 authority remains
`deckmaste_lexical_source::load_workspace` and declaration admission.

### DISCLOSE

| Measured tree / covered | No Reading | Unique | Multiple | Complete Readings |
|---|---:|---:|---:|---:|
| `xltvxkxk` baseline / 15,792 | 17,036 | 6,924 | 8,868 | 237,401 |
| `xltvxkxk` final / 15,997 | 16,831 | 6,986 | 9,011 | 245,342 |

Specificity-resolved selections: **zero before and after**. No construction
pair receives destructive preference; uniqueness is a census. A representative
of every newly covered face was inspected against its source, framed leaves
and constituent inventory; all its Readings also underwent structural
validation. Complete evidence is `library-position-gains-final.json` and its
named audit `library-position-gain-audit-final.json`; refreshed evidence is
`library-position-refreshed.json` and `library-position-gains-refreshed.json`.
The refreshed full census has zero changed face census results, Reading counts,
construction counts or issues compared with the pre-refresh final.

Selected-analysis keys for the identity list:

- **D** (145): Put's `SelectedPredicate` consumes Object NP +
  `LocativeComplement`; its ordinary On PP contains a noun-headed Top/Bottom NP,
  with Of selected internally when present.
- **M** (13): that destination plus the selected In marker and
  `MannerComplement` NP tail, in Put frame 4. It is not a free Adjunct or an
  NP Postmodifier.
- **C** (21): the same selected destination with the existing choice-of
  Top/Bottom nominal coordination; no new choice production.
- **F** (14): the existing selected Reveal/Exile Object contains a From PP
  whose determined Top NP selects Of internally. Existing From attachment
  licences remain, as the pruning-rule deferral requires; this landing does
  not claim source-frame reconciliation.
- **O** (11): existing `SelectedObjectPrepositionPredicate` and ordinal
  realization with the newly available determined Top noun. No ordinal
  production was added.
- **N** (1, Telling Time): existing Put Into frame with NP coordination;
  positional PPs are nominal modifiers of the coordinated quantities. This
  grammatical, game-semantically odd Reading remains admissible under the
  ADR's 2026-10-04 amendment. It does not discharge Telling Time's intended
  complement-cluster obligation, which remains with
  `english-v3-systemic-residuals`.

Named structural spot-checks include Totally Lost and Fallow Earth (selected
bare Top destination, noun frame 1); Hide (determined Bottom destination,
noun frame 1); Psychic Surgery, Misinformation, Agonizing Memories and
Petals of Insight (selected destination and order tail); Songbirds' Blessing
(selected Bottom destination and random-order tail); Mwonvuli Beast Tracker
(selected bare `on top`, noun frame 0); Territorial Bruntar and Mind Funeral
(Top noun inside the retained From NP-internal attachment); Enigma Sphinx and
Lost Hours (existing ordinal complement); and Uncharted Voyage (existing
choice nominal in the selected destination). Commune with Evil's Look and
Reckless Impulse's Exile were already covered at this baseline; their selected
NP slot and internal Top adjective/Of PP now have explicit structural tests.
Winding Way's quoted Reveal constituent passes independently of its unchanged
whole-face failure.

Every newly covered face follows; Reading counts are full Document counts.

| Identity | Card face | Selected analysis | Readings |
|---|---|---|---:|
| `00749bad-dded-4fec-bf34-ff14cf561c26#card` | Errand-Rider of Gondor | D | 1 |
| `00e25126-b807-4753-ba63-9a533755e5ce#card` | Lantern of Revealing | D | 4 |
| `0339bd11-ad71-4998-9b5d-a32790f0e5e3#card` | Barkform Harvester | D | 1 |
| `03dc8820-ec10-42c1-8200-d323cfcbf2b0#card` | Misinformation | M | 2 |
| `03f04c2c-1036-497e-8b13-06c61b0b0ba6#card` | Territorial Bruntar | F | 92 |
| `05a4acfd-3bf3-4ec2-a28a-bc43ba7c6fbf#card` | Fallow Earth | D | 1 |
| `08dfe42e-35c0-4be0-abba-57269792ff3d#card` | Chittering Rats | D | 1 |
| `09eec5a2-7835-4b24-9dd1-594612ee9152#card` | Misleading Motes | C | 5 |
| `0a35df9d-80a0-4b6f-91d5-dad980af47e1#card` | Footbottom Feast | D | 4 |
| `0ac7bacd-944d-44f4-927b-28e8c0a4371d#card` | Psychotic Episode | D | 4 |
| `0cc38d20-5ae8-413a-9f26-ff13cd92f24c#card` | Bone Harvest | D | 16 |
| `0d09049a-6786-4b69-b448-35efaa926a71#card` | Soaring Hope | D | 1 |
| `0d96e4bc-ff4e-4f37-89b6-8a4e3c1069a0#card` | Grasp of Phantoms | D | 1 |
| `10426070-2e6f-47a5-bc8d-16dcfb058b26#card` | Volcanic Spite | D | 1 |
| `1056b27c-125c-4daa-a0c8-533efc9ccc32#card` | Vanish from Sight | C | 5 |
| `1080c5b5-6651-4c6a-93e6-099fbe389e26#face:0` | Murderous Rider | D | 2 |
| `11e70692-2057-49c8-90e6-953362f4817f#card` | Hei Bai, Forest Guardian | F | 5060 |
| `15fbb7b1-c62d-4f82-9f35-2c10299779f4#card` | Lurking Predators | D | 2 |
| `16f6438d-2a29-41cb-bf0c-4d02bd66112b#card` | Hermit Druid | F | 32 |
| `170ee399-d3cd-40ef-a340-193c0f0f6002#card` | Trickster's Stratagem | F | 16 |
| `19053c47-97ce-4a25-a92c-1f4061e85bf3#face:0` | Warrant | D | 1 |
| `192f3c8a-1120-4f7f-ae0f-fe9bde59cd08#card` | Telling Time | N | 42 |
| `1ac6fabb-71db-442f-8c14-ba9e659f906c#card` | Chrome Companion | D | 2 |
| `1b3fb20a-e090-4286-9c03-6b71c27c45be#card` | Mortuary Mire | D | 2 |
| `1b721ad3-d0f6-4eec-9bf0-f57ea9dfa392#card` | Lapse of Certainty | D | 6 |
| `1d05d2d6-3ea1-4d8a-9f1c-6fc3f5e38ea9#card` | Preferred Selection | D | 9 |
| `1d6796ce-9316-430a-8126-8b0414cf181c#card` | Uncharted Voyage | C | 5 |
| `1d7a4e9c-b3c9-421e-b5f4-82d51d7d5125#card` | Temporal Spring | D | 1 |
| `1eb88d87-e024-47fe-bd63-0eac69c106f6#card` | Petals of Insight | M | 6 |
| `1f3fb367-60b5-4b36-9604-3064afdac0d9#card` | Avenging Druid | F | 372 |
| `23b36aef-961f-4a78-9652-8594ad73102e#card` | Plow Under | D | 1 |
| `24982bfe-5276-44a8-99a5-43b497ebf51f#card` | King Crab | D | 1 |
| `24a78116-d5e0-4e30-b192-d8bffe347a0c#card` | Drafna's Restoration | M | 4 |
| `24dbddad-998b-4755-b356-4c8aca3592b1#card` | Flitting Guerrilla | D | 5 |
| `2598294c-a9a7-4bef-a562-23f297a80536#card` | Songbirds' Blessing | M | 23 |
| `26d6a089-b867-4b89-be8e-2ad11bff0d64#card` | Meldweb Curator | D | 7 |
| `27a3faa1-4f1a-4581-9e19-9265faa9c10b#card` | Painful Memories | D | 8 |
| `2a9a05b4-1910-4f97-a0fe-0f57017f6df0#card` | Jeskai Charm | D | 6 |
| `2ac9d397-10ca-43ce-bff9-45d631553486#card` | Agonizing Memories | M | 8 |
| `2fc070dc-f2f7-4648-8069-31d74790a39c#card` | Hall of Heliod's Generosity | D | 2 |
| `2fe0ebf5-52ed-4e92-9b93-81e9ae564439#card` | Ultimate Nullification | D | 4 |
| `30cc8f7b-3c28-40f5-8f8f-157e8212280b#card` | Time Ebb | D | 1 |
| `319b4715-c3eb-49bf-b8d2-4d41e601ebe4#card` | Wayward Soul | D | 1 |
| `31a331a0-3065-4ecb-9e6a-ab11e241833b#card` | False Mourning | D | 1 |
| `31f32afa-5b2a-4218-ae7b-642e1cac3911#card` | Forever Young | D | 4 |
| `324889d6-c857-41ed-bb60-408809fc9964#card` | Bant Charm | D | 2 |
| `3415a199-05ac-4627-be97-f842eb1415a3#card` | Avenging Angel | D | 1 |
| `361b965f-2ce7-49f8-84e7-7325ea0c948d#card` | Bow of Nylea | M | 12 |
| `370974e7-d242-4f24-a031-cba71c935306#card` | Ardent Dustspeaker | D | 30 |
| `38a2ef3e-3311-4e0a-807d-e5ccdd05af49#card` | Nevermaker | D | 1 |
| `3901bf30-b7c1-4977-a7b1-fcdafcc266cd#card` | Chimney Imp | D | 1 |
| `3a35e772-2547-41a3-a2a2-125b48c23aa5#card` | Temporal Eddy | D | 1 |
| `3b3c5fc6-6c4a-4e23-9625-5954d1986160#card` | Keeper of the Cadence | D | 10 |
| `3bc47eae-cf6f-4b5e-96e1-94a4f307f474#card` | Lost Days | O | 8 |
| `3ceac6ce-acde-42ab-b79d-ff153074d2d8#card` | The Spot's Portal | D | 4 |
| `3dadb654-6191-4ad3-8315-46f5613fbc01#card` | Biblioplex Assistant | D | 7 |
| `3dd196b6-a85a-4e3e-bb57-ec34241f8117#card` | Terminus | D | 1 |
| `3e00326d-2bf4-4731-a6cd-1c015f94f553#card` | Anchor to the Aether | D | 1 |
| `3f453aca-e14f-4460-9334-f0c3f4256a0e#card` | Mystic Repeal | D | 2 |
| `400adeec-11b4-4a9c-9466-dcab524d87ea#card` | Harmonic Convergence | D | 1 |
| `429de3c3-3f2e-4d55-a04a-f414912aff3a#card` | Nightscape Apprentice | D | 2 |
| `48369aec-a991-4bef-8554-01c84302b063#card` | Aetherspouts | C | 5 |
| `49f292ab-0533-436b-a309-8bc795b66e5c#card` | Riptide Shapeshifter | F | 82 |
| `4a3f1110-919c-4fd0-8154-9c9441a7d770#card` | Lost Hours | O | 8 |
| `4a60db66-0347-4efc-8507-ab7ed8d984fe#card` | Transplant Theorist | D | 1 |
| `4ab1411e-b255-4a6e-b81a-5de7b2d4ea12#card` | Sunscape Apprentice | D | 2 |
| `4b78222f-6973-42a3-987e-eb8cc8fd6c76#card` | Shivan Wumpus | D | 1 |
| `4edbaac4-dcb3-40b4-abd7-ff2683bd366d#card` | Champion of Stray Souls | D | 7 |
| `4fd493b3-63f9-4633-8f88-8755e489da32#card` | Conjurer's Bauble | D | 2 |
| `4fd567cb-0b8e-41dd-a2b7-87a196de294a#card` | Nantuko Tracer | D | 2 |
| `565ba85c-bad8-4cbd-a2ff-b4384b886308#card` | Sudden Setback | C | 10 |
| `567b676b-54f3-44c7-8654-4d2b0262ea12#card` | Banishment Decree | D | 1 |
| `56fd8895-3be2-4591-86fa-87567d9cdc14#face:0` | Commit | O | 4 |
| `58374ee4-0497-4d12-847c-0f67555f8399#card` | Fencer Clique | D | 1 |
| `5a6bbc45-6cb8-40ff-9f7c-7b2c5713dd78#card` | Griptide | D | 1 |
| `5ae03181-a436-4bad-be3a-6c9f6c0ed4d6#card` | Undying Beast | D | 1 |
| `5f67698a-28db-43cd-aaf1-52371b0b47eb#card` | Temporal Cleansing | F | 4 |
| `6048b9ac-7d5e-4486-a89a-30adb45aab27#card` | Lodestone Bauble | M | 16 |
| `60674ef7-f8e2-41e8-bab3-0100cfc9fbe9#card` | Cogwork Archivist | D | 2 |
| `60e59923-ecd9-457e-b772-693138e05ab2#face:0` | Hide | D | 2 |
| `64399663-2c1e-447e-a3ed-c49ebd9f2231#card` | Hoverstone Pilgrim | D | 2 |
| `68f34b1c-e06b-4859-b7c0-62148341e38c#card` | Nascent Metamorph | M | 92 |
| `6c2cf22e-5dbb-4fa9-8ece-17069e6f1d7f#card` | Excommunicate | D | 1 |
| `6d1a9d06-028f-487a-b490-27100c128ea3#card` | Precognition | D | 8 |
| `6db5f1a7-4cd8-42e2-a5d8-1603206ad0cc#card` | Dovin's Dismissal | D | 32 |
| `6dbd0764-4bfe-4a58-905d-aeee70ceb798#card` | Vivien's Grizzly | D | 20 |
| `6dc6536f-455a-4d7b-beed-a78ec63c0030#card` | Isolation at Orthanc | O | 4 |
| `6e072b3a-7a83-4d77-bae3-72de0e646b81#card` | Repel | D | 1 |
| `6e93cb9a-44c8-49fb-b6a7-bfdf7bd725fe#card` | Civic Guildmage | D | 2 |
| `6ef46fd7-82db-45d3-b02e-7312384577f3#card` | Junktroller | D | 2 |
| `6ef9bf18-6b09-4398-a502-6867d999ab0c#card` | Run Aground | D | 1 |
| `6fb84ecd-ab60-4c2c-987d-8a0749db5777#card` | Enigma Sphinx | O | 9 |
| `7191f765-6e66-4bc4-8aa8-85280ac61d1e#card` | Dark Revenant | D | 1 |
| `7346c266-19d8-4432-acf5-f00aff074ad5#card` | Moonsnare Prototype | C | 50 |
| `73b8cf90-3c71-4f8b-a29f-61894b7f27c9#card` | Volrath's Stronghold | D | 2 |
| `75a7fdd4-046a-433b-b16f-a0424a05c904#card` | Tel-Jilad Stylus | D | 1 |
| `75e930dd-32e9-4c39-81a7-d833a3976b8d#face:1` | Chaos, the Endless | D | 4 |
| `77d6c141-5670-4120-9346-dc2fdcb66e50#card` | Metamorphose | D | 5 |
| `78304a06-4c9a-4cdf-b67f-067d88762381#card` | Clash of Elements | D | 2 |
| `797155cd-faf4-4321-8629-c8c352392748#card` | Reclaim | D | 1 |
| `7a80212a-aae3-483b-abb6-7271a62dff8a#card` | Gandalf, White Rider | O | 168 |
| `7e770c08-910e-4394-88c5-79c874750b6a#card` | Kami of Restless Shadows | D | 40 |
| `7f08c592-8ddd-4f52-95c3-5649cbb24d3a#card` | Sadistic Augermage | D | 1 |
| `8031285b-2add-42da-aa97-03c3f2fff226#card` | Vessel of Endless Rest | D | 2 |
| `829436df-a48b-4535-84ec-64682a16949b#card` | Shattered Ego | O | 2 |
| `84100106-1f97-4859-aadd-a35796ee000f#card` | Aura Extraction | D | 1 |
| `8460abd9-ab5d-4a7e-8132-185b4a310190#card` | Leashling | D | 2 |
| `891d5c5e-3d4d-4183-b9fd-ab694639d1c3#card` | Uproot | D | 1 |
| `8a397a64-542f-43fb-bf54-27ac651e1819#card` | Coral Fighters | D | 8 |
| `8bd2af55-5a48-4300-a893-ae6cd548db24#card` | Mortuary | D | 3 |
| `8d4d70c4-0490-4239-ac8f-dd1d1a008f08#card` | Anurid Scavenger | D | 6 |
| `9076a886-efe0-4c99-8661-f0fd7a7b3129#card` | Paramecia Coloniex | D | 2 |
| `94b15f1d-827b-45e9-a684-5d0c52ea0a09#card` | Hide in Plain Sight | M | 6 |
| `94ca5db6-86e1-42b4-b776-977efa2e12b6#card` | Bloodwater Entity | D | 5 |
| `96189b18-8cab-4709-b3b6-c4f2003829eb#card` | Run Out of Town | C | 5 |
| `97cabeda-9fe3-490d-99b4-4c8d87c17157#card` | Noxious Revival | D | 1 |
| `985192cd-ceb6-48ad-80b9-e10fc544260e#card` | Jade-Cast Sentinel | D | 2 |
| `98ebde8c-b47d-440b-93cb-a1407b807074#card` | Unlucky Drop | C | 5 |
| `9c83c32e-798e-40aa-9bb6-6ef5500d273b#card` | Rootrunner | D | 1 |
| `9d2e7099-dcd5-428f-a732-2a82b8d1dd1a#card` | Frantic Salvage | D | 4 |
| `a116329a-343e-4f10-a122-38bf8b5ac2c8#card` | Hidden Retreat | D | 48 |
| `a123aab8-3f3a-4187-abcb-21a99dcf13bb#card` | Riverwalk Technique | C | 5 |
| `a1eccd85-d243-4d27-bf49-d3ac816159e4#card` | Bamboozle | M | 9 |
| `a2ae1a1b-5c4f-46ed-90da-bd50ca914e7c#card` | Vanishment | D | 1 |
| `a2db1035-008e-4de3-b5e5-2a0de83082b2#card` | Grazing Kelpie | D | 2 |
| `a31a66ee-5ddf-4d70-9ae9-725fc7327706#card` | Gone Missing | D | 1 |
| `a3da7d5b-2c2b-45fe-b9c5-413b8c8fc0a2#card` | Academy Ruins | D | 2 |
| `a3ee8565-f34c-4073-bc94-3fe9ec6bbe8a#card` | Return to the Sewers | C | 5 |
| `a426a258-fd8b-489c-8642-9868ee47de85#card` | Golgari Thug | D | 2 |
| `a6898364-c29e-4b97-a500-344efa3ec24a#card` | Banishing Stroke | D | 2 |
| `a787b18d-d33f-41ea-a0f8-f05e272d06eb#card` | Forced Landing | D | 2 |
| `a78b063a-7f74-465a-9670-34f927f4bfe9#face:1` | Bubble Up | D | 5 |
| `a7c46e5f-6e3a-405b-86fa-404d3b6971db#card` | Chronostutter | O | 4 |
| `a7dd75bf-ecda-406c-baa2-8c051f138809#card` | Lashweed Lurker | D | 2 |
| `a85390b6-e90b-4d0c-9573-eb67ce01a0aa#card` | Thicket Elemental | F | 240 |
| `ac141fe0-328a-4d9b-8048-b0c47f399587#card` | Whisk Away | D | 1 |
| `ac7a9d73-dbdf-4e97-b382-67cd30ef5649#card` | Glowspore Shaman | D | 2 |
| `acc74d59-f587-46a2-a90b-865f721d494d#card` | Nulltread Gargantuan | D | 1 |
| `adca3929-4ecc-45bc-932e-604bcc32550a#face:1` | Lagoon Breach | C | 15 |
| `aeba5c63-aa80-4f31-90fb-f404c9a3057d#card` | Aphetto Vulture | D | 2 |
| `b04ef0bd-5e96-45d4-afa8-1f3bc8544e1a#card` | Treason of Isengard | D | 7 |
| `b526962e-be8a-4803-b038-57869bb96751#card` | Soldevi Digger | D | 2 |
| `b5e51171-2e36-4b09-b43d-e5d437a19305#card` | Fire Prophecy | D | 1 |
| `b6265654-84c3-469b-9afa-f752fd64abe0#card` | Mwonvuli Beast Tracker | D | 98 |
| `b6fe779f-b20d-49cc-96dd-54f1ffb312e1#card` | Sequestered Stash | D | 2 |
| `b7618312-6835-4851-9651-b5402001b3b0#card` | Swiftgear Drake | D | 4 |
| `b9906b19-912e-4a9d-888f-3c895bbed2c5#card` | Dukhara Scavenger | D | 5 |
| `bac52ae6-24ee-4c4f-a32c-3eb9859dc025#card` | Trip Up | C | 5 |
| `bbfb3e4a-b389-4391-8141-13b68c0ef2e0#card` | Memory Lapse | D | 6 |
| `bc7f39e1-97bf-4725-a84a-2a04121530c6#card` | Jailbreak Scheme | C | 15 |
| `c01a090a-11e4-469e-aded-74203c06fef4#card` | Salvage | D | 1 |
| `c28211c6-a5ee-40c3-bb6a-da3e7e73fd95#card` | Unholy Grotto | D | 2 |
| `c35c39be-6db4-4a80-ace1-07932d1bb460#card` | Cabaretti Ascendancy | D | 20 |
| `c46738a4-e0d4-41dc-a20d-4800f00cea62#card` | Seasons Past | D | 32 |
| `c474b52f-2f50-416e-b6c2-07043c7b339d#card` | Guiding Spirit | D | 4 |
| `c596b847-4e05-4279-8675-b4e39cb3dc91#card` | Manhole Missile | D | 1 |
| `c6921cfe-48f9-4378-b145-d9b430c2aaf6#card` | Gamekeeper | F | 92 |
| `c836f8e4-6154-4c7a-a8dd-165ff74f9115#card` | Thalakos Mistfolk | D | 1 |
| `c9db6b94-a7b1-4b93-b454-4dead8f85e34#card` | Hinder | C | 72 |
| `cb58055f-8190-4bc7-ad32-844ca66fd2b0#card` | Arashin Sovereign | C | 12 |
| `cc94462f-a207-4fdb-8704-aa979c244cab#card` | Gravepurge | D | 4 |
| `ccf1336c-2ea5-4582-84a5-0b29b3cca6c2#card` | Forced Retreat | D | 1 |
| `cdda63d7-e3ea-455c-b7d4-c6cfdda2703c#card` | Shadow Guildmage | D | 1 |
| `cdef800d-033e-4095-964d-9b6066f9fd36#card` | Phyrexian Archivist | D | 2 |
| `ce77fff1-32f8-4a55-9124-b1dafe95840d#card` | Bookwurm | O | 6 |
| `cec95b9f-9d1e-4988-befe-d0c9e20e428d#card` | Lost in Space | C | 5 |
| `cf5103c1-2590-4aed-b3ae-8a570f797738#card` | Mind Funeral | F | 46 |
| `d22d3934-8437-4a18-81c1-4e9c501846ca#card` | Tomb Trawler | D | 1 |
| `d4ab7848-5c37-4c6b-be29-0bb703333e5b#card` | Spell Crumple | D | 24 |
| `d5978943-52fc-4b57-9c1c-d802ade1b014#card` | Hunting Drake | D | 1 |
| `d5d869bc-8996-4d19-9e32-d1cfb968875c#card` | Revenge of the Drowned | C | 35 |
| `d65728cf-e9ce-46d8-a890-e1295b18cfbe#card` | Endless Detour | C | 25 |
| `d73913ea-44d0-408f-9f8b-8e91843f2826#card` | Stillness in Motion | M | 8 |
| `d7dcd1a3-3163-4e26-94e8-f4072e477e6a#card` | Halo-Charged Skaab | D | 5 |
| `daaf33d8-2ee2-41f8-9872-66b6493c6e13#card` | Happy Hogan, Bodyguard | F | 12 |
| `dae81b1b-28dd-4f7f-9bcb-65e72eaa37e2#card` | Desynchronize | C | 5 |
| `dcaaddee-00b1-495c-9161-40c91c900aac#card` | Hallowed Burial | D | 1 |
| `dd0cac88-0a78-408f-8eb0-080f79150180#card` | Aethertow | D | 1 |
| `ddd97b56-a80f-4515-9883-ae5fc1de0c5d#card` | Rebuking Ceremony | D | 1 |
| `de12ca82-4824-4975-9a31-309a6d9101a7#card` | The Balrog, Flame of Udûn | D | 8 |
| `de9e8b71-897d-4a9d-942b-af10f31e0425#card` | Sacred Guide | F | 224 |
| `e0462a2c-fb88-495f-813b-6476ee3e62bb#card` | Totally Lost | D | 1 |
| `e0fa6d08-1808-4a29-b1e4-5cec9d31d798#card` | Not Forgotten | C | 84 |
| `e1c49bdb-cfd1-4ebf-90f7-73bd5009f75e#card` | Looming Hoverguard | D | 1 |
| `e3480c8f-15ad-451b-bb34-b129805569e1#card` | Stunted Growth | M | 2 |
| `e3483dd4-0118-4143-9b55-51078a6f274a#card` | Epitaph Golem | D | 1 |
| `e5128230-3535-4f77-8b97-20d0791e7b1d#card` | Psychic Surgery | M | 3 |
| `e574bfdb-505b-4d61-a376-6d26f03cd9b1#card` | Reito Sentinel | D | 2 |
| `e6ed8820-4276-4ead-ba82-4654a92c415d#card` | Vedalken Dismisser | D | 1 |
| `e7592863-4aee-47b3-9abf-4b863ab0b6ec#card` | Azorius Charm | D | 2 |
| `e9aac98a-6fad-463e-89fa-3ad3a7c7f53a#card` | Haunted Crossroads | D | 2 |
| `ea871b01-ebaf-4987-a2bf-949cdd367ad1#card` | Reito Lantern | D | 2 |
| `ebc1d343-26f1-4d85-ac3a-610da24c990e#card` | Roil Spout | D | 1 |
| `ecf1e378-d6bc-4ee0-800e-eb8a955d781f#card` | Aether Gust | C | 10 |
| `efc12fda-054b-466a-a863-06cf54878172#card` | Oust | O | 4 |
| `f27f51e1-b881-4639-9d36-47cdf2a61117#card` | Set Adrift | D | 1 |
| `f33ef275-632e-46c8-a735-042dee340a12#card` | Prying Questions | D | 1 |
| `f3838ff1-5403-4f3f-8531-36029c13fcfc#card` | Riptide Gearhulk | O | 12 |
| `f8aa1893-60e5-439e-9316-872a9f6b6c94#card` | Argothian Wurm | D | 1 |
| `fab8d954-807b-426f-8057-99a5d2fec618#face:0` | Fell Horseman | D | 2 |
| `fb431500-152c-4524-b76b-de62922ff57f#card` | Foster | F | 23 |
| `fbafbe3a-3980-4457-a77b-3f72df7b3329#card` | Natural Obsolescence | D | 2 |
| `fd26127d-6807-41e3-9dcf-20ef257d71d3#card` | Mirror-Mad Phantasm | F | 118 |
| `ff27ff96-afd2-45df-b799-3046179e27c6#card` | Reinforcements | D | 3 |
| `ff9f4bfe-1585-4eaa-943c-1e85ed79a820#card` | Disempower | D | 1 |

The 29 inherited faces reconcile as follows. **20 already read at baseline;
21 read finally**, with Shadow Guildmage the one inherited gain. Failure causes
are corroborated with discriminating probes in the ignored evidence directory,
not inferred merely from absence of a whole-face Reading.

| Inherited face | Final outcome / next cause |
|---|---|
| Commune with Evil | Reads: 3 Readings |
| Strategic Planning | Reads: 3 Readings |
| Beast Hunt | Reads: 6 Readings |
| Vigean Intuition | Comma-then link; english-v3-then-sequencing. |
| Mulch | Reads: 6 Readings |
| Discerning Taste | Greatest-power/among comparison body; english-v3-scalar-comparisons, with residual superlative composition in english-v3-systemic-residuals. |
| Tracker's Instincts | From-among source in the selected object; english-v3-systemic-residuals (licensing prerequisite/re-application ledger). |
| Murmurs from Beyond | Reads: 3 Readings |
| Maestros Charm | Reads: 3 Readings |
| Confounding Riddle | Reads: 6 Readings |
| Winding Way | Bare singular type choice in “Choose creature or land”; english-v3-systemic-residuals. |
| Sultai Soothsayer | Reads: 3 Readings |
| Forbidden Alchemy | Reads: 3 Readings |
| Taigam, Sidisi's Hand | Reads: 12 Readings |
| Resentful Revelation | Reads: 3 Readings |
| Scattered Thoughts | Reads: 3 Readings |
| Tamiyo, Collector of Tales | Unconsumed Cause object + to-VerbPhrase frame in the first sentence; comma-then and From-among also remain. english-v3-systemic-residuals and english-v3-then-sequencing. |
| Pieces of the Puzzle | From-among source; english-v3-systemic-residuals. |
| Firja, Judge of Valor | Reads: 12 Readings |
| Ancestral Memories | Reads: 3 Readings |
| Ransack the Lab | Reads: 3 Readings |
| Organ Hoarder | Comma-then link; english-v3-then-sequencing. |
| Borborygmos Enraged | Reads: 18 Readings |
| Shadow Guildmage | Reads: 1 Readings |
| Testament Bearer | Reads: 3 Readings |
| Rakshasa's Bargain | Reads: 3 Readings |
| Bitter Revelation | Reads: 3 Readings |
| Kruphix's Insight | From-among source; english-v3-systemic-residuals. |
| Glimpse the Future | Reads: 3 Readings |

Harald, King of Skemfar also retains its From-among failure while its isolated
random-order destination passes. The explicitly out-of-scope Enigma Sphinx,
Lost Hours and Uncharted Voyage all read incidentally; their existing
productions were not expanded.

Frame reconciliation: Look's unsupported At + Object row and duplicate active
At + Object NP row become one At + Complement NP row (frame 0). Put's unsupported
On + FrameComplement row becomes Object NP + Locative Complement (frame 1);
its unsupported order row becomes Object NP + Locative Complement + In +
Manner Complement (frame 4). The old active On + Object NP row was retired.
Post-landing review found it was the only supplier for the On complement-cluster
Reading, so calling it redundant was incorrect; the follow-up below restores
that Reading through the Object + Locative Complement frame. Into remains
frame 3; Onto is preserved at frame 5 after renumbering.
Other unsupported source/To/Onto rows remain deferred, without pretending they
are consumed. Reveal and Exile keep their existing Object frames.

**Deviations and additions:**

- Four ordinary Constructions added: `PrepositionComplementNominal`,
  `BarePrepositionNounPhrase`, `BareNominalPreposition`, `MannerComplement`.
  One shared schema added: `SelectedGapLocativePredicate`, needed to preserve
  passive/Object Gap coverage when replacing the destination frame.
- Singular Count nouns Top and Bottom added with zero-complement and
  Of-complement nominal frames. Top alone receives Bare Preposition Use; bare Bottom has zero
  supported attestations and receives no such permission. Existing Top/Bottom
  adjectives remain. No compound On-top-of preposition is introduced: the
  right-branching NP analysis is a project ruling, grounded in *top of X*
  occurring outside the idiom ("the top or bottom of their library"). CGEL
  Ch. 7 §3.1, pp. 620–623 supplies diagnostics; p. 622 [14] leaves the choice
  to the evidence and does not decide *on top of*.
- Feature propagation carries nominal complement ownership, paired bare-NP
  permissions, and selected manner use. In gains its attested Verb Complement
  licence for the selected tail. A broader corpus scan found seven free
  `in turn order` hosts: Pain's Reward, Protection Racket, Sadistic Shell Game,
  Manifold Insights, Mages' Contest, Rejoin the Fight and Illicit Auction.
  Their noun-premodified distribution keeps its free-Adjunct permission; its
  pre-existing nominal-composition failure is not repaired in this ticket.
- Baseline Look/Reveal/Exile library-card NPs already read. Their tests prove
  structure rather than claiming new coverage. Ordinal/choice gains and the
  retained Telling Time alternative above are disclosed, not bespoke repairs.
- The systemic-residuals ledger records the landed frames and routes the eight
  remaining inherited failures to live owners. Its historical no-comma-then
  attribution is superseded by the current isolated probes, as the ticket
  requires; no admission rule or recorded ruling is changed.
- Glossary gaps closed: Manner Complement, Bare Preposition Use, Bare Nominal
  Complement, Nominal Complement Marker, Selected Preposition Use. Linguistic
  definitions cite CGEL; no Comprehensive Rules citations were introduced.

STOPs: **none**. Both the 14 corpus regressions and the stale lexical-source
frame assertion were fixed within this ticket. No recorded-ruling
contradiction was resolved by inventing an exception.

Assurance counts: **0 restored, 4 re-spelled, 0 newly ignored, 6 added,
0 removed**. Re-spelled tests are
`actual_movement_constituents_keep_selected_destination_before_depictive`,
`animal_magnetism_has_only_the_two_selected_segment_readings`,
`authentic_selected_object_and_passive_gap_preserve_frame_and_marker`, and
`complete_put_and_look_frames_preserve_their_typed_np_slots`. Exact-value
assertions and the same Oracle witnesses remain; obsolete row-number and
Object-vs-Complement spellings follow the corrected frame shape.

### REPORT

The v3 measured covered count is **15,997** on `xltvxkxk`, from **15,792** at its
baseline. The unchanged legacy coverage lock supplies no v3 admission authority.
After refresh, source declarations are **193 ordinary Constructions + 44
shared schemas = 237 constructor names**. This feature adds four ordinary
Constructions and one schema to the refreshed base (189 + 43 = 232). The
initial base was 189 + 45 = 234; the incorporated quotes follow-up retired
two unused schemas and proved unchanged coverage and per-face Reading counts
on its `qrkrqyzzlkrpmwmnopnvxmvrtysrxwnu` tree, covered 15,792. These are
provenance, not corpus-fitted gates. The final lexical inventory digest is
`3bc160c3406eb3ee82f923dfcdc0f8a803fa3932c0812d6623e331e8c527e779`.

Homograph surface inventory: **122**, with Top and Bottom noun/adjective pairs
the two additions to the 120-surface baseline. The inventory uses independently
realized Declared-case values, distinct owners, and excludes metadata Catalog
entries. All owners are named in `library-position-homograph-surfaces.tsv`.
Form-literal/vocabulary overlap inventory: **none**, before and after; no word
form literals were added. Homograph surfaces:

`'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `X`, `bottom`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `instead`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `top`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’s`, `∞`.

Performance advisory (same `xltvxkxk` stamps and covered counts):

| Tree / covered | Corpus wall ns | Checked-text thread CPU ns/B | Workers | Host load (1 / 5 / 15 min) |
|---|---:|---:|---:|---|
| Baseline / 15,792 | 77,977,867,462 | 263,035 ns/B | 12 | 8.49 / 8.76 / 8.06 |
| Refreshed final / 15,997 | 46,357,696,153 | 237,222 ns/B | 12 | 2.25 / 2.88 / 5.92 |

Both wall times exceed the 16.26-second quiet-host advisory. These complete-Reading
measurements are not a controlled quiet-host throughput comparison; no
coverage or Reading cap was introduced to fit the advisory.

Validation: `cargo xtask gate --changed --from nmtxxkuv --clippy --run`
derives `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p
deckmaste_english_v3 -p xtask` and `cargo clippy -p deckmaste_lexical_source -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D
warnings`. The pre-refresh derived commands pass: 736 tests passed, zero failed; Clippy
passes with warnings denied. The same derived commands pass again after refresh:
737 tests passed, zero failed, and Clippy passes with warnings denied. The gate retains one pre-existing ignored test,
`macros::templates::tests::macro_schema_census_count_matches_21`, whose recorded
reason is “cross-checks the live corpus against the census; run on demand”.
No ignore was added by this landing. Package formatting passes. Cite checks report
zero noncompliant sites and zero stale citations; the piped diff audit sees
zero changed Comprehensive Rules sites.

All census, selectors, probes, inventories, logs and audit evidence were written
under this workspace's ignored `target/english-v3/`, with no evidence under
`/tmp`. Before workspace retirement, the `library-position*` evidence is
preserved under the coordinator workspace's ignored
`target/english-v3/library-position/`. No session evidence or verifier enters
tracked source; this landing record is the required ticket disclosure.

### Post-landing fix (2026-10-06)

PROVE: the deleted On row supplied the only On complement-cluster frame; it
was not redundant. This follow-up restores parallel Object + Locative
Complement tails through Put's existing ObjectLocative frame. The unreachable
ObjectOnObject frame and its selected_object_marker row are deleted.
The full census exercises ObjectLocativeTail 72 times in the 36 restored
cluster Readings; its declaration is reachable. Stand Together has a regression
test requiring the cluster Reading and both Object + Locative Complement
coordinates, beyond successful recognition.

DISCLOSE: the following comparison uses the original measured baseline,
not the original pre-refresh and refreshed finals. The original frame change
moved all 83 already-covered `look at X` faces from Object to
Complement(NounPhrase), and all 1,058 already-covered `put … on X` faces from
marker On + Object NP to a Locative Complement PP. These counts use the
original baseline's covered faces and inflected surface matches, with all
identities listed in `relation-change-identities.json`. Put's selected goal
PP is supported by CGEL, Ch. 7 §2.1, p. 605 [6]. Reveal and Exile retain their
Object relation.

| Face | Original baseline | Original landed | Fix (before/after refresh) | Cause |
|---|---:|---:|---:|---|
| Brokers Ascendancy | 9 | 8 | 9 | Cluster regression restored |
| Captain America, Team Leader | 144 | 120 | 144 | Cluster regression restored |
| Claim the Kingdom | 16 | 14 | 16 | Cluster regression restored |
| Evolutionary Escalation | 9 | 8 | 9 | Cluster regression restored |
| Ich-Tekik, Salvage Splicer | 24 | 21 | 24 | Cluster regression restored |
| Juniper Order Ranger | 7 | 6 | 7 | Cluster regression restored |
| River Heralds' Boon | 6 | 5 | 6 | Cluster regression restored |
| Serrated Biskelion | 6 | 5 | 6 | Cluster regression restored |
| Stand Together | 6 | 5 | 6 | Cluster regression restored |
| X-23, Deadly Weapon | 7 | 6 | 7 | Cluster regression restored |
| Brawn, Amadeus Cho | 40 | 48 | 48 | Broader locative-frame attachment |
| Kutzil's Flanker | 42 | 52 | 52 | Broader locative-frame attachment |
| Shaile, Dean of Radiance | 6 | 8 | 8 | Broader locative-frame attachment |
| Thought Gorger | 126 | 140 | 140 | Broader locative-frame attachment |

The first ten faces lost their cluster Reading in the original landing and
regain it here. The remaining four gained grammatical alternatives because
Put's ObjectLocative frame accepts any licensed locative PP: a preceding PP
can belong inside the Object while a later PP fills Put's destination slot.
Thought Gorger and Brawn admit the final `in your hand` as Put's destination
with `on … for each card` inside the Object; Kutzil's Flanker and Shaile admit
`under your control` as that destination, with `on …` inside the Object (the
relative clause remains inside it). These are grammatical attachment
alternatives, independent of game interpretation; none is discarded.

The follow-up restores exactly **36 Readings** across those ten faces, with
**zero other per-face Reading-count changes**, zero new covered faces and zero
lost faces. `delta.json` names every affected identity and its before/after
cluster count; `original-count-changes.json` records all 14 original changes.
The original before/final stamps are `xltvxkxkvsyknxnkrztqqkqpqsvzzvrp`,
covered 15,792 / 15,997 respectively. The follow-up pre-refresh before/after stamps are
`owyyynoyxstupwposuuzymvuqwpunnyl` (`owyyynoy`), covered **15,997**
in both phases; `before.json` and `after.json` distinguish the measured trees.
SelectedComplementClustersPredicate occurrences were 411 at original baseline
(`xltvxkxk`, covered 15,792), 549 at original landing (`xltvxkxk`, covered
15,997), and are 585 after the isolated fix (`owyyynoy`, covered 15,997). The original aggregate
increase from newly covered Into/Onto faces concealed the 36 lost On-cluster
Readings; the named comparison above accounts for their restoration.

Refresh incorporated the completed english-v3-then-sequencing landing,
`lklxwwprzolopyowyuputxtpvzttpnpm`, covered **16,977**. Its 980 face gains
(15,997 → 16,977) and 101,103 Reading increase belong to that upstream landing.
The table above is unchanged after refresh. The combined tree's full census
is stamped `kttnulzsnmsokunlrnwqlsnnulyupvsp` (`kttnulzs`),
covered **16,977**. Against the incorporated parent census, this fix still
adds exactly **36 Readings on the same ten faces**, with **zero lost faces,
zero new covered faces, and zero other Reading-count changes**. All identity
and count changes are in `refreshed-delta.json`. The parent grammar and lexicon
match the upstream landing's saved, verified candidate A byte-for-byte after
removing only this fix; `then-parent-verification.json` records that comparison.
This makes `then-base.json` a measured baseline for the refreshed feature.
SelectedComplementClustersPredicate occurrences are 610 on that upstream tree
(`lklxwwpr`, covered 16,977) and 646 in the combined census (`kttnulzs`,
covered 16,977); ObjectLocativeTail still supplies exactly 72 occurrences.
The recorded original comma-then residual counts below describe the earlier
library-position landing. Comma-then is now landed by its existing owner;
remaining From-among and nominal/locative/discourse gaps stay with
english-v3-systemic-residuals.

The historical 298 was a surface-bucket estimate on `wlvwtnppyovn`, covered
13,716, not a gain forecast or a stable cohort. The measured original gain is
205 (15,792 → 15,997). On original landed `xltvxkxk`, covered 15,997,
566 unread faces contain `the top N cards of`; 293 contain `from among`,
96 contain `, then`, and 368 contain at
least one (21 contain both). These blockers coexist with library phrases, so
repairing the latter cannot make the whole face read. Other sampled blockers
are `in a face-down pile` (Abstract Performance), `the other` (Ashiok,
Wicked Manipulator), and `where X is` (Florian, Voldaren Scion). From-among and
these remaining nominal/locative/discourse gaps remain with
english-v3-systemic-residuals. Comma-then was owned by
english-v3-then-sequencing and has now landed, as the refreshed comparison
above records. The historical identities and sources are in
`top-cards-residuals.json`.

Citation corrections and project rulings (review ruling, 2026-10-06):

- Nominal Complement Marker's Complement/Postmodifier distinction cites
  CGEL, Ch. 5 §14, p. 439; p. 446 [14] supplies PP-modifier examples.
- Requiring both the noun's Bare Preposition Use and the preposition's Bare
  Nominal Complement permission is a project licensing rule, not a claim
  from CGEL pp. 620–623.
- The Manner Complement entry states the project's marker convention: it
  labels the NP inside selected `in`; CGEL p. 671 [6] calls the whole PP
  the manner phrase.
- The right-branching choice for `on top of` is a project ruling grounded in
  `top of X` occurring outside the idiom, including `the top or bottom of
  their library`. CGEL p. 622 [14] supplies distributional tests and leaves
  the choice to evidence; it does not decide `on top of`.
- In `their choice of the top or bottom of their library`, `of their library`
  is a Postmodifier of the coordinated Nominal because NominalConcord
  exports NominalComplementMarker = None. A single `top` or `bottom` takes
  it as a Complement. Both analyses are NP-internal; the function is
  inconsistent, disclosed here and retained without an out-of-scope repair.
  The outer `of the top or bottom …` remains internal to `choice`.

Pre-refresh validation on `owyyynoy`, covered 15,997: **245,378 complete
Readings** all
pass declaration admission, lexical ownership/context, byte-exact realization,
and construction/leaf traversal identity. All 32,828 face enumerations are
complete; validation issues, internal failures, duplicate/cyclic derivations,
limited/failed enumerations and undetermined results are zero. The inherited
independently authored position-PP and complement-cluster values keep the
converse roundtrip and exact Reading/traversal comparisons.

| Follow-up phase / stamp / covered | No Reading | Unique | Multiple | Readings |
|---|---:|---:|---:|---:|
| Before / `owyyynoy` / 15,997 | 16,831 | 6,986 | 9,011 | 245,342 |
| Isolated fix / `owyyynoy` / 15,997 | 16,831 | 6,986 | 9,011 | 245,378 |
| Upstream then / `lklxwwpr` / 16,977 | 15,851 | 7,025 | 9,952 | 346,445 |
| Combined / `kttnulzs` / 16,977 | 15,851 | 7,025 | 9,952 | 346,481 |

Every one of the **346,481 combined Readings** passes the same admission,
lexical ownership, exact roundtrip and traversal checks. All 32,828 face
enumerations are complete, with zero issues, internal failures, duplicates,
cycles, failed/limited enumerations and undetermined results.

Specificity-resolved selection remains zero: all grammatical Readings are
retained, with no construction-pair arbitration. Forbidden word-/lexeme-/card-
or construction-named admission guards added: **zero**. Lexical source and
grammar-environment loading succeeds without errors. The active v3 command
has no legacy `environment.rs` or emitted permitted-licensing-checker total;
no obsolete counter is fabricated. `of` remains NP-internal; the repair adds
no preposition licence or clause-attachment path.

`cargo xtask gate --changed --from pqtmvqqo --clippy --run` derives
`cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`
and `cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 -p
xtask --all-targets -- -D warnings`. Both pass on `owyyynoy`, covered 15,997:
**682 tests passed, zero failed**, with one pre-existing ignored test,
`macros::templates::tests::macro_schema_census_count_matches_21` (recorded
reason: "cross-checks the live corpus against the census; run on demand").
Clippy passes with warnings denied. Workspace-root formatting and cite checks
pass: zero noncompliant sites, zero stale citations; the piped diff audit
reports zero changed Comprehensive Rules sites.

REPORT before refresh: covered **15,997**, with **194 ordinary Constructions +
44 shared schemas = 238 constructor names**, on `owyyynoy` (covered 15,997).
The unchanged legacy coverage lock supplies no v3 authority. Homographs remain
122 (the named surface/owner inventory is the original
`library-position-homograph-surfaces.tsv`); form-literal/vocabulary overlaps
remain **none**. No lexical declaration changed; both census inventory digests
are `3bc160c3406eb3ee82f923dfcdc0f8a803fa3932c0812d6623e331e8c527e779`.

| Tree / covered | Corpus wall ns | Checked-text thread CPU | Workers | Host load (1 / 5 / 15 min) |
|---|---:|---:|---:|---|
| `owyyynoy before / 15,997` | 46,799,430,336 | 239,878 ns/B | 12 | 11.12 / 11.33 / 10.42 |
| `owyyynoy isolated fix / 15,997` | 47,238,619,497 | 224,987 ns/B | 12 | 8.13 / 9.84 / 10.29 |
| `kttnulzs combined / 16,977` | 176,519,001,975 | 422,331 ns/B | 12 | 18.64 / 15.54 / 12.75 |

All wall times exceed the 16.26-second quiet-host advisory. These loaded-host
complete-Reading measurements are provenance, not a quiet-host comparison or
an admission gate.

Refreshed REPORT (`kttnulzs`, covered 16,977): constructor names remain
238 (194 ordinary + 44 schemas). The upstream Then owner adds the sole new
Declared-case homograph surface `then`, bringing this record's inventory
method to **123 surfaces**; every surface and owner is named in
`homographs-refreshed.tsv`. This uses the same Declared-case/non-Catalog
method as the earlier 122, independent of the upstream ticket's separate
case-variant inventory. Form-literal/vocabulary overlaps remain **none**.
The refreshed lexical inventory digest is
`958e1ccfed83835a7cd480940ab23ae665b763d288e68ca25a7e278a9fc3fd4d`;
the sole lexical change is the incorporated Then declaration.

The refreshed gate was rerun with the same `--from pqtmvqqo` command,
including the incorporated lexical data. It derives
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p
deckmaste_english_v3 -p xtask` and
`cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p
deckmaste_english_v3 -p xtask --all-targets -- -D warnings`.
Both pass on the refreshed `owyyynoy` tree, covered **16,977**:
**746 tests passed, zero failed**, with the same one pre-existing ignored
test and no changed ignore. Clippy passes with warnings denied. Root formatting
and citation checks also pass after refresh: 16,112 citations checked, zero
stale, zero noncompliant sites; the piped audit selects zero changed CR sites.
Refresh exited zero without conflicts and retained both features. The source
digests and incorporated-parent comparison are saved in `refreshed-source.json`
and `then-parent-verification.json`.

Deviations and additions: one ObjectLocativeTail Construction supplies the
newly selected frame to existing complement-cluster coordination. No lexical
entry or frame is added. Two unreachable declaration entries (the
ObjectOnObject frame and its marker-table row) are removed. Glossary changes
correct attribution and explain the existing marker convention; no new term
or Comprehensive Rules citation is introduced.

Assurance: **0 restored tests, 0 re-spelled, 0 newly ignored, 1 added,
0 removed**; ten faces have their grammatical cluster Reading restored.
The added test is
`stand_together_retains_coordinated_object_and_locative_complements`.
Existing recipient-cluster and mixed Into/Onto-cluster exact-value tests
remain unchanged and pass.

Evidence lives only under the feature workspace's ignored
`target/english-v3/library-followup/`, and is preserved at the same path in the
coordinator before `kata drop`. No census, verifier or process artifact is
tracked. STOPs: **none**; all findings were repaired or disclosed within this
follow-up.
