||| The keyword-ability probe ([[idris-keyword-model]]): how much of the
||| keyword taxonomy (docs/rules-taxonomy.md §10) can the type system pin?
|||
||| Findings this file demonstrates (each backed by a positive or a `failing`
||| negative below):
|||
||| 1. **Parameterization is a property of the keyword, not the use site.**
|||    Each keyword declares its parameter SHAPE as a type index; the ability
|||    layer applies exactly that shape's arguments. "Deathtouch {2}" is
|||    unrepresentable — there is no ill-formed term to write down.
||| 2. **Intrinsic is a property of the keyword** (the taxonomy's 9 abilities
|||    needing native opcodes: [CR#702.7,702.2c,702.19b,702.20a] and
|||    [CR#702.22c,702.26b,702.140,702.139a]) — a total function over the
|||    closed constructors, never an index the use site chooses. An intrinsic
|||    can still be parameterized (mutate carries a cost, [CR#702.140a]).
||| 3. **Keyword names are an open set** (set-specific keywords keep
|||    arriving); what stays CLOSED is the shape vocabulary and the intrinsic
|||    set. `Custom` carries the open tail with its shape still pinned in the
|||    type — a custom keyword misapplying its declared shape is as
|||    unrepresentable as a misapplied built-in.
||| 4. **Keyword actions are not keyword abilities**: an action (scry, mill —
|||    [CR#701.22,701.17]) is a one-shot verb in effect position and confers
|||    nothing; it shares the shape vocabulary but lives in its own family.
module Experimental

import public Core as C
import public Macros

%default total

-- ===== The parameter-shape axis =====

namespace ParamShape
  ||| Taxonomy §10's observed keyword parameters: none, N, [cost],
  ||| "N—[cost]", quality/filter, card name. (The subtype and enumerated-
  ||| label shapes ride the same mechanism; add constructors when a modeled
  ||| keyword needs them.)
  public export
  data ParamShape
    = None        -- deathtouch, flying
    | Counted     -- annihilator N, toxic N ([CR#702.86a,702.164a])
    | Costed      -- ward [cost], cycling [cost] ([CR#702.21a,702.29a])
    | CountedCost -- suspend N—[cost] ([CR#702.62a])
    | Predicated  -- protection/hexproof from [quality] ([CR#702.16b,702.11d])
    | Named       -- partner with [name] ([CR#702.124j])

||| The arguments a shape demands — the total type-function refining the
||| closed enum (the probe's discipline: shape mismatch = no term).
public export
Args : ParamShape -> Ctx -> Type
Args None        b = ()
Args Counted     b = Count b
Args Costed      b = Cost b
Args CountedCost b = (Count b, Cost b)
Args Predicated  b = Filter b AnObject
Args Named       b = String

-- ===== Keywords =====

namespace Keyword
  ||| A keyword, indexed by its declared parameter shape. The intrinsic 9
  ||| (taxonomy §10) and representative composites are closed constructors;
  ||| `Custom` is the open tail (set-specific keywords are data-driven,
  ||| never a closed engine enum) with its shape still pinned in the type.
  public export
  data Keyword : ParamShape -> Type where
    -- the 9 intrinsic abilities (native opcodes; see isIntrinsic)
    FirstStrike  : Keyword None
    DoubleStrike : Keyword None
    Deathtouch   : Keyword None
    Trample      : Keyword None
    Vigilance    : Keyword None
    Banding      : Keyword None
    Phasing      : Keyword None
    Mutate       : Keyword Costed  -- an intrinsic CAN be parameterized [CR#702.140a]
    Companion    : Keyword None    -- its deck-condition parameter is outside this probe
    -- representative composites (macro sugar over the grammar)
    Flying        : Keyword None
    Reach         : Keyword None   -- the marker keyword: flying's predicate names it
    Menace        : Keyword None
    Haste         : Keyword None
    Lifelink      : Keyword None
    Ward          : Keyword Costed
    Cycling       : Keyword Costed
    Equip         : Keyword Costed
    Flashback     : Keyword Costed
    Annihilator   : Keyword Counted
    Toxic         : Keyword Counted
    Suspend       : Keyword CountedCost
    Protection    : Keyword Predicated
    HexproofFrom  : Keyword Predicated
    PartnerWith   : Keyword Named
    -- the open tail: name + declared shape ("Warp {2}{R}", "Firebending 1")
    Custom : (name : String) -> (shape : ParamShape) -> Keyword shape

||| Intrinsic = needs a native engine opcode (taxonomy §10's 9) — a total
||| function over the keyword, NOT a choice at the ability layer. Everything
||| else (including every `Custom`) composes from the grammar.
public export
isIntrinsic : Keyword p -> Bool
isIntrinsic FirstStrike  = True
isIntrinsic DoubleStrike = True
isIntrinsic Deathtouch   = True
isIntrinsic Trample      = True
isIntrinsic Vigilance    = True
isIntrinsic Banding      = True
isIntrinsic Phasing      = True
isIntrinsic Mutate       = True
isIntrinsic Companion    = True
isIntrinsic _            = False

-- ===== The ability layer =====

namespace KeywordUse
  ||| Applying a keyword's declared shape — the ONLY way to use a keyword.
  ||| `KA Deathtouch ()` and `KA Ward (Mana [^2])` typecheck;
  ||| `KA Deathtouch (Mana [^2])` has no type.
  |||
  ||| Contrast Core's `KeywordSpec`/`KeywordAbility`: there the enum is flat,
  ||| parameters ride ad-hoc per constructor (`Hexproof (Maybe (Filter b
  ||| AnObject))`), and costs cannot ride the spec at all ("`KeywordSpec`
  ||| precedes `Cost`" — Morph/Flashback are bare tags whose costs live in
  ||| their desugared abilities). The shape index removes both weaknesses:
  ||| every parameter kind (cost included) rides the spec, and a
  ||| misparameterized keyword has no term. Adopting this in Core would
  ||| replace `KeywordSpec`'s constructors with shape-indexed ones and let
  ||| `Bare`/`Composite` take a `KeywordUse` instead.
  public export
  data KeywordUse : Ctx -> Type where
    KA : {shape : ParamShape} -> Keyword shape -> Args shape b -> KeywordUse b

-- ===== Keyword actions =====

namespace KeywordAction
  ||| A keyword ACTION is a one-shot verb ([CR#701]), not a conferred
  ||| ability — it shares the shape vocabulary but is used in effect
  ||| position and confers nothing.
  public export
  data KeywordAction : ParamShape -> Type where
    Scry        : KeywordAction Counted
    Mill        : KeywordAction Counted
    Surveil     : KeywordAction Counted
    Investigate : KeywordAction None
    Explore     : KeywordAction None

  ||| A keyword action applied in effect position.
  public export
  data ActionUse : Ctx -> Type where
    Use : {shape : ParamShape} -> KeywordAction shape -> Args shape b -> ActionUse b

-- ===== Positives (must typecheck) =====

kDeathtouch : KeywordUse Base
kDeathtouch = KA Deathtouch ()

kWard2 : KeywordUse Base
kWard2 = KA Ward (Mana [^2])

kToxic1 : KeywordUse Base
kToxic1 = KA Toxic (^1)

kSuspend : KeywordUse Base
kSuspend = KA Suspend (^4, Mana [^1, ^Blue])

kProtection : KeywordUse Base
kProtection = KA Protection creature

kCustomCosted : KeywordUse Base
kCustomCosted = KA (Custom "Warp" Costed) (Mana [^2, ^Red])

aScry2 : ActionUse Base
aScry2 = Use Scry (^2)

-- ===== Negatives (each `failing` block must NOT typecheck) =====

-- A bare keyword cannot take a cost: "Deathtouch {2}" has no type.
failing "Mismatch between: Cost ?b and ()"
  bad : KeywordUse Base
  bad = KA Deathtouch (Mana [^2])

-- A costed keyword cannot be bare: "Ward" with no cost has no type.
failing "Cost Base"
  bad : KeywordUse Base
  bad = KA Ward ()

-- A counted keyword cannot take a predicate.
failing "Mismatch between: Filter ?b AnObject and Count ?b"
  bad : KeywordUse Base
  bad = KA Toxic creature

-- The open tail obeys its declared shape too: a Costed custom keyword
-- cannot be applied bare.
failing "Cost Base"
  bad : KeywordUse Base
  bad = KA (Custom "Warp" Costed) ()

-- A keyword action is not a keyword ability: `KA Scry …` has no type.
failing "Keyword ?shape"
  bad : KeywordUse Base
  bad = KA Scry (^2)
