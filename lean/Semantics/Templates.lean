import Semantics.Primitives
import Semantics.CallerScope

/-!
# Semantics.Templates

Phrasing helpers (`Template`s, `semantic_template`) for `semantics-v3`
(`docs/decisions/semantics-v3.md` §§1–3). A Template expands transparently in the caller's DRS
scope and uses `firstMention` / `laterMention` whenever a `NounPhrase` parameter is mentioned more
than once before a zone change. Templates are spelled over primitives and other Templates and
name no catalog item and no card type: the layering is Primitives → Templates → Definitions →
Cards. A phrasing helper that names a card type or a catalog entry ("this creature", "your party")
lives above it, in `Semantics.Definitions.Phrasings`, and one that needs a composite Definition in
`Semantics.Definitions.Actions` or `Semantics.Definitions.Keywords`.
-/

namespace Semantics.Templates

/-! ## Worded primitives

Shared phrasings over a single primitive, spelled here so a card never names a constructor. -/

/-- "X is <amount>": a numeric definition at action position, by the shared static
definition. -/
semantic_template define (letter : Letter) (amount : Amount) : Action :=
  .establish (.letterDefinition letter amount) none
/-- "<body> <times> times" -/
semantic_template repeatTimes (times : Amount) (body : Action) : Action := .repeat_ (.fixed times body)
/-- "both <left> and <right>" -/
semantic_template both (left right : NounPhrase) : NounPhrase := .and [left, right]
/-- "<left> or <right>" -/
semantic_template eitherOf (left right : NounPhrase) : NounPhrase := .or [left, right]
/-- "<left> or <right>" as a cost -/
semantic_template either (left right : Cost) : Cost := .or [left, right]
/-- "enchanted by <by_>", "equipped by <by_>": the host of an attachment `by_` names. -/
semantic_template attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Predicate :=
  .attachment .host word by_
/-- "attached to <host>" -/
semantic_template attachedTo (host : NounPhrase) : Predicate := .attachment .attachment none host
/-- "a legal object or player for <attachment> to enchant" [CR#303.4f,303.4g] -/
semantic_template legalHostOf (attachment : NounPhrase) : Predicate :=
  .legalAttachment .host attachment
/-- "that could enchant <host>": an Aura that could legally be attached to `host` [CR#701.3a]. -/
semantic_template couldEnchant (host : NounPhrase) : Predicate := .legalAttachment .attachment host
/-- A quality change: `op` applied to `payload` on `subject`'s characteristics. -/
semantic_template qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    StaticSpec := .characteristicChange subject (payload.edits op)
/-- "<subject> loses <abilities>" -/
semantic_template abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : StaticSpec :=
  .characteristicChange subject [.removedAbilities (.specified abilities)]
/-- "<subject> loses all abilities [except <except>]" -/
semantic_template allAbilityLoss (subject : NounPhrase) (except : Option Predicate := none) :
    StaticSpec :=
  .characteristicChange subject [.removedAbilities (.allExcept except)]
/-- "During [<whose>] <part>, <spec>" -/
semantic_template partScope (part : TurnPart) (whose : Option NounPhrase) (spec : StaticSpec) :
    StaticSpec := .conditional spec (.duringPart part whose) .asLongAs
/-- "remove <subject> from combat" -/
semantic_template removeFromCombat (subject : NounPhrase) : Action :=
  .combat subject (.participation .outsideCombat)
/-- "<subject> becomes attacking [<defender>]" -/
semantic_template becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Action :=
  .combat subject (.participation (.attacking defender))
/-- "<subject> becomes blocking <blocked>" -/
semantic_template becomeBlocking (subject blocked : NounPhrase) : Action :=
  .combat subject (.blocking .attached blocked)
/-- "<subject> becomes blocked": the Blocked status, which an effect can give an attacking creature
too, not a block by a particular creature [CR#509.1h,509.3c]. "Becomes blocked by <blocker>" stays
`block` with a `by` subject [CR#509.3d]. -/
semantic_template becomeBlocked (subject : NounPhrase) : Action :=
  .combat subject (.participation (.blocked true))
/-- "<subject> stops blocking <blocked>" -/
semantic_template stopBlocking (subject blocked : NounPhrase) : Action :=
  .combat subject (.blocking .unattached blocked)
/-- "unattach <subject>" -/
semantic_template unattach (subject : NounPhrase) : Action := .attachment .unattached subject none
/-- "you get an emblem with <abilities>" -/
semantic_template getEmblem (abilities : List Ability) : Action :=
  .createObject 1 (.emblem none abilities) none

/-! ## Pronouns -/

/-- "you" -/
semantic_template you : NounPhrase := .you
/-- "this": the source, by self-name [CR#113.7]. -/
semantic_template this_ : NounPhrase := .this

/-- "it" -/
semantic_template it : NounPhrase := .pro .bare .one
/-- "them" -/
semantic_template them : NounPhrase := .pro .bare .many
/-- "that card": the card slot's occupant. -/
semantic_template itCard : NounPhrase := .pro (.atSlot .card) .one
/-- "it": the permanent slot's occupant, as "sacrifice it" reads it. -/
semantic_template itPermanent : NounPhrase := .pro (.atSlot .permanent) .one
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
/-- "that token" -/
semantic_template thatToken : NounPhrase := that .token
/-- "that source": the source a choice or an event named, in any zone [CR#609.7a]. -/
semantic_template thatSource : NounPhrase := that .source
/-- "those tokens" -/
semantic_template thoseTokens : NounPhrase := those .token
/-- "they": the player most recently named. -/
semantic_template they : NounPhrase := .pro (.word .player) .one
/-- "them" reading players, as `they` reads one: "you choose one of them". -/
semantic_template themPlayers : NounPhrase := .pro (.word .player) .many
/-- The same over "any target": the player, or the permanent's controller. -/
semantic_template splitOverPermanent : NounPhrase :=
  eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf .permanent) .one))
/-! ## Wildcards

`any` is the unconstrained value of its sort: the unspecified agent of a passive ("can't be
blocked"), any object or player, any amount, cost, ballot, counter kind, roll, location or
timing. Each sort's `AnyValue` instance is its template, so the call survives inspection. -/

semantic_template Subject.any : Subject := .any
semantic_template NounPhrase.any : NounPhrase := .any
/-- "any kind of counter" -/
semantic_template anyKind : CounterKindSource := .anyKind
/-- "any result" of a roll -/
semantic_template anyResult : RollWatch := .anyResult
/-- "at any time" -/
semantic_template anyTiming : Timing := .anyTiming
/-- "<np> <verb>s": the clause's subject, in the active voice. -/
semantic_template actor (np : NounPhrase) : Subject := .actor np
/-- "… by <np>": the clause's subject, in the passive voice. -/
semantic_template by_ (np : NounPhrase) : Subject := .by np
/-- "any amount" -/
semantic_template anyAmount : Amount := .anyAmount
/-- "the <ordinal>", of a counter put: the put that crosses the ordinal's threshold [CR#122.7]. -/
semantic_template theNth (ordinal : Ordinal) : Amount := .theNth ordinal
/-- "any cost" -/
semantic_template anyCost : Cost := .anyCost
/-- Any of the listed choices, as a vote leaves them open [CR#701.38a]. -/
semantic_template anyBallot : Ballot := .anyBallot
/-- "anywhere" -/
semantic_template wherever : Location := .anyLocation
/-- A bare noun phrase in a subject's place is the active voice: `cant you …`. -/
instance [AuthoringContext] : Coe NounPhrase Subject := ⟨actor⟩
instance [AuthoringContext] : AnyValue Subject := ⟨Templates.Subject.any⟩
instance [AuthoringContext] : AnyValue NounPhrase := ⟨Templates.NounPhrase.any⟩
instance [AuthoringContext] : AnyValue Ballot := ⟨anyBallot⟩
instance [AuthoringContext] : AnyValue Cost := ⟨anyCost⟩
instance [AuthoringContext] : AnyValue Amount := ⟨anyAmount⟩
instance [AuthoringContext] : AnyValue CounterKindSource := ⟨anyKind⟩
instance [AuthoringContext] : AnyValue RollWatch := ⟨anyResult⟩
instance [AuthoringContext] : AnyValue Location := ⟨wherever⟩
instance [AuthoringContext] : AnyValue Timing := ⟨anyTiming⟩
/-! ## Quantities -/

semantic_template exactly (n : Nat) : Quantity := .range n n
semantic_template upTo (n : Nat) : Quantity := .range none n
semantic_template anyNumber : Quantity := .range none none
semantic_template orMore (n : Nat) : Quantity := .range n none
semantic_template oneThrough (n : Nat) : Quantity := .range (some 1) n
semantic_template exactlyOf (amount : Amount) : Quantity := .exactlyOf amount
semantic_template range (low : Option Nat := none) (high : Option Nat := none) : Quantity :=
  .range low high
semantic_template upToOf (amount : Amount) : Quantity := .upToOf amount
semantic_template worth (n : Nat) : Quantity := .worth n
/-- "more than N": the count a prohibition bounds ("can't cast more than one spell"). -/
semantic_template moreThan (n : Nat) : Quantity := .moreThanOf (.lit n)
/-- "an additional N": beyond what is otherwise allowed ("play an additional land"). -/
semantic_template additional (q : Quantity) : Quantity := .additional q
/-! ## Determiners -/

/-- "target …" -/
semantic_template target (p : Predicate) : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) p
/-- "each …" -/
semantic_template each (p : Predicate) : NounPhrase := .described .each p
/-- "all …" -/
semantic_template allOf (p : Predicate) : NounPhrase := .described .all p
/-- the bare plural: "creatures", "players can't gain life". Its rules meaning and its plural
referent are those of "all creatures" (`allOf`), whose tree it shares. -/
semantic_template bare (p : Predicate) : NounPhrase := .described .all p
/-- "a …" -/
semantic_template a (p : Predicate) : NounPhrase := .described (.a .unmarked) p
/-- "a … at random" -/
semantic_template aAtRandom (p : Predicate) : NounPhrase := .described (.a .atRandom) p
/-- "a … of their choice", the chooser being the last-mentioned player. -/
semantic_template aTheirChoice (p : Predicate) : NounPhrase := .described (.a .theirChoice) p
/-- "a … of your choice" -/
semantic_template aYourChoice (p : Predicate) : NounPhrase := .described (.a .yourChoice) p
/-- "the …" -/
semantic_template the (p : Predicate) : NounPhrase := .described (.the .one) p
/-- "the …s", the definite plural: "the exiled cards", "the discarded cards" [CR#607.2a]. -/
semantic_template thePlural (p : Predicate) : NounPhrase := .described (.the .many) p
/-- "N …", "up to N …" -/
semantic_template counted (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q none) p
/-- "N <p> at random" -/
semantic_template countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q (some .atRandom)) p
/-- "if there is a …", "if you control a …": the indefinite is published for the consequent
(`semantics-v3.md` §1.3, box 2). -/
semantic_template exists_ (p : Predicate) : Condition := .exists_ (.described (.a .unmarked) p)
/-- "if there are <np>", as "three or more lands with the same name", "if two or more creatures
are tied for least toughness": a counted indefinite, published for the consequent. -/
semantic_template thereAre (np : NounPhrase) : Condition := .exists_ np
/-- "the number of …s": the domain counted is all of them, closed after the count. -/
semantic_template countOf (p : Predicate) : Amount := .countOf (.described .all p)
/-- "the greatest power among …s": the domain ranged over is all of them, closed after it. -/
semantic_template aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (.described .all p)
semantic_template random (q : Quantity) (p : Predicate) : NounPhrase := countedAtRandom q p
/-! ## Locations -/

semantic_template battlefield : Location := .zone .battlefield none
semantic_template exileZone : Location := .zone .exile none
semantic_template hand : Location := .zone .hand none
semantic_template graveyard : Location := .zone .graveyard none
/-- "its owner's library": a library scoped to nobody, which an object goes to as its owner's
[CR#400.3]. -/
semantic_template library : Location := .zone .library none
semantic_template stack : Location := .zone .stack none
/-- "the command zone", scoped to nobody. -/
semantic_template commandZone : Location := .zone .command none
/-- "<player>'s library": a face-down pile [CR#401.2]. -/
semantic_template libraryOf (player : NounPhrase) : Location := .zone .library player
/-- "<player>'s hand" -/
semantic_template handOf (player : NounPhrase) : Location := .zone .hand player
/-- "<player>'s graveyard" -/
semantic_template graveyardOf (player : NounPhrase) : Location := .zone .graveyard player
/-- "your library" -/
semantic_template yourLibrary : Location := libraryOf you
/-- "libraries": every player's library, as "Players can't search libraries". -/
semantic_template libraries : Location := .zone .library none
/-- "your hand" -/
semantic_template yourHand : Location := handOf you
/-- "your graveyard" -/
semantic_template yourGraveyard : Location := graveyardOf you
/-- "the top of <loc>": the first place from its top [CR#401.7]. -/
semantic_template topOf (loc : Location) : Location := .fromEnd .top (.nth 1) loc
/-- "the bottom of <loc>" -/
semantic_template bottomOf (loc : Location) : Location := .fromEnd .bottom (.nth 1) loc
/-- "<n>th from the top of <loc>", as "seventh from the top" [CR#401.7]. -/
semantic_template fromTop (n : Nat) (loc : Location) : Location := .fromEnd .top (.nth n) loc
/-- "<n>th from the bottom of <loc>" -/
semantic_template fromBottom (n : Nat) (loc : Location) : Location := .fromEnd .bottom (.nth n) loc
/-- "exiled with <source>": the cards its linked ability exiled, face up unless that ability
exiled them face down [CR#406.3,406.6,607.2a]. -/
semantic_template exiledWith (source : NounPhrase) : Location := .exiledWith source
/-- "on the bottom of its owner's library" -/
semantic_template onBottom : Location := .fromEnd .bottom (.nth 1) (.zone .library none)
/-- "on top of its owner's library" -/
semantic_template onTop : Location := .fromEnd .top (.nth 1) (.zone .library none)
semantic_template place (end_ : PileEnd) (pile : Location) : Location := .fromEnd end_ (.nth 1) pile
semantic_template zone (zone : Zone) (scope : Option NounPhrase) : Location := .zone zone scope
/-! ## Predicates

Every `Predicate` template reads as a boolean-like test on the member: `is*` for types, kinds,
roles and states, `has*` for characteristics, and a participle or preposition for possessors,
locations and relations, leaving the bare nouns and statuses free for `NounWord` and `Status`.
-/

/-! ### Combinators

`and`, `or` and `not` combine predicates, conditions, noun phrases, locations, timings and costs,
and `andOr` ("and/or") combines predicates, noun phrases and locations. Each sort has its own
namespaced template (`Predicate.and`, `Condition.or`, `Location.not`, …); the bare spelling is
dispatched on the expected type (`elabExpectedTypeDispatch` below). -/

/-- "<p> and <q>": every conjunct holds. -/
semantic_template Predicate.and (conjuncts : List Predicate) : Predicate := .and conjuncts
/-- "<p> or <q>": some disjunct holds. -/
semantic_template Predicate.or (disjuncts : List Predicate) : Predicate := .or disjuncts
/-- "<p> and/or <q>": some disjunct holds, as "with flying and/or reach". -/
semantic_template Predicate.andOr (disjuncts : List Predicate) : Predicate := .coord .andOr disjuncts
/-- "not <p>" -/
semantic_template Predicate.not (p : Predicate) : Predicate := .not p
/-- "non<p>", as "nonland" or "noncreature". -/
semantic_template non (p : Predicate) : Predicate := .not p
semantic_template Condition.and (conjuncts : List Condition := []) : Condition := .and conjuncts
semantic_template Condition.or (disjuncts : List Condition := []) : Condition := .or disjuncts
semantic_template Condition.not (condition : Condition) : Condition := .not condition
semantic_template NounPhrase.and (phrases : List NounPhrase := []) : NounPhrase := .and phrases
semantic_template NounPhrase.or (phrases : List NounPhrase := []) : NounPhrase := .or phrases
/-- "<np> and/or <np>", as "a card named Magnifying Glass and/or a card named Thinking Cap". -/
semantic_template NounPhrase.andOr (phrases : List NounPhrase := []) : NounPhrase :=
  .coord .andOr phrases
/-- "<loc> and <loc>", as "graveyard, hand, and library". -/
semantic_template Location.and (locs : List Location) : Location := .coord .and locs
/-- "<loc> or <loc>", as "on the top or bottom of your library". -/
semantic_template Location.or (locs : List Location) : Location := .or locs
/-- "<loc> and/or <loc>", as "library and/or graveyard". -/
semantic_template Location.andOr (locs : List Location) : Location := .coord .andOr locs
/-- "anywhere other than <loc>" -/
semantic_template Location.not (loc : Location) : Location := .not loc
semantic_template Timing.and (conjuncts : List Timing := []) : Timing := .and conjuncts
semantic_template Timing.or (disjuncts : List Timing := []) : Timing := .or disjuncts
semantic_template Timing.not (timing : Timing) : Timing := .not timing
semantic_template Cost.or (costs : List Cost := []) : Cost := .or costs

open Lean Elab Term in
/-- Words several sorts share, each sort's template (or definition) by the result type it builds:
the combinators `and`, `or`, `andOr` and `not`; the verb and the determiner `target`; and the
other words whose sorts' templates are namespaced (`did`, `was`, `allBut`, `half`, `token`,
`written`, `copy`, `doIf`, `replace`, `nthOccurrence`, …). A word one of whose sorts is a
Definition (`crew`, `saddle`) is dispatched where that Definition is declared, above this
layer. -/
private def expectedTypeDispatch : List (Name × List (Name × Name)) :=
  [ (`and, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.and),
             (``Semantics.Condition, `Semantics.Templates.Condition.and),
             (``Semantics.NounPhrase, `Semantics.Templates.NounPhrase.and),
             (``Semantics.Location, `Semantics.Templates.Location.and),
             (``Semantics.Timing, `Semantics.Templates.Timing.and) ]),
    (`or, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.or),
            (``Semantics.Condition, `Semantics.Templates.Condition.or),
            (``Semantics.NounPhrase, `Semantics.Templates.NounPhrase.or),
            (``Semantics.Location, `Semantics.Templates.Location.or),
            (``Semantics.Timing, `Semantics.Templates.Timing.or),
            (``Semantics.Cost, `Semantics.Templates.Cost.or) ]),
    (`andOr, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.andOr),
               (``Semantics.NounPhrase, `Semantics.Templates.NounPhrase.andOr),
               (``Semantics.Location, `Semantics.Templates.Location.andOr) ]),
    (`not, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.not),
             (``Semantics.Condition, `Semantics.Templates.Condition.not),
             (``Semantics.Location, `Semantics.Templates.Location.not),
             (``Semantics.Timing, `Semantics.Templates.Timing.not) ]),
    (`target, [ (``Semantics.NounPhrase, `Semantics.Templates.target),
                (``Semantics.Action, `Semantics.Templates.Action.target) ]),
    (`attachment, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.attachment),
                    (``Semantics.Action, `Semantics.Templates.Action.attachment) ]),
    (`duringPart, [ (``Semantics.Timing, `Semantics.Templates.duringPart),
                    (``Semantics.Condition, `Semantics.Templates.Condition.duringPart) ]),
    (`did, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.did),
             (``Semantics.Condition, `Semantics.Templates.Condition.did) ]),
    (`was, [ (``Semantics.Predicate, `Semantics.Templates.Predicate.was),
             (``Semantics.Condition, `Semantics.Templates.Condition.was) ]),
    (`allBut, [ (``Semantics.PreventCut, `Semantics.Templates.PreventCut.allBut),
                (``Semantics.IgnoredOutcomes, `Semantics.Templates.IgnoredOutcomes.allBut) ]),
    (`half, [ (``Semantics.Amount, `Semantics.Templates.half),
              (``Semantics.PreventCut, `Semantics.Templates.PreventCut.half) ]),
    (`token, [ (``Semantics.CreationSpec, `Semantics.Templates.CreationSpec.token) ]),
    (`written, [ (``Semantics.TokenSpec, `Semantics.Templates.written),
                 (``Semantics.AbilityLost, `Semantics.Templates.AbilityLost.written) ]),
    (`ofColor, [ (``Semantics.PipPaid, `Semantics.Templates.ofColor),
                 (``Semantics.ManaTypeTerm, `Semantics.Templates.ManaTypeTerm.ofColor) ]),
    (`copy, [ (``Semantics.Action, `Semantics.Templates.Action.copy) ]),
    (`doIf, [ (``Semantics.Action, `Semantics.Templates.doIf),
              (``Semantics.Replacement, `Semantics.Templates.Replacement.doIf) ]),
    (`replace, [ (``Semantics.Action, `Semantics.Templates.Action.replace),
                 (``Semantics.StaticSpec, `Semantics.Templates.StaticSpec.replace) ]),
    (`nthOccurrence,
      [ (``Semantics.GameEvent, `Semantics.Templates.nthOccurrence),
        (``Semantics.WouldClause, `Semantics.Templates.WouldClause.nthOccurrence) ]) ]

open Lean Elab Term Meta in
/-- Top-down dispatch of a shared word on the expected type: `and [p, q]` elaborates as the one
template its expected type names, so a nested combinator costs one elaboration per node (an
overload would elaborate every alternative, bottom-up) and the elaborated term is that template's
call, as `Authoring.inspect` records it. With no expected type known, or a sort without such a
template, the ordinary application elaborator takes over. -/
def dispatchOnExpectedType (table : List (Name × List (Name × Name))) : TermElab :=
    fun stx expectedType? => do
  let fn := stx[0]
  unless fn.isIdent do throwUnsupportedSyntax
  let some sorts := table.lookup fn.getId | throwUnsupportedSyntax
  if (← getLCtx).findFromUserName? fn.getId |>.isSome then throwUnsupportedSyntax
  let some expected := expectedType? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  let some sort := (← whnfR expected).getAppFn.constName? | throwUnsupportedSyntax
  let some template := sorts.lookup sort | throwUnsupportedSyntax
  let call := stx.setArg 0 (mkCIdentFrom fn template)
  withMacroExpansion stx call <| elabTerm call expectedType?

open Lean Elab Term in
@[scoped term_elab Lean.Parser.Term.app, inherit_doc dispatchOnExpectedType]
def elabExpectedTypeDispatch : TermElab := dispatchOnExpectedType expectedTypeDispatch

/-! ### Types, kinds and roles -/

/-- "permanent": an object on the battlefield [CR#110.1]. -/
semantic_template isPermanent : Predicate := .in_ battlefield
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
semantic_template isSpell : Predicate := .and [.in_ stack, .not (.abilityHead .anyOnStack)]
semantic_template isCard : Predicate := .isCard
semantic_template isToken : Predicate := .isToken
semantic_template isNontoken : Predicate := .not .isToken
semantic_template isCopyOfACard : Predicate := .isCopyOfACard
/-- "<type>, ignoring any type-changing effects that would affect it" [CR#702.44a,613.1d] -/
semantic_template hasTypeIgnoringTypeChanges (type : CardType) : Predicate :=
  .hasTypeIgnoringTypeChanges type
/-- "source": the object dealing the damage under discussion. -/
semantic_template isSource : Predicate := .isSource
semantic_template isEmblem : Predicate := .isEmblem
semantic_template isManaAbility : Predicate := .abilityHead .mana
semantic_template isPlayer : Predicate := .anyPlayer
/-- "an opponent of <who>" [CR#102.2..102.3]. -/
semantic_template isOpponentOf (who : NounPhrase) : Predicate := .isOpponentOf who
/-- "opponent": an opponent of you. -/
semantic_template isOpponent : Predicate := isOpponentOf you
/-- "a teammate of <who>" [CR#102.3]. -/
semantic_template isTeammateOf (who : NounPhrase) : Predicate := .isTeammateOf who
/-- "teammate": a teammate of yours. -/
semantic_template isTeammate : Predicate := isTeammateOf you
/-- "chosen": one the choice in scope chose ("the chosen creature", "the chosen cards"). -/
semantic_template isChosen : Predicate := .chosen .theChoice
/-- "the chosen player" -/
semantic_template isChosenPlayer : Predicate := and [isPlayer, isChosen]
/-- "the last chosen player" -/
semantic_template theLastChosenPlayer : Predicate := and [isPlayer, .chosen .theLatestChoice]
/-- "another player", "other player": a bare "other", whose Contrast Anchor is "you" unless the
text mentions a player first ("Choose a player. Each other player…"). -/
semantic_template isOtherPlayer : Predicate := and [isPlayer, .other]

/-! ### Player groups -/

/-- "your opponents" -/
semantic_template yourOpponents : NounPhrase := allOf isOpponent
/-- "each player" -/
semantic_template eachPlayer : NounPhrase := each isPlayer
/-- "each opponent" -/
semantic_template eachOpponent : NounPhrase := each isOpponent

/-! ### States and characteristics -/

semantic_template isTapped : Predicate := .hasStatus .tapped
semantic_template isUntapped : Predicate := .hasStatus .untapped
semantic_template isFaceDown : Predicate := .hasStatus .faceDown
semantic_template isFaceUp : Predicate := .hasStatus .faceUp
semantic_template isPhasedOut : Predicate := .hasStatus .phasedOut
/-- A transformed permanent is the whole double-faced permanent with its back face up
[CR#701.27g]. -/
semantic_template isTransformed : Predicate := .currentFace .back
/-- "enchanted", "equipped", "fortified": the host of an attachment of that kind, or of any
attachment when `word` is none. -/
semantic_template isAttached (word : Option AttachWord := none) : Predicate :=
  .attachment .host word none
semantic_template isAttacking : Predicate := .inCombat .attackerOf none
semantic_template isBlocking : Predicate := .inCombat .blockerOf none
/-- "blocked": an attacking creature that blockers were declared for, or that an effect made
blocked, until it is removed from combat [CR#509.1h,506.4]. -/
semantic_template isBlocked : Predicate := .inCombat .blockedBy none
/-- "unblocked": an attacking creature once no blocker was declared for it [CR#509.1h], or one
that entered attacking after blockers were declared [CR#508.4d]. Not `not isBlocked`: before
blockers are declared an attacking creature is neither. -/
semantic_template isUnblocked : Predicate := .inCombat .unblocked none
/-- "colorless": an object of no color [CR#105.2c]. -/
semantic_template isColorless : Predicate := .colorCount .eq 0
/-- "monocolored" -/
semantic_template isMonocolored : Predicate := .colorCount .eq 1
/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
semantic_template isMulticolored : Predicate := .colorCount .atLeast 2
/-- "another", "other" with no "other than" written: other than its Contrast Anchor. -/
semantic_template isOther : Predicate := .other

/-! ### Possessors, locations and relations -/

/-- "<who> controls" -/
semantic_template controlledBy (who : NounPhrase) : Predicate := .hasPossessor .controller who
/-- "you control" -/
semantic_template youControl : Predicate := controlledBy you
/-- "you don't control" -/
semantic_template youDontControl : Predicate := not (controlledBy you)
/-- "your opponents control" -/
semantic_template opponentsControl : Predicate := controlledBy yourOpponents
/-- "<who> owns" -/
semantic_template ownedBy (who : NounPhrase) : Predicate := .hasPossessor .owner who
/-- "you own" -/
semantic_template youOwn : Predicate := ownedBy you
/-- "<player>" as the quality of "protection from <player>": each object `player` controls, and
each object `player` owns that no player controls [CR#702.16k]. -/
semantic_template objectsOfPlayer (player : NounPhrase) : Predicate :=
  .or [controlledBy player, .and [ownedBy player, .not (controlledBy (a .anyPlayer))]]
/-- "your opponents own" -/
semantic_template opponentsOwn : Predicate := ownedBy yourOpponents
/-- "in <loc>", as "in your graveyard" or "exiled with this artifact". -/
semantic_template in_ (loc : Location) : Predicate := .in_ loc
/-- "other than <np>" -/
semantic_template otherThan (np : NounPhrase) : Predicate := .otherThan np

/-! ### Characteristics and stat comparisons -/

semantic_template hasKeyword (kw : KeywordTerm) : Predicate := .hasKeyword kw
/-- The keyword a keyword ability is, as "with flying" names it: `hasKeyword flying`, or a list
`[flying, firstStrike, trample]`. A keyword ability is written under its declaration's name
(`.keyword (.named "flying") …`), so that name is the keyword. An ability that is no keyword
names none: its term is the empty label, which names no keyword and is refused
`knownKeywordTerm` wherever a keyword term is checked. -/
semantic_template keywordOf (ability : Ability) : KeywordTerm :=
  match ability with
  | .keyword label _ _ => .the label
  | _ => .the (.named "")
/-- A keyword ability reads as the keyword it is. -/
instance [AuthoringContext] : Coe Ability KeywordTerm := ⟨keywordOf⟩

open Lean Elab Term Meta in
/-- A keyword ability's Definition names its keyword wherever a `KeywordLabel` is expected:
`keywordCostPaid kicker it`, `abilityHead (.keyword cycling)`, `.anyIn ⟨protection, none⟩`. The
identifier elaborates as the authored label constant the Definition generates (`kicker.label`,
`Authoring.keywordLabelConstant`), so a card never spells the internal `KeywordLabel.named` and a
misspelled keyword is no identifier at all. Any other identifier, or an expected type other than `KeywordLabel`, is left to the
ordinary elaborator. -/
@[scoped term_elab ident]
def elabKeywordName : TermElab := fun stx expectedType? => do
  let some expected := expectedType? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  unless expected.getAppFn.isMVar || (← whnfR expected).isConstOf ``Semantics.KeywordLabel do
    throwUnsupportedSyntax
  if (← getLCtx).findFromUserName? stx.getId |>.isSome then throwUnsupportedSyntax
  let candidates ← try resolveGlobalConst stx catch _ => throwUnsupportedSyntax
  let env ← getEnv
  let some definition := candidates.find? (Authoring.definitionAttribute.hasTag env)
    | throwUnsupportedSyntax
  let label := Authoring.keywordLabelConstant definition
  unless env.contains label do throwUnsupportedSyntax
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  return mkConst label

open Lean Elab Term Meta in
/-- An Action Definition names its keyword action wherever a `Deed` is expected:
`.deed exile`. The identifier elaborates as the authored deed constant the
Definition generates (`exile.deed`, `Authoring.deedConstant`), so a pin or a Definition never
spells the internal `Deed.action` and a misspelled action is no identifier at all. Any other
identifier, or an expected type other than `Deed`, is left to the ordinary elaborator. -/
@[scoped term_elab ident]
def elabDeedName : TermElab := fun stx expectedType? => do
  let some expected := expectedType? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  unless expected.getAppFn.isMVar || (← whnfR expected).isConstOf ``Semantics.Deed do
    throwUnsupportedSyntax
  if (← getLCtx).findFromUserName? stx.getId |>.isSome then throwUnsupportedSyntax
  let candidates ← try resolveGlobalConst stx catch _ => throwUnsupportedSyntax
  let env ← getEnv
  let some definition := candidates.find? (Authoring.definitionAttribute.hasTag env)
    | throwUnsupportedSyntax
  let deed := Authoring.deedConstant definition
  unless env.contains deed do throwUnsupportedSyntax
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  return mkConst deed
/-- "with a counter on it", or "with a <kind> counter on it". -/
semantic_template hasCounter (kind : Option CounterKind := none) : Predicate := .hasCounters kind
/-- "named <name>" -/
semantic_template named (name : NameSource) : Predicate := .named name
semantic_template hasType (t : CardType) : Predicate := .hasType t
semantic_template hasSubtype (s : Subtype) : Predicate := .hasSubtype s
/-- A subtype is a predicate on objects of it: `goblin` for `hasSubtype goblin`. -/
instance [AuthoringContext] : Coe Subtype Predicate := ⟨hasSubtype⟩
semantic_template hasSupertype (s : Supertype) : Predicate := .hasSupertype s
semantic_template hasColor (c : Color) : Predicate := .colorIs c
/-- "with power <cmp> <amt>" -/
semantic_template hasPower (cmp : Comparator) (amt : Amount) : Predicate :=
  .compare [.stat .power] cmp amt
/-- "with toughness <cmp> <amt>" -/
semantic_template hasToughness (cmp : Comparator) (amt : Amount) : Predicate :=
  .compare [.stat .toughness] cmp amt
/-- "with mana value <cmp> <amt>" -/
semantic_template hasManaValue (cmp : Comparator) (amt : Amount) : Predicate :=
  .compare [.stat .manaValue] cmp amt

/-! ### Other predicates -/

/-- "of the chosen <quality>" -/
semantic_template ofChosen (sort : QualitySort) : Predicate := .ofChosen .theChoice sort
/-- "of the last chosen <quality>" -/
semantic_template ofTheLastChosen (sort : QualitySort) : Predicate :=
  .ofChosen .theLatestChoice sort
semantic_template cardOnTheStack : Predicate := and [isCard, isSpell]
semantic_template tokenOnTheBattlefield : Predicate := and [isToken, isPermanent]
/-- "that targets only <subject>" [CR#115.9c] -/
semantic_template targetsOnly (subject : NounPhrase) : Predicate := .targets subject .only
/-- "a color", "a creature type": a quality noun. -/
semantic_template quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
semantic_template qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort domain
/-- "itself": the member a comparison over a domain is measuring (`semantics-v3.md` §1.3 Box 5). -/
semantic_template ownMember : NounPhrase := .pro .bare .one
/-- "a <dom> with <stat> <r> <bound>": the stat of `member`, the member itself unless written. -/
semantic_template comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator)
    (bound : Amount) (member : NounPhrase := ownMember) : Predicate :=
  .compareOver domain (.statOf (.stat stat) member) comparator bound
/-- "the chosen color" -/
semantic_template thatColor : ColorTerm := .chosen .theChoice
semantic_template abilityHead (class_ : AbilityClass) : Predicate := .abilityHead class_
semantic_template abilityOf (source : NounPhrase) : Predicate := .abilityOf source
/-- "with <cost> in its activation cost": an activated ability whose cost includes `cost`
[CR#602.1a]. -/
semantic_template activationCostHas (cost : Cost) : Predicate := .activationCostHas cost
/-- "with the tap symbol or the untap symbol in its activation cost" [CR#302.6,107.5,107.6]. -/
semantic_template tapOrUntapInCost : Predicate :=
  .or [activationCostHas .tapSymbol, activationCostHas .untapSymbol]
/-- "that has triggered but not yet left the stack": waiting to be put on the stack, or on it
[CR#113.7,603.3]. -/
semantic_template triggeredNotLeftStack : Predicate := .triggeredNotLeftStack
/-- "another spell" -/
semantic_template anotherSpell : Predicate := .and [isSpell, .other]
semantic_template Predicate.attachment (side : AttachmentSide)
    (counterpart : Option NounPhrase := none) (word : Option AttachWord := none) : Predicate :=
  .attachment side word counterpart
/-- Rules text printed in square brackets: a fact about the wording, so it means what `x` means
and nothing more. The call survives in the authoring tree, where lowering reads it to find the text
that cleave removes [CR#702.148a,702.148b]. -/
semantic_template bracketed {α : Type} (x : α) : α := x
semantic_template cardIn (zone : Location) : Predicate := .and [.isCard, .in_ zone]
/-- "outside the game": in none of the game's zones. Outside the game is not a zone
[CR#400.11], so it is no `Location` but the complement of all seven [CR#400.1]: a sideboard's
cards are there [CR#400.11a], and so is a Lesson that learn reveals [CR#701.48a]. -/
semantic_template outsideTheGame : Predicate :=
  .not (.in_ (.or [library, hand, battlefield, graveyard, stack, exileZone, commandZone]))
/-- "<spell> was <designation>": the spell was a card holding it before it was cast, as "if this
spell was foretold" [CR#702.143c]. -/
semantic_template wasBeforeCast (designation : DesignationLabel) : Predicate :=
  .heldBeforeCast designation
semantic_template choseExtreme (op : AggregateOp) : Predicate := .choseExtreme op
semantic_template coinCameUp (face : CoinFace) : Predicate := .coinCameUp face
semantic_template colorCount (comparator : Comparator) (count : Nat) : Predicate :=
  .colorCount comparator count
semantic_template colorOf (object : NounPhrase) : ColorTerm := .colorOf object
semantic_template compare (comparator : Comparator) (bound : Amount) (axes : List ProjAxis := []) :
    Predicate :=
  .compare axes comparator bound
semantic_template compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator)
    (bound : Amount) : Predicate :=
  .compareOver domain measure comparator bound
semantic_template counterKindOn (subject : NounPhrase) : Predicate := .counterKindOn subject
semantic_template currentFace (side : CardFaceSide) : Predicate := .currentFace side
semantic_template hasDesignation (designation : DesignationLabel)
    (holder : Option NounPhrase := none) : Predicate :=
  .hasDesignation designation holder
semantic_template hasPossessor (axis : PossessorAxis) (possessor : NounPhrase) : Predicate :=
  .hasPossessor axis possessor
semantic_template hasStatus (status : Status) : Predicate := .hasStatus status
semantic_template inCombat (relation : CombatRelation) (counterpart : Option NounPhrase := none) :
    Predicate :=
  .inCombat relation counterpart
semantic_template inPile (pile : NounPhrase) : Predicate := .in_ (.pile pile)
semantic_template manaCostHas (symbol : ManaSymbol) : Predicate := .manaCostHas symbol
semantic_template notChosen : Predicate := .notChosen
/-- "each <value of `axis`> [among <domain>]" as a keyword's quality, as "protection from each
color among permanents you control": one ability per value [CR#702.16h,702.16i]. -/
semantic_template eachKind (axis : KindAxis) (domain : Option NounPhrase := none) : Predicate :=
  .eachKind axis domain
semantic_template ofYourChoice (sort : QualitySort) (domain : Option ChoiceDomain := none) :
    Predicate :=
  .ofYourChoice sort domain
semantic_template qualityNoun (sort : QualitySort) (domain : Option ChoiceDomain := none) :
    Predicate :=
  .qualityNoun sort domain
semantic_template related (label : RelationLabel) (counterpart : Option NounPhrase := none) :
    Predicate :=
  .related label counterpart
semantic_template shares (axis : QualitySort) (agreement : Agreement) (with_ : NounPhrase) :
    Predicate :=
  .shares axis agreement with_
semantic_template superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) :
    Predicate :=
  .superlative op axis domain
/-- "tied for least toughness", "tied for lowest life total": with the extreme `axis` among
`domain`, as `superlative` is. The subject's count ("two or more creatures are tied …") makes it
a tie. -/
semantic_template tiedFor (op : AggregateOp) (axis : ProjAxis) (domain : Predicate) : Predicate :=
  .superlative op axis domain
semantic_template targeting (subject : NounPhrase) (extent : TargetExtent) : Predicate :=
  .targets subject extent
semantic_template withMostVotes : Predicate := .withMostVotes
/-! ## Nouns -/

semantic_template anOpponent : NounPhrase := a (.isOpponentOf .you)
/-- "an opponent controls" -/
semantic_template anOpponentControls : Predicate := controlledBy anOpponent
semantic_template thisPermanent : NounPhrase := .headed .permanent .this
semantic_template thisSpell : NounPhrase := .headed .spell .this
semantic_template thisAbility : NounPhrase := .headed .ability .this
/-- "this card": the source as a card, in whatever zone it is [CR#109.1]. -/
semantic_template thisCard : NounPhrase := .headed .card .this
semantic_template thisToken : NounPhrase := .headed .token .this
semantic_template thisEmblem : NounPhrase := .headed .emblem .this
/-- "the rest of them" -/
semantic_template theRest (kind : Kind) : NounPhrase := .theRest kind .many
/-- "the other pile": the one remaining after a choice among two. -/
semantic_template theOther (kind : Kind) : NounPhrase := .theRest kind .one
/-- "N of <group>" -/
semantic_template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) none group
/-- "all of them": a slice that takes the whole group. -/
semantic_template whole : SliceCount := .whole
/-- "among <group>": the whole of a group, sliced. -/
semantic_template among (group : NounPhrase) : NounPhrase := .someOf .whole none group
/-- "N <p> from among <group>" -/
semantic_template fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) :
    NounPhrase :=
  .someOf (.counted quantity) p group
/-- "<group> with the same name" -/
semantic_template withTheSameName (group : NounPhrase) : NounPhrase := .agree .cardName .share group
/-- "<group> with different names" -/
semantic_template withDifferentNames (group : NounPhrase) : NounPhrase :=
  .agree .cardName .differ group
/-- "the <p> [from] among <group>": every member the description picks. -/
semantic_template allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase :=
  .someOf .whole p group
/-- "each object" -/
semantic_template everyObject : NounPhrase := allOf (.and [])
/-- "one pile": one of the piles just made. -/
semantic_template onePile : NounPhrase := .pileOf (.counted (exactly 1)) none
/-- "the pile of <player>'s choice" -/
semantic_template pileOfChoice (player : NounPhrase) : NounPhrase :=
  .pileOf (.counted (exactly 1)) player
/-- "you and <subject>" -/
semantic_template youAnd (subject : NounPhrase) : NounPhrase :=
  both .you subject
/-- "you or <subject>" -/
semantic_template youOr (subject : NounPhrase) : NounPhrase :=
  eitherOf .you subject
semantic_template controllerOf (subject : NounPhrase) : NounPhrase :=
  .possessorOf .controller subject
semantic_template ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
semantic_template topSlice (amount : Amount) : NounPhrase := .slice .top amount (libraryOf .you)
/-- "the bottom card of your library" -/
semantic_template bottomCard : NounPhrase := .slice .bottom 1 (libraryOf .you)
semantic_template agree (axis : QualitySort) (agreement : Agreement) (group : NounPhrase) :
    NounPhrase :=
  .agree axis agreement group
/-- `subject` named by a Head Noun: "<subject> as a <head>". -/
semantic_template headed (head : NounWord) (subject : NounPhrase) : NounPhrase :=
  .headed head subject
semantic_template attachHost (word : AttachWord) (head : NounWord) : NounPhrase :=
  .attachHost word head
semantic_template becomes (subject : NounPhrase) : NounPhrase := .becomes subject
semantic_template combatPlayer (role : CombatRole) : NounPhrase := .combatPlayer role
semantic_template counterpart (label : RelationLabel) (subject : NounPhrase) : NounPhrase :=
  .counterpart label subject
semantic_template defendingPlayer : NounPhrase := .combatPlayer .defending
semantic_template described (determiner : DetPhrase) (predicate : Predicate) : NounPhrase :=
  .described determiner predicate
semantic_template designated (designation : DesignationLabel) (whose : NounPhrase) : NounPhrase :=
  .designated designation whose
semantic_template eachOf (group : NounPhrase) : NounPhrase := .eachOf group
set_option linter.unusedVariables false in
semantic_template oneEachOf (pool : NounPhrase) (roles : List Predicate := []) : NounPhrase :=
  .oneEachOf roles pool
semantic_template pileOf (count : SliceCount) (by_ : Option NounPhrase := none) : NounPhrase :=
  .pileOf count by_
semantic_template possessorOf (axis : PossessorAxis) (subject : NounPhrase) : NounPhrase :=
  .possessorOf axis subject
semantic_template run (end_ : PileEnd) (stop : Predicate) (pile : Location) : NounPhrase :=
  .run end_ stop pile
semantic_template slice (end_ : PileEnd) (amount : Amount) (pile : Location) : NounPhrase :=
  .slice end_ amount pile
semantic_template theGrantor (head : NounWord) : NounPhrase := .theGrantor head
/-! ### Noun words

The bare nouns a pronoun or an attached host reads: "that creature", "those cards",
"enchanted creature".
-/

semantic_template card : NounWord := .card
semantic_template permanent : NounWord := .permanent
semantic_template player : NounWord := .player
semantic_template token : NounWord := .token
semantic_template copy : NounWord := .copy

/-! ### Attached hosts and targets -/

/-- "enchanted <w>" -/
semantic_template enchanted (w : NounWord) : NounPhrase := .attachHost .enchanted w
/-- "equipped <w>" -/
semantic_template equipped (w : NounWord) : NounPhrase := .attachHost .equipped w
/-- "fortified <w>" -/
semantic_template fortified (w : NounWord) : NounPhrase := .attachHost .fortified w
/-- "<q> target <p>s", as "up to two target creatures" or "any number of targets". -/
semantic_template targets (q : Quantity) (p : Predicate) : NounPhrase := .described (.target q) p

/-! ### Statuses -/

semantic_template tapped : Status := .tapped
semantic_template untapped : Status := .untapped
semantic_template faceDown : Status := .faceDown
semantic_template faceUp : Status := .faceUp
semantic_template phasedIn : Status := .phasedIn
semantic_template phasedOut : Status := .phasedOut
/-! ## Mana -/

semantic_template generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
semantic_template pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
/-- "{C}" -/
semantic_template colorlessPip : ManaSymbol := .simple (.specific .colorless)
/-- "{A/B}" -/
semantic_template hybridPip (left right : Color) : ManaSymbol :=
  .hybrid (.specific (.of left)) right
/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
semantic_template phyrexianPip (color : Color) : ManaSymbol := .phyrexian color none
/-- "{1} for each …": generic mana cost scaled by an amount. -/
semantic_template scaledGenericMana (amount : Amount) : Cost :=
  .scaled (.mana [generic 1]) amount
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
  .between none (some (.beginningOf .turn player (.next none)))
/-- "until <event>" -/
semantic_template untilEvent (event : GameEvent) : Duration := .between none event
/-- "for the rest of the game" -/
semantic_template restOfGame : Duration := .between none none
/-- A rider that modifies the meaning of the action before it, in force only for that action's
event: "It can't be regenerated", "The damage can't be prevented" [CR#608.2c]. -/
semantic_template modifiesPrecedingAction : Duration := .modifiesPrecedingAction
/-- "during <player>'s next turn" (`firstMention` / `laterMention` so `player` is not duplicated
[CR#115.1,608.2c]). -/
semantic_template duringNextTurnOf (player : NounPhrase) : Duration :=
  .between
    (some (.beginningOf .turn (some (.firstMention "duringNextTurnOf.player" player)) (.next none)))
    (some (.endOf .turn (some (.laterMention "duringNextTurnOf.player" player)) (.next none)))
/-- "on a later turn": from the end of this turn on [CR#702.143a,702.170d]. -/
semantic_template onALaterTurn : Duration := .between (some (.endOf .turn none .current)) none
semantic_template between (start end_ : GameEvent) : Duration := .between start end_
semantic_template forAsLongAs (condition : Condition) : Duration := .forAsLongAs condition
/-! ## History windows

A lookback's span ends at `now` ("this turn" so far), except "last turn", which ended. -/

/-- "this turn", as a lookback reads it: from the turn's beginning to now. -/
semantic_template soFarThisTurn : Duration :=
  .between (some (.beginningOf .turn none .current)) (some .now)
/-- "earlier this turn" -/
semantic_template earlierThisTurn : Duration :=
  .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this combat" -/
semantic_template thisCombat : Duration :=
  .between (some (.beginningOf .combat none .current)) (some .now)
/-- "last turn" -/
semantic_template lastTurn : Duration :=
  .between (some (.beginningOf .turn none .previous)) (some (.endOf .turn none .previous))
/-- "this game" -/
semantic_template thisGame : Duration := .between none (some .now)
/-! ## Timings -/

/-- "only as a sorcery": while you have priority, during your main phase, while the stack is
empty [CR#307.5]. Its negation, "any time a sorcery couldn't have been cast", therefore also
holds without priority [CR#307.5a]. -/
semantic_template asSorcery : Timing :=
  .and [.onlyIf (.hasPriority .you), .inPart .mainPhase (some .you) .current,
    .onlyIf (.not (exists_ (.in_ stack)))]
/-- "only as an instant": any time you have priority [CR#117.1a]. -/
semantic_template asInstant : Timing := .onlyIf (.hasPriority .you)
/-- "during <whose> <part>" -/
semantic_template duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .inPart part whose .current
/-- "before <whose> <part>" -/
semantic_template beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .relativeToEvent .before (.beginningOf part whose .current)
/-- "each turn": a rate adjunct, the deontic counted afresh in every turn. -/
semantic_template eachTurn : Timing := .inPart .turn none (.each none)
/-- "on each of your turns", "during each of your turns" -/
semantic_template onEachOfYourTurns : Timing := .inPart .turn (some .you) (.each none)
/-- "each combat": a rate adjunct over every combat phase [CR#506.1]. -/
semantic_template eachCombat : Timing := .inPart .combat none (.each none)
/-- "during their untap steps": each player's own untap steps, "their" the plural pronoun
resolved to the players the deontic subject names ("Players can't untap …") [CR#502.3]. -/
semantic_template duringTheirUntapSteps : Timing :=
  .inPart .untapStep (some (.pro (.word .player) .many)) (.each none)
/-- "during your untap step": each of your untap steps [CR#502.3]. -/
semantic_template duringYourUntapStep : Timing := .inPart .untapStep (some .you) (.each none)
/-- "on each of their turns": each of the turns of the player the deontic subject names, "their"
the singular pronoun resolved distributively ("Each player may play an additional land on each
of their turns"). -/
semantic_template onEachOfTheirTurns : Timing :=
  .inPart .turn (some (.pro (.word .player) .one)) (.each none)
/-- "while you're searching your library" -/
semantic_template whileSearchingYourLibrary : Timing :=
  .whileDoing (.does (.actor .you) (.search yourLibrary .any))
/-- "while <event>" -/
semantic_template whileDoing (event : GameEvent) : Timing := .whileDoing event
/-- "while <condition>" -/
semantic_template whileTrue (condition : Condition) : Timing := .onlyIf condition
/-! ## Usage limits -/

/-- "only once each turn" [CR#602.5b] -/
semantic_template oncePerTurn : UsageLimit := ⟨1, .turn⟩
/-- "only once each game" -/
semantic_template oncePerGame : UsageLimit := ⟨1, .game⟩
/-- "only once during each of your turns" -/
semantic_template onceEachYourTurn : UsageLimit := ⟨1, .yourTurn⟩
/-- "Activate only once": no more than one while the object exists [CR#602.5b,702.177a]. -/
semantic_template oncePerObject : UsageLimit := ⟨1, .thisObject⟩
/-- "<action>. Do this only once each turn.": its ability triggers only if its source's controller
has not yet taken the action this turn [CR#603.2h]. -/
semantic_template doOnlyOnceEachTurn (action : Action) : Action :=
  .limited action oncePerTurn
/-! ## Amounts -/

semantic_template lifeTotalOf (player : NounPhrase) : Amount :=
  .statOf (.playerStat .lifeTotal) player
/-- "<player>'s speed" [CR#702.179a]. -/
semantic_template speedOf (player : NounPhrase) : Amount := .statOf (.playerStat .speed) player
semantic_template powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
semantic_template toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
/-- "<subject>'s base power" [CR#208.4b] -/
semantic_template basePowerOf (subject : NounPhrase) : Amount := .statOf (.stat .basePower) subject
/-- "<subject>'s base toughness" [CR#208.4b] -/
semantic_template baseToughnessOf (subject : NounPhrase) : Amount :=
  .statOf (.stat .baseToughness) subject
semantic_template countersOn (kind : CounterKind) (holder : NounPhrase) : Amount :=
  .statOf (.counter kind) holder
/-- "<saga>'s final chapter number": the greatest of its chapter numbers, 0 with none
[CR#714.2d]. -/
semantic_template finalChapterNumber (saga : NounPhrase) : Amount := .aggregate .max .chapter saga
/-- "<subject>'s total toxic value": the sum of the N values of its toxic abilities
[CR#702.164b]. -/
semantic_template totalToxicValue (subject : NounPhrase) : Amount :=
  .aggregate .sum (.keywordNumber (.named "toxic")) subject
/-- "a chapter ability of <saga>" [CR#714.2,714.2a] -/
semantic_template chapterAbilityOf (saga : NounPhrase) : Predicate :=
  .and [.abilityOf saga, .compare [.chapter] .atLeast 1]
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
semantic_template forEach (per : Amount) (p : Predicate) : Amount := times per (countOf p)
semantic_template aggregateOf (op : AggregateOp) (amounts : List Amount := []) : Amount :=
  .aggregateOf op amounts
semantic_template aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount) : Amount :=
  .aggregateOver op domain body
semantic_template arith (op : ArithOp) (left right : Amount) : Amount := .arith op left right
semantic_template coinsShowing (face : CoinFace) : Amount := .coinsShowing face
semantic_template devotion (player : NounPhrase) (color : ColorTerm)
    (second : Option ColorTerm := none) : Amount :=
  .devotion player color second
semantic_template distinctCount (axis : KindAxis) (domain : NounPhrase) : Amount :=
  .distinctCount axis domain
semantic_template down (amount : Amount) : (Delta Amount) := .down amount
/-- How often, or how much, `subj` did `action` over `within`: a tally always reads a written
span. -/
semantic_template eventTally (subj : Subject) (action : Action) (within : LookbackSpan)
    (mode : TallyMode) : Amount :=
  .tally { subj, action, within } mode
semantic_template greatestStoredMatch (subject : NounPhrase) : Amount :=
  .greatestStoredMatch subject
semantic_template groupSize : Amount := .groupSize
semantic_template half (rounding : RoundMode) (amount : Amount) : Amount := .half rounding amount
semantic_template letter (letter : Letter) : Amount := .letter letter
semantic_template numberOf (subject : NounPhrase) : Amount := .countOf subject
semantic_template paid (facet : LinkedFacet) (subject : NounPhrase) : Amount := .paid facet subject
semantic_template setTo (amount : Amount) : (Delta Amount) := .set amount
semantic_template statOf (axis : ProjAxis) (subject : NounPhrase) : Amount := .statOf axis subject
semantic_template thatMuch : Amount := .thatMuch
semantic_template theDifference : Amount := .theDifference
semantic_template theOutcome (sort : OutcomeSort) : Amount := .theOutcome sort
semantic_template totalOf (axis : ProjAxis) (group : NounPhrase) : Amount :=
  .aggregate .sum axis group
semantic_template up (amount : Amount) : (Delta Amount) := .up amount
semantic_template votesFor (label : String) : Amount := .votesFor label
/-! ## Counters -/

/-- The Characteristic Edit one stat's delta writes: raised, lowered, or set. -/
semantic_template statEdit (axis : CharacteristicStat) (delta : Delta Amount) :
    CharacteristicEdit :=
  match delta with
  | .up amount => .stat .adds axis amount
  | .down amount => .stat .loses axis amount
  | .set amount => .stat .sets axis amount
/-- What one +X/+Y counter confers [CR#122.1a]: "+X/+Y" to its bearer [CR#613.4c]. A counter
definition's conferral is per counter (`Definition.counter`), so the deltas read no count. -/
semantic_template boost (power toughness : Delta Amount) : Conferral :=
  .property (.characteristicChange .this [statEdit .power power, statEdit .toughness toughness])
/-- What one keyword counter confers [CR#122.1b]: its bearer gains the keyword [CR#613.1f], named
by its declaration's name (`grants (.named "flying")`). -/
semantic_template grants (keyword : KeywordLabel) : Conferral :=
  .property (.characteristicChange .this [.addedAbilities [.keyword keyword [] []]])
/-- What the creature type confers [CR#508.1a]: its object is a Combatant. -/
semantic_template combatant : Conferral := .property (.combatant .this)
/-! ## Deontic clauses

A deontic rule constrains a clause: `cant`, `can` (permission [CR#101.2]), `must` and `gatedBy`
(a cost gate). Voice is carried by the `Subject`: an active subject is the bare noun phrase, an
unspecified-agent passive is `any`, an explicit-agent passive is `by_ np`. A quantifier rides the
noun phrase (`counted (moreThan 1) isSpell`), a rate or window rides `timing`. -/

/-- "<subj> can't <action>" [CR#101.2]. `causedBy` restricts the prohibition to an event that
cause causes, as `causedBy` does a would-clause: "damage doesn't cause you to lose life" is
`cant you (loseLife anyAmount) (causedBy := byEvent (does (a isSource) (dealDamage anyAmount you)))`
[CR#120.3a]. -/
semantic_template cant (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none)
    (causedBy : Option Causing := none) : StaticSpec :=
  .cant subj action { timing, asThough, except, cause := causedBy }
/-- "<subj> can <action>": a permission [CR#101.2], with `rider` the later text that modifies the
act it permits [CR#608.2c] (`DeonticMods.rider`). -/
semantic_template can (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none)
    (rider : Option Action := none) : StaticSpec :=
  .can subj action { timing, asThough, except, rider }
/-- "<subj> <action>s as though <condition>": an indicative statement that the act happens as
though the condition were true, for that event only [CR#609.4]; not a permission, which is
`can` [CR#101.2]. -/
semantic_template asThough (subj : Subject) (action : Action) (condition : AsThough) :
    StaticSpec :=
  .asThough subj action condition
/-- "<subj> <action>s if able", "<subj> must <action>". A requirement with the unspecified agent
`any` is one requirement on the whole clause: "this creature must be blocked if able" is one
requirement on the declaration of blockers [CR#509.1c], met when at least one creature blocks it.
`by_ (allOf …)` instead puts a requirement on each creature ("must be blocked by all creatures
able to block it"). -/
semantic_template must (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none) : StaticSpec :=
  .must subj action { timing, asThough, except }
/-- "<subj> can't <action> unless <its controller> pays <cost>" -/
semantic_template gatedBy (cost : Cost) (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none) : StaticSpec :=
  .gatedBy cost subj action { timing, asThough, except }
/-- "except by <objects>": the objects a prohibition exempts [CR#702.9b,702.111b]. -/
semantic_template byObjects (objects : NounPhrase) : DeonticExcept := .byObjects objects

/-! ## Verbs

The verb spine `instruct`, `does`, `would`, `did` and the deontic clauses share. An intransitive
verb (`die`, `enter`, `leave`, `phaseOut`, `phaseIn`, `become`) moves or changes the clause's own
subject, `NounPhrase.actor`; a transitive verb takes its patient and leaves the subject the
performing agent. -/

/-- "<subject> dies": put into a graveyard from the battlefield [CR#700.4]. -/
semantic_template die : Action :=
  .move .actor (.zone .battlefield none) (some (.zone .graveyard none)) .departure []
/-- "<subject> enters [from <from_>]" -/
semantic_template enter (from_ : Option Location := none) (riders : List MoveRider := []) :
    Action :=
  .move .actor (from_.getD .anyLocation) (some (.zone .battlefield none)) .arrival riders
/-- "<subject> leaves <from_>", the battlefield unless said. -/
semantic_template leave (from_ : Location := battlefield) : Action :=
  .move .actor from_ none .departure []
/-- "<subject> phases out" [CR#702.26b] -/
semantic_template phaseOut : Action := .setStatus .phasedOut .actor
/-- "<subject> phases in" [CR#702.26c] -/
semantic_template phaseIn : Action := .setStatus .phasedIn .actor
/-- "<subject> becomes <status>" [CR#603.2e] -/
semantic_template Status.become (status : Status) : Action := .setStatus status .actor
/-- The Zone Change every put and return Template writes: `subject` moves from `from_` to
`destination` with `riders` [CR#400.7]. "Put" and "return" name no deed, so the two verbs expand
to the same move; each Template keeps its own wording. The written origin, not the verb, decides
the Observation Point (`Location.observedOnDeparture`): observation follows the pattern, not an
individual occurrence's origin, so a move from the battlefield or a graveyard (or, to a hand or
library, from exile, the stack or the command zone), or from a list naming one of them, is observed
on its departure, and "from anywhere" is not a leaves-the-battlefield trigger
[CR#603.6c,603.10a]. -/
private semantic_template zoneChange (subject : NounPhrase) (destination : Location)
    (from_ : Location) (riders : List MoveRider) : Action :=
  .move subject from_ destination
    (if from_.observedOnDeparture destination then .departure else .arrival) riders
/-- "put <subject> into <destination> [from <from_>] [<riders>]", as "in any order" or "under
its owner's control". -/
semantic_template putInto (subject : NounPhrase) (destination : Location)
    (from_ : Option Location := none) (riders : List MoveRider := []) : Action :=
  zoneChange subject destination (from_.getD .anyLocation) riders
/-- "turn <subject> face up" -/
semantic_template turnFaceUp (subject : NounPhrase) : Action := .setStatus .faceUp subject
/-- "flip <subject>": a flip card's permanent turns to its flipped half, whose alternative name,
text box, type line, power and toughness apply [CR#710.1a,710.2]. Flipping is no keyword action
[CR#701.1], so this is a Template, naming no deed. -/
semantic_template flip (subject : NounPhrase) : Action := .setStatus .flipped subject
/-- "<subject> attacks [<defender>]" [CR#508.1a] -/
semantic_template attack (defender : Option NounPhrase := none) : Action := .attack defender
/-- "<subject> blocks [<attacker>]" [CR#509.1a] -/
semantic_template block (attacker : NounPhrase := Templates.NounPhrase.any) : Action :=
  .block attacker
/-- "<subject> attacks with <attackers> [<defender>]" -/
semantic_template attackWith (attackers : NounPhrase) (defender : Option NounPhrase := none) :
    Action :=
  .attackWith attackers defender
/-- "target <subject>": the act of targeting [CR#115.1]. -/
semantic_template Action.target (subject : NounPhrase) : Action := .target subject
/-- "<subject> deals [<amount>] [<kind>] damage [to <to>]" -/
semantic_template dealDamage (amount : Amount := anyAmount)
    (to : NounPhrase := Templates.NounPhrase.any) (kind : Option DamageKind := none) : Action :=
  .dealDamage amount to kind
/-- "<amount> damage is marked on <subject>": the result of damage dealt to a creature by a source
with neither wither nor infect [CR#120.3e]. -/
semantic_template markDamage (subject : NounPhrase) (amount : Amount := anyAmount) : Action :=
  .markDamage subject amount
/-- "<subject> wins the game" -/
semantic_template winGame : Action := .conclude .winGame
/-- "<subject> loses the game [for <cause>]": the cause rides the act ("can't lose the game for
having 0 or less life" [CR#704.5a]). -/
semantic_template loseGame (cause : Option StateBasedCause := none) : Action :=
  .conclude .loseGame cause
/-- "<subject> commits a crime" [CR#700.13] -/
semantic_template commitCrime : Action := .commitCrime
/-- "<subject> taps <permanent> for mana [of <type>]" [CR#106.12a] -/
semantic_template tapForMana (permanent : NounPhrase) (type : Option ManaTypeTerm := none) :
    Action :=
  .tapForMana permanent type
/-- "flip <count> coins [and win/lose] [showing <outcome>]" -/
semantic_template flipCoins (count : Amount := anyAmount)
    (call : Option FlipCall := none) (outcome : Option CoinFace := none) : Action :=
  .flipCoins (.count count) call outcome
/-- "roll <count> <sides>-sided dice [and watch for <watch>]": as a pattern, "roll a die" is the
default `1` and "roll one or more dice" `anyAmount`. -/
semantic_template rollDice (count : Amount := 1)
    (sides : DieSides := .sides 6) (watch : RollWatch := anyResult) : Action :=
  .rollDice count sides watch

/-! ## Actions -/

/-- A card exiled face down: no characteristics [CR#406.3a]. -/
semantic_template exiledFaceDown : FaceDown := ⟨none⟩
/-- "put <subject> <destination>" -/
semantic_template put (subject : NounPhrase) (destination : Location) : Action :=
  zoneChange subject destination .anyLocation []
/-- "put <subject> <destination>" with the move's riders ("second from the top", "on the top or
bottom") -/
semantic_template putWith (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider) : Action :=
  zoneChange subject destination .anyLocation riders
/-- "return <subject> to <zone>" -/
semantic_template returnTo (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider) : Action :=
  zoneChange subject destination .anyLocation riders
/-- "return <subject> [from <from_>] to the battlefield [riders]" -/
semantic_template returnToBattlefield (subject : NounPhrase) (riders : List MoveRider := [])
    (from_ : Option Location := none) : Action :=
  zoneChange subject battlefield (from_.getD .anyLocation) riders
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
semantic_template returnToBattlefieldTransformed (subject controller : NounPhrase) : Action :=
  returnTo subject battlefield [.entersTransformed, (.under controller)]
/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
semantic_template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  zoneChange subject battlefield .anyLocation
    [(.under who), (.withCounters amount (.printed kind) .fresh)]
/-- "put <subject> [from <from_>] onto the battlefield" -/
semantic_template putOntoBattlefield (subject : NounPhrase) (from_ : Option Location := none) :
    Action :=
  zoneChange subject battlefield (from_.getD .anyLocation) []
/-- "put <subject> [from <from_>] onto the battlefield attached to <host>" [CR#303.4i] -/
semantic_template putOntoBattlefieldAttachedTo (subject host : NounPhrase)
    (from_ : Option Location := none) : Action :=
  zoneChange subject battlefield (from_.getD .anyLocation) [.attachedTo host]
/-- "put <subject> [from <from_>] onto the battlefield tapped" -/
semantic_template putOntoBattlefieldTapped (subject : NounPhrase)
    (from_ : Option Location := none) : Action :=
  zoneChange subject battlefield (from_.getD .anyLocation) [(.entersAs .tapped)]
/-- "put <subject> [from <from_>] onto the battlefield under your control" -/
semantic_template putOntoBattlefieldUnderYourControl (subject : NounPhrase)
    (from_ : Option Location := none) : Action :=
  zoneChange subject battlefield (from_.getD .anyLocation) [(.under .you)]
/-- "put <subject> onto the battlefield tapped and attacking [<defender>]"; with no defender
named, its controller chooses [CR#508.4]. -/
semantic_template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := .anyLocation) (defender : Option NounPhrase := none) : Action :=
  zoneChange subject battlefield from_ [(.entersAs .tapped), (.entersAttacking defender)]
/-- Losing counters is removal from the named player. -/
semantic_template loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : NounPhrase := you) : Action :=
  .removeCounters (amount.map Quantity.exactlyOf) kind agent
semantic_template loseLife (amount : Amount) : Action :=
  .changeLife (.down amount)
semantic_template gainLife (amount : Amount) : Action :=
  .changeLife (.up amount)
/-- "<player>'s life total becomes <amount>" -/
semantic_template setLife (amount : Amount) : Action :=
  .changeLife (.set amount)
/-- "draw <amount> cards" -/
semantic_template draw (amount : Amount) : Action := .draw (.exactlyOf amount)
/-- "draw up to N cards", "can't draw more than one card": a draw bounded by a Quantity. -/
semantic_template drawCount (quantity : Quantity) : Action := .draw quantity
semantic_template lookAt (cards : NounPhrase) : Action := .expose .lookAt (.cards cards)
/-- "look at <player>'s hand" -/
semantic_template lookAtHandOf (player : NounPhrase) : Action :=
  .expose .lookAt (.zone (handOf player))
semantic_template doIf (condition : Condition) (action : Action)
    (otherwise : Option Action := none) : Action :=
  .doIf condition action otherwise
/-- "choose <subject>" -/
semantic_template choose (subject : NounPhrase) (disclosure : Disclosure := .openly) : Action :=
  .choose none subject disclosure none
/-- "choose <subject> as you <event>" -/
semantic_template chooseWhile (subject : NounPhrase) (while_ : Timing) : Action :=
  .choose none subject .openly while_
/-- One row of a results table: "<results> — <action>". -/
semantic_template rollRow (results : Quantity) (action : Action) : RollRow :=
  ⟨results, action⟩
/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
semantic_template offerWhen (body : Action) (triggerBody : Action) : Action :=
  .triggerReflexively (.withContinuation .optional body none none) none triggerBody
/-- "the chosen number" -/
semantic_template chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
semantic_template chooseSpree (modes : List (Option Cost × Action)) : Action :=
  .chooseModes (orMore 1) (modes.map fun (c, e) => (c, none, e))
/-- "Choose N — <modes>", no mode costing anything. -/
semantic_template chooseModes (quantity : Quantity) (modes : List Action) : Action :=
  .chooseModes quantity (modes.map (none, none, ·))
/-- "Choose N. <frame>" over bullets that are not clauses: the header sentence written once as
`frame`, completed by each bullet in `items` into one mode, no mode costing anything. The modes
are whole, as `chooseModes` writes them; the header names no plural over them, as modes are chosen
before targets [CR#601.2b,601.2c] and a mode's targets are chosen only if it is
[CR#700.2a,700.2c]. -/
semantic_template chooseModesOver {α : Type} (quantity : Quantity) (items : List α)
    (frame : splice (α → Action)) : Action :=
  .chooseModes quantity (items.map fun item => (none, none, frame item))
/-- "<source> deals damage equal to its power to <recipient>": a Handoff of the damage to
`source` [CR#120.1], whose "its" is an ordinary possessive pronoun that `D′` resolves to the
clause subject, `source`. -/
semantic_template dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Action :=
  .instruct source (.dealDamage (powerOf it) recipient)
/-- "for each color of mana spent to cast <n>" -/
semantic_template colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
semantic_template coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast 1
/-- "if no colored mana was spent to cast <spell>" -/
semantic_template noColoredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .eq 0
/-- "the amount of mana spent to cast <spell>" -/
semantic_template manaValueSpentToCast (spell : NounPhrase) : Amount := .paid .manaValueSpent spell
/-- "if no mana was spent to cast <spell>" -/
semantic_template noManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (manaValueSpentToCast spell) .eq 0
/-- "between N and M" -/
semantic_template fromTo (low high : Nat) : Quantity := .range low high
/-- The participant described by the nearest enclosing lookback (`NounPhrase.which`). -/
semantic_template which : NounPhrase := .which
/-- The number of occurrences of a historical action clause. -/
semantic_template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally { subj, action, within := some (.duration within timing) } .events
/-- The accumulated magnitude of historical action events. -/
semantic_template eventSum (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally { subj, action, within := some (.duration within timing) } .quantity
/-- "if there is no <designation>" ("if there is no monarch"): nothing holds it. -/
semantic_template thereIsNo (designation : DesignationLabel) : Condition :=
  .not (.exists_ (.described (.a .unmarked) (.hasDesignation designation none)))
/-- "<subject>'s <keyword> cost was paid", read back. -/
semantic_template paidCostRead (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
semantic_template timesPaid (which : LinkSource) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "if <subject>'s <cost> was paid" -/
semantic_template costWasPaid (which : LinkSource) (window : Option Duration)
    (subject : NounPhrase) : Condition :=
  .compareAmt (paidCostRead which window subject) .atLeast 1
/-- "if <subject>'s <which> was taken": the choice the Link Source offered was taken
[CR#702.104b]. -/
semantic_template choiceWasTaken (which : LinkSource) (subject : NounPhrase) : Condition :=
  .compareAmt (.paid (.taken which) subject) .atLeast 1
/-- "<object>'s mana cost" as the mana a cost shift changes by [CR#702.48a]. -/
semantic_template manaCostOf (object : NounPhrase) : ManaCostTerm := .manaCostOf object
/-- "<subject>'s total cost is reduced by <cost>", symbol by symbol [CR#702.48c,118.7]. -/
semantic_template reducedBySymbols (subject : NounPhrase) (cost : ManaCostTerm) : StaticSpec :=
  .costShift subject (.bySymbols (.down cost) false)
/-- "the last chosen color" -/
semantic_template theLastChosenColor : ColorTerm := .chosen .theLatestChoice
/-- "the last chosen number" -/
semantic_template theLastChosenNumber : Amount := .chosenNumber .theLatestChoice
/-- "the amount by which the ceiling was not reached" -/
semantic_template shortOfCeiling : Amount := .theOutcome .ceilingShortfall
/-- "the number of counters removed this way" -/
semantic_template removedThisWay : Amount := .theOutcome .countersRemoved
/-- "increase or decrease the result by <amount>" -/
semantic_template shiftResult (amount : Amount) : Action := .shiftResult [.up amount, .down amount]
/-- "You may play this card from <zone> <timing>": the land type's play permission [CR#305.1]. -/
semantic_template playableFrom (zone : Location) (timing : Timing) : Conferral :=
  .property (can you (.procedure .play this_ .itsOwnCost none zone none)
    (timing := timing))
/-- "You may cast this card from <zone> <timing>": the instant type's cast permission
[CR#304.1]. -/
semantic_template castableFrom (zone : Location) (timing : Timing) : Conferral :=
  .property (can you (.procedure .cast this_ .itsOwnCost none zone none)
    (timing := timing))
/-- "When <event>, <action>" as a delayed trigger. -/
semantic_template delay (event : GameEvent) (action : Action)
    (alternatives : List GameEvent := []) : Action :=
  .delay event alternatives none action
/-- "When <event> <duration>, <action>": a delayed trigger with a window. -/
semantic_template delayWithin (event : GameEvent) (duration : Duration) (action : Action)
    (alternatives : List GameEvent := []) : Action :=
  .delay event alternatives duration action
/-- "<action> at <event> unless <player> pays <cost> before that step": the delayed trigger, and a
special action `player` may take until it would trigger, paying `cost` to stop it from triggering
[CR#116.2c]: the special action establishes that the delayed trigger can't trigger, with no
Duration, since the text states none. Once it has triggered, paying no longer stops it. The text names the delayed trigger nowhere, so the special action reads
it by its Binding Identity (`delay.trigger`). -/
semantic_template delayUnlessPaid (event : GameEvent) (action : Action) (player : NounPhrase)
    (cost : Cost) : Action :=
  .sequentially
    [ .delay event [] none action,
      .establish
        (.specialAction (some player) none cost
          (.establish (.cant (.actor (.laterMention "delay.trigger" thisAbility)) .trigger {}) none))
        (some (.between none (some event))) ]
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
/-- "<source> deals N damage divided as you choose among <among>" -/
semantic_template dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) :
    Action :=
  .distribute (.damage source) amount among
/-- "distribute N <kind> counters among <among>" -/
semantic_template distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) :
    Action :=
  .distribute (.counters kind) amount among
/-- "remove <q> <kind> counters from <from>" -/
semantic_template removeCounters (quantity : Quantity) (kind : Option CounterKindSource)
    (from_ : NounPhrase) : Action :=
  .removeCounters quantity kind from_
/-- "<who> loses all [<kind>] counters" -/
semantic_template loseAllCounters (kind : Option CounterKindSource)
    (agent : NounPhrase := you) : Action :=
  loseCounters kind none (agent := agent)
/-- "remove all [<kind>] counters from <from>" -/
semantic_template removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Action :=
  .removeCounters none kind from_
/-- `cost` as what paying it pays. Paying a cost carries out its instructions [CR#118.1], so where
the cost is itself one plain payment ("Ward—Pay 5 life", "unless you pay 1 life"), paying it is that
payment, made once: `pay (payLife 5)` is `payLife 5`, never a payment of a payment. Its Reference
Scope (`inCaller`), its scaling ("for each age counter") and the other parts of a compound cost
are kept. -/
def _root_.Semantics.Cost.asPaid (cost : Cost) (fuel : Nat := 16) : Cost :=
  match fuel, cost with
  | _ + 1, .perform (.pay inner .once .paid none none) => inner
  | n + 1, .inCaller body => .inCaller (body.asPaid n)
  | n + 1, .scaled body amount => .scaled (body.asPaid n) amount
  | n + 1, .compound costs => .compound (costs.map (·.asPaid n))
  | _, cost => cost
termination_by structural fuel
/-- "may pay <cost>. If they don't, <action>." -/
semantic_template doUnless (action : Action) (cost : Cost) : Action :=
  .withContinuation .optional (.pay cost.asPaid .once .paid none none) none action
/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
semantic_template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .characteristicChange subject [statEdit .power power, statEdit .toughness toughness]
/-- "<subject> has base power and toughness P/T" -/
semantic_template getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPt subject (.set power) (.set toughness)
/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` for `slice`). -/
semantic_template lookAndSortSlice (slice : NounPhrase) (spill : Location)
    (spillRiders : List MoveRider := []) : Action :=
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anyLocation
        spill .arrival spillRiders,
      .move (theRest .object) .anyLocation onTop .arrival
        [(.inOrder .anyOrder)] ]
/-- "<subject> gets +P/+T [until …]" as an action. -/
semantic_template get (subject : NounPhrase) (power toughness : Delta Amount)
    (duration : Option Duration := none) : Action :=
  .establish (getsPt subject power toughness) duration
semantic_template gain (subject : NounPhrase) (ability : Ability)
    (duration : Option Duration := none) : Action :=
  .establish (.characteristicChange subject [.addedAbilities [ability]]) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
semantic_template become (subject : NounPhrase) (added : CharacteristicBundle)
    (duration : Option Duration := none) : Action :=
  .establish (qualityChange subject .adds (.bundle added none)) duration
/-- "<subject> becomes <colors> [until …]" -/
semantic_template becomeColor (subject : NounPhrase) (colors : ColorSpec)
    (duration : Option Duration := none) : Action :=
  .establish (qualityChange subject .sets (.colored colors)) duration
/-- Several static clauses sharing one subject, as one action. -/
semantic_template establishFor (subject : NounPhrase) (parts : List StaticSpec)
    (duration : Option Duration := none) : Action :=
  .establish (.conjunction subject parts) duration
/-- "If <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceEvent (clause : WouldClause) (replacement : Replacement)
    (duration : Option Duration) : Action :=
  .establish (.replace clause replacement none .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt [to <to>] [duration]": a prevention effect,
a replacement of each such damage event by its prevention [CR#615.1]. `to` left `any` is damage
dealt to anything. -/
semantic_template preventAll (to : NounPhrase) (duration : Option Duration)
    (kind : Option DamageKind := none) : Action :=
  .establish
    (.replace (.would .any (.dealDamage .anyAmount to kind))
      (.instead (.preventDamage .all none none)) none .repeatedly none)
    duration
/-- "Prevent the next N <kind> damage that would be dealt [to <to>] [duration]": a shield, used
up as it prevents [CR#615.7]. -/
semantic_template preventNext (to : NounPhrase) (amount : Amount) (duration : Option Duration)
    (kind : Option DamageKind := none) : Action :=
  .establish
    (.replace (.would .any (.dealDamage .anyAmount to kind))
      (.instead (.preventDamage (.shield amount) none none)) none .repeatedly none)
    duration
/-- "Prevent all <kind> damage that would be dealt [to <to>] by <source> [duration]" [CR#615.1]. -/
semantic_template preventAllBy (source : NounPhrase) (to : NounPhrase)
    (duration : Option Duration) (kind : Option DamageKind := none) : Action :=
  .establish
    (.replace (.would (.by source) (.dealDamage .anyAmount to kind))
      (.instead (.preventDamage .all none none)) none .repeatedly none)
    duration
/-- "The next time <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceNextEvent (clause : WouldClause) (replacement : Replacement)
    (duration : Option Duration) : Action :=
  .establish (.replace clause replacement none .nextTimeOnly none) duration
/-- "[<controller>] gain[s] control of <subject> [duration]": the performer by default, so "gain
control of target creature" and "target player gains control of …" (`instruct`) read alike
[CR#613.1b]. -/
semantic_template gainControl (subject : NounPhrase) (duration : Option Duration := none)
    (controller : NounPhrase := .actor) : Action :=
  .establish (.controlGrant controller subject) duration
/-- "gain control of <subject> [from <from_>]" as an event or a forbidden act: "whenever an
opponent gains control of a permanent from you", a trigger that looks back in time [CR#603.10d],
or "can't gain control of them". The imperative is `gainControl`. -/
semantic_template gainControlOf (subject : NounPhrase) (from_ : Option NounPhrase := none) :
    Action :=
  .changeControl subject from_
/-- "<spec> as long as <condition>" -/
semantic_template onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .asLongAs
/-- "<spec> unless <condition>": "<spec> if not <condition>". -/
semantic_template onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec (.not condition) .ifSo
/-- "<spec> if <condition>" -/
semantic_template onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .ifSo
/-- "<spec> <duration>": a clause holding for a stated duration. -/
semantic_template establishThroughout (spec : StaticSpec) (duration : Duration) : Action :=
  .establish spec duration
/-- "<spec> during [<whose>] untap step" -/
semantic_template duringUntapStep (whose : Option NounPhrase) (spec : StaticSpec) : StaticSpec :=
  partScope .untapStep whose spec
semantic_template addMana (amount : Amount) (produced : ProducedMana)
    (riders : List ManaRider := []) : Action :=
  .addMana amount produced riders
semantic_template addTurnPart (part : TurnPart) (count : Amount)
    (anchor : Option TurnPart := none) (followedBy : Option TurnPart := none) : Action :=
  .addTurnPart part anchor count followedBy
semantic_template atNext (part : TurnPart) (action : Action) : Action :=
  delay (.beginningOf part none (.next none)) action
semantic_template Action.attachment (move : AttachMove) (subject : NounPhrase)
    (host : Option NounPhrase := none) : Action :=
  .attachment move subject host
semantic_template changeLife (delta : Delta Amount) : Action := .changeLife delta
semantic_template chooseNewTargets (subject : NounPhrase) : Action := .chooseNewTargets subject
semantic_template chooseOne (first second : Action) : Action :=
  .chooseModes (exactly 1) [(none, none, first), (none, none, second)]
semantic_template clearDamage (subject : NounPhrase) : Action := .clearDamage subject
semantic_template Action.combat (subject : NounPhrase) (update : CombatUpdate) : Action :=
  .combat subject update
semantic_template conclude (verb : OutcomeVerb) : Action := .conclude verb none
/-- "copy <subject> [<times> times] [, except …]": a copy of a spell or of an ability on the stack,
put onto the stack (Twincast's spell, Strionic Resonator's triggered ability, Fork's red copy)
[CR#707.10]. -/
semantic_template copyOnStack (subject : NounPhrase) (times : Amount := 1)
    (exceptions : List CopyExcept := []) : Action :=
  .copy .fromStack subject times exceptions
semantic_template copyTargets (copy original : NounPhrase) : Action := .copyTargets copy original
semantic_template createDoubleFacedToken (front back : CharacteristicBundle)
    (riders : List MoveRider := []) : Action :=
  .createObject 1 (.token (.doubleFaced front back) riders) none
semantic_template createObject (count : Amount) (spec : CreationSpec)
    (under : Option NounPhrase := none) : Action :=
  .createObject count spec under
semantic_template distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase) : Action :=
  .distribute verb amount among
/-- "<action>. <forbidden> can't happen this turn": the action, then a prohibition of
`forbidden` until end of turn, its patient read back from the action ("It can't be
regenerated"). -/
semantic_template doAndForbid (action forbidden : Action) : Action :=
  .sequentially [action, .establish (.cant .any forbidden {}) untilEndOfTurn]
/-- "<action>. <forbidden> can't happen", with no duration printed: later text that modifies the
meaning of `action` (`Duration.modifiesPrecedingAction`), a prohibition in force only for that
action's event, its subject a mention of its own read back from the action ("It can't be
regenerated"). A destroy with this rider is one no regeneration shield replaces
[CR#608.2c,701.19c]. -/
semantic_template doUnderProhibition (action forbidden : Action) : Action :=
  .sequentially [action, .establish (.cant .any forbidden {}) (some .modifiesPrecedingAction)]
semantic_template doForEach (group : NounPhrase) (body : Action) : Action := .doForEach group body
semantic_template doForEachKind (axis : KindAxis) (sort : QualitySort) (body : Action)
    (domain : Option NounPhrase := none) : Action :=
  .doForEachKind axis domain sort body
semantic_template doIfDone (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action :=
  .withContinuation .required body ifDid ifNot
/-- "<action> if <condition>" ("counter target spell if its mana value is X"): the conditional
action with its condition printed after it (`postposed`): the condition reads the action's
targets and its mentions of earlier referents as the clause ranks them ("the creature you control
gets +2/+1 until end of turn if it's a Knight"), but nothing the action would choose or create
as it resolves [CR#601.2c,608.2c]. -/
semantic_template doOnlyIf (action : Action) (condition : Condition)
    (otherwise : Option Action := none) : Action :=
  .doIf condition action otherwise (postposed := true)
/-- "the game is a draw" [CR#104.4c] -/
semantic_template drawGame : Action := .conclude .drawGame none
semantic_template establish (spec : StaticSpec) (duration : Option Duration := none) : Action :=
  .establish spec duration
semantic_template expose (verb : ExposeVerb) (exposed : Exposed) : Action := .expose verb exposed
semantic_template gainDesignation (subject : NounPhrase) (designation : DesignationLabel)
    (duration : Option Duration := none) : Action :=
  .gainDesignation subject designation duration
semantic_template getNamedEmblem (name : String) (abilities : List Ability) : Action :=
  createObject 1 (.emblem name abilities)
semantic_template getsBoth (subject : NounPhrase) (delta : Delta Amount)
    (duration : Option Duration := none) : Action :=
  get subject delta delta duration
semantic_template holdUntil (action : Action) (event : GameEvent) : Action :=
  .holdUntil action event
semantic_template ignoreOutcomes (which : IgnoredOutcomes) : Action := .ignoreOutcomes which
semantic_template limited (action : Action) (limit : UsageLimit) : Action :=
  .limited action limit
semantic_template may (body : Action) (ifDid : Option Action := none)
    (ifNot : Option Action := none) : Action :=
  .withContinuation .optional body ifDid ifNot
semantic_template mayOrElse (body otherwise : Action) : Action := may body none otherwise
semantic_template merge (subject onto : NounPhrase) (position : MergePosition) : Action :=
  .merge subject onto position
/-- "move <amount> [<kind>] counters from <source> onto <destination>": the all-or-nothing move
(`Action.moveCounters`), a removal from the source and a placement on the destination
[CR#122.5]. With no kind written, what is put is counters of the kind removed (`those`). -/
semantic_template moveCounters (amount : Amount) (source destination : NounPhrase)
    (kind : Option CounterKindSource := none) : Action :=
  .moveCounters amount kind source destination
/-- "pay <cost>": paying a cost carries out its instructions [CR#118.1]. A cost that is itself one
plain payment ("Pay 5 life", `payLife`) is paid by making that payment once, not by paying a
payment (`Cost.asPaid`). -/
semantic_template pay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome := .paid)
    (whose : Option NounPhrase := none) (keyword : Option KeywordLabel := none) : Action :=
  .pay cost.asPaid times outcome whose keyword
semantic_template procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
    (resulting : Option Predicate := none) (from_ : Option Location := none)
    (of_ : Option NounPhrase := none) : Action :=
  .procedure kind subject payment resulting from_ of_
semantic_template putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase) :
    Action :=
  .putCounters amount kind on
semantic_template relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase) :
    Action :=
  .relate subject label counterpart
semantic_template repeat_ (repetition : Repetition) : Action := .repeat_ repetition
/-- "<replaced>. <replacement>": a self-replacement, the spell's or ability's own `replaced`
action replaced by `replacement`, as "draw a card; if …, draw two cards instead" [CR#614.15]. -/
semantic_template Action.replace (replaced : Action) (replacement : Replacement) : Action :=
  .replace replaced replacement
semantic_template rerollStored (quantity : Quantity) (whose : NounPhrase) : Action :=
  .rerollStored quantity whose
semantic_template restartGame : Action := .restartGame
/-- "return <subject> [from <from_>] to its owner's hand" -/
semantic_template returnToHand (subject : NounPhrase) (from_ : Location := wherever) :
    Action :=
  zoneChange subject hand from_ []
semantic_template revealChoices (sort : HiddenSort) : Action := .revealChoices sort
semantic_template selectRandom (quantity : Quantity) (what : Predicate) : Action :=
  choose (countedAtRandom quantity what)
semantic_template separateIntoPiles (group : NounPhrase) (piles : Nat)
    (faces : List PileFace := []) : Action :=
  .separateIntoPiles group piles faces
semantic_template sequentially (steps : List Action := []) : Action := .sequentially steps
/-- "it becomes <designation>": the game gains a designation the game can have [CR#731.1]. -/
semantic_template setGameDesignation (designation : DesignationLabel) : Action :=
  .gameGains designation
semantic_template setStatus (status : Status) (subject : NounPhrase) : Action :=
  .setStatus status subject
semantic_template simultaneously (steps : List Action := []) : Action := .simultaneously steps
semantic_template skipPart (part : TurnPart) (count : Amount) : Action := .skipPart part count
semantic_template skipUntap (subject : NounPhrase) (steps : Amount) : Action :=
  .skipUntap subject steps
semantic_template storeResults (on : NounPhrase) : Action := .storeResults on
/-- "<body>. When you do, <trigger>" [CR#603.12] -/
semantic_template triggerReflexively (body triggerBody : Action) : Action :=
  .triggerReflexively body none triggerBody
/-- "<body>. When <event> this way, <trigger>" [CR#603.12] -/
semantic_template triggerThisWay (body : Action) (event : GameEvent) (triggerBody : Action) :
    Action :=
  .triggerReflexively body event triggerBody
semantic_template turnFaceDown (subject : NounPhrase) (spec : FaceDown) : Action :=
  .turnFaceDown subject spec
semantic_template turnOver (subject : NounPhrase) : Action := .turnOver subject none
semantic_template unlock (door : Door) : Action := .unlock door
semantic_template withContinuation (policy : ContinuationPolicy) (body : Action)
    (ifDid : Option Action := none) (ifNot : Option Action := none) : Action :=
  .withContinuation policy body ifDid ifNot
/-! ## The performer and the handoff -/

/-- "whoever performs this action" -/
semantic_template theActor : NounPhrase := .actor

/-- "<performer> <body>": the handoff (`Action.instruct`), to a player or a permanent. -/
semantic_template instruct (performer : NounPhrase) (body : Action) : Action :=
  .instruct performer body
/-- "… <performer> controls" -/
semantic_template actorControls : Predicate := .hasPossessor .controller .actor
/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a]. -/
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
/-- "any number of <group> with total <axis> <comparator> <bound>" [CR#701.59a]. -/
semantic_template withTotal (axis : ProjAxis) (comparator : Comparator) (bound : Amount)
    (group : NounPhrase) : NounPhrase :=
  .someOf (.totalling axis comparator bound) none group

/-! ## Events -/

/-- "<subj> <action>s": an action event, in the voice `subj` carries. -/
semantic_template does (subj : Subject) (action : Action) : GameEvent := .does subj action
/-- "if <subj> would <action>": the event a replacement watches [CR#614.1]. -/
semantic_template would (subj : Subject) (action : Action) : WouldClause := .would subj action
/-- "if <clause> or <clause> would happen": any of several events [CR#614.1]. -/
semantic_template wouldAny (clauses : List WouldClause) : WouldClause := .wouldAny clauses
/-- "the <ordinal> time <clause> [each <per>]": the replaced event's ordinal, as "The first time
you would create one or more tokens each turn" [CR#614.1]. -/
semantic_template WouldClause.nthOccurrence (ordinal : Ordinal) (clause : WouldClause)
    (per : Option Period := none) : WouldClause :=
  .nthOccurrence ordinal per clause
/-- "at the beginning of <possessor>'s <part>", each time it comes -/
semantic_template beginningOfPossessed (part : TurnPart) (possessor : NounPhrase) : GameEvent :=
  .beginningOf part possessor (.each none)
/-- "at the beginning of your <part>", each time it comes -/
semantic_template beginningOfYour (part : TurnPart) : GameEvent :=
  .beginningOf part (some .you) (.each none)
/-- "at the beginning of each <part>" -/
semantic_template beginningOfEach (part : TurnPart) : GameEvent :=
  .beginningOf part none (.each none)
/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
semantic_template beginningOfNext (part : TurnPart) : GameEvent :=
  .beginningOf part none (.next none)
/-- "at the beginning of that turn's <part>" -/
semantic_template beginningOfThatTurns (part : TurnPart) : GameEvent := .beginningOf part none .that
/-- "<amount> <kind> counters are put on <subject>": "a +1/+1 counter is put" (`1`),
"one or more +1/+1 counters are put" (`anyAmount`) or "the twelfth hour counter is put"
(`theNth (.nth 12)`). -/
semantic_template countersPut (kind : CounterKind) (amount : Amount) (subject : NounPhrase) :
    GameEvent :=
  .does .any (.putCounters amount (.printed kind) subject)
/-- "<quantity> <kind> counters are removed from <subject>": "a … counter is removed"
(`exactly 1`) or "one or more … are removed" (`orMore 1`). -/
semantic_template countersRemoved (kind : CounterKind) (quantity : Quantity)
    (subject : NounPhrase) : GameEvent :=
  .does .any (.removeCounters quantity (some (.printed kind)) subject)
/-- "<amount> counters are put on <subject>", the kind unsaid. -/
semantic_template bareCountersPut (amount : Amount) (subject : NounPhrase) : GameEvent :=
  .does .any (.putCounters amount .anyKind subject)
/-- "<quantity> counters are removed from <subject>", the kind unsaid. -/
semantic_template bareCountersRemoved (quantity : Quantity) (subject : NounPhrase) : GameEvent :=
  .does .any (.removeCounters quantity none subject)
/-- "the last <kind> counter is removed from <subject> by <who>" -/
semantic_template lastCounterRemovedBy (kind : CounterKind) (subject who : NounPhrase) :
    GameEvent :=
  .does (.by who) (.removeCounters (some .theLast) (some (.printed kind)) subject)
/-- "one or more counters are put on <subject> by an effect" -/
semantic_template manyCountersPutByEffect (subject : NounPhrase) : GameEvent :=
  .causes .anEffect (.does .any (.putCounters .anyAmount .anyKind subject))
/-- "<who> puts one or more counters on <subject>" -/
semantic_template manyBareCountersPutBy (who subject : NounPhrase) : GameEvent :=
  .does (.actor who) (.putCounters .anyAmount .anyKind subject)
/-- A token-creation event, retaining its creator and controller. -/
semantic_template tokenCreationEvent (tokens : NounPhrase)
    (by_ under : Option NounPhrase) : GameEvent :=
  .does (match by_ with | some creator => .by creator | none => .any)
    (.createObject .anyAmount (.token (.copyOf tokens []) []) under)
/-- "one or more tokens would be created" -/
semantic_template tokensCreated (tokens : NounPhrase) : GameEvent :=
  tokenCreationEvent tokens none none
/-- "one or more tokens would be created under <under>'s control by an effect" -/
semantic_template tokensCreatedByEffectUnder (tokens under : NounPhrase) : GameEvent :=
  .causes .anEffect (tokenCreationEvent tokens none under)
/-- "the last <kind> counter is removed from <subject>" -/
semantic_template lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .does .any (.removeCounters (some .theLast) (some (.printed kind)) subject)
/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
semantic_template copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability
semantic_template beginningOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent :=
  .beginningOf part whose which
semantic_template causes (cause : Causing) (event : GameEvent) : GameEvent := .causes cause event
semantic_template chapterMark (chapters : List Nat := []) : GameEvent := .chapterMark chapters
semantic_template endOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent :=
  .endOf part whose which
/-- "whenever it becomes <designation>": the game gaining it [CR#731.1,731.1a]. -/
semantic_template gameBecomes (designation : DesignationLabel) : GameEvent :=
  .does .any (.gameGains designation)
semantic_template nthOccurrence (ordinal : Ordinal) (event : GameEvent)
    (per : Option Period := none) : GameEvent :=
  .nthOccurrence ordinal per event
semantic_template stateHolds (condition : Condition) : GameEvent := .stateHolds condition
/-- "<ability> triggers [by <cause>]": the ability is the subject of `trigger` [CR#603.2]; whose
ability it is ("a triggered ability of a creature you control") is `Predicate.abilityOf` in its
description, and what made it trigger is a cause. -/
semantic_template triggers (ability : NounPhrase) (by_ : Option NounPhrase := none) : GameEvent :=
  match by_ with
  | none => .does (.actor ability) .trigger
  | some cause => .causes (.source cause) (.does (.actor ability) .trigger)
/-! ## Abilities -/

/-- A keyword ability with no parameters and no written definition, by its label. This and the
`keyword…` helpers below are not Templates: they spell `Ability.keyword`, an internal expansion
carrier, so they serve Definitions and fixtures, and `spelled` refuses them in a card, which
writes the keyword's typed Definition instead. -/
def keyword (label : KeywordLabel) : Ability := .keyword label [] []
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
semantic_template abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
semantic_template flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "<keyword> <cost>" -/
def keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] []
/-- "<keyword> <quality>", e.g. "protection from red" -/
def keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] []
/-- "<keyword> <subject>", e.g. "enchant creature" -/
def keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] []
/-- "<keyword> N", e.g. "bushido 2" -/
def keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
def keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) :
    Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
def keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) :
    Ability :=
  .keyword label [.number amount, .cost cost] []
/-- "As <subject> enters, [secretly] choose <chosen>": a replacement effect whose choice is made
before the permanent enters [CR#603.6d,614.1c,614.12a]. -/
semantic_template entryChoice (subject chosen : NounPhrase) (disclosure : Disclosure := .openly) :
    StaticSpec :=
  .replace (would subject enter) (.before (.choose none chosen disclosure none) none) none
    .repeatedly none
/-- "As <subject> becomes attached, choose <chosen>": an Off-Stack Static whose choice is made as
it becomes attached. -/
semantic_template attachmentChoice (subject chosen : NounPhrase) : StaticSpec :=
  .asEvent (does .any (.attachment .attached subject none)) (.choose none chosen .openly none)
/-- "As <subject> enters, choose a <quality>." -/
semantic_template entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  entryChoice subject (a (quality sort))
/-- "As <subject> enters, choose a <quality> from <domain>." -/
semantic_template entersChoosingFrom (subject : NounPhrase) (sort : QualitySort)
    (domain : ChoiceDomain) : StaticSpec :=
  entryChoice subject (a (qualityFrom sort domain))
/-- "As <subject> enters, choose <player>", as "choose an opponent". -/
semantic_template entersChoosingPlayer (subject player : NounPhrase) : StaticSpec :=
  entryChoice subject player
/-- "As <subject> enters, secretly choose <player>". -/
semantic_template entersChoosingPlayerSecretly (subject player : NounPhrase) : StaticSpec :=
  entryChoice subject player .secretly
/-- "<subject> enters <riders>", a static ability: as `subject` would enter, it still enters,
with `riders` ("this land enters tapped", "creatures you control enter with an additional +1/+1
counter on them") [CR#603.6d,614.1c,614.1d]. -/
semantic_template enters (subject : NounPhrase) (riders : List MoveRider) : StaticSpec :=
  .replace (would subject enter) (.withRiders riders) none .repeatedly none
/-- "<subject> enters tapped" -/
semantic_template entersTapped (subject : NounPhrase) : StaticSpec :=
  enters subject [.entersAs .tapped]
/-- "<subject> enters <riders> unless <condition>": the condition is checked as `subject` would
enter, so the replacement carries it, not the static ability; while it holds, `subject` enters
unmodified [CR#614.1d,614.12]. -/
semantic_template entersUnless (subject : NounPhrase) (riders : List MoveRider)
    (condition : Condition) : StaticSpec :=
  .replace (would subject enter) (.onlyIf (.not condition) (.withRiders riders)) none .repeatedly
    none
/-- "<subject> enters tapped unless <condition>" -/
semantic_template entersTappedUnless (subject : NounPhrase) (condition : Condition) : StaticSpec :=
  entersUnless subject [.entersAs .tapped] condition
/-- "As <subject> becomes attached, choose a <quality>." -/
semantic_template attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  attachmentChoice subject (a (quality sort))
/-- "<subject> enters with N <kind> counters on it" -/
semantic_template entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  enters subject [.withCounters amount (.printed kind) .fresh]
/-- "<subject> enters with an additional N <kind> counters on it" -/
semantic_template entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec :=
  enters subject [.withCounters amount (.printed kind) .additional]
/-- "<subject> enters with N fewer <kind> counters on it" -/
semantic_template entersWithFewerCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec :=
  enters subject [.withCounters amount (.printed kind) .fewer]
/-- A further "whenever <event>" header joined onto a trigger. -/
semantic_template joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
/-- A joined "whenever <event> while <concurrent>" header. -/
semantic_template joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader :=
  ⟨event, [], while_, none⟩
/-! The trigger words. `when`, `whenever` and `at_` expand to the same triggered
ability [CR#603.1,113.3c]; the word is the Template's printed wording, never a field. Each takes the
header's optional parts: alternative events (`or_`, "whenever <event> or <event>"), further
headers (`joins`, "and whenever …"), a concurrent condition (`while_`), a window (`during`), a
limit ("this ability triggers only once each turn", `limit`) and an intervening "if" clause
(`if_`, "When/Whenever/At [trigger event], if [condition], [effect]" [CR#603.4]). -/
/-- "When <event>, <action>" -/
semantic_template when (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability :=
  .triggered event or_ while_ joins during limit if_ action
/-- "Whenever <event>, <action>" -/
semantic_template whenever (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability :=
  .triggered event or_ while_ joins during limit if_ action
/-- "At <event>, <action>" (`at` is a Lean keyword). -/
semantic_template at_ (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability :=
  .triggered event or_ while_ joins during limit if_ action
/-- "After <roll>, <modify>": a Roll Modifier, applied before the roll's result is final, not a
trigger [CR#706.2,706.2a,113.3c]. `limit` is a printed "only once each turn". -/
semantic_template after (roll : GameEvent) (modify : Action) (limit : Option UsageLimit := none) :
    Ability :=
  .static (.rollModifier roll modify limit)
semantic_template activated (cost : Cost) (action : Action) : Ability :=
  .activated cost action none none none none
/-- "[cost]: <action>. Activate only <timing>." -/
semantic_template activatedOnlyDuring (cost : Cost) (action : Action) (timing : Timing) :
    Ability :=
  .activated cost action timing none none none
/-- "[cost]: <action>. Only <who> may activate this ability." -/
semantic_template activatedBy (cost : Cost) (action : Action) (who : NounPhrase) : Ability :=
  .activated cost action none none none who
/-- "[cost]: <action>. Activate only once <limit>." -/
semantic_template activatedOnlyOnce (cost : Cost) (action : Action) (limit : UsageLimit) :
    Ability :=
  .activated cost action none limit none none
/-- "[cost]: <action>. Activate only if <guard>." -/
semantic_template activatedOnlyIf (cost : Cost) (action : Action) (guard : Condition) :
    Ability :=
  .activated cost action none none guard none
/-- `ability` with the restrictions a keyword or a granting effect adds to it [CR#113.10a]. -/
semantic_template qualified (ability : Ability) (timing : Option Timing := none)
    (limit : Option UsageLimit := none) (guard : Option Condition := none)
    (from_ : Option Zone := none) : Ability :=
  .qualified ability timing limit guard from_
/-- "<subject> has <selection> of <source>" (backup [CR#702.165a]). -/
semantic_template abilityGrantSelected (subject : NounPhrase) (source : NounPhrase)
    (selection : AbilitySelection) : StaticSpec :=
  .characteristicChange subject [.addedSelectionOf source selection]
/-- "[cost]: <action>. Activate only once <limit> and only if <guard>." -/
semantic_template activatedOnlyOnceIf (cost : Cost) (action : Action) (limit : UsageLimit)
    (guard : Condition) : Ability :=
  .activated cost action none limit guard none
/-- "<subject> has <ability>" -/
semantic_template abilityGrant (subject : NounPhrase) (ability : Ability) : StaticSpec :=
  .characteristicChange subject [.addedAbilities [ability]]
/-- "<subject> has <term>", for a term a "the same is true for" list names. A keyword named bare is
granted itself; a family term ("landwalk", "protection", "hexproof") grants each variant of it
that the objects `holders` describes have, "the <term> abilities of those objects" [CR#702.1c]. -/
semantic_template keywordGrant (subject : NounPhrase) (term : KeywordTerm)
    (holders : Predicate) : StaticSpec :=
  match term with
  | .the label => abilityGrant subject (keyword label)
  | _ => abilityGrantSelected subject (allOf holders) (.specified [.term term])
/-- "<subject> has all <classes> abilities of <source> [except <except>]" -/
semantic_template abilityGrantFrom (subject source : NounPhrase)
    (classes : List AbilityClass := []) (except : Option Predicate := none) : StaticSpec :=
  .characteristicChange subject [.addedAbilitiesOf source classes except]
semantic_template addSubtype (subject : NounPhrase) (subtype : Subtype) : StaticSpec :=
  .characteristicChange subject [.typeLine .adds { subtypes := [subtype] }]
semantic_template addedCost (cost : Cost) (offered : Bool) (times : PayTimes := PayTimes.once) :
    StaticSpec :=
  .addedCost cost offered times
semantic_template additionalTriggers (event : GameEvent) (quantity : Quantity) : StaticSpec :=
  .additionalTriggers event quantity
semantic_template addsTarget (subject : NounPhrase) (description : Predicate) : StaticSpec :=
  .addsTarget subject description
semantic_template allInstead (cost : Option Cost := none) (condition : Option Condition := none) :
    ModeChange :=
  .allInstead cost condition
/-- "<the ability, for the first keyword>. The same is true for <the other keywords>.": one ability
[CR#113.2c] that does, for each keyword in Oracle order, what `body` writes for it. A triggered
`body` makes one triggered ability under the first keyword's trigger, whose action runs each
keyword's action `sequentially`; a static `body` makes one static ability whose parts are each
keyword's (`conjunction`). Only the first keyword's trigger is read. Any other `body`, a mix of
triggered and static, or an empty list joins nothing: an empty `conjunction`, refused `nonEmpty`.
A keyword with variants grants each of them through `keywordGrant` [CR#702.1c]. -/
semantic_template alsoForKeywords (body : KeywordTerm → Ability)
    (keywords : List KeywordTerm := []) : Ability :=
  let parts := keywords.map body
  let actions := parts.filterMap fun
    | .triggered _ _ _ _ _ _ _ action => some action
    | _ => none
  let specs := parts.filterMap fun
    | .static spec => some spec
    | _ => none
  match parts with
  | .triggered event alternatives while_ joins timing limit intervening _ :: _ =>
    if actions.length == parts.length then
      .triggered event alternatives while_ joins timing limit intervening (.sequentially actions)
    else .static (.conjunction none [])
  | .static _ :: _ =>
    .static (.conjunction none (if specs.length == parts.length then specs else []))
  | _ => .static (.conjunction none [])
semantic_template altCost (subject : NounPhrase) (cost : Option Cost := none) : StaticSpec :=
  .altCost subject cost
semantic_template asEvent (event : GameEvent) (does : Action) : StaticSpec := .asEvent event does
/-- "If this spell's <keyword> cost was paid, <do X to> the permanent this spell becomes at the
beginning of the next end step": a Resolution Delayed Trigger, created as the permanent is put
onto the battlefield as the spell resolves [CR#608.3g]. -/
semantic_template resolutionDelayedTrigger (does : Action) : StaticSpec :=
  .resolutionDelayedTrigger does
semantic_template asThose : TokenSpec := .asThose
semantic_template characteristicChange (subject : NounPhrase)
    (edits : List CharacteristicEdit := []) : StaticSpec :=
  .characteristicChange subject edits
semantic_template compound (costs : List Cost := []) : Cost := .compound costs
semantic_template conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking) :
    StaticSpec :=
  .conditional spec condition marking
semantic_template conjunction (parts : List StaticSpec) (subject : Option NounPhrase := none) :
    StaticSpec :=
  .conjunction subject parts
semantic_template controlGrant (player subject : NounPhrase) : StaticSpec :=
  .controlGrant player subject
semantic_template copyChange (subject source : NounPhrase) (exceptions : List CopyExcept := []) :
    StaticSpec :=
  .copyChange subject source exceptions
semantic_template copyOf (source : NounPhrase) (exceptions : List CopyExcept := []) : TokenSpec :=
  .copyOf source exceptions
semantic_template costShift (subject : NounPhrase) (shift : CostShift) : StaticSpec :=
  .costShift subject shift
semantic_template extraModesCost (cost : Cost) : ModeChange := .extraModesCost cost
/-- "If <event> would …, <replacement> instead", "<subject> enters tapped", "As <subject> enters,
…": a replacement effect, `event` modified by `replacement` [CR#614.1a,614.1c,614.1d]. -/
semantic_template StaticSpec.replace (event : WouldClause) (replacement : Replacement)
    (timing : Option Timing := none) (use : ReplUse := .repeatedly)
    (limit : Option UsageLimit := none) : StaticSpec :=
  .replace event replacement timing use limit
/-- "instead": the word marking `action` as what happens in the replaced event's place
("draw two cards instead", "you may instead mill …") [CR#614.1a]. Cards write a conditional
replacement in Oracle word order, `doIf c (instead a)`. -/
semantic_template instead (action : Action) : Replacement := .instead action
/-- `action` done before the replaced event, which still happens, modified by `body` if one is
written: "As this enters, choose a color", whose choice is made before the permanent enters
[CR#614.1c,614.12a]. -/
semantic_template before (action : Action) (body : Option Replacement := none) : Replacement :=
  .before action body
/-- `action` done as part of the replaced event, which still happens ("with" it): "that player may
put … counters on it as it enters", "you may have this permanent enter with an additional +1/+1
counter on it" [CR#614.1c,702.104a,702.136a]. -/
semantic_template alongside (action : Action) : Replacement := .alongside action
/-- The replaced entering still happens, with Entry Riders `riders`: "enters tapped", "enters
with N counters", "enters as a copy", "enters transformed" [CR#614.1c,614.1d]. -/
semantic_template withRiders (riders : List MoveRider) : Replacement := .withRiders riders
/-- "If <condition>, <replacement> instead": a replacement applying only while `condition` holds,
the event happening unmodified otherwise, as "draw a card. If you control three or more
artifacts, draw two cards instead" [CR#614.1a,614.15]. A replacement that already has a
condition keeps it inside, and a caller's replacement stays its caller's. -/
semantic_template Replacement.doIf (condition : Condition) (replacement : Replacement) :
    Replacement :=
  .onlyIf condition replacement
semantic_template italicHead (word : ItalicWord) (ability : Ability) : Ability :=
  .italicHead word ability
/-- `onPermanent` on a permanent card and `onInstantOrSorcery` on an instant or sorcery card: a
keyword the rules define by Card Class [CR#110.4,110.4a]. -/
semantic_template byCardClass (onPermanent onInstantOrSorcery : Ability) : Ability :=
  .byCardClass onPermanent onInstantOrSorcery
semantic_template itsManaCost : Cost := .itsManaCost
/-! ## Play payments and ballots -/

/-- Paying the spell's or ability's own cost [CR#601.2f]. -/
semantic_template itsOwnCost : PlayPayment := .itsOwnCost
/-- "without paying its mana cost": an alternative cost [CR#118.9]. -/
semantic_template withoutPaying : PlayPayment := .withoutPaying
/-- "by paying <cost> rather than paying its mana cost": an alternative cost [CR#118.9]. -/
semantic_template payingInstead (cost : Cost) : PlayPayment := .payingInstead cost
/-- "by <cost> as an additional cost to cast it": the mana cost or an alternative cost, plus
`cost` [CR#118.8,601.2f]. -/
semantic_template plusAdditional (cost : Cost) : PlayPayment := .plusAdditional cost
/-- "using its <keyword> ability": the alternative cost that keyword ability offers
[CR#601.2b,601.2f]. -/
semantic_template usingKeyword (label : KeywordLabel) : PlayPayment := .byKeyword label
semantic_template letterDefinition (letter : Letter) (amount : Amount) : StaticSpec :=
  .letterDefinition letter amount
semantic_template loyaltySymbol (change : Delta Amount) : Cost := .loyaltySymbol change
semantic_template mana (cost : List ManaSymbol) : Cost := .mana cost
/-- A written mana cost is a cost: `[generic 1, pip .black]` for `mana [generic 1, pip .black]`. -/
instance [AuthoringContext] : Coe ManaCost Cost := ⟨mana⟩
semantic_template manaRetention (player : NounPhrase) (mana : ManaHeld) : StaticSpec :=
  .manaRetention player mana
semantic_template mayBeginOnBattlefield : Ability := .mayBeginOnBattlefield
semantic_template modeChoice (change : ModeChange) : StaticSpec := .modeChoice change
/-- "<subject> gets <delta> to one stat" -/
semantic_template modification (subject : NounPhrase) (stat : CharacteristicStat)
    (delta : Delta Amount) : StaticSpec :=
  .characteristicChange subject [statEdit stat delta]
semantic_template notChosenBefore (within : Duration) : ModeChange := .notChosenBefore within
semantic_template partSkip (player : NounPhrase) (part : TurnPart) : StaticSpec :=
  .partSkip player part
semantic_template paySubstitute (subject : NounPhrase) (substitute : ManaSubstitute) : StaticSpec :=
  .paySubstitute subject substitute
semantic_template perform (action : Action) : Cost := .perform action
/-- An action is the cost of performing it: `sacrifice thisArtifact` for
`perform (sacrifice thisArtifact)`. -/
instance [AuthoringContext] : Coe Action Cost := ⟨perform⟩
/-- "pay <amount> life": a cost payment that subtracts that much life, its payer losing that much
life [CR#119.4]; a player who can't lose life can't pay it [CR#119.8]. In a cost slot it is
coerced to `perform (payLife amount)`, one payment of that life. -/
semantic_template payLife (amount : Amount := anyAmount) : Action :=
  pay (perform (loseLife amount)) .once
/-- "<subject>'s power [and toughness] [are each] [is] equal to <amount>": a
Characteristic-Defining Ability setting each stat `slots` names [CR#604.3a,613.4a]. -/
semantic_template ptDefinition (subject : NounPhrase) (slots : DefinedSlots) (amount : Amount) :
    StaticSpec :=
  .characteristicDefining subject (match slots with
    | .powerAlone => [.stat .sets .power amount]
    | .toughnessAlone => [.stat .sets .toughness amount]
    | .bothEach => [.stat .sets .power amount, .stat .sets .toughness amount])
/-- "<subject>'s power is equal to <power> and its toughness is equal to <toughness>": one
Characteristic-Defining Ability setting both, the toughness free to read the power's number
back ("that number plus 1") [CR#604.3a,613.4a]. -/
semantic_template ptDefinitions (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  .characteristicDefining subject [.stat .sets .power power, .stat .sets .toughness toughness]
/-- "Switch <subject>'s power and toughness" [CR#613.4d] -/
semantic_template ptSwitch (subject : NounPhrase) : StaticSpec :=
  .characteristicChange subject [.ptSwitch]
/-- "<subject> is <quality>" as a Characteristic-Defining Ability: "this creature is all
colors", "this object is colorless", "this object is every creature type"
[CR#604.3a,702.73a,702.114a]. -/
semantic_template qualityDefinition (subject : NounPhrase) (op : QualityOp)
    (payload : QualityPayload) : StaticSpec :=
  .characteristicDefining subject (payload.edits op)
semantic_template repeatable : ModeChange := .repeatable
/-- "<spec>. This effect doesn't remove <subject>.": an exemption from the one state-based
consequence of `spec`, a protection grant or a "can't be enchanted" clause, that would remove
the Attachments `subject` names from its host [CR#702.16n,702.16p,303.4c,704.5m]
(`StaticSpec.retention`). -/
semantic_template retention (spec : StaticSpec) (subject : NounPhrase) : StaticSpec :=
  .retention spec subject
semantic_template scaled (cost : Cost) (amount : Amount) : Cost := .scaled cost amount
semantic_template setQuantity (quantity : Quantity) : ModeChange := .setQuantity quantity
semantic_template specialAction (when : Timing) (cost : Cost) (does : Action) : StaticSpec :=
  .specialAction none when cost does
/-- An instant or sorcery's spell ability: `body`, as it resolves [CR#113.3a]. -/
semantic_template spell (body : Action) (timing : Option Timing := none) : Ability :=
  .spell timing body
semantic_template static (spec : StaticSpec) : Ability := .static spec
semantic_template tapSymbol : Cost := .tapSymbol
semantic_template thatAbility (ref : ChoiceRef) : Ability := .thatAbility ref
semantic_template untapSymbol : Cost := .untapSymbol
semantic_template visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing) :
    StaticSpec :=
  .visibility verb player what
semantic_template whileHolds (condition : Condition) (does : Action) : StaticSpec :=
  .whileHolds condition does
semantic_template withSubstitute (cost : Cost) (substitute : ManaSubstitute) : Cost :=
  .withSubstitute cost substitute
semantic_template written (characteristics : CharacteristicBundle) : TokenSpec :=
  .written characteristics
/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
semantic_template stat (value : Nat) : Option Amount := some (.lit value)
/-- A single-faced card, written as the characteristics of its one Card Face. -/
semantic_template singleFacedCard (characteristics : Characteristics) : Card :=
  .singleFaced { characteristics }
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
semantic_template levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) :
    LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
semantic_template prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := cost, power := stat power, toughness := stat toughness }
/-! ## Conditions -/

semantic_template anyResultIs (comparator : Comparator) (bound : Amount) : Condition :=
  .anyResultIs comparator bound
semantic_template compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount) :
    Condition :=
  .compareAmt subject comparator bound
semantic_template Condition.duringPart (part : TurnPart) (whose : Option NounPhrase := none) :
    Condition :=
  .duringPart part whose
/-- "<condition> continuously since <since>": `condition` has held from `since` until now. -/
semantic_template heldSince (condition : Condition) (since : GameEvent) : Condition :=
  .heldSince condition since
/-- "<subject> has been under its controller's control continuously since their most recent turn
began" [CR#302.6]: the summoning-sickness reading, its controller the present one. -/
semantic_template controlledSinceTheirTurnBegan (subject : NounPhrase) : Condition :=
  heldSince (.matches subject (controlledBy (controllerOf it)))
    (.beginningOf .turn none (.mostRecent they))
/-- "if it's <designation>": the game has it [CR#731.1]. -/
semantic_template gameIs (designation : DesignationLabel) : Condition := .gameHas designation
semantic_template keywordCostPaid (label : KeywordLabel) (subject : NounPhrase) : Condition :=
  costWasPaid (.byKeyword label) none subject
/-- "<subject> is <predicate>" ("if it's a creature", "if this permanent isn't renowned" under
`not`): the condition that `subject` matches `predicate` (`matches` is Lean's word). -/
semantic_template is (subject : NounPhrase) (predicate : Predicate) : Condition :=
  .matches subject predicate
semantic_template rolledDoubles : Condition := .rolledDoubles
semantic_template voteLead (label : String) (orTied : Bool) : Condition := .voteLead label orTied
/- The verb `target` (`Action`) shares its name with the determiner `target` (`NounPhrase`), and
the verb `become` (a `Status`) with the characteristic change `become`; the argument types pick
the overload. -/
export Semantics.Templates.Action (target)
export Semantics.Templates.Status (become)
/-! ## Conferrals -/

semantic_template stateBased (when : Condition) (then_ : Action) : Conferral :=
  .stateBased when then_
/-! ## Pips -/

semantic_template PipPaid.generic : PipPaid := .generic
semantic_template ofColor (color : ColorTerm) : PipPaid := .ofColor color

/-! ## Moves and occurrences -/

/-- "<subject> moves from <from_> [to <to>]", observed on its `at_` side: the general zone change
the worded moves (`die`, `enter`, `leave`, `putInto`, …) specialize [CR#400.7]. -/
semantic_template move (subject : NounPhrase) (from_ : Location) (to : Option Location)
    (at_ : ObservationPoint) (riders : List MoveRider := []) :
    Action :=
  .move subject from_ to at_ riders
/-- "<pile>": one of the piles a separation made, as a place cards go. -/
semantic_template thePile (pile : NounPhrase) : Location := .pile pile
/-- "each" occurrence [of <player>'s]: each turn, each upkeep. -/
semantic_template eachTime (player : Option NounPhrase := none) : TurnRef := .each player
/-- "the next" occurrence [of <player>'s] -/
semantic_template nextTime (player : Option NounPhrase := none) : TurnRef := .next player
/-- "that turn's" occurrence -/
semantic_template thatTime : TurnRef := .that
/-- "this turn", "this game": the span a historical clause looks back over. -/
semantic_template within (d : Duration) (timing : Option Timing := none) : LookbackSpan :=
  .duration d timing
/-- "… this way": the event the enclosing action caused. -/
semantic_template thisWay : LookbackSpan := .thisWay
/-- The event that triggered the enclosing ability [CR#603.2]. -/
semantic_template triggering : LookbackSpan := .triggering
/-- The described object's own current existence: "spells you cast", "if you cast it from your
hand" read its own casting [CR#400.7]. -/
semantic_template thisObject : LookbackSpan := .thisObject
/-- A bare duration is the span over it: `did you attack soFarThisTurn`. -/
instance [AuthoringContext] : Coe Duration LookbackSpan := ⟨(within · none)⟩
/-- A bare duration is a written span over it, where the span is optional. -/
instance [AuthoringContext] : Coe Duration (Option LookbackSpan) := ⟨fun d => some (within d none)⟩
/-- "<subj> <action>ed [<within>]" as a description of the entity `which` stands for
("creatures that attacked this turn"); with no span written, the attributive participle a
Participial Description reads ("the creature you sacrificed"). An `ordinal` picks out the
`ordinal`th matching event ("the second spell you cast each turn"). -/
semantic_template Predicate.did (subj : Subject) (action : Action)
    (within : Option LookbackSpan := none) (ordinal : Option (Ordinal × Option Period) := none) :
    Predicate :=
  .did { subj, action, within, ordinal }
/-- "if <subj> <action>ed <within>": a condition always reads a written span. -/
semantic_template Condition.did (subj : Subject) (action : Action) (within : LookbackSpan) :
    Condition :=
  .did { subj, action, within }
/-- "<action>ed [<within>]", the passive with no agent: "the exiled card", "each creature
destroyed this way", "creatures that were dealt damage this turn". -/
semantic_template Predicate.was (action : Action) (within : Option LookbackSpan := none) :
    Predicate :=
  .did { subj := .any, action, within }
/-- "if <action> was done <within>", the passive with no agent. -/
semantic_template Condition.was (action : Action) (within : LookbackSpan) : Condition :=
  .did { subj := .any, action, within }
/-- "if <cause> would cause <clause>" -/
semantic_template causedBy (cause : Causing) (clause : WouldClause) : WouldClause :=
  .causedBy cause clause
/-- "<source> causes …": a spell, ability or other object as the cause. -/
semantic_template bySource (source : NounPhrase) : Causing := .source source
/-- "<event> causes …" -/
semantic_template byEvent (event : GameEvent) : Causing := .event event
/-- "an effect causes …", "if an effect would …": a resolving spell's or ability's effect, or a
replacement or prevention effect [CR#609.1,614.16]. Whether an Action is an effect follows from the
node that encloses it: a turn-based action is none [CR#703.1]. -/
semantic_template byAnEffect : Causing := .anEffect

/-! ## Counter kinds -/

/-- A counter kind printed on the card: "+1/+1 counter". -/
semantic_template printedKind (kind : CounterKind) : CounterKindSource := .printed kind
/-- "your choice of a <kind> or <kind> counter" -/
semantic_template chosenKind (menu : List CounterKind) : CounterKindSource := .chosen menu
/-- "N different counters from among <menu>" -/
semantic_template distinctChosenKinds (menu : List CounterKind) : CounterKindSource :=
  .distinctChosen menu
/-- "a counter of the chosen kind" -/
semantic_template thatKind : CounterKindSource := .bound
/-- "those counters": the counters just put. -/
semantic_template thoseCounters : CounterKindSource := .those
/-- "a counter of each kind on <source>" -/
semantic_template sameKindAs (source : NounPhrase) : CounterKindSource := .sameAs source
/-- A printed counter kind reads as its counter source. -/
instance [AuthoringContext] : Coe CounterKind CounterKindSource := ⟨printedKind⟩
/-- "<kind> counters on it" as an entering copy has them -/
semantic_template withEntryCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark) :
    CopyExcept :=
  .entersWithCounters amount kind mark

/-! ## Entering and copying -/

/-- "enters <status>", as "enters tapped" -/
semantic_template entersAs (status : Status) : MoveRider := .entersAs status
/-- "enters attacking [<defender>]" -/
semantic_template entersAttacking (defender : Option NounPhrase) : MoveRider :=
  .entersAttacking defender
/-- "attached to <host>", as an Aura put onto the battlefield enters [CR#303.4i] -/
semantic_template entersAttachedTo (host : NounPhrase) : MoveRider := .attachedTo host
/-- "with N <kind> counters on it" -/
semantic_template withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark) :
    MoveRider :=
  .withCounters amount kind mark
/-- "under <controller>'s control" -/
semantic_template under (controller : NounPhrase) : MoveRider := .under controller
/-- "[you may have it enter] as a copy of <source> [except …]" -/
semantic_template asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept) :
    MoveRider :=
  .asCopyOf optional source exceptions
/-- "face down", as `spec` lists its characteristics [CR#708.2,406.3a]. -/
semantic_template faceDownAs (spec : FaceDown) : MoveRider := .faceDown spec
/-- "in any order", "in a random order" [CR#401.4] -/
semantic_template inOrder (arrangement : Arrangement) : MoveRider := .inOrder arrangement
/-- "a token that's …", entering with `riders` -/
semantic_template CreationSpec.token (spec : TokenSpec) (riders : List MoveRider) : CreationSpec :=
  .token spec riders
/-- "with every <space> type" -/
semantic_template withEveryType (space : SubtypeSpace) : WrittenQuality := .everyTypeOf space
/-- "of <quality>", as "of the chosen color" -/
semantic_template withQuality (quality : Predicate) : WrittenQuality := .chosen quality
/-- "except it's <types> in addition to its other types" -/
semantic_template isAlso (types : List CardType) (subtypes : List Subtype) : CopyExcept :=
  .edits [.typeLine .adds (({ types := types, subtypes := subtypes } :
    Characteristics).typeChanges)]
/-- "except its name is <name>" -/
semantic_template isNamed (name : String) : CopyExcept := .edits [.name .sets name]
/-- "except it's <power>/<toughness>" -/
semantic_template hasPT (power toughness : Amount) : CopyExcept :=
  .edits [.stat .sets .power power, .stat .sets .toughness toughness]
/-- "except it isn't legendary" -/
semantic_template isntLegendary : CopyExcept :=
  .edits [.typeLine .loses { supertypes := some [.legendary] }]
/-- "except it's <color>" -/
semantic_template isColor (color : Color) : CopyExcept := .edits [.colors .sets (.some [color])]
/-- "except it has <ability>" -/
semantic_template alsoHas (ability : Ability) : CopyExcept := .ability ability
/-- "except it doesn't have this ability" -/
semantic_template exceptThisAbility : CopyExcept := .thisAbility
/-- "copy <subject> [<times> times] [except …]", a spell or a card [CR#707.10,707.12]. -/
semantic_template Action.copy (sort : CopySort) (subject : NounPhrase) (times : Amount)
    (exceptions : List CopyExcept) : Action :=
  .copy sort subject times exceptions

/-! ## Characteristics and abilities lost -/

/-- The characteristics a quality change adds or sets; "It's still a <retained>" keeps every
prior type [CR#205.1b]. -/
semantic_template bundle (characteristics : CharacteristicBundle) (retained : Option CardType := none) :
    QualityPayload :=
  .bundle characteristics retained
/-- "every <space> type" -/
semantic_template everyTypeOf (space : SubtypeSpace) : QualityPayload := .quality (.everyTypeOf space)
/-- "the chosen <quality>" -/
semantic_template chosenQuality (quality : Predicate) : QualityPayload := .quality (.chosen quality)
/-- "<colors>", as "becomes blue" -/
semantic_template colored (colors : ColorSpec) : QualityPayload := .colored colors
/-- "<ability>", lost as written -/
semantic_template AbilityLost.written (ability : Ability) : AbilityLost := .written ability
/-- "all <keyword> abilities", as "all protection abilities" -/
semantic_template abilities (keyword : KeywordTerm) : AbilityLost := .term keyword
/-- "a card named <name>" -/
semantic_template printedName (name : String) : NameSource := .printed name
/-- "the chosen name" -/
semantic_template chosenName : NameSource := .chosen
/-- A printed name reads as its name source: `named "Rune Snag"`. -/
instance [AuthoringContext] : Coe String NameSource := ⟨printedName⟩
/-- "<color>" as a color term, as "devotion to black" -/
semantic_template color (color : Color) : ColorTerm := .lit color
/-- "mana of <color>" -/
semantic_template ManaTypeTerm.ofColor (color : ColorTerm) : ManaTypeTerm := .ofColor color
/-- "<cost>" written out as mana symbols -/
semantic_template symbols (cost : ManaCost) : ManaCostTerm := .lit cost

/-! ## Mana production and spending -/

/-- "Add <runs>": one of the listed runs of mana symbols. -/
semantic_template manaOf (runs : List ProducedRun) : ProducedMana := .runs runs
/-- "mana of any color", "any one color", "any combination of colors" -/
semantic_template ofAnyColor (freedom : ColorFreedom) : ProducedMana := .anyColor freedom
/-- "mana of the chosen color" [or <alternative>] -/
semantic_template ofTheChosenColor (alternative : Option ProducedRun := none) : ProducedMana :=
  .ofChosenColor alternative
/-- "mana equal to <subject>'s mana cost" -/
semantic_template matchingManaCostOf (subject : NounPhrase) : ProducedMana := .asPrintedCost subject
/-- "one mana of any type <subject> produced" -/
semantic_template ofTypeProducedBy (subject : NounPhrase) : ProducedMana := .producedByEvent subject
/-- "one mana of any type <subject> could produce" -/
semantic_template ofTypeProducibleBy (subject : NounPhrase) : ProducedMana := .couldProduce subject
/-- "one mana of any of <subject>'s colors" -/
semantic_template ofAnyColorOf (subject : NounPhrase) : ProducedMana := .amongColorsOf subject
/-- "one mana of any of <colors>" -/
semantic_template ofAnyColorAmong (colors : List Color) : ProducedMana := .amongWritten colors
/-- "mana of the last noted type" -/
semantic_template ofTheLastNotedType (subject : NounPhrase) : ProducedMana := .lastNoted subject
/-- "Spend this mana only to <purposes>." -/
semantic_template spendOnly (purposes : List SpendPurpose) : ManaRider := .spendOnly purposes
/-- "You can't spend this mana to <purposes>." -/
semantic_template cantSpendOn (purposes : List SpendPurpose) : ManaRider := .spendNotOn purposes
/-- "When that mana is spent to cast <spell>, <says>"; "that spell <says>" (`mode`). -/
semantic_template whenSpentOn (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Action) :
    ManaRider :=
  .onSpent mode only spell says
/-- "to cast <predicate> spells" -/
semantic_template toCast (predicate : Predicate) : SpendPurpose := .toCast predicate
/-- "to activate abilities [of <source>]" -/
semantic_template toActivate (source : Option Predicate) : SpendPurpose := .toActivate source
/-- "to pay <cost>" -/
semantic_template toPay (cost : CostNamed) : SpendPurpose := .toPay cost
/-- "to turn <subject> face up" [CR#116.2b] -/
semantic_template toTurnFaceUp (subject : Predicate) : SpendPurpose := .toTurnFaceUp subject
/-- "this mana" -/
semantic_template thisMana : ManaHeld := .thisMana
/-- "unspent [<type>] mana" -/
semantic_template unspent (type : Option ColorOrColorless) : ManaHeld := .unspent type
/-- "spend mana as though it were mana of <as_> [to <purpose>]" [CR#609.4b] -/
semantic_template spendManaAs (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : AsThough :=
  .mana what as_ purpose
/-- "as though its <stat> were <amount> greater" -/
semantic_template greaterBy (stat : Stat) (amount : Amount) : AsThough := .greater stat amount
/-- "as though <cond>": the rule applies as though the condition held. -/
semantic_template holds (cond : Condition) : AsThough := .condition cond
/-- A condition reads as the "as though" it states. -/
instance [AuthoringContext] : Coe Condition AsThough := ⟨holds⟩

/-! ## Prevention and redirection -/

/-- "prevent <cut> [of <damage>]. [<also>]": prevent `cut` of the damage `damage` names, or of
"that damage", the damage in scope, then do `also`, the prevention's additional effect
[CR#615.1,615.5]. Under `cant`, "<damage> can't be prevented" [CR#615.12]. -/
semantic_template preventDamage (cut : PreventCut) (damage : Option WouldClause := none)
    (also : Option Action := none) : Action :=
  .preventDamage cut damage also
/-- "<cut> [of <damage>] is dealt to <to> instead": a redirection effect [CR#614.9]. Under
`cant`, "<damage> can't be dealt instead to <to>". -/
semantic_template redirectDamage (cut : PreventCut) (to : NounPhrase)
    (damage : Option WouldClause := none) : Action :=
  .redirectDamage cut to damage
/-- "all of that damage" -/
semantic_template allDamage : PreventCut := .all
/-- "<amount> of that damage" -/
semantic_template ofThatDamage (amount : Amount) : PreventCut := .some amount
/-- "the next <amount> damage" [CR#615.7] -/
semantic_template theNext (amount : Amount) : PreventCut := .shield amount
/-- "all but <amount> of that damage" -/
semantic_template PreventCut.allBut (amount : Amount) : PreventCut := .allBut amount
/-- "half that damage, rounded <rounding>" -/
semantic_template PreventCut.half (rounding : RoundMode) : PreventCut := .half rounding

/-! ## Cost shifts -/

/-- "costs <amount> less[. This effect can't reduce the mana in that cost to less than <floor>
mana]" -/
semantic_template less (amount : Amount) (floor : Option Amount := none) : CostShift :=
  .generic (.down amount) floor
/-- "costs <amount> more" -/
semantic_template more (amount : Amount) : CostShift := .generic (.up amount) none
/-- "costs <cost> less", symbol by symbol [CR#702.48c,118.7] -/
semantic_template lessBySymbols (cost : ManaCostTerm) (coloredOnly : Bool := false) : CostShift :=
  .bySymbols (.down cost) coloredOnly
/-- "costs <cost> more", symbol by symbol [CR#702.48c,118.7] -/
semantic_template moreBySymbols (cost : ManaCostTerm) : CostShift := .bySymbols (.up cost) false

/-! ## Visibility, exposure and exchanges -/

/-- "the top card of your library" as a player may look at it -/
semantic_template topOfLibrary : VisibleThing := .topOfLibrary
/-- "their hand" as a player plays with it revealed -/
semantic_template wholeHand : VisibleThing := .wholeHand
/-- "<objects>" a player may look at -/
semantic_template objects (objects : NounPhrase) : VisibleThing := .objects objects
/-- "<cards>", revealed or looked at -/
semantic_template cards (cards : NounPhrase) : Exposed := .cards cards
/-- "the chosen <sort>", revealed -/
semantic_template choiceOf (sort : ChoiceSort) : Exposed := .choice sort
/-- "exchange life totals [with <parties>]" [CR#701.12c] -/
semantic_template lifeTotals (parties : NounPhrase) : Exchanged := .lifeTotals parties
/-- "exchange control of <left> and <right>" [CR#701.12b] -/
semantic_template controlOf (left right : NounPhrase) : Exchanged := .controlOf left right
/-- "exchange <left> with <right>" across zones [CR#701.12a] -/
semantic_template cardsAcross (left right : NounPhrase) : Exchanged := .cardsAcross left right
/-- "exchange <left> and <right>": the contents of two zones [CR#701.12a] -/
semantic_template zones (left right : Location) : Exchanged := .zones left right
/-- "exchange <left> and <right>": two values [CR#701.12g] -/
semantic_template values (left right : Amount) : Exchanged := .values left right
/-- "exchange <exchanged>" [CR#701.12] -/
semantic_template exchange (exchanged : Exchanged) : Action := .exchange exchanged
/-- "this door" [CR#709.5j] -/
semantic_template thisDoor : Door := .thisDoor
/-- "a [<state>] door of <room>" [CR#709.5j] -/
semantic_template doorOf (state : Option LockState) (room : NounPhrase) : Door := .doorOf state room
/-- "fully unlock <room>": both its halves unlocked [CR#709.5i]. -/
semantic_template fullyUnlock (room : NounPhrase) : Action := .fullyUnlock room

/-! ## Choices -/

/-- "a card name [of <predicate>]" -/
semantic_template nameOfCard (predicate : Predicate) : ChoiceDomain := .nameOfCard predicate
/-- "a color other than <color>" -/
semantic_template colorOtherThan (color : Color) : ChoiceDomain := .colorOtherThan color
/-- "a creature type other than <subtype>" -/
semantic_template typeOtherThan (subtype : Subtype) : ChoiceDomain := .typeOtherThan subtype
/-- "a basic land type" -/
semantic_template basicTypesOnly : ChoiceDomain := .basicTypesOnly
/-- "a number <quantity>" -/
semantic_template number (quantity : Quantity) : ChoiceDomain := .number quantity
/-- "a <predicate> player" -/
semantic_template players (predicate : Predicate) : ChoiceDomain := .players predicate
/-- "<keyword> or <keyword>", as an ability chosen among named keywords -/
semantic_template abilitiesAmong (keywords : List KeywordTerm) : ChoiceDomain :=
  .abilitiesAmong keywords
/-- "choose <chosen> starting with <first>" -/
semantic_template chooseStartingWith (first chosen : NounPhrase) (disclosure : Disclosure := .openly)
    (when : Option Timing := none) : Action :=
  .choose first chosen disclosure when
/-- "for <options>", as a vote's choices -/
semantic_template byLabel (options : List VoteLabel) : Ballot := .byLabel options
/-- "for <candidates>", as a vote's candidates -/
semantic_template byCandidate (candidates : NounPhrase) : Ballot := .byCandidate candidates

/-! ## Coins, dice and repetition -/

/-- "flip a coin for each <each>" -/
semantic_template flipACoinForEach (each : NounPhrase) (call : Option FlipCall := none)
    (outcome : Option CoinFace := none) : Action :=
  .flipCoins (.per each) call outcome
/-- "a result in <quantity>" -/
semantic_template resultIn (quantity : Quantity) : RollWatch := .resultIn quantity
/-- "the highest natural result" -/
semantic_template highestNatural : RollWatch := .highestNatural
/-- "ignore the <extreme> roll" -/
semantic_template extreme (extreme : RollExtreme) : IgnoredOutcomes := .extreme extreme
/-- "ignore all but the <extreme> roll" -/
semantic_template IgnoredOutcomes.allBut (extreme : RollExtreme) : IgnoredOutcomes := .allBut extreme
/-- "ignore <amount> of the results of [<chooser>'s] choice" -/
semantic_template chosenResults (chooser : Option NounPhrase) (amount : Amount) : IgnoredOutcomes :=
  .chosen chooser amount
/-- A results table: one row per result range [CR#706.3a] -/
semantic_template applyResultsTable (rows : List RollRow) : Action := .applyResultsTable rows
/-- "again", "repeat this process" -/
semantic_template again : Repetition := .again
/-- "<times> more times" -/
semantic_template moreTimes (times : Amount) : Repetition := .moreTimes times
/-- "any number of times" -/
semantic_template anyNumberOfTimes : Repetition := .anyNumber
/-- "again, excluding the chosen" -/
semantic_template againExcludingChosen : Repetition := .againExcludingChosen

/-! ## Deck conditions -/

/-- "has <predicate>" as a starting-deck trait -/
semantic_template aCharacteristic (predicate : Predicate) : DeckTrait := .aCharacteristic predicate
/-- "an <parity> mana value" [CR#202.3] -/
semantic_template manaValueParity (parity : Parity) : DeckTrait := .manaValueParity parity
/-- "more than one of the same mana symbol in its mana cost" -/
semantic_template repeatedManaSymbol : DeckTrait := .repeatedManaSymbol
/-- "an <class_> ability" -/
semantic_template hasAbilityOf (class_ : AbilityClass) : DeckTrait := .hasAbilityOf class_
/-- "<trait> or <trait>" -/
semantic_template anyOf (traits : List DeckTrait) : DeckTrait := .anyOf traits
/-- "Each <scope> in your starting deck <trait>." [CR#702.139a] -/
semantic_template everyCardIs (scope : Predicate) (trait : DeckTrait) : DeckCondition :=
  .everyCardIs scope trait
/-- "No <scope> in your starting deck <trait>." [CR#702.139a] -/
semantic_template noCardIs (scope : Predicate) (trait : DeckTrait) : DeckCondition :=
  .noCardIs scope trait
/-- "Each <group> shares a card type", "… has a different name" [CR#702.139a] -/
semantic_template groupAgrees (group : NounPhrase) : DeckCondition := .groupAgrees group
/-- "at least <extra> cards more than the minimum deck size" -/
semantic_template deckSizeOverMinimum (extra : Nat) : DeckCondition := .deckSizeOverMinimum extra

/-! ## Deontic verbs over a vehicle or a mount -/

/-- "<subject> crews <vehicle>" [CR#702.122a] -/
semantic_template Action.crew (vehicle : NounPhrase) : Action := .crew vehicle
/-- "<subject> saddles <permanent>" [CR#702.171a] -/
semantic_template Action.saddle (permanent : NounPhrase) : Action := .saddle permanent
/-- "<ability> triggers": the act a trigger prohibition names. -/
semantic_template trigger : Action := .trigger

end Semantics.Templates
