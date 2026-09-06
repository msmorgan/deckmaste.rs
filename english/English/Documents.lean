import English.Grammar

namespace English.Documents

/-- A concrete tree with both its grammatical derivation and its annotated surface. -/
structure Witness (Lexeme : Type) (lexicon : Lexicon Lexeme) (category : Category) where
  tree : Syntax Lexeme
  surface : Surface
  derives : Derives lexicon tree category
  realizes : Realizes lexicon tree surface

def unary {Lexeme : Type} {lexicon : Lexicon Lexeme} {rule : DocumentRule}
    {input output : Category} {surface : Surface} (child : Witness Lexeme lexicon input)
    (production : DocumentProduction rule [input] output)
    (linearization : DocumentLinearizes rule [child.surface] surface) :
    Witness Lexeme lexicon output :=
  ⟨.node (.document rule) [child.tree], surface,
   .closedNode (.document production) (.cons child.derives .nil),
   .node (.cons child.realizes .nil) (.document linearization)⟩

def binary {Lexeme : Type} {lexicon : Lexicon Lexeme} {rule : DocumentRule}
    {left right output : Category} {surface : Surface}
    (first : Witness Lexeme lexicon left) (second : Witness Lexeme lexicon right)
    (production : DocumentProduction rule [left, right] output)
    (linearization : DocumentLinearizes rule [first.surface, second.surface] surface) :
    Witness Lexeme lexicon output :=
  ⟨.node (.document rule) [first.tree, second.tree], surface,
   .closedNode (.document production) (.cons first.derives (.cons second.derives .nil)),
   .node (.cons first.realizes (.cons second.realizes .nil)) (.document linearization)⟩

def ternary {Lexeme : Type} {lexicon : Lexicon Lexeme} {rule : DocumentRule}
    {a b c output : Category} {surface : Surface}
    (first : Witness Lexeme lexicon a) (second : Witness Lexeme lexicon b)
    (third : Witness Lexeme lexicon c)
    (production : DocumentProduction rule [a, b, c] output)
    (linearization :
      DocumentLinearizes rule [first.surface, second.surface, third.surface] surface) :
    Witness Lexeme lexicon output :=
  ⟨.node (.document rule) [first.tree, second.tree, third.tree], surface,
   .closedNode (.document production)
     (.cons first.derives (.cons second.derives (.cons third.derives .nil))),
   .node (.cons first.realizes (.cons second.realizes (.cons third.realizes .nil)))
     (.document linearization)⟩

inductive Lexeme where
  | attack | gain | ward | walk | land | mana | roman | level | range | stats | threshold
  | label | legendary | creature | elf | dieRange | whenever | flying
  deriving DecidableEq

def keywordFrame : List (FrameItem Lexeme) := [.argument ⟨.object, .keywordPhrase⟩]

def quotedFrame : List (FrameItem Lexeme) := [.argument ⟨.object, .document .quotedText⟩]

def lexicalWords : List (Lexeme × Category × Surface) :=
  [(.mana, .document .symbol, [.symbol "{2}"]),
   (.roman, .document .notation, ["I"]), (.level, .document .notation, ["2"]),
   (.range, .document .notation, ["1-3"]), (.stats, .document .notation, ["4/4"]),
   (.threshold, .document .notation, ["8"]), (.dieRange, .document .notation, ["1–9"]),
   (.label, .document .label, ["Example"]),
   (.legendary, .document .supertype, ["Legendary"]),
   (.creature, .document .type, ["Creature"]), (.elf, .document .subtype, ["Elf"])]

def lexicon : Lexicon Lexeme where
  noun head number := (head = .land ∧ number = .singular) ∨
    (head = .creature ∧ number = .plural)
  adjective _ := False
  nounForm head number surface :=
    (head = .land ∧ number = .singular ∧ surface = (["land"] : Surface)) ∨
    (head = .creature ∧ number = .plural ∧ surface = (["creatures"] : Surface))
  adjectiveForm _ _ := False
  word head category := ∃ surface, (head, category, surface) ∈ lexicalWords
  wordForm head category surface := (head, category, surface) ∈ lexicalWords
  verb head form voice frame := form = .plain ∧ voice = .active ∧
    ((head = .attack ∧ frame = []) ∨ (head = .gain ∧ (frame = keywordFrame ∨ frame = quotedFrame)))
  verbForm head form surface := form = .plain ∧
    ((head = .attack ∧ surface = (["attack"] : Surface)) ∨
     (head = .gain ∧ surface = (["gain"] : Surface)))
  finite head agreement form := head = .attack ∧ agreement = ⟨.third, .plural⟩ ∧ form = .plain
  subordinator head finiteness := head = .whenever ∧ finiteness = .finite
  markerForm head surface := head = .whenever ∧ surface = (["whenever"] : Surface)
  keyword head parameter placement :=
    (head = .ward ∧ parameter = some (.document .cost) ∧ placement = .free) ∨
    (head = .walk ∧ parameter = some (.nominal .singular) ∧ placement = .boundSuffix) ∨
    (head = .flying ∧ parameter = none ∧ placement = .free)
  keywordForm head surface := (head = .ward ∧ surface = (["ward"] : Surface)) ∨
    (head = .flying ∧ surface = (["flying"] : Surface))
  keywordSuffix head suffix := head = .walk ∧ suffix = "walk"

def word (head : Lexeme) (category : Category) (surface : Surface)
    (distribution : WordCategory category) (form : (head, category, surface) ∈ lexicalWords) :
    Witness Lexeme lexicon category :=
  ⟨.word head category, surface, .word distribution ⟨surface, form⟩, .word form⟩

def mana := word .mana (.document .symbol) [.symbol "{2}"] .symbol (by decide)
def roman := word .roman (.document .notation) ["I"] .notation (by decide)
def level := word .level (.document .notation) ["2"] .notation (by decide)
def range := word .range (.document .notation) ["1-3"] .notation (by decide)
def stats := word .stats (.document .notation) ["4/4"] .notation (by decide)
def dieRange := word .dieRange (.document .notation) ["1–9"] .notation (by decide)
def threshold := word .threshold (.document .notation) ["8"] .notation (by decide)
def label := word .label (.document .label) ["Example"] .label (by decide)
def legendary := word .legendary (.document .supertype) ["Legendary"] .supertype (by decide)
def creature := word .creature (.document .type) ["Creature"] .type (by decide)
def elf := word .elf (.document .subtype) ["Elf"] .subtype (by decide)

def attack : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .attack .plain []) [], ["attack"],
   .verb ⟨rfl, rfl, Or.inl ⟨rfl, rfl⟩⟩ .nil,
   .node .nil (.verb (v := ["attack"]) ⟨rfl, Or.inl ⟨rfl, rfl⟩⟩)⟩

def instruction : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node .imperative [attack.tree], attack.surface,
   .node .imperative (.cons attack.derives .nil),
   .node (.cons attack.realizes .nil) .imperative⟩

def sentence := unary instruction .sentence .sentence
def body := unary sentence (.body rfl) .body
def ordinary := unary body .ordinary .ordinary
def document := unary ordinary (.document rfl) .document
def repeatedDocument := binary ordinary ordinary (.document rfl) .document
def symbolCost := unary mana .costSymbol .costSymbol
def cost := unary symbolCost (.costs (n := 0)) .costs
def actionCost := unary attack .costAction .costAction
def costs := binary symbolCost actionCost (.costs (n := 1)) .costs
def activated := binary costs body .activated .activated

def creatures : Witness Lexeme lexicon (.nounPhrase ⟨.third, .plural⟩) :=
  ⟨.node .barePlural [.noun .creature .plural], ["creatures"],
   .node .barePlural (.cons (.noun (Or.inr ⟨rfl, rfl⟩)) .nil),
   .node (.cons (.noun (surface := ["creatures"]) (Or.inr ⟨rfl, rfl, rfl⟩)) .nil) .barePlural⟩

def event : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node (.finite ⟨.third, .plural⟩ .plain) [creatures.tree, attack.tree],
   creatures.surface ++ attack.surface,
   .finite creatures.derives attack.derives (.verb ⟨rfl, rfl, rfl⟩) rfl,
   .node (.cons creatures.realizes (.cons attack.realizes .nil)) .finite⟩

def trigger : Witness Lexeme lexicon (.subordinateClause .finite) :=
  ⟨.node (.subordinate .whenever .finite) [event.tree], ("whenever" : Atom) :: event.surface,
   .node (.subordinate ⟨rfl, rfl⟩) (.cons event.derives .nil),
   .node (.cons event.realizes .nil) (.subordinate (m := ["whenever"]) ⟨rfl, rfl⟩)⟩

def initialClause : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node (.initialAdverbial (.subordinateClause .finite)) [trigger.tree, instruction.tree],
   trigger.surface ++ [.closing ","] ++ instruction.surface,
   .node (.initialAdverbial .subordinate) (.cons trigger.derives (.cons instruction.derives .nil)),
   .node (.cons trigger.realizes (.cons instruction.realizes .nil)) .initialAdverbial⟩

def triggered := unary (unary (unary initialClause .sentence .sentence)
  (.body rfl) .body) .ordinary .ordinary

def ward : Witness Lexeme lexicon (.keywordPhrase) :=
  ⟨.node (.keyword .ward (some (.document .cost)) .free) [cost.tree],
   ("ward" : Atom) :: cost.surface,
   .node (.keywordParameter (Or.inl ⟨rfl, rfl, rfl⟩)) (.cons cost.derives .nil),
   .node (.cons cost.realizes .nil)
     (.keywordParameter (keywordSurface := ["ward"]) (Or.inl ⟨rfl, rfl⟩))⟩

def flying : Witness Lexeme lexicon (.keywordPhrase) :=
  ⟨.node (.keyword .flying none .free) [], ["flying"],
   .node (.keywordBare (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))) .nil,
   .node .nil (.keywordBare (Or.inr ⟨rfl, rfl⟩))⟩

def bareKeyword := unary flying (.keywordLine (n := 0)) .keywordLine

def landwalk : Witness Lexeme lexicon (.keywordPhrase) :=
  ⟨.node (.keyword .walk (some (.nominal .singular)) .boundSuffix) [.noun .land .singular],
   ["land", .closing "walk"],
   .node (.keywordParameter (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))) 
     (.cons (.noun (lexicon := lexicon) (lexeme := .land) (number := .singular)
     (Or.inl ⟨rfl, rfl⟩)) .nil),
   .node (.cons (.noun (surface := ["land"]) (Or.inl ⟨rfl, rfl, rfl⟩)) .nil)
     (.keywordBound (suffix := "walk") ⟨rfl, rfl⟩)⟩

def gain (keyword : Witness Lexeme lexicon (.keywordPhrase)) :
    Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .gain .plain keywordFrame) [keyword.tree], 
   (["gain"] : Surface) ++ [keyword.surface].flatten,
   .verb ⟨rfl, rfl, Or.inr ⟨rfl, Or.inl rfl⟩⟩
     (.argument (complement := ⟨.object, .keywordPhrase⟩) keyword.derives .nil),
   .node (.cons keyword.realizes .nil) (.verb (lexicon := lexicon) (head := .gain) (form := .plain)
      (v := ["gain"]) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩)⟩

def keywordLine := unary ward (.keywordLine (n := 0)) .keywordLine
def boundLine := unary landwalk (.keywordLine (n := 0)) .keywordLine
def keywordList := binary ward landwalk (.keywordLine (n := 1)) .keywordLine
def quoted := unary document .quote .quote

def gainQuoted : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .gain .plain quotedFrame) [quoted.tree],
   (["gain"] : Surface) ++ [quoted.surface].flatten,
   .verb ⟨rfl, rfl, Or.inr ⟨rfl, Or.inr rfl⟩⟩
     (.argument (complement := ⟨.object, .document .quotedText⟩) quoted.derives .nil),
   .node (.cons quoted.realizes .nil)
     (.verb (lexicon := lexicon) (head := .gain) (form := .plain)
       (v := ["gain"]) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩)⟩

def quotedInstruction : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node .imperative [gainQuoted.tree], gainQuoted.surface,
   .node .imperative (.cons gainQuoted.derives .nil),
   .node (.cons gainQuoted.realizes .nil) .imperative⟩

def nestedQuote := unary (unary (unary (unary (unary quotedInstruction
  .sentence .sentence) (.body rfl) .body) .ordinary .ordinary) (.document rfl) .document) .quote
    .quote

def reminderProse : Witness Lexeme lexicon (.document .parenthetical) :=
  ⟨.node (.document .reminder) [body.tree], [.opening "("] ++ body.surface ++ [.closing ")"],
   .reminder body.derives (by rfl), .node (.cons body.realizes .nil) (.document .reminder)⟩

def reminder := unary (unary reminderProse (.body rfl) .body) .ordinary .ordinary

def mode := unary body .mode .mode
def modes := binary instruction (binary mode mode (.modeList (n := 1)) .modeList) .modes .modes
def weightedMode := binary mana body .weightedMode .weightedMode
def labelled := binary label ordinary .label .label
def chapter := binary roman body .chapter .chapter
def classLevel := ternary cost level document .classLevel .classLevel
def levelBand := ternary range stats document .levelBand .levelBand
def solve := unary body .solve .solve
def solved := unary activated .solved .solved
def dieRow := binary dieRange body .dieRow .dieRow
def dieDashRow := binary level body .dieDashRow .dieDashRow
def station := binary threshold keywordLine .station .station

def typeLine := ternary (unary legendary (.supertypes (n := 1)) .supertypes)
  (unary creature (.types (n := 0)) .types) (unary elf (.subtypes (n := 0)) .subtypes)
    .subtypedLine .subtypedLine

/-- Faces contain independent typed text boxes; spelling never inserts a combined name. -/
structure Faces where
  layout : FaceLayout
  first : Witness Lexeme lexicon (.document .document)
  rest : List (Witness Lexeme lexicon (.document .document))

def room : Faces := ⟨.room, document, [document]⟩
def multiface (layout : FaceLayout) : Faces := ⟨layout, document, [document]⟩

/-- Each assertion checks exact punctuation and spacing against an independently written string. -/
theorem ordinary_text : Spells document.surface "Attack." := .cons .single

theorem activated_text : Spells activated.surface "{2}, Attack: Attack." :=
  .cons (.cons (.cons (.cons (.cons .single))))

theorem triggered_text : Spells triggered.surface "Whenever creatures attack, attack." :=
  .cons (.cons (.cons (.cons (.cons .single))))

theorem symbols_bind : Spells [.symbol "{2}", .symbol "{U}"] "{2}{U}" := .cons .single

theorem keyword_text : Spells keywordLine.surface "Ward {2}" := .cons .single

theorem bare_keyword_text : Spells bareKeyword.surface "Flying" := .single

theorem bound_text : Spells boundLine.surface "Landwalk" := .cons .single

theorem keyword_second_host : Spells (gain ward).surface "gain ward {2}" := .cons (.cons .single)

theorem bound_second_host : Spells (gain landwalk).surface "gain landwalk" := .cons (.cons .single)

theorem quote_text : Spells quoted.surface "\"Attack.\"" := .cons (.cons (.cons .single))

theorem nested_quote_text : Spells nestedQuote.surface "\"Gain 'Attack.'\"" :=
  .cons (.cons (.cons (.cons (.cons (.cons .single)))))

theorem reminder_text : Spells reminder.surface "(Attack.)" := .cons (.cons (.cons .single))

theorem chapter_text : Spells chapter.surface "I — Attack." := .cons (.cons (.cons .single))

theorem class_text : Spells classLevel.surface "{2}: Level 2\nAttack." :=
  .cons (.cons (.cons (.cons (.cons (.cons .single)))))

theorem level_text : Spells levelBand.surface "LEVEL 1-3\n4/4\nAttack." :=
  .cons (.cons (.cons (.cons (.cons (.cons .single)))))

theorem case_text : Spells solve.surface "To solve — Attack." :=
  .cons (.cons (.cons (.cons .single)))

theorem die_text : Spells dieRow.surface "1–9 | Attack." := .cons (.cons (.cons .single))

theorem die_dash_text : Spells dieDashRow.surface "2 — Attack." :=
  .cons (.cons (.cons .single))

theorem modes_text : Spells modes.surface "Attack —\n• Attack.\n• Attack." :=
  .cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))))

theorem solved_text : Spells solved.surface "Solved — {2}, Attack: Attack." :=
  .cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))

theorem station_text : Spells station.surface "8+ | Ward {2}" :=
  .cons (.cons (.cons (.cons .single)))

theorem type_line_text : Spells typeLine.surface "Legendary Creature — Elf" :=
  .cons (.cons (.cons .single))

theorem room_faces_text : Spells room.first.surface "Attack." ∧
    ∀ face ∈ room.rest, Spells face.surface "Attack." := by
  refine ⟨ordinary_text, ?_⟩
  intro face member
  have equality : face = document := by simpa [room] using member
  subst face
  exact ordinary_text

theorem repeated_paragraphs : Spells repeatedDocument.surface "Attack.\nAttack." :=
  .cons (.cons (.cons (.cons .single)))

theorem empty_sentence_rejected (surface : Surface) :
    ¬ DocumentLinearizes .sentence [[]] surface := by
  intro derivation
  cases derivation

theorem nested_reminder_rejected (content : Syntax Lexeme)
    (nested : content.reminderFree = false) :
    ¬ Derives lexicon (.node (.document .reminder) [content]) (.document .parenthetical) := by
  intro derivation
  cases derivation with
  | node production _ =>
    cases production with
    | document rule => cases rule
  | reminder _ free =>
    rw [nested] at free
    cases free


theorem type_line_order : ¬ DocumentProduction .subtypedLine
    [.document .subtypes, .document .types, .document .supertypes] (.document .typeLine) := by
  intro derivation
  cases derivation


end English.Documents
