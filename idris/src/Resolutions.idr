||| The per-card RESOLUTION-TABLE export ([[idris-tables-fixtures-v2]]): a
||| value-level walk over selected `Cards.idr` encodings that re-runs the
||| Core resolution machinery (`candidate`/`innermostBinder`/`innermostFrame`
||| /`compat` — the same functions the type-level proofs reduce through) at
||| every anaphor read, and renders each resolved binding in the Rust
||| elaborator's `--dump` line format:
|||
|||     <path>: <spelling> -> #<depth> <antecedent description>
|||
||| `emitTables` writes one fixture per exported card to
||| `crates/deckmaste_cards/tests/resolution/<Name>.txt`; the Rust
||| `resolution` test replays the CANON RON twin of the same card
||| (`plugins/canon/cards/<Name>.ron`) through
||| `elaborate_with_resolutions` and demands byte-for-byte agreement on the
||| anaphor rows — the mechanical both-ways gate (Idris exports, Rust
||| replays) that upgrades the pin-by-proof-and-byte-identical-tables story.
|||
||| SCOPE: the walker covers the constructs the exported corpus uses (Spell
||| abilities; Targeted/Sequence/Each/With/May/If/MustPay/MayPay/Delayed
||| bodies; the common verbs). Uncovered constructs walk to NO rows — the
||| safe direction: if the Rust side records a row there, the byte
||| comparison turns RED, never silently green-with-a-gap. Extend the walker
||| as the export list grows.
module Resolutions

import Core
import Macros
import Cards

%default total

-- ---------------------------------------------------------------------------
-- the walker stack: Core's `Ante` + the render DECORATION the Rust walker's
-- `Site` carries but Core's doesn't (the announce-slot index, the event-role
-- tag). Every transform below is the pair-level image of the Core transform
-- (the Core site predicates applied to `fst`), so the projection `map fst`
-- of a walker stack IS the Core stack the resolve functions see.
-- ---------------------------------------------------------------------------

public export
data Deco = DSlot Nat | DRole String | DNone

public export
WStack : Type
WStack = List (Ante, Deco)

public export
record WCtx where
  constructor MkWCtx
  pairs : WStack
  caps : EventCaps

export
wBase : WCtx
wBase = MkWCtx [] NoCaps

-- push order mirrors `Core.pushAntes` (the LAST listed lands nearest).
wPush : WStack -> WCtx -> WCtx
wPush ps w = MkWCtx (reverse ps ++ w.pairs) w.caps

wPlain : List Ante -> WStack
wPlain = map (\a => (a, DNone))

-- `Core.bindTargets`: the slot antecedents REPLACE any outer ones.
wBindTargets : WStack -> WCtx -> WCtx
wBindTargets slots w =
  MkWCtx (reverse slots ++ filter (notTargetSlotA . fst) w.pairs) w.caps

-- `Core.unbindTargets` ([CR#603.7c]).
wUnbindTargets : WCtx -> WCtx
wUnbindTargets w = MkWCtx (filter (notTargetSlotA . fst) w.pairs) w.caps

-- `Core.bindThat`.
wBindThat : (Ante, Deco) -> WCtx -> WCtx
wBindThat pr = wPush [pr]

-- `Core.bindIt`: re-sited at Loop/One; clears the `Allot` share.
wBindIt : (Ante, Deco) -> WCtx -> WCtx
wBindIt (a, d) w =
  MkWCtx ((MkAnte a.sort a.kind One Loop a.expectedZone a.label, d)
            :: filter (notAllotA . fst) w.pairs)
         w.caps

-- `Core.bindAllot`.
wBindAllot : (Ante, Deco) -> WCtx -> WCtx
wBindAllot (a, d) w =
  MkWCtx ((MkAnte Amount Anything One Allot Nothing Nothing, DNone)
            :: (MkAnte a.sort a.kind One Loop a.expectedZone a.label, d)
            :: w.pairs)
         w.caps

-- `Core.bindEvent`: role antecedents SHADOW an outer event's.
wBindEvent : EventCaps -> WStack -> WCtx -> WCtx
wBindEvent caps roles w =
  MkWCtx (reverse roles ++ filter (notEventRoleA . fst) w.pairs) caps

-- the role decos, in `Core.roleAntes` order (object, patient, actor,
-- amount, defender) — zipped against the roles the caps actually push.
wRoleDecos : EventCaps -> List Deco
wRoleDecos (MkEventCaps o a m p d) =
  (if o then [DRole "Object"] else [])
  ++ (case p of Just _ => [DRole "Patient"]; Nothing => [])
  ++ (if a then [DRole "Actor"] else [])
  ++ (if m then [DRole "Amount"] else [])
  ++ (if d then [DRole "Defender"] else [])

wRoles : EventCaps -> List Ante -> WStack
wRoles caps roles = zip roles (wRoleDecos caps)

-- ---------------------------------------------------------------------------
-- rendering, in the Rust `Ante::describe` / `Site` Debug spellings
-- ---------------------------------------------------------------------------

typeDebug : Type_ -> String
typeDebug Artifact = "Artifact"
typeDebug Battle = "Battle"
typeDebug Creature = "Creature"
typeDebug Enchantment = "Enchantment"
typeDebug Instant = "Instant"
typeDebug Kindred = "Kindred"
typeDebug Land = "Land"
typeDebug Planeswalker = "Planeswalker"
typeDebug Sorcery = "Sorcery"

sortDebug : Sort -> String
sortDebug Player = "Player"
sortDebug Card = "Card"
sortDebug Token = "Token"
sortDebug Spell = "Spell"
sortDebug StackObject = "StackObject"
sortDebug Permanent = "Permanent"
sortDebug (OfType t) = "OfType(" ++ typeDebug t ++ ")"
sortDebug Amount = "Amount"
sortDebug Pile = "Pile"

cardDebug : Cardinality -> String
cardDebug One = "One"
cardDebug Many = "Many"

zoneDebug : Zone -> String
zoneDebug Battlefield = "Battlefield"
zoneDebug Command = "Command"
zoneDebug Exile = "Exile"
zoneDebug Graveyard = "Graveyard"
zoneDebug Hand = "Hand"
zoneDebug Library = "Library"
zoneDebug Sideboard = "Sideboard"
zoneDebug Stack = "Stack"

-- the Rust `Site` Debug: the deterministic Frame and the resolution-time
-- choice both spell "Chosen" (walk.rs marks frames with `binder: true`).
siteDebug : Site -> Deco -> String
siteDebug TargetSlot (DSlot i) = "TargetSlot(" ++ show i ++ ")"
siteDebug TargetSlot _ = "TargetSlot(?)"
siteDebug Product _ = "Product"
siteDebug EventRole (DRole r) = "EventRole(" ++ r ++ ")"
siteDebug EventRole _ = "EventRole(?)"
siteDebug Loop _ = "Loop"
siteDebug Allot _ = "Allot"
siteDebug Chosen _ = "Chosen"
siteDebug Frame _ = "Chosen"

describePair : (Ante, Deco) -> String
describePair (a, d) =
  cardDebug a.card ++ " " ++ sortDebug a.sort ++ " at " ++ siteDebug a.site d
  ++ (case a.expectedZone of
        Nothing => ""
        Just z => ", expected zone " ++ zoneDebug z)
  ++ (case a.label of
        Nothing => ""
        Just l => ", label " ++ show l)

-- ---------------------------------------------------------------------------
-- value-level resolution with DEPTH (the Rust `#n` — index from nearest):
-- Core's `candidate` filter + the R2 singleton rule, over the walker stack.
-- The exported cards TYPECHECK, so every read resolves; a `Nothing` here
-- renders no row and the Rust agreement turns red — the safe direction.
-- ---------------------------------------------------------------------------

indexed : WStack -> List (Nat, (Ante, Deco))
indexed = go 0
  where
    go : Nat -> WStack -> List (Nat, (Ante, Deco))
    go _ [] = []
    go i (p :: ps) = (i, p) :: go (S i) ps

wFind : (Ante -> Bool) -> WStack -> Maybe (Nat, (Ante, Deco))
wFind f ps = find (\ip => f (fst (snd ip))) (indexed ps)

wResolveStack : Maybe Sort -> Cardinality -> WStack -> Maybe (Nat, (Ante, Deco))
wResolveStack w cd ps =
  case filter (\ip => candidate w cd (fst (snd ip))) (indexed ps) of
    [x] => Just x
    _ => Nothing

wResolveLabel : String -> Cardinality -> WStack -> Maybe (Nat, (Ante, Deco))
wResolveLabel l cd ps =
  case find (\ip => sameLabel (Just l) ((fst (snd ip)).label)) (indexed ps) of
    Just (i, pr) => if sameCard cd (fst pr).card then Just (i, pr) else Nothing
    Nothing => Nothing

row : (path : String) -> (spelling : String) -> (Nat, (Ante, Deco))
   -> (suffix : String) -> String
row path spelling (depth, pr) suffix =
  path ++ ": " ++ spelling ++ " -> #" ++ show depth ++ " " ++ describePair pr ++ suffix

rowM : String -> String -> Maybe (Nat, (Ante, Deco)) -> List String
rowM path spelling Nothing = []
rowM path spelling (Just x) = [row path spelling x ""]

-- ---------------------------------------------------------------------------
-- value-level KIND/SORT recovery. `binderAnte`/`filterSort` take the kind
-- as a runtime-RELEVANT implicit, but `With`/`Each` store their binder's
-- kind/cardinality ERASED — so the walker mirrors them at the value level,
-- recovering the kind from the constructors where it is pinned (`Nothing`
-- where it genuinely rides an erased index — the anaphors). The kind never
-- reaches the rendered rows (describe prints card/sort/site only) and
-- Core's `candidate` is kind-free, so the fallback is inert; the 0-pins
-- below hold the mirrors definitionally equal to the Core functions on
-- representative binders.
-- ---------------------------------------------------------------------------

mutual
  wRefKind : {0 b : Ctx} -> {0 k : RefKind} -> Reference b k -> Maybe RefKind
  wRefKind This = Just AnObject
  wRefKind (AttachHostOf _) = Just AnObject
  wRefKind (AttachedTo _) = Just AnObject
  wRefKind EventObject = Just AnObject
  wRefKind (ChosenObject) = Just AnObject
  wRefKind You = Just APlayer
  wRefKind (ControllerOf _) = Just APlayer
  wRefKind (OwnerOf _) = Just APlayer
  wRefKind EventActor = Just APlayer
  wRefKind DefendingPlayer = Just APlayer
  wRefKind ChosenPlayer = Just APlayer
  wRefKind (A of_) = wPredKind of_
  wRefKind (Single sel) = wSelKind sel
  wRefKind _ = Nothing

  wPredKind : {0 b : Ctx} -> {0 k : RefKind} -> Predicate b k -> Maybe RefKind
  wPredKind (SameAs r) = wRefKind r
  wPredKind (HasCounter c) = Just (counterScope c)
  wPredKind (HasDesignation d) = Just (designationScope d)
  wPredKind (PlayerStatCmp _ _ _) = Just APlayer
  wPredKind (Controls _) = Just APlayer
  wPredKind Anyone = Just APlayer
  wPredKind OpponentOf = Just APlayer
  wPredKind TeammateOf = Just APlayer
  wPredKind (And (q :: _)) = wPredKind q
  wPredKind (And []) = Nothing
  wPredKind (Not q) = wPredKind q
  wPredKind (Or {ks} _) = Just (foldr (\/) Empty ks)
  wPredKind _ = Just AnObject

  wSelKind : {0 b : Ctx} -> {0 k : RefKind} -> Selection b k -> Maybe RefKind
  wSelKind (SelectAll p) = wPredKind p
  wSelKind (Random _ p) = wPredKind p
  wSelKind (Union (g :: _)) = wSelKind g
  wSelKind (TopOfLibrary _) = Just AnObject
  wSelKind (BottomOfLibrary _) = Just AnObject
  wSelKind (Pick _ _) = Just AnObject
  wSelKind _ = Nothing

  -- `filterSort`, kind recovered by value ([CR#110.1,108.2]).
  wPredSort : {0 b : Ctx} -> {0 k : RefKind} -> Predicate b k -> Sort
  wPredSort p = case wPredKind p of
    Just APlayer => Player
    _ => sortFromPins (pins p)

  -- `selectionSort`, likewise.
  wSelSort : {0 b : Ctx} -> {0 k : RefKind} -> Selection b k -> Sort
  wSelSort (SelectAll p) = wPredSort p
  wSelSort (Random _ p) = wPredSort p
  wSelSort (TopOfLibrary _) = Card
  wSelSort (BottomOfLibrary _) = Card
  wSelSort (Pick _ prj) = wProjSort prj
  wSelSort (Union gs) = case gs of
    [] => Permanent
    (g :: rest) => if wUnionAgrees (wSelSort g) rest then wSelSort g else Permanent
  wSelSort They = Permanent
  wSelSort (Them _) = Permanent
  wSelSort (TheGroup _) = Permanent

  wUnionAgrees : {0 b : Ctx} -> {0 k : RefKind} -> Sort -> List (Selection b k) -> Bool
  wUnionAgrees x [] = True
  wUnionAgrees x (g :: gs) = sameSort x (wSelSort g) && wUnionAgrees x gs

  wProjSort : {0 b : Ctx} -> Projection b -> Sort
  wProjSort (Project (Objects p) _) = wPredSort p
  wProjSort (Project _ _) = Permanent

-- `binderAnte`, kinds/cardinalities recovered from the constructors
-- ([CR#608.2d,701.23,400.7j]); the pushed antecedent the walker binds.
wBinderAnte : {0 b : Ctx} -> {0 cd : Cardinality} -> {0 k : RefKind}
           -> Bindable b cd k -> Ante
wBinderAnte (Produce act) = MkAnte (produceSort act) AnObject One Product Nothing Nothing
wBinderAnte (ChooseOne p) = MkAnte (wPredSort p) (fromMaybe Anything (wPredKind p)) One Frame Nothing Nothing
wBinderAnte (SearchOne p) = MkAnte Card (fromMaybe Anything (wPredKind p)) One Product Nothing Nothing
wBinderAnte (TheRef r) = MkAnte (kindSort (fromMaybe AnObject (wRefKind r))) (fromMaybe Anything (wRefKind r)) One Frame Nothing Nothing
wBinderAnte (Existing g) = MkAnte (wSelSort g) (fromMaybe Anything (wSelKind g)) Many Frame Nothing Nothing
wBinderAnte (Choose _ p) = MkAnte (wPredSort p) (fromMaybe Anything (wPredKind p)) Many Frame Nothing Nothing
wBinderAnte (Search _ p) = MkAnte Card (fromMaybe Anything (wPredKind p)) Many Product Nothing Nothing

-- the mirrors agree with the Core functions on representative binders — a
-- drift in either side breaks the BUILD (definitional-equality pins).
0 wBinderAntePinChoose :
  wBinderAnte (the (Bindable Base Many AnObject) (Choose (^2) (InZone Hand)))
  = binderAnte (the (Bindable Base Many AnObject) (Choose (^2) (InZone Hand)))
wBinderAntePinChoose = Refl

0 wBinderAntePinExisting :
  wBinderAnte (the (Bindable Base Many AnObject) (Existing (SelectAll (HasChar Types Creature))))
  = binderAnte (the (Bindable Base Many AnObject) (Existing (SelectAll (HasChar Types Creature))))
wBinderAntePinExisting = Refl

0 wBinderAntePinProduce :
  wBinderAnte (the (Bindable Base One AnObject) (Produce (Move This (ToZone Exile))))
  = binderAnte (the (Bindable Base One AnObject) (Produce (Move This (ToZone Exile))))
wBinderAntePinProduce = Refl

-- 0..n-1 for the slot decos.
slotIndices : List Ante -> List Nat
slotIndices as = go 0 as
  where
    go : Nat -> List Ante -> List Nat
    go _ [] = []
    go i (_ :: rest) = i :: go (S i) rest

-- ---------------------------------------------------------------------------
-- the walk. Path segments mirror walk.rs's `scoped` calls; reads record at
-- the ENCLOSING effect scope (verbs add no segment). Constructs the export
-- corpus doesn't use walk to no rows (see the module header).
-- ---------------------------------------------------------------------------

mutual
  wRef : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
      -> Reference b k -> List String
  wRef p w It =
    case wFind (\a => isBinderSite a.site) w.pairs of
      Just x => [row p "It" x " (loop element)"]
      Nothing => rowM p "It" (wResolveStack Nothing One w.pairs)
  wRef p w (That s) =
    case wFind (\a => isFrameSite a.site) w.pairs of
      Just (i, pr) =>
        if sameCard One (fst pr).card && compat s (fst pr).sort
          then [row p ("That(" ++ sortDebug s ++ ")") (i, pr) " (binder frame)"]
          else []
      Nothing => rowM p ("That(" ++ sortDebug s ++ ")")
                      (wResolveStack (Just s) One w.pairs)
  wRef p w (The l) = rowM p "The" (wResolveLabel l One w.pairs)
  wRef p w (Single sel) = wSel p w sel
  wRef p w (ControllerOf r) = wRef p w r
  wRef p w (OwnerOf r) = wRef p w r
  wRef p w (AttachHostOf r) = wRef p w r
  wRef p w (AttachedTo r) = wRef p w r
  wRef p w (A of_ {by}) =
    (case by of
       Just r => wRef p w r
       Nothing => [])
    ++ wPred p w of_
  -- This/You and the explicit event/chosen reads record no `#` rows (the
  -- Rust twins render without a depth and are filtered from the fixture).
  wRef p w _ = []

  wSel : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
      -> Selection b k -> List String
  wSel p w (SelectAll f) = wPred p w f
  wSel p w (Union gs) = wSels p w gs
  wSel p w They =
    case wFind (\a => isFrameSite a.site) w.pairs of
      Just (i, pr) =>
        if sameCard Many (fst pr).card && wildReaches (fst pr).sort
          then [row p "They" (i, pr) " (binder frame)"]
          else []
      Nothing => rowM p "They" (wResolveStack Nothing Many w.pairs)
  wSel p w (Them s) =
    case wFind (\a => isFrameSite a.site) w.pairs of
      Just (i, pr) =>
        if sameCard Many (fst pr).card && compat s (fst pr).sort
          then [row p ("Them(" ++ sortDebug s ++ ")") (i, pr) " (binder frame)"]
          else []
      Nothing => rowM p ("Them(" ++ sortDebug s ++ ")")
                      (wResolveStack (Just s) Many w.pairs)
  wSel p w (TheGroup l) = rowM p "TheGroup" (wResolveLabel l Many w.pairs)
  wSel p w (Random q f) = wQuantity p w q ++ wPred p w f
  wSel p w (TopOfLibrary c {whose}) = wCount p w c ++ wRef p w whose
  wSel p w (BottomOfLibrary c {whose}) = wCount p w c ++ wRef p w whose
  wSel p w (Pick op prj) = []

  wSels : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
       -> List (Selection b k) -> List String
  wSels p w [] = []
  wSels p w (g :: gs) = wSel p w g ++ wSels p w gs

  wPred : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
       -> Predicate b k -> List String
  wPred p w (SameAs r) = wRef p w r
  wPred p w (SameName r) = wRef p w r
  wPred p w (SharesChar _ r) = wRef p w r
  wPred p w (ExiledBy r) = wRef p w r
  wPred p w (DamagedBy r) = wRef p w r
  wPred p w (StatCmp _ _ n) = wCount p w n
  wPred p w (PlayerStatCmp _ _ n) = wCount p w n
  wPred p w (ControlledBy q) = wPred p w q
  wPred p w (OwnedBy q) = wPred p w q
  wPred p w (Controls q) = wPred p w q
  wPred p w (Targets q) = wPred p w q
  wPred p w (TargetCount _ n) = wCount p w n
  wPred p w (And qs) = wPreds p w qs
  wPred p w (Not q) = wPred p w q
  wPred p w _ = []

  wPreds : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
        -> List (Predicate b k) -> List String
  wPreds p w [] = []
  wPreds p w (q :: qs) = wPred p w q ++ wPreds p w qs

  wCount : {0 b : Ctx} -> (path : String) -> WCtx -> Count b -> List String
  wCount p w (StatOf r _) = wRef p w r
  wCount p w (ManaValueOf r) = wRef p w r
  wCount p w (Damage r) = wRef p w r
  wCount p w (CountersOn _ r) = wRef p w r
  wCount p w (PlayerStatOf r _) = wRef p w r
  wCount p w (Plus x y) = wCount p w x ++ wCount p w y
  wCount p w (Minus x y) = wCount p w x ++ wCount p w y
  wCount p w (Times x y) = wCount p w x ++ wCount p w y
  wCount p w (Half _ x) = wCount p w x
  wCount p w (Min x y) = wCount p w x ++ wCount p w y
  wCount p w (Max x y) = wCount p w x ++ wCount p w y
  wCount p w ThatMany = rowM p "ThatMany" (wResolveStack (Just Amount) One w.pairs)
  wCount p w _ = []

  wQuantity : {0 b : Ctx} -> (path : String) -> WCtx -> Quantity b -> List String
  wQuantity p w (Range lo hi) =
    (case lo of Just n => wCount p w n; Nothing => [])
    ++ (case hi of Just n => wCount p w n; Nothing => [])

  wCost : {0 b : Ctx} -> (path : String) -> WCtx -> Cost b -> List String
  wCost p w (Do a) = wAction p w a
  wCost p w (Scaled n c) = wCount p w n ++ wCost p w c
  wCost p w (Costs cs) = wCosts p w cs
  wCost p w (ManaCostOf r) = wRef p w r
  wCost p w _ = []

  wCosts : {0 b : Ctx} -> (path : String) -> WCtx -> List (Cost b) -> List String
  wCosts p w [] = []
  wCosts p w (c :: cs) = wCost p w c ++ wCosts p w cs

  wDest : {0 b : Ctx} -> (path : String) -> WCtx -> Destination b -> List String
  wDest p w (ToZone _) = []
  wDest p w (ToLibrary (FromTop n)) = wCount p w n
  wDest p w (ToLibrary (FromBottom n)) = wCount p w n

  wBindable : {0 b : Ctx} -> {0 cd : Cardinality} -> {0 k : RefKind}
           -> (path : String) -> WCtx -> Bindable b cd k -> List String
  wBindable p w (Produce act) = wAction p w act
  wBindable p w (ChooseOne f {by}) = wRef p w by ++ wPred p w f
  wBindable p w (SearchOne f {by} {whose}) = wRef p w by ++ wRef p w whose ++ wPred p w f
  wBindable p w (TheRef r) = wRef p w r
  wBindable p w (Existing g) = wSel p w g
  wBindable p w (Choose q f {by}) = wRef p w by ++ wQuantity p w q ++ wPred p w f
  wBindable p w (Search q f {by} {whose}) = wRef p w by ++ wRef p w whose ++ wQuantity p w q ++ wPred p w f

  -- walk order per verb mirrors walk.rs's `action`/`player_action` arms.
  wAction : {0 b : Ctx} -> (path : String) -> WCtx -> Action b -> List String
  wAction p w (DealDamage r n {source}) = wRef p w r ++ wCount p w n ++ wRef p w source
  wAction p w (Move r d) = wRef p w r ++ wDest p w d
  wAction p w (Destroy r) = wRef p w r
  wAction p w (Counter r) = wRef p w r
  wAction p w (Tap r) = wRef p w r
  wAction p w (Untap r) = wRef p w r
  wAction p w (RemoveAllDamage r) = wRef p w r
  wAction p w (RemoveFromCombat r) = wRef p w r
  wAction p w (Transform r) = wRef p w r
  wAction p w (PhaseOut r) = wRef p w r
  wAction p w (GrantDesignation _ r) = wRef p w r
  wAction p w (Attach x y) = wRef p w x ++ wRef p w y
  wAction p w (Unattach r) = wRef p w r
  wAction p w (Draw n {actor}) = wRef p w actor ++ wCount p w n
  wAction p w (GainLife n {actor}) = wRef p w actor ++ wCount p w n
  wAction p w (LoseLife n {actor}) = wRef p w actor ++ wCount p w n
  wAction p w (SetLifeTo n {actor}) = wRef p w actor ++ wCount p w n
  wAction p w (Discard n {actor}) = wRef p w actor ++ wCount p w n
  wAction p w (Sacrifice f {actor}) = wRef p w actor ++ wPred p w f
  wAction p w (MoveArranged g _ d) = wSel p w g ++ wDest p w d
  wAction p w (PutCounters _ n r) = wCount p w n ++ wRef p w r
  wAction p w (RemoveCounters _ n r) = wCount p w n ++ wRef p w r
  wAction p w (MoveCounters _ x y) = wRef p w x ++ wRef p w y
  wAction p w (Reveal r) = wRef p w r
  wAction p w (Copy r) = wRef p w r
  wAction p w (ChangeTarget x y) = wRef p w x ++ wRef p w y
  wAction p w (ChooseNewTargets r {by}) = wRef p w r ++ wRef p w by
  wAction p w (ControlPlayer r) = wRef p w r
  wAction p w (Composite _ e) = wEffect p w e
  wAction p w _ = []

  wSpec : {0 b : Ctx} -> {0 k : RefKind} -> (path : String) -> WCtx
       -> TargetSpec b k -> List String
  wSpec p w (Target q f) = wQuantity p w q ++ wPred p w f
  wSpec p w (As _ t) = wSpec p w t
  wSpec p w (Distinct _ t) = wSpec p w t

  wSpecs : {0 b : Ctx} -> {0 ks : List RefKind} -> (path : String) -> (i : Nat)
        -> WCtx -> All (TargetSpec b) ks -> List String
  wSpecs p i w [] = []
  wSpecs p i w (t :: ts) =
    wSpec (p ++ ".targets[" ++ show i ++ "]") w t ++ wSpecs p (S i) w ts

  wSeq : {0 b : Ctx} -> (path : String) -> (i : Nat) -> WCtx -> SeqList b -> List String
  wSeq p i w [] = []
  wSeq p i w (e :: es) =
    wEffect (p ++ ".Sequence[" ++ show i ++ "]") w e
    ++ wSeq p (S i) (wPush (wPlain (introduces e)) w) es

  wEffect : {0 b : Ctx} -> (path : String) -> WCtx -> OneShotEffect b -> List String
  wEffect p w (Sequence es) = wSeq p 0 w es
  wEffect p w (Targeted {ks} ts e) =
    let pT = p ++ ".Targeted"
        sa = slotAntes ts
        slots = zip sa (map DSlot (slotIndices sa)) in
    wSpecs pT 0 w ts ++ wEffect pT (wBindTargets slots w) e
  wEffect p w (With that e) =
    let pW = p ++ ".With" in
    wBindable pW w that
    ++ wEffect pW (wBindThat (wBinderAnte that, DNone) w) e
  wEffect p w (Act a) = wAction p w a
  wEffect p w (May e {ifDid} {ifNot}) =
    let pM = p ++ ".May" in
    wEffect pM w e
    ++ (case ifDid of
          Just e2 => wEffect (pM ++ ".if_did") (wPush (wPlain (introduces e)) w) e2
          Nothing => [])
    ++ (case ifNot of
          Just e2 => wEffect (pM ++ ".if_not") w e2
          Nothing => [])
  wEffect p w (If c t {otherwise}) =
    let pI = p ++ ".If" in
    wCond pI w c ++ wEffect pI w t
    ++ (case otherwise of
          Just e2 => wEffect pI w e2
          Nothing => [])
  wEffect p w (MayPay cost andThen {actor} {or_else}) =
    let pM = p ++ ".MayPay"
        caps = costCaps cost in
    wRef pM w actor ++ wCost pM w cost
    ++ wEffect pM (wBindEvent caps (wRoles caps (costRoles caps)) w) andThen
    ++ (case or_else of
          Just e2 => wEffect pM w e2
          Nothing => [])
  wEffect p w (MustPay cost orElse {actor}) =
    let pM = p ++ ".MustPay" in
    wRef pM w actor ++ wCost pM w cost ++ wEffect pM w orElse
  wEffect p w (Each dom e) =
    let pE = p ++ ".Each" in
    wBindable pE w dom
    ++ wEffect pE (wBindIt (wBinderAnte dom, DNone) w) e
  wEffect p w (Distribute n among e) =
    let pD = p ++ ".DivideAmong" in
    wCount pD w n ++ wBindable pD w among
    ++ wEffect pD (wBindAllot (wBinderAnte among, DNone) w) e
  wEffect p w (Delayed q e) =
    let pD = p ++ ".Delayed"
        caps = eventQueryCaps q in
    wEffect pD (wBindEvent caps (wRoles caps (queryRoles q)) (wUnbindTargets w)) e
  wEffect p w (Reflexive e) = wEffect (p ++ ".Reflexive") w e
  -- Continuously/Modal/Vote/DivideAndChoose/WithChosenValue/Conclude/
  -- AdditionalCost: no rows yet (see the module-header scope note).
  wEffect p w _ = []

  wCond : {0 b : Ctx} -> (path : String) -> WCtx -> Condition b -> List String
  wCond p w (Matches r q) = wRef p w r ++ wPred p w q
  wCond p w (Compare x _ y) = wCount p w x ++ wCount p w y
  wCond p w (TurnOf q) = wPred p w q
  wCond p w (LegallyAttached r) = wRef p w r
  wCond p w (And cs) = wConds p w cs
  wCond p w (Or cs) = wConds p w cs
  wCond p w (Not c) = wCond p w c
  wCond p w _ = []

  wConds : {0 b : Ctx} -> (path : String) -> WCtx -> List (Condition b) -> List String
  wConds p w [] = []
  wConds p w (c :: cs) = wCond p w c ++ wConds p w cs

-- one ability, at the printed-face context.
wAbility : (path : String) -> Ability Base -> List String
wAbility p (Spell e) = wEffect (p ++ ".effect") wBase e
-- Keyword/Activated/Triggered/Static/…: no rows yet (scope note above);
-- the Rust agreement test turns red if the canon twin records any.
wAbility p _ = []

wAbilities : (path : String) -> (i : Nat) -> List (Ability Base) -> List String
wAbilities p i [] = []
wAbilities p i (a :: rest) =
  wAbility (p ++ ".abilities[" ++ show i ++ "]") a ++ wAbilities p (S i) rest

wFace : (path : String) -> Face -> List String
wFace p f = wAbilities p 0 (abilities f)

export
cardRows : Card -> List String
cardRows (Normal c) = wFace "face" c
cardRows (TwoFaced _ front back) = wFace "front" front ++ wFace "back" back

-- ---------------------------------------------------------------------------
-- the export list: canon-twinned cards whose Idris and RON encodings MIRROR
-- (the module-header discipline). A card here must exist as
-- `plugins/canon/cards/<name>.ron`; the Rust test walks that twin.
-- The two remaining canon∩Cards.idr names are EXCLUDED for documented
-- structural non-mirroring: Glorious Anthem (canon spells the anthem as the
-- Rust `Modify(of: Matching(…))` static — no anaphor read; the Idris
-- encoding is `Each … (Modify It …)`) and Pacifism (canon rides the Rust
-- `Keyword(Enchant(…))` shape; the Idris `enchant` macro is three
-- abilities). Re-encode a side to mirror before exporting either.
-- ---------------------------------------------------------------------------

export
resolutionFixtures : List (String, Card)
resolutionFixtures =
  [ ("Lightning Bolt", card_LightningBolt)
  , ("Pyroclasm", card_Pyroclasm)
  , ("Grizzly Bears", card_GrizzlyBears)
  , ("Typhoid Rats", card_TyphoidRats)
  , ("Giant Spider", card_GiantSpider)
  , ("Boggart Brute", card_BoggartBrute)
  , ("Brainstorm", card_Brainstorm)
  , ("Mana Leak", card_ManaLeak)
  ]

export
renderResolutions : (name : String) -> Card -> String
renderResolutions name c =
  "# GENERATED by idris/src/Resolutions.idr (emit-tables) - do not edit.\n"
  ++ "# Anaphor resolution table for \"" ++ name ++ "\": every `<path>: <read> -> #<depth> <antecedent>`\n"
  ++ "# row the Rust elaborator must reproduce byte-for-byte (tests/resolution.rs).\n"
  ++ concat (map (++ "\n") (cardRows c))
