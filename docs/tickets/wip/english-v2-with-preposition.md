---
needs: [english-v2-np-postmodifiers, english-v2-closed-class-single-owner, english-v2-adjunct-licence-removal]
---
**Add `with` to `vocab Preposition` and retire the three fused
`with`-postmodifiers.** The inventory has fourteen members (`after` … `under`)
and no `with`, so `"with"` appears as a bare form literal or codec-tail literal
at about eleven sites and cannot head a `PrepositionalPhrase`.
`scalar_qualification` (`"with" measure comparison`),
`degree_scalar_qualification` (`"with" degree measure`) and
`granted_ability_qualified_reference` (`reference "with" granted`) exist only
because of that gap; the general `prepositional_qualified_reference`
postmodifier and its noun-side licence check already carry every other
preposition. This is the unclosed remainder of the Plan 09 taxonomy audit's
HIGH finding that `with/without/by/as/between` cannot form PPs.

Pinned method, per the 2026-09-02 attachment amendment in
`docs/decisions/english-v2-rewrite.md`: `With`'s `PrepositionAttachment` class
is corpus-measured, never asserted, and admissibility is the conjunction of
that class and the complement head's declared licence. `PrepositionalComplement`
needs a measured decision on its non-`Object` complements here (a keyword-line
item, a quoted ability, a scalar measure): ~~add arms only where the census
shows them~~ (superseded by the 2026-09-04 coordinator ruling and the rewrite
decision's attestation-as-provenance rule: declare every Oracle-English
prepositional-complement shape; a zero-witness arm remains), and record the
census in the landing record.

Fences: declaring `With` adjunct-capable by intuition; keeping a fused
`with` construction "for now" beside the PP; a `checked by` guard naming `with`.
Verb-selected `with` (`WithObjectVerb`, `ObjectWithObjectVerb`,
`EnterWithCountersVerb`) is frame data and stays out of scope; it belongs to
`english-v2-frame-selected-prepositions`.

Acceptance: ~~`form_literal_vocab_overlaps` down by the retired `with` literals~~
(superseded by the 2026-09-04 coordinator ruling: retain the live ceiling of
5 and leave no `with` literal outside frame-selected preposition data),
the three fused constructions deleted with their coverage re-spelled through
the PP postmodifier, selection census before/after, byte-exact laws green.
Standard constraints apply.

## Landing record

Implemented on change `srvvykws` after the two 2026-09-04 coordinator
rulings. The ticket's overlap prediction was stale: adding the vocabulary
owner makes any surviving literal an overlap, so the ceiling remains exactly
5. The ticket's corpus-filtered-arm wording was also stale and is superseded
by the attestation-as-provenance ruling in
`docs/decisions/english-v2-rewrite.md`: grammatical complement arms are
declared even at zero witnesses.

`With` is a typed `Preposition` with the corpus-measured
`PostmodifierOnly` attachment class. The selected free-`with` census has
1,256 identities (1,267 shape incidences because eleven identities contain two
shapes): every selected use is a nominal postmodifier, and none is a free
predicate adjunct. Verb-selected `with` remains frame data. Admissibility is
the existing data-driven conjunction: the preposition's attachment class and
the modified nominal head's declared noun-side licence; no added guard names a
word, construction, verb, noun, preposition, or card.

The complement-shape census is provenance, never a filter:

| complement shape | selected `with` identities | example |
| --- | ---: | --- |
| object nominal | 130 | Bushmaster, Coiled Henchman |
| edge-of phrase | 0 | no `with` witness; the existing general arm is retained |
| scalar measure, exact | 38 | Isolate |
| scalar measure, comparison | 451 | Pernicious Deed |
| degree measure | 12 | Press the Enemy |
| granted keyword / keyword-line item | 556 | Shattered Wings |
| quoted ability | 40 | Chocobo Racetrack |
| power/toughness value | 40 | Frogify |

All arms are present regardless of those counts. `ScalarMeasureValue` admits
both an exact threshold and a comparison; `DegreeMeasure`,
`GrantedKeywordLine`, and `PowerToughnessValue` give the other non-object
shapes their own categories. The power/toughness arm admits the two Oracle
forms observed by the style/corpus reading: a fixed pair and “each equal to”
a scalar value. Its structural requirement rejects incomplete or mixed forms.

The original 22-identity retirement census was 11 scalar-measure comparisons
and 11 granted keyword items (object, edge, exact scalar, degree, quoted, and
power/toughness: zero). Every identity is restored through the general PP:

```text
id | card | shape | before -> after
0fffe634e9f0c326390084b6d78bc13494c28f197b22e5473ec7defca0b1dbb0 | Pernicious Deed | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
17cc7d250a98267931037617fe95888534dab820f682b56a0c9cf304878d74ca | Make Your Move | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
19b5b90bccc2207a2f8568d3970bdfe71d6dd30585ad3d41daacd5935106dca5 | Shattered Wings | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
209c5da23cf0f019eacd1e6cfc964db051dbbbbcd3e94e1ee6efb217974fd78c | Obscura Charm | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
220aa154c1a962d2c64974c4ff1243946a3c41eb1a792d22fcc1ff7fbbcd2be5 | Shoot Down | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
28acd05e1f85a9e7ad2b850ae520de6767e934f0f71e346488d9b3a6b1267072 | Zoyowa's Justice | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
4d55242599783e3b6aa4658895593886de8fb471a0d30d5e5a8e52bbdad3ddd8 | Atraxa's Fall | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
5669ae9d87590cffe0eb1acc1e004f1295550519ca78b3d5c0d7f545daeed1f7 | Broken Wings | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
64178de3d33fc3b6c0586dc5e16df0ce1da48da32a19f3319e9b909fa8744527 | Storm, Shaker of Skies | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
71a4a71adbc3ea755f1014d0b6bd3cbd4acb32836bd85f527b9d9cf01f27a4cc | Fragmentize | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
7951d4af00b7ac20c1eee8a4e9dd8ee021f4d11794555a0b498b52aa7fb9db3f | Long Goodbye | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
7a9f73046b9931230539af72b5cfada974741b06e215f2591661f5babbb24895 | Natural State | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
96f57309a5c4e6aaf7c94049171dc21fe7469fa1a240cf21acbf49b2e4e7acf4 | Return to the Earth | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
ac7e1dd3043728ba1ef0f1fb9c8d3fb0a5a84767e9a23eb6891b8b6393b06c59 | Exorcise | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
ba586bddf6ec5b6885c8ead3cb8b8144293ed6aec1070afb4a63423c2707cc40 | Spider Food | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
c3d09c1e2f3c7cd1acbf0b324af772532605c660f1730751edecadfdc1dae668 | March of Otherworldly Light | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
c897364894ecf5618d066c810cb1cf35fe70b1d444f0e4896592b06fb174a93f | Shower of Arrows | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
cc2a19f0c08648c9d7f9c3d87530810bc3cd75f31c727e6e3545063b50309442 | Airship Crash | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
d2b78784abc62fb9636f4a628012d4da4ec45bb499bc8de016ddd63d48c1cc50 | Mutant Chain Reaction | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
dbf80c8e57948be653fd0bda229d2ec42f3e77dbea6786714623d1aa07e51621 | Elven Riders | Keyword | granted_ability_qualified_reference/KeywordLineItem -> prepositional_qualified_reference/With/GrantedKeywordLine
e7f2827a71686367549d3b06f630a95d97eabec65847c45d17d578f9d095c3aa | Eliminate | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
f1e707f790fb216b33a9bf4b635530a8b2c701a14886d3c25e2fc01a7b048f24 | Tempted by the Oriq | ScalarMeasure | scalar_qualified_reference/scalar_qualification -> prepositional_qualified_reference/With/ScalarMeasureValue/Comparison
```

Coverage moved from 16,825 ±0 at the parent tip to 17,055 ±0, with zero drops
and 230 add-only gains. Every gain was read against its Oracle sentence; all
are positive, and every selected analysis routes through
`prepositional_qualified_reference`:

```text
id | card (face) | resolution | selected analysis
0148d3aef6f9ecd5e818baac2e8bf0417ab1e0fabdce4b0b81e2d611301a7585 | Queen Marchesa | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
0256be61ac4aaf04949426df1313d3d6e247e1b79bc3535c0748069d6b7e3002 | Frogify | specificity | prepositional_qualified_reference/With/PowerToughnessValue
029affc11cf695191b39c5c6a4eb233d3aba96a8ac6d51b826a2f7ca2b3aa459 | Snakeform | specificity | prepositional_qualified_reference/With/PowerToughnessValue
038481c2030c227e79513fe6de65710770e8bcff11b103727e91f5c2af4c74bb | Arc Blade | specificity | prepositional_qualified_reference/With/Object
039f36313d24c762a809cd65f322ecd57812311c2ecc0064576ea5036c037f1d | Laid to Rest | unique | prepositional_qualified_reference/With/Object
0782804fabe5cf8db073e65481d6a8cfbbcd152f406c08acae64afd7bcb9f08c | Riders of Rohan | unique | prepositional_qualified_reference/With/GrantedKeywordLine
07b37b4b5edfdd6756f2f4457dc88598dedd39562c1b0ac8119873e212aa6ef2 | Isolate | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
084707a8438463de82e15b5d7214dc35c8d7ac51429d2c5d242ea47e1d849f89 | Mer-Ek Nightblade | unique | prepositional_qualified_reference/With/Object
08e7a2dc300b74fde66ccaec6bb5a992c15641cfc00cff6dcfe69ca0e01d2dc1 | Tribute Mage | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
0a5c6907afd5b476c73140097a162caf9a6333142820445e18d30e7b57582efa | Lizard, Connors's Curse | specificity | prepositional_qualified_reference/With/PowerToughnessValue
0b010196ff3d7fec4e8e98cf45c8c83967c29346c82aa89edbefd8bc51b189ae | Oath of Kaya | unique | prepositional_qualified_reference/With/Object
0bba2172b3ca8ba4015d0bf8f2e9a356bd103b9e0c98890ded6953aaaba6b7cd | Village Pillagers | unique | prepositional_qualified_reference/With/Object
0cb9457a9d9c21b8f85681bb20ce52cb5cfaaadd59ed8745ad62eb16e9ac19b8 | Tempered Veteran | specificity | prepositional_qualified_reference/With/Object
0cc2c033a98a6e91d6263d6f79f018e855b9d270ab304f5ad3eec01b11856342 | I Am Iron Man | specificity | prepositional_qualified_reference/With/PowerToughnessValue
0d52a7a576a0708df420395c6a3f36b538e0f5c9a3f6229474a443dff16b80aa | Urabrask's Anointer | unique | prepositional_qualified_reference/With/Object
0d708f8153533977fddf076095692db3c8da837de26fcf747d735704a1765aa7 | Festering March | specificity | prepositional_qualified_reference/With/Object
0da3825d02c021306bb9ccc0eed1253eb139ca9cd2850beeead9ae5c3dff283d | Experiment Kraj | unique | prepositional_qualified_reference/With/Object
0f4e215bcb71f154699241502b6428686e450ab799a23b875e6d4632e51945de | The Ooze | unique | prepositional_qualified_reference/With/Object
0fe3d4c519b81195e4d127fc662fb0012b7aa3436551785fae9fd5b1608ec9f0 | Scale Up | specificity | prepositional_qualified_reference/With/PowerToughnessValue
108e3c24168bccb99b7c7687a99a3a1593f03f9027d67f0544332ea2624d6f42 | Serpentine Ambush | specificity | prepositional_qualified_reference/With/PowerToughnessValue
1193c38d94b601c2273550fb7c1d5d77e608efc844cd7c10e087347299b53bcf | Fix What's Broken | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
15b7ab35b6b868317506047d7b9da8433e491a9280f231ee356d1f9e5c4b6fb2 | Dueling Coach | specificity | prepositional_qualified_reference/With/Object
1772e3c2f2c78cfbf4dd1aa706cb47c93fbf07bcaef4aa4fbf064f80976d7502 | Abiding Grace | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
1916a44209294ff7e44bffbd90aa8e12b867bceb0d3c9cfbff3c9cf47ae11ced | Mutant's Prey | unique | prepositional_qualified_reference/With/Object
19dc572cd3fcd424f88f60f241d0804cfd1f8535267c15f985405f656150f054 | Oath of Eorl | unique | prepositional_qualified_reference/With/GrantedKeywordLine
1cc5a1058f80f4b266ec4193682c07a1434583e5f9da99b34bb7a74272b0e0a3 | Meddling Mage | unique | prepositional_qualified_reference/With/Object
1cd24d5298f6c2fc46a926e249b7b058829cb16cf45e90d733c39624f1b7a891 | Dragonshift | specificity | prepositional_qualified_reference/With/PowerToughnessValue
1eaea9f11dd1893707d6d85333795113589b215e5130eb1c4769b7903666c8bf | Longshot Squad | unique | prepositional_qualified_reference/With/Object
20e75c617ba80e251a667153c34ee3a8de356f8be3d93bbe413e687f0d5d99a0 | Tuskguard Captain | unique | prepositional_qualified_reference/With/Object
237d08dca72ac40357a2e6a957c0805b4c5ed185592be7e901fed8aa71d3dc0d | Hagra Constrictor | unique | prepositional_qualified_reference/With/Object
24f00453cc194b43f6c72d3afea659889afcc10a95add43a8835dc7c16c4ccf1 | Force of Rage | unique | prepositional_qualified_reference/With/GrantedKeywordLine
28841abe889d25fa7e91dade17d307b989d1b699a1fd26feda7c6e73e9087258 | Ivorytusk Fortress | specificity | prepositional_qualified_reference/With/Object
2a15566bbeae046fcd56ebe4e80a341bf52636feed30f8a5a001c83d9530efee | Chromium, the Mutable | specificity | prepositional_qualified_reference/With/PowerToughnessValue
2a377de398100e3dd3c73b7d2cf58c8523662ee0f0c9ef9d5158de16aec4368a | Twisted Spider-Clone | specificity | prepositional_qualified_reference/With/Object
2b68bc2fedaac1758c6b330001ebf9808d936bc7a189526fb45783faa2ae7d61 | Infernal Kirin | specificity | prepositional_qualified_reference/With/Object
2ee885b9b806b6113d96e815821fe9f92866c9e5d7294448ed12b603f4ccb967 | Fire Nation Salvagers | specificity | prepositional_qualified_reference/With/Object
300839963a5efe2cde20b9ef7f3fe7ae3bd3a090b6e4088963f8fc3291adb160 | Dancing Sword | unique | prepositional_qualified_reference/With/GrantedKeywordLine
30a6b03ab97b279c19c3497da3285d5e087a9a06710a3f64e2bbc3ac12ac03db | Disrupting Shoal | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
31429679f52e0a92c5aeac1b88b8a758310b7258581682054078150d2116ce5d | Polymorphist's Jest | specificity | prepositional_qualified_reference/With/PowerToughnessValue
35098e28ccbaed5128d668a17cfbe8d29df5fe712a17644154aed2e26c88cb46 | Cyclical Evolution | specificity | prepositional_qualified_reference/With/Object
3536423d0e32635e7824316f027618efc239f013a55881ee23ef306217862a32 | Armorcraft Judge | unique | prepositional_qualified_reference/With/Object
35377f321c2ddba7d5078d36ee12b005f65c1f838d2959a2b0b2000ac03d3ec2 | Foundry Hornet | specificity | prepositional_qualified_reference/With/Object
3619fa1081d6a30173267ae7931baeb560cce5d94d6dc720333002d4ba5fa7a1 | Abzan Battle Priest | unique | prepositional_qualified_reference/With/Object
36afba2bfcd178c6a95314fdd91ce50bd2eb024db4041673d5c7a76a733481b1 | Doom's Time Platform | specificity | prepositional_qualified_reference/With/Object
36d6f938c02462b752de1cc205ab26fe4e5167f1d3478e3aab86a9f2c62b46a6 | Metamorphic Blast | specificity | prepositional_qualified_reference/With/PowerToughnessValue
393dd6513c772fc542694033d16a0cea606bc599f1a00f9ec903e9b4ea80d86f | Nogi, Draco-Zealot | specificity | prepositional_qualified_reference/With/PowerToughnessValue
39b92a6f9c4fccbc2725c77d34a144b07c7eaab72572a0967821d7752791c7aa | Bred for the Hunt | unique | prepositional_qualified_reference/With/Object
3b8ecf2007efb6c18d84afa0d175a990ccf21969249c4e6c852f8b940f3b23cd | Heartless Act | unique | prepositional_qualified_reference/With/Object
3ba4e573b550fb63e1bb4e31313c53724a6d36e553b577c8eef3ce96dbf9fe8b | Isperia the Inscrutable | specificity | prepositional_qualified_reference/With/GrantedKeywordLine+Object
3c4c25a78753d95be17201555e8c6baf2cd9a248dd7160a32acff84d32ce0f7b | Lignify | specificity | prepositional_qualified_reference/With/PowerToughnessValue
3cd17451a3f08117f1677b9b07790cf55a04fef5530f69b4309962e2332ba72b | Haliya, Ascendant Cadet | unique | prepositional_qualified_reference/With/Object
3e7c7aef4ef7c639896248864b2606553be2bf7379e01eb93f381657f9cd333d | Rage Forger | unique | prepositional_qualified_reference/With/Object
3e84e502c2bc1315b661e1f46f1948e9f69e82f75819a4c80afa58d124b878e8 | Declaration of Naught | unique | prepositional_qualified_reference/With/Object
4041829285352e02dc144243d09f9e532f10a41a8e363f9103bf4ccfe0e51658 | Lockjaw Snapper | specificity | prepositional_qualified_reference/With/Object
41a5dc877fdc0561883b168ddb86a3f195324f007483cabe4fd737f303d8daa3 | Perilous Forays | specificity | prepositional_qualified_reference/With/Object
42131964ca3f1f0f701042ee1336a0e6502ce6dd067984d70fa48032e67bba35 | Tributary Instructor | unique | prepositional_qualified_reference/With/Object
42bd64562ee788f52c902d7835faa3322432cb0e8075d0e82d1046f7c2324a14 | Ichorplate Golem | specificity | prepositional_qualified_reference/With/Object
4602604a0f6ab146846f5691405b243c0935f3e0935b17b67b432fbc5c1732c7 | The Thirteenth Doctor | specificity | prepositional_qualified_reference/With/Object
4666a299f4b2cec8e90b2e3874ca3031d2ebb498ce4c4a37eeaf82a32218c69c | Stolen by the Fae | unique | prepositional_qualified_reference/With/GrantedKeywordLine+ScalarMeasure/Exact
4aa5a72024aadeb87570ba141d206e0021fa9c2a3f7eed74647cd06195f815ad | Skyclave Shadowcat | unique | prepositional_qualified_reference/With/Object
4b3a5a22bd252eb307fe10f744c3e73d5b4fc4143c0377f8416bc1a259737ad9 | Desperate Research | unique | prepositional_qualified_reference/With/Object
4d6b1c5d08e135d5b82b0e9e4a561ce15c4225e8e0f88744380d140036d0a84d | Trade Route Envoy | unique | prepositional_qualified_reference/With/Object
50f9f7a8a3edb81313b399d25912291919f0555050bb6d39339a19d43e2b8223 | Tenured Inkcaster | unique | prepositional_qualified_reference/With/Object
52977834b75d8bf7ebce6959dc45b204561c3d24e73fb5b2cf1a18f11bdca372 | Trollbred Guardian | unique | prepositional_qualified_reference/With/Object
544bdb2ce2423992f7c6855a2dd4db6f3b958ec1238864fd5146993cfbbe21b5 | Micromancer | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
569eb3c2a303f811e48e9ba6270ab760c731825f8bd00826038bb49bd655e848 | Daring Piracy | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
59404691b54596ed72d31a76a654c920ce4d9e6c038b87cc1dbc81261e7b7d53 | Prehistoric Turtlesaurus | unique | prepositional_qualified_reference/With/Object
5ccf03b86a4bfac702b09fa4c7052201dc667fe48ec502a7282d2118015aec93 | Crime // Punishment (Punishment) | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
5efb1d8633d284855d0d6a5badafc249f38adf87e93df0e7ad8adc9f897f67ea | Zegana, Utopian Speaker | specificity | prepositional_qualified_reference/With/Object
5f9c17f1683d1d61da108064422a3887bb15adf6751ca672a11f10ea5335b631 | Hearth Kami | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
5fd4e6b493518fbb57699ce979a37df985530cd247612b9a545568a2fa36f5e7 | Oran-Rief Ooze | specificity | prepositional_qualified_reference/With/Object
606513cb875817c92dd2fee19749abe260de4fe965199bb3510e9e646316274a | Patron of the Valiant | specificity | prepositional_qualified_reference/With/Object
60c2873abdb5d985c44d04282fb82c57fa880a763171528157c593b30b6107d5 | Thunderheads | unique | prepositional_qualified_reference/With/GrantedKeywordLine
63a2b6227571aefa6987ffa83f9126030eedcad2d0bdc1d4744bce7a0de77b04 | Opal Guardian | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
64fa82253409255153410e5ac166c0d904ab627c8c2a86ae968f2ce6e20b122c | Silverquill Silencer | unique | prepositional_qualified_reference/With/Object
652fd0ab649b4cb1c93066b2adfc85d68264deb32e5291699887aaebbcd4d57a | Startling Development | specificity | prepositional_qualified_reference/With/PowerToughnessValue
6573b00a1a34eb1d069557790c4a736756856b41d4866aacfc3adb3f62667d4e | Namor, Scourge of the Seas | specificity | prepositional_qualified_reference/With/Object
65c257c3a347fa93bef279e408282903417df2a50ea94ff32ac5dccc4b3e57b4 | Skatewing Spy | unique | prepositional_qualified_reference/With/Object
6661481bf6039c35210143d8dbac7c37634b1ce6b7ab1010cf4919865d22baf0 | Avatar of the Resolute | unique | prepositional_qualified_reference/With/Object
66ca4465149fb9655a88a5b792e27796099f2b15f920ec69cc8689722062ade0 | Animate Artifact | unique | prepositional_qualified_reference/With/PowerToughnessValue
677d3f84f348f7f05435dfe1aaa902e58ea5269e6a866198de350b450c17d677 | Relentless Dead | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
68c8f8b98d0b5e7c21e3d2fde8c682cba0bb116a36c778c2eafa28c48e5c0489 | Conjurer's Ban | unique | prepositional_qualified_reference/With/Object
6b330b78c3f5582080a36eb6ed48936b18a4f46482e3501e45babf8bebcf5b55 | Duskana, the Rage Mother | specificity | prepositional_qualified_reference/With/PowerToughnessValue
6b4d2f3a401c3ce0d460883a6a3d447a6702c70633d7d639f4bc12a2f5cd1be5 | Droning Bureaucrats | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
6bc5d24f4d6e2756b7e19a5a623e702299fecb545675b44e30550e9c5ee787bb | Celestial Kirin | specificity | prepositional_qualified_reference/With/Object
6c3386b82d6f9e1d03958df68e844365658f6d5dd7ec620288b6fb0ccccf8ea5 | Bramblewood Paragon | unique | prepositional_qualified_reference/With/Object
6cc35d220ef46286978b00cd888b7969bb25568599bb39ab1f538c47652d8016 | Opal Archangel | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
6f15a23e1624c3fd5ba1ffa6355897457f4c784084991892f35dfc48d3f646b4 | Suspended Sentence | specificity | prepositional_qualified_reference/With/Object
6f670757630a1d6bf650cc427c9db71fcdcf35adad1ec11d81245767a9994832 | Inspiring Paladin | specificity | prepositional_qualified_reference/With/Object
7007b9083ae0e5858657a505b215851a3a3f46e1c88e4b89fff39f595661318b | League Guildmage | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
7176d9d0a430ccf5145109a7d391be904e3e8f03aeed6a95b3e8b65026238837 | Spell Snare | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
7221c1c6638e1fb4a99205f8421da56e561ca5c722f99044d724adfb5dcf3550 | Vigean Graftmage | unique | prepositional_qualified_reference/With/Object
733ed3cc211001b724950028833476e4b220af985dab194a249d4e9fce99e2b4 | Molder | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
737ca234f21169d08742788f1bf32baa83de026bc574413c73f12e242a5421d4 | Ray Fillet, Wave Warrior | unique | prepositional_qualified_reference/With/Object
73bd19ccf60ade46f050020d6fcb08652fd749de82322edc977d3c7ad585be45 | Spark Rupture | unique | prepositional_qualified_reference/With/Object+PowerToughnessValue
78627407d44451511d9abc9af01bfa24e51501bb44e8123b93eb7a4e597e458d | Jiang Yanggu, Wildcrafter | unique | prepositional_qualified_reference/With/Object
789f984b2718fc584b0d38e281af96b6fbf46aae187ec6d2173efd8af071b47f | The Scorpion God | unique | prepositional_qualified_reference/With/Object
78f6175842749a4439062ca142fdc12c98bf02282a6f065e2cd30c530388875c | Mercurial Transformation | specificity | prepositional_qualified_reference/With/PowerToughnessValue
793c83aff79c9387a3e087b6e2c1d7fc65d87687a7276b7d5ffd76dbc60c81b6 | Dragon Broodmother | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
79bac0bd6b09f7f17aecf994f232247cdfce3f224961b332a62bbdfbdc0b1220 | Entrancing Melody | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
7a1ae25c82ffb57c94b6a092b54fbaa9cab8517f2126a8cd81ee9c84f6d023c6 | Spider-Man No More | specificity | prepositional_qualified_reference/With/PowerToughnessValue
7a35bfe1fd4c8e6d19c81bcf32b644737d67d95705e0f657a027343d36e77cbe | Curse of Chaos | unique | prepositional_qualified_reference/With/Object
7bbe107a12397763530cc4957078b3876f4184f4b3e895c438edfac7fb278195 | Crowned Ceratok | unique | prepositional_qualified_reference/With/Object
7bc64b9f98d3dc25c68a7cbd4674b5b43ead0e803919012ce45c77b4f3504b52 | Deepfire Elemental | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
7c881c88d9049103739c19b0e4e05d90bb2dbbe58767d0911f223e2e7556a768 | Marchesa, the Black Rose | unique | prepositional_qualified_reference/With/Object
7c95ae690baf506029432302f3dd4a635ca22f11309afea9bb541ee92118db5f | Venus, Torn Between Worlds | specificity | prepositional_qualified_reference/With/Object
7ccd910f48275e7580c96edff3b134e0ee150c044dc93117df426abcb64b828a | Necroskitter | unique | prepositional_qualified_reference/With/Object
7cde8a4114557d5bf102e23c8aa3b27089a7ea76cfb30188d3641563e5b9cfde | Retro-Mutation | specificity | prepositional_qualified_reference/With/PowerToughnessValue
7f6fddc4326b5de084fcfc54e12eca91a25fce12aaa88ef42deef71963dbd24e | Ichthyomorphosis | specificity | prepositional_qualified_reference/With/PowerToughnessValue
805350309161bbc39825a7b2dc02f37f2b6c821f56883c6fddd6fc98fede90f6 | Rise from the Wreck | specificity | prepositional_qualified_reference/With/Object
846e8fe5318ccae911e8b17f7588e211b30570701de36b2d14c1ab250e456204 | Trickster's Elk | specificity | prepositional_qualified_reference/With/PowerToughnessValue
8492941dec7095ade8d82bd78a6ab01974ad9755c3d8d5cdb5e4c8407247b13b | Cinderslash Ravager | unique | prepositional_qualified_reference/With/Object
87a191d2a192ba59df66e6fe81736c8079b257a66a090d71d33db8d36b97d20f | Bess, Soul Nourisher | specificity | prepositional_qualified_reference/With/PowerToughnessValue
8829e2419ccc56b46d92cb39955db6a91e87128b758ed0c4a7405315014bd8d2 | Abzan Falconer | unique | prepositional_qualified_reference/With/Object
8d76d2c15a27c9597cd99e5042c0b6926938f56b5c6e57968c115605ee77908e | Hunter of Eyeblights | unique | prepositional_qualified_reference/With/Object
8e6e22d5263a52d72f8594e0613eac68f3dd53f9bd794b3dd8882e7478740fc1 | Akoum Stonewaker | unique | prepositional_qualified_reference/With/GrantedKeywordLine
8ee55eeecb78c5ab25bd31fabe92ec873d40b393e347d006c5a7ef1edafb549e | Training Regimen | specificity | prepositional_qualified_reference/With/Object
8ffda2c7fd4b0c1dcee7f9fd0ce5105db2ff2c61f12a1556c28009dfd69a73e3 | Boseiju, Who Endures | specificity | prepositional_qualified_reference/With/Object
9003700b259156f5b3944f01e64d25065133aeb205a48b2be543e3d8d391b9b9 | Knollspine Invocation | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
9026b8260ca68277918e25afb26edd91666f061f55e7f3232e211560a187c545 | Elemental Eruption | unique | prepositional_qualified_reference/With/GrantedKeywordLine
911dd69826351e584cbb2d78c22262f193746515d8ab00241cc64703d2218906 | Mordenkainen's Polymorph | specificity | prepositional_qualified_reference/With/PowerToughnessValue
912be9bbdf4702715188ffe327a32fc638c84a4e0f4fc7e9251d52b89fd8d0db | Hornet Queen | unique | prepositional_qualified_reference/With/GrantedKeywordLine
944c6df6e2e3b55682448179146f89d190fc50c13343feb2d4ba3398e0e90a77 | Great Ugly-Looking Goblin // Clap! Snap! (Great Ugly-Looking Goblin) | unique | prepositional_qualified_reference/With/Object
94b101fc05a240f28e10c11fc092180d05bb7aa634a447d70df11898f2bcc5ec | Sporeback Troll | unique | prepositional_qualified_reference/With/Object
94e9d57503c2f379616dfe9fa113e7520ef0abe15caf1c437eb887b826d422dd | Cleopatra, Exiled Pharaoh | specificity | prepositional_qualified_reference/With/Object
9540b54b1f6d78eb0dccc3e912071b0bef9ccb4168ccf8d2691477f1d74a5305 | Sickening Shoal | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
95f9a3f6d2f6aaab4d380a1b6aaf8ac18c53bc0c8b989767563cf966cfcd0435 | Hazardous Conditions | unique | prepositional_qualified_reference/With/Object
9622cd66d139867e729b12d8e4900e2bcba122f9cd1a456894c25bc33ba67d79 | Chronicler of Heroes | unique | prepositional_qualified_reference/With/Object
9663bfa6899cfd9f921aac22f2d9e831ba431722511f6794c5f29cd36567dbe4 | Booby Trap | specificity | prepositional_qualified_reference/With/Object
97b48453671a13960bb8334ac995b08b023fa6c6e46f05d1c4a375c3eb03fc16 | Nahiri's Sacrifice | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
98615b23932e45110800acff7fba2719172cffe78622c5946bf4e140ea7afe95 | Simic Basilisk | unique | prepositional_qualified_reference/With/Object
988f0b0b85863c1ea05281403931f2da6d041f6c55e103e3853dc91818e4afa1 | Trophy Mage | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
992999c3dcba011794b31f449249aa0447ef73ffba348cf89b92d5c32eb31490 | Jenson Carthalion, Druid Exile | unique | prepositional_qualified_reference/With/GrantedKeywordLine
99c750b872e88887a7ab0f0943b22048a959f2e5050580672801ba77f43aaf16 | Wolfcaller's Howl | specificity | prepositional_qualified_reference/With/Object
9be89072ff5eca76bcf9b3f6d59933ace2b22b4361540473262dc7c2fb925a86 | Hamza, Guardian of Arashin | unique | prepositional_qualified_reference/With/Object
9cdfa39e3e13a617dfd933c5a818fc942f31d0e39aef7f5bd72bf8813fca0e01 | Rishkar, Peema Renegade | unique | prepositional_qualified_reference/With/Object
9d3454adf9688532fa67c8b52d61a930466ec604fbd0ccfec77e2b5de5958ba8 | Dance of the Skywise | specificity | prepositional_qualified_reference/With/PowerToughnessValue
9d86eee391a8d14a55769e861c2f5903caadda6789dc7ade6781ba89c48b053f | Crumbling Ashes | specificity | prepositional_qualified_reference/With/Object
9dfbe23fcdcc0816002504affff9a45b689531a93e83c14b4bd24c79d6bc42e3 | Eaten by Piranhas | specificity | prepositional_qualified_reference/With/PowerToughnessValue
9e1753f668a18c10ad1ffae47712e78f14d3ef1191685dc7b9063c850bf36aed | Spitting Dilophosaurus | unique | prepositional_qualified_reference/With/Object
9fe0443f0b123392302d6eb78f70e79cd373435d04b66ecccf5d0477f7db9744 | Failure // Comply (Comply) | unique | prepositional_qualified_reference/With/Object
a094a9843efb9a427416ff95b6f1e204d62d947960f02cd66f4c46d7ef9b81e0 | Swarm Shambler | specificity | prepositional_qualified_reference/With/Object
a384ac7a004c706064ba355c1a8893dd6aa1f1d5abe5d6017c19dd7ea19df0c9 | Postmortem Lunge | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
a63480812acf6a00c051595d5624cd69216488da6ef98c4c264c89127a0f6433 | Mind Transfer Protocol | specificity | prepositional_qualified_reference/With/PowerToughnessValue
a7d7c5962b0ad7e8f1c2ff17c12825bdca70b4cfb692c55a0bb876eef1e2b484 | Kenrith's Transformation | specificity | prepositional_qualified_reference/With/PowerToughnessValue
a7dc6f0c968a01f5eb5d31437730dbb909ab6332ba449c6005af88ea9607b9b4 | Amphibian Downpour | specificity | prepositional_qualified_reference/With/PowerToughnessValue
a8242394e24bf4f3491fc7b343a3c5c59f7ae5b2327c320c756a7bb25c5a479e | Valduk, Keeper of the Flame | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
a8715424891901704810c5b3e9fd34b9881c2061f5ab430038c5355881780d2d | Byrke, Long Ear of the Law | unique | prepositional_qualified_reference/With/Object
a8b885c7e0096519f8361dbd147ea9d5a9af6574d4618e9af78a8d12a72c451c | Nourishing Shoal | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
a987341080a5398766b2d8e2622131129c0c4c9f11214d4b0d4548788ca954c6 | Hornet Nest | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
aabafde69b13f842df42d122aa30493b7f82de4cdc1f229a52b7397a410cc155 | Nervous Gardener | specificity | prepositional_qualified_reference/With/Object
ac644b3fbcf4665c85a854406cae06f29f3fc61d6e871b91e9ea9673b2285798 | Razorfin Abolisher | unique | prepositional_qualified_reference/With/Object
acee425732e3073698e3409ac8f5ae4ae562f1eb2fa2404894f6d8f0978493f4 | Plaguebearer | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
ad062e41078b5148c4712e5762b92e3f99626a76dbc0d3fadd27c4392648d43e | Taigam, Master Opportunist | specificity | prepositional_qualified_reference/With/Object
af96c97a1a8e6529afff5afaa4464218ca12dd79b0ac852b978b7ad00b2d131a | Reprobation | specificity | prepositional_qualified_reference/With/PowerToughnessValue
b0058587f071c0f7e5b8aae9021ad0e060f5c9f50cfc6ea467b74f607867cc7b | Gorilla Shaman | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
b07cdd773f2ea4b19082ae7e7a2dde81097b29e508c6e65c4386a9a0b9dd027b | Muraganda Petroglyphs | unique | prepositional_qualified_reference/With/Object
b0fd0ed0bd2e45c818d36b38cbd3fe0af36ba8efc639d29724cbb317907e369f | Voracious Bibliophile | unique | prepositional_qualified_reference/With/Object
b267b58f8d4859a39ac8731beee26ebde831440b57fadbea75575436c2141ee6 | Bushmaster, Coiled Henchman | unique | prepositional_qualified_reference/With/Object
b75a762c138b04233b7b7843ffa123192d47f5c045d7ed99cf705a3990a17dd5 | Exava, Rakdos Blood Witch | unique | prepositional_qualified_reference/With/Object
b930999513f8bc5551251824c9a2132f4d8d580df3a63860ca26412ce0dcb2d0 | Skyfire Kirin | specificity | prepositional_qualified_reference/With/Object
ba425b8249982d313f6251e91c8754b077ee16585be8218ff18d834b9a8c3c20 | Blazing Shoal | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
bae817a9e634e6a7646ef2b4ca507926ea55bd9dd6de1001cfbac97e047d35f9 | Gift of Tusks | specificity | prepositional_qualified_reference/With/PowerToughnessValue
bbd4b2937e21a1f1ec4d7dce96bb330a9f805643a41cff92b9b946b4f1240f86 | Inspiring Refrain | specificity | prepositional_qualified_reference/With/Object
bbf456057d2d58aa3c0b1fe97300702c5f86bfe99059eabd55aad38b46fc8011 | Spell Blast | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
bd1b6297c83404c2376c49eac48f6418022c1dfd6e276b263cf19cb3798099ce | Sorin the Mirthless | specificity | prepositional_qualified_reference/With/GrantedKeywordLine
bf879912bfc908965c4c0f680c735f0bf16e36123333c87f3a6ffa8542777961 | Detonate | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
bfe347b2fd8803818e191d4edbb99bbf234fad871452712090b8401eeace2923 | Curse of Silence | specificity | prepositional_qualified_reference/With/Object
c00d7961fed82e9ca3e856a2030a26fa704a21ccfc933de436a43e8b37171469 | Damning Verdict | unique | prepositional_qualified_reference/With/Object
c1327317649681d92dfb2f2dda7f342f4cdcad5f4a77486282baf908f39a45a6 | Drider | unique | prepositional_qualified_reference/With/GrantedKeywordLine
c25bffa15f8d481df78b6b170b468638f2c590107b48fea5a49bb93187ab313a | Metathran Aerostat | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
c28cbb94ca8082419385aabd9dc9fb71b71ec364cc29033c4368613580c9d3fd | Meltstrider Eulogist | unique | prepositional_qualified_reference/With/Object
c35a8ee9561bfdbd57a8fa083a038f47cfc47a8126596f78c5ebcdeeadf4321d | Sapphire Drake | unique | prepositional_qualified_reference/With/Object
c38322384df89b730ff46e52b3d94a404b6bcc5f84fed139bf13c06336e92508 | Badgermole | unique | prepositional_qualified_reference/With/Object
c43b477ae00acb19e9f39df7019ff2e0c9a8c426d631792c3848f28e5c30744c | Cenn's Tactician | unique | prepositional_qualified_reference/With/Object
c4486507e5627a8c6a33fd550732293044afd98d8d1d990556e23f24a2379d76 | Rampage of the Valkyries | unique | prepositional_qualified_reference/With/GrantedKeywordLine
c4fad6b3515e9ead35371fa090a5c88576605e77aee96ca8602fa9b393d619bd | Immortal Servitude | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
c51e684f38f6eff22dbfc9da20a3b106e19721e64388e913161387c8841d46d4 | Warden of the First Tree | specificity | prepositional_qualified_reference/With/GrantedKeywordLine+PowerToughnessValue
c66dccab39ba30890cca5b6b33c353a96e8a1a8d8de65e739ff774151e20169d | Spell Burst | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
c7ddce9d85ab045ca1a4c107aec42de511bb045a607583d92517e5c7c75332bd | Starport Security | unique | prepositional_qualified_reference/With/Object
c803652a44b41246d9c6d2c332bdfe3a8350d9fa82efa555f1797bafd05f2a0d | Dauntless Dismantler | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
c910041d437a811dfcd184aa66fc0f3ddf4a2605bbc5e5e398ae50d0025d5f5c | Cabal Therapy | unique | prepositional_qualified_reference/With/Object
ca2f0bf78e843693a14c288282cfdda7014c7395c8c4ce638c1d0534ac11211c | Floodpits Drowner | specificity | prepositional_qualified_reference/With/Object
cad5b3f848314b14abc1484e28d91c16f4629ef4d68fc9dca18ad69b34cf635b | Mental Misstep | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
cb4bccad70dd1344ab7462c7416965a95778e4c70017bbd916c3d439bb29429c | Rigo, Streetwise Mentor | specificity | prepositional_qualified_reference/With/Object+ScalarMeasure/Comparison
cb848fbb9e9ae1d315b70f6219b665dd7b6d18b9995db112abf9bda18c5b98eb | The Mycosynth Gardens | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
ceceeef0519973cf6cb3da8bf954ab3822388096129b99f956847a9739804130 | Xenic Poltergeist | unique | prepositional_qualified_reference/With/PowerToughnessValue
cfac7357c9dab252d1bc1c5a4f509978b3b1206ee9351412f79d006174e1d47c | Alharu, Solemn Ritualist | unique | prepositional_qualified_reference/With/GrantedKeywordLine+Object
cfdaa0265b470456167282bef6f0194dde6a52f46ddf5d31a6dbc980b2e61bb6 | Repeal | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
d09b84b9f583c73b7a9399e6ac6cb6963a86e3b2566df5b89ee6a4cabe0ac1dd | Oona's Blackguard | unique | prepositional_qualified_reference/With/Object
d10f59aff0b2b85b47254524cb19d97e8c8080e1b6e913d7393e89476977756a | Disembowel | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
d4c90ad6dc40f981de562eaa4b6b85f2e8533e4e3bedad44c94b6ca23e7e0ada | Warrant // Warden (Warden) | unique | prepositional_qualified_reference/With/GrantedKeywordLine
d76b569ea4dc0bf2311ed4918cd33554d2ce74238453072fb225c25b9e5ae819 | The Locust God | unique | prepositional_qualified_reference/With/GrantedKeywordLine
d91e75b7e7eb5565c9aeb25faec0b6e27ca05e8fc1bd31b84a3f57fcd015a7a8 | Circle of the Moon Druid | specificity | prepositional_qualified_reference/With/PowerToughnessValue
d940a5a8da18621b05cf7e7b07f3cd723619f1f0339a53bd20a8c664dd98cef1 | Kaervek's Purge | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
d9486a7b8e1b0ce95027e68c4355bd8890e63c5026e07cca08d42ac8f94927d5 | Tesak, Judith's Hellhound | unique | prepositional_qualified_reference/With/Object
da4476511abe0e208eca5218bb240a9daa4a0a9f31e8a861a332b7b2a468463e | Moonsilver Key | specificity | prepositional_qualified_reference/With/Object
db83e6b05e1f195d96c65652cbec6debf5e0a734b223102b2899310f4920c6f9 | Ainok Bond-Kin | unique | prepositional_qualified_reference/With/Object
db8e70034d08ff8a61ddf657b6a626a1aac8cbb0f779bd8b1e2f0998d0a94520 | Curse of Shallow Graves | unique | prepositional_qualified_reference/With/Object
dc07c3fbffb5e35bf3762a06d5d339957d1ecf4e0139847053a8bd16d88d6e99 | Veiled Sentry | specificity | prepositional_qualified_reference/With/PowerToughnessValue
de37ca9dfaabda64de17366247dd57067cb3289e599705fe1bc6b4bdb739cc02 | Turn // Burn (Turn) | specificity | prepositional_qualified_reference/With/PowerToughnessValue
de6eecc4e4bb15a9cd8107018561b036c08713da76bed35db13b38c3eb75b4e4 | Oko, Thief of Crowns | specificity | prepositional_qualified_reference/With/PowerToughnessValue+ScalarMeasure/Comparison
dffddb410c055bfb63e21888e23cb16bc8e72ddb2f434bd97a20187673846579 | Webstrike Elite | unique | prepositional_qualified_reference/With/ScalarMeasure/Exact
e09c3045431e9a86ce4598478db798545731732779c55aecb22132b6433a1586 | Sphinx of the Chimes | unique | prepositional_qualified_reference/With/Object
e0a6883e514cfc9f4a739c1281c25985041e899fb77bc068bdcfbd2704aff562 | Reality Strobe | specificity | prepositional_qualified_reference/With/Object
e2790839e1975f784c96366840b64b163ae72c5a7350f1a6ed871e3951d89ce8 | Delta Bloodflies | specificity | prepositional_qualified_reference/With/Object
e3da21bdad24c8c35a590a239bc4b2ab43520ffae2c09323048f9836787e5a9a | Duskshell Crawler | unique | prepositional_qualified_reference/With/Object
e3e6b24cf90ea60a14ea5d407f02d846671e5122b1b0cd0405fe62e0685f10e2 | Liliana, Death Wielder | unique | prepositional_qualified_reference/With/Object
e476b6fae4f3892cdd79e35fcf325fd04e52e017ba6aa3e1e408e5f6f33b933b | The Legend of Roku // Avatar Roku (Avatar Roku) | unique | prepositional_qualified_reference/With/GrantedKeywordLine
e6087fd5aed4309532bf55a673e8f1f25982f12a8995d5b7e5beb8825171ec45 | Kulrath Knight | specificity | prepositional_qualified_reference/With/Object
e8c51426c678c9ab47d10cf815ae0d6c1f289d7078bf4980d05592b815f862d0 | High Sentinels of Arashin | unique | prepositional_qualified_reference/With/Object
ec94927408d04392284a43e3cd185b45bad1648ad53b396c591f1cd3c20cc931 | Chronomantic Escape | specificity | prepositional_qualified_reference/With/Object
ecdb79280ac569d562ea21ea65ed06b11d70178d9a1aa11c23d6685e8c254812 | March of the Machines | unique | prepositional_qualified_reference/With/PowerToughnessValue
ecff38e93bcbc756896dcd8b23d6e4bf6afa60d1127dd282463c9b0560b36964 | Oil-Gorger Troll | unique | prepositional_qualified_reference/With/Object
ee049f44f6cc228dbb392273a55602617c5c4a783a800a0a9b9053b58d8c766c | Kuldotha Cackler | unique | prepositional_qualified_reference/With/Object
efe81107e11a5623cc5c756bed31bc51b82c1cf6a423b31b9a5b6ff309da2d99 | Turn to Frog | specificity | prepositional_qualified_reference/With/PowerToughnessValue
effb1d60df12b47db3066f7ef17ab4c86d920fa338ed5afc3966119f9cae5ea7 | Valkyrie's Sword | unique | prepositional_qualified_reference/With/GrantedKeywordLine
f01b14b39ba4276088ce1d9d5172d3df919b6e2314225ebf0d48d5fc4857beec | Resonance Technician | specificity | prepositional_qualified_reference/With/ScalarMeasure/Exact
f01f24b4d719d13281df4694ae9a3618ddc996ca9d4cd67f987723fb434dfd34 | Iron Man, Futurist Paragon | specificity | prepositional_qualified_reference/With/PowerToughnessValue
f02a6922f39ca851a61e71a507da27a9505b53316d6108a44d8de732b85e7982 | Herald of Secret Streams | unique | prepositional_qualified_reference/With/Object
f0e5817d19afbfba378f91685070d416fa5b48f1a16bbcbae7aaf6334b3fa522 | Rayblade Trooper | unique | prepositional_qualified_reference/With/Object
f3c25e4b12ec70973aa201a08db8e164afde27da931e21c09e0151e8eae04d34 | Suit Up | specificity | prepositional_qualified_reference/With/PowerToughnessValue
f4862fef5e42112dd61d3ec19e5a68f141992bdcff623bc64771c69ab7abd546 | Dai Li Agents | unique | prepositional_qualified_reference/With/Object
f522d031622ee21cbeccd76ae339dc0bf18dca98e439fcf259482f3f4ebe453c | Pridemalkin | unique | prepositional_qualified_reference/With/Object
f56f765b522ca2204f322c1aaaffeeee0863923063aa66cec2e96d6a9f2a0659 | Yathan Tombguard | unique | prepositional_qualified_reference/With/Object
fa5fcf5708b85f514b84b10948cc0cc670fc7526c36331ffe798acdcf1062f9a | Nevermore | unique | prepositional_qualified_reference/With/Object
faaff79b0835c308b034c8248000e68092f27f868e8acdd8af6413a0f61dad9a | Wood Sage | unique | prepositional_qualified_reference/With/Object
fab759aed5f419b601da9502efdad1c663026ce30a55a9fa43a08da6c33f26b5 | Celestial Regulator | specificity | prepositional_qualified_reference/With/Object
fb6cb18bda0d8ca6c18a17b95305a1895cd4f19b5b6da38ca83d3e87cf6d2bf0 | Lifecrafter's Gift | specificity | prepositional_qualified_reference/With/Object
ffae45b7eda972fb205fc3cee58e4181f1e6e94ca079419d8f2012023fec68c5 | Drix Fatemaker | unique | prepositional_qualified_reference/With/Object
```

The parent-tip selection census was 16,825 selected (13,328 unique, 3,497
specificity, 0 exception, 0 ties) and the feature census was 17,055 selected
(13,455 unique, 3,600 specificity, 0 exception, 0 ties). Among the 16,825
parent-selected identities, 1,062 selected construction paths changed:
783 unique→unique, 258 specificity→specificity, 12 unique→specificity, and
9 specificity→unique. Of those, 1,026 are the intended fused-`with` to
`prepositional_qualified_reference` re-spelling; the remaining 36 are the
same keyword-subject analysis after folding the separate bare/qualified
products into one optional-modifier product. All 21 resolution-mode changes
were inspected: the 12 unique→specificity identities are Claws of Wirewood,
Whirling Catapult, Ifh-Bíff Efreet, Howling Gale, Mascot Exhibition,
Squallmonger, Forbidden Friendship, Hurricane, Borrowing the East Wind,
Squall Line, Rockcaster Platoon, and Cloudthresher; the 9 specificity→unique
identities are Coral Colony, Zoyowa's Justice, Doorkeeper, The Boulder, Ready
to Rumble, Radiant, Archangel, Arabella, Abandoned Doll, Godtoucher, Clip
Wings, and Run Afoul. Each still selects the intended low-attached nominal PP.

Construction count is 391 → 388, exactly −3. Seven constructions were removed
(the four direct scalar/granted layers plus the two nested keyword-subject
`with` modifiers and the redundant qualified keyword-subject product), while
four complement-shape constructions were added.

Deviations and additions:

- Added exact scalar-measure and fixed/equality power/toughness complement
  coverage because the Oracle corpus/style reading exhibits both shapes; the
  corrected ruling requires grammatical arms rather than only the ticket's
  examples.
- Added `GrantedKeywordLine` so coordinated “trample and haste” complements
  are one English constituent, while a single keyword-line item remains the
  same category value.
- Deleted the nested `keyword_scalar_subject_modifier` and
  `keyword_with_subject_modifier` fused paths so no fused `with`
  construction survives.
- Folded `qualified_bare_keyword_subject` into
  `bare_keyword_subject` as an optional modifier. This preserves the same
  subject language while satisfying the exact net −3 construction count.
- Converted the Clash and Exchange generated frame stubs from raw
  `Literal("with")` atoms to `Lex("Preposition", "With")`. These are still
  frame-selected preposition data; the conversion is required by the
  single-owner and overlap rulings.
- Recorded the coordinator's 2026-09-04 attestation-as-provenance ruling in
  the rewrite decision because the refreshed file lacked the ruling that this
  ticket was directed to cite.

Assurance: restored 22 corpus identities; re-spelled 3 existing tests and 2
fixture helpers; ignored 0; added 7 positive complement examples and 2
negative power/toughness structural examples; removed 0 tests. The coverage
lock contains only the 230 audited add-only identities.

The post-refresh gates are green: formatting; strict clippy for
`deckmaste_english_v2` and `xtask`; package tests; coverage with zero drops,
zero ties, zero ownership failures, and exactly 5 overlaps; ambiguity with
zero ties; and both byte-exact laws. No construction-core emitter or CR
citation changed, so workspace tests and cite gates are not in scope. With 8
workers, coverage took 108.928 s at 119,300 ns/B under host load
5.42/8.60/8.92; ambiguity took 106.404 s at 123,024 ns/B under load
7.95/8.53/8.85; and roundtrip took 113.332 s at 134,233 ns/B under load
6.81/8.10/8.65. These exceed the quiet-host advisory under shared-host
contention and are reported, not treated as a STOP.

STOP: none. Glossary gap: none. Decision wanted: none.
