module Experimental.ProofsE

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Creatures you control have convoke."
public export
badBattlefieldConvoke : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.allOf Macros.creatureYouControl) (KeywordAbility "Convoke" Nothing) {ok})
badBattlefieldConvoke Oh impossible

public export
badGrantedPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition Oh impossible


public export
badStarlessDefinedPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature])
       [Static (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))]
       (Just (2, 2)) {bx = ok})
badStarlessDefinedPt MkCardBox impossible


public export
badPtDefinitionClause : Unspellable (Effect []) (\ok =>
  Continuously (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))
               Nothing {cl = ok})
badPtDefinitionClause Oh impossible


||| "the creature with the total power among creatures you control"
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (CharAxis Power) Macros.creature {ex = ok})
badSumSelection Oh impossible


||| "the creature"
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Macros.the Macros.creature {ok})
badBareDefinite Oh impossible


||| "Choose the creature with the least toughness among creatures you control."
public export
badChooseDefinite : Unspellable (Effect []) (\ok =>
  Choose (Macros.the (And [Macros.creature,
                         Superlative MinOf (CharAxis Toughness)
                                     Macros.creatureYouControl])) Nothing Openly {ch = ok})
badChooseDefinite BareChoice impossible


||| "the creature with the highest life total among creatures you control"
public export
badLifeTotalSuperlative : Unspellable (Predicate [] Object) (\ok =>
  Superlative MaxOf (PlayerStatAxis LifeTotal) Macros.creature {sc = ok})
badLifeTotalSuperlative Refl impossible


public export
badAnnouncingRemovalAgent : Unspellable (GameEvent []) (\ok =>
  LastCounterRemoved Intervention Macros.thisEnchantment (Just (Macros.target AnyPlayer))
                     {ag = AgentVoiced {bl = ok}})
badAnnouncingRemovalAgent Refl impossible


||| "Each player gets a charge counter."
public export
badGetsChargeCounter : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Charge) You {sc = ok})
badGetsChargeCounter Oh impossible


||| "this instant"
public export
badAscribeInstant : Unspellable (Noun [] Object) (\ok =>
  AsType Instant This Nothing {way = ok})
badAscribeInstant Oh impossible


||| "this sorcery"
public export
badAscribeSorcery : Unspellable (Noun [] Object) (\ok =>
  AsType Sorcery This Nothing {way = ok})
badAscribeSorcery Oh impossible


||| "this kindred"
public export
badAscribeKindred : Unspellable (Noun [] Object) (\ok =>
  AsType Kindred This Nothing {way = ok})
badAscribeKindred Oh impossible


||| "this Aura land"
public export
badAscribeForeignSubtype : Unspellable (Noun [] Object) (\ok =>
  AsType Land This (Just (enchantmentType "Aura")) {way = ok})
badAscribeForeignSubtype Oh impossible


||| "target creature that is the monarch"
public export
badObjectMonarch : Unspellable (Predicate [] Object) (\ok =>
  HasDesignation Monarch {sc = ok})
badObjectMonarch Refl impossible


||| "You become goaded."
public export
badGoadedPlayer : Unspellable (Effect []) (\ok =>
  GainsDesignation You Goaded Instructed Nothing {sc = ok})
badGoadedPlayer Refl impossible


||| "goad target creature card in your graveyard"
public export
badGoadInGraveyard : Unspellable (Effect []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded Instructed Nothing {zn = ok})
badGoadInGraveyard (HolderOnField {ok = OnField}) impossible


||| "Unflip target creature."
public export
badUnflipInstruction : Unspellable (Effect []) (\ok =>
  SetStatus Unflipped (Macros.target Macros.creature) {at = ok})
badUnflipInstruction Oh impossible


||| "Ward"
public export
badBareWardLine : Unspellable Ability (\ok => KeywordAbility "Ward" Nothing {pf = ok})
badBareWardLine Oh impossible


||| "Ward red"
public export
badWardQuality : Unspellable Ability (\ok =>
  KeywordAbility "Ward" (Just (ParamQuality (ColorIs Red))) {pf = ok})
badWardQuality Oh impossible


||| "Flying {2}"
public export
badParamOnNullaryKeyword : Unspellable Ability (\ok =>
  KeywordAbility "Flying" (Just (ParamCost (Mana [Macros.generic 2]))) {pf = ok})
badParamOnNullaryKeyword Oh impossible


||| "Flyign"
public export
badUnknownKeywordLabel : Unspellable Ability (\ok =>
  KeywordAbility "Flyign" Nothing {pf = ok})
badUnknownKeywordLabel Oh impossible


||| "each creature with flyign"
public export
badUnknownKeywordPredicate : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (TheKeyword "Flyign") {kn = ok})
badUnknownKeywordPredicate Oh impossible


||| "Protection from red"
public export
badProtectionOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red)))] Nothing {tx = ok})
badProtectionOnInstant Oh impossible


||| "Protection from player"
public export
badProtectionFromPlayerRestriction : Unspellable Ability (\ok =>
  KeywordAbility "Protection" (Just (ParamSubject AnyPlayer)) {pf = ok})
badProtectionFromPlayerRestriction Oh impossible


||| "Equip {2}"
public export
badEquipOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "Equip" (Just (ParamCost (Mana [Macros.generic 2])))]
       Nothing {tx = ok})
badEquipOnSorcery Oh impossible


||| "each of one or more creatures"
public export
badEachOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.counted (Macros.atLeast 1) Macros.creature) {gm = ok})
badEachOfCountedGroup Oh impossible


||| "one of one or more creatures"
public export
badPartitiveOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  SomeOf (CountedSlice (Macros.exactly 1)) Nothing
         (Macros.counted (Macros.atLeast 1) Macros.creature)
         {gm = ok})
badPartitiveOfCountedGroup Oh impossible


||| "Whenever a creature attacks your opponents, …"
public export
badPluralAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (PlayerGroup YourOpponents) {sg = ok}))
badPluralAttackDefender Refl impossible


||| "Whenever a creature attacks another creature, …"
public export
badCreatureAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a Macros.creature) {at = ok}))
badCreatureAttackDefender Oh impossible


public export
badAgentChooseTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ []
               , Choose TheRest (Just (Macros.a Opponent)) Openly {ch = ok} ])
badAgentChooseTheRest AgentChoice impossible


||| "… if a creature was put this turn, …"
public export
badPlacementLookback : Unspellable (Condition []) (\ok =>
  Happened Placement (Macros.a Macros.creature) ThisTurn Nothing
           {cw = LeftBare {ok = ok}})
badPlacementLookback Oh impossible


||| "Whenever a creature enters during the turn, draw a card."
public export
badHeaderBareTurnWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing [] (Just (DuringWindow Turn Nothing {hw = ok})) Nothing Nothing
            (Macros.draw You (Lit 1)))
badHeaderBareTurnWindow Oh impossible


||| "If one or more creatures would be created under your control, …"
public export
badNonTokenCreationSubject : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) Macros.creature) Nothing Nothing (Just You) {tk = ok})
badNonTokenCreationSubject CountedTokens impossible
badNonTokenCreationSubject OneToken impossible


public export
badSingularCounterBatchSize : Unspellable (StaticEffect []) (\ok =>
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) OneCounter Nothing Nothing) [] Nothing
             (PutCounters (Plus (ThatMuch {ok}) (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) It)
             Repeatedly Nothing)
badSingularCounterBatchSize Refl impossible


public export
badCausedCounterWithAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You) (Just AnEffect) {cz = ok})
badCausedCounterWithAgent Oh impossible


||| "Destroy target creature. Create two of those tokens."
public export
badAnaphoricTokenAfterNonToken : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Create You (Lit 2) (TokenAsThose {ok}) [] ])
badAnaphoricTokenAfterNonToken Refl impossible


||| "Target creature becomes a Coward until end of turn. It's still a land."
public export
badStillOnSubtypeSet : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [creatureType "Coward"] []) [] Nothing) (Just Land)) {ok = ok})
badStillOnSubtypeSet Oh impossible


public export
badStillAnInstant : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) (Just Instant)) {ok = ok})
badStillAnInstant Oh impossible


||| "Target creature gets +1/+1 and gains flying and gains trample."
public export
badNestedCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso (Coord.(::) (AndAlso [ Gets (Macros.target Macros.creature)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains It (KeywordAbility "Flying" Nothing) ])
                      {nc = ok}
                      (Coord.(::) (Gains It (KeywordAbility "Trample" Nothing)) Coord.Nil)))
badNestedCoordination Oh impossible


||| a coordination of no statements
public export
badEmptyCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso [] {ne = ok})
badEmptyCoordination ItIsSucc impossible


||| "This creature gets +1/+1 and that creature has flying."
public export
badThatCreatureIsStaticSubject : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (That (TypeW Creature) {ok = ok}) (KeywordAbility "Flying" Nothing) ]))
badThatCreatureIsStaticSubject Refl impossible


||| "Enchanted creature gets +1/+1 and they have flying."
public export
badCoordinatedHostPlural : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Them {ok = ok}) (KeywordAbility "Flying" Nothing) ]))
badCoordinatedHostPlural Refl impossible


||| "Enchanted land gets +1/+1 and can't block."
public export
badCoordinatedLandHostBlocks : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Macros.deontic It Forbid ["Block"] Agent NoDeonticPatient {dp = ok} ]))
badCoordinatedLandHostBlocks Oh impossible


||| "equipped player"
public export
badEquippedPlayer : Unspellable (Noun [] Player) (\ok =>
  AttachHost Equipped PlayerW {ok})
badEquippedPlayer Oh impossible


||| "fortified creature"
public export
badFortifiedCreature : Unspellable (Noun [] Object) (\ok =>
  AttachHost Fortified (TypeW Creature) {ok})
badFortifiedCreature Oh impossible


||| "Target creature gains flash."
public export
badBattlefieldFlash : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.creature) (KeywordAbility "Flash" Nothing) {ok})
badBattlefieldFlash Oh impossible


||| "Target spell gains indestructible."
public export
badSpellIndestructible : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.spell) (KeywordAbility "Indestructible" Nothing) {ok})
badSpellIndestructible Oh impossible


||| a "Kindred Enchantment — Siege" card
public export
badSiegeWithoutBattle : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [battleType "Siege"] [Kindred, Enchantment])
       [KeywordAbility "Flying" Nothing] Nothing {ln = ok})
badSiegeWithoutBattle MkCardLine impossible


||| a "Kindred — Merfolk" card naming no other card type
public export
badKindredAlone : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Merfolk"] [Kindred])
       [KeywordAbility "Flying" Nothing] Nothing {ln = ok})
badKindredAlone MkCardLine impossible


||| "Enchanted planeswalker can't attack."
public export
badPlaneswalkerAttacks : Unspellable Ability (\ok =>
  Static (Macros.deontic (AttachHost Enchanted (TypeW Planeswalker))
                  Forbid ["Attack"] Agent NoDeonticPatient {dp = ok}))
badPlaneswalkerAttacks Oh impossible


||| "Create a 1/1 white Soldier creature token with 'Draw two cards.'"
public export
badTokenSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                 (MkTypeLine [creatureType "Soldier"] [Creature])
                                 [Spell (Macros.draw You (Lit 1))] Nothing) {ta = ok})
badTokenSpellAbility Oh impossible


||| "Instant and sorcery spells you cast have '{T}: Draw a card.'"
public export
badQuotedGrantOnSpell : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
        (Activated TapSymbol (Macros.draw You (Lit 1)) Nothing Nothing Nothing Nothing) {ok})
badQuotedGrantOnSpell Oh impossible


||| "You get an emblem with 'flying'."
public export
badKeywordEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [KeywordAbility "Flying" Nothing] {ea = ok})
badKeywordEmblem Oh impossible


||| "You get an emblem."
public export
badEmptyEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [] {ea = ok})
badEmptyEmblem Oh impossible


||| "you pay [+1]"
public export
badPayLoyalty : Unspellable (Effect []) (\ok =>
  Pay You (LoyaltySymbol (LoyaltyUp 1)) PaidOnce {pb = ok})
badPayLoyalty Oh impossible


||| "[+1]: Draw a card."
public export
badLoyaltySorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Loyalty Sorcery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) (Macros.draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {tx = ok})
badLoyaltySorcery Oh impossible


||| "Add."
public export
badEmptyProduction : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [] {ok}) [])
badEmptyProduction Oh impossible

||| "Add {R} or ."
public export
badEmptyAlternative : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[OfColor Red], []] {ok}) [])
badEmptyAlternative Oh impossible


||| "Add {C}. Spend this mana only."
public export
badPurposelessSpend : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [] {ne = ok}])
badPurposelessSpend MkSpendPurposes impossible


public export
badEmptyCopyTypeException : Unspellable (Effect []) (\ok =>
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] []) {ne = ok}]) [])
badEmptyCopyTypeException Oh impossible

||| "Copy target creature."
public export
badCopyPermanent : Unspellable (Effect []) (\ok =>
  CopyStack You (Macros.target Macros.creature) (Lit 1) [] {cp = ok})
badCopyPermanent SpellCopied impossible


||| "Choose new targets for target creature."
public export
badRetargetPermanent : Unspellable (Effect []) (\ok =>
  ChooseNewTargets (Macros.target Macros.creature) {cp = ok})
badRetargetPermanent SpellCopied impossible


||| "Copy target instant or sorcery spell. Untap that token."
public export
badStackCopyAsToken : Unspellable (Effect []) (\ok =>
  Sequentially [CopyStack You (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                          (Lit 1) [],
                SetStatus Untapped (That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badStackCopyAsToken (Refl, _) impossible


||| "Create a token that's a copy of target creature. Untap that copy."
public export
badTokenCopyAsCopyMention : Unspellable (Effect []) (\ok =>
  Sequentially [Create You (Lit 1) (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (That CopyW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTokenCopyAsCopyMention (Refl, _) impossible


public export
badOtherwiseReadsLeadingArm : Unspellable (Effect []) (\ok =>
  If (Exists Macros.creatureYouControl)
     (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Just (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsLeadingArm (Refl, _) impossible


public export
badLeadingConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (CompareAmt (PlayerStatOf LifeTotal You) Less
                               (PlayerStatOf LifeTotal Macros.anOpponent))
                   (Macros.gainsLife You (Lit 6))
                   Nothing,
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badLeadingConditionAntecedent Refl impossible

