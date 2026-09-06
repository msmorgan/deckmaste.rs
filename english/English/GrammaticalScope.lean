import English.Selection

namespace English.GrammaticalScope
variable {L : Type}
def coord (c : Coordinator) (cat : Category) (a b : Syntax L) :=
  Syntax.node (.coordinate c cat) [a, b]
def adjunct (cat : Category) (a m : Syntax L) :=
  Syntax.node (.adjunct cat .prepositionPhrase .after) [a, m]
inductive ScopeMove : Syntax L → Syntax L → Prop where
  | modifier (c : Coordinator) (number : Number) (m l r : Syntax L) :
      ScopeMove (.modify m (coord c (.nominal number) l r))
        (coord c (.nominal number) (.modify m l) r)
  | postmodifier (c : Coordinator) (cat : Category) (l r m : Syntax L) :
      coordinationResult c cat = cat →
      ScopeMove (adjunct cat (coord c cat l r) m) (coord c cat l (adjunct cat r m))
  | auxiliary (head : L) (form selected : InflectionalForm) (polarity : Polarity)
      (voice selectedVoice : Voice) (body mobile : Syntax L) :
      ScopeMove
        (adjunct (.verbPhrase form voice)
          (.node (.auxiliary head form selected polarity voice selectedVoice) [body]) mobile)
        (.node (.auxiliary head form selected polarity voice selectedVoice)
          [adjunct (.verbPhrase selected selectedVoice) body mobile])

/-- One local alternation inside unchanged grammatical context. -/
inductive ScopeStep : Syntax L → Syntax L → Prop where
  | direct {a b} : ScopeMove a b → ScopeStep a b
  | node {a b} (construction : Construction L) (before after : List (Syntax L)) :
      ScopeStep a b →
      ScopeStep (.node construction (before ++ [a] ++ after))
        (.node construction (before ++ [b] ++ after))
  | modifierLeft {a b} (head : Syntax L) : ScopeStep a b →
      ScopeStep (.modify a head) (.modify b head)
  | modifierRight {a b} (modifier : Syntax L) : ScopeStep a b →
      ScopeStep (.modify modifier a) (.modify modifier b)
abbrev Reading (lexicon : Lexicon L) (category : Category) (surface : Surface) :=
  {tree : Syntax L // Admissible lexicon tree category surface}
/-- Every intermediate in the equivalence chain is independently admitted. -/
inductive ScopeRelated {lexicon : Lexicon L} {category : Category} {surface : Surface} :
    Reading lexicon category surface → Reading lexicon category surface → Prop where
  | refl (a) : ScopeRelated a a
  | step {a b} : ScopeStep a.val b.val → ScopeRelated a b
  | symm {a b} : ScopeRelated a b → ScopeRelated b a
  | trans {a b c} : ScopeRelated a b → ScopeRelated b c → ScopeRelated a c
theorem two_regions {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a a' b b' : Syntax L} (construction : Construction L)
    (left : ScopeStep a a') (right : ScopeStep b b')
    (initial : Admissible lexicon (.node construction [a,b]) category surface)
    (middle : Admissible lexicon (.node construction [a',b]) category surface)
    (final : Admissible lexicon (.node construction [a',b']) category surface) :
    ScopeRelated ⟨.node construction [a,b],initial⟩ ⟨.node construction [a',b'],final⟩ := by
  exact ScopeRelated.trans
    (b := ⟨.node construction [a',b],middle⟩)
    (.step (.node construction [] [b] left))
    (.step (.node construction [a'] [] right))
def scopeSetoid (lexicon : Lexicon L) (category : Category) (surface : Surface) :
    Setoid (Reading lexicon category surface) where
  r := ScopeRelated
  iseqv := ⟨ScopeRelated.refl, ScopeRelated.symm, ScopeRelated.trans⟩
def scopeClass {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (tree : Reading lexicon category surface) : Quotient (scopeSetoid lexicon category surface) :=
  Quotient.mk _ tree
theorem same_class_iff {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (a b : Reading lexicon category surface) :
    scopeClass a = scopeClass b ↔ ScopeRelated a b := by
  constructor
  · intro h
    exact Quotient.exact h
  · intro h
    exact Quotient.sound (s := scopeSetoid lexicon category surface) h

/-- Lexical identity survives scope variation even when different lexemes spell alike. -/
def constructionLexemes : Construction L → List L
  | .keyword head _ _ | .compare head | .measure head | .preposition head _
  | .verb head _ _ _ | .auxiliary head _ _ _ _ _ | .subordinate head _ => [head]
  | _ => []

mutual
  def lexicalLeaves : Syntax L → List L
    | .noun head _ | .adjective head | .marker head | .word head _ => [head]
    | .modify a b | .sharedCoordination _ _ a b | .relative _ a b =>
        lexicalLeaves a ++ lexicalLeaves b
    | .node construction children => constructionLexemes construction ++ childLeaves children
    | .frameCoordination _ a b => childLeaves a ++ childLeaves b
    | .gap _ => []
    | .ellipsis antecedent => lexicalLeaves antecedent
  def childLeaves : List (Syntax L) → List L
    | [] => []
    | head :: rest => lexicalLeaves head ++ childLeaves rest
end

private theorem childLeaves_append (a b : List (Syntax L)) :
    childLeaves (a ++ b) = childLeaves a ++ childLeaves b := by
  induction a with
  | nil => rfl
  | cons head rest ih => simp [childLeaves, ih, List.append_assoc]

theorem move_preserves_lexemes {a b : Syntax L} (h : ScopeMove a b) :
    lexicalLeaves a = lexicalLeaves b := by
  cases h <;> simp [lexicalLeaves, childLeaves, constructionLexemes, coord, adjunct,
    List.append_assoc]

theorem step_preserves_lexemes {a b : Syntax L} (h : ScopeStep a b) :
    lexicalLeaves a = lexicalLeaves b := by
  induction h with
  | direct h => exact move_preserves_lexemes h
  | node construction before after _ ih =>
    simp [lexicalLeaves, childLeaves_append, childLeaves, ih]
  | modifierLeft head _ ih => simp [lexicalLeaves, ih]
  | modifierRight modifier _ ih => simp [lexicalLeaves, ih]

theorem related_preserves_lexemes {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : Reading lexicon category surface} (h : ScopeRelated a b) :
    lexicalLeaves a.val = lexicalLeaves b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_lexemes h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

def hostNode (construction : Construction L) (children : List (Syntax L)) : Syntax L :=
  match construction, children with
  | .adjunct _ _ _, [head, _] => head
  | _, _ => .node construction children

mutual
  /-- Remove mobile attachments while retaining the ordered host and document structure. -/
  def hostStructure : Syntax L → Syntax L
    | .modify _ head => hostStructure head
    | .node construction children => hostNode construction (hostChildren children)
    | .sharedCoordination c cat a b =>
        .sharedCoordination c cat (hostStructure a) (hostStructure b)
    | .relative n a b => .relative n (hostStructure a) (hostStructure b)
    | .frameCoordination c a b => .frameCoordination c (hostChildren a) (hostChildren b)
    | .ellipsis antecedent => .ellipsis (hostStructure antecedent)
    | tree => tree
  def hostChildren : List (Syntax L) → List (Syntax L)
    | [] => []
    | head :: rest => hostStructure head :: hostChildren rest
end

private theorem hostChildren_append (a b : List (Syntax L)) :
    hostChildren (a ++ b) = hostChildren a ++ hostChildren b := by
  induction a with
  | nil => rfl
  | cons head rest ih => simp [hostChildren, ih]

theorem move_preserves_hosts {a b : Syntax L} (h : ScopeMove a b) :
    hostStructure a = hostStructure b := by
  cases h <;> simp [hostStructure, hostChildren, hostNode, coord, adjunct]

theorem step_preserves_hosts {a b : Syntax L} (h : ScopeStep a b) :
    hostStructure a = hostStructure b := by
  induction h with
  | direct h => exact move_preserves_hosts h
  | node construction before after _ ih =>
    simp [hostStructure, hostChildren_append, hostChildren, ih]
  | modifierLeft head _ _ => rfl
  | modifierRight modifier _ ih => exact ih

theorem related_preserves_hosts {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : Reading lexicon category surface} (h : ScopeRelated a b) :
    hostStructure a.val = hostStructure b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_hosts h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

theorem map_related {lexicon : Lexicon L} {category output : Category}
    {surface target : Surface} (f : Syntax L → Syntax L)
    (steps : ∀ {a b}, ScopeStep a b → ScopeStep (f a) (f b))
    (valid : ∀ a : Reading lexicon category surface, Admissible lexicon (f a.val) output target)
    {a b : Reading lexicon category surface} (h : ScopeRelated a b) :
    ScopeRelated ⟨f a.val, valid a⟩ ⟨f b.val, valid b⟩ := by
  induction h with
  | refl => exact .refl _
  | step h => exact .step (steps h)
  | symm _ ih => exact .symm ih
  | trans _ _ ih₁ ih₂ => exact .trans ih₁ ih₂

/-- Pack only complete surviving readings related by licensed grammatical scope moves. -/
def package {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (survivors : Reading lexicon category surface → Prop)
    (representative : Reading lexicon category surface) :=
  Selection.pack survivors scopeClass (scopeClass representative)

theorem package_exact {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (survivors : Reading lexicon category surface → Prop)
    (representative tree : Reading lexicon category surface) :
    (package survivors representative).readings tree ↔
      survivors tree ∧ ScopeRelated tree representative := by
  exact and_congr_right (fun _ ↦ same_class_iff tree representative)

end English.GrammaticalScope
