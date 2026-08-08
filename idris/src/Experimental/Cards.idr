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
-- control." (Prey Upon; its reminder text "(Each deals damage equal to
-- its power to the other.)" omitted — a parenthetical gloss, not rules
-- text, and the same expansion finding 20 refuses to treat as the
-- operative spelling) — two same-sort slots are just two argument
-- positions.
preyUpon : Effect []
preyUpon = Fights (Target creatureYouControl) (Target creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct. The oracle writes ONE verb over coordinated
-- amount+recipient complements; the bench transcribes them
-- sequentially under `AndThen` as a named stand-in, which mis-orders
-- nothing binding-wise but serializes what the card states as a single
-- instruction (ledger).
arcTrail : Effect []
arcTrail = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                   (DealDamage This (Lit 1) (Target anyOtherTarget))

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
cloudshift = AndThen (exile (Target creatureYouControl))
                     (Move (That CardW) BattlefieldZ)

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
                           (AndThen (gainsHaste (That (TypeW Creature)) Nothing)
                                    (Delayed NextEndStep (sacrifice You (That (TypeW Creature)))))

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
                                            (discardsACard (That PlayerW)))

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
                            (losesLife (That PlayerW) (Lit 3))

-- "Exile this creature, then return it to the battlefield under its
-- owner's control." (Flickering Spirit's activated ability; its
-- mana-only cost elided) — the sorted self-reference moved by the
-- EFFECT: the exile mints the new object's binding mid-sentence
-- ([CR#400.7]) and "it" reads it. "Under its owner's control" is an
-- OVERRIDE of [CR#110.2a]'s default (the ability's controller is the
-- instructed player), so its elision is meaning-carrying — pending the
-- control-assignment axis.
flickeringSpirit : Effect []
flickeringSpirit = AndThen (exile (ThisOf Creature)) (Move It BattlefieldZ)

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
turnToMist = AndThen (exile (Target creature))
                     (Delayed NextEndStep (Move (That CardW) BattlefieldZ))

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
-- its owner's control" is a meaning-carrying elision — the override of
-- [CR#110.2a]'s instructed-player default, pending the
-- control-assignment axis) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (DiesThisTurn (Target creature))
                           (Move (That CardW) BattlefieldZ)

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
-- mentions don't bind) — the SEQUENTIAL cousin of fight: two ORDERED
-- one-shot damage events, where [CR#701.14a] deals both
-- simultaneously (state-based actions see neither mid-resolution,
-- [CR#704.4]), which is why `Fights` stays primitive.
karplusanYeti : Effect []
karplusanYeti = AndThen (DealDamage (ThisOf Creature) (PowerOf (ThisOf Creature)) (Target creature))
                        (DealDamage (That (TypeW Creature)) (PowerOf It) (ThisOf Creature))

-- "Choose two target creatures. Tap those creatures, then unattach
-- all Equipment from them." (Fulgent Distraction; the unattach clause
-- elided) — a counted group mention, read back by the sorted plural
-- demonstrative.
fulgentDistraction : Effect []
fulgentDistraction = AndThen (Choose (TargetGroup 2 creature))
                             (Tap (Those (TypeW Creature)))

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
voyagerStaff = MkActivated (sacrifice You (ThisOf Artifact))
                           (AndThen (exile (Target creature))
                                    (Delayed NextEndStep (Move (TheVerbed Exile CardW) BattlefieldZ)))

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
                                        (ManaValueOf (TheVerbed Sacrifice (TypeW Artifact)))
                                        (Target AnyTarget))

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
                        (DealDamage (ThisOf Enchantment)
                                    (ManaValueOf (TheVerbed Discard CardW))
                                    (Target AnyTarget))

-- "Target opponent loses 1 life for each attacking creature you
-- control. You gain that much life." (Foul-Tongue Shriek) — the
-- event-outcome read: the loss clause introduces its outcome (sort
-- LifeLost, projected from the surface; magnitude runtime), and
-- "that much" reads the unique outcome in scope, sort-blind. The
-- attacking modifier is a battlefield state word ([CR#508.1a]).
foulTongueShriek : Effect []
foulTongueShriek = AndThen (losesLife (Target Opponent)
                                      (ForEach 1 (And [Attacking, creature, ControlledBy You])))
                           (gainsLife You ThatMuch)

-- "Return target creature to its owner's hand." (Unsummon) — the
-- destination is bare `HandZ`: [CR#400.3] admits no other hand, so
-- the possessive is derived surface, never stored (finding 34).
unsummon : Effect []
unsummon = Move (Target creature) HandZ

-- "When this Equipment enters, attach it to up to one target creature
-- you control. Destroy up to one other target creature." (Phantom
-- Blade's trigger; the attach verb waits on the attachment axis, so
-- its TARGETING stands in as the fronted choice clause) — the up-to
-- mention is a real "other" anchor: the witness is the MENTION, not
-- a nonempty denotation (finding 35).
phantomBlade : Effect []
phantomBlade = AndThen (Choose (TargetUpTo 1 (And [creature, ControlledBy You])))
                       (destroy (TargetUpTo 1 (And [creature, Other])))

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
caseOfTheGatewayExpress = AndThen (Choose (Target creatureYouDontControl))
                                  (DealDamage (Each creatureYouControl) (Lit 1)
                                              (That (TypeW Creature)))

-- "Cycling {2}" — "{2}, Discard this card: Draw a card." ([CR#702.29a];
-- the {2} and the draw elided, the draw verb being unminted) — the
-- standing justification for `DiscardOk`'s bare-`This` row: bare `This`
-- is the source as an OBJECT ("this card"), so it projects no zone,
-- while the SORTED self-reference now denotes the permanent
-- ([CR#109.2]) and cannot be discarded (`badDiscardThisCreature`).
cyclingCost : Effect []
cyclingCost = discards You This

-- "Target nonattacking, nonblocking creature gets +0/+2 until end of
-- turn." (the blocking half waits on its status word) — a presupposed
-- zone projects THROUGH negation: negating the status does not negate
-- the battlefield, so the phrase keeps the [CR#109.2] default and
-- stays coherent. The nonattacking family is plentiful oracle ("Untap
-- target nonattacking creature.", "Target nonattacking creature gains
-- reach and deathtouch until end of turn."), which is what makes
-- reading the seed through `Not` an over-refusal rather than a nicety.
rawNonattacking : Predicate [] Object
rawNonattacking = And [creature, Not Attacking]

-- "Exile target creature." spelled raw — the tag-ALIGNED twin of
-- `badDestroyTaggedExile`: same body, agreeing tag. The `TagBody`
-- witness travels explicitly because its auto search is flaky even
-- here, at a concrete site with no nested autos — the same reason the
-- `exile` macro passes `{ok = ExileB}`.
rawExile : Effect []
rawExile = Composite Exile (Move (Target creature) ExileZ) {ok = ExileB}

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
failing "countWord"
  badStaleCarrier : Effect []
  badStaleCarrier = AndThen (exile (Target creatureYouControl))
                            (Move (That (TypeW Creature)) BattlefieldZ)

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
  badTwoCostMentions = MkActivated (AndThen (discardsACard You)
                                            (sacrifice You (A creature)))
                                   (exile It)

-- The zone half of sacrifice's implicit restriction as a type error:
-- an exiled referent is not sacrificeable [CR#701.21a].
failing "OnBattlefield"
  badSacrificeExiled : Effect []
  badSacrificeExiled = AndThen (exile (Target creature)) (sacrifice You It)

-- After the watched target dies, it no longer answers to "creature":
-- the event retag flips the carrier ([CR#700.4,110.1]) — "that card"
-- is the spelling that resolves (see `gracefulReprieve`).
failing "countWord"
  badDeadCreatureRead : Effect []
  badDeadCreatureRead = Delayed (DiesThisTurn (Target creature))
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
  badThemAmbig = AndThen (Choose (TargetGroup 2 creature))
                         (AndThen (Choose (TargetGroup 2 creature))
                                  (Tap Them))

-- Two predicate-inner opponents leave "that player" ambiguous — the
-- uniqueness gate reaches inside relative clauses too. (Not
-- oracle-legal text; the guide would repeat the noun.)
failing "countWord"
  badInnerAmbig : Effect []
  badInnerAmbig = AndThen (Fights (Target (And [creature, ControlledBy anOpponent]))
                                  (Target (And [creature, ControlledBy anOpponent])))
                          (losesLife (That PlayerW) (Lit 1))

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
  badVerbedAmbig = MkActivated (AndThen (sacrifice You (A creature))
                                        (sacrifice You (A creature)))
                               (Move (TheVerbed Sacrifice CardW) BattlefieldZ)

-- Voyager Staff's shape with the bare demonstrative: two card
-- mentions (the sacrificed self, the exiled target) make "that card"
-- ambiguous — the participle is what real text switches to here.
failing "countWord"
  badBareCardRead : Activated []
  badBareCardRead = MkActivated (sacrifice You (ThisOf Artifact))
                                (AndThen (exile (Target creature))
                                         (Delayed NextEndStep (Move (That CardW) BattlefieldZ)))

-- ===== Chapter nine negatives: the audit round =====

-- Tapping takes a battlefield object [CR#701.26a]: a graveyard card
-- cannot be tapped.
failing "OnBattlefield"
  badTapGraveyard : Effect []
  badTapGraveyard = Tap (Target (And [creature, InZone (GraveyardOf You)]))

-- Only battlefield creatures fight [CR#701.14b]: a graveyard card
-- cannot.
failing "OnBattlefield"
  badFightGraveyard : Effect []
  badFightGraveyard = Fights (Target (And [creature, InZone (GraveyardOf You)]))
                             (Target creature)

-- Only combatant types fight [CR#701.14a]: a land's TypeDef declares
-- no fight participation.
failing "FightParticipant"
  badFightLand : Effect []
  badFightLand = Fights (Target (HasType Land)) (Target creatureYouDontControl)

-- Dying is the battlefield-to-graveyard transition [CR#700.4]: an
-- already-graveyard card cannot die this turn.
failing "OnBattlefield"
  badDiesInGraveyard : Effect []
  badDiesInGraveyard = Delayed (DiesThisTurn (Target (And [creature, InZone (GraveyardOf You)])))
                               (Move (That CardW) BattlefieldZ)

-- Damage reaches players and battlefield objects only [CR#120.1]:
-- the destroyed referent sits in the graveyard.
failing "DamageRecipient"
  badDamageGraveyardCard : Effect []
  badDamageGraveyardCard = AndThen (destroy (Target creature))
                                   (DealDamage This (Lit 3) It)

-- A quality cannot take damage [CR#120.1].
failing "DamageRecipient"
  badDamageToColor : Effect []
  badDamageToColor = DealDamage This (Lit 1) (A (QualityNoun Color))

-- Damage goes to battles, creatures, planeswalkers, or players
-- [CR#120.1a]: a noncreature artifact takes none.
failing "DamageRecipient"
  badDamageArtifact : Effect []
  badDamageArtifact = DealDamage This (Lit 1) (Target (HasType Artifact))

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
-- the bare predicate: nested inside `Target`'s auto search this
-- failure misreports as the outer `Headed` one.
failing "Negatable"
  badNegatedPlayerHead : Predicate [] Player
  badNegatedPlayerHead = Not AnyPlayer

-- Negation binds nothing: "a creature an opponent DOESN'T control"
-- names no opponent for "that player" to read.
failing "countWord"
  badNegatedAntecedent : Effect []
  badNegatedAntecedent = AndThen (Tap (Target (And [creature, Not (ControlledBy anOpponent)])))
                                 (losesLife (That PlayerW) (Lit 1))

-- An object is in ONE zone: contradicted zone conjuncts refuse in
-- either order.
failing "ZoneCoherent"
  badConflictingZones : Effect []
  badConflictingZones = destroy (Target (And [creature, InZone BattlefieldZ, InZone GraveyardZ]))

-- Targets are objects and players [CR#115.1]: "target color" is
-- unwritten — qualities are chosen, never targeted.
failing "Targetable"
  badTargetColor : Effect []
  badTargetColor = Choose (Target (QualityNoun Color))

-- The one-shot stat modification takes a battlefield object: a dead
-- referent doesn't get +3/+3.
failing "OnBattlefield"
  badGetsGraveyard : Effect []
  badGetsGraveyard = AndThen (destroy (Target creature))
                             (Gets It 3 3 (Just UntilEndOfTurn))

-- A card never enters another player's hand [CR#400.3]: owned
-- destinations are owner-routed, so this is unwritable.
failing "DestOk"
  badMoveToTargetsHand : Effect []
  badMoveToTargetsHand = Move (Target creature) (HandOf (Target AnyPlayer))

-- Only a battlefield permanent is destroyable [CR#701.8a].
failing "OnBattlefield"
  badDestroyGraveyard : Effect []
  badDestroyGraveyard = destroy (Target (And [creature, InZone (GraveyardOf You)]))

-- Discarding moves a card from a HAND [CR#701.9a]: a battlefield
-- creature is not discardable.
failing "DiscardOk"
  badDiscardBattlefield : Effect []
  badDiscardBattlefield = discards You (A creature)

-- The tag and its body agree: a Destroy-tagged exile would let
-- indestructible cant an exile [CR#701.8b,702.12b].
failing "TagBody"
  badDestroyTaggedExile : Effect []
  badDestroyTaggedExile = Composite Destroy (Move (Target creature) ExileZ)

-- The zone demand lives on the tag-body relation: the raw Composite
-- spelling proves what the macro proves [CR#701.8a].
failing "OnBattlefield"
  badCompositeDestroyGraveyard : Effect []
  badCompositeDestroyGraveyard =
    Composite Destroy (Move (Target (And [creature, InZone GraveyardZ])) GraveyardZ) {ok = DestroyB}

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
  badGroupPower = AndThen (Choose (TargetGroup 2 creature))
                          (gainsLife You (PowerOf Them))

-- Two cards need not share an owner [CR#108.3] — oracle writes the
-- plural relational ("their owners' hands", Aether Burst), future
-- vocabulary.
failing "OneOf"
  badGroupOwner : Effect []
  badGroupOwner = AndThen (Choose (TargetGroup 2 creature))
                          (losesLife (OwnerOf Them) (Lit 1))

-- Counted numerals are written two-up: "zero target creatures" is
-- unwritten, and the one-member form is bare "target creature", not
-- a group.
failing "AtLeastTwo"
  badZeroGroup : Effect []
  badZeroGroup = Choose (TargetGroup 0 creature)

failing "AtLeastTwo"
  badOneGroup : Effect []
  badOneGroup = Choose (TargetGroup 1 creature)

-- The binary fight frame takes singular combatants — a group versus
-- one has no defined pairing; the plural form is the reciprocal
-- "those creatures fight each other" (ledger).
failing "OneOf"
  badFightGroup : Effect []
  badFightGroup = Fights (TargetGroup 2 creature) (Target creature)

-- "a creature two target opponents control": an object has one
-- controller [CR#109.4]; the union possessor ("creatures your
-- opponents control") is the player-groups vocabulary (ledger).
-- (Probed at the predicate itself: inside a larger phrase the stuck
-- slot stalls the outer coherence search instead.)
failing "OneOf"
  badControlledByGroup : Predicate [] Object
  badControlledByGroup = ControlledBy (TargetGroup 2 Opponent)

-- Hands and graveyards are per-player zones [CR#400.1]: one zone
-- owned by two players at once is unwritable.
failing "OneOf"
  badGraveyardOfGroup : ZoneExpr []
  badGraveyardOfGroup = GraveyardOf (TargetGroup 2 Opponent)

-- The minted dies-watcher is singular (Graceful Reprieve's shape);
-- plural watches wait for corpus evidence.
failing "OneOf"
  badDiesGroup : Effect []
  badDiesGroup = Delayed (DiesThisTurn (TargetGroup 2 creature)) (gainsLife You (Lit 1))

-- A bare type word denotes a permanent [CR#109.2], and what was
-- discarded left a HAND: "the discarded creature" is unwritten (the
-- corpus discard family reads "the discarded card" only) — the
-- stamp's at-verb frame refuses the type word.
failing "countVerbed"
  badDiscardedCreatureWord : Activated []
  badDiscardedCreatureWord =
    MkActivated (discards You (AAtRandom (And [creature, InZone HandZ])))
                (DealDamage This
                            (ManaValueOf (TheVerbed Discard (TypeW Creature)))
                            (Target AnyTarget))

-- "Of their choice" is a possessive pronoun: it demands a player
-- antecedent (a subject or one distributive group [CR#608.2d]) —
-- bare "Destroy a creature of their choice" is unwritten.
failing "countOnes Player"
  badUnboundTheirChoice : Effect []
  badUnboundTheirChoice = destroy (ATheirChoice creature)

-- "That much" with nothing done yet: no outcome to read.
failing "countOnes Outcome"
  badThatMuchUnbound : Effect []
  badThatMuchUnbound = DealDamage This ThatMuch (Target AnyTarget)

-- Two event clauses leave "that much" ambiguous — outcomes obey the
-- same strict uniqueness as every read.
failing "countOnes Outcome"
  badThatMuchAmbig : Effect []
  badThatMuchAmbig = AndThen (DealDamage This (Lit 3) (Target AnyTarget))
                             (AndThen (losesLife You (Lit 2))
                                      (gainsLife You ThatMuch))

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
  badOtherCrossHead = AndThen (destroy (Target (HasType Land)))
                              (destroy (Target (And [creature, Other])))

-- Every corpus for-each domain is noun-headed: "for each you control"
-- names no set to count — the positive-head demand the determiners
-- already carry (finding 39), now on the counted-set amount.
failing "Headed"
  badForEachHeadless : Amount []
  badForEachHeadless = ForEach 1 (ControlledBy You)

-- A written numeral is at least one — "1 life for each 0 creatures" is
-- unwritten English. (The comparisons that legitimately carry zero read
-- a count rather than write one; `Lit` stays ungated.)
failing "AtLeastOne"
  badForEachZero : Amount []
  badForEachZero = ForEach 0 creature

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
  badGroupDamageSource = DealDamage (TargetGroup 2 creature) (Lit 3) (Target AnyPlayer)

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

-- A type-worded self-reference denotes the PERMANENT ([CR#109.2]), and
-- discarding moves a card from a HAND ([CR#701.9a]): "discard this
-- creature" is unwritable — cycling's cost says "this card"
-- (`cyclingCost`).
failing "DiscardOk"
  badDiscardThisCreature : Effect []
  badDiscardThisCreature = discards You (ThisOf Creature)

-- ===== Chapter fourteen negatives: the fourth wave =====

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
-- can prove — two `Target`s would not be, and are not.
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
  badAnyTargetEmbedded = A (And [creature, ControlledBy (ControllerOf (Target AnyTarget))])

-- The untracked exception belongs to the bare self-reference alone —
-- it is not a free pass for every unplaced referent. A READ whose
-- antecedent records no zone is not thereby a hand card ([CR#701.9a]
-- moves one FROM a hand), which is precisely what the old zone-level
-- gate could not distinguish. (Posed at a zoneless binding — the shape
-- a singular object mention takes before anything places it.)
failing "DiscardOk"
  badDiscardIt : Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing)]
  badDiscardIt = discards You It

-- …and the class word is no hand card either: "any target" heads no
-- zone clause, so the phrase carries the battlefield default
-- ([CR#109.2]) and discard refuses it.
failing "DiscardOk"
  badDiscardAnyTarget : Effect []
  badDiscardAnyTarget = discards You (Target AnyTarget)

-- A controller relation presupposes the battlefield: objects that are
-- neither on the stack nor on the battlefield "aren't controlled by any
-- player" ([CR#109.4]), so "a creature you control in your graveyard"
-- places its referent in two zones at once — the finding-43 shape once
-- more, a projection made honest and the existing coherence gate
-- supplying the refusal.
failing "ZoneCoherent"
  badControlledInGraveyard : Predicate [] Object
  badControlledInGraveyard = And [creature, ControlledBy You, InZone GraveyardZ]
