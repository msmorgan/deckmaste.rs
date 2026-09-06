import English.Documents

namespace English.Documents

/-- The same initial-adverbial clause can precede further sentences in one paragraph. -/
def continuedTrigger := unary
  (ternary (unary initialClause .sentence .sentence) sentence sentence (.body rfl) .body)
  .ordinary .ordinary

def threeModes := ternary mode mode mode (.modeList (n := 2)) .modeList
def modalGroup := binary initialClause threeModes .modes .modes
def modalBody := unary modalGroup (.body rfl) .body
def ordinaryModes := unary modalBody .ordinary .ordinary
def activatedModes := binary costs modalBody .activated .activated

def sentenceHeaderModes := ternary sentence sentence threeModes
  (.sentenceModes (n := 1)) (.sentenceModes (headers := [sentence.surface, sentence.surface]))

def emptyDocument : Witness Lexeme lexicon (.document .document) :=
  ⟨.node (.document .document) [], [], .node (.document (.document rfl)) .nil,
   .node .nil (.document .document)⟩

def emptyLevelBand := ternary range stats emptyDocument .levelBand .levelBand

def threeCosts := ternary symbolCost actionCost symbolCost (.costs (n := 2)) .costs

def threeKeywords := ternary flying ward landwalk (.keywordLine (n := 2)) .keywordLine

def threeParagraphs := ternary ordinary activated triggered (.document rfl) .document

theorem continued_trigger_text :
    Spells continuedTrigger.surface "Whenever creatures attack, attack. Attack. Attack." :=
  .cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.single)))))))))

theorem three_modes_text :
    Spells ordinaryModes.surface
      "Whenever creatures attack, attack —\n• Attack.\n• Attack.\n• Attack." :=
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.single)))))))))))))))))

theorem activated_modes_text :
    Spells activatedModes.surface
      "{2}, Attack: Whenever creatures attack, attack —\n• Attack.\n• Attack.\n• Attack." :=
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.single)))))))))))))))))))))

theorem sentence_header_modes_text :
    Spells sentenceHeaderModes.surface "Attack. Attack.\n• Attack.\n• Attack.\n• Attack." :=
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (.single)))))))))))))))

theorem empty_document_text : Spells emptyDocument.surface "" := .empty

theorem empty_level_band_text : Spells emptyLevelBand.surface "LEVEL 1-3\n4/4" :=
  .cons (.cons (.cons (.single)))

theorem three_costs_text : Spells threeCosts.surface "{2}, Attack, {2}" :=
  .cons (.cons (.cons (.cons (.single))))

theorem three_keywords_text : Spells threeKeywords.surface "Flying, ward {2}, landwalk" :=
  .cons (.cons (.cons (.cons (.cons (.cons (.single))))))

theorem three_paragraphs_text : Spells threeParagraphs.surface
    "Attack.\n{2}, Attack: Attack.\nWhenever creatures attack, attack." :=
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (
  .cons (.cons (.cons (.cons (.cons (.single)))))))))))))))

theorem document_coordination_rejected (coordinator : Coordinator) (children : List Category)
    (output : Category) :
    ¬ Production lexicon (.coordinate coordinator (.document .document)) children output := by
  intro h
  cases h with
  | coordinate license => cases license

theorem nested_document_collection_rejected :
    ¬ DocumentProduction .document [.document .document] (.document .document) := by
  intro h
  cases h with
  | document license => cases license

theorem nested_body_collection_rejected :
    ¬ DocumentProduction .body [.document .body, .document .sentence] (.document .body) := by
  intro h
  cases h with
  | body license => cases license

theorem mode_collection_arity {children : List Category}
    (h : DocumentProduction .modeList children (.document .modes)) :
    ∃ n, children = List.replicate (n + 1) (.document .mode) := by
  cases h
  exact ⟨_, rfl⟩

theorem cost_collection_arity {children : List Category}
    (h : DocumentProduction .costs children (.document .cost)) :
    ∃ n, children = List.replicate (n + 1) (.document .costComponent) := by
  cases h
  exact ⟨_, rfl⟩

theorem supertype_collection_arity {children : List Category}
    (h : DocumentProduction .supertypes children (.document .supertypes)) :
    ∃ n, children = List.replicate n (.document .supertype) := by
  cases h
  exact ⟨_, rfl⟩

theorem keyword_line_arity {children : List Category}
    (h : DocumentProduction .keywordLine children (.document .ability)) :
    ∃ n, children = List.replicate (n + 1) .keywordPhrase := by
  cases h
  exact ⟨_, rfl⟩

end English.Documents
