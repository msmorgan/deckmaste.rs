import Semantics.Check.Abilities

/-!
# Semantics.Check.AbilityRules

The rules of the effect layer: for each `Instruction`, `StaticSpec`, `Cost`, token, and
ability constructor, the obligations the Idris put in its type, over the stack the profile
threads to it.
-/

namespace Semantics

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
  | .sameAs src => NounPhrase.check (some .object) bs src

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
  | .greater _ amt => Amount.check bs amt ++ refuse (Amount.introduced bs amt).isEmpty .bindingless

def Exchanged.check (bs : Bindings) : Exchanged → List Refusal
  | .lifeTotals parties => NounPhrase.check (some .player) bs parties ++ refuse (twoPartiesOk parties) .twoParties
  | .controlOf a b =>
    let bs' := nomIntro bs a
    NounPhrase.check (some .object) bs a ++ NounPhrase.check (some .object) bs' b ++
      refuse (controlExchangeZone (NounPhrase.zone bs a)) .controlExchangeZone ++
      refuse (controlExchangeZone (NounPhrase.zone bs' b)) .controlExchangeZone
  | .cardsAcross a b =>
    let bs' := nomIntro bs a
    NounPhrase.check (some .object) bs a ++ NounPhrase.check (some .object) bs' b ++
      refuse (cardSwapZonesOk (NounPhrase.zone bs a) (NounPhrase.zone bs' b)) .cardSwapZones
  | .zones a b =>
    ZoneExpr.check bs a ++ ZoneExpr.check (ZoneExpr.introduced bs a ++ bs) b ++
      refuse (zoneSwapOk a.sort b.sort) .zoneSwap
  | .values a b =>
    let bs' := Amount.intro bs a
    Amount.check bs a ++ Amount.check bs' b ++ refuse a.settableValue .settableValue ++
      refuse b.settableValue .settableValue ++ refuse (!selfExchanged a b) .selfExchanged
  | .textBoxes a b =>
    let bs' := nomIntro bs a
    NounPhrase.check (some .object) bs a ++ NounPhrase.check (some .object) bs' b ++
      refuse (zoneIsB (NounPhrase.zone bs a) .battlefield) (.zoneIs .battlefield) ++
      refuse (zoneIsB (NounPhrase.zone bs' b) .battlefield) (.zoneIs .battlefield)

def CostShift.check (bs : Bindings) : CostShift → List Refusal
  | .less amt floor =>
    Amount.check bs amt ++ (match floor with | none => [] | some f => Amount.check bs f)
  | .more amt => Amount.check bs amt
  | .run r _ _ => refuse (manaRun r) .manaRun ++ ManaCost.check r

def CountBound.check (bs : Bindings) : CountBound → List Refusal
  | .moreThan k => Amount.check bs k
  | .additional q => refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ Quantity.check bs q

def DeonticPatient.check (bs : Bindings) : DeonticPatient → List Refusal
  | .noPatient => []
  | .defendingPlayer m => NounPhrase.check Option.none bs m ++ refuse (m.attackable bs) .attackable
  | .counterpart m => NounPhrase.check Option.none bs m
  | .targetedBy m =>
    let k := m.kindOr .object
    NounPhrase.check Option.none bs m ++ refuse k.targeter (.targeter k)
  | .counterpartsAt cs =>
    refuse (!cs.isEmpty) .nonEmpty ++ cs.flatMap fun c => NounPhrase.check Option.none bs c.counterpart

def DamageScope.check (bs : Bindings) : DamageScope → List Refusal
  | .everywhere => []
  | .toRecipient n => NounPhrase.check none bs n ++ refuse (n.damageRecipient bs) .damageRecipient

def DamageAgent.check (bs : Bindings) : DamageAgent → List Refusal
  | .unattributed => []
  | .dealtBy n => NounPhrase.check (some .object) bs n

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
  | .damage src => NounPhrase.check (some .object) bs src
  | .counters kind => kind.check

def ProducedMana.check (bs : Bindings) : ProducedMana → List Refusal
  | .runs rs => refuse (producedRunsWritten rs) .producedRuns
  | .anyColor _ => []
  | .ofChosenColor _ =>
    let n := countChoice (.quality .color) bs
    refuse (n == 1) (.choiceRef .theChoice (.quality .color) n)
  | .asPrintedCost n => NounPhrase.check (some .object) bs n ++ refuse n.plur.isOne .singular
  | .producedByEvent n =>
    let m := countOutcomes .manaProduced bs
    NounPhrase.check (some .object) bs n ++ refuse n.plur.isOne .singular ++
      refuse (m == 1) (.outcomeInScope .manaProduced m)
  | .couldProduce n => NounPhrase.check (some .object) bs n
  | .amongColorsOf n => NounPhrase.check (some .object) bs n
  | .amongWritten cs => refuse (colorCountOk cs.length) .colorCountOk
  | .lastNoted n => NounPhrase.check (some .object) bs n ++ refuse n.plur.isOne .singular

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

/-- Written stat slots in order, each read after the ones before it ("X/X"). -/
def statsCheck (bs : Bindings) : List (Option Amount) → List Refusal
  | [] => []
  | none :: rest => statsCheck bs rest
  | some a :: rest => Amount.check bs a ++ statsCheck (Amount.intro bs a) rest

mutual
  def Instruction.check (bs : Bindings) : Instruction → List Refusal
    | .withOperands subjects body =>
      Instruction.checkOperands bs subjects ++ Instruction.check (captureOperands bs subjects) body
    | .dealDamage src amt to =>
      let bs' := selfSubjIntro bs src
      let bs'' := Amount.introduced bs' amt ++ nomIntro bs src
      NounPhrase.check (some .object) bs src ++ Amount.check bs' amt ++ NounPhrase.check none bs'' to ++
        refuse to.perMemberOk .perMember ++ refuse (to.damageRecipient bs'') .damageRecipient
    | .setStatus v n =>
      let k := n.kindOr .object
      NounPhrase.check none bs n ++ refuse (k == .object) .statusHolder ++
        zoneIsCheck (NounPhrase.zone bs n) .battlefield ++ refuse v.markable .statusMarkable
    | .turnOver what => NounPhrase.check (some .object) bs what ++ zoneIsCheck (NounPhrase.zone bs what) .battlefield
    | .combat n update =>
      let bs' := nomIntro bs n
      NounPhrase.check (some .object) bs n ++ zoneIsCheck (NounPhrase.zone bs n) .battlefield ++
        (match update with
         | .participation .outsideCombat => []
         | .participation (.blocked _) =>
           refuse (deedNounOk bs (.core .attack) .agent n) (.deedNounOk (.core .attack))
         | .participation (.attacking whom) =>
           refuse (deedNounOk bs (.core .attack) .agent n) (.deedNounOk (.core .attack)) ++
             AttackDefender.check bs' whom
         | .blocking _ what =>
           refuse (deedNounOk bs (.core .block) .agent n) (.deedNounOk (.core .block)) ++
             NounPhrase.check (some .object) bs' what ++ zoneIsCheck (NounPhrase.zone bs' what) .battlefield ++
             refuse (deedNounOk bs' (.core .block) .patient what) (.deedNounOk (.core .block)))
    | .attachment move what host =>
      NounPhrase.check (some .object) bs what ++ zoneIsCheck (NounPhrase.zone bs what) .battlefield ++
        (match host with
         | none => refuse (move == .unattached) .attachFits
         | some host =>
           let kh := host.kindOr .object
           NounPhrase.check none (nomIntro bs what) host ++
             refuse (Kind.lte kh (.join .object .player)) (.kindLte kh (.join .object .player)))
    | .clearDamage n =>
      NounPhrase.check (some .object) bs n ++ zoneIsCheck (NounPhrase.zone bs n) .battlefield
    | .doAndForbid e deed what =>
      let bs' := e.riderIntro bs
      let k := what.kindOr .object
      Instruction.check bs e ++ NounPhrase.check none bs' what ++ refuse (knownAct deed) (.knownAct deed) ++
        refuse (deedRidesOk deed) .deedRides ++
        refuse (deedFits [deed] .patient k what.isAbility (NounPhrase.headTys bs' what) (NounPhrase.zone bs' what))
          .deedFits
    | .gainDesignation n d w span =>
      let k := n.kindOr .object
      NounPhrase.check none bs n ++ OptDuration.check (nomIntro bs n) span ++
        refuse (d.scope == some (.heldBy k)) (.designationScope d) ++
        refuse (designationHolderOk d (NounPhrase.zone bs n)) (.designationHolder d k) ++
        refuse (conferralOk d w) (.designationChecked d)
    | .unlock door => Door.check bs door ++ refuse door.namesHost .doorNamesHost
    | .setGameDesignation d =>
      refuse d.gameWide (.designationScope d) ++ refuse d.checked (.designationChecked d)
    | .conclude _ who => NounPhrase.check (some .player) bs who
    | .drawGame | .restartGame | .revealChoices _ => []
    | .separateIntoPiles grp piles faces who =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ NounPhrase.check (some .object) bs' grp ++
        refuse (facesFit faces piles) .facesFit ++ refuse (grp.plur == .many) .plural
    | .choose first n _ when by_ =>
      OptNoun.check (some .player) bs first ++ OptNoun.check (some .player) bs by_ ++
        NounPhrase.check none (agentCtx bs by_) n ++ refuse (choiceOrderOk first by_) .choiceOrder ++
        refuse (choiceClauseOk by_ n) .choiceClause ++ OptConcurrent.check bs when
    | .vote first _ ballot voters =>
      let bs' := optAgentIntro bs first
      OptNoun.check (some .player) bs first ++ NounPhrase.check (some .player) bs' voters ++
        Ballot.check (nomIntro bs' voters) ballot ++
        refuse (choiceOrderOk first (some voters)) .choiceOrder
    | .move what to riders =>
      let bs' := nomIntro bs what
      NounPhrase.check none bs what ++
        (match to with
         | some destination =>
           ZoneExpr.check bs' destination ++ TokenRider.checkAll bs' riders ++
             refuse what.movable .movable ++ refuse destination.destOk .destOk ++
             refuse (orderOk what.plur destination) .arrangementOk ++
             refuse (destTypeOk (NounPhrase.ty bs what) destination.sort) .placeable ++
             refuse (ridersFitZone riders destination.sort) .ridersFit
         | none =>
           TokenRider.checkAll bs' riders ++ refuse what.movable .movable ++
             refuse riders.isEmpty .ridersFit)
    | .copy src what times exc agent =>
      let bs' := nomIntro bs agent
      let bs'' := nomIntro bs' what
      let k := what.kindOr .object
      NounPhrase.check (some .player) bs agent ++ NounPhrase.check none bs' what ++ Amount.check bs'' times ++
        CopyExcept.checkAll (Amount.intro bs'' times) exc ++ refuse k.phrasal (.phrasal k) ++
        refuse (copySourceOk bs' src what) .copySourceOk
    | .chooseNewTargets what => NounPhrase.check none bs what ++ refuse (what.copiable bs) .stackActOn
    | .copyTargets cp whom =>
      let kt := whom.kindOr .object
      NounPhrase.check none bs cp ++ NounPhrase.check none (nomIntro bs cp) whom ++
        refuse (cp.copiable bs) .stackActOn ++ refuse kt.targetable (.targetable kt)
    | .changeLife d who => NounPhrase.check (some .player) bs who ++ Amount.check (nomIntro bs who)
        d.amount
    | .exchange what => what.check bs
    | .addMana amt prod riders who =>
      let bs' := nomIntro bs who
      let bs'' := Amount.intro bs' amt
      NounPhrase.check (some .player) bs who ++ Amount.check bs' amt ++ prod.check bs'' ++
        ManaRider.checkAll bs'' riders
    | .draw amt who => NounPhrase.check (some .player) bs who ++ Amount.check (nomIntro bs who) amt
    | .expose _ what who => NounPhrase.check (some .player) bs who ++ Exposed.check (nomIntro bs
        who) what
    | .search sc q p who =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ sc.check bs' ++ Quantity.check bs' q ++
        Predicate.check .object bs' p ++ refuse q.nonZero .nonZeroQ ++
        refuse q.wellFormed .wellFormedQ ++ refuse p.seedZone.isNone .zoneCoherent
    | .shuffle whose => NounPhrase.check (some .player) bs whose
    | .flipCoins count who => NounPhrase.check (some .player) bs who ++ count.check (nomIntro bs
        who)
    | .rollDice count sides who =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ Amount.check bs' count ++
        sides.check (Amount.intro bs' count)
    | .applyResultsTable rows =>
      let n := countOutcomes .rollResult bs
      refuse (!rows.isEmpty) .nonEmpty ++ refuse (n == 1) (.outcomeInScope .rollResult n) ++
        Instruction.checkRows bs rows
    | .ignoreOutcomes which => which.check bs ++ refuse (which.ignorableFor bs) .ignorableFor
    | .shiftResult _ amt =>
      let n := countOutcomes .rollResult bs
      Amount.check bs amt ++ refuse (n == 1) (.outcomeInScope .rollResult n)
    | .storeResults on =>
      let n := countOutcomes .rollResult bs
      NounPhrase.check (some .object) bs on ++ refuse on.plur.isOne .singular ++
        refuse (n == 1) (.outcomeInScope .rollResult n)
    | .rerollStored q whose who =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ Quantity.check bs' q ++ NounPhrase.check (some .object) bs' whose ++
        refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse whose.plur.isOne .singular
    | .establish se span =>
      StaticSpec.check bs se ++ OptDuration.check (StaticSpec.intro bs se) span ++
        refuse (durationOk span) .durationOk ++ refuse se.clauseOk .clauseStatic
    | .createObject count spec agent =>
      let bs' := nomIntro bs agent
      let bs'' := Amount.intro bs' count
      NounPhrase.check (some .player) bs agent ++ Amount.check bs' count ++
        (match spec with
         | .token token riders => TokenSpec.check bs'' token ++ TokenRider.checkAll bs'' riders
         | .emblem abilities => refuse (emblemAbilitiesOk abilities) .emblemAbilities ++
             Ability.checkAll [] abilities)
    | .putCounters amt kind on =>
      let bs' := Amount.intro bs amt
      let bs'' := kind.intro bs'
      let k := on.kindOr .object
      Amount.check bs amt ++ kind.check bs' ++ NounPhrase.check none bs'' on ++
        refuse on.perMemberOk .perMember ++ refuse (kindAmountOk amt kind) .kindAmountOk ++
        refuse (kind.scope k) .counterSourceScope
    | .distribute v amt among =>
      let bs' := v.intro bs
      let bs'' := Amount.intro bs' amt
      v.check bs ++ Amount.check bs' amt ++ NounPhrase.check none bs'' among ++
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
        OptCounterKindSource.check bs kind ++ NounPhrase.check none bs' from_ ++
        refuse (optCounterSourceScope kind k) .counterSourceScope ++
        refuse (from_.counterMemoryOk bs') .counterMemory
    | .moveCounters amt kind src dst =>
      let bs' := Amount.intro bs amt
      let bs'' := nomIntro bs' src
      Amount.check bs amt ++ OptCounterKindSource.check bs kind ++ NounPhrase.check (some .object) bs' src ++
        NounPhrase.check (some .object) bs'' dst ++ refuse (optKindAmountOk amt kind) .kindAmountOk ++
        refuse (optCounterSourceScope kind .object) .counterSourceScope ++
        refuse (src.counterMemoryOk bs') .counterMemory ++ refuse dst.moveDestOk .moveDestination ++
        refuse dst.perMemberOk .perMember
    | .doubleCounters on =>
      let k := on.kindOr .object
      NounPhrase.check none bs on ++ refuse (counterHolderKind k) .counterHolderKind ++
        refuse on.perMemberOk .perMember
    | .enact v e subj =>
      OptNoun.check (some .player) bs subj ++ Instruction.check (agentCtx bs subj) e ++
        refuse (knownAct v) (.knownAct v) ++ refuse (enactAgentOk subj v) .enactAgentOk ++
        refuse (enactPatientZoneOk (agentCtx bs subj) v e) .zoneFits ++
        refuse (enactLibraryOwnerOk v e) .opponentsLibrary ++
        refuse (enactKeepsOuter bs subj e) .enactKeepsOuter
    | .pay c _ who =>
      NounPhrase.check (some .player) bs who ++ Cost.check (nomIntro bs who) c ++
        refuse c.payable .payable ++ refuse (payAgreesOk who c) .payAgrees
    | .withContinuation policy body ifDid ifNot =>
      let bs' := policy.context bs
      (match policy with
       | .optional agent => NounPhrase.check (some .player) bs agent
       | .required => []) ++
        Instruction.check bs' body ++ Instruction.checkOpt (body.intro bs') ifDid ++
        Instruction.checkOpt bs' ifNot ++
        (match policy with
         | .optional _ => []
         | .required => refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure ++
             refuse (ifDoneArmed ifDid ifNot) .ifDoneArmed)
    | .doOnlyIf e c otherwise =>
      let bs' := e.preIntro bs
      Instruction.check bs e ++ Condition.check bs' c ++
        Instruction.checkOpt (Condition.introduced bs' c ++ e.otherwiseCtx bs) otherwise
    | .doIf c e otherwise =>
      Condition.check bs c ++ Instruction.check (c.intro bs) e ++
        Instruction.checkOpt (e.otherwiseCtx (c.intro bs)) otherwise
    | .doForEach grp body =>
      let k := grp.kindOr .object
      let bs' := elemIntro bs k grp
      NounPhrase.check none bs grp ++ refuse k.phrasal (.phrasal k) ++ Instruction.check bs' body ++
        refuse (grp.plur == .many) .plural ++ refuse (keepsOuter bs' body) .keepsOuter
    | .doForEachKind ax dom q body =>
      let bs' := kindValueIntro bs q dom
      OptNoun.check (some .object) bs dom ++ refuse (ax.ok && ax.sort == some q) .kindAxisSort ++
        refuse (kindDomainOk ax dom) .kindDomainOk ++ Instruction.check bs' body ++
        refuse (keepsOuter bs' body) .keepsOuter
    | .repeat_ rep => rep.check bs
    | .sequentially es => refuse (!es.isEmpty) .nonEmpty ++ Instruction.checkSeq bs es
    | .simultaneously es => refuse (!es.isEmpty) .nonEmpty ++ Instruction.checkSim bs es
    | .chooseModes q modes =>
      Quantity.check bs q ++ refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse (atLeastTwo modes.length) .atLeastTwo ++
        refuse (q.modesFit modes.length) .modesFit ++
        refuse (modesCostedUniformly modes) .modesCosted ++ Instruction.checkModes bs modes
    | .delay ev alts span body =>
      GameEvent.check bs ev ++ GameEvent.checkAll bs alts ++ OptDuration.check bs span ++
        Instruction.check (delayedCtx bs alts ev) body ++ refuse (durationOk span) .durationOk
    | .replace replaced repl =>
      Instruction.check bs replaced ++ Instruction.check (replaced.replacedCtx bs) repl ++
        refuse (!replaced.isInstead) .notInstead ++ refuse (!repl.isInstead) .notInstead
    | .holdUntil e ev =>
      Instruction.check bs e ++ GameEvent.check (e.annIntro bs) ev ++
        refuse e.heldUntilOk .heldClause
    | .triggerReflexively body trig =>
      Instruction.check bs body ++ Instruction.check (reflexCtx bs body) trig ++
        refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure
    | .triggerThisWay body ev trig =>
      Instruction.check bs body ++ GameEvent.check (body.intro bs) ev ++
        Instruction.check (thisWayCtx bs body ev) trig ++
        refuse body.thisWayOutcomeOk .thisWayOutcome
    | .skipUntap n steps =>
      NounPhrase.check (some .object) bs n ++ Amount.check (nomIntro bs n) steps ++
        zoneIsCheck (NounPhrase.zone bs n) .battlefield
    | .skipPart _ count who =>
      NounPhrase.check (some .player) bs who ++ Amount.check (nomIntro bs who) count
    | .insertPart part anchor count followedBy who =>
      OptNoun.check (some .player) bs who ++ Amount.check (optAgentIntro bs who) count ++
        refuse (part.proper || (part == .turn && who.isSome && anchor.isNone && followedBy.isNone))
          .windowOk ++ refuse (anchor.elim true TurnPart.proper) .windowOk ++
        refuse (followedBy.elim true TurnPart.proper) .windowOk
  termination_by structural e => e

  def Repetition.check (bs : Bindings) : Repetition → List Refusal
    | .fixed n body =>
      let bs' := Amount.intro bs n
      Amount.check bs n ++ Instruction.check bs' body ++ refuse (keepsOuter bs' body) .keepsOuter
    | .moreTimes n => Amount.check bs n
    | .untilCond c => Condition.check bs c
    | _ => []
  termination_by structural rep => rep

  def Instruction.checkOperands (bs : Bindings) : List NounPhrase → List Refusal
    | [] => []
    | n :: ns => NounPhrase.check none bs n ++ Instruction.checkOperands (nomIntro bs n) ns
  termination_by structural ns => ns

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
    | .mana c => refuse (manaRun c) .manaRun ++ ManaCost.check c
    | .scaled c amt => Cost.check bs c ++ Amount.check bs amt ++ refuse amt.forEach .forEachAmount
    | .tapSymbol | .untapSymbol | .loyaltySymbol _ | .itsManaCost => []
    | .perform e => Instruction.check bs e ++ refuse e.costActionOk .costAction
    | .compound cs => refuse (!cs.isEmpty) .nonEmpty ++ Cost.checkSeq bs cs
    | .or cs =>
      refuse (atLeastTwo cs.length) .atLeastTwo ++ Cost.checkAlternatives bs cs ++
        cs.flatMap (fun c => refuse (!c.isCompound) .notCompound)
  termination_by structural c => c

  def Cost.checkAlternatives (bs : Bindings) : List Cost → List Refusal
    | [] => []
    | c :: cs => Cost.check bs c ++ Cost.checkAlternatives bs cs
  termination_by structural cs => cs

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
    | .noRider | .stateBased _ => []
    | .play from_ _ _ _ payment => OptZoneExpr.check bs from_ ++ PlayPayment.check bs payment
  termination_by structural r => r

  def DamageOp.check (bs : Bindings) : DamageOp → List Refusal
    | .prevent cut also =>
      cut.check bs ++ Instruction.checkOpt (outcomeB .damagePrevented :: cut.intro bs) also
    | .redirect cut to =>
      let bs' := cut.intro bs
      cut.check bs ++ NounPhrase.check none bs' to ++ refuse (to.damageRecipient bs') .damageRecipient ++
        refuse to.plur.isOne .singular
    | .scale sc => sc.check bs
  termination_by structural op => op

  def TokenRider.check (bs : Bindings) : TokenRider → List Refusal
    | .entersAs v => refuse v.markable .statusMarkable
    | .entersAttacking whom => AttackDefender.check bs whom
    | .entersTransformed | .entersMelded _ => []
    | .withCounters amt kind _ =>
      Amount.check bs amt ++ kind.check (Amount.intro bs amt) ++ refuse (kindAmountOk amt kind) .kindAmountOk
    | .under who => NounPhrase.check (some .player) bs who ++ refuse who.ctrlOverrideOk .ctrlOverride
    | .asCopyOf _ src exc =>
      NounPhrase.check (some .object) bs src ++ CopyExcept.checkAll bs exc ++ refuse src.perMemberOk .perMember
  termination_by structural r => r

  def TokenRider.checkAll (bs : Bindings) : List TokenRider → List Refusal
    | [] => []
    | r :: rs => TokenRider.check bs r ++ TokenRider.checkAll bs rs
  termination_by structural rs => rs

  def Characteristics.checkWritten (bs : Bindings) : Characteristics → List Refusal
    | ⟨_, _, _, _, _, _, text, power, toughness, loyalty, defense⟩ =>
      statsCheck bs [power, toughness, loyalty, defense] ++ Ability.checkAll [] text
  termination_by structural t => t

  def CharacteristicBundle.check (bs : Bindings) : CharacteristicBundle → List Refusal
    | ⟨characteristics, qualities⟩ =>
      Characteristics.checkWritten bs characteristics ++ TokenQuality.checkAll bs qualities
  termination_by structural b => b

  def TokenQuality.checkAll (bs : Bindings) : List TokenQuality → List Refusal
    | [] => []
    | .withEveryType _ :: qs => TokenQuality.checkAll bs qs
    | .withQuality q :: qs =>
      Predicate.check .object bs q ++ refuse q.qualityReadOk .qualityRead ++ TokenQuality.checkAll bs qs
  termination_by structural qs => qs

  /-- Numeric edits thread a local scope; other payloads retain the block's incoming scope. -/
  def CharacteristicEdit.checkAll (bs numeric : Bindings) (subject : Option NounPhrase) :
      List CharacteristicEdit → List Refusal
    | [] => []
    | .manaCost cost :: rest =>
      (cost.map ManaCost.check).getD [] ++ CharacteristicEdit.checkAll bs numeric subject rest
    | .stat _ _ amount :: rest =>
      Amount.check numeric amount ++
        CharacteristicEdit.checkAll bs (Amount.intro numeric amount) subject rest
    | .chosenQuality _ q :: rest =>
      Predicate.check .object bs q ++ CharacteristicEdit.checkAll bs numeric subject rest
    | .addedAbilities abilities :: rest =>
      Ability.checkAll [] abilities ++ CharacteristicEdit.checkAll bs numeric subject rest
    | .removedAbilities selection :: rest =>
      AbilitySelection.check bs subject selection ++ CharacteristicEdit.checkAll bs numeric subject rest
    | _ :: rest => CharacteristicEdit.checkAll bs numeric subject rest
  termination_by structural edits => edits

  def AbilitySelection.check (bs : Bindings) (subject : Option NounPhrase) :
      AbilitySelection → List Refusal
    | .specified abilities =>
      (match subject with
       | none => []
       | some n => zoneIsCheck (NounPhrase.zone bs n) .battlefield) ++
        refuse (!abilities.isEmpty) .nonEmpty ++ AbilityLost.checkAll bs abilities
    | .family class_ =>
      refuse class_.known .knownKeywordTerm ++
        (match subject with
         | none => []
         | some n => zoneIsCheck (NounPhrase.zone bs n) .battlefield)
    | .allExcept except =>
      OptPredicate.check .object (subjCtx bs subject) except ++
        (match subject with
         | none => []
         | some n => zoneIsCheck (NounPhrase.zone bs n) .battlefield)
  termination_by structural selection => selection

  def TokenSpec.check (bs : Bindings) : TokenSpec → List Refusal
    | .written t =>
      let c := t.characteristics
      CharacteristicBundle.check bs t ++ refuse c.typed .tokenTyped ++ refuse c.ptOk .tokenPtOk ++
        refuse (subsFitLine c.subtypes c.types) .subsFitLine ++
        refuse c.abilitiesOk .tokenAbilities ++ refuse c.canonical .tokenCanonical ++
        refuse t.qualsFit .tokenQualsFit
    | .asThose =>
      let n := countTokenSpecs bs
      refuse (n == 1) (.tokenSpecInScope n)
    | .copyOf src exc =>
      NounPhrase.check (some .object) bs src ++ CopyExcept.checkAll bs exc ++ refuse src.perMemberOk .perMember
  termination_by structural s => s

  def CopyExcept.check (bs : Bindings) : CopyExcept → List Refusal
    | .edits edits =>
      CharacteristicEdit.checkAll bs bs none edits ++
        refuse (!edits.isEmpty && edits.all (fun edit =>
          match edit with
          | .typeLine _ c => c.hasWrites
          | _ => true)) .lineNonEmpty ++
        refuse (edits.all CharacteristicEdit.canonical) .tokenCanonical ++
        refuse (edits.all CharacteristicEdit.copyFits) .becomesOk
    | .thisAbility => []
    | .ability ab => Ability.check [] ab ++ refuse ab.grantable .grantable
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
      NounPhrase.check (some .object) bs what ++ Instruction.check (nomIntro bs what) says ++
        zoneIsCheck (NounPhrase.zone bs what) .stack
  termination_by structural r => r

  def ManaRider.checkAll (bs : Bindings) : List ManaRider → List Refusal
    | [] => []
    | r :: rs => ManaRider.check bs r ++ ManaRider.checkAll bs rs
  termination_by structural rs => rs

  def StaticSpec.check (bs : Bindings) : StaticSpec → List Refusal
    | .modification n what d =>
      NounPhrase.check (some .object) bs n ++ Amount.check (selfSubjIntro bs n) d.amount ++
        zoneIsCheck (NounPhrase.zone bs n) .battlefield ++ refuse what.modifyOk .modifyStat
    | .ptDefinition n _ amt =>
      NounPhrase.check (some .object) bs n ++ Amount.check (selfSubjIntro bs n) amt ++
        refuse n.selfDefinedOk .selfDefinedOk
    | .ptSwitch n => NounPhrase.check (some .object) bs n ++ zoneIsCheck (NounPhrase.zone bs n)
        .battlefield
    | .costShift n sh =>
      NounPhrase.check (some .object) bs n ++ sh.check bs ++ refuse (n.costSubjectOk bs) .costSubject
    | .altCost n c =>
      NounPhrase.check (some .object) bs n ++ OptCost.check (selfSubjIntro bs n) c ++
        refuse (c.elim true Cost.offBattlefield) .altPayment ++ refuse (n.costSubjectOk bs) .costSubject
    | .addedCost c _ => Cost.check bs c ++ refuse c.offBattlefield .addedPayment
    | .letterDefinition l amt => Amount.check bs amt ++ refuse (anyOpenLetter l bs) (.openLetter l)
    | .abilityGrant n ab =>
      NounPhrase.check (some .object) bs n ++ Ability.check bs ab ++
        refuse (grantSubjectOk bs ab n) .grantSubject ++ refuse ab.grantable .grantable
    | .abilityGrantFrom n cls src except =>
      let bs' := nomIntro bs n
      NounPhrase.check (some .object) bs n ++ NounPhrase.check (some .object) bs' src ++
        OptPredicate.check .object (nomIntro bs' src) except ++ refuse (!cls.isEmpty) .nonEmpty ++
        refuse (distinctClasses cls) .distinct ++ refuse (cls.all AbilityClass.known) .knownKeywordTerm
    | .deonticRule n c deeds role bound patient asThough rider =>
      let k := n.kindOr .object
      let bs' := nomIntro bs n
      NounPhrase.check none bs n ++ Compulsion.check (selfSubjIntro bs n) c ++
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
        refuse (deedFits deeds role k n.isAbility (NounPhrase.headTys bs n) (NounPhrase.zone bs n)) .deedFits ++
        refuse (deonticBoundOk bs' deeds bound) .deonticBoundOk ++
        refuse (deonticPatientOk bs n deeds role patient rider) .deonticPatientOk ++
        refuse (asThoughOk c deeds asThough) .asThoughOk ++
        refuse (deonticRiderOk (patient.intro bs') deeds role c patient asThough.isSome rider)
          .deonticRiderOk
    | .manaRetention who what => NounPhrase.check (some .player) bs who ++ what.check (nomIntro bs
        who)
    | .partSkip who _ => NounPhrase.check (some .player) bs who
    | .characteristicChange n edits =>
      NounPhrase.check (some .object) bs n ++ CharacteristicEdit.checkAll bs bs (some n) edits ++
        refuse (CharacteristicEdit.fitsAll bs n edits) .becomesOk
    | .retention se n =>
      StaticSpec.check bs se ++ NounPhrase.check (some .object) (StaticSpec.intro bs se) n ++
        refuse se.notCarvedOut .notCarvedOut
    | .copyChange n src exc =>
      let bs' := nomIntro bs n
      NounPhrase.check (some .object) bs n ++ NounPhrase.check (some .object) bs' src ++
        CopyExcept.checkAll (nomIntro bs' src) exc ++ refuse src.perMemberOk .perMember
    | .controlGrant who what =>
      let bs' := nomIntro bs who
      NounPhrase.check (some .player) bs who ++ NounPhrase.check (some .object) bs' what ++
        zoneIsCheck (NounPhrase.zone bs' what) .battlefield
    | .replacement ev alts window repl _ limit =>
      GameEvent.check bs ev ++ GameEvent.checkAll bs alts ++ OptTiming.check bs window ++
        Instruction.check (interceptCtx bs alts ev) repl ++
        refuse (untriggeredLimitOk limit) .untriggeredLimit ++
        refuse (interceptOk ev) .interceptable ++ refuse (interceptArmsOk alts) .interceptable
    | .damageRule _ src scope op use =>
      let bs' := src.intro bs
      let bs'' := scope.intro bs'
      src.check bs ++ scope.check bs' ++ op.check bs'' ++ refuse (op.useOk use) .damageOpUse
    | .preventionBan _ what _ => what.check bs
    | .conditional se c marking =>
      StaticSpec.check bs se ++ Condition.check (StaticSpec.intro bs se) c ++
        refuse (markingOk marking c) .markingOk
    | .visibility v who what =>
      NounPhrase.check (some .player) bs who ++ VisibleThing.check (nomIntro bs who) what ++
        refuse (visibilityOk v what) .visibilityOk
    | .additionalTriggers ev q =>
      GameEvent.check bs ev ++ Quantity.check bs q ++ refuse q.nonZero .nonZeroQ ++
        refuse q.wellFormed .wellFormedQ ++ refuse (Quantity.introduced bs q).isEmpty .quantLiteral ++
        refuse (triggerCountOk ev) .triggerCountOk
    | .entryRider n rider =>
      NounPhrase.check (some .object) bs n ++ TokenRider.check (nomIntro bs n) rider ++
        zoneIsCheck (NounPhrase.zone bs n) .battlefield
    | .choice _ n q dom _ =>
      NounPhrase.check (some .object) bs n ++ sortedDomainCheck bs q dom ++ zoneIsCheck (NounPhrase.zone bs n) .battlefield
    | .conjunction subject parts =>
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
    | .written ab :: rest => Ability.check bs ab ++ refuse ab.grantable .grantable ++ AbilityLost.checkAll bs rest
    | .term t :: rest => refuse t.known .knownKeywordTerm ++ AbilityLost.checkAll bs rest
  termination_by structural abl => abl

  def KeywordParam.check (bs : Bindings) (qualityDomain : Option Kind) :
      KeywordParam → List Refusal
    | .cost c => Cost.check [] c
    | .quality p =>
      match qualityDomain with
      | some k => Predicate.check k bs p
      | none =>
        let k := p.kindOr .object
        Predicate.check k bs p ++ refuse k.qualityParam (.phrasal k)
    | .subject p => Predicate.check (p.kindOr .object) [] p
    | .number amt => Amount.check [] amt
    | .deckCondition dc => dc.check
  termination_by structural p => p

  def KeywordParam.checkAll (bs : Bindings) (schema : KeywordSchema) :
      List KeywordParam → List Refusal
    | [] => []
    | p :: ps =>
      KeywordParam.check bs (schema.head? >>= (·.qualityDomain)) p ++
        KeywordParam.checkAll bs schema.tail ps
  termination_by structural params => params

  def Ability.check (bs : Bindings) : Ability → List Refusal
    | .keyword k param body =>
      KeywordParam.checkAll bs ((keywordSchemaFor k param).getD []) param ++
        (match body with
         | none => []
         | some b => Ability.check [] b) ++
        refuse (keywordParamFits k param) (.keywordParamFits k) ++
        refuse (keywordBodyFits k body) (.keywordBodyFits k) ++
        refuse (keywordCostPaidByYou k param) (.keywordCostPaidByYou k)
    | .activated cost instr window limit guard activator =>
      let bsc := dropLetter .x bs
      Cost.check bsc cost ++ Instruction.check (publicOnly (cost.intro bsc)) instr ++
        refuse cost.tapOnce .costTapOnce ++ refuse cost.paidByYou .costPaidByYou ++
        OptTiming.check bs window ++ refuse (untriggeredLimitOk limit) .untriggeredLimit ++
        OptCondition.check bs guard ++ OptNoun.check (some .player) bs activator
    | .triggered ev alts while_ joins window limit intervening instr =>
      let hctx := headerCtx bs alts ev
      let jctx := joinedCtx bs joins hctx
      headerEventCheck bs ev ++ alts.flatMap (headerEventCheck bs) ++
        OptConcurrent.check hctx while_ ++ joins.flatMap (JoinedHeader.check bs) ++
        OptTiming.check bs window ++ OptCondition.check jctx intervening ++
        Instruction.check (interveningIntro jctx intervening) instr ++
        refuse (chapterDefaultsOk ev alts while_ joins window limit intervening) .chapterDefaults
    | .static se =>
      StaticSpec.check bs se ++ refuse (!anyTargetedAt (StaticSpec.intro bs se)) .nontarget
    | .spell window instr => OptTiming.check bs window ++ Instruction.check bs instr
    | .mayBeginOnBattlefield => []
    | .alsoForKeywords ab ks =>
      Ability.check bs ab ++ refuse ab.keywordExtendable .keywordExtendable ++
        refuse (keywordListOk ab ks) .keywordListOk
    | .italicHead _ ab => Ability.check bs ab ++ refuse ab.notWordHeaded .notWordHeaded
    | .thatAbility ref =>
      let n := countChoice (.quality .ability) bs
      refuse (ref.ok n) (.choiceRef ref (.quality .ability) n)
  termination_by structural ab => ab

  def Ability.checkAll (bs : Bindings) : List Ability → List Refusal
    | [] => []
    | ab :: abl => Ability.check bs ab ++ Ability.checkAll bs abl
  termination_by structural abl => abl
end

/-- A card's text: each line read against the choices the lines before it announced. -/
def Ability.checkText (bs : Bindings) : List Ability → List Refusal
  | [] => []
  | ab :: rest => Ability.check bs ab ++ Ability.checkText (ab.intro bs) rest

end Semantics
