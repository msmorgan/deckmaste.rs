import Semantics.Check.Words

/-!
# Semantics.Check.Refusal

Why a spelling is refused. One constructor per obligation, named after the obligation
(`SoleHolder` → `.soleHolder`), so a pin names the obligation it refutes. A few carry the count
they found, which is what a reader wants to know for an anaphora refusal.

The checker reports every failing obligation, so `check e = [r]` states that `r` is the only
refusal, which is what a non-vacuous pin claims.
-/

namespace Semantics

inductive Refusal where
  | recursionLimit
  /-- A performed payment requires a specified cost. -/
  | concreteCost
  | concreteBallot
  /-- A negated rider (`MoveRider.not`) is named only in a pattern: an action's riders are what
  its text writes, so outside a pattern there is nothing for a negation to deny. -/
  | riderNegationPattern
  /-- A control change (`Action.changeControl`) is named only as an event, lookback or deontic
  pattern; a resolving one is `establish (controlGrant …)` [CR#611.2a,613.1b]. -/
  | controlChangePattern
  /- sorts -/
  /-- A phrase's kind is fixed by two constraints that disagree. -/
  | kindMismatch (expected found : Kind)
  | phrasal (k : Kind)
  | targetable (k : Kind)
  | targeter (k : Kind)
  | kindLte (k bound : Kind)
  | possessorKind (axis : PossessorAxis) (k : Kind)
  | projScope (k : Kind)
  | axesAt (k : Kind)
  | designationHolder (designation : DesignationLabel) (k : Kind)
  | designationScope (designation : DesignationLabel)
  | designationPossessorFits (designation : DesignationLabel)
  /- number -/
  | soleHolder
  | singular
  | plural
  | groupMention
  | partitiveBase
  | countableGroup
  | countedMention
  | slicePossessor
  | perMember
  | existentialMention
  | complementAnchor
  | choosable
  | choiceClause
  | choiceOrder
  | testSubject
  /- zones -/
  | ordinalNonZero
  /-- An event ordinal counts within a period no event ordinal names (`Period.eventOrdinalOk`). -/
  | eventOrdinalPeriod (period : Period)
  /- anaphora and scope: what the read found -/
  | anaphor (r : Reach) (pl : Plurality) (found : Nat)
  /-- A Participial Description ("the exiled card", "each creature destroyed this way", "the
  card returned this way") reads back the objects its deed or its Zone Change acted on in scope,
  and found `found` of them, not exactly one [CR#607.1,607.2a]. -/
  | participialDescription (anchor : ParticipialAnchor) (found : Nat)
  /-- A Participial Description names an agent of its deed the checker cannot read back: a
  stamp records what a deed did, not who did it, so only no agent, "you", the performer, or the
  object whose keyword ability defines the verb are read. -/
  | participialAgent
  /-- A historical clause with no span written ("exiled", not "exiled this way" or "exiled this
  turn") reads back only as a Participial Description: a condition, a tally, or any other
  relative clause names the span it looks back over. -/
  | unanchoredLookback
  /-- "that turn" reads back the one turn an earlier phrase introduced; it found `found`. -/
  | turnAntecedent (found : Nat)
  /-- A Definition or Template body's later mention of a referent parameter finds no binding recorded under its
  Binding Identity, and its phrase would introduce one: reading it again would select anew. -/
  | bindingIdentity (id : BindingId)
  | choiceRef (ref : ChoiceRef) (sort : ChoiceSort) (found : Nat)
  | choiceInScope (k : Kind)
  | outcomeInScope (s : OutcomeSort) (found : Nat)
  | quantOutcomeInScope (found : Nat)
  | coinFlipInScope
  | damageDealtInScope
  | groupSizeInScope (found : Nat)
  | differenceInScope (found : Nat)
  | theRestFits (k : Kind)
  /-- A bare "another"/"other" of kind `k` finds no Contrast Anchor compatible with its head in
  kind and Card Type (`DRS.contrastAnchor?`). -/
  | contrastAnchor (k : Kind)
  | eventAgent
  | chooserInScope (found : Nat)
  | tokenSpecInScope (found : Nat)
  | openLetter (l : Letter)
  | bindingless
  | ignorableFor
  | readAmount
  /- local well-formedness -/
  | nonZeroQ
  | wellFormedQ
  | nonEmpty
  | atLeastTwo
  | flatConjuncts
  | flatDisjuncts
  | otherAnchored
  | coordinableDisjuncts
  | negatable
  | uniquifying
  | ballotLabelsOk
  | atLeastTwoZones
  | isExtremal
  | chosenQualityRead (q : QualitySort)
  | ascribable
  | linkSource
  | pileMention
  | predSays
  | amtNonZero
  | tallyOk
  | lookbackSubject
  | lookbackLocus
  | durationPossessor
  | quantLiteral
  | modesFit
  | facesFit
  | chapterMarks
  | distinct
  | kindDomainOk
  | kindAxisSort
  /- tables -/
  | knownAct (v : Deed)
  | knownKeywordTerm
  | linkedFacetNamed
  | eventHead
  | verbedVoiceOk
  /-- A handoff names neither a player nor an object. -/
  | handoffPerformer
  | tokenPhrase
  | numberChoiceInScope
  | notCoord
  | forEachAmount
  | notCompound
  | reflexEnclosure
  | thisWayOutcome
  /-- A "this way" lookback names an event no earlier text of its own spell or ability caused
  ("Draw a card. If you search your library this way, shuffle" [CR#608.2c]), or one whose Actor
  its subject cannot name. -/
  | thisWayUngrounded
  /-- A `triggering` lookback names an event that is not one its triggered ability triggers on
  [CR#603.2]. -/
  | triggeringUngrounded
  | ifDoneArmed
  | producedRuns
  | lineNonEmpty
  | counterHolderKind
  | qualityRead
  | notWordHeaded
  /- rules tables -/
  /-- A predefined token is named, because an effect creates it by that name [CR#111.10]. -/
  | tokenNamed
  /- registry definitions -/
  /-- A counter is placed on an object or a player [CR#122.1] and on nothing else. -/
  | definitionHolder (k : Kind)
  /-- A registry definition names what it defines; a nameless one is unreachable. -/
  | definitionNamed
  /- card frame laws -/
  | cardLine
  | modalFrame
  | doorFrame
  /-- "The abilities printed below <keyword>" reads a keyword line the card prints
  [CR#702.165a,702.165c]. -/
  | printedAfterKeyword (k : KeywordLabel)
  | cardName
  | cardBox
  | cardCost
  | flipHalf
  | levelRange
  | levelerFrame
  | bandsDisjoint
  | prototypeFrame
  /-- Entry Riders ride an entering: a replacement adding riders replaces a permanent's
  entering [CR#614.1c,614.12]. -/
  | riddenEntering
  /-- A Head Noun names an Object by its Object Class, Card Type or Subtype
  [CR#109.1,205.2a,205.3]; a player word, a pile or a stack word is none. -/
  | headNoun
  /-- A shift raises or lowers its value, never sets it: a cost shift (a cost set instead is an
  alternative cost [CR#118.9]), a damage shift, a loyalty symbol (which puts on or removes
  loyalty counters [CR#606.4]) or a die-result shift. -/
  | shiftSets
  /-- A replacement's riders are Entry Riders, never Pile Placements, which only a card put into
  another zone carries [CR#401.4,406.3,614.12]. -/
  | entryRidersOnly
  /-- An ability word is one the CR lists [CR#207.2c]. -/
  | knownAbilityWord (word : AbilityWordLabel)
  /- characteristic-defining abilities [CR#604.3a] -/
  /-- A Characteristic-Defining Ability defines colors, subtypes, power or toughness, and
  nothing else (criterion 1). -/
  | cdaEdit
  /-- A Characteristic-Defining Ability defines the object it is on, and affects no other
  object's characteristics (criterion 3). -/
  | cdaSubject
  /-- A Characteristic-Defining Ability is no grant: an object granting it, to itself or
  another, or an effect putting it in force, gives an ordinary continuous effect (criteria 2
  and 4). -/
  | cdaGranted
  /-- A Characteristic-Defining Ability sets its values unconditionally (criterion 5). -/
  | cdaConditional
  /- retention: an exemption from removing Attachments [CR#702.16n,702.16p,303.4c,704.5m] -/
  /-- A retention qualifies the one clause that removes Attachments already on its host as a
  state-based action: a protection grant, or a prohibition on attaching ("can't be enchanted")
  [CR#702.16c,702.16d,303.4c,704.5m]. A conjunction is refused: the exemption is the clause's,
  not the other clauses'. -/
  | retentionClause
  /-- A retention exempts Attachments of its clause's host: "this Aura" on the enchanted
  permanent, or a description of Auras, Equipment or Fortifications whose "attached to …" reads
  that host back [CR#702.16n,702.16p]. -/
  | retentionSubject
  deriving DecidableEq, Repr

end Semantics
