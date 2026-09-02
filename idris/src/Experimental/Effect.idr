||| The clause core: cost, effect, static-effect, and ability layers built
||| over the trigger algebra.
module Experimental.Effect

import public Experimental.Triggers

%default total

||| What a spend restriction [CR#106.6] lets its mana be spent on. The
||| list `SpendOnly` carries is the DISJUNCTION printed lines write
||| ("to cast an artifact spell or activate an ability of an artifact"),
||| so an arm is one way of naming a purpose and not one whole line.
|||
||| Two of the three name an OBJECT -- the spell being cast, the source
||| whose ability is activated -- because [CR#106.1] names casting and
||| activating as the usual occasions for spending mana. The third
||| names the COST, which is what that same rule says mana is actually
||| spent to pay, and what the object-taking arms cannot reach: a cost
||| is not an object and no predicate describes one.
|||
||| The zone-qualified lines are NOT a fourth arm and need nothing
||| here. "Spend this mana only to cast spells from your graveyard"
||| (Rootcoil Creeper, Lord of the Forsaken), "only to cast spells from
||| exile" (Interdimensional Web Watch), "only to cast a spell from
||| anywhere other than your hand" (Mm'menon, the Right Hand) and the
||| two negative hand lines (Karolina Dean, Vhal) -- 7 supported lines,
||| measured 2026-08-28 -- describe the SPELL by where it was cast
||| from, which `CastFrom` already says of an object: [CR#601.2a] moves
||| the card to the stack before [CR#601.2h] takes the payment, so the
||| provenance is fixed by the time the restriction is tested.
public export
data SpendPurpose : Bindings -> Type where
  ToCast : (p : Predicate bs Object) ->
           SpendPurpose bs
  ToActivate : (src : Maybe (Predicate bs Object)) ->
               SpendPurpose bs
  ||| "Spend this mana only on costs that contain {X}" (Rosheen
  ||| Meanderer), "only to pay cumulative upkeep costs" (Adarkar
  ||| Unicorn), "only to turn permanents face up" (Overgrown Zealot):
  ||| the purpose that names a COST rather than the object whose cost
  ||| it is. 15 supported lines of 204, measured 2026-08-28;
  ||| `CostNamed` carries the three ways they pick a cost out and the
  ||| count of each.
  |||
  ||| The landed `Keyword.CumulativeUpkeep` row does not unblock
  ||| Adarkar Unicorn on its own: that row is what a permanent PRINTS,
  ||| where this names the cost such a permanent's ability charges.
  ||| -- spelling: "to pay [c]", "on costs that contain [c]".
  ToPay : (c : CostNamed) -> {auto 0 nm : CostNameable c} ->
          SpendPurpose bs

||| The mana a persistence or pool sentence is ABOUT. Not a `Noun`, and
||| the omission is deliberate: [CR#106.1] makes mana "the primary
||| resource in the game" and no rule makes it an object or a player, so
||| no `Kind` names it and none should. What the two arms have in common
||| is that each picks out mana in a pool [CR#106.4] -- one by pointing
||| back at the clause that put it there, one by describing it.
public export
data ManaHeld : Bindings -> Type where
  ||| "this mana": the mana a preceding sentence added. Gated on exactly
  ||| one `ManaAdded` mention standing in the prefix, which is the
  ||| `TheTotal`/`CoinsShowing` idiom and not a new one -- an outcome a
  ||| clause left, read by the sentence after it. All 25 supported
  ||| persistence lines that write it sit after an add in the same
  ||| ability (measured 2026-08-28).
  ThisMana : {auto 0 ok : countOutcomes ManaAdded bs = 1} -> ManaHeld bs
  ||| "unspent mana", "unspent red mana": mana in a pool, described
  ||| rather than pointed at, and ungated because the description needs
  ||| no antecedent. [CR#106.4] is the phrase's own rule and names the
  ||| ERRATA that produced it -- "cards with abilities that produce mana
  ||| or refer to unspent mana have received errata ... to no longer
  ||| explicitly refer to the mana pool" -- which is why 0 supported
  ||| lines write "mana pool" and this row spells no pool either.
  UnspentMana : (ty : Maybe ColorOrColorless) -> ManaHeld bs

||| [CR#609.4]'s counterfactual, the premise a permission is scoped by:
||| "treat the game exactly as if the stated condition were true. For
||| all other purposes, treat the game normally." It is a SLOT on the
||| deontic row and never a node above one -- the permission is the
||| head and the counterfactual its rider -- because a node above would
||| admit pairings the rule cannot express, a free-floating May or
||| Can't under an unrelated premise. [CR#609.4a]'s stacked as-thoughs
||| are two permissions each carrying its own rider, not one nesting.
|||
||| The premise is a general `Predicate`, not a closed list of negated
||| keywords: the corpus writes six unrelated payload families over the
||| same 264 sentences -- a keyword ("as though it didn't have
||| defender"), a characteristic ("as though its power were 2
||| greater"), a combat fact ("as though it weren't blocked"), a status
||| ("as though they were untapped"), counters ("as though it had no
||| +1/+1 counters on it") and existence -- and each is already a
||| `Predicate bs Object` here, with `Not` and its `Negatable` gate
||| supplying the negation. [CR#609.4] states no restriction on the
||| condition, so a closed payload would refuse rules-meaningful
||| sentences and pin nothing.
|||
||| THREE arms, one per premise SORT, and which sort a deed admits is the
||| deed's own fact (`deedPremiseSort`) rather than a free choice here:
||| [CR#609.4b] gives spending mana "as though it were mana of any [type
||| or color]" its own rule and its own payload -- a mana matcher, not a
||| predicate over an object -- so a permission of the spend deed reads
||| the second arm and every other deed reads the first. Admitting either
||| payload at either deed would make "this creature can attack as though
||| it were mana of any color" spellable and pin nothing.
||| -- spelling: "as though [p]"; at the spend deed, "as though it were
||| [as]".
public export
data AsThough : Bindings -> Type where
  AsThoughOf : (p : Predicate bs Object) -> AsThough bs
  ||| "You may spend mana as though it were mana of any color to cast
  ||| that spell": [CR#609.4b]'s PAYMENT permission, and the whole of
  ||| what widens what already-made mana may pay for. 83 supported
  ||| sentences over two spellings (measured 2026-08-28) -- 50 write the
  ||| counterfactual outright and 33 write [CR#118.14]'s passive, "mana
  ||| of any type can be spent to cast that spell".
  |||
  ||| ONE row for both spellings, because the rules make them one thing
  ||| and say so: [CR#118.14] glosses "mana of any type can be spent" as
  ||| "players may spend mana as though it were colorless mana or mana of
  ||| any color to pay that cost", and [CR#609.4b] closes with "the same
  ||| is true for effects that say 'mana of any type can be spent'". The
  ||| passive is this counterfactual with its agent unwritten, which is
  ||| spelling -- exactly as the tapped-for-mana header's two voices are.
  |||
  ||| IT IS NOT `ManaRider`/`SpendOnly`, and the distinction is
  ||| [CR#106.6]'s against [CR#609.4b]'s. A spend restriction is attached
  ||| to mana AS IT IS PRODUCED and narrows what that mana may do; this is
  ||| a permission over mana already made, by whatever produced it, and it
  ||| widens. They differ in what they hang on as well as in direction:
  ||| the restriction rides the production, this rides a permission with a
  ||| subject of its own.
  |||
  ||| `what` is the mana the permission reaches -- `Nothing` where the
  ||| line leaves it at all mana (46 of 50) and a type where it names one
  ||| ("you may spend WHITE mana as though it were mana of any color", 4
  ||| lines). It is not a `DeonticCounterpart`: no `Kind` names mana, and
  ||| what it names is the deed's own object rather than a second
  ||| participant.
  |||
  ||| `purpose` is what the widened mana may be spent ON, and it is
  ||| `SpendPurpose` -- the same vocabulary the restriction names its
  ||| purposes with, which is what the two families share even though they
  ||| share no carrier. 46 of the 50 write one ("to cast that spell" and
  ||| its kin 39, "to activate" 7) and 4 write none. [CR#609.4b] is why it
  ||| rides the premise rather than the carrier: the permission "affects
  ||| only how the player may pay A COST", so the cost it is scoped to is
  ||| part of what the counterfactual says and not a separate statement.
  ||| -- spelling: "[who] may spend [what] mana as though it were [as]
  ||| [purpose]"; in the passive, "[as] can be spent [purpose]".
  AsThoughMana : (what : Maybe ColorOrColorless) ->
                 (as : ManaMatch) ->
                 (purpose : Maybe (SpendPurpose bs)) -> AsThough bs
  ||| "This creature saddles Mounts and crews Vehicles as though its
  ||| power were 2 greater": the VALUE counterfactual, and the whole of
  ||| what the crew/saddle permissions say. 18 supported lines write it
  ||| (measured 2026-09-02) -- 4 of them the printed static and 14 the
  ||| same sentence quoted inside a created Pilot token's text -- and all
  ||| 18 write the same characteristic and the same shift.
  |||
  ||| A THIRD ARM and not a payload of `AsThoughOf`: a `Predicate bs
  ||| Object` says what an object IS, and no predicate states a
  ||| characteristic SHIFTED by an amount. What the sentence shifts is
  ||| the quantity the deed's own cost is counted in -- [CR#702.122a]
  ||| makes the crew cost a total power and [CR#702.171a] the saddle cost
  ||| -- so the premise names a `Characteristic` and an amount, which is
  ||| what [CR#609.4]'s "treat the game exactly as if the stated
  ||| condition were true" has to be told here.
  |||
  ||| ONE DIRECTION, minted because it is the one printed: the arm says
  ||| "greater" in its name rather than carrying a sign, on `LoseCause`'s
  ||| law -- the rule states no restriction on the condition and a
  ||| printed "less" would mint its own arm beside this one, where a
  ||| direction slot would spell an axis no line writes.
  ||| The amount ANNOUNCES NOTHING, and for `MayPlayAdditionalLands`'
  ||| reason: the premise introduces no mention of its own, so an amount
  ||| that announced one would be announced nowhere.
  ||| -- spelling: "as though its [ch] were [amt] greater".
  AsThoughGreater : {bs : Bindings} -> (ch : Characteristic) ->
                    (amt : Amount bs) ->
                    {auto 0 nd : So (isNil (amtDelta amt))} -> AsThough bs

||| The premise's own sort, which the deed's rule must admit.
public export
asThoughSort : {0 bs : Bindings} -> AsThough bs -> PremiseSort
asThoughSort (AsThoughOf _) = ObjectPremise
asThoughSort (AsThoughMana _ _ _) = ManaPremise
asThoughSort (AsThoughGreater _ _) = ValuePremise


||| The pair an exchange runs between. [CR#701.12c] gives the effect
||| exactly two participants -- "each player gains or loses the amount of
||| life necessary to equal THE OTHER player's previous life total" -- and
||| the corpus writes that pair two ways: as a coordination naming each
||| party ("exchange life totals WITH target opponent", whose first party
||| is the unwritten "you") and as one mention counted at two ("TWO TARGET
||| PLAYERS exchange life totals"). One slot serves both, because the row
||| asks for two players and not for a particular way of naming them.
||| A coordination is admitted only when both arms are singular, since a
||| plural arm would put more than two in the exchange; a counted mention
||| is admitted only at an exactly-written two, since [CR#701.12a] refuses
||| an exchange it cannot complete in full and a range states no pair.
public export
twoPartiesOk : {bs : Bindings} -> Noun bs Player -> Bool
twoPartiesOk (BothOf l r) = case (nounPlur l, nounPlur {bs = nomIntro l} r) of
  (OneOf, OneOf) => True
  _ => False
twoPartiesOk (TargetGroup q _) = quantExact q == Just 2
twoPartiesOk (CountedGroup q _ _) = quantExact q == Just 2
twoPartiesOk _ = False


mutual
  public export
  record TokenChars (bs : Bindings) where
    constructor MkToken
    pt : Maybe (p : Amount bs ** Amount (amtIntro p))
    colors : List Color
    line : TypeLine
    abilities : List (AbilityAt [])
    name : Maybe String

  public export
  tokenTyped : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenTyped t = lineNonEmpty (MkTypeLine [] t.line.tys)

  public export
  ptWritten : {0 bs : Bindings} -> Maybe (p : Amount bs ** Amount (amtIntro p)) -> Bool
  ptWritten Nothing = False
  ptWritten (Just _) = True

  public export
  tokenPtOk : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenPtOk t = not (elem Creature t.line.tys) || ptWritten t.pt

  public export
  TokenTyped : TokenChars bs -> Type
  TokenTyped {bs} t = So (tokenTyped t)

  public export
  TokenPt : TokenChars bs -> Type
  TokenPt {bs} t = So (tokenPtOk t)

  public export
  SubtypesFit : TokenChars bs -> Type
  SubtypesFit {bs} t = So (subsFitLine t.line.subs t.line.tys)

  public export
  tokenCanonical : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenCanonical t = colorsDistinct t.colors && typesDistinct t.line.tys

  public export
  TokenCanonical : TokenChars bs -> Type
  TokenCanonical {bs} t = So (tokenCanonical t)

  public export
  additionUnnamed : {0 bs : Bindings} -> TokenChars bs -> Bool
  additionUnnamed t = isNothing t.name

  public export
  AdditionUnnamed : TokenChars bs -> Type
  AdditionUnnamed {bs} t = So (additionUnnamed t)

  public export
  someWritten : {0 a : Type} -> List a -> Bool
  someWritten [] = False
  someWritten (_ :: _) = True

  ||| What an addition line has to SAY, asked of the whole bundle where
  ||| `LineNonEmpty` and `AddsSomething` asked the type line alone.
  ||| [CR#613.1d] and [CR#613.1e] are different layers, so "becomes blue
  ||| in addition to its other colors" (Indigo Faerie) adds at layer 5
  ||| and names no type at all. Neither line-level refusal is dropped: a
  ||| line that is WRITTEN must still add a type the subject does not
  ||| already have, and the only bundle admitted without one is a bundle
  ||| that writes a colour. A P/T or a with-clause ability alone is still
  ||| refused -- [CR#613.4b]'s base P/T is `HasBasePt`'s statement and a
  ||| granted ability is `Gains`', and neither is printed without a type
  ||| word in an addition sentence.
  public export
  additionSaysSomething : {0 bs : Bindings} -> Maybe CardType ->
                          TokenChars bs -> Bool
  additionSaysSomething subj t =
    addsSomething subj t.line ||
      (not (lineNonEmpty t.line) && someWritten t.colors)

  public export
  AdditionSaysSomething : Maybe CardType -> TokenChars bs -> Type
  AdditionSaysSomething {bs} subj t = So (additionSaysSomething subj t)


  public export
  tokenHeadTy : {0 bs : Bindings} -> TokenChars bs -> Maybe CardType
  tokenHeadTy t = lastType t.line.tys

  public export
  data TokenSpec : Bindings -> Type where
    TokenWritten : (t : TokenChars bs) ->
                   {auto 0 tt : TokenTyped t} ->
                   {auto 0 tp : TokenPt t} ->
                   {auto 0 sf : SubtypesFit t} ->
                   {auto 0 ta : TokenAbilities t} ->
                   {auto 0 tc : TokenCanonical t} -> TokenSpec bs
    ||| The anaphoric specification: the create clause reads back a
    ||| definition of characteristics an earlier clause wrote [CR#111.3],
    ||| never the objects that definition made. NOT gated on the
    ||| antecedent's plurality -- one token carries a definition as a
    ||| batch does, and English spells the read to agree with what it
    ||| found ("those tokens" after a batch, "that token" after one),
    ||| which is spelling and not a second constructor.
    ||| -- spelling: "[count] of those tokens"; after a singular
    ||| antecedent, "that token".
    TokenAsThose : {auto 0 ok : countTokenSpecs bs = 1} -> TokenSpec bs
    TokenCopyOf : (src : Noun bs Object) -> (exc : List (CopyExcept bs)) ->
                  {auto 0 pm : PerMember src} -> TokenSpec bs

  public export
  specHeadTy : {bs : Bindings} -> TokenSpec bs -> Maybe CardType
  specHeadTy (TokenWritten t) = tokenHeadTy t
  specHeadTy TokenAsThose = tyOfThoseAny TokenW bs
  specHeadTy (TokenCopyOf src _) = nounTy src

  ||| What a written token's P/T amounts mention; a copy's source is left
  ||| out, since the copy clause already names it and a second mention
  ||| would double the pronoun's antecedents.
  public export
  ptDelta : {bs : Bindings} -> Maybe (p : Amount bs ** Amount (amtIntro p)) ->
            List Binding
  ptDelta Nothing = []
  ptDelta (Just (p ** t)) = amtDelta t ++ amtDelta p

  public export
  specDelta : {bs : Bindings} -> TokenSpec bs -> List Binding
  specDelta (TokenWritten t) = ptDelta t.pt
  specDelta TokenAsThose = []
  specDelta (TokenCopyOf _ _) = []

  public export
  data PtShift : Bindings -> Type where
    PtUp : (amt : Amount bs) -> PtShift bs
    PtDown : (amt : Amount bs) -> PtShift bs

  public export
  shiftAmount : {0 bs : Bindings} -> PtShift bs -> Amount bs
  shiftAmount (PtUp a) = a
  shiftAmount (PtDown a) = a

  public export
  shiftRises : {0 bs : Bindings} -> PtShift bs -> Bool
  shiftRises (PtUp _) = True
  shiftRises (PtDown _) = False

  public export
  shiftDelta : {bs : Bindings} -> PtShift bs -> List Binding
  shiftDelta s = amtDelta (shiftAmount s)

  public export
  shiftIntro : {bs : Bindings} -> PtShift bs -> Bindings
  shiftIntro s = amtIntro (shiftAmount s)

  public export
  writtenZero : {0 bs : Bindings} -> Amount bs -> Bool
  writtenZero (Lit Z) = True
  writtenZero _ = False

  ||| Direction agreement, not structural equality: no `Eq` instance.
  public export
  sameDirection : {0 bs : Bindings} -> PtShift bs -> PtShift bs -> Bool
  sameDirection p t = if shiftRises p then shiftRises t else not (shiftRises t)

  public export
  data CostShift : Bindings -> Type where
    ||| "[n] cost [amt] less to cast" / "... to activate", with the
    ||| printed FLOOR some cards write beside it: "This effect can't
    ||| reduce the mana in that cost to less than one mana" -- 10
    ||| supported cards, measured 2026-08-28 (Training Grounds,
    ||| Heartstone, Biomancer's Familiar, Forensic Gadgeteer,
    ||| Convergence of Dominion, Power Artifact, Agatha of the Vile
    ||| Cauldron, Spikeshell Harrier, Valiant Changeling, Zirda).
    |||
    ||| A SLOT on the reduction and not a prohibition of its own. The
    ||| sentence forbids nothing an agent does; it bounds this effect's
    ||| amount, which is why it is here and not on the deontic carrier.
    ||| Nor is it the rules' own floor restated: [CR#601.2f] already
    ||| stops a total cost at {0} ("It can't be reduced to less than
    ||| {0}"), so a line reading "to less than one mana" states a HIGHER
    ||| bound that only the card gives, and one no rule would supply.
    ||| -- spelling: the reduction, then "This effect can't reduce the
    ||| mana in that cost to less than [floor]."
    CostLess : (amt : Amount bs) -> (floor : Maybe (Amount bs)) -> CostShift bs
    CostMore : (amt : Amount bs) -> CostShift bs
    ||| "[n] cost {2}{R} more to cast" / "cost {W}{B} less to cast": the
    ||| shift written as a MANA RUN rather than as a number. A SECOND
    ||| payload beside `Amount`, never a widening of it: the letter has
    ||| to stay readable at the amount arms, where "this spell costs {X}
    ||| less to cast" (Ghalta, Cavern-Hoard Dragon) puts a `LetterVal`
    ||| and a `Define` reads it back, and a run holds no letter for
    ||| `amtDelta` to open.
    |||
    ||| It is the shift's payload and not a `Cost`, on [CR#601.2f]'s
    ||| ground: what a reduction subtracts from is the total cost, so
    ||| the sentence states an amount of mana and never an action a
    ||| player takes, which is what `Cost` is [CR#118.1].
    ||| 46 supported lines write a run with at least one non-generic
    ||| symbol (measured 2026-08-28): 25 of them are strive's "costs
    ||| [run] more to cast for each target beyond the first", the rest
    ||| ordinary shifts (Alabaster Leech, Derelor, Jade Leech, the five
    ||| Defilers, Edgewalker, Bard Class, Avatar Aang).
    ||| NO FLOOR ARM: no supported line pairs a run payload with the
    ||| "can't reduce ... to less than" rider, so the floor stays where
    ||| its ten cards write it.
    ||| WHAT A RUN PAYLOAD STILL MAY NOT SAY: Bard Class and Edgewalker
    ||| print "This effect reduces only the amount of colored mana you
    ||| pay" beside the run -- 2 lines, a rider on the reduction and not
    ||| a floor, recorded here and not minted.
    ||| -- spelling: "[n] cost[s] [run] more/less to cast".
    CostShiftRun : (run : ManaCost) -> (rises : Bool) ->
                   {auto 0 wr : ManaRun run} -> CostShift bs

  ||| The letters a shift's payload opens. A run opens none: a mana run
  ||| is written symbols and holds no `Amount` for `amtDelta` to read.
  public export
  costShiftDelta : {bs : Bindings} -> CostShift bs -> List Binding
  costShiftDelta (CostLess a _) = amtDelta a
  costShiftDelta (CostMore a) = amtDelta a
  costShiftDelta (CostShiftRun _ _) = []

  namespace Static
    public export
    data StaticEffect : Bindings -> Type where
      ||| The shift amounts sit at `selfSubjIntro n`, not `nomIntro n`:
      ||| the subject is written before them and a deictic one announces
      ||| itself, so "gets +2/+2 for each Aura attached to IT"
      ||| (Auramancer's Guise) reads the enchanted creature its own
      ||| statement named. That is the context `staticIntro` has always
      ||| exported for this row; the slots were the drift.
      Gets : (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
             (tou : PtShift (shiftIntro pow)) ->
             {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
             StaticEffect bs
      ||| `Gets`' move, at the defining amount: "becomes an artifact
      ||| creature with power and toughness each equal to ITS mana value"
      ||| (Titania's Song).
      DefinesPt : (n : Noun bs Object) -> (sl : DefinedSlots) ->
                  (amt : Amount (selfSubjIntro n)) ->
                  {auto 0 sd : SelfDefined n} ->
                  StaticEffect bs
      HasBasePt : (n : Noun bs Object) -> (pow : Amount bs) ->
                  (tou : Amount (amtIntro pow)) -> StaticEffect bs
      SwitchesPt : (n : Noun bs Object) ->
                   {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                   StaticEffect bs
      CostsToCast : {k : Kind} -> (n : Noun bs k) -> (sh : CostShift bs) ->
                    {auto 0 cs : CostSubject n} ->
                    StaticEffect bs
      ||| "[c] rather than pay this spell's mana cost" [CR#118.9], with
      ||| `Nothing` for the whole substitution ("without paying its mana
      ||| cost"). 107 supported self lines; 16 of them write the second
      ||| phrasing.
      |||
      ||| THREE COUNTED POPULATIONS STAY OUTSIDE IT, each refused by a
      ||| different slot, re-measured 2026-08-28 (the alternative-cost
      ||| round's own counts are corrected where they differ):
      |||
      ||| * 13 GENERIC GRANTS -- "rather than pay the mana cost FOR
      |||   [spells you cast]" (As Foretold, Fist of Suns, Dream Halls,
      |||   Jodah, Rooftop Storm, ...). They price a described CLASS of
      |||   spells, which is a subject slot this row has none of and
      |||   `CostsToCast` has. Counted 14 before; 13 on the re-measure.
      ||| * 11 NON-MANA DECLINED COSTS -- "you may pay {0} rather than
      |||   pay the equip cost" (Bruenor Battlehammer, Forge Anew), the
      |||   cycling, echo, power-up and crew ones, and K'rrik's per-pip
      |||   life swap. What they decline is a COST NAMED BY A KEYWORD or
      |||   a pip inside one, not the mana cost, so the declined side
      |||   wants to be a value -- `Pay`'s standing decline.
      ||| * 23 PRONOUN-TAIL LINES -- "you may cast this card from your
      |||   graveyard BY PAYING [c] rather than paying its mana cost"
      |||   (Worldheart Phoenix, Squee, Raffine's Guidance, Bolas's
      |||   Citadel). Those are play permissions with an alternative-cost
      |||   rider, so they are the play rider's: `PlayPayment`'s third,
      |||   cost-carrying arm and not this row's.
      |||
      ||| The other two the round left refused have since written with
      ||| nothing minted here, and are benched rather than counted:
      ||| Invigorate (`costActionOk` already admits the life gain) and
      ||| the 5 commander-gated free spells (`HasCardDesignation`
      ||| [CR#903.3] reads `CommanderD`'s card-held scope).
      ||| -- spelling: the cost, then "rather than pay this spell's mana
      ||| cost"; at `Nothing`, "without paying its mana cost".
      AltCost : (c : Maybe (Cost bs)) ->
                {auto 0 ap : AltPayment c} -> StaticEffect bs
      ||| "As an additional cost to cast this spell, [c]" [CR#118.8] --
      ||| 308 supported lines, 260 of them mandatory and 48 written with
      ||| "you may", which is what `offered` marks [CR#118.8b]. Beside
      ||| `AltCost` and not a mode of it: [CR#118.8] makes this a cost
      ||| paid "at the same time they pay the spell's mana cost" where
      ||| [CR#118.9] replaces that cost, and [CR#118.8d] keeps the mana
      ||| cost itself untouched, so the two rows say opposite things
      ||| about the same payment and a card may print both.
      ||| The offer is a slot rather than a `May` around the cost because
      ||| [CR#118.8a] announces the intention to pay at [CR#601.2b],
      ||| before any effect runs -- there is no clause here to wrap.
      ||| It names no object for the same reason `AltCost` does not: the
      ||| 7 lines whose subject is a CLASS of spells ("As an additional
      ||| cost to cast creature spells", Chorus of the Conclave and the
      ||| five Defilers; Molten Exhale writes the phrase as a postposed
      ||| rider) want the subject slot `CostsToCast` has and this row has
      ||| none of, which is the generic grants' gap and not this one's.
      ||| WHAT A CARD MAY STILL NOT READ OFF IT: what the cost's ACTION
      ||| did. 13 supported lines pair an any-number additional cost with
      ||| a for-each reduction that counts it ("you may sacrifice any
      ||| number of artifacts and/or creatures. This spell costs {2} less
      ||| to cast for each permanent sacrificed this way", Dargo), and
      ||| Burn at the Stake reads the same stamp from its damage line.
      ||| `staticChoiceDelta` carries a CHOOSER across the ability
      ||| boundary [CR#607.2d] and not a whole delta, so those reads wait.
      ||| -- spelling: "As an additional cost to cast this spell, [c]";
      ||| under `offered`, "..., you may [c]".
      AddedCost : (c : Cost bs) -> (offered : Bool) ->
                  {auto 0 ap : AddedPayment c} -> StaticEffect bs
      ||| The static twin of the clause-level `Define`: "[se], where [l] is
      ||| [amt]" as one member of an `AndAlso` after the statement that used
      ||| the letter.
      ||| -- spelling: as `Define`.
      Define : (l : Letter) -> (amt : Amount bs) ->
               {auto 0 ok : So (anyOpenLetter l bs)} -> StaticEffect bs
      Gains : (n : Noun bs Object) -> (ab : AbilityAt bs) ->
              {auto 0 ok : GrantSubject ab n} ->
              {auto 0 gr : Grantable ab} -> StaticEffect bs
      ||| The grant whose payload is DESCRIBED rather than quoted: "this
      ||| creature has all activated abilities of that card" (Conspicuous
      ||| Snoop, Skill Borrower), "each other planeswalker you control
      ||| has the loyalty abilities of Kasmina", "Nicol Bolas has all
      ||| loyalty abilities of all other planeswalkers on the
      ||| battlefield", "Koh has all activated and triggered abilities of
      ||| the last chosen card". [CR#613.1f]'s layer 6 again, and the
      ||| same operation `Gains` performs -- what differs is that no
      ||| ability is written down. `Gains` takes an `AbilityAt`, which is
      ||| a payload the card SPELLS; here the payload is a set picked out
      ||| of another object by class, and the abilities it names are
      ||| whatever that object has when the effect applies.
      |||
      ||| 29 supported lines write it (measured 2026-08-28) and the
      ||| description vocabulary they use is `AbilityClass`' own, already
      ||| minted for the ability on the stack: 26 write "all activated
      ||| abilities", 2 "all activated and triggered abilities" (Koh,
      ||| Idris, Soul of the TARDIS) and 2 the loyalty class (Kasmina's
      ||| "the loyalty abilities of", Nicol Bolas' "all loyalty abilities
      ||| of"). The determiner is spelling. The class slot is a LIST on
      ||| `Deontic`'s ground -- the English coordinates class words
      ||| inside one description -- and nothing is refused by count:
      ||| every arm of the vocabulary is a class of abilities an object
      ||| can have.
      |||
      ||| The EXCEPTION carves a class back out: Scheming Fence's "except
      ||| for loyalty abilities" and Sharkey, Tyrant of the Shire's
      ||| "except mana abilities" are the two supported lines. It is a
      ||| `Predicate` over `Ability` and not a second class list, because
      ||| the two printed exceptions are not both classes: "loyalty
      ||| abilities" is `AbilityHead LoyaltyClass` and a mana ability is
      ||| [CR#605.1a]'s derived property, which `IsManaAbility` already
      ||| spells. One description vocabulary serves both.
      |||
      ||| NO subject-zone demand. [CR#113.6b] makes a granted ability
      ||| function from the zones it names, so a grant may name a subject
      ||| in any zone; that every supported subject here is on the
      ||| battlefield is a count and not a rule.
      ||| -- spelling: "[n] has [cls] abilities of [src]", the classes
      ||| coordinated with "and"; with an exception, "except [except]".
      GainsAbilitiesOf : (n : Noun bs Object) ->
                         (cls : List AbilityClass) ->
                         (src : Noun (nomIntro n) Object) ->
                         (except : Maybe (Predicate (nomIntro src) Ability)) ->
                         {auto 0 ne : NonEmpty cls} ->
                         {auto 0 dc : So (distinctClasses cls)} ->
                         StaticEffect bs
      ||| THE deontic carrier: one subject, one modality, one or more
      ||| deeds, one role, an optional other participant and an optional
      ||| [CR#609.4] counterfactual. It is the whole modal algebra --
      ||| May, Can't, Must and Gate -- over the OPEN deed vocabulary, so
      ||| "this creature can attack", "creatures you control can't
      ||| attack", "this creature must be blocked if able" and "this
      ||| creature can't attack unless you pay {1}" are one row at four
      ||| `Compulsion` values, and "spells with the chosen name can't be
      ||| cast", "your opponents can't cast spells", "this ability can't
      ||| be activated", "you can't lose the game" and "this creature
      ||| can't be the target of nongreen spells" are the same row at
      ||| other labels. `PlayerCant`, `ObjectCant` and `OutcomeGate` were
      ||| three Cant-only carriers over three closed act enums; they say
      ||| nothing this one does not, and the corpus writes the same act
      ||| in both voices one card apart, which under labels is one deed
      ||| at two ROLES rather than a row in each of two vocabularies.
      |||
      ||| The deed slot is a LIST, and that list is the coordination the
      ||| deed, the outcome gate and the effect-scoped rider each wanted
      ||| separately: "can't attack or block" and "players can't lose the
      ||| game or win the game this turn" are one subject and one
      ||| modality over two labels, which is what the list says and what
      ||| no per-family fix could have said once.
      ||| THE PLAY PERMISSION IS THIS ROW. "You may cast this card from
      ||| your graveyard", "you may play lands from the top of your
      ||| library", "you may cast that spell without paying its mana
      ||| cost" are `Permit` at the `"Cast"` and `"Play"` deeds with the
      ||| played card as the complement, and everything the retired
      ||| `MayPlay` carried beyond that -- the source zone, the per-window
      ||| cap, the window, the exclusion and the payment -- is
      ||| `DeonticRider`'s, keyed to those two deeds by `deedPlays`. The
      ||| separate row said nothing this one does not: its verb was a
      ||| two-member enum where the deeds are labels, its complement was
      ||| the counterpart slot, and its own `castableTy` table was
      ||| `deedTypeOk` at the `Cast` patient spelled a second time.
      ||| -- spelling: "[n] can't/must/may [deed]", the deeds coordinated
      ||| with "or"; under `GatedBy`, "unless [cost]"; with a premise,
      ||| "as though [premise]"; with a rider, the rider's own phrases.
      Deontic : {k : Kind} -> (n : Noun bs k) ->
                (c : Compulsion (selfSubjIntro n)) ->
                (deeds : Deeds) -> (role : Role) ->
                (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
                (asThough : Maybe (AsThough (nomIntro n))) ->
                (rider : DeonticRider (deonticPatientIntro patient)) ->
                {auto 0 ne : NonEmpty deeds} ->
                {auto 0 dd : So (distinctDeeds deeds)} ->
                {auto 0 kd : KnownDeeds deeds} ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone deeds role)} ->
                {auto 0 dp : DeedParticipant deeds role k (nounTy n)} ->
                {auto 0 pt : So (deonticPatientOk n deeds role patient rider)} ->
                {auto 0 at : So (asThoughOk c deeds asThough)} ->
                {auto 0 rd : So (deonticRiderOk deeds role c patient
                                                (isJust asThough) rider)} ->
                StaticEffect bs
      ||| "Until end of turn, you don't lose this mana as steps and
      ||| phases end" (Tundra Fumarole, Kruphix's kin), "You don't lose
      ||| unspent green mana as steps and phases end" (Omnath, Locus of
      ||| Mana), "Players don't lose unspent mana as steps and phases
      ||| end" (Upwelling): the sentence that overrides [CR#106.4]'s
      ||| emptying. 32 supported lines (measured 2026-08-28), 25 of them
      ||| pointing back at an add with "this mana" and 7 describing a
      ||| player's unspent mana.
      |||
      ||| ITS OWN SENTENCE, never a rider on the production, and the
      ||| corpus settles it rather than taste. [CR#106.6] lists what a
      ||| production may say about its mana and this is not on the list;
      ||| 6 of the 7 description lines are static abilities of permanents
      ||| that add no mana at all, so no attachment could have carried
      ||| them; and all 25 mention lines write a duration that
      ||| `Continuously` already spells, which a rider slot would have had
      ||| to invent a second time. The two spellings are one row because
      ||| they differ only in how the mana is picked out, which is
      ||| `ManaHeld`'s axis.
      |||
      ||| The SUBJECT is written and never "you" by default: Upwelling
      ||| says it of every player at once, and the 1 line that writes
      ||| "they" says it of a player an earlier clause named.
      ||| It states no period of its own -- [CR#106.4] gives the loss its
      ||| occasion ("at the end of each step and phase") and the sentence
      ||| names that occasion rather than choosing one, so "as steps and
      ||| phases end" is spelling. What a line may still write is a SPAN,
      ||| and that is `Continuously`'s.
      ||| -- spelling: "[who] don't lose [what] as steps and phases end".
      KeepsUnspentMana : (who : Noun bs Player) ->
                         (what : ManaHeld (nomIntro who)) -> StaticEffect bs
      MayDeclineUntap : (n : Noun bs Object) ->
                        {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                        StaticEffect bs
      DoesntUntap : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      ||| "Untap [n] during each other player's untap step": Seedborn
      ||| Muse's family, 14 supported lines over 14 cards (measured
      ||| 2026-09-02). The GRANT where `DoesntUntap` and `CantMoreThan`
      ||| deny. [CR#502.3] gives the untap step's untapping to the ACTIVE
      ||| player's permanents alone -- "the active player determines
      ||| which permanents they control will untap" -- so a line that
      ||| untaps a non-active player's permanents there states an
      ||| addition to that turn-based action and not an exception to a
      ||| restriction.
      |||
      ||| It states NO window of its own. The window is `OnlyDuring`'s,
      ||| the same wrapper every other windowed static takes, and the
      ||| possessor the family writes is `Owner.EachOthers`; a private
      ||| window slot here would say a second time what that pair
      ||| already says, and would admit periods the deed has no rule
      ||| for. What is left on the row is the untapped SET, which is all
      ||| the fourteen lines differ in.
      ||| -- spelling: "Untap [n]", the window's own phrase after it.
      UntapsDuringStep : (n : Noun bs Object) ->
                         {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                         StaticEffect bs
      ||| "[who] can't [deed] more than [k] [p]": the COUNT CAP, and the
      ||| one qualifier family the carrier's complement cannot say. A cap
      ||| is a bound on how MANY times the deed may be done, not a
      ||| description of what it is done to, so it is its own row rather
      ||| than a noun in `DeonticCounterpart` -- and it is one row for
      ||| every deed rather than one per verb: 9 supported lines cap
      ||| untapping, 12 cap casting and 3 cap drawing (measured
      ||| 2026-08-28).
      |||
      ||| The PERIOD is the deed's own and is never a slot. [CR#502.3]
      ||| makes untapping a turn-based action of the untap step, so
      ||| "more than one land during their untap steps" caps one step's
      ||| worth; [CR#500.1] gives the turn to the deeds with no step of
      ||| their own, which is what "more than one spell each turn" and
      ||| "more than one card each turn" spell. The adverbial is
      ||| SPELLING of the deed's recurrence, so a window slot here would
      ||| admit periods no rule gives the deed.
      ||| -- spelling: "[who] can't [deed] more than [k] [p]" then the
      ||| deed's own period.
      CantMoreThan : (who : Noun bs Player) -> (deed : VerbLabel) ->
                     (k : Nat) -> (p : Predicate bs Object) ->
                     {auto 0 kd : KnownDeed deed} ->
                     {auto 0 pk : So (deedKindOk deed Patient Object)} ->
                     {auto 0 zn : ZoneFits (seedZone p) (deedZoneOf deed Patient)} ->
                     StaticEffect bs
      Skips : (who : Noun bs Player) -> (part : TurnPart) -> StaticEffect bs
      ||| The type ADDITION [CR#205.1b] at [CR#613.1d]'s layer 4, and the
      ||| colour addition [CR#613.1e] rides the same sentence: what the
      ||| bundle says is asked of the bundle (`AdditionSaysSomething`)
      ||| rather than of its type line, so the one printed colour-only
      ||| addition writes. `TokenCanonical` stands where
      ||| `ColorsDistinct` did: the distinctness the SETTING row demands
      ||| is the addition's too. Naming a card type twice in one line is
      ||| not a second addition -- [CR#205.1b] retains the prior types
      ||| and this line names what is added, so the repeat adds nothing
      ||| the first mention did not, while `tokenHeadTy` reads the head
      ||| off the LAST type and so reads one word twice. Measured
      ||| zero: no supported addition line repeats a card type or a
      ||| colour, and no ORDERING demand exists anywhere in the grammar
      ||| to be asked here or at the setting.
      ||| -- spelling: "[n] is/becomes [added] in addition to its other
      ||| types" (and "… other colors", where the bundle is a colour).
      BecomesAlso : (n : Noun bs Object) -> (added : TokenChars bs) ->
                    {auto 0 sw : AdditionSaysSomething (nounTy n) added} ->
                    {auto 0 af : AddedFits (nounTy n) added.line} ->
                    {auto 0 tc : TokenCanonical added} ->
                    {auto 0 ta : TokenAbilities added} ->
                    {auto 0 un : AdditionUnnamed added} -> StaticEffect bs
      AddsEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      {auto 0 sh : SpaceHosted space (nounTy n)} ->
                      StaticEffect bs
      ||| The quantifier's negative pole: "loses all creature types"
      ||| (4 lines), "loses all land types" (3). [CR#613.1d]'s layer 4
      ||| again, and the same `TypeSpace` payload and host gate as the
      ||| positive pole: [CR#205.1a]'s last sentence -- removing a
      ||| subtype "doesn't affect its card types at all" -- is why the
      ||| host gate survives the loss, the subject still having the card
      ||| type the emptied space belongs to.
      ||| The ability loss printed beside it on all three land lines is
      ||| NOT a rider here: [CR#613.1f] applies ability removal at layer
      ||| 6 where this applies at layer 4, so "loses all land types and
      ||| abilities" is two statements coordinated by `AndAlso`, the
      ||| second of them `LosesAllAbilities`' own row.
      ||| -- spelling: "[n] lose(s) all [space] types".
      LosesEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       {auto 0 sh : SpaceHosted space (nounTy n)} ->
                       StaticEffect bs
      ||| "Athreos isn't a creature", "Equipped permanent isn't a
      ||| planeswalker", "target artifact creature becomes blue and isn't
      ||| an artifact": ONE CARD TYPE taken off, [CR#613.1d]'s layer 4
      ||| again. [CR#205.1a] is the rule that knows the operation --
      ||| "if an object's card type is removed, the subtypes correlated
      ||| with that card type will remain if they are also the subtypes
      ||| of a card type the object currently has; otherwise, they are
      ||| also removed" -- so a removed card type is a thing the rules
      ||| state consequences for, not a paraphrase of a setting.
      ||| A row beside the three type-changing rows and not a case of
      ||| any of them: `SetsType` replaces a line, `BecomesAlso` adds to
      ||| one [CR#205.1b], and `LosesEveryType` empties a SUBTYPE space,
      ||| whose last sentence of [CR#205.1a] -- "removing an object's
      ||| subtype doesn't affect its card types at all" -- is exactly why
      ||| it cannot say this. None of the three can name a card type to
      ||| take away.
      ||| 28 supported lines write it (measured 2026-08-28), 21 of them
      ||| the devotion cycle's "[God] isn't a creature". Its own row
      ||| rather than a slot on `SetsType` because the two are printed
      ||| apart as often as together: 26 of the 28 write no setting on
      ||| the same statement.
      ||| The zone demand is the type-line rows', for their reason: this
      ||| is a permanent's type line and every supported line names a
      ||| battlefield subject.
      ||| -- spelling: "[n] isn't a [t]".
      LosesType : (n : Noun bs Object) -> (t : CardType) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  StaticEffect bs
      ||| The literal colour change, [CR#613.1e]'s layer 5 under the
      ||| "becomes"/copular verb with no type word in the sentence ("that
      ||| creature becomes green", "All creatures are black"). Its OWN
      ||| row rather than an empty type line on `SetsType`, and the
      ||| asymmetry with the addition above is the operation's own: a
      ||| SETTING that named no type would say the subject's types are
      ||| replaced by none, where an ADDITION of no type adds none and
      ||| the sentence still means what it says. Modeled on
      ||| `SetsChosenQuality`, which writes this same sentence over a
      ||| chosen colour [CR#607.2d]. The payload is the token bundle's
      ||| own colour list, so the empty list is "colorless" [CR#105.2c]
      ||| exactly as it is on a written token. Multi-colour settings are
      ||| a measured zero colour-only ("becomes a blue and red Dragon"
      ||| writes a type line) and are admitted with the list, and the
      ||| printed quantifier "all colors" is `ColorSpec`'s other arm.
      ||| NO subject-zone demand, and here the printed evidence is not
      ||| one line but eight: the lace cycle and Ersatz Gnomes set the
      ||| colour of a SPELL, which is on the stack. Same ground as the
      ||| chosen-quality rows below -- [CR#613.1] applies the layers to
      ||| an object's characteristics and [CR#109.1] makes a spell an
      ||| object.
      ||| -- spelling: "[n] becomes [color]", "[n] is/are [color]",
      ||| "[n] is all colors".
      SetsColor : (n : Noun bs Object) -> (cs : ColorSpec) ->
                  {auto 0 cd : ColorSpecOk cs} ->
                  StaticEffect bs
      SetsType : (n : Noun bs Object) -> (t : TokenChars bs) ->
                 (ret : Maybe CardType) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 {auto 0 ne : LineNonEmpty t.line} ->
                 {auto 0 af : AddedFits (nounTy n) t.line} ->
                 {auto 0 ta : TokenAbilities t} ->
                 {auto 0 tc : TokenCanonical t} ->
                 {auto 0 ro : RetentionOk t.line ret} -> StaticEffect bs
      ||| The two chosen-quality ascriptions carry NO subject-zone
      ||| demand, where the type-line rows beside them do. Decided on
      ||| Ashes of the Fallen, "Each creature card in your graveyard has
      ||| the chosen creature type in addition to its other types": the
      ||| layers apply to an OBJECT's characteristics [CR#613.1] and
      ||| [CR#109.1] makes a card an object, naming no zone, so nothing
      ||| makes a graveyard card's type unchangeable and the demand
      ||| refused printed text. What carries the meaning is `HostedRead` --
      ||| [CR#205.3d] refuses a subtype corresponding to none of the
      ||| object's types -- and it is asked of the subject's TYPE, which
      ||| a zone does not decide. Recorded overgeneration: a library or
      ||| hand subject, neither printed. The type-line rows keep their
      ||| demand, no printed line asking them to drop it.
      AddsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 hr : HostedRead q n} ->
                       StaticEffect bs
      SetsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 hr : HostedRead q n} ->
                       StaticEffect bs
      AlsoOffBattlefield : (se : StaticEffect bs) ->
                           {auto 0 nx : NotExtended se} -> StaticEffect bs
      ||| "This effect doesn't remove this Aura", "This effect doesn't
      ||| remove Auras already attached to those artifacts": the trailing
      ||| rider that carves a named object OUT of what the statement
      ||| before it reaches.
      ||| What "remove" names is a state-based action, and the rules say
      ||| which one. [CR#702.16c] has a permanent with protection unable
      ||| to be enchanted by Auras of the stated quality and "such Auras
      ||| attached to the permanent or player with protection will be put
      ||| into their owners' graveyards as a state-based action", which
      ||| is [CR#704.5m]; [CR#702.16d] says the same of Equipment and
      ||| Fortifications, which is [CR#704.5n]. The rider suspends that
      ||| action for the object it names, and "remove" is the printed
      ||| word for it.
      ||| ITS OWN construction and not a sign flip on `AlsoForKeywords`.
      ||| The two are both same-line trailers and they point opposite
      ||| ways -- that one widens a statement across a list of WORDS,
      ||| gated by `KeywordListOk`, this one narrows one statement by
      ||| naming an OBJECT it does not reach, read in the statement's own
      ||| context -- and no slot of either could carry the other's
      ||| payload.
      ||| It rides the STATEMENT, where the keyword trailer rides
      ||| `AbilityAt`, because "this effect" names the sentence before it
      ||| and not the line: Guardian Beast's rider trails a condition
      ||| over three coordinated statements about one subject, where the
      ||| twelve protection lines trail exactly one.
      ||| No gate says WHICH statements may carry it. What makes the
      ||| rider mean anything is that the statement makes a standing
      ||| attachment illegal, and the corpus writes two such statements
      ||| -- protection from a quality (14 of the 15 lines) and a plain
      ||| "can't be enchanted" prohibition (Guardian Beast) -- neither of
      ||| which is a computable property of the term, the deed
      ||| vocabulary being open. So the only gate is against nesting, on
      ||| `AlsoOffBattlefield`'s ground: a second rider repeats the
      ||| first.
      ||| 15 supported lines in four spellings, re-measured 2026-08-28:
      ||| 12 write "This effect doesn't remove this Aura" (the five
      ||| coloured Wards, Cho-Manno's Blessing, Flickering Ward, Floating
      ||| Shield, Pentarch Ward, Pledge of Loyalty, Tattoo Ward, Ward of
      ||| Lights), and the other three name the carved-out objects by
      ||| description -- Spectra Ward's "Auras", Benevolent Blessing's
      ||| "Auras and Equipment you control that are already attached to
      ||| it", Guardian Beast's "Auras already attached to those
      ||| artifacts".
      ||| -- spelling: "[se] This effect doesn't remove [n]."
      DoesntRemove : (se : StaticEffect bs) ->
                     (n : Noun (staticIntro se) Object) ->
                     {auto 0 nc : NotCarvedOut se} -> StaticEffect bs
      BecomesCopy : (n : Noun bs Object) -> (src : Noun (nomIntro n) Object) ->
                    (exc : List (CopyExcept (nomIntro src))) ->
                    {auto 0 pm : PerMember src} -> StaticEffect bs
      ||| "Enchanted creature loses all abilities", "All lands lose all
      ||| abilities except mana abilities" (Blood Sun): [CR#613.1f]'s
      ||| layer 6 emptying an object.
      ||| The EXCEPTION carves a described class back out, and it is the
      ||| slot `GainsAbilitiesOf` buys rather than one minted here: Blood
      ||| Sun is still the only supported line that writes an exception
      ||| on the LOSS (re-measured 2026-08-28), where the grant writes
      ||| two (Sharkey, Tyrant of the Shire's "except mana abilities" and
      ||| Scheming Fence's "except for loyalty abilities"). One shape,
      ||| three lines, two carriers -- the same `Predicate` over
      ||| `Ability` at both.
      ||| -- spelling: "[n] loses all abilities"; with an exception,
      ||| "except [except]".
      LosesAllAbilities : (n : Noun bs Object) ->
                          (except : Maybe (Predicate (selfSubjIntro n) Ability)) ->
                          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                          StaticEffect bs
      ||| The loss that names WHICH abilities: "All creatures lose
      ||| trample until end of turn" (Blind Fury), "Creatures your
      ||| opponents control lose hexproof and can't have or gain
      ||| hexproof" (Archetype of Endurance), "Permanents your opponents
      ||| control lose hexproof and indestructible until end of turn"
      ||| (Shadowspear). [CR#613.1f] puts ability-removing effects in the
      ||| same layer as ability-adding ones, and this is `Gains`' mirror
      ||| at that layer: the payload is an ability the line writes down,
      ||| where `LosesAllAbilities` names none and takes them all.
      ||| 76 supported lines write it (measured 2026-08-28) over 76
      ||| cards, "loses flying" the commonest at 32.
      ||| The payload is a LIST on `Deontic`'s ground -- the English is
      ||| n-ary ("lose hexproof, indestructible, protection, shroud, and
      ||| ward") -- and its elements are `AbilityLost`s: an ability the
      ||| line writes down, which carries the bare keyword, the
      ||| parameterised one ("protection from black", Cephalid Snitch)
      ||| and the quoted ability ("loses 'enchant creature card in a
      ||| graveyard'"); or a keyword TERM, which carries the word whose
      ||| parameter the line leaves open ("lose ... protection ... and
      ||| ward"). That element type's own docstring is where the split
      ||| is argued. A written element answers `grantableAb` at its own
      ||| constructor: an object can be made to lose only what it could
      ||| have had.
      ||| -- spelling: "[n] loses [abl]", the abilities coordinated with
      ||| "and".
      LosesAbilities : (n : Noun bs Object) -> (abl : List (AbilityLost bs)) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       {auto 0 ne : NonEmpty abl} ->
                       StaticEffect bs
      GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} -> StaticEffect bs
      ||| The cap slot is the trigger rider's word reused, not a third
      ||| `ReplUse` ending: [CR#614.3]'s two endings say how long the
      ||| replacement STANDS, where "the first time ... each turn" says how
      ||| often it may apply while it stands, and the two are written
      ||| together ("The first time you would draw a card each turn ... you
      ||| draw four cards instead"). Redundant beside `NextTimeOnly` rather
      ||| than meaningless, so it carries no gate, on `CreatedByUnder`'s
      ||| model. `OncePerGame` is measured at zero here and admitted with
      ||| the rest of the word.
      ||| -- spelling: "the first time [ev] each turn, [repl] instead".
      ||| The WINDOW slot: "If this creature would untap DURING YOUR UNTAP
      ||| STEP, remove a +1/+1 counter from it instead" (Bewitching
      ||| Leechcraft). [CR#614.1] has a replacement watch for a particular
      ||| event that would happen, and a printed window narrows WHICH
      ||| occurrences of it are watched -- the same narrowing a trigger
      ||| header writes with the same words, so the seat takes
      ||| `TriggerWindow` itself rather than a second window vocabulary,
      ||| and `windowOk`'s refusals carry over unchanged (the bare turn
      ||| included).
      Intercepts : (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
                   (window : Maybe TriggerWindow) ->
                   (repl : Effect (interceptCtx alts ev)) ->
                   (use : ReplUse) ->
                   (limit : Maybe UsageLimit) ->
                   {auto 0 ul : So (untriggeredLimitOk limit)} ->
                   {auto 0 ok : Interceptable ev} ->
                   {auto 0 oks : InterceptableArms alts} -> StaticEffect bs
      ||| The four prevention/redirection rows are two pairs, and the
      ||| split between the pairs is the SHIELD versus the CUT. `Prevents`
      ||| and `Redirects` size a shield the effect sets up ahead of the
      ||| damage -- `Shield` is `AllOfIt` or `TheNext n`, a standing
      ||| quantity spent as damage arrives. `PreventsFrom` and
      ||| `RedirectsFrom` name a SOURCE and cut each event it would cause
      ||| -- `PreventCut` is `CutAll`, `CutSome`, `CutAllBut` or
      ||| `CutHalf r`, a per-event arithmetic. Neither type is the
      ||| other's special case, and the two also disagree on two further
      ||| slots (`by` against `src`; the `ReplUse` only the source-side
      ||| rows carry), so the "which row you picked" fact is three facts
      ||| and not one. Recorded rather than merged.
      ||| Within a pair the difference IS one slot -- prevention has no
      ||| destination, redirection does -- which is why those two stay
      ||| two rows and not one with a `Maybe`.
      Prevents : (kind : DamageKind) -> (size : Shield bs) ->
                 (scope : DamageScope (shieldIntro size)) ->
                 (by : Maybe (Noun (scopeIntro scope) Object)) ->
                 (also : Maybe (Effect (outcomeB DamagePrevented :: byIntro by))) ->
                 StaticEffect bs
      PreventsFrom : (kind : DamageKind) ->
                     (src : DamageAgent bs) ->
                     (scope : DamageScope (agentIntro src)) ->
                     (cut : PreventCut (scopeIntro scope)) ->
                     (use : ReplUse) ->
                     (also : Maybe (Effect (outcomeB DamagePrevented :: cutIntro cut))) ->
                     StaticEffect bs
      Redirects : {k : Kind} -> (kind : DamageKind) -> (size : Shield bs) ->
                  (scope : DamageScope (shieldIntro size)) ->
                  (by : Maybe (Noun (scopeIntro scope) Object)) ->
                  (to : Noun (byIntro by) k) ->
                  {auto 0 rk : DamageRecipient to} ->
                  {auto 0 one : SingleRecipient to} -> StaticEffect bs
      RedirectsFrom : {k : Kind} -> (kind : DamageKind) ->
                      (src : DamageAgent bs) ->
                      (scope : DamageScope (agentIntro src)) ->
                      (to : Noun (scopeIntro scope) k) ->
                      (use : ReplUse) ->
                      {auto 0 rk : DamageRecipient to} ->
                      {auto 0 one : SingleRecipient to} -> StaticEffect bs
      Scales : (kind : DamageKind) -> (src : Noun bs Object) ->
               (scope : DamageScope (nomIntro src)) ->
               (op : DamageScale (scopeIntro scope)) ->
               (use : ReplUse) -> StaticEffect bs
      ||| "If [n] would enter the battlefield under an opponent's
      ||| control, it enters under [who]'s control instead" (Gather
      ||| Specimens): the entry with its CONTROLLER replaced. A dedicated
      ||| row whose body is fixed, on `Redirects`' model, and not an
      ||| entry written into `Intercepts`' body slot: [CR#614.1d] makes a
      ||| line reading "[objects] enter [the battlefield] ..." a
      ||| replacement effect and [CR#614.12] has such an effect modify
      ||| HOW the permanent enters, so what changes is a parameter of the
      ||| one entry rather than a second instruction to put the permanent
      ||| anywhere. The `Effect` vocabulary states no entry for that
      ||| reason -- entering is not an act a player is instructed to
      ||| take, which is why [CR#614.1c,614.1d]'s other entry riders
      ||| (`EntersRider`, `EntersWithCounters`, `EntersChoice`) are
      ||| static rows too.
      ||| The event's own side is written on the subject: the permanent
      ||| that would enter carries the control it would enter under,
      ||| which [CR#614.12] checks as it would exist on the battlefield.
      ||| `Replacement` and not `EntryRider`: those three are a
      ||| permanent's own printed entry riders, standing for as long as
      ||| the permanent does, where this is [CR#614.1a]'s "instead" and
      ||| takes a duration ("this turn").
      ||| -- spelling: "[n] enters under [who]'s control instead".
      EntersUnderInstead : (n : Noun bs Object) ->
                           (who : Noun (nomIntro n) Player) ->
                           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                           {auto 0 ps : SoleHolder who} -> StaticEffect bs
      ||| THE unpreventability statement, [CR#615.12]'s own sentence:
      ||| what damage it is about is `Unpreventable`'s question and what
      ||| it refuses is `PreventionBan`'s, so the described global, the
      ||| restricted static and the anaphoric rider are one row.
      ||| It is not a `Prevents` with a sign flipped and could not be:
      ||| [CR#615.12] leaves applicable prevention effects applying to
      ||| unpreventable damage and only stops them preventing any of it,
      ||| so this statement disables a shield rather than raising one.
      CantPrevent : (kind : DamageKind) -> (what : Unpreventable bs) ->
                    (ban : PreventionBan) -> StaticEffect bs
      ||| The leading static conditional, "as long as [c], [se]" / "if
      ||| [c], [se]": the condition is written first and the statement
      ||| reads what it announced. `OnlyWhile` is the postposed twin.
      ||| -- spelling: "as long as [c], [se]"; under `IfSo`, "if [c],
      ||| [se]"; under `NotCond` with `Unless`, "unless [c], [se]".
      Conditionally : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
                      (marking : CondMarking) ->
                      {auto 0 nn : NotConditional se} ->
                      {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      ||| The postposed static conditional, "[se] as long as [c]" / "[se]
      ||| unless [c]": the condition is written after the statement and reads
      ||| the statement's own subject ("has hexproof as long as IT's
      ||| untapped"), so it sits at `staticIntro se`. `Conditionally` is the
      ||| leading twin, whose statement reads the condition; neither is a
      ||| macro over the other and neither reads forward.
      ||| -- spelling: "[se] as long as [c]"; under `IfSo`, "[se] if [c]";
      ||| under `NotCond` with `Unless`, "[se] unless [c]".
      OnlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
                  (marking : CondMarking) ->
                  {auto 0 nn : NotConditional se} ->
                  {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      ||| "During [p] of [w], [se]" / "[se] only during [p] of [w]": the
      ||| WINDOW a static statement is confined to. `Conditionally`'s
      ||| twin at the turn structure rather than at the game state, and
      ||| the third thing a static statement can be qualified BY -- a
      ||| duration says when it ends [CR#611.2a], a condition says what
      ||| must hold, and a window says which part of a turn it is in
      ||| force during. None of the three is the others: "your opponents
      ||| can't cast spells during your turn" never ends, holds under no
      ||| condition, and applies in one part of every turn, so
      ||| `Continuously`'s span and `Conditionally`'s condition are both
      ||| the wrong shape for it. [CR#500.1] is what a window names -- a
      ||| phase or step of the turn, recurring every turn -- and
      ||| `windowOk` is the same gate an activation restriction's
      ||| `Timing` already puts on the pair.
      |||
      ||| Its readings under the two modalities are one reading, not
      ||| two: the window says WHEN the statement is in force. Under a
      ||| `Forbid` that is "during your turn, [n] can't [deed]"; under a
      ||| `Permit` it is "[deed] only during your end step", where the
      ||| permission is the only one there is and being confined is what
      ||| "only" says. [CR#601.3] leaves when a spell may be cast to
      ||| whatever rule or effect says so, which is what a confined
      ||| permission is.
      ||| -- spelling: "During [window], [se]" leading, "[se] only during
      ||| [window]" postposed; the window as its part and possessor.
      OnlyDuring : (p : TurnPart) -> (w : Maybe Owner) ->
                   (se : StaticEffect bs) ->
                   {auto 0 wk : WindowOk p w} ->
                   {auto 0 nw : So (notWindowed se)} ->
                   StaticEffect bs
      ||| The standing visibility rider: "[who] play(s) with [what]
      ||| revealed", "[who] may look at [what] any time". One row for
      ||| both audiences -- [CR#701.20e] makes looking revealing shown to
      ||| one player -- over the three surfaces `VisibleThing` names.
      ||| -- spelling: the verb, the subject and the complement's own
      ||| phrase, with "any time" on the look-at side.
      ||| "[who] don't lose the game for [cause]" -- Phyrexian Unlife's
      ||| family, 7 supported lines. NOT the whole-gate refusal, and not
      ||| the deontic carrier at the "LoseGame" label: [CR#104.3] names
      ||| several ways to lose the game and this line carves out exactly
      ||| ONE of them. [CR#104.3e]'s "an effect may state that a player
      ||| loses the game" still reaches the subject afterwards -- a
      ||| Phyrexian Unlife controller can still be made to lose by an
      ||| effect -- where `Deontic who Forbid ["LoseGame"]` (Lich's
      ||| Mastery, Platinum Angel) stops every cause at once. Two
      ||| different sentences with two different meanings, so two rows.
      ||| What it removes is one state-based action's application to one
      ||| player, which is why the subject is a player and there is no
      ||| modality slot: nothing here says "can't".
      ||| -- spelling: "[who] don't lose the game for [cause]".
      NoLossFrom : (who : Noun bs Player) -> (cause : LoseCause) ->
                   StaticEffect bs
      Visibility : (v : ExposeVerb) -> (who : Noun bs Player) ->
                   (what : VisibleThing (nomIntro who)) ->
                   {auto 0 vo : VisibilityOk v what} -> StaticEffect bs
      ||| The land allowance, "[who] may play [q] additional land(s)".
      ||| The bound was LITERAL, and the reason was never that literals
      ||| are what the corpus writes -- the printed ceilings "up to two"
      ||| and "up to three" (Journey of Discovery, Summer Bloom) are
      ||| literal ranges and always passed. The reason was that the
      ||| statement introduces no mention of its own, so an amount bound
      ||| that ANNOUNCED one would be announced nowhere. That is what the
      ||| gate now asks, and it is what the one blocked printing needed:
      ||| Nahiri's Lithoforming's "you may play X additional lands this
      ||| turn" reads a letter its own earlier sentence opened, so its
      ||| quantity announces nothing and passes, while an amount bound
      ||| that would introduce a mention still refuses.
      MayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity bs) ->
                               {auto 0 nz : NonZeroQ q} ->
                               {auto 0 wf : WellFormedQ q} ->
                               {auto 0 lt : So (isNil (quantDelta q))} ->
                               StaticEffect bs
      ||| The block allowance, "[n] can block [q] additional creature(s)":
      ||| the land allowance's OBJECT-SORTED sibling. 30 supported cards
      ||| write it (measured 2026-09-02); 20 of them say it of the
      ||| subject alone, 4 of a described set ("each creature you
      ||| control"), 3 of an attachment host, and the quantity axis
      ||| writes "an", "up to two", "seven" and "ninety-nine".
      |||
      ||| A SECOND ROW beside the land allowance and never a widening of
      ||| it, on `PlayerCant`/`ObjectCant`'s own ground: the subject is
      ||| an OBJECT, not a player, and the rule raised is
      ||| [CR#509.1a]'s blocker declaration -- "for each of the chosen
      ||| creatures, the defending player chooses ONE creature for it to
      ||| block" -- where the land allowance raises [CR#305.2]'s land
      ||| count. Nothing about the two shares a subject, a rule or a
      ||| deed.
      |||
      ||| The WINDOW is derived, exactly as it is on the land cell.
      ||| [CR#509.1] declares blockers once a combat, so "each combat"
      ||| adds nothing to a standing statement and "this turn" adds
      ||| nothing to a resolving one but the span `Continuously` already
      ||| spells; which of the two a card prints follows from whether
      ||| the sentence stands or resolves, and every line the corpus
      ||| writes agrees.
      ||| The quantity bound repeats the land cell's: a statement that
      ||| introduces no mention of its own can carry no amount that
      ||| would announce one.
      ||| -- spelling: "[n] can block [q] additional creature(s)", the
      ||| period from the sentence's own kind.
      MayBlockAdditional : (n : Noun bs Object) -> (q : Quantity bs) ->
                           {auto 0 nz : NonZeroQ q} ->
                           {auto 0 wf : WellFormedQ q} ->
                           {auto 0 lt : So (isNil (quantDelta q))} ->
                           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                           StaticEffect bs
      ||| [CR#506.3a] and [CR#508.4d] both say what happens when a
      ||| permanent enters attacking, so either rider is a real entry.
      EntersRider : (n : Noun bs Object) -> (rider : TokenRider) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      ||| "[n] enters with [amt] [kind] counter(s) on it". The kind slot
      ||| takes a printed word or a printed menu alike: [CR#614.12a] has
      ||| the pick made before the permanent enters, so a menu here is
      ||| the same replacement effect [CR#614.1c] spelled with its range
      ||| instead of its word.
      ||| -- spelling: "enters with your choice of a [k1], [k2], or [k3]
      ||| counter on it" (Denry Klin).
      EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                           (kind : CounterKindSource bs) ->
                           (mark : EntryCounterMark) ->
                           StaticEffect bs
      ||| "You may have this creature enter as a copy of any creature on
      ||| the battlefield, except it's a Wall in addition to its other
      ||| types": the copy-on-entry row, [CR#707.5]'s own sentence.
      ||| 60 supported lines over 60 cards write it (re-measured
      ||| 2026-09-02) and the would-enter-instead form is ZERO.
      |||
      ||| A ROW and not the composition it looks like. `Intercepts` over
      ||| a `Continuously (BecomesCopy …)` compiles and spells the
      ||| sentence [CR#707.5] exists to distinguish: the object "becomes
      ||| a copy as it enters the battlefield. It doesn't enter the
      ||| battlefield, and then become a copy". Two things ride on the
      ||| difference and neither composition can carry them --
      ||| [CR#707.5]'s "enters with" and "as [this] enters" abilities of
      ||| the COPIED text take effect, which a permanent that had already
      ||| entered would be past, and [CR#707.6] hands the copy's
      ||| controller the as-enters choices fresh rather than copying the
      ||| original's. So the row is the one place both attach, and they
      ||| are the rule's consequences of entering as a copy rather than
      ||| slots a card writes: no printed line states either.
      |||
      ||| `EntryRider`'s seat, with `EntersRider`'s subject gate:
      ||| [CR#614.1d] makes a line reading "[this permanent] enters …" a
      ||| replacement effect, and [CR#614.12] has it modify HOW the
      ||| permanent enters.
      |||
      ||| The SOURCE IS NOT ANNOUNCED, and this is the row's design law
      ||| rather than an omission -- `TokenCopyOf`'s `specDelta = []`
      ||| said first, for the same reason at the same seat. The
      ||| exceptions are elaborated in the SUBJECT's discourse and not
      ||| the source's, because 38 of the 60 lines write "except IT",
      ||| every one of them meaning the entering permanent; announcing
      ||| the source would put a second singular object beside it and
      ||| leave that pronoun with two antecedents. The source is a
      ||| constructor argument, never a discourse mention.
      |||
      ||| The optional flag is the printed "you may have": 56 of the 60
      ||| lines write it and the other 4 do not -- two describe a CLASS
      ||| of entering permanents rather than the source itself ("Creatures
      ||| you control enter as a copy of this creature"), which is also
      ||| why the subject is a noun and not fixed to the self.
      ||| -- spelling: "[You may have ][n] enter as a copy of [src][,
      ||| except [exc]]".
      EntersAsCopy : (n : Noun bs Object) -> (optional : Bool) ->
                     (src : Noun (selfSubjIntro n) Object) ->
                     (exc : List (CopyExcept (selfSubjIntro n))) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 pm : PerMember src} -> StaticEffect bs
      EntersChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      ||| "As [n] becomes attached to [host], choose [q]" -- the choice
      ||| an ATTACHMENT replacement makes. A distinct row beside
      ||| `EntersChoice` and never a relaxation of its gate: [CR#614.1c]
      ||| names the entering event, while [CR#301.5b] says Equipment
      ||| "enter the battlefield like other artifacts. They don't enter
      ||| the battlefield attached to a creature", and [CR#701.3a]'s
      ||| attaching takes the permanent "from where it currently is" and
      ||| puts it onto its host. So the two riders are both replacement
      ||| effects under [CR#614.1]'s general definition -- effects that
      ||| "watch for a particular event that would happen" -- watching
      ||| DIFFERENT events, which is also why this one re-fires on every
      ||| re-equip where the other fires once per entry.
      ||| The host phrase the line prints ("to a creature") is not
      ||| carried: [CR#301.5] fixes an Equipment's host and [CR#301.6] a
      ||| Fortification's outright, and [CR#303.4] hands an Aura's to its
      ||| own enchant ability, so the phrase restates what the subject
      ||| already carries and no line reads it back. No gate asks whether
      ||| the subject CAN attach either:
      ||| that is the `Subtype` catalog's vocabulary, and all three
      ||| supported carriers write an Equipment ascription.
      ||| -- spelling: "As [n] becomes attached to [host], choose [q]"
      ||| (Sanctuary Blade, Psychic Paper).
      AttachChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      AndAlso : {0 n : Nat} -> StaticParts n bs ->
                {auto 0 ne : IsSucc n} -> StaticEffect bs
      ||| "[n] gets +4/+4 and gains trample", "equipped creature gets
      ||| +1/+1 and has flying": VERB PHRASE coordination over ONE
      ||| subject, which English writes by eliding the second verb
      ||| phrase's subject rather than by pronominalising it. The subject
      ||| is written once and every part is predicated of it, so the
      ||| printed line contains no pronoun and this row's slots contain
      ||| no anaphor -- where `AndAlso` coordinates whole STATEMENTS,
      ||| each naming its own subject, and a second statement wanting the
      ||| first one's subject has to read it back.
      |||
      ||| That read is what this row retires. It is not a widening of any
      ||| gate: the shared subject is this construction's own earlier
      ||| argument, which is the forward binder contract's clause 4
      ||| rather than clause 3, and nothing here counts anything
      ||| (`docs/decisions/oracle-text-is-forward-anaphoric.md`).
      ||| 922 supported faces write the two-part form measured
      ||| 2026-08-27 -- 435 "gets [pt] and gains [ab]", 487 "gets [pt]
      ||| and has [ab]" -- and the two spellings are one row, since
      ||| `Gains` already spells both.
      |||
      ||| The parts thread left to right on `StaticParts`' model
      ||| [CR#608.2c], each typed in the previous part's output, and the
      ||| subject's own announcement is made once at the head. The
      ||| per-part obligations are the rows' own, asked of the shared
      ||| subject: a P/T shift wants a battlefield subject, a grant wants
      ||| a subject the ability may be granted to [CR#113.6e].
      ||| -- spelling: "[n] [vp1] and [vp2]", the second verb phrase
      ||| written without a subject.
      OfSubject : {0 k : Nat} -> (n : Noun bs Object) ->
                  (vps : SubjectVPs k (selfSubjIntro n)) ->
                  {auto 0 ne : IsSucc k} ->
                  {auto 0 ok :
                     So (vpsOk (nounZone n) (nounRegime n) (nounTy n) vps)} ->
                  StaticEffect bs

  ||| The modality, all four rows of it. [CR#609.4] writes the
  ||| permissive one in the rules' own words -- "a player may do
  ||| something ... or A CREATURE CAN do something" -- and it is the row
  ||| the closed enum lacked: "this creature can attack" was unwritable
  ||| before any counterfactual was reached, which is what blocked the
  ||| whole permission family.
  ||| The mention a gate's derived payer leaves for its cost to read:
  ||| one definite player, "their controller".
  public export
  gatePayer : Binding
  gatePayer = MkBinding TheD Player OneOf PlayerP

  public export
  data Compulsion : Bindings -> Type where
    ||| "[n] can't [deed]"
    Forbid : Compulsion bs
    ||| "[n] [deed]s if able"
    Require : Compulsion bs
    ||| "[n] can't [deed] unless [c]" -- the gate, whose cost is typed
    ||| one step further along than the other three modalities: it reads
    ||| the restriction's own SUBJECT (the carrier types every
    ||| `Compulsion` at `selfSubjIntro n`, `Gets`' seat and for `Gets`'
    ||| reason -- the subject is written before the cost and a deictic
    ||| one announces itself, so "unless you pay {1} for each +1/+1
    ||| counter on IT" reads the creature its own statement named) and,
    ||| on top of that, the PAYER the gate derives.
    |||
    ||| THE PAYER IS DERIVED AND SPELLED, never written as a slot.
    ||| [CR#508.1h] has the ACTIVE PLAYER determine the total cost to
    ||| attack and [CR#509.1d] has the DEFENDING PLAYER determine the
    ||| total cost to block, so who pays follows from whose creature is
    ||| doing the deed and is never a slot; the printed "their
    ||| controller" / "its controller" / "you" is that derivation
    ||| spelling itself.
    ||| Announcing it here is what lets the COST read it back, which
    ||| three printed shapes need: "for each creature THEY control
    ||| that's attacking you" (Propaganda, Ghostly Prison, Windborn
    ||| Muse, Koskun Falls, Elephant Grass, Collective Restraint, Onakke
    ||| Oathkeeper), "pays 1 life for each blocking creature THEY
    ||| control" (Heat Wave) and Sivitri, Dragon Master's life payment,
    ||| whose own payer noun the action cost writes.
    ||| The subject's mention serves the other anaphor: "unless their
    ||| controller pays {1} for each of THOSE creatures" is `GroupSize`
    ||| over the plural the subject announced -- 9 supported lines
    ||| (Archangel of Tithes twice, Archon of Absolution, Baird, Dain,
    ||| Norn's Annex, Sphere of Safety, Summon: Yojimbo, Forbidding
    ||| Spirit; Sivitri's is the tenth at a life payment) -- and "unless
    ||| you pay {1} for each +1/+1 counter on IT" is `CountersOn` over
    ||| the singular one (Myr Prototype, Phyrexian Marauder).
    ||| -- spelling: "unless [c]", the payer spelled from the subject.
    GatedBy : (c : Cost (Effect.gatePayer :: bs)) -> Compulsion bs
    ||| "[n] can [deed]", "you may have [n] [deed]". The optionality is
    ||| this row's own and needs no second carrier: a permission a player
    ||| declines is a permission unused.
    |||
    ||| WHOSE permission it is, decided here: the subject is the DEED'S
    ||| ACTOR, never the player who chooses to apply it. [CR#609.4]
    ||| bipartitions the surface into "a player MAY do something ... or a
    ||| CREATURE CAN do something", and the 19 supported lines that read
    ||| "You may have this creature assign its combat damage as though it
    ||| weren't blocked" fall on the creature's side of it: [CR#510.1a]
    ||| makes each attacking and blocking creature the thing that ASSIGNS
    ||| combat damage, and [CR#510.1] gives the player only the
    ||| announcement of how. The premise settles it independently -- "as
    ||| though it weren't blocked" is a counterfactual about the creature,
    ||| and [CR#609.4]'s "treat the game exactly as if the stated
    ||| condition were true" has nothing to mean if the premise is
    ||| predicated of a player who is not blocked in the first place. So
    ||| the cell writes one carrier with the creature as subject; the
    ||| printed "you may have" is this row spelling its own optionality on
    ||| the controller, not a second subject and not the play
    ||| permission's row.
    Permit : Compulsion bs

  ||| The deed's other participant, written or left out. Which arms a
  ||| given deed admits is `deonticPatientOk`'s question, asked once at
  ||| the carrier against `deedFacts`, so the arms are not indexed by the
  ||| deed and a new deed adds no constructor here.
  public export
  data DeonticPatient : {0 bs : Bindings} -> Deeds -> Role -> Type where
    NoDeonticPatient : DeonticPatient {bs} ds r
    ||| What an attack is aimed at [CR#506.3] -- a player, a planeswalker
    ||| or a battle, and under a joined kind a phrase naming either half.
    ||| The one place a deontic's other participant may be a PLAYER,
    ||| which is why the deed's row carries `deedDefends` beside its
    ||| patient types.
    ||| Re-measured 2026-08-28: 17 supported gate lines name a defender
    ||| and a cost at once ("creatures can't attack you unless their
    ||| controller pays ..."), and this arm serves them all at Attack /
    ||| `Agent` -- the case the row was recorded as refusing, which it
    ||| has not refused since the carrier unified.
    ||| WHAT IS STILL REFUSED HERE: a DISJOINED defender. 8 of those 17
    ||| write "you or planeswalkers you control", which is one mention
    ||| at a joined kind and wants a noun-level disjunction the grammar
    ||| does not spell; `Attackable` would admit it the moment one
    ||| exists.
    DefendingPlayer : {k : Kind} -> (m : Noun bs k) ->
                      {auto 0 at : Attackable m} ->
                      DeonticPatient {bs} ds r
    ||| The deed's OTHER participant, described -- and which end that is
    ||| follows from the ROLE the subject fills. With the subject at
    ||| `Patient` it is the deed's own counterpart, a blocker's attacker
    ||| or an attacker's blocker [CR#506.3]. With the subject at `Agent`
    ||| it is what the deed is done TO, which is the COMPLEMENT the
    ||| qualified acts carry: "your opponents can't cast spells with the
    ||| chosen name", "you can't cast noncreature spells", "this ability
    ||| can't be activated" in its other voice.
    |||
    ||| One arm serves both because `counterpartFits` asks its question
    ||| against `counterRole r` and the deed's own rule answers it:
    ||| [CR#601.2] takes a cast spell from where it is and puts it on the
    ||| stack, which is exactly where `deedFacts` seats a `Cast` patient,
    ||| and [CR#602.2] does the same for an activated ability. So the
    ||| qualified complement needed no slot of its own -- unifying the
    ||| carrier paid for it, and the five qualifier families that name an
    ||| object (a spell TYPE, a NAME, a colour, a chosen type) are one
    ||| noun here rather than five members of an act vocabulary.
    DeonticCounterpart : {k : Kind} -> (m : Noun bs k) ->
                         DeonticPatient {bs} ds r
    ||| WHAT may not target the subject: "spells or abilities your
    ||| opponents control", "nongreen spells or abilities from nongreen
    ||| sources", "Aura spells". The by-spell/by-source distinction the
    ||| act vocabularies had no room for needs no slot of its own here.
    ||| [CR#115.1a] makes a SPELL a targeter and [CR#115.1c,115.1d] make
    ||| an ABILITY one, so a line naming both names two kinds of object;
    ||| [CR#113.7] then gives an ability "the object that generated it",
    ||| and [CR#109.2c] reads the word "source" in a description as
    ||| exactly that object. `AbilityOf` is already the predicate that
    ||| names it. So the agent is ONE noun at the joined
    ||| kind the printed line writes ("spells or abilities"), gated by
    ||| `Targeter`, and "nongreen spells or abilities from nongreen
    ||| sources" spells the colour twice because the rules make it two
    ||| descriptions of two different objects.
    ||| Not a member of any act vocabulary, and not `CantBe`: that rider
    ||| denies one resolving effect's own consequence, where this states
    ||| a continuous restriction on a permanent. The two now share the
    ||| deed labels and differ in construction, which is exactly their
    ||| relationship.
    TargetedBy : {k : Kind} -> (m : Noun bs k) ->
                 {auto 0 tr : Targeter k} ->
                 DeonticPatient {bs} ds r
    ||| The complement written at ONE deed of a coordination: "enchanted
    ||| permanent can't attack, block, or CREW VEHICLES" (Revoke
    ||| Privileges, Bound in Gold, Intercessor's Arrest -- 3 supported
    ||| lines over 3 cards), "this creature saddles MOUNTS and crews
    ||| VEHICLES" (13 of the 18 counterfactual-value lines).
    |||
    ||| A SECOND complement arm and not a widening of the first, because
    ||| the two are different sentences and `deonticPatientOk` asks a
    ||| different question of each. `DeonticCounterpart` is the SHARED
    ||| complement -- one noun checked against every coordinated deed,
    ||| which is what a single-deed statement writes and what "can't be
    ||| blocked by Walls" would say of a coordination. This one is the
    ||| complement of one conjunct: [CR#506.3] admits only a planeswalker
    ||| or a battle at an attack's patient, so a Vehicle checked against
    ||| the attack arm fails, and the printed sentence says nothing of
    ||| the sort -- "attack" and "block" are intransitive there and only
    ||| "crew" takes the object. [CR#702.122d] states that conjunct's
    ||| meaning outright.
    ||| Each named deed is one of the statement's own and named once; a
    ||| deed the list leaves out is the intransitive conjunct.
    ||| -- spelling: the complement after its own deed word, the deeds
    ||| coordinated as usual.
    CounterpartsAt : (cs : List (DeedComplement bs)) ->
                     {auto 0 ne : NonEmpty cs} ->
                     DeonticPatient {bs} ds r

  ||| One conjunct's complement: the deed it belongs to, and the noun.
  public export
  data DeedComplement : Bindings -> Type where
    MkDeedComplement : {k : Kind} -> (d : VerbLabel) -> (m : Noun bs k) ->
                       DeedComplement bs

  public export
  complementDeed : {0 bs : Bindings} -> DeedComplement bs -> VerbLabel
  complementDeed (MkDeedComplement d _) = d

  ||| Where a statement's own rider is typed: after the complement, which
  ||| is the only part of the statement written between the deed and the
  ||| rider. A statement with no complement leaves the prefix as it found
  ||| it, and the per-conjunct complements announce nothing -- they are a
  ||| LIST, so no one of them is the seat a following phrase would read.
  public export
  deonticPatientIntro : {bs : Bindings} -> {0 ds : Deeds} -> {0 r : Role} ->
                        DeonticPatient {bs} ds r -> Bindings
  deonticPatientIntro NoDeonticPatient = bs
  deonticPatientIntro (DefendingPlayer m) = nomIntro m
  deonticPatientIntro (DeonticCounterpart m) = nomIntro m
  deonticPatientIntro (TargetedBy m) = nomIntro m
  deonticPatientIntro (CounterpartsAt cs) = bs

  ||| What a statement may write BESIDE the deed, where the deed's own
  ||| rule leaves it something to write. `DeonticPatient`'s mold at the
  ||| other slot: an unindexed sum whose admissibility is asked once at
  ||| the carrier against `deedFacts`, so the arms are not indexed by the
  ||| deed and a deed with nothing to say adds no constructor here.
  |||
  ||| ONE arm today, and it is the whole of the retired `MayPlay` row.
  ||| [CR#601.3] makes casting depend on a rule or effect ALLOWING it, so
  ||| a licence's own scope -- from where, how often, when, whether it
  ||| revokes the default, and at what price -- is part of what the
  ||| allowing sentence says rather than a second statement beside it.
  ||| That is why the five ride here together and why they ride the deed:
  ||| no other deed's rule leaves an effect any of those to state, which
  ||| `deedPlays` records.
  |||
  ||| `exclusive` is the negative co-ordinate Haakon, Stromgald Scourge
  ||| writes and nothing else does -- "you may cast this card from your
  ||| graveyard, BUT NOT FROM ANYWHERE ELSE". The default is itself an
  ||| allowing rule -- [CR#302.1] lets a player cast a creature card FROM
  ||| THEIR HAND -- so a line may take it away; that is a second thing
  ||| said about the same act in the same sentence, not a description of
  ||| the object, which is why it rides here rather than being a
  ||| `Predicate` or a second statement.
  ||| -- spelling: "from [from]" after the complement, then the limit,
  ||| the window, ", but not from anywhere else" and the payment.
  public export
  data DeonticRider : Bindings -> Type where
    NoDeonticRider : DeonticRider bs
    PlayRider : (from : Maybe (ZoneExpr bs)) ->
                (limit : Maybe PlayLimit) ->
                (window : Maybe PlayWindow) ->
                (exclusive : Bool) ->
                (payment : PlayPayment) -> DeonticRider bs

  ||| Whether the statement carries the play rider: the one question
  ||| every other gate asks of it.
  public export
  playRidden : {0 bs : Bindings} -> DeonticRider bs -> Bool
  playRidden NoDeonticRider = False
  playRidden (PlayRider _ _ _ _ _) = True

  public export
  agentRole : Role -> Bool
  agentRole Agent = True
  agentRole Patient = False

  ||| Whether the deed's own counterpart fits the role it is written
  ||| into: the kind, the card type (or its absence) and the zone, asked
  ||| of every coordinated deed at once. `DeedParticipant`'s content as a
  ||| Bool, because the counterpart is checked from the carrier rather
  ||| than at its own constructor.
  ||| The last argument says the complement is named BEFORE the deed
  ||| moves it, and the play permission is the only statement of which
  ||| that is true: [CR#601.2a] takes the object "from where it is" and
  ||| puts it on the stack, so a licence's complement is a CARD in a zone
  ||| it may be cast from where the same deed's PROHIBITION describes the
  ||| spell that casting produced ("spells with the chosen name can't be
  ||| cast"). Which zones the licence's complement may name is not one
  ||| zone and so cannot be `roleZone`'s: it is `playSourceOk`'s
  ||| question, asked at the rider with the written source in hand, and
  ||| asking the role's single zone here as well would refuse every
  ||| printed permission. The kind and the card type are asked of both
  ||| readings alike -- a land is no more castable before the move than
  ||| after it.
  public export
  counterpartFits : {bs : Bindings} -> {k : Kind} -> Deeds -> Role ->
                    Noun bs k -> Bool -> Bool
  counterpartFits {k} ds r m moved =
    all (\d => deedKindOk d r k) ds &&
    (case nounTy m of
       Just ty => all (\d => deedTypeOk d r ty) ds
       Nothing => all (\d => deedBareOk d r) ds) &&
    (moved || zoneFits (nounZone m) (deedsZone ds r))

  ||| A creature never blocks itself and never attacks itself.
  ||| [CR#509.1a] has the DEFENDING player choose the blockers from among
  ||| the creatures they control and, for each, "one creature for it to
  ||| block that's attacking that player"; [CR#508.1a] has the ACTIVE
  ||| player choose the attackers from among the creatures they control.
  ||| The two participants are therefore always under different
  ||| controllers and are never one object, so a statement naming the
  ||| subject as its own counterpart says what no game state can satisfy.
  ||| A bare `It` written as the counterpart is exactly that statement
  ||| whenever the subject made the only Object announcement the pronoun
  ||| could read -- the counterpart is typed at `nomIntro n`, and
  ||| bindings are nearest-first. `ItOtherThan` is the positive path: it
  ||| splits the subject's own delta off the prefix and reads what is
  ||| left, which is what `mustBlockIt` writes.
  public export
  counterpartNotSelf : {bs : Bindings} -> {k : Kind} -> {ka : Kind} ->
                       (n : Noun bs k) -> Noun (nomIntro n) ka -> Bool
  counterpartNotSelf n It = countOnes Object (nounDelta n) == 0
  counterpartNotSelf n _ = True

  ||| Which other participant each deed admits, asked once at the
  ||| carrier: a defender only where the deed is aimed at one
  ||| [CR#506.3], a targeter only where the deed is targeting
  ||| [CR#115.1a,115.1c,115.1d], and the deed's own counterpart wherever
  ||| the deed's other role can be filled at all.
  public export
  deonticPatientOk : {bs : Bindings} -> {k : Kind} -> (n : Noun bs k) ->
                     (ds : Deeds) -> (r : Role) ->
                     (patient : DeonticPatient {bs = nomIntro n} ds r) ->
                     DeonticRider (deonticPatientIntro patient) -> Bool
  deonticPatientOk n ds r NoDeonticPatient _ = True
  deonticPatientOk n ds r (DefendingPlayer m) _ = all deedDefendsOk ds && agentRole r
  deonticPatientOk n ds r (DeonticCounterpart m) rider =
    counterpartFits ds (counterRole r) m (playRidden rider) &&
    counterpartNotSelf n m
  deonticPatientOk n ds r (TargetedBy m) _ = all deedTargetedOk ds
  deonticPatientOk n ds r (CounterpartsAt cs) _ =
    distinctDeeds (map complementDeed cs) &&
    all (complementAtOk n ds r) cs

  ||| One conjunct's complement, checked against ITS OWN deed alone --
  ||| which is the whole of what this arm buys over the shared one. The
  ||| deed must be one the statement names, or the sentence would give an
  ||| object to a verb it never wrote.
  public export
  complementAtOk : {bs : Bindings} -> {k : Kind} -> (n : Noun bs k) ->
                   (ds : Deeds) -> (r : Role) ->
                   DeedComplement (nomIntro n) -> Bool
  complementAtOk n ds r (MkDeedComplement d m) =
    elem d ds && counterpartFits [d] (counterRole r) m False &&
    counterpartNotSelf n m

  ||| [CR#609.4]'s slot opens on the PERMISSION alone. The rule's own
  ||| sentence is "a player may do something 'as though' ... or a
  ||| creature can do something 'as though' ...", and the corpus agrees:
  ||| of 264 supported "as though" sentences, ZERO write one under a
  ||| can't, a must or a gate (measured 2026-08-28). The deed must admit
  ||| one too, which is what `deedCounterfactual` records.
  public export
  asThoughOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
               Compulsion bs -> Deeds -> Maybe (AsThough cs) -> Bool
  asThoughOk _ ds Nothing = True
  asThoughOk Permit ds (Just a) = all (deedPremiseOk (asThoughSort a)) ds
  asThoughOk _ _ (Just _) = False

  ||| The modality as a Bool, for the gates that open on the permission
  ||| alone.
  public export
  permits : {0 cs : Bindings} -> Compulsion cs -> Bool
  permits Permit = True
  permits _ = False

  ||| Which statements may carry the play rider, asked once at the
  ||| carrier. Five demands, each the fold's inheritance from the row it
  ||| replaced:
  ||| * the PERMISSION alone, and every deed a play deed
  |||   ([CR#601.3]/[CR#701.18a] give an allowing effect its scope; a
  |||   can't states no source and no price);
  ||| * the subject at the AGENT -- the player who plays, [CR#601.2]'s
  |||   own -- with the played card as the statement's complement, since
  |||   a licence with nothing licensed says nothing;
  ||| * `playSourceOk` over the complement's own zone word and the
  |||   written source, unchanged;
  ||| * `playWindowOk`, which refuses the one pairing that would spell a
  |||   turn phrase twice;
  ||| * the exclusion's written source, since with no source phrase there
  |||   is no "else" to exclude.
  public export
  deonticRiderOk : {bs : Bindings} -> {0 cs : Bindings} ->
                   (ds : Deeds) -> (r : Role) -> Compulsion cs ->
                   (patient : DeonticPatient {bs} ds r) -> Bool ->
                   DeonticRider (deonticPatientIntro patient) -> Bool
  deonticRiderOk ds r c pat at NoDeonticRider = True
  deonticRiderOk ds r c (DeonticCounterpart m) at (PlayRider from lim win exc pay) =
    permits c && all deedPlaysOk ds && agentRole r &&
    playSourceOk (nounZone m) from at &&
    playWindowOk lim win && (not exc || isJust from)
  deonticRiderOk ds r c _ at (PlayRider _ _ _ _ _) = False

  ||| THE NON-NESTING GATES. `notConditional`, `notWindowed`,
  ||| `notExtended`, `notCarvedOut`, `isCoord`, `isCompound`, `isInstead`
  ||| and `notWordHeaded` are one idiom, not eight: a `Bool` reader that
  ||| answers `False` for exactly the rows a wrapper may not wrap, and a
  ||| `So` of it at the wrapper's gate. The mechanism is already shared
  ||| -- it is `So` -- and the per-row names are the `GameEvent` gates'
  ||| idiom, kept so a refusal says which question was asked; nothing
  ||| further merges, because Idris has no way to say "matches this
  ||| constructor" once for eight different constructors of five
  ||| different datatypes. Each reader's own docstring carries its own
  ||| rule; none of them restates this paragraph.
  |||
  ||| The gate on both static conditionals: a conditioned statement is not
  ||| conditioned again. It is a NARROWING and not a pin, and it is
  ||| asserted by nothing on purpose. A nested conditional is
  ||| rules-meaningful — [CR#603.4] leaves "if" its normal English meaning
  ||| everywhere it is not an intervening clause — so a proof refusing one
  ||| would refuse no rules impossibility, which is the pin `Effect.If`'s
  ||| round retired for exactly that reason. Nor does the gate merely
  ||| prefer a canonical form: `AndCond` is not the same term, because it
  ||| holds every conjunct at `bs` where a nesting would type the inner
  ||| condition at `condIntro` of the outer and let it read what the outer
  ||| announced. So the gate withholds real width, and no supported line
  ||| pays for it — the eight "as long as … as long as" lines are
  ||| independent statements. It stays until one does.
  public export
  notConditional : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notConditional (Conditionally _ _ _) = False
  notConditional (OnlyWhile _ _ _) = False
  notConditional _ = True


  public export
  NotConditional : StaticEffect bs -> Type
  NotConditional {bs} se = So (notConditional se)

  ||| A windowed statement is not windowed again -- the same narrowing
  ||| `notConditional` puts on the conditional, and for the same reason:
  ||| no printed line writes two turn-part windows over one statement,
  ||| and the second would have to name a part inside the first for the
  ||| pair to mean anything [CR#500.1] does not already give the inner
  ||| one.
  public export
  notWindowed : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notWindowed (OnlyDuring _ _ _) = False
  notWindowed _ = True

  public export
  data Shield : Bindings -> Type where
    AllOfIt : Shield bs
    TheNext : (amt : Amount bs) -> Shield bs

  public export
  shieldIntro : {bs : Bindings} -> Shield bs -> Bindings
  shieldIntro AllOfIt = bs
  shieldIntro (TheNext amt) = amtIntro amt

  public export
  data DamageScope : Bindings -> Type where
    Everywhere : DamageScope bs
    ToRecipient : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 rk : DamageRecipient n} -> DamageScope bs

  public export
  scopeIntro : {bs : Bindings} -> DamageScope bs -> Bindings
  scopeIntro Everywhere = bs
  scopeIntro (ToRecipient n) = nomIntro n

  public export
  data DamageAgent : Bindings -> Type where
    Unattributed : DamageAgent bs
    DealtBy : (n : Noun bs Object) -> DamageAgent bs

  public export
  agentIntro : {bs : Bindings} -> DamageAgent bs -> Bindings
  agentIntro Unattributed = bs
  agentIntro (DealtBy n) = nomIntro n

  public export
  byIntro : {bs : Bindings} -> Maybe (Noun bs Object) -> Bindings
  byIntro Nothing = bs
  byIntro (Just n) = nomIntro n

  ||| WHICH damage a [CR#615.12] "can't be prevented" statement is about.
  ||| Two arms because the corpus writes two subjects, and one of them is
  ||| a description while the other is a mention.
  |||
  ||| A DESCRIBED subject is the standing statement: "Damage can't be
  ||| prevented this turn" (16 supported sentences, the bare global),
  ||| "Damage that would be dealt by this creature can't be prevented"
  ||| (4, the restricted). Its two coordinates are the ones every other
  ||| prevention row carries -- where the damage goes, and what deals it.
  |||
  ||| A MENTIONED subject is the rider a damage clause trails: "Combust
  ||| deals 5 damage to target white or blue creature. THE DAMAGE can't
  ||| be prevented" -- 9 supported sentences over 9 cards (Arrow Storm,
  ||| Banefire, Combust, Demonfire, Flames of the Blood Hand, Lightning
  ||| Surge, Pinpoint Avalanche, Urza's Rage, Volcano Hellion; measured
  ||| 2026-08-28). It names the one damage event its own text just
  ||| described, which neither coordinate says: the recipient is that
  ||| clause's target and the source is the spell, and writing both would
  ||| still be a second description rather than the anaphor the card
  ||| prints.
  ||| The gate is `DealtThisWay`'s, for `DealtThisWay`'s reason:
  ||| [CR#608.2c] lets later text read the instruction it follows and
  ||| settles no more than that the reading is available, so the licence
  ||| is an EXISTENCE test on some damage a clause dealt and no
  ||| discrimination between two of them is attempted. A text carrying two
  ||| damage mentions can write this rider pointing at either; that
  ||| mis-pairing is tolerated overgeneration, refused at the spelling
  ||| boundary.
  ||| It announces nothing: the statement names no new referent, and the
  ||| damage it points at was announced by the clause that dealt it.
  ||| -- spelling: described, "[kind] damage [scope] [by] can't be
  ||| prevented"; mentioned, "the damage can't be prevented".
  public export
  data Unpreventable : Bindings -> Type where
    DamageDescribed : (scope : DamageScope bs) ->
                      (by : Maybe (Noun (scopeIntro scope) Object)) ->
                      Unpreventable bs
    ThatDamage : {auto 0 ok : So (damageDealtInScope bs)} -> Unpreventable bs

  public export
  unpreventableIntro : {bs : Bindings} -> Unpreventable bs -> Bindings
  unpreventableIntro (DamageDescribed scope by) = byIntro by
  unpreventableIntro ThatDamage = bs

  ||| WHAT the statement refuses. [CR#615.12] names the plain refusal --
  ||| "Some effects state that damage 'can't be prevented'" -- and
  ||| [CR#614.9] names the other half of the conjoined one: an effect that
  ||| replaces damage dealt to one recipient with the same damage dealt to
  ||| another "such effects are called redirection effects". So the wider
  ||| ban stops two different continuous effects, and the arms are a
  ||| closed pair rather than a list: the rules name exactly these two
  ||| things a damage event can have done to it short of being modified in
  ||| size, and no printed line refuses a third.
  ||| 2 supported sentences write the conjoined ban (Lava Burst,
  ||| Whippoorwill), both spelling it "can't be prevented or dealt instead
  ||| to another permanent or player"; the rest of the corpus writes the
  ||| plain one. Measured 2026-08-28.
  ||| A FLAG and not a coordination of two statements: one subject, one
  ||| modality and one negation scope over both refusals, which is what
  ||| the printed "or" is under.
  ||| -- spelling: "can't be prevented"; under `NoRedirectEither`, "can't
  ||| be prevented or dealt instead to another permanent or player".
  public export
  data PreventionBan = NoPreventionOnly | NoRedirectEither

  ||| How much of ONE damage event a prevention takes. [CR#615.10] is the
  ||| rule the whole vocabulary answers to: it writes "If a source would
  ||| deal damage to you, prevent 1 of that damage" in its own words and
  ||| has such an effect prevent "only the indicated amount of damage in
  ||| any applicable damage event at any given time".
  ||| The written count (`CutSome`) is that rule's own example. The other
  ||| two arms indicate the amount some other way and are arms here rather
  ||| than `Amount` rows because neither has an amount to name: the damage
  ||| event's own size is not a phrase this grammar spells, so "all but
  ||| 1 of that damage" and "half that damage" can only be said of the
  ||| event a cut is already attached to.
  public export
  data PreventCut : Bindings -> Type where
    CutAll : PreventCut bs
    CutSome : (amt : Amount bs) -> PreventCut bs
    ||| "prevent all but [n] of that damage": the cut written as the
    ||| damage it LEAVES rather than the damage it takes. 4 supported
    ||| sentences over 4 cards (Ajani Steadfast's emblem, Forcefield,
    ||| Hyperion, Supreme Hero, Temple Altisaur), re-measured 2026-08-28.
    ||| The SHIELD seat's twin is a measured ZERO: no card in the corpus,
    ||| supported or not, writes "prevent all but [n] of the damage that
    ||| would be dealt ... this turn". All four printings are per-event
    ||| cuts -- three if-would statics and Forcefield's next-time -- so the
    ||| complement is one arm here and `Shield` grows none.
    ||| -- spelling: "prevent all but [amt] of that damage".
    CutAllBut : (amt : Amount bs) -> PreventCut bs
    ||| "prevent half that damage, rounded down" (Dark Sphere) / "rounded
    ||| up" (Gisela, Blade of Goldnight): the cut written as a FRACTION of
    ||| the event. 2 supported sentences over 2 cards, one at each
    ||| rounding direction, so the two cards populate both cells of the
    ||| one arm.
    ||| The rounding word is obligatory and not a `Maybe`: [CR#107.1a]
    ||| leaves a fraction unrounded only where the card says how, so a
    ||| halving that named no direction would state no amount.
    ||| Not `Amount`'s `Half`, which halves a written amount: there is no
    ||| amount here to write, only the event the cut is attached to.
    ||| -- spelling: "prevent half that damage, rounded [r]".
    CutHalf : (r : RoundMode) -> PreventCut bs

  public export
  cutIntro : {bs : Bindings} -> PreventCut bs -> Bindings
  cutIntro CutAll = bs
  cutIntro (CutSome amt) = amtIntro amt
  cutIntro (CutAllBut amt) = amtIntro amt
  cutIntro (CutHalf _) = bs

  public export
  data DamageScale : Bindings -> Type where
    Multiplied : (f : ScaleFactor) -> DamageScale bs
    Halved : (r : RoundMode) -> DamageScale bs
    Shifted : (d : ShiftDir) -> (amt : Amount bs) ->
              DamageScale bs

  public export
  scaleIntro : {bs : Bindings} -> DamageScale bs -> Bindings
  scaleIntro (Multiplied _) = bs
  scaleIntro (Halved _) = bs
  scaleIntro (Shifted _ amt) = amtIntro amt

  public export
  isCoord : {0 bs : Bindings} -> StaticEffect bs -> Bool
  isCoord (AndAlso _) = True
  isCoord (OfSubject _ _) = True
  isCoord _ = False

  public export
  NotCoord : StaticEffect bs -> Type
  NotCoord {bs} se = So (not (isCoord se))

  ||| "The same is true for …" is a rider on ONE statement, so a second
  ||| extension repeats the first. Nothing else is refused: no rule forbids a
  ||| static effect from naming the cards off the battlefield it also reaches.
  public export
  notExtended : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notExtended (AlsoOffBattlefield _) = False
  notExtended _ = True

  public export
  NotExtended : StaticEffect bs -> Type
  NotExtended {bs} se = So (notExtended se)

  ||| "This effect doesn't remove [n]" carves ONE object out of ONE
  ||| statement, so a second rider on the same statement says nothing the
  ||| first did not -- `AlsoOffBattlefield`'s reason, at the other
  ||| trailer. Nothing else is refused: which statements the rider means
  ||| anything on is the carrier's business, not a rules impossibility.
  public export
  notCarvedOut : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notCarvedOut (DoesntRemove _ _) = False
  notCarvedOut _ = True

  public export
  NotCarvedOut : StaticEffect bs -> Type
  NotCarvedOut {bs} se = So (notCarvedOut se)

  public export
  staticKind : {0 bs : Bindings} -> StaticEffect bs -> StaticKind
  staticKind (Define _ _) = LetterDefinition
  staticKind (Gets _ _ _) = PtDelta
  staticKind (DefinesPt _ _ _) = PtDefinition
  staticKind (HasBasePt _ _ _) = BasePtSet
  staticKind (SwitchesPt _) = PtSwitch
  staticKind (CostsToCast _ _) = CostModification
  staticKind (AltCost _) = CostModification
  staticKind (AddedCost _ _) = CostModification
  staticKind (Gains _ _) = KeywordGrant
  staticKind (GainsAbilitiesOf _ _ _ _) = KeywordGrant
  staticKind (Deontic _ _ _ _ _ _ _) = DeedRestriction
  staticKind (DoesntUntap _) = DeedRestriction
  staticKind (CantMoreThan _ _ _ _) = DeedRestriction
  staticKind (Skips _ _) = TurnSkip
  -- [CR#106.4]'s emptying is a rule of the game rather than an act any
  -- object performs, so the sentence that overrides it restricts no
  -- deed and modifies no cost.
  staticKind (KeepsUnspentMana _ _) = ManaPersistence
  staticKind (MayDeclineUntap _) = DeedRestriction
  -- an addition to [CR#502.3]'s turn-based action, restricting no deed
  -- and replacing no event: its own kind, beside the land allowance's.
  staticKind (UntapsDuringStep _) = UntapGrant
  staticKind (BecomesAlso _ _) = TypeAddition
  staticKind (AddsEveryType _ _) = TypeAddition
  staticKind (LosesEveryType _ _) = TypeLoss
  staticKind (LosesType _ _) = TypeLoss
  staticKind (SetsColor _ _) = ColorSet
  staticKind (BecomesCopy _ _ _) = CopyEffect
  staticKind (SetsType _ _ _) = TypeSet
  staticKind (AddsChosenQuality _ _) = TypeAddition
  staticKind (SetsChosenQuality _ _) = TypeSet
  staticKind (LosesAllAbilities _ _) = AbilityLoss
  staticKind (LosesAbilities _ _) = AbilityLoss
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _ _ _ _) = Replacement
  staticKind (Prevents _ _ _ _ _) = Prevention
  staticKind (PreventsFrom _ _ _ _ _ _) = Prevention
  staticKind (CantPrevent _ _ _) = Prevention
  staticKind (Redirects _ _ _ _ _) = Replacement
  staticKind (EntersUnderInstead _ _) = Replacement
  staticKind (RedirectsFrom _ _ _ _ _) = Replacement
  staticKind (Scales _ _ _ _ _) = Replacement
  staticKind (OnlyDuring _ _ se) = staticKind se
  staticKind (Conditionally _ _ _) = Conditional
  staticKind (OnlyWhile _ _ _) = Conditional
  staticKind (AlsoOffBattlefield se) = staticKind se
  staticKind (DoesntRemove se _) = staticKind se
  staticKind (NoLossFrom _ _) = OutcomeImmunity
  staticKind (Visibility _ _ _) = VisibilityRider
  staticKind (MayPlayAdditionalLands _ _) = LandAllowance
  staticKind (MayBlockAdditional _ _) = BlockAllowance
  staticKind (EntersRider _ _) = EntryRider
  -- an entry rider and not a `CopyEffect`: what it modifies is HOW the
  -- permanent enters [CR#614.12], and it stands as long as the permanent
  -- does, which is `EntersRider`'s seat and not `BecomesCopy`'s.
  staticKind (EntersAsCopy _ _ _ _) = EntryRider
  staticKind (EntersWithCounters _ _ _ _) = EntryRider
  staticKind (EntersChoice _ _ _) = EntryRider
  staticKind (AttachChoice _ _ _) = Replacement
  staticKind (AndAlso _) = Coordination
  staticKind (OfSubject _ _) = Coordination


  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (Define l amt) = defineLetter l (amtIntro amt)
  staticIntro (Gets n pow tou) = shiftDelta tou ++ shiftDelta pow ++ selfSubjIntro n
  -- a definition names a number outright, and a sibling slot in the same
  -- statement reads it back as "that number" (Lhurgoyf) — the quantity
  -- anaphor's machinery, spelled "that number" after a definition.
  staticIntro (DefinesPt n _ amt) =
    outcomeB NamedNumber :: (amtDelta amt ++ selfSubjIntro n)
  staticIntro (HasBasePt n pow tou) = amtDelta tou ++ amtDelta pow ++ selfSubjIntro n
  staticIntro (SwitchesPt n) = selfSubjIntro n
  staticIntro (CostsToCast n sh) = costShiftDelta sh ++ selfSubjIntro n
  staticIntro (AltCost _) = bs
  -- The cost is a statement about the payment, not a clause that
  -- announces anything: the additional cost is paid at [CR#601.2f..601.2h],
  -- long before any line of the card reads a mention.
  staticIntro (AddedCost _ _) = bs
  -- A granted keyword whose NUMBER parameter is a letter opens that
  -- letter for the where-clause that follows: "Ulamog has annihilator X,
  -- WHERE X IS the number of +1/+1 counters on it" [CR#702.86a] writes
  -- "Annihilator N" and the card puts a variable in the slot, which
  -- [CR#107.3f] leaves to be defined by the text -- and this is the text
  -- defining it. `Define` is the closing half and already exists; what
  -- was missing was the OPENING, since `ParamNumber`'s amount is typed
  -- at `[]` and its own `amtIntro` reaches nothing here.
  staticIntro (Gains n ab) = abLetterDelta ab ++ selfSubjIntro n
  -- the source is named after the subject, so what it announces is what
  -- the statement leaves; the class words and the exception describe
  -- abilities and name no object.
  staticIntro (GainsAbilitiesOf n _ src _) = nomIntro src
  staticIntro (Deontic n _ _ _ _ _ _) = selfSubjIntro n
  staticIntro (DoesntUntap n) = selfSubjIntro n
  staticIntro (CantMoreThan _ _ _ _) = bs
  staticIntro (Skips _ _) = bs
  staticIntro (KeepsUnspentMana who _) = nomIntro who
  staticIntro (MayDeclineUntap n) = selfSubjIntro n
  staticIntro (UntapsDuringStep n) = selfSubjIntro n
  staticIntro (BecomesAlso n _) = selfSubjIntro n
  staticIntro (AddsEveryType n _) = selfSubjIntro n
  staticIntro (LosesEveryType n _) = selfSubjIntro n
  staticIntro (LosesType n _) = selfSubjIntro n
  staticIntro (SetsColor n _) = selfSubjIntro n
  staticIntro (BecomesCopy n _ _) = selfSubjIntro n
  staticIntro (SetsType n _ _) = selfSubjIntro n
  staticIntro (AddsChosenQuality n _) = selfSubjIntro n
  staticIntro (SetsChosenQuality n _) = selfSubjIntro n
  staticIntro (LosesAllAbilities n _) = selfSubjIntro n
  staticIntro (LosesAbilities n _) = selfSubjIntro n
  -- the control change is a labeled act of its own: [CR#613.1b]
  -- applies it in layer 2 and [CR#110.2] makes the controller a
  -- property of the permanent, which stays where it stands. So the row
  -- stamps in place rather than moving, on `Search`'s model of a
  -- constructor that names its own label, and "Gain control of target
  -- creature until end of turn. Untap it" reads the stamped mention
  -- back through `ItVerbed "GainControl"` even where the ability's
  -- header announced a permanent of its own.
  staticIntro (GainsControl who what) = stampIntro (Just "GainControl") what
  staticIntro (Intercepts ev alts window repl use limit) = interceptCtx alts ev
  staticIntro (Prevents kind size scope by also) = byIntro by
  staticIntro (PreventsFrom kind src scope cut use also) = cutIntro cut
  staticIntro (CantPrevent kind what ban) = unpreventableIntro what
  staticIntro (Redirects kind size scope by to) = nomIntro to
  staticIntro (EntersUnderInstead n who) = nomIntro who
  staticIntro (RedirectsFrom kind src scope to use) = nomIntro to
  staticIntro (Scales kind src scope op use) = scaleIntro op
  staticIntro (OnlyDuring _ _ se) = staticIntro se
  staticIntro (Conditionally c se _) = staticIntro se
  staticIntro (OnlyWhile se c _) = staticIntro se
  staticIntro (AlsoOffBattlefield se) = staticIntro se
  staticIntro (DoesntRemove _ n) = nomIntro n
  staticIntro (NoLossFrom who _) = nomIntro who
  staticIntro (Visibility _ who what) = visibleIntro what
  staticIntro (MayPlayAdditionalLands who _) = nomIntro who
  staticIntro (MayBlockAdditional n _) = selfSubjIntro n
  staticIntro (EntersRider n _) = selfSubjIntro n
  -- the SUBJECT alone. The copy source is a constructor argument and
  -- never a mention, on `TokenCopyOf`'s law: a second singular object in
  -- the discourse would leave the 38 "except it" lines with two
  -- antecedents apiece.
  staticIntro (EntersAsCopy n _ _ _) = selfSubjIntro n
  staticIntro (EntersWithCounters n amt _ _) = amtDelta amt ++ selfSubjIntro n
  staticIntro (EntersChoice n _ _) = selfSubjIntro n
  staticIntro (AttachChoice n _ _) = selfSubjIntro n
  staticIntro (AndAlso parts) = partsIntro parts
  staticIntro (OfSubject n vps) = vpsIntro vps

  ||| The choice a STATEMENT binds, for the abilities after it to read
  ||| [CR#607.2d]. A delta rather than a context because a coordination
  ||| binds as many as it has parts: "choose a color and a creature
  ||| type" is ONE sentence of two choices (Riptide Replicator,
  ||| Volrath's Laboratory; "choose a color and an opponent", Call to
  ||| Arms), and what it wanted was the spelling of two choosers in one
  ||| statement, not a second linkage mechanism.
  public export
  staticChoiceDelta : {0 bs : Bindings} -> StaticEffect bs -> List Binding
  staticChoiceDelta (EntersChoice _ q _) = [choiceB q]
  staticChoiceDelta (AttachChoice _ q _) = [choiceB q]
  staticChoiceDelta (AndAlso parts) = partsChoiceDelta parts
  -- The chooser position an additional cost announces from: 6 supported
  -- lines write "As an additional cost to cast this spell, choose ..."
  -- (Caller of the Hunt, Close Encounter, Liquid Fire, and three under a
  -- "you may"), and the line that reads "the chosen [value]" is
  -- [CR#607.2d]'s linked ability -- the same link the as-enters rider
  -- and the ability bodies export across, at the position the cost
  -- occupies. Only the choice crosses: what a cost's ACTION stamped
  -- ("the number of creatures tapped this way", Burn at the Stake) is
  -- the effect's whole delta, and `effDelta` needs `bs` un-erased where
  -- this function has it at multiplicity 0, so that read is the row's
  -- one recorded remainder.
  staticChoiceDelta (AddedCost c _) = costChoiceDelta c
  staticChoiceDelta _ = []

  public export
  staticChoiceIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticChoiceIntro se = staticChoiceDelta se ++ bs

  public export
  data DividedVerb : Bindings -> Type where
    DividedDamage : (src : Noun bs Object) -> DividedVerb bs
    DistributedCounters : (kind : CounterKind) -> DividedVerb bs

  public export
  divIntro : {bs : Bindings} -> DividedVerb bs -> Bindings
  divIntro (DividedDamage src) = nomIntro src
  divIntro (DistributedCounters _) = bs

  public export
  data DivTag = DivDamage | DivCounters

  public export
  divTag : {0 bs : Bindings} -> DividedVerb bs -> DivTag
  divTag (DividedDamage _) = DivDamage
  divTag (DistributedCounters _) = DivCounters

  public export
  data DividedTakes : DivTag -> Noun bs k -> Type where
    DamageDivided : {auto 0 rk : DamageRecipient n} -> DividedTakes DivDamage n
    CountersDistributed : DividedTakes DivCounters {k = Object} n

  public export
  data CounterRider : Bindings -> Type where
    MkCounterRider : (amt : Amount bs) -> (kind : CounterKind) ->
                     CounterRider bs

  public export
  data MoveRiders : Bindings -> Type where
    MkMoveRiders : (entry : List TokenRider) ->
                   (ctrl : Maybe (Noun bs Player)) ->
                   (counters : Maybe (CounterRider bs)) ->
                   {auto 0 one : CtrlOverrideOk ctrl} -> MoveRiders bs

  ||| Which written controller a move may name. [CR#110.2] gives a
  ||| permanent ONE controller, so a plural player word in the slot names
  ||| no controller for anything -- that is `OneController`, and the
  ||| refusal it leaves is what `badMoveRidersPluralController` pins.
  ||| The per-member possessor is not that. "Return the exiled cards to
  ||| the battlefield under their owners' control" gives EACH card the
  ||| one controller [CR#110.2] demands and is plural only because the
  ||| moved group is; the rule it would break is broken by nothing here.
  ||| (Named `CtrlSingular` while `OneController` was its only positive
  ||| case; the name went with the case rather than the question.)
  public export
  data CtrlOverrideOk : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoOverride : CtrlOverrideOk Nothing
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlOverrideOk (Just n)
    PerMemberController : {0 bs : Bindings} -> {0 ax : PossessorAxis} ->
                          {0 grp : Noun bs Object} ->
                          {0 pl : nounPlur grp = ManyOf} ->
                          CtrlOverrideOk (Just (PossessorsOf ax grp {pl}))

  public export
  fieldRidersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  fieldRidersWritten (MkMoveRiders [] Nothing _) = False
  fieldRidersWritten _ = True

  public export
  counterRiderWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  counterRiderWritten (MkMoveRiders _ _ Nothing) = False
  counterRiderWritten (MkMoveRiders _ _ (Just _)) = True

  public export
  ridersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  ridersWritten r = fieldRidersWritten r || counterRiderWritten r

  public export
  ridersFitZone : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Bool
  ridersFitZone r z =
    not (fieldRidersWritten r) || z == Battlefield

  public export
  RidersFit : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Type
  RidersFit r z = So (ridersFitZone r z)

  public export
  data Cost : Bindings -> Type where
    Mana : (c : ManaCost) -> {auto 0 wr : ManaRun c} -> Cost bs
    ||| "pay {2} for each age counter on it" [CR#702.24a], "pay {G} for
    ||| each wind counter on it" (Cyclone): a mana payment whose size is
    ||| a COUNT, the amount carrying both the per-unit numeral and the
    ||| thing counted.
    |||
    ||| The unit is what the coloured lines want and what the generic
    ||| amount could not say: [CR#107.4a] makes coloured mana in a cost
    ||| payable only with mana of that colour, where generic mana takes
    ||| any type [CR#107.4b], so "{G} for each" and "{1} for each" differ
    ||| in what may pay them and not only in how they are written. 3
    ||| supported lines write a run (Cyclone {G}, Thelon's Curse {U},
    ||| Norn's Annex {W/P}); the rest write `GenericUnit`.
    |||
    ||| TWO MEASURED ZEROS, recorded and not pinned -- no rule refuses
    ||| either, and a count is never a refusal. (1) No supported line
    ||| writes a scaled ALTERNATIVE cost: the row reaches `AltCost`
    ||| structurally and nothing in [CR#118.9] would forbid one, so it is
    ||| owed nothing and refused nothing. Liesa, Shroud of Dusk is the
    ||| near miss and not the case -- it DECLINES a scaled cost and
    ||| substitutes a repeated life payment, which is the repetition
    ||| channel's. (2) No supported line writes a scaled non-mana
    ||| ADDITIONAL cost: "as an additional cost ... for each" returns
    ||| nothing, and [CR#118.8] would allow it.
    ScaledMana : (unit : ManaUnit) -> (amt : Amount bs) ->
                 {auto 0 fe : ForEachAmount amt} -> Cost bs
    TapSymbol : Cost bs
    UntapSymbol : Cost bs
    LoyaltySymbol : (s : LoyaltyCost) -> Cost bs
    Do : (e : Effect bs) -> {auto 0 ok : CostAction e} -> Cost bs
    Compound : {0 n : Nat} -> CostSeq n bs ->
               {auto 0 ne : IsSucc n} -> Cost bs
    ||| "[l] or [r]": a payer's CHOICE between two costs. "Cumulative
    ||| upkeep {G} or {W}" (Arctic Nishoba, Earthen Goo {R} or {G},
    ||| Jotun Owl Keeper {W} or {U}, Krovikan Whispers {U} or {B} -- 4
    ||| supported cards, measured 2026-08-28).
    |||
    ||| ON `Cost` AND NOT ON `ManaCost`. The English "or" joins two
    ||| RUNS, and `ManaCost` is a list of symbols with no room for a
    ||| choice between two of them: widening it would make every mana
    ||| cost in the grammar a potential disjunction to serve four cards,
    ||| and would still say nothing for a carrier whose arms are not
    ||| mana. Sitting here it serves every cost carrier at once, which
    ||| is the same reason `Compound` sits here -- that row is the
    ||| AND-join [CR#601.2h] and this is the OR.
    ||| Nor is it a hybrid symbol: [CR#107.4e] makes a hybrid symbol one
    ||| symbol of the cost, where these cards print two runs and the
    ||| word "or".
    ||| WHAT THE CHOICE MEANS is the keyword's own rule where a keyword
    ||| carries it: [CR#702.24a] says "if [cost] has choices associated
    ||| with it, each choice is made separately for each age counter,
    ||| then either the entire set of costs is paid, or none of them is
    ||| paid", so cumulative upkeep's disjunction is answered once per
    ||| age counter and not once for the whole payment.
    ||| IT ANNOUNCES NOTHING: `costIntro` is `bs`, because the payment
    ||| does not record which arm was taken and no later clause can read
    ||| one. An arm that would announce (a `Do` cost) therefore loses
    ||| what it announced, which is the construction's content rather
    ||| than an omission.
    ||| The arms are flat for `CostSeq`'s reason and not a new one: that
    ||| list refuses a nested `Compound` at its own cons, so a join
    ||| inside a join is spelled by neither row. A narrowing, asserted
    ||| by nothing, and no supported line pays for it.
    ||| -- spelling: "[l] or [r]".
    EitherCost : (l : Cost bs) -> (r : Cost bs) ->
                 {auto 0 nl : NotCompound l} ->
                 {auto 0 nr : NotCompound r} -> Cost bs
    ||| "its mana cost": the bearer's own printed cost [CR#202.1a], the
    ||| cost a sentence fixing a granted keyword's parameter names.
    ItsManaCost : Cost bs

  ||| Whether an amount is written as a PER-EACH product -- the shape a
  ||| scaled payment needs, and the only one it admits. Both product arms
  ||| answer yes: they differ in whether the per-unit was printed as a
  ||| numeral or is read [CR#118.4,107.3a], which is no difference to
  ||| this question.
  public export
  forEachAmount : {0 bs : Bindings} -> Amount bs -> Bool
  forEachAmount (Times _ _) = True
  forEachAmount (TimesOf _ _) = True
  forEachAmount _ = False

  public export
  ForEachAmount : Amount bs -> Type
  ForEachAmount {bs} a = So (forEachAmount a)

  public export
  costIntro : {bs : Bindings} -> Cost bs -> Bindings
  costIntro (Mana c) = if manaHasX c then letterB X :: bs else bs
  costIntro (ScaledMana _ _) = bs
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (LoyaltySymbol LoyaltyDownX) = letterB X :: bs
  costIntro (LoyaltySymbol _) = bs
  costIntro (Do e) = effIntro e
  costIntro (Compound cs) = costsIntro cs
  -- the payment does not record which arm was taken.
  costIntro (EitherCost _ _) = bs
  costIntro ItsManaCost = bs

  public export
  isCompound : {0 bs : Bindings} -> Cost bs -> Bool
  isCompound (Compound _) = True
  isCompound _ = False

  public export
  NotCompound : Cost bs -> Type
  NotCompound {bs} c = So (not (isCompound c))

  public export
  isLoyalty : {0 bs : Bindings} -> Cost bs -> Bool
  isLoyalty (LoyaltySymbol _) = True
  isLoyalty _ = False

  public export
  data ProducedMana : Bindings -> Type where
    Runs : (rs : List ProducedRun) ->
           {auto 0 ok : ProducedRuns rs} -> ProducedMana bs
    AnyColor : ColorFreedom -> ProducedMana bs
    OfChosenColor : (alt : Maybe ProducedRun) ->
                    {auto 0 ar : AltRunWritten alt} ->
                    {auto 0 cq : countChoice (QSort Color) bs = 1} ->
                    {auto 0 rd : ChosenQualityRead Color} -> ProducedMana bs
    ||| "Add mana equal to enchanted permanent's mana cost" (Elemental
    ||| Resonance): a production the card names by a PRINTED COST and
    ||| leaves the rules to translate. 1 supported line, measured
    ||| 2026-08-28.
    |||
    ||| Its own row, never a widening of `ProducedRun`. A run is a list
    ||| of [CR#106.1b]'s six mana types, and a printed cost is not one:
    ||| [CR#202.1] builds it out of [CR#107.4]'s symbols, and four of
    ||| those name no type until the rules act -- a hybrid symbol has its
    ||| half chosen [CR#106.8], a Phyrexian symbol adds mana of its
    ||| colour [CR#106.9], a generic symbol adds that much colorless
    ||| [CR#106.10] and snow symbols do the same [CR#106.11]. So the card
    ||| names the cost and those four rules say what is added; widening
    ||| the run would make the CARD state a translation it does not
    ||| write.
    |||
    ||| Ice Cauldron's "Add this artifact's last noted type and amount of
    ||| mana" is the family's second line and is NOT this row: what it
    ||| names is a note its own earlier ability took of mana that was
    ||| SPENT, not a cost printed anywhere, and this grammar has no note
    ||| channel. 1 line, its own gap.
    ||| -- spelling: "mana equal to [n]'s mana cost".
    AsPrintedCost : (n : Noun bs Object) ->
                    {auto 0 one : nounPlur n = OneOf} -> ProducedMana bs
    ||| "Add one mana of any type that land produced" (Mana Flare,
    ||| Vorinclex, Zendikar Resurgent): the type the AMBIENT mana
    ||| production actually made. 17 supported sentences over 17 cards
    ||| (measured 2026-08-28).
    |||
    ||| Gated on a production being in scope, and the gate is the whole
    ||| point: [CR#106.12a] fires the tapped-for-mana trigger "whenever
    ||| such a mana ability resolves and produces mana", so there is a
    ||| type to read only inside such a header. All 17 sentences sit
    ||| inside one -- a total covariance, and the warrant for gating here
    ||| rather than trusting the noun to be a land that was tapped.
    |||
    ||| `n` is the source whose production is read, named by the sentence
    ||| ("that land", "that permanent", "that artifact token") and not
    ||| assumed to be the header's subject: Extraplanar Lens writes the
    ||| controller's add of a land the header already picked out, and
    ||| Kinnan reads a nonland permanent.
    |||
    ||| It is NOT `CouldProduce`, and the two must not be measured
    ||| together: this row reads what an ability DID produce, where
    ||| [CR#106.7] defines a hypothetical over what a permanent "would
    ||| produce if the ability were to resolve at that time". They share
    ||| the phrase "mana of any type that", which is how a regex over it
    ||| once returned 24 for this row alone.
    ||| -- spelling: "one mana of any type that [n] produced".
    ProducedByEvent : (n : Noun bs Object) ->
                      {auto 0 one : nounPlur n = OneOf} ->
                      {auto 0 pm : countOutcomes ManaProduced bs = 1} ->
                      ProducedMana bs
    ||| "Add one mana of any color that a land an opponent controls could
    ||| produce" (Exotic Orchard, Fellwar Stone, Reflecting Pool): the
    ||| HYPOTHETICAL read. [CR#106.7] defines it -- the types a permanent
    ||| could produce "include[] any type of mana that an ability of that
    ||| permanent would produce if the ability were to resolve at that
    ||| time, taking into account any applicable replacement effects in
    ||| any possible order", ignoring whether the ability's costs could be
    ||| paid. 18 supported lines over 18 cards, of which 15 are add
    ||| payloads (measured 2026-08-28).
    |||
    ||| Ungated, where `ProducedByEvent` is gated, and the difference is
    ||| the rules': a hypothetical needs no event to have happened, only a
    ||| permanent to ask about, and [CR#106.7] answers "there's no type of
    ||| mana it could produce" where the question comes back empty rather
    ||| than leaving the sentence undefined.
    |||
    ||| The subject is PLURAL where the line writes one -- "a land you
    ||| control could produce" ranges over every such land -- so no
    ||| singular gate rides it.
    ||| -- spelling: "one mana of any [color/type] that [n] could
    ||| produce".
    CouldProduce : (n : Noun bs Object) -> ProducedMana bs
    ||| "Add one mana of any of the exiled card's colors" (Chrome Mox),
    ||| "of the exiled cards' colors" (Pit of Offerings), "X mana in any
    ||| combination of its colors" (Omnath, Locus of All): the colour SET
    ||| read off a mentioned object. 3 supported lines (measured
    ||| 2026-08-28).
    |||
    ||| Its own row and not a `ProducedRun`: a run is a list of types the
    ||| LINE writes [CR#106.1b], where this names a set the card does not
    ||| know until the object is there to read. [CR#105.2] is what makes
    ||| the read well formed -- an object "can be one or more of the five
    ||| colors, or it can be no color at all" -- so a colorless card
    ||| answers the empty set, which the row inherits rather than a gap.
    ||| It is not `OfChosenColor` either: no choice was made, and nothing
    ||| here reads back a chooser's answer.
    ||| -- spelling: "one mana of any of [n]'s colors".
    AmongColorsOf : (n : Noun bs Object) -> ProducedMana bs
    ||| "Add two mana in any combination of {R} and/or {G}": the
    ||| combination over a WRITTEN colour set, where `AnyColor EachColor`
    ||| ranges over all five. 12 supported sentences (measured
    ||| 2026-08-28), against 34 that write "in any combination of colors".
    |||
    ||| A written SET and not a run: `Runs` is what the line spends, one
    ||| symbol after another, where this leaves every unit free within a
    ||| set the line names. Two or more members, because a one-member
    ||| combination is that member repeated and `Runs` already says it.
    ||| Colours and not types: no supported line writes {C} into one of
    ||| these sets, and [CR#106.1a]'s five are what the printed sets draw
    ||| from.
    ||| -- spelling: "[amt] mana in any combination of [cs]".
    AmongWritten : (cs : List Color) ->
                   {auto 0 tw : So (colorCountOk (length cs))} ->
                   ProducedMana bs
    ||| "Add this artifact's last noted type and amount of mana" (Ice
    ||| Cauldron): the production a NOTE names. 1 supported line.
    |||
    ||| An anchored ungated read, on `GreatestStoredMatch`'s model and by
    ||| the same ruling: the note is STATE on the holder, not a value in a
    ||| name-keyed channel, and it is read off the permanent that holds it
    ||| rather than off an identifier the author invents. [CR#607.2e] is
    ||| what makes the pair one statement -- "if an object has an ability
    ||| printed on it that allows some information to be noted and another
    ||| ability which refers to information noted for that object, those
    ||| abilities are linked", and "the second ability refers only to
    ||| information noted as a result of the first". So the link is the
    ||| card's, exactly as [CR#607.2d]'s chosen-value link is, and no gate
    ||| here asks whether anything was noted: a permanent whose text never
    ||| noted is that link's business.
    |||
    ||| What is noted is mana that was SPENT, which is why this is not
    ||| `AsPrintedCost`: Ice Cauldron's first ability notes "the type and
    ||| amount of mana spent to pay this activation cost", a payment that
    ||| happened, where `AsPrintedCost` names a cost printed on a card.
    ||| The NOTING side is not built and is this row's remaining gap: no
    ||| effect here records state on a permanent, and minting one is the
    ||| stored-result family's shape rather than this row's.
    ||| -- spelling: "[n]'s last noted type and amount of mana".
    LastNotedMana : (n : Noun bs Object) ->
                    {auto 0 one : nounPlur n = OneOf} -> ProducedMana bs

  ||| Which of [CR#106.6]'s two EFFECT-BEARING riders a clause writes.
  ||| The rule lists them separately -- a production may "have an
  ||| additional effect that affects the spell or ability that mana is
  ||| spent on, or create a delayed triggered ability ... that triggers
  ||| when that mana is spent" [CR#603.7a] -- and the difference is
  ||| real rather than editorial: a delayed trigger uses the stack and
  ||| can be responded to where an additional effect cannot, and
  ||| [CR#106.6a] doubles them differently when a replacement increases
  ||| the mana ("a separate delayed triggered ability is created for each
  ||| mana produced", against one effect "once for each mana produced").
  ||| The corpus writes the difference too: "IF that mana is spent on a
  ||| creature spell, it gains haste" against "WHEN that mana is spent to
  ||| cast a creature spell, scry 1".
  public export
  data SpentMode = AffectsIt | TriggersThen

  ||| A per-mana string attached to produced mana. [CR#106.6] enumerates
  ||| exactly what one may say -- a production "restricts how that mana
  ||| can be spent, [has] an additional effect that affects the spell or
  ||| ability that mana is spent on, or create[s] a delayed triggered
  ||| ability ... that triggers when that mana is spent" -- so the rows
  ||| here are that list and nothing else.
  |||
  ||| WHAT IS NOT A RIDER, and the omission is the rule's: the mana's
  ||| PERSISTENCE. [CR#106.4] empties every pool at the end of each step
  ||| and phase, and "you don't lose this mana as steps and phases end"
  ||| overrides that -- but it is not on [CR#106.6]'s list, and the
  ||| corpus writes it as its own sentence with its own subject and its
  ||| own duration. 32 supported lines carry the body (measured
  ||| 2026-08-28): 25 say it of mana a preceding sentence added ("this
  ||| mana", all 25 under a written span) and 7 say it of a player's
  ||| unspent mana generally, of which 6 write no span at all and belong
  ||| to permanents that add no mana anywhere (Omnath Locus of Mana,
  ||| Leyline Tyrant, Upwelling). A rider slot could not have been
  ||| written by those 6, and a rider would have had to invent the span
  ||| the other 25 already spell with `Continuously`. So persistence is
  ||| `KeepsUnspentMana` under the ordinary duration envelope, and the
  ||| mana it names is read through the `ManaAdded` mention rather than
  ||| held by an attachment.
  |||
  ||| Snow is not a rider either, and the rules put it somewhere else
  ||| outright: [CR#106.3] makes the SOURCE of an ability's mana the
  ||| source of that ability, and [CR#205.4a] makes snow a supertype of
  ||| that permanent -- so [CR#107.4h]'s "{S}" is answered by reading the
  ||| producer's type line, never by a string carried on the mana.
  public export
  data ManaRider : Bindings -> Type where
    ||| "Spend this mana only to cast a creature spell": [CR#106.6]'s
    ||| restriction, in the positive. 164 supported sentences.
    SpendOnly : (ps : List (SpendPurpose bs)) ->
                {auto 0 ne : SpendPurposes ps} -> ManaRider bs
    ||| "This mana can't be spent to cast a nonartifact spell": the SAME
    ||| restriction stated by what it excludes. 9 supported lines
    ||| (measured 2026-08-28) -- Battery Bearer, Hydraulic Helper,
    ||| Jegantha, Jetfire, Karn Legacy Reforged, Karolina Dean, The
    ||| Mightstone and Weakstone, Thran Turbine, Vhal. A naive count
    ||| returns 40; 31 of those are the Powerstone token's REMINDER text
    ||| inside parentheses on cards that make one, and reminder text is
    ||| not a line this grammar writes.
    |||
    ||| Its own row beside `SpendOnly` rather than a polarity flag on it,
    ||| because the two say different things about the mana's OTHER
    ||| purposes: "only to cast a creature spell" forbids every purpose
    ||| but one, where "can't be spent to cast a nonartifact spell"
    ||| leaves every purpose but one open. A flag would make the pair
    ||| look like one statement read two ways, which they are not.
    |||
    ||| It is NOT a `Deontic`: no `Kind` names mana, so the subject the
    ||| carrier demands does not exist, and the restriction is a fact
    ||| about mana AS IT IS PRODUCED [CR#106.6] rather than a standing
    ||| prohibition on a permanent. It names its complement through
    ||| `SpendPurpose` and not through a noun, which is what the deontic
    ||| complement would have given it: Jegantha's "can't be spent to pay
    ||| generic mana costs" names a COST and no noun describes one, and
    ||| [CR#107.4b] is why -- numerical symbols "represent generic mana in
    ||| costs", a component of a cost rather than an object.
    SpendNotOn : (ps : List (SpendPurpose bs)) ->
                 {auto 0 ne : SpendPurposes ps} -> ManaRider bs
    ||| "If that mana is spent on a creature spell, it gains haste until
    ||| end of turn" and "When that mana is spent to cast a creature
    ||| spell that shares a creature type with your commander, scry 1":
    ||| [CR#106.6]'s two effect-bearing riders, sharing one row because
    ||| they share the only thing this grammar was missing -- a mention of
    ||| the OBJECT the mana was spent on. 14 supported cells (measured
    ||| 2026-08-28): 11 additional effects and 3 delayed triggers
    ||| ([CR#603.7a] -- Path of Ancestry, Primal Amulet, Pyromancer's
    ||| Goggles).
    |||
    ||| THE PAID-FOR OBJECT IS BOUND ONCE, by `what`, and every reading
    ||| of it comes off that one mention: "IT gains haste" (Arena of
    ||| Glory), "THAT SPELL can't be countered" (Boseiju), "THAT CREATURE
    ||| enters with an additional +1/+1 counter" (Animal Attendant),
    ||| "copy THAT SPELL" (Primal Amulet). It is a `Noun` and not a
    ||| `Predicate` for exactly that reason: a predicate describes and
    ||| announces nothing, and these bodies all read back.
    |||
    ||| `only` is the difference between the two spellings the corpus
    ||| writes, and it is why the restriction is not a second rider here:
    ||| Cavern of Souls and Delighted Halfling write "Spend this mana
    ||| ONLY to cast a creature spell of the chosen type, AND THAT SPELL
    ||| can't be countered" -- one sentence, one mention of the spell,
    ||| doing [CR#106.6]'s first and second things at once. Two riders
    ||| could not have said it: the rider list is a `List` and its
    ||| members announce nothing to one another, so a `SpendOnly` beside
    ||| an effect rider would leave the effect with no mention to read.
    ||| The other 9 write "IF that mana is spent on ..." and restrict
    ||| nothing.
    |||
    ||| The mention is on the STACK: [CR#601.2a] moves the card there
    ||| before [CR#601.2h] takes the payment, so whatever the mana pays
    ||| for is a spell by the time this rider can speak of it.
    ||| -- spelling: with `only`, "Spend this mana only to cast [what],
    ||| and [says]"; otherwise "If that mana is spent on [what], [says]"
    ||| at `AffectsIt` and "When that mana is spent to cast [what],
    ||| [says]" at `TriggersThen`.
    OnSpent : (mode : SpentMode) -> (only : Bool) ->
              (what : Noun bs Object) ->
              (says : Effect (nomIntro what)) ->
              {auto 0 zn : OnStack (nounZone what)} -> ManaRider bs

  public export
  data SpendPurposes : {0 bs : Bindings} -> List (SpendPurpose bs) -> Type where
    MkSpendPurposes : {0 p : SpendPurpose bs} -> {0 ps : List (SpendPurpose bs)} ->
                      SpendPurposes (p :: ps)

  ||| What a BUNDLE exception has to say to be one. A copy exception
  ||| naming a single characteristic already has a row -- `ExceptPt` for
  ||| the P/T, `ExceptColor` for the colour, `ExceptTypes` for the type
  ||| line -- so the bundle arm exists for the phrase those rows cannot
  ||| write: "a 4/4 black Zombie" is ONE noun phrase setting three
  ||| characteristics, and a list of three exceptions would spell three
  ||| sentences. Two stated characteristics is therefore the bar, and it
  ||| is a partition rather than a preference: no sentence has two
  ||| spellings.
  public export
  copyBundleSays : {0 bs : Bindings} -> TokenChars bs -> Bool
  copyBundleSays t =
    let said = (if ptWritten t.pt then 1 else 0) +
               (if someWritten t.colors then 1 else 0) +
               (if lineNonEmpty t.line then 1 else 0)
    in said >= 2

  public export
  CopyBundle : TokenChars bs -> Type
  CopyBundle {bs} t = So (copyBundleSays t)

  public export
  data CopyExcept : Bindings -> Type where
    ||| The TYPE-ONLY addition, and [CR#707.9d]'s own carve-out: that
    ||| rule stops a copy effect copying a characteristic-defining
    ||| ability for a characteristic it sets, and then exempts
    ||| "exceptions that state the object is a certain card type,
    ||| supertype, and/or subtype 'in addition to its other types'" --
    ||| in those cases the type-defining ability IS copied. So an added
    ||| type line is not a setting with a word on it; it is the other
    ||| operation, which is why this row carries no setting flag.
    ExceptTypes : (added : TypeLine) ->
                  {auto 0 ne : LineNonEmpty added} -> CopyExcept bs
    ||| "…, except its name is Sakashima the Impostor", "…, except his
    ||| name is Absorbing Man": the NAME exception. 16 supported lines
    ||| write it (re-measured 2026-09-02), always as its own clause and
    ||| never inside a bundle noun phrase.
    |||
    ||| A name is a characteristic [CR#109.3], [CR#707.9b] licenses the
    ||| operation -- "some copy effects modify a characteristic as part
    ||| of the copying process" -- and [CR#707.9d] names this shape of it,
    ||| a copy effect that "provides a specific set of values for a
    ||| certain characteristic". So the copy seat may set a name where the
    ||| ADDITION seat may not: `badNamedAddition`
    ||| pins "a Zombie named Bob in addition to its other types" as
    ||| unspellable, and [CR#205.1b] is why, adding types and leaving the
    ||| rest. Both stand together; a name is set here and added nowhere.
    ||| -- spelling: "except [its/his/her] name is [nm]".
    ExceptName : (nm : String) -> CopyExcept bs
    ||| "…, except it's a 4/4 black Zombie" (the Scarab God family),
    ||| "…, except it's a 3/3 Golem artifact creature in addition to its
    ||| other types": the CHARACTERISTICS BUNDLE. 35 supported lines
    ||| write a number inside a whole bundle (re-measured 2026-09-02).
    |||
    ||| ONE payload and not a list of three rows. The printed phrase is a
    ||| single noun phrase naming a P/T, a colour and a type line at
    ||| once; `[ExceptPt, ExceptColor, ExceptTypes]` would spell "except
    ||| it's 4/4, it's black and it's a Zombie in addition to its other
    ||| types", which is three sentences and not this one. The payload is
    ||| `SetsType`'s, the same phrase in the statement position.
    |||
    ||| `typesAdded` is carried and is not spelling: [CR#707.9d] makes it
    ||| a rules difference -- a bundle that SETS the types stops the
    ||| copied type-defining ability coming across, and one that adds
    ||| them "in addition to its other types" does not. Re-measured
    ||| 2026-09-02: 23 of the 35 set and 12 add, so the bundle is not
    ||| uniformly a setting and the flag cannot be defaulted away. The
    ||| P/T is a setting under either flag; no printed line writes a
    ||| power "in addition to" another.
    |||
    ||| It carries NO NAME under either flag, where `TokenChars` has the
    ||| slot: the name exception is its own clause in all 16 printed
    ||| lines, and letting one ride here would give the added case a
    ||| spelling `badNamedAddition` refuses at the statement seat.
    ||| -- spelling: "except it's [t]" / "except it's [t] in addition to
    ||| its other types".
    ExceptChars : (t : TokenChars bs) -> (typesAdded : Bool) ->
                  {auto 0 bd : CopyBundle t} ->
                  {auto 0 tc : TokenCanonical t} ->
                  {auto 0 ta : TokenAbilities t} ->
                  {auto 0 un : AdditionUnnamed t} -> CopyExcept bs
    ExceptAbility : (ab : AbilityAt []) ->
                    {auto 0 gr : Grantable ab} -> CopyExcept bs
    ExceptThisAbility : CopyExcept bs
    ExceptPt : (pow : Amount bs) -> (tou : Amount (amtIntro pow)) -> CopyExcept bs
    ExceptNonlegendary : CopyExcept bs
    ExceptColor : (c : Chroma.Color) -> CopyExcept bs
    ||| "…, except it enters with [amt] [kind] counters on it": the
    ||| entry-counter clause as a copy modification — the copy's carrier,
    ||| not `EntersWithCounters`', which is a printed static on the
    ||| entering object itself. Counters are not copiable values
    ||| [CR#707.2], so the clause has to ride the copy effect.
    ExceptEntersWithCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                               (mark : EntryCounterMark) -> CopyExcept bs

  public export
  data Repetition : Bindings -> Type where
    Again : Repetition bs
    MoreTimes : (n : Amount bs) ->
                Repetition bs
    AnyNumber : Repetition bs
    ||| "Repeat this process until [c]": the loop bounded by a TEST
    ||| rather than by a count. A condition stands where the count
    ||| stands, so the row sits beside `MoreTimes` rather than wrapping
    ||| it. The condition is read in the context BEFORE the repetition,
    ||| like every other `Repetition` slot -- [CR#608.2c] has the
    ||| instructions followed in the order written, and the process this
    ||| one repeats was written before it, so the loop states no body of
    ||| its own and names no context the earlier text had not named.
    ||| It exports nothing, for `AnyNumber`'s reason: a stopping test is
    ||| not a bound, so no determinate batch stands after the loop.
    ||| -- spelling: "repeat this process until [c]".
    Until : (c : Condition bs) -> Repetition bs
    ||| "Repeat this process except that [chooser] can't choose a
    ||| [thing] already chosen": the repeat with MEMORY of the earlier
    ||| passes' picks. Forgotten Lore and Shrouded Lore, 2 supported
    ||| lines over 2 cards (measured 2026-09-02), both "repeat this
    ||| process except that opponent can't choose a card already chosen
    ||| for [this card]".
    ||| Beside `Again` and not a rider on it. The exclusion is what makes
    ||| the loop terminate -- each pass takes one more card out of the
    ||| chooser's range, and a graveyard is finite -- so it is the
    ||| repetition's own bound, standing where `Until`'s condition and
    ||| `MoreTimes`' count stand.
    ||| It states neither the chooser nor the excluded description: both
    ||| are the repeated process's own, which [CR#608.2c] has already
    ||| been written before this clause is read, and no supported line
    ||| excludes anything but what the process itself chose. The printed
    ||| "already chosen FOR [name]" is the self-name [CR#201.5] -- text
    ||| naming the object it is on means that object -- and is
    ||| spelling.
    ||| -- spelling: "repeat this process except that [chooser] can't
    ||| choose a [thing] already chosen for [self]".
    AgainExcludingChosen : Repetition bs

  ||| One striation of a results table: the results it covers and the
  ||| effect they bring about. [CR#706.3a] gives the left column three
  ||| forms — a single number, a two-ended range "N1–N2", a one-ended
  ||| range "N+" — and each means "If the result was in this range,
  ||| [effect]", so the column is the quantity vocabulary's range and
  ||| needs nothing of its own. The gates are that vocabulary's own: a
  ||| range whose floor tops its ceiling covers no result, and a die is
  ||| numbered from 1 [CR#706.1a], so a row ceiling of zero covers none
  ||| either.
  public export
  data RollRow : Bindings -> Type where
    MkRollRow : (results : Quantity bs) -> (e : Effect bs) ->
                {auto 0 nz : NonZeroQ results} ->
                {auto 0 wf : WellFormedQ results} ->
                {auto 0 lt : So (quantLiteral results)} -> RollRow bs

  public export
  rowCount : {0 bs : Bindings} -> List (RollRow bs) -> Nat
  rowCount [] = Z
  rowCount (_ :: rs) = S (rowCount rs)

  ||| One type for verbs and frames together, where the crate splits them and
  ||| rejoins them through a wrapper. `docs/decisions/effect-atom-independence.md`
  ||| does not grade that count: it is scoped to ENGINE atoms, ruling that one
  ||| "depends only on its literal arguments and explicitly bound references"
  ||| and "does not infer meaning from its parent". The `Bindings` index is
  ||| that explicit-reference channel promoted to a type index, so a
  ||| context-reading constructor here is the ADR's sanctioned form rather
  ||| than an exception to it, and the single-type basis is settled by the
  ||| ADR's scope, not owed a boundary by it.
  |||
  ||| One obligation the ADR does own, at lowering. `Enact` and `Does`
  ||| LABEL a body with the keyword action it performs, and the atom
  ||| emitted has to be the BODY -- never an atom that reads its label to
  ||| learn what it does, which is the parent-inference the ADR forbids.
  ||| The open-label shape makes that free: the macro expands the keyword
  ||| action in full ([CR#701.9a] defines discarding AS the move), so the
  ||| label is data a lowering may drop and the wrapper buys nothing.
  public export
  data Effect : Bindings -> Type where
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) ->
                 {auto 0 pm : PerMember to} ->
                 {auto 0 rk : DamageRecipient to} -> Effect bs
    ||| "[src] deals damage equal to its [c] to [to]": `DealDamage` with
    ||| the amount the SOURCE'S OWN characteristic, written as a slot
    ||| rather than as an amount that names a possessor. [CR#120.1] makes
    ||| the object that deals damage the source of it, and the printed
    ||| "its" is that source -- the clause's own subject, which this row
    ||| supplies from its own earlier argument. So no pronoun is read
    ||| here and no candidate is counted: it is the forward binder
    ||| contract's clause 4, not clause 3.
    ||| Measured 2026-08-27: 275 supported faces write "deals damage …
    ||| equal to its [c]", every one of them source-bound, the 15 that
    ||| write the recipient first ("deals damage to itself equal to its
    ||| power") included -- the word order is spelling and the possessor
    ||| is the subject in all 275. 270 write power, 3 mana value, 2
    ||| toughness, so the characteristic is a slot and not a fixed word.
    ||| It does NOT subsume `DealDamage`: a written amount, an anaphoric
    ||| one and a characteristic of something OTHER than the source all
    ||| stay there, and this row is only the reading whose possessor the
    ||| construction already holds.
    ||| -- spelling: "[src] deals damage equal to its [c] to [to]".
    DealDamageOwn : {k : Kind} -> (src : Noun bs Object) -> (c : Characteristic) ->
                    (to : Noun (nomIntro src) k) ->
                    {auto 0 pm : PerMember to} ->
                    {auto 0 rk : DamageRecipient to} -> Effect bs
    Fights : (a : Noun bs Object) ->
             {auto 0 za : OnBattlefield (nounZone a)} ->
             {auto 0 ta : FightParticipant (nounTy a)} ->
             {auto 0 pa : nounPlur a = OneOf} ->
             (b : Noun (nomIntro a) Object) ->
             {auto 0 zb : OnBattlefield (nounZone b)} ->
             {auto 0 tb : FightParticipant (nounTy b)} ->
             {auto 0 pb : nounPlur b = OneOf} -> Effect bs
    SetStatus : {c : StatusCat} -> (v : StatusVal c) -> (n : Noun bs Object) ->
                {auto 0 ok : OnBattlefield (nounZone n)} ->
                {auto 0 at : StatusEffectVal v} -> Effect bs
    ||| "Transform this creature", "Transform Arlinn Kord", "Convert
    ||| target permanent": the permanent is turned over so that its
    ||| other face is up [CR#701.27a]. The whole act and the only
    ||| building block under it, which is why it is a row and not an
    ||| expansion the way [CR#701.9a]'s discard is -- and why ONE row
    ||| serves both printed words: [CR#701.28a] states convert by routing
    ||| it back through the transform rules in as many words, so the two
    ||| labels name one body.
    ||| NOT a status change. [CR#110.5] closes a permanent's status at
    ||| four categories of two values each, none of them a side of a card,
    ||| and [CR#701.27b] says outright that transforming a permanent and
    ||| turning one face up "are different game actions" that share only
    ||| the physical motion
    ||| -- so `SetStatus` is the wrong row here, and a fifth `StatusCat`
    ||| would have stated that difference away.
    ||| ON THE BATTLEFIELD, on [CR#701.27a]'s own word: it turns a
    ||| PERMANENT over, and [CR#712.9] restates the restriction over the
    ||| double-faced cards and tokens that can carry it out.
    ||| THREE REFUSALS LEFT TO THE ENGINE, and no term could state any
    ||| of them: [CR#701.27c] and [CR#712.9] ignore the instruction where
    ||| the permanent is represented by neither a double-faced token nor
    ||| a double-faced card, [CR#701.27d] and [CR#712.10] where the face
    ||| it would turn into is an instant or sorcery face, and
    ||| [CR#712.4c] where the card is a meld card. All three are facts
    ||| about the permanent the clause names, decided in play; the
    ||| printed line is a printed line either way, and each rule says
    ||| "nothing happens" rather than refusing the sentence.
    ||| 221 supported faces write the transform imperative and 23 the
    ||| convert one, reminder text stripped (measured 2026-08-28).
    ||| -- spelling: the label's own verb, imperative -- "[verb] [what]".
    TurnOver : (what : Noun bs Object) ->
               {auto 0 ok : OnBattlefield (nounZone what)} -> Effect bs
    RemoveFromCombat : (n : Noun bs Object) ->
                       {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    ||| "attach it to target creature you control", "attach this
    ||| Equipment to that creature", "attach any number of Equipment you
    ||| control to it": [CR#701.3]'s keyword action, written as an
    ||| instruction. 233 supported faces write the clause outside
    ||| reminder text (measured 2026-08-28); 297 further lines write it
    ||| inside reminder text, all of them the equip and reconfigure
    ||| expansions.
    ||| THE relation change and not a move: [CR#701.3a] has the act
    ||| "take it from where it currently is and put it onto that object
    ||| or player", and every zone the permanent could be in it stays
    ||| in, so nothing below this row composes it. That is why it is a
    ||| row rather than an `Enact` label over an expansion -- there is
    ||| no expansion to name -- and why no `verbFacts` row is bought
    ||| here: the label buys a stamp for a participial readback, and the
    ||| corpus reads an attachment back by DESCRIPTION ("an Equipment
    ||| that was attached to Zack Fair", `AttachedTo`) at every one of
    ||| its sites and by a stamp at none.
    ||| The host is written at every occurrence -- 0 of the 233 elide it
    ||| -- so it is an argument and not a slot, and it is kind-indexed
    ||| on the rule's own words ("onto that object or player"), which
    ||| the corpus writes at both seats ("attach this Aura to that
    ||| player", Curse of Leeches).
    ||| NO gate demands that the attached object be an Aura, Equipment
    ||| or Fortification. [CR#701.3b] answers a clause that names
    ||| something else -- "the effect does nothing and the first object
    ||| doesn't move" -- which makes it an instruction that accomplishes
    ||| nothing, never an unwritable sentence; and [CR#301.5f] and
    ||| [CR#303.4m] both read the relation off a permanent that "isn't
    ||| an Equipment"/"isn't an Aura". Nor does one demand that the HOST
    ||| be legal: [CR#701.3b] and [CR#303.4j] answer that too, and
    ||| [CR#702.6e] is a printed ability that attaches an Equipment to a
    ||| planeswalker "as though that planeswalker were a creature".
    ||| -- spelling: "attach [what] to [host]".
    AttachTo : {k : Kind} -> (what : Noun bs Object) ->
               {auto 0 zw : OnBattlefield (nounZone what)} ->
               (host : Noun (nomIntro what) k) ->
               {auto 0 hk : So (kindLte k (Object \/ Player))} -> Effect bs
    ||| "unattach it", "Unattach all Equipment from target creature",
    ||| "unattach enchanted Equipment": [CR#701.3d]'s act, which the
    ||| rules quote as a word of its own -- "to 'unattach' an Equipment
    ||| from a creature means to move it away from that creature so the
    ||| Equipment is on the battlefield but is not equipping anything".
    ||| Its own row and not `AttachTo` with the host left out. The two
    ||| are different words the corpus writes side by side in one
    ||| sentence (reconfigure's "Attach to target creature you control;
    ||| or unattach from a creature"), and an optional host would leave
    ||| a hole where this act's own meaning is complete: [CR#701.3d]
    ||| states an end position, not an attachment to nothing.
    ||| Measured 2026-08-28: 18 imperative occurrences over 18 supported
    ||| faces -- 9 as an effect and 9 as an activation cost, which
    ||| `costActionOk` admits on [CR#118.1]'s ground. The 4 "becomes
    ||| unattached from a permanent" lines beside them are the EVENT
    ||| [CR#701.3d] names in its last sentence and no part of this row.
    ||| ONE slot. 4 of the 9 effect lines write a "from [host]" phrase,
    ||| and what it does there is narrow WHICH attachment -- "all
    ||| Equipment from target creature" against every Equipment on the
    ||| battlefield -- which is the described object's own business and
    ||| `AttachedTo`'s phrase. A second slot would spell the same
    ||| relation twice with nothing to agree them, on `HappenedTo`'s
    ||| policy that the complement is one noun and any richer narrowing
    ||| is that noun's predicate.
    ||| -- spelling: "unattach [what]"; with the host described, "unattach
    ||| [what] from [host]".
    Unattach : (what : Noun bs Object) ->
               {auto 0 zw : OnBattlefield (nounZone what)} -> Effect bs
    ||| "[n] blocks [what]" as an instruction: the write that puts a
    ||| creature already on the battlefield into a blocking assignment.
    ||| [CR#506.3g] is the rule that knows the write -- it speaks of "a
    ||| resolving spell or ability [that] would cause a battle to become an
    ||| attacking or blocking creature" -- and [CR#509.1g] is what the
    ||| write makes true. It is not a requirement: "blocks if able" is a
    ||| deontic and leaves the declaration to its step, where this row
    ||| writes the assignment outright.
    ||| Against `RemoveFromCombat`, this is the second half of a reassign,
    ||| and [CR#509.3a] is what keeps the two readings apart. A creature
    ||| removed from combat and written back in "becomes a blocker as the
    ||| result of an effect" when it "wasn't a blocking creature at that
    ||| time", so a "whenever [it] blocks" ability triggers; a creature
    ||| that only `StopsBlocking` never stopped being one, so none does.
    ||| The blocked side is written, never left bare: [CR#509.1g] gives a
    ||| blocking creature the attacking creatures chosen for it, and a
    ||| creature put onto the battlefield blocking is the entry rider's
    ||| business [CR#506.3e].
    ||| -- spelling: "[n] blocks [what]"; after a removal, "[n] then
    ||| blocks [what]".
    BecomesBlocking : (n : Noun bs Object) ->
                      {auto 0 zn : OnBattlefield (nounZone n)} ->
                      {auto 0 dn : DeedParticipant ["Block"] Agent Object (nounTy n)} ->
                      (what : Noun (nomIntro n) Object) ->
                      {auto 0 zw : OnBattlefield (nounZone what)} ->
                      {auto 0 dw : DeedParticipant ["Block"] Patient Object (nounTy what)} ->
                      Effect bs
    ||| "[n] stops blocking [what]": one assignment unwritten and nothing
    ||| else. [CR#506.4] lists what removes a permanent from combat and a
    ||| dropped assignment is not among them; [CR#506.4a] settles the
    ||| neighbouring case the same way, an effect that would have kept a
    ||| creature from blocking not removing it once declared. So the
    ||| creature stays a blocking creature and stays blocking whatever else
    ||| it was blocking -- which is exactly what `RemoveFromCombat` does
    ||| not leave true, and why the two are separate rows.
    ||| The blocked side is written, never left bare: [CR#509.1g] leaves a
    ||| creature a blocking creature "until it's removed from combat or the
    ||| combat phase ends", so a bare "stops blocking" would name the
    ||| removal this row is defined against.
    ||| -- spelling: "[n] stops blocking [what]"
    StopsBlocking : (n : Noun bs Object) ->
                    {auto 0 zn : OnBattlefield (nounZone n)} ->
                    {auto 0 dn : DeedParticipant ["Block"] Agent Object (nounTy n)} ->
                    (what : Noun (nomIntro n) Object) ->
                    {auto 0 zw : OnBattlefield (nounZone what)} ->
                    {auto 0 dw : DeedParticipant ["Block"] Patient Object (nounTy what)} ->
                    Effect bs
    ||| "[n] is attacking [whom]": the attacking twin of
    ||| `BecomesBlocking`, for a permanent already on the battlefield.
    ||| `EntersAttacking` is the rider on entry and names no defender;
    ||| this row names one, through the same `AttackDefender` slot the
    ||| declaration event takes, so [CR#506.3]'s closed set and
    ||| [CR#508.1b]'s one-defender rule ride it unchanged.
    ||| It is an assignment, not a declaration: [CR#508.3a] keys attack
    ||| triggers on a creature being "declared as an attacker", which a
    ||| resolving effect never does.
    ||| -- spelling: "[n] is attacking [whom]"
    BecomesAttacking : (n : Noun bs Object) ->
                       {auto 0 zn : OnBattlefield (nounZone n)} ->
                       {auto 0 dn : DeedParticipant ["Attack"] Agent Object (nounTy n)} ->
                       (whom : AttackDefender (nomIntro n)) -> Effect bs
    Regenerate : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 Effect bs
    ||| "Destroy target creature. It can't be regenerated": the
    ||| effect-scoped rider, denying one resolving effect's own
    ||| consequence where the deontic carrier states a continuous
    ||| restriction on a permanent. The two constructions now read ONE
    ||| act vocabulary -- the deed labels -- and differ only in what they
    ||| are attached to; `deedRides` is which labels a rider may spell.
    CantBe : {k : Kind} -> (e : Effect bs) -> (deed : VerbLabel) ->
             (what : Noun (riderIntro e) k) ->
             {auto 0 kd : KnownDeed deed} ->
             {auto 0 rd : So (deedRidesOk deed)} ->
             {auto 0 kk : So (deedKindOk deed Patient k)} ->
             {auto 0 sub : DeedParticipant [deed] Patient k (nounTy what)} ->
             {auto 0 zn : ZoneFits (nounZone what) (deedZoneOf deed Patient)} ->
             Effect bs
    ||| The warrant tells the bare instruction from a keyword's expansion
    ||| body. A conferral may state how long it lasts: saddle's expansion
    ||| writes "until end of turn" [CR#702.171a] and ascend's and
    ||| storied's write "for the rest of the game"
    ||| [CR#702.131a,702.195a], while monstrosity and renown write none
    ||| [CR#701.37a,702.112a].
    GainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                       (w : GivingWarrant d) ->
                       (span : Maybe (Duration (nomIntro n))) ->
                       {auto 0 sc : designationScope d = HeldBy k} ->
                       {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
    GameBecomes : (d : Designation) ->
                  {auto 0 sc : designationScope d = HeldByGame} ->
                  {auto 0 at : So (designationGiven d)} -> Effect bs
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Effect bs
    GameDrawn : Effect bs
    ||| "Restart the game": [CR#727.1]'s procedure, one supported line
    ||| over one card (Karn Liberated, re-measured 2026-09-02). Minted at
    ||| its honest count because a procedure the rules give a section of
    ||| their own to is not a carrier: the restarted game "immediately
    ||| ends", no player wins, loses or draws it, and all its players
    ||| then start a new game by the start-of-game procedure that
    ||| begins at [CR#103.1].
    ||| Nullary. The one printed line's rider -- "leaving in exile all
    ||| non-Aura permanent cards exiled with Karn" -- is [CR#727.5]'s
    ||| exemption ("effects may exempt certain cards from the procedure
    ||| that restarts the game"), a second statement about which cards
    ||| the new game starts with rather than a slot on the restart; and
    ||| its following sentence is an ordinary `Move`, run at [CR#727.4]
    ||| where the restart finishes resolving.
    ||| -- spelling: "Restart the game".
    RestartsGame : Effect bs
    Choose : {k : Kind} -> (n : Noun bs k) ->
             (by : Maybe (Noun bs Player)) ->
             {auto 0 ch : ChoiceClause by n} -> Effect bs
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           (riders : MoveRiders (nomIntro what)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 pl : Placeable (nounTy what) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} -> Effect bs
    ||| "Counter target spell", "counter target activated ability",
    ||| "counter that spell or ability": [CR#701.6a] counters a spell or
    ||| an ability alike, so the subject is kind-indexed and `Counterable`
    ||| carries the per-kind demand -- the stack for a spell [CR#112.1],
    ||| no zone for an ability the kind places nowhere.
    CounterSpell : {k : Kind} -> (what : Noun bs k) ->
                   {auto 0 ct : Counterable what} -> Effect bs
    ||| "Copy target instant or sorcery spell", "Copy target triggered
    ||| ability you control", "copy that spell or ability twice":
    ||| [CR#707.10] copies a spell, an activated ability and a triggered
    ||| ability alike, so the complement is kind-indexed on
    ||| `CounterSpell`'s model and `Copiable` carries the per-kind demand.
    ||| The `Phrasal` is not a second gate but the payload's shape: what
    ||| the clause ANNOUNCES differs by kind -- an object copy is on the
    ||| stack, an ability copy is placeless -- and `copyPayload` is that
    ||| one difference written once.
    CopyStack : {k : Kind} -> (agent : Noun bs Player) ->
                (what : Noun (nomIntro agent) k) ->
                (times : Amount (nomIntro what)) ->
                (exc : List (CopyExcept (amtIntro times))) ->
                {auto ph : Phrasal k} ->
                {auto 0 cp : Copiable what} ->
                Effect bs
    ||| "Copy that card", "Copy the exiled card", "Copy that card three
    ||| times": [CR#707.12]'s verb, and a DIFFERENT rule and a different
    ||| row from the stack copy above. That rule copies an object "and
    ||| not just copy a spell": the copy "is created in the same zone the
    ||| object is in and then cast while another spell or ability is
    ||| resolving", where [CR#707.10] puts its copy on the stack.
    ||| 19 supported lines write the verb (re-measured 2026-09-02) and 39
    ||| write the "you may cast the copy" readback that follows it.
    |||
    ||| It SHARES the stack copy's mention word and nothing else, which
    ||| is why `CopyW` asks the origin alone: the copy clause is the only
    ||| thing that stamps `CopyOrigin`, and the zone the stamp appears in
    ||| is the copied object's, not the stack's.
    ||| The gate is `isCardZone` and not `OnStack`: [CR#707.12] is about
    ||| an object that is a card somewhere, which is exactly the set
    ||| `CardW` reads.
    ||| The cast permission wants NO new row -- the deontic carrier's
    ||| play rider already carries it, `PlayPayment.WithoutPaying` being
    ||| [CR#118.9]'s alternative cost at a permission -- so no second
    ||| without-paying rider is minted here. [CR#707.12a] settles the
    ||| plural's per-object choice within that permission.
    ||| -- spelling: "[who] cop(y|ies) [what][ [times] times]".
    CopyCard : (who : Noun bs Player) ->
               (what : Noun (nomIntro who) Object) ->
               (times : Amount (nomIntro what)) ->
               {auto 0 zn : So (isCardZone (nounZone what))} -> Effect bs
    ||| "You may choose new targets for the copy": [CR#707.10c]'s
    ||| permission, kind-indexed for `CopyStack`'s reason and under the
    ||| same gate -- what may be retargeted is what may be copied.
    ChooseNewTargets : {k : Kind} -> (what : Noun bs k) ->
                       {auto 0 cp : Copiable what} -> Effect bs
    ||| "The copy targets that token" (Frontline Heroism, [CR#707.10e]'s
    ||| own worked example), "The copy targets Ivy", "The copy targets
    ||| the chosen creature": the copy's SPECIFIED target. 6 supported
    ||| lines write it (re-measured 2026-09-02).
    |||
    ||| A statement whose subject is the copy mention and whose predicate
    ||| is a targeting relation no other row states. `AnyTarget` and the
    ||| target quantities describe a phrase's OWN targeting as it is
    ||| written, and `Targets` describes an object BY what it targets;
    ||| neither can say that one object's targets are set to another
    ||| after the fact, which is what [CR#707.10e] has an effect do --
    ||| "copy a spell or ability and specify a new target for the copy".
    ||| It is `Targets`' relation at the statement seat, on
    ||| `BlockerOf`/`BlockedBy`'s economy: one relation decided once and
    ||| spelled at both seats, with each seat's own gates.
    |||
    ||| The subject takes `Copiable` and not a copy-only gate: what may
    ||| be given a new target is what the copy clause put on the stack,
    ||| and the same kinds are at issue [CR#707.10]. The complement takes
    ||| `Targetable`, [CR#115.1]'s own set.
    ||| The permission [CR#707.10c] states is `ChooseNewTargets` and this
    ||| is not it: that one lets the copy's controller choose, this one
    ||| names the target outright and leaves no choice.
    ||| -- spelling: "[copy] targets [whom]".
    CopyTargets : {k : Kind} -> {kt : Kind} ->
                  (copy : Noun bs k) ->
                  (whom : Noun (nomIntro copy) kt) ->
                  {auto 0 cp : Copiable copy} ->
                  {auto 0 tk : Targetable kt} -> Effect bs
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    ||| "Exchange life totals with target opponent" (Magus of the Mirror,
    ||| Mirror Universe, Mister Negative), "Two target players exchange
    ||| life totals" (Axis of Mortality, Profane Transfusion, Soul
    ||| Conduit): the TWO-PARTICIPANT clause. 7 supported lines write it
    ||| (measured 2026-08-28).
    ||| The arithmetic is not what was missing: [CR#701.12c] settles each
    ||| side by having each player "gain or lose the amount of life
    ||| necessary to equal the other player's previous life total", which
    ||| is `LifeOp.Set` twice over, once per party, from values read
    ||| before either is written. What no row said is that ONE clause
    ||| binds two players symmetrically -- `ChangeLife` names one player
    ||| and writes one operation on them, and two of those would say
    ||| something else, since the second would read a life total the
    ||| first had already changed.
    ||| So the parties ride one slot and the operation rides none: the
    ||| row IS the exchange, and there is nothing left for an operand to
    ||| choose. [CR#701.12g]'s other exchange -- a life total against a
    ||| POWER or TOUGHNESS (Evra, Tree of Perdition, Tree of Redemption,
    ||| 3 lines) -- is a different pair of values, one of them an
    ||| object's, and is not this row.
    ||| It announces the pair, which is what "those players' life totals"
    ||| (Profane Transfusion) reads back.
    ||| -- spelling: with a coordination whose first arm is "you",
    ||| "exchange life totals with [r]"; otherwise "[parties] exchange
    ||| life totals".
    ExchangeLife : (parties : Noun bs Player) ->
                   {auto 0 tp : So (twoPartiesOk parties)} -> Effect bs
    AddMana : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
              (prod : ProducedMana (amtIntro amt)) ->
              (riders : List (ManaRider (amtIntro amt))) ->
              Effect bs
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           Effect bs
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Effect bs
    ||| "Search [scope] for [q] [description]" [CR#701.23a]. The count is
    ||| a slot because printed text writes it -- "up to two basic land
    ||| cards", "a number of Plains cards equal to the difference"
    ||| (Boreas Charger), "up to X basic land cards" (Celebrate the
    ||| Harvest) -- and a bare "a card" is `exactly 1`, which is what the
    ||| row spelled before the slot existed. It rides beside the
    ||| description rather than inside it for `CountedGroup`'s reason: a
    ||| description takes no count, and a group-level constraint ("with
    ||| different names") has to have a group to ride.
    ||| What the clause finds is announced at the count's own plurality,
    ||| so the sentence that says where the cards GO -- a separate clause,
    ||| since [CR#701.23e] makes even the reveal separate -- can name them
    ||| as "those cards", "them", "one of them" or "the rest".
    ||| -- spelling: "[who] search[es] [scope] for [q] [description]".
    Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) ->
             (q : Quantity (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} ->
             {auto 0 zf : ZoneFree p} -> Effect bs
    Shuffle : (whose : Noun bs Player) -> Effect bs
    ||| "Flip a coin", "Flip five coins", "target player flips a coin":
    ||| [CR#705.1]'s instruction and nothing more. The clause says a coin
    ||| is flipped; who calls it, which side lands up and who won are the
    ||| rule's business, and the two readings a later clause may take of
    ||| the flip are `FlipCalled` and `FlipFace`. The count is a slot
    ||| because printed text writes it ("Flip X coins"), and a bare flip
    ||| is `Lit 1`.
    ||| It introduces the flip so those readings have something to name.
    ||| The count slot is a `FlipScope`, so the same row writes the flip
    ||| made FOR each member of a described set ("flip a coin for each
    ||| creature that isn't a Demon, Devil, or Imp"): [CR#705.1] leaves a
    ||| coin owned by no referent and [CR#705.2] gives the flip to whoever
    ||| flips it, so the per-member arm changes how many coins there are
    ||| and never who the subject is.
    ||| -- spelling: "[who] flip[s] [count]"; with `You` in the subject
    ||| slot, the imperative "Flip a coin."
    FlipCoins : (who : Noun bs Player) -> (count : FlipScope (nomIntro who)) ->
                Effect bs
    ||| "Roll a d20", "Roll two six-sided dice", "roll that many dice":
    ||| [CR#706.1]'s instruction, which "will specify what kind of die to
    ||| roll and how many of those dice to roll" — so both are written
    ||| arguments and neither has a default. The kind is a `DieSides`,
    ||| which carries [CR#706.1a]'s positivity on its written arm and,
    ||| on its anaphoric one, the kind an announced roll already named —
    ||| what "instead roll that many dice plus one" writes over a roll it
    ||| replaces.
    ||| It introduces the roll's number, which "the result" reads
    ||| [CR#706.2] and a results table ranges over [CR#706.3a].
    ||| -- spelling: "[who] roll[s] [count] [sides]"
    RollDice : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
               (sides : DieSides (amtIntro count)) -> Effect bs
    ||| The results table [CR#706.3]: the striations that read the roll
    ||| the text has already made. A separate clause rather than a slot on
    ||| `RollDice`, because [CR#706.3b] binds the roll, "any additional
    ||| instructions based on the result of the roll, and the associated
    ||| results table" into one ability WITHOUT binding them into one
    ||| sentence — a modifier clause may stand between — and because
    ||| [CR#706.4] has rolls that carry no table at all. Its gate is the
    ||| roll's number, so a table with no roll before it is unwritable.
    ||| Only one striation happens [CR#706.3a], so like a mode list the
    ||| table introduces nothing for later text to read.
    ||| -- spelling: each row on its own line, "1—9 | [effect]"
    ResultsTable : (rows : List (RollRow bs)) ->
                   {auto 0 ne : IsSucc (rowCount rows)} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "and ignore the lower roll", "and ignore all but the highest
    ||| roll", "and ignore one": the instruction that sets aside some of
    ||| what the same clause has just randomised. [CR#706.6] gives it its
    ||| meaning outright -- an ignored roll "is considered to have never
    ||| happened", no ability triggers because of it and no effect
    ||| applies to it -- and settles the tie the superlative can leave.
    ||| Its own clause and not a slot on `RollDice`, on `ResultsTable`'s
    ||| ground: the roll and what is done with its results are one
    ||| ability [CR#706.3b] without being one node, and a roll may carry
    ||| no ignore at all.
    ||| It leaves what it ignores mentioned rather than reminting it: the
    ||| rolls that survive are the same roll the clause named, which is
    ||| what a following `ResultsTable` or `TheResult` reads.
    ||| Outcomes and not rolls alone. The word is printed over each of
    ||| the three randomisers this vocabulary has -- a roll (Berserker's
    ||| Frenzy), a coin (Krark's Thumb's "instead flip two coins and
    ||| ignore one") and the planar die (Ichor Elixir) -- and no rule
    ||| makes it meaningless on the two [CR#706.6] does not name. The gate
    ||| is asked per arm (`ignorableFor`), because the superlative arms
    ||| stay roll-shaped: `RollExtreme` names the ends of an order, and
    ||| neither a coin's two faces [CR#705.1] nor the planar die's
    ||| [CR#901.3a] are ordered.
    ||| -- spelling: "ignore the lowest/lower roll"; with `IgnoreAllBut`,
    ||| "ignore all but the highest roll"; with `IgnoreChosen`,
    ||| "ignore [n]" or "[who] choose(s) [n] of those rolls to ignore".
    IgnoreOutcomes : (which : IgnoredOutcomes bs) ->
                     {auto 0 ok : So (ignorableFor which)} -> Effect bs
    ||| "increase or decrease the result by 1": [CR#706.2]'s modifier,
    ||| the one the rule says may "come from other sources" rather than
    ||| riding the rolling instruction itself. The direction is not a
    ||| slot: every printed line writes the disjunction whole and leaves
    ||| the choice to whoever applies it, and no line writes a
    ||| one-directional shift, so a direction word here would name
    ||| nothing the corpus says. Gated on the roll like every other read
    ||| of it, and it mints nothing: [CR#706.2] makes the shifted number
    ||| the result of that same roll, which `TheResult` already names.
    ||| -- spelling: "increase or decrease the result by [amt]".
    ShiftResult : (amt : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "Roll the planar die": Planechase's own die as a written
    ||| instruction. Its own row and never a `RollDice` with a side
    ||| count, because [CR#901.3a] gives the planar die one Planeswalker
    ||| face, one chaos face and four blanks, which is not the die
    ||| [CR#706.1a] describes ("N equally likely outcomes, numbered from
    ||| 1 to N"). It announces no number for that reason: [CR#706.7] says
    ||| any effect referring to a numerical result of a die roll "ignores
    ||| the rolling of the planar die", so `TheResult`, `TheTotal` and
    ||| `ResultsTable` are inapplicable to it by rule and this row mints
    ||| nothing for them to read.
    ||| The count IS written, and is a slot for [CR#706.1]'s reason on
    ||| `RollDice`'s model: Ichor Elixir's "instead roll that many planar
    ||| dice plus one" writes it as an anaphor over the roll it replaces.
    ||| A bare "Roll the planar die." is `Lit 1`, as a bare flip is.
    ||| The count is the whole of what this row takes from [CR#706.1]:
    ||| there is no kind slot beside it, because [CR#901.3a]'s die is not
    ||| the numbered one [CR#706.1a] describes and naming it IS naming
    ||| the row.
    ||| It announces the roll as `PlanarRolled` and never as a result:
    ||| the mention carries no value (`outcomeIsQuantity` is False), so
    ||| `TheResult`, `TheTotal` and `ResultsTable` stay inapplicable by
    ||| [CR#706.7] while "ignore one" still has the planar dice to name.
    ||| The special action [CR#116.2i,901.9] is not this row. That one is
    ||| granted to a player by the Planechase rules and is written on no
    ||| card; this is the instruction a card's own ability gives.
    ||| -- spelling: "[who] roll[s] [count] planar dice", and with
    ||| `Lit 1` the singular "[who] roll[s] the planar die".
    RollPlanarDie : (who : Noun bs Player) ->
                    (count : Amount (nomIntro who)) -> Effect bs
    ||| "chaos ensues": the instruction beside the die face. [CR#311.7]
    ||| admits it outright — a chaos ability triggers "if the chaos symbol
    ||| is rolled on the planar die, if a resolving spell or ability says
    ||| that chaos ensues, or if a resolving spell or ability states that
    ||| chaos ensues for a particular object" — so a card may say it
    ||| without a planar roll anywhere in the sentence, which is how both
    ||| printed lines write it.
    ||| No subject and no slot. The rule's middle clause gives the
    ||| instruction no participant, and the object-scoped third clause is
    ||| written by no supported line. It mints nothing: what ensues is a
    ||| trigger on the plane card [CR#311.7], not a phrase this sentence
    ||| can go on to read.
    ||| -- spelling: "chaos ensues".
    ChaosEnsues : Effect bs
    ||| "and store those results on it": [CR#706.8a]'s noting of both the
    ||| kind of die rolled and the result of each roll. Gated on the roll
    ||| the same clause made, since those results are what it stores.
    ||| The holder is singular because a stored result is stored ON a
    ||| permanent and the rule notes it there.
    ||| -- spelling: "store those results on [on]".
    StoreResults : (on : Noun bs Object) ->
                   {auto 0 one : nounPlur on = OneOf} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "reroll any number of this creature's stored results":
    ||| [CR#706.8b]'s rerolling, which rolls one die of each noted kind
    ||| and stores the new results in the old ones' place. No gate on a
    ||| roll: the rolls it repeats are the ones already noted on the
    ||| holder, which is why the rule can define it without a roll in the
    ||| sentence, and [CR#706.8c] links this ability to the one that
    ||| stored them. It mints nothing either -- the new results are
    ||| stored, not left to read -- so `TheResult` finds nothing here.
    ||| -- spelling: "[who] reroll[s] [q] of [whose]'s stored results".
    RerollStored : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                   (whose : Noun (nomIntro who) Object) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 one : nounPlur whose = OneOf} -> Effect bs
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 sp : SpanOk (staticKind se) span} ->
                   {auto 0 cl : ClauseStatic se} -> Effect bs
    ||| The leading span, "During [span], [se]": the span is written first
    ||| and the statement reads the phrase it names -- "During TARGET
    ||| OPPONENT's next turn, creatures THAT PLAYER controls attack ... if
    ||| able". `Continuously` is the postposed twin, whose span sits at
    ||| `staticIntro se` and so announces into nothing; a span naming a
    ||| target announces it like any other phrase [CR#601.2c], and no
    ||| statement reads forward. Neither is a macro over the other, on the
    ||| model of `Conditionally`/`OnlyWhile`.
    ||| -- spelling: "During [span], [se]."
    Throughout : (span : Duration bs) ->
                 (se : StaticEffect (spanIntro span)) ->
                 {auto 0 sp : SpanOk (staticKind se) (Just span)} ->
                 {auto 0 cl : ClauseStatic se} -> Effect bs
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (spec : TokenSpec (amtIntro count)) -> (riders : List TokenRider) ->
             Effect bs
    GetsEmblem : (who : Noun bs Player) -> (abl : List (AbilityAt [])) ->
                 {auto 0 ea : EmblemAbilities abl} -> Effect bs
    ||| "Put [amt] [kind] counter(s) on [on]". The kind slot says how the
    ||| clause gives the kind -- the printed word, or a printed menu its
    ||| own "you" [CR#109.5] picks an arm of. Same verb and same gates
    ||| either way, so the menu widens the slot rather than doubling the
    ||| row; `PutCountersOfThoseKinds` is the contrast, where no kind is
    ||| given at all.
    ||| -- spelling: "put your choice of a [k1], [k2], or [k3] counter on
    ||| [on]" (Me, the Immortal).
    PutCounters : (amt : Amount bs) -> (kind : CounterKindSource bs) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 sc : CounterSourceScope kind Object} -> Effect bs
    Distribute : {k : Kind} -> (v : DividedVerb bs) ->
                 (amt : Amount (divIntro v)) ->
                 (among : Noun (amtIntro amt) k) ->
                 {auto 0 gm : GroupMention among} ->
                 {auto 0 tk : DividedTakes (divTag v) among} -> Effect bs
    ||| "Remove a +1/+1 counter from this creature", "Remove any number
    ||| of storage counters from this land" (the eleven storage lands),
    ||| "Remove up to X counters from target permanent" (Hex Parasite).
    |||
    ||| The count is a `Quantity` and not an `Amount` because a removal
    ||| is the one counter verb whose size the ACTOR chooses: 19
    ||| supported lines write "any number of" and 6 write "up to"
    ||| (measured 2026-08-28), and neither is a value read off the game.
    ||| A `Range` says both and says "a counter" too; no put line asks
    ||| for either, so `PutCounters` keeps its amount.
    |||
    ||| The count is OPTIONAL, and the silence spells "remove ALL
    ||| counters" (Vampire Hexmage) -- 50 supported lines, 12 of them
    ||| kind-blind (re-measured 2026-09-02; the routing ticket carried
    ||| 25). "All" is not a `Quantity` arm and could not become one:
    ||| `SliceCount`'s note says why -- a `Quantity` states a count the
    ||| text wrote, and `quantExact`/`quantWellFormed`/`NonZeroQ` are all
    ||| built on that, at every other `Quantity` position too. The split
    ||| is made HERE instead, in the shape this row's own player cell
    ||| already uses: `LosesCounters` takes a `Maybe Amount` where the
    ||| silence spells "all", and this takes a `Maybe Quantity` for the
    ||| same reason. `SliceCount`'s named-arm split is the alternative
    ||| and buys nothing at a position with one universal and no
    ||| `NonZeroQ` demand to carry.
    |||
    ||| It announces how many it took, which is what the storage
    ||| counters read back (`RemovedThisWay`).
    ||| -- spelling: "remove [q] [kind] counter(s) from [from]";
    ||| "remove all [kind] counters from [from]" at the silence.
    RemoveCounters : (q : Maybe (Quantity bs)) -> (kind : Maybe CounterKind) ->
                     (from : Noun (optQuantIntro q) Object) ->
                     {auto 0 wf : OptWellFormedQ q} ->
                     {auto 0 kn : CounterKindNamed Object kind} ->
                     {auto 0 cm : CounterMemory from} -> Effect bs
    ||| "Remove three counters from among creatures you control"
    ||| (Tayam, Luminous Enigma): the counter PARTITIVE. The count is on
    ||| the COUNTERS and the source is a described GROUP, so the removal
    ||| is one act over the group and the actor decides how it falls
    ||| across the members [CR#608.2d] -- where `RemoveCounters`' source
    ||| names the holder and `Each` there distributes per member.
    |||
    ||| Its own row and not a widening of `RemoveCounters`' source slot.
    ||| The two write different prepositions ("from" against "from
    ||| among") and the pooling is the whole of the difference, so one
    ||| shared slot would carry a gate saying which preposition each
    ||| reading spells -- the same fact one indirection further from the
    ||| row, which is the reason `LosesCounters` is kept apart too.
    ||| The group takes `PartitiveBase` for `SomeOf`'s reason:
    ||| [CR#608.2d] has the choice announced while the effect is
    ||| applied, so the members are the ones that answer the description
    ||| then. It takes `CounterMemory` for `RemoveCounters`'
    ||| [CR#122.2,400.7], and the count stays a `Quantity` because the
    ||| family writes "one or more" (Ooze Flux, Jetfire), "any number
    ||| of" (Galloping Lizrog, Iron Spider, Eventide's Shadow), "up to
    ||| three" (Sensational Spider-Man) and plain numbers alike.
    ||| Measured 2026-09-02: 18 supported cards write the partitive, 17
    ||| of them this removal -- 11 in an activation or alternative cost
    ||| (Tayam, Tekuthal, The Filigree Sylex, Retribution of the
    ||| Ancients, Hopeful Initiate, Novijen Sages, Dawnhand Dissident,
    ||| Ooze Flux, Jetfire, Light Up the Night, Quilled Greatwurm) and 6
    ||| in an effect. The 18th is the MOVE partitive (Slippery
    ||| Bogbonder, "move any number of counters from among creatures you
    ||| control onto that creature"), which `MoveCounters`' source does
    ||| not yet reach and which is ledgered, not built, at one line.
    |||
    ||| It announces how many it took, as `RemoveCounters` does: Ooze
    ||| Flux and Iron Spider read `RemovedThisWay` back.
    ||| -- spelling: "remove [q] [kind] counter(s) from among [grp]".
    RemoveCountersAmong : (q : Quantity bs) -> (kind : Maybe CounterKind) ->
                          (among : Noun (quantIntro q) Object) ->
                          {auto 0 wf : WellFormedQ q} ->
                          {auto 0 kn : CounterKindNamed Object kind} ->
                          {auto 0 cm : CounterMemory among} ->
                          {auto 0 pb : PartitiveBase among} -> Effect bs
    ||| "Move [amt] [kind] counter(s) from [src] onto [dst]": the
    ||| two-holder transfer verb [CR#122.5], which is a remove and a put
    ||| taken together. The kind is a `Maybe`, as everywhere: the slot
    ||| says whether the sentence NAMES a kind. The source carries
    ||| `CounterMemory` (a referent that moved zones has no counters left
    ||| to give [CR#122.2,400.7]) and the destination distributes like a
    ||| put, under `MoveDestination` — [CR#122.5] lists the same-object
    ||| case among the ones that make a move impossible, and a bare
    ||| readback at `dst` is that case written down. Its other listed
    ||| cases are engine questions about a particular game state, so no
    ||| zone gate: the rule bounds the move by whether each half can
    ||| happen, never by fixing one zone this row could name.
    ||| -- spelling: "move [amt] [kind] counter(s) from [src] onto [dst]".
    MoveCounters : (amt : Amount bs) -> (kind : Maybe CounterKind) ->
                   (src : Noun (amtIntro amt) Object) ->
                   (dst : Noun (nomIntro src) Object) ->
                   {auto 0 kn : CounterKindNamed Object kind} ->
                   {auto 0 cm : CounterMemory src} ->
                   {auto 0 md : MoveDestination dst} ->
                   {auto 0 pm : PerMember dst} -> Effect bs
    ||| The counter COPY-read: as many counters of each kind as [src]
    ||| holds are put on [dst], and [src] keeps its own. [CR#122.8] and
    ||| [CR#122.9] write this operation out in rules text and say in so
    ||| many words that it is NOT a move [CR#122.5] -- "the player puts
    ||| the same number of each kind of counter the first object had onto
    ||| the second object". That is the whole gate story: [CR#122.5]'s
    ||| impossibility list bounds a move, so `src` carries no
    ||| `CounterMemory` (the rule is written FOR a source that has left
    ||| the battlefield, and `CountersOn` already reads a dead referent's
    ||| counters) and `dst` no `MoveDestination`. Kind-blind and
    ||| amount-fixed: what `src` holds is both the kinds and the counts,
    ||| so neither is a slot. A plural `src` is tolerated -- nothing in
    ||| [CR#122.1] makes summing two holders' counters meaningless -- and
    ||| nothing writes one.
    ||| -- spelling: "put its counters on [dst]"; where an intervening
    ||| "if" has already named the source, "put the same number of each
    ||| kind of counter on [dst]" (Denry Klin) -- one node, two spellings.
    PutSameCounters : (src : Noun bs Object) ->
                      (dst : Noun (nomIntro src) Object) ->
                      {auto 0 pm : PerMember dst} -> Effect bs
    ||| The distributive counter-kind anaphor: the replacement body that
    ||| puts a derived number of counters OF THE ANNOUNCED BATCH'S KINDS on
    ||| its recipient. Presupposes exactly one announced batch, the same
    ||| way `PreventedThisWay` presupposes its outcome.
    ||| -- spelling: "that many plus one of each of those kinds of
    ||| counters are put on [dst]" (Pir, Doc Samson); with no per-kind
    ||| wording, "twice that many of those counters" (Doubling Season) —
    ||| one node, two spellings, since both distribute per kind.
    PutCountersOfThoseKinds : (amt : Amount bs) ->
                              (on : Noun (amtIntro amt) Object) ->
                              {auto 0 pm : PerMember on} ->
                              {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                              Effect bs
    ||| The player cell of the put verb, and NOT `PutCounters` at a kind
    ||| index. The two rows bind their slots in opposite order -- "put
    ||| [amt] counters on [on]" announces the amount before the
    ||| recipient, "[who] gets [amt] counters" announces the recipient
    ||| first -- and a constructor has one slot order. The kind-blind
    ||| rows below ARE kind-indexed (`GiveCountersOfOwnKinds`,
    ||| `DoubleCountersOfOwnKinds`), which is what the shared shape buys
    ||| where there is one: those carry no count at all.
    GetsCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                   (kind : CounterKind) ->
                   {auto 0 sc : counterScope kind = Player} -> Effect bs
    ||| `PutCountersOfThoseKinds`' player-side twin: the same kind-blind
    ||| distributive written under the player's own verb. [CR#122.1]
    ||| places a counter on an object OR a player, so a batch a player
    ||| would get has kinds to range over exactly as an object's does,
    ||| and the same single-announcement presupposition holds.
    ||| -- spelling: "you get that many plus one of each of those kinds
    ||| of counters instead" (Winding Constrictor).
    GetsCountersOfThoseKinds : (who : Noun bs Player) ->
                               (amt : Amount (nomIntro who)) ->
                               {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                               Effect bs
    ||| The SELF-reading kind-blind distributive: each recipient is given
    ||| one more counter of every kind it already carries. [CR#701.34a]
    ||| is the rule -- proliferate gives each chosen permanent or player
    ||| "one additional counter of each kind that permanent or player
    ||| already has" -- and it is what tells the kind-blind rows apart:
    ||| `PutCountersOfThoseKinds` and `GetsCountersOfThoseKinds` range
    ||| over an ANNOUNCED batch's kinds and presuppose one,
    ||| `PutSameCounters` reads a DIFFERENT holder's counters, and this
    ||| row reads its own recipient's with nothing announced anywhere.
    ||| Amount-fixed, on `PutSameCounters`' model: the rule gives one per
    ||| kind, and no printed line writes another number -- "proliferate
    ||| twice" and "proliferate X times" repeat the whole action, which
    ||| is `Repeated`.
    ||| Kind-indexed at [CR#122.1]'s own pair, a marker placed on an
    ||| object or a player, because the choice names both halves at once
    ||| and the giving clause distributes over the union. An ability on
    ||| the stack is an object too [CR#109.1] and is excluded anyway:
    ||| every counter kind `counterScope` names sits on a permanent, a
    ||| card or a player, and no printed line counters an ability.
    ||| A recipient holding no counters is given nothing, which is the
    ||| rule applied rather than a defect -- [CR#701.34a] asks for a
    ||| counter in the CHOICE, so no gate repeats the demand here.
    ||| -- spelling: "give each one additional counter of each kind that
    ||| [n] already has"; the reminder text's "give each another counter
    ||| of each kind already there" is the same node.
    GiveCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                             {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                             {auto 0 pm : PerMember on} -> Effect bs
    ||| The MULTIPLICATIVE twin of the self-reading distributive: every
    ||| kind of counter the recipient already carries is doubled. It is
    ||| the same self-reading -- the recipient's own counters are both
    ||| the kinds and the counts, with nothing announced anywhere -- and
    ||| it is NOT [CR#701.34a]'s giving: proliferate adds ONE per kind
    ||| whatever the holder has, and this adds however many the holder
    ||| already had. That is why it is a row beside
    ||| `GiveCountersOfOwnKinds` rather than an operation slot on it: the
    ||| two differ in the arithmetic, not in a direction, and the
    ||| REMOVING direction the slot would also have to serve is at a
    ||| measured zero over the supported corpus.
    ||| Amount-fixed for `GiveCountersOfOwnKinds`' reason, measured
    ||| rather than assumed: doubling is the only multiplier any
    ||| supported line writes, and a written count outside the operation
    ||| ("do this twice") is `Repeated` over the whole action.
    ||| Kind-indexed at [CR#122.1]'s own pair, a marker placed on an
    ||| object or a player, because the player seat is printed
    ||| ("double the number of each kind of counter you have", Aetheric
    ||| Amplifier) beside the permanent one.
    ||| A recipient holding no counters is doubled to nothing, which is
    ||| the arithmetic applied and not a defect, so no gate demands a
    ||| counter.
    ||| -- spelling: "double the number of each kind of counter on [on]";
    ||| at the player seat, "double the number of each kind of counter
    ||| you have".
    DoubleCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                               {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                               {auto 0 pm : PerMember on} -> Effect bs
    ||| Counters leave a player in a stated number as well as all at
    ||| once: [CR#728.1]'s own rules text has a player remove "one rad
    ||| counter from themselves", and printed removal lines count what
    ||| they take off an opponent. The amount is therefore a slot, and
    ||| leaving it unwritten is the "all" spelling rather than the only
    ||| reading available.
    ||| `RemoveCounters`' player cell, kept apart for `GetsCounters`'
    ||| reason and one more: the count slots are three different types
    ||| across the four verbs, each separately measured. A removal's size
    ||| is the actor's choice, so it is a `Quantity` (19 "any number of",
    ||| 6 "up to"); a player's loss writes a number or nothing, so it is
    ||| a `Maybe Amount` where the silence spells "all"; both put verbs
    ||| write a plain `Amount`. A merged row would carry a three-armed
    ||| count and a table saying which verb admits which arm, which is
    ||| the same three facts one indirection further from the row.
    LosesCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                    (amt : Maybe (Amount (nomIntro who))) ->
                    {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
    ||| The keyword-action carrier: an action, plus the label naming
    ||| which keyword action performing it amounts to. The label rides
    ||| the INNERMOST action rather than the whole expansion --
    ||| [CR#701.9a] defines the discard as the move, and choosing which
    ||| card is the surrounding instruction's business [CR#701.9b] -- so
    ||| an iterated batch is n labeled actions, which is what makes it
    ||| one event with n occurrences [CR#603.2c], and what a
    ||| would-replacement or an "at random" attaches to.
    ||| No gate relates label to body: the body IS the meaning and the
    ||| label only names it, so a mislabel is a spelling defect and not a
    ||| rules impossibility. The gates a keyword action really imposes
    ||| ride its macro, where the expansion is built.
    |||
    ||| **The macro layer is the only sanctioned way to reach `Enact` and
    ||| `Does`.** Keyword actions stack on top of the core rules without
    ||| disturbing them: the label exists because the game rules must
    ||| OBSERVE that a specific keyword action took place -- triggers and
    ||| replacements watch it -- while every keyword action is a composite
    ||| of pre-existing building blocks. So the label is the observability
    ||| hook, the macro is the sanctioned constructor, and the body is the
    ||| meaning. This is authoring policy, recorded here, not a compiler
    ||| gate: the carrier would be private if Idris could hide one
    ||| constructor of a `public export` type, and it cannot. A raw
    ||| `Enact`/`Does` outside `Macros` therefore still typechecks -- and
    ||| whatever it says its body already meant, spelled under a label no
    ||| macro imposed the verb's gates on. Pins do write them raw, on
    ||| purpose: a pin's business is the term the bench must not have.
    ||| -- spelling: the keyword action's own verb, imperative.
    Enact : (v : VerbLabel) -> (e : Effect bs) ->
            {auto 0 kn : KnownVerb v} -> Effect bs
    ||| `Enact`'s agentive surface: the same labeled action with the
    ||| player performing it written as its subject. Macro-only, on
    ||| `Enact`'s policy and for its reasons.
    ||| -- spelling: "[subj] [verb]s [body]".
    ||| The body is typed in `agentIntro`, not `nomIntro`: a
    ||| DISTRIBUTIVE subject hands its clause one member to read back
    ||| ("Each opponent sacrifices a creature with the greatest power
    ||| among creatures that player controls"), and every other subject
    ||| hands it the same prefix `nomIntro` always did.
    Does : (subj : Noun bs Player) -> (v : VerbLabel) ->
           (e : Effect (agentIntro subj)) ->
           {auto 0 kn : KnownVerb v} -> Effect bs
    ||| "[n]'s controller sacrifices it": the sacrifice written with its
    ||| agent, where the agent is DERIVED from the patient and the
    ||| printed pronoun is this row's own single noun slot. [CR#701.21a]
    ||| makes that derivation the rule's -- "to sacrifice a permanent,
    ||| ITS CONTROLLER moves it from the battlefield directly to its
    ||| owner's graveyard" -- so the possessive subject and the object
    ||| are one referent by the act's own definition, and neither the
    ||| possessive nor the pronoun is a read.
    ||| That is the whole of what this row adds over `Does (ControllerOf
    ||| n) "Sacrifice" (Move …)`, which has to write a second mention in
    ||| the object slot and so gates a pronoun that refuses wherever the
    ||| enclosing ability announced a permanent of its own (Basalt
    ||| Golem's blocked-by trigger, Goblin Ski Patrol's pump). Measured
    ||| 2026-08-27: 16 supported faces write "[possessor]'s controller
    ||| sacrifices it", and in all 16 the pronoun is the possessor.
    ||| It announces the controller, because the lines that follow read
    ||| that player back ("… sacrifices it. That player may search …",
    ||| Arcum Dagsson), and it stamps and re-zones the patient exactly as
    ||| the labeled move does.
    ||| -- spelling: "[n]'s controller sacrifices it".
    ControllerSacrifices : (n : Noun bs Object) ->
                           {auto 0 one : nounPlur n = OneOf} ->
                           {auto 0 zn : OnBattlefield (nounZone n)} -> Effect bs
    ||| "[who] pay[s] [c]", with the number of times the one offer may
    ||| be taken [CR#702.56a]. At `PaidOnce` the clause is what it always
    ||| was; at the two repeating values the payment leaves a COUNT for
    ||| the clause after it ("put that many +1/+1 counters on this
    ||| creature", the five Adversaries).
    ||| -- spelling: the payer, the verb and the cost, with `PayTimes`'
    ||| own words after it.
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          (times : PayTimes) ->
          {auto 0 pb : Payable c} ->
          {auto 0 ag : PayAgrees who c} -> Effect bs
    May : (offer : Maybe (Noun bs Player)) -> (body : Effect (mayCtx offer)) ->
          (ifDid : Maybe (Effect (effIntro body))) ->
          (ifNot : Maybe (Effect (mayCtx offer))) -> Effect bs
    ||| The postposed conditional, "[e] if [c]" / "[e] unless [c]": the
    ||| condition is written after the clause and reads what the clause has
    ||| announced — "Counter target spell if it's red" — and is checked as
    ||| the clause resolves, before the deed, so it sits at `preIntro e`.
    ||| Not a macro over `If`: the two introduce mentions in different
    ||| places, and oracle text reads backward only.
    |||
    ||| The `otherwise` arm reads what the CONDITION announced as well as
    ||| what the then-branch did, which is the same answer the leading
    ||| orientation gives: [CR#601.2c] announces a target at casting
    ||| whichever arm goes on to run, so an arm's scope cannot depend on
    ||| which side of the clause the condition was written. `If` gets it
    ||| structurally — its consequent is already typed at `condIntro c`, so
    ||| `otherwiseCtx` carries `condDelta c` through — and this row has to
    ||| write the same term explicitly, because its consequent is typed at
    ||| `bs`. The two therefore over-generate alike: a failed comparison's
    ||| margin stands in an arm that ran because the comparison failed,
    ||| tolerated and recorded at both orientations rather than at one.
    ||| -- spelling: "[e] if [c]"; under `NotCond`, "[e] unless [c]".
    OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) ->
             (otherwise : Maybe (Effect (condDelta c ++ otherwiseCtx e))) ->
             Effect bs
    ||| The leading conditional, "If [c], [e]. Otherwise, [o].": the
    ||| consequent reads what the condition introduced (`condDelta`) — a
    ||| comparison's margin, a phrase a comparison named. The otherwise
    ||| arm reads `otherwiseCtx e`.
    ||| -- spelling: "If [c], [e]."; with the arm, "… Otherwise, [o]."
    If : (c : Condition bs) -> (e : Effect (condIntro c)) ->
         (otherwise : Maybe (Effect (otherwiseCtx e))) -> Effect bs
    ||| "[Do something] unless [a player does something else]" [CR#118.12a],
    ||| carried for the cost arm: the offer is written after the effect, so
    ||| the payer reads what the effect has already named. A conditional
    ||| "unless" is `If e (NotCond c) Nothing` instead.
    Unless : (e : Effect bs) -> (who : Noun (preIntro e) Player) ->
             (c : Cost (nomIntro who)) ->
             {auto 0 pb : Payable c} ->
             {auto 0 ag : PayAgrees who c} -> Effect bs
    ||| "..., where [l] is [amt]": the definition of a letter an earlier
    ||| clause brought in by use. [CR#107.3c] fixes a text-defined X's
    ||| value as the ability resolves and has its controller choose
    ||| nothing, so the step presupposes an open X to define -- the gate --
    ||| and settles every open instance at once [CR#107.3i]. A second
    ||| definition finds none open and is refused by the same gate.
    ||| `Static.Define` writes the same construction in a static clause.
    ||| -- spelling: ", where X is [amt]" attached to the clause before it,
    ||| never "then"; a `Sequentially` whose member is a `Define` spells no
    ||| "then" at that seam.
    Define : (l : Letter) -> (amt : Amount bs) ->
             {auto 0 ok : So (anyOpenLetter l bs)} -> Effect bs
    ||| "For each [group], [body]": the anaphoric per-member loop,
    ||| [CR#608.2f]'s construction -- "some spells and abilities include
    ||| actions taken on multiple players and/or objects", processed
    ||| "considering each [player or object] individually".
    |||
    ||| The group's KIND indexes the row. It was typed at `Object` on
    ||| that sort's own evidence; the PLAYER sort is 53 supported
    ||| sentences over 50 cards, 32 of which read the member back as
    ||| "that player" (re-measured 2026-09-02), Blatant Thievery's "for
    ||| each opponent, gain control of target permanent that player
    ||| controls" among them -- the rule's own first worked example. The
    ||| joined sort is 2 cards, Kaboom! and Soulfire Eruption. All three
    ||| are ONE construction reaching one member at a time, and the
    ||| generalisation is over the element row's index rather than a
    ||| second marked row beside the object one, which is what
    ||| `elemPayload` mints.
    ||| The WIDER "for each [fresh description]" surface stays out: 237
    ||| supported sentences over 220 cards front those words, and most of
    ||| them are the ordinary multiplier `CountOf` already spells. What
    ||| this row admits is the read-back subset.
    ||| -- spelling: "For each [grp], [body]", the member read back by
    ||| its own kind's demonstrative.
    ForEachOf : {k : Kind} -> {auto ph : Phrasal k} ->
                (grp : Noun bs k) ->
                (body : Effect (elemIntro grp)) ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                Effect bs
    ||| "For each color among permanents you control, add one mana of
    ||| that color": the DISTRIBUTIVE twin of `DistinctCount`. One pass
    ||| per distinct value the axis takes over the domain, where
    ||| `ForEachOf` runs one pass per MEMBER -- the readings come apart
    ||| for `DistinctCount`'s reason, that one object carries several
    ||| card types [CR#205.2b] and several colours [CR#105.2].
    ||| NOT `Repeated (DistinctCount ax dom)`: every printed line of this
    ||| shape reads the value back ("of that color", "of that type",
    ||| "with that power"), and a counted iteration exports the count and
    ||| the body's own delta, never a value on the axis. So the pass
    ||| BINDS the value, and binds it as a quality, which is the sort the
    ||| existing chosen-quality reads already take: no new read is minted
    ||| for "that color".
    ||| The sort rides beside the axis and is gated to agree with it, as
    ||| `Aggregate` carries its kind beside its `ProjAxis`. An axis whose
    ||| values `QualitySort` does not yet name has no spelling here; see
    ||| `kindAxisSort`.
    ||| UNGATED on plurality, as `DistinctCount` is: a single object
    ||| still takes several values on these axes.
    ||| The domain is OPTIONAL. Written, the pass runs over the values
    ||| its group supplies; left out, it runs over the axis's whole value
    ||| set ("For each color, return up to one target card of that color
    ||| from your graveyard to your hand"), which only a set the rules
    ||| CLOSE can supply -- `kindDomainOk`.
    ||| -- spelling: "for each [axis] among [domain], [body]", or
    ||| "for each [axis], [body]" with no domain; the SCALING reading of
    ||| the same words ("draw a card for each color among permanents you
    ||| control") is no iteration at all and is `Times` over
    ||| `DistinctCount`.
    ForEachKindOf : (ax : KindAxis) -> (dom : Maybe (Noun bs Object)) ->
                    (q : QualitySort) ->
                    {auto 0 sc : kindAxisSort ax = Just q} ->
                    {auto 0 cl : So (kindDomainOk ax dom)} ->
                    (body : Effect (kindValueIntro q dom)) ->
                    Effect bs
    Repeat : (rep : Repetition bs) -> Effect bs
    ||| "[body] [n] times": the counted iteration, and the canonical
    ||| expansion of every counted keyword action -- "discard three
    ||| cards" is three passes of choose-one-and-discard-it. The iterated
    ||| singular is what makes a shortfall structural: each pass's choice
    ||| finds what it finds, which is [CR#609.3]'s do-as-much-as-possible
    ||| written into the term instead of asserted beside it. A batch
    ||| spelling belongs to the cards whose printed English carries the
    ||| cardinality itself ("choose two colors").
    ||| What it exports is the body's OWN delta pluralized -- one summary
    ||| mention per mention the body introduced, same payload and same
    ||| stamp -- so "the discarded cards" and "that many" read the whole
    ||| batch, beside the count the text wrote, which a shortfall can
    ||| leave larger than the batch [CR#609.3].
    ||| -- spelling: the body's, with the count written on it.
    Repeated : (n : Amount bs) -> (body : Effect (amtIntro n)) -> Effect bs
    ||| A coordination of one member denotes that member, so only the
    ||| empty list is refused: it instructs nothing.
    Sequentially : {0 n : Nat} -> Effects n bs ->
                   {auto 0 ne : IsSucc n} -> Effect bs
    Simultaneously : {0 n : Nat} -> SimEffects n bs ->
                     {auto 0 ne : IsSucc n} -> Effect bs
    Modal : (q : Quantity bs) -> (modes : List (Effect bs)) ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit q (modeCount modes)} ->
            {auto 0 dm : So (distinctModes modes)} -> Effect bs
    Delayed : (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (span : Maybe (Duration bs)) ->
              Effect (delayedCtx alts ev) ->
              {auto 0 so : DelaySpanOk span} -> Effect bs
    InsteadOf : (replaced : Effect bs) -> (repl : Effect (replacedCtx replaced)) ->
                {auto 0 na : NotInstead replaced} ->
                {auto 0 nb : NotInstead repl} -> Effect bs
    HeldUntil : (e : Effect bs) -> (ev : GameEvent (annIntro e)) ->
                {auto 0 ok : HeldClause e} -> Effect bs
    Reflexively : (body : Effect bs) -> (trig : Effect (reflexCtx body)) ->
                  {auto 0 en : ReflexEnclosure body} -> Effect bs
    ||| [CR#603.12]'s other trigger template, "when [something happens]
    ||| this way": the trigger names the EVENT the enclosure caused
    ||| rather than the player who acted, so it reads the enclosure's
    ||| outcome mentions. It is checked at once when the enclosure caused
    ||| the event as it resolved [CR#603.12]; when the enclosure's effect
    ||| is a replacement that applies later, as a regeneration shield
    ||| does [CR#701.19a], it waits as a delayed trigger [CR#603.7].
    ThisWay : (body : Effect bs) -> (ev : GameEvent (effIntro body)) ->
              (trig : Effect (thisWayCtx body ev)) ->
              {auto 0 oc : ThisWayOutcome body} -> Effect bs

    DoesntUntapNext : (n : Noun bs Object) -> (steps : Amount bs) ->
                      {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    SkipsNext : (who : Noun bs Player) -> (part : TurnPart) ->
                (count : Amount bs) -> Effect bs
    ||| "[who] skips all [part]s of their next turn": the QUANTIFIED
    ||| part, beside `SkipsNext`'s counted one. Empty City Ruse and
    ||| False Peace, 2 supported lines over 2 cards (measured
    ||| 2026-09-02), both writing "skips all combat phases of their next
    ||| turn".
    |||
    ||| A second row and not a count on `SkipsNext`. [CR#500.1] runs one
    ||| of each phase and step on every turn, so "their next combat
    ||| phase" and "all combat phases of their next turn" name the same
    ||| thing until [CR#500.8] or [CR#500.9] has added one; the
    ||| quantifier is written exactly so that the added ones are skipped
    ||| too, which is a statement about a turn's CONTENTS where the
    ||| counted row's is about a run of turns. No `Amount` says it: the
    ||| number is whatever the turn turns out to hold.
    ||| The turn is "their next" and is no slot -- both printed lines
    ||| write it, and a possessor other than the subject's would need
    ||| the mention `SkipsNext` also has no room for.
    ||| The part is gated by `partAddable`: [CR#500.1] makes a turn
    ||| neither a phase nor a step, so "all turns of their next turn"
    ||| quantifies over nothing.
    ||| -- spelling: "[who] skips all [part]s of their next turn".
    SkipsAllOf : (who : Noun bs Player) -> (part : TurnPart) ->
                 {auto 0 ad : AddedPart part} -> Effect bs
    ExtraTurn : (who : Noun bs Player) -> (count : Amount bs) -> Effect bs
    AdditionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) ->
                     (followedBy : Maybe TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten followedBy} -> Effect bs
    ||| "[who] get(s) [count] additional [part](s) after this
    ||| step/phase": the "YOU GET" frame. Obeka, Splitter of Seconds,
    ||| Paradox Haze and The Ninth Doctor, 3 supported lines over 3
    ||| cards (measured 2026-09-02).
    |||
    ||| A SECOND ROW beside `AdditionalPart`, and the difference is
    ||| MEANING and not wording. [CR#500.10a]: "if an effect that says
    ||| 'you get' an additional step or phase would add a step or phase
    ||| to a turn other than its controller's, no steps or phases are
    ||| added." `AdditionalPart`'s existential frame ("there is an
    ||| additional combat phase") carries no such subject and no such
    ||| restriction, so a subject slot on that row would have had one
    ||| value meaning something the rule denies and another meaning
    ||| nothing at all. The subject is written and never derived: Paradox
    ||| Haze says it of "that player" where the other two say "you".
    ||| The ANCHOR is not a slot. All three lines write the deictic
    ||| "after this step" / "after this phase". [CR#500.8] and
    ||| [CR#500.9] both add relative to a SPECIFIED phase or step, and
    ||| what the demonstrative specifies is the part the ability is
    ||| resolving in; which of the two sort words is printed follows
    ||| from that part and is spelling, exactly as it is on
    ||| `AdditionalPart`. No supported "you get" line names its anchor.
    ||| Obeka's variable count rides the shared `Amount`, reading the
    ||| combat damage its own header announced.
    ||| -- spelling: "[who] get(s) [count] additional [part](s) after
    ||| this step", the sort word derived from the anchoring part.
    GetsAdditionalPart : (who : Noun bs Player) -> (part : TurnPart) ->
                         (count : Amount bs) ->
                         {auto 0 ad : AddedPart part} -> Effect bs

  ||| [CR#610.3] hangs the "until" rider on a one-shot that changes an
  ||| object's zone and [CR#610.4] on one that phases a permanent out.
  ||| Those are the only two one-shots the CR gives the rider, so every
  ||| False below is that pair of rules speaking rather than a count.
  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (DealDamageOwn _ _ _) = False
  heldUntilOk (ControllerSacrifices _) = False
  heldUntilOk (DoesntUntapNext _ _) = False
  heldUntilOk (SkipsNext _ _ _) = False
  heldUntilOk (ExtraTurn _ _) = False
  heldUntilOk (AdditionalPart _ _ _ _) = False
  heldUntilOk (GetsAdditionalPart _ _ _) = False
  heldUntilOk (SkipsAllOf _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  -- [CR#610.4]: "until" also rides a permanent phasing out, and the
  -- second one-shot phases it back in.
  heldUntilOk (TurnOver _) = False
  heldUntilOk (SetStatus PhasedOut _) = True
  heldUntilOk (SetStatus _ _) = False
  heldUntilOk (GetsCounters _ _ _) = False
  heldUntilOk (GetsCountersOfThoseKinds _ _) = False
  heldUntilOk (LosesCounters _ _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
  heldUntilOk (AttachTo _ _) = False
  heldUntilOk (Unattach _) = False
  heldUntilOk (BecomesBlocking _ _) = False
  heldUntilOk (StopsBlocking _ _) = False
  heldUntilOk (BecomesAttacking _ _) = False
  heldUntilOk (Regenerate _) = False
  heldUntilOk (CantBe _ _ _) = False
  heldUntilOk (GainsDesignation _ _ _ _) = False
  heldUntilOk (GameBecomes _) = False
  heldUntilOk (Concludes _ _) = False
  heldUntilOk GameDrawn = False
  heldUntilOk RestartsGame = False
  heldUntilOk (CounterSpell _) = False
  heldUntilOk (CopyStack _ _ _ _) = False
  heldUntilOk (ChooseNewTargets _) = False
  heldUntilOk (CopyTargets _ _) = False
  heldUntilOk (CopyCard _ _ _) = False
  heldUntilOk (Choose _ _) = False
  heldUntilOk (Move _ _ _) = True
  heldUntilOk (ExchangeLife _) = False
  heldUntilOk (ChangeLife _ _) = False
  heldUntilOk (AddMana _ _ _ _) = False
  heldUntilOk (Draw _ _) = False
  heldUntilOk (Expose _ _ _) = False
  heldUntilOk (Search _ _ _ _) = False
  heldUntilOk (Shuffle _) = False
  heldUntilOk (FlipCoins _ _) = False
  heldUntilOk (RollDice _ _ _) = False
  heldUntilOk (ResultsTable _) = False
  heldUntilOk (IgnoreOutcomes _) = False
  heldUntilOk (ShiftResult _) = False
  heldUntilOk (RollPlanarDie _ _) = False
  heldUntilOk ChaosEnsues = False
  heldUntilOk (StoreResults _) = False
  heldUntilOk (RerollStored _ _ _) = False
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Throughout _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (GetsEmblem _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  heldUntilOk (RemoveCountersAmong _ _ _) = False
  heldUntilOk (MoveCounters _ _ _ _) = False
  heldUntilOk (PutSameCounters _ _) = False
  heldUntilOk (PutCountersOfThoseKinds _ _) = False
  heldUntilOk (GiveCountersOfOwnKinds _) = False
  heldUntilOk (DoubleCountersOfOwnKinds _) = False
  heldUntilOk (Enact _ (Move _ _ _)) = True
  heldUntilOk (Enact _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (OnlyIf _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (Unless _ _ _) = False
  heldUntilOk (Define _ _) = False
  heldUntilOk (ForEachOf _ _) = False
  heldUntilOk (ForEachKindOf _ _ _ _) = False
  heldUntilOk (Repeat _) = False
  heldUntilOk (Repeated _ _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _ _ _) = False
  heldUntilOk (InsteadOf _ _) = False
  heldUntilOk (HeldUntil _ _) = False
  heldUntilOk (Reflexively _ _) = False
  heldUntilOk (ThisWay _ _ _) = False

  ||| Why a clause is or is not a reflexive trigger's enclosure
  ||| [CR#603.12]; kept as an enum rather than a Bool since a False can
  ||| stand on several different structural grounds.
  public export
  data EncloseUse
    = ||| No PLAYER takes the action, so "you do" has no subject to
      ||| inflect for [CR#603.12].
      EncAgentless
    | ||| No single taken action for the pro-verb to abbreviate.
      EncNotOneAction
    | ||| The clause schedules its action rather than taking it, so
      ||| nothing has "occurred earlier during the resolution" [CR#603.12].
      EncNotYetTaken
    | ||| A player's own single action.
      EncReflexive

  public export
  reflexEncloseUse : {0 bs : Bindings} -> Effect bs -> EncloseUse
  reflexEncloseUse (DealDamage _ _ _) = EncAgentless
  reflexEncloseUse (DealDamageOwn _ _ _) = EncAgentless
  reflexEncloseUse (ControllerSacrifices _) = EncReflexive
  reflexEncloseUse (DoesntUntapNext _ _) = EncAgentless
  reflexEncloseUse (SkipsNext _ _ _) = EncNotYetTaken
  reflexEncloseUse (ExtraTurn _ _) = EncNotYetTaken
  reflexEncloseUse (AdditionalPart _ _ _ _) = EncAgentless
  reflexEncloseUse (GetsAdditionalPart _ _ _) = EncNotYetTaken
  reflexEncloseUse (SkipsAllOf _ _) = EncNotYetTaken
  reflexEncloseUse (Distribute _ _ _) = EncAgentless
  reflexEncloseUse (Fights _ _) = EncAgentless
  reflexEncloseUse (ExchangeLife _) = EncAgentless
  reflexEncloseUse (ChangeLife _ _) = EncAgentless
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncReflexive
  reflexEncloseUse (Continuously _ _) = EncAgentless
  reflexEncloseUse (Throughout _ (GainsControl _ _)) = EncReflexive
  reflexEncloseUse (Throughout _ _) = EncAgentless
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _ _) = EncReflexive      -- 66, all of them offered
  reflexEncloseUse (Enact _ _) = EncReflexive
  -- a status change is the effect's, not a player's: [CR#603.12]'s
  -- agent form has no subject to inflect.
  reflexEncloseUse (TurnOver _) = EncAgentless
  reflexEncloseUse (SetStatus _ _) = EncAgentless
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (GetsCountersOfThoseKinds _ _) = EncAgentless
  reflexEncloseUse (LosesCounters _ _ _) = EncAgentless
  reflexEncloseUse (RemoveFromCombat _) = EncAgentless
  reflexEncloseUse (AttachTo _ _) = EncAgentless
  reflexEncloseUse (Unattach _) = EncAgentless
  reflexEncloseUse (BecomesBlocking _ _) = EncAgentless
  reflexEncloseUse (StopsBlocking _ _) = EncAgentless
  reflexEncloseUse (BecomesAttacking _ _) = EncAgentless
  reflexEncloseUse (Regenerate _) = EncAgentless
  reflexEncloseUse (CantBe _ _ _) = EncAgentless
  reflexEncloseUse (GainsDesignation _ _ _ _) = EncAgentless
  reflexEncloseUse (GameBecomes _) = EncAgentless
  reflexEncloseUse (Concludes _ _) = EncAgentless
  reflexEncloseUse GameDrawn = EncAgentless
  reflexEncloseUse RestartsGame = EncAgentless
  reflexEncloseUse (CounterSpell _) = EncAgentless
  reflexEncloseUse (CopyStack _ _ _ _) = EncAgentless
  reflexEncloseUse (ChooseNewTargets _) = EncReflexive
  reflexEncloseUse (CopyTargets _ _) = EncAgentless
  reflexEncloseUse (CopyCard _ _ _) = EncAgentless
  reflexEncloseUse (Create _ _ _ _) = EncReflexive -- 8
  reflexEncloseUse (GetsEmblem _ _) = EncAgentless
  reflexEncloseUse (PutCounters _ _ _) = EncReflexive    -- 8
  reflexEncloseUse (RemoveCounters _ _ _) = EncReflexive -- 7
  reflexEncloseUse (RemoveCountersAmong _ _ _) = EncReflexive
  reflexEncloseUse (MoveCounters _ _ _ _) = EncReflexive
  reflexEncloseUse (PutSameCounters _ _) = EncReflexive
  reflexEncloseUse (PutCountersOfThoseKinds _ _) = EncReflexive
  reflexEncloseUse (GiveCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (DoubleCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (Move _ _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _ _) = EncReflexive       -- 1
  reflexEncloseUse (Search _ _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  reflexEncloseUse (FlipCoins _ _) = EncReflexive
  reflexEncloseUse (RollDice _ _ _) = EncReflexive
  reflexEncloseUse (ResultsTable _) = EncNotOneAction
  -- no PLAYER is written taking either action: the ignore and the
  -- shift are stated of the roll the clause made.
  reflexEncloseUse (IgnoreOutcomes _) = EncAgentless
  reflexEncloseUse (ShiftResult _) = EncAgentless
  reflexEncloseUse (RollPlanarDie _ _) = EncReflexive
  reflexEncloseUse ChaosEnsues = EncAgentless
  reflexEncloseUse (StoreResults _) = EncAgentless
  reflexEncloseUse (RerollStored _ _ _) = EncReflexive
  -- [CR#603.12] writes the reflexive over what a player did or didn't
  -- do, so a declined arm leaves one offered action to inflect.
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  -- [CR#603.12] asks whether the player took the action, and a postposed
  -- condition gates whether the enclosure's ONE action happens rather
  -- than adding a second one to abbreviate: "Then sacrifice it if it has
  -- five or more bloodstain counters on it. When you do, ..." (Blood
  -- Spatter Analysis) leaves exactly the question the pro-verb puts.
  -- `May`'s unbranched offer passes through for the same reason.
  reflexEncloseUse (OnlyIf e _ _) = reflexEncloseUse e
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (Unless _ _ _) = EncNotOneAction
  reflexEncloseUse (Define _ _) = EncAgentless
  reflexEncloseUse (ForEachOf _ _) = EncNotOneAction
  reflexEncloseUse (ForEachKindOf _ _ _ _) = EncNotOneAction
  reflexEncloseUse (Repeat _) = EncNotOneAction
  -- [CR#603.12a]'s named exception: "if a resolving spell or ability
  -- includes a choice to pay a cost multiple times and creates a
  -- triggered ability that triggers when that payment is made, paying
  -- that cost one or more times causes the reflexive triggered ability
  -- to trigger only once". A repetition whose body is a PAYMENT is
  -- therefore one enclosure with one agent, not the many-actions shape
  -- every other repetition is; the trigger it seats restates the offer
  -- ("When you pay this cost one or more times, ...") rather than
  -- naming an iteration.
  reflexEncloseUse (Repeated _ (Pay _ _ _)) = EncReflexive
  reflexEncloseUse (Repeated _ (May _ (Pay _ _ _) Nothing _)) = EncReflexive
  reflexEncloseUse (Repeated _ _) = EncNotOneAction
  reflexEncloseUse (Sequentially _) = EncNotOneAction
  reflexEncloseUse (Simultaneously _) = EncNotOneAction
  reflexEncloseUse (Modal _ _) = EncNotOneAction
  reflexEncloseUse (InsteadOf _ _) = EncNotOneAction
  reflexEncloseUse (Reflexively _ _) = EncNotOneAction
  reflexEncloseUse (ThisWay _ _ _) = EncNotOneAction
  reflexEncloseUse (Delayed _ _ _ _) = EncNotYetTaken
  reflexEncloseUse (HeldUntil _ _) = EncNotYetTaken

  public export
  admitsReflexEnclosure : EncloseUse -> Bool
  admitsReflexEnclosure EncAgentless = False
  admitsReflexEnclosure EncNotOneAction = False
  admitsReflexEnclosure EncNotYetTaken = False
  admitsReflexEnclosure EncReflexive = True

  public export
  ReflexEnclosure : Effect bs -> Type
  ReflexEnclosure e = So (admitsReflexEnclosure (reflexEncloseUse e))

  ||| Does the enclosure itself cause the event "this way" points at?
  ||| Only a clause that creates a delayed triggered ability fails: that
  ||| ability is a separate one that resolves on its own [CR#603.7], so
  ||| the deed when it comes is not the enclosure's for "this way" to
  ||| name. An effect that applies later without a second ability — a
  ||| replacement, a continuous effect, a skipped step — still caused it.
  ||| Separate from `reflexEncloseUse`: that asks for an agent to
  ||| inflect, this asks for a caused event, so a compound or agentless
  ||| enclosure is fine here.
  public export
  thisWayOutcomeOk : {0 bs : Bindings} -> Effect bs -> Bool
  thisWayOutcomeOk (Delayed _ _ _ _) = False
  -- an offer's outcome is its body's; the arms are separate sentences.
  thisWayOutcomeOk (May _ body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (DoesntUntapNext _ _) = True
  thisWayOutcomeOk (SkipsNext _ _ _) = True
  thisWayOutcomeOk (ExtraTurn _ _) = True
  thisWayOutcomeOk (AdditionalPart _ _ _ _) = True
  thisWayOutcomeOk (GetsAdditionalPart _ _ _) = True
  thisWayOutcomeOk (SkipsAllOf _ _) = True
  thisWayOutcomeOk (HeldUntil _ _) = True
  thisWayOutcomeOk (DealDamage _ _ _) = True
  thisWayOutcomeOk (Distribute _ _ _) = True
  thisWayOutcomeOk (Fights _ _) = True
  thisWayOutcomeOk (TurnOver _) = True
  thisWayOutcomeOk (SetStatus _ _) = True
  thisWayOutcomeOk (DealDamageOwn _ _ _) = True
  thisWayOutcomeOk (ControllerSacrifices _) = True
  thisWayOutcomeOk (GetsCounters _ _ _) = True
  thisWayOutcomeOk (GetsCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (LosesCounters _ _ _) = True
  thisWayOutcomeOk (RemoveFromCombat _) = True
  thisWayOutcomeOk (AttachTo _ _) = True
  thisWayOutcomeOk (Unattach _) = True
  thisWayOutcomeOk (BecomesBlocking _ _) = True
  thisWayOutcomeOk (StopsBlocking _ _) = True
  thisWayOutcomeOk (BecomesAttacking _ _) = True
  thisWayOutcomeOk (Regenerate _) = True
  thisWayOutcomeOk (CantBe _ _ _) = True
  thisWayOutcomeOk (GainsDesignation _ _ _ _) = True
  thisWayOutcomeOk (GameBecomes _) = True
  thisWayOutcomeOk (Concludes _ _) = True
  thisWayOutcomeOk GameDrawn = True
  thisWayOutcomeOk RestartsGame = True
  thisWayOutcomeOk (CounterSpell _) = True
  thisWayOutcomeOk (CopyStack _ _ _ _) = True
  thisWayOutcomeOk (ChooseNewTargets _) = True
  thisWayOutcomeOk (CopyTargets _ _) = True
  thisWayOutcomeOk (CopyCard _ _ _) = True
  thisWayOutcomeOk (Choose _ _) = True
  thisWayOutcomeOk (Move _ _ _) = True
  thisWayOutcomeOk (ExchangeLife _) = True
  thisWayOutcomeOk (ChangeLife _ _) = True
  thisWayOutcomeOk (AddMana _ _ _ _) = True
  thisWayOutcomeOk (Draw _ _) = True
  thisWayOutcomeOk (Expose _ _ _) = True
  thisWayOutcomeOk (Search _ _ _ _) = True
  thisWayOutcomeOk (Shuffle _) = True
  thisWayOutcomeOk (FlipCoins _ _) = True
  thisWayOutcomeOk (RollDice _ _ _) = True
  thisWayOutcomeOk (ResultsTable _) = True
  thisWayOutcomeOk (IgnoreOutcomes _) = True
  thisWayOutcomeOk (ShiftResult _) = True
  thisWayOutcomeOk (RollPlanarDie _ _) = True
  thisWayOutcomeOk ChaosEnsues = True
  thisWayOutcomeOk (StoreResults _) = True
  thisWayOutcomeOk (RerollStored _ _ _) = True
  thisWayOutcomeOk (Continuously _ _) = True
  thisWayOutcomeOk (Throughout _ _) = True
  thisWayOutcomeOk (Create _ _ _ _) = True
  thisWayOutcomeOk (GetsEmblem _ _) = True
  thisWayOutcomeOk (PutCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCountersAmong _ _ _) = True
  thisWayOutcomeOk (MoveCounters _ _ _ _) = True
  thisWayOutcomeOk (PutSameCounters _ _) = True
  thisWayOutcomeOk (PutCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (GiveCountersOfOwnKinds _) = True
  thisWayOutcomeOk (DoubleCountersOfOwnKinds _) = True
  thisWayOutcomeOk (Enact _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (Pay _ _ _) = True
  thisWayOutcomeOk (OnlyIf _ _ _) = True
  thisWayOutcomeOk (If _ _ _) = True
  thisWayOutcomeOk (Unless _ _ _) = True
  thisWayOutcomeOk (Define _ _) = True
  thisWayOutcomeOk (ForEachOf _ _) = True
  thisWayOutcomeOk (ForEachKindOf _ _ _ _) = True
  thisWayOutcomeOk (Repeat _) = True
  thisWayOutcomeOk (Repeated _ _) = True
  thisWayOutcomeOk (Sequentially _) = True
  thisWayOutcomeOk (Simultaneously _) = True
  thisWayOutcomeOk (Modal _ _) = True
  thisWayOutcomeOk (InsteadOf _ _) = True
  thisWayOutcomeOk (Reflexively _ _) = True
  thisWayOutcomeOk (ThisWay _ _ _) = True

  public export
  ThisWayOutcome : Effect bs -> Type
  ThisWayOutcome e = So (thisWayOutcomeOk e)

  public export
  payableOk : {0 bs : Bindings} -> Cost bs -> Bool
  payableOk (Mana _) = True
  payableOk (ScaledMana _ _) = True
  payableOk TapSymbol = False
  payableOk UntapSymbol = False
  payableOk (LoyaltySymbol _) = False
  payableOk (Do _) = True
  payableOk (Compound _) = True
  payableOk (EitherCost l r) = payableOk l && payableOk r
  payableOk ItsManaCost = True

  public export
  Payable : Cost bs -> Type
  Payable {bs} c = So (payableOk c)

  ||| [CR#118.1] makes a cost an action a player carries out, so any
  ||| instruction a payer can follow is admitted; `costNounOk` keeps its
  ||| own hold on which nouns a cost may name. Refused: the nodes that
  ||| instruct nothing at payment (a static, a replacement, a scheduled
  ||| or reflexive trigger, a skip [CR#614.10]) and the coordinations,
  ||| which claim an order [CR#601.2h] pays in any — `Compound` is the
  ||| cost-side telescope.
  |||
  ||| ATTESTATION, re-measured 2026-08-28. 78 rows, of which 20 refuse,
  ||| and every refusal is the sentence's own -- no row is refused for
  ||| want of a witness any more. The four cumulative upkeeps recorded
  ||| as disagreeing with this table are attested by benches and needed
  ||| nothing minted: Braid of Fire's `AddMana`, Psychic Vortex's
  ||| `Draw`, Varchild's War-Riders' `Create` and Wall of Shards'
  ||| `ChangeLife Up` all read `True` here. NO SECOND TABLE keyed by the
  ||| `Cost` carrier is owed -- [CR#118.1] answers for every carrier at
  ||| once -- and the colon-measured zeros of the activation cost are
  ||| untouched by that, the widening having admitted no new action
  ||| there.
  public export
  costActionOk : {0 bs : Bindings} -> Effect bs -> Bool
  costActionOk (DealDamage src _ _) = costNounOk src
  costActionOk (DealDamageOwn src _ _) = costNounOk src
  costActionOk (ControllerSacrifices n) = costNounOk n
  costActionOk (DoesntUntapNext n _) = costNounOk n
  costActionOk (SkipsNext _ _ _) = False
  costActionOk (ExtraTurn who _) = costNounOk who
  costActionOk (AdditionalPart _ _ _ _) = True
  costActionOk (GetsAdditionalPart who _ _) = costNounOk who
  costActionOk (SkipsAllOf _ _) = False
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (TurnOver n) = costNounOk n
  costActionOk (SetStatus _ n) = costNounOk n
  costActionOk (GetsCounters who _ _) = costNounOk who
  -- the distributive twin reads an announced batch, which no cost has.
  costActionOk (GetsCountersOfThoseKinds _ _) = False
  costActionOk (LosesCounters who _ _) = costNounOk who
  costActionOk (RemoveFromCombat n) = costNounOk n
  costActionOk (AttachTo what _) = costNounOk what
  costActionOk (Unattach what) = costNounOk what
  costActionOk (BecomesBlocking n _) = costNounOk n
  costActionOk (StopsBlocking n _) = costNounOk n
  costActionOk (BecomesAttacking n _) = costNounOk n
  costActionOk (Regenerate n) = costNounOk n
  costActionOk (CantBe e _ _) = costActionOk e
  costActionOk (GainsDesignation n _ _ _) = costNounOk n
  costActionOk (GameBecomes _) = True
  costActionOk (Concludes _ _) = True
  costActionOk GameDrawn = True
  costActionOk RestartsGame = False
  costActionOk (CounterSpell _) = True
  costActionOk (CopyStack _ what _ _) = costNounOk what
  costActionOk (ChooseNewTargets what) = costNounOk what
  costActionOk (CopyTargets copy _) = costNounOk copy
  costActionOk (CopyCard _ what _) = costNounOk what
  costActionOk (Choose n _) = costNounOk n
  costActionOk (Move what _ _) = costNounOk what
  costActionOk (ExchangeLife parties) = costNounOk parties
  costActionOk (ChangeLife _ _) = True
  costActionOk (AddMana who _ _ _) = costNounOk who
  costActionOk (Draw _ _) = True
  costActionOk (Expose _ who _) = costNounOk who
  costActionOk (Search who _ _ _) = costNounOk who
  costActionOk (Shuffle whose) = costNounOk whose
  costActionOk (FlipCoins who _) = costNounOk who
  costActionOk (RollDice who _ _) = costNounOk who
  costActionOk (ResultsTable _) = False
  -- each reads the roll a clause before it made, which no cost has.
  costActionOk (IgnoreOutcomes _) = False
  costActionOk (ShiftResult _) = False
  costActionOk (StoreResults _) = False
  costActionOk (RollPlanarDie who _) = costNounOk who
  costActionOk ChaosEnsues = False
  costActionOk (RerollStored who _ _) = costNounOk who
  costActionOk (Continuously _ _) = False
  costActionOk (Throughout _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (RemoveCountersAmong _ _ among) = costNounOk among
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  costActionOk (PutSameCounters src dst) = costNounOk src && costNounOk dst
  costActionOk (GiveCountersOfOwnKinds on) = costNounOk on
  costActionOk (DoubleCountersOfOwnKinds on) = costNounOk on
  -- the distributive kind anaphor reads an announced batch; no cost
  -- announces one, so the clause instructs nothing at payment.
  costActionOk (PutCountersOfThoseKinds _ _) = False
  costActionOk (Enact _ e) = costActionOk e
  costActionOk (Does _ _ e) = costActionOk e
  costActionOk (Pay _ _ _) = False
  costActionOk (May _ body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (OnlyIf e _ otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (If _ e otherwise) = costActionOk e && costActionOkOpt otherwise
  -- an offer another player answers at resolution [CR#118.12a]
  costActionOk (Unless _ _ _) = False
  -- a definition instructs nothing at payment but rides a cost's own
  -- amount; admitted so the cost telescope can carry "where X is".
  costActionOk (Define _ _) = True
  costActionOk (ForEachOf _ body) = costActionOk body
  costActionOk (ForEachKindOf _ _ _ body) = costActionOk body
  costActionOk (Repeat _) = False
  costActionOk (Repeated _ body) = costRepeatedOk body
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ modes) = costActionsOk modes
  costActionOk (Delayed _ _ _ _) = False
  costActionOk (InsteadOf _ _) = False
  costActionOk (HeldUntil _ _) = False
  costActionOk (Reflexively _ _) = False
  costActionOk (ThisWay _ _ _) = False

  public export
  costActionOkOpt : {0 bs : Bindings} -> Maybe (Effect bs) -> Bool
  costActionOkOpt Nothing = True
  costActionOkOpt (Just e) = costActionOk e

  public export
  costActionsOk : {0 bs : Bindings} -> List (Effect bs) -> Bool
  costActionsOk [] = True
  costActionsOk (e :: es) = costActionOk e && costActionsOk es

  public export
  CostAction : Effect bs -> Type
  CostAction {bs} e = So (costActionOk e)

  public export
  HeldClause : Effect bs -> Type
  HeldClause {bs} e = So (heldUntilOk e)

  public export
  isInstead : {0 bs : Bindings} -> Effect bs -> Bool
  isInstead (InsteadOf _ _) = True
  isInstead _ = False

  public export
  NotInstead : Effect bs -> Type
  NotInstead {bs} e = So (not (isInstead e))

  public export
  modeCount : {0 bs : Bindings} -> List (Effect bs) -> Nat
  modeCount [] = Z
  modeCount (_ :: es) = S (modeCount es)

  public export
  effEq : {0 bs : Bindings} -> Effect bs -> Effect bs -> Bool
  effEq (DealDamage _ _ _) _ = False
  effEq (DealDamageOwn _ _ _) _ = False
  effEq (ControllerSacrifices _) _ = False
  effEq (DoesntUntapNext n s) (DoesntUntapNext m t) = nounEqRef n m && boundEq s t
  effEq (DoesntUntapNext _ _) _ = False
  effEq (SkipsNext w p c) (SkipsNext x q d) =
    nounEqRef w x && p == q && boundEq c d
  effEq (SkipsNext _ _ _) _ = False
  effEq (ExtraTurn w c) (ExtraTurn x d) = nounEqRef w x && boundEq c d
  effEq (ExtraTurn _ _) _ = False
  effEq (AdditionalPart p a c _) (AdditionalPart q b d _) =
    p == q && a == b && boundEq c d
  effEq (AdditionalPart _ _ _ _) _ = False
  effEq (GetsAdditionalPart w p c) (GetsAdditionalPart x q d) =
    nounEqRef w x && p == q && boundEq c d
  effEq (GetsAdditionalPart _ _ _) _ = False
  effEq (SkipsAllOf w p) (SkipsAllOf x q) = nounEqRef w x && p == q
  effEq (SkipsAllOf _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (TurnOver a) (TurnOver b) = nounEqRef a b
  effEq (TurnOver _) _ = False
  effEq (SetStatus v a) (SetStatus w b) = sameStatusVal v w && nounEqRef a b
  effEq (SetStatus _ _) _ = False
  -- The two player-counter rows compare kind AND amount, and both narrow
  -- to `You` to do it: an `Amount` is indexed by its subject's discourse,
  -- so amounts under two different subjects are not the same type. `Draw`
  -- narrows the same way for the same reason.
  effEq (GetsCounters You x j) (GetsCounters You y l) = j == l && boundEq x y
  effEq (GetsCounters _ _ _) _ = False
  effEq (GetsCountersOfThoseKinds _ _) _ = False
  effEq (LosesCounters You j Nothing) (LosesCounters You l Nothing) = j == l
  effEq (LosesCounters You j (Just x)) (LosesCounters You l (Just y)) =
    j == l && boundEq x y
  effEq (LosesCounters _ _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
  -- the host is described at its OWN kind, so two attach clauses carry
  -- no comparable host mention; the attached object is what they are
  -- compared on.
  effEq (AttachTo a _) (AttachTo b _) = nounEqRef a b
  effEq (AttachTo _ _) _ = False
  effEq (Unattach a) (Unattach b) = nounEqRef a b
  effEq (Unattach _) _ = False
  effEq (BecomesBlocking _ _) _ = False
  effEq (StopsBlocking _ _) _ = False
  effEq (BecomesAttacking _ _) _ = False
  effEq (Regenerate a) (Regenerate b) = nounEqRef a b
  effEq (Regenerate _) _ = False
  effEq (CantBe _ _ _) _ = False
  effEq (GainsDesignation _ _ _ _) _ = False
  effEq (GameBecomes a) (GameBecomes b) = a == b
  effEq (GameBecomes _) _ = False
  effEq (Concludes v a) (Concludes w b) = v == w && nounEqRef a b
  effEq (Concludes _ _) _ = False
  effEq GameDrawn GameDrawn = True
  effEq GameDrawn _ = False
  effEq RestartsGame RestartsGame = True
  effEq RestartsGame _ = False
  -- kind-indexed, so two subjects need not share a kind to compare;
  -- `Choose`'s row gives up on the same ground, and the retarget row
  -- joined them when the copy verb's complement opened past `Object`.
  effEq (CounterSpell _) _ = False
  effEq (CopyStack _ _ _ _) _ = False
  effEq (ChooseNewTargets _) _ = False
  effEq (CopyTargets _ _) _ = False
  effEq (CopyCard _ _ _) _ = False
  effEq (Choose _ _) _ = False
  effEq (Move a s _) (Move b t _) = nounEqRef a b && zoneSort s == zoneSort t
  effEq (Move _ _ _) _ = False
  effEq (ExchangeLife a) (ExchangeLife b) = nounEqRef a b
  effEq (ExchangeLife _) _ = False
  effEq (ChangeLife _ _) _ = False
  effEq (AddMana _ _ _ _) _ = False
  effEq (Draw You a) (Draw You b) = boundEq a b
  effEq (Draw _ _) _ = False
  effEq (Expose _ _ _) _ = False
  effEq (Search _ _ _ _) _ = False
  effEq (Shuffle _) _ = False
  effEq (FlipCoins _ _) _ = False
  effEq (RollDice _ _ _) _ = False
  effEq (ResultsTable _) _ = False
  effEq (IgnoreOutcomes _) _ = False
  effEq (ShiftResult _) _ = False
  effEq (RollPlanarDie _ _) _ = False
  effEq ChaosEnsues _ = False
  effEq (StoreResults _) _ = False
  effEq (RerollStored _ _ _) _ = False
  effEq (Continuously _ _) _ = False
  effEq (Throughout _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (GetsEmblem _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (RemoveCountersAmong _ _ _) _ = False
  effEq (MoveCounters _ _ _ _) _ = False
  effEq (PutSameCounters _ _) _ = False
  effEq (PutCountersOfThoseKinds _ _) _ = False
  effEq (GiveCountersOfOwnKinds _) _ = False
  effEq (DoubleCountersOfOwnKinds _) _ = False
  effEq (Enact v e) (Enact w f) = v == w && effEq e f
  effEq (Enact _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (Pay _ _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (OnlyIf _ _ _) _ = False
  effEq (If _ _ _) _ = False
  effEq (Unless _ _ _) _ = False
  effEq (Define _ _) _ = False
  effEq (ForEachOf _ _) _ = False
  effEq (ForEachKindOf _ _ _ _) _ = False
  effEq (Repeat _) _ = False
  effEq (Repeated _ _) _ = False
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _ _ _) _ = False
  effEq (InsteadOf _ _) _ = False
  effEq (HeldUntil _ _) _ = False
  effEq (Reflexively _ _) _ = False
  effEq (ThisWay _ _ _) _ = False

  public export
  anyEffEq : {0 bs : Bindings} -> Effect bs -> List (Effect bs) -> Bool
  anyEffEq e [] = False
  anyEffEq e (f :: fs) = effEq e f || anyEffEq e fs

  public export
  distinctModes : {0 bs : Bindings} -> List (Effect bs) -> Bool
  distinctModes [] = True
  distinctModes (e :: es) = not (anyEffEq e es) && distinctModes es

  public export
  data Effects : Nat -> Bindings -> Type where
    Nil : Effects Z bs
    (::) : (e : Effect bs) ->
           Effects n (effIntro e) -> Effects (S n) bs

  namespace Sim
    public export
    data SimEffects : Nat -> Bindings -> Type where
      Nil : SimEffects Z bs
      (::) : (e : Effect bs) ->
             SimEffects n (annIntro e) -> SimEffects (S n) bs

  ||| The mentions a clause adds ON TOP of the context it was read in:
  ||| `effIntro` less that context. Every row of `effIntro` answers
  ||| `<new> ++ bs` or rewrites a binding already in `bs` in place, so the
  ||| length difference is exactly the clause's own introductions and a
  ||| clause that only re-zoned an outer mention has a delta of none.
  public export
  effDelta : {bs : Bindings} -> Effect bs -> Bindings
  effDelta e = take (length (effIntro e) `minus` length bs) (effIntro e)

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (DealDamageOwn src c to) = outcomeB DamageDealt :: nomIntro to
  -- the controller is announced, and the patient is stamped and re-zoned
  -- by the act that took it [CR#701.21a] -- the labeled move's own
  -- answer, written out because the agent is derived rather than given.
  effIntro (ControllerSacrifices n) =
    MkBinding TheD Player OneOf PlayerP
      :: moveIntro (Just "Sacrifice") n (Just Graveyard)
  effIntro (Fights a b) = nomIntro b
  effIntro (TurnOver n) = nomIntro n
  effIntro (SetStatus _ n) = nomIntro n
  effIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  effIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  effIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  effIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  effIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  effIntro (SkipsAllOf w _) = nomIntro w
  effIntro (GetsCounters who amt _) = amtIntro amt
  effIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  effIntro (LosesCounters who _ amt) = optAmtIntro amt
  effIntro (RemoveFromCombat n) = nomIntro n
  effIntro (AttachTo _ host) = nomIntro host
  effIntro (Unattach what) = nomIntro what
  effIntro (BecomesBlocking _ what) = nomIntro what
  effIntro (StopsBlocking _ what) = nomIntro what
  effIntro (BecomesAttacking n NoDefender) = nomIntro n
  effIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  effIntro (Regenerate n) = nomIntro n
  effIntro (CantBe e _ _) = effIntro e
  effIntro (GainsDesignation n _ _ _) = nomIntro n
  effIntro (GameBecomes _) = bs
  effIntro (Concludes _ who) = nomIntro who
  effIntro GameDrawn = bs
  effIntro RestartsGame = bs
  effIntro (CounterSpell what) = nomIntro what
  effIntro (CopyStack {k} {ph} agent what times exc) =
    MkBinding TheD k (outputPlur (nounPlur what) (amtPlur times))
              (copyPayload ph (nounTy what))
      :: amtIntro times
  effIntro (ChooseNewTargets what) = nomIntro what
  -- the target is named and the copy was already standing, so the
  -- statement announces what its complement introduced and nothing else.
  effIntro (CopyTargets copy whom) = nomIntro whom
  -- [CR#707.12] creates the copy "in the same zone the object is in",
  -- so the mention records the copied card's zone where the stack copy
  -- records the stack.
  effIntro (CopyCard who what times) =
    MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
              (ObjectP (nounTy what) (nounZone what) Nothing (Just CopyOrigin) Nothing)
      :: amtIntro times
  effIntro (Choose n Nothing) = chosenIntro n
  effIntro (Choose n (Just b)) = nounDelta b ++ chosenIntro n
  -- a destination that shuffles [CR#701.24c] takes the discourse with
  -- it, exactly as the bare `Shuffle` does.
  effIntro (Move what to _) =
    afterMoveTo to (moveIntro Nothing what (Just (zoneSort to)))
  effIntro (ExchangeLife parties) =
    outcomeB LifeGained :: outcomeB LifeLost :: nomIntro parties
  effIntro (ChangeLife who (Up a)) = outcomeB LifeGained :: lifeIntro (Up a)
  effIntro (ChangeLife who (Down a)) = outcomeB LifeLost :: lifeIntro (Down a)
  effIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  -- [CR#106.4] sends the mana to a pool, where it "can stay ... as
  -- unspent mana" -- so the clause leaves a mention the next sentence
  -- reads as "this mana". On `DealDamage`'s channels exactly: the
  -- outcome is in `effIntro` and `deedDelta` and in neither of the two
  -- pre-resolution channels, because nothing is in a pool until the
  -- clause resolves.
  effIntro (AddMana who amt _ _) = outcomeB ManaAdded :: amtIntro amt
  effIntro (Draw who amt) = amtIntro amt
  effIntro (Expose v who what) = exposedIntro what
  -- the found card is stamped by the label that found it, which is what
  -- keeps it out of a later shuffle [CR#701.24b].
  effIntro (Search who sc q p) =
    MkBinding AD Object (quantPlur q)
              (ObjectP (seedTy p) (searchZone sc)
                       (mkStamp (Just "Search") Nothing False) Nothing Nothing)
      :: (quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who)
  effIntro (Shuffle whose) = afterShuffle (nomIntro whose)
  effIntro (FlipCoins who count) = outcomeB CoinFlipped :: flipScopeIntro count
  effIntro (RollDice who count _) = outcomeB RollResult :: amtIntro count
  effIntro (ResultsTable rows) = bs
  effIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  effIntro (ShiftResult amt) = amtIntro amt
  effIntro (RollPlanarDie who count) = outcomeB PlanarRolled :: amtIntro count
  effIntro ChaosEnsues = bs
  effIntro (StoreResults on) = nomIntro on
  effIntro (RerollStored _ _ whose) = nomIntro whose
  effIntro (Continuously se _) = staticIntro se
  effIntro (Throughout _ se) = staticIntro se
  effIntro (Create agent count spec riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin) Nothing)
      :: (specDelta spec ++ amtIntro count)
  effIntro (GetsEmblem who _) = nomIntro who
  effIntro (PutCounters amt kind on) = nomIntro on
  effIntro (Distribute (DividedDamage _) amt among) = outcomeB DamageDealt :: nomIntro among
  effIntro (Distribute (DistributedCounters _) amt among) = nomIntro among
  effIntro (RemoveCounters q kind from) = outcomeB CountersRemoved :: nomIntro from
  effIntro (RemoveCountersAmong q kind among) = outcomeB CountersRemoved :: nomIntro among
  effIntro (MoveCounters amt kind src dst) = nomIntro dst
  effIntro (PutSameCounters src dst) = nomIntro dst
  effIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  effIntro (GiveCountersOfOwnKinds on) = nomIntro on
  effIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  effIntro (Enact v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Enact v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Enact _ e) = effIntro e
  effIntro (Does s v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Does s v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Does s v e) = effIntro e
  -- A repeating offer leaves the number of times it was taken, which is
  -- what "that many" reads on the five Adversaries. `RepeatCount` is
  -- already the mention for a count a clause WROTE rather than a batch
  -- it produced, and this is that count at the payment.
  effIntro (Pay who c PaidOnce) = costIntro c
  effIntro (Pay who c AnyNumberOfTimes) = outcomeB RepeatCount :: costIntro c
  effIntro (Pay who c (UpToTimes _)) = outcomeB RepeatCount :: costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- a conditioned clause exports what it ANNOUNCED and no more: the
  -- condition may have failed, so nothing the clause would have DONE
  -- stands after it -- but [CR#601.2c] chose its targets as the spell was
  -- cast, and a target is chosen whether or not the condition holds. So
  -- "…gets +2/+2 until end of turn if its power is 2. Then IT fights…"
  -- (Savage Swipe, 5 supported lines) reads the target back. The
  -- otherwise arm changes nothing here: both arms are typed over
  -- `annIntro e`, so the announcement is what either of them leaves.
  effIntro (OnlyIf e c oth) = annIntro e
  -- the LEADING conditional exports nothing even so: its consequent is
  -- typed over the condition's own delta, and a condition that failed
  -- announced nothing for the text after it to read.
  effIntro (If c e oth) = bs
  effIntro (Unless e who c) = bs
  effIntro (Define l amt) = defineLetter l (amtIntro amt)
  -- a DOMAIN-DRIVEN loop has a determinate iteration count, so the union
  -- over its passes is well-formed and is exported on `Repeated`'s
  -- precedent: one summary mention per mention the body introduced, same
  -- payload and same stamp, differing only in naming many where one pass
  -- named one. No count rides along -- `Repeated` exports one because the
  -- text WROTE a number, and here the number is the group's own size,
  -- which `GroupSize` already reads off the group's mention.
  effIntro (ForEachOf grp body) = pluralizeDelta (effDelta body) ++ bs
  effIntro (ForEachKindOf _ _ _ _) = bs
  -- an OPEN-ENDED repetition exports nothing: with no bound there is no
  -- determinate batch to summarise, and an until-condition supplies a
  -- stopping test rather than a count.
  effIntro (Repeat _) = bs
  effIntro (Repeated n body) =
    outcomeB RepeatCount :: (pluralizeDelta (effDelta body) ++ amtIntro n)
  effIntro (Sequentially es) = effsIntro es
  effIntro (Simultaneously es) = simIntro es
  effIntro (Modal q modes) = quantDelta q ++ bs
  effIntro (Delayed ev _ _ e) = bs               -- a future clause mentions nothing NOW
  effIntro (Reflexively body trig) = effIntro body
  effIntro (ThisWay body ev trig) = effIntro body
  effIntro (InsteadOf replaced repl) = annIntro replaced
  effIntro (HeldUntil e ev) = annIntro e

  ||| What a clause has announced by the time its own trailing condition
  ||| is read — the pre-resolution twin of `effIntro`, since the
  ||| condition is written after the clause but evaluated before it.
  public export
  preIntro : {bs : Bindings} -> Effect bs -> Bindings
  preIntro (DealDamage src amt to) = nomIntro to
  preIntro (DealDamageOwn src c to) = nomIntro to
  preIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  preIntro (Distribute v amt among) = nomIntro among
  preIntro (Fights a b) = nomIntro b
  preIntro (TurnOver n) = nomIntro n
  preIntro (SetStatus _ n) = nomIntro n
  preIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  preIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  preIntro (ExtraTurn w count) = amtDelta count ++ nomIntro w
  preIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  preIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  preIntro (SkipsAllOf w _) = nomIntro w
  preIntro (GetsCounters who amt _) = amtIntro amt
  preIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  preIntro (LosesCounters who _ amt) = optAmtIntro amt
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (AttachTo _ host) = nomIntro host
  preIntro (Unattach what) = nomIntro what
  preIntro (BecomesBlocking _ what) = nomIntro what
  preIntro (StopsBlocking _ what) = nomIntro what
  preIntro (BecomesAttacking n NoDefender) = nomIntro n
  preIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  preIntro (Regenerate n) = nomIntro n
  preIntro (CantBe e _ _) = preIntro e
  preIntro (GainsDesignation n _ _ _) = nomIntro n
  preIntro (GameBecomes _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro RestartsGame = bs
  preIntro (CounterSpell what) = nomIntro what
  preIntro (CopyStack agent what times exc) = amtIntro times
  preIntro (ChooseNewTargets what) = nomIntro what
  preIntro (CopyTargets copy whom) = nomIntro whom
  preIntro (CopyCard who what times) = amtIntro times
  preIntro (Choose n _) = chosenIntro n
  preIntro (Move what to _) = nomIntro what
  preIntro (ExchangeLife parties) = nomIntro parties
  preIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  preIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  preIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  preIntro (AddMana who amt _ _) = amtIntro amt
  preIntro (Draw who amt) = amtIntro amt
  preIntro (Expose v who what) = exposedIntro what
  preIntro (Search who sc q p) =
    quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who
  preIntro (Shuffle whose) = nomIntro whose
  preIntro (FlipCoins who count) = flipScopeIntro count
  preIntro (RollDice who count _) = amtIntro count
  preIntro (ResultsTable rows) = bs
  preIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  preIntro (ShiftResult amt) = amtIntro amt
  preIntro (RollPlanarDie who count) = amtIntro count
  preIntro ChaosEnsues = bs
  preIntro (StoreResults on) = nomIntro on
  preIntro (RerollStored _ _ whose) = nomIntro whose
  preIntro (Continuously se _) = staticIntro se
  preIntro (Throughout _ se) = staticIntro se
  preIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  preIntro (GetsEmblem who _) = nomIntro who
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters q kind from) = nomIntro from
  preIntro (RemoveCountersAmong q kind among) = nomIntro among
  preIntro (MoveCounters amt kind src dst) = nomIntro dst
  preIntro (PutSameCounters src dst) = nomIntro dst
  preIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  preIntro (GiveCountersOfOwnKinds on) = nomIntro on
  preIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  preIntro (Enact v (Move what to _)) = nomIntro what
  preIntro (Enact _ e) = preIntro e
  preIntro (Does s v (Move what to _)) = nomIntro what
  preIntro (Does s v e) = preIntro e
  preIntro (Pay who c _) = nomIntro who
  preIntro (May d body did notd) = mayIntro body did notd
  preIntro (OnlyIf e c oth) = annIntro e
  preIntro (If c e oth) = bs
  preIntro (Unless e who c) = annIntro e
  preIntro (Define l amt) = defineLetter l (amtIntro amt)
  preIntro (ForEachOf _ _) = bs
  preIntro (ForEachKindOf _ _ _ _) = bs
  preIntro (Repeat _) = bs
  preIntro (Repeated n _) = amtIntro n
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = quantDelta q ++ bs
  preIntro (Delayed ev _ _ e) = bs
  preIntro (Reflexively body trig) = preIntro body
  preIntro (ThisWay body ev trig) = preIntro body
  preIntro (InsteadOf replaced repl) = annIntro replaced
  preIntro (HeldUntil e ev) = annIntro e

  ||| What a RIDER reads -- the context its subject is typed in.
  ||| [CR#608.2c] reads a card's later text against its earlier text as
  ||| one statement rather than step by step, and takes THIS sentence as
  ||| its worked example ("Destroy target creature. It can't be
  ||| regenerated."). So the rider's subject names the referent as the
  ||| clause it rides left it -- carrying that clause's own label -- and
  ||| not as a following sentence would find it: [CR#701.19a] regenerates
  ||| a PERMANENT, and a subject reachable only in a graveyard is the
  ||| wrong subject.
  ||| It is `preIntro` with the clause's label written IN PLACE, never a
  ||| re-zoning. That is the shape `settleTargets` and `defineLetter`
  ||| already have -- one mark changed on a binding the prefix already
  ||| held, nothing minted and nothing dropped -- so every gate that was
  ||| a fold over the prefix still is one, and the bare pronoun still
  ||| counts what it counted. What the mark buys is the participle
  ||| ("creatures destroyed this way") and the verb-scoped pronoun,
  ||| which ask which label acted and cannot ask it of an unmarked
  ||| binding.
  ||| The WRAPPER rows recurse with `riderIntro` and not with `preIntro`:
  ||| a rider hangs off the sentence, and the label the sentence's last
  ||| clause wrote is the label the rider names, whatever wrapper the
  ||| sentence was written under. Falling through to `preIntro` there
  ||| dropped the stamp at every seam.
  ||| `Modal` does NOT recurse: the rider follows the list, and which
  ||| mode's label it names is a question the mode's chooser answers at
  ||| resolution, not one the text fixes -- so the wrapper's own
  ||| announcement stands and the verb-scoped reads find nothing.
  ||| `If`/`OnlyIf`/`Unless` do not recurse either: their clause may not
  ||| have run, so the stamp its label would have written may not exist.
  public export
  riderIntro : {bs : Bindings} -> Effect bs -> Bindings
  riderIntro (Enact v (Move what to _)) = stampIntro (Just v) what
  riderIntro (Does s v (Move what to _)) = stampIntro (Just v) what
  riderIntro (Enact _ e) = riderIntro e
  riderIntro (Does _ _ e) = riderIntro e
  riderIntro (CantBe e _ _) = riderIntro e
  riderIntro (Reflexively body _) = riderIntro body
  riderIntro (ThisWay body _ _) = riderIntro body
  riderIntro (Sequentially es) = riderIntros es
  riderIntro e = preIntro e

  ||| A sequence's rider reads its LAST clause, on `preIntros`' model:
  ||| [CR#608.2c] reads the sentence as one statement, and the rider is
  ||| written after the last thing the sentence did.
  public export
  riderIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  riderIntros [] = bs
  riderIntros (e :: []) = riderIntro e
  riderIntros (e :: es) = riderIntros es

  ||| What a clause's phrases have named by the time the spell is cast
  ||| [CR#601.2c] — the announcement channel, distinct from `effIntro`
  ||| (what the clause did) and `preIntro` (its flat-clause approximation).
  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
  annIntro (DealDamageOwn src c to) = nomIntro to
  annIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  annIntro (Distribute v amt among) = nomIntro among
  annIntro (Fights a b) = nomIntro b
  annIntro (TurnOver n) = nomIntro n
  annIntro (SetStatus _ n) = nomIntro n
  annIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  annIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  annIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  annIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  annIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  annIntro (SkipsAllOf w _) = nomIntro w
  annIntro (GetsCounters who amt _) = amtIntro amt
  annIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  annIntro (LosesCounters who _ amt) = optAmtIntro amt
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (AttachTo _ host) = nomIntro host
  annIntro (Unattach what) = nomIntro what
  annIntro (BecomesBlocking _ what) = nomIntro what
  annIntro (StopsBlocking _ what) = nomIntro what
  annIntro (BecomesAttacking n NoDefender) = nomIntro n
  annIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  annIntro (Regenerate n) = nomIntro n
  annIntro (CantBe e _ _) = annIntro e
  annIntro (GainsDesignation n _ _ _) = nomIntro n
  annIntro (GameBecomes _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro RestartsGame = bs
  annIntro (CounterSpell what) = nomIntro what
  annIntro (CopyStack agent what times exc) = amtIntro times
  annIntro (ChooseNewTargets what) = nomIntro what
  annIntro (CopyTargets copy whom) = nomIntro whom
  annIntro (CopyCard who what times) = amtIntro times
  annIntro (Choose n _) = chosenIntro n
  annIntro (Move what to _) = nomIntro what
  annIntro (ExchangeLife parties) = nomIntro parties
  annIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  annIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  annIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  annIntro (AddMana who amt _ _) = amtIntro amt
  annIntro (Draw who amt) = amtIntro amt
  annIntro (Expose v who what) = exposedIntro what
  annIntro (Search who sc q p) =
    quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who
  annIntro (Shuffle whose) = nomIntro whose
  annIntro (FlipCoins who count) = flipScopeIntro count
  annIntro (RollDice who count _) = amtIntro count
  annIntro (ResultsTable rows) = bs
  annIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  annIntro (ShiftResult amt) = amtIntro amt
  annIntro (RollPlanarDie who count) = amtIntro count
  annIntro ChaosEnsues = bs
  annIntro (StoreResults on) = nomIntro on
  annIntro (RerollStored _ _ whose) = nomIntro whose
  annIntro (Continuously se _) = staticIntro se
  annIntro (Throughout _ se) = staticIntro se
  annIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  annIntro (GetsEmblem who _) = nomIntro who
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters q kind from) = nomIntro from
  annIntro (RemoveCountersAmong q kind among) = nomIntro among
  annIntro (MoveCounters amt kind src dst) = nomIntro dst
  annIntro (PutSameCounters src dst) = nomIntro dst
  annIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  annIntro (GiveCountersOfOwnKinds on) = nomIntro on
  annIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  annIntro (Enact v (Move what to _)) = nomIntro what
  annIntro (Enact _ e) = annIntro e
  annIntro (Does s v (Move what to _)) = nomIntro what
  annIntro (Does s v e) = annIntro e
  annIntro (Pay who c _) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (OnlyIf e c oth) = annIntro e
  annIntro (If c e oth) = bs
  annIntro (Unless e who c) = annIntro e
  annIntro (Define l amt) = defineLetter l (amtIntro amt)
  annIntro (ForEachOf _ _) = bs
  annIntro (ForEachKindOf _ _ _ _) = bs
  annIntro (Repeat _) = bs
  annIntro (Repeated n _) = amtIntro n
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = quantDelta q ++ bs
  annIntro (Delayed ev _ _ e) = bs
  annIntro (Reflexively body trig) = annIntro body
  annIntro (ThisWay body ev trig) = annIntro body
  annIntro (InsteadOf replaced repl) = annIntro replaced
  annIntro (HeldUntil e ev) = annIntro e

  public export
  annSims : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  annSims [] = bs
  annSims (e :: es) = annSims es

  ||| What a replacement clause reads back. [CR#614.6] makes the replaced
  ||| event never happen, but its announcement still names the quantity —
  ||| "deals double THAT damage instead" — so the replaced deed's own
  ||| outcome is in scope here. A simultaneous list keeps its own telescope
  ||| [CR#608.2f] and is untouched.
  ||| It admits the whole deed delta where `otherwiseCtx` filters the same
  ||| shape down to outcomes, and the asymmetry is deliberate. What the
  ||| replaced clause left is a DEFINITION of characteristics [CR#111.3],
  ||| which every printed line of this family reads back — "instead create
  ||| those tokens plus an additional Food token", "creates twice that
  ||| many of those tokens instead", 14 supported lines, all of them a
  ||| definition or a magnitude and none of them the objects. An
  ||| "otherwise" arm has no such definition to read: its branch was not
  ||| taken and described nothing.
  ||| The object read rides the SAME binding as the definition read —
  ||| `Them` and `TokenAsThose` both find it — so no determiner-aware
  ||| filter separates them: the discriminator is which reader asks, not
  ||| what the binding records. The object read is therefore an
  ||| overgeneration RECORDED at its measured count of zero printed lines
  ||| rather than pinned, per the round that settled what a pin refuses.
  public export
  replacedCtx : {bs : Bindings} -> Effect bs -> Bindings
  replacedCtx (Sequentially es) = annSeqs es
  replacedCtx (May d body did notd) = replacedCtx body
  replacedCtx (OnlyIf e c oth) = replacedCtx e
  replacedCtx (If c e oth) = bs
  replacedCtx (Unless e who c) = replacedCtx e
  replacedCtx e = deedDelta e ++ annIntro e

  ||| The context an "otherwise" arm reads. It is the same ability's
  ||| alternative and runs when the condition failed, so it is typed as if
  ||| the branch it replaces were a deed that never happened: the phrases
  ||| that branch announced at casting [CR#601.2c] and the quantity it
  ||| wrote ("that much"), but none of the referents the deed would have
  ||| made — a token it would have created names nothing here.
  public export
  otherwiseCtx : {bs : Bindings} -> Effect bs -> Bindings
  otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e

  ||| A sequence announces every deed it strings together, so a replacement
  ||| over the whole sequence reads the quantity any of them named.
  public export
  annSeqs : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  annSeqs [] = bs
  annSeqs (e :: es) = deedDelta e ++ annSeqs es

  ||| The per-deed delta a simultaneous list sums — not the whole
  ||| `effIntro` answer, since the announcement telescope already carries
  ||| each element's phrases and folding it whole would double-count
  ||| [CR#608.2f].
  public export
  deedDelta : {bs : Bindings} -> Effect bs -> List Binding
  deedDelta (DealDamage src amt to) = [outcomeB DamageDealt]
  deedDelta (DealDamageOwn src c to) = [outcomeB DamageDealt]
  deedDelta (ControllerSacrifices _) = []
  deedDelta (Distribute (DividedDamage _) amt among) = [outcomeB DamageDealt]
  deedDelta (Distribute (DistributedCounters _) amt among) = []
  deedDelta (Fights a b) = []
  deedDelta (TurnOver _) = []
  deedDelta (SetStatus _ _) = []
  deedDelta (DoesntUntapNext _ _) = []
  deedDelta (SkipsNext _ _ _) = []
  deedDelta (ExtraTurn _ _) = []
  deedDelta (AdditionalPart _ _ _ _) = []
  deedDelta (GetsAdditionalPart _ _ _) = []
  deedDelta (SkipsAllOf _ _) = []
  deedDelta (GetsCounters _ _ _) = []
  deedDelta (GetsCountersOfThoseKinds _ _) = []
  deedDelta (LosesCounters _ _ _) = []
  deedDelta (RemoveFromCombat _) = []
  deedDelta (AttachTo _ _) = []
  deedDelta (Unattach _) = []
  deedDelta (BecomesBlocking _ _) = []
  deedDelta (StopsBlocking _ _) = []
  deedDelta (BecomesAttacking _ _) = []
  deedDelta (Regenerate _) = []
  deedDelta (CantBe e _ _) = deedDelta e
  deedDelta (GainsDesignation _ _ _ _) = []
  deedDelta (GameBecomes _) = []
  deedDelta (Concludes _ _) = []
  deedDelta GameDrawn = []
  deedDelta RestartsGame = []
  deedDelta (CounterSpell _) = []
  deedDelta (CopyStack {k} {ph} agent what times exc) =
    [MkBinding TheD k (outputPlur (nounPlur what) (amtPlur times))
               (copyPayload ph (nounTy what))]
  deedDelta (ChooseNewTargets _) = []
  deedDelta (CopyTargets _ _) = []
  deedDelta (CopyCard who what times) =
    [MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
               (ObjectP (nounTy what) (nounZone what) Nothing (Just CopyOrigin) Nothing)]
  deedDelta (Choose n _) = []
  deedDelta (Move what to _) = []
  -- [CR#701.12c] settles the exchange by having EACH player "gain or
  -- lose the amount of life necessary", so both outcomes are readable and
  -- neither is the clause's alone; Mister Negative reads the loss.
  deedDelta (ExchangeLife _) = [outcomeB LifeGained, outcomeB LifeLost]
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (AddMana _ _ _ _) = [outcomeB ManaAdded]
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Search who sc q p) =
    [MkBinding AD Object (quantPlur q)
               (ObjectP (seedTy p) (searchZone sc) Nothing Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (FlipCoins _ _) = [outcomeB CoinFlipped]
  deedDelta (RollDice _ _ _) = [outcomeB RollResult]
  deedDelta (ResultsTable _) = []
  -- an ignore takes rolls away and a shift restates one; neither
  -- leaves a number the roll had not already left.
  deedDelta (IgnoreOutcomes _) = []
  deedDelta (ShiftResult _) = []
  -- [CR#706.7]: no numerical result to announce.
  deedDelta (RollPlanarDie _ _) = [outcomeB PlanarRolled]
  deedDelta ChaosEnsues = []
  deedDelta (StoreResults _) = []
  deedDelta (RerollStored _ _ _) = []
  deedDelta (Continuously se _) = []
  deedDelta (Throughout _ _) = []
  deedDelta (Create agent count spec riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin) Nothing)]
  deedDelta (GetsEmblem _ _) = []
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters q kind from) = [outcomeB CountersRemoved]
  deedDelta (RemoveCountersAmong q kind among) = [outcomeB CountersRemoved]
  deedDelta (MoveCounters amt kind src dst) = []
  deedDelta (PutSameCounters src dst) = []
  deedDelta (PutCountersOfThoseKinds amt on) = []
  deedDelta (GiveCountersOfOwnKinds on) = []
  deedDelta (DoubleCountersOfOwnKinds on) = []
  deedDelta (Enact v (Move what to _)) = []
  deedDelta (Enact _ e) = deedDelta e
  deedDelta (Does s v (Move what to _)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (Pay who c _) = []
  deedDelta (May d body did notd) = []
  deedDelta (OnlyIf e c oth) = []
  deedDelta (If c e oth) = []
  deedDelta (Unless e who c) = []
  deedDelta (Define _ _) = []
  deedDelta (ForEachOf _ _) = []
  deedDelta (ForEachKindOf _ _ _ _) = []
  deedDelta (Repeat _) = []
  deedDelta (Repeated _ _) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev _ _ e) = []
  deedDelta (Reflexively body trig) = deedDelta body
  deedDelta (ThisWay body ev trig) = deedDelta body
  deedDelta (InsteadOf replaced repl) = []
  deedDelta (HeldUntil e ev) = []

  public export
  preIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  preIntros [] = bs
  preIntros (e :: []) = preIntro e
  preIntros (e :: es) = preIntros es

  ||| The context a may's body and its declined arm are typed in: an
  ||| offered may writes its decider first [CR#118.12], so the body reads
  ||| it; the mandatory form writes none.
  public export
  mayCtx : {bs : Bindings} -> Maybe (Noun bs Player) -> Bindings
  mayCtx Nothing = bs
  mayCtx (Just d) = nomIntro d

  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bindings
  mayIntro body Nothing Nothing = effIntro body
  mayIntro body (Just did) Nothing = effIntro did
  mayIntro body Nothing (Just notd) = effIntro body
  mayIntro body (Just did) (Just notd) = effIntro body

  ||| The post-state a reflexive trigger's body is typed in: it fires
  ||| because the action was taken [CR#603.12], so it reads `effIntro`
  ||| rather than just the announcement.
  public export
  reflexCtx : {bs : Bindings} -> Effect bs -> Bindings
  reflexCtx body = settleTargets (effIntro body)

  ||| The post-state a "this way" trigger's body is typed in: the event
  ||| has happened, so it reads the event's after-discourse with the
  ||| enclosure's targets settled [CR#603.12,603.6,601.2c].
  public export
  thisWayCtx : {bs : Bindings} -> (body : Effect bs) ->
               GameEvent (effIntro body) -> Bindings
  thisWayCtx body ev = settleTargets (eventAfter ev)

  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

  ||| A repetition writes ONE instruction and a count, so a cost that
  ||| spells one is judged by that instruction. The `Sequentially`
  ||| refusal above targets a cost written as a COORDINATION -- "do A,
  ||| then B", whose components [CR#601.2h] may be paid in any order --
  ||| and "do A n times" is not one: every pass is the same instruction,
  ||| and the canonical iterated-singular expansion writes the choice and
  ||| the act as two steps of that one action. So the body's own sequence
  ||| is looked through and each step judged, which is what lets
  ||| "Discard two cards:" be a cost cards print.
  public export
  costRepeatedOk : {0 bs : Bindings} -> Effect bs -> Bool
  costRepeatedOk (Sequentially es) = costStepsOk es
  costRepeatedOk e = costActionOk e

  public export
  costStepsOk : {0 bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bool
  costStepsOk [] = True
  costStepsOk (e :: es) = costActionOk e && costStepsOk es

  public export
  simIntro : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simIntro [] = bs
  simIntro (e :: es) = deedDelta e ++ simIntro es

  public export
  simPres : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simPres [] = bs
  simPres (e :: []) = preIntro e
  simPres (e :: es) = simPres es

  public export
  data KeywordParam : Bindings -> Type where
    ParamCost : Cost [] -> KeywordParam bs
    ||| "protection from red", "affinity for artifacts", "partner with
    ||| [name]" -- and "protection from the chosen player", which is why
    ||| the payload is kind-indexed rather than fixed at `Object`. See
    ||| `qualityParamKind`: [CR#702.16a]'s quality describes an object,
    ||| [CR#702.16k]'s variant names a player, and no printed slot names
    ||| both.
    ParamQuality : {k : Kind} -> (p : Predicate bs k) ->
                   {auto 0 pk : So (qualityParamKind k)} ->
                   KeywordParam bs
    ParamSubject : {k : Kind} -> (p : Predicate [] k) -> KeywordParam bs
    ParamNumber : (amt : Amount []) ->
                  KeywordParam bs
    ||| "Equip Knight {1}", "Equip commander {3}", "Equip planeswalker
    ||| {1}": the head [CR#702.6c] writes before the cost. The predicate
    ||| is fixed at `Object` where `ParamQuality`'s is kind-indexed, and
    ||| the rule is why: [CR#702.6c] restricts "what creatures may be
    ||| chosen as legal targets", and [CR#702.6e]'s variant names a
    ||| planeswalker -- both objects, where [CR#702.16k]'s
    ||| protection-from-a-player has no counterpart here.
    ||| It does NOT compose with the attachment. [CR#702.6c]'s last
    ||| sentence -- "Additional restrictions for an equip ability don't
    ||| restrict what the Equipment may be attached to" -- makes the head
    ||| the ability's TARGET restriction and nothing the attach step
    ||| reads, which is why it rides here and not on `AttachTo`.
    ParamQualityCost : (p : Predicate bs Object) -> (cost : Cost []) ->
                       KeywordParam bs
    ||| "Suspend 4--{1}{U}", "Suspend X--{X}{W}{W}": [CR#702.62a]'s count
    ||| beside its cost. The count is the number of time counters the
    ||| card is exiled with, so it is no component of the cost paid to
    ||| exile it, and the two are written as two.
    ParamNumberCost : (amt : Amount []) -> (cost : Cost []) ->
                      KeywordParam bs

  public export
  paramShapeOf : {0 bs : Bindings} -> Maybe (KeywordParam bs) -> KeywordParamShape
  paramShapeOf Nothing = NoParam
  paramShapeOf (Just (ParamCost _)) = CostParam
  paramShapeOf (Just (ParamQuality _)) = QualityParam
  paramShapeOf (Just (ParamSubject _)) = SubjectParam
  paramShapeOf (Just (ParamNumber _)) = NumberParam
  paramShapeOf (Just (ParamQualityCost _ _)) = CompoundParam QualityHead
  paramShapeOf (Just (ParamNumberCost _ _)) = CompoundParam NumberHead

  public export
  keywordParamFits : {0 bs : Bindings} -> KeywordLabel -> Maybe (KeywordParam bs) -> Bool
  -- Knownness is folded in, so this ONE gate refuses a typo as well as a
  -- mismatched parameter: an unknown word has no parameter shape to fit.
  -- The comparison is `paramShapeFits` and not equality for one word's
  -- sake: [CR#702.6c] makes equip's head a restriction an ability MAY
  -- write, so "Equip {3}" and "Equip Knight {1}" are one row's two
  -- spellings.
  keywordParamFits k p =
    knownKeyword k && paramShapeFits (keywordParamShape k) (paramShapeOf p)

  public export
  KeywordParamFits : KeywordLabel -> Maybe (KeywordParam bs) -> Type
  KeywordParamFits {bs} k p = So (keywordParamFits k p)

  ||| One element of the ability LOSS's list: an ability the line writes
  ||| DOWN, or a keyword TERM that names a class of them.
  |||
  ||| The term arm exists because a loss can name a word the object may
  ||| hold at any parameter, and the parameter is exactly what the line
  ||| does not write. Shay Cormac's "Permanents your opponents control
  ||| lose hexproof, indestructible, protection, shroud, and ward until
  ||| end of turn" writes five words in one coordination, and two of
  ||| them -- "protection" [CR#702.16a] and "ward" [CR#702.21a] -- have
  ||| rules that write a slot after the word, so neither can be a
  ||| `KeywordAbility`: `keywordParamFits` refuses the bare spelling, and
  ||| refuses it correctly, since "lose ward" takes away every ward
  ||| ability and not one written cost's. That is `AnyKeywordIn`'s own
  ||| reading -- a quantifier over the word's parameter -- which is why
  ||| this arm is `KeywordTerm` and not a second keyword-line seat.
  ||| Tolaria's "Target creature loses banding and all 'bands with
  ||| other' abilities" is the same pairing at [CR#702.22b]'s two words,
  ||| a bare one and a quantified one in one list.
  |||
  ||| ONE list and not a second slot, because the English coordinates
  ||| the two sorts in one series and the printed order is the list's:
  ||| Shay Cormac writes a term third and fifth, between bare words.
  |||
  ||| 4 supported lines write a term at this seat (measured 2026-09-02):
  ||| Shay Cormac, Tolaria, and two more that lose "all 'bands with
  ||| other' abilities". Every other loss line writes abilities down and
  ||| takes `LostWritten`.
  public export
  data AbilityLost : Bindings -> Type where
    LostWritten : (ab : AbilityAt bs) ->
                  {auto 0 hd : So (grantableAb ab)} -> AbilityLost bs
    LostTerm : (t : KeywordTerm) ->
               {auto 0 kn : KnownKeywordTerm t} -> AbilityLost bs

  public export
  data AbilityAt : Bindings -> Type where
    KeywordAbility : (k : KeywordLabel) ->
                     (param : Maybe (KeywordParam bs)) ->
                     {auto 0 pf : KeywordParamFits k param} -> AbilityAt bs
    ||| The activation cost is written at `dropLetter X bs`, not at
    ||| `bs`: [CR#107.3k] makes an activated ability's activation-cost X
    ||| "independent of any other values of X chosen for that object or
    ||| for other instances of abilities of that object", an explicit
    ||| exception to [CR#107.3i]'s one-value-per-object rule. A card
    ||| face's telescope opens the letter its PRINTED cost announced
    ||| [CR#107.3a], and leaving that letter in scope here would let an
    ||| activated ability's body read the value the CASTER announced
    ||| where the rules give it the ACTIVATOR's. So the object's letter
    ||| is taken out and the ability's own cost opens its own; 2
    ||| supported cards write both at once (Chamber Sentry's "{X}, {T},
    ||| Remove X +1/+1 counters from this creature: It deals X damage to
    ||| any target" and Defenders of Humanity), and Riptide Replicator's
    ||| "where X is the number of charge counters on this artifact"
    ||| defines its own inside a body whose cost has none.
    ||| The letter is dropped for the COST and everything downstream of
    ||| it; the guard and the activator stay at `bs`, being the object's
    ||| own statement about when and by whom rather than part of the
    ||| cost the rule exempts.
    Activated : (cost : Cost (dropLetter X bs)) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                (window : Maybe Timing) ->
                (limit : Maybe UsageLimit) ->
                {auto 0 ul : So (untriggeredLimitOk limit)} ->
                (guard : Maybe (Condition bs)) ->
                -- WHO may activate it, where the object says otherwise.
                -- [CR#602.2] states the default and its exception in one
                -- sentence -- "Only an object's controller (or its owner,
                -- if it doesn't have a controller) can activate its
                -- activated ability unless the object specifically says
                -- otherwise" -- so the slot IS that exception and
                -- `Nothing` is the rule's own answer. Not a `Deontic`:
                -- the restriction is part of the ability's own statement,
                -- as its window and its usage limit are, and no line
                -- states it about another object's ability.
                (activator : Maybe (Noun bs Player)) ->
                AbilityAt bs
    ||| The trigger header, its concurrent clause, and its joined second
    ||| header. `while` belongs to the head event [CR#603.1,603.2] and
    ||| `joins` is a second whole header over this one effect; both sit
    ||| BEFORE the intervening slot, which [CR#603.4] places after the
    ||| whole trigger condition and which is checked again on resolution.
    Triggered : (word : TriggerWord) -> (ev : GameEvent bs) ->
                (alts : List (GameEvent bs)) ->
                (while : Maybe (Concurrent (headerCtx alts ev))) ->
                (joins : List (JoinedHeader bs)) ->
                (window : Maybe TriggerWindow) ->
                (limit : Maybe UsageLimit) ->
                (intervening :
                   Maybe (Condition (joinedCtx joins (headerCtx alts ev)))) ->
                (eff : Effect (interveningIntro intervening)) ->
                {auto 0 hn : HeaderNontarget ev} ->
                {auto 0 hs : HeaderStatus ev} ->
                {auto 0 ae : AltEvent word alts} ->
                {auto 0 cd :
                   ChapterDefaults ev alts while joins window limit intervening} ->
                AbilityAt bs
    Static : (se : StaticEffect bs) ->
             {auto 0 ut : Untargeting se} -> AbilityAt bs
    Spell : (eff : Effect bs) -> AbilityAt bs
    ||| "If this card is in your opening hand, you may begin the game
    ||| with it on the battlefield": the PREGAME opening-hand action.
    ||| 17 supported cards write this sentence word for word -- the
    ||| sixteen Leylines and Leyline Axe (measured 2026-09-02).
    |||
    ||| A FIFTH ability kind, and none of the four. [CR#103.6] gives the
    ||| action its whole procedure: "some cards allow a player to take
    ||| actions with them from their opening hand. Once the mulligan
    ||| process ... is complete, the starting player may take any such
    ||| actions in any order. Then each other player in turn order may
    ||| do the same." [CR#103.8] has the starting player take their
    ||| first turn only afterwards, so the action is taken before the
    ||| game's first turn and on no stack: not activated (no cost is
    ||| paid and no ability is put on the stack [CR#602.2]), not
    ||| triggered (no event -- no turn has begun), not static (nothing
    ||| continuous is generated), and not a spell ability, the card
    ||| never being cast [CR#113.3a]. [CR#103.6a] states the deed
    ||| itself: "if a card allows a player to begin the game with that
    ||| card on the battlefield, the player taking this action puts that
    ||| card onto the battlefield."
    |||
    ||| It carries NO slot, because the seventeen lines differ in
    ||| nothing. The "if this card is in your opening hand" clause is
    ||| the rule's own precondition restated, not a written gate; the
    ||| "you may" is [CR#103.6]'s own permission; and the deed is the
    ||| one deed these cards take. The two supported lines that write
    ||| more are RESIDUES and not slots on this row -- Gemstone
    ||| Caverns adds a second condition ("and you're not the starting
    ||| player"), an entry rider ("with a luck counter on it") and a
    ||| follow-up ("if you do, exile a card from your hand"), and
    ||| Quicksilver, Brash Blur writes its own name and "him" where
    ||| these write "this card" and "it".
    ||| -- spelling: "If this card is in your opening hand, you may
    ||| begin the game with it on the battlefield."
    MayBeginOnBattlefield : AbilityAt bs
    AlsoForKeywords : (ab : AbilityAt bs) -> (ks : List KeywordTerm) ->
                      {auto 0 ex : KeywordExtendable ab} ->
                      {auto 0 lk : KeywordListOk ab ks} -> AbilityAt bs
    ||| [CR#207.2]: an italicized word prefixes an ability of any kind and
    ||| has no special rules meaning, so it wraps `AbilityAt` — the type
    ||| enclosing every ability kind — rather than any one of them, and the
    ||| rules-facing functions below read straight through it. Homed in this
    ||| grammar by user ruling: with no rules meaning there is nothing for
    ||| the core ability model or its `.ron` kinds to carry, and the word is
    ||| printed text the spelling layer must still be able to write.
    ||| The rule puts the word at the *beginning* of the ability, so it is
    ||| the outermost wrapper: `lineKeyword` reads no keyword through it,
    ||| which keeps `AlsoForKeywords` inside the word rather than outside.
    |||
    ||| ONE row for both italicized-word rules. [CR#207.2c]'s ability word
    ||| and [CR#207.2d]'s flavor word take the same position, carry the same
    ||| absent rules meaning, and nest no further; the only difference the
    ||| rules state is which vocabulary the word is drawn from, and
    ||| `ItalicWord` carries that. A second constructor would name the two
    ||| surfaces after their phrases and duplicate this one's wrapper,
    ||| no-nesting gate and eight read-through clauses to say nothing new.
    ||| The card language keeps the phrase names: `Macros.abilityWord` and
    ||| `Macros.flavorWord`.
    ||| -- spelling: "[word] — [ab]", the word italicized.
    ItalicHead : (word : ItalicWord) -> (ab : AbilityAt bs) ->
                 {auto 0 nw : NotWordHeaded ab} -> AbilityAt bs


  ||| One word per ability: [CR#207.2] gives the word the beginning of an
  ||| ability, and a word wrapping a worded ability spells no second
  ||| beginning — the inner ability is the same ability. Mirrors the
  ||| no-nesting gate `AlsoForKeywords` gets from `lineKeyword`.
  public export
  notWordHeaded : {0 bs : Bindings} -> AbilityAt bs -> Bool
  notWordHeaded (ItalicHead _ _) = False
  notWordHeaded _ = True

  public export
  NotWordHeaded : AbilityAt bs -> Type
  NotWordHeaded {bs} ab = So (notWordHeaded ab)

  public export
  Untargeting : {bs : Bindings} -> StaticEffect bs -> Type
  Untargeting {bs} se = So (not (anyTargetedAt (staticIntro se)))

  public export
  lineKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  lineKeyword (Static se) = statKeyword se
  lineKeyword (Triggered _ _ _ _ _ _ _ _ eff) = effKeyword eff
  lineKeyword _ = Nothing

  public export
  statKeyword : {0 bs : Bindings} -> StaticEffect bs -> Maybe KeywordLabel
  statKeyword (Conditionally _ se _) = statKeyword se
  statKeyword (OnlyWhile se _ _) = statKeyword se
  statKeyword (Gains _ ab) = grantedKeyword ab
  statKeyword _ = Nothing

  public export
  effKeyword : {0 bs : Bindings} -> Effect bs -> Maybe KeywordLabel
  effKeyword (Continuously se _) = statKeyword se
  effKeyword (Throughout _ se) = statKeyword se
  effKeyword _ = Nothing

  public export
  grantedKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  grantedKeyword (KeywordAbility k Nothing) =
    if keywordParamless k then Just k else Nothing
  grantedKeyword _ = Nothing

  public export
  keywordExtendableOk : {0 bs : Bindings} -> AbilityAt bs -> Bool
  keywordExtendableOk ab = case lineKeyword ab of
    Nothing => False
    Just _ => True

  ||| The list's four demands, unchanged in content and re-read at the
  ||| TERM: nonempty, every element writable bare, no element repeated,
  ||| and none of them the base word the line already carries. The class
  ||| term did not arrive by relaxing any of them -- `keywordTermBare`
  ||| still asks `keywordParamless` of a WORD, so "ward" is refused in a
  ||| list exactly as before, and a class is admitted by its own rule.
  public export
  keywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordTerm -> Bool
  keywordListOk ab ks = case lineKeyword ab of
    Nothing => False
    Just base => not (isNil ks) && allTermsBare ks && distinctTerms ks &&
                 not (elem (TheKeyword base) ks)


  public export
  allTermsBare : List KeywordTerm -> Bool
  allTermsBare [] = True
  allTermsBare (k :: ks) = keywordTermBare k && allTermsBare ks

  public export
  distinctTerms : List KeywordTerm -> Bool
  distinctTerms [] = True
  distinctTerms (k :: ks) = not (elem k ks) && distinctTerms ks

  public export
  KeywordExtendable : {0 bs : Bindings} -> AbilityAt bs -> Type
  KeywordExtendable {bs} ab = So (keywordExtendableOk ab)

  public export
  KeywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordTerm -> Type
  KeywordListOk {bs} ab ks = So (keywordListOk ab ks)

  ||| [CR#613.1f] applies ability-adding effects over abilities as such,
  ||| and [CR#113.3] gives four kinds of them; neither rule restricts
  ||| which kind an effect may add, nor says how the added text is
  ||| written, so whether the grant puts the ability in quotation marks
  ||| is rendering and no cell here indexes on it. The two Falses
  ||| are structural, not measured: a `Spell` line is [CR#113.3a]'s spell
  ||| ability, followed only while an instant or sorcery spell resolves,
  ||| so an object handed one has nothing to follow; and
  ||| `AlsoForKeywords` is not an ability at all but the spelling that
  ||| extends one line's keyword list, so a grant naming it would name no
  ||| second ability.
  public export
  grantableAb : {0 bs : Bindings} -> AbilityAt bs -> Bool
  grantableAb (KeywordAbility _ _) = True
  grantableAb (Activated _ _ _ _ _ _) = True
  grantableAb (Triggered _ _ _ _ _ _ _ _ _) = True
  grantableAb (Static _) = True
  grantableAb (Spell _) = False
  -- [CR#103.6] gives the action to a card in a player's OPENING HAND,
  -- which nothing on the battlefield can be granted into.
  grantableAb MayBeginOnBattlefield = False
  grantableAb (AlsoForKeywords _ _) = False
  grantableAb (ItalicHead _ ab) = grantableAb ab

  public export
  Grantable : AbilityAt bs -> Type
  Grantable {bs} ab = So (grantableAb ab)

  public export
  emblemAbilityOk : AbilityAt [] -> Bool
  emblemAbilityOk (KeywordAbility _ _) = False
  emblemAbilityOk (Activated _ _ _ _ _ _) = True
  emblemAbilityOk (Triggered _ _ _ _ _ _ _ _ _) = True
  emblemAbilityOk (Static _) = True
  emblemAbilityOk (AlsoForKeywords _ _) = False
  emblemAbilityOk (ItalicHead _ ab) = emblemAbilityOk ab
  emblemAbilityOk (Spell _) = False
  emblemAbilityOk MayBeginOnBattlefield = False

  public export
  emblemAbilitiesAll : List (AbilityAt []) -> Bool
  emblemAbilitiesAll [] = True
  emblemAbilitiesAll (a :: as) = emblemAbilityOk a && emblemAbilitiesAll as

  public export
  emblemAbilitiesOk : List (AbilityAt []) -> Bool
  emblemAbilitiesOk [] = False
  emblemAbilitiesOk (a :: as) = emblemAbilityOk a && emblemAbilitiesAll as

  public export
  EmblemAbilities : List (AbilityAt []) -> Type
  EmblemAbilities abl = So (emblemAbilitiesOk abl)

  public export
  abilitiesGrantable : List (AbilityAt []) -> Bool
  abilitiesGrantable [] = True
  abilitiesGrantable (a :: as) = grantableAb a && abilitiesGrantable as

  ||| The same question at a list typed in a live context, which the
  ||| ability LOSS asks of its payload: an object can be made to lose
  ||| only what it could have had.
  public export
  abilitiesHoldable : {0 bs : Bindings} -> List (AbilityAt bs) -> Bool
  abilitiesHoldable [] = True
  abilitiesHoldable (a :: as) = grantableAb a && abilitiesHoldable as

  public export
  tokenAbilitiesOk : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenAbilitiesOk t = abilitiesGrantable t.abilities

  public export
  TokenAbilities : TokenChars bs -> Type
  TokenAbilities {bs} t = So (tokenAbilitiesOk t)

  public export
  predRegime : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> Maybe StackRegime
  predRegime (CastBy _) = Just AtCasting
  predRegime (NthCastBy _ _ _) = Just AtCasting
  predRegime (CastFrom _) = Just AtCasting
  predRegime WasCast = Just AtCasting
  predRegime (ControlledBy _) = Just AtResolution
  predRegime (And ps) = predRegimeAll ps
  predRegime (Or ps) = predRegimeAll ps
  predRegime _ = Nothing

  public export
  predRegimeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Maybe StackRegime
  predRegimeAll [] = Nothing
  predRegimeAll (p :: ps) = case predRegime p of
    Just r => Just r
    Nothing => predRegimeAll ps

  public export
  nounRegime : {bs : Bindings} -> Noun bs Object -> Maybe StackRegime
  nounRegime (AllOf p) = predRegime p
  nounRegime (Indefinite _ p) = predRegime p
  nounRegime (Definite p) = predRegime p
  nounRegime (Each p) = predRegime p
  nounRegime (TargetGroup _ p) = predRegime p
  nounRegime (CountedGroup _ _ p) = predRegime p
  nounRegime (NamesAgree _ grp) = nounRegime grp
  nounRegime _ = Nothing

  public export
  abRegime : {0 bs : Bindings} -> AbilityAt bs -> Maybe StackRegime
  abRegime (KeywordAbility k _) = keywordStackRegime k
  abRegime (Activated _ _ _ _ _ _) = Nothing
  abRegime (Triggered _ _ _ _ _ _ _ _ _) = Nothing
  abRegime (Static _) = Nothing
  abRegime (AlsoForKeywords ab _) = abRegime ab
  abRegime (ItalicHead _ ab) = abRegime ab
  abRegime (Spell _) = Nothing
  abRegime MayBeginOnBattlefield = Nothing

  public export
  castingOnly : Maybe StackRegime -> Bool
  castingOnly (Just AtCasting) = True
  castingOnly _ = False

  public export
  regimeMatches : Maybe StackRegime -> Maybe StackRegime -> Bool
  regimeMatches (Just a) (Just b) = a == b
  regimeMatches _ _ = False

  public export
  ||| [CR#113.6b]: an ability that states which zones it functions in
  ||| functions from those zones, so a grant may name a subject in any
  ||| zone. The one refusal off the stack is [CR#113.6e]'s: an ability
  ||| granting an object another ability that modifies how that object is
  ||| played or cast functions only on the stack.
  grantSubjectOk : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Bool
  grantSubjectOk ab n = grantSubjectFits (nounZone n) (nounRegime n) ab

  ||| `grantSubjectOk` asked of the two facts it reads off the subject,
  ||| so a coordination whose parts are typed in a LATER context than
  ||| its subject can still ask it. Same rule, same two lines.
  public export
  grantSubjectFits : {0 bs : Bindings} -> Maybe Zone -> Maybe StackRegime ->
                     AbilityAt bs -> Bool
  grantSubjectFits zn reg ab =
    if onStackZone zn
      then regimeMatches (abRegime ab) reg
      else not (castingOnly (abRegime ab))

  public export
  GrantSubject : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Type
  GrantSubject {bs} ab n = So (grantSubjectOk ab n)

  ||| The choice an EFFECT binds, for the abilities after it to read.
  ||| `staticChoiceDelta`'s twin at the chooser positions the as-enters
  ||| rider does not reach -- a combat or upkeep trigger, an activated
  ||| ability, a Saga chapter -- and it is the CONTAINER those positions
  ||| wanted: the `Choose` clause already spells the choice and already
  ||| leaves the binding, and what was missing was the export across the
  ||| ability boundary that [CR#607.2d] links over. It runs at every
  ||| ability with a body, the spell ability included, because
  ||| [CR#607.2d] links two abilities printed on one object and says
  ||| nothing about their kinds; a keyword ability has no body to look
  ||| at.
  ||| Only the composition rows a printed chooser sits inside recurse:
  ||| a coordination ("Put a sleight counter on this Aura and choose a
  ||| color", Chromatic Armor) and an optional one ("you may choose a
  ||| number between 0 and 7", Shapeshifter), the latter on
  ||| `mayIntro`'s standing precedent that a may-body's announcements
  ||| are what the clause leaves. A chooser under a CONDITION exports
  ||| nothing, which is undergeneration and not a refusal; no supported
  ||| line writes one.
  ||| A PLURAL choice exports nothing either: `countChoice` counts
  ||| singular bindings, the plural chooser's reads are a separate and
  ||| declined cell, and exporting a singular binding for "choose two
  ||| colors" would let the singular read name one of two.
  public export
  effChoiceDelta : {0 bs : Bindings} -> Effect bs -> List Binding
  effChoiceDelta (Choose {k} (Indefinite _ _) _) = choiceDeltaAt k
  effChoiceDelta (Choose _ _) = []
  effChoiceDelta (Sequentially es) = effsChoiceDelta es
  effChoiceDelta (May _ body _ _) = effChoiceDelta body
  effChoiceDelta _ = []

  public export
  effsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                    Effects n bs -> List Binding
  effsChoiceDelta [] = []
  effsChoiceDelta (e :: es) = effsChoiceDelta es ++ effChoiceDelta e

  public export
  abIntro : {bs : Bindings} -> AbilityAt bs -> Bindings
  abIntro (KeywordAbility _ _) = bs
  abIntro (Activated _ eff _ _ _ _) = effChoiceDelta eff ++ bs
  abIntro (Triggered _ _ _ _ _ _ _ _ eff) = effChoiceDelta eff ++ bs
  abIntro (Static se) = staticChoiceIntro se
  abIntro (AlsoForKeywords ab _) = abIntro ab
  abIntro (ItalicHead _ ab) = abIntro ab
  abIntro (Spell eff) = effChoiceDelta eff ++ bs
  abIntro MayBeginOnBattlefield = bs

  ||| The letter a granted ability's NUMBER parameter leaves open. 4
  ||| supported lines write one: Ulamog, the Defiler's annihilator X,
  ||| Fumiko the Lowblood's bushido X, and mobilize X on Avenger of the
  ||| Fallen and Infantry Shield. Only the number parameter can carry a
  ||| letter -- a cost's {X} is announced as the spell is cast
  ||| [CR#107.3a] and is not this text's to define.
  ||| The COMPOUND's count is not this slot, and the same sentence says
  ||| why. 5 supported lines write "Suspend X--[cost]" (measured
  ||| 2026-09-02) and every one of them writes {X} in the cost beside it,
  ||| so the letter is the COST's announcement [CR#107.3a] and the count
  ||| reads the value it was paid at -- a read, never a definition. So
  ||| the compound arms open no letter and the catch-all answers them.
  ||| Monstrosity's five "{X}{X}{G}: Monstrosity X" lines are NOT this:
  ||| there the letter is the activation cost's, and [CR#701.37c] makes
  ||| the permanent's other abilities read the value X had as it became
  ||| monstrous -- a linked value, never a fresh read, and no
  ||| where-clause is written.
  public export
  abLetterDelta : {0 bs : Bindings} -> AbilityAt bs -> List Binding
  abLetterDelta (KeywordAbility _ (Just (ParamNumber (LetterVal l)))) = [letterB l]
  abLetterDelta _ = []


  namespace Coord
    public export
    data StaticParts : Nat -> Bindings -> Type where
      Nil : StaticParts Z bs
      (::) : (se : StaticEffect bs) -> {auto 0 nc : NotCoord se} ->
             StaticParts n (staticIntro se) -> StaticParts (S n) bs

  namespace Shared
    ||| One statement's VERB PHRASE with its subject left out: what a
    ||| shared-subject coordination coordinates. Each arm is the slot
    ||| list of the `StaticEffect` row that spells it, minus the subject
    ||| the coordination writes once -- so no arm names a referent, and
    ||| the elided subject of English's second conjunct is elided here
    ||| too rather than pronominalised.
    ||| Two arms, which are the two the corpus coordinates: the P/T
    ||| shift (`Gets`) and the ability grant (`Gains`, which spells both
    ||| "gains" and "has"). The vocabulary is OPEN in the same sense
    ||| `VerbLabel` is -- an arm is a row here and a clause in `vpOk`,
    ||| and adding one obliges nothing else -- and the statements the
    ||| corpus coordinates by writing the subject twice, or by reading it
    ||| back, keep writing `AndAlso`.
    public export
    data SubjectVP : Bindings -> Type where
      ||| `Gets`' slots: "… gets +4/+4 …".
      VPGets : (pow : PtShift bs) -> (tou : PtShift (shiftIntro pow)) ->
               (span : Maybe (Duration (shiftIntro tou))) -> SubjectVP bs
      ||| `Gains`' slot: "… and gains trample", "… and has flying".
      VPGains : (ab : AbilityAt bs) -> (span : Maybe (Duration bs)) ->
                SubjectVP bs
      ||| `Deontic`'s slots minus the subject: "… and can't be blocked
      ||| this turn", "… and can attack this turn", "… and must be
      ||| blocked this turn". The third arm, and the one the PER-PART
      ||| SPAN was bought with: 51 supported lines coordinate a grant
      ||| written "until end of turn" with a restriction written "this
      ||| turn" (measured 2026-09-02, 49 in that order and 2 reversed),
      ||| and those are two different `Duration` values, so the one
      ||| envelope `Continuously` puts over the whole coordination cannot
      ||| write them. Distortion Strike, Taigam's Strike, Teleportal and
      ||| Marchesa's Smuggler are the family's plainest members.
      ||| No patient and no counterfactual: the arm is the PLAIN
      ||| restriction the seat writes, and the four lines that name a
      ||| blocker class there ("can't be blocked by Walls this turn")
      ||| keep writing `AndAlso`.
      VPDeontic : (c : Compulsion bs) -> (deeds : Deeds) -> (role : Role) ->
                  (span : Maybe (Duration bs)) -> SubjectVP bs

    ||| The parts, threaded left to right [CR#608.2c] on `StaticParts`'
    ||| model: each typed in the previous one's output.
    public export
    data SubjectVPs : Nat -> Bindings -> Type where
      Nil : SubjectVPs Z bs
      (::) : (vp : SubjectVP bs) -> SubjectVPs n (vpIntro vp) ->
             SubjectVPs (S n) bs

  ||| What one verb phrase announces: its own amounts, and nothing about
  ||| the subject -- which the coordination announced once, before any
  ||| part was typed.
  public export
  vpIntro : {bs : Bindings} -> SubjectVP bs -> Bindings
  -- a span announces into nothing at this seat, exactly as
  -- `Continuously`'s does: only a span written BEFORE its statement is
  -- in a position to be read, and every arm's is written after.
  vpIntro (VPGets pow tou _) = shiftDelta tou ++ shiftDelta pow ++ bs
  vpIntro (VPGains _ _) = bs
  vpIntro (VPDeontic _ _ _ _) = bs

  public export
  vpsIntro : {0 k : Nat} -> {bs : Bindings} -> SubjectVPs k bs -> Bindings
  vpsIntro [] = bs
  vpsIntro (vp :: rest) = vpsIntro rest

  ||| `Deontic`'s subject demands as a Bool, asked of the shared
  ||| subject's own three facts rather than of a noun: the coordination
  ||| writes the subject once, so no arm has one to ask. Same content as
  ||| `counterpartFits`, at the seat where the noun is not in hand.
  public export
  deedSubjectFits : Maybe Zone -> Maybe CardType -> Deeds -> Role -> Bool
  deedSubjectFits zn ty ds r =
    all (\d => deedKindOk d r Object) ds &&
    (case ty of
       Just t => all (\d => deedTypeOk d r t) ds
       Nothing => all (\d => deedBareOk d r) ds) &&
    zoneFits zn (deedsZone ds r)

  ||| A written span is `SpanOk`'s own demand, asked at the arm: an
  ||| unwritten one is the elided form and the envelope's business.
  public export
  vpSpanOk : {0 bs : Bindings} -> Maybe (Duration bs) -> Bool
  vpSpanOk Nothing = True
  vpSpanOk (Just d) = durationOk d

  ||| One part's own obligation, asked of the SHARED subject's facts:
  ||| `Gets` wants a battlefield subject, `Gains` wants a subject the
  ||| ability may be granted to [CR#113.6e] and an ability that may be
  ||| granted at all, and the restriction wants what `Deontic` wants of
  ||| its subject. Every arm's own span is asked `durationOk`.
  public export
  vpOk : {0 bs : Bindings} -> Maybe Zone -> Maybe StackRegime ->
         Maybe CardType -> SubjectVP bs -> Bool
  vpOk zn reg ty (VPGets _ _ sp) = zoneFits zn (Just Battlefield) && vpSpanOk sp
  vpOk zn reg ty (VPGains ab sp) =
    grantSubjectFits zn reg ab && grantableAb ab && vpSpanOk sp
  vpOk zn reg ty (VPDeontic _ ds r sp) =
    not (isNil ds) && knownDeeds ds && deedSubjectFits zn ty ds r && vpSpanOk sp

  public export
  vpsOk : {0 k : Nat} -> {0 bs : Bindings} -> Maybe Zone ->
          Maybe StackRegime -> Maybe CardType -> SubjectVPs k bs -> Bool
  vpsOk zn reg ty [] = True
  vpsOk zn reg ty (vp :: rest) = vpOk zn reg ty vp && vpsOk zn reg ty rest

  namespace Paid
    public export
    data CostSeq : Nat -> Bindings -> Type where
      Nil : CostSeq Z bs
      (::) : (c : Cost bs) -> {auto 0 nc : NotCompound c} ->
             CostSeq n (costIntro c) -> CostSeq (S n) bs

  namespace Text
    public export
    data AbilitySeq : Bindings -> Type where
      Nil : AbilitySeq bs
      (::) : (ab : AbilityAt bs) -> AbilitySeq (abIntro ab) -> AbilitySeq bs

  public export
  costsIntro : {bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bindings
  costsIntro [] = bs
  costsIntro (c :: cs) = costsIntro cs

  ||| The choice a COST binds, for the ability after it to read --
  ||| `effChoiceDelta` reached through the cost's action, and nothing
  ||| else: a mana or symbol cost announces no value.
  public export
  costChoiceDelta : {0 bs : Bindings} -> Cost bs -> List Binding
  costChoiceDelta (Do e) = effChoiceDelta e
  costChoiceDelta (Compound cs) = costsChoiceDelta cs
  costChoiceDelta _ = []

  public export
  costsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                     CostSeq n bs -> List Binding
  costsChoiceDelta [] = []
  costsChoiceDelta (c :: cs) = costsChoiceDelta cs ++ costChoiceDelta c

  public export
  partsIntro : {0 n : Nat} -> {bs : Bindings} -> StaticParts n bs -> Bindings
  partsIntro [] = bs
  partsIntro (se :: rest) = partsIntro rest

  ||| Later parts bind NEARER, on `staticChoiceIntro`'s own order: a
  ||| coordination is read left to right [CR#608.2c] and the bindings
  ||| are nearest-first.
  public export
  partsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                     StaticParts n bs -> List Binding
  partsChoiceDelta [] = []
  partsChoiceDelta (se :: rest) = partsChoiceDelta rest ++ staticChoiceDelta se



  public export
  partsClauseOk : {0 n : Nat} -> {0 bs : Bindings} -> StaticParts n bs -> Bool
  partsClauseOk [] = True
  partsClauseOk (se :: rest) = clauseStaticOk se && partsClauseOk rest

  ||| Which continuous effects a resolving clause can establish. A
  ||| characteristic-defining ability is printed on the card it affects
  ||| [CR#604.3a], so no resolution establishes one. An alternative cost
  ||| modifies what one particular object costs to cast [CR#113.6d], and
  ||| `AltCost` names no object for the clause to price.
  public export
  clauseStaticOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
  clauseStaticOk (DefinesPt _ _ _) = False
  clauseStaticOk (AltCost _) = False
  clauseStaticOk (AndAlso parts) = partsClauseOk parts
  clauseStaticOk _ = True

  public export
  ClauseStatic : StaticEffect bs -> Type
  ClauseStatic {bs} se = So (clauseStaticOk se)

  public export
  selfTapPayment : {0 bs : Bindings} -> Cost bs -> Bool
  selfTapPayment (Mana _) = False
  selfTapPayment (ScaledMana _ _) = False
  selfTapPayment TapSymbol = True
  selfTapPayment UntapSymbol = True
  selfTapPayment (LoyaltySymbol _) = False
  selfTapPayment (Do _) = False
  selfTapPayment (Compound _) = False
  selfTapPayment (EitherCost l r) = selfTapPayment l || selfTapPayment r
  selfTapPayment ItsManaCost = False

  public export
  selfTapCount : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Nat
  selfTapCount [] = Z
  selfTapCount (c :: cs) =
    (if selfTapPayment c then S Z else Z) + selfTapCount cs

  public export
  selfTapOnce : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  selfTapOnce cs = lte (selfTapCount cs) 1

  public export
  costTapOnce : {0 bs : Bindings} -> Cost bs -> Bool
  costTapOnce (Mana _) = True
  costTapOnce (ScaledMana _ _) = True
  costTapOnce TapSymbol = True
  costTapOnce UntapSymbol = True
  costTapOnce (LoyaltySymbol _) = True
  costTapOnce (Do _) = True
  costTapOnce (Compound cs) = selfTapOnce cs
  costTapOnce (EitherCost l r) = costTapOnce l && costTapOnce r
  costTapOnce ItsManaCost = True

  public export
  CostTapOnce : Cost bs -> Type
  CostTapOnce {bs} c = So (costTapOnce c)

  public export
  costPaidByYou : {0 bs : Bindings} -> Cost bs -> Bool
  costPaidByYou (Mana _) = True
  costPaidByYou (ScaledMana _ _) = True
  costPaidByYou TapSymbol = True
  costPaidByYou UntapSymbol = True
  costPaidByYou (LoyaltySymbol _) = True
  costPaidByYou (Do (ChangeLife who _)) = nounIsYou who
  costPaidByYou (Do (Does subj _ _)) = nounIsYou subj
  costPaidByYou (Do _) = True
  costPaidByYou (Compound cs) = costsPaidByYou cs
  costPaidByYou (EitherCost l r) = costPaidByYou l && costPaidByYou r
  costPaidByYou ItsManaCost = True

  public export
  costsPaidByYou : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsPaidByYou [] = True
  costsPaidByYou (c :: cs) = costPaidByYou c && costsPaidByYou cs

  public export
  CostPaidByYou : Cost bs -> Type
  CostPaidByYou {bs} c = So (costPaidByYou c)

  public export
  payAgreesOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
                Noun bs Player -> Cost cs -> Bool
  payAgreesOk who c = not (nounIsYou who) || costPaidByYou c

  public export
  PayAgrees : Noun bs Player -> Cost cs -> Type
  PayAgrees {bs} {cs} who c = So (payAgreesOk who c)

  public export
  costOffBattlefield : {0 bs : Bindings} -> Cost bs -> Bool
  costOffBattlefield (Mana _) = True
  costOffBattlefield (ScaledMana _ _) = True
  costOffBattlefield TapSymbol = False
  costOffBattlefield UntapSymbol = False
  costOffBattlefield (LoyaltySymbol _) = False
  costOffBattlefield (Do _) = True
  costOffBattlefield (Compound cs) = costsOffBattlefield cs
  costOffBattlefield (EitherCost l r) = costOffBattlefield l && costOffBattlefield r
  costOffBattlefield ItsManaCost = True

  public export
  costsOffBattlefield : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsOffBattlefield [] = True
  costsOffBattlefield (c :: cs) = costOffBattlefield c && costsOffBattlefield cs

  public export
  data AltPayment : {0 bs : Bindings} -> Maybe (Cost bs) -> Type where
    NoAltPayment : AltPayment Nothing
    AltPaymentWritten : {0 c : Cost bs} ->
                        {auto 0 ok : So (costOffBattlefield c)} ->
                        AltPayment (Just c)

  ||| The same demand at `AddedCost`, minus the unwritten arm.
  ||| [CR#118.9]'s alternative cost may be nothing at all -- "without
  ||| paying its mana cost" is the whole substitution -- but [CR#118.8]
  ||| makes an additional cost one "listed in a spell's rules text", and
  ||| a listed cost of nothing is no cost. So this row's cost is always
  ||| written, and it is paid while casting, off the battlefield.
  public export
  data AddedPayment : {0 bs : Bindings} -> Cost bs -> Type where
    AddedPaymentWritten : {0 c : Cost bs} ->
                          {auto 0 ok : So (costOffBattlefield c)} ->
                          AddedPayment c

public export
Ability : Type
Ability = AbilityAt []
