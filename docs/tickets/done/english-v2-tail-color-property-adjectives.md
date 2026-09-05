---
needs: []
---
**The colour-property adjective class is one member short.** `Create a 1/1
monocolored Servo artifact creature token.` and `This permanent is
monocolored.` both select; replace *monocolored* with *colorless* and both
fail. `vocab Color` holds the five colour words and two of the three
colour-cardinality adjectives, and nothing else in the grammar spells the
third.

Sizing from the 2026-09-05 failure census (measured on change `osnrsxuvkrwo`,
19,198 / 32,641 covered, 13,443 parse failures, first-failure byte attribution;
re-measure at claim): **387 units** — the largest family in the corpus that no
ticket owns.

Not a scanner defect. `docs/tickets/fog.md` recorded this family as "scanner
splits `color` + `less`"; that diagnosis was wrong and is corrected by this
ticket. The probe shows the scanner offering `Noun { noun: Color }` over bytes
`12..18` of `Create a 1/1 colorless Servo…` and the parse failing at `18..22`
expecting `'`, `'s`, `, `, `, then `, `.` — a possessive-adjacency continuation.
`has_lexical_boundary` (`crates/deckmaste_english_v2/src/parser/scan.rs:1184`)
does enforce a right word boundary; the mid-word reading survives only because
an adjacency-marked follower suppresses it, and that branch then dies with no
effect on the outcome. The unit fails for one reason: **no lexeme spans
`colorless`**. Do not "fix the scanner" here.

Defect sentences (census, first-failure offset in the whole-face unit):

- Abstruse Interference — `You create a 1/1 colorless Eldrazi Scion creature
  token.` — `parse failed at bytes 82..86; expected `'`, `'s`, `, `, `, then `, `.``
- Access Denied — `Create X 1/1 colorless Thopter artifact creature tokens with
  flying, where X is that spell's mana value.` — `parse failed at bytes 40..44`
- Adverse Conditions — `Create a 1/1 colorless Eldrazi Scion creature token.` —
  `parse failed at bytes 128..132`
- Probe, attributive: `Create a 1/1 colorless Servo artifact creature token.` —
  `parse_failure span_start=18 span_end=22`
- Probe, predicative: `This permanent is colorless.` — `parse_failure
  span_start=23 span_end=27`
- Controls that select today: `Create a 1/1 monocolored Servo artifact creature
  token.`, `Create a 1/1 white Servo artifact creature token.`, `This permanent
  is monocolored.`

Pinned shape: **add `Colorless = "colorless"` to `vocab Color`
(`crates/deckmaste_english_v2/src/constructions.rs:184`) and change nothing
else.**

The enumeration is complete by construction against the colour system the class
realizes, not against the census. [CR#105.1] fixes the five colour words —
white, blue, black, red, green — all five present. [CR#105.2a..105.2c] fixes
the three colour-cardinality adjectives: monocolored [CR#105.2a] present,
multicolored [CR#105.2b] present, colorless [CR#105.2c] **absent**. Eight
members complete the class; the grammar declares seven. There is no ninth: any
further colour word would have to be a sixth colour, which [CR#105.1] excludes.

The four consumers of `lex Color` are all standalone `lex(color)` forms — no
host fusion, nothing embedded inside another form — so the new member reaches
every position the existing members reach with no construction work:

```
1751:    construction predicative_color: PredicativeColorComplement {
1753:        form predicative_color = lex(color);
2463:    construction color_modifier: NominalModifier {
2469:        form color_modifier = lex(color);
2562:    construction non_color_modifier: NominalModifier {
2568:        form non_color_modifier = prefix("non", lex(color));
4082:    construction fused_color_nominal: Nominal {
```

Witness chains, checked link by link for constituency:

- `Create a 1/1 colorless Servo artifact creature token.` — `lex Color` →
  `color_modifier` (form `lex(color)`, a standalone `NominalModifier`) →
  nominal → object. Every link is a node; `monocolored` selects through this
  exact chain today.
- `This permanent is colorless.` — `lex Color` → `predicative_color` (form
  `lex(color)`) → `PredicativeColorComplement` → `PredicativeComplement`
  (`:1236`) → copular predicate. `monocolored` selects through this exact chain
  today.
- `fused_color_nominal` and `non_color_modifier` take the member with no
  further work; both derive `onset` from the member (`derive onset =
  color.onset`), so *colorless* is consonantal like *monocolored* and the
  *a*/*an* choice needs no special handling.

Affected subset. Surface: `\bcolorless\b`. Touched constructions:
`color_modifier`, `non_color_modifier`, `predicative_color`,
`fused_color_nominal` — every card whose parent-tip selected path contains one
of the four, because the vocabulary they read gains a member. Witnesses: the
three census cards above. Negatives that must not move: any card using
*monocolored*, *multicolored*, or a bare colour word in those four positions.

Pre-ruled, so no STOP is spent on it: the new member makes
`non_color_modifier` admit *noncolorless* and `fused_color_nominal` admit a
fused *colorless* head. **That is correct and is not a defect** — attestation is
provenance, not a filter (rewrite ADR), and a landed construction admits its
full linguistic domain whether or not the corpus prints it. What *is* a STOP is
a real corpus identity that starts selecting a **wrong** analysis; check the
four consumers for that specifically.

Ruled against.

- A `colorless` form literal in any construction, or a construction minted to
  spell it. The class member is the whole change.
- Touching the scanner, `has_lexical_boundary`, or the adjacency suppression.
  The mid-word `color` reading is a dead branch, not the cause.
- Adding *colored*, *hybrid*, or any further member. The class is closed at
  eight by [CR#105.1] and [CR#105.2a..105.2c].
- Narrowing an existing consumer to keep a number.

STOP-and-report: any `require` or `checked by` naming a colour, the new member,
a noun, or a card; any dominance edge or exception entry added to resolve a
rivalry the new member exposes.

Routed, not in scope. The vocabulary is **misnamed**: [CR#105.4] states
"'Multicolored' is not a color. Neither is 'colorless.'", and the class already
holds *monocolored* and *multicolored* today, so `vocab Color` names a colour
system while holding a colour-**property** class. This ticket does not create
that defect and does not fix it; adding the eighth member sharpens it. Mint the
rename at landing (blast radius measured 2026-09-05: five sites in
`constructions.rs` plus `ast.rs`, nothing outside `deckmaste_english_v2`), and
record it as a routed obligation in the landing record.

Acceptance. The standard landing record (PROVE / DISCLOSE / REPORT per
CLAUDE.md and the rewrite ADR's 2026-09-04 amendment), plus:

- the five probe sentences above select, with *colorless* rendering byte-exact
  in both attributive and predicative position;
- the three controls still select and their selected analyses are unchanged;
- the construction count is unchanged — this landing adds no construction;
- both byte-exact laws green with total ownership, zero unresolved ties;
- `docs/tickets/fog.md`'s "scanner splits `color` + `less`" line is deleted, not
  merely re-counted: the diagnosis it records is wrong.

Baseline: measured on change `osnrsxuvkrwo`, 19,198 covered of 32,641, 13,443
parse failures, 0 unresolved ties, 0 internal failures. Re-measure at claim.

Glossary. `docs/contexts/oracle-english/CONTEXT.md` defines no colour term at
all. If the landing needs one — for the class this vocabulary holds, as
distinct from the Game Model's colours — amend that glossary through the
`domain-modeling` skill as part of the work and disclose it.

Tier: **terra** — one declared vocabulary member, four consumers verified as
standalone forms, no construction, no seam, no compiler work.

Standard constraints apply.

## Landing record

Measured on change `ppxwomzppxxl` after refresh, lock covered 19,469.

PROVE. Added only `Colorless = "colorless"` to `vocab Color`; no scanner,
construction, selection, or word-naming guard changed. The five probes and the
three existing controls select and render byte-exactly. Full roundtrip reports
19,469 clean of 19,469 accepted; ambiguity reports zero unresolved ties and
zero internal failures. The coverage census reports 21 permitted and 0
forbidden licensing checkers. No coverage identity was lost.

DISCLOSE. The parent-tip per-unit diff is exactly 271
`parse_failure -> selected` identities; every gain has an expected selected
consumer: `NominalModifierColorModifier` (280 occurrences across 263
identities), `PredicativeColorComplementPredicativeColor` (9 occurrences across
8 identities), and one `NominalModifierNonColorModifier` occurrence. That last
one is `nonblack` on Corpsehatch, an existing member on a unit this landing
gains for its *colorless* modifier — not a `noncolorless` gain. No gain uses an
unexpected consumer, and there is no fused-color gain in the corpus. Selection
census: 19,198 selected / 15,056 unique / 4,142 specificity-resolved before;
19,469 / 15,271 / 4,198 after. The specificity share rises by 56; no
construction pair was added. The routed follow-up
`english-v2-rename-color-vocabulary` is minted in `planned/`.

The fog register's retired scanner diagnosis was deleted. `noncolorless` and a
fused colorless head remain admitted by the existing general consumers, as
pre-ruled; the corpus prints neither, so both are witnessed by unit test only.
Deviations and additions: no construction was added or deleted; review added
two unit tests (below). Assurance counts: restored 0; re-spelled 0; ignored 0;
added 2; removed 0. Glossary gap: `docs/contexts/oracle-english/CONTEXT.md`
names no term for the color-**property** class this vocabulary holds — the five
Game Model Colors plus *monocolored*, *multicolored* and *colorless*, which
[CR#105.4] excludes from Color. Naming it is the pinned first step of the
routed `english-v2-rename-color-vocabulary`; this landing introduces no term
and keeps the existing source name.

The census sized this family at 387 units; 271 of them now select. The
remaining 116 fail for grammar this ticket does not own: sampled twelve
(Reef Roads, Foul Roads, Shorikai, Sugar Coat, Black Mage's Rod, Chaos Moon,
Dancer's Chakrams, Brood Birthing, Whirler Virtuoso, Kavaron, Baku Altar,
Corrupted Crossroads) and in every one the *colorless* span itself parses —
the first failure is downstream, at a quoted-ability verb (`saddles`, `crews`),
a coordinated quote-interior predicate, `Otherwise`, `count the number of`,
an energy or ki counter, Station, or a mana-restriction sentence. No sampled
residue is a color-property gap.

REPORT. Coverage lock: 19,198 -> 19,469 (+271); construction count: 393 ->
393; permitted licensing checkers 21, forbidden 0. Performance advisory, on the
integrated tree (change `mpkmytrvntos`, lock covered 19,469): full coverage
check with 8 workers took 107,852 ms against the 16.26 s quiet-host ceiling, at
121,722 ns/B; host load 8.01 / 12.62 / 13.72. The implementer's pre-review
measurement on `ppxwomzppxxl` was 130,292 ms at 152,535 ns/B, host load
19.70 / 13.99 / 10.32. True contention across both: five concurrent executors
plus this review (six sessions); the implementer's in-sandbox process count was
unavailable and is superseded by this stamp. The ceiling is exceeded under that
load and is reported, not fitted. Homograph inventory: `AttributiveAdjective::Untap` /
keyword-action `Untap`; `TargetingMarker::Target` / `CommonNoun::Target`.
Form-literal/vocabulary overlap inventory: `additional`, `to`, `the`, `next`,
`to`, `the`, `the`, `other`, `the` at the reported construction atoms. The
full gain delta follows; each line supplies the identity, card, and selected
analysis.

### Review corrections

- **No test witnessed the new vocabulary member.** Added
  `the_colorless_color_property_reaches_every_nominal_consumer`
  (`tests/nominal_grammar.rs`) — attributive `color_modifier`, `non_color_modifier`
  on *noncolorless*, and the fused `fused_color_nominal` head, each asserted by
  selected construction path and byte-exact render — and
  `predicative_color_complements_accept_the_colorless_property`
  (`tests/predicate_grammar.rs`) for *is/becomes colorless*. Both carry a
  *colored* negative so the eight-member closure is asserted, not assumed. Both
  falsified: with the vocabulary line removed each fails at the *colorless*
  span (`bytes 18..22` and `23..27`), then restored.
- **`NominalModifierNonColorModifier` gain misattributed.** The single
  occurrence is `nonblack`, not the pre-ruled `noncolorless`; DISCLOSE corrected
  and the two pre-ruled shapes are now stated as test-only witnesses.
- **Consumer identity counts off by two.** `NominalModifierColorModifier`
  covers 263 identities, not 261; the predicative consumer's 9 occurrences fall
  on 8 identities (Ersatz Gnomes carries two). Re-derived from the selected
  construction path of all 271 gain identities.
- **Sizing residue undisclosed.** The 387-unit census versus 271 gains was
  unexplained; the residue is now sampled and its cause class stated.
- **Glossary gap denied.** The record claimed none while routing a rename
  ticket that exists because Oracle English has no name for the class; recorded
  as a gap and routed.
- **Contention stamp missing.** Replaced the sandbox "unavailable" line with the
  true count (five executors plus this review).
- **Routed ticket lacked its authority.** `english-v2-rename-color-vocabulary`
  now cites [CR#105.1] and [CR#105.4] for the claim that *colorless* and
  *multicolored* are not Colors.
- **`cite check --list-noncompliant` was red on the delivered tree.** The perf
  advisory's raw `130291525784` ns wall time, printed with a decimal point after
  three digits, matched the bare-rule regex
  (`\b[0-9]{3}\.[0-9]+[a-z]*\b`); a wip ticket is in the checker's source set
  (only `docs/tickets/done/` is excluded), so the landing could not have passed
  its own citation gate. Wall times are now written in milliseconds.

Selection neutrality, re-derived at review on the final tree: of the 586
corpus units whose card carries the surface *colorless*, 271 are the gains and
315 select; **no unit that selected before this landing contains the surface at
all** (the other 315 are sibling faces of those cards), so no pre-existing
analysis can have moved. Each of the 271 gains was re-read from its selected
construction path, not from the record: 263 through
`NominalModifierColorModifier`, 8 through
`PredicativeColorComplementPredicativeColor`, none through
`NominalFusedColorNominal`. All nine predicative gains are *is/are/becomes
colorless*; every attributive gain modifies a token type, `creature`, `spell`,
`sources`, `nonland permanent` or `Forest land`. No negative oracle and no
wrong analysis.

Review gate scope: `cargo test -p deckmaste_english_v2 -p xtask` — the diff is
confined to `crates/deckmaste_english_v2`, whose only reverse dependency is
`xtask` (`cargo metadata --no-deps`), and trunk's new
`cargo xtask gate --changed --from default@` prints that exact command. No
`plugins/builtin_v2/`, `core_verbs.ron` or `emit/` path is touched, so the
workspace gate does not apply. `ambiguity --require-resolved` and
`roundtrip --require-clean` were not re-run separately: their exit predicates
read only `internal_failures`, `unresolved_ties` and accepted-render
`mismatched`, and the single full `coverage --check` above reports
`unresolved_ties:0`, `internal_failures:0` and `roundtrip_mismatch_units:0`
over all 32,641 units.

### Coverage lock delta

```text
+newly covered 271 corpus identities
newly covered	0064b3abaf6e7473d93963be37805d0cdddb1a4a4e57f3f35b8ffe7e6b856c20	card "Gargoyle Castle"	selected_analysis "{T}: Add {C}.\n{5}, {T}, Sacrifice this land: Create a 3/4 colorless Gargoyle artifact creature token with flying."
newly covered	0231be5baa94972c37453b43abffb91887b175f3d3876d52046cfd51dc729a6c	card "Breya's Apprentice"	selected_analysis "When this creature enters, create a 1/1 colorless Thopter artifact creature token with flying.\n{T}, Sacrifice an artifact: Choose one —\n• Exile the top card of your library. Until the end of your next turn, you may play that card.\n• Target creature gets +2/+0 until end of turn."
newly covered	030fab0e2163dea866e48676812a121e289e8b32c7912fcd25295e12739f5f4c	card "Master Splicer"	selected_analysis "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\nGolems you control get +1/+1."
newly covered	068427f4df1e624fe622860458121cdc88ac6d1a10158960b34cf0dead72f391	card "Go-Shintai of Shared Purpose"	selected_analysis "Vigilance\nAt the beginning of your end step, you may pay {1}. If you do, create a 1/1 colorless Spirit creature token for each Shrine you control."
newly covered	07c7e3b8182c9f47ec3477db0764a64de95599980e46e0a622415fca7ea7eab9	card "Masterful Replication"	selected_analysis "Choose one —\n• Create two 3/3 colorless Golem artifact creature tokens.\n• Choose target artifact you control. Each other artifact you control becomes a copy of that artifact until end of turn."
newly covered	08549885562d975b62a29e8aee15b412defd3ff869cd899f29ee182988c11a28	card "Broadcast Rambler"	selected_analysis "When this Vehicle enters, create a 1/1 colorless Thopter artifact creature token with flying.\nCrew 1"
newly covered	0d43a7db242c2cb5f9befe01bb7a154e2414d1b46f156d499edea14ca2d984e6	card "Cybernetica Datasmith"	selected_analysis "Protection from Robots\nField Reprogramming — {U}, {T}: Target player draws a card. Another target player creates a 4/4 colorless Robot artifact creature token with \"This token can't block.\""
newly covered	0e3c7ecda4ec5e41793d5a92dae84ef8d1c2ef2e3853d81f8b7a95ec706e9a65	card "Ersatz Gnomes"	selected_analysis "{T}: Target spell becomes colorless.\n{T}: Target permanent becomes colorless until end of turn."
newly covered	113b708b88ccabe7c38c44502329becbcfa2a1f0317b095643f47a00f64b0fe3	card "Mishra's Onslaught"	selected_analysis "Choose one —\n• Create two 1/1 colorless Soldier artifact creature tokens.\n• Creatures you control get +2/+0 until end of turn."
newly covered	118220b4f204818eb6a008596e7ef2ecdf260828ad085ab8f20b41fd1a02337d	card "Spawning Breath"	selected_analysis "Spawning Breath deals 1 damage to any target. Create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	11854dd3a865e3cd29971b2cab358c15d6e65f63998ae30080977ca6c291443e	card "Golden Guardian // Gold-Forge Garrison (Gold-Forge Garrison)"	selected_analysis "{T}: Add two mana of any one color.\n{4}, {T}: Create a 4/4 colorless Golem artifact creature token."
newly covered	11d0b38f29c3baa5c9c5ef183dcf032d6d31f899adc6cd8297cce4acba1ca2fb	card "Kozilek's Predator"	selected_analysis "When this creature enters, create two 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	12beded3bad58ea7f76d7483439c82a6374cfbbc83ac3ddf7a1476c1fc8f6bca	card "Rapacious One"	selected_analysis "Trample\nWhenever this creature deals combat damage to a player, create that many 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	13f48db6b6f1d34e80cb9fe8d066c02d41497efc996e4a104099a5b12d9346f7	card "Desolation Twin"	selected_analysis "When you cast this spell, create a 10/10 colorless Eldrazi creature token."
newly covered	1460d76bd0ee2eec79e747f86a4123b2a2459317f32acf0419a9253bba210c96	card "Dust Stalker"	selected_analysis "Devoid\nHaste\nAt the beginning of each end step, if you control no other colorless creatures, return this creature to its owner's hand."
newly covered	15607841fdca6c4bc6a6029566d8a0a89fb3077c27dcae45cf71e66252180859	card "Access Denied"	selected_analysis "Counter target spell. Create X 1/1 colorless Thopter artifact creature tokens with flying, where X is that spell's mana value."
newly covered	15619ff8f655c309c6149a0b0f19a42f7a55b9b8b61caa703bb3f27e9d3b955b	card "Barbed Spike"	selected_analysis "When this Equipment enters, create a 1/1 colorless Thopter artifact creature token with flying, then attach this Equipment to it.\nEquipped creature gets +1/+0.\nEquip {2}"
newly covered	15a4a3ba118b7574bf20287f0c15e4224fe0d20839437105d0cdc9525dc688e1	card "Realm of Koh"	selected_analysis "This land enters tapped unless you control a basic land.\n{T}: Add {B}.\n{3}{B}, {T}: Create a 1/1 colorless Spirit creature token with \"This token can't block or be blocked by non-Spirit creatures.\""
newly covered	15bfa2e13e3491437298809521381893acabc9b5ff8b772242a67056cbcb7c7e	card "Grizzled Angler // Grisly Anglerfish (Grizzled Angler)"	selected_analysis "{T}: Mill two cards. Then if there is a colorless creature card in your graveyard, transform this creature."
newly covered	1622cd52fa535b0ad72f285a9fb3ff055566befb2fbcb7f6bb20f0b0f03f10a9	card "Shatter Assumptions"	selected_analysis "Choose one —\n• Target opponent reveals their hand and discards all colorless nonland cards.\n• Target opponent reveals their hand and discards all multicolored cards."
newly covered	1680af34b45aebd7dcfecdf899d5a8813fb44c2289a09a9e7687c21c9bd7bfdd	card "Scion Summoner"	selected_analysis "Devoid\nWhen this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	16e2b42e0715e97320a6105afd12b4cb3991e52ed94bbed2e53d283eabc7e2cd	card "Efficient Construction"	selected_analysis "Whenever you cast an artifact spell, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	195b31293d1c83ab4e143818530c4cf884c11480588ecb580cfe91496020fba5	card "Dominator Drone"	selected_analysis "Devoid\nIngest\nWhen this creature enters, if you control another colorless creature, each opponent loses 2 life."
newly covered	1c32c4dfde4b3293b214d0e30222dc623ef9f31eb941c4b7d7a55a782414c180	card "Infested Fleshcutter"	selected_analysis "Equipped creature gets +2/+0.\nWhenever equipped creature attacks, create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\"\nEquip {2}{W}"
newly covered	1c9bef850718afef04f2dd9fb08a04daf3d5c3e0e969c145bc569993780c068c	card "Aviation Pioneer"	selected_analysis "When this creature enters, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	1d12aa1175fb5b7764ad687d3a92b1f86f2c236d0b0372fb11949445278d2466	card "Kayla's Command"	selected_analysis "Choose two —\n• Create a 2/2 colorless Construct artifact creature token.\n• Put a +1/+1 counter on a creature you control. It gains double strike until end of turn.\n• Search your library for a basic Plains card, reveal it, put it into your hand, then shuffle.\n• You gain 2 life and scry 2."
newly covered	1deed43d44bc23b89e84fac1249067cc0ca9433ac8741665df525fcbda2cf1b3	card "Urtet, Remnant of Memnarch"	selected_analysis "Whenever you cast a Myr spell, create a 1/1 colorless Myr artifact creature token.\nAt the beginning of combat on your turn, untap each Myr you control.\n{W}{U}{B}{R}{G}, {T}: Put three +1/+1 counters on each Myr you control. Activate only during your turn."
newly covered	1df0a9ee6d9d22fba11b236008a3f01aacd53732d107b59fd06459e758a13430	card "Nettle Drone"	selected_analysis "Devoid\n{T}: This creature deals 1 damage to each opponent.\nWhenever you cast a colorless spell, untap this creature."
newly covered	1ebc0c35c0608f3fc40608f58d3a6e8f0743b44ff47ce7256e12c4a61a695246	card "Pia Nalaar, Consul of Revival"	selected_analysis "Thopters you control have haste.\nWhenever you play a land from exile or cast a spell from exile, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	1eca785e0fb480a5c44cb39ffba2f0af7863f6f0b81073892b11ceeffd43a57b	card "Tinker's Tote"	selected_analysis "When this artifact enters, create two 1/1 colorless Gnome artifact creature tokens.\n{W}, Sacrifice this artifact: You gain 3 life."
newly covered	209d8e0ca7dfe4c152cd89714dc2f1a72172a1510bc0f3263a6c7a4cf0d81415	card "Call the Scions"	selected_analysis "Devoid\nCreate two 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	213535cca0d846db8422556ad98eb5afb8d5db5635260a4d6de4e7b8cee38bab	card "Birthing Boughs"	selected_analysis "{4}, {T}: Create a 2/2 colorless Shapeshifter creature token with changeling."
newly covered	226665bf46990c52125c76b83b5e04851d8096b3639dae676c31de41c60a5e89	card "Noggle the Mind"	selected_analysis "Flash\nEnchant creature\nEnchanted creature loses all abilities and is a colorless Noggle with base power and toughness 1/1."
newly covered	22cd476b224d62f59018dc86d7302b22a9119d90abe5c8b3f50d2f53650f623b	card "Thopter Spy Network"	selected_analysis "At the beginning of your upkeep, if you control an artifact, create a 1/1 colorless Thopter artifact creature token with flying.\nWhenever one or more artifact creatures you control deal combat damage to a player, draw a card."
newly covered	2310fd78db43a370be5055f747198be06353597dea900f23637cff2ad2c3275c	card "Spawning Pit"	selected_analysis "Sacrifice a creature: Put a charge counter on this artifact.\n{1}, Remove two charge counters from this artifact: Create a 2/2 colorless Spawn artifact creature token."
newly covered	252f742c67a231d929088322905a7d0e5273a9227e3f57587bf1055fb91c0d12	card "Biomechan Engineer"	selected_analysis "When this creature enters, create a Lander token.\n{8}: Draw two cards and create a 2/2 colorless Robot artifact creature token."
newly covered	254fcec58bca070905e4531c62f8c8f482374940619d84773405c2fca80ed25a	card "Auxiliary Boosters"	selected_analysis "When this Equipment enters, create a 2/2 colorless Robot artifact creature token and attach this Equipment to it.\nEquipped creature gets +1/+2 and has flying.\nEquip {3}"
newly covered	2639d78710afe4e0a070267a0197c39f0d4c8f8ac9763e8957d975b0b1dda146	card "Digsite Engineer"	selected_analysis "Whenever you cast an artifact spell, you may pay {2}. If you do, create a 0/0 colorless Construct artifact creature token with \"This token gets +1/+1 for each artifact you control.\""
newly covered	26408054d85d7ca2fb69a2a3f459d86901e9ce186ae1db0096fc8c8d13c3262a	card "Eldrazi Skyspawner"	selected_analysis "Devoid\nFlying\nWhen this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	26621fdb792190aec5999d0d44c34f488d109d5ac56c06035768ab69ed7a59c3	card "Hedron Blade"	selected_analysis "Equipped creature gets +1/+1.\nWhenever equipped creature becomes blocked by one or more colorless creatures, it gains deathtouch until end of turn.\nEquip {2}"
newly covered	26f76335f99d48a62351d5bee2574c3afa3c2f43d97e3828a55845ec0ca20930	card "Mouser Foundry"	selected_analysis "When this artifact enters or leaves the battlefield, create a 1/1 colorless Robot artifact creature token.\n{4}{R}, Sacrifice this artifact: It deals 3 damage to target creature."
newly covered	28a780aec197afa55826041a6c963f66f0e6843e5a35d2d8932a62d571cdb9b4	card "Filigree Crawler"	selected_analysis "When this creature dies, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	28d39a49a3d504dd3fcab8bcacb5245ef920188a8e74eec98c6a7e4b800a3664	card "Maul Splicer"	selected_analysis "When this creature enters, create two 3/3 colorless Phyrexian Golem artifact creature tokens.\nGolem creatures you control have trample."
newly covered	28d3c69bd7f8c35ecb096062eb7d872f53e1e1755e4f7d9b20f2b5d74374ef85	card "Launch Mishap"	selected_analysis "Counter target creature or planeswalker spell. Create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	2b2d4e65e0f55c000966ca34161c9897db2f267c01f4ca828a18c7f51380bd1d	card "Jan Jansen, Chaos Crafter"	selected_analysis "Haste\n{T}, Sacrifice an artifact creature: Create two Treasure tokens.\n{T}, Sacrifice a noncreature artifact: Create two 1/1 colorless Construct artifact creature tokens."
newly covered	2be6e9f37a370b7fd7add40ca8a9a3d23ce75c85c095d5b52283587a19667b8e	card "Doomed Artisan"	selected_analysis "Sculptures you control can't attack or block.\nAt the beginning of your end step, create a colorless Sculpture artifact creature token with \"This token's power and toughness are each equal to the number of Sculptures you control.\""
newly covered	2d3154c96fd6d493632881270d1f84274402be5ace6112eee22982578a2abb45	card "Skitterskin"	selected_analysis "Devoid\nThis creature can't block.\n{1}{B}: Regenerate this creature. Activate only if you control another colorless creature."
newly covered	2d6dc64cec350a4ffdbf1a4568e452d716df1227c1d268d10229537c92af6582	card "Blight Herder"	selected_analysis "When you cast this spell, you may put two cards your opponents own from exile into their owners' graveyards. If you do, create three 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	2e76d168945357c3092b015893f1d5afa029aa2f66e5705e68aa3fa61073012e	card "Wall Off"	selected_analysis "This spell costs {1} less to cast for each creature your opponents control.\nCreate a 0/4 colorless Wall creature token with defender. You gain 4 life."
newly covered	2f0579c76093ec67fc7fe09d2a2a51b7b305f8cfc2723c96255ede322c319c76	card "Ghostly Flame"	selected_analysis "Black and/or red permanents and spells are colorless sources of damage."
newly covered	3065d22e3f8ddf3ebfea0363d458ed91d73ffeb9b9a9687cc51b9614a9b04c08	card "Pia and Kiran Nalaar"	selected_analysis "When Pia and Kiran Nalaar enters, create two 1/1 colorless Thopter artifact creature tokens with flying.\n{2}{R}, Sacrifice an artifact: Pia and Kiran Nalaar deals 2 damage to any target."
newly covered	30a4e3a805384c776f76670dc7d6cab0ccc273dddfe673508811dfdaa03de721	card "Barrage Tyrant"	selected_analysis "Devoid\n{2}{R}, Sacrifice another colorless creature: This creature deals damage equal to the sacrificed creature's power to any target."
newly covered	312199b3e61691a022ba392bc23a5850f973a5887328c8c5f4418b5ae16cd0f5	card "Robotics Mastery"	selected_analysis "Flash\nEnchant creature\nWhen this Aura enters, create two 1/1 colorless Robot artifact creature tokens with flying.\nEnchanted creature gets +2/+2."
newly covered	3440a084e7862f010906cba9ec0394fc0573282aad3c5e5cd9ea948d3139e856	card "Cavalier of Dawn"	selected_analysis "Vigilance\nWhen this creature enters, destroy up to one target nonland permanent. Its controller creates a 3/3 colorless Golem artifact creature token.\nWhen this creature dies, return target artifact or enchantment card from your graveyard to your hand."
newly covered	35220af2482bdb1dcd59b917246ec64182297b243c10050b1442d8c210e55826	card "Fuss // Bother (Bother)"	selected_analysis "Create three 1/1 colorless Thopter artifact creature tokens with flying. Surveil 2."
newly covered	36ca8204316c3f8116facf69945951f5f51445076299b9b750d509487989c471	card "Master Trinketeer"	selected_analysis "Servos and Thopters you control get +1/+1.\n{3}{W}: Create a 1/1 colorless Servo artifact creature token."
newly covered	3722aa866b0b30debe01fce08c52f14c96b2b2383a430fd043577cd5d020d154	card "Myrsmith"	selected_analysis "Whenever you cast an artifact spell, you may pay {1}. If you do, create a 1/1 colorless Myr artifact creature token."
newly covered	382d206e9425582b3c4d3116fa7059dbfcfeff717e0ea5759e8b3dd55a137b50	card "Vannifar, Evolved Enigma"	selected_analysis "At the beginning of combat on your turn, choose one —\n• Cloak a card from your hand.\n• Put a +1/+1 counter on each colorless creature you control."
newly covered	3c6b92a538786148b7789a3cca10fcb7e500fcf30479e1cd4a59d24cf85753e7	card "Thought Harvester"	selected_analysis "Devoid\nFlying\nWhenever you cast a colorless spell, target opponent exiles the top card of their library."
newly covered	3d254d5743bee475b5e3c200cbffaf2c593ca888aba9f2d53f6b12bb5e973538	card "Extricator of Sin // Extricator of Flesh (Extricator of Sin)"	selected_analysis "When this creature enters, you may sacrifice another permanent. If you do, create a 3/2 colorless Eldrazi Horror creature token.\nDelirium — At the beginning of your upkeep, if there are four or more card types among cards in your graveyard, transform this creature."
newly covered	3d39a63415c9110f4df50babc16a38dd3ff43dd5640b4f6e4ae03a75fa8f6a54	card "Ancient Stone Idol"	selected_analysis "Flash\nThis spell costs {1} less to cast for each attacking creature.\nTrample\nWhen this creature dies, create a 6/12 colorless Construct artifact creature token with trample."
newly covered	3e7953773c05652d3d17f6feddcf45944a1f525b96ef4c006d2a9aaf91565ed4	card "Kozilek's Sentinel"	selected_analysis "Devoid\nWhenever you cast a colorless spell, this creature gets +1/+0 until end of turn."
newly covered	3ec8790c013ee23f555b379425ca604f10b8452755c3733a863ce397f2eedace	card "Vital Splicer"	selected_analysis "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\n{1}: Regenerate target Golem you control."
newly covered	3ee95e133339e17c4f27606e303cf145cd4813eb05f13fbc274add9b53b47063	card "Indoctrination Attendant"	selected_analysis "Toxic 1\nWhen this creature enters, you may return another permanent you control to its owner's hand. If you do, create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\""
newly covered	3ee98e5eee5ac70b87580b652f4a77594b5a35601d978a2f4aeb4847a433001d	card "Vishgraz, the Doomhive"	selected_analysis "Menace, toxic 1\nWhen Vishgraz enters, create three 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\"\nVishgraz gets +1/+1 for each poison counter your opponents have."
newly covered	40164235f5feb7adc830131c8290ed84ebb11ed467bb5d55aafd2a54f6761c0b	card "Emrakul's Hatcher"	selected_analysis "When this creature enters, create three 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	40d71e9ab7605bfaea3e60bd136daf2b97b1e3905cc42aadafbc4e0e714f90ed	card "Invisible Woman, Sue Storm"	selected_analysis "Lifelink\nWhenever you put one or more +1/+1 counters on one or more other Heroes you control, you may create a 0/4 colorless Wall creature token with defender."
newly covered	41dbafd47fcdf9cc5c64e79583d243000b2a7e5bb192d3f44cca1d6337675a6f	card "Concord with the Kami"	selected_analysis "At the beginning of your end step, choose one or more —\n• Put a +1/+1 counter on target creature with a counter on it.\n• Draw a card if you control an enchanted creature.\n• Create a 1/1 colorless Spirit creature token if you control an equipped creature."
newly covered	41ec1bc14c857363bd4c6d4383089d0402a630ce65c167cb983d2ade26788625	card "Crawling Chorus"	selected_analysis "Toxic 1\nWhen this creature dies, create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\""
newly covered	41fecda7346e60e4594b4897dec5825e4235c863529d153b6f7b24319f8deb4d	card "Glaring Fleshraker"	selected_analysis "Whenever you cast a colorless spell, create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\"\nWhenever another colorless creature you control enters, this creature deals 1 damage to each opponent."
newly covered	4384d809c498fbdff25ccfb1b6bc6c2910b5d8d97913e05004487bd3fdb4c917	card "Sai, Master Thopterist"	selected_analysis "Whenever you cast an artifact spell, create a 1/1 colorless Thopter artifact creature token with flying.\n{1}{U}, Sacrifice two artifacts: Draw a card."
newly covered	439a0c6796170cfebf37830eed403b20ea716563e0d74e117bb8eea4c9fc0c34	card "Flamewright"	selected_analysis "{1}, {T}: Create a 1/1 colorless Construct artifact creature token with defender.\n{T}, Sacrifice a creature with defender: This creature deals 1 damage to any target."
newly covered	442ef949678fe415888704a8d3456908e19f0cc85fcb74324b1da88b7c40e10e	card "Sanctum of Ugin"	selected_analysis "{T}: Add {C}.\nWhenever you cast a colorless spell with mana value 7 or greater, you may sacrifice this land. If you do, search your library for a colorless creature card, reveal it, put it into your hand, then shuffle."
newly covered	4434c3a6522b7b984c6c3a418b62d18cd858100f930cb5c3f3cd9d14e927b434	card "Crib Swap"	selected_analysis "Changeling\nExile target creature. Its controller creates a 1/1 colorless Shapeshifter creature token with changeling."
newly covered	444ffee3a04c59d634ce0a5cc6118722bc0f46b91a47c5e19a24381f91674ebc	card "Metrognome"	selected_analysis "When a spell or ability an opponent controls causes you to discard this card, create four 1/1 colorless Gnome artifact creature tokens.\n{4}, {T}: Create a 1/1 colorless Gnome artifact creature token."
newly covered	44d475af640d3a27df765f5368718f79b60157a95c1d08ee9832ff0dcd220db4	card "Basilica Shepherd"	selected_analysis "Flying\nWhen this creature enters, create two 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\""
newly covered	451216c548304c582af2e98d819daa5223dfeda337a042f315535ce4139ba9bc	card "Drowner of Hope"	selected_analysis "Devoid\nWhen this creature enters, create two 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\"\nSacrifice an Eldrazi Scion: Tap target creature."
newly covered	46b215a73ea5b701b6a771e457c96059c6d938d2effd840438d14a21b1494fbb	card "Myr Turbine"	selected_analysis "{T}: Create a 1/1 colorless Myr artifact creature token.\n{T}, Tap five untapped Myr you control: Search your library for a Myr creature card, put it onto the battlefield, then shuffle."
newly covered	473adf45ee9f74f0d9e8eb102725dd9b16ede71e8e659c164d575729873de080	card "Canoptek Scarab Swarm"	selected_analysis "Flying\nFeeder Mandibles — When this creature enters, exile target player's graveyard. For each artifact or land card exiled this way, create a 1/1 colorless Insect artifact creature token with flying."
newly covered	484861a5a8d9b6784dc0a93e73616c854d1da4552d361dc9dfd204a190f109f9	card "Grave Birthing"	selected_analysis "Devoid\nTarget opponent exiles a card from their graveyard. You create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"\nDraw a card."
newly covered	48e3926c6fef5de701edd4ff2e70271955ebca651ba3cfc058cdb1f0f8244c8e	card "Skittering Cicada"	selected_analysis "Flash\nYou may cast colorless spells as though they had flash.\nWhenever you cast a colorless spell, until end of turn, this creature gains trample and gets +X/+X, where X is that spell's mana value."
newly covered	495cbcb895d3da1a480c1c8581b9fe63c783b73caf539e453b62640718148a14	card "Hammer of Purphoros"	selected_analysis "Creatures you control have haste.\n{2}{R}, {T}, Sacrifice a land: Create a 3/3 colorless Golem enchantment artifact creature token."
newly covered	49fefe0b85536ce97bd98d5dc7d96224639305cd9b2c4db98d059fb91ee39c44	card "Catacomb Sifter"	selected_analysis "Devoid\nWhen this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"\nWhenever another creature you control dies, scry 1."
newly covered	4a6705de4008e7e7b9312f3e6d833081b8f60b45fae8854bd601125715e8c84c	card "Myr Matrix"	selected_analysis "Indestructible\nMyr creatures get +1/+1.\n{5}: Create a 1/1 colorless Myr artifact creature token."
newly covered	4b1cf0cbaeebe8bfd02395496024793d96663e386aeddc2b94f28628337a3b29	card "Melded Moxite"	selected_analysis "When this artifact enters, you may discard a card. If you do, draw two cards.\n{3}, Sacrifice this artifact: Create a tapped 2/2 colorless Robot artifact creature token."
newly covered	4c5768ec3b4b0edf0a5b8f96f71b3919de7a9609074c13121145cd5a510bcc6c	card "Basalt Golem"	selected_analysis "This creature can't be blocked by artifact creatures.\nWhenever this creature becomes blocked by a creature, that creature's controller sacrifices it at end of combat. If the player does, they create a 0/2 colorless Wall artifact creature token with defender."
newly covered	4d1f862003032c59bf60c6117fa2b72d0ffb51466819a97f53d955e5c7422237	card "Molten Nursery"	selected_analysis "Devoid\nWhenever you cast a colorless spell, this enchantment deals 1 damage to any target."
newly covered	4da64720b444030ae51a562c73b1cdb49e4f6149342f867c24a38a05ae6c2c6a	card "Mouser Attack!"	selected_analysis "Choose one —\n• Create a 1/1 colorless Robot artifact creature token.\n• Target creature gets +3/+0 and gains first strike until end of turn."
newly covered	4debedbd27b1cb40d0461180e8e62db07c08f8ce11f2518d57d0a2892d7696a0	card "Goblin Cratermaker"	selected_analysis "{1}, Sacrifice this creature: Choose one —\n• This creature deals 2 damage to target creature.\n• Destroy target colorless nonland permanent."
newly covered	4e5d21c0b0be0de25a891f01312649e480439efe532d49cb76bcdff37f59ba72	card "Pinnacle Emissary"	selected_analysis "Whenever you cast an artifact spell, create a 1/1 colorless Drone artifact creature token with flying and \"This token can block only creatures with flying.\"\nWarp {U/R}"
newly covered	501c0efc91e2cacedfbfa9a512d40c13a567f95be4c8863d83603f807856b838	card "Iron Man, Tony Stark"	selected_analysis "Flying\nAttacking creatures you control get +1/+0.\nWhenever you cast a red spell, create a 2/1 colorless Robot Hero artifact creature token with flying."
newly covered	5047331b6061c59058ca06bc249cf77306b38aba71e19d6107394c5ede68e19e	card "Foggy Swamp Spirit Keeper"	selected_analysis "Lifelink\nWhenever you draw your second card each turn, create a 1/1 colorless Spirit creature token with \"This token can't block or be blocked by non-Spirit creatures.\""
newly covered	53c49cbfac2ddacfb8968b17b3e679216a71d6bba480fbd82c0f75d011438ab9	card "Scrapwork Cohort"	selected_analysis "When this creature enters, create a 1/1 colorless Soldier artifact creature token.\nUnearth {2}{W}"
newly covered	54fe406e24aecc706dc4781bfbe0f659674f97fa7b8332cb42bbe919def3fb9f	card "The Restoration of Eiganjo // Architect of Restoration (Architect of Restoration)"	selected_analysis "Vigilance\nWhenever this creature attacks or blocks, create a 1/1 colorless Spirit creature token."
newly covered	55196c41c92fb0b865aaaea4017a262eab89aca3a73554dd9b50c987aa5f8949	card "Hangarback Walker"	selected_analysis "This creature enters with X +1/+1 counters on it.\nWhen this creature dies, create a 1/1 colorless Thopter artifact creature token with flying for each +1/+1 counter on this creature.\n{1}, {T}: Put a +1/+1 counter on this creature."
newly covered	597c7f80c02f72b49a099035c8a319dc8ea2a22fbddc6216fd6e5ed5e0e21796	card "Honden of Life's Web"	selected_analysis "At the beginning of your upkeep, create a 1/1 colorless Spirit creature token for each Shrine you control."
newly covered	5af4d0c9a3ba0e71df74002f2450928e9ae4e534c65f66daa69293f5280c30cd	card "Clay-Fired Bricks // Cosmium Kiln (Cosmium Kiln)"	selected_analysis "When this artifact enters, create two 1/1 colorless Gnome artifact creature tokens.\nCreatures you control get +1/+1."
newly covered	5b32b32d0390e0be9de49a2a0c788c1769ec6d6683c649c333181925d00d7c50	card "Whirlermaker"	selected_analysis "{4}, {T}: Create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	5b64f1bfc1e6f42cd3bc526ab558bb62872a108a989f7f3d3167cdf0c71d9592	card "Black Market Connections"	selected_analysis "At the beginning of your first main phase, choose one or more —\n• Sell Contraband — Create a Treasure token. You lose 1 life.\n• Buy Information — Draw a card. You lose 2 life.\n• Hire a Mercenary — Create a 3/2 colorless Shapeshifter creature token with changeling. You lose 3 life."
newly covered	5b9cff70180367d66de7d338781bc6157cc5dc0c16c9642e11a05b9480606b50	card "Forbidden Orchard"	selected_analysis "{T}: Add one mana of any color.\nWhenever you tap this land for mana, target opponent creates a 1/1 colorless Spirit creature token."
newly covered	5cf7942da7e004f999498d25c206a0267b6f2383d6ec8d4f1c1725e2f9075385	card "Chittering Dispatcher"	selected_analysis "Devoid\nMyriad\nWhen this creature leaves the battlefield, create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	5e714f3680ba94b754b8a2b69190c2a3868e4216f5bcdb697edab35d70e8f7fb	card "Flayer Drone"	selected_analysis "Devoid\nFirst strike\nWhenever another colorless creature you control enters, target opponent loses 1 life."
newly covered	5ea0d0dc8a2ab6632e6c1445c3a1d89081a6a63b0cdb9035893677708bb5b3d1	card "Stinging Hivemaster"	selected_analysis "Toxic 1\nWhen this creature dies, create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\""
newly covered	5eaee02663b5874da48a5d94de6b82f459824ee885fb5759b0409ffce4d69f8c	card "Personify"	selected_analysis "Exile target creature you control, then return that card to the battlefield under its owner's control. Create a 1/1 colorless Shapeshifter creature token with changeling."
newly covered	5ed6b84f7a43dc9db957ed671bb6d481a4a6b7519bd8476a1fd0d7964f5bdb91	card "Titan Forge"	selected_analysis "{3}, {T}: Put a charge counter on this artifact.\n{T}, Remove three charge counters from this artifact: Create a 9/9 colorless Golem artifact creature token."
newly covered	5f22cd4c1d520649c87c9ae572afda9a59dd571eef06f65439bedb045d4c2494	card "Triskelavus"	selected_analysis "Flying\nThis creature enters with three +1/+1 counters on it.\n{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Triskelavite artifact creature token with flying. It has \"Sacrifice this token: This token deals 1 damage to any target.\""
newly covered	5fc2c89dcdd8ae2a857c68b932754d39d265f8ce64caf199c9a7e13c0d3818a6	card "Release to Memory"	selected_analysis "Exile target opponent's graveyard. For each creature card exiled this way, create a 1/1 colorless Spirit creature token."
newly covered	5ff46575257cd30a51f3c35906887d62f964a70c4f719ece1efa87ff1b1775d6	card "Ultron, Unlimited"	selected_analysis "Flying\nWhenever Ultron attacks, he connives.\nWhenever a creature you control connives, you may pay {1}. If you do, create a 2/2 colorless Robot Villain artifact creature token."
newly covered	60f449df025e59d4926aff35900f463166e8235653197c57fe210c7070904082	card "Extricator of Sin // Extricator of Flesh (Extricator of Flesh)"	selected_analysis "Eldrazi you control have vigilance.\n{2}, {T}, Sacrifice a non-Eldrazi creature: Create a 3/2 colorless Eldrazi Horror creature token."
newly covered	61267a1a87446592161924b3acd499ed37e6b06f8a9d5b28e1a2372cee46b012	card "Station Monitor"	selected_analysis "Whenever you cast your second spell each turn, create a 1/1 colorless Drone artifact creature token with flying and \"This token can block only creatures with flying.\""
newly covered	616e96d667031253fd11db9542e5b70097076cc5934abab81bd1e68a70808041	card "Stalactite Dagger"	selected_analysis "When this Equipment enters, create a 1/1 colorless Shapeshifter creature token with changeling.\nEquipped creature gets +1/+1 and is all creature types.\nEquip {2}"
newly covered	62230bd618916ecd9bbd943289aa3e1955778262e9912622cfeb730be128cad3	card "Adverse Conditions"	selected_analysis "Devoid\nTap up to two target creatures. Those creatures don't untap during their controller's next untap step. Create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	62eaaeeb027b93b09e2ecc82b7d986ac90115c80d7e884214ccf8ce3ddadec74	card "Lost in the Spirit World"	selected_analysis "Return up to one target creature to its owner's hand. Create a 1/1 colorless Spirit creature token with \"This token can't block or be blocked by non-Spirit creatures.\""
newly covered	64f4413afe0293a5e978022cf431172be49cb6995bf2f3d75de8ad5e914ba2a2	card "Sram's Expertise"	selected_analysis "Create three 1/1 colorless Servo artifact creature tokens.\nYou may cast a spell with mana value 3 or less from your hand without paying its mana cost."
newly covered	65cefe218a6b2fcfcdf3ca42fea9375c295d415d3cd9b11a953fa242ea9bde10	card "Vile Aggregate"	selected_analysis "Devoid\nVile Aggregate's power is equal to the number of colorless creatures you control.\nTrample\nIngest"
newly covered	65d7826a98a4b169bc7dd65f595957474f34e9490be271d644d45ba2cc385d0f	card "Sifter of Skulls"	selected_analysis "Devoid\nWhenever another nontoken creature you control dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	67359e54d6f33f1b5f53f74f77e5841ade6385d3fcb48955fcf3c8be862fe9c3	card "Birthing Hulk"	selected_analysis "Devoid\nWhen this creature enters, create two 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\"\n{1}{C}: Regenerate this creature."
newly covered	689494db881a262e25c632dac6749b96096488c70a6fa3c8be30667df2054760	card "Triplicate Titan"	selected_analysis "Flying, vigilance, trample\nWhen this creature dies, create a 3/3 colorless Golem artifact creature token with flying, a 3/3 colorless Golem artifact creature token with vigilance, and a 3/3 colorless Golem artifact creature token with trample."
newly covered	6a1d18db92367ed0edb778718445a5640f468c76706d1ce819111e41f568a255	card "Eye of Ugin"	selected_analysis "Colorless Eldrazi spells you cast cost {2} less to cast.\n{7}, {T}: Search your library for a colorless creature card, reveal it, put it into your hand, then shuffle."
newly covered	6cfca1f57887b93412f6ddd3a00163a33a5ec13031e8a98ad3609d1ea467941a	card "Sly Requisitioner"	selected_analysis "Improvise\nWhenever a nontoken artifact you control is put into a graveyard from the battlefield, create a 1/1 colorless Servo artifact creature token."
newly covered	6f6494739984905dd070a8904607111a3535e98b5d9bb54691b632a05551aedf	card "Ghostfire"	selected_analysis "Ghostfire is colorless.\nGhostfire deals 3 damage to any target."
newly covered	6f84d404194c9210ba21c11d4c980a568c2dd203ff2d489b334e3e51731a4456	card "Mu Yanling, Wind Rider"	selected_analysis "When Mu Yanling enters, create a 3/2 colorless Vehicle artifact token with crew 1.\nVehicles you control have flying.\nWhenever one or more creatures you control with flying deal combat damage to a player, draw a card."
newly covered	704623bbc3cdeb1a12a58b3eb599bba24f64a0c57ff419f05132f30194ab38b2	card "Simulacrum Synthesizer"	selected_analysis "When this artifact enters, scry 2.\nWhenever another artifact you control with mana value 3 or greater enters, create a 0/0 colorless Construct artifact creature token with \"This token gets +1/+1 for each artifact you control.\""
newly covered	71bf4b1a43d131408ec005adddb3f8f2e1b2222829ca92f9e990585a44680f6f	card "Sandstorm Salvager"	selected_analysis "When this creature enters, create a 3/3 colorless Golem artifact creature token.\n{2}, {T}: Put a +1/+1 counter on each creature token you control. They gain trample until end of turn."
newly covered	72637774fa10eeda006f52c04810b4ae1b2e055dcf820c15037480bffa4f792f	card "Thopter Fabricator"	selected_analysis "Flying\nWhenever you draw your second card each turn, create a 1/1 colorless Thopter artifact creature token with flying.\nCrew 2"
newly covered	7380483e95864a6b1309ab884028d7c68d0ec616c6a58f2be86759c785bea85b	card "Dread Drone"	selected_analysis "When this creature enters, create two 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	738851481fd36fdc2f4dfa5b7f1c378979b34e194a2462793069a68628232fbf	card "Nuisance Engine"	selected_analysis "{2}, {T}: Create a 0/1 colorless Pest artifact creature token."
newly covered	73b3b660ee602777237a15455a34aabea4df6ce5ce7bd46a1921faa39096b654	card "Brood Sliver"	selected_analysis "Whenever a Sliver deals combat damage to a player, its controller may create a 1/1 colorless Sliver creature token."
newly covered	758da4a67a2b71737ce73d1721d9adc5baeaaf3bc45d43596d5af6c822aab589	card "Master's Call"	selected_analysis "Create two 1/1 colorless Myr artifact creature tokens."
newly covered	7592956324ae0fb4b6a57f8d0c59e9525113abac2afe5358f4338d650eb4ae99	card "Origin Spellbomb"	selected_analysis "{1}, {T}, Sacrifice this artifact: Create a 1/1 colorless Myr artifact creature token.\nWhen this artifact is put into a graveyard from the battlefield, you may pay {W}. If you do, draw a card."
newly covered	77f725cd152b3db72c14a1d8fb5b2c879d5e189a8ad7d3bab00b1c1d371f8272	card "Sky Scourer"	selected_analysis "Devoid\nFlying\nWhenever you cast a colorless spell, this creature gets +1/+0 until end of turn."
newly covered	78810fee49a663bcafe8139e7cc468953056cfd264569790e08ce0b2af79e133	card "Oltec Matterweaver"	selected_analysis "Whenever you cast a creature spell, choose one —\n• Create a 1/1 colorless Gnome artifact creature token.\n• Create a token that's a copy of target artifact token you control."
newly covered	78e946b985f7a1fb0c50c756c9f4b1cc0060aa58710d86a1a5b90c18e2ec4b6c	card "Irregular Cohort"	selected_analysis "Changeling\nWhen this creature enters, create a 2/2 colorless Shapeshifter creature token with changeling."
newly covered	7a03a5a893a53098551d5f8993919535f03f543577fe6d30882ac86adc21c446	card "Thopter Assembly"	selected_analysis "Flying\nAt the beginning of your upkeep, if you control no Thopters other than this creature, return this creature to its owner's hand and create five 1/1 colorless Thopter artifact creature tokens with flying."
newly covered	7a2f038d96224a351a5f85aa1532e06a04b0e1f07dd5faf8b9e8c26a2ab8360f	card "Blisterpod"	selected_analysis "Devoid\nWhen this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	7b284aa77d27733af431cbfeb273b3d871f6def04248064bb3fd9f6ea583c66d	card "Desculpting Blast"	selected_analysis "Return target nonland permanent to its owner's hand. If it was attacking, create a 1/1 colorless Drone artifact creature token with flying and \"This token can block only creatures with flying.\""
newly covered	7bf6f8ecd71b02ae8d919076f06ad1360d50a4c5061112981e85bbe5cd37af86	card "Spider-Slayer, Hatred Honed"	selected_analysis "Whenever Spider-Slayer deals damage to a Spider, destroy that creature.\n{6}, Exile this card from your graveyard: Create two tapped 1/1 colorless Robot artifact creature tokens with flying."
newly covered	7c94ac52f0b99bb71452d92aa0c123fee19374e289409b652fce972741afc111	card "Herald of Kozilek"	selected_analysis "Devoid\nColorless spells you cast cost {1} less to cast."
newly covered	7e12660a61c8f17d3235fc75f2573eebc16df62477fdb5f9c4f83e48525c7535	card "Wurmcoil Engine"	selected_analysis "Deathtouch, lifelink\nWhen this creature dies, create a 3/3 colorless Phyrexian Wurm artifact creature token with deathtouch and a 3/3 colorless Phyrexian Wurm artifact creature token with lifelink."
newly covered	7e6993585c25ecc8edb28331fb76fa3f9989aac24babd579940b067308a66e30	card "Servo Schematic"	selected_analysis "When this artifact enters or is put into a graveyard from the battlefield, create a 1/1 colorless Servo artifact creature token."
newly covered	7e6f860cf8f901ee3374338d030f10585d6f875c74cb8e0ee9981f0befbec59d	card "Urza's Factory"	selected_analysis "{T}: Add {C}.\n{7}, {T}: Create a 2/2 colorless Assembly-Worker artifact creature token."
newly covered	7eda7f4d56c10f281011b081c80bee008e34e7dc00f98d9055cadfb22984c15a	card "Aerith Rescue Mission"	selected_analysis "Choose one —\n• Take the Elevator — Create three 1/1 colorless Hero creature tokens.\n• Take 59 Flights of Stairs — Tap up to three target creatures. Put a stun counter on one of them."
newly covered	7ef367963bba2769b60e6339d80809e233724c4bf82a6e85abbb2d20e431fecd	card "Myr Sire"	selected_analysis "When this creature dies, create a 1/1 colorless Phyrexian Myr artifact creature token."
newly covered	7f9a2b010dcfe0075505e0d86ce89c8fc8072a5136d68f954a6e74469a708e43	card "Ghostflame Sliver"	selected_analysis "All Slivers are colorless."
newly covered	7fdd89c1ae852726761f2de8ef027c9f2255348db0c97b5f883aaf335ab6e713	card "Skystrike Officer"	selected_analysis "Flying\nWhenever this creature attacks, create a 1/1 colorless Soldier artifact creature token.\nTap three untapped Soldiers you control: Draw a card."
newly covered	805d3613c348b97776c9d6c3ca21c4be6b3abe1b2664223afae7fef3698b232f	card "Daretti, Ingenious Iconoclast"	selected_analysis "[+1]: Create a 1/1 colorless Construct artifact creature token with defender.\n[−1]: You may sacrifice an artifact. If you do, destroy target artifact or creature.\n[−6]: Choose target artifact card in a graveyard or artifact on the battlefield. Create three tokens that are copies of it."
newly covered	80852574b8242c472707d72a4d6e8b9eb2cf7f1b1bd6b4559d22d4596c01d52e	card "Ravenous Robots"	selected_analysis "Whenever you cast an artifact spell, create a 1/1 colorless Robot artifact creature token.\n{R}, {T}: Creature tokens you control gain haste until end of turn."
newly covered	8402ef1576122f425eda58b52671168c4479f06dc543c0abe3ecd46346c86f0d	card "Hive Stirrings"	selected_analysis "Create two 1/1 colorless Sliver creature tokens."
newly covered	851810f782dbc93cabb8b8a8d82e10684410e41049d8b502215980264e5457ec	card "Malevolent Rumble"	selected_analysis "Reveal the top four cards of your library. You may put a permanent card from among them into your hand. Put the rest into your graveyard. Create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	8536ef42d6f510311d400a338e60c6665ef7ff80c0b2c411b35632a39a2ab296	card "Ich-Tekik, Salvage Splicer"	selected_analysis "When Ich-Tekik enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\nWhenever an artifact is put into a graveyard from the battlefield, put a +1/+1 counter on Ich-Tekik and a +1/+1 counter on each Golem you control.\nPartner"
newly covered	859a668e65c8acc55f0e8bc21d20cb2fb9031fde6d6a3629329af5ae0d58ec6d	card "Servo Exhibition"	selected_analysis "Create two 1/1 colorless Servo artifact creature tokens."
newly covered	885ad4e61fc0af110b65c5477f7160c24033987b8b36173aa4784567bcce83c6	card "Aspiring Aeronaut"	selected_analysis "Flying\nWhen this creature enters, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	8944272e14b13c8b8a3b7f378dec628d7fa5b4e0e2674d7d932eb38e4280b53a	card "Dwarven Castle Guard"	selected_analysis "When this creature dies, create a 1/1 colorless Hero creature token."
newly covered	89df76877b215675ffc4358fd1b11ef2b0f6ff0bf7f060c8124c72788855bfaa	card "Brood Butcher"	selected_analysis "Devoid\nWhen this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"\n{B}{G}, Sacrifice a creature: Target creature gets -2/-2 until end of turn."
newly covered	89e91b01c662c3b8a06833df92d075c5220385705621e44605125ee913d8071a	card "Gruesome Slaughter"	selected_analysis "Until end of turn, colorless creatures you control gain \"{T}: This creature deals damage equal to its power to target creature.\""
newly covered	8a1b9a20a2b4ccdf6c04b93ee53c6ccbe336a38d69dd235375ea82e3b206c98f	card "Blazing Blade Askari"	selected_analysis "Flanking\n{2}: This creature becomes colorless until end of turn."
newly covered	8b3d3da7b68d616e3df47cab44b612074ddb5f741425471dc3b94f0b383d2027	card "Oltec Cloud Guard"	selected_analysis "Flying\nWhen this creature enters, create a 1/1 colorless Gnome artifact creature token."
newly covered	8c6ac23e86557657decd87bdc6d8a7f4b8e76692ad129c4a769e51dfb1714833	card "Thopter Engineer"	selected_analysis "When this creature enters, create a 1/1 colorless Thopter artifact creature token with flying.\nArtifact creatures you control have haste."
newly covered	8cf4a80f1d983eabfbf1464c78c85ab02146dc2f9a0035f3878f4e2a6f396452	card "Ceremonious Rejection"	selected_analysis "Counter target colorless spell."
newly covered	8d255c82e01e51a155bf6e355a3eb0367c19baace2ac0704a35a254b634958d0	card "Reaver Drone"	selected_analysis "Devoid\nAt the beginning of your upkeep, you lose 1 life unless you control another colorless creature."
newly covered	8dd863dab5216f7344d4613641d45a811f1c5cb773614ce129d10999fef0a0a0	card "Conversion Chamber"	selected_analysis "{2}, {T}: Exile target artifact card from a graveyard. Put a charge counter on this artifact.\n{2}, {T}, Remove a charge counter from this artifact: Create a 3/3 colorless Phyrexian Golem artifact creature token."
newly covered	8e1a40bf393d0818bfb423d310259d0c1fe3aee520418cc902d244de336fe09a	card "Doctor Spectrum"	selected_analysis "Flying\nWhen Doctor Spectrum enters, choose one —\n• Create a 0/4 colorless Wall creature token with defender.\n• Put a +1/+1 counter on each other Hero you control.\n• Destroy target enchantment."
newly covered	8f5bdb62de58fa8abc4acda1ed710e52778c818ad1b408c027cf442e959c55fd	card "Sami, Ship's Engineer"	selected_analysis "At the beginning of your end step, if you control two or more tapped creatures, create a tapped 2/2 colorless Robot artifact creature token."
newly covered	8fec477eeb9403f63702dcc970221c2d963c33e254b3198851c47c5374474b89	card "Etherium Spinner"	selected_analysis "Whenever you cast a spell with mana value 4 or greater, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	9010caefe2771592f234d2d05a9d24973fbe92e5b62f1e0e70ca7f618fb391ef	card "Experimental Aviator"	selected_analysis "Flying\nWhen this creature enters, create two 1/1 colorless Thopter artifact creature tokens with flying."
newly covered	91187c776a179bce5e8451a48312c13b97573ca8dcf1f02d2428e193da40586b	card "Skittering Precursor"	selected_analysis "Devoid\nMenace\nWhenever you sacrifice a nontoken permanent, create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	917c350b11e4236fa49f5b846f313ee0a0450bb209ec668e08195ee23acb4746	card "Writhing Chrysalis"	selected_analysis "Devoid\nWhen you cast this spell, create two 0/1 colorless Eldrazi Spawn creature tokens with \"Sacrifice this token: Add {C}.\"\nReach\nWhenever you sacrifice another Eldrazi, put a +1/+1 counter on this creature."
newly covered	924ad50594ca32c67284deb91df45e0cac60c82d916871242cceffb3679cd547	card "Mechanized Ninja Cavalry"	selected_analysis "When this creature enters, create a 1/1 colorless Robot artifact creature token."
newly covered	92790432acdcc2fea985756f421fb94bc5b8fc77e38916abfbb10916fd4c9cd7	card "Conscripted Infantry"	selected_analysis "When this creature dies, create a 1/1 colorless Soldier artifact creature token."
newly covered	96eb0757add139838b4b43a513424bc3edd9b9c91342e8dc6ac7dfe3206ff28e	card "Gravpack Monoist"	selected_analysis "Flying\nWhen this creature dies, create a tapped 2/2 colorless Robot artifact creature token."
newly covered	9951f62e0580ccb10bb56ef044533baf8a7241268d806a953b0f8d0c31752280	card "Renegade's Getaway"	selected_analysis "Target permanent gains indestructible until end of turn. Create a 1/1 colorless Servo artifact creature token."
newly covered	9a7f1cf90cfac4f423cf9ea2891ad577febd324ff73dbda0cc2840c61d54454f	card "Teachings of the Kirin // Kirin-Touched Orochi (Teachings of the Kirin)"	selected_analysis "I — Mill three cards. Create a 1/1 colorless Spirit creature token.\nII — Put a +1/+1 counter on target creature you control.\nIII — Exile this Saga, then return it to the battlefield transformed under your control."
newly covered	9ac3b9707e1f8c7f8fddb86369dbcdcdd59c62439e9b768eebf4a66e921a0347	card "Unfathomable Truths"	selected_analysis "Devoid\nDraw three cards and create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	9b15277c508db03823ac99147cfa4d1136292842592c7ab4829642ee05661d28	card "Golem Foundry"	selected_analysis "Whenever you cast an artifact spell, you may put a charge counter on this artifact.\nRemove three charge counters from this artifact: Create a 3/3 colorless Golem artifact creature token."
newly covered	9b5172bd1ebd8f3dad510c24f96a79c1f428e9d239d6d7c78c8bdc128fc8e5f2	card "The Crystal's Chosen"	selected_analysis "Create four 1/1 colorless Hero creature tokens. Then put a +1/+1 counter on each creature you control."
newly covered	9d9591547e21e86de8a9939284c73e7cc2eabd9d4b744e0236a188f7d94dbb64	card "Tide Drifter"	selected_analysis "Devoid\nOther colorless creatures you control get +0/+1."
newly covered	9e8ca72f0c219cfd240c7966ab2a61e4def6cfa2f926e9fd942c223b83f1c190	card "Eyeless Watcher"	selected_analysis "Devoid\nWhen this creature enters, create two 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	9fa2cc98c0b85e47950acefd1602e6193c5d47c3287193a87d90ba6e7dcb7b04	card "Third Path Iconoclast"	selected_analysis "Whenever you cast a noncreature spell, create a 1/1 colorless Soldier artifact creature token."
newly covered	a2b7663472f95b7050cc27e65ef5c9822332426fcba43e05637ba5a404ff7051	card "Raging Spirit"	selected_analysis "{2}: This creature becomes colorless until end of turn."
newly covered	a3183a366019fbf50009443622d6363b36e8952a568781dab7ae32a359674e89	card "Sliver Queen"	selected_analysis "{2}: Create a 1/1 colorless Sliver creature token."
newly covered	a3e394df596ee5adea50278dd08f566d9aed157d16474b7c1552ca5009d30958	card "Abstruse Interference"	selected_analysis "Devoid\nCounter target spell unless its controller pays {1}. You create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	a4086dfee49623fb840f5b63d57cb78efd0203b24e52d68c3591fdeb4039dfa0	card "Super-Skrull"	selected_analysis "Flying\n{2}{W}: Create a 0/4 colorless Wall creature token with defender.\n{3}{G}: Super-Skrull gets +4/+4 until end of turn.\n{4}{R}: Super-Skrull deals 4 damage to target creature.\n{5}{U}: Target player draws four cards."
newly covered	a6ee0428c597f076486928dfb4cad1bf470125f65cf2709c7ddc0a9696642ec2	card "Dune-Brood Nephilim"	selected_analysis "Whenever this creature deals combat damage to a player, create a 1/1 colorless Sand creature token for each land you control."
newly covered	a7caa3b44eb6d1e70f1353ae13013963473bc589ecb7213f20f77b79ae8ab2b1	card "Magitek Armor"	selected_analysis "When this Vehicle enters, create a 1/1 colorless Hero creature token.\nCrew 1"
newly covered	a8a4946e2d71737ee99b3082e1aa6daf4f53db5bd35ecad06eb3529fae77cb3b	card "Diamond Kaleidoscope"	selected_analysis "{3}, {T}: Create a 0/1 colorless Prism artifact creature token.\nSacrifice a Prism token: Add one mana of any color."
newly covered	a8edb99ab3b39b65e4925f4ec824d67cca67e75ccf9f6238372b796501f3a6b8	card "Corpsehatch"	selected_analysis "Destroy target nonblack creature. Create two 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	a9fbd9efa2cd4d79ca51e7739aaa6945c3295cc43c15e8fc12e978e4d1788386	card "Cogworker's Puzzleknot"	selected_analysis "When this artifact enters, create a 1/1 colorless Servo artifact creature token.\n{1}{W}, Sacrifice this artifact: Create a 1/1 colorless Servo artifact creature token."
newly covered	ab0ea60b7d0bd81dbb40a585d0bdc2569ba265e47efb91b67e1640c5b02aa0db	card "Sekki, Seasons' Guide"	selected_analysis "Sekki enters with eight +1/+1 counters on it.\nIf damage would be dealt to Sekki, prevent that damage, remove that many +1/+1 counters from Sekki, and create that many 1/1 colorless Spirit creature tokens.\nSacrifice eight Spirits: Return this card from your graveyard to the battlefield."
newly covered	ab565e80e1be67e23ffe72a2dbc947fcd90a6b2271453247218b645048c5b9e5	card "Skittering Invasion"	selected_analysis "Create five 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	adb9a1c8054b60859dcc84e3dcc8a5825d5f8b0f3408434a1a6e9afb01e007d7	card "Platoon Dispenser"	selected_analysis "At the beginning of your end step, if you control two or more other creatures, draw a card.\n{3}{W}: Create a 1/1 colorless Soldier artifact creature token.\nUnearth {2}{W}{W}"
newly covered	ae6b1c7cf5815803390ca40bb7dcbe208aa41bb31d270f5c6123e8b822adb6a1	card "Ruination Guide"	selected_analysis "Devoid\nIngest\nOther colorless creatures you control get +1/+0."
newly covered	b14c183938b8d3c4d79c0d7de75922d14e23d11d240f1502b2189cd1a9a00e8e	card "Myrel, Shield of Argive"	selected_analysis "During your turn, your opponents can't cast spells or activate abilities of artifacts, creatures, or enchantments.\nWhenever Myrel attacks, create X 1/1 colorless Soldier artifact creature tokens, where X is the number of Soldiers you control."
newly covered	b162ebf0a0a3f1d79938f4f4c6a12896a6f61b9ee5a8ce92d52bba5c7e0347ab	card "Stone Idol Trap"	selected_analysis "This spell costs {1} less to cast for each attacking creature.\nCreate a 6/12 colorless Construct artifact creature token with trample. Exile it at the beginning of your next end step."
newly covered	b23c482c61935d2f1e7f094a09703efecfc0d8fba590710d50c3697be93f01d3	card "Nest Invader"	selected_analysis "When this creature enters, create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	b4a0c436a968c37900bd66e804a50fb81ffccbd778133030d3d4170cc30a682b	card "Glitch Interpreter"	selected_analysis "When this creature enters, if you control no face-down permanents, return this creature to its owner's hand and manifest dread.\nWhenever one or more colorless creatures you control deal combat damage to a player, draw a card."
newly covered	b617f1a9ff76d747b8596d4302a6e8b7d5483958b2df965e6c12d6c382cd2937	card "Growth Spasm"	selected_analysis "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. Create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	b6a865207d9ba50bbbb4fd32727714d0a420bddc01af87e42ac4219a2e8b3fa6	card "The Birth of Meletis"	selected_analysis "I — Search your library for a basic Plains card, reveal it, put it into your hand, then shuffle.\nII — Create a 0/4 colorless Wall artifact creature token with defender.\nIII — You gain 2 life."
newly covered	b971ae5f5de6717ee4a1815a9e873ebf3e8c132a7fbb5b09d59f6a668470b81d	card "Infernal Reckoning"	selected_analysis "Exile target colorless creature. You gain life equal to its power."
newly covered	ba0aedb7ebf9919f259b26070a3aadfa13a2e2452066538dd1620c0f92652dd7	card "Propagator Drone"	selected_analysis "Devoid\nCreature tokens you control have evolve.\n{3}{G}: Create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	ba63e8e0a1429f48e304c55e9862896e1129a25c523a3222985583b64e9fcbbf	card "Ulamog's Dreadsire"	selected_analysis "Vigilance\nWard—Sacrifice a permanent with mana value 1 or greater.\n{T}: Create a 10/10 colorless Eldrazi creature token."
newly covered	bb0a95db4a009a62062314d26f21a91bbbaa96c5e567a63210b6235195709e98	card "Curator Beastie"	selected_analysis "Reach\nColorless creatures you control enter with two additional +1/+1 counters on them.\nWhenever this creature enters or attacks, manifest dread."
newly covered	bbe56778884cdcf8771a2ebda34fa5b9fe73326c38b5c06c479614228e5a64eb	card "Springleaf Parade"	selected_analysis "When this enchantment enters, create X 1/1 colorless Shapeshifter creature tokens with changeling.\nCreature tokens you control have \"{T}: Add one mana of any color.\""
newly covered	bc268fc74fd41dcfb505ba14029f098e24ccbc5ded3adbfdc6630bd9ea3e0439	card "Foundry of the Consuls"	selected_analysis "{T}: Add {C}.\n{5}, {T}, Sacrifice this land: Create two 1/1 colorless Thopter artifact creature tokens with flying."
newly covered	bc560631d8d3c3c1bdfdfc5e2c26225e1af6c60e5b1d8861b90af21d826b1d55	card "Tomb of the Spirit Dragon"	selected_analysis "{T}: Add {C}.\n{2}, {T}: You gain 1 life for each colorless creature you control."
newly covered	bd25da9b1cd7c7b49e7d728553340bd9e37c723c71834b4de783e5a7cc193ec1	card "From Beyond"	selected_analysis "Devoid\nAt the beginning of your upkeep, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"\n{1}{G}, Sacrifice this enchantment: Search your library for an Eldrazi card, reveal it, put it into your hand, then shuffle."
newly covered	be38a03a09b78fc7e2f3259ccbad62202dacd30111f789c6e5819ac348adadbd	card "Emergency Weld"	selected_analysis "Return target artifact or creature card from your graveyard to your hand. Create a 1/1 colorless Soldier artifact creature token."
newly covered	be534f4edc545b0e111aa1fe9bb73020729087f093b969e59439fcecc91c0d9b	card "Myr Incubator"	selected_analysis "{6}, {T}, Sacrifice this artifact: Search your library for any number of artifact cards, exile them, then create that many 1/1 colorless Myr artifact creature tokens. Then shuffle."
newly covered	beb63eb007ac128060579d0259116c5896b25489a99ae0f1342f496b4a5b019b	card "Whirler Rogue"	selected_analysis "When this creature enters, create two 1/1 colorless Thopter artifact creature tokens with flying.\nTap two untapped artifacts you control: Target creature can't be blocked this turn."
newly covered	bf9f2de3568fbf6c65bcbe596a4c8a50b3965513d70470f5f4c02565bab21414	card "Promise of Bunrei"	selected_analysis "When a creature you control dies, sacrifice this enchantment. If you do, create four 1/1 colorless Spirit creature tokens."
newly covered	c13003687a78acb8e9bca3aefcc67ad4776825f3a56b6780b8150c43171d71e8	card "Retrofitter Foundry"	selected_analysis "{3}: Untap this artifact.\n{2}, {T}: Create a 1/1 colorless Servo artifact creature token.\n{1}, {T}, Sacrifice a Servo: Create a 1/1 colorless Thopter artifact creature token with flying.\n{T}, Sacrifice a Thopter: Create a 4/4 colorless Construct artifact creature token."
newly covered	c140f1b2e031663eb9180ab78a2f03f9da9727bbf79409932b77dc3bcc33a032	card "Kozilek, the Broken Reality"	selected_analysis "When you cast this spell, up to two target players each manifest two cards from their hands. For each card manifested this way, you draw a card.\nOther colorless creatures you control get +3/+2."
newly covered	c1d0abc3ab807af20fb45d90cefaa6f4c1a546633e25665c1e6af60bf673bd1d	card "Harried Dronesmith"	selected_analysis "At the beginning of combat on your turn, create a 1/1 colorless Thopter artifact creature token with flying. It gains haste until end of turn. Sacrifice it at the beginning of your next end step."
newly covered	c35de4ffd2b0d7df692b222539ae2d6f27087fd062cbf1500ae7bba2157a3252	card "Belonging"	selected_analysis "When this creature enters, create three 1/1 colorless Shapeshifter creature tokens with changeling.\nEncore {6}{W}{W}"
newly covered	c4143a9ec467bdf25f513e1dd675faa8498fda7ab5f35c796d321aac9aa64951	card "Baboon Spirit"	selected_analysis "Whenever another nontoken Spirit you control enters, create a 1/1 colorless Spirit creature token with \"This token can't block or be blocked by non-Spirit creatures.\"\n{3}{U}: Exile another target creature you control. Return it to the battlefield under its owner's control at the beginning of the next end step."
newly covered	c440773b04f46cd563440a8014c99205940e7a04d343f24d197bd96688a177b1	card "Invasion of Kaladesh // Aetherwing, Golden-Scale Flagship (Invasion of Kaladesh)"	selected_analysis "When this Siege enters, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	c61b683155d0dceacee530e407efe68169c42513f0a91171d0fa0712b524e05d	card "Gods' Eye, Gate to the Reikai"	selected_analysis "{T}: Add {C}.\nWhen Gods' Eye is put into a graveyard from the battlefield, create a 1/1 colorless Spirit creature token."
newly covered	c64ba973840e9e1fefebecde2f35739a93941c9ec61d7179ee248e300beafb96	card "Baxter Stockman"	selected_analysis "When Baxter Stockman enters, create a 1/1 colorless Robot artifact creature token.\nAt the beginning of combat on your turn, target artifact creature you control gets +3/+0 and gains first strike and vigilance until end of turn."
newly covered	c7c7fe236e1ecef57a62129268b8b515f3af8da93ecb0e1301422169c78b01ae	card "Searchlight Companion"	selected_analysis "Flying\nWhen this creature enters, create a 1/1 colorless Spirit creature token."
newly covered	c840c882be7b713690582938105471d5c783038ae3ea6ba54044001aa90558c3	card "Envoy of Okinec Ahau"	selected_analysis "{4}{W}: Create a 1/1 colorless Gnome artifact creature token."
newly covered	c8b2bea7f64af643146e9004e7577ed7ce51dec047f179d54d2b9ec4e0a870b7	card "Pawn of Ulamog"	selected_analysis "Whenever this creature or another nontoken creature you control dies, you may create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	cc1ace8bedc9eeb462556ae3140d8c591a121933b22423cfa350debf32f9df4c	card "Summoning Station"	selected_analysis "{T}: Create a 2/2 colorless Pincher creature token.\nWhenever an artifact is put into a graveyard from the battlefield, you may untap this artifact."
newly covered	cc6c712382b100491336e3e2f631b6416ed911e70be5771806604fe9a047d2c5	card "Ghirapur Gearcrafter"	selected_analysis "When this creature enters, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	ce09e7bd9f7f79cfc47484d63b4ee590027a1b32ad23be4b558076a2575d918c	card "Sokenzan, Crucible of Defiance"	selected_analysis "{T}: Add {R}.\nChannel — {3}{R}, Discard this card: Create two 1/1 colorless Spirit creature tokens. They gain haste until end of turn. This ability costs {1} less to activate for each legendary creature you control."
newly covered	ce8aa0b8f600f00e3a6f7199640b780a3446900176a671d202adf07346b94ba4	card "Moonlace"	selected_analysis "Target spell or permanent becomes colorless."
newly covered	cec4d5c44ca6517cbc43e9a2be9d070dc4bda1f27ebd7f6d6a4ef8699714d2ca	card "Shuri, Vibranium Technologist"	selected_analysis "Vigilance\nWhen Shuri enters, choose one —\n• Create a 1/1 colorless Robot Hero artifact creature token with flying.\n• Draw a card."
newly covered	d0b86a5d3fedfc726a911e96a48a9a2c5a3efb46205be1e709b95d62d66286a9	card "Forerunner of Slaughter"	selected_analysis "Devoid\n{1}: Target colorless creature gains haste until end of turn."
newly covered	d1647c82e3eee78a98fbd90822febc564b2735207bbafe61e32f34c5897f1363	card "Ashnod, Flesh Mechanist"	selected_analysis "Deathtouch\nWhenever Ashnod attacks, you may sacrifice another creature. If you do, create a tapped Powerstone token.\n{5}, Exile a creature card from your graveyard: Create a tapped 3/3 colorless Zombie artifact creature token."
newly covered	d166b568ed82da6100e8195c4d5710c786ec8c20b824668ad36fc353fe09da97	card "Inspired Sphinx"	selected_analysis "Flying\nWhen this creature enters, draw cards equal to the number of opponents you have.\n{3}{U}: Create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	d35fcac0570781a1f31fe5e98e054481a458b906557063b7aeaf48d087deeabc	card "Dragoon's Wyvern"	selected_analysis "Flying\nWhen this creature enters, create a 1/1 colorless Hero creature token."
newly covered	d5fa3a9ddad49f7e0a712f321ce27817db1bd7653b9329782873d1e2428df542	card "Carrier Thrall"	selected_analysis "When this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	da3cd6d9403bb1c68718f866a4d26f05272f707830ba1ea1b4b87605dd0068de	card "It That Heralds the End"	selected_analysis "Colorless spells you cast with mana value 7 or greater cost {1} less to cast.\nOther colorless creatures you control get +1/+1."
newly covered	dcb357d65ee0035ed54c068f9920625edad99d24f8b39d72f3d121f24513e037	card "Pentavus"	selected_analysis "This creature enters with five +1/+1 counters on it.\n{1}, Remove a +1/+1 counter from this creature: Create a 1/1 colorless Pentavite artifact creature token with flying.\n{1}, Sacrifice a Pentavite: Put a +1/+1 counter on this creature."
newly covered	dd4f75274d18799d245caa6d8cc0f22f8502bfdcc26f04679a73ce9b34588ae4	card "Zanarkand, Ancient Metropolis // Lasting Fayth (Lasting Fayth)"	selected_analysis "Create a 1/1 colorless Hero creature token. Put a +1/+1 counter on it for each land you control."
newly covered	dd6909d6dfb151d154562e0132cc1abea7e33c9609bdbbb2ab48b45d76784d54	card "Thran Lens"	selected_analysis "All permanents are colorless."
newly covered	ddd7fda834fff3f9b6de67bcb6c28e86d5ad9daa7c72ebecdb52cb612f256b45	card "Essence Feed"	selected_analysis "Target player loses 3 life. You gain 3 life and create three 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	ddf2b6d7684bce72b3db71f1fb61699843c1438413bf326e150edf7fa3eff951	card "Blade Splicer"	selected_analysis "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\nGolems you control have first strike."
newly covered	decd8aa25a9196176ce1bac24fed607c774a624b977a476871ed84ea9854965d	card "Darksteel Splicer"	selected_analysis "Whenever this creature or another nontoken Phyrexian you control enters, create X 3/3 colorless Phyrexian Golem artifact creature tokens, where X is the number of opponents you have.\nGolems you control have indestructible."
newly covered	dffb21b90dfb620627a197794b486fd1e46100c9621df6765a4a6ce60ce68736	card "Siege Veteran"	selected_analysis "At the beginning of combat on your turn, put a +1/+1 counter on target creature you control.\nWhenever another nontoken Soldier you control dies, create a 1/1 colorless Soldier artifact creature token."
newly covered	e06679e7bc15a8806e2cd1f6705028f3cb0f4f2439362ab103edbb1b1850db43	card "Legion Extruder"	selected_analysis "When this artifact enters, it deals 2 damage to any target.\n{2}, {T}, Sacrifice another artifact: Create a 3/3 colorless Golem artifact creature token."
newly covered	e1024929dd8187e84c97151b8b3aa3952919d6a7651316dc0656e707eb834015	card "Path of Annihilation"	selected_analysis "Devoid\nWhen this enchantment enters, create two 0/1 colorless Eldrazi Spawn creature tokens with \"Sacrifice this token: Add {C}.\"\nEldrazi you control have \"{T}: Add one mana of any color.\"\nWhenever you cast a creature spell with mana value 7 or greater, you gain 4 life."
newly covered	e2dafba36fea050bfbf16ded72611d3a3c39675d511ea1b937aec84e940b6623	card "Mass Production"	selected_analysis "Create four 1/1 colorless Soldier artifact creature tokens."
newly covered	e4162f4b7334dd35ecbddb66f8d7b0ce846da693ae53cebc20795e16c3ad298b	card "Wharf Infiltrator"	selected_analysis "Skulk\nWhenever this creature deals combat damage to a player, you may draw a card. If you do, discard a card.\nWhenever you discard a creature card, you may pay {2}. If you do, create a 3/2 colorless Eldrazi Horror creature token."
newly covered	e6e48307fc5c57de899dfa42965c2b7e27ca2be6b93eccc842138c9907165e2d	card "Genesis Chamber"	selected_analysis "Whenever a nontoken creature enters, if this artifact is untapped, that creature's controller creates a 1/1 colorless Myr artifact creature token."
newly covered	e7f23c73e7c0421bedf737f6b1b87943610d3a8a353beaf59491164300b7797c	card "Nimble Thopterist"	selected_analysis "When this creature enters, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	e8fa61773bebca5052bbd24e91ad3955ef70a01ac8dac9bf334a86442a76f870	card "Desperate Sentry"	selected_analysis "When this creature dies, create a 3/2 colorless Eldrazi Horror creature token.\nDelirium — This creature gets +3/+0 as long as there are four or more card types among cards in your graveyard."
newly covered	e925e33e56a1224bdeb130cd597bc87d4fbe50a30990607cd9a7e6c968aa192a	card "Enlightened Maniac"	selected_analysis "When this creature enters, create a 3/2 colorless Eldrazi Horror creature token."
newly covered	e97427a89d37da2cc6edb90fdc9e29db8225701fa789edb6dc66f375aa65e544	card "Eldrazi Aggressor"	selected_analysis "Devoid\nThis creature has haste as long as you control another colorless creature."
newly covered	eb3e8c6af123587255ca209b6af87dcfa2bbb3a81739ab7cf23e8bb17b730e5c	card "Go-Shintai of Life's Origin"	selected_analysis "{W}{U}{B}{R}{G}, {T}: Return target enchantment card from your graveyard to the battlefield.\nWhenever Go-Shintai of Life's Origin or another nontoken Shrine you control enters, create a 1/1 colorless Shrine enchantment creature token."
newly covered	ed766faf544fe330931ec8256af93bb575cb5299cdd9284cca89bb7aa2a87ff2	card "Spawn-Gang Commander"	selected_analysis "Devoid\nWhen you cast this spell, create three 0/1 colorless Eldrazi Spawn creature tokens with \"Sacrifice this token: Add {C}.\"\n{1}{C}, Sacrifice an Eldrazi: This creature deals 2 damage to any target."
newly covered	edd19a251742df78f3dc49d25ee766253a1a0055eab5eadb71f732ffa0a98043	card "Maverick Thopterist"	selected_analysis "Improvise\nWhen this creature enters, create two 1/1 colorless Thopter artifact creature tokens with flying."
newly covered	ee16815d86f3522571b34654aacff8cbda52c79baa0b4ea546f276b870aedf04	card "Pia Nalaar"	selected_analysis "When Pia Nalaar enters, create a 1/1 colorless Thopter artifact creature token with flying.\n{1}{R}: Target artifact creature gets +1/+0 until end of turn.\n{1}, Sacrifice an artifact: Target creature can't block this turn."
newly covered	ee99d76a9bc334fedfcf9ff8678cc0249c36c6d2876bf3da0de66ac669add931	card "Wing Splicer"	selected_analysis "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\nGolem creatures you control have flying."
newly covered	eeb26478a32ed665c1fd5ebdbab8f7db4fa22a10189738086da17143392026fc	card "Spawning Bed"	selected_analysis "{T}: Add {C}.\n{6}, {T}, Sacrifice this land: Create three 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	efaa52d9548afae05d083974faef9389a729e77b51a92cb71bcd2f7e1c2f2aa0	card "Nimblewright Schematic"	selected_analysis "When this artifact enters or is put into a graveyard from the battlefield, create a 1/1 colorless Construct artifact creature token."
newly covered	f14431d145ce6ef6c8073c52204ef0813e3ce6134fac7daab654865401a6332a	card "Mite Overseer"	selected_analysis "First strike\nDuring your turn, creature tokens you control get +1/+0 and have first strike.\n{3}{W/P}: Create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\""
newly covered	f1dcf73b2a96f58e12275e2246dfdfa12084c6753da6d5a8858145cb687c852d	card "Ancient Kavu"	selected_analysis "{2}: This creature becomes colorless until end of turn."
newly covered	f1fe7220d12c94cf86ba4a203993a562c31776f67ad0c715aaec786c52227e9d	card "Age of Ultron"	selected_analysis "I — For each opponent, destroy up to one target nonartifact creature that player controls.\nII — For each opponent, you create a 2/2 colorless Robot Villain artifact creature token.\nIII — Artifact creatures you control gain deathtouch until end of turn. Put a +1/+1 counter on each of them."
newly covered	f263ed0289f06d7f661667cbef9a380ee7355b87f2e8567db1b78acee5b2b141	card "Parasitic Implant"	selected_analysis "Enchant creature\nAt the beginning of your upkeep, enchanted creature's controller sacrifices it and you create a 1/1 colorless Phyrexian Myr artifact creature token."
newly covered	f470cacf44e3e7f430ad9c0a6d03e11b93b5bd073f9b228218441a06d9325f2e	card "Brood Monitor"	selected_analysis "Devoid\nWhen this creature enters, create three 1/1 colorless Eldrazi Scion creature tokens. They have \"Sacrifice this token: Add {C}.\""
newly covered	f4883bd5d6ee88916186bdb998933cfe444c49fcbdc26d98270ace9ec09ec9cd	card "Thopter Mechanic"	selected_analysis "Whenever you draw your second card each turn, put a +1/+1 counter on this creature.\nWhen this creature dies, create a 1/1 colorless Thopter artifact creature token with flying."
newly covered	f495b66dc65c3390209c508e1256a8ac38b68923ad8a9814fc72f86331726078	card "Void Attendant"	selected_analysis "Devoid\n{1}{G}, Put a card an opponent owns from exile into that player's graveyard: Create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	f4e410dcc58622c3e5107e76be6431a771991a34374ef67cb17e91a178184914	card "Phyrexian Triniform"	selected_analysis "When this creature dies, create three 3/3 colorless Phyrexian Golem artifact creature tokens.\nEncore {12}"
newly covered	f747cca361a8ff88132031e8b369fad5821906b57e5ab89145cc54c754a57c28	card "Incubator Drone"	selected_analysis "Devoid\nWhen this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\""
newly covered	f95e24a1b4294e263831d0a706a871df1efbe9718891a102a1563c0c456b25e8	card "Sensor Splicer"	selected_analysis "When this creature enters, create a 3/3 colorless Phyrexian Golem artifact creature token.\nGolem creatures you control have vigilance."
newly covered	fa73cad9b8d57c897ad8e4b548936223c4faa08c9f6755f7e3b554f2832f0b4e	card "Eusocial Engineering"	selected_analysis "Landfall — Whenever a land you control enters, create a 2/2 colorless Robot artifact creature token.\nWarp {1}{G}"
newly covered	fabf045d2f49144346dd14da533fa355c491e9b2d385536e5ace88db94bf1010	card "Song of the Dryads"	selected_analysis "Enchant permanent\nEnchanted permanent is a colorless Forest land."
newly covered	fbda8c732f104e4dd853f26517dc0cb6ad897020ad1c077dcbfcee9659caebf2	card "Gadget Technician"	selected_analysis "When this creature enters or is turned face up, create a 1/1 colorless Thopter artifact creature token with flying.\nDisguise {U/R}{U/R}"
newly covered	fc1dabb751db25a8a5e8d10dba388f844587c4636911d91d263abe033b39b9e3	card "Emrakul's Messenger"	selected_analysis "Devoid\nFlying\nWhenever you draw your second card each turn, create a 0/1 colorless Eldrazi Spawn creature token with \"Sacrifice this token: Add {C}.\""
newly covered	fc3d51e20cc3b6816e8cf73c2b6914088d6a174dd47685a2e1e34a1b3100bc6b	card "Emrakul's Evangel"	selected_analysis "{T}, Sacrifice this creature and any number of other non-Eldrazi creatures: Create a 3/2 colorless Eldrazi Horror creature token for each creature sacrificed this way."
newly covered	fe3a3d863cc0af26282de3041d2e1a61a4f7fb7518a1c5339855280ca7161155	card "Spawnbed Protector"	selected_analysis "At the beginning of your end step, return up to one target Eldrazi creature card from your graveyard to your hand. Create two 1/1 colorless Eldrazi Scion creature tokens with \"Sacrifice this token: Add {C}.\""
newly covered	fe7661d330da991bcee96eb6cd092d28333d895db8542aec878785eab30c56dc	card "Awakening Zone"	selected_analysis "At the beginning of your upkeep, you may create a 0/1 colorless Eldrazi Spawn creature token. It has \"Sacrifice this token: Add {C}.\""
```
