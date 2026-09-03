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
