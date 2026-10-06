---
needs: []
---
**Twelve keyword actions cannot record their performer, because the Lean
deed table gives them no player performer and the enacted check admits no
other.** Found at
`plugins-v2-implicit-actor-spelling` (2026-10-05). Standard constraints
apply.

## The decision this blocks

The owner decided on 2026-10-05 that every named action records its actor,
discard included: "yeah i guess this is right, albeit a tad redundant"
(`semantics-v2-actor-handoff`, design brief §1.7). The keyword-action loader
(`crates/deckmaste_semantics_v2/src/keywords.rs`, `keyword_action_body`)
therefore writes `Enact(verb: Action(<label>), instruction: <body>, agent:
Some(Actor))` for every action with a wrapper. Twelve cannot take it.

## Evidence

- With `Some(Actor)` on every wrapped action, ten canon cards were refused
  `[Semantics.Refusal.enactAgentOk]`: Aeromunculus (adapt), Cached Defenses
  (bolster), Counterspell and Ice Out (counter), Deadeye Tracker (explore),
  Gluttonous Cyclops (monstrosity), Graf Rats (meld), Inaction Injunction
  (detain), Rimeshield Frost Giant (counter, through ward), Wake the
  Reflections (populate).
- The rule: `enactAgentOk` (`lean/Semantics/Check/Abilities.lean:350`)
  accepts a recorded agent only where `deedKindOk v .agent .player`
  (`lean/Semantics/Check/Events.lean:72`) holds, that is, where the deed's
  `agentRole` sort admits a player. `Instruction.check` applies it to
  `.enact v e subj` (`lean/Semantics/Check/AbilityRules.lean:330`).
- The table: `actFacts` (`lean/Semantics/Check/Words.lean:282`). Ten rows
  are `{}`, so `agentRole` defaults to `noRole` (`Words.lean:171`, no
  sort): Adapt (`:286`), Bolster (`:293`), Detain (`:311`), Endure (`:318`),
  Explore (`:324`), Harness (`:330`), Heal (`:331`), Monstrosity (`:341`),
  Populate (`:345`), Time Travel (`:369`). Meld (`:337`) sets only `dest` and
  `patientRole`. Counter (`:303`) has `agentRole := ⟨none, true, some
  .stack⟩`: no sort, a stack-zone agent (the countering spell or ability).
  A player agent is `playerAgent` (`Words.lean:211`).
- The workaround, at that landing: the twelve declarations write `agent:
  None` with a comment, and their terms are unchanged:
  `plugins_v2/builtin/macros/keyword_actions/` `adapt.ron:14`,
  `bolster.ron:18`, `counter.ron:14`, `detain.ron:16`, `endure.ron:14`,
  `explore.ron:19`, `harness.ron:16`, `heal.ron:14`, `meld.ron:15`,
  `monstrosity.ron:17`, `populate.ron:17`, `timeTravel.ron:19`. The loader
  accepts an `agent` field with the single value `None` and refuses any
  other; `macros/meta/KeywordAction.ron` keeps an elidable `agent` parameter
  for them, and `crates/deckmaste_construction_core/src/macro_def.rs`
  (`DiagnosticKeywordAction`) documents it.
- Endure, harness, heal and time travel appear in no canon card; endure is
  exercised by the testing card Endure Handoff Probe. That those four would
  be refused is inferred from their rows, not observed.

## What the rules say about who performs each

Read from `data/rules/cr.txt`. Where a rule names no performer, the
instruction is followed by the controller of the spell or ability
[CR#608.2c]: "The controller of the spell or ability follows its
instructions in the order written."

**A player performs it, by the rule's own words.**
- bolster [CR#701.39a]: "“Bolster N” means “Choose a creature you control
  with the least toughness or tied for least toughness among creatures you
  control. Put N +1/+1 counters on that creature.”" The chooser is "you".
- populate [CR#701.36a]: "To populate means to choose a creature token you
  control and create a token that’s a copy of that creature token."
- time travel [CR#701.56a]: "To time travel means to choose any number of
  permanents you control with one or more time counters on them and/or
  suspended cards you own in exile …"
- meld [CR#701.42c]: "If an effect instructs a player to meld objects that
  can’t be melded, they stay in their current zone." The defining rule
  [CR#701.42a] is imperative and names none: "To meld the two cards in a
  meld pair, put them onto the battlefield with their back faces up and
  combined."
- monstrosity [CR#701.37c]: "If a permanent’s ability instructs a player to
  “monstrosity X,” other abilities of that permanent may also refer to X."
  The defining rule [CR#701.37a] names none: "“Monstrosity N” means “If this
  permanent isn’t monstrous, put N +1/+1 counters on it and it becomes
  monstrous.”"

**A permanent performs it, and its controller does the steps.**
- explore [CR#701.44a]: "Certain spells and abilities instruct a permanent
  to explore. To do so, that permanent’s controller reveals the top card of
  their library."
- endure [CR#701.63a]: "Certain abilities instruct a permanent to endure N.
  To do so, that permanent’s controller creates an N/N white Spirit creature
  token unless they put N +1/+1 counters on that permanent."

**The rule names no performer.**
- adapt [CR#701.46a]: "“Adapt N” means “If this permanent has no +1/+1
  counters on it, put N +1/+1 counters on it.”"
- harness [CR#701.64a]: "“Harness [this permanent]” means “If this permanent
  isn’t harnessed, it becomes harnessed.”"
- counter [CR#701.6a]: "To counter a spell or ability means to cancel it,
  removing it from the stack." The table's stack-zone agent reads "a spell
  that counters"; no rule names a player who counters.
- detain [CR#701.35a]: "Certain spells and abilities can detain a permanent.
  Until the next turn of the controller of that spell or ability, that
  permanent can’t attack or block …" The spell or ability detains.
- heal [CR#701.69a]: "To heal damage already dealt to a permanent, remove
  that marked damage from that permanent. If an effect states that damage
  already dealt to a permanent “is healed,” that permanent’s controller
  removes all marked damage from that permanent." The imperative names
  none; the passive names the healed permanent's controller, which is not a
  permanent performing the deed.

## Decided (owner, 2026-10-05)

The twelve record a performer, split by who the rule says performs the deed:

- **The actor (a player)** on bolster, populate, time travel, meld, counter,
  detain and heal: the wrapper records `Some(Actor)` like every other wrapped
  action.
- **The permanent itself** on explore, endure, adapt, harness and
  monstrosity, since "whenever a creature you control explores" is about the
  creature.

Owner: "sure".

## Finding: an object performer has a role value but no enacted check (2026-10-05)

Read on this ticket's tree before writing the work below:

- A `DeedRole` whose sort is an object already exists as a value: block's
  agent (`coreDeedFacts`, `Words.lean:231`), and Crew, Saddle and Phasing in
  `abilityDeedFacts` (`Words.lean:263`) write `⟨some ⟨.object, [], …⟩, true,
  some .battlefield⟩`; attack's is `.either`. There is no named constant for
  it beside `playerAgent` (`Words.lean:211`); `fieldObject` is the
  patient-shaped one (not bare).
- What is missing is the enacted side. `enactAgentOk`
  (`Check/Abilities.lean:350`) is `deedKindOk v .agent .player` for any
  recorded subject, so a row that admits only objects refuses every recorded
  agent, and the `.enact` arm of `Instruction.check`
  (`Check/AbilityRules.lean:328`) checks the subject as a player
  (`OptNoun.check (some .player) bs subj`) and builds the body's context with
  `agentCtx`.

## Decided (owner, 2026-10-05): a permanent can be handed an instruction

The rules instruct the permanent itself: [CR#701.44a] "Certain spells and
abilities instruct a permanent to explore. To do so, that permanent’s
controller reveals the top card of their library." [CR#701.63a] "Certain
abilities instruct a permanent to endure N. To do so, that permanent’s
controller creates an N/N white Spirit creature token unless they put N
+1/+1 counters on that permanent." Owner: "CR literally says the permanent is
instructed to explore right? I guess we're doing that. whole hog it is".

- **The handoff accepts a permanent as well as a player.** "Target creature
  explores" is `act(target(creature), explore)`; "this creature endures 1" is
  `act(thisPermanent, endure(1))`. The deed wrapper records that permanent as
  the performer.
- **Inside a handoff to a permanent** (owner, 2026-10-05, correcting the
  orchestrator's first reading: "wouldn't it be act(controllerOf(actor),
  reveal(...)) or something?"): `actor` is whatever the instruction was
  handed to, here the permanent itself. The steps the rule gives to "that
  permanent’s controller" [CR#701.44a] are handed on in the body:
  `act(controllerOf(actor), sequentially([reveal(librarySlice(Top, 1,
  actor)), …]))`, where inside that inner handoff `actor` is the controller
  and the permanent stays in view for `that(Creature)` / `it`, as any handoff
  leaves its performer in view. So explore's body takes no parameter:
  `act(controllerOf(Param(0)), …)` becomes `act(controllerOf(actor), …)`
  and `Param(0)` becomes a reference to the permanent. The Lean prototype of
  this landing confirms that `actor` may denote an object and that the
  nested handoff resolves this way; the landing record says which way the
  checker was built.
- **With no enclosing handoff to an object** (orchestrator's reading, flagged
  the same way): printed "Adapt N" and "Monstrosity N" on a card's own
  ability, and any object-performed deed written with no enclosing handoff to
  an object, default their performer to the source permanent (`this`), the
  way player-performed deeds default to the controller.

## The work

1. **Lean, the object role (first).** Give the explore, endure, adapt,
   harness and monstrosity rows of `actFacts` an object agent role (bare, on
   the battlefield, as the rows above write it; a named constant beside
   `playerAgent` if it reads better), and make `enactAgentOk` test the
   recorded performer's own kind against the row (`deedKindOk v .agent <the
   performer's kind>`) instead of a fixed `.player`.
2. **Lean, the handoff.** The `act` constructor's player field becomes a noun
   phrase of either kind, with the checker rule stating which kinds it
   accepts (today `.act who body` checks `who` as a player,
   `Check/AbilityRules.lean`). Build the two readings above (`actor` and the
   permanent in view inside an object handoff; the source permanent as the
   default object performer) and pin each.
3. **Lean, the player rows.** Give the bolster, populate, time travel, meld,
   counter, detain and heal rows `playerAgent`. Counter's row is
   `⟨none, true, some .stack⟩` today. The rows are also read for event and
   deontic clauses (`deedFits`, `Events.lean:126`; "can't be countered" pins
   in `Proofs/Deontic.lean` read Counter's row), so a row change must keep
   those pins at their outcomes, and the five object rows must accept the
   permanent subject a card writes ("whenever a creature you control
   explores").
4. **Declarations.** `explore` loses its parameter and `endure` its first
   one; their bodies read `actor` and `it` as above. The seven player deeds
   take the ordinary `Some(Actor)` wrapper; the five record the permanent the
   handoff names (or the source permanent by default). Remove `agent: None`
   from the twelve declarations, from `macros/meta/KeywordAction.ron` and from
   `keyword_action_body`; re-spell the `keywords.rs` and
   `plugins_v2_declarations.rs` assertions that pin `agent: None` (adapt,
   heal) against the new shape.
5. **Cards.** Re-spell Deadeye Tracker and the probe cards over the handoff
   (Endure Handoff Probe is the only probe on this tree that calls `explore`
   or `endure`).
6. Re-run `cargo xtask lean-check`. `semantics-v2-drop-agent-fields` then
   deletes the wrapper's performer slot as planned, once the handoff carries
   the permanent.

## Proof

- The ten cards above prove `Card.check = []` with the performer recorded,
  and Endure Handoff Probe with it; the Lean pins keep their outcomes.
- The term classifier's class (b), a keyword action wrapper's agent `None`
  → a recorded performer, then covers every wrapped action: no declaration's
  wrapper records `None`.

## Ward: the nested form stays

Ward was re-spelled in `plugins-v2-implicit-actor-spelling` (S7) as
`act(controllerOf(that(Stack)), doUnless(act(you, counter(that(Stack))),
Param(0)))` [CR#702.21a], so once counter records the actor, the recorded
counterer is the ward ability's controller. Decided 2026-10-05: the nested
form stays. Owner: "my that's awkward, but i guess it's right...".

- Note: a `doUnless` shape taking the payer and the doer as two parts may be
  revisited.

## Landing record

The series, oldest first: `kzrywwvmyyxz` (claim), `ywowoyklkwzv` (Lean
deed table, enacted-performer rule, handoff to a permanent, default to the
source permanent, Rust mirror rename), `mlrsprtlnumv` (13 Lean pins),
`utkplvrxmpms` (loader, meta, the twelve declarations, cards, probe, two
re-spelled tests), `vrmoounmlroo` (ADR amendment, `lean/CONTRACTS.md`,
glossary, the `semantics-v2-drop-agent-fields` note), and this record. The
first two stages were one prototype, cut at the pins file: the stage-A tree
differs from the built stage-B tree only by `Proofs/Actor.lean`, so its Lean
build is the stage-B build; its Rust gate and `lean-check` were run on it.

**The rules** (all in `lean/Semantics/Check/`).
- Deed table (`Words.lean:218`, `actFacts`): `permanentAgent := ⟨some
  ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩` on Adapt, Endure,
  Explore, Harness, Monstrosity; `playerAgent` on Bolster, Counter (was
  `⟨none, true, some .stack⟩`), Detain, Heal, Meld, Populate, Time Travel.
- `deedAgentKind v` (`Events.lean:76`): an object where the row admits only
  objects, else a player. An event's performer is checked in that kind
  (`PhraseRules.lean`, `.verbedEvent`), so "whenever a creature you control
  explores" checks and "whenever a player explores" is refused.
- The handoff (`Abilities.lean:380`ff, `AbilityRules.lean` `.act`): `act
  performer body` checks the performer with no fixed kind and refuses
  (`Refusal.handoffPerformer`, new) anything but a player or an object whose
  zone is the battlefield [CR#110.1]. The frame's `NounShape.kind` is the
  performer's (`performerShape`); `actor` is checked as that kind
  (`PhraseRules.lean`, `.actor` arm) and `controllerOf`/`ownerOf` read their
  subject's kind through `actorView`. A handoff to `this` (`namesThis`:
  `this`, "this creature", "this permanent") puts one definite object
  binding in view under the frame (`performerInView`); `leaveHandoff` drops
  it with the frame, so it is not published.
- The enacted performer (`Abilities.lean:410`ff): `enactDefaultsToThis bs v
  s := s.isYouIn bs && !deedKindOk v .agent .player && deedKindOk v .agent
  .object`; `enactAgentOk bs subj v` accepts that default or a recorded
  performer whose kind (`actorView`) the row admits; `enactAgentKind` is the
  kind the subject is checked as; `enactCtx` checks the body under
  `actorCtx bs sourcePermanent` for the default, else `agentIntro` as
  before. `Instruction.profile`'s `.enact … (some s)` arm uses the same
  context and leaves it with `leaveHandoff`.

**What `act` accepts and what `actor` denotes.** A player (actor is that
player, as before), a group of players (each member), a permanent named by
any object phrase whose zone is the battlefield (`target(creature)`,
`thisCreature`, `thisPermanent`, `each(creature)`: actor is that permanent,
or each member), and nothing else (`handoffPerformer`). Inside `act(p, …)`
for a permanent `p`, `controllerOf(actor)` is a player, and inside an inner
`act(controllerOf(actor), …)` `actor` is that controller while "that
permanent" / "that creature" reaches `p`. Outside every handoff and inside
`act(you, …)` `actor` is the controller exactly as before.

**The two orchestrator readings.** Both held, and the checker was built
their way. (1) Inside a handoff to a permanent, `actor` is the permanent and
the body hands the player's steps on (`okExploreHandedToTargetCreature`,
`okExploreHandedToThisCreature`, `okExploreReadsThatCreature`,
`actorInPermanentHandoffIsThePermanent`). (2) A permanent-performed deed with
no handoff to a permanent defaults to the source permanent
(`okBareAdaptDefaultsToThis`, `okBareExploreAndEndureDefaultToThis`; canon
Aeromunculus, Gluttonous Cyclops). The default applies only where the actor
is the controller: inside a handoff to another player the actor is that
player and the deed is refused (`badAdaptHandedToOpponent`), where the
reading's letter ("no enclosing handoff to an object") would have defaulted
it too.

**Proof.**
- `cd lean && ./scripts/build` (`--wfail`): Build completed successfully
  (82 jobs), on `mlrsprtlnumv` and on the prototype before the pins. No
  existing pin was edited; every one keeps its asserted outcome, the
  `Proofs/Deontic.lean` pins reading Counter's row among them.
  `Proofs/Actor.lean`: 51 → 64 theorems.
- `cargo xtask lean-check`: on `ywowoyklkwzv`, canon 127/127, testing 5/5
  (2 min 4 s); on `utkplvrxmpms`, canon 127/127, testing 6/6 (2 min 26 s). The ten
  cards the actor once failed now prove with it recorded: Aeromunculus,
  Cached Defenses, Counterspell, Deadeye Tracker, Gluttonous Cyclops, Graf
  Rats, Ice Out, Inaction Injunction, Rimeshield Frost Giant, Wake the
  Reflections; and Endure Handoff Probe and Explore Handoff Probe.
- Gate: `cargo xtask gate --changed` derived `cargo test -p
  deckmaste_construction_core -p deckmaste_construction_v3_core -p
  deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`; run with
  `--no-fail-fast` on `ywowoyklkwzv` and on `utkplvrxmpms`: 90 binaries,
  1201 passed, 0 failed, 2 ignored (both pre-existing, each naming its
  blocker). `lean_drift` 4/4.
- `cargo xtask facts check`: up to date, every stage (`agentRole` is
  hand-kept; nothing is generated from it). `cargo xtask cite check
  --list-noncompliant`: 0; `cargo xtask cite check`: 0 stale (16040);
  `cite audit --diff` read for every stage. rustfmt clean on the changed
  Rust lines.

**Term change, classified** (`cargo xtask expansions` and the scratch card
dumper, before on `kzrywwvmyyxz`, after on `utkplvrxmpms`; 1463
declarations both sides, 0 failed).
- (b) wrapper `agent: None` → `Some(Actor)`: declarations adapt, bolster,
  counter, detain, harness, heal, meld, monstrosity, populate, timeTravel,
  and ward through counter; card terms Aeromunculus, Cached Defenses,
  Counterspell, Gluttonous Cyclops, Graf Rats, Ice Out, Inaction
  Injunction, Rimeshield Frost Giant, Wake the Reflections.
- Signature and body: `explore` (no parameter; body in full below) and
  `endure` (`[Amount]`); card terms Deadeye Tracker and Endure Handoff Probe
  (re-spelled over `act`).
- (r) the `Act` field rename `player` → `performer`, in the Rust debug form
  only (the Lean emission is positional and unchanged): declarations act,
  afflict, annihilator, demonstrate, evoke, extort, ingest, madness, ward;
  card terms Arrogant Wurm, Burglar Rat, Cirdan the Shipwright, Culling
  Drone, Damocles Base, Eldrazi Conscription, Hymn to Tourach, Incarnation
  Technique, Ingot Chewer, Khenra Eternal, Moment of Silence, Ominous
  Harvest, Syndic of Tithes, Thoughtseize, Amass Handoff Probe.
- New: Explore Handoff Probe. Unexplained: 0. Every other declaration and
  card is identical.

**The declarations.** All twelve lose `agent: None` and its comment; the
loader and meta take no `agent` field, so every one is `Enact(verb:
Action(<label>), instruction: <body>, agent: Some(Actor))`. Bolster,
populate, time travel, meld, counter, detain, heal: bodies unchanged.
Adapt, monstrosity, harness: bodies unchanged (`thisPermanent`, and
harness's `Param(0)`), since with the default the actor is the source
permanent they name; a comment says who performs them. Explore, in full:

```ron
params: [],
body: act(
    controllerOf(actor),
    sequentially([
        reveal(librarySlice(Top, 1, actor)),
        doIf(
            matches(that(Card), land),
            move(that(Card), wherever, hand),
            sequentially([
                putCounters(1, p1p1Counter, that(Permanent)),
                may(move(that(Card), wherever, graveyard)),
            ]),
        ),
    ]),
),
```

Endure, in full:

```ron
params: [Amount],
body: act(
    controllerOf(actor),
    chooseOne(
        putCounters(Param(0), p1p1Counter, that(Permanent)),
        createToken(creatureTokenOf(Param(0), Param(0), [White], [spirit])),
    ),
),
```

**Pins added** (`Proofs/Actor.lean`, "A permanent can be handed an
instruction"), each closed by `decide`: `okExploreHandedToTargetCreature`
(`[]`), `okExploreHandedToThisCreature` (`[]`), `okExploreReadsThatCreature`
(`that(Creature)` under both, `[]`), `okEndureHandedToThisCreature` (`[]`),
`okExploreHandedToEachCreature` (`[]`), `actorInPermanentHandoffIsThePermanent`
("target creature draws a card" = `[.kindMismatch .player .object]`; handed on
to `controllerOf actor`, `[]`), `badExploreHandedToOpponent`
(`[.kindMismatch .object .player, .possessorKind .controller .player, .anaphor
(.word .permanent) .one 0, .enactAgentOk]`), `badHandoffToCardInGraveyard`
(`[.handoffPerformer]`), `okBareAdaptDefaultsToThis` (`[]`),
`okBareExploreAndEndureDefaultToThis` (`[]`, `[]`), `badAdaptHandedToOpponent`
(`[.kindMismatch .object .player, .enactAgentOk]`),
`exploreEventTakesAPermanent` (a creature `[]`, a player `[.kindMismatch
.object .player]`, a player bolsters `[]`), `handedToThisPublishesOnlyTheBody`
(publishes `[(player, one, the)]`).

**Pins changed.** None.

**Tests.** Restored: 0. Re-spelled: 2, each keeping its value comparison:
`deckmaste_semantics_v2::keywords` `a_keyword_action_enacts_its_deed` (adapt
now `agent: Some(Actor)`; an `agent` field, `None` or `You`, is refused) and
`xtask/tests/plugins_v2_declarations.rs`
`a_keyword_declaration_builds_its_wrapper` (heal now `Some(Actor)`). Ignored:
0 new. Added: 13 Lean pins, one testing card. Removed: 0.

**Deviations and additions.**
1. The `Act` field is renamed `player` → `performer` in Lean, the Rust mirror
   and `act.ron` (its parameter too): it now holds a permanent, and the
   glossary rule is to rename the identifier. This is the (r) diff class.
2. The bodies read the permanent as `that(Permanent)`, the rules' own "that
   permanent" [CR#701.44a,701.63a], where the ticket wrote `that(Creature)` /
   `it`: `it` is ambiguous there (the revealed card is in view), and "that
   creature" does not reach the default source permanent, whose type is not
   known. `that(Creature)` under a handoff is pinned
   (`okExploreReadsThatCreature`).
3. Endure's whole choice is handed to the controller, not only the token,
   since "unless they put" is the controller's [CR#701.63a].
4. Beyond the ticket's letter: event performers take the row's kind
   (`deedAgentKind`), which also lets an object-only row such as block's or
   crew's be written with an object subject in a `verbedEvent`; the
   `handoffPerformer` refusal; `performerInView`/`leaveHandoff`; the scratch
   carddump copy under the session scratchpad.
5. No adapt probe was added: Aeromunculus is a canon bare "Adapt 1".

**STOPs.** None.

**Glossary.** **Actor** and **Handoff** extended to a Permanent, same form
and `_Avoid_` lines. No gap found.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed, so `coverage` was not
run.

**Routed.** `semantics-v2-drop-agent-fields`: dated line added (when the
wrapper's slot goes, `enactAgentOk`/`enactAgentKind` read the actor in
context).
