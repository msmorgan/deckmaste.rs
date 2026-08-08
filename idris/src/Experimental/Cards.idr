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
                           Move (That CardW) battlefieldZ]

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (Through the Breach; Splice line elided — real Sneak
-- Attack says "the creature", a definite read this chapter doesn't mint)
-- — the hand-to-battlefield move RESTORES the typed carrier (the head
-- type was projected at introduction, never lost), the choice-determined
-- referent survives the delay [CR#603.7c], and the trailing adverbial
-- is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = Sequentially [may You (Move (a (And [creature, InZone (handOf You)])) battlefieldZ),
                                 gainsHaste (That (TypeW Creature)) Nothing,
                                 Delayed (BeginningOf EndStep Nothing) (sacrifice You (That (TypeW Creature)))]

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
  Activated (Compound [Mana [generic 1, pip Black, pip Red, pip Red], TapSymbol,
                       Do (sacrifice You thisLand)])
            (Sequentially [DealDamage It (Lit 3) (target AnyPlayer),
                           discardsACard (That PlayerW)])
            {window = Just AsSorcery}

-- "{R}, Sacrifice this artifact: It deals 2 damage to any target."
-- (Pyrite Spellbomb, first ability; the card's second ability elided) —
-- the smallest cost-antecedent pair: the sorted self-reference moved by
-- the cost is the only mention "It" can reach. Its mana half is written
-- now, which is what makes the pair a two-component cost rather than a
-- one-component one with a note.
pyriteSpellbomb : Ability
pyriteSpellbomb = Activated (Compound [Mana [pip Red], Do (sacrifice You thisArtifact)])
                            (DealDamage It (Lit 2) (target AnyTarget))

-- "Target player sacrifices a creature of their choice." (Diabolic
-- Edict) — the declarative clause: the target subject introduces, the
-- verb phrase is typed after it, and "of their choice" is the marked
-- own-choice method on the indefinite (the corpus has no bare "Target
-- player sacrifices a creature").
diabolicEdict : Effect []
diabolicEdict = sacrifice (target AnyPlayer) (aTheirChoice creature)

-- "Each player sacrifices a creature of their choice." (Innocent
-- Blood) — a group subject: the same clause shape over "each player".
innocentBlood : Effect []
innocentBlood = sacrifice (Each AnyPlayer) (aTheirChoice creature)

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
                                 Move It battlefieldZ]

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
                           Delayed (BeginningOf EndStep Nothing) (Move (That CardW) battlefieldZ)]

-- "Target creature gains flying until end of turn." (Jump) — the
-- duration as trailing-adverbial data ([CR#611.2a]), and the clause
-- under the envelope that carries it: the grant is the static effect,
-- "until end of turn" the span it lasts.
jump : Effect []
jump = gains (target creature) (KeywordAbility Flying) (Just untilEndOfTurn)

-- "Target creature gets +3/+3 until end of turn." (Giant Growth)
giantGrowth : Effect []
giantGrowth = gets (target creature) 3 3 (Just untilEndOfTurn)

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
  gets (target (And [Blocking, creature, ControlledBy You])) 10 0 (Just untilEndOfCombat)

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
  gains thisCreature (KeywordAbility Flying) (Just untilYourNextUpkeep)

-- "Return target creature card from your graveyard to the
-- battlefield. It gains haste until your next turn." (Bond of
-- Revival) — an owned-zone source and the cross-turn duration; the
-- return is just a Move, and "it" reads the retagged referent.
bondOfRevival : Effect []
bondOfRevival = Sequentially [Move (target (And [creature, InZone (graveyardOf You)])) battlefieldZ,
                              gainsHaste It (Just untilYourNextTurn)]

-- "When target creature dies this turn, return that card to the
-- battlefield under its owner's control." (Graceful Reprieve; "under
-- its owner's control" is a meaning-carrying elision — the override of
-- [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (Dies (target creature)) {span = Just ThisTurn}
                           (Move (That CardW) battlefieldZ)

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
continueSpell = Sequentially [Choose (TargetGroup (upTo 4) (And [creature, InZone (graveyardOf You)])),
                              Move Them battlefieldZ]

-- "Choose a color. Sudden Demise deals X damage to each creature of
-- the chosen color." (Sudden Demise) — a quality mention: the chosen
-- color enters the discourse like any mention ([CR#105.1]) and the
-- predicate-internal read demands it uniquely; X is the announced
-- cost variable ([CR#107.3a]).
suddenDemise : Effect []
suddenDemise = Sequentially [Choose (a (QualityNoun Color)),
                             DealDamage This XVal (Each (And [creature, OfChosen Color]))]

-- "Choose a creature type. Destroy all creatures that aren't of the
-- chosen type." (Kindred Dominance) — the set-level "all" determiner
-- over a negated quality read.
kindredDominance : Effect []
kindredDominance = Sequentially [Choose (a (QualityNoun CreatureType)),
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
voyagerStaff : Ability
voyagerStaff = Activated (Compound [Mana [generic 2], Do (sacrifice You thisArtifact)])
                         (Sequentially [exile (target creature),
                                          Delayed (BeginningOf EndStep Nothing) (Move (TheVerbed Exile CardW) battlefieldZ)])

-- "{3}{R}, Sacrifice an artifact: Bosh deals damage equal to the
-- sacrificed artifact's mana value to any target." (Bosh, Iron Golem;
-- {3}{R} and its Trample line elided; the self-name is `This`) — the
-- TYPE-word participle noun: the referent is a graveyard card NOW,
-- but "artifact" describes it under the verb — checked on the
-- time-stable projected head type, the axis a declared type word
-- lives on (finding 27).
boshIronGolem : Ability
boshIronGolem = Activated (Compound [Mana [generic 3, pip Red],
                                     Do (sacrifice You (a (HasType Artifact)))])
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
pyromancy : Ability
pyromancy = Activated (Compound [Mana [generic 3],
                                 Do (discards You (aAtRandom (InZone handZ)))])
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
-- destination is the bare-scoped hand (`handZ`): [CR#400.3] admits no
-- other hand, so
-- the possessive is derived surface, never stored (finding 34).
unsummon : Effect []
unsummon = Move (target creature) handZ

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
rawExile = Composite Exile (Move (target creature) exileZ) {ok = ExileB}

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
infiltrate = cantBeBlocked (target creature) (Just thisTurn)

-- "Target creature can't attack this turn." (Change of Heart; its
-- Buyback line elided — a keyword rider, as with Cascade and Splice)
-- — the active voice of the same clause, checked at declare attackers
-- instead ([CR#508.1c]).
changeOfHeart : Effect []
changeOfHeart = cantAttack (target creature) (Just thisTurn)

-- "Blindblast deals 1 damage to target creature. That creature can't
-- block this turn." (Blindblast; "Draw a card." elided — no draw
-- vocabulary, ledgered) — the restriction takes an ANAPHORIC subject
-- like any other clause: the demonstrative reads the mention the damage
-- clause introduced, and the deontic needs nothing of its own for it.
blindblast : Effect []
blindblast = Sequentially [DealDamage This (Lit 1) (target creature),
                           cantBlock (That (TypeW Creature)) (Just thisTurn)]

-- "Any number of target creatures can't block this turn." (Blinding
-- Flare; its Strive cost-modification line elided) — the subject is
-- PLURAL, and the clause demands no grammatical number: a restriction
-- ranges over whatever its subject phrase describes, one creature or a
-- group announced at once.
blindingFlare : Effect []
blindingFlare = cantBlock (TargetGroup anyNumber creature) (Just thisTurn)

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
overload = If (destroy (target artifact))
              (CompareAmt (manaValueOf It) OrLess (Lit 2))
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
  Sequentially [gets (target (And [creature, Attacking])) 3 3 (Just untilEndOfTurn),
                If (gains It (KeywordAbility Trample) (Just untilEndOfTurn))
                   (itsA (And [artifact, creature]))
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
  Sequentially [DealDamage This (Lit 4) (target (And [creature, ControlledBy anOpponent])),
                If (DealDamage This (Lit 2)
                               (Each (And [creature, Other, ControlledBy (That PlayerW)])))
                   (Exists (And [creature, ControlledBy You,
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
  mayThen You (sacrifice You (a creature)) (discardsACard (Each Opponent))

-- "You may sacrifice an artifact. If you do, destroy target artifact or
-- creature." (Daretti, Ingenious Iconoclast's [−1]; the loyalty cost is
-- the ability layer's) — the taken branch reading FORWARD: the arm
-- announces its own target, which the branchless may could not have
-- carried on the same sentence.
darettisMinusOne : Effect []
darettisMinusOne =
  mayThen You (sacrifice You (a artifact))
              (destroy (target (Or [artifact, creature])))

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
  mayThenElse You (sacrifice You (a creature))
                  (PutCounters (Lit 1) PlusOnePlusOne thisCreature)
                  (RemoveCounters (Lit 1) PlusOnePlusOne thisCreature)

yawgmothDemon : Effect []
yawgmothDemon =
  mayElse You (sacrifice You (a artifact))
              (Sequentially [Tap thisCreature, DealDamage This (Lit 2) You])

-- ===== Objects made and counters moved: the creation chapter =====

-- "Create two 1/1 white Soldier creature tokens." (Raise the Alarm; its
-- Flash line elided, a keyword rider) — the flagship, and the whole card.
-- Every slot the bundle has in one phrase at its simplest: a written
-- count, a power and toughness, one color, one subtype, one card type.
-- The count is the ordinary amount vocabulary and the plural noun reads
-- off it (`amtPlur`).
raiseTheAlarm : Effect []
raiseTheAlarm = create (Lit 2) (creatureTok 1 1 [White] [Soldier])

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
additiveEvolution = Sequentially [create (Lit 1) (creatureTok 0 0 [Green, Blue] [Fractal]),
                                  PutCounters (Lit 3) PlusOnePlusOne It]

-- "When this creature enters, create a 1/1 colorless Thopter artifact
-- creature token with flying." (Aviation Pioneer; the trigger header
-- elided) — the COLORLESS bundle, where the empty color list is the word
-- ([CR#105.2c]); the COMPOUND type line, whose head noun is the last word
-- ("artifact creature"); and the with-clause over chapter seventeen's
-- keyword vocabulary unchanged.
aviationPioneer : Effect []
aviationPioneer =
  create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [Thopter] [Artifact, Creature])
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
  createTappedAttacking (Lit 1)
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
  Sequentially [If (create (Lit 1) (creatureTok 0 0 [Black] [Zombie, Army]))
                   (notSo (Exists (And [HasSubtype Army, creature, ControlledBy You])))
                   Nothing,
                Choose (a (And [HasSubtype Army, creature, ControlledBy You])),
                PutCounters (Lit 2) PlusOnePlusOne (That (TypeW Creature)),
                If (becomes It (subtypesOnly [Zombie]) Nothing)
                   (itIsntA (HasSubtype Zombie))
                   Nothing]

-- "Put a +1/+1 counter on target creature." (Battlegrowth) — the
-- counter flagship, and the whole card. Agent-silent ([CR#122.1]; core's
-- `PutCounters` carries no actor slot either) and battlefield-bound like
-- every other verb that touches a permanent.
battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) PlusOnePlusOne (target creature)

-- "{3}, {T}: Remove a -1/-1 counter from target creature."
-- (Chainbreaker; its mana and {T} cost elided as Icy Manipulator's are,
-- and its enters-with line with them — a replacement) — the removal
-- twin, and the other stat counter, so the two first-class kinds are
-- spelled end to end by real cards.
chainbreaker : Effect []
chainbreaker = RemoveCounters (Lit 1) MinusOneMinusOne (target creature)

-- "[−2]: Tap target creature. Put two stun counters on it." (Kaito, Bane
-- of Nightmares; the loyalty cost is the ability layer's, as Daretti's
-- is, and the reminder gloss of [CR#122.1d] with it) — the NAMED counter
-- kind's positive: a kind earns its row where a corpus line writes it as
-- a one-shot put, which stun is and charge, age, and quest are not.
kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [Tap (target creature),
                                      PutCounters (Lit 2) Stun It]

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
  Activated (Compound [TapSymbol, Do (sacrifice You thisArtifact)])
            (Sequentially [PutCounters (Lit 1) PlusOnePlusOne
                                         (target (And [creature, Not artifact])),
                             becomes (That (TypeW Creature)) (typesOnly [Artifact]) Nothing])

-- "{U}: Target creature becomes an artifact in addition to its other
-- types until end of turn." (Neurok Transmuter, first ability; its mana
-- cost and second ability elided) — the same clause with the adverbial
-- the construction does write, which is the cell that split `BothGrants`:
-- eighteen corpus lines end a bare type addition at end of turn and not
-- one ends one at end of combat.
neurokTransmuter : Effect []
neurokTransmuter = becomes (target creature) (typesOnly [Artifact]) (Just untilEndOfTurn)

-- "Target creature can't block this turn and becomes a Coward in
-- addition to its other types until end of turn." (Coward // Killer's
-- Coward half; its "Time travel." line elided, a keyword rider, and the
-- coordination transcribed as a two-clause sequence — the Arc Trail
-- stand-in) — the card that settles the current-turn split from inside
-- ONE sentence: the restriction takes "this turn" and the type addition
-- takes "until end of turn", the two adverbials chapter seventeen
-- measured apart, written here by one writer in one breath.
cowardKiller : Effect []
cowardKiller = Sequentially [cantBlock (target creature) (Just thisTurn),
                             becomes (That (TypeW Creature)) (subtypesOnly [Coward])
                                     (Just untilEndOfTurn)]

-- "Target attacking creature that isn't a Demon" — the negated subtype
-- word, widened from Clavileño, First of the Blessed's "target attacking
-- Vampire that isn't a Demon" (a named stand-in: Vampire has no row, and
-- the point is the NEGATION, not the head). This is why the subtype word
-- projects no card type: `negTypesOf` reads a negated member's projected
-- head, so a Creature projection here would read "that isn't a Demon" as
-- "noncreature" and refuse a phrase oracle writes.
clavilenoPhrase : Predicate [] Object
clavilenoPhrase = And [creature, Attacking, Not (HasSubtype Demon)]

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
  Sequentially [drawACard,
                If (Sequentially [losesLife (Each Opponent) (Lit 2),
                                  gainsLife You (Lit 2)])
                   (Exists (And [HasSubtype Demon, ControlledBy You]))
                   (Just (losesLife You (Lit 2)))]

-- ===== Cards drawn, modes chosen, and what a choice leaves behind: the composite-glue chapter =====

-- "Draw two cards." (Divination; the whole card) — the draw verb's
-- flagship, and the plainest shape it has: the imperative's unpronounced
-- subject spelled as `You`, and a written numeral in the count slot.
divination : Effect []
divination = drawCards 2

-- "Target player draws three cards." (Ancestral Recall; the whole card) —
-- the SUBJECTED form, and why the drawer is a slot rather than the
-- imperative's silent `You`: the same clause writes both.
ancestralRecall : Effect []
ancestralRecall = Draw (target AnyPlayer) (Lit 3)

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
collectiveUnconscious = Draw You (forEach creatureYouControl)

-- "Target creature gets +1/+1 until end of turn. Draw a card." (Killian's
-- Confidence; its graveyard-return trigger is a second ability) — the
-- commonest sentence in the corpus in the position it is almost always
-- written in, and the proof that it introduces nothing: the sequence ends
-- there because there is nothing left to say about the card drawn.
killiansConfidence : Effect []
killiansConfidence = Sequentially [gets (target creature) 1 1 (Just untilEndOfTurn),
                                   drawACard]

-- "Blindblast deals 1 damage to target creature. That creature can't
-- block this turn. Draw a card." (Blindblast; the whole card at last) —
-- the RETRO-OPEN of chapter fifteen's elision, which was elided for want
-- of the draw verb and nothing else.
blindblastWhole : Effect []
blindblastWhole = Sequentially [DealDamage This (Lit 1) (target creature),
                                cantBlock (That (TypeW Creature)) (Just thisTurn),
                                drawACard]

-- "{2}, Discard this card: Draw a card." — cycling's expansion
-- ([CR#702.29a]) with its EFFECT written, which `cyclingCost` above could
-- not have: the mana half of the cost stays elided as every mana cost
-- does, and the discard is the same term that justifies `DiscardOk`'s
-- bare-`This` row.
cycling : Ability
cycling = Activated (Compound [Mana [generic 2], Do (discards You This)]) drawACard

-- "Choose one — • Abrade deals 3 damage to target creature. • Destroy
-- target artifact." (Abrade; the whole card) — the modal flagship, at the
-- headcount four hundred seventy cards write. The two modes are typed in
-- the SAME discourse and not in one another's: neither reads the other,
-- and each announces its own targets only if it is chosen ([CR#700.2c]).
abrade : Effect []
abrade = chooseOne [DealDamage This (Lit 3) (target creature),
                    destroy (target artifact)]

-- "Choose two — • Destroy all artifacts. • Destroy all enchantments. •
-- Destroy all creatures with mana value 3 or less. • Destroy all
-- creatures with mana value 4 or greater." (Austere Command; the whole
-- card) — a counted headcount over four modes, with the comparison
-- chapter's bound inside two of them. Two of four, never two of two:
-- a headcount that fixes the whole list instructs no choice at all
-- (`ModesFit`, `badModalFixedWhole`).
austereCommand : Effect []
austereCommand =
  chooseTwo [destroy (AllOf artifact),
             destroy (AllOf enchantment),
             destroy (AllOf (And [creature, Compare ManaValue OrLess (Lit 3)])),
             destroy (AllOf (And [creature, Compare ManaValue OrGreater (Lit 4)]))]

-- "Choose one or both — • Target creature gets -1/-1 until end of turn. •
-- Put a +1/+1 counter on target creature." (Azula Always Lies; the whole
-- card) — the one-to-two range under the word that names the whole of a
-- two-item list, and the sharpest evidence for the list-not-telescope
-- shape: BOTH modes write "target creature" and neither is the other's
-- "other", because each announces its own ([CR#700.2c,601.2c]). Written
-- as a sequence the second phrase would have to say "another".
azulaAlwaysLies : Effect []
azulaAlwaysLies =
  chooseOneOrBoth [gets (target creature) (-1) (-1) (Just untilEndOfTurn),
                   PutCounters (Lit 1) PlusOnePlusOne (target creature)]

-- "Choose one or more — • Destroy target artifact. • Destroy target
-- enchantment. • Destroy target land." (Rain of Thorns; the whole card) —
-- the OPEN top, whose word says the mode list is its own bound. Nineteen
-- cards write it and none over two modes: at two the word is "both".
rainOfThorns : Effect []
rainOfThorns = chooseOneOrMore [destroy (target artifact),
                                destroy (target enchantment),
                                destroy (target land)]

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
  chooseAnyNumber [discardsACard (Each AnyPlayer),
                   sacrifice (Each AnyPlayer) (aTheirChoice creature)]

-- "Choose an opponent. That player sacrifices a creature of their
-- choice." (Myrkul's Edict, the 1—9 face of its d20 roll; the roll is a
-- carrier this grammar does not spell, elided as a trigger header is) —
-- the choose clause as an INTRODUCER, and one mention serving two reads:
-- the next sentence's demonstrative subject, and the unique player
-- antecedent "of their choice" needs (`countChoosers`).
myrkulsEdict : Effect []
myrkulsEdict = Sequentially [Choose (a Opponent),
                             sacrifice (That PlayerW) (aTheirChoice creature)]


-- ===== The group complement, the control verb, and the batch =====

-- "Choose target creature you control. It deals damage equal to its
-- power to each other creature." (Nibelheim Aflame; the flashback line
-- and the graveyard-cast rider elided) — the anchored complement over a
-- BOUND mention: the domain is every creature, and the anchor is the one
-- the first sentence announced, read back exactly as "it" reads it.
nibelheimAflame : Effect []
nibelheimAflame =
  Sequentially [Choose (target creatureYouControl),
                DealDamage It (powerOf It) (Each (otherCreature It))]

-- "{5}{W}, {T}: Other creatures you control get +1/+1 until end of
-- turn." (War Screecher; the activation cost elided as every cost line
-- is) — the PLURAL complement, and the corpus's commonest anchor: the
-- SOURCE. `This` introduces no binding and needs none here, the anchor
-- being carried by the phrase rather than searched for in the
-- discourse, which is what the ledgered self-exclusion entry was
-- waiting on.
warScreecher : Effect []
warScreecher =
  gets (AllOf (otherCreatureYouControl thisCreature)) 1 1 (Just untilEndOfTurn)

-- "Enrage — Whenever this creature is dealt damage, put a +1/+1 counter
-- on each other creature you control." (Bellowing Aegisaur; the trigger
-- header elided as every trigger header is) — the DISTRIBUTIVE
-- complement, source-anchored.
bellowingAegisaur : Effect []
bellowingAegisaur =
  PutCounters (Lit 1) PlusOnePlusOne (Each (otherCreatureYouControl thisCreature))

-- "{B}, Remove a -1/-1 counter from this creature: Put a -1/-1 counter
-- on each other creature." (Carnifex Demon; the cost line elided) — the
-- unrestricted complement, and the sweep that would hit the source
-- without it.
carnifexDemon : Effect []
carnifexDemon =
  PutCounters (Lit 1) MinusOneMinusOne (Each (otherCreature thisCreature))

-- "Each other player discards a card." (Syphon Mind's first sentence;
-- the second, "You draw a card for each card discarded this way", waits
-- on the participant-subset read) — the complement at the PLAYER kind,
-- anchored to `You`: the kind index makes the anchor a player without a
-- gate, and [CR#102.1] makes "player" the word the exclusion applies to.
syphonMind : Effect []
syphonMind = discardsACard (Each otherPlayer)

-- "{2}{R}, {T}: This creature fights another target creature." (Brash
-- Taunter; the cost line elided) — the SINGULAR complement, spelled
-- "another": one word, one row, and the determiner deciding whether it
-- selects one or subtracts from a group. The self-exclusion the fight
-- family writes twenty-four times ("Polukranos fights another target
-- creature" is the same shape under a card name) and that the bare
-- `Other` could not reach, its anchor search reading bindings where the
-- source leaves none.
brashTaunter : Effect []
brashTaunter = Fights thisCreature (target (otherCreature thisCreature))

-- "{1}{G}, {T}: Target creature you control fights another target
-- creature." (Ulvenwald Tracker; the cost line elided) — the CONTRAST
-- positive, and the reason the two rows stay apart: this "another" is
-- the bare `Other`, whose anchor is the target announced before it
-- (the word [CR#115.4] names, and [CR#601.2c] is why it is needed at
-- all: two instances of "target" may otherwise be given the same
-- object), where Brash Taunter's is a referent the clause names. Same
-- word, same slot, different relation.
ulvenwaldTracker : Effect []
ulvenwaldTracker = Fights (target creatureYouControl) (target (And [creature, Other]))

-- "Gain control of target creature until end of turn." (Act of
-- Treason's first sentence; "Untap that creature. It gains haste until
-- end of turn." elided — the untap verb is the parked object-status
-- axis) — the control verb under the ordinary current-turn grant
-- adverbial, a hundred and eleven corpus lines.
actOfTreason : Effect []
actOfTreason = gainControl (target creature) (Just untilEndOfTurn)

-- "Dominate Monster — When this creature enters, gain control of target
-- creature for as long as you control this creature." (Mind Flayer; the
-- trigger header elided) — the FOR-AS-LONG-AS duration, opened on the
-- clause chapter eighteen ledgered it waiting for. [CR#611.2b]'s own
-- example is this shape (Master Thief's "gain control of target artifact
-- for as long as you control this creature"), and the condition is the
-- reference frame chapter eighteen already built.
mindFlayer : Effect []
mindFlayer =
  gainControl (target creature) (Just (ForAsLongAs (Matches thisCreature (ControlledBy You))))

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
    [gainsControl (ControllerOf (target creature)) thisCreature Nothing,
     gainsControl (ControllerOf thisCreature) (That (TypeW Creature)) Nothing]

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
grismold = Create (Each AnyPlayer) (Lit 1) (creatureTok 1 1 [Green] [Plant]) []

-- "Sparkmage's Gambit deals 1 damage to each of up to two target
-- creatures. Those creatures can't block this turn." (Sparkmage's
-- Gambit, whole) — the EACH-OF recipient and the plural read it leaves
-- behind, in one card. The amount is per member ("1 damage", and each
-- of the two takes one), the mention is the group's and stays plural,
-- and the next sentence reads it with the sorted plural demonstrative.
sparkmagesGambit : Effect []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (TargetGroup (upTo 2) creature)),
                cantBlock (Those (TypeW Creature)) (Just thisTurn)]

-- "[+1]: Put a +1/+1 counter on each of up to two target creatures."
-- (Ajani, Adversary of Tyrants; the loyalty cost and the card's other
-- abilities elided) — the same recipient under the counter verb, which
-- is what settles that the distributed recipient is not damage's alone.
-- Twenty-nine corpus lines write "each of up to two target creatures".
ajaniAdversaryOfTyrants : Effect []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) PlusOnePlusOne (EachOf (TargetGroup (upTo 2) creature))

-- "Fall of the Titans deals X damage to each of up to two targets."
-- (Fall of the Titans; its surge cost line elided) — the each-of
-- recipient over the damage CLASS word, which is the phrase [CR#115.4]
-- names and the corpus writes only under this determiner or under a
-- division.
fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This XVal (EachOf (TargetGroup (upTo 2) AnyTarget))

-- "Choose any number of target creatures. Put a +1/+1 counter on each of
-- them." (Nature's Panoply; its strive cost line elided — an
-- additional-cost-per-target rider the cost algebra will spell) — the
-- each-of determiner over a plural READ rather than over a fresh
-- mention. Thirty-seven corpus lines write "each of them".
naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Choose (TargetGroup anyNumber creature),
                PutCounters (Lit 1) PlusOnePlusOne (EachOf Them)]

-- "Arc Lightning deals 3 damage divided as you choose among one, two, or
-- three targets." (Arc Lightning, whole) — the DIVISION: one written
-- amount split over a group, the split announced as the spell is cast
-- ([CR#601.2d]) and held fixed against any later change of targets
-- ([CR#115.7f]). The enumerated range is how the phrase spells the
-- caster's choice of how many.
arcLightning : Effect []
arcLightning = dealsDivided This (Lit 3) (TargetGroup (oneThrough 3) AnyTarget)

-- "Forked Bolt deals 2 damage divided as you choose among one or two
-- targets." (Forked Bolt, whole) — the same structure at the narrower
-- range, which twelve corpus lines write.
forkedBolt : Effect []
forkedBolt = dealsDivided This (Lit 2) (TargetGroup (oneThrough 2) AnyTarget)

-- "Boulderfall deals 5 damage divided as you choose among any number of
-- targets." (Boulderfall, whole) — the UNBOUNDED division, and the
-- positive that retired the class word's ban at that quantity: eighteen
-- corpus lines write "among any number of targets", and the structure
-- they needed is this one.
boulderfall : Effect []
boulderfall = dealsDivided This (Lit 5) (TargetGroup anyNumber AnyTarget)

-- "When this creature enters, distribute two +1/+1 counters among one or
-- two target creatures you control." (Armament Corps; the trigger header
-- elided) — the division's OTHER verb. [CR#601.2d] and [CR#115.7f] name
-- "divide or distribute" as one mechanic over one pair of examples
-- ("damage or counters"), and this is the counter half: the same
-- structure, a different word, forty-six corpus lines.
armamentCorps : Effect []
armamentCorps =
  distributeCounters (Lit 2) PlusOnePlusOne
                     (TargetGroup (oneThrough 2) creatureYouControl)


-- ===== The ordered library, the exposure clause, and the partition =====

-- "Look at target player's hand." / "Draw a card." (Peek) — the peek,
-- and the hand is a ZONE rather than a group of cards: oracle never
-- writes "look at all cards in target player's hand". Two printed
-- lines, one term.
peek : Effect []
peek = Sequentially [lookAtHandOf (target AnyPlayer), drawACard]

-- "Look at the top four cards of your library. Put one of them into
-- your hand and the rest on the bottom of your library in any order."
-- (Impulse) — THE partition. The look assembles a group ([CR#701.20e]
-- exposes it without moving it, [CR#701.20b]), the partitive takes a
-- member out of that group, and "the rest" is the complement of what
-- was taken WITHIN it. The second sentence is one verb over two
-- complements, transcribed as the named two-clause stand-in the ledger
-- already parks Arc Trail's beside.
impulse : Effect []
impulse = Sequentially [ lookAt (topCards 4)
                       , Move (oneOf Them) handZ
                       , Move TheRest (onBottomIn AnyOrder)
                       ]

-- "Look at the top three cards of your library. Put one of them into
-- your hand and the rest on the bottom of your library in any order."
-- (Anticipate) — the same frame at a different count, which is what
-- makes it a frame.
anticipate : Effect []
anticipate = Sequentially [ lookAt (topCards 3)
                          , Move (oneOf Them) handZ
                          , Move TheRest (onBottomIn AnyOrder)
                          ]

-- "Reveal the top four cards of your library. … Put the rest into your
-- graveyard." — the REVEAL half of the same shape ([CR#701.20a]: shown
-- to every player, not just the looker), with the partitive spelled
-- over the demonstrative the corpus also writes ("one of those cards",
-- fifty-two lines). The middle sentence of the printed card uses the
-- among-restriction and is elided; the frame under test is the
-- assemble-then-partition one.
revealFourPartition : Effect []
revealFourPartition = Sequentially [ revealCards (topCards 4)
                                   , Move (oneOf (Those CardW)) handZ
                                   , Move TheRest graveyardZ
                                   ]

-- "{R}, {T}: Look at the top eight cards of your library. Exile four of
-- them …, then put the rest on top of your library in any order." — the
-- COUNTED partitive and the top placement, showing that neither the
-- part nor the remainder has to be one card. (The "at random" rider on
-- the exile is elided — `ChoiceMode`'s marking belongs to the
-- indefinite phrase, not to a partitive, and no partitive carries one
-- in this vocabulary yet.)
exileFourOfThem : Effect []
exileFourOfThem = Sequentially [ lookAt (topCards 8)
                               , exile (someOf 4 Them)
                               , Move TheRest (onTopIn AnyOrder)
                               ]

-- "{2}: Put the bottom card of your library into your graveyard." — the
-- bottom slice's ONE corpus witness (Grenzo, Dungeon Warden; the
-- conditional second sentence needs the power comparison and a
-- battlefield placement, elided). It is also the clean case for the
-- retag: a library card put into a public zone is readable afterwards
-- ([CR#400.7j]), which the graveyard destination makes true here.
grenzoBottomCard : Effect []
grenzoBottomCard = Move bottomCard graveyardZ

-- "Search your library for a land card, reveal it, put it into your
-- hand, then shuffle." (Sylvan Scrying) — the search frame whole: the
-- found object is the clause's own mention, the reveal is a SEPARATE
-- instruction because [CR#701.23e] says an unwritten reveal does not
-- happen, the placement moves it out of the library, and the shuffle is
-- the sequence tail. Note the order: the card leaves before the
-- library is randomized, which is [CR#701.24b]'s own arrangement.
sylvanScrying : Effect []
sylvanScrying = Sequentially [ searchLibraryFor land
                             , revealCards It
                             , Move It handZ
                             , shuffle
                             ]

-- "Search your library for a creature card, put that card onto the
-- battlefield, then shuffle." — the destination split: the same frame
-- with the found card placed on the battlefield and no reveal, which is
-- [CR#701.23e]'s default and a hundred seven corpus lines.
searchToBattlefield : Effect []
searchToBattlefield = Sequentially [ searchLibraryFor creature
                                   , Move It battlefieldZ
                                   , shuffle
                                   ]

-- "Target player mills ten cards." (Glimpse the Unthinkable) — the mill
-- clause whole, subject and all ([CR#701.17a]).
glimpseTheUnthinkable : Effect []
glimpseTheUnthinkable = millsCards (target AnyPlayer) 10

-- "Mill three cards." — the imperative mill, and the clause whose
-- milled group is a real mention: the graveyard is public, so
-- [CR#701.17c] lets later text find what was milled where a drawn card
-- (hand, hidden) could never be found. Here the read is the plural
-- demonstrative; the corpus's own reads are the among-restriction and
-- the "this way" participle, both ledgered.
millThenReadGroup : Effect []
millThenReadGroup = Sequentially [millCards 3, exile (Those CardW)]

-- "Look at the top card of your library. You may put that card into
-- your graveyard." — the singular slice with the offer, which is what
-- keeps the count vocabulary honest: at one the numeral is unwritten
-- and the mention is singular, so the read is "that card" and not
-- "them".
lookAtTopThenBin : Effect []
lookAtTopThenBin =
  Sequentially [lookAt topCard, may You (Move (That CardW) graveyardZ)]


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
  Sequentially [ DealDamage This (Lit 6) (target creature)
               , ifWouldInstead (Dies (That (TypeW Creature))) (exile It) (Just thisTurn)
               ]

-- "{1}: The next time you would draw a card this turn, this enchantment
-- deals 2 damage to any target instead." (Words of War; the activation
-- cost is the cost round's) — the OTHER multiplicity word, and the
-- event that forces it: a draw repeats, so an unlimited shield and a
-- single-use one are different effects and the writer has to say which
-- ([CR#614.11] gives draw replacement its own paragraph).
wordsOfWar : Effect []
wordsOfWar = nextTimeWouldInstead (Draws You)
                                  (DealDamage This (Lit 2) (target AnyTarget))
                                  (Just thisTurn)

-- "{1}: The next time you would draw a card this turn, you gain 5 life
-- instead." (Words of Worship) — the same shield with a replacement
-- that touches no object at all, which is what [CR#611.2c]'s
-- rules-modifying half looks like from the clause side.
wordsOfWorship : Effect []
wordsOfWorship = nextTimeWouldInstead (Draws You)
                                      (gainsLife You (Lit 5))
                                      (Just thisTurn)

-- "Prevent all combat damage that would be dealt this turn." (Fog, Holy
-- Day, Darkness, Root Snare — five lines, one sentence) — the shield
-- with no recipient at all, [CR#611.2c]'s own example of an effect that
-- modifies the rules of the game rather than any object.
fog : Effect []
fog = preventAll CombatOnly Everywhere (Just thisTurn)

-- "Prevent all damage that would be dealt to target creature this
-- turn." (Indestructible Aura, Shielded Passage) — the same shield with
-- a recipient, and the recipient is the damage clause's own row rather
-- than a second one.
indestructibleAura : Effect []
indestructibleAura = preventAll AnyDamage (shieldingIt (target creature)) (Just thisTurn)

-- "Prevent the next 3 damage that would be dealt to any target this
-- turn." (Shieldmate's Blessing) — [CR#615.7]'s numbered shield, the
-- one that is used up a damage at a time, over the damage class word.
shieldmatesBlessing : Effect []
shieldmatesBlessing =
  preventNext AnyDamage (Lit 3) (shieldingIt (target AnyTarget)) (Just thisTurn)

-- "Exile target creature an opponent controls until this creature
-- leaves the battlefield." (Banisher Priest's triggered body; the
-- trigger wrapper is the abilities layer's) — the O-Ring family's
-- eighty-six lines, and NOT a duration: [CR#610.3] makes this a
-- one-shot zone change that pairs with a second one-shot effect
-- returning the object, which is why the rider sits on the clause and
-- the `Duration` row over the same event stays unclaimed.
banisherPriest : Effect []
banisherPriest = exileUntil (target (And [creature, ControlledBy anOpponent]))
                            (Leaves thisCreature)

-- "[0]: Draw a card. If you control three or more artifacts, draw two
-- cards instead." (Tezzeret, Artifice Master) — the SELF-replacement
-- ([CR#614.15]), the "instead" with no "would" in front of it. The
-- replacement is a conditional clause and the node pairs it with what
-- it replaces, which is what makes "instead" mean anything at all.
tezzeretDrawTwo : Effect []
tezzeretDrawTwo =
  insteadOf drawACard
            (If (drawCards 2)
                (CompareAmt (CountOf (And [artifact, ControlledBy You]))
                            OrGreater (Lit 3))
                Nothing)

-- "{4}, {T}: Draw a card. If you control eight or more lands, draw two
-- cards instead." (Zimone, Quandrix Prodigy) — the same frame at a
-- different domain and count, which is what makes it a frame.
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  insteadOf drawACard
            (If (drawCards 2)
                (CompareAmt (CountOf (And [land, ControlledBy You]))
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
  Activated (Compound [Mana [generic 1, hybridPip Blue Black], UntapSymbol])
            (gets (target creature) (-2) 0 (Just untilEndOfTurn))

-- "{1}{S}: This creature gets +1/+0 until end of turn." (Phyrexian
-- Snowcrusher) — the snow symbol ([CR#107.4h]), which is neither a color
-- nor a type of mana and is why `ManaSymbol` needs a row for it rather
-- than a colorless pip with a note.
phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Activated (Mana [generic 1, SnowMana])
            (gets thisCreature 1 0 (Just untilEndOfTurn))

-- "{1}{C}: This creature gets +2/+1 until end of turn." (Havoc Sower;
-- its Devoid line and reminder gloss elided) — the COLORLESS pip
-- ([CR#107.4c]), which is what `ColorOrColorless` was ported for: the
-- token's empty color list could say "colorless" but no color could say
-- "{C}".
havocSower : Ability
havocSower =
  Activated (Mana [generic 1, colorlessPip])
            (gets thisCreature 2 1 (Just untilEndOfTurn))

-- "{1}{B}, Pay 2 life: Draw a card." (Erebos, God of the Dead; its other
-- three lines elided) — the life payment as a cost COMPONENT, which is
-- the same `ChangeLife` clause the sentence grammar already had, wearing
-- the cost frame's verb.
erebos : Ability
erebos = Activated (Compound [Mana [generic 1, pip Black], payLife You 2]) drawACard

-- "{1}{R/G}: Put a +1/+1 counter on this creature. Activate only as a
-- sorcery." (Savageborn Hydra; its double strike and enters-with lines
-- elided) — the activation WINDOW ([CR#602.5d]), and a second hybrid
-- cost under it.
savagebornHydra : Ability
savagebornHydra =
  Activated (Mana [generic 1, hybridPip Red Green])
            (PutCounters (Lit 1) PlusOnePlusOne thisCreature)
            {window = Just AsSorcery}

-- "{1}{G}: This creature gets +2/+2 until end of turn. Activate only
-- once each turn." (Basking Rootwalla; its madness line elided) — the
-- use LIMIT ([CR#602.5b]), the second restriction slot.
baskingRootwalla : Ability
baskingRootwalla =
  Activated (Mana [generic 1, pip Green])
            (gets thisCreature 2 2 (Just untilEndOfTurn))
            {limit = Just OncePerTurn}

-- "{3}, {T}: Draw a card. Activate only if you control a creature with
-- power 4 or greater." (Bonders' Enclave; its mana ability elided) — the
-- activation GUARD, and the point of the seam chapter eighteen left: the
-- condition is `Condition` unchanged, the postnominal comparison is
-- chapter sixteen's unchanged, and the carrier is new.
bondersEnclave : Ability
bondersEnclave =
  Activated (Compound [Mana [generic 3], TapSymbol])
            drawACard
            {guard = Just (Exists (And [creature, ControlledBy You,
                                        Compare Power OrGreater (Lit 4)]))}

-- "{W}, {T}: Remove a -1/-1 counter from target creature. If you do, you
-- gain 2 life." (Woeleecher) — the MANDATORY "if you do" ([CR#118.12]),
-- which is `May` with its offer emptied: no "may" is written anywhere on
-- the line, and the arm still asks whether the payment was started.
woeleecher : Ability
woeleecher =
  Activated (Compound [Mana [pip White], TapSymbol])
            (doThen (RemoveCounters (Lit 1) MinusOneMinusOne (target creature))
                    (gainsLife You (Lit 2)))

-- "Sacrifice this creature unless you pay {2}." (Molting Harpy; its
-- upkeep trigger shell and flying line elided) — the UNLESS family's
-- first positive, and it is [CR#118.12a]'s own rewrite rather than a
-- construction: "[Do something] unless [a player does something else]"
-- means "[A player may do something else]. If [that player doesn't], [do
-- something]", so the sentence is the may node read backwards, the
-- payment in the body and the main clause in the declined arm.
moltingHarpy : Effect []
moltingHarpy = mayElse You (Pay You (Mana [generic 2])) (sacrifice You thisCreature)

-- "Tap this creature unless you pay 1 life." (Carnophage; upkeep shell
-- elided) — the same frame with a LIFE payment, which is what makes the
-- verb's complement a whole `Cost` and not a mana amount.
carnophage : Effect []
carnophage = mayElse You (Pay You (payLife You 1)) (Tap thisCreature)

-- "Sacrifice this enchantment unless you discard a card." (Solitary
-- Confinement; upkeep shell and its other three lines elided) — the
-- ACTION half of the same family, where the offered payment is an
-- ordinary clause and no "pay" is written at all. Seventy-one corpus
-- lines pay an unless with a non-mana action.
solitaryConfinement : Effect []
solitaryConfinement = mayElse You (discardsACard You) (sacrifice You thisEnchantment)


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
  Activated (Mana [pip White, pip White])
            (create (Lit 1) (creatureTok 1 1 [White] [Soldier]))
            {limit = Just OncePerTurn}
            {guard = Just (notSo (Exists (And [creature, ControlledBy You])))}



-- ===== The triggered line and the static line: the ability chapter =====

-- "When this creature enters, draw a card." (Cloudkin Seer; its flying
-- line is a second ability, not part of this one) — the trigger corpus's
-- single commonest shape, two thousand four hundred seventy-three lines
-- of "When this [permanent] enters".
cloudkinSeer : Ability
cloudkinSeer = Triggered When (Enters This) drawACard

-- "Whenever a creature dies, you gain 1 life." (Moonlit Wake, whole) —
-- the same event under a DESCRIPTION, which is what moves the header
-- word: one object enters once and dies once, so a fixed subject takes
-- "When", and a description ranging over many takes "Whenever".
moonlitWake : Ability
moonlitWake = Triggered Whenever (Dies (a creature)) (gainsLife You (Lit 1))

-- "Whenever a creature you control dies, exile it." (Promise of
-- Tomorrow's first line) — the trigger's participant read BACK, and the
-- read is what shows the after-discourse is right: "it" finds a card in
-- a graveyard ([CR#603.6,700.4]), which is exactly what exile wants and
-- exactly what the interception's replacement must not have.
promiseOfTomorrow : Ability
promiseOfTomorrow = Triggered Whenever (Dies (a creatureYouControl)) (exile It)

-- "At the beginning of your upkeep, draw a card." (Staff of Nin's first
-- line) — the turn-part event, the one row whose header word is fixed
-- ([CR#603.2b]) and the only one with no noun phrase in it.
staffOfNin : Ability
staffOfNin = Triggered At (BeginningOf Upkeep (Just Yours)) drawACard

-- "Whenever this creature attacks, draw a card." (Library Larcenist,
-- whole) — the fixed subject taking "Whenever", which is the header
-- word tracking the EVENT's repeatability rather than the subject's
-- fixity: a creature attacks many times.
libraryLarcenist : Ability
libraryLarcenist = Triggered Whenever (Attacks This) drawACard

-- "Whenever this creature blocks, it deals 1 damage to target attacking
-- creature." (Elite Javelineer, whole) — the block event, and a trigger
-- that TARGETS ([CR#115.1d]), which is the line the static row cannot
-- write.
eliteJavelineer : Ability
eliteJavelineer =
  Triggered Whenever (Blocks This)
            (DealDamage This (Lit 1) (target (And [creature, Attacking])))

-- "Whenever this creature deals combat damage to a player, draw a
-- card." (Jhessian Thief's second line; its prowess line is a keyword)
-- — the two-slot event, and its recipient reads the damage clause's own
-- table for the third time.
jhessianThief : Ability
jhessianThief =
  Triggered Whenever (DealsCombatDamage This (a AnyPlayer)) drawACard

-- "When this creature enters, if you control an artifact, draw a card."
-- (Scholar of Stars, whole) — the INTERVENING "if" ([CR#603.4]), which
-- is `Condition` verbatim in a fourth carrier. The rule is what makes it
-- a slot rather than an ordinary trailing conditional: it applies "only
-- to an 'if' that immediately follows a trigger condition", and it is
-- checked twice, once as the event occurs and again on resolution.
scholarOfStars : Ability
scholarOfStars =
  Triggered When (Enters This) drawACard
            {intervening = Just (Exists (And [artifact, ControlledBy You]))}

-- "Creatures you control can't attack." (Glacial Chasm's third line) —
-- the durationless deontic `badStaticCant` has refused since chapter
-- seventeen, in the container it was waiting for.
glacialChasmCant : Ability
glacialChasmCant = Static (Cant (AllOf creatureYouControl) Attack Agent)

-- "Prevent all damage that would be dealt to you." (Glacial Chasm's
-- fourth line) — `badStandingPrevention`'s own sentence, one line below
-- the last on the same card.
glacialChasmShield : Ability
glacialChasmShield = Static (Prevents AnyDamage AllOfIt (shieldingIt You))

-- "If a creature an opponent controls would die, exile it instead."
-- (Misery's Shadow's first line) — `badStandingIntercept` as a
-- positive, and the standing interception chapter twenty-five refused
-- with the container named.
miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (a (And [creature, ControlledBy (a Opponent)])))
                     (exile It) Repeatedly)

-- "Metalcraft — Creatures you control get +3/+0 as long as you control
-- three or more artifacts." (Jor Kadeen, the Prevailer's second line;
-- the ability word has no rules meaning [CR#207.2c]) — the CONDITIONAL
-- static, [CR#611.3a]'s live re-check, and the nine-hundred-twelve-line
-- family chapter twenty-two measured and could not write.
jorKadeen : Ability
jorKadeen =
  Static (asLongAs (CompareAmt (CountOf (And [artifact, ControlledBy You]))
                               OrGreater (Lit 3))
                   (Gets (AllOf creatureYouControl) 3 0))

-- "This land enters tapped." (Abandoned Outpost's first line) — the
-- entry rider [CR#603.6d] calls a static ability in as many words, owed
-- from three sites since chapter twenty-five and paid here.
abandonedOutpost : Ability
abandonedOutpost = Static (entersTapped (AsType Land This))

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
            (Sequentially [exile topCard,
                           Continuously (MayPlay You It) (Just untilYourNextEndStep)])

-- "Until end of combat on your next turn, you may play that card."
-- (Brazen Cannonade's second line, whose raid trigger and postcombat
-- main-phase header are two turn-structure words this vocabulary does
-- not have) — the SECOND `Unclaimed` cell the permission claims, and
-- the one chapter twenty-two named by card when it wrote the cell's
-- reason down.
brazenCannonadePermission : Effect []
brazenCannonadePermission =
  Sequentially [exile topCard,
                Continuously (MayPlay You It) (Just (Until (EndOf Combat (Just Yours))))]

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
                           Delayed (BeginningOf EndStep Nothing) (sacrifice You It)]

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
                                  Delayed (BeginningOf EndStep Nothing) (DealDamage This (Lit 1) (target anyOtherTarget))]

-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
failing "countWord"
  badStaleCarrier : Effect []
  badStaleCarrier = Sequentially [exile (target creatureYouControl),
                                  Move (That (TypeW Creature)) battlefieldZ]

-- A hidden-zone cost mention is unreadable past the colon: the card
-- bounced to hand is not among the public survivors ([CR#400.2] —
-- hand is hidden; no [CR#400.7] exception reaches it, and
-- Soratami Cloudskater-family costs write no such read back).
failing "publicOnly"
  badHiddenCost : Ability
  badHiddenCost = Activated (Do (Move (a creature) handZ)) (Tap It)

-- Two cost moves leave two candidate antecedents ("Discard a card,
-- Sacrifice a creature: …" — Falkenrath Pit Fighter-family): a bare
-- "It" past the colon is ambiguous. Real costs of this shape read
-- back with definite descriptions (the participle reads of chapter
-- eight), never a bare pronoun. (The verb is exile — zone-blind —
-- so the pin isolates the ambiguity, not a zone gate.)
failing "countOnes"
  badTwoCostMentions : Ability
  badTwoCostMentions = Activated (Compound [Do (discardsACard You),
                                            Do (sacrifice You (a creature))])
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
  badDeadCreatureRead = Delayed (Dies (target creature)) {span = Just ThisTurn}
                                (Move (That (TypeW Creature)) battlefieldZ)

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
  badVerbedWrongVerb : Ability
  badVerbedWrongVerb = Activated (Do (discardsACard You))
                                 (Move (TheVerbed Sacrifice CardW) battlefieldZ)

-- The noun word misses on the type axis: an artifact was sacrificed,
-- so "the sacrificed creature" has no referent.
failing "countVerbed"
  badVerbedWrongNoun : Ability
  badVerbedWrongNoun = Activated (Do (sacrifice You (a (HasType Artifact))))
                                 (Move (TheVerbed Sacrifice (TypeW Creature)) battlefieldZ)

-- Two same-verb stamps leave the participle ambiguous — the same
-- strict uniqueness as every read (real costs of this shape name
-- distinct verbs, which is what the filter buys).
failing "countVerbed"
  badVerbedAmbig : Ability
  badVerbedAmbig = Activated (Compound [Do (sacrifice You (a creature)),
                                        Do (sacrifice You (a creature))])
                             (Move (TheVerbed Sacrifice CardW) battlefieldZ)

-- No participle reads a player, and the word is what says so: it
-- fixes the phrase's KIND exactly as it does for the demonstrative
-- (`kindOfW`), so "the discarded player" is not an object phrase a
-- move could take at all — the refusal lands at the kind rather than
-- waiting for the stamp scan to come back empty (finding 27).
failing "kindOfW PlayerW"
  badVerbedPlayerWord : Effect []
  badVerbedPlayerWord = Move (TheVerbed Discard PlayerW) battlefieldZ

-- Voyager Staff's shape with the bare demonstrative: two card
-- mentions (the sacrificed self, the exiled target) make "that card"
-- ambiguous — the participle is what real text switches to here.
failing "countWord"
  badBareCardRead : Ability
  badBareCardRead = Activated (Do (sacrifice You thisArtifact))
                              (Sequentially [exile (target creature),
                                             Delayed (BeginningOf EndStep Nothing) (Move (That CardW) battlefieldZ)])

-- ===== Chapter nine negatives: the audit round =====

-- Tapping takes a battlefield object [CR#701.26a]: a graveyard card
-- cannot be tapped.
failing "OnBattlefield"
  badTapGraveyard : Effect []
  badTapGraveyard = Tap (target (And [creature, InZone (graveyardOf You)]))

-- Only battlefield creatures fight [CR#701.14b]: a graveyard card
-- cannot.
failing "OnBattlefield"
  badFightGraveyard : Effect []
  badFightGraveyard = Fights (target (And [creature, InZone (graveyardOf You)]))
                             (target creature)

-- Only combatant types fight [CR#701.14a]: a land's TypeDef declares
-- no fight participation.
failing "FightParticipant"
  badFightLand : Effect []
  badFightLand = Fights (target (HasType Land)) (target creatureYouDontControl)

-- Dying is the battlefield-to-graveyard transition [CR#700.4]: an
-- already-graveyard card cannot die this turn. The demand is
-- `zoneFits`' as of chapter twenty-eight rather than `OnBattlefield`'s,
-- and the refusal is unchanged by the loosening: a phrase that STATES
-- the graveyard still contradicts the battlefield, while the phrase
-- that states nothing ("this creature", the trigger corpus's own
-- subject at two thousand four hundred seventy-three lines) passes on
-- its silence.
failing "ZoneFits"
  badDiesInGraveyard : Effect []
  badDiesInGraveyard = Delayed (Dies (target (And [creature, InZone (graveyardOf You)]))) {span = Just ThisTurn}
                               (Move (That CardW) battlefieldZ)

-- Damage reaches players and battlefield objects only [CR#120.1]:
-- the destroyed referent sits in the graveyard.
failing "DamageRecipient"
  badDamageGraveyardCard : Effect []
  badDamageGraveyardCard = Sequentially [destroy (target creature),
                                         DealDamage This (Lit 3) It]

-- A quality cannot take damage [CR#120.1].
failing "DamageRecipient"
  badDamageToColor : Effect []
  badDamageToColor = DealDamage This (Lit 1) (a (QualityNoun Color))

-- Damage goes to battles, creatures, planeswalkers, or players
-- [CR#120.1a]: a noncreature artifact takes none.
failing "DamageRecipient"
  badDamageArtifact : Effect []
  badDamageArtifact = DealDamage This (Lit 1) (target (HasType Artifact))

-- A phrase needs a positive head ([CR#105.1,608.2d]): "choose a
-- noncolor" heads nothing — Not is a modifier, never a head.
failing "Headed"
  badNegatedQualityHead : Effect []
  badNegatedQualityHead = Choose (a (Not (QualityNoun Color)))

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
  badConflictingZones = destroy (target (And [creature, InZone battlefieldZ, InZone graveyardZ]))

-- Targets are objects and players [CR#115.1]: "target color" is
-- unwritten — qualities are chosen, never targeted.
failing "Targetable"
  badTargetColor : Effect []
  badTargetColor = Choose (target (QualityNoun Color))

-- The one-shot stat modification takes a battlefield object: a dead
-- referent doesn't get +3/+3.
failing "ZoneFits"
  badGetsGraveyard : Effect []
  badGetsGraveyard = Sequentially [destroy (target creature),
                                   gets It 3 3 (Just untilEndOfTurn)]

-- A card never enters another player's hand [CR#400.3]: owned
-- destinations are owner-routed, so this is unwritable.
failing "DestOk"
  badMoveToTargetsHand : Effect []
  badMoveToTargetsHand = Move (target creature) (handOf (target AnyPlayer))

-- Only a battlefield permanent is destroyable [CR#701.8a].
failing "OnBattlefield"
  badDestroyGraveyard : Effect []
  badDestroyGraveyard = destroy (target (And [creature, InZone (graveyardOf You)]))

-- Discarding moves a card from a HAND [CR#701.9a]: a battlefield
-- creature is not discardable.
failing "DiscardOk"
  badDiscardBattlefield : Effect []
  badDiscardBattlefield = discards You (a creature)

-- The tag and its body agree: a Destroy-tagged exile would let
-- indestructible cant an exile [CR#701.8b,702.12b].
failing "TagBody"
  badDestroyTaggedExile : Effect []
  badDestroyTaggedExile = Composite Destroy (Move (target creature) exileZ)

-- The zone demand lives on the tag-body relation: the raw Composite
-- spelling proves what the macro proves [CR#701.8a].
failing "OnBattlefield"
  badCompositeDestroyGraveyard : Effect []
  badCompositeDestroyGraveyard =
    Composite Destroy (Move (target (And [creature, InZone graveyardZ])) graveyardZ) {ok = DestroyB}

-- An agentive tag cannot shed its actor [CR#701.21a]: the raw
-- Composite spelling of sacrifice is refused outright.
failing "NonAgentive"
  badAgentlessSacrifice : Effect []
  badAgentlessSacrifice = Composite Sacrifice (Move (a creature) graveyardZ) {ok = SacrificeB}

-- Discarding moves a hand card [CR#701.9a]: the demand rides the tag
-- relation, so a battlefield "discard" is unspellable under Does too
-- — and with it the forged stamp `TheVerbed Discard` would read.
failing "DiscardOk"
  badDoesDiscardBattlefield : Effect []
  badDoesDiscardBattlefield = Does You Discard (Move (a creature) graveyardZ) {tb = DiscardB}

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
  badGraveyardOfGroup = graveyardOf (TargetGroup (exactly 2) Opponent)

-- …and the shared zones take no possessor at all: [CR#400.1] gives
-- each player a library, a hand, and a graveyard and shares the rest,
-- so "your battlefield" is not a phrase. The refusal is the ZONE's,
-- not the possessor's — the noun here is impeccable.
failing "Possessable"
  badOwnedBattlefield : ZoneExpr []
  badOwnedBattlefield = ZoneAt Battlefield (OwnedBy You)

-- The minted dies-watcher is singular (Graceful Reprieve's shape);
-- plural watches wait for corpus evidence.
failing "OneOf"
  badDiesGroup : Effect []
  badDiesGroup = Delayed (Dies (TargetGroup (exactly 2) creature)) {span = Just ThisTurn}
                         (gainsLife You (Lit 1))

-- A bare type word denotes a permanent [CR#109.2], and what was
-- discarded left a HAND: "the discarded creature" is unwritten (the
-- corpus discard family reads "the discarded card" only) — the
-- stamp's at-verb frame refuses the type word.
failing "countVerbed"
  badDiscardedCreatureWord : Ability
  badDiscardedCreatureWord =
    Activated (Do (discards You (aAtRandom (And [creature, InZone handZ]))))
              (DealDamage This
                            (manaValueOf (TheVerbed Discard (TypeW Creature)))
                            (target AnyTarget))

-- "Of their choice" is a possessive pronoun: it demands a player
-- antecedent (a subject or one distributive group [CR#608.2d]) —
-- bare "Destroy a creature of their choice" is unwritten.
failing "countOnes Player"
  badUnboundTheirChoice : Effect []
  badUnboundTheirChoice = destroy (aTheirChoice creature)

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
  badNotOnBattlefield = And [creature, Not (InZone battlefieldZ)]

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
  badAttackingInHand = And [Attacking, InZone handZ]

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
  badAnyTargetInGraveyard = And [AnyTarget, InZone graveyardZ]

-- "Any target" is ITSELF the targeting form, so only the targeting
-- determiners admit it: "a any target" and "each any target" are
-- unwritable.
failing "AnyTargetFree"
  badAnyTargetUnderA : Noun [] Object
  badAnyTargetUnderA = a AnyTarget

-- …and the targeting determiner admits it only where the count is the
-- CASTER's to make: "any target" is the singular damage-class form
-- [CR#115.4] defines (bolt, Arc Trail, Pyromancy), and every plural
-- spelling the corpus writes runs from one — "up to two targets" (Fall
-- of the Titans), "one, two, or three targets" (Arc Lightning), "any
-- number of targets" (Boulderfall). A FIXED plural count is the one
-- thing it never writes: "two targets" as a phrase is zero lines and
-- "among two targets" is zero lines, so "two any targets" stays closed.
failing "AnyTargetAtCount"
  badGroupAnyTarget : Noun [] Object
  badGroupAnyTarget = TargetGroup (exactly 2) AnyTarget

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
  badAnyTargetEmbedded = a (And [creature, ControlledBy (ControllerOf (target AnyTarget))])

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
  badControlledInGraveyard = And [creature, ControlledBy You, InZone graveyardZ]

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
  badCrossZoneDisjunction = Or [InZone handZ, InZone graveyardZ]

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
    And [creature, Or [Attacking, Blocking], InZone graveyardZ]

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
-- was enough to let `And [this, InZone graveyardZ]` stand, a phrase
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
  badCantAttackLand = cantAttack (target land) (Just thisTurn)

-- A coordinated head fixes no type (finding 50), and an untyped head
-- cannot prove participation: "target creature or land" would have to
-- carry the attack grant on an alternative that has none. The silence
-- is not permission — `DamageableTy` learned the same lesson.
failing "DeedParticipant"
  badCantDisjunctSubject : Effect []
  badCantDisjunctSubject = cantBlock (target (Or [creature, land])) (Just thisTurn)

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
  badCantBeAttacked = Continuously (Cant (target creature) Attack Patient) (Just thisTurn)

-- Combat is fought on the battlefield: a permanent that leaves it is
-- removed from combat ([CR#506.4]), so a graveyard card has no deed to
-- be denied. The gate is the one destroy, tap, and "gets" already
-- carry.
failing "ZoneFits"
  badCantInGraveyard : Effect []
  badCantInGraveyard =
    cantBlock (target (And [creature, InZone graveyardZ])) (Just thisTurn)

-- The class word names [CR#115.4]'s damage class, describes no object,
-- and so places none — and the restriction needed no rule of its own to
-- refuse it, and as of chapter twenty-eight the refusal lands one
-- demand over: the zone demand is `zoneFits`' now and silence passes it,
-- so what catches the class word is the DEED's head-type demand — a
-- phrase that describes no object fixes no type, which is
-- `badCantDisjunctSubject`'s refusal reaching a second silent phrase.
failing "DeedParticipant"
  badCantAnyTarget : Effect []
  badCantAnyTarget = cantBlock (target AnyTarget) (Just thisTurn)

-- The same span, the wrong word. Two hundred ninety-six corpus lines
-- write a one-shot restriction and every one of them says "this turn";
-- "until end of turn" belongs to the grants, which invert the count
-- (`badGainsThisTurn`). Under the envelope the refusal is one table
-- read: `spanUse` calls the bare end-of-turn endpoint `BothGrants`, and
-- `admitsSpan` gives the restriction row no share of it.
failing "SpanOk DeedRestriction"
  badCantUntilEndOfTurn : Effect []
  badCantUntilEndOfTurn = cantBlock (target creature) (Just untilEndOfTurn)

-- …and the inverse, which is what keeps the new word from opening a
-- hole: no corpus line grants an ability "this turn".
failing "SpanOk KeywordGrant"
  badGainsThisTurn : Effect []
  badGainsThisTurn = gains (target creature) (KeywordAbility Flying) (Just thisTurn)

-- The stat change writes the grant's adverbial too — "Target creature
-- gets +3/+3 until end of turn", never "this turn".
failing "SpanOk PtDelta"
  badGetsThisTurn : Effect []
  badGetsThisTurn = gets (target creature) 3 3 (Just thisTurn)

-- A durationless "can't" is the STATIC ability line ("Enchanted
-- creature can't attack", Pacifism), which is a different construction
-- and the parked ability layer's. The slot is now optional for everyone
-- — the grants need it, since the unwritten span is [CR#611.2a]'s
-- end-of-game default — so the refusal moved from the ABSENT SLOT to
-- the reason for it: `absentOk DeedRestriction = False`, the one row of
-- that table that says no.
failing "SpanOk DeedRestriction"
  badStaticCant : Effect []
  badStaticCant = cantBlock (target creature) Nothing

-- The upkeep endpoint belongs to the KEYWORD grant alone. Two corpus
-- lines write "until your next upkeep" and both grant an ability
-- (Gabriel Angelfire, and one forestwalk line); not one stat change
-- takes it. The decomposition is what made the cell writable at all —
-- and the table is what keeps the two grants from sharing it just
-- because they share the other three.
failing "SpanOk PtDelta"
  badGetsUntilYourNextUpkeep : Effect []
  badGetsUntilYourNextUpkeep = gets (target creature) 3 3 (Just untilYourNextUpkeep)

-- Real oracle English, no clause of ours: "until the end of your next
-- turn" runs to eighty-three lines, and every one of them is a play
-- permission, a control grant, or a can't-cast — never a keyword grant.
-- The `Unclaimed` row exists to say exactly this, and to keep it apart
-- from the endpoints nothing writes at all (`badGainsUntilUntapStep`).
failing "SpanOk KeywordGrant"
  badGainsUntilEndOfYourNextTurn : Effect []
  badGainsUntilEndOfYourNextTurn =
    gains (target creature) (KeywordAbility Flying) (Just (Until (EndOf Turn (Just Yours))))

-- The combat endpoint is the grants' too — Glyph of Destruction's
-- "+10/+0" and one banding line — and no corpus line ends a single-deed
-- restriction there. The restriction's two words stay "this turn" and
-- "until your next turn".
failing "SpanOk DeedRestriction"
  badCantUntilEndOfCombat : Effect []
  badCantUntilEndOfCombat = cantBlock (target creature) (Just untilEndOfCombat)

-- And the unattested end of the table: no corpus line ends a duration
-- at an untap step in words this vocabulary has. The one line that ends
-- one there possesses it with a NOUN — "until its controller's next
-- untap step" — which is neither of `Whose`'s two words, and its clause
-- (a base-type setting) is a layer word with no construction here
-- either, so the row waits on both.
failing "SpanOk KeywordGrant"
  badGainsUntilUntapStep : Effect []
  badGainsUntilUntapStep =
    gains (target creature) (KeywordAbility Flying)
          (Just (Until (StartOf UntapStep (Just Yours))))

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
  badMatchesAnyTarget = If (destroy (target artifact)) (itsA AnyTarget) Nothing

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
  badMatchesTargetSubject = Matches (target creature) artifact

-- A description that says NOTHING tests nothing. The copular frame does
-- not demand a HEAD — "if it's attacking" and "if it's tapped" are real
-- oracle and head nothing — so the empty conjunction slipped past the
-- gate the existential uses (`Headed`) and had to be refused by the
-- weaker one instead (`predSays`). Every corpus line writes at least one
-- word after the copula.
failing "PredSays"
  badMatchesNothing : Effect []
  badMatchesNothing =
    Sequentially [Tap (target creature),
                  If (gainsLife You (Lit 1)) (Matches It (And [])) Nothing]

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
    Sequentially [Tap (target creature),
                  If (gainsLife You (Lit 1))
                     (notSo (Matches It (Not artifact)))
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
    If (destroy (target creature)) (Matches It (InZone graveyardZ)) Nothing

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
    CompareAmt (CountOf creatureYouControl) OrGreater (powerOf This)

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
    Sequentially [If (gainsLife You (Lit 2)) (Exists creatureYouControl) Nothing,
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
  badIfNotReadsMayBody = mayElse You (sacrifice You (a creature)) (exile It)

-- ===== What a token may be, and what a counter may sit on =====

-- A subtype sits on one card type's own set ([CR#205.1a] names the six
-- sets; [CR#205.3m] makes every subtype here a creature type), so a
-- token whose line names
-- a subtype its types cannot carry describes nothing: a "Zombie artifact
-- token" writes a creature type onto an object with no creature type.
failing "SubtypesFit"
  badZombieArtifactToken : Effect []
  badZombieArtifactToken =
    create (Lit 1) (MkToken (Just (1, 1)) [Black] (MkTypeLine [Zombie] [Artifact])
                            [] Nothing)

-- A token has only the characteristics its creating effect defines
-- ([CR#111.3]), so a creature token's power and toughness ([CR#208.1])
-- have to be written — and every corpus creature-token line writes them.
-- (The converse is deliberately NOT demanded: a Vehicle token carries a
-- P/T with no creature type, [CR#301.7].)
failing "TokenPt"
  badCreatureTokenNoPt : Effect []
  badCreatureTokenNoPt =
    create (Lit 1) (MkToken Nothing [White] (MkTypeLine [Soldier] [Creature]) [] Nothing)

-- A token is a PERMANENT ([CR#111.1]), so its line names at least one
-- card type. The type-less spelling is the predefined name ("create a
-- Treasure token", [CR#111.10]) — core's own separate `TokenSpec::Named`
-- row — and it waits with that catalog. (Probed with no subtype either,
-- so the refusal is the missing type and not a subtype with nowhere to
-- sit.)
failing "TokenTyped"
  badTypelessToken : Effect []
  badTypelessToken =
    create (Lit 1) (MkToken (Just (1, 1)) [White] (MkTypeLine [] []) [] Nothing)

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
    Create You (Lit 1) (creatureTok 1 1 [Red] [Soldier]) [EntersAttacking]

-- Counters are put on permanents: no corpus line puts one on a card in a
-- graveyard, and the battlefield demand is the one destroy, tap, and
-- "gets" already carry.
failing "OnBattlefield"
  badPutCountersGraveyard : Effect []
  badPutCountersGraveyard =
    PutCounters (Lit 1) PlusOnePlusOne (target (And [creature, InZone (graveyardOf You)]))

-- …and the removal twin reads the same fold-state: a destroyed referent
-- has no counters to take off.
failing "OnBattlefield"
  badRemoveCountersDead : Effect []
  badRemoveCountersDead = Sequentially [destroy (target creature),
                                        RemoveCounters (Lit 1) PlusOnePlusOne It]

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
    create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [] [Creature, Artifact])
                            [] Nothing)

failing "TokenCanonical"
  badTokenDuplicateColor : Effect []
  badTokenDuplicateColor = create (Lit 1) (creatureTok 1 1 [White, White] [Soldier])

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
  badDrawZero = drawCards 0

failing "WrittenCount"
  badCreateZero : Effect []
  badCreateZero = create (Lit 0) (creatureTok 1 1 [White] [Soldier])

failing "WrittenCount"
  badPutZeroCounters : Effect []
  badPutZeroCounters = PutCounters (Lit 0) PlusOnePlusOne (target creature)

-- ===== What a type addition may add, and for how long =====

-- A creature subtype has nowhere to sit on a land ([CR#205.1a] — a
-- subtype correlated with a card type the object doesn't have is not one
-- of its subtypes), so the added line has to name the card type itself,
-- as "becomes a Spirit artifact creature" does, or find it on the
-- subject, as amass's "it becomes a Zombie" does of an Army creature
-- ([CR#701.47a]).
failing "AddedFits"
  badBecomesZombieLand : Effect []
  badBecomesZombieLand = becomes (target land) (subtypesOnly [Zombie]) Nothing

-- "Becomes in addition to its other types" has to say WHAT: an empty
-- type line adds nothing and spells no phrase.
failing "LineNonEmpty"
  badBecomesNothing : Effect []
  badBecomesNothing = becomes (target creature) (MkTypeLine [] []) Nothing

-- A type change is a continuous effect on a permanent: a graveyard card
-- has no types for the clause to add to on the battlefield.
failing "ZoneFits"
  badBecomesInGraveyard : Effect []
  badBecomesInGraveyard =
    becomes (target (And [creature, InZone (graveyardOf You)])) (typesOnly [Artifact]) Nothing

-- The combat endpoint stays the GRANTS' alone. Chapter seventeen could
-- not tell "until end of turn" and "until end of combat" apart, both
-- being written by the stat delta and the keyword grant and by neither
-- restriction; the type addition tells them apart, writing eighteen
-- end-of-turn lines and no end-of-combat line at all — which is why
-- `BothGrants` split and `GrantsAndTypes` exists.
failing "SpanOk TypeAddition"
  badBecomesUntilEndOfCombat : Effect []
  badBecomesUntilEndOfCombat =
    becomes (target creature) (typesOnly [Artifact]) (Just untilEndOfCombat)

-- …and the restriction's current-turn word is not the type addition's
-- either. Coward // Killer writes both adverbials in one sentence and
-- gives "this turn" to the "can't block" half, which is the division
-- stated by a card rather than by a count.
failing "SpanOk TypeAddition"
  badBecomesThisTurn : Effect []
  badBecomesThisTurn = becomes (target creature) (typesOnly [Artifact]) (Just thisTurn)

-- …and the upkeep endpoint stays the keyword grant's alone, as it was
-- against the stat delta (`badGetsUntilYourNextUpkeep`): two corpus
-- lines write "until your next upkeep" and both grant an ability.
failing "SpanOk TypeAddition"
  badBecomesUntilYourNextUpkeep : Effect []
  badBecomesUntilYourNextUpkeep =
    becomes (target creature) (typesOnly [Artifact]) (Just untilYourNextUpkeep)

-- A subtype word PRESUPPOSES its set's card type ([CR#205.1a]), so "an
-- Army that isn't a creature" describes nothing — the finding-43 shape
-- again, the presupposition riding the word and the existing coherence
-- gate supplying the refusal. What it does NOT do is project that type,
-- which is what keeps `clavilenoPhrase` writable.
failing "ContradictionFree"
  badZombieNoncreature : Predicate [] Object
  badZombieNoncreature = And [HasSubtype Zombie, Not creature]

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
  badBecomesOwnType = becomes (target creature) (typesOnly [Creature]) Nothing

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
    becomes (target (And [artifact, ControlledBy You]))
            (typesOnly [Creature])
            (Just untilYourNextTurn)

-- ===== What an else arm may read =====

-- The "Otherwise" arm runs exactly when the condition was false, so the
-- clause it replaces never happened and its phrase never named anything:
-- an arm that reads the if-arm's token is unwritable, which is `May`'s
-- declined-arm asymmetry (`badIfNotReadsMayBody`) one construction over.
-- The corpus writes no such line either — every else arm read this pass
-- reaches the sentences before the conditional or nothing at all.
failing "countOnes"
  badOtherwiseReadsIfArm : Effect []
  badOtherwiseReadsIfArm = If (create (Lit 1) (creatureTok 1 1 [Black] [Zombie]))
                              (Exists creatureYouControl)
                              (Just (Tap It))

-- ===== What a mode may read, what a headcount may fix, and what a draw leaves =====

-- A modal offers TWO OR MORE options ([CR#700.2] says so in its own
-- definition), so a one-mode "modal" is not one — it is the sentence
-- itself with a choice clause bolted on front, and no card writes it.
failing "AtLeastTwo"
  badModalOneMode : Effect []
  badModalOneMode = Modal (upTo 1) [destroy (target artifact)]

-- A headcount that fixes the whole list instructs no choice: "Choose two
-- —" over exactly two modes has one answer, and [CR#700.2] calls a spell
-- modal for the INSTRUCTIONS to choose. Zero cards write it — every
-- printed exact headcount is strictly under its list, and the two forms
-- whose top reaches the list ("one or both", "one or more") are ranges.
failing "ModesFit"
  badModalFixedWhole : Effect []
  badModalFixedWhole = chooseTwo [destroy (target artifact),
                                  destroy (target enchantment)]

-- Nor may a headcount reach PAST the list: three of two options names
-- nothing at all.
failing "ModesFit"
  badModalOverreach : Effect []
  badModalOverreach = Modal (exactly 3) [destroy (target artifact),
                                         destroy (target enchantment)]

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
  badModalReadsAcrossModes = chooseOne [destroy (target artifact), Tap It]

-- And nothing after the modal reads into it, for the same reason pointed
-- forward: Blood on the Snow writes "Then return a creature or
-- planeswalker card … from your graveyard" — a description covering both
-- modes' outcomes — exactly where an anaphor would have gone.
failing "countOnes"
  badReadsAfterModal : Effect []
  badReadsAfterModal =
    Sequentially [chooseOne [destroy (target artifact), destroy (target enchantment)],
                  Tap It]

-- WHICH condition frames take a "not" is a closed table. The comparison
-- frame does not: a bound has a negative of its own, and English writes
-- that instead — "if its power isn't 4 or greater" is written zero times.
failing "CondNegatable"
  badNegatedComparison : Effect []
  badNegatedComparison =
    If (destroy (target artifact))
       (notSo (CompareAmt (manaValueOf It) OrLess (Lit 2)))
       Nothing

-- Nor does a negation take one: no corpus line writes a condition under
-- two of them, English collapsing the pair into the positive.
failing "CondNegatable"
  badDoubleNegatedCondition : Effect []
  badDoubleNegatedCondition =
    If (destroy (target artifact))
       (notSo (notSo (Exists creatureYouControl)))
       Nothing

-- A drawn card is not a mention. The corpus never reads one back across a
-- sentence boundary — "Draw a card." followed by "it" or "that card" is
-- written zero times, and the card IS read only inside the coordination
-- that reveals it ("Draw a card and reveal it. If it isn't a land card,
-- discard it."), a verb-phrase coordination this grammar does not spell.
failing "countWord"
  badDrawnCardRemention : Effect []
  badDrawnCardRemention = Sequentially [drawACard, exile (That CardW)]

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
  badModalUpToTwo = Modal (upTo 2) [destroy (target artifact),
                                    destroy (target enchantment),
                                    drawACard]

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
  badDuplicateModes = chooseOne [drawACard, drawACard]

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
    Sequentially [If (create (Lit 1) (creatureTok 1 1 [White] [Soldier]))
                     (Exists creatureYouControl)
                     Nothing,
                  PutCounters (Lit 1) PlusOnePlusOne It]

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
    Sequentially [May (Just You) (gainsLife You (Lit 1))
                       (Just (create (Lit 1) (creatureTok 1 1 [White] [Soldier])))
                       (Just (create (Lit 2) (creatureTok 1 1 [White] [Soldier]))),
                  PutCounters (Lit 1) PlusOnePlusOne It]



-- ===== What a complement may exclude, and what a batch may read =====

-- The complement's anchor may not be an INDEFINITE: "each other
-- creature" announces one phrase, and "other than a creature" would
-- announce a second referent the sentence never spelled. Unwritten
-- English, and it stays refused.
failing "ComplementAnchor"
  badComplementAnchorAnnounces : Predicate [] Object
  badComplementAnchorAnnounces = OtherThan (a creature)

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
  Create (Each (And [AnyPlayer, OtherThan (target AnyPlayer)])) (Lit 1)
         (MkToken (Just (5, 5)) [Red] (MkTypeLine [Dragon] [Creature])
                  [Flying] Nothing) []

-- "Prevent all combat damage that would be dealt by creatures other than
-- target creature this turn." (Terrifying Presence) — the anchor half of
-- the second targeted line, written on its own. The full sentence needs
-- the SOURCE-restricted prevention shield chapter twenty-five ledgered
-- ("dealt BY", where this vocabulary's `Prevents` scopes by recipient),
-- so what lands here is the phrase, which is what the finding was about.
terrifyingPresenceAnchor : Predicate [] Object
terrifyingPresenceAnchor = And [creature, OtherThan (target creature)]

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
  badPluralComplementAnchor = OtherThan (TargetGroup (upTo 2) creature)

-- The anchor has to be something the phrase could have described:
-- "each other creature" anchored to a LAND subtracts nothing, and no
-- corpus line pairs "other" with a cross-head anchor — the same
-- evidence `anyTargetedTy` reads for the bare word.
failing "OtherAnchored"
  badComplementCrossHead : Effect []
  badComplementCrossHead =
    DealDamage This (Lit 1) (Each (And [creature, OtherThan thisLand]))

-- One selector slot per phrase, whichever spelling fills it: two
-- complements are two "other"s, and the guide gives the word one
-- position.
failing "OtherAnchored"
  badDoubleComplement : Effect []
  badDoubleComplement =
    DealDamage This (Lit 1)
               (Each (And [creature, OtherThan thisCreature, OtherThan thisCreature]))

-- And the two spellings share that slot: a phrase cannot write the bare
-- "other" and an anchored one at once.
failing "OtherAnchored"
  badOtherAndComplement : Effect []
  badOtherAndComplement =
    Sequentially [Tap (target creature),
                  DealDamage This (Lit 1)
                             (Each (And [creature, Other, OtherThan thisCreature]))]

-- A word that fills one phrase-level slot is not an ALTERNATIVE, which
-- is `badOtherInOr`'s refusal reaching the second spelling too.
failing "CoordinableDisjuncts"
  badComplementInOr : Predicate [] Object
  badComplementInOr = Or [And [creature, OtherThan thisCreature], land]

-- "Non-other" is unwritten, as "non-other" always was.
failing "Negatable"
  badNegatedComplement : Predicate [] Object
  badNegatedComplement = Not (OtherThan thisCreature)

-- A batch is at least TWO parts, the arity demand `Sequentially` makes
-- and for its reasons: nothing at all, and a second spelling of one
-- clause.
failing "AtLeastTwo"
  badEmptySimultaneous : Effect []
  badEmptySimultaneous = Simultaneously []

failing "AtLeastTwo"
  badSingletonSimultaneous : Effect []
  badSingletonSimultaneous = Simultaneously [destroy (target creature)]

-- Its elements are clauses and not batches — the re-minted tree
-- `NotSeq` refuses one construction over.
failing "NotSim"
  badNestedSimultaneous : Effect []
  badNestedSimultaneous =
    Simultaneously [Simultaneously [destroy (target creature), destroy (target artifact)],
                    destroy (target land)]

-- Nor SEQUENCES: an ordered list inside an unordered one contradicts the
-- container it sits in, and its announcements would reach the next
-- element only from its last clause besides.
failing "NotSeq"
  badSequenceInsideSimultaneous : Effect []
  badSequenceInsideSimultaneous =
    Simultaneously [Sequentially [destroy (target creature), destroy (target artifact)],
                    destroy (target land)]

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
    Simultaneously [exile (target creature), destroy (That CardW)]

-- The event OUTCOME is the same fact for magnitudes: no damage has been
-- dealt when a sibling is typed, so "that much" reads nothing.
failing "countOnes"
  badSimultaneousReadsOutcome : Effect []
  badSimultaneousReadsOutcome =
    Simultaneously [DealDamage This (Lit 2) (target creature),
                    gainsLife You ThatMuch]

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
    Simultaneously [may You (create (Lit 1) (creatureTok 1 1 [Green] [Plant])),
                    PutCounters (Lit 1) PlusOnePlusOne It]

-- …and the magnitude twin, which is `badSimultaneousReadsOutcome`
-- reached through the same wrapper.
failing "countOnes Outcome"
  badSimultaneousReadsMayOutcome : Effect []
  badSimultaneousReadsMayOutcome =
    Simultaneously [may You (DealDamage This (Lit 2) (target creature)),
                    gainsLife You ThatMuch]

-- OUTWARD, a batch leaves behind EVERY element's deed and not the last
-- one's: two creates in one instruction leave two tokens ([CR#608.2f]
-- processes both actions), so the sentence after cannot say "it".
-- Chapter twenty-two folded the last element's `effIntro` and recorded
-- the simplification; this is the row that shows it was one
-- (`simIntro`, `deedDelta`).
failing "countOnes Object"
  badBatchTwoCreatesThenIt : Effect []
  badBatchTwoCreatesThenIt =
    Sequentially [Simultaneously [create (Lit 1) (creatureTok 1 1 [Green] [Plant]),
                                 create (Lit 1) (creatureTok 1 1 [White] [Soldier])],
                  PutCounters (Lit 1) PlusOnePlusOne It]

-- The magnitude twin outward: two outcomes in one batch, and "that
-- much" does not say which.
failing "countOnes Outcome"
  badBatchTwoOutcomesThenThatMuch : Effect []
  badBatchTwoOutcomesThenThatMuch =
    Sequentially [Simultaneously [DealDamage This (Lit 2) (target creature),
                                 losesLife (target Opponent) (Lit 3)],
                  gainsLife You ThatMuch]

-- Control is a PERMANENT's ([CR#110.2] gives every permanent a
-- controller; [CR#109.4] gives an object that is neither on the stack
-- nor on the battlefield none), so the grant takes a battlefield
-- referent like every other continuous clause.
failing "ZoneFits"
  badGainControlGraveyard : Effect []
  badGainControlGraveyard =
    gainControl (target (And [creature, InZone graveyardZ])) Nothing

-- The control grant writes the GRANTS' current-turn word and never the
-- restrictions': "gain control of target creature this turn" is written
-- zero times, and the four corpus lines pairing the two words are event
-- clauses ("that attacked you this turn") rather than adverbials.
failing "SpanOk"
  badGainControlThisTurn : Effect []
  badGainControlThisTurn = gainControl (target creature) (Just thisTurn)

-- "Until the end of your next turn" is the control grant's ALONE — the
-- cell chapter seventeen left `Unclaimed` and this chapter claimed. The
-- keyword grant does not write it (the eighty-three lines beside the
-- four control ones are play permissions, another construction's).
failing "SpanOk"
  badKeywordGrantEndOfNextTurn : Effect []
  badKeywordGrantEndOfNextTurn =
    gainsHaste (target creature) (Just (Until (EndOf Turn (Just Yours))))

-- And "for as long as" is written by every construction but the type
-- ADDITION: the thirteen "becomes … for as long as" corpus lines are
-- type SETTINGS and copies, and the addition's own phrase pairs with
-- the adverbial zero times.
failing "SpanOk"
  badTypeAdditionForAsLongAs : Effect []
  badTypeAdditionForAsLongAs =
    becomes (target creature) (typesOnly [Artifact])
            (Just (ForAsLongAs (Matches thisCreature (ControlledBy You))))

-- A distributed creation exports a PLURAL mention, so the singular
-- pronoun has nothing to resolve to: "Each player creates a 1/1 green
-- Plant creature token. Put a +1/+1 counter on IT" is unwritable, and it
-- is the same clause that reads back fine undistributed (Additive
-- Evolution's "create a 0/0 … Fractal creature token. Put three +1/+1
-- counters on it."). The count is one in both.
failing "countOnes"
  badDistributedCreationIt : Effect []
  badDistributedCreationIt =
    Sequentially [Create (Each AnyPlayer) (Lit 1) (creatureTok 1 1 [Green] [Plant]) [],
                  PutCounters (Lit 1) PlusOnePlusOne It]

-- "Each of" distributes over MEMBERS, so its complement is plural:
-- "each of target creature" names one thing and has nothing to reach
-- into.
failing "ManyOf"
  badEachOfSingular : Noun [] Object
  badEachOfSingular = EachOf (target creature)

-- …and it is a group MENTION and not a description: "each of each
-- creature" is written zero times, the plain distributive being what a
-- description takes.
failing "GroupMention"
  badEachOfDistributive : Noun [] Object
  badEachOfDistributive = EachOf (Each creature)

-- …nor the universal, for the same reason and with the same count:
-- "each of all creatures" is written zero times.
failing "GroupMention"
  badEachOfAll : Noun [] Object
  badEachOfAll = EachOf (AllOf creature)

-- …and it does not stack: one determiner fills the position, and "each
-- of each of them" spells nothing twice.
failing "GroupMention"
  badNestedEachOf : Noun [] Object
  badNestedEachOf = EachOf (EachOf (TargetGroup (upTo 2) creature))

-- A clause writing ONE per-member amount refuses a bare plural
-- recipient: "put a +1/+1 counter on up to two target creatures" is the
-- sentence the corpus never writes, and "each of" is exactly the word it
-- writes instead (zero lines at two, three, or four against
-- twenty-nine at "each of up to two target creatures").
failing "PerMember"
  badBarePluralCounterRecipient : Effect []
  badBarePluralCounterRecipient =
    PutCounters (Lit 1) PlusOnePlusOne (TargetGroup (upTo 2) creature)

-- …and the damage verb reads the same way: "deals 1 damage to up to two
-- target creatures" is unwritten, while "to up to ONE target creature"
-- is twenty-eight lines and singular already.
failing "PerMember"
  badBarePluralDamageRecipient : Effect []
  badBarePluralDamageRecipient =
    DealDamage This (Lit 1) (TargetGroup (upTo 2) creature)

-- …and a plural READ is no better than a plural mention: the corpus's
-- "counters on them" lines are all relative clauses ("cards with intel
-- counters on them"), never a recipient.
failing "PerMember"
  badThemCounterRecipient : Effect []
  badThemCounterRecipient =
    Sequentially [Choose (TargetGroup anyNumber creature),
                  PutCounters (Lit 1) PlusOnePlusOne Them]

-- …and the universal determiner with them: "deals 2 damage to all
-- creatures" is zero lines, the sweep being written distributively
-- ("damage to each creature", two hundred thirty-five).
failing "PerMember"
  badAllOfDamageRecipient : Effect []
  badAllOfDamageRecipient = DealDamage This (Lit 1) (AllOf creature)

-- A division needs members to divide among: "deals 2 damage divided as
-- you choose among target creature" names one recipient and divides
-- nothing.
failing "ManyOf"
  badDivideAmongSingular : Effect []
  badDivideAmongSingular = dealsDivided This (Lit 2) (target creature)

-- …and the members must be a MENTION and not a description, since
-- [CR#601.2d] has the caster announce the division over the targets they
-- announced: "divided as you choose among each creature" is unwritten,
-- and every corpus line writes a counted target group or a plural read.
failing "GroupMention"
  badDivideAmongDescription : Effect []
  badDivideAmongDescription = dealsDivided This (Lit 2) (Each creature)

-- …and a division of nothing instructs nothing, which is the written-count
-- discipline reaching the new clause.
failing "WrittenCount"
  badDivideZero : Effect []
  badDivideZero = dealsDivided This (Lit 0) (TargetGroup (oneThrough 2) AnyTarget)

-- …and the counter half keeps the battlefield demand its undivided twin
-- carries: no corpus line distributes counters onto cards in a
-- graveyard.
failing "OnBattlefield"
  badDistributeCountersGraveyard : Effect []
  badDistributeCountersGraveyard =
    distributeCounters (Lit 2) PlusOnePlusOne
                       (TargetGroup (oneThrough 2) (And [creature, InZone (graveyardOf You)]))

-- "[-10]: Target player's life total becomes 1." — the set-to, chapter
-- twenty-two's ledgered player-attribute set. It leaves no outcome
-- mention behind, which is finding 121 made structural: a set is
-- realized as a gain or a loss depending on where the total stood, and
-- the sentence does not say which.
lifeTotalBecomesOne : Effect []
lifeTotalBecomesOne = lifeTotalBecomes (target AnyPlayer) (Lit 1)



-- ===== The ordered library, what may be exposed, and what may be subtracted =====

-- A library is ORDERED ([CR#401.2]), so "into your library" names no
-- place to put a card and oracle never writes it: every corpus
-- placement spells a position. `DestOk` has no row for the bare zone,
-- which is core's `exclude(Library)` on `Destination` exactly.
failing "DestOk"
  badMoveToBareLibrary : Effect []
  badMoveToBareLibrary = Move (target creature) (ZoneAt Library Bare)

-- The order rider needs two or more cards to order — [CR#401.4]'s own
-- condition, and English's: "put it on the bottom of your library in
-- any order" is zero lines.
failing "ArrangementOk"
  badSingularOrderRider : Effect []
  badSingularOrderRider =
    Sequentially [lookAt topCard, Move (That CardW) (onBottomIn AnyOrder)]

-- "The rest" of WHAT: with no group in the discourse the complement has
-- nothing to be the rest of.
failing "countGroups"
  badRestWithoutGroup : Effect []
  badRestWithoutGroup = Move TheRest graveyardZ

-- …and with a group but nothing taken out of it, "the rest" IS the
-- group and the sentence would have written "them".
failing "countParts"
  badRestWithoutPart : Effect []
  badRestWithoutPart = Sequentially [lookAt (topCards 4), Move TheRest onBottomZ]

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
    Sequentially [ lookAt (topCards 4)
                 , Move (oneOf Them) handZ
                 , Move TheRest onBottomZ
                 , Move TheRest graveyardZ
                 ]

-- The direction chapter twenty-two refused, and it stays refused: two
-- separately announced targets are two mentions and never a pair
-- ([CR#601.2c], finding 127), so no complement can subtract inside
-- them. What changed is that a group ASSEMBLED by one phrase now
-- exists — not that announcements can be added up.
failing "countGroups"
  badRestOverTwoAnnouncements : Effect []
  badRestOverTwoAnnouncements =
    Sequentially [ Fights (target creatureYouControl) (target creatureYouDontControl)
                 , Move TheRest graveyardZ
                 ]

-- A partitive is a selection, but the corpus names its chooser every
-- time ("an opponent chooses two of them") and `Choose` has no agent
-- slot; an agentless "Choose one of them" is unwritten English.
failing "Choosable"
  badChooseSomeOf : Effect []
  badChooseSomeOf = Sequentially [lookAt (topCards 4), Choose (oneOf Them)]

-- A shuffle randomizes the library and destroys what the discourse
-- knew about it ([CR#701.24a]; [CR#701.20d] makes a reordered revealed
-- card a NEW object), so a card still in the library cannot be read
-- back afterwards. Every search sentence places its find first for
-- exactly this reason.
failing "countOnes"
  badReadAfterShuffle : Effect []
  badReadAfterShuffle =
    Sequentially [searchLibraryFor land, shuffle, Move It handZ]

-- The slice is in a library, so every battlefield-demanding verb
-- refuses it through the demand it already carried.
failing "OnBattlefield"
  badTapLibraryTop : Effect []
  badTapLibraryTop = Tap topCard

-- …and it describes no card, which is the hidden zone's honesty made
-- structural ([CR#400.2,401.2]): the phrase names a place, so a typed
-- demonstrative has nothing to reach.
failing "countManyWord"
  badSliceTypeRead : Effect []
  badSliceTypeRead =
    Sequentially [lookAt (topCards 4), Move (Those (TypeW Creature)) handZ]

-- A library is not shown whole: "reveal your library" is zero lines,
-- and what oracle exposes of a library is a positioned slice.
failing "ExposableZone"
  badRevealWholeLibrary : Effect []
  badRevealWholeLibrary = Expose Reveal You (ExposedZone yourLibrary)

-- Nor is a public zone: a graveyard is already visible to everyone
-- ([CR#400.2]), so revealing one says nothing and no line does it.
failing "ExposableZone"
  badRevealGraveyard : Effect []
  badRevealGraveyard = Expose Reveal You (ExposedZone graveyardZ)

-- Nor is the battlefield searched: [CR#701.23a] is about finding a card
-- among cards you cannot otherwise read.
failing "SearchableZone"
  badSearchBattlefield : Effect []
  badSearchBattlefield = Search You battlefieldZ creature

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
  badSearchLibraryPosition = Search You (LibraryAt OnTop Nothing Bare) land

failing "WholeZone"
  badSearchLibraryPositionOrdered : Effect []
  badSearchLibraryPositionOrdered =
    Search You (LibraryAt OnBottom (Just RandomOrder) Bare) creature

-- The search's description is the zone's, not another zone's: the
-- clause supplies the place, so a phrase carrying its own is two
-- answers to one question.
failing "ZoneFree"
  badSearchZonedDescription : Effect []
  badSearchZonedDescription =
    searchLibraryFor (And [creature, InZone graveyardZ])

-- Whose library is one player's ([CR#400.1]) — the possessor demand
-- `handOf` and `graveyardOf` already carry, asked of the slice.
failing "OneOf"
  badSliceOfPluralPossessor : Effect []
  badSliceOfPluralPossessor =
    lookAt (LibrarySlice OnTop (Lit 1) (Each AnyPlayer))

-- A zero slice and a zero mill instruct nothing, which is
-- `WrittenCount`'s discipline reaching two more counts.
failing "WrittenCount"
  badZeroSlice : Effect []
  badZeroSlice = lookAt (topCards 0)

failing "WrittenCount"
  badMillZero : Effect []
  badMillZero = millCards 0

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
    Sequentially [Mill (Each AnyPlayer) (Lit 1), exile It]

-- A partitive reaches into a GROUP, not into a description: "one of a
-- creature you control" is not English, and the determiner has no
-- members to pick from until some phrase has fixed them.
failing "GroupMention"
  badPartitiveOfDescription : Effect []
  badPartitiveOfDescription = exile (SomeOf (exactly 1) (a creature))

-- …nor into another partitive: a part is what was taken, not a group to
-- take from ("two of one of them" is zero lines).
failing "GroupMention"
  badPartitiveOfPartitive : Effect []
  badPartitiveOfPartitive =
    Sequentially [lookAt (topCards 4), exile (oneOf (oneOf Them))]

-- …nor into the complement: "each of the rest" is zero lines, the
-- remainder being named rather than reached into.
failing "GroupMention"
  badEachOfTheRest : Effect []
  badEachOfTheRest =
    Sequentially [ lookAt (topCards 4)
                 , Move (oneOf Them) handZ
                 , exile (EachOf TheRest)
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
    ifWouldInstead (Dies (target creature)) (exile It) Nothing

-- The same refusal on the shield: forty-nine "Prevent all …" lines
-- state no duration and every one of them is a static ability
-- ("Prevent all combat damage that would be dealt to this creature").
failing "SpanOk Prevention"
  badStandingPrevention : Effect []
  badStandingPrevention = preventAll AnyDamage Everywhere Nothing

-- Prevention writes ONE adverbial. Two hundred fifty-five prevention
-- lines carry "this turn"; "prevent … until end of turn" is written
-- zero times, all scopes. The shield and the grants do not share a
-- current-turn word any more than the restriction and the grants do.
failing "SpanOk Prevention"
  badPreventUntilEndOfTurn : Effect []
  badPreventUntilEndOfTurn =
    preventAll CombatOnly Everywhere (Just untilEndOfTurn)

-- Nor a for-as-long-as one: no corpus line conditions a shield on a
-- tracked predicate ([CR#611.2b]'s adverbial), which is what keeps the
-- widest class in the table from swallowing the two new rows.
failing "SpanOk Replacement"
  badInterceptForAsLongAs : Effect []
  badInterceptForAsLongAs =
    ifWouldInstead (Dies (target creature)) (exile It)
                   (Just (ForAsLongAs (Exists creatureYouControl)))

-- Real oracle English, no clause of ours: thirty-one lines write "would
-- be destroyed" and twenty-five of them are regeneration's own reminder
-- text, whose replacement is [CR#614.8]'s four-part instruction — tap
-- it, remove it from combat, heal the damage on it — not one part of
-- which this vocabulary writes. `EventUnclaimed` says exactly that, and
-- keeps it apart from the events nothing writes at all.
failing "Interceptable"
  badInterceptDestruction : Effect []
  badInterceptDestruction =
    ifWouldInstead (IsDestroyed (target creature)) (exile It) (Just thisTurn)

-- The multiplicity word is the event's to choose. "The next time
-- [subject] would die" is written zero times against fifty-seven
-- conditional lines, because a creature dies once and the two shields
-- would be the same shield ([CR#614.3]).
failing "ReplUseOk"
  badNextTimeWouldDie : Effect []
  badNextTimeWouldDie =
    nextTimeWouldInstead (Dies (target creature)) (exile It) (Just thisTurn)

-- And the inverse, measured the same way: "if you would draw a card
-- this turn" is written zero times against nine "the next time you
-- would draw". A draw repeats, so the conditional would be an unlimited
-- shield and the corpus never writes one.
failing "ReplUseOk"
  badIfWouldDraw : Effect []
  badIfWouldDraw =
    ifWouldInstead (Draws You) (gainsLife You (Lit 5)) (Just thisTurn)

-- Dying is the battlefield-to-graveyard transition ([CR#700.4]), so the
-- watched object stands on the battlefield — `EventQuery`'s own demand
-- asked by the other reader of the same event.
failing "ZoneFits"
  badWouldDieInGraveyard : Effect []
  badWouldDieInGraveyard =
    ifWouldInstead (Dies (target (And [creature, InZone (graveyardOf You)])))
                   (exile It) (Just thisTurn)

-- The replacement is a HOLE outward. [CR#614.7] says a replacement
-- effect whose intercepted event never happens "simply doesn't do
-- anything", so a token the replacement would have made is not there
-- for the next sentence to read — which is the conditional arm's
-- refusal (finding 100) at a second site.
failing "countOnes Object"
  badInterceptReplacementAntecedent : Effect []
  badInterceptReplacementAntecedent =
    Sequentially [ nextTimeWouldInstead (Draws You)
                                        (create (Lit 1) (creatureTok 1 1 [Green] [Soldier]))
                                        (Just thisTurn)
                 , sacrifice You It
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
    gets (target creature) 2 2 (Just (UntilEvent (Leaves thisCreature)))

-- The rider goes on a zone change to EXILE and on nothing else: nothing
-- returns from a graveyard "until", and the corpus writes the rider
-- with the exile verb every time.
failing "HeldClause"
  badHeldUntilDestroy : Effect []
  badHeldUntilDestroy =
    HeldUntil (destroy (target creature)) (Leaves thisCreature)

-- And the event half of the same gate: the rider waits for a
-- leaves-the-battlefield event and for no other. Not one line writes
-- "exile [object] until [something] dies".
failing "Holdable"
  badHeldUntilDies : Effect []
  badHeldUntilDies =
    HeldUntil (exile (target creature)) (Dies thisCreature)

-- The held object's ZONE is not settled by the clause: the undo is
-- scheduled on an event that has not happened, so the exiled creature
-- may be in exile or back on the battlefield when a later sentence
-- reads it. The clause contributes its announcement and not the exile's
-- retag, so a graveyard-demanding read of the exiled card finds nothing
-- there to move.
failing "countWord CardW"
  badHeldUntilExileRetag : Effect []
  badHeldUntilExileRetag =
    Sequentially [ exileUntil (target creature) (Leaves thisCreature)
                 , Move (That CardW) handZ
                 ]

-- A replacement effect gets "only one opportunity to affect an event or
-- any modified events that may replace that event" ([CR#614.5]), so a
-- replacement of a replacement is not a sentence English writes.
failing "NotInstead"
  badNestedInstead : Effect []
  badNestedInstead =
    insteadOf (insteadOf drawACard (drawCards 2)) (drawCards 3)

-- [CR#614.6]: a replaced event "never happens". So the replaced
-- clause's OUTCOME is not there to read — "that much" after a damage
-- clause that was replaced measures nothing — even though the same
-- clause's announced target is ([CR#601.2c]). `annIntro` and `effIntro`
-- divide exactly here.
failing "countOnes Outcome"
  badInsteadReadsReplacedOutcome : Effect []
  badInsteadReadsReplacedOutcome =
    insteadOf (DealDamage This (Lit 3) (target AnyTarget))
              (gainsLife You ThatMuch)

-- The same refusal through a SEQUENCE, which is where it used to leak:
-- the replacement was typed in `preIntro replaced`, and a sequence's
-- pre-state is its last clause's over the DEED telescope, so an earlier
-- step's outcome walked into a replacement for an event that never
-- happened. The announcement channel is a hole at a sequence, its
-- elements being typed over each other's `effIntro` (`annIntro`).
failing "countOnes Outcome"
  badInsteadReadsReplacedSequenceOutcome : Effect []
  badInsteadReadsReplacedSequenceOutcome =
    insteadOf (Sequentially [DealDamage This (Lit 3) (target creature), drawACard])
              (gainsLife You ThatMuch)

-- And through a CONDITIONAL wrapping an optional clause, which is the
-- recursion that carried either defect: chapter twenty-five opened the
-- conditional's announcement channel on `preIntro e`, correctly for the
-- flat clause it was opened for (Overload's "that artifact", Colossal
-- Growth's "that creature") and not for a composite. It is structural
-- now, so the nested may's damage outcome is not there to read.
failing "countOnes Outcome"
  badConditionalInsteadReadsMayOutcome : Effect []
  badConditionalInsteadReadsMayOutcome =
    insteadOf (If (may You (DealDamage This (Lit 2) (target creature)))
                  (Exists creature)
                  Nothing)
              (gainsLife You ThatMuch)

-- ===== What may be paid, what may be spelled "pay", and what may be
-- granted =====

-- A draw is not a payment. The colon used to accept any clause at all,
-- which is what the ledger's cost-GRAMMAR entry named: [CR#602.1a] makes
-- a cost what the ACTIVATOR pays, and no corpus line writes "Draw a
-- card:" before a colon (zero, against eleven hundred eighty-four
-- sacrifice components).
failing "CostAction"
  badDrawAsCost : Ability
  badDrawAsCost = Activated (Do drawACard) drawACard

-- Nor is a destruction, which is the sharper half of the same table:
-- sacrifice and exile ARE cost verbs and destroy is not, so the refusal
-- has to key on the composite's TAG and not on the move underneath it
-- (zero "Destroy …:" components).
failing "CostAction"
  badDestroyAsCost : Ability
  badDestroyAsCost = Activated (Do (destroy (a creature))) drawACard

-- The life row is DIRECTIONAL: ninety-five "Pay N life" components
-- against zero gain-life ones, so paying life is a cost and gaining it
-- is not. A gain-life cost does exist — [CR#119.7] speaks of "a cost
-- that involves having that player gain life" — and the cards that print
-- one spell it as an ALTERNATIVE cost ([CR#118.9]), a base swap rather
-- than an activation cost.
failing "CostAction"
  badGainLifeCost : Ability
  badGainLifeCost = Activated (Do (gainsLife You (Lit 2))) drawACard

-- "Pay" is one English VERB and a cost is the whole thing an ability
-- charges. A sacrifice is a payment ([CR#118.1] — a cost is "an action
-- or payment") and is not payABLE:
-- the seventy-one non-mana unless lines write their own verb ("unless
-- you sacrifice a land"), never "pay".
failing "Payable"
  badPayBySacrificing : Effect []
  badPayBySacrificing = Pay You (Do (sacrifice You (a creature)))

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
  badPayCompound = Pay You (Compound [Mana [generic 1], TapSymbol])

-- Growing the container is what made this refusal necessary. English
-- grants an activated ability by QUOTING it — "Enchanted land has \"{T}:
-- Add {B}\"", twenty-five lines, and the equipped/all-Slivers twins with
-- it — which is a construction this grammar has no quotation for, where
-- "gains flying" is a bare keyword.
failing "Grantable"
  badGainsActivated : Effect []
  badGainsActivated = gains (target creature)
                            (Activated (Mana [generic 1]) drawACard)
                            (Just untilEndOfTurn)

-- The token with-clause is the same refusal one type lower, and it is
-- why that field holds `Keyword` and not the container: seven corpus
-- lines create a token with a quoted activated ability, and none of them
-- is writable without the quotation.
failing "Keyword"
  badTokenActivatedAbility : TokenChars
  badTokenActivatedAbility =
    MkToken (Just (1, 1)) [Red] (MkTypeLine [] [Creature])
            [Activated (Mana [generic 1]) drawACard] Nothing

-- A compound's ELEMENTS are components: nesting re-mints the
-- right-nested tree the telescope replaced, and core reaches the flat
-- form by normalizing instead (`Cost::normalize`).
failing "NotCompound"
  badNestedCompound : Ability
  badNestedCompound =
    Activated (Compound [Compound [Mana [generic 1], TapSymbol], TapSymbol]) drawACard

-- And a compound of one is the component itself spelled a second way —
-- the singleton-sequence refusal at the cost layer.
failing "AtLeastTwo"
  badSingletonCompound : Ability
  badSingletonCompound = Activated (Compound [Mana [generic 1]]) drawACard

-- The arm asymmetry carries to the MANDATORY twin unchanged, which is
-- the check that the two shapes really are one node: the declined arm
-- runs only when the payment was never started ([CR#118.12] — "started
-- to pay a mandatory cost, regardless of what events actually
-- occurred"), so the body's phrase named nobody it can read.
failing "countOnes"
  badIfNotReadsMandatoryBody : Effect []
  badIfNotReadsMandatoryBody = doElse (sacrifice You (a creature)) (exile It)


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
    mayElse (ControllerOf It) (Pay You (Mana [generic 1])) (Tap (target creature))


-- ===== What may trigger, what may be stated as a line, and what a
-- ===== permission may permit

-- [CR#603.2b] gives "at the beginning of" a phase or step its own
-- clause, and the corpus honors it without exception: one thousand six
-- hundred seventy-eight headers open with "At the beginning of" and
-- every one of them names a turn part. No object event takes the word.
failing "TriggerWordOk"
  badAtEnters : Ability
  badAtEnters = Triggered At (Enters This) drawACard

-- …and the inverse, which is the half that makes the table a table: the
-- turn-part beginning takes neither English word, "When the beginning of
-- your upkeep" being written zero times against six hundred forty-one
-- "At the beginning of your upkeep".
failing "TriggerWordOk"
  badWhenUpkeep : Ability
  badWhenUpkeep = Triggered When (BeginningOf Upkeep (Just Yours)) drawACard

-- Real oracle English in one mood and none in the other: thirty-one
-- lines write "would be destroyed" (regeneration's reminder text,
-- [CR#614.8]) and NOT ONE writes "whenever [something] is destroyed" as
-- a header — the modern templating for that event is "dies", which is a
-- different row. `EventUnclaimed` is the only class no reader claims.
failing "Triggerable"
  badTriggerOnDestruction : Ability
  badTriggerOnDestruction = Triggered Whenever (IsDestroyed (a creature)) drawACard

-- The untap step's beginning is written zero times in every possession —
-- the turn-part grid's emptiest row, and `PartUnattested` says so.
failing "PartTriggerable"
  badTriggerAtUntapStep : Ability
  badTriggerAtUntapStep = Triggered At (BeginningOf UntapStep Nothing) drawACard

-- Nor the turn's own beginning: "at the beginning of your turn" is zero
-- lines, because the UPKEEP is what English names there.
failing "PartTriggerable"
  badTriggerAtYourTurn : Ability
  badTriggerAtYourTurn = Triggered At (BeginningOf Turn (Just Yours)) drawACard

-- The other silence, and it is the possessor gap chapter twenty-seven
-- measured from the activation side: "At the beginning of each upkeep"
-- (thirty-six), "each opponent's upkeep" (thirty-three) and "the upkeep
-- of enchanted creature's controller" (twenty-seven) are real headers
-- whose possessor is a quantifier or a nominal, and `Whose` is a
-- two-word pronominal vocabulary. Unpossessed is not what they write.
failing "PartTriggerable"
  badTriggerAtTheUpkeep : Ability
  badTriggerAtTheUpkeep = Triggered At (BeginningOf Upkeep Nothing) drawACard

-- The trigger's effect reads the event's AFTER-discourse, and this is
-- the refusal that shows it: [CR#603.6] has a zone-change trigger "look
-- for the object in the zone that it moved to", so after "Whenever a
-- creature dies" the referent is a card in a graveyard and tapping it is
-- the dead-referent refusal ([CR#701.26a]). The interception's
-- replacement, over the same event row, taps it perfectly well — the
-- event has not happened there ([CR#614.6]).
failing "OnBattlefield"
  badTriggerTapsDeadCreature : Ability
  badTriggerTapsDeadCreature = Triggered Whenever (Dies (a creature)) (Tap It)

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
  badStaticTargets = Static (Cant (target creature) Attack Agent)

-- The one static effect English does not state as a line. Its stative
-- form is a different VERB — "You control enchanted creature" (seven
-- lines) — where every other row inflects the same verb it writes as a
-- clause ("gains"/"has", "becomes"/"is"). Zero lines write a
-- durationless "gains control of".
failing "StaticLine"
  badStaticGainsControl : Ability
  badStaticGainsControl = Static (GainsControl You (AllOf creatureYouControl))

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
    Continuously (asLongAs (Exists creatureYouControl)
                           (Gets (AllOf creatureYouControl) 1 1))
                 Nothing

-- And no line conditions a statement twice: the singleton discipline
-- `badNestedCompound` keeps at the cost layer, one type up.
failing "NotConditional"
  badDoubleConditional : Ability
  badDoubleConditional =
    Static (asLongAs (Exists creatureYouControl)
                     (asLongAs (Exists (And [artifact, ControlledBy You]))
                               (Gets (AllOf creatureYouControl) 1 1)))

-- The entry rider is a static ability and not a clause either
-- ([CR#603.6d] says so in as many words), which is the third of the
-- three families chapter twenty-five sent to a container that did not
-- exist. "Put [card] onto the battlefield tapped" is the one-shot twin
-- and it is a rider on a MOVE, not a continuous effect (ledger).
failing "SpanOk"
  badEntryRiderClause : Effect []
  badEntryRiderClause = Continuously (entersTapped (AllOf creatureYouControl)) Nothing

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
    Sequentially [exile topCard, Continuously (MayPlay You It) Nothing]

-- Nor at an endpoint the permission does not write: "until end of
-- combat" is the two GRANTS' word and the permission's zero times.
failing "SpanOk"
  badPermissionUntilEndOfCombat : Effect []
  badPermissionUntilEndOfCombat =
    Sequentially [exile topCard,
                  Continuously (MayPlay You It) (Just untilEndOfCombat)]

-- The new cell is the permission's ALONE, which is what `PermissionOnly`
-- claims: eight lines write "until your next end step" and every one of
-- them permits playing just-exiled cards. No grant writes it.
failing "SpanOk"
  badGainsUntilYourNextEndStep : Effect []
  badGainsUntilYourNextEndStep =
    gains (target creature) (KeywordAbility Flying) (Just untilYourNextEndStep)

-- A permanent on the battlefield has already been played: [CR#604.6]
-- files the permission as functioning "while a card is in any zone that
-- you could cast or play it from", and the battlefield is not one. The
-- refusal is the exact inverse of `Cant`'s battlefield DEMAND, which is
-- why the permissive twin could not be the prohibition with its polarity
-- flipped.
failing "PlayableFrom"
  badPlayFromBattlefield : Effect []
  badPlayFromBattlefield =
    Continuously (MayPlay You (a creature)) (Just thisTurn)

-- Growing the container grew its refusal with it: English grants a
-- triggered ability by QUOTING it, exactly as it grants an activated one
-- ("Enchanted creature has 'When this creature dies, …'"), and this
-- grammar has no quotation.
failing "Grantable"
  badGainsTriggered : Effect []
  badGainsTriggered =
    gains (target creature) (Triggered When (Enters This) drawACard)
          (Just untilEndOfTurn)

-- …and a static ability the same way ("as long as enchanted permanent is
-- an Equipment, it has 'Equipped creature gets +1/+1 and has trample'").
failing "Grantable"
  badGainsStatic : Effect []
  badGainsStatic =
    gains (target creature) (Static (Cant (AllOf creatureYouControl) Attack Agent))
          (Just untilEndOfTurn)

-- [CR#603.7b] gives a delayed trigger ONE stated duration and names the
-- phrase: "unless it has a stated duration, such as 'this turn'". The
-- corpus writes no other — Graceful Reprieve's "when target creature
-- dies this turn" is the family, and the boundary endpoints belong to
-- continuous effects, which a delayed trigger is not.
failing "DelaySpanOk"
  badDelayedUntilEndOfTurn : Effect []
  badDelayedUntilEndOfTurn =
    Delayed (Dies (target creature)) {span = Just untilEndOfTurn}
            (Move (That CardW) battlefieldZ)

-- The delayed clause reads three events of the ten and the attack is not
-- one: "sacrifice it when this creature attacks" is written zero times,
-- the delayed family being the end-step beginning, the departure and the
-- death.
failing "Awaitable"
  badDelayedOnAttack : Effect []
  badDelayedOnAttack =
    Delayed (Attacks (target creature)) (sacrifice You (That (TypeW Creature)))

-- The event vocabulary's four readers disagree, and the entry is the
-- clearest case: two thousand eight hundred ninety-one trigger headers
-- and no would/instead clause of ours. The fourteen enter-interceptions
-- the corpus does write are the entry RIDER — [CR#603.6d]'s static
-- ability, which is now a row of its own with its own spelling — so the
-- would-clause has nothing left to say about the event.
failing "Interceptable"
  badInterceptEnters : Effect []
  badInterceptEnters =
    ifWouldInstead (Enters (a creature)) (exile It) (Just thisTurn)

-- …and the [CR#610.3] rider does not wait for one either: "exile it
-- until a creature enters" is written zero times, the rider's whole
-- corpus being the departure.
failing "Holdable"
  badHeldUntilEnters : Effect []
  badHeldUntilEnters = exileUntil (target creature) (Enters (a creature))

-- The turn-part beginning is real oracle as a duration endpoint —
-- "until the beginning of your next upkeep", twenty-eight lines — and
-- the adverbial that writes it is `DurationEnd`'s own `StartOf` row. One
-- phrase, one slot: the event axis must not spell the same endpoint a
-- second way, which is what `Unclaimed` records here.
failing "SpanOk"
  badUntilBeginningOfUpkeep : Effect []
  badUntilBeginningOfUpkeep =
    gets (target creature) 3 3 (Just (UntilEvent (BeginningOf Upkeep (Just Yours))))
