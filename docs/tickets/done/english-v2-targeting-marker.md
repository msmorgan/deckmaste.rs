---
needs: [english-v2-np-postmodifiers, english-v2-closed-class-single-owner]
---
**Represent targeting-sense prenominal `target` as one lexical Targeting
Marker with two thin syntactic projections.** Use the Oracle English
[`Targeting Marker`](../../contexts/oracle-english/CONTEXT.md) and Game Model
[`Target`](../../contexts/game-model/CONTEXT.md) definitions. The marker is invariant and has
one semantic contribution, but English distribution requires a determinative
projection for bare singular *target creature* and a nominal-modifier
projection under *another*, numerals, *up to N*, and similar determiners.

Replace the duplicate `DeterminativeHead::Target` and
`AttributiveAdjective::Target` lexemes with that one marker after the nominal
and closed-class ownership WIP lands. Retain the ordinary Target Noun
*target/targets* for phrases such as *choose new targets* and *the target*.
The Target Verb *target/targets/targeted/targeting* for clauses such as *a
spell that targets* is split to
[`english-v2-target-verb-subject-selection`](../../english-grammar-migration-obligations.md#retired-ticket-routing)
(coordinator, 2026-09-04). Both are legitimate homographs, not projections of
the prenominal marker.

Update the scanner, grammar, AST, category inventories, diagnostics, and tests.
Append a dated amendment to the English-v2 rewrite decision that supersedes its
categorical lexical claims that *target* is or remains a determiner while
retaining the singular determinative projection, quantified modifier
projection, downstream semantic projection, and noun/verb homographs.

Acceptance covers *target creature*, *target artifacts*, *target tapped
creature*, *two target creatures*, *another target artifact*, and *choose new
targets*, while rejecting adjective-like grading or predicative use. *A spell
that targets* is SPLIT to `english-v2-target-verb-subject-selection`
(coordinator, 2026-09-04) and is not acceptance for this ticket.

Ruling 2026-09-03: the user authorized superseding the rewrite decision's
target-as-determiner claim (including its 2026-08-27 target-as-noun amendment);
the amendment step above is not a STOP.

## Landing record

Two STOPs were taken on the Target Verb, both RESOLVED BY SPLIT (coordinator,
2026-09-04) to `english-v2-target-verb-subject-selection`; this ticket lands
the Targeting Marker partial described below. STOP 1 (change `wxwlzqxs`,
2026-09-04): the Target Verb's imperative reading — resolved by split.
STOP 2 (change `lzyywvxm`, 2026-09-04, after the coordinator's imperative-head
licence ruling): the Target Verb's finite reading still outranks the marker
nominal, a Subject-selection gap the grammar cannot declare yet — resolved by
split. The landed partial replaces the two duplicate prenominal lexemes with one `TargetingMarker::Target` vocabulary
member and the ticket's two thin projections:
`TargetingMarkerDeterminative` for bare singular nominals and
`TargetingMarkerNominalModifier` beneath another Determinative. The existing
count noun remains independent. `AttributiveAdjective::New` is added because
the required *choose new targets* witness otherwise has no lexical owner. The
dated rewrite-decision amendment records the authorized lexical ruling.

The coordinator ruling authorized a general declared Verb feature whose domain
is `Allowed`/`Barred`, defaulting to `Allowed`, with only the Target Verb row
declared `Barred`. The imperative Clause construction reads that feature; no
word-naming guard, narrowed form, dominance edge, or exception is involved.
That implementation eliminated the 53 previously observed wrong imperative
selections while leaving the plain Inflectional Form available after a modal.

The required package gate then exposed a distinct finite-Clause regression in
the existing positive witness *Two target creatures or planeswalkers gain 2
life.* Adding the Target Verb creates two candidates. The intended
`NominalModifiedPluralCoordinationNominalValue` candidate uses the Targeting
Marker over *creatures or planeswalkers*. A competing
`ClauseCoordinationOrClauseCoordination` candidate instead reads *Two* as the
first Subject, the Target Verb as its finite Predicate with *creatures* as its
Object, and *planeswalkers gain 2 life* as the second Clause. Structural
specificity selects that competing analysis. The exact existing assertion
`coordination_minimum_arity_and_agreement_are_unconstructible_when_inconsistent`
failed with candidate census `2` rather than `1`; changing its expected value
would flip a correct positive witness to the wrong selected analysis and is
forbidden by the assurance rule.

The authorized feature can bar only an imperative Clause head and therefore
cannot eliminate this finite-Clause analysis. Doing so would require new
authority for a finite-clause or Subject licence, form narrowing, dominance,
an exception, or a construction/lexeme-naming guard. The implementation
experiment and its test-output re-spellings were removed. The coordinator's
2026-09-04 ruling split the Target Verb and its *a spell that targets*
acceptance out to `english-v2-target-verb-subject-selection`; the marker
partial below is this ticket's landing.

| measure | before | landed |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected / covered | 16,771 | 16,824 |
| parse failures | 15,870 | 15,817 |
| unique | 11,515 | 11,527 |
| specificity-resolved | 5,256 | 5,297 |
| unresolved / exception-resolved / internal | 0 / 0 / 0 | 0 / 0 / 0 |
| round-trip / ownership / traversal failures | 0 / 0 / 0 | 0 / 0 / 0 |
| licensed vocabulary/Lexeme homographs | 2 | 2 |
| unlicensed form-literal/vocabulary overlaps | 25 | 25 |
| Construction declarations | 395 | 397 |
| coverage-lock lines | 49,421 | 49,474 |

Every number in the table and below was measured on change `lzyywvxm`, whose
coverage lock reports `covered` = 16,824, and re-measured unchanged by the
landing review on the post-`kata refresh` tree at change `uvtnsnln`, same
lock, `covered` = 16,824.

The schema-4 lock is add-only `+53/-0`, with source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`,
normalization digest
`f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`,
and SHA-256
`d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`.
Every addition below was inspected: in each selected analysis,
*new* is `AttributiveAdjectiveModifier` and *target/targets* is the ordinary
`CommonNoun::Target`; the parent already determines any other targeting-sense
*target* in the same unit. Resolution, candidate count, and selected ordinal
follow the name.

- `742f4c5aeeb1a9f0f22b854b642a697d6244fa902bf4aeded157059d6dbcb104` — Aethersnatch (specificity, 2, 0)
- `ee2801b34c602643fb38f8ad85c37386d882ff5a86246bf86cf43b7cc252c45b` — Alania, Divergent Storm (specificity, 18, 1)
- `9d989767f519fbf7da8f5cd0d0d45a645d8cfc73aaa45261b93dd5cc085f4281` — Aziza, Mage Tower Captain (specificity, 4, 2)
- `63eedfee32bfc0a51332f55a910f49bc346e5e7a08d65600c3a292184aba0980` — Battlemage's Bracers (specificity, 2, 0)
- `c3c86fb652089cc5935a90637bf1369d8cd31a54dc713c0c9909b3238485f596` — Boltbender (specificity, 4, 0)
- `6ea8fd880c099b7ad220e2235cd71f41cee3394b432e9711d30ffa63bbf355d6` — Bygone Marvels (specificity, 4, 1)
- `359581f1043bbe8a23c899f57e0eb1e9cc5f9f44a1fbd6f45212696375051f65` — Chain of Acid (unique, 1, 0)
- `ce6e52149ee8e2ed89a1039ea0df7b971d51726a61b17fef1dd7e416aa6d106a` — Chain of Silence (unique, 1, 0)
- `2605f7c2b3565ad6e07ff516af075a4408e4d602e61e5584bc2d1bfbeb9a5118` — Chain of Smog (specificity, 2, 0)
- `77b88016229697232cf03104cfad5cc17dc3e98703e959317dc3efc7559aba64` — Chancellor of Tales (unique, 1, 0)
- `888a1c70d2d8b6025044c5925582dbc8ff43b9a341cbb1301a9dc386c2a735a7` — Chandra's Regulator (unique, 1, 0)
- `3c3be2e747952fb75cccae61cbbada05e98597fc804943496a4b44c3d49687ec` — Cloven Casting (specificity, 3, 0)
- `12f7e141ebb6c0bff4b97cace4755fa6e2189bd63149f9497d251170dd7a47cf` — Commandeer (specificity, 4, 0)
- `07bdb16111fd9bd77c72699321335be87d0b5ffcbdb513f5acd182bcfbe37a51` — Curse of Echoes (specificity, 2, 1)
- `61879b56e2bd3aedfc51a4cde9b9e8f85a9d2643ac69831ab891bfa58c226ad9` — Double Vision (unique, 1, 0)
- `2a5dddf6c2afd60026dbdbd47df6611a19412ac5f609a7f2c94f95562d5dcf3b` — Dual Casting (specificity, 2, 0)
- `406693d7edd7253667a1abf89680308fcbac189a71ffc01333d3bbdf0edf134f` — Dualcaster Mage (specificity, 2, 0)
- `d6f42100569b0a538a277f4a885ea2d95e88a18c8aef9b4126b65bc7636b9fbe` — Echo Mage (specificity, 4, 0)
- `6200769827e75b74c9adebdfa54c757aaca5c3dbea8a90d2d55cbd487c4985cf` — Expansion // Explosion (specificity, 2, 0)
- `6f2f1a55326c03b5459c15d44fc9e077b8c44e6e77dcfc81839b65cc8a9bc8cd` — Fire Lord Azula (unique, 1, 0)
- `4f8e97e5d05fe657fa3d6be90734657136be8f17b65e1413490360818837659b` — Flare of Duplication (specificity, 2, 0)
- `c552c26131314c5b6ab9ce00d4d4e22cfcaea167035c35d86726081d964482c5` — Geistblast (specificity, 2, 0)
- `08dd75a75c60de80338455691777ea78ebaadcff5a3ecea89605581857f08756` — Gorion, Wise Mentor (unique, 1, 0)
- `522aeeaa89ea4289987964bccc6d234fe6d19c8db3cd44b1ba363263858ded67` — Hive Mind (specificity, 2, 1)
- `052e0d7ca9de8b0dd98b31e464492199a4c604a737033af77b0f34637c1b38d2` — Illusionist's Bracers (specificity, 2, 0)
- `36fd4e1a56d0e416b9c7bb2ad5718f747ff08be878e00dabc23c37cb90e685f9` — Insidious Will (specificity, 2, 0)
- `8d0b5ed64a14e742b37e7861429d5803ddb40dba818a77030bb90928a15d07e1` — Invasion of Arcavios // Invocation of the Founders (specificity, 2, 1)
- `9c9cdc36a66af5fa1022b49fda0d0b816ad885a22fc75c04a5104ab757ece2e8` — Invasion of Vryn // Overloaded Mage-Ring (unique, 1, 0)
- `19c334eefe025869736dc743593650853c3c7347ce80408256d2d2e978f41b14` — Izzet Guildmage (unique, 1, 0)
- `26dd9a902856f118185290f03b41bf0574435a1b6c8ccca7f14e8a0c99918cf0` — Kalamax, the Stormsire (specificity, 2, 0)
- `6d870eb96115efa5e97610f6304a408adf03892579fe97ac49cfc0a1c75798ef` — Kurkesh, Onakke Ancient (specificity, 2, 0)
- `77bdd123fa53e1452b81b05e9b63a6d0ff8670b13bf545ed436143fe1baf00f7` — Lucky Clover (specificity, 3, 0)
- `95b0bf0d7a74b42c490077b67b024aa010f72adb129b04f6718727873a7e926a` — Meletis Charlatan (specificity, 2, 0)
- `531fcbece1359d5e0382921407486a03a0cca9ab7b028aa66ccd56b3b1c764e6` — Mirari (specificity, 2, 1)
- `7f6914917a474f84391951c12caa4c624e6f3a8c7832448af287cdeaf494c38b` — Mirrorpool (specificity, 4, 1)
- `a5f4753a083b015ff0997dd0b00b4f821c6c526a41f583ce7e04f3e9e96a0787` — Narset's Reversal (specificity, 2, 0)
- `ef66b810466ae4c492dea04e7a28f6ef9a8683dafd384cf3f518a42bcfcfcf6d` — Naru Meha, Master Wizard (specificity, 2, 0)
- `7bbce9ea1c6e8dff7280f4937536aa19b497064971a53f1caf672c0042eda7d2` — Nivix Guildmage (specificity, 2, 0)
- `83cccadea63021a21f13570c8f252c22fb0908944aeffcc8255d947103cd51d5` — Perplexing Chimera (specificity, 6, 0)
- `c36936b1413625edee71559ddd286922d9be54e30ea65663f5e9715e010b1bcf` — Redirect (unique, 1, 0)
- `9b0ca002344974f23b16dcb7adba42707dc10c960a6c1fa488a3b318eafb9cea` — Refuse // Cooperate (specificity, 2, 0)
- `7557ca42c6b264e37d38542f6edf6d2d4d1f862516064b2c0a793495fb095a47` — Reiterate (specificity, 2, 0)
- `fc15ba30ae6c2d6d2a63c3e0984607a46ac32dbe61ec3f9c69ebd19306531eed` — Reverberate (specificity, 2, 0)
- `163b94b818079c013a43177dd002a83bdaff6adbb7d6ffbd405a1746751eac4c` — Riku of Two Reflections (specificity, 2, 1)
- `9264811d62b582c30e9c4ba48c54c9e1b3ea30795e8f38e238fa564784a3dfe7` — Rings of Brighthearth (specificity, 2, 0)
- `1fe7e24c234c1752b77d05b021472d4924717e204bdd3ceabfe533b0a35bff38` — Rootha, Mercurial Artist (specificity, 2, 0)
- `59d46073fe12b64a50d91dd1f5c618817fe6f68d3510790889605615e95e1356` — Sevinne's Reclamation (unique, 1, 0)
- `f9dbd6004af95195971d4b09fcf1a440c8a9373a3bb6432ee38132d7a6872e9a` — Sevinne, the Chronoclasm (unique, 1, 0)
- `4c10e36d2c99267ddf70624978253a90b07936af3121528cfa696c403ba54322` — Sigil Tracer (specificity, 4, 0)
- `f5951acd474fc14a3f7fd554cbe76ef71ea6b39c2b35df29cfbce5c3c5492500` — Sudden Substitution (specificity, 6, 0)
- `71eb3b2f729ec94ea975ed3dc83c8274564751cacedc85e88904bccf20be1fa1` — Swarm Intelligence (specificity, 2, 1)
- `0e009089cac56da6a1e0a31e236d5d34a0a5a1b68c032bc2b35ae5318b217da4` — Twincast (specificity, 2, 0)
- `deb968ba7615fa4e37b4ba8fbc3a31950e81d4f46ed39f42108cab201e2ad19c` — Uyo, Silent Prophet (specificity, 2, 0)

The landed-tree parent comparison covers all 16,771 previously covered
identities. Among the 5,805 target-bearing identities, 5,238 paths differ only by replacing
`DeterminativeSingularSimpleDeterminative` or
`NominalModifierAttributiveAdjectiveModifier` with the corresponding
Targeting Marker projection, 567 target-bearing paths are otherwise
byte-identical, and zero selected analyses change beyond that replacement.

Evidence carried to `english-v2-target-verb-subject-selection`: the withdrawn
Target Verb experiment changed the following 53 existing selected analyses.
None of these changes is present in the landed tree. For every listed identity the before analysis is a
`SentenceDeclarative/FiniteClausePlainFiniteClause` whose
`SubjectSubjectNominal` begins with the targeting-sense *target* projection;
the after analysis is a `SentenceImperative` whose head is the Target Verb and
whose `ObjectObjectNominal` incorrectly consumes the rest of the declarative
clause (sometimes through predicate-adjunct structure). This shared
before/after statement applies individually to every identity below.

- `f358f00631ebec6f92563ccf3a87524e3e69d798708c9f143427c4a3eb1bbb3e` — Aim for the Head
- `cddeef9bf2fcdc4a26ab99a51d26e5fb52846cd5218765ad88bb3c611816949a` — Allied Strategies
- `b86525fad7971e72143d7a1a313320837cb12ec9fc9d2fcbe1c6740b819837b9` — Ancestral Recall
- `905970f18e8576bfea7d19647cb92871eb43ec8f3ae2c441225598e34f95a4b7` — Ancestral Vision
- `bdbc59fedbb54176d795a552f1771f749d3718aeaa8c3200374c33141d4ff639` — Archmage's Charm
- `00626449f3414de8a4aee7e98a2e63b5945ad055630ee6578762f8825637c943` — Ashling's Command
- `2d35439acd3e51876242729c5416b67e1f818083a39876eec88ffda8711ae4d7` — Bargain
- `7d08b1d5adc10f574fe71d4979901f2d8379940a5f9af046e0c7e46a9b2400e2` — Blue Sun's Zenith
- `a15bb70b92c60c3073eba1c359d6c5e13ffbdd276fe73213faafbbe970d7be26` — Braingeyser
- `1c8d771ba60823749b7c106d88b17c9fe67e945806a0d92a68670f1e78e4f9c2` — Channeled Force
- `b00b7ca0aa5b724abf26c2942c6873f0c5055e711c233056e21b44310617c925` — Combat Tutorial
- `5b593e3d7d1ca56e1355acbb0e538f407894313bba5dc4e48ccefb00274c35a1` — Comparative Analysis
- `0f27891f66426aaf6f0dcce569e01a33e8cd4593761f3eec394a9cac4d18ed7d` — Compulsive Research
- `3c0db2cf1894a94486655535b46f9147a789ee6a6e558a57ae935487cd4b7185` — Consult the Necrosages
- `904cb9821649b859d5910cb6e0dc281a50db19dbd3885298bbd9a5d77d0a3397` — Debt to the Kami
- `94a4ec39d35be093b5878fc2ba04a06f15904aee342cc84927d7b2221602a518` — Deep Analysis
- `add30db29324ed3d93026923b10895fa8d4c75d0eff823ef1379740fd386af93` — Dimensional Infiltrator
- `29dbc150ecb974573e3d8df785b47b9b23d33499d1598c66ba2a15772686e6ad` — Dirgur Focusmage // Braingeyser
- `53f4eb4785a0a982b3f71497ad07bfb057e47ddda48bee08948bcc428a2e90b1` — Doomfall
- `c6e1e3d6ab2e5a1e927340668d38e974118cb58f92c5d7f169ba5601cadd12cb` — Early Winter
- `3729c229a19eafda08c9e7f88d44ee00e564ba5f679e16e190a20d4600de5afe` — Emeritus of Ideation // Ancestral Recall
- `5eb817d56e46f25939938e3df2625242731c02cc38dd4224020ba2dafc8df4c3` — Etched Monstrosity
- `b39cefa17b0031b062ade57faf0a7e3489d05c529d260ffd36d2fa9d1b9cd9d2` — Etched Oracle
- `237b77cd3fcdbbaa4720b7c6393db5d321ceb67afbf1bf5b7474b606f72e4e3c` — Expansion // Explosion
- `a8957f96a3d03cd14fed0c75aa754f0938e44d696bc19b642572e577686eca29` — Graveyard Shovel
- `be4bee8640271bac66752ebbceac93b719a3f72a5b825140f6c6ebab42bf9d4e` — Here Comes a New Hero!
- `8f8d35fcf645a2cac855d81a74ab77c701aa3d89d848036d05e37485e8fb3be7` — Homesickness
- `d5ed1b31585f1c7f2847391dd769a51ffa9b30fb22e66601620d7599087d6711` — Inspiration
- `400f12c2dbe97800831b5533b527fd32847b4cfddda1a67744a6c5286116c1b7` — Jace Beleren
- `6ced3f6efc9313b7c7b01c54085754da52f4007d9648967d942f1364f3073711` — Jace Beleren // Jace Beleren
- `2afdc2b6dda07670087009e1ef0893c9ba3f59f56c70f740d7502e7962a553eb` — Jushi Apprentice // Tomoya the Revealer
- `4d9b51e4e69c3483e6876d18e8b572b23c2fb55990999a2d818decc12132e643` — Knacksaw Clique
- `f00ae2e072a8ed424603f4040acadccd49b038a1ee42caa2d9c2572e33e56b65` — Kyoki, Sanity's Eclipse
- `462f33d9d96813fbcbf33afdfc57a9785430c78e49edb8bc93e5e66b35ea5b4c` — Limestone Golem
- `b1df4503abef6817ed7e861ee54436deb6d1e18e82f6a3028acaf83be08009d2` — Merrow Bonegnawer
- `ca09d20b5206ab70ecc74973415244d54c3efe3c38e93bafac3155346d666add` — Mudhole
- `f611d25e86d41994e31e6c854d6173fe1d008579489b479345e6f344aa396b03` — Oona's Grace
- `48fd9816488893a4181e28d69f93031f0c6abdd13d92ef620c5f19d63eaf13e3` — Opportunity
- `33ba0d7554cbd978947dec1ae8c83adf86aa9450e4ad3b0ec42ff7d5cabae445` — Ornate Kanzashi
- `66b4888dc2d03d4b9063826b6b6f36a830fcb587a3da00617c98ae65b4c82e3c` — Overflowing Insight
- `51540a8ffe3a67e398061371dd348bc41e0ceb9664dc07abcc603bddc3d2e20c` — Perfect Intimidation
- `1b6f5e6fede4f14f78608eff30ecc6f414f9a3885bdef6c6cef223e555af8152` — Relic of Progenitus
- `410fff1e2b52147ca38c2dcbb22187408b013fbd95bad65e8cf70c8743bb8f56` — Roiling Waters
- `b668663e740dcb8085b933dfa6d915ff3284400f5591452599d5d20b4571c58c` — Ruthless Negotiation
- `8982020853b3aea47553acd5d04f52aa1b2f03ba5729821d0bc478242a48765a` — Saltwater Stalwart
- `9f8c426cac9ac3e2c63e2252c5c99f5d6106cff7b1dc64ef33f5ea83a2b5e9a0` — Scrabbling Claws
- `4331a1cc5747285282a2791d93d6c0aa8c324342d31aa2efec5532c9431024d2` — Skullcap Snail
- `909e5af00d1b9e3276e9577e02fa8fe17be5ce6ad083ebd02ddba25c2bbfd877` — Strategic Betrayal
- `ab184fe9e4e8b5ea8ab276fdc8b7feb2c5198eff43fa8803473d334bff44ec3e` — Stroke of Genius
- `b34b54b14db3860697ea07fe36ab74eaec37e3654e36d30ba96dade074ce2519` — Thief of Existence
- `1964cc435dd34bedc0531e4953bd191298af22d124dd21882e50ee982a791957` — Thought-Knot Seer
- `213c171445dcea6a838aa7c95aea8e1b967678f9ab19f8b7c6386457542b914f` — Unscrupulous Agent
- `fbd886dc0d46c2715aa62fbe2183ab0ef3530a4aa18939a3b7589dceba77964e` — Xira Arien

Performance advisory on the landed tree, as re-measured by the landing review
on the final refreshed tree (8 workers, the shared-host cap): `coverage
--check` took 47.864877074 seconds at 93,437 ns/B, host load
28.40/31.50/38.75; `ambiguity --require-resolved` took 66.293516552 seconds at
103,801 ns/B, host load 31.58/32.19/38.45; `roundtrip --require-clean` took
112,036 ms at 104,184 ns/B, host load 35.93/34.97/38.83 (written in
milliseconds because a three-digit second count reads as a rule number). The
implementer's own pre-refresh
run used 24 workers and reported 121,211 / 194,643 / 193,073 ns/B; the
per-byte figures fall with fewer workers, so neither run is a quiet-host
measurement. Contention stamp (coordinator-supplied, replacing the
implementer's sandbox-visible `pgrep -c -x codex` count of 1, which measures
nothing): 4-5 concurrent codex executors and two reviewers were running on the
host throughout both sets of measurements. All three gates exceeded the
16.26-second quiet-host ceiling under load; this is advisory, not an additional
STOP.

Assurance census: restored 0; re-spelled 20 existing test functions and 5 test
helpers/fixtures; ignored with blockers 0; added 1 test function; removed 0.
The addition authenticates both marker projections, the retained count noun,
the *new* modifier, byte-exact rendering, and adjective-like grading and
predicative rejection with `is_err()` assertions. No positive witness was
flipped into a negative.

Landing-review gate artifacts on the refreshed tree (change `uvtnsnln`, 8
workers):
`cargo fmt --all` clean; `cargo clippy -p deckmaste_english_v2 --all-targets
-- -D warnings` finished with no warnings; `cargo test -p deckmaste_english_v2
-p xtask` green (`test result: ok. 27 passed` nominal_grammar, `39 passed`
parser, `100 passed` predicate_grammar, `18 passed` vertical_slice, `430
passed; 1 ignored` xtask, every other suite and doc-test ok, 0 failed
anywhere); `coverage --check` `summary {"total_units":32641,
"selected_units":16824,"covered_units":16824,"selected_uncovered_units":0,
"unresolved_ties":0,"internal_failures":0,"roundtrip_mismatch_units":0,
"ownership_failure_units":0,"licensed_vocab_lexicon_homographs":2,
"form_literal_vocab_overlaps":25}`; `ambiguity --require-resolved`
`unresolved_ties=0 internal_failures=0 exception_uses=0`, census
`unique=11527 specificity_resolved=5297`; `roundtrip --require-clean`
`parse accepted 16824 / clean 16824 / mismatched 0`; `cite check
--list-noncompliant` `0 non-compliant citation-looking string(s)`; `cite check`
`checked 18259 citations against cr.txt (eff. 2026-08-07); 0 stale`; the diff
cite audit read 9 sites. The `construction` declaration count is 397 against
the base's 395.

Implementer's pre-refresh artifacts: `cargo fmt --all`; strict all-target Clippy for
`deckmaste_english_v2`; `cargo test -p deckmaste_english_v2 -p xtask`
(`test result: ok. 143 passed; 0 failed`, `test result: ok. 430 passed; 0
failed; 1 ignored`, and every integration/doc suite green); `coverage --check`
(16,824 selected and covered, zero failures); `ambiguity --require-resolved`
(zero unresolved ties, exceptions, and internal failures); and `roundtrip
--require-clean` (16,824 clean, zero mismatches, ambiguities, and internal
failures) all exited zero. Citation checks report 0 non-compliant strings and
0 stale citations; the diff audit reads [CR#115.1], [CR#115.7a] and
[CR#115.7d] against the amendment's claims.

### Deviations and additions

- Exactly two Constructions were added, both required projections; none was
  added or deleted beyond the ticket's letter.
- `AttributiveAdjective::New` is the lexical dependency of the ticket's
  required *choose new targets* acceptance. It accounts for all 53 reviewed
  add-only coverage identities above.
- The Target Verb acceptance is not present, and *a spell that targets* is not
  acceptance for this ticket: the coordinator split it (2026-09-04) to
  `english-v2-target-verb-subject-selection` after the second STOP.
- No `checked by` or `require` names a word, lexeme, construction, or card. No
  dominance edge, selection exception, or form narrowing was added. No scratch
  copy or probe tree will remain at handoff.

glossary gap: Imperative Clause Head Licence — the declared permission for a
Verb's plain Inflectional Form to head an imperative Clause, independently of
the Clause's Finiteness. The gap belongs to the withdrawn Target Verb work and
travels with `english-v2-target-verb-subject-selection`; no landed construction
reads such a feature.

### Review corrections

Landing review (Opus reviewer, 2026-09-04). Probes run against the landed tree:
twelve of the fifty-three newly covered identities inspected with
`english_v2 inspect` (*new* is `vocab:AttributiveAdjective/New` and
*target/targets* is `lexeme:CommonNoun/Target/{singular,plural}` in every one,
never the marker); sixteen target-bearing shapes probed on the default line and
on this tree with `english_v2 probe` and diffed (only the
`DeterminativeSingularSimpleDeterminative` /
`NominalModifierAttributiveAdjectiveModifier` node is replaced by the marker
projection; *any target*, *the target*, *can't be the target of* keep the count
noun; *a spell that targets* fails identically on both trees; *a more target
creature* and *This creature is target.* are rejected on both).

- MEDIUM: the ticket's letter still demanded the *a spell that targets*
  acceptance the landing does not deliver. Fixed: the acceptance list and the
  homograph paragraph mark it SPLIT to
  `english-v2-target-verb-subject-selection` (coordinator, 2026-09-04).
- MEDIUM: both STOP entries were recorded without a resolution. Fixed: each now
  reads resolved by split (coordinator, 2026-09-04) to
  `english-v2-target-verb-subject-selection`, and the "must not be integrated"
  sentence is replaced by the split ruling.
- MEDIUM: the record's numbers carried no measurement stamp. Fixed: a stamp
  line names change `lzyywvxm` (`52369af7`) and its lock `covered` = 16,824.
- MEDIUM: the performance advisory's contention figure was the meaningless
  sandbox-visible `pgrep` count. Fixed: the coordinator-supplied stamp of 4-5
  concurrent codex executors and two reviewers replaces it.
- LOW: the rewrite-decision amendment named the Target Noun and Target Verb by
  paraphrase ("count noun", "verb paradigm") instead of the Oracle English
  glossary terms, and left the Target Verb's undeclared status implicit. Fixed.
- LOW: the amendment cited [CR#115.7a] beside a *choose new targets* witness
  that rule does not govern. Fixed: [CR#115.7a] now carries the change-a-target
  claim and [CR#115.7d] the choose-new-targets claim.
- LOW: commit descriptions read `english v2:`; the house prefix is
  `english-v2:`. Both were re-described.

- Conflict resolution during the second `kata refresh`: the sibling landing
  `english-v2: rename the verb-frame vocabulary across compiler, grammar, and
  engine` renamed `TransitiveFrameTransitivePredicate` to
  `TransitiveLexicalVerbPhraseTransitivePredicate` in the same
  `nominal_grammar.rs` path strings this ticket re-spells, and appended its own
  amendment to the rewrite decision. Both conflicts were resolved by keeping
  both sides: every conflicted path string carries the sibling's frame rename
  and this ticket's marker projection, and the rewrite decision keeps the
  sibling's Verb Frame amendment followed by this ticket's Targeting Marker
  amendment. All gates were re-run on the resolved tree.
- Out-of-scope fix, disclosed: `docs/tickets/done/workbench-turn-parts.md`
  carried an unbracketed cleanup-step rule number that made `cite check
  --list-noncompliant` non-empty on the refreshed base. It now reads
  [CR#514.3a], the exception under which another cleanup step begins, which is
  exactly what the sentence claims; no claim changed. That file is the
  fifteenth file in this diff.

Observed on the default line, since fixed there: an earlier refreshed base left
`crates/deckmaste_lowering/tests/diagnostics.rs` unformatted; the sibling
landing that arrived with the second refresh formatted it, so `cargo fmt --all`
is clean on the final tree.
