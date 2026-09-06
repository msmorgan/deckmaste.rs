import Semantics.Macros.CharacteristicInputs
import Semantics.Check.Phrase

/-! Named macros for exposed primitives and fixed wording. Generated wrappers preserve
constructor signatures; the worded forms below expand into shared operations. The authoring
boundary records their names. -/

declare_semantic_primitives

namespace Semantics.Macros.Primitives.Instruction

/-- A numeric definition at instruction position uses the shared static definition. -/
semantic_macro define (letter : Letter) (amount : Amount) : Semantics.Instruction :=
  .establish (.letterDefinition letter amount) none

semantic_macro offer (body : Semantics.Instruction) (ifDid ifNot : Option Semantics.Instruction)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .withContinuation (.optional agent) body ifDid ifNot

semantic_macro doIfDone (body : Semantics.Instruction) (ifDid ifNot : Option Semantics.Instruction) :
    Semantics.Instruction := .withContinuation .required body ifDid ifNot

semantic_macro repeatTimes (times : Amount) (body : Semantics.Instruction) : Semantics.Instruction :=
  .repeat_ (.fixed times body)

end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.NounPhrase

semantic_macro both (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .and [left, right]
semantic_macro eitherOf (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .or [left, right]

end Semantics.Macros.Primitives.NounPhrase

namespace Semantics.Macros.Primitives.Cost

semantic_macro either (left right : Semantics.Cost) : Semantics.Cost := .or [left, right]

end Semantics.Macros.Primitives.Cost

namespace Semantics.Macros.Primitives.Predicate

semantic_macro isAttached (word : Option AttachWord) : Semantics.Predicate :=
  .attachment .host word none

semantic_macro attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Semantics.Predicate :=
  .attachment .host word (some by_)

semantic_macro attachedTo (host : NounPhrase) : Semantics.Predicate :=
  .attachment .attachment none (some host)

/-- A transformed permanent is the whole double-faced permanent with its back face up. -/
semantic_macro isTransformed : Semantics.Predicate := .currentFace .back

end Semantics.Macros.Primitives.Predicate

namespace Semantics.Macros.Primitives.StaticSpec

semantic_macro qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    Semantics.StaticSpec := .characteristicChange subject (payload.edits op)

semantic_macro abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : Semantics.StaticSpec :=
  .characteristicChange subject [.removedAbilities (.specified abilities)]

semantic_macro allAbilityLoss (subject : NounPhrase) (except : Option Semantics.Predicate) :
    Semantics.StaticSpec := .characteristicChange subject [.removedAbilities (.allExcept except)]

end Semantics.Macros.Primitives.StaticSpec

namespace Semantics.Macros.Primitives.CopyExcept

semantic_macro types (types : List CardType) (subtypes : List Subtype) : Semantics.CopyExcept :=
  .edits [.typeLine .adds (({ types := types, subtypes := subtypes } :
    Characteristics).typeChanges)]

semantic_macro name (name : String) : Semantics.CopyExcept := .edits [.name .sets name]
semantic_macro pt (power toughness : Amount) : Semantics.CopyExcept :=
  .edits [.stat .sets .power power, .stat .sets .toughness toughness]
semantic_macro nonlegendary : Semantics.CopyExcept :=
  .edits [.typeLine .loses { supertypes := some [.legendary] }]
semantic_macro color (color : Color) : Semantics.CopyExcept := .edits [.colors .sets (.some [color])]

end Semantics.Macros.Primitives.CopyExcept

namespace Semantics.Macros.Primitives.Instruction

semantic_macro removeFromCombat (subject : NounPhrase) : Semantics.Instruction :=
  .combat subject (.participation .outsideCombat)

semantic_macro becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Semantics.Instruction :=
  .combat subject (.participation (.attacking defender))

semantic_macro becomeBlocking (subject blocked : NounPhrase) : Semantics.Instruction :=
  .combat subject (.blocking .attached blocked)

semantic_macro stopBlocking (subject blocked : NounPhrase) : Semantics.Instruction :=
  .combat subject (.blocking .unattached blocked)

semantic_macro attachTo (subject host : NounPhrase) : Semantics.Instruction :=
  .enact (.action "Attach") (.attachment .attached subject (some host))

semantic_macro unattach (subject : NounPhrase) : Semantics.Instruction :=
  .attachment .unattached subject none

semantic_macro addTurn (count : Amount) (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) :
    Semantics.Instruction := .insertPart .turn none count none (some agent)

semantic_macro addPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart) (agent : Option NounPhrase := none) : Semantics.Instruction :=
  .insertPart part anchor count followedBy agent

end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.Instruction

semantic_macro create (count : Amount) (token : TokenSpec) (riders : List TokenRider)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .createObject count (.token token riders) agent

semantic_macro getEmblem (abilities : List Ability)
    (agent : NounPhrase := Semantics.Macros.Primitives.NounPhrase.you) : Semantics.Instruction :=
  .createObject (.lit 1) (.emblem abilities) agent

end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.Instruction

/-- Counter is one labeled stack-to-graveyard move. -/
semantic_macro counterSpell (subject : NounPhrase) : Semantics.Instruction :=
  .enact (.action "Counter") (.move subject (.zone .graveyard .bare) [])

end Semantics.Macros.Primitives.Instruction

namespace Semantics.Macros.Primitives.StaticSpec

semantic_macro entryChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    (disclosure : Disclosure) : Semantics.StaticSpec :=
  .choice .entry subject sort domain disclosure

semantic_macro attachmentChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain) :
    Semantics.StaticSpec := .choice .attachment subject sort domain .openly

end Semantics.Macros.Primitives.StaticSpec

namespace Semantics.Macros.Primitives.GameEvent

semantic_macro isDealtDamage (kind : DamageKind) (subject : NounPhrase) : Semantics.GameEvent :=
  .damage kind none (some subject)

semantic_macro dealsDamage (kind : DamageKind) (source : NounPhrase) (patient : Option NounPhrase) :
    Semantics.GameEvent := .damage kind (some source) patient

end Semantics.Macros.Primitives.GameEvent

namespace Semantics.Macros.Primitives.GameEvent

/-- A battlefield-to-graveyard transition observed before departure [CR#700.4,603.10a]. -/
semantic_macro dies (subject : NounPhrase) : Semantics.GameEvent :=
  .zoneChange subject (some (.zones [.zone .battlefield .bare]))
    (some (.zone .graveyard .bare)) .before

semantic_macro leaves (subject : NounPhrase) (from_ : Option EventSource) : Semantics.GameEvent :=
  .zoneChange subject from_ none .before

semantic_macro enters (subject : NounPhrase) (from_ : Option EventSource) : Semantics.GameEvent :=
  .zoneChange subject from_ (some (.zone .battlefield .bare)) .after

/-- Observation follows the pattern, not an individual occurrence's origin. In particular,
"from anywhere" is not a leaves-the-battlefield trigger [CR#603.6c,603.10a]. -/
semantic_macro putInto (subject : NounPhrase) (destination : ZoneExpr) (from_ : Option EventSource) :
    Semantics.GameEvent :=
  let before : Bool := match from_ with
    | some (.zones zs) => !zs.isEmpty && zs.all (fun z =>
        z.sort == .battlefield || z.sort == .graveyard ||
          ((destination.sort == .hand || destination.sort == .library) &&
            (z.sort == .exile || z.sort == .stack || z.sort == .command)))
    | _ => false
  .zoneChange subject from_ (some destination) (if before then .before else .after)

end Semantics.Macros.Primitives.GameEvent

namespace Semantics.Macros.Primitives.StaticSpec

semantic_macro partScope (part : TurnPart) (whose : Option NounPhrase) (spec : Semantics.StaticSpec) :
    Semantics.StaticSpec := .conditional spec (.duringPart part whose) .asLongAs

end Semantics.Macros.Primitives.StaticSpec
