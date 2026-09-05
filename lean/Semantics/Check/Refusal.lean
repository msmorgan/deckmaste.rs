import Semantics.Check.Words

/-!
# Semantics.Check.Refusal

Why a spelling is refused. One constructor per Idris obligation, named after the Idris
obligation (`SoleHolder` → `.soleHolder`), so a pin names the obligation it refutes the way
the Idris docstring did in prose. A few carry the count they found, which is what a reader
wants to know for an anaphora refusal.

Idris elaboration refuses at the first failing obligation; the checker here reports every
failing one, so `check e = [r]` states that `r` is the only refusal, which is what a
non-vacuous pin claims.
-/

namespace Semantics

inductive Refusal where
  /- sorts -/
  /-- A phrase's kind is fixed by two constraints that disagree. -/
  | kindMismatch (expected found : Kind)
  | phrasal (k : Kind)
  | targetable (k : Kind)
  | targeter (k : Kind)
  | kindLte (k bound : Kind)
  | possessorKind (axis : PossessorAxis) (k : Kind)
  | counterScope (k : Kind)
  | projScope (k : Kind)
  | axesAt (k : Kind)
  | counterKindNamed (k : Kind)
  | designationHolder (designation : DesignationLabel) (k : Kind)
  | designationScope (designation : DesignationLabel)
  | designationChecked (designation : DesignationLabel)
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
  | selfDefined
  /- zones -/
  | zoneIs (z : Zone)
  | zoneFits
  | zoneCoherent
  | possessable (z : Zone)
  | playableFrom
  | placeArrangementFits
  | placeOrdinalFits
  | ordinalNonZero
  | exposableZone (z : Zone)
  | costSubject
  | paidSubject
  | stackActOn
  | copySourceOk
  | movable
  | destOk
  | arrangementOk
  | placeable
  | ridersFit
  | discardOk
  | moveDestination
  | counterMemory
  | attachHeadOk
  | attachFits
  | combatRelOk
  | damageRecipient
  | attackable
  | statusHolder
  | statusMarkable
  | statusWord
  /- anaphora and scope: what the read found -/
  | anaphor (r : Reach) (pl : Plurality) (found : Nat)
  | choiceRef (ref : ChoiceRef) (sort : ChoiceSort) (found : Nat)
  | choiceInScope (k : Kind)
  | outcomeInScope (s : OutcomeSort) (found : Nat)
  | quantOutcomeInScope (found : Nat)
  | coinFlipInScope
  | damageDealtInScope
  | groupSizeInScope (found : Nat)
  | gapInScope (found : Nat)
  | theRestFits (k : Kind)
  | anyTargeted (k : Kind)
  | eventAgent
  | chooserInScope (found : Nat)
  | tokenSpecInScope (found : Nat)
  | openLetter (l : Letter)
  | bindingless
  | ignorableFor
  | readAmount
  | keepsOuter
  /- local well-formedness -/
  | nonZeroQ
  | wellFormedQ
  | nonEmpty
  | atLeastTwo
  | flatConjuncts
  | flatDisjuncts
  | contradictionFree
  | otherAnchored
  | parallelDisjuncts
  | coordinableDisjuncts
  | negatable
  | uniquifying
  | rolesOk
  | ballotLabelsOk
  | atLeastTwoZones
  | colorBoundOk
  | isExtremal
  | chosenQualityRead (q : QualitySort)
  | subtypeType
  | ascribable
  | ascriptionOk
  | permanentSpellType
  | linkSource
  | pileMention
  | predSays
  | markingOk
  | amtNonZero
  | tallyOk
  | statHeadTysOk
  | complementPlain
  | complementSourced
  | complementWritten
  | lookbackSubject
  | lookbackComplement
  | lookbackSource
  | lookbackDest
  | lookbackLocus
  | visibilityOk
  | windowOk
  | pointWindowOk
  | durationPossessor
  | quantLiteral
  | modesFit
  /-- Every mode is costed, or none is [CR#702.172a]. -/
  | modesCosted
  | facesFit
  | chapterMarks
  | distinct
  | kindDomainOk
  | kindAxisSort
  /- tables -/
  | knownAct (v : VerbLabel)
  | knownKeyword (k : KeywordLabel)
  | knownKeywordTerm
  | knownCounter
  | paidFacetNamed
  | costNameable
  | keywordParamFits (k : KeywordLabel)
  | keywordBodyFits (k : KeywordLabel)
  | keywordCost (k : KeywordLabel)
  /-- A keyword whose parameter is a cost the controller pays takes a cost payable by you. -/
  | keywordCostPaidByYou (k : KeywordLabel)
  | deedFits
  /-- The deed opens an opponent's library [CR#701.29a]. -/
  | opponentsLibrary
  | deedRides
  | verbPatientOk
  | verbedVoiceOk
  | verbBecomesOk
  | enactAgentOk
  | featureNounOk (f : DeedFeature)
  | nontarget
  | counterBatchOk
  | causedByOk
  | tokenPhrase
  | eventUnderway
  | headerNontarget
  | doorNamesHost
  | durationOk
  | settableValue
  | selfExchanged
  | attacker
  | numberChoiceInScope
  | deonticBoundOk
  | deonticPatientOk
  | asThoughOk
  | deonticRiderOk
  | triggerCountOk
  | deckReadable
  | deckComparable
  | modifyStat
  | selfDefinedOk
  | grantSubject
  | grantable
  | altPayment
  | addedPayment
  | notExtended
  | notCarvedOut
  | notCoord
  | clauseStatic
  | becomesOk
  | untriggeredLimit
  | interceptable
  | damageOpUse
  | ctrlOverride
  | manaRun
  /-- A hybrid or Phyrexian symbol names two different halves [CR#107.4e,107.4f]. -/
  | manaSymbolOk
  | forEachAmount
  | costAction
  | notCompound
  | payable
  | payAgrees
  | costTapOnce
  | costPaidByYou
  | heldClause
  | reflexEnclosure
  | thisWayOutcome
  | notInstead
  | ifDoneArmed
  | enactKeepsOuter
  | producedRuns
  | colorCountOk
  | twoParties
  | controlExchangeZone
  | cardSwapZones
  | zoneSwap
  | tokenTyped
  | tokenPtOk
  | subsFitLine
  | tokenAbilities
  | tokenCanonical
  | tokenQualsFit
  | additionUnnamed
  | copyBundle
  | lineNonEmpty
  | kindAmountOk
  | counterSourceScope
  | counterHolderKind
  | qualityRead
  | keywordExtendable
  | keywordListOk
  | notWordHeaded
  | emblemAbilities
  | chapterDefaults
  /- card frame laws -/
  | cardLine
  | cardText
  | modalFrame
  | chapterFrame
  | doorFrame
  | cardName
  | cardBox
  | cardCost
  | jointChoices
  | adventureInset
  | flipHalf
  | levelRange
  | levelerFrame
  | bandsDisjoint
  | prototypeFrame
  deriving DecidableEq, Repr

end Semantics
