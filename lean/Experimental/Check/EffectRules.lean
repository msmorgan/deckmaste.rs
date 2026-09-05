import Experimental.Check.Effect

/-!
# Experimental.Check.EffectRules

The rules of the effect layer: for each `Instruction`, `StaticSpec`, `Cost`, token, and
ability constructor, the obligations the Idris put in its type, over the stack the profile
threads to it.
-/

namespace Mtg

def zoneIsCheck (z : Option Zone) (want : Zone) : List Refusal := refuse (zoneIsB z want) (.zoneIs want)

def CounterKindSource.check (bs : Bindings) : CounterKindSource → List Refusal
  | .printed c => c.check
  | .chosen menu => refuse (!menu.isEmpty) .nonEmpty ++ menu.flatMap CounterKind.check
  | .distinctChosen menu => refuse (!menu.isEmpty) .nonEmpty ++ menu.flatMap CounterKind.check
  | .bound =>
    let n := countChoice (.quality .counterKind) bs
    refuse (n == 1) (.choiceRef .theChoice (.quality .counterKind) n)
  | .those =>
    let n := countOutcomes .countersPut bs
    refuse (n == 1) (.outcomeInScope .countersPut n)
  | .own => []
  | .sameAs src => Noun.check (some .object) bs src

def OptCounterKindSource.check (bs : Bindings) : Option CounterKindSource → List Refusal
  | none => []
  | some s => s.check bs

def SpendPurpose.check (bs : Bindings) : SpendPurpose → List Refusal
  | .toCast p => Predicate.check .object bs p
  | .toActivate src => OptPredicate.check .object bs src
  | .toPay c => refuse c.nameable .costNameable

def AsThough.check (bs : Bindings) : AsThough → List Refusal
  | .of p => Predicate.check .object bs p
  | .mana _ _ purpose =>
    match purpose with
    | none => []
    | some sp => sp.check bs
  | .greater _ amt => Amount.check bs amt ++ refuse (Amount.delta bs amt).isEmpty .bindingless

def Exchanged.check (bs : Bindings) : Exchanged → List Refusal
  | .lifeTotals parties => Noun.check (some .player) bs parties ++ refuse (twoPartiesOk parties) .twoParties
  | .controlOf a b =>
    let bs' := nomIntro bs a
    Noun.check (some .object) bs a ++ Noun.check (some .object) bs' b ++
      refuse (controlExchangeZone (Noun.zone bs a)) .controlExchangeZone ++
      refuse (controlExchangeZone (Noun.zone bs' b)) .controlExchangeZone
  | .cardsAcross a b =>
    let bs' := nomIntro bs a
    Noun.check (some .object) bs a ++ Noun.check (some .object) bs' b ++
      refuse (cardSwapZonesOk (Noun.zone bs a) (Noun.zone bs' b)) .cardSwapZones
  | .zones a b =>
    ZoneExpr.check bs a ++ ZoneExpr.check (ZoneExpr.delta bs a ++ bs) b ++
      refuse (zoneSwapOk a.sort b.sort) .zoneSwap

def CostShift.check (bs : Bindings) : CostShift → List Refusal
  | .less amt floor =>
    Amount.check bs amt ++ (match floor with | none => [] | some f => Amount.check bs f)
  | .more amt => Amount.check bs amt
  | .run r _ _ => refuse (manaRun r) .manaRun

def CountBound.check (bs : Bindings) : CountBound → List Refusal
  | .moreThan k => Amount.check bs k
  | .additional q => refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ Quantity.check bs q

def DeonticPatient.check (bs : Bindings) : DeonticPatient → List Refusal
  | .noPatient => []
  | .defendingPlayer m => Noun.check Option.none bs m ++ refuse (m.attackable bs) .attackable
  | .counterpart m => Noun.check Option.none bs m
  | .targetedBy m =>
    let k := m.kindOr .object
    Noun.check Option.none bs m ++ refuse k.targeter (.targeter k)
  | .counterpartsAt cs =>
    refuse (!cs.isEmpty) .nonEmpty ++ cs.flatMap fun c => Noun.check Option.none bs c.m

def DamageScope.check (bs : Bindings) : DamageScope → List Refusal
  | .everywhere => []
  | .toRecipient n => Noun.check none bs n ++ refuse (n.damageRecipient bs) .damageRecipient

def DamageAgent.check (bs : Bindings) : DamageAgent → List Refusal
  | .unattributed => []
  | .dealtBy n => Noun.check (some .object) bs n

def Unpreventable.check (bs : Bindings) : Unpreventable → List Refusal
  | .described src scope => src.check bs ++ scope.check (src.intro bs)
  | .thatDamage => refuse (damageDealtInScope bs) .damageDealtInScope

def PreventCut.check (bs : Bindings) : PreventCut → List Refusal
  | .some amt | .shield amt | .allBut amt => Amount.check bs amt
  | _ => []

def DamageScale.check (bs : Bindings) : DamageScale → List Refusal
  | .shifted _ amt => Amount.check bs amt
  | _ => []

def DividedVerb.check (bs : Bindings) : DividedVerb → List Refusal
  | .damage src => Noun.check (some .object) bs src
  | .counters kind => kind.check

def ProducedMana.check (bs : Bindings) : ProducedMana → List Refusal
  | .runs rs => refuse (producedRunsWritten rs) .producedRuns
  | .anyColor _ => []
  | .ofChosenColor _ =>
    let n := countChoice (.quality .color) bs
    refuse (n == 1) (.choiceRef .theChoice (.quality .color) n)
  | .asPrintedCost n => Noun.check (some .object) bs n ++ refuse n.plur.isOne .singular
  | .producedByEvent n =>
    let m := countOutcomes .manaProduced bs
    Noun.check (some .object) bs n ++ refuse n.plur.isOne .singular ++
      refuse (m == 1) (.outcomeInScope .manaProduced m)
  | .couldProduce n => Noun.check (some .object) bs n
  | .amongColorsOf n => Noun.check (some .object) bs n
  | .amongWritten cs => refuse (colorCountOk cs.length) .colorCountOk
  | .lastNoted n => Noun.check (some .object) bs n ++ refuse n.plur.isOne .singular

def Repetition.check (bs : Bindings) : Repetition → List Refusal
  | .moreTimes n => Amount.check bs n
  | .untilCond c => Condition.check bs c
  | _ => []

def ManaHeld.check (bs : Bindings) : ManaHeld → List Refusal
  | .thisMana =>
    let n := countOutcomes .manaAdded bs
    refuse (n == 1) (.outcomeInScope .manaAdded n)
  | .unspent _ => []

def DieSides.check (bs : Bindings) : DieSides → List Refusal
  | .sides n => refuse (n != 0) .nonZeroQ
  | .thoseDice =>
    let n := countOutcomes .diceRolled bs
    refuse (n == 1) (.outcomeInScope .diceRolled n)

/-- Idris `ChoiceDomain : ChoiceSort → Type`: the domain's sort must be the announced one. -/
def ChoiceDomain.sort : ChoiceDomain → ChoiceSort
  | .nameOfCard _ => .quality .cardName
  | .colorOtherThan _ => .quality .color
  | .typeOtherThan _ => .quality (.subtype .creature)
  | .basicTypesOnly | .nonbasicTypesOnly => .quality (.subtype .land)
  | .numberAbove _ | .numberBetween _ _ => .quality .number
  | .opponentsOnly => .player

def sortedDomainCheck (bs : Bindings) (q : ChoiceSort) (dom : Option ChoiceDomain) : List Refusal :=
  OptChoiceDomain.check bs dom ++
    (match dom with
     | none => []
     | some d => refuse (d.sort == q) .kindAxisSort)

mutual
  def Instruction.check (bs : Bindings) : Instruction → List Refusal
    | .dealDamage src amt to =>
      let bs' := nomIntro bs src
      let bs'' := Amount.intro bs' amt
      Noun.check (some .object) bs src ++ Amount.check bs' amt ++ Noun.check none bs'' to ++
        refuse to.perMemberOk .perMember ++ refuse (to.damageRecipient bs'') .damageRecipient
    | .fights a b =>
      let bs' := nomIntro bs a
      Noun.check (some .object) bs a ++ zoneIsCheck (Noun.zone bs a) .battlefield ++
        refuse (featureNounOk bs .attacking .agent a) (.featureNounOk .attacking) ++
        refuse a.plur.isOne .singular ++ Noun.check (some .object) bs' b ++
        zoneIsCheck (Noun.zone bs' b) .battlefield ++
        refuse (featureNounOk bs' .attacking .agent b) (.featureNounOk .attacking) ++
        refuse b.plur.isOne .singular
    | .setStatus v n =>
      let k := n.kindOr .object
      Noun.check none bs n ++ refuse (k == .object) .statusHolder ++
        zoneIsCheck (Noun.zone bs n) .battlefield ++ refuse v.markable .statusMarkable
    | .turnOver what => Noun.check (some .object) bs what ++ zoneIsCheck (Noun.zone bs what) .battlefield
    | .removeFromCombat n => Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield
    | .attachTo what host =>
      let kh := host.kindOr .object
      Noun.check (some .object) bs what ++ zoneIsCheck (Noun.zone bs what) .battlefield ++
        Noun.check none (nomIntro bs what) host ++
        refuse (Kind.lte kh (.join .object .player)) (.kindLte kh (.join .object .player))
    | .unattach what => Noun.check (some .object) bs what ++ zoneIsCheck (Noun.zone bs what) .battlefield
    | .becomesBlocking n what =>
      let bs' := nomIntro bs n
      Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield ++
        refuse (featureNounOk bs .blocking .agent n) (.featureNounOk .blocking) ++
        Noun.check (some .object) bs' what ++ zoneIsCheck (Noun.zone bs' what) .battlefield ++
        refuse (featureNounOk bs' .blocking .patient what) (.featureNounOk .blocking)
    | .stopsBlocking n what =>
      let bs' := nomIntro bs n
      Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield ++
        refuse (featureNounOk bs .blocking .agent n) (.featureNounOk .blocking) ++
        Noun.check (some .object) bs' what ++ zoneIsCheck (Noun.zone bs' what) .battlefield ++
        refuse (featureNounOk bs' .blocking .patient what) (.featureNounOk .blocking)
    | .becomesAttacking n whom =>
      Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield ++
        refuse (featureNounOk bs .attacking .agent n) (.featureNounOk .attacking) ++
        AttackDefender.check (nomIntro bs n) whom
    | .regenerate n => Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield
    | .cantBe e deed what =>
      let bs' := e.riderIntro bs
      let k := what.kindOr .object
      Instruction.check bs e ++ Noun.check none bs' what ++ refuse (knownAct deed) (.knownAct deed) ++
        refuse (deedRidesOk deed) .deedRides ++
        refuse (deedFits [deed] .patient k what.isAbility (Noun.headTys bs' what) (Noun.zone bs' what))
          .deedFits
    | .gainsDesignation n d w span =>
      let k := n.kindOr .object
      Noun.check none bs n ++ OptDuration.check (nomIntro bs n) span ++
        refuse (d.scope == .heldBy k) (.designationScope d) ++
        refuse (designationHolderOk d (Noun.zone bs n)) (.designationHolder d k) ++
        refuse (givingWarrantOk d w) (.designationChecked d)
    | .unlock door => Door.check bs door ++ refuse door.namesHost .doorNamesHost
    | .gameBecomes d =>
      refuse (d.scope == .heldByGame) (.designationScope d) ++ refuse d.checked (.designationChecked d)
    | .concludes _ who => Noun.check (some .player) bs who
    | .gameDrawn | .restartsGame | .choicesRevealed _ => []
    | .separateIntoPiles who grp piles faces =>
      let bs' := nomIntro bs who
      Noun.check (some .player) bs who ++ Noun.check (some .object) bs' grp ++
        refuse (facesFit faces piles) .facesFit ++ refuse (grp.plur == .many) .plural
    | .choose first by_ n _ =>
      OptNoun.check (some .player) bs first ++ OptNoun.check (some .player) bs by_ ++
        Noun.check none (agentCtx bs by_) n ++ refuse (choiceOrderOk first by_) .choiceOrder ++
        refuse (choiceClauseOk by_ n) .choiceClause
    | .vote first voters _ ballot =>
      let bs' := optAgentIntro bs first
      OptNoun.check (some .player) bs first ++ Noun.check (some .player) bs' voters ++
        Ballot.check (nomIntro bs' voters) ballot ++
        refuse (choiceOrderOk first (some voters)) .choiceOrder
    | .move what to riders =>
      let bs' := nomIntro bs what
      Noun.check none bs what ++ ZoneExpr.check bs' to ++ TokenRider.checkAll bs' riders ++
        refuse what.movable .movable ++ refuse to.destOk .destOk ++
        refuse (orderOk what.plur to) .arrangementOk ++
        refuse (destTypeOk (Noun.ty bs what) to.sort) .placeable ++
        refuse (ridersFitZone riders to.sort) .ridersFit
    | .counterSpell what => Noun.check none bs what ++ refuse (what.counterable bs) .stackActOn
    | .copy src agent what times exc =>
      let bs' := nomIntro bs agent
      let bs'' := nomIntro bs' what
      let k := what.kindOr .object
      Noun.check (some .player) bs agent ++ Noun.check none bs' what ++ Amount.check bs'' times ++
        CopyExcept.checkAll (Amount.intro bs'' times) exc ++ refuse k.phrasal (.phrasal k) ++
        refuse (copySourceOk bs' src what) .copySourceOk
    | .chooseNewTargets what => Noun.check none bs what ++ refuse (what.copiable bs) .stackActOn
    | .copyTargets cp whom =>
      let kt := whom.kindOr .object
      Noun.check none bs cp ++ Noun.check none (nomIntro bs cp) whom ++
        refuse (cp.copiable bs) .stackActOn ++ refuse kt.targetable (.targetable kt)
    | .changeLife who d => Noun.check (some .player) bs who ++ Amount.check (nomIntro bs who) d.amount
    | .exchange what => what.check bs
    | .addMana who amt prod riders =>
      let bs' := nomIntro bs who
      let bs'' := Amount.intro bs' amt
      Noun.check (some .player) bs who ++ Amount.check bs' amt ++ prod.check bs'' ++
        ManaRider.checkAll bs'' riders
    | .draw who amt => Noun.check (some .player) bs who ++ Amount.check (nomIntro bs who) amt
    | .expose _ who what => Noun.check (some .player) bs who ++ Exposed.check (nomIntro bs who) what
    | .search who sc q p =>
      let bs' := nomIntro bs who
      Noun.check (some .player) bs who ++ sc.check bs' ++ Quantity.check bs' q ++
        Predicate.check .object bs' p ++ refuse q.nonZero .nonZeroQ ++
        refuse q.wellFormed .wellFormedQ ++ refuse p.seedZone.isNone .zoneCoherent
    | .shuffle whose => Noun.check (some .player) bs whose
    | .flipCoins who count => Noun.check (some .player) bs who ++ count.check (nomIntro bs who)
    | .rollDice who count sides =>
      let bs' := nomIntro bs who
      Noun.check (some .player) bs who ++ Amount.check bs' count ++
        sides.check (Amount.intro bs' count)
    | .resultsTable rows =>
      let n := countOutcomes .rollResult bs
      refuse (!rows.isEmpty) .nonEmpty ++ refuse (n == 1) (.outcomeInScope .rollResult n) ++
        Instruction.checkRows bs rows
    | .ignoreOutcomes which => which.check bs ++ refuse (which.ignorableFor bs) .ignorableFor
    | .shiftResult _ amt =>
      let n := countOutcomes .rollResult bs
      Amount.check bs amt ++ refuse (n == 1) (.outcomeInScope .rollResult n)
    | .rollPlanarDie who count => Noun.check (some .player) bs who ++ Amount.check (nomIntro bs who) count
    | .chaosEnsues what => OptNoun.check (some .object) bs what
    | .storeResults on =>
      let n := countOutcomes .rollResult bs
      Noun.check (some .object) bs on ++ refuse on.plur.isOne .singular ++
        refuse (n == 1) (.outcomeInScope .rollResult n)
    | .rerollStored who q whose =>
      let bs' := nomIntro bs who
      Noun.check (some .player) bs who ++ Quantity.check bs' q ++ Noun.check (some .object) bs' whose ++
        refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse whose.plur.isOne .singular
    | .continuously se span =>
      StaticSpec.check bs se ++ OptDuration.check (StaticSpec.intro bs se) span ++
        refuse (spanOk span) .spanOk ++ refuse se.clauseOk .clauseStatic
    | .create agent count spec riders =>
      let bs' := nomIntro bs agent
      let bs'' := Amount.intro bs' count
      Noun.check (some .player) bs agent ++ Amount.check bs' count ++ TokenSpec.check bs'' spec ++
        TokenRider.checkAll bs'' riders
    | .getsEmblem who abl =>
      Noun.check (some .player) bs who ++ refuse (emblemAbilitiesOk abl) .emblemAbilities ++
        AbilityAt.checkAll [] abl
    | .putCounters amt kind on =>
      let bs' := Amount.intro bs amt
      let bs'' := kind.intro bs'
      let k := on.kindOr .object
      Amount.check bs amt ++ kind.check bs' ++ Noun.check none bs'' on ++
        refuse on.perMemberOk .perMember ++ refuse (kindAmountOk amt kind) .kindAmountOk ++
        refuse (kind.scope k) .counterSourceScope
    | .distribute v amt among =>
      let bs' := v.intro bs
      let bs'' := Amount.intro bs' amt
      v.check bs ++ Amount.check bs' amt ++ Noun.check none bs'' among ++
        refuse among.groupMention .groupMention ++
        (match v with
         | .damage _ => refuse (among.damageRecipient bs'') .damageRecipient
         | .counters _ => kindCheck .object among.kind?)
    | .removeCounters q kind from_ =>
      let bs' := optQuantIntro bs q
      let k := from_.kindOr .object
      (match q with
       | none => []
       | some q => Quantity.check bs q ++ refuse q.wellFormed .wellFormedQ) ++
        OptCounterKindSource.check bs kind ++ Noun.check none bs' from_ ++
        refuse (optCounterSourceScope kind k) .counterSourceScope ++
        refuse (from_.counterMemoryOk bs') .counterMemory
    | .moveCounters amt kind src dst =>
      let bs' := Amount.intro bs amt
      let bs'' := nomIntro bs' src
      Amount.check bs amt ++ OptCounterKindSource.check bs kind ++ Noun.check (some .object) bs' src ++
        Noun.check (some .object) bs'' dst ++ refuse (optKindAmountOk amt kind) .kindAmountOk ++
        refuse (optCounterSourceScope kind .object) .counterSourceScope ++
        refuse (src.counterMemoryOk bs') .counterMemory ++ refuse dst.moveDestOk .moveDestination ++
        refuse dst.perMemberOk .perMember
    | .doubleCounters on =>
      let k := on.kindOr .object
      Noun.check none bs on ++ refuse (counterHolderKind k) .counterHolderKind ++
        refuse on.perMemberOk .perMember
    | .losesCounters who kind amt =>
      Noun.check (some .player) bs who ++ OptCounterKindSource.check bs kind ++
        (match amt with
         | none => []
         | some a => Amount.check (nomIntro bs who) a) ++
        refuse (optCounterSourceScope kind .player) .counterSourceScope
    | .enact subj v e =>
      OptNoun.check (some .player) bs subj ++ Instruction.check (agentCtx bs subj) e ++
        refuse (knownAct v) (.knownAct v) ++ refuse (enactAgentOk subj v) .enactAgentOk ++
        refuse (enactKeepsOuter bs subj e) .enactKeepsOuter
    | .controllerSacrifices n =>
      Noun.check (some .object) bs n ++ refuse n.plur.isOne .singular ++
        zoneIsCheck (Noun.zone bs n) .battlefield
    | .pay who c _ =>
      Noun.check (some .player) bs who ++ Cost.check (nomIntro bs who) c ++
        refuse c.payable .payable ++ refuse (payAgreesOk who c) .payAgrees
    | .may offer body ifDid ifNot =>
      let bs' := mayCtx bs offer
      Noun.check (some .player) bs offer ++ Instruction.check bs' body ++
        Instruction.checkOpt (body.intro bs') ifDid ++ Instruction.checkOpt bs' ifNot
    | .ifDone body ifDid ifNot =>
      Instruction.check bs body ++ Instruction.checkOpt (body.intro bs) ifDid ++
        Instruction.checkOpt bs ifNot ++ refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure ++
        refuse (ifDoneArmed ifDid ifNot) .ifDoneArmed
    | .onlyIf e c otherwise =>
      let bs' := e.preIntro bs
      Instruction.check bs e ++ Condition.check bs' c ++
        Instruction.checkOpt (Condition.delta bs' c ++ e.otherwiseCtx bs) otherwise
    | .ifThen c e otherwise =>
      Condition.check bs c ++ Instruction.check (c.intro bs) e ++
        Instruction.checkOpt (e.otherwiseCtx (c.intro bs)) otherwise
    | .define l amt => Amount.check bs amt ++ refuse (anyOpenLetter l bs) (.openLetter l)
    | .forEachOf grp body =>
      let k := grp.kindOr .object
      let bs' := elemIntro bs k grp
      Noun.check none bs grp ++ refuse k.phrasal (.phrasal k) ++ Instruction.check bs' body ++
        refuse (grp.plur == .many) .plural ++ refuse (keepsOuter bs' body) .keepsOuter
    | .forEachKindOf ax dom q body =>
      let bs' := kindValueIntro bs q dom
      OptNoun.check (some .object) bs dom ++ refuse (ax.sort == some q) .kindAxisSort ++
        refuse (kindDomainOk ax dom) .kindDomainOk ++ Instruction.check bs' body ++
        refuse (keepsOuter bs' body) .keepsOuter
    | .repeatProcess rep => rep.check bs
    | .repeated n body =>
      let bs' := Amount.intro bs n
      Amount.check bs n ++ Instruction.check bs' body ++ refuse (keepsOuter bs' body) .keepsOuter
    | .sequentially es => refuse (!es.isEmpty) .nonEmpty ++ Instruction.checkSeq bs es
    | .simultaneously es => refuse (!es.isEmpty) .nonEmpty ++ Instruction.checkSim bs es
    | .modal q modes =>
      Quantity.check bs q ++ refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse (atLeastTwo modes.length) .atLeastTwo ++
        refuse (q.modesFit modes.length) .modesFit ++ Instruction.checkModes bs modes
    | .delayed ev alts span body =>
      GameEvent.check bs ev ++ GameEvent.checkAll bs alts ++ OptDuration.check bs span ++
        Instruction.check (delayedCtx bs alts ev) body ++ refuse (spanOk span) .spanOk
    | .insteadOf replaced repl =>
      Instruction.check bs replaced ++ Instruction.check (replaced.replacedCtx bs) repl ++
        refuse (!replaced.isInstead) .notInstead ++ refuse (!repl.isInstead) .notInstead
    | .heldUntil e ev =>
      Instruction.check bs e ++ GameEvent.check (e.annIntro bs) ev ++
        refuse e.heldUntilOk .heldClause
    | .reflexively body trig =>
      Instruction.check bs body ++ Instruction.check (reflexCtx bs body) trig ++
        refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure
    | .thisWay body ev trig =>
      Instruction.check bs body ++ GameEvent.check (body.intro bs) ev ++
        Instruction.check (thisWayCtx bs body ev) trig ++
        refuse body.thisWayOutcomeOk .thisWayOutcome
    | .doesntUntapNext n steps =>
      Noun.check (some .object) bs n ++ Amount.check bs steps ++
        zoneIsCheck (Noun.zone bs n) .battlefield
    | .skipsNext who _ count => Noun.check (some .player) bs who ++ Amount.check bs count
    | .extraTurn who count => Noun.check (some .player) bs who ++ Amount.check bs count
    | .additionalPart who part anchor count followedBy =>
      OptNoun.check (some .player) bs who ++ Amount.check bs count ++
        refuse part.proper .windowOk ++ refuse (anchor.elim true TurnPart.proper) .windowOk ++
        refuse (followedBy.elim true TurnPart.proper) .windowOk
  termination_by structural e => e

  def Instruction.checkOpt (bs : Bindings) : Option Instruction → List Refusal
    | none => []
    | some e => Instruction.check bs e
  termination_by structural e => e

  def Instruction.checkSeq (bs : Bindings) : List Instruction → List Refusal
    | [] => []
    | e :: es => Instruction.check bs e ++ Instruction.checkSeq (e.intro bs) es
  termination_by structural es => es

  def Instruction.checkSim (bs : Bindings) : List Instruction → List Refusal
    | [] => []
    | e :: es => Instruction.check bs e ++ Instruction.checkSim (e.annIntro bs) es
  termination_by structural es => es

  def Instruction.checkModes (bs : Bindings) : List (Option Cost × Instruction) → List Refusal
    | [] => []
    | (c, e) :: es =>
      (match c with
       | none => []
       | some c => Cost.check bs c) ++ Instruction.check bs e ++ Instruction.checkModes bs es
  termination_by structural es => es

  def Instruction.checkRows (bs : Bindings) : List RollRow → List Refusal
    | [] => []
    | ⟨results, instruction⟩ :: rs =>
      Quantity.check bs results ++ refuse results.nonZero .nonZeroQ ++
        refuse results.wellFormed .wellFormedQ ++ refuse results.literal .quantLiteral ++
        Instruction.check bs instruction ++ Instruction.checkRows bs rs
  termination_by structural rs => rs

  def Cost.check (bs : Bindings) : Cost → List Refusal
    | .mana c => refuse (manaRun c) .manaRun
    | .scaled c amt => Cost.check bs c ++ Amount.check bs amt ++ refuse amt.forEach .forEachAmount
    | .tapSymbol | .untapSymbol | .loyaltySymbol _ | .itsManaCost => []
    | .action e => Instruction.check bs e ++ refuse e.costActionOk .costAction
    | .compound cs => refuse (!cs.isEmpty) .nonEmpty ++ Cost.checkSeq bs cs
    | .either l r =>
      Cost.check bs l ++ Cost.check bs r ++ refuse (!l.isCompound) .notCompound ++
        refuse (!r.isCompound) .notCompound
  termination_by structural c => c

  def Cost.checkSeq (bs : Bindings) : List Cost → List Refusal
    | [] => []
    | c :: cs => Cost.check bs c ++ Cost.checkSeq (c.intro bs) cs
  termination_by structural cs => cs

  def OptCost.check (bs : Bindings) : Option Cost → List Refusal
    | none => []
    | some c => Cost.check bs c
  termination_by structural c => c

  def Compulsion.check (bs : Bindings) : Compulsion → List Refusal
    | .gatedBy c => Cost.check (gatePayer :: bs) c
    | _ => []
  termination_by structural c => c

  def PlayPayment.check (bs : Bindings) : PlayPayment → List Refusal
    | .payingInstead c => Cost.check bs c ++ refuse c.offBattlefield .altPayment
    | _ => []
  termination_by structural p => p

  def DeonticRider.check (bs : Bindings) : DeonticRider → List Refusal
    | .noRider => []
    | .play from_ _ _ _ payment => OptZoneExpr.check bs from_ ++ PlayPayment.check bs payment
  termination_by structural r => r

  def DamageOp.check (bs : Bindings) : DamageOp → List Refusal
    | .prevent cut also =>
      cut.check bs ++ Instruction.checkOpt (outcomeB .damagePrevented :: cut.intro bs) also
    | .redirect cut to =>
      let bs' := cut.intro bs
      cut.check bs ++ Noun.check none bs' to ++ refuse (to.damageRecipient bs') .damageRecipient ++
        refuse to.plur.isOne .singular
    | .scale sc => sc.check bs
  termination_by structural op => op

  def TokenRider.check (bs : Bindings) : TokenRider → List Refusal
    | .entersAs v => refuse v.markable .statusMarkable
    | .entersAttacking whom => AttackDefender.check bs whom
    | .entersTransformed | .entersMelded _ => []
    | .withCounters amt kind _ =>
      Amount.check bs amt ++ kind.check (Amount.intro bs amt) ++ refuse (kindAmountOk amt kind) .kindAmountOk
    | .under who => Noun.check (some .player) bs who ++ refuse who.ctrlOverrideOk .ctrlOverride
    | .asCopyOf _ src exc =>
      Noun.check (some .object) bs src ++ CopyExcept.checkAll bs exc ++ refuse src.perMemberOk .perMember
  termination_by structural r => r

  def TokenRider.checkAll (bs : Bindings) : List TokenRider → List Refusal
    | [] => []
    | r :: rs => TokenRider.check bs r ++ TokenRider.checkAll bs rs
  termination_by structural rs => rs

  def TokenChars.check (bs : Bindings) : TokenChars → List Refusal
    | ⟨pt, _, _, abilities, _, quals⟩ =>
      (match pt with
       | none => []
       | some (p, tou) => Amount.check bs p ++ Amount.check (Amount.intro bs p) tou) ++
        AbilityAt.checkAll [] abilities ++ TokenQuality.checkAll bs quals
  termination_by structural t => t

  def TokenQuality.checkAll (bs : Bindings) : List TokenQuality → List Refusal
    | [] => []
    | .withEveryType _ :: qs => TokenQuality.checkAll bs qs
    | .withQuality q :: qs =>
      Predicate.check .object bs q ++ refuse q.qualityReadOk .qualityRead ++ TokenQuality.checkAll bs qs
  termination_by structural qs => qs

  def QualityPayload.check (bs : Bindings) : QualityPayload → List Refusal
    | .bundle t _ => TokenChars.check bs t
    | .chosenQuality q => Predicate.check .object bs q
    | _ => []
  termination_by structural q => q

  def TokenSpec.check (bs : Bindings) : TokenSpec → List Refusal
    | .written t =>
      TokenChars.check bs t ++ refuse t.typed .tokenTyped ++ refuse t.ptOk .tokenPtOk ++
        refuse (subsFitLine t.typeLine.subtypes t.typeLine.types) .subsFitLine ++
        refuse t.abilitiesOk .tokenAbilities ++ refuse t.canonical .tokenCanonical ++
        refuse t.qualsFit .tokenQualsFit
    | .asThose =>
      let n := countTokenSpecs bs
      refuse (n == 1) (.tokenSpecInScope n)
    | .copyOf src exc =>
      Noun.check (some .object) bs src ++ CopyExcept.checkAll bs exc ++ refuse src.perMemberOk .perMember
  termination_by structural s => s

  def CopyExcept.check (bs : Bindings) : CopyExcept → List Refusal
    | .types added => refuse added.nonEmpty .lineNonEmpty
    | .name _ | .thisAbility | .nonlegendary | .color _ => []
    | .chars t _ =>
      TokenChars.check bs t ++ refuse t.copyBundleSays .copyBundle ++
        refuse t.canonical .tokenCanonical ++ refuse t.abilitiesOk .tokenAbilities ++
        refuse t.qualsFit .tokenQualsFit ++ refuse t.additionUnnamed .additionUnnamed
    | .ability ab => AbilityAt.check [] ab ++ refuse ab.grantable .grantable
    | .pt pow tou => Amount.check bs pow ++ Amount.check (Amount.intro bs pow) tou
    | .entersWithCounters amt kind _ => Amount.check bs amt ++ kind.check
  termination_by structural e => e

  def CopyExcept.checkAll (bs : Bindings) : List CopyExcept → List Refusal
    | [] => []
    | e :: es => CopyExcept.check bs e ++ CopyExcept.checkAll bs es
  termination_by structural es => es

  def ManaRider.check (bs : Bindings) : ManaRider → List Refusal
    | .spendOnly ps => refuse (!ps.isEmpty) .nonEmpty ++ ps.flatMap (SpendPurpose.check bs)
    | .spendNotOn ps => refuse (!ps.isEmpty) .nonEmpty ++ ps.flatMap (SpendPurpose.check bs)
    | .onSpent _ _ what says =>
      Noun.check (some .object) bs what ++ Instruction.check (nomIntro bs what) says ++
        zoneIsCheck (Noun.zone bs what) .stack
  termination_by structural r => r

  def ManaRider.checkAll (bs : Bindings) : List ManaRider → List Refusal
    | [] => []
    | r :: rs => ManaRider.check bs r ++ ManaRider.checkAll bs rs
  termination_by structural rs => rs

  def StaticSpec.check (bs : Bindings) : StaticSpec → List Refusal
    | .modify n what d =>
      Noun.check (some .object) bs n ++ Amount.check (selfSubjIntro bs n) d.amount ++
        zoneIsCheck (Noun.zone bs n) .battlefield ++ refuse what.modifyOk .modifyStat
    | .definesPt n _ amt =>
      Noun.check (some .object) bs n ++ Amount.check (selfSubjIntro bs n) amt ++
        refuse n.selfDefinedOk .selfDefinedOk
    | .switchesPt n => Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield
    | .costs n sh =>
      Noun.check (some .object) bs n ++ sh.check bs ++ refuse (n.costSubjectOk bs) .costSubject
    | .altCost n c =>
      Noun.check (some .object) bs n ++ OptCost.check (selfSubjIntro bs n) c ++
        refuse (c.elim true Cost.offBattlefield) .altPayment ++ refuse (n.costSubjectOk bs) .costSubject
    | .addedCost c _ => Cost.check bs c ++ refuse c.offBattlefield .addedPayment
    | .definesLetter l amt => Amount.check bs amt ++ refuse (anyOpenLetter l bs) (.openLetter l)
    | .gains n ab =>
      Noun.check (some .object) bs n ++ AbilityAt.check bs ab ++
        refuse (grantSubjectOk bs ab n) .grantSubject ++ refuse ab.grantable .grantable
    | .gainsAbilitiesOf n cls src except =>
      let bs' := nomIntro bs n
      Noun.check (some .object) bs n ++ Noun.check (some .object) bs' src ++
        OptPredicate.check .object (nomIntro bs' src) except ++ refuse (!cls.isEmpty) .nonEmpty ++
        refuse (distinctClasses cls) .distinct ++ refuse (cls.all AbilityClass.known) .knownKeywordTerm
    | .deontic n c deeds role bound patient asThough rider =>
      let k := n.kindOr .object
      let bs' := nomIntro bs n
      Noun.check none bs n ++ Compulsion.check (selfSubjIntro bs n) c ++
        (match bound with
         | none => []
         | some b => b.check bs') ++
        DeonticPatient.check bs' patient ++
        (match asThough with
         | none => []
         | some a => a.check bs') ++
        DeonticRider.check (patient.intro bs') rider ++
        refuse (!deeds.isEmpty) .nonEmpty ++ refuse (distinctDeeds deeds) .distinct ++
        refuse (knownActs deeds) .deedFits ++
        refuse (deedFits deeds role k n.isAbility (Noun.headTys bs n) (Noun.zone bs n)) .deedFits ++
        refuse (deonticBoundOk bs' deeds bound) .deonticBoundOk ++
        refuse (deonticPatientOk bs n deeds role patient rider) .deonticPatientOk ++
        refuse (asThoughOk c deeds asThough) .asThoughOk ++
        refuse (deonticRiderOk (patient.intro bs') deeds role c patient asThough.isSome rider)
          .deonticRiderOk
    | .keepsUnspentMana who what => Noun.check (some .player) bs who ++ what.check (nomIntro bs who)
    | .skips who _ => Noun.check (some .player) bs who
    | .becomes n op q =>
      Noun.check (some .object) bs n ++ QualityPayload.check bs q ++ refuse (becomesOk bs op n q) .becomesOk
    | .alsoOffBattlefield se => StaticSpec.check bs se ++ refuse se.notExtended .notExtended
    | .doesntRemove se n =>
      StaticSpec.check bs se ++ Noun.check (some .object) (StaticSpec.intro bs se) n ++
        refuse se.notCarvedOut .notCarvedOut
    | .becomesCopy n src exc =>
      let bs' := nomIntro bs n
      Noun.check (some .object) bs n ++ Noun.check (some .object) bs' src ++
        CopyExcept.checkAll (nomIntro bs' src) exc ++ refuse src.perMemberOk .perMember
    | .losesAllAbilities n except =>
      Noun.check (some .object) bs n ++ OptPredicate.check .object (selfSubjIntro bs n) except ++
        zoneIsCheck (Noun.zone bs n) .battlefield
    | .losesAbilities n abl =>
      Noun.check (some .object) bs n ++ zoneIsCheck (Noun.zone bs n) .battlefield ++
        refuse (!abl.isEmpty) .nonEmpty ++ AbilityLost.checkAll bs abl
    | .gainsControl who what =>
      let bs' := nomIntro bs who
      Noun.check (some .player) bs who ++ Noun.check (some .object) bs' what ++
        zoneIsCheck (Noun.zone bs' what) .battlefield
    | .intercepts ev alts window repl _ limit =>
      GameEvent.check bs ev ++ GameEvent.checkAll bs alts ++ OptTriggerWindow.check bs window ++
        Instruction.check (interceptCtx bs alts ev) repl ++
        refuse (untriggeredLimitOk limit) .untriggeredLimit ++
        refuse (interceptOk ev.name) .interceptable ++ refuse (interceptArmsOk alts) .interceptable
    | .damageRule _ src scope op use =>
      let bs' := src.intro bs
      let bs'' := scope.intro bs'
      src.check bs ++ scope.check bs' ++ op.check bs'' ++ refuse (op.useOk use) .damageOpUse
    | .cantPrevent _ what _ => what.check bs
    | .conditionally se c marking =>
      StaticSpec.check bs se ++ Condition.check (StaticSpec.intro bs se) c ++
        refuse (markingOk marking c) .markingOk
    | .onlyDuring p w se =>
      OptNoun.check (some .player) bs w ++ StaticSpec.check bs se ++ refuse (windowOk p w) .windowOk
    | .noLossFromZeroLife who => Noun.check (some .player) bs who
    | .visibility v who what =>
      Noun.check (some .player) bs who ++ VisibleThing.check (nomIntro bs who) what ++
        refuse (visibilityOk v what) .visibilityOk
    | .triggersAdditionally ev q =>
      GameEvent.check bs ev ++ Quantity.check bs q ++ refuse q.nonZero .nonZeroQ ++
        refuse q.wellFormed .wellFormedQ ++ refuse (Quantity.delta bs q).isEmpty .quantLiteral ++
        refuse (triggerCountOk ev.name) .triggerCountOk
    | .entersRider n rider =>
      Noun.check (some .object) bs n ++ TokenRider.check (nomIntro bs n) rider ++
        zoneIsCheck (Noun.zone bs n) .battlefield
    | .entersChoice n q dom _ =>
      Noun.check (some .object) bs n ++ sortedDomainCheck bs q dom ++ zoneIsCheck (Noun.zone bs n) .battlefield
    | .attachChoice n q dom =>
      Noun.check (some .object) bs n ++ sortedDomainCheck bs q dom ++ zoneIsCheck (Noun.zone bs n) .battlefield
    | .andAlso subject parts =>
      OptNoun.check (some .object) bs subject ++ refuse (!parts.isEmpty) .nonEmpty ++
        StaticSpec.checkParts (subjCtx bs subject) parts
  termination_by structural se => se

  def StaticSpec.checkParts (bs : Bindings) : List StaticSpec → List Refusal
    | [] => []
    | se :: rest =>
      StaticSpec.check bs se ++ refuse (!se.isCoord) .notCoord ++
        StaticSpec.checkParts (StaticSpec.intro bs se) rest
  termination_by structural parts => parts

  def AbilityLost.checkAll (bs : Bindings) : List AbilityLost → List Refusal
    | [] => []
    | .written ab :: rest => AbilityAt.check bs ab ++ refuse ab.grantable .grantable ++ AbilityLost.checkAll bs rest
    | .term t :: rest => refuse t.known .knownKeywordTerm ++ AbilityLost.checkAll bs rest
  termination_by structural abl => abl

  def KeywordParam.check (bs : Bindings) : KeywordParam → List Refusal
    | .cost c => Cost.check [] c
    | .quality p =>
      let k := p.kindOr .object
      Predicate.check k bs p ++ refuse k.qualityParam (.phrasal k)
    | .subject p => Predicate.check (p.kindOr .object) [] p
    | .number amt => Amount.check [] amt
    | .qualityCost p cost => Predicate.check .object bs p ++ Cost.check [] cost
    | .numberCost amt cost => Amount.check [] amt ++ Cost.check [] cost
    | .deckCondition dc => dc.check
  termination_by structural p => p

  def AbilityAt.check (bs : Bindings) : AbilityAt → List Refusal
    | .keyword k param body =>
      (match param with
       | none => []
       | some p => KeywordParam.check bs p) ++
        (match body with
         | none => []
         | some b => AbilityAt.check [] b) ++
        refuse (keywordParamFits k param) (.keywordParamFits k) ++
        refuse (keywordBodyFits k body) (.keywordBodyFits k)
    | .activated cost instr window limit guard activator =>
      let bsc := dropLetter .x bs
      Cost.check bsc cost ++ Instruction.check (publicOnly (cost.intro bsc)) instr ++
        refuse cost.tapOnce .costTapOnce ++ refuse cost.paidByYou .costPaidByYou ++
        OptTiming.check bs window ++ refuse (untriggeredLimitOk limit) .untriggeredLimit ++
        OptCondition.check bs guard ++ OptNoun.check (some .player) bs activator
    | .triggered word ev alts while_ joins window limit intervening instr =>
      let hctx := headerCtx bs alts ev
      let jctx := joinedCtx bs joins hctx
      headerEventCheck bs ev ++ alts.flatMap (headerEventCheck bs) ++
        OptConcurrent.check hctx while_ ++ joins.flatMap (JoinedHeader.check bs) ++
        OptTriggerWindow.check bs window ++ OptCondition.check jctx intervening ++
        Instruction.check (interveningIntro jctx intervening) instr ++
        refuse (chapterDefaultsOk ev alts while_ joins window limit intervening) .chapterDefaults ++
        (let _ := word; [])
    | .static se =>
      StaticSpec.check bs se ++ refuse (!anyTargetedAt (StaticSpec.intro bs se)) .nontarget
    | .spell window instr => OptTiming.check bs window ++ Instruction.check bs instr
    | .mayBeginOnBattlefield => []
    | .alsoForKeywords ab ks =>
      AbilityAt.check bs ab ++ refuse ab.keywordExtendable .keywordExtendable ++
        refuse (keywordListOk ab ks) .keywordListOk
    | .italicHead _ ab => AbilityAt.check bs ab ++ refuse ab.notWordHeaded .notWordHeaded
  termination_by structural ab => ab

  def AbilityAt.checkAll (bs : Bindings) : List AbilityAt → List Refusal
    | [] => []
    | ab :: abl => AbilityAt.check bs ab ++ AbilityAt.checkAll bs abl
  termination_by structural abl => abl
end

/-- A card's text: each line read against the choices the lines before it announced. -/
def AbilitySeq.check (bs : Bindings) : AbilitySeq → List Refusal
  | [] => []
  | ab :: rest => AbilityAt.check bs ab ++ AbilitySeq.check (ab.intro bs) rest

end Mtg
