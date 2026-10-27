---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# Restore the printed head nouns that card spellings drop

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`; line numbers are as
of that change.

A Head Noun narrows what a phrase is known to be (`docs/contexts/game-model/CONTEXT.md`, **Head
Noun**, [CR#109.1,205.2a]). Where Oracle prints "permanent", "card" or "creature", the spelling
writes the matching predicate. The rulings already in force: `lean-v3-authoring-surface` §3
(`docs/tickets/done/lean-v3-authoring-surface.md:552`) gives the canonical spellings
`and [isCard, in_ yourGraveyard]`, `and [isCreature, isCard, in_ yourHand]` and
`and [isCard, in_ (exiledWith thisArtifact)]`, and `lean-v3-typed-verbed-deeds` rewrote sites "to
match their Oracle text … actual head nouns". The Templates are `isPermanent` (`Templates.lean:390`,
`.in_ battlefield`), `isCard` (`:393`) and `cardIn` (`:616`, `and [isCard, in_ zone]`).
`youControl` and `controlledBy` (`:469`, `:467`) are possessor tests only and do not imply the
battlefield, so dropping "permanent" widens the set to spells you control.

The sites using `exiledWithThisArtifact` (Muse Vessel, Synod Sanctum, Chrome Mox, Ice Cauldron,
Idol of Endurance, Cold Storage) are **not** in this ticket; `lean-v3-template-twins` §6 owns them.

## 1. Sites with no head noun at all

Rewrite each exactly as given. Every rewrite was compiled in a scratch file on 2026-10-25 and
checked `[]`.

| # | Site | Card, Oracle | Now | Write |
|---|---|---|---|---|
| 1 | `Cards/Static.lean:473` | Seedborn Muse: "Untap all permanents you control during each other player's untap step." | `allOf youControl` | `allOf (and [isPermanent, youControl])` |
| 2 | `Cards/Static.lean:496` | Blatant Thievery: "For each opponent, gain control of target permanent that player controls." | `target (controlledBy (that player))` | `target (and [isPermanent, controlledBy (that player)])` |
| 3 | `Cards/Counters.lean:44-47` (`alaundoTheSeer`) | Alaundo the Seer: "…Then remove a time counter from each other card you own in exile." | `each (and [youOwn, in_ …])` | `each (and [isCard, isOther, youOwn, in_ exileZone])`. This also restores the printed "other". In the fragment the Contrast Anchor falls back to the source (`docs/decisions/semantics-v3.md` §0.1 item 2, step 4); in the whole card it is the card the ability exiled first (step 2). |
| 4 | `Cards/Static.lean:779` | Myr Welder: "This creature has all activated abilities of all cards exiled with it." | `allOf (in_ (exiledWith this_))` | `allOf (cardIn (exiledWith this_))` |
| 5 | `Cards/Choice.lean:1290` | Rogue Class level 3: "You may play cards exiled with this Class, …" | `play (allOf (in_ (exiledWith thisClass)))` | `play (allOf (cardIn (exiledWith thisClass)))`. Leave `toCast (in_ (exiledWith thisClass))` at `:1292` as it is (see "Not in this ticket"). |
| 6 | `Cards/Cost.lean:680` | Void Maw: "Put a card exiled with this creature into its owner's graveyard: …" | `put (a (in_ (exiledWith thisCreature))) graveyard` | `putInto (a (cardIn (exiledWith thisCreature))) graveyard`. The verb change is `lean-v3-template-twins` §1's; make both changes here, and that ticket skips this line. |
| 7 | `Cards/Keyword.lean:262` | Drach'Nyen: "…where X is the exiled card's power." | `powerOf (the (in_ (exiledWith thisEquipment)))` | `powerOf (the (cardIn (exiledWith thisEquipment)))` |
| 8 | `Cards/Anaphora.lean:300` | Thalia's Lancers: "…search your library for a legendary card, reveal it…" | `a (and [hasSupertype .legendary])` | `a (and [isCard, hasSupertype .legendary])` |
| 9 | `Cards/Trigger.lean:385` | Aven Shrine: "…X is the number of cards in all graveyards with the same name as that spell." | `countOf (and [in_ graveyard, (shares .cardName .share (that .spell))])` | `countOf (and [isCard, in_ graveyard, shares .cardName .share (that .spell)])` |
| 10 | `Cards/Anaphora.lean:266` | Eradicate: "…for all cards with the same name as that creature…" | `allOf (shares .cardName .share (that card))` | `allOf (and [isCard, shares .cardName .share (that card)])` |
| 11 | `Cards/Choice.lean:1112` | Memoricide: "…any number of cards with that name…" | `counted anyNumber (named chosenName)` | `counted anyNumber (and [isCard, named chosenName])` |
| 12 | `Cards/Choice.lean:1317` | Vraska's Scorn: "…a card named Vraska, Scheming Gorgon…" | `a (named "Vraska, Scheming Gorgon")` | `a (and [isCard, named "Vraska, Scheming Gorgon"])` |
| 13 | `Cards/Keyword.lean:1133` | Tower Winder: "…a card named Command Tower…" | `a (named "Command Tower")` | `a (and [isCard, named "Command Tower"])` |
| 14 | `Cards/Trigger.lean:659` and `:667` | Agency Outfitter: "…a card named Magnifying Glass and/or a card named Thinking Cap…" | `a (named "Magnifying Glass")`, `a (named "Thinking Cap")` | `a (and [isCard, named "Magnifying Glass"])`, `a (and [isCard, named "Thinking Cap"])`, at both lines |
| 15 | `Cards/Keyword.lean:1657` | Korlash, Heir to Blackblade (grandeur): "Discard another card named Korlash, Heir to Blackblade: …" | `and [named "Korlash, Heir to Blackblade", isOther, in_ yourHand]` | `and [isCard, named "Korlash, Heir to Blackblade", isOther, in_ yourHand]` |
| 16 | `Cards/Cost.lean:587` | Force of Will: "…exile a blue card from your hand" | `and [hasColor .blue, in_ yourHand]` | `and [isCard, hasColor .blue, in_ yourHand]` |
| 17 | `Cards/Mana.lean:453`, `:468`, `:479`, `:489`, `:500` | The five Shoals print "…exile a <color> card with mana value X from your hand" | `and [hasColor .<c>, hasManaValue .eq (letter .x), …]` | insert `isCard` as the first conjunct at each of the five lines |
| 18 | `Cards/Static.lean:560` | Sunscour: "…exiling two white cards from your hand" | `and [hasColor .white, in_ yourHand]` | `and [isCard, hasColor .white, in_ yourHand]` |
| 19 | `Cards/Keyword.lean:1038` | Fumiko the Lowblood: "…X is the number of attacking creatures." | `countOf isAttacking` | `countOf (and [isCreature, isAttacking])` |
| 20 | `Cards/Choice.lean:471` | Rune Snag: "…each card named Rune Snag in each graveyard" | `and [named "Rune Snag", in_ graveyard]` | `and [isCard, named "Rune Snag", in_ graveyard]` |

Row 17's lines are, in order, Shining Shoal (`shiningShoal`), Disrupting Shoal, Blazing Shoal,
Sickening Shoal and Nourishing Shoal (`Cards/Mana.lean:447-500`).

## 2. Sites that write the type but not "card"

Each def below prints "<type> card(s)" (Oracle checked, 2026-10-25) and writes the type
predicate with a zone but without `isCard`. In every Predicate conjunction in the def that
describes a printed "… card", insert `isCard` immediately after the last card-type, subtype or
color predicate, giving the canonical `and [isCreature, isCard, in_ yourHand]` order. Where the
conjunction is exactly `and [<zone test>]` plus types, the result is the same; do not otherwise
reorder or rewrite. Where a def has two such conjunctions (the lines are listed), change both.

- `Cards/Anaphora.lean`: `bondOfRevival` :52, `riseFromTheGrave` :233, `everAfter` :241,
  `sylvanScrying` :165, `mysticalTutor` :288, `celebrateTheHarvest` :408, `scapeshift` :513
- `Cards/Choice.lean`: `continueSpell` :221, `zimoneQuandrixProdigy` :346, `preeminentCaptain`
  :352, `phyrexianIngesterPump` :410, `kohChooser` :521, `ashesOfTheFallen` :892,
  `cultivatorColossus` :1085, `zimoneAndDina` :1093, `cryptLurker` :1104, `lostHoursPlacement`
  :1120, `boreasChargerSpell` :1193 ("Plains cards"), `nautiloidShipTrigger` :1276,
  `summonEsperValigarmandaCast` :1281 and :1285, `oldGrowthDryads` :1329
- `Cards/Copy.lean`: `flawlessForgeryLine` :57
- `Cards/Cost.lean`: `sistersOfStoneDeathRecall` :69, `sphinxOfTheChimes` :428,
  `deathMaskDuplicant` :784, :787 and :788
- `Cards/Counters.lean`: `jhoiraOfTheGhitu` :40, `voraciousBrood` :262, `urborgScavengers` :809
  and :810, `charnelTroll` :1143 and :1149
- `Cards/Damage.lean`: `bullseyeModalCost` :995, `sandStranglerDamage` :1030, `syrKonradTheGrim`
  :1298, `theLastRoninII` :1449
- `Cards/Description.lean`: `roguesGallery` :338
- `Cards/Faces.lean`: `midnightScavengers` :451, `garrukRelentless` :620 and :627
- `Cards/Keyword.lean`: `bloodfireEnforcers` :779 and :780, `stormOfSouls` :785,
  `sedrisTheTraitorKing` :835, `grixis` :841, `dralnuLichLord` :847, `cairnWanderer` :1257 and
  :1258, `arcanumWingsAuraSwap` :1642
- `Cards/Mana.lean`: `nahiriLoyaltyRead` :623, `birthingPodSearch` :701,
  `hibernationsEndTrigger` :713, `deliveryMoogle` :821, `moxDiamond` :845
- `Cards/Piles.lean`: `custodiSquire` :121, `deathOrGlory` :158
- `Cards/Static.lean`: `hymnOfRebirth` :38, `abolish` :543, `lhurgoyfDefinition` :649
- `Cards/Trigger.lean`: `carrionGrub` :162
- `Cards/Turn.lean`: `throughTheBreach` :25, `bleedingEffect` :297 and :299, `landTax` :400,
  `thawingGlaciers` :462

`cardIn (exiledWith …)`-style conjunctions that already carry `isCard` (for example
`Cards/Turn.lean:604`, `Cards/Trigger.lean:632`) are already right; leave them.

`nahiriLoyaltyRead` (`Cards/Mana.lean:622-624`, Nahiri, the Unforgiving's 0: "Exile target
creature or Equipment card with mana value less than Nahiri's loyalty from your graveyard") also
drops "or Equipment". Write `and [or [isCreature, hasSubtype equipment], isCard, in_ yourGraveyard,
hasManaValue .less (statOf (.stat .loyalty) this_)]`, and correct its docstring's quotation to the
printed "creature or Equipment card".

## 3. A fixture that is no card

`Cards/Damage.lean:646-651`, `firesongJoinEcho` and its pin `okFiresongJoinEcho` (`:651`), adds
"deals 1 damage to that creature or player" to Firesong and Sunspeaker, whose Oracle text is
"Whenever a white instant or sorcery spell causes you to gain life, Firesong and Sunspeaker deals
3 damage to target creature or player." Every fixture must be a Vintage-legal card's text (owner,
2026-10-25). Retire both, because the fixture asserts text no card prints. The real card stays
pinned by `okFiresongAndSunspeaker` (`Cards/Keyword.lean:166`). If `thatJoin` then has no
"that creature or player" witness, do not add one.

## What must not change

- No Template, Definition, Core constructor or checker code changes. This ticket edits `Cards/`
  only.
- Do not change a site's determiner, quantity, zone or any other conjunct beyond what is listed.
- No pin may be weakened. Every `spelled` card and `ok…` pin named or implied above must stay
  green. If a head-noun insertion makes a pin fail (for example a later "that card" or "it" now
  resolves differently), STOP and report the site, the pin and the refusal; do not drop the
  head noun to make it pass.

## Pins

- Rewritten in place by the edits (their trees change, their claims stay): `okAlaundoTheSeer`
  (`Cards/Counters.lean:48`), `okMyrWelderBorrowedAbilities` (`Cards/Static.lean:780`),
  `okRogueClassLevelThree` (`Cards/Choice.lean:1294`), `okVoidMawPutCost` (`Cards/Cost.lean:682`),
  `okThaliasLancersSearch` (`Cards/Anaphora.lean:302`), `okEradicateSearch` (`:269`),
  `okMemoricideSearch` (`Cards/Choice.lean:1115`), `memoricideExilesWhatItFinds`
  (`Proofs/CardFidelity.lean:234`), `okAgencyOutfitterSearch` (`Cards/Trigger.lean:662`),
  `okGrandeurDiscardCost` (`Cards/Keyword.lean:1658`), `okFumikoBushidoX` (`:1039`), and each
  §2 def's co-located `ok<Name>` pin. Also keep green the pins that read §2 defs:
  `graveyardOriginsAreDepartureObserved` (`Proofs/CardFidelity.lean:158`),
  `spelledGraveyardOriginsAreDepartureObserved` (`:172`), the `flawlessForgeryLine` pin
  (`Proofs/CardFidelity.lean:37`), `Proofs/Keyword.lean:458` and `:478`, `Proofs/Trigger.lean:352`,
  `Proofs/Counters.lean:781`, `costCoercionsKeepTheCard` (`Proofs/Authoring.lean:350-356`) and
  `Proofs/Mana.lean:385`. Where a pin states a term equality against a copied tree, update the
  copy in lockstep.
- Add one pin: `seedbornMuseUntapsOnlyPermanents` in `Proofs/CardFidelity.lean`, asserting that
  Seedborn Muse's untap patient carries `isPermanent` (a `decide` or `rfl` over the spelled
  card's static).
- Retired: `okFiresongJoinEcho` (§3), because it asserts a non-card.

## Acceptance

- `./lean/scripts/build` passes with `--wfail`, no `sorry`.
- `cargo xtask lean check`.
- `grep -n 'allOf youControl' lean/Semantics/Cards` is empty; the 20 §1 rows and every §2 line
  carry the head noun.
- CR citation gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check`
  0 stale.

## Not in this ticket

- Rogue Class's and Summon: Esper Valigarmanda's `toCast (in_ (exiledWith …))`
  (`Cards/Choice.lean:1292`, `:1280-1285`) re-describe "those spells" without anaphora; that is an
  open question for the owner, not a head-noun fix.
- Ice Cauldron (`Cards/Mana.lean:1026-1031`) adds 1 mana for "this artifact's last noted type and
  amount of mana" and drops "last" from "the last card exiled with this artifact". It needs a
  spelling decision; it is reported to the owner.
