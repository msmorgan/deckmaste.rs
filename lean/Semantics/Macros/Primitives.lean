import Semantics.Macros.CharacteristicInputs

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

namespace Semantics.Macros.Primitives.StaticSpec

def qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    Semantics.StaticSpec := .characteristicChange subject (payload.edits op)

def abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : Semantics.StaticSpec :=
  .characteristicChange subject [.removedAbilities (.specified abilities)]

def allAbilityLoss (subject : NounPhrase) (except : Option Semantics.Predicate) :
    Semantics.StaticSpec := .characteristicChange subject [.removedAbilities (.allExcept except)]

register_semantic_macros
end Semantics.Macros.Primitives.StaticSpec

namespace Semantics.Macros.Primitives.CopyExcept

def types (types : List CardType) (subtypes : List Subtype) : Semantics.CopyExcept :=
  .edits [.typeLine .adds (({ types := types, subtypes := subtypes } :
    Characteristics).typeChanges)]

def name (name : String) : Semantics.CopyExcept := .edits [.name .sets name]
def pt (power toughness : Amount) : Semantics.CopyExcept :=
  .edits [.stat .sets .power power, .stat .sets .toughness toughness]
def nonlegendary : Semantics.CopyExcept :=
  .edits [.typeLine .loses { supertypes := some [.legendary] }]
def color (color : Color) : Semantics.CopyExcept := .edits [.colors .sets (.some [color])]

register_semantic_macros
end Semantics.Macros.Primitives.CopyExcept

namespace Semantics.Macros.Primitives.Instruction

def removeFromCombat (subject : NounPhrase) : Semantics.Instruction :=
  .combat subject (.participation .outsideCombat)

def becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Semantics.Instruction :=
  .combat subject (.participation (.attacking defender))

def becomeBlocking (subject blocked : NounPhrase) : Semantics.Instruction :=
  .combat subject (.blocking .attached blocked)

def stopBlocking (subject blocked : NounPhrase) : Semantics.Instruction :=
  .combat subject (.blocking .unattached blocked)

def attachTo (subject host : NounPhrase) : Semantics.Instruction :=
  .enact (.action "Attach") (.attachment .attached subject (some host))

def unattach (subject : NounPhrase) : Semantics.Instruction :=
  .attachment .unattached subject none

def addTurn (count : Amount) (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) :
    Semantics.Instruction := .insertPart .turn none count none (some agent)

def addPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart) (agent : Option NounPhrase := none) : Semantics.Instruction :=
  .insertPart part anchor count followedBy agent

register_semantic_macros
end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.Instruction

def create (count : Amount) (token : TokenSpec) (riders : List TokenRider)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .createObject count (.token token riders) agent

def getEmblem (abilities : List Ability)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .createObject (.lit 1) (.emblem abilities) agent

register_semantic_macros
end Semantics.Macros.Primitives.Instruction
