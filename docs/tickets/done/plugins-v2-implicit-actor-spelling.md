---
needs: [semantics-v2-actor-handoff]
---
**Every RON helper, keyword action and canon card is spelled with the
implicit actor.** Landing (2) of the actor-handoff design agreed with the
owner on 2026-10-05; the decisions are recorded in
`semantics-v2-actor-handoff`, which must be done first (it adds `act`,
`actor` and `actorControls`). Standard constraints apply.

## The change

1. **Instruction helpers lose their agent parameter.** Every helper under
   `plugins_v2/builtin/macros/` that takes an `agent` (or a `who`/performer
   parameter that fills an agent field) drops it and writes `actor` in that
   slot: `draw(2)`, `scry(1)`, `may(body)`, `choose(a(…))`,
   `createToken(creatureToken(…))`, `sacrifice(a(creature))`, `loseLife(2)`,
   `mill(3)`, `revealHand`. The model's agent fields stay (they are deleted in
   `semantics-v2-drop-agent-fields`); only the RON spelling changes.
   *Corrected 2026-10-05 at landing:* not every helper. `changeLife` keeps
   its agent for extort (STOP 2 of the landing record), and the
   optional-agent helpers `enact`, `insertPart`, `returnTo`,
   `returnToBattlefield`, `meldInto` and `exileWithCounters` keep theirs
   (Lean `Option NounPhrase := none`, "no recorded performer").
2. **Keyword action declarations lose `agent:` and their performer
   parameters.** The loader (`crates/deckmaste_semantics_v2/src/keywords.rs`,
   `keyword_action_body`) writes `Enact(Action(label), body, actor)` for every
   action that has a wrapper, discard included (owner: "yeah i guess this is
   right, albeit a tad redundant"). The meta-macro
   `macros/meta/KeywordAction.ron` drops its `agent` parameter. Bodies say
   `actor` where they said `you` (`actorControls` beside `youControl`,
   `handOf(actor)`); `you` keeps meaning the controller [CR#109.5].
   `adapt` and `monstrosity` keep their implicit `thisPermanent`;
   `explore(permanent)` keeps its permanent parameter.
   *Corrected 2026-10-05 at landing:* the loader records the actor on every
   wrapped action except twelve, whose declarations keep `agent: None`
   because the Lean deed table gives them no player agent (STOP 1;
   `semantics-v2-deed-performer-roles`). The loader accepts that one value
   and refuses any other, so `KeywordAction.ron` keeps its `agent`
   parameter (elidable) for them.
3. **`exileBy` and the agent on `returnToHand` retire into the plain forms.**
   The eight files that call `exileBy` (ingest, scavenge, myriad, embalm,
   eternalize, recover, unearth, forage) write `exile(…)`, under `act` where
   another player exiles; `returnToHand` loses its `agent` parameter.
   *Corrected 2026-10-05 at landing:* `exileBy` was replaced by
   `exileFrom(subject, from)`, the exile with its origin stated, which
   embalm, eternalize, scavenge and forage write; ingest, myriad, recover
   and unearth write `exile(…)` (ingest under `act(they, …)`).
4. **Every keyword body and canon card is re-spelled**, reaching the owner's
   approved bodies for Thoughtseize, Burglar Rat and `amass`
   (`semantics-v2-actor-handoff`, "Target bodies"). A card where another
   player performs writes `act(player, …)`.
5. **Lean macros** that mirror these helpers (`lean/Semantics/Macros.lean`)
   follow where a RON helper is ported from one; the Lean bench itself is not
   re-spelled here.
   *At landing (2026-10-05):* no Lean file changed; no RON helper was ported
   from a Lean macro.

## Left by `semantics-v2-actor-handoff` (2026-10-05)

- **Cards already on the actor.** Thoughtseize, Burglar Rat and Hymn to
  Tourach are spelled over `act`, and the helpers still taking an agent
  there already hold `actor` (`choose(actor, …)`, `loseLife(2, actor)` in
  Thoughtseize). `amass` writes `agent: actor` and `actor` in its
  `createToken` and `choose` slots. These need only their agent arguments
  removed. `theirHand`, `revealHand`, `act`, `actor` and `actorControls`
  exist.
- **The discard wrapper gains the actor and the actor's hand.**
  `keyword_actions/discard.ron` still has no agent and a bare hand
  (`move(Param(0), hand, graveyard)`, so the loader writes
  `Enact(Action("Discard"), …, None)`), and its comment still says "the
  deed takes no agent", which the 2026-10-05 ruling withdrew. The Lean
  `Actor.discard` writes the actor in both places:
  `.enact (.action "Discard") (.move subject (.zone .hand (.possessedBy
  .actor)) graveyard []) (agent := some .actor)` [CR#701.9a]. This landing
  makes the RON declaration match it.
- **`card`, not `isCard`: decided 2026-10-05.** The approved bodies write
  `a(card)`. The owner chose the rename ("your recs are fine"): the alias of
  `Predicate.IsCard` is `card` in RON, named apart from Lean's `isCard`
  (§11 of `docs/decisions/semantics-v2.md`), and every caller writes
  `card`; no second alias is added.
- **No `army` predicate: decided 2026-10-05.** `army` stays a subtype, and
  `amass` keeps `and([hasSubtype(army), creature, actorControls])` (Lean
  `Actor.army`); the owner declined a predicate named `army` beside the
  subtype declaration ("your recs are fine").
- **ADR sentences that become false here.** In
  `docs/decisions/semantics-v2.md` §11, "a keyword action's instruction and
  its `agent`" (the declarations lose `agent:`); re-check "a file writes
  `keyword_params` or `deed: None` where its definition does not take the
  derived arguments or deed" against the five keyword actions that still
  write `deed: None` (`create`, `reveal`, `search`, `shuffle`, `vote`) and
  `macros/meta/KeywordAction.ron`. §12.1's alias example `draw(amount: …,
  agent: …)` describes the constructor alias, which keeps its `agent` until
  `semantics-v2-drop-agent-fields`; it stays true here. The `Actor` bucket
  of §12.1's Lean-only list shrinks as RON helpers take these spellings;
  recount it with the method stated there. `lean/README.md` ("required
  player slots default to `.you`") is about the explicit-agent Lean macros
  and stays true while they exist.

## Proof (orchestrator's call)

`cargo xtask expansions` before and after cannot be byte-identical, because
`You` becomes the actor in agent slots. So:

- For every declaration and card that contains no handoff, the expansion after
  must equal the expansion before once the actor is mapped back to `You`.
  The mapping is a one-off comparison run from gitignored scratch; it is
  plan-scoped verifier code and does not enter `crates/`.
- Cards with a handoff are proven by `cargo xtask lean-check` alone.
- List in the landing record every declaration or card whose expansion differs
  after the mapping, with the reason; an unexplained difference is a STOP.
- Every helper parameter removed or reordered is listed by helper.

## Out of scope

- Deleting the agent fields from the model (`semantics-v2-drop-agent-fields`).
- New helpers (`plugins-v2-keyword-helper-additions`) and the gift variants
  (`plugins-v2-gift-variants`), which are written in this spelling after it
  lands.

## Landing record

The series, oldest first, on the claim `ovynlxqprqyz`:

- S1 `zsomyzszxnrm`: keyword actions take no performer; the wrapper records
  the actor.
- S2 `uplqsovvkmrz`: instruction helpers default no performer.
- S3 `qpuuwpqpmnkt`: no instruction helper takes a performer.
- S4 `rswkmksoxrly`: `docs/decisions/semantics-v2.md` §11 and §12.1 after
  the re-spelling.
- S5 `ppmqozskrkky`: the card predicate is spelled `card`.
- S6 `rotrspuuvwro`: life changes by another player go through the handoff;
  the "Left by" list in `semantics-v2-drop-agent-fields`.

Each stage was gated before its commit. After S5, a rustfmt fix to
`crates/deckmaste_semantics_v2/src/keywords.rs` was squashed into S1 and S2–S5
were rebased onto it; the rebased tip was gated again. This record and the two
finding tickets are written on top of S6 and change only `docs/tickets/`. No
file under `lean/` changed in the series.

**Proof.**

- Per-stage gate, identical at every stage (S1–S6):

  | check | result |
  |---|---|
  | `cargo xtask lean-check`, `plugins_v2/canon` | 122/122 cards prove `Card.check = []` |
  | `cargo xtask lean-check`, `plugins_v2/testing` | 3/3 at S1–S2; 4/4 from S3 (Endure Handoff Probe added) |
  | `cargo xtask facts check` | up to date |
  | `cargo xtask gate --changed`, run with `--no-fail-fast` | 96 binaries, 1244 passed, 0 failed, 1 ignored (ignored before this landing) |
  | `cargo xtask cite check --list-noncompliant` / `cite check` | 0 non-compliant / 0 stale |

  The derived command: `cargo test -p deckmaste_construction_core -p
  deckmaste_lexical_model -p deckmaste_construction_v3_core -p
  deckmaste_lexical -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
- Classified term diff (`cargo xtask expansions` and the card dumps, compared
  by a session-only scratch classifier that is not tracked). A term is
  *unchanged* if byte-identical; *identity class* if it is identical once
  these expected changes are normalised: (a) `Actor` where `You` stood in a
  performer slot; (b) a keyword action wrapper's agent `None` →
  `Some(Actor)`; (c) discard's origin, a bare hand → the actor's hand
  [CR#701.9a]; (d) proliferate's `Choose` agent `None` → `Some(Actor)`; (e)
  the parameter lists of the changed helpers and actions. *Handoff class*:
  `X(…, agent: p)` became `Act(p, X(…, agent: Actor))`, so the term is
  proven by `lean-check`, not by the comparison. Anything else is
  *unexplained*, a STOP.
  - Baseline (claim) → S5, declarations: 1501 after (1502 before), of which
    48 are skipped by `expansions` both times (the same 48: 24 bodyless, 9
    meta-macros, 10 type declarations, 5 with no readable sample) and 1453
    printed: 1333 unchanged, 109 identity class, 9 handoff class, 0
    unexplained; added `exileFrom`, `card`; removed `exileBy`,
    `revealTheirHand`, `isCard`.
  - Baseline → S5, cards (canon and testing dumps): 126: 52 unchanged, 61
    identity class, 12 handoff class, 0 unexplained, 1 added (Endure Handoff
    Probe).
  - S5 → S6: declarations all byte-identical; cards 124 byte-identical
    (Eumidian Terrabotanist's source moved from `changeLife(up(1), actor)`
    to `gainLife(1)`, the same term), 2 handoff class (Damocles Base, Sword
    of Kang; Ominous Harvest), 0 unexplained.
- Handoff-class declarations and the card that proves each:

  | declaration | performer handed to | proved by |
  |---|---|---|
  | afflict | `defendingPlayer` | Khenra Eternal |
  | annihilator | `defendingPlayer` | Eldrazi Conscription |
  | demonstrate | `the(chosenPlayer)`, twice | Incarnation Technique |
  | evoke | `controllerOf(thisPermanent)` | Ingot Chewer |
  | ingest | `they` | Culling Drone |
  | madness | `ownerOf(this)` | Arrogant Wurm |
  | ward | `controllerOf(that(Stack))` | Rimeshield Frost Giant |
  | explore | `controllerOf(Param(0))`, one handoff over the whole body | Deadeye Tracker |
  | endure | `controllerOf(Param(0))` | Endure Handoff Probe (testing) |

  Handoff-class cards, all proving under `lean-check`: Arrogant Wurm, Cirdan
  the Shipwright, Culling Drone, Damocles Base, Sword of Kang, Deadeye
  Tracker, Eldrazi Conscription, Incarnation Technique, Ingot Chewer, Khenra
  Eternal, Moment of Silence, Ominous Harvest, Rimeshield Frost Giant.
- The `card` rename (S5) alone: all 126 card dumps byte-identical before and
  after; in the expansions only the file name `isCard.expanded` →
  `card.expanded` and its index line changed.
- rustfmt: the changed lines are clean. The pre-existing formatting diffs in
  `macro_def.rs` and `corpus.rs` lie outside the changed lines and were left.

What is proven and what is not: `lean-check` proves the checker accepts
every canon and testing card after each stage; the classifier shows every
other term difference falls in a named class. That a handoff-class term
means what the old term meant is inferred from the rule texts, not proven.

**Helper and action signatures.**
- S2, the defaulted agent dropped: `draw`, `flipCoins`, `gainControl`,
  `gainLife`, `loseLife`, `setLife`, `lookAt`, `lookAtHandOf`, `put`,
  `revealCards`, `rollDice`, `searchLibraryFor`, `searchLibraryOrGraveyard`,
  `searchTheirLibraryFor`, `searchZonesOf`, `shuffleInto`,
  `voteStartingWith`.
- S3, the performer dropped: `choose`, `may`, `doUnless`, `createToken`,
  `createTappedAttacking`, `copySpell`, `payLife`, `returnToHand`; the
  constructor aliases `addMana`, `conclude`, `copy`, `createObject`,
  `expose`, `pay`, `rerollStored`, `separateIntoPiles`, `skipPart` lost their
  trailing agent. `exileBy` deleted, replaced by `exileFrom(subject, from)`;
  `revealTheirHand` deleted (no callers).
- Kept: `changeLife(delta, agent)` (extort, STOP 2); the optional-agent
  helpers `enact`, `insertPart`, `returnTo`, `returnToBattlefield`,
  `meldInto`, `exileWithCounters` (Lean `Option := none`, "no recorded
  performer").
- Keyword actions: airbend `[Subject, Subject]` → `[Subject]`; clash
  `[Subject]` → `[]`; create → `[Amount, TokenSpec]`;
  faceAVillainousChoice → `[Instruction, Instruction]`; forage → `[]`;
  investigate → `[]`; mill → `[Amount]`; recruit → `[]`; reveal →
  `[Subject]`; sacrifice → `[Subject]`; search → three parameters; shuffle →
  `[]`; suspect → `[Subject]`; vote → `[Disclosure, Ballot]`. `agent:`
  dropped from amass, behold, blight. Bodies now saying `actor`: behold,
  blight, bolster, earthbend, populate, timeTravel, recruit, forage, clash;
  discard moves from `handOf(actor)`; proliferate's chooser is the actor.

**Tests.** Restored: 0. Re-spelled: 5, each keeping its subject and its
value comparison: `deckmaste_semantics_v2/src/keywords.rs`
`a_keyword_action_enacts_its_deed`; `xtask/tests/plugins_v2_declarations.rs`
`a_keyword_declaration_builds_its_wrapper` and
`a_turn_part_declaration_may_name_its_own_constructor`;
`deckmaste_semantics_v2/tests/reader.rs`
`the_create_token_and_add_subtype_helpers_expand_to_their_basis_terms`;
`deckmaste_semantics_v2/tests/corpus.rs` `every_ported_alias_expands` (its
`choose` and `createToken` entries). Added: assertions for `agent: None`
and for the refusal of any other agent value (`keywords.rs`), and heal's
`agent: None` (the declarations test); one testing card. Ignored: 0.
Removed: 0. `ALLOWED_RAW` in `keyword_bodies.rs` is unchanged.

**Deviations and additions.**
1. The loader still accepts an `agent` field, with the single value `None`,
   and refuses any other; twelve declarations write it and the meta-macro
   keeps the parameter (STOP 1).
2. The `changeLife` alias keeps its agent parameter; extort is its only
   caller in `plugins_v2/` (STOP 2).
3. The optional-agent helpers keep their agent (`enact`, `insertPart`,
   `returnTo`, `returnToBattlefield`, `meldInto`, `exileWithCounters`), and
   their callers that pass one are listed in
   `semantics-v2-drop-agent-fields`.
4. `exileFrom(subject, from)` is a new helper, the orchestrator's name,
   flagged to the owner: `exile` takes no origin, and four callers of the old
   `exileBy` state one.
5. A testing card, Endure Handoff Probe ("{T}: This creature endures 1."),
   proves endure's handoff; no canon card endures.
6. The gate widened beyond the ticket's crates because
   `deckmaste_construction_core` changed (a doc comment on the
   keyword-action meta signature in `macro_def.rs`), and its reverse
   dependencies were run.
7. The rustfmt fix to `keywords.rs` was squashed into S1 after S5 and S2–S5
   rebased; the tip was gated again.
8. The proof normalised four expected-change classes beyond the ticket's
   "actor mapped back to `You`": (b) to (e) above.
9. Merge opportunities seen and not taken, under the mechanical rule "one
   `act` per call": Damocles Base's inner `act(T, …)` inside an outer
   handoff to the same player; endure; demonstrate's two consecutive
   handoffs to the chosen player; Ominous Harvest's `act(target, draw(1))`
   followed by `act(they, loseLife(1))`.
10. ADR edits (S4, S5): §11's keyword-declaration sentence; §11's named-apart
    list ("Five …" → "Seven …": `aRandom`, `card` (Lean `isCard`), `random`,
    `revealHand`, `exileFrom`, `selectRandom`, `theirHand`); §12.1's
    shuffle/vote sentence, the `Actor` bucket sentence, a recount (315 → 314
    declarations, 107 → 108 Lean-only, eight buckets, the new bucket "Its RON
    helper was retired for a handoff (1): `revealTheirHand`"), and the alias
    paragraph.

**STOPs.** Two, each resolved by keeping the old term and ticketing the
cause.
1. *The checker refused a faithful term.* With `Enact(…, Some(Actor))` on
   every wrapped action, ten canon cards failed with
   `[Semantics.Refusal.enactAgentOk]`: Aeromunculus, Cached Defenses,
   Counterspell, Deadeye Tracker, Gluttonous Cyclops, Graf Rats, Ice Out,
   Inaction Injunction, Rimeshield Frost Giant, Wake the Reflections. The
   Lean deed table (`actFacts`) gives twelve deeds no player agent: adapt,
   bolster, counter, detain, endure, explore, harness, heal, meld,
   monstrosity, populate, timeTravel. Resolution: those twelve declarations
   write `agent: None` with a comment, their terms unchanged; every other
   wrapped action records `Some(Actor)`. This departs from the owner's
   decision that every named action records its actor; owned by
   `semantics-v2-deed-performer-roles`.
2. *A group handoff publishes no total.* Extort written
   `act(each(opponent), loseLife(1))` then `gainLife(thatMuch)` is refused
   `[Semantics.Refusal.quantOutcomeInScope 0]` (canon card Syndic of
   Tithes) [CR#702.101a]. Resolution: extort keeps
   `changeLife(down(1), each(opponent))`, its old term, so `changeLife`
   keeps its agent. Owned by `semantics-v2-group-handoff-publishes-no-outcome`.

**Owner decisions** (2026-10-05, owner: "your recs are fine"): the alias of
`Predicate.IsCard` is spelled `card`; no `army` predicate; keyword action
bodies are read in their own reference scope (ticketed in
`semantics-v2-keyword-body-reference-scope`, not implemented). Flagged to
the owner, not decided: the name `exileFrom`.

**Glossary.** No new term; **Actor** and **Handoff** in
`docs/contexts/game-model/CONTEXT.md` cover this landing.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed, so `coverage` was not
run. Lean pins and the Lean bench: no Lean file changed.

**Routed.**
- `semantics-v2-deed-performer-roles` (new): give the twelve deeds a player
  performer, then remove `agent: None` from them, the meta-macro and the
  loader.
- `semantics-v2-group-handoff-publishes-no-outcome` (new): let "that much"
  read the total a group handoff produced, so extort can hand off and
  `changeLife` lose its agent.
- `semantics-v2-drop-agent-fields`: now needs both tickets above; its "Left
  by" section lists every performer still written in `plugins_v2/`.
- `semantics-v2-keyword-body-reference-scope`: decided, open for work.

**After the record (S7).** A review found that the mechanical rule gave ward
the wrong counterer: `act(controllerOf(that(Stack)), doUnless(counter(…),
cost))` handed the counter to the player who may pay. Ward now reads
`act(controllerOf(that(Stack)), doUnless(act(you, counter(that(Stack))),
Param(0)))` [CR#702.21a]; the term gains `Act(You, …)` around the counter, a
deliberate change outside the identity and handoff classes (only
`keyword_abilities/ward` and Rimeshield Frost Giant changed). Every handoff
this landing introduced (9 declarations, 13 cards, the 2 calls S6 added) was
re-read against its printed text: 24 checked, 1 fixed (ward); every other
unwritten performer inside a handed-off body is that player's (afflict,
annihilator, demonstrate, evoke, ingest, madness, endure, explore; Arrogant
Wurm, Cirdan the Shipwright, Culling Drone, Damocles Base, Deadeye Tracker,
Eldrazi Conscription, Incarnation Technique, Ingot Chewer, Khenra Eternal,
Moment of Silence, Ominous Harvest twice, Endure Handoff Probe; Damocles
already hands "you draw two cards" back with `act(you, …)`). §12.1's alias
paragraph was corrected (the example is now `clearDamage`, the nine
agentless aliases are named as departing from Lean's, and seven helpers keep
an agent parameter: `changeLife` and six optional ones). Gate on the S7 tree:
`cargo xtask lean-check` canon 122/122, testing 4/4; `cargo xtask facts
check` up to date; the derived nine-crate `cargo test … --no-fail-fast`: 96
binaries, 1244 passed, 0 failed, 1 ignored; cite check 0 noncompliant, 0
stale.

**After the refresh.** The default line gained "Intern lexical identities with
dpsi Ident" between the measurements above and integration. On the refreshed
tip `ryoukzuxsrou`: `cargo xtask lean-check` canon 122/122, testing 4/4; the
nine-crate gate command above with `--no-fail-fast`: 96 binaries, 1245
passed, 0 failed, 1 ignored (one test more than before, from the lexical
landing); `cargo xtask cite check` 0 stale, 0 non-compliant; the ticket graph
check passes. `cargo xtask gate --changed` now derives the narrower six-crate
command; the wider one was run.
