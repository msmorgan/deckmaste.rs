import English.Readings

/-! Synthetic lexical declarations and grammatical witnesses. These are not corpus coverage claims. -/
namespace English.LexicalWitnesses

def singular : Agreement := ⟨.third, .singular⟩
def plural : Agreement := ⟨.third, .plural⟩
def speaker : Agreement := ⟨.first, .singular⟩
def addressee : Agreement := ⟨.second, .singular⟩

inductive Lexeme where
  | it | they | you | be | exile | attack | creature | turn | white | during | if_
  | targetNoun | targetVerb | targetMarker | oneNoun | oneDet | i | see | herPossessive | herObject | duckNoun | duckVerb
  | by_ | mana | much | two | plus | artifactType | creatureSubtype
  | nontoken | degreeAdverb | three | scalarVariable | nameVerb | cardName | defend
  | amount | of_ | greenSymbol | get | unknown
  deriving DecidableEq

abbrev Row := FeatureBundle × Surface

def rows : Lexeme → List Row
  | .it => [(.word (.nounPhrase singular), ["it"])]
  | .they => [(.word (.nounPhrase plural), ["they"])]
  | .you => [(.word (.nounPhrase addressee), ["you"])]
  | .be => [(.verb .preterite (some (.past, singular)), ["was"]),
      (.verb .preterite (some (.past, plural)), ["were"]),
      (.verb .preterite (some (.past, addressee)), ["were"])]
  | .exile => [(.verb .pastParticiple none, ["exiled"])]
  | .attack => [(.verb .plain none, ["attack"]),
      (.verb .plain (some (.present, plural)), ["attack"]),
      (.verb .thirdSingularPresent (some (.present, singular)), ["attacks"])]
  | .creature => [(.noun .plural .count, ["creatures"]), (.noun .singular .count, ["creature"])]
  | .turn => [(.noun .plural .count, ["turns"])]
  | .white => [(.adjective, ["white"])]
  | .during => [(.preposition (.nounPhrase plural), ["during"])]
  | .if_ => [(.subordinator .finite, ["if"])]
  | .targetNoun => [(.noun .singular .count, ["target"])]
  | .targetVerb => [(.verb .plain none, ["target"]),
      (.verb .plain (some (.present, plural)), ["target"])]
  | .targetMarker => [(.targeting, ["target"]), (.determinative .singular .count, ["target"])]
  | .oneNoun => [(.noun .singular .count, ["one"])]
  | .oneDet => [(.determinative .singular .count, ["one"])]
  | .i => [(.word (.nounPhrase speaker) .nominative, ["I"])]
  | .see => [(.verb .preterite (some (.past, speaker)), ["saw"])]
  | .herPossessive => [(.determinative .singular .count, ["her"])]
  | .herObject => [(.word (.nounPhrase singular) .accusative, ["her"])]
  | .duckNoun => [(.noun .singular .count, ["duck"])]
  | .duckVerb => [(.verb .plain none, ["duck"]) ]
  | .by_ => [(.preposition .measurePhrase, ["by"])]
  | .mana => [(.noun .singular .mass, ["mana"])]
  | .much => [(.determinative .singular .mass, ["much"])]
  | .two => [(.word .measurePhrase, ["2"]), (.word (.cardinalNumeral .plural), ["two"])]
  | .plus => [(.measure, ["plus"])]
  | .artifactType => [(.word (.document .type), ["Artifact"])]
  | .creatureSubtype => [(.word (.document .subtype), ["Golem"])]
  | .nontoken => [(.attributive, ["nontoken"])]
  | .degreeAdverb => [(.word .adverbPhrase, ["very"])]
  | .three => [(.word .unsignedScalar, [.symbol "3"])]
  | .scalarVariable => [(.word .unsignedScalar, [.symbol "X"])]
  | .nameVerb => [(.verb .pastParticiple none, ["named"])]
  | .cardName => [(.identity .name, ["Powerstone", "Shard"])]
  | .defend => [(.verb .gerundParticiple none, ["defending"])]
  | .amount => [(.noun .singular .count, ["amount"])]
  | .of_ => [(.fixed, ["of"])]
  | .greenSymbol => [(.word (.document .symbol), [.symbol "{G}"])]
  | .get => [(.verb .plain none, ["get"])]
  | .unknown => []

def objectFrame : List (FrameItem Lexeme) := [.argument ⟨.object, .nounPhrase plural⟩]

def declaration (head : Lexeme) : LexemeDeclaration Lexeme where
  lemma := ((rows head).headD (.fixed, [])).2
  provenance := "synthetic-v3"
  bundles := (rows head).map Prod.fst
  morphology := {
    defaultForm := fun _ ↦ []
    overrides := fun bundle ↦ some (((rows head).filter (fun row ↦ row.1 == bundle)).map Prod.snd) }
  verbFrame form voice frame := match head with
    | .nameVerb => form = .pastParticiple ∧ voice = .passive ∧
        frame = [.argument ⟨.complement, .name⟩]
    | .defend => form = .gerundParticiple ∧ voice = .active ∧ frame = []
    | .get => form = .plain ∧ voice = .active ∧
        frame = [.argument ⟨.complement, .measurePhrase .pair⟩]
    | _ =>
    (head = .exile ∧ form = .pastParticiple ∧ voice = .passive ∧ frame = []) ∨
    (head = .attack ∧ voice = .active ∧ frame = []) ∨
    (head = .targetVerb ∧ voice = .active ∧ frame = objectFrame) ∨
    (head = .duckVerb ∧ form = .plain ∧ voice = .active ∧ frame = []) ∨
    (head = .see ∧ form = .preterite ∧ voice = .active ∧
      (frame = [.argument ⟨.object, .nounPhrase singular⟩] ∨
        frame = [.argument ⟨.object, .nounPhrase singular⟩, .argument ⟨.complement, .verbPhrase .plain⟩]))
  auxiliary form selected voice selectedVoice :=
    head = .be ∧ form = .preterite ∧ selected = .pastParticiple ∧
      voice = .passive ∧ selectedVoice = .passive
  temporal := head = .turn
  participialAttributive := head = .defend
  nounComplement marker category :=
    head = .amount ∧ marker = .of_ ∧ category = .symbolSequence

def environment : LexicalEnvironment Lexeme
  | .unknown => none
  | head => some (declaration head)

def word (head : Lexeme) (bundle : FeatureBundle) (spelling : Surface)
    (capitalization : Capitalization := .declared) : WordForm Lexeme :=
  ⟨head, bundle, spelling, capitalization, "synthetic-v3"⟩

/-- Every declared spelling in this environment is a well-formed surface, so the payload side
condition of `LexicalAnalysis` is discharged once here rather than at each licensing site. -/
theorem rows_well_formed (head : Lexeme) : ∀ row ∈ rows head, Surface.WellFormed row.2 := by
  cases head <;> decide

theorem row_licensed (head : Lexeme) (bundle : FeatureBundle) (spelling : Surface)
    (capitalization : Capitalization)
    (caseLicensed : capitalization = .declared ∨ spelling.capitalize ≠ spelling)
    (known : head ≠ .unknown)
    (member : (bundle, spelling) ∈ rows head) :
    (word head bundle spelling capitalization).Licensed environment := by
  refine ⟨declaration head, ?_, rfl, ?_, ?_, caseLicensed, rfl,
    rows_well_formed head (bundle, spelling) member⟩
  · cases head <;> simp_all [environment, word]
  · exact List.mem_map.mpr ⟨(bundle, spelling), member, rfl⟩
  · change spelling ∈ (((rows head).filter (fun row ↦ row.1 == bundle)).map Prod.snd)
    exact List.mem_map.mpr ⟨(bundle, spelling), List.mem_filter.mpr ⟨member, by simp⟩, rfl⟩

def was := word .be (.verb .preterite (some (.past, singular))) ["was"]
def were := word .be (.verb .preterite (some (.past, plural))) ["were"]
def wereYou := word .be (.verb .preterite (some (.past, addressee))) ["were"]
def exiled := word .exile (.verb .pastParticiple none) ["exiled"]
def it := word .it (.word (.nounPhrase singular)) ["it"]
def they := word .they (.word (.nounPhrase plural)) ["they"]
def you := word .you (.word (.nounPhrase addressee)) ["you"]
def creatures := word .creature (.noun .plural .count) ["creatures"]
def creature := word .creature (.noun .singular .count) ["creature"]
def attack := word .attack (.verb .plain none) ["attack"]
def attackPlural := word .attack (.verb .plain (some (.present, plural))) ["attack"]
def attacks := word .attack (.verb .thirdSingularPresent (some (.present, singular))) ["attacks"]

@[simp] theorem was_licensed : was.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem were_licensed : were.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem wereYou_licensed : wereYou.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem exiled_licensed : exiled.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem it_licensed : it.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem they_licensed : they.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem you_licensed : you.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem creatures_licensed : creatures.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem creature_licensed : creature.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem attack_licensed : attack.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem attackPlural_licensed : attackPlural.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem attacks_licensed : attacks.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

/-- Neither valid lexical alternative licenses their crossed agreement/spelling pair. -/
theorem crossed_was_plural :
    ¬ (word .be (.verb .preterite (some (.past, plural))) ["was"]).Licensed environment := by
  simp [WordForm.Licensed, LexicalAnalysis, word, environment, declaration, rows,
    Morphology.forms, singular, plural, addressee]

theorem crossed_were_singular :
    ¬ (word .be (.verb .preterite (some (.past, singular))) ["were"]).Licensed environment := by
  simp [WordForm.Licensed, LexicalAnalysis, word, environment, declaration, rows,
    Morphology.forms, singular, plural, addressee]

theorem noun_verb_homographs :
    (word .targetNoun (.noun .singular .count) ["target"]).Licensed environment ∧
    (word .targetVerb (.verb .plain none) ["target"]).Licensed environment :=
  ⟨row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows]),
   row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])⟩

theorem noun_determinative_homographs :
    (word .oneNoun (.noun .singular .count) ["one"]).Licensed environment ∧
    (word .oneDet (.determinative .singular .count) ["one"]).Licensed environment :=
  ⟨row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows]),
   row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])⟩

end English.LexicalWitnesses
