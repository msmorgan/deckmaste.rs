import Semantics

/-! Named macros for exposed primitives and fixed wording. Generated wrappers preserve
constructor signatures; the worded forms below expand into shared operations. The authoring
boundary records their names. -/

declare_semantic_primitives

namespace Semantics.Macros.Primitives.Instruction

/-- A numeric definition at instruction position uses the shared static definition. -/
def define (letter : Letter) (amount : Amount) : Semantics.Instruction :=
  .establish (.letterDefinition letter amount) none

def offer (body : Semantics.Instruction) (ifDid ifNot : Option Semantics.Instruction)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .withContinuation (.optional agent) body ifDid ifNot

def doIfDone (body : Semantics.Instruction) (ifDid ifNot : Option Semantics.Instruction) :
    Semantics.Instruction := .withContinuation .required body ifDid ifNot

def repeatTimes (times : Amount) (body : Semantics.Instruction) : Semantics.Instruction :=
  .repeat_ (.fixed times body)

register_semantic_macros
end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.NounPhrase

def both (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .and [left, right]
def eitherOf (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .or [left, right]

register_semantic_macros
end Semantics.Macros.Primitives.NounPhrase

namespace Semantics.Macros.Primitives.Cost

def either (left right : Semantics.Cost) : Semantics.Cost := .or [left, right]

register_semantic_macros
end Semantics.Macros.Primitives.Cost

namespace Semantics.Macros.Primitives.Predicate

def isAttached (word : Option AttachWord) : Semantics.Predicate :=
  .attachment .host word none

def attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Semantics.Predicate :=
  .attachment .host word (some by_)

def attachedTo (host : NounPhrase) : Semantics.Predicate :=
  .attachment .attachment none (some host)

/-- A transformed permanent is the whole double-faced permanent with its back face up. -/
def isTransformed : Semantics.Predicate := .currentFace .back

register_semantic_macros
end Semantics.Macros.Primitives.Predicate
