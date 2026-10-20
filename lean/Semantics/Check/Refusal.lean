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
  /-- A Relation no declaration declares [CR#702.55b,702.95b,702.99b]. -/
  | knownRelation (relation : RelationLabel)
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
  /- anaphora and scope: what the read found -/
  | anaphor (r : Reach) (pl : Plurality) (found : Nat)
  /-- "that turn" reads back the one turn an earlier phrase introduced; it found `found`. -/
  | turnAntecedent (found : Nat)
  /-- A Definition or Template body's later mention of a referent parameter finds no binding recorded under its
  Binding Identity, and its phrase would introduce one: reading it again would select anew. -/
  | bindingIdentity (id : BindingId)
  | choiceRef (ref : ChoiceRef) (sort : ChoiceSort) (found : Nat)
  | choiceInScope (k : Kind)
  /-- "The player chosen as [cost] was paid" names a cost whose payment chooses no player, so
  there is no choice for the linked ability to read back [CR#607.2d,607.2i]. -/
  | linkedChoiceInScope (which : LinkSource)
  | outcomeInScope (s : OutcomeSort) (found : Nat)
  | quantOutcomeInScope (found : Nat)
  | coinFlipInScope
  | damageDealtInScope
  | groupSizeInScope (found : Nat)
  | differenceInScope (found : Nat)
  | theRestFits (k : Kind)
  | anyTargeted (k : Kind)
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
  | knownCounter
  | linkedFacetNamed
  /-- A facet is read as what it records: an amount as an amount, the chosen player or the
  objects a deed acted on as a noun phrase. -/
  | linkedFacetKind
  | costNameable
  | keywordParamFits (k : KeywordLabel)
  | eventHead
  | verbedVoiceOk
  /-- A handoff names neither a player nor an object. -/
  | handoffPerformer
  | causedByOk
  | tokenPhrase
  | numberChoiceInScope
  | notCoord
  | forEachAmount
  | notCompound
  | reflexEnclosure
  | thisWayOutcome
  | ifDoneArmed
  | producedRuns
  | lineNonEmpty
  | counterHolderKind
  | qualityRead
  | keywordExtendable
  | keywordListOk
  | notWordHeaded
  /- rules tables -/
  /-- A predefined token is named, because an effect creates it by that name [CR#111.10]. -/
  | tokenNamed
  /- registry definitions -/
  /-- A counter is placed on an object or a player [CR#122.1] and on nothing else. -/
  | definitionHolder (k : Kind)
  /-- A registry definition names what it defines; a nameless one is unreachable. -/
  | definitionNamed
  /-- A designation's zone, card type and room half describe the object holding it, so only an
  object-held designation [CR#701.15b] writes one. -/
  | definitionScoped (designation : DesignationLabel)
  /- card frame laws -/
  | cardLine
  | cardText
  | modalFrame
  | chapterFrame
  | doorFrame
  /-- "The abilities printed below <keyword>" reads a keyword line the card prints
  [CR#702.165a,702.165c]. -/
  | printedAfterKeyword (k : KeywordLabel)
  | cardName
  | cardBox
  | cardCost
  | adventureInset
  | flipHalf
  | levelRange
  | levelerFrame
  | bandsDisjoint
  | prototypeFrame
  /-- Entry Riders ride an entering: a replacement adding riders replaces a permanent's
  entering [CR#614.1c,614.12]. -/
  | riddenEntering
  /-- A shift raises or lowers its value, never sets it: a cost shift, a damage shift, a loyalty
  symbol or a die-result shift [CR#118.9,606.4]. -/
  | shiftSets
  /-- A replacement's riders are Entry Riders, never Pile Placements, which only a card put into
  another zone carries [CR#401.4,406.3,614.12]. -/
  | entryRidersOnly
  deriving DecidableEq, Repr

end Semantics
