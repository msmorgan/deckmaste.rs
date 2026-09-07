import English.AmbiguityWitnesses

namespace English.DependencyWitnesses
open LexicalWitnesses GrammarWitnesses FamilyWitnesses

def relativeBody : Reading Lexeme :=
  .node (.finite plural .plain) [.gap (.nounPhrase plural), predicate]
def relative : Reading Lexeme := .relative .plural noun relativeBody
def zeroRelative : Reading Lexeme := .relative .plural noun relativeBody .zero

theorem relative_body_derives : Judges grammar relativeBody (.clause .finite) [.nounPhrase plural] :=
  .finite .gap predicate_derives (.verb ⟨attackPlural_licensed, .present, rfl⟩) rfl

theorem relative_body_realizes : Reading.Realizes grammar relativeBody ["attack"] :=
  .node (.cons .gap (.cons predicate_realizes .nil)) .finite

theorem relative_admitted : Reading.Admitted environment [] relative (.nominal .plural)
    ["creatures", "that", "attack"] := by
  refine ⟨⟨.relative noun_derives relative_body_derives, ?_, ?_, ?_⟩,
    .relative noun_realizes relative_body_realizes⟩
  · simp [relative, relativeBody, noun, predicate, Features.Conforms, Features.ChildrenConform,
      Features.Local, Syntax.nominalCase, FrameCases, Case.Allows]
  · simp [relative, relativeBody, noun, predicate, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.RelativeLicense, Dependencies.exposed, Dependencies.frameExposed]
  · simp [relative, relativeBody, noun, predicate, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]

theorem zero_subject_relative_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] zeroRelative (.nominal .plural) surface := by
  intro admitted
  exact Dependencies.zero_subject_excluded (Lexical.dependencies environment) relativeBody plural
    rfl admitted.1.2.2.1.1

def shared : Reading Lexeme := .sharedCoordination .and_ (.clause .finite) relativeBody relativeBody
def sharedRelative : Reading Lexeme := .relative .plural noun shared

theorem shared_relative_admitted : Reading.Admitted environment [] sharedRelative (.nominal .plural)
    ["creatures", "that", "attack", "and", "attack"] := by
  refine ⟨⟨.relative noun_derives (.sharedCoordination rfl relative_body_derives relative_body_derives),
      ?_, ?_, ?_⟩,
    .relative noun_realizes (.sharedCoordination relative_body_realizes relative_body_realizes)⟩
  · simp [sharedRelative, shared, relativeBody, noun, predicate, Features.Conforms,
      Features.ChildrenConform, Features.Local, Syntax.nominalCase, FrameCases, Case.Allows]
  · simp [sharedRelative, shared, relativeBody, noun, predicate, Dependencies.Safe,
      Dependencies.ChildrenSafe, Dependencies.Local, Dependencies.RelativeLicense,
      Dependencies.exposed, Dependencies.frameExposed]
  · simp [sharedRelative, shared, relativeBody, noun, predicate, Reading.GrammarConforms,
      Reading.ChildrenConform, Reading.LocalGrammar]

theorem ordinary_coordination_cannot_discharge_shared_gap (surface : Surface) :
    ¬ Reading.Admitted environment []
      (.relative .plural noun (.node (.coordinate .and_ (.clause .finite)) [relativeBody, relativeBody]))
      (.nominal .plural) surface := by
  intro admitted
  have safe := admitted.1.2.2.1.2.2.1
  exact Dependencies.ordinary_coordination_cannot_share (Lexical.dependencies environment) .and_
    (.clause .finite) relativeBody relativeBody (by decide) safe

def gappedModifier : Reading Lexeme := .modify adjective (.gap (.nominal .plural))
def sharing : Reading Lexeme := .sharedCoordination .and_ (.nominal .plural) gappedModifier gappedModifier
def raised : Reading Lexeme := .node (.rightNodeRaising (.nominal .plural) (.nominal .plural))
  [sharing, noun]

theorem raised_admitted : Reading.Admitted environment [] raised (.nominal .plural)
    ["white", "and", "white", "creatures"] := by
  have gapped : Judges grammar gappedModifier (.nominal .plural) [.nominal .plural] :=
    .modify adjective_derives .gap
  have spelled : Reading.Realizes grammar gappedModifier ["white"] :=
    .modify adjective_realizes .gap
  refine ⟨⟨.rightNodeRaising (.sharedCoordination rfl gapped gapped) noun_derives, ?_, ?_, ?_⟩,
    .node (.cons (.sharedCoordination spelled spelled) (.cons noun_realizes .nil)) .rightNodeRaising⟩
  · simp [raised, sharing, gappedModifier, adjective, noun, Features.Conforms, Features.Local,
      Features.containsTarget]
  · simp [raised, sharing, gappedModifier, adjective, noun, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.exposed]
  · simp [raised, sharing, gappedModifier, adjective, noun, Reading.GrammarConforms,
      Reading.ChildrenConform, Reading.LocalGrammar, Reading.gapOccurrences, coordinable]

theorem single_gap_is_not_sharing (surface : Surface) :
    ¬ Reading.Admitted environment []
      (.node (.rightNodeRaising (.nominal .plural) (.nominal .plural)) [.gap (.nominal .plural), noun])
      (.nominal .plural) surface := by
  intro admitted
  have localCheck := admitted.1.2.2.2.1
  change _ ∧ 2 ≤ 1 at localCheck
  omega

def omission : Reading Lexeme := .ellipsis .pastParticiple .passive

theorem contextual_ellipsis_admitted : Reading.Admitted environment exiledTree.antecedents omission
    (.verbPhrase .pastParticiple .passive) [] := by
  refine ⟨⟨.ellipsis (by simp [exiledTree, Syntax.antecedents, childAntecedents]), ?_, ?_, ?_⟩, .ellipsis⟩
  all_goals exact ⟨trivial, trivial⟩

theorem missing_antecedent_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] omission (.verbPhrase .pastParticiple .passive) surface := by
  intro admitted
  cases admitted.1.1 with
  | ellipsis member => cases member

theorem wrong_voice_antecedent_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [.verbPhrase .pastParticiple .active] omission
      (.verbPhrase .pastParticiple .passive) surface := by
  intro admitted
  cases admitted.1.1 with
  | ellipsis member => simp at member

end English.DependencyWitnesses
