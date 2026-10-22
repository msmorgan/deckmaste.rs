import Semantics.Definitions.Actions

/-!
# Semantics.Definitions.Keywords

Rules-defined abilities spelled over `Semantics.Templates` and the actions of
`Semantics.Definitions.Actions`: abilities, ability words and keyword abilities.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Abilities -/

/-- "Companion — <condition>" -/
semantic_definition companion (condition : DeckCondition) : Ability :=
  .keyword (.named "companion") [.deckCondition condition] []
/-- "Level up [cost]" with its definition, an activated ability: "[Cost]: Put a level counter on
this permanent. Activate only as a sorcery." [CR#702.87a] -/
semantic_definition levelUp (cost : Cost) : Ability :=
  .keyword (.named "levelUp") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (putCounters 1 levelCounter thisPermanent) asSorcery ]
/-- "Storm" with its reminder text: "When you cast this spell, copy it for each other spell that
was cast before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_definition storm : Ability :=
  .keyword (.named "storm") []
    [ when (does you (cast thisSpell))
        (sequentially
          [ .copy .fromStack thisSpell
              (eventCount
                (cast (a (and [isSpell, isOther])))
                (a .anyPlayer) earlierThisTurn)
              [],
            may (.chooseNewTargets (.pro (.word .copy) .many)) ]) ]
/-- "Cascade" with its reminder text [CR#702.85a]. -/
semantic_definition cascade : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  .keyword (.named "cascade") []
    [ when (does you (cast thisSpell))
        (sequentially
          [ exile (.run .top (and [non isLand, cheaper]) (libraryOf .you)),
            may (castWithoutPaying (that .card) cheaper),
            .move
              (allOf (and [isCard, was (exile which) thisWay, not (was (cast which) thisWay)]))
              .anyLocation onBottom .arrival none [(.inOrder .randomOrder)] ]) ]
/-- "Ripple N" with its definition [CR#702.60a]: "When you cast this spell, you may reveal the top
N cards of your library. If you do, you may cast any of those cards with the same name as this
spell without paying their mana costs, then put all revealed cards not cast this way on the bottom
of your library in any order." -/
semantic_definition ripple (amount : Amount) : Ability :=
  .keyword (.named "ripple") [.number (.inCaller amount)]
    [ when (does you (cast thisSpell))
        (may (reveal (.slice .top (.inCaller amount) (libraryOf .you)))
          (some
            (sequentially
              [ castWithoutPaying
                  (.someOf (.counted anyNumber) (some (.shares .cardName .share this_))
                    (those .card))
                  none,
                .move
                  (allOf
                    (and [isCard, was (reveal which) thisWay, not (was (cast which) thisWay)]))
                  .anyLocation onBottom .arrival none [.inOrder .anyOrder] ]))) ]
/-- "Renown N" with its reminder text: "When this creature deals combat damage to a player, if it
isn't renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_definition renown (count : Amount) : Ability :=
  .keyword (.named "renown") [.number (.inCaller count)]
    [ when (does thisCreature (dealDamage (to := a .anyPlayer) (kind := some .combat)))
        (if_ := some (not (is thisCreature (hasDesignation renowned))))
        (sequentially
          [ putCounters (.inCaller count) p1p1Counter thisCreature,
            gainDesignation thisCreature renowned ]) ]
/-- "Cumulative upkeep [cost]" with its reminder text [CR#702.24a]. -/
semantic_definition cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost (.inCaller cost)]
    [ at_ (.beginningOf .upkeep (some .you) (.each none))
        (if_ := is thisPermanent isPermanent)
        (sequentially
          [ putCounters 1 ageCounter thisPermanent,
            doUnless (sacrifice thisPermanent)
              (.scaled (.inCaller cost) (times 1 (countersOn ageCounter thisPermanent))) ]) ]
semantic_definition reach : Ability := keyword (.named "reach")
/-- "Flying" with its definition, a static ability: "This creature can't be blocked except by
creatures with flying and/or reach." [CR#702.9b] -/
semantic_definition flying : Ability :=
  .keyword (.named "flying") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          andOr [hasKeyword (.the (.named "flying")), hasKeyword reach]])))) ]
/-- "Bushido N" with its definition, a triggered ability [CR#702.45a]. -/
semantic_definition bushido (count : Amount) : Ability :=
  .keyword (.named "bushido") [.number (.inCaller count)]
    [ whenever (does thisCreature block) (or_ := [does any (block thisCreature)])
        (get thisCreature (.up (.inCaller count)) (.up (.inCaller count)) untilEndOfTurn) ]
/-- "Modular N" with its definition, a static ability and a triggered one [CR#702.43a]. -/
semantic_definition modular (count : Amount) : Ability :=
  .keyword (.named "modular") [.number (.inCaller count)]
    [ static (entersWithCounters thisPermanent (.inCaller count) p1p1Counter),
      when (does thisPermanent die)
        (may (putCounters (countersOn p1p1Counter thisPermanent) p1p1Counter
          (target (and [isArtifact, isCreature])))) ]
/-- "Cycling [cost]" with its definition, an activated ability: "[Cost], Discard this card: Draw a
card." [CR#702.29a] -/
semantic_definition cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost (.inCaller cost)]
    [activated (.compound [.inCaller cost, .perform (discard this_)]) (draw 1)]

/-! ## Ability words

An ability word has no rules meaning [CR#207.2c]: each is a Template, the italic word over the
ability it heads, and no Definition. -/

semantic_template adamant (ability : Ability) : Ability := abilityWord (.named "adamant") ability
semantic_template addendum (ability : Ability) : Ability := abilityWord (.named "addendum") ability
semantic_template alliance (ability : Ability) : Ability := abilityWord (.named "alliance") ability
semantic_template battalion (ability : Ability) : Ability := abilityWord (.named "battalion") ability
semantic_template bloodrush (ability : Ability) : Ability := abilityWord (.named "bloodrush") ability
semantic_template celebration (ability : Ability) : Ability := abilityWord (.named "celebration") ability
semantic_template channel (ability : Ability) : Ability := abilityWord (.named "channel") ability
semantic_template chroma (ability : Ability) : Ability := abilityWord (.named "chroma") ability
semantic_template cohort (ability : Ability) : Ability := abilityWord (.named "cohort") ability
semantic_template constellation (ability : Ability) : Ability :=
  abilityWord (.named "constellation") ability
semantic_template converge (ability : Ability) : Ability := abilityWord (.named "converge") ability
semantic_template councilsDilemma (ability : Ability) : Ability :=
  abilityWord (.named "council’s dilemma") ability
semantic_template coven (ability : Ability) : Ability := abilityWord (.named "coven") ability
semantic_template delirium (ability : Ability) : Ability := abilityWord (.named "delirium") ability
semantic_template descend4 (ability : Ability) : Ability := abilityWord (.named "descend 4") ability
semantic_template descend8 (ability : Ability) : Ability := abilityWord (.named "descend 8") ability
semantic_template disappear (ability : Ability) : Ability := abilityWord (.named "disappear") ability
semantic_template domain (ability : Ability) : Ability := abilityWord (.named "domain") ability
semantic_template eerie (ability : Ability) : Ability := abilityWord (.named "eerie") ability
semantic_template eminence (ability : Ability) : Ability := abilityWord (.named "eminence") ability
semantic_template enrage (ability : Ability) : Ability := abilityWord (.named "enrage") ability
semantic_template fatefulHour (ability : Ability) : Ability := abilityWord (.named "fateful hour") ability
semantic_template fathomlessDescent (ability : Ability) : Ability :=
  abilityWord (.named "fathomless descent") ability
semantic_template ferocious (ability : Ability) : Ability := abilityWord (.named "ferocious") ability
semantic_template flurry (ability : Ability) : Ability := abilityWord (.named "flurry") ability
semantic_template formidable (ability : Ability) : Ability := abilityWord (.named "formidable") ability
semantic_template grandeur (ability : Ability) : Ability := abilityWord (.named "grandeur") ability
semantic_template hellbent (ability : Ability) : Ability := abilityWord (.named "hellbent") ability
semantic_template heroic (ability : Ability) : Ability := abilityWord (.named "heroic") ability
semantic_template imprint (ability : Ability) : Ability := abilityWord (.named "imprint") ability
semantic_template infusion (ability : Ability) : Ability := abilityWord (.named "infusion") ability
semantic_template inspired (ability : Ability) : Ability := abilityWord (.named "inspired") ability
semantic_template joinForces (ability : Ability) : Ability := abilityWord (.named "join forces") ability
semantic_template kinship (ability : Ability) : Ability := abilityWord (.named "kinship") ability
semantic_template landfall (ability : Ability) : Ability := abilityWord (.named "landfall") ability
semantic_template lieutenant (ability : Ability) : Ability := abilityWord (.named "lieutenant") ability
semantic_template magecraft (ability : Ability) : Ability := abilityWord (.named "magecraft") ability
semantic_template metalcraft (ability : Ability) : Ability := abilityWord (.named "metalcraft") ability
semantic_template morbid (ability : Ability) : Ability := abilityWord (.named "morbid") ability
semantic_template opus (ability : Ability) : Ability := abilityWord (.named "opus") ability
semantic_template packTactics (ability : Ability) : Ability := abilityWord (.named "pack tactics") ability
semantic_template paradox (ability : Ability) : Ability := abilityWord (.named "paradox") ability
semantic_template parley (ability : Ability) : Ability := abilityWord (.named "parley") ability
semantic_template radiance (ability : Ability) : Ability := abilityWord (.named "radiance") ability
semantic_template raid (ability : Ability) : Ability := abilityWord (.named "raid") ability
semantic_template rally (ability : Ability) : Ability := abilityWord (.named "rally") ability
semantic_template renew (ability : Ability) : Ability := abilityWord (.named "renew") ability
semantic_template repartee (ability : Ability) : Ability := abilityWord (.named "repartee") ability
semantic_template revolt (ability : Ability) : Ability := abilityWord (.named "revolt") ability
semantic_template secretCouncil (ability : Ability) : Ability :=
  abilityWord (.named "secret council") ability
semantic_template spellMastery (ability : Ability) : Ability :=
  abilityWord (.named "spell mastery") ability
semantic_template strive (ability : Ability) : Ability := abilityWord (.named "strive") ability
semantic_template survival (ability : Ability) : Ability := abilityWord (.named "survival") ability
semantic_template sweep (ability : Ability) : Ability := abilityWord (.named "sweep") ability
semantic_template temptingOffer (ability : Ability) : Ability :=
  abilityWord (.named "tempting offer") ability
semantic_template threshold (ability : Ability) : Ability := abilityWord (.named "threshold") ability
semantic_template undergrowth (ability : Ability) : Ability := abilityWord (.named "undergrowth") ability
semantic_template valiant (ability : Ability) : Ability := abilityWord (.named "valiant") ability
semantic_template vivid (ability : Ability) : Ability := abilityWord (.named "vivid") ability
semantic_template void (ability : Ability) : Ability := abilityWord (.named "void") ability
semantic_template willOfTheCouncil (ability : Ability) : Ability :=
  abilityWord (.named "will of the council") ability

/-! ## Keyword abilities -/

semantic_definition absorb (amount : Amount) : Ability :=
  .keyword (.named "absorb") [.number (.inCaller amount)]
    [ static
        (.damageRule .unattributed (.toRecipient thisCreature)
          (.prevent (.some (.inCaller amount)) none)
          .repeatedly) ]
semantic_definition affinity (quality : Predicate) : Ability :=
  .keyword (.named "affinity") [.quality (.inCaller quality)]
    [ static
        (.costShift thisSpell
          (.generic (.down (forEach 1 (and [.inCaller quality, youControl]))) none)) ]
semantic_definition afflict (amount : Amount) : Ability :=
  .keyword (.named "afflict") [.number (.inCaller amount)]
    [ whenever (does .any (.block thisCreature))
        (instruct defendingPlayer (loseLife (.inCaller amount))) ]
semantic_definition afterlife (amount : Amount) : Ability :=
  .keyword (.named "afterlife") [.number (.inCaller amount)]
    [ when (does thisPermanent die)
        (createToken (creatureToken 1 1 [.white, .black] [spirit] [flying]) (.inCaller amount)) ]
semantic_definition aftermath : Ability :=
  .keyword (.named "aftermath") []
    [ static
        (can theActor (cast this_ (from_ := yourGraveyard))),
      static
        (cant theActor
          (cast this_ (from_ := some (not yourGraveyard)))),
      static
        (replace (would thisSpell (leave stack))
          (doIf (is thisSpell (.castFrom graveyard)) (instead (exile thisSpell)))) ]
semantic_definition amplify (amount : Amount) : Ability :=
  .keyword (.named "amplify") [.number (.inCaller amount)]
    [ static
        (replace (would thisPermanent enter)
          (also
            (sequentially
              [ .expose .reveal
                  (.cards
                    (counted anyNumber
                      (and [cardIn yourHand, .shares (.subtype .creature) .share thisPermanent]))),
                putCounters (times (.inCaller amount) (.countOf them)) p1p1Counter
                  thisPermanent ]))) ]
semantic_definition annihilator (amount : Amount) : Ability :=
  .keyword (.named "annihilator") [.number (.inCaller amount)]
    [ whenever (does thisCreature (.attack none))
        (instruct defendingPlayer
          (sacrifice (counted (.exactlyOf (.inCaller amount)) isPermanent))) ]
semantic_definition ascend : Ability :=
  .keyword (.named "ascend") []
    [ static
        (onlyIfSo
          (.whileHolds
            (and
              [ .compareAmt (countOf (and [isPermanent, youControl])) .atLeast 10,
                not (is .you (hasDesignation citysBlessing)) ])
            (gainDesignation .you citysBlessing restOfGame))
          (is this_ isPermanentType)),
      spell
        (doIf (is this_ isInstantOrSorcery)
          (doIf
            (and
              [ .compareAmt (countOf (and [isPermanent, youControl])) .atLeast 10,
                not (is .you (hasDesignation citysBlessing)) ])
            (gainDesignation .you citysBlessing restOfGame)
            none)
          none) ]
semantic_definition assist : Ability := keyword (.named "assist")
semantic_definition auraSwap (cost : Cost) : Ability :=
  .keyword (.named "auraSwap") [.cost (.inCaller cost)]
    [ activated (.inCaller cost)
        (may
          (.exchange
            (.cardsAcross thisPermanent
              (a (and [.isCard, .hasSubtype aura, .in_ yourHand]))))) ]
semantic_definition awaken (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "awaken") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      spell
        (doIf (keywordCostPaid (.named "awaken") this_)
          (sequentially
            [ putCounters (.inCaller amount) p1p1Counter
                (target (and [isLand, youControl])),
              .establish
                (.characteristicChange (that land)
                  [ .typeLine .adds
                      { types := some [.creature], subtypes := some [elemental] },
                    .stat .sets .power 0,
                    .stat .sets .toughness 0,
                    .addedAbilities [keyword (.named "haste")] ])
                none ])
          none) ]
semantic_definition backup (amount : Amount) : Ability :=
  .keyword (.named "backup") [.number (.inCaller amount)]
    [ when (does thisCreature enter)
        (sequentially
          [ putCounters (.inCaller amount) p1p1Counter (target isCreature),
            doIf (is (that creature) (and [isCreature, isOther]))
              (establishThroughout
                (Templates.abilityGrantSelected (that creature) thisCreature
                  (.printedAfter (.named "backup")))
                untilEndOfTurn)
              none ]) ]
semantic_definition banding : Ability := keyword (.named "banding")
/-- "Bands with other <quality>", a special form of banding [CR#702.22b]. Unlike a variant, it
keeps its own label: the banding rules count a creature with it as one without banding
[CR#702.22c], and only losing banding takes it too [CR#702.22b]. -/
semantic_definition bandsWithOther (quality : Predicate) : Ability :=
  keywordQuality (.named "bandsWithOther") (.inCaller quality)
semantic_definition bargain : Ability :=
  .keyword (.named "bargain") []
    [ static
        (.addedCost
          (.perform
            (sacrifice
              (a
                (or
                  [ and [isArtifact, isPermanent],
                    and [isEnchantment, isPermanent],
                    .isToken ]))))
          true .once) ]
semantic_definition battleCry : Ability :=
  .keyword (.named "battleCry") []
    [ whenever (does thisCreature (.attack none))
        (establishThroughout
          (Templates.getsPt (each (and [isCreature, isAttacking, isOther]))
            (.up 1) (.up 0))
          untilEndOfTurn) ]
semantic_definition bestow (cost : Cost) : Ability :=
  .keyword (.named "bestow") [.cost (.inCaller cost)]
    [static (.altCost thisSpell (some (.inCaller cost)))]
semantic_definition blitz (cost : Cost) : Ability :=
  .keyword (.named "blitz") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does you
            (.pay .itsManaCost .once .paid thisSpell (some (.named "blitz"))))
          (atNext .endStep (sacrifice (.becomes thisSpell)))),
      static
        (onlyWhile (abilityGrant thisPermanent (keyword (.named "haste")))
          (keywordCostPaid (.named "blitz") thisPermanent)),
      static
        (onlyWhile
          (abilityGrant thisPermanent
            (when (does thisPermanent die) (draw 1)))
          (keywordCostPaid (.named "blitz") thisPermanent)) ]
semantic_definition bloodthirst (amount : Amount) : Ability :=
  .keyword (.named "bloodthirst") [.number (.inCaller amount)]
    [ static
        (replace (would thisPermanent enter)
          (doIf (did anOpponent (.dealDamage .anyAmount .which) soFarThisTurn)
            (withRiders [.withCounters (.inCaller amount) p1p1Counter .fresh]))) ]
semantic_definition boast (ability : Ability) : Ability :=
  .keyword (.named "boast") [.ability (.inCaller ability)]
    [ .qualified (.inCaller ability) none oncePerTurn
        (is thisCreature (did which (.attack none) soFarThisTurn))
        none ]
semantic_definition buyback (cost : Cost) : Ability :=
  .keyword (.named "buyback") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .once),
      static
        (replace (.would .any (.move thisSpell stack graveyard .departure none []))
          (doIf (keywordCostPaid (.named "buyback") thisSpell)
            (instead (.move thisSpell .anyLocation hand .arrival none [])))) ]
semantic_definition casualty (amount : Amount) : Ability :=
  .keyword (.named "casualty") [.number (.inCaller amount)]
    [ static
        (.addedCost
          (.perform (sacrifice (a (and [isCreature, hasPower .atLeast (.inCaller amount)]))))
          true .once),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid (.named "casualty") thisSpell)
        (sequentially [copyOnStack thisSpell, may (.chooseNewTargets (that .copy))]) ]
semantic_definition champion (quality : Predicate) : Ability :=
  .keyword (.named "champion") [.subject (.inCaller quality)]
    [ when (does thisPermanent enter)
        (mayOrElse
          (pay
            (.perform (exile (a (and [.inCaller quality, youControl, isOther]))))
            .once)
          (sacrifice thisPermanent)),
      when (does thisPermanent leave)
        (returnTo (the (and [was (exile which), isCard])) battlefield
          [.under (ownerOf it)]) ]
/-- "Changeling" means "This object is every creature type": a Characteristic-Defining Ability
of the object, a Kindred instant or sorcery as much as a creature [CR#702.73a,604.3a]. -/
semantic_definition changeling : Ability :=
  .keyword (.named "changeling") [] [static (qualityDefinition this_ .adds (everyTypeOf .creature))]
semantic_definition chooseABackground : Ability := keyword (.named "chooseABackground")
semantic_definition cipher : Ability :=
  .keyword (.named "cipher") []
    [ spell
        (doIf (is this_ .isCard)
          (may
            (sequentially
              [ exile thisSpell,
                .relate this_ encodedOn (a (and [isCreature, youControl])) ]))
          none),
      static
        (abilityGrant (.counterpart encodedOn this_)
          (whenever
            (does thisCreature (.dealDamage .anyAmount (a .anyPlayer) (some .combat)))
            (may
              (sequentially
                [ .copy .fromCardZone
                    (the (and [.isCard, .related encodedOn thisCreature]))
                    1 [],
                  may (cast (that .copy) (payment := withoutPaying)) ])))) ]
semantic_definition cleave (cost : Cost) : Ability :=
  .keyword (.named "cleave") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (onlyWhile (.textChange thisSpell .removeBracketed)
          (keywordCostPaid (.named "cleave") thisSpell)) ]
semantic_definition compleated : Ability := keyword (.named "compleated")
semantic_definition conspire : Ability :=
  .keyword (.named "conspire") []
    [ static
        (.addedCost
          (.perform
            (tap
              (counted (exactly 2)
                (and [isUntapped, isCreature, youControl, .shares .color .share thisSpell]))))
          true .once),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid (.named "conspire") thisSpell)
        (sequentially [copyOnStack thisSpell, may (.chooseNewTargets (that .copy))]) ]
semantic_definition convoke : Ability :=
  .keyword (.named "convoke") []
    [ static
        (.paySubstitute thisSpell
          ⟨ tap (a (and [isUntapped, isCreature, youControl])),
            [.generic, .ofColor (.colorOf (that creature))] ⟩) ]
semantic_definition craft (cost : Cost) : Ability :=
  keywordCosting (.named "craft") (.inCaller cost)
semantic_definition crew (amount : Amount) : Ability :=
  .keyword (.named "crew") [.number (.inCaller amount)]
    [ activated
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast (.inCaller amount)
              (allOf
                (and
                  [ isUntapped,
                    and [isCreature, youControl, isOther] ])))))
        (establishThroughout
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          untilEndOfTurn) ]
semantic_definition dash (cost : Cost) : Ability :=
  .keyword (.named "dash") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does you
            (.pay .itsManaCost .once .paid thisSpell (some (.named "dash"))))
          (atNext .endStep (returnToHand (.becomes thisSpell)))),
      static
        (onlyWhile (abilityGrant thisPermanent (keyword (.named "haste")))
          (keywordCostPaid (.named "dash") thisPermanent)) ]
semantic_definition daybound : Ability :=
  .keyword (.named "daybound") []
    [ static
        (replace (would thisPermanent enter)
          (doIf (gameIs night) (withRiders [.entersTransformed]))),
      static
        (.asEvent (gameBecomes night)
          (doIf (is thisPermanent (.currentFace .front))
            (transform thisPermanent)
            none)),
      static
        (cant any (transform thisPermanent) (except := some (.byKeyword (.named "daybound")))) ]
semantic_definition deathtouch : Ability := keyword (.named "deathtouch")
semantic_definition decayed : Ability :=
  .keyword (.named "decayed") []
    [ static (cant thisCreature (.block .any)),
      when (does thisCreature (.attack none))
        (atNext .endOfCombat (sacrifice thisPermanent)) ]
semantic_definition defender : Ability :=
  .keyword (.named "defender") [] [static (cant thisCreature (.attack none))]
semantic_definition delve : Ability :=
  .keyword (.named "delve") []
    [ static
        (.paySubstitute thisSpell
          ⟨ exile (a (cardIn yourGraveyard)),
            [.generic] ⟩) ]
semantic_definition demonstrate : Ability :=
  .keyword (.named "demonstrate") []
    [ when (does you (cast thisSpell))
        (may (sequentially [copyOnStack thisSpell, may (.chooseNewTargets (that .copy))])
          (some
            (sequentially
              [ choose anOpponent,
                instruct (the isChosenPlayer) (copyOnStack thisSpell),
                instruct (the isChosenPlayer) (may (.chooseNewTargets (that .copy))) ]))) ]
semantic_definition dethrone : Ability :=
  .keyword (.named "dethrone") []
    [ whenever
        (does thisCreature
          (.attack
            (some
              (the (and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))))
        (putCounters 1 p1p1Counter thisCreature) ]
/-- "Devoid" means "This object is colorless": a Characteristic-Defining Ability
[CR#702.114a,604.3a]. -/
semantic_definition devoid : Ability :=
  .keyword (.named "devoid") [] [static (qualityDefinition this_ .sets (colored (.some [])))]
/-- "<it> devours <subject>": sacrifices it as a result of its devour ability as it enters, the
verb devour defines for itself [CR#702.82b]. It is done by sacrificing, so the sacrifice stands
too ("each creature sacrificed this way" and "creatures it devoured" both read it). Named in its
sort, as `devour` is the keyword ability. -/
semantic_template Action.devour (subject : NounPhrase) : Action :=
  enact (.ofAbility (.named "devour")) (sacrifice subject)
/-- Devour's definition over the permanents `quality` describes, counted by the variant's head
noun `head`: "As this object enters, you may sacrifice any number of [quality] permanents. This
permanent enters with N +1/+1 counters on it for each [head] sacrificed this way." Devour counts
each creature [CR#702.82a], devour [quality] each permanent [CR#702.82c]. -/
semantic_template devourEntry (quality head : Predicate) (amount : Amount) : Ability :=
  static
    (replace (would thisPermanent enter)
      (also
        (may (Action.devour (counted anyNumber (and [quality, .in_ battlefield])))
          (some
            (putCounters (forEach amount (and [head, was (sacrifice which) thisWay]))
              p1p1Counter thisPermanent)))))
/-- "Devour [quality] N", a variant of devour, so it carries devour's label [CR#702.82c]. -/
semantic_definition devourQuality (quality : Predicate) (amount : Amount) : Ability :=
  .keyword (.named "devour") [.quality (.inCaller quality), .number (.inCaller amount)]
    [devourEntry (.inCaller quality) isPermanent (.inCaller amount)]
/-- "Devour N": devour over creatures [CR#702.82a]. -/
semantic_definition devour (amount : Amount) : Ability :=
  .keyword (.named "devour") [.number (.inCaller amount)]
    [devourEntry isCreature isCreature (.inCaller amount)]
semantic_definition disguise (cost : Cost) : Ability :=
  .keyword (.named "disguise") [.cost (.inCaller cost)]
    [ static
        (can you (cast thisCard (payment := .faceDown faceDownWardedCreature (mana [generic 3])))),
      static (.specialAction none (.inCaller cost) (turnFaceUp this_)) ]
semantic_definition disturb (cost : Cost) : Ability :=
  keywordCosting (.named "disturb") (.inCaller cost)
semantic_definition doctorsCompanion : Ability := keyword (.named "doctorsCompanion")
semantic_definition doubleStrike : Ability := keyword (.named "doubleStrike")
semantic_definition dredge (amount : Amount) : Ability :=
  .keyword (.named "dredge") [.number (.inCaller amount)]
    [ static
        (onlyWhile
          (replace (would you (draw 1))
            (instead
              (may (sequentially [mill (.inCaller amount), returnToHand this_ yourGraveyard]))))
          (.compareAmt (countOf (cardIn yourLibrary)) .atLeast (.inCaller amount))) ]
semantic_definition echo (cost : Cost) : Ability := keywordCosting (.named "echo") (.inCaller cost)
semantic_definition embalm (cost : Cost) : Ability :=
  .keyword (.named "embalm") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create 1
          (.copyOf this_
            [ .edits
                [ .colors .sets (.some [.white]),
                  .manaCost none,
                  .typeLine .adds { subtypes := [zombie] } ] ]))
        asSorcery ]
semantic_definition emerge (cost : Cost) : Ability :=
  .keyword (.named "emerge") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_
            (payment :=
              .payingInstead (.compound [.inCaller cost, .perform (sacrifice (a isCreature))])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (less
              (.statOf (.stat .manaValue) (the (and [was (sacrifice which), isCreature])))
              none))
          (keywordCostPaid (.named "emerge") thisSpell)) ]
/-- "Emerge from [quality] [cost]", a variant of emerge, so it carries emerge's label, its
quality telling it apart, and its cost is an emerge cost [CR#702.119b]. -/
semantic_definition emergeFrom (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "emerge") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ static
        (can you
          (cast this_
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform (sacrifice (a (and [.inCaller quality, isPermanent]))) ])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (less
              (.statOf (.stat .manaValue) (the (and [was (sacrifice which), isPermanent])))
              none))
          (keywordCostPaid (.named "emerge") thisSpell)) ]
semantic_definition enchant (quality : Predicate) : Ability :=
  .keyword (.named "enchant") [.subject (.inCaller quality)]
    [ static (.addsTarget thisAura (.inCaller quality)),
      static (mayBeAttachedTo thisAura (allOf (.inCaller quality))),
      static
        (.asEvent (does .any (.move thisAura (not stack) battlefield .arrival none []))
          (attachTo thisAura (a (.inCaller quality)))) ]
semantic_definition encore (cost : Cost) : Ability :=
  keywordCosting (.named "encore") (.inCaller cost)
semantic_definition enlist : Ability := keyword (.named "enlist")
semantic_definition entwine (cost : Cost) : Ability :=
  .keyword (.named "entwine") [.cost (.inCaller cost)]
    [static (.modeChoice (.allInstead (some (.inCaller cost)) none))]
semantic_definition epic : Ability :=
  .keyword (.named "epic") []
    [ spell
        (establishThroughout (cant you (cast .any)) restOfGame),
      spell
        (delayWithin (beginningOfYour .upkeep) restOfGame
          (sequentially
            [ .copy .fromStack thisSpell 1
                [.edits [.removedAbilities (.specified [.term (.the (.named "epic"))])]],
              may (.chooseNewTargets (that .copy)) ])) ]
semantic_definition equip (cost : Cost) : Ability :=
  .keyword (.named "equip") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (and [isCreature, youControl])))
        asSorcery ]
semantic_definition equipQuality (quality : Predicate) (cost : Cost) : Ability :=
  keywordQualityCosting (.named "equip") (.inCaller quality) (.inCaller cost)
semantic_definition escalate (cost : Cost) : Ability :=
  .keyword (.named "escalate") [.cost (.inCaller cost)]
    [static (.modeChoice (.extraModesCost (.inCaller cost)))]
semantic_definition escape (cost : Cost) : Ability :=
  .keyword (.named "escape") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard) (payment := .payingInstead (.inCaller cost)))) ]
semantic_definition eternalize (cost : Cost) : Ability :=
  .keyword (.named "eternalize") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create 1 (.copyOf this_ [ .edits
                  [ .colors .sets (.some [.black]),
                    .stat .sets .power 4,
                    .stat .sets .toughness 4,
                    .manaCost none,
                    .typeLine .adds { subtypes := [zombie] } ] ]) [])
        asSorcery ]
semantic_definition evoke (cost : Cost) : Ability :=
  .keyword (.named "evoke") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      when (does thisPermanent enter)
        (if_ := keywordCostPaid (.named "evoke") thisPermanent)
        (instruct (controllerOf thisPermanent) (sacrifice thisPermanent)) ]
semantic_definition evolve : Ability :=
  .keyword (.named "evolve") []
    [ whenever (does (a (and [isCreature, youControl])) enter)
        (if_ := some (or
          [ .compareAmt (powerOf (that creature)) .greater (powerOf thisCreature),
            .compareAmt (toughnessOf (that creature)) .greater (toughnessOf thisCreature) ]))
        (putCounters 1 p1p1Counter thisCreature) ]
/-- "Whenever a creature you control attacks alone" [CR#702.83a]: a creature attacks alone when it
is the only creature declared as an attacker [CR#702.83b], so the trigger event is you attacking
with exactly one creature, not a check made again on resolution. -/
semantic_definition exalted : Ability :=
  .keyword (.named "exalted") []
    [ whenever (does you (attackWith (counted (exactly 1) (and [isCreature, youControl]))))
        (get (that creature) (.up 1) (.up 1) untilEndOfTurn) ]
semantic_definition exhaust (ability : Ability) : Ability :=
  .keyword (.named "exhaust") [.ability (.inCaller ability)]
    [.qualified (.inCaller ability) none oncePerObject none none]
semantic_definition exploit : Ability :=
  .keyword (.named "exploit") []
    [ when (does thisCreature enter)
        (may (sacrifice (a isCreature))) ]
semantic_definition extort : Ability :=
  .keyword (.named "extort") []
    [ whenever (does you (cast (a isSpell)))
        (may (pay (.mana [hybridPip .white .black]) .once)
          (sequentially [instruct eachOpponent (loseLife 1), gainLife .thatMuch])) ]
semantic_definition fabricate (amount : Amount) : Ability :=
  .keyword (.named "fabricate") [.number (.inCaller amount)]
    [ when (does thisPermanent enter)
        (mayOrElse (putCounters (.inCaller amount) p1p1Counter thisPermanent)
          (createToken (artifactCreatureToken 1 1 [] [servo]) (.inCaller amount))) ]
semantic_definition fading (amount : Amount) : Ability :=
  .keyword (.named "fading") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) fadeCounter),
      at_ (beginningOfYour .upkeep)
        (doIfDone (removeCounters (oneThrough 1) fadeCounter thisPermanent) none
          (sacrifice thisPermanent)) ]
/-- "Fear": "This creature can't be blocked except by artifact creatures and/or black
creatures." [CR#702.36b] -/
semantic_definition fear : Ability :=
  .keyword (.named "fear") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, andOr [isArtifact, hasColor .black]])))) ]
semantic_definition firebending (amount : Amount) : Ability :=
  .keyword (.named "firebending") [.number (.inCaller amount)]
    [ whenever (does thisCreature (.attack none))
        (sequentially
          [ .addMana (.inCaller amount) (.runs [[ColorOrColorless.of .red]]) [],
            establishThroughout (.manaRetention .you .thisMana) untilEndOfCombat ]) ]
semantic_definition firstStrike : Ability := keyword (.named "firstStrike")
semantic_definition flanking : Ability :=
  .keyword (.named "flanking") []
    [ whenever
        (does (.by (a (and [isCreature, not (.hasKeyword (.the (.named "flanking")))])))
          (.block thisCreature))
        (get (that creature) (.down 1) (.down 1) untilEndOfTurn) ]
semantic_definition flash : Ability :=
  .keyword (.named "flash") [] [static (can you (play thisCard) (timing := asInstant))]
semantic_definition flashback (cost : Cost) : Ability :=
  .keyword (.named "flashback") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard) (payment := .payingInstead (.inCaller cost)))),
      static
        (replace (would thisSpell (leave stack))
          (doIf (keywordCostPaid (.named "flashback") thisSpell) (instead (exile thisSpell)))) ]
semantic_definition forMirrodin : Ability :=
  .keyword (.named "forMirrodin") []
    [ when (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 2 2 [.red] [rebel]),
            attachTo thisEquipment thatToken ]) ]
semantic_definition forecast (ability : Ability) : Ability :=
  .keyword (.named "forecast") [.ability (.inCaller ability)]
    [ .qualified (.inCaller ability) (duringPart .upkeep (some (ownerOf this_)))
        oncePerTurn none (some .hand) ]
semantic_definition foretell (cost : Cost) : Ability :=
  .keyword (.named "foretell") [.cost (.inCaller cost)]
    [ static
        (specialAction (duringPart .turn you) (mana [generic 2])
          (sequentially
            [ exileFaceDown thisCard,
              establishThroughout (visibility .lookAt you (objects it))
                (forAsLongAs (is it (in_ exileZone))),
              establishThroughout
                (can you
                  (cast it (from_ := exileZone) (payment := payingInstead (.inCaller cost))))
                onALaterTurn ])) ]
semantic_definition fortify (cost : Cost) : Ability :=
  .keyword (.named "fortify") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (and [isLand, youControl])))
        asSorcery ]
semantic_definition freerunning (cost : Cost) : Ability :=
  keywordCosting (.named "freerunning") (.inCaller cost)
semantic_definition frenzy (amount : Amount) : Ability :=
  keywordNumber (.named "frenzy") (.inCaller amount)
semantic_definition fuse : Ability := keyword (.named "fuse")
semantic_definition gift (promise : Action) : Ability :=
  .keyword (.named "gift") []
    [ static (.addedCost (.perform (choose anOpponent)) true .once),
      when (does thisPermanent enter) (if_ := is this_ isPermanentType)
        (doIf (keywordCostPaid (.named "gift") this_)
          (instruct (the isChosenPlayer) (.inCaller promise))
          none),
      spell
        (doIf (is this_ isInstantOrSorcery)
          (doIf (keywordCostPaid (.named "gift") this_)
            (instruct (the isChosenPlayer) (.inCaller promise))
            none)
          none) ]
/-! The licensed phrasings of gift: its templating is not normative ("Gift a card", "Gift a
tapped Fish"), so each is a Template over `gift` [CR#702.174a,702.174d..702.174i]. -/

semantic_template giftACard : Ability := gift (draw 1)
semantic_template giftAFood : Ability := gift (createToken foodToken)
semantic_template giftATappedFish : Ability :=
  gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
semantic_template giftATreasure : Ability := gift (createToken treasureToken)
semantic_template giftAnExtraTurn : Ability := gift (extraTurn 1)
semantic_template giftAnOctopus : Ability :=
  gift (createToken (creatureToken 8 8 [.blue] [octopus]))
semantic_definition graft (amount : Amount) : Ability :=
  .keyword (.named "graft") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) p1p1Counter),
      whenever
        (does (a (and [isCreature, isOther])) enter)
        (if_ := is thisPermanent (.hasCounters (some p1p1Counter)))
        (may (moveCounters 1 thisPermanent (that creature) p1p1Counter)) ]
semantic_definition gravestorm : Ability :=
  .keyword (.named "gravestorm") []
    [ when (does you (cast thisSpell))
        (sequentially
          [ copyOnStack thisSpell
              (eventCount die (a isPermanent) soFarThisTurn),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition harmonize (cost : Cost) : Ability :=
  .keyword (.named "harmonize") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard)
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform
                    (tap
                      (counted (upTo 1) (and [isUntapped, and [isCreature, youControl]]))) ])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (.generic (.down (powerOf (the (and [was (tap which), isCreature])))) none))
          (keywordCostPaid (.named "harmonize") thisSpell)),
      static
        (replace (would thisSpell (leave stack))
          (doIf (keywordCostPaid (.named "harmonize") thisSpell) (instead (exile thisSpell)))) ]
semantic_definition haste : Ability := keyword (.named "haste")
/-- "<subject> gains haste [until …]" -/
semantic_template gainHaste (subject : NounPhrase) (duration : Option Duration := none) : Action :=
  gain subject haste duration
semantic_definition haunt : Ability :=
  .keyword (.named "haunt") []
    [ when (does thisPermanent die)
        (if_ := is this_ isPermanentType)
        (sequentially
          [ exile it,
            .relate it haunting (target isCreature) ]),
      when (does .any (.move thisSpell stack graveyard .departure none []))
        (if_ := is this_ isInstantOrSorcery)
        (sequentially
          [ exile it,
            .relate it haunting (target isCreature) ]) ]
semantic_definition hexproof : Ability :=
  .keyword (.named "hexproof") []
    [static (cant (.by (allOf (and [.in_ stack, opponentsControl]))) (.target thisPermanent))]
/-- "Hexproof from [quality]": "This permanent can't be the target of [quality] spells your
opponents control or abilities your opponents control from [quality] sources." It is a hexproof
ability, so it is written as hexproof with a quality (`Definition`, [CR#702.11d]). -/
semantic_definition hexproofFrom (quality : Predicate) : Ability :=
  .keyword (.named "hexproof") [.quality (.inCaller quality)]
    [ static
        (cant
          (by_
            (allOf
              (and
                [ or
                    [ and [isSpell, .inCaller quality],
                      abilityOf (allOf (and [isSource, .inCaller quality])) ],
                  opponentsControl ])))
          (target thisPermanent)) ]
semantic_definition hideaway (amount : Amount) : Ability :=
  let exiledCard : NounPhrase := the (and [was (exile which), isCard])
  .keyword (.named "hideaway") [.number (.inCaller amount)]
    [ when (does thisPermanent enter)
        (sequentially
          [ lookAt (topSlice (.inCaller amount)),
            exileFaceDown (someOf (oneThrough 1) them),
            .move (theRest .object) .anyLocation onBottom .arrival none
              [.inOrder .randomOrder],
            establishThroughout (visibility .lookAt you (objects exiledCard))
              (forAsLongAs (is exiledCard (in_ exileZone))) ]) ]
semantic_definition horsemanship : Ability :=
  .keyword (.named "horsemanship") []
    [ static
        (cant (.by (allOf (and [isCreature, not (.hasKeyword (.the (.named "horsemanship")))])))
          (.block thisCreature)) ]
semantic_definition impending (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "impending") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (replace (would thisPermanent enter)
          (doIf (keywordCostPaid (.named "impending") thisPermanent)
            (withRiders [.withCounters (.inCaller amount) timeCounter .fresh]))),
      static
        (onlyWhile
          (.characteristicChange thisPermanent [.typeLine .loses { types := some [.creature] }])
          (and
            [ keywordCostPaid (.named "impending") thisPermanent,
              is thisPermanent (.hasCounters timeCounter) ])),
      at_ (beginningOfYour .endStep)
        (if_ := some (and
          [ keywordCostPaid (.named "impending") thisPermanent,
            is thisPermanent (.hasCounters timeCounter) ]))
        (removeCounters (oneThrough 1) timeCounter thisPermanent) ]
semantic_definition improvise : Ability :=
  .keyword (.named "improvise") []
    [ static
        (.paySubstitute thisSpell
          ⟨ tap (a (and [isUntapped, isArtifact, youControl])),
            [.generic] ⟩) ]
semantic_definition increment : Ability :=
  .keyword (.named "increment") []
    [ whenever (does you (cast (a isSpell)))
        (if_ := some (and
          [ is thisPermanent isCreature,
            or
              [ .compareAmt (manaValueSpentToCast (that .spell)) .greater (powerOf thisCreature),
                .compareAmt (manaValueSpentToCast (that .spell)) .greater
                  (toughnessOf thisCreature) ] ]))
        (putCounters 1 p1p1Counter thisCreature) ]
semantic_definition indestructible : Ability :=
  .keyword (.named "indestructible") []
    [ static
        (cant .any
          (destroy thisPermanent)) ]
semantic_definition infect : Ability := keyword (.named "infect")
semantic_definition infinity (ability : Ability) : Ability :=
  .keyword (.named "infinity") [.ability (.inCaller ability)]
    [ static
        (onlyWhile (abilityGrant thisPermanent ability)
          (is thisPermanent (hasDesignation harnessed))) ]
semantic_definition ingest : Ability :=
  .keyword (.named "ingest") []
    [ whenever (does thisCreature (.dealDamage .anyAmount (a .anyPlayer) (some .combat)))
        (instruct they
          (exile (.slice .top 1 (libraryOf they)))) ]
/-- "Intimidate": "This creature can't be blocked except by artifact creatures and/or creatures
that share a color with it." [CR#702.13b] -/
semantic_definition intimidate : Ability :=
  .keyword (.named "intimidate") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          andOr [isArtifact, shares .color .share thisCreature]])))) ]
semantic_definition jobSelect : Ability :=
  .keyword (.named "jobSelect") []
    [ when (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 1 1 [] [hero]),
            attachTo thisEquipment thatToken ]) ]
semantic_definition jumpStart : Ability := keyword (.named "jumpStart")
semantic_definition kicker (cost : Cost) : Ability :=
  .keyword (.named "kicker") [.cost (.inCaller cost)]
    [static (.addedCost (.inCaller cost) true .once)]
semantic_definition landwalk (quality : Predicate) : Ability :=
  .keyword (.named "landwalk") [.quality (.inCaller quality)]
    [ static
        (onlyWhile (cant .any (.block thisCreature))
          (exists_ (and [isLand, .inCaller quality, controlledBy defendingPlayer]))) ]
semantic_definition lifelink : Ability := keyword (.named "lifelink")
semantic_definition livingMetal : Ability :=
  .keyword (.named "livingMetal") []
    [ static
        (onlyWhile
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          (.duringPart .turn (some .you))) ]
semantic_definition livingWeapon : Ability :=
  .keyword (.named "livingWeapon") []
    [ when (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 0 0 [.black] [phyrexian, germ]),
            attachTo thisEquipment thatToken ]) ]
semantic_definition madness (cost : Cost) : Ability :=
  .keyword (.named "madness") [.cost (.inCaller cost)]
    [ static
        (replace (would (a isPlayer) (discard thisCard))
          (instead
            (enact (.action "discard") (.move thisCard hand exileZone .arrival none [])))),
      when
        (.causes (.event (does any (discard thisCard)))
          (does any (.move thisCard hand exileZone .arrival none [])))
        (instruct (ownerOf thisCard)
          (mayOrElse
            (establish
              (can theActor
                (cast thisCard (from_ := exileZone)
                  (payment := .payingInstead (.inCaller cost)))))
            (putInto thisCard graveyard))) ]
semantic_definition maxSpeed (ability : Ability) : Ability :=
  .keyword (.named "maxSpeed") [.ability (.inCaller ability)]
    [static (onlyWhile (abilityGrant this_ ability) (.compareAmt (speedOf .you) .eq 4))]
semantic_definition mayhem (cost : Cost) : Ability :=
  .keyword (.named "mayhem") [.cost (.inCaller cost)]
    [ static
        (onlyWhile
          (can you
            (cast thisCard (from_ := yourGraveyard)
              (payment := .payingInstead (.inCaller cost))))
          (did you (discard thisCard) soFarThisTurn)) ]
semantic_definition megamorph (cost : Cost) : Ability :=
  .keyword (.named "megamorph") [.cost (.inCaller cost)]
    [ static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      static
        (.specialAction none (.inCaller cost)
          (sequentially [turnFaceUp this_, putCounters 1 p1p1Counter this_])) ]
semantic_definition melee : Ability :=
  .keyword (.named "melee") []
    [ whenever (does thisCreature (.attack none))
        (get thisCreature
          (.up
            (times 1
              (eventCount (.attackWith (a isCreature) (some .which)) you thisCombat)))
          (.up
            (times 1
              (eventCount (.attackWith (a isCreature) (some .which)) you thisCombat)))
          untilEndOfTurn) ]
/-- "Menace": "This creature can't be blocked except by two or more creatures."
[CR#702.111b] -/
semantic_definition menace : Ability :=
  .keyword (.named "menace") []
    [ static (cant any (block thisCreature)
        (except := byObjects (counted (orMore 2) isCreature))) ]
semantic_definition mentor : Ability :=
  .keyword (.named "mentor") []
    [ whenever (does thisCreature (.attack none))
        (putCounters 1 p1p1Counter
          (target (and [isCreature, isAttacking, hasPower .less (powerOf thisCreature)]))) ]
semantic_definition miracle (cost : Cost) : Ability :=
  keywordCosting (.named "miracle") (.inCaller cost)
semantic_definition mobilize (amount : Amount) : Ability :=
  .keyword (.named "mobilize") [.number (.inCaller amount)]
    [ whenever (does thisCreature (.attack none))
        (sequentially
          [ createTappedAttacking (.inCaller amount) (creatureToken 1 1 [.red] [warrior]),
            atNext .endStep
              (sacrifice thoseTokens) ]) ]
semantic_definition moreThanMeetsTheEye (cost : Cost) : Ability :=
  keywordCosting (.named "moreThanMeetsTheEye") (.inCaller cost)
semantic_definition morph (cost : Cost) : Ability :=
  .keyword (.named "morph") [.cost (.inCaller cost)]
    [ static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      static (.specialAction none (.inCaller cost) (turnFaceUp this_)) ]
semantic_definition multikicker (cost : Cost) : Ability :=
  keywordCosting (.named "multikicker") (.inCaller cost)
semantic_definition mutate (cost : Cost) : Ability :=
  .keyword (.named "mutate") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.addsTarget thisSpell
          (and
            [isCreature, not (.hasSubtype human), ownedBy (ownerOf thisSpell)])),
      static
        (.replace (would thisSpell enter)
          (doIf (keywordCostPaid (.named "mutate") thisSpell)
            (instead
              (chooseOne
                (.merge thisSpell (the (was (target which))) .top)
                (.merge thisSpell (the (was (target which))) .bottom))))
          none .repeatedly none) ]
semantic_definition myriad : Ability :=
  .keyword (.named "myriad") []
    [ whenever (does thisCreature (.attack none))
        (sequentially
          [ .doForEach (each (and [isOpponent, .otherThan defendingPlayer]))
              (may
                (createTokenCopy thisCreature
                  [ .entersAs .tapped,
                    .entersAttacking
                      (some (or [they, a (and [isPlaneswalker, controlledBy they])])) ])),
            doIf (.compareAmt (.countOf thoseTokens) .atLeast 1)
              (atNext .endOfCombat (exile thoseTokens))
              none ]) ]
semantic_definition nightbound : Ability :=
  .keyword (.named "nightbound") []
    [ static
        (.asEvent (gameBecomes day)
          (doIf (is thisPermanent isTransformed)
            (transform thisPermanent)
            none)),
      static
        (cant any (transform thisPermanent) (except := some (.byKeyword (.named "nightbound")))) ]
semantic_definition ninjutsu (cost : Cost) : Ability :=
  .keyword (.named "ninjutsu") [.cost (.inCaller cost)]
    [ activated
        (.compound
          [ .inCaller cost,
            .perform (.expose .reveal (.cards this_)),
            .perform (returnToHand (a (and [isCreature, isAttacking, isUnblocked, youControl]))) ])
        (putOntoBattlefieldTappedAttacking this_ yourHand) ]
semantic_definition offering (quality : Predicate) : Ability :=
  .keyword (.named "offering") [.quality (.inCaller quality)]
    [ static
        (.addedCost
          (.perform (sacrifice (a (and [.inCaller quality, isPermanent]))))
          true .once),
      static
        (onlyWhile
          (reducedBySymbols thisSpell
            (.manaCostOf (the (and [was (sacrifice which), isPermanent]))))
          (keywordCostPaid (.named "offering") thisSpell)),
      static
        (onlyWhile (can you (cast this_) asInstant)
          (keywordCostPaid (.named "offering") this_)) ]
semantic_definition offspring (cost : Cost) : Ability :=
  .keyword (.named "offspring") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .once),
      when (does thisPermanent enter)
        (if_ := keywordCostPaid (.named "offspring") thisPermanent)
        (create 1
          (.copyOf thisPermanent
            [.edits [.stat .sets .power 1, .stat .sets .toughness 1]])) ]
semantic_definition outlast (cost : Cost) : Ability :=
  .keyword (.named "outlast") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.compound [.inCaller cost, .tapSymbol])
        (putCounters 1 p1p1Counter thisCreature)
        asSorcery ]
semantic_definition overload (cost : Cost) : Ability :=
  .keyword (.named "overload") [.cost (.inCaller cost)]
    [ static (can you (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (onlyWhile (.textChange thisSpell .targetToEach)
          (keywordCostPaid (.named "overload") thisSpell)) ]
semantic_definition paradigm : Ability := keyword (.named "paradigm")
semantic_definition partner : Ability := keyword (.named "partner")
semantic_definition partnerCharacterSelect : Ability := keyword (.named "partnerCharacterSelect")
semantic_definition partnerFatherAndSon : Ability := keyword (.named "partnerFatherAndSon")
semantic_definition partnerFriendsForever : Ability := keyword (.named "partnerFriendsForever")
semantic_definition partnerSurvivors : Ability := keyword (.named "partnerSurvivors")
semantic_definition partnerWith (quality : Predicate) : Ability :=
  keywordQuality (.named "partnerWith") (.inCaller quality)
semantic_definition persist : Ability :=
  .keyword (.named "persist") []
    [ when (does thisPermanent die)
        (if_ := some (not (is thisPermanent (.hasCounters (some m1m1Counter)))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters 1 m1m1Counter .fresh]) ]
semantic_definition phasing : Ability := keyword (.named "phasing")
semantic_definition plot (cost : Cost) : Ability :=
  .keyword (.named "plot") [.cost (.inCaller cost)]
    [ static
        (specialAction asSorcery (.inCaller cost)
          (sequentially
            [ exile thisCard,
              establishThroughout
                (can you (cast it (from_ := exileZone) (payment := withoutPaying))
                  (timing := asSorcery))
                onALaterTurn ])) ]
semantic_definition poisonous (amount : Amount) : Ability :=
  .keyword (.named "poisonous") [.number (.inCaller amount)]
    [ whenever (does thisCreature (.dealDamage .anyAmount (a .anyPlayer) (some .combat)))
        (putCounters (.inCaller amount) poison they) ]
semantic_definition powerUp (ability : Ability) : Ability :=
  .keyword (.named "powerUp") [.ability (.inCaller ability)]
    [ .qualified (.inCaller ability) none oncePerObject none none,
      static
        (onlyWhile (reducedBySymbols thisAbility (.manaCostOf thisPermanent))
          (did thisPermanent enter soFarThisTurn)) ]
semantic_definition protection (quality : Predicate) : Ability :=
  .keyword (.named "protection") [.quality (.inCaller quality)]
    [ static
        (cant
          (.by
            (allOf
              (or
                [ and [isSpell, .inCaller quality],
                  .abilityOf (allOf (and [.isSource, .inCaller quality])) ])))
          (.target thisPermanent)),
      static
        (cant any (attachTo (allOf (and [.hasSubtype aura, .inCaller quality])) thisPermanent)),
      static
        (cant any
          (attachTo
            (allOf
              (and [or [.hasSubtype equipment, .hasSubtype fortification], .inCaller quality]))
            thisPermanent)),
      static
        (.damageRule (.dealtBy (allOf (and [.isSource, .inCaller quality])))
          (.toRecipient thisPermanent)
          (.prevent .all none) .repeatedly),
      static (cant (.by (allOf (and [isCreature, .inCaller quality]))) (.block thisCreature)) ]
set_option linter.unusedVariables false in
semantic_definition prototype (cost : Cost) (power toughness : Amount) : Ability :=
  keyword (.named "prototype")
semantic_definition provoke : Ability :=
  .keyword (.named "provoke") []
    [ whenever (does thisCreature (.attack none))
        (may
          (establishThroughout
            (must (target (and [isCreature, controlledBy defendingPlayer]))
              (.block thisCreature))
            untilEndOfCombat)
          (untap (that creature))) ]
semantic_definition prowess : Ability :=
  .keyword (.named "prowess") []
    [ whenever
        (does you (cast (a (and [.in_ stack, non isCreature]))))
        (get thisCreature (.up 1) (.up 1) untilEndOfTurn) ]
/-- "Prowl <cost>": cast for <cost> if a player was dealt combat damage this turn by a source
you controlled that shared a creature type with this spell [CR#702.76a]. -/
semantic_definition prowl (cost : Cost) : Ability :=
  .keyword (.named "prowl") [.cost (.inCaller cost)]
    [ static
        (onlyWhile (can you (cast this_ (payment := .payingInstead (.inCaller cost))))
          (did (a (and [.shares (.subtype .creature) .share thisSpell, youControl]))
            (.dealDamage .anyAmount (a isPlayer) (some .combat)) soFarThisTurn)) ]
semantic_definition rampage (amount : Amount) : Ability :=
  .keyword (.named "rampage") [.number (.inCaller amount)]
    [ whenever (does .any (.block thisCreature))
        (get thisCreature
          (.up
            (times (.inCaller amount)
              (minus (countOf (and [isCreature, .inCombat .blockerOf thisCreature]))
                1)))
          (.up
            (times (.inCaller amount)
              (minus (countOf (and [isCreature, .inCombat .blockerOf thisCreature]))
                1)))
          untilEndOfTurn) ]
semantic_definition ravenous : Ability :=
  .keyword (.named "ravenous") []
    [ static (entersWithCounters thisPermanent (.letter .x) p1p1Counter),
      when (does thisPermanent enter)
        (if_ := some (.compareAmt (.letter .x) .atLeast 5))
        (draw 1) ]
semantic_definition readAhead : Ability :=
  .keyword (.named "readAhead") []
    [ static
        (onlyWhile
          (cant
            (.actor
              (allOf
                (and
                  [ chapterAbilityOf thisSaga,
                    not (.compare [.chapter] .eq (countersOn loreCounter thisSaga)) ])))
            .trigger)
          (did thisSaga enter soFarThisTurn)) ]
semantic_definition rebound : Ability :=
  .keyword (.named "rebound") []
    [ static
        (replace (.would .any (.move thisSpell stack graveyard .departure none []))
          (doIf (is thisSpell (.castFrom yourHand))
            (instead
              (sequentially
                [ exile thisSpell,
                  delay (beginningOf .upkeep (some .you) (.next none))
                    (may (cast this_ (payment := withoutPaying))) ])))) ]
semantic_definition reconfigure (cost : Cost) : Ability :=
  .keyword (.named "reconfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (and [isCreature, isOther, youControl])))
        asSorcery,
      .activated (.inCaller cost)
        (enact (.action "attach") (.attachment .unattached thisPermanent none))
        asSorcery none
        (is thisPermanent (.attachment .attachment none (some (a isCreature))))
        none ]
semantic_definition recover (cost : Cost) : Ability :=
  .keyword (.named "recover") [.cost (.inCaller cost)]
    [ when
        (does .any (.move (a isCreature) battlefield yourGraveyard .departure none []))
        (may (pay (.inCaller cost) .once) (returnToHand this_ yourGraveyard)
          (exile this_)) ]
semantic_definition reinforce (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "reinforce") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ activated
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (putCounters (.inCaller amount) p1p1Counter (target isCreature)) ]
semantic_definition replicate (cost : Cost) : Ability :=
  .keyword (.named "replicate") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid (.named "replicate") thisSpell)
        (sequentially
          [ copyOnStack thisSpell (timesPaid (.byKeyword (.named "replicate")) thisSpell),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition retrace : Ability :=
  .keyword (.named "retrace") []
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard)
            (payment := .payingInstead
              (.compound
                [.itsManaCost, .perform (discard (a (and [.isCard, isLand, .in_ yourHand])))])))) ]
semantic_definition riot : Ability :=
  .keyword (.named "riot") []
    [ static
        (replace (would thisPermanent enter)
          (also
            (mayOrElse (putCounters 1 p1p1Counter thisPermanent)
              (gainHaste thisPermanent)))) ]
semantic_definition saddle (amount : Amount) : Ability :=
  .keyword (.named "saddle") [.number (.inCaller amount)]
    [ activatedOnlyDuring
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast (.inCaller amount)
              (allOf
                (and
                  [ isUntapped,
                    and [isCreature, youControl, isOther] ])))))
        (gainDesignation thisPermanent saddled untilEndOfTurn)
        asSorcery ]
semantic_definition scavenge (cost : Cost) : Ability :=
  .keyword (.named "scavenge") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (putCounters (powerOf (the (and [isCard, did you (exile which)])))
          p1p1Counter (target isCreature))
        asSorcery ]
semantic_definition shadow : Ability :=
  .keyword (.named "shadow") []
    [ static
        (cant (.by (allOf (and [isCreature, not (.hasKeyword (.the (.named "shadow")))])))
          (.block thisCreature)),
      static
        (cant (.by (allOf (and [isCreature, .hasKeyword (.the (.named "shadow"))])))
          (.block (allOf (and [isCreature, not (.hasKeyword (.the (.named "shadow")))])))) ]
semantic_definition shroud : Ability :=
  .keyword (.named "shroud") [] [static (cant .any (.target thisPermanent))]
semantic_definition skulk : Ability :=
  .keyword (.named "skulk") []
    [ static
        (cant (.by (allOf (and [isCreature, hasPower .greater (powerOf thisCreature)])))
          (.block thisCreature)) ]
semantic_definition sneak (cost : Cost) : Ability :=
  .keyword (.named "sneak") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast thisSpell
            (payment := payingInstead
              (compound
                [ .inCaller cost,
                  perform (returnToHand (a (and [isCreature, isUnblocked, youControl]))) ])))
          (timing := some (and [asInstant, duringPart .declareBlockers (some you)]))) ]
semantic_definition solved (ability : Ability) : Ability :=
  .keyword (.named "solved") [.ability (.inCaller ability)]
    [ .qualified (.inCaller ability) none none
        (is thisCase (hasDesignation DesignationLabel.solved))
        none ]
semantic_definition soulbond : Ability :=
  .keyword (.named "soulbond") []
    [ when (does thisCreature enter)
        (if_ := some (and
          [ is thisCreature (and [youControl, not (.related paired none)]),
            exists_
              (and
                [ and [isCreature, youControl, isOther],
                  not (.related paired none) ]) ]))
        (may
          (.relate thisCreature paired
            (a
              (and
                [ and [isCreature, youControl, isOther],
                  not (.related paired none) ])))),
      whenever
        (does (a (and [isCreature, youControl, isOther])) enter)
        (if_ := some (and
          [ is thisCreature (and [youControl, not (.related paired none)]),
            is (that creature) (and [youControl, not (.related paired none)]) ]))
        (may (.relate (that creature) paired thisCreature)) ]
semantic_definition soulshift (amount : Amount) : Ability :=
  .keyword (.named "soulshift") [.number (.inCaller amount)]
    [ when (does thisPermanent die)
        (may
          (returnToHand
            (target
              (and
                [ .isCard,
                  .hasSubtype spirit,
                  hasManaValue .atMost (.inCaller amount),
                  .in_ yourGraveyard ]))
            yourGraveyard)) ]
semantic_definition spectacle (cost : Cost) : Ability :=
  .keyword (.named "spectacle") [.cost (.inCaller cost)]
    [ static
        (onlyWhile (can you (cast this_ (payment := .payingInstead (.inCaller cost))))
          (did anOpponent (loseLife .anyAmount) soFarThisTurn)) ]
/-- "Splice onto [quality] [cost]" [CR#702.47a]. Named in its sort, `Ability.splice`, as a
declaration whose bare name is taken is (`Subtype.army`, `DesignationLabel.solved`): `splice` is
the authoring syntax's word for a supplied body. -/
semantic_definition Ability.splice (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "splice") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ static
        (.replace
          (.would you
            (cast (a (and [isSpell, .inCaller quality]))))
          (instead
            (may (.expose .reveal (.cards this_))
              (some
                (.establish
                  (.conjunction none
                    [ .addedCost (.inCaller cost) false .once,
                      .textChange (that .spell) (.appendTextOf this_) ])
                  none))))
          none .repeatedly none) ]
semantic_definition splitSecond : Ability :=
  .keyword (.named "splitSecond") []
    [ static
        (onlyWhile
          (.conjunction none
            [ cant (bare isPlayer) (cast (allOf anotherSpell)),
              cant (bare isPlayer)
                (activate (allOf (and [.abilityHead .anyActivated, non isManaAbility]))) ])
          (is thisSpell (.in_ stack))) ]
semantic_definition spree : Ability :=
  .keyword (.named "spree") [] [static (.modeChoice (.setQuantity (orMore 1)))]
semantic_definition squad (cost : Cost) : Ability :=
  .keyword (.named "squad") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      when (does thisCreature enter)
        (if_ := keywordCostPaid (.named "squad") thisCreature)
        (create (timesPaid (.byKeyword (.named "squad")) thisCreature) (.copyOf thisCreature [])) ]
semantic_definition startYourEngines : Ability := keyword (.named "startYourEngines")
semantic_definition station : Ability :=
  .keyword (.named "station") []
    [ activatedOnlyDuring
        (.perform (tap (a (and [isCreature, isOther, isUntapped, youControl]))))
        (putCounters (powerOf (the (and [was (tap which), isCreature])))
          chargeCounter thisPermanent)
        asSorcery ]
semantic_definition storied : Ability :=
  .keyword (.named "storied") []
    [ static
        (.whileHolds
          (and
            [ .compareAmt
                (countOf
                  (and
                    [ isPermanent,
                      youControl,
                      andOr
                        [ isArtifact,
                          .hasSubtype saga,
                          .hasSupertype .legendary ] ]))
                .atLeast 3,
              not (is .you (hasDesignation enduringStory)) ])
          (gainDesignation .you enduringStory restOfGame)) ]
semantic_definition sunburst : Ability :=
  .keyword (.named "sunburst") []
    [ static
        (replace (would thisPermanent enter)
          (doIf (is thisPermanent isCreature)
            (withRiders
              [.withCounters (colorsSpentToCast thisSpell) p1p1Counter .fresh]))),
      static
        (replace (would thisPermanent enter)
          (doIf (not (is thisPermanent isCreature))
            (withRiders
              [.withCounters (colorsSpentToCast thisSpell) chargeCounter .fresh]))) ]
semantic_definition surge (cost : Cost) : Ability :=
  .keyword (.named "surge") [.cost (.inCaller cost)]
    [ static
        (onlyWhile (can you (cast this_ (payment := .payingInstead (.inCaller cost))))
          (did (actor (or [.you, a isTeammate])) (cast (a anotherSpell)) soFarThisTurn)) ]
semantic_definition suspend (amount : Amount) (cost : Cost) : Ability :=
  keywordNumberCosting (.named "suspend") (.inCaller amount) (.inCaller cost)
semantic_definition teamwork (amount : Amount) : Ability :=
  .keyword (.named "teamwork") [.number (.inCaller amount)]
    [ static
        (.addedCost
          (.perform
            (tap
              (withTotal (.stat .power) .atLeast (.inCaller amount)
                (allOf (and [isCreature, youControl])))))
          true .once) ]
semantic_definition tiered : Ability :=
  .keyword (.named "tiered") [] [static (.modeChoice (.setQuantity (oneThrough 1)))]
semantic_definition toxic (amount : Amount) : Ability :=
  keywordNumber (.named "toxic") (.inCaller amount)
semantic_definition training : Ability :=
  .keyword (.named "training") []
    [ whenever
        (does
          (.actor
            (and
              [ thisCreature,
                counted (orMore 1)
                  (and
                    [ isCreature,
                      isOther,
                      hasPower .greater (powerOf thisCreature) ]) ]))
          (.attack none))
        (putCounters 1 p1p1Counter thisCreature) ]
semantic_definition trample : Ability := keyword (.named "trample")
semantic_definition transfigure (cost : Cost) : Ability :=
  .keyword (.named "transfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (sacrifice thisPermanent) ])
        (sequentially
          [ .search yourLibrary
              (a (and
                [.isCard, isCreature, hasManaValue .eq (.statOf (.stat .manaValue) thisPermanent)])),
            putOntoBattlefield (that .card),
            .shuffle (libraryOf .actor) ])
        asSorcery ]
semantic_definition transmute (cost : Cost) : Ability :=
  .keyword (.named "transmute") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (sequentially
          [ .search yourLibrary
              (a (and
                [ .isCard,
                  hasManaValue .eq
                    (.statOf (.stat .manaValue) (the (and [was (discard which), isCard]))) ])),
            .expose .reveal (.cards (that .card)),
            putInto it hand,
            .shuffle (libraryOf .actor) ])
        asSorcery ]
semantic_definition tribute (amount : Amount) : Ability :=
  .keyword (.named "tribute") [.number (.inCaller amount)]
    [ static
        (replace (would thisCreature enter)
          (also
            (sequentially
              [ choose anOpponent,
                instruct (the isChosenPlayer)
                  (may (putCounters (.inCaller amount) p1p1Counter thisCreature)) ]))) ]
/-- "<Type>cycling <cost>": a cycling ability qualified by the card it searches for, labelled
cycling, since typecycling abilities are cycling abilities [CR#702.29e,702.29f]. -/
semantic_definition typecycling (quality : Predicate) (cost : Cost) : Ability :=
  keywordQualityCosting (.named "cycling") (.inCaller quality) (.inCaller cost)
semantic_definition umbraArmor : Ability :=
  .keyword (.named "umbraArmor") []
    [ static
        (replace (would any (destroy (enchanted .permanent)))
          (instead (sequentially [clearDamage (enchanted .permanent), destroy thisAura]))) ]
semantic_definition undaunted : Ability :=
  .keyword (.named "undaunted") []
    [static (.costShift thisSpell (.generic (.down (forEach 1 isOpponent)) none))]
semantic_definition undying : Ability :=
  .keyword (.named "undying") []
    [ when (does thisPermanent die)
        (if_ := some (not (is thisPermanent (.hasCounters (some p1p1Counter)))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters 1 p1p1Counter .fresh]) ]
semantic_definition unearth (cost : Cost) : Ability :=
  .keyword (.named "unearth") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (sequentially
          [ returnToBattlefield thisCard (from_ := yourGraveyard),
            gainHaste thisPermanent,
            atNext .endStep
              (exile thisPermanent),
            replaceEvent (would thisPermanent leave)
              (instead (exile thisPermanent))
              none ])
        asSorcery ]
semantic_definition unleash : Ability :=
  .keyword (.named "unleash") []
    [ static
        (replace (would thisPermanent enter)
          (also (may (putCounters 1 p1p1Counter thisPermanent)))),
      static
        (onlyWhile (cant thisPermanent (.block .any))
          (is thisPermanent (.hasCounters p1p1Counter))) ]
semantic_definition vanishing (amount : Amount) : Ability :=
  .keyword (.named "vanishing") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) timeCounter),
      at_ (beginningOfYour .upkeep)
        (if_ := is thisPermanent (.hasCounters (some timeCounter)))
        (removeCounters (oneThrough 1) timeCounter thisPermanent),
      when (lastCounterRemoved timeCounter thisPermanent)
        (sacrifice thisPermanent) ]
semantic_definition vigilance : Ability := keyword (.named "vigilance")
semantic_definition ward (cost : Cost) : Ability :=
  .keyword (.named "ward") [.cost (.inCaller cost)]
    [ whenever (does (.by (a (and [.in_ stack, opponentsControl]))) (.target thisPermanent))
        (instruct (controllerOf (that .stack))
          (mayOrElse (pay (.inCaller cost) .once)
            (instruct .you (counter (that .stack))))) ]
semantic_definition warp (cost : Cost) : Ability :=
  .keyword (.named "warp") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_ (from_ := yourHand) (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does you
            (.pay .itsManaCost .once .paid thisSpell (some (.named "warp"))))
          (atNext .endStep
            (sequentially
              [ exile (.becomes thisSpell),
                establishThroughout
                  (can (ownerOf it)
                    (cast it (from_ := exileZone)))
                  onALaterTurn ]))) ]
semantic_definition webSlinging (cost : Cost) : Ability :=
  .keyword (.named "webSlinging") [.cost (.inCaller cost)]
    [ static
        (can you
          (cast this_ (payment := .payingInstead (.compound
                [ .inCaller cost,
                  .perform (returnToHand (a (and [isCreature, isTapped, youControl]))) ])))) ]
semantic_definition wither : Ability := keyword (.named "wither")

open Lean Elab Term in
/-- `crew`, `saddle` and `devour` name a keyword ability declared here and the action its
definition performs (`Action.crew`, `Action.saddle`, `Action.devour`
[CR#702.122a,702.171a,702.82b]): the expected type picks one, as `Semantics.Templates`
dispatches the words its own sorts share. -/
@[scoped term_elab Lean.Parser.Term.app]
def elabKeywordDispatch : TermElab := Templates.dispatchOnExpectedType
  [ (`crew, [ (``Semantics.Action, `Semantics.Templates.Action.crew),
              (``Semantics.Ability, `Semantics.Definitions.crew) ]),
    (`saddle, [ (``Semantics.Action, `Semantics.Templates.Action.saddle),
                (``Semantics.Ability, `Semantics.Definitions.saddle) ]),
    (`devour, [ (``Semantics.Action, `Semantics.Definitions.Action.devour),
                (``Semantics.Ability, `Semantics.Definitions.devour) ]) ]

end Semantics.Definitions
