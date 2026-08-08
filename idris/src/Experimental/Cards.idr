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
                           Delayed NextEndStep (Move (That CardW) battlefieldZ)]

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
gracefulReprieve = Delayed (DiesThisTurn (target creature))
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
voyagerStaff : Activated []
voyagerStaff = MkActivated (sacrifice You thisArtifact)
                           (Sequentially [exile (target creature),
                                          Delayed NextEndStep (Move (TheVerbed Exile CardW) battlefieldZ)])

-- "{3}{R}, Sacrifice an artifact: Bosh deals damage equal to the
-- sacrificed artifact's mana value to any target." (Bosh, Iron Golem;
-- {3}{R} and its Trample line elided; the self-name is `This`) — the
-- TYPE-word participle noun: the referent is a graveyard card NOW,
-- but "artifact" describes it under the verb — checked on the
-- time-stable projected head type, the axis a declared type word
-- lives on (finding 27).
boshIronGolem : Activated []
boshIronGolem = MkActivated (sacrifice You (a (HasType Artifact)))
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
pyromancy = MkActivated (discards You (aAtRandom (InZone handZ)))
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
                          [KeywordAbility Flying] Nothing)

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
             [KeywordAbility Flying] (Just "Ballistic Boulder"))

-- "create a 0/0 black Zombie Army creature token" — amass's first leg,
-- and the rule's own words rather than a card's reminder text:
-- [CR#701.47a] defines "amass [subtype] N" as "If you don't control an
-- Army creature, create a 0/0 black [subtype] Army creature token.
-- Choose an Army creature you control. Put N +1/+1 counters on that
-- creature. If it isn't a [subtype], it becomes a [subtype] in addition
-- to its other types." Angrath, Captain of Chaos writes "Amass Zombies
-- 2"; this is the token it makes. The negated-condition wrapper is the
-- elision (`Not(Condition)` is ledgered), and the MULTI-SUBTYPE line is
-- what the leg contributes on its own: two subtypes over one card type.
amassZombiesToken : Effect []
amassZombiesToken = create (Lit 1) (creatureTok 0 0 [Black] [Zombie, Army])

-- "Choose an Army creature you control. Put two +1/+1 counters on that
-- creature. It becomes a Zombie in addition to its other types."
-- (amass Zombies 2's remaining legs, [CR#701.47a]; the last sentence's
-- "if it isn't a Zombie" wrapper elided with the negated condition, and
-- the leg is written apart from the token above because the rule's
-- branch makes them the SAME object and this grammar has no way to say
-- so — written as one term the two mentions would be two) — the subtype
-- word as a head noun, the counter clause over an anaphoric subject, and
-- the durationless type addition that is amass's whole point.
amassZombiesArmy : Effect []
amassZombiesArmy =
  Sequentially [Choose (a (And [HasSubtype Army, creature, ControlledBy You])),
                PutCounters (Lit 2) PlusOnePlusOne (That (TypeW Creature)),
                becomes (That (TypeW Creature)) (subtypesOnly [Zombie]) Nothing]

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
ashnodsTransmogrant : Activated []
ashnodsTransmogrant =
  MkActivated (sacrifice You thisArtifact)
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

-- "If you control a Demon, each opponent loses 2 life and you gain 2
-- life. Otherwise, you lose 2 life." (Unholy Annex's end-step trigger;
-- the trigger header and the sentence before it — "draw a card", no draw
-- vocabulary — elided as Blindblast's is) — chapter eighteen's ELSE arm,
-- opened with the card that finally writes both arms in vocabulary this
-- grammar has. The consequent is a two-clause sequence in a body slot,
-- which is where a sequence may still stand; the arm is a `Maybe` field
-- and reads only what preceded the conditional.
unholyAnnex : Effect []
unholyAnnex = If (Sequentially [losesLife (Each Opponent) (Lit 2),
                                gainsLife You (Lit 2)])
                 (Exists (And [HasSubtype Demon, ControlledBy You]))
                 (Just (losesLife You (Lit 2)))

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
                                  Move (That (TypeW Creature)) battlefieldZ]

-- A hidden-zone cost mention is unreadable past the colon: the card
-- bounced to hand is not among the public survivors ([CR#400.2] —
-- hand is hidden; no [CR#400.7] exception reaches it, and
-- Soratami Cloudskater-family costs write no such read back).
failing "publicOnly"
  badHiddenCost : Activated []
  badHiddenCost = MkActivated (Move (a creature) handZ) (Tap It)

-- Two cost moves leave two candidate antecedents ("Discard a card,
-- Sacrifice a creature: …" — Falkenrath Pit Fighter-family): a bare
-- "It" past the colon is ambiguous. Real costs of this shape read
-- back with definite descriptions (the participle reads of chapter
-- eight), never a bare pronoun. (The verb is exile — zone-blind —
-- so the pin isolates the ambiguity, not a zone gate.)
failing "countOnes"
  badTwoCostMentions : Activated []
  badTwoCostMentions = MkActivated (Sequentially [discardsACard You,
                                                  sacrifice You (a creature)])
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
  badVerbedWrongVerb : Activated []
  badVerbedWrongVerb = MkActivated (discardsACard You)
                                   (Move (TheVerbed Sacrifice CardW) battlefieldZ)

-- The noun word misses on the type axis: an artifact was sacrificed,
-- so "the sacrificed creature" has no referent.
failing "countVerbed"
  badVerbedWrongNoun : Activated []
  badVerbedWrongNoun = MkActivated (sacrifice You (a (HasType Artifact)))
                                   (Move (TheVerbed Sacrifice (TypeW Creature)) battlefieldZ)

-- Two same-verb stamps leave the participle ambiguous — the same
-- strict uniqueness as every read (real costs of this shape name
-- distinct verbs, which is what the filter buys).
failing "countVerbed"
  badVerbedAmbig : Activated []
  badVerbedAmbig = MkActivated (Sequentially [sacrifice You (a creature),
                                              sacrifice You (a creature)])
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
  badBareCardRead : Activated []
  badBareCardRead = MkActivated (sacrifice You thisArtifact)
                                (Sequentially [exile (target creature),
                                               Delayed NextEndStep (Move (That CardW) battlefieldZ)])

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
-- already-graveyard card cannot die this turn.
failing "OnBattlefield"
  badDiesInGraveyard : Effect []
  badDiesInGraveyard = Delayed (DiesThisTurn (target (And [creature, InZone (graveyardOf You)])))
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
failing "OnBattlefield"
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
  badDiesGroup = Delayed (DiesThisTurn (TargetGroup (exactly 2) creature)) (gainsLife You (Lit 1))

-- A bare type word denotes a permanent [CR#109.2], and what was
-- discarded left a HAND: "the discarded creature" is unwritten (the
-- corpus discard family reads "the discarded card" only) — the
-- stamp's at-verb frame refuses the type word.
failing "countVerbed"
  badDiscardedCreatureWord : Activated []
  badDiscardedCreatureWord =
    MkActivated (discards You (aAtRandom (And [creature, InZone handZ])))
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
failing "OnBattlefield"
  badCantInGraveyard : Effect []
  badCantInGraveyard =
    cantBlock (target (And [creature, InZone graveyardZ])) (Just thisTurn)

-- The class word names [CR#115.4]'s damage class, describes no object,
-- and so places none — and the restriction needed no rule of its own to
-- refuse it, the projection doing it through the battlefield demand
-- exactly as for destroy and tap (`badDestroyAnyTarget`).
failing "OnBattlefield"
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
failing "OnBattlefield"
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
