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
bolt = DealDamage This (Lit 3) (Macros.target AnyTarget)

-- "Barrage of Boulders deals 1 damage to each creature you don't
-- control." (Ferocious rider line elided) — the corpus has no "each
-- creature an opponent controls": opponent-scoped sweeps say "you don't
-- control" or plural "your opponents control" (a later chapter's noun).
barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Each Macros.creatureYouDontControl)

-- "Target creature you control deals damage equal to its power to target
-- creature you don't control." — THE in-situ dividend: "its" is read
-- where only slot A precedes, so plain uniqueness resolves it; slot B
-- doesn't exist yet. Hoisted, this exact card needed positional reads.
rabidBite : Effect []
rabidBite = DealDamage (Macros.target Macros.creatureYouControl)
                       (Macros.powerOf It)
                       (Macros.target Macros.creatureYouDontControl)

-- "Target creature you control fights target creature you don't
-- control." (Prey Upon; its reminder text "(Each deals damage equal to
-- its power to the other.)" omitted — a parenthetical gloss, not rules
-- text, and the same expansion finding 20 refuses to treat as the
-- operative spelling) — two same-sort slots are just two argument
-- positions.
preyUpon : Effect []
preyUpon = Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct. The oracle writes ONE verb over coordinated
-- amount+recipient complements; the bench transcribes them as a
-- two-clause `Sequentially`, a named stand-in that mis-orders nothing
-- binding-wise but serializes what the card states as a single
-- instruction (ledger).
arcTrail : Effect []
arcTrail = Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
                         DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

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
cloudshift = Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
                           Move (That CardW) Macros.battlefieldZ]

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (Through the Breach; Splice line elided — real Sneak
-- Attack says "the creature", a definite read this chapter doesn't mint)
-- — the hand-to-battlefield move RESTORES the typed carrier (the head
-- type was projected at introduction, never lost), the choice-determined
-- referent survives the delay [CR#603.7c], and the trailing adverbial
-- is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = Sequentially [Macros.may You (Move (Macros.a (And [Macros.creature, InZone (Macros.handOf You)])) Macros.battlefieldZ),
                                 Macros.gainsHaste (That (TypeW Creature)) Nothing,
                                 Delayed (BeginningOf EndStep Nothing) (Macros.sacrifice You (That (TypeW Creature)))]

-- "Destroy target creature. Its controller loses 2 life." (Bitter
-- Downfall; its cost-reduction line elided) — the relational noun:
-- `ControllerOf It` derives a NEW player referent from the destroyed
-- object (whose "its" still resolves — the retag moved it to the
-- graveyard, it didn't unmention it).
bitterDownfall : Effect []
bitterDownfall = Sequentially [Macros.destroy (Macros.target Macros.creature),
                               Macros.losesLife (ControllerOf It) (Lit 2)]

-- "Tap target creature. It deals damage equal to its power to another
-- target creature." (Deadshot) — pronoun as source, and "another" is
-- the same `Other` modifier "any other target" uses.
deadshot : Effect []
deadshot = Sequentially [Tap (Macros.target Macros.creature),
                         DealDamage It (Macros.powerOf It) (Macros.target (And [Macros.creature, Other]))]

-- "{1}{B}{R}{R}, {T}, Sacrifice this land: It deals 3 damage to target
-- player. That player discards a card. Activate only as a sorcery."
-- (Immersturm Skullcairn; the land's other lines elided) — the ability
-- WHOLE, cost components and activation instruction included, where the
-- same card was three elisions deep last chapter. The cost's sacrificed
-- self is the effect's "It": the cost move mints the referent
-- ([CR#400.7]) and it survives the colon publicly ([CR#400.7j]); then
-- the sorted demonstrative at Player kind and a keyword-action macro
-- (Discard) whose body moves a hand-zone choice.
immersturmSkullcairn : Ability
immersturmSkullcairn =
  Activated (Compound [Mana [Macros.generic 1, Macros.pip Black, Macros.pip Red, Macros.pip Red], TapSymbol,
                       Do (Macros.sacrifice You Macros.thisLand)])
            (Sequentially [DealDamage It (Lit 3) (Macros.target AnyPlayer),
                           Macros.discardsACard (That PlayerW)])
            {window = Just AsSorcery}

-- "{R}, Sacrifice this artifact: It deals 2 damage to any target."
-- (Pyrite Spellbomb, first ability; the card's second ability elided) —
-- the smallest cost-antecedent pair: the sorted self-reference moved by
-- the cost is the only mention "It" can reach. Its mana half is written
-- now, which is what makes the pair a two-component cost rather than a
-- one-component one with a note.
pyriteSpellbomb : Ability
pyriteSpellbomb = Activated (Compound [Mana [Macros.pip Red], Do (Macros.sacrifice You Macros.thisArtifact)])
                            (DealDamage It (Lit 2) (Macros.target AnyTarget))

-- "Target player sacrifices a creature of their choice." (Diabolic
-- Edict) — the declarative clause: the target subject introduces, the
-- verb phrase is typed after it, and "of their choice" is the marked
-- own-choice method on the indefinite (the corpus has no bare "Target
-- player sacrifices a creature").
diabolicEdict : Effect []
diabolicEdict = Macros.sacrifice (Macros.target AnyPlayer) (Macros.aTheirChoice Macros.creature)

-- "Each player sacrifices a creature of their choice." (Innocent
-- Blood) — a group subject: the same clause shape over "each player".
innocentBlood : Effect []
innocentBlood = Macros.sacrifice (Each AnyPlayer) (Macros.aTheirChoice Macros.creature)

-- "Target player discards a card." (Cry of Contrition, first line; its
-- Haunt lines elided) — the declarative discard whose imperative twin
-- is the same macro with `You`.
cryOfContrition : Effect []
cryOfContrition = Macros.discardsACard (Macros.target AnyPlayer)

-- "Destroy target creature an opponent controls. That player loses 3
-- life." (Suspended Sentence; its self-exile clause and Suspend lines
-- elided) — the relative clause's inner mention folds: the indefinite
-- opponent enters the discourse from INSIDE the target's predicate,
-- and the sorted demonstrative reads it.
suspendedSentence : Effect []
suspendedSentence = Sequentially [Macros.destroy (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                                  Macros.losesLife (That PlayerW) (Lit 3)]

-- "Exile this creature, then return it to the battlefield under its
-- owner's control." (Flickering Spirit's activated ability; its
-- mana-only cost elided) — the sorted self-reference moved by the
-- EFFECT: the exile mints the new object's binding mid-sentence
-- ([CR#400.7]) and "it" reads it. "Under its owner's control" is an
-- OVERRIDE of [CR#110.2a]'s default (the ability's controller is the
-- instructed player), so its elision is meaning-carrying — pending the
-- control-assignment axis.
flickeringSpirit : Effect []
flickeringSpirit = Sequentially [Macros.exile Macros.thisCreature,
                                 Move It Macros.battlefieldZ]

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
turnToMist = Sequentially [Macros.exile (Macros.target Macros.creature),
                           Delayed (BeginningOf EndStep Nothing) (Move (That CardW) Macros.battlefieldZ)]

-- "Target creature gains flying until end of turn." (Jump) — the
-- duration as trailing-adverbial data ([CR#611.2a]), and the clause
-- under the envelope that carries it: the grant is the static effect,
-- "until end of turn" the span it lasts.
jump : Effect []
jump = Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.untilEndOfTurn)

-- "Target creature gets +3/+3 until end of turn." (Giant Growth)
giantGrowth : Effect []
giantGrowth = Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn)

-- "Target blocking Wall you control gets +10/+0 until end of combat."
-- (Glyph of Destruction; its damage-prevention and delayed-destruction
-- lines elided, and the "Wall" subtype with them — a creature type the
-- vocabulary cannot spell, so the transcription WIDENS the subject to
-- any blocking creature you control, a named stand-in) — the second
-- endpoint the decomposition opened: combat ends at its own boundary
-- ([CR#511.2]), not the turn's, and the stat change is one of the two
-- constructions that write it.
glyphOfDestruction : Effect []
glyphOfDestruction =
  Macros.gets (Macros.target (And [Blocking, Macros.creature, ControlledBy You])) (PtUp (Lit 10)) (PtUp (Lit 0)) (Just Macros.untilEndOfCombat)

-- "Gabriel Angelfire gains that ability until your next upkeep."
-- (Gabriel Angelfire; its upkeep trigger header elided as Case of the
-- Gateway Express's enters trigger is, and with it the choice the
-- header makes — "choose flying, first strike, trample, or rampage 3"
-- — so "that ability" is transcribed as one of the four written
-- choices; the chosen-ABILITY anaphor waits with the ability layer) —
-- the endpoint the KEYWORD grant writes alone, and the reason the
-- decomposition had to reach a step inside the turn at all.
gabrielAngelfire : Effect []
gabrielAngelfire =
  Macros.gains Macros.thisCreature (KeywordAbility Flying) (Just Macros.untilYourNextUpkeep)

-- "Return target creature card from your graveyard to the
-- battlefield. It gains haste until your next turn." (Bond of
-- Revival) — an owned-zone source and the cross-turn duration; the
-- return is just a Move, and "it" reads the retagged referent.
bondOfRevival : Effect []
bondOfRevival = Sequentially [Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.battlefieldZ,
                              Macros.gainsHaste It (Just Macros.untilYourNextTurn)]

-- "When target creature dies this turn, return that card to the
-- battlefield under its owner's control." (Graceful Reprieve; "under
-- its owner's control" is a meaning-carrying elision — the override of
-- [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (Dies (Macros.target Macros.creature)) {span = Just ThisTurn}
                           (Move (That CardW) Macros.battlefieldZ)

-- "Destroy target creature. You gain life equal to its toughness."
-- (Vraska's Stoneglare; its tutor clause elided) — a last-known read:
-- reads ignore zone, so the dead referent's characteristics stay
-- readable; which values they see is runtime (the ability layer's
-- story).
vraskasStoneglare : Effect []
vraskasStoneglare = Sequentially [Macros.destroy (Macros.target Macros.creature),
                                  Macros.gainsLife You (Macros.toughnessOf It)]

-- "Destroy target creature. Its controller loses life equal to its
-- power plus its toughness." (Phthisis; its Suspend line elided) —
-- the relational noun over the dead referent, and amount arithmetic
-- threading left to right.
phthisis : Effect []
phthisis = Sequentially [Macros.destroy (Macros.target Macros.creature),
                         Macros.losesLife (ControllerOf It) (Plus (Macros.powerOf It) (Macros.toughnessOf It))]

-- "This creature deals damage equal to its power to target creature.
-- That creature deals damage equal to its power to this creature."
-- (Karplusan Yeti's activated ability; its {T} cost elided, and the
-- source-referring "its" is spelled as the self-reference — source
-- mentions don't bind) — the SEQUENTIAL cousin of fight: two ORDERED
-- one-shot damage events, where [CR#701.14a] deals both
-- simultaneously (state-based actions see neither mid-resolution,
-- [CR#704.4]), which is why `Fights` stays primitive.
karplusanYeti : Effect []
karplusanYeti = Sequentially [DealDamage Macros.thisCreature (Macros.powerOf Macros.thisCreature) (Macros.target Macros.creature),
                              DealDamage (That (TypeW Creature)) (Macros.powerOf It) Macros.thisCreature]

-- "Choose two target creatures. Tap those creatures, then unattach
-- all Equipment from them." (Fulgent Distraction; the unattach clause
-- elided) — a counted group mention, read back by the sorted plural
-- demonstrative.
fulgentDistraction : Effect []
fulgentDistraction = Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                                   Tap (Those (TypeW Creature))]

-- "Choose up to four target creature cards in your graveyard that
-- were put there from the battlefield this turn. Return them to the
-- battlefield." (Continue?; its look-back restrictive clause elided —
-- event-history predicates are unminted) — the bounded group, an
-- owned-zone predicate, and the plural wildcard riding the return's
-- retag.
continueSpell : Effect []
continueSpell = Sequentially [Choose (TargetGroup (Macros.upTo 4) (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Move Them Macros.battlefieldZ]

-- "Choose a color. Sudden Demise deals X damage to each creature of
-- the chosen color." (Sudden Demise) — a quality mention: the chosen
-- color enters the discourse like any mention ([CR#105.1]) and the
-- predicate-internal read demands it uniquely; X is the announced
-- cost variable ([CR#107.3a]).
suddenDemise : Effect []
suddenDemise = Sequentially [Choose (Macros.a (QualityNoun Color)),
                             DealDamage This XVal (Each (And [Macros.creature, OfChosen Color]))]

-- "Choose a creature type. Destroy all creatures that aren't of the
-- chosen type." (Kindred Dominance) — the set-level "all" determiner
-- over a negated quality read.
kindredDominance : Effect []
kindredDominance = Sequentially [Choose (Macros.a (QualityNoun CreatureType)),
                                 Macros.destroy (AllOf (And [Macros.creature, Not (OfChosen CreatureType)]))]

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
voyagerStaff : Ability
voyagerStaff = Activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                         (Sequentially [Macros.exile (Macros.target Macros.creature),
                                          Delayed (BeginningOf EndStep Nothing) (Move (TheVerbed Exile CardW) Macros.battlefieldZ)])

-- "{3}{R}, Sacrifice an artifact: Bosh deals damage equal to the
-- sacrificed artifact's mana value to any target." (Bosh, Iron Golem;
-- {3}{R} and its Trample line elided; the self-name is `This`) — the
-- TYPE-word participle noun: the referent is a graveyard card NOW,
-- but "artifact" describes it under the verb — checked on the
-- time-stable projected head type, the axis a declared type word
-- lives on (finding 27).
boshIronGolem : Ability
boshIronGolem = Activated (Compound [Mana [Macros.generic 3, Macros.pip Red],
                                     Do (Macros.sacrifice You (Macros.a (HasType Artifact)))])
                          (DealDamage This
                                      (Macros.manaValueOf (TheVerbed Sacrifice (TypeW Artifact)))
                                      (Macros.target AnyTarget))

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
pyromancy : Ability
pyromancy = Activated (Compound [Mana [Macros.generic 3],
                                 Do (Macros.discards You (Macros.aAtRandom (InZone Macros.handZ)))])
                      (DealDamage Macros.thisEnchantment
                                  (Macros.manaValueOf (TheVerbed Discard CardW))
                                  (Macros.target AnyTarget))

-- "Target opponent loses 1 life for each attacking creature you
-- control. You gain that much life." (Foul-Tongue Shriek) — the
-- event-outcome read: the loss clause introduces its outcome (sort
-- LifeLost, projected from the surface; magnitude runtime), and
-- "that much" reads the unique outcome in scope, sort-blind. The
-- attacking modifier is a battlefield state word ([CR#508.1a]).
foulTongueShriek : Effect []
foulTongueShriek = Sequentially [Macros.losesLife (Macros.target Opponent)
                                           (Macros.forEach (And [Attacking, Macros.creature, ControlledBy You])),
                                 Macros.gainsLife You ThatMuch]

-- "Return target creature to its owner's hand." (Unsummon) — the
-- destination is the bare-scoped hand (`handZ`): [CR#400.3] admits no
-- other hand, so
-- the possessive is derived surface, never stored (finding 34).
unsummon : Effect []
unsummon = Move (Macros.target Macros.creature) Macros.handZ

-- "When this Equipment enters, attach it to up to one target creature
-- you control. Destroy up to one other target creature." (Phantom
-- Blade's trigger; the attach verb waits on the attachment axis, so
-- its TARGETING stands in as the fronted choice clause) — the up-to
-- mention is a real "other" anchor: the witness is the MENTION, not
-- a nonempty denotation (finding 35).
phantomBlade : Effect []
phantomBlade = Sequentially [Choose (TargetGroup (Macros.upTo 1) (And [Macros.creature, ControlledBy You])),
                             Macros.destroy (TargetGroup (Macros.upTo 1) (And [Macros.creature, Other]))]

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
caseOfTheGatewayExpress = Sequentially [Choose (Macros.target Macros.creatureYouDontControl),
                                        DealDamage (Each Macros.creatureYouControl) (Lit 1)
                                                   (That (TypeW Creature))]

-- "Cycling {2}" — "{2}, Discard this card: Draw a card." ([CR#702.29a];
-- the {2} and the draw elided, the draw verb being unminted) — the
-- standing justification for `DiscardOk`'s bare-`This` row: bare `This`
-- is the source as an OBJECT ("this card"), so it projects no zone,
-- while the SORTED self-reference now denotes the permanent
-- ([CR#109.2]) and cannot be discarded (`badDiscardThisCreature`).
cyclingCost : Effect []
cyclingCost = Macros.discards You This

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
rawNonattacking = And [Macros.creature, Not Attacking, Not Blocking]

-- "Exile target creature." spelled raw — the tag-ALIGNED twin of
-- `badDestroyTaggedExile`: same body, agreeing tag. The `TagBody`
-- witness travels explicitly because its auto search is flaky even
-- here, at a concrete site with no nested autos — the same reason the
-- `exile` macro passes `{ok = ExileB}`.
rawExile : Effect []
rawExile = Composite Exile (Move (Macros.target Macros.creature) Macros.exileZ) {ok = ExileB}

-- ===== Alternatives under one determiner: the disjunction chapter =====

-- "Destroy target artifact or enchantment." (Disenchant) — the
-- flagship. The card writes "target" ONCE, so it announces one target
-- ([CR#601.2c]) and the determiner scopes over the whole coordination:
-- the parser brackets it `<<target> <<artifact> <or <enchantment>>>>`,
-- with the alternatives inside the determined phrase rather than
-- beside it. That is why disjunction is a PREDICATE, not a second
-- noun.
disenchant : Effect []
disenchant = Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))

-- "Tap target artifact, creature, or land." (Icy Manipulator; its mana
-- and {T} cost elided) — a third alternative costs no machinery, and
-- the guide puts the serial comma before the coordinator from three
-- items up. This is the argument for a member LIST over a binary
-- connective, and it is core's shape too (`Predicate::Or` takes a
-- slice).
icyManipulator : Effect []
icyManipulator = Tap (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))

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
ratsOfRath = Macros.destroy (Macros.target (And [Or [Macros.artifact, Macros.creature, Macros.land], ControlledBy You]))

-- "Arrows of Justice deals 4 damage to target attacking or blocking
-- creature." — alternatives that CONTRAST: no creature is both, and
-- the phrase is none the worse for it. This is what the coherence
-- scans must not do to a disjunction, because splicing its members
-- into the surrounding conjunction would read the card as "attacking
-- and blocking" and refuse a phrase ninety-five corpus lines write.
arrowsOfJustice : Effect []
arrowsOfJustice = DealDamage This (Lit 4)
                             (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))

-- "another target creature or land" — the selector reaches over the
-- whole coordination, as the corpus writes it ("Another target Wolf or
-- Werewolf you control"), and the anchor it demands is compatible with
-- SOME alternative's head: a coordinated head fixes no type on its
-- referent but offers one per alternative (finding 44). Posed here at
-- an untyped target mention, which any head accepts — the
-- presupposition being the point rather than the test; the cross-head
-- refusal is `badDisjunctiveOtherCrossHead`'s.
anotherDisjunctPhrase : Predicate [MkBinding TargetD Object OneOf
                                             (ObjectP Nothing (Just Battlefield) Nothing Nothing)] Object
anotherDisjunctPhrase = And [Or [Macros.creature, Macros.land], Other]

-- ===== What a clause forbids: the deontics chapter =====

-- "Target creature can't be blocked this turn." (Infiltrate) — the
-- flagship, and the whole card. The clause creates a continuous effect
-- for a stated span ([CR#611.2a]) denying its subject a deed, which is
-- what the declare-blockers step then checks ([CR#509.1b]). The deed
-- word carries the ROLE: the subject of "can't be blocked" is the
-- creature being blocked, core's `on` slot, where "can't block" would
-- put it in `by`.
infiltrate : Effect []
infiltrate = Macros.cantBeBlocked (Macros.target Macros.creature) (Just Macros.thisTurn)

-- "Target creature can't attack this turn." (Change of Heart; its
-- Buyback line elided — a keyword rider, as with Cascade and Splice)
-- — the active voice of the same clause, checked at declare attackers
-- instead ([CR#508.1c]).
changeOfHeart : Effect []
changeOfHeart = Macros.cantAttack (Macros.target Macros.creature) (Just Macros.thisTurn)

-- "Blindblast deals 1 damage to target creature. That creature can't
-- block this turn." (Blindblast; "Draw a card." elided — no draw
-- vocabulary, ledgered) — the restriction takes an ANAPHORIC subject
-- like any other clause: the demonstrative reads the mention the damage
-- clause introduced, and the deontic needs nothing of its own for it.
blindblast : Effect []
blindblast = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                           Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn)]

-- "Any number of target creatures can't block this turn." (Blinding
-- Flare; its Strive cost-modification line elided) — the subject is
-- PLURAL, and the clause demands no grammatical number: a restriction
-- ranges over whatever its subject phrase describes, one creature or a
-- group announced at once.
blindingFlare : Effect []
blindingFlare = Macros.cantBlock (TargetGroup Macros.anyNumber Macros.creature) (Just Macros.thisTurn)

-- ===== A bound on a characteristic: the comparatives chapter =====

-- "Destroy target creature with power 2 or less." (Defeat) — the
-- flagship, and the whole card. The qualifier is a sibling MODIFIER
-- like any other, so the phrase needed no new noun layer: what is new
-- is the three written parts core spells in the same order
-- (`Stat(Power, AtMost, 2)`), and the presupposition that comes with
-- them — only a creature has power ([CR#208.3]).
defeat : Effect []
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare Power AtMost (Lit 2)]))

-- "Destroy target attacking creature with power 3 or less." (Terashi's
-- Verdict) — a status word and a bound in ONE phrase, which is the
-- interaction worth a positive: both members presuppose a creature and
-- the contradiction scan has to let them, positive types stacking
-- rather than clashing. The zone comes from `Attacking` alone; the
-- bound places nothing.
terashisVerdict : Effect []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare Power AtMost (Lit 3)]))

-- "Exile target creature with toughness 4 or greater." (Pillar of
-- Light) — the other comparator and the other creature-gated
-- characteristic, so the two-row vocabulary is spelled end to end by
-- real cards rather than by one card and an argument.
pillarOfLight : Effect []
pillarOfLight =
  Macros.exile (Macros.target (And [Macros.creature, Compare Toughness AtLeast (Lit 4)]))

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
  Sequentially [Macros.destroy (Macros.target (And [Macros.artifact,
                                      Compare ManaValue AtMost (Lit 3)])),
                Macros.gainsLife You (Lit 3)]

-- ===== What a clause can be conditioned on: the conditions chapter =====

-- "Destroy target artifact if its mana value is 2 or less." (Overload;
-- the kicker line and its "if this spell was kicked … instead" rider
-- elided — an additional cost and a replacement, two later axes) — the
-- flagship trailing conditional, and the one that fixes the argument
-- order: "its" is the artifact the MAIN clause targeted, so the
-- condition has to be typed after the clause it modifies. The
-- comparison is chapter sixteen's vocabulary unchanged — one
-- `Comparator`, one `StatOf` read, one written bound — in the
-- predicative frame instead of the postnominal one.
overload : Effect []
overload = If (Macros.destroy (Macros.target Macros.artifact))
              (CompareAmt (Macros.manaValueOf It) AtMost (Lit 2))
              Nothing

-- "Target attacking creature gets +3/+3 until end of turn. If it's an
-- artifact creature, it gains trample until end of turn." (Built to
-- Smash; the whole card) — the LEADING linearization of the same node:
-- the condition reads only what the first sentence introduced, so
-- nothing the second clause announces is inside it and the "If …, …"
-- order is available. Also the reference-matches row's positive, whose
-- predicate is two positive types stacked — the phrase "artifact
-- creature" a noun phrase would spell the same way.
builtToSmash : Effect []
builtToSmash =
  Sequentially [Macros.gets (Macros.target (And [Macros.creature, Attacking])) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn),
                If (Macros.gains It (KeywordAbility Trample) (Just Macros.untilEndOfTurn))
                   (Macros.itsA (And [Macros.artifact, Macros.creature]))
                   Nothing]

-- "Flames of the Raze-Boar deals 4 damage to target creature an
-- opponent controls. Then Flames of the Raze-Boar deals 2 damage to
-- each other creature that player controls if you control a creature
-- with power 4 or greater." (the whole card; the ability word
-- "Ferocious —" is a name for the condition, not a second clause) —
-- the existence row's positive, and it earns its place twice over: the
-- conditioned clause is the SECOND of a sequence, so the condition is
-- typed in a discourse two mentions deep, and the phrase it quantifies
-- over is a bound one, which is the comparison chapter's qualifier
-- inside the conditions chapter's quantifier.
flamesOfTheRazeBoar : Effect []
flamesOfTheRazeBoar =
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                If (DealDamage This (Lit 2)
                               (Each (And [Macros.creature, Other, ControlledBy (That PlayerW)])))
                   (Exists (And [Macros.creature, ControlledBy You,
                                 Compare Power AtLeast (Lit 4)]))
                   Nothing]

-- "You may sacrifice a creature. If you do, each opponent discards a
-- card." (Braids's Frightful Return, chapter I; the Saga's read-ahead
-- reminder and its other two chapters are separate abilities) — the
-- if-you-do branch. Two sentences on the page, ONE clause here: the
-- anaphor conditions on whether the offer was taken, which is not a
-- fact about the board, so it rides the may rather than the condition
-- type.
braidsFrightfulReturn : Effect []
braidsFrightfulReturn =
  Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.discardsACard (Each Opponent))

-- "You may sacrifice an artifact. If you do, destroy target artifact or
-- creature." (Daretti, Ingenious Iconoclast's [−1]; the loyalty cost is
-- the ability layer's) — the taken branch reading FORWARD: the arm
-- announces its own target, which the branchless may could not have
-- carried on the same sentence.
darettisMinusOne : Effect []
darettisMinusOne =
  Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Macros.destroy (Macros.target (Or [Macros.artifact, Macros.creature])))

-- "You may sacrifice an artifact. If you don't, tap this creature and
-- it deals 2 damage to you." (Yawgmoth Demon; the upkeep trigger
-- elided) — the DECLINED branch, and the asymmetry that makes the two
-- arms different types: this arm runs exactly when the sacrifice did
-- not, so the artifact it would have taken never existed and the arm
-- cannot mention it (`badIfNotReadsMayBody`). The card's "it deals" is
-- written here as the source itself: the pronoun and `This` name the
-- same object, but the source enters no discourse (ledger), so the
-- read has nothing to resolve against and the meaning is carried by
-- the phrase the pronoun abbreviates.
-- "You may sacrifice a creature. If you do, put a +1/+1 counter on
-- Crovax. If you don't, remove a +1/+1 counter from Crovax." (Crovax the
-- Cursed's upkeep trigger, whole but for the trigger header; its
-- enters-with line and its flying activation are other abilities) —
-- BOTH arms on one card, which is the shape chapter twenty-one needed to
-- settle what a two-armed may exports. The two arms are typed
-- differently and the export is neither: the body's discourse is all
-- that survives, because [CR#118.12] has the branch record whether the
-- offer was taken and nothing after the may can know which arm ran
-- (`badBothArmsAntecedent`). The card itself reads neither arm
-- afterward, and names its subject rather than pronouncing it.
crovaxTheCursed : Effect []
crovaxTheCursed =
  Macros.mayThenElse You (Macros.sacrifice You (Macros.a Macros.creature))
                  (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
                  (RemoveCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

yawgmothDemon : Effect []
yawgmothDemon =
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Sequentially [Tap Macros.thisCreature, DealDamage This (Lit 2) You])

-- ===== Objects made and counters moved: the creation chapter =====

-- "Create two 1/1 white Soldier creature tokens." (Raise the Alarm; its
-- Flash line elided, a keyword rider) — the flagship, and the whole card.
-- Every slot the bundle has in one phrase at its simplest: a written
-- count, a power and toughness, one color, one subtype, one card type.
-- The count is the ordinary amount vocabulary and the plural noun reads
-- off it (`amtPlur`).
raiseTheAlarm : Effect []
raiseTheAlarm = Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier])

-- "When this enchantment enters, create a 0/0 green and blue Fractal
-- creature token. Put three +1/+1 counters on it." (Additive Evolution's
-- enters trigger; the trigger header elided as Case of the Gateway
-- Express's is, and the card's second ability with it) — the token as a
-- DISCOURSE MENTION, which is what makes creation a binding construction
-- and not just a verb: the created token is the "it" the counter clause
-- reads, on the battlefield ([CR#111.1]) under the head its type line
-- writes. Also the two-color bundle ([CR#105.2b] -- a multicolored
-- object is two or more of the five colors) and the 0/0 body a counter
-- clause exists to fix.
additiveEvolution : Effect []
additiveEvolution = Sequentially [Macros.create (Lit 1) (Macros.creatureTok 0 0 [Green, Blue] [Fractal]),
                                  PutCounters (Lit 3) Macros.plusOnePlusOne It]

-- "When this creature enters, create a 1/1 colorless Thopter artifact
-- creature token with flying." (Aviation Pioneer; the trigger header
-- elided) — the COLORLESS bundle, where the empty color list is the word
-- ([CR#105.2c]); the COMPOUND type line, whose head noun is the last word
-- ("artifact creature"); and the with-clause over chapter seventeen's
-- keyword vocabulary unchanged.
aviationPioneer : Effect []
aviationPioneer =
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [] (MkTypeLine [Thopter] [Artifact, Creature])
                          [Flying] Nothing)

-- "Whenever you attack, create a 2/1 colorless Construct artifact
-- creature token with flying named Ballistic Boulder that's tapped and
-- attacking." (Fire Navy Trebuchet; the trigger header and the card's
-- delayed sacrifice sentence elided — "that token" is a token carrier
-- word `NounWord` does not have) — the whole adjective run in one
-- phrase, and the line that fixes the ORDER of the two trailing slots:
-- the with-clause precedes the name, and the arrival relative clause
-- follows both.
fireNavyTrebuchet : Effect []
fireNavyTrebuchet =
  Macros.createTappedAttacking (Lit 1)
    (MkToken (Just (Lit 2, Lit 1)) [] (MkTypeLine [Construct] [Artifact, Creature])
             [Flying] (Just "Ballistic Boulder"))

-- "If you don't control an Army creature, create a 0/0 black Zombie Army
-- creature token. Choose an Army creature you control. Put two +1/+1
-- counters on that creature. If it isn't a Zombie, it becomes a Zombie
-- in addition to its other types." — AMASS ZOMBIES 2, whole and as ONE
-- term, in the rule's own words rather than a card's reminder text:
-- [CR#701.47a] defines "amass [subtype] N" as those four sentences, and
-- Angrath, Captain of Chaos writes "Amass Zombies 2" for them.
-- This was the gap chapters nineteen and twenty could only write in two
-- halves, and chapter twenty-one's branch-arm rule closes it: the token
-- the first sentence conditionally creates is arm-local, so by the time
-- "that creature" is written the Army the second sentence CHOSE is the
-- only creature mention in scope and the sorted demonstrative resolves
-- without guessing.
-- The rule agrees twice over. [CR#701.47c] says the phrases "the Army
-- you amassed" and "the amassed Army" refer to "the creature you
-- chose" — the choice is the binder, not the creation. And a channel
-- merging the two mentions into one referent would have been FALSE, not
-- merely unspellable: Doubling Season reads "If an effect would create
-- one or more tokens under your control, it creates twice that many of
-- those tokens instead", which [CR#614.1a] makes a replacement effect,
-- so amass can create TWO Armies and then choose either one. Core's own
-- macro has the same shape — conditional create, then a choose binder,
-- then the counters on what was chosen.
amassZombiesTwo : Effect []
amassZombiesTwo =
  Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 0 0 [Black] [Zombie, Army]))
                   (Macros.notSo (Exists (And [HasSubtype Army, Macros.creature, ControlledBy You])))
                   Nothing,
                Choose (Macros.a (And [HasSubtype Army, Macros.creature, ControlledBy You])),
                PutCounters (Lit 2) Macros.plusOnePlusOne (That (TypeW Creature)),
                If (Macros.becomes It (Macros.subtypesOnly [Zombie]) Nothing)
                   (Macros.itIsntA (HasSubtype Zombie))
                   Nothing]

-- "Put a +1/+1 counter on target creature." (Battlegrowth) — the
-- counter flagship, and the whole card. Agent-silent ([CR#122.1]; core's
-- `PutCounters` carries no actor slot either) and battlefield-bound like
-- every other verb that touches a permanent.
battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target Macros.creature)

-- "{3}, {T}: Remove a -1/-1 counter from target creature."
-- (Chainbreaker; its mana and {T} cost elided as Icy Manipulator's are,
-- and its enters-with line with them — a replacement) — the removal
-- twin, and the other stat counter, so the two first-class kinds are
-- spelled end to end by real cards.
chainbreaker : Effect []
chainbreaker = RemoveCounters (Lit 1) Macros.minusOneMinusOne (Macros.target Macros.creature)

-- "[−2]: Tap target creature. Put two stun counters on it." (Kaito, Bane
-- of Nightmares; the loyalty cost is the ability layer's, as Daretti's
-- is, and the reminder gloss of [CR#122.1d] with it) — the NAMED counter
-- kind's positive: a kind earns its row where a corpus line writes it as
-- a one-shot put, which stun is and charge, age, and quest are not.
kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [Tap (Macros.target Macros.creature),
                                      PutCounters (Lit 2) Stun It]

-- "{2}, Exile a nonland card from your hand: Put four time counters on
-- the exiled card." (Jhoira of the Ghitu; the second sentence "If it
-- doesn't have suspend, it gains suspend" elided — the suspend keyword,
-- whose reminder-text machinery this round builds components for and
-- does not spell) — the COUNTER IN EXILE, and it needed no new clause:
-- the cost's exile is public ([CR#400.7j]), so the participle read
-- reaches it after the colon exactly as a sacrifice's does, and the only
-- thing that had to change is the verb's zone table.
jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
            (PutCounters (Lit 4) Time (TheVerbed Exile CardW))

-- "…Then remove a time counter from each other card you own in exile."
-- (Alaundo the Seer's last sentence; "other" and the ownership relation
-- elided, both ledgered, and the three sentences before it want a
-- mana-value amount and a granted triggered ability) — the REMOVAL twin
-- in the second zone, and the phrase places its own referent there
-- rather than inheriting a mention's fold-state.
alaundoTheSeer : Effect []
alaundoTheSeer = RemoveCounters (Lit 1) Time (Each (InZone Macros.exileZ))

-- "Arc Blade deals 2 damage to any target. Exile Arc Blade with three
-- time counters on it." (Arc Blade, both sentences; its suspend line is
-- the keyword this round builds components for and does not spell) — the
-- EXILE-WITH-COUNTERS rider, the delay family's own frame, and the card
-- exiles ITSELF, which is why the second sentence names no new phrase.
arcBlade : Effect []
arcBlade = Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
                         Macros.exileWithCounters This (Lit 3) Time]

-- "Exile target creature you control, then return that card to the
-- battlefield under its owner's control with a +1/+1 counter on it."
-- (Daydream, whole; its flashback line is a keyword `Keyword` has no row
-- for) — the SAME rider on the other destination, worn beside a
-- controller override, which is what settles that the rider belongs to
-- the placement rather than to the exile verb. Cloudshift with a counter
-- on the return trip, and the demonstrative flips carrier for chapter
-- one's reason ([CR#110.1]).
daydream : Card
daydream =
  Macros.card "Daydream" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
                             Macros.returnToBattlefieldWithCounters
                               (That CardW) (OwnerOf (That CardW))
                               (Lit 1) Macros.plusOnePlusOne])]
       Nothing

-- "{T}, Sacrifice this artifact: Put a +1/+1 counter on target
-- nonartifact creature. That creature becomes an artifact in addition to
-- its other types." (Ashnod's Transmogrant; the {T} component elided) —
-- the type addition's flagship, durationless ([CR#205.1b] keeps every
-- prior type; the unwritten span is [CR#611.2a]'s end-of-game default,
-- which is what "in addition" clauses mostly write). The negated type
-- word in the subject is chapter thirteen's vocabulary unchanged, and the
-- demonstrative reaches past the sacrificed self because that one sits in
-- a graveyard.
ashnodsTransmogrant : Ability
ashnodsTransmogrant =
  Activated (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
            (Sequentially [PutCounters (Lit 1) Macros.plusOnePlusOne
                                         (Macros.target (And [Macros.creature, Not Macros.artifact])),
                             Macros.becomes (That (TypeW Creature)) (Macros.typesOnly [Artifact]) Nothing])

-- "{U}: Target creature becomes an artifact in addition to its other
-- types until end of turn." (Neurok Transmuter, first ability; its mana
-- cost and second ability elided) — the same clause with the adverbial
-- the construction does write, which is the cell that split `BothGrants`:
-- eighteen corpus lines end a bare type addition at end of turn and not
-- one ends one at end of combat.
neurokTransmuter : Effect []
neurokTransmuter = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfTurn)

-- "Target creature can't block this turn and becomes a Coward in
-- addition to its other types until end of turn." (Coward // Killer's
-- Coward half; its "Time travel." line elided, a keyword rider, and the
-- coordination transcribed as a two-clause sequence — the Arc Trail
-- stand-in) — the card that settles the current-turn split from inside
-- ONE sentence: the restriction takes "this turn" and the type addition
-- takes "until end of turn", the two adverbials chapter seventeen
-- measured apart, written here by one writer in one breath.
cowardKiller : Effect []
cowardKiller = Sequentially [Macros.cantBlock (Macros.target Macros.creature) (Just Macros.thisTurn),
                             Macros.becomes (That (TypeW Creature)) (Macros.subtypesOnly [Coward])
                                     (Just Macros.untilEndOfTurn)]

-- "Target attacking creature that isn't a Demon" — the negated subtype
-- word, widened from Clavileño, First of the Blessed's "target attacking
-- Vampire that isn't a Demon" (a named stand-in: Vampire has no row, and
-- the point is the NEGATION, not the head). This is why the subtype word
-- projects no card type: `negTypesOf` reads a negated member's projected
-- head, so a Creature projection here would read "that isn't a Demon" as
-- "noncreature" and refuse a phrase oracle writes.
clavilenoPhrase : Predicate [] Object
clavilenoPhrase = And [Macros.creature, Attacking, Not (HasSubtype Demon)]

-- "Draw a card. If you control a Demon, each opponent loses 2 life and
-- you gain 2 life. Otherwise, you lose 2 life." (Unholy Annex's end-step
-- trigger, whole but for the trigger header) — chapter eighteen's ELSE
-- arm, opened with the card that finally writes both arms in vocabulary
-- this grammar has. The consequent is a two-clause sequence in a body
-- slot, which is where a sequence may still stand; the arm is a `Maybe`
-- field and reads only what preceded the conditional. The leading draw is
-- the RETRO-OPEN: chapter eighteen elided it for want of the verb, and
-- the elision is what made this a bare conditional rather than the
-- sequence the card writes.
unholyAnnex : Effect []
unholyAnnex =
  Sequentially [Macros.drawACard,
                If (Sequentially [Macros.losesLife (Each Opponent) (Lit 2),
                                  Macros.gainsLife You (Lit 2)])
                   (Exists (And [HasSubtype Demon, ControlledBy You]))
                   (Just (Macros.losesLife You (Lit 2)))]

-- ===== Cards drawn, modes chosen, and what a choice leaves behind: the composite-glue chapter =====

-- "Draw two cards." (Divination; the whole card) — the draw verb's
-- flagship, and the plainest shape it has: the imperative's unpronounced
-- subject spelled as `You`, and a written numeral in the count slot.
divination : Effect []
divination = Macros.drawCards 2

-- "Target player draws three cards." (Ancestral Recall; the whole card) —
-- the SUBJECTED form, and why the drawer is a slot rather than the
-- imperative's silent `You`: the same clause writes both.
ancestralRecall : Effect []
ancestralRecall = Draw (Macros.target AnyPlayer) (Lit 3)

-- "Each player draws X cards." (Prosperity; the whole card, its {X}
-- announced with the mana cost as every X is [CR#107.3a]) — the
-- distributive subject and the announced amount, neither of them
-- vocabulary this row had to mint.
prosperity : Effect []
prosperity = Draw (Each AnyPlayer) XVal

-- "Draw a card for each creature you control." (Collective Unconscious;
-- the whole card) — the for-each amount in the count slot, which is the
-- whole point of reusing `Amount` instead of minting a number path: the
-- adverbial that scales damage scales a draw unchanged.
collectiveUnconscious : Effect []
collectiveUnconscious = Draw You (Macros.forEach Macros.creatureYouControl)

-- "Target creature gets +1/+1 until end of turn. Draw a card." (Killian's
-- Confidence; its graveyard-return trigger is a second ability) — the
-- commonest sentence in the corpus in the position it is almost always
-- written in, and the proof that it introduces nothing: the sequence ends
-- there because there is nothing left to say about the card drawn.
killiansConfidence : Effect []
killiansConfidence = Sequentially [Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn),
                                   Macros.drawACard]

-- "Blindblast deals 1 damage to target creature. That creature can't
-- block this turn. Draw a card." (Blindblast; the whole card at last) —
-- the RETRO-OPEN of chapter fifteen's elision, which was elided for want
-- of the draw verb and nothing else.
blindblastWhole : Effect []
blindblastWhole = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                                Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn),
                                Macros.drawACard]

-- "{2}, Discard this card: Draw a card." — cycling's expansion
-- ([CR#702.29a]) with its EFFECT written, which `cyclingCost` above could
-- not have: the mana half of the cost stays elided as every mana cost
-- does, and the discard is the same term that justifies `DiscardOk`'s
-- bare-`This` row.
cycling : Ability
cycling = Activated (Compound [Mana [Macros.generic 2], Do (Macros.discards You This)]) Macros.drawACard

-- "Choose one — • Abrade deals 3 damage to target creature. • Destroy
-- target artifact." (Abrade; the whole card) — the modal flagship, at the
-- headcount four hundred seventy cards write. The two modes are typed in
-- the SAME discourse and not in one another's: neither reads the other,
-- and each announces its own targets only if it is chosen ([CR#700.2c]).
abrade : Effect []
abrade = Macros.chooseOne [DealDamage This (Lit 3) (Macros.target Macros.creature),
                    Macros.destroy (Macros.target Macros.artifact)]

-- "Choose two — • Destroy all artifacts. • Destroy all enchantments. •
-- Destroy all creatures with mana value 3 or less. • Destroy all
-- creatures with mana value 4 or greater." (Austere Command; the whole
-- card) — a counted headcount over four modes, with the comparison
-- chapter's bound inside two of them. Two of four, never two of two:
-- a headcount that fixes the whole list instructs no choice at all
-- (`ModesFit`, `badModalFixedWhole`).
austereCommand : Effect []
austereCommand =
  Macros.chooseTwo [Macros.destroy (AllOf Macros.artifact),
             Macros.destroy (AllOf Macros.enchantment),
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue AtMost (Lit 3)])),
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue AtLeast (Lit 4)]))]

-- "Choose one or both — • Target creature gets -1/-1 until end of turn. •
-- Put a +1/+1 counter on target creature." (Azula Always Lies; the whole
-- card) — the one-to-two range under the word that names the whole of a
-- two-item list, and the sharpest evidence for the list-not-telescope
-- shape: BOTH modes write "target creature" and neither is the other's
-- "other", because each announces its own ([CR#700.2c,601.2c]). Written
-- as a sequence the second phrase would have to say "another".
azulaAlwaysLies : Effect []
azulaAlwaysLies =
  Macros.chooseOneOrBoth [Macros.gets (Macros.target Macros.creature) (PtDown (Lit 1)) (PtDown (Lit 1)) (Just Macros.untilEndOfTurn),
                   PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target Macros.creature)]

-- "Choose one or more — • Destroy target artifact. • Destroy target
-- enchantment. • Destroy target land." (Rain of Thorns; the whole card) —
-- the OPEN top, whose word says the mode list is its own bound. Nineteen
-- cards write it and none over two modes: at two the word is "both".
rainOfThorns : Effect []
rainOfThorns = Macros.chooseOneOrMore [Macros.destroy (Macros.target Macros.artifact),
                                Macros.destroy (Macros.target Macros.enchantment),
                                Macros.destroy (Macros.target Macros.land)]

-- "Whenever Rankle deals combat damage to a player, choose any number —
-- • Each player discards a card. • Each player sacrifices a creature of
-- their choice." (Rankle, Master of Pranks; the trigger header elided as
-- every trigger header is, and the MIDDLE mode with it — "Each player
-- loses 1 life and draws a card" is one subject under two verbs, a
-- verb-phrase coordination this grammar does not spell) — the unbounded
-- head, whose floor is genuinely zero: [CR#107.1c] lets a player told to
-- choose "any number" choose "any positive number or zero", so declining
-- every mode is a legal reading of the instruction and the head says so.
rankleMasterOfPranks : Effect []
rankleMasterOfPranks =
  Macros.chooseAnyNumber [Macros.discardsACard (Each AnyPlayer),
                   Macros.sacrifice (Each AnyPlayer) (Macros.aTheirChoice Macros.creature)]

-- "Choose an opponent. That player sacrifices a creature of their
-- choice." (Myrkul's Edict, the 1—9 face of its d20 roll; the roll is a
-- carrier this grammar does not spell, elided as a trigger header is) —
-- the choose clause as an INTRODUCER, and one mention serving two reads:
-- the next sentence's demonstrative subject, and the unique player
-- antecedent "of their choice" needs (`countChoosers`).
myrkulsEdict : Effect []
myrkulsEdict = Sequentially [Choose (Macros.a Opponent),
                             Macros.sacrifice (That PlayerW) (Macros.aTheirChoice Macros.creature)]


-- ===== The group complement, the control verb, and the batch =====

-- "Choose target creature you control. It deals damage equal to its
-- power to each other creature." (Nibelheim Aflame; the flashback line
-- and the graveyard-cast rider elided) — the anchored complement over a
-- BOUND mention: the domain is every creature, and the anchor is the one
-- the first sentence announced, read back exactly as "it" reads it.
nibelheimAflame : Effect []
nibelheimAflame =
  Sequentially [Choose (Macros.target Macros.creatureYouControl),
                DealDamage It (Macros.powerOf It) (Each (Macros.otherCreature It))]

-- "{5}{W}, {T}: Other creatures you control get +1/+1 until end of
-- turn." (War Screecher; the activation cost elided as every cost line
-- is) — the PLURAL complement, and the corpus's commonest anchor: the
-- SOURCE. `This` introduces no binding and needs none here, the anchor
-- being carried by the phrase rather than searched for in the
-- discourse, which is what the ledgered self-exclusion entry was
-- waiting on.
warScreecher : Effect []
warScreecher =
  Macros.gets (AllOf (Macros.otherCreatureYouControl Macros.thisCreature)) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)

-- "Enrage — Whenever this creature is dealt damage, put a +1/+1 counter
-- on each other creature you control." (Bellowing Aegisaur; the trigger
-- header elided as every trigger header is) — the DISTRIBUTIVE
-- complement, source-anchored.
bellowingAegisaur : Effect []
bellowingAegisaur =
  PutCounters (Lit 1) Macros.plusOnePlusOne (Each (Macros.otherCreatureYouControl Macros.thisCreature))

-- "{B}, Remove a -1/-1 counter from this creature: Put a -1/-1 counter
-- on each other creature." (Carnifex Demon; the cost line elided) — the
-- unrestricted complement, and the sweep that would hit the source
-- without it.
carnifexDemon : Effect []
carnifexDemon =
  PutCounters (Lit 1) Macros.minusOneMinusOne (Each (Macros.otherCreature Macros.thisCreature))

-- "Each other player discards a card." (Syphon Mind's first sentence;
-- the second, "You draw a card for each card discarded this way", waits
-- on the participant-subset read) — the complement at the PLAYER kind,
-- anchored to `You`: the kind index makes the anchor a player without a
-- gate, and [CR#102.1] makes "player" the word the exclusion applies to.
syphonMind : Effect []
syphonMind = Macros.discardsACard (Each Macros.otherPlayer)

-- "{2}{R}, {T}: This creature fights another target creature." (Brash
-- Taunter; the cost line elided) — the SINGULAR complement, spelled
-- "another": one word, one row, and the determiner deciding whether it
-- selects one or subtracts from a group. The self-exclusion the fight
-- family writes twenty-four times ("Polukranos fights another target
-- creature" is the same shape under a card name) and that the bare
-- `Other` could not reach, its anchor search reading bindings where the
-- source leaves none.
brashTaunter : Effect []
brashTaunter = Fights Macros.thisCreature (Macros.target (Macros.otherCreature Macros.thisCreature))

-- "{1}{G}, {T}: Target creature you control fights another target
-- creature." (Ulvenwald Tracker; the cost line elided) — the CONTRAST
-- positive, and the reason the two rows stay apart: this "another" is
-- the bare `Other`, whose anchor is the target announced before it
-- (the word [CR#115.4] names, and [CR#601.2c] is why it is needed at
-- all: two instances of "target" may otherwise be given the same
-- object), where Brash Taunter's is a referent the clause names. Same
-- word, same slot, different relation.
ulvenwaldTracker : Effect []
ulvenwaldTracker = Fights (Macros.target Macros.creatureYouControl) (Macros.target (And [Macros.creature, Other]))

-- "Gain control of target creature until end of turn." (Act of
-- Treason's first sentence; "Untap that creature. It gains haste until
-- end of turn." elided — the untap verb is the parked object-status
-- axis) — the control verb under the ordinary current-turn grant
-- adverbial, a hundred and eleven corpus lines.
actOfTreason : Effect []
actOfTreason = Macros.gainControl (Macros.target Macros.creature) (Just Macros.untilEndOfTurn)

-- "Dominate Monster — When this creature enters, gain control of target
-- creature for as long as you control this creature." (Mind Flayer; the
-- trigger header elided) — the FOR-AS-LONG-AS duration, opened on the
-- clause chapter eighteen ledgered it waiting for. [CR#611.2b]'s own
-- example is this shape (Master Thief's "gain control of target artifact
-- for as long as you control this creature"), and the condition is the
-- reference frame chapter eighteen already built.
mindFlayer : Effect []
mindFlayer =
  Macros.gainControl (Macros.target Macros.creature) (Just (ForAsLongAs (Matches Macros.thisCreature (ControlledBy You))))

-- "{2}{U}{U}: Exchange control of this creature and target creature.
-- (This effect lasts indefinitely.)" (Phyrexian Infiltrator; the cost
-- line elided, the reminder text being [CR#611.2a]'s end-of-game default
-- printed out loud) — the SIMULTANEOUS batch, written out because
-- English writes the exchange as one verb and this file has no primitive
-- for it. Both halves read ONE pre-state ([CR#701.12b]: each player
-- gains control of the permanent that "was controlled by the other
-- player"), and the second half reads the first half's ANNOUNCEMENT, not
-- its effect — which is what the batch's telescope threads.
phyrexianInfiltrator : Effect []
phyrexianInfiltrator =
  Simultaneously
    [Macros.gainsControl (ControllerOf (Macros.target Macros.creature)) Macros.thisCreature Nothing,
     Macros.gainsControl (ControllerOf Macros.thisCreature) (That (TypeW Creature)) Nothing]

-- "At the beginning of your end step, each player creates a 1/1 green
-- Plant creature token." (Grismold, the Dreadsower; the trigger header
-- and the card's other lines elided) — the DISTRIBUTED creation, one
-- token per player from a count of one. Eighteen lines write "each
-- player creates", eight "each opponent creates". The count says
-- singular and the clause exports a PLURAL mention, because the agent
-- distributes (`outputPlur`); the reading it enables is Elephant
-- Resurgence's "Those creatures", whose own sentence waits elsewhere
-- (ledger).
grismold : Effect []
grismold = Create (Each AnyPlayer) (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]) []

-- "Sparkmage's Gambit deals 1 damage to each of up to two target
-- creatures. Those creatures can't block this turn." (Sparkmage's
-- Gambit, whole) — the EACH-OF recipient and the plural read it leaves
-- behind, in one card. The amount is per member ("1 damage", and each
-- of the two takes one), the mention is the group's and stays plural,
-- and the next sentence reads it with the sorted plural demonstrative.
sparkmagesGambit : Effect []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)),
                Macros.cantBlock (Those (TypeW Creature)) (Just Macros.thisTurn)]

-- "[+1]: Put a +1/+1 counter on each of up to two target creatures."
-- (Ajani, Adversary of Tyrants; the loyalty cost and the card's other
-- abilities elided) — the same recipient under the counter verb, which
-- is what settles that the distributed recipient is not damage's alone.
-- Twenty-nine corpus lines write "each of up to two target creatures".
ajaniAdversaryOfTyrants : Effect []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf (TargetGroup (Macros.upTo 2) Macros.creature))

-- "Fall of the Titans deals X damage to each of up to two targets."
-- (Fall of the Titans; its surge cost line elided) — the each-of
-- recipient over the damage CLASS word, which is the phrase [CR#115.4]
-- names and the corpus writes only under this determiner or under a
-- division.
fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This XVal (EachOf (TargetGroup (Macros.upTo 2) AnyTarget))

-- "Choose any number of target creatures. Put a +1/+1 counter on each of
-- them." (Nature's Panoply; its strive cost line elided — an
-- additional-cost-per-target rider the cost algebra will spell) — the
-- each-of determiner over a plural READ rather than over a fresh
-- mention. Thirty-seven corpus lines write "each of them".
naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf Them)]

-- "Arc Lightning deals 3 damage divided as you choose among one, two, or
-- three targets." (Arc Lightning, whole) — the DIVISION: one written
-- amount split over a group, the split announced as the spell is cast
-- ([CR#601.2d]) and held fixed against any later change of targets
-- ([CR#115.7f]). The enumerated range is how the phrase spells the
-- caster's choice of how many.
arcLightning : Effect []
arcLightning = Macros.dealsDivided This (Lit 3) (TargetGroup (Macros.oneThrough 3) AnyTarget)

-- "Forked Bolt deals 2 damage divided as you choose among one or two
-- targets." (Forked Bolt, whole) — the same structure at the narrower
-- range, which twelve corpus lines write.
forkedBolt : Effect []
forkedBolt = Macros.dealsDivided This (Lit 2) (TargetGroup (Macros.oneThrough 2) AnyTarget)

-- "Boulderfall deals 5 damage divided as you choose among any number of
-- targets." (Boulderfall, whole) — the UNBOUNDED division, and the
-- positive that retired the class word's ban at that quantity: eighteen
-- corpus lines write "among any number of targets", and the structure
-- they needed is this one.
boulderfall : Effect []
boulderfall = Macros.dealsDivided This (Lit 5) (TargetGroup Macros.anyNumber AnyTarget)

-- "When this creature enters, distribute two +1/+1 counters among one or
-- two target creatures you control." (Armament Corps; the trigger header
-- elided) — the division's OTHER verb. [CR#601.2d] and [CR#115.7f] name
-- "divide or distribute" as one mechanic over one pair of examples
-- ("damage or counters"), and this is the counter half: the same
-- structure, a different word, forty-six corpus lines.
armamentCorps : Effect []
armamentCorps =
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (TargetGroup (Macros.oneThrough 2) Macros.creatureYouControl)


-- ===== The ordered library, the exposure clause, and the partition =====

-- "Look at target player's hand." / "Draw a card." (Peek) — the peek,
-- and the hand is a ZONE rather than a group of cards: oracle never
-- writes "look at all cards in target player's hand". Two printed
-- lines, one term.
peek : Effect []
peek = Sequentially [Macros.lookAtHandOf (Macros.target AnyPlayer), Macros.drawACard]

-- "Look at the top four cards of your library. Put one of them into
-- your hand and the rest on the bottom of your library in any order."
-- (Impulse) — THE partition. The look assembles a group ([CR#701.20e]
-- exposes it without moving it, [CR#701.20b]), the partitive takes a
-- member out of that group, and "the rest" is the complement of what
-- was taken WITHIN it. The second sentence is one verb over two
-- complements, transcribed as the named two-clause stand-in the ledger
-- already parks Arc Trail's beside.
impulse : Effect []
impulse = Sequentially [ Macros.lookAt (Macros.topCards 4)
                       , Move (Macros.oneOf Them) Macros.handZ
                       , Move TheRest (Macros.onBottomIn AnyOrder)
                       ]

-- "Look at the top three cards of your library. Put one of them into
-- your hand and the rest on the bottom of your library in any order."
-- (Anticipate) — the same frame at a different count, which is what
-- makes it a frame.
anticipate : Effect []
anticipate = Sequentially [ Macros.lookAt (Macros.topCards 3)
                          , Move (Macros.oneOf Them) Macros.handZ
                          , Move TheRest (Macros.onBottomIn AnyOrder)
                          ]

-- "Reveal the top four cards of your library. … Put the rest into your
-- graveyard." — the REVEAL half of the same shape ([CR#701.20a]: shown
-- to every player, not just the looker), with the partitive spelled
-- over the demonstrative the corpus also writes ("one of those cards",
-- fifty-two lines). The middle sentence of the printed card uses the
-- among-restriction and is elided; the frame under test is the
-- assemble-then-partition one.
revealFourPartition : Effect []
revealFourPartition = Sequentially [ Macros.revealCards (Macros.topCards 4)
                                   , Move (Macros.oneOf (Those CardW)) Macros.handZ
                                   , Move TheRest Macros.graveyardZ
                                   ]

-- "{R}, {T}: Look at the top eight cards of your library. Exile four of
-- them …, then put the rest on top of your library in any order." — the
-- COUNTED partitive and the top placement, showing that neither the
-- part nor the remainder has to be one card. (The "at random" rider on
-- the exile is elided — `ChoiceMode`'s marking belongs to the
-- indefinite phrase, not to a partitive, and no partitive carries one
-- in this vocabulary yet.)
exileFourOfThem : Effect []
exileFourOfThem = Sequentially [ Macros.lookAt (Macros.topCards 8)
                               , Macros.exile (Macros.someOf 4 Them)
                               , Move TheRest (Macros.onTopIn AnyOrder)
                               ]

-- "{2}: Put the bottom card of your library into your graveyard." — the
-- bottom slice's ONE corpus witness (Grenzo, Dungeon Warden; the
-- conditional second sentence needs the power comparison and a
-- battlefield placement, elided). It is also the clean case for the
-- retag: a library card put into a public zone is readable afterwards
-- ([CR#400.7j]), which the graveyard destination makes true here.
grenzoBottomCard : Effect []
grenzoBottomCard = Move Macros.bottomCard Macros.graveyardZ

-- "Search your library for a land card, reveal it, put it into your
-- hand, then shuffle." (Sylvan Scrying) — the search frame whole: the
-- found object is the clause's own mention, the reveal is a SEPARATE
-- instruction because [CR#701.23e] says an unwritten reveal does not
-- happen, the placement moves it out of the library, and the shuffle is
-- the sequence tail. Note the order: the card leaves before the
-- library is randomized, which is [CR#701.24b]'s own arrangement.
sylvanScrying : Effect []
sylvanScrying = Sequentially [ Macros.searchLibraryFor Macros.land
                             , Macros.revealCards It
                             , Move It Macros.handZ
                             , Macros.shuffle
                             ]

-- "Search your library for a creature card, put that card onto the
-- battlefield, then shuffle." — the destination split: the same frame
-- with the found card placed on the battlefield and no reveal, which is
-- [CR#701.23e]'s default and a hundred seven corpus lines.
searchToBattlefield : Effect []
searchToBattlefield = Sequentially [ Macros.searchLibraryFor Macros.creature
                                   , Move It Macros.battlefieldZ
                                   , Macros.shuffle
                                   ]

-- "Target player mills ten cards." (Glimpse the Unthinkable) — the mill
-- clause whole, subject and all ([CR#701.17a]).
glimpseTheUnthinkable : Effect []
glimpseTheUnthinkable = Macros.millsCards (Macros.target AnyPlayer) 10

-- "Mill three cards." — the imperative mill, and the clause whose
-- milled group is a real mention: the graveyard is public, so
-- [CR#701.17c] lets later text find what was milled where a drawn card
-- (hand, hidden) could never be found. Here the read is the plural
-- demonstrative; the corpus's own reads are the among-restriction and
-- the "this way" participle, both ledgered.
millThenReadGroup : Effect []
millThenReadGroup = Sequentially [Macros.millCards 3, Macros.exile (Those CardW)]

-- "Look at the top card of your library. You may put that card into
-- your graveyard." — the singular slice with the offer, which is what
-- keeps the count vocabulary honest: at one the numeral is unwritten
-- and the mention is singular, so the read is "that card" and not
-- "them".
lookAtTopThenBin : Effect []
lookAtTopThenBin =
  Sequentially [Macros.lookAt Macros.topCard, Macros.may You (Move (That CardW) Macros.graveyardZ)]


-- ===== Replacement, prevention, and the event-ended rider =====

-- "Bot Bashing Time deals 6 damage to target creature. If that creature
-- would die this turn, exile it instead." — the would/instead clause
-- whole, and the frame twenty-four corpus lines write word for word.
-- The interception's subject is the FIRST sentence's announced target,
-- read back as a demonstrative; its replacement says "it" of the same
-- creature, which is what typing the replacement in the event's own
-- announcement buys ([CR#601.2c]). The duration is the shield's
-- ([CR#614.3] — a replacement effect lasts until used up or until its
-- duration expires), and "this turn" is the word this construction
-- writes.
botBashingTime : Effect []
botBashingTime =
  Sequentially [ DealDamage This (Lit 6) (Macros.target Macros.creature)
               , Macros.ifWouldInstead (Dies (That (TypeW Creature))) (Macros.exile It) (Just Macros.thisTurn)
               ]

-- "{1}: The next time you would draw a card this turn, this enchantment
-- deals 2 damage to any target instead." (Words of War; the activation
-- cost is the cost round's) — the OTHER multiplicity word, and the
-- event that forces it: a draw repeats, so an unlimited shield and a
-- single-use one are different effects and the writer has to say which
-- ([CR#614.11] gives draw replacement its own paragraph).
wordsOfWar : Effect []
wordsOfWar = Macros.nextTimeWouldInstead (Draws You)
                                  (DealDamage This (Lit 2) (Macros.target AnyTarget))
                                  (Just Macros.thisTurn)

-- "{1}: The next time you would draw a card this turn, you gain 5 life
-- instead." (Words of Worship) — the same shield with a replacement
-- that touches no object at all, which is what [CR#611.2c]'s
-- rules-modifying half looks like from the clause side.
wordsOfWorship : Effect []
wordsOfWorship = Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.gainsLife You (Lit 5))
                                      (Just Macros.thisTurn)

-- "Prevent all combat damage that would be dealt this turn." (Fog, Holy
-- Day, Darkness, Root Snare — five lines, one sentence) — the shield
-- with no recipient at all, [CR#611.2c]'s own example of an effect that
-- modifies the rules of the game rather than any object.
fog : Effect []
fog = Macros.preventAll CombatOnly Everywhere (Just Macros.thisTurn)

-- "Prevent all damage that would be dealt to target creature this
-- turn." (Indestructible Aura, Shielded Passage) — the same shield with
-- a recipient, and the recipient is the damage clause's own row rather
-- than a second one.
indestructibleAura : Effect []
indestructibleAura = Macros.preventAll AnyDamage (Macros.shieldingIt (Macros.target Macros.creature)) (Just Macros.thisTurn)

-- "Prevent the next 3 damage that would be dealt to any target this
-- turn." (Shieldmate's Blessing) — [CR#615.7]'s numbered shield, the
-- one that is used up a damage at a time, over the damage class word.
shieldmatesBlessing : Effect []
shieldmatesBlessing =
  Macros.preventNext AnyDamage (Lit 3) (Macros.shieldingIt (Macros.target AnyTarget)) (Just Macros.thisTurn)

-- "Exile target creature an opponent controls until this creature
-- leaves the battlefield." (Banisher Priest's triggered body; the
-- trigger wrapper is the abilities layer's) — the O-Ring family's
-- eighty-six lines, and NOT a duration: [CR#610.3] makes this a
-- one-shot zone change that pairs with a second one-shot effect
-- returning the object, which is why the rider sits on the clause and
-- the `Duration` row over the same event stays unclaimed.
banisherPriest : Effect []
banisherPriest = Macros.exileUntil (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent]))
                            (Leaves Macros.thisCreature)

-- "[0]: Draw a card. If you control three or more artifacts, draw two
-- cards instead." (Tezzeret, Artifice Master) — the SELF-replacement
-- ([CR#614.15]), the "instead" with no "would" in front of it. The
-- replacement is a conditional clause and the node pairs it with what
-- it replaces, which is what makes "instead" mean anything at all.
tezzeretDrawTwo : Effect []
tezzeretDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (Macros.drawCards 2)
                (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                            AtLeast (Lit 3))
                Nothing)

-- "{4}, {T}: Draw a card. If you control eight or more lands, draw two
-- cards instead." (Zimone, Quandrix Prodigy) — the same frame at a
-- different domain and count, which is what makes it a frame.
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (Macros.drawCards 2)
                (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                            AtLeast (Lit 8))
                Nothing)

-- ===== Chapter twenty-seven positives: costs, symbols, and the first
-- ability container =====

-- "{1}{U/B}, {Q}: Target creature gets -2/-0 until end of turn."
-- (Merrow Grimeblotter; its reminder gloss "({Q} is the untap symbol.)"
-- omitted as reminder text always is) — the HYBRID symbol and the untap
-- symbol in one cost, which is the pair the port had to carry and the
-- corpus writes together on exactly this card.
merrowGrimeblotter : Ability
merrowGrimeblotter =
  Activated (Compound [Mana [Macros.generic 1, Macros.hybridPip Blue Black], UntapSymbol])
            (Macros.gets (Macros.target Macros.creature) (PtDown (Lit 2)) (PtDown (Lit 0)) (Just Macros.untilEndOfTurn))

-- "{1}{S}: This creature gets +1/+0 until end of turn." (Phyrexian
-- Snowcrusher) — the snow symbol ([CR#107.4h]), which is neither a color
-- nor a type of mana and is why `ManaSymbol` needs a row for it rather
-- than a colorless pip with a note.
phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Activated (Mana [Macros.generic 1, SnowMana])
            (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

-- "{1}{C}: This creature gets +2/+1 until end of turn." (Havoc Sower;
-- its Devoid line and reminder gloss elided) — the COLORLESS pip
-- ([CR#107.4c]), which is what `ColorOrColorless` was ported for: the
-- token's empty color list could say "colorless" but no color could say
-- "{C}".
havocSower : Ability
havocSower =
  Activated (Mana [Macros.generic 1, Macros.colorlessPip])
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

-- "{1}{B}, Pay 2 life: Draw a card." (Erebos, God of the Dead; its other
-- three lines elided) — the life payment as a cost COMPONENT, which is
-- the same `ChangeLife` clause the sentence grammar already had, wearing
-- the cost frame's verb.
erebos : Ability
erebos = Activated (Compound [Mana [Macros.generic 1, Macros.pip Black], Macros.payLife You 2]) Macros.drawACard

-- "{1}{R/G}: Put a +1/+1 counter on this creature. Activate only as a
-- sorcery." (Savageborn Hydra; its double strike and enters-with lines
-- elided) — the activation WINDOW ([CR#602.5d]), and a second hybrid
-- cost under it.
savagebornHydra : Ability
savagebornHydra =
  Activated (Mana [Macros.generic 1, Macros.hybridPip Red Green])
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
            {window = Just AsSorcery}

-- "{1}{G}: This creature gets +2/+2 until end of turn. Activate only
-- once each turn." (Basking Rootwalla; its madness line elided) — the
-- use LIMIT ([CR#602.5b]), the second restriction slot.
baskingRootwalla : Ability
baskingRootwalla =
  Activated (Mana [Macros.generic 1, Macros.pip Green])
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
            {limit = Just OncePerTurn}

-- "{3}, {T}: Draw a card. Activate only if you control a creature with
-- power 4 or greater." (Bonders' Enclave; its mana ability elided) — the
-- activation GUARD, and the point of the seam chapter eighteen left: the
-- condition is `Condition` unchanged, the postnominal comparison is
-- chapter sixteen's unchanged, and the carrier is new.
bondersEnclave : Ability
bondersEnclave =
  Activated (Compound [Mana [Macros.generic 3], TapSymbol])
            Macros.drawACard
            {guard = Just (Exists (And [Macros.creature, ControlledBy You,
                                        Compare Power AtLeast (Lit 4)]))}

-- "{W}, {T}: Remove a -1/-1 counter from target creature. If you do, you
-- gain 2 life." (Woeleecher) — the MANDATORY "if you do" ([CR#118.12]),
-- which is `May` with its offer emptied: no "may" is written anywhere on
-- the line, and the arm still asks whether the payment was started.
woeleecher : Ability
woeleecher =
  Activated (Compound [Mana [Macros.pip White], TapSymbol])
            (Macros.doThen (RemoveCounters (Lit 1) Macros.minusOneMinusOne (Macros.target Macros.creature))
                    (Macros.gainsLife You (Lit 2)))

-- "Sacrifice this creature unless you pay {2}." (Molting Harpy; its
-- upkeep trigger shell and flying line elided) — the UNLESS family's
-- first positive, and it is [CR#118.12a]'s own rewrite rather than a
-- construction: "[Do something] unless [a player does something else]"
-- means "[A player may do something else]. If [that player doesn't], [do
-- something]", so the sentence is the may node read backwards, the
-- payment in the body and the main clause in the declined arm.
moltingHarpy : Effect []
moltingHarpy = Macros.mayElse You (Pay You (Mana [Macros.generic 2])) (Macros.sacrifice You Macros.thisCreature)

-- "Tap this creature unless you pay 1 life." (Carnophage; upkeep shell
-- elided) — the same frame with a LIFE payment, which is what makes the
-- verb's complement a whole `Cost` and not a mana amount.
carnophage : Effect []
carnophage = Macros.mayElse You (Pay You (Macros.payLife You 1)) (Tap Macros.thisCreature)

-- "Sacrifice this enchantment unless you discard a card." (Solitary
-- Confinement; upkeep shell and its other three lines elided) — the
-- ACTION half of the same family, where the offered payment is an
-- ordinary clause and no "pay" is written at all. Seventy-one corpus
-- lines pay an unless with a non-mana action.
solitaryConfinement : Effect []
solitaryConfinement = Macros.mayElse You (Macros.discardsACard You) (Macros.sacrifice You Macros.thisEnchantment)


-- "{W}{W}: Create a 1/1 white Soldier creature token. Activate only if
-- you control no creatures and only once each turn." (Security Detail —
-- the WHOLE card, one line long) — two restrictions conjoined, which is
-- why the slots are separate rather than one restriction row, and the
-- guard is a NEGATED condition: chapter eighteen measured "if you
-- control no …" and could not place it, every carrier of those lines
-- being a trigger's intervening-"if" or an activation restriction. This
-- is that carrier.
securityDetail : Ability
securityDetail =
  Activated (Mana [Macros.pip White, Macros.pip White])
            (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
            {limit = Just OncePerTurn}
            {guard = Just (Macros.notSo (Exists (And [Macros.creature, ControlledBy You])))}



-- ===== The triggered line and the static line: the ability chapter =====

-- "When this creature enters, draw a card." (Cloudkin Seer; its flying
-- line is a second ability, not part of this one) — the trigger corpus's
-- single commonest shape, two thousand four hundred seventy-three lines
-- of "When this [permanent] enters".
cloudkinSeer : Ability
cloudkinSeer = Triggered When (Enters Macros.thisCreature) Macros.drawACard

-- "Whenever a creature dies, you gain 1 life." (Moonlit Wake, whole) —
-- the same event under a DESCRIPTION, which is what moves the header
-- word: one object enters once and dies once, so a fixed subject takes
-- "When", and a description ranging over many takes "Whenever".
moonlitWake : Ability
moonlitWake = Triggered Whenever (Dies (Macros.a Macros.creature)) (Macros.gainsLife You (Lit 1))

-- "Whenever a creature you control dies, exile it." (Promise of
-- Tomorrow's first line) — the trigger's participant read BACK, and the
-- read is what shows the after-discourse is right: "it" finds a card in
-- a graveyard ([CR#603.6,700.4]), which is exactly what exile wants and
-- exactly what the interception's replacement must not have.
promiseOfTomorrow : Ability
promiseOfTomorrow = Triggered Whenever (Dies (Macros.a Macros.creatureYouControl)) (Macros.exile It)

-- "At the beginning of your upkeep, draw a card." (Staff of Nin's first
-- line) — the turn-part event, the one row whose header word is fixed
-- ([CR#603.2b]) and the only one with no noun phrase in it.
staffOfNin : Ability
staffOfNin = Triggered At (BeginningOf Upkeep (Just Yours)) Macros.drawACard

-- "Whenever this creature attacks, draw a card." (Library Larcenist,
-- whole) — the fixed subject taking "Whenever", which is the header
-- word tracking the EVENT's repeatability rather than the subject's
-- fixity: a creature attacks many times.
libraryLarcenist : Ability
libraryLarcenist = Triggered Whenever (Attacks Macros.thisCreature) Macros.drawACard

-- "Whenever this creature blocks, it deals 1 damage to target attacking
-- creature." (Elite Javelineer, whole) — the block event, and a trigger
-- that TARGETS ([CR#115.1d]), which is the line the static row cannot
-- write.
eliteJavelineer : Ability
eliteJavelineer =
  Triggered Whenever (Blocks Macros.thisCreature Nothing)
            (DealDamage This (Lit 1) (Macros.target (And [Macros.creature, Attacking])))

-- "Whenever this creature deals combat damage to a player, draw a
-- card." (Jhessian Thief's second line; its prowess line is a keyword)
-- — the two-slot event, and its recipient reads the damage clause's own
-- table for the third time.
jhessianThief : Ability
jhessianThief =
  Triggered Whenever (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer)) Macros.drawACard

-- "When this creature enters, if you control an artifact, draw a card."
-- (Scholar of Stars, whole) — the INTERVENING "if" ([CR#603.4]), which
-- is `Condition` verbatim in a fourth carrier. The rule is what makes it
-- a slot rather than an ordinary trailing conditional: it applies "only
-- to an 'if' that immediately follows a trigger condition", and it is
-- checked twice, once as the event occurs and again on resolution.
scholarOfStars : Ability
scholarOfStars =
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Exists (And [Macros.artifact, ControlledBy You]))}

-- "Creatures you control can't attack." (Glacial Chasm's third line) —
-- the durationless deontic `badStaticCant` has refused since chapter
-- seventeen, in the container it was waiting for.
glacialChasmCant : Ability
glacialChasmCant = Static (Deontic (AllOf Macros.creatureYouControl) Forbid Attack Agent Nothing)

-- "Prevent all damage that would be dealt to you." (Glacial Chasm's
-- fourth line) — `badStandingPrevention`'s own sentence, one line below
-- the last on the same card.
glacialChasmShield : Ability
glacialChasmShield = Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You))

-- "If a creature an opponent controls would die, exile it instead."
-- (Misery's Shadow's first line) — `badStandingIntercept` as a
-- positive, and the standing interception chapter twenty-five refused
-- with the container named.
miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (Macros.a (And [Macros.creature, ControlledBy (Macros.a Opponent)])))
                     (Macros.exile It) Repeatedly)

-- "If you would draw a card, draw two cards instead." (Thought
-- Reflection, the whole card) — the standing DRAW replacement, and the
-- line that moved the multiplicity table off the wrong axis. The cell
-- refusing it was measured on "if you would draw a card THIS TURN"
-- (zero, correctly): a duration-bounded form, which says nothing about
-- the durationless line twenty-one cards print. What picks the opening
-- word is the CARRIER — a static line writes "if", a one-shot spun up by
-- a cost or a trigger writes "the next time" — and that dimension is
-- ledgered.
thoughtReflection : Ability
thoughtReflection =
  Static (Intercepts (Draws You) (Draw You (Lit 2)) Repeatedly)

-- "Metalcraft — Creatures you control get +3/+0 as long as you control
-- three or more artifacts." (Jor Kadeen, the Prevailer's second line;
-- the ability word has no rules meaning [CR#207.2c]) — the CONDITIONAL
-- static, [CR#611.3a]'s live re-check, and the nine-hundred-twelve-line
-- family chapter twenty-two measured and could not write.
jorKadeen : Ability
jorKadeen =
  Static (Macros.asLongAs (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                               AtLeast (Lit 3))
                   (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 3)) (PtUp (Lit 0))))

-- "This land enters tapped." (Abandoned Outpost's first line) — the
-- entry rider [CR#603.6d] calls a static ability in as many words, owed
-- from three sites since chapter twenty-five and paid here.
abandonedOutpost : Ability
abandonedOutpost = Static (Macros.entersTapped (AsType Land This))

-- "Destroy target tapped creature." (Aerial Assault; per the round's
-- recon) — the status word inside an ordinary target phrase, seeding the
-- battlefield the way the combat designations do but presupposing no
-- type of its own.
aerialAssault : Effect []
aerialAssault = Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

-- "Destroy target untapped creature." (Asphyxiate) — the paired value,
-- its own word and not a negation of the first.
asphyxiate : Effect []
asphyxiate = Macros.destroy (Macros.target (And [Macros.creature, Macros.untapped]))

-- "{T}: Untap target artifact or creature." (Aphetto Alchemist) — the
-- untap effect under the tap-symbol cost: the two surfaces of the same
-- tapped axis, kept apart as cost and effect.
aphettoAlchemist : Ability
aphettoAlchemist = Activated TapSymbol
                             (Untap (Macros.target (Or [Macros.artifact, Macros.creature])))

-- "This artifact doesn't untap during your untap step." (Time Vault,
-- second line; its first line is the enters-tapped family's and its
-- third is unwritable machinery) — the standing untap-step lock as a
-- bare ability line, `abandonedOutpost`'s shape with the new statement.
timeVaultLock : Ability
timeVaultLock = Static (DoesntUntap (AsType Artifact This))

-- "Destroy target permanent." (Vindicate) — the permanent head as an
-- ordinary target phrase: no projected type, battlefield by default
-- ([CR#109.2,110.1]).
vindicate : Effect []
vindicate = Macros.destroy (Macros.target Permanent)

-- "{T}: Exile the top card of your library. Until your next end step,
-- you may play it." (Yasmin Khan's first ability) — the play PERMISSION,
-- the May-side deontic the surface has lacked since chapter fifteen. It
-- is the construction that claims the end-step endpoint the span table
-- has carried as `Unclaimed` since chapter seventeen, and the zone it
-- permits from is the sentence BEFORE it: the exile retags the card, so
-- "it" is in exile and `playableFrom` reads that off the binding.
yasminKhan : Ability
yasminKhan =
  Activated TapSymbol
            (Sequentially [Macros.exile Macros.topCard,
                           Continuously (MayPlay You It) (Just Macros.untilYourNextEndStep)])

-- "Until end of combat on your next turn, you may play that card."
-- (Brazen Cannonade's second line, whose raid trigger and postcombat
-- main-phase header are two turn-structure words this vocabulary does
-- not have) — the SECOND `Unclaimed` cell the permission claims, and
-- the one chapter twenty-two named by card when it wrote the cell's
-- reason down.
brazenCannonadePermission : Effect []
brazenCannonadePermission =
  Sequentially [Macros.exile Macros.topCard,
                Continuously (MayPlay You It) (Just (Until (EndOf Combat (Just Yours))))]

-- ===== The reflexive trigger: what "you do" stands for =====

-- "When this creature enters, you may sacrifice an artifact. When you
-- do, this creature deals 3 damage to any target." (Cornered Crook, the
-- whole card) — the flagship, and the family's dominant enclosure: an
-- OFFER, which is [CR#603.12]'s "allow" half. Three sentences on the
-- page and one ability here, the third of them a SECOND ability that
-- this one creates as it resolves.
corneredCrook : Ability
corneredCrook =
  Triggered When (Enters Macros.thisCreature)
    (Macros.mayWhen You (Macros.sacrifice You (Macros.a Macros.artifact))
                 (DealDamage This (Lit 3) (Macros.target AnyTarget)))

-- "Mill four cards. When you do, return target creature card from your
-- graveyard to your hand." (The Last Ronin, chapter II; the Saga's
-- reminder line and its other two chapters are separate abilities) — the
-- INSTRUCTION enclosure, [CR#603.12]'s "instruct" half and ninety-one of
-- the family's two hundred ninety-one lines. No offer, so no wrapper:
-- the construction takes the bare clause, and the anaphor takes its
-- pronoun from that clause's own agent exactly as `doThen`'s does.
-- Nothing here is optional and the trigger can still fail to fire — a
-- mandatory action that could not be taken is [CR#118.12]'s own
-- Standstill example — which is why this is a trigger rather than a
-- second sentence.
theLastRoninII : Effect []
theLastRoninII =
  Reflexively (Macros.millCards 4)
              (Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.handZ)

-- "Whenever this creature attacks, you may pay {2}{W}. When you do, tap
-- target creature." (Thousand Moons Crackshot, the whole card) — the
-- PAYMENT enclosure, sixty-six lines, and the cell where this
-- construction and `mayThen` sit closest: [CR#118.12] makes the offered
-- action a cost either way. What separates them is not the offer but
-- the consequence's container, and [CR#603.12] puts this one on the
-- stack as its own object.
thousandMoonsCrackshot : Ability
thousandMoonsCrackshot =
  Triggered Whenever (Attacks Macros.thisCreature)
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 2, Macros.pip White]))
                 (Tap (Macros.target Macros.creature)))

-- "Whenever a creature attacks, you may pay {3}. When you do, put a
-- +1/+1 counter on that creature." (Anointer of Valor; its Flying line
-- is a keyword) — the trigger body reading past the enclosure into the
-- HEADER's discourse. Three contexts in one sentence: the event names
-- the attacker, the offer names a payment that mentions nobody, and the
-- reflexive body reaches back over both to "that creature". This is the
-- whole reason the construction is typed in `effIntro body` rather than
-- at the empty context — a trigger row typed at `bs` would have made
-- the attacker unreachable.
anointerOfValor : Ability
anointerOfValor =
  Triggered Whenever (Attacks (Macros.a Macros.creature))
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 3]))
                 (PutCounters (Lit 1) Macros.plusOnePlusOne (That (TypeW Creature))))

-- ===== The card container round: arrival riders =====

-- "{1}, {T}: You may put a land card from your hand onto the battlefield
-- tapped." (Zimone, Quandrix Prodigy's first ability) — the arrival
-- rider's centre, three hundred fifteen lines. The adverbial rides the
-- PLACEMENT and is not a second verb: [CR#110.5b] makes untapped the
-- default a permanent enters with "unless a spell or ability says
-- otherwise", and this is the sentence saying otherwise.
zimoneQuandrixProdigy : Ability
zimoneQuandrixProdigy =
  Activated (Compound [Mana [Macros.generic 1], TapSymbol])
            (Macros.may You (Macros.putOntoBattlefieldTapped
                        (Macros.a (And [Macros.land, InZone (Macros.handOf You)]))))

-- "Whenever this creature attacks, you may put a Soldier creature card
-- from your hand onto the battlefield tapped and attacking."
-- (Preeminent Captain's second line; its First strike line is a
-- keyword) — the two entry riders COMBINED, which is why the bundle is
-- a list and not a scalar. Nineteen lines, and [CR#506.3a] words the
-- second one as an effect that "would put a … permanent onto the
-- battlefield attacking".
preeminentCaptain : Ability
preeminentCaptain =
  Triggered Whenever (Attacks Macros.thisCreature)
    (Macros.may You (Macros.putOntoBattlefieldTappedAttacking
                (Macros.a (And [HasSubtype Soldier, Macros.creature, InZone (Macros.handOf You)]))))

-- "Put target creature card from a graveyard onto the battlefield under
-- your control." (Hymn of Rebirth, the whole card's text) — the
-- CONTROLLER override, a hundred twenty-seven of the hundred thirty-five
-- control lines. [CR#110.2a] already gives the instructed player control
-- of what they put onto the battlefield, so the phrase restates the
-- default here and the other eight lines are the departures from it —
-- which is why the slot is a `Maybe` and not a field.
hymnOfRebirth : Effect []
hymnOfRebirth =
  Macros.putOntoBattlefieldUnderYourControl
    (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))

-- "This creature can't attack unless you control an artifact."
-- (Desperate Castaways, the whole card's text) — the MARKING WORD
-- chapter twenty-eight measured a hundred fifty lines behind, and the
-- structure it needed was already built: this is `asLongAs` over a
-- negated condition with the negation spelled on the subordinator
-- instead of inside the clause. A hundred eleven "can't … unless"
-- lines, and this is the shape all of them have.
desperateCastaways : Ability
desperateCastaways =
  Static (Macros.unlessSo (Exists (And [Macros.artifact, ControlledBy You]))
                   (Deontic Macros.thisCreature Forbid Attack Agent Nothing))

-- "{3}{B}: Destroy target blocking creature at end of combat."
-- (Silent Assassin, the whole card's text) — the SIXTH turn part.
-- [CR#511.2] says what "at end of combat" names in as many words:
-- abilities that trigger there "trigger as the end of combat step
-- begins", so it is a part beginning like the other five and only the
-- surface is short — the part word carries its own boundary noun, which
-- is why [CR#513.1a] had to errata the end step's identical wording ("at
-- end of turn") and left this one alone. The DELAYED clause is the
-- bench's witness rather than the eleven trigger headers, because every
-- one of those wants a blocking-relation predicate or a combat lookback.
silentAssassin : Ability
silentAssassin =
  Activated (Mana [Macros.generic 3, Macros.pip Black])
            (Delayed (BeginningOf EndOfCombat Nothing)
                     (Macros.destroy (Macros.target (And [Blocking, Macros.creature]))))

-- "This creature enters with four +1/+1 counters on it." (Workhorse's
-- first line; its mana ability is a second line and wants a production
-- clause) — the ledger's enters-with-COUNTERS entry, three hundred
-- eighty-six lines, and the re-check that opened it: the blocker was
-- recorded as a missing amount in `TokenRider`, and the amount was never
-- going to fit there — that enum is shared with an unindexed record.
-- Put in its own row instead, the line needs no new vocabulary at all,
-- the count being `WrittenCount`'s and the kind `CounterKind`'s.
workhorse : Ability
workhorse = Static (Macros.entersWithCounters Macros.thisCreature 4 Macros.plusOnePlusOne)

-- ===== The card container round: the spell carrier =====

-- "Counter target spell." (Counterspell, the whole card's text) — the
-- verb the stack row exists for, and fifty-two lines write exactly this
-- sentence. The complement is a SPELL and the word is the ZONE's:
-- [CR#112.1] says "a spell is a card on the stack", so the noun phrase
-- is a zone clause whose word goes unspelled, exactly as "card" goes
-- unspelled under a graveyard clause.
counterspell : Effect []
counterspell = Macros.counterSpell (Macros.target Macros.spell)

-- "Whenever you cast a creature spell, draw a card." (Beast Whisperer,
-- the whole card's text) — the CAST event, chapter twenty-eight's
-- largest unminted family at a thousand and sixty-nine headers. Two
-- noun slots, and the complement needs no vocabulary of its own: "a
-- creature spell" is the type word under the stack clause.
beastWhisperer : Ability
beastWhisperer =
  Triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell]))) Macros.drawACard

-- "You may cast this card from your graveyard." (Skaab Ruinator's third
-- line; its additional-cost line is [CR#118.8]'s and its Flying line a
-- keyword) — the card chapter twenty-eight named by name as waiting for
-- this. What it was waiting for turned out to be two smaller things
-- than a spell carrier: the VERB, which [CR#604.6] brackets as a slot
-- and [CR#701.5b] frees from the complement's kind ("to cast a card is
-- to cast it as a spell"), and the SOURCE-zone slot, "this card"
-- projecting no zone of its own for the derivation to read.
skaabRuinator : Ability
skaabRuinator =
  Static (MayPlay You This {verb = Cast} {from = Just (Macros.graveyardOf You)})

-- ===== The card container round: "this way" =====

-- "Exile the top five cards of your library. You may play cards exiled
-- this way until the end of your next turn." (Escape to the Wilds' first
-- line; its second, "You may play an additional land this turn", is the
-- play-COUNT and has no row) — the participial back-reference, and the
-- two things it needed. The PLURAL participle read, because the actions
-- this construction reads back are group actions and `verbedMatch`
-- refused every `ManyOf` binding; and the MARKING, because "this way" is
-- the same read as "the exiled cards" wearing a deictic instead of a
-- prefix — one construction, two surfaces, and under this grammar's
-- `= 1` uniqueness there is never a second stamp for the deictic to
-- disambiguate against.
escapeToTheWilds : Effect []
escapeToTheWilds =
  Sequentially [Macros.exile (Macros.topCards 5),
                Continuously
                  (MayPlay You (ThoseVerbed Exile CardW {marking = ThisWay}))
                  (Just (Until (EndOf Turn (Just Yours))))]

-- ===== The card container round: six WHOLE CARDS =====
--
-- Everything above this point is a fragment: a clause, a sentence, an
-- ability line. These six are the first terms here that are a printed
-- CARD top to bottom — name, mana cost, type line, every line of the
-- text box, and the power and toughness — one per container shape.
-- Where a part is elided the comment says which and why.

-- Scathe Zombies {2}{B}, Creature — Zombie, 2/2, no rules text. The
-- VANILLA card, and the shape that shows what the container is: an
-- empty text box is a card with nothing to say, not a card that failed
-- to be written. Nothing elided.
scatheZombies : Card
scatheZombies = Macros.card "Scathe Zombies" (Just [Macros.generic 2, Macros.pip Black]) []
                     (MkTypeLine [Zombie] [Creature]) [] (Just (2, 2))

-- Rorix Bladewing {3}{R}{R}{R}, Legendary Creature — Dragon, 6/5,
-- "Flying, haste". The FRENCH VANILLA card: two keyword lines printed
-- as one comma-joined line, which the text box holds as two abilities —
-- [CR#702.1] has an object "list only the name of the ability as a
-- 'keyword'", one name one ability. It is also the
-- SUPERTYPE's witness — the one row [CR#205.4a]'s five that a card here
-- needs. Nothing elided.
rorixBladewing : Card
rorixBladewing =
  Macros.card "Rorix Bladewing" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red, Macros.pip Red])
       [Legendary] (MkTypeLine [Dragon] [Creature])
       [KeywordAbility Flying, KeywordAbility Haste] (Just (6, 5))

-- Aladdin's Ring {8}, Artifact, "{8}, {T}: This artifact deals 4 damage
-- to any target." The ACTIVATED-ability card, whole: a compound cost
-- with a mana run and the tap symbol, and a damage clause with the
-- class word in its recipient slot. Nothing elided.
aladdinsRing : Card
aladdinsRing =
  Macros.card "Aladdin's Ring" (Just [Macros.generic 8]) [] (MkTypeLine [] [Artifact])
       [Activated (Compound [Mana [Macros.generic 8], TapSymbol])
                  (DealDamage Macros.thisArtifact (Lit 4) (Macros.target AnyTarget))]
       Nothing

-- Moonlit Wake {2}{W}, Enchantment, "Whenever a creature dies, you gain
-- 1 life." The TRIGGERED-ability card, whole — the same ability line
-- chapter twenty-eight benched, now with the card around it. Nothing
-- elided.
moonlitWakeCard : Card
moonlitWakeCard =
  Macros.card "Moonlit Wake" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment]) [moonlitWake] Nothing

-- Anthem of Champions {G}{W}, Enchantment, "Creatures you control get
-- +1/+1." The STATIC-ability card, whole, and the shortest statement in
-- the corpus that is one: [CR#604.1] "written as statements, and
-- they're simply true". Nothing elided.
anthemOfChampions : Card
anthemOfChampions =
  Macros.card "Anthem of Champions" (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [Static (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing

-- Counterspell {U}{U}, Instant, "Counter target spell." The SPELL card,
-- whole, and the one that needed the container to exist: the same
-- sentence is a spell ability here and would be an activated ability's
-- effect after a colon on a permanent, and [CR#113.3a] settles which by
-- asking what the CARD is. Nothing elided.
counterspellCard : Card
counterspellCard =
  Macros.card "Counterspell" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant]) [Spell counterspell] Nothing

-- Hymn of Rebirth {3}{G}{W}, Sorcery, "Put target creature card from a
-- graveyard onto the battlefield under your control." A second spell
-- card, and the arrival rider's whole-card witness at the same time.
hymnOfRebirthCard : Card
hymnOfRebirthCard =
  Macros.card "Hymn of Rebirth" (Just [Macros.generic 3, Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Sorcery]) [Spell hymnOfRebirth] Nothing

-- Chandra's Pyrohelix {1}{R}, Instant, "Chandra's Pyrohelix deals 2
-- damage divided as you choose among one or two targets." — the card the
-- divided-amounts ledger entry NAMED, whole and nothing elided. Its line
-- is Forked Bolt's word for word, so the same term spells both cards and
-- the sharing is the witness: "one or two targets" is the enumerated
-- range `oneThrough 2`, which the bench has written since chapter
-- twenty-three. A later audit recorded this card as one count word short
-- of writable; the count word was already here.
chandrasPyrohelixCard : Card
chandrasPyrohelixCard =
  Macros.card "Chandra's Pyrohelix" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant]) [Spell forkedBolt] Nothing

-- Char-Rumbler {2}{R}{R}, Creature — Elemental, -1/3, "Double strike"
-- and "{R}: This creature gets +1/+0 until end of turn." (the double
-- strike line elided, that keyword having no row here; the Elemental
-- subtype elided with the catalog it is not in) — the SIGNED printed
-- power, and the card that showed the container misrepresenting its
-- input instead of refusing it: [CR#107.1b] lets a creature's power be
-- "less than zero", Idris saturates a negative `Nat` literal, so the
-- printed -1 was stored as 0 and no gate could have caught it. Fixing a
-- representation is what a container is for.
charRumbler : Card
charRumbler =
  Macros.card "Char-Rumbler" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Creature])
       [Activated (Mana [Macros.pip Red]) (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))]
       (Just (-1, 3))

-- "{2}{B}: Put a creature card exiled with Sisters of Stone Death onto
-- the battlefield under your control." (Sisters of Stone Death's third
-- ability; the two before it want the blocking RELATION a parallel study
-- owns) — the LINKAGE read's flagship, and it is the CR's own worked
-- example: [CR#607.5]'s Quicksilver Elemental walkthrough names this
-- card and this ability to show that the phrase reaches the exiles of
-- ONE ability and not the exile zone. The source is written as the
-- printed name here and as "this creature" in modern templating; finding
-- 188 keeps the name unread, so the self-word is the phrase.
sistersOfStoneDeathRecall : Ability
sistersOfStoneDeathRecall =
  Activated (Mana [Macros.generic 2, Macros.pip Black])
            (Macros.putOntoBattlefieldUnderYourControl
               (Macros.a (And [Macros.creature, ExiledWith Macros.thisCreature])))

-- Synod Sanctum {1}, Artifact, "{2}, {T}: Exile target permanent you
-- control." / "{2}, Sacrifice this artifact: Return all cards exiled
-- with this artifact to the battlefield under your control." (the second
-- ability; the first wants the PERMANENT word `Predicate` has no row
-- for) — the LIFETIME witness, and the card puts the question the sharp
-- way round: the source is sacrificed IN THE COST, so it is in a
-- graveyard by the time the effect after the colon reads its group, and
-- the group is still there. That is what makes the linkage a
-- source-keyed NOTE rather than a property of a live object — [CR#607.1]
-- links two abilities printed on an object and says nothing about where
-- the object is, and [CR#406.5] has the exiled cards kept in their own
-- pile "due to … the abilities of the cards that exiled them".
synodSanctumReturn : Ability
synodSanctumReturn =
  Activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
            (Macros.putOntoBattlefieldUnderYourControl (AllOf Macros.exiledWithThisArtifact))

-- Cold Storage {4}, Artifact, "{3}: Exile target creature you control."
-- / "Sacrifice this artifact: Return each creature card exiled with this
-- artifact to the battlefield under your control." The LINKAGE's whole
-- card, and the one the ledger has named since chapter fourteen as the
-- linked-ability entry's example. Nothing elided: two abilities, the
-- first exiling and the second reading what it exiled, which is
-- [CR#406.6]'s pair written as shortly as English writes it.
coldStorage : Card
coldStorage =
  Macros.card "Cold Storage" (Just [Macros.generic 4]) [] (MkTypeLine [] [Artifact])
       [Activated (Mana [Macros.generic 3]) (Macros.exile (Macros.target Macros.creatureYouControl)),
        Activated (Do (Macros.sacrifice You Macros.thisArtifact))
                  (Macros.putOntoBattlefieldUnderYourControl
                     (Each (And [Macros.creature, Macros.exiledWithThisArtifact])))]
       Nothing

-- "{1}: Choose a card exiled with this artifact. You may play that card
-- this turn." (Muse Vessel's second ability; its first wants a hand-zone
-- possessive read of a targeted player) — the linkage read under the
-- other determiner, and the sentence AFTER it reads the choice back with
-- an ordinary demonstrative. Two anaphoric channels in one line and they
-- do not touch: the linkage names the group across abilities, "that
-- card" names the choice across sentences.
museVesselPlay : Ability
museVesselPlay =
  Activated (Mana [Macros.generic 1])
            (Sequentially [Choose (Macros.a Macros.exiledWithThisArtifact),
                           Continuously (MayPlay You (That CardW)) (Just Macros.thisTurn)])

-- Aerial Volley {G}, Instant — "Aerial Volley deals 3 damage divided as you
-- choose among one, two, or three target creatures with flying."
aerialVolley : Card
aerialVolley =
  Macros.card "Aerial Volley" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
       [Spell (Macros.dealsDivided This (Lit 3)
                            (TargetGroup (Macros.oneThrough 3)
                              (And [Macros.creature, HasKeyword Flying])))] Nothing

-- Yotian Soldier {3}, Artifact Creature — Soldier, vigilance, 1/4.
yotianSoldier : Card
yotianSoldier =
  Macros.card "Yotian Soldier" (Just [Macros.generic 3]) []
       (MkTypeLine [Soldier] [Artifact, Creature])
       [KeywordAbility Vigilance] (Just (1, 4))

-- Pym Particles' first coordinated sentence is "Target creature gains
-- vigilance until end of turn and can't be blocked this turn." This term
-- transcribes only its first conjunct; "Draw a card" remains elided.
pymParticlesVigilanceGrant : Effect []
pymParticlesVigilanceGrant =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Vigilance) (Just Macros.untilEndOfTurn)

-- Demonic Consultation and Void's exact first sentences. Their later
-- sentences are outside this catalog witness.
demonicConsultationChoice : Effect []
demonicConsultationChoice = Choose (Macros.a (QualityNoun CardName))

voidChoice : Effect []
voidChoice = Choose (Macros.a (QualityNoun Number))

-- The five basic land cards, and Snow-Covered Forest: no mana cost, no P/T,
-- and no ability line — [CR#305.6] gives a land with a basic land type its
-- "{T}: Add [mana symbol]" intrinsically, so what the box prints is a symbol
-- (Snow-Covered Forest's reminder text) and not a sentence. Snow-Covered
-- Forest is the proof that one printed line carries Basic and Snow together.
basicLandCards : List Card
basicLandCards =
  [ Macros.card "Plains" Nothing [Basic] (MkTypeLine [Plains] [Land]) [] Nothing
  , Macros.card "Island" Nothing [Basic] (MkTypeLine [Island] [Land]) [] Nothing
  , Macros.card "Swamp" Nothing [Basic] (MkTypeLine [Swamp] [Land]) [] Nothing
  , Macros.card "Mountain" Nothing [Basic] (MkTypeLine [Mountain] [Land]) [] Nothing
  , Macros.card "Forest" Nothing [Basic] (MkTypeLine [Forest] [Land]) [] Nothing
  ]

snowCoveredForest : Card
snowCoveredForest =
  Macros.card "Snow-Covered Forest" Nothing [Basic, Snow]
       (MkTypeLine [Forest] [Land]) [] Nothing

-- "Target creature gains deathtouch until end of turn." (Bladebrand's first
-- line, "Draw a card." elided), and the double-strike and first-strike twins
-- (Critical Hit, Lightning Blow, each eliding its own second line) — the
-- grant slot `gainsHaste` already had, now over the keywords core keeps for
-- itself.
bladebrand : Effect []
bladebrand =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Deathtouch) (Just Macros.untilEndOfTurn)

criticalHit : Effect []
criticalHit =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility DoubleStrike) (Just Macros.untilEndOfTurn)

lightningBlow : Effect []
lightningBlow =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility FirstStrike) (Just Macros.untilEndOfTurn)

-- "When you cycle this card, put a flying counter on target creature you
-- control." (Avian Oddity; the cycling trigger header elided as other
-- headers are) — the keyword product under the same one-shot put verb the
-- stat product takes, and the only witness `flyingCounter` needs.
avianOddity : Effect []
avianOddity = PutCounters (Lit 1) Macros.flyingCounter (Macros.target Macros.creatureYouControl)

-- "Put a flying counter on each creature you control without flying." (Song
-- of Eärendil's third chapter, its saga header elided) — the possession
-- predicate NEGATED, which is the surface `negatable` licenses and the
-- reason the keyword row and the keyword counter meet on one line.
songOfEarendil : Effect []
songOfEarendil =
  PutCounters (Lit 1) Macros.flyingCounter
              (Each (And [Macros.creature, ControlledBy You, Not (HasKeyword Flying)]))

-- ===== What a complement may exclude, and what a batch may read =====

-- "Each player other than target player creates a 5/5 red Dragon
-- creature token with flying." (Death by Dragons, whole) — the TARGETED
-- anchor, and chapter twenty-six's correction: the old blanket
-- bindingless demand refused this line, reading "announces a referent
-- the sentence never spelled" off a phrase that spells its referent out
-- loud. [CR#601.2c] announces it as the spell is cast like every other
-- target, and `predDelta` carries the announcement out with the phrase
-- the complement modifies. The distributed create is Grismold's own
-- shape at a different count and a token with a keyword.
deathByDragons : Effect []
deathByDragons =
  Create (Each (And [AnyPlayer, OtherThan (Macros.target AnyPlayer)])) (Lit 1)
         (MkToken (Just (Lit 5, Lit 5)) [Red] (MkTypeLine [Dragon] [Creature])
                  [Flying] Nothing) []

-- "Prevent all combat damage that would be dealt by creatures other than
-- target creature this turn." (Terrifying Presence) — the anchor half of
-- the second targeted line, written on its own. The full sentence needs
-- the SOURCE-restricted prevention shield chapter twenty-five ledgered
-- ("dealt BY", where this vocabulary's `Prevents` scopes by recipient),
-- so what lands here is the phrase, which is what the finding was about.
terrifyingPresenceAnchor : Predicate [] Object
terrifyingPresenceAnchor = And [Macros.creature, OtherThan (Macros.target Macros.creature)]

-- The anchor is SINGULAR, which is the second half of the correction and
-- the one that keeps the deferred family deferred: this constructor
-- subtracts ONE referent, and a plural anchor passed to it is group
-- subtraction — finding 111's subset complement, which no corpus line
-- writes ("other than them/those/these" returns zero lines in supported
-- and all-cards scope alike). Written over a counted target, which asks
-- the number question and nothing else; the plural READ ("other than
-- those creatures") is the same gate a group mention later.
-- "[-10]: Target player's life total becomes 1." — the set-to, chapter
-- twenty-two's ledgered player-attribute set. It leaves no outcome
-- mention behind, which is finding 121 made structural: a set is
-- realized as a gain or a loss depending on where the total stood, and
-- the sentence does not say which.
lifeTotalBecomesOne : Effect []
lifeTotalBecomesOne = Macros.lifeTotalBecomes (Macros.target AnyPlayer) (Lit 1)


-- ===== The becomes-status event: observing a transition =====

-- "Whenever a creature an opponent controls becomes tapped, put a +1/+1
-- counter on this creature." (Gideon's Avenger, the whole card's text) —
-- the tapped cell's witness: [CR#603.2e]'s untapped-to-tapped transition
-- under a controller-qualified subject, with the effect reading the
-- SOURCE rather than the event's subject.
gideonsAvenger : Ability
gideonsAvenger =
  Triggered Whenever
            (BecomesStatus (Macros.a (And [Macros.creature, ControlledBy Macros.anOpponent])) Tapped)
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

-- "Whenever a permanent becomes untapped, that permanent's controller
-- mills a card." (Mesmeric Orb, the whole card's text) — the untapped
-- cell AND the event-subject readback: the transition changes no zone,
-- so the subject's binding survives into the trigger body and "that
-- permanent's controller" is the sorted demonstrative under the
-- relational noun.
mesmericOrb : Ability
mesmericOrb =
  Triggered Whenever (BecomesStatus (Macros.a Permanent) Untapped)
            (Macros.millsCards (ControllerOf (That PermanentW)) 1)

-- ===== The timed untap restriction =====

-- "{3}: Target creature doesn't untap during its controller's next
-- untap step." (Barl's Cage, the whole card's text) — the standalone
-- timed clause: no preceding action, the generic-mana activated
-- carrier, and the third-party possessor derived from the target
-- subject ([CR#109.5]).
barlsCage : Ability
barlsCage = Activated (Mana [Macros.generic 3])
                      (DoesntUntapNext (Macros.target Macros.creature) 1)

-- "Tap target creature. It doesn't untap during its controller's next
-- untap step." (Take into Custody) — the separate-sentence tap rider
-- with the plain pronoun: the tap introduces the mention, and the timed
-- clause reads it through the ordinary uniqueness discipline; no
-- tap-specific antecedent relation exists.
takeIntoCustody : Effect []
takeIntoCustody = Sequentially [Tap (Macros.target Macros.creature),
                                DoesntUntapNext It 1]

-- "Chandra's Revolution deals 4 damage to target creature. Tap target
-- land. That land doesn't untap during its controller's next untap
-- step." (Chandra's Revolution) — the multi-action sequence: two
-- singular object mentions precede, so the plain pronoun would be
-- ambiguous and the card switches to the sorted demonstrative, exactly
-- the clarity rule the guide states (§5).
chandrasRevolution : Effect []
chandrasRevolution = Sequentially [DealDamage This (Lit 4) (Macros.target Macros.creature),
                                   Tap (Macros.target Macros.land),
                                   DoesntUntapNext (That (TypeW Land)) 1]

-- "{2}{W}, {T}: This creature deals 3 damage to target attacking or
-- blocking creature. This creature doesn't untap during your next
-- untap step." (Arbalest Elite, the whole ability) — the non-tap
-- predecessor whose restricted object is the damage SOURCE rather than
-- the damage patient, and the self subject whose possessor derives to
-- "your" ([CR#109.5]). The attacking-or-blocking target is
-- `arrowsOfJustice`'s phrase.
arbalestElite : Ability
arbalestElite =
  Activated (Compound [Mana [Macros.generic 2, Macros.pip White], TapSymbol])
            (Sequentially [DealDamage Macros.thisCreature (Lit 3)
                                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])),
                           DoesntUntapNext Macros.thisCreature 1])

-- ===== The face and phase transitions =====

-- "Turn target creature face down." (Cyber Conversion; its second
-- sentence "It's a 2/2 Cyberman artifact creature." elided) — the down
-- cell of the face verb. The elision is [CR#708.2a]'s own seam: the
-- clause turns the permanent face down and the default it lands on is a
-- 2/2 with no text unless the ability LISTS characteristics, and listing
-- them is the layer family's word (base power, toughness and types),
-- ledgered, not this clause's.
cyberConversion : Effect []
cyberConversion = ToFace FaceDown (Macros.target Macros.creature)

-- "Turn target face-down creature an opponent controls face up." (Break
-- Open, the whole card's text) — the up cell, and the one line where
-- both readers of the same status value meet: `HasStatus FaceDown`
-- describes the patient the transition is about and `ToFace FaceUp`
-- changes it.
breakOpen : Effect []
breakOpen = ToFace FaceUp (Macros.target (And [Macros.creature, Macros.faceDown,
                                               ControlledBy Macros.anOpponent]))

-- "Whenever a permanent you control is turned face up, draw a card."
-- (Secret Plans' trigger; the card's other line is the static
-- "Face-down creatures you control get +0/+1.") — the turned-face-up
-- event over a description subject, the family's second-largest cell
-- after the morph self.
secretPlans : Ability
secretPlans =
  Triggered Whenever
            (IsTurnedFace (Macros.a (And [Permanent, ControlledBy You])) FaceUp)
            Macros.drawACard

-- "{U}{U}, {T}: Target creature phases out." (Vodalian Illusionist's
-- ability; the phasing reminder text elided as reminder text always is)
-- — the phase verb's subject-first spelling under an activated carrier:
-- the patient is the SUBJECT, no player being told to do anything.
vodalianIllusionist : Ability
vodalianIllusionist =
  Activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
            (Phases PhasedOut (Macros.target Macros.creature))

-- "Whenever this creature phases out, discard a card." (Teferi's Imp,
-- first trigger; the card's `Phasing` keyword line and its reminder text
-- are elided — the keyword itself is out of scope for this workbench.)
teferisImpPhasesOut : Ability
teferisImpPhasesOut =
  Triggered Whenever (PhaseTransition Macros.thisCreature PhasedOut)
            (Macros.discardsACard You)

-- "Whenever this creature phases in, draw a card." (Teferi's Imp, second
-- trigger) — the paired direction on the same card, which is what makes
-- the one indexed row rather than two verbs the honest shape.
teferisImpPhasesIn : Ability
teferisImpPhasesIn =
  Triggered Whenever (PhaseTransition Macros.thisCreature PhasedIn)
            Macros.drawACard

-- "Whenever this creature phases in, target creature phases out."
-- (Shimmering Efreet's trigger; reminder text elided) — the event and
-- the effect on one line, in opposite directions.
shimmeringEfreet : Ability
shimmeringEfreet =
  Triggered Whenever (PhaseTransition Macros.thisCreature PhasedIn)
            (Phases PhasedOut (Macros.target Macros.creature))

-- ===== The blocking relation and removal from combat =====

-- "{4}, {T}: Remove target attacking or blocking creature from combat."
-- (Labyrinth of Skophos' second ability; its mana ability "{T}: Add {C}."
-- is elided — mana production is unbuilt) — the verb under an activated
-- carrier, and the target phrase carrying the combat state in the
-- DESCRIPTION where the clause's own gate is only zonal.
labyrinthOfSkophos : Ability
labyrinthOfSkophos =
  Activated (Compound [Mana [Macros.generic 4], TapSymbol])
            (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

-- "When this creature enters, remove target attacking or blocking
-- creature from combat." (Hollowhenge Spirit; its Flash and Flying
-- keyword lines elided as keyword lines always are) — the same clause
-- under the entry trigger, the two carriers the verb's whole operative
-- corpus writes.
hollowhengeSpirit : Ability
hollowhengeSpirit =
  Triggered When (Enters Macros.thisCreature)
            (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

-- "Whenever this creature blocks a creature with flying, this creature
-- gets +2/+0 until end of turn." (Netcaster Spider's trigger; its Reach
-- keyword line and reminder text elided) — the TRANSITIVE block header:
-- the patient slot written, and described.
netcasterSpider : Ability
netcasterSpider =
  Triggered Whenever
            (Blocks Macros.thisCreature
                    (Just (Macros.a (And [Macros.creature, HasKeyword Flying]))))
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

-- "Whenever this creature becomes blocked by a creature, this creature
-- gets +2/+2 until end of turn." (Viashino Weaponsmith, the whole card's
-- text) — the same event with the complement WRITTEN, which is what
-- makes the slot optional rather than two rows.
viashinoWeaponsmith : Ability
viashinoWeaponsmith =
  Triggered Whenever
            (BecomesBlocked Macros.thisCreature (Just (Macros.a Macros.creature)))
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

-- "Whenever a creature you control becomes blocked, it gets +1/+1 until
-- end of turn." (Somberwald Alpha's trigger; the card's other line is an
-- activated trample grant) — the complement LEFT OUT, and a DESCRIPTION
-- subject where the witnesses above are the sorted self. When this was
-- written it HAD to be a description: every self-subject line in the bare
-- cell writes its body with the pronoun, and the self-word announced no
-- mention for `It` to read. Chapter forty-eight closed that gap, and
-- Deeproot Warrior — the line named here as the blocked one — is now a
-- witness of its own below.
somberwaldAlpha : Ability
somberwaldAlpha =
  Triggered Whenever (BecomesBlocked (Macros.a Macros.creatureYouControl) Nothing)
            (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

-- "At end of combat, destroy all creatures blocking or blocked by this
-- creature." (Kjeldoran Frostbeast, the whole card's text) — the
-- relational pair under one determiner, which is the line finding 196
-- counted and could not write. The head is the type word and the
-- relation is a disjunction of two ordinary modifiers under it; no group
-- vocabulary is involved.
kjeldoranFrostbeast : Ability
kjeldoranFrostbeast =
  Triggered At (BeginningOf EndOfCombat Nothing)
            (Macros.destroy (AllOf (And [Macros.creature,
                                         Or [BlockerOf Macros.thisCreature,
                                             BlockedBy Macros.thisCreature]])))

-- "Whenever this creature blocks a creature, tap that creature. That
-- creature doesn't untap during its controller's next untap step."
-- (Vertigo Spawn's trigger; its Defender keyword line elided) — the
-- transitive header whose BODY reads the patient twice, and the second
-- transitive witness the round wanted: two singular object mentions
-- precede, so the plain pronoun would be ambiguous and the card writes
-- the sorted demonstrative, which resolves onto the patient the event
-- announced.
vertigoSpawn : Ability
vertigoSpawn =
  Triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
            (Sequentially [Tap (That (TypeW Creature)),
                           DoesntUntapNext (That (TypeW Creature)) 1])

-- ===== The set-level untap cap =====

-- "You can't untap more than one land during your untap step." (Mungha
-- Wurm, the whole card's text) — the SELF subject, and the one line in
-- the family whose step possessive is singular and second-person. The
-- possessive is not written here because the subject's number writes it.
munghaWurm : Ability
munghaWurm = Static (CantUntapMoreThan You 1 Macros.land)

-- "Players can't untap more than one artifact during their untap steps."
-- (Damping Field, the whole card's text; Imi Statue prints the identical
-- line on an artifact rather than an enchantment, so the bench witnesses
-- it once) — the bare plural subject, five of the family's seven lines.
dampingField : Ability
dampingField = Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.artifact)

-- "Players can't untap more than one creature during their untap steps."
-- (Smoke, the whole card's text) — the same shape at a third set word,
-- which is what makes the set an ordinary predicate rather than a closed
-- list.
smoke : Ability
smoke = Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.creature)

-- "As long as this artifact is untapped, players can't untap more than
-- one land during their untap steps." (Winter Orb, the whole card's
-- text) — the cap COMPOSED under the existing conditional wrapper, which
-- is the whole of what the composition needed: no cell opened, no gate
-- relaxed.
winterOrb : Ability
winterOrb =
  Static (Macros.asLongAs (Matches (AsType Artifact This) Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.land))

-- "As long as this artifact is untapped, players can't untap more than
-- two permanents during their untap steps." (Static Orb, the whole
-- card's text) — the bound-two cell and the permanent head, under the
-- same wrapper.
staticOrb : Ability
staticOrb =
  Static (Macros.asLongAs (Matches (AsType Artifact This) Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 2 Permanent))

-- ===== The player's counters =====

-- "Each opponent gets a poison counter." (Prologue to Phyresis' first
-- line; its second, "Draw a card.", is the card's other spell ability)
-- — the player verb at its commonest recipient, and the poison kind's
-- cleanest one-shot line.
prologueToPhyresis : Effect []
prologueToPhyresis = GetsCounters (Each Opponent) (Lit 1) Poison

-- "Whenever this creature attacks, each player gets two rad counters."
-- (Screeching Scorchbeast's first trigger; its Flying and menace keyword
-- line and its mill trigger are the card's other text) — the rad kind,
-- a written count above one, and the symmetric player domain.
screechingScorchbeast : Ability
screechingScorchbeast =
  Triggered Whenever (Attacks Macros.thisCreature)
            (GetsCounters (Each AnyPlayer) (Lit 2) Rad)

-- "Whenever another creature you control dies, you get an experience
-- counter." (Meren of Clan Nel Toth's first line; her second wants a
-- phrasal mana-value comparison against the counter read, which
-- `writtenBound` refuses) — the experience kind and the `You` recipient.
merenOfClanNelToth : Ability
merenOfClanNelToth =
  Triggered Whenever (Dies (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature)))
            (GetsCounters You (Lit 1) Experience)

-- "Each opponent loses all counters." (one mode of Final Act's "Choose
-- one or more —"; the other four modes destroy planeswalkers and battles
-- and exile graveyards, none of them this vocabulary's) — the removal
-- verb's KIND-BLIND cell, where the sentence names no kind at all.
finalActCounterMode : Effect []
finalActCounterMode = LosesAllCounters (Each Opponent) Nothing

-- "Target player loses all poison counters." (Leeches' first sentence;
-- its second, "Leeches deals that much damage to that player.", reads
-- the removed COUNT back as an anaphor and is elided) — the same verb
-- with a kind named, which the scope gate holds to a player's kind.
leeches : Effect []
leeches = LosesAllCounters (Macros.target AnyPlayer) (Just Poison)

-- "At the beginning of your end step, put a number of +1/+1 counters on
-- target creature equal to the number of experience counters you have."
-- (Kratos, Stoic Father's second line; his first is a coordinated
-- trigger and his third the Partner keyword) — the counter READ at its
-- player-holder spelling, feeding the object verb's amount slot. The
-- phrasal amount postposes exactly as `rabidBite`'s "equal to its power"
-- does; the construction is the same and the spelling layer's.
kratosStoicFather : Ability
kratosStoicFather =
  Triggered At (BeginningOf EndStep (Just Yours))
            (PutCounters (CountersOn Experience You) Macros.plusOnePlusOne
                         (Macros.target Macros.creature))

-- ===== The intervening "if" =====

-- "Whenever this creature attacks, if you control a creature with power 4
-- or greater, this creature gets +2/+2 until end of turn." (Ornery
-- Dilophosaur's trigger; its Deathtouch keyword line and reminder text
-- elided) — the intervening slot's first population, and the commonest
-- shape in the family: a control check over an ordinary description,
-- read as `Exists` exactly as a trailing conditional would read it. The
-- slot demands no vocabulary of its own; finding 76's carrier-blind
-- contract is what makes the whole round cheap.
orneryDilophosaur : Ability
orneryDilophosaur =
  Triggered Whenever (Attacks Macros.thisCreature)
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
            {intervening = Just (Exists (And [Macros.creature, ControlledBy You,
                                              Compare Power AtLeast (Lit 4)]))}

-- "Whenever this creature attacks, if an opponent has three or more
-- poison counters, creatures you control get +1/+1 until end of turn."
-- (Incisor Glider's trigger; its Flying keyword line elided, and the
-- "Corrupted —" ability word with it, an ability word having "no special
-- rules meaning" [CR#207.2c]) — the slot carrying a THRESHOLD, and
-- chapter forty's counter read at its first consumer: `CountersOn` feeds
-- `CompareAmt`'s left side with no adaptation at either end.
incisorGlider : Ability
incisorGlider =
  Triggered Whenever (Attacks Macros.thisCreature)
            (Continuously (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))
                          (Just Macros.untilEndOfTurn))
            {intervening = Just (CompareAmt (CountersOn Poison (Macros.a Opponent))
                                            AtLeast (Lit 3))}

-- ===== The event-history lookback =====

-- "Raid — When this creature enters, if you attacked this turn, draw a
-- card." (Storm Fleet Spy, the whole card's text; the Raid ability word
-- elided, an ability word having "no special rules meaning"
-- [CR#207.2c]) — the history read at its largest cell, and the one whose
-- subject is a PLAYER: thirty-eight lines write "if you attacked this
-- turn" and this is the shape they take.
stormFleetSpy : Ability
stormFleetSpy =
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened AttackDeclaration You Lookback.ThisTurn)}

-- "Morbid — At the beginning of each end step, if a creature died this
-- turn, put a +1/+1 counter on this creature." (Vashta Nerada's trigger;
-- its Indestructible and Shadow keyword lines elided, and the Morbid
-- ability word with them) — the OBJECT subject as an indefinite
-- description, and [CR#608.2i]'s own example of why this is not a
-- description of the present: the creature that died is in a graveyard
-- when the condition is checked.
vashtaNerada : Ability
vashtaNerada =
  Triggered At (BeginningOf EndStep (Just EachPlayers))
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
            {intervening = Just (Happened Death (Macros.a Macros.creature)
                                          Lookback.ThisTurn)}

-- "At the beginning of your end step, if you gained life this turn, draw
-- a card." (Tippy-Toe, Terrific Partner's second line; its first is a
-- token-creation replacement) — the life-change event read as HISTORY,
-- which is the whole reason its `EventName` row exists: no `GameEvent`
-- row produces it and the lookback does not need one.
tippyToe : Ability
tippyToe =
  Triggered At (BeginningOf EndStep (Just Yours)) Macros.drawACard
            {intervening = Just (Happened LifeGain You Lookback.ThisTurn)}

-- "When this creature enters, if you've cast two or more spells this
-- turn, draw a card." (Loan Shark's first line; its Plot keyword line
-- elided) — the NUMERIC lookback, which is the count-valued twin under
-- the existing comparison and adds no comparison vocabulary at all.
loanShark : Ability
loanShark =
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (CompareAmt (EventCount SpellCast You Lookback.ThisTurn)
                                            AtLeast (Lit 2))}

-- ===== The history read in the description =====

-- "Sizzling Barrage deals 4 damage to target creature that blocked this
-- turn." (Sizzling Barrage, the whole card's text) — the history read as
-- a postnominal relative, and the rare BARE one: the block family writes
-- a by-complement almost without exception, and this is one of the two
-- lines that does not.
sizzlingBarrage : Effect []
sizzlingBarrage =
  DealDamage This (Lit 4)
             (Macros.target (And [Macros.creature,
                                  HappenedTo BlockDeclaration Lookback.ThisTurn]))

-- "{2}{B}, {T}: Destroy target creature that was dealt damage this turn."
-- (Witch's Mist, the whole card's text) — the read's largest object cell,
-- and [CR#608.2i] doing visible work: the damage was dealt when the
-- creature was somewhere and something else, and the phrase asks only
-- that it was.
witchsMist : Ability
witchsMist =
  Activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
            (Macros.destroy (Macros.target (And [Macros.creature,
                                                 HappenedTo DamageTaken Lookback.ThisTurn])))

-- "Destroy all creatures that entered this turn." (Force of Despair's
-- second line; its first is an alternative cost) — the universal
-- determiner over the read, where the two above target.
forceOfDespair : Effect []
forceOfDespair =
  Macros.destroy (AllOf (And [Macros.creature,
                              HappenedTo Entry Lookback.ThisTurn]))

-- "At the beginning of your end step, put a +1/+1 counter on this
-- creature for each opponent who was dealt damage this turn." (Furious
-- Spinesplitter's trigger; its Trample keyword line elided) — the PLAYER
-- head, whose relativizer is "who" where the three above write "that",
-- and the for-each domain's first history consumer: `CountOf` takes the
-- read as an ordinary modifier and needed nothing.
furiousSpinesplitter : Ability
furiousSpinesplitter =
  Triggered At (BeginningOf EndStep (Just Yours))
            (PutCounters (Macros.forEach (And [Opponent,
                                               HappenedTo DamageTaken Lookback.ThisTurn]))
                         Macros.plusOnePlusOne Macros.thisCreature)

-- ===== Characteristic predicates =====

-- "Players can't untap more than one nonbasic land during their untap
-- steps." (Winter Moon, the whole card's text) — chapter thirty-nine's
-- named gap, BACKFILLED: the cap's fifth set word was the one that needed
-- a supertype predicate, and it needed the negated prefix form at that.
winterMoon : Ability
winterMoon =
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1
                            (And [Macros.land, Not (HasSupertype Basic)]))

-- "Destroy target nonblack creature that entered this turn." (Cradle to
-- Grave, the whole card's text) — chapter forty-three's failed witness,
-- BACKFILLED, and the round's composition test: the colour negation and
-- the history read are two ordinary modifiers under one head, and neither
-- knew about the other.
cradleToGrave : Effect []
cradleToGrave =
  Macros.destroy (Macros.target (And [Macros.creature, Not (ColorIs Black),
                                      HappenedTo Entry Lookback.ThisTurn]))

-- "Destroy target legendary creature." (Hero's Demise, the whole card's
-- text) — the supertype read positively, where Winter Moon reads it
-- negated.
herosDemise : Effect []
herosDemise =
  Macros.destroy (Macros.target (And [Macros.creature, HasSupertype Legendary]))

-- "Destroy target multicolored permanent." (Pure // Simple's Simple half;
-- a split card's two halves are two spells and the bench witnesses this
-- one) — the colour-COUNT word, which is not a colour ([CR#105.4]) and
-- not a value of the colour row.
simpleHalf : Effect []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Multicolored]))

-- "{2}{U}: Creatures named Leitmotif Composer can't be blocked this
-- turn." (Leitmotif Composer's activated ability) — the NAME read, whose
-- string is a payload the spelling carries and no gate ever looks at.
leitmotifComposer : Ability
leitmotifComposer =
  Activated (Mana [Macros.generic 2, Macros.pip Blue])
            (Continuously (Deontic (AllOf (And [Macros.creature,
                                                Named "Leitmotif Composer"]))
                                   Forbid Block Patient Nothing)
                          (Just Macros.thisTurn))

-- ===== The possessor axis =====

-- "At the beginning of each upkeep, if you lost life last turn, put a
-- +1/+1 counter on this creature." (Paladin of Atonement's first line;
-- his second is a dies-trigger reading his own toughness) — chapter
-- forty-two's `LastTurn` window, BACKFILLED: the window landed there with
-- corpus evidence and no bench positive because every one of its lines
-- writes the bare-each header, and the bare-each header is what this
-- round gave a possessor.
paladinOfAtonement : Ability
paladinOfAtonement =
  Triggered At (BeginningOf Upkeep (Just EachPlayers))
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
            {intervening = Just (Happened LifeLoss You Lookback.LastTurn)}

-- "Raid — At the beginning of each of your postcombat main phases, if you
-- attacked this turn, exile the top card of your library." (Brazen
-- Cannonade's second line, first sentence; the Raid ability word elided,
-- and its second sentence — "Until end of combat on your next turn, you
-- may play that card." — is an EXISTING PIN, the permission-under-an
-- end-of-combat span `badPermissionUntilEndOfCombat` refuses) — finding
-- 196's named blocker, three chapters in the making: the until-duration
-- was chapter seventeen's, the raid lookback chapter forty-two's, and the
-- postcombat main phase is this round's.
brazenCannonade : Ability
brazenCannonade =
  Triggered At (BeginningOf PostcombatMain (Just EachYours))
            (Macros.exile Macros.topCard)
            {intervening = Just (Happened AttackDeclaration You Lookback.ThisTurn)}

-- "At the beginning of your first main phase, draw a card." (Four Knocks'
-- second line; its Vanishing keyword line and reminder text elided) — the
-- first main phase, one of the three parts this round added, and the only
-- one of its 52 headers whose body is not mana production.
fourKnocks : Ability
fourKnocks =
  Triggered At (BeginningOf FirstMain (Just Yours)) Macros.drawACard

-- "{2}{R}{R}{R}: Return this card from your graveyard to your hand.
-- Activate only during your upkeep." (Hammer of Bogardan's second line;
-- its first is a spell ability) — the activation WINDOW's first
-- population, and the slot chapter twenty-eight left open with its
-- blocker named.
hammerOfBogardan : Ability
hammerOfBogardan =
  Activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red])
            (Move This Macros.handZ)
            {window = Just (DuringPart Upkeep (Just Yours))}

-- ===== The requirement =====

-- "This creature attacks each combat if able." (Berserkers of Blood
-- Ridge, the whole card's text) — the requirement's standing form, and
-- the cleanest possible demonstration that the cadence is not a span: the
-- line is a printed static ability with no duration adverbial anywhere,
-- so it is a `Static` line and never a `Continuously` clause.
berserkersOfBloodRidge : Ability
berserkersOfBloodRidge = Static (Deontic Macros.thisCreature Require Attack Agent Nothing)

-- "{1}{G}: Target creature blocks this creature this turn if able."
-- (Trumpeting Armodon, the whole card's text) — the Block/Agent cell,
-- whose PATIENT the table requires: all 38 of the family's lines name
-- what must be blocked, and the span rides the `Continuously` envelope
-- exactly as a restriction's does.
trumpetingArmodon : Ability
trumpetingArmodon =
  Activated (Mana [Macros.generic 1, Macros.pip Green])
            (Continuously (Deontic (Macros.target Macros.creature) Require Block Agent
                                   (Just Macros.thisCreature))
                          (Just Macros.thisTurn))

-- "{2}{G}: This creature must be blocked this turn if able." (Loathsome
-- Catoblepas' first ability; its second is a dies-trigger) — the PATIENT
-- role, which writes the modal outright where the two agent cells write
-- the plain present, and which names no blocker.
loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Activated (Mana [Macros.generic 2, Macros.pip Green])
            (Continuously (Deontic Macros.thisCreature Require Block Patient Nothing)
                          (Just Macros.thisTurn))

-- "You may choose not to untap this artifact during your untap step."
-- (Ashnod's Battle Gear's first line; its second is an activated pump
-- riding a for-as-long-as-tapped condition) — the permission to decline,
-- the untap step's third static after the lock and the cap.
ashnodsBattleGear : Ability
ashnodsBattleGear = Static (MayDeclineUntap Macros.thisArtifact)

-- ===== The designations =====

-- "At the beginning of your end step, if you're the monarch, put a +1/+1
-- counter on this creature." (Throne Warden, the whole card's text) — the
-- monarch READ, and the route the intervening-if slot already had: the
-- 18-line check family costs no condition vocabulary of its own, being
-- `Matches You` over an ordinary description.
throneWarden : Ability
throneWarden =
  Triggered At (BeginningOf EndStep (Just Yours))
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
            {intervening = Just (Matches You (HasPlayerDesignation Monarch))}

-- "When Aragorn enters, you become the monarch." (Aragorn, King of
-- Gondor's first trigger; his keyword line and his attack trigger are the
-- card's other text) — the designation GAINED, and the `Monarch` value's
-- own verb.
aragornKingOfGondor : Ability
aragornKingOfGondor =
  Triggered When (Enters Macros.thisCreature) (TakesDesignation You Monarch)

-- "{3}, {T}: Goad target creature." (reminder text elided) — the keyword
-- ACTION whose effect is [CR#701.15b]'s designation. What the reminder
-- spells out is the designation's rules text and never a printed
-- operative clause, which is why chapter forty-six's requirement row
-- neither writes it nor needs to.
goadTargetCreature : Ability
goadTargetCreature =
  Activated (Compound [Mana [Macros.generic 3], TapSymbol])
            (Goad (Macros.target Macros.creature))

-- "Whenever a goaded creature attacks, it deals 1 damage to its
-- controller." — the object designation read prenominally, in a trigger
-- header's subject.
goadedAttackTrigger : Ability
goadedAttackTrigger =
  Triggered Whenever (Attacks (Macros.a (And [Macros.creature, IsGoaded])))
            (DealDamage It (Lit 1) (ControllerOf It))

-- "Whenever day becomes night or night becomes day, draw a card."
-- (Firmament Sage's second line; its first is an as-enters conditional) —
-- the GAME scope's event, whose spelling is the whole disjunctive phrase
-- ten of the family's eleven headers write.
firmamentSage : Ability
firmamentSage = Triggered Whenever DayNightShift Macros.drawACard

-- ===== The event subject's own mention =====

-- "Whenever this creature becomes blocked, it gets +1/+1 until end of
-- turn." (Deeproot Warrior, the whole card's text) — the line chapter
-- thirty-eight named as blocked and witnessed around, landing at last.
-- Its "it" is the trigger's own subject, which the event's
-- after-discourse now mints (`selfSubjIntro`).
deeprootWarrior : Ability
deeprootWarrior =
  Triggered Whenever (BecomesBlocked Macros.thisCreature Nothing)
            (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

-- "Whenever this creature attacks, it gets +2/+0 until end of turn."
-- (Borderland Marauder, the whole card's text) — the same mention at a
-- different event row, which is what shows the minting is the POSITION's
-- and not one row's.
borderlandMarauder : Ability
borderlandMarauder =
  Triggered Whenever (Attacks Macros.thisCreature)
            (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

-- ===== The deontic gate =====


-- "This creature can't block creatures with power 3 or greater unless
-- you pay {1}." (Hipparion, the whole card's text) — the COST gate, and
-- the only line in its twenty-six-line family this grammar can write
-- whole. It needs both of the round's new capabilities at once: the
-- gate polarity carrying its cost, and the restriction's optional
-- PATIENT, which the merged table admits at exactly this cell.
hipparion : Ability
hipparion =
  Static (Deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  Block Agent
                  (Just (AllOf (And [Macros.creature,
                                     Compare Power AtLeast (Lit 3)]))))

-- ===== The conditional's threading =====

-- "As long as Frodo Baggins is your Ring-bearer, it must be blocked if
-- able." (Frodo Baggins' second line; his first — "Whenever Frodo Baggins
-- or another legendary creature you control enters, the Ring tempts you"
-- — stays elided, its Ring-tempts being a keyword action [CR#701.54a]
-- and its subject a coordinated event) — THE ARC'S PAYOFF. Chapter
-- forty-six minted the requirement he needs, chapter forty-seven the
-- Ring-bearer read, chapter forty-eight diagnosed exactly why his
-- pronoun would not resolve, and this chapter threads the container that
-- was the diagnosis. The "it" is the CONDITION's subject, minted for the
-- body by `condIntro`.
frodoBaggins : Ability
frodoBaggins =
  Static (Macros.asLongAs (Matches Macros.thisCreature YourRingBearer)
                          (Deontic It Require Block Patient Nothing))

-- "As long as this creature is attacking, it gets +2/+0." (Adanto
-- Vanguard's first line; its second is an activated indestructible
-- grant) — the same threading over an ordinary status condition, which
-- is what shows the minting is the CONTAINER's and not the Ring-bearer
-- read's: fifty-five of the fronted lines write a self-typed subject and
-- pronominalise it, and this is their shape.
adantoVanguard : Ability
adantoVanguard =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets It (PtUp (Lit 2)) (PtUp (Lit 0))))

-- ===== The attachment host =====

-- "Enchanted creature attacks each combat if able." (Bloodshed Fever,
-- the whole card's text; its "Enchant creature" line is the Aura's own
-- keyword ability, elided as keyword lines always are) — chapter
-- forty-six's recorded block, BACKFILLED: that chapter minted the
-- requirement and could not witness its second-largest carrier because
-- the subject had no noun. This is that noun.
bloodshedFever : Ability
bloodshedFever =
  Static (Deontic (AttachHost Enchanted (TypeW Creature)) Require Attack Agent Nothing)

-- "Enchanted creature can't attack unless its controller pays {3}."
-- (Brainwash, the whole card's text; Enchant creature elided) — chapter
-- forty-nine's recorded block, BACKFILLED, and the cost gate's SECOND
-- witness: that chapter landed the gate with exactly one line because
-- every other flat gate in the family is an Aura. The payer is derived
-- and is the HOST's controller, which is what "its controller" writes.
brainwash : Ability
brainwash =
  Static (Deontic (AttachHost Enchanted (TypeW Creature))
                  (GatedBy (Mana [Macros.generic 3])) Attack Agent Nothing)

-- "As long as Enkira is equipped, it must be blocked if able." (Enkira,
-- Hostile Scavenger's second line; his enters-trigger and his attack
-- trigger are the card's other text) — chapter fourteen's threading
-- meeting the INVERSE direction: the condition asks whether the self has
-- an attachment, and the body pronominalises the self. Frodo's sibling,
-- and the second card to close on `condIntro`.
enkiraHostileScavenger : Ability
enkiraHostileScavenger =
  Static (Macros.asLongAs (Matches Macros.thisCreature IsEquipped)
                          (Deontic It Require Block Patient Nothing))

-- "Whenever enchanted creature attacks, it deals 2 damage to any
-- target." (Extra Arms, the whole card's text; Enchant creature elided)
-- — the attachment host as an EVENT SUBJECT with a pronoun in the body,
-- which is round twelve's minting extended to the new noun. Its mention
-- is `TheD` and not `SelfD`, and the corpus is why: five lines
-- demonstrate back to an attachment host where none demonstrates back to
-- a trigger's own self.
extraArms : Ability
extraArms =
  Triggered Whenever (Attacks (AttachHost Enchanted (TypeW Creature)))
            (DealDamage It (Lit 2) (Macros.target AnyTarget))

-- ===== The game outcome =====

-- "You can't lose the game." (Lich's Mastery's second line; its Hexproof
-- keyword line and its two life-change triggers are the card's other
-- text) — the OUTCOME GATE, and the corpus's only one-sided line: every
-- other gate line is the paired two-clause sentence the two cards below
-- write, whose second half waited on the plural-player subject until
-- chapter fifty-three.
lichsMasteryGate : Ability
lichsMasteryGate = Static (OutcomeGate CantLose You)

-- "When Lich's Mastery leaves the battlefield, you lose the game." (the
-- same card's last line) — the IMPERATIVE outcome over the self, and the
-- card's own answer to the gate two lines above it.
lichsMasteryLoss : Ability
lichsMasteryLoss =
  Triggered When (Leaves Macros.thisEnchantment) (Concludes LoseGame You)

-- "Whenever Phage deals combat damage to a player, that player loses the
-- game." (Phage the Untouchable's third line; her enters-trigger reads a
-- cast-from-hand history this grammar has no window for, and her second
-- is a regeneration rider) — the TRIGGERED PAYLOAD, whose patient is a
-- third party the same ability's trigger bound: seventeen lines take this
-- shape and the demonstrative is how they all reach it.
phageTheUntouchable : Ability
phageTheUntouchable =
  Triggered Whenever
            (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
            (Concludes LoseGame (That PlayerW))

-- ===== The plural player subject =====

-- Platinum Angel {7}, Artifact Creature — Angel, 4/4, "Flying / You can't
-- lose the game and your opponents can't win the game." The gate family's
-- DOMINANT surface, whole, and the card the plural player subject was
-- deferred four chapters to reach: eight supported cards write this one
-- sentence and Lich's Mastery's one-sided line is the outlier the bench
-- had to settle for. Its coordination is the surface's and not a
-- construction: two independent clauses, each a statement that is simply
-- true, are two static abilities ([CR#113.3d]), which is finding 374's
-- "one clause, one gate" read off the card. Nothing elided.
platinumAngel : Card
platinumAngel =
  Macros.card "Platinum Angel" (Just [Macros.generic 7]) []
       (MkTypeLine [Angel] [Artifact, Creature])
       [ KeywordAbility Flying
       , Static (OutcomeGate CantLose You)
       , Static (OutcomeGate CantWin (PlayerGroup YourOpponents)) ]
       (Just (4, 4))

-- Abyssal Persecutor {2}{B}{B}, Creature — Demon, 6/6, "Flying, trample /
-- You can't win the game and your opponents can't lose the game." The
-- near-MIRROR of the card above — the same two rows with the two subjects
-- exchanged, which is the whole of its drawback — and the pair is why the
-- gate takes a kind and a subject and reads nothing about who is being
-- protected. Nothing elided.
abyssalPersecutor : Card
abyssalPersecutor =
  Macros.card "Abyssal Persecutor"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Demon] [Creature])
       [ KeywordAbility Flying
       , KeywordAbility Trample
       , Static (OutcomeGate CantWin You)
       , Static (OutcomeGate CantLose (PlayerGroup YourOpponents)) ]
       (Just (6, 6))

-- ===== The union possessor =====

-- Smog Elemental {4}{B}{B}, Creature — Elemental, 3/3, "Flying /
-- Creatures with flying your opponents control get -1/-1." The UNION
-- READ's first bench line, and the phrase is the most-written thing this
-- grammar could not spell — 377 supported cards write "your opponents
-- control" as a relative clause, against the 903 that write "an opponent
-- controls" and the 294 that write "you don't control". Its head is the
-- ordinary conjunction: a head noun, a keyword modifier, and the control
-- clause with a plural possessor in it, which is the whole of what the
-- widening buys. Nothing elided.
smogElemental : Card
smogElemental =
  Macros.card "Smog Elemental"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Elemental] [Creature])
       [ KeywordAbility Flying
       , Static (Gets (AllOf (And [Macros.creature, HasKeyword Flying,
                                   ControlledBy (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

-- "Creatures your opponents control attack each combat if able." (Angler
-- Turtle's second line; its Hexproof keyword line is the card's other
-- text and that keyword is out of scope until the verifier transition) —
-- the union read as a DEONTIC subject, which is chapter forty-six's
-- requirement row meeting the possessor it was always missing: the
-- corpus's own way of saying "your opponents' creatures must attack" is
-- to describe the creatures, never to name the players.
anglerTurtle : Ability
anglerTurtle =
  Static (Deontic (AllOf Macros.creatureYourOpponentsControl) Require Attack Agent Nothing)

-- ===== The loss event =====

-- "Whenever a player loses the game, put five +1/+1 counters on this
-- creature." (Blood Tyrant's third line; its keyword line spells and its
-- upkeep trigger does not, that one scaling a counter count by the life
-- lost this way) — the OUTCOME WATCHED, and the shape five of the eight
-- watching lines take: the repeatable word, an indefinite player subject,
-- and a body that never asks which player it was.
bloodTyrant : Ability
bloodTyrant =
  Triggered Whenever (LosesGame (Macros.a AnyPlayer))
            (PutCounters (Lit 5) Macros.plusOnePlusOne Macros.thisCreature)

-- "If you would lose the game, instead exile The Golden Throne and your
-- life total becomes 1." (The Golden Throne's first line; its ability
-- word "Arcane Life-support" is elided as the practice has been since
-- chapter forty-one, and its mana ability waits on mana production) —
-- the OUTCOME REPLACED, standing form: five of the seven replacement
-- lines write exactly this "If you would … instead" with no bound on how
-- often, which is `Repeatedly`.
theGoldenThrone : Ability
theGoldenThrone =
  Static (Intercepts (LosesGame You)
                     (Sequentially [Macros.exile Macros.thisArtifact,
                                    ChangeLife You (Set (Lit 1))])
                     Repeatedly)

-- "The next time you would lose the game this turn, instead draw seven
-- cards and your life total becomes 1." (Stunning Reversal's first
-- sentence; "Exile Stunning Reversal" is the card's other line) — the
-- same replacement BOUNDED, and it exercises two slots at once: the
-- multiplicity word `NextTimeOnly` and the this-turn span, which rides
-- the `Continuously` envelope exactly as `Intercepts`' own comment says
-- a spanned replacement must.
stunningReversal : Ability
stunningReversal =
  Spell (Continuously (Intercepts (LosesGame You)
                                  (Sequentially [Draw You (Lit 7),
                                                 ChangeLife You (Set (Lit 1))])
                                  NextTimeOnly)
                      (Just Macros.thisTurn))

-- ===== The life-total read =====

-- "At the beginning of your upkeep, if you have 40 or more life, you win
-- the game." (Felidar Sovereign's third line; its Vigilance and Lifelink
-- keyword lines wait on the keyword catalog) — the THRESHOLD, which is
-- the family's dominant surface at 31 lines and needed nothing but a
-- left-hand side: the comparison, the intervening "if" and the outcome
-- verb have all been writable for chapters. The card was named as blocked
-- in chapter fifty-two and again in fifty-five, and the block was always
-- this one read.
-- [CR#603.4] is what the intervening slot spells here, and the CR's own
-- worked example for the double check uses this card.
felidarSovereign : Ability
felidarSovereign =
  Triggered At (BeginningOf Upkeep (Just Yours)) (Concludes WinGame You)
            {intervening = Just (CompareAmt (PlayerStatOf LifeTotal You)
                                            AtLeast (Lit 40))}

-- Exquisite Archangel {5}{W}{W}, Creature — Angel, 5/5, "Flying / If you
-- would lose the game, instead exile this creature and your life total
-- becomes equal to your starting life total." The whole card, and the
-- second read's only writable line: chapter fifty-five landed the loss
-- replacement and this one clause of it stayed out for want of the
-- STARTING total, which is a term of its own ([CR#119.1]) and not a bound
-- on the current one. Nothing elided.
exquisiteArchangel : Card
exquisiteArchangel =
  Macros.card "Exquisite Archangel"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , Static (Intercepts (LosesGame You)
                            (Sequentially [Macros.exile Macros.thisCreature,
                                           ChangeLife You (Set (PlayerStatOf StartingLifeTotal You))])
                            Repeatedly) ]
       (Just (5, 5))

-- Space-Time Anomaly {2}{W}{U}, Sorcery, "Target player mills cards equal
-- to your life total." The whole card, and the BARE read: no comparison,
-- no set, just the number standing where an amount stands, which is what
-- makes it the cleanest evidence that the row is an `Amount` and not a
-- condition's fragment. Nothing elided.
spaceTimeAnomaly : Card
spaceTimeAnomaly =
  Macros.card "Space-Time Anomaly"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Mill (Macros.target AnyPlayer) (PlayerStatOf LifeTotal You)) ]
       Nothing

-- ===== The read-versus-read comparison =====

-- Timely Reinforcements {2}{W}, Sorcery, "If you have less life than an
-- opponent, you gain 6 life. If you control fewer creatures than an
-- opponent, create three 1/1 white Soldier creature tokens." The whole
-- card, and both readers of the widened bound in one sentence pair: a
-- player stat held against a player stat, then a count held against a
-- count. The second clause is why the widening is not the life read's
-- private extension — the standard is any amount the sentence can name.
-- Both clauses spell the comparison as a HAVE- or CONTROL-clause, which
-- is the family's dominant surface and the frame's business, not the
-- catalog's. Nothing elided.
timelyReinforcements : Card
timelyReinforcements =
  Macros.card "Timely Reinforcements"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ If (Macros.gainsLife You (Lit 6))
                       (CompareAmt (PlayerStatOf LifeTotal You) Less
                                   (PlayerStatOf LifeTotal Macros.anOpponent))
                       Nothing
                  , If (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [Soldier]))
                       (CompareAmt (CountOf Macros.creatureYouControl) Less
                                   (CountOf (And [Macros.creature,
                                                  ControlledBy Macros.anOpponent])))
                       Nothing ]) ]
       Nothing

-- "You gain 2 life. Then if you have more life than an opponent, draw a
-- card." (Survival Cache's first two sentences; its Rebound line waits on
-- the keyword catalog) — the queue's own headline sentence, and the
-- strict relation at the frame the family actually writes. The "then" is
-- the sequence's, exactly as it is everywhere else.
survivalCache : Effect []
survivalCache =
  Sequentially [ Macros.gainsLife You (Lit 2)
               , If Macros.drawACard
                    (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                (PlayerStatOf LifeTotal Macros.anOpponent))
                    Nothing ]

-- "As long as your life total is greater than or equal to your starting
-- life total, creatures you control get +1/+1." (Path of Bravery's first
-- line; its attack trigger wants the attacking-creature count) — four
-- landed pieces meeting: the conditional static, the anthem, chapter
-- fifty-six's two player stats, and the or-relation spelled long because
-- its bound is a read. Current against starting is the second stat's
-- dominant use.
pathOfBravery : Ability
pathOfBravery =
  Static (Macros.asLongAs (CompareAmt (PlayerStatOf LifeTotal You) AtLeast
                                      (PlayerStatOf StartingLifeTotal You))
                          (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))))

-- Ensnaring Bridge {3}, Artifact, "Creatures with power greater than the
-- number of cards in your hand can't attack." The whole card, and the
-- PREDICATE frame's witness: one bound test, two frames, and this is the
-- second reader. The vs-read half is where this frame's weight lies —
-- seventy-seven supported lines bound a power, toughness or mana value by
-- another read. Nothing elided.
ensnaringBridge : Card
ensnaringBridge =
  Macros.card "Ensnaring Bridge" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Deontic (AllOf (And [Macros.creature,
                                      Compare Power Greater
                                              (CountOf (InZone (Macros.handOf You)))]))
                         Forbid Attack Agent Nothing) ]
       Nothing

-- "When this creature enters, if your life total is less than 7, your
-- life total becomes 7." (Elderscale Wurm's second line; its Trample and
-- its damage-prevention shield are separate lines) — the STRICT relation
-- against a WRITTEN bound, which chapter sixteen's finding 60 recorded as
-- written zero times. It is written: fifteen supported lines put "fewer
-- than [numeral]" in a condition and this one puts "less than 7" there,
-- and the cell that is genuinely empty is the postnominal qualifier's
-- ("with power less than 4"), which is a fact about one frame's
-- preferences and not about the relation.
elderscaleWurm : Ability
elderscaleWurm =
  Triggered When (Enters Macros.thisCreature)
            (ChangeLife You (Set (Lit 7)))
            {intervening = Just (CompareAmt (PlayerStatOf LifeTotal You) Less (Lit 7))}

-- "At the beginning of each combat, if you have more life than an
-- opponent, this creature gains double strike until end of turn."
-- (Glorious Enforcer's second line; its Flying and lifelink keyword line
-- waits on the keyword catalog) — the strict relation under a TRIGGER's
-- intervening "if", which is where the family most often sits. The
-- round's recon had this card down as blocked on its bare "each combat"
-- timing; chapter forty-five's bare-each ruling had already answered it,
-- and the cell is the active player's combat under its second spelling.
gloriousEnforcer : Ability
gloriousEnforcer =
  Triggered At (BeginningOf Combat (Just EachPlayers))
            (Macros.gains Macros.thisCreature (KeywordAbility DoubleStrike)
                          (Just Macros.untilEndOfTurn))
            {intervening = Just (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                            (PlayerStatOf LifeTotal Macros.anOpponent))}

-- ===== The difference =====

-- "At the beginning of your upkeep, if you have fewer than seven cards in
-- hand, draw cards equal to the difference." (Damia, Sage of Stone's
-- third line; its Deathtouch keyword and its "Skip your draw step" wait
-- on the keyword catalog and on a step-skipping vocabulary) — the
-- anaphoric gap's archetype, and the sentence the queue carried. Both
-- halves are chapter fifty-seven's: the comparison is the STRICT relation
-- against a written bound that finding 401 had to un-refuse, so without
-- the previous round none of this family is writable at all.
damia : Ability
damia =
  Triggered At (BeginningOf Upkeep (Just Yours))
            (Draw You TheDifference)
            {intervening = Just (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                            Less (Lit 7))}

-- "When Krang enters, if you have fewer than four cards in hand, draw
-- cards equal to the difference." (Krang, Master Mind's second line; its
-- Affinity and its per-artifact pump are separate lines) — the same
-- reading at a different trigger word and a different event, which is
-- what says the gap belongs to the intervening SLOT and not to one
-- header. Doctor Octopus, The Ten Rings and Kozilek write this sentence
-- again at their own numbers.
krang : Ability
krang =
  Triggered When (Enters Macros.thisCreature)
            (Draw You TheDifference)
            {intervening = Just (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                            Less (Lit 4))}

-- ===== The definition rider =====

-- "{T}: Create X 1/1 red Goblin creature tokens, where X is the number of
-- Goblins you control." (Krenko, Mob Boss's ability; its 3/3 body and its
-- Legendary line are the card's, not the ability's) — the rider at its
-- cleanest: one definition, one read, and a count on the right. The
-- catalog grew one word, `Goblin`, for the token and the domain alike.
krenko : Ability
krenko =
  Activated TapSymbol
            (WhereLetter LetterX (CountOf (And [HasSubtype Goblin, ControlledBy You]))
                         (Create You (DefinedLetter LetterX)
                            (Macros.creatureTok 1 1 [Red] [Goblin]) []))

-- Chain Reaction {2}{R}{R}, Sorcery, "Chain Reaction deals X damage to
-- each creature, where X is the number of creatures on the battlefield."
-- The whole card, and the rider over the damage clause: the definition is
-- a count of the very set the damage is dealt to, which is why the card
-- writes a letter rather than repeating the phrase. Nothing elided.
chainReaction : Card
chainReaction =
  Macros.card "Chain Reaction"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (WhereLetter LetterX (CountOf Macros.creature)
                            (DealDamage This (DefinedLetter LetterX)
                                        (Each Macros.creature))) ]
       Nothing

-- Ivory Tower {1}, Artifact, "At the beginning of your upkeep, you gain X
-- life, where X is the number of cards in your hand minus 4." The whole
-- card, and the definition that pays chapter fifty-eight's debt: `Minus`
-- landed there with no bench line at all, and the rider is what its five
-- corpus lines were waiting for. The floor is the row's ([CR#107.1b]) and
-- the card relies on it — a hand of two gains nothing, not minus two.
ivoryTower : Card
ivoryTower =
  Macros.card "Ivory Tower" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Triggered At (BeginningOf Upkeep (Just Yours))
                  (WhereLetter LetterX
                               (Minus (CountOf (InZone (Macros.handOf You))) (Lit 4))
                               (Macros.gainsLife You (DefinedLetter LetterX))) ]
       Nothing

-- Harsh Sustenance {1}{W}{B}, Sorcery, "Harsh Sustenance deals X damage to
-- any target and you gain X life, where X is the number of creatures you
-- control." The whole card, and the MULTI-MENTION witness: one definition
-- read at two sites, which is the half of the family a sugar reading
-- cannot serve — inline the definition at each site and the card says the
-- count twice, which is not what it says. The coordinated pair is
-- transcribed as a two-clause sequence exactly as Arc Trail's is.
harshSustenance : Card
harshSustenance =
  Macros.card "Harsh Sustenance"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (WhereLetter LetterX (CountOf Macros.creatureYouControl)
                            (Sequentially
                              [DealDamage This (DefinedLetter LetterX)
                                          (Macros.target AnyTarget),
                               Macros.gainsLife You (DefinedLetter LetterX)])) ]
       Nothing

-- ===== The variable pump =====

-- Nightmarish End {2}{B}, Instant, "Target creature gets -X/-X until end
-- of turn, where X is the number of cards in your hand." The whole card,
-- and the pump the two integers could not spell: a NEGATIVE modification
-- whose magnitude is read rather than written, which is the definition
-- rider meeting the stat change. Its right side is Ivory Tower's own
-- count one card over. Nothing elided.
nightmarishEnd : Card
nightmarishEnd =
  Macros.card "Nightmarish End"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (WhereLetter LetterX (CountOf (InZone (Macros.handOf You)))
                            (Macros.gets (Macros.target Macros.creature)
                                         (PtDown (DefinedLetter LetterX))
                                         (PtDown (DefinedLetter LetterX))
                                         (Just Macros.untilEndOfTurn))) ]
       Nothing

-- "Adelbert Steiner gets +1/+1 for each Equipment you control."
-- (Adelbert Steiner's second line; its Lifelink keyword waits on the
-- keyword catalog and its Human Knight line is the card's) — the FOR-EACH
-- pump, which is the family that falls out of the signature change with
-- no vocabulary of its own: `nForEach` has been `Times n (CountOf p)`
-- since chapter twenty, and a slot that takes an amount takes it. Three
-- hundred fifty-three supported cards write this shape. The catalog grew
-- its first artifact type, `Equipment`.
adelbertSteiner : Ability
adelbertSteiner =
  Static (Gets Macros.thisCreature
               (PtUp (Macros.forEach (And [HasSubtype Equipment, ControlledBy You])))
               (PtUp (Macros.forEach (And [HasSubtype Equipment, ControlledBy You]))))

-- "{4}{G}{G}, {T}: Create an X/X green Elemental creature token, where X
-- is the number of lands you control." (Dokai, Weaver of Life — the back
-- face of Budoka Gardener; the flip container is its own axis and the
-- front face's land-drop trigger is another ability) — the COMPUTED
-- token, which is the second field this round unlocked: the X/X token is
-- 62 supported cards and 52 of its lines ride a rider, so the two changes
-- had to land together for any of them to be writable.
dokai : Ability
dokai =
  Activated (Compound [Mana [Macros.generic 4, Macros.pip Green, Macros.pip Green],
                       TapSymbol])
            (WhereLetter LetterX (CountOf (And [Macros.land, ControlledBy You]))
                         (Macros.create (Lit 1)
                            (Macros.creatureTokOf (DefinedLetter LetterX)
                                                  (DefinedLetter LetterX)
                                                  [Green] [Elemental])))

-- ===== The static-line rider =====

-- Death's Shadow {B}, Creature — Avatar, 13/13, "This creature gets -X/-X,
-- where X is your life total." The whole card, and the sentence that
-- names the frame: the pump is a STATEMENT and not an instruction, so its
-- letter is read at every moment the statement applies rather than fixed
-- once on resolution ([CR#611.3a] against [CR#611.2d]) — which is the
-- whole of what the card does. Chapter fifty-nine's binder could not
-- reach it and chapter sixty had to demote it; this is the row it was
-- waiting for. Nothing elided.
deathsShadow : Card
deathsShadow =
  Macros.card "Death's Shadow" (Just [Macros.pip Black]) []
       (MkTypeLine [Avatar] [Creature])
       [ Static (WhereLetterStatic LetterX (PlayerStatOf LifeTotal You)
                                   (Gets Macros.thisCreature
                                         (PtDown (DefinedLetter LetterX))
                                         (PtDown (DefinedLetter LetterX)))) ]
       (Just (13, 13))

-- "Enchanted creature gets -X/-0, where X is the number of cards in your
-- graveyard." (Spontaneous Mutation's third line; its Flash and its
-- "Enchant creature" line are keyword abilities the catalog does not
-- carry) — the rider over the family's most-written SUBJECT, the
-- attachment host, and the one line where the two rounds meet: the
-- untouched slot writes the sign its partner does ("-0", chapter sixty's
-- measured spelling) while the touched one reads a letter.
spontaneousMutation : Ability
spontaneousMutation =
  Static (WhereLetterStatic LetterX (CountOf (InZone (Macros.graveyardOf You)))
                            (Gets (AttachHost Enchanted (TypeW Creature))
                                  (PtDown (DefinedLetter LetterX))
                                  (PtDown (Lit 0))))

-- Stag Beetle {3}{G}{G}, Creature — Insect, 0/0, "This creature enters
-- with X +1/+1 counters on it, where X is the number of other creatures
-- on the battlefield." The whole card, and the round's windfall: the
-- entry replacement has taken an `Amount` since it was minted, so the
-- 23 enters-with-counters rider lines needed the wrapper and nothing
-- else — the same shape chapter sixty found at the for-each pump, where
-- one signature had already been general enough for years.
stagBeetle : Card
stagBeetle =
  Macros.card "Stag Beetle"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Insect] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (CountOf (Macros.otherCreature Macros.thisCreature))
                   (EntersWithCounters Macros.thisCreature
                                       (DefinedLetter LetterX)
                                       Macros.plusOnePlusOne)) ]
       (Just (0, 0))

-- ===== The aggregate amount =====

-- Accelerated Mutation {3}{G}{G}, Instant, "Target creature gets +X/+X
-- until end of turn, where X is the greatest mana value among permanents
-- you control." The whole card, and the fold at its cleanest: an
-- instruction's rider whose right side is the new row and nothing else,
-- with the duration outside the rider where the corpus writes it
-- (`badRiderInsideDuration`).
acceleratedMutation : Card
acceleratedMutation =
  Macros.card "Accelerated Mutation"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (WhereLetter LetterX
                  (Aggregate MaxOf (CharAxis ManaValue)
                             (And [Permanent, ControlledBy You]))
                  (Continuously (Gets (Macros.target Macros.creature)
                                      (PtUp (DefinedLetter LetterX))
                                      (PtUp (DefinedLetter LetterX)))
                                (Just Macros.untilEndOfTurn))) ]
       Nothing

-- Carrion Grub {3}{B}, Creature — Insect, 0/5, "This creature gets +X/+0,
-- where X is the greatest power among creature cards in your graveyard. /
-- When this creature enters, mill four cards." The whole card, and the
-- fold at the STATEMENT frame: the letter is read at any moment the
-- statement applies ([CR#611.3a]) and the set it folds is a graveyard's,
-- which the domain describes like any other. The mill line's reminder text
-- is elided.
carrionGrub : Card
carrionGrub =
  Macros.card "Carrion Grub" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [Insect] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (Aggregate MaxOf (CharAxis Power)
                              (And [Macros.creature,
                                    InZone (Macros.graveyardOf You)]))
                   (Gets Macros.thisCreature
                         (PtUp (DefinedLetter LetterX))
                         (PtUp (Lit 0))))
       , Triggered When (Enters Macros.thisCreature) (Macros.millCards 4) ]
       (Just (0, 5))

-- Repay in Kind {5}{B}{B}, Sorcery, "Each player's life total becomes the
-- lowest life total among all players." The whole card, and the round's
-- other three claims in one sentence: the LEAST op, whose only
-- amount-position line this is; the player-sorted axis over a player-sorted
-- domain; and a fold read BARE, with no rider between it and the slot that
-- consumes it. The set recipient is the slot chapter fifty-six kept apart
-- from the distributive READ it refuses (`badDistributiveLifeTotalRead`).
-- The card prints "among all players" and this writes the bare plural, one
-- fold under the two spellings the corpus gives it (Scourge of the
-- Skyclaves writes "among players" of the same fold).
repayInKind : Card
repayInKind =
  Macros.card "Repay in Kind"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.lifeTotalBecomes (Each AnyPlayer)
                  (Aggregate MinOf (PlayerStatAxis LifeTotal) AnyPlayer)) ]
       Nothing

-- "This creature enters with X +1/+1 counters on it, where X is the total
-- toughness of other creatures you control." (Towering Titan's first line;
-- its sacrifice-activated trample grant waits on the keyword catalog) —
-- the SUM, whose preposition is "of" where the extremes write "among", at
-- the entry replacement chapter sixty-one landed free.
toweringTitan : Ability
toweringTitan =
  Static (WhereLetterStatic LetterX
            (Aggregate SumOf (CharAxis Toughness)
                       (Macros.otherCreatureYouControl Macros.thisCreature))
            (EntersWithCounters Macros.thisCreature
                                (DefinedLetter LetterX)
                                Macros.plusOnePlusOne))

-- ===== The cost-modification statement =====

-- Ghalta, Primal Hunger {10}{G}{G}, Legendary Creature — Elder Dinosaur,
-- 12/12, "This spell costs {X} less to cast, where X is the total power of
-- creatures you control." + Trample. The whole card, and the round's
-- marquee for what it composes rather than for what it adds: the
-- definition rider is chapter fifty-nine's, the statement frame it rides
-- is chapter sixty-one's, the fold on its right side is chapter
-- sixty-two's, and the only new thing in the sentence is the statement
-- itself. The reminder text on Trample is elided.
ghalta : Card
ghalta =
  Macros.card "Ghalta, Primal Hunger"
       (Just [Macros.generic 10, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [Elder, Dinosaur] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (Aggregate SumOf (CharAxis Power) Macros.creatureYouControl)
                   (CostsToCast This (CostLess (DefinedLetter LetterX))))
       , KeywordAbility Trample ]
       (Just (12, 12))

-- "This spell costs {1} less to cast for each attacking creature."
-- (Ancient Stone Idol's second line; its Flash and Trample are keyword
-- abilities and its death trigger creates a token with one of them) — the
-- affinity SHAPE ([CR#702.41a]), and the third round running in which a
-- family arrives inside a signature that was already general enough: the
-- scaled reduction is `nForEach`'s own term, unchanged, in a slot that
-- takes an amount.
ancientStoneIdol : Ability
ancientStoneIdol =
  Static (CostsToCast This
            (CostLess (Macros.forEach (And [Macros.creature, Attacking]))))

-- Thorn of Amethyst {2}, Artifact, "Noncreature spells cost {1} more to
-- cast." The whole card, and the class subject at the TAX polarity. It is
-- Thalia, Guardian of Thraben's own sentence written by an artifact, which
-- is why it is here instead: the same line without buying the Human
-- subtype or the first-strike keyword to print it (chapter fifty-seven's
-- Feudkiller swap).
thornOfAmethyst : Card
thornOfAmethyst =
  Macros.card "Thorn of Amethyst" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Not Macros.creature, Macros.spell]))
                             (CostMore (Lit 1))) ]
       Nothing

-- Feroz's Ban {6}, Artifact, "Creature spells cost {2} more to cast." The
-- whole card, and the plainest subject the family has: a type word and the
-- stack's carrier noun, which is all "creature spell" is.
ferozsBan : Card
ferozsBan =
  Macros.card "Feroz's Ban" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Macros.creature, Macros.spell]))
                             (CostMore (Lit 2))) ]
       Nothing

-- Urza's Filter {4}, Artifact, "Multicolored spells cost {2} less to
-- cast." The whole card, the class subject at the REDUCTION polarity, and
-- chapter forty-four's characteristic predicate paying off in a frame that
-- did not exist when it landed.
urzasFilter : Card
urzasFilter =
  Macros.card "Urza's Filter" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Multicolored, Macros.spell]))
                             (CostLess (Lit 2))) ]
       Nothing

-- ===== The cast relation =====

-- Emerald Medallion {2}, Artifact, "Green spells you cast cost {1} less to
-- cast." The whole card and the round's cleanest sentence: a color word,
-- the stack's carrier noun and the new clause, with nothing bought for it.
emeraldMedallion : Card
emeraldMedallion =
  Macros.card "Emerald Medallion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [ColorIs Green, Macros.spell, CastBy You]))
                             (CostLess (Lit 1))) ]
       Nothing

-- Foundry Inspector {3}, Artifact Creature — Construct, 3/2, "Artifact spells
-- you cast cost {1} less to cast." The whole card at the TYPE head, and one
-- of the encodings chapter sixty-three found live in the generated plugin.
foundryInspector : Card
foundryInspector =
  Macros.card "Foundry Inspector" (Just [Macros.generic 3]) []
       (MkTypeLine [Construct] [Artifact, Creature])
       [ Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                             (CostLess (Lit 1))) ]
       (Just (3, 2))

-- Daru Warchief {2}{W}{W}, Creature — Human Soldier, 1/1, "Soldier spells you
-- cast cost {1} less to cast. / Soldier creatures you control get +1/+2."
-- The whole card, the SUBTYPE head, and the two relations in one card: the
-- cast clause over a spell on the stack and the control clause over the
-- battlefield, which is what the widened admissibility table keeps apart.
daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Human, Soldier] [Creature])
       [ Static (CostsToCast (AllOf (And [HasSubtype Soldier, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (Gets (AllOf (And [HasSubtype Soldier, Macros.creature, ControlledBy You]))
                      (PtUp (Lit 1)) (PtUp (Lit 2))) ]
       (Just (1, 1))

-- Grand Arbiter Augustin IV {2}{W}{U}, Legendary Creature — Human Advisor,
-- 2/3, "White spells you cast cost {1} less to cast. / Blue spells you cast
-- cost {1} less to cast. / Spells your opponents cast cost {1} more to cast."
-- The whole card, both polarities, and the union possessor of chapter
-- fifty-four reading the new clause at its third table — the bare head being
-- what the opponents' line writes, God-Pharaoh's Statue's sentence exactly.
grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Human, Advisor] [Creature])
       [ Static (CostsToCast (AllOf (And [ColorIs White, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (CostsToCast (AllOf (And [ColorIs Blue, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (CostsToCast (AllOf (And [Macros.spell,
                                          CastBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 1))) ]
       (Just (2, 3))

-- ===== The union head =====

-- Goblin Electromancer {U}{R}, Creature — Goblin Wizard, 2/2, "Instant and
-- sorcery spells you cast cost {1} less to cast." The whole card, and the
-- family's icon: chapters sixty-three and sixty-four both named it as the
-- line they could not reach, and what it was waiting for turned out to be a
-- spelling correction rather than a construction — the disjunction has taken
-- a member list since chapter fourteen, and the word "and" is what the
-- coordinator writes over an affirmative plural class head.
goblinElectromancer : Card
goblinElectromancer =
  Macros.card "Goblin Electromancer"
       (Just [Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [Goblin, Wizard] [Creature])
       [ Static (CostsToCast (AllOf (And [Macros.instantOrSorcery, Macros.spell,
                                          CastBy You]))
                             (CostLess (Lit 1))) ]
       (Just (2, 2))

-- Arcane Melee {4}{U}, Enchantment, "Instant and sorcery spells cost {2} less
-- to cast." The whole card and the union head UNQUALIFIED — the subject
-- chapter sixty-three landed with eighteen lines, now written with the head
-- it could not spell.
arcaneMelee : Card
arcaneMelee =
  Macros.card "Arcane Melee" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Macros.instantOrSorcery, Macros.spell]))
                             (CostLess (Lit 2))) ]
       Nothing

-- Mana Matrix {6}, Artifact, "Instant and enchantment spells you cast cost
-- {2} less to cast." The whole card, and the pair that shows the head is not
-- instant-and-sorcery special-cased: any two type words coordinate, and the
-- coordinator is the same derived word.
manaMatrix : Card
manaMatrix =
  Macros.card "Mana Matrix" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Or [Macros.instant, Macros.enchantment],
                                          Macros.spell, CastBy You]))
                             (CostLess (Lit 2))) ]
       Nothing

-- Aura of Silence {1}{W}{W}, Enchantment, "Artifact and enchantment spells
-- your opponents cast cost {2} more to cast. / Sacrifice this enchantment:
-- Destroy target artifact or enchantment." The whole card, and the round's
-- two-line proof of the derivation: ONE disjunction term, written "and" over
-- the plural class head and "or" under the singular "target" — the second
-- line being Disenchant's own sentence, benched since chapter fourteen.
-- The first line rides chapters fifty-four, sixty-three and sixty-four
-- together (the union possessor, the cost statement, the cast relation).
auraOfSilence : Card
auraOfSilence =
  Macros.card "Aura of Silence"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Or [Macros.artifact, Macros.enchantment],
                                          Macros.spell,
                                          CastBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 2)))
       , Activated (Do (Macros.sacrifice You Macros.thisEnchantment))
                   (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                       Macros.enchantment]))) ]
       Nothing

-- Young Pyromancer {1}{R}, Creature — Human Shaman, 2/1, "Whenever you cast
-- an instant or sorcery spell, create a 1/1 red Elemental creature token."
-- The whole card, and the OVERSIGHT family's first witness: the trigger over
-- a singular indefinite union has been writable since chapter fourteen met
-- the cast event, 291 supported lines of it, and no round had asked.
youngPyromancer : Card
youngPyromancer =
  Macros.card "Young Pyromancer" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Human, Shaman] [Creature])
       [ Triggered Whenever
                   (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])))
                   (Create You (Lit 1) (Macros.creatureTok 1 1 [Red] [Elemental]) []) ]
       (Just (2, 1))
