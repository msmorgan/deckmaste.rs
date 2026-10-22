---
needs: []
---
# One encoding for a bare "another"/"other"; the checker resolves its Contrast Anchor

Standard constraints apply. This is finding 3 of the spelling-layer audit: printed "another" or
"other" is written today as `isOther`, `otherThan this_`, `otherThan thisCreature`,
`otherThan it` or `otherThan (that …)`, at the author's choice. The text marks none of these, so
spelling cannot parse the word back to one tree.

## Ruling (owner, 2026-10-21)

ADR [semantics-v3](../../decisions/semantics-v3.md) §0.1 is amended to match [CR#702.151a]
(reconfigure's "another target creature you control") and the Olivia Voldaren example under
[CR#707.11] ("deals 1 damage to another target creature"). A bare "another"/"other" does not
always presuppose an earlier target.

- `Predicate.other` (`isOther`) is the only encoding of a bare printed "another"/"other".
  "Any other target" is `isAnyOtherTarget` (`and [isAnyTarget, isOther]`).
- `Predicate.otherThan <NP>` is reserved for a printed "other than <NP>".
- The checker resolves `other`'s Contrast Anchor in this order:
  1. the nearest earlier mention of a compatible kind in scope, target or not ("Choose a player.
     Each other player…"). Nearest is DRS recency, so on *Arc Trail* the first target beats the
     self-mention "Arc Trail deals". The thing the word describes, and anything that resolves to
     it, is never a candidate;
  2. failing that, a compatible participant in the same clause or event, read forward. When
     "another <X>" is the agent of a `would`, `damageRule` or event clause, that clause's
     compatible recipient or object is the anchor;
  3. otherwise the source, for an object head, or `you`, for a player head ([CR#109.5]).
- If a card needs an anchor other than the one these steps pick, STOP for an owner ruling.
- Backup edge case ([CR#702.165a], `Definitions/Keywords.lean:277-287`): "If that's another
  creature" is `is (that creature) (and [isCreature, isOther])`. The subject of `is` is never a
  candidate anchor, so the contrast is with "this creature", the source. The code writes
  `is (that creature) (.otherThan thisCreature)` today (`:282`). Without the exclusion, "that
  creature" (the target) is the nearest step-1 candidate and the condition could never hold.
- *Inquisitor's Flail* (`Cards/Damage.lean:1220-1230`) needs step 2. Its second ability reads "If
  another creature would deal combat damage to equipped creature, it deals double that damage to
  equipped creature instead". No creature is mentioned earlier, so the anchor is the clause's
  recipient, "equipped creature", and not the Equipment. A redirection effect ([CR#614.9]: *Harm's
  Way*, *Kor Chant*, *Captain's Maneuver*, *Nomads en-Kor*) can make the equipped creature deal
  combat damage to itself. The first ability already doubles that damage. "Another" keeps the
  second ability from applying to the same event as well ([CR#614.1a,616.1f]), which would double
  it twice.
- Retire `otherThan this*` wherever the card prints "another"/"other". *Arc Trail*
  (`Cards/Damage.lean:35`, `isAnyOtherTarget`) and *Screaming Nemesis* (`Cards/Damage.lean:743`,
  `and [isAnyTarget, otherThan this_]`) both print "any other target", so they end with the same
  encoding.

A delayed trigger reads earlier targets as settled particulars ([CR#603.7c]). They are earlier
mentions in scope, so step 1 can anchor an "other" in the delayed trigger on them (ADR §0.1,
accepted by the owner 2026-10-21).

## Review rulings (owner, 2026-10-21)

These amend the ruling above and ADR §0.1.

- **Q1.** Mentions of the source and of "you" are never step-1 or step-2 candidates. They are
  the step-3 fallback only.
  - A referent a Condition introduces (an intervening "if", a `doIf`/`doOnlyIf` condition,
    `exists_`) is never a candidate. Targets the action branch announces still are
    ([CR#601.2c]). This settles *Flames of the Raze-Boar* (the target) and soulbond (this
    creature).
  - Compatibility includes zone: the head's zone against the candidate's. This settles
    *Hulkling, Young Avenger*, *Ink-Treader Nephilim*, *Mirrorwing Dragon*, *Invasion of
    Tarkir*, and a drawn card.
  - A mention of the source coordinated with the "other" phrase is that phrase's anchor
    (*Slimefoot and Squee*'s "Return this card and up to one other target creature card").
- **Q2.** Steps 1 and 2 check card type. The step-3 source fallback checks only kind and zone, so
  *Ebony Fly*, *Restless Ridgeline*, *Gideon Blackblade*, *Goliath Truck* and the Vehicles
  check. This overturns "must then refuse a source whose card types do not include creature"
  below. *Inquisitor's Flail* stays at step 2.
- **Q3.** Step 2 takes only particulars: targets, definite descriptions, and attachment hosts
  ("equipped creature"). It never takes a generic indefinite. *Embermaw Hellion*'s "a permanent
  or player" falls through to the source.
- **Q4.** Step 2 applies to every clause agent, including handoff, deontic, copy and
  characteristic-change subjects. *Mirrorweave*'s "Each other creature becomes a copy of target
  nonlegendary creature" anchors on the target.
- A head that may be an object or a player ("any other target") falls back to the source, never
  to "you".
- A written granted ability is read with a source of unknown type, and the granting text's
  objects are never its anchor (*Setessan Tactics*, *Hardy Outlander*, *Shameless Charlatan*).
- Compatibility includes controller, at every step (coordinator clarification of "compatible").
  A candidate whose controller restriction contradicts the head's ("you control" against "an
  opponent controls") is never the anchor. *Bartz and Boko*'s "each other Bird you control"
  therefore anchors on the source, not on "target creature an opponent controls".
- The exclusion of the `is` subject covers only the predicate's own "another", not a noun
  phrase nested in it. *Unexpected Allies*' "if it has the same name as another creature you
  control" anchors on "it".

## Checker change

- `Predicate.other` (`Check/Phrase.lean:2145-2147`) stops refusing with `anyTargeted` when no
  target is visible. It resolves the Contrast Anchor by steps 1 to 3. `otherAnchorOkWith`
  (`Check/Phrase.lean:600-614`) stops using `anyTargeted` (`Check/Words.lean:784`) for a bare
  `other`. `otherAnchored` still refuses two "other"s in one conjunction.
- `Condition.matches subj pred` (`Check/Phrase.lean:2590`, `:2642`) excludes `subj`'s referents
  from anchor candidates while stepping `pred`.
- Make kind and card-type compatibility real for every anchor, the source anchor included. Today
  bare `.this` is pushed with an untyped payload, `.object [] …` (`Check/Phrase.lean:2464-2468`).
  `anchorTyOk t []` is true (`Check/Words.lean:787-789`), so `anchorTyFits`
  (`Check/PredicateShape.lean:111-112`) accepts any head against `this_`. Give the source referent
  the type evidence its card frame or Head Noun gives it. A creature-headed "another" must then
  refuse a source whose card types do not include creature. A source of unknown type, such as a
  Definition's "this permanent" (graft, champion), stays compatible. Heads that name no card type
  ("another blue spell" on *Dream Thief*, "another card named Korlash") are unaffected.
- Step 2 needs the clause's participants before the clause is complete. A damage-rule or `would`
  clause steps its agent before its recipient. Collect step-2 candidates from the whole clause or
  event, and exclude the described thing and its anaphors (*Tromell*'s "enters with … on it").
- Rewrite the doc comments on `Predicate.other` and `Predicate.otherThan`
  (`Abilities.lean:252-258`). They currently say `other` contrasts with earlier targets, and give
  "another creature you control" as an `otherThan` example.
- Update `lean/CONTRACTS.md` (the `ComplementAnchor` row and any statement of `other`'s anchor).

## Sites

Line numbers verified 2026-10-21 against the Scryfall Oracle text in `data/scryfall/`.

**Keep `otherThan`** (the card prints "other than"):
- *Disciple of Caelus Nin* `Cards/Choice.lean:1477` ("All permanents other than this creature
  that weren't chosen this way").
- *Terrifying Presence* `Cards/Description.lean:123`.
- *Death by Dragons* `Cards/Keyword.lean:77`.
- myriad `Definitions/Keywords.lean:1019` ("each opponent other than defending player",
  [CR#702.116a]).

**Already `isOther`; keep, pin the anchor:** *Phantom Blade* `Cards/Choice.lean:52`; *Arc Trail*
`Cards/Damage.lean:35`; *Deadshot* `:40`; *Flames of the Raze-Boar* `:78`; *Ulvenwald Tracker*
`:102`; *Carom* `:413`; *Feral Contest* `Cards/Counters.lean:995`;
`Cards/Description.lean:41`.

**Rewrite to `isOther`** (the card prints a bare "another"/"other"):
- Templates: `isOtherPlayer` `Templates.lean:407` (`otherThan you`) and `anotherSpell`
  `Templates.lean:584-585`.
- Definitions:
  - `Definitions/Keywords.lean`: storm `:33` [CR#702.40a], backup `:282` (per the edge case
    above), battle cry `:310` [CR#702.91a], champion `:364` [CR#702.72a], crew `:430`
    [CR#702.122a], graft `:763` [CR#702.58a], reconfigure `:1205` [CR#702.151a], saddle `:1257`
    [CR#702.171a], soulbond `:1306,1312,1315` [CR#702.95a], station `:1376` [CR#702.184a],
    training `:1439` [CR#702.149a].
  - support `Definitions/Actions.lean:408` [CR#701.41a].
- Cards, anchor from an earlier non-source mention (step 1):
  - *Bile Blight* `Cards/Anaphora.lean:534` and *Echoing Ruin* `:545` (`otherThan (that …)`).
  - *Nibelheim Aflame* `Cards/Damage.lean:96` (`otherThan it`).
  - *Contractual Safeguard* `Cards/Counters.lean:985` (`otherThan it`, anchored on "a creature
    you control").
  - *Autumn-Tail, Kitsune Sage* `Cards/Faces.lean:272` (`otherThan (that creature)`, anchored on
    "a creature").
- Card, anchor from a clause participant (step 2): *Inquisitor's Flail* `Cards/Damage.lean:1227`
  (`otherThan (equipped creature)` becomes `isOther`).
- Cards, anchor is the source or `you`:
  - `Cards/Cost.lean`: *Tranquil Grove* `:329`, *Boom Scholar* `:490`, *Hulk, Gamma Goliath*
    `:497`, *Patrician Geist* `:685`, *Ahn-Crop Invader* `:994`, *Deadly Dancer* `:1021`, *Unesh*
    `:1036`, *Words of Wisdom* `:1062`.
  - `Cards/Choice.lean`: *Adaptive Automaton* `:377`, *Zimone and Dina* `:946`.
  - `Cards/Faces.lean`: *Missy* `:55`, *Yedora* `:71`, *Chittering Host* `:383`.
  - `Cards/Description.lean`: *War Screecher* `:72`, *Approach of the Second Sun* `:264`.
  - `Cards/Damage.lean`: *Brash Taunter* `:99`, *Screaming Nemesis* `:743`, *Syr Konrad*
    `:1182`.
  - `Cards/Turn.lean`: *Fettergeist* `:326`.
  - `Cards/Mana.lean`: *Sage of Fables* `:456`, *Sheltered Valley* `:929`.
  - `Cards/Trigger.lean`: *Bard, Heir of Girion* `:346`, *Unstable Shapeshifter* `:373`, *Altar
    of the Brood* `:794`.
  - `Cards/Keyword.lean`: *Bria* `:177`, *Pontiff of Blight* `:183`, *Consul's Lieutenant*
    `:248`, *Drumbellower* `:691`, *Tayam* `:696`, *Dream Thief* `:812`, *Deepway Navigator*
    `:835`, *Soul of Emancipation* `:1093`, Korlash's grandeur cost `:1631`, *Deadpool*
    `:1695,1698`.
  - `Cards/Counters.lean`: *Bellowing Aegisaur* `:74`, *Carnifex Demon* `:77`, *Meren* `:126`,
    *Stag Beetle* `:165`, *Towering Titan* `:172`, *Aragorn* `:381`, *Captain Marvel*
    `:385,1053`, *Tromell* `:437`, *Grumgully* `:653`, *Metallic Mimic* `:672,682`, *Bramblewood
    Paragon* `:709`, *Oona's Blackguard* `:724`, *Thranduil's Company* `:1005`, *Bloodline
    Pretender* `:1110`, *Mistbreath Elder* `:1165`, *Syphon Mind* `:1215`.
  - `Cards/Static.lean`: *Seedborn Muse* `:481`, *Unwinding Clock* `:490`, *Thousand Moons
    Infantry* `:497`, *Kasmina* `:799`, *Nicol Bolas, Dragon-God* `:804`.

Battle cry `:310` and *Deadly Dancer* `:1021` are also rewritten by
[lean-v3-gets-printed-zero](lean-v3-gets-printed-zero.md). Whichever ticket lands second rebases
onto the other's encoding.

When this lands, every remaining `otherThan` site must print "other than". Re-run the inventory
(`rg 'otherThan|isOther|\.other\b' lean/Semantics`) against Oracle text before closing.

## Tests

Rewrite these pins, which assert what the ruling overturns:
- `badOther` (`Proofs/Damage.lean:55-58`): "This deals 1 damage to any other target" now checks
  with the source as anchor. Rename it and assert that.
- `badDelayedOther` (`Proofs/Turn.lean:35-41`): the delayed "any other target" now checks,
  anchored on the earlier, settled target.
- `badBareOtherWithoutAnchor` (`Proofs/Counters.lean:554-559`): the bare "other" now anchors on
  the source.

Keep `badDoubleComplement`, `badOtherAndComplement`, `badComplementCrossHead` and the
`complementAnchor` pins (`Proofs/Damage.lean:235-269`, `Proofs/Zone.lean:188`). They pin
`otherThan` and double-"other" behaviour, which the ruling leaves in place.

Add these pins:
- *Arc Trail* and *Screaming Nemesis* write the same predicate for "any other target". Arc Trail
  anchors on its first target, Screaming Nemesis on the source.
- Olivia Voldaren's "deals 1 damage to another target creature" and reconfigure's "another target
  creature you control" check with no earlier target, anchored on the source.
- "Choose a player. Each other player …" anchors on the chosen player, not on `you`.
- Backup's `is (that creature) (and [isCreature, isOther])` anchors on the source, never on
  "that creature".
- *Contractual Safeguard* anchors on "a creature you control", which is not a target.
- *Inquisitor's Flail*'s second ability anchors on the equipped creature (step 2), not on the
  Equipment.
- A creature-headed "another" with no earlier mention and no clause participant anchors on the
  source whatever its card types (Q2), for example a probe Equipment printing "another creature
  dies".
