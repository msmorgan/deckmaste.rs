---
needs: []
---
`once each turn` is not a Frequency Predicate Adjunct. `Draw a card once each
turn.` fails without any focus adverb, so the family is independent of
`english-v2-tail-restrictive-focus-adverb` (done, 2026-09-05), which measured
101 corpus units of the shape `Activate only once each turn.` /
`Do this only once each turn.` and left every one of them a parse failure with
its 2026-09-05 exclusion recorded.

Defect. `vocab FrequencyAdverb { Once, Twice }` and
`frequency_predicate_adjunct` (`crates/deckmaste_english_v2/src/constructions.rs`)
admit the bare adverb alone. The attested adverbial is the adverb qualified by
a distributive temporal phrase — `once each turn`, `twice each turn`,
`once during each of your turns` — and neither the qualification nor the
distributive complement has a derivation. `FrequencyReference` covers the
comparative shapes (`more than once`) but not this one.

Shape. Give the Frequency Predicate Adjunct a general qualified member whose
complement is the existing distributive/temporal machinery, never a member per
attested surface and never a form literal spelling `turn`. The restrictive
focus adverb already composes with any Predicate Adjunct member, so
`only once each turn` needs no focus work: the 101 units select as soon as the
unfocused adverbial derives.

Acceptance. `Draw a card once each turn.` and `Activate only once each turn.`
select; the coverage delta names the units gained from the 101-unit block;
no construction, `require`, or `checked by` names an adverb, a noun, or a card.
Standard constraints apply.

## Landing record

Measured on feature change `mkvwwvzuwytwyrrupsokssqqvomvqktx` against parent
change `vxlqpmnknkpwtqqomqrypsrvlkosrkxo`. Every figure below was re-measured by
the landing reviewer after a second `kata refresh` onto the default line that
now carries the colorless-vocabulary landing; the implementer's pre-refresh
figures (parent 19,198 covered, +94) are superseded and are restated in
`### Review corrections`. The source fingerprint on both reports is
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.

### PROVE

One general `QualifiedFrequencyPredicateAdjunct` member now derives a
Frequency Adverb followed by an existing Predicate Adjunct. That shared
complement is where the already-authored Duration Phrase and temporal
Prepositional Predicate Adjunct machinery converge, so `once each turn`,
`twice each turn`, and `once during each of your turns` need neither a member
per surface nor a new temporal category. The form contains only
`lex(adverb) qualification`; it has no form literal spelling `turn`. Its only
guard reads the complement's declared `focus` feature, and names no word,
lexeme, construction, card, or surface. The existing Focused Predicate Adjunct
therefore composes outside it without special handling.

The added assurance selects all four witnesses through
`PredicateAdjunctQualifiedFrequencyPredicateAdjunct`: `Draw a card once each
turn.`, `Draw a card twice each turn.`, `Draw a card once during each of your
turns.`, and `Activate only once each turn.` The synthetic `Activate`
declaration was aligned with the production custom frame set so the exact
focus witness exercises the ordinary declaration-verb path. The AST export,
visitor export, and exhaustive diagnostic nonterminal mapping expose the new
member through the existing generated interfaces.

Subset-first iteration used a 767-card / 787-face jq corpus containing every
`once`/`twice` surface, every parent-selected use of the touched Frequency
Predicate Adjunct, and the acceptance witnesses. Its 660 supported units
measured 94 gains, 0 losses, 0 changed prior winners, 0 ties, and 155/155 clean
round trips before the full refreshed gates were run.

Positive final artifacts on the refreshed tree:

- `cargo fmt --all` completed successfully.
- Strict clippy completed successfully for `deckmaste_english_v2` and `xtask`
  with `--all-targets -- -D warnings`.
- The reverse-dependency closure gate was exactly `cargo test -p
  deckmaste_english_v2 -p xtask`; its result lines were:

```text
test result: ok. 146 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 111 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 468 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8` exited 0 and reported: `newly covered 95 corpus identities`;
  19,564 selected and covered, 13,077 parse failures, and zero selected-uncovered
  units, unresolved ties, internal failures, exception uses, round-trip
  mismatches, ownership failures, traversal failures, gaps, overlaps, synthetic
  claims, and provenance-plan mismatches. The following `--bless` wrote exactly
  that covered set.
- `cargo xtask english_v2 ambiguity --json --require-resolved --workers 8`
  exited 0 with 0 unresolved ties. `cargo xtask english_v2 roundtrip
  --require-clean --workers 8` exited 0 with 19,564 accepted, 19,564 clean, and
  0 mismatched.
- No citation-bearing source changed, so no citation gate or lock operation was
  required.

### DISCLOSE

| measure | refreshed parent | measured tree |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered | 19,469 | 19,564 |
| ordinary parse failures | 13,172 | 13,077 |
| unique selections | 15,271 | 15,353 |
| specificity-resolved selections | 4,198 | 4,211 |
| unresolved ties | 0 | 0 |
| construction declarations | 393 | 394 |
| licensing checkers permitted / forbidden | 21 / 0 | 21 / 0 |
| coverage-lock identities | 19,469 | 19,564 |

The report-mode lock delta is exactly +95/-0 identities. The lock moved from
52,119 lines and SHA-256
`2e5687ade92519966717ab650929aa4bfbef2b8d16ed4267dafc842fca9f9b34`
to 52,214 lines and SHA-256
`87170db56144824a4a7703b4511dccd5dbfcc4e741290692ff087c5c11fd3d0a`.
No identity stopped being covered, so no retirement or re-coverage obligation
is owed.

The parent and measured-tree ambiguity JSON reports were diffed by identity.
Exactly 95 parse failures became selected: 82 unique and 13
specificity-resolved. There are 0 losses, 0 changed selected paths and 0
changed resolution kinds among previously selected identities. Every one of the
95 gained analyses contains `PredicateAdjunctQualifiedFrequencyPredicateAdjunct`,
whose complement is the existing Duration Predicate Adjunct for 88 identities
and the existing temporal Prepositional Predicate Adjunct for 7; no gain reaches
the member through any other complement category.
Every gained selected analysis was read; all are the intended positive oracle,
and none is wrong or negative. This per-identity list is corpus provenance, not
grammar authority:

- **Aggressive Mining** (6f4f5a790142e4fba6be40de7b8a7cfb84baf9059307e9969f32b2e9fc12548b): selected analysis "You can't play lands.\nSacrifice a land: Draw two cards. Activate only once each turn."; unique, qualified frequency > duration.
- **Akki Avalanchers** (dfe9ff178872374ceb0adccb8afbd34c483b60fb078d1359c8738ffd351302b6): selected analysis "Sacrifice a land: This creature gets +2/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **As Foretold** (f27f865780ab01d1d6948ed95fdc2451700f15bbb937377814aaac1df03fab11): selected analysis "At the beginning of your upkeep, put a time counter on this enchantment.\nOnce each turn, you may pay {0} rather than pay the mana cost for a spell you cast with mana value X or less, where X is the number of time counters on this enchantment."; specificity, qualified frequency > duration.
- **Avizoa** (7a73dceb6f5235fbef6b43cbccbe5bd9d61cb570531b4fbb590a9899fa9c939c): selected analysis "Flying\n{0}: This creature gets +2/+2 until end of turn. You skip your next untap step. Activate only once each turn."; specificity, qualified frequency > duration.
- **Azimaet Drake** (89a3074da78c1c5f4976797198744633302823d52c133c8cffa19ef8381944dc): selected analysis "Flying\n{U}: This creature gets +1/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Barbarian Bully** (c5ab492a0147111394f82f1d5b8c6fc7e3eeb2e7f76f3b6625ee37c50ddd9d35): selected analysis "Discard a card at random: This creature gets +2/+2 until end of turn unless a player has this creature deal 4 damage to them. Activate only once each turn."; unique, qualified frequency > duration.
- **Barrels of Blasting Jelly** (06a7674a8e48ada03c3429569be55e3f065b91ed23191cc02ad7daee00f267cb): selected analysis "{1}: Add one mana of any color. Activate only once each turn.\n{5}, {T}, Sacrifice this artifact: It deals 5 damage to target creature."; unique, qualified frequency > duration.
- **Basking Rootwalla** (d47a82ace21ab2866501173bbe317cc3933235f688e6a6be309cc622fa4c9ccb): selected analysis "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.\nMadness {0}"; unique, qualified frequency > duration.
- **Battlefield Scrounger** (38b3e55eddc55e7dd386cb9704a503511c4903b24de047cd8e26cd55b303c4c9): selected analysis "Threshold — Put three cards from your graveyard on the bottom of your library: This creature gets +3/+3 until end of turn. Activate only once each turn and only if there are seven or more cards in your graveyard."; specificity, qualified frequency > duration.
- **Beetleform Mage** (ab2b9d2085c4e8ad43126965ad460875eea4777489ccc4c843a84a5871bd25db): selected analysis "{G}{U}: This creature gets +2/+2 and gains flying until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Beledros Witherbloom** (67a61c2e52e1031c6c0ac239eeb0cc676f2b14dec3f3916dbf05e59fda0f6cb7): selected analysis "Flying\nAt the beginning of each upkeep, create a 1/1 black and green Pest creature token with \"When this token dies, you gain 1 life.\"\nPay 10 life: Untap all lands you control. Activate only once each turn."; specificity, qualified frequency > duration.
- **Blazing Rootwalla** (3dacd47c9b8c74750919c9e4280db90e0ca7d1dcf93d8897f427492b151b4f8a): selected analysis "{R}: This creature gets +2/+0 until end of turn. Activate only once each turn.\nMadness {0}"; unique, qualified frequency > duration.
- **Boreal Centaur** (854a27a339507ca1238fa820a93f4b6976be71b3b06ad35a8615f4a2f4d9279a): selected analysis "{S}: This creature gets +1/+1 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Brutal Deceiver** (04fbf4e0daa4bb5ddbe2f3357bdc07adc3e3a96948f8c17cf643dc463792f8b1): selected analysis "{1}: Look at the top card of your library.\n{2}: Reveal the top card of your library. If it's a land card, this creature gets +1/+0 and gains first strike until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Call the Bloodline** (0b9d0063d61ca54387199e71613e645c7098aa54fca78555ea717669c3d37ea1): selected analysis "{1}, Discard a card: Create a 1/1 black Vampire Knight creature token with lifelink. Activate only once each turn."; unique, qualified frequency > duration.
- **Callous Deceiver** (d98d1b46745e5ddcb96f04817e39f24ab309f503f47e0036c4f735e990446973): selected analysis "{1}: Look at the top card of your library.\n{2}: Reveal the top card of your library. If it's a land card, this creature gets +1/+0 and gains flying until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Charred Foyer // Warped Space** (2fc4801fa63e69f0a92ad4f430b4ca398fe9277079cb7d835d4e43750b5b63e2): selected analysis "Once each turn, you may pay {0} rather than pay the mana cost for a spell you cast from exile."; unique, qualified frequency > duration.
- **Chronatog** (2dc16e6e3e5b3d2d6a4edd856f69ddb0c13d281e1bbe2cb588849f4c61eefe9a): selected analysis "{0}: This creature gets +3/+3 until end of turn. You skip your next turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Chronatog Totem** (1afe7df0bec6b971da3545c8acead5d4bb2c9c7e4580df951692e9319bb91342): selected analysis "{T}: Add {U}.\n{1}{U}: This artifact becomes a 1/2 blue Atog artifact creature until end of turn.\n{0}: This creature gets +3/+3 until end of turn. You skip your next turn. Activate only once each turn and only if this permanent is a creature."; unique, qualified frequency > duration.
- **Crazed Armodon** (58975b1aa8ff938b18a4ea306bec22549e4ddc2f6cb468591db1182989d65708): selected analysis "{G}: This creature gets +3/+0 and gains trample until end of turn. Destroy this creature at the beginning of the next end step. Activate only once each turn."; unique, qualified frequency > duration.
- **Cutthroat Centurion** (157d08ab14ea31bb154b1b5b9a8442ba853e3f68e22aad1856d599ebf9fa123b): selected analysis "Sacrifice another artifact or creature: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Cutthroat Contender** (e55a1fce962a0f61a53af95166d520e59cad7fe28e885d0a7144035eab94cff9): selected analysis "Pay 1 life: This creature gets +1/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Dai Li Censor** (bcc8d54fa625a9ff4a2b346fa5e7b65ede101ec5d6fc536f45cb456a949d1f18): selected analysis "{1}, Sacrifice another creature: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Danitha, New Benalia's Light** (9a82687b15b3aa674cef23a68be18b21cb06f6e4e28406440633c176943744db): selected analysis "Vigilance, trample, lifelink\nOnce during each of your turns, you may cast an Aura or Equipment spell from your graveyard."; specificity, qualified frequency > temporal prepositional.
- **Darksteel Monolith** (eb9e6d45a2937a4e1e666be8f2d6921ccd1abeb7b7124e3713b71c3e153b0cd3): selected analysis "Indestructible\nOnce each turn, you may pay {0} rather than pay the mana cost for a colorless spell you cast from your hand."; unique, qualified frequency > duration. Gained only on the reviewer's refreshed tree: it needs both this member and the colorless color property that landed on the default line meanwhile.
- **Darkthicket Wolf** (e4b7c7de662a6aa94d59b55199d81a83f734ecf16b7d2c582884838570daa4f9): selected analysis "{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Desolation Prowler** (4c12472d920c30c76c34453f76ab1dc8c82b89427642c9453eb03a4dad075531): selected analysis "Pay 2 life: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Dire Wolf Prowler** (e2d001b8cb02e1e75d3d715b92593c97a7d91d7fa5788bf86557dd9f6dc8490b): selected analysis "{1}{G}: This creature gets +2/+2 and gains haste until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Drake Hatchling** (ee71c716aed3220ea0b1b5b10fe9c502fcddc5670ad8856dd6ddc345d1f3f1e9): selected analysis "Flying\n{U}: This creature gets +1/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Dream Coat** (c8182431ce035f590bfe24e044a082eb00257ead338210d76ada6681be146c7e): selected analysis "Enchant creature\n{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn."; unique, qualified frequency > duration.
- **Ebon Praetor** (e81b919d6dfbca12f8519d27e7ffeeeb99ffefad9966c19d78f734a1537ab17f): selected analysis "First strike, trample\nAt the beginning of your upkeep, put a -2/-2 counter on this creature.\nSacrifice a creature: Remove a -2/-2 counter from this creature. If the sacrificed creature was a Thrull, put a +1/+0 counter on this creature. Activate only during your upkeep and only once each turn."; specificity, qualified frequency > duration.
- **Ecstatic Awakener // Awoken Demon** (da8b41543312b127fc3e1155497cb93349a019db318bc9b539951c25c750d0a2): selected analysis "{2}{B}, Sacrifice another creature: Draw a card, then transform this creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Erstwhile Trooper** (be04fce8e5a6301761246db383ec770f97a4afcd7eb8e6cff85608d785cea0ea): selected analysis "Discard a creature card: This creature gets +2/+2 and gains trample until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Fault Riders** (5bfbd829e582f7777adca1054a6170af0d7cd8796dc67f847514368adbbea3df): selected analysis "Sacrifice a land: This creature gets +2/+0 and gains first strike until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Feral Deceiver** (52bfee4a6b683ddf8982babfef4f8ed2db9f5142ff1222b1c9695680bba7db0e): selected analysis "{1}: Look at the top card of your library.\n{2}: Reveal the top card of your library. If it's a land card, this creature gets +2/+2 and gains trample until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Festerleech** (2a6e8f9e3274753cf0a615aca30ef738b880c9d8e04a660b90489fffee79b8ab): selected analysis "Whenever this creature deals combat damage to a player, you mill two cards.\n{1}{B}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Fire Drake** (213f84846c9b260405a5e36bc9cd85d8be1a3bacde8b1efbafe81fdc4ae6afc7): selected analysis "Flying\n{R}: This creature gets +1/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Foraging Wickermaw** (500389207c2fea67c08e721a07cf4d36894fd2da9c536e11a6d741259700b763): selected analysis "When this creature enters, surveil 1.\n{1}: Add one mana of any color. This creature becomes that color until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Frilled Oculus** (26ac6a522c421cdf5436859ac933815b5235ff7c00e215575d83adc268e4d222): selected analysis "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Frilled Sandwalla** (518d73c8975d75b8f4c069dbd1d952822df66191cc72cee4f0063d7979b1db64): selected analysis "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Frostwalla** (601a23b5e6c1b99aa47a903b1ab62f231b6b2d3e394ff54237f92ca37908ab7e): selected analysis "{S}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Gate to Phyrexia** (5a86a31fe37eaa171d0637026a057b0bc7e9e7b327af5cdf4523b1424c5646de): selected analysis "Sacrifice a creature: Destroy target artifact. Activate only during your upkeep and only once each turn."; unique, qualified frequency > duration.
- **Ghor-Clan Bloodscale** (17191358626683f62f676d4da2e0cd1ec9d983ff1846dc6d4aeecab30e2aaf2d): selected analysis "First strike\n{3}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Gisa and Geralf** (c23a058a9fff1450ba6d586a2f9f6f0042a84a717877c87bea93bd06b70fbc8d): selected analysis "When Gisa and Geralf enters, mill four cards.\nOnce during each of your turns, you may cast a Zombie creature spell from your graveyard."; unique, qualified frequency > temporal prepositional.
- **Graverobber Spider** (deb615879bef246f8a44f1d803ae03e769b670a48707797f83337540bfb97f14): selected analysis "Reach\n{3}{B}: This creature gets +X/+X until end of turn, where X is the number of creature cards in your graveyard. Activate only once each turn."; unique, qualified frequency > duration.
- **Gravestone Strider** (6f35239d55afbefd929c61c7f20c6513153c0bfabddb69abea2353d6ba7f6478): selected analysis "{1}: Add one mana of any color. Activate only once each turn.\n{2}, Exile this card from your graveyard: Exile target card from a graveyard."; unique, qualified frequency > duration.
- **Groundling Pouncer** (f262956738639caf789dfd9d2fe75cd6a034456dd84aa6ddcd796573e8a90376): selected analysis "{G/U}: This creature gets +1/+3 and gains flying until end of turn. Activate only once each turn and only if an opponent controls a creature with flying."; unique, qualified frequency > duration.
- **Heir of Falkenrath // Heir to the Night** (8797cfee9f5e629cb6a4e798bfb78e43682ff8cf8b21bc3698891b24ea2b8fab): selected analysis "Discard a card: Transform this creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Hollow Scavenger // Bakery Raid** (73a6902ef2e679eb13e709514098a413e6f0aad5020a58e8cae8d75d01ad47c1): selected analysis "{1}, Sacrifice a Food: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Instill Energy** (d9ecf9f488c4882195e831b99e41d58478c630c66c943359c65ac4600284b726): selected analysis "Enchant creature\nEnchanted creature can attack as though it had haste.\n{0}: Untap enchanted creature. Activate only during your turn and only once each turn."; unique, qualified frequency > duration.
- **Karador, Ghost Chieftain** (be2cdf1b3771d73dcd7479cd4142662a731cf89c8bafc072ad2d88b1c6471ecf): selected analysis "This spell costs {1} less to cast for each creature card in your graveyard.\nOnce during each of your turns, you may cast a creature spell from your graveyard."; unique, qualified frequency > temporal prepositional.
- **Kess, Dissident Mage** (83ce0cd76bd62e1b6f6f0fbf519a9f8c0270aa9e011daa462ffcb498e7e3bf22): selected analysis "Flying\nOnce during each of your turns, you may cast an instant or sorcery spell from your graveyard. If a spell cast this way would be put into your graveyard, exile it instead."; specificity, qualified frequency > temporal prepositional.
- **Knight of the Skyward Eye** (3e9121b336ed27617bc6647f8501104bd7dd3e6a5efb16eab14cfc4a678b7de4): selected analysis "{3}{G}: This creature gets +3/+3 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Kozilek's Translator** (db1c3ad8e1b46e3c681e84bf5bee266cc484082c4880f9e9b2e01ff328d78f60): selected analysis "Devoid\nPay 1 life: Add {C}. Activate only once each turn."; unique, qualified frequency > duration.
- **Kraven's Cats** (34740e2e021e2b76d9c186e5d283602437e5dbfa62802f6fe6846ae83ed8313d): selected analysis "{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Kyscu Drake** (ce06282efcffe861242d66ed0b88f577405da86ef11e5d856686c3e9fe784b0b): selected analysis "Flying\n{G}: This creature gets +0/+1 until end of turn. Activate only once each turn.\nSacrifice this creature and a creature named Spitting Drake: Search your library for a card named Viashivan Dragon, put that card onto the battlefield, then shuffle."; specificity, qualified frequency > duration.
- **Locust Swarm** (bbb3538ca2694df69cced9652daef649d02cda388fdbcfb5edcbeed0fe521d64): selected analysis "Flying\n{G}: Regenerate this creature.\n{G}: Untap this creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Mana Bloom** (08ac5087bffe2a002de4686be04665d053d608f3b6bca037a95e0da73c39f152): selected analysis "This enchantment enters with X charge counters on it.\nRemove a charge counter from this enchantment: Add one mana of any color. Activate only once each turn.\nAt the beginning of your upkeep, if this enchantment has no charge counters on it, return it to its owner's hand."; specificity, qualified frequency > duration.
- **Mindful Biomancer** (1d274fd397e395fef7df6ba955b43524d1bc5edf01a5b75946f6d8ab313fdd38): selected analysis "When this creature enters, you gain 1 life.\n{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Mobile Fort** (8cac37827f3b3a38f60ae7e295064dc3a46ea9905ce84e60768ca1762adc0635): selected analysis "Defender\n{3}: This creature gets +3/-1 until end of turn and can attack this turn as though it didn't have defender. Activate only once each turn."; unique, qualified frequency > duration.
- **Plague Nurse** (308bbede947aed140ce0aa0873b0cf45d82517d109574b0746d9c4d6bc517ceb): selected analysis "Toxic 2\n{2}{G}: Each other creature you control with toxic gains toxic 1 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Plated Rootwalla** (a11896414cf199059d8124615a601602543ef94034f5d2b8a49285a2bd691d5a): selected analysis "{2}{G}: This creature gets +3/+3 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Pulsating Illusion** (02c9964836a770ee6268fe8dac83e9eb4255b90094b27270e43e886e2cbfddb8): selected analysis "Flying\nDiscard a card: This creature gets +4/+4 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Putrid Leech** (81120564382032b388d5aae1114a1fae85a79a40fdba71f0adc6b0ba2fc8d5a7): selected analysis "Pay 2 life: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Quirion Ranger** (b68eff69afdba6826fc6bb7e332dff4e297a1d9952eaa50a2f3befda31702364): selected analysis "Return a Forest you control to its owner's hand: Untap target creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Ramos, Dragon Engine** (d3bcd528a9ad5b767a4da26f7f813694a4fe44734c9c4f6a5e259aa16f34b5d5): selected analysis "Flying\nWhenever you cast a spell, put a +1/+1 counter on Ramos for each of that spell's colors.\nRemove five +1/+1 counters from Ramos: Add {W}{W}{U}{U}{B}{B}{R}{R}{G}{G}. Activate only once each turn."; unique, qualified frequency > duration.
- **Raul, Trouble Shooter** (c521abc3819e8690277025f6a22417aa9eb2606646f62577e577602c28190a6f): selected analysis "Once during each of your turns, you may cast a spell from among cards in your graveyard that were milled this turn.\n{T}: Each player mills a card."; unique, qualified frequency > temporal prepositional.
- **Rootwalla** (2f5e0f655e8a07a9137a2e96fc4b7bc3f81027afb769603b3058933573d6a55e): selected analysis "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Salvaged Manaworker** (4ca779fd75e0e3f01234db8c73e3d9f5e04ae77901af06394f4dd8657471c75f): selected analysis "{1}: Add one mana of any color. Activate only once each turn."; unique, qualified frequency > duration.
- **Savage Knuckleblade** (278caa5321ef37d33f039c610978b6e02a9a985edca0e22fb992a01178fb4c37): selected analysis "{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.\n{2}{U}: Return this creature to its owner's hand.\n{R}: This creature gains haste until end of turn."; unique, qualified frequency > duration.
- **Scarecrow Guide** (fcc15650a11af157eb7779e6d7b8d4e22dec587a4a8ec331d0e1942b3315ed7c): selected analysis "Reach\n{1}: Add one mana of any color. Activate only once each turn."; unique, qualified frequency > duration.
- **Scryb Ranger** (a6261926ff9350e80a605d6f9ab24bf8aa58eb6ae6eb0b9f9da325c9e19fa6ee): selected analysis "Flash\nFlying, protection from blue\nReturn a Forest you control to its owner's hand: Untap target creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Security Detail** (db56d61476be9883a91c6ae23b765d19366c0b3c9f87d43fb9a811c3bbb995e2): selected analysis "{W}{W}: Create a 1/1 white Soldier creature token. Activate only if you control no creatures and only once each turn."; specificity, qualified frequency > duration.
- **Sepulcher Ghoul** (7d8f1bc730544553c45d9204992b33855d9829e6f8ca3d3b2c41ff602bf8aa2b): selected analysis "Sacrifice another creature: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Setessan Griffin** (3dfd04dbd39eac6e6b2913cf3698eadf21e89ee01c11f01780660169b5ca915e): selected analysis "Flying\n{2}{G}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Shire Scarecrow** (0865ff925b2919adcd6e68199f946aacac81becb3f3eb01d556a7833137c9e9d): selected analysis "Defender\n{1}: Add one mana of any color. Activate only once each turn."; unique, qualified frequency > duration.
- **Skinshifter** (f53ed273c59c19061036f29853c13c3762a36ad029c4bfbb81d3dbaf5ad08cf8): selected analysis "{G}: Choose one. Activate only once each turn.\n• Until end of turn, this creature becomes a Rhino with base power and toughness 4/4 and gains trample.\n• Until end of turn, this creature becomes a Bird with base power and toughness 2/2 and gains flying.\n• Until end of turn, this creature becomes a Plant with base power and toughness 0/8."; specificity, qualified frequency > duration.
- **Snarling Wolf** (a2f8b031c8eb4ce4799eb0098940b5d184b6a3bd334ad5bb6ff58611867dcdf5): selected analysis "{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Snowslope Hunter** (fc57ca8fb722d46b619e2b05d1b4bcfab1ad54deef897960bc64910637ab28f1): selected analysis "Sacrifice another creature or artifact: Exile the top card of your library. You may play it until the end of your next turn. Activate only during your turn and only once each turn."; unique, qualified frequency > duration.
- **Spitting Drake** (881fcb1b09e1302a837388a1a1fdd99c96d5718f2f2c52ab1f89f3f4e451b338): selected analysis "Flying\n{R}: This creature gets +1/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Stalking Drone** (af5901fcb3ce1bf033bbd55c62df69f5db01a166bf5ba613be6a5219f11202a5): selected analysis "Devoid\n{C}: This creature gets +1/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Stromkirk Condemned** (77e7fb2f32af98e62e200eebcbe87e176074529325b6f9e37b7d28a18f6291f0): selected analysis "Discard a card: Vampires you control get +1/+1 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Sunbathing Rootwalla** (360331a219837a67b3c585a878f1aedf9bc4055fbcda10bdd752e457d935c60b): selected analysis "Domain — {3}{G}: Until end of turn, this creature gets +1/+1 for each basic land type among lands you control. Activate only once each turn."; specificity, qualified frequency > duration.
- **Three Tree Mascot** (a8dc995e38d78353fff9a4e586e4e5268e5d15f8ce11bf9a675c9635dad85f56): selected analysis "Changeling\n{1}: Add one mana of any color. Activate only once each turn."; unique, qualified frequency > duration.
- **Tlincalli Hunter // Retrieve Prey** (36116f3f648e276178aee3bcd206c17c66fa6f0493515a0c9cb00314b5bfbc78): selected analysis "Trample\nOnce each turn, you may pay {0} rather than pay the mana cost for a creature spell you cast from exile."; unique, qualified frequency > duration.
- **Twinblade Slasher** (9985d75893307e5ce5295812566e76e5c7fd4eeb87b0b27ebd9fc8b25a032ffa): selected analysis "Wither\n{1}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Ursine Champion** (294394c7b32e2a0d9ef4bf60a06fdffbff36e428c3425c618c52a862aa559c58): selected analysis "{5}{G}: This creature gets +3/+3 and becomes a Bear Berserker until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Viashino Slaughtermaster** (aa307171488ec8d5a13f64aca4629834ab5550821e67d2a6a3a0492f5733acc0): selected analysis "Double strike\n{B}{G}: This creature gets +1/+1 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Vision, Spectral Synthezoid** (c05779c8c93fb5d12a10a18c3e97b701319b5a95aea03af6921b9fc8e9cc85de): selected analysis "Flying\nOnce during each of your turns, you may cast a noncreature or Robot spell from your hand without paying its mana cost."; unique, qualified frequency > temporal prepositional.
- **Walking Wall** (7a5e864fd938233915ad7e7d4334c0c252721ea956208195397ad713d4cf1656): selected analysis "Defender\n{3}: This creature gets +3/-1 until end of turn and can attack this turn as though it didn't have defender. Activate only once each turn."; unique, qualified frequency > duration.
- **Wall of Roots** (0743f8de887e0c175f765ab9a31405c19ff3463a1aa3199a0dab6dcb8a5132f8): selected analysis "Defender\nPut a -0/-1 counter on this creature: Add {G}. Activate only once each turn."; unique, qualified frequency > duration.
- **Wild Aesthir** (1e70467c45c08f2d4bb4a8ec6922a94c41c38abc51af8d929ee663fd388bb4de): selected analysis "Flying, first strike\n{W}{W}: This creature gets +2/+0 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Wirewood Symbiote** (2243da8ed200c4c9cd7bb6eb7d44c3875428a844e51e91f75fac4ec4261270b7): selected analysis "Return an Elf you control to its owner's hand: Untap target creature. Activate only once each turn."; unique, qualified frequency > duration.
- **Wolfsbane, Highland Hero** (50119e021758c372d7278441c12c8c88274a941817adf07dd878bf6366047e45): selected analysis "Trample\n{2}{G}: Wolfsbane gets +2/+2 until end of turn. Activate only once each turn."; unique, qualified frequency > duration.
- **Zaffai and the Tempests** (362f187fd3e5f038e2cde743560bfe7a8740c1e018d5fdc99d796144fcb0b20a): selected analysis "Once during each of your turns, you may cast an instant or sorcery spell from your hand without paying its mana cost."; specificity, qualified frequency > temporal prepositional.

Block accounting for the census family this ticket inherited. The
`english-v2-tail-restrictive-focus-adverb` landing routed a 101-unit
`only once each turn` block here; the tail-family register re-measured the same
family at 79 units on 2026-09-05. Of the 95 gains, 84 carry
`only once each turn`, 7 carry `Once during each of your turns`, and 4 carry a
sentence-initial `Once each turn,`. 278 corpus units print
`only once each turn`; 194 of them still fail, and exactly 20 of those fail
INSIDE the frequency phrase: 18 are the `Do this only once each turn.` shape and
2 are a coordinated focus tail (`… and only once each turn`, on Nature's Chosen
and Sawback Manticore). The 18 are blocked independently of this family —
`Do this.` on its own is a parse failure at its final period, so what they wait
on is the pro-verb predicate, not the frequency adverbial. The remaining 174
fail earlier, elsewhere in the unit. `twice each turn` prints on 8 units, all
still failing outside the phrase; its unit witness `Draw a card twice each
turn.` selects. The comparative `FrequencyReference` shapes are untouched:
`more than once` selects on the same 10 units and fails on the same 13 before
and after.

### REPORT

The canonical `cargo xtask english_v2 report` construction count is 393 ->
394. The measured coverage inventory reports the same two licensed
vocabulary/lexicon homographs as the parent: `AttributiveAdjective::Untap`
beside the declared `Untap` keyword action, and `TargetingMarker::Target`
beside `CommonNoun::Target`. The nine form-literal/vocabulary overlaps are:
`additional` in `additional_cost`; `to` in `up_to_quantifying_determiner`;
`the` and `next` in `definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and
`the` in `positional_partitive`. These are provenance inventories and were not
fitted to. Licensing checkers remain 21 permitted and 0 forbidden; no checker
names a word, lexeme, construction, or card.

Performance advisory, all at 8 workers on the reviewer's refreshed tree:
coverage `--check` took 129 s (128,826 ms) at 159,997 ns/B under 1-minute host
load 15; the following `--bless` took 117 s (116,579 ms) at 122,305 ns/B under
load 19; ambiguity took 131 s (130,820 ms) at 149,321 ns/B under load 17;
roundtrip took 183 s (183,190 ms) at 224,129 ns/B under load 16. Each exceeded
the 16,260 ms quiet-host ceiling under load and is advisory, not a STOP. The
per-byte figures are thread-CPU, so they scale with the contention rather than
with this diff: the one added construction cannot account for them.

Contention stamp (reviewer): three sibling executors held live claims during
these gates — `english-v2-scope-device-collapse`,
`english-v2-bare-duration-adjunct-licence` and
`english-v2-affinity-quality-surface` — plus this review's own gate run, four
concurrent workloads on the shared host. The implementer's sandbox-visible
count of 4 processes measures nothing about that.

Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added 1
test function; removed 0. The one added test function carries four positive
selection assertions. One shared synthetic declaration fixture was adjusted to
the production `Activate` frame set; no existing test function or oracle was
weakened or removed.

Deviations and additions: none beyond the ticket's letter. The one production
construction is the coordinator-pinned general qualified member; the AST,
visitor, diagnostic mapping, test, and exact coverage-lock update are its
required consumers and evidence. No dominance edge, selection exception,
per-surface member, narrowed form, new temporal category, word-naming guard, or
card-specific logic was added. Attestation supplied the corpus witnesses and
provenance only; it does not license the grammar.

STOPs: none. The ticket and the 2026-09-04/05 rewrite rulings agree; there is no
selection tie, wrong or negative newly covered analysis, loss, roundtrip
mismatch, or prohibited guard.

glossary gap: **Frequency Adverb**. `docs/contexts/oracle-english/CONTEXT.md`
defines Adverb, Adverb Phrase, Focus Adverb and Adjunct, but no frequency term,
so the category this landing's member is headed by — and the qualified
frequency adverbial itself — have no glossary entry. The vocabulary predates
this landing, so no entry was added here; naming it is owed to whichever ticket
next amends that glossary.

decision wanted: none.

### Review corrections

Landing review (Opus reviewer/integrator), applied in this workspace after a
second `kata refresh` onto the default line carrying the colorless-vocabulary
landing.

- MEDIUM — the record's whole measurement was stamped on the pre-refresh
  parent (19,198 covered, +94, lock 51,942 lines). The refresh moved every
  corpus figure. Fix: re-ran the full gates on the refreshed tree and replaced
  the PROVE artifacts, the DISCLOSE table, the lock delta, and the ambiguity
  diff paragraph with the re-measured numbers (parent 19,469 → tree 19,564,
  +95/-0). The gain count rose by one because Darksteel Monolith needs this
  member *and* the colorless color property; that identity was added to the
  per-identity list and the lock was re-blessed to include it.
- MEDIUM — every wall-time figure was written with a three-digit fractional
  part (the coverage, ambiguity, roundtrip and ceiling seconds), the shape the
  cite checker reads as a rule number and the shape that has failed three
  landings. Fix: all times are integers with units.
- MEDIUM — the record left the contention stamp to the reviewer and the
  inherited `only once each turn` block unaccounted. Fix: the true contention
  is stamped in the performance advisory, and DISCLOSE now carries the block
  accounting, including that the 18 residual `Do this only once each turn.`
  units are blocked by the pro-verb predicate (`Do this.` fails alone), not by
  this family.
- LOW — `glossary gap: none` was wrong: Oracle English defines no Frequency
  Adverb. Fix: the gap is recorded.
- Verified and NOT changed: the new member is one construction over the
  existing `PredicateAdjunct` sum — the same complement category
  `focused_predicate_adjunct` already takes — with no new temporal category and
  no form literal spelling `turn`; its single `require` reads the complement's
  declared `focus` feature and names nothing lexical. The
  `Activate` test fixture change matches
  `plugins/builtin_v2/macros/stubs/keyword_actions/Activate.ron` exactly, so it
  is an alignment, not a fixture fitted to the witness. Assurance counts are
  true against the diff (added 1, removed 0). Reading the whole gain set
  identity by identity confirmed all 95 select through the qualified member on
  a Duration or temporal Prepositional complement; none is a negative oracle.

Residue routed at review: the 18-unit `Do this only once each turn.` shape and
the 2-unit coordinated `… and only once each turn` tail are recorded in the
tail-family register's routed-residue note in `docs/tickets/fog.md`; neither is
owed to this ticket.
