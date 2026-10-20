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
        (.putCounters (.lit 1) (.printed levelCounter) thisPermanent .one true) asSorcery ]
/-- "Storm" with its reminder text: "When you cast this spell, copy it for each other spell that
was cast before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_definition storm : Ability :=
  .keyword (.named "storm") []
    [ when (does (.actor .you) (cast thisSpell))
        (sequentially
          [ .copy .fromStack thisSpell
              (eventCount
                (cast (a (and [isSpell, .otherThan thisSpell])))
                (.actor (a .anyPlayer)) earlierThisTurn)
              [],
            may (.chooseNewTargets (.pro (.word .copy) .many)) ]) ]
/-- "Cascade" with its reminder text [CR#702.85a]. -/
semantic_definition cascade : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  .keyword (.named "cascade") []
    [ when (does (.actor .you) (cast thisSpell))
        (sequentially
          [ exile (.run .top (and [non isLand, cheaper]) (libraryOf .you)),
            may (castWithoutPaying (that .card) (some cheaper)),
            .move
              (allFromAmong
                (not (.did ⟨.any, cast .which, .thisWay⟩))
                (theVerbed (.action "exile") .many))
              .anyLocation (some onBottom) .arrival none [(.inOrder .randomOrder)] ]) ]
/-- "Ripple N" with its definition [CR#702.60a]: "When you cast this spell, you may reveal the top
N cards of your library. If you do, you may cast any of those cards with the same name as this
spell without paying their mana costs, then put all revealed cards not cast this way on the bottom
of your library in any order." -/
semantic_definition ripple (amount : Amount) : Ability :=
  .keyword (.named "ripple") [.number (.inCaller amount)]
    [ when (does (.actor .you) (cast thisSpell))
        (may (reveal (.slice .top (.inCaller amount) (libraryOf .you)))
          (some
            (sequentially
              [ castWithoutPaying
                  (.someOf (.counted anyNumber) (some (.shares .cardName .share this_))
                    (those .card))
                  none,
                .move
                  (allFromAmong
                    (not (.did ⟨.any, cast .which, .thisWay⟩))
                    (theVerbed (.action "reveal") .many (some .card)))
                  .anyLocation (some onBottom) .arrival none [.inOrder .anyOrder] ]))) ]
/-- "Renown N" with its reminder text: "When this creature deals combat damage to a player, if it
isn't renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_definition renown (count : Amount) : Ability :=
  .keyword (.named "renown") [.number (.inCaller count)]
    [ triggeredIf (does thisCreature (dealDamage (to := a .anyPlayer) (kind := .combatOnly)))
        (not (matches_ thisCreature (.hasDesignation renowned none)))
        (sequentially
          [ .putCounters (.inCaller count) (.printed p1p1Counter) thisCreature .one true,
            .gainDesignation thisCreature renowned none ]) ]
/-- "Cumulative upkeep [cost]" with its reminder text [CR#702.24a]. -/
semantic_definition cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost (.inCaller cost)]
    [ triggeredIf (.beginningOf .upkeep (some .you) (.each none))
        (matches_ thisPermanent isPermanent)
        (sequentially
          [ .putCounters (.lit 1) (.printed ageCounter) thisPermanent .one true,
            doUnless (sacrifice thisPermanent)
              (.scaled (.inCaller cost) (times (.lit 1) (countersOn ageCounter thisPermanent))) ]) ]
semantic_definition reach : Ability := keyword (.named "reach")
/-- "Flying" with its definition, a static ability: "This creature can't be blocked except by
creatures with flying and/or reach." [CR#702.9b] -/
semantic_definition flying : Ability :=
  .keyword (.named "flying") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          or [hasKeyword (.the (.named "flying")), hasKeyword reach]])))) ]
/-- "Bushido N" with its definition, a triggered ability [CR#702.45a]. -/
semantic_definition bushido (count : Amount) : Ability :=
  .keyword (.named "bushido") [.number (.inCaller count)]
    [ triggeredOr (does thisCreature block) [does any (block thisCreature)]
        (get thisCreature (.up (.inCaller count)) (.up (.inCaller count)) (some untilEndOfTurn)) ]
/-- "Modular N" with its definition, a static ability and a triggered one [CR#702.43a]. -/
semantic_definition modular (count : Amount) : Ability :=
  .keyword (.named "modular") [.number (.inCaller count)]
    [ static (entersWithCounters thisPermanent (.inCaller count) p1p1Counter),
      when (does thisPermanent die)
        (may (.putCounters (countersOn p1p1Counter thisPermanent) (.printed p1p1Counter)
          (target (and [isArtifact, isCreature])) .one true)) ]
/-- "Cycling [cost]" with its definition, an activated ability: "[Cost], Discard this card: Draw a
card." [CR#702.29a] -/
semantic_definition cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost (.inCaller cost)]
    [activated (.compound [.inCaller cost, .perform (discard this_)]) (draw (.lit 1))]

/-! ## Ability words -/

semantic_definition adamant (ability : Ability) : Ability := abilityWord "Adamant" ability
semantic_definition addendum (ability : Ability) : Ability := abilityWord "Addendum" ability
semantic_definition alliance (ability : Ability) : Ability := abilityWord "Alliance" ability
semantic_definition battalion (ability : Ability) : Ability := abilityWord "Battalion" ability
semantic_definition bloodrush (ability : Ability) : Ability := abilityWord "Bloodrush" ability
semantic_definition celebration (ability : Ability) : Ability := abilityWord "Celebration" ability
semantic_definition channel (ability : Ability) : Ability := abilityWord "Channel" ability
semantic_definition chroma (ability : Ability) : Ability := abilityWord "Chroma" ability
semantic_definition cohort (ability : Ability) : Ability := abilityWord "Cohort" ability
semantic_definition constellation (ability : Ability) : Ability :=
  abilityWord "Constellation" ability
semantic_definition converge (ability : Ability) : Ability := abilityWord "Converge" ability
semantic_definition councilsDilemma (ability : Ability) : Ability :=
  abilityWord "Council's dilemma" ability
semantic_definition coven (ability : Ability) : Ability := abilityWord "Coven" ability
semantic_definition delirium (ability : Ability) : Ability := abilityWord "Delirium" ability
semantic_definition descend4 (ability : Ability) : Ability := abilityWord "Descend 4" ability
semantic_definition descend8 (ability : Ability) : Ability := abilityWord "Descend 8" ability
semantic_definition disappear (ability : Ability) : Ability := abilityWord "Disappear" ability
semantic_definition domain (ability : Ability) : Ability := abilityWord "Domain" ability
semantic_definition eerie (ability : Ability) : Ability := abilityWord "Eerie" ability
semantic_definition eminence (ability : Ability) : Ability := abilityWord "Eminence" ability
semantic_definition enrage (ability : Ability) : Ability := abilityWord "Enrage" ability
semantic_definition fatefulHour (ability : Ability) : Ability := abilityWord "Fateful hour" ability
semantic_definition fathomlessDescent (ability : Ability) : Ability :=
  abilityWord "Fathomless descent" ability
semantic_definition ferocious (ability : Ability) : Ability := abilityWord "Ferocious" ability
semantic_definition flurry (ability : Ability) : Ability := abilityWord "Flurry" ability
semantic_definition formidable (ability : Ability) : Ability := abilityWord "Formidable" ability
semantic_definition grandeur (ability : Ability) : Ability := abilityWord "Grandeur" ability
semantic_definition hellbent (ability : Ability) : Ability := abilityWord "Hellbent" ability
semantic_definition heroic (ability : Ability) : Ability := abilityWord "Heroic" ability
semantic_definition imprint (ability : Ability) : Ability := abilityWord "Imprint" ability
semantic_definition infusion (ability : Ability) : Ability := abilityWord "Infusion" ability
semantic_definition inspired (ability : Ability) : Ability := abilityWord "Inspired" ability
semantic_definition joinForces (ability : Ability) : Ability := abilityWord "Join forces" ability
semantic_definition kinship (ability : Ability) : Ability := abilityWord "Kinship" ability
semantic_definition landfall (ability : Ability) : Ability := abilityWord "Landfall" ability
semantic_definition lieutenant (ability : Ability) : Ability := abilityWord "Lieutenant" ability
semantic_definition magecraft (ability : Ability) : Ability := abilityWord "Magecraft" ability
semantic_definition metalcraft (ability : Ability) : Ability := abilityWord "Metalcraft" ability
semantic_definition morbid (ability : Ability) : Ability := abilityWord "Morbid" ability
semantic_definition opus (ability : Ability) : Ability := abilityWord "Opus" ability
semantic_definition packTactics (ability : Ability) : Ability := abilityWord "Pack tactics" ability
semantic_definition paradox (ability : Ability) : Ability := abilityWord "Paradox" ability
semantic_definition parley (ability : Ability) : Ability := abilityWord "Parley" ability
semantic_definition radiance (ability : Ability) : Ability := abilityWord "Radiance" ability
semantic_definition raid (ability : Ability) : Ability := abilityWord "Raid" ability
semantic_definition rally (ability : Ability) : Ability := abilityWord "Rally" ability
semantic_definition renew (ability : Ability) : Ability := abilityWord "Renew" ability
semantic_definition repartee (ability : Ability) : Ability := abilityWord "Repartee" ability
semantic_definition revolt (ability : Ability) : Ability := abilityWord "Revolt" ability
semantic_definition secretCouncil (ability : Ability) : Ability :=
  abilityWord "Secret council" ability
semantic_definition spellMastery (ability : Ability) : Ability :=
  abilityWord "Spell mastery" ability
semantic_definition strive (ability : Ability) : Ability := abilityWord "Strive" ability
semantic_definition survival (ability : Ability) : Ability := abilityWord "Survival" ability
semantic_definition sweep (ability : Ability) : Ability := abilityWord "Sweep" ability
semantic_definition temptingOffer (ability : Ability) : Ability :=
  abilityWord "Tempting offer" ability
semantic_definition threshold (ability : Ability) : Ability := abilityWord "Threshold" ability
semantic_definition undergrowth (ability : Ability) : Ability := abilityWord "Undergrowth" ability
semantic_definition valiant (ability : Ability) : Ability := abilityWord "Valiant" ability
semantic_definition vivid (ability : Ability) : Ability := abilityWord "Vivid" ability
semantic_definition void (ability : Ability) : Ability := abilityWord "Void" ability
semantic_definition willOfTheCouncil (ability : Ability) : Ability :=
  abilityWord "Will of the council" ability

/-! ## Keyword abilities -/

semantic_definition absorb (amount : Amount) : Ability :=
  .keyword (.named "absorb") [.number (.inCaller amount)]
    [ static
        (.damageRule .any .unattributed (.toRecipient thisCreature)
          (.prevent (.some (.inCaller amount)) none)
          .repeatedly) ]
semantic_definition affinity (quality : Predicate) : Ability :=
  .keyword (.named "affinity") [.quality (.inCaller quality)]
    [ static
        (.costShift thisSpell
          (.less (forEach (.lit 1) (and [.inCaller quality, youControl])) none)) ]
semantic_definition afflict (amount : Amount) : Ability :=
  .keyword (.named "afflict") [.number (.inCaller amount)]
    [ triggered (does .any (.block thisCreature))
        (instruct defendingPlayer (loseLife (.inCaller amount))) ]
semantic_definition afterlife (amount : Amount) : Ability :=
  .keyword (.named "afterlife") [.number (.inCaller amount)]
    [ when (does thisPermanent die)
        (createToken (creatureToken 1 1 [.white, .black] [spirit] [flying]) (.inCaller amount)) ]
semantic_definition aftermath : Ability :=
  .keyword (.named "aftermath") []
    [ static
        (can (.actor .actor) (cast this_ (from_ := some yourGraveyard))),
      static
        (cant (.actor .actor)
          (cast this_ (from_ := some (not yourGraveyard)))),
      static
        (onlyWhile
          (replace (would thisSpell (leave stack)) (instead (exile thisSpell)))
          (matches_ thisSpell (.castFrom graveyard))) ]
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
                putCounters (times (.inCaller amount) (.countOf them)) (.printed p1p1Counter)
                  thisPermanent ]))) ]
semantic_definition annihilator (amount : Amount) : Ability :=
  .keyword (.named "annihilator") [.number (.inCaller amount)]
    [ triggered (does (.actor thisCreature) (.attack none))
        (instruct defendingPlayer
          (sacrifice (counted (.exactlyOf (.inCaller amount)) isPermanent))) ]
semantic_definition ascend : Ability :=
  .keyword (.named "ascend") []
    [ static
        (onlyIfSo
          (.whileHolds
            (and
              [ .compareAmt (countOf (and [isPermanent, youControl])) .atLeast (.lit 10),
                not (matches_ .you (.hasDesignation citysBlessing none)) ])
            (.gainDesignation .you citysBlessing (some restOfGame)))
          (matches_ this_ isPermanentType)),
      spell
        (doIf (matches_ this_ isInstantOrSorcery)
          (doIf
            (and
              [ .compareAmt (countOf (and [isPermanent, youControl])) .atLeast (.lit 10),
                not (matches_ .you (.hasDesignation citysBlessing none)) ])
            (.gainDesignation .you citysBlessing (some restOfGame))
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
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      spell
        (doIf (keywordCostPaid (.named "awaken") this_)
          (sequentially
            [ putCounters (.inCaller amount) (.printed p1p1Counter)
                (target (and [isLand, youControl])),
              .establish
                (.characteristicChange (that land)
                  [ .typeLine .adds
                      { types := some [.creature], subtypes := some [elemental] },
                    .stat .sets .power (.lit 0),
                    .stat .sets .toughness (.lit 0),
                    .addedAbilities [keyword (.named "haste")] ])
                none ])
          none) ]
semantic_definition backup (amount : Amount) : Ability :=
  .keyword (.named "backup") [.number (.inCaller amount)]
    [ triggered (does thisCreature enter)
        (sequentially
          [ putCounters (.inCaller amount) (.printed p1p1Counter) (target isCreature),
            doIf (matches_ (that creature) (.otherThan thisCreature))
              (establishThroughout
                (Templates.abilityGrantSelected (that creature) thisCreature
                  (.printedAfter (.named "backup")))
                untilEndOfTurn)
              none ]) ]
semantic_definition banding : Ability := keyword (.named "banding")
/-- "Bands with other <quality>", a special form of banding [CR#702.22b]. -/
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
    [ triggered (does (.actor thisCreature) (.attack none))
        (establishThroughout
          (Templates.modification (each (and [isCreature, isAttacking, .otherThan thisCreature])) .power
            (.up (.lit 1)))
          untilEndOfTurn) ]
semantic_definition bestow (cost : Cost) : Ability :=
  .keyword (.named "bestow") [.cost (.inCaller cost)]
    [static (.altCost thisSpell (some (.inCaller cost)))]
semantic_definition blitz (cost : Cost) : Ability :=
  .keyword (.named "blitz") [.cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "blitz"))))
          (atNext .endStep (sacrifice (.becomes thisSpell)))),
      static
        (onlyWhile (abilityGrant thisPermanent (keyword (.named "haste")))
          (keywordCostPaid (.named "blitz") thisPermanent)),
      static
        (onlyWhile
          (abilityGrant thisPermanent
            (triggered (does thisPermanent die) (.draw (.lit 1))))
          (keywordCostPaid (.named "blitz") thisPermanent)) ]
semantic_definition bloodthirst (amount : Amount) : Ability :=
  .keyword (.named "bloodthirst") [.number (.inCaller amount)]
    [ static
        (replace (would thisPermanent enter)
          (doIf (happened (.dealDamage .anyAmount .which .any) (.actor anOpponent) soFarThisTurn)
            (withRiders [.withCounters (.inCaller amount) (.printed p1p1Counter) .fresh]))) ]
semantic_definition boast (ability : Ability) : Ability :=
  .keyword (.named "boast") [.ability ability]
    [ .qualified ability none (some oncePerTurn)
        (some (matches_ thisCreature (happenedTo (.actor .which) (.attack none) soFarThisTurn)))
        none ]
semantic_definition buyback (cost : Cost) : Ability :=
  .keyword (.named "buyback") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .once),
      static
        (onlyWhile
          (replace (.would .any (.move thisSpell stack (some graveyard) .departure none []))
            (instead (.move thisSpell .anyLocation (some hand) .arrival none [])))
          (keywordCostPaid (.named "buyback") thisSpell)) ]
semantic_definition casualty (amount : Amount) : Ability :=
  .keyword (.named "casualty") [.number (.inCaller amount)]
    [ static
        (.addedCost
          (.perform (sacrifice (a (and [isCreature, hasPower .atLeast (.inCaller amount)]))))
          true .once),
      triggeredIf (does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "casualty") thisSpell)
        (sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))]) ]
semantic_definition champion (quality : Predicate) : Ability :=
  .keyword (.named "champion") [.subject (.inCaller quality)]
    [ triggered (does thisPermanent enter)
        (mayOrElse
          (pay
            (.perform (exile (a (and [.inCaller quality, youControl, .otherThan thisPermanent]))))
            .once)
          (sacrifice thisPermanent)),
      triggered (does thisPermanent leave)
        (returnTo (linkedObject (.byDeed (.action "exile")) (.action "exile") this_) battlefield
          [.under (ownerOf (that .card))]) ]
semantic_definition changeling : Ability :=
  .keyword (.named "changeling") []
    [static (.characteristicChange thisCreature [.quality .adds (.everyTypeOf .creature)])]
semantic_definition chooseABackground : Ability := keyword (.named "chooseABackground")
semantic_definition cipher : Ability :=
  .keyword (.named "cipher") []
    [ spell
        (doIf (matches_ this_ .isCard)
          (may
            (sequentially
              [ exile thisSpell,
                .relate this_ encodedOn (a (and [isCreature, youControl])) ]))
          none),
      static
        (abilityGrant (.counterpart encodedOn this_)
          (triggered
            (does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
            (may
              (sequentially
                [ .copy .fromCardZone
                    (the (and [.isCard, .related encodedOn (some thisCreature)]))
                    (.lit 1) [],
                  may (cast (that .copy) (payment := withoutPaying)) ])))) ]
semantic_definition cleave (cost : Cost) : Ability :=
  .keyword (.named "cleave") [.cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
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
      triggeredIf (does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "conspire") thisSpell)
        (sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))]) ]
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
                    and [isCreature, youControl, .otherThan thisPermanent] ])))))
        (establishThroughout
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          untilEndOfTurn) ]
semantic_definition dash (cost : Cost) : Ability :=
  .keyword (.named "dash") [.cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "dash"))))
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
          (doIf (matches_ thisPermanent (.currentFace .front))
            (transform thisPermanent)
            none)),
      static
        (cant any (transform thisPermanent) (except := some (.byKeyword (.named "daybound")))) ]
semantic_definition deathtouch : Ability := keyword (.named "deathtouch")
semantic_definition decayed : Ability :=
  .keyword (.named "decayed") []
    [ static (cant (.actor thisCreature) (.block .any)),
      triggered (does (.actor thisCreature) (.attack none))
        (atNext .endOfCombat (sacrifice thisPermanent)) ]
semantic_definition defender : Ability :=
  .keyword (.named "defender") [] [static (cant (.actor thisCreature) (.attack none))]
semantic_definition delve : Ability :=
  .keyword (.named "delve") []
    [ static
        (.paySubstitute thisSpell
          ⟨ exile (a (cardIn yourGraveyard)),
            [.generic] ⟩) ]
semantic_definition demonstrate : Ability :=
  .keyword (.named "demonstrate") []
    [ triggered (does (.actor .you) (cast thisSpell))
        (may (sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))])
          (some
            (sequentially
              [ choose anOpponent,
                instruct (the isChosenPlayer) (copySpell thisSpell),
                instruct (the isChosenPlayer) (may (.chooseNewTargets (that .copy))) ]))) ]
semantic_definition dethrone : Ability :=
  .keyword (.named "dethrone") []
    [ triggered
        (does (.actor thisCreature)
          (.attack
            (some
              (the (and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))))
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition devoid : Ability :=
  .keyword (.named "devoid") [] [static (.characteristicChange this_ [.colors .sets (.some [])])]
semantic_definition devourQuality (quality : Predicate) (amount : Amount) : Ability :=
  .keyword (.named "devour") [.quality (.inCaller quality), .number (.inCaller amount)]
    [ static
        (replace (would thisPermanent enter)
          (also
            (may (sacrifice (counted anyNumber (and [.inCaller quality, .in_ battlefield])))
              (some
                (putCounters
                  (.arith .times (.inCaller amount)
                    (.countOf (theVerbed (.action "sacrifice") .many)))
                  (.printed p1p1Counter) thisPermanent))))) ]
semantic_definition devour (amount : Amount) : Ability :=
  .keyword (.named "devour") [.number (.inCaller amount)] [devourQuality isCreature amount]
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
  .keyword (.named "dredge") [.number amount]
    [ static
        (onlyWhile
          (replace (would you (draw (.lit 1)))
            (instead (may (sequentially [mill amount, returnToHand this_ yourGraveyard]))))
          (.compareAmt (countOf (cardIn yourLibrary)) .atLeast amount)) ]
semantic_definition echo (cost : Cost) : Ability := keywordCosting (.named "echo") (.inCaller cost)
semantic_definition embalm (cost : Cost) : Ability :=
  .keyword (.named "embalm") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create (.lit 1)
          (.copyOf this_
            [ .edits
                [ .colors .sets (.some [.white]),
                  .manaCost none,
                  .typeLine .adds { subtypes := some [zombie] } ] ]))
        asSorcery ]
semantic_definition emerge (cost : Cost) : Ability :=
  .keyword (.named "emerge") [.cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_
            (payment :=
              .payingInstead (.compound [.inCaller cost, .perform (sacrifice (a isCreature))])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (.less
              (.statOf (.stat .manaValue)
                (linkedObject .theAlternative (.action "sacrifice") thisSpell))
              none))
          (keywordCostPaid (.named "emerge") thisSpell)) ]
semantic_definition emergeFrom (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "emergeFrom") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform (sacrifice (a (and [.inCaller quality, isPermanent]))) ])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (.less
              (.statOf (.stat .manaValue)
                (linkedObject .theAlternative (.action "sacrifice") thisSpell))
              none))
          (keywordCostPaid (.named "emergeFrom") thisSpell)) ]
semantic_definition enchant (quality : Predicate) : Ability :=
  .keyword (.named "enchant") [.subject (.inCaller quality)]
    [ static (.addsTarget thisAura (.inCaller quality)),
      static (mayBeAttachedTo thisAura (allOf (.inCaller quality))),
      static
        (.asEvent (does .any (.move thisAura (not stack) (some battlefield) .arrival none []))
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
        (establishThroughout (cant (.actor .you) (cast .any)) restOfGame),
      spell
        (delayWithin (beginningOfYour .upkeep) restOfGame
          (sequentially
            [ .copy .fromStack thisSpell (.lit 1)
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
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard) (payment := .payingInstead (.inCaller cost)))) ]
semantic_definition eternalize (cost : Cost) : Ability :=
  .keyword (.named "eternalize") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create (.lit 1) (.copyOf this_ [ .edits
                  [ .colors .sets (.some [.black]),
                    .stat .sets .power (.lit 4),
                    .stat .sets .toughness (.lit 4),
                    .manaCost none,
                    .typeLine .adds { subtypes := some [zombie] } ] ]) [])
        asSorcery ]
semantic_definition evoke (cost : Cost) : Ability :=
  .keyword (.named "evoke") [.cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      triggeredIf (does thisPermanent enter)
        (keywordCostPaid (.named "evoke") thisPermanent)
        (instruct (controllerOf thisPermanent) (sacrifice thisPermanent)) ]
semantic_definition evolve : Ability :=
  .keyword (.named "evolve") []
    [ triggeredIf (does (a (and [isCreature, youControl])) enter)
        (or
          [ .compareAmt (powerOf (that creature)) .greater (powerOf thisCreature),
            .compareAmt (toughnessOf (that creature)) .greater (toughnessOf thisCreature) ])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition exalted : Ability :=
  .keyword (.named "exalted") []
    [ triggeredIf (does (.actor (a (and [isCreature, youControl]))) (.attack none))
        (.compareAmt (countOf (and [isCreature, isAttacking])) .eq (.lit 1))
        (get (that creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ]
semantic_definition exhaust (ability : Ability) : Ability :=
  .keyword (.named "exhaust") [.ability ability]
    [.qualified ability none (some oncePerObject) none none]
semantic_definition exploit : Ability :=
  .keyword (.named "exploit") []
    [ triggered (does thisCreature enter)
        (may (sacrifice (a isCreature))) ]
semantic_definition extort : Ability :=
  .keyword (.named "extort") []
    [ triggered (does (.actor .you) (cast (a isSpell)))
        (may (pay (.mana [hybridPip .white .black]) .once)
          (some (sequentially [instruct eachOpponent (loseLife (.lit 1)), gainLife .thatMuch]))) ]
semantic_definition fabricate (amount : Amount) : Ability :=
  .keyword (.named "fabricate") [.number (.inCaller amount)]
    [ triggered (does thisPermanent enter)
        (mayOrElse (putCounters (.inCaller amount) (.printed p1p1Counter) thisPermanent)
          (createToken (artifactCreatureToken 1 1 [] [servo]) (.inCaller amount))) ]
semantic_definition fading (amount : Amount) : Ability :=
  .keyword (.named "fading") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) fadeCounter),
      triggered (beginningOfYour .upkeep)
        (doIfDone (removeCounters (oneThrough 1) (some (.printed fadeCounter)) thisPermanent) none
          (some (sacrifice thisPermanent))) ]
/-- "Fear": "This creature can't be blocked except by artifact creatures and/or black
creatures." [CR#702.36b] -/
semantic_definition fear : Ability :=
  .keyword (.named "fear") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, or [isArtifact, hasColor .black]])))) ]
semantic_definition firebending (amount : Amount) : Ability :=
  .keyword (.named "firebending") [.number (.inCaller amount)]
    [ triggered (does (.actor thisCreature) (.attack none))
        (sequentially
          [ .addMana (.inCaller amount) (.runs [[ColorOrColorless.of .red]]) [],
            establishThroughout (.manaRetention .you .thisMana) untilEndOfCombat ]) ]
semantic_definition firstStrike : Ability := keyword (.named "firstStrike")
semantic_definition flanking : Ability :=
  .keyword (.named "flanking") []
    [ triggered
        (does (.by (a (and [isCreature, not (.hasKeyword (.the (.named "flanking")))])))
          (.block thisCreature))
        (get (that creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)) ]
semantic_definition flash : Ability :=
  .keyword (.named "flash") [] [static (can you (play thisCard) (timing := some asInstant))]
semantic_definition flashback (cost : Cost) : Ability :=
  .keyword (.named "flashback") [.cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard) (payment := .payingInstead (.inCaller cost)))),
      static
        (onlyWhile
          (replace (would thisSpell (leave stack)) (instead (exile thisSpell)))
          (keywordCostPaid (.named "flashback") thisSpell)) ]
semantic_definition forMirrodin : Ability :=
  .keyword (.named "forMirrodin") []
    [ triggered (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 2 2 [.red] [rebel]),
            attachTo thisEquipment itAsToken ]) ]
semantic_definition forecast (ability : Ability) : Ability :=
  .keyword (.named "forecast") [.ability ability]
    [ .qualified ability (some (duringPart .upkeep (some (ownerOf this_)))) (some oncePerTurn) none
        (some .hand) ]
semantic_definition foretell (cost : Cost) : Ability :=
  .keyword (.named "foretell") [.cost (.inCaller cost)]
    [ static
        (specialAction (duringPart .turn (some you)) (mana [generic 2])
          (sequentially
            [ exileFaceDown thisCard,
              establishThroughout (visibility .lookAt you (objects it))
                (forAsLongAs (matches_ it (in_ exileZone))),
              establishThroughout
                (can you
                  (cast it (from_ := some exileZone) (payment := payingInstead (.inCaller cost))))
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
      triggeredIf (does thisPermanent enter) (matches_ this_ isPermanentType)
        (doIf (keywordCostPaid (.named "gift") this_)
          (instruct (chosenAsPaid (.named "gift") this_) (.inCaller promise))
          none),
      spell
        (doIf (matches_ this_ isInstantOrSorcery)
          (doIf (keywordCostPaid (.named "gift") this_)
            (instruct (chosenAsPaid (.named "gift") this_) (.inCaller promise))
            none)
          none) ]
/-! The licensed phrasings of gift: its templating is not normative ("Gift a card", "Gift a
tapped Fish"), so each is a Template over `gift` [CR#702.174a,702.174d..702.174i]. -/

semantic_template giftACard : Ability := gift (draw (.lit 1))
semantic_template giftAFood : Ability := gift (createToken foodToken)
semantic_template giftATappedFish : Ability :=
  gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
semantic_template giftATreasure : Ability := gift (createToken treasureToken)
semantic_template giftAnExtraTurn : Ability := gift (extraTurn (.lit 1))
semantic_template giftAnOctopus : Ability :=
  gift (createToken (creatureToken 8 8 [.blue] [octopus]))
semantic_definition graft (amount : Amount) : Ability :=
  .keyword (.named "graft") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) p1p1Counter),
      triggeredIf
        (does (a (and [isCreature, .otherThan thisPermanent])) enter)
        (matches_ thisPermanent (.hasCounters (some p1p1Counter)))
        (may (moveCounters (.lit 1) thisPermanent (that creature) (some (.printed p1p1Counter)))) ]
semantic_definition gravestorm : Ability :=
  .keyword (.named "gravestorm") []
    [ triggered (does (.actor .you) (cast thisSpell))
        (sequentially
          [ copySpell thisSpell
              (eventCount die (bare isPermanent) soFarThisTurn),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition harmonize (cost : Cost) : Ability :=
  .keyword (.named "harmonize") [.cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard)
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform
                    (tap
                      (counted (upTo 1) (and [isUntapped, and [isCreature, youControl]]))) ])))),
      static
        (onlyWhile
          (.costShift thisSpell
            (.less (powerOf (linkedObject .theAlternative (.action "tap") thisSpell)) none))
          (keywordCostPaid (.named "harmonize") thisSpell)),
      static
        (onlyWhile
          (replace (would thisSpell (leave stack)) (instead (exile thisSpell)))
          (keywordCostPaid (.named "harmonize") thisSpell)) ]
semantic_definition haste : Ability := keyword (.named "haste")
semantic_definition haunt : Ability :=
  .keyword (.named "haunt") []
    [ triggeredIf (does thisPermanent die)
        (matches_ this_ isPermanentType)
        (sequentially
          [ exile it,
            .relate it haunting (target isCreature) ]),
      triggeredIf (does .any (.move thisSpell stack (some graveyard) .departure none []))
        (matches_ this_ isInstantOrSorcery)
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
  let exiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
  .keyword (.named "hideaway") [.number (.inCaller amount)]
    [ when (does thisPermanent enter)
        (sequentially
          [ lookAt (topSlice (.inCaller amount)),
            exileFaceDown (someOf (oneThrough 1) them),
            .move (theRest .object) .anyLocation (some onBottom) .arrival none
              [.inOrder .randomOrder],
            establishThroughout (visibility .lookAt you (objects exiledCard))
              (forAsLongAs (matches_ exiledCard (in_ exileZone))) ]) ]
semantic_definition horsemanship : Ability :=
  .keyword (.named "horsemanship") []
    [ static
        (cant (.by (allOf (and [isCreature, not (.hasKeyword (.the (.named "horsemanship")))])))
          (.block thisCreature)) ]
semantic_definition impending (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "impending") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (replace (would thisPermanent enter)
          (doIf (keywordCostPaid (.named "impending") thisPermanent)
            (withRiders [.withCounters (.inCaller amount) (.printed timeCounter) .fresh]))),
      static
        (onlyWhile
          (.characteristicChange thisPermanent [.typeLine .loses { types := some [.creature] }])
          (and
            [ keywordCostPaid (.named "impending") thisPermanent,
              matches_ thisPermanent (.hasCounters (some timeCounter)) ])),
      triggeredIf (beginningOfYour .endStep)
        (and
          [ keywordCostPaid (.named "impending") thisPermanent,
            matches_ thisPermanent (.hasCounters (some timeCounter)) ])
        (removeCounters (oneThrough 1) (some (.printed timeCounter)) thisPermanent) ]
semantic_definition improvise : Ability :=
  .keyword (.named "improvise") []
    [ static
        (.paySubstitute thisSpell
          ⟨ tap (a (and [isUntapped, isArtifact, youControl])),
            [.generic] ⟩) ]
semantic_definition increment : Ability :=
  .keyword (.named "increment") []
    [ triggeredIf (does (.actor .you) (cast (a isSpell)))
        (and
          [ matches_ thisPermanent isCreature,
            or
              [ .compareAmt (manaValueSpentToCast (that .spell)) .greater (powerOf thisCreature),
                .compareAmt (manaValueSpentToCast (that .spell)) .greater
                  (toughnessOf thisCreature) ] ])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition indestructible : Ability :=
  .keyword (.named "indestructible") []
    [ static
        (cant .any
          (destroy thisPermanent)) ]
semantic_definition infect : Ability := keyword (.named "infect")
semantic_definition infinity (ability : Ability) : Ability :=
  .keyword (.named "infinity") [.ability ability]
    [ static
        (onlyWhile (abilityGrant thisPermanent ability)
          (matches_ thisPermanent (.hasDesignation harnessed none))) ]
semantic_definition ingest : Ability :=
  .keyword (.named "ingest") []
    [ triggered (does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
        (instruct they
          (exile (.slice .top (.lit 1) (libraryOf they)))) ]
/-- "Intimidate": "This creature can't be blocked except by artifact creatures and/or creatures
that share a color with it." [CR#702.13b] -/
semantic_definition intimidate : Ability :=
  .keyword (.named "intimidate") []
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          or [isArtifact, shares .color .share thisCreature]])))) ]
semantic_definition jobSelect : Ability :=
  .keyword (.named "jobSelect") []
    [ triggered (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 1 1 [] [hero]),
            attachTo thisEquipment itAsToken ]) ]
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
    [ triggered (does thisEquipment enter)
        (sequentially
          [ createToken (creatureToken 0 0 [.black] [phyrexian, germ]),
            attachTo thisEquipment itAsToken ]) ]
semantic_definition madness (cost : Cost) : Ability :=
  .keyword (.named "madness") [.cost (.inCaller cost)]
    [ static
        (replace (would (a isPlayer) (discard thisCard))
          (instead
            (enact (.action "discard") (.move thisCard hand (some exileZone) .arrival none [])))),
      triggered
        (.causes (.event (does any (discard thisCard)))
          (does any (.move thisCard hand (some exileZone) .arrival none [])))
        (instruct (ownerOf thisCard)
          (mayOrElse
            (establish
              (can theActor
                (cast thisCard (from_ := some exileZone)
                  (payment := .payingInstead (.inCaller cost)))))
            (putInto thisCard graveyard))) ]
semantic_definition maxSpeed (ability : Ability) : Ability :=
  .keyword (.named "maxSpeed") [.ability ability]
    [static (onlyWhile (abilityGrant this_ ability) (.compareAmt (speedOf .you) .eq (.lit 4)))]
semantic_definition mayhem (cost : Cost) : Ability :=
  .keyword (.named "mayhem") [.cost (.inCaller cost)]
    [ static
        (onlyWhile
          (can you
            (cast thisCard (from_ := some yourGraveyard)
              (payment := .payingInstead (.inCaller cost))))
          (happened (discard thisCard) you soFarThisTurn)) ]
semantic_definition megamorph (cost : Cost) : Ability :=
  .keyword (.named "megamorph") [.cost (.inCaller cost)]
    [ static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      static
        (.specialAction none (.inCaller cost)
          (sequentially [turnFaceUp this_, putCounters (.lit 1) p1p1Counter this_])) ]
semantic_definition melee : Ability :=
  .keyword (.named "melee") []
    [ triggered (does (.actor thisCreature) (.attack none))
        (get thisCreature
          (.up
            (times (.lit 1)
              (eventCount (.attackWith (a isCreature) (some .which)) (.actor .you) thisCombat)))
          (.up
            (times (.lit 1)
              (eventCount (.attackWith (a isCreature) (some .which)) (.actor .you) thisCombat)))
          (some untilEndOfTurn)) ]
/-- "Menace": "This creature can't be blocked except by two or more creatures."
[CR#702.111b] -/
semantic_definition menace : Ability :=
  .keyword (.named "menace") []
    [ static (cant any (block thisCreature)
        (except := byObjects (counted (atLeast 2) isCreature))) ]
semantic_definition mentor : Ability :=
  .keyword (.named "mentor") []
    [ triggered (does (.actor thisCreature) (.attack none))
        (putCounters (.lit 1) (.printed p1p1Counter)
          (target (and [isCreature, isAttacking, hasPower .less (powerOf thisCreature)]))) ]
semantic_definition miracle (cost : Cost) : Ability :=
  keywordCosting (.named "miracle") (.inCaller cost)
semantic_definition mobilize (amount : Amount) : Ability :=
  .keyword (.named "mobilize") [.number (.inCaller amount)]
    [ triggered (does (.actor thisCreature) (.attack none))
        (sequentially
          [ createTappedAttacking (.inCaller amount) (creatureToken 1 1 [.red] [warrior]),
            atNext .endStep
              (sacrifice themAsTokens) ]) ]
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
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      static
        (.addsTarget thisSpell
          (and
            [isCreature, not (.hasSubtype human), ownedBy (ownerOf thisSpell)])),
      static
        (onlyWhile
          (.replace (.would .any (putOntoBattlefield thisSpell))
            (instead
              (chooseOne
                (.merge thisSpell (linkedObject .theAlternative (.core .target) thisSpell)
                  .top)
                (.merge thisSpell (linkedObject .theAlternative (.core .target) thisSpell)
                  .bottom)))
            none .repeatedly none)
          (keywordCostPaid (.named "mutate") thisSpell)) ]
semantic_definition myriad : Ability :=
  .keyword (.named "myriad") []
    [ triggered (does (.actor thisCreature) (.attack none))
        (sequentially
          [ .doForEach (each (and [isOpponent, .otherThan defendingPlayer]))
              (may
                (createTokenCopy thisCreature
                  [ .entersAs .tapped,
                    .entersAttacking
                      (some (or [they, a (and [isPlaneswalker, controlledBy they])])) ])),
            doIf (.compareAmt (.countOf themAsTokens) .atLeast (.lit 1))
              (atNext .endOfCombat (exile themAsTokens))
              none ]) ]
semantic_definition nightbound : Ability :=
  .keyword (.named "nightbound") []
    [ static
        (.asEvent (gameBecomes day)
          (doIf (matches_ thisPermanent isTransformed)
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
            (.manaCostOf (linkedObject .theAdditional (.action "sacrifice") thisSpell)))
          (keywordCostPaid (.named "offering") thisSpell)),
      static
        (onlyWhile (can (.actor .you) (cast this_) (some asInstant))
          (keywordCostPaid (.named "offering") this_)) ]
semantic_definition offspring (cost : Cost) : Ability :=
  .keyword (.named "offspring") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .once),
      triggeredIf (does thisPermanent enter)
        (keywordCostPaid (.named "offspring") thisPermanent)
        (create (.lit 1)
          (.copyOf thisPermanent
            [.edits [.stat .sets .power (.lit 1), .stat .sets .toughness (.lit 1)]])) ]
semantic_definition outlast (cost : Cost) : Ability :=
  .keyword (.named "outlast") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.compound [.inCaller cost, .tapSymbol])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature)
        asSorcery ]
semantic_definition overload (cost : Cost) : Ability :=
  .keyword (.named "overload") [.cost (.inCaller cost)]
    [ static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
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
    [ triggeredIf (does thisPermanent die)
        (not (matches_ thisPermanent (.hasCounters (some m1m1Counter))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters (.lit 1) (.printed m1m1Counter) .fresh]) ]
semantic_definition phasing : Ability := keyword (.named "phasing")
semantic_definition plot (cost : Cost) : Ability :=
  .keyword (.named "plot") [.cost (.inCaller cost)]
    [ static
        (specialAction asSorcery (.inCaller cost)
          (sequentially
            [ exile thisCard,
              establishThroughout
                (can you (cast it (from_ := some exileZone) (payment := withoutPaying))
                  (timing := some asSorcery))
                onALaterTurn ])) ]
semantic_definition poisonous (amount : Amount) : Ability :=
  .keyword (.named "poisonous") [.number (.inCaller amount)]
    [ triggered (does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
        (putCounters (.inCaller amount) (.printed poison) they) ]
semantic_definition powerUp (ability : Ability) : Ability :=
  .keyword (.named "powerUp") [.ability ability]
    [ .qualified ability none (some oncePerObject) none none,
      static
        (onlyWhile (reducedBySymbols thisAbility (.manaCostOf thisPermanent))
          (matches_ thisPermanent (happenedTo .any (putOntoBattlefield .which) soFarThisTurn))) ]
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
        (.damageRule .any (.dealtBy (allOf (and [.isSource, .inCaller quality])))
          (.toRecipient thisPermanent)
          (.prevent .all none) .repeatedly),
      static (cant (.by (allOf (and [isCreature, .inCaller quality]))) (.block thisCreature)) ]
set_option linter.unusedVariables false in
semantic_definition prototype (cost : Cost) (power toughness : Amount) : Ability :=
  keyword (.named "prototype")
semantic_definition provoke : Ability :=
  .keyword (.named "provoke") []
    [ triggered (does (.actor thisCreature) (.attack none))
        (may
          (establishThroughout
            (must (.actor (target (and [isCreature, controlledBy defendingPlayer])))
              (.block thisCreature))
            untilEndOfCombat)
          (some (untap (that creature)))) ]
semantic_definition prowess : Ability :=
  .keyword (.named "prowess") []
    [ triggered
        (does (.actor .you) (cast (a (and [.in_ stack, non isCreature]))))
        (get thisCreature (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ]
/-- "Prowl <cost>": cast for <cost> if a player was dealt combat damage this turn by a source
you controlled that shared a creature type with this spell [CR#702.76a]. -/
semantic_definition prowl (cost : Cost) : Ability :=
  .keyword (.named "prowl") [.cost (.inCaller cost)]
    [ static
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (.dealDamage .anyAmount (a isPlayer) .combatOnly)
            (.actor (a (and [.shares (.subtype .creature) .share thisSpell, youControl])))
            soFarThisTurn)) ]
semantic_definition rampage (amount : Amount) : Ability :=
  .keyword (.named "rampage") [.number (.inCaller amount)]
    [ triggered (does .any (.block thisCreature))
        (get thisCreature
          (.up
            (times (.inCaller amount)
              (minus (countOf (and [isCreature, .inCombat .blockerOf (some thisCreature)]))
                (.lit 1))))
          (.up
            (times (.inCaller amount)
              (minus (countOf (and [isCreature, .inCombat .blockerOf (some thisCreature)]))
                (.lit 1))))
          (some untilEndOfTurn)) ]
semantic_definition ravenous : Ability :=
  .keyword (.named "ravenous") []
    [ static (entersWithCounters thisPermanent (.letter .x) p1p1Counter),
      triggeredIf (does thisPermanent enter)
        (.compareAmt (.letter .x) .atLeast (.lit 5))
        (.draw (.lit 1)) ]
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
          (happened (putOntoBattlefield thisSaga) .any soFarThisTurn)) ]
semantic_definition rebound : Ability :=
  .keyword (.named "rebound") []
    [ static
        (onlyIfSo
          (replace (.would .any (.move thisSpell stack (some graveyard) .departure none []))
            (instead
              (sequentially
                [ exile thisSpell,
                  delay (beginningOf .upkeep (some .you) (.next none))
                    (may (cast this_ (payment := withoutPaying))) ])))
          (matches_ thisSpell (.castFrom yourHand))) ]
semantic_definition reconfigure (cost : Cost) : Ability :=
  .keyword (.named "reconfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (and [isCreature, .otherThan thisPermanent, youControl])))
        asSorcery,
      .activated (.inCaller cost)
        (enact (.action "attach") (.attachment .unattached thisPermanent none))
        (some asSorcery) none
        (some (matches_ thisPermanent (.attachment .attachment none (some (a isCreature)))))
        none ]
semantic_definition recover (cost : Cost) : Ability :=
  .keyword (.named "recover") [.cost (.inCaller cost)]
    [ triggered
        (does .any (.move (a isCreature) battlefield (some yourGraveyard) .departure none []))
        (may (pay (.inCaller cost) .once) (some (returnToHand this_ yourGraveyard))
          (some (exile this_))) ]
semantic_definition reinforce (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "reinforce") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ activated
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (putCounters (.inCaller amount) (.printed p1p1Counter) (target isCreature)) ]
semantic_definition replicate (cost : Cost) : Ability :=
  .keyword (.named "replicate") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      triggeredIf (does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "replicate") thisSpell)
        (sequentially
          [ copySpell thisSpell (timesPaid (.byKeyword (.named "replicate")) thisSpell),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition retrace : Ability :=
  .keyword (.named "retrace") []
    [ static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard)
            (payment := .payingInstead
              (.compound
                [.itsManaCost, .perform (discard (a (and [.isCard, isLand, .in_ yourHand])))])))) ]
semantic_definition riot : Ability :=
  .keyword (.named "riot") []
    [ static
        (replace (would thisPermanent enter)
          (also
            (mayOrElse (putCounters (.lit 1) (.printed p1p1Counter) thisPermanent)
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
                    and [isCreature, youControl, .otherThan thisPermanent] ])))))
        (.gainDesignation thisPermanent saddled (some untilEndOfTurn))
        asSorcery ]
semantic_definition scavenge (cost : Cost) : Ability :=
  .keyword (.named "scavenge") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (putCounters (powerOf (theVerbed (.action "exile") .one (some .card)))
          (.printed p1p1Counter) (target isCreature))
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
  .keyword (.named "solved") [.ability ability]
    [ .qualified ability none none
        (some (matches_ thisCase (.hasDesignation DesignationLabel.solved none)))
        none ]
semantic_definition soulbond : Ability :=
  .keyword (.named "soulbond") []
    [ triggeredIf (does thisCreature enter)
        (and
          [ matches_ thisCreature (and [youControl, not (.related paired none)]),
            exists_
              (and
                [ and [isCreature, youControl, .otherThan thisCreature],
                  not (.related paired none) ]) ])
        (may
          (.relate thisCreature paired
            (a
              (and
                [ and [isCreature, youControl, .otherThan thisCreature],
                  not (.related paired none) ])))),
      triggeredIf
        (does (a (and [isCreature, youControl, .otherThan thisCreature])) enter)
        (and
          [ matches_ thisCreature (and [youControl, not (.related paired none)]),
            matches_ (that creature) (and [youControl, not (.related paired none)]) ])
        (may (.relate (that creature) paired thisCreature)) ]
semantic_definition soulshift (amount : Amount) : Ability :=
  .keyword (.named "soulshift") [.number (.inCaller amount)]
    [ triggered (does thisPermanent die)
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
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (loseLife .anyAmount) (.actor anOpponent) soFarThisTurn)) ]
semantic_definition splice_ (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "splice") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ static
        (.replace
          (.would (.actor .you)
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
                (activate (allOf (and [.abilityHead .anyActivated, non .isManaAbility]))) ])
          (matches_ thisSpell (.in_ stack))) ]
semantic_definition spree : Ability :=
  .keyword (.named "spree") [] [static (.modeChoice (.setQuantity (atLeast 1)))]
semantic_definition squad (cost : Cost) : Ability :=
  .keyword (.named "squad") [.cost (.inCaller cost)]
    [ static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      triggeredIf (does thisCreature enter)
        (keywordCostPaid (.named "squad") thisCreature)
        (create (timesPaid (.byKeyword (.named "squad")) thisCreature) (.copyOf thisCreature [])) ]
semantic_definition startYourEngines : Ability := keyword (.named "startYourEngines")
semantic_definition station : Ability :=
  .keyword (.named "station") []
    [ activatedOnlyDuring
        (.perform (tap (a (and [isCreature, .otherThan thisPermanent, isUntapped, youControl]))))
        (putCounters (powerOf (theVerbed (.action "tap") .one (some creature)))
          (.printed chargeCounter) thisPermanent)
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
                      or
                        [ isArtifact,
                          .hasSubtype saga,
                          .hasSupertype .legendary ] ]))
                .atLeast (.lit 3),
              not (matches_ .you (.hasDesignation enduringStory none)) ])
          (.gainDesignation .you enduringStory (some restOfGame))) ]
semantic_definition sunburst : Ability :=
  .keyword (.named "sunburst") []
    [ static
        (replace (would thisPermanent enter)
          (doIf (matches_ thisPermanent isCreature)
            (withRiders
              [.withCounters (colorsSpentToCast thisSpell) (.printed p1p1Counter) .fresh]))),
      static
        (replace (would thisPermanent enter)
          (doIf (not (matches_ thisPermanent isCreature))
            (withRiders
              [.withCounters (colorsSpentToCast thisSpell) (.printed chargeCounter) .fresh]))) ]
semantic_definition surge (cost : Cost) : Ability :=
  .keyword (.named "surge") [.cost (.inCaller cost)]
    [ static
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (cast (a anotherSpell))
            (.actor (or [.you, a isTeammate]))
            soFarThisTurn)) ]
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
    [ triggered
        (does
          (.actor
            (and
              [ thisCreature,
                counted (atLeast 1)
                  (and
                    [ isCreature,
                      .otherThan thisCreature,
                      hasPower .greater (powerOf thisCreature) ]) ]))
          (.attack none))
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition trample : Ability := keyword (.named "trample")
semantic_definition transfigure (cost : Cost) : Ability :=
  .keyword (.named "transfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (sacrifice thisPermanent) ])
        (sequentially
          [ .search yourLibrary (oneThrough 1)
              (and
                [.isCard, isCreature, hasManaValue .eq (.statOf (.stat .manaValue) thisPermanent)]),
            putOntoBattlefield (theVerbed (.action "search") .one),
            .shuffle (libraryOf .actor) ])
        asSorcery ]
semantic_definition transmute (cost : Cost) : Ability :=
  .keyword (.named "transmute") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (sequentially
          [ .search yourLibrary (oneThrough 1)
              (and
                [ .isCard,
                  hasManaValue .eq
                    (.statOf (.stat .manaValue)
                      (theVerbed (.action "discard") .one (some .card))) ]),
            .expose .reveal (.cards (theVerbed (.action "search") .one)),
            putInto (theVerbed (.action "search") .one) hand,
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
                  (may (putCounters (.inCaller amount) (.printed p1p1Counter) thisCreature)) ]))) ]
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
    [static (.costShift thisSpell (.less (forEach (.lit 1) isOpponent) none))]
semantic_definition undying : Ability :=
  .keyword (.named "undying") []
    [ triggeredIf (does thisPermanent die)
        (not (matches_ thisPermanent (.hasCounters (some p1p1Counter))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters (.lit 1) (.printed p1p1Counter) .fresh]) ]
semantic_definition unearth (cost : Cost) : Ability :=
  .keyword (.named "unearth") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (sequentially
          [ returnToBattlefield thisCard (from_ := some yourGraveyard),
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
          (also (may (putCounters (.lit 1) (.printed p1p1Counter) thisPermanent)))),
      static
        (onlyWhile (cant (.actor thisPermanent) (.block .any))
          (matches_ thisPermanent (.hasCounters (some p1p1Counter)))) ]
semantic_definition vanishing (amount : Amount) : Ability :=
  .keyword (.named "vanishing") [.number (.inCaller amount)]
    [ static (entersWithCounters thisPermanent (.inCaller amount) timeCounter),
      triggeredIf (beginningOfYour .upkeep)
        (matches_ thisPermanent (.hasCounters (some timeCounter)))
        (removeCounters (oneThrough 1) (some (.printed timeCounter)) thisPermanent),
      triggered (lastCounterRemoved timeCounter thisPermanent)
        (sacrifice thisPermanent) ]
semantic_definition vigilance : Ability := keyword (.named "vigilance")
semantic_definition ward (cost : Cost) : Ability :=
  .keyword (.named "ward") [.cost (.inCaller cost)]
    [ triggered (does (.by (a (and [.in_ stack, opponentsControl]))) (.target thisPermanent))
        (instruct (controllerOf (that .stack))
          (mayOrElse (pay (.inCaller cost) .once)
            (instruct .you (counter (that .stack))))) ]
semantic_definition warp (cost : Cost) : Ability :=
  .keyword (.named "warp") [.cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_ (from_ := some yourHand) (payment := .payingInstead (.inCaller cost)))),
      static
        (.asEvent
          (does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "warp"))))
          (atNext .endStep
            (sequentially
              [ exile (.becomes thisSpell),
                establishThroughout
                  (can (.actor (ownerOf it))
                    (cast it (from_ := some exileZone)))
                  onALaterTurn ]))) ]
semantic_definition webSlinging (cost : Cost) : Ability :=
  .keyword (.named "webSlinging") [.cost (.inCaller cost)]
    [ static
        (can (.actor .you)
          (cast this_ (payment := .payingInstead (.compound
                [ .inCaller cost,
                  .perform (returnToHand (a (and [isCreature, isTapped, youControl]))) ])))) ]
semantic_definition wither : Ability := keyword (.named "wither")

end Semantics.Definitions
