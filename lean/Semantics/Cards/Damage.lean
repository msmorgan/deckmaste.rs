import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Description
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Damage

Port of `idris/src/Experimental/Cards/Damage.idr`: the printed cards of the Damage family and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Lightning Bolt -/
def bolt : Action := Primitives.Action.dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly
theorem okBolt : Action.check (DRS.ofList []) bolt = [] := by decide
def barrageOfBoulders : Action := Primitives.Action.dealDamage (.lit 1) (each (and [isCreature, youDontControl])) .noncombatOnly
theorem okBarrageOfBoulders : Action.check (DRS.ofList []) barrageOfBoulders = [] := by decide
def rabidBite : Action :=
  instruct (target (and [isCreature, youControl])) (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) it) (target (and [isCreature, youDontControl])) .noncombatOnly)
theorem okRabidBite : Action.check (DRS.ofList []) rabidBite = [] := by decide
def preyUpon : Action := instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl])))
theorem okPreyUpon : Action.check (DRS.ofList []) preyUpon = [] := by decide
def arcTrail : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly, Primitives.Action.dealDamage (.lit 1) (target isAnyOtherTarget) .noncombatOnly ]
theorem okArcTrail : Action.check (DRS.ofList []) arcTrail = [] := by decide
def deadshot : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.setStatus .tapped (target isCreature),
      instruct it (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) it) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.other])) .noncombatOnly) ]
theorem okDeadshot : Action.check (DRS.ofList []) deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .black, pip .red, pip .red], Primitives.Cost.tapSymbol,
      Primitives.Cost.perform (sacrifice thisLand)])
    (Primitives.Action.sequentially
      [ Primitives.Action.dealDamage (.lit 3) (target Primitives.Predicate.anyPlayer) .noncombatOnly, discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (that
          .player))])) ])
    asSorcery
theorem okImmersturmSkullcairn : Ability.check (DRS.ofList []) immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.perform (sacrifice thisArtifact)])
    (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)
theorem okPyriteSpellbomb : Ability.check (DRS.ofList []) pyriteSpellbomb = [] := by decide
def karplusanYeti : Action :=
  Primitives.Action.sequentially
    [ dealDamageOwnPower thisCreature (target isCreature),
      instruct (that (.type .creature)) (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) it) thisCreature .noncombatOnly) ]
theorem okKarplusanYeti : Action.check (DRS.ofList []) karplusanYeti = [] := by decide
def suddenDemise : Action :=
  Primitives.Action.sequentially
    [ choose (a (quality .color)),
      Primitives.Action.dealDamage (Primitives.Amount.letter .x) (each (Primitives.Predicate.and [isCreature, ofChosen .color])) .noncombatOnly ]
theorem okSuddenDemise : Action.check (DRS.ofList []) suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Action :=
  Primitives.Action.sequentially
    [ choose (target (and [isCreature, youDontControl])),
      instruct (each (and [isCreature, youControl])) (Primitives.Action.dealDamage (.lit 1) (that (.type .creature)) .noncombatOnly) ]
theorem okCaseOfTheGatewayExpress : Action.check (DRS.ofList []) caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Action :=
  Primitives.Action.dealDamage (.lit 4) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.or [isAttacking, isBlocking]])) .noncombatOnly
theorem okArrowsOfJustice : Action.check (DRS.ofList []) arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 4) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller anOpponent])) .noncombatOnly,
      Primitives.Action.doOnlyIf
        (Primitives.Action.dealDamage (.lit 2) (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.other, Primitives.Predicate.hasPossessor .controller (that .player)])) .noncombatOnly)
        (exists_ (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))
        none ]
theorem okFlamesOfTheRazeBoar : Action.check (DRS.ofList []) flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Action :=
  Primitives.Action.offer (sacrifice (a isArtifact)) none
    (some (Primitives.Action.sequentially [Primitives.Action.setStatus .tapped thisCreature, instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) Primitives.NounPhrase.you .noncombatOnly)]))
theorem okYawgmothDemon : Action.check (DRS.ofList []) yawgmothDemon = [] := by decide
def arcBlade : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly, exileWithCounters Primitives.NounPhrase.this (.lit 3) (.named "timeCounter") ]
theorem okArcBlade : Action.check (DRS.ofList []) arcBlade = [] := by decide
def abrade : Action :=
  chooseModes (exactly 1) [Primitives.Action.dealDamage (.lit 3) (target isCreature) .noncombatOnly, destroy (target isArtifact)]
theorem okAbrade : Action.check (DRS.ofList []) abrade = [] := by decide
def nibelheimAflame : Action :=
  Primitives.Action.sequentially
    [ choose (target (and [isCreature, youControl])),
      instruct it (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) it) (each ((and [isCreature, otherThan it]))) .noncombatOnly) ]
theorem okNibelheimAflame : Action.check (DRS.ofList []) nibelheimAflame = [] := by decide
def brashTaunter : Action := instruct thisCreature (fight (target ((and [isCreature, otherThan thisCreature]))))
theorem okBrashTaunter : Action.check (DRS.ofList []) brashTaunter = [] := by decide
def ulvenwaldTracker : Action :=
  instruct (target (and [isCreature, youControl])) (fight (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.other])))
theorem okUlvenwaldTracker : Action.check (DRS.ofList []) ulvenwaldTracker = [] := by decide

def botBashingTime : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 6) (target isCreature) .noncombatOnly,
      replaceEvent (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (that (.type .creature)) (battlefield) (some graveyard) .departure none [])) (exile it) (some thisTurn) ]
theorem okBotBashingTime : Action.check (DRS.ofList []) botBashingTime = [] := by decide
def wordsOfWar : Action :=
  replaceNextEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.draw Primitives.Amount.anyAmount)) (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) (some thisTurn)
theorem okWordsOfWar : Action.check (DRS.ofList []) wordsOfWar = [] := by decide

/-- Thunderwave -/
def thunderwave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderwave", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ rollDice (.lit 1) (.sides 20),
              Primitives.Action.applyResultsTable
                [ rollRow (fromTo 1 9) (Primitives.Action.dealDamage (.lit 3) (each isCreature) .noncombatOnly),
                  rollRow (fromTo 10 19)
                    (Primitives.Action.sequentially
                      [ may (choose (a isCreature)),
                        Primitives.Action.dealDamage (.lit 3) (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.notChosen])) .noncombatOnly ]),
                  rollRow (fromTo 20 20)
                    (Primitives.Action.dealDamage (.lit 6) (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))])) .noncombatOnly) ] ]) ] } }

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Action := preventAll .combatOnly Primitives.DamageScope.everywhere (some thisTurn)
theorem okFog : Action.check (DRS.ofList []) fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Action :=
  preventAll .any (Primitives.DamageScope.toRecipient (target isCreature)) (some thisTurn)
theorem okIndestructibleAura : Action.check (DRS.ofList []) indestructibleAura = [] := by decide
def shieldmatesBlessing : Action :=
  preventNext .any (Primitives.DamageScope.toRecipient (target isAnyTarget)) (.lit 3) (some thisTurn)
theorem okShieldmatesBlessing : Action.check (DRS.ofList []) shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := whenever (does (actor (a isCreature)) die) (gainLife (.lit 1))
theorem okMoonlitWake : Ability.check (DRS.ofList []) moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (does (actor thisCreature) block) (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (target (Primitives.Predicate.and [isCreature, isAttacking])) .noncombatOnly))
theorem okEliteJavelineer : Ability.check (DRS.ofList []) eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly)
theorem okGlacialChasmShield : Ability.check (DRS.ofList []) glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (does (actor thisCreature) enter)
    (offerWhen (sacrifice (a isArtifact)) (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target
        isAnyTarget) .noncombatOnly)))
theorem okCorneredCrook : Ability.check (DRS.ofList []) corneredCrook = [] := by decide

def aladdinsRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aladdin's Ring", cost := some [generic 8], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 8], Primitives.Cost.tapSymbol])
            (instruct thisArtifact (Primitives.Action.dealDamage (.lit 4) (target isAnyTarget) .noncombatOnly)) ] } }

def chandrasRevolution : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 4) (target isCreature) .noncombatOnly, Primitives.Action.setStatus .tapped (target isLand),
      Primitives.Action.skipUntap (that (.type .land)) (.lit 1) ]
theorem okChandrasRevolution : Action.check (DRS.ofList []) chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .white], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.or [isAttacking, isBlocking]])) .noncombatOnly),
        Primitives.Action.skipUntap thisCreature (.lit 1) ])
theorem okArbalestElite : Ability.check (DRS.ofList []) arbalestElite = [] := by decide
def sizzlingBarrage : Action :=
  Primitives.Action.dealDamage (.lit 4) (target
    (Primitives.Predicate.and [isCreature, happenedTo (Primitives.Subject.actor (Primitives.NounPhrase.asMarker .permanent (relative .object))) (Primitives.Action.block (Primitives.NounPhrase.any)) soFarThisTurn])) .noncombatOnly
theorem okSizzlingBarrage : Action.check (DRS.ofList []) sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasDesignation (.named "goaded") none]))) attack)
    (instruct it (Primitives.Action.dealDamage (.lit 1) (controllerOf it) .noncombatOnly))
theorem okGoadedAttackTrigger : Ability.check (DRS.ofList []) goadedAttackTrigger = [] := by decide

def extraArms : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extra Arms", cost := some [generic 4, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does (actor (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) attack)
            (instruct it (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) ] } }

def chainReaction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chain Reaction", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) (each isCreature) .noncombatOnly, Primitives.Action.define .x (countOf isCreature) ]) ] } }

/-- Black Vise -/
def blackVise : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Vise", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisArtifact (some (Primitives.ChoiceDomain.players (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))),
          at_ (beginningOfPossessed .upkeep (the isChosenPlayer))
            (Primitives.Action.sequentially
              [ instruct thisArtifact (Primitives.Action.dealDamage (Primitives.Amount.letter .x) (that .player) .noncombatOnly),
                Primitives.Action.define .x (Primitives.Amount.arith .minus (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf they)])) (.lit 4)) ]) ] } }

def harshSustenance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harsh Sustenance", cost := some [generic 1, pip .white, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) (target isAnyTarget) .noncombatOnly,
              gainLife (Primitives.Amount.letter .x),
              Primitives.Action.define .x (countOf (and [isCreature, youControl])) ]) ] } }

/-- Purging Scythe -/
def purgingScythe : Ability :=
  at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Action.sequentially
      [ instruct thisArtifact (Primitives.Action.dealDamage (.lit 2) (the (Primitives.Predicate.and [isCreature, Primitives.Predicate.superlative .min (.stat .toughness) isCreature])) .noncombatOnly),
        Primitives.Action.doIf
          (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.superlative .min (.stat .toughness) isCreature]))
            .atLeast (.lit 2))
          (choose (someOf (exactly 1) them)) none ])
theorem okPurgingScythe : Ability.check (DRS.ofList []) purgingScythe = [] := by decide

def lionHeart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lion Heart", cost := some [generic 4], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (does (actor thisEquipment) enter) (instruct it (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)),
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1))),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

def lightmineField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightmine Field", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (does (actor (counted (atLeast 1) isCreature)) attack)
            (instruct thisEnchantment (Primitives.Action.dealDamage (Primitives.Amount.countOf (those (.type .creature))) (Primitives.NounPhrase.eachOf (those (.type .creature))) .noncombatOnly)) ] } }

def ingeniousArtillerist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ingenious Artillerist", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever (does (actor (counted (atLeast 1) (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) enter)
            (instruct thisCreature (Primitives.Action.dealDamage Primitives.Amount.groupSize (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)) ],
      power := stat 3, toughness := stat 1 } }

/-- Inferno Elemental -/
def infernoElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno Elemental", cost := some [generic 4, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredOr (does (actor thisCreature) (block (a isCreature)))
            [does (by_ (a isCreature)) (block thisCreature)]
            (instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (that (.type .creature)) .noncombatOnly)) ],
      power := stat 4, toughness := stat 4 } }

/-- Psychic Purge -/
def psychicPurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Psychic Purge", cost := some [pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly),
          when
            (Primitives.GameEvent.causes
              (Primitives.Causing.source (a (Primitives.Predicate.and [Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyOnStack],
                                 Primitives.Predicate.hasPossessor .controller (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))])))
              (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (discard (Primitives.NounPhrase.this))))
            (instruct (that .player) (loseLife (.lit 5))) ] } }

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (Primitives.Cost.loyaltySymbol .downX) (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (Primitives.Amount.letter .x) (target isCreature) .noncombatOnly))
theorem okChandraNalaarsX : Ability.check (DRS.ofList []) chandraNalaarsX = [] := by decide

def etherealHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ethereal Haze", cost := some [pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text := [Primitives.Ability.spell none (preventAllBy .any (allOf isCreature) Primitives.DamageScope.everywhere (some thisTurn))] } }

def defang : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defang", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) Primitives.DamageScope.everywhere
            (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly) ] } }

def sphereOfPurity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Purity", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isArtifact)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ] } }

def dazzlingReflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dazzling Reflection", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ gainLife (Primitives.Amount.statOf (.stat .power) (target isCreature)),
              Primitives.Action.establish
                (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (that (.type .creature))) Primitives.DamageScope.everywhere (Primitives.DamageOp.prevent Primitives.PreventCut.all none)
                  .nextTimeOnly)
                (some thisTurn) ]) ] } }

def thunderstaff : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderstaff", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (a isCreature)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly)
            (Primitives.Condition.matches thisArtifact isUntapped)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (get (allOf (Primitives.Predicate.and [isCreature, isAttacking])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0))
              (some untilEndOfTurn)) ] } }

def gideonAllyOfZendikar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon, Ally of Zendikar", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Action.sequentially
              [ Primitives.Action.establish
                  (Primitives.StaticSpec.qualityChange thisPlaneswalker .sets
                    (Primitives.QualityPayload.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier", creatureType "Ally"],
                          text := [keyword (.named "indestructible")], power := stat 5, toughness := stat 5 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (Primitives.DamageScope.toRecipient it) (some thisTurn) ]),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (createToken (creatureToken 2 2 [.white] [creatureType "Knight", creatureType "Ally"])),
          activated (Primitives.Cost.loyaltySymbol (.down 4))
            (Primitives.Action.getEmblem
              [Primitives.Ability.static (getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))]) ],
      loyalty := stat 4 } }

def turnTheTables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn the Tables", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .combatOnly Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.redirect Primitives.PreventCut.all (target (Primitives.Predicate.and [isCreature, isAttacking]))) .repeatedly)
            (some thisTurn)) ] } }

def pariah : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pariah", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.redirect Primitives.PreventCut.all (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) .repeatedly) ] } }

def martyrsOfKorlis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Martyrs of Korlis", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf isArtifact)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.redirect Primitives.PreventCut.all thisCreature) .repeatedly)
            (Primitives.Condition.matches thisCreature isUntapped)) ],
      power := stat 1, toughness := stat 6 } }

def wardOfPiety : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward of Piety", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          activated (Primitives.Cost.mana [generic 1, pip .white])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
                (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) (target isAnyTarget)) .repeatedly)
              (some thisTurn)) ] } }

def mirrorwoodTreefolk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirrorwood Treefolk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ activated (Primitives.Cost.mana [generic 2, pip .red, pip .white])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient thisCreature)
                (Primitives.DamageOp.redirect Primitives.PreventCut.all (target isAnyTarget)) .nextTimeOnly)
              (some thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def carom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carom", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish
                (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target isCreature))
                  (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.other]))) .repeatedly)
                (some thisTurn),
              Primitives.Action.draw (.lit 1) ]) ] } }

def daughterOfAutumn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daughter of Autumn", cost := some [generic 2, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Avatar"],
      text :=
        [ activated (Primitives.Cost.mana [pip .white])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.colorIs .white])))
                (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) thisCreature) .repeatedly)
              (some thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def aegisOfHonor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aegis of Honor", cost := some [pip .white], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isInstantOrSorcery)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.redirect Primitives.PreventCut.all (controllerOf it)) .nextTimeOnly)
              (some thisTurn)) ] } }

def candlesGlow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles' Glow", cost := some [generic 1, pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target isAnyTarget))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3)) (some (gainLife preventedThisWay)))
                  .repeatedly)
            (some thisTurn)) ] } }

/-- Inkshield -/
def inkshieldRider : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
      (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (createToken (creatureToken 2 1 [.white, .black] []) (count := preventedThisWay))))
      .repeatedly)
    (some thisTurn)
theorem okInkshieldRider : Action.check (DRS.ofList []) inkshieldRider = [] := by decide

def urzasArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Armor", cost := some [generic 6], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSource)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ] } }

def circleOfProtectionRed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Circle of Protection: Red", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice (Primitives.Predicate.and [isSource, Primitives.Predicate.colorIs .red]))) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .nextTimeOnly)
              (some thisTurn)) ] } }

def healingGrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Healing Grace", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish
                (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource)) (Primitives.DamageScope.toRecipient (target isAnyTarget))
                  (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3)) none) .repeatedly)
                (some thisTurn),
              gainLife (.lit 3) ]) ] } }

def reverseDamage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reverse Damage", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (gainLife preventedThisWay))) .nextTimeOnly)
            (some thisTurn)) ] } }

def deflectingPalm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deflecting Palm", cost := some [pip .red, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage Primitives.Amount.thatMuch (controllerOf it) .noncombatOnly)))) .nextTimeOnly)
            (some thisTurn)) ] } }

def templeAltisaur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temple Altisaur", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Dinosaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSource))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Dinosaur"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.DamageOp.prevent (Primitives.PreventCut.allBut (.lit 1)) none) .repeatedly) ],
      power := stat 3, toughness := stat 4 } }

def darkSphere : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Sphere", cost := some [], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact)])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.prevent (Primitives.PreventCut.half .down) none) .nextTimeOnly)
              (some thisTurn)) ] } }

def shadowbane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadowbane", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource))
              (Primitives.DamageScope.toRecipient (youAnd (allOf (and [isCreature, youControl]))))
              (Primitives.DamageOp.prevent Primitives.PreventCut.all
                (some (Primitives.Action.doIf (Primitives.Condition.preventedFromSource (Primitives.Predicate.and [isSource, Primitives.Predicate.colorIs .black]))
                  (gainLife preventedThisWay) none)))
              .nextTimeOnly)
            (some thisTurn)) ] } }

def sphereOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Law", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [isSource, Primitives.Predicate.colorIs .red]))) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 2)) none) .repeatedly) ] } }

def lavaAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lava Axe", cost := some [generic 4, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.dealDamage (.lit 5) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer])) .noncombatOnly) ] } }

def searingFlesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Searing Flesh", cost := some [generic 6, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.dealDamage (.lit 7) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)])) .noncombatOnly) ] } }

def onakkeJavelineerBolt : Ability :=
  activated Primitives.Cost.tapSymbol
    (instruct thisCreature (Primitives.Action.dealDamage (.lit 2) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .battle, Primitives.Predicate.anyPlayer])) .noncombatOnly))
theorem okOnakkeJavelineerBolt : Ability.check (DRS.ofList []) onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target (Primitives.Predicate.or [isCreature, Primitives.Predicate.anyPlayer])) .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) thatJoin .noncombatOnly) ]
theorem okFiresongJoinEcho : Action.check (DRS.ofList []) firesongJoinEcho = [] := by decide

/-- Forcefield -/
def forcefield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forcefield", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Action.establish
              (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (aYourChoice (Primitives.Predicate.and [isCreature, isUnblocked])))
                (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent (Primitives.PreventCut.allBut (.lit 1)) none) .nextTimeOnly)
              (some thisTurn)) ] } }

def endure : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endure", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (preventAll .any
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (some thisTurn)) ] } }

def harmsWay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harm's Way", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice isSource))
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 2)) (target isAnyTarget)) .repeatedly)
            (some thisTurn)) ] } }

def divineDeflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Deflection", cost := some [.variable, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (Primitives.Amount.letter .x)) (some (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage Primitives.Amount.thatMuch (target isAnyTarget) .noncombatOnly))))
              .repeatedly)
            (some thisTurn)) ] } }

def glarecasterShield : Ability :=
  activated (Primitives.Cost.mana [generic 5, pip .white])
    (Primitives.Action.establish
      (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (youAnd thisCreature))
        (Primitives.DamageOp.redirect Primitives.PreventCut.all (target isAnyTarget)) .nextTimeOnly)
      (some thisTurn))
theorem okGlarecasterShield : Ability.check (DRS.ofList []) glarecasterShield = [] := by decide

def furnaceOfRath : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Furnace of Rath", cost := some [generic 1, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSource))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly) ] } }

def gratuitousViolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gratuitous Violence", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (and [isCreature, youControl])))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly) ] } }

def fieryEmancipation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiery Emancipation", cost := some [generic 3, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [isSource, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .tripled))
            .repeatedly) ] } }

def sulfuricVapors : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sulfuric Vapors", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [isSpell, Primitives.Predicate.colorIs .red])))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.shifted .up (.lit 1)))
            .repeatedly) ] } }

def lashknifeBarrier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lashknife Barrier", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (does (actor thisEnchantment) enter) (Primitives.Action.draw (.lit 1)),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSource)) (Primitives.DamageScope.toRecipient (a (and [isCreature, youControl])))
            (Primitives.DamageOp.scale (Primitives.DamageScale.shifted .down (.lit 1))) .repeatedly) ] } }

def ghostsOfTheInnocent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghosts of the Innocent", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSource))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.halved .down)) .repeatedly) ],
      power := stat 4, toughness := stat 5 } }

def fireServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fire Servant", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any
            (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [isInstantOrSorcery, Primitives.Predicate.colorIs .red, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            Primitives.DamageScope.everywhere (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly) ],
      power := stat 4, toughness := stat 3 } }

def platedPegasus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plated Pegasus", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Pegasus"],
      text :=
        [ keyword (.named "flash"),
          keyword (.named "flying"),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a isSpell)) (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [isPermanent, Primitives.Predicate.anyPlayer])))
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ],
      power := stat 1, toughness := stat 2 } }

def spitemare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spitemare", cost := some [generic 2, hybridPip .red .white, hybridPip .red .white],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (does Primitives.Subject.any (dealDamage (to := thisCreature))) (instruct it (Primitives.Action.dealDamage Primitives.Amount.thatMuch (target isAnyTarget) .noncombatOnly)) ],
      power := stat 3, toughness := stat 3 } }

def grollub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grollub", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Beast"],
      text := [whenever (does Primitives.Subject.any (dealDamage (to := thisCreature))) (instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (gainLife Primitives.Amount.thatMuch))],
      power := stat 3, toughness := stat 3 } }

def moggManiac : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Maniac", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (does Primitives.Subject.any (dealDamage (to := thisCreature)))
            (instruct it (Primitives.Action.dealDamage Primitives.Amount.thatMuch (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)])) .noncombatOnly)) ],
      power := stat 1, toughness := stat 1 } }

def repercussion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repercussion", cost := some [generic 1, pip .red, pip .red], types := [.enchantment],
      text :=
        [ whenever (does Primitives.Subject.any (dealDamage (to := a isCreature)))
            (instruct thisEnchantment (Primitives.Action.dealDamage Primitives.Amount.thatMuch (controllerOf (that (.type .creature))) .noncombatOnly)) ] } }

def spitefulShadows : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spiteful Shadows", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does Primitives.Subject.any (dealDamage (to := Primitives.NounPhrase.attachHost .enchanted (.type .creature))))
            (instruct it (Primitives.Action.dealDamage Primitives.Amount.thatMuch (controllerOf it) .noncombatOnly)) ] } }

def bindingAgony : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Binding Agony", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does Primitives.Subject.any (dealDamage (to := Primitives.NounPhrase.attachHost .enchanted (.type .creature))))
            (instruct thisAura (Primitives.Action.dealDamage Primitives.Amount.thatMuch (controllerOf (that (.type .creature))) .noncombatOnly)) ] } }

def darienKingOfKjeldor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darien, King of Kjeldor", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (does Primitives.Subject.any (dealDamage (to := Primitives.NounPhrase.you)))
            (may (createToken (creatureToken 1 1 [.white] [creatureType "Soldier"]) (count := Primitives.Amount.thatMuch))) ],
      power := stat 3, toughness := stat 3 } }

def screamingNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Screaming Nemesis", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "haste"),
          whenever (does Primitives.Subject.any (dealDamage (to := thisCreature)))
            (Primitives.Action.sequentially
              [ instruct it (Primitives.Action.dealDamage Primitives.Amount.thatMuch (target (Primitives.Predicate.and [isAnyTarget, Primitives.Predicate.otherThan Primitives.NounPhrase.this])) .noncombatOnly),
                Primitives.Action.doIf (Primitives.Condition.dealtThisWay Primitives.Predicate.anyPlayer)
                  (Primitives.Action.establish (cant (actor they) (gainLife anyAmount)) (some restOfGame)) none ]) ],
      power := stat 3, toughness := stat 3 } }

def sonicShrieker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sonic Shrieker", cost := some [generic 2, pip .red, pip .white, pip .black],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          when (does (actor thisCreature) enter)
            (Primitives.Action.sequentially
              [ instruct it (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly),
                gainLife (.lit 2),
                Primitives.Action.doIf (Primitives.Condition.dealtThisWay Primitives.Predicate.anyPlayer) (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf they)]))) none
                    ]) ],
      power := stat 4, toughness := stat 4 } }

def grievousWoundLifeLock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grievous Wound", cost := some [generic 3, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") Primitives.Predicate.anyPlayer,
          Primitives.Ability.static (cant (actor (Primitives.NounPhrase.attachHost .enchanted .player)) (gainLife anyAmount)),
          whenever (does Primitives.Subject.any (dealDamage (to := Primitives.NounPhrase.attachHost .enchanted .player)))
            (instruct they (loseLife (Primitives.Amount.half .up (lifeTotalOf they)))) ] } }

def cursedScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Scroll", cost := some [generic 1], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ choose (a (quality .cardName)),
                reveal (aAtRandom (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])),
                Primitives.Action.doIf (Primitives.Condition.matches (that .card) (Primitives.Predicate.named Primitives.NameSource.chosen))
                  (instruct thisArtifact (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) none ]) ] } }

def magusOfTheScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Scroll", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ choose (a (quality .cardName)),
                reveal (aAtRandom (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])),
                Primitives.Action.doIf (Primitives.Condition.matches (that .card) (Primitives.Predicate.named Primitives.NameSource.chosen))
                  (instruct thisCreature (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) none ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Stuffy Doll -/
def stuffyDoll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stuffy Doll", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ keyword (.named "indestructible"),
          Primitives.Ability.static (entersChoosingPlayer thisCreature none),
          whenever (does Primitives.Subject.any (dealDamage (to := thisCreature))) (instruct it (Primitives.Action.dealDamage Primitives.Amount.thatMuch (the isChosenPlayer) .noncombatOnly)),
          activated Primitives.Cost.tapSymbol (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) thisCreature .noncombatOnly)) ],
      power := stat 0, toughness := stat 1 } }

def saheeliRaiPlusOne : Action :=
  Primitives.Action.sequentially [scry (.lit 1), instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)]
theorem okSaheeliRaiPlusOne : Action.check (DRS.ofList []) saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and [isCreature, isSpell,
                Primitives.Predicate.or [ Primitives.Predicate.compare [.stat .power] .eq (.lit 4), Primitives.Predicate.compare [.stat .power] .eq (.lit 5),
                      Primitives.Predicate.compare [.stat .power] .eq (.lit 6) ]]))) Primitives.PlayPayment.itsOwnCost none none none))
    (instruct thisEnchantment (Primitives.Action.dealDamage (.lit 4) (target isAnyTarget) .noncombatOnly))
theorem okSarkhansUnsealingLine : Ability.check (DRS.ofList []) sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.doOnlyIf (get (target (and [isCreature, youControl])) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some
        untilEndOfTurn))
        (Primitives.Condition.compareAmt (Primitives.Amount.statOf (.stat .power) it) .eq (.lit 2)) none,
      instruct it (fight (target (and [isCreature, youDontControl]))) ]
theorem okSavageSwipeLine : Action.check (DRS.ofList []) savageSwipeLine = [] := by decide

def infernoOfTheStarMounts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno of the Star Mounts", cost := some [generic 4, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ Primitives.Ability.static (cant Primitives.Subject.any (counter Primitives.NounPhrase.this)),
          keyword (.named "flying"),
          keyword (.named "haste"),
          activated (Primitives.Cost.mana [pip .red])
            (Primitives.Action.triggerThisWay (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
              (Primitives.GameEvent.statBecomes it .power (.lit 20))
              (instruct it (Primitives.Action.dealDamage (.lit 20) (target isAnyTarget) .noncombatOnly))) ],
      power := stat 6, toughness := stat 6 } }

def incinerate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Incinerate", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly,
              Primitives.Action.establish
                (cant Primitives.Subject.any (regeneration (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.did (⟨Primitives.Subject.any, (Primitives.Action.dealDamage Primitives.Amount.anyAmount (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any), Primitives.LookbackSpan.thisWay⟩)]))))
                (some thisTurn) ]) ] } }

def ashZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ash Zealot", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ keyword (.named "firstStrike"),
          keyword (.named "haste"),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.procedure .cast ((a
            (Primitives.Predicate.and [isSpell, Primitives.Predicate.castFrom graveyard]))) Primitives.PlayPayment.itsOwnCost none none none))
            (instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (that .player) .noncombatOnly)) ],
      power := stat 2, toughness := stat 2 } }

def galvanicBlastLine : Action :=
  Primitives.Action.replace (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)
    (Primitives.Action.doOnlyIf (Primitives.Action.dealDamage (.lit 4) thatJoin .noncombatOnly)
      (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 3)) none)
theorem okGalvanicBlastLine : Action.check (DRS.ofList []) galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Action :=
  Primitives.Action.dealDamage (.lit 2) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) isAnyTarget)) .noncombatOnly
theorem okFuriousReprisal : Action.check (DRS.ofList []) furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol,
      Primitives.Cost.perform (chooseModes (exactly 1)
        [ sacrifice (a isArtifact), discard (a (Primitives.Predicate.and [Primitives.Predicate.not isLand, Primitives.Predicate.in_ yourHand])) ])])
    (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly))
theorem okBullseyeModalCost : Ability.check (DRS.ofList []) bullseyeModalCost = [] := by decide

/-- Twinshot Sniper -/
def twinshotSniper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twinshot Sniper", cost := some [generic 3, pip .red], types := [.artifact, .creature],
      subtypes := [creatureType "Goblin", creatureType "Archer"],
      text :=
        [ keyword (.named "reach"),
          when (does (actor thisCreature) enter) (instruct it (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)),
          abilityWord "channel"
            (activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .red], Primitives.Cost.perform (discard Primitives.NounPhrase.this)])
              (instruct it (Primitives.Action.dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly))) ],
      power := stat 2, toughness := stat 3 } }

/-- Quakebringer -/
def quakebringerDamage : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (Primitives.Condition.or [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.in_ battlefield),
           Primitives.Condition.and [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]),
                  exists_ (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Giant"),
                                 Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ] ])
    (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly))
theorem okQuakebringerDamage : Ability.check (DRS.ofList []) quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  triggeredIf (does (actor thisCreature) enter)
    (Primitives.Condition.or [ exists_ (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Desert"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
           exists_ (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Desert"), Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]) ])
    (may (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target isCreature) .noncombatOnly)))
theorem okSandStranglerDamage : Ability.check (DRS.ofList []) sandStranglerDamage = [] := by decide

def brazenDwarf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Dwarf", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Shaman"],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
            (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)) ],
      power := stat 1, toughness := stat 3 } }

/-- Lavalanche -/
def lavalanche : Action :=
  Primitives.Action.simultaneously
    [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker .noncombatOnly,
      Primitives.Action.dealDamage (Primitives.Amount.letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okLavalanche : Action.check (DRS.ofList []) lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Action :=
  Primitives.Action.simultaneously
    [ Primitives.Action.dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      Primitives.Action.dealDamage (.lit 4) eachCreatureThatSplitControls .noncombatOnly ]
theorem okFlameWave : Action.check (DRS.ofList []) flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Action :=
  Primitives.Action.simultaneously
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 10) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 10) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okChandraNalaarUltimate : Action.check (DRS.ofList []) chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Action :=
  Primitives.Action.simultaneously
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 6) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 6) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okChandraPyrogeniusUltimate : Action.check (DRS.ofList []) chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Action :=
  Primitives.Action.simultaneously
    [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker .noncombatOnly,
      Primitives.Action.dealDamage (Primitives.Amount.letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okBonfireOfTheDamned : Action.check (DRS.ofList []) bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Action :=
  Primitives.Action.simultaneously
    [ Primitives.Action.dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      Primitives.Action.dealDamage (.lit 1) eachCreatureThatSplitControls .noncombatOnly ]
theorem okChandrasFury : Action.check (DRS.ofList []) chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Action :=
  Primitives.Action.simultaneously
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okAngrathMinotaurPirateBolt : Action.check (DRS.ofList []) angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Action :=
  Primitives.Action.simultaneously
    [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) targetOpponentOrPlaneswalker .noncombatOnly,
      Primitives.Action.dealDamage (Primitives.Amount.letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okWhichOfYouBurnsBrightestBody :
    Action.check (DRS.ofList []) whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Action :=
  Primitives.Action.simultaneously
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) .noncombatOnly) ]
theorem okChandraPyromasterBolt : Action.check (DRS.ofList []) chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Action :=
  Primitives.Action.simultaneously
    [ instruct thisCreature (Primitives.Action.dealDamage (.lit 2) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct thisCreature (Primitives.Action.dealDamage (.lit 2) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) .noncombatOnly) ]
theorem okRavagerOfTheFellsBolt : Action.check (DRS.ofList []) ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Action :=
  Primitives.Action.simultaneously
    [ instruct thisCreature (Primitives.Action.dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) .noncombatOnly) ]
theorem okSoulOfShandalarBolt : Action.check (DRS.ofList []) soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Action :=
  Primitives.Action.simultaneously
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) .noncombatOnly) ]
theorem okSoulOfShandalarGraveyardBolt :
    Action.check (DRS.ofList []) soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly,
      discard (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf splitOverPlaneswalker)])) ]
theorem okBlightning : Action.check (DRS.ofList []) blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) targetOpponentOrPlaneswalker .noncombatOnly,
      discard (counted (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x)) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf splitOverPlaneswalker)])) ]
theorem okRakdossReturn : Action.check (DRS.ofList []) rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 7) targetPlayerOrPlaneswalker .noncombatOnly),
      discard (counted (exactly 7) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf splitOverPlaneswalker)])),
      instruct splitOverPlaneswalker (sacrifice (counted (exactly 7) isPermanent)) ]
theorem okNicolBolasUltimate : Action.check (DRS.ofList []) nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      Primitives.Action.doIf (Primitives.Condition.compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf Primitives.NounPhrase.you))
        (putInto Primitives.NounPhrase.this hand) none ]
theorem okPulseOfTheForge : Action.check (DRS.ofList []) pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Action :=
  Primitives.Action.sequentially
    [ instruct thisArtifact (Primitives.Action.dealDamage (countOf (and [isCreature, youControl])) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct thisArtifact (Primitives.Action.dealDamage (countOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) Primitives.NounPhrase.you .noncombatOnly) ]
theorem okGoblinLyreLoseFlip : Action.check (DRS.ofList []) goblinLyreLoseFlip = [] := by decide
/-- Quenchable Fire -/
def quenchableFire : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      delay (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.next none))
        (instruct splitOverPlaneswalker (doUnless (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) thatJoin .noncombatOnly)) (Primitives.Cost.mana [pip .blue]))) ]
theorem okQuenchableFire : Action.check (DRS.ofList []) quenchableFire = [] := by decide
/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  abilityWord "landfall"
    (Primitives.Ability.spell none
      (Primitives.Action.replace
        (Primitives.Action.simultaneously
          [ Primitives.Action.dealDamage (.lit 1) targetPlayerOrPlaneswalker .noncombatOnly,
            Primitives.Action.dealDamage (.lit 1) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) .noncombatOnly ])
        (Primitives.Action.simultaneously
          [ Primitives.Action.dealDamage (.lit 3) thatJoin .noncombatOnly,
            Primitives.Action.dealDamage (.lit 3) (that (.type .creature)) .noncombatOnly ])))
theorem okSearingBlaze : Ability.check (DRS.ofList []) searingBlaze = [] := by decide

/-- Heart of Bogardan -/
def heartOfBogardan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Bogardan", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.mana [generic 2]),
          when heartOfBogardanHeader
            (Primitives.Action.sequentially
              [ Primitives.Action.simultaneously
                  [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker .noncombatOnly),
                    instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (Primitives.Amount.letter .x) eachCreatureThatSplitControls .noncombatOnly) ],
                Primitives.Action.define .x
                  (Primitives.Amount.arith .minus (times (.lit 2) (countersOn (.named "ageCounter") thisEnchantment)) (.lit 2)) ]) ] } }

/-- Burn at the Stake -/
def burnAtTheStake : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn at the Stake", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost
            (Primitives.Cost.perform (tap (counted anyNumber (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, isUntapped]))))
            false .once),
          Primitives.Ability.spell none (Primitives.Action.dealDamage (times (.lit 3) Primitives.Amount.groupSize) (target isAnyTarget) .noncombatOnly) ] } }

/-- Explosive Singularity -/
def explosiveSingularity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explosive Singularity", cost := some [generic 8, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost
            (Primitives.Cost.perform (tap (counted anyNumber (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, isUntapped]))))
            true .once),
          Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (times (.lit 1) Primitives.Amount.groupSize) none)),
          Primitives.Ability.spell none (Primitives.Action.dealDamage (.lit 10) (target isAnyTarget) .noncombatOnly) ] } }

/-- Tyrant of Valakut -/
def tyrantOfValakut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tyrant of Valakut", cost := some [generic 5, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keywordCosting (.named "surge") (Primitives.Cost.mana [generic 3, pip .red, pip .red]),
          keyword (.named "flying"),
          triggeredIf (does (actor thisCreature) enter) (costWasPaid (.byKeyword (.named "surge")) none thisCreature)
            (instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly)) ],
      power := stat 5, toughness := stat 4 } }

/-- Fall of the Titans -/
def fallOfTheTitansCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fall of the Titans", cost := some [.variable, .variable, pip .red], types := [.instant],
      text :=
        [ keywordCosting (.named "surge") (Primitives.Cost.mana [.variable, pip .red]),
          Primitives.Ability.spell none
            (Primitives.Action.dealDamage (Primitives.Amount.letter .x) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) isAnyTarget)) .noncombatOnly) ] } }

/-- Tribal Flames -/
def tribalFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tribal Flames", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ abilityWord "domain"
            (Primitives.Ability.spell none (Primitives.Action.sequentially
              [ Primitives.Action.dealDamage (Primitives.Amount.letter .x) (target isAnyTarget) .noncombatOnly,
                Primitives.Action.define .x (Primitives.Amount.distinctCount (.subtype .land .basicOnly)
                  (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ])) ] } }

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  abilityWord "vivid"
    (when (does (actor thisCreature) enter)
      (Primitives.Action.sequentially
        [ instruct it (Primitives.Action.dealDamage (Primitives.Amount.letter .x) (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller anOpponent])) .noncombatOnly),
          Primitives.Action.define .x (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check (DRS.ofList []) explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (does (actor thisCreature) (dealDamage (to := a Primitives.Predicate.anyPlayer) (kind := .combatOnly)))
    (Primitives.Action.sequentially
      [ instruct thisCreature (Primitives.Action.dealDamage (Primitives.Amount.letter .x) (target isAnyTarget) .noncombatOnly),
        instruct (target Primitives.Predicate.anyPlayer) (Primitives.Action.draw (Primitives.Amount.letter .x)),
        gainLife (Primitives.Amount.letter .x),
        Primitives.Action.define .x (Primitives.Amount.distinctCount .colorPair
          (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.colorCount .eq 2]))) ])
theorem okNivMizzetGuildpactTrigger : Ability.check (DRS.ofList []) nivMizzetGuildpactTrigger = [] := by decide

/-- Aurelia, the Law Above -/
def aureliaTheLawAbove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aurelia, the Law Above", cost := some [generic 3, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"),
          keyword (.named "vigilance"),
          keyword (.named "haste"),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.attackWith (counted (atLeast 3) isCreature) none)) (Primitives.Action.draw (.lit 1)),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.attackWith (counted (atLeast 5) isCreature) none))
            (Primitives.Action.sequentially [instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly), gainLife (.lit 3)]) ],
      power := stat 4, toughness := stat 4 } }

/-- Syr Konrad, the Grim -/
def syrKonradTheGrim : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Syr Konrad, the Grim", cost := some [generic 3, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ triggeredOr (does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) die)
            [ does Primitives.Subject.any (putInto (a (Primitives.Predicate.and [Primitives.Predicate.isCard, isCreature])) graveyard (from_ := some (Primitives.Location.not battlefield))),
              does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))) (leave (from_ := graveyardOf Primitives.NounPhrase.you)) ]
            (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)),
          activated (Primitives.Cost.mana [generic 1, pip .black]) (instruct (each Primitives.Predicate.anyPlayer) (mill (.lit 1) (each Primitives.Predicate.anyPlayer))) ],
      power := stat 5, toughness := stat 4 } }

/-- Destructive Revelry -/
def destructiveRevelry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Destructive Revelry", cost := some [pip .red, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ destroy (target (Primitives.Predicate.or [isArtifact, isEnchantment])),
              Primitives.Action.dealDamage (.lit 2) (controllerOf (that .permanent)) .noncombatOnly ]) ] } }

/-- Fuming Effigy -/
def fumingEffigy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fuming Effigy", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ whenever (does (actor (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))) (leave (from_ := graveyardOf Primitives.NounPhrase.you)))
            (instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)) ],
      power := stat 4, toughness := stat 3 } }

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Action :=
  Primitives.Action.sequentially
    [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly),
      exile (Primitives.NounPhrase.slice .top (.lit 1) (libraryOf (Primitives.NounPhrase.eachOf (those .player)))) ]
theorem okEachOfThoseOpponentsTopCard : Action.check (DRS.ofList []) eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Inquisitor's Flail -/
def inquisitorsFlail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inquisitor's Flail", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (Primitives.NounPhrase.attachHost .equipped (.type .creature))) Primitives.DamageScope.everywhere
            (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .combatOnly
            (Primitives.DamageAgent.dealtBy (a ((and [isCreature, otherThan (Primitives.NounPhrase.attachHost .equipped (.type .creature))]))))
            (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .equipped (.type .creature))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

/-- Oath of Kaya -/
def oathOfKaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oath of Kaya", cost := some [generic 1, pip .white, pip .black],
      supertypes := [.legendary], types := [.enchantment],
      text :=
        [ when (does (actor «this») enter)
            (Primitives.Action.sequentially [instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly), gainLife (.lit 3)]),
          whenever
            (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.attackWith (counted (atLeast 1) isCreature) (some (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))))
            (Primitives.Action.sequentially [instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (that .player) .noncombatOnly), gainLife (.lit 2)]) ] } }

/-- Frostwielder -/
def frostwielder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frostwielder", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.move (a (Primitives.Predicate.and [isCreature,
              Primitives.Predicate.did (⟨(Primitives.Subject.actor thisCreature), (Primitives.Action.dealDamage Primitives.Amount.anyAmount                 (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any), (Primitives.LookbackSpan.duration soFarThisTurn none)⟩)])) (battlefield) (some graveyard) .departure none [])) (exile it) none .repeatedly none),
          activated Primitives.Cost.tapSymbol (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly)) ],
      power := stat 1, toughness := stat 2 } }

/-- Crackling Doom -/
def cracklingDoom : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.dealDamage (.lit 2) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly,
      instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (sacrifice
        (a (Primitives.Predicate.and [isCreature,
                  Primitives.Predicate.superlative .max (.stat .power)
                    (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (that .player)])]))) ]
theorem okCracklingDoom : Action.check (DRS.ofList []) cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
    (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.and [Primitives.Predicate.or [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.hasType .planeswalker],
                   happenedTo (Primitives.Subject.actor thisCreature) (Primitives.Action.dealDamage Primitives.Amount.anyAmount                      (relative (.join .player .object)) .any) thisGame])) .noncombatOnly))
theorem okTheFallen : Ability.check (DRS.ofList []) theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (does (actor (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.compare [.stat .power] .atMost (.lit 1)]))) attack)
    (instruct thisEnchantment (Primitives.Action.dealDamage (.lit 1) (the (Primitives.Predicate.and [Primitives.Predicate.or [Primitives.Predicate.anyPlayer, Primitives.Predicate.hasType .planeswalker],
                  Primitives.Predicate.inCombat .attackedBy (some (that (.type .creature)))])) .noncombatOnly))
theorem okCavalcadeOfCalamity : Ability.check (DRS.ofList []) cavalcadeOfCalamity = [] := by decide

/-- Blind Fury -/
def blindFury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blind Fury", cost := some [generic 2, pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish (Primitives.StaticSpec.abilityLoss (allOf isCreature) [Primitives.AbilityLost.written (keyword (.named "trample"))])
                (some untilEndOfTurn),
              Primitives.Action.establish
                (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (a isCreature)) (Primitives.DamageScope.toRecipient (a isCreature))
                  (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly)
                (some thisTurn) ]) ] } }

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Action :=
  Primitives.Action.getEmblem
    [ at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
        (instruct (Primitives.NounPhrase.asMarker .emblem Primitives.NounPhrase.this) (Primitives.Action.dealDamage (.lit 1) Primitives.NounPhrase.you .noncombatOnly)) ]
theorem okChandraAwakenedInfernoEmblem :
    Action.check (DRS.ofList []) chandraAwakenedInfernoEmblem = [] := by decide

/-- Keeper of the Flame -/
def keeperOfTheFlame : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Flame", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ chooseWhile
                  (target (Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]))
                  (whileDoing (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .activate thisAbility Primitives.PlayPayment.itsOwnCost none none none))),
                instruct thisCreature (Primitives.Action.dealDamage (.lit 2) (that .player) .noncombatOnly) ]) ],
      power := stat 1, toughness := stat 2 } }

def diabolicEdict : Action := instruct (target Primitives.Predicate.anyPlayer) (sacrifice (aTheirChoice isCreature))
theorem okDiabolicEdict : Action.check (DRS.ofList []) diabolicEdict = [] := by decide
def innocentBlood : Action := instruct (each Primitives.Predicate.anyPlayer) (sacrifice (aTheirChoice isCreature))
theorem okInnocentBlood : Action.check (DRS.ofList []) innocentBlood = [] := by decide
def cryOfContrition : Action := discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (target Primitives.Predicate.anyPlayer))]))
theorem okCryOfContrition : Action.check (DRS.ofList []) cryOfContrition = [] := by decide
def cyclingCost : Action := discard Primitives.NounPhrase.this
theorem okCyclingCost : Action.check (DRS.ofList []) cyclingCost = [] := by decide
def raiseTheAlarm : Action :=
  createToken (creatureToken 1 1 [.white] [creatureType "Soldier"]) (count := (.lit 2))
theorem okRaiseTheAlarm : Action.check (DRS.ofList []) raiseTheAlarm = [] := by decide
def actOfTreason : Action := gainControl (target isCreature) (some untilEndOfTurn) (controller :=
    Primitives.NounPhrase.you)
theorem okActOfTreason : Action.check (DRS.ofList []) actOfTreason = [] := by decide
def wordsOfWorship : Action :=
  replaceNextEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.draw Primitives.Amount.anyAmount)) (gainLife (.lit 5)) (some thisTurn)
theorem okWordsOfWorship : Action.check (DRS.ofList []) wordsOfWorship = [] := by decide
def theLastRoninII : Action :=
  Primitives.Action.triggerReflexively (mill (.lit 4) Primitives.NounPhrase.you)
    (putInto (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) hand)
theorem okTheLastRoninII : Action.check (DRS.ofList []) theLastRoninII = [] := by decide

def moonlitWakeCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Wake", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [moonlitWake] } }

def laquatussDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Laquatus's Disdain", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ counter (target (Primitives.Predicate.and [isSpell, Primitives.Predicate.castFrom graveyard])), Primitives.Action.draw (.lit 1) ]) ] } }

/-- Stifle -/
def stifle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stifle", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (counter (target (Primitives.Predicate.or [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityHead .anyTriggered]))) ] } }

/-- Disallow -/
def disallow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disallow", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (counter
              (target (Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityHead .anyTriggered]))) ] } }

/-- Fry -/
def fry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fry", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (cant Primitives.Subject.any (counter thisSpell)),
          Primitives.Ability.spell none
            (Primitives.Action.dealDamage (.lit 5) (target (Primitives.Predicate.and [Primitives.Predicate.or [isCreature, Primitives.Predicate.hasType .planeswalker], Primitives.Predicate.or [Primitives.Predicate.colorIs .white, Primitives.Predicate.colorIs .blue]])) .noncombatOnly) ] } }

/-- Termination Facilitator -/
def terminationFacilitator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Termination Facilitator", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ activatedOnlyDuring Primitives.Cost.tapSymbol
            (Primitives.Action.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "bountyCounter"))
              (target (Primitives.Predicate.or [isCreature, Primitives.Predicate.hasType .planeswalker])) .many true)
            asSorcery,
          whenever
            (does Primitives.Subject.any (dealDamage (to := a (Primitives.Predicate.and [Primitives.Predicate.or [isCreature, Primitives.Predicate.hasType .planeswalker], Primitives.Predicate.hasPossessor .controller anOpponent,
                        Primitives.Predicate.hasCounters (some (.named "bountyCounter"))]))))
            (destroy it) ],
      power := stat 1, toughness := stat 3 } }

end Semantics.Cards
