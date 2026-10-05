---
needs: []
---
**Twelve keyword actions cannot record their actor, because the Lean deed
table gives them no player performer.** Found at
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

## The work

1. Lean: give these deeds a player agent, either by giving their `actFacts`
   rows a role that admits a player (`playerAgent`, or an `.either` sort as
   `coreDeedFacts`' attack row has, `Words.lean:228`), or by making
   `enactAgentOk` accept `actor` as the recorded agent. The rows are also
   read for event and deontic clauses (`deedFits`, `Events.lean:126`;
   "can't be countered" pins in `Proofs/Deontic.lean` read Counter's row), so
   a row change must keep those pins at their outcomes and must not refuse a
   permanent subject where a card has one ("whenever a creature you control
   explores").
2. Re-run `cargo xtask lean-check` with `Some(Actor)` on all twelve.
3. Remove `agent: None` from the twelve declarations, from
   `macros/meta/KeywordAction.ron` and from `keyword_action_body`, so the
   loader accepts no `agent` field at all; re-spell the `keywords.rs` and
   `plugins_v2_declarations.rs` assertions that pin `agent: None` (adapt,
   heal) against the new shape.

## Proof

- The ten cards above prove `Card.check = []` with the actor recorded, and
  Endure Handoff Probe with it; the Lean pins keep their outcomes.
- The term classifier's class (b), a keyword action wrapper's agent `None`
  → `Some(Actor)`, then covers every wrapped action: no declaration's
  wrapper records `None`.

## Open points (not decided)

- Whether a permanent-performed deed (explore, endure) should record the
  permanent as well as the player. Today its body is handed to
  `controllerOf(Param(0))` inside the wrapper, while the wrapper would
  record the outer actor, which is a different player when the permanent is
  another player's.
- Ward was re-spelled in `plugins-v2-implicit-actor-spelling` (S7) as
  `act(controllerOf(that(Stack)), doUnless(act(you, counter(that(Stack))),
  Param(0)))` [CR#702.21a], so once counter records the actor, the recorded
  counterer is the ward ability's controller. What remains: whether
  `doUnless` should take the payer and the doer as two parts of one shape
  rather than a handoff nested in a handoff.
