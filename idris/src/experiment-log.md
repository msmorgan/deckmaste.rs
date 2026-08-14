# Experimental.idr — Findings Record

The semantics-target workbench: what shape should the semantics layer
be, sitting between English and core? This module is DELIBERATELY
divorced from `Semantics` — no import, no reuse of its types — because
the verifier is what the original reasoning tool grew into under
direct-runnability pressure, and the destination has to be designable
without that baggage. Nothing here is emitted, mirrored, or run; the
typechecker is the only consumer. Grow the target shape in this file;
the eventual macro/card rewrite aims at what accumulates here.
(`Bridge` holds worked new-form ⇄ verifier-form pairs — the seed of the
lowering changes that accompany that rewrite; parked out of the build
for now so chapters here don't drag the old grammar along.)

The draft contract is `docs/decisions/semantics-v2.md` — the layer
boundary, the context-data rule, vocabulary, the constructor/macro
boundary, and the macro discipline live THERE; this module is its
evidence bench and does not restate it.

## Chapter One — The Binding Spine

Chapter one, the binding spine — findings (each backed by a positive
or a `failing` negative in `Experimental.Cards`):

1. **In-situ nouns under linear threading.** Every argument position
   is typed in the discourse context of everything textually earlier
   (`nomIntro`/`amtIntro`/`effIntro` fold mentions left to right), so
   the term IS the sentence shape. One binding context, one read
   discipline — the announce list is gone.
2. **Hoisting is what created the fight problem.** With all slots
   lifted to one prenex list, same-sort slots forced positional index
   reads past the anaphor uniqueness gate. In situ, Rabid Bite's "its"
   (`rabidBite`) is read at a point where only slot A precedes — it
   resolves by plain uniqueness, the way the English does — while a
   genuinely ambiguous pronoun is still refused (`badIt`), and a
   same-sort second slot is just another position (`preyUpon`,
   [CR#701.14a]).
3. **"Other" is a modifier with a presupposition** (its anchor set
   completed by the second-wave audit, head-TYPED by the third —
   finding 44). Oracle's "any other
   target" ([CR#115.4]) is the modifier `Other` inside one noun's
   flat modifier set — distinct from its ANCHOR, with the
   obligation that an anchor exist. The anaphoric anchor is any
   earlier same-kind target mention (`anyTargeted`; up-to
   mentions count, finding 35) of a compatible head type
   (`OtherAnchored`); the second mode anchors on the
   SOURCE ("Olivia Voldaren deals 1 damage to another target
   creature", Red Hulk's "…to any other target" — no earlier
   target exists, "other" excludes the source), which waits on the
   source entering the discourse (ledger). Anchorless "other" stays UNSPELLABLE
   (`badOther`): textual precedence replaces the old
   sibling-index `Distinct` constructor and its range gate.
   Unconstrained slots still legally share an object
   ([CR#601.2c,115.3] — one choice per instance), which is why the
   edge is opt-in predicate content, not a default.
4. **A boundary is a view derived from the determiner** (revised in
   chapter four). "…at the beginning of the next end step" stays
   attached to its clause as English attaches it; `Delayed` marks
   the clause future, and its body reads the full discourse as
   SETTLED PARTICULARS (`settleTargets`): [CR#603.7c] refers to
   particular objects determiner-blind — Turn to Mist returns "that
   card" whose referent was a target, Junkyo Bell delays sacrificing
   a live target — while announcing stays local (the delayed
   ability targets in its own event, [CR#603.3d,601.2c]; Swooping
   Pteranodon does both in one sentence), so the "other"
   presupposition stops at the boundary (`badDelayedOther`).
   Staleness is the carrier/zone question (`badStale` fails the
   sacrifice zone demand, not a read gate; the fire-time zone
   expectation of [CR#603.7c] is runtime — a no-op, not an
   illegality). No timing tag is stored.

## Chapter Two — Anaphora and Carriers

Chapter two, anaphora and carriers (category inventory:
oracle-style-guide.md §"Names, self-reference, pronouns, and anaphora"
and §"Describing objects, players, and targets"):

5. **The carrier is derived, never stored.** A binding records phrase
   projections (determiner, kind, plurality, head type) plus one piece
   of fold-state — the referent's current zone, which `Move` retags in
   place. The carrier word is a FUNCTION of those ([CR#109.2,110.1]):
   Cloudshift's exile makes "that card" resolve and "that creature"
   unspellable (`cloudshift`, `badStaleCarrier`), and the return trip
   restores the typed battlefield noun for free because the head type
   was never lost (`throughTheBreach`'s "that creature" after the
   hand-to-battlefield move).
6. **Reads are strict-unique after their filter.** `It`/`They` demand
   exactly one kind-compatible singular antecedent; `That c` demands
   exactly one after the carrier filter. No nearest-wins tiebreak
   exists at this layer — the guide's "repeat a noun instead of
   stacking ambiguous pronouns" IS the type discipline.
7. **Relational nouns introduce; re-mentions don't.** "its controller"
   is `ControllerOf It` — a new Player referent enters the discourse
   (readable as "that player"), while pronoun/demonstrative reads add
   no binding, so a re-mentioned referent never becomes its own
   ambiguity (`bitterDownfall`, `immersturmSkullcairn`).
8. **Keyword actions are Composite-tagged macros.** `destroy` mirrors
   `plugins/builtin/macros/action/Destroy.ron` — the tag deontics key
   on plus the `Move` body [CR#701.8a,701.8b]; `sacrifice` is spelled
   the same way and recorded as a core whittling candidate
   [CR#701.21a]; `exile` is speculative pending its real macro. The
   retag concentrates in `Move`, so every zone-change verb inherits
   carrier tracking through its body (`bitterDownfall`'s "its
   controller" still resolves after the destroy).

## Chapter Three — Clause Structure

Chapter three, clause structure (evidence: Diabolic Edict, Innocent
Blood, Cry of Contrition, Pyrite Spellbomb, Immersturm Skullcairn;
Browbeat/Risk Factor for the decider split. The clause SEQUENCE is
n-ary, as core's `OneShotEffect::Sequentially` is: a card writes a
LIST of sentences, and the binary `AndThen` this replaced imposed a
right-nested tree no card has. Its elements are a TELESCOPE, each
typed in what its predecessors introduced — finding 1's threading
made a constructor — and it runs two clauses at minimum, one being
the clause itself and none being no instruction at all
(`badSingletonSequence`, `badEmptySequence`). Its elements are
clauses and not sequences: nesting one re-mints the very tree the
list replaced, spelling a three-sentence card twice
(`NotSeq`, `badNestedSequence`), while the slots that take a BODY
— a "may" arm, a delayed clause, a tagged composite — go on
holding whatever the card brackets there):

9. **Subjects are the factored who-slot.** Verbs the CR gives a
   player actor ([CR#701.21a,701.9a]) put their performer in clause
   position (`Does`), typed before their phrase, and REQUIRE it;
   the imperative supplies an explicit `You`; effect-verbs take a
   subject OPTIONALLY (finding 38's correction — destroy and exile
   spell both ways), and an
   object source (DealDamage's src) is the verb's own argument. A
   dependent context cannot re-use the subject term at each inner
   slot the way the real macros ride their agent param, so the slot
   factors to the clause and lowering redistributes it.
10. **The may-decider is a slot, not the performer.** `May` names
   its decider ([CR#608.2d]; resolving default the controller
   [CR#608.2c]) over an arbitrary clause — "[player] may have
   [source] deal …" separates decider from performer, killing the
   auxiliary-subject reading (`throughTheBreach`'s "You may put").
11. **Choice method is surface data, and its own axis.** "of their
   choice" / "at random" MARK one indefinite determiner
   (`Indefinite` over `ChoiceMode`; `a`/`aTheirChoice`/`aAtRandom`
   spell the three), mirroring the real macros' explicit chooser
   slot and its absence in the random variant ([CR#701.9b]); no CR
   rule derives a chooser, and the corpus has no bare "Target
   player sacrifices a creature" (`diabolicEdict`,
   `innocentBlood`). Core marks it the same way round — one
   `ChooseOne` with a `by` slot, not a second article
   (`binder.rs`) — and the pronoun's obligation rides the marked
   mode, where the possessive is what needs an antecedent.
12. **The colon is a public-zone filter.** An activated effect reads
   its cost's mentions through `publicOnly` ([CR#400.2], current
   zone): a tapped cost-mention survives unmoved, the moved sorted
   self-reference (`AsType`; `thisArtifact`) mints the new object's binding
   ([CR#400.7]) that the effect's "It" reads ([CR#400.7j] —
   `pyriteSpellbomb`, `immersturmSkullcairn`), a bounce-to-hand is
   unreadable past the colon (`badHiddenCost`), and two cost moves
   make a bare pronoun ambiguous (`badTwoCostMentions` — real text
   switches to definite descriptions there).
13. **Owned zones where English owns them, and only where a
   player has one.** A zone phrase is a zone and a SCOPE
   (`ZoneAt`): "your hand" is the hand under a possessive
   (`handOf You`), and expansions keep the sort-only form so the
   CR's owner-routing ([CR#701.8a,701.9a]) never injects phantom
   mentions. WHICH zones take a possessor is [CR#400.1]'s own
   division — each player has a library, a hand, and a graveyard,
   the rest are shared — so it is a closed table over the zone
   (`Possessable`) rather than a pair of possessive constructors:
   "your battlefield" is refused by the ZONE
   (`badOwnedBattlefield`), where the group possessor is refused by
   the noun (`badGraveyardOfGroup`). Core divides it the same way,
   naming zones bare (`StatePredicate::InZone(Zone)`) and owners
   beside them (`RelationPredicate::Owner`, `filter.rs`). And
   sacrifice demands its referent stand on the battlefield
   (`OnBattlefield`, `badSacrificeExiled`) — the zone half of the
   verb's implicit restriction, controller half parked.

## Chapter Four — Reference Machinery

Chapter four, reference machinery (evidence: Suspended Sentence,
Turn to Mist, Flickering Spirit; Junkyo Bell and Swooping
Pteranodon ground the boundary revision):

14. **Relative clauses fold their mentions.** `nounDelta`/`predDelta`
   make a phrase's introductions one computation: "target creature
   an opponent controls" leaves the opponent readable ("That
   player…", `suspendedSentence`), and two inner opponents are
   refused as ambiguous (`badInnerAmbig`) — the uniqueness gate
   reaches inside predicates.
15. **The delayed boundary settles; it does not drop.** Chapter
   one's target filter modeled the wrong axis — see revised finding
   4: reads pass as settled particulars, while the "other"
   presupposition and fresh announcing stay local to the delayed
   ability.
16. **The self-reference moves like anything else.** An
   effect-position ascribed-self move (`AsType`) mints the new object's binding
   mid-sentence ([CR#400.7]) — Flickering Spirit's "it" reads its
   own exile.

## Chapter Five — Temporal Grammar

Chapter five, temporal grammar (evidence: Jump, Giant Growth, Bond
of Revival, Graceful Reprieve, Vraska's Stoneglare, Phthisis,
Karplusan Yeti):

17. **Duration is trailing-adverbial data.** The clause carries the
   stated duration ([CR#611.2a]); the unstated form is an explicit
   `Nothing` (lasts until end of game) per the written-Nothing
   convention — no defaults. WHICH duration a clause may state is
   not free, though: it is the construction's own word (finding
   55). The slot moved onto the `Continuously` envelope in chapter
   seventeen, which is where the adverbial belonged all along —
   one slot for every continuous clause, not one per construction
   (finding 65).
18. **An event query transforms the delayed context.** `Delayed`
   names what it waits for; a time query settles only, and "when
   [target] dies this turn" both announces its watched referent and
   retags it to the graveyard ([CR#700.4]) — the event is a zone
   transition, so `gracefulReprieve`'s "that card" resolves and
   "that creature" is refused (`badDeadCreatureRead`).
19. **Last-known reads are the standing semantics.** Reads ignore
   zone, so a dead referent's characteristics stay readable
   (`vraskasStoneglare`; `phthisis` threads "power plus toughness"
   as amount arithmetic); which VALUES those reads see is runtime —
   the ability layer's story.
20. **`Fights` is confirmed primitive, and the granularity round
   found out WHY.** No operative oracle text spells the
   mutual-damage expansion (reminder text only), and the sequential
   family (`karplusanYeti`) is not equivalent — it deals two
   ORDERED damage events where [CR#701.14a] deals both
   SIMULTANEOUSLY (event granularity is observer-dependent,
   [CR#700.1]); the order is observable to triggers and
   replacements, while state-based actions see neither
   mid-resolution ([CR#704.3,704.4] — the audit's correction of
   this finding's original rationale). The round then tried to make
   it a macro anyway, core's own expansion being exactly that (a
   guarded `Simultaneously` pair of `DealDamage` clauses,
   `plugins/builtin/macros/effect/Fight.ron` — whose existence
   contradicts `OneShotEffect::Simultaneously`'s own doc comment
   upstream, a flag for trunk). It cannot be written here, and the
   obstacle is not the missing batch constructor. A damage clause
   types its recipient in the context its SOURCE's mentions built,
   so a reciprocal pair wants each participant in both roles —
   each participant in two contexts at once. Telescoped
   (`b` after `a`), the SECOND clause is refused: `nounDelta a ++
   bs` against `nounDelta b ++ nomIntro a`. With both participants
   in one context — exactly what a non-threading simultaneous batch
   demands of its elements — the FIRST clause is refused instead:
   `bs` against `nounDelta a ++ bs`. Both were put to the compiler
   rather than argued. What would dissolve this is weakening a noun
   into a larger context, and that must not exist: `It` is gated on
   there being exactly ONE antecedent (`countOnes Object bs = 1`),
   a gate a larger context breaks, so weakening is unsound wherever
   it is not vacuous. English dodges it the way English always
   does, with anaphora — the reminder text is one clause, "(Each
   deals damage equal to its power to the other.)": a distributive
   subject over the pair, and the group COMPLEMENT in the second
   slot, so each participant is mentioned once and read back. That
   complement is the missing PRIMITIVE the definite sweep ledgered
   (a binding records plurality and determiner, never how many), so
   the macro waits on it and not on a batch constructor —
   `Simultaneously` is not minted here, because it would arrive
   with no writable body, and this file deletes vocabulary that has
   none rather than accreting it. Chapter twenty-two minted the
   batch on the exchange family's witnesses and put both halves of
   this finding to the compiler (finding 120): the expansion is
   writable when one participant is the SOURCE and refused for the
   two-target frame, so the obstacle really was the pair
   complement, and `Fights` stays primitive.

## Chapter Six — Group Reference and Qualities

Chapter six, group reference and qualities (evidence: Fulgent
Distraction, Continue?, Sudden Demise, Kindred Dominance; the
amounts axis stayed evidence-only — see the not-settled list):

21. **Groups are bindings like any other.** A counted target
   mention ("two target creatures", "up to four …") binds once, and
   its quantity is ARITY data: the only thing any consumer reads off
   it is grammatical number (`quantPlur`), never a membership fact
   ([CR#601.2c] fixes the count at announce, and the one read that
   wants a number wants the ACTUAL count, not the bound).
   `Them`/`Those c` are the strict-unique plural twins of the
   singular reads (`badThemAmbig`), riding the same carrier and
   retag machinery.
22. **The fronted choose-sentence is a scope divergence.** "Choose
   two target creatures." announces in its own clause and scopes
   everything after; core's choose binders are resolution-time and
   nontarget ([CR#115.1]), so the divergence is recorded on the
   constructor, not mirrored away.
23. **Qualities are a kind, not a carrier.** `Kind` grows one
   parameterized constructor (`Quality`); a chosen color/type
   enters the discourse like any mention and `OfChosen` demands it
   uniquely per sort (`badChosenWrongSort`). A quality mention has
   no carrier word — `carrier` returns `Maybe`, junk-free — and
   kind matching routes through `sameKind`, whose deliberate lack
   of a catch-all makes the NEXT kind a totality error instead of a
   silent zero (the old fused-carrier derivation this finding named
   is decomposed in finding 28 — quality mentions now simply fail
   every noun word). "All" is a surface determiner distinct from
   distributive "each" (the guide separates them; the CR does not),
   stored as `AllD`.
24. **"That much" / "that many" read event results, not amounts.**
   The corpus antecedents are quantities of what HAPPENED — cards
   moved (Asmodeus the Archfiend), aggregate life lost (the Extort
   reminder), an intervening-if's count (Feast of the Victorious
   Dead), payment and cost quantities (Harnessed Lightning, Arcee)
   — runtime facts, not surface projections of any clause's amount
   term, so the read is NOT typed this chapter; the sole
   written-amount coincidence (Foul-Tongue Shriek) would prove the
   wrong rule.

## Chapter Seven — The Payload Split

Chapter seven, the payload split (the ruling is recorded in the
decision record §3):

25. **Context data is kind-indexed.** `Payload k` gives each kind
   exactly its own data — objects a head type and zone fold-state,
   players and qualities nothing — so a binding cannot record what
   its kind cannot have: "a player in your hand" is unrepresentable
   in the CONTEXT (`badPlayerInHand`) exactly as it is in the
   surface grammar (`InZone` is Object-kinded), and a junk write (a
   zone retag on a player) is refused by construction instead of
   absorbed by a dead field. The per-kind matches keep the
   no-catch-all discipline: a new `Payload` constructor is a
   totality error in `carrier`/`bindingZone`/`pubB`/`setZone`, not
   a silent pass-through.

## Chapter Eight — Definite Reads

Chapter eight, definite reads (evidence: Voyager Staff, Bosh, Iron
Golem, Pyromancy; corpus: participles of tagged verbs are frequent
— sacrificed/exiled/discarded — while untagged verbs' are near
absent: returned/destroyed/tapped):

26. **The participle is a provenance filter.** A tagged move
   stamps its verb on the binding it retags (`ObjectP`'s third
   component — object-only, which the payload split makes free); a
   bare move clears it, so the participle names the LAST verb
   event. "The [verbed] [noun]" (`TheVerbed`) reads the unique
   stamped mention: where the bare demonstrative is ambiguous —
   Voyager Staff's effect holds two card mentions, the sacrificed
   self and the exiled target — oracle switches to the participle,
   exactly as finding 12 predicted (`badBareCardRead`). Provenance
   follows the TAG channel: the dying retag stamps nothing
   ([CR#700.4] is an event, not a keyword action), matching the
   corpus absence of "the destroyed …" reads. Provenance is its
   own query (`stampedBy`) and the word its own check
   (`verbedWordOk`); `stampWordOk` is their conjunction, and the
   read's uniqueness (`countVerbed`) is counted over it.
27. **The participle noun keeps the word axes apart.** Ruling:
   card types/subtypes/supertypes are intrinsic to NEITHER core
   nor semantics — each is a macro-DECLARED catalog word carrying
   its rules grants (`cardtype/Creature.ron`, down to
   `permanent: true`). So `NounWord` splits the axes: a TYPE word
   checks the time-stable projected head type ("the sacrificed
   artifact" — Bosh: a graveyard card NOW, an artifact under the
   verb), the intrinsic CARD word checks current zone fold-state
   ("the exiled card" — Voyager Staff); both attested, uniqueness
   supplied by the verb filter. The word now fixes the phrase's
   KIND as it does for the demonstrative (`kindOfW`), rather than
   the read assuming `Object` over a word that may name a player.
   The axes separate as QUERIES and not as constructors: a
   word-free provenance read ("the sacrificed") is not English and
   can answer no projection at all, because WHICH anchor to read
   is the word's own business — the at-verb frame for a type word,
   the current zone for the card word — so every participle scan
   is keyed on the pair and has nothing to say about a verb alone.
   Probed: a word-free row leaves eight noun functions
   non-covering (`nounZone`, `nounTy`, `nounPlur`, `nounDelta`,
   `nounEqRef`, `nounAnyTargetFree`, `nounIsAnyTarget`,
   `moveIntro`).
28. **The demonstrative shares the split word vocabulary.**
   `That`/`Those` take the same `NounWord`, anchored to the
   CURRENT state where the participle anchors to the verb event
   (`wordNow` vs `wordOk`), and the fused `Carrier` type dissolves
   — a type word checks projected type + battlefield, the
   intrinsic CARD and PLAYER words check zone and kind. Same
   refusals as before (the carrier negatives re-pin on
   `countWord`); no fused type-carrier value remains in the model.

## Chapter Nine — Verb Frames and Agreement

Chapter nine, verb frames and agreement (an adversarial audit —
Codex and agy probed for terms that typecheck but should not;
every accepted finding is now a gate plus a pinned negative):

29. **Verb slots demand their frames.** Damage recipients are
   players or battlefield objects, never qualities ([CR#120.1] —
   `badDamageGraveyardCard`, `badDamageToColor`); tap, fight,
   destroy, and the dying query take battlefield referents, fight
   creatures specifically
   ([CR#701.26a,701.14b,701.8a,700.4] — `badTapGraveyard`,
   `badFightGraveyard`, `badFightLand`, `badDestroyGraveyard`,
   `badDiesInGraveyard`, `badGetsGraveyard`); discard takes a hand
   card ([CR#701.9a], `badDiscardBattlefield`); "target" takes
   objects and players only ([CR#115.1], `badTargetColor`); move
   destinations are owner-routed, so only bare zone destinations
   are writable ([CR#400.3], `badMoveToTargetsHand`); and a tag
   agrees with its body — a Destroy-tagged exile is unspellable
   ([CR#701.8b,702.12b], `badDestroyTaggedExile`). The participle
   type word likewise demands its referent stood on the
   battlefield AT the verb (the stamp's `wasField`): "the
   discarded creature" is unwritten (`badDiscardedCreatureWord`).
30. **Reads agree in number.** Possessive amounts and relational
   nouns take singular arguments ([CR#208.1] — power/toughness
   are one object's numbers; aggregates are explicit, "the total
   power of the sacrificed creatures", Soulblast; owners are per
   card [CR#108.3], "their owners' hands", Aether Burst, being
   the plural relational, future vocabulary): `nounPlur` projects
   grammatical number (`badGroupPower`, `badGroupOwner`). Counted
   mentions refine: the quantity is a written one, so its maximum
   is at least one (`badZeroGroup`), and the number it projects is
   its own — "up to one" and the exact one both bind singular (Ty
   Lee, Chi Blocker's "It" remention; `quantPlur` reads the maximum,
   which is what both spellings share).
   This finding's original refusal of up-to mentions as "other"
   witnesses is repealed by finding 35.
31. **Marked clauses carry their obligations.** "Of their choice"
   demands exactly one player antecedent — a subject or one
   distributive group ([CR#608.2c..608.2d];
   `badUnboundTheirChoice`). This finding's companion claim — that
   `Does` admits only agentive clauses — is corrected by finding
   38: the actor is REQUIRED for sacrifice and discard
   ([CR#701.21a,701.9a]) and OPTIONAL for destroy and exile.

## Chapter Ten — Event Outcomes

Chapter ten, event outcomes (evidence: Foul-Tongue Shriek; the
recon corpus — "that much" antecedents are damage, life, and mana
both produced AND spent (Tellah, Great Sage: "If eight or more
mana was spent to cast that spell, sacrifice Tellah and it deals
that much damage to each opponent." — the if-condition supplies
the scalar, the third-wave audit's correction of this header);
"that many" consumers are counters, draws, and tokens; "this way"
is the largest family of all):

32. **Event outcomes are the third binding class, realized.** An
   event clause prepends an outcome mention — kind `Outcome`,
   payload its surface-projected SORT alone (damage dealt, life
   gained/lost); the magnitude stays runtime (§3's ruling).
   `ThatMuch` reads the unique outcome in scope as an amount,
   SORT-BLIND — the corpus crosses damage→life, count→life,
   damage→mana — under the same strict uniqueness as every read
   (`badThatMuchUnbound`, `badThatMuchAmbig`). Only clauses
   introduce outcomes: no determiner phrase binds one (`Phrasal`
   keeps `bindFor` total), and every existing read's kind filter
   excludes them for free — the payload architecture absorbing a
   fourth kind without touching object data.
33. **Countability belongs to the read site.** "That much" vs
   "that many" is the reading phrase's choice, not stored data —
   both consume the same event class; the count reads and the
   "this way" participant subsets wait on their consumer
   vocabulary. Overgeneration accepted: an outcome follows a
   literal-amount event too, though oracle style repeats the
   literal instead of writing "that much" — linearization's
   business, like extraposition; and Fights/Move events stay
   outcome-silent until a read wants them.

## Chapter Eleven — Owner-Rooted Zones

Chapter eleven, owner-rooted zones (evidence: Unsummon; the
morning rulings):

34. **The destination possessive is derived, not stored.** A card
   reaches only its owner's hand/library/graveyard — [CR#400.3]
   redirects any other — so "to its owner's hand" adds nothing to
   the zone sort: the bare zone IS the owner-rooted destination,
   `DestOk`'s bare-zone gate is the complete destination grammar
   for those zones (an owned destination naming ANY chooser stays
   unwritable — `badMoveToTargetsHand`), and rendering re-adds the
   possessive ("your hand" only where the owner is contextually
   You, as in search effects). PREDICATE possessives contrast: "a
   card in an opponent's graveyard" FILTERS by owner — the same
   rule fuses zone and ownership into one fact — so the OWNED
   scope stays real inside predicates while `DestOk` admits the
   bare one only. And the
   other parked gate closes the opposite way: a self-fight is
   DEFINED — [CR#701.14c] has it deal twice its power to itself —
   so distinctness is per-card templating ("another", the `Other`
   modifier) and `Fights` takes no distinctness gate, ever.

## Chapter Twelve — The Second Wave

Chapter twelve, the second wave (evidence: Phantom Blade; the
audits' verified corrections, corpus- and CR-checked one by one):

35. **"Other"'s witness is the mention, not the denotation.**
   Phantom Blade pairs an up-to-one target with "up to one OTHER
   target creature": the anchor SLOT exists even when its
   denotation may be empty ([CR#115.6]), distinctness over an
   empty anchor is vacuous, and distinctness stays predicate
   content ([CR#601.2c]). `anyTargeted` counts up-to mentions;
   finding 30's contrary refusal is repealed. That the two spellings
   are one witness is why they now share one determiner tag — the
   emptiness is the quantity's, not the word's.
36. **Grammatical number gates the argument slots.** The binary
   fight frame takes two singular combatants ([CR#701.14a];
   every corpus "X fights Y" line is singular — the plural form
   is the RECIPROCAL frame "those creatures fight each other",
   its own future construction); possessors are singular (one
   controller [CR#109.4]; per-player zones [CR#400.1] — the
   union read "creatures your opponents control" waits with the
   player groups, "their owners' hands" with the plural
   relationals); the minted dies-watcher is singular; and a
   counted mention carries its number in its QUANTITY. One
   constructor serves every quantity (`TargetGroup`, mirroring
   core's single `TargetSpec::Target(Quantity, Predicate)` announce
   form) and the named forms are macros over the one `Range`
   primitive exactly as core's are — `target p = TargetGroup
   (exactly 1) p`, `upTo n`, `anyNumber` — so both the singular and
   the up-to phrase are spellings of one mention, and two-up
   survives as a RENDERING rule rather than a type floor: the
   numeral is unwritten at one ("target creature", never "one target
   creature") and written from two ("two target creatures"). What
   the type still refuses is a quantity that admits nothing or
   reads backwards: the MAXIMUM is at least one (`NonZeroQ`,
   `badZeroGroup`), which kills "up to zero" in the same stroke as
   "zero", and the range runs UPWARD from a written minimum of at
   least one (`WellFormedQ`) — a descending pair names an empty
   interval and would read as plural besides, and a zero minimum
   spells what "any number of" already spells, [CR#107.1c] having
   "any number" permit zero outright (`badDescendingRange`,
   `badZeroLowerRange`). Fight
   participation reads a GRANT, not a type name: `combatant` is
   the stand-in for a TypeDef-declared combat-participant grant
   — distinct from May(Attack)/May(Block), fight keying on type
   membership and dealing non-combat damage [CR#701.14d] — so a
   future type joins by declaration, not by gate rewrites.
37. **Obligations live on relations, not phrases.** The tag-body
   witness carries each verb's SOURCE-zone demand ([CR#701.8a]
   destruction moves a battlefield permanent, [CR#701.9a]
   discarding a hand card; exile zone-blind), so raw
   `Composite`/`Does` spellings prove exactly what the macros
   prove (`badCompositeDestroyGraveyard`,
   `badDoesDiscardBattlefield`) and a forged provenance stamp is
   unwritable — only a legal tagged move writes one. The damage
   recipient likewise gained its head-type demand ([CR#120.1a];
   `badDamageArtifact`).
38. **Agentivity is one table, read by its MISSING rows** (the
   third-wave audit's correction: the original claimed two tables
   and refused a subject on destroy — an actor is required
   exactly where `NonAgentive` has no row). `Does` carries the
   verb tag itself and demands nothing of the verb; `Composite`
   demands `NonAgentive`, so sacrifice and discard cannot shed
   their actor ([CR#701.21a,701.9a]; `badAgentlessSacrifice`)
   while destroy and exile take one OPTIONALLY — real oracle text
   writes both, "You destroy four lands you control, then target
   opponent destroys four lands they control." (Burning of Xinye)
   and "Each player exiles two cards from their hand." (The
   refused negative went with it; the Burning of Xinye positive
   waits on counted untargeted groups.) Overgeneration accepted:
   a choice-free subject form ("You destroy target creature") is
   spellable. The old conservative `clauseVerb` catch-all
   dissolves with nothing left to leak.
39. **Phrases are well-formed or unwritable.** Determiner
   phrases demand a positive HEAD (`Headed` — "choose a
   noncolor" and "target non-player" head nothing,
   [CR#105.1,608.2d]); a conjunction's explicit zones must agree
   (`ZoneCoherent` — an object is in one zone, and the gate no
   longer depends on conjunct order); and negation is a binding
   HOLE (`Not`'s inner mentions do not fold: "a creature an
   opponent doesn't control" names no opponent — a positive
   relation names the one controller [CR#109.4], a negated one
   selects nobody; `badNegatedAntecedent`).

## Chapter Thirteen — The Third Wave

Chapter thirteen, the third wave (both ducks probing for terms
that typecheck but should not; every accepted finding is a gate
plus a pinned negative, and each negative is probed at the
SMALLEST construct carrying its gate — an auto-search failure
nested inside another auto stalls the outer search and misreports,
so the bench writes bare `Predicate`/`Amount`/`Noun` probes
wherever one exists):

40. **"Any target" is a lone class word.** The guide reserves it
   for the rules-defined damage target class ([CR#115.4]) and
   forbids it as a synonym for "any object". Four refusals
   follow: it takes no modifier but "other" (Arc Trail's "any
   other target" is the sole corpus companion — `AnyTargetLone`,
   `badAnyTargetInGraveyard`); it is never negated, there being no
   complement class to name (`Negatable`, which also refuses
   double negation — `badNegatedAnyTarget`); and it is ITSELF the
   targeting form, so the non-targeting determiners and the
   for-each domain demand an any-target-free phrase — "a any
   target" and "each any target" are unwritable (`AnyTargetFree`,
   `badAnyTargetUnderA`); and it DESCRIBES no object, so
   [CR#109.2] has nothing to place on the battlefield and the
   phrase projects no zone — which is what refuses it to every
   battlefield-demanding verb (`nounZone`, `badDestroyAnyTarget`,
   `badTapAnyTarget`) while damage still takes it, by a recipient
   row read off the NOUN (`DamageRecipient`, `badDamageThis`).
   What is stated here is the SINGULAR form's rule:
   [CR#115.4] names "another target," "two targets," and similar
   plural damage-class forms in the same breath, and those carry
   their own structures (a division, an each-of recipient —
   ledger), so "lone class word" constrains "any target" itself,
   not the family it belongs to. That makes the third refusal a
   question of the QUANTITY once the counted mention is one
   constructor (finding 36): the class word belongs at exactly
   one, where the phrase IS the singular form ("Lightning Bolt
   deals 3 damage to any target"), and at any up-to bound, which
   the corpus writes outright ("each of up to two targets", Fall
   of the Titans); the exact group from two up and the unbounded
   "any number of" are refused until the plural structures land
   — `AnyTargetAtCount`, `badGroupAnyTarget`,
   `badAnyNumberAnyTarget`. What those two quantities permit is the
   class word as the phrase's HEAD, not the word wherever it hides:
   a possessor buried in a relative clause spells it under a
   counted mention exactly as it does under "a"
   (`badEmbeddedAnyTargetExact1`), so the embedded scan runs
   beneath every quantity.
41. **The sorted self-reference stands on the battlefield.** "This
   creature" / "this enchantment" is a description including a
   card type, so [CR#109.2] denotes the PERMANENT: the ascription
   projects the battlefield where bare `This` — the source as an
   object ("this spell", cycling's "Discard this card") — projects
   nothing. The granularity round then put the two halves on their
   own axes: identity is `This` (core's `Reference::This` carries
   no type either), the type word is `AsType` over it, and the
   battlefield projection belongs to the WORD, not to the
   self-reference — which is the rule's own reading. What may be
   ascribed stays a closed table (`Ascribable`, the source alone),
   so the projection cannot silently follow the constructor onto a
   noun that never earned it. Discarding the sorted form is thereby unwritable
   ([CR#701.9a] moves a HAND card; `badDiscardThisCreature`), and
   the cost that justifies discard's bare-`This` row is cycling's
   own ([CR#702.29a]; `cyclingCost`) — the row now sits on the NOUN
   (`DiscardOk`), so it exempts that one word rather than every
   referent whose zone happens to be untracked (`badDiscardIt`).
   The audit then closed the
   opposite permissive row for good: `OnBattlefield`'s untracked
   constructor existed ONLY for the sorted self-reference, so
   deleting it left the whole bench standing — every
   battlefield-demanding slot now demands the battlefield, full
   stop. (`Enchantment` joined the type catalog in the same pass:
   Pyromancy's "This enchantment deals …" had been spelled with
   bare `This` as a workaround for the missing row.)
42. **A phrase may not contradict itself.** Zone negation is real
   oracle — "Each Vampire creature card you own that isn't on the
   battlefield has madness." (Falkenrath Gorger) — so `Not (InZone
   …)` stays writable; what a conjunction cannot do is rule OUT
   the zone it places its referent in, [CR#109.2]'s battlefield
   default included (`ZoneCoherent`, `badNotOnBattlefield`), nor
   hold a member that syntactically negates a sibling
   (`ContradictionFree`, `badQualityContradiction`). Both scans
   FLATTEN nested conjunctions, so a clash one level down is no
   laundering (`badNestedContradiction`). The member equality they
   share is deliberately conservative on the rows carrying a noun:
   "not provably the same" under-refuses rather than over-refuses.
   It is NOT conservative on the rows carrying structure, though it
   began that way: a conjunction compares member by member, so the
   same phrase written twice is recognized wherever the equality is
   consulted — including as an alternative repeated word for word,
   which the blanket row waved through (`badRepeatedStructuredDisjunct`).
43. **Status words seed the zone they presuppose.** Attackers are
   declared from creatures their controller controls
   ([CR#508.1a]), and leaving the battlefield removes a permanent
   from combat — it "stops being an attacking … creature"
   ([CR#506.4]) — so `Attacking` projects the battlefield exactly
   as a zone clause does. The nonsense that follows ("an
   attacking creature in your hand") is then refused by the
   EXISTING coherence gate rather than a new one
   (`badAttackingInHand`) — the cheapest shape a finding can take:
   a projection made honest, and the refusal falls out.
44. **"Other" is typed by its own head.** The anchor obligation
   splits in two: `Other`'s own gate demands a same-KIND target
   mention, and the conjunction it sits in supplies the head TYPE
   that mention must be compatible with (`OtherAnchored`; an
   untyped head — Arc Trail's "any other target", a player-kind
   "other" — accepts any same-kind anchor, which is exactly
   finding 3's gate). The head is a SET, once a coordination can
   write one: "another target creature or land" fixes no type on
   its referent (finding 50) but offers TWO heads to anchor
   against, and reading that silence as untypedness let an earlier
   artifact anchor it (`badDisjunctiveOtherCrossHead`) — silence as
   permission, the third time. The guide reserves "another" for excluding
   the source or first referent and writes two separately
   described roles WITHOUT it ("target creature and target
   planeswalker"); the corpus pairs "other" only with overlapping
   heads. Sharing one object across disjoint-headed slots stays
   rules-LEGAL ([CR#601.2c]'s artifact-land example), which is
   precisely why this is templating enforced where the head type
   is known (`badOtherCrossHead`).
45. **The counted-set amount is a noun phrase.** Every corpus
   for-each domain is noun-HEADED — "for each you control" names
   no set to count (`Headed`, `badForEachHeadless`) — and its
   per-unit is a written numeral, hence at least one
   (`AtLeastOne`, `badForEachZero`): the same surface-numeral
   discipline finding 36 gives counted groups. The comparisons
   that legitimately carry zero READ a count rather than write
   one, so `Lit` stays ungated. The two demands answer two
   different questions, and the granularity round put each on the
   operation it belongs to: the head demand on the count read
   (`CountOf`), the numeral demand on the scaling (`Times`), which
   is core's own split (`Count::CountOf` against `Count::Times`)
   and leaves "for each" as what it is — an adverbial spelling of
   a product, owned by `forEach`/`nForEach`.
46. **Damage subjects distribute; they do not collect.** A group
   source is legal exactly when the group is distributive: "Each
   creature you control deals 1 damage to that creature." (Case of
   the Gateway Express) spreads the singular deal frame over its
   members, while a COLLECTIVE plural subject ("two target
   creatures deal …") is unattested — oracle either distributes or
   names one source (`DamageSource`, `badGroupDamageSource`). The
   distributive lines carrying a per-member "its" ("Each creature
   deals 1 damage to its controller.") wait on dependent iteration,
   which is why the bench positive reads its recipient back as a
   demonstrative instead.

## Chapter Fourteen — Disjunction

Chapter fourteen, disjunction (evidence: Disenchant, Icy
Manipulator, Rats of Rath, Arrows of Justice; the corpus counts and
the bracketed parses are this chapter's whole argument):
47. **A disjunction is a PREDICATE, not a second noun.** Disenchant
   writes "target" once — "Destroy target artifact or enchantment."
   — so the phrase announces ONE target ([CR#601.2c], whose own
   example turns on a spell using the word in two places), and the
   parse agrees about where the coordination sits:
   `<<target> <<artifact> <or <enchantment>>>>`, inside the
   determined phrase rather than beside it. That places "or" among
   the descriptions, where the kind index already forces the
   alternatives to describe the same sort of thing and no gate has
   to say so. `Or` carries a member LIST for the reason `And` does,
   and core's `Predicate::Or` does (`filter.rs`, a slice beside
   `And`'s): Icy Manipulator's third alternative then costs no
   machinery, only the guide's serial comma. Two-alternative
   "target X or Y" runs to about a thousand corpus lines and the
   Oxford form to eighty-eight, so this is a first-rank
   construction, not a curiosity.
48. **Modifier scope falls out of the nesting.** "Destroy target
   artifact, creature, or land you control." (Rats of Rath) is
   `And [Or […], ControlledBy You]` — the relative clause a SIBLING
   of the coordination, which is exactly how the parse brackets it
   (`<<target> <<artifact><, creature><, or land>> <<you>
   <control>>>`). One hundred sixty-six corpus lines write the
   shared trailing modifier and it needed nothing new; the other
   reading, the clause repeated per alternative, is what the guide
   reserves for "alternatives with different domains or modifiers".
49. **The member scans must not flatten through "or".**
   `flattenPs` flattens conjunctions only. Splicing alternatives
   into the surrounding conjunction would read Arrows of Justice's
   "attacking or blocking creature" as "attacking AND blocking" and
   refuse ninety-five corpus lines, so the coherence gates see a
   disjunction through its PROJECTIONS instead — and those project
   what the alternatives AGREE on. Both status words stand on the
   battlefield and both presuppose a creature ([CR#506.3] names
   them in one breath), so the disjunction does too, and the
   nonsense around it is refused by the gates that were already
   there (`badAttackingOrBlockingInGraveyard`,
   `badNoncreatureAttackingOrBlocking`) — finding 43's shape a
   second time. Not flattening puts the whole weight on those
   projections, so they have to reach as far as the flattening
   would have: a presupposition written inside a nested
   CONJUNCTION, which an alternative may hold, is written all the
   same, and the type seed had no such row until an "attacking
   artifact or blocking land" laundered one past the negation
   (`badWrappedStatusLaunder`).
50. **A disjunctive head is untyped, and the untyped row had to
   go.** "Artifact or enchantment" fixes no card type, so `seedTy`
   projects none — the honest silence. `DamageableTy` used to read
   a missing type word as no evidence of an illegal one, which was
   defensible only while nothing projected `Nothing` deliberately;
   a disjunction does, and the row would have made every
   disjunction damageable, including the very types [CR#120.1a]
   says damage cannot reach (`badDamageDisjunctHead`). Deleted, and
   `DamageableTy` is now the single creature row. Reading a
   phrase's silence as permission is the recurring bug; the last
   chapter found the same one in [CR#109.2]'s zone default.
51. **Alternatives are parallel, and three words are not
   alternatives at all.** Parallel in RANK — a head noun and a bare
   status word cannot stand in each other's place, which the guide
   says outright ("Repeat the carrier when the alternatives have
   different domains or modifiers") — and parallel in PLACE, since
   the phrase places its referent once. Parallel in what they
   COMMIT, too, which the place rule only looked like: comparing
   the alternatives through [CR#109.2]'s default made a committed
   zone and a silent one agree, and the disagreement then surfaced
   downstream as a projection of nothing, where a conjunction could
   place the phrase somewhere else entirely
   (`badPartialZoneJoin`). Comparing the SEEDS, silence included,
   is what makes the joins honest: a projection of nothing now
   means every alternative was silent. The three words that cannot
   be alternatives share one reason: each is written once for the
   whole coordination. "Any target" IS the union [CR#115.4] fixes
   by disjunction ("creatures, players, planeswalkers, or
   battles"), so coordinating it re-opens a closed class; "other"
   fills its one selector slot for the coordination entire ("Another
   target Wolf or Werewolf you control"); and an `Or` inside an `Or`
   is the flat coordination written with brackets oracle cannot
   print — core reaches the same shape by flattening associatively
   in `normalize`, where the workbench refuses the second spelling.
   Each alternative is read through `flattenPs`, so a singleton
   conjunction launders none of the three
   (`badAnyTargetInOrLaundered`) — the lesson finding 42's negation
   row learned, applied to a new list-carrying constructor before
   it could be exploited.
52. **Negation does not reach a coordination.** The corpus spells
   "non-(A or B)" zero times, while the De Morgan the writer does
   instead is plentiful and comma-chained: "noncreature, nonland
   card", "Target nonattacking, nonblocking creature". `Not (Or …)`
   is unwritable on that evidence (`badNegatedDisjunction`), and
   the second half of `rawNonattacking`'s card now spells, chained
   exactly as the writer chains it.
53. **A disjunction is a binding HOLE.** Exactly one alternative is
   realized and the phrase never says which, so a possessor written
   inside one names nobody the next sentence can read
   (`badDisjunctAntecedent`) — negation's rule (finding 39's
   `predDelta`) for a neighbouring reason. What is unaffected is
   the phrase's OWN binding: the determiner mints that at the noun
   layer, which is why "Destroy target artifact or enchantment"
   still leaves an "it" behind.

## Chapter Fifteen — The One-Shot Deontic

Chapter fifteen, the one-shot deontic (evidence: Infiltrate, Change
of Heart, Blindblast, Blinding Flare; the duration corpus split is
the chapter's sharpest single fact):
54. **A "can't" with a duration is a CLAUSE; a "can't" without one
   is an ability.** "Target creature can't block this turn."
   resolves and creates a continuous effect for the span it states
   ([CR#611.2a]) — core's `Continuously { effect:
   Deontic(Cant(…)), duration }` (`deckmaste_core/src/effect.rs`,
   `deontic.rs`) with exactly the two halves English writes. What
   the effect creates is a restriction the declare steps check
   ([CR#508.1c] for attacking, [CR#509.1b] for blocking and for
   being blocked), and it beats any permission it meets
   ([CR#101.2]). The static line — "Enchanted creature can't
   attack", Pacifism — is durationless, continuous, and the parked
   ability layer's; here it is unwritable (`badStaticCant`). That
   is the whole scope line, and it is the smallest bite that
   spells a real card end to end. The refusal was first the ABSENT
   SLOT — the restriction simply had no optional span — and
   chapter seventeen re-seated it on the reason instead, since the
   envelope's slot has to be optional for the grants that need it:
   `absentOk DeedRestriction = False` is the one row of that table
   that says no, and it says why.
55. **The duration adverbial is the CONSTRUCTION's word, not the
   writer's.** "This turn" and "until end of turn" name the same
   span, and the corpus does not let a clause choose between them:
   two hundred ninety-six one-shot restriction lines say "this
   turn" and not one says "until end of turn", while two hundred
   sixty-six "gets +1/+1" lines and one hundred ninety-seven "gains
   haste" lines say "until end of turn" and not one says "this
   turn". The guide prints both wordings a section apart and
   assigns neither, so the corpus is what assigns them, and the
   workbench carries the assignment as a table — two complementary
   ones at first (`restrictionSpan`, `grantSpan`), a single
   attestation table over the decomposed endpoints from chapter
   seventeen (`spanUse` and `admitsSpan`, findings 66 and 67). The
   cross-turn span is the one
   word both constructions write — detain's "can't attack or block
   until your next turn" against Bond of Revival's "It gains haste
   until your next turn" — which is what keeps the tables from
   being a single flipped bit. Minting the row was also this
   chapter's one chance to open a hole, a new duration being
   available to every construction that takes one, so the grant
   side was gated in the same edit rather than after the fact
   (`badGainsThisTurn`, `badGetsThisTurn`) — wave five's coupling
   rule, applied before the hole existed.
56. **The role is an axis, because the clause has one subject.**
   Core tells "can't block" from "can't be blocked" by which SLOT
   carries the reference (`DeonticAction::Block { by, on }`), and
   the predecessor grammar does the same (`Enact Block <agent>
   <patient>`, `Semantics.idr`). A clause has ONE subject and no
   second slot to put the distinction in, so English marks it in
   the VOICE — and the granularity round gave the voice its own
   argument (`Role`, `Agent`/`Patient`, core's two slot names)
   rather than a third deed word. The chapter's first shape spelled
   `BeBlocked` as vocabulary, which said that the passive of block
   was a verb unrelated to block; the pair says what core says. The
   verb phrase after "can't" is then the DEED under its VOICE, and
   the macros own it (`cantAttack`, `cantBlock`, `cantBeBlocked`).
   Both vocabularies stay CLOSED and every table over them is
   written out, so a new deed is a totality error that has to
   declare which types carry its grant, in which voice, before it
   can be written — the `combatant` precedent, one chapter on.
57. **The deed's grant table is not the fight table, and it reads
   the voice.** [CR#506.3]'s first sentence — "Only a creature can
   attack or block" — answers the active rows, and the passive of
   BLOCK too, because what a blocker blocks is an attacking
   creature ([CR#509.1a]). That is the same verdict `combatant`
   gives and a DIFFERENT question: the fight chapter minted that
   table precisely because fight keys on type membership and deals
   non-combat damage ([CR#701.14b,701.14d]), where these deeds are
   combat proper and read the grants Creature.ron actually confers.
   So a second table, `deedType`, written out in every direction
   like `sameKind`. It has no untyped row: a coordinated subject
   fixes no type (finding 50) and therefore cannot prove
   participation (`badCantDisjunctSubject`) — silence read as
   permission, refused a third time. Splitting the voice off the
   deed word then made one more term WRITABLE and the table had to
   answer for it: "can't be attacked" is refused of every card type
   this grammar spells, by the SECOND sentence of the same rule —
   only a player, a planeswalker, or a battle can be attacked
   (`badCantBeAttacked`). The phrase is real oracle ("The
   Aetherspark can't be attacked"; "you can't be attacked except by
   creatures with flying"), so the row is waiting on the
   planeswalker and battle types and on the ledgered player
   subject, not on a corpus witness — a deferral the table now
   states instead of a vocabulary gap that hid it.
58. **Everything else the restriction needed was already there.**
   The class word places nothing, so the battlefield demand refuses
   it with no rule of its own (`badCantAnyTarget`), exactly as it
   refuses destroy and tap; a graveyard card is refused by the same
   gate ([CR#506.4] removes a permanent that leaves the
   battlefield from combat; `badCantInGraveyard`). Grammatical
   number is NOT demanded, and the corpus is why: "Any number of
   target creatures can't block this turn." (Blinding Flare) and
   "Other creatures can't attack this turn." (Intimidation Bolt)
   restrict groups, so a plurality gate would have been invention.
   The subject enters the discourse as any clause's
   does, an anaphoric subject reads back into it (Blindblast's
   "That creature"), and the clause nests under `May`, `Delayed`,
   `Sequentially`, and a colon without a gate misfiring — audited,
   with nothing to fix. A chapter that adds one constructor and
   four gates and finds the rest already standing is the shape the
   projections were built for.

## Chapter Sixteen — The Bounded Comparison

Chapter sixteen, the bounded comparison (evidence: Defeat,
Terashi's Verdict, Pillar of Light, Lucky Offering; the corpus
split between the two comparison FRAMES is the chapter's sharpest
single fact):
59. **A bound is a MODIFIER, and all three of its parts are
   written.** "Destroy target creature with power 2 or less."
   (Defeat) puts the qualifier where "attacking" and "you control"
   already stand — a sibling in the flat modifier list, heading
   nothing (`badBareComparison`) — so the phrase needed no new noun
   layer, only a new word. What the word carries is the
   characteristic, the comparator, and the bound, which is core's
   `CharacteristicPredicate::Stat(Stat, Cmp, Count)` part for part
   (`filter.rs`, whose own example is `Stat(Power, AtLeast, 3)`).
   Four hundred sixty-three corpus lines bound a mana value, three
   hundred three a power, twenty-five a toughness: a first-rank
   construction, and one the workbench could not spell at all until
   now.
60. **There are two comparison FRAMES, and the bound's shape picks
   the one.** Core's `Cmp` offers five comparators (`condition.rs`)
   and English writes two of them against a numeral. "With power
   less than 4" and "with mana value greater than 3" are written
   zero times each; the strict comparators appear only against a
   PHRASAL standard, and in the other word order — "with power less
   than Yasova Dragonclaw's power", "with mana value less than or
   equal to the number of lands you control", ninety-nine lines of
   "less than or equal to" alone. So the postposed frame takes a
   WRITTEN bound and nothing else: a numeral, or the X announced
   with the cost ([CR#107.3a]; ninety-one lines), never an amount
   that has to be computed (`badPhrasalBound`, `WrittenBound`). The
   english crate had already cut the surface at exactly this joint
   and for its own reasons — `QuantityRepr::OrComparison(value,
   word)` for this frame against `ComparisonComplement { Than |
   ThanOrEqualTo, standard }` for the other (`syntax/phrase.rs`) —
   which is the strongest kind of corroboration available here: two
   independent measurements of the same corpus drawing the same
   line. The comparative WORD is not a choice either. Every scalar
   characteristic takes "greater"/"less" and never "more"/"fewer",
   which is why the comparator rows carry their own words rather
   than deriving them from a direction.
61. **The characteristic presupposes a TYPE and, pointedly, not a
   zone.** Only a creature has power or toughness: a noncreature
   permanent has neither, and a noncreature object off the
   battlefield has them only if they are printed on it
   ([CR#208.3]). Mana value is defined for every object ([CR#202.3],
   with [CR#202.3a] giving even a costless one the value zero). The
   corpus says both in its own voice — no power or toughness bound
   sits on any head but a creature's, while mana value is bounded
   on cards, spells, permanents, artifacts, planeswalkers, and
   enchantments alike — so `seedType` gets its first row backed by a
   TABLE (`comparedType`) where the status words had a fixed
   answer, and "attacking noncreature"'s refusal extends to
   "noncreature with power 2 or less" (`badNoncreaturePower`) with
   no new gate. The zone half is where the parallel with `Attacking`
   STOPS, and it matters: attacking is a battlefield designation,
   but a creature card keeps the power printed on it wherever it
   lies ([CR#208.1] prints it on the CARD, and [CR#208.3] withholds
   it off the battlefield only from noncreature objects), and every
   object has a mana value wherever it stands, so the bound seeds
   no zone and "return target creature
   card with power 2 or less from your graveyard to the
   battlefield" stands. Reading a presupposition as a placement
   would have been finding 43's move made once too often. The
   characteristic enum is three rows against core's five for the
   same reason the comparator enum is two against five: "loyalty N
   or less" and "defense N or greater" ([CR#209.1,210.1]) are
   written zero times, so the short enum is a measurement, not a
   shortcut.
62. **A phrase bounds a characteristic at most once.** No corpus
   noun phrase carries two bounds; the lines that look like it are
   two phrases ("Creatures you control with power 2 or less can't
   be blocked by creatures with power 3 or greater"). The gate is
   therefore a multiplicity CAP, the one "other" and the class word
   already answer to, and not a range solver — which is what makes
   it both honest and cheap: the empty pair ("power 2 or less"
   beside "power 4 or greater") and the satisfiable pair are
   refused on the same evidence, that oracle writes neither
   (`badDoubleComparison`). An interval, when a line finally wants
   one, will be a construction with its own word rather than two
   qualifiers stacked. This is the fifth auto `And` now carries, so
   the masking question was asked of every one: each pin fails on
   its OWN gate, the new cap included, and the class word still
   refuses a bound at `AnyTargetLone` one slot earlier
   (`badAnyTargetComparison`).
63. **The negation is a comparator flip, and the rest was already
   standing.** Oracle never negates a bound — zero "non-", zero
   "doesn't have power", zero "without power 2" — and it does not
   have to, because the game's numbers are integers ([CR#107.1]):
   "not power 2 or less" IS "power 3 or greater", exactly, with no
   gap between them for a negation to name. So the row is
   unnegatable (`badNegatedComparison`), which is finding 42's De
   Morgan argument made exact by the discreteness rather than by
   comma-chaining. Everything else the bound met was already built:
   alternatives must presuppose ALIKE, so a power bound beside a
   mana value bound is refused by finding 51's gate with no rule of
   its own (`badMixedCharacteristicDisjunct`); `predEq` gained its
   three-part row so a repeated alternative is seen
   (`badRepeatedComparisonDisjunct`); and every determiner,
   quantity, and for-each domain carries a bound untouched — as do
   a disjunctive head ("Destroy target artifact or enchantment with
   mana value 3 or less") and an "other" with a real anchor, both
   audited.

## Chapter Seventeen — The Continuous Envelope

Chapter seventeen, the continuous envelope and the decomposed
duration (evidence: Jump, Giant Growth, Bond of Revival, Glyph of
Destruction, Gabriel Angelfire, Brazen Cannonade, Halfdane,
Infiltrate, Change of Heart; the granularity round's biggest seam,
and the one that moved every grant and restriction at once):

64. **A duration endpoint is a BOUNDARY of a PART, optionally
   POSSESSED — three axes, not a list of phrases.** The three
   words the workbench had were three constructors, and reading
   them as structure is what showed how little of the space they
   covered: "until end of turn" is the END boundary of the TURN,
   unpossessed; "until your next turn" is the START boundary of
   the same part, possessed; "until your next upkeep" is the same
   boundary of a different part. Decomposed, the phrases the
   corpus actually writes fall out of the axes instead of being
   enumerated — and so do the ones it doesn't, which is the point
   of finding 66. Two things stay OUT of the structure because
   they are the spelling layer's: "next" is derivable (a possessed
   endpoint is always the next one — there is no "your previous
   upkeep" for it to contrast with), and so is the article, which
   the bare form drops and the possessed form keeps ("until end of
   turn" against "until the end of your next turn"). Brazen
   Cannonade settles that they are surface and not structure by
   writing the possessive somewhere else entirely — "until end of
   combat on your next turn", a possessed combat endpoint spelled
   as a turn possessive. UPSTREAM: core cannot spell this space.
   `TurnMarker` (`deckmaste_core/src/temporal.rs`) has three flat
   rows — `EndOfTurn`, `EndOfCombat`, `YourNextTurn` — with no
   boundary axis on the possessed row and no possessor beyond
   "your", so "until the end of your next turn" and "until your
   next upkeep" have no spelling there at all. Its own doc says it
   "grows by card demand"; this decomposition is what it grows
   into.
65. **The adverbial is ONE slot, so the clause it modifies is one
   node.** `Gain`, `Gets`, and `Cant` each carried a duration
   argument and each carried its own span gate, which spelled one
   English fact — a trailing adverbial on a continuous clause — as
   three. Core had already made the cut: `Continuously { effect,
   duration }` (`deckmaste_core/src/effect.rs`) puts every lasting
   change behind one duration-bearing node, whether the change is
   a characteristic modification (`StaticEffect::Modify`) or a
   prohibition (`StaticEffect::Deontic`), and `Until(duration,
   [parts])` extends the same shape to a list. So the three
   constructors became three rows of a small `StaticEffect` — the
   stat delta ([CR#613.4c], layer 7c), the keyword grant, the deed
   restriction — under one `Continuously` envelope, and every
   demand re-keyed onto the payload rather than being restated:
   the battlefield subject rides its static row, the deed's grant
   table rides the restriction row, and the span gate reads the
   row's own name (`staticKind`). The macros own the surface
   exactly as they do elsewhere (`gets`, `gains`, `gainsHaste`,
   `cantAttack`, `cantBlock`, `cantBeBlocked`), so every positive
   respelled without a single clause changing meaning. What the
   envelope buys beyond tidiness is the next chapter's shape: core's
   remaining static rows — becoming a copy, losing abilities,
   setting base power and toughness, the conditional static — are
   rows to add here, and each will have to declare its adverbials
   before it can be written.
66. **Attestation closure belongs in a TABLE, and it has to tell
   two silences apart.** Decomposing multiplied the writable
   endpoints from three to thirty-one, and most of them are not
   English. Putting that in the constructors would have meant a
   constructor per phrase again, so the closure went where this
   workbench keeps closure — a full-rows table, `spanUse`, over
   (boundary x part x possession) plus the bare current-turn
   adverbial, so a new `TurnPart`, a new `Whose`, or a new
   boundary is a totality error that must be answered with
   evidence before anything can be written with it. The table's
   value type is where the round learned something: a Boolean
   would have collapsed two very different silences. `Unattested`
   means no corpus line writes the phrase at all — nothing ends a
   duration at an untap step in words this vocabulary has, nothing
   writes a bare "until upkeep". `Unclaimed` means the phrase is
   real oracle English and no construction HERE writes it: "until
   the end of your next turn" is eighty-three lines of play
   permissions and control grants, "until end of combat on your
   next turn" is Brazen Cannonade's play permission, "until the
   end of your next upkeep" is Halfdane's base-power-and-toughness
   setting, and the three end-step endpoints are play permissions
   and a copy effect. Every `Unclaimed` cell names a construction
   the grammar is missing, which makes the table a frontier map as
   well as a gate (`badGainsUntilEndOfYourNextTurn` against
   `badGainsUntilUntapStep` — the two silences, pinned separately).
67. **The construction's word, re-measured on the decomposed
   space, gets sharper rather than softer.** Finding 55's
   complementarity survived the reshape intact and gained two
   cells. Measured jointly — the adverbial has to belong to THAT
   clause, not merely share a line with it — "until end of turn"
   writes one thousand five hundred fifty-nine stat changes and
   one thousand three hundred ninety-one keyword grants and NOT
   ONE single-deed restriction, while "this turn" inverts it
   exactly at two hundred ninety-six restrictions and neither
   grant. [CR#514.2] is the rule that makes the split pure
   convention rather than semantics: cleanup ends "all 'until end
   of turn' and 'this turn' effects" together, in one turn-based
   action, so the two phrases name the same instant and only the
   construction chooses between them. The new cells: "until end of
   combat" ([CR#511.2] — it expires at the combat phase's end,
   not the turn's) takes the two GRANTS and no restriction (Glyph
   of Destruction's "+10/+0", one banding line), and "until your
   next upkeep" takes the KEYWORD GRANT ALONE — Gabriel Angelfire
   and one forestwalk line, with no stat change anywhere in the
   corpus taking it (`badGetsUntilYourNextUpkeep`). That last cell
   is why the two grants could not be one row of the answer table:
   they agree on three endpoints out of four and part on the
   fourth. `admitsSpan` carries the answers, full rows in both
   directions, and `absentOk` carries the fourth column — whether
   the construction may write NO adverbial at all, which the
   grants may ([CR#611.2a]'s end-of-game default, Through the
   Breach) and the restriction may not, that being the static
   ability line (finding 54).
68. **The possessor stayed a WORD, and the refusal is the
   finding.** A possessed endpoint could have taken a noun — core
   spells whose-turn relations as their own enum
   (`WhoseTurn::Your`/`EachPlayers`/`AnOpponents`,
   `deckmaste_core/src/event.rs`), but a grammar with binding
   already has richer player nouns lying around, and "its
   controller's next untap step" is a real corpus line that wants
   one. Taking it would have made `Duration` bindings-indexed —
   every consumer re-indexed, every span table carrying a context
   — to buy a form no construction here writes: that line's clause
   is "Target land becomes a Swamp", a base-TYPE setting, a layer
   word this grammar has no construction for. So the possessor is
   a closed two-word vocabulary, "your" and "that player's", which
   is everything the corpus possesses a duration endpoint with;
   the nominal possessor waits with the clause that needs it, and
   the anaphoric row's own obligation — "that player's"
   presupposes a unique player antecedent, `They`'s demand — waits
   with the first cell that opens for it, since no current
   construction writes one. Two axes deliberately left with no row
   at all, on the same reasoning: the draw step and the two main
   phases carry no duration-class adverbial anywhere in the
   corpus, so they are not parts this type names yet.

## Chapter Eighteen — The Conditions Axis

Chapter eighteen, the conditions axis (evidence: Overload, Built
to Smash, Flames of the Raze-Boar, Braids's Frightful Return,
Daretti Ingenious Iconoclast, Yawgmoth Demon; and, for what the
chapter refused, Blood Lust, Hidetsugu's Second Rite, Meddle,
There and Back Again, Cosmos Elixir. The round the corpus split
into thirds — one third built, one third handed to the cost
algebra by rule, one third measured and left):

69. **`Condition` is core's `Condition`, and its first three rows
   are the three questions English can ask with the vocabulary
   this grammar already has.** Core's enum
   (`deckmaste_core/src/condition.rs`) opens with
   `Exists(Predicate)`, `Matches(Reference, Predicate)`, and
   `Compare(Count, Cmp, Count)`, and those three are exactly what
   the workbench can spell today, because each takes one thing it
   already owns and asks whether it holds: a description, a
   reference against a description, an amount against a bound.
   Every other core row reaches past that — attachment,
   event history, cost tags, saga crossings, turn and phase — and
   each arrives with the axis it names, so the agreement is not
   imitation but a shared floor. The counts back the three: "if
   you control …" with an article runs to three hundred
   thirty-eight lines against seven for the existential-there,
   which is why `Exists` writes the fronted-subject spelling as
   its primary surface and the existential-there as its second;
   "if it's a/an …" runs to two hundred sixty-five, of which
   "creature" alone is a hundred sixteen; and the written numeric
   comparison ("is [n] or greater/less/more/fewer") runs to
   eighty-three. What did NOT become a row is the negation. Core
   has `Not(Condition)` and English writes "if you control no
   creatures" (twenty-four lines with "no", six of them
   creatures), but every carrier of those lines is a triggered
   ability's intervening-"if" or an activation restriction — R7
   both — so the row has no clause here to sit on and waits with
   them.
70. **A written comparison is ONE relation in two word orders, and
   the amount vocabulary divides by which SIDE it can stand on.**
   The temptation was a second comparison type: the postnominal
   qualifier "with power 4 or greater" (chapter sixteen) and the
   predicative "if its power is 4 or greater" look like different
   machinery. They are not — the comparator is the same two
   words, the characteristic read is the same `StatOf`, the bound
   is the same numeral-or-X — so `CompareAmt` reuses `Comparator`
   and `WrittenBound` unchanged and adds exactly one thing: a
   table saying which amounts can be MEASURED. `readAmount` turns
   out to be `writtenBound`'s exact complement. The two readers
   are the object's own number and a set's cardinality; the two
   writables are the numeral and the announced X; and the scaled
   product, the sum, and the event-outcome read are neither,
   appearing on no side of a written comparison anywhere. So one
   `Amount` vocabulary serves both halves of the frame with
   disjoint answers, and a new amount now has two tables to face
   rather than one. What the frame does NOT get is the strict
   comparators. The condition frame writes them where the
   postnominal frame cannot — "if your life total is less than 7"
   — and admitting them would turn `Comparator` from a closed
   two-row vocabulary into a per-frame attestation table, which
   is a chapter's worth of evidence and not a row. The counts say
   wait: fifty-one "is less than" lines and eighteen "is greater
   than", but almost all of them take a PHRASAL standard ("less
   than or equal to the number of Islands you control"), which is
   the frame chapter sixteen already ledgered, and the life total
   they mostly measure is not a `Characteristic` this grammar
   reads at all.
71. **A condition introduces nothing — and the one exception is a
   TARGET, which is named rather than smoothed over.** `condDelta`
   answers `[]` on every row, which is `predDelta (Or _) = []` one
   layer up and for the identical reason: a condition may be
   false, so a mention written inside one names nobody the
   sentences after it can read back. Corpus is not unanimous, and
   the counterexample is precise rather than vague. Ten lines
   write "if target …"; four of them lead their sentence with it,
   and three of those four read the announced target in the
   consequent — "If target creature has toughness 5 or greater,
   it gets +4/-4 until end of turn" (Blood Lust), Hidetsugu's
   Second Rite, Meddle. Those are not sloppy templating: targets
   are announced as the spell is cast whatever clause spells them
   ([CR#601.2c]), so the condition's truth never gated the
   announcement and the referent really is there. The workbench
   cannot represent a delta that carries only the target half of
   what a phrase introduces, so the family is refused WHOLE at the
   subject slot rather than admitted with a lie about its
   bindings: `Matches` demands a `Bindingless` subject, re-keyed
   onto `nounDelta` rather than mirroring the noun constructors,
   and Blood Lust is a pin (`badMatchesTargetSubject`). This is
   the chapter's one open user-level fork, recorded here so it is
   not decided by accretion.
72. **Argument order is textual order, so the TRAILING conditional
   fixes the node's shape — and buys a divergence worth stating.**
   Oracle writes both linearizations and they are not equally
   easy: six hundred ninety lines put the condition after the
   clause, four hundred eighty-six put it first. The trailing form
   is where the binding flow is forced — "Destroy target artifact
   if its mana value is 2 or less" (Overload) reads "its" off the
   artifact the MAIN clause targeted — so the condition is typed
   in the clause's post-context, like every other trailing
   argument in this grammar, and the leading form is the same node
   whenever the condition uses nothing the clause introduced,
   which is the ordinary case (Built to Smash writes
   "If it's an artifact creature, it gains trample until end of
   turn" and its condition reaches back only to the sentence
   before). The DIVERGENCE the shape buys: a trailing condition is
   evaluated before the clause it modifies takes effect but
   written after it, so the telescope types it against a discourse
   the clause has already updated — Overload's artifact is
   retagged to the graveyard by the destroy before "its mana
   value" is read. Nothing in this vocabulary notices, because
   mana value belongs to every object ([CR#202.3]) and is read
   zone-free; a zone-sensitive read in a trailing condition would,
   and that is the first thing to check when one lands. The ELSE
   sentence belongs to this node and is not built: "Otherwise, …"
   runs to a hundred and seventy-five lines and is plainly a third
   slot on `If` rather than a `Sequentially` element — an
   else-arm has no meaning without the "if" that governs it, where
   a sequence's elements are independent clauses — but every
   corpus else-arm read this pass needs vocabulary this grammar
   lacks (counters, draw, mana, tokens, the library), and the one
   that nearly fits is Blood Lust, refused by finding 71.
73. **"If you do" is not a condition. It is a BRANCH on the may,
   and its two arms have different types.** This was the round's
   hardest design and the corpus decided it twice over. The
   anaphor has no referent of its own to condition on: it asks
   whether the immediately preceding OPTIONAL ACTION was taken,
   which is not a fact about the board and not a mention in the
   discourse, so making it a `Condition` row would mean minting a
   channel that records what the last clause offered. Core reached
   the same place from the other direction — `May { who, effect,
   if_did, if_not }` (`deckmaste_core/src/effect.rs`), one node
   whether or not the branches are present — and the ledger had
   already filed these as May's branches. So `May` grew two
   `Maybe` slots and three macros own the surfaces (`may`,
   `mayThen`, `mayElse`), the anaphor's pronoun being the
   DECIDER's own ("if you do" against Risk Factor's "if they
   don't"). The finding is the ASYMMETRY between the arms. The
   taken arm runs only when the body ran, so it reads everything
   the body introduced and may announce more of its own
   (Daretti's "If you do, destroy target artifact or creature").
   The declined arm runs only when the body did NOT, so the body's
   phrase never named anything and the arm reads only what
   preceded the may — Yawgmoth Demon's "If you don't, tap this
   creature and it deals 2 damage to you", Braids's "If they
   don't, they lose 2 life" reaching the decider, Chandra's "If
   you don't, …" reaching the sentence before. "You may sacrifice
   a creature. If you don't, exile it." is unwritable
   (`badIfNotReadsMayBody`) and the corpus writes no such line.
   `mayIntro` carries the same cut outward: the arm that continues
   the main line flows out of the clause, the arm that REPLACES it
   is a hole. And the rules say what the branch reads, which is
   sharper than the corpus alone could: [CR#118.12] makes the
   offered action a COST paid on resolution and has the "if [a
   player] does" clause check "whether the player chose to pay an
   optional cost or started to pay a mandatory cost, regardless of
   what events actually occurred". The branch is therefore not an
   event read at all — no outcome channel would have served it —
   and the arm asymmetry follows directly: a payment never started
   introduced no phrase to mention. The MANDATORY twin ("[Do
   something]. If you do, …", no "may") is the same rule's first
   shape rather than a different reader, a hundred and forty-two of
   the thirteen hundred ten "if you do" lines writing no "may"
   anywhere on the line; it waits with the cost algebra that has to
   spell an unoffered payment.
74. **"Unless" divides by whether what follows it is an ACTION or a
   STATE, and the rules make the division rather than the
   grammar.** [CR#118.12a] rewrites the action form outright:
   "[Do something] unless [a player does something else]" means
   the same thing as "[A player may do something else]. If [that
   player doesn't], [do something]." That is finding 73's node
   exactly, which means the action-unless family needs no new
   structure at all — it is `May` with an `ifNot` arm, and what it
   waits on is the vocabulary in the may body. That vocabulary is
   cost: of six hundred ninety-six "unless" lines, a hundred
   forty-five write "unless you pay" and a hundred eighty-six
   "unless [someone] pays", and the one-shot family is
   overwhelmingly the counter-unless-pays punisher. Another
   seventy-one pay a non-mana cost ("unless you sacrifice /
   discard / exile / tap"), still [CR#118.12] payments. So the
   action half is R7's, ledgered with its counts and nothing
   built. The STATE half is not [CR#118.12a] at all — "unless you
   control a legendary creature" tests the board, so it is a
   negated `Condition` — and its sixty-two lines have no one-shot
   carrier either: they are entering-tapped replacements (R6) and
   durationless static deontics ("Bast can't attack or block
   unless you control three or more creatures"; a hundred nine
   "can't … unless" lines), which is the parked ability layer.
   When the conditioned deontic does land it should re-use this
   chapter's `Condition` inside `Cant`'s shape rather than grow a
   restriction-local test — core does exactly that with
   `StaticEffect::Conditionally(Condition, StaticEffect)`
   (`continuous.rs`).
75. **"For as long as" is a row waiting on its CLAUSES, not on its
   shape.** Core spells it `Duration::ForAsLongAs(Condition)`
   (`continuous.rs`), the condition type it needs now exists here,
   and the row would EXTEND chapter seventeen's decomposed
   `Duration` — a third alternative beside `ThisTurn` and `Until`
   — rather than parallel it. What is missing is a clause to hang
   it on, and the measurement is unusually clean. Two hundred and
   ten corpus lines write the adverbial. Forty-two are control
   grants, a verb this grammar has no row for. Four are keyword
   grants and all four grant "indestructible", which `Keyword`
   does not carry. Four are stat deltas and all four end "for as
   long as this artifact remains tapped", which needs the
   object-status axis the representability frontier already names.
   Nine are restrictions and eight of those coordinate two deeds
   or name a deed with no clause here ("doesn't untap", "can't
   phase in", "can't have counters put on it"). The single line
   that is one deed under one condition — "Up to one target
   creature can't block for as long as you control this Saga"
   (There and Back Again) — needs a type word `CardType` does not
   have. So nothing was opened: an unwitnessed row is a row
   without a positive, and the honest form of "real English, no
   construction here" is a ledger entry, which is the same
   `Unclaimed` distinction chapter seventeen's span table makes
   one level down. Its sibling stays apart from it and is
   larger: the bare "as long as" conditioning a static ability
   ([CR#611.3]) accounts for the rest of eleven hundred
   thirty-five "as long as" lines, and that is a static-ability
   container, not a duration.
76. **Nothing in this chapter is one-shot-only, and the seam is
   deliberate.** [CR#603.4]'s own parenthetical is what licenses
   the ordinary conditional here — "the word 'if' has only its
   normal English meaning anywhere else in the text of a card" —
   and it is also what marks the boundary: the intervening-"if" of
   a triggered ability is an "if" immediately following a trigger
   condition, and it is checked twice, at the trigger event
   ([CR#603.4]) and again on resolution ([CR#608.2a]). That is a
   CARRIER distinction and not a condition one. The three rows
   here ask about the board, the bindings they are typed in are
   whatever their carrier supplies, and `condDelta`'s opacity
   holds for any of them, so the trigger layer reuses `Condition`
   verbatim for its intervening-"if" clause and for the "Activate
   only if" restriction (two hundred thirty-seven lines) without
   re-deriving anything. The one-shot `If` node is the reader that
   arrived first, not the type's owner — which is why the type's
   doc comment says so and why none of the gates mention clauses.

## Chapter Nineteen — Creation and Counters

Chapter nineteen, objects made and markers moved (evidence: Raise
the Alarm, Additive Evolution, Aviation Pioneer, Fire Navy
Trebuchet, Battlegrowth, Chainbreaker, Kaito Bane of Nightmares,
Ashnod's Transmogrant, Neurok Transmuter, Coward // Killer,
Unholy Annex, and amass itself by way of [CR#701.47a]; and, for
what the chapter refused, Memnarch, Clavileño First of the
Blessed, Olivia Voldaren, Brood Birthing, Blood Lust. The round
where a fourth construction split a table three constructions had
agreed on):

77. **The five colors are a PORT, and they are the one vocabulary
   in this file that does not answer to a card.** Every other
   catalog here — `CardType`, `Deed`, `Keyword`, `TurnPart` —
   grew a row at a time behind a bench positive, and `Color` did
   not: it is `deckmaste_core/src/color.rs` row for row and in
   core's order, by explicit ruling, and the reason the ruling is
   right is that a color is a rules-fixed set ([CR#105.1] names
   all five in one sentence) rather than a construction English
   might or might not write. A missing row would be a hole in the
   rules, not an unattested phrase. Two things did NOT come with
   the port. Colorless is not a sixth row, because [CR#105.2c]
   makes it the absence of colors and core says the same with an
   empty `color_indicator` — so the token's color slot is a
   `List Color` and `[]` is exactly where English writes the word
   "colorless" (three hundred ninety-one corpus lines write it of
   a token). And core's sibling `ColorOrColorless` did not come
   across at all: its whole job is the mana-symbol channel — its
   `from_code` reads the symbol letter "C" — so it belongs to the
   symbols round the ruling deferred, and porting it here would
   have put a type in the file with nothing to spell. That second
   call is the chapter's one deliberate narrowing of an explicit
   instruction, recorded here so it can be reversed in a line.
   The name needed a namespace: the color `Color` and the quality
   sort `Color` ("Choose a color") are different words, which is
   the split `Verb` already makes for the tag `Exile` against the
   zone, and Idris resolves the two by the type each stands in.
78. **A subtype is a catalog atom like a card type, and the
   catalog PORT is the alternative that was measured and left.**
   Core declares subtypes as open plugin data — `Subtype { name,
   types, confers }` resolved through a generated registry of
   hundreds — and a bench cannot witness hundreds of rows, so the
   default held: eight rows, each with a card behind it (Zombie
   and Army from [CR#701.47a], Soldier from Raise the Alarm,
   Thopter from Aviation Pioneer, Construct from Fire Navy
   Trebuchet, Fractal from Additive Evolution, Coward from
   Coward // Killer, Demon from Unholy Annex), and all eight are
   on [CR#205.3m]'s creature-type list. The port alternative is
   ledgered rather than dismissed: it is the same open-registry
   shape core gives counter kinds and token names, and the day
   the workbench stops being a bench is the day it wants all
   three. What the eight rows buy immediately is a CLOSED TABLE:
   `subtypeType` says which card type's set each subtype comes
   from ([CR#205.1a] names the six sets), every row answers
   Creature today, and that is not a boring table but a forced
   one — an artifact type like Equipment or an enchantment type
   like Aura is a totality error until someone declares its
   answer. The table is what refuses a "Zombie artifact token"
   (`badZombieArtifactToken`): a subtype has to sit on a card
   type the same line names.
79. **The subtype word PRESUPPOSES its card type and PROJECTS
   none, and the asymmetry is load-bearing rather than a
   hedge.** The obvious reading is that "an Army" projects
   Creature onto its referent, and it was wrong for a reason the
   negation scan makes visible: `negTypesOf` reads a negated
   member's PROJECTED head, so a Creature projection on
   `HasSubtype` would make "that isn't a Demon" mean
   "noncreature" and refuse a phrase oracle writes plainly —
   Clavileño, First of the Blessed's "target attacking Vampire
   that isn't a Demon" (`clavilenoPhrase`, transcribed at a
   widened head). So `seedTy` stays silent and `seedType`
   carries the answer, which is exactly the split chapter
   thirteen drew for the status words: "attacking" presupposes a
   creature without being one. The presupposition still bites
   where it should — "an Army that isn't a creature" is refused
   by the coherence gate that was already there
   (`badZombieNoncreature`) — and nothing new was needed to make
   it.
80. **"Create" is the ONLY surface, measured rather than
   assumed.** The old wording is worth a sentence because the
   temptation was to carry both: "put a … token onto the
   battlefield" was the phrase for a decade and a half, and it
   survives on ZERO current oracle lines. Three thousand five
   hundred twenty-eight lines write "create … token"; the two
   lines that match the old pattern are a "nontoken permanent"
   restriction and a Lander token's own quoted land-search
   ability, neither of them a token creation. So `Create` is one
   constructor and the ledger gets no second spelling.
81. **The token's characteristics are a BUNDLE in a fixed
   adjective order, and one card fixes the order nothing shorter
   could.** Power and toughness, colors, subtypes, card types,
   the noun "token", the with-clause, the name, the arrival
   clause. Most cards write three or four of the slots; Fire Navy
   Trebuchet writes six in one phrase — "a 2/1 colorless
   Construct artifact creature token with flying named Ballistic
   Boulder that's tapped and attacking" — which is what settles
   that the with-clause precedes the name rather than the other
   way round, and that the arrival clause follows both. The
   record is `TokenChars`, core's `Token` field for field minus
   what no line here needs (supertypes, forty-six "legendary …
   token" lines, ledgered; abilities beyond bare keywords). Two
   demands earn their place beyond the subtype table: the line
   names at least one card type, because a token IS a permanent
   ([CR#111.1]) and the type-less spelling is the PREDEFINED name
   ("create a Treasure token", [CR#111.10], six hundred
   thirty-seven lines) that core gives a separate `TokenSpec` row
   and this file ledgers; and a creature token writes a power and
   toughness, because a token has only the characteristics its
   creating effect defines ([CR#111.3]) and no card prints them
   for it. That second demand is ONE-DIRECTIONAL on purpose — a
   Vehicle token carries a P/T with no creature type ([CR#301.7])
   — so the noncreature slot is left free rather than forced
   empty.
82. **A created token is a DISCOURSE MENTION, which is what makes
   creation a binding construction rather than a verb.** Additive
   Evolution reads it in the next breath: "create a 0/0 green and
   blue Fractal creature token. Put three +1/+1 counters on it."
   So `Create` prepends a binding — indefinite determiner, object
   kind, battlefield zone ([CR#111.1] puts tokens there), the
   head its type line writes — and the number of that mention
   reads off the COUNT, which is the ordinary amount vocabulary
   and no parallel number path (`amtPlur`, `quantPlur`'s twin one
   vocabulary over). "Create two 1/1 white Soldier creature
   tokens" is a `Lit`, "create a … token for each Elf you
   control" is the for-each amount, and core's own slot is its
   `Count` likewise. The head projection needed a rule of its own
   and got the honest one: a type LINE is written in a fixed
   order where a modifier list is not, so the head is the LAST
   type word ("artifact creature token" heads on "creature", and
   [CR#205.1b]'s own example spells the compound the same way
   round, "artifact land creatures") rather than `seedTyAll`'s
   first member. No bench read tests it — no corpus line yet
   found rements a compound-typed token by a type word — so it is
   the chapter's thinnest projection and is flagged as such.
83. **The arrival riders are an ENUMERATION, not a product, and
   the rules say why.** Sixty-six lines create a token "tapped
   and attacking"; a hundred fifty-nine write the prenominal "a
   tapped … token"; ATTACKING WITHOUT TAPPED is written zero
   times. That is not an accident of style: [CR#508.4] gives the
   attacking designation to a creature put onto the battlefield
   attacking and taps nothing, because the tap belongs to the
   declare-attackers turn-based action ([CR#508.1f]) that such a
   creature never went through — so a writer who wants both has
   to say both, and every writer does. `ridersOk` lists the three
   attested shapes rather than checking a pair of flags
   (`badAttackingUntapped`), which is the same
   whole-attested-surface discipline `ChoiceMode` and the deontic
   macros take. The enters-tapped REPLACEMENT ("This land enters
   tapped", a hundred thirty-two lines) is a different
   construction and the replacement axis's; these ride the create
   instruction, where core files them too.
84. **Counters are TWO verbs, agent-silent, and object-only —
   and each of those three is the corpus's answer rather than
   core's.** Put and remove are separate rows in core and here,
   and not because removal is a negative put: removal is
   cost-eligible where a put is not, and it can fail for want of
   counters. Agent-silent because no corpus line writes "[player]
   puts a +1/+1 counter on" — the verb is the effect's own
   imperative, and core's `PutCounters` is on its
   agent-carrying-none list for the same reason. Object-only
   because although [CR#122.1] places counters on players as
   readily as on objects, ENGLISH does not put them there with
   this verb: "put a … counter on [a player]" is written zero
   times and the player form takes a different verb entirely
   ("target player gets a poison counter", forty-eight lines), so
   the player-borne counters wait on that verb rather than on a
   kind index. The battlefield demand is the one destroy, tap,
   and "gets" already carry, and it is exact: no corpus line puts
   a counter on a card in a graveyard or in exile with this
   clause (`badPutCountersGraveyard`, `badRemoveCountersDead`).
85. **A counter KIND earns a row where a one-shot line writes
   it, and the two stat counters lead by an order of
   magnitude.** "Put a +1/+1 counter on" runs to one thousand
   four hundred ninety-three lines and "put N +1/+1 counters on"
   to four hundred thirty-one, against eighty-eight and twenty
   for -1/-1; each carries its own rule ([CR#122.1a]), so both
   are first-class. Of the named kinds the corpus is thick with —
   charge sixty-six, stun fifty-six, age thirty-seven, quest
   twenty-seven, loyalty twenty, oil sixteen, level sixteen —
   only STUN has a line that is a one-shot put of the sort this
   grammar writes: Kaito, Bane of Nightmares' "Tap target
   creature. Put two stun counters on it." The rest appear as
   activation costs, upkeep triggers, and enters-with riders,
   which are three other axes, so they wait there. Core's shape
   is the open registry again (`CounterRef` into a plugin
   catalog), and the same answer serves: witnessed rows now, the
   registry ledgered with the subtypes and the token names.
86. **The type addition is ONE construction over two of core's
   ops, and [CR#205.1b] names the phrase outright.** That rule is
   the whole warrant for treating "in addition to its other
   types" as its own thing rather than a flavour of type-setting:
   it says the phrase in those words and states that the object
   retains ALL its prior types, where [CR#205.1a]'s bare setting
   replaces them. Core spells the change as two layer-4
   collection ops, `Modification::CardTypes(Add …)` and
   `Subtypes(Add …)` ([CR#613.1d]); English writes them together
   in one type line ("becomes a Spirit artifact creature in
   addition to its other types"), so `TypeLine` is those two
   lists under one name and it is the SAME record the token
   bundle carries — the two clauses differ in what surrounds the
   line, not in the line. The subtype check comes back with one
   more place to look than the token's: an added subtype may sit
   on a card type the same clause adds, or on one the SUBJECT
   already has, which is exactly what amass needs ("it becomes a
   Zombie" said of an Army creature) and exactly what refuses
   "target land becomes a Zombie" (`badBecomesZombieLand`). A
   subject that projects no head type answers for nothing, the
   honest reading of silence finding 50 established.
87. **A FOURTH static construction split a span class that three
   constructions had agreed on.** This is the chapter's structural
   finding and it is what the closed full-row tables are for.
   Chapter seventeen classified "until end of turn" and "until end
   of combat" into one class, `BothGrants`, because the stat delta
   and the keyword grant both wrote both and no restriction wrote
   either — three constructions could not tell the two endpoints
   apart. The type addition can: eighteen corpus lines end the
   phrase itself at "until end of turn" and NOT ONE ends one at
   end of combat. So the class split — `GrantsAndTypes` for the
   turn endpoint, `BothGrants` keeping its name, its meaning, and
   the combat endpoint alone — and the lesson is that a
   `SpanUse` row is a partition of the corpus by construction,
   which a new construction may refine and never silently join.
   The current-turn division came with a card that writes both
   halves in one sentence: Coward // Killer's "Target creature
   can't block this turn and becomes a Coward in addition to its
   other types until end of turn" gives the restriction "this
   turn" and the type addition "until end of turn" in one breath,
   which is chapter seventeen's count-derived split stated by a
   writer instead. `absentOk TypeAddition` is `True` and this is
   the row where the unwritten span is the NORM: most of the two
   hundred twenty "in addition" lines state no duration at all,
   and Memnarch prints the reason in reminder text — "(This
   effect lasts indefinitely.)", which is [CR#611.2a]'s
   end-of-game default said out loud on a card. One cell is
   thin and is flagged rather than hidden: the cross-turn span
   (then `EveryStatic`) is opened on a SINGLE line, "Until your
   next turn, target artifact you control becomes a 5/5 creature
   in addition to its other types", whose clause fuses a base-P/T
   setting this vocabulary has no word for onto the type
   addition. Answering it `False` would have made the name
   `EveryStatic` a lie, and the line really does write the
   adverbial over a type addition, so the cell is open and marked
   for re-measurement. (Re-measured and CLOSED in chapter
   twenty-one, finding 104: three co-occurrences in two hundred
   sixty-four lines, each otherwise explained, so the cell is
   `False` and the class is `GrantsAndRestrictions` — the name
   went rather than the honesty.)
88. **The "Otherwise" arm is `May`'s declined arm one
   construction over — and it opened on the card, not on the
   design.** Chapter eighteen wrote the design and could not fill
   it: the else arm is a third slot on `If` rather than a
   `Sequentially` element, because an else arm has no meaning
   without the "if" that governs it where a sequence's elements
   are independent clauses. What it waited on was a card whose
   BOTH arms this grammar could write, and the counters and
   tokens brought one: Unholy Annex's "If you control a Demon,
   each opponent loses 2 life and you gain 2 life. Otherwise, you
   lose 2 life." The typing is finding 73's, unchanged and
   unargued: the arm runs when the condition was FALSE, so the
   main clause never happened and its phrase never named
   anything, which makes the arm typed in the discourse BEFORE
   the conditional and unable to read what the clause introduced
   (`badOtherwiseReadsIfArm`); and it contributes nothing outward,
   the arm that REPLACES the main line being a hole exactly as
   `mayIntro` has it. One linearization side condition comes with
   it and is recorded rather than checked: "Otherwise" needs its
   "if" in front of it, so only the LEADING order spells an else
   arm. The measurement behind the wait was worth doing over:
   of the hundred seventy-five "Otherwise" lines, the great bulk
   are library reveals ("If it's a creature card, put it into
   your hand. Otherwise, …"), static "as long as" Auras, event
   histories, and turn conditions; Blood Lust is still refused by
   finding 71; and the briefing premise that a draw verb already
   existed was false — there is no `Draw` in this file, and the
   two else-arm cards that would otherwise have fit (Tribute to
   the World Tree, Primal Empathy) put the draw in the arm the
   grammar would have to write.
89. **Amass's legs land, and the authority is the CR's own words
   rather than a card's reminder text.** [CR#701.47a] defines
   "amass [subtype] N" in a single quoted sentence — create a 0/0
   black [subtype] Army creature token if you control no Army,
   choose an Army creature you control, put N +1/+1 counters on
   it, and if it isn't a [subtype] it becomes one in addition to
   its other types — and that matters here beyond convenience:
   the phrase this chapter builds against appears on cards ONLY
   as reminder text, and finding 20's ruling refuses reminder
   text as the operative spelling. The rule is not reminder text.
   All three legs are written (then `amassZombiesToken` and
   `amassZombiesArmy`; folded into `amassZombiesTwo` by chapter
   twenty-one), and only the two negated conditions are
   elided, both already ledgered. They are written as TWO terms
   rather than one for a reason worth stating: the rule's branch
   makes the created token and the chosen Army the same object,
   and this grammar has no way to say so, so a single term would
   have put two mentions where the rule has one. (That premise is
   FALSE and chapter twenty-one says why: Doubling Season can make
   two Armies, [CR#614.1a], so the created token and the chosen
   Army need not be the same object at all — and the one term
   lands once the created token stops escaping its branch.) R3 adds the
   modal frame and the branch; the legs are here.

## Chapter Twenty — The Composite Glue

Chapter twenty, the round whose exit test was a keyword-action
composite written end to end (evidence: Divination, Ancestral
Recall, Prosperity, Collective Unconscious, Killian's Confidence,
Blindblast, Abrade, Austere Command, Azula Always Lies, Rain of
Thorns, Myrkul's Edict, Unholy Annex, cycling by way of
[CR#702.29a], and amass by way of [CR#701.47a]; and, for what the
round measured rather than built, Blood on the Snow, Thermal
Flux, Kogla and Yidaro, Blizzard Specter, Judith Carnage
Connoisseur, Kitsune Ace, Gylwain Casting Director, Pip-Boy 3000,
Fblthp the Lost. The round that got three of amass's four
sentences into one term and can name the anaphor that blocks the
fourth):

90. **The draw verb is its own clause row and not a keyword
   action, and it introduces nothing.** [CR#701] is the keyword
   actions and drawing is not among them — [CR#121.1] gives it a
   rule of its own — so the clause sits beside `ChangeLife`
   rather than under a `Composite` tag, and it carries its
   SUBJECT for `ChangeLife`'s reason: both spellings are ordinary
   oracle, the imperative with its unpronounced `You` (a thousand
   nine hundred sixty-three lines write "draw a card") and the
   subjected form ("Target player draws a card", twenty; "Each
   player draws a card", twenty-nine). The count is the ordinary
   magnitude vocabulary and no parallel number path: `Lit`
   ("Draw two cards", two hundred seventy-four), `XVal` ("Draw X
   cards", seventy-six), the for-each amount ("Draw a card for
   each …", a hundred thirty-seven), and a read ("Draw cards equal
   to the sacrificed creature's power", eighty-two). What the row
   contributes to the discourse is MEASURED and it is nothing but
   its subject: no corpus line reads a drawn card back across a
   sentence boundary — "Draw a card." followed by "it" or "that
   card" is written zero times, and the three near misses are two
   reminder parentheticals and Fblthp, whose "if it entered from
   your library" reads the creature that entered rather than the
   card drawn. The one place the drawn card IS read is inside the
   coordination that reveals it ("Draw a card and reveal it. If it
   isn't a land card, discard it.", four lines), a verb-phrase
   coordination this grammar does not spell, and that construction
   is what would reopen the question. No OUTCOME binding either:
   "that many cards" reads a count from somewhere else, never from
   a draw.
91. **[CR#700.2] supplies both halves of the modal node's shape,
   and the headcount table is the corpus's, not the brief's.** The
   rule defines a modal spell as one with "two or more options in
   a bulleted list preceded by instructions for a player to choose
   a number of those options" — so the mode minimum is the rule's
   own and not a presumption (`AtLeastTwo`, reused unchanged from
   the sequence chapter), and the headcount is a NUMBER, which
   means it is the one quantity vocabulary. Measured by card:
   "Choose one —" four hundred seventy, "Choose two —" thirty-two,
   "Choose three —" one (Mishra, Eminent One, over six modes),
   "Choose one or both —" fifty-two, "Choose one or more —"
   nineteen, "Choose up to one —" seven. The briefing premise that
   "up to two" and "up to three" were attested was false: both are
   written zero times in every scope, as are "choose four", "two
   or more", and "one or two". The mode COUNTS are as informative:
   "one or both" appears over exactly two modes every one of its
   fifty-two times, which is what the word "both" says, and "one
   or more" never appears over two — at two the word is "both".
92. **The mode list is a LIST and not a telescope, and that is
   this chapter's structural finding.** Every mode is typed in the
   discourse the modal itself stands in, so a mode reads
   everything before the modal and nothing a sibling introduced,
   and the modal contributes nothing outward. All three directions
   were measured before they were encoded. INWARD: of the six
   bullets in the corpus that open with a pronoun, every one
   reaches past the modal to the sentence before it — Kogla and
   Yidaro's "When Kogla and Yidaro enters, choose one — • It gains
   trample and haste until end of turn. • It fights target
   creature you don't control." has BOTH modes reading the same
   outside antecedent, and Blizzard Specter's "That player",
   Judith's "That spell", Kitsune Ace's "That Vehicle", Gylwain's
   and Pip-Boy 3000's "that creature" are the same shape.
   ACROSS: zero bullets read a sibling. OUTWARD: of the forty-eight
   modal cards with a non-bullet line after the list, all but two
   are keyword packaging (entwine, equip, cycling, flashback,
   rebound, crew, suspend, reinforce); Thermal Flux's trailing
   line names nothing; and Blood on the Snow is the positive
   proof — "Choose one — • Destroy all creatures. • Destroy all
   planeswalkers. Then return a creature or planeswalker card …
   from your graveyard to the battlefield" writes a DESCRIPTION
   covering both modes' outcomes exactly where an anaphor would
   have gone. The rule says why: [CR#700.2a] chooses the modes at
   cast, and [CR#700.2c] has an unchosen mode's targets never
   announced at all, the spell being "treated as though it did not
   have those targets". So this is `predDelta (Or _) = []` and
   `condDelta = []` one construction up, and for their reason
   (`badModalReadsAcrossModes`, `badReadsAfterModal`).
   Azula Always Lies is what the list buys: "Choose one or both —
   • Target creature gets -1/-1 until end of turn. • Put a +1/+1
   counter on target creature." writes "target creature" in BOTH
   modes and neither is the other's "other", because each
   announces its own ([CR#601.2c] lets the same object be chosen
   once per instance of the word). Written as a sequence the
   second phrase would have had to say "another".
93. **The modal headcount is what finally needed core's `AtLeast`
   and `Between`.** The quantity macros have carried a note since
   chapter one that those two spell over the same `Range`
   primitive and wait on a corpus line that needs them; "Choose
   one or more —" is `atLeast 1` and "Choose one or both —" is the
   one-to-two `Between`, nineteen and fifty-two cards. What still
   waits is the TARGET-position spelling of either ("one or two
   target creatures"), which no bench card writes. Which WORDS a
   headcount spells depends on the mode count as well as the
   range — a top that equals the list writes "or both" at two and
   "or more" above — and that is a linearization side condition,
   recorded and unchecked exactly as the leading-versus-trailing
   conditional is.
94. **`ModesFit` is a relation on the PAIR, and its two demands
   are impossibility and choice.** A headcount cannot reach past
   the modes offered — three of two options names nothing
   (`badModalOverreach`) — and a headcount that fixes the whole
   list instructs no choice at all, [CR#700.2] calling a spell
   modal for the INSTRUCTIONS to choose a number: "Choose two —"
   over exactly two modes has one answer, and zero cards write it
   (`badModalFixedWhole`). Every printed exact headcount is
   strictly under its list; the two forms whose top reaches the
   list are ranges, which have a choice to make inside them. The
   relation lives beside `WellFormedQ` rather than inside it
   because neither demand is about the quantity alone.
95. **The condition negation lands, and the ledger's own reason
   for holding it was the thing that had gone stale.** Chapter
   eighteen deferred core's `Not(Condition)` on the count of
   twenty-four "if you control no …" lines whose every carrier was
   a trigger's intervening-"if" or an activation restriction. The
   count was short: a hundred and twelve one-shot lines write it
   across the two frames it reaches — "if you don't control a/an
   …" fifty-nine, "if you control no …" twenty-four, "if there are
   no …" seventeen, "if no [creature/player/opponent] …" twelve on
   the existential side; "if it isn't …" twenty-four and "if it's
   not a …" thirty-seven on the reference side — and [CR#701.47a]
   writes BOTH of amass's branches with it. WHICH frames take a
   "not" is a closed table and not the row's license: the
   existential and the reference frames do, the COMPARISON frame
   does not, because a bound has a negative of its own and English
   writes that instead ("if its power isn't 4 or greater", zero
   lines), and a negation under a negation is written zero times
   likewise (`badNegatedComparison`,
   `badDoubleNegatedCondition`). The constructor is named
   `NotCond` because `Predicate` already owns `Not` in this
   namespace — `CompareAmt` against `Compare` made the same move
   first.
96. **The choose-then-refer glue needed no new machinery, which is
   the finding.** `Choose` already introduced what it chose
   (`effIntro (Choose n) = nomIntro n`), so the pattern the amass
   macro is built on — a choice clause whose referent the
   following sentences read — was writable the moment the reads it
   feeds existed. Myrkul's Edict is the demonstration on a card:
   "Choose an opponent. That player sacrifices a creature of their
   choice." has one choice clause serving two reads, the next
   sentence's demonstrative subject and the unique player
   antecedent "of their choice" demands (`countChoosers`). What
   this chapter had to add for amass was on the OTHER side of the
   glue: the negated condition finding 95 opened, and nothing
   else.
97. **THE AMASS VERDICT: pass with a gap, and the gap is one
   anaphor wide.** Amass Zombies 2 is now three sentences in one
   term and one sentence beside it, where chapter nineteen had two
   terms with both negated conditions elided. `amassZombiesToken`
   writes [CR#701.47a]'s first sentence WHOLE — "If you don't
   control an Army creature, create a 0/0 black Zombie Army
   creature token." — and `amassZombiesArmy` writes the last three
   whole: "Choose an Army creature you control. Put two +1/+1
   counters on that creature. If it isn't a Zombie, it becomes a
   Zombie in addition to its other types." Nothing is elided in
   either. What refuses is joining them, and the failing sentence
   is exactly identifiable: "Put N +1/+1 counters on that
   creature." With the conditional creation in front of it the
   discourse holds TWO creature mentions — the token that sentence
   may have made, and the Army this one chose — so the sorted
   demonstrative has two candidates and the no-recency discipline
   (finding 26) refuses it rather than guess
   (then `badAmassOneTerm`, a pin so the seam is bench-visible;
   converted to the positive `amassZombiesTwo` by chapter
   twenty-one, finding 102). The
   diagnosis chapter nineteen guessed at is confirmed and can now
   be stated exactly: the rule's two mentions denote ONE object in
   either branch — when the token exists it is the only Army
   creature there is to choose, and when it does not exist there
   is only the chosen one — so the count that fails is a count of
   MENTIONS where the rule is speaking about objects. Two designs
   would close it and neither is a row: a HEDGED-mention channel,
   marking a binding introduced under a condition and having the
   definite reads prefer an unhedged candidate when one exists
   (which [CR#701.47a] itself is the evidence for, and which the
   corpus does not contradict — every conditional mention it reads
   back, Lieutenant's Thopter and the Eldrazi Spawn among them, is
   the only candidate there is); or an identity channel saying two
   mentions co-refer. Both are context-data questions the §3
   ruling has to answer first, and both are ledgered rather than
   forced. (Neither was needed. Chapter twenty-one closed the gap
   from the other end — the conditional arm stops exporting at all
   — and [CR#701.47c] says the referent was the CHOSEN creature
   the whole time, so the identity channel would have encoded a
   falsehood.)
98. **"The chosen [noun]" is witnessed and stays shut, and the
   reason is the bench rather than the corpus.** The surface is
   real and plentiful — "the chosen creature" thirty lines, "the
   chosen player" twenty-nine, "the chosen card" twenty-six, "the
   chosen permanent" ten, "the chosen creatures" eight — and it is
   `TheVerbed`'s frame exactly, a definite participle read that
   real text switches to where a bare demonstrative would be
   ambiguous (Stolen Uniform's "Attach it to the chosen creature", with
   two mentions in scope, is finding 26's shape on the nose). It
   is not opened because no card that writes the SINGULAR read is
   writable here: every candidate needs vocabulary this file does
   not have — the plural definite, mill, kicker, teamwork, copies,
   an X counted off artifacts — and the house rule is that a row
   without a bench positive from a real card does not open. Two
   further halves of the family were already ledgered and stay
   there: the plural twin, and the PLAYER read, which needs
   provenance on the binding rather than on the object payload
   (finding 27's frontier — `Stamp` lives inside `ObjectP`, so no
   participle reaches a player). Whoever opens it should note that
   the provenance vocabulary would have to widen: `VerbName` is
   the [CR#701] keyword-action tags today, and "choose" is not one
   of them.
99. **Four retro-opens, claimed.** Chapter fifteen's Blindblast
   elided "Draw a card." for want of the verb and now writes the
   whole card (`blindblastWhole`); chapter eighteen's Unholy Annex
   elided the same sentence and was thereby a bare conditional
   where the card writes a sequence (`unholyAnnex`); cycling's
   expansion had its effect elided on the same ground and now
   writes it, the mana half of the cost staying elided as every
   mana cost does (`cycling`, [CR#702.29a]); and the quantity
   macros' standing note that core's `AtLeast` and `Between` await
   a corpus line is discharged by finding 93. What the round did
   NOT reopen is the draw CLUSTER the ledger parks beside the
   verb — the library zone and its ordered positions, miracle's
   draw-ordinal memory — none of which the verb alone touches.

## Chapter Twenty-One — The Branch Arm

Chapter twenty-one, a FIX round rather than a build round: the
fifth duck wave's thirteen findings and three directed questions,
each re-verified against the machinery and against the cards
before anything was touched (evidence: Crovax the Cursed, Through
the Breach, Akoum Stonewaker, Baral and Kari Zev, Overload,
Doubling Season, Rankle Master of Pranks, Aviation Pioneer,
Additive Evolution, Tezzeret Cruel Machinist, Rootwise Survivor,
Absorbing Man, Angrath Captain of Chaos, Abrade, Divination, Raise
the Alarm, Battlegrowth, Dryad Arbor, Flanking Licid, and amass by
way of [CR#701.47a,701.47c]; and, for what the round measured
rather than built, Elephant Resurgence, Ajani Adversary of
Tyrants, Blizzard Specter, Kogla and Yidaro, Blood on the Snow,
Thermal Flux, Rankle and Torbran. The round that got amass's four
sentences into ONE term). All thirteen machinery claims
reproduced; two of the wave's card attributions did not, and are
recorded as refused below.

100. **A branch arm is a HOLE, and the conditioned clause is one
   of them.** The workbench had this right for the two arms that
   REPLACE a main line — "if you don't" and "Otherwise" both
   contribute nothing, because the arm runs exactly when the main
   line did not — and wrong for the main line of a conditional
   itself: `If` exported everything its clause introduced, so
   "create a token if you control a creature" left an
   unconditional "it" behind for every sentence after it. The
   reason the replacing arms are holes is the reason this clause
   is one, read the other way round: the condition may have been
   FALSE, and then the clause never ran and its phrase named
   nobody. What survives a conditional is now the discourse that
   entered it (`badConditionalArmAntecedent`). And the two-armed
   `May` joins at the BODY rather than at either arm: [CR#118.12]
   makes the branch a record of whether the player chose to pay
   "regardless of what events actually occurred", so exactly one
   arm ran and nothing after the may can know which — selecting
   the if-you-do arm's mentions there was reading one branch as
   though it were both (`badBothArmsAntecedent`). Crovax the
   Cursed is the card that writes the pair, and it reads neither
   arm afterward. What did NOT change is the may's own body,
   because Through the Breach reads it in the very next sentence
   ("You may put a creature card from your hand onto the
   battlefield. That creature gains haste.") — a declined may
   skips at runtime, not in scope, which is the ruling `mayIntro`
   has carried since it was minted and which the wave's blanket
   arm-local proposal would have broken.
101. **A trailing condition is read in the clause's
   ANNOUNCEMENT, not in its result.** Chapter eighteen wrote the
   divergence into `If`'s own comment — the condition is evaluated
   before the clause it modifies and WRITTEN after it, so the
   telescope typed it against a discourse the clause had already
   updated — and named the thing to watch for: a zone-sensitive
   read in a trailing condition. It landed. "Destroy target
   creature if it's in a graveyard" typechecked precisely because
   the destroy had already retagged its own target. The fix is the
   timing rather than the textual order: `preIntro` is what a
   clause has ANNOUNCED by the time its condition is read — its
   phrases' mentions with their announced zones, since [CR#601.2c]
   announces targets whatever clause spells them — and NOT what
   the clause did, so the three zone-writing rows leave their
   object unmoved and unstamped, `Create` leaves no token
   ([CR#111.1] — an effect is what puts one onto the
   battlefield), and the damage and life
   rows leave no outcome. The other half is a gate: a copular
   condition says something about a referent placed somewhere
   else already, so its description's zone must agree with the
   subject's ([CR#109.2a] — a description naming a zone means an
   object in that zone; `ZoneFits`,
   `badTrailingPostStateZone`). Overload is untouched — mana
   value belongs to every object [CR#202.3] and is read zone-free
   — which is what makes this a correction to WHEN the condition
   is read and not to where it is written.
102. **Amass is one term.** The payoff of finding 100, and the
   gap chapters nineteen and twenty could only write in halves.
   [CR#701.47a] defines "amass [subtype] N" as four sentences:
   the conditional token creation, the choice, the counters on
   "that creature", and the conditional type addition. With the
   conditional arm a hole, the token the first sentence may have
   made is not in scope when the third sentence writes its
   demonstrative, so "that creature" has exactly one candidate —
   the Army the second sentence CHOSE — and the whole definition
   typechecks (`amassZombiesTwo`; the two half-cards it replaces
   are gone, their split having existed only for this gap). The
   rule agrees twice. [CR#701.47c] says "the Army you amassed"
   means "the creature you chose", so the choice is the binder and
   the creation is not. And the channel the wave's alternative
   would have needed — merging the created token with the chosen
   Army into one referent — is not merely unspellable but FALSE:
   Doubling Season reads "If an effect would create one or more
   tokens under your control, it creates twice that many of those
   tokens instead", which [CR#614.1a] makes a replacement effect,
   so amass can create TWO Armies and then choose either. The
   grammar's refusal to identify them is the rules' refusal.
103. **A choice clause SELECTS, or it is not a choice.** The
   choose glue had no gate on its noun at all, so "Choose you"
   typechecked and was treated as a fresh introduction. A choice
   binds a new referent out of a described set, and the corpus
   writes exactly that: "choose a/an …" four hundred ninety-one
   lines, "choose target …" a hundred sixty, plus "choose two",
   "choose up to", "choose any number of", "choose another" — and
   "choose you", "choose it", "choose them" zero times each. The
   twenty-five "choose the …" lines are all descriptive nouns
   ("the name of a nonland card", "the value of X", "the same
   mode"), not definite participants. So the two INTRODUCING
   determiners are the whole vocabulary (`choosable`,
   `badChooseYou`), which is chapter twenty's introduction
   discipline asked of the clause instead of the article. The gate
   is about the determiner and not the kind: "Choose an opponent"
   is untouched (`myrkulsEdict`).
104. **A type addition must ADD, and it does not cross turns.**
   Two corrections to the same row. First: "in addition to its
   other types" retains what the object had and states what it
   gains ([CR#205.1b]), so a clause stating only what its subject
   already is states nothing — "target creature becomes a creature
   in addition to its other types" instructed nothing and
   typechecked (`badBecomesOwnType`). Tezzeret's adds creature to
   an ARTIFACT and Neurok Transmuter's adds artifact to a
   CREATURE; the gate under-refuses in `predEq`'s direction, a
   subtype never being provably redundant here and [CR#701.47a]
   guarding that case in the text instead ("If it isn't a
   [subtype], …"). Second: chapter nineteen opened the cross-turn
   span cell on ONE apparent witness and flagged it for
   re-measurement. The re-measurement closes it. Two hundred
   sixty-four supported lines write "in addition to its/their/
   his/her other types"; exactly three carry "until your next
   turn", and every one is otherwise explained — Rootwise
   Survivor's duration belongs to the separate haste grant in the
   next sentence, Absorbing Man's clause is a copy construction,
   and Tezzeret, Cruel Machinist's "becomes a 5/5 creature in
   addition to its other types" fuses a base-P/T setting onto the
   addition. No line writes a NAKED type addition across turns
   (`badTypeAdditionAcrossTurns`), and the class had to be renamed
   with the cell: `EveryStatic` had stopped being every static, so
   it is `GrantsAndRestrictions` now, which is what it is.
   Tezzeret goes to the ledger as the compound construction's
   witness.
105. **A condition negates a POSITIVE description, and tests one
   that says something.** Two laundering routes through the
   copular frame. `condNegatable` answered `True` for the
   reference frame whatever its predicate, so "if it isn't a
   non-artifact" typechecked — a negation of a negation, which the
   predicate layer already refuses of itself (`negatable (Not _) =
   False`) and which the condition frame could take of an
   already-negative phrase from outside. The polarity is now read
   through the combinators, where an inner negation can hide
   inside a conjunction the predicate layer's own row never sees
   (`predNegFree`, `badNegatedNegativeMatch`). And the frame
   demanded no content at all, so `Matches It (And [])` tested
   nothing: the copular frame cannot demand a HEAD — "if it's
   attacking" and "if it's tapped" are real oracle and head
   nothing — so the weaker demand is its own table (`predSays`,
   `badMatchesNothing`).
106. **The modal headcount vocabulary is CLOSED, and the modes
   are distinct.** The frame admitted any range the algebra
   permitted, so "Choose up to two —" typechecked though no card
   writes it. Measured over supported lines carrying the bulleted
   em-dash: "choose one —" eighty-four, "choose two —" three,
   "choose three —" one, "choose up to one —" five, "choose one or
   both —" four, "choose one or more —" four, "choose any number
   —" two. Written zero times in any scope: "choose up to two —",
   "choose up to three —", "choose four", "two or more", "one or
   two". So the head vocabulary is the three small fixed counts,
   the one-capped "up to", the two open tops whose maximum IS the
   list, and the unbounded head (`modalHead`, `badModalUpToTwo`).
   The wave's proposed attested set omitted "up to one"; the
   corpus has it, so it is a row. Separately, two identical modes
   are one mode written twice and the choice between them decides
   nothing — [CR#700.2] wants "instructions for a player to choose
   a number of those options" and [CR#700.2d] has a player
   normally unable to "choose the same mode more than once", the
   cards that lift it saying so in words rather than by printing
   the bullet twice (`distinctModes`, `badDuplicateModes`). The
   check is structural and conservative in `predEq`'s direction,
   and most of `effEq`'s rows are that conservatism rather than
   laziness: a clause's later arguments are typed in the context
   its earlier ones built, so two clauses' payloads generally
   inhabit two different types and cannot be compared at all.
   Finally the attested unbounded head got its surface
   (`chooseAnyNumber`, Rankle, Master of Pranks), and its floor is
   honestly zero: [CR#107.1c] lets a player told to choose "any
   number" choose "any positive number or zero", and [CR#700.2]
   imposes no minimum of its own, so declining every mode is a
   legal reading and the quantity says so structurally.
107. **A token's characteristics are a PHRASE, not a set.**
   [CR#111.3] makes the stated characteristics the token's text,
   which cuts both ways: a color written twice is a word written
   twice, and the type words come in an order. Measured: "artifact
   creature" five hundred ninety-four lines against "creature
   artifact" none, "artifact land" four against none, "land
   creature" eleven against none (Dryad Arbor's token),
   "enchantment creature" thirty-three, and one line placing
   enchantment ahead of artifact — a single total order over the
   four words this vocabulary has, written once as a rank rather
   than guessed at pairwise, with the Enchantment/Land pair riding
   on transitivity and the Licid template's "creature enchantment"
   the one counterexample (`typeRank`, `badTokenTypeOrder`).
   Colors take the duplicate demand and NOT the ordering one, and
   the corpus is why: Additive Evolution writes "a 0/0 green and
   blue Fractal creature token", which the mana order would have
   spelled the other way round (`badTokenDuplicateColor`).
108. **A WRITTEN action count is at least one; a read one need
   not be.** "Draw zero cards", "create zero tokens", and "put
   zero counters" all typechecked and none is written anywhere, as
   a numeral or as a determiner, in any scope — [CR#121.1] makes a
   draw the movement of a card, [CR#111.1] a token a marker put
   onto the battlefield, [CR#122.1] a counter a marker placed on
   something, and a zero of any of them instructs nothing. What
   stays writable is the count that EVALUATES to zero: X's value
   is its controller's to choose and announce ([CR#107.3a]) with
   nothing flooring it, a for-each domain can be empty,
   "that much" can be nothing. So the gate is on the SPELLING and
   not the value (`writtenCount`, `badDrawZero`, `badCreateZero`,
   `badPutZeroCounters`), which is finding 45's discipline
   (`badForEachZero`) reaching the action counts while `Lit`
   itself stays ungated for the bounds — "mana value 0 or less"
   measures rather than instructs.
109. **What the wave got wrong, recorded rather than smoothed
   over.** All thirteen machinery claims reproduced exactly, which
   is the useful part; two card attributions did not. Angrath,
   Captain of Chaos was cited as writing "Choose an Army creature
   you control" and "If it isn't a Zombie, it becomes a Zombie in
   addition to its other types" — its reminder text writes neither
   ("Put two +1/+1 counters on an Army you control. It's also a
   Zombie. If you don't control an Army, create a 0/0 black Zombie
   Army creature token first.", the pre-errata wording). Both
   phrases are [CR#701.47a]'s own, which is where this file takes
   them from and always did. And "Wick, Whorled Mind", offered as
   a negative control for the Amass question, is not a card;
   nothing by that name exists. The blanket direction on finding
   100 was refused for the same kind of reason: real oracle DOES
   read an arm-introduced referent from outside — Through the
   Breach reads the may's body, Akoum Stonewaker reads the
   if-you-do arm's token ("If you do, create a 3/1 red Elemental
   creature token … Exile that token …"), and Baral and Kari Zev
   reads the if-you-DON'T arm's ("If you don't, create First Mate
   Ragavan … It gains haste until end of turn.") — so the leak was
   the unconditional JOIN, not the escape, and the fix is the join.
   Two findings were refused as fix-round work and ledgered
   whole: distributed creation plurality (Elephant Resurgence) and
   the each-of distributed recipient (Ajani, Adversary of
   Tyrants), which belong to the plurality axis and not to a gate
   on either clause.
110. **Modes are a list, re-attacked and unmoved.** Chapter
   twenty's ruling — a mode reads everything before the modal and
   nothing a sibling introduced, and a modal exports nothing — was
   put to a wider sweep: five hundred eighty-one supported modal
   card records, no sibling-mode referent read and no post-modal
   sentence reading a mode-introduced referent. The apparent
   counterexamples resolve BEFORE the list. Blizzard Specter's
   "That player" is the combat-damaged player of the trigger that
   heads the ability, not anything a mode named; Kogla and
   Yidaro's "It" is the source, read alike by both bullets; and
   Blood on the Snow's trailing "Then return a creature or
   planeswalker card …" writes a DESCRIPTION covering both modes'
   outcomes exactly where an anaphor would have gone, which is the
   positive proof rather than a near miss. [CR#700.2c] is the
   reason: an unchosen mode's targets are never announced, the
   spell being "treated as though it did not have those targets",
   so a sibling's phrase may have named nobody at all. No code
   changed.

## Chapter Twenty-Two — The Anchored Complement

Chapter twenty-two, the complement and the batch (evidence:
Nibelheim Aflame, War Screecher, Bellowing Aegisaur, Carnifex
Demon, Syphon Mind, Brash Taunter, Ulvenwald Tracker, Act of
Treason, Mind Flayer, Phyrexian Infiltrator; and, for what the
round measured rather than built, Prey Upon, Nightfall Predator,
Polukranos Unchained, Switcheroo, Vedalken Plotter, Mister
Negative, Tahngarth Talruum Hero, Master Thief by way of
[CR#611.2b]'s own example).

111. **The group complement is TWO subtractions, and the definite
   sweep filed them as one.** The sweep's entry named "the rest",
   "the other", and "both" a single family and called it the one
   deferral that named a missing PRIMITIVE — a binding records a
   mention's plurality and its determiner, neither of which
   carries how MANY, so nothing could presuppose a two-membered
   antecedent or subtract one mention from another. That is exact
   for those three words and wrong for the family beside them.
   "Each other creature", "other creatures you control", and "each
   other player" subtract a REFERENT from a DESCRIPTION, and a
   description has no cardinality to consult: the domain is
   whatever answers the phrase, minus one named thing. So the
   anchored half lands with no counting at all, and the
   subset-of-a-group half ("the rest", six hundred and forty-eight
   lines; "the other", a hundred and ten; "both", nineteen) keeps
   the blocker the sweep found, now stated as its own — and its
   carriers are elsewhere-blocked besides, every "the rest" line
   being a library partition after a look or a reveal (the
   information cluster and the ordered library, both ledgered).
112. **One word, two relations, two rows.** `Other` is the
   TARGETING distinctness a slot announces against the slots
   before it: [CR#115.4]'s "another target", whose anchor is every
   earlier target of the kind and whose presupposition the
   discourse answers. `OtherThan` is the anchored complement,
   which carries the referent it subtracts. They are not the same
   relation wearing two determiners — a phrase can be written with
   either and mean something different by it — so they are two
   constructors sharing one namespace and one English word, the
   shape `Compare`/`CompareAmt` and `Not`/`NotCond` already have.
   Ulvenwald Tracker and Brash Taunter are the pair that proves
   it: "Target creature you control fights another target
   creature" excludes the OTHER TARGET, "This creature fights
   another target creature" excludes the SOURCE, and the two
   sentences differ in no word.
113. **The anchor is a read, and the source is what the corpus
   anchors to.** The complement takes a `Bindingless` noun, the
   discipline `Matches` makes of its subject and for the same
   reason: "each other creature" announces one phrase, so an
   anchor written "a creature" or "target creature" would announce
   a second the sentence never spelled
   (`badComplementAnchorAnnounces`, `badComplementAnchorTargets`).
   That is what unblocks the ledger's self-exclusion entry, which
   had the diagnosis right and the remedy pointed the wrong way:
   it waited for `This` to ENTER the discourse, and what was
   needed instead was a phrase that does not search the discourse
   at all. Of the hundred and thirty-five "each other creature"
   lines, fifteen have a Choose-or-target clause before them on
   the same line and the rest are the source's; the plural "other
   creatures you control" (a hundred and thirty-seven) is the
   source's throughout.
114. **The complement re-keys the gates it shares rather than
   growing new ones.** Both spellings fill ONE selector slot — the
   guide gives other/another a single position — so the phrase-level
   cap counts them together (`badDoubleComplement`,
   `badOtherAndComplement`), neither is an alternative inside a
   coordination (`badComplementInOr`), and neither is negatable
   ("non-other" stays unwritten, `badNegatedComplement`). The one
   NEW obligation is the anchor's own head type, and it is
   `anchorFound`'s question asked of a noun instead of a mention:
   an anchor the phrase could not have described subtracts nothing,
   and no corpus line pairs "other" with a cross-head anchor
   (`badComplementCrossHead`). An anchor with no projected type at
   all — bare `This`, `You` — fits any head, which is `anchorTyOk`
   unchanged. The gate is written as an `if` over the cap rather
   than a third conjunct, because the cap makes the two branches
   exclusive and a `x && True` neutral is what leaves an abstract
   anchor stuck.
115. **`Simultaneously` threads ANNOUNCEMENTS, and that is the
   whole contrast with `Sequentially`.** The settled
   defer-until-first-witness design cashed in here, and the shape
   it arrived with is one line of difference from the sequence
   beside it: `Effects` threads `effIntro` — each clause reads what
   its predecessors DID — and `SimEffects` threads `preIntro`, so
   each element reads what its predecessors ANNOUNCED and nothing
   any of them did. That is what "one pre-state" means precisely.
   Every element reads ONE game state, which is [CR#608.2f]'s
   general "each such action is processed simultaneously" for one
   instruction spread over several objects — its own example being
   a control grant, Blatant Thievery gaining control of every
   target at once — and is the property an exchange DEPENDS on ([CR#701.12b] has each player
   gain control of the permanent that "was controlled by the other
   player" — both halves read the controllers as they stood before
   either applied); but the DISCOURSE still accumulates, because
   [CR#601.2c] announced every target as the text was written,
   long before any of it resolved. An element reading a sibling's
   zone retag or a sibling's damage outcome is refused
   (`badSimultaneousReadsRetag`, `badSimultaneousReadsOutcome`) —
   and the retag pin is the sharp one, since the same two clauses
   in a `Sequentially` typecheck. Chapter twenty-one built
   `preIntro` for a trailing conditional; it turns out to be what
   a simultaneous batch is made of.
116. **Outward the batch contributes its whole discourse, and the
   corpus is what settled it.** A hole would have been the
   conservative guess and it is wrong: Volatile Stormdrake reads
   its exchanged target in the next breath ("exchange control of
   this creature and target creature an opponent controls. If you
   do, … sacrifice that creature …"), and Sudden Substitution does
   the same ("Then the spell's controller may choose new targets
   for it"). The answer is written as the LAST element's own
   `effIntro` over the announcement telescope, which is the union
   for every row the container reaches today — a control grant
   introduces its phrase and retags nothing. A batch whose earlier
   element MOVED an object would lose that retag and force the
   union to be built rather than read off the end; no corpus line
   writes one, and the zone-exchange family [CR#701.12d] names is
   the ledger's. Hygiene mirrors the sequence's: two elements at
   least, no batch inside a batch (`badNestedSimultaneous`), and
   no SEQUENCE inside one either (`badSequenceInsideSimultaneous`
   — an ordered list inside an unordered one contradicts its
   container, and its own announcements would reach the next
   element from its last clause only). The all-or-nothing rule
   [CR#701.12a] states is a resolution fact about failed halves,
   not a typing one, and stays with the legality layer.
117. **The control verb is ONE row, and core splits it in two.**
   Core keeps a one-shot `Action::GainControl` for the exchange
   family beside a layer-2 `Modification::SetController` for the
   duration-bounded grants, and says so in the first constructor's
   own doc. English writes one verb with one optional trailing
   adverbial, and the adverbial is already the `Continuously`
   envelope's slot, so the split would spell one construction
   twice: `GainsControl` is a static row like `Gets` and `Gains`,
   and the unwritten span is [CR#611.2a]'s end-of-game default
   exactly as it is for them — which is what Phyrexian
   Infiltrator's reminder text says out loud, "(This effect lasts
   indefinitely.)". The subject is a slot for the reason
   `ChangeLife` and `Draw` have one, both spellings being ordinary
   oracle: the imperative's unpronounced `You` (two hundred and
   forty-six lines), the written subject (seventy-three), and the
   distributive one (ten), so no grammatical number is demanded of
   it. The patient is a permanent — [CR#110.2] gives every
   permanent a controller and [CR#109.4] gives an object neither
   on the stack nor on the battlefield none — so the battlefield
   demand tap and destroy already carry applies
   (`badGainControlGraveyard`); the STACK half of [CR#109.4] is
   real English ("exchange control of target noncreature spell and
   target creature") and waits with the stack-object CONTROL carrier —
   the spell carrier itself landed in chapter thirty and left that line
   where it was, finding 197.
118. **The for-as-long-as duration landed, and it made `Duration`
   a context.** Chapter eighteen measured this row and ledgered it
   on a missing clause rather than a missing shape; the clause is
   `GainsControl` and forty-two of the two hundred and ten corpus
   lines are its. [CR#611.2b] gives the row its meaning and prints
   a control grant as its own example (Master Thief's "gain
   control of target artifact for as long as you control this
   creature"). The cost is that a condition is typed in bindings,
   so every duration now is: `Duration : Bindings -> Type`, and the
   `Continuously` envelope threads its static effect's own
   announcements into the adverbial (`Duration (staticIntro se)`),
   which is what Old Man of the Sea needs — "for as long as … that
   creature's power remains less than or equal to this creature's
   power" reads the grant's own target. The type moved into the
   mutual block to get there, which is the whole of the structural
   change: `DurationEnd`, `SpanUse`, and both span tables never
   mention a binding and stayed where they were.
119. **A fifth construction renamed the span classes where a
   fourth had split them.** Chapter nineteen divided `BothGrants`
   when the type addition wrote one current-turn endpoint and not
   the other; the control grant writes BOTH, so it joins two
   classes rather than dividing them, and `BothGrants` and
   `GrantsAndTypes` stopped being true of themselves the way
   `EveryStatic` had — they are `GrantsAndControl` and
   `GrantsTypesAndControl` now. What it did divide is `Unclaimed`:
   "until the end of your next turn" was eighty-three lines of
   play permissions and control grants with no construction to
   claim it, four of them naked control grants, so the cell is
   `ControlGrantOnly` and the class is one construction's alone
   (`badKeywordGrantEndOfNextTurn`). The ninth class is the new
   adverbial's, everything but the type addition
   (`GrantsRestrictionsAndControl`): "for as long as" takes the
   control grant forty-two times, the restriction ten, the stat
   delta six, the keyword grant four, and the type ADDITION not
   once — the thirteen "becomes … for as long as" lines are type
   SETTINGS and copies, and the three that do write an addition
   under the adverbial write it with the copula ("That land IS an
   Island in addition to its other types for as long as it has a
   flood counter on it"), which is the layer words' construction
   (`badTypeAdditionForAsLongAs`). Two cells stayed shut on
   measurements rather than assumptions: the control grant never
   writes "this turn" (the four lines pairing the words are event
   clauses — "that attacked you this turn" — not adverbials,
   `badGainControlThisTurn`), and never an end step (the one line
   pairing them writes a DELAYED clause, "An opponent gains
   control of this land at the beginning of the next end step").
120. **The batch constructor arrived and the fight primitive
   survived it, which is finding 20 proved rather than
   restated.** That finding parked the fight macro on the group
   complement and said the obstacle was not the missing batch
   constructor. Both halves are now testable and both hold. The
   expansion is writable exactly when one participant is the
   SOURCE: `Simultaneously [this deals its power to the target,
   the target deals its power to this]` typechecks, because `This`
   introduces no binding and so leaves the target the unique
   mention the second element reads back. It is NOT writable for
   the frame the corpus actually writes — "Target creature you
   control fights target creature you don't control" (Prey Upon)
   puts two creature mentions in the second element's context, and
   `That creature` counts two. So the wall is the PAIR complement
   after all, exactly as the reminder text says it is ("Each deals
   damage equal to its power to the other."), and it is the
   subtract-a-subset half of finding 111 rather than the anchored
   half that landed. `Fights` stays primitive; nothing was
   deleted, and no second spelling of a fight was minted. What the
   complement DID unblock is the fight family's self-exclusion:
   "This creature fights another target creature" (Brash Taunter)
   and "Polukranos fights another target creature" are the same
   shape and neither could be written before, the bare `Other`'s
   anchor search reading bindings where the source leaves none.
   The self-fight reading was re-verified against the rule and
   stands: [CR#701.14c] says a creature that fights itself "deals
   damage to itself equal to twice its power", which is the
   same-source-same-target pair coalescing into one doubled event
   rather than two events of its power each — so the unconditional
   pair needs no distinctness gate. [CR#701.14b] is the
   both-or-neither guard and it is deliberately OUTSIDE the body:
   it asks whether each participant is still a creature on the
   battlefield when the ability resolves, which is legality and
   not grammar, and it will live with the trigger and legality
   layer beside [CR#701.12a]'s all-or-nothing rule. Neither is
   built here.
121. **An exchange of life totals is not a pair of transfers, and
   the rules say so.** The control exchange decomposes cleanly —
   [CR#701.12b] has each player "simultaneously" gain control of
   the permanent the other controlled, which is two one-way grants
   in one batch and is exactly what the bench writes. The life
   exchange does not: [CR#701.12c] has each player "gain or lose
   the amount of life necessary to equal the other player's
   previous life total", which is a SET realized as whichever of a
   gain or a loss gets there, with the direction depending on
   which total was larger. There is no symmetric pair to write,
   and forcing one would spell a rule that does not exist —
   [CR#119.7,119.8] confirm the asymmetry from the other side, a
   player who can't gain life being blocked from an exchange that
   would raise their total specifically. What it wants is a
   life-total READ (the player attribute the comparatives split
   already ledgered) and a set-to form; the corpus writes a
   set-to elsewhere ("life total becomes", twenty-nine lines), so
   the form is real and its own axis. Seven lines write the
   exchange. Ledgered, with the reason rather than the count.

## Chapter Twenty-Three — Plurality and Distribution

Chapter twenty-three, where number stops being a per-phrase fact
(evidence: Grismold, the Dreadsower, Sparkmage's Gambit, Ajani,
Adversary of Tyrants, Fall of the Titans, Nature's Panoply, Arc
Lightning, Forked Bolt, Boulderfall, Armament Corps; and, for
what the round measured rather than built, Elephant Resurgence,
Oversimplify, Rendmaw, Creaking Nest, Additive Evolution, Rowan,
Fearless Sparkmage, Crackle with Power, Sorrow's Path, Prey Upon).

122. **The plurality of a created group is the CLAUSE's, and the
   count was only half of it.** `Create` read its output number
   off the count alone, which is exact for the imperative and
   wrong the moment the AGENT distributes: "Each player creates a
   green Elephant creature token. Those creatures have …"
   (Elephant Resurgence) writes a count of one and reads back a
   plural, because one token per player over many players is many
   tokens. The fix is a two-argument table in the machinery that
   already computed the number (`outputPlur`, re-keying
   `effIntro`'s `Create` row) rather than a second plurality path
   beside it: many agents or a plural count make a plural mention,
   and the singular survives only when both halves are singular —
   which is Additive Evolution's "create a 0/0 … Fractal creature
   token. Put three +1/+1 counters on it." unchanged. The refusal
   is the sharp half and it is the same two sentences with one
   determiner moved: after a distributed creation the singular
   pronoun resolves to nothing (`badDistributedCreationIt`).
   Eighteen corpus lines write "each player creates", eight "each
   opponent creates", one "each other player creates"; Grismold,
   the Dreadsower is the bench positive, and Elephant Resurgence's
   own second sentence is NOT, for two reasons found by trying to
   write it. It grants a quoted ability, which is unbuilt; and its
   token writes no P/T, which `tokenPtOk` refuses. That refusal's
   ledger claim — that every corpus creature-token line writes its
   two numbers — is FALSE, and the exception is systematic rather
   than stray: ten lines create a creature token with no printed
   P/T, and every one of them supplies the numbers with a
   characteristic-defining ability in the same breath ("Create a
   white Avatar creature token. It has 'This token's power and
   toughness are each equal to your life total.'"). The gate is
   still right — a creature token's numbers come from SOMEWHERE
   [CR#111.3] — and its stated reason is not; the CDA is the other
   source, and it arrives with the quoted-ability vocabulary.
   One boundary the pin does not claim: Oversimplify writes "Each
   player creates a 0/0 green and blue Fractal creature token and
   puts a number of +1/+1 counters on IT", where the singular is
   correct because the second verb is INSIDE the distributive
   scope. That is a coordinated verb phrase under one subject, a
   construction this grammar does not have (`Sequentially` is the
   sentence-level stand-in and the pin is a sentence-level claim),
   and it is what the fronted iteration clause would spell.
123. **"Each of" is a determiner over a MENTION, where "each" is a
   determiner over a description.** That one sentence is the whole
   of `EachOf`, and it is why the constructor sits beside `Each`
   rather than inside it. `Each` distributes over whatever answers
   a phrase, so it announces the phrase; "each of up to two target
   creatures" distributes over the members of a mention already
   made, so it announces NOTHING — the targets were announced when
   the group was written [CR#601.2c], and the word "each" adds no
   referent (`nounDelta` passes its complement's delta through
   unchanged). Two consequences follow and both are corpus facts.
   The phrase stays PLURAL, so the next sentence reads it as one
   group: "Sparkmage's Gambit deals 1 damage to each of up to two
   target creatures. Those creatures can't block this turn." is
   the whole card and the bench's witness, and Rowan, Fearless
   Sparkmage writes the same two sentences. And the complement is
   exactly the group forms the corpus writes after the word,
   measured rather than assumed: a counted target mention ("each
   of up to two target creatures", twenty-nine lines; "each of up
   to X target creatures", nine; "each of up to three targets",
   four; "each of two target creatures", three; "each of any
   number of target creatures", three) or a plural read ("each of
   them", thirty-seven; "each of those creatures", twenty-four;
   "each of those cards", four; "each of those tokens", three).
   Nothing else — not a second distributive, not the universal,
   not another "each of" (`badEachOfDistributive`, `badEachOfAll`,
   `badNestedEachOf`) — and the determiner needs a plural
   complement to reach into at all (`badEachOfSingular`). That
   closed list is `GroupMention`, and naming it is worth more than
   the gate it discharges: it is the first time this grammar has a
   word for "a phrase whose members the sentence may reach", which
   is what a division needs too and what the pair complement is
   still missing (finding 127). The determiner is general and not
   a recipient's: real oracle writes it as a subject ("Each of
   them gets +X/+X and gains vigilance until end of turn",
   Sigardian Zealot; "Each of them searches their library …") and
   over a move ("return each of them to the battlefield under ITS
   owner's control"), where the singular possessive inside the
   clause is the per-member reading made visible.
124. **A per-member amount asks its recipient a question, and
   English answers it with a word rather than leaving it to
   arithmetic.** The damage and counter clauses write ONE
   magnitude and one recipient phrase, and between them the two
   have to say whether the magnitude is each member's or the
   group's. A singular recipient asks nothing; the two
   distributive determiners answer outright — `Each` over a
   description (Bellowing Aegisaur's "put a +1/+1 counter on each
   other creature you control") and `EachOf` over a group mention
   (Ajani, Adversary of Tyrants' "on each of up to two target
   creatures"). A BARE plural recipient answers neither, and the
   corpus never writes one. "Put a … counter on" reaches a counted
   group through "each of" and no other way: zero lines at two, at
   three, or at four, against twenty-nine at "each of up to two
   target creatures" — while "up to ONE target creature",
   twenty-eight lines, is singular and passes on its own number.
   The damage verb reads identically. The plural READS tell the
   same story from the other side: "counters on them" is
   thirty-six relative clauses ("cards with intel counters on
   them") and no recipient, "damage to them" is the singular
   epicene player every one of its twenty-five times, and "damage
   to those …" is written zero times. So is the universal —
   "damage to all creatures" and "counter on all creatures" are
   zero lines each, the sweep being written distributively
   ("damage to each creature", two hundred thirty-five). That is
   `PerMember`, shaped after `damageSrcOk` because it is that
   gate's mirror at the other end of the verb: two named rows and
   a number catch-all (`badBarePluralCounterRecipient`,
   `badBarePluralDamageRecipient`, `badThemCounterRecipient`,
   `badAllOfDamageRecipient`).
125. **The class word's plural ban was written as a maximum and
   should have been a minimum.** `AnyTargetAtCount` admitted "any
   target" at exactly one and under "up to", and refused the exact
   group from two up AND the unbounded "any number of" — the
   second on the ground that its structure was unbuilt. The
   structures landed, and the corpus then says something sharper
   than the old gate did. Every plural spelling [CR#115.4] names
   runs from one and leaves the count to the caster: "up to two
   targets" (Fall of the Titans), "up to three targets" (Jaya's
   Immolating Inferno), "one or two targets" (Forked Bolt, twelve
   lines), "one, two, or three targets" (Arc Lightning, nine),
   "any number of targets" (Boulderfall, eighteen under "among").
   What it never writes is a FIXED plural count — "two targets" is
   zero lines and "among two targets" is zero lines — because
   those spellings belong to structures that let the caster choose
   how many things to hit, which [CR#601.2c] announces as a
   variable target count and [CR#601.2d] pairs with an announced
   division. So the gate asks for a minimum of one or none at all,
   `badAnyNumberAnyTarget` retires against Boulderfall, and one
   refusal is left (`badGroupAnyTarget`). What used to ride on the
   quantity — that a plural class-word mention may not stand as a
   bare recipient — moved to `PerMember`, where the clause that
   writes the amount is the one asking.
126. **The division is ONE mechanic with two idioms, and core's
   free body is wider than English.** [CR#601.2d] and [CR#115.7f]
   both name "divide or distribute" as a single thing over a
   single pair of examples ("such as damage or counters"), and
   core reads that as one primitive with a free body and a
   `Count::Allotment` anaphor for the share. English writes it
   with two idioms and no third: the damage verb takes an
   adverbial and swaps its preposition ("Arc Lightning deals 3
   damage DIVIDED AS YOU CHOOSE among one, two, or three targets",
   sixty-nine lines), and the counter verb changes its own word
   ("DISTRIBUTE two +1/+1 counters among one or two target
   creatures you control", Armament Corps, forty-six lines). The
   spellings never cross — "counters divided as you choose" is
   zero lines, "distribute … damage" is zero lines — and no other
   verb divides anything. So `Distribute` lands with a CLOSED
   two-row verb table (`DividedVerb`) rather than a free body: a
   recorded narrowing of core, and the honest English claim, since
   a free body would spell instructions the language does not
   have. The share stays implicit for the same reason — the words
   "divided as you choose" ARE the allotment, and no line names a
   member's share twice. The recipient is a plural `GroupMention`
   and the division's own demand rather than a borrowed one
   (`badDivideAmongSingular`, `badDivideAmongDescription`), and
   each row keeps the obligation its undivided twin carries, which
   is what lets the class word stand under "among"
   (`badDistributeCountersGraveyard`). Two rules were read against
   the structure and both belong outside it. [CR#601.2d]'s floor —
   each target "must receive at least one of whatever is being
   divided" — is a legality question about the announced numbers
   and not an ungrammatical sentence, so it goes to the legality
   layer beside [CR#701.14b] and [CR#701.12a]. And [CR#608.2d]
   runs the identical split at RESOLUTION for untargeted
   recipients ("distribute that many +1/+1 counters among any
   number of creatures you control"); it is the same structure
   read at a different time, and the group mention is the only
   thing that differs, so it needs no row of its own. What
   [CR#115.7f] adds is the argument that a division is a structure
   at all: the original division survives a later change of
   targets, which is a fact about a division as an object rather
   than about the sentence that announced it.
127. **The pair the reminder text reads is a pair the sentence
   never announces, and that is the fight's real blocker.**
   Finding 120 left the two-target fight expansion unwritable and
   called the wall the pair complement. The wall is one step
   further back. "Target creature you control fights target
   creature you don't control" uses the word "target" twice, and
   [CR#601.2c] announces each instance separately — the same
   object may even be chosen for both — so the sentence puts TWO
   mentions in the discourse and never a two-membered group. The
   reminder text's "each deals damage equal to its power to the
   OTHER" reads a pair that the operative text did not build, and
   no complement can subtract from a group that was never
   assembled. That is why the expansion fails, and it is a reason
   to keep `Fights` primitive rather than an absence of machinery.
   The pair complement itself is real English and this round can
   now say exactly what it needs. Sorrow's Path is the
   operative-text witness — "Choose two target blocking creatures
   controlled by the same opponent. If each of those creatures
   could block all creatures that the other is blocking, remove
   both of them from combat." — and it shows the three parts
   arriving together: a group mention that IS a pair (one "choose"
   over an exact count of two), an each-of reach into it, and "the
   other" inside the per-member scope. Two of the three are now
   built. What is missing is the ELEMENT-scoped context, which is
   the fronted iteration clause "For each of [group], …"
   (thirty-seven corpus lines as counted then; REMEASURED in chapter
   thirty-two at twenty-five supported fronted — fifty-one in any
   position, and fronted is the count this entry wants — core's
   `OneShotEffect::Each`, and
   deliberately not what `EachOf` is — a determiner binds no
   element), and a complement whose domain is a group mention
   rather than a description, which is finding 111's
   subtract-a-subset half unchanged. The "both" family measures
   the same shape from the other side and is where the sweep's
   nineteen-line count resolves: thirteen "both creatures" lines,
   eleven of them soulbond statics whose pair comes from the
   pairing and two whose pair comes from a blocking EVENT
   ("Whenever equipped creature blocks or becomes blocked by a
   creature, destroy both creatures", Dead-Iron Sledge), plus
   three "both of them" and one "both players". Every one of them
   reads a pair some earlier structure assembled — a soulbond
   pairing, a combat event, a two-target choice — and none
   assembles one out of two announcements. `Fights` stays
   primitive, nothing was deleted, and the settled Fight-as-macro
   design and the two-target exchange stay recorded rather than
   cashed.

## Chapter Twenty-Four — The Ordered Library

Chapter twenty-four, where the hidden zone gets a grammar and the
complement finally has something to subtract from (evidence: Impulse,
Anticipate, Peek, Sylvan Scrying, Glimpse the Unthinkable, Grenzo,
Dungeon Warden; and, for what the round measured rather than built,
Ral's Outburst, Dark Bargain, Telling Time, Index, Ponder, Lay of the
Land, Diabolic Tutor, Thought Scour).

128. **The library's ORDER is the zone's and the position is the
   placement's, and English writes them in two different phrases.** The
   library lands as a fifth `Zone` row and nothing about its order lives
   there: [CR#401.2] makes it "a single face-down pile" whose order
   players may neither inspect nor change, and where a card sits in that
   pile is what a PLACEMENT says, not what a zone sort records. So
   `ZoneAt Library` is the zone whole — what a search looks through and
   a shuffle randomizes — and `LibraryAt pos ord Bare` is a place in it,
   and the bare zone is refused as a destination outright
   (`badMoveToBareLibrary`): "put it into your library" names nowhere to
   put it and oracle never writes it. That refusal is core's, arrived at
   independently and stated in core's own words — `Destination` carries
   `exclude(Library, Stack)` and its doc calls the exclusion "the Idris
   `DestinationOk` gate", the anchored form being "the single canonical
   spelling". The two words are the whole attested position vocabulary,
   and they divide the labour asymmetrically: the TOP is the source word
   (twelve hundred thirty-eight lines write "the top card/cards of your
   library", seventy-one more another player's) and the BOTTOM is the
   destination word (four hundred eighteen placements against ONE
   source, Grenzo, Dungeon Warden's "the bottom card of your library").
   Both rows exist because both are attested somewhere, and the count is
   recorded so the one-line row is not mistaken for a measured family.
129. **The order RIDER restates a rule in one form and overrides it in
   the other, and it needs a plural because the rule does.**
   [CR#401.4] answers the arrangement question before any card does: if
   an effect puts "two or more cards in a specific position in a library
   at the same time, the owner of those cards may arrange them in any
   order". So "in any order" (two hundred thirty-seven lines) says what
   would happen anyway and "in a random order" (three hundred
   thirty-two) is the only rider that changes anything — which is why
   the absent rider and the any-order rider mean the same thing and both
   are written. The rule's own "two or more" is the gate: a singular
   placement has no order to state, and English agrees exactly, "put it
   on the bottom of your library in any order" being zero lines
   (`badSingularOrderRider`). Two rows, and core has four — `SameOrder`
   and a `ChosenOrder(Reference)` are real structures with zero English
   lines behind them ("in the same order", "in an order of their
   choice", both zero) — so `Arrangement` is a recorded NARROWING of
   core in finding 126's sense, the second time a corpus measurement has
   made this file's table smaller than the taxonomy's. The rider rides
   the DESTINATION rather than the verb, because that is where English
   writes it; what the verb owns is the agreement between the rider and
   its patient's number. Core splits the same fact into a second verb
   (`MoveGroup`) because its group term is a different sort; here
   plurality is a property of the one patient slot, so no second row was
   minted.
130. **The slice describes a PLACE and not a card, and that silence is
   the whole of this round's hidden-zone honesty.** `LibrarySlice`
   projects no head type, and it is not a hedge: the cards are in a
   hidden zone ([CR#400.2]) whose order and contents no player may
   inspect ([CR#401.2]), so a phrase that picks them out by position has
   said nothing about what they are. Every consequence follows from that
   one answer with no gate of its own — the battlefield-demanding verbs
   refuse the slice through the zone demand they already carried
   (`badTapLibraryTop`), and a typed demonstrative has nothing to reach
   afterwards (`badSliceTypeRead`, the sharp one: "Look at the top four
   cards of your library" leaves no "those creatures" behind however
   many creatures are up there). What DOES reach it is "those cards",
   because a library is a card zone ([CR#108.2], `isCardZone`) — which
   is Telling Time's own spelling ("Put one of those cards into your
   hand", fifty-two lines) and the reason the type-blind word is the
   only one. The grammar encodes what the sentence says about the pile,
   and the sentence says where.
131. **LOOK and REVEAL are one operation with two audiences, and the
   audience is not a slot.** [CR#701.20a] has revealing "show that card
   to all players"; [CR#701.20e] has looking follow "the same rules as
   revealing a card, except that the card is shown only to the specified
   player" — one operation, two audiences, and the specified player is
   the clause's own subject. So the axis is a verb TAG and not an
   audience slot: an audience slot would let the grammar write "reveal
   it to target opponent", which oracle does not, while the verb carries
   no second player at all. Core draws the line in the same place from
   the other side, one `Reveal { what, to }` whose optional `to` its doc
   says "names a player instead = 'look at'". Exposing MOVES nothing
   ([CR#701.20b]), which is why the clause has no retag and why the
   following placement has somewhere to move the cards FROM. The
   complement is two things and measured: a card group (four hundred
   forty-nine "look at the top N cards of your library" lines, two
   hundred twenty at one card, ninety-five and ninety-nine for the
   reveal) and a whole HAND, which is a ZONE rather than a group —
   "reveals their hand" is a hundred twenty-eight lines and "reveal all
   cards in your hand" is zero, so the zone phrase is the construction
   and not an abbreviation of one. The hand exposure contributes
   NOTHING outward, and that is measured too: every corpus line that
   reads a revealed hand back reads it as a zone ("that player exiles a
   card from IT", "you choose a card … from it"), which is a zone
   anaphor this grammar has no word for, so the clause introduces its
   possessor and refuses to invent a card group oracle never names.
   Only the hand is exposable — a graveyard and the battlefield are
   public already ([CR#400.2]) and a library is exposed by SLICE, never
   whole (`badRevealWholeLibrary`, `badRevealGraveyard`).
132. **The partition landed, and what unblocked it was the GROUP rather
   than any counting.** Finding 111 split the complement family in two
   and left the subtract-a-subset half blocked on a missing primitive: a
   binding records plurality and determiner, neither of which carries
   how many, so nothing could subtract one mention from another. That
   diagnosis was right about MENTIONS and wrong about what "the rest"
   needs. The chain that writes Impulse has three links and none of them
   counts anything: a slice phrase ASSEMBLES one group, a partitive
   TAKES members out of that group, and the complement is what the group
   has left. So the gate asks for exactly that pair — one group mention,
   and at least one part taken from it — and both halves earn a pin
   (`badRestWithoutGroup`: nothing to be the rest of;
   `badRestWithoutPart`: with nothing taken, "the rest" IS the group and
   the sentence would have written "them"). The partitive needed a
   determiner tag of its own (`PartD`) for one reason, and it is the
   reason the gate could not have been written over `countManys`: "four
   of them" is plural too, so a context after a counted partitive holds
   two plural object mentions and only one of them is a group. What
   chapter twenty-two refused STAYS refused, and it has its own pin
   (`badRestOverTwoAnnouncements`): two separately announced targets are
   two mentions and never a pair ([CR#601.2c], finding 127), so no
   complement can subtract inside them and the binary fight's expansion
   is no closer than it was. Impulse and Anticipate are the bench
   witnesses, the reveal half is written beside them, and the counted
   partitive has its own ("Exile four of them at random, then put the
   rest on top of your library in any order").
133. **The complement's own count was three families wearing one word,
   and finding 111 had counted all three.** Six hundred forty-eight
   corpus lines write "the rest", and they are not one thing. Nineteen
   are the TEMPORAL idiom — "You have no maximum hand size for the rest
   of the game", "goaded for the rest of the game" — which is not a
   group complement at all and belongs to the duration vocabulary if
   anywhere. Of the six hundred twenty-nine that remain, five hundred
   ninety-nine name a LIBRARY, five hundred sixty of them writing the
   assembling verb outright (a look at the top, a reveal of the top, an
   exile from the top), and those are what this round opened. Of the
   thirty that name no library, twenty-seven subtract from a set a
   CHOICE or a pile separation fixed ("Each player chooses three
   permanents they control, then sacrifices the rest"; "Target player
   chooses a card in their hand and discards the rest"), which needs the
   counted untargeted group and the agentful choice clause, both
   ledgered; the last three subtract from another assembled group — the
   cards exiled with a Saga among them — and each waits on the structure
   that assembled it. So
   finding 111's parenthetical claim that every "the rest" line is "a
   library partition after a look or a reveal" is FALSE as stated and
   right about the bulk: the correction is recorded here rather than
   edited there, and the round opened five hundred ninety-nine lines'
   worth of frame rather than six hundred forty-eight.
134. **"The other" is the same complement with a singular remainder, and
   THAT is what finding 111's counting blocker actually blocks.** Ral's
   Outburst writes "Look at the top two cards of your library. Put one
   of them into your hand and the other into your graveyard" and Dark
   Bargain writes "Look at the top three cards of your library. Put two
   of them into your hand and the other into your graveyard" — the same
   three-link chain as Impulse, with the group one member larger than
   what was taken. So the word is not a different relation; it is the
   complement's SPELLING, chosen by the remainder's grammatical number,
   which is the group's size minus the part's. That subtraction is the
   arithmetic finding 111 said a binding cannot do, and locating it here
   is the sharpening: the counting blocker does not block the complement
   at all, it blocks the complement's WORD CHOICE. Of the hundred
   sixty-one "the other" lines (word-bounded; the definite sweep's
   hundred and ten counted a narrower pattern), sixty-six sit inside
   parenthetical
   reminder text (the fight gloss above all), and the group-internal
   complements are about sixty-nine — sixty writing "the other into
   [zone]" and nine "the other on [position]". Ledgered with the count
   and with the one thing it needs, which is cardinality in the payload.
135. **A mill introduces its group where a draw does not, and the RULES
   say why rather than the corpus.** `Draw` was measured silent — no
   corpus line reads a drawn card back — and the reason looked like an
   accident of what cards happen to say. It is not. [CR#701.17c] lets an
   effect that refers to a milled card "find that card in the zone it
   moved to from the library, as long as that zone is a public zone",
   which a graveyard is and a hand is not ([CR#400.2]); [CR#400.7j] says
   the same thing generally. So the very rule that licenses the mill
   read denies the draw one, and the corpus agrees exactly: sixty-one
   lines read a milled group back and zero read a drawn card. `Mill` is
   `Draw`'s twin in every other respect — its own row rather than a
   `Composite` tag, because a mill moves a slice the sentence never
   names and there is no patient phrase to wrap; the subject a slot for
   the same reason ("Target player mills ten cards", a hundred eighteen
   lines; "Each player mills", thirty-four). Its group carries no head
   type, the same silence the slice keeps and for the same reason. What
   is missing is not the mention but its READERS: the corpus reads it
   with the among-restriction ("from among them", eighteen) and the
   "this way" participle ("milled this way", forty-three), neither of
   which this vocabulary spells, so the bench reads it with the plural
   demonstrative instead. Glimpse the Unthinkable is the whole-card
   witness.
136. **A shuffle DESTROYS discourse, and it is the one visibility rule
   the corpus forces into the machinery.** [CR#701.24a] randomizes a
   library "so that no player knows their order" and [CR#701.20d] makes
   any revealed card that gets reordered "stop being revealed and become
   a new object". A mention still lying in the shuffled library is
   therefore not there to be named, and `effIntro` drops exactly those
   (`badReadAfterShuffle`). Mentions that LEFT the library first are
   untouched — which is both what [CR#701.24b] says of the found cards
   ("the found cards aren't included in the shuffle") and what every
   search sentence in the corpus relies on, writing its placement before
   its shuffle without exception. This is the visibility stance in its
   entirety, and it is deliberately small: no information-set model, no
   per-player knowledge, no exposure flag on the payload. What the
   grammar encodes is what the SENTENCE does — a look, a reveal, a
   public destination, a shuffle — and the one place a visibility
   distinction changes what later text may honestly reference is where
   it gets a mechanism.
137. **The search's zone is on the VERB, so its description carries
   none.** [CR#701.23a] gives the clause its three parts — "look at all
   cards in that zone (even if it's a hidden zone) and find a card that
   matches the given description" — and the row is those three: who
   searches, which zone, what description. The description is a
   PREDICATE rather than a noun phrase, and that is forced: written as a
   noun, "a creature card" would seed the battlefield ([CR#109.2]) and
   the clause would have to overwrite its own argument's projection, so
   the clause mints the found mention itself and places it in the zone
   it was found in. A description carrying its own zone answers the
   question twice and is refused (`badSearchZonedDescription`,
   `ZoneFree`). Which zones are searchable is measured and is exactly
   the rule's own subject matter: the library (eight hundred twenty-five
   lines), the graveyard (ninety-five), the hand (twenty-one), and
   nothing public — "search the battlefield" is zero lines, because
   there is nothing to find in a zone everyone can already read
   (`badSearchBattlefield`). The REVEAL is a separate clause and not a
   rider because [CR#701.23e] says so outright: "if the effect that
   contains the search instruction doesn't also contain instructions to
   reveal the found card(s), then they're not revealed" — the exposure
   is written or it does not happen. Sylvan Scrying writes all four
   clauses and is the bench's whole-card witness; the destination split
   has its own positive. Two rules were read against the structure and
   both belong outside it: [CR#701.23b]'s permission not to find in a
   hidden zone and [CR#701.23d]'s obligation to find a bare quantity are
   legality questions about a resolution choice, not ungrammatical
   sentences, so they go to the legality layer beside [CR#601.2d]'s
   division floor and [CR#701.14b]'s fight guard. The grammar encodes
   the sentence; the fail-to-find rule encodes what a player may do with
   it.
138. **The set-to leaves no outcome behind, and finding 121 is the
   reason.** `LifeOp` grows its third row — "life total becomes",
   twenty-nine corpus lines, chapter twenty-two's ledgered
   player-attribute set — and the interesting half is what it does NOT
   contribute. The two deltas each leave an outcome mention ("that
   much" reads a magnitude a clause produced), and a set-to leaves
   none, because [CR#701.12c] is right that a set is realized as
   "whichever of a gain or a loss gets there" and the direction depends
   on where the total stood. The sentence does not say which, so the
   clause records neither rather than guessing one — finding 121's
   asymmetry, structural rather than argued.

## Chapter Twenty-Five — The Intercepted Event

Chapter twenty-five, where the grammar learns to write a sentence
about an event that has not happened (evidence: Bot Bashing Time,
Words of War, Words of Worship, Fog, Indestructible Aura,
Shieldmate's Blessing, Banisher Priest, Tezzeret Artifice Master,
Zimone Quandrix Prodigy; and, for what the round measured rather
than built, Overload, Fatal Push, Anoint with Affliction, Ana
Sanctuary, Galvanic Blast, Banishing Light, Adaptive Shimmerer,
Scarwood Treefolk, Doubling Season, and regeneration by way of
[CR#614.8]).

139. **A replacement effect is a CONTINUOUS effect, so the envelope
   was already there.** [CR#614.1] opens the replacement chapter with
   the sentence that decided this round's shape — "Some continuous
   effects are replacement effects" — and [CR#615.1] opens the
   prevention chapter with its twin. So the would/instead clause is
   not a new kind of sentence at all: it is a `StaticEffect` row under
   the `Continuously` envelope chapter seventeen built, its duration
   adverbial the same one slot, and its span answered by the same two
   tables. Core had already filed it there
   (`StaticEffect::Replacement`, `StaticEffect::Prevention`,
   `deckmaste_core/src/continuous.rs`), which is corroboration and not
   the reason; the reason is that the adverbial behaves exactly as
   chapter seventeen said an adverbial behaves. What the two new rows
   part from the older five on is that they modify no OBJECT.
   [CR#611.2c] cuts continuous effects in two — those that "modify the
   characteristics or change the controller of any objects", whose
   affected set is fixed when the effect begins, and those that do
   neither and so "modify the rules of the game" — and the rule's own
   worked example is a prevention effect. So `Intercepts` and
   `Prevents` carry no subject noun where `Gets`, `Gains`, `Cant`,
   `BecomesAlso` and `GainsControl` all do, and what they contribute
   to the discourse is what their PHRASES announced rather than what
   they will do to anybody.
140. **What the workbench encodes is the SENTENCE, and the
   layered machinery is expressly not here.** [CR#614] is mostly a
   chapter about APPLICATION — [CR#614.5] gives an effect one
   opportunity per event, [CR#614.6] says a replaced event never
   happens, [CR#616.1] orders competing effects through a five-step
   choice in APNAP order, [CR#616.2] lets one replacement make another
   applicable. None of that is written on a card and none of it is
   here. What IS written on a card is a sentence with two halves — the
   event intercepted and the substitute — and that is the whole of
   what this round encodes. The rules that DO reach the sentence reach
   it as gates: [CR#614.6] is why the replaced clause's outcome is
   unreadable, [CR#614.7] is why the replacement contributes nothing
   outward, [CR#614.5] is why an `InsteadOf` does not nest, and
   [CR#614.3]'s two ways for a shield to end is why the multiplicity
   word is a slot. The application layer is the engine's, filed with
   the damage pipeline the deferrals section already parks there.
141. **The event patterns are ONE vocabulary two constructions read
   in two moods.** "Would die" and "leaves the battlefield" are the
   same happening written twice: the interception names it to watch
   for it and puts "would" in front, the [CR#610.3] rider names it to
   wait for it and writes it finite. So `GameEvent` carries the
   pattern and the reading construction supplies the mood, which is
   the same cut the grammar already makes between a predicate and the
   frame that spells it. Which events each construction may read is a
   table and not the type's business, and the table tells the two
   silences apart the way `spanUse` does: `EventUnattested` for a
   happening no clause anywhere is written over, `EventUnclaimed` for
   one that is real oracle English with no construction here. Two rows
   are claimed and two are not, and the unclaimed pair is worth its
   reasons. `IsDestroyed` is thirty-one lines and twenty-five of them
   are regeneration's own reminder text, whose replacement is
   [CR#614.8]'s four-part instruction — tap it, remove it from combat,
   heal the damage on it — not one part of which this vocabulary
   writes. `IsDealtDamage` is three hundred eighty-five lines and its
   replacement is the redirection family [CR#614.9] names, which wants
   a damage clause whose amount is the intercepted event's own. Below
   those, the events measured and NOT minted, each with its count: the
   put-into-a-graveyard event (fifty-seven), the enter event
   (fourteen), the token-creation event (thirteen, Doubling Season's
   own), and the life-change pair (eight and one).
142. **The multiplicity word belongs to the EVENT, and the corpus
   assigns it without exception.** [CR#614.3] gives a replacement
   effect two ways to end — "until they're used up or their duration
   has expired" — and English marks the difference with the clause's
   opening word: "if [subject] would …" for the shield that fires
   whenever the event happens, "the next time [subject] would …" for
   the shield that fires once. The measurement is total and it is not
   free variation. "The next time [subject] would die" is written ZERO
   times against fifty-seven "if … would die this turn"; "if you would
   draw a card this turn" is written ZERO times against nine "the next
   time you would draw"; regeneration's reminder text writes "the next
   time [permanent] would be destroyed this turn" twenty-five times
   and the conditional never. The reason is the event's own
   repeatability: a creature dies once, so an unlimited shield and a
   single-use one are the same shield and English takes the shorter
   word; a draw repeats, so the two are different effects and the
   writer must say which. That makes the word a gate rather than a
   spelling note (`replUseOk`, `badNextTimeWouldDie`,
   `badIfWouldDraw`) — a rare case where a closed table over two words
   records a semantic fact rather than a convention.
143. **The shield's span is one word, and it splits the
   restriction's.** Two hundred fifty-five prevention clauses carry
   "this turn"; "prevent … until end of turn", "prevent … until end of
   combat", and "prevent … for as long as" are written zero times
   each, all scopes. So `Prevention` writes exactly one adverbial and
   the current-turn word that chapter seventeen found belonged to the
   deed restriction alone belongs to three constructions now
   (`RestrictionsAndShields`). The interception is wider and by two
   cells rather than by argument: besides "this turn" it writes the
   fronted cross-clause endpoints, "Until end of turn, if one or more
   tokens would be created under your control, twice that many of
   those tokens are created instead" (seven lines) and "Until your
   next turn, if that creature would deal combat damage to one of your
   opponents, it deals triple that damage to that player instead"
   (two). Both cells were named for the constructions that already
   wrote them and are renamed rather than split, which is chapter
   twenty-two's move a second time. And both shields answer `absentOk`
   with `False` for the deed restriction's reason exactly: a
   durationless interception or shield is the STATIC ABILITY line
   ([CR#611.3] — a continuous effect from a static ability states no
   duration because it lasts while the ability functions), which the
   corpus confirms from both sides. Every one-shot shield states a
   span; the forty-nine durationless "Prevent all …" lines are static
   abilities to a line ("Prevent all combat damage that would be dealt
   to enchanted creature"), and so are the standing interceptions ("If
   a creature an opponent controls would die, exile it instead").
   `badStaticCant` a third and fourth time (`badStandingIntercept`,
   `badStandingPrevention`).
144. **The event-ended "until" is TWO constructions wearing one
   phrase, and the ledger's ninety-one lines are ninety-one lines of
   the wrong one.** The entry the ledger has carried since chapter
   seventeen said the O-Ring endpoint waited on an events axis, and
   the axis arriving is what showed the entry was mis-filed. [CR#610.3]
   files "exile [object] until [event]" under ONE-SHOT effects, not
   continuous ones: the clause resolves once, and "a second one-shot
   effect is created immediately after the specified event" returns
   the object to its previous zone. [CR#610.4] says the same of the
   phase-out twin. Measured, the ninety-one lines are eighty-six
   exiles, three phasings, and three genuine [CR#611.2a] continuous
   effects — two base-TYPE settings ("Target land becomes a Forest
   until this creature leaves the battlefield") and one becomes-a-copy,
   every one of them a construction this grammar lacks. So the
   `Duration` row exists and its answer is `Unclaimed`
   (`badGetsUntilLeavesBattlefield`), and the writable half of the
   family is a RIDER on the clause instead (`HeldUntil`), gated to the
   one clause the corpus hangs it on. Core makes the opposite cut and
   says why in its own comment: `Duration::UntilEvent` there is one row
   for both, cited to [CR#610.3] with the note that "the engine pairs
   the undo one-shot". That is right for a runtime, which must schedule
   the undo either way, and wrong for a grammar, where the two phrases
   sit in different slots of different clauses. Two more things the
   rider does not need: [CR#610.3c] returns the object "under its
   owner's control unless otherwise specified", so there is no
   controller slot, and the object's ZONE afterwards is not settled by
   the sentence at all — the undo hangs on an event that has not
   happened — so the clause contributes its announcement and not the
   exile's retag (`badHeldUntilExileRetag`).
145. **The self-replacement is the OTHER "instead", and the corpus
   divides the two on one word.** Six hundred twenty-nine of the
   thousand and thirty-nine "instead" lines pair the word with "would"
   and are the interception; four hundred and ten write no "would"
   anywhere and are [CR#614.15]'s self-replacement — "an effect of a
   resolving spell or ability that replace part or all of that spell or
   ability's own effect(s)", with the rule adding that the text
   creating one "is usually part of the ability whose effect is being
   replaced", which is exactly why English writes it as the NEXT
   SENTENCE. So `InsteadOf` pairs a clause with the clause it replaces,
   and the pairing is the node: "instead" means nothing without the
   instruction it stands in for, the way "Otherwise" means nothing
   without its "if" (finding 88). The condition that almost always
   governs it is the replacement clause's own `If` and not a slot here
   — "Draw a card. If you control three or more artifacts, draw two
   cards instead" is a conditional clause wearing an "instead", not an
   "instead" wearing a condition — which is what lets the ordinary
   conditional machinery carry it unchanged (`tezzeretDrawTwo`,
   `zimoneDrawTwo`).
146. **The announcement channel is what makes "that artifact" work,
   and it was one equation.** Chapter twenty-one closed a leak by
   making a conditional clause a HOLE — the condition may have been
   false, so the clause never ran and its phrase named nobody — and
   named the cost in the same breath: Overload's second sentence reads
   the first sentence's announced target across an `If`, and the
   workbench could not spell it. The fix is that the cost was charged
   to the wrong function. `effIntro` is what a clause DID and stays a
   hole; `preIntro` is what its phrases ANNOUNCED, and [CR#601.2c]
   chooses targets as the spell is cast, whatever clause spells them
   and whatever any condition later says. So `preIntro (If e c oth)`
   exports `preIntro e` now, and the two functions differ on this row
   the way chapter twenty-one's own distinction says they should. The
   consumer is the node this round minted: `InsteadOf` types its
   replacement in `preIntro replaced`, which is [CR#614.6] and
   [CR#601.2c] read together — the replaced EVENT never happened, so
   its outcome is unreadable (`badInsteadReadsReplacedOutcome`), while
   its TARGET was announced and is there to say "that artifact" of.
   What the channel does NOT yet buy is a whole card, and the reason is
   worth recording precisely: every corpus line that exercises the
   pairing needs a condition from another axis — Overload and Prohibit
   want kicker, Fatal Push wants revolt, Anoint with Affliction wants
   poison counters, Welcome to the Fold wants madness, Ana Sanctuary
   wants a permanent-word conjunction. The channel is open, its
   consumer is built, and its witnesses are ledgered by name.
147. **The entry riders are STATIC ABILITIES, which is the third
   time this round the answer was "the container is missing".**
   [CR#603.6d] is explicit — text reading "[This permanent] enters with
   …", "As [this permanent] enters …", "[This permanent] enters as …",
   or "[This permanent] enters tapped" is "a static ability — not a
   triggered ability — whose effect occurs as part of the event that
   puts the permanent onto the battlefield" — and the replacement
   chapter files the same text as a replacement effect, the "enters
   with" and "enters as" wordings at [CR#614.1c] and the bare "enters
   tapped" at [CR#614.1d]. So chapter nineteen was right to
   send them to the replacement axis and this axis has to send them
   back to the abilities layer: the SENTENCE is a static ability line,
   the same refusal `badStaticCant` makes and the same one both shields
   make. Measured before ledgering, because the counts are what a later
   round will need: "enters with … counters on it" three hundred
   eighty-six lines of the four hundred fourteen "enters with";
   "enters tapped" a hundred thirty-two, and only three of those are a
   bare one-sentence ability, the rest carrying an "unless" clause, a
   counter rider, or a second conjunct. What IS writable today is the
   ONE-SHOT twin, and it is a rider on a move rather than a replacement
   at all: "put [card] onto the battlefield tapped" (three hundred
   fifteen lines), "tapped and attacking" (nineteen — chapter
   nineteen's own `ridersOk` trio at a second site), and the control
   assignment "onto the battlefield under [whose] control" (a hundred
   thirty-two), which is the ledger's existing control-override entry
   and not this round's. Those three are one field on `Move` and
   forty-three call sites, so they are ledgered with their counts
   rather than taken here. And "enters under [whose] control" is
   attested but tiny and elsewhere-blocked: nine lines, four of them
   "under the control of an opponent of your choice", two the bare
   "under your control", one the replacement "it enters under your
   control instead" that wants the enter event this round measured and
   did not mint.
148. **The branch frontier's own card is answered in SHAPE and
   still not writable, and saying which is the point.** The
   representability frontier has carried Galvanic Blast's "instead if"
   since chapter eighteen as the open question of whether the branch is
   a replacement or an else-arm. It is a replacement, and `InsteadOf`
   is its node: "Galvanic Blast deals 2 damage to any target. Metalcraft
   — Galvanic Blast deals 4 damage instead if you control three or more
   artifacts" is [CR#614.15]'s self-replacement with the ordinary
   trailing condition, and that condition is the one `tezzeretDrawTwo`
   already writes. What still blocks the CARD is neither the branch nor
   the condition: the replacement clause writes no recipient at all
   ("deals 4 damage instead"), an ellipsis that inherits the replaced
   clause's complement, and this grammar has no way to write a damage
   clause with an unstated patient. The ability word is the smaller
   half ([CR#207.2c] gives it no rules meaning), and the ellipsis is the
   real one — a new entry rather than an old one, and a cousin of the
   coordination and extraposition gaps the ledger already keeps.

## Chapter Twenty-Six — The Announcement Channel

Chapter twenty-six, where the function three constructions had been
borrowing turns out to be two functions (evidence: Death by Dragons,
Terrifying Presence, Locke Treasure Hunter, Sylvan Scrying, Impulse,
Phyrexian Infiltrator, Colossal Growth, Volatile Stormdrake, Sudden
Substitution; and, for what the round measured rather than built,
Calix Destiny's Hand, Basilica Guards, Secret Invasion, Gaea's Liege,
Graceful Antelope, Rampant Growth, Cloudshift, Scarwood Treefolk).

149. **The announcement channel is its own function, and `preIntro`
   was never it.** Chapter twenty-one drew the pre/post distinction and
   chapter twenty-five opened the conditional's announcement export on
   it, and both were right about the CLAUSE they were looking at and
   wrong about the function they reached for. `preIntro` is the
   pre-resolution twin of `effIntro` — what the discourse holds one
   moment before the clause resolves — and on a FLAT clause that is the
   same list as its announcements, which is why three constructions
   could type their announcement-only slots on it and look correct for
   four chapters. They come apart wherever a clause CONTAINS another
   clause, and the round found all three directions and proved each with
   a term that typechecked. A `May`'s row is `mayIntro`, the optional
   body's DEED, so a simultaneous sibling could put a counter on a token
   the player may never have made: [CR#118.12] makes the offer a cost
   checked by whether the player chose to pay it, "regardless of what
   events actually occurred", and [CR#608.2f] processes a batch's actions
   at once, so nothing in the batch's one pre-state can be that token
   (`badSimultaneousReadsMayDeed`, `badSimultaneousReadsMayOutcome`). A
   `Sequentially`'s row is its last clause's pre-state over the DEED
   telescope, so an earlier step's outcome walked into a replacement for
   an event [CR#614.6] says never happened — and the ATOMIC form of the
   same read was already refused, which is what proved the wrapper opened
   it rather than the row (`badInsteadReadsReplacedSequenceOutcome`
   against `badInsteadReadsReplacedOutcome`). And `If` forwarded either
   defect recursively (`badConditionalInsteadReadsMayOutcome`). So the
   repair is a distinct STRUCTURAL traversal, `annIntro`: what a clause's
   phrases have named by the time the spell is cast ([CR#601.2c]) and
   nothing about what it did. `SimEffects` threads it, `preIntro (If …)`
   exports it, and `InsteadOf` and `HeldUntil` type their trailing halves
   in it. `preIntro` keeps the job it was built for — the trailing
   condition's context, which is a pre-state question and not an
   announcement one.
150. **A sequence is a HOLE in that channel, and the hole is a
   consequence rather than a hedge.** `annIntro (Sequentially es) = bs`,
   and the reason is the telescope: `Effects` types each element over its
   predecessors' `effIntro`, so no later element's announcement can be
   lifted out without carrying the deeds it was typed in. A BATCH is not
   a hole, and the difference is exactly that its telescope is this
   function's — which is what makes announcements accumulate across a
   batch while deeds do not, in one line of one type. The same shape
   answers the `May` arms, which contribute nothing here: an arm is typed
   over the body's `effIntro`, so `annIntro (May …)` takes the BODY's
   phrases and stops. Under-reporting, and in the direction that refuses.
151. **The batch's outward discourse is the UNION, and chapter
   twenty-two's recorded simplification was one.** That chapter folded
   the LAST element's `effIntro` over the announcement telescope and
   wrote down that this "IS the union for every row the container reaches
   today" — true then, and the case that shows it was a simplification is
   two deeds of one kind: two creates in one instruction leave two
   tokens, both actions having been processed ([CR#608.2f]), so the
   sentence after cannot say "it" and must refuse as ambiguous
   (`badBatchTwoCreatesThenIt`, `badBatchTwoOutcomesThenThatMuch`). The
   fold takes DELTAS (`deedDelta`) rather than whole answers, and that is
   the interesting half: the telescope has already carried every
   element's announced phrases, so folding `effIntro`s whole would count
   them twice and a doubled mention breaks every uniqueness gate a read
   makes. So each row states what its DEED adds that its PHRASE did not —
   an outcome, a created token, a found or milled group — and the rows
   that RETAG (the zone-writing three) or REMOVE (a shuffle) answer with
   nothing, having no delta to give. That last is chapter twenty-two's
   own note kept and made uniform: no corpus line writes a batch whose
   element moves an object, and the reads INSIDE such a batch were
   already refused (`badSimultaneousReadsRetag`). The exchange family is
   unmoved, which is the check that mattered — a control grant introduces
   its phrase and does nothing else, so union and last-element agree on
   it exactly.
152. **A search takes the ZONE and never a place in it, and the sort
   could not tell the difference.** `zoneSort` projects `Library` off
   "the top of your library" as readily as off "your library", so gating
   the search on the sort alone admitted "search the top of your library"
   — arrangement rider and all, which makes the nonsense plain, a search
   cannot look through cards "in a random order". [CR#701.23a] defines
   searching as looking at "all cards in that zone", and [CR#401.2] is
   what lets the two phrases be told apart at all: a library is "a single
   face-down pile", so its positions are places in a zone and not zones.
   The clause asks the PHRASE now as well as the sort (`WholeZone`,
   `badSearchLibraryPosition`), which is the same two-questions-one-gate
   correction the complement anchor gets below.
153. **A distributed mill's NUMBER is the clause's, which is the
   subject and the count together.** [CR#701.17a] has each milled-at
   player put that many cards from their own library into their own
   graveyard, so "each player mills a card" puts one card per player
   there and the group is plural though the count says one. The row was
   keyed on the count alone, which made the mention singular and let "it"
   through; it is `outputPlur (nounPlur who) (amtPlur amt)` now, the
   derivation `Create` has used since chapter twenty-three, and the
   singular read is refused (`badDistributedMillSingular`). Locke,
   Treasure Hunter is the line — "each player mills a card. If a land
   card was milled this way, create a Treasure token. Until end of turn,
   you may cast a spell from among those cards" — and it lands no
   positive here, its plural read being the among-restriction the ledger
   already keeps and its shell a trigger. The fix arrives with its
   refusal pinned and its witness waiting, which is the honest order.
154. **A complement's anchor is a READ or a written TARGET, and
   "valid anchor" was never the same question as "introduces no
   binding".** Chapter twenty-two gave `OtherThan` the blanket
   bindingless demand `Matches` makes of its subject, on the reading that
   an anchor with a determiner would announce a referent the sentence
   never spelled. That is exactly right for the indefinite — "other than
   a creature" is unwritten English and stays refused
   (`badComplementAnchorAnnounces`) — and exactly wrong for a target,
   which spells its referent out loud. The corpus writes two such lines
   and the demand refused both: "Each player other than target player
   creates a 5/5 red Dragon creature token with flying" (Death by
   Dragons) and "Prevent all combat damage that would be dealt by
   creatures other than target creature this turn" (Terrifying Presence),
   and `other than target` returns exactly those two in supported and
   all-cards scope alike. [CR#601.2c] announces those targets as the
   spell is cast like any other, so the announcement now travels with the
   phrase the complement modifies (`predDelta (OtherThan n)`). Death by
   Dragons lands WHOLE on the distributed-create machinery chapter
   twenty-three built; Terrifying Presence lands as its phrase, the rest
   of its sentence wanting the source-restricted shield chapter
   twenty-five ledgered. The second half of the correction is the one
   that keeps the deferred family deferred: the anchor must be SINGULAR.
   The constructor subtracts one referent, and a plural anchor passed to
   it is group subtraction — finding 111's subset complement — which the
   loosened shape would otherwise have admitted silently through "those"
   (`badPluralComplementAnchor`). No corpus line writes one: `other than
   them`, `other than those`, and `other than these` return zero in both
   scopes.
155. **A partition's remainder is spent when it is placed, and the
   state was already on the binding list.** "The rest" names what is
   OUTSTANDING of a group, so once the rest has been placed there is
   nothing outstanding and a second "the rest" in the same breath names
   nothing — and nothing in the phrase's presupposition said so, which
   let one look be partitioned once and disposed of twice. The fix is one
   word in the move's contribution: disposing of the remainder SPENDS the
   group mention (`groupSpent`), which makes `theRestOk` false for every
   later clause on the same path and leaves the branch arms alone, an arm
   being typed in the discourse BEFORE the disposition. That is exactly
   what the corpus asks for — Impulse writes one partition and one
   disposition, and the only three lines carrying two "the rest" phrases
   put them in mutually exclusive if/instead/otherwise arms
   (`badRestDisposedTwice`). It costs nothing already written: all four
   partition positives dispose of the remainder in their last clause, and
   the group-read-after-disposal hole was chapter twenty-four's own note.
156. **The one leak reported against the control grant was not one,
   and the dependency is why.** An outside reading of
   `staticIntro (GainsControl who what) = nomIntro what` said the gaining
   player is dropped, so that "target player gains control …" could not
   be read back by a later sentence or by the clause's own duration. The
   right-hand side is only half the row: `what` is typed in
   `nomIntro who`, so the player's announcement is carried THROUGH the
   dependency and both reads typecheck today. Probed both ways before
   refusing — a following sentence reads the player, and so does a
   `ForAsLongAs` on the grant itself — and the reading is what
   Phyrexian Infiltrator's batch has depended on since chapter
   twenty-two, its second half finding the first half's target through
   exactly this path. Refused with the mechanism named rather than
   with a measurement, because the measurement points the other way:
   the family IS attested, four lines reading the gaining player back
   ("Target opponent gains control of another target permanent you
   control. If they do, you draw a card"), and every one of them reads
   it inside the mandatory "if/when [a player] does" arm [CR#118.12]
   governs, which waits with the cost algebra.
157. **What the round measured and returned.** The entry-rider FIELD
   is confirmed in shape by a second reading and stays ledgered with its
   execution: [CR#603.6d] calls "[This permanent] enters tapped" a static
   ability in as many words and [CR#614.1c,614.1d] file the text as a
   replacement effect, so the sentence belongs to the abilities layer,
   while the one-shot twin (Rampant Growth's "put that card onto the
   battlefield tapped") is a rider on the move; the settled shape is a
   closed rider BUNDLE on `Move` rather than a scalar, gated to
   battlefield destinations, with the controller as an optional override
   over [CR#110.2a]'s default. Chapter twenty-five's ninety-one-line
   "until [object] leaves the battlefield" split is independently
   confirmed at 86 one-shot exile/undo lines ([CR#610.3]), three phasing
   lines ([CR#610.4]), and three continuous ones, the totals overlapping
   once because Secret Invasion writes both families in one line; the
   three continuous ones are Secret Invasion's copy clause, Gaea's Liege
   and Graceful Antelope's base-type settings, none writable by the
   landed continuous rows. Two readings of landed rows were checked and
   both came back legal rather than leaky: Calix, Destiny's Hand's
   "Exile target creature or enchantment you don't control until target
   enchantment you control leaves the battlefield" is a `HeldUntil` whose
   ENDPOINT is a second announced target, which the rider's own typing
   permits and no rule forbids; and Basilica Guards' "each opponent loses
   1 life and you gain that much life" is not a simultaneous batch that
   would have to read a sibling's outcome, [CR#608.2c] following
   instructions "in the order written", so it is a `Sequentially` and the
   outcome read is the ordinary one. Last, an existential event DURATION
   ("until a creature dies") was raised and is unattested: zero lines in
   either scope for that phrasing and zero for any "until … dies", the
   nearest real forms being the iterated reveal ("reveals cards from the
   top of their library until a creature card is revealed"), which is a
   repetition construction and not a duration at all, and one
   monarch-endpoint exile. Dropped rather than ledgered.

## Chapter Twenty-Seven — The Cost Algebra

Chapter twenty-seven, the first thing above a clause (evidence:
Immersturm Skullcairn, Merrow Grimeblotter, Phyrexian Snowcrusher,
Havoc Sower, Erebos God of the Dead, Savageborn Hydra, Basking
Rootwalla, Bonders' Enclave, Security Detail, Woeleecher, Molting
Harpy, Carnophage, Solitary Confinement; and, for what the round
measured rather than built, Solphim Mayhem Dominus, Stiltzkin Moogle
Merchant, Yes Man, Bill Ferny, Sacred Mesa. The round the colon
stopped accepting anything):

158. **The mana symbols are a port, and the hybrid families are ONE
   row because the rules build them that way.** The explicit ruling
   that carried `Color` in chapter nineteen carries the symbol side
   here, so `SimpleManaSymbol`/`ManaSymbol`/`ManaCost` are core's
   (`deckmaste_core/src/mana.rs`) row for row, and the settled Idris
   spec agrees with core row for row as well — three independent
   spellings of one closed catalog. What the port BUYS is visible in
   the shape rather than the count: `Hybrid (SimpleManaSymbol) Color`
   is one constructor covering three printed families, the two-color
   `{W/U}`, the monocolored `{2/B}`, and the colorless-hybrid `{C/W}`,
   because [CR#107.4e] says a hybrid symbol is a colored symbol "even
   if one of its components is colorless" and the left half is exactly
   the component that varies. `Phyrexian` takes a `Color` and not a
   `SimpleManaSymbol` on its left, which is the type carrying its own
   well-formedness off [CR#107.4f]'s list: the fifteen printed
   Phyrexian symbols are five colored and ten hybrid-colored, and
   there is no `{2/P}` or `{C/P}` to write. The one place the port is
   deliberately looser than the CR is where core is, an unprinted
   `{5/W}` being representable on purpose, and that is inherited
   rather than re-litigated. `ColorOrColorless` lands here too, the
   piece chapter nineteen named and skipped, and it lands where core
   puts it — beside `Color`, not beside the symbols — because the
   colorless pip is a fact about mana and not about the symbol
   grammar. The bench witnesses the common rows and three of the
   exotic ones: the hybrid and the untap symbol together on Merrow
   Grimeblotter, the snow symbol on Phyrexian Snowcrusher, the
   colorless pip on Havoc Sower. The PHYREXIAN row stands on the
   ruling alone, and the reason is measured rather than assumed: the
   corpus writes thirty-nine Phyrexian components in activation costs
   and not one of their abilities is writable here — twenty-odd are
   "Transform this creature", and the rest want an indestructible
   counter, a counted untargeted group, or a cast-from-exile
   permission. Solphim, Mayhem Dominus is the named real spelling.
159. **A cost is not an Effect, and what says so is a per-VERB table.**
   The colon has accepted any clause at all since it was minted, which
   is the ledger's own cost-GRAMMAR entry, and the repair is not a
   blanket rule but a measurement: [CR#602.1a] makes a cost what the
   ACTIVATOR pays, so the question is which verbs oracle actually
   writes before a colon. Counted over the components of symbol-led
   activation costs — an undercount, since an action-only cost leads
   its own line — sacrifice runs to eleven hundred eighty-four,
   discard two hundred twenty, remove-counters two hundred twelve,
   exile a hundred ninety-one, pay-life ninety-five, tap forty-four,
   return twenty-nine, put-counters fourteen, reveal seven, mill five.
   The refusals are the same measurement coming back zero: no line
   writes a destruction, a draw, a shuffle or a search as a cost
   (`badDrawAsCost`, `badDestroyAsCost`). The sharp case is the
   composite, where the refusal has to key on the TAG and not on the
   move underneath it — sacrifice and exile are cost verbs and destroy
   is not, and all three are a `Move` to a zone. Core's
   `Action::is_cost_eligible` lists the same verbs and reaches them
   from the engine side, which is the agreement worth having. The LIFE
   row is directional and that is measured too: ninety-five "Pay N
   life" components against zero gain-life ones (`badGainLifeCost`).
   And the ledger's own note about this cell needs one correction. It
   said "you lose N life" is not a payment, citing [CR#119.4]; the
   rule says the opposite in as many words — "if a player pays life,
   the payment is subtracted from their life total; in other words,
   the player loses that much life" — so the two are ONE clause and
   the distinction is the FRAME it stands in ([CR#118.1]: a cost is an
   action "necessary to take another action"). `Do (ChangeLife who
   (Down n))` spells "Pay 2 life" before a colon and "you lose 2 life"
   in a sentence, which is what the spelling comment now says.
160. **The tap symbol is not the tap clause, and the corpus writes
   both.** [CR#107.5] gives "{T}" a fixed meaning in an activation
   cost — "tap this permanent" — with no noun phrase in it at all,
   where "Tap an untapped creature you control" is an ordinary clause
   with a described patient. The counts are three thousand two hundred
   fifty-nine symbol components against a hundred seventy-one English
   tap clauses, and the untap symbol "{Q}" is eighteen more. So `Cost`
   carries `TapSymbol` and `UntapSymbol` as rows beside `Do`, which is
   core's split exactly (`CostComponent::{Tap, Untap}`), and it is a
   recorded DIVERGENCE from the settled Idris spec, which normalizes
   "{T}" to `Do (Tap This)`. The spec is not wrong about the game; it
   is making a semantic identification this grammar cannot make,
   because the two are different things on the page. The untap symbol
   earns its own row rather than a negation for a second reason worth
   keeping: [CR#602.5a] names "{T}" and "{Q}" TOGETHER as the pair a
   summoning-sick creature cannot pay, which is what core's
   `CostPredicate::IncludesTapSymbol` asks about, so a grammar that
   spelled one as the other's negation would have to unspell it there.
161. **A cost's components thread ANNOUNCEMENTS and not payments, and
   the rule is what fixes that.** Oracle joins components with commas
   ("{1}{B}{R}{R}, {T}, Sacrifice this land:") and the effect after the
   colon reads what they named, so the sequence has to accumulate
   something. What it must NOT accumulate is settled by [CR#601.2h]:
   the player pays the components "in any order", so no component can
   depend on a sibling having been paid, and no corpus line writes one
   that does. `CostSeq` is therefore the announcement telescope
   `SimEffects` is, not the deed telescope `Effects` is — the same one
   line of difference between the two containers chapter twenty-six
   drew, arrived at from a rule instead of from a leak. The compound
   inherits the two refusals its siblings carry: an element is a
   component and never a compound (`badNestedCompound` — core reaches
   the flat form by normalizing instead, `Cost::normalize`), and a
   compound of one is the component spelled twice
   (`badSingletonCompound`).
162. **"Pay" is one English VERB where a cost is everything an ability
   charges, and the two sets are not the same.** The unless family
   needed a payment CLAUSE — [CR#118.12a] rewrites "[Do something]
   unless [a player does something else]" into "[A player may do
   something else]. If [that player doesn't], [do something]", so the
   may's body has to be able to say "pays {3}" — and the obvious move,
   letting the verb take a whole `Cost`, is half right. Its complement
   is measured where the verb appears most: of the unless-payment
   complements, a hundred seventy-eight are one symbol run and
   twenty-four are "N life", and every other payment after "unless"
   writes its OWN verb — "unless you sacrifice a land", "unless you
   discard a card", seventy-one lines across four verbs — and never
   "pay". So a sacrifice is a payment and is not payABLE
   (`badPayBySacrificing`), the tap symbol still more sharply, being a
   cost that exists only before a colon (`badPayTapSymbol`), and the
   compound is unattested under the verb (`badPayCompound`). The row
   itself is core's `Action::Pay(Cost)`: one `Cost` value standing in
   two frames, paid to activate in one and resolving as a sentence in
   the other.
163. **The ability container is the granted-ability type GROWN, and it
   is unindexed.** `Ability` has existed here since chapter twelve as
   one row, `KeywordAbility Keyword`, with a doc comment calling
   itself the granted-ability vocabulary "minimally". Growing that
   into the container rather than minting a second type is what
   [CR#113.3] licenses — the four categories are one category of
   thing — and it is what core and the settled spec both do
   (`Ability::{Static, Activated, Triggered, Spell, Keyword}`). It is
   UNINDEXED, and that is a claim rather than a convenience: an
   ability line is context-closed, its cost being the first thing on
   the line with no discourse before it. Core is unindexed too; the
   spec indexes its `Ability` so a keyword DESUGARING can carry an
   anaphor, and this file has no keyword desugaring. The widening
   costs exactly one refusal, and paying it is what keeps the
   container honest: English grants an activated ability by QUOTING
   it — "Enchanted land has \"{T}: Add {B}\"" (twenty-five lines), the
   equipped and all-Slivers twins (ten and seven), the token
   with-clause form (seven) — and this grammar has no quotation, so
   the grant site says no (`Grantable`, `badGainsActivated`). The
   token's with-clause slot is narrowed to `Keyword` for the same
   reason and refuses the container by type
   (`badTokenActivatedAbility`).
164. **Three activation restrictions are three SLOTS because oracle
   conjoins them, and the guard is typed before the cost.** A thousand
   lines write "Activate only …" and they divide into three families
   that co-occur: the window ("as a sorcery", five hundred
   twenty-three; "as an instant", five), the use limit ("only once
   each turn", eighty-six; "only once", eight), and the state guard
   ("only if …", two hundred thirty-seven). The conjunction is what
   settles the shape — "Activate only during your upkeep and only once
   each turn", "Activate only if you control ten or more permanents
   and only as a sorcery" — because one restriction row would have had
   to spell a conjunction of unlike things. Security Detail is the
   bench's whole-card witness at two of the three. The GUARD reuses
   `Condition` verbatim, which is chapter eighteen's seam paying off
   exactly as that chapter predicted, and it is typed at the EMPTY
   context rather than in the cost's survivors — [CR#602.5]'s own
   placement, since a restriction on use is checked before the ability
   is activated at all where the cost is not paid until [CR#601.2h].
   Nothing the cost names can be read by the condition deciding
   whether the cost may be paid. That is also chapter eighteen's
   negation finding finding its carrier: "Activate only if you control
   no creatures" is three lines and Security Detail is one of them.
165. **The mandatory "if you do" is the may node with its OFFER
   emptied, and one rule is why.** [CR#118.12] states both shapes in a
   single sentence — "[Do something]. If [a player] [does, doesn't, or
   can't], [effect]." Or "[A player] may [do something]. …" — and
   gives them ONE reader, checking "whether the player chose to pay an
   optional cost OR STARTED TO PAY A MANDATORY COST, regardless of
   what events actually occurred". Every consequence chapter eighteen
   drew for the offered form therefore carries over untouched: the arm
   asymmetry, the opacity of the join, the deed that may never have
   happened — the rule's own Standstill example is a MANDATORY
   sacrifice that could not be paid and whose arm did not run. So the
   offer is the only thing that varies, and it is the only thing that
   varies: `May` takes a `Maybe` decider now, `Nothing` being the bare
   instruction whose "if you do" takes its pronoun from the BODY's own
   agent because the sentence never wrote a decider. A second row
   would have duplicated `mayIntro`, `annIntro`, `deedDelta` and the
   refusals with them, and the check that it would have been a
   duplicate is that the declined arm's refusal reproduces itself
   without a line of new machinery (`badIfNotReadsMandatoryBody`
   beside `badIfNotReadsMayBody`). Woeleecher is the positive, and it
   is a whole activated ability: "{W}, {T}: Remove a -1/-1 counter
   from target creature. If you do, you gain 2 life."
166. **The unless family lands, and the wall it hits is a
   LINEARIZATION wall.** Six hundred ninety-six lines write "unless",
   and the round can now say which of them the machinery reaches.
   [CR#118.12a]'s rewrite needs no new structure at all, exactly as
   chapter eighteen recorded — the sentence is `May` with an `ifNot`
   arm, the payment in the body and the main clause in the arm — so
   what decides writability is who the PAYER is. Two hundred
   forty-one lines pay in the second person ("unless you pay",
   a hundred forty-five; "unless you sacrifice/discard/exile/tap" and
   their neighbours, ninety-six), and those land: Molting Harpy,
   Carnophage and Solitary Confinement are the three frames. A
   hundred twenty-five write "unless its/their controller pays",
   forty-three "unless that player pays" — SPLIT in chapter thirty-two
   into ten whose payer is the main clause's own subject ("Whenever a
   player casts a spell, counter it unless that player pays {3}") and
   thirty-three reached by a RELATION, thirty of those being the ward
   frame's controller-of-the-countered-spell, so it is the relational
   thirty-three and not the whole forty-three that this wall holds — and
   nine "unless any player
   pays", and those do NOT, for a reason the node makes visible: the
   rules' rewrite is not linearization-preserving. It puts the may
   first, so the payer phrase is typed before the clause that
   announces what it reads, and "its controller" has nothing to reach
   (`badUnlessAnaphoricPayer`). That is a sharper statement than the
   stack blocker those lines mostly also carry, and it is the one that
   would survive the stack arriving. The STATE half is re-checked and
   unmoved: "unless you control a legendary creature" is a negated
   `Condition` whose carriers are entering-tapped replacements and
   durationless static deontics (sixty-two lines and a hundred eleven
   "can't … unless"), and the cost vocabulary does nothing for either
   — they want the abilities layer, which is the next round's.
167. **What the round retro-opened, and what it did not.** Claimed:
   the cost GRAMMAR entry, whose repair is finding 159's table; the
   MANDATORY "if you do" entry, finding 165; the action half of the
   "unless" entry, finding 166; and chapter eighteen's activation
   restriction, which its own finding 76 promised would reuse
   `Condition` verbatim and does. Re-checked and still shut, each with
   its blocker relocated rather than repeated: finding 156's four
   control-grant read-backs are no longer waiting on the [CR#118.12]
   arm, which now exists, and are waiting on four different things —
   Stiltzkin, Moogle Merchant on the PERMANENT word ("another target
   permanent you control"), Bill Ferny on a subtype head plus a
   remove-from-combat verb, Yes Man on a reflexive trigger
   ("When they do") plus the turn-part window, and Kain on a trigger
   shell plus the "that many"/"that much" reads. Checked and NOT
   opened: the `InsteadOf` whole-card exercisers and the conditional
   headcount upgrade, whose ledgered blockers are kicker, revolt,
   madness, poison counters, the commander and monarch reads and a
   conjunction `Condition` has no frame for — kicker is the only one
   the cost algebra touches at all, and it wants the declared OPTIONAL
   cost with its tag and its three read channels, not the cost type.
   The event-query entry's own wording is updated instead of claimed:
   Slaughter Pact waited on "pay, tokens, and an unknown-zone retag",
   and pay has landed, leaving the delayed upkeep query and a
   lose-the-game clause.
168. **What the round measured and returned.** The MANA ability is an
   activated ability and nothing else structurally — [CR#605.1a] makes
   it one by four criteria, none of them about its shape — so it
   belongs in this container and cannot be written here for want of a
   production clause, three hundred thirty-two lines spelling the bare
   "{T}: Add …" alone. The LOYALTY cost is eight hundred thirty-two
   components and a symbol vocabulary of its own (the bracketed
   "[+1]"/"[−7]"), which is why core's third use-limit row
   (`LoyaltyOncePerTurn`, [CR#606.3]'s cap shared across a
   planeswalker's abilities) waits with it rather than with the two
   built here. The turn-part WINDOW is about a hundred twenty lines
   ("during your upkeep", thirty-two; "during your turn", forty-one;
   "during your turn, before attackers are declared", nineteen) and is
   nearly writable — `TurnPart` and `Whose` are chapter seventeen's —
   with two lines naming the exact gap: "Activate only during any
   upkeep step" writes a possessor `Whose` has no word for. The
   cost-language reads are seventy-four lines and one row apiece:
   "unless [someone] pays its echo cost" and the upkeep-cost twin want
   a keyword's own cost, and "pays its mana cost" is core's
   `ManaCostOf` over a reference. And the whole total-cost pipeline is
   named and left where it was: [CR#601.2f]'s increases and reductions
   (core's `CostChange`), [CR#118.9]'s alternative-cost base swap, and
   [CR#118.8b]'s declared optional costs with their tags — engine
   traffic and keyword packaging, not sentence grammar.

## Chapter Twenty-Eight — The Ability Line

Chapter twenty-eight, where the container gets its two remaining rows
and the grammar learns to write a sentence that is not an instruction
(evidence: Cloudkin Seer, Moonlit Wake, Promise of Tomorrow, Staff of
Nin, Library Larcenist, Elite Javelineer, Jhessian Thief, Scholar of
Stars, Glacial Chasm, Misery's Shadow, Jor Kadeen the Prevailer,
Abandoned Outpost, Yasmin Khan, Brazen Cannonade; and, for what the
round measured rather than built, Locke Treasure Hunter, Ana Sanctuary,
Krovikan Vampire, Crucible of Worlds, Skaab Ruinator, Heart-Piercer
Manticore):

169. **A static ability is a STATEMENT, and English marks it by
   ASPECT.** [CR#604.1] says what the line is in words the grammar can
   use — "static abilities do something all the time rather than being
   activated or triggered. They are written as statements, and they're
   simply true" — and [CR#611.3b] says why it states no duration: the
   effect "applies at all times that the permanent generating it is on
   the battlefield or the object generating it is in the appropriate
   zone". So `Static` takes a `StaticEffect` and nothing else, and the
   families three chapters refused as effect-sentences move in with no
   new vocabulary at all: Glacial Chasm's "Creatures you control can't
   attack" is `badStaticCant`'s own sentence, its "Prevent all damage
   that would be dealt to you" is `badStandingPrevention`'s, and Misery's
   Shadow's "If a creature an opponent controls would die, exile it
   instead" is `badStandingIntercept`'s. What the round had to discover
   before the row would hold was that the line and the clause are not the
   same sentence twice. The clause is an instruction and takes the
   INCHOATIVE verb; the line is a statement and takes the STATIVE one.
   "Target creature gains flying until end of turn" against "Creatures
   you control have haste" — six hundred fourteen lines of the second;
   "Target land becomes an Island in addition to its other types" against
   "Creatures you control are the chosen type in addition to their other
   types" — a hundred fifty-eight against seventy-two. The stat delta and
   the deed restriction write one word in both frames ("get", "can't")
   because English has no separate inchoative for either, which is why
   the split stayed invisible until a construction needed both moods of
   one row. `staticAsAbility` is the table, full rows, and it has exactly
   one refusal: the CONTROL grant, whose stative form is a different verb
   altogether. English states the standing fact as "You control enchanted
   creature" (seven lines) and writes a durationless "gains control of"
   zero times, so the line is a construction that row does not spell
   rather than an inflection of it (`badStaticGainsControl`).
170. **A static ability cannot TARGET, and the rule enumerates who
   can.** [CR#115.1a..115.1e] list the things that are targeted — an
   instant or sorcery spell, an activated ability, a triggered ability,
   and the keyword abilities that represent one of those — and a static
   ability is not on the list. [CR#115.1b] says the nearest case outright:
   "an Aura permanent doesn't target anything; only the spell is
   targeted." That is the sharpest single difference between the three
   ability rows this file now holds, because the other two DO target
   ([CR#115.1c] for the activated, [CR#115.1d] for the triggered), and
   Elite Javelineer's "Whenever this creature blocks, it deals 1 damage
   to target attacking creature" is the trigger exercising it on the
   bench. So the line demands a target-free statement (`Untargeting`,
   `badStaticTargets`), and the demand is asked of the discourse the
   statement ANNOUNCES rather than of its constructors — one kind-blind
   scan of `staticIntro`, which is `anyTargeted` with the kind index
   dropped, because the question the rule asks is not which kind was
   targeted but whether anything was.
171. **The event vocabulary has FOUR readers now, and the merge the
   ledger has carried since chapter twenty-five is taken.** `EventQuery`
   is gone. Its two rows were an end-step beginning and a death, both of
   which `GameEvent` spells, and keeping two vocabularies for one set of
   happenings was the thing the ledger said the trigger container would
   settle. What the merge exposed is that the readers disagree about the
   TENSE as well as about the event. The interception's replacement reads
   what the event's phrases announced, because [CR#614.6] says the
   replaced event never happens; the trigger's effect reads what the
   event LEFT BEHIND, because [CR#603.6] has a zone-change trigger "look
   for the object in the zone that it moved to". So `eventIntro` gained a
   twin, `eventAfter`, and the difference is a whole card: "Whenever a
   creature you control dies, exile it" (Promise of Tomorrow) finds a
   card in a graveyard, while "If a creature would die, exile it instead"
   finds a permanent on the battlefield, and tapping the trigger's
   referent is the dead-referent refusal (`badTriggerTapsDeadCreature`).
   Only the two zone-change events retag, which is [CR#603.6]'s own pair;
   the DEPARTURE deliberately does not, because [CR#603.6c] has a
   leaves-the-battlefield ability check "only in the first zone that it
   went to" and the sentence never says which zone that is — a retag the
   grammar cannot compute is a silence, not a guess. The old delayed
   context is one line now (`delayedCtx ev = settleTargets (eventAfter
   ev)`), the retag it used to spell per query having become everybody's.
172. **The trigger's opening word is a SLOT, and exactly one third of
   it is derivable.** [CR#603.1] writes the shape with the three words in
   a bracket and [CR#113.3c] lists them together as words a triggered
   ability "include(s) (and usually begin(s) with)"; no rule anywhere
   assigns one. What the corpus assigns absolutely is `At`, and
   [CR#603.2b] is why — "when a phase or step begins, all abilities that
   trigger 'at the beginning of' that phase or step trigger" — with one
   thousand six hundred seventy-eight headers opening that way and every
   one of them a turn part, and no object event taking the word at all
   (`badAtEnters`, `badWhenUpkeep`). The When/Whenever split is measured
   and NOT gated, and saying why is the finding. The word tracks
   REPEATABILITY, which is finding 142's own reasoning arriving at a
   second construction: a once-per-object event with a fixed subject
   takes "When" ("When this [permanent] enters" two thousand four hundred
   seventy-three against eighty-five, "When this [permanent] dies" three
   hundred eighty-two against one), the same event over a description
   takes "Whenever" because the description ranges over many objects
   (four hundred eighteen against twelve, two hundred eighty-five against
   six), and an event that repeats for one object takes "Whenever" even
   with the fixed subject ("Whenever this creature attacks" six hundred
   ten against nine). The residue is what keeps it out of the type. Every
   counterexample on the "When" side is an ability that destroys its own
   source ("When a creature enters, sacrifice this artifact and create X
   … tokens"), so the word is answering a question about the EFFECT and
   not about the event; and the eighty-five "Whenever this [permanent]
   enters" lines are compound headers ("enters or attacks") this grammar
   has no coordination for. A gate keyed on the event alone would have to
   refuse real oracle in one direction or the other, so it refuses only
   what a rule settles.
173. **The turn-part header is the shared endpoint vocabulary, and
   chapter seventeen said so before it existed.** That chapter's own doc
   wrote that `TurnPart` "is the SHARED vocabulary — the trigger headers
   the later chapter must spell name the same parts, and a second enum
   would drift from this one", and the promise is kept: `BeginningOf`
   takes chapter seventeen's part and chapter seventeen's possessor, and
   the fifteen cells get their own attestation table (`partUse`). The
   grid is startlingly empty. `Upkeep` possessed is six hundred forty-one
   lines, `EndStep` possessed three hundred fifty-nine and UNPOSSESSED
   twenty-five ("At the beginning of the end step, destroy all Goblins" —
   the one cell where a bare part word is the whole phrase), `Combat`
   possessed two hundred thirty-four with the possessive extraposed onto
   the turn ("At the beginning of combat on your turn"), and every other
   cell is a silence. `Turn` is zero in all three possessions, because
   the turn's own beginning is not an event English names — the upkeep is
   what it names there — and `UntapStep` is zero likewise
   (`badTriggerAtYourTurn`, `badTriggerAtUntapStep`). The `PartUnclaimed`
   cells are chapter twenty-seven's possessor gap measured from the other
   side: "at the beginning of each upkeep" (thirty-six), "each opponent's
   upkeep" (thirty-three), "the upkeep of enchanted creature's
   controller" (twenty-seven) and "each end step" (eighty) are real
   headers whose possessor is a QUANTIFIER or a nominal, and `Whose` is a
   two-word pronominal vocabulary by construction (`badTriggerAtTheUpkeep`).
   The event row is also the only one with no noun phrase in it, which is
   why it is the only one whose header word is fixed.
174. **The intervening "if" is `Condition` verbatim in a fourth
   carrier, and the rule makes it a slot rather than a sentence.**
   [CR#603.4] is unusually explicit about all three things a grammar
   needs: it applies "only to an 'if' that immediately follows a trigger
   condition", the ability "triggers only if it is [true]", and "it
   checks the stated condition again as it resolves". The last clause is
   the carrier distinction chapter eighteen predicted — one condition
   checked at two times, which is a fact about the ABILITY and not about
   the condition — and the rule closes the door on reading it as ordinary
   English in the same breath: "the word 'if' has only its normal English
   meaning anywhere else in the text of a card". One thousand and
   forty-seven lines write the frame (three hundred forty-four after
   "When", two hundred seventy after "Whenever", four hundred thirty-three
   after "At the beginning"). It is typed in the event's AFTER-discourse
   for the same reason the effect is, and the corpus settles that rather
   than the rule: "Whenever another creature you control dies, if it had
   counters on it, put its counters on this creature" reads the dead
   creature inside the condition. Scholar of Stars is the bench's whole
   card.
175. **A delayed trigger and a trigger line are one vocabulary in two
   containers, and they should not be merged further.** [CR#603.7a] makes
   the delayed one a thing that is CREATED — "during the resolution of
   spells or abilities, as the result of a replacement effect being
   applied, or as a result of a static ability that allows a player to
   take an action" — so it is an `Effect` row typed in the discourse its
   creator built, where the card-text trigger is an `Ability` row typed
   at the empty context. Two containers, one event vocabulary, which is
   exactly the relation `Intercepts` and `HeldUntil` have had since
   chapter twenty-five. [CR#603.7] even records the linearization
   difference: the delayed ability contains "when", "whenever", or "at",
   "although that word won't usually begin the ability", and the corpus
   agrees at four hundred twelve to one — "at the beginning of the next
   end step" appears four hundred thirteen times and begins its ability
   once. The ledger's open question about the "this turn" on the old
   death query is answered by [CR#603.7b] naming the phrase: a delayed
   ability "will trigger only once — the next time its trigger event
   occurs — unless it has a stated duration, such as 'this turn'". So the
   span is the frame's and a slot, its unstated value is the rule's
   once-only default rather than an omission, and "this turn" is the one
   adverbial the family writes (`admitsDelaySpan`,
   `badDelayedUntilEndOfTurn`). The delayed clause reads three of the ten
   events and the attestation says which: the turn-part beginning, the
   departure (Portcullis, Stangg, Mysterio), and the death (Graceful
   Reprieve), with the attack and its neighbours refused
   (`badDelayedOnAttack`). One demand stayed behind with the container it
   belongs to: the watched referent is SINGULAR (`eventSubjectPlur`,
   `badDiesGroup`), which is [CR#603.7b]'s single firing, where the
   trigger line writes "Whenever one or more creatures die" as ordinary
   English.
176. **The conditional static is core's wrapper and English's
   adverbial, and one preposition tells it from a duration.**
   [CR#611.3a] is what makes it possible at all — a continuous effect
   from a static ability "isn't 'locked in'; it applies at any given
   moment to whatever its text indicates" — and core files the row at the
   same address with the same cite (`StaticEffect::Conditionally`,
   `continuous.rs`), with its own comment saying it used to be a
   `condition:` FIELD and is now "a composable effect wrapper". English
   agrees with the wrapper reading: the clause is one adverbial over a
   whole statement, and the statement is any of the others ("as long as
   you control a Gate, this creature has double strike"; "as long as an
   artifact creature you control is attacking, prevent all damage that
   would be dealt to Sanwell"). Nine hundred twelve corpus lines carry
   the bare "as long as" against two hundred ten for [CR#611.2b]'s "FOR
   as long as", which is the duration `Duration.ForAsLongAs` has spelled
   since chapter twenty-two — one preposition apart and two different
   constructions, and the table is what keeps them apart
   (`badConditionalClause`: the conditional static answers `False` to
   every `admitsSpan` cell and to `absentOk`, being an ability line and
   never a clause). It does not nest, no line writing two "as long as"
   clauses over one statement (`badDoubleConditional`). Jor Kadeen, the
   Prevailer is the bench's line, ability word and all — [CR#207.2c]
   gives "Metalcraft" no rules meaning, so the sentence under it is the
   whole ability.
177. **The May-side deontic arrives, and it is not `Cant` with its
   polarity flipped.** [CR#604.6] files the family and names its
   templates: a static ability "appl(ies) while a card is in any zone
   that you could cast or play it from (usually your hand)", and the
   shapes are "You may [cast/play] [this card] …", "You can't [cast/play]
   [this card] …", and "[Cast/Play] [this card] only …". Three hundred
   fifty-two lines write "you may play" and two hundred twenty-eight of
   them are the impulse family. The row is core's `Deontic::May(Cast{…})`
   with two of core's four fields DERIVED rather than carried. The VERB
   is derived because the glossary derives it — "to play a card is to
   play that card as a land or cast that card as a spell, whichever is
   appropriate", and [CR#601.1a] says the same — so "play" is the general
   verb over a CARD and "cast" the narrow one over a SPELL, and this
   vocabulary describes cards and permanents and has no spell carrier, so
   the two hundred ninety-three "you may cast … from your graveyard"
   lines wait with the carrier that would let their complement be a spell
   (Skaab Ruinator, Hogaak). The SOURCE zone is derived because the
   sentence before it supplies one: the impulse family exiles its cards
   and then says "that card", so the binding already carries the zone and
   `playableFrom` reads it off, which is finding 124's derivable-default
   discipline at a third site. And the row is emphatically not the
   prohibition negated, because the ZONE demand inverts: `Cant` demands a
   battlefield subject since combat is fought there, and the permission
   refuses one outright, a permanent on the battlefield having already
   been played (`badPlayFromBattlefield`).
178. **The permission is what claims two `Unclaimed` span cells, and
   chapter twenty-two named one of them by card.** The G3-era span table
   has carried its silences with their reasons attached, and two of those
   reasons were this construction. "Until your next end step" is eight
   lines and every one of them permits playing just-exiled cards, so the
   cell is `PermissionOnly` and Yasmin Khan is the bench's whole ability
   ("{T}: Exile the top card of your library. Until your next end step,
   you may play it"). "Until end of combat on your next turn" is one line
   — Brazen Cannonade's — which is the weight chapter seventeen gave
   Glyph of Destruction at the unpossessed cell, and chapter twenty-two's
   own doc had already written down what it was waiting for. Four more
   cells are RENAMED rather than opened, which is chapter twenty-two's
   move a third time: the permission writes "this turn" (ninety-eight
   lines), "until the end of your next turn" (twenty-two, joining the
   four naked control grants that had the cell to themselves), "for as
   long as" (twenty-six) and "until end of turn" (two, Spark of
   Creativity's own). The two end-step siblings stay `Unclaimed` on a
   measurement: "until the next end step" is one becomes-a-copy line and
   "until that player's next end step" is one permission carrying
   [CR#118.9]'s alternative cost, a rider the row does not spell.
   The DELAYED clause joins "this turn" in the same table, which is why
   the current-turn cell now names four constructions.
179. **The entry rider is a static ability, and the corpus writes ONE
   of the two riders it could.** [CR#603.6d] settles the category in as
   many words — text reading "[This permanent] enters tapped" is "a
   static ability — not a triggered ability — whose effect occurs as part
   of the event that puts the permanent onto the battlefield" — and
   [CR#614.1d] files the effect as a replacement, which is the engine's.
   So the sentence is a line and the row is here, three sites' worth of
   ledger paid at once. The vocabulary is chapter nineteen's `TokenRider`
   SHARED rather than re-minted: a token's with-clause and a permanent's
   own text name the same two riders, and the asymmetry between them is
   the finding. A token is created by a resolving effect that knows there
   is a combat, so "tapped and attacking" is nineteen lines; a permanent's
   static ability applies whenever it enters, from any zone in any step,
   so it can say "tapped" (a hundred thirty-two lines, of which
   Abandoned Outpost's "This land enters tapped." is one of three bare
   ones) and writes the second rider zero times (`entryRiderOk`,
   `badEntersAttackingLine`). The counters half is the bigger one and it
   is ledgered: "enters with … counters on it" is three hundred
   eighty-six lines of the four hundred fourteen "enters with", and it
   wants a counter AMOUNT where the rider vocabulary is a two-row enum.
180. **The battlefield demand was too strong, and the round that
   needed `This` as a subject is the round that showed it.** Every
   continuous-effect row demanded `OnBattlefield (nounZone n)`, which
   accepts only a phrase that PROJECTS the battlefield. The trigger
   corpus's commonest subject is "this creature" and `This` projects no
   zone at all, so the demand refused two thousand four hundred
   seventy-three lines of "When this creature enters" before they could
   be written. The repair is not to drop the demand but to ask the
   question the refusal was always for: `zoneFits` already says that
   silence on either side is no evidence and a STATED zone must agree,
   which is exactly the difference between "this creature" and "target
   creature in your graveyard". Every refusal survives the change with
   its witness renamed (`badDiesInGraveyard`, `badWouldDieInGraveyard`,
   `badGetsGraveyard`, `badCantInGraveyard`, `badBecomesInGraveyard`,
   `badGainControlGraveyard`), and one relocates in a way worth
   recording: the class word "any target" now meets the DEED's head-type
   demand rather than the zone one (`badCantAnyTarget`), because a phrase
   that describes no object fixes no type either, which is
   `badCantDisjunctSubject`'s refusal reaching a second silent phrase.
   The same reading is why the attack and block events carry NO deed
   gate: [CR#506.3]'s "only a creature can attack or block" is about the
   object at the moment it attacks, and a trigger's subject is a
   description that need not be a creature when the sentence is read —
   "When a Vehicle you control attacks, exile enchanted creature", a
   Vehicle being an artifact ([CR#301.7]) that a crew ability animates
   first. The deontic demands the grant; the event demands nothing of the
   type; one rule, two constructions, opposite answers.
181. **What the round retro-opened, and what it did not.** Claimed:
   the standing interception and the standing shield, both ledgered from
   chapter twenty-five and both now bench positives on real cards; the
   entry rider, owed from three sites; the durationless deontic, owed
   since chapter seventeen; the event-vocabulary MERGE; the conditional
   static, measured in chapter twenty-two at nine hundred-odd lines with
   no carrier; and two `Unclaimed` span cells. Partly claimed, with the
   remainder located exactly: the "unless" STATE half, whose carrier has
   arrived — a negated `Condition` under a conditional static is
   writable, and thirteen corpus lines write precisely that spelling
   ("Creatures you control get +1/+1 as long as you control no
   nonartifact, nonwhite creatures") — while the hundred eleven "can't …
   unless" lines and the thirty-nine "enters tapped unless" lines are one
   MARKING WORD away, "unless" being a second word for a negated "as long
   as" rather than a second construction; and Locke, Treasure Hunter's
   Mug ability, whose shell was a trigger and is now writable ("Whenever
   Locke attacks"), leaving the "this way" participle and the
   among-restriction, both already ledgered. Re-checked and still shut,
   each with its blocker relocated: Ana Sanctuary now has its trigger
   shell and its intervening "if" and waits on the PERMANENT word, a
   colour-quality predicate, and a conjunction `Condition` has no frame
   for; finding 156's four are unmoved except Kain, whose trigger shell
   exists and whose "that many"/"that much" reads do not, and Yes Man,
   whose reflexive trigger is deferred below; Brazen Cannonade's
   permission clause lands and its raid header does not, wanting a
   postcombat main phase `TurnPart` has no row for and an "if you
   attacked this turn" lookback. The "lose control of" family is
   relocated rather than opened: chapter twenty-two sent its ten lines to
   the event axis, and the event axis now exists and has no row for them
   — "When you lose control of this creature" is an event ([CR#603.10d]
   makes it one that looks back in time), and ten lines is the whole
   corpus, so it is ledgered with its count and its rule.
182. **What the round measured and returned.** The REFLEXIVE trigger
   is deferred with its blocker named, and the blocker is not the May
   machinery. [CR#603.12] describes it as an ability a resolving effect
   creates "that triggers 'when [a player] [does or doesn't]' take that
   action or 'when [something happens] this way'", and the corpus writes
   "When you do," two hundred ninety times. What the `May` node and the
   cost algebra already give is the OFFER; what is missing is the trigger
   EVENT, because "you do" is an anaphor to the offer's own body and
   `GameEvent` has no row that reads a clause rather than describing a
   happening. That is a construction and not a row — the same kind of
   thing as the "this way" participle it shares a rule with — so it goes
   to its own mini-round with the shape recorded. The CAST event is the
   biggest single family the round did not mint: nine hundred forty-nine
   headers write "whenever [someone] casts …", and its complement is a
   SPELL, the carrier the ledger has kept since the exchange chapter
   ([CR#109.4] gives stack objects a controller and `Zone` has no stack
   row). Below it, measured and ledgered with counts: the
   put-into-a-graveyard event (a hundred nine trigger headers, ninety of
   them "from the battlefield" — [CR#603.6c] says an ability that
   triggers on a zone "from anywhere" is never a leaves-the-battlefield
   ability, so the two are different rows); the becomes-family
   ([CR#603.2e] — "becomes tapped" a hundred, "becomes untapped"
   thirty-three, "becomes blocked" a hundred fifty-nine, "becomes the
   target of" a hundred twenty-eight, and the rule's own note that these
   "trigger only at the time the named event happens"); the life-gain
   trigger (sixty, and [CR#119.9] rewrites it before it triggers); the
   end-of-combat header ("At end of combat", eleven lines, which
   [CR#511.2] makes the end of combat STEP's beginning — a sixth turn
   part `TurnPart` has no row for, [CR#513.1a] recording that the
   end-step twin was errata'd out of this wording); the trigger's own
   instructions ([CR#603.1a] limits on targets, [CR#603.2h]'s "Do this
   only once each turn" at thirty-two lines, [CR#603.2d]'s "triggers an
   additional time" at twenty-nine — a third restriction surface beside
   the activated line's three); and the STATE trigger ([CR#603.8]), which
   is a trigger over a game STATE rather than an event and whose corpus
   here is zero lines. Two things the static row did NOT let in: the
   generic PLURAL subject ("You may play lands from your graveyard",
   Crucible of Worlds), which the ledger has kept since the deontic
   chapter and which is why the permission's graveyard cell has an
   attestation and no bench line; and the QUOTED grant, which grew two
   more refusals rather than fewer — English grants a triggered ability
   and a static ability by quoting them exactly as it grants an activated
   one (`badGainsTriggered`, `badGainsStatic`).

## Chapter Twenty-Nine — The Clause Anaphor

Chapter twenty-nine, one construction (evidence: Cornered Crook,
The Last Ronin, Thousand Moons Crackshot, Anointer of Valor; and,
for what the round measured rather than built, Heart-Piercer
Manticore, Yes Man Personal Securitron, Elektra Femme Fatale,
Olivia Crimson Bride, Hawkeye Master Marksman). The mini-round
chapter twenty-eight sent the reflexive trigger to, with the
blocker it named turning out to be the design:

183. **The reflexive trigger is a CONSTRUCTION over the enclosing
   clause, and "you do" is a PRO-VERB.** [CR#603.12] describes a
   resolving spell or ability that "may allow or instruct a player to
   take an action and create a triggered ability that triggers 'when
   [a player] [does or doesn't]' take that action", and the corpus
   writes "When you do" two hundred ninety-one times. Chapter
   twenty-eight deferred it on the observation that `GameEvent` has no
   row reading a clause rather than describing a happening, and the
   observation was right in a stronger way than it looked: "do" is not
   an event word at all. It is the pro-verb English uses to abbreviate
   a verb phrase already on the page, so the trigger's event slot is
   filled by the SENTENCE BEFORE IT, and no event row could have held
   that. `Reflexively` therefore takes the enclosing clause itself and
   the trigger body, and mints nothing else — no event, no decider
   slot, no header word. Core reaches the opposite place and the
   divergence is worth recording rather than smoothing: `ability.rs`
   says in as many words that "delayed ([CR#603.7]) and reflexive
   ([CR#603.12]) triggers are the same value", and then `EventFilter`'s
   thirty-odd rows have nothing to put in the `event` field that value
   demands. Both readings can be right about the ENGINE — a reflexive
   really is a triggered ability on the stack — and only one of them
   is a sentence. The rule itself files the family the same way this
   node does, by container: reflexive triggers "follow the rules for
   delayed triggered abilities … except that they're checked
   immediately after being created", so `Delayed` beside it is the
   sibling and the difference is which of the two knows what it is
   waiting for.
184. **The enclosure table looks THROUGH the offer, because the rule
   does — and its refusals have four different grounds.**
   [CR#603.12] writes "allow OR INSTRUCT", one action under two
   markings, and the corpus divides exactly there: two hundred of the
   two hundred ninety-one lines are a may (a hundred thirty-four a
   verb phrase, sixty-six a payment) and ninety-one a bare
   instruction, which is [CR#118.12]'s mandatory twin arriving at a
   second construction with the same reasoning finding 177 used. So
   `reflexEncloseUse` asks a branchless `May` its BODY's question and
   the may/imperative split appears nowhere in the table, which is
   what keeps the per-row counts honest: the payment is sixty-six
   lines and every one of them is offered, the sacrifice is
   seventy-one offered and seventeen instructed. The Trues are
   `Does` at a hundred fifteen, `Pay` at sixty-six, the exile at
   fifty-one, the tap at thirteen, the create and the counter-put at
   eight each, the counter-removal at seven, the mill at five, the
   return at three, the reveal at two, and the draw and the choice at
   one apiece. What makes this a table and not a predicate is that the
   Falses disagree about WHY, so a Bool would have hidden which ground
   each row stood on. The AGENTLESS rows are a rule rather than a
   count: [CR#120.1] makes "an object that deals damage … the source of
   that damage", [CR#701.14a] has a fight instruct "a creature to fight
   another creature", and [CR#119.9] rewrites the life-gain trigger as
   "whenever a source causes [a player] to gain life" — three clauses
   whose agent is not a player, and zero corpus lines each. The one
   apparent counterexample proves it: Elektra, Femme Fatale writes "you
   may HAVE her deal 2 damage to you. When you do, …", where the having
   is yours and the dealing is hers, and a causative is a True row
   (`badReflexiveOnSourceDeed`, `badReflexiveOnLifeGain`). The same
   rule reads the life change twice with opposite answers, which is
   finding 180's shape again: "you may pay 2 life" IS an enclosure,
   paying being an action a player takes where gaining life is
   something done to one. The other three grounds are that no single
   action is there to abbreviate (a sequence, a batch, a modal, a
   conditional, a branched may — `badReflexiveOnSequence`,
   `badReflexiveOnBranchedMay`), that the action is SCHEDULED rather
   than taken and so did not occur "earlier during the resolution"
   ([CR#603.12]'s own check — `badReflexiveOnDelayed`), and plain
   silence, which the search and the shuffle hold.
185. **The body reads the enclosure's POST-state with its targets
   settled; what follows the construction reads none of the trigger.**
   Both halves are the construction's content and each has its rule.
   INWARD, the reflexive fires precisely BECAUSE the action was taken,
   so what the action did is there to read — a hundred and one bodies
   write "it"/"its", thirty-eight write "that [creature/card/…]", ten
   write "that many"/"that much" — and that is the exact opposite of
   `InsteadOf` beside it, whose replaced event never happened
   ([CR#614.6]) and whose replacement therefore reads announcements
   only. Targets are SETTLED on the way in, for `delayedCtx`'s reason
   at a second site: the reflexive is its own object on the stack
   ([CR#603.3]) choosing its own targets there ([CR#603.3d]), so a
   target the enclosing spell announced is a particular by the time
   this body speaks of it, and English agrees — "exile target creature
   card from a graveyard. When you do, put X +1/+1 counters on target
   Symbiote, where X is THE EXILED CARD's toughness". OUTWARD the node
   contributes the enclosure's discourse and nothing of the trigger's,
   and [CR#603.3] says why in one clause: the ability goes on the stack
   "the next time a player would receive priority", so the rest of the
   resolving effect finishes before it resolves and cannot mention what
   it will do (`badAfterReflexiveReadsTrigger`). That is `Delayed`'s
   "a future clause mentions nothing NOW" with a clause in front of it
   that DID run. Anointer of Valor is the card that forced the inward
   typing: "Whenever a creature attacks, you may pay {3}. When you do,
   put a +1/+1 counter on that creature" reaches back over a payment
   that mentions nobody to the trigger header's attacker, which a
   trigger row typed at the empty context could not have found.
186. **Three slots the family does not get, and one refusal that is
   the rule's rather than English's.** The trigger WORD is not a slot:
   two hundred ninety-one "When", zero "Whenever", zero "At", which
   makes this the one trigger in the file whose opening word finding
   172 does not have to leave open — and no `admitsTrigger` row is
   forced either, there being no event to key one on. The SPAN is not a
   slot: [CR#603.7b] gives a delayed trigger "this turn" and
   [CR#603.12a] replaces that rule outright for this one, the reflexive
   triggering "once for each of those times" the event occurred rather
   than once unless told otherwise, so the count is the event's
   multiplicity; zero lines write a duration, and the rule's own
   exception has exactly one card (Hawkeye's "you may pay {1} up to
   three times. When you do, …", which [CR#603.12a] fires once). The
   intervening "if" IS licensed — [CR#603.4] applies to any triggered
   ability and three bodies open with one — and is ledgered at that
   count. The DECLINED reflexive is the interesting silence: the rule
   words the family as triggering "when [a player] [does or doesn't]"
   take the action, so a negative reflexive is licensed outright, and
   English writes it zero times. The single "When you don't" line in the
   corpus is a quoted STATE trigger (Olivia, Crimson Bride's "When you
   don't control a legendary Vampire, exile this creature"), and the
   declined branch stays where its eighty-three lines are, on `May`'s
   `ifNot` (`badReflexiveOnDeclinedMay`). One offer, one reader: no line
   writes both "if you do" and "when you do" over one choice, and
   Heart-Piercer Manticore's own ruling says what the card would have to
   choose between — with the reflexive, "players may cast spells and
   activate abilities before a creature is sacrificed and then again
   after the creature is sacrificed but before damage is dealt"
   (`badReflexiveOnBranchedMay`).
187. **What the round claimed, and where the two remaining cards are
   blocked.** Claimed from chapter twenty-eight's ledger: the reflexive
   itself, deferred there with its blocker named, now four bench
   positives across the enclosure shapes it opens — the offer (Cornered
   Crook, the whole card), the instruction (The Last Ronin's chapter
   II), the payment (Thousand Moons Crackshot, the whole card) and the
   read-back (Anointer of Valor). Relocated rather than claimed, each
   with its blocker moved somewhere smaller: Yes Man, Personal
   Securitron's reflexive shell exists now, and what stops the card is a
   quest counter `CounterKind` does not carry — its enclosure is the
   family's one line whose action is a CONTROL GRANT, attested by
   ruling rather than by inference (2024-03-08: "that effect is part of
   a reflexive triggered ability that triggers only if the target
   opponent gains control of Yes Man"), so the cell is `EncUnclaimed`
   with the card written on it (`badReflexiveOnGainsControl`). And
   [CR#603.12]'s OWN worked example is the one this construction cannot
   write: Heart-Piercer Manticore's "when you do, this creature deals
   damage equal to that creature's power" reads the sacrificed creature
   as a CREATURE while it sits in a graveyard, which is
   [CR#608.2h]'s last-known information ("if it's no longer in that
   zone … the effect uses the object's last known information", and the
   card's 2017-04-18 ruling says outright that "the sacrificed
   creature's last known existence on the battlefield is checked to
   determine its power"). The workbench's carrier rule is right to
   refuse it — an object "stops being a permanent as it's moved to
   another zone" [CR#110.1], so the type word stops reaching it, which is
   `badReflexiveTapsSacrificed`'s own refusal one word over — so the
   family that needs a last-known read is ledgered whole rather than
   smuggled in under a demonstrative. The "this way" participle
   [CR#603.12] shares its sentence with is untouched and still ledgered
   at a thousand and fifteen lines. Below those, the vocabulary the
   ninety-one instructions asked for and did not get, measured: the
   keyword actions (amass, earthbend, surveil), the die roll, the mana
   addition, and the attach.

## Chapter Thirty — The Card

Chapter thirty, the outermost container and the last absent primitive
family (evidence: Scathe Zombies, Rorix Bladewing, Aladdin's Ring,
Moonlit Wake, Anthem of Champions, Counterspell, Hymn of Rebirth,
Zimone Quandrix Prodigy, Preeminent Captain, Workhorse, Desperate
Castaways, Silent Assassin, Beast Whisperer, Skaab Ruinator, Escape to
the Wilds; and, for what the round measured rather than built, Squee
the Immortal, Hogaak Arisen Necropolis, Brazen Cannonade, Locke
Treasure Hunter, Escape to the Wilds' second line). The round the
fragments became cards:

188. **The card container is a thin RECORD, and what it buys is the one
   question no ability line can answer about itself.** [CR#200.1] lists
   fifteen parts; five of them are language and the rest are printing,
   so `Card` carries the name, the mana cost, the type line, the text
   box and the power-and-toughness pair, and leaves the illustration,
   the expansion symbol and their neighbours where they are. Nothing in
   that list is a construction — this is the first type here that is
   not one — and the container would be inert if the parts were all it
   held. What makes it necessary is [CR#113.3a]: "any text on an
   instant or sorcery spell is a spell ability unless it's an activated
   ability, a triggered ability, or a static ability that fits the
   criteria described" [CR#113.6]. That sentence qualifies an ability by
   what its CARD is, and no line carries that fact, so `Ability` could
   grow its fourth [CR#113.3] row but could not gate it until something
   knew the type line. `cardAbilityOk` is the gate and the reason the
   type exists. Core reaches the same shape at
   `deckmaste_semantics/src/card.rs` and the three divergences are each
   measured: core's flat `types`/`subtypes` fields are chapter
   nineteen's `TypeLine` record here (the same two lists under one name,
   shared with the two clause readers); core's `color_indicator`
   ([CR#204.1] — a printed dot, not language), `loyalty` and `defense`
   are absent; and core's `Card` wraps a face in a two-faced enum
   ([CR#712.8]) where a face is what this record is. The NAME is a
   `String` and the one field with no grammar in it, carried for
   `TokenChars`' reason at a second site — [CR#201.1] makes it a printed
   part, and modern oracle templating writes "this creature" in the text
   box, so nothing here reads it back.
189. **The rules text is a LIST OF LINES on BOTH sides of the split, and
   the rule the round was sent to read says so in its own "unless".**
   The question set was whether a permanent card's text is ability lines
   and an instant's is an `Effect`. It is not: [CR#113.3a] makes the
   spell ability a CATEGORY of ability ([CR#113.3] listing four), so
   both cards hold a list and what differs is which rows the list may
   contain. The spell card's default row then holds exactly the `Effect`
   the other reading would have made the whole field, which is why the
   two designs look alike from outside and why only one of them can say
   what Counterspell's line is. The table is full rows over two card
   classes and five ability rows, and each cell has its own ground. The
   SPELL row is [CR#113.3a] read forwards on a spell card and backwards
   on a permanent one — a spell ability is followed "while an instant or
   sorcery spell is resolving", which a permanent card's text never is.
   The ACTIVATED and TRIGGERED rows are open on both sides, that being
   the rule's own list of exceptions. The STATIC row is open on
   permanents and shut on spell cards, and the shutting is [CR#113.6]'s
   criteria rather than a count: its first sentence sends an instant's
   abilities to the stack where every `StaticEffect` row here
   establishes a battlefield continuous effect, and the two exception
   families a spell card really prints — [CR#113.6d]'s alternative costs
   and [CR#113.6e]'s cast restrictions — this grammar spells neither
   (`badStaticOnSorcery`). The KEYWORD row is shut on spell cards by
   measurement instead: the three keywords this file carries are
   flying, trample and haste, and none is printed on an instant or a
   sorcery (`badKeywordOnInstant`). Two card types had to arrive for the
   table to be askable, `Instant` and `Sorcery`, and their `typeRank`
   entries are the file's first UNWITNESSED cells — no printed type line
   here writes either beside another card type, so the order is a
   convention the rank must pick and nothing measures.
190. **Six whole cards, and what changed is what a bench term IS.**
   Every positive before this round was a fragment — a clause, a
   sentence, an ability line, with the rest of the card named in a
   comment and elided. These are printed cards top to bottom, one per
   container shape, and none of them elides a part: Scathe Zombies
   ({2}{B}, Creature — Zombie, 2/2, no rules text) is the VANILLA card
   and the shape that shows an empty text box is a card with nothing to
   say rather than one that failed to be written; Rorix Bladewing
   ({3}{R}{R}{R}, Legendary Creature — Dragon, 6/5, "Flying, haste") is
   the FRENCH VANILLA and the supertype's witness, its one comma-joined
   line holding two abilities; Aladdin's Ring ({8}, Artifact) is the
   ACTIVATED card, a compound cost and a damage clause with the class
   word in its recipient slot; Moonlit Wake ({2}{W}, Enchantment) the
   TRIGGERED card; Anthem of Champions ({G}{W}, Enchantment) the STATIC
   card, and the shortest statement in the corpus that is one; and
   Counterspell ({U}{U}, Instant, "Counter target spell.") the SPELL
   card, which needed the container to exist at all — the same sentence
   after a colon on a permanent would be an activated ability's effect,
   and [CR#113.3a] settles which by asking what the card is. Hymn of
   Rebirth ({3}{G}{W}, Sorcery) is a seventh, and it doubles as the
   arrival rider's whole-card witness. The four demands are separate
   witnesses rather than one gathered one, and that is a pin discipline
   rather than a taste: a gathering type reports its own name whichever
   question refused (`CardLine`, `CardText`, `CardPt`, `CardCost`).
   SUPERTYPES land, at one row of [CR#205.4a]'s five, and they land on
   the CARD rather than on `TypeLine` — [CR#205.4b] makes a supertype
   "independent of its card type and subtype", and `TypeLine`'s two
   existing readers write none between them, so the third reader is the
   one that has them. `Legendary` is Rorix's; `Basic` wants a land-type
   subtype vocabulary, `Snow` a supertype nothing here writes (its mana
   symbol landed in chapter twenty-seven), and `World` and `Ongoing` are
   legacy and Archenemy. LOYALTY does not land and the blocker is not
   the cost: a loyalty ability is an activated ability whose cost is a
   symbol change, and `Cost` could carry that cheaply, but no whole
   planeswalker is writable — `CardType` has no `Planeswalker` row,
   [CR#606.3]'s shared once-per-turn cap is core's third use-limit row
   waiting with it, the eight hundred thirty-two bracketed cost
   components are a symbol vocabulary of their own, and the planeswalker
   subtypes are a set apart. Ledgered whole, as the round was told to if
   the witnesses did not land whole.
191. **The Move rider bundle costs FORTY-THREE fewer call sites than the
   ledger promised, and the reason is an idiom that arrived after the
   estimate.** The design was settled in chapter twenty-six and only its
   execution waited: one arrival slot on `Move` carrying a closed rider
   BUNDLE rather than a scalar, because the riders combine — three
   hundred fifteen lines write "onto the battlefield tapped", nineteen
   of them "tapped and attacking", and a hundred thirty-five "under
   [someone]'s control", and a placement can wear a rider from each half
   at once. Separate `MoveTapped` / `MoveUnderControl` constructors
   would have multiplied the verb by its combinations. The entry half is
   chapter nineteen's `TokenRider` list SHARED and its `ridersOk` shapes
   verbatim, which is finding 179's sharing at a third site; the
   CONTROLLER is a second field and not a third rider word, because it
   takes a noun where the other two are bare participles, and it is an
   OVERRIDE rather than a required field because [CR#110.2a] already
   gives the instructed player control. The gate is the battlefield, and
   both halves have their own rule for it: [CR#110.5b] says permanents
   "enter the battlefield untapped … unless a spell or ability says
   otherwise", which is the tapped rider's own rule and exists nowhere
   else, and [CR#109.4] gives an object off the stack and off the
   battlefield no controller to override (`badMoveRidersToGraveyard`,
   `badMoveControlToHand`). What the ledger's forty-three call sites
   turned out to be is zero: chapter twenty-seven's activated line
   introduced the `{default …}` named-implicit slot, and a defaulted
   slot changes no pattern and no call site. The estimate was written
   before the idiom existed. Witnesses are Zimone, Quandrix Prodigy's
   first ability (tapped), Preeminent Captain's second line (tapped and
   attacking), and Hymn of Rebirth whole (under your control).
192. **The enters-with-COUNTERS entry opens, and the blocker the ledger
   recorded was about the wrong container.** Three hundred eighty-six
   lines of the four hundred fourteen "enters with" write counters, and
   the entry has said since chapter nineteen that they want "a counter
   AMOUNT where `TokenRider` is a two-row enum". They do; what the
   re-check found is that an `Amount bs` was never going to fit there at
   any price, `TokenRider` being shared with `TokenChars`, an unindexed
   record. Put in a second `StaticEffect` row instead, the line needs no
   new vocabulary at all — the count is `WrittenCount`'s written
   magnitude and the kind is `CounterKind`'s — and the two rows answer
   the same `staticKind`, which is the claim that they are one class of
   statement ([CR#603.6d,614.1d] file them together). Workhorse's first
   line is the witness. The kind vocabulary reaches a hundred sixty-eight
   of the three hundred eighty-six (ninety-nine plural and sixty-two
   singular "+1/+1", seven "-1/-1") and the rest are the counter catalog
   itself — time, oil, fade, charge, indestructible, finality, shield,
   divinity, eight lines apiece and fewer — which is a different ledger
   entry from the one this closes.
193. **The spell carrier is four rows and a zone, and it stops exactly
   where the round said to stop.** The STACK arrives as a `Zone` row and
   not as a second object sort, because [CR#112.1] identifies the two in
   one sentence — "a spell is a card on the stack" — so the spell WORD
   is this zone's carrier noun exactly as "card" is the four
   card-zones' ([CR#108.2]) and a type word is the battlefield's
   ([CR#109.2]). That identity is what makes the whole family cheap: "a
   spell" is `InZone stackZ`, "a creature spell" the type word under the
   same clause, and `NounWord` gains `SpellW` so "that spell" reads back
   (thirty-one lines). The zone is public ([CR#400.2] names it), shared
   rather than possessed ([CR#400.1]), and never a `Move` destination —
   [CR#601.2a] puts a card there as the first step of CASTING, an action
   a player takes rather than a placement a sentence writes, which is
   core's own `exclude(Library, Stack)` (`badMoveToStack`). On top of it:
   the CAST event, a thousand and sixty-nine headers (nine hundred
   forty-eight "Whenever", a hundred twenty-one "When"; the caster is
   "you" seven hundred thirty-six times, "a player" a hundred nine, "an
   opponent" eighty-four), two noun slots for `DealsCombatDamage`'s
   reason, and the one event row that does NOT retag because the rule
   already did — [CR#601.2a] moved the card to the stack before the
   event finished. And the COUNTER verb ([CR#701.6a]), a hundred
   ninety-three lines of which fifty-two are Counterspell's bare
   sentence, with one noun slot and no destination: the rule does say
   where a countered spell goes and the sentence does not, and a `Move`
   the sentence never writes is the engine's fact. The BOUNDARY is where
   the round was told to leave it. Copy machinery is out (seventy-one
   "spell you control" lines are all of it); the ability half of
   [CR#701.6a] is out, an ability on the stack being an object
   [CR#109.1] this noun vocabulary has no word for; and the
   counter-UNLESS family is refused by chapter twenty-seven's
   linearization wall, not by anything this round could have fixed —
   all fifty-three of those lines write "unless ITS CONTROLLER pays",
   the anaphoric payer [CR#118.12a]'s rewrite types before the clause
   that announces it (`badUnlessAnaphoricPayer`, unchanged).
194. **The cast PERMISSION was waiting on two things smaller than a
   spell carrier, and one of them was a reading rather than a row.**
   Chapter twenty-eight sent the two hundred ninety-three "you may cast
   … from your graveyard" lines to wait for the carrier, on the reading
   that "cast" takes a SPELL where "play" takes a CARD. [CR#701.5b] says
   the opposite in four words — "to cast a card is to cast it as a
   spell" — so the complement is not what separates the verbs, and
   [CR#604.6] brackets them as a SLOT in its own templates ("You may
   [cast/play] [this card] …"). What does separate them is the LAND:
   [CR#601.1a] makes "play" the union of playing a land and casting a
   spell, and [CR#305.9] makes the exclusion a rule — "it can't be cast
   as a spell" — so the gate is a type demand on the complement's head
   (`badCastALand`). The second thing was the SOURCE zone, derived in
   chapter twenty-eight from the complement's binding because the
   impulse family exiles first and says "that card"; the graveyard
   family writes the zone IN the permission over a complement that
   carries none, so the slot is now a defaulted `Maybe` and `Nothing`
   means "read it off the complement". Skaab Ruinator, which chapter
   twenty-eight named by name, is the witness. And the family's SIZE is
   corrected in passing: of the two hundred ninety-three lines, two
   hundred eight are parenthetical keyword REMINDER text (disturb,
   flashback, escape and their neighbours) and eighty-five are real
   ability lines; the "from exile" hundred eighteen divide a hundred
   thirteen to five the same way. The ledger's count was a reminder-text
   count, which is the sort of thing only a carrier arriving can show.
195. **"This way" is ONE construction with two surfaces, and the
   plurality was the real gap.** A thousand and fifteen lines carry the
   phrase. The participial back-reference and the attributive participle
   pick out the same referent under this grammar's uniqueness discipline
   — a read's obligation is `= 1`, so there is never a second stamp of
   the same verb for the deictic to disambiguate against — which makes
   "the card exiled this way" and "the exiled card" two spellings of
   `TheVerbed` rather than two constructions. That is finding 26's own
   reasoning one level up: the participle is already the disambiguator
   English switches to where a bare demonstrative would be ambiguous,
   and "this way" is the same switch worn as a phrase. It is a TABLE and
   not a free slot, because the two surfaces are not interchangeable per
   verb: exile writes both (a hundred eighty-four attributive against a
   hundred eighteen deictic), sacrifice both (a hundred twenty-three
   against twenty-nine), discard both (twenty-two against thirty-nine),
   and DESTROY writes the deictic only — fifty-one lines against zero
   (`badDestroyedAttributive`). What was actually missing was the
   PLURAL: `verbedMatch` answered `False` to every `ManyOf` binding, so
   a group action's participants had no participle to be read by, and
   the family's actions are group actions. `ThoseVerbed` is to `Them`
   what `TheVerbed` is to `That`, and Escape to the Wilds' first line is
   the witness — "Exile the top five cards of your library. You may play
   cards exiled this way until the end of your next turn." The verb
   families are attested with their counts and the boundary named:
   revealed a hundred twenty-three, exiled a hundred eighteen, destroyed
   fifty-one, milled forty-three, discarded thirty-nine, cast
   thirty-three, countered thirty-one, sacrificed twenty-nine, tapped
   nineteen. Only the four that are `VerbName` tags can be written,
   because the stamp keys on that enum and reveal, mill, tap and cast
   are `Effect` rows with no tag; and the family's two commonest
   CONSUMERS want structure this round did not build — the distributive
   determiner ("each card exiled this way" is twenty-one lines, the
   single commonest frame) and the for-each amount ("for each creature
   destroyed this way"), which wants `CountOf` to accept a group MENTION
   where it takes a description. Both are ledgered with those counts.
196. **What the round retro-opened, and what it did not.** Claimed:
   the enters-with-counters entry (finding 192); the cast EVENT and the
   cast-verb PERMISSION, chapter twenty-eight's two largest (findings
   193, 194); the "unless" MARKING word, which chapter twenty-eight
   measured at a hundred fifty lines and one word away and which is now
   a two-row table gated on the condition's polarity — "unless [C]" is
   "as long as not [C]" with the negation on the subordinator, so the
   row demands a negated condition and spells the positive underneath
   (Desperate Castaways, the whole card; `badUnlessOnPositive`); and
   the SIXTH turn part, [CR#511.2] saying outright that abilities which
   trigger "at end of combat" trigger "as the end of combat step
   begins", so it is a part beginning like the other five and only its
   surface is short — [CR#513.1a] recording that the end step's
   identical wording was errata'd and this one was not. The bench's
   witness for it is the DELAYED clause (Silent Assassin, the whole
   card) rather than one of the eleven headers, because every one of
   those wants a blocking-RELATION predicate ("creatures blocking or
   blocked by this creature") or a combat lookback ("if this creature
   attacked or blocked this combat"). ANNOTATED, chapter thirty-eight:
   the relation half landed and seven headers are now writable; the
   lookback half did not (finding 271). Re-checked and still shut, with
   its blocker unchanged: Brazen Cannonade's raid header, which the
   sixth part does not open — it wants a postcombat main phase (ten
   corpus lines) and an "if you attacked this turn" lookback, neither of
   them the end of combat. Partly claimed, narrowed: Locke, Treasure
   Hunter's Mug ability, whose trigger shell landed in chapter
   twenty-eight and whose cast VERB landed here, leaving three smaller
   things — "if a land card was MILLED this way" wants a participle tag
   the mill clause does not carry, "create a Treasure token" wants the
   predefined-token catalog, and "from among those cards" is the
   among-restriction, re-measured at three hundred thirty "from among
   them/those cards" of four hundred fifty-seven "from among" and still
   a domain restriction on a description rather than a determiner over a
   group.
197. **What the round measured and returned.** The [CR#109.4] STACK
   caveat is revisited now that the zone exists, and the answer is that
   `seedZone (ControlledBy _)` stays on the battlefield: the relation
   constrains its referent to a two-zone SET where `seedZone` names one
   zone, so seeding the stack instead would lose the graveyard refusal
   without buying the spell, and what the shape wants is a zone-SET seed
   or a second admissibility table. Measured before leaving it —
   seventy-one lines write "spell(s) you control", six "an opponent
   controls", two "you don't control", and every one is copy machinery
   (`badControlledSpell`). The DELAYED cast is real and unwritable:
   thirty-six lines write "When you next cast a … spell this turn", and
   every body either grants an ability to an object on the stack, which
   `Gains` refuses by zone, or copies the spell. The cast event's other
   two readers are measured silences — nothing intercepts a cast, the
   nearest family being [CR#118.9]'s alternative cost, and no line ends
   a duration at one. And the entry-rider half of the "unless" marking
   is attested and unreached: all thirty-nine "enters tapped unless"
   lines want a basic land TYPE ([CR#305.6]'s five, a subtype vocabulary
   this file has no rows for), the "legendary" supertype as a
   PREDICATE rather than a type-line field, a counted comparison over
   "other lands", or a game-state condition — so the marking word lands
   on the deed restriction alone.

## Chapter Thirty-One — The Second Zone

Chapter thirty-one, where the exile zone stops being a destination and
becomes a place things happen (evidence: Jhoira of the Ghitu, Alaundo the
Seer, Arc Blade, Daydream, Cold Storage, Sisters of Stone Death, Synod
Sanctum, Muse Vessel; and, for what the round measured rather than built,
Karn Liberated, All Hallow's Eve, Delay, Mari the Killing Quill, Kianne
Dean of Substance, Volatile Chimera, Mechtitan Core. The round a gate
that three chapters had answered "the battlefield" turned out to have
two rows):

198. **The counter verbs' battlefield demand was half a measurement, and
   the half that was wrong is the one the rules state outright.** Chapter
   nineteen wrote finding 84's gate in its own words — "no corpus line
   puts a counter on a card in a graveyard or in exile with this clause"
   — and the graveyard half holds while the exile half does not.
   [CR#122.1a] gives a +X/+Y counter its meaning "on a creature card in a
   zone other than the battlefield" and [CR#122.1b] does the same for a
   keyword counter, so counters outside the battlefield are not an
   oversight in the rules but a named case; and English writes them.
   Jhoira of the Ghitu's "{2}, Exile a nonland card from your hand: Put
   four time counters on the exiled card" is the ordinary put verb with
   the ordinary participle read of a card the COST just exiled — public
   zone, [CR#400.7j], so nothing new was needed to reach it — and Alaundo
   the Seer's "remove a time counter from each other card you own in
   exile" is the removal twin over a phrase that places its own referent
   there. So `OnBattlefield` is replaced at those two slots by
   `CounterHolder`, a closed full-row table over the seven zone answers,
   and the other eleven battlefield-demanding slots did NOT widen with
   them: destroy, sacrifice, tap, fight, the stat deltas and the copular
   reads keep the old type, which is why the widening is a second witness
   type rather than a loosened first one.
199. **The four shut rows are silences of three different kinds, and the
   stack's has a rule behind it.** The GRAVEYARD is a counted silence:
   all seventeen lines that write "counter" and a graveyard in one
   sentence put the counter on a battlefield object and read the
   graveyard for a number ("a +1/+1 counter on this creature for each
   creature card in your graveyard") or return the card to the
   battlefield first ("Return this card from your graveyard to the
   battlefield with a finality counter on it"), so the graveyard is where
   counters are counted FROM and never put ON
   (`badPutCountersGraveyard`, `badRemoveCountersDead`, both re-keyed
   onto the new payload and both still refusing). The HAND and the
   LIBRARY are the same silence with the same shape — every "counter"
   line naming them counts cards there. The STACK is the one with a rule:
   "put a counter on target spell" is zero lines, and [CR#122.6] says why
   in one sentence — a bare counter instruction "refers to putting
   counters on that object while it's on the battlefield and also to an
   object that's given counters as it enters the battlefield" — which
   makes the battlefield the DEFAULT reading and a second zone something
   a line has to name. Exile is the one zone oracle names.
200. **The zone is open to counters, not to a counter, and the kind
   catalog gains exactly one row.** `CounterKind` does not index the
   table, and the corpus is why: twenty-one kinds ride an exile in main
   text — aegis, blood, brain, collection, croak, delay, discovery,
   dream, egg, hatching, hit, ice, kick, memory, scream, silver, stash,
   study, takeover, time and void — and more arrive by the put verb
   afterwards (page, refine, voyage), so a kind-indexed zone table would
   have been twenty-odd rows saying the same thing. What earns a `CounterKind` row is finding 85's test
   unchanged, a one-shot put or remove of the sort this grammar writes,
   and `Time` passes it three times over: a hundred fifty-nine lines
   write the words, Arc Blade puts three on in an exile rider, Jhoira
   puts four on with the put verb, Alaundo takes one off with the remove
   verb. It is the first row in the catalog with no rule of its own in
   [CR#122.1]'s list, and that is the point rather than a gap — a time
   counter does nothing by itself and the abilities that read it supply
   the meaning, [CR#702.62a] being the biggest reader and the keyword
   this round deliberately does not spell. The other twenty stay
   unwritten for finding 85's reason, and the ledger's counter-kind
   catalog entry keeps them.
201. **The exile-with-counters rider is the MOVE's, not the exile
   verb's, and one card proves it in a sentence.** "Exile Arc Blade with
   three time counters on it" and "return that card to the battlefield
   under its owner's control with a +1/+1 counter on it" are the same
   adverbial in the same slot after two different destinations, and
   Daydream writes the second one beside a controller override in one
   placement — so the rider goes on `MoveRiders` beside the entry
   participles and the controller, and there is no exile-rider structure
   at all. A hundred and two lines write it on an exile (forty-seven of
   them the suspend family's keyword reminder line, fifty-five real
   ability lines) and eighty-three on a return to the battlefield. Finding
   192 said an `Amount bs` "was never going to fit" a rider slot at any
   price, and that was true of the container it was about: `TokenRider` is
   shared with the unindexed `TokenChars`, where `MoveRiders` has carried
   a `Noun bs Player` since chapter thirty and is indexed already. So the
   count is typed in the patient's own announcement, which is what "with
   a number of time counters on it equal to ITS mana value" needs. The
   frame is attested down to its last word: "on it" is written by all
   hundred and two and omitted by none, so it is spelling and not a slot;
   the
   plural "on them" is written once and by a separate put clause rather
   than a rider; and the count obeys `WrittenCount` like every other
   (`badExileZeroCounters`).
202. **The rider bundle's zone admissibility is PER HALF now, and that
   is what keeps the two halves from leaking.** Chapter thirty's
   `ridersFitZone` was one question — is anything written, and is the
   destination the battlefield — and it could not stay one, because the
   counter rider rides two destinations where the other two ride one. So
   the entry participles and the controller keep their battlefield
   demand and their rules ([CR#110.5b]'s untapped default, [CR#506.3a]'s
   attacking placement, [CR#109.4] giving an off-battlefield object no
   controller to override), and the counter rider answers `counterZone`
   instead — the SAME table the put and remove verbs answer, because it
   is the same fact about where a counter may sit. Both leaks are pinned
   and both are measured: "exile it tapped" is zero lines and is refused
   (`badExileTapped`), "exile it under your control" is zero lines too
   (the two corpus hits are returns FROM exile), and a counter rider on a
   move into a graveyard is refused by the counter table rather than by
   the battlefield one (`badMoveCountersToGraveyard`). The keyword-action
   tag learned the same distinction: `TagBody` indexes the bundle now, so
   the exile tag has two rows — the bare placement and the one ridden
   shape oracle writes — and a destroy or a sacrifice keeps the empty
   bundle it always had.
203. **The linkage is a SOURCE-KEYED note, and the rules give it its own
   sentence in two different chapters.** [CR#406.6] states it in the
   exile chapter — an object with an ability that exiles cards and an
   ability referring to "the exiled cards" or to cards "exiled with [this
   object]" has the two LINKED, the second referring "only to cards that
   have been exiled due to the first" — and [CR#607.2a] states it again
   in the linked-abilities chapter, adding that the reference reaches
   "cards in the exile zone that were put there" by that ability.
   [CR#406.5] even gives it a physical form: exiled cards that matter
   "due to … the abilities of the cards that exiled them" are kept in
   their own pile. So the group is keyed to a SOURCE and not to a
   discourse, which is what `TheVerbed` could never have been — a stamp
   lives in the bindings one clause hands the next, and this outlives the
   sentence, the ability and the turn. `ExiledWith` is therefore a
   PREDICATE and not a noun row, and that one choice is what makes the
   whole family cheap: it is head-bearing and zone-projecting exactly as
   `InZone` is ([CR#406.2] — "an exiled card is a card that's been put
   into the exile zone" — so the carrier noun comes with the phrase), and
   every determiner already in the file then spells a frame. A hundred
   seventy-five lines write "exiled with"; a hundred forty-one of them
   head on "card(s)"; nineteen write "all cards exiled with", twenty-six
   "a card exiled with", twenty-one "from among cards exiled with", nine
   "each card exiled with" — and `AllOf`, `Indefinite`, `SomeOf` and
   `Each` write those four with no new vocabulary at all.
204. **The linkage OUTLIVES its source, and the corpus states it in the
   sharpest possible frame — the source is destroyed in the COST.** Cold
   Storage is the whole card and it is the card the ledger has named for
   this entry since chapter fourteen: "{3}: Exile target creature you
   control." and "Sacrifice this artifact: Return each creature card
   exiled with this artifact to the battlefield under your control." The
   sacrifice is paid before the effect after the colon runs, so the
   source is in a graveyard when its own group is read, and the group is
   still there; Synod Sanctum writes the same shape over `AllOf`. Nothing
   in [CR#607] makes the relation a property of a live object — it links
   two abilities PRINTED on one, and [CR#608.2h] lets the ability use
   last-known information for a source that has moved — so the honest
   reading is that the note survives anything that happens to the object
   that made it, which is also why a restart of the game would leave it
   standing. The SOURCE SLOT is where the rules bite instead:
   [CR#607.1]'s "printed on it" makes the phrase name the reading
   ability's own object and no other, and [CR#607.5]'s worked example
   turns on exactly that — a Quicksilver Elemental holding two different
   exiling abilities can return only what the LINKED one exiled. So the
   slot takes the self-word under either of its two spellings, the bare
   one and the sorted one ("cards exiled with this artifact", thirty-six
   lines), and refuses a target, a described object, and by extension
   every foreign source (`badExiledWithOtherSource`,
   `badExiledWithDescribedSource`). English has ONE phrase for the other
   reading and it is a different construction: the passive "exiled by",
   written once against a hundred seventy-five.
205. **The linked cards are IN EXILE and the phrase says so, which is
   what settles two refusals nobody had to argue about.** [CR#607.2a]'s
   own words put the group in the exile zone, so `seedZone` answers
   `Just Exile` and the phrase behaves like any other zone clause: "a
   creature card exiled with this creature" is Sisters of Stone Death's
   real phrase and lands, while "a card you control exiled with this
   artifact" and "an attacking creature exiled with this artifact" are
   refused by `ZoneCoherent` — [CR#109.4] gives an exiled card no
   controller, and the attacking designation is declared over creatures
   their controller controls ([CR#508.1a])
   (`badExiledWithControlled`, `badExiledWithAttacking`). And the phrase
   does not NEGATE: "not exiled with" is zero lines against a hundred
   seventy-five positive, because a source-keyed group is named to be
   acted on and the cards outside it are described by another zone or
   another phrase rather than by this one turned inside out
   (`badNegatedExiledWith`).
206. **"The exiled card" is TWO constructions wearing one surface, and
   the disambiguator is the ability boundary — which is the first thing
   in this file that boundary has ever decided.** Finding 195 made "the
   exiled card" and "the card exiled this way" two spellings of
   `TheVerbed`, a read of a stamp the same discourse laid down. [CR#607.3]
   says the same words are ALSO the linkage's: "if, within a pair of
   linked abilities, one ability refers to a single object as 'the exiled
   card,' 'a card exiled with [this object],' or a similar phrase" — so
   the surface is ambiguous and what tells the two apart is whether the
   exile happened in THIS ability. The corpus writes both freely (a
   hundred seventy-eight lines write "the exiled card"; some are Jhoira's
   same-clause read and some are a later ability's, "you may play the
   exiled card without paying its mana cost" said three sentences and one
   colon away from the exile that made it). This file spells only the
   first, and the reason is a gap with a name rather than a decision:
   there is no definite determiner over a DESCRIPTION here — `TheVerbed`
   is a read of a binding, and "the card exiled with this artifact" would
   want "the" over `ExiledWith`, which no row provides. Ledgered with its
   count. What the boundary DOES decide today is that `Ability` stays
   unindexed and correct in staying so: the linkage carries no discourse
   across the boundary — no binding, no anaphor, nothing a later sentence
   reads back — only a group that a phrase DESCRIBES, which is exactly
   why a predicate was the right shape and an indexed ability line would
   have been the wrong one.
207. **`HeldUntil` and the counters rider are opposite answers about the
   same card, and the corpus never asks for both.** Chapter
   twenty-five's rider is [CR#610.3]'s one-shot pair: the clause exiles
   and schedules its own undo, so nothing has to ask for the card back.
   The counters rider does the reverse — it leaves a card in exile with a
   number on it and waits for an ABILITY to read that number, which is
   the whole of what [CR#702.62a] builds out of these components. So they
   are reconciled by measurement rather than by argument: "exile … with …
   counters on it until …" is zero lines, and `heldUntilOk` now reads the
   bundle and takes the unridden exile only (`badHeldUntilWithCounters`).
   The other reconciliation the round was sent to make needed no code at
   all: `HeldUntil`'s implicit single-object recall is the RULE's, not a
   phrase's — [CR#610.3c] returns the object "under its owner's control
   unless otherwise specified" and the sentence writes no recall — where
   the linkage exists precisely because a later ability has to name
   something the rules will not hand back on their own. One family says
   "and then undo this", the other says "and remember what I did".
208. **Karn's minus reaches its linkage and stops at five other things,
   each named.** "[−14]: Restart the game, leaving in exile all non-Aura
   permanent cards exiled with Karn. Then put those cards onto the
   battlefield under your control." The LINKAGE lands — "cards exiled
   with Karn" is `ExiledWith` under `AllOf`, the same phrase Synod
   Sanctum writes — and the sentence still wants: the RESTART event,
   which is the game-as-object family and which [CR#727.1] introduces
   with the words "One card (Karn Liberated) restarts the game" (one
   corpus line, and a family of one is not a family); the "leaving in
   exile" EXEMPTION, [CR#727.5]'s "effects may exempt certain cards from
   the procedure", a participial adverbial with no second instance; the
   negated SUBTYPE "non-Aura", which would force `subtypeType`'s first
   non-creature answer and a subtype vocabulary this file grows one
   witnessed row at a time; the PERMANENT word as a head noun, which the
   ledger has carried since chapter fourteen; and the whole planeswalker
   container — no `CardType` row, no loyalty cost symbols — which chapter
   thirty ledgered whole. So the restart is NOT minted, on the round's
   own instruction and on the honest count, and the linkage is designed
   to survive one: finding 204's lifetime argument is what makes that
   claim cheap, since a note that outlives its source being sacrificed
   outlives its source being shuffled into a new library, and [CR#727.2]
   keeps every card involved in the game that restarted.
209. **What the round retro-opened, and what it did not.** Claimed: the
   LINKED-ABILITY MEMORY entry, which the ledger has carried since
   chapter fourteen with Cold Storage named — that card is now a whole
   card on the bench, both abilities, nothing elided (findings 203, 204).
   Claimed: the counter-kind catalog's `Time` row and the counter verbs'
   ZONE, which the "counters as per-holder state" frontier entry called
   half-landed and which is now landed on both zones English writes
   (findings 198, 200). Claimed: the enters-with-counters family's
   ONE-SHOT twin at its second destination — finding 192 opened the
   static "enters with counters" line and this round opens the exile and
   battlefield PLACEMENTS that write the same phrase as an adverbial
   (finding 201). Partly claimed, narrowed: the SUSPEND-adjacent entries,
   which now have every component the keyword's rules text names —
   [CR#702.62a] wants "exile it with N time counters on it" (finding
   201), "remove a time counter from it" while it is exiled (finding
   198), and a trigger on the last counter's removal (which is a
   `GameEvent` row over the counter verbs, unminted and ledgered with its
   counts — nine lines write "time counter is removed", five "the last
   [kind] counter is removed"). The keyword itself stays
   out on instruction, and the components were built for it rather than
   from it. Re-checked and still shut: the PLAYER-borne counters, which
   take a different verb entirely ("target player gets a poison counter",
   forty-eight lines) and which this round's zone widening does not
   touch; the kind-blind quantifiers and counter MOVEMENT, which want a
   quantity over kinds; and the DIVISION of counters, whose `DividedTakes`
   row keeps the battlefield because all forty-six "distribute … counters
   among" lines divide among creatures on the battlefield and the exile
   family writes one card at a time.
210. **What the round measured and returned.** The card-level LINKAGE
   CREATION check is returned, and the reason is a boundary rather than a
   cost. [CR#406.6] and [CR#607.1] require the recalling ability and the
   exiling one to be printed on the SAME object, which is a fact about a
   card's ability LIST and not about any sentence; the workbench could
   ask it, `Card` having carried `text : List Ability` since chapter
   thirty, but only through two deep traversals — one for "does any
   ability here exile", one for "does any ability here recall" — over
   `Effect`, `Noun`, `Predicate`, `Amount` and `Cost`, some hundred and
   ten rows of new table and a totality tax on five more functions for
   every future round. Against that, [CR#607.5a] is the nearest thing the
   rules say about an unlinked reference and it does not call one
   illegal: an undefined linked reference means "that part of the ability
   won't do anything". A gate that refused it would over-refuse, which is
   the one direction this file's gates are not allowed to err in. So it
   is ledgered with its price, and what IS pinned is the part that is a
   fact about the phrase: whose exiles it may name (finding 204) and
   where those cards are (finding 205). Also measured and left: the
   NAMED-source linkage, [CR#607.2n]'s "cards exiled with cards named
   [this object's name]", which is a second linkage rule over a
   before-the-game static ability and wants the printed name read back
   that finding 188 keeps unread (three corpus lines — Volatile Chimera,
   Arcane Savant, Caller of the Untamed); the
   OWNERSHIP relation on an object ("a card you own in exile", which
   Alaundo and Mari both write and which the bench elides), a possessor
   axis distinct from `ControlledBy` and one this zone makes necessary —
   [CR#109.4] leaves an exiled card no controller to be named by, so
   ownership is the only possessor the phrase has left ([CR#108.3]); and
   the plural OWNERS' destination that Mechtitan Core's recall needs ("to
   the battlefield tapped under their owners' control"), which is
   `CtrlSingular`'s standing refusal at a second site.

## Chapter Thirty-Two — The Position of a Gate

Chapter thirty-two, where an outside audit's findings were re-grounded
one at a time and the surviving ones turned out to share a shape
(evidence: Bosh Iron Golem, Erebos God of the Dead, Oblivion Ring,
Thought Reflection, Control Magic, Cloudkin Seer, Beast Whisperer,
Char-Rumbler, Aladdin's Ring, Tamiyo Compleated Sage, Chandra's
Pyrohelix; and, for what the round refused rather than fixed, Rorix
Bladewing, Hymn of Rebirth, Geode Golem, Karador Ghost Chieftain, Demon
of Dark Schemes, Soul of Windgrace. The round that learned a gate has a
POSITION, and that asking the right question in the wrong place is its
own kind of wrong):

211. **A cost announces and never reads, and the demand belongs to the
   POSITION rather than the clause.** The telescope threads each
   component's announcement into the next one's context, which is what
   lets an ability BODY read what its cost did — Bosh, Iron Golem's
   "damage equal to the sacrificed artifact's mana value" is that read
   and has been a positive since chapter twenty-seven. What the threading
   also did was let a SIBLING component read it, and [CR#601.2h] is why
   that is wrong: the player pays in two tiers and each of them "in any
   order", so no component may presuppose another has been paid. The
   obvious repair — strip the deed stamps out of the thread — breaks
   Bosh, because the body reads through the same channel. So the demand
   went on the component's own noun instead (`badCostReadsSiblingDeed`),
   and the same position carries two more: a cost noun may not be
   TARGET-marked, since [CR#601.2c] announces targets while the ability
   is still being proposed and [CR#601.2h] pays at the end of that
   procedure (`badTargetedCost`), and a cost component names its payer
   "you", [CR#602.1a] making the activation cost the activator's to pay
   (`badForeignPayerCost`, `badForeignSacrificeCost`).
212. **The payer demand had to be rescoped, and the count is what
   rescoped it.** Written on the life component itself, the payer rule
   refuses a hundred seventy-seven attested lines — the "unless its
   controller pays" family and its neighbours — because `Pay`'s clause
   accepts exactly the component the activation position does, and that
   clause's whole purpose is to name a payer the sentence chose. So
   `CostPaidByYou` rides the ACTIVATION and the `Pay` clause gets a
   one-directional agreement gate of its own (`badMismatchedPayer`),
   which closes the mismatch between an offer's decider and its
   payment's grammatical subject without touching the named-subject
   family. It is the round's clearest case of a true rule in a false
   place: [CR#602.1a] is about activation costs, and the file had been
   about to apply it to every cost.
213. **A placement onto the battlefield asks what the card IS, and a cost
   never makes one.** `DestOk` had always gated the destination PHRASE
   and never the patient, so a written instruction could put an instant
   card onto the battlefield; [CR#110.4] says instant and sorcery cards
   "can't enter the battlefield and thus can't be permanents" and
   [CR#110.4a] lists the six that can (`badInstantOntoBattlefield`). The
   gate is one-directional on purpose — an untyped patient still passes,
   because Oblivion Ring's "return the exiled card to the battlefield"
   writes the card word and no type, and refusing that would be the one
   direction these gates may not err in. The COST half is a separate
   count: every zone-change verb that opens a cost removes or downgrades
   — sacrifice three hundred forty-one, discard sixty-eight, exile
   twenty-eight, return nine and all nine to a hand, put six and none of
   them to the battlefield — against zero battlefield entries, so that
   destination alone leaves the cost position (`badMoveOntoBattlefieldAsCost`).
214. **Departure clears its evidence.** `eventAfter (Leaves n)` was the
   one zone-change event that did not retag, so the referent kept the
   battlefield tag it was introduced with and a trigger body could tap a
   permanent that had left. [CR#603.6c] settles it: an ability that
   attempts something to the card that left "checks for it only in the
   first zone that it went to", and the sentence does not say which zone
   that is. So the binding survives and its zone does not
   (`badLeavesThenTap`), which is the honest shape — the referent is
   still nameable, and every battlefield-demanding verb now refuses it
   through the demand it already carried.
215. **Zone silence was being read as evidence, and the class word is
   where it leaked.** `AnyTarget` projects no zone by construction —
   that is finding 147's own settlement, the phrase naming [CR#115.4]'s
   damage class rather than describing an object — and `zoneFits`
   treats silence as no evidence and passes. Where a verb's demand IS
   the zone, silence therefore passed as proof: "exile any target" and
   "counter any target" both went through. [CR#115.4] refuses them at
   the phrase, its own last sentence saying other game objects "can't be
   chosen" that way, and [CR#112.1] makes a spell a card on the stack,
   so the counter verb and the cast event ask for the stack STRICTLY now
   rather than through silence (`badExileAnyTarget`,
   `badCounterAnyTarget`, `badCastsAnyTarget`). The determiner half of
   the same verb is measured and left: "Counter target" is a hundred
   ninety-nine lines and "Counter a" is zero, but "Counter all other
   spells" is real, so the row that would refuse the indefinite has to
   keep the universal and that is a determiner table this round did not
   buy.
216. **The multiplicity word tracks the CARRIER, and the table that says
   otherwise was measured on the wrong axis.** Chapter twenty-five keyed
   "if" against "the next time" by EVENT, and the measurement it used
   for the draw row was "if you would draw a card this turn" — zero
   lines. That is a fact about a duration-bounded shield, and it was read
   as a fact about drawing. Twenty-one supported cards print the
   durationless "If you would draw a card, … instead" as a standing
   static line, Thought Reflection among them, and the pattern generalises
   past the draw: "the next time X would Y this turn" occurs nowhere as a
   standing ability — all nine draw instances are the payoff of an
   activated cost or a delayed trigger, and all twenty-five destruction
   instances are regeneration's own reminder. So the cell is flipped
   ([CR#614.1a] making the "instead" clause a replacement effect either
   way), `thoughtReflection` is the positive that replaced the refusal,
   and the pin that asserted the inverse is RETIRED with its reasoning
   left in place. What the two-dimensional table still cannot say is the
   carrier itself, and that is ledgered below.
217. **A conditional wrapper was hiding its payload's mood.**
   `staticKind` answered `Conditional` for the wrapper without looking
   inside, so a control grant that `StaticLine` refuses bare passed the
   moment it was wrapped in "as long as". [CR#604.1] has static abilities
   "written as statements, and they're simply true", and Control Magic's
   statement is "You control enchanted creature" and never "you gain
   control" — the inchoative is the thing being refused, and a qualifier
   in front of it does not change what mood the clause is in
   (`badConditionalGainControl`).
218. **The trigger header's subject is TYPE-ASCRIBED, and the corpus says
   so exhaustively rather than by sample.** Every word following "this"
   in every top-level When/Whenever line was enumerated: creature three
   thousand seven hundred thirty-five, enchantment two hundred forty,
   artifact a hundred thirty-four, Aura a hundred twenty-four, Vehicle
   ninety-two, land seventy-one — and not one bare verb. "When this
   enters" is zero lines and so is "Whenever this enters"; the attested
   spellings are "When this creature enters" at eighteen hundred eighteen
   and its Whenever twin at seventy-four. So the self-subject of an event
   takes the sorted word (`badEntersBareThis`), and the eight bench
   positives that had been writing the bare form were MIS-SPELLING their
   own cards — Cloudkin Seer's line is "When this creature enters, draw a
   card" and the bench said "when this enters". Two more demands ride the
   same header: an ordinary card trigger announces no target, the
   attested targeted form being the delayed carrier Graceful Reprieve
   writes ([CR#115.1d]; `badTargetedDeathHeader`), and the cast event's
   complement is ONE spell, [CR#601.2a] moving a single proposed card to
   the stack (`badCastsPluralComplement`).
219. **A spell card's activated ability is classified by its COST, and
   the keyword cell stopped guessing.** The card cell had been open to
   every activated line on the strength of cycling, which is real; what
   it admitted besides was a tap-symbol ability on a sorcery. [CR#107.5]
   gives "{T}" the fixed meaning "Tap this permanent" and [CR#113.6j]
   says an activated ability "functions from any zone in which its cost
   can be paid" — which for a sorcery card and a tap symbol is no zone at
   all, since [CR#110.4] never makes one a permanent. Zero instant or
   sorcery cards carry a top-level tap ability, and the cell asks the
   cost now (`badTapSorcery`). Beside it, the keyword cell's wildcard is
   gone: `keywordCardOk` spells all six cells explicitly, so a future
   keyword row is a totality error rather than a silent inheritance —
   which is this file's own stated rule applied to the one table that had
   been breaking it. No keyword was added and no spell keyword was built.
220. **The mana run is nonempty and its hybrid halves differ.** A cost
   component may not be an empty symbol run: that spells an activation
   line opening with a bare colon, which the corpus never writes, and the
   CARD-level distinction stays exactly where it was — `[]` is
   [CR#202.1b]'s "no mana cost", the unpayable absence, and "{0}" is
   [CR#118.5]'s payment of nothing, one symbol (`badEmptyManaCost`).
   [CR#107.4f] lists ten hybrid Phyrexian symbols, which is the count of
   unordered pairs of five DISTINCT colors, so the two halves cannot
   agree (`badSameColorPhyrexian`), and [CR#107.4e]'s ordinary hybrid
   "represents a cost that can be paid in one of two ways" — which a
   same-colored symbol does not (`badSameColorHybrid`). One self-tap per
   cost, besides: [CR#107.5] says a permanent already tapped "can't be
   tapped again to pay the cost" and [CR#118.3] refuses a payment the
   payer has not got (`badDoubleTapCost`). That demand sits on the
   ACTIVATION and not on the compound, and for a mechanical reason worth
   recording — an auto-implicit on the constructor is solved before its
   own component list's errors surface, so every malformed component
   started reporting the tap question instead of its own.
221. **The card container gained the two witnesses it never had.**
   Printed power and toughness are SIGNED now: Char-Rumbler prints -1/3,
   `Nat`'s negative literal saturates, and the container had been storing
   0/3 and calling it the card — misrepresenting silently where it should
   have refused. [CR#107.1b] says outright that "it's possible for a game
   value, such as a creature's power, to be less than zero", and
   [CR#208.1] describes the two numbers in the corner; tokens keep their
   unsigned pair, a token's printed values being defined by the spell
   that made it ([CR#111.3]) and never negative. The supertype field had
   carried no witness at all, so a word could be printed twice
   (`badDuplicateSupertype`), and the type line's only cross-check had
   been ORDER, which let a permanent type and a spell type share a line
   because the ranks happened to ascend ([CR#110.4,110.4a];
   `badMixedPermanentSpellLine`).
222. **What the round REFUSED, and on what evidence.** Five audit claims
   did not survive re-grounding, and none of them failed for being
   careless. The COMMAND zone: its absence is a measured decision this
   file already recorded — "zero corpus lines here need it" — and Geode
   Golem's permission wants the commander designation besides, so a zone
   row would have been a hole rather than a fix. Rorix Bladewing's
   "Flying, haste" stored as two abilities: a keyword IS the name of an
   ability ([CR#702.1] — the object "lists only the name of the ability as
   a keyword"), so two names are two abilities, and the bench comment has
   said so since chapter thirty. Hymn of Rebirth "eliding" the card word:
   the phrase says `InZone graveyardZ`, and which zones make an object
   answer to "card" is `isCardZone`, a table this file has carried since
   chapter eleven and one of three carrier-word questions it asks of a
   zone — so the word is ENTAILED by the phrase rather than dropped from
   it, and the elision is in the reader and not the term. A compound
   `Exile` tag blind to its body: `TagBody` has gated it all along, and a
   damage clause under an exile tag is refused. And [CR#707.12]
   contradicting [CR#701.5b] on what casting reaches: the two have
   different subjects — the second defines what casting a CARD means, the
   first extends the casting rules to a copy — and a carve-out is not a
   contradiction.
223. **What the round LEDGERED, with prices.** The CAST permission's
   carrier-relative classifier: "you may cast a creature spell from your
   graveyard" is real (Karador, Haakon) and the spell word forces the
   stack, so the complement's surface classifier and its source have to
   be carrier-relative rather than current-zone-only. The natural
   spelling is writable today — a creature card in your graveyard — so
   what is missing is the prospective WORD and not the family, which is
   why it is a ledger entry and not a leak. The replacement CARRIER
   dimension (finding 216's residue): the table is event-by-word and
   wants a third axis, standing against duration-bounded. The ORDERED
   linearizations: `MoveRiders` spells the entry participles before the
   controller and the three activation restrictions have a fixed slot
   order, and both lose a real alternative — though the measurement
   reverses the audit's emphasis, "tapped under your control" leading
   "under your control tapped" thirteen to eight and its owner's twin
   twenty to two, so a fixed order keeps the majority and loses ten
   lines rather than contradicting the family. The BARE GENERIC PLURAL,
   which `AllOf` cannot spell: three hundred sixteen supported lines open
   with "[Noun] you control" as a bare subject against exactly one that
   writes "All", so the determiner is wrong for very nearly the whole
   family rather than for Anthem of Champions alone. Making `Card`
   ABSTRACT so `MkCard` cannot bypass the four witnesses: the smart
   constructor carries them, the record does not, and closing it is an
   Idris export question rather than a grammar one. And the entry
   event's Whenever cell, which pure-ETB measurement suggests is empty
   for artifact, enchantment and land — the seventy-four creature hits
   were sampled rather than enumerated, and a sampled count is not
   enough to move a table this file measured before.
224. **What the round measured and returned.** Chandra's Pyrohelix is a
   whole card on the bench now, and it went there to settle a
   disagreement rather than to add a witness: an audit had recorded it as
   one count word short of writable, and its line is Forked Bolt's word
   for word — the enumerated range `oneThrough 2` has spelled "one or two
   targets" since chapter twenty-three, so the same term spells both
   cards. The bench is a hundred eighty-seven positives and three hundred
   forty-two pins, and one pin LEFT it: the inverse draw-replacement
   refusal, retired with its reasoning kept in place, which is the first
   time a pin has been withdrawn rather than re-keyed. Also measured and
   left: the `Expose Reveal` and `Mill` cost rows carry a player slot and
   are not payer-gated, wanting the count finding 212 did for the life
   component; and the counter verb's determiner table, whose numbers are
   in finding 215 and whose universal row is what makes it more than a
   one-line refusal.

## Chapter Thirty-Three — The Catalog Sweep

Chapter thirty-three sweeps five vocabularies — the keyword rows and the
possession predicate, the supertypes, the basic land types, the counter
kinds, and the chosen-quality sorts — and then CORRECTS what the sweep
first landed. Its witnesses are whole cards or first sentences of them:
Aerial Volley, Yotian Soldier, Pym Particles, Bladebrand, Critical Hit,
Lightning Blow, Avian Oddity, Song of Eärendil, Demonic Consultation, Void,
the five basic lands, and Snow-Covered Forest.
225. **A catalog row names English; it does not duplicate RON mechanics.**
   The word is this file's and the body is core's, and the two answer
   different questions. `Counter.confers`, the layer system and the
   state-based actions stay RON-side, and so does what a keyword DOES; what
   lands here is the atom, its spelling, and the tables that gate it.
226. **The stat-counter family is a product, not a list.** `Counter.Delta`
   is `Up Nat | Down Nat`, the binding-free twin of `LifeOp`'s amount
   algebra, and `BoostCounter Delta Delta` is one constructor for
   [CR#122.1a]'s "+X/+Y" and "-X/-Y". Twelve distinct pairs are printed and
   +1/+1 and -1/-1 take all but forty-three of the mentions; +1/+0, +2/+2,
   +0/+1, -0/-1 and the other six are representable now without a mint or a
   refusal, where two rows had spelled two of them. `Down 0` is valid and
   carries no proof — "-0/-1" is six lines. The third row the sweep minted,
   `Set Nat`, is gone: [CR#122.1a] does not know the shape and no card sets
   a stat counter to a value, so it was a phrase nothing prints.
227. **A catalog WORD belongs in the macro layer once the vocabulary
   beneath it is a product.** The sweep inlined
   `BoostCounter (Up 1) (Up 1)` at thirty bench call sites,
   which is a product spelled out where a word was wanted. `plusOnePlusOne`,
   `minusOneMinusOne` and `flyingCounter` are macros in
   `Experimental/Macros.idr` and the call sites read as they did before the
   product existed. What a new kind word costs is now a line in the macro
   layer rather than a decision in the grammar — which is the whole reason
   to find the product.
228. **A keyword counter is the keyword product, closed by rule.**
   `KeywordCounter` takes a `Keyword` through `keywordCounterOk`, whose rows
   all answer True because every keyword this file carries is on
   [CR#122.1b]'s closed list of fifteen. The table is a FORCING function and
   not a measurement: a row off that list (banding, phasing) has to answer
   False before a counter of its name can be formed. Forty-five corpus lines
   write one as a one-shot put — Avian Oddity's "put a flying counter on
   target creature you control" is the bench witness — and it is the same
   verb the stat product takes, which is why the two are one type.
229. **A counter NAME sighted in a line is not a row.** The entry-family
   query reports four hundred six supported lines and sixty-three distinct
   kinds. The sweep minted fifty-four flat named rows off that census —
   arrowhead, croak, polyp, reprieve and the rest — with no construction
   transcribed for a single one of them, and all fifty-four are deleted. The
   bar this file already carried is the right one and is restored: a flat
   kind earns its row where a corpus line writes it as a one-shot put or
   remove, which is why `Stun` ([CR#122.1d]; Kaito, Bane of Nightmares) and
   `Time` (chapter thirty-one's exile tail) are rows and charge, oil, fade,
   loyalty, shield and finality are not — their lines are costs, upkeep
   triggers and enters-with riders, which are other axes. The catalog is
   open in core (`CounterRef` into a plugin registry) and it stays open
   here; a census is a ledger entry, not a mint.
230. **Possession is not granting, and STANDING is core's call.**
   `HasKeyword` is [CR#702.1d]'s "with [keyword]" / "that has [keyword]":
   non-head, no type or zone projection, negatable — "each creature you
   control without flying" (Song of Eärendil) — and contradiction-checked
   against its own negation. Aerial Volley is the whole card. The vocabulary
   it ranges over is SEVEN rows and not the fifteen the sweep minted, and
   the deciding evidence is core's own setup rather than any list of words:
   `Trample`, `Vigilance`, `Deathtouch`, `DoubleStrike` and `FirstStrike`
   have NO macro under `plugins/builtin/macros/keyword/` because the engine
   implements them itself, while hexproof, indestructible, lifelink, menace,
   reach, shadow and exalted each have one, and decayed is composite by its
   own rule ([CR#702.147a] — "a static ability and a triggered ability").
   A composite is the queued composite carrier's to spell, never a row of
   its own. Haste and Flying are the two composite rows here, carried in
   from earlier chapters. Bladebrand, Critical Hit and Lightning Blow
   witness the three new rows in the grant slot `gainsHaste` already had,
   and Yotian Soldier prints vigilance as a bare line.
231. **Selection and readback are separate catalog decisions.**
   `QualitySort` gains `CardName` and `Number`: "Choose a card name." is
   Demonic Consultation's first sentence, thirty lines writing it and
   thirteen more with "nonland", and "Choose a number." is Void's, thirteen
   lines. The READ is a different question and the measurement settles it —
   "of the chosen color" is forty-four lines and "of the chosen type"
   ninety-one, while "of the chosen name" and "of the chosen number" are
   ZERO, those two sorts reading back instead as "with the chosen name"
   (thirty-seven) and "equal to the chosen number" (eight). So
   `chosenQualityReadOk` is a full-row table with two False rows that refuse
   nothing anyone prints, `OfChosen` carries its witness, and [CR#607.2d] is
   the linkage the later equality reads will answer to.
232. **Basic is a supertype; Forest is a subtype.** [CR#205.4a] closes the
   supertypes at five and [CR#205.3i,305.6] name the five basic land types,
   and both are catalog words rather than rules-fixed structures. `Basic`
   and `Snow` join `Supertype` on witnesses — the five basic land cards, and
   Snow-Covered Forest, whose printed "Basic Snow Land — Forest" is also the
   proof that one line carries two supertype words where it may not carry
   one twice (`badDuplicateSnow`). World and Ongoing stay unminted. The five
   land subtypes answer Land in `subtypeType`, which is what makes
   `badForestNonland` refuse. No `HasSupertype`, no basic/nonbasic
   predicate, and no chosen-land-type surface was added. Counts: "basic"
   four hundred thirty-nine lines, "snow" a hundred twenty-one, "basic land"
   four hundred thirty-seven, "nonbasic land" sixty-eight, "basic land type"
   ninety-three.
233. **What the correction removed, and the rule it was made under.** The
   sweep read "thorough" as "enumerate the catalog", and the ruling (the
   user, 2026-08-08) is that a round like this owes the basic structure and
   the primitives, so that everything else can be authored as macros later;
   deleting what was extra is part of the delivery, not a loss. Deleted on
   that ruling: `Set` from `Counter.Delta`; fifty-four flat `CounterKind`
   rows and the `counterCatalogWitnesses` atom list that was their only
   bench presence — a term that transcribed no line and still counted as a
   positive; eight composite `Keyword` rows with their `sameKeyword`,
   `keywordCounterOk` and `keywordCardOk` cells (`keywordCardOk` had scaled
   to thirty cells for words no card here writes, and is fourteen now); and
   two pins that were clones of pins already standing, `badVigilanceInstant`
   of `badKeywordOnInstant` and `badDuplicateBasic` of
   `badDuplicateSupertype`. Restructured: the two stat words, out of the
   grammar and into macros. Kept: the two products, the possession
   predicate, the supertype and land-type rows, and the chosen-quality gate.
   The bench is a hundred ninety-nine positives and three hundred forty-eight
   pins; the six pins this round adds each fail for their own reason under a
   single adversarial pass, six expected and six observed, and the build is
   clean.

## Chapter Thirty-Four — Object Nouns and Status

Chapter thirty-four gives the grammar the object words that are not card
types — "permanent" and "token" as heads and readback words, with the
token's provenance becoming mention state — and the [CR#110.5] status
product with its description word, the untap effect, and the standing
untap-step lock. Witnesses: Aerial Assault, Asphyxiate, Aphetto
Alchemist, Time Vault, and Vindicate for the permanent head; the token
head lands pin-backed, no bare-head line being attested (ledger L3). The
source family was measured
and deliberately NOT minted (finding 239). The round's recon also
re-merged the curated gnarly aliases at forty-four cards against the
stated forty-three, an inventory discrepancy held open rather than
silently resolved (ledger).

234. **"Permanent" is one predicate row; the zone picks the carrier.**
   `Permanent` is the [CR#110.4a,110.4b] type-set as a noun head: bare, it
   defaults to the battlefield ([CR#109.2]) and names [CR#110.1]'s
   permanent ("target permanent", three hundred forty-six lines); under a
   card-zone clause it is the "permanent card" Eureka moves (two hundred
   seventy-three lines); under the stack, the "permanent spell"
   (sixty-one). It projects no single type and seeds no zone of its own —
   which is what lets Aether Helix write both senses side by side in one
   vocabulary — and the type-set is `permanentType`, the table the
   placement gate has read since chapter thirty, now given its noun
   surface. Not a seventh `CardType`, and not a zone synonym: four
   hundred thirty "permanents you control" lines modify it freely.
   Battle and planeswalker sit outside the six-row catalog, so the
   writable set is four types of [CR#110.4a]'s six (ledger, by name).
235. **The permanent head extends the contradiction scan.**
   `contradictionFree` now also refuses a phrase that couples the
   permanent word with a projected non-permanent head type: [CR#110.4]
   rules instants and sorceries out in as many words, so "permanent
   instant card" describes nothing (`badPermanentInstant`). Read off
   `seedTy` through `permanentType` — no new table, one new reader.
236. **`PermanentW` reads current state on the battlefield and events
   through the stamp.** The word's `wordNow` is `onFieldZone` and nothing
   else ([CR#110.1] gives and takes the word with the zone —
   `badThatPermanentDeparted`); its participial row asks only that the
   referent stood on the battlefield at the stamped event, the `TypeW`
   row minus its type demand (Broadside Bombardiers reads "the
   sacrificed permanent" through a possessive). "That permanent" as a
   current-state demonstrative was not measured this round and rides
   along as tolerated over-generation, noted not gated.
237. **Token provenance is mention state, written once.** `ObjectP`
   carries a fourth field, `Maybe Origin`, and `Origin` is ONE row:
   `TokenOrigin`, written only by the create clause's introduction
   ([CR#111.1]) and preserved by the zone retags. The card-born
   complement has no reader — "nontoken" is a description-side negation,
   never a readback word — so no row is minted for it. `TokenW` demands
   the battlefield AND the origin ([CR#111.7]; `badThatTokenOfCard`),
   which is Fire Navy Trebuchet's "Sacrifice that token" chain in
   vocabulary, though that card's own header is unwritable (its subject
   is "this Vehicle", a subtype ascription `AsType` does not spell). A
   DESCRIBED token mention ("target token … that token") does not yet
   seed the field — the description-to-origin projection is ledgered
   with its measurement, not guessed.
238. **"Token" heads; "nontoken" negates; creation keeps its compound.**
   `IsToken` is a head ("a token", four hundred one lines; "target
   token", fourteen), seeds the battlefield ([CR#111.7]), and negates as
   the two-hundred-twenty-two-line "nontoken" family (Lich). The
   creation-side compound stays `TokenChars` ("creature token"), and the
   ordering zeroes are the closed fact: token-before-type is written
   zero times, so no second spelling is minted.
239. **Source is a role with no landing site, so no row was minted.**
   The recon's counts are real — sixty-three "source of your choice"
   lines, fifty "source you control", thirty-seven "source would deal",
   eighteen "that source", ZERO "target source" — and [CR#109.2c,120.7]
   make sourcehood relational and cross-zone: eligibility comes from the
   enclosing damage, ability, or mana relation, not from a zone test
   either carrier check could answer. Every measured consumer lives in
   constructions this grammar has deliberately ledgered — the
   prevention by-source rider ([CR#609.7], the `Prevents` row's own
   ledger note) and the damage redirection family — so a source head or
   readback word today would be vocabulary with no consuming
   construction, exactly what the primitives-first ruling deletes. The
   family is recorded PARTIAL: the by-source rider is the named landing
   site, and the noun row lands with it.
240. **The status product is closed by rule, and the surface is a
   measured table.** `StatusCat`/`StatusVal` spell [CR#110.5]'s four
   categories of two values with the category as the index — one value
   per category is a fact of the representation, a new category is a
   totality event, and status is not a characteristic ([CR#110.5a]) nor
   anything an off-battlefield card has ([CR#110.5d]). The product is
   persistent mention state because [CR#110.5c] retains status until a
   spell, ability, or turn-based action changes it. `statusWordOk` is
   the description surface's attestation: tapped/untapped and face
   up/face down write; "unflipped" and "phased-in" are zero everywhere,
   and the lone "flipped" line and twenty-two participial "phased out"
   lines are unclassified frames — four refusing cells that refuse
   nothing anyone measured, held for a classifying recon (ledger). The
   entry defaults ([CR#110.5b]) stay the entry riders' and the engine's.
241. **The status word is a non-head, type-neutral, battlefield-seeding
   modifier.** `HasStatus` joins the predicate tables the way `Attacking`
   did, minus the type presupposition — [CR#110.5] holds of every
   permanent, and the corpus writes tapped artifacts, lands, permanents,
   and tokens beside its hundred eighteen tapped creatures. It seeds the
   battlefield ([CR#110.5d]; `badTappedGraveyard`), heads nothing
   (`badBareTappedHead`), does not negate — the paired values are each
   their own word (`badNonTapped`) — and same-category values contradict
   through the new `statusClash` scan while cross-category values stack
   (`badTappedUntapped`; a tapped face-down morph is writable). The
   chapter-eighteen condition row's comment had already counted "if
   it's tapped" (two lines): `Matches` composes it now with nothing
   further minted.
242. **Untap is tap's paired effect row; legality stays the engine's.**
   `Untap` mirrors `Tap`'s zonal demand ([CR#701.26b];
   `badUntapGraveyard`) over a hundred fifty "untap target" lines,
   Aphetto Alchemist's "{T}: Untap target artifact or creature." the
   witness — the {T} cost and the untap effect in one line, two surfaces
   of one tapped axis kept apart as `Cost` and `Effect`
   ([CR#107.5,107.6]). That tapping takes an untapped permanent and untapping a
   tapped one ([CR#701.26a..701.26b]) is the resolving engine's fact:
   the sentence is well-formed either way, so the grammar's demand stays
   zonal, the answer to the recon's enforcement-layer question.
243. **The untap-step lock is a statement, not a deed denial.**
   `DoesntUntap` spells the hundred-twenty-one-line "doesn't untap
   during … untap step" frame ("does not untap": zero). The untap-step
   untap is a turn-based action in which the active player determines
   which permanents they control will untap and untaps them all
   simultaneously ([CR#502.3,703.4c]), so the lock restricts a step's
   transition rather than denying a deed a type grants — no `Deed` row,
   no [CR#506.3] grant gate. It files under `DeedRestriction`, the
   `EntersWithCounters` precedent: one class of statement per every
   kind-keyed reader — standing line yes (Time Vault), durationless
   clause no (`badUntapLockClause`), and the restriction spans, "for as
   long as" being the measured one (Ty Lee's trailing adverbial). The
   step's possessor is agreement, not a slot: a permanent untaps only
   during its controller's untap step, so the spelling writes "your" for
   the self subject and "its controller's" otherwise. Ledgered, with
   counts: the one-shot "next untap step" variant, the single
   non-"during" line, and the seven-line "can't untap" active spelling,
   none of which this row claims. Totality also gives `Untap` the
   Tap-shaped `heldUntilOk`, `costActionOk`, equality, introduction,
   announcement, and deed-delta rows; `reflexEncloseUse` is
   `EncUnattested`, and `DoesntUntap` needs no rows beyond the three
   designed static tables because `notConditional` already catches it.
244. **Measured and deliberately deferred.** Three families this round's
   recon measured stay unminted, each for a stated reason. The
   becomes-status EVENT ("becomes tapped" a hundred lines, "becomes
   untapped" thirty-three, every other value zero) has a designed shape —
   an `EventName` row with `TriggeredOnly` use and a per-value
   attestation gate, `BeginningOf`'s pattern — but the recon's only
   quoted line (Agent Maria Hill) carries a "to pay a … cost" rider this
   grammar cannot write, so the row waits for a clean witness rather
   than landing unexercised. The face/flip/phase one-shot transitions
   ("turn … face up" two hundred twenty, "phases out" thirty-five
   finite) entangle morph's [CR#708] permission machinery and an
   intransitive verb frame the effect layer has no precedent for; the
   phasing tail ("as though it doesn't exist", fifteen reminder lines,
   [CR#702.26]) is expressly out of scope. And "enters untapped" (six
   lines) is a real override of [CR#110.5b]'s default with no quoted
   witness line, so `TokenRider` keeps its two rows. Census recorded,
   structure withheld — chapter thirty-three's rule applied before the
   fact instead of after it. The bench stands at two hundred four
   positives and three hundred sixty-one pins; the thirteen pins this
   round adds each fail for their own reason under a single adversarial
   pass, thirteen expected and thirteen observed, and the build is clean.

## Engine-Boundary Deferrals

Engine-boundary deferrals (deliberate, and to stay so): the
workbench spells the ENGLISH; committed event structure is
core's. The per-combatant fight fact is the type case — core
commits `Fight` once per subject, a self-fight being ONE subject
([CR#701.14a,701.14c]; `deckmaste_core/src/event.rs`,
`plugins/builtin/macros/filter/Fight.ron`) — and the clause here
stays event-silent until triggers and the "this way" reads
consume it. The same boundary holds the damage pipeline
(prevention and replacement), trigger firing and state-based
actions, and every runtime magnitude (§3: sorts stored, values
never).

## Not Settled Yet

Not settled yet: the kind union ("any target" spans objects and
players [CR#115.4,115.1] — elided to `Object`). The elision's LEAK
is closed, and by fixing a projection rather than patching a gate:
the class word describes no object, so [CR#109.2]'s battlefield
default has nothing to place and the phrase projects no zone,
which is how every battlefield-demanding verb comes to refuse it
through the demand it already carried (`badDestroyAnyTarget`,
`badTapAnyTarget`); the binding the mention introduces records the
same silence, so reading it back inherits nothing better
(`badDestroyAnyTargetRemention`). Damage keeps the phrase by a
recipient row of its own — `DamageRecipient` asks the NOUN now, as
`DiscardOk` does — which is what a zone-free object row could not
have done without admitting bare `This`, the source as an object
(`badDamageThis`). What is left is the union proper, and it is a
KIND question rather than a zone one: `AnyTarget` is typed
`Predicate bs Object`, so the mention binds an OBJECT, the reads
that reach it are the object pronouns ("it", never "that
player"), and its recipient row asserts damageability without
saying which kind was chosen. Core settles none of this by
example: it has no kind index at all — one `Reference` spanning
both, with `DealDamage(Reference, Count, Reference)` taking the
same type in the source and patient slots
(`deckmaste_core/src/action.rs`, `reference.rs`: "Players are
objects") — whereas the kind index HERE is what buys the anaphora
discipline (uniqueness counted per kind). So the union wants `Or`
at the predicate level and a kind join the reads can project
through, not the deletion of kinds — which is what the `AnyTarget`
constructor has claimed since it was minted. Half of that has
arrived: chapter fourteen built `Or`, and settled that the class
word is expressly NOT one of its alternatives, [CR#115.4]'s union
being closed already (`badAnyTargetInOr`). What is still missing
is the JOIN: `Or`'s alternatives share one `Kind` index by
construction, so "target player or planeswalker" (Chandra Nalaar)
cannot be written as a disjunction of a player description and an
object one, and the reads have no joined kind to project through
— the union proper, now stated as the one thing it needs;
owned-zone PREDICATE mentions beyond `You` ("a card in an
opponent's graveyard" — [CR#400.3] makes the possessive an owner
FILTER, finding 34; the inner noun does not fold yet); controller
fold-state (the controller half of verb restrictions, entangled
with [CR#109.4]; sharpened by audit — `ControllerOf` accepts a
graveyard-introduced object, but [CR#109.4] gives off-battlefield
objects no controller, and last-known information preserves only
what existed ([CR#608.2h] is zone-general, not battlefield-only —
the audit's correction), so the read wants zone/provenance
evidence); [LANDED, findings 128–129 — the
library zone and its ORDERED positions. The order turned out to be no
part of the zone SORT at all: [CR#401.2] makes the pile ordered and
[CR#401.4] makes a placement's arrangement a property of the placement,
so `ZoneAt Library` is the zone whole and `LibraryAt pos ord Bare` is a
place in it, with the bare library refused as a destination exactly as
core's `exclude(Library)` refuses it. What is STILL shut is the ORDINAL
offset — "into its owner's library second from the top" (ten lines) and
"third from the top" (fourteen), [CR#401.7]'s own subject — which is a
different PHRASE rather than a third position word, and wants an ordinal
numeral vocabulary this file does not have; core spells all three with
one indexed row (`Anchor = FromTop(Count) | FromBottom(Count)`)];
miracle's reveal condition ("the first card you've
drawn this turn" [CR#702.94a] — a turn-scoped draw-ordinal
memory, and now shut on THAT rather than on the library, whose
vocabulary chapter twenty-four supplied; the keyword wrapper is the
other half); the up-to-one singular
remention positive (Ty Lee, Chi Blocker — its second sentence is
the UNTAP deed under a "for as long as" duration ([CR#611.2b]), so
it waits on both, chapter fifteen's restriction clause having
landed with neither); [LANDED, finding 73 — `May`'s if-you-do /
if-not branches are two `Maybe` slots on the clause, and the
declined arm reads only what preceded the may]; [LANDED, finding 165 — the MANDATORY
"if you do" is `May` with its OFFER emptied, not a second row.
[CR#118.12] states both shapes in one sentence and gives them one
reader ("chose to pay an optional cost or STARTED TO PAY A MANDATORY
COST, regardless of what events actually occurred"), so the arm
asymmetry and the opacity carry over untouched and the decider slot
became a `Maybe`. The bare instruction's "if you do" takes its pronoun
from the body's own agent; Woeleecher is the positive]; [LANDED, finding 88 — the ELSE sentence
"Otherwise, …" is `If`'s third slot, a `Maybe` field typed in the
discourse BEFORE the conditional and contributing nothing outward,
opened on Unholy Annex once tokens and counters made both of a
card's arms writable; only the leading linearization spells it]; the LEADING conditional that
announces a target its consequent reads ("If target creature has
toughness 5 or greater, it gets +4/-4 until end of turn", Blood
Lust; Hidetsugu's Second Rite; Meddle — ten "if target" lines,
four sentence-initial), which is legitimate by [CR#601.2c] and
needs a delta carrying only a phrase's target half. This is the
target-announcement channel's REMAINING half: chapter twenty-five
opened the other one (finding 146 — `preIntro` of a conditional now
exports what its clause announced, which is what `InsteadOf`'s
replacement reads), and what is still shut is the delta INSIDE a
condition's own subject phrase, which is a different demand;
the whole-card witnesses of the opened half are ledgered by their
blockers rather than by the channel — Overload and Prohibit want
kicker, Fatal Push revolt, Anoint with Affliction poison counters,
Welcome to the Fold madness, and Ana Sanctuary, whose blocker list
SHRANK in chapter twenty-eight without emptying: its trigger shell and
its intervening "if" are built now ("At the beginning of your upkeep, if
you control a blue or black permanent, …"), so what remains is the
permanent WORD, a colour-quality predicate, and the conjunction
`Condition` has no frame for; the STRICT
comparators in the condition frame ("if your life total is less
than 7"; fifty-one "is less than" lines, eighteen "is greater
than", almost all against a phrasal standard) together with the
life-total reader they mostly measure, which `Characteristic`
does not carry and which would make `Comparator` a per-frame
table; [LANDED, finding 95 — the CONDITION negation is
`NotCond`, over a closed table of the frames English negates:
the existential and the reference frames, not the comparison
frame and not another negation. The wait's own count was short by
a factor of five]; [HALF LANDED, finding 166 — "unless"
whole. The action form needed no new structure, exactly as chapter
eighteen recorded: it is [CR#118.12a]'s rewrite onto `May`'s `ifNot`
arm, and what it waited on was the cost vocabulary. Two hundred
forty-one lines pay in the SECOND PERSON and land (Molting Harpy,
Carnophage, Solitary Confinement — the pay-mana, pay-life and action
frames). What is still shut is the ANAPHORIC PAYER: a hundred
twenty-five "unless its/their controller pays", forty-three "unless
that player pays" and nine "unless any player pays" name their payer
by reading the MAIN clause, and the rules' own rewrite is not
linearization-preserving — it puts the may first, so the payer phrase
is typed before the clause that announces what it reads
(`badUnlessAnaphoricPayer`). That blocker outlives the stack
vocabulary those lines mostly also want. The STATE form is unmoved and
still a negated condition whose carriers are entering-tapped
replacements and durationless static deontics (sixty-two "unless you
control", a hundred eleven "can't … unless"), and the conditioned
deontic should re-use `Condition` inside `Cant`'s shape as core does
with `StaticEffect::Conditionally`]; "if able" (two hundred fifty-one
lines, fifty-eight of them "attacks each combat if able"), which
is the deontic-obligation polarity core keeps beside `Cant` and
not a condition at all; the chosen-OBJECT
definites, both numbers (finding 98: "the chosen creature" thirty
lines, "the chosen card" twenty-six, "the chosen permanent" ten,
"the chosen creatures" eight, "the chosen player" twenty-nine —
`TheVerbed`'s frame exactly, and shut because no card writing the
singular read is writable in this vocabulary, every candidate
needing the plural definite, mill, kicker, teamwork, copies, or an
X counted off artifacts. Opening it widens `VerbName` past the
[CR#701] keyword-action tags it holds today, and the PLAYER read
needs provenance on the binding rather than inside `ObjectP`;
V.A.T.S./Victimize wait on "any number of" groups and if-you-do
besides); plural participle reads
("the exiled cards", Hide on the Ceiling — group twins of
`TheVerbed`); the PERMANENT word (demonstrative "that permanent"
and participle "the sacrificed permanent", Broadside Bombardiers —
the before-state question and its "or" disjunction; no bench card
spells either, so `NounWord` waits to grow it); the
additional-cast-cost juncture (Fling — "the sacrificed creature"
across a casting cost, the same public-survivors discipline as
the colon, constructor unminted); [LANDED, finding 159 — the cost
GRAMMAR. The colon takes a `Cost` now, whose action row is gated by a
per-VERB table measured off the corpus rather than by a blanket rule,
and core's `Action::is_cost_eligible` names the same verbs from the
engine side. One part of this entry was wrong and is corrected in
place: "you lose N life" IS the same clause as "Pay N life" —
[CR#119.4] says paying life is losing that much life in as many words
— so the distinction is the FRAME ([CR#118.1]) and not the verb, and
`Do (ChangeLife who (Down n))` spells both]; hidden-zone identity (a move into
hand keeps its binding readable, but [CR#400.7] mints a new
object and [CR#400.7j] lets the effect re-find it only in a
PUBLIC zone — introduction-in-hand via predicate stays legal,
retention across a hidden-bound move must not; wants a
trackedness distinction the payload does not yet carry); [LANDED in the
vocabulary, findings 171 and 173 — the upkeep query and the
leaves-the-battlefield query are `BeginningOf Upkeep (Just Yours)` and
`Leaves` now, and the delayed clause reads both; the end-of-combat one is
still missing its TURN PART, "At end of combat" being the end of combat
STEP's beginning ([CR#511.2]) at eleven lines. What the named cards want
is unchanged and is elsewhere: Mirror Match and Kjeldoran Elite Guard on
tokens and an unknown-zone retag, Slaughter Pact on a lose-the-game
clause]; the player pronoun "they"
(corpus-attested only inside trigger and unless clauses — Havoc,
Tergrid's Lantern — so `They`'s positive waits on those
constructions), player groups ("each opponent … they"
distributives); [LANDED, findings 132–133 — "the rest", the
subtract-a-subset half of finding 111. What unblocked it was the GROUP
and not any counting: a slice phrase assembles one, a partitive takes
members out of it, and the complement is what is left. Five hundred
ninety-nine of the six hundred forty-eight lines are that frame; the
other families are measured and stay shut]; event-outcome residues (chapter
ten built the scalar "that much" read; "that many" count reads
wait on their consumers — draw, tokens, counters are the
corpus's big three — "this way" participant-subset participles
are the `TheVerbed` cousin and the largest family, and more
sorts wait with them: mana produced, Sakiko; card counts,
Asmodeus; and condition-supplied scalars — Tellah, Great Sage's
mana SPENT is not a clause outcome at all, so it arrives with the
conditions axis); [LANDED, findings 122–126 — the whole plurality
and distribution axis. The "any number of" target group has its
verified whole-card positive (Boulderfall), the PLURAL
damage-class forms [CR#115.4] names have both their structures
(`Distribute` for the division, `EachOf` for the each-of
recipient), the counter clause takes the each-of recipient beside
damage (Ajani, Adversary of Tyrants), and output plurality is
derived from agent distributivity as well as per-agent amount
(`outputPlur`). `AnyTargetAtCount` re-keyed from a maximum to a
MINIMUM in the process: the class word's plural spellings all
leave the count to the caster, so the one refusal left is the
fixed plural count. What is still shut is the VARIABLE bound —
"each of up to X target creatures" (nine lines) and "deals five
times X damage to each of up to X targets" (Crackle with Power)
want a range whose endpoint is an `Amount` rather than a `Nat`,
which is core's `Count`-valued range and this file's `Nat` one,
and no gate stands in the way of it: it is a widening of
`Quantity` and belongs with the other variable-magnitude work.
Elephant Resurgence's own second sentence is likewise still shut,
and doubly — it grants a quoted characteristic-defining ability,
and its token writes no P/T because that ability supplies one
(see finding 122)];
counted UNTARGETED groups ("four lands you control" — Burning of
Xinye's subject-destroy positive waits on them); the
reciprocal fight frame ("those creatures fight each other" — one
exactly-two-membered plural subject, ten corpus lines, a distinct
construction from binary `Fights`; the blocker is NARROWED again
by chapter twenty-three and stated as two named halves rather than
one wall. The group MENTION now exists as a first-class thing —
`GroupMention` is the phrase a sentence may reach into, and
`EachOf` reaches into one — so what the pair complement is missing
is no longer "a group to subtract from" but (a) an ELEMENT-scoped
context, which is the fronted iteration clause "For each of
[group], …" (thirty-seven corpus lines as counted then; REMEASURED in
chapter thirty-two at twenty-five supported fronted, fifty-one
any-position; core's `OneShotEffect::Each`) and which `EachOf`
deliberately does not
provide, being a determiner that binds no element, and (b) a
complement whose domain is that group mention rather than a
description, which is finding 111's subtract-a-subset half
unchanged. Sorrow's Path is the operative-text witness that both
halves are real English and that they arrive together — "Choose
two target blocking creatures controlled by the same opponent. If
each of those creatures could block all creatures that THE OTHER
is blocking, remove both of them from combat." The binary fight
does NOT reach this even so, and finding 127 is why); union-read plural possessors
("creatures your opponents control", corpus-attested — waits with
the player groups); [LANDED, finding 113 — the
self-exclusion "other" anchor. The diagnosis here was right and
the remedy pointed the wrong way: what was needed was not for the
source to ENTER the discourse but a phrase that does not search
the discourse at all, and `OtherThan` carries its anchor as a
`Bindingless` noun. Brash Taunter and Polukranos, Unchained are
the fight family's witnesses; War Screecher is the plural one];
static "as long as" conditions ([CR#611.3], Kitesail Corsair —
the card has no "for"; the rest of eleven hundred thirty-five
"as long as" lines once the two hundred ten "for as long as" are
taken out, and a static-ability container rather than a duration)
and [LANDED, finding 118 — effect-created "for as long as"
durations ([CR#611.2b]). Finding 75's measurement was right in
every part: the row EXTENDS chapter seventeen's `Duration`, the
condition half already existed, and what it waited on was a
clause. The clause is `GainsControl`, and the forty-two control
grants are what opened it — Mind Flayer is the bench witness and
[CR#611.2b]'s own example (Master Thief) is the same sentence.
The cost was indexing `Duration` by the discourse and moving it
into the mutual block. What is still shut behind other vocabulary
is the rest of the two hundred and ten: There and Back Again's
single-deed line still needs a type word `CardType` does not
have, the four keyword grants all grant "indestructible", and the
stat deltas all end "for as long as this artifact remains tapped"
— a per-object status word the object-status axis owes]; [LANDED, finding 144 — the EVENT-ended
endpoint, and the entry was mis-filed. The ninety-one lines are two
constructions wearing one phrase: eighty-six are [CR#610.3] one-shot
zone changes that schedule their own undo and three are the [CR#610.4]
phasing twin, neither a continuous effect at all, so the writable half
is a clause RIDER (`HeldUntil`) and not a duration. The three that ARE
[CR#611.2a] continuous effects — two base-TYPE settings and one
becomes-a-copy — have no clause here, so `Duration`'s `UntilEvent` row
is minted and answered `Unclaimed` (`badGetsUntilLeavesBattlefield`).
Core's single `Duration::UntilEvent` is the runtime's cut and is
recorded as a divergence rather than followed];
the DURING-scope adverbial ("During target player's next turn",
Gideon and Mindslaver), which is not an endpoint at all but a
window the clause holds inside, and which pairs with the
player-subject restriction below ("Each opponent can't cast
spells during that player's next turn"); the NOMINAL duration
possessor ("until its controller's next untap step" — the one
corpus line whose endpoint is possessed by a noun instead of one
of `Whose`'s two words, and whose clause is a base-type setting,
so it waits on the layer word as much as on the possessor,
finding 68); last-known VALUES (reads ignore zone — finding
19 — but [CR#109.4] gives off-battlefield objects no controller,
so the value story belongs to the ability layer); Token / Spell
/ stack-object / Amount carriers ("that much"; bare `This` stays
untracked, and "this spell" / "this card" carriers with it); "the
chosen [quality]" [PARTLY TAKEN, finding 231 — all four sorts bind and
select; `OfChosen` remains Color/CreatureType only, while CardName and
Number await equality-specific reads]; coordinated verb
COMPLEMENTS ("deals 2 damage to any target and 1 damage to any
other target", Arc Trail — one verb distributing over paired
amount+recipient complements; the bench transcribes them as a
two-clause `Sequentially`, a named stand-in that mis-orders nothing
binding-wise but serializes what the card states as one
instruction); amount EXTRAPOSITION (`DealDamage` fixes
amount-before-recipient order while oracle writes both — "deals
damage equal to its power to target creature" against "deals damage
to any target equal to the mana value of the discarded card",
Pyromancy; amounts introduce no bindings, so the fixed order is
binding-neutral and the divergence is linearization's, like the
coordination above); event-history restrictive clauses ("that were
put there from the battlefield this turn", Continue? — no
vocabulary, so `continueSpell` elides the clause, which widens the
domain it expresses until the axis arrives); and CONTROL assignment
on battlefield moves ([CR#110.2a] defaults an instructed put to the
instructed player, so Cloudshift's "under your control" is the
derivable default, while "under its owner's control" is an OVERRIDE
the vocabulary cannot spell — four positives elide it, each named
meaning-carrying in its comment: Turn to Mist, Flickering Spirit,
Graceful Reprieve, Voyager Staff); and the rest of the DEONTIC
surface, chapter fifteen having taken the one-shot restriction
clause and nothing else. [The STATIC form LANDED, finding 169 — the
durationless "can't" is `Ability.Static` over the same `Cant` row, and
Glacial Chasm's "Creatures you control can't attack." is the bench line.
The May-side polarity landed with it in ONE shape only, finding 177:
the play permission ([CR#604.6]) is a `StaticEffect` row of its own, and
the Must and Gate polarities core keeps beside `Cant` are still the
requirement family ([CR#508.1d,509.1c]), arbitrated rather than
subtracted, at fifty-eight "attacks each combat if able" lines.] Beside it: deed
COORDINATION, one "can't" over two deeds ("Target creature can't
attack or block this turn.", six lines, and the detain family's
cross-turn form — `Or` coordinates PREDICATES, and a deed is not
one); PLAYER subjects ("Target player can't cast spells this
turn"), which want the cast deed and a player-subject clause, the
kind index making them structurally unwritable today; the deeds
with no clause yet — "can't be regenerated" (one hundred forty-two
lines, and usually with NO stated duration, so it wants
[CR#611.2a]'s end-of-game default in a span slot the restriction
row is answered `False` for — `absentOk` would have to say yes for
that deed and no for these, which is a per-DEED answer the table
does not have an axis for yet), "can't be countered"
(thirty-three), and
"doesn't untap" (one hundred twenty-two, Ty Lee above) — each
needing its own `deedType` row and the last two a subject this
vocabulary cannot describe; the EXCEPT rider ("can't be blocked
except by Walls", one hundred ninety-one lines), a restriction
carrying an exception clause rather than a plain one; the other
deontic polarities (core keeps `May`, `Must`, and `Gate` beside
`Cant` — requirements are the "attacks if able" family
[CR#508.1d,509.1c], arbitrated rather than subtracted); and the
bare GENERIC PLURAL subject ("Creatures without flying can't block
this turn.", Falter), which is not the "all" determiner `AllOf`
spells and has no noun of its own.
Chapter nineteen's own deferrals, each measured: token COPIES
("a token that's a copy of …", three hundred five lines; four
hundred twenty-one token lines mention a copy at all), which is
core's `TokenSpec::Copy` over a whole `CopySpec` and an axis of its
own — the exception riders ("except it's not legendary") and the
becomes-a-copy static ride it too; PREDEFINED token names ("create
a Treasure token", six hundred thirty-seven lines, [CR#111.10]),
core's `TokenSpec::Named` resolving through a registry, which is
the same open-catalog shape the subtype and counter-kind ports
below have and wants a token whose defined abilities this
vocabulary cannot spell; the SUBTYPE and COUNTER-KIND catalog PORTS
themselves (core declares both as open plugin data —
`Subtype { name, types, confers }`, `CounterRef` into a counter
registry — where this file grows witnessed rows, and the two choices are
the same choice made twice; PARTLY TAKEN in chapter thirty-three: the
five basic land subtype rows landed, and the counter-kind port did NOT —
what landed there is two PRODUCTS, finding 229, and the flat catalog is
as open as core leaves it);
token SUPERTYPES
("create Boo, a legendary 1/1 red Hamster creature token", forty-six
lines), a third list on the type line neither of THIS record's readers
witnesses — the CARD's own supertype list landed in chapter thirty at
one row and it landed on the card rather than here, [CR#205.4b] making a
supertype independent of the card type and subtype, so the token's
forty-six lines stay with the predefined-token catalog they arrive with;
the X/X token BODY (forty-seven of the sixty X/X token lines say
"where X is …"), which is an effect-DEFINED X rather than the
announced cost variable [CR#107.3a] `XVal` carries, so it waits on
a where-clause construction; the TOKEN carrier word ("Sacrifice
that token", Fire Navy Trebuchet's second sentence), which is a
`NounWord` row and waits with the permanent word; counter
MOVEMENT and the kind-blind quantifiers (five hundred forty-four
lines write a move-counters phrase; "remove all … counters", fifty;
proliferate, seventy-seven), which core keeps as its own
`MoveCounters` action over a `CounterSpec` whose `AllKinds` row
quantifies over the kinds PRESENT — a quantity over kinds the
written amount vocabulary has no term for; [the WHOLE entry is LANDED as
of chapter thirty and only the counter-KIND catalog remains of it. The
tapped half went in chapter twenty-eight, finding 179 — "enters tapped"
is a hundred thirty-two lines and `Ability.Static` over `EntersRider` is
the row, Abandoned Outpost's "This land enters tapped." being one of the
three bare ones. The COUNTERS half is finding 192's, and the blocker
recorded here was about the wrong container: an `Amount bs` could never
have gone into `TokenRider`, that enum being shared with the unindexed
`TokenChars`, so the line is a second `StaticEffect` row answering the
same `staticKind` and needing no new vocabulary (Workhorse). What is
left is a kind count that is construction-relative, finding 229: the
entry family is four hundred six supported lines over sixty-three kinds,
and what `CounterKind` reaches is now every stat pair and every
[CR#122.1b] keyword name, through two products rather than a row apiece.
The flat named kinds past stun and time are NOT taken and stay here at
the bar finding 229 restores — a one-shot put or remove writes the word,
or the word waits. The ONE-SHOT
twin is finding 191's, a closed rider BUNDLE on `Move` — "onto the
battlefield tapped" three hundred fifteen lines, "tapped and attacking"
nineteen (chapter nineteen's own `ridersOk` trio at a second site), and
"onto the battlefield under [whose] control" a hundred thirty-five, the
control-override entry above — and the forty-three call sites this entry
promised turned out to be zero, a defaulted named implicit changing no
pattern and no site.
"Enters under [whose] control" is attested and tiny: nine lines,
four "under the control of an opponent of your choice", two the bare
"under your control", one the replacement "it enters under your
control instead" that wants the enter event finding 141 measured and
did not mint. The FIELD's SHAPE is settled as of chapter twenty-six and
only its execution waits: one dependent arrival slot on `Move` carrying
a closed rider BUNDLE rather than a scalar, because tapped, attacking,
and the controller override COMBINE and separate `MoveTapped` /
`MoveUnderControl` constructors would multiply the move by its
combinations; gated to battlefield destinations, there being nothing
for an arrival rider to say about any other zone; and the controller
written as an OPTIONAL override rather than a required field, since
[CR#110.2a] supplies the instructed player's control by default and the
hundred thirty-two lines are the departures from it]; the type-SETTING sibling of
finding 86 ([CR#205.1a] — the bare "becomes a 3/3 Elemental
creature", which REPLACES the prior types where the "in addition"
phrase keeps them) together with the base-P/T setting so often
fused to it ("becomes a 5/5 creature"), which are the layer words —
and the COMPOUND of the two is now this axis's named witness:
Tezzeret, Cruel Machinist's "[0]: Until your next turn, target
artifact you control becomes a 5/5 creature in addition to its
other types" is one clause setting base power and toughness AND
adding a type, and it is the only line in the corpus that appeared
to put a cross-turn span on a type addition. Chapter twenty-one
closed that cell (`GrantsAndRestrictions`, finding 104) on the
reading that the span belongs to the compound rather than to the
naked addition, so this card is what the compound construction has
to write when it opens; and the
subtype-headed DAMAGE recipient, refused today because
`HasSubtype` projects no card type for `DamageableTy` to read
(finding 79) — conservative rather than measured, and the first
thing to check if a "deals N damage to target [subtype]" line
turns up.
Chapter twenty's own deferrals, each measured: modal keyword
PACKAGING, which is a cost story rather than a grammar one and is
ledgered by name and count — entwine (thirty-two cards, "Choose
both if you pay the entwine cost"), spree (twenty-one, "Choose one
or more additional costs"), escalate (nine, "Pay this cost for each
mode chosen beyond the first"), the per-mode additional cost
([CR#700.2h]), the pawprint worth-of-modes headcount
([CR#700.2i]), and the "You may choose the same mode more than
once" instruction ([CR#700.2d], six lines); the CONDITIONAL
HEADCOUNT UPGRADE ("Choose one. If this spell was kicked, you may
choose both instead", twenty-six cards, which is why twenty-two
lines write the header with a period instead of the em dash), whose
NODE arrived with chapter twenty-five — it is `InsteadOf` over a
modal, [CR#614.15]'s self-replacement of the headcount — and whose
blocker is now only the conditions, none of the twenty-six being
writable: kicker and the other cost words, the commander and monarch
reads, "as you cast this spell", an exact life total, and a
conjunction `Condition` has no frame for; "Choose up to
one —" (seven cards, every one over exactly two modes), which the
quantity vocabulary already spells (`upTo 1`) and which has no
macro or positive because not one of those seven has both modes
writable in this vocabulary; the AMASS seam of finding 97, wanting
either a hedged-mention channel (a binding introduced under a
condition, which the definite reads pass over when an unhedged
candidate exists) or a co-reference channel, both of them §3
context-data questions; amass's own definites ([CR#701.47c] — "the
amassed Army" three lines, "the Army you amassed" one, a
keyword-scoped read no general vocabulary reaches); the drawn-card
read inside a revealing coordination ("Draw a card and reveal it.",
four lines), which is the verb-phrase coordination the ledger
already parks beside Arc Trail's complements; a draw OUTCOME sort
(no line reads a draw's own magnitude, so `OutcomeSort` grew no
row); and the READ-count extraposition on a draw ("draw cards equal
to the sacrificed creature's power" against "draw two cards" — the
bare plural before the phrase, one more linearization side
condition).

Chapter twenty-two's own deferrals, each measured: the TWO-TARGET
exchange ("Exchange control of two target creatures", Switcheroo;
"Exchange control of two target lands", Vedalken Plotter — of the
thirty-five "exchange control of" lines, the ones naming both
participants with the word "target" rather than pairing the SOURCE
with one target), which the batch constructor does not reach for
the reason the two-target fight does not: the second element has
to read BOTH participants back, and two creature mentions in one
context are two, so `That creature` counts two and refuses. This
is the pair complement above wearing a different verb, and it is
the same measurement finding 120 made for the fight — which also
means the exchange has no GENERAL macro, only the source-paired
shape the bench writes out (Phyrexian Infiltrator); the
LIFE-TOTAL exchange (seven lines), which is not a pair of transfers
at all — [CR#701.12c] has each player "gain or lose the amount of
life necessary to equal the other player's previous life total",
a SET realized as whichever direction reaches it, so it wants the
life-total READ the comparatives split already ledgers plus a
set-to form ("life total becomes", twenty-nine lines elsewhere)
and would be a fabricated symmetry if forced into the batch
(finding 121); the ZONE exchange ([CR#701.12d] — "exchange cards
in one zone with cards in a different zone", and aura swap's
"exchange this permanent with an Aura card in your hand"
[CR#702.65a]), which wants the same pair machinery over zones and
arrives with the information cluster; the NUMERIC-value exchange
([CR#701.12g]) and its near neighbour that the rule expressly
excludes ("Switch this creature's power and toughness", eight
lines — [CR#701.12g] says the exchange rule "does not apply" to
it), both of which want the layer words; the TEXT-BOX exchange
([CR#701.12h,612.5], Exchange of Words — one card, and the
text-changing layer's); the STACK-object control carrier
("Exchange control of target noncreature spell and target
creature" — [CR#109.4] gives stack objects a controller, and the stack
ZONE landed in chapter thirty without moving this: `seedZone` gives the
controller relation ONE zone where the rule gives it two, so "target
spell you control" is refused by the zone coherence rather than by a
missing carrier (`badControlledSpell`, seventy-one lines plus eight, all
of them copy machinery), and what the shape wants is a zone-SET seed or
a second admissibility table beside `seedZone`); "lose control of" (ten lines, every one of
them a trigger condition — "When you lose control of this
creature, …" — so it is the event axis's word and not a clause
verb, and as of chapter twenty-eight the event axis exists and has no
ROW for it: [CR#603.10d] makes it a trigger that looks back in time,
Duplicity and Gustha's Scepter write the line form and Krovikan Vampire
the delayed one, and ten lines is the whole corpus); and the batch's OUTWARD contribution where an element moves
an object, which is written today as the last element's own
`effIntro` and would have to be a built union instead (finding
116); no corpus line writes a batch over zone-changing clauses,
so the cell is measured-empty rather than guessed.

Chapter twenty-four's own deferrals, each measured: the ORDINAL library
offset (above, with the ordered positions); "THE OTHER" as the
complement's singular spelling (about sixty-nine group-internal lines of
a hundred sixty-one, sixty-six of the rest being parenthetical reminder
text), which is the same relation as "the rest" under a remainder of one
and wants the cardinality a binding does not carry — finding 111's
counting blocker, now located precisely (finding 134); the
AMONG-restriction ("You may reveal a creature card from among them and
put it into your hand", three hundred six lines), which is a domain
restriction on a description rather than a determiner over a group —
core keeps it as `Selection::AmongNoted` over a recorded key expressly so
that a "this way" anaphor does not re-evaluate a filter — and which is
the OTHER route into a partition, the one three hundred and more "the
rest" lines take, re-measured in chapter thirty at three hundred thirty
"from among them/those cards" of four hundred fifty-seven "from among"
and unmoved; [the "THIS WAY" participles are LANDED, finding 195 — the
construction is `TheVerbed` with a second SURFACE and a plural twin
rather than a family of its own, and [CR#701.20a] and [CR#701.17c] are
still the reason those reads are legitimate where a drawn card's would
not be. What survives the entry is the verbs: only `VerbName`'s four
tags carry a stamp, so "revealed this way" (a hundred twenty-three),
"milled this way" (forty-three), "cast this way" (thirty-three),
"countered this way" (thirty-one) and "tapped this way" (nineteen) want
a participle vocabulary wider than the keyword-action tags, and the two
commonest CONSUMERS want structure of their own — the distributive
determiner ("each card exiled this way", twenty-one lines) and the
for-each amount ("for each creature destroyed this way"), which wants
`CountOf` to accept a group MENTION];
FACE-DOWN exile ("exile the top card of your library face down",
ninety-three lines), a per-object status the move vocabulary has no rider
for and which pairs with the object-status axis the stat deltas already
owe; COUNTED search finds ("search your library for up to two basic land
cards", a hundred six lines) and the type-free "search your library for a
card" (ninety), waiting on counted untargeted groups and a card-headed
predicate respectively; the AGENTFUL choice ("an opponent chooses two of
them", and the thirty-two "chooses N …, then sacrifices the rest" lines
with it), which is `Choose` with a subject slot it does not have
(`badChooseSomeOf`); HAND-zone anaphora ("Target player reveals their
hand, then you choose a card … from it", Lobotomy), a zone read with no
word here, which is why the hand exposure contributes nothing outward;
the arrangement rows core has and English does not (`SameOrder`,
`ChosenOrder` — zero lines each, measured-empty rather than guessed); and
the keyword MECHANICS whose definitions are library operations but whose
packaging is the macro layer's. Scry is the sharp case and worth its
count: [CR#701.22a] defines it as "look at the top N cards of your
library, then put any number of them on the bottom of your library in
any order and THE REST on top of your library in any order" — which is
this chapter's vocabulary word for word, the unbounded partitive
included. The expansion was compiled during this round to check that
claim rather than assert it, and then removed: what is missing is the
keyword WRAPPER and the reminder-text relation, not a single piece of
library grammar. Surveil, discover, and miracle's reveal condition stand
behind the same boundary.

Chapter twenty-five's own deferrals, each measured: REDIRECTION
([CR#614.9]), the replacement whose substitute is the SAME damage event
aimed elsewhere or scaled — "that damage is dealt to [other] instead"
(nine lines), "it deals double/twice/triple that damage … instead"
(thirty-three) — which wants a damage clause whose amount is the
intercepted event's own and whose recipient may be the intercepted
event's own, two reads no `Amount` row and no noun provides; the
prevention SOURCE restriction ("Prevent all combat damage that would be
dealt this turn by attacking creatures", thirty-seven lines matching the
common source phrasings, plus the twenty-eight "the next time a source of
your choice would deal damage" shields and the eight bare "by a source of
your choice"), which [CR#609.7] gives its own machinery and [CR#615.9]
its own recheck rule, and which is a rider on the damage NOUN rather than
a slot on the clause — its complement ANCHOR came unblocked in chapter
twenty-six, Terrifying Presence's "creatures other than target creature"
being `terrifyingPresenceAnchor`, so what the line waits on is the
shield's source slot alone; prevention's ADDITIONAL effect ([CR#615.5] — "the
prevention takes place at the time the original event would have
happened; the rest of the effect takes place immediately afterward";
twenty lines read "damage prevented this way"), which is the "this way"
participle family the ledger already calls the largest, wearing a
prevention verb; the DIVIDED shield ("Prevent the next 3 damage that
would be dealt this turn to any number of targets, divided as you
choose", five lines), which is chapter twenty-three's division over a
shield rather than a verb and wants `DividedVerb` to grow a third row
it has no other carrier for; "damage can't be prevented" ([CR#615.12] —
twenty-seven "can't be prevented" lines, twenty-four of them the damage
phrase), NOTED and not built: it is a deontic over an EVENT CLASS rather
than over a deed, so it belongs with the rest of the deontic surface and
not with the shield it names; the SKIP family ([CR#614.1b,614.10] — "Skip
[something]" is expressly "Instead of doing [something], do nothing", so
the rules make it a replacement and English gives it a verb of its own;
twenty-three lines), which wants a turn-structure noun this vocabulary
does not have; the put-into-a-graveyard interception (a hundred lines
across possessors, fifty-seven of them the bare "a graveyard" and
thirty-two of those "from anywhere"), whose subject is a description
spanning every zone at once and whose home is a static ability besides —
the container question finding 147 files, now owed from three sites
(the standing interception, the standing shield, and the entry rider);
the complement ELLIPSIS (Galvanic Blast's "deals 4 damage instead", the
replacement clause that inherits the replaced clause's patient rather
than restating it), a new entry beside Arc Trail's coordination and
Pyromancy's extraposition and the same kind of thing — a linearization
the sentence grammar has no slot for; and [the EVENT-VOCABULARY MERGE
LANDED, findings 171 and 175 — `EventQuery` is gone, the delayed clause
reads `GameEvent` like the other three constructions, and the "this
turn" is the FRAME's: [CR#603.7b] names the phrase as a delayed
ability's stated duration in as many words. What the merge exposed is
the thing a single filter type could not have: the readers disagree
about TENSE as well as about the event, so `eventIntro` (what the
phrases announced, [CR#614.6]) and `eventAfter` (what the event left
behind, [CR#603.6]) are two projections and the trigger reads the
second].

Chapter twenty-seven's own deferrals, each measured: the MANA ability,
which [CR#605.1a] makes an activated ability by four criteria none of
which is about its shape — so the container already holds it and what is
missing is a production clause, three hundred thirty-two lines writing
the bare "{T}: Add …" and core keeping produced mana a separate type
from printed symbols (`ManaSpec` beside `ManaSymbol`); the LOYALTY cost
(eight hundred thirty-two bracketed components, "[+1]"/"[−7]"), a symbol
vocabulary of its own, which core's third use-limit row carries with it
(`LoyaltyOncePerTurn`, [CR#606.3]'s cap shared across a planeswalker's
abilities rather than per-ability); the turn-part WINDOW ("Activate only
during your upkeep", thirty-two lines; "during your turn", forty-one;
"during your turn, before attackers are declared", nineteen; about a
hundred twenty in all, core's `Timing::DuringTurn`/`DuringStep`), whose
vocabulary is nearly chapter seventeen's already and whose exact gap is
two lines wide — "Activate only during any upkeep step" writes a
possessor `Whose` has no word for, neither yours nor a named player's;
the cost-language READS (seventy-four lines — "unless [someone] pays its
echo cost" and its upkeep-cost twin want a keyword's own cost, and "pays
its mana cost" is core's `ManaCostOf` over a reference, the cost-language
twin of the numeric mana-value read); the declared OPTIONAL cost with its
tag and three read channels ([CR#118.8b]; core's `OptionalCost` +
`CostTag`, read by `Condition::PaidCost`, `Count::TimesPaid` and
`Predicate::WasPaidWith`), which is what kicker and the conditional
headcount upgrade actually want and is keyword packaging rather than
sentence grammar; the total-cost PIPELINE ([CR#601.2f] increases and
reductions, core's `CostChange`) and the alternative-cost base swap
([CR#118.9], core's `AlternativeCost`), both engine traffic; the
per-pip alternative payment (convoke/delve/improvise — core's
`PayPips(PipClass, PayAct)`), which is a static ability's; the aggregate
TapTotal cost ([CR#702.122a] crew — core and the settled spec both carry
it as one row and no corpus line here needs it yet); the QUOTED ability
grant ("Enchanted land has \"{T}: Add {B}\"", twenty-five lines; the
equipped and all-Slivers twins, ten and seven; the token with-clause,
seven), a quotation construction this grammar has no row for and the one
thing the container's widening had to refuse (`badGainsActivated`,
`badTokenActivatedAbility`); the UNTAP clause, five cost components
("Untap a permanent you control") with no `Effect` row to stand on — the
untap SYMBOL landed, the verb did not; and the PHYREXIAN symbol row,
which stands on the port ruling alone: thirty-nine Phyrexian components
appear in activation costs and not one of their abilities is writable
here (twenty-odd are "Transform this creature"; the rest want an
indestructible counter, a counted untargeted group, or a cast-from-exile
permission), with Solphim, Mayhem Dominus named as the real spelling.

Chapter twenty-eight's own deferrals, each measured. The REFLEXIVE
trigger is TAKEN, finding 183: the mini-round found that "you do" is a
pro-verb rather than an event word, so the construction reads the
enclosing clause and no `GameEvent` row was needed — what remains from
that entry is two cards whose blockers moved somewhere smaller (Yes Man,
Personal Securitron on a quest `CounterKind`; Heart-Piercer Manticore on
[CR#608.2h]'s last-known information). The "this way" participle the rule
names in the same breath, a thousand and fifteen lines, is TAKEN too,
finding 195: it is one construction with two surfaces rather than a second
construction, and what was actually missing was the PLURAL participle
read — what remains from that entry is the family's two commonest
consumers, the distributive determiner ("each card exiled this way",
twenty-one lines and the single commonest frame) and the for-each amount
("for each creature destroyed this way"), which wants `CountOf` to accept
a group MENTION where it takes a description, plus the verbs beyond
`VerbName`'s four tags (revealed a hundred twenty-three, milled
forty-three, cast thirty-three, countered thirty-one, tapped nineteen).
The CAST event is TAKEN, finding 193, and the stack row and the spell
word arrived with it. The CAST-verb permission is TAKEN, finding 194,
and its recorded blocker was a reading rather than a row — [CR#701.5b]
makes "to cast a card" ordinary and [CR#604.6] brackets the verb as a
slot — with its count corrected there: of the two hundred ninety-three
"from your graveyard" lines two hundred eight are keyword REMINDER text
and eighty-five are ability lines, and the hundred eighteen "from exile"
divide a hundred thirteen to five. The enters-with COUNTERS rider is
TAKEN, finding 192, leaving the counter-KIND catalog beyond
`CounterKind`'s rows. TIME came off that tail in chapter thirty-one,
finding 200 — a hundred fifty-nine lines and one-shot puts and removes
at three sites — leaving oil, fade, charge, indestructible, finality,
shield, divinity and their neighbours at eight lines apiece and fewer,
plus the twenty other kinds the exile rider writes (scream, hit, delay,
study, memory, void, silver, brain, hatching, egg, ice, blood,
collection, stash, croak, takeover, discovery, dream, kick), each one or
two lines and none of them a second construction. Chapter thirty-three
measured that tail again and left it here, finding 229: the products
reached the stat pairs and the keyword names, and a flat name still
waits on a construction. The "unless" MARKING is TAKEN, finding
196, on the deed
restriction alone: the hundred eleven "can't … unless" lines are
writable and the thirty-nine "enters tapped unless" ones remain
unwritable. The five basic land subtype rows have landed; remaining
blockers include the generic chosen/basic-land-type surface, the
"legendary" supertype as a PREDICATE rather than a type-line field, a
counted comparison over "other lands", and a game-state condition. And
the END-OF-COMBAT header is TAKEN as a
`TurnPart` row, finding 196, with the eleven headers themselves still
unwritable — each wants a blocking-RELATION predicate or a combat
lookback — so the delayed clause carries the witness. ANNOTATED, chapter
thirty-eight: the relation landed (`BlockerOf`/`BlockedBy`) and seven of
the eleven are writable, Kjeldoran Frostbeast being the bench's first
end-of-combat header; the remaining four want the past-tense lookback
(finding 271) and stay. Still open: the
DELAYED cast (thirty-six lines, "When you next cast a … spell this
turn", every body either granting an ability to an object on the stack,
which `Gains` refuses by zone, or copying the spell); the
put-into-a-graveyard TRIGGER (a hundred
nine, ninety of them "from the battlefield"), a different row from the
departure because [CR#603.6c] says an ability triggering on a zone "from
anywhere" is never a leaves-the-battlefield ability; the BECOMES family
([CR#603.2e] — "becomes tapped" a hundred, "becomes untapped"
thirty-three, "becomes blocked" a hundred fifty-nine, "becomes the target
of" a hundred twenty-eight), one row per state transition and each with
the rule's own note that they "trigger only at the time the named event
happens"; the life-gain trigger (sixty, and [CR#119.9] rewrites it into a
source-caused event before it triggers); the trigger's own INSTRUCTIONS, a third restriction
surface beside the activated line's three ([CR#603.1a]'s target limits,
[CR#603.2h]'s "Do this only once each turn" at thirty-two lines,
[CR#603.2d]'s "triggers an additional time" at twenty-nine); the STATE
trigger ([CR#603.8]), zero corpus lines here and a trigger over a game
state rather than an event; the permission's
generic PLURAL subject ("You may play lands from your graveyard",
Crucible of Worlds), which is the deontic chapter's own bare-plural entry
at a new site; and the QUOTED grant grown by two — English grants a
triggered ability and a static ability by quoting them exactly as it
grants an activated one (`badGainsTriggered`, `badGainsStatic`).

Chapter thirty's own deferrals, each measured. LOYALTY, whole: a loyalty
ability is an activated ability whose cost is a symbol change and `Cost`
would carry that cheaply, but no planeswalker is writable end to end —
`CardType` has no `Planeswalker` row, the bracketed "[+1]"/"[−7]" cost
symbols are eight hundred thirty-two components and a vocabulary of their
own, [CR#606.3]'s once-per-turn cap shared across a planeswalker's
abilities is core's third use-limit row waiting with them, and the
planeswalker subtypes are a set apart. The card's remaining PARTS: the
colour indicator ([CR#204.1] — a printed dot and not language), the
defense number, and [CR#208.2]'s star power and toughness, which is a
characteristic-defining ability ([CR#604.3]) this container has no
ability row for. Of [CR#205.4a]'s five SUPERTYPE rows, `Basic` and
`Snow` are TAKEN in chapter thirty-three; only `World` and `Ongoing`
remain. The "legendary" supertype as a PREDICATE rather than a
type-line field, which is what the "enters tapped unless you control a
legendary creature" lines want. Two-FACED cards ([CR#712.8] gives each
face its own characteristics; core wraps a face in a layout enum where
this record IS a face). The SpellCard-by-Static cell's one real
exception: [CR#604.6]'s play permission is printed on spell cards
(flashback's own template) and the container's table refuses it there,
a per-row carve-out wanting an axis the table does not have. The COUNTER
verb's ability half ([CR#701.6a] counters "a spell or ability"), an
ability on the stack being an object [CR#109.1] this noun vocabulary has
no word for. And Escape to the Wilds' second line, "You may play an
additional land this turn", the play COUNT — a permission over how many
times an action may be taken rather than over what may be played.

L1 **Source noun family (PARTIAL).** Head/readback vocabulary waits on
its landing site: the [CR#609.7] prevention by-source rider (the
`Prevents` row's existing ledger note) and the damage-redirection
interception. Counts held: 63 choice, 50 controlled, 37 would-deal, 18
"that source", 0 "target source". Last-known-information source
identity ([CR#120.7]) is engine work beyond the noun surface.

L2 **Permanent-card eligibility gap, by name:** planeswalker and battle
are [CR#110.4a] members outside the six-row `CardType`; a permanent-card
line naming either is unwritable until the catalog decision. (The bare
head itself is witnessed — Vindicate, "Destroy target permanent.")

L3 **Described-token readback.** A described mention ("target token …
that token") does not seed `Origin`; only creation does. The
description-to-origin projection (a `seedTy`-shaped scan) waits for a
measured spanning line. Participial token reads ("the sacrificed
token") are unmeasured and `verbedWordOk TokenW` is closed at False.
**No positive witness lands for the token head:** the corpus writes no
bare token-head line. Its three nearest attestations are each blocked by
machinery this grammar has not minted — a modal-spell mode
("• Destroy target token."), a cost ("Sacrifice a token:"), and an ETB
trigger with a controller rider ("destroy target token an opponent
controls"). The head is pin-backed until one of those three
constructions lands, and it is the modal mode that is nearest.

L4 **"Nonpermanent."** Unmeasured this round; `negatable Permanent`
closed at False pending a count.

L5 **Status frames not yet classified:** the lone "flipped" line, the
twenty-two participial "phased out" lines, and off-battlefield face
orientation (face-down exile, library piles, draft — expressly not
status under [CR#110.5d]); morph/[CR#708] characteristics and
turn-face-up permission; phasing's [CR#702.26] semantic tail.
PARTIALLY CLOSED in chapter thirty-seven: the face and phasing VERBS and
their events are represented (`ToFace`, `IsTurnedFace`, `Phases`,
`PhaseTransition`); the FLIP verb is refused a shape by finding 263 and
stays, as do the participial description cells, the off-battlefield face
orientation, the two exiled-card face-up lines, and [CR#702.26g]'s
indirect phasing.

L6 **Untap-lock variants:** the one-shot "during its controller's next
untap step" family, the single "doesn't untap" line without "during",
and the seven "can't untap" lines — the 121/1/7 split as counted then;
REMEASURED in chapter thirty-five at 54 standing / 67 timed / 1
replacement reminder / 7 caps (finding 250). The 67 timed lines are
REPRESENTED in chapter thirty-six (`DoesntUntapNext`); the one
replacement reminder (Bewitching Leechcraft) is ledgered with quoted
granted replacements and attachment, and the seven caps are their own
future static-restriction round. **CLOSED, chapter thirty-nine:** the
caps are represented (`CantUntapMoreThan`, over `CapDomain` and
`CapBound`; findings 273–279), so every cell of the census is now either
represented or classified and only Bewitching Leechcraft remains, with
its own two blockers.

L7 **Becomes-status trigger event:** designed (EventName row,
`TriggeredOnly`, per-value attestation), blocked on a clean witness
line — as recorded then; LANDED in chapter thirty-five (findings
245–248: `BecomesStatus`, `StatusChange`, `statusEventOk`; Gideon's
Avenger and Mesmeric Orb the witnesses); "becomes tapped" 100,
"becomes untapped" 33, all other values 0; "remains tapped" (40)
classified by finding 249 as a "for as long as" duration condition. The
FACE and PHASING transition events land in chapter thirty-seven
(`IsTurnedFace`/`TurnedFaceUp` with `faceEventOk`, 113 up against 0 down;
`PhaseTransition`/`PhasingChange`, both directions open).

L8 **"Enters untapped"** (6 lines): a measured [CR#110.5b]-default
override with no quoted witness; `TokenRider` unchanged.

L9 **Gnarly inventory discrepancy:** the recoverable object-class alias
union is 44 cards against the stated 43, with no recorded exclusion;
both numbers held until adjudicated.

The context-as-phrase-telescope collapse (bindings storing
the mention terms themselves, every projection computed) stays open as
a possible later simplification — less pressing since the payload
split gave each kind exactly its own data.

## The Representability Frontier

The representability frontier (the second-wave audit's merged map;
every gap evidenced by real oracle text plus the core taxonomy,
none spellable today). New structural axes: attachment — ONE
relation ([CR#701.3a]; core's `AttachedTo`), with
"enchanted"/"equipped" as derived reads of an Aura/Equipment edge
([CR#303.4b,301.5a]); the two keywords are NOT parallel — Enchant
statically restricts an Aura's legal target and host ([CR#702.5a]
— a deontic grant over the attach basis, not edge structure),
while Equip is the activated ability that does the attaching
([CR#702.6a]);
object status — newly representable: status words in ordinary
descriptions and conditions; the permanent head across its three
carriers; the token head, "nontoken", and the created-token readback;
the untap effect; the standing untap-step lock; becomes-status triggers
(chapter thirty-five); the timed one-shot untap lock (chapter
thirty-six); and the FACE and PHASING transitions, verb and event alike
(chapter thirty-seven). Still out: source phrases; the flip verb, whose
shape question finding 263 states; pile partitions
(Death or Glory partitions the GRAVEYARD, so this is not the
library gap — [CR#700.3a] puts each object in exactly one pile
"unless the effect specifies otherwise" and [CR#700.3d] lets a
pile be empty, while [CR#700.3c] only keeps the piles in the zone
they came from; labels, choice, and complement are the card's own
instructions); modal clauses ("Choose one —",
[CR#700.2,115.8] — effect-row alternatives with branch-local
targets); truth-valued conditions and branching [LANDED in part,
chapter eighteen — `Condition` and the `If` clause; the BRANCH is
ANSWERED, finding 148 — it is a replacement and not an else-arm,
[CR#614.15]'s self-replacement, and `InsteadOf` is its node with the
ordinary trailing condition. Galvanic Blast itself is still not
writable and the blocker moved: its replacement clause writes no
recipient ("deals 4 damage instead"), a complement ELLIPSIS inheriting
the replaced clause's patient, which is a new ledger entry beside the
coordination and extraposition gaps. What is still frontier here is
[CR#603.4]'s separation of intervening-if from English "if", the seam
finding 76 leaves for the trigger layer]; linked-ability memory [LANDED, chapter thirty-one, findings 203-205 —
`ExiledWith`, a source-keyed exile group named by a PREDICATE and not by
a discourse read, its head and zone `InZone`'s ([CR#406.2,406.6,607.2a])
and its source slot the self-word ([CR#607.1]). Cold Storage, the card
this entry named, is a whole card on the bench with both abilities. What
survives the entry is smaller and named in finding 210: the CARD-LEVEL
creation check ([CR#406.6]'s "printed on it" is a fact about an ability
LIST, and [CR#607.5a] makes an unlinked reference do nothing rather than
be illegal, so a gate would over-refuse — its price is two deep
traversals over `Effect`/`Noun`/`Predicate`/`Amount`/`Cost`); the
NAMED-source linkage ([CR#607.2n], three lines, wanting the printed name
read back); and the definite determiner over a DESCRIPTION, which is what
"the card exiled with this artifact" would need and what makes finding
206's second surface unwritable]; divided amounts [LANDED, chapter
twenty-three — `DividedDamage` and its counter twin `Distribute`, the
allocation announced as the spell is cast ([CR#601.2d]) and held against
a later change of targets ([CR#115.7f]); Arc Lightning, Forked Bolt and
Boulderfall are the three ranges. The card this entry NAMED is writable
too and is now on the bench whole — finding 224, Chandra's Pyrohelix
writing Forked Bolt's line word for word];
dependent iteration (Killing Wave's "for each creature, its
controller…" — a per-member singular frame no plural noun
provides); aggregation and extremal selection (Crackling Doom's
"greatest power among"); data-dependent repetition (Torment of
Hailfire's "repeat this process X times"); turn-schedule
insertion (Relentless Assault, [CR#500.8]); standing triggered
abilities ([CR#603.1] — the ability SHAPE beyond delayed
queries); counters as per-HOLDER state [LANDED in part, chapters nineteen
and thirty-three — the object-borne put and remove verbs over a witnessed
`CounterKind`, whose stat and keyword halves became PRODUCTS in finding
226 and 228 — and their ZONE landed in chapter thirty-one, findings
198-199: the gate is `CounterHolder`'s closed full-row table over the
seven zones, open on the battlefield and on EXILE ([CR#122.1a,122.1b]
name counters outside the battlefield and oracle writes them there,
Jhoira of the Ghitu and Alaundo the Seer) and shut on the other four,
the stack's silence being [CR#122.6]'s own default. What is still
frontier here is the PLAYER holder, which English reaches with a
different verb ("target player gets a poison counter", forty-eight
lines), the counter COUNT read ("the number of +1/+1 counters on it"),
and the kind-blind quantifiers below]; mana production and payment
([CR#106.4]); DESIGNATIONS, which ride all three holders the way
the counters above ride two — a player's ("the monarch",
[CR#725.1]; "the initiative", [CR#726.1]), a permanent's (the
Ring-bearer, [CR#701.54b]), and the GAME's own (day and night,
[CR#731.1]) — each a definite noun phrase naming a role rather
than an object, which is why the definite sweep below surfaced
them and had nowhere to put them; the GAME as an object of a verb,
which chapter thirty-one measured and returned (finding 208): RESTARTING
is [CR#727]'s whole chapter and [CR#727.1] opens it with "One card (Karn
Liberated) restarts the game", one corpus line, and the sentence around
it wants [CR#727.5]'s "leaving in exile" exemption besides — a family of
one, ledgered rather than minted, with the linkage designed to survive
one ([CR#727.2] keeps every card involved); object OWNERSHIP as a relation ("a card you own in exile", Alaundo the
Seer and Mari the Killing Quill; the bench elides it), which is a
possessor axis distinct from `ControlledBy` and one the exile zone makes
necessary — [CR#109.4] leaves an exiled card no controller to be named
by, so [CR#108.3]'s owner is the only possessor the phrase has left; the
trigger on a counter's LAST removal ("When the last time counter is
removed from this card", three lines; "time counter is removed", nine;
"the last [kind] counter is removed", five), a `GameEvent` row over the
counter verbs and the third component [CR#702.62a] names, unminted with
findings 198 and 201 built beside it; and control ASSIGNMENT on a
battlefield move [LANDED, chapter thirty — `MoveRiders`' `ctrl` slot,
held to one controller by `CtrlSingular`; [CR#110.2a] gives the
instructed player by default, which is why the slot is a `Maybe` and the
phrase restates the default at a hundred twenty-seven of the hundred
thirty-five control lines. Hymn of Rebirth, Sisters of Stone Death,
Synod Sanctum and Cold Storage all write it. What the slot does not
carry is ORDER, and finding 223 prices that]; and the GAME OUTCOME as an
effect, which this ledger had never named at all — "you win the game"
thirty-seven lines, "you lose the game" twenty-three, and the whole
family a hundred fifteen lines over a hundred seventeen supported cards.
[CR#104.2b] and [CR#104.3e] are what make it something a sentence can
say, an effect that "may state that a player wins/loses the game"
standing beside the state-based ways ([CR#104.3b,104.3c,104.3d]), and it
has four shapes and a boilerplate: the bare instruction (Door to
Nothingness), the conditioned one (Near-Death Experience's "if you have
exactly 1 life"), the recurring upkeep check (Immortal Coil), and the
PROHIBITION — "You can't lose the game", nine lines, which is the
deontic over an outcome rather than an outcome — with poison's reminder
text as the boilerplate. It wants an `Effect` row and a `GameEvent` row
at once, and the prohibition wants the deontic coverage the sweep below
names. Leaf
vocabulary: Transform ([CR#701.27a]);
life-total Set [LANDED — `LifeOp`'s `Set` row ([CR#119.5]), the thing
finding 121 said the life-total exchange would need; "target player's
life total becomes 1" is the witness]; the non-additive continuous family —
"becomes", lose abilities, set base P/T, the layer words — of
which GAIN CONTROL landed in chapter twenty-two ([CR#613.1b] layer
2; `GainsControl`, finding 117) and the rest still wait.

The frontier above is this file's own walk, gap by gap as the chapters
met them. Read against it, the vintage-gauntlet primitive inventory
(2026-08-08; two hundred forty-nine cards and a hundred thirteen
canonical primitives after the user rulings, with the draft protocol
filed beside the engine-boundary deferrals) sorts the same ground four
ways, and the SHAPE of that sort is the part worth keeping here.
VOCABULARY ROWS, each an enum row or a table cell and nothing more: the
object-class nouns — permanent, spell, card, source — which is the
biggest single catalog win at forty-three cards; the entry and exile
counter NAME tails [NOT taken, finding 229 — measured, and the flat
names left to the bar that governs them]; the turn-part catalog, which
is what the activation WINDOW above waits on; the participle verb tags
beyond the four; Basic/Snow and the five basic land types [LANDED,
finding 232]; keyword possession [LANDED, finding 230]; and the
characteristic predicates.
NEW AXES, by demand: object STATUS
(tapped, flipped, face-down, phased — forty-four cards, the largest
single axis), ability-as-a-value (thirty-three, and its one blocker is
`grantableAb`'s four False rows), the fronted for-each binder
(twenty-eight, remeasured above), the repeat-process loop (twenty-eight,
five termination shapes), the counterfactual "as though" (twenty-six),
ability borrowing (twenty-five), the deontic beyond `Cant` (twenty), and
then extremal selection, the layer words, event-history reads, turn
order, and the game outcome added above. PROTOCOL BUNDLES, which have to
be designed whole and are misleading costed row by row: the draft, the
secret-choice and voting pair, pile partitions, bidding, and
end-the-turn. SPELLING RESIDUES, where the semantics exist and only a
surface is missing: clause ellipsis, effect disjunction, and the bare
generic plural finding 223 measures.

Two of those entries carry a DECISION rather than a measurement.
KEYWORDS are a capability bar and not an authoring list: what the
completion proof wants is the vocabulary rows, the possession predicate,
the half-dozen truly intrinsic keywords whose effects are non-composite,
and a composite carrier for all the rest — which is the
ability-as-a-value axis under another name — while per-keyword authoring
stays out of scope, and the intrinsic set is read off core's own setup
rather than off any paraphrase. [LANDED in vocabulary, findings 230/228:
the rows, `HasKeyword`, and the `KeywordCounter` carrier, with the seven
rows set by which keywords core implements itself. The composite carrier
is what remains, and it is still the ability-as-a-value axis.] The GAME RESTART is MINT QUEUED:
chapter thirty-one measured it and declined on the honest count
([CR#727.1]'s one corpus line, and a family of one is not a family), and
that decline is overridden — it is a small late round, and finding 208's
own lifetime argument is what makes the linkage survive it.

## The English-AST Frontier

The English-AST frontier — §8's other half, the parser's own
construction inventory read against this file rather than the gaps
the chapters walked into. Deferred deliberately, each waiting on
an axis already named above: the general prepositional, adverbial,
and subordinating OPERATORS — particular relations are ledgered
one at a time (the "if" and "as long as" conditions, the "when"
events), but the operator carrying any of them is not, and the
counterfactual "as though" has no ledger entry at all ("This
creature can attack this turn as though it didn't have
defender"); the bounded and
alternative quantity WORDINGS — `Quantity` is already a range with
both bounds optional, so "at least three", "three or more", and
"more than two" are ONE bound pair under three words (a count
against a literal, not chapter sixteen's bound on a
characteristic — and English keeps the two apart in the word
itself, count nouns taking "more"/"fewer" where a characteristic
takes "greater"/"less"), which puts the gap on the spelling rather than the
structure: the semantics is right to collapse them and the
renderer has to undo the collapse; the two that are not bounds are
"one or two" (a disjunction of exact counts, waiting with the
divided damage above) and "both" ("both creatures have
deathtouch"), which is not a quantity word at all but the group
COMPLEMENT below, a definite plural presupposing a two-membered
antecedent no binding records; activation restrictions and
frequency riders [LANDED, chapter twenty-eight, finding 164 — the
`Activated` line's three slots, the window ([CR#602.5d]'s sorcery
timing), the use limit and the state guard; Security Detail carries two
of the three on the bench and Basking Rootwalla, Bonder's Enclave and
Savage-born Hydra the others. This summary line is the half that never
caught up with its own chapter. What genuinely remains is the WINDOW's
vocabulary, which is `Timing`'s two rows against the turn-part catalog,
and the ORDER of the three, which finding 223 prices]; nonfinite and
elliptical clauses — infinitive, gerund, and ellipsis have nothing
to lower to while every effect here is a saturated clause; the
passive and its reduced-recipient forms ("Whenever this creature
is dealt damage, it deals that much damage to the chosen player",
Stuffy Doll) — recipient fronted and agent dropped, arriving with
the event queries above rather than as a clause shape of its own;
exception riders ("except it's legendary", "except it's not
legendary") — one characteristic of what the clause just made,
overridden after the fact, so they want the copy/token vocabulary
and the layer words together; the group COMPLEMENT, which SPLIT in chapter twenty-two and
whose two halves are two different subtractions (finding 111).
What LANDED is the ANCHORED complement — "each other creature"
(a hundred and thirty-five lines), "other creatures you control"
(a hundred and thirty-seven), "each other player" (fifty-three) —
which takes a REFERENT out of a DESCRIPTION and so needs no
cardinality at all; it is a predicate row (`OtherThan`) carrying
its anchor as a read, and the sweep's own diagnosis is what hid
it, a description having nothing to count. What STAYS is the
subtract-a-SUBSET half, and it keeps the sweep's blocker exactly:
"the rest" ("Put one of them into your hand and the rest on the
bottom of your library in any order", six hundred and fifty-two
lines), "the other" ("Exile one of them and put the other into
your hand", a hundred and ten bare uses), and "both" above
(nineteen) each select from an established group and name the
leftover it defines, where a binding records a mention's
`Plurality` and its `Determiner` and neither carries how MANY — so
nothing here can presuppose a two-membered antecedent or subtract
one mention from another. It is the twin of the pile partitions
below, the reciprocal fight frame is still waiting on it, and the
BINARY fight's own expansion joined the queue in chapter
twenty-two now that the batch constructor exists and does not
help ("(Each deals damage equal to its power to the other.)",
findings 20 and 120). "The rest" is doubly shut besides: every one
of its lines is a library partition after a look or a reveal, so
its carriers are the information cluster's and the ordered
library's as much as this primitive's; set-exception
noun phrases
("choose a card type other than creature", Arachne, Psionic
Weaver) — a complement
over a QUALITY domain, which `Other`'s object distinctness is not;
the binding mechanism now covers four selected sorts, but the CardType
domain and the complement remain; arithmetic and rounding
("Target opponent loses half their life, rounded up") — `Plus` is
the only composition, and subtraction, halving, and a rounding
mode have no constructor; quoted abilities (Master of the Hunt's
token has "bands with other creatures named Wolves of the
Hunt.") — an ability as a VALUE and a verb that installs it, the
layer words' carrier question in a new place; the causative ("You
may have it become a 3/3 Elemental creature with haste and menace
until end of turn") — `have` over a bare-infinitive complement,
and only its narrowest "may have [player] deal" reading is `May`
today; coin flips and their result clauses ("You
and target opponent each flip a coin", Mana Clash) — a random
outcome to bind and a side predicate to read it back; the
rules-defined bundles, which SPLIT — historic ([CR#700.6]),
modified, party, and outlaw name sets the CR fixes, landing with
the predicate vocabulary, while devotion is a measured value
taking a mandatory "to [color]" ([CR#700.5]; "each opponent
loses X life, where X is your devotion to black", Gray Merchant
of Asphodel), landing with the amounts;
the COORDINATION families chapter fourteen did not take, each
waiting on a constituent this file has no term for: CLAUSE
coordination in its six spellings (`ClauseCoordination`, its comma
and asyndetic variants, and the three copular-noun-prepositional
ones) waits on no new structure — `Sequentially` composes clauses
already — but on the DISTINCTION the guide draws between
coordinated and sequenced results ("and" for results with no
emphasized ordering, "then" when the order matters), which the
clause list currently collapses, the Arc Trail entry above being
the same collapse seen from one card; QUOTED-ability coordination
(`VerbPhraseQuotedAbilityCoordination`,
`VerbPhraseAbilityQuotedCoordination`) waits on the quoted ability
as a VALUE, above; the coordinated-MODIFIER trio
(`CoordinatedModifierConjoined`, `CoordinatedModifierOxford`,
`NominalCoordinatedModifier`), the coordinated-ADJECTIVE trio
(`VerbPhraseCoordinatedAdjective`,
`CopularRemainderCoordinatedAdjective`,
`RelativeContractedCopularCoordinatedAdjective`), and the
postpositive-adjective spellings
(`nominal_postpositive_adjective_conjoined` with its Oxford and
prepositional twins) all wait on the same missing constituent, an
ADJECTIVE phrase: the status words here are predicates, and
nothing in this file is adjective-shaped; and
`PrepositionalPhraseSiblingCoordinated` waits on the general
prepositional operator named at the top of this list. Two
disjunctions are likewise named rather than built — the
CROSS-ZONE one, which the single-valued zone projection refuses
outright rather than mis-place ("an Equipment card from your hand
or graveyard", seventeen corpus lines under a shared preposition;
`badCrossZoneDisjunction`), and the KIND-crossing one, which is
the kind-union entry above wearing a coordinator. And `and/or` is
a THIRD coordinator, not a spelling of this one: the parser keeps
`Conjunction::AndOr` beside `And` and `Or` in its coordination
constructions, and the guide gives it its own rule — either
category alone or both together qualify, "instant and/or sorcery
cards"; and the striated ability
frames — class levels (Monk Class's "{W}{U}: Level 2"), saga
chapters (History of Benalia's "I, II —"), level bands (Kargan
Dragonlord's "LEVEL 4-7"), station thresholds (The Eternity
Elevator's "20+ |"), and die-roll rows — one shape under five
names: a keyed band of the CARD granting an ability set, sometimes
with a P/T box, once a counter, level, or roll reaches its key, so
the counters are the frontier's per-holder state while the frame
itself sits above every clause.

One gap is the AUTHORING side rather than the semantic one: a
fragment is authorable at Nominal, Sentence, Cost, KeywordLine, or
Ability and at no other category, so an `Amount`, a `Duration`, or
an `EventQuery` has no category to be authored AT — which is why
the spelling pass marked those constructors construction-owned or
TODO instead of giving them a fragment of their own.

One family was held back as the next CHAPTER rather than a
deferral — the generic definite and possessive noun phrases — and
the sweep that went to take it found no chapter there. The
parser's eight generic constructions (the closed determiner word,
the quantity determiner, the possessive noun bare, determined,
adjective-premodified, and occupying the determiner slot, the
bare demonstrative, and the determiner-nominal carrier itself)
are all CARRIERS — a slot for a word — where this file names the
RELATION the word carries, so the sweep MAPPED rather than built.
The closed determiners go across one for one: "a"/"an" is
`Indefinite` (its choice mode the adverbial, not the article),
"each" is `Each`, "all" is `AllOf`, "another" is the `Other`
conjunct on a determined head, "this"/"that"/"those" are
`This`/`AsType`/`That`/`Those`, and "any" is `AnyTarget` in the
damage class beside the unbounded `Quantity` of "any number of"
(its third use, "mana of any color", rides the mana axis above).
Two do not go across, and neither is a definite: "no" is a
condition word — its corpus mass is "if you control no untapped
lands", "if no mana was spent", and "with no mana cost", which
are the conditions, the layer words, and the exception riders
above — and the bare quantity determiner is the counted
untargeted group already ledgered.

"The" itself is what the sweep was for, and it is not a free
anaphoric determiner. The guide gives the antecedent job to
"that"/"those" and licenses "the" for the chosen quality, the
participle read, and "the rest" — selection now covers four quality
sorts, but `OfChosen` remains Color/CreatureType only and CardName/Number
reads are equality-specific and still open; `TheVerbed` is typed here
(uniqueness-gated exactly as the guide states it), and the third is the
group complement above,
which is why `TheD` is already a `Determiner` row. Across the
eleven thousand corpus lines carrying "the", the mass is zone
words ("the battlefield", a bare-scoped `ZoneAt`), the trigger
frame ("the
beginning of"), and axes named above — library order ("the top",
"the bottom"), aggregation ("the number of"), extremal selection
("the greatest"), copying ("the copy"), the stack read ("the
target"), and the designations. The residue is a SURFACE
alternate: a re-mention written "the player" where the guide
would write "that player" ("Then if that player has no cards in
hand" beside "If the player doesn't, creatures they control can't
attack you this turn"). A `The` constructor would be `That` with
a different word and no new meaning, so that gap is the
renderer's rather than this file's.

The possessives divide by RELATION, which is how this file spells
them: "its owner's hand" and "an opponent's graveyard" are the
owned zone scope (`handOf`/`graveyardOf`), "its controller"/"its
owner" are
`ControllerOf`/`OwnerOf`, "that creature's power" is `powerOf`
(`StatOf Power`, the stat read under its characteristic),
and the adjective-premodified possessor is benched already ("the
sacrificed artifact's mana value"). The possessor is gated on
being SINGULAR and never on being definite, which the corpus
supports — sixty-five lines write "an opponent's graveyard". The
possessed nouns with no constructor are nouns this file has no
term for at all, the library and the turn parts and control
assignment and the life total, each waiting on its own axis
rather than on possession. Two possessor SHAPES wait: the plural
relational ("their owners' hands", a hundred and four lines,
`badGroupOwner`) and the distributive singular ("each player's
graveyard", thirteen), the second being the dependent iteration
above rather than a possessive gap. The bare demonstrative noun
phrase, last of the eight, has no rules-text witness at all:
every corpus line ending a clause on one is reminder text
("Target a creature as you cast this", "Put a level counter on
this"), where "this" is the source `This` already names.

Comparatives were a second such family and have SPLIT, as
coordination and the auxiliaries did before them: chapter sixteen
took the BOUNDED qualifier — a written
numeral or X, under "or less"/"or greater" — and four families are
left, each waiting on an axis rather than on more comparison
machinery. The PHRASAL standard is the largest: "with power less
than Yasova Dragonclaw's power", "with mana value less than or
equal to the number of lands you control" — a hundred-odd lines,
the other word order, and the whole of core's strict `Cmp` rows,
waiting on a comparison whose right-hand side is a noun phrase
read off a second object (finding 60's other frame). The PLAYER
attribute is next, and is not a variant of that one: "13 or
less life" is the same frame around a player's own number rather
than an object's — core spells it in a separate constructor for
that reason (`Predicate::PlayerStatCmp(PlayerAttr, Cmp, Count)`,
`filter.rs`) — and even the comparative word changes, life taking
"more"/"less" where a characteristic takes "greater"/"less" and
"N or greater life" appearing zero times. It waits on the player
attributes, not on this chapter's row. The COORDINATED
characteristic is smaller and sharper: "with power or toughness 4
or greater" coordinates the READS under one bound in eight lines,
which `Or` cannot spell because it coordinates PREDICATES and the
bound here is one word for the pair. The AGGREGATE is the last —
"with total power N or greater" (crew and saddle, thirty-four
lines), "with total mana value N or greater" (collect evidence,
forty-one) — a bound on a chosen SET's sum rather than on one
object's number, which is core's `CountBound` and
`CostComponent::TapTotal` territory and wants the counted-group
amount `badGroupPower` already names. The deontic
auxiliaries were a third and have SPLIT the way coordination did:
chapter fifteen took the one-shot restriction clause, and what is
left is the STATIC reading ("Enchanted creature can't attack or
block", Pacifism) with the ability layer, plus the rest of the
auxiliary slot itself — the parser's slot is general, twelve of
them stacking, and the deontic reading was only ever one slice of
it. Coordination was the fourth and has SPLIT: chapter fourteen
took its noun-phrase half — `noun_phrase_coordination` and
`shared_determiner_nominal`, which is Disenchant's alternatives
and Rats of Rath's shared modifier — and the rest is deferred by
axis rather than unmapped, in the frontier entry above.

## Chapter thirty-five — the becomes-status event

Chapter thirty-four gave the grammar [CR#110.5]'s status product and two of its
three consumers: `HasStatus` DESCRIBES a status and `Untap` CHANGES one. This
chapter adds the third — the event that OBSERVES a change — in the shape
finding 244 fixed: an `EventName` row (`StatusChange`) with `TriggeredOnly`
use and a per-value attestation gate on `BeginningOf`'s pattern. Bench:
206 positives / 372 pins.

**Finding 245 — a status transition is its own event, and [CR#603.2e] is the
whole warrant.** The rule gives "becomes" trigger events their reading: they
trigger only at the time the named event happens — not if the state already
exists, and not again while it persists — which for the tap pair is the
change from untapped to tapped and back. That separates three constructions
the vocabulary now keeps apart: the state description (`HasStatus`), the
requested change (`Untap`), and the observed transition (`BecomesStatus`).
The constructor is one row over an ordinary object noun and a status value,
not a per-noun family: the corpus varies the subject across the bare type,
the controller-qualified phrase, the sorted self, and the permanent head
(100 supported "becomes tapped" lines, 33 "becomes untapped"; 109/35 under
all-cards scope — the figures are scope-labeled so a data refresh is not
mistaken for a semantic change) while the event relation never varies. The
subject stands on the battlefield ([CR#110.5d] — only permanents have
status), demanded as `zoneFits`' silence-passing gate, and carries the
family's `SelfSorted` demand — measured this round rather than inherited: the
self subject is one of the family's largest cells ("Whenever this creature
becomes tapped", "Whenever this artifact becomes tapped", and the whole
`Inspired —` untapped family), and every line sorts it. A bare "this becomes
tapped" is written zero times.

**Finding 246 — the attestation gate is a second total reader of the value
product, not an alias of `statusWordOk`.** Eight values, two attested:
"becomes tapped" 100 lines, "becomes untapped" 33, and the other six exact
event phrases zero apiece, individually queried. The face pair is the proof
the two tables are different questions: FaceUp and FaceDown are real
descriptions (`statusWordOk` admits them) and unwritten events
(`statusEventOk` refuses them). The zeroes are phrase-specific — the face
and phasing transitions exist under their own rules and verb families
([CR#708]'s "turned face up", [CR#710]'s flip cards, [CR#702.26]'s "phases
out") and stay on the ledger with the morph and phasing machinery, outside
this row. Six pins, one per unattested value, each posed over an ordinary
subject so the refusal is attributable to the value alone. One of the six is
stronger than attestation: flipping is a one-way process and a flipped
permanent can never become unflipped ([CR#710.4]), so the Unflipped cell
refuses an event the rules cannot produce, not merely one the corpus declines
to write.

**Finding 247 — the grammar admits both `When` and `Whenever` and refuses
`At`, and the residue is now explained rather than merely measured.** The
direct headers split 81 `Whenever` against five Aura `When` spellings on
seven cards ("When enchanted permanent becomes tapped, destroy it"), every
`When` subject attachment-shaped. The style guide (§8, "Triggered
abilities") supplies what [CR#603.1] does not: **When** marks a discrete
event naturally singular in context, **Whenever** a repeatable event class —
and the Aura lines are naturally singular because their effect destroys the
enchanted permanent and ends the relationship. So the split is real, keyed
on the EFFECT, and the grammar deliberately declines to encode it: tolerated
over-generation, the same posture the When/Whenever slot has held since
chapter twenty-eight. `At` stays [CR#603.2b]'s and is pinned. The guide also
forbids `If`/`During`/`Each time`/`After` as trigger words for an ordinary
standalone triggered ability; `TriggerWord` has no such rows, so those
refusals are structural and need no pin.

**Finding 248 — the event preserves its subject through `eventAfter` by
ordinary nominal introduction, because the transition is not a zone
change.** Status is the permanent's physical state and not a characteristic
([CR#110.5,110.5a]), and both directions of the tap pair are a rotation in
place that moves nothing between zones ([CR#701.26a,701.26b] — sideways from
upright, and back again),
so `eventAfter (BecomesStatus n _) = nomIntro n` — `IsDealtDamage`'s row,
not the retagging three's. That is what makes the family's readbacks
structural: Mesmeric Orb's "that permanent's controller" is the sorted
demonstrative (`That PermanentW`, whose word reads the binding's battlefield
zone) under the relational noun, and Mine Layer's "destroy it" would read
the same binding once its mine-counter subject becomes writable. No
reflexive relation is consumed; the ordinary event binding covers every
measured read.

**Finding 249 — "remains tapped" is a duration condition, not an event, and
leaves this family's ledger.** All forty supported "remains tapped" lines
sit inside a "for as long as" adverbial ([CR#611.2b]); zero trigger headers
write the phrase. The enclosed effects range over control grants (9),
stat/keyword grants (13), type settings and copies (3), and locks (15) —
the construction belongs to the duration envelope, not to any effect row.
Giant Oyster and The Pandorica each write the duration BESIDE a separate
"becomes untapped" trigger in one text box, which is [CR#603.2e]'s
distinction printed on a card. No row lands: the temporal spelling "X
remains tapped" (against `Matches`'s present-tense "X is tapped") and the
six lines needing condition conjunction (Hivis of the Scale, Rubinia
Soulsinger, and family; Old Man of the Sea) are the ledgered gap. The
recommended boundary pin — a remains-condition supplied where a transition
event is required — is unwritable today because no "remains" condition
exists to misuse, and is recorded here rather than pinned. The neighbouring
question is settled by the same measurement: no line ends a duration AT the
transition either, so `eventSpan StatusChange = Unattested` is measured, not
assumed. The only two "until … becomes tapped/untapped" lines end at a turn
boundary ("Until your next turn, whenever a creature becomes tapped, destroy
it"; "Until end of turn, it gains … 'Whenever this creature becomes tapped,
…'") — the transition is the nested TRIGGER there, never the endpoint.

**Finding 250 — the doesn't-untap census splits 54/67/1/7, and the standing
row was correctly narrow with stale bookkeeping.** Of the 121 "doesn't
untap during … untap step(s)" lines: 54 are standing statics (15 "your",
39 "its controller's") — `DoesntUntap`'s actual corpus, with Time Vault,
Merieke Ri Berit, and Claustrophobia as exact witnesses; 67 carry
`next`/`next two` and are timed one-shot restrictions (12/55 by possessor;
Reveka, Chandra's Revolution, Frost Lynx) that the endpoint-less standing
row cannot author and must not claim; one line is a non-"during"
parenthetical; and seven "can't untap" lines (Winter Orb: "players can't
untap more than one land during their untap steps") are set-level quantity
caps, a third construction. The 15/39 and 12/55 possessor splits both agree
with the derived-spelling rule (self ⇒ "your", third party ⇒ "its
controller's"), so the constructor keeps no possessor slot. The
constructor's "unmeasured splits" comment is corrected; the timed family's
representation is future design and stays ledgered.

**Finding 251 — the gnarly-inventory discrepancy closes at 43 cards / 44
alias memberships.** The five alias sets hold 44 membership rows over 43
distinct card names; Celestial Dawn is the sole overlap, legitimately
belonging to both `object-class-noun` and `permanent-word` ("Nonland
permanents you control are white. The same is true for spells you control
and nonland cards you own that aren't on the battlefield."). Forty-three is
the card count; forty-four is only the membership count. Bookkeeping;
nothing structural.

Ledger updates this chapter:

- L5/OPEN-5 (**closed**): "remains tapped" classified — duration condition
  inside "for as long as" ([CR#611.2b]); the "remains" spelling and the
  six-line condition-conjunction demand move to the duration/condition
  ledger. Face/flip/phase transitions stay in L5/OPEN-7 unchanged, with
  their rule pointers recorded, all three families now named: [CR#708]
  (face-down spells and permanents), [CR#710] (flip cards), and [CR#702.26]
  (phasing). Flip is a one-way process ([CR#710.4]), so the unflipped
  transition is impossible rather than merely unwritten.
- L6/OPEN-6 (**updated**): 54 standing during-lines (the `DoesntUntap`
  row's), 67 `next`-timed during-lines (own future construction), 1
  non-during line (unclassified conditional), 7 set-level "can't untap"
  caps (own future construction).
- L9/OPEN-4 (**closed**): 43 distinct cards, 44 alias memberships,
  Celestial Dawn the one overlap.
- New near-miss entries from the whole-line-witness discipline, none
  required by this round's row: the Dwarf subtype; Wind and Mine counter
  kinds and a counter-presence noun predicate ("a land with a mine counter
  on it"); the predefined Treasure token; the `Inspired —` ability-word
  presentation; coordinated event subjects; the "to pay a teamwork cost"
  event rider; the attachment edge behind "enchanted permanent" (already on
  the Representability Frontier).
- Scope note: the 100/33 event counts are supported-scope; all-cards is
  109/35.

## Chapter thirty-six — the timed untap restriction

Chapter thirty-five measured the doesn't-untap census (finding 250: 54
standing / 67 timed / 1 replacement reminder / 7 caps) and left the 67 timed
lines as a future construction. This chapter mints it: `DoesntUntapNext`, the
resolving one-shot restriction "[subject] doesn't untap during [possessor]
next [count] untap step(s)", over an ordinary battlefield object noun and a
closed next-step count. Witnesses: Barl's Cage, Take into Custody, Chandra's
Revolution, and Arbalest Elite. The chapter also drains four ledger items
settled elsewhere and repairs three stale in-place figures. Bench:
210 positives / 375 pins (designed 210 positives / 375 pins).

**Finding 252 — the timed restriction is one resolving clause, not a tap
rider and not a duration.** Coding all 67 supported "doesn't untap during …
next … untap step(s)" lines by frame: 48 ride a preceding one-shot action
against 19 that stand alone; of the riders, 38 follow a tap and 10 follow
something else (six mana additions, two damage actions, two shroud grants);
39 restrict the preceding action's patient and 9 name a different referent
(Arbalest Elite restricts the damage SOURCE); 38 are a separate sentence and
10 coordinate in the same sentence. So "rider" is a placement fact the
existing clause sequence consumes, the sentence boundary is the renderer's,
and the construction is one `Effect` row — not a field of `Tap`, and not a
`Continuously` span: a restriction created by resolution lasts as long as its
text states ([CR#611.2a]) where a standing static applies while its source
remains in place ([CR#611.3b]), and what the clause restricts is [CR#502.3]'s
turn-based untap. Opening a general `Duration` cell instead would expose the
endpoint to every `DeedRestriction` — attack and block restrictions for which
it has zero evidence — and "during" names the interval the restriction
governs, not an ending boundary, so neither `Until (StartOf UntapStep …)` nor
`UntilEvent (BeginningOf …)` is the phrase (style guide §11,
"Turn-structure names"; §11, "Continuous and bounded durations"). The
`spanUse`/`partUse`/`eventSpan` zeroes stay closed.

**Finding 253 — the endpoint is construction-owned, and its count is a
closed two-row table.** The complete endpoint inventory of the 67 lines:
"during your next untap step." 12, "during its controller's next untap
step." 54, "during its controller's next two untap steps." 1. Every line
writes "next"; the only explicit count is the word "two", once, and only that
cell pluralizes "steps" — no digit, no bare "the next", no count of three or
more, and no endpoint other than an untap step (style guide §4, "Fixed counts
of things are words"; §11, "This, next, and each"; the contraction is §1,
"Voice and tense" — "does not untap" is written zero times). `NextUntapCount`
is the closed table over the written Nat: rows for one and two, a pin at
three (`badUntapNextThree`). The unattested self-plus-two combination is
ADMITTED as tolerated over-generation rather than pinned: the semantic family
offers no distinction between the self and third-party possessor at count
two, and over-refusal is the forbidden direction — the zero cell is recorded
here instead of gated. [CR#614.10a] settles what "next" means across skipped
untap steps (the scheduled event waits for the first occurrence that is not
skipped); that is the engine's bookkeeping and no part of the phrase.

**Finding 254 — the possessor is derived at the timed site exactly as at the
standing one.** The 12/55 measured split is exactly self ⇒ "your" and third
party ⇒ "its controller's", matching finding 250's standing-row split
(15/39), and [CR#109.5] is the warrant: "you"/"your" refer to the spell or
ability's controller, so the possessor is a function of the restricted noun
and no slot is minted. A plural subject distributes "its controller's" per
object; how the engine snapshots controllers for members that change hands
before their steps is a runtime question outside this grammar, and deciding
it here would have put a slot on the clause that no line writes.

**Finding 255 — totality answers, read against each reader's contract.**
The new row introduces its noun and changes nothing else: `effIntro`,
`preIntro`, and `annIntro` all answer `nomIntro n` — a flat clause with no
retag, no stamp, and no outcome has one answer at all three sites, which is
what lets the family's 22 "it" subjects and 23 sorted demonstratives use the
existing binding discipline with no tap-specific readback — and `deedDelta`
is empty, the announcement telescope already carrying the phrase.
`heldUntilOk` is False (the [CR#610.3] rider rides tagged exile moves and
nothing else), and `costActionOk` is False (no oracle line writes the clause
before a colon). Two answers were revised against the contracts during
design: `reflexEncloseUse` is `EncAgentless`, not `EncUnattested` — the
clause is a declarative that instructs no player, so "do" has nobody to
stand for, `Continuously`'s situation rather than `Untap`'s unattested
imperative — and `effEq` COMPARES, `nounEqRef` on the subject and equality
on the count, because both arguments live in the clause's own context and
the equality contract reserves False for payloads the telescope makes
incomparable. The `isSeq`/`isSim` catch-alls compile untouched.

**Finding 256 — four witnesses span the frame axes; Telekinesis stays
evidence.** Barl's Cage ("{3}: Target creature doesn't untap during its
controller's next untap step.") is the standalone clause under the activated
carrier; Take into Custody ("Tap target creature. It doesn't untap during
its controller's next untap step.") is the separate-sentence tap rider with
the plain pronoun; Chandra's Revolution ("Chandra's Revolution deals 4
damage to target creature. Tap target land. That land doesn't untap during
its controller's next untap step.") is the multi-action sequence whose two
singular mentions force the sorted demonstrative (style guide §5,
"Pronouns"); Arbalest Elite ("{2}{W}, {T}: This creature deals 3 damage to
target attacking or blocking creature. This creature doesn't untap during
your next untap step.") is the non-tap predecessor, the source-not-patient
restriction, and the derived "your" in one ability — its target phrase
already witnessed by `arrowsOfJustice`. Telekinesis, the sole count-two
line, fails the whole-line rule on its [CR#609.7] by-source prevention rider
(ledger L1), so `TwoNextSteps` lands with corpus evidence and no bench
positive — whole-line discipline limits the bench, never the grammar's
evidence. Three pins: a graveyard subject at the battlefield gate
(`badUntapNextGraveyard`), an ambiguous "it" after two singular
introductions (`badUntapNextAmbiguousIt`), and count three
(`badUntapNextThree`). `badUntapLockClause` remains the standing/effect
boundary pin. The single adversarial pass produced 3 expected pin diagnostics
and 3 observed pin diagnostics; after restoration, the build is clean.

**Finding 257 — ledger drain: two recovered rounds, two user rulings, the
cap deferral, the residual line, and three in-place repairs.**

- **ROUND A, "The Blocking Relation," filed as a future round.** The
  blocking relation as read in both voices and tenses (claimed 131 cards /
  72 ex-reminder); a first two-place combat relation and a third
  `groupMention` kind beside `TargetGroup`/`LibrarySlice`; a
  `RemoveFromCombat` verb (claimed 24 operative sites, [CR#506.4]), with a
  Cant≠removal refusal pin per [CR#506.4a]. Claimed to discharge finding
  196's eleven end-of-combat headers and finding 167's Bill Ferny blocker,
  and to complete Labyrinth of Skophos and Hollowhenge Spirit. The counts
  are the prior session's claims, filed as claimed: this round's packet
  carried no re-measurement, and the figures are marked UNVERIFIED until one
  lands.
- **ROUND B, "Combat-Assignment Surgery," filed as a future round.** A
  `CouldBlock` condition (claimed N=2; resolution-time, reading restrictions
  and tapped-ness but not requirements or costs) and a
  `Blocks`/`StopsBlocking` write (claimed N=5, needing the
  remove-then-write versus stops-blocking distinction, [CR#509.3a]). Same
  caveat: counts filed as the prior session's claims, UNVERIFIED.
- **User ruling, ability words:** the grammar gets `AbilityWord Name
  Ability` — a plain two-argument constructor. The word is semantically
  inert but RETAINED, because it prints and this grammar encodes the printed
  line. The shape correlation ("Inspired —" appearing only on "Whenever
  this creature becomes untapped") is content, authored as a MACRO over the
  primitive, never as a gate in the grammar.
- **User ruling, keyword scope:** adding keywords that will eventually
  become macros is OUT OF SCOPE for this workbench until it becomes the
  verifier; `docs/keyword-policy.md`'s keywords-by-name mapping describes
  the OLD Idris model. Flying and Haste are deliberate passes already in the
  tree and stay. Keywords already implemented may be USED where they unlock
  better bench witnesses, but no round mints a new one.
- **The quantity caps are deferred to their own round.** The seven "can't
  untap more than" lines are one parameterized static cap — player domain
  5/1/1 (players/you/your opponents), bound 5/2 (one/two), five object
  sets, step possessor 6/1, embedding 2/4/1 (as-long-as conditional / bare /
  quoted emblem) — and structurally separate from the timed lock: no cap
  line writes "next", no timed line writes "more than", and the cap bounds
  the cardinality of [CR#502.3]'s per-step choice where the lock names fixed
  objects. Implementing it needs three axes the timed row does not:
  bare/all player domains including "your opponents", a set-level "more
  than N" bound, and recurring "your/their untap step(s)" agreement. Mungha
  Wurm ("You can't untap more than one land during your untap step.") is
  the cleanest whole-line witness for that round; Damping Field supplies the
  plural-player cell; Winter Orb tests composition under the existing
  as-long-as wrapper; Dovin Baan's emblem is evidence only. Future pins:
  bound three, a "next" endpoint, a non-untap-eligible set.
- **Bewitching Leechcraft is the one non-"during" line, and it is a fourth
  construction.** The operative quoted ability is a granted static
  replacement effect ([CR#604.2]): "instead" marks replacement
  ([CR#614.1a]) and the replaced untap never happens ([CR#614.6]); the
  parenthetical is reminder text with no game function ([CR#207.2]). It is
  unwritable for three independent reasons — no "would untap" event (the
  nearest `BecomesStatus … Untapped` is trigger-only and fails
  `Interceptable`), `Grantable` rejects nonkeyword static abilities for
  want of quotation, and Aura attachment/"enchanted creature" is absent —
  so it is ledgered with quoted granted replacements and attachment, and
  offers no witness.
- **All-cards lexical outlier, retained as a migration/style note:** Goblin
  Polka Band writes "do not untap during their controllers' next untap
  phases" — outside the supported "doesn't … next untap step(s)" family; it
  does not change the 67-line inventory and is not generalized into the
  grammar.
- **Three in-place repairs**, in chapter twenty-three's annotation form: the
  fronted "For each of [group], …" figure of thirty-seven corpus lines is
  annotated with chapter thirty-two's remeasurement (twenty-five supported
  fronted, fifty-one any-position); ledger L6's primary 121/1/7 text is
  annotated with chapter thirty-five's 54/67/1/7 and this chapter's
  representation of the 67; L7 and the Representability Frontier are
  annotated for chapter thirty-five's becomes-status landing and this
  chapter's timed lock.

Ledger updates this chapter:

- L6 (**updated in place**): the 67 timed lines move to REPRESENTED
  (`DoesntUntapNext`). Remaining in the family: 1 replacement reminder
  (Bewitching Leechcraft — with quoted granted replacements and
  attachment) and 7 set-level caps (own future round, witnesses named
  above).
- L7 and the Frontier (**annotated in place**): becomes-status landed in
  chapter thirty-five; the timed one-shot untap lock lands here.
- Recovered rounds A and B filed above with claimed, UNVERIFIED counts; a
  re-measurement is owed before either round is scheduled.
- The two user rulings recorded verbatim above (ability words; keyword
  scope).
- Telekinesis remains L1-blocked: the only "next two" line, waiting on the
  [CR#609.7] by-source prevention rider; the count-two row carries its
  evidence meanwhile.

## Chapter thirty-seven — the face and phase transitions

Chapter thirty-five closed the becomes-status event on the tap pair and
recorded, in finding 246, why the face and phasing cells refused: those
transitions exist under their own verb families, ledgered, outside that
row. This chapter mints two of the three. `ToFace` turns a permanent face
down or face up ([CR#708]) and `IsTurnedFace` observes the turning;
`Phases` phases a permanent out or in ([CR#702.26]) and `PhaseTransition`
observes the phasing. All four are VALUE-INDEXED over [CR#110.5]'s own
paired values rather than split into four verbs, which is
`BecomesStatus`' idiom carried to a second and third category. The flip
verb does NOT land: its premise failed on measurement and finding 263
records what the corpus actually writes. Bench: 217 positives / 378 pins
(the count of pin declarations across the four proof modules; chapter
thirty-six's "375" does not match the 372 those files held before this
round, a bookkeeping drift not introduced here).

**Finding 258 — the direction is an ARGUMENT, not a second verb, and
[CR#110.5]'s pairing is the warrant.** Status is four categories with two
values each, and English writes the transition verbs the same way: "turn
[n] face down" and "turn [n] face up" are one word taking one complement,
"[n] phases out" and "[n] phases in" one word taking one particle. So the
four rows this round could have been are two, each indexed at its
category — `ToFace : StatusVal FaceC -> …`, `Phases : StatusVal PhaseC ->
…`, and their two event twins — and the index is what closes each
vocabulary: no tap value can be written where a face value belongs and no
attestation table is needed to say so, the type saying it instead. That
is the sharpest structural gain of the round, and it is a gain the
becomes-status row already paid for: `BecomesStatus` is one row over a
value with a table beside it, and these are the same shape at narrower
domains, two of them needing no table at all. The one place the idiom
does NOT extend is the SPELLING. `ToFace` writes the status value's own
word ("turn target creature face down"), exactly as `BecomesStatus`
writes it; `Phases` writes the PARTICLE ("out", "in") where the value's
own word is "phased out", because the verb and the state are different
English. The row comments say so at the spelling line, and no renderer
may derive one from the other.

**Finding 259 — the face EVENT is one direction only, and the zero is
pinned rather than left structural.** This session's recon counts 134
"is turned face up" trigger lines and zero "is turned face down" ones;
the corpus script's distinct-line view of the same family is 113 supported
lines up, zero down (the two figures differ by method — the recon counts
lines as printed, the script deduplicates identical lines across cards —
and neither disturbs the ratio, which is a mass against a zero). Both
header words are written and the split runs opposite to the
becomes-status family's: "When this creature is turned face up, …" is the
morph mass, "Whenever a permanent you control is turned face up, …" the
description-subject family, and the grammar declines to encode the
difference exactly as everywhere else ([CR#603.1] assigns no word). `At`
is refused ([CR#603.2b]). The refused DOWN cell is `faceEventOk`'s, a
two-row table over `StatusVal FaceC`, with `badTurnedFaceDownEvent`
stating the measurement — finding 246's discipline, not a silently
missing constructor, and the distinction matters here because the
face-down turning is a real and frequent game event ([CR#708.2a]) that no
header names. Its refusal is attestation alone, which is exactly how it
differs from the flip pair, whose unflipped cell [CR#710.4] makes
impossible.

**Finding 260 — the face-down direction is written exactly once as a
clause, and it is a DURATION endpoint.** Vesuvan Shapeshifter: "until this
creature is turned face down, it becomes a copy of that creature". That
is the only clause of any kind in the corpus over the face-down turning,
and it belongs to the duration reader, not to the trigger header —
`eventSpan TurnedFaceUp` is `Unattested` for the direction the grammar
HAS, and the direction it does not have would need an endpoint the
`Duration` vocabulary would have to open. Recorded rather than built: one
line, and it also wants `becomes`' copy word. The neighbouring measurement
is the same shape and settles the phase row's cell: no line ends a
duration at a phasing either, the nearest ("target creature phases out
until this enchantment leaves the battlefield") ending at a DEPARTURE
while its effect is a phase-out.

**Finding 261 — phasing has no imperative form at all, and the clause is
subject-first because [CR#702.26a] is.** "Phase out target creature" is
written zero times. Every line is the intransitive declarative "Target
creature phases out", the patient as SUBJECT, which is the rule's own
wording for the turn-based action — "all phased-in permanents with phasing
that player controls 'phase out'". So `Phases` spells
"<Param(1)> phases <Param(0)>" where `Tap` spells "tap <Param(0)>", which
is `DealDamage`'s and `Fights`' shape and no new capability: the effect
layer's spelling templates already write subject-first frames, so
assumption A1 held on inspection rather than on argument. The clause is
still an ordinary resolving effect — `reflexEncloseUse` is `EncAgentless`
for the reason the spelling is subject-first, no player being instructed
([CR#603.12]).

**Finding 262 — the phase pair needs no attestation table, and the effect
cells are lopsided in a way the ledger records rather than the type.**
Trigger headers: seven lines each direction by this session's recon, six
distinct lines by the script, and Teferi's Imp writes both directions on
one card — so the EVENT row opens both cells with no table. The EFFECT
cells are 60 phase-out lines against a single effect-position phase-in:
The Pandorica's "When The Pandorica becomes untapped or leaves the
battlefield, that permanent phases in", verified this round against the
corpus rather than assumed, with one plural sibling ("Simultaneously, all
phased-out creatures phase in and all creatures with phasing phase out").
Every other "phases in" occurrence is reminder text or a trigger header.
One attested line is a written cell, so the row admits it and no
per-value effect table is minted; but the Pandorica line's coordinated
trigger event ("becomes untapped or leaves the battlefield") is
unwritable here, so the cell lands with corpus evidence and NO bench
positive — `TwoNextSteps`' posture from chapter thirty-six, where
whole-line discipline limits the bench and never the grammar's evidence.

**Finding 263 — the flip verb does not land, because its premise failed
on measurement.** The round's brief supposed an argument-less self-flip
row whose family was the five Kamigawa ascendants. Neither half survives
the count. Fifteen supported lines write [CR#710]'s keyword action, and
the surface splits seven "flip this creature", four "flip it", and four
naming an ascendant (Kuon, Sasaya, Erayo, Rune-Tail — four, not five;
Homura writes "return it to the battlefield flipped" instead and is a
different construction). Every one of the fifteen flips the SOURCE, so
the semantic claim is right — there is no "flip target creature" and no
other-object flip anywhere — but every one of the fifteen also writes an
explicit patient, and no `Effect` row in this grammar takes none. An
argument-less row would therefore spell a sentence the corpus never
writes while failing to spell the three it does, and the implicit-self
precedent that exists ({T} and {Q}, [CR#107.5,107.6]) is a COST surface,
not an effect one. The right shape is a self-subject noun argument —
`Tap`'s shape with a sortedness demand — and choosing it is a design call
this round declines to improvise. What blocks a witness independently:
all fifteen lines sit under machinery that does not exist here (an
intervening-if condition on a life total or a hand count, an ordinal
spell-count trigger, a died-this-turn count), so no ascendant elaborates
end-to-end today regardless of the row's shape. The flip family stays on
L5's ledger with the shape question stated.

**Finding 264 — witnesses and pins.** Seven positives: Cyber Conversion
("Turn target creature face down.", its second sentence elided —
[CR#708.2a] makes those the LISTED characteristics the permanent takes
instead of the 2/2 default, and setting base characteristics is the layer
family's word); Break Open, the whole card, where `HasStatus FaceDown`
describes the patient and `ToFace FaceUp` changes it on one line; Secret
Plans' trigger over a description subject; Vodalian Illusionist's
activated phase-out; Teferi's Imp's two triggers, the paired directions on
one card; and Shimmering Efreet, the phase-in EVENT with a phase-out
EFFECT in its body. Six pins: two zone gates (`badTurnFaceDownGraveyard`,
`badPhasesOutInHand`), the face-down event cell
(`badTurnedFaceDownEvent`), the `At` header (`badAtPhasesOut`), and the
two silent readers — nothing replaces a phasing
(`badInterceptPhasesOut`) and no [CR#610.3] rider waits for a turning
(`badHeldUntilTurnedFaceUp`). The gates are `Tap`'s and only `Tap`'s: no
status precondition rides any value, and [CR#110.5] is why the grammar
cannot carry one — status is the permanent's state at a moment of play
and not a fact about the phrase, so "a face-down permanent can't be
turned face down" ([CR#708.2b] — "nothing happens") is the resolving
engine's no-op exactly as tapping a tapped permanent is. The battlefield
gate needed one argument it had not needed before: a phased-out permanent
has NOT left the battlefield ([CR#702.26d] — the phasing event "doesn't
actually cause a permanent to change zones"; it is only treated as though
it does not exist, [CR#702.26b]), so the phase-IN subject is a battlefield
object and the same demand fits it.

Ledger updates this chapter:

- L5/OPEN-7 (**partially closed**): the FACE family and the PHASING
  family are represented — `ToFace`/`IsTurnedFace`/`faceEventOk` and
  `Phases`/`PhaseTransition`, with `TurnedFaceUp` and `PhasingChange` as
  their `EventName` rows, all four readers answered per row. What remains
  in the item: the FLIP verb (finding 263 — shape question stated,
  premise refuted, no witness available); the twenty-two participial
  "phased out" lines and the lone "flipped" line, which are
  `statusWordOk`'s refused description cells and not these verbs';
  off-battlefield face orientation (face-down exile, library piles,
  draft), expressly not status under [CR#110.5d]; the two "turn the
  exiled card(s) face up" lines, which the battlefield gate refuses and
  which belong to the exiled-card linkage rather than to this verb;
  indirect phasing ([CR#702.26g] — attached objects phase out with what
  they are attached to), which has no oracle surface at all and is engine
  bookkeeping; and the coordinated event subject that blocks Pine Walker,
  King of the Oathbreakers, and The Pandorica alike.
- L7 and the Frontier (**annotated in place**): the face and phase
  transition events land here beside chapter thirty-five's
  becomes-status.
- The face-down DURATION endpoint (Vesuvan Shapeshifter, finding 260) is
  a new near-miss for the duration ledger: one line, wanting both an
  endpoint the `Duration` vocabulary does not open and the copy word.
- Near-miss inventory from the whole-line discipline, none required by
  this round's rows: the coordinated trigger subject ("this creature or
  another creature you control"); the coordinated trigger EVENT ("phases
  out or leaves the battlefield", "becomes untapped or leaves the
  battlefield"); the Cyberman and Detective creature types; Secret Plans'
  and Teferi's Imp's sibling lines (a face-down-scoped stat static, the
  `Phasing` keyword).

## Chapter thirty-eight — the blocking relation

The combat vocabulary has had the block DESIGNATION since chapter
seventeen (`Blocking`, the word "target attacking or blocking creature"
coordinates) and the block EVENT since chapter twenty-eight (`Blocks`,
one-place). What it has never had is the RELATION — which creature is
blocking which — and without it the eleven end-of-combat bodies finding
196 counted, Labyrinth of Skophos, and Hollowhenge Spirit all sat
unwritable. This chapter mints it: two relational predicates
(`BlockerOf`, `BlockedBy`), an optional partner slot on `Blocks` and a
new passive event `BecomesBlocked` beside it, and the verb
`RemoveFromCombat`. Bench: 224 positives / 384 pins.

**Finding 265 — the relation is two ordinary modifiers, and the round's
group-mention premise was wrong.** The recovered round claimed this family
needed "a third `groupMention` kind beside `TargetGroup`/`LibrarySlice`".
It needs none, and the claim was wrong twice over: `groupMention` already
answers True on FIVE rows (`TargetGroup`, `Them`, `Those`,
`LibrarySlice`, `ThoseVerbed`), and the phrase the claim was reaching for
— "all creatures blocking or blocked by this creature" — is a determiner
over a description, not a group mention at all. `AllOf` takes an ordinary
predicate; the predicate is `And [creature, Or [BlockerOf …, BlockedBy
…]]`; the head is the type word and the relation is two postnominal
participial modifiers under it. Nothing new is spent. That is the whole
mechanism behind the direct end-of-combat bodies, and Kjeldoran
Frostbeast is the whole card written out of it.

**Finding 266 — the two predicates are two rows because the corpus writes
two phrases, and neither carries a singularity gate.** `BlockerOf` spells
"blocking [m]" ([CR#509.1g] — the blocker "is blocking the attacking
creatures chosen for it") and `BlockedBy` spells "blocked by [m]"
([CR#509.1h] — the attacker "becomes a blocked creature").
`ControlledBy`'s shape for both, and `ControlledBy`'s naming for the
passive one. What does NOT transfer is `ControlledBy`'s one-possessor
gate: that is [CR#109.4]'s fact about control, and the block relation is
many-to-many in both directions by rule — an attacker may be blocked by
"one or more creatures" ([CR#509.1h]) and a blocker may be blocking "the
attacking creatures chosen for it" ([CR#509.1g], plural). What DOES ride
each row is a zone demand on the RELATUM, [CR#506.3]'s creature and the
battlefield the relation lives on (`badBlockingGraveyardRelatum`), and a
`negatable` of False: "not blocking [m]" and "not blocked by [m]" are
zero corpus lines apiece, `ExiledWith`'s situation exactly, while the two
negations the family does write — "target nonattacking, nonblocking
creature" and "if it wasn't blocking" — are the bare designation's and
leave that row True (`badNegatedBlockedBy`).

**Finding 267 — the designation word and the relational word are
different readers, and [CR#509.4] is the proof rather than a stylistic
preference.** It would have been tempting to give `Blocking` an optional
relatum and call the bare row its `Nothing` case. Two things forbid it.
The corpus: "attacking or blocking" coordinates the bare designation with
`Attacking` and never takes a complement, while "creature blocking this
creature" names the relatum and never coordinates — two phrases with
disjoint syntax. And the rules: a creature put onto the battlefield
blocking "is 'blocking' but, for the purposes of trigger events and
effects, it never 'blocked'" ([CR#509.4]), so the STATE and the EVENT can
come apart on the same object in the same combat. A vocabulary that
routed both through one row would have to make that difference somewhere
else. Three rows for three questions: `Blocking` describes the
designation, `BlockerOf`/`BlockedBy` describe the relation, `Blocks`
watches the declaration.

**Finding 268 — the block event is ONE turn-based action with TWO
subjects, and the complement is a slot rather than a second pair of
rows.** [CR#509.1g] and [CR#509.1h] are the same declaration written
twice: the chosen creature "becomes a blocking creature" and the attacker
it was chosen for "becomes a blocked creature". So `Blocks` and
`BecomesBlocked` are two rows with two `EventName`s
(`BlockDeclaration`, `BlockedDeclaration`) answering the four readers
separately — and each takes an OPTIONAL partner, because the corpus
writes both halves of each: 24 transitive "Whenever this creature blocks
a creature with flying, …" headers against the bare form's mass, and 61
"Whenever this creature becomes blocked by a creature, …" against the
bare "becomes blocked". One slot, `CtrlSingular`'s shape — a
`BlockPartner` family over the `Maybe` with a row per case — so the gates
that would ride a required noun ride the written one and ask nothing of
the unwritten one (`badBlocksBareThisPartner`). The partner THREADS the
subject's context (`Fights`' second slot), which is what lets Vertigo
Spawn's body read the patient twice through the sorted demonstrative. The
row demands nothing about legality: that a blocker must be untapped
([CR#509.1a]) is the declaring engine's check on a turn-based action and
not a fact about the sentence, `Tap`'s posture at a new consumer. And
`BecomesBlocked` is emphatically NOT a becomes-status event — blocked-ness
is a combat designation that no [CR#110.5] category holds, which is why
`StatusCat` still has four rows.

**Finding 269 — `RemoveFromCombat` is the one sentence in [CR#506.4]'s
list, and the gate is zonal only.** The rule enumerates every way a
permanent leaves combat — leaving the battlefield, changing controller,
phasing out, regenerating, ceasing to be a creature — and exactly one of
them is a clause a card writes: "an effect specifically removes it from
combat". The verb takes `Tap`'s gate and only `Tap`'s. Being attacking or
blocking is dynamic combat state, the same argument [CR#110.5] forces on
the status verbs, and the corpus settles it at the surface: every
operative line puts the combat words in the target DESCRIPTION ("Remove
target attacking or blocking creature from combat"), where `Attacking`
and `Blocking` already live. This session's recon counts 25
effect-position lines; a filter excluding regeneration's reminder text
("instead tap it, remove it from combat, and heal all damage on it",
[CR#701.19]'s wording, which is the overwhelming majority of all "from
combat" matches) finds 22 — the same family, two methods.

**Finding 270 — [CR#506.4a] is a real distinction and no phrase refuses
it, so it is recorded and not pinned.** The rule: "once a creature has
been declared as an attacking or blocking creature, spells or abilities
that would have kept that creature from attacking or blocking don't
remove the creature from combat." The corpus writes both sides
independently — "{1}{R}, {T}: Target creature can't block this turn" is
an activated prohibition usable after blockers are declared, where
Labyrinth of Skophos removes — and both sides are independently
spellable here, `Cant`'s deed restriction and this row. The queue asked
for a refusal pin. There is none to write: the difference is what the two
clauses MEAN at resolution, not a shape one of them cannot take, and a
pin that merely posed a `Cant` where a `RemoveFromCombat` belongs would
refuse nothing — both terms typecheck, as they should. Recorded here
against the rule's text, in the form finding 249 used for the unwritable
remains-condition boundary.

**Finding 271 — the past-tense lookback does not land, and the reason is
that it is not this axis.** The relational pair describes a state holding
NOW: [CR#509.1g] says the blocker "remains a blocking creature until it's
removed from combat or the combat phase ends". What the eight
past-relative lines write is different — "each creature that blocked this
turn", "creatures that were blocked by that creature this turn" — a
lookback over what HAPPENED, carrying its own span, and the four delayed
end-of-combat forms are the same construction. Giving the participial
rows a duration slot would have been the cheap move and the wrong one: it
would put a span on a DESCRIPTION, which is the cell finding 252 declined
to open without evidence, and it would still not express the tense, a
creature that blocked and was then removed from combat satisfying the
lookback and failing the description. The honest shape is an event-history
query — the axis core spells as a condition lookback over the shared
`EventName` vocabulary — and that is a round, not a slot. So the present
participial pair lands alone, and seven of finding 196's eleven
end-of-combat bodies become writable while the four delayed forms wait
with the eight other lookback lines.

**Finding 272 — witnesses, pins, and the bare cell's witness swap.**
Seven positives: Labyrinth of Skophos and Hollowhenge Spirit, the verb
under its two carriers (activated and entry-triggered), each with the
combat words in the target description; Netcaster Spider, the transitive
block header with a described patient; Vertigo Spawn, the transitive
header whose body reads the patient twice through the sorted
demonstrative and reuses chapter thirty-six's timed lock; Viashino
Weaponsmith, the passive event with its "by" complement written;
Somberwald Alpha, the same event with the complement left out; and
Kjeldoran Frostbeast, the whole card, the relational pair under one
determiner. The bare cell's witness is a DESCRIPTION-subject line and
had to be: every self-subject line in that cell writes its body with the
pronoun ("Whenever this creature becomes blocked, IT gets +1/+1 until end
of turn" — Deeproot Warrior, and the whole bushido and rampage reminder
family), and the self-word announces no mention for `It` to read
(chapter thirty's split), so Deeproot Warrior is evidence and Somberwald
Alpha is the bench. Bill Ferny is attempted and DECLINED: its
becomes-blocked header, its modal, its gain-control mode and the Horse
subtype all exist, but both modes create Treasure tokens and the
predefined-token catalog ([CR#111.10]) does not, and a modal missing a
mode's body is not the card. Six pins: two zone gates
(`badRemoveFromCombatGraveyard` on the verb, `badBlockingGraveyardRelatum`
on the relatum), the relational negation (`badNegatedBlockedBy`), the
`At` header (`badAtBecomesBlocked`), the partner's sortedness
(`badBlocksBareThisPartner`), and the silent reader — nothing replaces a
block declaration, the abilities that stop one being restrictions checked
as it is made ([CR#509.1b]) rather than replacements of an event
(`badInterceptBecomesBlocked`).

Ledger updates this chapter:

- The blocking-relation round (**partially closed**): the relation, both
  event voices, and the verb land. Remaining and narrowed in the queue:
  the past-tense lookback (finding 271 — 8 past-relative lines plus
  finding 196's 4 delayed end-of-combat forms), and Bill Ferny, now
  blocked on the predefined-token catalog alone.
- **Combat-assignment surgery stays a separate future round**, untouched
  by this one and unblocked by it: its corpus is 1 stops-blocking line
  and 19 could-block lines, and it needed this round's relation to exist
  first ([CR#509.3a]'s remove-then-write versus stops-blocking
  distinction).
- **The 38 "blocks … this turn if able" requirement lines are NOT this
  round's** and are routed to the queued "deontic beyond `Cant`" entry:
  [CR#509.1c] makes a requirement a check on the declaration, the
  positive twin of the restriction `Cant` already writes, and it wants
  the deontic axis rather than the block event.
- Also deferred out, each to its standing queue entry: the bare
  participial "blocked"/"unblocked" descriptions (participle verb tags);
  banding; multi-blocker ordering; attacks-alone (attack-side).
- Near-miss inventory: the coordinated block EVENT ("blocks or becomes
  blocked by a creature") is the family's single largest unwritable
  frame and joins round thirty-seven's coordinated-event entries; "the
  blocking creature" as a definite determiner over a description is the
  standing spelling residue, and it is what keeps the recon's own
  transitive example ("Whenever a creature blocks a black or red
  creature, the blocking creature gets +1/+1 until end of turn") off the
  bench; the predefined Treasure token, again.

## Chapter thirty-nine — the set-level untap cap

Chapter thirty-five's doesn't-untap census split the family four ways
(finding 250: 54 standing / 67 timed / 1 replacement reminder / 7 caps);
chapter thirty-six minted the timed lock and deferred the caps to their
own round, naming three axes they would need. This chapter mints them —
`CantUntapMoreThan`, over a closed player DOMAIN and a closed BOUND — and
finds that the third axis was never an axis. Bench: 229 positives / 388
pins.

**Finding 273 — the cap bounds a CARDINALITY, and that is why it is not
`DoesntUntap` with a number.** [CR#502.3] gives the untap step one
turn-based action in two sentences: "the active player determines which
permanents they control will untap. Then they untap them all
simultaneously." The standing lock names FIXED objects and takes them out
of the second sentence; the cap names no object at all and bounds how
many the FIRST sentence may include. That is a different statement about
a different half of the same rule, which is why the row's subject is a
player domain rather than a noun phrase and its object a described SET
rather than a phrase naming anybody — and why it announces nothing
(`staticIntro` answers the incoming context, the first static row with no
subject phrase to contribute, which is `BeginningOf`'s answer at the
event layer for `BeginningOf`'s reason). It files under `DeedRestriction`
beside `DoesntUntap` all the same: one class of statement per kind-keyed
reader, the `EntersWithCounters` precedent the untap lock already cited.

**Finding 274 — the domain is three words, and NOT a step toward plural
player nouns.** The family is closed at seven distinct lines on eight
cards, and the domain across them is bare "Players" five times, "You"
once (Mungha Wurm), "Your opponents" once (Dovin Baan's quoted emblem).
Nothing else is ever written there — no "each opponent", no described
player set — so `CapDomain` is a three-value enum, `StatusVal`'s idiom at
a three-cell domain. The alternative was a plural player NOUN, and
declining it is a scoping choice this chapter records rather than hides:
such a noun is a real axis with real evidence elsewhere (the counter
verbs' PLAYER holder, forty-eight lines, ledgered), and building it on
seven lines of cap evidence would have spent it everywhere at the price
of one family's convenience. When that round comes, this enum is three
call sites to revisit, which is the cheap direction to be wrong in.

**Finding 275 — the third axis dissolved: the step's possessor is a
spelling agreement, not a slot.** Chapter thirty-six listed three things
the cap would need that the timed lock did not, and one of them was
"recurring 'your/their untap step(s)' agreement". Measured, it is not a
degree of freedom at all: "your untap step" occurs with `Yourself` and
"their untap steps" with the other two domains, one to one across all
seven lines, with the number of the step word agreeing too. So the
possessive is a FUNCTION of the domain and the spelling owns it — which
is exactly the arrangement `DoesntUntap` already had, its two spellings
agreeing with its subject ("your untap step" for the self, "its
controller's" otherwise), and exactly the arrangement finding 254 found
at the timed row. Three rows now derive the same possessive from their
own subject and none of them carries a slot for it. `Whose` stays
two-valued; no agreement machinery exists in this grammar and none was
needed. The general lesson is worth stating because it has now been paid
for three times: an axis that varies perfectly with something already
written is a spelling fact wearing an axis's clothes, and the way to tell
is to tabulate the pair rather than to count either side.

**Finding 276 — the bound is a closed two-row table and the comparator is
construction-owned.** `CapBound` is `NextUntapCount`'s move at a second
site: rows for one (five lines, singular set word) and two (two lines,
Static Orb and the emblem, "permanents" plural), with three and up
refused (`badUntapCapThree`). And the comparator needs no vocabulary,
because the corpus has exactly one phrasing of it: "can't untap more than
N", every line, with no "at most" and no "untap only" anywhere in the
family. Storage Matrix's "can untap only permanents of the chosen type"
is not the exception it looks like — it is a different construction
entirely (finding 280) and its "only" is not this comparator.

**Finding 277 — the capped set is an ordinary predicate carrying `AllOf`'s
demands, and the fifth set word is what it costs.** Five sets are
written: land, artifact, creature, permanents, and nonbasic land. Four
are existing vocabulary and the row takes them as a bare `Predicate` with
no determiner over it, gated exactly as the universal determiner gates
its own — the phrase must say a head noun (`Headed`;
`badUntapCapHeadless` refuses "more than one tapped", the status word
heading nothing), must not hide the damage class, and must describe
battlefield permanents, [CR#502.3]'s untap reaching nothing else
(`badUntapCapGraveyardSet`). The fifth needs a SUPERTYPE predicate:
"nonbasic land" is [CR#205.4c]'s own term for a land without the
basic supertype, and supertypes exist in
this grammar only as a printed card's field, never as a description
(ledgered since chapter thirty). So Winter Moon is corpus evidence with
no bench positive — `TwoNextSteps`' and the phase-in cell's posture, the
third time this round-shape has come up and the first time it was
predicted before implementation rather than discovered during it.

**Finding 278 — composition needed nothing.** Two of the seven lines
write the cap inside "as long as this artifact is untapped" (Winter Orb,
Static Orb). `Conditionally` wraps it untouched: no cell opened, no gate
relaxed, no nesting demand disturbed, because [CR#611.3a] licenses the
wrapper over any static statement and the cap is one. Both cards are
whole-card witnesses. This is the round's cheapest result and worth
recording as such — a new `StaticEffect` row that composes for free is
evidence the wrapper was cut at the right joint.

**Finding 279 — the "next" endpoint pin fell as a real refusal, not a
structural absence.** Finding 257 named a future pin here and the shape
was genuinely uncertain: the row has no timing slot of its own to refuse,
so the honest expectation was a structural absence recorded in prose. It
is not. The cap composes under `Continuously`, which HAS a span slot, and
`spanUse` already answers `Unattested` at every untap-step endpoint — so
"Players can't untap more than one land until your next untap step" is a
writable-looking sentence that the existing span table refuses, and
`badUntapCapUntilNextUntapStep` is that refusal at this row
(`SpanOk DeedRestriction (Just (Until (StartOf UntapStep (Just
Yours))))`). The cap names the interval it governs in its own words
("during their untap steps") and takes no endpoint on top; the pin now
says so. Recorded here because the round's brief asked which way it fell.

**Finding 280 — four neighbouring families, measured and left.**

- **Storage Matrix is a corpus SINGLETON and a different construction.**
  "As long as this artifact is untapped, each player chooses artifact,
  creature, or land during their untap step. That player can untap only
  permanents of the chosen type this step." It is a choose-a-category
  effect with a per-step chosen value read back by a following sentence —
  neither a cap nor a lock — and one line does not buy a construction.
  Recorded, no queue entry.
- **The Seedborn family — "untap … during each other player's untap
  step", 12 lines — is the cap's mirror image and its own round.** It
  GRANTS an untap where these deny one, and it needs the player-domain
  axis this chapter deliberately did not build (finding 274). I propose a
  queue line for it and have NOT added one; the coordinator's call.
- **The may-choose-not-to-untap deontic routes to the deontic entry**,
  exactly as round thirty-eight's "blocks if able" requirements did: this
  session's recon counts 8, a distinct-line filter finds 7 ("You may
  choose not to untap Hivis / Rubinia Soulsinger / The Blackstaff of
  Waterdeep / The Pandorica / this artifact / this creature / this land
  during your untap step") — the same family, two methods. It is a
  PERMISSION over the turn-based action, the positive twin of the
  restriction, and belongs with `MayPlay`'s axis rather than here.
- **Mudslide's pay-to-untap is not an untap-step rider at all.** Its
  first line is an ordinary standing lock ("Creatures without flying
  don't untap during their controllers' untap steps"); its second is an
  UPKEEP trigger with a per-object cost ("At the beginning of each
  player's upkeep, that player may choose any number of tapped creatures
  without flying they control and pay {2} for each creature chosen this
  way. If the player does, untap those creatures."). One card, and the
  interesting half wants per-member cost scaling, which is the dependent
  iteration entry's business. Recorded with the correction.

Ledger updates this chapter:

- L6 (**closed**): the doesn't-untap census is now fully represented or
  fully classified. 54 standing (`DoesntUntap`, chapter thirty-four), 67
  timed (`DoesntUntapNext`, chapter thirty-six), 7 caps
  (`CantUntapMoreThan`, here), 1 replacement reminder (Bewitching
  Leechcraft, ledgered with quoted granted replacements and attachment).
  Nothing in the family is unclassified.
- The `YourOpponents` cell is ADMITTED with corpus evidence and no bench
  positive: its one printed line is inside Dovin Baan's quoted emblem,
  and emblem quotation is unwritable here (`Grantable`'s stance). The
  cell is exercised as a well-formed term inside `badUntapCapThree`,
  whose refusal is the BOUND, so nothing in the round leaves the value
  untypechecked.
- Winter Moon (**recorded, not witnessed**): waiting on a supertype
  PREDICATE, which is the description-side half of a field chapter thirty
  minted for the printed card only.
- Near-miss inventory: the supertype predicate ("nonbasic land"); the
  per-member cost ("pay {2} for each creature chosen this way",
  Mudslide); the choose-a-category-then-read-it-back construction
  (Storage Matrix); emblem quotation, again.

## Chapter forty — the player's counters

[CR#122.1] has said since the beginning that a counter is "a marker
placed on an object OR PLAYER", and until this round every row in the
catalog was an object's. This chapter reads the rule's other half: three
player kinds, the two verbs that reach them, the count read at both
holder sorts, and the last-removal event the counter verbs have owed
since finding 198. One scope table carries all of it. Bench: 235
positives / 394 pins.

**Finding 281 — round thirty-nine's prediction was wrong about WHERE the
plural-player axis lives, and the deferral survives the correction.**
Finding 274 declined to mint a plural player noun on seven lines of cap
evidence and named the counter verbs' player holder as the round that
would motivate it properly. Measured, that round does not: the counter
recipients are You 41, target player 5 and target opponent 3, each
player 6 and each opponent 5, defending player 2, and the anaphoric
"that player" 13 — every one an existing noun row — while "your
opponents" as a counter recipient is written ZERO times. The stale
"forty-eight lines" figure the docstring quoted (finding 84's) does not
reproduce either; today's numbers are 66 non-energy kind-named lines
against 145 with energy. So the decision stands and its stated reason
does not: the sole remaining consumer of the plural-player axis is the
recurring untap GRANT (Seedborn Muse's twelve lines). Both live pointers
are amended — `CapDomain`'s docstring and the queue's Seedborn line —
and the log is left as written, this finding being the correction.

**Finding 282 — the scope table answers in `Kind`, and that is what makes
the whole round cheap.** `counterScope : CounterKind -> Kind` is full
rows over the catalog: the four object products answer `Object`, the
three new kinds answer `Player`. Every consumer then gates by ordinary
equality — `counterScope kind = Object` on the put and remove verbs,
`= Player` on the two player verbs, and `= k` on the count read, whose
own `{k : Kind}` index is the holder's sort. One table, five gates, no
scope enum of its own and no per-verb recipient description. Answering in
a bespoke `Scope` type would have cost a second mapping into `Kind` at
the read; answering in `Kind` bought the holder agreement for free.

**Finding 283 — two verbs, and the corpus separates them absolutely.**
Objects never "get" a counter (zero lines) and players never receive
"put" (zero lines), so `GetsCounters` is a different WORD and not a
widened `PutCounters` — which is what lets the scope table gate both
rather than each verb re-describing its own recipients
(`badPutPoisonOnCreature`, `badGetsBoostCounter`). The player verb is
agent-silent for a sharper reason than its object twin: the recipient is
the SUBJECT, so there is no room for an agent phrase at all. And
[CR#702.90b] is the one place the rules and the oracle disagree about the
word — infect "causes that source's controller to GIVE the player that
many poison counters" where every printed line says "gets" — which this
grammar resolves the way it always does, by recording what cards print.
The one recipient the round does not reach is "defending player" (2
lines), wanting a combat-role player predicate; recorded, not minted.

**Finding 284 — Ticket is refused, and the reason is its SPELLING REGIME
rather than its count.** Both ticket lines write "You get {TK} (a ticket
counter)": symbol notation with the counter name in a parenthetical
gloss. That is energy's regime, not this verb's — energy's 79 lines
write {E} for acquisition and "Pay {E}{E}{E}" for spending, both
symbol-notated — so ticket routes with energy to the symbol round and
waits there rather than waiting on a line count. This matters because
finding 85's test is about SHAPE and not volume (it admitted `Stun` on
one line and refused `charge` on sixty-six), and a two-line kind of the
right shape would have passed; ticket fails a different test, and saying
which one is the point.

**Finding 285 — the player's removal verb is always-all, so its
kind-blind cell is IN this vocabulary while the quantifiers stay out.**
All four corpus lines remove the whole holding and none removes a
number: "Each opponent loses all counters" and "Target opponent loses all
counters" name no kind, "Target player loses all rad counters" and
"Target player loses all poison counters" do. So `LosesAllCounters`'
`Maybe CounterKind` is not a quantity term in disguise — it says whether
the sentence NAMES a kind, not how many of one it takes — which is
exactly why it can land here while "a counter" (82 lines), "with a
counter on it" (26) and "move a counter" (36) stay ledgered, all three
wanting a quantity over KINDS the `Amount` vocabulary has no term for.
The gate is `CtrlSingular`'s and `BlockPartner`'s shape at a third site
(`PlayerCounterKind`), riding the written cell and asking nothing of the
unwritten one (`badLosesAllBoostCounters`).

**Finding 286 — the count read is ONE row, and round thirty-nine's lesson
is why.** The corpus writes two spellings, "the number of [kind] counters
on [object]" (296 lines) and "the number of [kind] counters [player] has"
(9). They vary perfectly with the holder's SORT and with nothing else,
which finding 275 already named: an axis that varies perfectly with
something written is a spelling fact wearing an axis's clothes. So
`CountersOn` is one `{k : Kind}`-indexed row in `Matches`' and
`DealDamage`' idiom, the possessive spelling deriving from the holder,
and the kind tied to the holder by the one table
(`badCountersHeldByPlayer`). The PLAYER spelling is witnessed (Kratos,
Stoic Father, feeding the object verb's amount slot, where the phrasal
amount postposes exactly as `rabidBite`'s "equal to its power" does); the
OBJECT spelling, at thirty-three times the line count, is not — every
clean candidate fails for a reason outside this row. Two write the
recipient before the amount ("deals damage to target creature equal to
the number of +1/+1 counters on this creature"), which is a linearization
the spelling layer does not have; two feed token catalogs this grammar
lacks; one uses the read as a phrasal `Compare` bound, which
`writtenBound` refuses by design, that being the ledgered phrasal-standard
frame. The commonest spelling in the family lands unwitnessed, and saying
so is more useful than reaching for a witness that elides its way in.

**Finding 287 — the last-removal event, and fading is not it.**
`LastCounterRemoved` watches the removal that empties a holding, the
third of the three abilities suspend represents ([CR#702.62a] writes the
header out; [CR#702.63a] writes the same one into vanishing). Fading
looks alike and is a different mechanism: [CR#702.32a] says "remove a
fade counter from this permanent. If you CAN'T, sacrifice the permanent"
— a failure-to-remove check inside the upkeep trigger, firing when the
holding is ALREADY empty, where this row fires on the removal that
empties it. One counts down to zero and acts; the other tries at zero and
fails. The subject is an OBJECT and never a player — structural, not
measured-and-refused, the noun slot being object-sorted so there is no
cell to pin, and the corpus agreeing at zero lines. Its zone demand is
the counter family's own two-zone table, which is not a convenience:
every explicit line watches a card IN EXILE. And the row lands with NO
bench positive, all five explicit lines blocked, with one cause running
through all three time lines: each writes "while it's exiled" or "if it's
exiled" on the header, which is the intervening-if seam ([CR#603.4],
queued since finding 76). Suspend's own line is additionally a granted
QUOTED ability; the Knight-token line additionally wants flanking and
protection from white; the third line ("creatures can't be blocked this
turn") wants nothing else at all and is one seam away from writable. The
ore and refine lines carry no kind rows and are evidence without
vocabulary, finding 200's posture.

**Finding 288 — witnesses and pins.** Six positives: Prologue to
Phyresis' first line (the poison kind at the family's commonest
recipient), Screeching Scorchbeast's trigger (rad, a count above one,
the symmetric player domain), Meren of Clan Nel Toth's first line
(experience, the `You` recipient), Final Act's counter mode (the removal
verb's kind-blind cell), Leeches' first sentence (the same verb with a
kind named; its second reads the removed COUNT back as an anaphor and is
elided), and Kratos, Stoic Father's second line (the count read at its
player spelling). Six pins: the two scope refusals at the two verbs, the
named-kind cell of the removal verb, the read's holder agreement, the
event's kind, and the `At` header. THREE of the six share one inferred
obligation — `counterScope plusOnePlusOne = Player` — refused at three
different rows, which is the scope table doing its job at every consumer
rather than three tables agreeing by hand.

**Finding 289 — a binder-unification observation, recorded and not
fixed.** `CounterKind` is a live name in TWO independent module families:
this one's closed catalog, and the older `Semantics` module's open
`MkCounterKind Scope String (List (Ability Base))`, whose own scope field
is the same axis this chapter minted as `counterScope` and whose read is
also called `CountersOn`. The two never meet — `Experimental` imports
nothing from `Semantics` and says so in its header — so this is a name
straddle across two vocabularies rather than a conflict, and
`Macros.levelCounter` resolves to the OLD module's `Macros`, not
`Experimental/Macros`. Recorded for the binder-unification work; nothing
changed this round.

Ledger updates this chapter:

- The counter cluster's two queue entries (**closed and replaced**): the
  PLAYER holder and its two verbs, the count read, and the last-removal
  trigger all land here. What survives is one narrowed entry, the
  kind-blind quantifiers and the move-a-counter transfer (82 / 26 / 36
  lines), which want a quantity over kinds.
- **Energy defers wholesale** to the mana and payment round, acquisition
  and spending alike (79 lines, all symbol-notated), and **ticket goes
  with it** for the same reason (finding 284, 2 lines).
- Unwitnessed but landed, each with its blocker named: the count read's
  OBJECT spelling (finding 286) and the whole last-removal event (finding
  287, the intervening-if seam).
- Near-miss inventory: a combat-role player predicate ("defending
  player", 2 lines); the phrasal `Compare` standard, which now blocks a
  second family; the recipient-before-amount linearization; flanking and
  protection-from; the intervening-if seam, which this round promotes
  from a queued nicety to the single thing standing between the corpus
  and a whole event row's bench.

## Chapter forty-one — the intervening "if"

Finding 76 left the trigger layer a slot and a contract: `Triggered`'s
`{default Nothing intervening : Maybe (Condition (eventAfter ev))}`,
carrier-blind, reusing `Condition` verbatim on [CR#603.4]'s authority.
For thirteen chapters nothing was put in it. This chapter POPULATES it —
two witnesses, no new machinery of any kind — measures the one gap the
slot's design left open, and declines to build what the measurement says
is a different family. Bench: 237 positives / 395 pins.

**Finding 290 — the slot needed nothing, which is the result.** Both
witnesses are ordinary `Condition` terms dropped into a default argument:
Ornery Dilophosaur's control check is `Exists` over an ordinary
description, and Incisor Glider's threshold is `CompareAmt` over an
ordinary amount. No row was added, no gate was written, no table gained a
cell, and `Experimental.idr`'s only change this round is a comment. That
is finding 76's carrier-blind contract being paid off exactly as
designed, and it is worth recording as such: a slot that costs a comment
to populate thirteen chapters after it was declared is evidence the cut
was in the right place.

**Finding 291 — the marking word is "if" and there is no second cell,
and the rule says so before the corpus does.** The round asked whether
the slot should gain a `CondMarking`-style If/While pair. It should not,
on two independent grounds. [CR#603.4] restricts itself in its own text:
the rule applies to "an 'if' that immediately follows a trigger
condition", and the parenthetical adds that "the word 'if' has only its
normal English meaning anywhere else in the text of a card". A "while"
clause is outside the rule, so marking one as an intervening condition
would claim the double-check for a word the rule does not reach. And the
corpus makes it a different construction rather than a different word:
sixty header-internal "while" lines, NONE of them comma-marked, the
clause attaching to the EVENT ("Whenever this creature attacks while
saddled") where the intervening condition sits between two commas after
it. At least twelve condition shapes across those sixty — saddled (about
twenty-five), monarch, graveyard presence, counter presence, exile
presence, control, life totals — and three of them name an action IN
PROGRESS rather than a state at all: "while you're activating a craft
ability", "while casting a spell with emerge", "while scrying". A
marking word on a state condition cannot express those, which settles it.
The family is measured, ledgered, and deliberately not designed here.

**Finding 292 — the zone check works, and the seed logic behaves exactly
right at the one place it could have gone wrong.** The question was
whether `Matches`' `ZoneFits` gate would refuse the very zone being
checked — a real risk, since the gate exists to stop a phrase being asked
about a zone it contradicts. It does not, and the reason is a distinction
the vocabulary already had: the BARE self-word states no zone of its own,
so `Matches This (InZone exile)` passes by silence and reads "this card
is exiled". What the gate does refuse is the SORTED self-word, which
places its referent on the battlefield by writing a type ([CR#109.2]) and
therefore cannot be asked whether it is in exile
(`badExileCheckOnSortedSelf`, the round's one pin, refusing
`ZoneFits (Just Battlefield) (Just Exile)`). That is the seed discipline
drawing exactly the line it should: a phrase that places its referent may
not be interrogated about a different placement, and a phrase that places
nothing may.

**Finding 293 — Veiling Oddity is now blocked on two SURFACE facts and
nothing else, which is the sharpest thing this round measured.** Finding
287 called it "one seam away from writable". The seam is crossed and the
card still does not land, but every remaining obstacle is a spelling
fact rather than a semantic gap, and each was verified by elaboration
rather than argued: its BODY elaborates ("creatures can't be blocked this
turn" is `Cant` over the universal determiner under a this-turn span —
the bare-plural residue does not reach it); its ZONE CHECK elaborates
(finding 292); its intervening SLOT elaborates (finding 290). What is
left is the marking word — the card writes "while it's exiled" where the
slot spells "if" (finding 291) — and the PRONOUN, the card writing "it"
where the self announces no mention for `It` to read, which is chapter
thirty's split hitting a third construction after the becomes-blocked
bodies and the counter reads. So the last-removal event's bench stays
open, and the five lines now divide cleanly: two are inside granted
QUOTED abilities (suspend's own, Uvilda's refine pair) and wait on
quotation; one is the ore-counter Aura, which carries no intervening
condition at all and waits on its missing kind row and "enchanted land";
and the two free-standing time lines wait on the "while" family, one of
them additionally on flanking, protection-from and the token catalog
(Riftmarked Knight). Veiling Oddity is the reachable one.

**Finding 294 — a premise correction: the suspended family is not the
last-removal family.** "If this card is suspended" reads as though it
belonged with the last-removal lines and does not. [CR#702.62b] defines
the state — "a card is 'suspended' if it's in the exile zone, has
suspend, and has a time counter on it" — a conjunction of a zone, a
keyword and a COUNTER PRESENCE, and the four trigger lines that check it
are a recurring family watching ordinary events ("Whenever an opponent
casts a spell, if this card is suspended, remove a time counter from
it"), not a last-removal one. A fifth line writes the same check as an
activation restriction ("Activate only if this card is suspended") and is
a third construction again. They route to the kind-blind counter
quantifier entry, because the counter-presence conjunct is precisely the
"with a counter on it" read that entry carries (26 lines); the zone and
keyword conjuncts are already writable. Recorded so the mis-grouping is
not re-propagated. Correspondingly: all five last-removal lines condition
on the card being EXILED, and the ore-counter Aura line carries no
intervening condition whatsoever.

**Finding 295 — chapter forty's counter read composes at its first
consumer with no adaptation.** Incisor Glider's threshold is
`CompareAmt (CountersOn Poison (a Opponent)) OrGreater (Lit 3)`: the new
`Amount` row feeds the existing comparison's left side, its
`writtenCount` answer is never consulted there, and the bound stays a
numeral so `writtenBound`'s refusal of phrasal standards is untouched.
Two rounds' rows meeting for the first time and needing nothing is the
same evidence `Conditionally`-over-the-cap gave in chapter thirty-nine,
and it is the cheapest kind a workbench gets.

**Finding 296 — two surfaces, one query, for the round that comes next.**
The event-history lookback is written in two positions and the axis round
should build one thing read twice, not two families. The CONDITION
position is the 223 lookback headers, 176 of them a past-tense verb ("if
you attacked this turn", "if a creature died this turn"), and it lands in
the very slot this chapter populated — which is why the slot's landing
does not shorten that round by much and does define its shape: whatever
the query is, it is a `Condition` row. The PREDICATE position is finding
271's 8 past-relative lines plus finding 196's 4 delayed end-of-combat
forms, a postnominal relative over the same happening ("each creature
that blocked this turn"). Both ask a question of the shared `EventName`
vocabulary and differ only in where the answer is read. The two queue
entries that used to carry these separately are merged accordingly.

**Finding 297 — witnesses, the one pin, and one cell with none.** Two
positives, both whole cards once their keyword lines come off: Ornery
Dilophosaur (Deathtouch elided) and Incisor Glider (Flying elided, and
the "Corrupted —" ability word with it, an ability word having "no
special rules meaning" [CR#207.2c] — the same standing this grammar gives
reminder text). One pin, `badExileCheckOnSortedSelf`, and it is not the
slot's: the slot reuses `Condition` verbatim, so every gate that could
refuse inside it is already pinned generically at the condition layer and
there was nothing new there to refuse. Saying so is the honest report;
the pin that did land is about the zone check's NOUN, which is finding
292's line. One cell goes unwitnessed: the negated control check ("if you
don't control …"), whose six corpus lines each want something else
entirely — a predefined Food or Pest token, a quoted granted ability, a
named-card read, or a named token — so `NotCond` in the slot is recorded
without a bench positive.

Ledger updates this chapter:

- The intervening-if seam (**closed**): the slot is populated and needed
  no machinery. Its queue entry merges into the event-history lookback.
- The combat lookback and the intervening-if seam (**merged**) into ONE
  queue entry, "The event-history lookback", carrying finding 271's
  argument, the 223/176 header measurement, the 8+4 predicate-position
  lines, and finding 296's two-surfaces-one-query framing. Bill Ferny's
  residue rides with it.
- The header-internal "while" clause (**new queue entry**): 60 lines,
  twelve-plus shapes, three of them action-in-progress; not a marking
  variant ([CR#603.4] does not reach it); named payoff is Veiling Oddity.
- The suspended-state family (**re-routed**, finding 294): four recurring
  triggers and one activation restriction, to the kind-blind counter
  quantifier entry via [CR#702.62b]'s counter-presence conjunct.
- Near-miss inventory: the self-pronoun, now blocking a third
  construction and worth its own line the next time it costs a witness.

## Chapter forty-two — the event-history lookback

Finding 271 named the axis and declined to build it; finding 296 said it
was one query read in two positions and merged its queue entries. This
chapter builds the query and its CONDITION reader: `Lookback`, two new
`EventName` rows, `Happened`, and `EventCount`. The predicate-position
reader is deliberately not here, and the round's own recon corrected the
figure that made it look small. Bench: 241 positives / 398 pins.

**Finding 298 — history is not description, and the rule says so in the
words the vocabulary needed.** [CR#608.2i]: an effect may "look back in
time and require information about previous game states and actions
rather than considering the current game state", and the objects it names
"don't need to be currently in the zone they were in … nor do they need
to currently meet the criteria described in the action, as long as they
did so at the specified time". That is the whole warrant for a separate
row: a creature that died this turn is in a graveyard when the condition
is checked, so `Exists` over a description cannot ask the question and no
amount of predicate machinery would have made it. The WINDOW is likewise
its own vocabulary and not a `Duration` row. Core states the rule and
this chapter takes it as the design's anchor — `Timing` is a permission
window, `Lookback` a history window, "never conflating the duration and
history readings of 'this turn'" (`deckmaste_core/src/temporal.rs`) — and
the Idris side had its own evidence in `Duration`'s forward-only
commitment. The separation then got ENFORCED rather than merely written
down: `Duration` already owns the constructors `ThisTurn` and `LastTurn`,
so `Lookback` lives in its own namespace beside `Counter`'s, one English
word and two grammatical objects told apart by where they live.

**Finding 299 — the bridge to the event vocabulary is the NAME plus
tables, not a mirror of core's filter.** Core matches an open
`EventFilter` — a structured pattern with its own subject, complement and
qualifier slots — and this vocabulary could have mirrored it. It does not,
and the reason is the same one that has governed every event round here:
`EventName` is the shared catalog the four trigger-side readers already
key on, and a fifth reader that keyed on something else would have made
the catalog stop being shared. So `Happened` names an `EventName` and
describes its subject beside it, and everything a filter would have
carried in slots is carried in tables instead — which is also what makes
the gaps VISIBLE. A mirrored filter would have accepted "if this creature
dealt combat damage to an opponent this turn" by giving the recipient a
slot, and the fact that the complement is unwritable would have gone
unrecorded. Here it is a refused cell with a reason (finding 301).

**Finding 300 — the subject is a NOUN, and the round's predicate premise
failed on measurement.** The row was drafted with a `Predicate` subject on
the reasoning that a condition announces nothing. The corpus writes four
subject shapes in this position and only ONE of them is a predicate: the
indefinite description ("if a creature died this turn"). The other three
are nouns no predicate can spell — "you", which carries the raid mass and
the cast, draw and life families, well over a hundred lines; the SELF,
fifty-eight lines ("if this creature attacked or blocked this turn", "if
Wolverine dealt damage to another creature this turn"); and the
anaphoric player. So the row takes a noun. Nothing is lost at the binding
layer, and that is the point rather than a rescue: a condition
contributes nothing either way (`condDelta` answers the empty list, as
`Exists` does over its own predicate's mentions), so the choice between
noun and predicate was never a semantic one and English settles it. One
row, not two; the fork the round warned against is not taken.

**Finding 301 — the attestation table needs TWO axes, and one family
forced it.** Every event name in this position takes a subject of one sort
— except the attack declaration, which takes both: "if you attacked this
turn" is thirty-eight lines with a player subject, "if this creature
attacked or blocked this turn" six with an object one. A single-valued
`EventName -> Kind` table would have had to refuse one of them, so
`lookbackSubjectOk` is over the pair and `Happened` is indexed at the kind
it admits — `replUseOk`'s and `triggerWordOk`'s shape at a third site.
Full rows over eighteen names and the two phrasal kinds. The Trues, each
measured in the condition position and nowhere else: death, departure
(12), entry, damage taken (6), block declaration (4) and attack
declaration (6) on the object side; spell cast (21), card drawn, life
gained (37), life lost (18) and attack declaration (38) on the player
side. The zeroes are queried and real — no destruction, status change,
face turning, phasing, becomes-blocked, last-counter removal or turn-part
beginning is written as a history read, and "if an upkeep began this
turn" is not English (`badLookbackPartBeginning`).
TWO cells are refused for a reason sharper than a zero, and keeping them
apart is what the name-plus-tables bridge buys. Combat damage is written
twice here and both lines carry a RECIPIENT complement ("dealt combat
damage to an opponent"); the player side of the attack declaration writes
a complement on every line that is not bare ("if you attacked with a Hero
this turn"). A one-noun query cannot finish either, so the cells stay
False and the COMPLEMENT is ledgered as the gap it is.

**Finding 302 — two event names with no producer, and the vocabulary
already had the word for that.** `LifeGain` and `LifeLoss` are minted
because the lookback reads `EventName` directly and never goes through
`GameEvent`, so "if you gained life this turn" (18 lines) and "if an
opponent lost life last turn" are writable while "Whenever you gain life"
is not — no `GameEvent` row makes either name. The four trigger-side
tables still owe answers, and `eventUse`'s `EventUnclaimed` is exactly the
cell: "real oracle English and no construction HERE writes it". That was
already a class in the enum, unchanged since chapter twenty-eight, and
needing it for a name rather than a construction is the first time it has
been read that way. `triggerWordOk` is answered from the English the
corpus writes rather than left blank, because a full-row table has to say
something and the corpus's answer is the one that stays true when the
producer lands. The names are `LifeGain`/`LifeLoss` and not
`LifeGained`/`LifeLost` because the deed telescope already owns those two
as OUTCOME words — a collision the compiler caught, and the
nominalisation is `Death`'s and `Departure`'s anyway.

**Finding 303 — three windows land, two are refused on membership, and
one the round's brief did not list is the corpus's.** `ThisTurn` is the
mass (296 condition-position lines) and `ThisCombat` seventeen (Kytheon,
the velocity Vehicles, the pack-tactics family, Tolsimir). `LastTurn` was
not in the draft and has eleven: "if a player cast two or more spells last
turn", "if you lost life last turn", "if an opponent lost life last turn".
Core carries it too. Refused: `ThisGame` reads backward on twenty-four
lines and nearly all of them are one specialised count — "for each time
you've cast your commander from the command zone this game" and its
family, plus named-card counts and Ring temptations — none routing
through this query's events; and the `SinceYour` family is five lines
across three spellings, two syntactic positions and two possessors
(O-Kagachi writes "during their last turn"), too thin and too varied to
close a row over. Both ledgered, as `ThisGame`'s membership analysis is
the sort of thing a count alone would have got wrong.

**Finding 304 — the numeric route is a count, and the SUM is a different
row this chapter does not build.** "If you've cast two or more spells this
turn" is `CompareAmt (EventCount SpellCast You ThisTurn) OrGreater
(Lit 2)`: the amount row under the existing comparison, no comparison
vocabulary added, which is chapter forty's counter read arriving at the
same seam a second time. What `EventCount` cannot do is what core keeps a
second row for — "EventCount counts; EventSum sums" — and the corpus
divides exactly there: "if you gained life this turn" (18 lines) counts
gainings and is `Happened`'s, while "if you gained 3 or more life this
turn" (15) wants the LIFE and not the number of gainings. The sum is
ledgered rather than guessed at, and the split is core's own.

**Finding 305 — the history read negates, on the auxiliary.** Nineteen
condition-position lines write didn't, hasn't or haven't, eight of them
"if you haven't cast a spell from your hand this turn", against ONE
writing the other surface ("if no creature died this turn"). So
`condNegatable` answers True for this row and the negation is spelled
where English puts it; the "no [description]" surface is a spelling
residue for the same meaning and is recorded, not built. The cell lands
with no bench positive, and the blocker is finding 301's: every "haven't
cast" line writes "a spell FROM YOUR HAND", a complement the one-noun
query cannot carry.

**Finding 306 — a correction: the predicate-position surface is four
times what finding 296 recorded.** That finding's "8 past-relative lines
plus 4 delayed end-of-combat forms" was BLOCK-FAMILY-SCOPED — it counted
what the blocking relation left over and nothing else. Measured across
the verb set, the postnominal relative is over a hundred lines: died
about thirty, entered about thirty-two, dealt-damage about twenty-three,
attacked about ten, blocked about twelve, plus the zone-change relative
the log already elides. The two-surfaces-one-query framing survives
unharmed and is if anything strengthened — the second surface is a real
family, not a residue — but the queue entry's size estimate was wrong by
a factor and is corrected there.

**Finding 307 — witnesses, pins, and three cells that landed without
one.** Four positives: Storm Fleet Spy (the raid mass, a player subject,
a whole card once its ability word comes off), Vashta Nerada (the morbid
shape, an indefinite object subject), Tippy-Toe (the life-change name at
its only reader), and Loan Shark (the numeric route through
`CompareAmt`). Three pins, all against `lookbackSubjectOk`: a player
subject on the death read, an object subject on the cast read, and the
turn-part beginning. What landed WITHOUT a witness, each with a named
blocker: `LastTurn`, whose eleven lines all write "At the beginning of
each upkeep", a header cell `partUse` has held at `PartUnclaimed` since
chapter seventeen; the negation (finding 305); and Brazen Cannonade,
which finding 196 named as this axis's payoff and which now has ONE
blocker instead of two — its "if you attacked this turn" is writable as
of this chapter and its postcombat main phase still is not.

Ledger updates this chapter:

- The event-history lookback (**half closed**): the query and its
  condition reader land. The queue entry is replaced by the
  predicate-position reader alone, with finding 306's corrected figure.
- **Recorded gaps, none of them rows here.** The event COMPLEMENT is the
  round's largest and now blocks four cells (combat damage's recipient,
  the attack-with phrase, the cast-from-hand phrase, and with them the
  negation's whole bench). CAUSE-NARROWED verbs — milled, discarded,
  sacrificed — are narrowings of one zone change by the verb tag, which
  is how core spells them (`VerbName` on `ZoneChange`, no `EventName`
  rows), so they are not names this catalog gains; the participle read
  finding 196 wanted for "milled" is the same fact at the other surface.
  "Countered" is one line. `EventSum` is finding 304's. `ThisGame` and
  the `SinceYour` window are finding 303's.
- Near-miss inventory: the each-upkeep header cell, which now costs a
  window its only witness; the from-your-hand cast qualifier; the
  postcombat main phase, still Brazen Cannonade's last blocker.

## Chapter forty-three — the history read in the description

Finding 296 claimed the event-history lookback was ONE query read in two
positions, and chapter forty-two built the query and the condition
reader on that claim. This chapter builds the second reader and so tests
it. `HappenedTo` is a `Predicate`, takes no subject argument, and is
gated by chapter forty-two's own table. The claim holds, at the cost of
exactly one re-measured cell. Bench: 245 positives / 400 pins.

**Finding 308 — the second reader adds a row and a spelling, and nothing
else.** `HappenedTo` names an `EventName` and a `Lookback` and stops
there: the head noun IS the subject, so the row has no subject slot, its
kind index is the subject sort, and `lookbackSubjectOk` gates it
unchanged. No new attestation table, no second window vocabulary, no
per-surface event catalog. That is what finding 296's claim predicted and
it is the strongest form the prediction could have taken — the alternative
this vocabulary rejected in chapter forty-two, mirroring core's open
`EventFilter` at each site, would have made the two surfaces two families
that merely resembled each other. The shape is `BlockerOf`'s and
`BlockedBy`'s: a postnominal participial relative that heads nothing and
modifies the noun in front of it.

**Finding 309 — one cell moved, and moving it is a rule about shared
tables rather than a correction.** `lookbackSubjectOk DamageTaken Player`
was False. Chapter forty-two measured it in the condition position, which
was the only reader that existed, and found one line in a window this
vocabulary does not carry. The predicate position writes the same event
over a player EIGHT times — "for each opponent who was dealt damage this
turn", "each player who was dealt combat damage this turn", "target
player who was dealt combat damage by [source]" — so the cell is True.
The principle is worth stating because it will recur every time a shared
table gains a consumer: a shared table's cells are a property of the
QUERY, not of a reader, so the measurement behind them is a UNION over
readers and a new reader can only ever add Trues, never subtract. The
round that adds a reader owes the re-measurement, and this is the only
cell that owed one — the other seventeen names' cells read the same at
both surfaces, which is itself the evidence that the table is genuinely
one table.

**Finding 310 — the seeding is a SILENCE, and one corpus line proves it
could not have been anything else.** Every other participial modifier in
this vocabulary seeds a zone: `Attacking` and `Blocking` seed the
battlefield, `ExiledWith` seeds exile, and the block relation seeds the
battlefield at both ends. The history read seeds NOTHING, and [CR#608.2i]
is the reason in its own words — the objects a history read names "don't
need to be currently in the zone they were in at the time of that
previous game state or action". Continue? writes the proof: "creature
cards in your graveyard that were put there from the battlefield this
turn", a phrase whose own zone clause says GRAVEYARD about an event that
happened on the battlefield. Under a battlefield seed `zonesOk` would
have refused it. The line is the seeding's proof and NOT a witness — its
surface is the zone-change wording rather than a past verb this row
spells — so it stays the near-miss it was. `seedType` is silent for the
ordinary reason: the head noun carries the type and this modifier
presupposes none.

**Finding 311 — the relativizer agrees with the head's sort, which is
finding 275's lesson for the third time.** "that blocked this turn" over
an object head, "who was dealt damage this turn" over a player one, one
to one across the whole surface. No axis is minted: the word is a
function of the sort the row is already indexed at, so the spelling owns
it exactly as `CapDomain` owns its step possessive and `DoesntUntap` owns
its. Three rounds have now paid for the same diagnostic, and it is worth
saying plainly: when a candidate axis varies perfectly with something the
phrase already writes, tabulate the pair before minting anything.

**Finding 312 — the two surfaces agree about where the negation goes.**
Chapter forty-two found the condition surface negating on the auxiliary
(nineteen didn't/hasn't/haven't lines against one "no creature died") and
answered `condNegatable` True. The predicate surface does the same: "all
untapped creatures that didn't attack this turn", "each creature that
didn't enter this turn" — six lines, all auxiliary-negated, none writing
a "non-" prefix or a bare quantifier. So `negatable` is True here too, and
the agreement is a result rather than an inheritance: the two tables were
measured separately and came out the same, which is what a single query
read twice should look like. (The "who didn't discard a creature card
this way" family is NOT among them — "this way" is cause-anaphora, finding
307's explicit exclusion, and stays `TheVerbed`'s territory.)

**Finding 313 — the COMPLEMENT is deferred whole, and it is now the
family's largest single gap.** The row lands bare. What it cannot write:
the by-source damage family (~20+ lines, "that was dealt damage by a
source you control"), the block relation's by-complement (~9 lines, and
the block family writes one almost without exception — the bare form is
two lines, one of which is this chapter's witness), and ALL FOUR of
finding 196's delayed end-of-combat forms. They want a third participant
slot with a per-event complement-sort question, which is a round. The
four delayed forms carry an INDEPENDENT second blocker besides, recorded
plainly so the queue does not promise them cheaply: the "At this turn's
next end of combat" delayed shell, which no current machinery reaches at
all. The complement now blocks cells at both surfaces — chapter
forty-two's `CombatDamage` and attack-with cells and the whole negation
bench, this chapter's by-source and by-blocker families — which is why it
gets the queue entry rather than a ledger line.

**Finding 314 — witnesses, pins, and the families that stay out.** Four
positives, three of them whole cards: Sizzling Barrage (the block cell,
and one of only two bare lines in a complement-bearing family), Witch's
Mist (the damage cell, [CR#608.2i] doing visible work — the damage was
dealt when the creature was elsewhere and otherwise), Force of Despair's
second line (the universal determiner where the others target), and
Furious Spinesplitter (the PLAYER head, its "who" relativizer, and the
for-each domain's first history consumer — `CountOf` took the read as an
ordinary modifier and needed nothing). Two pins, both against the shared
table and deliberately so: an object head asking a player's event
(`badHappenedToObjectCast`) and a player head asking an object's
(`badHappenedToPlayerDied`), the second sharing its inferred obligation
with chapter forty-two's `badLookbackPlayerDied` — the same table
refusing at its two readers, which is the chapter's thesis stated as a
pin. Excluded and not re-litigated: the "this way" cause-anaphora family
(finding 307); the "attacked during their controller's last turn" line,
which is a static RESTRICTION and not a lookback at all; and the
`SinceYour` cousin among the entered lines, out of scope with its window
(finding 303).

Ledger updates this chapter:

- The event-history lookback (**closed as an axis**): query, condition
  reader and predicate reader all land. The queue entry narrows to the
  COMPLEMENT residue.
- `sameEventName` and `sameLookback` (**minted**): the closed-vocabulary
  comparisons every other closed word in the file already carried, needed
  because both of this row's arguments are closed and `predEq` could
  therefore answer honestly instead of conservatively.
- Near-miss inventory: Continue?, now recorded as the seeding's proof
  rather than a pending witness; the "At this turn's next end of combat"
  delayed shell; the by-source and by-blocker complements.

## Chapter forty-four — characteristic predicates

Core's `CharacteristicPredicate` has nine variants (`filter.rs`): Type,
Subtype, Supertype, ColorIs, Named, Stat, Multicolored, Colorless, and
the open `Has(KeywordRef)`. This vocabulary carried four of them and a
narrowed fifth. This chapter closes the gap: five rows land and a sixth
is minted that core does not carry, leaving exactly one variant
outstanding and that one by standing ruling. Two earlier chapters'
recorded gaps are backfilled in the process. Bench: 250 positives / 402
pins.

**Finding 315 — colour was UNADDRESSED, not deferred, and the
distinction matters for what the round is allowed to do.** Finding 77
ported `Chroma.Color` from core for TOKEN minting and said so in its own
terms — a rules-fixed set rather than a construction English might or
might not write, [CR#105.1]'s closed five — explicitly excluding the mana
channel and never reaching the DESCRIPTION reading. So no ruling ever
declined "target black creature"; the row simply had no consumer until a
round asked for one. That also means the taxonomy decision does not
reach here: types and subtypes are macro-declared atoms because the game
adds them, and colours are five because the rules say five. The row is
`ColorIs : Chroma.Color -> Predicate bs Object`, stacking rather than
clashing with itself ([CR#105.2] — an object "can be one or more of the
five colors"), presupposing neither zone nor type (a black card in a
graveyard and a red spell are both ordinary oracle), and negated by
PREFIX: "nonblack" 77 lines, nonwhite 15, nonblue 5, nonred 4, nongreen
9, against zero writing "not black". `IsToken`'s arrangement with
"nontoken", and the row is named `IsColorless`'s sibling for the same
reason `IsColorless` is named that at all — `Chroma.ColorOrColorless`
already owns the bare word in the mana channel, which is `Lookback`'s
one-word-two-objects situation a second time.

**Finding 316 — the three colour-COUNT words are not colours, and the
rules say so twice.** `IsColorless` is not a sixth value of `ColorIs`:
[CR#105.2c] says "a colorless object has no color" and [CR#105.4] adds
"'Multicolored' is not a color. Neither is 'colorless.'" Core splits them
the same way. `Multicolored` is [CR#105.2b]'s "two or more". None of the
three negates — "noncolorless", "nonmulticolored" and "nonmonocolored"
are zero lines apiece (`badNonMulticolored`) — where the five colour
words and the supertypes are negated in quantity, which is a clean split
between the words that describe a colour and the words that describe a
COUNT of them.

**Finding 317 — `Monocolored` is minted against core, on fifteen lines
and an argument.** Core carries `Multicolored` and `Colorless` and no
monocolored variant, and it can afford to: a filter that counts colours
reaches the concept without a word for it. This vocabulary has no
colour-count term, so omitting the row would have left fifteen attested
lines ([CR#105.2a], "exactly one of the five colors") unwritable to buy a
parity that costs the corpus. English writes two words where a count
would have written one bound, and the two words are two rows. Recorded as
a divergence rather than slipped in, because the next round to compare
the two vocabularies should find the reason rather than the difference.

**Finding 318 — the supertype's third reader arrives, and the vocabulary
moves to meet it.** Chapter thirty minted `Supertype` for the printed
card's own field and ledgered it as "a third list neither reader
witnesses" — the token's defined characteristics and the type-addition
clause write no supertype between them. The DESCRIPTION is the third
reader: legendary 181 lines, basic 383, snow 67, with "nonlegendary" 43
and "nonbasic" 68 on the negated side. The row is `HasSupertype`, and
placing it required moving `data Supertype` and `sameSupertype` out of
the card-frame section and up beside the other closed catalogs, which is
where every vocabulary with more than one reader in this file already
lives (`Chroma.Color`, `CounterKind`, `Keyword`, `Lookback`). The
card-frame's own list functions stay where they are. World and Ongoing
remain unminted, chapter thirty's note unchanged.

**Finding 319 — `Named`'s string is a payload, and the one place it is
looked at is not a gate.** The repo's standing validator ruling is that no
gate may match on a name string. This row honours it exactly: none of the
nine tables branches on the string, and the single site that reads one is
`predEq`, which asks whether TWO phrases name the same card and never
what either of them names. That distinction is [CR#201.2a]'s own — "two
or more objects have the same name if they have at least one name in
common" — so name EQUALITY is rules-real where name MATCHING would have
been a validator smuggled into the grammar. The practical consequence is
that the contradiction scan can see "creature named X that isn't named X"
without any auto-implicit ever inspecting a character. Negation is
attested at four lines and answers True.

**Finding 320 — one contradiction is added, and its narrowness is the
point.** `colorClashOf` mirrors `statusClashOf`: a member saying
`IsColorless` and a member naming a colour describe nothing together,
[CR#105.2c] giving a colorless object "no color" in as many words
(`badColorlessWhite`). What it deliberately does NOT do is clash two
COLOURS with each other — verified by probe, not asserted: a term
conjoining black and red elaborates, because [CR#105.2] lets an object be
"one or more of the five" and a gold card is both. (The probe is evidence
and not a bench positive: no corpus line poses a two-colour conjunction
in a description, the corpus writing disjunctions there instead.) The
count words are left out of the scan too — whether "monocolored
multicolored" contradicts is arithmetic over a set the phrase does not
write, and no line poses it.

**Finding 321 — two backfills, and what they say about recorded gaps.**
Winter Moon lands whole ("Players can't untap more than one nonbasic land
during their untap steps"), closing the fifth set word chapter
thirty-nine recorded as corpus evidence without a bench positive; the
blocker was exactly the supertype predicate that chapter named. Cradle to
Grave lands whole ("Destroy target nonblack creature that entered this
turn"), which chapter forty-three tried to witness and could not — and it
is the round's composition test besides, the colour negation and the
history read sitting as two ordinary modifiers under one head with
neither knowing about the other. Both old chapters stay as written; the
backfill is recorded here, which is the arrangement in-place annotation
was invented to avoid needing. Five rows landed and two of them were
owed to earlier rounds by name — evidence that the "recorded, not
witnessed" posture pays back when the missing vocabulary is stated
precisely enough to be looked up.

**Finding 322 — what is left of core's enum, item by item.** After this
round the subset gap is ONE variant. `Has(KeywordRef)` stays out by
standing boundary: core's is an open bare-ident reference matching by
name, and this vocabulary's `HasKeyword` is closed at seven pending the
verifier transition, which is a deliberate line and not a gap this round
may cross. `Stat` is present as `Compare` over `Characteristic`, whose
three rows (power, toughness, mana value) are the three the description
surface writes — "with mana value N or less" alone is 466 lines — and the
candidates for a fourth are measured at zero here: "with loyalty N or
less" 0 lines, "with defense N or less" 0, so the planeswalker and battle
stats are read by their own constructions and not by this one. The colour
INDICATOR is 0 lines: it is a frame element that DEFINES colour
([CR#105.2]) and no card describes an object by having one, so it is not
a predicate at all. Nothing else in the enum is outstanding.

Ledger updates this chapter:

- Characteristic predicates (**closed**): the vocabulary line's five
  missing rows land plus one core does not carry. The queue entry is
  deleted; `Has(KeywordRef)` is not part of it, being the keyword
  boundary's business.
- Chapter thirty-nine's Winter Moon gap and chapter forty-three's Cradle
  to Grave gap (**backfilled**, finding 321); both chapters stand as
  written.
- `Supertype` (**relocated**), card-frame section to the closed-catalog
  region, on the third-reader rule the file already follows.
- Near-miss inventory: the two-colour conjunction, which elaborates and
  which no corpus line writes; the definite-determiner residue, which is
  finding 272's actual blocker on "the blocking creature" and NOT a
  colour gap — recorded because the round's own framing had it wrong.

## Chapter forty-five — the possessor axis

The queue called this the turn-part catalog. Measurement says otherwise:
three parts were missing and the corpus wanted them, but what had been
blocking headers since chapter twenty-eight was the POSSESSOR — the
`PartUnclaimed` cells that named a quantifier the vocabulary had no word
for. This chapter mints the words, adds the three parts, and gives the
activation window its own reader of the same grid. Bench: 254 positives /
405 pins.

**Finding 323 — the possessor is its OWN vocabulary, and `Whose`'s
docstring is what settles it.** The obvious move was four more rows on
`Whose`. `Whose`'s own comment forbids it: that type is the DURATION
endpoint's, "closed and pronominal", possessing an endpoint "with a
possessive DETERMINER and nothing else". The header site is not
pronominal — it writes quantifiers, "each player's upkeep" 76 headers,
"each opponent's upkeep" 33, "each of your postcombat main phases" 7 —
and the duration endpoint writes none of them, every quantifier spelling
measuring zero at every endpoint. Two readers, two measurements, two
vocabularies: `statusWordOk` beside `statusEventOk`, `Lookback` beside
`Duration`. The decision also had a price tag, and paying attention to it
was the other half of the argument: `spanUse` is full rows over (boundary
x part x possession), so four shared possessors would have added NINETY
cells to it, every one `Unattested`, to a table whose whole value is that
each of its cells is a measurement. With the vocabularies separate it
grew by eighteen — the three new parts against the two possessors it
already had — and every one of those eighteen is a real zero.
`Owner` is a superset of core's `WhoseTurn` (`Your`, `EachPlayers`,
`AnOpponents`), adding the opponent QUANTIFIER and the plural-iterated
self, both of which the corpus writes and core's three do not cover.
Finding 254's derived possessor does not apply and the reason is
structural: there the possessive agreed with a restricted NOUN the clause
already wrote, and a trigger header has no such noun to read off.

**Finding 324 — the bare "each upkeep" is the SAME cell as "each
player's upkeep", and the rules say so.** [CR#500.1] gives a turn five
phases, "each of these phases takes place every turn", and the beginning
phase's upkeep step is the active player's; so every upkeep in the game
is some player's, and "at the beginning of each upkeep" and "at the
beginning of each player's upkeep" name the same moments. One cell, two
spellings — 36 bare against 76 possessed at the upkeep, 79 against 17 at
the end step, 21 against 5 at combat — recorded in the vocabulary's own
spelling comment rather than split into a row that would have had to
claim a difference the rules deny.
That decision then answered a question nobody had asked: what the
POSSESSOR-LESS cell is for. It is the DELAYED clause's. "At the beginning
of the next end step" writes no possessor because it names one occurrence
rather than a class, and it is a different construction from the header's
bare "each end step" — which is why `EndStep Nothing` stays open while
the header's bare spelling moves to `EachPlayers`. The upkeep has no
delayed reading, so `Upkeep Nothing` is now `PartUnattested` where it was
`PartUnclaimed`: the vocabulary has the words, and what is left is a real
silence rather than a missing one (`badTriggerAtEachUpkeep` refuses
either way, a pin unaffected by which silence it names).

**Finding 325 — the second main phase has two names and the possessor
chooses between them.** [CR#505.1] gives the phase both — "the first main
phase (also known as the precombat main phase)" — and at the trigger
header the split is perfect: `Yours` writes "your second main phase" (5
headers) and `EachYours` writes "each of your postcombat main phases"
(7), with both cross cells at zero. Finding 275's diagnostic a fourth
time, so one part row carries two spellings and no axis is minted. The
SCOPE is stated because it is not the language's fact: "your postcombat
main phase" occurs eight times elsewhere in the corpus, so the perfection
belongs to this reader and a future reader must re-measure rather than
inherit — the union doctrine's other edge.

**Finding 326 — three parts arrive, and the untap step still does not.**
`FirstMain` (52 headers), `PostcombatMain` (12 across its two spellings)
and `DrawStep` (18) are minted on the catalog's standing philosophy: a
part is minted when a construction needs it. What is NOT minted is any
cell for the untap step, which measures ZERO headers at every possessor —
the step whose turn-based action is the untap itself ([CR#502.3]) and
which the corpus addresses only through the doesn't-untap family chapters
thirty-four to thirty-nine built. The cleanup step is likewise absent.
Both silences are now stated across a nine-part grid rather than a
six-part one, which is the difference between a gap and an answer.

**Finding 327 — the activation window is a SECOND reader of one grid,
and the two disagree in both directions.** `Timing` gains `DuringPart`,
core's `DuringTurn`/`DuringStep` under one row because this vocabulary's
`TurnPart` already carries the turn itself as a part — core splits them
only because its `PhaseStep` does not include the whole turn, which is a
fact about core's type and not about English. It is gated by its own
table, `windowOk`, and the disagreements are the reason that table
exists: the window opens `AnOpponents`, which no header writes ("Activate
only during an opponent's turn" is three lines against zero headers,
`badTriggerAtAnOpponentsUpkeep`), and it opens the bare TURN, which no
header names at all (68 lines of "Activate only during your turn" against
a `partUse` row that is `PartUnattested` throughout); while the header
opens six parts the window never names (`badWindowDuringYourEndStep`).
One grid, two readers, two tables — `eventUse` and `eventSpan`'s
arrangement at a new axis. It also closes the file's own named
unspellable: "Activate only during any upkeep step" was recorded as
writing "a possessor `Whose` has no word for", and `EachPlayers` is that
word, the window spelling the quantifier "any" where the header writes
"each" or drops it.

**Finding 328 — Brazen Cannonade, three chapters in the making, lands to
its last sentence.** Finding 196 named it as the end-of-combat round's
blocked card and listed two obstacles: a postcombat main phase and an "if
you attacked this turn" lookback. Chapter forty-two supplied the second
and reported the card down to one blocker; this chapter supplies the
first. What lands is the whole of its second line up to its final
sentence — the header with its `EachYours` possessor, the raid lookback
in the intervening slot, and the exile. The residue is exact and is an
EXISTING PIN rather than a new gap: "Until end of combat on your next
turn, you may play that card" is a play permission under an
end-of-combat span, which `badPermissionUntilEndOfCombat` refuses on
chapter seventeen's measurement. The card is now blocked by a cell this
grammar has deliberately closed, which is a different and better position
than being blocked by vocabulary it lacks.

**Finding 329 — a re-baseline, and a second backfill.** Chapter
forty-two recorded the `LastTurn` window's family as "eleven lines, all
'each upkeep'". It does not reproduce: 10 supported, 12 all-scope, 9 of
them bare-each. The figure is re-baselined here and the log stays
historical. The window also lands its first bench positive — Paladin of
Atonement, whose header is exactly the bare-each cell this round minted —
so a window that landed on corpus evidence alone three chapters ago is
now witnessed, which is the second time in two rounds that stating a gap
precisely enough turned out to be the whole cost of closing it.

**Finding 330 — what stays out, and one cell that lands unwitnessed.**
The NOMINAL possessor tier is not this round's: "enchanted player's
upkeep", "its controller's end step" and the Curse and Aura mass write a
possessive over a NOUN rather than a quantifier word, which is a
different axis (`ThatPlayers`' cells are left exactly as chapter
seventeen set them). The activation window's BEFORE and COMPOSITE forms
are deferred with their counts: "during your turn, before attackers are
declared" is 19 lines and the bare "before attackers are declared" 24,
and both want a boundary-RELATIVE marker rather than a part-during, which
is a slot this row does not have. Witnesses: Paladin of Atonement (the
bare-each backfill), Brazen Cannonade (above), Four Knocks (the first
main phase, and the only one of its 52 headers whose body is not mana
production), Hammer of Bogardan (the window's first population). The
`EachOpponents` cell lands with 33 corpus lines and NO bench positive,
and the blocker is worth recording because it is the possessor axis's own
shape: every one of those lines continues "…, that player sacrifices…",
referring back to the possessor — and the possessor is a WORD, not a
phrase, so it announces no mention for "that player" to read. A
quantifier possessor cannot be an antecedent.

Ledger updates this chapter:

- The turn-part catalog (**closed**): three parts minted, the possessor
  axis given its own vocabulary, the activation window populated. The
  queue entry is replaced by the two residues below.
- **The nominal possessor tier** and **the window's before/composite
  forms** (19 + 24 lines) are the survivors and go to the queue as one
  narrowed line each.
- Chapter forty-two's `LastTurn` figure (**re-baselined**, finding 329);
  chapter forty-two's unwitnessed window (**backfilled**).
- Near-miss inventory: the quantifier possessor as an antecedent, which
  costs `EachOpponents` its bench; the boundary-relative marker; the
  cleanup step, measured at zero for the first time.

## Chapter forty-six — the requirement

`Cant` has spelled [CR#508.1c] and [CR#509.1b]'s restrictions since
chapter twenty-two. This chapter mints their other half — [CR#508.1d] and
[CR#509.1c]'s requirements — as `Must`, plus the untap step's permission
to decline. The round's largest decision is one it declined to make, and
the chapter states it as a ruling with its own trigger attached. Bench:
258 positives / 409 pins.

**Finding 331 — the deontic exists three times in this repo, and this
round deliberately does not converge them.** Core has
`Deontic{May,Cant,Must,Gate}` over a `DeonticAction` spine
(`deckmaste_core/src/deontic.rs`), whose actions carry `by`/`on`
predicates and count bounds. The OLD module has
`Constrain : Compulsion -> Deed b -> StaticEffect b` over an eight-kind
relation spine with `Require`/`Forbid` as its polarity and a `Priced`
sibling for gates (`Semantics.idr`). The workbench has narrow `Cant`, and
now narrow `Must` beside it — two rows on one `Deed x Role` grid, not one
row with a polarity argument.
That is a RULING and not an oversight, and the reason is what this
workbench is for: rows are minted where lines are measured (finding 85's
bar), and the old module is the thing being superseded by re-derivation
from the corpus rather than the thing being ported. Generalising to a
compulsion axis here would have re-derived `Constrain` from `Constrain`.
The three cells `Must` needs — Attack/Agent, Block/Agent, Block/Patient —
are already True in `deedType`, so the mirror costs one row and zero
table growth; a polarity argument would have cost a refactor of a landed
row for no new line.
THE CONVERGENCE TRIGGER IS NAMED. A GATE polarity landing with evidence
is what would justify refactoring both rows onto one compulsion axis, and
the evidence already exists and is measured here: "can't attack unless"
86 lines, "can't block unless" 14, 25 of them a payment. That is the next
deontic round's material, and when it lands these two rows should merge
with it rather than a third being added beside them.

**Finding 332 — "each combat" is CADENCE, not span, and [CR#508.1d]
carries the semantics itself.** The phrase looks like a duration and is
not: the rule states the re-check — a requirement is evaluated as each
declaration is made — so "each combat" is the construction's own words
for a rule the engine already has, and no `Duration` row is minted for
it. That left the round's one genuinely open shape: `absentOk
DeedRestriction = False`, so a durationless `Continuously` clause is
refused, and the standing lines ("This creature attacks each combat if
able", a whole card) have no duration adverbial at all. The resolution is
that they are not clauses. They are static ABILITY lines, and
`staticAsAbility DeedRestriction = True` admits them — which is the exact
arrangement `Cant` and `DoesntUntap` already have, the standing lock
being a `Static` and the timed restriction a `Continuously` with a span.
So no new `StaticKind`, and the requirement's real duration (standing,
this-turn) rides the envelope as the restriction's does.

**Finding 333 — "if able" is constant, so it is spelling.** Every genuine
requirement line carries it — 58 attack lines, 38 block lines, 30
must-be-blocked lines — and the bare form is written zero times, the two
bare-looking lines carrying it after all. A rider that is never absent is
not a slot, which is the same test that kept the possessive off
`DoesntUntap` (finding 254) and the step word off the cap (finding 275).

**Finding 334 — the patient is required in one cell and refused in three,
so the table decides it and not the writer.** Every one of the 38
Block/Agent requirement lines names what must be blocked ("blocks this
creature this turn if able"); no Attack/Agent line names a defender and
no Block/Patient line names a blocker. So the slot is a `Maybe` at the
type and `mustPatientOk` says which cells may fill it —
`BlockPartner`'s family from chapter thirty-eight with the cells decided
by measurement (`badMustAttackWithPatient` refuses a defender,
`badMustBlockNoPatient` refuses the bare block). This is the one place
the mirror of `Cant` is inexact, and it is inexact because English is:
the restriction "can't block" needs no patient and the requirement
"blocks X if able" cannot do without one.
GOAD would have been the exception and is out, and checking WHY moved it:
[CR#701.15b] makes goaded a DESIGNATION — "neither an ability nor part of
the permanent's copiable values" — not a keyword, so the leg routes to
the designations family beside the monarch and the initiative rather than
behind the keyword boundary this round first filed it under. Its ~27
reminder lines are not evidence for an Attack/Agent patient either way.

**Finding 335 — the untap step gains its third static, and it is a
permission.** "You may choose not to untap [n] during your untap step" is
seven lines and one shape, overriding [CR#502.3]'s default that the
active player untaps all their permanents. `MayDeclineUntap` files under
`DeedRestriction` with `DoesntUntap` and the cap — not because it
restricts anything but because that class is exactly its reader profile:
a durationless clause refused (`badDeclineUntapClause`) and the printed
static line admitted, which is what seven lines of card text with no
adverbial need. The possessive is derived and not a slot, finding 254 at
a third site: the permission belongs to the untapping player.

**Finding 336 — witnesses, and one pin landed deliberately redundant.**
Four positives, two of them whole cards: Berserkers of Blood Ridge (the
standing requirement, and the cleanest demonstration that the cadence is
not a span — a printed line with no adverbial anywhere), Trumpeting
Armodon (the Block/Agent cell with its required patient, under a span),
Loathsome Catoblepas' first ability (the patient role, which writes the
modal outright where the agent cells write the plain present), and
Ashnod's Battle Gear's first line (the decline permission). The AURA
grant does not land — "Enchanted creature attacks each combat if able" is
six lines and every one of them needs the attachment family, which is
ledgered — so the requirement's second-largest carrier is recorded, not
witnessed.
`badMustAttackLand` refutes the SAME obligation as `badCantAttackLand`:
`DeedParticipant Attack Agent (Just Land)`, one P at two polarities. That
redundancy is the point and it is landed on round seven's precedent,
where the shared lookback table was pinned at both of its readers: a
claim that two rows share a grid is worth a proof that they do, and the
proof is that the same refusal appears from both.

**Finding 337 — four residues, routed rather than absorbed.** "All
creatures able to block [X] do so" is 17 lines and belongs to the
combat-assignment surgery entry, whose could-block condition is exactly
what it needs — it is a universal over an ABILITY to block, not a
requirement on a named creature. The targeting requirement is ONE line
(the Flagbearer family) and is recorded, not minted, at finding 85's bar.
The event-level "if able" riders are 5 lines and ride an effect rather
than a static, which is a different carrier. And the GATE family is
measured at close for the trigger finding 331 states: 100 lines, 25 with
a payment. Two families confirmed NOT this one and not absorbed: the
as-though counterfactual (~45+ lines, its own [CR#609.4] machinery) and
the causative "you may have X …" (262 lines, an effect and not a
deontic).

Ledger updates this chapter:

- The deontic beyond `Cant` (**half closed**): the requirement and the
  decline permission land. The queue entry narrows to the GATE polarity
  and its unless-pay family, which is also finding 331's convergence
  trigger.
- The three-vocabulary situation (**recorded**, finding 331) with the
  ruling and its forward edge, so the next round finds the reason rather
  than the divergence.
- Routed: the all-able-do-so universal to the surgery entry; the
  Flagbearer targeting requirement and the event-level riders to the
  ledger at their counts.
- Near-miss inventory: the Aura grant, which costs the requirement its
  second carrier and is the attachment family's again; goad's attack leg,
  routed to the DESIGNATIONS family on [CR#701.15b]'s own word.

## Chapter forty-seven — the designations

Three roles the rules call designations, one the game itself holds, and a
grammar that had no word for any of them. This chapter gives the
workbench its designation axis at all three scopes: the reads, the
gaining effects, and the game's own transition event. Bench: 263
positives / 412 pins.

**Finding 338 — the designation exists three times and the workbench had
NONE of it, which is a sharper starting point than the round expected.**
Core's taxonomy is complete and wired: `DesignationScope{Object, Player,
Game}` over `DesignationDef{Stored, Derived, DerivedIf}`, with shape,
uniqueness and persistence metadata (`deckmaste_core/src/designation.rs`).
The OLD module has an open `MkDesignation Scope String (List Ability)`
carrying the player and object scopes and no game scope — no Ring-bearer,
no day and night as a designation — plus a stringly `SetGameDesignation`
write with no read. That write is the OLD module's, not this one's, which
the round's brief had placed here: `Experimental` contains no designation
machinery of any kind. So these rows are the first Idris consumers of the
axis at EVERY scope rather than at the game scope alone, and the plugin
emitter's own note is the gap they close from the other end —
"Game-scoped designations have no Idris `Scope` and are simply absent (a
reference then gaps)" (`deckmaste_plugin/src/idris_emit.rs`).

**Finding 339 — the game gets two rows and not a `Kind`.** The tempting
move was a third sort beside `Object` and `Player`, so a designation read
could be kind-indexed throughout as the counter read is (chapter forty's
`counterScope` answering in `Kind`). The corpus does not pay for it. Four
check lines and eleven transition lines buy two rows; nothing else in the
grammar ever describes, targets, counts or quantifies over the game, and
a `Kind` row would have obliged every kind-keyed table in the file to
answer for a sort with one inhabitant. So `TimeOfDay` is a two-value enum
and the game-scoped read and write are dedicated rows with no subject
slot — `ItIsNow` and `BecomesTime` naming the game with a dummy pronoun,
exactly as English does.

**Finding 340 — "is your Ring-bearer" is a COMPOUND, and writing it as
one row is the spelling-honest choice.** [CR#701.54e] spells the
condition out: a creature is your Ring-bearer exactly when it "is on the
battlefield under your control and has the Ring-bearer designation" —
three conjuncts. The corpus writes NONE of them separately; all three
attested lines write the four words whole. Composing `ControlledBy You`
with a bare designation read would therefore have spelled a phrase no
card prints while leaving the printed one unwritable, which is the wrong
trade in both directions. The conjuncts live in the row's comment and the
first of them lives in its seeding, which is what
`badRingBearerInGraveyard` refuses.

**Finding 341 — the game's two designations are asymmetric in the CHECK
and symmetric in the TRANSITION, and the row set says so.** "If it's
night" and its siblings are four lines; "if it's day" is written zero
times (`badItIsDay`), so `timeCheckOk` gates the read at finding 246's
idiom. The becoming writes both directions freely, which is what makes
the asymmetry a fact about the check rather than about the designation —
and why `BecomesTime` is ungated beside a gated `ItIsNow`.
The TRANSITION event is the round's other spelling decision. Ten of its
eleven headers write the FULL disjunction, "day becomes night or night
becomes day", always in that order and never separately; [CR#731.1a]
gives both phrases as a pair. So `DayNightShift` takes no direction
argument and spells the coordination as one lexicalised phrase — which
keeps it clear of chapter thirty-eight's coordinated-EVENT gap rather
than falling into it, because that gap is about composing two events a
card names separately and this card text never separates them. The single
solo line is recorded rather than generalised into a slot no other line
would use.

**Finding 342 — three designations the old module carries are not minted,
and one argument covers all three.** The city's blessing (28 lines, 15 of
them checks), monstrous (64 / 9) and renowned (12 / 3) are each conferred
by a KEYWORD — ascend, monstrosity, renown — so their BECOMING sits
behind the standing keyword boundary while only their checks are ordinary
card text. Minting the check alone would have given this grammar a
designation that nothing in it can confer, which is a worse state than
not having it: a row whose only reachable use is to test for a condition
no row can bring about. They wait for the keyword transition together,
and the argument generalises — a designation is mintable here when its
becoming is card text.

**Finding 343 — four figures corrected, three of them the round's own.**
The monarch intervening-if family was carried at 16 with no locatable
source; re-measured it is 18, or 12 excluding the "there is no monarch"
absence checks, and this round's own closest measure of the positive
check is 21 lines. Goad's reminder-text figure in `Must`'s row comment
said "~27"; the closest measure is 33 and the live comment is reconciled.
Goad's operative clauses measure 24 where the round's brief said 25. And
the becomes-the-monarch EVENT was briefed at 4 header lines and measures
TWO — both "Whenever an opponent becomes the monarch", both with bodies
this grammar cannot write (a for-as-long-as control grant; an
as-the-turn-began lookback window). That last correction is why the event
is RECORDED and not minted: two lines with no writable body do not buy an
`EventName`, whose cost is now ten table clauses including two
`lookbackSubjectOk` cells for a name no history read writes. The day and
night transition, at ten, does.

**Finding 344 — four boundaries, each argued rather than assumed.** THE
RING TEMPTS YOU is out: [CR#701.54a] makes it a keyword action whose
expansion is choose-then-designate, and its 51 lines are that action's
trigger phrase, not a designation clause. DAYBOUND and NIGHTBOUND are
out: [CR#702.145a] puts them "on opposite faces of some double-faced
cards", so their whole text is keyword-static boilerplate over a card
frame this vocabulary does not model. GOAD'S OWN RULES TEXT is out and
stays rules-implicit — the attacks-each-combat-and-a-player-other-than
sentence is [CR#701.15b]'s, printed only as reminder text, so chapter
forty-six's requirement row neither spells it nor needs to. And the
MONARCH'S PACKAGE is out in an asymmetric way worth recording: its draw
half is printed exactly once (Archivist of Gondor, "At the beginning of
the monarch's end step", blocked on the nominal possessor tier chapter
forty-five queued) and its damage-transfer half is never printed at all,
so the package is rules text with one card-text corner.

**Finding 345 — witnesses, pins, and three rows that land unwitnessed.**
Five positives: Throne Warden (the whole card — the monarch check feeding
the intervening slot for free), Aragorn (the designation gained, and the
`Monarch` value's own verb), the goad clause under an activated carrier,
a goaded-description trigger header, and Firmament Sage's second line
(the game-scope event). Three pins: the day cell, the monarch read's
negation (zero lines, where the family's real negative is the absence
check), and the Ring-bearer's battlefield seeding.
THREE ROWS LAND WITHOUT A BENCH POSITIVE, each with a named blocker.
`ItIsNow` and `BecomesTime` have four and eleven corpus lines between
them and not one is writable whole: the checks ride a cost reduction, a
replacement "instead", or an if-otherwise pair, and the becoming appears
only inside those. `YourRingBearer` has three lines, and its best —
Frodo Baggins' "As long as Frodo Baggins is your Ring-bearer, it must be
blocked if able", which would have closed the card with chapter
forty-six's requirement — is blocked by the SELF-PRONOUN gap alone: the
card pronominalises its second self-mention and this grammar writes the
self-word twice. That is the fifth construction the gap has cost a
witness, and it is now the most expensive unbuilt thing in the
vocabulary.

Ledger updates this chapter:

- The designations (**closed**): all three scopes land — the player
  reads and the gaining effect, the object reads and the goad verb, the
  game read, write and event. The queue entry is deleted.
- **Recorded, not minted**: the becomes-the-monarch event (2 lines,
  finding 343); the absence check "there is no monarch" (5 lines, an
  existential over the holder); the three keyword-conferred designations
  (finding 342).
- **Boundaries** (finding 344): the Ring tempts you; daybound/nightbound;
  goad's rules text; the monarch package's two halves.
- Near-miss inventory: the SELF-PRONOUN gap, now at five constructions
  and costing this round its best witness; the nominal possessor tier,
  which costs Archivist of Gondor; the plural-player possessor, which
  costs the goaded-can't-block line.

## Chapter forty-eight — the event subject's own mention

Five chapters have recorded the same near-miss in their inventories: a
trigger whose subject is the self and whose body says "it". Chapter
thirty-eight witnessed around it, chapter forty called it a limit, chapter
forty-two met it again, chapter forty-five paid for it, and chapter
forty-seven called it the most expensive unbuilt thing in the vocabulary.
This chapter builds it — one determiner row, one screening clause, one
minting function, seven call sites — and reports what it did not build
just as precisely. Bench: 265 positives / 413 pins.

**Finding 346 — the gap was TWO axes running together, and separating
them is most of the work.** The log's prose has treated one phenomenon
where there are two. AXIS ONE is ANAPHORA: `nounDelta This` is the empty
list, so a bare or sorted self announces no mention and `It` and the
demonstratives have nothing to resolve against. AXIS TWO is
ZONE-EVIDENCE: the sorted self writes a type word, which places it on the
battlefield ([CR#109.2], `selfSortedOk`), so a phrase asking about
another zone contradicts it. Chapter forty-one's probe belongs to axis
TWO — it was `Matches This (InZone exile)`, a zone question, not a
pronoun one — and this chapter's own brief had it filed as `Matches It`.
The two axes share a symptom (a self-reference that will not do what a
card's text does) and nothing else. THIS ROUND TOUCHES ONLY AXIS ONE, and
`badEntersBareThis`, `badBlocksBareThisPartner` and
`badExileCheckOnSortedSelf` are all green and untouched, which is the
proof that the separation is real rather than rhetorical.
A bookkeeping correction rides here: the running count of constructions
the gap had cost went third (chapter forty-one) to fifth (chapter
forty-seven) with no fourth locatable in between. The jump is recorded
rather than reconstructed; the count that matters is the one this chapter
closes.

**Finding 347 — `This` never announces, and the minting belongs to the
CARRIER POSITION.** Core draws the line and this vocabulary keeps it: an
exophoric reference "names the game situation, is never bound by an
operator", and lives on the frame OUTSIDE the anaphora record `It` reads
(`deckmaste_engine/src/stack.rs`). So `nounDelta This` stays the empty
list and every sorted self built over it announces nothing. What core
ALSO has is binder-scoped minting — inside a binder, `It` is the
innermost bound element — and the workbench's analogue is the position
the self sits in. The grammar has done this once since chapter three: a
MOVED sorted self mints a fresh binding because [CR#400.7] makes the
moved object a new one, which is what lets Flickering Spirit read "it"
(finding 16). `selfSubjIntro` is that same move at the EVENT-SUBJECT
position, and [CR#603.6]'s reading discipline is the warrant — a trigger's
body looks for the object the event happened to, so the header's subject
is a thing the body can refer to. Seven event rows take it, and the
zone-change rows do not need it because `moveIntro` already minted for
them. 270 lines of "this creature …, it" are what it buys.

**Finding 348 — the demonstrative skips the self, which is English's rule
and not a convenience.** "That creature" never picks out the speaker. So
the minted mention has to be visible to `It` and invisible to `That` —
and the vocabulary already had the right place to say it, the DETERMINER,
whose job is exactly to record which construction built a binding and
therefore which readers may see it. `SelfD` is that row; `wordNow` screens
it off in one clause, and every demonstrative reader inherits the rule
through that one function — the counts, the zone read and the retag
scans alike. `countOnes` needs no change at all, because it never looked
at determiners. The refusal it buys is `badThatCreatureIsSelf`
("Whenever this creature attacks, that creature gets +2/+0"), which the
corpus writes zero times.
This is finding 275's diagnostic in a new place: the visibility varies
perfectly with the reader, so it is a fact about the words rather than an
axis, and one clause states it.

**Finding 349 — the patient branch is NOT minted, and minting it would
have been WRONG rather than merely conservative.** Where a trigger writes
a patient, that phrase is the announcement and the subject gets none. The
round's first design minted in both branches and let `It` refuse on
ambiguity; the corpus refutes it. Two lines write a patient and then a
bare "it" — "Whenever this creature attacks a battle, it gets +1/+1 until
end of turn" and "Whenever this creature blocks two or more creatures, it
gains first strike until end of turn" — and in both the pronoun resolves
to the SUBJECT, not by uniqueness. That is a subject-preference rule, a
different resolution discipline from this grammar's uniqueness pronoun,
and refusing those lines as ambiguous would have been the wrong refusal.
The honest residue is stated plainly: in the patient branch the grammar
now admits a bare "it" and reads it as the PATIENT where English reads it
as the subject. That is an over-generation that MISREADS, which no pin
can catch, and closing it needs subject-preference machinery — its own
axis, and not this round's. Both corpus lines are unwritable for other
reasons besides (a battle type; a count-bounded patient), so nothing
writable turns on it today.

**Finding 350 — the constraint matrix, cell by cell.** Deeproot Warrior
lands and is a whole card. Borderland Marauder lands at a DIFFERENT event
row, which is what shows the minting is the position's and not one row's;
the becomes-status cell was probed and lands too. Vertigo Spawn is
UNCHANGED and green — its body reads the patient through `That (TypeW
Creature)`, and finding 348's screening is exactly what keeps the new
mention out of that count. `badIt` and `badTheyIt` are green: two
non-self mentions are still two. Somberwald Alpha and every
description-subject witness are green and untouched, their mentions
having always been announced by `nounDelta`. The whole bench is green —
265 positives and 413 pins — which for a round that revises the discourse
core is the result rather than a gate.

**Finding 351 — Veiling Oddity's pronoun half is FIXED, and what stops it
now is the other axis.** The probe is the round's cleanest evidence for
finding 346's separation. `Matches It (InZone exile)` under a
last-counter-removed trigger no longer fails on the pronoun: `It`
resolves, the count obligation is discharged, and the error that remains
is `ZoneFits (zoneOfIt …) (Just Exile)` — the minted mention says
battlefield, because [CR#109.2] places a sorted self there, and the card
asks about exile. Axis one closed; axis two exactly where finding 346
said it was. The card is still unwitnessed and now has two named blockers
rather than one vague one: the while-marking (chapter forty-one) and the
self's zone evidence.

**Finding 352 — Frodo is diagnosed and NOT landed, and the diagnosis is
the deliverable.** His pronoun sits in a static's body reading a fronted
as-long-as condition's self-subject. `Conditionally` wires the flow the
other way: its own comment says "the condition reads what the wrapped
statement ANNOUNCED, not the other way round: it is typed in `bs` and the
statement in `condDelta`'s nothing". The probe confirms it exactly —
`countOnes Object [] = 1`, the body's context EMPTY. So Frodo needs a
minting site this round did not build: either a static line's own subject
minting for its body, or a condition minting for the statement it wraps,
and choosing between those is a design question about `Conditionally`'s
direction rather than an extension of the event-subject rule. Adding a
second mechanism in the same round is what the round was told not to do,
so it stops here with the site named. Frodo remains blocked on that and
on nothing else — his Ring-bearer read landed last chapter and his
must-be-blocked the chapter before.

Ledger updates this chapter:

- The SELF-PRONOUN gap (**closed at the event-subject position**), which
  five chapters' near-miss inventories recorded. The remaining sites are
  named rather than inventoried: the static-line body (finding 352,
  Frodo), the patient branch's subject preference (finding 349), and
  axis two's zone evidence (findings 346 and 351).
- `SelfD` (**minted**) as a `Determiner` row, and `wordNow` split so the
  screening lives in one clause with `wordReaches` carrying the word's
  own grid.
- Chapter forty-three's `somberwaldAlpha` comment (**corrected in
  place**): it said the bare becomes-blocked cell "has to be" witnessed
  by a description subject, which stopped being true here.
- No queue entry existed for this gap and none is added; the closure is
  this chapter's.

## Chapter forty-nine — the deontic converged

Finding 331 declined to build a compulsion axis and named the trigger
that would justify one. This chapter fires that trigger: `Cant` and
`Must` merge into one `Deontic` row over the `Deed x Role` grid with the
polarity on its own axis, and the GATE joins them there. It also probes
first, and the probe shrank the round twice over. Bench: 266 positives /
415 pins.

**Finding 353 — the convergence, and the one polarity it does NOT
carry.** All three prior arts agree on the shape and disagree on the
spine: core's `Deontic{May, Cant, Must, Gate}` over a `DeonticAction`
whose variants carry `by`/`on` predicates
(`deckmaste_core/src/deontic.rs`); the old module's
`Constrain : Compulsion -> Deed` over an eight-kind relation spine with a
`Priced` sibling; and this vocabulary's narrow grid, which now carries
the polarity as `Compulsion{Forbid, Require, GatedBy Cost}`. The gate's
cost rides its own VALUE rather than a fourth field on the row, which is
`BoostCounter`'s arrangement and for its reason: the payload sits exactly
where the polarity needs it, so no other polarity carries an empty slot
and no gate can be written without one.
What is not taken from either prior art is the PERMISSION. Core's `May`
is the existential floor a granted row widens; this grammar has no line
asking for it, "may attack" being written zero times because attacking is
permitted by default ([CR#506.3]). Three rows, measured; a fourth would
have been symmetry — which is the same test finding 331 applied to
decline the merge in the first place, run again and answering the other
way.

**Finding 354 — PART ONE'S RESULT: the condition-unless mass was already
writable, and already WITNESSED.** The round came to build a gate for 68
"can't attack unless [condition]" lines. They need nothing: the
conditional static's `Unless` marking already spells them, the negation
moved onto the subordinator exactly as that row's comment describes — and
the bench has carried the proof since chapter THIRTY, where the `Unless`
marking landed — Desperate Castaways ("This creature can't attack unless you control an artifact"),
whose own comment counts "a hundred eleven 'can't … unless' lines" and
says "this is the shape all of them have". The round's brief treated a
settled result as open and so did this round's first hour; the probe is
what caught it, and the honest record is that Part One discovered
nothing except that it had already been discovered.
It did REFINE it. The composition needs a NEGATABLE condition
(`markingOk Unless` demands `condNegated`, and `NotCond` is what supplies
it), and `condNegatable` answers False for `CompareAmt`. So the family
splits: an existential condition composes ("unless you control an
artifact") and a COUNTED one does not ("unless you control four or more
artifacts", Gadrak). That is a real boundary inside the 68 and it was not
recorded anywhere before; it belongs to the comparison's own negation
gap, not to the deontic.

**Finding 355 — the gate's writable surface is ONE line, and the reason
is a gap in BOTH vocabularies.** The cost-gate family is 26 lines.
EIGHTEEN of them write a scaled cost — "pays {2} for each creature they
control that's attacking you", Propaganda's family — which the `Cost`
vocabulary has no term for; six write a coordinated deed ("can't attack
or block"), which `Deed` cannot spell as one row; several are Auras
("Enchanted creature can't attack unless its controller pays {3}",
Brainwash), waiting on attachment. What is left is Hipparion. The scaled
cost is worth recording carefully because it is not a narrowing of core:
core's gate carries a flat `Arc<[CostComponent]>` and cannot spell "for
each" either, so this is a gap in both vocabularies at once and the round
that closes it closes both.

**Finding 356 — the payer is derived, at a fourth site, with one
principled exception.** Every attack and block gate names the gated
subject's own controller — "its controller", "their controller", or "you"
where the subject is the self — so the possessive is a function of the
subject and the spelling owns it, which is finding 254's rule for the
fourth time (after the standing lock, the timed lock and the cap). The
exception is the Block/PATIENT cell, where the DEFENDING player pays
("can't be blocked unless defending player pays"), and it is not an
exception to the rule so much as an instance of it: the payer is still
read off the deed's participants, and at that cell the relevant
participant is the other one. Four lines, all of them per-each or Aura,
so the cell is admitted with corpus evidence and no bench positive.

**Finding 357 — the patient cell goes THREE-VALUED, and that is the
merge's real gain.** Chapter forty-six's table was required-or-refused
because the requirement is. The restriction is not: "can't block
creatures with power 3 or greater" is fourteen lines and the bare "can't
block" many more, so `ForbidT Block Agent` is OPTIONAL — a cell `Cant`
never had, because `Cant` had no patient slot at all. So the merged row
buys fifteen lines of new surface (fourteen restrictions with a patient,
plus Hipparion) rather than the one the gate alone would have bought,
and that is what tipped the trade. `deonticPatientOk` is full rows over
(polarity x deed x role), twelve cells, each measured in its own family
and none by symmetry.

**Finding 358 — the gate's "unless" is the THIRD in this grammar, and
construction-owned.** It is not `CondMarking.Unless`, which marks a
negated CONDITION on a conditional static (finding 354's family). It is
not [CR#118.12a]'s may-else, which offers a player a choice between doing
a thing and paying. It prices a DEED: the clause forbids, and naming a
payment lifts the forbidding. [CR#508.1d] and [CR#509.1c] both write it
into their own text — "if a creature can't attack unless a player pays a
cost, that player is not required to pay that cost" — which is also the
sentence that keeps the gate out of the requirement solver.

**Finding 359 — the migration, and which inferred obligations moved.**
Seventeen call sites across five files, all green. TWO pins' obligations
are byte-identical because their table did not change:
`badCantAttackLand` and `badMustAttackLand` both still refute
`DeedParticipant Attack Agent (Just Land)` — one at each polarity, which
is the pair chapter forty-six landed deliberately and which the merge
now makes literally the same row refusing twice. TWO pins' obligations
SHIFTED and the shift is presentational: `badMustAttackWithPatient` and
`badMustBlockNoPatient` refuted `mustPatientOk`'s application before and
now refute `admitsPatient`/`notRequired` over `deonticPatientOk`'s, which
reduce all the way to `False = True`. The refusal is the same question at
the same cell; what is lost is that the displayed obligation no longer
NAMES the cell, and the pins' docstrings and terms carry it instead. That
is the one quality cost of the merge and it is recorded rather than
hidden.

**Finding 360 — one new witness, two new pins, and the residues.**
Hipparion lands whole and needs both of the round's capabilities at once
— the gate polarity carrying its cost, and the restriction's optional
patient. Desperate Castaways already carried the condition cell and is
cited rather than duplicated. Two pins at the merged table's refused
cells: a patient on the attack restriction, and a patient on the
block-patient gate. The residues are precise: the SCALED cost (18 lines,
and a core gap too); the coordinated deed (6 lines, chapter
thirty-eight's coordinated-event gap in a new place); the Aura carrier;
the counted-condition unless (finding 354); and the defending-player
noun, which chapter forty already recorded and which this round meets
again at the Block/Patient gate.
A boundary confirmed and not absorbed: WARD is not a gate. [CR#702.21a]
makes it a triggered ability whose effect counters the spell unless a
cost is paid, so its "unless" prices a TRIGGER's resolution and not a
deed's legality, and the corpus's counter-unless family is disjoint from
this one. Recorded with the old module's own `tToll` mislabelling —
`Spec.idr` files a Downstream toll where the ward reading wants
AtDeclaration — as a flagged note about that module, not this round's fix.

Ledger updates this chapter:

- The deontic (**converged**): `Cant` and `Must` are one `Deontic` row
  with `Compulsion`'s three polarities. Finding 331's trigger is
  discharged and its ruling superseded by its own terms.
- The condition-unless family (**closed, and it was already closed**):
  finding 354, with the counted-condition boundary as the one new fact.
- The queue's GATE entry deletes; the residues go to precise lines.
- Near-miss inventory: the scaled cost, in both vocabularies; the
  coordinated deed; the defending-player noun, now costing two cells.

## Chapter fifty — the conditional's threading

Chapter forty-eight minted the event subject's own mention and diagnosed,
precisely and without building it, why the same pronoun would not resolve
inside a conditional static. This chapter builds that — the same three
lines at the second container — and closes the card the diagnosis was
about. Bench: 268 positives / 416 pins.

**Finding 361 — a correction first: the threading this comment described
was never implemented.** `Conditionally`'s row said "the condition reads
what the wrapped statement ANNOUNCED, not the other way round: it is
typed in `bs` and the statement in `condDelta`'s nothing". Half of that
was true and the operative half was not. Both arguments sat at the raw
incoming context; neither read the other; nothing was typed in anything.
The sentence described an INTENT, and chapter forty-eight's diagnosis
quoted it as though it described the code — which is how a probe came to
be needed to discover that the flow ran in no direction at all. The
comment now describes what this chapter builds, and the lesson is worth
one line: a doc comment that states a threading is a claim about a type
signature, and the signature is where it should have been read.

**Finding 362 — two flows, and only one of them is a threading this
constructor can have.** The 424 fronted "As long as" lines divide three
ways. 173 are INDEPENDENT — the condition names its own subject and the
body names its own — and have always been writable; every `Conditionally`
witness the bench carries is one of them, which is why the rewiring broke
nothing. 110 repeat the self-word in both halves and were likewise
writable. 141 PRONOMINALISE the condition's subject in the body (55
self-typed subjects, 8 proper-named including Frodo and Enkira's
byte-similar sibling), and those are this chapter's.
The other direction is real and is NOT this chapter's: about 70 trailing
lines pronominalise the STATEMENT's subject inside the condition ("X has
hexproof as long as IT's untapped"). One constructor cannot type both
arguments in each other's context, so the two flows cannot share a
threading, and the round takes the fronted one — the larger family, and
the one whose diagnosis was already written. The row's linearization
comment is corrected accordingly: fronted and trailing are the same
sentence when the condition is INDEPENDENT, and two constructions when a
pronoun crosses. (A third family, "for as long as it remains exiled" and
its ~59 siblings, is `ForAsLongAs`'s and already threads through
`Continuously`'s `staticIntro` — a different construction, untouched.)

**Finding 363 — the container threads; the condition stays opaque.**
`condSubjIntro` is `selfSubjIntro`'s three lines at a second site, and the
chapter sets them side by side because the ONE difference is
informative: an event's subject is a NOUN, so a description announces
itself through `nomIntro` and only the self needed minting; a condition
announces nothing on any row, so there is no fallback and the unminted
case passes the incoming context through unchanged. `condDelta` is
untouched and still answers the empty list everywhere — the container
mints for its own body, and the condition does not become transparent.
The regression set proves the distinction rather than asserting it.
`badMatchesTargetSubject` (the `Bindingless` gate) and
`badConditionAntecedent` (`Effect.If`'s opacity) are both green with
byte-identical obligations, and `Effect.If` is untouched: its condition is
typed at `preIntro e`, threading the other way, which is a second
container with a second answer and finding 76's cross-carrier contract is
what lets each decide for itself.
The demonstrative screen needed no work at all. `SelfD` is invisible to
`wordNow`, so the new mention is visible to "it" and not to "that
creature" — and the corpus writes zero fronted lines whose body
demonstrates its own subject, exactly as it wrote zero of the trigger
twin (`badThatCreatureIsCondSubject`, whose obligation is
`badThatCreatureIsSelf`'s to the letter).

**Finding 364 — Frodo Baggins, four chapters on.** "As long as Frodo
Baggins is your Ring-bearer, it must be blocked if able." Chapter
forty-six minted the requirement, chapter forty-seven the Ring-bearer
read, chapter forty-eight diagnosed the pronoun and stopped rather than
adding a second mechanism in one round, and this chapter threads the
container the diagnosis named. His first line stays elided — the Ring
tempts you is a keyword action ([CR#701.54a]) over a coordinated event
subject — and the second is the whole of what the arc was about. Adanto
Vanguard lands beside him on an ordinary status condition, which is what
shows the minting belongs to the container and not to the Ring-bearer
read.
The two who did NOT land are recorded: Enkira's sibling wants "as long as
[it] is equipped", an attachment predicate the Aura family owes, and the
enchanted-subject lines want a noun that does not exist for the same
reason.

Ledger updates this chapter:

- The self-pronoun gap (**closed at the conditional's fronted flow**),
  the second of the sites chapter forty-eight named. What remains of it:
  the TRAILING-reverse flow (finding 362), and the patient branch's
  subject preference (finding 349).
- `Conditionally`'s row comment (**corrected**, finding 361) and its
  linearization story (**corrected**, finding 362).
- Frodo Baggins (**closed**), the arc chapters forty-six to fifty built.
- Near-miss inventory: the attachment predicate, which costs Enkira and
  the enchanted-subject family; the trailing-reverse orientation.

## Chapter fifty-one — the attachment host

Fourteen chapters have recorded the same block under different names: the
Aura grant chapter forty-six could not witness, the flat gate chapter
forty-nine could not find a second line for, Enkira beside Frodo in
chapter fifty, and "enchanted creature" in every near-miss inventory
since chapter twenty-two. All of it was one missing NOUN. This chapter
mints it — 1,774 corpus lines across its head words, the campaign's
largest measured unlock — and the boundary around it turns out to be
clean. Bench: 272 positives / 418 pins.

**Finding 365 — the gap is COMMON to both prior arts, and the rules make
the boundary clean.** Core has the reference: `Reference::AttachHostOf`,
"the permanent that attachment R is attached to … covers Equipment hosts,
Aura enchantees, and Fortification hosts alike"
(`deckmaste_core/src/reference.rs`). The OLD module has the same
primitive with `refIntro (AttachHostOf r) = refIntro r`, a passthrough
that announces nothing. NEITHER has a NOUN surface, so this is the first
one in the repo rather than a port of either.
What makes it landable with no attachment machinery at all is a pair of
rules that decouple the word from the speaker: [CR#303.4m] and
[CR#301.5f] both say the reference works "even if the permanent with the
ability isn't an Aura"/"isn't an Equipment". So the participle is a
POINTER a permanent may write, not a fact about what kind of permanent it
is — no Aura subtype, no attach verb, no relation. The corpus confirms
the boundary from the other side: the paraphrase "the creature this is
attached to" is written ZERO times, so the participle is not one spelling
among several but the only one.

**Finding 366 — the participle x head grid, and why three lines mint a
row.** Full rows over (participle x head word), thirty-three cells.
ENCHANTED spans six heads because [CR#303.4b] lets an Aura attach to an
object OR a player: creature 931, land 79, permanent 76, player 54,
artifact 22, enchantment 4. EQUIPPED is creature-only at 605,
[CR#301.5a] naming no other host (`badEquippedLand`). FORTIFIED is
land-only at 3 (`badFortifiedCreature`), and those three lines MINT the
row rather than record it — which looks like a departure from finding
85's bar and is not. That bar asked for a line of the right SHAPE, and
[CR#301.6] applies the Equipment rules "to Fortifications in relation to lands
just as they apply to Equipment in relation to creatures", so "fortified
land" is the rules' own term with exactly the status "equipped creature"
has. The row is rules-backed vocabulary, and the three lines are its
witnesses rather than its justification.

**Finding 367 — the head word carries the evidence AND the sort.** This
is `AsType`'s arrangement generalised: there a type word placed the self
on the battlefield ([CR#109.2]), and here the head word both places the
host and names its type, so "enchanted creature" projects a battlefield
creature and "enchanted player" projects nothing at all — players having
no zone. The KIND follows the head too, `Noun bs (kindOfW h)`, which is
`That`'s own indexing; that is what lets the 54 Curse lines' player host
share the row instead of forking it, and it is chapter forty's
`counterScope`-in-`Kind` move at a third site.

**Finding 368 — the mention is ORDINARY, and the two exophora differ.**
The new noun announces nothing by itself (`nounDelta` empty, both prior
arts agreeing), so the pronoun lines need the container to mint — rounds
twelve and fourteen's rows extended, at the event subject and the
condition subject. The DETERMINANT is the round's real question, and the
corpus answers it against the obvious guess. `SelfD` exists because no
line demonstrates back to a trigger's own subject (finding 348, zero
lines). Five lines demonstrate back to an attachment HOST — "When
enchanted creature dies, THAT CREATURE's controller loses life equal to
its toughness" and its siblings — so the host's mention is visible to the
demonstratives and takes `TheD`. English's rule is what separates them
and it is the same rule in both cases: "that creature" cannot mean the
speaker, and an enchanted creature is not the speaker but a third party
the sentence named. Two exophoric references, two determinants, one
principle.
(Two of the seven candidate lines demonstrate the block PATIENT rather
than the host and are not evidence; the count is five.)

**Finding 369 — the inverse direction lands too, and a third is
recorded.** "Is enchanted" (14 lines) and "is equipped" (19) ask whether
something HAS an attachment rather than what an attachment points at, and
they are predicates rather than a noun — written predicatively after the
copula and never prenominally. Both negate, four isn't-forms saying so. A
THIRD direction exists and is not this chapter's: "as long as this
Equipment is attached to a creature", twelve lines, which names the
RELATION from the attachment's own side and is the one place an attach
relation would actually be written. Queued.

**Finding 370 — the backfill harvest, three chapters closed by name.**
Bloodshed Fever closes chapter forty-six's recorded block: that chapter
minted the requirement and could not witness its second-largest carrier
because the subject had no noun. Brainwash closes chapter forty-nine's:
the cost gate landed there with exactly ONE writable line because every
other flat gate in the family is an Aura, and this is its second witness.
Enkira closes chapter fifty's sibling — the condition asks whether the
self has an attachment and the body pronominalises the self, so he lands
on `condSubjIntro` exactly as Frodo did, one chapter and one predicate
later. Extra Arms witnesses the minting extension itself. Four cards, and
every one of them was named as blocked in an earlier chapter's ledger.

**Finding 371 — what is still blocked, precisely.** The enchanted PLAYER
cell has 54 corpus lines and no bench positive: its two cleanest lines
are "Enchanted player can't gain life" and "can't cast more than one
spell each turn", and neither prohibition is a `Deed` this vocabulary
carries. The "enchanted player's upkeep" family (10 lines) overlaps the
nominal-possessor tier chapter forty-five queued and is untouched. And a
MOVED host mints nothing — `This`'s answer rather than `AsType`'s —
because an Aura that moves its own host stops being attached to it, so no
line reads a moved host back; recorded rather than minted.

Ledger updates this chapter:

- The attachment subject (**closed**), 1,774 lines across its heads, with
  the inverse direction beside it.
- Bewitching Leechcraft (**down to two blockers** from three): the
  attachment subject is gone; the would-untap event and the quotation gap
  remain. Queue entry updated.
- Chapters forty-six, forty-nine and fifty's recorded blocks
  (**backfilled**, finding 370).
- Near-miss inventory: the enchanted-player prohibitions, which want
  deeds this grammar has no rows for; the attachment's own direction (12
  lines); the moved host.
