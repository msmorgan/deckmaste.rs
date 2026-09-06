import Semantics.Check.PhraseRules

namespace Semantics

def GameEvent.checkAll (bs : Bindings) (evs : List GameEvent) : List Refusal :=
  evs.flatMap (GameEvent.check bs)

def DurationEnd.check (bs : Bindings) : DurationEnd → List Refusal
  | .startOf _ w => OptNoun.check (some .player) bs w ++ refuse (durationPossessorOk w) .durationPossessor
  | .endOf _ w => OptNoun.check (some .player) bs w ++ refuse (durationPossessorOk w) .durationPossessor

def Duration.check (bs : Bindings) : Duration → List Refusal
  | .thisTurn | .restOfGame => []
  | .until_ e => DurationEnd.check bs e
  | .forAsLongAs c => Condition.check bs c
  | .untilEvent ev => GameEvent.check bs ev
  | .duringNextTurnOf who => NounPhrase.check (some .player) bs who ++ refuse who.plur.isOne .singular

def OptDuration.check (bs : Bindings) : Option Duration → List Refusal
  | none => []
  | some d => Duration.check bs d

def Timing.check (bs : Bindings) : Timing → List Refusal
  | .asSorcery | .asInstant => []
  | .duringPart p w => OptNoun.check (some .player) bs w ++ refuse (windowOk p w) .windowOk
  | .beforePart p w =>
    OptNoun.check (some .player) bs w ++ refuse p.proper .windowOk ++
      refuse (pointWindowOk w) .pointWindowOk

def OptTiming.check (bs : Bindings) : Option Timing → List Refusal
  | none => []
  | some t => Timing.check bs t

def Concurrent.check (bs : Bindings) : Concurrent → List Refusal
  | .whileTrue c => Condition.check bs c
  | .whileDoing ev => GameEvent.check bs ev ++ refuse (eventUnderwayOk ev) .eventUnderway

def OptConcurrent.check (bs : Bindings) : Option Concurrent → List Refusal
  | none => []
  | some c => Concurrent.check bs c

/-- Idris `HeaderNontarget` and `HeaderStatus` on a header event and each `or`-arm. -/
def headerEventCheck (bs : Bindings) (ev : GameEvent) : List Refusal :=
  GameEvent.check bs ev ++ refuse (ev.headerNontarget bs) .headerNontarget ++
    refuse ev.headerStatusOk .statusMarkable

def JoinedHeader.check (bs : Bindings) (j : JoinedHeader) : List Refusal :=
  headerEventCheck bs j.event ++ j.alternatives.flatMap (headerEventCheck bs) ++
    OptConcurrent.check (headerCtx bs j.alternatives j.event) j.while_ ++
    OptTiming.check bs j.timing

end Semantics
