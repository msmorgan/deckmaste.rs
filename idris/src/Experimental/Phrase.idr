||| The phrase layer: zone/name/predicate/noun/amount/quantity/condition
||| descriptions and the referring expressions built from them.
module Experimental.Phrase

import public Experimental.Words
import public Experimental.Events

%default total

mutual
  public export
  data ZoneScope : Bindings -> Zone -> Type where
    Bare : ZoneScope bs z
    OwnedBy : (n : Noun bs Player) -> {auto 0 ps : Possessable z} ->
              ZoneScope bs z

  ||| Where in a library a card lands: one end of the ordered pile
  ||| [CR#401.2], the top-or-bottom disjunction, or no position at all.
  ||| The chooser is a
  ||| separable slot on the
  ||| disjunction alone — Write into Being writes the bare coordination.
  public export
  data LibPlace : Bindings -> Type where
    OneEnd : (pos : LibPos) -> LibPlace bs
    EitherEnd : (chooser : Maybe (Noun bs Player)) ->
                {auto 0 ag : EventAgent chooser} -> LibPlace bs
    ||| "shuffle [what] into [a] library": the destination that names no
    ||| position, because the act that puts the card there randomizes
    ||| the pile around it [CR#701.24a,701.24c]. A PLACE and not an
    ||| `Arrangement`: an arrangement says in what order several cards
    ||| enter one position [CR#401.4], and this one gives them none to
    ||| enter.
    Shuffled : LibPlace bs

  ||| One card goes to one end, so a disjunction over the two ends states
  ||| no order [CR#401.4]. A shuffle states none from the other side of
  ||| the same rule: [CR#401.4] arranges cards put "in a specific
  ||| position", and a shuffle puts them in no position at all.
  public export
  placeArrangementOk : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Bool
  placeArrangementOk (OneEnd _) _ = True
  placeArrangementOk (EitherEnd _) Nothing = True
  placeArrangementOk (EitherEnd _) (Just _) = False
  placeArrangementOk Shuffled Nothing = True
  placeArrangementOk Shuffled (Just _) = False

  public export
  PlaceArrangementFits : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Type
  PlaceArrangementFits pl ord = So (placeArrangementOk pl ord)

  ||| The offset counts down from the top card [CR#401.7], so it needs a
  ||| position to count from; a shuffle names one nowhere.
  public export
  placeOrdinalOk : {0 bs : Bindings} -> LibPlace bs -> Maybe LibOrdinal -> Bool
  placeOrdinalOk Shuffled (Just _) = False
  placeOrdinalOk Shuffled Nothing = True
  placeOrdinalOk (OneEnd _) _ = True
  placeOrdinalOk (EitherEnd _) _ = True

  public export
  PlaceOrdinalFits : {0 bs : Bindings} -> LibPlace bs -> Maybe LibOrdinal -> Type
  PlaceOrdinalFits pl off = So (placeOrdinalOk pl off)

  public export
  data ZoneExpr : Bindings -> Type where
    ZoneAt : (z : Zone) -> ZoneScope bs z -> ZoneExpr bs
    LibraryAt : (place : LibPlace bs) -> (ord : Maybe Arrangement) ->
                (off : Maybe LibOrdinal) ->
                {auto 0 af : PlaceArrangementFits place ord} ->
                {auto 0 nf : PlaceOrdinalFits place off} ->
                ZoneScope bs Library -> ZoneExpr bs

  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort (ZoneAt z _) = z
  zoneSort (LibraryAt _ _ _ _) = Library

  public export
  zoneArrangement : ZoneExpr bs -> Maybe Arrangement
  zoneArrangement (ZoneAt _ _) = Nothing
  zoneArrangement (LibraryAt _ ord _ _) = ord

  public export
  zoneOrdinal : ZoneExpr bs -> Maybe LibOrdinal
  zoneOrdinal (ZoneAt _ _) = Nothing
  zoneOrdinal (LibraryAt _ _ off _) = off

  ||| Whether the destination's own act randomizes the library
  ||| [CR#701.24a]. It is what makes a shuffle-into leave the discourse
  ||| the way a bare `Shuffle` does.
  public export
  placeShuffles : {0 bs : Bindings} -> LibPlace bs -> Bool
  placeShuffles Shuffled = True
  placeShuffles (OneEnd _) = False
  placeShuffles (EitherEnd _) = False

  public export
  zoneShuffles : ZoneExpr bs -> Bool
  zoneShuffles (ZoneAt _ _) = False
  zoneShuffles (LibraryAt place _ _ _) = placeShuffles place

  ||| What a move leaves the discourse holding once its destination has
  ||| had its say. [CR#701.24c] shuffles the library whatever becomes of
  ||| the objects named, so after a shuffle-into no library mention is
  ||| readable -- the moved card's own included, since [CR#701.24a]
  ||| leaves no player knowing where it went.
  public export
  afterMoveTo : {0 bs : Bindings} -> ZoneExpr bs -> Bindings -> Bindings
  afterMoveTo to out = if zoneShuffles to then afterShuffle out else out

  public export
  data NameSource : Bindings -> Type where
    PrintedName : (name : String) -> NameSource bs
    ChosenName : {auto 0 ok : countChoice (QSort CardName) bs = 1} -> NameSource bs
    ||| "… with the same name as that creature", "… as those creatures".
    ||| The relatum is ungated in number: [CR#201.2c] states the comparison
    ||| against "a second object or group of objects" outright, and
    ||| [CR#201.2a] states the positive relation over two or more objects,
    ||| so a plural relatum is a name held in common with them and not a
    ||| category error.
    SameNameAs : (n : Noun bs Object) -> NameSource bs

  public export
  Eq (NameSource bs) where
    (==) (PrintedName a) (PrintedName b) = a == b
    (==) (PrintedName _) _ = False
    (==) ChosenName ChosenName = True
    (==) ChosenName _ = False
    (==) (SameNameAs a) (SameNameAs b) = nounEqRef a b
    (==) (SameNameAs _) _ = False

  public export
  nameSrcDelta : {bs : Bindings} -> NameSource bs -> List Binding
  nameSrcDelta (PrintedName _) = []
  nameSrcDelta ChosenName = []
  nameSrcDelta (SameNameAs n) = nounDelta n

  ||| Indexed by sort so each domain narrows its own way — a name by a
  ||| card description [CR#201.4a], a color/creature-type by exclusion, a
  ||| number by a floor — and the index refuses every crossing for free.
  public export
  data ChoiceDomain : ChoiceSort -> Type where
    NameOfCard : (p : Predicate [] Object) -> ChoiceDomain (QSort CardName)
    ColorOtherThan : (c : Chroma.Color) -> ChoiceDomain (QSort Color)
    TypeOtherThan : (s : Subtype) ->
                    {auto 0 ct : subtypeType s = Creature} ->
                    ChoiceDomain (QSort (SubtypeQ Creature))
    ||| "choose a basic land type", "choose a nonbasic land type": the
    ||| land type's own split, and the index fixes the host because
    ||| [CR#305.6] gives the split to that host alone -- the same ground
    ||| `subtypeScopeOk` states for `SubtypeAxis`. An unnarrowed "choose
    ||| a land type" writes no domain at all.
    BasicTypesOnly : ChoiceDomain (QSort (SubtypeQ Land))
    NonbasicTypesOnly : ChoiceDomain (QSort (SubtypeQ Land))
    NumberAbove : (n : Nat) -> ChoiceDomain (QSort Number)
    ||| "choose an opponent", where a bare "choose a player" writes no
    ||| domain. [CR#102.1] makes every person in the game a player and
    ||| [CR#102.2] names a player's opponent among them, so the narrowing
    ||| is the rules' own and not a second sort; the `Opponent` head noun
    ||| beside it defers the team reading [CR#102.3] and so does this. 29
    ||| supported cards write a chosen-player read; 18 of them narrow the
    ||| chooser to an opponent and 7 leave it at "a player".
    OpponentsOnly : ChoiceDomain PlayerC
    ||| "choose a number between [lo] and [hi]": the number sort's other
    ||| narrowing, a printed RANGE where `NumberAbove` writes a floor
    ||| alone. 7 supported lines. The bounds are gated because
    ||| [CR#608.2d] forbids choosing "an option that's illegal or
    ||| impossible" and an empty range leaves no option at all; the
    ||| inclusive reading is English's own and no rule narrows it.
    NumberBetween : (lo : Nat) -> (hi : Nat) ->
                    {auto 0 ok : So (lo <= hi)} -> ChoiceDomain (QSort Number)

  ||| Not an `Eq` instance: this equality calls `predEq`, which calls back,
  ||| and an implementation is opaque to the size-change checker, so the
  ||| mutual block loses totality.
  ||| The indices are FREE of each other: a read at `Object` carries its
  ||| sort in a slot rather than in its kind, so two domains reaching this
  ||| comparison need not be at one sort, and a crossing pair answers
  ||| False here exactly as the index refuses it where the sorts do match.
  public export
  sameChoiceDomain : {0 a, b : ChoiceSort} ->
                     ChoiceDomain a -> ChoiceDomain b -> Bool
  sameChoiceDomain (NameOfCard a) (NameOfCard b) = predEq a b
  sameChoiceDomain (ColorOtherThan a) (ColorOtherThan b) = a == b
  sameChoiceDomain (TypeOtherThan a) (TypeOtherThan b) = a == b
  sameChoiceDomain BasicTypesOnly BasicTypesOnly = True
  sameChoiceDomain NonbasicTypesOnly NonbasicTypesOnly = True
  sameChoiceDomain (NumberAbove a) (NumberAbove b) = a == b
  sameChoiceDomain (NumberBetween a b) (NumberBetween c d) = a == c && b == d
  sameChoiceDomain OpponentsOnly OpponentsOnly = True
  sameChoiceDomain _ _ = False

  public export
  sameDomainOpt : {0 a, b : ChoiceSort} ->
                  Maybe (ChoiceDomain a) -> Maybe (ChoiceDomain b) -> Bool
  sameDomainOpt Nothing Nothing = True
  sameDomainOpt (Just a) (Just b) = sameChoiceDomain a b
  sameDomainOpt _ _ = False

  ||| Where an event's object came FROM, as the clause names it. ONE
  ||| shape for both seats -- the prospective `from` on
  ||| `PutInto`/`Leaves` and the retrospective `FromZones` payload --
  ||| because English writes the same three phrases at each: the zones
  ||| named ("from your graveyard", "from your hand or library"), every
  ||| zone at once ("from anywhere"), and every zone but those named
  ||| ("from anywhere other than the battlefield"). It lives here, beside
  ||| `ZoneExpr`, because the retrospective seat is declared here and the
  ||| prospective one imports it.
  ||| Coordination is a plain list at the zone sort and no marked union
  ||| row -- "from your hand or library" is two zones. The exclusion takes
  ||| a list for the same reason and not because a printing coordinates
  ||| one: [CR#400.1] lists the zones a game has, so naming the ones a
  ||| clause does NOT admit picks out the rest of that list exactly,
  ||| whether one zone is named or several. Every supported printing
  ||| excludes one.
  ||| -- spelling: "from [zs]", the zones joined by "or"; "from
  ||| anywhere"; "from anywhere other than [zs]".
  public export
  data EventSource : Bindings -> Type where
    FromAnywhere : EventSource bs
    FromZone : (zs : List (ZoneExpr bs)) -> EventSource bs
    FromAnywhereBut : (zs : List (ZoneExpr bs)) -> EventSource bs

  ||| The one zone a source phrase names, where it names exactly one.
  ||| "Anywhere", an exclusion and a coordination each name a SET of
  ||| zones, so none of them fixes a zone for a subject's own zone to
  ||| agree with.
  public export
  sourceZone : {0 bs : Bindings} -> Maybe (EventSource bs) -> Maybe Zone
  sourceZone Nothing = Nothing
  sourceZone (Just FromAnywhere) = Nothing
  sourceZone (Just (FromZone [z])) = Just (zoneSort z)
  sourceZone (Just (FromZone _)) = Nothing
  sourceZone (Just (FromAnywhereBut _)) = Nothing

  public export
  sourceDelta : {bs : Bindings} -> EventSource bs -> List Binding
  sourceDelta FromAnywhere = []
  sourceDelta (FromZone zs) = zonesDelta zs
  sourceDelta (FromAnywhereBut zs) = zonesDelta zs

  ||| Every zone a source phrase names must be one the event's clause may
  ||| name as its origin, and a phrase that names none is no phrase --
  ||| whether it names them to admit them or to exclude them. "From
  ||| anywhere" names no zone at all, so it asks instead whether the event
  ||| writes an origin.
  public export
  lookbackSourceOk : {0 bs : Bindings} -> EventName -> EventSource bs -> Bool
  lookbackSourceOk ev FromAnywhere = eventNamesOrigin ev
  lookbackSourceOk ev (FromZone zs) = lookbackZonesOk ev zs
  lookbackSourceOk ev (FromAnywhereBut zs) = lookbackZonesOk ev zs

  public export
  data EventComplement : Bindings -> EventName -> Kind -> Type where
    Involving : {kc : Kind} -> (what : Noun bs kc) ->
                {auto 0 cp : LookbackComplement ev ks kc} ->
                EventComplement bs ev ks
    ||| The complement's SECOND payload sort -- a zone where `Involving`
    ||| carries a participant. It names where the event's object came
    ||| from: [CR#601.2a] moves a cast card out of the zone it was in, and
    ||| [CR#903.8] is the family that reads that origin back, "for each
    ||| previous time the player casting it has cast it from the command
    ||| zone that game". The participant it wraps is optional because
    ||| English writes the two in one complement ("you've cast your
    ||| commander from the command zone") and writes the zone alone where
    ||| no participant is named ("if you haven't cast a spell from your
    ||| hand this turn" names both; Myth Unbound's passive names only the
    ||| zone). Coordination is a plain list at the zone sort and no marked
    ||| union row -- "from your hand or library" is two zones.
    ||| -- spelling: "[what] from [src]".
    FromZones : (src : EventSource bs) ->
                (what : Maybe (EventComplement bs ev ks)) ->
                {auto 0 pl : So (complementPlain what)} ->
                {auto 0 ok : So (lookbackSourceOk ev src)} ->
                EventComplement bs ev ks
    ||| "a creature card was put into your graveyard from anywhere this
    ||| turn", "if a permanent was put into your hand from the battlefield
    ||| this turn": the complement's THIRD payload sort, the zone the
    ||| event's object ARRIVED in. [CR#400.7] makes a zone change a move
    ||| from one zone to another, so a placement's clause has two ends to
    ||| name and this is the far one; `lookbackDestOk` keys which events
    ||| write it, and a placement is the only one that does.
    ||| The origin rides INSIDE it, because English writes the
    ||| destination first ("into your graveyard from anywhere") and never
    ||| the other way round; `complementSourced` is what admits the
    ||| nesting one way and refuses it the other. Leaving the destination
    ||| off is the relative clause's own reading -- "creature cards in
    ||| your graveyard that were put THERE from the battlefield this turn"
    ||| names it by the described noun's own zone -- so a bare
    ||| `FromZones` stands for that reading and this row is written where
    ||| the zone is.
    ||| -- spelling: "put into [to] [what]".
    IntoZone : (to : ZoneExpr bs) ->
               (what : Maybe (EventComplement bs ev ks)) ->
               {auto 0 pl : So (complementSourced what)} ->
               {auto 0 ok : So (lookbackDestOk ev (zoneSort to))} ->
               EventComplement bs ev ks

  public export
  data ComplementWritten : {0 bs : Bindings} -> {0 ev : EventName} ->
                           {0 ks : Kind} ->
                           Maybe (EventComplement bs ev ks) -> Type where
    LeftBare : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
               {auto 0 ok : So (bareLookbackOk ev ks)} ->
               ComplementWritten {bs} {ev} {ks} Nothing
    Written : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
              {0 c : EventComplement bs ev ks} -> ComplementWritten (Just c)

  ||| What an ORIGIN may wrap: a participant, and nothing that names a
  ||| second zone.
  public export
  complementPlain : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                    Maybe (EventComplement bs ev ks) -> Bool
  complementPlain Nothing = True
  complementPlain (Just (Involving _)) = True
  complementPlain (Just (FromZones _ _)) = False
  complementPlain (Just (IntoZone _ _)) = False

  ||| What a DESTINATION may wrap: the origin, or a participant. Not a
  ||| second destination -- one clause names one arrival.
  public export
  complementSourced : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                      Maybe (EventComplement bs ev ks) -> Bool
  complementSourced Nothing = True
  complementSourced (Just (Involving _)) = True
  complementSourced (Just (FromZones _ _)) = True
  complementSourced (Just (IntoZone _ _)) = False

  ||| Every named zone must be one the event's clause may name as its
  ||| origin, and a coordination that names none is no coordination.
  public export
  lookbackZonesOk : {0 bs : Bindings} -> EventName -> List (ZoneExpr bs) -> Bool
  lookbackZonesOk _ [] = False
  lookbackZonesOk ev (z :: zs) =
    lookbackOriginOk ev (zoneSort z) && allOriginZonesOk ev zs

  public export
  allOriginZonesOk : {0 bs : Bindings} -> EventName -> List (ZoneExpr bs) -> Bool
  allOriginZonesOk _ [] = True
  allOriginZonesOk ev (z :: zs) =
    lookbackOriginOk ev (zoneSort z) && allOriginZonesOk ev zs

  public export
  data Predicate : Bindings -> Kind -> Type where
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
    HasSubtype : Subtype -> Predicate bs Object
    AnyPlayer : Predicate bs Player                      -- head noun "player" (any player, [CR#102.1])
    Opponent : Predicate bs Player                       -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    ||| "the chosen player", "the last chosen player": the read of a
    ||| player an earlier chooser bound [CR#607.2d]. At this kind the
    ||| read IS the head noun, where the object-side `OfChosen` is a
    ||| description matching a chosen value against a characteristic --
    ||| a player is no characteristic of anything [CR#109.3], so there is
    ||| nothing for that shape to match and the phrase names the referent
    ||| outright.
    ||| ONE row for both spellings, on `ChosenNumber`'s ground:
    ||| [CR#607.2d] writes one linkage over "the chosen [value]", "the
    ||| last chosen [value]," or similar. The gate is existence rather
    ||| than `OfChosen`'s uniqueness because the marked spelling is
    ||| printed behind a repeatable chooser, and the two gates agree
    ||| over the whole corpus: no supported card writes two singular
    ||| player choosers (measured zero), so nothing here weakens what
    ||| `badTwoChoosersOneSortRead` pins at the quality sorts.
    ||| -- spelling: "the chosen player" behind a single chooser, "the
    ||| last chosen player" behind a repeatable one.
    ChosenPlayer : {auto 0 ok : ChoiceStands (countChoice PlayerC bs)} ->
                   Predicate bs Player
    QualityNoun : (q : QualitySort) ->
                  (dom : Maybe (ChoiceDomain (QSort q))) ->
                  Predicate bs (Quality q)
    ||| "a counter on [n]", "a kind of counter on [n]": the counter-kind
    ||| noun whose range the BOARD fixes rather than a printed menu. ONE
    ||| row for both spellings, and the rule is what merges them:
    ||| [CR#122.1] makes "counters with the same name or description
    ||| interchangeable", so pointing at a counter on a permanent picks
    ||| out nothing but its kind -- a counter is not an object and has no
    ||| characteristics to tell two of a name apart.
    ||| It is a DESCRIPTION and not a `ChoiceDomain`: a domain is a
    ||| bindingless narrowing of a sort ("a color other than red"), while
    ||| this one names a permanent, and naming it is what lets the next
    ||| sentence say "each OTHER creature you control" -- the mention the
    ||| description leaves is the anchor that "other" is other than.
    ||| The holder is an object; [CR#122.1]'s player holder ("a counter
    ||| on target permanent or player", Animation Module) waits for a
    ||| witness, that card being blocked on its union recipient.
    ||| -- spelling: "a counter on [n]" (Aven Courier, Animation Module),
    ||| "a kind of counter on [n]" (Contractual Safeguard).
    CounterKindOn : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    Predicate bs (Quality CounterKindQ)
    -- reads the unique chosen quality; the choice was made at
    -- resolution by another clause [CR#608.2d].
    OfChosen : (q : QualitySort) -> {auto 0 ok : countChoice (QSort q) bs = 1} ->
               {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    ||| The marked read, at every sort the plain one reads: existence
    ||| rather than `OfChosen`'s uniqueness [CR#607.2d], since "the last
    ||| chosen [value]" names the latest of however many the chooser
    ||| made; bindings are nearest-first, so the latest choice is what it
    ||| reads. Sorted for `OfChosen`'s reason -- [CR#607.2d] links the
    ||| reader to a choice OF THAT VALUE -- and gated by the same
    ||| `ChosenQualityRead`, because the two spellings match a chosen
    ||| value against a characteristic in exactly the same way and
    ||| differ only in which of the chooser's choices they name.
    ||| -- spelling: "the last chosen color" (Sanctuary Blade), "the
    ||| last chosen name", "the last chosen creature type" (Psychic
    ||| Paper).
    OfLastChosen : (q : QualitySort) ->
                   {auto 0 ok : ChoiceStands (countChoice (QSort q) bs)} ->
                   {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    -- the determiner that chooses in its own phrase, where `OfChosen`
    -- reads a choice made by some other clause [CR#607.2d] — there is
    -- no first ability for this one to be linked to.
    ||| The domain slot is `QualityNoun`'s, at the determiner that
    ||| chooses in its own phrase: "the BASIC land type of your choice"
    ||| narrows the same sort the same way [CR#305.6] narrows it for a
    ||| separate chooser, and the index refuses every crossing for free.
    OfYourChoice : (q : QualitySort) -> (dom : Maybe (ChoiceDomain (QSort q))) ->
                   {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    HasKeyword : (k : KeywordLabel) -> {auto 0 kn : KnownKeyword k} ->
                 Predicate bs Object
    ControlledBy : (n : Noun bs Player) -> {auto 0 ps : SoleHolder n} -> Predicate bs Object
    CastBy : (n : Noun bs Player) -> {auto 0 ps : SoleHolder n} -> Predicate bs Object
    CastFrom : (z : ZoneExpr bs) ->
               {auto 0 pf : So (playableFrom (Just (zoneSort z)))} ->
               Predicate bs Object
    ||| "it wasn't cast": the casting history read with NO agent and no
    ||| origin -- the bare question of whether the object went through
    ||| [CR#601.2]'s procedure at all. `CastBy` and `CastFrom` each
    ||| narrow a casting that happened; this one asks whether there was
    ||| one, which is the question an entry replacement puts to a
    ||| permanent that may have arrived without a spell: [CR#111.1]
    ||| makes a token an object that never was a card on the stack, and
    ||| [CR#707.10] makes even a spell's own copy uncast.
    ||| Not `Not (CastBy p)` for any p: that denies one NAMED player's
    ||| casting and leaves every other player's standing.
    ||| It is history and not a location, so it seeds no zone, on
    ||| `CastBy`'s own reasons [CR#400.7d].
    ||| -- spelling: "[n] was cast"; under `Not`, "[n] wasn't cast".
    WasCast : Predicate bs Object
    Attacking : Predicate bs Object
    ||| "[n] is being declared as an attacker": the attack declaration IN
    ||| PROGRESS, which is not the same question as attacking.
    ||| [CR#508.1a] has the active player choose which creatures will
    ||| attack; [CR#508.1f] taps those chosen creatures, stating that
    ||| "attacking simply causes creatures to become tapped"; and
    ||| [CR#508.1k] makes each of them an attacking creature only after
    ||| that. So at the moment of the tap the creature is not yet
    ||| attacking, and `Attacking` answers the wrong question -- which is
    ||| why the two printed lines that watch the tap (Verity Circle,
    ||| Rhoda, Geist Avenger) need this word to exclude the declaration's
    ||| own tap and cannot say "isn't attacking".
    ||| It seeds the battlefield and the creature type for `Attacking`'s
    ||| reason: [CR#508.1a] chooses from the creatures the active player
    ||| controls.
    ||| -- spelling: "[n] is being declared as an attacker".
    BeingDeclaredAttacker : Predicate bs Object
    Blocking : Predicate bs Object
    BlockerOf : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    BlockedBy : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    ||| "creature that could block [m]" -- the hypothetical block, asked of
    ||| a block that has not been declared. [CR#509.1a] and [CR#509.1b] are
    ||| the whole of what it reads: the prospective blocker must be
    ||| untapped, and no restriction ("effects that say a creature can't
    ||| block, or that it can't block unless some condition is met") may be
    ||| disobeyed. It reads NEITHER of the two checks that follow.
    ||| [CR#509.1c]'s requirements are no part of it -- a creature that must
    ||| block could block whether or not it is made to -- and neither are
    ||| that same rule's costs, which "that player is not required to pay"
    ||| and which [CR#509.1d] locks in only once blockers are chosen. That
    ||| refusal is the construction's content, not an omission: a reading
    ||| that took either in would answer Sorrow's Path's question wrongly.
    ||| The relatum is the attacking creature and is gated like every other
    ||| combat relatum [CR#509.1g]; the described side is a creature by
    ||| `seedType`, as `BlockerOf`'s is.
    ||| -- spelling: "[n] could block [m]"
    CouldBlock : (m : Noun bs Object) ->
                 {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                 Predicate bs Object
    ||| "creature that could be blocked by [m]" -- `CouldBlock`'s other
    ||| voice, on the model of `BlockerOf`/`BlockedBy`. Same rules, same
    ||| refusal; the two differ only in which side the description is of,
    ||| and General Jarkeld writes this one where Sorrow's Path writes the
    ||| other.
    ||| -- spelling: "[n] could be blocked by [m]"
    CouldBeBlockedBy : (m : Noun bs Object) ->
                       {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                       Predicate bs Object
    HappenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
                 (what : Maybe (EventComplement bs ev k)) ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
    ColorIs : (c : Chroma.Color) -> Predicate bs Object
    IsColorless : Predicate bs Object
    Multicolored : Predicate bs Object
    Monocolored : Predicate bs Object
    ||| "that's exactly two colors", "exactly three colors": how many of
    ||| [CR#105.1]'s colours an object is, counted rather than bounded --
    ||| `Multicolored` is [CR#105.2b]'s two-or-more and `Monocolored`
    ||| [CR#105.2a]'s one. `colorCountOk` floors it at two and ceilings it
    ||| at five: the lower counts are printed with their own words and
    ||| there is no sixth colour.
    ||| -- spelling: "exactly [n] colors"
    ExactlyColors : (n : Nat) -> {auto 0 ok : So (colorCountOk n)} ->
                    Predicate bs Object
    HasSupertype : (s : Supertype) -> Predicate bs Object
    Named : (src : NameSource bs) -> Predicate bs Object
    HasDesignation : (d : Designation) ->
                     {auto 0 sc : designationScope d = HeldBy k} ->
                     {auto 0 at : So (designationChecked d)} ->
                     Predicate bs k
    IsAttached : (w : AttachWord) ->
                 {auto 0 ok : So (attachedCheckOk w)} -> Predicate bs Object
    ||| "enchanted by two or more Auras", "enchanted by an Aura you
    ||| control", "enchanted by other Auras": the attachment with its
    ||| ATTACHERS described, where `IsAttached` asks only whether the
    ||| permanent is attached to anything. [CR#303.4] writes the phrase in
    ||| the rules' own words -- "other effects can limit what a permanent
    ||| can be enchanted by" -- and [CR#701.3a] puts each attachment onto
    ||| the permanent one at a time with no rule capping how many, so how
    ||| MANY are attached is a real question the bare word cannot put.
    ||| The count and every other narrowing ride the attachers' own
    ||| determiner rather than a slot here, on `HappenedTo`'s policy: the
    ||| complement is one noun and any richer narrowing is that noun's
    ||| predicate.
    ||| -- spelling: "[n] is enchanted by [by]".
    AttachedBy : (w : AttachWord) -> (by : Noun bs Object) ->
                 {auto 0 ok : So (attachedCheckOk w)} -> Predicate bs Object
    Permanent : Predicate bs Object
    ||| "a card", "one or more cards": the bare card head -- the word with
    ||| no zone written beside it. [CR#109.2] names "card" among the four
    ||| words that take a description OUT of the battlefield-permanent
    ||| default, so the word heads a phrase on its own; [CR#108.2] is what
    ||| it means, a Magic card or an object represented by one, and
    ||| [CR#108.2b] keeps tokens out of it.
    ||| Beside `InZone` and not instead of it: [CR#109.2a] is the reading
    ||| of the word written TOGETHER with a zone, which is the zone
    ||| clause's own head, and this is the same word where the clause
    ||| writes no zone. So it seeds none: [CR#109.2a] fixes a zone only
    ||| where the text states one, and "one or more cards are put into
    ||| exile" (Stonebinder's Familiar) states the destination of the
    ||| move, never a zone of the phrase.
    ||| -- spelling: "card", "cards".
    IsCard : Predicate bs Object
    IsToken : Predicate bs Object
    HasStatus : {c : StatusCat} -> (v : StatusVal c) -> Predicate bs Object
    HasCounters : (kind : Maybe CounterKind) ->
                  {auto 0 kn : CounterKindNamed Object kind} ->
                  Predicate bs Object
    ||| "if this spell was kicked", "if its madness cost was paid", "a
    ||| kicked spell": the object's controller declared, as it was cast,
    ||| the intention to pay one of the object's own optional costs
    ||| [CR#702.33d]. STATE on the object and not a mention threaded from
    ||| an earlier clause -- [CR#707.2] copies "whether it was kicked"
    ||| along with the spell's other casting choices, and [CR#702.152a]
    ||| reads the same state off the permanent the spell became -- so the
    ||| read is anchored to whatever object it describes and the zone is
    ||| ungated. [CR#607.2i] links the offering ability to this one and
    ||| makes the read say WHICH cost, which is all the `PaidCostName`
    ||| slot is; no clause mints a name and none is matched.
    ||| ONE relation and not the crate's pair. The rules write the same
    ||| readback for an additional cost [CR#702.27a] and for an
    ||| alternative one [CR#702.34a], and the corpus writes both as
    ||| "[keyword] cost was paid" -- which kind a keyword offers is that
    ||| keyword's fact, never the reading clause's.
    ||| [CR#702.33c] makes a multikicker cost a kicker cost, so the word
    ||| a card declares and the word a read names may differ; reading
    ||| "Multikicker" is the same state under the other name and is
    ||| tolerated.
    ||| -- spelling: "[n] was kicked", "[n]'s [keyword] cost was paid",
    ||| "[n] was cast for its [keyword] cost"; with the ordinal, "[n] was
    ||| kicked with its [cost] [keyword]".
    PaidCost : (which : PaidCostName) ->
               {auto 0 nc : PaidCostNamed which} -> Predicate bs Object
    -- the bound slot is open to any amount at every relation; see
    -- `CompareAmt`'s docstring for the recorded verdict.
    Compare : (c : Characteristic) -> (r : Comparator) -> (bound : Amount bs) ->
              Predicate bs Object
    ||| The bound read of a counter-bearing description: the referent's own
    ||| count of [kind] counters on a comparison's left. At the kind index —
    ||| the poison lines are the player cells ([CR#122.1f] states the
    ||| player-side test in this ranged shape) and no marked player twin
    ||| exists. Not a quantity slot on `HasCounters`, which stays the bare
    ||| existence read.
    ||| -- spelling: at Object, "with [bound] or more/fewer [kind] counters
    ||| on it/them"; at Player, "who has [bound] or more [kind] counters".
    CounterCompare : (kind : Maybe CounterKind) -> (r : Comparator) ->
                     (bound : Amount bs) ->
                     {auto 0 kn : CounterKindNamed k kind} ->
                     Predicate bs k
    Superlative : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                  (dom : Predicate bs k) ->
                  {auto 0 ex : IsExtremal op} ->
                  {auto 0 sc : projScope ax = k} ->
                  Predicate bs k
    ||| "an opponent who controls more lands than you", "a player who has
    ||| more cards in hand than you": the member-relative comparison.
    ||| `Compare` reads one of the referent's OWN characteristics against
    ||| a bound; this reads a whole AMOUNT taken on the referent, on
    ||| `AggregateOver`'s element binder -- the domain binds one member
    ||| (`TheD`, `OneOf`) and the measured side reads it back as
    ||| `They`/`It`. The domain rides the row rather than being conjoined
    ||| beside it for `Superlative`'s reason: the member the measurement
    ||| is taken on is the domain's member, and nothing outside the row
    ||| can bind it.
    ||| The bound is read in the OUTER context, so "than you" is the
    ||| reader's own count and no member leaks into it.
    ||| It leaves the margin behind, as `CompareAmt` does: [CR#608.2h]
    ||| settles both counts once, when the effect applies, so the amount
    ||| by which one exceeds the other is a determinate number, and
    ||| [CR#608.2c] is what lets the text after it name that number.
    ||| -- spelling: "[dom] who/that [measure] more/fewer … than [bound]".
    CompareOver : {k : Kind} -> (dom : Predicate bs k) ->
                  {auto ph : Phrasal k} ->
                  (measure : Amount (bindFor TheD OneOf ph dom
                                       :: (predDelta dom ++ bs))) ->
                  (r : Comparator) -> (bound : Amount bs) ->
                  Predicate bs k
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
    ExiledWith : (src : Noun bs Object) ->
                 {auto 0 ls : LinkSource src} -> Predicate bs Object
    And : (ps : List (Predicate bs k)) -> {auto 0 zc : ZoneCoherent ps} ->
          {auto 0 cf : ContradictionFree ps} -> {auto 0 oa : OtherAnchored ps} ->
          Predicate bs k
    Or : (ps : List (Predicate bs k)) -> {auto 0 ne : NonEmpty ps} ->
         {auto 0 pd : ParallelDisjuncts ps} ->
         {auto 0 cd : CoordinableDisjuncts ps} ->
         {auto 0 dd : DistinctDisjuncts ps} -> Predicate bs k
    Not : (p : Predicate bs k) -> {auto 0 ng : Negatable p} -> Predicate bs k
    Other : {auto 0 ok : So (anyTargeted k bs)} -> Predicate bs k
    ||| The complement names a referent to SUBTRACT [CR#601.2c], not a
    ||| member of the phrase's own domain, so it is described at its own
    ||| kind: "any target other than this creature" subtracts an object
    ||| from a phrase that may denote a player too.
    OtherThan : {kn : Kind} -> (n : Noun bs kn) ->
                {auto 0 ca : ComplementAnchor n} -> Predicate bs k
    ||| The cross-kind head: one description per kind, and the phrase's kind
    ||| is their join. "Target creature or player" is
    ||| `Joined (HasType Creature) AnyPlayer`, admitted by [CR#115.1]: a
    ||| spell's targets are objects and/or players. "Any target" is the same
    ||| row over [CR#115.4]'s own four-way list. Which half English writes first
    ||| is the spelling layer's business, so the semantics fixes the object
    ||| arm left and one kind order serves every phrasing.
    Joined : {ka : Kind} -> {kb : Kind} -> (l : Predicate bs ka) ->
             (r : Predicate bs kb) -> Predicate bs (ka \/ kb)
    ||| "each player whose coin comes up tails", "each creature whose
    ||| coin comes up tails": the uncalled face read [CR#705.2]
    ||| distributed over the members a clause flipped a coin for. The
    ||| `FlipFace` condition reads the one coin a clause flipped and so
    ||| takes no subject; this narrows a described set by each member's
    ||| own coin, which is what a per-member flip ("each player flips a
    ||| coin", "flip a coin for each creature") leaves to read. It is a
    ||| description and not a condition for that reason alone -- the
    ||| members it keeps are the ones whose coin showed the face.
    ||| No player wins or loses a flip read this way [CR#705.2], so no
    ||| call rides here either. Both kinds are printed and [CR#705.1]
    ||| makes a coin a physical object no kind of referent owns, so the
    ||| gate is the coarse one the join lattice gives: a coin flipped for
    ||| an ability or a turn is a category error, and nothing narrower is
    ||| written.
    ||| -- spelling: "[dom] whose coin comes up [face]".
    CoinCameUp : {k : Kind} -> (face : CoinFace) ->
                 {auto 0 fl : So (coinFlipInScope bs)} ->
                 {auto 0 rk : So (kindLte k (Object \/ Player))} ->
                 Predicate bs k
    IsSource : Predicate bs Object
    ||| "spells with {X} in their mana costs": an object described by a
    ||| SYMBOL its printed mana cost writes. [CR#202.1] makes the mana
    ||| cost a printed characteristic indicated by mana symbols, and
    ||| [CR#107.3a] makes {X} in a mana cost the placeholder whose value
    ||| the controller announces as the spell is cast -- so the phrase
    ||| asks about the printed cost and never about the value X took,
    ||| which is why it is a predicate here and not a comparison against
    ||| an amount. `manaHasX` is the reader, and it was already written.
    |||
    ||| ONE symbol, not a matcher over the symbol vocabulary. Gaddock
    ||| Teeg's second line is the only supported sentence that describes
    ||| an object by a symbol in its cost; a general matcher would spell
    ||| a description for every `ManaSymbol` and no card writes one.
    ||| -- spelling: "with {X} in [its/their] mana cost[s]".
    ManaCostHasX : Predicate bs Object
    AbilityHead : (cls : AbilityClass) -> Predicate bs Ability
    AbilityOf : (src : Noun bs Object) -> Predicate bs Ability
    ActivatedBy : (who : Noun bs Player) ->
                  {auto 0 ps : SoleHolder who} -> Predicate bs Ability
    IsManaAbility : Predicate bs Ability
    ||| "spell that targets this creature", "spells your opponents cast
    ||| that target this creature", "abilities you activate that target
    ||| this creature": a spell or an ability described by what it
    ||| targets. [CR#115.9b] states this reading in the rules' own words
    ||| -- "an object that looks for a '[spell or ability] that targets
    ||| [something]'" -- so the relation is the rules' own and not a
    ||| paraphrase.
    ||| The other voice of `GameEvent`'s `BecomesTarget`, sharing its two
    ||| gates: the described side is a `Targeter`
    ||| [CR#115.1a,115.1c,115.1d] and the side it names is `Targetable`'s
    ||| own set, which [CR#115.1] closes to objects and players. Deciding
    ||| the relation once and spelling it at both seats is
    ||| `BlockerOf`/`BlockedBy`'s economy across two vocabularies.
    ||| The described side is kind-indexed and not object-kinded because
    ||| [CR#115.1c,115.1d] give the word to an ability as readily as
    ||| [CR#115.1a] gives it to a spell, and the corpus describes both.
    ||| The extent is a slot for [CR#115.9c]'s reason: "targets only
    ||| [something]" is a second check the rules state on the same
    ||| relation, over how many different things were chosen.
    ||| It does NOT name WHICH instance of the word: [CR#115.3] lets one
    ||| spell choose the same thing once per instance of "target", and
    ||| [CR#115.9c] reads such a spell as targeting it once all the same.
    ||| -- spelling: "[n] that target(s) [m]"; with `SoleTarget`, "[n]
    ||| that target(s) only [m]".
    Targets : {kt : Kind} -> (m : Noun bs kt) -> (extent : TargetExtent) ->
              {auto 0 tk : Targetable kt} ->
              {auto 0 tr : Targeter k} -> Predicate bs k

  ||| The head type a predicate projects onto its referent — what an
  ||| anaphor remembers across a zone change.
  public export
  seedTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedTy (HasType t) = Just t
  seedTy (And ps) = seedTyAll ps
  seedTy (Or ps) = seedTyJoin ps
  seedTy (Joined l r) = joinSeed (seedTy l) (seedTy r)
  seedTy (CompareOver dom _ _ _) = seedTy dom
  seedTy _ = Nothing

  ||| The head type per half of the phrase's kind. Only a joined head has
  ||| more than one to give; every other phrase names one description,
  ||| which each half of a joined kind then shares. `And`/`Or` over a
  ||| joined kind fold to one description through `seedTy`: a fold names
  ||| the whole phrase's head, not one half's.
  public export
  seedTys : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> HeadTy k
  seedTys (Joined l r) = JoinTy (seedTys l) (seedTys r)
  seedTys p = SoleTy (seedTy p)

  public export
  seedTyAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyAll [] = Nothing
  seedTyAll (p :: ps) = case seedTy p of
    Just t => Just t
    Nothing => seedTyAll ps

  public export
  seedTyJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyJoin [] = Nothing
  seedTyJoin (p :: ps) = case seedTy p of
    Nothing => Nothing
    Just t => if allSeedTy t ps then Just t else Nothing

  public export
  allSeedTy : {0 bs : Bindings} -> {0 k : Kind} ->
              CardType -> List (Predicate bs k) -> Bool
  allSeedTy t [] = True
  allSeedTy t (p :: ps) = case seedTy p of
    Nothing => False
    Just u => t == u && allSeedTy t ps

  public export
  optCT : Maybe CardType -> List CardType
  optCT Nothing = []
  optCT (Just t) = [t]

  public export
  headTys : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  headTys (And ps) = headTysAll ps
  headTys (Or ps) = headTysJoin ps
  headTys (Joined l r) = headTys l ++ headTys r
  headTys p = optCT (seedTy p)

  public export
  headTysJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> List CardType
  headTysJoin [] = []
  headTysJoin (p :: ps) = headTys p ++ headTysJoin ps

  public export
  headTysAll : {0 bs : Bindings} -> {0 k : Kind} ->
               List (Predicate bs k) -> List CardType
  headTysAll [] = []
  headTysAll (p :: ps) = case headTys p of
    [] => headTysAll ps
    ts => ts

  public export
  seedZone : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe Zone
  seedZone (InZone z) = Just (zoneSort z)
  seedZone Attacking = Just Battlefield
  seedZone BeingDeclaredAttacker = Just Battlefield
  seedZone Blocking = Just Battlefield
  seedZone (BlockerOf _) = Just Battlefield
  seedZone (BlockedBy _) = Just Battlefield
  seedZone (CouldBlock _) = Just Battlefield
  seedZone (CouldBeBlockedBy _) = Just Battlefield
  seedZone (HappenedTo _ _ _) = Nothing
  seedZone (ColorIs _) = Nothing
  seedZone IsColorless = Nothing
  seedZone Multicolored = Nothing
  seedZone Monocolored = Nothing
  seedZone (ExactlyColors _) = Nothing
  seedZone (HasSupertype _) = Nothing
  seedZone (Named _) = Nothing
  seedZone (HasDesignation d) = designationSeedZone d
  seedZone (CoinCameUp _) = Nothing
  seedZone (IsAttached _) = Just Battlefield
  seedZone (AttachedBy _ _) = Just Battlefield
  seedZone IsToken = Just Battlefield
  seedZone (HasStatus _) = Just Battlefield
  seedZone (HasCounters _) = Nothing
  -- payment is history the object carries [CR#707.2], readable on the
  -- stack and on the permanent the spell became [CR#702.152a]
  seedZone (PaidCost _) = Nothing
  seedZone (ControlledBy _) = Nothing
  -- Casting is history, not a location: [CR#400.7d] lets a permanent's
  -- ability reference the spell it was cast as, and [CR#702.40a] counts
  -- spells cast earlier this turn that have long left the stack.
  seedZone (CastBy _) = Nothing
  -- a spell or ability has its targets while it is on the stack:
  -- [CR#115.1] declares them as it is put there, and [CR#115.9b] reads
  -- the current ones back. The ability side carries no zone of its own
  -- and ignores this.
  seedZone (Targets _ _) = Just Stack
  seedZone (ExiledWith _) = Just Exile
  seedZone (CompareOver dom _ _ _) = seedZone dom
  seedZone (And ps) = seedZoneAll ps
  seedZone (Or ps) = seedZoneJoin ps
  seedZone _ = Nothing

  public export
  seedZoneAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneAll [] = Nothing
  seedZoneAll (p :: ps) = case seedZone p of
    Just z => Just z
    Nothing => seedZoneAll ps

  public export
  seedZoneJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneJoin [] = Nothing
  seedZoneJoin (p :: ps) = case seedZone p of
    Nothing => Nothing
    Just z => if allSeedZone z ps then Just z else Nothing

  public export
  allSeedZone : {0 bs : Bindings} -> {0 k : Kind} ->
                Zone -> List (Predicate bs k) -> Bool
  allSeedZone z [] = True
  allSeedZone z (p :: ps) = case seedZone p of
    Nothing => False
    Just w => z == w && allSeedZone z ps

  public export
  seedsToken : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  seedsToken IsToken = True
  seedsToken (And ps) = seedsTokenAny ps
  seedsToken (Or ps) = seedsTokenAll ps
  seedsToken (CompareOver dom _ _ _) = seedsToken dom
  seedsToken _ = False

  public export
  seedsTokenAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAny [] = False
  seedsTokenAny (p :: ps) = seedsToken p || seedsTokenAny ps

  public export
  seedsTokenAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAll [] = False
  seedsTokenAll (p :: ps) = seedsToken p && seedsTokenAll ps

  public export
  zoneAdmit : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  zoneAdmit (ControlledBy _) = [Battlefield, Stack]
  zoneAdmit _ = []

  ||| The card type a predicate presupposes of its referent — distinct
  ||| from `seedTy`, which projects the phrase's own head [CR#506.3].
  public export
  seedType : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedType Attacking = Just Creature
  seedType BeingDeclaredAttacker = Just Creature
  seedType Blocking = Just Creature
  seedType (BlockerOf _) = Just Creature
  seedType (BlockedBy _) = Just Creature
  seedType (CouldBlock _) = Just Creature
  seedType (CouldBeBlockedBy _) = Just Creature
  seedType (HappenedTo _ _ _) = Nothing
  seedType (ColorIs _) = Nothing
  seedType IsColorless = Nothing
  seedType Multicolored = Nothing
  seedType Monocolored = Nothing
  seedType (ExactlyColors _) = Nothing
  seedType (HasSupertype _) = Nothing
  seedType (Named _) = Nothing
  seedType (HasDesignation d) = designationSeedType d
  seedType (IsAttached _) = Nothing
  seedType (AttachedBy _ _) = Nothing
  -- [CR#115.1a,115.1c,115.1d] give the word to a spell and to an
  -- ability, neither of which is a card type the described thing has.
  seedType (Targets _ _) = Nothing
  seedType (Compare c _ _) = comparedType c
  seedType (Superlative _ (CharAxis c) _) = comparedType c
  seedType (Superlative _ (PlayerStatAxis _) _) = Nothing
  seedType (CompareOver dom _ _ _) = seedType dom
  seedType (HasSubtype s) = Just (subtypeType s)
  seedType (And ps) = seedTypeAll ps
  seedType (Or ps) = seedTypeJoin ps
  seedType _ = Nothing

  public export
  seedTypeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> Maybe CardType
  seedTypeAll [] = Nothing
  seedTypeAll (p :: ps) = case seedType p of
    Just t => Just t
    Nothing => seedTypeAll ps

  public export
  seedTypeJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                 List (Predicate bs k) -> Maybe CardType
  seedTypeJoin [] = Nothing
  seedTypeJoin (p :: ps) = case seedType p of
    Nothing => Nothing
    Just t => if allSeedType t ps then Just t else Nothing

  public export
  allSeedType : {0 bs : Bindings} -> {0 k : Kind} ->
                CardType -> List (Predicate bs k) -> Bool
  allSeedType t [] = True
  allSeedType t (p :: ps) = case seedType p of
    Nothing => False
    Just u => t == u && allSeedType t ps

  ||| Whether a description writes its own head noun. English word class
  ||| only: the kind index, not the head word, supplies a description's
  ||| domain, and [CR#109.2] assigns a default zone to a description that
  ||| names a card type or subtype without saying anything about one that
  ||| does not. No determiner demands a head, and no gate demands one. The
  ||| word class is read by `parallelDisjuncts` alone, to keep the arms of
  ||| one coordination alike.
  public export
  hasHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasHead (HasType _) = True
  hasHead (HasSubtype _) = True
  hasHead AnyPlayer = True
  hasHead Opponent = True
  hasHead ChosenPlayer = True
  hasHead (QualityNoun _ _) = True
  hasHead (CounterKindOn _) = True
  hasHead (OfChosen _) = False
  hasHead (OfLastChosen _) = False
  hasHead (OfYourChoice _ _) = False
  hasHead (AbilityHead _) = True
  hasHead (AbilityOf _) = False
  hasHead (ActivatedBy _) = False
  hasHead IsManaAbility = False
  -- a relative clause, never the head noun: the head is the "spell" or
  -- "ability" the clause hangs off [CR#115.9b].
  hasHead (Targets _ _) = False
  hasHead IsSource = True
  hasHead ManaCostHasX = False
  hasHead (HasKeyword _) = False
  hasHead (ControlledBy _) = False
  hasHead (CastBy _) = False
  hasHead Attacking = False
  hasHead BeingDeclaredAttacker = False
  hasHead Blocking = False
  hasHead (BlockerOf _) = False
  hasHead (BlockedBy _) = False
  hasHead (CouldBlock _) = False
  hasHead (CouldBeBlockedBy _) = False
  hasHead (HappenedTo _ _ _) = False
  hasHead (CastFrom _) = False
  hasHead WasCast = False
  hasHead (ColorIs _) = False
  hasHead IsColorless = False
  hasHead Multicolored = False
  hasHead Monocolored = False
  hasHead (ExactlyColors _) = False
  hasHead (HasSupertype _) = False
  hasHead (Named _) = False
  hasHead (HasDesignation _) = False
  hasHead (CoinCameUp _) = False
  hasHead (IsAttached _) = False
  hasHead (AttachedBy _ _) = False
  hasHead Permanent = True
  hasHead IsCard = True
  hasHead IsToken = True
  hasHead (HasStatus _) = False
  hasHead (HasCounters _) = False
  hasHead (PaidCost _) = False
  hasHead (Compare _ _ _) = False
  hasHead (CounterCompare _ _ _) = False
  hasHead (Superlative _ _ _) = False
  -- the row writes its domain out ("an opponent who ..."), so the head
  -- word is the domain's.
  hasHead (CompareOver dom _ _ _) = hasHead dom
  hasHead (InZone _) = True
  hasHead (ExiledWith _) = True
  -- ANY member heads a conjunction, but EVERY alternative has to head a
  -- disjunction: each disjunct stands where the phrase's head would.
  hasHead (And ps) = hasHeadAny ps
  hasHead (Or ps) = hasHeadAll ps
  hasHead (Not _) = False
  hasHead Other = False
  hasHead (OtherThan _) = False
  hasHead (Joined _ _) = True

  public export
  hasHeadAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAny [] = False
  hasHeadAny (p :: ps) = hasHead p || hasHeadAny ps

  public export
  hasHeadAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAll [] = True
  hasHeadAll (p :: ps) = hasHead p && hasHeadAll ps

  public export
  qualityReadOk : {0 bs : Bindings} -> Predicate bs Object -> Bool
  qualityReadOk (OfChosen _) = True
  qualityReadOk (OfLastChosen _) = True
  qualityReadOk (OfYourChoice _ _) = True
  qualityReadOk (Named _) = True
  qualityReadOk _ = False

  public export
  QualityRead : Predicate bs Object -> Type
  QualityRead {bs} p = So (qualityReadOk p)

  ||| The card type a chosen-quality read's sort belongs to, where its
  ||| sort has one. [CR#205.3c] correlates each subtype to its own card
  ||| type, so a read at `SubtypeQ h` names a value only an `h` can
  ||| carry; the other sorts name characteristics every object has
  ||| [CR#109.3] and answer `Nothing`.
  public export
  qualityReadHost : {0 bs : Bindings} -> Predicate bs Object -> Maybe CardType
  qualityReadHost (OfChosen (SubtypeQ h)) = Just h
  qualityReadHost (OfLastChosen (SubtypeQ h)) = Just h
  qualityReadHost (OfYourChoice (SubtypeQ h) _) = Just h
  qualityReadHost _ = Nothing

  public export
  uniquifiesAny : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  uniquifiesAny [] = False
  uniquifiesAny (p :: ps) = uniquifies p || uniquifiesAny ps

  public export
  uniquifies : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  -- [CR#607.2d]'s linkage makes the phrase name exactly one player.
  uniquifies ChosenPlayer = True
  uniquifies (Superlative _ _ _) = True
  uniquifies (And ps) = uniquifiesAny ps
  uniquifies _ = False

  public export
  Uniquifying : Predicate bs k -> Type
  Uniquifying {bs} {k} p = So (uniquifies p)

  public export
  flattenPs : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k)
  flattenPs [] = []
  flattenPs (And qs :: ps) = flattenPs qs ++ flattenPs ps
  flattenPs (p :: ps) = p :: flattenPs ps

  public export
  zonesAgree : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> List (Predicate bs k) -> Bool
  zonesAgree acc [] = True
  zonesAgree acc (p :: ps) = case seedZone p of
    Nothing => zonesAgree acc ps
    Just z => case acc of
      Nothing => zonesAgree (Just z) ps
      Just w => w == z && zonesAgree (Just w) ps

  public export
  negZonesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  negZonesOf (Not (InZone z)) = [zoneSort z]
  negZonesOf _ = []

  public export
  negZones : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List Zone
  negZones [] = []
  negZones (p :: ps) = negZonesOf p ++ negZones ps

  public export
  zoneAdmits : Zone -> List Zone -> Bool
  zoneAdmits z [] = True
  zoneAdmits z zs = elem z zs

  public export
  zoneAdmitsAll : {0 bs : Bindings} -> {0 k : Kind} ->
                  Zone -> List (Predicate bs k) -> Bool
  zoneAdmitsAll z [] = True
  zoneAdmitsAll z (p :: ps) = zoneAdmits z (zoneAdmit p) && zoneAdmitsAll z ps

  public export
  zonesOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  zonesOk ps = zonesAgree Nothing (flattenPs ps) &&
               not (elem (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                         (negZones (flattenPs ps))) &&
               zoneAdmitsAll (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                             (flattenPs ps)

  public export
  ZoneCoherent : List (Predicate bs k) -> Type
  ZoneCoherent {bs} {k} ps = So (zonesOk ps)

  ||| Syntactic predicate equality, deliberately conservative: `False`
  ||| means "not provably the same referent", so the gate under-refuses
  ||| rather than over-refuses.
  public export
  predEq : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  predEq (HasType a) (HasType b) = a == b
  predEq (HasType _) _ = False
  predEq (HasSubtype a) (HasSubtype b) = a == b
  predEq (HasSubtype _) _ = False
  predEq ChosenPlayer ChosenPlayer = True
  predEq ChosenPlayer _ = False
  predEq AnyPlayer AnyPlayer = True
  predEq AnyPlayer _ = False
  predEq Opponent Opponent = True
  predEq Opponent _ = False
  predEq (QualityNoun a d) (QualityNoun a e) = sameDomainOpt d e
  predEq (QualityNoun _ _) _ = False
  predEq (CounterKindOn a) (CounterKindOn b) = nounEqRef a b
  predEq (CounterKindOn _) _ = False
  predEq (OfChosen a) (OfChosen b) = a == b
  predEq (OfChosen _) _ = False
  predEq (OfLastChosen a) (OfLastChosen b) = a == b
  predEq (OfLastChosen _) _ = False
  predEq (OfYourChoice a d) (OfYourChoice b e) = a == b && sameDomainOpt d e
  predEq (OfYourChoice _ _) _ = False
  predEq (AbilityHead a) (AbilityHead b) = a == b
  predEq (AbilityHead _) _ = False
  predEq (AbilityOf a) (AbilityOf b) = nounEqRef a b
  predEq (AbilityOf _) _ = False
  predEq (ActivatedBy a) (ActivatedBy b) = nounEqRef a b
  predEq (ActivatedBy _) _ = False
  predEq IsManaAbility IsManaAbility = True
  predEq IsManaAbility _ = False
  -- the targeted side is described at its OWN kind, so two of these
  -- carry no comparable mention; `OtherThan`'s ground.
  predEq (Targets _ _) _ = False
  predEq IsSource IsSource = True
  predEq IsSource _ = False
  predEq ManaCostHasX ManaCostHasX = True
  predEq ManaCostHasX _ = False
  predEq (HasKeyword a) (HasKeyword b) = a == b
  predEq (HasKeyword _) _ = False
  predEq (ControlledBy a) (ControlledBy b) = nounEqRef a b
  predEq (ControlledBy _) _ = False
  predEq (CastBy a) (CastBy b) = nounEqRef a b
  predEq (CastBy _) _ = False
  predEq (ExiledWith a) (ExiledWith b) = nounEqRef a b
  predEq (ExiledWith _) _ = False
  predEq Attacking Attacking = True
  predEq Attacking _ = False
  predEq BeingDeclaredAttacker BeingDeclaredAttacker = True
  predEq BeingDeclaredAttacker _ = False
  predEq Blocking Blocking = True
  predEq Blocking _ = False
  predEq (BlockerOf a) (BlockerOf b) = nounEqRef a b
  predEq (BlockerOf _) _ = False
  predEq (BlockedBy a) (BlockedBy b) = nounEqRef a b
  predEq (BlockedBy _) _ = False
  predEq (CouldBlock a) (CouldBlock b) = nounEqRef a b
  predEq (CouldBlock _) _ = False
  predEq (CouldBeBlockedBy a) (CouldBeBlockedBy b) = nounEqRef a b
  predEq (CouldBeBlockedBy _) _ = False
  -- both arguments are closed words, so this row compares wholly rather
  -- than conservatively.
  predEq (HappenedTo a v Nothing) (HappenedTo b w Nothing) =
    sameEventName a b && sameLookback v w
  predEq (HappenedTo _ _ _) _ = False
  predEq (ColorIs a) (ColorIs b) = a == b
  predEq (ColorIs _) _ = False
  predEq IsColorless IsColorless = True
  predEq IsColorless _ = False
  predEq Multicolored Multicolored = True
  predEq Multicolored _ = False
  predEq Monocolored Monocolored = True
  predEq Monocolored _ = False
  predEq (ExactlyColors a) (ExactlyColors b) = a == b
  predEq (ExactlyColors _) _ = False
  predEq (HasSupertype a) (HasSupertype b) = a == b
  predEq (HasSupertype _) _ = False
  predEq (Named a) (Named b) = a == b
  predEq (Named _) _ = False
  predEq (HasDesignation a) (HasDesignation b) = a == b
  predEq (HasDesignation _) _ = False
  predEq (CoinCameUp Heads) (CoinCameUp Heads) = True
  predEq (CoinCameUp Tails) (CoinCameUp Tails) = True
  predEq (CoinCameUp _) _ = False
  predEq (IsAttached a) (IsAttached b) = a == b
  predEq (IsAttached _) _ = False
  predEq (AttachedBy a x) (AttachedBy b y) = a == b && nounEqRef x y
  predEq (AttachedBy _ _) _ = False
  predEq Permanent Permanent = True
  predEq Permanent _ = False
  predEq IsCard IsCard = True
  predEq IsCard _ = False
  predEq IsToken IsToken = True
  predEq IsToken _ = False
  predEq (HasStatus v) (HasStatus w) = sameStatusVal v w
  predEq (HasStatus _) _ = False
  predEq (HasCounters Nothing) (HasCounters Nothing) = True
  predEq (HasCounters (Just a)) (HasCounters (Just b)) = a == b
  predEq (HasCounters _) _ = False
  predEq (PaidCost a) (PaidCost b) = a == b
  predEq (PaidCost _) _ = False
  predEq (Compare c r b) (Compare d s e) = c == d && r == s &&
                                           boundEq b e
  predEq (Compare _ _ _) _ = False
  predEq (CounterCompare Nothing r b) (CounterCompare Nothing s e) =
    r == s && boundEq b e
  predEq (CounterCompare (Just a) r b) (CounterCompare (Just c) s e) =
    a == c && r == s && boundEq b e
  predEq (CounterCompare _ _ _) _ = False
  predEq (Superlative o a d) (Superlative p b e) =
    o == p && a == b && predEq d e
  predEq (Superlative _ _ _) _ = False
  -- the two measurements live under their own domains' binders, so no
  -- comparison of them is even well typed; the conservative answer is
  -- the honest one.
  predEq (CompareOver _ _ _ _) _ = False
  predEq (CastFrom z) (CastFrom w) = zoneSort z == zoneSort w
  predEq (CastFrom _) _ = False
  predEq WasCast WasCast = True
  predEq WasCast _ = False
  predEq (InZone z) (InZone w) = zoneSort z == zoneSort w
  predEq (InZone _) _ = False
  predEq (And xs) (And ys) = predEqAll xs ys
  predEq (And _) _ = False
  -- the blanket row is free here: nothing may nest an `Or` in an `Or`
  -- (`CoordinableDisjuncts`), so two coordinations never meet as
  -- alternatives.
  predEq (Or _) _ = False
  predEq (Not a) (Not b) = predEq a b
  predEq (Not _) _ = False
  predEq Other Other = True
  predEq Other _ = False
  -- the complement carries its own kind index, so two anchors need not be
  -- comparable; conservative, like the `Or` row above. Nothing is lost:
  -- `coordinable` keeps a complement out of an `Or`, and `OtherAnchored`
  -- admits at most one per `And`, so two never have to be told apart.
  predEq (OtherThan _) _ = False
  -- deliberately conservative, like the `Or` row above: two joined
  -- descriptions are never provably the same referent [CR#601.2c].
  predEq (Joined _ _) _ = False

  public export
  predEqAll : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k) -> Bool
  predEqAll [] [] = True
  predEqAll (x :: xs) (y :: ys) = predEq x y && predEqAll xs ys
  predEqAll _ _ = False

  public export
  negates : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  negates (Not a) b = predEq a b
  negates a (Not b) = predEq a b
  negates _ _ = False

  public export
  anyNegates : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> List (Predicate bs k) -> Bool
  anyNegates p [] = False
  anyNegates p (q :: qs) = negates p q || anyNegates p qs

  public export
  noNegatedPair : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noNegatedPair [] = True
  noNegatedPair (p :: ps) = not (anyNegates p ps) && noNegatedPair ps

  public export
  statusClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> Predicate bs k -> Bool
  statusClashOf (HasStatus v) (HasStatus w) = statusClash v w
  statusClashOf _ _ = False

  public export
  colorClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                 Predicate bs k -> Predicate bs k -> Bool
  colorClashOf IsColorless (ColorIs _) = True
  colorClashOf (ColorIs _) IsColorless = True
  colorClashOf _ _ = False

  ||| [CR#108.2b] says outright that tokens aren't cards, so a
  ||| description conjoining the two words denotes nothing.
  public export
  cardTokenClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                     Predicate bs k -> Predicate bs k -> Bool
  cardTokenClashOf IsCard IsToken = True
  cardTokenClashOf IsToken IsCard = True
  cardTokenClashOf _ _ = False

  public export
  anyCardTokenClash : {0 bs : Bindings} -> {0 k : Kind} ->
                      Predicate bs k -> List (Predicate bs k) -> Bool
  anyCardTokenClash p [] = False
  anyCardTokenClash p (q :: qs) = cardTokenClashOf p q || anyCardTokenClash p qs

  public export
  noCardTokenClash : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noCardTokenClash [] = True
  noCardTokenClash (p :: ps) = not (anyCardTokenClash p ps) && noCardTokenClash ps

  public export
  anyColorClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> List (Predicate bs k) -> Bool
  anyColorClash p [] = False
  anyColorClash p (q :: qs) = colorClashOf p q || anyColorClash p qs

  public export
  noColorClash : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noColorClash [] = True
  noColorClash (p :: ps) = not (anyColorClash p ps) && noColorClash ps

  public export
  anyStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                   Predicate bs k -> List (Predicate bs k) -> Bool
  anyStatusClash p [] = False
  anyStatusClash p (q :: qs) = statusClashOf p q || anyStatusClash p qs

  public export
  noStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  noStatusClash [] = True
  noStatusClash (p :: ps) = not (anyStatusClash p ps) && noStatusClash ps

  public export
  isPermanentHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isPermanentHead Permanent = True
  isPermanentHead _ = False

  public export
  anyPermanentHead : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Bool
  anyPermanentHead [] = False
  anyPermanentHead (p :: ps) = isPermanentHead p || anyPermanentHead ps

  public export
  anyNonPermanentTy : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  anyNonPermanentTy [] = False
  anyNonPermanentTy (p :: ps) = case seedTy p of
    Just t => not (permanentType t) || anyNonPermanentTy ps
    Nothing => anyNonPermanentTy ps

  public export
  negTypesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  negTypesOf (Not p) = case seedTy p of
    Just t => [t]
    Nothing => []
  negTypesOf _ = []

  public export
  negTypes : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List CardType
  negTypes [] = []
  negTypes (p :: ps) = negTypesOf p ++ negTypes ps

  ||| The card types a member is satisfiable under. [CR#205.3m] gives
  ||| creatures and kindreds one shared subtype list, so a creature subtype
  ||| word describes a Kindred as readily as a creature.
  public export
  seedTypeAlts : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  seedTypeAlts (HasSubtype s) =
    if subtypeType s == Creature then [Creature, Kindred] else [subtypeType s]
  seedTypeAlts p = case seedType p of
    Just t => [t]
    Nothing => []

  public export
  allNegated : List CardType -> List CardType -> Bool
  allNegated negs [] = True
  allNegated negs (t :: ts) = elem t negs && allNegated negs ts

  ||| A conjunction is empty when a negated type word rules out EVERY card
  ||| type one of its members could be satisfied under.
  public export
  anySeedEmptied : {0 bs : Bindings} -> {0 k : Kind} -> List CardType ->
                   List (Predicate bs k) -> Bool
  anySeedEmptied negs [] = False
  anySeedEmptied negs (p :: ps) = case seedTypeAlts p of
    [] => anySeedEmptied negs ps
    ts => allNegated negs ts || anySeedEmptied negs ps

  public export
  contradictionFree : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  contradictionFree ps = noNegatedPair (flattenPs ps) &&
                         not (anySeedEmptied (negTypes (flattenPs ps))
                                             (flattenPs ps)) &&
                         noStatusClash (flattenPs ps) &&
                         noColorClash (flattenPs ps) &&
                         noCardTokenClash (flattenPs ps) &&
                         not (anyPermanentHead (flattenPs ps) &&
                              anyNonPermanentTy (flattenPs ps))

  public export
  ContradictionFree : List (Predicate bs k) -> Type
  ContradictionFree {bs} {k} ps = So (contradictionFree ps)

  public export
  hasOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasOther Other = True
  hasOther (OtherThan _) = True
  hasOther (And ps) = hasOtherAny ps
  hasOther (Or _) = False
  hasOther _ = False

  public export
  hasOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasOtherAny [] = False
  hasOtherAny (p :: ps) = hasOther p || hasOtherAny ps

  ||| [CR#115.4] lists "another target" among the class words, and
  ||| [CR#601.2c] is why it is written: without it the same object may be
  ||| chosen once for each separate instance of "target". So the word has
  ||| one job — to name a target other than one already chosen — and
  ||| neither rule asks anything of the two descriptions' head nouns; any
  ||| earlier target of the kind anchors it. The
  ||| named complement ("other than this creature") still has to name
  ||| something the phrase could describe.
  public export
  otherAnchorOk : {bs : Bindings} -> (k : Kind) -> List CardType ->
                  List (Predicate bs k) -> Bool
  otherAnchorOk k ts ps =
    if atMostOne (countOthers (flattenPs ps))
      then (if hasBareOtherAny ps
              then anyTargeted k bs
              else complementAnchorsOk ts (flattenPs ps))
      else False

  public export
  anchorTyFitsSome : List CardType -> Maybe CardType -> Bool
  anchorTyFitsSome [] t = False
  anchorTyFitsSome (u :: us) t = anchorTyOk u t || anchorTyFitsSome us t

  public export
  anchorTyFits : List CardType -> Maybe CardType -> Bool
  anchorTyFits [] t = True
  anchorTyFits (u :: us) t = anchorTyFitsSome (u :: us) t

  public export
  complementAnchorsOk : {bs : Bindings} -> {k : Kind} -> List CardType ->
                        List (Predicate bs k) -> Bool
  complementAnchorsOk ts [] = True
  complementAnchorsOk ts (OtherThan n :: ps) =
    complementAnchorsOk ts ps && anchorTyFits ts (nounTy n)
  complementAnchorsOk ts (_ :: ps) = complementAnchorsOk ts ps

  public export
  OtherAnchored : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> Type
  OtherAnchored {bs} {k} ps = So (otherAnchorOk k (headTysAll ps) ps)

  public export
  isOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOther Other = True
  isOther (OtherThan _) = True
  isOther _ = False

  public export
  hasBareOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasBareOther Other = True
  hasBareOther (And ps) = hasBareOtherAny ps
  hasBareOther _ = False

  public export
  hasBareOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasBareOtherAny [] = False
  hasBareOtherAny (p :: ps) = hasBareOther p || hasBareOtherAny ps

  public export
  isSourceHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isSourceHead IsSource = True
  -- [CR#109.2] exempts a description carrying the word "card" from the
  -- battlefield default in the same breath as "source", and [CR#109.2a]
  -- fixes a zone for it only where the clause writes one -- so the word
  -- alone places nothing.
  isSourceHead IsCard = True
  isSourceHead _ = False

  public export
  anyIsSource : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsSource [] = False
  anyIsSource (p :: ps) = isSourceHead p || anyIsSource ps

  ||| [CR#120.7] makes a source the object that dealt some damage — a
  ||| position in an event rather than an object in a zone — so a phrase
  ||| headed by the source word places nothing. [CR#109.2] names "card"
  ||| beside "source" among the words that take a description off the
  ||| battlefield, and the bare card head writes no zone of its own
  ||| [CR#109.2a], so it places nothing either. Those two head words are
  ||| the whole of it; the union family's placelessness is the kind's (see
  ||| `phraseZone`).
  public export
  headIsPlaceless : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsPlaceless p = anyIsSource (flattenPs [p])

  ||| Where a description places its referent. [CR#109.2] reads a bare type
  ||| word onto the battlefield, so an object description defaults there.
  ||| A phrase whose kind reaches past objects places NOTHING: [CR#400.1]
  ||| makes a zone a place where objects can be and [CR#109.1] lists what
  ||| an object is, and a player is none of them. That single fact is what
  ||| refuses destroy, exile, tap, untap, return, counter and sacrifice
  ||| over a joined phrase, with no rule written for the purpose.
  public export
  phraseZone : {0 bs : Bindings} -> {k : Kind} -> Predicate bs k -> Maybe Zone
  phraseZone p = if headIsPlaceless p || not (kindLte k Object)
                   then Nothing
                   else Just (zoneOr Battlefield (seedZone p))

  public export
  countOthers : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Nat
  countOthers [] = Z
  countOthers (p :: ps) = if isOther p then S (countOthers ps) else countOthers ps

  public export
  atMostOne : Nat -> Bool
  atMostOne Z = True
  atMostOne (S Z) = True
  atMostOne (S (S _)) = False

  public export
  exactlyOne : Nat -> Bool
  exactlyOne Z = False
  exactlyOne (S Z) = True
  exactlyOne (S (S _)) = False

  public export
  isComparison : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isComparison (Compare _ _ _) = True
  isComparison (Superlative _ _ _) = True
  isComparison (CounterCompare _ _ _) = True
  isComparison (CompareOver _ _ _ _) = True
  isComparison _ = False

  public export
  countComparisons : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Nat
  countComparisons [] = Z
  countComparisons (p :: ps) =
    if isComparison p then S (countComparisons ps) else countComparisons ps

  public export
  headsUniform : {0 bs : Bindings} -> {0 k : Kind} ->
                 Bool -> List (Predicate bs k) -> Bool
  headsUniform b [] = True
  headsUniform b (p :: ps) = (if hasHead p then b else not b) && headsUniform b ps

  public export
  seedsUniform : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> Maybe CardType ->
                 List (Predicate bs k) -> Bool
  seedsUniform z t [] = True
  seedsUniform z t (p :: ps) = z == seedZone p &&
                               t == seedType p &&
                               seedsUniform z t ps

  ||| `seedsUniform`'s zone half alone, for the arms that answer the type
  ||| half with their own head word.
  public export
  zonesUniform : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone ->
                 List (Predicate bs k) -> Bool
  zonesUniform z [] = True
  zonesUniform z (p :: ps) = z == seedZone p && zonesUniform z ps

  ||| The arms of a disjunction have to stand in one another's place, and
  ||| the demand splits by whether they write their own head word.
  |||
  ||| The ZONE is demanded of every arm alike. It is not the head's
  ||| content: [CR#109.2a] locates a card-worded description by the zone
  ||| the phrase states, `phraseZone` defaults an unstated one to the
  ||| battlefield, and the projection is single-valued -- so arms naming
  ||| two zones would project none and be placed on the battlefield.
  ||| That is `badCrossZoneDisjunction`, a representation limit and not a
  ||| meaningless phrase.
  |||
  ||| The TYPE is demanded only of arms that write NO head. Such arms
  ||| modify one head word shared between them, and the type each
  ||| presupposes is that one head's, so a disagreement contradicts.
  ||| Where every arm writes its own head the type it presupposes IS that
  ||| head -- [CR#205.3c] correlates a subtype to its own card type -- so
  ||| the alternatives are free to name different ones. "Enchant creature
  ||| or Food" names a creature or an artifact and is one phrase all the
  ||| same; "with mana value, power, or toughness equal to the chosen
  ||| number" is headless and still refused, its arms disagreeing about
  ||| what they are said of.
  public export
  parallelDisjuncts : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  parallelDisjuncts [] = True
  parallelDisjuncts (p :: ps) =
    headsUniform (hasHead p) ps &&
    (if hasHead p then zonesUniform (seedZone p) ps
                  else seedsUniform (seedZone p) (seedType p) ps)

  public export
  ParallelDisjuncts : List (Predicate bs k) -> Type
  ParallelDisjuncts {bs} {k} ps = So (parallelDisjuncts ps)

  public export
  isOr : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOr (Or _) = True
  isOr _ = False

  public export
  anyIsOr : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsOr [] = False
  anyIsOr (p :: ps) = isOr p || anyIsOr ps

  public export
  coordinable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  coordinable p = not (hasOther p)

  public export
  coordinableAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  coordinableAll [] = True
  coordinableAll (p :: ps) = coordinable p && coordinableAll ps

  public export
  CoordinableDisjuncts : List (Predicate bs k) -> Type
  CoordinableDisjuncts {bs} {k} ps = So (coordinableAll ps)

  public export
  anyPredEq : {0 bs : Bindings} -> {0 k : Kind} ->
              Predicate bs k -> List (Predicate bs k) -> Bool
  anyPredEq p [] = False
  anyPredEq p (q :: qs) = predEq p q || anyPredEq p qs

  public export
  noRepeatedPair : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  noRepeatedPair [] = True
  noRepeatedPair (p :: ps) = not (anyPredEq p ps) && noRepeatedPair ps

  public export
  DistinctDisjuncts : List (Predicate bs k) -> Type
  DistinctDisjuncts {bs} {k} ps = So (noRepeatedPair ps)

  ||| A "non-" prefix names the complement of one modifier inside its own
  ||| kind. Refused only where the rules leave that complement empty: the
  ||| universal player word covers every person in the game [CR#102.1]; a
  ||| quality noun names its whole sort, which for colour is closed at five
  ||| [CR#105.1] — the cell over-reaches a domain-restricted quality noun,
  ||| whose complement is NOT empty; and [CR#120.7] makes a source the
  ||| object that dealt some damage, a position any object may occupy rather
  ||| than a property it lacks. Every other modifier has something outside it.
  ||| A CONJUNCTION is among those others, and deliberately so. By De
  ||| Morgan its complement is the disjunction of the conjuncts'
  ||| complements, and the rules read object properties one at a time --
  ||| [CR#205.2b] has an object satisfy the criteria for any of its card
  ||| types, and [CR#205.4c] makes every land without the supertype a
  ||| nonbasic land -- so "other than a basic land card" leaves nonbasic
  ||| cards and nonland cards behind. That is a property of the conjunction
  ||| and not of the modifiers inside it, which is why no arm-by-arm test
  ||| stands here: an arm whose own complement is empty contributes an empty
  ||| disjunct and takes nothing away from the others.
  public export
  negatable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  negatable AnyPlayer = False
  negatable ChosenPlayer = False
  negatable (QualityNoun _ _) = False
  negatable (CounterKindOn _) = False
  negatable IsSource = False
  negatable _ = True

  public export
  Negatable : Predicate bs k -> Type
  Negatable {bs} {k} p = So (negatable p)

  public export
  predSays : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predSays (HasType _) = True
  predSays (HasSubtype _) = True
  predSays AnyPlayer = True
  predSays ChosenPlayer = True
  predSays Opponent = True
  predSays (QualityNoun _ _) = True
  predSays (CounterKindOn _) = True
  predSays (OfChosen _) = True
  predSays (OfLastChosen _) = True
  predSays (OfYourChoice _ _) = True
  predSays (AbilityHead _) = True
  predSays (AbilityOf _) = True
  predSays (ActivatedBy _) = True
  predSays IsManaAbility = True
  predSays (Targets _ _) = True
  predSays IsSource = True
  predSays ManaCostHasX = True
  predSays (HasKeyword _) = True
  predSays (ControlledBy _) = True
  predSays (CastBy _) = True
  predSays (ExiledWith _) = True
  predSays Attacking = True
  predSays BeingDeclaredAttacker = True
  predSays Blocking = True
  predSays (BlockerOf _) = True
  predSays (BlockedBy _) = True
  predSays (CouldBlock _) = True
  predSays (CouldBeBlockedBy _) = True
  predSays (HappenedTo _ _ _) = True
  predSays (CastFrom _) = True
  predSays WasCast = True
  predSays (ColorIs _) = True
  predSays IsColorless = True
  predSays Multicolored = True
  predSays Monocolored = True
  predSays (ExactlyColors _) = True
  predSays (HasSupertype _) = True
  predSays (Named _) = True
  predSays (HasDesignation _) = True
  predSays (CoinCameUp _) = True
  predSays (IsAttached _) = True
  predSays (AttachedBy _ _) = True
  predSays Permanent = True
  predSays IsCard = True
  predSays IsToken = True
  predSays (HasStatus _) = True
  predSays (HasCounters _) = True
  predSays (PaidCost _) = True
  predSays (Compare _ _ _) = True
  predSays (CounterCompare _ _ _) = True
  predSays (Superlative _ _ _) = True
  predSays (CompareOver _ _ _ _) = True
  predSays (InZone _) = True
  predSays (And ps) = predSaysAny ps
  predSays (Or _) = True
  predSays (Not p) = predSays p
  predSays Other = True
  predSays (OtherThan _) = True
  predSays (Joined _ _) = True

  public export
  predSaysAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  predSaysAny [] = False
  predSaysAny (p :: ps) = predSays p || predSaysAny ps

  public export
  PredSays : Predicate bs k -> Type
  PredSays {bs} {k} p = So (predSays p)

  public export
  predNegFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predNegFree (HasType _) = True
  predNegFree (HasSubtype _) = True
  predNegFree AnyPlayer = True
  predNegFree ChosenPlayer = True
  predNegFree Opponent = True
  predNegFree (QualityNoun _ _) = True
  predNegFree (CounterKindOn _) = True
  predNegFree (OfChosen _) = True
  predNegFree (OfLastChosen _) = True
  predNegFree (OfYourChoice _ _) = True
  predNegFree (AbilityHead _) = True
  predNegFree (AbilityOf _) = True
  predNegFree (ActivatedBy _) = True
  predNegFree IsManaAbility = True
  predNegFree (Targets _ _) = True
  predNegFree IsSource = True
  predNegFree ManaCostHasX = True
  predNegFree (HasKeyword _) = True
  predNegFree (ControlledBy _) = True
  predNegFree (CastBy _) = True
  predNegFree (ExiledWith _) = True
  predNegFree Attacking = True
  predNegFree BeingDeclaredAttacker = True
  predNegFree Blocking = True
  predNegFree (BlockerOf _) = True
  predNegFree (BlockedBy _) = True
  predNegFree (CouldBlock _) = True
  predNegFree (CouldBeBlockedBy _) = True
  predNegFree (HappenedTo _ _ _) = True
  predNegFree (CastFrom _) = True
  predNegFree WasCast = True
  predNegFree (ColorIs _) = True
  predNegFree IsColorless = True
  predNegFree Multicolored = True
  predNegFree Monocolored = True
  predNegFree (ExactlyColors _) = True
  predNegFree (HasSupertype _) = True
  predNegFree (Named _) = True
  predNegFree (HasDesignation _) = True
  predNegFree (CoinCameUp _) = True
  predNegFree (IsAttached _) = True
  predNegFree (AttachedBy _ _) = True
  predNegFree Permanent = True
  predNegFree IsCard = True
  predNegFree IsToken = True
  predNegFree (HasStatus _) = True
  predNegFree (HasCounters _) = True
  predNegFree (PaidCost _) = True
  predNegFree (Compare _ _ _) = True
  predNegFree (CounterCompare _ _ _) = True
  predNegFree (Superlative _ _ _) = True
  predNegFree (CompareOver _ _ _ _) = True
  predNegFree (InZone _) = True
  predNegFree (And ps) = predNegFreeAll ps
  predNegFree (Or ps) = predNegFreeAll ps
  predNegFree (Not _) = False
  predNegFree Other = True
  predNegFree (OtherThan _) = True
  predNegFree (Joined l r) = predNegFree l && predNegFree r

  public export
  predNegFreeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  predNegFreeAll [] = True
  predNegFreeAll (p :: ps) = predNegFree p && predNegFreeAll ps

  public export
  ZoneFree : Predicate bs k -> Type
  ZoneFree {bs} {k} p = seedZone p = Nothing

  public export
  zoneOr : Zone -> Maybe Zone -> Zone
  zoneOr z Nothing = z
  zoneOr z (Just w) = w

  ||| What each half of a joined phrase carries. A joined phrase places
  ||| nothing — [CR#400.1] makes a zone a place where objects can be and
  ||| [CR#109.1] lists what an object is, so a phrase that may denote a
  ||| player names no zone — and each object half carries the head type
  ||| ITS OWN description named, which is what the demonstrative echo
  ||| reads back. The `Phrasal` and the `HeadTy` walk in lockstep, so no
  ||| half can be handed the other's type; a phrase that named one
  ||| description for the pair hands the same leaf to both.
  public export
  joinHalfPayload : {k : Kind} -> Phrasal k -> HeadTy k -> Payload k
  joinHalfPayload PhObject (SoleTy ty) = ObjectP ty Nothing Nothing Nothing
  joinHalfPayload PhPlayer _ = PlayerP
  joinHalfPayload {k = Quality q} PhQuality _ = QualityP
  joinHalfPayload PhAbility _ = AbilityP
  joinHalfPayload (PhJoin l r) (JoinTy a b) =
    JoinP (joinHalfPayload l a) (joinHalfPayload r b)
  joinHalfPayload (PhJoin l r) (SoleTy ty) =
    JoinP (joinHalfPayload l (SoleTy ty)) (joinHalfPayload r (SoleTy ty))

  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Phrasal k -> Predicate bs k -> Binding
  bindFor det plur PhObject p =
    MkBinding det Object plur
              (ObjectP (seedTy p)
                       (Just (zoneOr Battlefield (seedZone p)))
                       Nothing
                       (if seedsToken p then Just TokenOrigin else Nothing))
  bindFor det plur PhPlayer p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} PhQuality p = MkBinding det (Quality q) plur QualityP
  bindFor det plur PhAbility p = MkBinding det Ability plur AbilityP
  -- the branch that used to CHOOSE a payload: it now fills one in, since
  -- the kind fixes the shape and each half's own description fixes its
  -- type [CR#205.2a] — a class word naming no card type leaves none.
  bindFor det plur ph@(PhJoin l r) p =
    MkBinding det k plur (joinHalfPayload ph (seedTys p))

  public export
  data Noun : Bindings -> Kind -> Type where
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    AsType : (t : CardType) -> (n : Noun bs Object) ->
             (sub : Maybe Subtype) ->
             {auto 0 asc : Ascribable n} ->
             {auto 0 way : So (ascriptionOk t sub)} -> Noun bs Object
    You : Noun bs Player        -- "you" [CR#109.5]
    PlayerGroup : (w : PlayerGroupWord) -> Noun bs Player
    Each : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
           Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    Indefinite : (m : ChoiceMode bs) -> (p : Predicate bs k) ->
                 {auto ph : Phrasal k} ->
                 Noun bs k
    Definite : (p : Predicate bs k) ->
               {auto ph : Phrasal k} ->
               {auto 0 uq : Uniquifying p} -> Noun bs k
    TargetGroup : (q : Quantity bs) -> (p : Predicate bs k) ->
                  {auto tk : Targetable k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} -> Noun bs k
    ||| "[q] [description]", with the way the members are picked written
    ||| beside the count: "two cards at random", "three creatures of your
    ||| choice". The mode is `Indefinite`'s, at the counted determiner --
    ||| [CR#701.9b] marks a random or another-player's pick on the
    ||| INSTRUCTION, not on the number, so the two are orthogonal and the
    ||| slot rides the count rather than replacing it. `Nothing` where the
    ||| clause writes no mode, which is the ordinary case; a plural
    ||| `Indefinite` is not the alternative, since `nounPlur (Indefinite
    ||| _ _) = OneOf` is definitional and its delta mints at `AD OneOf`.
    ||| -- spelling: the count, the description, then the mode's own
    ||| phrase ("at random", "of your choice").
    CountedGroup : (q : Quantity bs) -> (mode : Maybe (ChoiceMode bs)) ->
                   (p : Predicate bs k) ->
                   {auto ph : Phrasal k} -> {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   Noun bs k
    AllOf : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            Noun bs k
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    ||| "you and permanents you control" [CR#109.5]: two phrases coordinated
    ||| across kinds, so the pair's kind is their join. It leaves NO joint
    ||| referent — each arm mints its own bindings and no constructor writes
    ||| a third — which is why nothing reads a mixed group back. The
    ||| asymmetry with a joined HEAD is structural, not a gate: a head is a
    ||| description, and a determiner over it goes through `bindFor`.
    Both : {ka : Kind} -> {kb : Kind} -> (l : Noun bs ka) ->
           (r : Noun (nomIntro l) kb) -> Noun bs (ka \/ kb)
    ||| "that player or that planeswalker's controller": two mentions of
    ||| the SAME kind coordinated by "or", denoting whichever of them
    ||| denotes. The fourth coordination, beside the cross-kind noun
    ||| conjunction `Both`, the cross-kind head `Joined`, and the same-kind
    ||| predicate `Or`. A union target is an object and/or a player
    ||| [CR#115.1], so a clause that names each half instead of echoing the
    ||| whole needs one arm per half, and the disjunction is what makes the
    ||| pair total again. Both arms are read in the SAME context — they are
    ||| alternatives, not a sequence — and the pair writes no joint
    ||| binding, for `Both`'s reason: no constructor builds a `Payload k`
    ||| out of an arbitrary mention.
    ||| -- spelling: the two arms joined by "or".
    EitherOf : (l : Noun bs k) -> (r : Noun bs k) ->
               {auto 0 ag : nounPlur l = nounPlur r} -> Noun bs k
    ||| "target creature and all other creatures with the same name as
    ||| that creature", "target artifact and target land", "their hand
    ||| and graveyard": two mentions of the SAME kind coordinated by
    ||| "and". The FIFTH coordination, completing the grid `EitherOf`'s
    ||| docstring names -- the cross-kind noun conjunction is `Both`, the
    ||| cross-kind head `Joined`, the same-kind predicate disjunction
    ||| `Or`, the same-kind noun disjunction `EitherOf`.
    ||| TWO MENTIONS and never one filter, which is why the right arm is
    ||| read in the left arm's discourse rather than beside it:
    ||| [CR#601.2c] says outright that where a spell "uses the word
    ||| 'target' in multiple places, the same object or player can be
    ||| chosen once for each instance", and gives "Destroy target
    ||| artifact and target land" as its own example of a spell that may
    ||| target one artifact land twice. A union head or a coordinating
    ||| description writes the word once and cannot say that.
    ||| It leaves NO joint referent, for `Both`'s reason: no constructor
    ||| builds a `Payload k` out of an arbitrary mention. The zone and
    ||| the head type project only where the arms agree, since a phrase
    ||| naming two places names no one place [CR#109.2a].
    ||| -- spelling: the two arms joined by "and".
    BothOf : (l : Noun bs k) -> (r : Noun (nomIntro l) k) -> Noun bs k
    ||| "You and target opponent EACH draw a card", "each opponent and
    ||| you each create a Treasure token": the trailing "each" over a
    ||| coordinated pair. The word IS the construction -- the effect is
    ||| taken once per arm, so a per-referent quantity is minted twice --
    ||| and it is a DISTINCT row from the joint pair rather than a
    ||| marking on it, because the two say different things about the
    ||| same coordination.
    ||| It is not `EachOf`: that one partitions a group some mention
    ||| already named (`GroupMention`), and a pair leaves no joint
    ||| referent to partition. What this distributes over is the arms
    ||| themselves.
    ||| -- spelling: the pair's own two arms, then "each".
    EachOfBoth : (pair : Noun bs k) ->
                 {auto 0 pr : CoordinatedPair pair} -> Noun bs k
    ||| "the top [amt] cards of [whose] library": the end-anchored slice
    ||| [CR#401.2] keeps in order, named by a position and a count over
    ||| one player's pile. TWO surfaces, one cell, and which word
    ||| pluralises says which: a plural COUNT pluralises the card word
    ||| over a single library ("the top three cards of your library"),
    ||| while a distributive plural POSSESSOR pluralises the zone word
    ||| over one card apiece ("the top card of their libraries"), because
    ||| [CR#400.1] gives each player their own library. `outputPlur`
    ||| takes both, so the mention binds plural under either.
    ||| -- spelling: "the top/bottom [amt] card(s) of [whose] library",
    ||| with the zone word plural where the possessor is.
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 sp : SlicePossessor whose} ->
                   Noun bs Object
    ||| The partitive: [q] members of a group an earlier phrase named,
    ||| optionally under a description of their own. TWO surfaces, one
    ||| cell -- "any number of them" writes the bare slice, and "a
    ||| creature card from among them" writes the same slice with the
    ||| members described. The preposition is the only difference: "of"
    ||| where nothing describes the slice, "from among" where something
    ||| does, because English needs the fuller phrase once a head noun
    ||| stands between the count and the pronoun. A second constructor
    ||| for the second preposition would duplicate this one's
    ||| `NonZeroQ`/`WellFormedQ`/`GroupMention` plumbing and denote the
    ||| same thing.
    ||| The description is a test on ONE member, as every `Predicate`
    ||| here is, and it names no zone of its own: [CR#109.2a] locates a
    ||| card-worded description by the zone the phrase STATES, and a
    ||| partitive states a group in that slot instead. So the slice's
    ||| zone is the group's whatever the description says, and the head
    ||| type is the description's where it names one.
    ||| -- spelling: "[q] of [grp]" bare; "[q] [description] from among
    ||| [grp]" described.
    SomeOf : (q : Quantity bs) -> (descr : Maybe (Predicate bs Object)) ->
             (grp : Noun bs Object) ->
             {auto 0 gm : GroupMention grp} ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} -> Noun bs Object
    ||| "three artifact cards with different names", "two or more
    ||| permanents with the same name as one another": a constraint on the
    ||| GROUP a counted mention picks out. Every `Predicate` in this
    ||| vocabulary tests one member, and no test on one member can say that
    ||| no two of n share a name, so the constraint rides the counted
    ||| mention and not the mention's description. [CR#201.2b] states the
    ||| negative pole in exactly that shape, over the group.
    ||| It wraps rather than binds: the wrapped mention keeps its own
    ||| count, zone, head type and bindings, so the phrase stays one
    ||| mention of one referent. The gate is the count, since a group of
    ||| one has no two members to compare [CR#201.2a].
    NamesAgree : (agr : NameAgreement) -> (grp : Noun bs Object) ->
                 {auto 0 cm : CountedMention grp} ->
                 {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
    TheRest : {auto 0 ok : So (theRestOk bs)} -> Noun bs Object
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    ||| "it", read at the carrier the CONSUMING VERB's rule admits
    ||| rather than across every singular object mention. [CR#109.2]
    ||| gives a verb slot its carrier and [CR#701.21a] lets a player
    ||| sacrifice a permanent and nothing else, so "…, sacrifice it"
    ||| after a becomes-target header has one candidate on the
    ||| battlefield even though the header announced the targeting spell
    ||| too. The gate is `It`'s, narrowed: counted uniqueness over a
    ||| smaller candidate set, never a preference among a larger one, so
    ||| two candidates sharing the slot's carrier still refuse.
    ||| The carrier is the verb's, not a word the card prints, so only a
    ||| macro writes this: an author choosing one by hand would be
    ||| choosing the verb's own rule.
    ||| -- spelling: "it", exactly as `It` spells.
    ItAt : (sl : SlotCarrier) -> {auto 0 ok : countOnesAt sl bs = 1} ->
           Noun bs Object
    ||| "it", read at the verb that STAMPED its referent rather than
    ||| across every singular object mention. Where `ItAt` narrows by the
    ||| CONSUMING verb's own rule, this narrows by the PRODUCING one: the
    ||| clause a rider or a following sentence hangs off named a keyword
    ||| action, and the pronoun names what that action acted on. "Destroy
    ||| target creature. It can't be regenerated." reads the destroyed
    ||| permanent and not the spell that destroyed it.
    ||| The label is the same one the participle read carries and buys
    ||| the same fact; what differs is the SPELLING. `TheVerbed` writes
    ||| the participle ("the destroyed creature") and so owes a
    ||| participle to write; this writes the pronoun and owes none, which
    ||| is why its second gate is only that the label is a real one.
    ||| The gate is `It`'s, narrowed: counted uniqueness over a smaller
    ||| candidate set, never a preference among a larger one, so two
    ||| mentions the same label stamped still refuse. It does not compose
    ||| with `ItAt`'s narrowing -- a read wanting both the consuming
    ||| verb's carrier and the producing verb's stamp is written by no
    ||| line and has no constructor.
    ||| -- spelling: "it", exactly as `It` spells.
    ItVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
               {auto 0 ok : countVerbedIt v bs = 1} -> Noun bs Object
    ||| "it", read at the clause that MADE its referent. `ItVerbed`
    ||| narrows by the label a keyword action left on something the text
    ||| already had; this narrows by the ORIGIN a create clause wrote on
    ||| the mention it minted, which is a fact about where the referent
    ||| came from rather than about what was done to it. "Create a Clue
    ||| token. It's an artifact with '{2}, Sacrifice this token: Draw a
    ||| card.'" reads the token and not the permanent whose ability
    ||| created it.
    ||| No label is owed, because a create clause names no keyword
    ||| action: [CR#111.1] has the effect put a token onto the
    ||| battlefield and [CR#111.2] makes the creating player its
    ||| controller, and the create clause already records that origin on
    ||| its own mention.
    ||| The gate is `It`'s, narrowed: counted uniqueness over a smaller
    ||| candidate set, never a preference among a larger one, so two
    ||| tokens two clauses made still refuse. It is the OBJECT reading of
    ||| a create clause, where `TokenAsThose` is the reading of the
    ||| DEFINITION [CR#111.3] the same clause wrote; the two are gated on
    ||| different counts for that reason and do not compose.
    ||| -- spelling: "it", exactly as `It` spells.
    ItToken : {auto 0 ok : countItToken bs = 1} -> Noun bs Object
    ||| "it" in a verb's OBJECT slot, read over the prefix that verb's own
    ||| CO-ARGUMENT did not mint. "[Another] target creature blocks IT
    ||| this turn if able" names the creature the blocker is made to
    ||| block, and the one referent it cannot name is the blocker itself:
    ||| [CR#509.1a] has the DEFENDING player choose blockers from among
    ||| the creatures they control and choose, for each, a creature to
    ||| block that is attacking that player, while [CR#508.1a] has the
    ||| ACTIVE player choose attackers from among the creatures THEY
    ||| control. The two are different players, so the blocked creature
    ||| is never the blocker. The candidate is excluded by the deed's own
    ||| rule and not by a preference among candidates, which is what the
    ||| house doctrine asks of every exclusion.
    ||| Where the three narrowings above ask a per-binding question of
    ||| the whole prefix, this one asks `It`'s own question of a NAMED
    ||| SEGMENT of it: `co` is what the co-argument announced, `rest` the
    ||| prefix the co-argument was read in, and the split is the equation
    ||| `bs = co ++ rest` rather than a position, so nothing here indexes
    ||| a slot list. `countBySplit` makes the narrowed count no larger
    ||| than the whole prefix's, and it is still a count: two candidates
    ||| in `rest` refuse exactly as two candidates in `bs` do.
    ||| The exclusion is the CONSTRUCTOR's and never the sentence's. A
    ||| description nested inside the object -- "attach to IT any number
    ||| of Auras on the battlefield" (Bruna, Light of Alabaster) -- is a
    ||| different verb's argument and excludes nothing, which is why the
    ||| segment is written rather than derived from the clause. Which
    ||| delta is the co-argument's is the owning construction's fact, so
    ||| only a macro writes this, on `ItAt`'s ground.
    ||| -- spelling: "it", exactly as `It` spells.
    ItOtherThan : (co : Bindings) -> (rest : Bindings) ->
                  {auto 0 sp : bs = co ++ rest} ->
                  {auto 0 ok : countOnes Object rest = 1} -> Noun bs Object
    ||| "it", read among the mentions the IMMEDIATELY PRECEDING member of
    ||| a coordination made. "Tap target creature an opponent controls
    ||| and put a stun counter on IT" reads the creature the tap clause
    ||| named, whatever the sentence before it announced.
    ||| `Effects` hands each member `effIntro e = effDelta e ++ bs`
    ||| [CR#608.2c] -- the instructions are followed in the order
    ||| written, and later text is read against the text before it -- so
    ||| the preceding member's own delta is a segment the CONSTRUCTION
    ||| names, not a position this read invents. That is the whole of the
    ||| exclusion: a conjunct elaborating its neighbour reads its
    ||| neighbour's mentions.
    ||| `ItOtherThan`'s twin at the other end of the same split: that one
    ||| counts the segment the co-argument did NOT mint, this one counts
    ||| the segment the preceding clause DID. Both are `It`'s gate over a
    ||| named segment and neither ranks anything -- two singular objects
    ||| in `made` refuse, and a `made` holding none refuses too, where
    ||| the bare `It` remains the spelling.
    ||| It is emphatically NOT clause recency: recency would rank the
    ||| whole prefix and take a winner, where this counts one segment and
    ||| refuses a tie. A macro writes it for a coordination's own
    ||| neighbour and for nothing else.
    ||| -- spelling: "it", exactly as `It` spells.
    ItPrior : (made : Bindings) -> (before : Bindings) ->
              {auto 0 sp : bs = made ++ before} ->
              {auto 0 ok : countOnes Object made = 1} -> Noun bs Object
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    Them : {auto 0 ok : countManys Object bs = 1} -> Noun bs Object
    ||| "them", read at the verb that STAMPED its referents rather than
    ||| across every group mention: `ItVerbed`'s plural twin, and the
    ||| same fact spelled at the same place in the sentence. A clause
    ||| that names the batch its OWN keyword action made ("Sacrifice any
    ||| number of lands. Search your library for up to that many land
    ||| cards, put THEM onto the battlefield tapped") writes this, and
    ||| the bare `Them` beside it would count two groups and refuse.
    ||| The gate is `Them`'s, narrowed exactly as the singular row
    ||| narrows `It`'s: counted uniqueness over the mentions one label
    ||| stamped, so two groups the same label stamped still refuse. The
    ||| second obligation is the singular row's too -- that the label is
    ||| a real one -- and no participle is owed, because the pronoun
    ||| spells none. It does not compose with a carrier narrowing: no
    ||| printed line wants both, and there is no constructor for it.
    ||| -- spelling: "them", exactly as `Them` spells.
    ThemVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
                 {auto 0 ok : countVerbedThem v bs = 1} -> Noun bs Object
    Those : (w : NounWord) -> {auto 0 ok : countManyWord w bs = 1} -> Noun bs (kindOfW w)
    That : (w : NounWord) -> {auto 0 ok : countWord w bs = 1} -> Noun bs (kindOfW w)
    ||| One ARM of the split read of a union mention: "that player" in
    ||| "that player or that planeswalker's controller". `EitherOf`'s two
    ||| arms are read in the same context and name halves of the SAME
    ||| union [CR#115.1], so the pair is gated on that one mention being
    ||| unique rather than on each arm's word being unique in the whole
    ||| prefix — which is what a header announcing a second player of its
    ||| own would otherwise break. The word is the half echo
    ||| `halfReaches` already defines; the whole-union read stays `That
    ||| JoinW`.
    ||| -- spelling: the half's own word, as the demonstrative spells it.
    ThatHalf : (w : NounWord) -> {auto 0 ok : countUnionHalf w bs = 1} ->
               Noun bs (kindOfW w)
    AttachHost : (w : AttachWord) -> (h : NounWord) ->
                 {auto 0 ok : AttachHeadOk w h} -> Noun bs (kindOfW h)
    TheVerbed : (v : VerbLabel) -> (w : NounWord) ->
                (marking : VerbedMarking) ->
                {auto 0 ok : countVerbed v w bs = 1} ->
                {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    ThoseVerbed : (v : VerbLabel) -> (w : NounWord) ->
                  (marking : VerbedMarking) ->
                  {auto 0 ok : countManyVerbed v w bs = 1} ->
                  {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    ControllerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player
    OwnerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player
    ||| "your commander" [CR#903.3]: the card-scope designation is an
    ||| attribute of the card itself, so the possessed noun reads it in
    ||| every zone. The possessive is the only determiner written.
    Designated : (d : Designation) -> (whose : Noun bs Player) ->
                 {auto 0 sc : designationScope d = HeldByCard} -> Noun bs Object

  ||| Referent equality between two possessor nouns, deliberately the
  ||| smallest honest relation: `True` only for the atomic words whose
  ||| referent the binding context already fixes, so two occurrences
  ||| inside one phrase provably denote the same thing. Everything else
  ||| is `False`, including two target mentions [CR#601.2c].
  public export
  nounEqRef : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Noun bs k -> Bool
  nounEqRef This This = True
  nounEqRef This _ = False
  nounEqRef (AsType _ _ _) _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
  nounEqRef (PlayerGroup v) (PlayerGroup w) = v == w
  nounEqRef (PlayerGroup _) _ = False
  nounEqRef (Each _) _ = False
  nounEqRef (Indefinite _ _) _ = False
  nounEqRef (Definite _) _ = False
  nounEqRef (TargetGroup _ _) _ = False
  nounEqRef (CountedGroup _ _ _) _ = False
  nounEqRef (AllOf _) _ = False
  nounEqRef (EachOf _) _ = False
  nounEqRef (Both _ _) _ = False
  nounEqRef (EitherOf _ _) _ = False
  nounEqRef (BothOf _ _) _ = False
  nounEqRef (EachOfBoth _) _ = False
  nounEqRef (LibrarySlice _ _ _) _ = False
  nounEqRef (SomeOf _ _ _) _ = False
  nounEqRef (NamesAgree _ _) _ = False
  nounEqRef TheRest _ = False
  nounEqRef It It = True
  nounEqRef It _ = False
  nounEqRef (ItAt _) _ = False
  nounEqRef (ItVerbed _) _ = False
  nounEqRef (ItToken) _ = False
  nounEqRef (ItOtherThan _ _) _ = False
  nounEqRef (ItPrior _ _) _ = False
  nounEqRef They They = True
  nounEqRef They _ = False
  nounEqRef Them _ = False
  nounEqRef (ThemVerbed _) _ = False
  nounEqRef (Those _) _ = False
  nounEqRef (That _) _ = False
  nounEqRef (ThatHalf _) _ = False
  nounEqRef (AttachHost _ _) _ = False
  nounEqRef (TheVerbed _ _ _) _ = False
  nounEqRef (ThoseVerbed _ _ _) _ = False
  nounEqRef (ControllerOf _) _ = False
  nounEqRef (OwnerOf _) _ = False
  nounEqRef (Designated _ _) _ = False

  ||| Destination legality for the move primitive [CR#400.3]: an owned
  ||| destination naming an arbitrary player is unwritable — a moved card
  ||| is routed to its owner's zone regardless of the sentence, so the
  ||| bare zone IS the owner-rooted destination. A library destination
  ||| always takes the position form, never the bare zone [CR#401.2].
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)
    LibraryPosOk : {auto 0 af : PlaceArrangementFits place arrg} ->
                   {auto 0 nf : PlaceOrdinalFits place offs} ->
                   DestOk (LibraryAt place arrg offs {af} {nf} Bare)

  public export
  orderOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Bool
  orderOk pl z = case zoneArrangement z of
                   Nothing => True
                   Just _ => not (isOne pl)

  public export
  ArrangementOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Type
  ArrangementOk {bs} pl z = So (orderOk pl z)

  ||| The head type a partitive's referent carries. The description
  ||| names it where the phrase writes one that names a card type ("a
  ||| creature card from among them" reads back as a creature card); a
  ||| description that names none ("a nonland card") leaves the group's
  ||| own, which is what the bare slice reads.
  public export
  sliceTy : {bs : Bindings} -> Maybe (Predicate bs Object) -> Noun bs Object ->
            Maybe CardType
  sliceTy Nothing grp = nounTy grp
  sliceTy (Just p) grp = case seedTy p of
                           Just t => Just t
                           Nothing => nounTy grp

  ||| What a partitive's description announces. A description is read
  ||| between the count and the group, so its own bindings thread there.
  public export
  sliceDelta : {bs : Bindings} -> Maybe (Predicate bs Object) -> List Binding
  sliceDelta Nothing = []
  sliceDelta (Just p) = predDelta p

  public export
  nounDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  nounDelta This = []
  nounDelta (AsType t n _) = nounDelta n
  nounDelta You = []
  nounDelta (PlayerGroup _) = []
  nounDelta (Each p {ph}) = bindFor EachD ManyOf ph p :: predDelta p
  nounDelta (Indefinite m p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (Definite p {ph}) = bindFor TheD OneOf ph p :: predDelta p
  nounDelta (TargetGroup q p {tk}) =
    bindFor TargetD (quantPlur q) (targetablePhrasal tk) p
      :: (quantDelta q ++ predDelta p)
  nounDelta (CountedGroup q _ p {ph}) =
    bindFor CountD (quantPlur q) ph p :: (quantDelta q ++ predDelta p)
  nounDelta (AllOf p {ph}) = bindFor AllD ManyOf ph p :: predDelta p
  nounDelta (EachOf grp) = nounDelta grp
  nounDelta (Both l r) = nounDelta r ++ nounDelta l
  nounDelta (EitherOf l r) = nounDelta l ++ nounDelta r
  nounDelta (BothOf l r) = nounDelta r ++ nounDelta l
  nounDelta (EachOfBoth p) = nounDelta p
  nounDelta (LibrarySlice pos amt whose) =
    MkBinding TheD Object (outputPlur (nounPlur whose) (amtPlur amt))
              (ObjectP Nothing (Just Library) Nothing Nothing)
      :: nounDelta whose
  -- the constraint is a modifier on the wrapped mention, so the mention
  -- binds once and the phrase reads back as itself.
  nounDelta (NamesAgree _ grp) = nounDelta grp
  nounDelta (SomeOf q d grp) =
    MkBinding PartD Object (quantPlur q) (ObjectP (sliceTy d grp) (nounZone grp) Nothing Nothing)
      :: (quantDelta q ++ sliceDelta d ++ nounDelta grp)
  nounDelta TheRest = []
  nounDelta It = []
  nounDelta (ItAt _) = []
  nounDelta (ItVerbed _) = []
  nounDelta ItToken = []
  nounDelta (ItOtherThan _ _) = []
  nounDelta (ItPrior _ _) = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (ThemVerbed _) = []
  nounDelta (That w) = []
  nounDelta (ThatHalf w) = []
  nounDelta (AttachHost _ _) = []
  nounDelta (Those w) = []
  nounDelta (TheVerbed v w _) = []
  nounDelta (ThoseVerbed v w _) = []
  -- a possessive already announces whatever its base announces, so a
  -- DESCRIBED base leaves the thing possessed readable ("target
  -- creature's controller … it"). A deictic base has no `nounDelta` of
  -- its own, and `selfSubjDelta`'s rows are exactly the announcement it
  -- makes instead; threading them here is what keeps the two bases
  -- parallel. The row it mints is `SelfD`, so "it" reaches the named
  -- object and the demonstrative words still do not.
  nounDelta (ControllerOf n) =
    MkBinding TheD Player OneOf PlayerP :: (selfSubjDelta n ++ nounDelta n)
  nounDelta (OwnerOf n) =
    MkBinding TheD Player OneOf PlayerP :: (selfSubjDelta n ++ nounDelta n)
  nounDelta (Designated _ _) = []

  ||| What a per-member pass hands its body: ONE member of the group,
  ||| under the group's own type, zone and stamp. The stamp comes along
  ||| because a member of a batch a label acted on is a thing that label
  ||| acted on -- "for each of those exiled creatures, …" leaves the body
  ||| able to say "the exiled creature", and to know the member was on the
  ||| battlefield, which the participle read asks of every referent it
  ||| names.
  public export
  elemIntro : {bs : Bindings} -> Noun bs Object -> Bindings
  elemIntro grp =
    MkBinding TheD Object OneOf
              (ObjectP (nounTy grp) (nounZone grp) (nounProv grp) Nothing)
      :: nomIntro grp

  ||| `elemIntro`'s twin at the VALUE: what a distributive pass over an
  ||| axis leaves its body -- the value the pass is running over, bound as
  ||| a quality of the axis's sort, over the domain's own mentions. Where
  ||| `elemIntro` hands the body a member of the group, this hands it a
  ||| label the group's members carry.
  public export
  kindValueIntro : {bs : Bindings} -> QualitySort -> Maybe (Noun bs Object) -> Bindings
  kindValueIntro q (Just dom) = qualityB q :: nomIntro dom
  kindValueIntro {bs} q Nothing = qualityB q :: bs

  ||| A pass with no domain runs over the axis's whole value set, so the
  ||| rules must CLOSE that set; a written domain supplies the values and
  ||| leaves nothing for the rule to close.
  public export
  kindDomainOk : {0 bs : Bindings} -> KindAxis -> Maybe (Noun bs Object) -> Bool
  kindDomainOk _ (Just _) = True
  kindDomainOk ax Nothing = kindAxisClosed ax

  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (AbilityOf n) = nounDelta n
  predDelta (ActivatedBy n) = nounDelta n
  predDelta (Targets m _) = nounDelta m
  predDelta (ControlledBy n) = nounDelta n
  predDelta (CastBy n) = nounDelta n
  predDelta (BlockerOf m) = nounDelta m
  predDelta (CounterKindOn n) = nounDelta n
  predDelta (BlockedBy m) = nounDelta m
  predDelta (CouldBlock m) = nounDelta m
  predDelta (CouldBeBlockedBy m) = nounDelta m
  predDelta (HappenedTo _ _ what) = complementDelta what
  predDelta (CastFrom z) = zoneDelta z
  predDelta (ColorIs _) = []
  predDelta IsColorless = []
  predDelta Multicolored = []
  predDelta Monocolored = []
  predDelta (ExactlyColors _) = []
  predDelta (HasSupertype _) = []
  predDelta (Named src) = nameSrcDelta src
  predDelta (HasDesignation _) = []
  predDelta (CoinCameUp _) = []
  predDelta (IsAttached _) = []
  predDelta (AttachedBy _ by) = nounDelta by
  predDelta (InZone z) = zoneDelta z
  predDelta (And ps) = predDeltaAll ps
  predDelta (Not p) = []
  predDelta (Or ps) = []
  predDelta (OtherThan n) = nounDelta n
  predDelta (Compare _ _ b) = amtDelta b
  predDelta (Superlative _ _ d) = predDelta d
  -- the margin the comparison names, and the domain's own mentions; the
  -- measurement is per-member, so its deltas stay inside, exactly as
  -- `AggregateOver`'s body's do.
  predDelta (CompareOver dom _ _ bound) = gapB :: (predDelta dom ++ amtDelta bound)
  predDelta (Joined l r) = predDelta l ++ predDelta r
  predDelta (CounterCompare _ _ b) = amtDelta b
  predDelta _ = []

  public export
  predDeltaAll : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> List Binding
  predDeltaAll [] = []
  predDeltaAll (p :: ps) = predDelta p ++ predDeltaAll ps

  public export
  placeDelta : {bs : Bindings} -> LibPlace bs -> List Binding
  placeDelta (OneEnd _) = []
  placeDelta (EitherEnd Nothing) = []
  placeDelta (EitherEnd (Just n)) = nounDelta n
  placeDelta Shuffled = []

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (ZoneAt z (OwnedBy n)) = nounDelta n
  zoneDelta (ZoneAt z Bare) = []
  zoneDelta (LibraryAt pl _ _ (OwnedBy n)) = placeDelta pl ++ nounDelta n
  zoneDelta (LibraryAt pl _ _ Bare) = placeDelta pl

  ||| Whether a coordinated search names more than one DISTINCT zone. A
  ||| one-zone coordination is `OneZone` written twice over, and a repeated
  ||| zone names the same pile twice.
  public export
  zonesDistinct : List Zone -> Bool
  zonesDistinct [] = True
  zonesDistinct (z :: zs) = not (elem z zs) && zonesDistinct zs

  public export
  atLeastTwoZones : List Zone -> Bool
  atLeastTwoZones zs = case zs of
    (_ :: _ :: _) => zonesDistinct zs
    _ => False

  public export
  AtLeastTwoZones : List Zone -> Type
  AtLeastTwoZones zs = So (atLeastTwoZones zs)

  ||| What a search clause names: one zone, or a coordination of them.
  ||| Each named zone is searched per [CR#701.23a].
  |||
  ||| The coordination is ORDINARY COORDINATION AT THE ZONE SORT and no
  ||| marked union row, per
  ||| `docs/decisions/kind-index-joins-union-marking-is-spelling.md` -- the
  ||| same `List (ZoneExpr bs)` the event lookback's origin payload takes,
  ||| and the reason the fixed graveyard-hand-library sweep is gone rather
  ||| than kept beside it: that was a named row for one phrasing.
  ||| It is where "and/or" earns a constructor. At an object DESCRIPTION
  ||| the word coordinates alternatives one referent may answer, which is
  ||| `Predicate.Or` and is already written ("search your library for up
  ||| to two basic land cards and/or Gate cards" is one zone and a
  ||| disjoined description); here the arms are PLACES, the clause looks
  ||| in every one of them [CR#701.23a], and the count it finds is a count
  ||| of CARDS rather than one per zone [CR#701.23d], so the list is the
  ||| whole of the construction and no per-zone quantity rides it.
  ||| "Search your graveyard, hand, and library" and "search your
  ||| graveyard, hand and/or library" are one term under two spellings for
  ||| exactly that reason.
  ||| The possessor is written ONCE over the whole coordination -- "your
  ||| graveyard, hand and/or library", "that player's graveyard, hand, and
  ||| library" -- and every printed line shares it, so it sits beside the
  ||| list and announces once. `Nothing` is the possessorless spelling; no
  ||| supported line writes one, and it is the honest shape for a zone
  ||| [CR#400.1] holds in common.
  public export
  data SearchScope : Bindings -> Type where
    OneZone : (z : ZoneExpr bs) -> SearchScope bs
    SomeZones : (whose : Maybe (Noun bs Player)) -> (zs : List Zone) ->
                {auto 0 tw : AtLeastTwoZones zs} -> SearchScope bs

  ||| A coordination fixes no single zone for what it finds.
  public export
  searchZone : {0 bs : Bindings} -> SearchScope bs -> Maybe Zone
  searchZone (OneZone z) = Just (zoneSort z)
  searchZone (SomeZones _ _) = Nothing

  public export
  searchDelta : {bs : Bindings} -> SearchScope bs -> List Binding
  searchDelta (OneZone z) = zoneDelta z
  searchDelta (SomeZones Nothing _) = []
  searchDelta (SomeZones (Just whose) _) = nounDelta whose

  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro n = nounDelta n ++ bs

  ||| The determiner a CHOSEN mention of objects carries. A choice made
  ||| as the effect applies partitions the described set -- [CR#608.2d]
  ||| has the player announce it then and refuses an illegal option, so
  ||| the members it picks come out of that set and the unchosen stay
  ||| behind -- which is the same mark "one of them" leaves over a group
  ||| the text assembled. Only the object side is marked, since only the
  ||| object side has a complement to read.
  public export
  chosenDet : {0 k : Kind} -> Phrasal k -> Determiner -> Determiner
  chosenDet PhObject _ = PartD
  chosenDet _ d = d

  ||| `bindFor` for the mention a CHOICE clause leaves. The same binding
  ||| in every respect but the player payload, which records that a
  ||| CHOICE made this mention [CR#607.2d]: the player is the one kind
  ||| whose chooser leaves a mention otherwise indistinguishable from
  ||| "you" or "each opponent", so the mark is what `countChoice` reads
  ||| there. What it mints is exactly `choiceB PlayerC`, so an
  ||| effect-level chooser and the as-enters one leave one binding
  ||| between them and a read cannot tell which position bound it.
  public export
  chosenBind : Determiner -> Plurality -> {k : Kind} ->
               Phrasal k -> Predicate bs k -> Binding
  chosenBind det plur PhPlayer p = MkBinding det Player plur ChosenPlayerP
  chosenBind det plur ph p = bindFor det plur ph p

  ||| `nounDelta` for the mention a choice clause announces. A target is
  ||| not this: it was chosen as the spell was cast [CR#601.2c], so a
  ||| choice clause naming one partitions nothing and keeps its own
  ||| determiner.
  public export
  chosenDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  chosenDelta (Indefinite m p {ph}) = chosenBind (chosenDet ph AD) OneOf ph p :: predDelta p
  chosenDelta (CountedGroup q _ p {ph}) =
    chosenBind (chosenDet ph CountD) (quantPlur q) ph p :: (quantDelta q ++ predDelta p)
  chosenDelta (NamesAgree _ grp) = chosenDelta grp
  chosenDelta n = nounDelta n

  public export
  chosenIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  chosenIntro n = chosenDelta n ++ bs

  public export
  complementDelta : {bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                    Maybe (EventComplement bs ev ks) -> List Binding
  complementDelta Nothing = []
  complementDelta (Just (Involving what)) = nounDelta what
  complementDelta (Just (FromZones src what)) =
    sourceDelta src ++ complementDelta what
  complementDelta (Just (IntoZone to what)) =
    zoneDelta to ++ complementDelta what

  public export
  zonesDelta : {bs : Bindings} -> List (ZoneExpr bs) -> List Binding
  zonesDelta [] = []
  zonesDelta (z :: zs) = zoneDelta z ++ zonesDelta zs

  ||| A colour a clause NAMES: the printed word, or "that color" reading
  ||| the one an earlier chooser bound [CR#607.2d]. Its own small
  ||| vocabulary rather than a second `Devotion` row, because the two
  ||| readings differ in the SLOT's filler and in nothing else -- the
  ||| count [CR#700.5] computes is the same count either way.
  ||| The gate is `OfChosen`'s uniqueness, not `ChoiceStands`' existence:
  ||| both carriers write the unmarked "that color" behind a single
  ||| chooser in their own ability, and no printed devotion line reads a
  ||| second chooser back.
  ||| -- spelling: the colour word; "that color".
  public export
  data ColorTerm : Bindings -> Type where
    LitColor : Chroma.Color -> ColorTerm bs
    ThatColor : {auto 0 ok : countChoice (QSort Color) bs = 1} ->
                {auto 0 rd : ChosenQualityRead Color} -> ColorTerm bs

  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PlayerStatOf : (w : PlayerStat) -> (n : Noun bs Player) ->
                   {auto 0 one : nounPlur n = OneOf} -> Amount bs
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    CountOf : {k : Kind} -> (p : Predicate bs k) ->
              Amount bs
    Aggregate : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                (p : Predicate bs k) ->
                {auto 0 sc : projScope ax = k} ->
                Amount bs
    CountersOn : {k : Kind} -> (kind : CounterKind) -> (holder : Noun bs k) ->
                 {auto 0 sc : counterScope kind = k} ->
                 {auto 0 one : nounPlur holder = OneOf} -> Amount bs
    ||| "the number of times it was kicked", "for each time it was
    ||| kicked": how many times one named optional cost was paid for the
    ||| object as it was cast. The count-valued twin of `PaidCost`, on
    ||| `CountersOn`'s model -- payment is state on the object, and the
    ||| state a payment leaves is a number. [CR#702.33d] is what makes
    ||| that number bigger than one: a spell with two kicker costs or
    ||| with multikicker "may be kicked multiple times". So repeatability
    ||| is a fact of the COST the card declared and no flag anywhere; the
    ||| count reads at every paid-cost name, and one that was offered
    ||| once reads 0 or 1 -- tolerated, as `GreatestStoredMatch` tolerates
    ||| a permanent that stored nothing. The holder is singular because
    ||| [CR#118.10] applies each payment to one spell or ability and no
    ||| rule sums two objects' payments.
    ||| -- spelling: "the number of times [n] was kicked", "for each time
    ||| [n] was kicked".
    TimesPaid : (which : PaidCostName) -> (whose : Noun bs Object) ->
                {auto 0 nc : PaidCostNamed which} ->
                {auto 0 one : nounPlur whose = OneOf} -> Amount bs
    EventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
                 (w : Lookback) ->
                 (what :
                    Maybe (EventComplement (nomIntro who) ev k)) ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
    Times : (per : Nat) -> (a : Amount bs) ->
            {auto 0 nz : IsSucc per} -> Amount bs
    ||| "that much": the quantity an earlier clause's outcome wrote. The
    ||| gate counts the outcome mentions that CARRY a number
    ||| (`outcomeIsQuantity`) rather than every outcome mention, because a
    ||| coin flip leaves one that carries none [CR#705.2].
    ThatMuch : {auto 0 ok : countQuantOutcomes bs = 1} -> Amount bs
    ||| "the chosen number", "the last chosen number": a chosen number
    ||| read where the sentence wants an AMOUNT. It is the read
    ||| `chosenQualityReadOk Number = False` refuses at the other seat and
    ||| the reason the two come apart is that rule's own: a number is not
    ||| one of [CR#109.3]'s characteristics, so nothing on an object
    ||| matches it, while an amount slot asks for a number and nothing
    ||| else.
    ||| ONE row for both spellings, because [CR#607.2d] writes one
    ||| linkage over "the chosen [value]", "the last chosen [value]," or
    ||| similar: which word a card prints follows from how many times its
    ||| chooser may fire, and no rule tells the two readings apart.
    ||| The gate is `ChoiceStands`' existence rather than uniqueness for
    ||| that same reason -- Shapeshifter writes two choosers and reads
    ||| them with one phrase -- and it carries the marked read's recorded
    ||| repeatability overgeneration unchanged.
    ||| -- spelling: "the chosen number" behind a single chooser, "the
    ||| last chosen number" behind a repeatable one.
    ChosenNumber : {auto 0 ok : ChoiceStands (countChoice (QSort Number) bs)} ->
                   Amount bs
    PreventedThisWay : {auto 0 ok : countOutcomes DamagePrevented bs = 1} ->
                       Amount bs
    ||| "the result": the number on the die a clause rolled. [CR#706.2]
    ||| names it -- the natural result once every modifier has been
    ||| applied -- so the read is sorted to the roll rather than left to
    ||| `ThatMuch`, exactly as `PreventedThisWay` is sorted to prevention.
    ||| -- spelling: "equal to the result", "where X is the result"
    TheResult : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
    ||| "the total of those results", "If you rolled 7": the sum of the
    ||| results of the dice one clause rolled. [CR#706.2] defines a
    ||| result per die and stops there, so the total is a second read of
    ||| the same roll and not a spelling of `TheResult`; nothing in the
    ||| rules makes it undefined, and the clause that rolled several dice
    ||| is what it sums. Gated on the roll like `TheResult`, and a total
    ||| over a single die -- equal to that die's result -- is tolerated.
    ||| -- spelling: "the total of those results"; on a comparison's
    ||| left, "If you rolled [n]".
    TheTotal : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
    ||| "the number of coins that came up heads", "for each coin that
    ||| comes up heads": how many of the flipped coins show a face. The
    ||| plural twin of the `FlipFace` condition, which reads one coin --
    ||| [CR#705.2]'s uncalled reading, the one that has a face and no
    ||| winner -- and it takes no subject for the same reason. The gate
    ||| is `FlipFace`'s existence test, not a count, because "Flip five
    ||| coins" leaves one mention for any number of coins; a count over a
    ||| single flip is 0 or 1 and is tolerated.
    CoinsShowing : (face : CoinFace) ->
                   {auto 0 fl : So (coinFlipInScope bs)} -> Amount bs
    ||| "the greatest number of stored results on it of the same value":
    ||| the largest group of a permanent's stored results sharing one
    ||| value. [CR#706.8a] is what gives the phrase its two parts -- a
    ||| stored result is noted information on a permanent, and "the
    ||| result is the `value` of that stored result" -- so the read
    ||| groups by that value and reports the biggest group's size. One
    ||| row and not an `Aggregate`: the domain is noted numbers rather
    ||| than a described set, so no `ProjAxis` names it and no predicate
    ||| ranges over it. The holder is singular because a stored result is
    ||| stored ON a permanent, and no rule sums two permanents' notes.
    ||| The read presupposes nothing beyond the holder: [CR#706.8c] links
    ||| the storing ability to the reading one, so a permanent whose text
    ||| never stored anything is that link's business, not a gate here,
    ||| and a permanent with no stored results reads zero.
    ||| -- spelling: "the greatest number of stored results on [n] of the
    ||| same value".
    GreatestStoredMatch : (n : Noun bs Object) ->
                          {auto 0 one : nounPlur n = OneOf} -> Amount bs
    GroupSize : {auto 0 ok : countManysAny bs = 1} -> Amount bs
    TheDifference : {auto 0 ok : countOnes Gap bs = 1} -> Amount bs
    ||| The letter, wherever the text writes it. It INTRODUCES the letter
    ||| when no earlier mention -- cost or text -- put one in the prefix,
    ||| and reads it otherwise; cost X and text X are one variable
    ||| [CR#107.3i]. Ungated, and no discharge gate stands at the ability
    ||| boundary either: the rules give every X a value. An ability may
    ||| define it [CR#107.3c]; a cost announces it [CR#107.3a]; failing
    ||| both, its controller chooses it [CR#107.3]; and in a gained
    ||| ability that defines none it is 0 [CR#107.3j]. A gate would refuse
    ||| those last two, which are rules-meaningful, so an undefined letter
    ||| leaving an ability is recorded and not refused.
    LetterVal : (l : Letter) -> Amount bs
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    Minus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    ||| "your devotion to [color]" / "… to [color] and [color]": the count
    ||| of mana symbols of the named colour(s) among the mana costs of
    ||| permanents the player controls [CR#700.5]. Its own row, never
    ||| `CountOf`: the domain is symbols inside costs, not a set of
    ||| objects. The pair slot is UNGATED: [CR#700.5] computes a pair over
    ||| the symbols that are "[color 1], [color 2], or both colors", which
    ||| is well defined when the two names coincide (it is then the
    ||| single-colour count), so no rule refuses the repeated colour.
    ||| -- spelling: "[whose] devotion to [color]"; with the second colour,
    ||| "[whose] devotion to [color] and [color]".
    Devotion : (who : Noun bs Player) -> (c : ColorTerm bs) ->
               (d : Maybe (ColorTerm bs)) ->
               {auto 0 one : nounPlur who = OneOf} -> Amount bs
    ||| "half [amt], rounded down/up": the halving read the corpus writes,
    ||| generalising `RoundMode`'s one existing site (`DamageScale.Halved`).
    ||| The mode is a required slot: oracle text always writes it.
    ||| -- spelling: "half [amt], rounded down" / "…, rounded up".
    Half : (r : RoundMode) -> (a : Amount bs) -> Amount bs
    ||| "the difference between [a] and [b]": the SYMMETRIC margin of two
    ||| written amounts — plain English's absolute difference, so the
    ||| directional floored `Minus` stays untouched beside it.
    ||| -- spelling: "the difference between [a] and [b]".
    DifferenceBetween : (a : Amount bs) -> (b : Amount (amtIntro a)) ->
                        Amount bs
    ||| "the amount of damage dealt to you this turn", "the amount of life
    ||| you gained this turn": `EventCount`'s numeric twin — the SUM of a
    ||| magnitude-bearing event's amounts over the lookback window, gated
    ||| to the events that happen in an amount (`eventHasMagnitude`).
    ||| -- spelling: "the amount of [event phrase] [window]", "the total
    ||| amount of …".
    EventSum : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               (what :
                  Maybe (EventComplement (nomIntro who) ev k)) ->
               {auto 0 cw : ComplementWritten what} ->
               {auto 0 sb : LookbackSubject ev k} ->
               {auto 0 qm : So (eventHasMagnitude ev)} -> Amount bs
    ||| "the total power of the sacrificed creatures", "the greatest power
    ||| among them", "their total toughness": the fold whose complement is
    ||| a group MENTION rather than a description — the plural twin of
    ||| `StatOf`, whose gate takes one referent. A second slot shape, no
    ||| new fold machinery.
    ||| -- spelling: with `SumOf`, "the total [axis] of [grp]" and the
    ||| possessive "[grp]'s total [axis]"; with an extremal op, "the
    ||| greatest/least [axis] among [grp]".
    AggregateOf : (op : AggregateOp) -> (ax : ProjAxis) ->
                  {k : Kind} -> (grp : Noun bs k) ->
                  {auto 0 sc : projScope ax = k} ->
                  {auto 0 pl : nounPlur grp = ManyOf} -> Amount bs
    ||| "the greatest number of creatures a player controls": the fold
    ||| whose per-element read is RELATIVIZED to the member — the element
    ||| binder both prior arts carry (core's `Projection` over a bound
    ||| `It`; the legacy module's `Project`/`bindIt`). The domain binds one
    ||| member (`TheD`, `OneOf`) for the body to read back as `It`/`They`.
    ||| The body's own phrase deltas are NOT exported: a per-member phrase
    ||| has no single announcement to make, so a target written inside the
    ||| body is dropped — tolerated overgeneration, noted here.
    ||| -- spelling: "the greatest/least [body] [domain relative clause]",
    ||| e.g. "the greatest number of creatures a player controls".
    AggregateOver : {k : Kind} -> (op : AggregateOp) ->
                    (dom : Predicate bs k) -> {auto ph : Phrasal k} ->
                    (body : Amount (bindFor TheD OneOf ph dom
                                      :: (predDelta dom ++ bs))) ->
                    Amount bs
    ||| "the number of creatures destroyed this way", "the number of
    ||| them": the count whose domain is a group MENTION rather than a
    ||| description -- `CountOf`'s mention twin, as `AggregateOf` is
    ||| `Aggregate`'s. It READS the group an earlier clause left, where
    ||| `GroupSize` re-mentions a quantity the text already stated: that
    ||| one spells "that many" and finds the unique plural mention, this
    ||| one names its group and may stand as a comparison's subject.
    ||| -- spelling: "the number of [grp]".
    CountOfGroup : {k : Kind} -> (grp : Noun bs k) ->
                   {auto 0 pl : nounPlur grp = ManyOf} ->
                   {auto 0 gm : GroupMention grp} -> Amount bs
    ||| "the number of card types among cards in all graveyards", "the
    ||| number of basic land types among lands you control": the count of
    ||| the DISTINCT values an axis takes over a domain, where `CountOf`
    ||| counts the domain's own members. The two readings come apart
    ||| because one object carries more than one card type [CR#205.2b] and
    ||| more than one colour [CR#105.2] -- Tarmogoyf counts card types,
    ||| not cards.
    ||| Its own head, not an `AggregateOp` row: [CR#205.2a]'s types and
    ||| [CR#105.1]'s colours are not numbers, so no existing op folds them
    ||| and no existing axis answers a distinct count. `KindAxis` names
    ||| the label set for the same reason `ProjAxis` cannot -- a `ProjAxis`
    ||| projects one number per object.
    ||| The domain is a `Noun`, which reaches the description ("among
    ||| cards in all graveyards", an `AllOf`) and the mention ("among
    ||| those creatures") in one slot; no description/mention twin is
    ||| minted, since one surface writes both.
    ||| UNGATED on plurality: a single object still has a set of types
    ||| [CR#205.2b] and of colours [CR#105.2], and the corpus writes that
    ||| reading with its own words ("the number of colors that spell is").
    ||| -- spelling: "the number of [axis] among [domain]", with
    ||| "different" before an axis a sum could otherwise be read over;
    ||| over one object, "the number of [axis] [domain] is".
    DistinctCount : (ax : KindAxis) -> (dom : Noun bs Object) -> Amount bs
    ||| "up to [bound]": the number the acting player announces as the
    ||| effect applies, no greater than the written bound. The count
    ||| slot's twin of `Quantity`'s `UpToOf`, which ceilings a chosen SET
    ||| where this one ceilings a number: "draw up to three cards" picks
    ||| no cards to draw, only how many.
    ||| [CR#608.2d] is the announcement: an effect's own choices are made
    ||| while applying the effect, and an illegal option cannot be chosen.
    ||| No chooser slot is minted -- [CR#121.2b] and [CR#121.3] both put
    ||| the ceiling draw's choice with the drawing player ("the affected
    ||| player", "that player") -- so the chooser is whichever player the
    ||| clause names as acting. A `may` above it offers the action; this
    ||| row offers the number.
    ||| The bound is any amount, ungated: "up to X" prints, and no rule
    ||| refuses a bound the game state supplies. A ceiling inside a
    ||| ceiling is unwritten English no rule refuses; tolerated, at its
    ||| zero.
    ||| -- spelling: "up to [bound]".
    UpTo : (bound : Amount bs) -> Amount bs

  public export
  amtDelta : {bs : Bindings} -> Amount bs -> List Binding
  amtDelta (Lit _) = []
  amtDelta (StatOf _ nom) = nounDelta nom
  amtDelta (PlayerStatOf _ nom) = nounDelta nom
  amtDelta (CountersOn _ holder) = nounDelta holder
  amtDelta (TimesPaid _ whose) = nounDelta whose
  amtDelta (EventCount _ who _ what) = nounDelta who ++ complementDelta what
  amtDelta (CountOf p) = predDelta p
  amtDelta (Aggregate _ _ p) = predDelta p
  amtDelta (Times _ a) = amtDelta a
  amtDelta ThatMuch = []
  amtDelta ChosenNumber = []
  amtDelta PreventedThisWay = []
  amtDelta TheResult = []
  amtDelta TheTotal = []
  amtDelta (CoinsShowing _) = []
  amtDelta (GreatestStoredMatch _) = []
  amtDelta GroupSize = []
  amtDelta TheDifference = []
  amtDelta (LetterVal l) = letterDelta l bs
  amtDelta (Plus a b) = amtDelta a ++ amtDelta b
  amtDelta (Minus a b) = amtDelta a ++ amtDelta b
  amtDelta (Devotion who _ _) = nounDelta who
  amtDelta (Half _ a) = amtDelta a
  amtDelta (DifferenceBetween a b) = amtDelta a ++ amtDelta b
  amtDelta (EventSum _ who _ what) = nounDelta who ++ complementDelta what
  amtDelta (AggregateOf _ _ grp) = nounDelta grp
  amtDelta (AggregateOver _ dom _) = predDelta dom
  amtDelta (CountOfGroup grp) = nounDelta grp
  amtDelta (DistinctCount _ dom) = nounDelta dom
  amtDelta (UpTo b) = amtDelta b

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (PlayerStatOf w nom) = nomIntro nom
  amtIntro (CountersOn _ holder) = nomIntro holder
  amtIntro (TimesPaid _ whose) = nomIntro whose
  amtIntro (EventCount _ who _ what) = complementDelta what ++ nomIntro who
  amtIntro (CountOf p) = predDelta p ++ bs
  amtIntro (Aggregate _ _ p) = predDelta p ++ bs
  amtIntro (Times per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro ChosenNumber = bs
  amtIntro PreventedThisWay = bs
  amtIntro TheResult = bs
  amtIntro TheTotal = bs
  amtIntro (CoinsShowing _) = bs
  amtIntro (GreatestStoredMatch _) = bs
  amtIntro GroupSize = bs
  amtIntro TheDifference = bs
  amtIntro (LetterVal l) = letterDelta l bs ++ bs
  amtIntro (Plus a b) = amtIntro b
  amtIntro (Minus a b) = amtIntro b
  amtIntro (Devotion who _ _) = nomIntro who
  amtIntro (Half _ a) = amtIntro a
  amtIntro (DifferenceBetween a b) = amtIntro b
  amtIntro (EventSum _ who _ what) = complementDelta what ++ nomIntro who
  amtIntro (AggregateOf _ _ grp) = nomIntro grp
  amtIntro (AggregateOver _ dom _) = predDelta dom ++ bs
  amtIntro (CountOfGroup grp) = nomIntro grp
  amtIntro (DistinctCount _ dom) = nomIntro dom
  amtIntro (UpTo b) = amtIntro b

  ||| An unwritten amount introduces nothing: the slot's absence is the
  ||| bare "all" spelling, not a mention a later clause could read.
  public export
  optAmtIntro : {bs : Bindings} -> Maybe (Amount bs) -> Bindings
  optAmtIntro Nothing = bs
  optAmtIntro (Just a) = amtIntro a

  public export
  amtPlur : {0 bs : Bindings} -> Amount bs -> Plurality
  amtPlur (Lit (S Z)) = OneOf
  amtPlur (Lit _) = ManyOf
  amtPlur (StatOf _ _) = ManyOf
  amtPlur (PlayerStatOf _ _) = ManyOf
  amtPlur (CountersOn _ _) = ManyOf
  amtPlur (TimesPaid _ _) = ManyOf
  amtPlur (EventCount _ _ _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Aggregate _ _ _) = ManyOf
  amtPlur (Times _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur ChosenNumber = ManyOf
  amtPlur PreventedThisWay = ManyOf
  amtPlur TheResult = ManyOf
  amtPlur TheTotal = ManyOf
  amtPlur (CoinsShowing _) = ManyOf
  amtPlur (GreatestStoredMatch _) = ManyOf
  amtPlur GroupSize = ManyOf
  amtPlur TheDifference = ManyOf
  amtPlur (LetterVal _) = ManyOf
  amtPlur (Plus _ _) = ManyOf
  amtPlur (Minus _ _) = ManyOf
  amtPlur (Devotion _ _ _) = ManyOf
  amtPlur (Half _ _) = ManyOf
  amtPlur (DifferenceBetween _ _) = ManyOf
  amtPlur (EventSum _ _ _ _) = ManyOf
  amtPlur (AggregateOf _ _ _) = ManyOf
  amtPlur (AggregateOver _ _ _) = ManyOf
  amtPlur (CountOfGroup _) = ManyOf
  amtPlur (DistinctCount _ _) = ManyOf
  -- Agreement follows the written bound: "up to three cards", "up to one card".
  amtPlur (UpTo b) = amtPlur b

  public export
  writtenBound : {0 bs : Bindings} -> Amount bs -> Bool
  writtenBound (Lit _) = True
  writtenBound (StatOf _ _) = False
  writtenBound (PlayerStatOf _ _) = False
  writtenBound (CountersOn _ _) = False
  writtenBound (TimesPaid _ _) = False
  writtenBound (EventCount _ _ _ _) = False
  writtenBound (CountOf _) = False
  writtenBound (Aggregate _ _ _) = False
  writtenBound (Times _ _) = False
  writtenBound ThatMuch = False
  writtenBound ChosenNumber = False
  writtenBound PreventedThisWay = False
  writtenBound TheResult = False
  writtenBound TheTotal = False
  writtenBound (CoinsShowing _) = False
  writtenBound (GreatestStoredMatch _) = False
  writtenBound GroupSize = False
  writtenBound TheDifference = False
  writtenBound (LetterVal _) = True
  writtenBound (Plus _ _) = False
  writtenBound (Minus _ _) = False
  writtenBound (Devotion _ _ _) = False
  writtenBound (Half _ _) = False
  writtenBound (DifferenceBetween _ _) = False
  writtenBound (EventSum _ _ _ _) = False
  writtenBound (AggregateOf _ _ _) = False
  writtenBound (AggregateOver _ _ _) = False
  writtenBound (CountOfGroup _) = False
  writtenBound (DistinctCount _ _) = False
  -- The bound is written; the number the ceiling stands for is not.
  writtenBound (UpTo _) = False

  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq (LetterVal a) (LetterVal b) = a == b
  boundEq _ _ = False

  public export
  data LifeOp : Bindings -> Type where
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
    Down : Amount bs -> LifeOp bs   -- "loses [amt] life"
    Set : Amount bs -> LifeOp bs    -- "[whose] life total becomes [amt]"

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a
  lifeIntro (Set a) = amtIntro a

  ||| How many coins a flip instruction calls for: a written number, or
  ||| one coin per member of a described set. [CR#705.1] makes a coin a
  ||| physical randomiser that belongs to no referent, so a flip made FOR
  ||| a thing is not a flip made BY it -- and [CR#705.2] gives the flip to
  ||| "the player who flips the coin", which is the instruction's subject
  ||| in both arms. That is why the per-member arm is a second slot and
  ||| never a plural subject: Warp Vortex's "flip a coin for each opponent
  ||| you have" then reads "for each flip YOU win".
  ||| Kind-polymorphic under the coarse join `CoinCameUp` uses, because
  ||| both kinds are printed ("for each creature", "for each opponent");
  ||| plural, because one member is `FlipCount (Lit 1)` written out.
  ||| -- spelling: with `FlipCount`, "[n] coin(s)"; with `FlipPer`,
  ||| "a coin for each [each]".
  public export
  data FlipScope : Bindings -> Type where
    FlipCount : (n : Amount bs) -> FlipScope bs
    FlipPer : {k : Kind} -> (each : Noun bs k) ->
              {auto 0 pl : nounPlur each = ManyOf} ->
              {auto 0 rk : So (kindLte k (Object \/ Player))} -> FlipScope bs

  public export
  flipScopeIntro : {bs : Bindings} -> FlipScope bs -> Bindings
  flipScopeIntro (FlipCount n) = amtIntro n
  flipScopeIntro (FlipPer each) = nomIntro each

  ||| Which of a clause's randomised outcomes an ignore instruction sets
  ||| aside [CR#706.6]. The two superlative arms are printed over rolls
  ||| alone -- "ignore the lower roll" against "ignore all but the
  ||| highest roll" -- and neither is a spelling of the other, since a
  ||| clause that rolled more than two dice keeps a different number
  ||| under each.
  ||| `IgnoreChosen` is the third form, and it names no end at all:
  ||| "ignore one", "you choose one of those rolls to ignore". The count
  ||| is an `Amount` on `FlipCount`'s model: [CR#706.6] states the word
  ||| one roll at a time and puts no bound on how many an instruction may
  ||| name. Every printed line writes the literal one, so anything else
  ||| is overgeneration, named here at its zero.
  ||| The chooser is written only on this arm. [CR#706.6] hands the
  ||| choice to the rolling player of its own accord, as the tie-break
  ||| under "the lowest roll", so the superlative arms need no slot; the
  ||| chosen arm gets one because a printed line puts the choice
  ||| somewhere else -- Bamboozling Beeble replaces TARGET player's roll
  ||| and then has YOU choose. Bindingless on `EitherEnd`'s model: the
  ||| chooser names a player the sentence already has, and mints none.
  ||| -- spelling: with `IgnoreChosen n Nothing`, "ignore [n]"; with a
  ||| chooser, "[who] choose(s) [n] of those rolls to ignore".
  public export
  data IgnoredOutcomes : Bindings -> Type where
    IgnoreExtreme : RollExtreme -> IgnoredOutcomes bs
    IgnoreAllBut : RollExtreme -> IgnoredOutcomes bs
    IgnoreChosen : (chooser : Maybe (Noun bs Player)) -> (n : Amount bs) ->
                   {auto 0 ag : EventAgent chooser} -> IgnoredOutcomes bs

  public export
  ignoredOutcomesIntro : {bs : Bindings} -> IgnoredOutcomes bs -> Bindings
  ignoredOutcomesIntro (IgnoreExtreme _) = bs
  ignoredOutcomesIntro (IgnoreAllBut _) = bs
  -- the chooser is bindingless, so the count is all this arm announces.
  ignoredOutcomesIntro (IgnoreChosen _ n) = amtIntro n

  ||| What each arm needs the clause to have randomised. The chosen arm
  ||| takes any of the three ("ignore one" is printed over a roll, a coin
  ||| and the planar die alike); the two superlative arms take a roll and
  ||| only a roll. [CR#706.6] writes "the lowest roll" over results, and
  ||| [CR#706.2] makes a result a NUMBER, so the ends `RollExtreme` names
  ||| are the ends of an order. A coin has no such order -- [CR#705.1]
  ||| gives it two faces and ranks neither -- and neither do
  ||| [CR#901.3a]'s planar faces, so there is no lowest one to set aside.
  public export
  ignorableFor : {bs : Bindings} -> IgnoredOutcomes bs -> Bool
  ignorableFor (IgnoreExtreme _) = countOutcomes RollResult bs == 1
  ignorableFor (IgnoreAllBut _) = countOutcomes RollResult bs == 1
  ignorableFor (IgnoreChosen _ _) = ignorableInScope bs

  public export
  readAmount : {0 bs : Bindings} -> Amount bs -> Bool
  readAmount (Lit _) = False
  readAmount (StatOf _ _) = True
  readAmount (PlayerStatOf _ _) = True
  readAmount (CountersOn _ _) = True
  readAmount (TimesPaid _ _) = True
  readAmount (EventCount _ _ _ _) = True
  readAmount (CountOf _) = True
  readAmount (Aggregate _ _ _) = True
  readAmount (Times _ _) = False
  readAmount ThatMuch = False
  -- Announced under [CR#608.2d], as `UpTo` is; no game state carries it.
  readAmount ChosenNumber = False
  readAmount PreventedThisWay = False
  -- the die's number is a read of the roll, not a re-mention of a
  -- quantity the text already stated, so a comparison may take it as
  -- its subject: "If the result is 0 or less, …" [CR#706.2].
  readAmount TheResult = True
  -- the total reads the dice the same way, and printed text puts it on
  -- a comparison's left: "If you rolled 7, \u2026".
  readAmount TheTotal = True
  -- how the coins came up is a fact about the flip, read like any other
  -- count of what happened (`EventCount`).
  readAmount (CoinsShowing _) = True
  readAmount (GreatestStoredMatch _) = True
  readAmount GroupSize = False
  readAmount TheDifference = True
  -- the announced letter READS game state — the value its announcement
  -- fixed [CR#107.3a] — so "If X is 1" measures a fact, where a bare
  -- numeral on the left states arithmetic (badCompareLiteralSubject).
  readAmount (LetterVal _) = True
  readAmount (Plus _ _) = False
  readAmount (Minus _ _) = False
  readAmount (Devotion _ _ _) = True
  readAmount (Half _ _) = False
  readAmount (DifferenceBetween _ _) = False
  readAmount (EventSum _ _ _ _) = True
  readAmount (AggregateOf _ _ _) = True
  readAmount (AggregateOver _ _ _) = True
  readAmount (CountOfGroup _) = True
  readAmount (DistinctCount _ _) = True
  -- Announced under [CR#608.2d], not read off the game state.
  readAmount (UpTo _) = False

  public export
  ReadAmount : Amount bs -> Type
  ReadAmount {bs} a = So (readAmount a)

  ||| How many of a described set a phrase picks out. Moved here from the
  ||| word catalog the day a bound became able to carry a written AMOUNT:
  ||| "up to X target creatures" prints, so the ceiling column is open to
  ||| the amount vocabulary, and that vocabulary lives in this mutual
  ||| block. The literal gates below keep their literal rows — an amount
  ||| ceiling is statically no bound at all, so each gate answers it
  ||| whole-constructor rather than becoming runtime-undecidable.
  public export
  data Quantity : Bindings -> Type where
    Range : Maybe Nat -> Maybe Nat -> Quantity bs
    ||| "up to [amt]": the ceiling that is a written amount. Only the
    ||| ceiling arm is minted — the printed forms are ceilings — and a
    ||| written floor or exact amount waits on a measured line.
    ||| -- spelling: "up to [amt]".
    UpToOf : (a : Amount bs) -> Quantity bs
    ||| "a number of [description] equal to [a]": the EXACT count that is
    ||| a written amount, the arm `UpToOf`'s own note said would wait on a
    ||| measured line. Boreas Charger's "a number of Plains cards equal to
    ||| the difference" and Ondu Giant's kin write it; the ceiling reading
    ||| of the same shape ("a number of basic land cards LESS THAN OR
    ||| EQUAL TO the difference") is `UpToOf` and not this.
    ||| -- spelling: "a number of [description] equal to [a]".
    ExactlyOf : (a : Amount bs) -> Quantity bs

  public export
  data NonZeroQ : Quantity bs -> Type where
    UnboundedAbove : NonZeroQ (Range lo Nothing)
    MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))
    -- "up to X" admits X = 0 at resolution and still permits one when
    -- X is positive; the statically-zero ceiling the literal arm refuses
    -- (badZeroGroup) cannot be written here.
    AmountCeiling : NonZeroQ (UpToOf a)
    -- an exact amount may resolve to zero and still permits one when the
    -- amount is positive, exactly as the ceiling arm does.
    AmountExact : NonZeroQ (ExactlyOf a)

  public export
  ||| A range whose floor is above its ceiling picks out nothing;
  ||| [CR#107.1c] otherwise leaves the vocabulary open, and a written zero
  ||| floor is a second spelling of "any number of".
  quantWellFormed : {0 bs : Bindings} -> Quantity bs -> Bool
  quantWellFormed (Range Nothing _) = True
  quantWellFormed (Range (Just _) Nothing) = True
  quantWellFormed (Range (Just lo) (Just hi)) = lte lo hi
  quantWellFormed (UpToOf _) = True
  quantWellFormed (ExactlyOf _) = True

  public export
  WellFormedQ : Quantity bs -> Type
  WellFormedQ q = So (quantWellFormed q)

  public export
  quantPlur : {0 bs : Bindings} -> Quantity bs -> Plurality
  quantPlur (Range _ (Just (S Z))) = OneOf
  quantPlur (Range _ _) = ManyOf
  -- "up to X creatures" is written plural whatever X resolves to.
  quantPlur (UpToOf _) = ManyOf
  -- "a number of Plains cards equal to the difference" is written plural
  -- whatever the amount resolves to, as the ceiling arm is.
  quantPlur (ExactlyOf _) = ManyOf

  public export
  ||| [CR#700.2]: a mode is chosen from the list printed on the card, so a
  ||| headcount reaching past the list names modes that are not there. An
  ||| amount headcount is not statically past any list, so it is admitted.
  modesFit : {0 bs : Bindings} -> Quantity bs -> Nat -> Bool
  modesFit (Range Nothing Nothing) n = True
  modesFit (Range Nothing (Just hi)) n = lte hi n
  modesFit (Range (Just lo) Nothing) n = lte lo n
  modesFit (Range (Just lo) (Just hi)) n = lte hi n
  modesFit (UpToOf _) n = True
  modesFit (ExactlyOf _) n = True

  public export
  ModesFit : Quantity bs -> Nat -> Type
  ModesFit q n = So (modesFit q n)

  ||| [CR#706.3a] gives a results table's left column three forms — a
  ||| single number, "N1—N2", "N+" — all numbers, so a table row's range
  ||| is literal by rule.
  public export
  quantLiteral : {0 bs : Bindings} -> Quantity bs -> Bool
  quantLiteral (Range _ _) = True
  quantLiteral (UpToOf _) = False
  quantLiteral (ExactlyOf _) = False

  ||| What a quantity's bound mentions: nothing for the literal ranges,
  ||| the amount's own delta for the amount ceiling — so "up to X target
  ||| creatures" introduces its letter like any other written X.
  public export
  quantDelta : {bs : Bindings} -> Quantity bs -> List Binding
  quantDelta (Range _ _) = []
  quantDelta (UpToOf a) = amtDelta a
  quantDelta (ExactlyOf a) = amtDelta a

  public export
  Bindingless : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Bindingless {bs} {k} n = nounDelta n = []

  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase This = True
  anchorPhrase (AsType t n _) = anchorPhrase n
  anchorPhrase You = True
  anchorPhrase (PlayerGroup _) = True
  anchorPhrase (Each _) = False
  anchorPhrase (Indefinite _ _) = False
  anchorPhrase (Definite _) = False
  anchorPhrase (TargetGroup _ _) = True
  anchorPhrase (CountedGroup _ _ _) = False
  anchorPhrase (AllOf _) = False
  anchorPhrase (EachOf _) = False
  anchorPhrase (Both _ _) = False
  anchorPhrase (EitherOf l r) = anchorPhrase l && anchorPhrase r
  anchorPhrase (BothOf _ _) = False
  anchorPhrase (EachOfBoth _) = False
  anchorPhrase (LibrarySlice _ _ _) = False
  anchorPhrase (SomeOf _ _ _) = False
  anchorPhrase (NamesAgree _ grp) = anchorPhrase grp
  anchorPhrase TheRest = False
  anchorPhrase It = True
  anchorPhrase (ItAt _) = True
  anchorPhrase (ItVerbed _) = True
  anchorPhrase ItToken = True
  anchorPhrase (ItOtherThan _ _) = True
  anchorPhrase (ItPrior _ _) = True
  anchorPhrase They = True
  anchorPhrase Them = True
  anchorPhrase (ThemVerbed _) = True
  anchorPhrase (Those _) = True
  anchorPhrase (That _) = True
  anchorPhrase (ThatHalf _) = True
  anchorPhrase (AttachHost _ _) = True
  anchorPhrase (TheVerbed _ _ _) = True
  anchorPhrase (ThoseVerbed _ _ _) = True
  anchorPhrase (ControllerOf _) = True
  anchorPhrase (OwnerOf _) = True
  anchorPhrase (Designated _ _) = True

  public export
  data ComplementAnchor : Noun bs k -> Type where
    MkComplementAnchor : {auto 0 sh : So (anchorPhrase n)} ->
                         {auto 0 one : nounPlur n = OneOf} ->
                         ComplementAnchor n

  public export
  data LinkSource : Noun bs k -> Type where
    SelfLinked : LinkSource This
    ||| The same self-link written at a type word rather than by name.
    ||| The subtype slot rides along because [CR#205.3c] correlates a
    ||| subtype word to its own card type, so "this Saga" names the
    ||| linking object exactly as "this enchantment" does and the note
    ||| [CR#406.6] hangs on the object, not on which word named it.
    SortedSelfLinked : {0 t : CardType} -> {0 sub : Maybe Subtype} ->
                       {0 asc : Ascribable This} ->
                       {0 way : So (ascriptionOk t sub)} ->
                       LinkSource (AsType t This sub {asc} {way})

  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable This = False
  choosable (AsType _ _ _) = False
  choosable You = False
  choosable (PlayerGroup _) = False
  choosable (Each _) = False
  choosable (Indefinite _ _) = True
  choosable (Definite _) = False
  choosable (TargetGroup _ _) = True
  choosable (CountedGroup _ _ _) = True
  choosable (AllOf _) = False
  choosable (EachOf _) = False
  choosable (Both _ _) = False
  choosable (EitherOf _ _) = False
  choosable (BothOf _ _) = False
  choosable (EachOfBoth _) = False
  choosable (LibrarySlice _ _ _) = False
  choosable (SomeOf _ _ _) = False
  choosable (NamesAgree _ grp) = choosable grp
  choosable TheRest = False
  choosable It = False
  choosable (ItAt _) = False
  choosable (ItVerbed _) = False
  choosable ItToken = False
  choosable (ItOtherThan _ _) = False
  choosable (ItPrior _ _) = False
  choosable They = False
  choosable Them = False
  choosable (ThemVerbed _) = False
  choosable (Those _) = False
  choosable (That _) = False
  choosable (ThatHalf _) = False
  choosable (AttachHost _ _) = False
  choosable (TheVerbed _ _ _) = False
  choosable (ThoseVerbed _ _ _) = False
  choosable (ControllerOf _) = False
  choosable (OwnerOf _) = False
  choosable (Designated _ _) = False

  public export
  Choosable : Noun bs k -> Type
  Choosable {bs} {k} n = So (choosable n)

  public export
  agentChoosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  agentChoosable (SomeOf _ _ _) = True
  agentChoosable n = choosable n

  public export
  data ChoiceClause : {0 bs : Bindings} -> {0 k : Kind} ->
                      Maybe (Noun bs Player) -> Noun bs k -> Type where
    BareChoice : {0 n : Noun bs k} ->
                 {auto 0 ok : So (choosable n)} -> ChoiceClause Nothing n
    AgentChoice : {0 by : Noun bs Player} -> {0 n : Noun bs k} ->
                  {auto 0 ok : So (agentChoosable n)} ->
                  ChoiceClause (Just by) n

  public export
  groupMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  groupMention (TargetGroup _ _) = True
  groupMention Them = True
  groupMention (ThemVerbed _) = True
  groupMention (Those _) = True
  groupMention This = False
  groupMention (AsType _ _ _) = False
  groupMention You = False
  groupMention (PlayerGroup _) = False
  groupMention (Each _) = False
  groupMention (Indefinite _ _) = False
  groupMention (Definite _) = False
  groupMention (CountedGroup _ _ _) = False
  groupMention (AllOf _) = False
  groupMention (EachOf _) = False
  groupMention (Both _ _) = False
  groupMention (EitherOf _ _) = False
  groupMention (BothOf _ _) = False
  groupMention (EachOfBoth _) = False
  groupMention (LibrarySlice _ _ _) = True
  groupMention (SomeOf _ _ _) = False
  groupMention (NamesAgree _ grp) = groupMention grp
  groupMention TheRest = False
  groupMention It = False
  groupMention (ItAt _) = False
  groupMention (ItVerbed _) = False
  groupMention ItToken = False
  groupMention (ItOtherThan _ _) = False
  groupMention (ItPrior _ _) = False
  groupMention They = False
  groupMention (That _) = False
  groupMention (ThatHalf _) = False
  groupMention (AttachHost _ _) = False
  groupMention (TheVerbed _ _ _) = False
  groupMention (ThoseVerbed _ _ _) = True
  groupMention (ControllerOf _) = False
  groupMention (OwnerOf _) = False
  groupMention (Designated _ _) = False

  public export
  GroupMention : Noun bs k -> Type
  GroupMention {bs} {k} n = So (groupMention n)

  ||| What the trailing "each" may distribute over: a coordinated pair
  ||| and nothing else. The distributive reads the ARMS, so there has to
  ||| be a written pair of them; every other mention names one referent
  ||| or one group, and a group's distributive is `Each`/`EachOf`.
  public export
  coordinatedPair : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  coordinatedPair (BothOf _ _) = True
  coordinatedPair _ = False

  public export
  CoordinatedPair : Noun bs k -> Type
  CoordinatedPair {bs} {k} n = So (coordinatedPair n)

  ||| Which mentions write their own headcount: the two that carry a
  ||| `Quantity` over their own description. A partitive counts a slice of a
  ||| group some earlier phrase introduced, and every other mention counts
  ||| nothing, so neither has a written group for a group-level constraint
  ||| to attach to.
  public export
  countedMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedMention (CountedGroup _ _ _) = True
  countedMention (TargetGroup _ _) = True
  countedMention _ = False

  public export
  CountedMention : Noun bs k -> Type
  CountedMention {bs} {k} n = So (countedMention n)

  ||| The counted mention a condition can quantify over: a counted mention
  ||| UNDER a group-level name constraint, and nothing else. The bare form
  ||| is refused twice over. "Three or more artifacts" already has a
  ||| spelling — a comparison against the described set's headcount — and a
  ||| second one would say the same thing in a second shape. And a targeted
  ||| group would be worse than redundant: a comparison's two amounts carry
  ||| their phrases' bindings out to the clause they govern, where
  ||| [CR#601.2c] has every target announced, while this condition tests and
  ||| introduces nothing, so a target written inside it would be announced
  ||| nowhere.
  public export
  countedExistential : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedExistential (NamesAgree _ grp) = countedMention grp
  countedExistential _ = False

  public export
  CountedExistential : Noun bs k -> Type
  CountedExistential {bs} {k} n = So (countedExistential n)

  public export
  perMemberOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  perMemberOk (Each _) = True
  perMemberOk (EachOf _) = True
  perMemberOk (AllOf _) = True
  perMemberOk n = isOne (nounPlur n)

  public export
  PerMember : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  PerMember {bs} {k} n = So (perMemberOk n)


  ||| A chosen-quality ascription's subject has to be able to CARRY the
  ||| value read: [CR#205.3d] forbids an object to gain a subtype that
  ||| does not correspond to one of its types, which makes "becomes the
  ||| basic land type of your choice" nonsense of a creature -- exactly
  ||| what the bespoke land-typed row it replaces said, now stated once
  ||| for the creature-type and land-type rows alike. Sorts with no host
  ||| demand nothing of the subject.
  public export
  hostedRead : {bs : Bindings} -> {k : Kind} ->
               Predicate bs Object -> Noun bs k -> Bool
  hostedRead p n = case qualityReadHost p of
                     Nothing => True
                     Just h => tyIs h (nounTy n)

  public export
  HostedRead : {bs : Bindings} -> {k : Kind} ->
               Predicate bs Object -> Noun bs k -> Type
  HostedRead {bs} {k} p n = So (hostedRead p n)

  ||| `ActSubject` retired with `ObjectAct`. Its six rows said, per act,
  ||| what KIND of referent the act is done to and what ZONE the act's
  ||| own rule finds it in; both are now `deedFacts` fields
  ||| (`roleKinds`, `roleZone`) read by one gate at the unified carrier,
  ||| so a new deed adds a row rather than a constructor here and a
  ||| clause in every reader.

  ||| The possessor of a relation an object can hold to only ONE player:
  ||| [CR#110.2] gives a permanent one controller, the player under whose
  ||| control it entered, [CR#601.2a] one caster, and [CR#602.2a] one
  ||| activator ("its controller is the player who activated the
  ||| ability"), which is the `ActivatedBy` row. A group word
  ||| distributes — each member holds the relation to its own objects — so
  ||| "creatures players control" names a set. A counted plural does not
  ||| distribute: it asks for the object all of a named two control at
  ||| once, and [CR#110.2] leaves that empty.
  public export
  soleHolderOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  soleHolderOk (PlayerGroup _) = True
  soleHolderOk n = isOne (nounPlur n)

  public export
  SoleHolder : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SoleHolder {bs} {k} n = So (soleHolderOk n)

  ||| Whose library a slice may name. [CR#400.1] gives each player their
  ||| OWN library, so a possessive reaching several players names one
  ||| library apiece: the slice distributes and the phrase pluralises the
  ||| ZONE word rather than the card word -- "the top card of their
  ||| libraries", one card per library, which is the surface
  ||| `outputPlur` already computes.
  ||| Which plurals may say that is `soleHolderOk`'s recorded argument,
  ||| at the possessed zone instead of the possessed object: the
  ||| distributive words do -- "each player", "each of those opponents",
  ||| and the bare group words -- and a COUNTED plural does not, since it
  ||| asks for the one library a named two have between them and
  ||| [CR#400.1] leaves that empty.
  public export
  slicePossessorOk : {bs : Bindings} -> Noun bs Player -> Bool
  slicePossessorOk (Each _) = True
  slicePossessorOk (EachOf _) = True
  slicePossessorOk (PlayerGroup _) = True
  slicePossessorOk n = isOne (nounPlur n)

  public export
  SlicePossessor : {bs : Bindings} -> Noun bs Player -> Type
  SlicePossessor {bs} n = So (slicePossessorOk n)

  public export
  costSubjectOk : {bs : Bindings} -> Noun bs Object -> Bool
  costSubjectOk This = True
  costSubjectOk n = onStackZone (nounZone n)

  public export
  data CostSubject : {0 k : Kind} -> Noun bs k -> Type where
    MkCostSubject : {0 n : Noun bs Object} ->
                    {auto 0 ok : So (costSubjectOk n)} -> CostSubject n
    AbilityCostSubject : {0 n : Noun bs Ability} -> CostSubject n

  ||| Which kinds a counter instruction can name. [CR#701.6a] counters a
  ||| spell or an ability by removing it from the stack, so the subject
  ||| is whatever the stack holds: a spell, which is a card on the stack
  ||| [CR#112.1], or an ability on the stack, which [CR#109.1] makes an
  ||| object of its own. "Counter target spell or ability" names the pair
  ||| at once, so a joined phrase counts when every half does. A player
  ||| is on neither list [CR#109.1], which is what keeps "counter target
  ||| creature or player" out with no rule written for the purpose.
  public export
  counterKind : Kind -> Bool
  counterKind Object = True
  counterKind Ability = True
  counterKind (a \/ b) = counterKind a && counterKind b
  counterKind _ = False

  ||| What a counter instruction may name, row by row. The object row
  ||| asks for the stack, since [CR#112.1] puts a spell there. The
  ||| ability row asks no zone: the `Ability` kind places nothing,
  ||| because the same kind writes the abilities a permanent HAS
  ||| ("activated abilities of artifacts can't be activated"), and it is
  ||| the head's CLASS and not a zone that says an ability reached the
  ||| stack. The joined row asks the kind alone: the union family is
  ||| placeless, so a joined phrase's `nounZone` is `Nothing` whatever
  ||| its halves said and no half's stack can be re-asked here. A head
  ||| whose class never
  ||| uses the stack [CR#113.3d,113.9] passes every row; that is
  ||| tolerated overgeneration, and no printed line writes one.
  public export
  data Counterable : {0 k : Kind} -> Noun bs k -> Type where
    SpellCountered : {0 n : Noun bs Object} ->
                     {auto 0 zn : OnStack (nounZone n)} -> Counterable n
    AbilityCountered : {0 n : Noun bs Ability} -> Counterable n
    JoinCountered : {0 ka : Kind} -> {0 kb : Kind} ->
                    {0 n : Noun bs (ka \/ kb)} ->
                    {auto 0 ck : So (counterKind (ka \/ kb))} -> Counterable n

  public export
  selfDefinedOk : {bs : Bindings} -> Noun bs Object -> Bool
  selfDefinedOk This = True
  selfDefinedOk (AsType _ n _) = selfDefinedOk n
  selfDefinedOk _ = False

  public export
  SelfDefined : {bs : Bindings} -> Noun bs Object -> Type
  SelfDefined {bs} n = So (selfDefinedOk n)

  public export
  data Condition : Bindings -> Type where
    Exists : {k : Kind} -> (p : Predicate bs k) ->
             Condition bs
    ||| "if you control two or more nonland, nontoken permanents with the
    ||| same name as one another", "if you control three or more lands with
    ||| the same name": the counted existential. `Exists` asks whether ANY
    ||| object answers a description; this asks whether a group of the
    ||| written size does, which is the only place a group-level constraint
    ||| [CR#201.2a] can be tested, since the constraint rides the mention
    ||| and a description takes no count. That constraint is the whole
    ||| reason it exists, and its gate admits nothing else: a bare counted
    ||| mention is the comparison's business, and a targeted one would go
    ||| unannounced here. It tests and names nothing, so it introduces
    ||| nothing, like every other described-set condition.
    ExistsGroup : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 ce : CountedExistential n} -> Condition bs
    Happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               (what :
                  Maybe (EventComplement (nomIntro who) ev k)) ->
               {auto 0 cw : ComplementWritten what} ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
    GameIs : (d : Designation) ->
             {auto 0 sc : designationScope d = HeldByGame} ->
             {auto 0 at : So (designationChecked d)} -> Condition bs
    ||| "there is no monarch" [CR#725.1]: an existential over the holder,
    ||| asking whether ANY player holds the designation where
    ||| `HasDesignation` describes one that does. Only the absence is
    ||| written, so the absence is the whole condition.
    NoHolder : (d : Designation) ->
               {auto 0 sc : designationScope d = HeldBy Player} ->
               {auto 0 at : So (designationChecked d)} -> Condition bs
    Matches : {k : Kind} -> (n : Noun bs k) -> (p : Predicate bs k) ->
              {auto 0 bl : Bindingless n} ->
              {auto 0 sy : PredSays p} ->
              {auto 0 zc : ZoneFits (nounZone n) (seedZone p)} ->
              Condition bs
    ||| The bound slot is open to ANY amount at every relation: the rules
    ||| make a comparison against any number meaningful, and what bound
    ||| shapes English prints at which relation is spelling-boundary
    ||| knowledge, recorded on the deciding ticket rather than gated here.
    CompareAmt : (subj : Amount bs) -> (r : Comparator) ->
                 (bound : Amount (amtIntro subj)) ->
                 {auto 0 rd : ReadAmount subj} ->
                 Condition bs
    ||| "If a player is dealt damage this way, ..." (Screaming Nemesis),
    ||| "If a creature is dealt damage this way, ..." (Burn from Within):
    ||| the kind refinement on damage the text has dealt.
    ||| [CR#608.2c] is the rule that lets later text read the instruction it
    ||| follows -- one of its two worked examples is a "...this way"
    ||| back-reference -- and it settles no more than that the reading is
    ||| available. WHICH earlier instruction "this way" names is left to
    ||| whoever reads the card, so the licence here is deliberately loose:
    ||| the gate asks only that SOME damage a clause dealt is in scope
    ||| (`damageDealtInScope`), and no discrimination between two of them is
    ||| attempted. A text carrying two damage mentions can therefore write a
    ||| refinement pointing at one and an anaphor resolving against the
    ||| other; that mis-pairing is tolerated overgeneration, refused at the
    ||| spelling boundary, and narrowing it would need the discourse to
    ||| order its outcome mentions, which nothing else asks for.
    ||| The description is bounded by what damage can be dealt
    ||| to -- battles, creatures, planeswalkers and players [CR#120.1] --
    ||| whose coarsest bound in the kind lattice is `Object \/ Player`; a
    ||| refinement at any other kind is a category error.
    ||| It tests and names no referent, like every other described-set
    ||| condition. Under a joined kind it needs to name none: the body's
    ||| "they" reads the union binding the damage went to, at its player
    ||| half, and this condition is what discharges the presupposition that
    ||| read carries.
    ||| -- spelling: "If [description] is dealt damage this way, [e]."
    DealtThisWay : {k : Kind} -> (p : Predicate bs k) ->
                   {auto 0 wy : So (damageDealtInScope bs)} ->
                   {auto 0 rk : So (kindLte k (Object \/ Player))} ->
                   Condition bs
    ||| "If you win the flip, …", "If you lose the flip, …": the called
    ||| reading of a coin an earlier clause flipped. [CR#705.2] settles
    ||| both halves of the shape — the flipper calls heads or tails and
    ||| wins the flip when the call matches, and "only the player who
    ||| flips the coin wins or loses the flip; no other players are
    ||| involved", which is why the subject is a slot and not the whole
    ||| construction's business. The arms are conditions over the flip
    ||| rather than slots ON it because the same rule gives a flip a
    ||| SECOND reading (`FlipFace`) that no winner attends, and because
    ||| [CR#705.1]'s flip is complete without either: a cumulative upkeep
    ||| that is a bare flip writes no arm at all.
    ||| The gate is an existence test on the flip's mention, like every
    ||| other back-reference to a clause's own outcome; it tests and
    ||| names no referent, so it introduces nothing.
    ||| -- spelling: "If you win the flip, [e]." / "… lose the flip, …"
    FlipCalled : (who : Noun bs Player) -> (call : FlipCall) ->
                 {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
    ||| "If the coin comes up heads, …": [CR#705.2]'s other reading, for
    ||| the effects that "care only about whether the coin comes up heads
    ||| or tails". The rule says no player wins or loses a flip read this
    ||| way, so this condition takes no subject.
    ||| -- spelling: "If the coin comes up heads, [e]."; after a flip
    ||| already named, "If it comes up tails, [e]."
    FlipFace : (face : CoinFace) ->
               {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
    ||| "If any of those results was 10 or higher, …": the existential
    ||| over the results of the dice one clause rolled. [CR#706.2] gives
    ||| each die its own result and stops there, so a clause that rolled
    ||| several left several numbers behind; `TheResult` reads one and
    ||| `TheTotal` sums them, and neither asks whether SOME one of them
    ||| clears a bound. The bound is written with a comparator and an
    ||| amount, as `CompareAmt`'s is and unlike the roll header's
    ||| striation range, because this one stands in a sentence ("was 10
    ||| or higher") rather than in the determiner's place.
    ||| Gated on the roll exactly as `TheResult` is, and it names no
    ||| referent, so it introduces nothing. Over a single die it agrees
    ||| with the plain comparison on `TheResult` and is tolerated
    ||| overgeneration, named here at its zero.
    ||| -- spelling: "if any of those results was [r] [bound]".
    AnyResultIs : (r : Comparator) -> (bound : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} ->
                  Condition bs
    ||| "If you rolled doubles, …": [CR#706.5] defines the phrase
    ||| outright -- "A player has rolled doubles if the result of each of
    ||| those rolls is equal to the other" -- so it is a read over the
    ||| rolls one clause made and carries no bound of its own. It takes
    ||| no subject for `TheTotal`'s reason: the rule's "those rolls" is
    ||| the roll the clause already named, and the roller comes with it.
    ||| The rule's own wording is about two rolls; a clause that rolled
    ||| some other number leaves the phrase saying that every result
    ||| equals every other, which is tolerated overgeneration.
    ||| -- spelling: "if you rolled doubles".
    RolledDoubles : {auto 0 ok : countOutcomes RollResult bs = 1} ->
                    Condition bs
    NotCond : (c : Condition bs) -> Condition bs
    AndCond : (cs : List (Condition bs)) ->
              {auto 0 tw : TwoConjuncts cs} ->
              {auto 0 fl : FlatConjuncts cs} -> Condition bs
    ||| "if X or if Y": the condition disjunction, `AndCond`'s twin.
    ||| [CR#603.4] is the one rule that gives "if" a meaning of its own,
    ||| and it declines this one outright -- the word "has only its normal
    ||| English meaning anywhere else in the text of a card" -- so a
    ||| coordination of two conditions is ordinary English disjunction and
    ||| carries no rules content past the arms it joins.
    |||
    ||| An arm is an ATOM or a FLAT CONJUNCTION, one level and nothing
    ||| deeper. The printed scope marker is the reduplicated "if": each
    ||| "if" opens a disjunct, so a conjunction binds inside the disjunct
    ||| its own "if" opened, which is what Quakebringer writes whole
    ||| ("only if Quakebringer is on the battlefield or if Quakebringer is
    ||| in your graveyard and you control a Giant"). `FlatDisjuncts` is
    ||| `FlatConjuncts`' COMPANION and not a copy: it admits the
    ||| conjunction its twin refuses and refuses only a nested
    ||| disjunction.
    ||| The mirror nesting stays where `flatConjuncts` already leaves it --
    ||| a conjunction may take a disjunct arm ("If this creature would
    ||| enter and it wasn't cast or no mana was spent to cast it",
    ||| Primeval Spawn, the one supported line) -- because no marking
    ||| disambiguates that one in print and this row does not narrow it.
    ||| -- spelling: "if [c] or if [c]"; singly marked, "if [c] or [c]".
    OrCond : (cs : List (Condition bs)) ->
             {auto 0 tw : TwoDisjuncts cs} ->
             {auto 0 fd : FlatDisjuncts cs} -> Condition bs

  public export
  atLeastTwoCs : {0 bs : Bindings} -> List (Condition bs) -> Bool
  atLeastTwoCs [] = False
  atLeastTwoCs (_ :: []) = False
  atLeastTwoCs (_ :: _ :: _) = True

  public export
  TwoConjuncts : List (Condition bs) -> Type
  TwoConjuncts {bs} cs = So (atLeastTwoCs cs)

  ||| The same arity demand for the OR row: `atLeastTwoCs` counts arms and
  ||| does not care which word joins them, so the measurement is shared and
  ||| only the name is new.
  public export
  TwoDisjuncts : List (Condition bs) -> Type
  TwoDisjuncts {bs} cs = So (atLeastTwoCs cs)

  public export
  isAndCond : {0 bs : Bindings} -> Condition bs -> Bool
  isAndCond (AndCond _) = True
  isAndCond _ = False

  public export
  isOrCond : {0 bs : Bindings} -> Condition bs -> Bool
  isOrCond (OrCond _) = True
  isOrCond _ = False

  public export
  flatConjuncts : {0 bs : Bindings} -> List (Condition bs) -> Bool
  flatConjuncts [] = True
  flatConjuncts (c :: cs) = not (isAndCond c) && flatConjuncts cs

  public export
  FlatConjuncts : List (Condition bs) -> Type
  FlatConjuncts {bs} cs = So (flatConjuncts cs)

  ||| `flatConjuncts`' companion, and deliberately not its copy: a
  ||| disjunct may BE a conjunction -- the reduplicated "if" is what binds
  ||| the conjunction inside its own arm -- so the only arm this refuses is
  ||| a second disjunction, where the conjunct gate refuses a second
  ||| conjunction.
  public export
  flatDisjuncts : {0 bs : Bindings} -> List (Condition bs) -> Bool
  flatDisjuncts [] = True
  flatDisjuncts (c :: cs) = not (isOrCond c) && flatDisjuncts cs

  public export
  FlatDisjuncts : List (Condition bs) -> Type
  FlatDisjuncts {bs} cs = So (flatDisjuncts cs)

  public export
  condNegated : {0 bs : Bindings} -> Condition bs -> Bool
  condNegated (Exists _) = False
  condNegated (ExistsGroup _) = False
  condNegated (Happened _ _ _ _) = False
  condNegated (GameIs _) = False
  -- atomic: the absence is the condition's own content, not a marked
  -- negation of one.
  condNegated (NoHolder _) = False
  condNegated (Matches _ _) = False
  condNegated (CompareAmt _ _ _) = False
  condNegated (DealtThisWay _) = False
  condNegated (FlipCalled _ _) = False
  condNegated (FlipFace _) = False
  condNegated (AnyResultIs _ _) = False
  condNegated RolledDoubles = False
  condNegated (NotCond _) = True
  condNegated (AndCond _) = False
  condNegated (OrCond _) = False

  public export
  markingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Bool
  markingOk AsLongAs _ = True
  markingOk Unless c = condNegated c

  public export
  data MarkingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Type where
    MkMarkingOk : {0 c : Condition bs} ->
                  {auto 0 ok : So (markingOk m c)} -> MarkingOk m c

  ||| What a condition introduces for the clause it governs to read: the
  ||| statement's own subject re-mentioned, so the body may say "it"
  ||| (Adanto Vanguard); a comparison's margin, "the difference"; and the
  ||| phrases a comparison's two amounts name — a target inside a
  ||| condition is announced at casting like any other [CR#601.2c].
  |||
  ||| That last principle reaches exactly the rows that can CARRY a
  ||| mention, which is why the table looks partial and is not. A target
  ||| rides a determiner on a mention, and `CompareAmt`'s two amounts are
  ||| the only place a condition writes one: `Exists` and `DealtThisWay`
  ||| take a `Predicate`, which describes and mentions nothing;
  ||| `ExistsGroup` is gated to a counted described group
  ||| (`CountedExistential`) and `Matches` to a `Bindingless` subject. The
  ||| two rows that re-mention a SUBJECT (`Happened`, `Matches`) announce
  ||| that and nothing else, for the reasons written at each. A
  ||| designation check, a flip or roll read and a disjunction announce
  ||| nothing at all: they test, and name no referent. A negation keeps
  ||| what its condition announced, less the margin a failed comparison
  ||| never left.
  public export
  condDelta : {bs : Bindings} -> Condition bs -> List Binding
  condDelta (Exists _) = []
  condDelta (ExistsGroup _) = []
  -- [CR#603.4] has an intervening "if" clause checked when the ability
  -- would trigger AND again as it resolves, so the clause it governs
  -- reads a condition that HELD -- and the subject it held of is a
  -- referent the body may name. What it announces is the SUBJECT alone,
  -- on `Matches`' model below: the complement is what the condition
  -- TESTED the subject against, and announcing it too would leave a
  -- printed "it" with two candidates (Whirling Dervish).
  condDelta (Happened _ who _ _) = selfSubjDelta who
  condDelta (GameIs _) = []
  condDelta (NoHolder _) = []
  condDelta (Matches n _) = selfSubjDelta n
  condDelta (CompareAmt subj _ bound) =
    gapB :: (tiedDelta subj ++ amtDelta bound ++ amtDelta subj)
  condDelta (DealtThisWay _) = []
  condDelta (FlipCalled _ _) = []
  condDelta (FlipFace _) = []
  condDelta (AnyResultIs _ _) = []
  condDelta RolledDoubles = []
  -- a comparison that did NOT hold leaves neither a margin nor a tie
  -- set, so the negated row announces only the phrases its two amounts
  -- named. Written out rather than filtered: the margin is findable by
  -- kind and the tie set is not, so the row that minted them is the row
  -- that has to unmake them.
  condDelta (NotCond (CompareAmt subj _ bound)) = amtDelta bound ++ amtDelta subj
  condDelta (NotCond c) = dropGaps (condDelta c)
  condDelta (AndCond cs) = condDeltaAll cs
  -- a disjunction cannot say WHICH arm held, so it announces nothing:
  -- summing the arms would hand the consequent a phrase from an arm that
  -- may have been false. No supported line reads back from a disjunct,
  -- and a conjunction still sums, since every conjunct held.
  condDelta (OrCond _) = []

  ||| The group a counted comparison over a UNIQUIFYING description
  ||| names: "If two or more creatures are tied for greatest power, you
  ||| choose one of THEM." A superlative describes however many objects
  ||| hold the extreme rather than exactly one, so counting it is how
  ||| English asks whether the extreme is tied -- and the set it counted
  ||| is what the consequent then partitions, by a choice the acting
  ||| player makes as the effect applies [CR#608.2d]. Minted exactly where the
  ||| description uniquifies: an ordinary count ("if you control three or
  ||| more lands") names a number and no group, which is why the row is a
  ||| gate on `uniquifies` and not on the comparison.
  ||| Object and player halves both print it; other kinds write no such
  ||| line and mint nothing.
  public export
  tiedDelta : {bs : Bindings} -> Amount bs -> List Binding
  tiedDelta (CountOf {k = Object} p) =
    if uniquifies p then [bindFor TheD ManyOf PhObject p] else []
  tiedDelta (CountOf {k = Player} p) =
    if uniquifies p then [bindFor TheD ManyOf PhPlayer p] else []
  tiedDelta _ = []

  public export
  condDeltaAll : {bs : Bindings} -> List (Condition bs) -> List Binding
  condDeltaAll [] = []
  condDeltaAll (c :: cs) = condDelta c ++ condDeltaAll cs

  ||| A negated condition unmakes no announcement — a target written
  ||| inside "unless [comparison]" is announced at casting like any other
  ||| [CR#601.2c] — but a comparison that did NOT hold leaves no margin,
  ||| so the `Gap` binding alone is dropped.
  public export
  dropGaps : List Binding -> List Binding
  dropGaps [] = []
  dropGaps (MkBinding _ Gap _ GapP :: bs) = dropGaps bs
  dropGaps (b :: bs) = b :: dropGaps bs

  ||| The card types an attack may name [CR#506.3]: "Only a player, a
  ||| planeswalker, or a battle can be attacked." A creature is never
  ||| attacked, and neither is any other permanent type.
  public export
  attackableTy : Maybe CardType -> Bool
  attackableTy (Just Planeswalker) = True
  attackableTy (Just Battle) = True
  attackableTy _ = False

  ||| The same closed set read off a phrase's kind, ONE HALF AT A TIME. A
  ||| player half is attackable however it is described; an object half
  ||| only at an attackable type; a joined phrase only when both halves
  ||| are, each against its OWN head type, so "a planeswalker or a
  ||| creature" is refused where "that player or planeswalker" passes.
  ||| A phrase that named one description for the pair is asked about it
  ||| at every half, which is what "any target" [CR#115.4] fails: its
  ||| object half projects no single attackable type.
  public export
  attackableKind : (k : Kind) -> HeadTy k -> Bool
  attackableKind Player _ = True
  attackableKind Object (SoleTy t) = attackableTy t
  attackableKind (a \/ b) (JoinTy l r) = attackableKind a l && attackableKind b r
  attackableKind (a \/ b) (SoleTy t) =
    attackableKind a (SoleTy t) && attackableKind b (SoleTy t)
  attackableKind _ _ = False

  ||| The phrase names something that can be attacked [CR#506.3]. The
  ||| joined kind alone would admit a creature defender, which the rule
  ||| closes out.
  public export
  Attackable : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Attackable {k} n = So (attackableKind k (nounTys n))

  public export
  data EventAgent : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    AgentUnvoiced : EventAgent Nothing
    AgentVoiced : {0 w : Noun bs Player} ->
                  {auto 0 bl : Bindingless w} -> EventAgent (Just w)

  public export
  data TokenPhrase : {0 bs : Bindings} -> Noun bs Object -> Type where
    CountedTokens : {0 q : Quantity bs} -> {0 m : Maybe (ChoiceMode bs)} ->
                    {0 p : Predicate bs Object} ->
                    {0 ph : Phrasal Object} -> {0 nz : NonZeroQ q} ->
                    {0 wf : WellFormedQ q} ->
                    {auto 0 ok : So (seedsToken p)} ->
                    TokenPhrase (CountedGroup q m p {ph} {nz} {wf})
    OneToken : {0 m : ChoiceMode bs} -> {0 p : Predicate bs Object} ->
               {0 ph : Phrasal Object} ->
               {auto 0 ok : So (seedsToken p)} ->
               TokenPhrase (Indefinite m p {ph})

  ||| The announcement a DEICTIC phrase makes on its own behalf: the
  ||| source, or an attachment's host, re-mentioned so a clause reading
  ||| this one may say "it". A described phrase names its referent
  ||| through its own `nounDelta` and needs no row here; a self-name and
  ||| an attach word have no delta of their own, which is the whole
  ||| reason the rows exist.
  ||| Two seats read it: a clause's SUBJECT, through `selfSubjIntro`, and
  ||| a possessive's BASE, through `nounDelta`'s relational rows -- the
  ||| two places a deictic is named in full and a later pronoun can point
  ||| back at it.
  public export
  selfSubjDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  selfSubjDelta (AsType t This _) =
    [MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  selfSubjDelta (AttachHost _ (TypeW t)) =
    [MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  selfSubjDelta (AttachHost _ PermanentW) =
    [MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing)]
  selfSubjDelta (AttachHost _ PlayerW) = [MkBinding TheD Player OneOf PlayerP]
  selfSubjDelta _ = []

  public export
  selfSubjIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  selfSubjIntro n = selfSubjDelta n ++ nomIntro n

  ||| Which of the prefix's mentions a test's SUBJECT names, where the
  ||| grammar can say it without a word. The three type-less singular
  ||| object reads are exactly the ones that can be answered: `It` and
  ||| its two narrowings name a referent by counted uniqueness and carry
  ||| no head word of their own, so what they resolved to is what the
  ||| test was about. Every other subject either names its own type
  ||| already -- a demonstrative and a participle read both write a noun
  ||| word -- or is a description rather than a read, and a test on it
  ||| tells the clauses after it nothing they could not already write.
  public export
  remarkTest : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k ->
               Maybe (Binding -> Bool)
  remarkTest It = Just (itReaches OneOf)
  remarkTest (ItAt sl) = Just (itAtReaches sl)
  remarkTest (ItVerbed v) = Just (itVerbedReaches v)
  remarkTest ItToken = Just itTokenReaches
  -- the segment reads answer no per-binding question: their gate is
  -- `It`'s over ONE segment of the prefix, and a test asked of every
  -- binding would reach the candidates the segment excludes. So they
  -- say nothing about the subject rather than say it wrongly.
  remarkTest (ItOtherThan _ _) = Nothing
  remarkTest (ItPrior _ _) = Nothing
  remarkTest _ = Nothing

  ||| ...and which re-mark a whole condition licenses: the test its
  ||| subject supplies, paired with the type its complement names.
  ||| `Nothing` where the condition licenses none, which is every
  ||| condition but the positive copula and every copula whose subject
  ||| names its own word.
  public export
  condRemarkAt : {0 bs : Bindings} -> Condition bs ->
                 Maybe (Binding -> Bool, Maybe CardType)
  condRemarkAt (Matches n p) =
    case remarkTest n of
      Nothing => Nothing
      Just q => Just (q, seedTy p)
  condRemarkAt _ = Nothing

  ||| What a condition leaves MARKED on the prefix, as against what it
  ||| ANNOUNCES (`condDelta`). A copula test naming a card type is the
  ||| one condition that leaves knowledge behind about a mention that
  ||| already stood -- "If it's a creature card, …" is true of the card
  ||| the clause before it exiled -- and the honest place to put that
  ||| knowledge is on the binding, in place. Announcing it instead would
  ||| mint a second mention of one referent and leave the pronoun after
  ||| it with two candidates.
  |||
  ||| Only the POSITIVE, un-coordinated copula re-marks. A negated test
  ||| says which type the mention is not, and one type slot cannot hold
  ||| that; a coordination's arms would each have to be walked, and no
  ||| supported line writes "if it's a creature card and …" over a
  ||| type-less mention. Both are left at the identity, and both are
  ||| re-openable by a row here rather than by a change of shape.
  public export
  condRemark : {bs : Bindings} -> Condition bs -> Bindings
  condRemark c = maybe bs (\qt => markFirst (fst qt) (snd qt) bs) (condRemarkAt c)

  ||| A condition hands its consequent what it announced, in front of the
  ||| prefix AS THE CONDITION LEFT IT MARKED. The second term is an
  ||| in-place re-mark of the prefix and never an insertion from
  ||| elsewhere, which is the form `settleTargets` and `defineLetter`
  ||| already have; §5 of `Experimental.ProofsAnaphora` states it and
  ||| proves that every counted gate reads the re-marked prefix exactly
  ||| as it read the prefix.
  public export
  condIntro : {bs : Bindings} -> Condition bs -> Bindings
  condIntro c = condDelta c ++ condRemark c

  public export
  interveningIntro : {bs : Bindings} -> Maybe (Condition bs) -> Bindings
  interveningIntro Nothing = bs
  interveningIntro (Just c) = condIntro c

  ||| The third argument is the PRESENCE of a [CR#609.4] counterfactual
  ||| and no longer its value: `PlayAsThough = HadFlash` retired into the
  ||| carrier's general premise slot, and what this gate ever asked of it
  ||| was whether a counterfactual was written at all. A permission
  ||| carrying one describes the object by what it would be rather than
  ||| by where it is, so the complement answers `castComplementOk`.
  public export
  playSourceOk : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) ->
                 Bool -> Bool
  -- with no source phrase written, the complement's own sort word is the
  -- only thing that could name one, so the question is whether that word
  -- LOCATES the object at all -- `complementLocates`, not `playableFrom`,
  -- which answers a written origin phrase.
  playSourceOk zn Nothing False = complementLocates zn
  playSourceOk zn (Just z) False =
    playableFrom (Just (zoneSort z)) &&
    (not (complementLocates zn) || zoneFits zn (Just (zoneSort z)))
  playSourceOk zn Nothing True = castComplementOk zn
  playSourceOk zn (Just z) True =
    castComplementOk zn && playableFrom (Just (zoneSort z))

  public export
  data PlaySource : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) ->
                    Bool -> Type where
    MkPlaySource : {0 fz : Maybe (ZoneExpr bs)} ->
                   {auto 0 ok : So (playSourceOk zn fz at)} -> PlaySource zn fz at

  public export
  data Exposed : Bindings -> Type where
    ExposedCards : (n : Noun bs Object) -> Exposed bs
    ExposedZone : (z : ZoneExpr bs) ->
                  {auto 0 ok : ExposableZone (zoneSort z)} -> Exposed bs

  public export
  exposedIntro : {bs : Bindings} -> Exposed bs -> Bindings
  exposedIntro (ExposedCards n) = nomIntro n
  exposedIntro (ExposedZone z) = zoneDelta z ++ bs

  ||| What a STANDING visibility rider exposes -- the complement of
  ||| "play with [x] revealed" and "you may look at [x] any time". Two
  ||| POSITIONED surfaces and one described GROUP, and the split is what
  ||| the overridden rule is: [CR#401.2] and [CR#402.3] hide a library
  ||| and a hand by their place, so the place arms name a position and
  ||| nothing else and the possessive agrees with the subject; [CR#708.5]
  ||| hides a face-down spell or permanent by what it IS -- "you can't
  ||| look at ... face-down spells or permanents controlled by another
  ||| player" -- and an object arm is the only way to say which ones.
  ||| It is still not the exposure clause's `Exposed`: that one's zone
  ||| arm is a whole `ZoneExpr` a one-shot instruction names, where the
  ||| two place arms here are the standing rider's own two surfaces.
  ||| -- spelling: "the top card of [subj]'s library", "[subj]'s hand",
  ||| or the group's own phrase.
  public export
  data VisibleThing : Bindings -> Type where
    TopOfLibrary : VisibleThing bs
    WholeHand : VisibleThing bs
    ||| "face-down creatures you don't control" (Keeper of the Lens):
    ||| the group [CR#708.5] hides, described rather than placed.
    VisibleObjects : (n : Noun bs Object) -> VisibleThing bs

  public export
  visibleIntro : {bs : Bindings} -> VisibleThing bs -> Bindings
  visibleIntro TopOfLibrary = bs
  visibleIntro WholeHand = bs
  visibleIntro (VisibleObjects n) = nomIntro n

  public export
  visibilityOk : {0 bs : Bindings} -> ExposeVerb -> VisibleThing bs -> Bool
  visibilityOk Reveal TopOfLibrary = True
  visibilityOk Reveal WholeHand = True
  visibilityOk LookAt TopOfLibrary = True
  -- [CR#402.3] already gives a player leave to look at their own hand any
  -- time; "look at [subj]'s hand" can only agree with its own subject, so
  -- the sentence would just restate the rule.
  visibilityOk LookAt WholeHand = False
  -- [CR#708.5] states BOTH halves in one sentence: a player may look at
  -- their own face-down spells and permanents at any time, and "can't
  -- look at ... face-down spells or permanents controlled by another
  -- player". The look-at arm is that second half's override, so it is
  -- the one the corpus writes. Nothing closes the reveal cell -- a
  -- standing "play with [group] revealed" restates no rule and breaks
  -- none -- so it stands open at zero supported lines rather than
  -- refusing on a count.
  visibilityOk _ (VisibleObjects _) = True

  public export
  VisibilityOk : {0 bs : Bindings} -> ExposeVerb -> VisibleThing bs -> Type
  VisibilityOk v w = So (visibilityOk v w)

  public export
  costNounOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  costNounOk This = True
  costNounOk (AsType t n _) = costNounOk n
  costNounOk You = True
  costNounOk (PlayerGroup _) = True
  costNounOk (Each _) = True
  costNounOk (Indefinite _ _) = True
  costNounOk (Definite _) = True
  costNounOk (TargetGroup _ _) = True
  costNounOk (CountedGroup _ _ _) = True
  costNounOk (AllOf _) = True
  costNounOk (EachOf grp) = costNounOk grp
  costNounOk (NamesAgree _ grp) = costNounOk grp
  costNounOk (Both _ _) = False
  costNounOk (EitherOf l r) = costNounOk l && costNounOk r
  costNounOk (BothOf _ _) = False
  costNounOk (EachOfBoth _) = False
  costNounOk (LibrarySlice _ _ _) = True
  costNounOk (SomeOf _ _ grp) = costNounOk grp
  costNounOk TheRest = True
  costNounOk It = True
  costNounOk (ItAt _) = True
  costNounOk (ItVerbed _) = True
  costNounOk ItToken = True
  costNounOk (ItOtherThan _ _) = True
  costNounOk (ItPrior _ _) = True
  costNounOk They = True
  costNounOk Them = True
  costNounOk (ThemVerbed _) = True
  costNounOk (Those _) = True
  costNounOk (That _) = True
  costNounOk (ThatHalf _) = True
  costNounOk (AttachHost _ _) = True
  costNounOk (TheVerbed _ _ _) = False
  costNounOk (ThoseVerbed _ _ _) = False
  costNounOk (ControllerOf _) = True
  costNounOk (OwnerOf _) = True
  costNounOk (Designated _ _) = True

  public export
  nounIsYou : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsYou You = True
  nounIsYou (PlayerGroup _) = False
  nounIsYou This = False
  nounIsYou (AsType _ _ _) = False
  nounIsYou (Each _) = False
  nounIsYou (Indefinite _ _) = False
  nounIsYou (Definite _) = False
  nounIsYou (TargetGroup _ _) = False
  nounIsYou (CountedGroup _ _ _) = False
  nounIsYou (AllOf _) = False
  nounIsYou (EachOf _) = False
  nounIsYou (NamesAgree _ _) = False
  nounIsYou (Both _ _) = False
  nounIsYou (EitherOf _ _) = False
  nounIsYou (BothOf _ _) = False
  nounIsYou (EachOfBoth _) = False
  nounIsYou (LibrarySlice _ _ _) = False
  nounIsYou (SomeOf _ _ _) = False
  nounIsYou TheRest = False
  nounIsYou It = False
  nounIsYou (ItAt _) = False
  nounIsYou (ItVerbed _) = False
  nounIsYou ItToken = False
  nounIsYou (ItOtherThan _ _) = False
  nounIsYou (ItPrior _ _) = False
  nounIsYou They = False
  nounIsYou Them = False
  nounIsYou (ThemVerbed _) = False
  nounIsYou (Those _) = False
  nounIsYou (That _) = False
  nounIsYou (ThatHalf _) = False
  nounIsYou (AttachHost _ _) = False
  nounIsYou (TheVerbed _ _ _) = False
  nounIsYou (ThoseVerbed _ _ _) = False
  nounIsYou (ControllerOf _) = False
  nounIsYou (OwnerOf _) = False
  nounIsYou (Designated _ _) = False

  public export
  nounTargeted : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounTargeted (TargetGroup _ _) = True
  nounTargeted (CountedGroup _ _ _) = False
  nounTargeted This = False
  nounTargeted (AsType _ n _) = nounTargeted n
  nounTargeted You = False
  nounTargeted (PlayerGroup _) = False
  nounTargeted (Each _) = False
  nounTargeted (Indefinite _ _) = False
  nounTargeted (Definite _) = False
  nounTargeted (AllOf _) = False
  nounTargeted (EachOf grp) = nounTargeted grp
  nounTargeted (NamesAgree _ grp) = nounTargeted grp
  nounTargeted (Both l r) = nounTargeted l || nounTargeted r
  nounTargeted (EitherOf l r) = nounTargeted l || nounTargeted r
  nounTargeted (BothOf l r) = nounTargeted l || nounTargeted r
  nounTargeted (EachOfBoth p) = nounTargeted p
  nounTargeted (LibrarySlice _ _ _) = False
  nounTargeted (SomeOf _ _ grp) = nounTargeted grp
  nounTargeted TheRest = False
  nounTargeted It = False
  nounTargeted (ItAt _) = False
  nounTargeted (ItVerbed _) = False
  nounTargeted ItToken = False
  nounTargeted (ItOtherThan _ _) = False
  nounTargeted (ItPrior _ _) = False
  nounTargeted They = False
  nounTargeted Them = False
  nounTargeted (ThemVerbed _) = False
  nounTargeted (Those _) = False
  nounTargeted (That _) = False
  nounTargeted (ThatHalf _) = False
  nounTargeted (AttachHost _ _) = False
  nounTargeted (TheVerbed _ _ _) = False
  nounTargeted (ThoseVerbed _ _ _) = False
  nounTargeted (ControllerOf _) = False
  nounTargeted (OwnerOf _) = False
  nounTargeted (Designated _ _) = False

  public export
  Nontarget : Noun bs k -> Type
  Nontarget {bs} {k} n = So (not (nounTargeted n))

  ||| [CR#122.2]: counters on an object cease to exist when it moves from
  ||| one zone to another, so a clause that takes counters off a referent
  ||| an earlier clause moved names none — the object it reads is a new
  ||| one [CR#400.7]. Only the anaphors that can name a moved referent
  ||| carry the check; a description names its own.
  public export
  counterMemoryOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  counterMemoryOk It = not (stampMoves (provOfIt bs))
  counterMemoryOk (ItAt sl) = not (stampMoves (provOfItAt sl bs))
  counterMemoryOk (ItVerbed v) = not (stampMoves (provOfVerbedIt v bs))
  counterMemoryOk ItToken = not (stampMoves (provOfItToken bs))
  counterMemoryOk (ItOtherThan _ rest) = not (stampMoves (provOfIt rest))
  counterMemoryOk (ItPrior made _) = not (stampMoves (provOfIt made))
  counterMemoryOk Them = not (stampMoves (provOfThem bs))
  counterMemoryOk (ThemVerbed v) = not (stampMoves (provOfVerbedThem v bs))
  -- a participle read names a referent some labeled action MOVED, so
  -- the counters it had are gone by the same rules.
  counterMemoryOk (TheVerbed _ _ _) = False
  counterMemoryOk (ThoseVerbed _ _ _) = False
  counterMemoryOk _ = True

  public export
  CounterMemory : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  CounterMemory {bs} n = So (counterMemoryOk n)

  ||| [CR#122.5] makes a move impossible when the first and second objects
  ||| are the same object. A destination that is only the source read back
  ||| names that case, so it is refused here rather than left to the
  ||| engine.
  public export
  moveDestOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  moveDestOk It = False
  moveDestOk (ItAt _) = False
  moveDestOk (ItVerbed _) = False
  moveDestOk ItToken = False
  moveDestOk (ItOtherThan _ _) = False
  moveDestOk (ItPrior _ _) = False
  moveDestOk Them = False
  moveDestOk (ThemVerbed _) = False
  moveDestOk _ = True

  public export
  MoveDestination : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  MoveDestination {bs} n = So (moveDestOk n)

  ||| [CR#120.1]'s recipient set read off a phrase's kind, one half at a
  ||| time, as `attackableKind` reads [CR#506.3]'s. A player half takes
  ||| damage however it is described; an object half at a damageable type
  ||| and at no type at all, since a description naming none names nothing
  ||| off the set and [CR#120.1a] bounds the referent.
  public export
  damageableKind : (k : Kind) -> HeadTy k -> Bool
  damageableKind Player _ = True
  damageableKind Object (SoleTy t) = damageableHalfTy t
  damageableKind (a \/ b) (JoinTy l r) = damageableKind a l && damageableKind b r
  damageableKind (a \/ b) (SoleTy t) =
    damageableKind a (SoleTy t) && damageableKind b (SoleTy t)
  damageableKind _ _ = False

  public export
  data DamageRecipient : Noun bs k -> Type where
    PlayerTakes : DamageRecipient {k = Player} n
    ||| A joined phrase is dealt damage on either half's account:
    ||| [CR#120.1] admits a battle, a creature, a planeswalker or a player
    ||| and nothing else, and every half must name something on that list.
    ||| No zone is asked, because damage asks none — `ObjectTakes` asks for
    ||| one only where the phrase has one to give. The object half is
    ||| WIDER than `ObjectTakes`'s `DamageableTy`: a half naming no card
    ||| type passes, since "a permanent or player" is attested and names
    ||| nothing off the set, which [CR#120.1a] bounds. A half that DOES name a
    ||| type must name a damageable one, so a same-kind join such as
    ||| `Joined (HasType Land) (HasType Land)` is refused here.
    JoinTakes : {auto 0 dm : So (damageableKind (ka \/ kb) (nounTys n))} ->
                DamageRecipient {k = ka \/ kb} n
    ObjectTakes : {auto 0 field : OnBattlefield (nounZone n)} ->
                  {auto 0 dm : DamageableTy (nounTy n)} ->
                  DamageRecipient {k = Object} n

  public export
  SingleRecipient : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SingleRecipient {bs} {k} n = So (isOne (nounPlur n))

  public export
  data DiscardOk : Noun bs Object -> Type where
    DiscardThis : DiscardOk This
    DiscardTracked : {auto 0 z : nounZone n = Just Hand} -> DiscardOk n

  public export
  data Ascribable : Noun bs Object -> Type where
    AscribeThis : Ascribable This

  public export
  setZone : Maybe VerbLabel -> Maybe Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty oldZn _ og)) =
    MkBinding det Object plur (ObjectP ty z (mkStamp p oldZn (not (oldZn == z))) og)
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det Player plur ChosenPlayerP) =
    MkBinding det Player plur ChosenPlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP
  setZone p z (MkBinding det Outcome plur (OutcomeP s)) =
    MkBinding det Outcome plur (OutcomeP s)
  setZone p z (MkBinding det Gap plur GapP) = MkBinding det Gap plur GapP
  setZone p z (MkBinding det (LetterK l) plur LetterP) = MkBinding det (LetterK l) plur LetterP
  setZone p z (MkBinding det TurnRef plur TurnRefP) = MkBinding det TurnRef plur TurnRefP
  setZone p z (MkBinding det Ability plur AbilityP) = MkBinding det Ability plur AbilityP
  setZone p z (MkBinding det (a \/ b) plur (JoinP l r)) = MkBinding det (a \/ b) plur (JoinP l r)

  public export
  setZoneHead : Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  ||| A moved singular referent is re-zoned where the pronoun that names it
  ||| would find it, so the write uses the same test `zoneOfIt` reads with.
  ||| A joined binding is reached and left alone: `setZone`'s own join row
  ||| is the identity, because moving an object says nothing about the half
  ||| of the reference that was never an object [CR#400.1].
  public export
  setZoneIt : Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (b :: bs) =
    if itReaches OneOf b then setZone p z b :: bs else b :: setZoneIt p z bs

  public export
  setZoneItAt : SlotCarrier -> Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneItAt sl p z [] = []
  setZoneItAt sl p z (b :: bs) =
    if itAtReaches sl b then setZone p z b :: bs else b :: setZoneItAt sl p z bs

  public export
  setZoneVerbedIt : VerbLabel -> Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneVerbedIt v p z [] = []
  setZoneVerbedIt v p z (b :: bs) =
    if itVerbedReaches v b then setZone p z b :: bs
                           else b :: setZoneVerbedIt v p z bs

  public export
  setZoneItToken : Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneItToken p z [] = []
  setZoneItToken p z (b :: bs) =
    if itTokenReaches b then setZone p z b :: bs
                        else b :: setZoneItToken p z bs

  ||| A union half is re-zoned as the whole mention is: the pair is one
  ||| binding, and moving what one arm names moves the mention.
  public export
  setZoneUnionHalf : Maybe VerbLabel -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneUnionHalf p w z [] = []
  setZoneUnionHalf p w z (b :: bs) =
    if isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
      then setZone p z b :: bs
      else b :: setZoneUnionHalf p w z bs

  public export
  setZoneThem : Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (b :: bs) =
    if itReaches ManyOf b then setZone p z b :: bs else b :: setZoneThem p z bs

  public export
  setZoneVerbedThem : VerbLabel -> Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneVerbedThem v p z [] = []
  setZoneVerbedThem v p z (b :: bs) =
    if themVerbedReaches v b then setZone p z b :: bs
                             else b :: setZoneVerbedThem v p z bs

  public export
  setZoneThose : Maybe VerbLabel -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThose p w z [] = []
  setZoneThose p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (ManyOf, True) => setZone p z b :: bs
      _ => b :: setZoneThose p w z bs

  public export
  setZoneThat : Maybe VerbLabel -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThat p w z [] = []
  setZoneThat p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (OneOf, True) => setZone p z b :: bs
      _ => b :: setZoneThat p w z bs

  public export
  setZoneVerbed : Maybe VerbLabel -> VerbLabel -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneVerbed p v w z [] = []
  setZoneVerbed p v w z (b :: bs) =
    if verbedMatch v w b then setZone p z b :: bs else b :: setZoneVerbed p v w z bs

  public export
  setZoneManyVerbed : Maybe VerbLabel -> VerbLabel -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneManyVerbed p v w z [] = []
  setZoneManyVerbed p v w z (b :: bs) =
    if verbedMatchMany v w b then setZone p z b :: bs
                             else b :: setZoneManyVerbed p v w z bs

  ||| Re-zones a moved object, minting a fresh binding for a moved sorted
  ||| self [CR#400.7]. The leading verb stamps the binding so a later
  ||| participle read (`TheVerbed`/`ThoseVerbed`) can find it as "the
  ||| destroyed creature"/"the exiled card" [CR#701.17c].
  ||| The stamp a labeled action leaves when it changes its patient's
  ||| STATE rather than its zone -- "each creature tapped this way"
  ||| [CR#701.26a] reads exactly the mention the tap acted on. The same
  ||| write as `moveIntro`, with the zone left where it was.
  public export
  stampIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbLabel -> Noun bs k -> Bindings
  stampIntro p n = moveIntro p n (nounZone n)

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbLabel -> Noun bs k -> Maybe Zone -> Bindings
  moveIntro p nn@(Each pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(Indefinite m pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(Definite pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(TargetGroup q pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(CountedGroup q _ pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(AllOf pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p (EachOf grp) z = moveIntro p grp z
  moveIntro p (NamesAgree _ grp) z = moveIntro p grp z
  moveIntro p nn@(Both _ _) z = nomIntro nn
  moveIntro p nn@(EitherOf _ _) z = nomIntro nn
  moveIntro p nn@(BothOf _ _) z = nomIntro nn
  moveIntro p nn@(EachOfBoth _) z = nomIntro nn
  moveIntro p nn@(LibrarySlice _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(SomeOf _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p TheRest z = groupSpent bs
  moveIntro p It z = setZoneIt p z bs
  moveIntro p (ItAt sl) z = setZoneItAt sl p z bs
  moveIntro p (ItVerbed v) z = setZoneVerbedIt v p z bs
  moveIntro p ItToken z = setZoneItToken p z bs
  -- the re-mark stays INSIDE the segment the read counted, and the two
  -- halves are put back in the order the split named them: a segment
  -- read rewrites its own binding in place, exactly as the bare `It`
  -- does over the whole prefix.
  moveIntro p (ItOtherThan co rest) z = co ++ setZoneIt p z rest
  moveIntro p (ItPrior made before) z = setZoneIt p z made ++ before
  moveIntro p Them z = setZoneThem p z bs
  moveIntro p (ThemVerbed v) z = setZoneVerbedThem v p z bs
  moveIntro p (That w) z = setZoneThat p w z bs
  moveIntro p (ThatHalf w) z = setZoneUnionHalf p w z bs
  moveIntro p (Those w) z = setZoneThose p w z bs
  moveIntro p (TheVerbed v w _) z = setZoneVerbed p v w z bs
  moveIntro p (ThoseVerbed v w _) z = setZoneManyVerbed p v w z bs
  -- a moved bare self is announced like an ascribed one: `condDelta` and
  -- `selfSubjIntro` already mint `SelfD` for `AsType t This _`, and the
  -- object a cost discarded is the same object under the same
  -- determiner. It names no card type, because `This` names none.
  moveIntro p This z =
    MkBinding SelfD Object OneOf (ObjectP Nothing z (mkStamp p Nothing (isJust z)) Nothing) :: bs
  -- an attachment's host is announced where it is moved or stamped, on
  -- `selfSubjDelta`'s rows: the attach word names a referent no delta of
  -- its own mints, so a clause acting on it would otherwise leave the
  -- next sentence with nothing to read.
  moveIntro p (AttachHost _ (TypeW t)) z =
    MkBinding TheD Object OneOf
              (ObjectP (Just t) z (mkStamp p (Just Battlefield)
                                            (not (z == Just Battlefield))) Nothing)
      :: bs
  moveIntro p (AttachHost _ PermanentW) z =
    MkBinding TheD Object OneOf
              (ObjectP Nothing z (mkStamp p (Just Battlefield)
                                           (not (z == Just Battlefield))) Nothing)
      :: bs
  moveIntro p (AttachHost _ PlayerW) z = MkBinding TheD Player OneOf PlayerP :: bs
  moveIntro p (AttachHost _ w) z = bs
  -- the ascribed self is the SOURCE under a card type, so it is announced
  -- under `SelfD` exactly as the bare `This` is: "this creature" and
  -- "this" name one object, and the demonstrative noun words read
  -- neither.
  moveIntro p (AsType t This _) z =
    MkBinding SelfD Object OneOf
              (ObjectP (Just t) z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing)
      :: bs
  moveIntro p (AsType t n _) z =
    MkBinding TheD Object OneOf
              (ObjectP (Just t) z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing)
      :: bs
  moveIntro p You z = bs
  moveIntro p (PlayerGroup _) z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)
  moveIntro p (Designated d n) z = bs

  ||| The stamp the mention's antecedent carries, read like `nounZone`.
  ||| Only an ANAPHOR reports one: a stamp is written onto a binding by a
  ||| labeled action, so a phrase that describes its referent afresh names
  ||| nothing any label acted on, and a deictic names the source, which no
  ||| clause of this text has acted on either.
  public export
  nounProv : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Stamp
  nounProv TheRest = provOfGroup bs
  nounProv It = provOfIt bs
  nounProv (ItAt sl) = provOfItAt sl bs
  nounProv (ItVerbed v) = provOfVerbedIt v bs
  nounProv ItToken = provOfItToken bs
  nounProv (ItOtherThan _ rest) = provOfIt rest
  nounProv (ItPrior made _) = provOfIt made
  nounProv Them = provOfThem bs
  nounProv (ThemVerbed v) = provOfVerbedThem v bs
  nounProv (That w) = provOfThat w bs
  nounProv (ThatHalf w) = provOfUnionHalf w bs
  nounProv (Those w) = provOfThose w bs
  nounProv (TheVerbed v w _) = provOfVerbed v w bs
  nounProv (ThoseVerbed v w _) = provOfManyVerbed v w bs
  nounProv (EachOf grp) = nounProv grp
  nounProv (NamesAgree _ grp) = nounProv grp
  nounProv (SomeOf _ _ grp) = nounProv grp
  nounProv _ = Nothing

  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n _) = Just Battlefield
  nounZone You = Nothing
  nounZone (PlayerGroup _) = Nothing
  nounZone (Each p) = phraseZone p
  nounZone (Indefinite m p) = phraseZone p
  nounZone (Definite p) = phraseZone p
  nounZone (TargetGroup q p) = phraseZone p
  nounZone (CountedGroup q _ p) = phraseZone p
  nounZone (AllOf p) = phraseZone p
  nounZone (EachOf grp) = nounZone grp
  nounZone (NamesAgree _ grp) = nounZone grp
  nounZone (Both _ _) = Nothing
  nounZone (EitherOf _ _) = Nothing
  -- the pair's place is its arms' where they agree; a phrase naming
  -- two zones names no one zone [CR#109.2a].
  nounZone (BothOf l r) = if nounZone l == nounZone r then nounZone l else Nothing
  nounZone (EachOfBoth p) = nounZone p
  nounZone (LibrarySlice _ _ _) = Just Library
  nounZone (SomeOf _ _ grp) = nounZone grp
  nounZone TheRest = zoneOfGroup bs
  nounZone It = zoneOfIt bs
  nounZone (ItAt sl) = zoneOfItAt sl bs
  nounZone (ItVerbed v) = zoneOfVerbedIt v bs
  nounZone ItToken = zoneOfItToken bs
  nounZone (ItOtherThan _ rest) = zoneOfIt rest
  nounZone (ItPrior made _) = zoneOfIt made
  nounZone They = Nothing
  nounZone Them = zoneOfThem bs
  nounZone (ThemVerbed v) = zoneOfVerbedThem v bs
  nounZone (That w) = zoneOfThat w bs
  nounZone (ThatHalf w) = zoneOfUnionHalf w bs
  nounZone (AttachHost _ h) = attachHostZone h
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w _) = zoneOfVerbed v w bs
  nounZone (ThoseVerbed v w _) = zoneOfManyVerbed v w bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing
  -- [CR#903.3]: the designation is an attribute of the card, so it
  -- survives a zone change and the noun names no zone.
  nounZone (Designated _ _) = Nothing

  public export
  nounTy : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe CardType
  nounTy This = Nothing
  nounTy (AsType t n _) = Just t
  nounTy You = Nothing
  nounTy (PlayerGroup _) = Nothing
  nounTy (Each p) = seedTy p
  nounTy (Indefinite m p) = seedTy p
  nounTy (Definite p) = seedTy p
  nounTy (TargetGroup q p) = seedTy p
  nounTy (CountedGroup q _ p) = seedTy p
  nounTy (AllOf p) = seedTy p
  nounTy (EachOf grp) = nounTy grp
  nounTy (NamesAgree _ grp) = nounTy grp
  nounTy (Both _ _) = Nothing
  nounTy (EitherOf _ _) = Nothing
  nounTy (BothOf l r) = if nounTy l == nounTy r then nounTy l else Nothing
  nounTy (EachOfBoth p) = nounTy p
  nounTy (LibrarySlice _ _ _) = Nothing
  nounTy (SomeOf _ d grp) = sliceTy d grp
  nounTy TheRest = tyOfGroup bs
  nounTy It = tyOfIt bs
  nounTy (ItAt sl) = tyOfItAt sl bs
  nounTy (ItVerbed v) = tyOfVerbedIt v bs
  nounTy ItToken = tyOfItToken bs
  nounTy (ItOtherThan _ rest) = tyOfIt rest
  nounTy (ItPrior made _) = tyOfIt made
  nounTy They = Nothing
  nounTy Them = tyOfThem bs
  nounTy (ThemVerbed v) = tyOfVerbedThem v bs
  nounTy (That w) = tyOfThat w bs
  nounTy (ThatHalf w) = tyOfUnionHalf w bs
  nounTy (AttachHost _ h) = attachHostTy h
  nounTy (Those w) = tyOfThose w bs
  nounTy (TheVerbed v w _) = tyOfVerbed v w bs
  nounTy (ThoseVerbed v w _) = tyOfManyVerbed v w bs
  nounTy (ControllerOf n) = Nothing
  nounTy (OwnerOf n) = Nothing
  nounTy (Designated _ _) = Nothing

  ||| `nounTy`'s per-half twin: the head type the phrase projects onto EACH
  ||| half of its kind. A determiner over a joined head passes the head's
  ||| pair through, a mixed group takes one from each arm, and every other
  ||| noun names one description — an anaphor included, since a binding
  ||| remembers one type per mention and `nounTy` is what reads it back.
  public export
  nounTys : {bs : Bindings} -> {k : Kind} -> Noun bs k -> HeadTy k
  nounTys (Each p) = seedTys p
  nounTys (Indefinite m p) = seedTys p
  nounTys (Definite p) = seedTys p
  nounTys (TargetGroup q p) = seedTys p
  nounTys (CountedGroup q _ p) = seedTys p
  nounTys (AllOf p) = seedTys p
  nounTys (EachOf grp) = nounTys grp
  nounTys (NamesAgree _ grp) = nounTys grp
  nounTys (SomeOf _ d grp) = SoleTy (sliceTy d grp)
  nounTys (Both l r) = JoinTy (nounTys l) (nounTys r)
  -- the pair is same-kinded and so names one description, its arms'
  -- where they agree; `SoleTy`, never `JoinTy`, which indexes a join.
  nounTys (BothOf l r) = SoleTy (if nounTy l == nounTy r then nounTy l else Nothing)
  nounTys (EachOfBoth p) = nounTys p
  nounTys n = SoleTy (nounTy n)

  public export
  nounPlur : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Plurality
  nounPlur This = OneOf
  nounPlur (AsType t n _) = nounPlur n
  nounPlur You = OneOf
  nounPlur (PlayerGroup _) = ManyOf
  nounPlur (Each p) = ManyOf
  nounPlur (Indefinite m p) = OneOf
  nounPlur (Definite p) = OneOf
  nounPlur (TargetGroup q p) = quantPlur q
  nounPlur (CountedGroup q _ p) = quantPlur q
  nounPlur (AllOf p) = ManyOf
  nounPlur (EachOf grp) = ManyOf
  nounPlur (NamesAgree _ grp) = nounPlur grp
  nounPlur (Both _ _) = ManyOf
  nounPlur (BothOf _ _) = ManyOf
  nounPlur (EachOfBoth _) = ManyOf
  nounPlur (EitherOf l r) = nounPlur l
  nounPlur (LibrarySlice _ amt whose) = outputPlur (nounPlur whose) (amtPlur amt)
  nounPlur (SomeOf q _ _) = quantPlur q
  nounPlur TheRest = ManyOf
  nounPlur It = OneOf
  nounPlur (ItAt _) = OneOf
  nounPlur (ItVerbed _) = OneOf
  nounPlur ItToken = OneOf
  nounPlur (ItOtherThan _ _) = OneOf
  nounPlur (ItPrior _ _) = OneOf
  nounPlur They = OneOf
  nounPlur Them = ManyOf
  nounPlur (ThemVerbed _) = ManyOf
  nounPlur (That w) = OneOf
  nounPlur (ThatHalf w) = OneOf
  nounPlur (AttachHost _ _) = OneOf
  nounPlur (Those w) = ManyOf
  nounPlur (TheVerbed v w _) = OneOf
  nounPlur (ThoseVerbed v w _) = ManyOf
  nounPlur (ControllerOf n) = OneOf
  nounPlur (OwnerOf n) = OneOf
  nounPlur (Designated _ _) = OneOf

  ||| [CR#500.1] runs every phase and step on every turn, so a part in
  ||| a named player's turn picks out a real span. A bare turn picks out
  ||| none: every moment of the game is during some turn. And a window
  ||| introduces no turn, so the deictic possessor reaches no antecedent.
  public export
  windowOk : TurnPart -> Maybe Owner -> Bool
  windowOk Turn Nothing = False
  windowOk _ (Just ThatTurns) = False
  windowOk _ _ = True

  public export
  WindowOk : TurnPart -> Maybe Owner -> Type
  WindowOk p w = So (windowOk p w)

  ||| Whose turn a boundary-relative window falls in. An activation
  ||| restriction introduces no turn, so the deictic possessor reaches no
  ||| antecedent; every quantifier word names a turn on its own.
  public export
  pointWindowOk : TurnPoint -> Maybe Owner -> Bool
  pointWindowOk _ (Just ThatTurns) = False
  pointWindowOk _ _ = True

  public export
  PointWindowOk : TurnPoint -> Maybe Owner -> Type
  PointWindowOk pt w = So (pointWindowOk pt w)
