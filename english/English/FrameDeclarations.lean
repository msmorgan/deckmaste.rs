import English.Documents
import English.Dependencies

/-! Declared optional frame positions select an ordinary ordered frame before composition. -/
namespace English.FrameDeclarations

variable {L : Type}

inductive Item (L : Type) where
  | required (item : FrameItem L)
  | optional (item : FrameItem L)

/-- Expansion changes presence only; it cannot reorder or change a declared role or marker. -/
inductive Expands : List (Item L) → List (FrameItem L) → Prop where
  | nil : Expands [] []
  | required {item : FrameItem L} {schema : List (Item L)} {frame : List (FrameItem L)} :
      Expands schema frame → Expands (.required item :: schema) (item :: frame)
  | present {item : FrameItem L} {schema : List (Item L)} {frame : List (FrameItem L)} :
      Expands schema frame → Expands (.optional item :: schema) (item :: frame)
  | absent {item : FrameItem L} {schema : List (Item L)} {frame : List (FrameItem L)} :
      Expands schema frame → Expands (.optional item :: schema) frame

theorem one_optional_exact (first last : FrameItem L) (frame : List (FrameItem L)) :
    Expands [.required first, .optional last] frame ↔
      frame = [first] ∨ frame = [first, last] := by
  constructor
  · intro h
    cases h with
    | required tail =>
      cases tail with
      | present rest => cases rest; exact Or.inr rfl
      | absent rest => cases rest; exact Or.inl rfl
  · intro h
    rcases h with rfl | rfl
    · exact .required (.absent .nil)
    · exact .required (.present .nil)

inductive Lexeme where
  | give | creature | artifact | to | during
  deriving DecidableEq

def plural : Agreement := ⟨.third, .plural⟩
def nominalCategory := Category.nounPhrase plural
def role : FrameSlot := ⟨.complement, nominalCategory⟩

def object : FrameItem Lexeme := .argument ⟨.object, nominalCategory⟩
def recipient : FrameItem Lexeme := .marked .to role
def optionalRecipient : List (Item Lexeme) := [.required object, .optional recipient]

/-- Voice selects a declaration independently of the spelling of the participle. -/
def lexicon : Lexicon Lexeme where
  noun head number := (head = .creature ∨ head = .artifact) ∧ number = .plural
  adjective _ := False
  nounForm head number surface := number = .plural ∧
    ((head = .creature ∧ surface = (["creatures"] : Surface)) ∨
     (head = .artifact ∧ surface = (["artifacts"] : Surface)))
  adjectiveForm _ _ := False
  verb head form voice frame := head = .give ∧
    ((form = .plain ∧ voice = .active ∧
      (Expands optionalRecipient frame ∨ frame = [object, object])) ∨
     (form = .pastParticiple ∧ voice = .passive ∧ frame = [object]))
  markerForm head surface := head = .to ∧ surface = (["to"] : Surface)
  verbForm head form surface := head = .give ∧
    ((form = .plain ∧ surface = (["give"] : Surface)) ∨
     (form = .pastParticiple ∧ surface = (["given"] : Surface)))

def creatures : Syntax Lexeme := .node .barePlural [.noun .creature .plural]
def artifacts : Syntax Lexeme := .node .barePlural [.noun .artifact .plural]

theorem creatures_derives : Derives lexicon creatures nominalCategory :=
  .node .barePlural (.cons (.noun (lexicon := lexicon) ⟨Or.inl rfl, rfl⟩) .nil)

theorem artifacts_derives : Derives lexicon artifacts nominalCategory :=
  .node .barePlural (.cons (.noun (lexicon := lexicon) ⟨Or.inr rfl, rfl⟩) .nil)

theorem optional_absent : Derives lexicon
    (.node (.verb .give .plain [object]) [creatures]) (.verbPhrase .plain) :=
  .verb ⟨rfl, Or.inl ⟨rfl, rfl, Or.inl (.required (.absent .nil))⟩⟩
    (.argument (slot := ⟨.object, nominalCategory⟩) creatures_derives .nil)

theorem optional_present : Derives lexicon
    (.node (.verb .give .plain [object, recipient]) [creatures, .marker .to, artifacts])
    (.verbPhrase .plain) :=
  .verb ⟨rfl, Or.inl ⟨rfl, rfl, Or.inl (.required (.present .nil))⟩⟩
    (.argument (slot := ⟨.object, nominalCategory⟩) creatures_derives
      (.marked (marker := Lexeme.to) (slot := role) artifacts_derives .nil))

theorem optional_does_not_drop_required : ¬ Expands optionalRecipient [] := by
  intro h
  cases h

theorem optional_does_not_change_marker :
    ¬ Expands optionalRecipient [object, .marked .during role] := by
  intro h
  rcases (one_optional_exact object recipient _).mp h with h | h
  · cases h
  · cases h

theorem optional_does_not_reorder : ¬ Expands optionalRecipient [recipient, object] := by
  intro h
  rcases (one_optional_exact object recipient _).mp h with h | h
  · cases h
  · cases h

/-- The lexical declaration admits both the marked and double-Object realization. -/
theorem multiple_complement_orders :
    Derives lexicon
      (.node (.verb .give .plain [object, recipient]) [artifacts, .marker .to, creatures])
      (.verbPhrase .plain) ∧
    Derives lexicon
      (.node (.verb .give .plain [object, object]) [creatures, artifacts])
      (.verbPhrase .plain) :=
  ⟨.verb ⟨rfl, Or.inl ⟨rfl, rfl, Or.inl (.required (.present .nil))⟩⟩
     (.argument (slot := ⟨.object, nominalCategory⟩) artifacts_derives
       (.marked (marker := Lexeme.to) (slot := role) creatures_derives .nil)),
   .verb ⟨rfl, Or.inl ⟨rfl, rfl, Or.inr rfl⟩⟩
     (.argument (slot := ⟨.object, nominalCategory⟩) creatures_derives
       (.argument (slot := ⟨.object, nominalCategory⟩) artifacts_derives .nil))⟩

def retainedObject : Syntax Lexeme :=
  .node (.verb .give .pastParticiple [object] .passive) [creatures]

theorem passive_retains_declared_object :
    Derives lexicon retainedObject (.verbPhrase .pastParticiple .passive) :=
  .verb ⟨rfl, Or.inr ⟨rfl, rfl, rfl⟩⟩
    (.argument (slot := ⟨.object, nominalCategory⟩) creatures_derives .nil)

theorem retained_object_surface : Realizes lexicon retainedObject ["given", "creatures"] :=
  .node (.cons (.node (.cons (.noun (lexicon := lexicon)
    ⟨rfl, Or.inl ⟨rfl, rfl⟩⟩) .nil) .barePlural) .nil)
    (.verb (v := ["given"]) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩)

theorem passive_cannot_drop_declared_object :
    ¬ JudgeFrame lexicon [] [object] [] := by
  intro h
  cases h

theorem passive_cannot_acquire_another_object :
    ¬ JudgeFrame lexicon [creatures, artifacts] [object] [] := by
  have impossible {gaps : List Category}
      (h : JudgeFrame lexicon [creatures, artifacts] [object] gaps) : False := by
    cases h with
    | argument _ tail => cases tail
  exact impossible

theorem passive_cannot_borrow_active_optional_role :
    ¬ lexicon.verb .give .pastParticiple .passive [object, recipient] := by
  intro h
  rcases h.2 with h | h
  · cases h.1
  · cases h.2.2

theorem retained_gap_keeps_object_relation :
    Dependencies.exposed
      (.node (.verb Lexeme.give .pastParticiple [object] .passive)
        [.gap nominalCategory]) = [⟨nominalCategory, .object⟩] := rfl

theorem marked_gap_keeps_complement_relation :
    Dependencies.exposed
      (.node (.verb Lexeme.give .plain [object, recipient])
        [creatures, .marker .to, .gap nominalCategory]) =
      [⟨nominalCategory, .complement⟩] := rfl

end English.FrameDeclarations
