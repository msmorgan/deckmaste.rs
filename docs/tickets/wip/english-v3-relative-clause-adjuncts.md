---
needs: [english-v3-generic-frame-consumption]
---
# Admit adjuncts inside object relative clauses

Pinned witness: **Seedborn Muse**, "Untap all permanents you control during each
other player's untap step." No current Reading places the `during` Preposition
Phrase inside the Object Relative Clause `you control …`.

The user's three-Reading ruling (2026-10-05) licenses attachment to `Untap …`,
to `control` inside the relative clause, and to the Nominal `permanents you
control`. The first and third already exist and must remain admitted. CGEL
Ch. 5 §14.2, p. 446 licenses temporal PPs as post-head Modifiers and stacked
modification; §15, p. 454 describes their essentially labile order. The nominal
attachment's semantic oddness concerns Preference, not Admission.

The `needs:` edge is sequencing only: grammar tickets are worked one at a time.
Reuse the existing clause-level Adjunct machinery rather than adding a
relative-clause-specific recipe. Add no guard naming a word or card.

Acceptance:

- Independently construct Reading 2, with the PP inside `you control …`, and
  establish both roundtrip laws.
- Un-ignore `seedborn_muse_retains_relative_clause_attachment` in
  `tests/reading_support.rs` and merge its assertion into the single Seedborn
  Muse test, so that test asserts all three attachments.
- Compare corpus Reading identities before and after on this ticket's own
  implementation tree, accounting for every addition and loss.

Standard constraints apply.

## Landing record

Implementation tree: `mlsztruk`, covered 13,596. Baseline grammar tree: claim
tree `runqsyyp`, covered 13,596. Both measurements use the same 32,828 supported
face identities, input digest
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`
and lexical inventory digest
`3529edcda18b9a654fead1641ec76614ca3c449d2066a0147d337956b031be07`.
The baseline executable was built before the declaration edit; its report records
`mlsztruk` at report-writing time, which is superseded here by the measured
baseline `runqsyyp`. The baseline Seedborn Muse census is six exact Readings;
both independent identity exports agree exactly with their respective census.
V3 has no coverage lock: covered counts here are its complete corpus census.

### PROVE — retention and structural laws

All 157,307 baseline Reading identities survive byte-for-byte in generated
Reading Debug encoding. The after-tree retains 162,352 identities: 5,045 added,
zero removed, zero duplicate identities, zero lost covered faces. Each added
identity contains at least one newly admitted `PrepositionPredicate` at
`FiniteObjectGap` or `BareObjectGap`. The comparison uses SHA-256 of the complete
generated Reading value per face, without representation normalization, surface
equivalence or Preference pruning. Every affected face and all its new PP
attachment surfaces are listed below; complete hash sets remain scratch evidence
at `/tmp/relative-adjunct-added-identities.jsonl` and the before/after exports.

Every counted corpus Reading passes declaration admission, lexical ownership,
byte-exact realization, construction traversal and leaf traversal comparison
against materialization traces. Issues, cyclic derivations, duplicate
materializations and internal failures are zero on both measured trees.
Independent authored values prove the opposite roundtrip law for the complete
Seedborn Muse Document, its internal Object Relative Clause, and finite/bare
gap predicates. Negative checks independently admit the mismatched predicate
and nominal-only PP before establishing rejection in their surrounding host.

Forbidden word/card/lexeme-named admission guards added: zero. Existing
`modifier.AdverbialUse = Yes` and agreement feature projection apply unchanged.
Lexical environment loading succeeds with zero errors. There is no active
`environment.rs`; loading authority is `deckmaste_lexical_source::load_workspace`.

### DISCLOSE — census and attachment analyses

| Tree / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| `runqsyyp` / 13,596 | 19,232 | 6,648 | 6,948 | 157,307 |
| `mlsztruk` / 13,596 | 19,232 | 6,641 | 6,955 | 162,352 |

Newly covered identities: none. Seven formerly unique faces now retain multiple
Readings; all former unique values survive. Specificity-resolved admission is
zero: Preference does not determine coverage. The added contrast is PP
attachment within `FiniteObjectGap`/`BareObjectGap` versus the retained outer
`PrepositionPredicate`, `ClausalPreposition` and `PostmodifiedNominal` analyses.
The eight Seedborn Muse values include all three user-authorized attachments;
its six previous identities survive and two add internal relative attachment.

The 5,045 added values contain 5,477 finite-gap PP occurrences and 33 bare-gap
PP occurrences (values can contain several occurrences). The bare cases occur
under auxiliaries on Firbolg Flutist, Spreading Insurrection and Toxrill, the
Corrosive. The table names all 198 affected face identities, counts the added
values, and lists every newly observed internal PP surface. Stacked and
lexically ambiguous PPs remain distinct. Shorter PPs before a genitive ending
on Prop Room and Ivorytusk Fortress reflect alternate genitive possessor
boundaries; their grammatical admission adds no semantic plausibility filter.

| Face | Identity | Added Readings | PP surfaces inside object-gap predicates |
|---|---|---:|---|
| Abzan Battle Priest | `1b44fe0a-4a99-4166-b0b3-102b36b54ffa#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Abzan Falconer | `8b972819-507a-40a9-ab1f-1a674ea56083#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Act of Aggression | `9d7624ea-ffff-470b-928b-bbac66ddbd1c#card` | 4 | FiniteObjectGap: until end of turn |
| Aerial Assault | `a4eb06ca-16d8-41c2-8fc6-22e73872b5b0#card` | 2 | FiniteObjectGap: with flying |
| Ainok Bond-Kin | `8a4028cd-6004-4fb0-a044-0bee662a78e1#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Air Nomad Legacy | `9e830359-ab92-4bfd-82b3-d9c67ac80cf7#card` | 1 | FiniteObjectGap: with flying |
| Alela, Artful Provocateur | `936ae5dc-9838-47f3-ba1f-66523a7f5b76#card` | 64 | FiniteObjectGap: with flying |
| Alert Heedbonder | `683a06b6-af77-440f-81b9-8958597a7324#card` | 3 | FiniteObjectGap: with vigilance |
| Angel of Sanctions | `3c49e98d-d33b-47fa-8b07-b38b367de2c7#card` | 2 | FiniteObjectGap: until this creature leaves the battlefield |
| Angelic Arbiter | `34dd789c-4d80-4ccc-a4f7-2e5388a9689d#card` | 6 | FiniteObjectGap: with a creature |
| Armorcraft Judge | `d7f49243-a96e-499f-b2d7-8e9842432420#card` | 28 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Avatar of the Resolute | `15fd66db-f9dd-40ab-92bc-9e0575bd7489#card` | 100 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Aven Gagglemaster | `8c601628-b1ed-467b-ba92-8572343147f8#card` | 3 | FiniteObjectGap: with flying |
| Badgermole | `2084a7f6-226b-4fb4-a30e-703f674d33b4#card` | 3 | FiniteObjectGap: on them; FiniteObjectGap: with +1/+1 counters; FiniteObjectGap: with +1/+1 counters on them |
| Banisher Priest | `9f560b83-32d4-4bb4-a956-8f5db18599db#card` | 1 | FiniteObjectGap: until this creature leaves the battlefield |
| Banishing Light | `f28b21a6-f7ce-437a-8c5b-0423cb55cefb#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Banishment | `9c9b3bf7-afb7-4e9b-bdbc-e2b4d5ba477f#card` | 48 | FiniteObjectGap: as that permanent until this enchantment leaves the battlefield; FiniteObjectGap: with the same name; FiniteObjectGap: with the same name as that permanent until this enchantment leaves the battlefield |
| Baxter, Fly in the Ointment | `94551fbd-a72c-4a1f-be01-7ba1c7c03eed#card` | 9 | FiniteObjectGap: on it; FiniteObjectGap: with a counter; FiniteObjectGap: with a counter on it |
| Blot Out | `9609cd5c-b537-4cb2-9dde-4b12795531f7#card` | 96 | FiniteObjectGap: among creatures; FiniteObjectGap: among creatures and planeswalkers; FiniteObjectGap: among creatures and planeswalkers they control; FiniteObjectGap: with the greatest mana value; FiniteObjectGap: with the greatest mana value among creatures; FiniteObjectGap: with the greatest mana value among creatures and planeswalkers; FiniteObjectGap: with the greatest mana value among creatures and planeswalkers they control |
| Borrowed Time | `f1ac4e0e-5633-46e4-9a21-d3924e481d13#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Bramblewood Paragon | `be455184-c57d-4504-8c34-3be42bc3f04b#card` | 63 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Broadcast Takeover | `e0d221da-16cd-4399-830f-eecc9a0912eb#card` | 4 | FiniteObjectGap: until end of turn |
| Bushmaster, Coiled Henchman | `21067a25-f749-4b79-bc6d-820a91af2834#card` | 7 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Call for Aid | `6d650eef-1156-4e3b-8fa8-90a6ee04d9a8#card` | 16 | FiniteObjectGap: until end of turn |
| Cast Out | `f90b00f6-36e0-4988-9409-57297483a952#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Cathedral Acolyte | `85fa4ed3-14e3-4eb0-b31b-62e2088321d0#card` | 9 | FiniteObjectGap: on it; FiniteObjectGap: with a counter; FiniteObjectGap: with a counter on it |
| Cavalry Master | `7083dedd-b246-4fe0-b1ca-00c490ed9c6b#card` | 2 | FiniteObjectGap: with flanking |
| Celebrate the Mountain-king | `d51136fa-3c13-48a5-83fd-51fe00010a4b#card` | 3 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Chained to the Rocks | `afb0c14c-41ac-4536-bfce-f1c36066b278#card` | 1 | FiniteObjectGap: until this Aura leaves the battlefield |
| Chains of Custody | `5acff7a3-bc84-4251-bd37-80231396ac38#card` | 2 | FiniteObjectGap: until this Aura leaves the battlefield |
| Chevill, Bane of Monsters | `50863075-a30b-4238-8e31-5af32a39d886#card` | 90 | FiniteObjectGap: on it; FiniteObjectGap: with a bounty counter; FiniteObjectGap: with a bounty counter on it |
| Chocobo Knights | `6cbefcba-0c1c-48fd-9b78-905ed61aae1e#card` | 9 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Citizen's Arrest | `6f554418-9ee6-4a03-b81e-f93d5c70873e#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Conclave Tribunal | `e938ee4c-d5df-4d93-bd61-9e518fb1dc30#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Conquering Manticore | `140b58b9-7e82-4feb-99d7-5e8b707b845c#card` | 4 | FiniteObjectGap: until end of turn |
| Consulate Crackdown | `128f7e5a-17e0-4069-a185-c9f874c7d234#card` | 1 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Crack in Time | `a3099143-0146-435f-8d29-cd364aefbc2b#card` | 4 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Crowned Ceratok | `5cdeb1fe-9ce3-4088-a35c-dcfb762f76d8#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Crypsis | `6077e650-32f9-4954-8286-9c8a3f3227ae#card` | 2 | FiniteObjectGap: until end of turn |
| Cynette, Jelly Drover | `8170ddd6-c054-4851-bdc2-4d319b60ba41#card` | 8 | FiniteObjectGap: with flying |
| Deputy of Detention | `0e4150db-ac43-48b4-9791-0d874906acf5#card` | 48 | FiniteObjectGap: as that permanent until this creature leaves the battlefield; FiniteObjectGap: with the same name; FiniteObjectGap: with the same name as that permanent until this creature leaves the battlefield |
| Detention Chariot | `0ea2c5ad-2982-4041-8c9d-a1d31ced11aa#card` | 2 | FiniteObjectGap: until this Vehicle leaves the battlefield |
| Dimensional Exile | `8f0777bc-4fca-4601-beed-1fd060f28075#card` | 2 | FiniteObjectGap: until this Aura leaves the battlefield |
| Double Trouble | `2ef063d2-8702-43a3-a7d0-fc255f8512d3#card` | 2 | FiniteObjectGap: until end of turn |
| Dragonclaw Strike | `69e0d9f1-f9dd-4614-ad9f-13e7153710ce#card` | 10 | FiniteObjectGap: until end of turn |
| Drix Fatemaker | `583102f8-7232-4fbb-a715-087448536bb2#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Drumbellower | `03dee43b-6377-4f7b-956b-a384160322e4#card` | 2 | FiniteObjectGap: during each other player's untap step |
| Dueling Coach | `aae4dd38-3b35-4c92-a0d4-5d37cfd93df8#card` | 7 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Duskfang Mentor | `d8b770c2-0406-4a2d-95d9-e14bb5d3bdc6#card` | 2 | FiniteObjectGap: with lifelink |
| Duskshell Crawler | `e54ad5a1-e79d-42db-a3e9-5caecae62c9f#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Edgar, Moonlit Sovereign | `1917bda6-c0e1-4c80-8009-74c28cf6b8e9#card` | 84 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Eerie Interference | `35ba0e70-da7c-4ed9-b8ec-7dd5c5ce110e#card` | 18 | FiniteObjectGap: by creatures |
| Emberheart Challenger | `5b4b21ed-c24e-4038-83fb-bd7f3c3426cd#card` | 160 | FiniteObjectGap: for the first time |
| Empyrean Eagle | `270d14b2-07bc-46bc-918f-658102265ccf#card` | 2 | FiniteObjectGap: with flying |
| End of the Hunt | `74b8182e-54be-444a-97b1-84fc5fd3c290#card` | 96 | FiniteObjectGap: among creatures; FiniteObjectGap: among creatures and planeswalkers; FiniteObjectGap: among creatures and planeswalkers they control; FiniteObjectGap: with the greatest mana value; FiniteObjectGap: with the greatest mana value among creatures; FiniteObjectGap: with the greatest mana value among creatures and planeswalkers; FiniteObjectGap: with the greatest mana value among creatures and planeswalkers they control |
| Exava, Rakdos Blood Witch | `524249cd-68d9-472a-89cd-5872641ca6de#card` | 7 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Fairgrounds Warden | `388b7b0f-b26d-4de9-bfa6-8c3cbcc2e284#card` | 1 | FiniteObjectGap: until this creature leaves the battlefield |
| Faith Unbroken | `a57fe358-baf3-4eb5-9417-8945b41f84de#card` | 1 | FiniteObjectGap: until this Aura leaves the battlefield |
| Favorable Winds | `2361ca87-6352-4ba3-8d91-b3d71242914d#card` | 1 | FiniteObjectGap: with flying |
| Firbolg Flutist | `5bf88cac-5997-4254-8349-2473de854c50#card` | 8 | BareObjectGap: until end of turn; FiniteObjectGap: until end of turn |
| Flamehold Grappler | `e2d21635-182f-4ebf-a6f5-d4c1fd7f9705#card` | 40 | FiniteObjectGap: when you cast it |
| Flowerfoot Swordmaster | `523d075d-b577-4f24-8c14-9a76761a2eaf#card` | 240 | FiniteObjectGap: for the first time |
| Food Coma | `aab3774c-4ec3-4946-a09b-af0fab7cacb7#card` | 1 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Frillscare Mentor | `3c9d217e-d7f8-43df-9bb6-58d29f93e2c4#card` | 2 | FiniteObjectGap: with menace |
| Gallia, the Merrymaker | `87f39199-3e4b-44fa-8406-62019eb43c10#card` | 21 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Gamma Grotesque | `b92b4894-0786-49a7-8003-d99393f080d4#card` | 56 | FiniteObjectGap: on it; FiniteObjectGap: with a counter; FiniteObjectGap: with a counter on it |
| Gladehart Cavalry | `4d937ea3-bfd4-4cdf-b722-08f2b99ea2a5#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Glorious Protector | `6bdc4996-f7d9-4dc4-b70a-aa63f6816b1d#card` | 3 | FiniteObjectGap: until this creature leaves the battlefield |
| Gnarlid Colony | `e5481431-c952-4ad9-94fe-355076ef632b#card` | 27 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Goblin Researcher | `c8762dfa-a027-4e1c-a2fa-bfdab1de102a#card` | 2 | FiniteObjectGap: with this creature |
| Grasp of Fate | `1b26a54f-6c27-4940-b77e-2a66175c5eff#card` | 3 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Great Ugly-Looking Goblin | `27e17542-549b-4c05-8091-c10a245c916b#face:0` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Hagra Constrictor | `3282c7fb-b43d-4dd6-b5a3-cc7d93e01f19#card` | 15 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Hardlight Containment | `8054092a-1940-4f4b-991d-81f4d384cdb1#card` | 1 | FiniteObjectGap: until this Aura leaves the battlefield |
| Haytham Kenway | `ee6a9bac-cf5b-4edd-8cb2-874d8dd0e62a#card` | 32 | FiniteObjectGap: until Haytham Kenway leaves the battlefield |
| Henchbots | `1f097c91-6c2e-410f-8c8f-c021f39491af#card` | 2 | FiniteObjectGap: until this creature leaves the battlefield |
| Henrika, Infernal Seer | `0c0845c3-f4b5-444e-8f42-da0c7dbf2841#face:1` | 2 | FiniteObjectGap: with flying, deathtouch, and/or lifelink |
| Herald of Secret Streams | `366a218e-84d3-4cf9-bbc8-f2f8ecce92a3#card` | 3 | FiniteObjectGap: on them; FiniteObjectGap: with +1/+1 counters; FiniteObjectGap: with +1/+1 counters on them |
| Hideous Taskmaster | `8f48c43e-fa70-4c4f-bb88-be0526303453#card` | 12 | FiniteObjectGap: until end of turn |
| Hieromancer's Cage | `207c7f93-3abf-44c5-ace6-3f86359c9745#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| High Sentinels of Arashin | `f0e7d147-379e-45b7-bfc4-d926637e060d#card` | 20 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Hooded Blightfang | `342eb666-838c-43d0-bf52-2f704f2a80e3#card` | 12 | FiniteObjectGap: with deathtouch |
| Hornbash Mentor | `ec3031ff-93db-48e9-9e09-335786cb85c2#card` | 2 | FiniteObjectGap: with trample |
| Inspiring Call | `9b9a10ff-5a5d-4df8-88aa-18d84ff9117c#card` | 40 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Inspiring Paladin | `95efbb81-2418-42ed-ae34-5759efff8253#card` | 3 | FiniteObjectGap: on them; FiniteObjectGap: with +1/+1 counters; FiniteObjectGap: with +1/+1 counters on them |
| Isolation Zone | `7c1890cf-36db-4720-a472-9158bdc0d8aa#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Ivorytusk Fortress | `10ae7744-5f98-4d6f-993a-ffb2a34b23e7#card` | 44 | FiniteObjectGap: during each other player; FiniteObjectGap: during each other player's untap step; FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it; FiniteObjectGap: with a +1/+1 counter on it during each other player; FiniteObjectGap: with a +1/+1 counter on it during each other player's untap step |
| J. Jonah Jameson | `de861715-fd0b-493e-9a7c-c470a23044c0#card` | 1 | FiniteObjectGap: with menace |
| Keensight Mentor | `500a17a0-8332-414c-9c3a-7157a7800887#card` | 2 | FiniteObjectGap: with vigilance |
| Keldon Twilight | `c886a076-dbe2-4d80-8dab-537b84d0837a#card` | 20 | FiniteObjectGap: since the beginning; FiniteObjectGap: since the beginning of the turn |
| Kraven the Hunter | `3e2bfa3a-ae83-453a-8e3f-ca6205a9af12#card` | 33 | FiniteObjectGap: among creatures; FiniteObjectGap: among creatures that player controls; FiniteObjectGap: with the greatest power; FiniteObjectGap: with the greatest power among creatures; FiniteObjectGap: with the greatest power among creatures that player controls |
| Kulrath Knight | `f3d1b2e9-6ede-416d-937b-2063ea278d2f#card` | 9 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Kwende, Pride of Femeref | `3adebbbf-39fe-4b42-b719-2f8edb009693#card` | 1 | FiniteObjectGap: with first strike |
| Laid to Rest | `ae6dc45c-b8c9-4cbf-892f-8c00f4199aaf#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Light of Sanction | `c34cf404-729d-46ef-8661-1095e5581766#card` | 30 | FiniteObjectGap: by sources; FiniteObjectGap: by sources you control |
| Liminal Hold | `101407b4-e7e8-4aff-84a2-a97947358b88#card` | 3 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Longshot Squad | `c153e202-1d93-4044-b457-b8ebd5b36ec3#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Luminous Broodmoth | `28c7c816-07e7-42fb-923c-bf149ba28b38#card` | 62 | FiniteObjectGap: without flying |
| Makeshift Binding | `942f3f06-1df0-40b4-8537-71cbec111852#card` | 1 | FiniteObjectGap: until this enchantment leaves the battlefield |
| March from Velis Vel | `25ba6f57-3d50-4c3e-ab55-284e91d684a6#card` | 6 | FiniteObjectGap: until end of turn |
| Marchesa, the Black Rose | `17a59d3d-9e01-48cd-bb4a-3eaaa077751c#card` | 120 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Mass Mutiny | `d96763a0-6a6e-4520-899a-468b4bb307c8#card` | 6 | FiniteObjectGap: until end of turn |
| Meltstrider Eulogist | `046b60da-0a14-40b0-a36c-5328f6b8972b#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Memory Trap | `70616a8c-9b6c-408e-8f3e-2c348ec136d8#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Mer-Ek Nightblade | `56a8554c-d427-44ca-a01d-9e12c89acd1e#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Mightform Harmonizer | `8e025cda-71c5-4d06-8436-decd408667af#card` | 4 | FiniteObjectGap: until end of turn |
| Molten Primordial | `8d8c9f7b-92c7-4284-ad9c-304ce42edba5#card` | 6 | FiniteObjectGap: until end of turn |
| Mouse Trapper | `5b0a4530-6e19-41a2-9ea6-e4c8cf85b6c4#card` | 80 | FiniteObjectGap: for the first time |
| Murkfiend Liege | `61d28182-498f-4bbc-bb7a-c5e1ef872dda#card` | 36 | FiniteObjectGap: during each other player's untap step |
| Mutant's Prey | `3836cf33-bd1d-485d-94a2-85ea210ddf7a#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Mutational Advantage | `2daa5b89-e772-4a2b-ad52-bbbf148c7b2f#card` | 66 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Necroskitter | `536f7c92-2f1e-49e7-98de-47382a9488f1#card` | 15 | FiniteObjectGap: on it; FiniteObjectGap: with a -1/-1 counter; FiniteObjectGap: with a -1/-1 counter on it |
| Nettle Guard | `9ac2dcb6-dacc-4fcb-b820-9fc6744ec503#card` | 240 | FiniteObjectGap: for the first time |
| Ognis, the Dragon's Lash | `a6e50e06-6d1d-457d-8fbb-ec9a866960c3#card` | 1 | FiniteObjectGap: with haste |
| Ogre Geargrabber | `a483b1fd-751a-447e-8e4f-a54b1c194d2c#card` | 5 | FiniteObjectGap: until end of turn |
| Ollenbock Escort | `7a8fa68b-8c87-4ecd-ac29-50fd90c62159#card` | 6 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| On Thin Ice | `6736bb96-6718-48bb-9385-789d3349dc20#card` | 2 | FiniteObjectGap: until this Aura leaves the battlefield |
| Orzhov Advokist | `d5e9942e-096f-433f-8668-4b594f3c1b02#card` | 1 | FiniteObjectGap: until your next turn |
| Ossification | `e29bfd62-286f-4982-813f-7086573c333b#card` | 4 | FiniteObjectGap: until this Aura leaves the battlefield |
| Overgrown Battlement | `585f62dc-4461-42f1-a3a4-b19a1e550d2d#card` | 2 | FiniteObjectGap: with defender |
| Patron of the Valiant | `4d36ea21-e8cb-442d-ba3f-c399289ac208#card` | 8 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Perimeter Captain | `05608055-d97a-4c8f-833d-47b3dd1ea255#card` | 1 | FiniteObjectGap: with defender |
| Pileated Provisioner | `0534e20d-8352-4d9f-a617-378070602673#card` | 1 | FiniteObjectGap: without flying |
| Plumecreed Mentor | `0eb8983e-4e44-4bfe-81dd-ddb41007a166#card` | 6 | FiniteObjectGap: with flying; FiniteObjectGap: without flying |
| Prayer of Binding | `4505ff9f-97f1-45d1-aea9-5590fefb4a4d#card` | 3 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Pridemalkin | `f9672b63-415a-448b-a3da-140df63a0f0c#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Primordial Mist | `afc9e1d3-b012-49e9-8d22-89abbee03c22#card` | 8 | FiniteObjectGap: face up |
| Prison Realm | `808a8491-2966-4388-8590-a46ba147f65a#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Prop Room | `c46a02db-13d6-477f-9da0-822599470168#face:1` | 4 | FiniteObjectGap: during each other player; FiniteObjectGap: during each other player's untap step |
| Quandrix, the Proof | `2cbccc46-bdef-4dfb-90a4-0278c5c8488a#card` | 4 | FiniteObjectGap: from your hand |
| Radiant, Serra Archangel | `4a8227ff-0ae4-4405-8e44-1f5187a7f9f5#card` | 16 | FiniteObjectGap: with flying |
| Rally of Wings | `984336a7-85bd-4549-9771-03a0c2367db0#card` | 2 | FiniteObjectGap: with flying |
| Roar of Endless Song | `a3570093-3c16-42e7-a83d-2ff611a891b5#card` | 5 | FiniteObjectGap: until end of turn |
| Salvation Swan | `0f68ae67-1671-4281-8b6b-3e52fdd4915f#card` | 228 | FiniteObjectGap: without flying |
| Sapphire Drake | `1987bb61-a31b-4d5a-910f-88e9cbc6c8b3#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Sauron, the Lidless Eye | `00b9d3a3-fd64-4757-9159-b3af06b5f5b1#card` | 24 | FiniteObjectGap: until end of turn |
| Seal Away | `c8909015-ea49-47cb-8d37-653904f965bb#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Seal from Existence | `72c4c682-df97-4408-997b-e849f54aba39#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Seedborn Muse | `463865bc-087e-477b-9e86-84e77f1ad931#card` | 2 | FiniteObjectGap: during each other player's untap step |
| Seedglaive Mentor | `f50bbe5e-a839-40e1-ba79-1e96f91bd218#card` | 80 | FiniteObjectGap: for the first time |
| Seedpod Squire | `e2f57e48-ff5b-4a42-9b7c-1bd23b0d78ed#card` | 3 | FiniteObjectGap: without flying |
| Shadewing Laureate | `53a17d4f-cdbc-4669-a30e-5d5a334cc3c0#card` | 1 | FiniteObjectGap: with flying |
| Sheltered by Ghosts | `d13fc657-c6fc-4394-bc70-691050550226#card` | 2 | FiniteObjectGap: until this Aura leaves the battlefield |
| Shire Shirriff | `14f36157-f55f-495d-9ffe-f38135a59750#card` | 1 | FiniteObjectGap: until this creature leaves the battlefield |
| Skatewing Spy | `49701c15-c6f7-4b72-9e89-d34148e5c675#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Skycat Sovereign | `84adce5c-39c7-425e-b163-4a1a3977364b#card` | 32 | FiniteObjectGap: with flying |
| Skyclave Shadowcat | `043ded41-2506-4dab-91c4-142d9287e7d7#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Sonic the Hedgehog | `b07a7b00-9394-48a9-a597-17f391495909#card` | 6 | FiniteObjectGap: with flash or haste |
| Spire Mangler | `6416dae0-0d40-4d88-b30b-e1a0c8380e78#card` | 3 | FiniteObjectGap: with flying |
| Spirit of the Spires | `bf10ce45-c6d6-461e-ab46-06b4205d31fb#card` | 2 | FiniteObjectGap: with flying |
| Spitting Dilophosaurus | `19f2c2c1-cd29-4e66-b5d9-e85e1979d843#card` | 3 | FiniteObjectGap: on them; FiniteObjectGap: with -1/-1 counters; FiniteObjectGap: with -1/-1 counters on them |
| Spreading Insurrection | `bc59977f-b3a7-4bc1-9f20-0a1e36d418ae#card` | 8 | BareObjectGap: until end of turn; FiniteObjectGap: until end of turn |
| Sprite Noble | `bf3e3004-4133-4542-9ee3-2fb98e759ffa#card` | 32 | FiniteObjectGap: with flying |
| Stalwart Shield-Bearers | `55aafc25-76c5-4621-941b-754bd8f13e64#card` | 2 | FiniteObjectGap: with defender |
| Stasis Snare | `6a83768e-672e-4fec-8931-853f5e96d43d#card` | 1 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Static Net | `3f285781-ee03-4a58-a85e-a33f5efffc9d#card` | 6 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Steel-Plume Marshal | `e7ed9c56-c388-4188-98e2-307d35ca3088#card` | 9 | FiniteObjectGap: with flying |
| Stonebrow, Krosan Hero | `5f7f553d-ec94-49e5-ba17-68126a94cdbd#card` | 3 | FiniteObjectGap: with trample |
| Stormplain Detainment | `4ad9c550-5605-4ffe-8e27-e9f2d9ada000#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Super Villain Lockup | `192e150a-96b0-42d6-9d39-03d0bed61f41#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Swarm Shambler | `f46c72b8-e2e8-42e6-a2e7-ffa5810a9bf3#card` | 30 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Swooping Pteranodon | `0b1f6e77-55db-4638-9030-3f4e4d37eb1b#card` | 26 | FiniteObjectGap: until end of turn; FiniteObjectGap: with flying |
| Synchronized Charge | `faa0055b-c1d2-4ce7-875f-41b4be8030bd#card` | 54 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Tangle Wire | `c4ea0931-43e0-4214-9e0b-213eee3a7e69#card` | 68 | FiniteObjectGap: for each fade counter; FiniteObjectGap: for each fade counter on this artifact; FiniteObjectGap: on this artifact |
| Tenured Inkcaster | `780536da-2923-48e7-916a-61c701382d28#card` | 6 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Tesak, Judith's Hellhound | `4117e428-0c78-4dea-9019-dd1836fbb03e#card` | 18 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| The Flesh Is Weak | `dafbb927-3d9d-4be7-b427-f6564b295161#card` | 12 | FiniteObjectGap: on them; FiniteObjectGap: with +1/+1 counters; FiniteObjectGap: with +1/+1 counters on them |
| The Ooze | `82a6fa6f-13c2-4ea3-bec2-c51b56b2783b#card` | 180 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Thopter Arrest | `9b9cc771-3988-41af-a49e-f3bb7a8e397c#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Thrakkus the Butcher | `f7cd7ede-51c1-4117-99b5-484d0d80c95f#card` | 2 | FiniteObjectGap: until end of turn |
| Thunderclap Wyvern | `15276d3b-a117-44bf-87c3-c17e032e4a26#card` | 2 | FiniteObjectGap: with flying |
| Time of Heroes | `fcfb96d7-9b3b-4f60-8714-eebe4d9c002b#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a level counter; FiniteObjectGap: with a level counter on it |
| Toxrill, the Corrosive | `c71b2325-bde6-4364-a93b-8477ffeb25d8#card` | 35 | BareObjectGap: on it; BareObjectGap: with a slime counter; BareObjectGap: with a slime counter on it; FiniteObjectGap: on it; FiniteObjectGap: with a slime counter; FiniteObjectGap: with a slime counter on it |
| Trapjaw Tyrant | `00605b83-faf6-4779-be9d-6a2c8f0ea722#card` | 2 | FiniteObjectGap: until this creature leaves the battlefield |
| Trapped in the Screen | `20f2fc0e-b1d4-4f80-9c7a-a9fb25fef7b8#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Tributary Instructor | `1c70b9d8-b04e-48f8-a4e0-12ce077d22f2#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Triumph of Gerrard | `426a5f3f-f161-49b3-97d0-02cda554752b#card` | 10 | FiniteObjectGap: with the greatest power |
| Trollbred Guardian | `e9c756a5-f4d4-4b84-be60-ad4bc0d1acf9#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Tuskguard Captain | `aa149597-087a-4e1f-953c-2cd04e90a93b#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Twisted Spider-Clone | `58dee718-6820-495a-8c03-5cc130b84167#card` | 8 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Tyrant Guard | `560e6fd6-c061-44ba-b5c5-814d6a14e469#card` | 6 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Unity of Purpose | `9fbcae6e-5523-47a3-b0ff-ab7a8bb72822#card` | 5 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Unnatural Growth | `7324abaa-48da-439d-9339-b0ea5eea612e#card` | 5 | FiniteObjectGap: until end of turn |
| Unwinding Clock | `153fac93-5d2b-4348-a468-a5eef6a12da3#card` | 2 | FiniteObjectGap: during each other player's untap step |
| Vault Guardsman | `0068f8c1-d1a2-4f7b-b39f-963acb2c023b#card` | 2 | FiniteObjectGap: until this creature leaves the battlefield |
| Veteran Guardmouse | `2313a3f9-3eac-4684-a25e-6a6ae445a723#card` | 320 | FiniteObjectGap: for the first time |
| Village Pillagers | `54b44499-398b-47ad-b790-bd5b5994d1a6#card` | 3 | FiniteObjectGap: on it; FiniteObjectGap: with a counter; FiniteObjectGap: with a counter on it |
| Waterkin Shaman | `64e0c580-0e55-4f42-8594-11ccb584d633#card` | 3 | FiniteObjectGap: with flying |
| Web Up | `9b9b086c-d201-4574-a1e9-0d6f8de53d7d#card` | 2 | FiniteObjectGap: until this enchantment leaves the battlefield |
| Whiskerquill Scribe | `cb1e1acc-3805-418b-bed1-65838ddb1d62#card` | 80 | FiniteObjectGap: for the first time |
| White Auracite | `6022608a-6cf2-45bd-adec-63211710a5ed#card` | 2 | FiniteObjectGap: until this artifact leaves the battlefield |
| Wildsear, Scouring Maw | `9ceac4c6-e62e-4445-852c-b2c621e4a83b#card` | 4 | FiniteObjectGap: from your hand |
| Windstorm Drake | `16977ebd-6384-4c28-b9c3-448563a07807#card` | 2 | FiniteObjectGap: with flying |
| Winged Hive Tyrant | `6f0d8900-8460-4b31-8b67-eb4e470c3317#card` | 7 | FiniteObjectGap: on them; FiniteObjectGap: with counters; FiniteObjectGap: with counters on them |
| Wingmantle Chaplain | `99e24041-910e-4ae2-a55b-b938d752eeff#card` | 1,128 | FiniteObjectGap: with defender |
| Wingspan Mentor | `2b3b0e56-0b83-403b-bdfd-b5b7c0accf1b#card` | 2 | FiniteObjectGap: with flying |
| Zegana, Utopian Speaker | `04260930-ff38-4b35-9ec4-e89196a4f2c7#card` | 15 | FiniteObjectGap: on it; FiniteObjectGap: with a +1/+1 counter; FiniteObjectGap: with a +1/+1 counter on it |
| Zidane, Tantalus Thief | `f267fb54-1881-464c-81af-9e03c3f3b41d#card` | 12 | FiniteObjectGap: until end of turn |

The current corpus command emits no legacy permitted-licensing-checker total.
This change adds zero guard expressions and reuses the existing declared-feature
guard; no word-named checker is introduced.

### Deviations and additions

- Reuse `PrepositionPredicate` for two new category instances: finite and bare
  object-gap predicates. The bare instance also admits PP attachment below an
  auxiliary, matching the existing nominal adjunct treatment of both gap types.
  No Construction or schema was added or deleted.
- Tests: restored 1 ignored assertion, merged into the existing Seedborn Muse
  test as requested; re-spelled 1 surviving test name/comment; added 2 tests;
  removed 0 test subjects or asserted outcomes. The former standalone ignored
  `seedborn_muse_retains_relative_clause_attachment` function is folded into the surviving all-three-attachments test.
  No ignore was added; the unrelated pre-existing xtask macro census ignore remains.
- No STOP was required. The three-Reading ruling is preserved, including the
  nominal attachment whose semantic oddness concerns Preference.
- Glossary gaps: none; Relative Clause, Gap, Adjunct, Admission and Preference
  are already defined in the Oracle English glossary.

### REPORT — construction economy and lexical inventories

All following inventory counts are stamped to `runqsyyp` / covered 13,596
and `mlsztruk` / covered 13,596. Both have 170 ordinary Constructions and 44
shared schemas: 214 named constructors. Two category instances are added.
Static Productions rise from 530 to 532; compiled Productions from 586 to 588.
Grammar declaration lines rise from 2,693 to 2,695. Constructions observed in
the corpus: 161 before and 161 after.
Form-literal/vocabulary overlaps: none; form literals are punctuation,
delimiters, symbols and separators. Exact declared-form homographs: 120 on
both trees (catalog-name and symbol categories excluded). The unchanged named
list follows.

| Declared surface | Lexical owners |
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

`cargo xtask gate --changed --clippy --run` derives and runs the closure:

```text
cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The derived Cargo test command passes: 613 passed, zero failed, one unrelated
pre-existing ignore across 64 suites. The initial combined run then reported
Clippy's `replace_box` lint in the new negative test. Replacing the box contents
fixes the allocation; a final focused run passes both new tests, including the
strengthened independent-admission check for the nominal-only PP (its lexical
owner is the declared `vocab:DefiniteMarker/The`). The identical strict Clippy
command above passes on the final tree. Prior complete test and corpus results
remain applicable to the unchanged implementation and all other tests.

Both full corpus commands use `cargo xtask english-v3 --all --workers 8
--samples-per-face 0 --output /tmp/relative-adjunct-{before,after}.json`.
The explicit `--card-name "Seedborn Muse"` subset also reports zero issues.
Formatting passes; citation checks report zero noncompliant strings and zero
stale citations. No Comprehensive Rules citation was changed.

| Measurement tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU ns/B |
|---|---:|---|---:|---:|
| `runqsyyp` / 13,596 | 8 | 19.27/13.49/16.48 | 541,266,100,569 | 1,340,169 ns/B |
| `mlsztruk` / 13,596 | 8 | 26.72/20.75/18.96 | 479,893,548,291 | 1,473,794 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling; this is
a performance advisory, not a gate. Corpus comparison tools, reports and
identity exports remain under `/tmp/relative-adjunct-*`, outside tracked code.
