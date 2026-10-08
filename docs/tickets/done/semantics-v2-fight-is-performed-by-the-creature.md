---
needs: []
---
**Fight is performed by a creature: the first creature is the performer
and the second is the parameter.** "A spell or ability may instruct a
creature to fight another creature … Each of those creatures deals damage
equal to its power to the other creature" [CR#701.14a]. Routed from
`builtin-v2-keyword-spelling-templates-residue`, owner decision D2
(2026-10-07): the subject is never a slot, and the performer is `actor`,
never a parameter. Small. Standard constraints apply.

## Why it stopped

The residue landing tried fight as `params: [Subject]` with an `actor`
body, and Prey Upon as
`act(target(creatureYouControl), fight(target(creatureYouDontControl)))`.
`cargo xtask lean check` gave canon 133/134. Prey Upon was refused with
`[kindMismatch player object, enactAgentOk]`. The deed table row
`("fight", {})` in `lean/Semantics/Check/Words.lean` names no performer.
The semantics-v2 ADR §7, amendment 2026-10-06, records this as a ruling:
"A deed whose facts row names no performer ("fight" [CR#701.14a]) records
none, and is refused when a handoff names one."

## The work

1. In `lean/Semantics/Check/Words.lean`, `("fight", {})` becomes
   `("fight", { agentRole := permanentAgent })`. Explore, endure, adapt,
   harness and monstrosity already have that row.
2. In the semantics-v2 ADR §7, amendment 2026-10-06, the sentence that
   names "fight" as the no-performer example gets another example, or is
   reworded.
3. `plugins_v2/builtin/macros/keyword_actions/fight.ron` becomes
   `params: [Subject]`. Its body is
   `dealDamage(actor, powerOf(actor), Param(0))` and
   `dealDamage(Param(0), powerOf(Param(0)), actor)`.
4. Prey Upon and every other fight caller are re-spelled to the handoff
   shape, `act(<first>, fight(<second>))`. Today Prey Upon is the only
   caller.
5. Fight's spelling becomes `fight <Param(0)>`. Empty `PENDING_RULING` in
   `crates/deckmaste_construction_core/tests/builtin_v2_keyword_spelling_templates.rs`
   so that the test asserts all 258 keyword declarations have templates,
   and update the fight pin.

## Proof

- `cargo xtask lean check`: canon 134/134, testing 7/7.
- `cargo xtask expansions` before and after, `diff -r`: only fight and its
  callers differ.
- `cargo test -p` passes for deckmaste_construction_core,
  deckmaste_semantics_v2 and xtask.
- `cargo xtask lexical --card-name "Black Lotus" --export`: byte-identical.

## Landing record

Commits: `ktspzmvo` (Lean: the deed row, pins re-spelled, the §7 wording),
`oxnomykx` (`fight.ron`, Prey Upon, the template). Numbers measured on
`oxnomykx` after `kata refresh`. A second refresh brought english_v3 and
`lexicon/core.ron` changes (`english-v3-once-frequency`); the crate tests,
clippy and the lexicon export were re-run on it, with the same results.

**PROVE.**
- `cargo xtask lean check`: canon 134/134, testing 7/7 (unchanged; Prey
  Upon now checks as `act(target(creatureYouControl),
  fight(target(creatureYouDontControl)))`).
- `lake build --wfail`: green (87 jobs). `cargo xtask lean definitions`:
  553/553. `cargo xtask facts check`: up to date (the facts table carries
  labels only, no deed roles; nothing to generate).
- `cargo xtask expansions` before/after, `diff -r`: only
  `keyword_actions/fight.expanded` differs (`fight(You)`, `actor` as the
  first source and the second recipient, the parameter `fight.0`). No builtin
  macro calls fight; Prey Upon is the only caller.
- Tests: `cargo test -p deckmaste_construction_core` 464 passed,
  `-p deckmaste_semantics_v2` 96 passed, `-p xtask` 392 passed / 1 ignored
  (pre-existing); clippy `--tests -- -D warnings` clean for the three.
  `PENDING_RULING` is empty: all 258 keyword declarations carry templates.
- Cites: `--list-noncompliant` 0, 0 stale (16,335), `cite audit --diff` from
  the claim: 10 sites, each read against its rule text.
- `kanban check`: OK.

**DISCLOSE.**
- Fight's body: `simultaneously([dealDamage(actor, powerOf(actor),
  Param(0)), dealDamage(Param(0), powerOf(Param(0)), actor)])`, template
  `fight <Param(0)>`. Its facts row is `{ agentRole := permanentAgent }`
  [CR#701.14a]. No game-rule refusal added.
- The §7 amendment of 2026-10-06 now names "the Ring tempts you"
  [CR#701.54a] as its no-performer example, with a dated note; the same
  example replaces fight in `handedOne`'s doc comment. The spelling ADR's
  fight paragraph records the resolution.
- Assurance counts: restored 0; re-spelled 6; ignored 0; added 1; removed 0.
  Re-spelled: `Zone.badAgentedAgentlessAct` ("You fight target creature" now
  defaults to the source permanent, as bare adapt does; same claim, a player
  handed fight is refused, spelled "Target opponent fights target creature":
  `[kindMismatch object player ×3, enactAgentOk]`); `BindingIdentity`'s
  `fightByIdentity` re-spelled to the loader's new handoff shape, carrying
  `okFightByIdentity` (the twin: `act(target(creature),
  fight(target(creature)))` checks), `fightTargetsEachArgumentOnce` (2),
  `okFightByIdentityThisCreature`, and `badFightByIdentityGroup` (the group
  is now the parameter: `[perMember, singular]`); the fight template pin.
  Added: `okFightHandedToEachOfAGroup` — a group handed fight distributes it
  per member and checks, as every group handoff does; this was
  `badFightByIdentityGroup`'s old term, whose refusal belonged to the
  two-parameter body.
- Bench alignment (commit "lean bench: fight takes one creature, the actor
  fights it", at the coordinator's direction under the 2026-10-05 rule that
  the bench tracks the RON target): `Macros.lean`'s `fight` takes one
  captured creature and reads `actor` for the first, mirroring `fight.ron`;
  its creature guard is gone [CR#701.14b]. Re-spelled 19, removed 0: the
  four `Cards/Damage.lean` cards (Prey Upon, Brash Taunter, Ulvenwald
  Tracker, Savage Swipe) and `Proofs/Damage.lean`'s `okFightCreatures`,
  `badFightGroup`, `badIt`, `badInnerAmbig`, `okFightControlledCreatures`,
  `badFightGraveyard`, `okFightLand`, `okFightPermanentUnderCreatureGuard`,
  `badRestOverTwoAnnouncements`; `MacroParameters`'
  `identicalArgumentsRemainSeparateSelections` and both forged-context
  `#guard_msgs` witnesses; `ActionFamilies`' `fightAcceptsAnExistingReference`,
  `fightAcceptsItsOwnSource`, `capturePreservesTheResolvedPermanentView`.
  Every outcome holds except two refusal lists: `badFightGroup` puts the
  group in the parameter (a group performer distributes), `[perMember,
  singular]`; `badFightGraveyard` is now refused at the handoff,
  `[handoffPerformer]` [CR#110.1], where it was `[zoneFits,
  damageRecipient]`. `lean check` stays 134/134, 7/7.

**STOPs.** None.

**Glossary.** No gap.

**REPORT.** Lexicon export (`cargo xtask lexical --card-name "Black Lotus"
--export`): byte-identical between the claim `tmyzvrtw` and `oxnomykx`
(35,671,262 bytes after the second refresh). No English grammar or lexicon source changed, so the
coverage lock, selection census and performance advisory do not apply.
