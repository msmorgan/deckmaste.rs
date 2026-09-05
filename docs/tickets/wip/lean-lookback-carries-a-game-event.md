---
needs: []
---
**A lookback is a game event with a gap, not a parallel event-name taxonomy.**
`lean/Semantics/Events.lean` declares `EventName`, a bare enumeration of event
classes (`death`, `departure`, `damageTaken`, …, `verbedAct deed`), and
`lean/Semantics/Phrase.lean` builds a lookback ("creature that died this turn") as
`LookbackClause.mk (event : EventName) (lookback) (complement : Option
EventComplement)`. `GameEvent.name : GameEvent → EventName` projects the
sentence form onto the class so `EventName.facts` can key the checker's event
table, and six small `eventName` projections (`CombatRelation`, `AttachMove`,
`StatusCat`, `FlipCall`, `PaymentOutcome`, `LifeMove`, plus
`counterEventName`) keep the two taxonomies aligned by hand.
`EventComplement` (`involving`, `fromZones`, `intoZone`, `atZone`) re-invents
the zone fields `GameEvent.leaves`/`enters`/`putInto` already carry.

Change: a lookback carries a `GameEvent` whose relativised participant is
marked (a `NounPhrase` gap standing for the noun the clause modifies), and the
event facts (`subjectKinds`, `complementKinds`, `bareRefused`, `hasMagnitude`,
`interceptable`, `countable`, `boundsDuration`, `underway`) become a function
of the `GameEvent` constructor. Delete `EventName`, `EventComplement`, the
seven projections and `GameEvent.name`; the lookback complement checks become
the ordinary event participant checks. The Anaphora and Zone pin suites are
the ones this reaches; every pin keeps its name and expected list under
spelling normalisation.

Decisions already made: a law reads a declared feature or matches a closed CR
constructor, never a lexeme; `TriggerWord` was deleted 2026-09-05 for the same
reason (a header word is not a model concept). Glossary entry **Event**
([CR#700.1]) already covers the concept; no new term.
