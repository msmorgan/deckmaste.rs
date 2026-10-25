---
needs: []
---
# Retire what duplicates `Deed`, and keep `Deed`

An external proposal (2026-10-25) asked to delete `Deed` and `CoreDeed`, give `Action.ownScope` a
label plus an actor flag, drop the generated `<def>.deed` constants and `elabDeedName`, and merge
`Deed` with the checker's `EventVerb` into one `ActionVerb`. It predates
[lean-v3-checker-and-drt-fixes](../done/lean-v3-checker-and-drt-fixes.md), so it was audited against
the tree at `wszxknxt` (parent `qtypooyz`). This ticket keeps only what survived the audit.

## Verdict on the proposal

`Deed` is not vestigial. It is the Game Model's **Deed** (`docs/contexts/game-model/CONTEXT.md`),
and the ADR prescribes it on `ownScope` (`docs/decisions/semantics-v3.md` §1.4, §2.1). Its identity
is the **Keyword Action** [CR#701.1], not the Definition. Several Definitions share one deed:
`exile` appears at `Definitions/Actions.lean:83,88,95,319,326`, `exchange` six times from `:294`,
and `fight` at `:190,210`. `Proofs/Authoring.lean:281` pins `searchZonesOf.deed = search.deed`, and
`Proofs/Damage.lean:71` pins `fightEachOther.deed = fight.deed`. Deleting `Deed` would lose four
things:

- **The keyword-ability deeds.** `Deed.ofAbility` covers devour [CR#702.82b]
  (`Definitions/Keywords.lean:512`), crew and saddle [CR#702.122b,702.171c]
  (`Check/Phrase.lean:191-192`), and cycling (`Proofs/CorpusRecency.lean:222`). A label-only
  header has no place for them, and `participialAgentOk` reads them (`Check/Phrase.lean:293`).
- **The Core Deeds' stamps.** Targeting, drawing and gaining control stamp referents
  (`Check/Phrase.lean:2571,3994,4542`), so "the targeted creature" and "the drawn card" read back
  [CR#115.1,121.1,613.1b].
- **The three-way `DeedPermanent`.** The proposal's `Bool` is stale. `none`, `performs` and
  `isAbout` [CR#701.44a,701.46a,701.37a,608.2c] drive four sites: the stepper's source performer
  (`Check/Phrase.lean:3632`), `causedPerformer` and `causedPatient` (`:1273-1291`), and
  `subjectIsPatient` (`:1232-1235`).
- **The authoring boundary.** The `KnownAct` contract (`lean/CONTRACTS.md:899`) depends on the
  generated constants and `elabDeedName`. A card cannot misspell a keyword action because each
  card names one through its Definition's generated constant. Live sites:
  `Definitions/Keywords.lean:1068`, `Cards/Counters.lean:385-387`, `Cards/Anaphora.lean:501-503`,
  `Proofs/Actor.lean:683,688` and `Proofs/Tables.lean:63-67`.

A per-Definition `ActionDefMeta` would split one keyword action across its Definitions, which
breaks "the exiled card" after `exileFrom`. A `KeywordActionLabel` plus a permanence field is
exactly `Deed.action` as it stands. The proposed name `ActionVerb` breaks the glossary's _Avoid_
line under **Deed**: a verb is the word, a Deed is what it names.

## Change (proposed)

1. **Retire the `EventVerb` classes that shadow a Deed.** `EventVerb` (`Check/Words.lean:169-181`,
   47 constructors) documents itself as the class of an event "that names no Deed and writes no
   Zone Change". Seven of its constructors name events that always do name a Deed:
   - `targeted`, `drew`, `triggered`, `controlChanged` and `fullyUnlocked` mirror `CoreDeed`;
   - `crewed` and `saddled` mirror `Deed.ofAbility`.

   For these heads `Action.namedDeed` (`Check/Phrase.lean:189-195`) always returns a deed, so the
   lookback's anchor is `some (.deed _)`. `CausedEvent.readBy` (`Check/Words.lean:1333-1343`)
   consults `verbs` only when the anchor is `none`, so these seven classes are never read. None of
   them is used outside `Action.eventVerbs` (`Check/Phrase.lean:1174-1225`). Delete them and their
   `eventVerbs` arms. The other 40 classes name events no Deed names (damage, life, status, combat,
   counters, mana, turn parts, payment) and stay. `CausedEvent` keeps
   `deed`/`zoneChange`/`verbs`: these are three different ways to identify an event.
2. **Delete the dead `distinctDeeds`** (`Check/Events.lean:127-129`). Its only references are its
   own recursion (`lean_references`).
3. **Pin the checker's raw deed literals.** The checker cannot import the generated constants,
   because `Definitions/Actions.lean` imports `Check/Phrase.lean`. So it spells `.action "cast"`
   and `.action "play"` (`Check/Words.lean:1338`, [CR#701.18b]), `searchDeed`
   (`Check/Words.lean:126`) and `.action "tap"` (`Check/AbilityRules.lean:24,43`) by hand. A typo,
   or a permanence that does not match, would silently match nothing, because `Deed` equality
   includes `DeedPermanent`. Add `decide` pins equating each literal with its Definition's
   generated constant (`cast.deed`, `play.deed`, `search.deed`, `tap.deed`), next to
   `deedIsGenerated` (`Proofs/Authoring.lean:281`).

No Core AST change. `Deed`, `CoreDeed` (five constructors, `Words.lean:321-323`), `DeedPermanent`,
`ownScope`, the `.deed` synthesis (`Authoring.lean:238-300`) and `elabDeedName`
(`Templates.lean:534-547`) all stay.

## Claims checked and found stale or false (no action)

- `does`, `would`, `cant` and `did` take an `Action` (`Abilities.lean:525,841,938`; `DidClause`).
  This is true, but nothing is left to retire.
- "9 of 14 `CoreDeed` constructors are dead" is stale: `xvknqmor` deleted them.
- `Action.enact` and `inferredDeed` are already gone (`mqmzkunq`; now `Action.namedDeed`).
- No Rust mirror exists: there is no `crates/deckmaste_semantics_v3`, `plugins_v3/` or
  `deckmaste_lowering_v3`. ADR §2.1 still plans to lower the header to `core::Action::Composite`
  and `core::EventFilter::Act` by `VerbName` (`crates/deckmaste_core/src/event.rs:168`), which is
  the label `Deed.action` carries.

## Overlap

[lean-v3-rules-and-card-fidelity](../wip/lean-v3-rules-and-card-fidelity.md) (in progress) has
three overlaps:

- It replaces the raw `.search`, `.shuffle`, `.exchange` and `.expose` heads in
  `Definitions/Keywords.lean` with their Definitions.
- It moves `regenerate`'s deed (`Definitions/Actions.lean:612-617`).
- It touches `Proofs/ActionFamilies.lean`.

[lean-v3-exchange-through-definitions](lean-v3-exchange-through-definitions.md) (added
2026-10-25) moves the nine card sites that write `exchange` through the deed-less Template onto
the exchange Definitions; until it lands, `exchanged` is still written by bare heads.

This ticket edits only `Check/` and `Proofs/`. After that ticket lands, re-check whether
`searched`, `shuffled`, `exchanged`, `voted` and `created` are still written anywhere as bare heads
outside a Definition. If none is, they shadow a Deed too (question 2). Neither ticket needs the
other.

## Open questions for the owner (ruled below)

1. `ownScope none` is written only in `Proofs/` (`CallerScope.lean:59-76`, `Actor.lean:187`,
   `ActionFamilies.lean:277`) and matched in four checker arms. Every Definition writes
   `some`. Should the header become a required `Deed`? That would amend ADR §1.4's
   `Option Deed := none`.
2. Should the five classes in the Overlap section go too, once no bare head writes them?
3. Should authoring refuse two Definitions that share a keyword-action label but disagree on
   `DeedPermanent`? Today the label `fight` agrees by hand.

## Owner rulings, 2026-10-25

1. **Keep `Option Deed`.** The premise was stale: `regenerate` is now a Definition with
   `.ownScope none` (rules-and-card-fidelity), since creating a regeneration shield is not
   regenerating [CR#701.19c]; only `regeneration` carries the deed. No ADR §1.4 change.
2. **Yes.** After `lean-v3-rules-and-card-fidelity` lands, delete each of `searched`, `shuffled`,
   `exchanged`, `voted` and `created` that no bare head outside a Definition still writes.
3. **Yes.** Authoring (or `Check/`) refuses two Action Definitions that share a keyword-action
   label but disagree on `DeedPermanent`, with a pin that a mismatch is refused.
