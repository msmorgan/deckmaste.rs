||| The macro layer: one definition per English phrase shape, mirroring
||| Experimental's Macros-over-Semantics split.
module Experimental.Macros

import public Experimental

%default total


public export
exactly : Nat -> Quantity bs
exactly n = Range (Just n) (Just n)

public export
upTo : Nat -> Quantity bs
upTo n = Range Nothing (Just n)

public export
anyNumber : Quantity bs
anyNumber = Range Nothing Nothing

public export
atLeast : Nat -> Quantity bs
atLeast n = Range (Just n) Nothing

public export
oneOrBoth : Quantity bs
oneOrBoth = Range (Just 1) (Just 2)

public export
oneThrough : Nat -> Quantity bs
oneThrough n = Range (Just 1) (Just n)

public export
target : (p : Predicate bs k) -> {auto tk : Targetable k} -> Noun bs k
target p = TargetGroup (exactly 1) p {tk}

||| "any target" [CR#115.4]: the class word, written out as the rule's own
||| list — a creature, a player, a planeswalker or a battle. A function, not
||| a row: the marking is spelling, and the expansion is what the rule says.
||| -- spelling: "any target" — the whole target phrase in two words, never
||| the four nouns coordinated.
public export
anyTarget : Predicate bs (Object \/ Player)
anyTarget = Joined (Or [HasType Creature, HasType Planeswalker, HasType Battle])
                   AnyPlayer

||| "target creature or player", "target permanent or player", "target
||| player or planeswalker": the cross-kind head, over any two descriptions.
||| [CR#115.1] is the rule that admits it — a spell's targets are objects
||| and/or players — not [CR#115.4], which covers the class word written
||| INSTEAD of "target [something]". The arguments are player-then-object, as the retired row
||| wrote them; the printed order of the two halves is the spelling layer's.
||| -- spelling: the two halves joined by "or", in whichever order the card
||| prints them.
public export
kindJoin : (who : Predicate bs Player) -> (what : Predicate bs Object) ->
           Predicate bs (Object \/ Player)
kindJoin who what = Joined what who

||| "you and permanents you control" [CR#109.5]: the mixed group. It binds
||| nothing jointly — see `Both` — so no clause reads the pair back.
||| -- spelling: "you and [phrase]", the player half first.
public export
youAnd : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youAnd n = Both You n

||| "that creature or player", "that permanent or player": the demonstrative
||| that reads a joined mention back. The word is `JoinW`, whose kind is now
||| the join, so the read lands at the antecedent's own kind.
||| -- spelling: "that " then the two halves as the antecedent spelled them.
public export
thatJoin : {auto 0 ok : countWord JoinW bs = 1} -> Noun bs (Object \/ Player)
thatJoin = That JoinW {ok}

||| "that player or that planeswalker's controller", "that player or that
||| permanent's controller": the SPLIT read of a union mention, naming each
||| half in turn where `thatJoin` echoes the whole. A union target is an
||| object and/or a player [CR#115.1], so the player arm reads the player
||| half directly and the class arm reads the object half through its
||| controller; between them the two arms name a player whichever half the
||| target turned out to be. The class arm is written out by the caller
||| because the card writes its own echo word there.
||| -- spelling: "that player or " then the class arm's possessive.
public export
thatSplitController : (cls : Noun bs Object) ->
                      {auto 0 one : nounPlur cls = OneOf} ->
                      {auto 0 pk : countUnionHalf PlayerW bs = 1} ->
                      Noun bs Player
thatSplitController cls = EitherOf (ThatHalf PlayerW {ok = pk}) (ControllerOf cls {one})

||| "that player or that planeswalker's controller": the split read over a
||| union head whose object half named its card type, so the class arm
||| echoes that word.
||| -- spelling: "that player or that planeswalker's controller".
public export
splitOverPlaneswalker : {bs : Bindings} ->
                        {auto 0 ck : countUnionHalf (TypeW Planeswalker) bs = 1} ->
                        {auto 0 pk : countUnionHalf PlayerW bs = 1} ->
                        Noun bs Player
splitOverPlaneswalker = thatSplitController (ThatHalf (TypeW Planeswalker) {ok = ck}) {pk}

||| "that player or that permanent's controller": the split read over the
||| class word [CR#115.4], whose object half named no card type, so the
||| class arm writes the generic word instead of an echo.
||| -- spelling: "that player or that permanent's controller".
public export
splitOverPermanent : {bs : Bindings} ->
                     {auto 0 ck : countUnionHalf PermanentW bs = 1} ->
                     {auto 0 pk : countUnionHalf PlayerW bs = 1} ->
                     Noun bs Player
splitOverPermanent = thatSplitController (ThatHalf PermanentW {ok = ck}) {pk}


public export
battlefieldZ : ZoneExpr bs
battlefieldZ = ZoneAt Battlefield Bare

public export
exileZ : ZoneExpr bs
exileZ = ZoneAt Exile Bare

public export
handZ : ZoneExpr bs
handZ = ZoneAt Hand Bare

public export
stackZ : ZoneExpr bs
stackZ = ZoneAt Stack Bare

||| The command zone is shared [CR#400.1], so it takes no possessor and
||| the printed lines write it bare -- "the command zone", never "your".
public export
commandZ : ZoneExpr bs
commandZ = ZoneAt Command Bare

public export
spell : Predicate bs Object
spell = InZone stackZ

||| A one-faced card, spelled as its face's six printed parts.
public export
cardOf : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
         (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
         (box : Maybe PrintedBox) ->
         {auto 0 ln : CardLine line} ->
         {auto 0 sp : CardSupers supers} ->
         {auto 0 tx : CardText line text} ->
         {auto 0 ch : CardChapters line text} ->
         {auto 0 bx : CardBox line text box} ->
         {auto 0 mc : CardCost line cost} ->
         Card
cardOf name cost supers line text box =
  SingleFaced (MkFace name cost supers line text box)
              {fl = MkFaceLaws {ln} {sp} {tx} {ch} {bx} {mc}}

||| The lower-right box a creature card writes [CR#208.1], from its two plain
||| numbers.
public export
printedBox : Maybe (Integer, Integer) -> Maybe PrintedBox
printedBox Nothing = Nothing
printedBox (Just (p, t)) = Just (PtBox (PrintedNum p) (PrintedNum t))

||| The lower-right box a planeswalker card writes [CR#209.1].
public export
loyaltyBox : Integer -> Maybe PrintedBox
loyaltyBox n = Just (LoyaltyBox (PrintedNum n))

||| The lower-right box a battle card writes [CR#210.1].
public export
defenseBox : Integer -> Maybe PrintedBox
defenseBox n = Just (DefenseBox (PrintedNum n))

public export
card : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
       (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
       (stats : Maybe (Integer, Integer)) ->
       {auto 0 ln : CardLine line} ->
       {auto 0 sp : CardSupers supers} ->
       {auto 0 tx : CardText line text} ->
       {auto 0 ch : CardChapters line text} ->
       {auto 0 bx : CardBox line text (printedBox stats)} ->
       {auto 0 mc : CardCost line cost} ->
       Card
card name cost supers line text stats =
  cardOf name cost supers line text (printedBox stats) {ln} {sp} {tx} {ch} {bx} {mc}

public export
counterSpell : {k : Kind} -> (n : Noun bs k) ->
               {auto 0 ct : Counterable n} -> Effect bs
counterSpell n = CounterSpell n {ct}

public export
graveyardZ : ZoneExpr bs
graveyardZ = ZoneAt Graveyard Bare

public export
handOf : (n : Noun bs Player) -> ZoneExpr bs
handOf n = ZoneAt Hand (OwnedBy n {ps = HandIsOwned})

public export
graveyardOf : (n : Noun bs Player) -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (OwnedBy n {ps = GraveyardIsOwned})


public export
thisCreature : Noun bs Object
thisCreature = AsType Creature This Nothing

public export
thisArtifact : Noun bs Object
thisArtifact = AsType Artifact This Nothing

public export
thisEnchantment : Noun bs Object
thisEnchantment = AsType Enchantment This Nothing

public export
thisLand : Noun bs Object
thisLand = AsType Land This Nothing

||| "this Planeswalker": the self-reference read at a card type.
public export
thisPlaneswalker : Noun bs Object
thisPlaneswalker = AsType Planeswalker This Nothing

public export
thisAura : Noun bs Object
thisAura = AsType Enchantment This (Just (enchantmentType "Aura"))

public export
thisEquipment : Noun bs Object
thisEquipment = AsType Artifact This (Just (artifactType "Equipment"))

public export
thisSaga : Noun bs Object
thisSaga = AsType Enchantment This (Just (enchantmentType "Saga"))

public export
thisClass : Noun bs Object
thisClass = AsType Enchantment This (Just (enchantmentType "Class"))

public export
thisCase : Noun bs Object
thisCase = AsType Enchantment This (Just (enchantmentType "Case"))

public export
thisRoom : Noun bs Object
thisRoom = AsType Enchantment This (Just (enchantmentType "Room"))

public export
thisVehicle : Noun bs Object
thisVehicle = AsType Artifact This (Just (artifactType "Vehicle"))

public export
thisSpacecraft : Noun bs Object
thisSpacecraft = AsType Artifact This (Just (artifactType "Spacecraft"))

public export
exiledWithThisArtifact : Predicate bs Object
exiledWithThisArtifact = ExiledWith thisArtifact

public export
creature : Predicate bs Object
creature = HasType Creature

public export
artifact : Predicate bs Object
artifact = HasType Artifact

public export
source : Predicate bs Object
source = IsSource

public export
enchantment : Predicate bs Object
enchantment = HasType Enchantment

public export
land : Predicate bs Object
land = HasType Land

public export
instant : Predicate bs Object
instant = HasType Instant

public export
sorcery : Predicate bs Object
sorcery = HasType Sorcery

public export
instantOrSorcery : Predicate bs Object
instantOrSorcery = Or [instant, sorcery]

public export
creatureYouControl : Predicate bs Object
creatureYouControl = And [creature, ControlledBy You]

public export
creatureYouDontControl : Predicate bs Object
creatureYouDontControl = And [creature, Not (ControlledBy You)]

public export
creatureYourOpponentsControl : Predicate bs Object
creatureYourOpponentsControl = And [creature, ControlledBy (PlayerGroup YourOpponents)]

public export
tapped : Predicate bs Object
tapped = HasStatus Tapped

public export
untapped : Predicate bs Object
untapped = HasStatus Untapped

public export
faceDown : Predicate bs Object
faceDown = HasStatus FaceDown

public export
nontoken : Predicate bs Object
nontoken = Not IsToken


public export
a : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
    Noun bs k
a p = Indefinite Unmarked p {ph}

public export
aTheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
               {auto 0 ch : countChoosers bs = 1} ->
               Noun bs k
aTheirChoice p = Indefinite (TheirChoice {ch}) p {ph}

public export
aYourChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
              Noun bs k
aYourChoice p = Indefinite YourChoice p {ph}

||| "… with different names": the group-level constraint's negative pole on
||| a counted mention [CR#201.2b].
public export
withDifferentNames : (grp : Noun bs Object) ->
                     {auto 0 cm : CountedMention grp} ->
                     {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
withDifferentNames grp = NamesAgree DifferentNames grp {cm} {pl}

||| "… with the same name as one another", and the elliptical "… with the
||| same name" that writes the same relation without the reciprocal: the
||| positive pole [CR#201.2a].
public export
withTheSameName : (grp : Noun bs Object) ->
                  {auto 0 cm : CountedMention grp} ->
                  {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
withTheSameName grp = NamesAgree SameName grp {cm} {pl}

public export
aAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            Noun bs k
aAtRandom p = Indefinite AtRandom p {ph}

||| "[n] [description] at random": the counted determiner with
||| [CR#701.9b]'s random pick written on it. `aAtRandom`'s plural, and
||| not a plural `Indefinite`: the count is `CountedGroup`'s and the mode
||| rides beside it.
public export
countedAtRandom : (q : Quantity bs) -> (p : Predicate bs k) ->
                  {auto ph : Phrasal k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} -> Noun bs k
countedAtRandom q p = CountedGroup q (Just AtRandom) p {ph} {nz} {wf}

public export
anOpponent : Noun bs Player
anOpponent = a Opponent

public export
anyOtherTarget : {auto 0 ok : anyTargeted (Object \/ Player) bs = True} ->
                 Predicate bs (Object \/ Player)
anyOtherTarget = And [anyTarget, Other {ok = eqToSo ok}] {oa = eqToSo ok}


public export
otherCreature : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                Predicate bs Object
otherCreature n = And [creature, OtherThan n {ca}] {oa = eqToSo ty}

public export
otherCreatureYouControl : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                          {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                          Predicate bs Object
otherCreatureYouControl n = And [creature, ControlledBy You, OtherThan n {ca}] {oa = eqToSo ty}

public export
otherPlayer : Predicate bs Player
otherPlayer = And [AnyPlayer, OtherThan You]


public export
powerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
powerOf n = StatOf Power n {one}

public export
toughnessOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
toughnessOf n = StatOf Toughness n {one}

public export
manaValueOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
manaValueOf n = StatOf ManaValue n {one}


public export
nForEach : {k : Kind} -> (n : Nat) -> (p : Predicate bs k) ->
           {auto 0 nz : IsSucc n} ->
           Amount bs
nForEach n p = Times n (CountOf p) {nz}

public export
forEach : {k : Kind} -> (p : Predicate bs k) ->
          Amount bs
forEach p = nForEach 1 p

||| "no riders": the empty rider record a bare move writes.
public export
noRiders : MoveRiders bs
noRiders = MkMoveRiders [] Nothing Nothing

||| "Put <what> into <zone>": the bare move, riding nothing.
public export
move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur what) to} ->
       {auto 0 pl : Placeable (nounTy what) (zoneSort to)} -> Effect bs
move what to = Move what to noRiders {ok} {arr} {pl}

public export
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
          Effect bs
destroy n = Enact "Destroy" (Move n graveyardZ noRiders)

public export
exile : (n : Noun bs Object) -> Effect bs
exile n = Enact "Exile" (Move n exileZ noRiders)

||| "<player> exiles <n>": `exile`'s agentive surface -- the same labeled
||| move with the player performing it written as its subject.
public export
exiles : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
         Effect bs
exiles agent n = Does agent "Exile" (Move n exileZ noRiders)

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) -> Effect bs
exileWithCounters n amt kind =
  Enact "Exile"
        (Move n exileZ (MkMoveRiders [] Nothing (Just (MkCounterRider amt kind))))

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ (MkMoveRiders [] (Just who) (Just (MkCounterRider amt kind)) {one = OneController {one}}) {pl}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ noRiders {pl}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ (MkMoveRiders [EntersTapped] Nothing Nothing) {pl}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ (MkMoveRiders [EntersTapped, EntersAttacking] Nothing Nothing) {pl}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ (MkMoveRiders [] (Just You) Nothing) {pl}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
sacrifice agent n =
  Does agent "Sacrifice" (Move n graveyardZ noRiders)

||| The bare pronoun read at the PERMANENT carrier: the slot's rule takes
||| something on the battlefield [CR#109.2,110.1], so the candidates are
||| the battlefield mentions and a spell or a card the same clause named
||| is not among them. The macro layer is the only carrier for `ItAt`,
||| and a site that writes this one names the rule that gives its slot
||| this carrier.
public export
itAsPermanent : {auto 0 ok : countOnesAt PermanentSlot bs = 1} -> Noun bs Object
itAsPermanent = ItAt PermanentSlot {ok}

||| The same at the CARD carrier: the slot writes the word "card", which
||| [CR#109.2] is what takes such a description off the battlefield.
public export
itAsCard : {auto 0 ok : countOnesAt CardSlot bs = 1} -> Noun bs Object
itAsCard = ItAt CardSlot {ok}

||| "…, sacrifice it". [CR#701.21a] lets a player sacrifice a permanent
||| and nothing else, so the slot's own rule bounds what the pronoun may
||| resolve to and the macro supplies that carrier -- which is why the
||| body of a becomes-target trigger writes, though its header announced
||| the targeting spell beside the permanent it targeted.
public export
sacrificeIt : (agent : Noun bs Player) ->
              {auto 0 ok : countOnesAt PermanentSlot (nomIntro agent) = 1} ->
              {auto 0 zn : OnBattlefield (zoneOfItAt PermanentSlot (nomIntro agent))} ->
              Effect bs
sacrificeIt agent = sacrifice agent (itAsPermanent {ok}) {ok = zn}

public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n =
  Does agent "Discard" (Move n graveyardZ noRiders)

public export
discardsACard : (agent : Noun bs Player) -> Effect bs
-- sort-only: an owned-hand expansion needs a subject-read noun the
-- vocabulary doesn't have yet.
discardsACard agent = discards agent (a (InZone handZ))

||| "… discards a card at random": `discardsACard` with the selection
||| mode written.
public export
discardsACardAtRandom : (agent : Noun bs Player) -> Effect bs
discardsACardAtRandom agent = discards agent (aAtRandom (InZone handZ))

||| "Discard [n]": the ATOM -- the labeled action over a referent the
||| sentence has already named, and nothing else. Every counted or
||| choosing spelling of a keyword action is built from its atom, never
||| beside it: the atom says what discarding IS [CR#701.9a], and the
||| wrapper says how many times, and who picks [CR#701.9b].
public export
discard : (n : Noun bs Object) -> {auto 0 dk : DiscardOk n} -> Effect bs
discard n = Enact "Discard" (Move n graveyardZ noRiders)

||| "Tap [n]": [CR#701.26a] turns the permanent sideways, which is the
||| status change the body already writes -- so the keyword action is
||| that body plus its name, and adding it costs no row in any table.
public export
tap : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
      Effect bs
tap n = Enact "Tap" (SetStatus Tapped n)

||| "Untap [n]": `tap`'s twin at [CR#701.26b], which rotates the
||| permanent back upright and likewise moves nothing. The label is what
||| "Untap target creature. It gains haste until end of turn" reads back
||| (99 supported faces, re-measured 2026-08-27, every one of them naming
||| the permanent the untap acted on) even where the enclosing ability
||| announced a permanent of its own.
public export
untap : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
        Effect bs
untap n = Enact "Untap" (SetStatus Untapped n)

||| "Return [n] to [to]": the labeled zone change. "Return" is no
||| [CR#701] keyword action -- [CR#701.1] leaves it its standard English
||| meaning -- and the label states nothing the body does not; what it
||| buys is the stamp, which "Return target creature card from your
||| graveyard to the battlefield. It gains haste" reads back (93
||| supported faces, re-measured 2026-08-27).
public export
returnTo : (n : Noun bs Object) -> (to : ZoneExpr (nomIntro n)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur n) to} ->
           {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
           Effect bs
returnTo n to = Enact "Return" (Move n to noRiders {ok} {arr} {pl})

||| `returnTo` at the battlefield, the destination 93 of the family's
||| lines write.
public export
returnToBattlefield : (n : Noun bs Object) ->
                      {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                      {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                      Effect bs
returnToBattlefield n = returnTo n battlefieldZ {arr} {pl}

||| The bare pronoun read at the CREATE clause that made its referent:
||| the token side of `itAsPermanent`'s discipline, and the one narrowing
||| whose fact is an origin rather than a carrier or a label. "Create a
||| Clue token. It's an artifact with …" -- 170 supported faces,
||| re-measured 2026-08-27, all of them token-bound.
public export
itAsToken : {auto 0 ok : countItToken bs = 1} -> Noun bs Object
itAsToken = ItToken {ok}

||| "[n] [vp1] and [vp2] [until …]": the shared-subject coordination as a
||| clause. The subject is written once and the parts are predicated of
||| it, so nothing here is a pronoun and nothing is counted.
public export
sharedSubject : {0 k : Nat} -> (n : Noun bs Object) ->
                (vps : SubjectVPs k (selfSubjIntro n)) ->
                {auto 0 ne : IsSucc k} ->
                {auto 0 ok : So (vpsOk (nounZone n) (nounRegime n) vps)} ->
                (d : Maybe (Duration (vpsIntro vps))) ->
                {auto 0 sp : SpanOk Coordination d} -> Effect bs
sharedSubject n vps d = Continuously (OfSubject n vps {ne} {ok}) d {sp}


public export
dealsDivided : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
               (among : Noun (amtIntro amt) k) ->
               {auto 0 gm : GroupMention among} ->
               {auto 0 rk : DamageRecipient among} -> Effect bs
dealsDivided src amt among =
  Distribute (DividedDamage src) amt among {gm}
             {tk = DamageDivided {rk}}

public export
distributeCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (among : Noun (amtIntro amt) Object) ->
                     {auto 0 gm : GroupMention among} -> Effect bs
distributeCounters amt kind among =
  Distribute (DistributedCounters kind) amt among {gm}
             {tk = CountersDistributed}


public export
thisTurn : Duration bs
thisTurn = ThisTurn

public export
untilEndOfTurn : Duration bs
untilEndOfTurn = Until (EndOf Turn Nothing)

public export
untilYourNextTurn : Duration bs
untilYourNextTurn = Until (StartOf Turn (Just Yours))

public export
untilEndOfCombat : Duration bs
untilEndOfCombat = Until (EndOf Combat Nothing)

public export
untilYourNextUpkeep : Duration bs
untilYourNextUpkeep = Until (StartOf Upkeep (Just Yours))

public export
untilYourNextEndStep : Duration bs
untilYourNextEndStep = Until (StartOf EndStep (Just Yours))

public export
asLongAs : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
asLongAs c se = Conditionally c se AsLongAs {nn}

public export
unlessSo : (c : Condition bs) -> (se : StaticEffect (condIntro (NotCond c))) ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
unlessSo c se = Conditionally (NotCond c) se Unless {nn}

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

||| "… enters with N <kind> counters on it": the counters it enters with,
||| as against `entersWithAdditionalCounters`' extra ones.
public export
entersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                     (kind : CounterKind) -> StaticEffect bs
entersWithCounters n amt kind = EntersWithCounters n amt (PrintedKind kind) Fresh

public export
gets : (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
       (tou : PtShift (shiftIntro pow)) ->
       {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
       (d : Maybe (Duration (staticIntro (Gets n pow tou {ok})))) ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou {ok}) d {sp}

public export
gains : (n : Noun bs Object) -> (a : AbilityAt bs) ->
        (d : Maybe (Duration (selfSubjIntro n))) ->
        {auto 0 ok : GrantSubject a n} ->
        {auto 0 gr : Grantable a} ->
        {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gains n a d = Continuously (Gains n a) d

public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 ok : GrantSubject (KeywordAbility "Haste" Nothing) n} ->
             {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gainsHaste n d = gains n (KeywordAbility "Haste" Nothing) d

public export
plusOnePlusOne : CounterKind
plusOnePlusOne = BoostCounter (Up 1) (Up 1)

public export
minusOneMinusOne : CounterKind
minusOneMinusOne = BoostCounter (Down 1) (Down 1)

public export
flyingCounter : CounterKind
flyingCounter = KeywordCounter "Flying"


||| The deontic carrier with no counterfactual: the five slots the bench
||| writes wherever [CR#609.4]'s rider is absent, which is every
||| prohibition, every requirement and every gate -- of 264 supported "as
||| though" sentences, ZERO write one under a can't, a must or a gate
||| (measured 2026-08-28), and the carrier refuses the pairing anyway.
public export
deontic : {k : Kind} -> (n : Noun bs k) -> (c : Compulsion bs) ->
          (deeds : Deeds) -> (role : Role) ->
          (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
          {auto 0 ne : NonEmpty deeds} ->
          {auto 0 kd : KnownDeeds deeds} ->
          {auto 0 zn : ZoneFits (nounZone n) (deedsZone deeds role)} ->
          {auto 0 dp : DeedParticipant deeds role k (nounTy n)} ->
          {auto 0 pt : So (deonticPatientOk n deeds role patient)} ->
          StaticEffect bs
deontic n c deeds role patient =
  Deontic n c deeds role patient Nothing {ne} {kd} {zn} {dp} {pt}

public export
cantAttack : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Attack"] Agent)} ->
             {auto 0 dp : DeedParticipant ["Attack"] Agent Object (nounTy n)} ->
             {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttack n span =
  Continuously (Deontic n Forbid ["Attack"] Agent NoDeonticPatient Nothing {zn} {dp}) span {sp}

public export
cantBlock : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Agent)} ->
            {auto 0 dp : DeedParticipant ["Block"] Agent Object (nounTy n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span =
  Continuously (Deontic n Forbid ["Block"] Agent NoDeonticPatient Nothing {zn} {dp}) span {sp}

||| "[n] can't attack or block": ONE subject, ONE modality, TWO deeds --
||| the coordination the carrier's deed list is, and the shape the
||| outcome gate's "can't lose the game or win the game" reads too.
||| 109 supported lines write it (measured 2026-08-28).
public export
cantAttackOrBlock : (n : Noun bs Object) ->
                    (span : Maybe (Duration (selfSubjIntro n))) ->
                    {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Attack", "Block"] Agent)} ->
                    {auto 0 dp : DeedParticipant ["Attack", "Block"] Agent Object (nounTy n)} ->
                    {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttackOrBlock n span =
  Continuously (Deontic n Forbid ["Attack", "Block"] Agent NoDeonticPatient Nothing
                  {zn} {dp}) span {sp}

||| "[n] can [deed]": the permission, the deed restriction's twin. No
||| counterfactual is present and none is needed -- [CR#609.4]'s slot is
||| a rider on this row, not the reason for it.
public export
canDo : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
        {auto 0 kd : KnownDeeds [deed]} ->
        {auto 0 zn : ZoneFits (nounZone n) (deedsZone [deed] Agent)} ->
        {auto 0 dp : DeedParticipant [deed] Agent k (nounTy n)} ->
        StaticEffect bs
canDo n deed = Deontic n Permit [deed] Agent NoDeonticPatient Nothing {kd} {zn} {dp}

||| "[n] can [deed] as though [p]": the permission with [CR#609.4]'s
||| premise riding it. "This creature can attack as though it didn't have
||| defender" is `canDoAsThough n "Attack" (Not (HasKeyword "Defender"))`
||| -- 52 supported lines, the largest cell of the permission family.
public export
canDoAsThough : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
                (p : Predicate (nomIntro n) Object) ->
                {auto 0 kd : KnownDeeds [deed]} ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone [deed] Agent)} ->
                {auto 0 dp : DeedParticipant [deed] Agent k (nounTy n)} ->
                {auto 0 at : So (asThoughOk (Permit {bs}) [deed] (Just (AsThoughOf p)))} ->
                StaticEffect bs
canDoAsThough n deed p =
  Deontic n Permit [deed] Agent NoDeonticPatient (Just (AsThoughOf p)) {kd} {zn} {dp} {at}

||| "[who] can't [deed]": the player-subject prohibition, `PlayerCant`'s
||| whole content as a spelling over the carrier.
public export
playerCant : (deed : VerbLabel) -> (who : Noun bs Player) ->
             {auto 0 kd : KnownDeeds [deed]} ->
             {auto 0 zn : ZoneFits (nounZone who) (deedsZone [deed] Agent)} ->
             {auto 0 dp : DeedParticipant [deed] Agent Player (nounTy who)} ->
             StaticEffect bs
playerCant deed who = Deontic who Forbid [deed] Agent NoDeonticPatient Nothing {kd} {zn} {dp}

||| "[what] can't be [deed]ed": the object-subject prohibition,
||| `ObjectCant`'s whole content as a spelling over the carrier. The same
||| label as `playerCant` at the other ROLE, which is what retired the
||| two parallel act enums: the corpus writes both voices of one act one
||| card apart.
public export
objectCant : {k : Kind} -> (deed : VerbLabel) -> (what : Noun bs k) ->
             {auto 0 kd : KnownDeeds [deed]} ->
             {auto 0 zn : ZoneFits (nounZone what) (deedsZone [deed] Patient)} ->
             {auto 0 dp : DeedParticipant [deed] Patient k (nounTy what)} ->
             StaticEffect bs
objectCant deed what =
  Deontic what Forbid [deed] Patient NoDeonticPatient Nothing {kd} {zn} {dp}

||| "[what] can't be the target of [by]": the targeting prohibition, 30
||| real supported sentences (measured 2026-08-28; a naive sweep returns
||| 216, of which 186 are the reminder text printed under hexproof and
||| shroud and are no card's own line).
public export
cantBeTargetedBy : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                   (by : Noun (nomIntro what) ka) ->
                   {auto 0 tr : Targeter ka} ->
                   {auto 0 zn : ZoneFits (nounZone what) (deedsZone ["Target"] Patient)} ->
                   {auto 0 dp : DeedParticipant ["Target"] Patient k (nounTy what)} ->
                   StaticEffect bs
cantBeTargetedBy what by =
  Deontic what Forbid ["Target"] Patient (TargetedBy by {tr}) Nothing {zn} {dp}

||| "[what] can be the target of [by] as though [p]": Glaring Spotlight's
||| line, the targeting deed's permission with [CR#609.4]'s premise.
public export
canBeTargetedAsThough : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                        (by : Noun (nomIntro what) ka) ->
                        (p : Predicate (nomIntro what) Object) ->
                        {auto 0 tr : Targeter ka} ->
                        {auto 0 zn : ZoneFits (nounZone what) (deedsZone ["Target"] Patient)} ->
                        {auto 0 dp : DeedParticipant ["Target"] Patient k (nounTy what)} ->
                        StaticEffect bs
canBeTargetedAsThough what by p =
  Deontic what Permit ["Target"] Patient (TargetedBy by {tr}) (Just (AsThoughOf p)) {zn} {dp}

public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Patient)} ->
                {auto 0 dp : DeedParticipant ["Block"] Patient Object (nounTy n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span =
  Continuously (Deontic n Forbid ["Block"] Patient NoDeonticPatient Nothing {zn} {dp}) span {sp}

||| "[n] blocks IT this turn if able": the forced block whose blocked
||| creature is named by the pronoun (Fighter Class, Feral Contest,
||| Avalanche Tusker, Tower Above -- 6 supported faces).
||| The counterpart is read over the prefix the SUBJECT did not
||| announce, and the exclusion is the deed's own rule rather than a
||| preference: [CR#509.1a] has the defending player choose blockers
||| from among the creatures THEY control and, for each, a creature to
||| block that is attacking that player, while [CR#508.1a] has the
||| active player choose attackers from among the creatures THEY
||| control. A creature therefore never blocks itself, so the blocker's
||| own mention is not a candidate for what it is made to block.
||| The macro owns the segment, as the carrier macros own theirs:
||| `nounDelta n` is what the subject announced, and no author writes it
||| by hand.
public export
mustBlockIt : {bs : Bindings} -> (n : Noun bs Object) ->
              (span : Maybe (Duration (selfSubjIntro n))) ->
              {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Agent)} ->
              {auto 0 dp : DeedParticipant ["Block"] Agent Object (nounTy n)} ->
              {auto 0 ok : countOnes Object bs = 1} ->
              {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                 (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok})))} ->
              {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
mustBlockIt n span =
  Continuously (Deontic n Require ["Block"] Agent
                  (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok}))
                  Nothing {zn} {dp} {pt})
               span {sp}


public export
gainControl : (n : Noun bs Object) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              (d : Maybe (Duration (staticIntro (GainsControl You n {zn})))) ->
              {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainControl n d = Continuously (GainsControl You n {zn}) d {sp}

public export
gainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
               {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} ->
               (d : Maybe (Duration (staticIntro (GainsControl who what {zn})))) ->
               {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainsControl who what d = Continuously (GainsControl who what {zn}) d {sp}

public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)


public export
may : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
may d body = May (Just d) body Nothing Nothing

public export
mayThen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (effIntro body) -> Effect bs
mayThen d body did = May (Just d) body (Just did) Nothing

public export
mayElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (nomIntro decider) -> Effect bs
mayElse d body notd = May (Just d) body Nothing (Just notd)

public export
-- the did-branch reads everything the body introduced (it only runs if
-- the body did); the didn't-branch reads only what preceded the offer,
-- since the body never ran.
mayThenElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
              Effect (effIntro body) -> Effect (nomIntro decider) -> Effect bs
mayThenElse d body did notd = May (Just d) body (Just did) (Just notd)


public export
creatureTokOf : (pow : Amount bs) -> (tou : Amount (amtIntro pow)) ->
                List Color -> List Subtype -> TokenChars bs
creatureTokOf pow tou cs ss = MkToken (Just (pow ** tou)) cs (MkTypeLine ss [Creature]) [] Nothing

public export
creatureTok : (pow : Nat) -> (tou : Nat) -> List Color -> List Subtype -> TokenChars bs
creatureTok pow tou cs ss = creatureTokOf (Lit pow) (Lit tou) cs ss

public export
subtypesOnly : List Subtype -> TypeLine
subtypesOnly ss = MkTypeLine ss []

public export
typesOnly : List CardType -> TypeLine
typesOnly ts = MkTypeLine [] ts

public export
create : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
         {auto 0 tt : TokenTyped tok} ->
         {auto 0 tp : TokenPt tok} ->
         {auto 0 sf : SubtypesFit tok} ->
         {auto 0 ta : TokenAbilities tok} ->
         {auto 0 tc : TokenCanonical tok} ->
         Effect bs
create count tok = Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc}) []

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} ->
                        {auto 0 ta : TokenAbilities tok} ->
                        {auto 0 tc : TokenCanonical tok} ->
                        Effect bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc})
         [EntersTapped, EntersAttacking]

public export
becomesAs : (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 sw : AdditionSaysSomething (nounTy n) added} ->
            {auto 0 af : AddedFits (nounTy n) added.line} ->
            {auto 0 tc : TokenCanonical added} ->
            {auto 0 ta : TokenAbilities added} ->
            {auto 0 un : AdditionUnnamed added} ->
            {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomesAs n added d =
  Continuously (BecomesAlso n added {sw} {af} {tc} {ta} {un}) d {sp}

public export
becomes : (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 sw : AdditionSaysSomething (nounTy n) (MkToken {bs} Nothing [] added [] Nothing)} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 tc : TokenCanonical (MkToken {bs} Nothing [] added [] Nothing)} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = becomesAs n (MkToken Nothing [] added [] Nothing) d {sw} {af} {tc} {sp}

||| "[n] becomes [color]" / "[n] is [color]": the literal colour setting
||| with its duration, `becomes`' twin at [CR#613.1e]'s layer.
public export
becomesColor : (n : Noun bs Object) -> (cs : ColorSpec) ->
               (d : Maybe (Duration (selfSubjIntro n))) ->
               {auto 0 cd : ColorSpecOk cs} ->
               {auto 0 sp : SpanOk ColorSet d} -> Effect bs
becomesColor n cs d = Continuously (SetsColor n cs {cd}) d {sp}

public export
basicLandLine : (ss : List Subtype) -> {auto 0 bl : BasicLandTypes ss} -> TypeLine
basicLandLine ss = MkTypeLine ss []


public export
drawACard : Effect bs
drawACard = Draw You (Lit 1)

public export
drawCards : (n : Nat) -> Effect bs
drawCards n = Draw You (Lit n)

public export
drawsACard : (who : Noun bs Player) -> Effect bs
drawsACard who = Draw who (Lit 1)


public export
chooseOne : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 1) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 2) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Macros.oneOrBoth {bs}) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrBoth modes = Modal Macros.oneOrBoth modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast {bs} 1) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Macros.anyNumber {bs}) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {wf = Oh} {tw} {mf} {dm = eqToSo dm}

public export
notSo : (c : Condition bs) -> Condition bs
notSo c = NotCond c

public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
       Condition bs
itsA p = Matches (It {ok}) p {sy} {zc}

public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc})

||| "If it's a creature card, …": the copula's complement writes the word
||| "card", which [CR#109.2] takes off the battlefield outright, so the
||| pronoun reads the card candidates and not every singular
||| object the clause announced. The word "card" is the READ's carrier
||| here, not a zone the sentence states -- which is why the complement
||| stays the bare type word.
public export
itsACard : (p : Predicate bs Object) ->
           {auto 0 ok : countOnesAt CardSlot bs = 1} ->
           {auto 0 sy : PredSays p} ->
           {auto 0 zc : ZoneFits (zoneOfItAt CardSlot bs) (seedZone p)} ->
           Condition bs
itsACard p = Matches (itAsCard {ok}) p {sy} {zc}


public export
libraryOf : (n : Noun bs Player) -> ZoneExpr bs
libraryOf n = ZoneAt Library (OwnedBy n {ps = LibraryIsOwned})

public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt (OneEnd OnTop) Nothing Nothing Bare

public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt (OneEnd OnBottom) Nothing Nothing Bare

public export
onTopIn : (a : Arrangement) ->
          {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnTop) (Just a)} ->
          ZoneExpr bs
onTopIn a = LibraryAt (OneEnd OnTop) (Just a) Nothing {af} {nf = Oh} Bare

public export
onBottomIn : (a : Arrangement) ->
             {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnBottom) (Just a)} ->
             ZoneExpr bs
onBottomIn a = LibraryAt (OneEnd OnBottom) (Just a) Nothing {af} {nf = Oh} Bare

public export
nthFromTop : (n : LibOrdinal) -> ZoneExpr bs
nthFromTop n = LibraryAt (OneEnd OnTop) Nothing (Just n) Bare

||| "on the top or bottom of <a> library": the bare position disjunction,
||| no chooser named (Write into Being).
public export
topOrBottomZ : ZoneExpr bs
topOrBottomZ = LibraryAt (EitherEnd Nothing) Nothing Nothing Bare

||| "on <player>'s choice of the top or bottom of <a> library": the
||| separable chooser slot over the same disjunction.
public export
choiceOfTopOrBottom : (chooser : Noun bs Player) ->
                      {auto 0 ag : EventAgent (Just chooser)} -> ZoneExpr bs
choiceOfTopOrBottom chooser = LibraryAt (EitherEnd (Just chooser) {ag}) Nothing Nothing Bare

||| "into <a> library Nth from the top or on the bottom": the offset
||| spelling, whose ordinal rides the top alternative [CR#401.7].
public export
nthFromTopOrBottomZ : (n : LibOrdinal) -> ZoneExpr bs
nthFromTopOrBottomZ n = LibraryAt (EitherEnd Nothing) Nothing (Just n) Bare

||| "into <a> library, shuffled": the randomizing destination
||| [CR#701.24a]. Owner-rooted like every other move destination
||| [CR#400.3], so the scope is bare -- "its owner's library" is what the
||| bare form spells.
public export
shuffledIntoZ : ZoneExpr bs
shuffledIntoZ = LibraryAt Shuffled Nothing Nothing Bare

||| "Shuffle [n] into its owner's library": the move whose destination
||| randomizes the pile it lands in [CR#701.24c]. The library is shuffled
||| whatever becomes of the card named, and a set that turns out empty
||| shuffles it too [CR#701.24d] -- neither is the clause's business, so
||| the clause is the move and nothing more.
public export
shuffleInto : (n : Noun bs Object) ->
              {auto 0 pl : Placeable (nounTy n) Library} -> Effect bs
shuffleInto n = Enact "Shuffle" (Move n shuffledIntoZ noRiders {pl})

||| "[agent] shuffles [n] into their library": the same move in the
||| agentive voice, on `puts`' model. The library is still the moved
||| card's owner's [CR#400.3]; the subject is who performs the act.
public export
shufflesInto : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
               {auto 0 pl : Placeable (nounTy n) Library} -> Effect bs
shufflesInto agent n = Does agent "Shuffle" (Move n shuffledIntoZ noRiders {pl})

||| "the top [amt] cards of your library": the slice a look opens over,
||| with the count written as an amount rather than a literal.
public export
topSlice : (amt : Amount bs) -> Noun bs Object
topSlice amt = LibrarySlice OnTop amt You

public export
topCards : (n : Nat) -> Noun bs Object
topCards n = topSlice (Lit n)

public export
topCard : Noun bs Object
topCard = topCards 1

public export
bottomCard : Noun bs Object
bottomCard = LibrarySlice OnBottom (Lit 1) You

public export
oneOf : (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} -> Noun bs Object
oneOf grp = SomeOf (exactly 1) Nothing grp {gm}

public export
someOf : (n : Nat) -> (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} ->
         {auto 0 nz : NonZeroQ (exactly {bs} n)} ->
         {auto 0 wf : WellFormedQ (exactly {bs} n)} -> Noun bs Object
someOf n grp = SomeOf (exactly n) Nothing grp {gm} {nz} {wf}

||| "[q] [description] from among [grp]": the DESCRIBED partitive --
||| "put a creature card from among them into your hand". `someOf`'s
||| twin with the slice's own head noun written, which is the whole of
||| what makes the preposition "from among" rather than "of".
public export
fromAmong : (q : Quantity bs) -> (p : Predicate bs Object) ->
            (grp : Noun bs Object) ->
            {auto 0 gm : GroupMention grp} ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} -> Noun bs Object
fromAmong q p grp = SomeOf q (Just p) grp {gm} {nz} {wf}

||| "a [description] from among [grp]": `fromAmong` at the one-member
||| count, which is what most of the family writes.
public export
oneFromAmong : (p : Predicate bs Object) -> (grp : Noun bs Object) ->
               {auto 0 gm : GroupMention grp} -> Noun bs Object
oneFromAmong p grp = SomeOf (exactly 1) (Just p) grp {gm}


public export
lookAt : (n : Noun bs Object) -> Effect bs
lookAt n = Expose LookAt You (ExposedCards n)

public export
revealCards : (n : Noun bs Object) -> Effect bs
revealCards n = Expose Reveal You (ExposedCards n)

public export
lookAtHandOf : (n : Noun bs Player) -> Effect bs
lookAtHandOf n = Expose LookAt You (ExposedZone (handOf n))

public export
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countOnes Player (nomIntro who) = 1} -> Effect bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))



public export
searchLibraryFor : (p : Predicate bs Object) ->
                   {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryFor p = Search You (OneZone yourLibrary) (exactly 1) p {zf}

||| "Search your library for [q] [description]": the counted find.
public export
searchLibraryForCount : (q : Quantity bs) -> (p : Predicate bs Object) ->
                        {auto 0 nz : NonZeroQ q} ->
                        {auto 0 wf : WellFormedQ q} ->
                        {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryForCount q p = Search You (OneZone yourLibrary) q p {zf}

||| "Search <player>'s graveyard, hand, and library for …": the three-zone
||| sweep, possessor-anchored [CR#701.23a], now written as the ordinary
||| coordination the sort takes.
public export
searchZonesOf : (whose : Noun bs Player) -> (p : Predicate bs Object) ->
                {auto 0 zf : ZoneFree p} -> Effect bs
searchZonesOf whose p =
  Search You (SomeZones (Just whose) [Graveyard, Hand, Library]) (exactly 1) p {zf}

||| "Search your library and/or graveyard for …": the two-zone
||| coordination, the "and/or" family's commonest arity.
public export
searchLibraryOrGraveyard : (p : Predicate bs Object) ->
                           {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryOrGraveyard p =
  Search You (SomeZones (Just You) [Library, Graveyard]) (exactly 1) p {zf}

||| "<player> puts <it> into/onto <zone>": the agentive placement clause.
public export
puts : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
       (to : ZoneExpr (nomIntro n)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur n) to} ->
       {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
       Effect bs
puts agent n to = Does agent "Put" (Move n to noRiders {ok} {arr} {pl})

public export
shuffle : Effect bs
shuffle = Shuffle You

public export
lifeTotalBecomes : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
lifeTotalBecomes who a = ChangeLife who (Set a)


public export
ifWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                 (d : Maybe (Duration (eventIntro ev))) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 sp : SpanOk Replacement d} -> Effect bs
ifWouldInstead ev repl d = Continuously (Intercepts ev [] Nothing repl Repeatedly Nothing {ok}) d {sp}

public export
nextTimeWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 sp : SpanOk Replacement d} -> Effect bs
nextTimeWouldInstead ev repl d =
  Continuously (Intercepts ev [] Nothing repl NextTimeOnly Nothing {ok}) d {sp}

public export
preventAll : (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAll kind scope d = Continuously (Prevents kind AllOfIt scope Nothing Nothing) d {sp}

public export
preventNext : (kind : DamageKind) -> (amt : Amount bs) ->
              (scope : DamageScope (amtIntro amt)) ->
              (d : Maybe (Duration (scopeIntro scope))) ->
              {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNext kind amt scope d =
  Continuously (Prevents kind (TheNext amt) scope Nothing Nothing) d {sp}

public export
preventAllBy : (kind : DamageKind) -> (scope : DamageScope bs) ->
               (src : Noun (scopeIntro scope) Object) ->
               (d : Maybe (Duration (nomIntro src))) ->
               {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAllBy kind scope src d =
  Continuously (Prevents kind AllOfIt scope (Just src) Nothing) d {sp}

public export
preventNextBy : (kind : DamageKind) -> (amt : Amount bs) ->
                (scope : DamageScope (amtIntro amt)) ->
                (src : Noun (scopeIntro scope) Object) ->
                (d : Maybe (Duration (nomIntro src))) ->
                {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNextBy kind amt scope src d =
  Continuously (Prevents kind (TheNext amt) scope (Just src) Nothing) d {sp}

public export
shieldingIt : {k : Kind} -> (n : Noun bs k) ->
              {auto 0 rk : DamageRecipient n} -> DamageScope bs
shieldingIt n = ToRecipient n {rk}

public export
exileUntil : (n : Noun bs Object) -> (ev : GameEvent (preIntro (exile n))) ->
             Effect bs
exileUntil n ev = HeldUntil (exile n) ev {ok = Oh}

||| "<permanent> phases out until [event]": [CR#610.4]'s rider, the
||| second one-shot the CR hangs "until" on beside [CR#610.3]'s zone
||| change.
public export
phasesOutUntil : (n : Noun bs Object) ->
                 {auto 0 zn : OnBattlefield (nounZone n)} ->
                 {auto 0 at : StatusEffectVal PhasedOut} ->
                 (ev : GameEvent (annIntro (SetStatus PhasedOut n {ok = zn} {at}))) ->
                 Effect bs
phasesOutUntil n ev = HeldUntil (SetStatus PhasedOut n {ok = zn} {at}) ev {ok = Oh}

public export
insteadOf : (replaced : Effect bs) -> (repl : Effect (replacedCtx replaced)) ->
            {auto 0 na : NotInstead replaced} ->
            {auto 0 nb : NotInstead repl} -> Effect bs
insteadOf replaced repl = InsteadOf replaced repl {na} {nb}


public export
generic : Nat -> ManaSymbol
generic n = Simple (Generic n)

public export
pip : Color -> ManaSymbol
pip c = Simple (Specific (OfColor c))

public export
colorlessPip : ManaSymbol
colorlessPip = Simple (Specific Colorless)

public export
hybridPip : (a : Color) -> (b : Color) ->
            {auto 0 ds : HalvesDistinct (Specific (OfColor a)) b} -> ManaSymbol
hybridPip a b = Hybrid (Specific (OfColor a)) b {ds}

public export
monoHybridPip : Nat -> Color -> ManaSymbol
monoHybridPip n c = Hybrid (Generic n) c

public export
phyrexianPip : Color -> ManaSymbol
phyrexianPip c = Phyrexian c Nothing

public export
payLife : (who : Noun bs Player) -> (n : Nat) -> Cost bs
payLife who n = Do (ChangeLife who (Down (Lit n)))

public export
doThen : (body : Effect bs) -> Effect (effIntro body) -> Effect bs
doThen body did = May Nothing body (Just did) Nothing

public export
doElse : (body : Effect bs) -> Effect bs -> Effect bs
doElse body notd = May Nothing body Nothing (Just notd)

public export
-- a reflexive trigger waits on the stack rather than continuing in the
-- same resolution as mayThen's arm [CR#603.3], so its bindings are
-- settled here first.
mayWhen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (settleTargets (effIntro body)) ->
          {auto 0 ok : So (admitsReflexEnclosure (reflexEncloseUse body))} ->
          Effect bs
mayWhen d body trig =
  Reflexively (May (Just d) body Nothing Nothing) trig {en = ok}


-- Wrapping macros for the optional and proof-carrying slots, so a card
-- never binds an implicit: every slot below is reached by a positional
-- argument here instead.

||| "Target player mills N cards."
public export
mills : (agent : Noun bs Player) -> (amt : Amount (nomIntro agent)) ->
        (whose : Noun (nomIntro agent) Player) ->
        {auto 0 sp : SlicePossessor whose} ->
        Effect bs
mills agent amt whose =
  Does agent "Mill"
       (Move (LibrarySlice OnTop amt whose {sp}) graveyardZ noRiders)

||| The context a scry's or surveil's split reads: the slice its look put
||| in front of the player.
public export
lookedTop : (bs : Bindings) -> (amt : Amount bs) -> Bindings
lookedTop bs amt = nomIntro (topSlice {bs} amt)

||| The context the "and the rest" clause reads: what moving the chosen
||| pile left of the looked-at group. [CR#608.2d] has the player announce
||| the choice as the effect applies, so the unchosen stay behind and the
||| next clause names them as the complement.
public export
lookedRest : (bs : Bindings) -> (0 mn : countManys Object bs = 1) ->
             (z : Zone) -> Bindings
lookedRest bs mn z = moveIntro {bs} Nothing (SomeOf anyNumber Nothing (Them {ok = mn}) {gm = Oh} {wf = Oh}) (Just z)

||| The look a slice-partitioning keyword action opens with, over the
||| player the clause has already named.
public export
theyLookAtTop : {bs : Bindings} -> (amt : Amount bs) ->
                {auto 0 an : countOnes Player bs = 1} -> Effect bs
theyLookAtTop amt =
  Expose LookAt (They {ok = an})
         (ExposedCards (LibrarySlice OnTop amt (They {ok = an})))

||| "Scry [amt]" [CR#701.22a] in full: look at the top [amt] cards of your
||| library, then put any number of them on the bottom of your library in
||| any order and the rest on top of your library in any order. The split
||| is the choice [CR#608.2d] has the player announce as the effect
||| applies, so the unchosen read back as "the rest". `scryOne` writes the
||| one-card spelling, which the singular slice forces.
public export
scry : {bs : Bindings} -> (amt : Amount bs) ->
       {auto 0 mn : countManys Object (lookedTop bs amt) = 1} ->
       {auto 0 ps : Placeable (tyOfThem (lookedTop bs amt)) Library} ->
       {auto 0 tr : So (theRestOk (lookedRest (lookedTop bs amt) mn Library))} ->
       {auto 0 pr : Placeable (tyOfGroup (lookedRest (lookedTop bs amt) mn Library)) Library} ->
       Effect bs
scry amt =
  Does You "Scry" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf anyNumber Nothing (Them {ok = mn}) {gm = Oh} {wf = Oh})
                            (onBottomIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

||| "Surveil [amt]" [CR#701.25a] in full: the same look and the same
||| split, with the chosen pile going to the graveyard instead of under
||| the library. No order clause rides that pile: the rule writes one only
||| on the remainder.
public export
surveil : {bs : Bindings} -> (amt : Amount bs) ->
          {auto 0 mn : countManys Object (lookedTop bs amt) = 1} ->
          {auto 0 ps : Placeable (tyOfThem (lookedTop bs amt)) Graveyard} ->
          {auto 0 tr : So (theRestOk (lookedRest (lookedTop bs amt) mn Graveyard))} ->
          {auto 0 pr : Placeable (tyOfGroup (lookedRest (lookedTop bs amt) mn Graveyard)) Library} ->
          Effect bs
surveil amt =
  Does You "Surveil" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf anyNumber Nothing (Them {ok = mn}) {gm = Oh} {wf = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

||| "Scry 1": [CR#701.22a] over a one-card slice. "Any number of them" of
||| one card is a free choice and "the rest" is what declining it leaves
||| on top, so the split is written as the offer -- the spelling the
||| printed reminder text uses. The plural spelling is unavailable here,
||| not merely unchosen: a one-card slice binds singular, and the group
||| anaphor "them" and its complement both want a plural antecedent. The
||| offer names the looked-at card by its word rather than as "it", which
||| is what lets a scry stand in a clause that has already named an
||| object: a cast spell is on the stack, and no card word reaches it.
public export
scryOne : {bs : Bindings} ->
          {auto 0 iw : countWord CardW (lookedTop bs (Lit 1)) = 1} ->
          {auto 0 pi : Placeable (tyOfThat CardW (lookedTop bs (Lit 1))) Library} ->
          Effect bs
scryOne =
  Does You "Scry" {kn = Oh}
       (Sequentially [ lookAt topCard
                     , may You (move (That CardW {ok = iw}) onBottomZ {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pi}) ])

||| "Surveil 1": [CR#701.25a] over a one-card slice, `scryOne`'s spelling
||| with the graveyard as the offered destination.
public export
surveilOne : {bs : Bindings} ->
             {auto 0 iw : countWord CardW (lookedTop bs (Lit 1)) = 1} ->
             {auto 0 pi : Placeable (tyOfThat CardW (lookedTop bs (Lit 1))) Graveyard} ->
             Effect bs
surveilOne =
  Does You "Surveil" {kn = Oh}
       (Sequentially [ lookAt topCard
                     , may You (move (That CardW {ok = iw}) graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = pi}) ])

||| "Target player scries N." / "Target player surveils N."
||| [CR#701.22a] and [CR#701.25a] name one player and read that player's
||| own library, so the subject is written once and the slice, the split
||| and both destinations read it back as the anaphor.
public export
playerScries : {bs : Bindings} -> (agent : Noun bs Player) ->
               (amt : Amount (nomIntro agent)) ->
               {auto 0 an : countOnes Player (nomIntro agent) = 1} ->
               {auto 0 mn : countManys Object (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) = 1} ->
               {auto 0 ps : Placeable (tyOfThem (nomIntro (LibrarySlice OnTop amt (They {ok = an})))) Library} ->
               {auto 0 tr : So (theRestOk (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Library))} ->
               {auto 0 pr : Placeable (tyOfGroup (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Library)) Library} ->
               Effect bs
playerScries agent amt =
  Does agent "Scry" {kn = Oh}
       (Sequentially [ theyLookAtTop amt {an}
                     , move (SomeOf anyNumber Nothing (Them {ok = mn}) {gm = Oh} {wf = Oh})
                            (onBottomIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
playerSurveils : {bs : Bindings} -> (agent : Noun bs Player) ->
                 (amt : Amount (nomIntro agent)) ->
                 {auto 0 an : countOnes Player (nomIntro agent) = 1} ->
                 {auto 0 mn : countManys Object (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) = 1} ->
                 {auto 0 ps : Placeable (tyOfThem (nomIntro (LibrarySlice OnTop amt (They {ok = an})))) Graveyard} ->
                 {auto 0 tr : So (theRestOk (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Graveyard))} ->
                 {auto 0 pr : Placeable (tyOfGroup (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Graveyard)) Library} ->
                 Effect bs
playerSurveils agent amt =
  Does agent "Surveil" {kn = Oh}
       (Sequentially [ theyLookAtTop amt {an}
                     , move (SomeOf anyNumber Nothing (Them {ok = mn}) {gm = Oh} {wf = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

||| "permanents and/or players that have a counter": the description
||| [CR#701.34a] has proliferate choose from. The object half asks for a
||| permanent carrying any counter and the player half for the same read
||| at the player seat, where `HasCounters` has no cell -- [CR#122.1f]
||| writes the player-side test in the ranged shape `CounterCompare`
||| takes. The printed reminder text drops the restriction ("choose any
||| number of permanents and/or players"); the rule keeps it, and the
||| two agree on outcome, since a holder with no counters is given none.
public export
proliferable : Predicate bs (Object \/ Player)
proliferable = kindJoin (CounterCompare Nothing AtLeast (Lit 1))
                        (And [Permanent, HasCounters Nothing])

||| The context proliferate's giving clause reads: the union mention its
||| choice announced.
public export
proliferated : (bs : Bindings) -> Bindings
proliferated bs = chosenIntro {bs} (CountedGroup Macros.anyNumber Nothing Macros.proliferable)

||| "Proliferate" [CR#701.34a] in full: choose any number of permanents
||| and/or players that have a counter, then give each one additional
||| counter of each kind that permanent or player already has. The two
||| clauses are the rule's own, and the second reads the first back as
||| the union demonstrative -- one choice, then a giving distributed
||| over its members. "Proliferate twice" and "proliferate X times" are
||| `Repeated` over this: [CR#701.34a] fixes the per-kind amount at one,
||| so a written count can only iterate the whole action.
public export
proliferate : {bs : Bindings} ->
              {auto 0 mj : countManyWord JoinW (Macros.proliferated bs) = 1} ->
              Effect bs
proliferate =
  Enact "Proliferate" {kn = Oh}
        (Sequentially [ Choose (CountedGroup Macros.anyNumber Nothing Macros.proliferable) Nothing
                      , GiveCountersOfOwnKinds (EachOf (Those JoinW {ok = mj})) ])

||| "<player> loses N <kind> counters": the counted removal beside the
||| bare "all" spelling `LosesCounters` writes with the slot unfilled.
public export
losesCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                (kind : Maybe CounterKind) ->
                {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesCounters who amt kind = LosesCounters who kind (Just amt) {pk}

||| "<player> loses all <kind> counters": the bare removal, which the
||| unwritten amount slot spells.
public export
losesAllCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                   {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesAllCounters who kind = LosesCounters who kind Nothing {pk}

||| "Flying", "Trample": a keyword written with no parameter.
public export
keyword : {0 bs : Bindings} -> (kw : KeywordLabel) ->
          {auto 0 pf : KeywordParamFits {bs} kw
                         (the (Maybe (KeywordParam bs)) Nothing)} ->
          AbilityAt bs
keyword kw = KeywordAbility kw Nothing {pf}

||| "Enchant creature": a keyword whose parameter is a subject phrase.
public export
keywordSubject : {0 bs : Bindings} -> {k : Kind} -> (kw : KeywordLabel) ->
                 (p : Predicate [] k) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamSubject {bs} p))} ->
                 AbilityAt bs
keywordSubject kw p = KeywordAbility kw (Just (ParamSubject p)) {pf}

||| "Equip {2}", "Ward {2}": a keyword whose parameter is a cost.
public export
keywordCosting : {0 bs : Bindings} -> (kw : KeywordLabel) -> (c : Cost []) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamCost {bs} c))} ->
                 AbilityAt bs
keywordCosting kw c = KeywordAbility kw (Just (ParamCost c)) {pf}

||| "Protection from red": a keyword whose parameter is a quality.
public export
keywordQuality : (kw : KeywordLabel) -> (q : Predicate bs Object) ->
                 {auto 0 pf : KeywordParamFits kw (Just (ParamQuality q))} ->
                 AbilityAt bs
keywordQuality kw q = KeywordAbility kw (Just (ParamQuality q)) {pf}

||| "Renown 1": a keyword whose parameter is a written number.
public export
keywordNumber : {0 bs : Bindings} -> (kw : KeywordLabel) -> (amt : Amount []) ->
                {auto 0 pf : KeywordParamFits {bs} kw
                               (Just (ParamNumber {bs} amt))} ->
                AbilityAt bs
keywordNumber kw amt = KeywordAbility kw (Just (ParamNumber amt)) {pf}

||| "Whenever <event>, <effect>": the bare trigger — no alternative event,
||| window, limit or intervening-if clause written.
public export
triggered : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
            (eff : Effect (eventAfter ev)) ->
            {auto 0 hn : HeaderNontarget ev} ->
            {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
            {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing Nothing} ->
            AbilityAt bs
triggered word ev eff =
  Triggered word ev [] Nothing [] Nothing Nothing Nothing eff {hn} {ae} {cd}

||| "Whenever …, if <condition>, …": a trigger with an intervening-if clause.
public export
triggeredIf : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (cond : Condition (headerCtx [] ev)) ->
              (eff : Effect (interveningIntro (Just cond))) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
              {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing (Just cond)} ->
              AbilityAt bs
triggeredIf word ev cond eff =
  Triggered word ev [] Nothing [] Nothing Nothing (Just cond) eff {hn} {ae} {cd}

||| "Whenever X, Y, or Z, …": a trigger whose header coordinates further
||| events. The arms are a list, so the same macro writes the two-armed
||| header and the three-armed one.
public export
triggeredOr : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (eff : Effect (headerCtx alts ev)) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 ae : AltEvent word alts} ->
              {auto 0 cd : ChapterDefaults ev alts Nothing [] Nothing Nothing Nothing} ->
              AbilityAt bs
triggeredOr word ev alts eff =
  Triggered word ev alts Nothing [] Nothing Nothing Nothing eff {hn} {ae} {cd}

||| "Whenever …, during <window>, …": a trigger confined to a window.
public export
triggeredOnlyDuring : {bs : Bindings} -> (word : TriggerWord) ->
                      (ev : GameEvent bs) -> (w : TriggerWindow) ->
                      (eff : Effect (eventAfter ev)) ->
                      {auto 0 hn : HeaderNontarget ev} ->
                      {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                      {auto 0 cd : ChapterDefaults ev [] Nothing [] (Just w) Nothing Nothing} ->
                      AbilityAt bs
triggeredOnlyDuring word ev w eff =
  Triggered word ev [] Nothing [] (Just w) Nothing Nothing eff {hn} {ae} {cd}

||| "Whenever <event> while <state>, <effect>": the header's concurrent
||| clause on a bare trigger [CR#603.1,603.2].
public export
triggeredWhile : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                 (wh : Concurrent (headerCtx (the (List (GameEvent bs)) []) ev)) ->
                 (eff : Effect (eventAfter ev)) ->
                 {auto 0 hn : HeaderNontarget ev} ->
                 {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                 {auto 0 cd : ChapterDefaults ev [] (Just wh) [] Nothing Nothing Nothing} ->
                 AbilityAt bs
triggeredWhile word ev wh eff =
  Triggered word ev [] (Just wh) [] Nothing Nothing Nothing eff {hn} {ae} {cd}

||| "While <state>": the concurrent clause naming a game state.
public export
whileState : {0 bs : Bindings} -> Condition bs -> Concurrent bs
whileState c = WhileTrue c

||| "When <event1> and <word> <event2>, <effect>": two whole headers over
||| one effect. The effect reads what the headers announce alike
||| (`joinedCtx`).
public export
triggeredJoined : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                  (joins : List (JoinedHeader bs)) ->
                  (eff : Effect (joinedCtx joins (headerCtx (the (List (GameEvent bs)) []) ev))) ->
                  {auto 0 hn : HeaderNontarget ev} ->
                  {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                  {auto 0 cd : ChapterDefaults ev [] Nothing joins Nothing Nothing Nothing} ->
                  AbilityAt bs
triggeredJoined word ev joins eff =
  Triggered word ev [] Nothing joins Nothing Nothing Nothing eff {hn} {ae} {cd}

||| A joined header with no coordination, no concurrent clause and no
||| window of its own.
public export
joinedHead : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
             {auto 0 hn : HeaderNontarget ev} ->
             {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
             JoinedHeader bs
joinedHead word ev = MkJoinedHeader word ev [] Nothing Nothing {hn} {ae}

||| A joined header carrying its own concurrent clause -- Autarch
||| Mammoth's "and whenever it attacks while saddled".
public export
joinedHeadWhile : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                  (wh : Concurrent (headerCtx (the (List (GameEvent bs)) []) ev)) ->
                  {auto 0 hn : HeaderNontarget ev} ->
                  {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                  JoinedHeader bs
joinedHeadWhile word ev wh = MkJoinedHeader word ev [] (Just wh) Nothing {hn} {ae}

||| "Whenever …, … . This triggers only once each turn."
public export
triggeredOnlyOnce : {bs : Bindings} -> (word : TriggerWord) ->
                    (ev : GameEvent bs) -> (lim : UsageLimit) ->
                    (eff : Effect (eventAfter ev)) ->
                    {auto 0 hn : HeaderNontarget ev} ->
                    {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                    {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing (Just lim) Nothing} ->
                    AbilityAt bs
triggeredOnlyOnce word ev lim eff =
  Triggered word ev [] Nothing [] Nothing (Just lim) Nothing eff {hn} {ae} {cd}

||| "<cost>: <effect>": the bare activated ability — no window, usage limit
||| or activation condition written.
public export
activated : (cost : Cost bs) ->
            (eff : Effect (publicOnly (costIntro cost))) ->
            {auto 0 tp : CostTapOnce cost} ->
            {auto 0 py : CostPaidByYou cost} -> AbilityAt bs
activated cost eff = Activated cost eff Nothing Nothing Nothing {tp} {py}

||| "Activate only as a sorcery" / "only during your upkeep".
public export
activatedOnlyDuring : (cost : Cost bs) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (w : Timing) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyDuring cost eff w = Activated cost eff (Just w) Nothing Nothing {tp} {py}

||| "Activate only once each turn" (or once each game).
public export
activatedOnlyOnce : (cost : Cost bs) ->
                    (eff : Effect (publicOnly (costIntro cost))) ->
                    (lim : UsageLimit) ->
                    {auto 0 tp : CostTapOnce cost} ->
                    {auto 0 py : CostPaidByYou cost} ->
                    AbilityAt bs
activatedOnlyOnce cost eff lim = Activated cost eff Nothing (Just lim) Nothing {tp} {py}

||| "Activate only if <condition>."
public export
activatedOnlyIf : (cost : Cost bs) ->
                  (eff : Effect (publicOnly (costIntro cost))) ->
                  (g : Condition bs) ->
                  {auto 0 tp : CostTapOnce cost} ->
                  {auto 0 py : CostPaidByYou cost} ->
                  AbilityAt bs
activatedOnlyIf cost eff g = Activated cost eff Nothing Nothing (Just g) {tp} {py}

||| "Activate only once each turn and only if <condition>."
public export
activatedOnlyOnceIf : (cost : Cost bs) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (lim : UsageLimit) -> (g : Condition bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyOnceIf cost eff lim g =
  Activated cost eff Nothing (Just lim) (Just g) {tp} {py}

||| "You may play <what>."
public export
mayPlay : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
          {auto 0 pz : PlaySource (nounZone what)
                                  (the (Maybe (ZoneExpr (nomIntro what))) Nothing)
                                  False} ->
          {auto 0 cv : CastableTy Play (nounTy what)} -> StaticEffect bs
mayPlay who what = MayPlay who what Play Nothing Nothing Nothing Nothing {pz} {cv}

||| "You may cast <what> from <zone>."
public export
mayCastFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 pz : PlaySource (nounZone what) (Just from) False} ->
              {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFrom who what from =
  MayPlay who what Cast (Just from) Nothing Nothing Nothing {pz} {cv}

||| "You may play <what> from <zone>."
public export
mayPlayFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 pz : PlaySource (nounZone what) (Just from) False} ->
              {auto 0 cv : CastableTy Play (nounTy what)} -> StaticEffect bs
mayPlayFrom who what from =
  MayPlay who what Play (Just from) Nothing Nothing Nothing {pz} {cv}

||| "You may cast <what> from <zone>", under a play limit.
public export
mayCastFromLimited : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     (from : ZoneExpr (nomIntro what)) -> (lim : PlayLimit) ->
                     {auto 0 pz : PlaySource (nounZone what) (Just from) False} ->
                     {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFromLimited who what from lim =
  MayPlay who what Cast (Just from) Nothing (Just lim) Nothing {pz} {cv}

||| "You may cast <what> as though it had flash." The hardcoded
||| `PlayAsThough = HadFlash` retired: the premise is the carrier's
||| general [CR#609.4] one, and flash is `HasKeyword "Flash"` like any
||| other counterfactual payload. 89 supported lines write it.
public export
mayCastAsThough : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                  {auto 0 pz : PlaySource (nounZone what)
                                          (the (Maybe (ZoneExpr (nomIntro what))) Nothing)
                                          True} ->
                  {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastAsThough who what =
  MayPlay who what Cast Nothing (Just (AsThoughOf (HasKeyword "Flash"))) Nothing Nothing
          {pz} {cv}

||| "[n] leaves the battlefield": the zone the leaves-the-battlefield
||| ability names [CR#603.10a], written into the row's source slot.
public export
leavesBattlefield : {0 bs : Bindings} -> (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    GameEvent bs
leavesBattlefield n = Leaves n (Just (FromZone [battlefieldZ])) {zn}

||| "[n] leaves [z]": any other zone a header watches an object leave --
||| "one or more cards leave your graveyard" [CR#603.10a].
public export
leavesZone : {0 bs : Bindings} -> (n : Noun bs Object) -> (z : ZoneExpr bs) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just (zoneSort z))} ->
             GameEvent bs
leavesZone n z = Leaves n (Just (FromZone [z])) {zn}

||| "… is put into <zone> from <source>."
public export
putIntoFrom : (n : Noun bs Object) -> (to : ZoneExpr bs) -> (src : EventSource bs) ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource (Just src)} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone (Just src))} -> GameEvent bs
putIntoFrom n to src = PutInto n to (Just src) {dk} {sk} {zn}

||| "… enters with an additional counter on it."
public export
entersWithAdditionalCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                               (kind : CounterKind) -> StaticEffect bs
entersWithAdditionalCounters n amt kind =
  EntersWithCounters n amt (PrintedKind kind) Additional

||| "… enters with N fewer <kind> counters on it": compleated's reminder.
public export
entersWithFewerCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                          (kind : CounterKind) -> StaticEffect bs
entersWithFewerCounters n amt kind = EntersWithCounters n amt (PrintedKind kind) Fewer

||| "Whenever <creature> attacks": no defender written.
public export
attacks : (n : Noun bs Object) ->
          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
attacks n = Attacks n NoDefender {zn}

||| "Whenever <creature> attacks <player>", and the same shape wherever
||| [CR#506.3] lets the defender be named: a planeswalker, a battle, or a
||| joined phrase such as "that player or planeswalker".
public export
attacksPlayer : {k : Kind} -> (n : Noun bs Object) -> (whom : Noun (nomIntro n) k) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 sg : nounPlur whom = OneOf} ->
                {auto 0 at : Attackable whom} -> GameEvent bs
attacksPlayer n whom = Attacks n (OneDefender whom {sg} {at}) {zn}

||| "Whenever one or more tokens are created."
public export
tokensCreated : (n : Noun bs Object) ->
                {auto 0 tk : TokenPhrase n} -> GameEvent bs
tokensCreated n = TokensCreated n Nothing Nothing Nothing {tk}

||| "Whenever one or more tokens are created under <player>'s control."
public export
tokensCreatedUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                     {auto 0 tk : TokenPhrase n} ->
                     {auto 0 vo : CreationVoice Nothing Nothing (Just under)} ->
                     GameEvent bs
tokensCreatedUnder n under = TokensCreated n Nothing Nothing (Just under) {tk} {vo}

||| "Whenever an effect creates tokens under <player>'s control."
public export
tokensCreatedByEffectUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                             {auto 0 tk : TokenPhrase n} ->
                             {auto 0 vo : CreationVoice (Just AnEffect) Nothing
                                                        (Just under)} ->
                             GameEvent bs
tokensCreatedByEffectUnder n under =
  TokensCreated n (Just AnEffect) Nothing (Just under) {tk} {vo}

||| "Whenever one or more [kind] counters are put on …": the many-counter
||| reading, kind named.
public export
manyCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                   (n : Noun bs Object) ->
                   {auto 0 sc : counterScope kind = Object} -> GameEvent bs
manyCounterEvent dir kind n =
  CounterEvent dir (Just kind) n ManyCounters Nothing Nothing

||| "Whenever a [kind] counter is put on …": the one-counter reading.
public export
singleCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                     (n : Noun bs Object) ->
                     {auto 0 sc : counterScope kind = Object} -> GameEvent bs
singleCounterEvent dir kind n =
  CounterEvent dir (Just kind) n OneCounter Nothing Nothing

||| "Whenever a counter is put on …" / "… removed from …": the kind-blind
||| single counter. The holder may be a player, whose verb is "get".
public export
bareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                   GameEvent bs
bareCounterEvent dir n = CounterEvent dir Nothing n OneCounter Nothing Nothing

||| "Whenever one or more counters are put on …" / "if one or more
||| counters would be put on …" / "if you would get one or more
||| counters": the kind-blind batch, on either holder.
public export
manyBareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                       GameEvent bs
manyBareCounterEvent dir n =
  CounterEvent dir Nothing n ManyCounters Nothing Nothing

||| "If an effect would put one or more counters on …": the kind-blind
||| batch with its cause voiced (Doubling Season).
public export
manyCountersPutByEffect : (n : Noun bs Object) -> GameEvent bs
manyCountersPutByEffect n =
  CounterEvent CounterPut Nothing n ManyCounters Nothing (Just AnEffect)

||| "If <player> would put one or more counters on …": the kind-blind batch
||| with its agent voiced (Doc Samson).
public export
manyBareCountersPutBy : (who : Noun bs Player) -> (n : Noun bs Object) ->
                        {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
manyBareCountersPutBy who n =
  CounterEvent CounterPut Nothing n ManyCounters (Just who) Nothing {ag}

||| "When the last <kind> counter is removed from … by <player>."
public export
lastCounterRemovedBy : (kind : CounterKind) -> (n : Noun bs Object) ->
                       (who : Noun bs Player) ->
                       {auto 0 sc : counterScope kind = Object} ->
                       {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
lastCounterRemovedBy kind n who =
  LastCounterRemoved kind n (Just who) {sc} {ag}

||| "Choose <noun>": a choice with no chooser named, so you choose.
public export
choose : {k : Kind} -> (n : Noun bs k) ->
         {auto 0 ch : ChoiceClause (the (Maybe (Noun bs Player)) Nothing) n} ->
         Effect bs
choose n = Choose n Nothing {ch}

||| "<player> chooses …": a choice made by someone other than you.
public export
chooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
          {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
chooses who n = Choose n (Just who) {ch}

||| "Discard [amt] cards": the counted wrapper over `discard`, in the
||| canonical iterated-singular form -- one pass per card, each choosing
||| from the hand and discarding what it chose. A shortfall needs no
||| special arm: a pass whose choice finds nothing does nothing, which is
||| what [CR#609.3] asks for.
-- sort-only, as `discardsACard`: an owned-hand expansion needs a
-- subject-read noun the vocabulary doesn't have yet.
public export
aCardInHand : Noun bs Object
aCardInHand = a (InZone handZ)

||| The context one pass of `discardN` reads: the card that pass chose.
public export
handPick : (bs : Bindings) -> Bindings
handPick bs = nomIntro {bs} (aCardInHand {bs})

public export
discardN : (amt : Amount bs) ->
           {auto 0 pk : countWord CardW (handPick (amtIntro amt)) = 1} ->
           {auto 0 dz : zoneOfThat CardW (handPick (amtIntro amt)) = Just Hand} ->
           Effect bs
discardN amt =
  Repeated amt (Sequentially [ choose (aCardInHand)
                             , discard (That CardW {ok = pk})
                                       {dk = DiscardTracked {z = dz}} ])

||| "it", read at the label that stamped its referent: the same fact the
||| participle read carries, spelled as the pronoun. A rider or a
||| following sentence that names what its own clause acted on writes
||| this rather than the bare `It`, and the label it names is the
||| clause's own -- so "Destroy target creature. It can't be
||| regenerated." states, at the site, that the pronoun reads the
||| DESTROYED permanent [CR#608.2c].
public export
itVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
           {auto 0 ok : countVerbedIt v bs = 1} -> Noun bs Object
itVerbed v = ItVerbed v {kn} {ok}

||| "them", read at the label that stamped its referents: `itVerbed`'s
||| plural twin. A clause naming the batch its own labelled action made
||| writes this where a second batch stands announced, and the label it
||| names is the clause's own.
public export
themVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
             {auto 0 ok : countVerbedThem v bs = 1} -> Noun bs Object
themVerbed v = ThemVerbed v {kn} {ok}

||| "it", read among the mentions the clause immediately before it made.
||| The segment is that clause's OWN delta, taken off the clause itself
||| rather than written out, so the coordination names its own
||| neighbour: "Tap target creature an opponent controls and put a stun
||| counter on IT" [CR#608.2c]. The preceding clause is written twice --
||| once as the coordination's member and once here -- and the two are
||| held together by the type, since the pronoun's context is that
||| member's `effIntro` and no other clause's is.
public export
itPrior : {bs : Bindings} -> (prev : Effect bs) ->
          {auto 0 sp : effIntro prev = effDelta prev ++ bs} ->
          {auto 0 ok : countOnes Object (effDelta prev) = 1} ->
          Noun (effIntro prev) Object
itPrior prev = ItPrior (effDelta prev) bs {sp} {ok}

||| "the exiled card": the attributive singular participle anaphor.
public export
theVerbed : (v : VerbLabel) -> (w : NounWord) ->
            {auto 0 ok : countVerbed v w bs = 1} ->
            {auto 0 mk : VerbedMarkingOk v Attributive} -> Noun bs (kindOfW w)
theVerbed v w = TheVerbed v w Attributive {ok} {mk}

||| "the creature destroyed this way": the marked singular participle
||| anaphor, `thoseVerbedThisWay`'s twin. A printed line writes the
||| indefinite article here ("A creature destroyed this way can't be
||| regenerated") because the statement is general in English while the
||| clause it rides destroyed one thing; the article is spelling and the
||| gate is the uniqueness the clause guarantees.
public export
theVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                   {auto 0 ok : countVerbed v w bs = 1} ->
                   {auto 0 mk : VerbedMarkingOk v ThisWay} -> Noun bs (kindOfW w)
theVerbedThisWay v w = TheVerbed v w ThisWay {ok} {mk}

||| "those exiled cards": the attributive plural participle anaphor.
public export
thoseVerbed : (v : VerbLabel) -> (w : NounWord) ->
              {auto 0 ok : countManyVerbed v w bs = 1} ->
              {auto 0 mk : VerbedMarkingOk v Attributive} -> Noun bs (kindOfW w)
thoseVerbed v w = ThoseVerbed v w Attributive {ok} {mk}

||| "those cards destroyed this way": the marked plural anaphor.
public export
thoseVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                     {auto 0 ok : countManyVerbed v w bs = 1} ->
                     {auto 0 mk : VerbedMarkingOk v ThisWay} -> Noun bs (kindOfW w)
thoseVerbedThisWay v w = ThoseVerbed v w ThisWay {ok} {mk}

||| "Take an extra <part>": an added turn part with no successor named.
public export
additionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                 (count : Amount bs) ->
                 {auto 0 ad : AddedPart part} ->
                 {auto 0 an : AnchorPart anchor} -> Effect bs
additionalPart part anchor count = AdditionalPart part anchor count Nothing {ad} {an}

||| "… followed by <part>": an added turn part with a successor.
public export
additionalPartThen : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) -> (next : TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AnchorPart anchor} ->
                     {auto 0 fb : FollowerPart (Just next)} -> Effect bs
additionalPartThen part anchor count next =
  AdditionalPart part anchor count (Just next) {ad} {an} {fb}

||| "When <event>, …": a delayed trigger with no span written.
public export
delayed : (ev : GameEvent bs) -> (eff : Effect (delayedCtx [] ev)) -> Effect bs
delayed ev eff = Delayed ev [] Nothing eff

||| "When <event> this turn, …": a delayed trigger with an explicit span.
public export
delayedWithin : (ev : GameEvent bs) -> (span : Duration bs) ->
                (eff : Effect (delayedCtx [] ev)) ->
                {auto 0 so : DelaySpanOk (Just span)} -> Effect bs
delayedWithin ev span eff = Delayed ev [] (Just span) eff {so}

||| "a color", "a creature type": the quality noun over its whole domain.
public export
quality : (q : QualitySort) -> Predicate bs (Quality q)
quality q = QualityNoun q Nothing

||| "a creature type other than Wall": a quality noun with a choice domain.
public export
qualityFrom : (q : QualitySort) -> (d : ChoiceDomain (QSort q)) -> Predicate bs (Quality q)
qualityFrom q d = QualityNoun q (Just d)

||| "As … enters, choose a color."
public export
entersChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 StaticEffect bs
entersChoosing n q = EntersChoice n (QSort q) Nothing {zn}

||| "As … enters, choose a color other than red."
public export
entersChoosingFrom : (n : Noun bs Object) -> (q : QualitySort) ->
                     (d : ChoiceDomain (QSort q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
entersChoosingFrom n q d = EntersChoice n (QSort q) (Just d) {zn}

||| "As … enters, choose a player." / "… choose an opponent."
public export
entersChoosingPlayer : (n : Noun bs Object) ->
                       (d : Maybe (ChoiceDomain PlayerC)) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       StaticEffect bs
entersChoosingPlayer n d = EntersChoice n PlayerC d {zn}

||| "As this Equipment becomes attached to a creature, choose a color."
public export
attachChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 StaticEffect bs
attachChoosing n q = AttachChoice n (QSort q) Nothing {zn}

||| "this Siege": the self-reference read at a subtype.
public export
thisSiege : Noun bs Object
thisSiege = AsType Battle This (Just (battleType "Siege"))

||| "… that was dealt damage this turn": the bare lookback description.
public export
happenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement bs ev k)) Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
happenedTo ev w = HappenedTo ev w Nothing {cw}

||| "… that was dealt damage by <noun> this turn": the lookback complement.
public export
happenedToInvolving : {ks : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (w : Lookback) -> (what : Noun bs kc) ->
                      {auto 0 cp : LookbackComplement ev ks kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev ks} -> Predicate bs ks
happenedToInvolving ev w what =
  HappenedTo ev w (Just (Involving what {cp})) {cw} {sb}

||| "if you cast a spell this turn": the bare lookback condition.
public export
happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
           (w : Lookback) ->
           {auto 0 cw : ComplementWritten
                          (the (Maybe (EventComplement (nomIntro who) ev k))
                               Nothing)} ->
           {auto 0 sb : LookbackSubject ev k} -> Condition bs
happened ev who w = Happened ev who w Nothing {cw}

||| "if you cast <noun> this turn": a lookback condition with a complement.
public export
happenedInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                    (who : Noun bs k) -> (w : Lookback) ->
                    (what : Noun (nomIntro who) kc) ->
                    {auto 0 cp : LookbackComplement ev k kc} ->
                    {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                    {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedInvolving ev who w what =
  Happened ev who w (Just (Involving what {cp})) {cw} {sb}

||| "the number of spells you cast this turn": the bare counted lookback.
public export
eventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
             (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement (nomIntro who) ev k))
                                 Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCount ev who w = EventCount ev who w Nothing {cw}

||| "the number of <noun> you cast this turn": a counted lookback.
public export
eventCountInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (who : Noun bs k) -> (w : Lookback) ->
                      (what : Noun (nomIntro who) kc) ->
                      {auto 0 cp : LookbackComplement ev k kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountInvolving ev who w what =
  EventCount ev who w (Just (Involving what {cp})) {cw} {sb}

||| "if you haven't cast a spell from your hand this turn": a lookback
||| condition whose complement names the event's origin zone beside its
||| participant.
public export
happenedFrom : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
               (who : Noun bs k) -> (w : Lookback) ->
               (what : Noun (nomIntro who) kc) ->
               (src : EventSource (nomIntro who)) ->
               {auto 0 cp : LookbackComplement ev k kc} ->
               {auto 0 pl : So (complementPlain
                                  (Just (Involving {bs = nomIntro who} what {cp})))} ->
               {auto 0 zo : So (lookbackSourceOk ev src)} ->
               {auto 0 cw : ComplementWritten
                              (Just (FromZones src
                                       (Just (Involving what {cp})) {pl} {ok = zo}))} ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedFrom ev who w what src =
  Happened ev who w
    (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb}

||| "for each time you've cast your commander from the command zone this
||| game": `happenedFrom`'s counted twin, the commander tax's readback
||| [CR#903.8].
public export
eventCountFrom : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                 (who : Noun bs k) -> (w : Lookback) ->
                 (what : Noun (nomIntro who) kc) ->
                 (src : EventSource (nomIntro who)) ->
                 {auto 0 cp : LookbackComplement ev k kc} ->
                 {auto 0 pl : So (complementPlain
                                    (Just (Involving {bs = nomIntro who} what {cp})))} ->
                 {auto 0 zo : So (lookbackSourceOk ev src)} ->
                 {auto 0 cw : ComplementWritten
                                (Just (FromZones src
                                         (Just (Involving what {cp})) {pl} {ok = zo}))} ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountFrom ev who w what src =
  EventCount ev who w
    (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb}

||| "At the beginning of enchanted player's upkeep, …": a turn part
||| possessed by a noun rather than by a quantifier word.
public export
beginningOfPossessed : (part : TurnPart) -> (poss : Noun bs Player) ->
                       {auto 0 pn : PossessorNoun poss} ->
                       {auto 0 pu : PartTriggerable part (ByNoun poss {pn})} ->
                       GameEvent bs
beginningOfPossessed part poss = BeginningOf part (ByNoun poss {pn}) {pu}

||| "<noun> becomes <designation>": a conferral with no span written.
public export
gainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                   (w : GivingWarrant d) ->
                   {auto 0 sc : designationScope d = HeldBy k} ->
                   {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
gainsDesignation n d w = GainsDesignation n d w Nothing {sc} {zn}

||| "Monstrosity N" [CR#701.37a]: the keyword action spells its own
||| expansion body, and that body is the only place `Monstrous` is
||| conferred.
public export
monstrosity : {bs : Bindings} -> (amt : Amount bs) ->
              Effect bs
monstrosity amt =
  -- [CR#701.37a] reads the gate over "this permanent", so the bare self
  -- mention is the condition's subject; the counters go on the creature.
  If (notSo (Matches This (HasDesignation Monstrous)))
     (Sequentially [ PutCounters amt (PrintedKind plusOnePlusOne) thisCreature
                   , GainsDesignation thisCreature Monstrous
                                      (InExpansionOf MonstrosityW) Nothing ])
     Nothing

||| Ascend's expansion body [CR#702.131a]: "you get the city's blessing
||| for the rest of the game."
public export
getsCitysBlessing : Effect bs
getsCitysBlessing =
  GainsDesignation You CitysBlessing (InExpansionOf AscendW) (Just RestOfGame)

||| Saddle's expansion body [CR#702.171a]: "This permanent becomes
||| saddled until end of turn."
public export
becomesSaddled : Effect bs
becomesSaddled =
  GainsDesignation (AsType Artifact This Nothing) Saddled (InExpansionOf SaddleW) (Just untilEndOfTurn)

||| "your commander" [CR#903.3]: the card-scope designation read as a
||| possessed noun.
public export
yourCommander : Noun bs Object
yourCommander = Designated CommanderD You

||| "there is no monarch" [CR#725.1]: the designation's absence check.
public export
thereIsNo : (d : Designation) ->
            {auto 0 sc : designationScope d = HeldBy Player} ->
            {auto 0 at : So (designationChecked d)} -> Condition bs
thereIsNo d = NoHolder d {sc} {at}

||| "If [c], [e]."
public export
ifThen : (c : Condition bs) -> Effect (condIntro c) -> Effect bs
ifThen c e = If c e Nothing

||| "[se] as long as [c]."
public export
onlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
            {auto 0 nn : NotConditional se} -> StaticEffect bs
onlyWhile se c = OnlyWhile se c AsLongAs {nn}

||| "[se] unless [c]."
public export
onlyUnless : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
             {auto 0 nn : NotConditional se} -> StaticEffect bs
onlyUnless se c = OnlyWhile se (NotCond c) Unless {nn}

||| "While you're searching your library, you may cast <what> from <zone>."
public export
mayCastFromWhileSearching : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                            (from : ZoneExpr (nomIntro what)) ->
                            {auto 0 pz : PlaySource (nounZone what) (Just from) False} ->
                            {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFromWhileSearching who what from =
  MayPlay who what Cast (Just from) Nothing Nothing (Just WhileSearchingLibrary) {pz} {cv}

||| "N1—N2": a results table's two-ended range [CR#706.3a].
public export
fromTo : Nat -> Nat -> Quantity bs
fromTo lo hi = Range (Just lo) (Just hi)

||| "Flip a coin." [CR#705.1]
public export
flipACoin : Effect bs
flipACoin = FlipCoins You (FlipCount (Lit 1))

||| "Flip [n] coins."
public export
flipCoins : (n : Nat) -> Effect bs
flipCoins n = FlipCoins You (FlipCount (Lit n))

||| "If you win the flip, …" [CR#705.2]
public export
youWinTheFlip : {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
youWinTheFlip = FlipCalled You WinsFlip {fl}

||| "If you lose the flip, …" [CR#705.2]
public export
youLoseTheFlip : {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
youLoseTheFlip = FlipCalled You LosesFlip {fl}

||| "If the coin comes up heads, …", "If it comes up tails, …"
||| [CR#705.2] — the reading no player wins.
public export
comesUp : (face : CoinFace) -> {auto 0 fl : So (coinFlipInScope bs)} ->
          Condition bs
comesUp face = FlipFace face {fl}

||| "Roll a d[sides]." — the same construction as "Roll a [sides]-sided
||| die", which is the other spelling [CR#706.1a].
public export
rollADie : (sides : Nat) -> {auto 0 nz : IsSucc sides} -> Effect bs
rollADie sides = RollDice You (Lit 1) (SidesOf sides {nz})

||| "Roll [count] d[sides]."
public export
rollDice : (count : Nat) -> (sides : Nat) -> {auto 0 nz : IsSucc sides} ->
           Effect bs
rollDice count sides = RollDice You (Lit count) (SidesOf sides {nz})

||| "the result" [CR#706.2]
public export
theResult : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theResult = TheResult {ok}

||| One striation of a results table, "[results] | [effect]" [CR#706.3a].
public export
rollRow : (results : Quantity bs) -> (e : Effect bs) ->
          {auto 0 nz : NonZeroQ results} ->
          {auto 0 wf : WellFormedQ results} ->
          {auto 0 lt : So (quantLiteral results)} -> RollRow bs
rollRow results e = MkRollRow results e {nz} {wf}

||| The results table that reads a roll already written [CR#706.3].
public export
resultsTable : (rows : List (RollRow bs)) ->
               {auto 0 ne : IsSucc (rowCount rows)} ->
               {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
resultsTable rows = ResultsTable rows {ne} {ok}

||| "Whenever you win a coin flip, …" [CR#705.2]
public export
youWinACoinFlip : GameEvent bs
youWinACoinFlip = FlipEvent You WinsFlip

||| "Whenever you lose a coin flip, …" [CR#705.2]
public export
youLoseACoinFlip : GameEvent bs
youLoseACoinFlip = FlipEvent You LosesFlip

||| "Whenever you roll one or more dice, …" [CR#706.7]
public export
youRollDice : GameEvent bs
youRollDice = RollsDice You ManyDice AnyDie AnyResult

||| "Whenever you roll a die, …" -- the singular determiner [CR#706.7].
public export
youRollADie : GameEvent bs
youRollADie = RollsDice You OneDie AnyDie AnyResult

||| "Whenever you roll a 4 or higher, …", "Whenever you roll a 6, …":
||| the roll header carrying a result test [CR#706.3a].
public export
youRollResultIn : (q : Quantity bs) ->
                  {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} ->
                  {auto 0 lt : So (quantLiteral q)} -> GameEvent bs
youRollResultIn q = RollsDice You OneDie AnyDie (ResultIn q {nz} {wf} {lt})

||| "Whenever you roll a die's highest natural result, …" -- the test
||| against the die's own maximum [CR#706.2,706.1a].
public export
youRollHighestNatural : GameEvent bs
youRollHighestNatural = RollsDice You OneDie AnyDie HighestNatural

||| "If you would roll one or more planar dice, ..." -- the roll header
||| narrowed to Planechase's own die [CR#901.3a].
public export
youRollPlanarDice : GameEvent bs
youRollPlanarDice = RollsDice You ManyDice PlanarDie AnyResult

||| "If you would flip a coin, ..." -- the flipping act [CR#705.1], which
||| is not either arm of the call [CR#705.2].
public export
youFlipACoin : GameEvent bs
youFlipACoin = FlipsCoin You

||| "Roll the planar die." [CR#901.3a] -- one die, as a bare flip is one
||| coin.
public export
rollThePlanarDie : Effect bs
rollThePlanarDie = RollPlanarDie You (Lit 1)

||| "Flip a coin for each [each]." [CR#705.1]
public export
flipACoinFor : {k : Kind} -> (each : Noun bs k) ->
               {auto 0 pl : nounPlur each = ManyOf} ->
               {auto 0 rk : So (kindLte k (Object \/ Player))} -> Effect bs
flipACoinFor each = FlipCoins You (FlipPer each {pl} {rk})

||| "[lo] or higher" as a results range [CR#706.3a].
public export
orHigher : Nat -> Quantity bs
orHigher lo = Range (Just lo) Nothing

||| "the total of those results" [CR#706.2]
public export
theTotal : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theTotal = TheTotal {ok}

||| "the number of coins that came up [face]" [CR#705.2]
public export
coinsThatCameUp : (face : CoinFace) ->
                  {auto 0 fl : So (coinFlipInScope bs)} -> Amount bs
coinsThatCameUp face = CoinsShowing face {fl}
