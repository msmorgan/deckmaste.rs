import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Choice
import Semantics.Cards.Damage
import Semantics.Cards.Counters
import Semantics.Cards.Description

/-!
# Semantics.Proofs.ContrastAnchor

What a bare "another"/"other" contrasts with (`semantics-v3.md` §0.1): a mention of the Source
coordinated with it; else the nearest earlier mention in scope; else a particular of the same
clause read forward; else the Source for an Object or "you" for a Player
[CR#702.151a,707.11,109.5]. Each pin reads the anchors the checker resolved
(`DRS.contrastAnchors`, latest first).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ContrastAnchor

/-- The Contrast Anchors an action resolves. -/
private def anchorsOf (action : Action) (drs : DRS := DRS.ofList []) : List ContrastAnchor :=
  (stepAction 64 .postEvent .obj drs action).1.contrastAnchors

/-- Each anchor that is an earlier mention, by its determiner and Card Types. -/
private def mentionFacts (drs : DRS) : List (Option (Determiner × List CardType)) :=
  drs.contrastAnchors.map fun
    | .mention id => (drs.findById? id).map fun ref => (ref.det, ref.ty)
    | _ => none

/-- `mentionFacts` after an action. -/
private def mentionsOf (action : Action) (drs : DRS := DRS.ofList []) :
    List (Option (Determiner × List CardType)) :=
  mentionFacts (stepAction 64 .postEvent .obj drs action).1

/-- The context of a face whose Card Types are `types`. -/
private def framed (types : List CardType) : DRS := { DRS.ofList [] with sourceTypes := types }

/-- A spell card's first spell ability. -/
private def spellOf (card : Spelled) : Action :=
  match card.card with
  | .singleFaced face => match face.characteristics.text with
    | .spell _ action :: _ => action
    | _ => .sequentially []
  | _ => .sequentially []

/-- The context a triggered ability's effect is read in, after its header and intervening "if",
and the effect. -/
private def triggerParts (drs : DRS) : Ability → DRS × Action
  | .triggered event alternatives while_ _ _ _ guard action =>
    let header := (stepTriggerHeaderWith (stepOps 64) drs event alternatives while_ none).1
    let guarded := match guard with
      | some condition => (stepCondition 64 (nextClause header) condition).1
      | none => header
    (nextClause guarded, action)
  | _ => (drs, .sequentially [])

/-- The Contrast Anchors a triggered ability's effect resolves. -/
private def effectAnchors (ability : Ability) (drs : DRS := DRS.ofList []) :
    List ContrastAnchor :=
  let (context, action) := triggerParts drs ability
  (anchorsOf action context).take
    ((anchorsOf action context).length - context.contrastAnchors.length)

/-! ## "Any other target": one predicate, two anchors -/

/-- Arc Trail and Screaming Nemesis both print "any other target" and write it alike. -/
theorem anyOtherTargetIsOneEncoding : isAnyOtherTarget = (and [isAnyTarget, isOther] : Predicate) :=
  rfl

/-- Arc Trail: "Arc Trail deals 2 damage to any target and 1 damage to any other target." The
nearest earlier mention is the first target. -/
theorem arcTrailAnchorsOnItsFirstTarget :
    mentionsOf Cards.arcTrail = [some (.target, [])] := by
  decide

/-- Screaming Nemesis: "Whenever Screaming Nemesis is dealt damage, it deals that much damage to
any other target." No earlier mention but the Source's own: the anchor is the Source. -/
theorem screamingNemesisAnchorsOnTheSource :
    let header := GameEvent.extend (framed [.creature])
      (.does .any (dealDamage (to := thisCreature)))
    anchorsOf (instruct it (dealDamage thatMuch (target isAnyOtherTarget))) (nextClause header)
      = [.source] := by
  decide

/-! ## No earlier target: the Source -/

/-- Olivia Voldaren, the example under [CR#707.11]: "Olivia Voldaren deals 1 damage to another
target creature." -/
theorem oliviaAnchorsOnTheSource :
    let olivia := instruct this_ (dealDamage 1 (target (and [isCreature, isOther])))
    Action.check (framed [.creature]) olivia = [] ∧ anchorsOf olivia (framed [.creature]) = [.source]
    := by
  decide

/-- Reconfigure [CR#702.151a]: "Attach this permanent to another target creature you control." -/
theorem reconfigureAnchorsOnTheSource :
    let attach := attachTo thisPermanent (target (and [isCreature, isOther, youControl]))
    Action.check (DRS.ofList []) attach = [] ∧ anchorsOf attach = [.source] := by
  decide

/-- Embermaw Hellion: "If another red source you control would deal damage to a permanent or
player, it deals that much damage plus 1 to that permanent or player instead." "A permanent or
player" is no particular, so no participant anchors it: the anchor is the Source. -/
theorem embermawHellionAnchorsOnTheSource :
    let rule := damageRule (dealtBy (a (and [isSource, hasColor .red, youControl, isOther])))
      (toRecipient (a (or [isPermanent, isPlayer]))) (scale (shifted (up 1))) .repeatedly
    let after := stepStaticSpec 64 (framed [.creature]) rule
    after.2 = [] ∧ after.1.contrastAnchors = [.source] := by
  decide

/-! ## The Source's fallback reads no Card Type -/

/-- A probe Equipment printing "Whenever another creature dies, draw a card": nothing is
mentioned earlier and the clause has no other participant, so the anchor is the Source, though
it is no creature. -/
def probeEquipment : Card := .singleFaced
  { characteristics :=
    { name := some "Probe", cost := some [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ whenever (does (a (and [isCreature, isOther])) die) (draw 1),
          equip (mana [generic 1]) ] } }

theorem okCreatureHeadedOtherOnANoncreatureSource :
    probeEquipment.check = [] ∧
      (GameEvent.extend (framed [.artifact])
        (does (a (and [isCreature, isOther])) die)).contrastAnchors = [.source] := by
  decide

/-- Goliath Truck: "Whenever this Vehicle attacks, put two +1/+1 counters on another target
attacking creature." Only creatures attack [CR#508.1a], and a Vehicle becomes one by crewing
[CR#301.7]; its card is no creature, and the anchor is the Source. -/
def goliathTruck : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goliath Truck", cost := some [generic 4], types := [.artifact],
      subtypes := [vehicle],
      text :=
        [ whenever (does thisVehicle attack)
            (putCounters 2 p1p1Counter (target (and [isCreature, isAttacking, isOther]))),
          crew 2 ],
      power := stat 4, toughness := stat 4 } }

theorem goliathTruckAnchorsOnTheSource :
    effectAnchors (whenever (does thisVehicle attack)
      (putCounters 2 p1p1Counter (target (and [isCreature, isAttacking, isOther]))))
      (framed [.artifact]) = [.source] := by
  decide

/-! ## An earlier mention need not be a target -/

/-- "Choose a player. Each other player draws a card." The chosen player, not "you". -/
theorem otherPlayerAnchorsOnTheChosenPlayer :
    mentionsOf (sequentially [choose (a isPlayer), instruct (each isOtherPlayer) (draw 1)])
      = [some (.a, [])] := by
  decide

/-- With no player mentioned first, "each other player" is other than "you". -/
theorem otherPlayerAnchorsOnYou :
    anchorsOf (sequentially [draw 2, instruct (each isOtherPlayer) (draw 1)])
      = [.you] := by
  decide

/-- Contractual Safeguard: "Choose a kind of counter on a creature you control. Put a counter of
that kind on each other creature you control." The anchor is "a creature you control", which
is no target. -/
theorem contractualSafeguardAnchorsOnTheChosenCreature :
    mentionsOf Cards.contractualSafeguardPass = [some (.a, [.creature])] := by
  decide

/-- Autumn-Tail, Kitsune Sage: "Attach target Aura attached to a creature to another creature."
The nearer mention, the Aura, is no creature; the anchor is "a creature". -/
theorem autumnTailAnchorsOnTheCreatureNotTheAura :
    mentionsOf (attachTo (target (and [hasSubtype aura, attachedTo (a isCreature)]))
      (a (and [isCreature, isOther]))) = [some (.a, [.creature])] := by
  decide

/-! ## The earlier targets of the corpus -/

/-- Phantom Blade: "When this Equipment enters, attach it to up to one target creature you
control. Destroy up to one other target creature." -/
theorem phantomBladeAnchorsOnTheAttachTarget :
    let (context, action) := triggerParts (DRS.ofList []) Cards.phantomBlade
    mentionFacts (stepAction 64 .postEvent .obj context action).1 = [some (.target, [.creature])]
    := by
  decide

/-- Deadshot: "Tap target creature. It deals damage equal to its power to another target
creature." -/
theorem deadshotAnchorsOnTheTappedTarget :
    mentionsOf Cards.deadshot = [some (.target, [.creature])] := by
  decide

/-- Ulvenwald Tracker: "Target creature you control fights another target creature." -/
theorem ulvenwaldTrackerAnchorsOnTheFirstTarget :
    mentionsOf Cards.ulvenwaldTracker = [some (.target, [.creature])] := by
  decide

/-- Carom: "The next 1 damage that would be dealt to target creature this turn is dealt to
another target creature instead." -/
theorem caromAnchorsOnTheFirstTarget :
    mentionsOf (spellOf Cards.carom) (framed [.instant]) = [some (.target, [.creature])] := by
  decide

/-- Feral Contest: "Put a +1/+1 counter on target creature you control. Another target creature
blocks it this turn if able." -/
theorem feralContestAnchorsOnTheFirstTarget :
    mentionsOf (spellOf Cards.feralContest) (framed [.sorcery]) = [some (.target, [.creature])]
    := by
  decide

/-- "another creature or land", after a target on the battlefield. -/
theorem anotherDisjunctPhraseAnchorsOnTheTarget :
    let target : Referent :=
      { det := .target, plur := .one, payload := .object [] (some .battlefield) none none none }
    (stepPredicate 64 { DRS.ofList [target] with relativeKind := some .object }
      Cards.anotherDisjunctPhrase).1.contrastAnchors = [.mention 0] := by
  decide

/-! ## A Condition's referents are hypothetical -/

/-- Flames of the Raze-Boar: "… deals 4 damage to target creature an opponent controls. Then …
deals 2 damage to each other creature that player controls if you control a creature with power
4 or greater." The condition's creature is no candidate; the anchor is the target. -/
theorem flamesOfTheRazeBoarAnchorsOnTheTarget :
    mentionsOf Cards.flamesOfTheRazeBoar = [some (.target, [.creature])] := by
  decide

private def soulbondFirst : Ability :=
  match Definitions.soulbond with
  | .keyword _ _ (first :: _) => first
  | other => other

/-- Soulbond [CR#702.95a]: "When this creature enters, if you control both this creature and
another creature and both are unpaired, you may pair this creature with another unpaired
creature you control …" The second "another" does not anchor on the condition's creature: both
contrast with this creature, the Source. -/
theorem soulbondAnchorsOnTheSource :
    let (context, effect) := triggerParts (enterOwnScope (DRS.ofList [])) soulbondFirst
    let paired := match effect with
      | .withContinuation _ body _ _ => body
      | other => other
    context.contrastAnchors = [.source] ∧
      anchorsOf paired context = [.source, .source] := by
  decide

/-! ## The described thing is never its own anchor -/

/-- Backup [CR#702.165a]: "Put a +1/+1 counter on target creature. If that's another creature,
…" The subject of `is` is no candidate, so the contrast is with this creature, the Source. -/
theorem backupAnchorsOnTheSourceNotThatCreature :
    let put := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (putCounters 1 p1p1Counter (target isCreature))).1
    (stepCondition 64 (nextClause put) (is (that creature) (and [isCreature, isOther]))).1
      |>.contrastAnchors = [.source] := by
  decide

/-- Unexpected Allies: "Target creature you control gets +2/+0. It gains first strike if it has
the same name as another creature you control." The subject of `is` is excluded only from its
own predicate's "another", not from a noun phrase nested in it: the anchor is "it", the target.
(The effects are written as taps.) -/
theorem unexpectedAlliesAnchorsOnIt :
    mentionsOf (sequentially
      [ tap (target (and [isCreature, youControl])),
        doOnlyIf (tap it)
          (is it (shares .cardName .share (a (and [isCreature, youControl, isOther])))) ])
      (framed [.instant]) = [some (.target, [.creature])] := by
  decide

/-! ## Every anchor fits the head's controller -/

/-- Bartz and Boko: "When Bartz and Boko enters, each other Bird you control deals damage equal
to its power to target creature an opponent controls." The target, controlled by an opponent,
is no anchor for a Bird you control: the anchor is the Source. -/
theorem bartzAndBokoAnchorsOnTheSource :
    anchorsOf (dealDamageOwnPower (each (and [hasSubtype bird, youControl, isOther]))
        (target (and [isCreature, anOpponentControls])))
      (framed [.creature]) = [.source] := by
  decide

/-! ## A mention of the Source coordinated with the phrase -/

/-- Slimefoot and Squee: "Sacrifice a Saproling: Return this card and up to one other target
creature card from your graveyard to the battlefield." The sacrificed Saproling is the nearer
earlier mention in the graveyard; the anchor is this card, coordinated with the phrase. -/
theorem slimefootAndSqueeAnchorsOnThisCard :
    let cost := (stepCost 64 (framed [.creature]) (perform (sacrifice (a isCreature)))).1
    let after := (stepNounPhrase 64 (nextClause cost) .obj true
      (both thisCard (targets (upTo 1) (and [isCreature, isCard, isOther, in_ yourGraveyard])))).1
    (after.contrastAnchors.map fun
      | .mention id => (after.findById? id).map (·.defaultAnchor)
      | _ => none) = [some true] := by
  decide

/-! ## A particular of the same clause, read forward -/

/-- Inquisitor's Flail: "If another creature would deal combat damage to equipped creature, it
deals double that damage to equipped creature instead." No creature is mentioned earlier: the
anchor is the clause's recipient, the equipped creature. -/
theorem inquisitorsFlailAnchorsOnTheEquippedCreature :
    let rule := damageRule (dealtBy (a (and [isCreature, isOther])))
      (toRecipient (equipped creature)) (scale (multiplied .doubled)) .repeatedly (some .combat)
    let after := stepStaticSpec 64 (framed [.artifact]) rule
    let recipient := after.1.findById? 1
    after.2 = [] ∧ recipient.map (·.attachmentHost) = some true ∧
      after.1.contrastAnchors = recipient.toList.map (fun ref => ContrastAnchor.participant ref.payload)
    := by
  decide

/-- Mirrorweave: "Each other creature becomes a copy of target nonlegendary creature until end
of turn." The copy effect's subject reads forward to the target. -/
theorem mirrorweaveAnchorsOnTheTarget :
    let copy := copyChange (each (and [isCreature, isOther]))
      (target (and [isCreature, non (hasSupertype .legendary)]))
    let after := stepStaticSpec 64 (framed [.instant]) copy
    after.2 = [] ∧
      (after.1.contrastAnchors.map fun
        | .participant facts => some facts.ty
        | _ => none) = [some [.creature]] := by
  decide

end Semantics.Proofs.ContrastAnchor
