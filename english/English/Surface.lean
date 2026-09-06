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

/-- Read the terminal punctuation through any number of enclosing quotation marks. -/
private def terminalPeriod : Surface → Bool
  | .closing "." :: _ => true
  | .closing "\"" :: rest | .closing "'" :: rest => terminalPeriod rest
  | _ => false

/-- An embedded quoted sentence already owns the enclosing sentence's terminal period. -/
def Surface.finishSentence (surface : Surface) : Surface :=
  if terminalPeriod surface.reverse then surface else surface ++ [.closing "."]

def Atom.nestedQuote : Atom → Atom
  | .opening "\"" => .opening "'"
  | .closing "\"" => .closing "'"
  | .opening "'" => .opening "\""
  | .closing "'" => .closing "\""
  | atom => atom

def Surface.quote (surface : Surface) : Surface :=
  [.opening "\""] ++ surface.map Atom.nestedQuote ++ [.closing "\""]

/-- Separators belong between items, including when a collection has one or zero items. -/
def Surface.join (separator : Surface) : List Surface → Surface
  | [] => []
  | [item] => item
  | item :: next :: rest => item ++ separator ++ Surface.join separator (next :: rest)

def Surface.capitalize : Surface → Surface
  | [] => []
  | first :: rest => first.capitalize :: rest

def Surface.followingLine : Surface → Surface
  | [] => []
  | first :: rest => .lineBreak :: first :: rest

/-- Exact text spelling of an annotated surface; no whitespace normalization is assumed. -/
inductive Spells : Surface → String → Prop where
  | empty : Spells [] ""
  | single {atom : Atom} : Spells [atom] atom.text
  | cons {first second : Atom} {tail : Surface} {text : String} :
      Spells (second :: tail) text →
      Spells (first :: second :: tail) (first.text ++ first.separator second ++ text)

end English
