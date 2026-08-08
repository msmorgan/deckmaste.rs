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
   none rather than accreting it.

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
evidence); the
library zone and its ORDERED positions ("into its owner's library
second from the top", "on the bottom of its owner's library" — a
sequence structure no current zone carries, arriving with the
draw cluster); miracle's reveal condition ("the first card you've
drawn this turn" [CR#702.94a] — a turn-scoped draw-ordinal
memory, same cluster); the up-to-one singular
remention positive (Ty Lee, Chi Blocker — its second sentence is
the UNTAP deed under a "for as long as" duration ([CR#611.2b]), so
it waits on both, chapter fifteen's restriction clause having
landed with neither); [LANDED, finding 73 — `May`'s if-you-do /
if-not branches are two `Maybe` slots on the clause, and the
declined arm reads only what preceded the may]; the MANDATORY
"if you do" ("[Do something]. If you do, …" with no offer — a
hundred and forty-two lines write no "may" anywhere), which
[CR#118.12] governs jointly with the offered form and calls a
cost check rather than an event read, so it waits with the cost
algebra that has to spell an unoffered payment; [LANDED, finding 88 — the ELSE sentence
"Otherwise, …" is `If`'s third slot, a `Maybe` field typed in the
discourse BEFORE the conditional and contributing nothing outward,
opened on Unholy Annex once tokens and counters made both of a
card's arms writable; only the leading linearization spells it]; the LEADING conditional that
announces a target its consequent reads ("If target creature has
toughness 5 or greater, it gets +4/-4 until end of turn", Blood
Lust; Hidetsugu's Second Rite; Meddle — ten "if target" lines,
four sentence-initial), which is legitimate by [CR#601.2c] and
needs a delta carrying only a phrase's target half; the STRICT
comparators in the condition frame ("if your life total is less
than 7"; fifty-one "is less than" lines, eighteen "is greater
than", almost all against a phrasal standard) together with the
life-total reader they mostly measure, which `Characteristic`
does not carry and which would make `Comparator` a per-frame
table; [LANDED, finding 95 — the CONDITION negation is
`NotCond`, over a closed table of the frames English negates:
the existential and the reference frames, not the comparison
frame and not another negation. The wait's own count was short by
a factor of five]; "unless" whole
— the action form is [CR#118.12a]'s own rewrite into finding 73's
node and waits on the cost algebra (a hundred forty-five "unless
you pay", a hundred eighty-six "unless [someone] pays",
seventy-one non-mana payments), the state form is a negated
condition whose carriers are entering-tapped replacements and
durationless static deontics (sixty-two "unless you control", a
hundred nine "can't … unless"), and the conditioned deontic
should re-use `Condition` inside `Cant`'s shape as core does with
`StaticEffect::Conditionally`; "if able" (two hundred fifty-one
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
the colon, constructor unminted); the cost GRAMMAR (the colon
accepts ANY clause as a cost, but [CR#602.1a] makes a cost what
the activator pays — delayed clauses and "you lose N life" are
not payments [CR#118.1,119.4], and a cost's OUTCOME mention leaks
through `publicOnly` to the effect; restrict the pre-colon sort
with the cost-participle work); hidden-zone identity (a move into
hand keeps its binding readable, but [CR#400.7] mints a new
object and [CR#400.7j] lets the effect re-find it only in a
PUBLIC zone — introduction-in-hand via predicate stays legal,
retention across a hidden-bound move must not; wants a
trackedness distinction the payload does not yet carry); more event queries (upkeep / end-of-combat /
leaves-the-battlefield — Slaughter Pact, Mirror Match, and
Kjeldoran Elite Guard wait on pay, tokens, and an unknown-zone
retag); the player pronoun "they"
(corpus-attested only inside trigger and unless clauses — Havoc,
Tergrid's Lantern — so `They`'s positive waits on those
constructions), player groups ("each opponent … they"
distributives), and "the rest"; event-outcome residues (chapter
ten built the scalar "that much" read; "that many" count reads
wait on their consumers — draw, tokens, counters are the
corpus's big three — "this way" participant-subset participles
are the `TheVerbed` cousin and the largest family, and more
sorts wait with them: mana produced, Sakiko; card counts,
Asmodeus; and condition-supplied scalars — Tellah, Great Sage's
mana SPENT is not a clause outcome at all, so it arrives with the
conditions axis); "any number of" / "X" target groups
(corpus-frequent; the QUANTITY spells the unbounded form now, while
the variable one waits on bounds that admit a variable — core's
range is over a `Count`, whose `X` row this one's `Nat` has no
answer to — and both wait on verified whole cards for a positive),
and
with them the PLURAL damage-class forms — [CR#115.4] names
"another target," "two targets," and similar expressly alongside
"any target", and the corpus writes them either as a division
("deals 2 damage divided as you choose among one or two targets",
Chandra's Pyrohelix) or as an each-of recipient ("Fall of the
Titans deals X damage to each of up to two targets"); the fifth
duck wave adds two witnesses to the same axis, both refused as
fix-round patches and parked here instead. The COUNTER clause
takes the each-of recipient too — "Put a +1/+1 counter on each of
up to two target creatures" (Ajani, Adversary of Tyrants) — so the
distributed recipient is not damage's alone, and what the counter
verb accepts today (a bare up-to group) is one distributive word
short of what the card writes. And a distributed AGENT makes a
distributed group: "Each player creates a green Elephant creature
token. Those creatures have …" (Elephant Resurgence) makes one
token per player and reads them back as a plural, where the create
clause reads its output plurality off the COUNT alone and so calls
them one. Both want output plurality derived from agent
distributivity as well as per-agent amount, which is the plurality
axis itself rather than a gate on either clause. The bare
up-to noun already spells (`TargetGroup (upTo n) AnyTarget`), as
does the unbounded quantity itself (`anyNumber`), while the
division and each-of RECIPIENT structures and a verified whole-card
positive wait with this axis, and the exact-count mention admits
the class word only at ONE — the singular form itself — refusing it
from two up, and from the unbounded quantity, until they land
(`AnyTargetAtCount`);
counted UNTARGETED groups ("four lands you control" — Burning of
Xinye's subject-destroy positive waits on them); the
reciprocal fight frame ("those creatures fight each other" — one
exactly-two-membered plural subject, corpus-attested, a distinct
construction from binary `Fights`); union-read plural possessors
("creatures your opponents control", corpus-attested — waits with
the player groups); the self-exclusion "other" anchor (Olivia
Voldaren, Red Hulk — "another"/"any other" excluding the SOURCE;
the sorted self-reference is typed and battlefield-projected
already (finding 41), so what this waits on is the source ENTERING
the discourse: `This` and its ascriptions introduce no binding, and the anchor
search reads bindings);
static "as long as" conditions ([CR#611.3], Kitesail Corsair —
the card has no "for"; the rest of eleven hundred thirty-five
"as long as" lines once the two hundred ten "for as long as" are
taken out, and a static-ability container rather than a duration)
and effect-created "for as long as" durations ([CR#611.2b] —
finding 75 measured it: core's `Duration::ForAsLongAs(Condition)`
is a row that EXTENDS chapter seventeen's `Duration` and the
condition half now exists, so what it waits on is a clause.
Forty-two of the two hundred ten lines are control grants, four
are keyword grants and all four grant "indestructible", four are
stat deltas and all four end "for as long as this artifact remains
tapped", nine are restrictions and eight coordinate deeds or name
a deed with no clause here; the one single-deed line, There and
Back Again's "Up to one target creature can't block for as long
as you control this Saga", needs a type word `CardType` does not
have); the EVENT-ended endpoint ("Exile another target
creature until this creature leaves the battlefield", the O-Ring
family, ninety-one lines — an endpoint named by an event rather
than a turn boundary, which is why chapter seventeen's
`DurationEnd` has no row for it: core spells it
`Duration::UntilEvent(EventFilter)` and the events axis does not
exist here, so the row waits on the axis and not on the shape);
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
chosen [quality]" (quality-kind bindings); coordinated verb
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
clause and nothing else. The STATIC form is the largest piece:
"Enchanted creature can't attack" (Pacifism) is durationless and
continuous, so it belongs to the ability layer with the other
static abilities rather than to any clause. Beside it: deed
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
registry — where this file grows witnessed rows, and the two
choices are the same choice made twice); token SUPERTYPES
("create Boo, a legendary 1/1 red Hamster creature token", forty-six
lines), a third list on the type line neither reader witnesses;
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
written amount vocabulary has no term for; the enters-with riders
("enters with … counters", four hundred fourteen lines; "enters
tapped", a hundred thirty-two), which are the replacement axis's
and not the create instruction's; the type-SETTING sibling of
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
lines write the header with a period instead of the em dash), a
replacement of the modal's own quantity and so R6's; "Choose up to
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
object status — tapped/flipped/face-down/phased as a per-object
state dimension ([CR#110.5]; no Untap verb, no Tapped predicate:
"destroy target tapped creature" is unspellable); pile partitions
(Death or Glory partitions the GRAVEYARD, so this is not the
library gap — [CR#700.3a] puts each object in exactly one pile
"unless the effect specifies otherwise" and [CR#700.3d] lets a
pile be empty, while [CR#700.3c] only keeps the piles in the zone
they came from; labels, choice, and complement are the card's own
instructions); modal clauses ("Choose one —",
[CR#700.2,115.8] — effect-row alternatives with branch-local
targets); truth-valued conditions and branching [LANDED in part,
chapter eighteen — `Condition` and the `If` clause; what is still
frontier here is the BRANCH, Galvanic Blast's "instead if" being a
replacement (R6) rather than an else-arm, and [CR#603.4]'s
separation of intervening-if from English "if" being the seam
finding 76 leaves for the trigger layer]; linked-ability memory (Cold Storage, [CR#607.2a] —
source-keyed exile notes across abilities, beyond any one
discourse); divided amounts (Chandra's Pyrohelix,
[CR#601.2d,115.7f] — an allocation announced and locked);
dependent iteration (Killing Wave's "for each creature, its
controller…" — a per-member singular frame no plural noun
provides); aggregation and extremal selection (Crackling Doom's
"greatest power among"); data-dependent repetition (Torment of
Hailfire's "repeat this process X times"); turn-schedule
insertion (Relentless Assault, [CR#500.8]); standing triggered
abilities ([CR#603.1] — the ability SHAPE beyond delayed
queries); counters as per-HOLDER state [LANDED in part, chapter
nineteen — the object-borne put and remove verbs over a witnessed
`CounterKind`; what is still frontier here is the PLAYER holder,
which English reaches with a different verb ("target player gets a
poison counter", forty-eight lines), the counter COUNT read ("the
number of +1/+1 counters on it"), and the kind-blind quantifiers
below]; mana production and payment
([CR#106.4]); DESIGNATIONS, which ride all three holders the way
the counters above ride two — a player's ("the monarch",
[CR#725.1]; "the initiative", [CR#726.1]), a permanent's (the
Ring-bearer, [CR#701.54b]), and the GAME's own (day and night,
[CR#731.1]) — each a definite noun phrase naming a role rather
than an object, which is why the definite sweep below surfaced
them and had nowhere to put them; and control ASSIGNMENT on a
battlefield move — a
controller argument the move primitive does not carry, since
[CR#110.2a] gives the instructed player by default and only an
explicit "under [player]'s control" overrides it. Leaf
vocabulary: Transform ([CR#701.27a]);
life-total Set ([CR#119.5]); the non-additive continuous family
(gain control, "becomes", lose abilities, set base P/T — the
layer words).

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
frequency riders
("Activate only as a sorcery", "Activate only during your turn and
only once each turn") — a legality rider with a turn-scoped count,
the ability layer's rather than any clause's; nonfinite and
elliptical clauses — infinitive, gerund, and ellipsis have nothing
to lower to while every effect here is a saturated clause; the
passive and its reduced-recipient forms ("Whenever this creature
is dealt damage, it deals that much damage to the chosen player",
Stuffy Doll) — recipient fronted and agent dropped, arriving with
the event queries above rather than as a clause shape of its own;
exception riders ("except it's legendary", "except it's not
legendary") — one characteristic of what the clause just made,
overridden after the fact, so they want the copy/token vocabulary
and the layer words together; the group COMPLEMENT, one family
under three words — "the rest" ("Put one of them into your hand
and the rest on the bottom of your library in any order", six
hundred and fifty-two lines), "the other" ("Exile one of them and
put the other into your hand", a hundred and ten bare uses), and
"both" above (nineteen) — selection from an established group and
the leftover it defines, twin of the pile partitions below, and
the one deferral the definite sweep found that names a missing
PRIMITIVE rather than a missing word: a binding records a
mention's `Plurality` and its `Determiner`, neither of which
carries how MANY, so nothing here can presuppose a two-membered
antecedent or subtract one mention from another — and the fight
macro is the second construction now waiting on it, its own
expansion being "(Each deals damage equal to its power to the
other.)" (finding 20); set-exception
noun phrases
("choose a card type other than creature", Arachne, Psionic
Weaver) — a complement
over a QUALITY domain, which `Other`'s object distinctness is not,
waiting with the chosen-quality bindings; arithmetic and rounding
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
participle read, and "the rest" — the first two typed here
already as `OfChosen` and `TheVerbed` (uniqueness-gated exactly
as the guide states it), the third the group complement above,
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
