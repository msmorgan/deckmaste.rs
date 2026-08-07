||| The translation guide between the workbench's target shape
||| (`Experimental`, in-situ encoded English) and the verifier's current
||| shape (`Semantics`, prenex binders + two entity channels). Each pair
||| below is one fixture: the new form quoted in the comment, the old form
||| as a CHECKED term (spellings taken from `Cards.idr`/`Spec.idr`), and
||| the transformation rules it exercises. This is the seed spec for the
||| lowering changes that accompany the macro/card rewrite — the
||| rearrangement work the new shape no longer does at authoring time.
|||
||| The transformation inventory so far (T-rules referenced per pair):
|||
||| - **T1 hoist**: collect in-situ `Target` nominals in textual order
|||   into the scope's prenex `Targeted` slot list; indices are collection
|||   order; collection stops at announcement boundaries ([CR#601.2c] —
|||   delayed clauses, modes).
||| - **T2 read-back**: the in-situ nominal's own argument position
|||   becomes the positional read `Target n`.
||| - **T3 other**: the `Other` modifier becomes `Distinct [every earlier
|||   compatible slot index]` on the later slot.
||| - **T4 pronoun**: an anaphor whose antecedent is an announced slot
|||   becomes a positional `Target n`; one whose antecedent is a
|||   product/choice frame stays `It`/`That sort` on the stack channel.
||| - **T5 indefinite**: `A pred` becomes the `With (ChooseOne pred) …`
|||   frame binder; its later mentions read the frame ([CR#608.2d]).
||| - **T6 sequence**: `AndThen` chains become the `Sequentially`
|||   telescope.
||| - **T7 delay**: the `Delayed` clause adverbial becomes
|||   `Delayed <event-query>` with the announce list unbound
|||   ([CR#603.7c,603.3d]).
||| - **T8 group**: `Each pred` becomes the `Each (Existing (SelectAll
|||   pred))` loop binder, its per-element mention read as `It`.
||| - **T9 move retag**: the new form's `Move` updates the referent's
|||   zone fold-state in place (its carrier is derived from it); the old
|||   form models the same fact by PUSHING the moved object onto the
|||   stack channel under its new noun ([CR#400.7j] — the effect can
|||   still find what it moved).
||| - **T10 relational**: `ControllerOf`/`OwnerOf` map one-to-one onto
|||   the old `Reference` constructors of the same names.
||| - **T11 tag erasure**: keyword-action macros (`Composite <verb>`
|||   over a body) lower to the old grammar's direct spelling — v1 has
|||   no tag channel, so the tag erases here; core keeps it, since only
|||   a tagged move IS the keyword action ([CR#701.8b]).
module Bridge

import Semantics
import Macros

%default total

-- NEW (Experimental.bolt):
--   DealDamage This (Lit 3) (Target AnyTarget)
-- Rules: T1 (one slot hoisted), T2 (the recipient position reads slot 0).
boltOld : OneShotEffect Base
boltOld = Targeted [anyTarget] (Act (DealDamage This (^3) (Target 0)))

-- NEW (Experimental.arcTrail):
--   AndThen (DealDamage This (Lit 2) (Target AnyTarget))
--           (DealDamage This (Lit 1) (Target anyOtherTarget))
-- Rules: T1 (both slots hoisted across the clause sequence), T2, T3
-- (`Other` → `Distinct [0]`), T6.
arcTrailOld : OneShotEffect Base
arcTrailOld =
  Targeted [ anyTarget, Distinct [0] anyTarget ]
    (Sequentially [ Act (DealDamage This (^2) (Target {k = Anything} 0))
                  , Act (DealDamage This (^1) (Target {k = Anything} 1)) ])

-- NEW (Experimental.eachSweep):
--   DealDamage This (Lit 3) (Each (And [creature, ControlledBy anOpponent]))
-- Rules: T8 (the in-situ group nominal becomes the loop binder wrapping
-- the clause; the damage recipient position becomes the loop's `It`).
eachSweepOld : OneShotEffect Base
eachSweepOld = Each (Existing (SelectAll (And [creature, ControlledBy opponent])))
                    (Act (DealDamage This (^3) It))

-- NEW (Experimental.sneakAttack):
--   AndThen (May (Move (A (And [creature, InZone Hand])) Battlefield))
--           (AndThen (gainsHaste (That (Perm Creature)))
--                    (Delayed (sacrifice (That (Perm Creature)))))
-- Rules: T5 (`A pred` → `With (ChooseOne …)`), T4 (both "that creature"
-- reads stay sorted reads — old noun `Permanent`, the pushed product's
-- zone noun; new carrier `Perm Creature`, derived from zone fold-state
-- plus the projected head type), T6, T7, T9 (the battlefield move is
-- what re-carriers the referent), T11 (the Sacrifice tag erases into
-- the direct graveyard move). (Old form from `Spec.idr`'s Through the
-- Breach shape; the haste grant spells out as a continuous modification
-- the new chapter still elides into `Gain`.)
throughTheBreachOld : OneShotEffect Base
throughTheBreachOld =
  Sequentially [ May (With (ChooseOne (And [inHand, creature])) (Act (Move It (ToZone Battlefield))))
               , Continuously UntilEndOfTurn (Modify (That Permanent) (GrantAbility (keyword Haste)))
               , Delayed nextEndStep (Act (Move (That Permanent) (ToZone Graveyard))) ]

-- NEW (Experimental.cloudshift):
--   AndThen (exile (Target creatureYouControl))
--           (Move (That CardC) Battlefield)
-- Rules: T1 (one slot hoisted), T2 (the exile position reads slot 0),
-- T4 ("that card" stays a sorted read — old `That Card` against the
-- pushed exile product, new `That CardC` against the retagged
-- fold-state), T6 ("then" → the telescope), T9, T11 (the Exile tag
-- erases into the direct `Move … (ToZone Exile)`). Old form from
-- `Cards.idr`'s Cloudshift spelling.
cloudshiftOld : OneShotEffect Base
cloudshiftOld =
  Targeted [ Target (^1) (And [permanent, creature, ControlledBy you]) ]
    (Sequentially [ Act (Move (Target 0) (ToZone Exile))
                  , Act (Move (That Card) (ToZone Battlefield)) ])
