---
needs: [english-v3-generic-frame-consumption]
---
# Compose maximum quantities with existing cardinal phrases

Refine quantity/Determiner composition to admit “up to” with a cardinal expression
and feed the existing Noun Phrase/Nominal pipeline. Do not add a bespoke Noun Phrase
recipe for each noun or target combination, or treat recognition of “up to” as
permission to broaden Oracle English beyond its attested quantity forms.

Pinned witness: Sanguine Indulgence's “up to two target creature cards”. Exact
Noun Phrase probing gives zero Readings, as does Cardinal probing of “up to two”;
“two target creature cards” has one. All words have Lexical Analyses. Reasonable
Doubt supplies “up to one target creature”, also currently unrecognized. Counted
targets and targeted genitives otherwise already compose: preserve that structure.

Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered
13,226). The raw supported-Oracle scan found 966 overlapping up-to-target surface
occurrences, including reminder text; this is not a gain forecast or sole-cause
classification. Fetch supported current card text for implementation witnesses.

Acceptance: independently construct authentic maximum-quantity constituents,
assert the intended quantity structure and appropriate Number/Agreement, preserve
lexical ownership, targeting and both roundtrip laws. Reuse existing schemas where
possible. Report complete corpus identity-level gains/losses and remaining causes;
recognition alone does not establish the intended quantity analysis. Scratch
/tmp/english-v3-np-probes is optional diagnostic evidence. Standard constraints apply.

## Sequencing

The `needs:` edge is sequencing, not a technical dependency: these grammar
tickets touch overlapping structures and are worked one at a time. Write the
pinned witnesses as tests before implementing.
The landing compares corpus Reading identities before and after on its own tree.

## Pinned analysis

Quantitative *up to two* is a Preposition Phrase in determiner function, not a
Cardinal head. CGEL Ch. 5, p. 357, example [6] lists *up to twenty minutes*
beside *around ten thousand copies* and *under ten new drugs* as PPs in that
position, and the glossary's Quantity entry already counts *up to two* among
Determiners. The counted Noun Phrase therefore keeps its noun head and takes
either a Cardinal or this PP as its determiner; no second Noun Phrase family is
introduced.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It reached the analysis pinned above. Read it for ideas if
useful. Do not rebase onto it, and treat every claim in its ticket notes as
unverified.

## Landing record

Completed 2026-10-05. Standard constraints apply. The measured grammar baseline
is claim parent `ntxupumn`, covered 13,277; implementation is `pnluytkv`,
covered 13,594. The before report stamps the then-empty working-copy change
`pnluytkv`; its runtime declarations were those of `ntxupumn`. The report stamps
are distinguished here by their measured declaration state and covered count.

### PROVE — retained identities and structural laws

All 32,828 supported faces were enumerated completely on both trees. Face losses:
0. Reading losses: 0 after normalizing only the new, transparent
`CardinalDeterminer { value }` projection. All 149,172 prior Reading identities
remain; no previously covered face gains or loses an analysis. The 317 new faces
contribute 8,401 Readings, for 157,573 total. No retirement, re-coverage obligation
or regression is hidden by a coverage decrease. The diagnostic exporter and
normalizer stay in `/tmp`; no process artifact or verifier entered a crate.

Full census validation reports zero internal, admission, exact-realization,
lexical-ownership, duplicate-Reading or traversal issues. Independent values
check both roundtrip laws and complete construction/leaf identity for the pinned
constituents, non-target lands, variable X, and a targeted genitive. Singular
and plural noun agreement, finite singular agreement, Countability, complement
selection, and invalid independently constructed values are tested explicitly.

The two new licensing guard forms read the declared `QuantitativeComplement`
feature; word-named licensing checkers added: 0. Lexical source loading and
GrammarEnvironment construction succeed. V3 does not emit the retired coverage
lock or a legacy permitted-licensing-checker total; the reported covered count is
the complete V3 census, not a claim about the retired parser.

### DISCLOSE — analysis, census and remaining causes

`up to two` is a quantitative Preposition Phrase functioning as Determiner,
not a Cardinal head. Its structure is `up [to [two]]`, with separately owned
Preposition leaves and the existing Cardinal Complement. Two category instances
share one PP schema. The existing `CountedNounPhrase` accepts a common
Quantitative Determiner interface and retains the Nominal noun head and Targeting.
Cardinal and PP projections have cost 0, preserving previous preference costs.
No second counted Noun Phrase family or noun-specific recipe was added.

The independently constructed witnesses are constituents of Sanguine Indulgence
and Reasonable Doubt, plus Kazandu Stomper, Rampaging War Mammoth, Torgaar,
Rat Out and Tormod's Crypt. Current supported local Oracle text was checked
against [Sanguine Indulgence](https://mtg.wtf/card/m21/121/Sanguine-Indulgence)
and [Reasonable Doubt](https://mtg.wtf/card/mkm/69/Reasonable-Doubt). Both pinned
maximum-target sentences now parse, and the plain counted constituents survive.
The baseline already has two analyses of `one target creature`: lexical
Determinative and Cardinal. The new test preserves both rather than imposing
a one-Reading assumption from the exploration notes.

| Tree / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| `ntxupumn` / 13,277 | 19,551 | 6,576 | 6,701 | 149,172 |
| `pnluytkv` / 13,594 | 19,234 | 6,647 | 6,947 | 157,573 |

Specificity-resolved admission: 0 before and after; every grammatical Reading is
retained. Optional preference does not decide coverage. Every new Reading has
the nested PP quantity analysis; the complete exporter finds 12 distinct
quantitative PP identities: cardinals one through seven and variable X, including
the attested sentence-initial capitalization variants.

All newly covered identities follow. Each row retains the indicated PP in
Determiner function over the existing counted Nominal; the Readings column is
the complete count, not a preferred-sample count.

| Identity | Card face | Maximum determiner(s) | Readings |
|---|---|---|---:|
| `0023888e-7bec-43e0-8dee-d1a4eb94b372#card` | Palinchron | up to seven | 1 |
| `01499544-6850-49f0-a66e-113df19c4340#card` | Pheres-Band Brawler | up to one | 2 |
| `017aa9b3-a8ea-4588-9c50-e914a7d8e4ee#card` | Metamorphosis Fanatic | up to one | 42 |
| `0217da8e-e74f-4f5c-ab75-f17600f94405#card` | Rock Soldiers | up to one | 1 |
| `0296d57e-e5b9-456f-bb21-eb584adefb4c#face:1` | Meager Meal | up to one | 1 |
| `0369627e-3b83-4e5b-a8af-a99f944443b3#card` | Conduct Electricity | up to one | 1 |
| `05e910aa-cf84-468c-8a48-256588cbbf08#card` | Rampaging War Mammoth | up to X | 1 |
| `07c3e91c-3593-4797-b7be-ef4fb2913cef#card` | Dream Spoilers | up to one | 36 |
| `08f361c5-bcb5-4a9b-9610-70609cdc047f#card` | Dwell on the Past | up to four | 9 |
| `09955b4b-6052-4c27-8b63-9548482b3c5c#card` | Heritage Reclamation | up to one | 8 |
| `0a512e55-793e-4936-bd27-80d749f4657e#card` | War Machine, James Rhodes | up to one | 3 |
| `0b9bb0d7-4139-46b6-a058-2f79be64aef2#card` | Kitesail Cleric | up to two | 1 |
| `0bd67481-6bd9-48d6-92bd-8933b5ea1eae#card` | Peregrine Drake | up to five | 1 |
| `0c0bb9b0-e7e0-465e-826b-2e56032aed1b#card` | Stunning Shot | up to one | 8 |
| `0d0c4c91-683a-4563-a0b0-39f3c04f6307#card` | Splash Lasher | up to one | 3 |
| `0d46a97a-b1d6-4bf3-b3ba-a5cdfd4c8bf5#card` | Pitiless Fists | up to one | 2 |
| `0d5eb0b9-9b08-4d26-b141-a9d612a680a6#card` | Roaming Ghostlight | up to one | 1 |
| `0ea80bd9-ab7e-4a8f-85b0-167fe7fae43a#card` | Cathartic Parting | up to four | 50 |
| `0edaf433-4ded-4841-bc86-136780c0767f#card` | Unliving Legionnaire | up to one | 6 |
| `0f68ae67-1671-4281-8b6b-3e52fdd4915f#card` | Salvation Swan | up to one | 1,026 |
| `0f730bd9-2060-46b1-9208-0ac6562e8b2a#card` | Essence Capture | up to one | 2 |
| `101407b4-e7e8-4aff-84a2-a97947358b88#card` | Liminal Hold | up to one | 15 |
| `106e9c0d-67a2-4db7-97d9-03e1ea1b40b5#card` | Cryogen Relic | up to one | 2 |
| `1175d482-d8a2-467f-bc50-2f4b241966bb#card` | Arashin Sunshield | up to two | 5 |
| `11b21086-0779-4d16-8a56-252c6bee9bed#card` | Djeru's Renunciation | up to two | 1 |
| `121f6c96-2911-4ee3-b14e-d3b143eb14c3#card` | Kazandu Stomper | up to two | 1 |
| `12c2b6bb-05f8-4291-b9a5-c4ddb7a69b9e#card` | Decompose | up to three | 4 |
| `12d51534-a98d-487b-9ba9-c10861dff992#card` | S.H.I.E.L.D. Flying Car | up to one | 36 |
| `13d5b664-db13-406e-9956-3d9fe4177d33#card` | Scale the Heights | up to one | 2 |
| `14104a52-c4e8-4755-8196-7fa5c5a35f36#card` | Duneblast | up to one | 1 |
| `16c99713-f19e-4b25-9b03-6a8d8203adfe#card` | Crawl from the Cellar | up to one | 4 |
| `16cae855-d93a-445d-9cac-dbc5a7c18cb1#card` | Shred Memory | up to four | 4 |
| `1722f814-2d4e-4d59-9b74-10ef8e89def9#face:0` | Feral Deathgorger | up to two | 5 |
| `1722f814-2d4e-4d59-9b74-10ef8e89def9#face:1` | Dusk Sight | up to one | 1 |
| `17975a20-2c13-4325-be1e-0bcda5063a2d#card` | Terastodon | up to three | 1 |
| `19cab267-3c51-4322-b1af-22941bbbdadf#card` | Wondrous Revival | up to three | 4 |
| `19cede61-1982-47d1-9e0e-49ac6de00798#card` | Wicked Wolf | up to one | 4 |
| `19f2c2c1-cd29-4e66-b5d9-e85e1979d843#card` | Spitting Dilophosaurus | up to one | 2 |
| `1ad5766b-9ae3-437b-bd91-c4c988a8b095#card` | Agonasaur Rex | up to one | 2 |
| `1b26a54f-6c27-4940-b77e-2a66175c5eff#card` | Grasp of Fate | up to one | 18 |
| `1bb1ad32-9ce5-4f14-a408-e142ff30c679#card` | Scholar of the Ages | up to two | 3 |
| `1bb7009b-aeb0-4545-be4c-c8c5d723bcf5#card` | Morbid Plunder | up to two | 3 |
| `1bc9a89c-7831-4c08-8934-896286d63e7c#card` | Put Away | up to one | 10 |
| `1bde8eaa-9beb-4529-b540-5e0155c09713#card` | Rotten Reunion | up to one | 28 |
| `1c0a72dc-4756-4d60-b6b4-4c2c69699efd#card` | Pull from the Deep | up to one | 3 |
| `1d9b8859-14e3-4bbe-ad92-916b25a37b31#card` | Raiding Party | up to two | 3,256 |
| `1f3e7420-86aa-4847-bc2d-21f6bae470c8#card` | Gearbane Orangutan | up to one | 2 |
| `212af747-2ac1-4d33-9456-8b3cc8988f5d#face:0` | Find | up to two | 3 |
| `2279005d-5434-49ce-b16a-212759c5775a#card` | Markov Warlord | up to two | 3 |
| `22c74e4b-b1fe-4a35-acd8-fa4840af3c06#card` | Famished Ghoul | up to two | 4 |
| `23cda76b-6e47-4922-a38d-93f7aae1eb00#face:1` | Skimming Strike | up to one | 1 |
| `248d7759-aa73-4430-aa86-d45dc607944b#card` | Burning Sun's Avatar | up to one | 1 |
| `25012c1f-c069-43d7-b930-a3a5a7eb1389#card` | Dismantling Wave | up to one | 6 |
| `25434ea3-bcfa-4dae-a16e-10ab87ce32af#card` | Mishra's Command | up to X | 24 |
| `269208e1-353c-4334-941a-2177f6665fba#card` | Feeling of Dread | up to two | 1 |
| `27ecb717-6251-4066-b57e-7070e9e44bbf#card` | Archon of the Triumvirate | up to two | 3 |
| `2800830c-73c6-4097-be7b-d698c6813c38#card` | Banishing Slash | up to one | 9 |
| `2972b9c8-d9b3-4bb3-b9c5-9aa70fb1ac71#card` | Super Mutant Scavenger | up to one | 7 |
| `2a483d95-ff99-40b4-9071-6e78585090d5#card` | Combat Tutorial | up to one | 2 |
| `2a74a055-ff7f-434b-8dfb-d9fb72579cee#card` | Hazel's Nocturne | up to two | 3 |
| `2c769dfc-79be-4fcb-840c-b72cafdd0a23#card` | Rapid Decay | up to three | 4 |
| `2e0d12dc-55fa-4363-b28f-612e7cfa12c6#face:0` | Invasion of Muraganda | up to one | 2 |
| `2efb908f-d069-4041-930b-5c1ccec00db2#card` | Of Herbs and Stewed Rabbit | up to one | 22 |
| `2f15abe7-54aa-40ed-a15d-5b827f1c3923#card` | Swordsman, Sharp Scoundrel | up to one | 4 |
| `2f2d75a0-d751-4c74-b357-8f92a992447f#face:0` | Kellan, Inquisitive Prodigy | up to one | 4 |
| `2f3237fe-9e07-4aad-8bc8-7ef4b510dd36#card` | Ebony Charm | up to three | 8 |
| `3066802a-35d0-4e07-99e2-b495398d88b6#card` | Unearthly Blizzard | up to three | 3 |
| `308e551a-6d60-4a28-9e22-1181995ca068#card` | Bleeding Edge | up to one | 2 |
| `3431338b-928a-47f2-bc2f-f3c5d8eda8d0#card` | Host of the Hereafter | up to one | 20 |
| `36f71d49-0f64-4b5d-b1ee-bb0af5e9db9a#card` | Restoration Specialist | up to one | 3 |
| `373c6cd0-0bf7-4dab-bc18-7ecf5f015e10#card` | Rhythmic Water Vortex | up to two | 30 |
| `382097b3-f753-493c-bde4-101c0538feb4#card` | Frost Breath | up to two | 6 |
| `38eabfea-5ea5-4baa-8ad6-7a7cb00e1fed#card` | Spider Food | up to one | 5 |
| `3a877718-ff90-4d6e-9f95-47b06adad7f8#card` | Bond of Insight | up to two | 2 |
| `3b6ef144-bb98-4686-9718-204f1c3cf020#card` | Worldsoul's Rage | up to X | 4 |
| `3b8240ad-71d3-4117-8531-6c8e79bffdb9#card` | Scolding Administrator | up to one | 40 |
| `3db407ec-da4b-4b25-b613-e45c164ffe86#card` | Cho-Arrim Bruiser | up to two | 1 |
| `3dc73517-0767-46ae-bc8d-ee378237a509#card` | Rooftop Percher | up to two | 5 |
| `3dee768a-612c-4960-bad2-2fe5b83a70c3#card` | Yuna, Hope of Spira | up to one | 84 |
| `3ecc4b57-3e0c-48b8-9952-e4b63b6bc45d#face:1` | Authority | up to two | 4 |
| `4121b7d9-5fbe-40e2-a2a6-d7633f423b5f#card` | Nightmare Sower | up to one | 6 |
| `4354eb0a-28c5-4f3f-8e78-fd7eec230c7b#card` | Dead Revels | up to two | 3 |
| `4460962e-c9cd-4cfd-b227-4278ee636206#card` | Supreme Inquisitor | up to five | 14 |
| `4505ff9f-97f1-45d1-aea9-5590fefb4a4d#card` | Prayer of Binding | up to one | 15 |
| `45687a24-e8cb-4864-8721-410eb8320aab#card` | Gold Rush | up to one | 3 |
| `45866598-4ee4-46b6-ade2-ffcdbe98e58f#card` | Necron Deathmark | up to one | 2 |
| `45c517d4-944c-43f6-8ead-a47965377575#card` | Chilling Grasp | up to two | 6 |
| `46f1618d-9c95-4a4b-9a20-7c378b811849#card` | Coronation of Chaos | up to three | 3 |
| `47f5ea79-2bf2-4e17-9c6d-e1cbcd9d30d1#card` | Repel the Darkness | up to two | 1 |
| `480454a0-fb10-432b-a963-8b0c0ffc6680#card` | Sepulchral Primordial | up to one | 36 |
| `48d63b86-8ddd-4306-960f-98490d1d80b9#card` | Seeds of Renewal | up to two | 2 |
| `493acba2-92b0-43a4-9f4b-d49a91ca77f5#card` | Rag Dealer | up to three | 4 |
| `49a7c053-81e9-47b7-af5c-c62cd0389ed4#card` | Lethal Protection | up to one | 3 |
| `4a58954a-1d20-48b8-847c-3615de67e122#card` | Warbriar Blessing | up to one | 2 |
| `4ab42daa-8726-4973-947d-1d6c32f0ba61#card` | Fight On! | up to two | 3 |
| `4b4bdbdc-3934-471b-9027-31277e7765ab#card` | Winterthorn Blessing | up to one | 32 |
| `4c48de92-e053-4af7-b936-d7ec0875e0d3#card` | Ambush Wolf | up to one | 5 |
| `4c591977-f2af-4a6e-a0be-3dfb45a3598b#card` | Nebelgast Intruder | up to one | 6 |
| `4eafe717-4ba4-4901-8c67-11757230eb54#card` | Elder Deep-Fiend | up to four | 2 |
| `4f0bcfe5-7e52-4249-a26a-482619716f18#face:0` | Invasion of Regatha | up to one | 1 |
| `4f4358cb-59df-46d9-be27-69929f5a615c#face:1` | Faith & Grief | up to two | 3 |
| `500ccb61-83b0-4b1f-ada6-65850ed5a825#card` | Qutrub Forayer | up to two | 28 |
| `520b6637-0a9f-4dc4-846e-f1cd2b868263#card` | Will of the Naga | up to two | 6 |
| `52f0dcbd-ccf3-4525-8e55-2d38b4991834#card` | Touch the Spirit Realm | up to one | 216 |
| `53a40369-0e62-4ad2-a6bd-65a5ff0da65e#card` | March of the Returned | up to two | 3 |
| `5404c6a5-399d-4237-b03c-41b6a1b79050#card` | Abandon the Post | up to two | 3 |
| `54731e3f-a84a-4fc9-8c02-2f769133c71c#card` | Diregraf Scavenger | up to one | 20 |
| `54deeeef-be90-4975-9213-8355ddf3ce0c#card` | Fall of the Impostor | up to one | 24 |
| `55a0a40c-cb40-469d-9bf3-9c33b378f4f7#card` | Baloth Null | up to two | 3 |
| `57544b37-fd5d-4da5-bbb6-7b4a09501dd1#card` | Explosive Entry | up to one | 1 |
| `581425e6-3128-4f83-9efc-17028fc65078#card` | Barrier Breach | up to three | 1 |
| `58d7b8b3-ac47-4805-8259-4ba8e70d7dc3#card` | Primal Might | up to one | 4 |
| `59cbed39-a07e-491e-a95a-adafe19eb652#card` | Jace's Ruse | up to two | 32 |
| `5a651557-ce08-46ba-aecd-0e4fbfdef3c0#card` | Bile-Vial Boggart | up to one | 1 |
| `5c2d0ead-8bfc-4ca7-9b38-2fc07af1c1d5#card` | Yosei, the Morning Star | up to five | 4 |
| `5cf069cf-3a10-4427-a02b-5405429b5652#card` | Frostveil Ambush | up to two | 6 |
| `5d1d76e0-049c-4a33-96ec-8e60deed16d5#face:1` | Done | up to two | 6 |
| `5d32dbf3-c157-4543-8813-da88f463de0e#card` | Mind Roots | up to one | 12 |
| `5d7eea87-b60f-4ab9-b485-30aadda0cd79#card` | Ant-Man's Air Force | up to one | 3 |
| `5d936655-8e46-4e30-8f9c-4fa1a4d34a17#card` | Storm, Shaker of Skies | up to one | 18 |
| `5f31a2d6-649a-40f4-89ce-95c635340e9e#card` | Atraxi Warden | up to one | 1 |
| `60b6bd93-3eb3-4b81-bc4d-1e3351ee2e64#card` | Soul Salvage | up to two | 3 |
| `60ead8a5-11fb-4eeb-99bf-0caaf1de1731#card` | Psychic Pickpocket | up to one | 1 |
| `64212311-9d66-4c25-8cf0-88ae4cac132f#card` | Terashi's Cry | up to three | 1 |
| `670df8be-21cd-49d8-92e3-d7cc38b256d6#card` | Back for More | up to one | 4 |
| `678b3fb5-aa9f-4268-9433-2a89dd37b927#card` | Canoptek Tomb Sentinel | up to one | 1 |
| `682eb224-cab4-4829-810b-72e63a9ceb5d#card` | River Heralds' Boon | up to one | 6 |
| `697b8045-0edb-4f7b-a933-4d41ec2b53d5#card` | Dubious Delicacy | up to one | 3 |
| `69e0d9f1-f9dd-4614-ad9f-13e7153710ce#card` | Dragonclaw Strike | up to one | 38 |
| `6a9cc11b-a223-46f0-bf58-c1e08ad1253d#card` | Forum Filibuster | up to one | 56 |
| `6cb8c627-6b2f-48a7-b2d9-8772defea560#card` | Aphetto Dredging | up to three | 43 |
| `6d819742-3d02-4d60-9b10-82caa49a9d49#card` | Novel Nunchaku | up to one | 2 |
| `6e09ceaf-9005-4b06-ac5a-5e2250d4b663#card` | Decision Paralysis | up to two | 6 |
| `6eefa487-1542-4d24-80bd-4893d9d908c8#card` | Resounding Silence | up to two | 1 |
| `6eff5e17-946b-4433-9f48-88f103844c42#card` | Griffnaut Tracker | up to two | 5 |
| `6fa27f8a-bade-460f-853a-abc1c05c946a#card` | Behold the Sinister Six! | up to six | 12 |
| `6fdddab4-fcc9-4622-9c17-c69f9d4bca08#card` | Render Speechless | up to one | 4 |
| `71362163-2ac3-4f8b-86b9-34fe083e2095#card` | New Frontiers | up to X | 24 |
| `71b81f3f-6092-4819-8529-e27bccfcf088#card` | Ragamuffin Raptor | up to one | 7 |
| `71faaee5-70ff-449c-85c8-560eb189a0a7#card` | Unexplained Absence | up to one | 6 |
| `73905f94-9665-4d82-8927-725d2c4fc4bd#card` | Dissection Practice | up to one | 4 |
| `740f9ab2-fc94-4abc-82b4-6b53116bf1a4#card` | Yawgmoth's Vile Offering | up to one | 21 |
| `75e9ac31-5993-4039-93b2-9380ced2499a#card` | Memory's Journey | up to three | 9 |
| `78780f37-f215-4ae1-adce-6255b0146873#face:1` | Restorative Burst | up to two | 3 |
| `78eb5fb1-cb91-4300-9056-4225a416aec0#card` | Downpour | up to three | 1 |
| `7903666e-7803-4cfc-9ed9-482a6b577c96#card` | Salvation Engine | up to one | 9 |
| `79edf527-2de1-433a-9cac-c91f461b1ebf#card` | Druidic Ritual | up to one | 2 |
| `7a335332-87ff-42dc-a547-c84295dd8138#card` | Unnerving Grasp | up to one | 1 |
| `7aae7c7b-0b31-4b48-a8d3-81c913f604cf#card` | Study Break | up to two | 1 |
| `7ad97dd9-1342-4ed4-bea9-1bb21d748e04#card` | Witch of the Moors | up to one | 9 |
| `7b3c0d89-55ed-43d7-a00a-7b060857ec4e#face:1` | Caetus, Sea Tyrant of Segovia | up to four | 4 |
| `7c520355-1ab8-4d6d-9a29-0f63d6b60024#card` | Cavalier of Dawn | up to one | 5 |
| `7e050495-bed7-43b9-abce-866c61beb1da#face:1` | Pull | up to two | 36 |
| `7ea1b0e8-a577-47e2-b4cb-a6ca973742d0#card` | Rogues' Gallery | up to one | 9 |
| `810f2b50-59c7-490a-8e6a-00f982cd16a0#card` | Rise of Extus | up to one | 11 |
| `823006a8-098c-457a-8546-77ae2457c459#card` | Glider Staff | up to one | 1 |
| `836d074a-8888-42a0-8dce-11ffce00d2f5#card` | Cruel Revival | up to one | 3 |
| `83ccbcb1-7cf0-41a0-8abd-18f4bc647b5f#card` | Katara's Reversal | up to four | 4 |
| `845472ca-3fcb-4bcd-953c-96eb8d914af9#card` | Spider-Byte, Web Warden | up to one | 1 |
| `84681f12-2978-4f62-ab89-b579033c5192#card` | Divergent Equation | up to X | 3 |
| `84d37104-2681-42c1-9d19-91a2ebe6a435#card` | Sigrid, God-Favored | up to one | 24 |
| `85a59833-b661-4599-a5d1-042dee033617#card` | Intrusive Packbeast | up to two | 2 |
| `85e1556d-c4da-4979-a5db-4ce3842c7797#card` | Gathering of Darkness | up to one | 3 |
| `88075828-a76e-412d-b738-182f50e3a133#card` | Demonic Junker | up to one | 8 |
| `882d3be7-1c0f-4d2b-8f2e-32369488ef82#card` | Urborg Uprising | up to two | 3 |
| `8841fdb9-0565-4ef5-814c-293c14242741#card` | Extortion | up to two | 8 |
| `8910f79a-66c9-41b3-b55a-e3e651ae6f26#card` | Amphin Mutineer | up to one | 1 |
| `89793c8c-98a3-4621-ad3d-cfc5949c65da#card` | Utrom Scientists | up to one | 3 |
| `89c89499-23a8-412d-9406-1a7ffc88c64d#card` | Wreck Remover | up to one | 5 |
| `89f1aefd-feb6-45ce-a992-707d027714e7#face:1` | Pixie Dust | up to three | 2 |
| `8a011d0d-da29-4287-b50c-12746abfa38c#card` | Tilling Treefolk | up to two | 3 |
| `8b5add91-2e82-4430-bfc5-f4a6366dbbdb#face:0` | Invasion of Xerex | up to one | 1 |
| `8b9043d3-a1c6-4f49-9c49-ef78bbfbd4ac#card` | Scarab Feast | up to three | 4 |
| `8c1c519a-6edd-4204-b825-b0feca5afa42#card` | Chelonian Tackle | up to one | 4 |
| `8ccb3bca-73da-450d-a9d0-61d3e2bd05a5#card` | Indentured Djinn | up to three | 1 |
| `8cd88e87-6585-437a-bcad-b494fffc0544#card` | Fates' Reversal | up to one | 3 |
| `8d813b72-8ead-4a61-88ff-e93c8d843196#card` | Ghostform | up to two | 3 |
| `8d8c9f7b-92c7-4284-ad9c-304ce42edba5#card` | Molten Primordial | up to one | 36 |
| `8daed4d8-1a43-4452-85dd-51b320714489#card` | Denying Wind | up to seven | 7 |
| `8ed57194-7508-4aef-9373-64f7e80612d8#card` | Treachery | up to five | 1 |
| `8eeb83b4-dca1-497e-a6b7-0485807aeef3#card` | Rise from the Wreck | up to one | 9 |
| `8f48c43e-fa70-4c4f-bb88-be0526303453#card` | Hideous Taskmaster | up to one | 72 |
| `9013214f-c066-4a09-871f-e9f8396f38a4#card` | Stream of Consciousness | up to four | 9 |
| `932177b8-70f8-4908-a48f-fe03204daddc#card` | Light of Judgment | up to one | 1 |
| `93c97e1a-bd7e-4307-8ada-58f53a887cf8#card` | Kemba, Kha Enduring | up to one | 8 |
| `9779f32c-b1a2-42a3-8e78-14c28c3ad254#card` | Sting, Bilbo's Sword | up to one | 6 |
| `97e260c9-3e78-402b-8d97-af4b515b278b#card` | Hammers of Moradin | up to one | 2 |
| `97fad58a-8ee4-4f01-a47a-5ddc9ed845af#card` | Reconstruct History | up to one | 3 |
| `998daf11-c0ed-4801-b013-d8b3136f16d1#card` | Acolyte Hybrid | up to one | 2 |
| `9b5a3765-943b-4925-b008-fc734b2934f7#card` | Baseball Bat | up to one | 1 |
| `9bf2f0d9-57fb-4e44-90a5-260d735076a0#card` | High-Speed Hoverbike | up to one | 1 |
| `9c0f7cdc-9095-40ae-bf83-75a5ffda72b9#card` | Quandrix Command | up to three | 18 |
| `9d3136da-3efa-4103-9d35-8da87d318544#card` | Glarewielder | up to two | 3 |
| `9d581188-ce80-494e-bd38-f411e1f4efb5#face:0` | Bridgeworks Battle | up to one | 4 |
| `9e8e5ad4-05a3-4757-8797-c37bf49506d5#card` | Lead Astray | up to two | 1 |
| `9ed4e48f-91bd-45f9-9f31-a2705b330713#card` | The Art of Tea | up to one | 2 |
| `9f9cdbc2-d2f2-4390-8506-350fabf3f89a#card` | Early Frost | up to three | 1 |
| `9fe57d74-0394-4263-a1b1-da9773b485a6#card` | Double Negative | up to two | 1 |
| `a099d8fa-d51e-4dbc-a03f-6f912097d41e#card` | Martyr of Bones | up to X | 12 |
| `a0cbb1ba-73b1-4418-bbe3-5d1a02007c08#card` | Vibrant Outburst | up to one | 1 |
| `a10a4bb2-c6fa-482b-9c8b-e64cbb8424ba#card` | Grasping Current | up to two | 15 |
| `a1e232c0-dc38-47be-a5a0-f68bc1d86a29#card` | Yawgmoth, Thran Physician | up to one | 4 |
| `a347123d-191a-4758-aab1-9cc7769eac66#card` | Collector's Case | up to one | 3 |
| `a3bdb026-e09d-443f-ba02-fe0d5412fc97#card` | Struggle for Skemfar | up to one | 2 |
| `a3fdb633-7fe9-4e72-a3a3-aa17b6f17864#card` | Commander Sofia Daguerre | up to one | 1 |
| `a60348fa-b182-4667-9e38-15bdf3511f82#card` | Choking Tethers | up to four | 1 |
| `a9564593-b5a6-4a83-be1e-2af8caf647d7#card` | Daring Discovery | up to three | 3 |
| `a97d9a72-2e47-491b-b1db-9829a9abbbd7#card` | All Suns' Dawn | up to one | 5 |
| `a9ddb3dc-f289-46a1-af17-3a8523a26b0d#card` | The Watcher in the Water | up to one | 45 |
| `aa913409-7a7a-4e74-82ff-7ff8195585ad#card` | Roll-Roll-Roll-Roll | up to one | 75 |
| `ab1cc360-b9de-48d9-9983-4dfe4a7d2a37#card` | Arcane Denial | up to two | 81 |
| `abfa5753-8c02-4e2f-8f98-b0636907293a#card` | Blood Fountain | up to two | 3 |
| `ac8fba34-512c-4f24-999a-ab72f1ce4acb#card` | Life from the Loam | up to three | 3 |
| `ac914d98-221e-426c-8a50-342896b15f9e#card` | Snap | up to two | 1 |
| `ac96d27c-dbae-4094-9209-3ef143f64713#card` | Soul of Shandalar | up to one | 32 |
| `ace64875-ca4b-45c7-8b46-29ef2f2629dd#card` | Lyev Decree | up to two | 2 |
| `adc1f3b4-cce7-42c9-a208-dbab1d1924f5#card` | Cloud of Faeries | up to two | 1 |
| `adf59918-a021-4b4e-b672-d82be960726e#card` | Myconid Spore Tender | up to one | 1 |
| `ae7643ea-ef1c-43a3-9605-4e8585239ea3#card` | Storm the Seedcore | up to four | 18 |
| `aeb85786-16fc-45d8-9383-60054a878c40#card` | Fatal Lore | up to three, up to two | 6 |
| `b0672aa3-3207-4ff8-af8f-bdabacbb4d04#card` | Mechanical Mobster | up to one | 5 |
| `b0870f91-c544-446a-8f7b-a8d03fdb0a2a#card` | Obscura Interceptor | up to one | 1 |
| `b0fdf2e0-3025-4919-a9dc-d33100e487fd#card` | Boom Box | up to one | 1 |
| `b34507d3-55ba-4629-9581-7983d0025e89#card` | Powder Ganger | up to one | 1 |
| `b3ea71d2-6298-452a-9c54-7c880b7e9cc0#card` | Unseal the Necropolis | up to two | 2 |
| `b45afde2-eaaf-4174-8082-d344efa54c44#card` | Cost of Brilliance | up to one | 1 |
| `b5483037-5dd2-4282-a201-de2e08aaa76e#card` | Expose Evil | up to two | 1 |
| `b5ba8155-520b-4053-be31-2744b77fb862#card` | Soul of Innistrad | up to three | 18 |
| `b5d856a3-0305-4c3b-b447-99570d6e8195#card` | Vivid Revival | up to three | 3 |
| `b6b2d63f-5b8c-47ee-8712-58596e0e9e94#card` | Dreamdew Entrancer | up to one | 3 |
| `b81894bc-d5ca-44ee-b837-bdb8fec32881#card` | Seismic Shift | up to two | 3 |
| `b8805aa6-fa5f-4c70-ad90-8502611bef5a#card` | Font of Return | up to three | 3 |
| `b98f6ff6-780f-420e-af22-67c9834db2a9#card` | Bilbo's Burglaring | up to one | 3 |
| `ba4d644f-1931-4fc4-aed5-681a476a5a58#face:0` | Cease | up to two | 4 |
| `ba858b68-ae93-4e1c-8a3a-3e4ac97282e8#card` | Moonsnare Specialist | up to one | 1 |
| `bb27bfdf-fe8d-45bd-ad62-8118dce06eda#card` | Rewind | up to four | 1 |
| `bbb4f08f-3016-40c2-ae7c-afc720eeb27a#card` | Waxing Moon | up to one | 4 |
| `bbd0a2a4-f81b-4a6b-bc4d-d7e53dc51333#card` | She-Hulk, Jade Defender | up to one | 6 |
| `bd4ab393-9538-4230-b2a1-bc77099481c9#card` | Iron Hills Stalwart | up to one | 8 |
| `bd5478df-3854-404b-8634-9debcef69b99#card` | Gaea's Blessing | up to three | 54 |
| `bd9b3009-f389-4831-b250-80c4bb8f25e5#card` | Petrifying Meddler | up to one | 6 |
| `be738992-77fe-498d-b219-e5da4ce5bf07#card` | Tidal Surge | up to three | 4 |
| `bf0ee9da-6071-440c-9149-d441f730fcfb#card` | Persistent Constrictor | up to one | 16 |
| `c1346677-d89e-49d0-9819-08805f00bb64#card` | Panic Attack | up to three | 3 |
| `c2032f88-a000-4ed2-a7e5-61d29723d45e#card` | Redeem | up to two | 19 |
| `c43ce30a-0346-479d-8dbc-56c3872ff82a#card` | Great Whale | up to seven | 1 |
| `c469133e-174d-476b-b135-bbf15e415e72#card` | Aggressive Negotiations | up to one | 14 |
| `c5373026-8b43-42a7-aa69-a1b0f1619795#card` | Pull from the Grave | up to two | 3 |
| `c7d8beb6-472c-4758-9876-e8cbf98f4f72#card` | Faerie Macabre | up to two | 4 |
| `c844cc98-b8b9-447d-92ee-64f4a4906e7c#card` | Explosive Getaway | up to one | 18 |
| `c87da757-3daa-4874-a1e4-be0aa2adcb28#card` | Yoshimaru, Scrappy Stray | up to one | 8 |
| `c8da73ed-2834-4094-8723-a4ad75660290#card` | Impractical Joke | up to one | 3 |
| `c8f958da-3a31-4064-9a78-66e9b65b85ab#card` | Dutiful Return | up to two | 3 |
| `c96d67fb-359c-4439-aa1b-598d9bca2880#card` | Invisible Force Field | up to four | 4 |
| `c9739382-9fe0-4d24-b58e-1e45a88491b6#card` | Smell Fear | up to one | 2 |
| `cac993d5-28a1-48b6-9c2f-f216cb5f521d#face:1` | Rubble | up to three | 6 |
| `cba0eae3-73ea-4ea9-87bf-1015e490da90#card` | Azorius Justiciar | up to two | 2 |
| `cbb6139f-827f-490b-b9b7-e8c3ef714d44#card` | Quakefoot Cyclops | up to two | 9 |
| `cdb84830-faf2-4beb-8d57-ad696d75876b#face:1` | Beat a Path | up to two | 3 |
| `d12b875c-0056-4a57-a7ff-09c73d00f8cc#card` | Nightbird's Clutches | up to two | 3 |
| `d39a1e12-bb6b-4e7a-96a4-5dab98084716#card` | Enigma Thief | up to one | 3 |
| `d4d65797-2b92-4265-9169-133120c86c7f#card` | Cityscape Leveler | up to one | 2 |
| `d51136fa-3c13-48a5-83fd-51fe00010a4b#card` | Celebrate the Mountain-king | up to one | 18 |
| `d5cdfa86-a234-4ddd-bfa0-4a3bc51eebce#card` | Saiba Trespassers | up to two | 24 |
| `d64a51be-2dd1-485f-98fa-c93d4c296e68#card` | Carrion Beetles | up to three | 4 |
| `d64adb26-62c4-44aa-bc2e-d05bd1cf657b#card` | Trap Essence | up to one | 1 |
| `d6ea0e58-7b14-425b-9135-57e149c0e47e#card` | Conciliator's Duelist | up to one | 360 |
| `d73191d8-6f94-4fba-acd2-2d0490e3ac00#card` | The Balrog of Moria | up to one | 2 |
| `d7337b13-5459-434a-aa56-e1547b875594#card` | Thorn Mammoth | up to one | 2 |
| `d740dbd9-8e90-4121-8d53-c6ddf5178d58#card` | Chomping Changeling | up to one | 1 |
| `d7947858-f11d-438c-8ad3-991afe950a11#card` | Diregraf Horde | up to two | 40 |
| `d7aaffed-a143-49d6-a516-b9bc8b503aee#card` | Peerless Ropemaster | up to one | 1 |
| `d8eed5f7-8724-45c7-b6fc-dcfcc400f09a#card` | Wave Elemental | up to three | 4 |
| `d8ff239c-cd71-4cd2-9e96-792a116989ca#card` | Courier Bat | up to one | 3 |
| `d96763a0-6a6e-4520-899a-468b4bb307c8#card` | Mass Mutiny | up to one | 30 |
| `d9b99c58-8e2e-496c-8288-eda934f49162#card` | Unmake the Graves | up to two | 3 |
| `d9cc9fc9-522e-4598-9a09-c95824d77acd#card` | Tri-Sentinel, Act of Vengeance | up to one | 2 |
| `da9ec010-8b42-4927-b602-4e96bced4e99#card` | All-Fates Stalker | up to one | 6 |
| `db3b294a-87b1-4dda-b1f6-ea63a9722faf#card` | Urza's Rebuff | up to two | 1 |
| `dbf7d799-98c9-4dc6-a58c-258bd9c19425#card` | Wand of Vertebrae | up to five | 9 |
| `dcc51bcb-a61e-471f-8a01-7fcbc069fcba#card` | Relive the Past | up to one | 44 |
| `de861715-fd0b-493e-9a7c-c470a23044c0#card` | J. Jonah Jameson | up to one | 1 |
| `df9cdc11-cb92-46ce-a83d-eb26e3cd004a#card` | Another Chance | up to two | 2 |
| `dfd2bc6e-32c8-4027-9c0d-8a3050b8eeda#card` | Tempest Owl | up to three | 1 |
| `e262b226-9463-40a9-95b2-fe2c3eafc043#card` | Kenku Artificer | up to one | 6 |
| `e262ea6f-c8c7-45c7-a9a6-29dcbe6300a4#card` | Markov Enforcer | up to one | 4 |
| `e26a08de-6108-4051-9c42-b5ce9dd8b533#card` | Soul-Guide Gryff | up to one | 5 |
| `e2a35850-92c5-40ab-8697-2e995819d7ac#card` | Armaggon, Future Shark | up to three | 1 |
| `e38b8ecb-e7ae-474d-b6a4-29cc1aa8ccd9#card` | Unwind | up to three | 1 |
| `e42a6880-7ca6-498d-abc9-fb7ccd08b111#card` | Rootwise Survivor | up to one | 704 |
| `e451b5aa-5832-42b3-8d72-f67e126a529f#card` | Curious Farm Animals | up to one | 1 |
| `e514546a-ca03-48ba-90e1-7a0e145137f2#card` | Patron of the Moon | up to two | 4 |
| `e5b29094-0bb4-4c18-ad47-dce4d4379a4f#card` | Stream of Thought | up to four | 5 |
| `e5df4597-1647-4ac2-bdb3-a517598d1431#card` | Summer Bloom | up to three | 2 |
| `e69252a6-8c27-4af9-8cbe-11eb416a6a46#card` | Krosan Reclamation | up to two | 9 |
| `e8237d12-2860-4cc8-85a4-458e4c68e69a#card` | Zulaport Duelist | up to one | 3 |
| `e84becc0-6e15-4a74-aec0-9746b86ab5cc#card` | Sokka and Suki | up to one | 8 |
| `ea27c8bf-33c5-443f-b8b2-51235efc2491#face:1` | Dizzying Swoop | up to two | 1 |
| `ea9fc868-374b-4acc-be0d-4283907b4524#card` | Mutant Chain Reaction | up to one | 5 |
| `ebae61bf-6e8c-4cc9-8071-b63e781f6530#card` | Scholar of Combustion | up to one | 65 |
| `ebb445f7-75da-4af4-9153-93d419c838e0#card` | Turn the Earth | up to three | 4 |
| `ec74ae5d-1284-443c-9842-18954f8cf5a8#face:0` | Disruptive Stormbrood | up to one | 1 |
| `ee6a9bac-cf5b-4edd-8cb2-874d8dd0e62a#card` | Haytham Kenway | up to one | 176 |
| `f01c779a-97be-4727-ab62-33579f52447a#card` | Iceman and Firestar | up to one | 4 |
| `f0435065-a8ca-4b4d-a7da-0ef41749118f#card` | The Binding of the Titans | up to two | 40 |
| `f04a7763-5fba-42e5-a45a-b3dc2daba902#card` | Earth Rumble | up to one | 2 |
| `f0c533a2-751a-4c5a-8cec-b83da3c8c62f#card` | Afterthought Sentry | up to one | 10 |
| `f25c5451-58fa-4916-8342-ae88c5e406cc#card` | Wander in Death | up to two | 3 |
| `f75ac0a3-8795-48b4-91fd-4717900ef5b7#card` | Raven Eagle | up to one | 40 |
| `f81327c2-a0e4-475d-bb89-8a42b1ddc93c#card` | Rakdos Firewheeler | up to one | 1 |
| `fa25d142-e6ba-4fc9-81e2-114c32afc030#card` | Space Marine Devastator | up to one | 1 |
| `fb03befb-b4e4-4ae9-908d-a16d5b03b054#card` | Star Athlete | up to one | 1 |
| `fb2600a8-0ba6-4888-ac8a-f206469d4c71#card` | Fear of Immobility | up to one | 1 |
| `fc0abdf3-3a88-4cb2-ba79-35fcaebbf7d2#card` | Wail of War | up to two | 6 |
| `fc467fe5-85ab-4c95-8d21-49096778b107#card` | Apex Altisaur | up to one | 8 |
| `fcb6c6d4-ea8d-49cf-b6df-377084bdb757#card` | Immersturm Predator | up to one | 24 |
| `fe7938ce-6289-4bf8-a8f5-85ecccbb7f86#card` | Graft Surgeon | up to one | 10 |

No-Reading causes remain outside these maximum-count constituents. Sanguine
Indulgence's first cost-reduction/condition sentence and Reasonable Doubt's
Counter-unless-payment sentence still have zero Readings when probed separately;
the whole cards therefore remain uncovered. Torgaar's `life total` Nominal also
has zero Readings; its source has additional vocabulary gaps. Its maximum-count
possessor is independently verified, and the existing targeted genitive witness
uses authentic `target player's graveyard` from Tormod's Crypt.
`up to three times` remains outside FrequencyPhrase because its existing
frequency consumer takes Cardinal, not this PP determiner. Anaphoric bounds
and scalar Amount bounds likewise require their own composition work.
These remaining causes route to live `english-v3-systemic-residuals`; retained
frequency/quantity composition is not silently broadened by recognizing `up`.

The final source inventory has 1,095 no-Reading faces containing
`up to` after reminder removal, of which 117 have lexical gaps.
The following overlapping surface inventories locate residual work; they are
not sole-cause classifications or gain forecasts:

- anaphoric count bounds: 20 faces.
- frequency bounds: 2 faces.
- Arabic scalar bounds: 0 faces.
- distributive each after a maximum subject: 60 faces.

### Deviations and additions

- Added `QuantitativePrepositionPhrase` (one shared schema, two instances),
  `CardinalDeterminer`, and `PrepositionDeterminer`. The latter two are zero-cost
  projections required to keep one counted NP family with the existing compiler,
  whose schema instances require distinct result Categories. No compiler change
  or grammar Construction was deleted.
- Declared Preposition `up` alongside the existing Adverb, and a quantitative
  Complement property on the existing Preposition `to`. The spelling does not
  license an arbitrary Complement or an ordinary locative/adjunct use.
- Glossary gaps closed: Quantitative Determiner and Quantitative Preposition
  Phrase, with CGEL Ch. 5 §4, pp. 357–358 and Ch. 7 §3.2, pp. 624–625. These
  are linguistic terms; no Game Model term or rules citation was changed.
- Tests: restored 0; re-spelled 1 (`independent_slash_consumers_preserve_count_components_and_leaf_order`),
  added 5; removed 0; newly ignored 0. The existing value/leaf equality assertions
  are retained. No subject was removed to obtain green tests.
- No ruling contradiction or regression STOP arose. Test-authoring assumptions
  about singular Reading uniqueness and equality of complete summaries were
  corrected against evidence: `up` has a vowel Onset, unlike bare `one`/`two`.
  Actual Number constraints are checked by exact independent values, finite
  agreement, and rejection of singular/plural mismatches. The existing Torgaar
  nominal gap is disclosed above rather than counted as a maximum-quantity gain.

### REPORT — economy, inventories and performance

V3 covered counts are census provenance; no V3 coverage lock is emitted.

| Quantity | `ntxupumn` / 13,277 | `pnluytkv` / 13,594 |
|---|---:|---:|
| Shared schemas | 43 | 44 |
| Ordinary Constructions | 168 | 170 |
| Named Reading constructors | 211 | 214 |
| Category instances | 306 | 308 |
| Static Productions | 526 | 530 |
| Compiled Productions | 582 | 586 |
| Declaration lines | 2,537 | 2,574 |
| Constructions used in corpus | 158 | 161 |

The exact declared-form homograph inventory (Catalog and Symbol entries excluded)
has 119 surfaces before and 120 after. The added surface is `up`, with owners
`vocab:Adverb/Up` and `vocab:Preposition/Up`. All names follow; the other entries
are unchanged. Grammar form-literal/vocabulary overlap inventory: none on both
measured trees. No grammatical word is supplied by a form literal.

| Surface | Lexical owners |
|---|---|
| 'd | `core-verb:HaveContracted`, `core-verb:WouldContracted` |
| 's | `core-verb:BeContracted`, `core-verb:HaveContracted`, `vocab:Genitive/Default` |
| Adamant | `catalog:ability-words.txt/Adamant`, `lexeme:ability_word/adamant` |
| Addendum | `catalog:ability-words.txt/Addendum`, `lexeme:ability_word/addendum` |
| Alliance | `catalog:ability-words.txt/Alliance`, `lexeme:ability_word/alliance` |
| Battalion | `catalog:ability-words.txt/Battalion`, `lexeme:ability_word/battalion` |
| Bloodrush | `catalog:ability-words.txt/Bloodrush`, `lexeme:ability_word/bloodrush` |
| Celebration | `catalog:ability-words.txt/Celebration`, `lexeme:ability_word/celebration` |
| Channel | `catalog:ability-words.txt/Channel`, `lexeme:ability_word/channel` |
| Chroma | `catalog:ability-words.txt/Chroma`, `lexeme:ability_word/chroma` |
| Cohort | `catalog:ability-words.txt/Cohort`, `lexeme:ability_word/cohort` |
| Constellation | `catalog:ability-words.txt/Constellation`, `lexeme:ability_word/constellation` |
| Converge | `catalog:ability-words.txt/Converge`, `lexeme:ability_word/converge` |
| Council's dilemma | `catalog:ability-words.txt/Council's dilemma`, `lexeme:ability_word/councilsDilemma` |
| Coven | `catalog:ability-words.txt/Coven`, `lexeme:ability_word/coven` |
| Delirium | `catalog:ability-words.txt/Delirium`, `lexeme:ability_word/delirium` |
| Descend 4 | `catalog:ability-words.txt/Descend 4`, `lexeme:ability_word/descend4` |
| Descend 8 | `catalog:ability-words.txt/Descend 8`, `lexeme:ability_word/descend8` |
| Disappear | `catalog:ability-words.txt/Disappear`, `lexeme:ability_word/disappear` |
| Domain | `catalog:ability-words.txt/Domain`, `lexeme:ability_word/domain` |
| Eerie | `catalog:ability-words.txt/Eerie`, `lexeme:ability_word/eerie` |
| Eminence | `catalog:ability-words.txt/Eminence`, `lexeme:ability_word/eminence` |
| Enrage | `catalog:ability-words.txt/Enrage`, `lexeme:ability_word/enrage` |
| Fateful hour | `catalog:ability-words.txt/Fateful hour`, `lexeme:ability_word/fatefulHour` |
| Fathomless descent | `catalog:ability-words.txt/Fathomless descent`, `lexeme:ability_word/fathomlessDescent` |
| Ferocious | `catalog:ability-words.txt/Ferocious`, `lexeme:ability_word/ferocious` |
| Flurry | `catalog:ability-words.txt/Flurry`, `lexeme:ability_word/flurry` |
| Formidable | `catalog:ability-words.txt/Formidable`, `lexeme:ability_word/formidable` |
| Grandeur | `catalog:ability-words.txt/Grandeur`, `lexeme:ability_word/grandeur` |
| Hellbent | `catalog:ability-words.txt/Hellbent`, `lexeme:ability_word/hellbent` |
| Heroic | `catalog:ability-words.txt/Heroic`, `lexeme:ability_word/heroic` |
| I | `vocab:ChapterNumeral/One`, `vocab:SubjectPronoun/I` |
| Imprint | `catalog:ability-words.txt/Imprint`, `lexeme:ability_word/imprint` |
| Infusion | `catalog:ability-words.txt/Infusion`, `lexeme:ability_word/infusion` |
| Inspired | `catalog:ability-words.txt/Inspired`, `lexeme:ability_word/inspired` |
| Join forces | `catalog:ability-words.txt/Join forces`, `lexeme:ability_word/joinForces` |
| Kinship | `catalog:ability-words.txt/Kinship`, `lexeme:ability_word/kinship` |
| Landfall | `catalog:ability-words.txt/Landfall`, `lexeme:ability_word/landfall` |
| Lieutenant | `catalog:ability-words.txt/Lieutenant`, `lexeme:ability_word/lieutenant` |
| Magecraft | `catalog:ability-words.txt/Magecraft`, `lexeme:ability_word/magecraft` |
| Metalcraft | `catalog:ability-words.txt/Metalcraft`, `lexeme:ability_word/metalcraft` |
| Morbid | `catalog:ability-words.txt/Morbid`, `lexeme:ability_word/morbid` |
| More Than Meets the Eye | `catalog:keyword-abilities.txt/More Than Meets the Eye`, `lexeme:keyword_ability/moreThanMeetsTheEye` |
| Opus | `catalog:ability-words.txt/Opus`, `lexeme:ability_word/opus` |
| Pack tactics | `catalog:ability-words.txt/Pack tactics`, `lexeme:ability_word/packTactics` |
| Paradox | `catalog:ability-words.txt/Paradox`, `lexeme:ability_word/paradox` |
| Parley | `catalog:ability-words.txt/Parley`, `lexeme:ability_word/parley` |
| Radiance | `catalog:ability-words.txt/Radiance`, `lexeme:ability_word/radiance` |
| Raid | `catalog:ability-words.txt/Raid`, `lexeme:ability_word/raid` |
| Rally | `catalog:ability-words.txt/Rally`, `lexeme:ability_word/rally` |
| Renew | `catalog:ability-words.txt/Renew`, `lexeme:ability_word/renew` |
| Repartee | `catalog:ability-words.txt/Repartee`, `lexeme:ability_word/repartee` |
| Revolt | `catalog:ability-words.txt/Revolt`, `lexeme:ability_word/revolt` |
| Secret council | `catalog:ability-words.txt/Secret council`, `lexeme:ability_word/secretCouncil` |
| Spell mastery | `catalog:ability-words.txt/Spell mastery`, `lexeme:ability_word/spellMastery` |
| Strive | `catalog:ability-words.txt/Strive`, `lexeme:ability_word/strive` |
| Survival | `catalog:ability-words.txt/Survival`, `lexeme:ability_word/survival` |
| Sweep | `catalog:ability-words.txt/Sweep`, `lexeme:ability_word/sweep` |
| Tempting offer | `catalog:ability-words.txt/Tempting offer`, `lexeme:ability_word/temptingOffer` |
| Threshold | `catalog:ability-words.txt/Threshold`, `lexeme:ability_word/threshold` |
| Undergrowth | `catalog:ability-words.txt/Undergrowth`, `lexeme:ability_word/undergrowth` |
| Valiant | `catalog:ability-words.txt/Valiant`, `lexeme:ability_word/valiant` |
| Vivid | `catalog:ability-words.txt/Vivid`, `lexeme:ability_word/vivid` |
| Void | `catalog:ability-words.txt/Void`, `lexeme:ability_word/void` |
| Will of the council | `catalog:ability-words.txt/Will of the council`, `lexeme:ability_word/willOfTheCouncil` |
| control | `core-verb:Control`, `lexeme:CommonNoun/Control` |
| copies | `core-verb:Copy`, `lexeme:CommonNoun/Copy` |
| copy | `core-verb:Copy`, `lexeme:CommonNoun/Copy` |
| cost | `core-verb:Cost`, `lexeme:CommonNoun/Cost` |
| costs | `core-verb:Cost`, `lexeme:CommonNoun/Cost` |
| counter | `lexeme:CommonNoun/Counter`, `lexeme:keyword_action/counter` |
| counters | `lexeme:CommonNoun/Counter`, `lexeme:keyword_action/counter` |
| cycling | `core-verb:Cycle`, `lexeme:keyword_ability/cycling` |
| deathtouch | `lexeme:counter_kind/deathtouchCounter`, `lexeme:keyword_ability/deathtouch` |
| decayed | `lexeme:counter_kind/decayedCounter`, `lexeme:keyword_ability/decayed` |
| die | `core-verb:Die`, `lexeme:CommonNoun/Die` |
| double strike | `lexeme:counter_kind/doubleStrikeCounter`, `lexeme:keyword_ability/doubleStrike` |
| draw | `core-verb:Draw`, `lexeme:CommonNoun/Draw` |
| draws | `core-verb:Draw`, `lexeme:CommonNoun/Draw` |
| exalted | `lexeme:counter_kind/exaltedCounter`, `lexeme:keyword_ability/exalted` |
| exile | `lexeme:CommonNoun/Exile`, `lexeme:keyword_action/exile` |
| exiles | `lexeme:CommonNoun/Exile`, `lexeme:keyword_action/exile` |
| first strike | `lexeme:counter_kind/firstStrikeCounter`, `lexeme:keyword_ability/firstStrike` |
| flying | `lexeme:counter_kind/flyingCounter`, `lexeme:keyword_ability/flying` |
| goaded | `lexeme:designation/goaded`, `lexeme:keyword_action/goad` |
| harnessed | `lexeme:designation/harnessed`, `lexeme:keyword_action/harness` |
| haste | `lexeme:counter_kind/hasteCounter`, `lexeme:keyword_ability/haste` |
| her | `vocab:ObjectPronoun/Her`, `vocab:PossessiveDeterminerPronoun/Her` |
| hexproof | `lexeme:counter_kind/hexproofCounter`, `lexeme:keyword_ability/hexproof` |
| his | `vocab:PossessiveAbsolutePronoun/His`, `vocab:PossessiveDeterminerPronoun/His` |
| if | `vocab:Preposition/If`, `vocab:Subordinator/If` |
| indestructible | `lexeme:counter_kind/indestructibleCounter`, `lexeme:keyword_ability/indestructible` |
| it | `vocab:ObjectPronoun/It`, `vocab:SubjectPronoun/It` |
| less | `vocab:Adjective/Less`, `vocab:Determinative/Less` |
| lifelink | `lexeme:counter_kind/lifelinkCounter`, `lexeme:keyword_ability/lifelink` |
| menace | `lexeme:counter_kind/menaceCounter`, `lexeme:keyword_ability/menace` |
| name | `lexeme:CommonNoun/Name`, `lexeme:Verb/Name` |
| names | `lexeme:CommonNoun/Name`, `lexeme:Verb/Name` |
| one | `lexeme:CommonNoun/One`, `vocab:CardinalDeterminative/One` |
| reach | `lexeme:counter_kind/reachCounter`, `lexeme:keyword_ability/reach` |
| shadow | `lexeme:counter_kind/shadowCounter`, `lexeme:keyword_ability/shadow` |
| solved | `lexeme:designation/solved`, `lexeme:keyword_ability/solved` |
| suspected | `lexeme:designation/suspected`, `lexeme:keyword_action/suspect` |
| tapped | `lexeme:keyword_action/tap`, `lexeme:keyword_action/tap/adjective` |
| target | `lexeme:CommonNoun/Target`, `lexeme:Verb/Target`, `vocab:TargetingMarker/Target` |
| targets | `lexeme:CommonNoun/Target`, `lexeme:Verb/Target` |
| that | `vocab:SingularDemonstrative/That`, `vocab:Subordinator/That` |
| time | `lexeme:CommonNoun/Time`, `lexeme:counter_kind/timeCounter` |
| to | `vocab:InfinitivalMarker/To`, `vocab:Preposition/To` |
| trample | `lexeme:counter_kind/trampleCounter`, `lexeme:keyword_ability/trample` |
| turn | `core-verb:Turn`, `lexeme:CommonNoun/Turn` |
| turns | `core-verb:Turn`, `lexeme:CommonNoun/Turn` |
| untap | `lexeme:keyword_action/untap`, `vocab:AttributiveAdjective/Untap` |
| untapped | `lexeme:keyword_action/untap`, `lexeme:keyword_action/untap/adjective` |
| up | `vocab:Adverb/Up`, `vocab:Preposition/Up` |
| vigilance | `lexeme:counter_kind/vigilanceCounter`, `lexeme:keyword_ability/vigilance` |
| you | `vocab:ObjectPronoun/You`, `vocab:SubjectPronoun/You` |
| ’d | `core-verb:HaveContracted`, `core-verb:WouldContracted` |
| ’s | `core-verb:BeContracted`, `core-verb:HaveContracted`, `vocab:Genitive/Default` |
| ∞ | `catalog:keyword-abilities.txt/∞`, `lexeme:keyword_ability/infinity` |

### Verification and performance advisory

Gate command: `cargo xtask gate --changed --clippy --run`.

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The derived test closure passes: 652 passed, 0 failed, 2 pre-existing ignored
tests across 72 test/doc-test suites. Strict clippy passes with warnings denied.
Formatting passes. The separately rerun scalar-cardinal suite also passes all
five tests on the final zero-cost declaration; no test was removed or ignored.

Full census command: `cargo xtask english-v3 --all --workers 8
--samples-per-face 0 --output /tmp/scalar-cardinals-after.json`; zero issues.
The five scalar-cardinal tests pass on the final declaration, including the
zero-cost projections. The full census consumes no costs with zero samples;
its admission, identity, roundtrip and traversal results remain applicable after
adding those zero costs. A final 317-face sampled rerun checks cost annotations
without repeating the unaffected full census. Citation checks report 0
noncompliant strings and 0 stale citations; the piped diff audit has no new
Comprehensive Rules sites. No citation blessing is required.

| Tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `ntxupumn` / 13,277 | 8 | 16.49/14.94/12.83 | 272,258,372,661 | 991,280 ns/B |
| `pnluytkv` / 13,594 | 8 | 6.46/9.58/11.35 | 356,590,732,222 | 1,152,138 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling. Wall time
rose from 272,258 ms to 356,591 ms and per-byte thread CPU from 991,280 ns/B to
1,152,138 ns/B. Different host load and overlapping local verification limit
a causal comparison; this remains a performance advisory, routed to
`english-v3-census-tractability`, not a gate or a fitted target.

Scratch provenance: `/tmp/scalar-cardinals-before.json`,
`/tmp/scalar-cardinals-after.json`, `/tmp/scalar-cardinals-subset.json`,
`/tmp/scalar-cardinals-before-identities.jsonl`,
`/tmp/scalar-cardinals-after-identities.jsonl`, `/tmp/scalar-cardinals-delta.json`,
`/tmp/scalar-cardinals-inventory.json` and the final sampled gains report.
