||| The workbench's evidence bench: typechecking positives and pinned
||| `failing` negatives over real cards. Every finding in `Experimental`'s
||| ledger is backed by a term here. The machinery, the contract pointers,
||| and the findings ledger itself live in `Experimental`; this module only
||| exercises them.
module Experimental.Cards

import Experimental
import Experimental.Macros

%default total

-- ===== Positives (must typecheck) =====

-- "Lightning Bolt deals 3 damage to any target."
bolt : Effect []
bolt = DealDamage This (Lit 3) (target AnyTarget)

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
rabidBite = DealDamage (target creatureYouControl)
                       (powerOf It)
                       (target creatureYouDontControl)

-- "Target creature you control fights target creature you don't
-- control." (Prey Upon; its reminder text "(Each deals damage equal to
-- its power to the other.)" omitted — a parenthetical gloss, not rules
-- text, and the same expansion finding 20 refuses to treat as the
-- operative spelling) — two same-sort slots are just two argument
-- positions.
preyUpon : Effect []
preyUpon = Fights (target creatureYouControl) (target creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct. The oracle writes ONE verb over coordinated
-- amount+recipient complements; the bench transcribes them as a
-- two-clause `Sequentially`, a named stand-in that mis-orders nothing
-- binding-wise but serializes what the card states as a single
-- instruction (ledger).
arcTrail : Effect []
arcTrail = Sequentially [DealDamage This (Lit 2) (target AnyTarget),
                         DealDamage This (Lit 1) (target anyOtherTarget)]

-- "Exile target creature you control, then return that card to the
-- battlefield under your control." (Cloudshift) — the exile RETAGS the
-- referent's zone, so the carrier the demonstrative must use flips from
-- creature to card mid-sentence ([CR#110.1]), and the return trip is
-- just another Move. "Under your control" is the DEFAULT made explicit
-- — [CR#110.2a] puts an object under the control of the player the
-- effect instructed to put it there, and You are that player — so the
-- elision is derivable, unlike the owner-control override its siblings
-- write.
cloudshift : Effect []
cloudshift = Sequentially [exile (target creatureYouControl),
                           Move (That CardW) BattlefieldZ]

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (Through the Breach; Splice line elided — real Sneak
-- Attack says "the creature", a definite read this chapter doesn't mint)
-- — the hand-to-battlefield move RESTORES the typed carrier (the head
-- type was projected at introduction, never lost), the choice-determined
-- referent survives the delay [CR#603.7c], and the trailing adverbial
-- is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = Sequentially [May You (Move (A (And [creature, InZone (HandOf You)])) BattlefieldZ),
                                 gainsHaste (That (TypeW Creature)) Nothing,
                                 Delayed NextEndStep (sacrifice You (That (TypeW Creature)))]

-- "Destroy target creature. Its controller loses 2 life." (Bitter
-- Downfall; its cost-reduction line elided) — the relational noun:
-- `ControllerOf It` derives a NEW player referent from the destroyed
-- object (whose "its" still resolves — the retag moved it to the
-- graveyard, it didn't unmention it).
bitterDownfall : Effect []
bitterDownfall = Sequentially [destroy (target creature),
                               losesLife (ControllerOf It) (Lit 2)]

-- "Tap target creature. It deals damage equal to its power to another
-- target creature." (Deadshot) — pronoun as source, and "another" is
-- the same `Other` modifier "any other target" uses.
deadshot : Effect []
deadshot = Sequentially [Tap (target creature),
                         DealDamage It (powerOf It) (target (And [creature, Other]))]

-- "Sacrifice this land: It deals 3 damage to target player. That
-- player discards a card." (Immersturm Skullcairn; its mana and {T}
-- cost components, the land's other lines, and its timing line elided)
-- — the cost's sacrificed self is the effect's "It": the cost move
-- mints the referent ([CR#400.7]) and it survives the colon publicly
-- ([CR#400.7j]); then the sorted demonstrative at Player kind and a
-- keyword-action macro (Discard) whose body moves a hand-zone choice.
immersturmSkullcairn : Activated []
immersturmSkullcairn = MkActivated (sacrifice You thisLand)
                                   (Sequentially [DealDamage It (Lit 3) (target AnyPlayer),
                                                  discardsACard (That PlayerW)])

-- "{R}, Sacrifice this artifact: It deals 2 damage to any target."
-- (Pyrite Spellbomb, first ability; {R} and the card's second ability
-- elided) — the smallest cost-antecedent pair: the sorted
-- self-reference moved by the cost is the only mention "It" can reach.
pyriteSpellbomb : Activated []
pyriteSpellbomb = MkActivated (sacrifice You thisArtifact)
                              (DealDamage It (Lit 2) (target AnyTarget))

-- "Target player sacrifices a creature of their choice." (Diabolic
-- Edict) — the declarative clause: the target subject introduces, the
-- verb phrase is typed after it, and "of their choice" is the marked
-- own-choice method on the indefinite (the corpus has no bare "Target
-- player sacrifices a creature").
diabolicEdict : Effect []
diabolicEdict = sacrifice (target AnyPlayer) (ATheirChoice creature)

-- "Each player sacrifices a creature of their choice." (Innocent
-- Blood) — a group subject: the same clause shape over "each player".
innocentBlood : Effect []
innocentBlood = sacrifice (Each AnyPlayer) (ATheirChoice creature)

-- "Target player discards a card." (Cry of Contrition, first line; its
-- Haunt lines elided) — the declarative discard whose imperative twin
-- is the same macro with `You`.
cryOfContrition : Effect []
cryOfContrition = discardsACard (target AnyPlayer)

-- "Destroy target creature an opponent controls. That player loses 3
-- life." (Suspended Sentence; its self-exile clause and Suspend lines
-- elided) — the relative clause's inner mention folds: the indefinite
-- opponent enters the discourse from INSIDE the target's predicate,
-- and the sorted demonstrative reads it.
suspendedSentence : Effect []
suspendedSentence = Sequentially [destroy (target (And [creature, ControlledBy anOpponent])),
                                  losesLife (That PlayerW) (Lit 3)]

-- "Exile this creature, then return it to the battlefield under its
-- owner's control." (Flickering Spirit's activated ability; its
-- mana-only cost elided) — the sorted self-reference moved by the
-- EFFECT: the exile mints the new object's binding mid-sentence
-- ([CR#400.7]) and "it" reads it. "Under its owner's control" is an
-- OVERRIDE of [CR#110.2a]'s default (the ability's controller is the
-- instructed player), so its elision is meaning-carrying — pending the
-- control-assignment axis.
flickeringSpirit : Effect []
flickeringSpirit = Sequentially [exile thisCreature,
                                 Move It BattlefieldZ]

-- "Exile target creature. Return that card to the battlefield under
-- its owner's control at the beginning of the next end step." (Turn to
-- Mist; "under its owner's control" is a meaning-carrying elision —
-- the override of [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — the delayed clause reads the
-- TARGET-determined referent as a settled particular under its
-- retagged carrier ([CR#603.7c] is determiner-blind); the fire-time
-- zone expectation stays runtime (a mismatch is a no-op, not an
-- illegality).
turnToMist : Effect []
turnToMist = Sequentially [exile (target creature),
                           Delayed NextEndStep (Move (That CardW) BattlefieldZ)]

-- "Target creature gains flying until end of turn." (Jump) — the
-- duration as trailing-adverbial data ([CR#611.2a]).
jump : Effect []
jump = Gain (target creature) (KeywordAbility Flying) (Just UntilEndOfTurn)

-- "Target creature gets +3/+3 until end of turn." (Giant Growth)
giantGrowth : Effect []
giantGrowth = Gets (target creature) 3 3 (Just UntilEndOfTurn)

-- "Return target creature card from your graveyard to the
-- battlefield. It gains haste until your next turn." (Bond of
-- Revival) — an owned-zone source and the cross-turn duration; the
-- return is just a Move, and "it" reads the retagged referent.
bondOfRevival : Effect []
bondOfRevival = Sequentially [Move (target (And [creature, InZone (GraveyardOf You)])) BattlefieldZ,
                              gainsHaste It (Just UntilYourNextTurn)]

-- "When target creature dies this turn, return that card to the
-- battlefield under its owner's control." (Graceful Reprieve; "under
-- its owner's control" is a meaning-carrying elision — the override of
-- [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (DiesThisTurn (target creature))
                           (Move (That CardW) BattlefieldZ)

-- "Destroy target creature. You gain life equal to its toughness."
-- (Vraska's Stoneglare; its tutor clause elided) — a last-known read:
-- reads ignore zone, so the dead referent's characteristics stay
-- readable; which values they see is runtime (the ability layer's
-- story).
vraskasStoneglare : Effect []
vraskasStoneglare = Sequentially [destroy (target creature),
                                  gainsLife You (toughnessOf It)]

-- "Destroy target creature. Its controller loses life equal to its
-- power plus its toughness." (Phthisis; its Suspend line elided) —
-- the relational noun over the dead referent, and amount arithmetic
-- threading left to right.
phthisis : Effect []
phthisis = Sequentially [destroy (target creature),
                         losesLife (ControllerOf It) (Plus (powerOf It) (toughnessOf It))]

-- "This creature deals damage equal to its power to target creature.
-- That creature deals damage equal to its power to this creature."
-- (Karplusan Yeti's activated ability; its {T} cost elided, and the
-- source-referring "its" is spelled as the self-reference — source
-- mentions don't bind) — the SEQUENTIAL cousin of fight: two ORDERED
-- one-shot damage events, where [CR#701.14a] deals both
-- simultaneously (state-based actions see neither mid-resolution,
-- [CR#704.4]), which is why `Fights` stays primitive.
karplusanYeti : Effect []
karplusanYeti = Sequentially [DealDamage thisCreature (powerOf thisCreature) (target creature),
                              DealDamage (That (TypeW Creature)) (powerOf It) thisCreature]

-- "Choose two target creatures. Tap those creatures, then unattach
-- all Equipment from them." (Fulgent Distraction; the unattach clause
-- elided) — a counted group mention, read back by the sorted plural
-- demonstrative.
fulgentDistraction : Effect []
fulgentDistraction = Sequentially [Choose (TargetGroup (exactly 2) creature),
                                   Tap (Those (TypeW Creature))]

-- "Choose up to four target creature cards in your graveyard that
-- were put there from the battlefield this turn. Return them to the
-- battlefield." (Continue?; its look-back restrictive clause elided —
-- event-history predicates are unminted) — the bounded group, an
-- owned-zone predicate, and the plural wildcard riding the return's
-- retag.
continueSpell : Effect []
continueSpell = Sequentially [Choose (TargetGroup (upTo 4) (And [creature, InZone (GraveyardOf You)])),
                              Move Them BattlefieldZ]

-- "Choose a color. Sudden Demise deals X damage to each creature of
-- the chosen color." (Sudden Demise) — a quality mention: the chosen
-- color enters the discourse like any mention ([CR#105.1]) and the
-- predicate-internal read demands it uniquely; X is the announced
-- cost variable ([CR#107.3a]).
suddenDemise : Effect []
suddenDemise = Sequentially [Choose (A (QualityNoun Color)),
                             DealDamage This XVal (Each (And [creature, OfChosen Color]))]

-- "Choose a creature type. Destroy all creatures that aren't of the
-- chosen type." (Kindred Dominance) — the set-level "all" determiner
-- over a negated quality read.
kindredDominance : Effect []
kindredDominance = Sequentially [Choose (A (QualityNoun CreatureType)),
                                 destroy (AllOf (And [creature, Not (OfChosen CreatureType)]))]

-- "{2}, Sacrifice this artifact: Exile target creature. Return the
-- exiled card to the battlefield under its owner's control at the
-- beginning of the next end step." (Voyager Staff; {2} elided, and
-- "under its owner's control" is a meaning-carrying elision — the
-- override of [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — THE finding-12 partner: past the colon
-- the discourse holds TWO card mentions (the sacrificed self, the
-- exiled target), so a bare demonstrative is ambiguous
-- (badBareCardRead) and the text switches to the participle, whose
-- verb filter picks the exile; the delay carries it as usual
-- ([CR#603.7c]).
voyagerStaff : Activated []
voyagerStaff = MkActivated (sacrifice You thisArtifact)
                           (Sequentially [exile (target creature),
                                          Delayed NextEndStep (Move (TheVerbed Exile CardW) BattlefieldZ)])

-- "{3}{R}, Sacrifice an artifact: Bosh deals damage equal to the
-- sacrificed artifact's mana value to any target." (Bosh, Iron Golem;
-- {3}{R} and its Trample line elided; the self-name is `This`) — the
-- TYPE-word participle noun: the referent is a graveyard card NOW,
-- but "artifact" describes it under the verb — checked on the
-- time-stable projected head type, the axis a declared type word
-- lives on (finding 27).
boshIronGolem : Activated []
boshIronGolem = MkActivated (sacrifice You (A (HasType Artifact)))
                            (DealDamage This
                                        (manaValueOf (TheVerbed Sacrifice (TypeW Artifact)))
                                        (target AnyTarget))

-- "{3}, Discard a card at random: This enchantment deals damage to
-- any target equal to the mana value of the discarded card."
-- (Pyromancy; {3} elided; the trailing "equal to …" is extraposition
-- — spelling's linearization of the same deal(src, amt, to) frame,
-- and amounts introduce no bindings, so the fixed order is
-- binding-neutral) — the at-random marked indefinite finally has its
-- positive ([CR#701.9b] — no chooser), and the colon read is the
-- participle at the intrinsic CARD word, checked on current zone
-- fold-state. The self-reference is the SORTED one now that the type
-- word exists: "this enchantment" is a description including a card
-- type, so [CR#109.2] denotes the permanent (bare `This` was standing
-- in for the missing `Enchantment` row).
pyromancy : Activated []
pyromancy = MkActivated (discards You (AAtRandom (InZone HandZ)))
                        (DealDamage thisEnchantment
                                    (manaValueOf (TheVerbed Discard CardW))
                                    (target AnyTarget))

-- "Target opponent loses 1 life for each attacking creature you
-- control. You gain that much life." (Foul-Tongue Shriek) — the
-- event-outcome read: the loss clause introduces its outcome (sort
-- LifeLost, projected from the surface; magnitude runtime), and
-- "that much" reads the unique outcome in scope, sort-blind. The
-- attacking modifier is a battlefield state word ([CR#508.1a]).
foulTongueShriek : Effect []
foulTongueShriek = Sequentially [losesLife (target Opponent)
                                           (forEach (And [Attacking, creature, ControlledBy You])),
                                 gainsLife You ThatMuch]

-- "Return target creature to its owner's hand." (Unsummon) — the
-- destination is bare `HandZ`: [CR#400.3] admits no other hand, so
-- the possessive is derived surface, never stored (finding 34).
unsummon : Effect []
unsummon = Move (target creature) HandZ

-- "When this Equipment enters, attach it to up to one target creature
-- you control. Destroy up to one other target creature." (Phantom
-- Blade's trigger; the attach verb waits on the attachment axis, so
-- its TARGETING stands in as the fronted choice clause) — the up-to
-- mention is a real "other" anchor: the witness is the MENTION, not
-- a nonempty denotation (finding 35).
phantomBlade : Effect []
phantomBlade = Sequentially [Choose (TargetGroup (upTo 1) (And [creature, ControlledBy You])),
                             destroy (TargetGroup (upTo 1) (And [creature, Other]))]

-- "When this Case enters, choose target creature you don't control.
-- Each creature you control deals 1 damage to that creature." (Case of
-- the Gateway Express's enters trigger; the trigger shape itself and
-- the card's To solve / Solved lines elided) — the DISTRIBUTIVE damage
-- subject: "each creature you control" spreads the singular deal frame
-- over its members, which is why the source gate admits `Each` where a
-- COLLECTIVE group source stays unattested. The fronted choice
-- sentence scopes the demonstrative that follows (finding 22), and the
-- group source is no antecedent for it — `countWord` counts singulars.
caseOfTheGatewayExpress : Effect []
caseOfTheGatewayExpress = Sequentially [Choose (target creatureYouDontControl),
                                        DealDamage (Each creatureYouControl) (Lit 1)
                                                   (That (TypeW Creature))]

-- "Cycling {2}" — "{2}, Discard this card: Draw a card." ([CR#702.29a];
-- the {2} and the draw elided, the draw verb being unminted) — the
-- standing justification for `DiscardOk`'s bare-`This` row: bare `This`
-- is the source as an OBJECT ("this card"), so it projects no zone,
-- while the SORTED self-reference now denotes the permanent
-- ([CR#109.2]) and cannot be discarded (`badDiscardThisCreature`).
cyclingCost : Effect []
cyclingCost = discards You This

-- "Target nonattacking, nonblocking creature gets +0/+2 until end of
-- turn." — a presupposed zone projects THROUGH negation: negating the
-- status does not negate the battlefield, so the phrase keeps the
-- [CR#109.2] default and stays coherent. The nonattacking family is
-- plentiful oracle ("Untap target nonattacking creature.", "Target
-- nonattacking creature gains reach and deathtouch until end of
-- turn."), which is what makes reading the seed through `Not` an
-- over-refusal rather than a nicety. The card's second negation now
-- spells too, and comma-chained exactly as the writer chains it —
-- which is the same fact `badNegatedDisjunction` states from the
-- other side.
rawNonattacking : Predicate [] Object
rawNonattacking = And [creature, Not Attacking, Not Blocking]

-- "Exile target creature." spelled raw — the tag-ALIGNED twin of
-- `badDestroyTaggedExile`: same body, agreeing tag. The `TagBody`
-- witness travels explicitly because its auto search is flaky even
-- here, at a concrete site with no nested autos — the same reason the
-- `exile` macro passes `{ok = ExileB}`.
rawExile : Effect []
rawExile = Composite Exile (Move (target creature) ExileZ) {ok = ExileB}

-- ===== Alternatives under one determiner: the disjunction chapter =====

-- "Destroy target artifact or enchantment." (Disenchant) — the
-- flagship. The card writes "target" ONCE, so it announces one target
-- ([CR#601.2c]) and the determiner scopes over the whole coordination:
-- the parser brackets it `<<target> <<artifact> <or <enchantment>>>>`,
-- with the alternatives inside the determined phrase rather than
-- beside it. That is why disjunction is a PREDICATE, not a second
-- noun.
disenchant : Effect []
disenchant = destroy (target (Or [artifact, enchantment]))

-- "Tap target artifact, creature, or land." (Icy Manipulator; its mana
-- and {T} cost elided) — a third alternative costs no machinery, and
-- the guide puts the serial comma before the coordinator from three
-- items up. This is the argument for a member LIST over a binary
-- connective, and it is core's shape too (`Predicate::Or` takes a
-- slice).
icyManipulator : Effect []
icyManipulator = Tap (target (Or [artifact, creature, land]))

-- "Destroy target artifact, creature, or land you control." (Rats of
-- Rath; its mana cost elided) — the shared trailing modifier, and it
-- needs nothing new either. The relative clause is a SIBLING of the
-- coordination within the conjunction, which is where the parse puts
-- it (`<<target> <<artifact><, creature><, or land>> <<you>
-- <control>>>`), so modifier scope falls out of the nesting rather
-- than wanting a rule. The other reading — the clause repeated per
-- alternative — is what the guide reserves for alternatives with
-- different domains.
ratsOfRath : Effect []
ratsOfRath = destroy (target (And [Or [artifact, creature, land], ControlledBy You]))

-- "Arrows of Justice deals 4 damage to target attacking or blocking
-- creature." — alternatives that CONTRAST: no creature is both, and
-- the phrase is none the worse for it. This is what the coherence
-- scans must not do to a disjunction, because splicing its members
-- into the surrounding conjunction would read the card as "attacking
-- and blocking" and refuse a phrase ninety-five corpus lines write.
arrowsOfJustice : Effect []
arrowsOfJustice = DealDamage This (Lit 4)
                             (target (And [creature, Or [Attacking, Blocking]]))

-- "another target creature or land" — the selector reaches over the
-- whole coordination, as the corpus writes it ("Another target Wolf or
-- Werewolf you control"), and the anchor it demands is compatible with
-- SOME alternative's head: a coordinated head fixes no type on its
-- referent but offers one per alternative (finding 44). Posed here at
-- an untyped target mention, which any head accepts — the
-- presupposition being the point rather than the test; the cross-head
-- refusal is `badDisjunctiveOtherCrossHead`'s.
anotherDisjunctPhrase : Predicate [MkBinding TargetD Object OneOf
                                             (ObjectP Nothing (Just Battlefield) Nothing)] Object
anotherDisjunctPhrase = And [Or [creature, land], Other]

-- ===== What a clause forbids: the deontics chapter =====

-- "Target creature can't be blocked this turn." (Infiltrate) — the
-- flagship, and the whole card. The clause creates a continuous effect
-- for a stated span ([CR#611.2a]) denying its subject a deed, which is
-- what the declare-blockers step then checks ([CR#509.1b]). The deed
-- word carries the ROLE: the subject of "can't be blocked" is the
-- creature being blocked, core's `on` slot, where "can't block" would
-- put it in `by`.
infiltrate : Effect []
infiltrate = cantBeBlocked (target creature) ThisTurn

-- "Target creature can't attack this turn." (Change of Heart; its
-- Buyback line elided — a keyword rider, as with Cascade and Splice)
-- — the active voice of the same clause, checked at declare attackers
-- instead ([CR#508.1c]).
changeOfHeart : Effect []
changeOfHeart = cantAttack (target creature) ThisTurn

-- "Blindblast deals 1 damage to target creature. That creature can't
-- block this turn." (Blindblast; "Draw a card." elided — no draw
-- vocabulary, ledgered) — the restriction takes an ANAPHORIC subject
-- like any other clause: the demonstrative reads the mention the damage
-- clause introduced, and the deontic needs nothing of its own for it.
blindblast : Effect []
blindblast = Sequentially [DealDamage This (Lit 1) (target creature),
                           cantBlock (That (TypeW Creature)) ThisTurn]

-- "Any number of target creatures can't block this turn." (Blinding
-- Flare; its Strive cost-modification line elided) — the subject is
-- PLURAL, and the clause demands no grammatical number: a restriction
-- ranges over whatever its subject phrase describes, one creature or a
-- group announced at once.
blindingFlare : Effect []
blindingFlare = cantBlock (TargetGroup anyNumber creature) ThisTurn

-- ===== A bound on a characteristic: the comparatives chapter =====

-- "Destroy target creature with power 2 or less." (Defeat) — the
-- flagship, and the whole card. The qualifier is a sibling MODIFIER
-- like any other, so the phrase needed no new noun layer: what is new
-- is the three written parts core spells in the same order
-- (`Stat(Power, AtMost, 2)`), and the presupposition that comes with
-- them — only a creature has power ([CR#208.3]).
defeat : Effect []
defeat = destroy (target (And [creature, Compare Power OrLess (Lit 2)]))

-- "Destroy target attacking creature with power 3 or less." (Terashi's
-- Verdict) — a status word and a bound in ONE phrase, which is the
-- interaction worth a positive: both members presuppose a creature and
-- the contradiction scan has to let them, positive types stacking
-- rather than clashing. The zone comes from `Attacking` alone; the
-- bound places nothing.
terashisVerdict : Effect []
terashisVerdict =
  destroy (target (And [creature, Attacking, Compare Power OrLess (Lit 3)]))

-- "Exile target creature with toughness 4 or greater." (Pillar of
-- Light) — the other comparator and the other creature-gated
-- characteristic, so the two-row vocabulary is spelled end to end by
-- real cards rather than by one card and an argument.
pillarOfLight : Effect []
pillarOfLight =
  exile (target (And [creature, Compare Toughness OrGreater (Lit 4)]))

-- "Destroy target artifact with mana value 3 or less. You gain 3
-- life." (Lucky Offering) — mana value on a NONCREATURE head, which is
-- the whole point of the characteristic table having a second answer:
-- every object has a mana value ([CR#202.3]), so this phrase
-- presupposes no type and the artifact head stands unchallenged. The
-- corpus bounds mana value on cards, spells, permanents, artifacts,
-- planeswalkers, and enchantments alike, and bounds power on nothing
-- but creatures.
luckyOffering : Effect []
luckyOffering =
  Sequentially [destroy (target (And [artifact,
                                      Compare ManaValue OrLess (Lit 3)])),
                gainsLife You (Lit 3)]

-- ===== Negatives (each `failing` block must NOT typecheck) =====

-- "other" with no target before it: the presupposition has no witness.
-- Forward and self references are unspellable the same way — there is
-- no context in which a later mention precedes.
failing "anyTargeted"
  badOther : Effect []
  badOther = DealDamage This (Lit 1) (target anyOtherTarget)

-- A genuinely ambiguous pronoun: two singular Object mentions precede
-- "it", so the uniqueness gate refuses. (Not oracle-legal text — which
-- is the point: the controlled language never writes this.)
failing "countOnes"
  badIt : Effect []
  badIt = Sequentially [Fights (target creature) (target creature),
                        Tap It]

-- A group is not a singular antecedent: "each creature … it" has no
-- referent for "it" (the plurality guard).
failing "countOnes"
  badTheyIt : Effect []
  badTheyIt = Sequentially [DealDamage This (Lit 3) (Each creature),
                            Tap It]

-- The delay does not launder a dead referent: sacrifice's zone demand
-- reads fold-state through the boundary, and the destroyed target sits
-- in the graveyard ([CR#701.21a]; contrast Junkyo Bell, which legally
-- delays sacrificing a LIVE target — the distinction is the referent's
-- zone, never its determiner).
failing "OnBattlefield"
  badStale : Effect []
  badStale = Sequentially [destroy (target creature),
                           Delayed NextEndStep (sacrifice You It)]

-- "Another target" inside a delayed clause can only be distinct from
-- the DELAYED ability's own targets — it announces in its own event
-- ([CR#603.3d,601.2c]), so the outer clause's settled targets are no
-- witness for the presupposition. (Soundness-derived: the corpus
-- writes no such line; Swooping Pteranodon shows the two halves — a
-- fresh "target land" announced at delay time reading back "that
-- creature" from the outer clause.)
failing "anyTargeted"
  badDelayedOther : Effect []
  badDelayedOther = Sequentially [DealDamage This (Lit 2) (target AnyTarget),
                                  Delayed NextEndStep (DealDamage This (Lit 1) (target anyOtherTarget))]

-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
failing "countWord"
  badStaleCarrier : Effect []
  badStaleCarrier = Sequentially [exile (target creatureYouControl),
                                  Move (That (TypeW Creature)) BattlefieldZ]

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
-- back with definite descriptions (the participle reads of chapter
-- eight), never a bare pronoun. (The verb is exile — zone-blind —
-- so the pin isolates the ambiguity, not a zone gate.)
failing "countOnes"
  badTwoCostMentions : Activated []
  badTwoCostMentions = MkActivated (Sequentially [discardsACard You,
                                                  sacrifice You (A creature)])
                                   (exile It)

-- The zone half of sacrifice's implicit restriction as a type error:
-- an exiled referent is not sacrificeable [CR#701.21a].
failing "OnBattlefield"
  badSacrificeExiled : Effect []
  badSacrificeExiled = Sequentially [exile (target creature),
                                     sacrifice You It]

-- After the watched target dies, it no longer answers to "creature":
-- the event retag flips the carrier ([CR#700.4,110.1]) — "that card"
-- is the spelling that resolves (see `gracefulReprieve`).
failing "countWord"
  badDeadCreatureRead : Effect []
  badDeadCreatureRead = Delayed (DiesThisTurn (target creature))
                                (Move (That (TypeW Creature)) BattlefieldZ)

-- "The chosen type" with only a color chosen: the quality read is
-- sort-filtered — no witness.
failing "countQuality"
  badChosenWrongSort : Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
  badChosenWrongSort = OfChosen CreatureType

-- Two group mentions leave "them" ambiguous — the plural wildcard has
-- the same strict-uniqueness gate as the singular.
failing "countManys"
  badThemAmbig : Effect []
  badThemAmbig = Sequentially [Choose (TargetGroup (exactly 2) creature),
                               Choose (TargetGroup (exactly 2) creature),
                               Tap Them]

-- Two predicate-inner opponents leave "that player" ambiguous — the
-- uniqueness gate reaches inside relative clauses too. (Not
-- oracle-legal text; the guide would repeat the noun.)
failing "countWord"
  badInnerAmbig : Effect []
  badInnerAmbig = Sequentially [Fights (target (And [creature, ControlledBy anOpponent]))
                                       (target (And [creature, ControlledBy anOpponent])),
                                losesLife (That PlayerW) (Lit 1)]

-- The representation-level witness of the §3 kind-indexing rule: a
-- binding cannot record data its kind cannot have — "a player in your
-- hand" fails to CONSTRUCT (`Payload Player` has no zone slot), the
-- same refusal the surface grammar makes at `InZone`'s kind.
failing "Payload Player"
  badPlayerInHand : Binding
  badPlayerInHand = MkBinding AD Player OneOf (ObjectP Nothing (Just Hand) Nothing)

-- The participle's verb filter has no witness: the cost discarded,
-- nothing was sacrificed.
failing "countVerbed"
  badVerbedWrongVerb : Activated []
  badVerbedWrongVerb = MkActivated (discardsACard You)
                                   (Move (TheVerbed Sacrifice CardW) BattlefieldZ)

-- The noun word misses on the type axis: an artifact was sacrificed,
-- so "the sacrificed creature" has no referent.
failing "countVerbed"
  badVerbedWrongNoun : Activated []
  badVerbedWrongNoun = MkActivated (sacrifice You (A (HasType Artifact)))
                                   (Move (TheVerbed Sacrifice (TypeW Creature)) BattlefieldZ)

-- Two same-verb stamps leave the participle ambiguous — the same
-- strict uniqueness as every read (real costs of this shape name
-- distinct verbs, which is what the filter buys).
failing "countVerbed"
  badVerbedAmbig : Activated []
  badVerbedAmbig = MkActivated (Sequentially [sacrifice You (A creature),
                                              sacrifice You (A creature)])
                               (Move (TheVerbed Sacrifice CardW) BattlefieldZ)

-- Voyager Staff's shape with the bare demonstrative: two card
-- mentions (the sacrificed self, the exiled target) make "that card"
-- ambiguous — the participle is what real text switches to here.
failing "countWord"
  badBareCardRead : Activated []
  badBareCardRead = MkActivated (sacrifice You thisArtifact)
                                (Sequentially [exile (target creature),
                                               Delayed NextEndStep (Move (That CardW) BattlefieldZ)])

-- ===== Chapter nine negatives: the audit round =====

-- Tapping takes a battlefield object [CR#701.26a]: a graveyard card
-- cannot be tapped.
failing "OnBattlefield"
  badTapGraveyard : Effect []
  badTapGraveyard = Tap (target (And [creature, InZone (GraveyardOf You)]))

-- Only battlefield creatures fight [CR#701.14b]: a graveyard card
-- cannot.
failing "OnBattlefield"
  badFightGraveyard : Effect []
  badFightGraveyard = Fights (target (And [creature, InZone (GraveyardOf You)]))
                             (target creature)

-- Only combatant types fight [CR#701.14a]: a land's TypeDef declares
-- no fight participation.
failing "FightParticipant"
  badFightLand : Effect []
  badFightLand = Fights (target (HasType Land)) (target creatureYouDontControl)

-- Dying is the battlefield-to-graveyard transition [CR#700.4]: an
-- already-graveyard card cannot die this turn.
failing "OnBattlefield"
  badDiesInGraveyard : Effect []
  badDiesInGraveyard = Delayed (DiesThisTurn (target (And [creature, InZone (GraveyardOf You)])))
                               (Move (That CardW) BattlefieldZ)

-- Damage reaches players and battlefield objects only [CR#120.1]:
-- the destroyed referent sits in the graveyard.
failing "DamageRecipient"
  badDamageGraveyardCard : Effect []
  badDamageGraveyardCard = Sequentially [destroy (target creature),
                                         DealDamage This (Lit 3) It]

-- A quality cannot take damage [CR#120.1].
failing "DamageRecipient"
  badDamageToColor : Effect []
  badDamageToColor = DealDamage This (Lit 1) (A (QualityNoun Color))

-- Damage goes to battles, creatures, planeswalkers, or players
-- [CR#120.1a]: a noncreature artifact takes none.
failing "DamageRecipient"
  badDamageArtifact : Effect []
  badDamageArtifact = DealDamage This (Lit 1) (target (HasType Artifact))

-- A phrase needs a positive head ([CR#105.1,608.2d]): "choose a
-- noncolor" heads nothing — Not is a modifier, never a head.
failing "Headed"
  badNegatedQualityHead : Effect []
  badNegatedQualityHead = Choose (A (Not (QualityNoun Color)))

-- "Target non-player" is refused EARLIER than headlessness now: the
-- universal player word names one of the people in the game
-- ([CR#102.1]), so there is no complement class for "non-" to take —
-- the same argument the class word's row makes, and the negation is
-- unwritable before the phrase ever reaches the head gate. Probed at
-- the bare predicate: nested inside `target`'s auto search this
-- failure misreports as the outer `Headed` one.
failing "Negatable"
  badNegatedPlayerHead : Predicate [] Player
  badNegatedPlayerHead = Not AnyPlayer

-- Negation binds nothing: "a creature an opponent DOESN'T control"
-- names no opponent for "that player" to read.
failing "countWord"
  badNegatedAntecedent : Effect []
  badNegatedAntecedent = Sequentially [Tap (target (And [creature, Not (ControlledBy anOpponent)])),
                                       losesLife (That PlayerW) (Lit 1)]

-- An object is in ONE zone: contradicted zone conjuncts refuse in
-- either order.
failing "ZoneCoherent"
  badConflictingZones : Effect []
  badConflictingZones = destroy (target (And [creature, InZone BattlefieldZ, InZone GraveyardZ]))

-- Targets are objects and players [CR#115.1]: "target color" is
-- unwritten — qualities are chosen, never targeted.
failing "Targetable"
  badTargetColor : Effect []
  badTargetColor = Choose (target (QualityNoun Color))

-- The one-shot stat modification takes a battlefield object: a dead
-- referent doesn't get +3/+3.
failing "OnBattlefield"
  badGetsGraveyard : Effect []
  badGetsGraveyard = Sequentially [destroy (target creature),
                                   Gets It 3 3 (Just UntilEndOfTurn)]

-- A card never enters another player's hand [CR#400.3]: owned
-- destinations are owner-routed, so this is unwritable.
failing "DestOk"
  badMoveToTargetsHand : Effect []
  badMoveToTargetsHand = Move (target creature) (HandOf (target AnyPlayer))

-- Only a battlefield permanent is destroyable [CR#701.8a].
failing "OnBattlefield"
  badDestroyGraveyard : Effect []
  badDestroyGraveyard = destroy (target (And [creature, InZone (GraveyardOf You)]))

-- Discarding moves a card from a HAND [CR#701.9a]: a battlefield
-- creature is not discardable.
failing "DiscardOk"
  badDiscardBattlefield : Effect []
  badDiscardBattlefield = discards You (A creature)

-- The tag and its body agree: a Destroy-tagged exile would let
-- indestructible cant an exile [CR#701.8b,702.12b].
failing "TagBody"
  badDestroyTaggedExile : Effect []
  badDestroyTaggedExile = Composite Destroy (Move (target creature) ExileZ)

-- The zone demand lives on the tag-body relation: the raw Composite
-- spelling proves what the macro proves [CR#701.8a].
failing "OnBattlefield"
  badCompositeDestroyGraveyard : Effect []
  badCompositeDestroyGraveyard =
    Composite Destroy (Move (target (And [creature, InZone GraveyardZ])) GraveyardZ) {ok = DestroyB}

-- An agentive tag cannot shed its actor [CR#701.21a]: the raw
-- Composite spelling of sacrifice is refused outright.
failing "NonAgentive"
  badAgentlessSacrifice : Effect []
  badAgentlessSacrifice = Composite Sacrifice (Move (A creature) GraveyardZ) {ok = SacrificeB}

-- Discarding moves a hand card [CR#701.9a]: the demand rides the tag
-- relation, so a battlefield "discard" is unspellable under Does too
-- — and with it the forged stamp `TheVerbed Discard` would read.
failing "DiscardOk"
  badDoesDiscardBattlefield : Effect []
  badDoesDiscardBattlefield = Does You Discard (Move (A creature) GraveyardZ) {tb = DiscardB}

-- Two creatures have no single power [CR#208.1] — the aggregate is
-- written explicitly ("the total power of the sacrificed creatures",
-- Soulblast), and is future vocabulary.
failing "OneOf"
  badGroupPower : Effect []
  badGroupPower = Sequentially [Choose (TargetGroup (exactly 2) creature),
                                gainsLife You (powerOf Them)]

-- Two cards need not share an owner [CR#108.3] — oracle writes the
-- plural relational ("their owners' hands", Aether Burst), future
-- vocabulary.
failing "OneOf"
  badGroupOwner : Effect []
  badGroupOwner = Sequentially [Choose (TargetGroup (exactly 2) creature),
                                losesLife (OwnerOf Them) (Lit 1)]

-- A written quantity permits at least one: "zero target creatures" is
-- unwritten English, and so is the "up to zero" spelling of it — the
-- demand is on the quantity's MAXIMUM. (One IS writable: it is the
-- singular form the `target` macro spells, rendered without its
-- numeral.)
failing "NonZeroQ"
  badZeroGroup : Effect []
  badZeroGroup = Choose (TargetGroup (exactly 0) creature)

-- The binary fight frame takes singular combatants — a group versus
-- one has no defined pairing; the plural form is the reciprocal
-- "those creatures fight each other" (ledger).
failing "OneOf"
  badFightGroup : Effect []
  badFightGroup = Fights (TargetGroup (exactly 2) creature) (target creature)

-- "a creature two target opponents control": an object has one
-- controller [CR#109.4]; the union possessor ("creatures your
-- opponents control") is the player-groups vocabulary (ledger).
-- (Probed at the predicate itself: inside a larger phrase the stuck
-- slot stalls the outer coherence search instead.)
failing "OneOf"
  badControlledByGroup : Predicate [] Object
  badControlledByGroup = ControlledBy (TargetGroup (exactly 2) Opponent)

-- Hands and graveyards are per-player zones [CR#400.1]: one zone
-- owned by two players at once is unwritable.
failing "OneOf"
  badGraveyardOfGroup : ZoneExpr []
  badGraveyardOfGroup = GraveyardOf (TargetGroup (exactly 2) Opponent)

-- The minted dies-watcher is singular (Graceful Reprieve's shape);
-- plural watches wait for corpus evidence.
failing "OneOf"
  badDiesGroup : Effect []
  badDiesGroup = Delayed (DiesThisTurn (TargetGroup (exactly 2) creature)) (gainsLife You (Lit 1))

-- A bare type word denotes a permanent [CR#109.2], and what was
-- discarded left a HAND: "the discarded creature" is unwritten (the
-- corpus discard family reads "the discarded card" only) — the
-- stamp's at-verb frame refuses the type word.
failing "countVerbed"
  badDiscardedCreatureWord : Activated []
  badDiscardedCreatureWord =
    MkActivated (discards You (AAtRandom (And [creature, InZone HandZ])))
                (DealDamage This
                            (manaValueOf (TheVerbed Discard (TypeW Creature)))
                            (target AnyTarget))

-- "Of their choice" is a possessive pronoun: it demands a player
-- antecedent (a subject or one distributive group [CR#608.2d]) —
-- bare "Destroy a creature of their choice" is unwritten.
failing "countOnes Player"
  badUnboundTheirChoice : Effect []
  badUnboundTheirChoice = destroy (ATheirChoice creature)

-- "That much" with nothing done yet: no outcome to read.
failing "countOnes Outcome"
  badThatMuchUnbound : Effect []
  badThatMuchUnbound = DealDamage This ThatMuch (target AnyTarget)

-- Two event clauses leave "that much" ambiguous — outcomes obey the
-- same strict uniqueness as every read.
failing "countOnes Outcome"
  badThatMuchAmbig : Effect []
  badThatMuchAmbig = Sequentially [DealDamage This (Lit 3) (target AnyTarget),
                                   losesLife You (Lit 2),
                                   gainsLife You ThatMuch]

-- ===== Chapter thirteen negatives: the refuse-nonsense wave =====
-- (Probed at the smallest construct that carries the gate — a bare
-- `Predicate`/`Amount`/`Noun` where one exists: an auto-search failure
-- nested inside another auto stalls the OUTER search and misreports.)

-- "Other" needs a head-COMPATIBLE anchor. The guide reserves
-- "another" for excluding the source or first referent and writes two
-- separately described roles WITHOUT it ("target creature and target
-- planeswalker"); the corpus pairs "other" only with overlapping
-- heads. A land target is no anchor for "another creature" — even
-- though sharing an object across such slots is rules-legal
-- ([CR#601.2c]), which is what makes this templating, gated at the
-- conjunction where the head type is known.
failing "OtherAnchored"
  badOtherCrossHead : Effect []
  badOtherCrossHead = Sequentially [destroy (target (HasType Land)),
                                    destroy (target (And [creature, Other]))]

-- Every corpus for-each domain is noun-headed: "for each you control"
-- names no set to count — the positive-head demand the determiners
-- already carry (finding 39), now on the counted-set amount.
failing "Headed"
  badForEachHeadless : Amount []
  badForEachHeadless = forEach (ControlledBy You)

-- A written numeral is at least one — "1 life for each 0 creatures" is
-- unwritten English. (The comparisons that legitimately carry zero read
-- a count rather than write one; `Lit` stays ungated.)
failing "AtLeastOne"
  badForEachZero : Amount []
  badForEachZero = nForEach 0 creature

-- Zone negation itself is REAL oracle — "Each Vampire creature card
-- you own that isn't on the battlefield has madness." (Falkenrath
-- Gorger) — so `Not (InZone …)` stays writable. What it cannot do is
-- contradict the zone the phrase itself places its referent in: a bare
-- "creature" means the battlefield [CR#109.2].
failing "ZoneCoherent"
  badNotOnBattlefield : Predicate [] Object
  badNotOnBattlefield = And [creature, Not (InZone BattlefieldZ)]

-- No member negates a sibling: "of the chosen color and not of the
-- chosen color" describes nothing.
failing "ContradictionFree"
  badQualityContradiction : Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
  badQualityContradiction = And [OfChosen Color, Not (OfChosen Color)]

-- …and the nested spelling is refused the same way — the member scan
-- flattens conjunctions, so a clash one level down is no laundering.
-- (A nested ZONE clash is the same nonsense but trips the zone gate
-- first, so the nested probe uses a type contradiction to keep the pin
-- unambiguous.)
failing "ContradictionFree"
  badNestedContradiction : Predicate [] Object
  badNestedContradiction = And [creature, And [Not creature]]

-- Attackers are declared from creatures their controller controls
-- ([CR#508.1a]) and leaving the battlefield removes a permanent from
-- combat ([CR#506.4]), so the status word seeds its own zone and
-- clashes with an explicit hand clause — no new gate, the existing
-- coherence one.
failing "ZoneCoherent"
  badAttackingInHand : Predicate [] Object
  badAttackingInHand = And [Attacking, InZone HandZ]

-- A COLLECTIVE group damage subject is unattested: oracle distributes
-- the frame ("Each creature you control deals 1 damage to that
-- creature.", Case of the Gateway Express) or names one source.
failing "DamageSource"
  badGroupDamageSource : Effect []
  badGroupDamageSource = DealDamage (TargetGroup (exactly 2) creature) (Lit 3) (target AnyPlayer)

-- The class word is never negated: [CR#115.4] defines "any target"
-- positively as the damage target class, and the guide forbids using it
-- as a synonym for "any object" — so there is nothing for "non-" to
-- take a complement in.
failing "Negatable"
  badNegatedAnyTarget : Predicate [] Object
  badNegatedAnyTarget = Not AnyTarget

-- …and it takes no modifier but "other" (Arc Trail's "any other
-- target"): "any target in a graveyard" would be that forbidden
-- synonym, spelled as a restriction.
failing "AnyTargetLone"
  badAnyTargetInGraveyard : Predicate [] Object
  badAnyTargetInGraveyard = And [AnyTarget, InZone GraveyardZ]

-- "Any target" is ITSELF the targeting form, so only the targeting
-- determiners admit it: "a any target" and "each any target" are
-- unwritable.
failing "AnyTargetFree"
  badAnyTargetUnderA : Noun [] Object
  badAnyTargetUnderA = A AnyTarget

-- …and the targeting determiner admits it only at the quantities that
-- spell it: "any target" is the singular damage-class form [CR#115.4]
-- defines, so `target AnyTarget` is the whole of the exact case (bolt,
-- Arc Trail, Pyromancy), joined by the up-to mention the corpus writes
-- outright ("each of up to two targets", Fall of the Titans). The
-- plural damage-class forms [CR#115.4] names alongside it carry
-- structures the bare exact-group mention does not spell — a division,
-- an each-of recipient (ledger) — so "two any targets" stays closed.
failing "AnyTargetAtCount"
  badGroupAnyTarget : Noun [] Object
  badGroupAnyTarget = TargetGroup (exactly 2) AnyTarget

-- …and the unbounded quantity with it: "any number of any targets"
-- names no attested structure either, and the ban has to see the
-- quantity's SHAPE rather than a numeral to refuse it.
failing "AnyTargetAtCount"
  badAnyNumberAnyTarget : Noun [] Object
  badAnyNumberAnyTarget = TargetGroup anyNumber AnyTarget

-- A type-worded self-reference denotes the PERMANENT ([CR#109.2]), and
-- discarding moves a card from a HAND ([CR#701.9a]): "discard this
-- creature" is unwritable — cycling's cost says "this card"
-- (`cyclingCost`).
failing "DiscardOk"
  badDiscardThisCreature : Effect []
  badDiscardThisCreature = discards You thisCreature

-- The ascription is the SOURCE's, and only the source's: "target
-- creature" already says its type in the predicate it carries, so
-- sorting it a second time spells nothing new — and it would carry
-- [CR#109.2]'s battlefield projection onto a phrase that never argued
-- for it. The closed table is the whole refusal.
failing "Ascribable"
  badAscribedTarget : Noun [] Object
  badAscribedTarget = AsType Creature (target creature)

-- ===== Laundering routes: wrappers, doubled words, untracked reads =====

-- Negation is ATOMIC: oracle's non-/isn't/doesn't attaches to one
-- modifier, and a conjunction is negated per-member (De Morgan is the
-- writer's job). A singleton `And` would otherwise launder every
-- Negatable ban — `predEq (And _) _ = False` makes the wrapper
-- invisible to the contradiction scan as well.
failing "Negatable"
  badNegatedConjunction : Predicate [] Object
  badNegatedConjunction = Not (And [creature])

-- The syntactically identical contradiction, no longer laundered by a
-- vacuous member equality: "you" denotes the same player at both
-- mentions, so the phrase asserts and denies one fact of one referent.
-- `nounEqRef You You` is exactly the case the conservative relation
-- can prove — two target mentions would not be, and are not.
failing "ContradictionFree"
  badControlContradiction : Predicate [] Object
  badControlContradiction = And [ControlledBy You, Not (ControlledBy You)]

-- Only a creature can attack ([CR#506.3]), so the status word
-- presupposes the type as well as the zone and "attacking noncreature"
-- describes nothing — the finding-43 shape again: a projection made
-- honest, the refusal falling out of the existing coherence gate.
failing "ContradictionFree"
  badAttackingNoncreature : Predicate [] Object
  badAttackingNoncreature = And [Attacking, Not creature]

-- The class word is written ONCE: no corpus line repeats it inside one
-- phrase, and "any target and any target" names one referent twice.
failing "AnyTargetLone"
  badDoubleAnyTarget : Predicate [] Object
  badDoubleAnyTarget = And [AnyTarget, AnyTarget]

-- …and so is the modifier: the guide's selector order gives
-- other/another a single slot. (Posed in a creature-target context so
-- the anchor presupposition itself is satisfied — what refuses is the
-- doubling, not the witness search.)
failing "OtherAnchored"
  badDoubleOther : Predicate [MkBinding TargetD Object OneOf
                                        (ObjectP (Just Creature) (Just Battlefield) Nothing)] Object
  badDoubleOther = And [creature, Other, Other]

-- The class word hides just as poorly inside an EMBEDDED noun: a
-- relative clause's possessor is a phrase like any other, so "a
-- creature the controller of any target controls" spells "any target"
-- under the non-targeting determiner that forbids it.
failing "AnyTargetFree"
  badAnyTargetEmbedded : Noun [] Object
  badAnyTargetEmbedded = A (And [creature, ControlledBy (ControllerOf (target AnyTarget))])

-- The untracked exception belongs to the bare self-reference alone —
-- it is not a free pass for every unplaced referent. A READ whose
-- antecedent records no zone is not thereby a hand card ([CR#701.9a]
-- moves one FROM a hand), which is precisely what the old zone-level
-- gate could not distinguish. (Posed at a zoneless binding — the shape
-- a singular object mention takes before anything places it.)
failing "DiscardOk"
  badDiscardIt : Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing)]
  badDiscardIt = discards You It

-- …and the class word is no hand card either: it heads no zone clause
-- and takes no [CR#109.2] default (it describes no object to place),
-- so the phrase reaches discard with no zone at all — neither the bare
-- self-reference nor a tracked hand card, which is the whole of the
-- gate.
failing "DiscardOk"
  badDiscardAnyTarget : Effect []
  badDiscardAnyTarget = discards You (target AnyTarget)

-- A controller relation presupposes the battlefield: objects that are
-- neither on the stack nor on the battlefield "aren't controlled by any
-- player" ([CR#109.4]), so "a creature you control in your graveyard"
-- places its referent in two zones at once — the finding-43 shape once
-- more, a projection made honest and the existing coherence gate
-- supplying the refusal.
failing "ZoneCoherent"
  badControlledInGraveyard : Predicate [] Object
  badControlledInGraveyard = And [creature, ControlledBy You, InZone GraveyardZ]

-- ===== The converge-on-core round: sequence arity =====

-- A sequence of no clauses is no instruction at all. Core admits
-- `Sequentially([])` structurally — it is a data shape there, and an
-- empty body is a useful identity for the engine — but nothing writes
-- an empty sentence list, so the workbench refuses what it cannot
-- render.
failing "AtLeastTwo"
  badEmptySequence : Effect []
  badEmptySequence = Sequentially []

-- …and a sequence of one clause is not a sequence either: it spells
-- exactly what the clause alone spells, and one meaning gets one
-- spelling.
failing "AtLeastTwo"
  badSingletonSequence : Effect []
  badSingletonSequence = Sequentially [destroy (target creature)]

-- ===== The kind-union round: the class word places nothing =====

-- "Any target" NAMES [CR#115.4]'s damage class — creature, player,
-- planeswalker, or battle — instead of describing an object, so
-- [CR#109.2] has nothing to place on the battlefield and the phrase
-- projects no zone. Destruction moves a battlefield permanent
-- ([CR#701.8a]); the corpus writes no "Destroy any target", and the
-- refusal now falls out of the zone gate the verb already had.
failing "OnBattlefield"
  badDestroyAnyTarget : Effect []
  badDestroyAnyTarget = destroy (target AnyTarget)

-- The same refusal one verb over and at the raw constructor — tapping
-- takes a battlefield object ([CR#701.26a]) — so it is the projection
-- that changed, not one macro's demand.
failing "OnBattlefield"
  badTapAnyTarget : Effect []
  badTapAnyTarget = Tap (target AnyTarget)

-- …and the mention is no better placed when it is read back: the
-- binding an any-target phrase introduces records the phrase's own
-- silence, so "it" inherits no battlefield the phrase never claimed.
failing "OnBattlefield"
  badDestroyAnyTargetRemention : Effect []
  badDestroyAnyTargetRemention = Sequentially [DealDamage This (Lit 3) (target AnyTarget),
                                               destroy It]

-- The recipient gate reads the NOUN, and that is what keeps the class
-- word's own row from becoming a zone-free hole: bare `This` is the
-- source as an object ("this spell") and projects no zone either, but
-- it is no damage recipient ([CR#120.1a] — a battle, a creature, or a
-- planeswalker; players by [CR#120.1]).
failing "DamageRecipient"
  badDamageThis : Effect []
  badDamageThis = DealDamage This (Lit 1) This

-- ===== The disjunction chapter: what is not an alternative =====

-- A coordination offers alternatives, so it needs two. At one it
-- offers none and spells exactly what the bare alternative spells; at
-- zero it spells nothing at all — the arity discipline `Sequentially`
-- already carries, one construction over.
failing "TwoDisjuncts"
  badEmptyOr : Predicate [] Object
  badEmptyOr = Or []

failing "TwoDisjuncts"
  badSingletonOr : Predicate [] Object
  badSingletonOr = Or [creature]

-- …and the same argument once more at two: "artifact or artifact"
-- offers a choice between a thing and itself.
failing "DistinctDisjuncts"
  badRepeatedDisjunct : Predicate [] Object
  badRepeatedDisjunct = Or [artifact, artifact]

-- Alternatives are PARALLEL — each one has to be able to stand where
-- the others stand. A head noun and a bare status word cannot:
-- "artifact or attacking" leaves the second alternative nothing to be,
-- and the guide says as much outright ("Repeat the carrier when the
-- alternatives have different domains or modifiers").
failing "ParallelDisjuncts"
  badHeadlessDisjunct : Predicate [] Object
  badHeadlessDisjunct = Or [artifact, Attacking]

-- The same demand about PLACE. Oracle does write cross-zone
-- alternatives — "an Equipment card from your hand or graveyard",
-- seventeen corpus lines — but under a shared preposition and with a
-- zone that can name two. This projection names one, so the phrase is
-- refused rather than quietly placed on the battlefield by default
-- (the axis is ledgered).
failing "ParallelDisjuncts"
  badCrossZoneDisjunction : Predicate [] Object
  badCrossZoneDisjunction = Or [InZone HandZ, InZone GraveyardZ]

-- "Any target" is ALREADY the union [CR#115.4] fixes — creatures,
-- players, planeswalkers, or battles — so coordinating it with an
-- alternative re-opens a closed class.
failing "CoordinableDisjuncts"
  badAnyTargetInOr : Predicate [] Object
  badAnyTargetInOr = Or [AnyTarget, creature]

-- …and a singleton conjunction wrapped around it launders nothing:
-- every alternative is read through `flattenPs`, which is the lesson
-- the negation row learned when `And [x]` could still hide anything.
failing "CoordinableDisjuncts"
  badAnyTargetInOrLaundered : Predicate [] Object
  badAnyTargetInOrLaundered = Or [And [AnyTarget], creature]

-- "Other" fills one selector slot for the whole coordinated phrase
-- ("Another target Wolf or Werewolf you control"), so it is not an
-- alternative either — and the corpus puts it outside the coordination
-- every time it appears with one.
failing "CoordinableDisjuncts"
  badOtherInOr : Predicate [MkBinding TargetD Object OneOf
                                      (ObjectP (Just Creature) (Just Battlefield) Nothing)] Object
  badOtherInOr = Or [And [creature, Other], land]

-- Nesting is the flat coordination written with brackets oracle has no
-- way to print. Core reaches the same place by flattening the two
-- spellings together in `normalize`; the workbench refuses the second
-- one instead.
failing "CoordinableDisjuncts"
  badNestedOr : Predicate [] Object
  badNestedOr = Or [Or [creature, land], artifact]

-- Negation attaches to one modifier at a time. The writer spells
-- "noncreature, nonland card" — comma-chained atoms, plentiful — and
-- never "non-(creature or land)", which the corpus does not write once.
failing "Negatable"
  badNegatedDisjunction : Predicate [] Object
  badNegatedDisjunction = Not (Or [creature, land])

-- Damage can't be dealt to an object that isn't a battle, a creature,
-- or a planeswalker ([CR#120.1a]), and a disjunctive head fixes no
-- type at all — so the phrase that declines to say WHICH type it names
-- cannot become a damage recipient on the strength of that silence.
-- (The refusal surfaces at the recipient gate, as `badDamageThis`
-- does; what closed underneath it is `DamageableTy`'s untyped row.)
failing "DamageRecipient"
  badDamageDisjunctHead : Effect []
  badDamageDisjunctHead = DealDamage This (Lit 2) (target (Or [artifact, enchantment]))

-- The contrast positive's mirror: alternatives that AGREE on a zone
-- still project it, so an attacking-or-blocking creature stands on the
-- battlefield and cannot also be in a graveyard.
failing "ZoneCoherent"
  badAttackingOrBlockingInGraveyard : Predicate [] Object
  badAttackingOrBlockingInGraveyard =
    And [creature, Or [Attacking, Blocking], InZone GraveyardZ]

-- …and they project the TYPE they agree on the same way: only a
-- creature can attack or block ([CR#506.3]), a presupposition the
-- disjunction inherits from both alternatives at once.
failing "ContradictionFree"
  badNoncreatureAttackingOrBlocking : Predicate [] Object
  badNoncreatureAttackingOrBlocking = And [Not creature, Or [Attacking, Blocking]]

-- A disjunction is a binding HOLE, for the reason negation is one:
-- exactly one alternative is realized and the phrase never says which,
-- so a possessor written inside one of them names nobody the next
-- sentence can read.
failing "countWord"
  badDisjunctAntecedent : Effect []
  badDisjunctAntecedent =
    Sequentially [Tap (target (Or [And [creature, ControlledBy anOpponent],
                                   And [land, ControlledBy You]])),
                  losesLife (That PlayerW) (Lit 1)]

-- ===== What a wrapper hides: buried seeds, head sets, re-minted trees =====

-- A presupposition written inside an alternative is written all the
-- same. Both alternatives here are attacking or blocking, so both
-- demand a creature ([CR#506.3]) and the conjunction may not go on to
-- negate the type — but the conjunction BURIED in each alternative hid
-- that from the scan, `flattenPs` leaving an `Or` whole by design.
failing "ContradictionFree"
  badWrappedStatusLaunder : Predicate [] Object
  badWrappedStatusLaunder =
    And [Or [And [artifact, Attacking], And [land, Blocking]], Not creature]

-- Alternatives that place their referent differently are not
-- alternatives at all: "attacking artifact" stands on the battlefield
-- and a bare "land" says nothing about where it stands. Compared
-- through [CR#109.2]'s default the two looked parallel, and the
-- disagreement came back instead as a projection of NOTHING — which
-- was enough to let `And [this, InZone GraveyardZ]` stand, a phrase
-- placing its referent in two zones at once. Refused at the
-- coordination, where the disagreement is.
failing "ParallelDisjuncts"
  badPartialZoneJoin : Predicate [] Object
  badPartialZoneJoin = Or [And [artifact, Attacking], land]

-- The singular quantity licenses the class word as the phrase's HEAD,
-- not wherever it can hide: "target creature the controller of any
-- target controls" spells it in a possessor, where a counted mention
-- forbids it exactly as "a" does (`badAnyTargetEmbedded`).
failing "AnyTargetAtCount"
  badEmbeddedAnyTargetExact1 : Noun [] Object
  badEmbeddedAnyTargetExact1 =
    target (And [creature, ControlledBy (ControllerOf (target AnyTarget))])

-- A range runs upward: "between three and two target creatures" names
-- an empty interval, and the number read off the maximum would call
-- that plural besides.
failing "WellFormedQ"
  badDescendingRange : Noun [] Object
  badDescendingRange = TargetGroup (Range (Just 3) (Just 2)) creature

-- …and it starts at one. "Zero or more target creatures" is a second
-- spelling of "any number of target creatures" — [CR#107.1c] has "any
-- number" permit zero already — and the corpus writes only the second.
failing "WellFormedQ"
  badZeroLowerRange : Noun [] Object
  badZeroLowerRange = TargetGroup (Range (Just 0) Nothing) creature

-- A coordinated head offers one type per alternative, not none: an
-- earlier LAND anchors "another target artifact or enchantment" no
-- better than it anchors "another target artifact" (`badOtherCrossHead`).
-- Posed at a land target, the anchor being the point.
failing "OtherAnchored"
  badDisjunctiveOtherCrossHead : Predicate [MkBinding TargetD Object OneOf
                                                      (ObjectP (Just Land) (Just Battlefield) Nothing)] Object
  badDisjunctiveOtherCrossHead = And [Or [artifact, enchantment], Other]

-- "Artifact or artifact" is caught by comparing the two words; the
-- same repetition spelled with a modifier went through, the member
-- equality having declined to look inside a conjunction at all.
failing "DistinctDisjuncts"
  badRepeatedStructuredDisjunct : Predicate [] Object
  badRepeatedStructuredDisjunct =
    Or [And [creature, ControlledBy You], And [creature, ControlledBy You]]

-- A sequence's ELEMENTS are clauses. Nesting one re-mints the
-- right-nested tree the n-ary list replaced and spells a
-- three-sentence card a second way; what may still hold a sequence is
-- a clause slot taking a body, not an element position.
failing "NotSeq"
  badNestedSequence : Effect []
  badNestedSequence =
    Sequentially [Sequentially [destroy (target creature), exile (target creature)],
                  destroy (target land)]

-- ===== What a restriction may forbid, of whom, and for how long =====

-- Only a creature can attack or block ([CR#506.3]), so a land has no
-- grant for the deed to remove. The demand is the fight slots' shape
-- over a DIFFERENT table: `combatant` answers a non-combat damage deed
-- ([CR#701.14d]), `deedType` the combat grants themselves.
failing "DeedParticipant"
  badCantAttackLand : Effect []
  badCantAttackLand = cantAttack (target land) ThisTurn

-- A coordinated head fixes no type (finding 50), and an untyped head
-- cannot prove participation: "target creature or land" would have to
-- carry the attack grant on an alternative that has none. The silence
-- is not permission — `DamageableTy` learned the same lesson.
failing "DeedParticipant"
  badCantDisjunctSubject : Effect []
  badCantDisjunctSubject = cantBlock (target (Or [creature, land])) ThisTurn

-- Splitting the voice off the deed word made "can't be attacked"
-- WRITABLE as a term for the first time, so the table has to say why
-- it is not writable of a creature: only a player, a planeswalker, or
-- a battle can be attacked ([CR#506.3]). The phrase itself is real
-- oracle ("The Aetherspark can't be attacked"; "until your next turn,
-- you can't be attacked except by creatures with flying") and waits on
-- those types and on the ledgered player subject — which is why this
-- voice gets no macro of its own.
failing "DeedParticipant"
  badCantBeAttacked : Effect []
  badCantBeAttacked = Cant (target creature) Attack Patient ThisTurn

-- Combat is fought on the battlefield: a permanent that leaves it is
-- removed from combat ([CR#506.4]), so a graveyard card has no deed to
-- be denied. The gate is the one destroy, tap, and "gets" already
-- carry.
failing "OnBattlefield"
  badCantInGraveyard : Effect []
  badCantInGraveyard =
    cantBlock (target (And [creature, InZone GraveyardZ])) ThisTurn

-- The class word names [CR#115.4]'s damage class, describes no object,
-- and so places none — and the restriction needed no rule of its own to
-- refuse it, the projection doing it through the battlefield demand
-- exactly as for destroy and tap (`badDestroyAnyTarget`).
failing "OnBattlefield"
  badCantAnyTarget : Effect []
  badCantAnyTarget = cantBlock (target AnyTarget) ThisTurn

-- The same span, the wrong word. Two hundred ninety-six corpus lines
-- write a one-shot restriction and every one of them says "this turn";
-- "until end of turn" belongs to the grants, which invert the count
-- (`badGainsThisTurn`).
failing "RestrictionSpan"
  badCantUntilEndOfTurn : Effect []
  badCantUntilEndOfTurn = cantBlock (target creature) UntilEndOfTurn

-- …and the inverse, which is what keeps the new word from opening a
-- hole: no corpus line grants an ability "this turn".
failing "GrantSpan"
  badGainsThisTurn : Effect []
  badGainsThisTurn = Gain (target creature) (KeywordAbility Flying) (Just ThisTurn)

-- The stat change writes the grant's adverbial too — "Target creature
-- gets +3/+3 until end of turn", never "this turn".
failing "GrantSpan"
  badGetsThisTurn : Effect []
  badGetsThisTurn = Gets (target creature) 3 3 (Just ThisTurn)

-- A durationless "can't" is the STATIC ability line ("Enchanted
-- creature can't attack", Pacifism), which is a different construction
-- and the parked ability layer's — so the refusal here is the ABSENT
-- SLOT rather than a gate, the span not being optional. Pinned on the
-- slot's own name, which is why it carries one.
failing "span : Duration"
  badStaticCant : Effect []
  badStaticCant = cantBlock (target creature)

-- ===== What carries a bound, and how it is written =====

-- A bound is a MODIFIER: "with power 2 or less" describes something
-- and names nothing, so it cannot be what a determiner determines.
-- The same refusal "attacking" and "other" already take.
failing "Headed"
  badBareComparison : Noun [] Object
  badBareComparison = target (Compare Power OrLess (Lit 2))

-- Only a creature has power ([CR#208.3] — a noncreature permanent has
-- none, and a noncreature object off the battlefield has one only if
-- it is printed there), so a phrase that bounds power and denies the
-- type describes nothing. The presupposition rides the bound exactly
-- as it rides "attacking" (`badAttackingNoncreature`), the type table
-- supplying the answer in place of a fixed row.
failing "ContradictionFree"
  badNoncreaturePower : Predicate [] Object
  badNoncreaturePower = And [Compare Power OrLess (Lit 2), Not creature]

-- Oracle never negates a bound. It flips the comparator instead, and
-- can: the game's numbers are integers ([CR#107.1]), so "not power 2
-- or less" IS "power 3 or greater" exactly, with no gap between the
-- two for a negation to name. The corpus writes no "non-", no "doesn't
-- have power", and no "without power 2" — the same De Morgan the
-- writer does for conjunctions (finding 42), here made exact by the
-- discreteness rather than by chaining.
failing "Negatable"
  badNegatedComparison : Predicate [] Object
  badNegatedComparison = Not (Compare Power OrLess (Lit 2))

-- One phrase, one bound. The empty pair is the obvious case — power at
-- most two and at least four describes nothing — but the gate is a
-- multiplicity cap rather than a range solver, so the SATISFIABLE pair
-- is refused on the same evidence: no corpus noun phrase carries two
-- bounds at all. An interval is a construction with its own word, not
-- two qualifiers stacked.
failing "LoneComparison"
  badDoubleComparison : Predicate [] Object
  badDoubleComparison = And [creature, Compare Power OrLess (Lit 2),
                             Compare Power OrGreater (Lit 4)]

-- The bound is WRITTEN — a numeral or the announced X — and a phrasal
-- standard belongs to the other frame. Oracle writes "with power less
-- than this creature's power", never "with this creature's power or
-- less": admitting the amount here would spell a real comparison in a
-- word order the language does not use, so the frame is refused and
-- the comparison-to-a-phrase family waits on the ledger.
failing "WrittenBound"
  badPhrasalBound : Predicate [] Object
  badPhrasalBound = Compare Power OrLess (powerOf This)

-- An alternative repeated word for word is no alternative, and a bound
-- is compared by all three of its written parts to see it — the row
-- `predEq` gained this chapter, on the lesson `And` taught it
-- (`badRepeatedStructuredDisjunct`).
failing "DistinctDisjuncts"
  badRepeatedComparisonDisjunct : Predicate [] Object
  badRepeatedComparisonDisjunct = Or [Compare Power OrLess (Lit 2),
                                      Compare Power OrLess (Lit 2)]

-- Alternatives must presuppose alike, and two characteristics do not:
-- a power bound demands a creature where a mana value bound demands
-- nothing, so "with power 2 or less or mana value 3 or less" commits
-- on one side and stays silent on the other. This is the guide's
-- repeat-the-carrier rule reaching a new pair of seeds with no gate of
-- its own — finding 51's shape, one chapter on.
failing "ParallelDisjuncts"
  badMixedCharacteristicDisjunct : Predicate [] Object
  badMixedCharacteristicDisjunct = Or [Compare Power OrLess (Lit 2),
                                       Compare ManaValue OrLess (Lit 3)]

-- The class word takes no modifiers but "other" ([CR#115.4] fixes the
-- class by rule, and a bound would narrow it), so the qualifier is
-- refused by the gate that was already there — no comparison rule
-- needed to say so.
failing "AnyTargetLone"
  badAnyTargetComparison : Predicate [] Object
  badAnyTargetComparison = And [AnyTarget, Compare Power OrLess (Lit 2)]
