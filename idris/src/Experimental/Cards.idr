||| The workbench's evidence bench: typechecking positives and pinned
||| `failing` negatives over real cards. Every finding in `Experimental`'s
||| ledger is backed by a term here. The machinery, the contract pointers,
||| and the findings ledger itself live in `Experimental`; this module only
||| exercises them.
module Experimental.Cards

import Experimental

%default total

-- ===== Positives (must typecheck) =====

-- "Lightning Bolt deals 3 damage to any target."
bolt : Effect []
bolt = DealDamage This (Lit 3) (Target AnyTarget)

-- "Barrage of Boulders deals 1 damage to each creature you don't
-- control." (Ferocious rider line elided) — the corpus has no "each
-- creature an opponent controls": opponent-scoped sweeps say "you don't
-- control" or plural "your opponents control" (a later chapter's noun).
barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Each creatureYouDontControl)

-- "Target creature you control deals damage equal to its power to target
-- creature you don't control." — THE in-situ dividend: "its" is read
-- where only slot A precedes, so plain uniqueness resolves it; slot B
-- doesn't exist yet. Hoisted, this exact card needed positional reads.
rabidBite : Effect []
rabidBite = DealDamage (Target creatureYouControl)
                       (PowerOf It)
                       (Target creatureYouDontControl)

-- "Target creature you control fights target creature you don't
-- control." — two same-sort slots are just two argument positions.
preyUpon : Effect []
preyUpon = Fights (Target creatureYouControl) (Target creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct.
arcTrail : Effect []
arcTrail = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                   (DealDamage This (Lit 1) (Target anyOtherTarget))

-- "Exile target creature you control, then return that card to the
-- battlefield." (Cloudshift; "under your control" elided with zone
-- ownership) — the exile RETAGS the referent's zone, so the carrier the
-- demonstrative must use flips from creature to card mid-sentence
-- ([CR#110.1]), and the return trip is just another Move.
cloudshift : Effect []
cloudshift = AndThen (exile (Target creatureYouControl))
                     (Move (That CardC) BattlefieldZ)

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (Through the Breach; Splice line elided — real Sneak
-- Attack says "the creature", a definite read this chapter doesn't mint)
-- — the hand-to-battlefield move RESTORES the typed carrier (the head
-- type was projected at introduction, never lost), the choice-determined
-- referent survives the delay [CR#603.7c], and the trailing adverbial
-- is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = AndThen (May You (Move (A (And [creature, InZone (HandOf You)])) BattlefieldZ))
                           (AndThen (gainsHaste (That (Perm Creature)) Nothing)
                                    (Delayed NextEndStep (sacrifice You (That (Perm Creature)))))

-- "Destroy target creature. Its controller loses 2 life." (Bitter
-- Downfall; its cost-reduction line elided) — the relational noun:
-- `ControllerOf It` derives a NEW player referent from the destroyed
-- object (whose "its" still resolves — the retag moved it to the
-- graveyard, it didn't unmention it).
bitterDownfall : Effect []
bitterDownfall = AndThen (destroy (Target creature))
                         (losesLife (ControllerOf It) (Lit 2))

-- "Tap target creature. It deals damage equal to its power to another
-- target creature." (Deadshot) — pronoun as source, and "another" is
-- the same `Other` modifier "any other target" uses.
deadshot : Effect []
deadshot = AndThen (Tap (Target creature))
                   (DealDamage It (PowerOf It) (Target (And [creature, Other])))

-- "Sacrifice this land: It deals 3 damage to target player. That
-- player discards a card." (Immersturm Skullcairn; its mana and {T}
-- cost components, the land's other lines, and its timing line elided)
-- — the cost's sacrificed self is the effect's "It": the cost move
-- mints the referent ([CR#400.7]) and it survives the colon publicly
-- ([CR#400.7j]); then the sorted demonstrative at Player kind and a
-- keyword-action macro (Discard) whose body moves a hand-zone choice.
immersturmSkullcairn : Activated []
immersturmSkullcairn = MkActivated (sacrifice You (ThisOf Land))
                                   (AndThen (DealDamage It (Lit 3) (Target AnyPlayer))
                                            (discardsACard (That PlayerC)))

-- "{R}, Sacrifice this artifact: It deals 2 damage to any target."
-- (Pyrite Spellbomb, first ability; {R} and the card's second ability
-- elided) — the smallest cost-antecedent pair: the sorted
-- self-reference moved by the cost is the only mention "It" can reach.
pyriteSpellbomb : Activated []
pyriteSpellbomb = MkActivated (sacrifice You (ThisOf Artifact))
                              (DealDamage It (Lit 2) (Target AnyTarget))

-- "Target player sacrifices a creature of their choice." (Diabolic
-- Edict) — the declarative clause: the target subject introduces, the
-- verb phrase is typed after it, and "of their choice" is the marked
-- own-choice method on the indefinite (the corpus has no bare "Target
-- player sacrifices a creature").
diabolicEdict : Effect []
diabolicEdict = sacrifice (Target AnyPlayer) (ATheirChoice creature)

-- "Each player sacrifices a creature of their choice." (Innocent
-- Blood) — a group subject: the same clause shape over "each player".
innocentBlood : Effect []
innocentBlood = sacrifice (Each AnyPlayer) (ATheirChoice creature)

-- "Target player discards a card." (Cry of Contrition, first line; its
-- Haunt lines elided) — the declarative discard whose imperative twin
-- is the same macro with `You`.
cryOfContrition : Effect []
cryOfContrition = discardsACard (Target AnyPlayer)

-- "Destroy target creature an opponent controls. That player loses 3
-- life." (Suspended Sentence; its self-exile clause and Suspend lines
-- elided) — the relative clause's inner mention folds: the indefinite
-- opponent enters the discourse from INSIDE the target's predicate,
-- and the sorted demonstrative reads it.
suspendedSentence : Effect []
suspendedSentence = AndThen (destroy (Target (And [creature, ControlledBy anOpponent])))
                            (losesLife (That PlayerC) (Lit 3))

-- "Exile this creature, then return it to the battlefield under its
-- owner's control." (Flickering Spirit's activated ability; its
-- mana-only cost and "under its owner's control" elided) — the sorted
-- self-reference moved by the EFFECT: the exile mints the new object's
-- binding mid-sentence ([CR#400.7]) and "it" reads it.
flickeringSpirit : Effect []
flickeringSpirit = AndThen (exile (ThisOf Creature)) (Move It BattlefieldZ)

-- "Exile target creature. Return that card to the battlefield under
-- its owner's control at the beginning of the next end step." (Turn to
-- Mist; "under its owner's control" elided) — the delayed clause reads
-- the TARGET-determined referent as a settled particular under its
-- retagged carrier ([CR#603.7c] is determiner-blind); the fire-time
-- zone expectation stays runtime (a mismatch is a no-op, not an
-- illegality).
turnToMist : Effect []
turnToMist = AndThen (exile (Target creature))
                     (Delayed NextEndStep (Move (That CardC) BattlefieldZ))

-- "Target creature gains flying until end of turn." (Jump) — the
-- duration as trailing-adverbial data ([CR#611.2a]).
jump : Effect []
jump = Gain (Target creature) (KeywordAbility Flying) (Just UntilEndOfTurn)

-- "Target creature gets +3/+3 until end of turn." (Giant Growth)
giantGrowth : Effect []
giantGrowth = Gets (Target creature) 3 3 (Just UntilEndOfTurn)

-- "Return target creature card from your graveyard to the
-- battlefield. It gains haste until your next turn." (Bond of
-- Revival) — an owned-zone source and the cross-turn duration; the
-- return is just a Move, and "it" reads the retagged referent.
bondOfRevival : Effect []
bondOfRevival = AndThen (Move (Target (And [creature, InZone (GraveyardOf You)])) BattlefieldZ)
                        (gainsHaste It (Just UntilYourNextTurn))

-- "When target creature dies this turn, return that card to the
-- battlefield under its owner's control." (Graceful Reprieve; "under
-- its owner's control" elided) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (DiesThisTurn (Target creature))
                           (Move (That CardC) BattlefieldZ)

-- "Destroy target creature. You gain life equal to its toughness."
-- (Vraska's Stoneglare; its tutor clause elided) — a last-known read:
-- reads ignore zone, so the dead referent's characteristics stay
-- readable; which values they see is runtime (the ability layer's
-- story).
vraskasStoneglare : Effect []
vraskasStoneglare = AndThen (destroy (Target creature))
                            (gainsLife You (ToughnessOf It))

-- "Destroy target creature. Its controller loses life equal to its
-- power plus its toughness." (Phthisis; its Suspend line elided) —
-- the relational noun over the dead referent, and amount arithmetic
-- threading left to right.
phthisis : Effect []
phthisis = AndThen (destroy (Target creature))
                   (losesLife (ControllerOf It) (Plus (PowerOf It) (ToughnessOf It)))

-- "This creature deals damage equal to its power to target creature.
-- That creature deals damage equal to its power to this creature."
-- (Karplusan Yeti's activated ability; its {T} cost elided, and the
-- source-referring "its" is spelled as the self-reference — source
-- mentions don't bind) — the SEQUENTIAL cousin of fight: two one-shot
-- damage clauses, not [CR#701.14a]'s single simultaneous event
-- (state-based actions can intervene between the sentences), which is
-- why `Fights` stays primitive.
karplusanYeti : Effect []
karplusanYeti = AndThen (DealDamage (ThisOf Creature) (PowerOf (ThisOf Creature)) (Target creature))
                        (DealDamage (That (Perm Creature)) (PowerOf It) (ThisOf Creature))

-- "Choose two target creatures. Tap those creatures, then unattach
-- all Equipment from them." (Fulgent Distraction; the unattach clause
-- elided) — a counted group mention, read back by the sorted plural
-- demonstrative.
fulgentDistraction : Effect []
fulgentDistraction = AndThen (Choose (TargetGroup 2 creature))
                             (Tap (Those (Perm Creature)))

-- "Choose up to four target creature cards in your graveyard that
-- were put there from the battlefield this turn. Return them to the
-- battlefield." (Continue?; its look-back restrictive clause elided —
-- event-history predicates are unminted) — the bounded group, an
-- owned-zone predicate, and the plural wildcard riding the return's
-- retag.
continueSpell : Effect []
continueSpell = AndThen (Choose (TargetUpTo 4 (And [creature, InZone (GraveyardOf You)])))
                        (Move Them BattlefieldZ)

-- "Choose a color. Sudden Demise deals X damage to each creature of
-- the chosen color." (Sudden Demise) — a quality mention: the chosen
-- color enters the discourse like any mention ([CR#105.1]) and the
-- predicate-internal read demands it uniquely; X is the announced
-- cost variable ([CR#107.3a]).
suddenDemise : Effect []
suddenDemise = AndThen (Choose (A (QualityNoun Color)))
                       (DealDamage This XVal (Each (And [creature, OfChosen Color])))

-- "Choose a creature type. Destroy all creatures that aren't of the
-- chosen type." (Kindred Dominance) — the set-level "all" determiner
-- over a negated quality read.
kindredDominance : Effect []
kindredDominance = AndThen (Choose (A (QualityNoun CreatureType)))
                           (destroy (AllOf (And [creature, Not (OfChosen CreatureType)])))

-- ===== Negatives (each `failing` block must NOT typecheck) =====

-- "other" with no target before it: the presupposition has no witness.
-- Forward and self references are unspellable the same way — there is
-- no context in which a later mention precedes.
failing "anyTargeted"
  badOther : Effect []
  badOther = DealDamage This (Lit 1) (Target anyOtherTarget)

-- A genuinely ambiguous pronoun: two singular Object mentions precede
-- "it", so the uniqueness gate refuses. (Not oracle-legal text — which
-- is the point: the controlled language never writes this.)
failing "countOnes"
  badIt : Effect []
  badIt = AndThen (Fights (Target creature) (Target creature)) (Tap It)

-- A group is not a singular antecedent: "each creature … it" has no
-- referent for "it" (the plurality guard).
failing "countOnes"
  badTheyIt : Effect []
  badTheyIt = AndThen (DealDamage This (Lit 3) (Each creature)) (Tap It)

-- The delay does not launder a dead referent: sacrifice's zone demand
-- reads fold-state through the boundary, and the destroyed target sits
-- in the graveyard ([CR#701.21a]; contrast Junkyo Bell, which legally
-- delays sacrificing a LIVE target — the distinction is the referent's
-- zone, never its determiner).
failing "OnBattlefield"
  badStale : Effect []
  badStale = AndThen (destroy (Target creature)) (Delayed NextEndStep (sacrifice You It))

-- "Another target" inside a delayed clause can only be distinct from
-- the DELAYED ability's own targets — it announces in its own event
-- ([CR#603.3d,601.2c]), so the outer clause's settled targets are no
-- witness for the presupposition. (Soundness-derived: the corpus
-- writes no such line; Swooping Pteranodon shows the two halves — a
-- fresh "target land" announced at delay time reading back "that
-- creature" from the outer clause.)
failing "anyTargeted"
  badDelayedOther : Effect []
  badDelayedOther = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                            (Delayed NextEndStep (DealDamage This (Lit 1) (Target anyOtherTarget)))

-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
failing "countCarrier"
  badStaleCarrier : Effect []
  badStaleCarrier = AndThen (exile (Target creatureYouControl))
                            (Move (That (Perm Creature)) BattlefieldZ)

-- A hidden-zone cost mention is unreadable past the colon: the card
-- bounced to hand is not among the public survivors ([CR#400.2] —
-- hand is hidden; no [CR#400.7] exception reaches it, and
-- Soratami Cloudskater-family costs write no such read back).
failing "publicOnly"
  badHiddenCost : Activated []
  badHiddenCost = MkActivated (Move (A creature) HandZ) (Tap It)

-- Two cost moves leave two candidate antecedents ("Discard a card,
-- Sacrifice a creature: …" — Falkenrath Pit Fighter-family): a bare
-- "It" past the colon is ambiguous. Real costs of this shape read
-- back with definite descriptions ("the sacrificed creature" — a
-- later chapter's noun), never a bare pronoun.
failing "countOnes"
  badTwoCostMentions : Activated []
  badTwoCostMentions = MkActivated (AndThen (discardsACard You)
                                            (sacrifice You (A creature)))
                                   (Tap It)

-- The zone half of sacrifice's implicit restriction as a type error:
-- an exiled referent is not sacrificeable [CR#701.21a].
failing "OnBattlefield"
  badSacrificeExiled : Effect []
  badSacrificeExiled = AndThen (exile (Target creature)) (sacrifice You It)

-- After the watched target dies, it no longer answers to "creature":
-- the event retag flips the carrier ([CR#700.4,110.1]) — "that card"
-- is the spelling that resolves (see `gracefulReprieve`).
failing "countCarrier"
  badDeadCreatureRead : Effect []
  badDeadCreatureRead = Delayed (DiesThisTurn (Target creature))
                                (Move (That (Perm Creature)) BattlefieldZ)

-- "The chosen type" with only a color chosen: the quality read is
-- sort-filtered — no witness.
failing "countQuality"
  badChosenWrongSort : Effect []
  badChosenWrongSort = AndThen (Choose (A (QualityNoun Color)))
                               (destroy (AllOf (And [creature, Not (OfChosen CreatureType)])))

-- Two group mentions leave "them" ambiguous — the plural wildcard has
-- the same strict-uniqueness gate as the singular.
failing "countManys"
  badThemAmbig : Effect []
  badThemAmbig = AndThen (Choose (TargetGroup 2 creature))
                         (AndThen (Choose (TargetGroup 2 creature))
                                  (Tap Them))

-- Two predicate-inner opponents leave "that player" ambiguous — the
-- uniqueness gate reaches inside relative clauses too. (Not
-- oracle-legal text; the guide would repeat the noun.)
failing "countCarrier"
  badInnerAmbig : Effect []
  badInnerAmbig = AndThen (Fights (Target (And [creature, ControlledBy anOpponent]))
                                  (Target (And [creature, ControlledBy anOpponent])))
                          (losesLife (That PlayerC) (Lit 1))
