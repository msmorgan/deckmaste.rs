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
giantGrowth = Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.untilEndOfTurn)

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
  Macros.gets (Macros.target (And [Blocking, Macros.creature, ControlledBy You])) 10 0 (Just Macros.untilEndOfCombat)

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
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare Power OrLess (Lit 2)]))

-- "Destroy target attacking creature with power 3 or less." (Terashi's
-- Verdict) — a status word and a bound in ONE phrase, which is the
-- interaction worth a positive: both members presuppose a creature and
-- the contradiction scan has to let them, positive types stacking
-- rather than clashing. The zone comes from `Attacking` alone; the
-- bound places nothing.
terashisVerdict : Effect []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare Power OrLess (Lit 3)]))

-- "Exile target creature with toughness 4 or greater." (Pillar of
-- Light) — the other comparator and the other creature-gated
-- characteristic, so the two-row vocabulary is spelled end to end by
-- real cards rather than by one card and an argument.
pillarOfLight : Effect []
pillarOfLight =
  Macros.exile (Macros.target (And [Macros.creature, Compare Toughness OrGreater (Lit 4)]))

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
                                      Compare ManaValue OrLess (Lit 3)])),
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
              (CompareAmt (Macros.manaValueOf It) OrLess (Lit 2))
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
  Sequentially [Macros.gets (Macros.target (And [Macros.creature, Attacking])) 3 3 (Just Macros.untilEndOfTurn),
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
                                 Compare Power OrGreater (Lit 4)]))
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
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [Thopter] [Artifact, Creature])
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
    (MkToken (Just (2, 1)) [] (MkTypeLine [Construct] [Artifact, Creature])
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
killiansConfidence = Sequentially [Macros.gets (Macros.target Macros.creature) 1 1 (Just Macros.untilEndOfTurn),
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
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue OrLess (Lit 3)])),
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue OrGreater (Lit 4)]))]

-- "Choose one or both — • Target creature gets -1/-1 until end of turn. •
-- Put a +1/+1 counter on target creature." (Azula Always Lies; the whole
-- card) — the one-to-two range under the word that names the whole of a
-- two-item list, and the sharpest evidence for the list-not-telescope
-- shape: BOTH modes write "target creature" and neither is the other's
-- "other", because each announces its own ([CR#700.2c,601.2c]). Written
-- as a sequence the second phrase would have to say "another".
azulaAlwaysLies : Effect []
azulaAlwaysLies =
  Macros.chooseOneOrBoth [Macros.gets (Macros.target Macros.creature) (-1) (-1) (Just Macros.untilEndOfTurn),
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
  Macros.gets (AllOf (Macros.otherCreatureYouControl Macros.thisCreature)) 1 1 (Just Macros.untilEndOfTurn)

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
                            OrGreater (Lit 3))
                Nothing)

-- "{4}, {T}: Draw a card. If you control eight or more lands, draw two
-- cards instead." (Zimone, Quandrix Prodigy) — the same frame at a
-- different domain and count, which is what makes it a frame.
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (Macros.drawCards 2)
                (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                            OrGreater (Lit 8))
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
            (Macros.gets (Macros.target Macros.creature) (-2) 0 (Just Macros.untilEndOfTurn))

-- "{1}{S}: This creature gets +1/+0 until end of turn." (Phyrexian
-- Snowcrusher) — the snow symbol ([CR#107.4h]), which is neither a color
-- nor a type of mana and is why `ManaSymbol` needs a row for it rather
-- than a colorless pip with a note.
phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Activated (Mana [Macros.generic 1, SnowMana])
            (Macros.gets Macros.thisCreature 1 0 (Just Macros.untilEndOfTurn))

-- "{1}{C}: This creature gets +2/+1 until end of turn." (Havoc Sower;
-- its Devoid line and reminder gloss elided) — the COLORLESS pip
-- ([CR#107.4c]), which is what `ColorOrColorless` was ported for: the
-- token's empty color list could say "colorless" but no color could say
-- "{C}".
havocSower : Ability
havocSower =
  Activated (Mana [Macros.generic 1, Macros.colorlessPip])
            (Macros.gets Macros.thisCreature 2 1 (Just Macros.untilEndOfTurn))

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
            (Macros.gets Macros.thisCreature 2 2 (Just Macros.untilEndOfTurn))
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
                                        Compare Power OrGreater (Lit 4)]))}

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
  Triggered Whenever (Blocks Macros.thisCreature)
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
glacialChasmCant = Static (Cant (AllOf Macros.creatureYouControl) Attack Agent)

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
                               OrGreater (Lit 3))
                   (Gets (AllOf Macros.creatureYouControl) 3 0))

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
                   (Cant Macros.thisCreature Attack Agent))

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
       [Static (Gets (AllOf Macros.creatureYouControl) 1 1)] Nothing

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
       [Activated (Mana [Macros.pip Red]) (Macros.gets Macros.thisCreature 1 0 (Just Macros.untilEndOfTurn))]
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

-- ===== Negatives (each `failing` block must NOT typecheck) =====


-- ===== Chapter nine negatives: the audit round =====















-- ===== The kind-union round: the class word places nothing =====

-- The cast event's complement is the same phrase in the same zone, so
-- it takes the same strict demand: a thousand and sixty-nine headers
-- write "casts a … spell" and not one names the damage class.
failing "OnStack"
  badCastsAnyTarget : Ability
  badCastsAnyTarget = Triggered Whenever (Casts You (Macros.target AnyTarget)) Macros.drawACard

-- ===== What a wrapper hides: buried seeds, head sets, re-minted trees =====

-- Alternatives that place their referent differently are not
-- alternatives at all: "attacking artifact" stands on the battlefield
-- and a bare "land" says nothing about where it stands. Compared
-- through [CR#109.2]'s default the two looked parallel, and the
-- disagreement came back instead as a projection of NOTHING — which
-- was enough to let `And [this, InZone graveyardZ]` stand, a phrase
-- placing its referent in two zones at once. Refused at the
-- coordination, where the disagreement is.
failing "ParallelDisjuncts"
  badPartialZoneJoin : Predicate [] Object
  badPartialZoneJoin = Or [And [Macros.artifact, Attacking], Macros.land]

-- The singular quantity licenses the class word as the phrase's HEAD,
-- not wherever it can hide: "target creature the controller of any
-- target controls" spells it in a possessor, where a counted mention
-- forbids it exactly as "a" does (`badAnyTargetEmbedded`).
failing "AnyTargetAtCount"
  badEmbeddedAnyTargetExact1 : Noun [] Object
  badEmbeddedAnyTargetExact1 =
    Macros.target (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))])

-- …and it starts at one. "Zero or more target creatures" is a second
-- spelling of "any number of target creatures" — [CR#107.1c] has "any
-- number" permit zero already — and the corpus writes only the second.
failing "WellFormedQ"
  badZeroLowerRange : Noun [] Object
  badZeroLowerRange = TargetGroup (Range (Just 0) Nothing) Macros.creature

-- A coordinated head offers one type per alternative, not none: an
-- earlier LAND anchors "another target artifact or enchantment" no
-- better than it anchors "another target artifact" (`badOtherCrossHead`).
-- Posed at a land target, the anchor being the point.
failing "OtherAnchored"
  badDisjunctiveOtherCrossHead : Predicate [MkBinding TargetD Object OneOf
                                                      (ObjectP (Just Land) (Just Battlefield) Nothing Nothing)] Object
  badDisjunctiveOtherCrossHead = And [Or [Macros.artifact, Macros.enchantment], Other]

-- "Artifact or artifact" is caught by comparing the two words; the
-- same repetition spelled with a modifier went through, the member
-- equality having declined to look inside a conjunction at all.
failing "DistinctDisjuncts"
  badRepeatedStructuredDisjunct : Predicate [] Object
  badRepeatedStructuredDisjunct =
    Or [And [Macros.creature, ControlledBy You], And [Macros.creature, ControlledBy You]]

-- A sequence's ELEMENTS are clauses. Nesting one re-mints the
-- right-nested tree the n-ary list replaced and spells a
-- three-sentence card a second way; what may still hold a sequence is
-- a clause slot taking a body, not an element position.
failing "NotSeq"
  badNestedSequence : Effect []
  badNestedSequence =
    Sequentially [Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.exile (Macros.target Macros.creature)],
                  Macros.destroy (Macros.target Macros.land)]

-- ===== What a restriction may forbid, of whom, and for how long =====

-- Only a creature can attack or block ([CR#506.3]), so a land has no
-- grant for the deed to remove. The demand is the fight slots' shape
-- over a DIFFERENT table: `combatant` answers a non-combat damage deed
-- ([CR#701.14d]), `deedType` the combat grants themselves.
failing "DeedParticipant"
  badCantAttackLand : Effect []
  badCantAttackLand = Macros.cantAttack (Macros.target Macros.land) (Just Macros.thisTurn)

-- A coordinated head fixes no type (finding 50), and an untyped head
-- cannot prove participation: "target creature or land" would have to
-- carry the attack grant on an alternative that has none. The silence
-- is not permission — `DamageableTy` learned the same lesson.
failing "DeedParticipant"
  badCantDisjunctSubject : Effect []
  badCantDisjunctSubject = Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just Macros.thisTurn)

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
  badCantBeAttacked = Continuously (Cant (Macros.target Macros.creature) Attack Patient) (Just Macros.thisTurn)

-- Combat is fought on the battlefield: a permanent that leaves it is
-- removed from combat ([CR#506.4]), so a graveyard card has no deed to
-- be denied. The gate is the one destroy, tap, and "gets" already
-- carry.
failing "ZoneFits"
  badCantInGraveyard : Effect []
  badCantInGraveyard =
    Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn)

-- The class word names [CR#115.4]'s damage class, describes no object,
-- and so places none — and the restriction needed no rule of its own to
-- refuse it, and as of chapter twenty-eight the refusal lands one
-- demand over: the zone demand is `zoneFits`' now and silence passes it,
-- so what catches the class word is the DEED's head-type demand — a
-- phrase that describes no object fixes no type, which is
-- `badCantDisjunctSubject`'s refusal reaching a second silent phrase.
failing "DeedParticipant"
  badCantAnyTarget : Effect []
  badCantAnyTarget = Macros.cantBlock (Macros.target AnyTarget) (Just Macros.thisTurn)

-- The same span, the wrong word. Two hundred ninety-six corpus lines
-- write a one-shot restriction and every one of them says "this turn";
-- "until end of turn" belongs to the grants, which invert the count
-- (`badGainsThisTurn`). Under the envelope the refusal is one table
-- read: `spanUse` calls the bare end-of-turn endpoint `BothGrants`, and
-- `admitsSpan` gives the restriction row no share of it.
failing "SpanOk DeedRestriction"
  badCantUntilEndOfTurn : Effect []
  badCantUntilEndOfTurn = Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfTurn)

-- …and the inverse, which is what keeps the new word from opening a
-- hole: no corpus line grants an ability "this turn".
failing "SpanOk KeywordGrant"
  badGainsThisTurn : Effect []
  badGainsThisTurn = Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.thisTurn)

-- The stat change writes the grant's adverbial too — "Target creature
-- gets +3/+3 until end of turn", never "this turn".
failing "SpanOk PtDelta"
  badGetsThisTurn : Effect []
  badGetsThisTurn = Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.thisTurn)

-- A durationless "can't" is the STATIC ability line ("Enchanted
-- creature can't attack", Pacifism), which is a different construction
-- and the parked ability layer's. The slot is now optional for everyone
-- — the grants need it, since the unwritten span is [CR#611.2a]'s
-- end-of-game default — so the refusal moved from the ABSENT SLOT to
-- the reason for it: `absentOk DeedRestriction = False`, the one row of
-- that table that says no.
failing "SpanOk DeedRestriction"
  badStaticCant : Effect []
  badStaticCant = Macros.cantBlock (Macros.target Macros.creature) Nothing

-- The upkeep endpoint belongs to the KEYWORD grant alone. Two corpus
-- lines write "until your next upkeep" and both grant an ability
-- (Gabriel Angelfire, and one forestwalk line); not one stat change
-- takes it. The decomposition is what made the cell writable at all —
-- and the table is what keeps the two grants from sharing it just
-- because they share the other three.
failing "SpanOk PtDelta"
  badGetsUntilYourNextUpkeep : Effect []
  badGetsUntilYourNextUpkeep = Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.untilYourNextUpkeep)

-- Real oracle English, no clause of ours: "until the end of your next
-- turn" runs to eighty-three lines, and every one of them is a play
-- permission, a control grant, or a can't-cast — never a keyword grant.
-- The `Unclaimed` row exists to say exactly this, and to keep it apart
-- from the endpoints nothing writes at all (`badGainsUntilUntapStep`).
failing "SpanOk KeywordGrant"
  badGainsUntilEndOfYourNextTurn : Effect []
  badGainsUntilEndOfYourNextTurn =
    Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just (Until (EndOf Turn (Just Yours))))

-- The combat endpoint is the grants' too — Glyph of Destruction's
-- "+10/+0" and one banding line — and no corpus line ends a single-deed
-- restriction there. The restriction's two words stay "this turn" and
-- "until your next turn".
failing "SpanOk DeedRestriction"
  badCantUntilEndOfCombat : Effect []
  badCantUntilEndOfCombat = Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfCombat)

-- And the unattested end of the table: no corpus line ends a duration
-- at an untap step in words this vocabulary has. The one line that ends
-- one there possesses it with a NOUN — "until its controller's next
-- untap step" — which is neither of `Whose`'s two words, and its clause
-- (a base-type setting) is a layer word with no construction here
-- either, so the row waits on both.
failing "SpanOk KeywordGrant"
  badGainsUntilUntapStep : Effect []
  badGainsUntilUntapStep =
    Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying)
          (Just (Until (StartOf UntapStep (Just Yours))))

-- ===== What carries a bound, and how it is written =====

-- A bound is a MODIFIER: "with power 2 or less" describes something
-- and names nothing, so it cannot be what a determiner determines.
-- The same refusal "attacking" and "other" already take.
failing "Headed"
  badBareComparison : Noun [] Object
  badBareComparison = Macros.target (Compare Power OrLess (Lit 2))

-- Only a creature has power ([CR#208.3] — a noncreature permanent has
-- none, and a noncreature object off the battlefield has one only if
-- it is printed there), so a phrase that bounds power and denies the
-- type describes nothing. The presupposition rides the bound exactly
-- as it rides "attacking" (`badAttackingNoncreature`), the type table
-- supplying the answer in place of a fixed row.
failing "ContradictionFree"
  badNoncreaturePower : Predicate [] Object
  badNoncreaturePower = And [Compare Power OrLess (Lit 2), Not Macros.creature]

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
  badDoubleComparison = And [Macros.creature, Compare Power OrLess (Lit 2),
                             Compare Power OrGreater (Lit 4)]

-- The bound is WRITTEN — a numeral or the announced X — and a phrasal
-- standard belongs to the other frame. Oracle writes "with power less
-- than this creature's power", never "with this creature's power or
-- less": admitting the amount here would spell a real comparison in a
-- word order the language does not use, so the frame is refused and
-- the comparison-to-a-phrase family waits on the ledger.
failing "WrittenBound"
  badPhrasalBound : Predicate [] Object
  badPhrasalBound = Compare Power OrLess (Macros.powerOf This)

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

-- ===== What may be tested, and what a test leaves behind =====

-- A condition quantifies over a DESCRIPTION, so the phrase inside it
-- has to name something: "if you control attacking" heads nothing, the
-- same refusal the indefinite article and the for-each domain already
-- make. The existence row needed no rule of its own — it re-keys onto
-- the head projection the noun layer computes.
failing "Headed"
  badExistsUnheaded : Condition []
  badExistsUnheaded = Exists Attacking

-- The class word NAMES [CR#115.4]'s damage class and describes no
-- object, so there is nothing for an existential to be true of. Third
-- consumer of the same gate (the article and the for-each domain being
-- the first two), and it cost one hypothesis rather than a rule.
failing "AnyTargetFree"
  badExistsAnyTarget : Condition []
  badExistsAnyTarget = Exists AnyTarget

-- Nor inside the reference-matches frame: "if it's any target" would
-- test a phrase that fixes the class by rule rather than describing the
-- referent.
failing "AnyTargetFree"
  badMatchesAnyTarget : Effect []
  badMatchesAnyTarget = If (Macros.destroy (Macros.target Macros.artifact)) (Macros.itsA AnyTarget) Nothing

-- The condition's SUBJECT is a read and never a mention. This is the
-- refusal that keeps `condDelta`'s opacity honest instead of merely
-- convenient: "If target creature has toughness 5 or greater, it gets
-- +4/-4 until end of turn" (Blood Lust) announces a target inside the
-- if-clause that the consequent then reads, which [CR#601.2c] makes
-- legitimate — targets are announced as the spell is cast, whatever
-- clause spells them — and which this grammar cannot represent without
-- letting conditions bind. Ten corpus lines write "if target …"; the
-- family is refused whole rather than mis-bound, and waits on the
-- ledger.
failing "Bindingless"
  badMatchesTargetSubject : Condition []
  badMatchesTargetSubject = Matches (Macros.target Macros.creature) Macros.artifact

-- A description that says NOTHING tests nothing. The copular frame does
-- not demand a HEAD — "if it's attacking" and "if it's tapped" are real
-- oracle and head nothing — so the empty conjunction slipped past the
-- gate the existential uses (`Headed`) and had to be refused by the
-- weaker one instead (`predSays`). Every corpus line writes at least one
-- word after the copula.
failing "PredSays"
  badMatchesNothing : Effect []
  badMatchesNothing =
    Sequentially [Tap (Macros.target Macros.creature),
                  If (Macros.gainsLife You (Lit 1)) (Matches It (And [])) Nothing]

-- A condition negates a POSITIVE description. "If it isn't a
-- non-artifact" is a negation of a negation, which the predicate layer
-- already refuses of itself (`negatable (Not _) = False`,
-- `badDoubleNegation`) and which the condition frame could launder by
-- taking its "not" of an already-negative phrase. Corpus writes the
-- single negation everywhere ([CR#701.47a]'s "If it isn't a [subtype]",
-- `amassZombiesTwo`) and the doubled one nowhere.
failing "CondNegatable"
  badNegatedNegativeMatch : Effect []
  badNegatedNegativeMatch =
    Sequentially [Tap (Macros.target Macros.creature),
                  If (Macros.gainsLife You (Lit 1))
                     (Macros.notSo (Matches It (Not Macros.artifact)))
                     Nothing]

-- A trailing condition is EVALUATED before the clause it modifies and
-- WRITTEN after it, and reading it in the clause's post-state let it ask
-- about a world the clause had not made: "Destroy target creature if
-- it's in a graveyard" typechecked because the destroy had already
-- retagged its own target. Chapter twenty-one types the condition in
-- `preIntro` — what the clause's phrases ANNOUNCED, with the announced
-- zone — so the subject here is on the battlefield, and `ZoneFits`
-- refuses the description that puts it elsewhere ([CR#109.2a]). Overload
-- is unaffected: mana value belongs to every object [CR#202.3] and is
-- read zone-free.
failing "ZoneFits"
  badTrailingPostStateZone : Effect []
  badTrailingPostStateZone =
    If (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ)) Nothing

-- A written comparison measures a READ against a written value, and a
-- numeral is not a read: "if 3 is 4 or greater" states an arithmetic
-- fact, not a fact about the game. The subject table is `writtenBound`'s
-- exact complement, and this is the cell where they differ most
-- visibly.
failing "ReadAmount"
  badCompareLiteralSubject : Condition []
  badCompareLiteralSubject = CompareAmt (Lit 3) OrGreater (Lit 4)

-- The announced X is a written value too ([CR#107.3a]) — the OTHER
-- amount `writtenBound` says yes to — so it is a bound and never a
-- subject. No corpus line compares a bare X against a numeral; where X
-- is tested at all, what is measured is the phrase X was defined from.
failing "ReadAmount"
  badCompareXSubject : Condition []
  badCompareXSubject = CompareAmt XVal OrGreater (Lit 4)

-- And the bound stays written on this side of the frame as well. The
-- phrasal standard ("less than or equal to the number of Islands you
-- control") is real oracle English and a real comparison, and it is
-- still the other frame's — with its own word order and its own
-- comparator words — so the condition frame refuses it exactly as the
-- postnominal qualifier does (`badPhrasalBound`, chapter sixteen).
failing "WrittenBound"
  badConditionPhrasalBound : Condition []
  badConditionPhrasalBound =
    CompareAmt (CountOf Macros.creatureYouControl) OrGreater (Macros.powerOf This)

-- A condition introduces nothing. The mention written inside one is
-- reachable while the condition is being written — that is the
-- telescope — but the clause that follows the conditional cannot read
-- it, because the condition may have been false and then there was no
-- such creature to speak of. `predDelta (Or _) = []` at clause level,
-- and refused by the pronoun's own uniqueness gate rather than by a
-- rule about conditions.
failing "countOnes"
  badConditionAntecedent : Effect []
  badConditionAntecedent =
    Sequentially [If (Macros.gainsLife You (Lit 2)) (Exists Macros.creatureYouControl) Nothing,
                  Tap It]

-- The two branches of a may are not two spellings of one arm. The
-- if-you-DON'T arm runs exactly when the body did not, so the body's
-- phrase never named anything: "You may sacrifice a creature. If you
-- don't, exile it." is unwritable, and the corpus writes no such line —
-- every declined arm read this pass reaches the decider or the
-- sentences before the may. The taken arm has the opposite type and
-- reads the body in full (`darettisMinusOne`).
failing "countOnes"
  badIfNotReadsMayBody : Effect []
  badIfNotReadsMayBody = Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile It)

-- ===== What a token may be, and what a counter may sit on =====

-- A subtype sits on one card type's own set ([CR#205.1a] names the six
-- sets; [CR#205.3m] makes every subtype here a creature type), so a
-- token whose line names
-- a subtype its types cannot carry describes nothing: a "Zombie artifact
-- token" writes a creature type onto an object with no creature type.
failing "SubtypesFit"
  badZombieArtifactToken : Effect []
  badZombieArtifactToken =
    Macros.create (Lit 1) (MkToken (Just (1, 1)) [Black] (MkTypeLine [Zombie] [Artifact])
                            [] Nothing)

-- A token has only the characteristics its creating effect defines
-- ([CR#111.3]), so a creature token's power and toughness ([CR#208.1])
-- have to be written — and every corpus creature-token line writes them.
-- (The converse is deliberately NOT demanded: a Vehicle token carries a
-- P/T with no creature type, [CR#301.7].)
failing "TokenPt"
  badCreatureTokenNoPt : Effect []
  badCreatureTokenNoPt =
    Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [Soldier] [Creature]) [] Nothing)

-- A token is a PERMANENT ([CR#111.1]), so its line names at least one
-- card type. The type-less spelling is the predefined name ("create a
-- Treasure token", [CR#111.10]) — core's own separate `TokenSpec::Named`
-- row — and it waits with that catalog. (Probed with no subtype either,
-- so the refusal is the missing type and not a subtype with nowhere to
-- sit.)
failing "TokenTyped"
  badTypelessToken : Effect []
  badTypelessToken =
    Macros.create (Lit 1) (MkToken (Just (1, 1)) [White] (MkTypeLine [] []) [] Nothing)

-- The arrival riders are not a free product: sixty-six corpus lines
-- create a token "tapped and attacking" and a hundred fifty-nine write
-- the prenominal "a tapped … token", while ATTACKING WITHOUT TAPPED is
-- written zero times. The rules are why the pair is written out rather
-- than derived: [CR#508.4] gives the attacking designation to a creature
-- put onto the battlefield attacking and taps nothing — the tap belongs
-- to the declare-attackers turn-based action ([CR#508.1f]), which such a
-- creature never went through — so a writer who wants both has to say
-- both, and every writer does.
failing "RidersOk"
  badAttackingUntapped : Effect []
  badAttackingUntapped =
    Create You (Lit 1) (Macros.creatureTok 1 1 [Red] [Soldier]) [EntersAttacking]

-- The GRAVEYARD is the counter table's measured silence and stays shut
-- with the zone gate widened: all seventeen lines writing "counter" and
-- "in a graveyard" together put the counter on a battlefield object and
-- read the graveyard for a count, or return the card to the battlefield
-- first ("Return this card from your graveyard to the battlefield with a
-- finality counter on it").
failing "CounterHolder"
  badPutCountersGraveyard : Effect []
  badPutCountersGraveyard =
    PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))

-- …and the removal twin reads the same fold-state: a destroyed referent
-- has no counters to take off. The destination is what decides — the
-- same clause with an EXILE in front of it is Jhoira of the Ghitu.
failing "CounterHolder"
  badRemoveCountersDead : Effect []
  badRemoveCountersDead = Sequentially [Macros.destroy (Macros.target Macros.creature),
                                        RemoveCounters (Lit 1) Macros.plusOnePlusOne It]

-- A token's stated characteristics ARE its text ([CR#111.3]), so the
-- bundle is a surface phrase and not a set of facts about an object: a
-- color written twice is a word written twice, and no corpus line writes
-- one. The order is likewise the phrase's: measured over supported
-- oracle, "artifact creature" runs five hundred ninety-four lines to
-- "creature artifact"'s none, and the four type words this vocabulary
-- has fall into one total order (`typeRank`). Colors take the
-- duplicate demand and NOT the ordering one, because Additive Evolution
-- writes "a 0/0 green and blue Fractal creature token" and the mana
-- order would have spelled it the other way round.
failing "TokenCanonical"
  badTokenTypeOrder : Effect []
  badTokenTypeOrder =
    Macros.create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [] [Creature, Artifact])
                            [] Nothing)

failing "TokenCanonical"
  badTokenDuplicateColor : Effect []
  badTokenDuplicateColor = Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [Soldier])

-- A written action count is at least one. [CR#121.1] makes a draw the
-- movement of a card, [CR#111.1] a token a marker put onto the
-- battlefield, [CR#122.1] a counter a marker placed on something, and a
-- zero of any of them instructs nothing — which is why no corpus line
-- spells one, as a numeral or as a determiner, in any scope. What stays
-- writable is the count that EVALUATES to zero: X is announced zero
-- (its controller's to choose and announce, [CR#107.3a]) and a for-each
-- domain can be empty, so only the literal
-- spelling is refused (`writtenCount`).
failing "WrittenCount"
  badDrawZero : Effect []
  badDrawZero = Macros.drawCards 0

failing "WrittenCount"
  badCreateZero : Effect []
  badCreateZero = Macros.create (Lit 0) (Macros.creatureTok 1 1 [White] [Soldier])

failing "WrittenCount"
  badPutZeroCounters : Effect []
  badPutZeroCounters = PutCounters (Lit 0) Macros.plusOnePlusOne (Macros.target Macros.creature)

-- ===== What a type addition may add, and for how long =====

-- A creature subtype has nowhere to sit on a land ([CR#205.1a] — a
-- subtype correlated with a card type the object doesn't have is not one
-- of its subtypes), so the added line has to name the card type itself,
-- as "becomes a Spirit artifact creature" does, or find it on the
-- subject, as amass's "it becomes a Zombie" does of an Army creature
-- ([CR#701.47a]).
failing "AddedFits"
  badBecomesZombieLand : Effect []
  badBecomesZombieLand = Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [Zombie]) Nothing

-- "Becomes in addition to its other types" has to say WHAT: an empty
-- type line adds nothing and spells no phrase.
failing "LineNonEmpty"
  badBecomesNothing : Effect []
  badBecomesNothing = Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing

-- A type change is a continuous effect on a permanent: a graveyard card
-- has no types for the clause to add to on the battlefield.
failing "ZoneFits"
  badBecomesInGraveyard : Effect []
  badBecomesInGraveyard =
    Macros.becomes (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Macros.typesOnly [Artifact]) Nothing

-- The combat endpoint stays the GRANTS' alone. Chapter seventeen could
-- not tell "until end of turn" and "until end of combat" apart, both
-- being written by the stat delta and the keyword grant and by neither
-- restriction; the type addition tells them apart, writing eighteen
-- end-of-turn lines and no end-of-combat line at all — which is why
-- `BothGrants` split and `GrantsAndTypes` exists.
failing "SpanOk TypeAddition"
  badBecomesUntilEndOfCombat : Effect []
  badBecomesUntilEndOfCombat =
    Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfCombat)

-- …and the restriction's current-turn word is not the type addition's
-- either. Coward // Killer writes both adverbials in one sentence and
-- gives "this turn" to the "can't block" half, which is the division
-- stated by a card rather than by a count.
failing "SpanOk TypeAddition"
  badBecomesThisTurn : Effect []
  badBecomesThisTurn = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.thisTurn)

-- …and the upkeep endpoint stays the keyword grant's alone, as it was
-- against the stat delta (`badGetsUntilYourNextUpkeep`): two corpus
-- lines write "until your next upkeep" and both grant an ability.
failing "SpanOk TypeAddition"
  badBecomesUntilYourNextUpkeep : Effect []
  badBecomesUntilYourNextUpkeep =
    Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilYourNextUpkeep)

-- A subtype word PRESUPPOSES its set's card type ([CR#205.1a]), so "an
-- Army that isn't a creature" describes nothing — the finding-43 shape
-- again, the presupposition riding the word and the existing coherence
-- gate supplying the refusal. What it does NOT do is project that type,
-- which is what keeps `clavilenoPhrase` writable.
failing "ContradictionFree"
  badZombieNoncreature : Predicate [] Object
  badZombieNoncreature = And [HasSubtype Zombie, Not Macros.creature]

-- "In addition to its other types" RETAINS what the object had and
-- states what it gains ([CR#205.1b]), so a clause that states only what
-- its subject already is states nothing at all. Tezzeret's adds creature
-- to an ARTIFACT and Neurok Transmuter's adds artifact to a CREATURE;
-- no line adds a type to a subject that already heads it. Subtypes are
-- never provably redundant here — no mention carries its subtypes — and
-- [CR#701.47a] guards that case in the text instead, with a condition
-- ("If it isn't a [subtype], …") rather than a grammar rule.
failing "AddsSomething"
  badBecomesOwnType : Effect []
  badBecomesOwnType = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing

-- No line writes a NAKED type addition across turns. Chapter nineteen
-- opened the cell on one apparent witness and flagged it for
-- re-measurement; the re-measurement is chapter twenty-one's and closes
-- it. Two hundred sixty-four supported lines write "in addition to
-- its/their/his/her other types" and exactly three carry "until your
-- next turn": Rootwise Survivor's duration belongs to the separate haste
-- grant in the next sentence, Absorbing Man's clause is a copy
-- construction, and Tezzeret, Cruel Machinist's "becomes a 5/5 creature
-- in addition to its other types" fuses a base power/toughness setting
-- onto the addition — the compound construction this vocabulary has no
-- word for, and which waits on the ledger.
failing "SpanOk"
  badTypeAdditionAcrossTurns : Effect []
  badTypeAdditionAcrossTurns =
    Macros.becomes (Macros.target (And [Macros.artifact, ControlledBy You]))
            (Macros.typesOnly [Creature])
            (Just Macros.untilYourNextTurn)

-- ===== What an else arm may read =====

-- The "Otherwise" arm runs exactly when the condition was false, so the
-- clause it replaces never happened and its phrase never named anything:
-- an arm that reads the if-arm's token is unwritable, which is `May`'s
-- declined-arm asymmetry (`badIfNotReadsMayBody`) one construction over.
-- The corpus writes no such line either — every else arm read this pass
-- reaches the sentences before the conditional or nothing at all.
failing "countOnes"
  badOtherwiseReadsIfArm : Effect []
  badOtherwiseReadsIfArm = If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [Zombie]))
                              (Exists Macros.creatureYouControl)
                              (Just (Tap It))

-- ===== What a mode may read, what a headcount may fix, and what a draw leaves =====

-- A modal offers TWO OR MORE options ([CR#700.2] says so in its own
-- definition), so a one-mode "modal" is not one — it is the sentence
-- itself with a choice clause bolted on front, and no card writes it.
failing "AtLeastTwo"
  badModalOneMode : Effect []
  badModalOneMode = Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)]

-- A headcount that fixes the whole list instructs no choice: "Choose two
-- —" over exactly two modes has one answer, and [CR#700.2] calls a spell
-- modal for the INSTRUCTIONS to choose. Zero cards write it — every
-- printed exact headcount is strictly under its list, and the two forms
-- whose top reaches the list ("one or both", "one or more") are ranges.
failing "ModesFit"
  badModalFixedWhole : Effect []
  badModalFixedWhole = Macros.chooseTwo [Macros.destroy (Macros.target Macros.artifact),
                                  Macros.destroy (Macros.target Macros.enchantment)]

-- Nor may a headcount reach PAST the list: three of two options names
-- nothing at all.
failing "ModesFit"
  badModalOverreach : Effect []
  badModalOverreach = Modal (Macros.exactly 3) [Macros.destroy (Macros.target Macros.artifact),
                                         Macros.destroy (Macros.target Macros.enchantment)]

-- The mode list is a LIST and not a telescope: the modes are chosen at
-- cast ([CR#700.2a]) and an unchosen one's targets are never announced
-- ([CR#700.2c] — the spell is "treated as though it did not have those
-- targets"), so a mode that reads a sibling's mention reads something
-- that may never have existed. Every bullet in the corpus
-- that opens with a pronoun reaches PAST the modal to the trigger before
-- it — Kogla and Yidaro's two modes both read the same outside antecedent
-- and neither reads the other — and zero read a sibling.
failing "countOnes"
  badModalReadsAcrossModes : Effect []
  badModalReadsAcrossModes = Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Tap It]

-- And nothing after the modal reads into it, for the same reason pointed
-- forward: Blood on the Snow writes "Then return a creature or
-- planeswalker card … from your graveyard" — a description covering both
-- modes' outcomes — exactly where an anaphor would have gone.
failing "countOnes"
  badReadsAfterModal : Effect []
  badReadsAfterModal =
    Sequentially [Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                  Tap It]

-- WHICH condition frames take a "not" is a closed table. The comparison
-- frame does not: a bound has a negative of its own, and English writes
-- that instead — "if its power isn't 4 or greater" is written zero times.
failing "CondNegatable"
  badNegatedComparison : Effect []
  badNegatedComparison =
    If (Macros.destroy (Macros.target Macros.artifact))
       (Macros.notSo (CompareAmt (Macros.manaValueOf It) OrLess (Lit 2)))
       Nothing

-- Nor does a negation take one: no corpus line writes a condition under
-- two of them, English collapsing the pair into the positive.
failing "CondNegatable"
  badDoubleNegatedCondition : Effect []
  badDoubleNegatedCondition =
    If (Macros.destroy (Macros.target Macros.artifact))
       (Macros.notSo (Macros.notSo (Exists Macros.creatureYouControl)))
       Nothing

-- A drawn card is not a mention. The corpus never reads one back across a
-- sentence boundary — "Draw a card." followed by "it" or "that card" is
-- written zero times, and the card IS read only inside the coordination
-- that reveals it ("Draw a card and reveal it. If it isn't a land card,
-- discard it."), a verb-phrase coordination this grammar does not spell.
failing "countWord"
  badDrawnCardRemention : Effect []
  badDrawnCardRemention = Sequentially [Macros.drawACard, Macros.exile (That CardW)]

-- The modal headcount vocabulary is CLOSED over what oracle writes, not
-- over what the range algebra permits. "Choose up to two —" and "Choose
-- up to three —" are written zero times in any scope, where "Choose up
-- to one —" heads five lines, so the "up to" head is capped at one and
-- the range that would spell the others is refused (`modalHead`). The
-- fixed counts stop at three (Mishra, Eminent One) and the two open tops
-- take their maximum from the list, which is what "or both" and "or
-- more" say.
failing "ModalHead"
  badModalUpToTwo : Effect []
  badModalUpToTwo = Modal (Macros.upTo 2) [Macros.destroy (Macros.target Macros.artifact),
                                    Macros.destroy (Macros.target Macros.enchantment),
                                    Macros.drawACard]

-- Two identical modes are one mode written twice, and the choice between
-- them decides nothing — [CR#700.2] wants "instructions for a player to
-- choose a number of those options", and [CR#700.2d] has a player
-- normally unable to "choose the same mode more than once", the cards
-- that lift it saying so in words rather than by printing the bullet
-- twice. The check is structural and conservative (`effEq`, `predEq`'s
-- discipline one layer up): it catches the degenerate repetition, not
-- every semantic twin.
failing "distinctModes"
  badDuplicateModes : Effect []
  badDuplicateModes = Macros.chooseOne [Macros.drawACard, Macros.drawACard]

-- ===== What a choice clause may select =====

-- A choice BINDS a new referent out of a described set, so the phrase
-- has to describe one. Corpus writes "choose a/an …", "choose target …",
-- "choose two …", "choose up to …", "choose any number of …", "choose
-- another …" — selections, every one — and writes "choose you", "choose
-- it", and "choose them" zero times each. Choosing an already definite
-- participant selects among nothing and would announce a mention the
-- clause did not bind, which is chapter twenty's introduction discipline
-- asked of the choice clause (`choosable`). "Choose a player" and
-- "Choose an opponent" are unaffected — the gate is about the
-- determiner, not the kind (`myrkulsEdict`).
failing "Choosable"
  badChooseYou : Effect []
  badChooseYou = Choose You

-- ===== What a branch arm leaves behind =====

-- A conditioned clause is a HOLE: the condition may be false, and then
-- the clause never ran and its phrase named nothing. So the token
-- "create a 1/1 white Soldier creature token if you control a creature"
-- may make cannot be the "it" of the sentence after the conditional —
-- which is the else arm's rule (`badOtherwiseReadsIfArm`) and the
-- declined may's (`badIfNotReadsMayBody`) read from outside instead of
-- from inside. Oracle writes the re-binding rather than the anaphor
-- where it means to reach the object: amass's second sentence chooses an
-- Army rather than saying "it" of the token its first sentence may have
-- created ([CR#701.47a], `amassZombiesTwo`).
failing "countOnes"
  badConditionalArmAntecedent : Effect []
  badConditionalArmAntecedent =
    Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
                     (Exists Macros.creatureYouControl)
                     Nothing,
                  PutCounters (Lit 1) Macros.plusOnePlusOne It]

-- With BOTH arms written, the may exports its BODY and neither arm.
-- [CR#118.12] says why: the branch checks "whether the player chose to
-- pay an optional cost … regardless of what events actually occurred",
-- so exactly one arm ran and the sentences after the may cannot know
-- which — reading the if-you-do arm's token here is reading one branch
-- as though it were both. Crovax the Cursed is the positive that writes
-- the pair (`crovaxTheCursed`), and it reads neither arm afterward.
failing "countOnes"
  badBothArmsAntecedent : Effect []
  badBothArmsAntecedent =
    Sequentially [May (Just You) (Macros.gainsLife You (Lit 1))
                       (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])))
                       (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier]))),
                  PutCounters (Lit 1) Macros.plusOnePlusOne It]



-- ===== What a complement may exclude, and what a batch may read =====

-- The complement's anchor may not be an INDEFINITE: "each other
-- creature" announces one phrase, and "other than a creature" would
-- announce a second referent the sentence never spelled. Unwritten
-- English, and it stays refused.
failing "ComplementAnchor"
  badComplementAnchorAnnounces : Predicate [] Object
  badComplementAnchorAnnounces = OtherThan (Macros.a Macros.creature)

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
         (MkToken (Just (5, 5)) [Red] (MkTypeLine [Dragon] [Creature])
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
failing "ComplementAnchor"
  badPluralComplementAnchor : Predicate [] Object
  badPluralComplementAnchor = OtherThan (TargetGroup (Macros.upTo 2) Macros.creature)

-- The anchor has to be something the phrase could have described:
-- "each other creature" anchored to a LAND subtracts nothing, and no
-- corpus line pairs "other" with a cross-head anchor — the same
-- evidence `anyTargetedTy` reads for the bare word.
failing "OtherAnchored"
  badComplementCrossHead : Effect []
  badComplementCrossHead =
    DealDamage This (Lit 1) (Each (And [Macros.creature, OtherThan Macros.thisLand]))

-- One selector slot per phrase, whichever spelling fills it: two
-- complements are two "other"s, and the guide gives the word one
-- position.
failing "OtherAnchored"
  badDoubleComplement : Effect []
  badDoubleComplement =
    DealDamage This (Lit 1)
               (Each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature]))

-- And the two spellings share that slot: a phrase cannot write the bare
-- "other" and an anchored one at once.
failing "OtherAnchored"
  badOtherAndComplement : Effect []
  badOtherAndComplement =
    Sequentially [Tap (Macros.target Macros.creature),
                  DealDamage This (Lit 1)
                             (Each (And [Macros.creature, Other, OtherThan Macros.thisCreature]))]

-- A word that fills one phrase-level slot is not an ALTERNATIVE, which
-- is `badOtherInOr`'s refusal reaching the second spelling too.
failing "CoordinableDisjuncts"
  badComplementInOr : Predicate [] Object
  badComplementInOr = Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land]

-- "Non-other" is unwritten, as "non-other" always was.
failing "Negatable"
  badNegatedComplement : Predicate [] Object
  badNegatedComplement = Not (OtherThan Macros.thisCreature)

-- A batch is at least TWO parts, the arity demand `Sequentially` makes
-- and for its reasons: nothing at all, and a second spelling of one
-- clause.
failing "AtLeastTwo"
  badEmptySimultaneous : Effect []
  badEmptySimultaneous = Simultaneously []

failing "AtLeastTwo"
  badSingletonSimultaneous : Effect []
  badSingletonSimultaneous = Simultaneously [Macros.destroy (Macros.target Macros.creature)]

-- Its elements are clauses and not batches — the re-minted tree
-- `NotSeq` refuses one construction over.
failing "NotSim"
  badNestedSimultaneous : Effect []
  badNestedSimultaneous =
    Simultaneously [Simultaneously [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)],
                    Macros.destroy (Macros.target Macros.land)]

-- Nor SEQUENCES: an ordered list inside an unordered one contradicts the
-- container it sits in, and its announcements would reach the next
-- element only from its last clause besides.
failing "NotSeq"
  badSequenceInsideSimultaneous : Effect []
  badSequenceInsideSimultaneous =
    Simultaneously [Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)],
                    Macros.destroy (Macros.target Macros.land)]

-- What a batch's elements share is ONE pre-state — [CR#608.2f]'s rule
-- for one instruction spread over several objects, whose own example is
-- a control grant (Blatant Thievery gains control of every target
-- "simultaneously") — so an element may read what a sibling ANNOUNCED
-- and nothing a sibling DID.
-- The zone retag is the sharp case: after an exile the card word
-- reaches the referent, and inside the batch it does not, because
-- nothing has been exiled yet.
failing "countWord"
  badSimultaneousReadsRetag : Effect []
  badSimultaneousReadsRetag =
    Simultaneously [Macros.exile (Macros.target Macros.creature), Macros.destroy (That CardW)]

-- The event OUTCOME is the same fact for magnitudes: no damage has been
-- dealt when a sibling is typed, so "that much" reads nothing.
failing "countOnes"
  badSimultaneousReadsOutcome : Effect []
  badSimultaneousReadsOutcome =
    Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                    Macros.gainsLife You ThatMuch]

-- The MAY is the same fact through a wrapper, and chapter twenty-six's
-- headline: the batch's telescope threaded `preIntro`, which is not the
-- announcement-only function it was taken for — `preIntro (May …)` is
-- `mayIntro`, the optional clause's DEED. So a sibling could put a
-- counter on a token the player may not have made. [CR#118.12] makes
-- the offer a cost checked by whether the player chose to pay it,
-- "regardless of what events actually occurred", and [CR#608.2f]
-- processes the batch's actions at once, so nothing in the batch's one
-- pre-state can be the token. The telescope threads `annIntro` now.
failing "countOnes Object"
  badSimultaneousReadsMayDeed : Effect []
  badSimultaneousReadsMayDeed =
    Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant])),
                    PutCounters (Lit 1) Macros.plusOnePlusOne It]

-- …and the magnitude twin, which is `badSimultaneousReadsOutcome`
-- reached through the same wrapper.
failing "countOnes Outcome"
  badSimultaneousReadsMayOutcome : Effect []
  badSimultaneousReadsMayOutcome =
    Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                    Macros.gainsLife You ThatMuch]

-- OUTWARD, a batch leaves behind EVERY element's deed and not the last
-- one's: two creates in one instruction leave two tokens ([CR#608.2f]
-- processes both actions), so the sentence after cannot say "it".
-- Chapter twenty-two folded the last element's `effIntro` and recorded
-- the simplification; this is the row that shows it was one
-- (`simIntro`, `deedDelta`).
failing "countOnes Object"
  badBatchTwoCreatesThenIt : Effect []
  badBatchTwoCreatesThenIt =
    Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]),
                                 Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])],
                  PutCounters (Lit 1) Macros.plusOnePlusOne It]

-- The magnitude twin outward: two outcomes in one batch, and "that
-- much" does not say which.
failing "countOnes Outcome"
  badBatchTwoOutcomesThenThatMuch : Effect []
  badBatchTwoOutcomesThenThatMuch =
    Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                                 Macros.losesLife (Macros.target Opponent) (Lit 3)],
                  Macros.gainsLife You ThatMuch]

-- Control is a PERMANENT's ([CR#110.2] gives every permanent a
-- controller; [CR#109.4] gives an object that is neither on the stack
-- nor on the battlefield none), so the grant takes a battlefield
-- referent like every other continuous clause.
failing "ZoneFits"
  badGainControlGraveyard : Effect []
  badGainControlGraveyard =
    Macros.gainControl (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing

-- The control grant writes the GRANTS' current-turn word and never the
-- restrictions': "gain control of target creature this turn" is written
-- zero times, and the four corpus lines pairing the two words are event
-- clauses ("that attacked you this turn") rather than adverbials.
failing "SpanOk"
  badGainControlThisTurn : Effect []
  badGainControlThisTurn = Macros.gainControl (Macros.target Macros.creature) (Just Macros.thisTurn)

-- "Until the end of your next turn" is the control grant's ALONE — the
-- cell chapter seventeen left `Unclaimed` and this chapter claimed. The
-- keyword grant does not write it (the eighty-three lines beside the
-- four control ones are play permissions, another construction's).
failing "SpanOk"
  badKeywordGrantEndOfNextTurn : Effect []
  badKeywordGrantEndOfNextTurn =
    Macros.gainsHaste (Macros.target Macros.creature) (Just (Until (EndOf Turn (Just Yours))))

-- And "for as long as" is written by every construction but the type
-- ADDITION: the thirteen "becomes … for as long as" corpus lines are
-- type SETTINGS and copies, and the addition's own phrase pairs with
-- the adverbial zero times.
failing "SpanOk"
  badTypeAdditionForAsLongAs : Effect []
  badTypeAdditionForAsLongAs =
    Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact])
            (Just (ForAsLongAs (Matches Macros.thisCreature (ControlledBy You))))

-- A distributed creation exports a PLURAL mention, so the singular
-- pronoun has nothing to resolve to: "Each player creates a 1/1 green
-- Plant creature token. Put a +1/+1 counter on IT" is unwritable, and it
-- is the same clause that reads back fine undistributed (Additive
-- Evolution's "create a 0/0 … Fractal creature token. Put three +1/+1
-- counters on it."). The count is one in both.
failing "countOnes"
  badDistributedCreationIt : Effect []
  badDistributedCreationIt =
    Sequentially [Create (Each AnyPlayer) (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]) [],
                  PutCounters (Lit 1) Macros.plusOnePlusOne It]


-- …and it is a group MENTION and not a description: "each of each
-- creature" is written zero times, the plain distributive being what a
-- description takes.
failing "GroupMention"
  badEachOfDistributive : Noun [] Object
  badEachOfDistributive = EachOf (Each Macros.creature)

-- …nor the universal, for the same reason and with the same count:
-- "each of all creatures" is written zero times.
failing "GroupMention"
  badEachOfAll : Noun [] Object
  badEachOfAll = EachOf (AllOf Macros.creature)

-- …and it does not stack: one determiner fills the position, and "each
-- of each of them" spells nothing twice.
failing "GroupMention"
  badNestedEachOf : Noun [] Object
  badNestedEachOf = EachOf (EachOf (TargetGroup (Macros.upTo 2) Macros.creature))

-- A clause writing ONE per-member amount refuses a bare plural
-- recipient: "put a +1/+1 counter on up to two target creatures" is the
-- sentence the corpus never writes, and "each of" is exactly the word it
-- writes instead (zero lines at two, three, or four against
-- twenty-nine at "each of up to two target creatures").
failing "PerMember"
  badBarePluralCounterRecipient : Effect []
  badBarePluralCounterRecipient =
    PutCounters (Lit 1) Macros.plusOnePlusOne (TargetGroup (Macros.upTo 2) Macros.creature)

-- …and the damage verb reads the same way: "deals 1 damage to up to two
-- target creatures" is unwritten, while "to up to ONE target creature"
-- is twenty-eight lines and singular already.
failing "PerMember"
  badBarePluralDamageRecipient : Effect []
  badBarePluralDamageRecipient =
    DealDamage This (Lit 1) (TargetGroup (Macros.upTo 2) Macros.creature)

-- …and a plural READ is no better than a plural mention: the corpus's
-- "counters on them" lines are all relative clauses ("cards with intel
-- counters on them"), never a recipient.
failing "PerMember"
  badThemCounterRecipient : Effect []
  badThemCounterRecipient =
    Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature),
                  PutCounters (Lit 1) Macros.plusOnePlusOne Them]

-- …and the universal determiner with them: "deals 2 damage to all
-- creatures" is zero lines, the sweep being written distributively
-- ("damage to each creature", two hundred thirty-five).
failing "PerMember"
  badAllOfDamageRecipient : Effect []
  badAllOfDamageRecipient = DealDamage This (Lit 1) (AllOf Macros.creature)


-- …and the members must be a MENTION and not a description, since
-- [CR#601.2d] has the caster announce the division over the targets they
-- announced: "divided as you choose among each creature" is unwritten,
-- and every corpus line writes a counted target group or a plural read.
failing "GroupMention"
  badDivideAmongDescription : Effect []
  badDivideAmongDescription = Macros.dealsDivided This (Lit 2) (Each Macros.creature)

-- …and a division of nothing instructs nothing, which is the written-count
-- discipline reaching the new clause.
failing "WrittenCount"
  badDivideZero : Effect []
  badDivideZero = Macros.dealsDivided This (Lit 0) (TargetGroup (Macros.oneThrough 2) AnyTarget)

-- …and the counter half keeps the battlefield demand its undivided twin
-- carries: no corpus line distributes counters onto cards in a
-- graveyard.
failing "OnBattlefield"
  badDistributeCountersGraveyard : Effect []
  badDistributeCountersGraveyard =
    Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                       (TargetGroup (Macros.oneThrough 2) (And [Macros.creature, InZone (Macros.graveyardOf You)]))

-- "[-10]: Target player's life total becomes 1." — the set-to, chapter
-- twenty-two's ledgered player-attribute set. It leaves no outcome
-- mention behind, which is finding 121 made structural: a set is
-- realized as a gain or a loss depending on where the total stood, and
-- the sentence does not say which.
lifeTotalBecomesOne : Effect []
lifeTotalBecomesOne = Macros.lifeTotalBecomes (Macros.target AnyPlayer) (Lit 1)



-- ===== The ordered library, what may be exposed, and what may be subtracted =====

-- A library is ORDERED ([CR#401.2]), so "into your library" names no
-- place to put a card and oracle never writes it: every corpus
-- placement spells a position. `DestOk` has no row for the bare zone,
-- which is core's `exclude(Library)` on `Destination` exactly.
failing "DestOk"
  badMoveToBareLibrary : Effect []
  badMoveToBareLibrary = Move (Macros.target Macros.creature) (ZoneAt Library Bare)

-- The order rider needs two or more cards to order — [CR#401.4]'s own
-- condition, and English's: "put it on the bottom of your library in
-- any order" is zero lines.
failing "ArrangementOk"
  badSingularOrderRider : Effect []
  badSingularOrderRider =
    Sequentially [Macros.lookAt Macros.topCard, Move (That CardW) (Macros.onBottomIn AnyOrder)]

-- "The rest" of WHAT: with no group in the discourse the complement has
-- nothing to be the rest of.
failing "countGroups"
  badRestWithoutGroup : Effect []
  badRestWithoutGroup = Move TheRest Macros.graveyardZ

-- …and with a group but nothing taken out of it, "the rest" IS the
-- group and the sentence would have written "them".
failing "countParts"
  badRestWithoutPart : Effect []
  badRestWithoutPart = Sequentially [Macros.lookAt (Macros.topCards 4), Move TheRest Macros.onBottomZ]

-- One disposition per remainder, per path. "The rest" names what is
-- OUTSTANDING of a group, and once it has been placed nothing is:
-- Impulse writes one partition and one disposition, and the only three
-- corpus lines carrying two "the rest" phrases put them in mutually
-- exclusive if/instead/otherwise arms, which this refusal leaves
-- writable because a branch arm is typed in the discourse BEFORE the
-- disposition. The move spends the group (`groupSpent`).
failing "countGroups"
  badRestDisposedTwice : Effect []
  badRestDisposedTwice =
    Sequentially [ Macros.lookAt (Macros.topCards 4)
                 , Move (Macros.oneOf Them) Macros.handZ
                 , Move TheRest Macros.onBottomZ
                 , Move TheRest Macros.graveyardZ
                 ]

-- The direction chapter twenty-two refused, and it stays refused: two
-- separately announced targets are two mentions and never a pair
-- ([CR#601.2c], finding 127), so no complement can subtract inside
-- them. What changed is that a group ASSEMBLED by one phrase now
-- exists — not that announcements can be added up.
failing "countGroups"
  badRestOverTwoAnnouncements : Effect []
  badRestOverTwoAnnouncements =
    Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
                 , Move TheRest Macros.graveyardZ
                 ]

-- A partitive is a selection, but the corpus names its chooser every
-- time ("an opponent chooses two of them") and `Choose` has no agent
-- slot; an agentless "Choose one of them" is unwritten English.
failing "Choosable"
  badChooseSomeOf : Effect []
  badChooseSomeOf = Sequentially [Macros.lookAt (Macros.topCards 4), Choose (Macros.oneOf Them)]

-- A shuffle randomizes the library and destroys what the discourse
-- knew about it ([CR#701.24a]; [CR#701.20d] makes a reordered revealed
-- card a NEW object), so a card still in the library cannot be read
-- back afterwards. Every search sentence places its find first for
-- exactly this reason.
failing "countOnes"
  badReadAfterShuffle : Effect []
  badReadAfterShuffle =
    Sequentially [Macros.searchLibraryFor Macros.land, Macros.shuffle, Move It Macros.handZ]

-- The slice is in a library, so every battlefield-demanding verb
-- refuses it through the demand it already carried.
failing "OnBattlefield"
  badTapLibraryTop : Effect []
  badTapLibraryTop = Tap Macros.topCard

-- …and it describes no card, which is the hidden zone's honesty made
-- structural ([CR#400.2,401.2]): the phrase names a place, so a typed
-- demonstrative has nothing to reach.
failing "countManyWord"
  badSliceTypeRead : Effect []
  badSliceTypeRead =
    Sequentially [Macros.lookAt (Macros.topCards 4), Move (Those (TypeW Creature)) Macros.handZ]

-- A library is not shown whole: "reveal your library" is zero lines,
-- and what oracle exposes of a library is a positioned slice.
failing "ExposableZone"
  badRevealWholeLibrary : Effect []
  badRevealWholeLibrary = Expose Reveal You (ExposedZone Macros.yourLibrary)

-- Nor is a public zone: a graveyard is already visible to everyone
-- ([CR#400.2]), so revealing one says nothing and no line does it.
failing "ExposableZone"
  badRevealGraveyard : Effect []
  badRevealGraveyard = Expose Reveal You (ExposedZone Macros.graveyardZ)

-- Nor is the battlefield searched: [CR#701.23a] is about finding a card
-- among cards you cannot otherwise read.
failing "SearchableZone"
  badSearchBattlefield : Effect []
  badSearchBattlefield = Search You Macros.battlefieldZ Macros.creature

-- Nor is a POSITION in a library a zone a search looks through, which is
-- the same rule read one step further in: [CR#701.23a] looks at "all
-- cards in that zone" and [CR#401.2] makes the library "a single
-- face-down pile", so "the top of your library" is a place in the zone
-- and not the zone. The sort alone could not tell them apart —
-- `zoneSort` projects `Library` off either — so the clause asks the
-- PHRASE as well (`WholeZone`). The arrangement rider makes the nonsense
-- plain: a search cannot look through cards "in a random order".
failing "WholeZone"
  badSearchLibraryPosition : Effect []
  badSearchLibraryPosition = Search You (LibraryAt OnTop Nothing Bare) Macros.land

failing "WholeZone"
  badSearchLibraryPositionOrdered : Effect []
  badSearchLibraryPositionOrdered =
    Search You (LibraryAt OnBottom (Just RandomOrder) Bare) Macros.creature

-- The search's description is the zone's, not another zone's: the
-- clause supplies the place, so a phrase carrying its own is two
-- answers to one question.
failing "ZoneFree"
  badSearchZonedDescription : Effect []
  badSearchZonedDescription =
    Macros.searchLibraryFor (And [Macros.creature, InZone Macros.graveyardZ])


-- A zero slice and a zero mill instruct nothing, which is
-- `WrittenCount`'s discipline reaching two more counts.
failing "WrittenCount"
  badZeroSlice : Effect []
  badZeroSlice = Macros.lookAt (Macros.topCards 0)

failing "WrittenCount"
  badMillZero : Effect []
  badMillZero = Macros.millCards 0

-- A DISTRIBUTED mill leaves a plural group, and the count is not what
-- says so — the subject is. [CR#701.17a] has each milled-at player put
-- that many cards from their own library into their own graveyard, so
-- "each player mills a card" puts one card per player there and the
-- sentence after it cannot say "it". Locke, Treasure Hunter reads the
-- group plural in the next breath ("each player mills a card. … you may
-- cast a spell from among those cards"; the trigger shell and the
-- among-restriction are ledgered). Derived exactly as `Create`'s
-- distributed count is (`outputPlur`).
failing "countOnes Object"
  badDistributedMillSingular : Effect []
  badDistributedMillSingular =
    Sequentially [Mill (Each AnyPlayer) (Lit 1), Macros.exile It]

-- A partitive reaches into a GROUP, not into a description: "one of a
-- creature you control" is not English, and the determiner has no
-- members to pick from until some phrase has fixed them.
failing "GroupMention"
  badPartitiveOfDescription : Effect []
  badPartitiveOfDescription = Macros.exile (SomeOf (Macros.exactly 1) (Macros.a Macros.creature))

-- …nor into another partitive: a part is what was taken, not a group to
-- take from ("two of one of them" is zero lines).
failing "GroupMention"
  badPartitiveOfPartitive : Effect []
  badPartitiveOfPartitive =
    Sequentially [Macros.lookAt (Macros.topCards 4), Macros.exile (Macros.oneOf (Macros.oneOf Them))]

-- …nor into the complement: "each of the rest" is zero lines, the
-- remainder being named rather than reached into.
failing "GroupMention"
  badEachOfTheRest : Effect []
  badEachOfTheRest =
    Sequentially [ Macros.lookAt (Macros.topCards 4)
                 , Move (Macros.oneOf Them) Macros.handZ
                 , Macros.exile (EachOf TheRest)
                 ]


-- ===== What may be intercepted, prevented, and held =====

-- A durationless interception is the STATIC ABILITY line, not a clause.
-- "If a creature an opponent controls would die, exile it instead" is a
-- permanent's own ability ([CR#611.3] — a continuous effect from a
-- static ability lasts while the ability functions and states no
-- duration), and the corpus divides cleanly: every ONE-SHOT
-- interception writes a span. So `absentOk Replacement` is `False` and
-- the whole clause is unwritable here, which is `badStaticCant`'s
-- refusal a third time.
failing "SpanOk Replacement"
  badStandingIntercept : Effect []
  badStandingIntercept =
    Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) Nothing

-- The same refusal on the shield: forty-nine "Prevent all …" lines
-- state no duration and every one of them is a static ability
-- ("Prevent all combat damage that would be dealt to this creature").
failing "SpanOk Prevention"
  badStandingPrevention : Effect []
  badStandingPrevention = Macros.preventAll AnyDamage Everywhere Nothing

-- Prevention writes ONE adverbial. Two hundred fifty-five prevention
-- lines carry "this turn"; "prevent … until end of turn" is written
-- zero times, all scopes. The shield and the grants do not share a
-- current-turn word any more than the restriction and the grants do.
failing "SpanOk Prevention"
  badPreventUntilEndOfTurn : Effect []
  badPreventUntilEndOfTurn =
    Macros.preventAll CombatOnly Everywhere (Just Macros.untilEndOfTurn)

-- Nor a for-as-long-as one: no corpus line conditions a shield on a
-- tracked predicate ([CR#611.2b]'s adverbial), which is what keeps the
-- widest class in the table from swallowing the two new rows.
failing "SpanOk Replacement"
  badInterceptForAsLongAs : Effect []
  badInterceptForAsLongAs =
    Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It)
                   (Just (ForAsLongAs (Exists Macros.creatureYouControl)))

-- Real oracle English, no clause of ours: thirty-one lines write "would
-- be destroyed" and twenty-five of them are regeneration's own reminder
-- text, whose replacement is [CR#614.8]'s four-part instruction — tap
-- it, remove it from combat, heal the damage on it — not one part of
-- which this vocabulary writes. `EventUnclaimed` says exactly that, and
-- keeps it apart from the events nothing writes at all.
failing "Interceptable"
  badInterceptDestruction : Effect []
  badInterceptDestruction =
    Macros.ifWouldInstead (IsDestroyed (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn)

-- The multiplicity word is the event's to choose. "The next time
-- [subject] would die" is written zero times against fifty-seven
-- conditional lines, because a creature dies once and the two shields
-- would be the same shield ([CR#614.3]).
failing "ReplUseOk"
  badNextTimeWouldDie : Effect []
  badNextTimeWouldDie =
    Macros.nextTimeWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn)

-- (The inverse pin `badIfWouldDraw` stood here and is RETIRED, its cell
-- having been measured on the wrong axis: "if you would draw a card this
-- turn" is written zero times, which was read as a fact about the draw
-- event when it is a fact about the CARRIER. Twenty-one cards print the
-- durationless "If you would draw a card, … instead" as a static line —
-- `thoughtReflection` is the positive that replaced this refusal — and
-- the duration-bounded conditional the pin wrote is refused by nothing
-- the two-dimensional table can say. The carrier dimension is ledgered.)

-- Dying is the battlefield-to-graveyard transition ([CR#700.4]), so the
-- watched object stands on the battlefield — `EventQuery`'s own demand
-- asked by the other reader of the same event.
failing "ZoneFits"
  badWouldDieInGraveyard : Effect []
  badWouldDieInGraveyard =
    Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])))
                   (Macros.exile It) (Just Macros.thisTurn)

-- The replacement is a HOLE outward. [CR#614.7] says a replacement
-- effect whose intercepted event never happens "simply doesn't do
-- anything", so a token the replacement would have made is not there
-- for the next sentence to read — which is the conditional arm's
-- refusal (finding 100) at a second site.
failing "countOnes Object"
  badInterceptReplacementAntecedent : Effect []
  badInterceptReplacementAntecedent =
    Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                        (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Soldier]))
                                        (Just Macros.thisTurn)
                 , Macros.sacrifice You It
                 ]

-- No corpus line ends a CONTINUOUS effect at a leaves-the-battlefield
-- event with a clause this grammar writes. Of the ninety-one lines that
-- write the phrase, eighty-six are [CR#610.3] zone changes and three
-- are [CR#610.4] phasings — neither a continuous effect — and the three
-- that are one set a base type or make a copy. `Unclaimed` is the cell,
-- and it names the constructions the grammar is missing rather than
-- claiming the phrase.
failing "SpanOk PtDelta"
  badGetsUntilLeavesBattlefield : Effect []
  badGetsUntilLeavesBattlefield =
    Macros.gets (Macros.target Macros.creature) 2 2 (Just (UntilEvent (Leaves Macros.thisCreature)))

-- The rider goes on a zone change to EXILE and on nothing else: nothing
-- returns from a graveyard "until", and the corpus writes the rider
-- with the exile verb every time.
failing "HeldClause"
  badHeldUntilDestroy : Effect []
  badHeldUntilDestroy =
    HeldUntil (Macros.destroy (Macros.target Macros.creature)) (Leaves Macros.thisCreature)

-- And the event half of the same gate: the rider waits for a
-- leaves-the-battlefield event and for no other. Not one line writes
-- "exile [object] until [something] dies".
failing "Holdable"
  badHeldUntilDies : Effect []
  badHeldUntilDies =
    HeldUntil (Macros.exile (Macros.target Macros.creature)) (Dies Macros.thisCreature)

-- …and the exile the rider takes is the UNRIDDEN one. The two
-- constructions say opposite things about the same card: [CR#610.3]
-- schedules a return that no ability has to ask for, where counters sit
-- in exile waiting for an ability to read them, and no corpus line asks
-- for both — "exile … with … counters on it until …" is zero lines.
failing "HeldClause"
  badHeldUntilWithCounters : Effect []
  badHeldUntilWithCounters =
    HeldUntil (Macros.exileWithCounters (Macros.target Macros.creature) (Lit 3) Time)
              (Leaves Macros.thisCreature)

-- The held object's ZONE is not settled by the clause: the undo is
-- scheduled on an event that has not happened, so the exiled creature
-- may be in exile or back on the battlefield when a later sentence
-- reads it. The clause contributes its announcement and not the exile's
-- retag, so a graveyard-demanding read of the exiled card finds nothing
-- there to move.
failing "countWord CardW"
  badHeldUntilExileRetag : Effect []
  badHeldUntilExileRetag =
    Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Leaves Macros.thisCreature)
                 , Move (That CardW) Macros.handZ
                 ]

-- A replacement effect gets "only one opportunity to affect an event or
-- any modified events that may replace that event" ([CR#614.5]), so a
-- replacement of a replacement is not a sentence English writes.
failing "NotInstead"
  badNestedInstead : Effect []
  badNestedInstead =
    Macros.insteadOf (Macros.insteadOf Macros.drawACard (Macros.drawCards 2)) (Macros.drawCards 3)

-- [CR#614.6]: a replaced event "never happens". So the replaced
-- clause's OUTCOME is not there to read — "that much" after a damage
-- clause that was replaced measures nothing — even though the same
-- clause's announced target is ([CR#601.2c]). `annIntro` and `effIntro`
-- divide exactly here.
failing "countOnes Outcome"
  badInsteadReadsReplacedOutcome : Effect []
  badInsteadReadsReplacedOutcome =
    Macros.insteadOf (DealDamage This (Lit 3) (Macros.target AnyTarget))
              (Macros.gainsLife You ThatMuch)

-- The same refusal through a SEQUENCE, which is where it used to leak:
-- the replacement was typed in `preIntro replaced`, and a sequence's
-- pre-state is its last clause's over the DEED telescope, so an earlier
-- step's outcome walked into a replacement for an event that never
-- happened. The announcement channel is a hole at a sequence, its
-- elements being typed over each other's `effIntro` (`annIntro`).
failing "countOnes Outcome"
  badInsteadReadsReplacedSequenceOutcome : Effect []
  badInsteadReadsReplacedSequenceOutcome =
    Macros.insteadOf (Sequentially [DealDamage This (Lit 3) (Macros.target Macros.creature), Macros.drawACard])
              (Macros.gainsLife You ThatMuch)

-- And through a CONDITIONAL wrapping an optional clause, which is the
-- recursion that carried either defect: chapter twenty-five opened the
-- conditional's announcement channel on `preIntro e`, correctly for the
-- flat clause it was opened for (Overload's "that artifact", Colossal
-- Growth's "that creature") and not for a composite. It is structural
-- now, so the nested may's damage outcome is not there to read.
failing "countOnes Outcome"
  badConditionalInsteadReadsMayOutcome : Effect []
  badConditionalInsteadReadsMayOutcome =
    Macros.insteadOf (If (Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)))
                  (Exists Macros.creature)
                  Nothing)
              (Macros.gainsLife You ThatMuch)

-- ===== What may be paid, what may be spelled "pay", and what may be
-- granted =====

-- A draw is not a payment. The colon used to accept any clause at all,
-- which is what the ledger's cost-GRAMMAR entry named: [CR#602.1a] makes
-- a cost what the ACTIVATOR pays, and no corpus line writes "Draw a
-- card:" before a colon (zero, against eleven hundred eighty-four
-- sacrifice components).
failing "CostAction"
  badDrawAsCost : Ability
  badDrawAsCost = Activated (Do Macros.drawACard) Macros.drawACard


-- The life row is DIRECTIONAL: ninety-five "Pay N life" components
-- against zero gain-life ones, so paying life is a cost and gaining it
-- is not. A gain-life cost does exist — [CR#119.7] speaks of "a cost
-- that involves having that player gain life" — and the cards that print
-- one spell it as an ALTERNATIVE cost ([CR#118.9]), a base swap rather
-- than an activation cost.
failing "CostAction"
  badGainLifeCost : Ability
  badGainLifeCost = Activated (Do (Macros.gainsLife You (Lit 2))) Macros.drawACard

-- A cost component may not read a SIBLING component's deed. The
-- telescope threads the stamp because the ability BODY reads it (Bosh,
-- Iron Golem), but [CR#601.2h] pays the components in two tiers and each
-- of them "in any order", so no component may presuppose that a SIBLING
-- has already been paid — and no corpus line writes one that does.
failing "CostAction"
  badCostReadsSiblingDeed : Ability
  badCostReadsSiblingDeed =
    Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                         Do (Macros.exile (TheVerbed Sacrifice CardW))])
              Macros.drawACard

-- Nor may one carry a TARGET. [CR#601.2c] announces targets while the
-- ability is still being proposed and [CR#601.2h] pays the costs at the
-- END of that same procedure, so the determiner belongs past the colon
-- and never before it.
failing "CostAction"
  badTargetedCost : Ability
  badTargetedCost = Activated (Do (Macros.sacrifice You (Macros.target Macros.creature))) Macros.drawACard

-- And the payer is the ACTIVATOR: [CR#602.1a] says the activation cost
-- "must be paid by the player who is activating it", so a component that
-- names its payer names "you" (Erebos, God of the Dead's "Pay 2 life")
-- and no line charges an opponent. The demand is the POSITION's and not
-- the clause's — the same life component under the `Pay` clause names
-- whoever the sentence names, which is the hundred seventy-seven
-- "unless its controller / that player / any player pays" lines
-- `badUnlessAnaphoricPayer` is measured against.
failing "CostPaidByYou"
  badForeignPayerCost : Ability
  badForeignPayerCost = Activated (Macros.payLife Macros.anOpponent 2) Macros.drawACard

-- …and the subjected cost verbs the same way: "an opponent sacrifices a
-- creature" is real English as a resolving clause and is no payment of
-- YOURS before a colon.
failing "CostPaidByYou"
  badForeignSacrificeCost : Ability
  badForeignSacrificeCost =
    Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) Macros.drawACard

-- "Pay" is one English VERB and a cost is the whole thing an ability
-- charges. A sacrifice is a payment ([CR#118.1] — a cost is "an action
-- or payment") and is not payABLE:
-- the seventy-one non-mana unless lines write their own verb ("unless
-- you sacrifice a land"), never "pay".
failing "Payable"
  badPayBySacrificing : Effect []
  badPayBySacrificing = Pay You (Do (Macros.sacrifice You (Macros.a Macros.creature)))

-- The pay clause spells its subject ONCE, so the component under the
-- verb names the same player: "you pay 1 life" (Carnophage) is one payer
-- written once, and "you pay [an opponent pays 1 life]" is a sentence
-- with two. Conservative in the direction the telescope allows — the
-- imperative's component must name you, a named subject's is left to the
-- anaphoric-payer family `badUnlessAnaphoricPayer` measures.
failing "PayAgrees"
  badMismatchedPayer : Effect []
  badMismatchedPayer = Macros.mayElse You (Pay You (Macros.payLife Macros.anOpponent 1)) Macros.drawACard

-- The tap symbol still more sharply: it is a cost that exists only
-- before a colon ([CR#107.5] gives it its meaning there), and no
-- sentence anywhere spells it as a verb phrase.
failing "Payable"
  badPayTapSymbol : Effect []
  badPayTapSymbol = Pay You TapSymbol

-- And no line writes "pay" over a comma-joined cost: the compound is a
-- cost SHAPE, not a complement English's pay-verb takes.
failing "Payable"
  badPayCompound : Effect []
  badPayCompound = Pay You (Compound [Mana [Macros.generic 1], TapSymbol])

-- Growing the container is what made this refusal necessary. English
-- grants an activated ability by QUOTING it — "Enchanted land has \"{T}:
-- Add {B}\"", twenty-five lines, and the equipped/all-Slivers twins with
-- it — which is a construction this grammar has no quotation for, where
-- "gains flying" is a bare keyword.
failing "Grantable"
  badGainsActivated : Effect []
  badGainsActivated = Macros.gains (Macros.target Macros.creature)
                            (Activated (Mana [Macros.generic 1]) Macros.drawACard)
                            (Just Macros.untilEndOfTurn)

-- A compound's ELEMENTS are components: nesting re-mints the
-- right-nested tree the telescope replaced, and core reaches the flat
-- form by normalizing instead (`Cost::normalize`).
failing "NotCompound"
  badNestedCompound : Ability
  badNestedCompound =
    Activated (Compound [Compound [Mana [Macros.generic 1], TapSymbol], TapSymbol]) Macros.drawACard

-- And a compound of one is the component itself spelled a second way —
-- the singleton-sequence refusal at the cost layer.
failing "AtLeastTwo"
  badSingletonCompound : Ability
  badSingletonCompound = Activated (Compound [Mana [Macros.generic 1]]) Macros.drawACard

-- One self-tap per cost. [CR#107.5] says it in as many words — "a
-- permanent that's already tapped can't be tapped again to pay the
-- cost" — and [CR#118.3] refuses a payment the payer has not got the
-- resources for, so a second "{T}" charges a state the permanent has
-- once. No corpus line writes "{T}, {T}:" and none writes the untap
-- symbol beside it either, the two spending the same state.
failing "CostTapOnce"
  badDoubleTapCost : Ability
  badDoubleTapCost = Activated (Compound [TapSymbol, TapSymbol]) Macros.drawACard

-- A cost component's symbol RUN is written. The empty list is a real
-- value on a CARD — [CR#202.1b]'s "no mana cost", the unpayable absence
-- a land prints — but before a colon the run is the component's whole
-- spelling, so an empty one spells an activation line opening on a bare
-- colon. The payment of nothing is "{0}" ([CR#118.5]), one symbol.
failing "ManaRun"
  badEmptyManaCost : Ability
  badEmptyManaCost = Activated (Mana []) Macros.drawACard

-- A hybrid Phyrexian symbol names two DIFFERENT colors, and [CR#107.4f]
-- says so by counting: five ordinary Phyrexian symbols and "ten hybrid
-- Phyrexian mana symbols", ten being the unordered pairs of five
-- distinct colors. There is no "{W/W/P}" to write.
failing "PhyrexianDistinct"
  badSameColorPhyrexian : ManaSymbol
  badSameColorPhyrexian = Phyrexian White (Just White)

-- …and the ordinary hybrid the same way: [CR#107.4e] makes the symbol "a
-- cost that can be paid in one of two ways", and two ways spelled the
-- same word is one way.
failing "HalvesDistinct"
  badSameColorHybrid : ManaSymbol
  badSameColorHybrid = Macros.hybridPip Blue Blue

-- The arm asymmetry carries to the MANDATORY twin unchanged, which is
-- the check that the two shapes really are one node: the declined arm
-- runs only when the payment was never started ([CR#118.12] — "started
-- to pay a mandatory cost, regardless of what events actually
-- occurred"), so the body's phrase named nobody it can read.
failing "countOnes"
  badIfNotReadsMandatoryBody : Effect []
  badIfNotReadsMandatoryBody = Macros.doElse (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile It)


-- The unless family's OTHER half, and the shape of its wall: a hundred
-- twenty-five "unless its/their controller pays" lines, forty-three
-- "unless that player pays" and nine "unless any player pays" name their
-- payer with an anaphor into the MAIN clause ("Return target creature to
-- its owner's hand unless its controller pays {1}"). [CR#118.12a]'s
-- rewrite is not linearization-preserving — it puts the may first — so
-- the payer phrase is typed before the clause that announces what it
-- reads, and "it" has nothing to reach.
failing "countOnes"
  badUnlessAnaphoricPayer : Effect []
  badUnlessAnaphoricPayer =
    Macros.mayElse (ControllerOf It) (Pay You (Mana [Macros.generic 1])) (Tap (Macros.target Macros.creature))


-- ===== What may trigger, what may be stated as a line, and what a
-- ===== permission may permit

-- [CR#603.2b] gives "at the beginning of" a phase or step its own
-- clause, and the corpus honors it without exception: one thousand six
-- hundred seventy-eight headers open with "At the beginning of" and
-- every one of them names a turn part. No object event takes the word.
failing "TriggerWordOk"
  badAtEnters : Ability
  badAtEnters = Triggered At (Enters Macros.thisCreature) Macros.drawACard

-- The trigger WORD table is right and the SUBJECT was not: "When this
-- enters" and "Whenever this enters" are written zero times apiece,
-- against eighteen hundred eighteen "When this creature enters" and
-- seventy-four "Whenever this creature enters", and every other
-- object-subject event measures the same way (dies three hundred
-- eighty-one, attacks five hundred sixty-eight, blocks sixty-nine,
-- deals combat damage two hundred thirty-one — all against zero bare
-- ones). The type word is what places the referent ([CR#109.2]); bare
-- `This` is the source as an object and stands nowhere.
failing "SelfSorted"
  badEntersBareThis : Ability
  badEntersBareThis = Triggered When (Enters This) Macros.drawACard

-- An ordinary trigger's header announces no TARGET. [CR#115.1d] chooses
-- a triggered ability's targets "as the ability is put on the stack",
-- which is after the event that put it there, so the header is not where
-- one can be announced — the ability targets one clause later
-- (`eliteJavelineer`). The delayed carrier is the exception the corpus
-- writes and it still builds (`gracefulReprieve`).
failing "HeaderNontarget"
  badTargetedDeathHeader : Ability
  badTargetedDeathHeader = Triggered Whenever (Dies (Macros.target Macros.creature)) Macros.drawACard


-- …and the inverse, which is the half that makes the table a table: the
-- turn-part beginning takes neither English word, "When the beginning of
-- your upkeep" being written zero times against six hundred forty-one
-- "At the beginning of your upkeep".
failing "TriggerWordOk"
  badWhenUpkeep : Ability
  badWhenUpkeep = Triggered When (BeginningOf Upkeep (Just Yours)) Macros.drawACard

-- Real oracle English in one mood and none in the other: thirty-one
-- lines write "would be destroyed" (regeneration's reminder text,
-- [CR#614.8]) and NOT ONE writes "whenever [something] is destroyed" as
-- a header — the modern templating for that event is "dies", which is a
-- different row. `EventUnclaimed` is the only class no reader claims.
failing "Triggerable"
  badTriggerOnDestruction : Ability
  badTriggerOnDestruction = Triggered Whenever (IsDestroyed (Macros.a Macros.creature)) Macros.drawACard

-- The untap step's beginning is written zero times in every possession —
-- the turn-part grid's emptiest row, and `PartUnattested` says so.
failing "PartTriggerable"
  badTriggerAtUntapStep : Ability
  badTriggerAtUntapStep = Triggered At (BeginningOf UntapStep Nothing) Macros.drawACard

-- Nor the turn's own beginning: "at the beginning of your turn" is zero
-- lines, because the UPKEEP is what English names there.
failing "PartTriggerable"
  badTriggerAtYourTurn : Ability
  badTriggerAtYourTurn = Triggered At (BeginningOf Turn (Just Yours)) Macros.drawACard

-- The other silence, and it is the possessor gap chapter twenty-seven
-- measured from the activation side: "At the beginning of each upkeep"
-- (thirty-six), "each opponent's upkeep" (thirty-three) and "the upkeep
-- of enchanted creature's controller" (twenty-seven) are real headers
-- whose possessor is a quantifier or a nominal, and `Whose` is a
-- two-word pronominal vocabulary. Unpossessed is not what they write.
failing "PartTriggerable"
  badTriggerAtTheUpkeep : Ability
  badTriggerAtTheUpkeep = Triggered At (BeginningOf Upkeep Nothing) Macros.drawACard

-- The trigger's effect reads the event's AFTER-discourse, and this is
-- the refusal that shows it: [CR#603.6] has a zone-change trigger "look
-- for the object in the zone that it moved to", so after "Whenever a
-- creature dies" the referent is a card in a graveyard and tapping it is
-- the dead-referent refusal ([CR#701.26a]). The interception's
-- replacement, over the same event row, taps it perfectly well — the
-- event has not happened there ([CR#614.6]).
failing "OnBattlefield"
  badTriggerTapsDeadCreature : Ability
  badTriggerTapsDeadCreature = Triggered Whenever (Dies (Macros.a Macros.creature)) (Tap It)

-- …and the DEPARTURE is the same refusal where the destination is not
-- stated. [CR#603.6c] has a leaves-the-battlefield ability check for the
-- object "only in the first zone that it went to", and the sentence
-- never names that zone, so an unknown destination is not still the
-- battlefield: the binding survives the event and its zone does not.
failing "OnBattlefield"
  badLeavesThenTap : Ability
  badLeavesThenTap = Triggered Whenever (Leaves (Macros.a Macros.creature)) (Tap It)

-- A static ability does not TARGET. [CR#115.1a..115.1e] enumerate what
-- can — an instant or sorcery spell, an activated ability, a triggered
-- ability, and the keyword abilities that represent those — and
-- [CR#115.1b] says of the nearest case outright that "an Aura permanent
-- doesn't target anything; only the spell is targeted". The same
-- statement is impeccable as a resolving CLAUSE with a span
-- (`cantAttack (target creature) (Just thisTurn)`), which is what makes
-- this the line's own refusal rather than the deontic's.
failing "Untargeting"
  badStaticTargets : Ability
  badStaticTargets = Static (Cant (Macros.target Macros.creature) Attack Agent)

-- The one static effect English does not state as a line. Its stative
-- form is a different VERB — "You control enchanted creature" (seven
-- lines) — where every other row inflects the same verb it writes as a
-- clause ("gains"/"has", "becomes"/"is"). Zero lines write a
-- durationless "gains control of".
failing "StaticLine"
  badStaticGainsControl : Ability
  badStaticGainsControl = Static (GainsControl You (AllOf Macros.creatureYouControl))

-- …and the "as long as" wrapper does not launder it. The mood is the
-- statement's and not the qualifier's: [CR#604.1] has a static ability
-- "written as a statement" that is "simply true", so qualifying an
-- unstatable one leaves it unstatable — Control Magic writes "You
-- control enchanted creature" whether or not a condition rides on it.
failing "StaticLine"
  badConditionalGainControl : Ability
  badConditionalGainControl =
    Static (Macros.asLongAs (Exists Macros.artifact) (GainsControl You (Macros.a Macros.creature)))

-- "As long as" is not a duration adverbial and its statement is not a
-- resolving clause: the conditional static is an ability line and
-- nothing else, so `absentOk Conditional` is `False` and every
-- `admitsSpan` cell with it is too. One preposition tells the two
-- constructions apart — [CR#611.2b]'s "FOR as long as" is the duration
-- (two hundred ten lines, and `Duration.ForAsLongAs` already spells it),
-- [CR#611.3a]'s bare "as long as" is this (nine hundred twelve).
failing "SpanOk"
  badConditionalClause : Effect []
  badConditionalClause =
    Continuously (Macros.asLongAs (Exists Macros.creatureYouControl)
                           (Gets (AllOf Macros.creatureYouControl) 1 1))
                 Nothing

-- And no line conditions a statement twice: the singleton discipline
-- `badNestedCompound` keeps at the cost layer, one type up.
failing "NotConditional"
  badDoubleConditional : Ability
  badDoubleConditional =
    Static (Macros.asLongAs (Exists Macros.creatureYouControl)
                     (Macros.asLongAs (Exists (And [Macros.artifact, ControlledBy You]))
                               (Gets (AllOf Macros.creatureYouControl) 1 1)))

-- The entry rider is a static ability and not a clause either
-- ([CR#603.6d] says so in as many words), which is the third of the
-- three families chapter twenty-five sent to a container that did not
-- exist. "Put [card] onto the battlefield tapped" is the one-shot twin
-- and it is a rider on a MOVE, not a continuous effect (ledger).
failing "SpanOk"
  badEntryRiderClause : Effect []
  badEntryRiderClause = Continuously (Macros.entersTapped (AllOf Macros.creatureYouControl)) Nothing

-- …and the line writes ONE of the two riders. A token's with-clause can
-- say "tapped and attacking" because a resolving effect knows there is a
-- combat; a permanent's own static ability applies whenever it enters,
-- from any zone in any step, and the corpus writes the second rider on
-- such a line zero times.
failing "EntryRiderOk"
  badEntersAttackingLine : Ability
  badEntersAttackingLine = Static (EntersRider (AsType Land This) EntersAttacking)

-- The permission divides the same way, and it is the sharpest of the
-- three because it writes spans freely: ninety-eight "this turn",
-- twenty-two "until the end of your next turn", twenty-six "for as long
-- as", eight "until your next end step" — and states none at all only
-- when it is the card's own line ("You may cast this card from your
-- graveyard").
failing "SpanOk"
  badStandingPermission : Effect []
  badStandingPermission =
    Sequentially [Macros.exile Macros.topCard, Continuously (MayPlay You It) Nothing]

-- Nor at an endpoint the permission does not write: "until end of
-- combat" is the two GRANTS' word and the permission's zero times.
failing "SpanOk"
  badPermissionUntilEndOfCombat : Effect []
  badPermissionUntilEndOfCombat =
    Sequentially [Macros.exile Macros.topCard,
                  Continuously (MayPlay You It) (Just Macros.untilEndOfCombat)]

-- The new cell is the permission's ALONE, which is what `PermissionOnly`
-- claims: eight lines write "until your next end step" and every one of
-- them permits playing just-exiled cards. No grant writes it.
failing "SpanOk"
  badGainsUntilYourNextEndStep : Effect []
  badGainsUntilYourNextEndStep =
    Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.untilYourNextEndStep)

-- A permanent on the battlefield has already been played: [CR#604.6]
-- files the permission as functioning "while a card is in any zone that
-- you could cast or play it from", and the battlefield is not one. The
-- refusal is the exact inverse of `Cant`'s battlefield DEMAND, which is
-- why the permissive twin could not be the prohibition with its polarity
-- flipped. (The pin now names `PlaySource`, the witness that reads
-- `playableFrom` on whichever of the two places states the zone — the
-- complement's binding here, since no source phrase is written.)
failing "PlaySource"
  badPlayFromBattlefield : Effect []
  badPlayFromBattlefield =
    Continuously (MayPlay You (Macros.a Macros.creature)) (Just Macros.thisTurn)

-- Growing the container grew its refusal with it: English grants a
-- triggered ability by QUOTING it, exactly as it grants an activated one
-- ("Enchanted creature has 'When this creature dies, …'"), and this
-- grammar has no quotation.
failing "Grantable"
  badGainsTriggered : Effect []
  badGainsTriggered =
    Macros.gains (Macros.target Macros.creature) (Triggered When (Enters Macros.thisCreature) Macros.drawACard)
          (Just Macros.untilEndOfTurn)

-- …and a static ability the same way ("as long as enchanted permanent is
-- an Equipment, it has 'Equipped creature gets +1/+1 and has trample'").
failing "Grantable"
  badGainsStatic : Effect []
  badGainsStatic =
    Macros.gains (Macros.target Macros.creature) (Static (Cant (AllOf Macros.creatureYouControl) Attack Agent))
          (Just Macros.untilEndOfTurn)

-- [CR#603.7b] gives a delayed trigger ONE stated duration and names the
-- phrase: "unless it has a stated duration, such as 'this turn'". The
-- corpus writes no other — Graceful Reprieve's "when target creature
-- dies this turn" is the family, and the boundary endpoints belong to
-- continuous effects, which a delayed trigger is not.
failing "DelaySpanOk"
  badDelayedUntilEndOfTurn : Effect []
  badDelayedUntilEndOfTurn =
    Delayed (Dies (Macros.target Macros.creature)) {span = Just Macros.untilEndOfTurn}
            (Move (That CardW) Macros.battlefieldZ)

-- The delayed clause reads three events of the ten and the attack is not
-- one: "sacrifice it when this creature attacks" is written zero times,
-- the delayed family being the end-step beginning, the departure and the
-- death.
failing "Awaitable"
  badDelayedOnAttack : Effect []
  badDelayedOnAttack =
    Delayed (Attacks (Macros.target Macros.creature)) (Macros.sacrifice You (That (TypeW Creature)))

-- The event vocabulary's four readers disagree, and the entry is the
-- clearest case: two thousand eight hundred ninety-one trigger headers
-- and no would/instead clause of ours. The fourteen enter-interceptions
-- the corpus does write are the entry RIDER — [CR#603.6d]'s static
-- ability, which is now a row of its own with its own spelling — so the
-- would-clause has nothing left to say about the event.
failing "Interceptable"
  badInterceptEnters : Effect []
  badInterceptEnters =
    Macros.ifWouldInstead (Enters (Macros.a Macros.creature)) (Macros.exile It) (Just Macros.thisTurn)

-- …and the [CR#610.3] rider does not wait for one either: "exile it
-- until a creature enters" is written zero times, the rider's whole
-- corpus being the departure.
failing "Holdable"
  badHeldUntilEnters : Effect []
  badHeldUntilEnters = Macros.exileUntil (Macros.target Macros.creature) (Enters (Macros.a Macros.creature))

-- The turn-part beginning is real oracle as a duration endpoint —
-- "until the beginning of your next upkeep", twenty-eight lines — and
-- the adverbial that writes it is `DurationEnd`'s own `StartOf` row. One
-- phrase, one slot: the event axis must not spell the same endpoint a
-- second way, which is what `Unclaimed` records here.
failing "SpanOk"
  badUntilBeginningOfUpkeep : Effect []
  badUntilBeginningOfUpkeep =
    Macros.gets (Macros.target Macros.creature) 3 3 (Just (UntilEvent (BeginningOf Upkeep (Just Yours))))

-- ===== What "you do" may stand for, and what it cannot reach =====

-- The anaphor is a PRO-VERB and its subject is a player, so the clause
-- it abbreviates has to have one. [CR#120.1] gives a damage clause an
-- OBJECT as its agent — "an object that deals damage is the source of
-- that damage" — and "this creature deals 3 damage to any target. When
-- you do, …" is written zero times. The one corpus line that looks like
-- the counterexample writes the causative instead and proves the point:
-- Elektra, Femme Fatale's "you may HAVE her deal 2 damage to you. When
-- you do, she deals 4 damage to target creature", where the having is
-- yours and the dealing is hers.
failing "ReflexEnclosure"
  badReflexiveOnSourceDeed : Effect []
  badReflexiveOnSourceDeed =
    Reflexively (DealDamage This (Lit 3) (Macros.target AnyTarget)) Macros.drawACard

-- The same demand at the row where a rule rather than a count settles
-- it: [CR#119.9] rewrites the life-gain trigger as "whenever a source
-- causes [a player] to gain life", which makes the player the PATIENT of
-- a life change and leaves "do" nobody to inflect for. English writes
-- the payment instead — "you may pay 2 life. When you do, …" — and
-- paying IS an action a player takes, which is why one life change is
-- an enclosure in the cost frame and none is in the sentence frame.
failing "ReflexEnclosure"
  badReflexiveOnLifeGain : Effect []
  badReflexiveOnLifeGain =
    Reflexively (Macros.gainsLife You (Lit 2)) Macros.drawACard

-- One verb phrase, because "do" abbreviates one. A sequence has no
-- single action for it to stand for, and the corpus writes no reflexive
-- over one: every one of the two hundred ninety-one enclosures is a
-- single clause, and a card with two sentences before the anaphor hangs
-- it on the LAST of them (Hypothesizzle's "Draw two cards. Then you may
-- discard a nonland card. When you do, …").
failing "ReflexEnclosure"
  badReflexiveOnSequence : Effect []
  badReflexiveOnSequence =
    Reflexively (Sequentially [Macros.drawACard, Macros.sacrifice You (Macros.a Macros.creature)]) Macros.drawACard

-- A clause that SCHEDULES its action has not taken it. [CR#603.12]
-- triggers the reflexive on whether the event "occurred earlier during
-- the resolution of the spell or ability that created them", and a
-- delayed trigger's clause happens at some later time by construction —
-- so "sacrifice it at the beginning of the next end step. When you do,
-- …" names an action that has not happened yet. Zero corpus lines.
failing "ReflexEnclosure"
  badReflexiveOnDelayed : Effect []
  badReflexiveOnDelayed =
    Reflexively (Delayed (BeginningOf EndStep (Just Yours)) Macros.drawACard) Macros.drawACard

-- One offer, one reader. [CR#118.12]'s "if you do" and [CR#603.12]'s
-- "when you do" ask the same question of the same choice — the first
-- inside the resolution, the second as a new ability — and no corpus
-- line writes both over one offer. Heart-Piercer Manticore's own ruling
-- states the difference the card has to choose between: with the
-- reflexive, "players may cast spells and activate abilities before a
-- creature is sacrificed and then again after the creature is
-- sacrificed but before damage is dealt".
failing "ReflexEnclosure"
  badReflexiveOnBranchedMay : Effect []
  badReflexiveOnBranchedMay =
    Reflexively (Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.creature)) Macros.drawACard) Macros.drawACard

-- …and the declined arm is refused by the same row, which is the
-- untaken path this construction has no arm for at all. [CR#603.12]
-- words the family as triggering "when [a player] [does or doesn't]"
-- take the action, so the rule licenses a negative reflexive; English
-- writes it zero times ("When you don't" is one corpus line and it is a
-- quoted STATE trigger, Olivia's "When you don't control a legendary
-- Vampire, exile this creature"). The declined branch stays `May`'s
-- `ifNot`, where eighty-three lines are.
failing "ReflexEnclosure"
  badReflexiveOnDeclinedMay : Effect []
  badReflexiveOnDeclinedMay =
    Reflexively (Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) Macros.drawACard) Macros.drawACard

-- The `EncUnclaimed` cell as a pin. "Target opponent gains control of
-- Yes Man. When they do, …" is real oracle and a reflexive trigger by
-- ruling rather than by inference (Yes Man, Personal Securitron,
-- 2024-03-08: "that effect is part of a reflexive triggered ability that
-- triggers only if the target opponent gains control of Yes Man"), and
-- it is the family's only line whose enclosure establishes a continuous
-- effect instead of naming an action. One card, and its trigger body
-- wants a quest counter `CounterKind` does not carry, so the cell is
-- ledgered rather than opened — the same distinction chapter
-- twenty-eight's `PartUnclaimed` cells make one level down.
failing "ReflexEnclosure"
  badReflexiveOnGainsControl : Effect []
  badReflexiveOnGainsControl =
    Reflexively (Macros.gainsControl (Macros.target Opponent) This Nothing) Macros.drawACard

-- What the construction contributes OUTWARD is the enclosure's discourse
-- and none of the trigger's. [CR#603.3] puts a triggered ability on the
-- stack "the next time a player would receive priority", so the rest of
-- this resolution finishes before the reflexive resolves and cannot
-- mention what it will do: "Mill four cards. When you do, create a
-- token. Tap that creature." names no creature at the third sentence.
-- `Delayed`'s hole with a clause in front of it that DID run.
failing "countWord"
  badAfterReflexiveReadsTrigger : Effect []
  badAfterReflexiveReadsTrigger =
    Sequentially [Reflexively (Macros.millCards 4)
                              (Macros.create (Lit 1) (MkToken (Just (1, 1)) [White]
                                                       (MkTypeLine [Soldier] [Creature])
                                                       [] Nothing)),
                  Tap (That (TypeW Creature))]

-- …and INWARD it is the enclosure's post-state, not its announcement,
-- which is the whole difference from `InsteadOf` beside it: that node's
-- replaced event never happened ([CR#614.6]) where this one fires
-- because the action DID. So the sacrificed creature is in its graveyard
-- when the trigger body reads it, and tapping it is
-- `badTriggerTapsDeadCreature`'s refusal at a second construction.
failing "OnBattlefield"
  badReflexiveTapsSacrificed : Effect []
  badReflexiveTapsSacrificed =
    Reflexively (Macros.sacrifice You (Macros.a Macros.creature)) (Tap It)

-- ===== What a placement may arrive with =====

-- An arrival rider is BATTLEFIELD-only. [CR#110.5b] states the default
-- the "tapped" rider overrides — permanents "enter the battlefield
-- untapped … unless a spell or ability says otherwise" — and there is
-- no such default anywhere else, a card in a graveyard being neither
-- tapped nor untapped ([CR#110.5] gives STATUS to permanents). Zero
-- corpus lines write a rider on any other destination.
failing "RidersFit"
  badMoveRidersToGraveyard : Effect []
  badMoveRidersToGraveyard =
    Move (Macros.target Macros.creature) Macros.graveyardZ
         {riders = MkMoveRiders [EntersTapped] Nothing}

-- …and the control override is refused there by the same gate, for
-- [CR#109.4]'s reason rather than [CR#110.5b]'s: an object that is
-- neither on the stack nor on the battlefield "aren't controlled by any
-- player", so a hand arrival has no controller to name.
failing "RidersFit"
  badMoveControlToHand : Effect []
  badMoveControlToHand =
    Move (Macros.target Macros.creature) Macros.handZ {riders = MkMoveRiders [] (Just You)}

-- The entry riders keep chapter nineteen's SHAPES, sharing `ridersOk`
-- with the token with-clause rather than re-minting one: attacking
-- alone is written zero times in either frame, a creature put onto the
-- battlefield attacking being put there tapped as well.
failing "RidersOk"
  badMoveAttackingUntapped : Effect []
  badMoveAttackingUntapped =
    Move (Macros.target Macros.creature) Macros.battlefieldZ
         {riders = MkMoveRiders [EntersAttacking] Nothing}

-- …and the order is fixed there too, no line writing "attacking and
-- tapped".
failing "RidersOk"
  badMoveRidersReversed : Effect []
  badMoveRidersReversed =
    Move (Macros.target Macros.creature) Macros.battlefieldZ
         {riders = MkMoveRiders [EntersAttacking, EntersTapped] Nothing}

-- ONE controller ([CR#109.4] — an object on the battlefield has a
-- controller, not controllers), which is `ControlledBy`'s own demand at
-- a second site. "Under their owners' control" is two lines and a
-- plural relational this vocabulary does not spell.
failing "CtrlSingular"
  badMoveRidersPluralController : Effect []
  badMoveRidersPluralController =
    Move (Macros.target Macros.creature) Macros.battlefieldZ
         {riders = MkMoveRiders [] (Just (AllOf Macros.otherPlayer))}

-- The COUNTER rider does not answer the battlefield question the other
-- two answer, and this is the pin that says the halves are gated apart:
-- a graveyard placement takes no counters ([CR#122.1a]'s zone-other-than
-- clause is real, but zero corpus lines write counters onto a card
-- entering a graveyard) and it is `counterZone` that refuses it, not
-- `sameZone … Battlefield`.
failing "RidersFit"
  badMoveCountersToGraveyard : Effect []
  badMoveCountersToGraveyard =
    Move (Macros.target Macros.creature) Macros.graveyardZ
         {riders = MkMoveRiders [] Nothing
                                {counters = Just (MkCounterRider (Lit 1) Macros.plusOnePlusOne)}}

-- …and the leak the other way is refused by the same table's other half:
-- an exile writes the counter rider and nothing else, "exile it tapped"
-- being zero lines because [CR#110.5b]'s untapped default and
-- [CR#110.5]'s status words are the battlefield's alone. One gate, two
-- questions, and each rider half asks only its own.
failing "RidersFit"
  badExileTapped : Effect []
  badExileTapped =
    Composite Exile (Move (Macros.target Macros.creature) Macros.exileZ
                          {riders = MkMoveRiders [EntersTapped] Nothing})

-- The rider's count is a WRITTEN magnitude like every other, so the
-- unwritable zero is unwritable here too (`badPutZeroCounters` at a
-- second site).
failing "WrittenCount"
  badExileZeroCounters : Effect []
  badExileZeroCounters = Macros.exileWithCounters (Macros.target Macros.creature) (Lit 0) Time

-- ===== What a linkage may name, and where its cards are =====

-- A linkage read names the exiles of the ability's OWN object and no
-- other. [CR#607.1] builds the relation out of two abilities "printed on
-- it", [CR#406.6] repeats it for this family by name, and [CR#607.5]'s
-- worked example turns on exactly this: a creature that has gained two
-- different exiling abilities can return only what the LINKED one
-- exiled. "Cards exiled with target creature" is unwritten and
-- unwritable.
failing "LinkSource"
  badExiledWithOtherSource : Predicate [] Object
  badExiledWithOtherSource = ExiledWith (Macros.target Macros.creature)

-- …and a DESCRIBED source is refused by the same gate, which is the
-- sharper half of the same fact: the linkage is not a relation between a
-- card and whatever exiled it ("a card exiled with an artifact" is
-- unwritten), it is one object's note about its own exiles. English has
-- one phrase for the other reading and it is a different construction —
-- the passive "exiled by", which the corpus writes ONCE against a
-- hundred seventy-five "exiled with".
failing "LinkSource"
  badExiledWithDescribedSource : Predicate [] Object
  badExiledWithDescribedSource = ExiledWith (Macros.a Macros.artifact)

-- The linked cards are IN EXILE, which [CR#607.2a] says in as many words
-- — the second ability "refers only to cards in the exile zone that were
-- put there" by the first — so the phrase cannot also say its referent
-- is controlled by somebody: [CR#109.4] gives an object that is neither
-- on the stack nor on the battlefield no controller at all. Zero corpus
-- lines write "a card you control exiled with …".
failing "ZoneCoherent"
  badExiledWithControlled : Predicate [] Object
  badExiledWithControlled = And [ControlledBy You, Macros.exiledWithThisArtifact]

-- …and the battlefield type word is refused by the same table, which is
-- what keeps "a creature card exiled with this creature" (real, and
-- Sisters of Stone Death's own phrase) apart from "an attacking creature
-- exiled with this creature" (not): the card word travels with the
-- linkage, the status word does not.
failing "ZoneCoherent"
  badExiledWithAttacking : Predicate [] Object
  badExiledWithAttacking = And [Attacking, Macros.exiledWithThisArtifact]

-- The linkage read does not NEGATE. "Not exiled with" is zero corpus
-- lines against a hundred seventy-five positive ones, and the reason is
-- what the phrase is for: a source-keyed group is named to be acted on,
-- and the cards outside it are described by another zone or another
-- phrase rather than by this one turned inside out.
failing "Negatable"
  badNegatedExiledWith : Predicate [] Object
  badNegatedExiledWith = Not Macros.exiledWithThisArtifact

-- ===== What a card may be made of, what a spell carrier may hold, and
-- ===== what a participle may be marked with

-- A permanent card's text is never a SPELL ability. [CR#113.3a] defines
-- the category by when it is followed — "while an instant or sorcery
-- spell is resolving" — and a creature card's text is never that. This
-- is the container's central gate seen from the side the rule states.
failing "CardText"
  badSpellAbilityOnPermanent : Card
  badSpellAbilityOnPermanent =
    Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
         [Spell Macros.drawACard] (Just (1, 1))

-- …and a spell card's text is not a STATIC ability, on the same rule's
-- other clause: [CR#113.3a] admits one only if it "fits the criteria
-- described" [CR#113.6], and that rule says abilities of an instant or
-- sorcery "usually function only while that object is on the stack"
-- where every `StaticEffect` row here establishes a continuous effect on
-- the battlefield.
failing "CardText"
  badStaticOnSorcery : Card
  badStaticOnSorcery =
    Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
         [Static (Gets (AllOf Macros.creatureYouControl) 1 1)] Nothing

-- The keyword row is shut on a spell card by MEASUREMENT rather than by
-- rule: the seven keywords this file carries are all [CR#702] abilities of
-- a permanent in combat, and no instant or sorcery in the supported corpus
-- is printed with one as a bare line — zero, for all seven.
failing "CardText"
  badKeywordOnInstant : Card
  badKeywordOnInstant =
    Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
         [KeywordAbility Flying] Nothing

-- The ACTIVATED row is open on a spell card — cycling is one, printed on
-- sorceries — but not for every cost: [CR#113.6j] lets an activated
-- ability function off the battlefield exactly when its cost can be paid
-- there, and [CR#110.4] never puts an instant or sorcery card on the
-- battlefield. "{T}" means "Tap this permanent" [CR#107.5], so the line
-- is one no sorcery could ever activate, and zero Instant or Sorcery
-- cards print one. (`cycling`, whose cost is a symbol run and a discard,
-- is the shape that passes.)
failing "CardText"
  badTapSorcery : Card
  badTapSorcery =
    Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
         [Activated TapSymbol Macros.drawACard] Nothing

-- A creature card writes its two numbers ([CR#208.1] — "a creature card
-- has two numbers separated by a slash printed in its lower right
-- corner"), which is `tokenPtOk`'s demand at the printed card.
failing "CardPt"
  badCreatureCardNoPt : Card
  badCreatureCardNoPt =
    Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing

-- A land card writes NO mana cost ([CR#202.1b]: "some objects have no
-- mana cost. This normally includes all land cards"), the absence being
-- an unpayable cost [CR#118.6] rather than an omission.
failing "CardCost"
  badLandWithManaCost : Card
  badLandWithManaCost =
    Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing

-- The SUPERTYPE field carried no witness at all, so a word could be
-- printed twice. [CR#205.4b] makes a supertype a property an object HAS
-- or LACKS — one that "gains or loses a supertype … retains any OTHER
-- supertypes it had" — so "Legendary Legendary Creature" is one fact
-- written twice, the colors' refusal at the catalog list beside it.
failing "CardSupers"
  badDuplicateSupertype : Card
  badDuplicateSupertype =
    Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
         [] (Just (1, 1))

-- A permanent type and a spell type do not share a line. [CR#110.4]
-- says "instant and sorcery cards can't enter the battlefield and thus
-- can't be permanents" and [CR#110.4a] lists the six that can, so this
-- line names a card that would have to be a permanent and not be one.
-- The ORDER check could not catch it, and that is the point: the ranks
-- put the spell types last, so [Land, Creature, Instant] ascends
-- perfectly and combination legality is a second question.
failing "CardLine"
  badMixedPermanentSpellLine : Card
  badMixedPermanentSpellLine =
    Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1))

-- A card's type line says SOMETHING — [CR#205.1] has it contain "the
-- card's card type(s)" without qualification, where the subtypes and
-- supertypes are there "if applicable".
failing "CardLine"
  badCardNoTypes : Card
  badCardNoTypes =
    Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing

-- …and it writes them in the printed ORDER, which is `typesOrdered`'s
-- rank at its third reader: "artifact creature" is five hundred
-- ninety-four lines and "creature artifact" none.
failing "CardLine"
  badCardTypeOrder : Card
  badCardTypeOrder =
    Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Artifact]) []
         (Just (2, 2))

-- A clause cannot GRANT a spell ability, and this refusal is a category
-- error rather than the quotation gap its three siblings carry: a
-- `Gains` clause grants to a permanent on the battlefield and a spell
-- ability is something an instant or sorcery spell has while it resolves
-- ([CR#113.3a]).
failing "Grantable"
  badGainsSpellAbility : Effect []
  badGainsSpellAbility =
    Macros.gains (Macros.target Macros.creature) (Spell Macros.drawACard) Nothing

-- The countering's complement is a SPELL, and the zone is what says so
-- ([CR#112.1] — "a spell is a card on the stack"). A battlefield
-- permanent has already resolved and there is nothing left to cancel.
failing "OnStack"
  badCounterPermanent : Effect []
  badCounterPermanent = Macros.counterSpell (Macros.target Macros.creature)

-- Cancelling somebody else's spell is not a PAYMENT. [CR#602.1a] makes
-- an activation cost what the activator pays, and the cost table's
-- measurement is the same one finding 159 made of destroy: zero corpus
-- lines write a counter before a colon.
failing "CostAction"
  badCounterAsCost : Ability
  badCounterAsCost =
    Activated (Do (Macros.counterSpell (Macros.target Macros.spell))) Macros.drawACard

-- The stack is not a place one plays a card FROM: [CR#112.1] makes an
-- object there a spell, and a spell has already been cast. That is the
-- battlefield refusal one step earlier.
failing "PlaySource"
  badPlayFromStack : Effect []
  badPlayFromStack =
    Continuously (MayPlay You (Macros.a Macros.spell)) (Just Macros.thisTurn)

-- "Cast" excludes the LAND, and the rule is [CR#305.9]: "if an object is
-- both a land and another card type, it can be played only as a land. It
-- can't be cast as a spell." The general verb is what those lines write
-- ("You may play lands from your graveyard").
failing "CastableTy"
  badCastALand : Effect []
  badCastALand =
    Continuously (MayPlay You (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)]))
                             {verb = Cast})
                 (Just Macros.thisTurn)

-- A written source phrase must AGREE with what the complement already
-- says: the permission's two ways of naming a zone are one fact, and
-- `zoneFits` is the same silence-is-no-evidence reading every other
-- zone demand uses.
failing "PlaySource"
  badPlayFromWrongZone : Effect []
  badPlayFromWrongZone =
    Continuously (MayPlay You (Macros.a (And [Macros.creature, InZone Macros.exileZ]))
                             {from = Just (Macros.graveyardOf You)})
                 (Just Macros.thisTurn)

-- Nothing MOVES to the stack. [CR#601.2a] puts a card there as the first
-- step of casting it, which is an action a player takes and not a
-- placement a sentence writes — core excludes the same destination by
-- name (`Destination`'s `exclude(Library, Stack)`).
failing "DestOk"
  badMoveToStack : Effect []
  badMoveToStack = Move (Macros.target Macros.creature) Macros.stackZ

-- The battlefield destination asks about its PATIENT and not only about
-- its own phrase: [CR#110.4] says "instant and sorcery cards can't enter
-- the battlefield and thus can't be permanents" and [CR#110.4a] lists
-- the six types that can, so a graveyard instant has no placement.
-- (`Placeable` reads the PROJECTED head type, so the untyped phrase
-- still places — Oblivion Ring's "return the exiled card to the
-- battlefield" writes no type word, and over-refusal is the one
-- direction these gates may not err in.)
failing "Placeable"
  badInstantOntoBattlefield : Effect []
  badInstantOntoBattlefield =
    Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ]))

-- …and no cost component places anything on the battlefield. Every
-- zone-change verb the corpus writes before a colon REMOVES or
-- DOWNGRADES — sacrifice three hundred forty-one components, discard
-- sixty-eight, exile twenty-eight, "Return … to its owner's hand" nine,
-- and the six "Put …" costs name a graveyard, the top of a library or a
-- counter — against zero battlefield entries. [CR#601.2h] pays the cost
-- to activate; a placement is what the ability buys.
failing "CostAction"
  badMoveOntoBattlefieldAsCost : Ability
  badMoveOntoBattlefieldAsCost =
    Activated (Do (Macros.putOntoBattlefield (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))))
              Macros.drawACard

-- "Unless" IS the negation, so it takes a negated condition and spells
-- the positive underneath. A positive condition under the word would be
-- the negation written twice, which no line writes.
failing "MarkingOk"
  badUnlessOnPositive : Ability
  badUnlessOnPositive =
    Static (Conditionally (Exists (And [Macros.artifact, ControlledBy You]))
                          (Cant Macros.thisCreature Attack Agent)
                          {marking = Unless})

-- The end-of-combat header takes no POSSESSOR. Zero lines write "at your
-- end of combat" or its neighbours; the four "end of combat on your …"
-- lines are [CR#511.2]'s other reading, the phase-endpoint duration that
-- `EndOf Combat` already spells.
failing "PartTriggerable"
  badTriggerAtYourEndOfCombat : Ability
  badTriggerAtYourEndOfCombat =
    Triggered At (BeginningOf EndOfCombat (Just Yours)) Macros.drawACard

-- …and the sixth part names no duration endpoint at all, [CR#511.2]
-- putting "until end of combat" at the end of the combat PHASE, which is
-- `Combat`'s cell. One phrase, one slot.
failing "SpanOk"
  badUntilEndOfCombatStep : Effect []
  badUntilEndOfCombatStep =
    Macros.gets (Macros.target Macros.creature) 1 1 (Just (Until (StartOf EndOfCombat Nothing)))

-- The participle's two surfaces are not interchangeable per verb.
-- "Destroyed this way" is fifty-one lines and "the destroyed [noun]" is
-- zero, so the destroy row writes the deictic only — which is what makes
-- this a table and not a free slot.
failing "VerbedMarkingOk"
  badDestroyedAttributive : Effect []
  badDestroyedAttributive =
    Sequentially [Macros.destroy (Macros.target Macros.creature),
                  Macros.exile (TheVerbed Destroy CardW {marking = Attributive})]

-- The controller relation still seeds the BATTLEFIELD, and the stack
-- row's arrival is what makes that a measured refusal rather than a
-- caveat: [CR#109.4] gives stack objects a controller too, so "target
-- spell you control" is real English (seventy-one lines, plus six for an
-- opponent and two negated), and the phrase is refused because
-- `seedZone` names one zone where the relation constrains to two. Every
-- one of those lines is copy machinery, which is why the shape is
-- ledgered rather than repaired here.
failing "ZoneCoherent"
  badControlledSpell : Predicate [] Object
  badControlledSpell = And [Macros.spell, ControlledBy You]

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

-- The six unattested values, one pin each so a future broadening fails
-- at the VALUE gate with the failure attributable to nothing else: the
-- subject and context are ordinary throughout. The zero counts are
-- exact and individually queried ("becomes flipped/unflipped/face
-- up/face down/phased in/phased out" — zero supported lines apiece);
-- the operations exist under their own verbs ([CR#708] for the face
-- pair, [CR#710] for flip, [CR#702.26] for phasing) and are the
-- ledger's. Unflipped is stronger still: flipping is one-way, so the
-- transition cannot happen ([CR#710.4]).
failing "StatusEventVal"
  badBecomesFlipped : Ability
  badBecomesFlipped =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) Flipped) Macros.drawACard

failing "StatusEventVal"
  badBecomesUnflipped : Ability
  badBecomesUnflipped =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) Unflipped) Macros.drawACard

failing "StatusEventVal"
  badBecomesFaceUp : Ability
  badBecomesFaceUp =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceUp) Macros.drawACard

failing "StatusEventVal"
  badBecomesFaceDown : Ability
  badBecomesFaceDown =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceDown) Macros.drawACard

failing "StatusEventVal"
  badBecomesPhasedIn : Ability
  badBecomesPhasedIn =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedIn) Macros.drawACard

failing "StatusEventVal"
  badBecomesPhasedOut : Ability
  badBecomesPhasedOut =
    Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedOut) Macros.drawACard

-- [CR#603.2b] keeps `At` for phases and steps: "At a creature becomes
-- tapped" is unwritten, as it is for every other object event.
failing "TriggerWordOk"
  badAtBecomesTapped : Ability
  badAtBecomesTapped =
    Triggered At (BecomesStatus (Macros.a Macros.creature) Tapped) Macros.drawACard

-- Trigger-only, reader by reader — `badInterceptEnters`'s shape at the
-- new event. Nothing intercepts a becomes-tapped: no "if [it] would
-- become tapped, … instead" line exists.
failing "Interceptable"
  badInterceptBecomesTapped : Effect []
  badInterceptBecomesTapped =
    Macros.ifWouldInstead (BecomesStatus (Macros.target Macros.creature) Tapped)
                   (Macros.exile It) (Just Macros.thisTurn)

-- …and the [CR#610.3] rider does not wait for one: "exile it until
-- [something] becomes untapped" is written zero times, the rider's
-- whole corpus being the departure.
failing "Holdable"
  badHeldUntilBecomesUntapped : Effect []
  badHeldUntilBecomesUntapped =
    Macros.exileUntil (Macros.target Macros.creature) (BecomesStatus (Macros.a Macros.creature) Untapped)

-- …nor does the delayed clause: the delayed family stays the end-step
-- beginning, the departure, and the death.
failing "Awaitable"
  badDelayedOnBecomesTapped : Effect []
  badDelayedOnBecomesTapped =
    Delayed (BecomesStatus (Macros.target Macros.creature) Tapped)
            (Macros.sacrifice You (That (TypeW Creature)))

-- No measured duration ends at a status transition: the tapped-STATE
-- span is "for as long as … remains tapped" ([CR#611.2b]), a condition
-- inside the for-as-long-as adverbial and not an event endpoint (forty
-- of forty corpus "remains tapped" lines; zero write "until … becomes
-- untapped" in a clause this grammar spells).
failing "SpanOk PtDelta"
  badGetsUntilBecomesUntapped : Effect []
  badGetsUntilBecomesUntapped =
    Macros.gets (Macros.target Macros.creature) 2 2
         (Just (UntilEvent (BecomesStatus Macros.thisCreature Untapped)))

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

-- the timed clause's subject stands on the battlefield, `Tap`/`Untap`'s
-- own demand at the new row ([CR#701.26a]'s zone, `badUntapGraveyard`'s
-- twin).
failing "OnBattlefield"
  badUntapNextGraveyard : Effect []
  badUntapNextGraveyard =
    DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) 1

-- "it" after two singular object introductions reaches two mentions and
-- resolves neither — the strict uniqueness gate, unchanged at the new
-- consumer.
failing "countOnes"
  badUntapNextAmbiguousIt : Effect []
  badUntapNextAmbiguousIt = Sequentially [Tap (Macros.target Macros.creature),
                                          Tap (Macros.target Macros.artifact),
                                          DoesntUntapNext It 1]

-- the count vocabulary is closed at the attested one and two: "next
-- three untap steps" is written zero times, and the table refuses it.
failing "NextUntapCount"
  badUntapNextThree : Effect []
  badUntapNextThree = DoesntUntapNext (Macros.target Macros.creature) 3
