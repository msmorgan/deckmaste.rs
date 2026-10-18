import Semantics.Primitives
import Semantics.Check
import Semantics.CallerScope

/-!
# Semantics.Templates

Phrasing helpers (`Template`s, `semantic_template`) for `semantics-v3`
(`docs/decisions/semantics-v3.md` §§1–3). A Template expands transparently in the caller's DRS
scope and uses `firstMention` / `laterMention` whenever a `NounPhrase` parameter is mentioned more
than once before a zone change. Templates are spelled over primitives and other Templates only;
a phrasing helper that needs rules-defined vocabulary lives at the end of `Semantics.Definitions`.
-/

namespace Semantics.Templates

/-! ## Pronouns -/

/-- "it" -/
semantic_template it : NounPhrase := .pro .bare .one
/-- "them" -/
semantic_template them : NounPhrase := .pro .bare .many
/-- "that card": the card slot's occupant. -/
semantic_template itCard : NounPhrase := .pro (.atSlot .card) .one
/-- "it": the permanent slot's occupant, as "sacrifice it" reads it. -/
semantic_template itPermanent : NounPhrase := .pro (.atSlot .permanent) .one
/-- "that token": the token just created. -/
semantic_template itAsToken : NounPhrase := .pro .tokenBorn .one
/-- "itself" / "himself" / "herself" -/
semantic_template itself : NounPhrase := .refl .one
/-- "themselves" -/
semantic_template themselves : NounPhrase := .refl .many
/-- "that <word>", e.g. `that .player`. -/
semantic_template that (w : NounWord) : NounPhrase := .pro (.word w) .one
/-- "that player or planeswalker": the joined phrase just named. -/
semantic_template thatJoin : NounPhrase := that .join
/-- "those <word>s", e.g. `those .card`. -/
semantic_template those (w : NounWord) : NounPhrase := .pro (.word w) .many
/-- "they": the player most recently named. -/
semantic_template they : NounPhrase := .pro (.word .player) .one
/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
semantic_template splitOverPlaneswalker : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one))
/-- The same over "any target": the player, or the permanent's controller. -/
semantic_template splitOverPermanent : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf .permanent) .one))
/-- "the <verb>ed <word>", e.g. "the tapped creatures"; with no word, the pronoun the deed
stamped: "it" or "them", "the exiled card". -/
semantic_template theVerbed (verb : Deed) (plurality : Plurality) (word : Option NounWord := none)
    (marking : VerbedMarking := .attributive) : NounPhrase :=
  .pro (.verbed verb word marking) plurality
/-! ## Quantities -/

semantic_template exactly (n : Nat) : Quantity := .range (some n) (some n)
semantic_template upTo (n : Nat) : Quantity := .range none (some n)
semantic_template anyNumber : Quantity := .range none none
semantic_template atLeast (n : Nat) : Quantity := .range (some n) none
semantic_template oneThrough (n : Nat) : Quantity := .range (some 1) (some n)
semantic_template exactlyOf (amount : Semantics.Amount) : Semantics.Quantity :=
  (Semantics.Quantity.exactlyOf amount)
semantic_template range (low : (Option Nat) := none) (high : (Option Nat) := none) : Semantics.Quantity :=
  (Semantics.Quantity.range low high)
semantic_template Quantity.upTo (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range none (some n))
semantic_template upToOf (amount : Semantics.Amount) : Semantics.Quantity :=
  (Semantics.Quantity.upToOf amount)
semantic_template worth (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.worth n)
/-! ## Determiners -/

/-- "target …" -/
semantic_template target (p : Predicate) : NounPhrase := .described (.target (.range (some 1) (some 1))) p
/-- "each …" -/
semantic_template each (p : Predicate) : NounPhrase := .described .each p
/-- "all …" -/
semantic_template allOf (p : Predicate) : NounPhrase := .described .all p
/-- the bare plural: "creatures" -/
semantic_template bare (p : Predicate) : NounPhrase := .described .bare p
/-- "a …" -/
semantic_template a (p : Predicate) : NounPhrase := .described (.a .unmarked) p
/-- "a … at random" -/
semantic_template aAtRandom (p : Predicate) : NounPhrase := .described (.a .atRandom) p
/-- "a … of their choice", the chooser being the last-mentioned player. -/
semantic_template aTheirChoice (p : Predicate) : NounPhrase := .described (.a .theirChoice) p
/-- "a … of your choice" -/
semantic_template aYourChoice (p : Predicate) : NounPhrase := .described (.a .yourChoice) p
/-- "the …" -/
semantic_template the (p : Predicate) : NounPhrase := .described .the p
/-- "N …", "up to N …" -/
semantic_template counted (q : Quantity) (p : Predicate) : NounPhrase := .described (.count q none) p
/-- "N <p> at random" -/
semantic_template countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q (some .atRandom)) p
/-- "if there is a …" -/
semantic_template exists_ (p : Predicate) : Condition := .exists_ (bare p)
/-- "the number of …s" -/
semantic_template countOf (p : Predicate) : Amount := .countOf (bare p)
/-- "the greatest power among …s" -/
semantic_template aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (bare p)
semantic_template aRandom (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.atRandom) p)
semantic_template random (q : Semantics.Quantity) (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.count q (some Semantics.ChoiceMode.atRandom)) p)
/-! ## Zones -/

semantic_template battlefield : Location := .zone .battlefield none
semantic_template exileZone : Location := .zone .exile none
semantic_template hand : Location := .zone .hand none
semantic_template graveyard : Location := .zone .graveyard none
semantic_template stack : Location := .zone .stack none
semantic_template command : Location := .zone .command none
/-- "the command zone", scoped to nobody. -/
semantic_template commandZone : Location := .zone .command none
semantic_template yourLibrary : Location := .zone .library (some .you)
/-- "<player>'s library": a face-down pile [CR#401.2]. -/
semantic_template library (player : NounPhrase) : Location := .zone .library (some player)
/-- "on the bottom of its owner's library" -/
semantic_template onBottom : Location := .fromEnd .bottom (.nth 1) (.zone .library none)
/-- "on top of its owner's library" -/
semantic_template onTop : Location := .fromEnd .top (.nth 1) (.zone .library none)
semantic_template handOf (player : NounPhrase) : Location := .zone .hand (some player)
/-- "your hand" -/
semantic_template yourHand : Location := handOf .you
semantic_template graveyardOf (player : NounPhrase) : Location := .zone .graveyard (some player)
semantic_template place (end_ : Semantics.PileEnd) (pile : Semantics.Location) : Semantics.Location :=
  (Semantics.Location.fromEnd end_ (Semantics.Ordinal.nth 1) pile)
semantic_template wherever : Semantics.Location :=
  Semantics.Location.anyLocation
semantic_template zone (zone : Semantics.Zone) (scope : Option Semantics.NounPhrase) : Semantics.Location :=
  (Semantics.Location.zone zone scope)
/-! ## Predicates -/

semantic_template creature : Predicate := .hasType .creature
semantic_template artifact : Predicate := .hasType .artifact
semantic_template enchantment : Predicate := .hasType .enchantment
semantic_template land : Predicate := .hasType .land
semantic_template instant : Predicate := .hasType .instant
semantic_template sorcery : Predicate := .hasType .sorcery
semantic_template instantOrSorcery : Predicate := .or [instant, sorcery]
/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
semantic_template permanentType : Predicate :=
  .or [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
        .hasType .land, .hasType .planeswalker ]
semantic_template tapped : Predicate := .hasStatus .tapped
semantic_template faceDown : Predicate := .hasStatus .faceDown
/-- "another player": any player other than you. -/
semantic_template otherPlayer : Predicate := .and [.anyPlayer, .otherThan .you]
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
semantic_template spell : Predicate := .and [.in_ stack, .not (.abilityHead .anyOnStack)]
semantic_template untapped : Predicate := .hasStatus .untapped
semantic_template nontoken : Predicate := .not .isToken
/-- "permanent": an object on the battlefield [CR#110.1]. -/
semantic_template permanent : Predicate := .in_ battlefield
/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
semantic_template permanentCard : Predicate :=
  .and
    [ .isCard,
      .or
        [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
          .hasType .land, .hasType .planeswalker ] ]
/-- "colorless": an object of no color [CR#105.2c]. -/
semantic_template colorless : Predicate := .colorCount .eq 0
/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
semantic_template multicolored : Predicate := .colorCount .atLeast 2
/-- "monocolored" -/
semantic_template monocolored : Predicate := .colorCount .eq 1
/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
semantic_template historic : Predicate :=
  .or [.hasType .artifact, .hasSupertype .legendary, .hasSubtype (.of .enchantment "Saga")]
semantic_template attacking : Predicate := .inCombat .attackerOf none
semantic_template blocking : Predicate := .inCombat .blockerOf none
semantic_template blocked : Predicate := .inCombat .blockedBy none
semantic_template unblocked : Predicate := .not blocked
semantic_template creatureYouControl : Predicate := .and [creature, .hasPossessor .controller .you]
semantic_template creatureYouDontControl : Predicate := .and [creature, .not (.hasPossessor .controller .you)]
semantic_template creatureYourOpponentsControl : Predicate :=
  .and [creature, .hasPossessor .controller (.described .all (.isOpponentOf .you))]
/-- "another creature", anchored on <n>. -/
semantic_template otherCreature (n : NounPhrase) : Predicate := .and [creature, .otherThan n]
/-- "another creature you control", anchored on <n>. -/
semantic_template otherCreatureYouControl (n : NounPhrase) : Predicate :=
  .and [creature, .hasPossessor .controller .you, .otherThan n]
/-- "source": the object dealing the damage under discussion. -/
semantic_template source : Predicate := .isSource
semantic_template emblem : Predicate := .isEmblem
/-- "the chosen player" -/
semantic_template chosenPlayer : Predicate := .chosenPlayer .theChoice
/-- "the last chosen player" -/
semantic_template theLastChosenPlayer : Predicate := .chosenPlayer .theLatestChoice
/-- "of the chosen <quality>" -/
semantic_template ofChosen (sort : QualitySort) : Predicate := .ofChosen .theChoice sort
/-- "of the last chosen <quality>" -/
semantic_template ofTheLastChosen (sort : QualitySort) : Predicate := .ofChosen .theLatestChoice sort
semantic_template copyOfACard : Predicate := .isCopyOfACard
semantic_template cardOnTheStack : Predicate := .and [.isCard, spell]
semantic_template tokenOnTheBattlefield : Predicate := .and [.isToken, permanent]
/-- "that targets only <subject>" [CR#115.9c] -/
semantic_template targetsOnly (subject : NounPhrase) : Predicate := .targets subject .only
/-- The four party roles, in rule order [CR#700.8]. -/
semantic_template partyRoles : List Predicate :=
  [ .hasSubtype (.of .creature "Cleric"), .hasSubtype (.of .creature "Rogue"),
    .hasSubtype (.of .creature "Warrior"), .hasSubtype (.of .creature "Wizard") ]
/-- "any target" [CR#115.4]. -/
semantic_template anyTarget : Predicate :=
  .or [.hasType .creature, .hasType .planeswalker, .hasType .battle, .anyPlayer]
/-- "cast by <player>" -/
semantic_template castBy (player : NounPhrase) : Predicate := .castBy player none
/-- "the Nth spell <player> cast <period>" -/
semantic_template nthCastBy (ordinal : Ordinal) (player : NounPhrase) (period : Period) : Predicate :=
  .castBy player (some (ordinal, period))
/-- "any other target" -/
semantic_template anyOtherTarget : Predicate := .and [anyTarget, .other]
/-- "a color", "a creature type": a quality noun. -/
semantic_template quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
semantic_template qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
/-- "itself": the member a comparison over a domain is measuring (`semantics-v3.md` §1.3 Box 5). -/
semantic_template ownMember : NounPhrase := .pro .bare .one
/-- "a <dom> with <stat> <r> <bound>": the stat of `member`, the member itself unless written. -/
semantic_template comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator) (bound : Amount)
    (member : NounPhrase := ownMember) : Predicate :=
  .compareOver domain (.statOf (.stat stat) member) comparator bound
/-- "the chosen color" -/
semantic_template thatColor : ColorTerm := .chosen .theChoice
semantic_template creatureType (label : String) : Subtype := .of .creature label
semantic_template artifactType (label : String) : Subtype := .of .artifact label
semantic_template landType (label : String) : Subtype := .of .land label
semantic_template enchantmentType (label : String) : Subtype := .of .enchantment label
semantic_template spellType (label : String) : Subtype := .spell label
semantic_template planeswalkerType (label : String) : Subtype := .of .planeswalker label
/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, or Warlock [CR#700.12]. -/
semantic_template outlaw : Predicate :=
  .or [ .hasSubtype (creatureType "Assassin"), .hasSubtype (creatureType "Mercenary"),
        .hasSubtype (creatureType "Pirate"), .hasSubtype (creatureType "Rogue"),
        .hasSubtype (creatureType "Warlock") ]
semantic_template outlawYouControl : Predicate := .and [outlaw, .hasPossessor .controller .you]
/-- "an Army you control" -/
semantic_template armyYouControl : Predicate :=
  .and [.hasSubtype (creatureType "Army"), creature, .hasPossessor .controller .you]
semantic_template abilityHead (class_ : Semantics.AbilityClass) : Semantics.Predicate :=
  (Semantics.Predicate.abilityHead class_)
semantic_template abilityOf (source : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.abilityOf source)
semantic_template Predicate.activatedBy (activator : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.activatedBy activator)
semantic_template Predicate.and (conjuncts : (List Semantics.Predicate) := []) : Semantics.Predicate :=
  (Semantics.Predicate.and conjuncts)
semantic_template anotherSpell : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])
semantic_template anyPlayer : Semantics.Predicate :=
  Semantics.Predicate.anyPlayer
semantic_template Predicate.attachment (side : Semantics.AttachmentSide) (counterpart : (Option Semantics.NounPhrase) := none) (word : (Option Semantics.AttachWord) := none) : Semantics.Predicate :=
  (Semantics.Predicate.attachment side word counterpart)
semantic_template Predicate.bracketed (predicate : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.bracketed predicate)
semantic_template card : Semantics.Predicate :=
  Semantics.Predicate.isCard
semantic_template cardIn (zone : Semantics.Location) : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ zone)])
semantic_template castFrom (zone : Semantics.Location) : Semantics.Predicate :=
  (Semantics.Predicate.castFrom zone)
semantic_template choseExtreme (op : Semantics.AggregateOp) : Semantics.Predicate :=
  (Semantics.Predicate.choseExtreme op)
semantic_template coinCameUp (face : Semantics.CoinFace) : Semantics.Predicate :=
  (Semantics.Predicate.coinCameUp face)
semantic_template colorCount (comparator : Semantics.Comparator) (count : Nat) : Semantics.Predicate :=
  (Semantics.Predicate.colorCount comparator count)
semantic_template colorIs (color : Semantics.Color) : Semantics.Predicate :=
  (Semantics.Predicate.colorIs color)
semantic_template colorOf (object : Semantics.NounPhrase) : Semantics.ColorTerm :=
  (Semantics.ColorTerm.colorOf object)
semantic_template compare (comparator : Semantics.Comparator) (bound : Semantics.Amount) (axes : (List Semantics.ProjAxis) := []) : Semantics.Predicate :=
  (Semantics.Predicate.compare axes comparator bound)
semantic_template compareOver (domain : Semantics.Predicate) (measure : Semantics.Amount) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Predicate :=
  (Semantics.Predicate.compareOver domain measure comparator bound)
semantic_template controlledBy (player : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller player)
semantic_template counterKindOn (subject : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.counterKindOn subject)
semantic_template currentFace (side : Semantics.CardFaceSide) : Semantics.Predicate :=
  (Semantics.Predicate.currentFace side)
semantic_template exiledWith (source : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.in_ (Semantics.Location.exiledWith source))
semantic_template hasCounters (kind : (Option Semantics.CounterKind) := none) : Semantics.Predicate :=
  (Semantics.Predicate.hasCounters kind)
semantic_template hasDesignation (designation : Semantics.DesignationLabel) (holder : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.hasDesignation designation holder)
semantic_template hasKeyword (keyword : Semantics.KeywordTerm) : Semantics.Predicate :=
  (Semantics.Predicate.hasKeyword keyword)
semantic_template hasPossessor (axis : Semantics.PossessorAxis) (possessor : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor axis possessor)
semantic_template hasStatus (status : Semantics.Status) : Semantics.Predicate :=
  (Semantics.Predicate.hasStatus status)
semantic_template hasSubtype (subtype : Semantics.Subtype) : Semantics.Predicate :=
  (Semantics.Predicate.hasSubtype subtype)
semantic_template hasSupertype (supertype : Semantics.Supertype) : Semantics.Predicate :=
  (Semantics.Predicate.hasSupertype supertype)
semantic_template hasType (type : Semantics.CardType) : Semantics.Predicate :=
  (Semantics.Predicate.hasType type)
semantic_template inCombat (relation : Semantics.CombatRelation) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.inCombat relation counterpart)
semantic_template inPile (pile : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.in_ (Semantics.Location.pile pile))
semantic_template inZone (zone : Semantics.Location) : Semantics.Predicate :=
  (Semantics.Predicate.in_ zone)
semantic_template isCopyOfACard : Semantics.Predicate :=
  Semantics.Predicate.isCopyOfACard
semantic_template isEmblem : Semantics.Predicate :=
  Semantics.Predicate.isEmblem
semantic_template isManaAbility : Semantics.Predicate :=
  Semantics.Predicate.isManaAbility
semantic_template isSource : Semantics.Predicate :=
  Semantics.Predicate.isSource
semantic_template isToken : Semantics.Predicate :=
  Semantics.Predicate.isToken
semantic_template manaCostHas (symbol : Semantics.ManaSymbol) : Semantics.Predicate :=
  (Semantics.Predicate.manaCostHas symbol)
semantic_template named (source : Semantics.NameSource) : Semantics.Predicate :=
  (Semantics.Predicate.named source)
semantic_template Predicate.not (predicate : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.not predicate)
semantic_template notChosen : Semantics.Predicate :=
  Semantics.Predicate.notChosen
semantic_template ofYourChoice (sort : Semantics.QualitySort) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.Predicate :=
  (Semantics.Predicate.ofYourChoice sort domain)
semantic_template opponent : Semantics.Predicate :=
  (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)
semantic_template Predicate.or (disjuncts : (List Semantics.Predicate) := []) : Semantics.Predicate :=
  (Semantics.Predicate.or disjuncts)
semantic_template other : Semantics.Predicate :=
  Semantics.Predicate.other
semantic_template otherThan (anchor : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.otherThan anchor)
semantic_template ownedBy (player : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner player)
semantic_template planeswalker : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.planeswalker)
semantic_template qualityNoun (sort : Semantics.QualitySort) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.Predicate :=
  (Semantics.Predicate.qualityNoun sort domain)
semantic_template related (label : Semantics.RelationLabel) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.related label counterpart)
semantic_template shares (axis : Semantics.QualitySort) (agreement : Semantics.Agreement) (with_ : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.shares axis agreement with_)
semantic_template Predicate.spell : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])
semantic_template superlative (op : Semantics.AggregateOp) (axis : Semantics.ProjAxis) (domain : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.superlative op axis domain)
semantic_template targets (subject : Semantics.NounPhrase) (extent : Semantics.TargetExtent) : Semantics.Predicate :=
  (Semantics.Predicate.targets subject extent)
semantic_template wasCast : Semantics.Predicate :=
  (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.which Semantics.PlayPayment.itsOwnCost none none none) Semantics.LookbackSpan.thisWay))
semantic_template withMostVotes : Semantics.Predicate :=
  Semantics.Predicate.withMostVotes
semantic_template youControl : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)
semantic_template youOwn : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.you)
/-! ## Nouns -/

semantic_template anOpponent : NounPhrase := a (.isOpponentOf .you)
semantic_template thisCreature : NounPhrase := .asType .creature .this none
semantic_template thisArtifact : NounPhrase := .asType .artifact .this none
semantic_template thisLand : NounPhrase := .asType .land .this none
semantic_template thisPermanent : NounPhrase := .asMarker .permanent .this
semantic_template thisSpell : NounPhrase := .asMarker .spell .this
semantic_template thisRoom : NounPhrase := .asType .enchantment .this (some (enchantmentType "Room"))
semantic_template thisPlaneswalker : NounPhrase := .asType .planeswalker .this none
semantic_template thisAbility : NounPhrase := .asMarker .ability .this
semantic_template thisEnchantment : NounPhrase := .asType .enchantment .this none
semantic_template thisAura : NounPhrase := .asType .enchantment .this (some (enchantmentType "Aura"))
semantic_template thisEquipment : NounPhrase := .asType .artifact .this (some (artifactType "Equipment"))
semantic_template thisSiege : NounPhrase := .asType .battle .this (some (.of .battle "Siege"))
semantic_template thisVehicle : NounPhrase := .asType .artifact .this (some (artifactType "Vehicle"))
semantic_template thisSpacecraft : NounPhrase := .asType .artifact .this (some (artifactType "Spacecraft"))
semantic_template thisSaga : NounPhrase := .asType .enchantment .this (some (enchantmentType "Saga"))
semantic_template thisFortification : NounPhrase :=
  .asType .artifact .this (some (artifactType "Fortification"))
semantic_template thisCase : NounPhrase := .asType .enchantment .this (some (enchantmentType "Case"))
semantic_template thisClass : NounPhrase := .asType .enchantment .this (some (enchantmentType "Class"))
/-- "your commander" -/
semantic_template yourCommander : NounPhrase := .designated (.named "commander") .you
/-- "a card exiled with this artifact" -/
semantic_template exiledWithThisArtifact : Predicate := .in_ (.exiledWith thisArtifact)
/-- "the rest of them" -/
semantic_template theRest (kind : Kind) : NounPhrase := .theRest kind .many
/-- "the other pile": the one remaining after a choice among two. -/
semantic_template theOther (kind : Kind) : NounPhrase := .theRest kind .one
/-- "N of <group>" -/
semantic_template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) none group
/-- "among <group>": the whole of a group, sliced. -/
semantic_template among (group : NounPhrase) : NounPhrase := .someOf .whole none group
/-- "N <p> from among <group>" -/
semantic_template fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) (some p) group
/-- "<group> with the same name" -/
semantic_template withTheSameName (group : NounPhrase) : NounPhrase := .agree .cardName .share group
/-- "<group> with different names" -/
semantic_template withDifferentNames (group : NounPhrase) : NounPhrase := .agree .cardName .differ group
/-- "the <p> from among <group>": every member the description picks. -/
semantic_template allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase := .someOf .whole (some p) group
/-- "the <p> among <group>" -/
semantic_template allAmong (p : Predicate) (group : NounPhrase) : NounPhrase := allFromAmong p group
/-- "each object" -/
semantic_template everyObject : NounPhrase := allOf (.and [])
/-- "one pile": one of the piles just made. -/
semantic_template onePile : NounPhrase := .pileOf (.counted (exactly 1)) none
/-- "the pile of <player>'s choice" -/
semantic_template pileOfChoice (player : NounPhrase) : NounPhrase := .pileOf (.counted (exactly 1)) (some player)
/-- "you and <subject>" -/
semantic_template youAnd (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.both .you subject
/-- "you or <subject>" -/
semantic_template youOr (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.eitherOf .you subject
semantic_template controllerOf (subject : NounPhrase) : NounPhrase := .possessorOf .controller subject
semantic_template ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
semantic_template topSlice (amount : Amount) : NounPhrase := .slice .top amount (library .you)
/-- "the bottom card of your library" -/
semantic_template bottomCard : NounPhrase := .slice .bottom (.lit 1) (library .you)
/-- "[a player]'s party" [CR#700.8]. -/
semantic_template partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller player]))
/-- "your party" -/
semantic_template party : NounPhrase := partyOf .you
/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
semantic_template partySizeOf (player : NounPhrase) : Amount := .countOf (partyOf player)
semantic_template partySize : Amount := partySizeOf .you
/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
semantic_template fullPartyOf (player : NounPhrase) : Condition := .compareAmt (partySizeOf player) .eq (.lit 4)
semantic_template fullParty : Condition := fullPartyOf .you
semantic_template agree (axis : Semantics.QualitySort) (agreement : Semantics.Agreement) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.agree axis agreement group)
semantic_template NounPhrase.and (phrases : (List Semantics.NounPhrase) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.and phrases)
semantic_template asMarker (marker : Semantics.MarkerWord) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asMarker marker subject)
semantic_template asType (type : Semantics.CardType) (subject : Semantics.NounPhrase) (subtype : (Option Semantics.Subtype) := none) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType type subject subtype)
semantic_template attachHost (word : Semantics.AttachWord) (head : Semantics.NounWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.attachHost word head)
semantic_template becomes (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.becomes subject)
semantic_template combatPlayer (role : Semantics.CombatRole) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.combatPlayer role)
semantic_template counterpart (label : Semantics.RelationLabel) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.counterpart label subject)
semantic_template defendingPlayer : Semantics.NounPhrase :=
  (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending)
semantic_template described (determiner : Semantics.DetPhrase) (predicate : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described determiner predicate)
semantic_template designated (designation : Semantics.DesignationLabel) (whose : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.designated designation whose)
semantic_template eachOf (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.eachOf group)
set_option linter.unusedVariables false in
semantic_template gap (kind : Semantics.Kind) : Semantics.NounPhrase :=
  Semantics.NounPhrase.which
semantic_template oneEachOf (pool : Semantics.NounPhrase) (roles : (List Semantics.Predicate) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.oneEachOf roles pool)
semantic_template NounPhrase.or (phrases : (List Semantics.NounPhrase) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.or phrases)
semantic_template pileOf (count : Semantics.SliceCount) (by_ : (Option Semantics.NounPhrase) := none) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pileOf count by_)
semantic_template possessorOf (axis : Semantics.PossessorAxis) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.possessorOf axis subject)
semantic_template resolvedPermanent (spell : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.resolvedPermanent spell)
semantic_template run (end_ : Semantics.PileEnd) (stop : Semantics.Predicate) (pile : Semantics.Location) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.run end_ stop pile)
semantic_template slice (end_ : Semantics.PileEnd) (amount : Semantics.Amount) (pile : Semantics.Location) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.slice end_ amount pile)
semantic_template theGrantor (marker : Semantics.MarkerWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.theGrantor marker)
semantic_template themAsTokens : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)
semantic_template this_ : Semantics.NounPhrase :=
  Semantics.NounPhrase.this
semantic_template you : Semantics.NounPhrase :=
  Semantics.NounPhrase.you
/-! ## Mana -/

semantic_template generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
semantic_template pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
/-- "{C}" -/
semantic_template colorlessPip : ManaSymbol := .simple (.specific .colorless)
/-- "{A/B}" -/
semantic_template hybridPip (left right : Color) : ManaSymbol := .hybrid (.specific (.of left)) right
/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
semantic_template phyrexianPip (color : Color) : ManaSymbol := .phyrexian color none
/-- "{1} for each …": generic mana cost scaled by an amount. -/
semantic_template scaledGenericMana (amount : Amount) : Cost :=
  .scaled (.mana [generic 1]) amount
semantic_template ManaSymbol.generic (amount : Nat) : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic amount))
/-! ## Durations -/

/-- "until end of turn" -/
semantic_template untilEndOfTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "this turn", as an effect lasts: until the turn ends. -/
semantic_template thisTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "until your next turn" -/
semantic_template untilYourNextTurn : Duration :=
  .between none (some (.beginningOf .turn (some .you) (.next none)))
/-- "until end of combat" -/
semantic_template untilEndOfCombat : Duration := .between none (some (.endOf .combat none .current))
/-- "until your next end step" -/
semantic_template untilYourNextEndStep : Duration :=
  .between none (some (.beginningOf .endStep (some .you) (.next none)))
/-- "until <player>'s next turn" -/
semantic_template untilNextTurnOf (player : NounPhrase) : Duration :=
  .between none (some (.beginningOf .turn (some player) (.next none)))
/-- "until <event>" -/
semantic_template untilEvent (event : GameEvent) : Duration := .between none (some event)
/-- "for the rest of the game" -/
semantic_template restOfGame : Duration := .between none none
/-- "during <player>'s next turn" (`firstMention` / `laterMention` so `player` is not duplicated
[CR#115.1,608.2c]). -/
semantic_template duringNextTurnOf (player : NounPhrase) : Duration :=
  .between
    (some (.beginningOf .turn (some (.firstMention "duringNextTurnOf.player" player)) (.next none)))
    (some (.endOf .turn (some (.laterMention "duringNextTurnOf.player" player)) (.next none)))
/-- "on a later turn": from the end of this turn on [CR#702.143a,702.170d]. -/
semantic_template onALaterTurn : Duration := .between (some (.endOf .turn none .current)) none
semantic_template between (start : Semantics.GameEvent) (end_ : Semantics.GameEvent) : Semantics.Duration :=
  (Semantics.Duration.between (some start) (some end_))
semantic_template forAsLongAs (condition : Semantics.Condition) : Semantics.Duration :=
  (Semantics.Duration.forAsLongAs condition)
/-! ## History windows

A lookback's span ends at `now` ("this turn" so far), except "last turn", which ended. -/

/-- "this turn", as a lookback reads it: from the turn's beginning to now. -/
semantic_template soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "earlier this turn" -/
semantic_template earlierThisTurn : Duration :=
  .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this combat" -/
semantic_template thisCombat : Duration := .between (some (.beginningOf .combat none .current)) (some .now)
/-- "last turn" -/
semantic_template lastTurn : Duration :=
  .between (some (.beginningOf .turn none .previous)) (some (.endOf .turn none .previous))
/-- "this game" -/
semantic_template thisGame : Duration := .between none (some .now)
/-! ## Timings -/

/-- "only as a sorcery": during your main phase, while the stack is empty [CR#307.5]. -/
semantic_template asSorcery : Timing :=
  .and [.inPart .mainPhase (some .you) .current, .onlyIf (.not (exists_ (.in_ stack)))]
/-- "only as an instant": any time you have priority [CR#117.1a]. -/
semantic_template asInstant : Timing := .onlyIf (.hasPriority .you)
/-- "during <whose> <part>" -/
semantic_template duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .inPart part whose .current
/-- "before <whose> <part>" -/
semantic_template beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .relativeToEvent .before (.beginningOf part whose .current)
/-- "during each of your turns" -/
semantic_template duringEachOfYourTurns : Timing := .inPart .turn (some .you) (.each none)
/-- "while you're searching your library" -/
semantic_template whileSearchingYourLibrary : Timing :=
  .whileDoing (.does (.actor .you) (.search (yourLibrary) anyNumber (.and [])))
/-- "while <event>" -/
semantic_template whileDoing (event : GameEvent) : Timing := .whileDoing event
/-- "while <condition>" -/
semantic_template whileTrue (condition : Condition) : Timing := .onlyIf condition
semantic_template Timing.and (conjuncts : (List Semantics.Timing) := []) : Semantics.Timing :=
  (Semantics.Timing.and conjuncts)
semantic_template Timing.not (timing : Semantics.Timing) : Semantics.Timing :=
  (Semantics.Timing.not timing)
semantic_template Timing.or (disjuncts : (List Semantics.Timing) := []) : Semantics.Timing :=
  (Semantics.Timing.or disjuncts)
/-! ## Usage limits -/

/-- "only once each turn" [CR#602.5b] -/
semantic_template oncePerTurn : UsageLimit := ⟨.moreThan (.lit 1), .turn⟩
/-- "only once each game" -/
semantic_template oncePerGame : UsageLimit := ⟨.moreThan (.lit 1), .game⟩
/-- "only once during each of your turns" -/
semantic_template onceEachYourTurn : UsageLimit := ⟨.moreThan (.lit 1), .yourTurn⟩
/-- "Activate only once": no more than one while the object exists [CR#602.5b,702.177a]. -/
semantic_template oncePerObject : UsageLimit := ⟨.moreThan (.lit 1), .thisObject⟩
/-- "<instruction>. Do this only once each turn." [CR#603.2h] -/
semantic_template doOnlyOnceEachTurn (instruction : Action) : Action :=
  .limited instruction oncePerTurn
/-! ## Amounts -/

semantic_template lifeTotalOf (player : NounPhrase) : Amount := .statOf (.playerStat .lifeTotal) player
/-- "<player>'s speed" [CR#702.179a]. -/
semantic_template speedOf (player : NounPhrase) : Amount := .statOf (.playerStat .speed) player
semantic_template powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
semantic_template toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
semantic_template countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := .statOf (.counter kind) holder
/-- "<saga>'s final chapter number": the greatest of its chapter numbers, 0 with none
[CR#714.2d]. -/
semantic_template finalChapterNumber (saga : NounPhrase) : Amount := .aggregate .max .chapter saga
/-- "a chapter ability of <saga>" [CR#714.2,714.2a] -/
semantic_template chapterAbilityOf (saga : NounPhrase) : Predicate :=
  .and [.abilityOf saga, .compare [.chapter] .atLeast (.lit 1)]
semantic_template plus (left right : Amount) : Amount := .arith .plus left right
semantic_template minus (left right : Amount) : Amount := .arith .minus left right
semantic_template times (per amount : Amount) : Amount := .arith .times per amount
/-- "the smaller of <a> and <b>", "<a> or <b>, whichever is smaller" [CR#704.5q]. -/
semantic_template smallerOf (a b : Amount) : Amount := .aggregateOf .min [a, b]
/-- "the greater of <a> and <b>", "<a> or <b>, whichever is greater". -/
semantic_template greaterOf (a b : Amount) : Amount := .aggregateOf .max [a, b]
/-- "that much damage prevented this way" -/
semantic_template preventedThisWay : Amount := .theOutcome .damagePrevented
/-- "N for each <p>" -/
semantic_template forEach (per : Nat) (p : Predicate) : Amount := times (.lit per) (countOf p)
semantic_template aggregateOf (op : Semantics.AggregateOp) (amounts : (List Semantics.Amount) := []) : Semantics.Amount :=
  (Semantics.Amount.aggregateOf op amounts)
semantic_template aggregateOver (op : Semantics.AggregateOp) (domain : Semantics.Predicate) (body : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.aggregateOver op domain body)
semantic_template arith (op : Semantics.ArithOp) (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.arith op left right)
semantic_template coinsShowing (face : Semantics.CoinFace) : Semantics.Amount :=
  (Semantics.Amount.coinsShowing face)
semantic_template devotion (player : Semantics.NounPhrase) (color : Semantics.ColorTerm) (second : (Option Semantics.ColorTerm) := none) : Semantics.Amount :=
  (Semantics.Amount.devotion player color second)
semantic_template distinctCount (axis : Semantics.KindAxis) (domain : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.distinctCount axis domain)
semantic_template down (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.down amount)
semantic_template eventTally (clause : Semantics.DidClause) (mode : Semantics.TallyMode) : Semantics.Amount :=
  (Semantics.Amount.tally clause mode)
semantic_template greatestStoredMatch (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.greatestStoredMatch subject)
semantic_template groupSize : Semantics.Amount :=
  Semantics.Amount.groupSize
semantic_template half (rounding : Semantics.RoundMode) (amount : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.half rounding amount)
semantic_template letter (letter : Semantics.Letter) : Semantics.Amount :=
  (Semantics.Amount.letter letter)
semantic_template lit (value : Semantics.Amount) : Semantics.Amount :=
  value
semantic_template numberOf (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.countOf subject)
semantic_template paid (facet : Semantics.LinkedFacet) (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.paid facet subject)
semantic_template set (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.set amount)
semantic_template statOf (axis : Semantics.ProjAxis) (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf axis subject)
semantic_template thatMuch : Semantics.Amount :=
  Semantics.Amount.thatMuch
semantic_template theDifference : Semantics.Amount :=
  Semantics.Amount.theDifference
semantic_template theOutcome (sort : Semantics.OutcomeSort) : Semantics.Amount :=
  (Semantics.Amount.theOutcome sort)
semantic_template totalOf (axis : Semantics.ProjAxis) (group : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.aggregate Semantics.AggregateOp.sum axis group)
semantic_template up (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.up amount)
semantic_template Amount.upTo (bound : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.upTo bound)
semantic_template votesFor (label : String) : Semantics.Amount :=
  (Semantics.Amount.votesFor label)
/-! ## Counters -/

/-- What one +X/+Y counter confers [CR#122.1a]: "+X/+Y" to its bearer [CR#613.4c]. A counter
definition's conferral is per counter (`Definition.counter`), so the deltas read no count. -/
semantic_template boost (power toughness : Delta Amount) : Conferral :=
  .property (.ptModification .this power toughness)
/-- What one keyword counter confers [CR#122.1b]: its bearer gains the keyword [CR#613.1f], named
by its declaration's name (`grants (.named "flying")`). -/
semantic_template grants (keyword : KeywordLabel) : Conferral :=
  .property (.abilityGrant .this (.keyword keyword [] []))
/-- What the creature type confers [CR#508.1a]: its object is a Combatant. -/
semantic_template combatant : Conferral := .property (.combatant .this)
/-! ## Actions -/

semantic_template move (subject : NounPhrase) (destination : Location) : Action :=
  .move subject .anyLocation (some destination) .arrival none []
/-- A card exiled face down: no characteristics [CR#406.3a]. -/
semantic_template exiledFaceDown : FaceDown := ⟨none⟩
/-- A 2/2 face-down creature with no text, no name, no subtypes and no mana cost [CR#708.2a]. -/
semantic_template faceDownCreature : FaceDown :=
  ⟨some { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) }⟩
/-- A 2/2 face-down creature with ward {2}, no name, no subtypes and no mana cost
[CR#701.58a,702.168a]. -/
semantic_template faceDownWardedCreature : FaceDown :=
  ⟨some { types := [.creature], text := [.keyword (.named "ward") [.cost (.mana [generic 2])] []],
          power := some (.lit 2), toughness := some (.lit 2) }⟩
/-- "put <subject> <destination>" -/
semantic_template put (subject : NounPhrase) (destination : Location) : Action :=
  .enact (.core .put) (.move subject .anyLocation (some destination) .arrival none []) none none
/-- "put <subject> <destination>" with the move's riders ("second from the top", "on the top or
bottom") -/
semantic_template putWith (subject : NounPhrase) (destination : Location) (riders : List MoveRider) :
    Action :=
  .enact (.core .put) (.move subject .anyLocation (some destination) .arrival none riders) none none
/-- "return <subject> to <zone>" -/
semantic_template returnTo (subject : NounPhrase) (destination : Location) (riders : List MoveRider) :
    Action :=
  .enact (.core .return_) (.move subject .anyLocation (some destination) .arrival none riders) none none
/-- "return <subject> to the battlefield" -/
semantic_template returnToBattlefield (subject : NounPhrase) : Action := returnTo subject battlefield []
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
semantic_template returnToBattlefieldTransformed (subject controller : NounPhrase) : Action :=
  returnTo subject battlefield [.entersTransformed, (.under controller)]
/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
semantic_template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  .move subject .anyLocation (some battlefield) .arrival none
    [(.under who), (.withCounters amount (.printed kind) .fresh)]
semantic_template putOntoBattlefield (subject : NounPhrase) : Action :=
  .move subject .anyLocation (some battlefield) .arrival none []
semantic_template putOntoBattlefieldTapped (subject : NounPhrase) : Action :=
  .move subject .anyLocation (some battlefield) .arrival none [(.entersAs .tapped)]
/-- "put <subject> onto the battlefield under your control" -/
semantic_template putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Action :=
  .move subject .anyLocation (some battlefield) .arrival none [(.under .you)]
/-- "put <subject> onto the battlefield tapped and attacking" -/
semantic_template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := .anyLocation) : Action :=
  .move subject from_ (some battlefield) .arrival none
    [(.entersAs .tapped), (.entersAttacking none)]
/-- Losing counters is removal from the named player. -/
semantic_template loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Action :=
  .removeCounters (amount.map Quantity.exactlyOf) kind agent .one true
semantic_template loseLife (amount : Amount) : Action :=
  .changeLife (.down amount)
semantic_template gainLife (amount : Amount) : Action :=
  .changeLife (.up amount)
/-- "<player>'s life total becomes <amount>" -/
semantic_template setLife (amount : Amount) : Action :=
  .changeLife (.set amount)
semantic_template draw (amount : Amount) : Action := .draw amount
semantic_template lookAt (cards : NounPhrase) : Action := .expose .lookAt (.cards cards)
/-- "look at <player>'s hand" -/
semantic_template lookAtHandOf (player : NounPhrase) : Action :=
  .expose .lookAt (.zone (handOf player))
semantic_template doIf (condition : Condition) (instruction : Action)
    (otherwise : Option Action := none) : Action :=
  .doIf condition instruction otherwise
/-- "choose <subject>" -/
semantic_template choose (subject : NounPhrase) (disclosure : Disclosure := .openly) : Action :=
  .choose none subject disclosure none
/-- "choose <subject> as you <event>" -/
semantic_template chooseWhile (subject : NounPhrase) (while_ : Timing) : Action :=
  .choose none subject .openly (some while_)
semantic_template rollDice (count : Nat) (sides : Nat) : Action :=
  .rollDice (.lit count) (.sides sides) .one .anyResult
/-- One row of a results table: "<results> — <instruction>". -/
semantic_template rollRow (results : Quantity) (instruction : Action) : RollRow := ⟨results, instruction⟩
semantic_template flipCoins (count : Nat) : Action :=
  .flipCoins (.count (.lit count)) none none
/-- "<player> flips a coin" as an event -/
semantic_template flipsCoin (player : NounPhrase) : GameEvent :=
  .does (.actor player) (.flipCoins (.count (.lit 1)) none none)
/-- "may <body>" -/
semantic_template offer (body : Action) : Action := Primitives.Action.offer body none none
/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
semantic_template offerWhen (body : Action) (triggerBody : Action) : Action :=
  .triggerReflexively (Primitives.Action.offer body none none) triggerBody
/-- "the chosen number" -/
semantic_template chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
semantic_template chooseSpree (modes : List (Option Cost × Action)) : Action :=
  .chooseModes (atLeast 1) (modes.map fun (c, e) => (c, none, e))
/-- "Choose N — <modes>", no mode costing anything. -/
semantic_template chooseModes (quantity : Quantity) (modes : List Action) : Action :=
  .chooseModes quantity (modes.map (none, none, ·))
/-- "<source> deals damage equal to its power to <recipient>" (`Template`, `firstMention` /
`laterMention` without `Window.top`). -/
semantic_template dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Action :=
  .instruct (.firstMention "dealDamageOwnPower.source" source)
    (.dealDamage (.statOf (.stat .power) (.laterMention "dealDamageOwnPower.source" source))
      recipient .any)
/-- A token's characteristics from the parts a creature token names. -/
semantic_template creatureTokenOf (power toughness : Amount) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.creature], subtypes, power := some power,
        toughness := some toughness } }
semantic_template creatureToken (power toughness : Nat) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes
/-- "for each color of mana spent to cast <n>" -/
semantic_template colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
semantic_template coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)
/-- "if no colored mana was spent to cast <spell>" -/
semantic_template noColoredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .eq (.lit 0)
/-- "the amount of mana spent to cast <spell>" -/
semantic_template manaValueSpentToCast (spell : NounPhrase) : Amount := .paid .manaValueSpent spell
/-- "if no mana was spent to cast <spell>" -/
semantic_template noManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (manaValueSpentToCast spell) .eq (.lit 0)
/-- "between N and M" -/
semantic_template fromTo (low high : Nat) : Quantity := .range (some low) (some high)
/-- The participant described by the nearest enclosing lookback (`NounPhrase.which`). -/
semantic_template relative (_kind : Kind) : NounPhrase := .which
/-- The number of occurrences of a historical action clause. -/
semantic_template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally ⟨subj, action, .duration within timing⟩ .events
/-- A historical action clause. -/
semantic_template happened (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Condition :=
  .did ⟨subj, action, .duration within timing⟩
/-- The action the enclosing instruction caused: "if you searched your library this way". -/
semantic_template happenedThisWay (action : Action) (subj : Subject) : Condition :=
  .did ⟨subj, action, .thisWay⟩
/-- A historical action clause whose `.which` stands for the entity being described. -/
semantic_template happenedTo (subj : Subject) (action : Action) (within : Duration)
    (timing : Option Timing := none) : Predicate :=
  .did ⟨subj, action, .duration within timing⟩
/-- The accumulated magnitude of historical action events. -/
semantic_template eventSum (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally ⟨subj, action, .duration within timing⟩ .quantity
/-- "if there is no <designation>" -/
semantic_template thereIsNo (designation : DesignationLabel) : Condition := .noHolder designation
/-- "<subject>'s <keyword> cost was paid", read back. -/
semantic_template paidCostRead (which : LinkSource) (window : Option Duration) (subject : NounPhrase) :
    Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
semantic_template timesPaid (which : LinkSource) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "the player chosen as <subject>'s <label> cost was paid", read back by the linked ability
[CR#607.2d,702.174a] -/
semantic_template chosenAsPaid (label : KeywordLabel) (subject : NounPhrase) : NounPhrase :=
  .paid (.chosenPlayer (.byKeyword label)) subject
/-- "if <subject>'s <cost> was paid" -/
semantic_template costWasPaid (which : LinkSource) (window : Option Duration) (subject : NounPhrase) :
    Condition :=
  .compareAmt (paidCostRead which window subject) .atLeast (.lit 1)
/-- "the sacrificed creature", "the exiled card": the one object `deed` acted on as the Link
Source `which` of `subject` was paid or resolved [CR#607.2a]. -/
semantic_template linkedObject (which : LinkSource) (deed : Deed) (subject : NounPhrase) : NounPhrase :=
  .paid (.objects which deed .one) subject
/-- "the exiled cards", "creatures it devoured": the objects `deed` acted on as the Link Source
`which` of `subject` was paid or resolved [CR#607.2a,702.82b]. -/
semantic_template linkedObjects (which : LinkSource) (deed : Deed) (subject : NounPhrase) :
    NounPhrase :=
  .paid (.objects which deed .many) subject
/-- "if <subject>'s <which> was taken": the choice the Link Source offered was taken
[CR#702.104b]. -/
semantic_template choiceWasTaken (which : LinkSource) (subject : NounPhrase) : Condition :=
  .compareAmt (.paid (.taken which) subject) .atLeast (.lit 1)
/-- "<object>'s mana cost" as the mana a cost shift changes by [CR#702.48a]. -/
semantic_template manaCostOf (object : NounPhrase) : ManaCostTerm := .manaCostOf object
/-- "<subject>'s total cost is reduced by <cost>", symbol by symbol [CR#702.48c,118.7]. -/
semantic_template reducedBySymbols (subject : NounPhrase) (cost : ManaCostTerm) : StaticSpec :=
  .costShift subject (.bySymbols cost false false)
/-- "the last chosen color" -/
semantic_template theLastChosenColor : ColorTerm := .chosen .theLatestChoice
/-- "the last chosen number" -/
semantic_template theLastChosenNumber : Amount := .chosenNumber .theLatestChoice
/-- "the amount by which the ceiling was not reached" -/
semantic_template shortOfCeiling : Amount := .theOutcome .ceilingShortfall
/-- "the number of counters removed this way" -/
semantic_template removedThisWay : Amount := .theOutcome .countersRemoved
/-- "increase or decrease the result by N" -/
semantic_template shiftResult (amount : Amount) : Action := .shiftResult none amount
/-- Map a legacy `Deed` + patient to its `v3` `Action` for deontic helper templates. -/
def deonticActionFor (deed : Deed) (patient : NounPhrase) : Action :=
  match deed with
  | .core .attack => .attack (match patient with | .any => none | p => some p)
  | .core .block => .block patient
  | .core .target => .target patient
  | .core .copy => .copy .fromStack patient .anyAmount []
  | .core .trigger => .trigger
  | .core .fullyUnlock => .fullyUnlock patient
  | .core .unlock => .unlock (.doorOf none patient)
  | .ofAbility (.named "crew") => .crew patient
  | .ofAbility (.named "saddle") => .saddle patient
  | .ofAbility (.named "phasing") => .setStatus .phasedIn patient
  | .core .draw => .draw .anyAmount
  | .core .gainLife => .changeLife (.up .anyAmount)
  | .core .loseGame => .conclude .loseGame
  | .core .winGame => .conclude .winGame
  | .core .put => .putCounters .anyAmount .anyKind patient .one false
  | .core .return_ => .move patient .anyLocation (some (.zone .battlefield none)) .arrival none []
  | .core .gainControl => .changeControl patient
  | .core .tapForMana => .tapForMana patient none
  | .action "tap" => .setStatus .tapped patient
  | .action "untap" => .setStatus .untapped patient
  | .action "destroy" =>
    .ownScope (some (.action "destroy"))
      (.enact (.action "destroy")
        (.move (.inCaller patient) (.zone .battlefield none) (some (.zone .graveyard none)) .departure none [])
        none none)
  | .action "sacrifice" =>
    .ownScope (some (.action "sacrifice"))
      (.enact (.action "sacrifice")
        (.move (.inCaller patient) (.zone .battlefield none) (some (.zone .graveyard none)) .departure none [])
        none none)
  | .action "regenerate" =>
    .ownScope (some (.action "regenerate"))
      (.enact (.action "regenerate") (.clearDamage (.inCaller patient)) none none)
  | .action "counter" =>
    .ownScope (some (.action "counter"))
      (.enact (.action "counter")
        (.move (.inCaller patient) (.zone .stack none) (some (.zone .graveyard none)) .arrival none [])
        none none)
  | .action "cast" => .procedure .cast patient .itsOwnCost none none none
  | .action "play" => .procedure .play patient .itsOwnCost none none none
  | .action "activate" => .procedure .activate patient .itsOwnCost none none none
  | .action "attach" => .attachment .attached .this (some patient)
  | .action "search" => .search (.anyLocation) (.range none none) (.and [])
  | .action "transform" => .turnOver patient
  | .action name =>
    .ownScope (some (.action name)) (.enact (.action name) (.clearDamage patient) none none)
  | _ => .setStatus .tapped patient
/-- "<player> may play N additional lands" -/
semantic_template mayPlayAdditionalLands (player : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .can (.actor player) (.procedure .play (counted (.additional quantity) land) .itsOwnCost none none none) {}
/-- The procedure a play deed names: casting, activating, or playing [CR#701.5a,701.2a,701.18a]. -/
def procedureOfDeed : Deed → Procedure
  | .action "cast" => .cast
  | .action "activate" => .activate
  | _ => .play
/-- "<player> may <deed> <what> [from <zone>] [<timing>] [paying …] [as though …]" -/
semantic_template mayPlayDeed (deed : Deed) (player what : NounPhrase) (asThough : Option AsThough)
    (from_ : Option Location) (timing : Option Timing) (payment : PlayPayment) : StaticSpec :=
  .can (.actor player) (.procedure (procedureOfDeed deed) what payment none from_ none)
    { asThough, timing }
/-- "You may play this card from <zone> <timing>": the land type's play permission [CR#305.1]. -/
semantic_template playableFrom (zone : Location) (timing : Timing) : Conferral :=
  .property (mayPlayDeed (.action "play") .you .this none (some zone) (some timing) .itsOwnCost)
/-- "You may cast this card from <zone> <timing>": the instant type's cast permission
[CR#304.1]. -/
semantic_template castableFrom (zone : Location) (timing : Timing) : Conferral :=
  .property (mayPlayDeed (.action "cast") .you .this none (some zone) (some timing) .itsOwnCost)
/-- "When <event>, <instruction>" as a delayed trigger. -/
semantic_template delay (event : GameEvent) (instruction : Action)
    (alternatives : List GameEvent := []) : Action :=
  .delay event alternatives none instruction
/-- "When <event> <duration>, <instruction>": a delayed trigger with a window. -/
semantic_template delayWithin (event : GameEvent) (duration : Duration) (instruction : Action) :
    Action :=
  .delay event [] (some duration) instruction
/-- "<subject> phases out until <event>" -/
semantic_template phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (.setStatus .phasedOut subject) event
/-- "take <count> extra turns after this one": the performer takes them [CR#500.7]. -/
semantic_template extraTurn (count : Amount) : Action := .addTurnPart .turn none count none
/-- "there is an additional <part> [after <anchor>] [, followed by an additional <next>]": phases
added to the current turn [CR#500.8]. -/
semantic_template additionalPhase (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart := none) : Action :=
  .addTurnPart part anchor count followedBy
/-- "there is an additional <part> [after <anchor>]": steps added to the current turn's phase
[CR#500.9]. -/
semantic_template additionalStep (part : TurnPart) (anchor : Option TurnPart) (count : Amount) :
    Action :=
  .addTurnPart part anchor count none
/-- "<subject> can't attack [this turn]" -/
semantic_template forbidAttack (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant (.actor subject) (.attack none) {}) duration
/-- "<subject> can't block [this turn]" -/
semantic_template forbidBlock (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant (.actor subject) (.block .any) {}) duration
/-- "<subject> can't be blocked [this turn]" -/
semantic_template forbidBeingBlocked (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant .any (.block subject) {}) duration
/-- "<subject> blocks it this turn if able": ordinary pronoun `it` resolved by `D′` (no `Window`
override). -/
semantic_template requireBlockIt (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.must (.actor subject) (.block it) {}) duration
/-- "<source> deals N damage divided as you choose among <among>" -/
semantic_template dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) : Action :=
  .distribute (.damage source) amount among
/-- "distribute N <kind> counters among <among>" -/
semantic_template distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) : Action :=
  .distribute (.counters kind) amount among
/-- "remove <q> <kind> counters from <from>" -/
semantic_template removeCounters (quantity : Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Action :=
  .removeCounters (some quantity) kind from_ .one true
/-- "<who> loses all [<kind>] counters" -/
semantic_template loseAllCounters (kind : Option CounterKindSource) (agent : NounPhrase := Primitives.NounPhrase.you) : Action :=
  loseCounters kind none (agent := agent)
/-- "remove all [<kind>] counters from <from>" -/
semantic_template removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) : Action :=
  .removeCounters none kind from_ .one true
/-- "may pay <cost>. If they don't, <instruction>." -/
semantic_template doUnless (instruction : Action) (cost : Cost) : Action :=
  Primitives.Action.offer (.pay cost .once .paid none none) none (some instruction)
/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
semantic_template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .ptModification subject power toughness
/-- "<subject> has base power and toughness P/T" -/
semantic_template getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPt subject (.set power) (.set toughness)
/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` without `Window`
plumbing). -/
semantic_template lookAndSortSlice (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider := []) : Action :=
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anyLocation
        (some spill) .arrival none spillRiders,
      .move (theRest .object) .anyLocation (some onTop) .arrival none
        [(.inOrder .anyOrder)] ]
/-- "look at the top N cards of <whose> library, put any number of them on the bottom in any order
and the rest on top in any order" (`Template` using `firstMention` / `laterMention` without
`Window` plumbing). -/
semantic_template lookAndSort (whose : NounPhrase) (amount : Amount) : Action :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anyLocation
        (some onBottom) .arrival none [(.inOrder .anyOrder)],
      .move (theRest .object) .anyLocation (some onTop) .arrival none
        [(.inOrder .anyOrder)] ]
/-- The same look, spilling the cards put aside into <spill> instead of the bottom. -/
semantic_template lookAndSortInto (whose : NounPhrase) (amount : Amount) (spill : Location) :
    Action :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) spill,
      .move (theRest .object) .anyLocation (some onTop) .arrival none
        [(.inOrder .anyOrder)] ]
semantic_template get (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Action :=
  .establish (getsPt subject power toughness) duration
semantic_template gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Action :=
  .establish (.abilityGrant subject ability) duration
/-- "<subject> gains haste [until …]" -/
semantic_template gainHaste (subject : NounPhrase) (duration : Option Duration) : Action :=
  gain subject (.keyword (.named "haste") [] []) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
semantic_template become (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration) :
    Action :=
  .establish (Primitives.StaticSpec.qualityChange subject .adds (.bundle added none)) duration
/-- "<subject> becomes <colors> [until …]" -/
semantic_template becomeColor (subject : NounPhrase) (colors : ColorSpec) (duration : Option Duration) :
    Action :=
  .establish (Primitives.StaticSpec.qualityChange subject .sets (.colored colors)) duration
/-- Several static clauses sharing one subject, as one instruction. -/
semantic_template establishFor (subject : NounPhrase) (parts : List StaticSpec) (duration : Option Duration) :
    Action :=
  .establish (.conjunction (some subject) parts) duration
/-- "If <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceEvent (clause : WouldClause) (replacement : Action) (duration : Option Duration) :
    Action :=
  .establish (.instead clause replacement none .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt <scope> [duration]." -/
semantic_template preventAll (kind : DamageKind) (scope : DamageScope) (duration : Option Duration) :
    Action :=
  .establish (.damageRule kind .unattributed scope (.prevent .all none) .repeatedly) duration
/-- "Prevent the next N <kind> damage that would be dealt <scope> [duration]." -/
semantic_template preventNext (kind : DamageKind) (scope : DamageScope) (amount : Amount)
    (duration : Option Duration) : Action :=
  .establish (.damageRule kind .unattributed scope (.prevent (.shield amount) none) .repeatedly)
    duration
/-- "Prevent all <kind> damage that would be dealt by <source> <scope> [duration]." -/
semantic_template preventAllBy (kind : DamageKind) (source : NounPhrase) (scope : DamageScope)
    (duration : Option Duration) : Action :=
  .establish (.damageRule kind (.dealtBy source) scope (.prevent .all none) .repeatedly) duration
/-- "The next time <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceNextEvent (clause : WouldClause) (replacement : Action)
    (duration : Option Duration) : Action :=
  .establish (.instead clause replacement none .nextTimeOnly none) duration
/-- "<player> gains control of <subject> [duration]" -/
semantic_template gainControl (subject : NounPhrase) (duration : Option Duration) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Action :=
  .establish (.controlGrant agent subject) duration
/-- "<subject> can't be <deed>ed" -/
semantic_template objectCant (deed : Deed) (subject : NounPhrase) : StaticSpec :=
  .cant .any (deonticActionFor deed subject) {}
/-- "<player> can't <deed>" -/
semantic_template playerCant (deed : Deed) (player : NounPhrase) : StaticSpec :=
  .cant (.actor player) (deonticActionFor deed .any) {}
/-- "<spec> as long as <condition>" -/
semantic_template onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .asLongAs
/-- "<spec> unless <condition>" -/
semantic_template onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec (.not condition) .unless_
/-- "<spec> if <condition>" -/
semantic_template onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .ifSo
/-- "<who> can't <deed> <what>" -/
semantic_template cantDoTo (deed : Deed) (who what : NounPhrase) : StaticSpec :=
  .cant (.actor who) (deonticActionFor deed what) {}
/-- "<n> can <deed> as though it didn't have <p>" -/
semantic_template canDoAsThough (n : NounPhrase) (deed : Deed) (p : Predicate) : StaticSpec :=
  .can (.actor n) (deonticActionFor deed .any) { asThough := some (.condition (.matches (.pro .bare .one) p)) }
/-- "<what> can't be the target of <by>" -/
semantic_template cantBeTargetedBy (what by_ : NounPhrase) : StaticSpec :=
  .cant (.by by_) (.target what) {}
/-- "<what> can be the target of <by> as though it didn't have <p>" -/
semantic_template canBeTargetedAsThough (what by_ : NounPhrase) (p : Predicate) : StaticSpec :=
  .can (.by by_) (.target what) { asThough := some (.condition (.matches (.pro .bare .many) p)) }
/-- "<spec> <duration>": a clause holding for a stated duration. -/
semantic_template establishThroughout (spec : StaticSpec) (duration : Duration) : Action :=
  .establish spec (some duration)
/-- "<n> doesn't untap during [<whose>] untap step" -/
semantic_template doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose (objectCant (.action "untap") subject)
/-- "<subject> can block an additional creature each combat" -/
semantic_template mayBlockAdditional (subject : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .can (.actor subject) (.block (counted (.additional quantity) creature)) {}
/-- "<who> may vote an additional time" -/
semantic_template mayVoteAdditional (who : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .can (.actor who)
    (.ownScope (some (.action "vote"))
      (.enact (.action "vote") (.vote none .openly .anyBallot (some (.additional quantity))) none none))
    {}
/-- "<who> may spend mana as though it were mana of <as> [to <purpose>]" -/
semantic_template maySpendAsThough (who : NounPhrase) (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : StaticSpec :=
  .can (.actor who) (.pay .anyCost .once .paid none none)
    { asThough := some (.mana what as_ purpose) }
/-- "<player> can't <deed> more than N <p>" -/
semantic_template cantMoreThan (player : NounPhrase) (deed : Deed) (bound : Nat) (p : Predicate) :
    StaticSpec :=
  .cant (.actor player) (deonticActionFor deed (counted (.moreThanOf (.lit bound)) p)) {}
semantic_template addMana (amount : Semantics.Amount) (produced : Semantics.ProducedMana) (riders : (List Semantics.ManaRider) := []) : Semantics.Action :=
  (Semantics.Action.addMana amount produced riders)
semantic_template addTurnPart (part : Semantics.TurnPart) (count : Semantics.Amount) (anchor : (Option Semantics.TurnPart) := none) (followedBy : (Option Semantics.TurnPart) := none) : Semantics.Action :=
  (Semantics.Action.addTurnPart part anchor count followedBy)
semantic_template artifactCreatureToken (power : Nat) (toughness : Nat) (colors : (List Semantics.Color)) (subtypes : (List Semantics.Subtype)) (abilities : (List Semantics.Ability) := []) : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := colors, supertypes := [], types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := subtypes, text := abilities, power := (some (Semantics.Amount.lit power)), toughness := (some (Semantics.Amount.lit toughness)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)
semantic_template atNext (part : Semantics.TurnPart) (instruction : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.delay (Semantics.GameEvent.beginningOf part none (Semantics.TurnRef.next none)) [] none instruction)
semantic_template Action.attachment (move : Semantics.AttachMove) (subject : Semantics.NounPhrase) (host : (Option Semantics.NounPhrase) := none) : Semantics.Action :=
  (Semantics.Action.attachment move subject host)
semantic_template Action.bracketed (instruction : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.bracketed instruction)
semantic_template changeLife (delta : (Semantics.Delta Semantics.Amount)) : Semantics.Action :=
  (Semantics.Action.changeLife delta)
semantic_template chooseNewTargets (subject : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.chooseNewTargets subject)
semantic_template chooseOne (first : Semantics.Action) (second : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, first), (none, none, second)])
semantic_template clearDamage (subject : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.clearDamage subject)
semantic_template Action.combat (subject : Semantics.NounPhrase) (update : Semantics.CombatUpdate) : Semantics.Action :=
  (Semantics.Action.combat subject update)
semantic_template conclude (verb : Semantics.OutcomeVerb) : Semantics.Action :=
  (Semantics.Action.conclude verb)
semantic_template copySpell (subject : Semantics.NounPhrase) (times : Semantics.Amount := (Semantics.Amount.lit 1)) : Semantics.Action :=
  (Semantics.Action.copy Semantics.CopySort.fromStack subject times [])
semantic_template copyTargets (copy : Semantics.NounPhrase) (original : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.copyTargets copy original)
semantic_template createDoubleFacedToken (front : Semantics.CharacteristicBundle) (back : Semantics.CharacteristicBundle) (riders : (List Semantics.MoveRider) := []) : Semantics.Action :=
  (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.doubleFaced front back) riders) true none)
semantic_template createObject (count : Semantics.Amount) (spec : Semantics.CreationSpec) : Semantics.Action :=
  (Semantics.Action.createObject count spec true none)
semantic_template createToken (characteristics : Semantics.CharacteristicBundle) (count : Semantics.Amount := (Semantics.Amount.lit 1)) : Semantics.Action :=
  (Semantics.Action.createObject count (Semantics.CreationSpec.token (Semantics.TokenSpec.written characteristics) []) true none)
semantic_template createTokenCopy (source : Semantics.NounPhrase) (riders : (List Semantics.MoveRider) := []) : Semantics.Action :=
  (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf source []) riders) true none)
semantic_template dealDamage (source : Semantics.NounPhrase) (amount : Semantics.Amount) (recipient : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.instruct source (Semantics.Action.dealDamage amount recipient Semantics.DamageKind.any))
semantic_template distribute (verb : Semantics.DividedVerb) (amount : Semantics.Amount) (among : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.distribute verb amount among)
semantic_template doAndForbid (instruction : Semantics.Action) (deed : Semantics.Deed) (subject : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.doAndForbid instruction deed subject)
semantic_template doForEach (group : Semantics.NounPhrase) (body : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.doForEach group body)
semantic_template doForEachKind (axis : Semantics.KindAxis) (sort : Semantics.QualitySort) (body : Semantics.Action) (domain : (Option Semantics.NounPhrase) := none) : Semantics.Action :=
  (Semantics.Action.doForEachKind axis domain sort body)
semantic_template doIfDone (body : Semantics.Action) (ifDid : (Option Semantics.Action) := none) (ifNot : (Option Semantics.Action) := none) : Semantics.Action :=
  (Semantics.Action.withContinuation Semantics.ContinuationPolicy.required body ifDid ifNot)
semantic_template doOnlyIf (instruction : Semantics.Action) (condition : Semantics.Condition) (otherwise : (Option Semantics.Action) := none) : Semantics.Action :=
  (Semantics.Action.doOnlyIf instruction condition otherwise)
semantic_template doublePowerAndToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "double")) (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)))) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject))))) duration))
semantic_template doubleToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "double")) (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "doubleToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.lit 0)) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "doubleToughness.subject" (Semantics.NounPhrase.inCaller subject))))) duration))
semantic_template drawGame : Semantics.Action :=
  Semantics.Action.drawGame
semantic_template enact (verb : Semantics.Deed) (instruction : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.enact verb instruction none none)
semantic_template establish (spec : Semantics.StaticSpec) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.establish spec duration)
semantic_template expose (verb : Semantics.ExposeVerb) (exposed : Semantics.Exposed) : Semantics.Action :=
  (Semantics.Action.expose verb exposed)
semantic_template gainDesignation (subject : Semantics.NounPhrase) (designation : Semantics.DesignationLabel) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.gainDesignation subject designation duration)
semantic_template getNamedEmblem (name : String) (abilities : (List Semantics.Ability)) : Semantics.Action :=
  (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.emblem (some name) abilities) true none)
semantic_template gets (subject : Semantics.NounPhrase) (power : (Semantics.Delta Semantics.Amount)) (toughness : (Semantics.Delta Semantics.Amount)) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.establish (Semantics.StaticSpec.ptModification subject power toughness) duration)
semantic_template getsBoth (subject : Semantics.NounPhrase) (delta : (Semantics.Delta Semantics.Amount)) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.establish (Semantics.StaticSpec.ptModification subject delta delta) duration)
semantic_template holdUntil (instruction : Semantics.Action) (event : Semantics.GameEvent) : Semantics.Action :=
  (Semantics.Action.holdUntil instruction event)
semantic_template ignoreOutcomes (which : Semantics.IgnoredOutcomes) : Semantics.Action :=
  (Semantics.Action.ignoreOutcomes which)
semantic_template limited (instruction : Semantics.Action) (limit : Semantics.UsageLimit) : Semantics.Action :=
  (Semantics.Action.limited instruction limit)
semantic_template may (body : Semantics.Action) (ifDid : (Option Semantics.Action) := none) (ifNot : (Option Semantics.Action) := none) : Semantics.Action :=
  (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional body ifDid ifNot)
semantic_template mayOrElse (body : Semantics.Action) (otherwise : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional body none (some otherwise))
semantic_template merge (subject : Semantics.NounPhrase) (onto : Semantics.NounPhrase) (position : Semantics.MergePosition) : Semantics.Action :=
  (Semantics.Action.merge subject onto position)
semantic_template moveCounters (amount : Semantics.Amount) (source : Semantics.NounPhrase) (destination : Semantics.NounPhrase) (kind : (Option Semantics.CounterKindSource) := none) : Semantics.Action :=
  (Semantics.Action.moveCounters amount kind source destination)
semantic_template pay (cost : Semantics.Cost) (times : Semantics.PayTimes) : Semantics.Action :=
  (Semantics.Action.pay cost times Semantics.PaymentOutcome.paid none none)
semantic_template procedure (kind : Semantics.Procedure) (subject : Semantics.NounPhrase) (payment : Semantics.PlayPayment) (resulting : (Option Semantics.Predicate) := none) : Semantics.Action :=
  (Semantics.Action.procedure kind subject payment resulting none none)
semantic_template putCounters (amount : Semantics.Amount) (kind : Semantics.CounterKindSource) (on : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.putCounters amount kind on Semantics.CounterBatch.one true)
semantic_template relate (subject : Semantics.NounPhrase) (label : Semantics.RelationLabel) (counterpart : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.relate subject label counterpart)
semantic_template repeat_ (repetition : Semantics.Repetition) : Semantics.Action :=
  (Semantics.Action.repeat_ repetition)
semantic_template replace (replaced : Semantics.Action) (replacement : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.replace replaced replacement)
semantic_template rerollStored (quantity : Semantics.Quantity) (whose : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.rerollStored quantity whose)
semantic_template restartGame : Semantics.Action :=
  Semantics.Action.restartGame
semantic_template returnToHand (subject : Semantics.NounPhrase) (from_ : Semantics.Location := Semantics.Location.anyLocation) : Semantics.Action :=
  (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move subject (from_) (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none)
semantic_template revealChoices (sort : Semantics.HiddenSort) : Semantics.Action :=
  (Semantics.Action.revealChoices sort)
semantic_template selectRandom (quantity : Semantics.Quantity) (what : Semantics.Predicate) : Semantics.Action :=
  (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.count quantity (some Semantics.ChoiceMode.atRandom)) what) Semantics.Disclosure.openly none)
semantic_template separateIntoPiles (group : Semantics.NounPhrase) (piles : Nat) (faces : (List Semantics.PileFace) := []) : Semantics.Action :=
  (Semantics.Action.separateIntoPiles group piles faces)
semantic_template sequentially (steps : (List Semantics.Action) := []) : Semantics.Action :=
  (Semantics.Action.sequentially steps)
semantic_template setGameDesignation (designation : Semantics.DesignationLabel) : Semantics.Action :=
  (Semantics.Action.setGameDesignation designation)
semantic_template setStatus (status : Semantics.Status) (subject : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.setStatus status subject)
semantic_template simultaneously (steps : (List Semantics.Action) := []) : Semantics.Action :=
  (Semantics.Action.simultaneously steps)
semantic_template skipPart (part : Semantics.TurnPart) (count : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.skipPart part count)
semantic_template skipUntap (subject : Semantics.NounPhrase) (steps : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.skipUntap subject steps)
semantic_template storeResults (on : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.storeResults on)
semantic_template triggerReflexively (body : Semantics.Action) (triggerBody : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.triggerReflexively body triggerBody)
semantic_template triggerThisWay (body : Semantics.Action) (event : Semantics.GameEvent) (triggerBody : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.triggerThisWay body event triggerBody)
semantic_template triplePower (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "triple")) (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "triplePower.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "triplePower.subject" (Semantics.NounPhrase.inCaller subject))))) (Semantics.Delta.up (Semantics.Amount.lit 0))) duration))
semantic_template tripleToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "triple")) (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "tripleToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.lit 0)) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "tripleToughness.subject" (Semantics.NounPhrase.inCaller subject)))))) duration))
semantic_template turnFaceDown (subject : Semantics.NounPhrase) (spec : Semantics.FaceDown) : Semantics.Action :=
  (Semantics.Action.turnFaceDown subject spec)
semantic_template turnOver (subject : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.turnOver subject)
semantic_template unlock (door : Semantics.Door) : Semantics.Action :=
  (Semantics.Action.unlock door)
semantic_template withContinuation (policy : Semantics.ContinuationPolicy) (body : Semantics.Action) (ifDid : (Option Semantics.Action) := none) (ifNot : (Option Semantics.Action) := none) : Semantics.Action :=
  (Semantics.Action.withContinuation policy body ifDid ifNot)
/-! ## The performer and the handoff -/

/-- "whoever performs this instruction" -/
semantic_template actor : NounPhrase := .actor
/-- "<performer> <body>": the handoff (`Action.instruct`), to a player or a permanent. -/
semantic_template instruct (performer : NounPhrase) (body : Action) : Action :=
  .instruct performer body
/-- Alias for `instruct` for compatibility during porting. -/
semantic_template act (performer : NounPhrase) (body : Action) : Action :=
  .instruct performer body
/-- "… <performer> controls" -/
semantic_template actorControls : Predicate := .hasPossessor .controller .actor
/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a], spelled as the RON helper `substitute`. -/
semantic_template substitute (action : Action) (pays : List PipPaid) : ManaSubstitute :=
  { action, pays }
/-! ## Text Changes -/

/-- A Text Change of `subject`'s rules text, as `change` names it. -/
semantic_template textChange (subject : NounPhrase) (change : TextChange) : StaticSpec :=
  .textChange subject change
/-- "Remove all text found within square brackets" (cleave [CR#702.148a]). -/
semantic_template removeBracketed : TextChange := .removeBracketed
/-- "Replace all instances of the word 'target' with the word 'each'" (overload
[CR#702.96a]). -/
semantic_template targetToEach : TextChange := .targetToEach
/-- "Gains the text of <source>'s rules text" (splice [CR#702.47a,612.10]). -/
semantic_template appendTextOf (source : NounPhrase) : TextChange := .appendTextOf source
/-- "Replace all instances of one <sort> word with another", the kind one of `sorts` [CR#612.2];
an absent word is the performer's choice. -/
semantic_template replaceWord (sorts : List QualitySort) (from_ : Option TextWord := none)
    (to : Option TextWord := none) : TextChange :=
  .replaceWord sorts from_ to
/-- Exchange the subject's rules text with `other`'s [CR#612.5,701.12h]. -/
semantic_template exchangeWith (other : NounPhrase) : TextChange := .exchangeWith other
namespace Actor

/-- "<performer> may cast <what> from <zone>", paying its own cost. -/
semantic_template mayCastFrom (what : NounPhrase) (zone : Location) : StaticSpec :=
  mayPlayDeed (.action "cast") .actor what none (some zone) none .itsOwnCost
/-- An Army the performer controls. -/
semantic_template army : Predicate := .and [.hasSubtype (creatureType "Army"), creature, actorControls]
/-- "any number of <group> with total <axis> <comparator> <bound>" [CR#701.59a]. -/
semantic_template withTotal (axis : ProjAxis) (comparator : Comparator) (bound : Amount)
    (group : NounPhrase) : NounPhrase :=
  .someOf (.totalling axis comparator bound) none group
end Actor

/-! ## Events -/

semantic_template leavesBattlefield (subject : NounPhrase) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (battlefield))
/-- "<subject> leaves <zone>" -/
semantic_template leavesZone (subject : NounPhrase) (zone : Location) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (zone))
/-- "at the beginning of <possessor>'s <part>", each time it comes -/
semantic_template beginningOfPossessed (part : TurnPart) (possessor : NounPhrase) : GameEvent :=
  .beginningOf part (some possessor) (.each none)
/-- "at the beginning of your <part>", each time it comes -/
semantic_template beginningOfYour (part : TurnPart) : GameEvent := .beginningOf part (some .you) (.each none)
/-- "at the beginning of each <part>" -/
semantic_template beginningOfEach (part : TurnPart) : GameEvent := .beginningOf part none (.each none)
/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
semantic_template beginningOfNext (part : TurnPart) : GameEvent := .beginningOf part none (.next none)
/-- "at the beginning of that turn's <part>" -/
semantic_template beginningOfThatTurns (part : TurnPart) : GameEvent := .beginningOf part none .that
semantic_template dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  Primitives.GameEvent.dealsDamage .combatOnly source (some patient)
semantic_template attacks (subject : NounPhrase) : GameEvent := .does (.actor subject) (.attack none)
/-- "<subject> attacks <whom>" -/
semantic_template attacksPlayer (subject whom : NounPhrase) : GameEvent :=
  .does (.actor subject) (.attack (some whom))
/-- "<n> is put into <zone> from <source>" -/
semantic_template putIntoFrom (subject : NounPhrase) (destination : Location) (source : Location) : GameEvent :=
  Primitives.GameEvent.putInto subject destination (some source)
/-- "N <kind> counters are put on / removed from <subject>" -/
semantic_template counterEvent (move : CounterMove) (kind : CounterKind) (batch : CounterBatch)
    (subject : NounPhrase) : GameEvent :=
  match move with
  | .put => .does .any (.putCounters .anyAmount (.printed kind) subject batch false)
  | .removed => .does .any (.removeCounters none (some (.printed kind)) subject batch false)
/-- "counters are put on / removed from <subject>", the kind unsaid. -/
semantic_template bareCounterEvent (move : CounterMove) (batch : CounterBatch) (subject : NounPhrase) :
    GameEvent :=
  match move with
  | .put => .does .any (.putCounters .anyAmount .anyKind subject batch false)
  | .removed => .does .any (.removeCounters none none subject batch false)
/-- "the last <kind> counter is removed from <subject> by <who>" -/
semantic_template lastCounterRemovedBy (kind : CounterKind) (subject who : NounPhrase) : GameEvent :=
  .does (.by who) (.removeCounters none (some (.printed kind)) subject .emptying false)
/-- "one or more counters are put on <subject> by an effect" -/
semantic_template manyCountersPutByEffect (subject : NounPhrase) : GameEvent :=
  .does .any (.putCounters .anyAmount .anyKind subject .many true)
/-- "<who> puts one or more counters on <subject>" -/
semantic_template manyBareCountersPutBy (who subject : NounPhrase) : GameEvent :=
  .does (.actor who) (.putCounters .anyAmount .anyKind subject .many false)
/-- A token-creation event, retaining its creator, controller, and effect restriction. -/
semantic_template tokenCreationEvent (tokens : NounPhrase) (byEffect : Bool)
    (by_ under : Option NounPhrase) : GameEvent :=
  .does (match by_ with | some creator => .by creator | none => .any)
    (.createObject .anyAmount (.token (.copyOf tokens []) []) byEffect under)
/-- "one or more tokens would be created" -/
semantic_template tokensCreated (tokens : NounPhrase) : GameEvent :=
  tokenCreationEvent tokens false none none
/-- "you roll a die and the result is <q>" -/
semantic_template youRollResultIn (quantity : Quantity) : GameEvent :=
  .does (.actor .you) (.rollDice (.lit 1) (.sides 6) .one (.resultIn quantity))
/-- "you roll a die and the natural result is the highest" -/
semantic_template youRollHighestNatural : GameEvent :=
  .does (.actor .you) (.rollDice (.lit 1) (.sides 6) .one .highestNatural)
/-- "one or more tokens would be created under <under>'s control by an effect" -/
semantic_template tokensCreatedByEffectUnder (tokens under : NounPhrase) : GameEvent :=
  tokenCreationEvent tokens true none (some under)
semantic_template blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .does (.actor subject) (.block (blocked.getD .any))
semantic_template becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .does (match by_ with | some b => .by b | none => .any) (.block subject)
/-- "the last <kind> counter is removed from <subject>" -/
semantic_template lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .does .any (.removeCounters none (some (.printed kind)) subject .emptying false)
/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
semantic_template copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability
semantic_template activates (player : Semantics.NounPhrase) (ability : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.procedure Semantics.Procedure.activate ability Semantics.PlayPayment.itsOwnCost none none none))
semantic_template GameEvent.attachment (move : Semantics.AttachMove) (subject : Semantics.NounPhrase) (host : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.attachment move subject (some host)))
semantic_template attacksWith (player : Semantics.NounPhrase) (attackers : Semantics.NounPhrase) (defender : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.attackWith attackers defender))
semantic_template becomesTarget (subject : Semantics.NounPhrase) (by_ : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.by by_) (Semantics.Action.target subject))
semantic_template beginningOf (part : Semantics.TurnPart) (whose : Semantics.NounPhrase) (which : Semantics.TurnRef) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part (some whose) which)
semantic_template casts (player : Semantics.NounPhrase) (spell : (Option Semantics.NounPhrase) := none) (from_ : (Option Semantics.Location) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.procedure Semantics.Procedure.cast (Option.getD spell Semantics.NounPhrase.any) Semantics.PlayPayment.itsOwnCost none from_ none))
semantic_template causes (cause : Semantics.Causing) (event : Semantics.GameEvent) : Semantics.GameEvent :=
  (Semantics.GameEvent.causes cause event)
semantic_template chapterMark (chapters : (List Nat) := []) : Semantics.GameEvent :=
  (Semantics.GameEvent.chapterMark chapters)
set_option linter.unusedVariables false in
semantic_template GameEvent.combat (relation : Semantics.CombatRelation) (subject : Semantics.NounPhrase) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor subject) (Semantics.Action.attack counterpart))
semantic_template commitsCrime (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) Semantics.Action.commitCrime)
semantic_template damage (kind : Semantics.DamageKind) (patient : (Option Semantics.NounPhrase) := none) (source : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match source with | some s => Semantics.Subject.actor s | none => Semantics.Subject.any) (Semantics.Action.dealDamage Semantics.Amount.anyAmount (Option.getD patient Semantics.NounPhrase.any) kind))
semantic_template dies (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move subject ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []))
semantic_template discards (player : Semantics.NounPhrase) (card : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "discard") card) none none))
semantic_template draws (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.draw Semantics.Amount.anyAmount))
semantic_template endOf (part : Semantics.TurnPart) (whose : Semantics.NounPhrase) (which : Semantics.TurnRef) : Semantics.GameEvent :=
  (Semantics.GameEvent.endOf part (some whose) which)
semantic_template enters (subject : Semantics.NounPhrase) (from_ : (Option Semantics.Location) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move subject (Option.getD from_ Semantics.Location.anyLocation) (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none []))
semantic_template gameBecomes (designation : Semantics.DesignationLabel) : Semantics.GameEvent :=
  (Semantics.GameEvent.gameBecomes designation)
semantic_template lifeChanges (player : Semantics.NounPhrase) (move : Semantics.LifeMove) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.changeLife (match move with | Semantics.LifeMove.up => Semantics.Delta.up Semantics.Amount.anyAmount | Semantics.LifeMove.down => Semantics.Delta.down Semantics.Amount.anyAmount)))
semantic_template losesGame (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.conclude Semantics.OutcomeVerb.loseGame))
semantic_template nthOccurrence (ordinal : Semantics.Ordinal) (event : Semantics.GameEvent) (per : (Option Semantics.TurnPart) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.nthOccurrence ordinal per event)
semantic_template paysCost (outcome : Semantics.PaymentOutcome) (whose : Semantics.NounPhrase) (keyword : Semantics.KeywordLabel) (player : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match player with | some p => Semantics.Subject.actor p | none => Semantics.Subject.any) (Semantics.Action.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once outcome (some whose) (some keyword)))
semantic_template paysLife (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.changeLife (Semantics.Delta.down Semantics.Amount.anyAmount)))
semantic_template statBecomes (subject : Semantics.NounPhrase) (stat : Semantics.Stat) (value : Semantics.Amount) : Semantics.GameEvent :=
  (Semantics.GameEvent.statBecomes subject stat value)
semantic_template stateHolds (condition : Semantics.Condition) : Semantics.GameEvent :=
  (Semantics.GameEvent.stateHolds condition)
semantic_template statusEvent (subject : Semantics.NounPhrase) (status : Semantics.Status) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.setStatus status subject))
semantic_template tappedForMana (source : Semantics.NounPhrase) (player : (Option Semantics.NounPhrase) := none) (type : (Option Semantics.ManaTypeTerm) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match player with | some p => Semantics.Subject.by p | none => Semantics.Subject.any) (Semantics.Action.tapForMana source type))
semantic_template triggers (ability : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.triggers ability none none)
semantic_template unlocksDoor (player : Semantics.NounPhrase) (door : Semantics.Door) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Action.unlock door))
semantic_template verbedEvent (verb : Semantics.Deed) (patient : (Option Semantics.NounPhrase) := none) (agent : (Option Semantics.NounPhrase) := none) (becomes : (Option Semantics.Predicate) := none) (locus : (Option Semantics.Location) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match agent with | some a => Semantics.Subject.actor a | none => Semantics.Subject.any) (Semantics.Action.enact verb (Semantics.Templates.deonticActionFor verb (Option.getD patient Semantics.NounPhrase.any)) becomes locus))
semantic_template zoneChange (subject : Semantics.NounPhrase) (observation : Semantics.ObservationPoint) (from_ : (Option Semantics.Location) := none) (to : (Option Semantics.Location) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move subject (Option.getD from_ Semantics.Location.anyLocation) to observation none []))
/-! ## Abilities -/

semantic_template keyword (label : KeywordLabel) : Ability := .keyword label [] []
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
semantic_template abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
semantic_template flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Pay N life" as a cost. -/
semantic_template payLife (amount : Nat) : Cost :=
  .perform (.changeLife (.down (.lit amount)))
/-- "<keyword> <cost>" -/
semantic_template keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] []
/-- "<keyword> <quality>", e.g. "protection from red" -/
semantic_template keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] []
/-- "<keyword> <subject>", e.g. "enchant creature" -/
semantic_template keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] []
/-- "<keyword> N", e.g. "bushido 2" -/
semantic_template keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
semantic_template keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) : Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
semantic_template keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) : Ability :=
  .keyword label [.number amount, .cost cost] []
/-- "When <event>, if <condition>, <instruction>" -/
semantic_template triggeredIf (event : GameEvent) (condition : Condition)
    (instruction : Action) : Ability :=
  .triggered event [] none [] none none (some condition) instruction
/-- "Whenever <event>, <instruction>. This ability triggers only once each turn." -/
semantic_template triggeredOnlyOnce (event : GameEvent) (limit : UsageLimit) (instruction : Action) :
    Ability :=
  .triggered event [] none [] none (some limit) none instruction
/-- "As <subject> enters, choose a <quality>." -/
semantic_template entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) none .openly
/-- "As <subject> enters, choose a <quality> from <domain>." -/
semantic_template entersChoosingFrom (subject : NounPhrase) (sort : QualitySort) (domain : ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) (some domain) .openly
/-- "As <subject> enters, choose a player [from <domain>]." -/
semantic_template entersChoosingPlayer (subject : NounPhrase) (domain : Option ChoiceDomain) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .openly
/-- "As <subject> enters, secretly choose a player [from <domain>]." -/
semantic_template entersChoosingPlayerSecretly (subject : NounPhrase) (domain : Option ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .secretly
/-- "<subject> enters tapped" -/
semantic_template entersTapped (subject : NounPhrase) : StaticSpec := .entryRider subject (.entersAs .tapped)
/-- "As <subject> becomes attached, choose a <quality>." -/
semantic_template attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.attachmentChoice subject (.quality sort) none
/-- "<subject> enters with N <kind> counters on it" -/
semantic_template entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fresh)
/-- "<subject> enters with an additional N <kind> counters on it" -/
semantic_template entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .additional)
/-- "<subject> enters with N fewer <kind> counters on it" -/
semantic_template entersWithFewerCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fewer)
semantic_template triggered (event : GameEvent) (instruction : Action) : Ability :=
  .triggered event [] none [] none none none instruction
/-- "Whenever <event> and whenever <joined events>, <instruction>" -/
semantic_template triggeredJoined (event : GameEvent) (joins : List JoinedHeader) (instruction : Action) :
    Ability :=
  .triggered event [] none joins none none none instruction
/-- A further "whenever <event>" header joined onto a trigger. -/
semantic_template joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
/-- "Whenever <event> or <alternatives>, <instruction>" -/
semantic_template triggeredOr (event : GameEvent) (alternatives : List GameEvent) (instruction : Action) :
    Ability :=
  .triggered event alternatives none [] none none none instruction
/-- "Whenever <event> while <concurrent>, <instruction>" -/
semantic_template triggeredWhile (event : GameEvent) (while_ : Timing) (instruction : Action) :
    Ability :=
  .triggered event [] (some while_) [] none none none instruction
/-- "Whenever <event> during <timing>, <instruction>" -/
semantic_template triggeredOnlyDuring (event : GameEvent) (timing : Timing) (instruction : Action) :
    Ability :=
  .triggered event [] none [] (some timing) none none instruction
/-- A joined "whenever <event> while <concurrent>" header. -/
semantic_template joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader :=
  ⟨event, [], some while_, none⟩
/-- "When <event>, <instruction>" -/
semantic_template when (event : GameEvent) (instruction : Action) : Ability := triggered event instruction
/-- "Whenever <event>, <instruction>" -/
semantic_template whenever (event : GameEvent) (instruction : Action) : Ability :=
  triggered event instruction
/-- "At <event>, <instruction>" (`at` is a Lean keyword). -/
semantic_template at_ (event : GameEvent) (instruction : Action) : Ability := triggered event instruction
/-- "After <event>, <instruction>": the dice template's word. -/
semantic_template after (event : GameEvent) (instruction : Action) : Ability := triggered event instruction
semantic_template activated (cost : Cost) (instruction : Action) : Ability :=
  .activated cost instruction none none none none
/-- "[cost]: <instruction>. Activate only <timing>." -/
semantic_template activatedOnlyDuring (cost : Cost) (instruction : Action) (timing : Timing) : Ability :=
  .activated cost instruction (some timing) none none none
/-- "[cost]: <instruction>. Only <who> may activate this ability." -/
semantic_template activatedBy (cost : Cost) (instruction : Action) (who : NounPhrase) : Ability :=
  .activated cost instruction none none none (some who)
/-- "[cost]: <instruction>. Activate only once <limit>." -/
semantic_template activatedOnlyOnce (cost : Cost) (instruction : Action) (limit : UsageLimit) : Ability :=
  .activated cost instruction none (some limit) none none
/-- "[cost]: <instruction>. Activate only if <guard>." -/
semantic_template activatedOnlyIf (cost : Cost) (instruction : Action) (guard : Condition) : Ability :=
  .activated cost instruction none none (some guard) none
/-- `ability` with the restrictions a keyword or a granting effect adds to it [CR#113.10a]. -/
semantic_template qualified (ability : Ability) (timing : Option Timing := none)
    (limit : Option UsageLimit := none) (guard : Option Condition := none)
    (from_ : Option Zone := none) : Ability :=
  .qualified ability timing limit guard from_
/-- "<subject> has <selection> of <source>" (backup [CR#702.165a]). -/
semantic_template abilityGrantSelected (subject : NounPhrase) (source : NounPhrase)
    (selection : AbilitySelection) : StaticSpec :=
  .abilityGrantSelected subject source selection
/-- "[cost]: <instruction>. Activate only once <limit> and only if <guard>." -/
semantic_template activatedOnlyOnceIf (cost : Cost) (instruction : Action) (limit : UsageLimit)
    (guard : Condition) : Ability :=
  .activated cost instruction none (some limit) (some guard) none
semantic_template abilityGrant (subject : Semantics.NounPhrase) (ability : Semantics.Ability) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.abilityGrant subject ability)
semantic_template abilityGrantFrom (subject : Semantics.NounPhrase) (source : Semantics.NounPhrase) (classes : (List Semantics.AbilityClass) := []) (except : (Option Semantics.Predicate) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.abilityGrantFrom subject classes source except)
semantic_template Ability.activatedBy (cost : Semantics.Cost) (instruction : Semantics.Action) (who : Semantics.NounPhrase) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction none none none (some who))
semantic_template addSubtype (subject : Semantics.NounPhrase) (subtype : Semantics.Subtype) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.characteristicChange subject [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [subtype]), retained := none } : Semantics.TypeLineChanges))])
semantic_template addedCost (cost : Semantics.Cost) (offered : Bool) (times : Semantics.PayTimes := Semantics.PayTimes.once) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.addedCost cost offered times)
semantic_template additionalTriggers (event : Semantics.GameEvent) (quantity : Semantics.Quantity) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.additionalTriggers event quantity)
semantic_template addsTarget (subject : Semantics.NounPhrase) (description : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.addsTarget subject description)
semantic_template allInstead (cost : (Option Semantics.Cost) := none) (condition : (Option Semantics.Condition) := none) : Semantics.ModeChange :=
  (Semantics.ModeChange.allInstead cost condition)
semantic_template alsoForKeywords (ability : Semantics.Ability) (keywords : (List Semantics.KeywordTerm) := []) : Semantics.Ability :=
  (Semantics.Ability.alsoForKeywords ability keywords)
semantic_template altCost (subject : Semantics.NounPhrase) (cost : (Option Semantics.Cost) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.altCost subject cost)
semantic_template asEvent (event : Semantics.GameEvent) (does : Semantics.Action) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.asEvent event does)
semantic_template asThose : Semantics.TokenSpec :=
  Semantics.TokenSpec.asThose
semantic_template cantBeAttachedTo (subject : Semantics.NounPhrase) (host : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Action.attachment Semantics.AttachMove.attached subject (some host)) ({} : Semantics.DeonticMods))
semantic_template cantBeBlockedBy (subject : Semantics.NounPhrase) (blockers : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all blockers)) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) subject) ({} : Semantics.DeonticMods))
semantic_template cantBeBlockedExceptBy (subject : Semantics.NounPhrase) (blockers : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) subject) ({ except := (some (Semantics.DeonticExcept.byObjects blockers)) } : Semantics.DeonticMods))
semantic_template cantExcept (deed : Semantics.Deed) (subject : Semantics.NounPhrase) (cause : Semantics.DeonticExcept) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor deed subject) ({ except := (some cause) } : Semantics.DeonticMods))
semantic_template characteristicChange (subject : Semantics.NounPhrase) (edits : (List Semantics.CharacteristicEdit) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.characteristicChange subject edits)
semantic_template choice (occasion : Semantics.ChoiceOccasion) (subject : Semantics.NounPhrase) (sort : Semantics.ChoiceSort) (disclosure : Semantics.Disclosure) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.choice occasion subject sort domain disclosure)
semantic_template compound (costs : (List Semantics.Cost) := []) : Semantics.Cost :=
  (Semantics.Cost.compound costs)
semantic_template conditional (spec : Semantics.StaticSpec) (condition : Semantics.Condition) (marking : Semantics.CondMarking) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conditional spec condition marking)
semantic_template conjunction (subject : (Option Semantics.NounPhrase) := none) (parts : (List Semantics.StaticSpec) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conjunction subject parts)
semantic_template controlGrant (player : Semantics.NounPhrase) (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.controlGrant player subject)
semantic_template copyChange (subject : Semantics.NounPhrase) (source : Semantics.NounPhrase) (exceptions : (List Semantics.CopyExcept) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.copyChange subject source exceptions)
semantic_template copyOf (source : Semantics.NounPhrase) (exceptions : (List Semantics.CopyExcept) := []) : Semantics.TokenSpec :=
  (Semantics.TokenSpec.copyOf source exceptions)
semantic_template costShift (subject : Semantics.NounPhrase) (shift : Semantics.CostShift) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.costShift subject shift)
semantic_template damageRule (kind : Semantics.DamageKind) (source : Semantics.DamageAgent) (scope : Semantics.DamageScope) (op : Semantics.DamageOp) (use : Semantics.ReplUse) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.damageRule kind source scope op use)
semantic_template Timing.duringPart (part : Semantics.TurnPart) (whose : (Option Semantics.NounPhrase) := none) : Semantics.Timing :=
  (Semantics.Timing.inPart part whose Semantics.TurnRef.current)
semantic_template entryRider (subject : Semantics.NounPhrase) (rider : Semantics.MoveRider) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject rider)
semantic_template extraModesCost (cost : Semantics.Cost) : Semantics.ModeChange :=
  (Semantics.ModeChange.extraModesCost cost)
semantic_template insteadOf (event : Semantics.WouldClause) (replacement : Semantics.Action) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.instead event replacement none Semantics.ReplUse.repeatedly none)
semantic_template italicHead (word : Semantics.ItalicWord) (ability : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead word ability)
semantic_template itsManaCost : Semantics.Cost :=
  Semantics.Cost.itsManaCost
semantic_template letterDefinition (letter : Semantics.Letter) (amount : Semantics.Amount) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.letterDefinition letter amount)
semantic_template loyaltySymbol (loyalty : Semantics.LoyaltyCost) : Semantics.Cost :=
  (Semantics.Cost.loyaltySymbol loyalty)
semantic_template mana (cost : (List Semantics.ManaSymbol)) : Semantics.Cost :=
  (Semantics.Cost.mana cost)
semantic_template manaRetention (player : Semantics.NounPhrase) (mana : Semantics.ManaHeld) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.manaRetention player mana)
semantic_template mayBeginOnBattlefield : Semantics.Ability :=
  Semantics.Ability.mayBeginOnBattlefield
semantic_template mayCastFor (player : Semantics.NounPhrase) (cost : Semantics.Cost) (from_ : (Option Semantics.Location) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.can (Semantics.Subject.actor player) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead cost) none from_ none) ({} : Semantics.DeonticMods))
semantic_template modeChoice (change : Semantics.ModeChange) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.modeChoice change)
semantic_template modification (subject : Semantics.NounPhrase) (stat : Semantics.Stat) (delta : (Semantics.Delta Semantics.Amount)) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.modification subject stat delta)
semantic_template notChosenBefore (within : Semantics.Duration) : Semantics.ModeChange :=
  (Semantics.ModeChange.notChosenBefore within)
semantic_template Cost.or (costs : (List Semantics.Cost) := []) : Semantics.Cost :=
  (Semantics.Cost.or costs)
semantic_template partSkip (player : Semantics.NounPhrase) (part : Semantics.TurnPart) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.partSkip player part)
semantic_template paySubstitute (subject : Semantics.NounPhrase) (substitute : Semantics.ManaSubstitute) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.paySubstitute subject substitute)
semantic_template perform (instruction : Semantics.Action) : Semantics.Cost :=
  (Semantics.Cost.perform instruction)
semantic_template preventionBan (kind : Semantics.DamageKind) (damage : Semantics.Unpreventable) (ban : Semantics.PreventionBan) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.preventionBan kind damage ban)
semantic_template ptDefinition (subject : Semantics.NounPhrase) (slots : Semantics.DefinedSlots) (amount : Semantics.Amount) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptDefinition subject slots amount)
semantic_template ptModification (subject : Semantics.NounPhrase) (power : (Semantics.Delta Semantics.Amount)) (toughness : (Semantics.Delta Semantics.Amount)) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptModification subject power toughness)
semantic_template ptSwitch (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptSwitch subject)
semantic_template repeatable : Semantics.ModeChange :=
  Semantics.ModeChange.repeatable
semantic_template replacement (event : Semantics.WouldClause) (replacement : Semantics.Action) (use : Semantics.ReplUse) (timing : (Option Semantics.Timing) := none) (limit : (Option Semantics.UsageLimit) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.instead event replacement timing use limit)
semantic_template retention (spec : Semantics.StaticSpec) (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.retention spec subject)
semantic_template scaled (cost : Semantics.Cost) (amount : Semantics.Amount) : Semantics.Cost :=
  (Semantics.Cost.scaled cost amount)
semantic_template setQuantity (quantity : Semantics.Quantity) : Semantics.ModeChange :=
  (Semantics.ModeChange.setQuantity quantity)
semantic_template specialAction (when : Semantics.Timing) (cost : Semantics.Cost) (does : Semantics.Action) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.specialAction (some when) cost does)
semantic_template Ability.spell (instruction : Semantics.Action) (timing : (Option Semantics.Timing) := none) : Semantics.Ability :=
  (Semantics.Ability.spell timing instruction)
semantic_template static (spec : Semantics.StaticSpec) : Semantics.Ability :=
  (Semantics.Ability.static spec)
semantic_template tapSymbol : Semantics.Cost :=
  Semantics.Cost.tapSymbol
semantic_template thatAbility (ref : Semantics.ChoiceRef) : Semantics.Ability :=
  (Semantics.Ability.thatAbility ref)
semantic_template triggeredOrOnlyOnce (event : Semantics.GameEvent) (alternatives : (List Semantics.GameEvent)) (limit : Semantics.UsageLimit) (instruction : Semantics.Action) : Semantics.Ability :=
  (Semantics.Ability.triggered event alternatives none [] none (some limit) none instruction)
semantic_template untapSymbol : Semantics.Cost :=
  Semantics.Cost.untapSymbol
semantic_template visibility (verb : Semantics.ExposeVerb) (player : Semantics.NounPhrase) (what : Semantics.VisibleThing) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.visibility verb player what)
semantic_template whileHolds (condition : Semantics.Condition) (does : Semantics.Action) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.whileHolds condition does)
semantic_template withSubstitute (cost : Semantics.Cost) (substitute : Semantics.ManaSubstitute) : Semantics.Cost :=
  (Semantics.Cost.withSubstitute cost substitute)
semantic_template written (characteristics : Semantics.CharacteristicBundle) : Semantics.TokenSpec :=
  (Semantics.TokenSpec.written characteristics)
/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
semantic_template stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
semantic_template levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
semantic_template prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }
/-! ## Conditions -/

semantic_template Condition.and (conjuncts : (List Semantics.Condition) := []) : Semantics.Condition :=
  (Semantics.Condition.and conjuncts)
semantic_template anyResultIs (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Condition :=
  (Semantics.Condition.anyResultIs comparator bound)
semantic_template choseThisWay (chooser : Semantics.NounPhrase) (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.choseThisWay chooser predicate)
semantic_template compareAmt (subject : Semantics.Amount) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Condition :=
  (Semantics.Condition.compareAmt subject comparator bound)
semantic_template dealtThisWay (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.dealtThisWay predicate)
semantic_template Condition.duringPart (part : Semantics.TurnPart) (whose : (Option Semantics.NounPhrase) := none) : Semantics.Condition :=
  (Semantics.Condition.duringPart part whose)
semantic_template flipCalled (caller : Semantics.NounPhrase) (call : Semantics.FlipCall) : Semantics.Condition :=
  (Semantics.Condition.flipCalled caller call)
semantic_template flipFace (face : Semantics.CoinFace) : Semantics.Condition :=
  (Semantics.Condition.flipFace face)
semantic_template gameIs (designation : Semantics.DesignationLabel) : Semantics.Condition :=
  (Semantics.Condition.gameIs designation)
semantic_template keywordCostPaid (label : Semantics.KeywordLabel) (subject : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword label) none) subject) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))
semantic_template matches_ (subject : Semantics.NounPhrase) (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.matches subject predicate)
semantic_template noHolder (designation : Semantics.DesignationLabel) : Semantics.Condition :=
  (Semantics.Condition.noHolder designation)
semantic_template Condition.not (condition : Semantics.Condition) : Semantics.Condition :=
  (Semantics.Condition.not condition)
semantic_template Condition.or (disjuncts : (List Semantics.Condition) := []) : Semantics.Condition :=
  (Semantics.Condition.or disjuncts)
semantic_template preventedFromSource (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.preventedFromSource predicate)
semantic_template rolledDoubles : Semantics.Condition :=
  Semantics.Condition.rolledDoubles
semantic_template voteLead (label : String) (orTied : Bool) : Semantics.Condition :=
  (Semantics.Condition.voteLead label orTied)
/-! ## Conferrals -/

semantic_template stateBased (when : Semantics.Condition) (then_ : Semantics.Action) : Semantics.Conferral :=
  (Semantics.Conferral.stateBased when then_)
/-! ## Pips -/

semantic_template PipPaid.generic : Semantics.PipPaid :=
  Semantics.PipPaid.generic
semantic_template ofColor (color : Semantics.ColorTerm) : Semantics.PipPaid :=
  (Semantics.PipPaid.ofColor color)

end Semantics.Templates
