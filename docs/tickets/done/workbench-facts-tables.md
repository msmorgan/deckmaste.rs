---
needs: []
---
**Merge the two verb-label tables into one `ActFacts` table, and turn the
hand-rolled event predicates into an `EventFacts` record table.** 2026-09-02
workbench audit (F4, F5). Open label vocabularies stay open — this merges
tables, it does not close them.

## One verb-facts table (F4, M)

`Words.VerbFacts:782` (label, participle, actPatient, actZone, actDest,
actStepwise, actLoci, actIntransitive; 21 rows at `:794-840`) and
`Events.DeedFacts:498` (deed, agent/patient `DeedRole` = kinds/types/bare/zone;
defends, targeted, counterfactual, rides, plays; 20 rows at `:510-595`)
describe one verb space twice:

- "Sacrifice" is in both (`Words:798` patient Object / Battlefield→Graveyard;
  `Events:561` agent Player, patient Object [6 types] Battlefield); so is
  "Untap" (`Words:812`, `Events:569`).
- "Search" (`Words:820`) and "SearchLibrary" (`Events:574`) name the same act.
- `Draw` is an effect constructor while "DrawCard" is a deed row.
- The gates differ: `VerbedEvent` (`Triggers:302`) uses `KnownVerb`, while
  `Deontic`/`CantBe`/`CantMoreThan` use `KnownDeed(s)` — so "whenever you
  sacrifice" and "you can't sacrifice" consult different tables.

Fix: one `ActFacts` record = the `VerbFacts` fields + `agent, patient :
DeedRole` + the five deed flags, as `Maybe`s where a verb has no deed reading.
One table, one `Known`; `deedKindOk`/`deedTypeOk`/`deedZoneOf` read the merged
rows; `actZone` and `patient.roleZone` collapse. Assert no duplicate labels at
table build — a `So (distinctLabels …)` on the list, like `distinctDeeds:681`.

## Event facts as a record table (F5, M)

`Events.lookbackSubjectOk:274-363` (89 clauses), `lookbackComplementOk:371-423`,
`bareLookbackOk:426-436`, `eventHasMagnitude:224-265`, `interceptOk:195`,
`triggerCountOk:204`, `spanEventOk:209`, `eventUnderwayOk:217`, and the
81-clause `sameEventName:96` are eight hand-rolled columns of one table over
the 41-row `EventName:9`.

Fix: `record EventFacts` (subjectKinds, complementKinds, bareOk, hasMagnitude,
interceptable, countable, spannable, underway) and one `eventFactsOf :
EventName -> EventFacts` (41 clauses), with the eight predicates as field
reads. This is the `verbFacts` idiom the module already uses next door.

Size: M.

Done when: build is 23/23; `Events.DeedFacts` and `Words.VerbFacts` are one
record with one table and one `Known` predicate; the duplicate-label assertion
elaborates; the eight event predicates are field reads over `eventFactsOf`;
`sameEventName`'s 81 clauses are gone; the deed and lookback pins in `Proofs*`
still refute and are non-vacuous. Standard constraints apply.

## As landed

- One table: `Words.ActFacts` (the `VerbFacts` fields + `agent, patient :
  DeedRole` + `actDefends`, `actTargeted`, `actCounterfactual`, `actRides`,
  `actPlays`); `actFacts` holds 38 rows — the 21 verb rows and 20 deed rows
  with "Sacrifice" and "Untap" merged and "SearchLibrary" folded into
  "Search". `DeedRole`, `noRole`, `PremiseSort` moved to `Words`;
  `Events.DeedFacts`/`deedFacts`/`deedIn`/`deedFactsFor` and
  `Words.VerbFacts`/`verbFacts`/`verbFactsFor` are gone.
- One `Known`: `KnownAct` (list form `KnownActs`) replaces `KnownVerb`,
  `KnownDeed`, `KnownDeeds` at every site (Effect, Macros, Triggers,
  ProofsAnaphora), so `VerbedEvent` and `Deontic`/`CantBe`/`CantMoreThan`
  consult the same rows.
- `deedRoleOf`, `deedKindOk`, `deedTypeOk`, `deedZoneOf` and the flag readers
  read the merged rows through `actFactsFor`; `actZone` is deleted and
  `actZoneOf v` is the patient role's `roleZone`.
- Roles are plain `DeedRole`, not `Maybe`: a verb with no deed reading carries
  `noRole` (empty kinds admit nothing), because the patient role is where the
  collapsed zone lives for verb-only rows.
- Duplicate-label assertion: `distinctActLabels` and the checked proof
  `actLabelsDistinct : So (distinctActLabels actFacts) = Oh`. Probed
  non-vacuous by duplicating the "Proliferate" row: the proof fails with
  `Mismatch between: True and False`.
- `Cards.idr`: the two `playerCant "SearchLibrary"` sites re-spelled to
  `"Search"`. `Draw` (effect) vs "DrawCard" (label) left as the ticket found
  it — no fix was named.
- `record EventFacts` (subjectKinds, complementKinds, bareRefused,
  hasMagnitude, interceptable, countable, spannable, underway) with
  `eventFactsOf` (41 clauses); `interceptOk`, `triggerCountOk`,
  `spanEventOk`, `eventUnderwayOk`, `eventHasMagnitude`,
  `lookbackSubjectOk`, `lookbackComplementOk`, `bareLookbackOk` are field
  reads (`kindIn`/`kindPairIn`/`kindAny` split `\/` uniformly). The column is
  `bareRefused` rather than `bareOk`: the original predicate is default-true
  and the `sb`-gate pins in ProofsG rely on `LeftBare` resolving for a
  non-subject kind, so a positive list breaks their elaboration.
- `sameEventName`'s 81 clauses are gone: `eventIx` (41 clauses) plus a
  3-clause `sameEventName`; `StateMatch` stays never-equal.
- Pins: none added or re-spelled; the deed and lookback `Unspellable …
  impossible` refutations in Proofs/ProofsD/ProofsF/ProofsG still elaborate,
  which is their non-vacuity. New proof name: `actLabelsDistinct`.
- Build time: `idris2 --check` Words 3.3s/2.1s (trunk, cold/warm) vs 1.3s
  (new); Events 1.6s/1.1s vs 1.5s — no slowdown from the larger table.
- `cite check --list-noncompliant` reports one pre-existing `CR`-plus-year string in
  `docs/tickets/critical/engine-cost-preflight-atomicity.md:22`, outside this
  round's diff and edit scope; `cite check` 0 stale; audit selects 0 sites
  (no citations changed).
