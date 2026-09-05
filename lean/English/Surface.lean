import Init

namespace English

/-- Binding is structural metadata, not a guess from a word's spelling. -/
inductive Atom where
  | word (text : String)
  | opening (text : String)
  | closing (text : String)
  | symbol (text : String)
  | lineBreak
  deriving DecidableEq

instance : Coe String Atom := ⟨Atom.word⟩

abbrev Surface := List Atom

def Atom.text : Atom → String
  | .word text | .opening text | .closing text | .symbol text => text
  | .lineBreak => "\n"

def Atom.separator : Atom → Atom → String
  | .lineBreak, _ | _, .lineBreak | .opening _, _ | _, .closing _ => ""
  | .symbol _, .symbol _ => ""
  | _, _ => " "

def Atom.capitalize : Atom → Atom
  | .word text => .word text.capitalize
  | atom => atom

/-- An embedded quoted sentence already owns the enclosing sentence's terminal period. -/
def Surface.finishSentence (surface : Surface) : Surface :=
  match surface.reverse with
  | .closing "\"" :: .closing "." :: _ | .closing "'" :: .closing "." :: _ => surface
  | .closing "." :: _ => surface
  | _ => surface ++ [.closing "."]

def Atom.nestedQuote : Atom → Atom
  | .opening "\"" => .opening "'"
  | .closing "\"" => .closing "'"
  | .opening "'" => .opening "\""
  | .closing "'" => .closing "\""
  | atom => atom

def Surface.quote (surface : Surface) : Surface :=
  [.opening "\""] ++ surface.map Atom.nestedQuote ++ [.closing "\""]

/-- Exact text spelling of an annotated surface; no whitespace normalization is assumed. -/
inductive Spells : Surface → String → Prop where
  | empty : Spells [] ""
  | single {atom : Atom} : Spells [atom] atom.text
  | cons {first second : Atom} {tail : Surface} {text : String} :
      Spells (second :: tail) text →
      Spells (first :: second :: tail) (first.text ++ first.separator second ++ text)

end English
