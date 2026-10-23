---
needs: [lean-v3-this-way-and-stop-trigger-folds]
---
# Damage rules by composition: `preventDamage`, `asEvent`, `replace` and `cant`

Standard constraints apply. Provenance: row B2 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). This
ticket supersedes `semantics-v3-damage-results` (the 2026-10-13 `DamageResult` / `ResultMode`
row design, Q86-Q88), which is deleted in the same landing as this ticket's minting; its open
items are carried below. The `needs` entry exists for the final step 6 (*Shadowbane*) alone; steps 1-5
do not read that ticket.

## Ruling (owner, 2026-10-22)

- Retire `StaticSpec.damageResult`, `damageRule` and `preventionBan`, and the damage-only types
  `DamageResult`, `ResultMode`, `DamageOp`, `DamageScale`, `DamageAgent`, `DamageScope`,
  `Unpreventable` and `PreventionBan` (verify the list).
- Use one new `Action.preventDamage (cut) (damage : Option WouldClause := none) (also : Option
  Action := none)`, plus the existing `asEvent`, `replace` and `cant`.
- Additive damage results (lifelink, toxic, planeswalker and battle loss) go via `asEvent`.
  Wither and infect go via `replace` over `markDamage` / `loseLife` with `causedBy`, so
  [CR#614.6] gives [CR#120.3d] for free.
- "Can't be prevented" is `cant any (preventDamage .all …)`. Prevention [CR#615.1], redirection
  [CR#614.9] and scaling [CR#614.1a] go via `replace`.
- The rule rows in `Proofs/Rules.lean` that use `damageResult` move too.
- Split the work into sub-steps.

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

**The retire list exists and is damage-only.** `StaticSpec.damageResult` (`Abilities.lean:938`),
`damageRule` (`:939-940`), `preventionBan` (`:941-942`); `DamageResult` (`:873-878`),
`ResultMode` (`Events.lean:36-40`, registered at `:59`), `DamageOp` (`Abilities.lean:1026-1029`),
`DamageScale` (`:683-687`), `DamageAgent` (`:668-670`), `DamageScope` (`:664-666`),
`Unpreventable` (`:672-674`), `PreventionBan` (`:54-56`). None is used outside the three static
arms, their checker, Templates and number slots, the pins, and `RulesTables.damageResult`.

**The list misses**: `ScaleFactor` (`Words.lean:397-399`, used only by `DamageScale.multiplied`);
`RulesTables.damageResult : List DamageResult` (`Rules.lean:141`); the `damageRule` and
`preventionBan` Templates (`Templates.lean:1966`, `:2030`) and the helpers `dealtBy`,
`unattributed`, `toRecipient`, `everywhere`, `prevent`, `redirect`, `scale`, `multiplied`,
`halved`, `shifted`, `damageDealt`, `thatDamage` (`:2340-2377`); and the doc comment naming
`ResultMode.instead` (`Words.lean:109`). The Action Templates `preventAll` (`:1432`),
`preventNext` (`:1436`) and `preventAllBy` (`:1441`) expand to `damageRule` and are re-bodied.

**Shared, kept**: `PreventCut = all | some N | shield N | allBut N | half r`
(`Abilities.lean:676-681`; Templates `allDamage`, `ofThatDamage`, `theNext`, `PreventCut.allBut`,
`PreventCut.half`, `Templates.lean:2358-2366`) is the `cut`. `RoundMode`, `ReplUse`, `DamageKind`,
`OutcomeSort.damageMarked` and `.damagePrevented` stay.

**The primitives**: `StaticSpec.asEvent (event) (does)` (`Abilities.lean:964`, Template
`Templates.lean:1945`); `StaticSpec.replace (event : WouldClause) (replacement) (timing) (use :
ReplUse) (limit)` (`Abilities.lean:934`; not the self-replacement `Action.replace`, `:1280`);
`StaticSpec.cant (subj) (action) (mods)` (`:890`, Template `Templates.lean:1045`).
`Action.markDamage (subject) (amount)` exists (`Abilities.lean:1112`) and nothing but checker arms
uses it (`Check/Events.lean:32,73`, `Check/NumberSlots.lean:155`, `Check/Phrase.lean:3035`).
`loseLife` is a Template over `changeLife (.down …)` (`Templates.lean:1213`). There is no
`Action.prevent…`.

**Corrections to the ruling's premises:**

- Lifelink, wither, infect and toxic are not encoded with `damageResult` or anything else: each is
  a bodiless keyword (`Definitions/Keywords.lean:986,1673,938,1541-1542`). Their half of the
  ruling is new authoring, not migration.
- The only `damageResult` rows are the planeswalker and battle rows in `Proofs/Rules.lean`
  (`planeswalkerLoyaltyDamage :119-121`, `battleDefenseDamage :127-129`, `mode := .intrinsic`,
  doing `loseCounters` of loyalty/defense counters). `Rules.lean` holds none. No card uses
  `damageResult`.
- There are no [CR#120.3a] (life loss) or [CR#120.3e] (marking) rows for wither and infect to
  replace; they must be written first (step 2).

## Sites

**`damageRule` (directly or through `preventAll`/`preventNext`/`preventAllBy`)**

- Prevention, `Cards/Damage.lean`: fog `:135`, indestructibleAura `:139`, shieldmatesBlessing
  `:142`, glacialChasmShield `:153`, *Ethereal Haze* `:297`, *Defang* `:304`, *Sphere of Purity*
  `:310`, *Dazzling Reflection* `:318`, *Thunderstaff* `:326`, *Gideon, Ally of Zendikar* `:346`,
  *Candles' Glow* `:431` (prevention with `also`), inkshieldRider `:437`, *Urza's Armor* `:445`,
  *Circle of Protection: Red* `:453`, *Healing Grace* `:462`, *Reverse Damage* `:470`, *Deflecting
  Palm* `:477`, *Temple Altisaur* `:487`, *Dark Sphere* `:496`, *Shadowbane* `:502`, *Sphere of
  Law* `:512`, *Forcefield* `:544`, *Endure* `:551`, *Divine Deflection* `:564`, *Plated Pegasus*
  `:635`. Elsewhere: *Stormwild Capridor*, *Test of Faith*, *Temper*, *Chromatic Armor*
  (`Cards/Counters.lean:520,529,538,567`); *Gideon Jura*, *Gideon's Intervention*, *Glacial
  Chasm* (`Cards/Deontic.lean:271,325,468`); *Angus Mackenzie* (`Cards/Cost.lean:279`).
- Redirection: *Turn the Tables* `:357`, *Pariah* `:367`, *Martyrs of Korlis* `:375`, *Ward of
  Piety* `:387`, *Mirrorwood Treefolk* `:395`, *Carom* `:403`, *Daughter of Autumn* `:414`, *Aegis
  of Honor* `:423`, *Harm's Way* `:557`, glarecasterShield `:571` (`Cards/Damage.lean`); *Shining
  Shoal* (`Cards/Mana.lean:447`).
- Scaling: *Furnace of Rath* `:579`, *Gratuitous Violence* `:587`, *Fiery Emancipation* `:595`,
  *Sulfuric Vapors* `:602`, *Lashknife Barrier* `:610`, *Ghosts of the Innocent* `:617`, *Fire
  Servant* `:625`, *Inquisitor's Flail* `:1166,1168`, *Blind Fury* `:1232` (`Cards/Damage.lean`);
  *Tok-Tok, Volcano Born* (`Cards/Faces.lean:216`).
- Definitions: absorb (`Definitions/Keywords.lean:187-191`) and protection (`:1227-1230`). Changing
  protection re-checks every `spelled` card with protection.
- `Check.lean:269-281` `sliceECopyAndPreventPin` (raw constructors).

**`preventionBan`** (`Cards/Deontic.lean`): *Excruciator* `:241`, *Flaring Pain* `:248`, *Pinpoint
Avalanche* `:280`, whippoorwillImmunity `:285` (`.noRedirectEither`), *Call In a Professional*
`:294`. (*Skullcrack* and *Leyline of Punishment*, named in drafting, are not on the bench.)

**Checker**: `stepDamageAgentWith` `Check/Phrase.lean:1654`, `stepDamageScopeWith` `:1661`,
`stepDamageAgentAheadWith` `:1670` (carries the Contrast Anchor's forward read),
`stepPreventCutWith` `:1676`, `stepDamageOpWith` `:1681-1700`; the static arms `:3729-3751`;
`Check/NumberSlots.lean:43,50,56,141-143`; `Check/CardFacts.lean:109,112`; `Check/Rules.lean:68`.
Refusals these arms emit, all used elsewhere too: `.singular` (a redirect target, `:1694`),
`.kindLte` (`:1695`, `:3733`), `.shiftSets` (`:1700`), `.damageDealtInScope` (the ban's "that
damage", `:3751`), `.outcomeInScope .damagePrevented`, `.quantOutcomeInScope`, `.knownCounter`.

## Rulings (owner, 2026-10-22, second batch)

The four open items and the noted question are ruled. Nothing in this ticket waits on the owner.

**4a. (Revised by the third batch: the shield is `redirectDamage`'s `cut`; no `ReplUse` change.) Partial redirection: an amount shield on the replacement's use.** Give `ReplUse` an amount
shield, matching `PreventCut.shield`, so "the next N damage … is dealt to X instead" composes over
`StaticSpec.replace` (*Ward of Piety*, *Carom*, *Daughter of Autumn*, *Harm's Way*, *Shining
Shoal*) [CR#614.9,615.7]. VERIFY: the name is true, `ReplUse = repeatedly | nextTimeOnly`, but it
is in `Events.lean:23-25`, not `Abilities.lean`. Layering consequence: `Events.lean` imports only
`Words`, and `Amount` is `Abilities.lean:392`, so a constructor `shield (amount : Amount)` cannot
sit there. Move `ReplUse` into `Abilities.lean`'s inductive block beside `StaticSpec.replace`
(`:934-938`), its only user, and step the shield amount where `PreventCut.shield` is stepped
(`stepPreventCutWith`). `PreventCut.shield` itself stays for `preventDamage`: prevention shields
size through the cut, redirection shields through the use.

**4b. (Revised by the third batch: `Action.redirectDamage` exists; *Whippoorwill* is `cant any (redirectDamage …)`.) No `Action.redirectDamage`.**

- A redirection is `replace (would … dealDamage …) (instead (dealDamage thatMuch newRecipient))`.
- *Whippoorwill*'s "can't be … dealt instead to another permanent or player" is a `cant` over the
  existing `Action.replace (replaced : Action) (replacement : Replacement)` (VERIFY: true,
  `Abilities.lean:1280`; the old text's `:1263` was stale):
  ```lean
  cant any (Action.replace (dealDamage anyAmount (that creature))
    (instead (dealDamage thatMuch (a (and [or [isPermanent, isPlayer], isOther])))))
  ```
  `Action.deonticHeadOk` (`Check/Events.lean:46-61`) must admit `Action.replace` as a `cant` head.
- The "can't be prevented" half is `cant any (preventDamage .all …)`, as already ruled.

**4c. Toxic is option B:** one result reading the object's total toxic value, the sum of every
toxic ability's N [CR#702.164b..702.164c]. Not one row per `toxic N` instance. If no `Amount`
reads a total toxic value, add one (name per the glossary). The implementer places the single row
(a rule conferred as a property, as in step 2, or in the keyword's definition) and states why.

**4d. (Revised by the third batch: both checks are enforced on `redirectDamage`'s `to`.) Redirect checks: which is true.** The ruling expected `dealDamage`'s own checks to give the
redirect target's singular and object-or-player checks for free. VERIFY: half true.
`dealDamage`'s arm (`Check/Phrase.lean:3033-3043`) refuses `.perMember` and `.kindLte`
(`:3042-3043`), so the object-or-player kind check IS inherited. But the `.singular` check is not:
`NounPhrase.perMemberOk` (`Check/Phrase.lean:358-359`) is `det == each || det == all || plur ==
.one`, so a plural recipient with `each`/`all` is legal for `dealDamage` (as the ticket's earlier
text said). Under the new spelling `badRedirectToPlural` (`Proofs/Damage.lean:415`, `allOf (and
[isCreature, youControl])`, det `all`) would NOT be refused, and `badRedirectToGroup` (`:533`, a
`youAnd` coordination) would be refused as `.perMember`, not `.singular`. So the singular check is
kept explicitly, as a law on the redirection's instead-action: an `instead (dealDamage thatMuch r)`
that replaces a damage would-clause, whether in `StaticSpec.replace` or in `Action.replace`, needs
`r.plur == .one` (`.singular`) [CR#614.9]. The implementer confirms the two verdicts by evaluating
the pins before and after (this was read from the code, not run).

**4e. *Shadowbane*'s `Condition.preventedFromSource` folds into `did … (preventDamage …)
thisWay`** once B2 (this ticket) and B4 (`lean-v3-this-way-and-stop-trigger-folds`) land. It is the
final step 6 below and why this ticket `needs` that one. VERIFY: `preventedFromSource` is
`Abilities.lean:477` (Template `Templates.lean:2123-2124`, checker `Check/Phrase.lean:2857`,
*Shadowbane* `Cards/Damage.lean:502-505`); the other ticket's `:474` is stale.

## Rulings (owner, 2026-10-22, third batch): `Action.redirectDamage`

This batch revises 4a, 4b and 4d below and supersedes their text wherever they disagree. Nothing
else in the second batch moves.

- Add `Action.redirectDamage (cut : PreventCut) (to : NounPhrase)`, mirroring `Action.preventDamage`.
  The ticket's `preventDamage` is `(cut) (damage : Option WouldClause := none) (also : Option Action
  := none)` and is not yet in the code (`grep` finds no `preventDamage` in `Abilities.lean`; it is
  added by step 1). Mirroring its damage-identifying slot too, `damage : Option WouldClause :=
  none`, is the implementer's call: it lets a ban or a "that damage" reading name the redirected
  damage the same way `cant any (preventDamage .all (damage := some clause))` does, and it keeps the
  two Actions checkable by one shape (same stepping of `cut`, same `damageDealtInScope` refusal).
  The implementer states the decision and, if `damage` is dropped, why.
- One Action covers three items:
  - **4a, partial redirection** ("the next 2 damage", *Harm's Way*, *Ward of Piety*, *Carom*,
    *Daughter of Autumn*, *Shining Shoal*) goes through the `cut` shield [CR#614.9,615.7]. The
    planned `ReplUse` shield amount is dropped: `ReplUse` stays in `Events.lean` and does not move.
    `PreventCut.shield` is stepped by `stepPreventCutWith`, as for prevention.
  - **4b, *Whippoorwill*'s** "can't be ... dealt instead to another permanent or player" becomes
    `cant any (redirectDamage …)`. It no longer uses `cant any (Action.replace …)`, and
    `Action.deonticHeadOk` admits `redirectDamage` (with `preventDamage`), not `Action.replace`.
  - **4d, redirect checks**: the redirect target's singular check (`.singular`) and its
    object-or-player kind check (`.kindLte`) are enforced on `redirectDamage`'s `to`.
- Redirection is no longer written `instead (dealDamage thatMuch …)`: a redirection is `replace
  (would <src> (dealDamage …)) (instead (redirectDamage cut to))`. The "explicit singular law on the
  instead-action" of 4d is replaced by the checks on `to`.
- Owner's reasons:
  - Passive redirection ("All damage that would be dealt to you is dealt to X instead", *Pariah*)
    has no source noun phrase to hand to `dealDamage`.
  - `Action.replace` is for [CR#614.15] self-replacement and is not a deontic head.
  - Partial redirection redirects up to the shield, not `thatMuch`.
- Citations: redirection effects [CR#614.9]; amount shields [CR#615.7].

## Reading of the ruling (confirm in step 1 before the bulk rewrite)

- Prevention is `replace (would <source> (dealDamage …)) (instead (preventDamage cut (also :=
  …)))`, with the use (`.repeatedly`, `.nextTimeOnly`) and limit on `replace` and a one-shot's
  duration on the enclosing `establish`. `damage := none` there.
- "Can't be prevented" names the damage: `cant any (preventDamage .all (damage := some clause))`.
  *Pinpoint Avalanche*'s "The damage can't be prevented" reads the damage just dealt; keep its
  `.damageDealtInScope` refusal on whichever form says "that damage". Prevention effects still
  apply to unpreventable damage and their additional effects still happen [CR#615.12]; the engine
  mirror owns that.
- `preventDamage` steps its cut before it publishes its `damagePrevented` outcome, then its `also`
  [CR#615.5] (`badShieldSizedByItsOwnPrevention` depends on the order).

## Sub-steps

1. **`preventDamage`, prevention and the bans.** Add the constructor and Template; admit it in
   `Action.deonticHeadOk` (`Check/Events.lean:46-61`) and, if patterns read it, `eventHeadOk`
   (`:25-41`); number slots and `namesThisDoor`. Re-body `preventAll`/`preventNext`/`preventAllBy`,
   absorb and protection. Rewrite the prevention and ban cards, `sliceECopyAndPreventPin`, and
   their pins. *Whippoorwill* uses `cant any (redirectDamage …)` and redirection uses `redirectDamage`'s `cut`
   and `to` checks (third batch); both are ruled. Add `Action.redirectDamage` and its Template here
   too, admitted in `Action.deonticHeadOk`.
2. **Results via `asEvent`.** Write the base [CR#120.3a] and [CR#120.3e] rows and move the
   planeswalker and battle rows [CR#120.3c,120.3h] off `damageResult`. These are rules, not
   abilities: confer them as a property (`Conferral.property`, `Rules.lean:69`), not as an
   Ability, so ability removal cannot switch them off. Give lifelink its body [CR#702.15b];
   toxic is ruling 4c (one result reading the total toxic value). Files: `Rules.lean`, `Check/Rules.lean`, `Proofs/Rules.lean`,
   `Definitions/Keywords.lean`.
3. **Wither and infect via `replace`.** `markDamage` publishes `damageMarked` (today it publishes
   no outcome, so "that much" after it is refused). Wither replaces marking with -1/-1 counters
   [CR#702.80a]; infect replaces marking [CR#702.90c] and a player's life loss with poison
   counters [CR#702.90b]. Write infect's player row with `causedBy (.event (does … (dealDamage
   …)))`, not `.source`, so other life loss from the same source is not caught. A source with
   both wither and infect puts counters once [CR#120.3d,614.6]: pin it. Depends on step 2.
4. **Redirection and scaling via `replace`.** Redirection: `replace (would <src> (dealDamage …))
   (instead (redirectDamage cut to))`. Scaling: `replace (would <src> (dealDamage …)) (instead
   (instruct <that source> (dealDamage …)))`. Writing "that source" waits on
   [lean-v3-that-source-noun-word](../wip/lean-v3-that-source-noun-word.md) (`it` falls back to the
   Source: the *Deflecting Palm* defect). Redirection uses `redirectDamage`'s `cut` shield (4a) and the `to` checks (4d).
5. **Delete** the retired types, Templates, checker functions and arms, `RulesTables.damageResult`
   and `ScaleFactor`, and amend the docs: `lean/CONTRACTS.md:695-699,714,761,1174-1176`;
   `lean/README.md:83-90`; ADR `docs/decisions/semantics-v3.md:19` (the Contrast Anchor's
   "damage-rule" clauses and *Inquisitor's Flail*), `:25`, `:414`, §3.7 (`:449`), `:485`
   (Q86-Q88); glossary **Rules Table** (lists Damage Results as a row kind) and **Damage Result**
   (re-check "additive, not a replacement" against wither and infect replacing a result), and add
   **Redirection Effect** [CR#614.9] through `domain-modeling` if the landing uses the term.

6. **Fold *Shadowbane*'s `preventedFromSource`** (ruling 4e; `needs`
   `lean-v3-this-way-and-stop-trigger-folds`). Delete `Condition.preventedFromSource`
   (`Abilities.lean:477`) and its Template and checker arm; write *Shadowbane*'s rider as `did …
   (preventDamage …) thisWay` over the black-source prevention; rewrite its pins
   (`okPreventedFromSourceAnnounced :428`, `okPreventedFromSourceInAClause :433`,
   `badPreventedFromSourceUnannounced :438`) onto the `did` form with the same claims
   ("announced" and "in a clause" in scope; unannounced refused), keeping `okPreventedThisWay*`.

Order: steps 1 and 4 both rewrite `Cards/Damage.lean` and `Proofs/Damage.lean` (sequential);
steps 2 and 3 both touch `Definitions/Keywords.lean` and `Proofs/Rules.lean` (sequential); 1 and 2
may run side by side.

## Tests

Every `spelled` card is a check at elaboration. Rewrite each to the new tree with its claim kept:

- `Cards/Damage.lean`: `okFog :136`, `okIndestructibleAura :140`, `okShieldmatesBlessing :143`,
  `okGlacialChasmShield :154`, `okInkshieldRider :440`, `okGlarecasterShield :573`;
  `Cards/Deontic.lean`: `okWhippoorwillImmunity :287` (`cant any (redirectDamage …)`, third batch).
- `Proofs/Damage.lean`: `okPreventDealtToPermanent :402`, `okRedirectToSingleCreature :409`,
  `badRedirectToPlural :415`, `okPreventedFromSourceAnnounced :428`,
  `okPreventedFromSourceInAClause :433`, `badPreventedFromSourceUnannounced :438`,
  `okPreventedThisWayAnnounced :449`, `okPreventedThisWayInAClause :454`, `okShieldRepeatedly
  :503`, `badShieldSizedByItsOwnPrevention :509`, `okRedirectToOne :527`, `badRedirectToGroup
  :533`, `okScaleDoubled :548`, `badScaleShiftByThatMuch :555`, and the
  `preventLastChosenColor` pins (`:678,688,699`).
- `Proofs/Deontic.lean`: `okTheDamageAfterDealing :203`, `badTheDamageAfterLifeGain :211`,
  `okTheDamageAnnounced :223`, `badTheDamageUnannounced :229`.
- `Proofs/ContrastAnchor.lean`: `embermawHellionAnchorsOnTheSource :106`,
  `caromAnchorsOnTheFirstTarget :202`, `inquisitorsFlailAnchorsOnTheEquippedCreature :301`
  (asserts a referent id, which shifts with a `would`-clause tree: re-derive it, keep the anchor
  claim of ADR §0.1 step 2).
- `Proofs/Numbers.lean`: `okMultipliedDamageDeclaresNoSlot :146`, `okShiftedDamageClamped :152`
  move to the `dealDamage` slot over the scaled amount (keep the [CR#107.1b] note).
- `Proofs/KeywordHolders.lean`: `preventedRecipient?` `:49-51` and
  `protectionEnchantAndDamageReadItsHolder :57`.
- `Proofs/Rules.lean`: `damageResultCheck :20`, the planeswalker and battle rows and their `ok`
  pins (`:119-130`), `badDamageResultUndeclaredCounter :133` (the `.knownCounter` check is still
  wanted), `builtinTables :476-479`, `okBuiltinTables :481`.
- `Check.lean`: `slice_e_copy_and_prevent_pin_sound :283`.
- `badRedirectToPlural` and `badRedirectToGroup` are kept and assert `.singular` on `redirectDamage`'s
  `to` (third batch, 4d); they are not weakened to `.perMember`. Add `okShieldedRedirectionSizesItsCut`
  (*Harm's Way*'s "the next 2 damage" is `redirectDamage (.shield 2) …`, [CR#614.9,615.7]; replaces
  the planned `okShieldedRedirectionSizesItsUse`), `okPassiveRedirectionNeedsNoSource` (*Pariah*'s
  "All damage that would be dealt to you is dealt to X instead" checks clean), a redirect target of
  the wrong kind is refused `.kindLte`, and `whippoorwillCantIsADeonticOverRedirectDamage` (the
  `cant any (redirectDamage …)` tree checks clean; a plural target inside it is refused
  `.singular`; `Action.replace` is not a deontic head).
- Add `toxicIsOneResultReadingTheTotal` (ruling 4c: two toxic instances give one result row, not
  two) and, with step 6, `shadowbaneRiderIsDidPreventDamageThisWay`.
- Add: `witherAndInfectPutCountersOnce` (one `markDamage` replaced once [CR#614.6,120.3d]);
  `lifelinkIsAnAdditionalResult`; `cantBePreventedIsADeonticOverPreventDamage`;
  `planeswalkerDamageResultIsNotAnAbility`.

Canon witnesses carried from `semantics-v3-damage-results` (each Vintage-legal with matching
text): one lifelink, one wither and one infect card; *Everlasting Torment* and *Phyrexian Unlife*
("as though its source had wither/infect": decide whether the phrase composes the keyword's rows
by reference or is written out, and say which); *Archon of Coronation* ("damage doesn't cause you
to lose life") and *Soul-Scar Mage* if they fit.

## Other tickets on the same files

- `lean-v3-regeneration-rider-duration` cites *Pinpoint Avalanche* as a convention witness; this
  ticket rewrites its tree. The duration is now `some .modifiesPrecedingAction` (second-batch
  ruling 1), not `none`: `establish (cant any (preventDamage .all (damage := some thatDamage)))
  (some .modifiesPrecedingAction)`. Whichever lands second carries the duration over.
- `lean-v3-this-way-and-stop-trigger-folds` (B4): step 6 needs it, and it rewrites *Screaming
  Nemesis* and *Sonic Shrieker* in the same file.
- `lean-v3-cause-flag-fold` changes `loseCounters` (the planeswalker and battle rows) and adds the
  `.anEffect` Template; `Proofs/Rules.lean` is shared.
- `lean-v3-this-way-and-stop-trigger-folds` rewrites *Screaming Nemesis* and *Sonic Shrieker* in
  `Cards/Damage.lean` and pins in `Proofs/Damage.lean` (`stopTriggering`).
- [lean-v3-that-source-noun-word](../wip/lean-v3-that-source-noun-word.md) (*Deflecting Palm*,
  step 4).
- `lean-v3-rules-and-card-fidelity`: *Inkshield*'s `damageRule (kind := some .combat)` and *Temple
  Altisaur*'s missing "another" are the same cards; whichever lands second re-spells them.
- `lean-v3-authoring-surface` §5.4 lists `damageRule` among Templates to expose; drop it there.
- `lean-v3-keyword-bodies-and-catalog-sync` points lifelink, wither, infect and toxic here.
- `engine-damage-results-life-events` is the engine half; it agrees that wither and infect replace
  a result, not the damage.
