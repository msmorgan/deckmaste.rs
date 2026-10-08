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
  | amountParameter (scope index : Nat)
  | lexicalScope (scope : Nat)
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
  /- zones -/
  | ordinalNonZero
  | counterMemory
  /- anaphora and scope: what the read found -/
  | anaphor (r : Reach) (pl : Plurality) (found : Nat)
  /-- A macro body's later mention of a referent parameter finds no binding recorded under its
  Binding Identity, and its phrase would introduce one: reading it again would select anew. -/
  | bindingIdentity (id : BindingId)
  | choiceRef (ref : ChoiceRef) (sort : ChoiceSort) (found : Nat)
  | choiceInScope (k : Kind)
  /-- "The player chosen as [cost] was paid" names a cost whose payment chooses no player, so
  there is no choice for the linked ability to read back [CR#607.2d,607.2i]. -/
  | paidChoiceInScope (which : PaidCostName)
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
  | lookbackLocus
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
  | knownAct (v : Deed)
  | knownKeyword (k : KeywordLabel)
  | knownKeywordTerm
  | knownCounter
  | paidFacetNamed
  /-- A facet is read as what it records: an amount as an amount, the chosen player as a noun
  phrase. -/
  | paidFacetKind
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
  /-- A handoff names neither a player nor a permanent [CR#110.1]. -/
  | handoffPerformer
  /-- A deed whose patient is the performer's own is done to an object whose possessor is known to
  be a different player than its performer [CR#701.21a,701.9a]. -/
  | patientPossessor
  /-- A noun in a deed's agent or patient role does not fit what that deed takes there. -/
  | deedNounOk (d : Deed)
  | nontarget
  | counterBatchOk
  | causedByOk
  | tokenPhrase
  | headerNontarget
  | doorNamesHost
  | settableValue
  | selfExchanged
  | numberChoiceInScope
  | deonticBoundOk
  | deonticPatientOk
  | asThoughOk
  | deonticRiderOk
  | deckReadable
  | deckComparable
  | selfDefinedOk
  | grantSubject
  | grantable
  | altPayment
  | addedPayment
  | notCarvedOut
  | notCoord
  | clauseStatic
  | becomesOk
  | damageOpUse
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
  | reflexEnclosure
  | thisWayOutcome
  | ifDoneArmed
  | enactKeepsOuter
  | producedRuns
  | colorCountOk
  | twoParties
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
