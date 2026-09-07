import English.Documents

namespace English.EllipsisInteractions

inductive Lexeme where
  | attack | do_ | you | if_
  deriving DecidableEq

def agreement : Agreement := ⟨.second, .singular⟩

def lexicon : Lexicon Lexeme where
  noun _ _ := False
  adjective _ := False
  nounForm _ _ _ := False
  adjectiveForm _ _ := False
  word head category := head = .you ∧ category = .nounPhrase agreement
  wordForm head category surface :=
    head = .you ∧ category = .nounPhrase agreement ∧ surface = (["you"] : Surface)
  verb head form voice frame := head = .attack ∧ form = .plain ∧ voice = .active ∧ frame = []
  verbForm head form surface := form = .plain ∧
    ((head = .attack ∧ surface = (["attack"] : Surface)) ∨
     (head = .do_ ∧ surface = (["do"] : Surface)))
  auxiliary head form selected voice selectedVoice := head = .do_ ∧ form = .plain ∧
    selected = .plain ∧ voice = .active ∧ selectedVoice = .active
  finite head subject form := head = .do_ ∧ subject = agreement ∧ form = .plain
  subordinator head finiteness := head = .if_ ∧ finiteness = .finite
  markerForm head surface := head = .if_ ∧ surface = (["if"] : Surface)

def attack : Syntax Lexeme := .node (.verb .attack .plain []) []
def instruction : Syntax Lexeme := .node .imperative [attack]
def firstSentence : Syntax Lexeme := .node (.document .sentence) [instruction]
def you : Syntax Lexeme := .word .you (.nounPhrase agreement)
def doEllipsis : Syntax Lexeme := .node (.auxiliary .do_ .plain .plain .positive) [.ellipsis .plain]
def ellipticalClause : Syntax Lexeme := .node (.finite agreement .plain) [you, doEllipsis]
def condition : Syntax Lexeme := .node (.subordinate .if_ .finite) [ellipticalClause]
def continuedClause : Syntax Lexeme :=
  .node (.initialAdverbial (.subordinateClause .finite)) [condition, instruction]
def secondSentence : Syntax Lexeme := .node (.document .sentence) [continuedClause]
def paragraph : Syntax Lexeme := .node (.document .body) [firstSentence, secondSentence]

theorem attack_derives (context : List Category) :
    DerivesIn lexicon context attack (.verbPhrase .plain) :=
  .verb ⟨rfl, rfl, rfl, rfl⟩ .nil

theorem first_derives (context : List Category) :
    DerivesIn lexicon context firstSentence (.document .sentence) :=
  .node (.document .sentence) (.cons (.node .imperative (.cons (attack_derives context) .nil)) .nil)

theorem second_derives (context : List Category) (recoverable : .verbPhrase .plain ∈ context) :
    DerivesIn lexicon context secondSentence (.document .sentence) := by
  have youD : DerivesIn lexicon context you (.nounPhrase agreement) := .word .pronoun ⟨rfl, rfl⟩
  have doD : DerivesIn lexicon context doEllipsis (.verbPhrase .plain) :=
    .node (.auxiliary ⟨rfl, rfl, rfl, rfl, rfl⟩) (.cons (.ellipsis recoverable) .nil)
  have clauseD : DerivesIn lexicon context ellipticalClause (.clause .finite) :=
    .finite youD doD (.auxiliary ⟨rfl, rfl, rfl⟩) rfl
  have conditionD : DerivesIn lexicon context condition (.subordinateClause .finite) :=
    .node (.subordinate ⟨rfl, rfl⟩) (.cons clauseD .nil)
  have instructionD : DerivesIn lexicon context instruction (.clause .finite) :=
    .node .imperative (.cons (attack_derives context) .nil)
  exact .node (.document .sentence) (.cons
    (.node (.initialAdverbial .subordinate) (.cons conditionD (.cons instructionD .nil))) .nil)

theorem paragraph_derives : Derives lexicon paragraph (.document .body) :=
  .paragraph (.body rfl) (.cons (first_derives [])
    (.cons (second_derives _ (by simp [firstSentence, instruction, attack,
      Syntax.antecedents, childAntecedents])) .nil))

theorem sentence_forms :
    Realizes lexicon firstSentence ["Attack", .closing "."] ∧
    Realizes lexicon secondSentence ["If", "you", "do", .closing ",", "attack", .closing "."] := by
  have attackForm : Realizes lexicon attack ["attack"] :=
    .node .nil (.verb (v := ["attack"]) ⟨rfl, Or.inl ⟨rfl, rfl⟩⟩)
  have instructionForm : Realizes lexicon instruction ["attack"] :=
    .node (.cons attackForm .nil) .imperative
  have firstForm : Realizes lexicon firstSentence ["Attack", .closing "."] :=
    .node (.cons instructionForm .nil) (.document .sentence)
  have ellipticForm : Realizes lexicon ellipticalClause ["you", "do"] :=
    .node (.cons (.word ⟨rfl, rfl, rfl⟩)
      (.cons (.node (.cons .ellipsis .nil)
        (.auxiliary (v := ["do"]) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩)) .nil)) .finite
  have conditionForm : Realizes lexicon condition ["if", "you", "do"] :=
    .node (.cons ellipticForm .nil) (.subordinate (m := ["if"]) ⟨rfl, rfl⟩)
  have continuedForm : Realizes lexicon continuedClause
      ["if", "you", "do", .closing ",", "attack"] :=
    .node (.cons conditionForm (.cons instructionForm .nil)) .initialAdverbial
  have secondForm : Realizes lexicon secondSentence
      ["If", "you", "do", .closing ",", "attack", .closing "."] :=
    .node (.cons continuedForm .nil) (.document .sentence)
  exact ⟨firstForm, secondForm⟩

theorem paragraph_text : Written lexicon paragraph (.document .body)
    "Attack. If you do, attack." :=
  ⟨_, ⟨paragraph_derives,
    .node (.cons sentence_forms.1 (.cons sentence_forms.2 .nil)) (.document .body)⟩,
    .cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))⟩

theorem ellipsis_requires_context : ¬ Derives lexicon (.ellipsis .plain) (.verbPhrase .plain) := by
  intro h
  cases h with
  | ellipsis member => cases member

theorem omitted_form_must_match :
    ¬ DerivesIn lexicon [.verbPhrase .plain] (.ellipsis .pastParticiple)
      (.verbPhrase .pastParticiple) := by
  intro h
  cases h with
  | ellipsis member => simp at member

theorem future_cannot_license_first (rest : List (Syntax Lexeme)) (categories : List Category) :
    ¬ JudgeParagraph lexicon [] (.ellipsis .plain :: rest) (.verbPhrase .plain :: categories) := by
  intro h
  cases h with
  | cons first _ => exact ellipsis_requires_context first

theorem omission_introduces_nothing (form : InflectionalForm) (voice : Voice) :
    (Syntax.ellipsis (Lexeme := Lexeme) form voice).antecedents = [] := rfl

theorem quotation_context_isolated (context : List Category) (content : Syntax Lexeme) :
    (Construction.document (Lexeme := Lexeme) .quote).childContext context = [] ∧
    (Syntax.node (.document .quote) [content]).antecedents = [] := ⟨rfl, rfl⟩

theorem reminder_inherits_without_exporting (context : List Category) (content : Syntax Lexeme) :
    (Construction.document (Lexeme := Lexeme) .reminder).childContext context = context ∧
    (Syntax.node (.document .reminder) [content]).antecedents = [] := ⟨rfl, rfl⟩

def reminderBody : Syntax Lexeme := .node (.document .body) [secondSentence]
def parenthetical : Syntax Lexeme := .node (.document .reminder) [reminderBody]
def inlineReminder : Syntax Lexeme := .node (.document .body) [firstSentence, parenthetical]

theorem inline_reminder_derives : Derives lexicon inlineReminder (.document .body) := by
  let context := firstSentence.antecedents ++ []
  have recoverable : Category.verbPhrase .plain ∈ context := by
    simp [context, firstSentence, instruction, attack, Syntax.antecedents, childAntecedents]
  have reminderD : DerivesIn lexicon context parenthetical (.document .parenthetical) :=
    .reminder (.paragraph (.body rfl) (.cons (second_derives context recoverable) .nil)) rfl
  exact .paragraph (.body rfl) (.cons (first_derives []) (.cons reminderD .nil))

theorem inline_reminder_text : Written lexicon inlineReminder (.document .body)
    "Attack. (If you do, attack.)" :=
  ⟨_, ⟨inline_reminder_derives,
    .node (.cons sentence_forms.1 (.cons
      (.node (.cons (.node (.cons sentence_forms.2 .nil) (.document .body)) .nil)
        (.document .reminder)) .nil)) (.document .body)⟩,
    .cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))))⟩

/-! The paragraph body's second judgment route, and its removal. `.document .body` used to be a
`DocumentProduction`, so `Production.document` lifted it and `JudgesIn.node` accepted every
paragraph a second way — checking all items in one shared context and passing their gaps out
through the node, neither of which is what `JudgeParagraph` enforces. The rule is now
`ParagraphProduction`, which only `JudgesIn.paragraph` consumes. -/

/-- After the repair: no `Production` lifts the paragraph body, so the `.node` route is closed for
`.document .body` at every arity and every result category. -/
theorem body_has_no_node_production (categories : List Category) (category : Category) :
    ¬ Production lexicon (.document .body) categories category := by
  intro h
  cases h with
  | document rule => cases rule

/-- What the retired route consumed is still all satisfied for this paragraph — the body rule
holds of its categories, and every item is judged in one shared context. So the second derivation
of one and the same value really was available, and closing it removed a duplicate route, not a
grammatical reading: `paragraph_derives` still derives the same tree. -/
theorem retired_node_route_premises :
    ParagraphProduction [.document .sentence] ∧
    JudgeChildren lexicon [firstSentence] [.document .sentence] [] :=
  ⟨.body rfl, .cons (first_derives []) .nil⟩

/-- The statement of the repair: every judgment of a body node now goes through
`JudgesIn.paragraph`, so `JudgeParagraph`'s ordered threading is not merely *a* route but the
*only* one. Two consequences follow immediately — a paragraph is gap-closed (`JudgesIn.paragraph`
concludes with `[]`), and `future_cannot_license_first` above governs every body node rather than
only those built the paragraph way. -/
theorem paragraph_is_the_only_route {context : List Category} {children : List (Syntax Lexeme)}
    {category : Category} {gaps : List Category}
    (h : JudgesIn lexicon context (.node (.document .body) children) category gaps) :
    ∃ categories, ParagraphProduction categories ∧
      JudgeParagraph lexicon context children categories ∧ gaps = [] := by
  cases h with
  | node production _ => exact absurd production (body_has_no_node_production _ _)
  | paragraph production items => exact ⟨_, production, items, rfl⟩

end English.EllipsisInteractions
