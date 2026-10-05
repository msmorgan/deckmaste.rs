---
needs: []
---
# Generate type-word lexical uses from their declarations

Centralize type-word noun/modifier uses and negative forms through declaration
macros instead of repeating individual lexical entries. The originating class
controls spelling: nonlegendary, nonartifact and non-Vampire. Preserve existing
noun owners, declared plural exceptions and grammatical category/function
distinctions. Connect type-word premodification and negative modifiers through
declared grammatical properties, not word-named guards. Use authentic Oracle
positive witnesses, including the relevant Skyfisher Spider constituents.
Standard constraints apply; corpus runs use at least eight workers.

## Landing record

### PROVE

Type, Subtype and SpellSubtype macros declare singular noun Premodifier
eligibility and negative-prefix joining. Existing noun owners and their full
number paradigms survive. Joined type negatives and hyphenated subtype
negatives use one shared exporter preserving explicit surface alternatives,
unavailable plurals, countability, source ownership and prefix onset.
Ordinary supertype adjectives use a native class recipe; Legendary keeps its
existing adjective owner rather than gaining a second adjective analysis.

Independent values verify genuine noun modifier bases before hosts and
coordination. The head supplies Number and Countability. Modifier leaves
remain Singular Nouns; post-head dependents cannot enter the atomic modifier.
Complete independent AST comparisons and lexical values verify both
roundtrip laws and ownership, including compound and negative forms.
No lexical/card/construction-named admission guard is added. Generic grammar
constraints read declared properties; lexical source loading succeeds.

All 32,828 source identities reconcile: covered 4,995 → 6,186, with 1,191 gains
and zero losses. Each of the 4,995 previously covered faces retains its exact
Reading count. Complete new corpus enumeration validates 29,653/29,653
Readings, with zero issues, internal failures, duplicates, cyclic derivations,
failed faces or limited enumerations. All Type Lines remain unique.

### DISCLOSE

New modifier constructions: NounPremodifier, NounPremodifiedNominal,
NounPremodifierCoordination, NounPremodifierSeriesEnd,
NounPremodifierSeriesContinuation and SerialNounPremodifier. Binary normal
coordination and three-or-more-item Oxford series connect the independently
verified primitive. Categories NounPremodifier/NounPremodifierSeries and the
NounPremodifier=Yes property are added. No unrestricted Nominal premodifier
route, adjective duplication, or correlative expansion is introduced.

Positive witnesses are attested Oracle constituents with card/context
comments: Skyfisher Spider, Go for the Throat, Anowon, Kalitas, Mishra's Factory,
Time Lord Regeneration, Sanguine Indulgence, Ogre Battlecaster and Custodi
Squire. Lexical tests independently verify default/irregular/syncretic and
unavailable paradigms without inventing standalone instructions.

Unique/multiple census 3,687/1,308 → 4,182/2,004. The 1,191 gains comprise
495 unique and 696 ambiguous faces; no formerly unique face becomes ambiguous.
Readings 13,492 → 29,653; all 16,161 added Readings belong to newly covered
faces. Every gain's sampled selected analysis is listed below; bounded audits
cover all 1,191 selected trees and find no definite incorrect new analysis.
This linguistic audit covers samples; the complete enumeration validates
structural laws, not an independent linguistic judgment for every alternative.

Skyfisher Spider now has 288 complete Readings. An all-288-tree audit finds one
lexical sequence and fixed new modifier leaves, with existing ellipsis and
PP attachment factoring its four sentences as 1 × 2 × 18 × 8. All cost 102;
preference does not yet choose the intended closest attachment. Go for the
Throat and Cast Down have one Reading each; Anowon has two. Ogre Battlecaster,
Custodi Squire and Time Lord Regeneration gain the tested constituents but
still lack complete card coverage because other grammar/vocabulary gaps
remain. No card-specific production is introduced.

STOP and resolution: the initial gate rejects three historical fixtures that
put FixedTerm grammar through Type after that macro declares a noun recipe.
The onset-only fixture now uses CounterKind, with unchanged spelling/onset
assertions. The category-safe ChargeType and path/iteration fixtures retain
Type identity and use Noun(singular:"charge",plural:Unavailable), preserving
one realized surface and every assertion. Full declaration-reader tests pass;
production validation is not relaxed. Three existing tests are re-spelled.

Deviations and additions: SpellSubtype receives the same subtype-class recipe
because it is a separate authoring macro for the same lexical class. Native
supertype adjective formation fills the third requested class without
migrating positive owners. Six generic noun modifier/coordination constructions
connect the new lexical facts. Fourteen tests are added (two metadata, two
native class validation, four noun paradigms, one supertype class and five
noun modifier tests); restored, removed or newly ignored tests: zero.
The glossary defines Type Word and clarifies noun premodification; the lexical
ADR records macro-owned joining and provenance. No recorded ruling is reversed.

### REPORT

Measured change `opopkxsxzqlntxxkxpopnzuktvlksrsq`; covered 6,186 (v3 emits no
legacy English coverage-lock or permitted-licensing-checker field).
Source SHA-256 `49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
Lexical SHA-256 before `dfab2d351583dcd38e3eb29bd2c6fa8b13bc5befec7db3ea8a8bed8bc549777f`;
after `3d56de49f2e3203130fc710231516aa3dfa67247a944ef330c1f79395c28cae1`.

Construction inventory 459 → 465. Lexical owners 34,837 → 35,313 (472 negative
noun owners and four negative adjective owners, each parent/non); no owner is
removed. Basic, Snow and World retain their positive owners while becoming
ordinary adjectives; their exact Type Line catalog atoms stay separate.
Legendary's existing ordinary adjective and unused core Catalog atom both
retain their identities. Independent lexical values 43,116 → 44,412, all pass.
Raw unknown vocabulary remains 9,171 occurrences / 1,279 spellings; before this
change a bound non affix and an underlying noun could already cover those
scalars lexically without supplying a connected negative noun construction.

New distinct-owner lemma homographs: none. New syncretic singular/plural
negative surfaces: non-Aetherborn, non-Eldrazi, non-Equipment, non-Fish,
non-Kithkin, non-Merfolk, non-Myr, non-Plains, non-Samurai, non-Spacecraft,
non-Treefolk and non-Zubera (with initial-capital variants). These preserve
explicit parent number alternatives. New form literals and new literal/
vocabulary overlaps: none; modifier forms reuse space/comma separators.

All 32,828 Type Lines have one Reading, zero failures/issues/limits.
Performance advisory: corpus wall 63.560s versus quiet-host reference 16.26s;
991,453 ns/B checked-text thread CPU; 24 workers; host load
[24.35791015625,13.80517578125,8.00634765625]. Lexical/type-line census and gate
work overlapped; this is not a controlled quiet-host speed comparison.

Derived gate command:
`cargo test -p deckmaste_construction_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The initial `cargo xtask gate --changed --run` stops at the three obsolete
fixtures above. Full construction_core rerun passes 425 unit and 32 integration
tests after re-spelling. The complete remaining five-package command validates
the same dependency closure and exits zero, including whole-corpus and Lean
integration checks. All six required package suites pass. Formatting and
citation checks pass; zero noncompliant strings and stale citations. No CR
citation is added or changed.

### Newly covered identities and selected analyses

Each row identifies the sampled retained analysis's new noun modifier,
coordination or supertype adjective use. Negative owners explicitly name their
source class. All rows refer to the measured source and change above.

- `00225c40-27ff-410f-a591-5c788c6a2bd6#card` — Ruthless Knave: noun premodifiers Treasure.
- `00d1596a-c3e2-4109-86da-388934a0c652#card` — Coeurl: noun premodifiers nonenchantment; negative owners lexeme:type/enchantment/non.
- `015004d7-989b-4549-9ffa-b062126d04d2#card` — Inquisitive Puppet: noun premodifiers Human, creature.
- `01813091-75e8-431b-ba3f-b1ea1ba20a3b#card` — Nimble Thopterist: noun premodifiers Thopter, artifact, creature.
- `01c729d7-9b80-479d-8d30-ae47c76c2c50#card` — Rhonas's Last Stand: noun premodifiers Snake, creature.
- `020963ef-24c0-48cc-8776-bc257df684bc#card` — April O'Neil, Human Element: noun premodifiers sorcery, Mutagen.
- `02af53c1-83e6-447f-908b-2757c8aa445c#card` — Funeral Pyre: noun premodifiers Spirit, creature.
- `02d69413-e040-46b1-8f58-69bde666329d#card` — Walker of the Grove: noun premodifiers Elemental, creature.
- `02dbb6f8-ef84-4156-87dc-34033e64fdb7#card` — Chief Engineer: noun premodifiers artifact.
- `0366c2f6-6e78-4526-808c-fa7bace6006e#card` — Faerie Formation: noun premodifiers Faerie, creature.
- `03931270-504a-4b0b-af3f-918bf124ea07#face:1` — Rocket Volley: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `045ece1e-23aa-4122-8726-86b2af4345ba#card` — Thirst for Identity: noun premodifiers creature.
- `0498ea14-53d9-4655-a616-0ff3cf73de4e#card` — Foundry of the Consuls: noun premodifiers Thopter, artifact, creature.
- `049e9963-1b74-476f-9a81-dbfae543e1c5#card` — Gridlock: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `04cffc41-05da-45fb-82da-2c4d4108c7e1#card` — Nightsquad Commando: noun premodifiers Human, Soldier, creature.
- `0561043a-3896-4bb1-8ff5-ee674ddfea2a#card` — Sengir Autocrat: noun premodifiers Serf, creature.
- `05849bd6-8f38-4031-be2b-e2aa03beb8cc#card` — Pongify: noun premodifiers Ape, creature.
- `05a6a571-643e-429e-8e5c-1c3f8b0dc746#card` — Utvara Hellkite: noun premodifiers Dragon, creature.
- `05e73886-8311-4a0d-9c8d-4a03c7f20ea2#card` — Roost of Drakes: noun premodifiers Drake, creature.
- `0607b289-74a6-4f3a-b15c-a639244873b6#card` — Awaken the Sky Tyrant: noun premodifiers Dragon, creature.
- `06157e34-dbe1-4936-a718-3b1d70726794#card` — Gallows Warden: noun premodifiers Spirit.
- `06343800-b555-4a13-ad4d-19e37d200138#card` — Fangkeeper's Familiar: noun premodifiers creature.
- `06424141-2c26-4ee8-83c2-dac62e3d3540#card` — Soulsworn Jury: noun premodifiers creature.
- `06494c26-3822-43ee-bedf-611c9ec7cc6b#card` — Puppet Conjurer: noun premodifiers Homunculus, artifact, creature.
- `066cd584-773c-4623-be53-8f6feda5a26a#card` — Cabal Stronghold: adjectives basic.
- `0673f4e0-66ff-458c-b4ba-eb067e560cce#card` — Regisaur Alpha: noun premodifiers Dinosaur, creature.
- `0695af85-7405-4512-bb97-42517a532ada#card` — Workshop Warchief: noun premodifiers Rhino, Warrior, creature.
- `06c12fcc-1c08-40a0-a0da-e139c42f083f#card` — Druid of Horns: noun premodifiers Aura, Beast, creature.
- `06e0459f-d691-4470-884a-27c09b6612d2#card` — Muscle Sliver: noun premodifiers Sliver.
- `070aa6c7-4741-47de-b68a-629aa4201ac0#card` — Eager Glyphmage: noun premodifiers Inkling, creature.
- `07180afc-5c63-42e0-907b-0ed52f777165#card` — Sideswipe: noun premodifiers Arcane.
- `0728b195-2dfe-4d73-91ae-e658a97b0b9a#card` — Hard Evidence: noun premodifiers Crab, creature.
- `07462467-e4a3-409e-bcef-9cc92ca4c299#card` — Pegasus Refuge: noun premodifiers Pegasus, creature.
- `079d4698-dc11-4d29-91b6-725d78dfb75d#card` — Rend Flesh: noun premodifiers non-Spirit; negative owners lexeme:creature_subtype/spirit/non.
- `079f0d7f-57a5-4101-a9e3-c5a57148b3fb#card` — Elder Auntie: noun premodifiers Goblin, creature.
- `0826cf7d-7ccc-459b-a92f-29dc169628f8#card` — Muddle the Mixture: noun premodifiers instant, sorcery; binary modifier coordination.
- `0868ebbb-2fe9-4f53-9697-acad8cda5ec1#card` — Greed's Gambit: noun premodifiers Bat, creature.
- `087018b6-6355-4534-9a35-9336b4b7fccc#face:1` — Assemble: noun premodifiers Elf, Knight, creature.
- `08740dda-0629-4b6f-bbcc-c8088105b5e7#card` — Omen of the Sun: noun premodifiers Human, Soldier, creature.
- `08b3328c-1d96-4a05-ae8b-f1b654084faa#face:1` — Extricator of Flesh: noun premodifiers non-Eldrazi, Eldrazi, Horror, creature; negative owners lexeme:creature_subtype/eldrazi/non.
- `0935377b-384f-4e4c-9fbb-8d2a7d5dc280#card` — Dwarven Miner: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `09386931-59bd-481f-bccc-32e8a8abc0f9#card` — Elephant Ambush: noun premodifiers Elephant, creature.
- `0944ec2e-1dd9-459f-8f1d-667242cf52fe#card` — Rapacious Dragon: noun premodifiers Treasure.
- `09a70ae8-3859-4a09-901d-dce063fa3b5f#card` — Wasteland: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `0a2075b5-9609-433d-bcd2-e0a637456cf8#card` — Maja, Bretagard Protector: noun premodifiers Human, Warrior, creature.
- `0a85402a-e3be-4ebb-8001-9d9a20dde7e8#card` — Fungal Plots: noun premodifiers creature, Saproling.
- `0a911cb2-caae-4378-bf09-1c5b0751dd35#card` — Cast Through Time: noun premodifiers instant, sorcery; binary modifier coordination.
- `0abc993b-18c7-4bb8-aa58-0d06a41cb48f#card` — Gisa's Bidding: noun premodifiers Zombie, creature.
- `0abcf934-2552-4885-bbe7-38d93ce0e642#card` — Penumbra Kavu: noun premodifiers Kavu, creature.
- `0adcf8d2-6eb9-4679-8ad5-f28e2f4aba39#card` — Student of Ojutai: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `0b8c3337-04dd-4798-8203-6d8b8cfb936b#card` — Lingering Souls: noun premodifiers Spirit, creature.
- `0b96cd7a-2132-4ff2-9666-6e2fbd0a2160#card` — Thurid, Mare of Destiny: noun premodifiers Pegasus, Unicorn, Horse, creature; Oxford modifier series.
- `0c052420-2d8b-4bbf-8197-b8a36e7208c9#card` — Loyal Fire Sage: noun premodifiers Ally, creature.
- `0c201f79-9eca-48c8-88ba-a20fa6918a07#card` — Anowon, the Ruin Sage: noun premodifiers non-Vampire; negative owners lexeme:creature_subtype/vampire/non.
- `0c26ab0d-80f6-4e5b-9d0e-af17c1519583#card` — March of the Multitudes: noun premodifiers Soldier, creature.
- `0c572396-4e53-4c86-9b04-f41341dcea05#card` — Crucible of Fire: noun premodifiers Dragon.
- `0c73f566-3b8a-4d57-b7a9-7437b6f1f66d#card` — Flamewright: noun premodifiers Construct, artifact, creature.
- `0c8ab7bf-7b6a-4a34-b988-9f3f4d84a9ea#card` — Flurry of Horns: noun premodifiers Minotaur, creature.
- `0d3617c3-1526-40e6-bc90-6139b5d3e1f1#card` — Opal Champion: noun premodifiers creature, Knight.
- `0dbd6e47-a8b4-4268-ba44-8924cd4963a6#card` — Drill Bit: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `0dd0e91a-d16b-4718-8d11-1a3fcf8e0753#card` — Thragtusk: noun premodifiers Beast, creature.
- `0e4150db-ac43-48b4-9791-0d874906acf5#card` — Deputy of Detention: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `0e9d8e12-b9da-4452-a758-5caffd8a752e#card` — Magitek Armor: noun premodifiers Hero, creature.
- `0ec88e35-191b-411b-9eb0-2dfef7ae4174#card` — Belonging: noun premodifiers Shapeshifter, creature.
- `0f2335bf-6a13-4483-997a-fd19d30c4d48#card` — S.H.I.E.L.D. Helicarrier: noun premodifiers Soldier, creature.
- `0f909cfd-c643-48d2-983a-da2e38cd2bff#card` — Fen Hauler: noun premodifiers artifact.
- `0f93d88c-9d2e-416d-a10b-99483360b1fb#card` — Zurgo Stormrender: noun premodifiers creature.
- `0fb9a928-9adb-4dc1-9761-857df7e6c643#card` — Steamcore Scholar: noun premodifiers instant, sorcery, creature; binary modifier coordination.
- `0fc64fd6-f057-4056-9dca-47accb7ff036#card` — Sythis, Harvest's Hand: noun premodifiers enchantment.
- `0fea3406-e5c7-4d5c-9890-6f1bc04a7cb8#card` — Brandywine Farmer: noun premodifiers Food.
- `105cea6e-18e9-4dec-a533-0dca4d41320b#card` — Vampire's Kiss: noun premodifiers Blood.
- `106e343f-c6f4-4155-ad01-63e482d4d38e#card` — Groundshaker Sliver: noun premodifiers Sliver.
- `10762505-ebaa-47de-9f3e-08d48f2ac240#card` — Beckon Apparition: noun premodifiers Spirit, creature.
- `107e9ada-4e2e-4032-addd-274bca956621#card` — Zoo Escapees: noun premodifiers Mutagen.
- `10b0b60f-9976-41f9-855b-1786b6f86f55#card` — Elvish Soultiller: noun premodifiers creature.
- `10da681b-50bd-4a1b-9934-f0c90e0af55c#card` — Tectonic Reformation: noun premodifiers land.
- `10e95489-a94d-4523-964c-ec9753103a62#card` — Blanket of Night: noun premodifiers land.
- `110e5695-a9df-4201-9230-99f6abe9978f#card` — Blade Sliver: noun premodifiers Sliver.
- `1141a12a-2ebd-4e7b-8c8c-c7a1f160244c#card` — Rime Tender: adjectives snow.
- `115bd33d-4875-4014-ae9f-0052f5acdbb2#card` — Snake Pit: noun premodifiers Snake, creature.
- `11809d9b-30f3-49d5-9119-e45b4de044c8#card` — Knight Luminary: noun premodifiers Human, Soldier, creature.
- `11a20648-fdb6-4f52-8fda-c66985e0c044#card` — Shamble Back: noun premodifiers creature, Zombie.
- `11cb509d-21af-48f1-b355-135ebd3e4bd1#card` — Doomed Dissenter: noun premodifiers Zombie, creature.
- `11dfe74c-bc8e-4d58-b526-41ea5b409135#card` — Meletis Charlatan: noun premodifiers sorcery.
- `11f17f85-ca97-4551-838f-7cb32f0e5f10#card` — Argothian Enchantress: noun premodifiers enchantment.
- `11f1d2b7-0c2b-40df-bf90-3c55b23449af#card` — Farmer Cotton: noun premodifiers Halfling, creature, Food.
- `11f55ec5-b9a3-4670-8afb-31d29478fb59#card` — Bar the Gate: noun premodifiers creature, planeswalker; binary modifier coordination.
- `1201c146-54af-4668-a544-cde3ece23f2e#card` — Halo Scarab: noun premodifiers Treasure.
- `121bd1af-da21-4937-a911-377c5b430796#card` — Intimidation Tactics: noun premodifiers creature.
- `1251691e-3c4c-4e45-9f59-f156852f2268#card` — Lossarnach Captain: noun premodifiers Human, Soldier, creature.
- `1267dfda-eb1a-4963-9fe3-fa619d924d7a#card` — Agonizing Remorse: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `132786bb-157c-4e2c-8f82-13df6a667448#card` — Dross Scorpion: noun premodifiers artifact.
- `13e8d063-eece-4f38-83f6-02403130defa#card` — Brazen Freebooter: noun premodifiers Treasure.
- `140cb5cf-8aa2-457c-9d65-5acefea00195#card` — Eternal Student: noun premodifiers Inkling, creature.
- `14537413-3628-4152-b23f-78b0e9c416ac#card` — Broodwarden: noun premodifiers Eldrazi, Spawn.
- `147d1e5c-fcfd-4115-a2f9-bea9b7ad7bb6#face:0` — Intrepid Trufflesnout: noun premodifiers Food.
- `149694e6-5cf0-4983-9546-a4bd579b2ade#card` — Scrapwork Cohort: noun premodifiers Soldier, artifact, creature.
- `14d14ded-8831-47ad-ad90-f6f242cccf03#card` — Leonin Iconoclast: noun premodifiers enchantment.
- `1566af29-bd7b-41bb-a589-499348fbd32c#card` — Soul of Magma: noun premodifiers Arcane.
- `15fb3a7a-a6c3-4bef-9f72-1d04b623063e#card` — Molten-Tail Masticore: noun premodifiers creature.
- `15fc4e74-300e-4c2d-8ed7-004553b2f7c2#face:0` — Ondu Inversion: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `1610875f-48da-490c-a371-c439dc36f533#card` — Barrow Ghoul: noun premodifiers creature.
- `1657be64-b38f-4074-a743-bf558dc57901#card` — Harsh Annotation: noun premodifiers Inkling, creature.
- `169728d6-7820-4a6d-a815-70714ad3e887#card` — Perilous Predicament: noun premodifiers artifact, nonartifact; negative owners lexeme:type/artifact/non.
- `16dfd80a-9847-45b8-954f-2af9b7cfa21d#card` — Roc Egg: noun premodifiers Bird, creature.
- `16e03598-30bf-4b6c-9b88-36c3c5c010dd#card` — Cursed Monstrosity: noun premodifiers land.
- `18207fe8-41e4-418d-b5f9-e1239c527e67#card` — Extract the Truth: noun premodifiers planeswalker.
- `184c763c-b52a-471c-bd87-d01729ebe5e4#card` — Glittermonger: noun premodifiers Treasure.
- `185316ff-6fb7-4b5c-8a60-3238674562a6#card` — Invasion Reinforcements: noun premodifiers Ally, creature.
- `18563bc9-6090-4629-b304-89a67d93f635#face:0` — Tovolar's Huntmaster: noun premodifiers Wolf, creature.
- `18563bc9-6090-4629-b304-89a67d93f635#face:1` — Tovolar's Packleader: noun premodifiers Wolf, creature.
- `18d6b89a-3588-43d9-a247-4747a7d784fc#card` — Balduvian Dead: noun premodifiers creature, Graveborn.
- `19053c47-97ce-4a25-a92c-1f4061e85bf3#face:1` — Warden: noun premodifiers Sphinx, creature.
- `191ec087-7817-4c56-b2d5-66de5c663536#card` — Lyev Skyknight: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `192dd1e2-b6c2-432a-a1b4-e52ff39b3db2#card` — News Helicopter: noun premodifiers Human, Citizen, creature.
- `193f9b12-ee34-40c2-9315-6794416a4f62#card` — Clarion Cathars: noun premodifiers Human, creature.
- `194166ad-0179-42a3-86b9-ba7f322ec576#card` — Blade Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `195bd8fe-581c-44ed-b7f8-e800df3502ff#card` — Gorion, Wise Mentor: noun premodifiers Adventure.
- `1962d9b3-f057-4d2c-9c5b-386d9a66148a#card` — Goblin Cratermaker: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `196d6ef6-80d9-4bfd-a5ed-d9e8b00faad0#card` — Salvage Slasher: noun premodifiers artifact.
- `1a8d7714-a566-4195-8605-550929584c61#card` — Patrol Signaler: noun premodifiers Kithkin, Soldier, creature.
- `1ad604ed-c645-4f94-8c5f-8e0ce66a85ff#card` — Felhide Petrifier: noun premodifiers Minotaur.
- `1ad78038-44f9-4599-84db-fc1a87d9ed45#card` — Galerider Sliver: noun premodifiers Sliver.
- `1b2ad355-3f1e-4c8b-93ee-848934646b23#card` — Winnow: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `1b388371-f9ef-45b4-82a3-ca20a8cd7807#card` — Dovin's Veto: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `1b9ec782-0ba1-41f1-bc39-d3302494ecb3#card` — Bake into a Pie: noun premodifiers Food.
- `1bd5fd97-f898-4e17-9e8f-5fa93b9eef71#card` — Renewing Touch: noun premodifiers creature.
- `1be56a3d-a6c0-4b65-ae71-3d90ceefc6c0#card` — Sporemound: noun premodifiers Saproling, creature.
- `1c0a53fc-8037-46e3-90ea-cb8b73631a83#card` — Field Marshal: noun premodifiers Soldier.
- `1c63c4ca-5d5a-4ab9-af0d-ed1254b581cc#card` — Wanted Scoundrels: noun premodifiers Treasure.
- `1cad92ce-55c8-4b78-8d8c-56645ef8e6ee#card` — Firespitter Whelp: noun premodifiers Dragon; negative owners lexeme:type/creature/non.
- `1d0f1186-be6c-45c3-9703-f0c1e13892fb#card` — Sentinel Sliver: noun premodifiers Sliver.
- `1d95ad32-768f-40a1-a461-e0562326b2b7#card` — Bonescythe Sliver: noun premodifiers Sliver.
- `1d9bd0a5-3ba0-44e5-9cb3-966f6b9dc14a#card` — Ravenous Baboons: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `1e05e6ef-14af-451d-9d54-e75b1f8871ab#card` — Thirst for Discovery: noun premodifiers land; adjectives basic.
- `1e2c4508-54ad-45c2-b35f-678f8c376b27#card` — Stronghold Biologist: noun premodifiers creature.
- `1e51fab7-3ca5-4fbb-a1e9-b39c842895e8#card` — Cyberman Patrol: noun premodifiers artifact.
- `1e67b4eb-e5d2-4368-8284-4ba5c4c42455#card` — Creakwood Liege: noun premodifiers Worm, creature.
- `1ea71ea2-65ca-4907-b857-dbd1ba8efacd#card` — Cerebral Confiscation: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `1eb9e401-8975-447b-839d-f7cd23897465#card` — Gleaming Barrier: noun premodifiers Treasure.
- `1ed80dd0-0980-4830-aad2-5b78ca373098#card` — Piper's Melody: noun premodifiers creature.
- `1f592b83-7838-4466-8bcb-093ac7346c55#card` — Wandering Stream: noun premodifiers land; adjectives basic.
- `1f789fcf-3df6-45a6-a732-9f43e33718d6#card` — Hunted Troll: noun premodifiers Faerie, creature.
- `1fc31651-70fe-4a58-9b7e-7359510bba51#face:1` — Signaling Roar: noun premodifiers Soldier, creature.
- `1fde570c-aa96-45ca-a6be-f59c8e4f4b6a#card` — Hunted Witness: noun premodifiers Soldier, creature.
- `20150b58-a442-472c-8191-b44581322eeb#card` — Mass Production: noun premodifiers Soldier, artifact, creature.
- `2065e9d0-c6e9-4270-898e-9822b532d467#card` — Cogworker's Puzzleknot: noun premodifiers Servo, artifact, creature.
- `207c7f93-3abf-44c5-ace6-3f86359c9745#card` — Hieromancer's Cage: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `21017dc0-9b79-48b8-8eae-561abfad31d5#card` — Elvish Eulogist: noun premodifiers Elf.
- `210940f1-3c11-4877-bbfc-2429c03f98ee#card` — Masked Vandal: noun premodifiers creature.
- `212d058b-69c6-4dc2-8c93-bdfe26dc2ffe#card` — Hammer of Purphoros: noun premodifiers Golem, enchantment, artifact, creature.
- `21aaecc7-b9ec-4068-8e47-4dffaa12f102#card` — Broadcast Rambler: noun premodifiers Thopter, artifact, creature.
- `21b8192e-46a2-4a6a-abd5-3a074efa30a9#card` — Burning Earth: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `21d269f2-c11c-4094-b526-500f0785f483#card` — Clockwork Gnomes: noun premodifiers artifact.
- `21eea63f-72b2-4155-bfad-f9937a8f8614#card` — Falcon Abomination: noun premodifiers Zombie, creature.
- `228fbae1-423e-461d-b8c3-55786938a3cb#card` — Thermo-Alchemist: noun premodifiers instant, sorcery; binary modifier coordination.
- `2315b044-bfd2-40dd-a73e-4cf064b36999#card` — Hidden Gibbons: noun premodifiers instant, Ape.
- `233e3b4b-faaf-451d-8a2e-e9f5a6ced671#card` — Giant-Sized Flying Ant: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `234a734b-ba28-4f1b-9d01-3c3e7d516590#card` — An Offer You Can't Refuse: noun premodifiers noncreature, Treasure; negative owners lexeme:type/creature/non.
- `23c3de07-8aa2-48e9-848c-50b5a40bd133#face:1` — Craft with Pride: noun premodifiers Treasure.
- `244443f0-c92b-4640-88b6-308b0312c8be#card` — Sliversmith: noun premodifiers Sliver, artifact, creature.
- `2450d35b-7d87-4885-b396-f8e1d3ac0b22#card` — Battleground Geist: noun premodifiers Spirit.
- `24a691f3-88dc-47a5-b5cd-e02bb8026470#card` — Cruel Administrator: noun premodifiers Soldier, creature.
- `24e96682-14ca-4a62-a920-2d686c8a5dbf#card` — Winged Sliver: noun premodifiers Sliver.
- `24fe8bc3-7e3a-435b-a232-7f80787aba1f#card` — Fable of Wolf and Owl: noun premodifiers Wolf, creature, Bird.
- `250f8642-9754-48fd-8f09-70ed13d7a42c#card` — Preemptive Strike: noun premodifiers creature.
- `2522a9dd-bcf3-4b84-abff-52b9e6dbde34#card` — Bag End Banquet: noun premodifiers Food.
- `25794033-bd17-4056-b116-68d3acf52569#card` — Myr Sire: noun premodifiers Phyrexian, Myr, artifact, creature.
- `25cb5c86-83cd-4a06-b0d1-a0f6fc9158a6#card` — Goblin Offensive: noun premodifiers Goblin, creature.
- `25d955e3-2b98-4a75-bb51-6072000111be#card` — Ayula's Influence: noun premodifiers land, Bear, creature.
- `2608f905-a678-4e40-9e73-cf1c42ec880d#card` — Rageblood Shaman: noun premodifiers Minotaur.
- `27083627-a5d0-4fc6-9e66-fa44803ef080#card` — Elturgard Ranger: noun premodifiers Wolf, creature.
- `276f49ee-7cf3-4a4d-9b14-74aedcbef69f#card` — Nature's Resurgence: noun premodifiers creature.
- `27a9e0af-76f0-4910-9f93-2a3617b4e79a#card` — Spore Burst: noun premodifiers Saproling, creature, land; adjectives basic.
- `27c3d7cd-f92e-4203-9fb5-f6b4776f6ffd#card` — Shivan Harvest: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `27d6b1fb-ce53-45d6-a601-9f5b9ee7ab2f#card` — Guardian of Cloverdell: noun premodifiers Kithkin, Soldier, creature.
- `27d80a75-688e-4f68-9622-54484dd3bd91#card` — Unyaro Griffin: noun premodifiers sorcery.
- `28274c39-19e2-4b4c-bd53-d31d6063c2ba#card` — Weaver of Lightning: noun premodifiers sorcery.
- `284b0605-4b33-499e-aff3-e8b9edcce39b#card` — Psychic Spear: noun premodifiers Arcane.
- `288afdb9-708d-4a52-991a-e1b07f62ee99#card` — Nuisance Engine: noun premodifiers Pest, artifact, creature.
- `28a972a6-cf67-4ee3-aa91-f7b2549f6c48#card` — Castigate: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `291e4b91-17c7-4b7b-8057-f0d361424016#card` — Gelatinous Genesis: noun premodifiers Ooze, creature.
- `2987c385-011a-4032-a516-a46d1e9dc9e8#card` — Crib Swap: noun premodifiers Shapeshifter, creature.
- `2ac6f1c7-f4c5-4d45-9644-da49d8fe4758#card` — Hisoka's Defiance: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `2b2b3b16-389c-4581-b889-b8a2e76f83a2#card` — Meticulous Artisan: noun premodifiers Treasure.
- `2baf2ee6-e290-4a2b-89a0-0e725edde06d#card` — Allied Reinforcements: noun premodifiers Knight, Ally, creature.
- `2bd91ce8-1797-4dd7-8150-0d4caa7dad61#card` — Kavu Scout: noun premodifiers land; adjectives basic.
- `2cb96408-9195-4e41-ad9a-f8c74fbad083#card` — Torens, Fist of the Angels: noun premodifiers creature, Human, Soldier.
- `2cbccc46-bdef-4dfb-90a4-0278c5c8488a#card` — Quandrix, the Proof: noun premodifiers instant, sorcery; binary modifier coordination.
- `2d338aa5-22bc-4cc4-a1e0-cc5018ce001e#card` — Gluttonous Guest: noun premodifiers Blood.
- `2d3e6549-6cc6-434f-a189-ba3b55e64c34#card` — Rampaging Baloths: noun premodifiers Beast, creature.
- `2d4aedc5-31c5-4281-98e1-b0c2233c3c8a#card` — Artifact Blast: noun premodifiers artifact.
- `2d56a491-b3eb-4f3e-82ab-981d3036c45d#card` — Krang, Utrom Warlord: noun premodifiers artifact.
- `2d6a387f-7ad4-419e-951a-9dcd4c9ac823#card` — Flow of Maggots: noun premodifiers non-Wall; negative owners lexeme:creature_subtype/wall/non.
- `2db8dda3-8136-4cf2-ba2d-ecc344a36725#card` — Mechanized Ninja Cavalry: noun premodifiers Robot, artifact, creature.
- `2e712090-2284-4325-8ee8-38cd0e2bedd8#card` — Psychic Barrier: noun premodifiers creature.
- `2e735849-1930-4f43-86b4-da660ade15bc#card` — Fire Nation Archers: noun premodifiers Soldier, creature.
- `2e9289d6-dbc6-456d-88cf-d1f534e731d6#card` — Firebrand Archer: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `2ea95e03-b95d-4ccf-afc6-1a26ac6cf354#card` — Sergeant-at-Arms: noun premodifiers Soldier, creature.
- `2f092562-9e17-43cd-aeb8-d0567f99363e#card` — Go for the Throat: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `2f26f270-26d9-46d3-957f-19f52d51eb03#face:1` — Avatar Roku: noun premodifiers Dragon, creature.
- `2f3f5a1c-8b88-4080-bd80-14db4d6c8c50#card` — Eusocial Engineering: noun premodifiers Robot, artifact, creature.
- `2f70f1bb-29fa-4abb-afc2-653acd0a08b9#card` — Woodfall Primus: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `2f865ae9-9328-4f6f-924f-d5e0a65aaa96#face:1` — Bathe in Gold: noun premodifiers Treasure.
- `2fa94a07-c932-4f85-b6e0-a97d2b29eb52#card` — Electrostatic Field: noun premodifiers instant, sorcery; binary modifier coordination.
- `2fd01a30-0c0e-4ac5-b96d-b2a6dad31f0c#card` — Buy Your Silence: noun premodifiers nonland, Treasure; negative owners lexeme:type/land/non.
- `30062dd0-c049-4872-8011-8b4810a3fa26#card` — Envelop: noun premodifiers sorcery.
- `30536d1f-8b1a-474f-a508-d3426480a532#card` — Call to the Feast: noun premodifiers Vampire, creature.
- `30ab812f-a055-4c68-ad81-9dd23e38827f#card` — Knight of the New Coalition: noun premodifiers Knight, creature.
- `31338e24-a437-4587-acab-cec46be021e7#card` — Harsh Scrutiny: noun premodifiers creature.
- `3165fe8f-52d7-40f7-bb14-8f4300a564e6#card` — Night Soil: noun premodifiers creature, Saproling.
- `31816924-1430-4581-86e8-325e2ca16217#card` — Agents of HYDRA: noun premodifiers Villain, creature.
- `31be5bf4-9950-456c-8365-74b48e132ef5#card` — Bone to Ash: noun premodifiers creature.
- `31fefd48-6fff-41bf-a559-35a1699357ca#card` — Feral Incarnation: noun premodifiers Beast, creature.
- `32001382-3079-4bbe-8d86-05da654c0d1a#card` — Early Harvest: adjectives basic.
- `328a0691-9453-4f2d-a9c3-5c69da252d20#card` — Dragon Trainer: noun premodifiers Dragon, creature.
- `32e4de02-8437-4ffc-acc9-a3736652d282#card` — Chief of the Scale: noun premodifiers Warrior.
- `33598888-0367-4085-9415-f4e21da08354#card` — Armada Wurm: noun premodifiers Wurm, creature.
- `337287d6-cd22-4b70-8bbb-318e3cf91bee#card` — Play of the Game: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `33b6555e-035d-4241-a341-a9b9139ca6f0#card` — Thornplate Intimidator: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `33d42811-686d-4aca-bb3b-086de453ddaf#card` — Bartered Cow: noun premodifiers Food.
- `3407fe41-fdd3-4119-8f70-4bc4590a379f#card` — Negate: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `342d20ca-178d-433e-9630-6fed33a8d0bc#face:1` — Threat: noun premodifiers Beast, creature.
- `34c0f75d-10a9-428d-b202-3defb3949090#card` — Ant-Man's Army: noun premodifiers Food, Treasure.
- `35df2d01-2f9f-49a7-8fa1-cfaa52e5c7b6#card` — Survey the Wreckage: noun premodifiers Goblin, creature.
- `360f7bfd-3b4c-4e87-a251-fd55c8a62320#card` — Armory Paladin: noun premodifiers Equipment.
- `367282d6-956b-470d-a131-d22beff610b0#card` — Bramblecrush: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `368b4052-174e-4458-a6e6-eaf8093aa0fe#card` — Goblin Chieftain: noun premodifiers Goblin.
- `369c3b6c-6715-4b6a-89d2-e7542400ba8f#card` — Centaur Glade: noun premodifiers Centaur, creature.
- `370d67f6-8d43-46ca-ae6c-00799bb0bd0c#card` — Lucky Clover: noun premodifiers Adventure, sorcery.
- `372f2534-25dc-4ff3-9891-31ee42b33345#card` — Kobold Taskmaster: noun premodifiers Kobold.
- `376271a6-80f0-47a6-990a-5ec5517aae42#card` — Spectral Procession: noun premodifiers Spirit, creature.
- `37731c49-1238-4e0e-8993-204e36c1f4ee#card` — Melancholic Poet: noun premodifiers instant, sorcery; binary modifier coordination.
- `37733e29-3ade-4bfd-89c9-efd13b548a53#card` — Worthy Knight: noun premodifiers Knight, Human, creature.
- `3795fc31-2369-47d0-b9c9-120f44b8f114#card` — Elderleaf Mentor: noun premodifiers Elf, Warrior, creature.
- `38394ee3-6726-4f91-bc25-36bce0c6aab9#card` — Reiterate: noun premodifiers sorcery.
- `38503d34-f4f9-4eeb-9b1e-4fec1df921b3#face:1` — At the Door: noun premodifiers Dwarf, creature.
- `3891a914-e597-4126-846f-39bf8534eb92#card` — Museum Nightwatch: noun premodifiers Detective, creature.
- `38a71429-e673-4de4-80a8-f59e5cf98acc#card` — Emmara Tandris: noun premodifiers creature.
- `393ef069-ea6b-4969-b878-d942281cc918#card` — Exquisite Huntmaster: noun premodifiers Elf, Warrior, creature.
- `39419d29-47e7-4d71-953b-96a2e14ae964#card` — Songs of the Damned: noun premodifiers creature.
- `395aa62e-0786-4bd8-a88f-998e4700742e#card` — Giant Opportunity: noun premodifiers Giant, creature, Food.
- `3970e021-209d-41d3-9466-8b3ef9ab1a4c#card` — Horizon Seed: noun premodifiers Arcane.
- `3979067a-9c68-443d-a85f-d9f07be880b9#card` — Monastery Mentor: noun premodifiers noncreature, Monk, creature; negative owners lexeme:type/creature/non.
- `397b9dcf-690a-4a58-8637-bb9baab7cc2f#card` — Elder Gargaroth: noun premodifiers Beast, creature.
- `399fba23-fc13-440b-ab67-192eede6ecab#card` — Fire Nation Attacks: noun premodifiers Soldier, creature.
- `39a4416f-9b40-432e-aeb6-cc7bf519b37a#card` — Hunted Dragon: noun premodifiers Knight, creature.
- `3a59c882-8bb8-49ba-862f-125020dd5bec#card` — Empty the Warrens: noun premodifiers Goblin, creature.
- `3ab13412-1eaf-40c0-905d-8a3ca0c51be5#card` — Plundering Pirate: noun premodifiers Treasure.
- `3aef097b-711a-4463-a59c-831e4dd5e69a#card` — Full Moon's Rise: noun premodifiers Werewolf.
- `3b1f8108-6911-49e9-8f78-f950bb58cb6c#card` — Hornet Queen: noun premodifiers Insect, creature.
- `3b583358-d0d5-4f53-b06c-c6439156e684#card` — Release the Gremlins: noun premodifiers Gremlin, creature.
- `3b64051e-d320-4991-bc9e-6e1435a23c17#card` — Saproling Cluster: noun premodifiers Saproling, creature.
- `3b97794a-3aaa-45a6-9cad-f1b768615b6d#card` — Selhoff Entomber: noun premodifiers creature.
- `3b9b5b22-5a7d-4e37-a870-ca0f0efa4f36#card` — Sigil of the Empty Throne: noun premodifiers enchantment, Angel, creature.
- `3bfa51dd-daed-473a-8377-7bb34a2c3371#card` — Rubblebelt Runner: noun premodifiers creature.
- `3c0be888-0d66-4bad-84f4-90c3934915d7#card` — Falkenrath Celebrants: noun premodifiers Blood.
- `3c2aec69-ffd9-4a34-888c-58adbbb99bb5#card` — Wakening Sun's Avatar: noun premodifiers non-Dinosaur; negative owners lexeme:creature_subtype/dinosaur/non.
- `3c414ace-fd04-4fac-9379-24f5910f20e4#card` — Eater of the Dead: noun premodifiers creature.
- `3c43efd6-b1a8-452c-ae20-9a936c3340ab#card` — Treasure Vault: noun premodifiers Treasure.
- `3c49e98d-d33b-47fa-8b07-b38b367de2c7#card` — Angel of Sanctions: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `3c59ab03-8744-4148-be06-eb1b0147f2cc#card` — Eyes of the Wisent: noun premodifiers Elemental, creature.
- `3ca07da2-29a1-40ce-a7f3-4838e95ca1d9#face:0` — Heartflame Duelist: noun premodifiers instant, sorcery; binary modifier coordination.
- `3cc79d36-1a24-4395-8ad1-915a65db8a60#card` — Awaken the Woods: noun premodifiers Forest, Dryad, land, creature.
- `3cda958a-f777-433d-850e-a6c8f581db4e#card` — Flowering of the White Tree: negative owners vocab:PredicativeAdjective/Legendary/non; adjectives legendary, nonlegendary.
- `3ceae65e-938d-4e76-b3f6-7be70232549e#card` — Gloomwidow's Feast: noun premodifiers Spider, creature.
- `3d01c351-6dbd-4d63-9843-3aa1a3c039a6#card` — Namazu Trader: noun premodifiers Treasure.
- `3d18cd9b-6013-48d7-9269-4787e7585a51#card` — Person of Interest: noun premodifiers Detective, creature.
- `3d4a7dd3-9258-4bd1-adc4-08f0205e196a#card` — Dwynen's Elite: noun premodifiers Elf, Warrior, creature.
- `3dfb006f-6620-4602-ac3c-d40a5a15a981#card` — The Council of Four: noun premodifiers Knight, creature.
- `3e5d3574-cc92-434a-a210-fcda906391c8#card` — Dragon's Disciple: noun premodifiers Dragon.
- `3eaf1e6b-bf4c-4ea8-83ea-6827470fd802#card` — Hungry for More: noun premodifiers Vampire, creature.
- `3ef02f2b-b611-40c3-a1d5-8b34989643cf#face:0` — Pollen-Shield Hare: noun premodifiers creature.
- `3f086b72-6e7c-40d4-b995-d5f94c494462#card` — Goblin Furrier: adjectives snow.
- `3f3439e1-75ce-482d-881f-836492dca6e9#card` — Lys Alana Huntmaster: noun premodifiers Elf, Warrior, creature.
- `3f5acd90-199c-4ed3-9c8a-eeac4670d876#card` — Beast Attack: noun premodifiers Beast, creature.
- `3f6f4c98-ed7e-4fb1-8bfe-4210a39f77f2#card` — Unstoppable Plan: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `3fa67c92-6328-4d0a-95e3-656d447ab20e#card` — Zealots en-Dal: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `3fa71348-fa4d-4f39-a451-cf1570591991#card` — Imperious Perfect: noun premodifiers Elf, Warrior, creature.
- `3fc4bdf1-7da3-4f9f-806f-f82cd18ea65f#card` — Razorkin Hordecaller: noun premodifiers Gremlin, creature.
- `4045c24e-098f-4582-8c50-576a94c7b705#card` — Ambassador Oak: noun premodifiers Elf, Warrior, creature.
- `404e958b-ead9-4ec7-b786-7b3d65e29967#card` — Mouser Foundry: noun premodifiers Robot, artifact, creature.
- `4052f710-6ba7-406f-8fb7-653a684a80e0#card` — Progenitor Exarch: noun premodifiers Incubator.
- `412197f2-0e95-47fb-83db-ae8301c90ff0#card` — Crack Open: noun premodifiers Treasure.
- `413fb2db-f1a1-4d22-ac37-a52821d35ca2#card` — Dragonback Assault: noun premodifiers Dragon, creature.
- `419b5b4f-cac7-44bf-b1e2-655b5a5d37b1#card` — Brain Maggot: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `424683a5-d239-400d-af2b-9c00d1d32d7f#card` — Scatter Arc: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `424c6f5e-b386-47e9-b3fe-25b263097d40#card` — Sprout: noun premodifiers Saproling, creature.
- `426569c7-d868-4af1-bc83-5b5200532d17#card` — Searchlight Companion: noun premodifiers Spirit, creature.
- `429eff2b-635b-43f6-b20f-82e25992c0b9#card` — Sandwurm Convergence: noun premodifiers Wurm, creature.
- `42a1954f-774e-4d80-98b3-a736f23e2060#card` — Yavimaya Sapherd: noun premodifiers Saproling, creature.
- `42de2d46-c98d-4ae5-a53d-97531592529b#card` — Daysquad Marshal: noun premodifiers Human, Soldier, creature.
- `4310524d-dc0e-48ad-ae66-4c64aaa39340#card` — Enlightened Maniac: noun premodifiers Eldrazi, Horror, creature.
- `43144f06-079b-4515-a03a-01ea3e90d586#card` — Encroaching Wastes: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `43355ac4-bf8c-48f6-a322-fafbc9d132d1#card` — Talrand's Invocation: noun premodifiers Drake, creature.
- `4337f35b-baab-4640-9e1c-05f61c49bbdd#card` — Hidden Guerrillas: noun premodifiers artifact, Soldier.
- `4344abeb-47ab-4638-b50f-dd502f14c1af#card` — Skittering Horror: noun premodifiers creature.
- `434e82b5-abd4-41ac-a5b1-2dd01c667374#face:1` — Lord of Lineage: noun premodifiers Vampire, creature.
- `437be0ad-2dd1-42c8-bf62-5af95228b2f8#card` — Haunted Angel: noun premodifiers Angel, creature.
- `43975c07-103c-4664-b7e6-55bb29d85183#card` — Ostracize: noun premodifiers creature.
- `441cdd7d-6c4c-4a91-a118-7eaa35276660#card` — Priest of the Blood Rite: noun premodifiers Demon, creature.
- `442d06e6-8cf6-4980-b2bf-73cfb4d3a468#card` — Rootpath Purifier: noun premodifiers land; adjectives basic.
- `44623693-51d6-49ad-8cd7-140505caf02f#card` — Fury Sliver: noun premodifiers Sliver.
- `460c0f0b-bbf4-4307-9b44-350175d83bfe#card` — Cartographer's Companion: noun premodifiers Map.
- `46418fe4-065c-4dfa-b796-eee02c14f351#card` — Captain's Call: noun premodifiers Soldier, creature.
- `464968a8-865e-4ac8-927e-7c931e3dabb3#card` — Black Market Tycoon: noun premodifiers Treasure.
- `465843dc-57d0-46fd-ac47-238723034563#card` — Arenson's Aura: noun premodifiers enchantment.
- `46665089-aa3d-44c3-964d-6638dfbb5782#card` — Essence Scatter: noun premodifiers creature.
- `467d6acb-9a3c-433f-836d-54e0e670b7d0#card` — Head of the Homestead: noun premodifiers Rabbit, creature.
- `468a8c2e-0cc2-4108-809d-42ca1eb25ff2#card` — Ring of Immortals: noun premodifiers Aura.
- `469f25de-c180-47e9-9b1b-e23269139bbd#card` — Riveteers Requisitioner: noun premodifiers Treasure.
- `46a07fdd-0ca3-4654-9caa-a88fae80c414#card` — Myr Matrix: noun premodifiers Myr, artifact, creature.
- `46c3a69a-4984-40b0-905f-4b0762c7ad72#card` — Titania, Nature's Force: noun premodifiers Elemental, creature.
- `46ddc2e1-386f-4aaf-b37b-37b1708e0d9c#card` — Shadow Sliver: noun premodifiers Sliver.
- `470f0bbf-a0c9-4216-96f0-d400dac17aef#card` — Jewel Thief: noun premodifiers Treasure.
- `47657df2-0e58-46c8-87e2-cc752708a612#card` — Sliver Legion: noun premodifiers Sliver.
- `47678f44-b595-492e-af00-4e73f941e24e#card` — Penumbra Spider: noun premodifiers Spider, creature.
- `47f221a4-b51a-46b9-a4fc-46bc8ec7ddd3#card` — Experimental Aviator: noun premodifiers Thopter, artifact, creature.
- `47f8df80-abc9-4d46-8078-8fbc23430259#card` — Patron of the Arts: noun premodifiers Treasure.
- `48054407-b5e1-431c-ad03-c12a31d0d9a8#card` — Sedris, the Traitor King: noun premodifiers creature.
- `48090c64-f41a-447b-ad7a-54e3194f759b#card` — Triplicate Spirits: noun premodifiers Spirit, creature.
- `480ea052-87d7-4fea-ac97-b6121b78f863#face:0` — Invasion of New Phyrexia: noun premodifiers Knight, creature.
- `482f5fc8-ca3b-4280-8cf7-a1c55f8b9f12#card` — Sarkhan's Whelp: noun premodifiers Sarkhan.
- `484eeb13-fe74-45fe-b089-7beec87cfcdb#card` — Wrench Mind: noun premodifiers artifact.
- `48a99dce-0aa9-4aac-81df-cec5f94c639d#card` — Archon of Sun's Grace: noun premodifiers Pegasus, creature.
- `49039519-62db-4605-b600-cef6f34c893f#card` — Rampage of the Valkyries: noun premodifiers Angel, creature.
- `493ecd11-e980-42da-a79e-cddd9a94cc7a#card` — Temple Thief: noun premodifiers enchantment.
- `494050f4-0a55-415d-9ae9-feb17e61c4e1#card` — Savage Conception: noun premodifiers Beast, creature.
- `4965f5fe-a5fe-4976-822c-5cf40fea8c34#card` — Knight Watch: noun premodifiers Knight, creature.
- `4987c458-604a-4727-b360-170616e91e67#card` — Kambal, Consul of Allocation: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `499f1e1e-d96e-481f-a5d9-eb38da927cd7#card` — Primordial Sage: noun premodifiers creature.
- `49f1dd11-aa10-442d-aaca-dd94d1ca108a#card` — Coalborn Entity: noun premodifiers creature.
- `49fe9f5a-5821-4586-b913-7d8aef1f8669#card` — Cenn's Enlistment: noun premodifiers Kithkin, Soldier, creature.
- `4a4a061d-bc4e-49da-8d20-90fcb571b025#card` — Imperial Oath: noun premodifiers Samurai, creature.
- `4a4f6e85-df16-49a4-8fc6-149a28c2cb42#card` — Vanguard of Brimaz: noun premodifiers Cat, Soldier, creature.
- `4a7a5ea9-b633-459f-bcd4-40de53afce0e#card` — Swarm Intelligence: noun premodifiers sorcery.
- `4ac53b34-20d2-4a53-bade-4f998fb6ae67#card` — Thopter Engineer: noun premodifiers Thopter, artifact, creature.
- `4ad9c550-5605-4ffe-8e27-e9f2d9ada000#card` — Stormplain Detainment: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `4adf959b-2e09-47ff-a1b1-b889348735b8#card` — Battering Sliver: noun premodifiers Sliver.
- `4ae2750e-3650-42df-a539-ea4cb72fa136#face:0` — Start: noun premodifiers Warrior, creature.
- `4b457bc2-2d8c-42cb-9a63-bb4c459d14e3#card` — Atomize: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `4b5f30a6-b0fb-4dcb-85d5-1db1bba73737#card` — Valorous Steed: noun premodifiers Knight, creature.
- `4b74d2d9-d75b-496e-bbb4-2b95b3d3ba55#card` — Deadly Derision: noun premodifiers Treasure.
- `4b8d82fe-571d-4fd6-8d0c-77b7e5ef3e23#card` — Deeproot Historian: noun premodifiers Druid.
- `4bae4e34-fcf4-4aee-8da2-5b3ee41a595a#card` — Thought Erasure: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `4bc91d7d-37aa-475c-8fe9-763975d51add#card` — Goblin Marshal: noun premodifiers Goblin, creature.
- `4bdfd718-f474-4223-88d8-7fa9fb0c86b4#card` — Prowling Serpopard: noun premodifiers creature.
- `4c13e2b5-961a-4031-84b1-15bd19b94286#card` — Afterlife: noun premodifiers Spirit, creature.
- `4cb3a303-9ded-46d5-90e7-1cc1c65a75da#card` — Tura Kennerüd, Skyknight: noun premodifiers instant, sorcery, Soldier, creature; binary modifier coordination.
- `4daa7b7d-e410-4bcf-b3e2-2751774bef47#card` — Prava of the Steel Legion: noun premodifiers creature, Soldier.
- `4e0b9273-6614-41a8-9738-959349c0717b#card` — Anointer Priest: noun premodifiers creature.
- `4e0de4d9-7a30-4a2b-94ba-8239baad2a0a#card` — Tsabo's Decree: noun premodifiers creature.
- `4e93b23c-a7f7-4bdd-bbca-0dc48bb5c223#card` — Sling-Gang Lieutenant: noun premodifiers Goblin, creature.
- `4ec9d9eb-14e2-47ea-88a2-d2394e363784#card` — Spyglass Siren: noun premodifiers Map.
- `4ef4b4f9-47cd-4862-8cbb-b6e624d6e6fc#face:0` — Promise of Aclazotz: noun premodifiers non-Demon; negative owners lexeme:creature_subtype/demon/non.
- `4ef4b4f9-47cd-4862-8cbb-b6e624d6e6fc#face:1` — Foul Rebirth: noun premodifiers non-Demon, Vampire, Demon, creature; negative owners lexeme:creature_subtype/demon/non.
- `4f4a3c9e-80c4-4fa1-940d-2c57065901a3#card` — Enchanted Carriage: noun premodifiers Mouse, creature.
- `4f5ebbb8-f49b-488f-bb85-888aaa918bdd#face:1` — Tempt with Treats: noun premodifiers Food.
- `4f891c68-c959-4210-94e5-94a8e487d5ef#card` — False Summoning: noun premodifiers creature.
- `4fad3816-41af-4fa4-85b4-ba37614ec848#face:1` — Bakery Raid: noun premodifiers Food.
- `4fdfa41a-75a5-4b36-8c9e-083e26d137e2#card` — Chief of the Foundry: noun premodifiers artifact.
- `5097f4e6-50af-4641-909f-db44abf0ce32#card` — Rite of the Dragoncaller: noun premodifiers instant, sorcery, Dragon, creature; binary modifier coordination.
- `50bbbb39-4709-478f-bb51-716c7e4f8323#card` — Stronghold Machinist: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `50bee80e-89df-48c2-b608-ba66ff3e794a#face:1` — Train Troops: noun premodifiers Knight, creature.
- `50de88d4-0beb-45e5-9664-d4f0e280e652#card` — Ant Queen: noun premodifiers Insect, creature.
- `51a6819d-c8fb-4f52-ab0c-4ae845728ed2#card` — Redrock Sentinel: noun premodifiers Treasure.
- `51abc95c-bd92-46f7-a0a1-6e9f8e0dda87#card` — Illness in the Ranks: noun premodifiers creature.
- `51bb6d3b-be6a-435d-8a80-0c0f0161892d#card` — Storyteller Pixie: noun premodifiers Adventure.
- `520abf49-7631-4e7f-889e-547ccc8c3b95#card` — Chandler: noun premodifiers artifact.
- `52241b9c-7a69-4176-9234-8bdab09d8e64#card` — Sai, Master Thopterist: noun premodifiers artifact, Thopter, creature.
- `522dd417-364b-44ab-8ca9-fb55db5f26a6#card` — Get Lost: noun premodifiers Map.
- `528fe479-dab9-4cf8-bda0-1d226a644366#card` — Kav Landseeker: noun premodifiers Lander.
- `529f259a-df66-4283-bc28-de3933c42e67#card` — Chancellor of Tales: noun premodifiers Adventure.
- `52a0dae4-2a95-487e-acd4-eabdb2d031e2#card` — Crux of Fate: noun premodifiers Dragon, non-Dragon; negative owners lexeme:creature_subtype/dragon/non.
- `52c73c88-3b87-4184-b8e1-cd2b5688800d#card` — Rank Officer: noun premodifiers Zombie, creature.
- `52e8ec03-23ed-43a0-bce2-ad0cd5c4d737#card` — Megantic Sliver: noun premodifiers Sliver.
- `52fa7e9b-36d9-409c-8981-6f8dfd8bb031#card` — Stir the Sands: noun premodifiers Zombie, creature.
- `53015c41-af2d-43e2-a690-d8877537b8dd#face:1` — Psionic Pulse: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `5324192b-6687-41e4-8e56-326b21a5dbf3#card` — Moorland Haunt: noun premodifiers creature, Spirit.
- `5396045d-744b-435e-8324-a1bfbb03449d#card` — Clachan Festival: noun premodifiers Kithkin, creature.
- `54866c25-b332-46c8-a2c5-1a71b702ddc0#card` — Mindsparker: noun premodifiers sorcery.
- `55e4d4b2-1bd8-4c29-a21a-c014a974b715#card` — The First Sliver: noun premodifiers Sliver.
- `55eca80c-dcd8-4c2f-aa0f-fb0aec7b80f7#card` — Ophiomancer: noun premodifiers Snake, creature.
- `562684ca-6e3c-47df-9e94-8ff772c25a96#card` — Renowned Weaver: noun premodifiers Spider, enchantment, creature.
- `564dd5e7-6c1e-4959-a594-60e08552aec9#card` — Urbis Protector: noun premodifiers Angel, creature.
- `568cf486-0261-4634-ac36-a6507101b2d0#card` — Vedalken Archmage: noun premodifiers artifact.
- `56b4a2f7-2d35-4d05-8d26-4f9f16c5f9f5#card` — Lunar Insight: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `56c78349-e4fa-48ba-ae8e-d2312ae6edc6#card` — Grisly Ritual: noun premodifiers Blood.
- `56e05b17-aa1d-483d-857c-4cab4f12aa8a#card` — Triplicate Titan: noun premodifiers Golem, artifact, creature.
- `573151f0-00d4-4a8a-8a09-745c5f376532#face:0` — Hydroelectric Specimen: noun premodifiers sorcery.
- `57ad3c3a-6ac7-4a35-bc07-00f6ea0988c5#face:0` — Alive: noun premodifiers Centaur, creature.
- `5839bc83-868e-402a-9233-ad3d2871ac6d#card` — Kobold Overlord: noun premodifiers Kobold.
- `597afe7c-a4ba-48d0-9c37-d9affe5ba195#card` — Deadeye Plunderers: noun premodifiers Treasure.
- `599c3fd6-3309-4b5d-adec-9c4062848ad5#card` — Stromkirk Captain: noun premodifiers Vampire.
- `59dbc873-72d9-4e7b-9442-2c7b8e8523e1#card` — Ruination: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `59f10ddb-8287-4d2c-8c60-1080b108fa78#card` — Requiem Angel: noun premodifiers non-Spirit, Spirit, creature; negative owners lexeme:creature_subtype/spirit/non.
- `5a3f9c4d-20a8-46aa-9cf3-3916f56a7906#card` — Blood Servitor: noun premodifiers Blood.
- `5a43b164-61a4-4dcb-9397-45356ad4260e#card` — Zuberi, Golden Feather: noun premodifiers Griffin.
- `5a620d20-f14e-43d0-8e57-c2a197e2ec51#card` — Urza's Factory: noun premodifiers Assembly-Worker, artifact, creature.
- `5a66802a-76f9-4a31-a776-24110dcfca64#card` — Hero of Precinct One: noun premodifiers Human, creature.
- `5a898412-5b1e-4217-ac78-5b4099ed8e16#card` — Angelic Ascension: noun premodifiers Angel, creature.
- `5af48f87-7b94-44de-90e3-91f10ced00d3#card` — Efficient Construction: noun premodifiers artifact, Thopter, creature.
- `5b2364d7-a811-4595-a1b4-224c70555ffa#card` — Raise the Alarm: noun premodifiers Soldier, creature.
- `5b31ac7b-7901-484c-b59f-556284fd562c#card` — Lifebane Zombie: noun premodifiers creature.
- `5b3687e5-546f-4a70-aa5c-b7db3300a3f4#card` — Distract the Guards: noun premodifiers Human, Rogue, creature.
- `5b8472c8-a7e7-46f2-aecf-e954082a5ef5#face:1` — Call: noun premodifiers Bird, creature.
- `5bf9f397-0216-4ec9-a57b-406758dcc233#card` — Undead Servant: noun premodifiers Zombie, creature.
- `5cf18398-9552-415e-a61f-2c149e35ec3d#card` — Zuko's Exile: noun premodifiers Clue.
- `5d27c63e-d1ef-48af-b51d-01ebc6daeac9#card` — Mikaeus, the Unhallowed: noun premodifiers non-Human; negative owners lexeme:creature_subtype/human/non.
- `5d46b8dd-2a4a-4d9f-b4c6-2b45f05081c2#card` — Revel of the Fallen God: noun premodifiers Satyr, creature.
- `5d4ed768-0a37-4604-8121-43920ed9fb4a#card` — Launch Mishap: noun premodifiers creature, planeswalker, Thopter, artifact; binary modifier coordination.
- `5d950fd4-8956-4b1a-ab0f-ab31bcffce4e#card` — Callous Inspector: noun premodifiers Clue.
- `5d96840e-f9b3-4c86-8f59-f12e5736318f#card` — Voracious Greatshark: noun premodifiers creature.
- `5da7eea8-bb9e-47ce-a554-8a1ee058bd7a#card` — Beast Whisperer: noun premodifiers creature.
- `5df44b8d-b337-49d8-8427-34253f52cb47#face:0` — Decadent Dragon: noun premodifiers Treasure.
- `5e0bd136-e7e9-49f0-910b-a9fb0b9c481a#card` — Lotleth Giant: noun premodifiers creature.
- `5e4c02c2-2e54-4e8d-8d94-6b2454027e7e#card` — Queen's Commission: noun premodifiers Vampire, creature.
- `5ea295af-e117-49cc-836b-da16b5d5bbdf#card` — Nullify: noun premodifiers Aura.
- `5effb651-f9e0-4c14-8192-bd0e132b8d5d#card` — Bitterbloom Bearer: noun premodifiers Faerie, creature.
- `5f2a3797-28aa-4c7a-ba2b-fd243a1747fd#card` — Lifecrafter's Bestiary: noun premodifiers creature.
- `5f3f68b5-8c6a-4181-bb8a-9c73967d198a#card` — Glen Elendra Archmage: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `5f532b7f-cb76-4be7-b361-0106c561dcb2#card` — Sami's Curiosity: noun premodifiers Lander.
- `5f808e3b-6b69-4b46-8a92-36d960527acc#card` — Nissa's Defeat: noun premodifiers Nissa.
- `5fac139a-07d3-4e6c-98e3-d98b199f7a6f#card` — Young Pyromancer: noun premodifiers instant, sorcery, Elemental, creature; binary modifier coordination.
- `5fd4be4c-70a3-4c91-a4c6-788a2051ff13#card` — Viscerid Drone: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non; adjectives snow.
- `5fe3aabd-3271-41bf-ae50-f6efaba7c4ae#card` — Carrot Cake: noun premodifiers Rabbit, creature.
- `602132c2-8ee8-41f8-bfac-cb17d32203f5#card` — Savvy Hunter: noun premodifiers Food.
- `6022608a-6cf2-45bd-adec-63211710a5ed#card` — White Auracite: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `60662463-6050-46c6-84ab-64e203b67b4d#card` — Jungleborn Pioneer: noun premodifiers Merfolk, creature.
- `60e36f86-e284-468e-8073-07f19a3ed65e#card` — Introduction to Annihilation: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `61105cb5-d7a1-4021-a006-dd1b947dfa68#card` — Aether Sting: noun premodifiers creature.
- `61529c3a-9b7e-4e73-9f6e-58c992ef1ffc#card` — Dreamcatcher: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `6158ab85-a929-472c-9618-8e76d2d4b228#card` — Promise of Bunrei: noun premodifiers Spirit, creature.
- `61ad6bd2-c079-44e7-a1d9-8089f086ec69#card` — Pack Guardian: noun premodifiers land, Wolf, creature.
- `61e2d557-f08d-489b-94c8-aff87e5189f9#card` — Take Up Arms: noun premodifiers Warrior, creature.
- `61f18560-8f26-4d7b-a6f8-ec388766123b#card` — Syphon Sliver: noun premodifiers Sliver.
- `62421084-96e6-40dd-8d34-7c8c1091d4a4#card` — Muse's Encouragement: noun premodifiers Elemental, creature.
- `628e5c1d-c7f9-4a98-98fb-13d7d5f81f64#card` — Spectral Reserves: noun premodifiers Spirit, creature.
- `62c240ac-f2dd-4e39-9e82-50307efa833e#card` — Archfiend's Vessel: noun premodifiers Demon, creature.
- `635b854d-de71-4c85-9755-8c97bf3cba36#card` — Sweettooth Witch: noun premodifiers Food.
- `636a9364-54a6-4f67-ad96-731646e8bde7#card` — Chief of the Edge: noun premodifiers Warrior.
- `636ed3e3-c31d-403a-a3e7-03e01350ce4f#card` — Mardu Woe-Reaper: noun premodifiers creature.
- `63b0538e-aa94-4e23-adf1-704cbf5bc3e5#card` — Brass's Bounty: noun premodifiers Treasure.
- `63e17cbd-6dae-42d2-a074-7924db1477bf#card` — Jarvis, Earth's Mightiest Butler: noun premodifiers Hero.
- `63e79a6f-ed98-480a-a740-d44e111046f7#card` — Guarded Heir: noun premodifiers Knight, creature.
- `64553181-9852-459f-a6e0-54ce188dd937#card` — Horned Sliver: noun premodifiers Sliver.
- `6467cbb7-1e4e-482d-a20f-6cb9fc0f1ad1#card` — Whirlwind of Thought: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `6494e3c4-5489-44a5-94b6-4ee2d7830896#card` — Boneclad Necromancer: noun premodifiers creature, Zombie.
- `64ed5c35-8185-4e68-8028-b7b632ef5828#card` — Skullport Merchant: noun premodifiers Treasure.
- `64f3af46-f8ab-4ba4-9dff-7412cefb4eea#card` — Mausoleum Guard: noun premodifiers Spirit, creature.
- `650890bc-aba6-45fb-8096-3a36b0a015b3#card` — Golem's Heart: noun premodifiers artifact.
- `653de668-beb0-4be7-bb05-30de5f52a3f1#card` — Conclave Cavalier: noun premodifiers Elf, Knight, creature.
- `65428247-b386-410c-91b7-81dd278c0927#card` — Grim Flowering: noun premodifiers creature.
- `661736eb-97d7-4b3c-ab18-708479a5fb29#card` — Reduce to Memory: noun premodifiers nonland, Spirit, creature; negative owners lexeme:type/land/non.
- `66465bc1-4a34-4951-be53-57f90002976a#card` — Gleeful Demolition: noun premodifiers Phyrexian, Goblin, creature.
- `668312dd-7b6e-46bf-9c6a-bfb03dd75b8f#card` — Icatian Crier: noun premodifiers Citizen, creature.
- `6698b059-f1ed-4023-9d44-fbdf39fb159a#card` — Grave Robbers: noun premodifiers artifact.
- `66deb3f3-0314-427e-9cb8-6befa365ae94#card` — Silverquill, the Disputant: noun premodifiers instant, sorcery; binary modifier coordination.
- `67158085-4807-4f0e-8599-c55ec7ec252e#card` — Skyfisher Spider: noun premodifiers nonland, creature; negative owners lexeme:type/land/non.
- `67247945-17b7-4cb8-ba65-d0dfec367b77#card` — Thraben Heretic: noun premodifiers creature.
- `674202f0-6832-4392-a118-d513162bcd03#card` — Sprouting Renewal: noun premodifiers Elf, Knight, creature.
- `679d5a1d-8da8-4142-924b-97b359f55309#card` — Thraben Standard Bearer: noun premodifiers Human, Soldier, creature.
- `67ceffa4-2fdb-499c-88cd-49fb5eb9be59#card` — Bant Sojourners: noun premodifiers Soldier, creature.
- `6800d01c-345d-4932-a2a4-8df0fb1902f2#card` — Magus of the Moon: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `6845a62f-e0f1-448c-beeb-abac2dabd975#card` — Vile Rebirth: noun premodifiers creature, Zombie.
- `688a559d-249f-4d03-98e6-581843e7ddc3#card` — Riders of Rohan: noun premodifiers Human, Knight, creature.
- `68c90a32-d6ee-4595-8517-5f4cb0e4062d#card` — Vault Robber: noun premodifiers creature, Treasure.
- `68cefb44-afad-4638-8fdc-65a099b51d55#card` — Jenson Carthalion, Druid Exile: noun premodifiers Angel, creature.
- `68f1db12-82fb-4bf1-908d-db38fd67efe5#card` — Sire of the Storm: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `690df8d2-9538-4c51-b730-cee09ad0ef1c#card` — Sphinx of the Chimes: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `69501650-ed48-4ebf-9287-b0338c5bc5d5#card` — Trumpeting Herd: noun premodifiers Elephant, creature.
- `695eea46-1535-48c5-bbb6-0b8379e77bfc#card` — Arasta of the Endless Web: noun premodifiers instant, sorcery, Spider, creature; binary modifier coordination.
- `696f554d-0485-48a5-9273-3f6fb7d16a5d#card` — Witherbloom Apprentice: noun premodifiers instant, sorcery; binary modifier coordination.
- `6a15599d-8b00-4e0f-ac0c-d05f07c1e50d#card` — Striking Sliver: noun premodifiers Sliver.
- `6aad6906-96ec-4994-bfc3-9a76f893bc8c#card` — Armored Kincaller: noun premodifiers Dinosaur.
- `6b087ae1-1412-4dd9-9b96-3b45be52d386#card` — Sourbread Auntie: noun premodifiers Goblin, creature.
- `6b589e70-9c1d-48dd-a597-479959e692bf#card` — Predatory Sliver: noun premodifiers Sliver.
- `6b68acc2-b9d5-495b-8054-c04bae1349f1#card` — Sokka, Tenacious Tactician: noun premodifiers noncreature, Ally, creature; negative owners lexeme:type/creature/non.
- `6b7e6ae4-2ee1-44bf-ac93-79fc87494515#card` — Merrow Reejerey: noun premodifiers Merfolk.
- `6b8119e9-a9a9-4349-be32-adcb84b8eb79#card` — Mobilization: noun premodifiers Soldier, creature.
- `6bdc4996-f7d9-4dc4-b70a-aa63f6816b1d#card` — Glorious Protector: noun premodifiers non-Angel; negative owners lexeme:creature_subtype/angel/non.
- `6bef2904-1ebe-4e7e-879a-79f4968174fb#card` — Sin Collector: noun premodifiers sorcery.
- `6c12cd05-b673-453c-9f6b-f3aa022467c1#card` — Crush of Wurms: noun premodifiers Wurm, creature.
- `6c5c7b26-9cd6-4f29-bbb2-a0745153ee5d#card` — Teferi's Care: noun premodifiers enchantment.
- `6d720de8-b7b7-478f-ab1a-3487ce76aa00#card` — Night Terrors: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `6d7be242-a072-40ce-b540-95880506cccd#card` — Dispel: noun premodifiers instant.
- `6d92f14b-470b-40bc-b294-f725d46f307e#card` — Rakka Mar: noun premodifiers Elemental, creature.
- `6daf3c59-7636-4d44-872b-67722b4868cb#card` — Jade Mage: noun premodifiers Saproling, creature.
- `6dc98143-7c4c-4b75-9bbb-5226d800b1d6#card` — Dragon Roost: noun premodifiers Dragon, creature.
- `6dd1f375-e8f2-4725-8fbd-9750c4860820#card` — Heart Sliver: noun premodifiers Sliver.
- `6de8bdc0-9b2a-4105-bbac-d5a2f73f52dc#card` — Pirate's Prize: noun premodifiers Treasure.
- `6df03db9-e4b3-4931-94e3-c582d2b04110#face:1` — Gentleman's Rise: noun premodifiers Zombie, creature.
- `6df620b8-1e54-4f63-8556-12c75e5679af#card` — Countersquall: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `6e2c2423-d854-4478-99e6-64f29851f026#card` — Elvish Archdruid: noun premodifiers Elf.
- `6e48981b-8971-4b7c-ae05-29c7e4091a05#card` — Straw Golem: noun premodifiers creature.
- `6eb93545-a1a6-4447-82c5-421d8e9f023d#card` — Erebor Flamesmith: noun premodifiers instant, sorcery; binary modifier coordination.
- `6ec2a242-9068-4ee2-8ac8-8341cc570f56#face:0` — Emeria's Call: noun premodifiers Angel, Warrior, creature, non-Angel; negative owners lexeme:creature_subtype/angel/non.
- `6f2e4607-bece-42ba-a2a3-5dc50f246793#card` — Mammoth Bellow: noun premodifiers Elephant, creature.
- `6f885041-3e57-4a69-84f2-fd207ff9f31b#card` — Null Brooch: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `6ff4ff67-ad08-447f-a112-1a071c1474a4#card` — Hanged Executioner: noun premodifiers Spirit, creature.
- `70034860-5198-421f-871d-7c1676337b6e#card` — Reaper King: noun premodifiers Scarecrow.
- `7054c42b-48dd-4887-b878-69c005259d27#card` — Thistledown Players: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `70616a8c-9b6c-408e-8f3e-2c348ec136d8#card` — Memory Trap: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `70c32155-dc07-45cb-bceb-e19d3ebc915d#card` — Hidden Stag: noun premodifiers Elk, Beast.
- `70de24d9-c585-4bf6-ac2c-c5b4b7aa298c#card` — Diregraf Captain: noun premodifiers Zombie.
- `70e58068-90d2-4720-a11c-5d243046b4a3#card` — Thirst for Meaning: noun premodifiers enchantment.
- `70ecc25b-2004-4a3c-80c3-6d97bf0711eb#card` — Symbiotic Beast: noun premodifiers Insect, creature.
- `70f26726-0373-4e50-a1de-37392ca94890#card` — Ox Drover: noun premodifiers Ox, creature.
- `70fa209f-5e79-44de-81ce-1d8d0d8c1006#card` — Cemetery Reaper: noun premodifiers Zombie, creature.
- `710dc9b1-7d32-4330-95a8-3e91a668202a#card` — Ghirapur Gearcrafter: noun premodifiers Thopter, artifact, creature.
- `713f16db-95ec-479e-a48c-7a69f7668d7f#card` — Void Rend: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `71a80491-eea2-4661-9446-26a87efbf4a8#card` — Kykar, Wind's Fury: noun premodifiers noncreature, Spirit, creature; negative owners lexeme:type/creature/non.
- `71b178bd-3b37-4de9-97a3-96e23d5444a4#card` — Hornswoggle: noun premodifiers creature, Treasure.
- `71ba6f1b-2fa6-4d4c-8778-f0765b1a5d8e#card` — Maverick Thopterist: noun premodifiers Thopter, artifact, creature.
- `72a39730-acc3-470c-960e-3a91c5eae4be#card` — Wurmcoil Larva: noun premodifiers Phyrexian, Wurm, artifact, creature.
- `72aeed80-653b-4c53-ba65-a339aad14b9a#card` — Hivestone: noun premodifiers creature.
- `72c4c682-df97-4408-997b-e849f54aba39#card` — Seal from Existence: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `73094bde-5466-41a4-88f9-a98e67f5ed2e#card` — Break Down: noun premodifiers Junk.
- `73109353-51fa-43a1-93e0-b74536f372a5#card` — Sculptor of Winter: adjectives snow.
- `73733fa5-d292-40ec-8f17-05ba5feb40ab#card` — Reckless Lackey: noun premodifiers Treasure.
- `7392c58b-1ba9-4cf3-8449-aa5de3c42130#card` — Fiddlehead Kami: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `7402d98f-6d58-4ff2-a0db-9eec4417a75f#card` — Bovine Intervention: noun premodifiers Ox, creature.
- `74910702-e385-48f7-a6b3-064ebed08be7#card` — Talon Sliver: noun premodifiers Sliver.
- `7493d78c-26c4-402e-bec5-6449773d0344#face:1` — Ashmouth Dragon: noun premodifiers instant, sorcery; binary modifier coordination.
- `749d2994-44e7-40d3-8630-7bebed239e9e#card` — Haywire Mite: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `753cb2b2-24ce-484f-a2d0-be6fd2c67ebd#card` — Grafdigger's Cage: noun premodifiers creature.
- `758e68fd-76ba-467e-a756-c5daa7db16d5#card` — Exhibition Magician: noun premodifiers Citizen, creature, Treasure.
- `75b2dd64-a2f5-4032-a19f-5b485dc44b16#card` — Flash Counter: noun premodifiers instant.
- `75b39a67-6720-482f-935a-c142ca0f7891#card` — Seed the Land: noun premodifiers Snake, creature.
- `767c896d-4ff6-4cd4-b3a4-80179e84e1cd#card` — Virulent Sliver: noun premodifiers Sliver.
- `76a6ddc2-f894-4331-89a9-87ca16efd0c0#card` — Servo Exhibition: noun premodifiers Servo, artifact, creature.
- `7735eeba-693b-47e2-bd51-414379cf1016#card` — Beast Within: noun premodifiers Beast, creature.
- `78b2b68d-6e27-49f3-8373-6e19faf6f679#card` — Inspiring Statuary: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `78dbcc18-291c-4b2c-9576-f8dc93914936#card` — Soul Shepherd: noun premodifiers creature.
- `78de77ef-c325-4342-869f-042921229577#card` — Biotech Specialist: noun premodifiers Lander.
- `78e65286-204d-43ab-8929-b8e2191b1fce#card` — Windrider Wizard: noun premodifiers Wizard.
- `794aaf8a-6d8e-41fb-acbe-a2b73d65d9d9#card` — Martyr of Dusk: noun premodifiers Vampire, creature.
- `795b096a-2bce-4588-a2c9-abc5ea40dc0c#card` — Enchantress's Presence: noun premodifiers enchantment.
- `79638767-fbc7-451a-b29f-d93f2ac6f102#card` — Kher Keep: noun premodifiers Kobold, creature.
- `79a9fc1c-a6a9-483a-9ba7-d09fe41760c3#card` — Featherbrained Filcher: noun premodifiers Food.
- `79b56cf8-d573-40cf-b797-6b4eef2bed23#card` — Sigarda, Heron's Grace: noun premodifiers Human, Soldier, creature.
- `79b69b38-c01d-499a-a865-8c1cecf9fae0#face:1` — Retrieve Prey: noun premodifiers creature.
- `79bc9b71-5b04-46b8-a9de-9e9f17936f29#card` — Pheres-Band Warchief: noun premodifiers Centaur.
- `79d99305-b3dd-4687-af69-bf4b28704aae#card` — Goblin Gang Leader: noun premodifiers Goblin, creature.
- `7a0fe73f-6e08-4c9a-80e1-c2ead6cebe06#card` — Surrak, Elusive Hunter: noun premodifiers creature.
- `7a21ea22-3cd7-4c11-8895-5943c0d93a0d#card` — Verdant Force: noun premodifiers Saproling, creature.
- `7aae54a6-3011-4867-b5ee-21a2111aecc6#card` — Tunnel Surveyor: noun premodifiers Glimmer, enchantment, creature.
- `7aba1039-f108-423e-8aa4-d9d8b4da2414#card` — Mold Shambler: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `7acd7ef5-0e26-436d-962a-1a391f81c2e8#card` — Perilous Vault: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `7af53dd7-3994-48f1-9ae1-44380ac56ee0#card` — Karplusan Hound: noun premodifiers Chandra.
- `7b175a33-a05c-49fa-802a-36f7013c892a#card` — Nullstone Gargoyle: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `7b3c0d89-55ed-43d7-a00a-7b060857ec4e#face:0` — Invasion of Segovia: noun premodifiers Kraken, creature.
- `7b414f70-a9b1-42c3-9655-6aa520d3b19b#card` — Fallow Wurm: noun premodifiers land.
- `7b8f4879-578e-40e4-863a-41d0c32c6bdd#card` — Mogg War Marshal: noun premodifiers Goblin, creature.
- `7b92ac54-3883-48cf-b01e-be9797dc1649#card` — Gnaw to the Bone: noun premodifiers creature.
- `7bb1b48b-aa6b-4064-9355-c43b0f509388#card` — Distress: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `7c07970d-063a-44a1-9761-b80aafa90dac#card` — Stark Industries Executive: noun premodifiers Treasure.
- `7c3171da-7a95-4f8e-ba30-c31af21c1828#card` — Goblin Flectomancer: noun premodifiers sorcery.
- `7c72df45-c3bd-4fb9-8365-5a5639566742#card` — Molten Frame: noun premodifiers artifact.
- `7c766365-82c2-46d6-8521-42e73129f4ef#card` — Surrak Dragonclaw: noun premodifiers creature.
- `7c8b38b3-b808-4ad8-b4e5-39e216063ff6#card` — Liliana's Defeat: noun premodifiers Liliana.
- `7ca3aa03-e62d-464a-9111-e754abe17f76#card` — Strix Serenade: noun premodifiers artifact, creature, planeswalker, Bird; Oxford modifier series.
- `7d48ad95-d428-4e2d-b697-56907c3ce579#card` — Dire Fleet Hoarder: noun premodifiers Treasure.
- `7d54a535-e924-47c7-81db-e04df1c3f44e#card` — Belligerent Sliver: noun premodifiers Sliver.
- `7d9fc9e7-d80b-49c3-871c-ed25b3059ae8#card` — Impulsive Pilferer: noun premodifiers Treasure.
- `7db22241-fe43-4071-9af9-de1cf394f7f5#face:1` — Cooperate: noun premodifiers instant, sorcery; binary modifier coordination.
- `7df3e379-c217-416e-a1c8-46338608c49e#card` — Elvish Promenade: noun premodifiers Elf, Warrior, creature.
- `7e00b0cd-d212-4604-ba07-da21f4fe00b0#card` — Sram, Senior Edificer: noun premodifiers Aura, Equipment, Vehicle; Oxford modifier series.
- `7e23a2e7-b8b3-424f-8922-a1adb1db3f4d#card` — Luminescent Rain: noun premodifiers creature.
- `7e481e22-4122-4259-8978-4727eb307195#card` — Bearscape: noun premodifiers Bear, creature.
- `7e6c7b57-bd6a-4ac8-bcab-c3a879f479c2#card` — Chatterstorm: noun premodifiers Squirrel, creature.
- `7e84aff5-2cb6-4214-befd-3d2de31229e5#face:1` — Heart's Desire: noun premodifiers Human, creature.
- `7ed8a69b-fba0-4d4d-98b2-311a92373669#card` — Skeletal Vampire: noun premodifiers Bat, creature.
- `7f8dad7f-a573-469f-9095-4ba3886b4f0b#face:1` — The Broken Sky: noun premodifiers creature, Spirit.
- `7fbf6afc-8a16-447b-bdef-b7d0b640f76c#card` — Wing Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `8039df5c-b26b-4277-8226-4c08a5a69fbb#card` — Gelectrode: noun premodifiers instant, sorcery; binary modifier coordination.
- `8060dc97-9868-4846-85f4-0ef70da2b021#card` — Grim Bounty: noun premodifiers Treasure.
- `8088d110-2314-45c3-b6ee-3f55d1f388e4#card` — Insidious Will: noun premodifiers sorcery.
- `80e4e857-3f7e-4344-bf41-64d3b32f328f#card` — Reflex Sliver: noun premodifiers Sliver.
- `80e9a42e-f4f4-4bfc-91a2-ad124737dca2#card` — Deep Forest Hermit: noun premodifiers Squirrel, creature.
- `814e321d-16fd-4f7b-a8d8-2b089be76f2c#card` — Resculpt: noun premodifiers Elemental, creature.
- `81622cf6-3021-4ad4-933d-a7e3d8ac817f#face:1` — Echo of Death's Wail: noun premodifiers Rat.
- `8191342b-b25e-4c4d-8f69-aee662148ff4#card` — Teysa, Orzhov Scion: noun premodifiers Spirit, creature.
- `81b15ed1-7069-4f8b-96b9-f6d67298afef#card` — Voice of the Provinces: noun premodifiers Human, creature.
- `81cb888c-c4d3-45a2-a03e-bfd91a2af2be#card` — Mirrorform: noun premodifiers nonland, non-Aura; negative owners lexeme:type/land/non, lexeme:enchantment_subtype/aura/non.
- `820dab0f-3b6b-437c-ba4b-2311437709c3#card` — Aphemia, the Cacophony: noun premodifiers enchantment, Zombie, creature.
- `82284fe4-ecb3-4b27-bb03-27290ad15b17#card` — Tempting Witch: noun premodifiers Food.
- `8239c63d-097a-449a-953a-cd480e594aa8#card` — Watchful Giant: noun premodifiers Human, creature.
- `82a6fa6f-13c2-4ea3-bec2-c51b56b2783b#card` — The Ooze: noun premodifiers Mutagen.
- `82b6a283-9897-4ce1-8de3-e7579cc9b75b#card` — Ruthless Lawbringer: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `8305d576-21d8-4ce7-8eda-a7cd9793aca5#card` — Archmage Emeritus: noun premodifiers instant, sorcery; binary modifier coordination.
- `8345187a-a210-49d7-bdce-10fb2909e050#card` — Amnesia: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `83705258-1f42-41db-a32e-de99ffd759eb#card` — Saheeli, Jewel of Avishkar: noun premodifiers noncreature, Thopter, artifact, creature; negative owners lexeme:type/creature/non.
- `83874e60-291b-47a7-ba9f-69437fa7e3c7#card` — Flux Channeler: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `83930f98-5596-4713-998c-732d5ecedf72#card` — Pestilent Spirit: noun premodifiers instant, sorcery; binary modifier coordination.
- `83cdff93-531d-4148-bff7-5312b3b53bf0#card` — Thatcher Revolt: noun premodifiers Human, creature.
- `8415a482-3957-4c00-9015-99defc049877#card` — Spitting Sliver: noun premodifiers Sliver.
- `84910453-e5dc-4bff-8554-86a5f4ab2746#card` — Harbinger of the Seas: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `849a7ae8-2b4e-4fa5-9607-2bfc961fee40#card` — Join the Maestros: noun premodifiers Ogre, Warrior, creature.
- `84adce5c-39c7-425e-b163-4a1a3977364b#card` — Skycat Sovereign: noun premodifiers Cat, Bird, creature.
- `852621bf-9ab7-4027-89fb-b438e4080684#card` — Sage of the Falls: noun premodifiers non-Human; negative owners lexeme:creature_subtype/human/non.
- `854cac11-9a22-4dea-8c1b-c49a4858a754#card` — Raiding Schemes: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `855b168b-e514-4a43-8bbe-7f1df40031ae#card` — Call the Cavalry: noun premodifiers Knight, creature.
- `85e797de-da27-4cd5-8fa4-4aef948988b2#face:1` — Final Iteration: noun premodifiers sorcery, Human, Wizard, creature.
- `8629fdef-dbbe-4a97-a7e1-6b1646f4ca0b#card` — Elemental Eruption: noun premodifiers Dragon, Elemental, creature.
- `86529f12-6f0f-4b0d-96f1-785474f6763a#card` — Broodmate Dragon: noun premodifiers Dragon, creature.
- `86746a84-3e8a-44b4-8494-60cbee9816b5#card` — Galactic Wayfarer: noun premodifiers Lander.
- `868b758c-e21c-4079-a1fc-4c89af106f4b#card` — Howl of the Night Pack: noun premodifiers Wolf, creature.
- `86a7f685-05d0-4167-b5bb-8f6943c23087#card` — Zhentarim Bandit: noun premodifiers Treasure.
- `86c0ddaa-6926-450d-949a-6c89f43c71c2#card` — Crested Herdcaller: noun premodifiers Dinosaur, creature.
- `86ec0b56-497f-481c-a03a-c4e64e8e7404#card` — Searslicer Goblin: noun premodifiers Goblin, creature.
- `87217160-0687-4482-b9a3-91bad8527675#card` — Soul of Zendikar: noun premodifiers Beast, creature.
- `874e0b54-bc24-4887-abe1-1ecfd3a3abae#card` — Trostani's Summoner: noun premodifiers Knight, creature, Centaur, Rhino.
- `87547bf0-02cf-4d83-9a1f-c82d2f1a22f1#card` — Tranquil Domain: noun premodifiers non-Aura; negative owners lexeme:enchantment_subtype/aura/non.
- `8776d14f-a5a2-4ef3-98d0-e1148e579e4d#card` — Sacred Mesa: noun premodifiers Pegasus, creature.
- `87a77482-f286-4fb1-b179-85b2095bb768#card` — The Hive: noun premodifiers Insect, artifact, creature.
- `87e4c6ff-f83f-412b-9d23-aac7a57ef6db#card` — Seller of Songbirds: noun premodifiers Bird, creature.
- `87e9065d-814a-47fa-9e92-19fb6fd6d78c#card` — Historian of Zhalfir: noun premodifiers Teferi.
- `8845ba0d-c2f4-49e4-b06e-54a06a8297e0#face:0` — Brine Comber: noun premodifiers Aura, Spirit, creature.
- `8866d33e-8159-4e10-8b11-914c12a0918f#card` — Rhox Pikemaster: noun premodifiers Soldier.
- `888d12d4-aaa5-406f-9dc8-8c4e856eb2fe#card` — Opal Archangel: noun premodifiers creature, Angel.
- `88bb91b5-2ccd-4ce9-8cd4-e54d63c12abf#card` — Coruscation Mage: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `88bbbc00-0704-4dc0-897f-93184b2e1eed#card` — Ruthless Invasion: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `88e9006a-cfdf-40f3-80d5-23465f3dceca#face:1` — Bother: noun premodifiers Thopter, artifact, creature.
- `88fb9e82-28b9-4275-a1e2-cb3a9bfda127#card` — Vitu-Ghazi, the City-Tree: noun premodifiers Saproling, creature.
- `894a220b-1372-4a76-b38f-bf0d7001af41#card` — Giantbaiting: noun premodifiers Giant, Warrior, creature.
- `89d9641c-84a4-41df-af1f-5b552305e895#card` — Plated Sliver: noun premodifiers Sliver.
- `8a98f48d-a5e6-497f-ba9a-92af7b489871#card` — Criminal Enterprise: noun premodifiers Villain, creature.
- `8ad4f2fe-6d98-4279-a331-3817d40ae46d#card` — Seismic Assault: noun premodifiers land.
- `8b022754-6d16-470e-b754-4df6e4f4709e#card` — Goblin Instigator: noun premodifiers Goblin, creature.
- `8b33d890-7d67-44a8-a253-e2b171d7ca9d#card` — Sprout Swarm: noun premodifiers Saproling, creature.
- `8b4670b9-6701-4ca6-afd7-256742267e86#card` — Diplomacy of the Wastes: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `8c35fd11-be45-4984-bd83-6e4f3fbc47a9#card` — Wild Ricochet: noun premodifiers sorcery.
- `8c5264d2-8544-4e83-8ebc-7261dde80f01#card` — Voices from the Void: noun premodifiers land; adjectives basic.
- `8c5298da-6759-444e-8870-6c96d226c940#card` — Zombie Scavengers: noun premodifiers creature.
- `8ca33b31-4707-40d0-b4ed-cbe625793122#card` — Lancer Sliver: noun premodifiers Sliver.
- `8cbbb1f4-4654-4865-a14c-0c94cd5d8b19#card` — No Way Out: noun premodifiers Zombie, creature.
- `8d0213d7-9e25-49d2-93fa-1d346cb2e869#card` — Akroan Horse: noun premodifiers Soldier, creature.
- `8d035688-089f-4d5c-bcad-9140a6c1681b#card` — Cybermen Squadron: noun premodifiers artifact; negative owners vocab:PredicativeAdjective/Legendary/non; adjectives nonlegendary.
- `8d7cf56c-94fd-47d8-9e0f-cd3163688983#card` — Luminous Angel: noun premodifiers Spirit, creature.
- `8dd0f86d-6f9e-47e3-97dd-3a3fc075ea93#card` — Wurmcalling: noun premodifiers Wurm, creature.
- `8dd5f5af-d2d8-4356-8617-8381081b930c#card` — Yavimaya, Cradle of Growth: noun premodifiers land.
- `8ddfc283-c9b4-41a5-af88-cf0068e986cc#card` — Swan Song: noun premodifiers enchantment, instant, sorcery, Bird, creature; Oxford modifier series.
- `8e0b605f-2095-4189-8c71-9d1ac93db42e#card` — Sudden Insight: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `8e0f93d7-7208-4902-bd92-e7bf66d35fef#card` — Lightning-Rig Crew: noun premodifiers Pirate.
- `8e5d72a1-6ae0-445d-bac7-62da3bd3a8d6#card` — Mirran Spy: noun premodifiers artifact.
- `8e5e8bca-b52a-4178-ab53-bdd0e0e11dc1#card` — Heartless Pillage: noun premodifiers Treasure.
- `8e7b31eb-7a91-4992-b24a-d81173e1dbc8#card` — Deeproot Waters: noun premodifiers Merfolk, creature.
- `8e7c5649-41c2-4a0c-b873-e6f6c5908567#card` — Hidden Herd: noun premodifiers Beast; negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `8e953251-aa5c-4aa3-8e69-1ff1f609f3ff#card` — General's Enforcer: noun premodifiers creature, Human, Soldier; adjectives legendary.
- `8eb7c0a5-6190-40de-b473-2d1daa3bbe28#card` — Dualcaster Mage: noun premodifiers sorcery.
- `8ed8e27b-2137-4e19-99bb-5e0d6d32e3af#card` — Jace's Defeat: noun premodifiers Jace, planeswalker.
- `8f1e8f25-ee23-40a6-a3f2-369bb960267f#card` — Carrion Call: noun premodifiers Phyrexian, Insect, creature.
- `8f4b8a19-72f4-48ed-ac05-62a7c7525797#card` — Mesa Enchantress: noun premodifiers enchantment.
- `8f878efc-850f-43d2-a6fe-5ea8d1dd5afb#card` — Twincast: noun premodifiers instant, sorcery; binary modifier coordination.
- `8f8fd2ba-df0e-49bd-a916-cc4f50b9fa24#card` — Earthbending Student: noun premodifiers land.
- `8fa7bd8f-f712-4dd1-9ffb-e30dfe744389#face:1` — Throne of Death: noun premodifiers creature.
- `8fb599a7-2c61-446b-a7a7-cd5367f9b796#card` — Spore Swarm: noun premodifiers Saproling, creature.
- `8fc154ff-f2a2-4089-a504-f483f545606e#card` — Rise of Eagles: noun premodifiers Bird, enchantment, creature.
- `903d114b-1899-4e67-bee3-af0673850388#card` — Faebloom Trick: noun premodifiers Faerie, creature.
- `9087928a-4732-4661-b3ed-d0b1ea06edf6#card` — Grixis Slavedriver: noun premodifiers Zombie, creature.
- `9093e6f1-12bc-4dfb-bc78-6ead0752ad61#card` — Walk the Plank: noun premodifiers non-Merfolk; negative owners lexeme:creature_subtype/merfolk/non.
- `90e1bcef-53af-4c08-b713-bf10dfcbd577#face:1` — Cosmium Kiln: noun premodifiers Gnome, artifact, creature.
- `910ae0ca-257d-4b45-b039-f26e7b2f3d5c#card` — Aven Brigadier: noun premodifiers Bird, Soldier.
- `9110480b-a5d6-4cf8-a72c-dc1a50f11fac#card` — Practical Research: noun premodifiers instant, sorcery; binary modifier coordination.
- `919d21ad-3d1e-4f80-8005-64e3f8bf64d1#card` — Call of the Nightwing: noun premodifiers Horror, creature.
- `91c2f488-2b49-4499-8d5d-c1aff73a4b57#card` — Shattered Dreams: noun premodifiers artifact.
- `922176b8-7174-4248-92fb-b196ceb802c7#card` — Treetop Freedom Fighters: noun premodifiers Ally, creature.
- `922a8e27-a367-40c6-84df-86ed32f71c7c#card` — Oltec Cloud Guard: noun premodifiers Gnome, artifact, creature.
- `92e2f982-f8b9-4c3b-ab50-14c025370e5c#card` — Blaze Commando: noun premodifiers sorcery, Soldier, creature.
- `92f5f80c-de1a-44e8-93f1-e405241c1e82#card` — Might Sliver: noun premodifiers Sliver.
- `930248b3-44e6-4c0b-bd7d-4b48323a7272#card` — Bear's Companion: noun premodifiers Bear, creature.
- `9331cfd7-a818-45ed-9e4a-4d3c381052e0#card` — Wasp of the Bitter End: noun premodifiers Bolas, planeswalker.
- `936ae5dc-9838-47f3-ba1f-66523a7f5b76#card` — Alela, Artful Provocateur: noun premodifiers artifact, enchantment, Faerie, creature; binary modifier coordination.
- `939b77f1-d7c4-4e52-b487-5866145ace01#card` — Marching Duodrone: noun premodifiers Treasure.
- `939e6f71-185e-41f2-9d54-72cce06f1dce#card` — Thirst for Knowledge: noun premodifiers artifact.
- `93b0bd0e-414b-45bd-9d3d-7323140ef0c0#card` — Runaway Trash-Bot: noun premodifiers enchantment.
- `944904a4-aab6-4ad9-96d7-6a3e47c118ca#card` — Earthshaker: noun premodifiers Arcane.
- `94e8b0a9-44a1-4dce-8d44-78681ae638a1#card` — Fountainport: noun premodifiers Fish, creature, Treasure.
- `94fac5fe-97d5-4c12-a80c-8efff9d853ae#card` — Blood Moon: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `953a2bd3-5bca-41fc-8785-66b2d7fa381a#card` — Mai, Scornful Striker: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `95ff3ba5-cbd2-4723-9498-3fc6df5be973#card` — Reap the Seagraf: noun premodifiers Zombie, creature.
- `963f2848-15bd-441b-a55c-635f53b7b63f#card` — Gargoyle Castle: noun premodifiers Gargoyle, artifact, creature.
- `9676da93-1e17-4f6c-b610-a242b78c71ab#card` — Sensor Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `96818698-c104-48f4-b73f-c8722d1d1be4#card` — Darling of the Masses: noun premodifiers Citizen, creature.
- `977f0d8d-4359-44ae-8f22-2103ea909c40#card` — Errand of Duty: noun premodifiers Knight, creature.
- `9792ae99-7b3a-46cb-bd22-0b33c499ce1a#card` — Trapfinder's Trick: noun premodifiers Trap.
- `9794e31f-e73d-4aa0-a421-dc496080c987#face:1` — Invocation of the Founders: noun premodifiers sorcery.
- `97b37822-b030-4792-bcd7-c84499f51b18#card` — Drooling Ogre: noun premodifiers artifact.
- `97f9aa5e-e34d-4029-9213-b32ebaf9859c#card` — Their Name Is Death: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `980cd7d5-a511-49ab-bc52-08b71c65db76#card` — Kindlespark Duo: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `980f6341-3c13-4405-8550-b838b228f788#face:0` — Mouth: noun premodifiers Hippo, creature.
- `98116aec-2ab1-4bee-b727-9feff6274825#card` — Rukh Egg: noun premodifiers Bird, creature.
- `987aab63-a889-400f-a3bd-feaef978b407#card` — Fire Nation Warship: noun premodifiers Clue.
- `9895a33f-9bbd-4440-8c1a-0d401431b77f#card` — Tomik, Distinguished Advokist: noun premodifiers land.
- `99140891-face-4015-aacd-1309e87d8f9f#card` — Display of Power: noun premodifiers instant, sorcery; binary modifier coordination.
- `999a9310-d145-4c1a-8518-9d96602ca672#card` — Inkling Summoning: noun premodifiers Inkling, creature.
- `99e24041-910e-4ae2-a55b-b938d752eeff#card` — Wingmantle Chaplain: noun premodifiers Bird, creature.
- `99e3b8ee-29e3-41de-af31-a4e8800c4619#card` — Symbiotic Wurm: noun premodifiers Insect, creature.
- `9a107e48-3d50-4941-95b1-10f2b29a4245#card` — Stroke of Midnight: noun premodifiers nonland, Human, creature; negative owners lexeme:type/land/non.
- `9a538bd7-e665-4896-95df-bbab1e28f75e#card` — Watcher Sliver: noun premodifiers Sliver.
- `9a691e96-fb47-4c23-a063-3b287c51147b#card` — Treasure Dredger: noun premodifiers Treasure.
- `9b3155c1-c662-4ff1-80e2-4064e6233dc5#face:1` — Aqueous Aria: noun premodifiers Elemental, creature.
- `9b33059c-c862-4452-8609-14a0e2551fb3#card` — Cleaving Sliver: noun premodifiers Sliver.
- `9b984236-39cf-4552-827b-e81c26cfb388#card` — Broodmate Tyrant: noun premodifiers Dragon, creature.
- `9b9b086c-d201-4574-a1e9-0d6f8de53d7d#card` — Web Up: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `9bc8a69b-6065-4ef7-a580-adb1e7c72ef4#card` — Timber Protector: noun premodifiers Treefolk.
- `9be8e04f-f87f-4477-93f3-c546e9b346f0#card` — Mardu Strike Leader: noun premodifiers Warrior, creature.
- `9c247a17-361c-435e-90c8-ce9c6b71f7db#face:1` — Bring Back: noun premodifiers Human, creature.
- `9c5bc3a7-42fe-4f8f-b799-ece7dd98ea4d#card` — Thunderheads: noun premodifiers Weird, creature.
- `9c9b3bf7-afb7-4e9b-bdbc-e2b4d5ba477f#card` — Banishment: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `9ccb6ce8-ccb6-4809-a1cc-cebd38744815#card` — Goblin Rally: noun premodifiers Goblin, creature.
- `9ceac4c6-e62e-4445-852c-b2c621e4a83b#card` — Wildsear, Scouring Maw: noun premodifiers enchantment.
- `9cf65178-6408-46da-a940-f5cb0960b61f#card` — Trench Wurm: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `9cfe86ae-eebe-44aa-a956-4b3e9e621105#card` — Krenko's Command: noun premodifiers Goblin, creature.
- `9d1c3ec1-00a4-4890-93a3-e3cf137ca7e4#card` — Hobbling Zombie: noun premodifiers Zombie, creature.
- `9d6686a0-1648-42f6-a606-16d2a84138a7#card` — Embodiment of Agonies: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `9d9b52b2-2edc-4f7f-a8d9-e024b1398847#face:1` — Architect of Restoration: noun premodifiers Spirit, creature.
- `9da65d66-71f0-4e17-a1f9-6da6cd1cfa8a#card` — Scalelord Reckoner: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `9dc6281b-08e0-4c66-9aef-abc7184ca36a#card` — Prosperous Pirates: noun premodifiers Treasure.
- `9e0c5919-aed6-4d98-85c8-2658be78bdf0#card` — Master Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `9e0d7408-3827-4a65-9905-9fef3ae23c7e#card` — Divest: noun premodifiers creature.
- `9e767d44-feff-449a-bed9-0866c7bce846#card` — Joven: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `9e830359-ab92-4bfd-82b3-d9c67ac80cf7#card` — Air Nomad Legacy: noun premodifiers Clue.
- `9ea58595-fcb9-455c-9558-b6a3fb2b83c5#card` — Karmic Justice: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `9f2af06d-8fb7-4276-bb92-3559a6d1fa18#card` — Captain of the Watch: noun premodifiers Soldier, creature.
- `9f3e1eef-31db-4e1a-b5a9-ac835f6a3eab#card` — Eyeblight's Ending: noun premodifiers non-Elf; negative owners lexeme:creature_subtype/elf/non.
- `9fa38462-57b6-4f2f-a73c-f14a66f56946#card` — Mona Lisa, Ever Adaptable: noun premodifiers creature, Mutagen.
- `9fb80575-9c8e-4745-9a8f-d48cdcbbb868#card` — Guttural Response: noun premodifiers instant.
- `a08475a6-1fac-4a3f-b343-aed16a61bc16#card` — Jhoira's Toolbox: noun premodifiers artifact.
- `a0f6834c-faa9-4d90-adad-847a8b337fe6#card` — Bonesplitter Sliver: noun premodifiers Sliver.
- `a1090a50-2347-48d0-bb67-379accb8e02d#card` — Beskir Shieldmate: noun premodifiers Human, Warrior, creature.
- `a1112750-8d38-49e0-ab1c-77110b2bbc4d#card` — Ferocious Pup: noun premodifiers Wolf, creature.
- `a131c32d-2b4f-4ee4-aab2-ab05ab010978#card` — Pride Sovereign: noun premodifiers Cat, creature.
- `a13f4095-300b-434f-aa9a-3608aaae5163#card` — Piggy Bank: noun premodifiers Treasure.
- `a145ff8c-5812-4bcb-bd16-9839dc25121d#card` — Storm-Kiln Artist: noun premodifiers instant, sorcery, Treasure; binary modifier coordination.
- `a1afb796-1b0b-4f41-9c8f-5320c9af5989#card` — Absorbing Man and Titania: noun premodifiers creature.
- `a1d17244-9c92-4094-9843-d7ee31c85ea4#face:1` — Cast Off: noun premodifiers non-Giant; negative owners lexeme:creature_subtype/giant/non.
- `a1ea2cba-f184-4a27-9971-3a56b0a0e390#card` — Mascot Exhibition: noun premodifiers Inkling, creature, Spirit, Elemental.
- `a1f55890-31c5-4ed4-a2cd-7a4a9f05f8ca#card` — Reverberate: noun premodifiers instant, sorcery; binary modifier coordination.
- `a1fdfcc6-4b3c-40f8-a9e4-dff9ace74a59#card` — Honden of Life's Web: noun premodifiers Spirit, creature.
- `a2193302-402a-4cf0-8014-5facecad647b#card` — Twin-Silk Spider: noun premodifiers Spider, creature.
- `a2373025-20c1-4416-91f7-e51f68dbd146#card` — Elven Ambush: noun premodifiers Elf, Warrior, creature.
- `a26820b9-3bec-41b2-9812-99e4a2211297#card` — Glimmerburst: noun premodifiers Glimmer, enchantment, creature.
- `a275f7ac-7dae-4422-875a-f34f24b6f28a#card` — Samut, Tyrant of Naktamun: noun premodifiers instant, sorcery; binary modifier coordination.
- `a2f33166-6cfc-4dc5-8edc-088bf40f06b5#card` — Noggle Robber: noun premodifiers Treasure.
- `a30907c0-fbde-4fd3-a8c7-f304305fcea7#card` — Doomed Traveler: noun premodifiers Spirit, creature.
- `a317d5f4-a998-4f72-b080-5345bc7cc668#card` — Wiccan, Young Avenger: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `a349e526-882c-454f-ba79-ad17340f8153#card` — Avalanche: adjectives snow.
- `a3643f66-dfa2-4275-a093-23e162496354#card` — Perforating Artist: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `a3c1af66-63c8-41ec-a401-a3da1131dc67#face:1` — Deluge of the Dead: noun premodifiers Zombie, creature.
- `a4825093-ea29-47d9-b52c-1c7735f1909e#card` — Spirit Summoning: noun premodifiers Spirit, creature.
- `a5530fac-0bae-451d-921d-0dd4865ec91d#card` — Gild: noun premodifiers Gold.
- `a5a05edd-2a30-414b-b766-f0651d5783f5#card` — Forecasting Fortune Teller: noun premodifiers Clue.
- `a5bbfe32-2fb9-48a0-bc8a-e9899aac1fda#card` — Sentinel of the Nameless City: noun premodifiers Map.
- `a5ce2ccd-8c78-4749-8854-cb5fa5f4738e#card` — Allied Strategies: noun premodifiers land; adjectives basic.
- `a5fdf1b2-da22-4fa6-810a-85c3f7587535#card` — Halt Order: noun premodifiers artifact.
- `a6409aa6-035c-4859-8135-90d34d67f72c#card` — Kobold Drill Sergeant: noun premodifiers Kobold.
- `a6450b8e-eb18-431c-9eb7-7daf107978b2#card` — Hordeling Outburst: noun premodifiers Goblin, creature.
- `a6c5c5ea-0bd0-4d5a-bd3f-6217f14c9939#card` — Piper of the Swarm: noun premodifiers Rat, creature.
- `a6f38908-aa4f-4f99-a28e-85d11dab52e4#card` — Ruinous Ultimatum: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `a70d9645-7c25-4fcb-9d16-fb73ce8b55e2#card` — Sanguine Evangelist: noun premodifiers Bat, creature.
- `a7213bef-0e9c-44e1-97cb-d1da0ef370bf#card` — Call the Skybreaker: noun premodifiers Elemental, creature.
- `a76667b5-2144-4395-a5c1-57cb3c29a99e#card` — Hunted Lammasu: noun premodifiers Horror, creature.
- `a784481f-eccb-4112-bb38-04a659319660#card` — Pitiless Plunderer: noun premodifiers Treasure.
- `a79e21fc-89bd-4547-8bef-5510554e1431#card` — Midnight Haunting: noun premodifiers Spirit, creature.
- `a7a0e473-77c1-4a85-b6d7-b0dfdadb696f#card` — Contract Killing: noun premodifiers Treasure.
- `a7ec13c6-7ade-433a-b5a2-047854eef486#card` — Corsair Captain: noun premodifiers Treasure.
- `a842cc2b-52eb-4dc5-86c6-6575c2ed913d#card` — Zendikar's Roil: noun premodifiers Elemental, creature.
- `a8bac5a8-5a60-4f7a-a4d6-cb922da92472#card` — Drogskol Cavalry: noun premodifiers Spirit, creature.
- `a8c9f91a-b1e7-451d-b6db-ae865e2b853c#card` — Exclude: noun premodifiers creature.
- `a90d1d35-6d67-43b3-8256-b0318b3a4b07#card` — Prideful Parent: noun premodifiers Cat, creature.
- `a9268e61-2697-46ff-b219-4f2be646cbe4#card` — Wakandan Shield Guard: noun premodifiers Soldier, creature.
- `a94bb381-74e2-488f-896d-a017a2b568ab#card` — Avid Reclaimer: noun premodifiers Nissa.
- `a98c2d81-4add-4292-bbdd-e1b69ff936d4#card` — Planar Cleansing: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `a9a0088a-3c86-4baf-9754-79006f733ce1#card` — Nomads' Assembly: noun premodifiers Kor, Soldier, creature.
- `a9a6c2ba-30ba-4e45-9ea0-ea9833342069#card` — Dragon Broodmother: noun premodifiers Dragon, creature.
- `a9c5b155-7c09-4100-9679-8fca2b5e9222#face:0` — Child of the Pack: noun premodifiers Wolf, creature.
- `aa16307a-9c1f-4538-872c-95206e2b7a6e#card` — Join the Ranks: noun premodifiers Soldier, Ally, creature.
- `aa321138-b1a7-4b8e-a2ca-b9ce65704e92#card` — Satyr Enchanter: noun premodifiers enchantment.
- `aa571676-b390-4b8a-baea-4c4cd60c01f6#card` — Regal Caracal: noun premodifiers Cat, creature.
- `aa57f607-c5eb-45a7-bfb1-adce0aef49c7#card` — Master Trinketeer: noun premodifiers Servo, artifact, creature.
- `aa8b60b6-cf55-4aaa-9caa-2b17942d8269#card` — Icatian Town: noun premodifiers Citizen, creature.
- `aaadfe41-b2be-4183-b45e-a70e53a59d2e#card` — Retrofitter Foundry: noun premodifiers Servo, artifact, creature, Thopter, Construct.
- `aab3774c-4ec3-4946-a09b-af0fab7cacb7#card` — Food Coma: noun premodifiers Food.
- `ab8d5f5c-1976-4f77-8ed2-8d28ee666741#card` — Tireless Provisioner: noun premodifiers Food, Treasure.
- `abccb19f-098b-4453-b7bc-838ad5914ebe#card` — Penumbra Wurm: noun premodifiers Wurm, creature.
- `abd12238-ea9d-4124-8229-3a479b8464d7#card` — Golgari Raiders: noun premodifiers creature.
- `abd839ab-8b08-4bc0-900a-3a161c62b06c#card` — Nimble Larcenist: noun premodifiers sorcery.
- `ac230b68-f945-4a1a-bd89-33e6d62e3c36#card` — Silverquill Lecturer: noun premodifiers creature.
- `ac65ed8c-abe8-4044-9ce5-70b1370d0e19#card` — Boggart Mischief: noun premodifiers Goblin, creature.
- `ac6c5852-71c7-4f19-8c87-ccb345052862#card` — Distant Melody: noun premodifiers creature.
- `ac8cae63-270c-4f73-b29b-50f8f2395fd9#card` — City Pigeon: noun premodifiers Food.
- `acd17fbf-645b-4c4d-b2c6-44127e3b743d#card` — Feral Lightning: noun premodifiers Elemental, creature.
- `ace86e56-efde-4eb7-8815-71456a4c3abe#card` — Warren Soultrader: noun premodifiers Treasure.
- `ad08952a-bcae-41d6-8093-c18535dad7d5#card` — Necrogenesis: noun premodifiers creature, Saproling.
- `ad09b3c3-c8e7-481c-8c45-e7f234935117#card` — Anguished Unmaking: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `ad11ff81-acaa-4529-ba15-718dadbea259#card` — Aviation Pioneer: noun premodifiers Thopter, artifact, creature.
- `ad4dda1e-2236-45bf-b2f3-f63e790d837d#card` — Crush: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `ad69f48e-c387-4376-a04f-37d3992c7946#card` — Fierce Witchstalker: noun premodifiers Food.
- `adfba95e-c73c-415d-adea-edcd04372ba4#card` — Virulent Plague: noun premodifiers creature.
- `ae58e71c-b617-404f-a5dc-5e6a487f2675#card` — Steelform Sliver: noun premodifiers Sliver.
- `ae896d87-59c7-4d8d-a137-9e471d3e174e#card` — Spirit Cairn: noun premodifiers Spirit, creature.
- `ae9ca82c-e07e-4a41-a387-0ef7d6df14b6#card` — Onslaught: noun premodifiers creature.
- `af199b80-d5d8-417a-97d5-459a5bd48b22#card` — Prescient Chimera: noun premodifiers sorcery.
- `af38d683-77a3-4599-ad2a-1c293ae01d45#card` — Scuzzback Scrounger: noun premodifiers Treasure.
- `af4684cf-f109-44be-adfb-7c551a36635e#card` — Bishop of Wings: noun premodifiers Spirit, creature.
- `b0104d66-5c28-482f-838a-416aabaeaf43#card` — Destructor Dragon: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `b0709570-a145-4c3c-ba56-f06d25753855#card` — Hidden Spider: noun premodifiers creature, Spider.
- `b0aaa4b1-5188-43a6-997d-7a9b2ad452bc#card` — Chatter of the Squirrel: noun premodifiers Squirrel, creature.
- `b0d0c534-8bd2-4801-aadd-dcc9c92d6ec3#card` — Maul Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `b13b40f1-1b1f-40e8-989b-ef24dc05007a#card` — Despise: noun premodifiers planeswalker.
- `b13c0f76-fbda-4911-9442-c3d7e97f1aac#card` — Remove Soul: noun premodifiers creature.
- `b1fbf818-6699-4f05-9a91-19aa296526bf#card` — One Dozen Eyes: noun premodifiers Beast, creature, Insect.
- `b2223f1c-e607-43e0-86dd-5e3225330066#card` — Worm Harvest: noun premodifiers Worm, creature, land.
- `b2347910-d6c6-4681-8316-7ef27056485c#card` — Secure the Wastes: noun premodifiers Warrior, creature.
- `b24a87af-407f-4c58-80b4-caab9c65a233#card` — Crustacean Commando: noun premodifiers Mutagen.
- `b29abb09-75ea-431c-ab60-f2940d002e83#card` — Kyoshi Warriors: noun premodifiers Ally, creature.
- `b2b0a5d0-0084-43a4-b6dc-e893ef42cdb7#card` — Unruly Catapult: noun premodifiers instant, sorcery; binary modifier coordination.
- `b2c9f074-57ca-4709-976d-f432f632483f#card` — Extinguish: noun premodifiers sorcery.
- `b2e31c0f-8914-4a18-bc05-44efb7e0f18c#card` — Aspiring Aeronaut: noun premodifiers Thopter, artifact, creature.
- `b32253d6-5f00-40a8-a7e5-dc4655cd288c#face:1` — Chilling Chronicle: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `b34601d2-2aeb-4649-9d73-d0895611bd69#card` — Scion of Oona: noun premodifiers Faerie.
- `b349f018-c20b-48b0-9e65-d5fd56b24b88#card` — Entreat the Angels: noun premodifiers Angel, creature.
- `b373e978-c58b-465a-b68a-9d4acec9dce0#card` — Dwarven Blastminer: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `b37e9bc5-6a2c-4f08-b856-3f8d0d16defb#card` — Fire Nation Raider: noun premodifiers Clue.
- `b38c567e-fd79-4a74-ab5f-df4a6a8f7e00#card` — Dwarven Reinforcements: noun premodifiers Dwarf, Berserker, creature.
- `b395925b-2456-454e-9262-3b55c9f2799a#card` — Biomechan Engineer: noun premodifiers Lander, Robot, artifact, creature.
- `b3eb8f65-5eba-42a9-a1b4-4f37b14d03d9#card` — Attended Knight: noun premodifiers Soldier, creature.
- `b420d983-4edf-4cc1-b442-af2b67e49c76#card` — Kinsbaile Borderguard: noun premodifiers Kithkin, Soldier, creature.
- `b43f40e6-c0ad-4a12-b75d-f2ba12629bfe#card` — Goblin Trenches: noun premodifiers Goblin, Soldier, creature.
- `b47fd32d-1a56-414f-9d91-ddd46d4b5ac7#card` — Centaur's Herald: noun premodifiers Centaur, creature.
- `b4931b44-815e-40b5-b239-682d350f3347#card` — Wand of the Elements: noun premodifiers Elemental, creature.
- `b4fc198f-3595-4880-ad12-96c18fbf9c33#card` — Ego Drain: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `b500c460-2814-47a4-b307-17ca519f3565#card` — Gallant Cavalry: noun premodifiers Knight, creature.
- `b51e0ae3-126b-4b7e-9dd1-818a5b8e512a#card` — Dwarven Castle Guard: noun premodifiers Hero, creature.
- `b582b696-17fa-4eea-8124-293f9bd3a208#card` — Mad Auntie: noun premodifiers Goblin.
- `b5905ef3-6565-462f-a015-2bb5b5b09eff#card` — Wizened Cenn: noun premodifiers Kithkin.
- `b5ae8016-92ed-4ae8-83a8-65068defc616#card` — Tannuk, Steadfast Second: noun premodifiers artifact, creature.
- `b5cd9c55-34ad-4e1f-92b2-527c9babc4d4#card` — Steel Golem: noun premodifiers creature.
- `b5fcfcf5-8af5-4395-b989-427fa92e0dd2#card` — Geist Snatch: noun premodifiers creature, Spirit.
- `b6492082-d0ef-4f7c-a4b1-f8ecf6eb0890#card` — Liliana's Elite: noun premodifiers creature.
- `b75c3902-633e-4d24-acde-d7a9cc8f466e#card` — Glorybringer: noun premodifiers non-Dragon; negative owners lexeme:creature_subtype/dragon/non.
- `b800c3e5-4ed9-4603-a987-d830e90cf25e#card` — Phantom General: noun premodifiers creature.
- `b829de81-04cd-498e-9f31-074f23e2a488#face:0` — Spite: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `b8376cca-ea96-478a-8e98-c4482031300a#card` — Sliver Queen: noun premodifiers Sliver, creature.
- `b881d61c-d7cf-49af-bed7-3bc657ea1208#card` — Elvish Vatkeeper: noun premodifiers Incubator.
- `b891a683-2ebc-4e9c-b402-5dd9c1b42b69#card` — Contested Cliffs: noun premodifiers Beast.
- `b8b9e6fd-b3ed-4fbc-8753-74018fa33caa#card` — Roar of the Wurm: noun premodifiers Wurm, creature.
- `b907d375-b715-446b-8966-79959feb9f58#card` — Edge Rover: noun premodifiers Lander.
- `b90e2fb8-9b09-4d95-a8a2-5287b5f18132#card` — Synchronous Sliver: noun premodifiers Sliver.
- `b93383ea-0206-43c4-908e-f424ef4523c3#card` — Synod Artificer: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `b93cd38a-1d0c-40e6-aaf1-621a2c88bf46#card` — Harmonized Crescendo: noun premodifiers creature.
- `b947b9a7-7a42-447d-b3fd-e20dba8a8f41#card` — Grid Monitor: noun premodifiers creature.
- `b94c233e-11d3-4556-a1da-bd193e58b663#card` — Tempest Djinn: adjectives basic.
- `b94efd70-705d-4cdf-9990-3a065a541f28#card` — Tangleroot: noun premodifiers creature.
- `b97d21b4-d730-479f-872f-3e1645b66751#card` — Release the Dogs: noun premodifiers Dog, creature.
- `b9959d63-5489-4ed1-a04e-9d6ba4a5f68e#card` — Encroach: noun premodifiers land; negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `b9b0c209-e346-45ee-a272-0b53589c21df#card` — Summoner's Bane: noun premodifiers creature, Illusion.
- `b9cd714b-2ad8-4fdb-a8aa-82b17730e071#card` — Deranged Hermit: noun premodifiers Squirrel, creature.
- `b9efd1bd-d82a-43ad-aba7-df172b9751ce#card` — Nightsnare: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `ba2c2a65-256a-46db-bf2b-b02b7d114a42#card` — Doomfall: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `ba944437-0b55-47cb-92dc-2477ce6726c3#card` — Molecule Man: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `bacedc99-46d9-4757-8a27-8df77d7c2f02#face:1` — Welcome Home: noun premodifiers Bear, creature.
- `bb0107f4-723a-4f2c-8e4b-237d5f668e3c#card` — Discordant Piper: noun premodifiers Goat, creature.
- `bb813ffa-2615-4177-a7fb-6e0dfb819fc2#card` — Aven Wind Guide: noun premodifiers creature.
- `bb8c6dd2-1abe-4eb2-be1f-b0c078e5f257#card` — Turtle Blimp: noun premodifiers Mutant, creature.
- `bba44fad-b3ca-4892-b7d1-2d03478062af#card` — Carrion Imp: noun premodifiers creature.
- `bbe740f8-6e7c-4cfd-9134-9bf8d5f48cf5#card` — Beamsaw Prospector: noun premodifiers Lander.
- `bbf183bc-d502-4432-8202-f29f60c08396#card` — Argothian Pixies: noun premodifiers artifact.
- `bbfee53c-30a6-4ed0-beef-5c254be75fc5#card` — Jewel-Eyed Cobra: noun premodifiers Treasure.
- `bc00f29a-e9d2-4b86-bb22-33b686ac4365#card` — Bestial Incursion: noun premodifiers Beast, creature.
- `bc6f4f73-e10c-44ae-926d-9e5f6ddc14be#face:0` — Invasion of Kaladesh: noun premodifiers Thopter, artifact, creature.
- `bc8427fb-e5ee-4b44-9a4f-6d9f01b4b896#card` — Flamekin Gildweaver: noun premodifiers Treasure.
- `bc891258-2f22-4248-9972-4f41905744c6#card` — Magmaw: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `bca33fb6-cd37-4259-8c0b-7a4675754a00#card` — Hunting Velociraptor: noun premodifiers Dinosaur.
- `bcc53728-3f6e-48cf-a042-393a2d8098fc#card` — Chill to the Bone: negative owners vocab:Supertype/Snow/non; adjectives nonsnow.
- `bcd3e6a1-7b76-408d-9236-3791c3e1064a#card` — Excavation Technique: noun premodifiers nonland, Treasure; negative owners lexeme:type/land/non.
- `bd104c7e-311e-4b03-98d3-5f20f3a99d26#card` — Liliana's Mastery: noun premodifiers Zombie, creature.
- `bd4c46a3-b723-4b35-9061-9a1dee7cc9d8#card` — Fulminator Mage: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `bd95c832-9ae9-404a-a2bc-0760a5b9ff56#card` — Human Frailty: noun premodifiers Human.
- `bdce1af0-3643-4e77-88f9-320206a191d4#card` — Zombie Infestation: noun premodifiers Zombie, creature.
- `bdf28770-0962-4317-822f-c187bc6706d1#card` — Whirlermaker: noun premodifiers Thopter, artifact, creature.
- `be455184-c57d-4504-8c34-3be42bc3f04b#card` — Bramblewood Paragon: noun premodifiers Warrior.
- `be7e220e-272e-45ca-8265-8d78da2cd2a7#card` — Unforgiving Aim: noun premodifiers Elf, creature.
- `be857d6e-5bf2-491b-ab46-963b4d45adee#card` — Two-Headed Sliver: noun premodifiers Sliver.
- `bf276236-18a6-4a48-b47d-5227832d26e6#face:1` — Return: noun premodifiers Zombie, creature.
- `bf69dc2a-9aec-4181-bbe9-70875055ec03#face:0` — Infestation Expert: noun premodifiers Insect, creature.
- `bf69dc2a-9aec-4181-bbe9-70875055ec03#face:1` — Infested Werewolf: noun premodifiers Insect, creature.
- `bfddc488-ab38-4e6d-8dfc-22bf1a8e03fa#card` — Vessel of Ephemera: noun premodifiers Spirit, creature.
- `c04dd88f-fb7f-43be-b586-7fc5642073dc#card` — Avoid Fate: noun premodifiers Aura.
- `c0636d16-671c-4e80-af8c-67d80d2cd979#card` — Irregular Cohort: noun premodifiers Shapeshifter, creature.
- `c0c4ecf6-84e6-443e-8a81-e395b31c631d#card` — Prizefight: noun premodifiers Treasure.
- `c0cb1f37-1679-42c7-a794-81e088157eeb#card` — Desolation Twin: noun premodifiers Eldrazi, creature.
- `c1161505-1623-4475-a88e-2e48072cc2f1#card` — Goblin Warrens: noun premodifiers Goblin, creature.
- `c1372f18-9b33-40e8-bfe2-dc4ca3b3ef55#card` — Shatter Assumptions: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `c1669020-8f81-471a-9083-7e9ce3e8f387#card` — S.H.I.E.L.D. Deployment Drone: noun premodifiers Soldier, creature.
- `c17d5153-cc74-4fbb-9d75-f7ee7b557b0a#card` — Scatter the Seeds: noun premodifiers Saproling, creature.
- `c1c5f9da-1396-4406-b803-b64f66b8e49d#card` — Ooze Spill: noun premodifiers Mutagen.
- `c1d9c41a-0941-4b7e-b8a6-890d30386458#card` — Hunted Bonebrute: noun premodifiers Dog, creature.
- `c1f1ca09-897a-434c-bbb1-e4ad4a1547aa#card` — Cruel Witness: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `c247382f-821e-4f3c-9d34-450794ce74c3#card` — Kami of Fire's Roar: noun premodifiers Arcane.
- `c33604ac-636a-43ad-9efe-8d7b3a28bf81#card` — Drogskol Captain: noun premodifiers Spirit.
- `c3469de8-4bdb-42d4-9fc0-75fe31c3f6e5#card` — Lymph Sliver: noun premodifiers Sliver.
- `c34c17c7-3827-49a2-be25-67f44fdfe150#card` — Strike It Rich: noun premodifiers Treasure.
- `c3775748-ff62-469f-916b-8e551ccce23c#card` — Careening Mine Cart: noun premodifiers Treasure.
- `c45090bd-a589-45a0-8388-025b8e0b3487#card` — Argivian Cavalier: noun premodifiers Soldier, creature.
- `c4698dcc-baa0-44c5-a283-22f9dfe150f5#card` — Avarice Totem: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `c46a02db-13d6-477f-9da0-822599470168#face:0` — Dazzling Theater: noun premodifiers creature.
- `c470a802-931f-4b63-92fd-ef9cf2e796dd#card` — Junktown: noun premodifiers Junk.
- `c4efae03-99de-4439-925b-504602623bfd#card` — Sailor of Means: noun premodifiers Treasure.
- `c53d6d3e-64e3-48b9-afab-3bde3164888f#card` — Lab Rats: noun premodifiers Rat, creature.
- `c56f0f4c-c85a-4931-b22a-42dc5d1dc7dd#card` — Zombie Mob: noun premodifiers creature.
- `c5f5b7e5-af1e-4c8a-8d44-3e2152955d4e#card` — Tidal Wave: noun premodifiers Wall, creature.
- `c616242a-95f0-4cd8-99a1-9237cfea0e57#card` — Howling Giant: noun premodifiers Wolf, creature.
- `c62a9e10-cfc6-48cc-a3f7-68d624182a5a#card` — Endless Swarm: noun premodifiers Snake, creature.
- `c6836d78-ca71-4413-aced-d4de19976221#card` — Birthing Boughs: noun premodifiers Shapeshifter, creature.
- `c6bdaf76-6a03-4695-9c4b-f040e73435af#card` — Guttersnipe: noun premodifiers sorcery.
- `c6cd6934-b661-455a-8de5-ceb920664bbe#card` — Skittering Monstrosity: noun premodifiers creature.
- `c72ab66b-2011-449c-93a6-1808669653f0#card` — Woodwraith Strangler: noun premodifiers creature.
- `c7371040-160c-4502-b7f9-77b7f54cc020#card` — Hama Pashar, Ruin Seeker: noun premodifiers Room.
- `c77dc11e-d95d-4e7e-bb24-d971c179e9b0#card` — Legion Extruder: noun premodifiers Golem, artifact, creature.
- `c7ca25f3-7a22-477b-8546-4c2597d5d1ff#card` — Extinguish All Hope: noun premodifiers nonenchantment; negative owners lexeme:type/enchantment/non.
- `c7dbd008-e941-497e-8305-80c53b1dffe2#card` — Tukatongue Thallid: noun premodifiers Saproling, creature.
- `c7dd0614-e11e-4b6b-a2cb-9904e457148f#card` — Hive Stirrings: noun premodifiers Sliver, creature.
- `c7f33cea-2ec8-4081-9208-a5b1d86721b3#card` — Bastion of Remembrance: noun premodifiers Human, Soldier, creature.
- `c7feecf0-5229-4c12-806a-16c9ab38e147#card` — Vodalian Mystic: noun premodifiers sorcery.
- `c8bc28ea-5630-49d5-963a-5789fa83f3c7#face:1` — Have for Dinner: noun premodifiers Human, creature, Food.
- `c8c88c33-b8df-4b19-b12c-79c9709af8f1#card` — Mintstrosity: noun premodifiers Food.
- `c8f6cc1f-b7e4-470d-8d48-ef848d7c1116#face:1` — Rider in Need: noun premodifiers Knight, creature.
- `c9abbb90-e3ae-4e7c-87ca-06ebebf65edb#card` — Sunrise Sovereign: noun premodifiers Giant.
- `c9db31d7-00cf-4200-a11f-74cc2d9e843e#card` — Thundering Spineback: noun premodifiers Dinosaur, creature.
- `ca9d1d85-1b29-4779-9036-e4db6d330e71#card` — Teller of Tales: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `caed9999-74b8-43e3-a319-fa505c04c71c#card` — Magic Pot: noun premodifiers Treasure.
- `cb491d8a-2e9f-46fe-9590-44c3b4a25f1b#card` — Jan Jansen, Chaos Crafter: noun premodifiers artifact, Treasure, noncreature, Construct, creature; negative owners lexeme:type/creature/non.
- `cb752313-dc7f-47fc-9077-9e3298e8f5fb#card` — Root Sliver: noun premodifiers Sliver.
- `cba94732-1f1f-4ffd-9aba-45db990043fa#card` — Utter End: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `cbb5526b-3aa1-43d9-acdb-bd230ce3fe63#card` — Glimmer Seeker: noun premodifiers Glimmer, enchantment, creature.
- `cc60181a-5eeb-4d66-9876-3194bac1e7a7#card` — Lord of the Unreal: noun premodifiers Illusion.
- `cc71b6c4-09d4-423b-a134-2f2f2fd7b533#card` — Veteran Swordsmith: noun premodifiers Soldier.
- `cd1e6225-31b7-4a70-81ab-681abe860328#card` — Tinker's Tote: noun premodifiers Gnome, artifact, creature.
- `cd8a90a7-5502-405f-a49e-ad4252872595#card` — Blisterspit Gremlin: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `cd98a31b-cc7e-43f9-982e-109ad9850908#card` — Verduran Enchantress: noun premodifiers enchantment.
- `cdaab6b0-1a2d-4809-8e6b-56013acd8f78#card` — Cast Down: negative owners vocab:PredicativeAdjective/Legendary/non; adjectives nonlegendary.
- `cdab50a8-1e02-4ba5-8c09-e2837e7652f7#card` — Thundering Wurm: noun premodifiers land.
- `ce5ca56a-fce8-4e19-807f-7b8638fe4bcf#card` — Mudhole: noun premodifiers land.
- `ce76d17e-d536-46f0-98e2-3e1b1b4f61a4#card` — Stimulus Package: noun premodifiers Treasure, Citizen, creature.
- `ce873790-de4d-4ba3-8023-c44306ba3ff8#card` — Deathbloom Thallid: noun premodifiers Saproling, creature.
- `ce87e5ed-2562-4563-9a5b-bd53c04ec4a5#card` — Moan of the Unhallowed: noun premodifiers Zombie, creature.
- `ceddea59-de47-4ddd-b424-6e1dcb13ab79#card` — Allied Teamwork: noun premodifiers Ally, creature.
- `cef37acf-ee97-48bf-a605-b17efcce5ce8#card` — Fortified Area: noun premodifiers Wall.
- `cef7396d-2168-4282-bf38-670ca9eebd91#card` — Thought-Knot Seer: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `cef82674-1f61-407b-801b-a76a3803e598#card` — Voldaren Epicure: noun premodifiers Blood.
- `cf0f3aee-78b6-4a98-8e09-8861b73c2e74#card` — Conscripted Infantry: noun premodifiers Soldier, artifact, creature.
- `cf5b3249-619f-48f5-8885-00fd41d24825#card` — Dogged Hunter: noun premodifiers creature.
- `cf6bb391-4fde-43c4-a0c5-990630d0ddcf#card` — Protector of Gondor: noun premodifiers Human, Soldier, creature.
- `cfaa0d5d-d9f1-4586-a7b1-6576a58c0f1e#card` — Symbiotic Elf: noun premodifiers Insect, creature.
- `d02179a0-a6b7-49ef-a73e-cf11cabb896f#face:1` — Ysgard's Call: noun premodifiers Soldier, creature.
- `d06f0f7c-0270-44be-991a-a5ebbddb0900#card` — Purge: noun premodifiers artifact.
- `d08b8dd6-3a44-4e86-8ce8-2d0a014a9aa8#card` — Opal Acrolith: noun premodifiers creature, Soldier.
- `d08e9784-75f7-4164-ac48-d06160f8c56b#card` — Annul: noun premodifiers artifact, enchantment; binary modifier coordination.
- `d0b17b41-bf8e-4ffd-9301-8db745b01112#card` — Helldozer: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `d0d2c45b-b6e3-4999-bdab-976e8f0d6617#card` — Dragon Fodder: noun premodifiers Goblin, creature.
- `d0d7250b-70d2-43f9-ae15-837227061ccb#card` — Life and Limb: noun premodifiers Saproling, Forest.
- `d0fa43f6-d8f6-4674-b538-6d546b7dcf40#card` — Moon-Vigil Adherents: noun premodifiers creature.
- `d13f0907-de39-4a90-940a-831740d7aa9b#card` — Pilfer: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `d1678eed-a335-454c-9fa1-749317500a36#card` — Dregscape Sliver: noun premodifiers Sliver, creature.
- `d1a60f44-7696-49ee-91fb-cab5b3102962#card` — Wurmcoil Engine: noun premodifiers Phyrexian, Wurm, artifact, creature.
- `d1a9fd14-fecb-4d2a-983c-70143238275b#card` — Sworn Companions: noun premodifiers Soldier, creature.
- `d21c3c8f-d105-4ba9-bf69-e5f26f0f8ec5#card` — Intangible Virtue: noun premodifiers creature.
- `d2567448-f8a5-4bf3-9804-f5aac3b645ce#card` — Jund Battlemage: noun premodifiers Saproling, creature.
- `d25e8f54-1915-4d0b-b4b1-93ee40da2cfa#card` — Wriggling Grub: noun premodifiers Worm, creature.
- `d264903b-d23e-48c8-95f1-cb1606a705a2#card` — Akroma's Devoted: noun premodifiers Cleric.
- `d27ffb4c-3a34-41aa-b464-6b897132c4ea#card` — Wily Goblin: noun premodifiers Treasure.
- `d297ce39-da98-4351-be9d-24f4d09bee7f#card` — Kunoros, Hound of Athreos: noun premodifiers creature.
- `d2a372d8-3eb2-4ba4-a75f-7951654de383#card` — Informed Inkwright: noun premodifiers instant, sorcery, Inkling, creature; binary modifier coordination.
- `d2bd23a6-4f77-4d6e-bf8f-339cb7a4184d#card` — Mystic Denial: noun premodifiers sorcery.
- `d37e595f-afe7-4ad9-87db-b9ceb3f435e8#card` — Ring of the Lucii: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `d3839a3f-3018-4a8d-b298-3f5b5fc70cd1#card` — Vraska's Conquistador: noun premodifiers Vraska.
- `d385c2e1-a62e-43ad-80ef-a6178deac82c#card` — Brindle Shoat: noun premodifiers Boar, creature.
- `d38937a6-ab1c-4de8-a278-d81a6ce0d921#card` — Paradox Engine: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `d398dca3-f3a5-44c9-998f-b4fce9e5c2bf#card` — Infestation Sage: noun premodifiers Insect, creature.
- `d3ba6922-c2f7-45ab-87a3-d4bbd770d1ba#card` — Graf Harvest: noun premodifiers creature, Zombie.
- `d3d61a6a-7870-44df-92e5-513dac2c2c3e#card` — Ravenform: noun premodifiers Bird, creature.
- `d3df7128-31dd-4d71-90be-87e2e9ff51b4#card` — Dust Bowl: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `d3e1848a-c236-4352-a7be-b0b4699af968#card` — Hunting Pack: noun premodifiers Beast, creature.
- `d3fa1320-9e95-437a-aa36-102a2722fc42#card` — Thief of Hope: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `d407abd0-69a9-405d-ab69-75b379a45780#card` — Soul Sculptor: noun premodifiers creature.
- `d4246e4d-390d-4925-a5a8-89cd096a237c#card` — Sphinx of the Final Word: noun premodifiers instant, sorcery; binary modifier coordination.
- `d445a584-98e7-4501-bb20-e76c9cafa757#card` — Zuko, Avatar Hunter: noun premodifiers Soldier, creature.
- `d4a9d63a-5a8a-4f51-9e49-43d6dc3234b1#card` — Greedy Freebooter: noun premodifiers Treasure.
- `d512d4ae-5db3-4fe5-8017-6a942004712d#card` — Frenzy Sliver: noun premodifiers Sliver.
- `d513e5df-31b0-4756-bf47-403460d0068e#card` — Battle Sliver: noun premodifiers Sliver.
- `d56afdf6-51e0-4151-a86c-4c4827a50a0e#card` — Umbral Juke: noun premodifiers Inkling, creature.
- `d5b021e3-a0c1-471b-9088-a76816806354#card` — Spontaneous Generation: noun premodifiers Saproling, creature.
- `d6192840-5e5d-48fa-a256-3b1e8c1c307a#card` — Forbidden Friendship: noun premodifiers Dinosaur, creature, Human, Soldier.
- `d64687e4-ec37-4db2-9e60-4626c7a3b011#card` — Winternight Stories: noun premodifiers creature.
- `d68439b5-7011-4b1d-a7da-14ef0f4d9994#card` — Inevitable Defeat: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `d6a9746c-eba5-4735-aec1-e70edba2c686#card` — Secure the Scene: noun premodifiers nonland, Soldier, creature; negative owners lexeme:type/land/non.
- `d6c9c1f0-050f-40e2-ab6c-6566d0af418c#card` — Master's Call: noun premodifiers Myr, artifact, creature.
- `d6cd8c51-dd5b-4037-b6ae-1776e3743338#card` — Rootgrapple: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `d709f396-021f-49eb-adb5-374aa790eaae#card` — Merrow Skyswimmer: noun premodifiers Merfolk, creature.
- `d7158101-f6ac-4d61-9570-a3adc3c8b583#face:0` — Supply: noun premodifiers Saproling, creature.
- `d7644325-8a0f-4f40-bab6-518df7ac1d14#card` — Soulscour: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `d7c18aa7-4184-4658-9a5a-0b64992348ee#card` — Mystic Meditation: noun premodifiers creature.
- `d7fd16ce-282d-49cd-b2b4-0d25935e7a72#card` — Mycoloth: noun premodifiers Saproling, creature.
- `d812c8e3-a475-4ad2-a93f-7208ad873676#card` — Madame Hydra: noun premodifiers Villain, creature.
- `d8560ee9-8cbb-440b-a132-e102a1eb3e89#card` — Bloodlord of Vaasgoth: noun premodifiers Vampire, creature.
- `d8b7b78e-91b2-4ad5-aeb8-6ebdca363e71#card` — Rockslide Sorcerer: noun premodifiers Wizard.
- `d8fc015c-7e6d-441d-bbeb-98ca74b21314#card` — Join the Dance: noun premodifiers Human, creature.
- `d90882ae-6572-4214-8d9c-0bdfb75f9be2#card` — Prismatic Omen: noun premodifiers land; adjectives basic.
- `d981d85c-e97b-4fe7-b415-6e73a3e7e35c#card` — Akroan Crusader: noun premodifiers Soldier, creature.
- `d99fa026-6347-450a-9011-1fd79070a55c#card` — Goblin Ruinblaster: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `d9c42e56-1818-4b73-9cd6-1c5aaf6a5b56#card` — Kessig Wolfrider: noun premodifiers Wolf, creature.
- `d9d33dcd-3352-478e-afc4-40f2ff369c39#card` — Rise of the Ants: noun premodifiers Insect, creature.
- `d9e24d7c-0907-45c1-b5d3-9fb16a929868#card` — Organic Extinction: noun premodifiers nonartifact; negative owners lexeme:type/artifact/non.
- `d9f11aa1-9219-42a8-85a9-a8f204160706#card` — Meteor Golem: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `da113cc9-bb2b-4af4-9c43-32c165b87363#card` — Spider Spawning: noun premodifiers Spider, creature.
- `da363da1-aecf-4d49-8e77-8dc56981a741#card` — Emergency Eject: noun premodifiers nonland, Lander; negative owners lexeme:type/land/non.
- `da46904c-8fb8-44c2-b2ab-775a1cc12ec3#card` — Dramatic Reversal: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `da5ee877-143e-4f53-ad20-1bc527a8fff8#card` — Molten Vortex: noun premodifiers land.
- `da6f79ec-5f7e-4099-89c5-6d2bfc14d957#card` — Vitu-Ghazi Guildmage: noun premodifiers Centaur, creature.
- `da785227-cf8a-4d44-9e7c-fc909ea868f2#card` — Prosperous Innkeeper: noun premodifiers Treasure.
- `dae8dc90-8047-434c-9e3a-a2eaa6aba575#card` — Herald of Dromoka: noun premodifiers Warrior.
- `db2af2b0-a5aa-4e4a-b5bc-52a5ac3968fc#card` — Dreampod Druid: noun premodifiers Saproling, creature.
- `db6174d7-211d-4817-b8e4-8384594c83f9#card` — Urborg, Tomb of Yawgmoth: noun premodifiers land.
- `db649e15-24ca-4fd8-9f0b-f1df28c9c57d#card` — Vinereap Mentor: noun premodifiers Food.
- `db65f874-6bf3-4fdb-a977-10b01b7ee2e9#card` — Waterwind Scout: noun premodifiers Map.
- `db69c3c9-d809-4361-8055-8c2dd4c6dd67#card` — Gather the White Lotus: noun premodifiers Ally, creature.
- `db7c5a2f-9aea-4b14-86cf-7af79d6484af#card` — Hidden Horror: noun premodifiers creature.
- `dbde53c1-c6b7-4775-87d3-e95166392e60#card` — Reclusive Wight: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `dc430e1b-d57f-4035-b518-9bc2e26a6c04#card` — Hidden Ancients: noun premodifiers enchantment, Treefolk.
- `dc9b963e-eae7-4919-a5a9-16aad04e4b22#card` — Innocence Kami: noun premodifiers Arcane.
- `dcaf3c6f-06e2-4762-82aa-625113e375a1#card` — Steward of Solidarity: noun premodifiers Warrior, creature.
- `dccf2ca8-8c87-41c9-8373-351859396d05#card` — Primal Clay: noun premodifiers artifact, Wall.
- `dcd4da46-5438-4454-8b1b-43ca51bda1f9#card` — Murmuring Mystic: noun premodifiers instant, sorcery, Bird, Illusion, creature; binary modifier coordination.
- `dd3fd7cd-d8a1-48e2-861d-c7c40086dc75#card` — Blinkmoth Well: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `dd42b9eb-c341-4712-88ee-fce38aace663#card` — Chimeric Coils: noun premodifiers Construct, artifact.
- `dda4b515-49c0-43fe-9f6a-36defa326bb1#card` — Maalfeld Twins: noun premodifiers Zombie, creature.
- `ddc7f59a-bbb1-4ba1-82c8-6813fd191940#card` — Siege-Gang Commander: noun premodifiers Goblin, creature.
- `ddc83b35-cf7c-495b-bd30-e409b622e299#card` — Breeding Pit: noun premodifiers Thrull, creature.
- `de1ca6ed-b275-4f62-ba05-f31b3659b352#card` — Windgrace's Judgment: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `de7f1725-833c-4784-94b1-7a33e36aec94#card` — Rotlung Reanimator: noun premodifiers Zombie, creature.
- `de8ccd32-f537-4a11-8dc4-e48836a0b139#card` — Sidewinder Sliver: noun premodifiers Sliver.
- `def45f3a-dba0-4d08-b086-5236d6e0edb1#card` — Ojutai's Summons: noun premodifiers Djinn, Monk, creature.
- `df35bf19-4cbf-4624-925d-ceca6bff596e#card` — Somberwald Beastmaster: noun premodifiers Wolf, creature, Beast.
- `df3d5b86-7b75-4df9-8985-7159cde12261#card` — Ironheart, Clever Champion: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `df5c1f34-94e4-4f89-b1af-943c430862d0#card` — Oyobi, Who Split the Heavens: noun premodifiers Spirit, Arcane, creature; binary modifier coordination.
- `df7e296d-96de-4280-a4d8-49189544b9e3#card` — Avacynian Priest: noun premodifiers non-Human; negative owners lexeme:creature_subtype/human/non.
- `dfeb239b-8904-4ef0-9a91-edc901b0c49a#card` — Silverfur Partisan: noun premodifiers sorcery, Wolf, creature.
- `e01b3d52-297d-42e3-9de9-4f45c2359bcc#card` — Audacious Infiltrator: noun premodifiers artifact.
- `e01c8122-9159-4f28-ac6c-338bd889650e#card` — Slime Molding: noun premodifiers Ooze, creature.
- `e055bc6d-d395-4f85-8e15-3bce65b7fcca#card` — Redcap Thief: noun premodifiers Treasure.
- `e05f3236-dc82-4353-87ab-d26ae7caa0e7#card` — Fugitive Druid: noun premodifiers Aura.
- `e070028e-468a-41e9-9ddd-a8cd01551266#card` — Myr Galvanizer: noun premodifiers Myr.
- `e0f05979-daba-43eb-b356-24284e7ca99c#card` — Blur Sliver: noun premodifiers Sliver.
- `e10065ba-fe24-4974-a500-d4151397a4d9#card` — Trail of Evidence: noun premodifiers instant, sorcery; binary modifier coordination.
- `e101b744-9175-4aa5-bf80-d6944359e538#card` — Airbending Lesson: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `e11b2b6e-3819-4dcb-b64e-2dfe6a6b049d#card` — Skywarp Skaab: noun premodifiers creature.
- `e1635acd-ed1e-4038-a11a-6518df285253#card` — Call of the Conclave: noun premodifiers Centaur, creature.
- `e1899133-635d-46ff-b90d-6f8d46a1ffe1#card` — Vital Splicer: noun premodifiers Phyrexian, Golem, artifact, creature.
- `e1aad679-93ec-420e-881b-35ebc99763a2#card` — Presence of the Master: noun premodifiers enchantment.
- `e1c458ca-10f6-4da8-a93d-3adfb86fe97d#card` — Preening Champion: noun premodifiers Elemental, creature.
- `e1c6e606-c4e4-4067-8cb9-46753785dfeb#card` — Riptide Chronologist: noun premodifiers creature.
- `e20e5ad9-36b5-4962-acc2-555d069c5daf#card` — Wayfaring Giant: noun premodifiers land; adjectives basic.
- `e21ed0d0-26b1-4ffe-971a-babf37f4492c#card` — Kinsbaile Cavalier: noun premodifiers Knight.
- `e220138c-5fc6-487d-9fa9-f21b2fb1f12c#card` — Tempered Steel: noun premodifiers artifact.
- `e25e2a3f-4add-4744-b1f8-2e880998a51f#card` — Recruit the Worthy: noun premodifiers Soldier, creature.
- `e35d4c62-5211-4785-b683-a83719195d87#card` — Spinerock Tyrant: noun premodifiers instant, sorcery; binary modifier coordination.
- `e36b78ed-4e3e-406d-85b3-fe0deb493418#card` — Ray of Ruin: negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `e3b9c9e3-5a35-4dd2-ab17-d1b6a140b185#card` — Riddlesmith: noun premodifiers artifact.
- `e3d2df4d-8c79-4391-bc4e-69325958a5be#card` — Mage Hunter: noun premodifiers instant, sorcery; binary modifier coordination.
- `e3d7cea4-0d4c-4320-966d-17ef69b5e433#card` — Edgewall Innkeeper: noun premodifiers creature.
- `e445825a-e70e-4796-99be-4a7c4080b8cc#card` — Rhet-Tomb Mystic: noun premodifiers creature.
- `e462e78d-2d26-4a2d-8d8f-5f31abe5c924#face:0` — Invasion of Belenon: noun premodifiers Knight, creature.
- `e47253d1-9b12-4989-80c0-604906c88b5e#card` — Traumatic Revelation: noun premodifiers battle.
- `e4fe33e7-b952-4152-8c83-81d948756d2f#card` — Sliver Hivelord: noun premodifiers Sliver.
- `e5a9d34b-c7d4-4682-8e26-dd82e38ee84e#card` — Ore Gorger: noun premodifiers Spirit, Arcane; binary modifier coordination; negative owners vocab:Supertype/Basic/non; adjectives nonbasic.
- `e5c15ef5-6795-40c2-a131-a73f74142314#card` — Dread Rider: noun premodifiers creature.
- `e5fbd5c8-7b0c-4740-9430-456e86267966#card` — Scarblade Elite: noun premodifiers Assassin.
- `e60b6b71-eea8-42b3-81e2-c8fb1af5e218#card` — Nested Ghoul: noun premodifiers Phyrexian, Zombie, creature.
- `e67bd8eb-66db-48af-82f8-74f5595f1928#face:1` — Treats to Share: noun premodifiers Food.
- `e6e7b5ba-ea10-4f53-8afc-045aa1906c6b#card` — Syphon Essence: noun premodifiers creature, planeswalker, Blood; binary modifier coordination.
- `e6f59807-eaf6-4889-8c89-2ac915afefff#face:1` — That's Mine: noun premodifiers Treasure.
- `e7438616-cc42-4a6d-a7e3-5856cb0782fa#card` — Rooting Kavu: noun premodifiers creature.
- `e7bc7c44-946c-4131-b764-add0c5f21020#card` — Garrison Cat: noun premodifiers Human, Soldier, creature.
- `e7fa2fc2-8542-4069-b75f-5520dd30f74d#card` — Ral's Reinforcements: noun premodifiers Elemental, creature.
- `e84164cf-c3bf-45ae-9a0a-9dfeacd36768#card` — Elgaud Inquisitor: noun premodifiers Spirit, creature.
- `e85c6bd2-db15-4052-88c0-3ae25414f0ca#face:1` — Awaken the Ages: noun premodifiers Spirit, creature.
- `e86870b1-4028-4111-9717-7005f86afe85#card` — Feast of Dreams: noun premodifiers enchantment.
- `e8a9350a-07c1-47ed-8c4f-88e4b3b17545#card` — Hop to It: noun premodifiers Rabbit, creature.
- `e938ee4c-d5df-4d93-bd61-9e518fb1dc30#card` — Conclave Tribunal: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `e9bee79c-144f-41e4-a53d-17df8381f691#card` — Anaba Spirit Crafter: noun premodifiers Minotaur.
- `e9db79fd-a5b6-4d59-9176-e713e2e9c708#card` — Aether Shockwave: noun premodifiers non-Spirit; negative owners lexeme:creature_subtype/spirit/non.
- `e9e25800-9ee7-40c9-b22d-611c7281c125#card` — Coalition Victory: noun premodifiers land; adjectives basic.
- `ea1eb902-a23c-44ff-9169-19baf71de238#card` — Talrand, Sky Summoner: noun premodifiers sorcery, Drake, creature.
- `ea6616e4-db8d-4905-80f8-cb0162906850#card` — Emeria Angel: noun premodifiers Bird, creature.
- `ea74e89e-0724-4744-a6c9-e35c415624fb#card` — Sprouting Thrinax: noun premodifiers Saproling, creature.
- `eaaed253-9b11-47e2-8624-0340ab2207f6#card` — Resolute Reinforcements: noun premodifiers Soldier, creature.
- `eabcb1bd-5317-4e01-8f18-94f01c12c852#card` — Opal Caryatid: noun premodifiers creature, Soldier.
- `eada96a1-4964-4ad0-b0e3-2df08d6ae188#card` — Wilhelt, the Rotcleaver: noun premodifiers Zombie, creature.
- `eadd9559-6dcb-4b96-8c95-58abddd0e120#card` — Kessig Flamebreather: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `eb62aa4b-c11b-4195-ae85-cff8f78ce17b#card` — Advent of the Wurm: noun premodifiers Wurm, creature.
- `ebc70734-bf1a-4754-8094-bcbb4db91f30#card` — Envoy of Okinec Ahau: noun premodifiers Gnome, artifact, creature.
- `ebcbfdba-34c7-40eb-91a9-d306ff5ae8b9#card` — Common Crook: noun premodifiers Treasure.
- `ec136f6f-015b-4200-97b6-a2c1bf39e135#card` — Butterbur, Bree Innkeeper: noun premodifiers Food.
- `ecb198fc-5663-4380-b2da-ad54421e966d#card` — Knight Exemplar: noun premodifiers Knight.
- `ed5429bb-233a-4528-bf7d-df5f6b192b1c#card` — Mercenary Knight: noun premodifiers creature.
- `ed7a2ee0-289d-4e5f-92e0-1539395baf33#card` — Dawnhart Geist: noun premodifiers enchantment.
- `ed8e0c38-64b1-48a5-9623-935d59ffaacc#card` — Weirding Shaman: noun premodifiers Goblin, Rogue, creature.
- `edd8d1e8-be43-4c38-bb3a-83081fbaf0b5#card` — Thoughtseize: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `ee0a59ba-8d34-43d5-b9a5-42015588e3d5#card` — Ordered Migration: noun premodifiers Bird, creature, land; adjectives basic.
- `ee243f81-f51c-4d9a-a396-f7cef84b46c1#card` — Call of the Herd: noun premodifiers Elephant, creature.
- `ee25680a-d01a-4ddb-908b-27a9bea0ceae#card` — Smelt-Ward Minotaur: noun premodifiers instant, sorcery; binary modifier coordination.
- `ef08d371-9d6c-4a2d-a444-d475fbf1b633#card` — Bestial Menace: noun premodifiers Snake, creature, Wolf, Elephant.
- `ef4b334b-8407-4147-b434-602e59806906#card` — Hunted Phantasm: noun premodifiers Goblin, creature.
- `ef693b9f-11e3-49bf-8387-b8f480b9007a#card` — Leaf-Crowned Visionary: noun premodifiers Elf.
- `efef2d33-5354-4fce-880e-cb91fa60f730#card` — Opal Gargoyle: noun premodifiers creature, Gargoyle.
- `f009ef70-d718-45cf-bb22-e560484bbfe5#card` — Rite of the Serpent: noun premodifiers Snake, creature.
- `f00dc657-4695-497d-aad3-af7b6a3ccf94#card` — Fortifying Provisions: noun premodifiers Food.
- `f05d5632-0a82-45bf-b66e-22cf4080ed5e#card` — Shadowbeast Sighting: noun premodifiers Beast, creature.
- `f0787397-8edb-4794-8ee1-22f97a057be8#card` — Penumbra Bobcat: noun premodifiers Cat, creature.
- `f0a7ef59-7897-4e84-a85e-1fcbe3dc9f6a#card` — Brood Weaver: noun premodifiers Spider, creature.
- `f0cbf361-d7dc-4556-a654-71c08785095a#card` — Crypt Lurker: noun premodifiers creature.
- `f0db78c7-09b5-4981-8ca8-6dbab597c5d9#card` — Waylay: noun premodifiers Knight, creature.
- `f10039ae-2695-4b0a-82b2-82ee79a992c7#card` — Witherbloom Pledgemage: noun premodifiers sorcery.
- `f105960e-6609-4192-afc0-e243013b7cc1#card` — Volo, Guide to Monsters: noun premodifiers creature.
- `f17d3aa4-6e88-4973-b2a4-942098a1c463#card` — Dragonlair Spider: noun premodifiers Insect, creature.
- `f1ac4e0e-5633-46e4-9a21-d3924e481d13#card` — Borrowed Time: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `f21c3d22-6149-475f-ae6e-37a25ef020e5#card` — Gaea's Herald: noun premodifiers creature.
- `f28b21a6-f7ce-437a-8c5b-0423cb55cefb#card` — Banishing Light: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `f28b2ebe-49ef-4585-9bf2-7340abf3f0d3#card` — Hunter Sliver: noun premodifiers Sliver.
- `f2e22c9d-b0ec-42bb-a520-3c36df546ba0#card` — Sigardian Priest: noun premodifiers non-Human; negative owners lexeme:creature_subtype/human/non.
- `f328012f-4696-47bb-ab43-247bdcb13f03#card` — Dragoon's Wyvern: noun premodifiers Hero, creature.
- `f340018d-8b68-45d5-a72e-a922be68365d#card` — Paladin of the Bloodstained: noun premodifiers Vampire, creature.
- `f3aaef18-dc32-40d6-b48c-f957aa31247f#card` — Argothian Treefolk: noun premodifiers artifact.
- `f3abd4d1-a975-4e85-8684-aa0fce029670#card` — Grave Titan: noun premodifiers Zombie, creature.
- `f3b071a1-7501-4424-979a-780fa90eecb8#card` — Thermokarst: adjectives snow.
- `f3c104e2-470b-429f-a047-a21edc2adb3b#face:0` — Lambholt Raconteur: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `f3c104e2-470b-429f-a047-a21edc2adb3b#face:1` — Lambholt Ravager: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `f3f01fe8-0a2c-4afa-94d2-268d27c0d252#card` — Extinction: noun premodifiers creature.
- `f414f997-06f3-4741-a9a5-9753c182e862#card` — Kraul Foragers: noun premodifiers creature.
- `f457f182-69d0-468d-9bf0-7170363bac26#card` — Hinterland Drake: noun premodifiers artifact.
- `f4ad190a-6c82-4923-af54-b4cce8d6465f#card` — It of the Horrid Swarm: noun premodifiers Insect, creature.
- `f4d54a89-8409-4fbc-b01f-3b03f352820f#card` — Transluminant: noun premodifiers Spirit, creature.
- `f5240f9a-597b-43e7-9876-74559df1354e#card` — Kyoki, Sanity's Eclipse: noun premodifiers Spirit, Arcane; binary modifier coordination.
- `f5677831-6ca9-4c7b-b02d-f44079c3f663#card` — Voltaic Construct: noun premodifiers artifact.
- `f570bac8-9987-4963-af02-476d18abc847#card` — Slithering Cryptid: noun premodifiers Mutagen.
- `f5c5f64c-6911-430c-a825-b32b96d39c7d#card` — Beetleback Chief: noun premodifiers Goblin, creature.
- `f5d1a004-5598-4201-ac07-bca6cd23008f#card` — Furious Assault: noun premodifiers creature.
- `f61c0fc0-7241-455e-bbf5-f1f3862d46e3#face:1` — Fungus Frolic: noun premodifiers Saproling, creature.
- `f61f4307-4eec-476a-97e5-d4d3da2b54c3#face:1` — Ardenvale Fealty: noun premodifiers Knight, creature.
- `f621dcaa-c500-4a17-a65a-60d9b5516a3b#card` — Goblin Wizardry: noun premodifiers Goblin, Wizard, creature.
- `f664596a-9ad1-4db3-95b0-457698ad2f66#card` — First Sliver's Chosen: noun premodifiers Sliver.
- `f6771d32-395e-4832-b33d-cbc12dff1516#face:0` — Thranduil, Sindarin Liege: noun premodifiers Elf, creature.
- `f6abdf34-cfdc-465c-b067-cdebb659406e#card` — Elfsworn Giant: noun premodifiers Elf, Warrior, creature.
- `f6b499d5-0e39-46ce-9de9-f64a2cc2cb32#card` — Chimney Rabble: noun premodifiers Phyrexian, Goblin, creature.
- `f7156897-2b02-4ecd-868d-d4d59244e9ed#card` — Third Path Iconoclast: noun premodifiers noncreature, Soldier, artifact, creature; negative owners lexeme:type/creature/non.
- `f74095aa-232f-432e-a899-86e9463ab5be#card` — Filigree Crawler: noun premodifiers Thopter, artifact, creature.
- `f762c225-1c8c-4f12-aa88-c8c89e0a7185#face:0` — Gingerbread Hunter: noun premodifiers Food.
- `f7971f22-2689-4c6f-9fea-1afc4f5afed3#card` — Drekavac: noun premodifiers noncreature; negative owners lexeme:type/creature/non.
- `f7d8b91b-6541-4d3e-af51-7e000eac69c1#face:1` — Adanto, the First Fort: noun premodifiers Vampire, creature.
- `f8526a36-eb8f-457d-b532-e12b47622cfa#card` — Phyrexian Triniform: noun premodifiers Phyrexian, Golem, artifact, creature.
- `f893d3d6-efef-4394-8e15-e01deed72b4f#card` — Sterling Keykeeper: noun premodifiers non-Mount; negative owners lexeme:creature_subtype/mount/non.
- `f90b00f6-36e0-4988-9409-57297483a952#card` — Cast Out: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `f90ebf35-08f9-4277-92d9-75165f32a84e#card` — Soul of Migration: noun premodifiers Bird, creature.
- `f92673fe-cd73-48ae-bfbd-aa750582b80b#card` — Taste of Death: noun premodifiers Food.
- `f943c005-9b77-411a-b522-1182e22724e1#card` — Cloudshredder Sliver: noun premodifiers Sliver.
- `f947808c-a1cc-41ea-9d06-e1279a0da527#card` — Compulsive Research: noun premodifiers land.
- `f9726123-f87b-41bb-9b6f-6d2fbcf0eae2#card` — Enduring Sliver: noun premodifiers Sliver.
- `f9e668f9-21af-4fd8-9a37-6505ab2234e3#card` — Elemental Summoning: noun premodifiers Elemental, creature.
- `f9ea5b9e-a23d-48cf-8601-56e79ca844f9#card` — Mardu Hordechief: noun premodifiers Warrior, creature.
- `fa04f161-4141-491b-ad6f-a16f69c862f3#card` — Dark Inquiry: noun premodifiers nonland; negative owners lexeme:type/land/non.
- `fa4afcf7-9cf7-4f67-95b9-1f7fe66bf332#card` — Grumgully, the Generous: noun premodifiers non-Human; negative owners lexeme:creature_subtype/human/non.
- `fa99cf82-ebf1-4526-ab9c-b24eb2970f2a#card` — Pawpatch Formation: noun premodifiers Food.
- `fae1d5d9-a50e-423e-8692-3e41f39df081#card` — Skittering Skirge: noun premodifiers creature.
- `fae37e28-e137-4177-b973-fa8b4dd8f409#card` — Generous Gift: noun premodifiers Elephant, creature.
- `fb3defc0-c93a-4423-9ff9-c399f8a8ef1f#card` — Jadar, Ghoulcaller of Nephalia: noun premodifiers Zombie, creature.
- `fb868840-09fa-49b1-85cb-b08ad065e972#card` — Bitterblossom: noun premodifiers Faerie, Rogue, creature.
- `fbb9dafa-b1aa-401a-b26c-c0c7e8b7222f#card` — Kor Castigator: noun premodifiers Eldrazi.
- `fbdec63e-0679-47e9-b82a-daa56df50481#card` — Spinneret Sliver: noun premodifiers Sliver.
- `fc6278e5-7eb1-4590-880e-fe9efa0ec4d9#card` — Veil of Birds: noun premodifiers Bird.
- `fcb34e92-f991-401c-a494-c2919083bf9e#card` — Veteran Armorsmith: noun premodifiers Soldier.
- `fcdeb83f-531c-4087-9254-7f19eb065f00#face:1` — Battery: noun premodifiers Elephant, creature.
- `fda06d9e-d17f-4bb6-916b-06116e47c8e8#card` — Rural Recruit: noun premodifiers Boar, creature.
- `fe84933e-d98f-4cbc-b272-387a80894e31#face:0` — Invasion of Ergamon: noun premodifiers Treasure.
- `fefef024-3a05-4221-bb74-419ee08716e5#card` — Pretending Poxbearers: noun premodifiers Ally, creature.
- `ff4297d3-3d96-4bd6-a606-1bdc20a6df2b#card` — Aether Storm: noun premodifiers creature.
- `ff48cf80-4950-4ae4-9f7c-8d826b2f26f7#card` — Dralnu's Crusade: noun premodifiers creature.
- `ffe05d4b-0ec9-4319-bb11-1366dc091224#card` — Venom Sliver: noun premodifiers Sliver.
- `ffff90c3-63c4-4dee-a21d-6b2b113f4f80#card` — Sinew Sliver: noun premodifiers Sliver.
