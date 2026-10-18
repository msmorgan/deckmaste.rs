import Semantics.Macros.CharacteristicInputs
import Semantics.Check.Phrase

/-! Named macros for exposed primitives and fixed wording. Generated wrappers preserve
constructor signatures; the worded forms below expand into shared operations. The authoring
boundary records their names. -/

declare_semantic_primitives

/-- An entry rider is a move rider as written: the RON mirror reads a bare entry rider at a move
rider's place (`#[macro_ron(embed)]`), and the bench writes it the same way. -/
instance : Coe Semantics.EntryRider Semantics.MoveRider := ⟨Semantics.Macros.Primitives.MoveRider.entry⟩

namespace Semantics.Macros.Primitives.Action

/-- A numeric definition at instruction position uses the shared static definition. -/
semantic_template define (letter : Letter) (amount : Amount) : Semantics.Action :=
  .establish (.letterDefinition letter amount) none

semantic_template offer (body : Semantics.Action) (ifDid ifNot : Option Semantics.Action) :
    Semantics.Action :=
  .withContinuation .optional body ifDid ifNot

semantic_template doIfDone (body : Semantics.Action) (ifDid ifNot : Option Semantics.Action) :
    Semantics.Action := .withContinuation .required body ifDid ifNot

semantic_template repeatTimes (times : Amount) (body : Semantics.Action) : Semantics.Action :=
  .repeat_ (.fixed times body)

end Semantics.Macros.Primitives.Action

namespace Semantics.Macros.Primitives.NounPhrase

semantic_template both (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .and [left, right]
semantic_template eitherOf (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .or [left, right]

end Semantics.Macros.Primitives.NounPhrase

namespace Semantics.Macros.Primitives.Cost

semantic_template either (left right : Semantics.Cost) : Semantics.Cost := .or [left, right]

end Semantics.Macros.Primitives.Cost

namespace Semantics.Macros.Primitives.Predicate

semantic_template isAttached (word : Option AttachWord) : Semantics.Predicate :=
  .attachment .host word none

semantic_template attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Semantics.Predicate :=
  .attachment .host word (some by_)

semantic_template attachedTo (host : NounPhrase) : Semantics.Predicate :=
  .attachment .attachment none (some host)

/-- A transformed permanent is the whole double-faced permanent with its back face up. -/
semantic_template isTransformed : Semantics.Predicate := .currentFace .back

end Semantics.Macros.Primitives.Predicate

namespace Semantics.Macros.Primitives.StaticSpec

semantic_template qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    Semantics.StaticSpec := .characteristicChange subject (payload.edits op)

semantic_template abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : Semantics.StaticSpec :=
  .characteristicChange subject [.removedAbilities (.specified abilities)]

semantic_template allAbilityLoss (subject : NounPhrase) (except : Option Semantics.Predicate) :
    Semantics.StaticSpec := .characteristicChange subject [.removedAbilities (.allExcept except)]

end Semantics.Macros.Primitives.StaticSpec

namespace Semantics.Macros.Primitives.CopyExcept

semantic_template types (types : List CardType) (subtypes : List Subtype) : Semantics.CopyExcept :=
  .edits [.typeLine .adds (({ types := types, subtypes := subtypes } :
    Characteristics).typeChanges)]

semantic_template name (name : String) : Semantics.CopyExcept := .edits [.name .sets name]
semantic_template pt (power toughness : Amount) : Semantics.CopyExcept :=
  .edits [.stat .sets .power power, .stat .sets .toughness toughness]
semantic_template nonlegendary : Semantics.CopyExcept :=
  .edits [.typeLine .loses { supertypes := some [.legendary] }]
semantic_template color (color : Color) : Semantics.CopyExcept := .edits [.colors .sets (.some [color])]

end Semantics.Macros.Primitives.CopyExcept

namespace Semantics.Macros.Primitives.Action

semantic_template removeFromCombat (subject : NounPhrase) : Semantics.Action :=
  .combat subject (.participation .outsideCombat)

semantic_template becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Semantics.Action :=
  .combat subject (.participation (.attacking defender))

semantic_template becomeBlocking (subject blocked : NounPhrase) : Semantics.Action :=
  .combat subject (.blocking .attached blocked)

semantic_template stopBlocking (subject blocked : NounPhrase) : Semantics.Action :=
  .combat subject (.blocking .unattached blocked)

semantic_definition attachTo (subject host : NounPhrase) : Semantics.Action :=
  .ownScope (some (.action "attach"))
    (.enact (.action "attach") (.attachment .attached (.inCaller (.firstMention "attachTo.subject" subject)) (some (.inCaller (.firstMention "attachTo.host" host)))) none none)

semantic_template unattach (subject : NounPhrase) : Semantics.Action :=
  .attachment .unattached subject none

end Semantics.Macros.Primitives.Action

namespace Semantics.Macros.Primitives.Action

semantic_definition create (count : Amount) (token : TokenSpec) (riders : List EntryRider) :
    Semantics.Action :=
  .ownScope (some (.action "create"))
    (.enact (.action "create") (.createObject (.inCaller count) (.token (.inCaller token) (riders.map EntryRider.inCaller)) true none) none none)

semantic_template getEmblem (abilities : List Ability) : Semantics.Action :=
  .createObject (.lit 1) (.emblem none abilities) true none

end Semantics.Macros.Primitives.Action

namespace Semantics.Macros.Primitives.Action

/-- Counter is one labeled stack-to-graveyard move [CR#701.6a]. -/
semantic_definition counterSpell (subject : NounPhrase) : Semantics.Action :=
  .ownScope (some (.action "counter"))
    (.enact (.action "counter")
      (.move (.inCaller (.firstMention "counterSpell.subject" subject)) (.zones [.zone .stack .bare]) (some (.zone .graveyard .bare)) .arrival none [])
      none none)

end Semantics.Macros.Primitives.Action

namespace Semantics.Macros.Primitives.StaticSpec

semantic_template entryChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    (disclosure : Disclosure) : Semantics.StaticSpec :=
  .choice .entry subject sort domain disclosure

semantic_template attachmentChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain) :
    Semantics.StaticSpec := .choice .attachment subject sort domain .openly

end Semantics.Macros.Primitives.StaticSpec

namespace Semantics.Macros.Primitives.GameEvent

semantic_template isDealtDamage (kind : DamageKind) (subject : NounPhrase) : Semantics.GameEvent :=
  .does .any (.dealDamage .anyAmount subject kind)

semantic_template dealsDamage (kind : DamageKind) (source : NounPhrase) (patient : Option NounPhrase) :
    Semantics.GameEvent := .does (.actor source) (.dealDamage .anyAmount (patient.getD .any) kind)

end Semantics.Macros.Primitives.GameEvent

namespace Semantics.Macros.Primitives.GameEvent

/-- A battlefield-to-graveyard transition observed before departure [CR#700.4,603.10a]. -/
semantic_template dies (subject : NounPhrase) : Semantics.GameEvent :=
  .does .any (.move subject (.zones [.zone .battlefield .bare])
    (some (.zone .graveyard .bare)) .departure none [])

semantic_template leaves (subject : NounPhrase) (from_ : Option EventSource) : Semantics.GameEvent :=
  .does .any (.move subject (from_.getD .anywhere) none .departure none [])

semantic_template enters (subject : NounPhrase) (from_ : Option EventSource) : Semantics.GameEvent :=
  .does .any (.move subject (from_.getD .anywhere) (some (.zone .battlefield .bare)) .arrival none [])

/-- Observation follows the pattern, not an individual occurrence's origin. In particular,
"from anywhere" is not a leaves-the-battlefield trigger [CR#603.6c,603.10a]. -/
semantic_template putInto (subject : NounPhrase) (destination : ZoneExpr) (from_ : Option EventSource) :
    Semantics.GameEvent :=
  let before : Bool := match from_ with
    | some (.zones zs) => !zs.isEmpty && zs.all (fun z =>
        z.sort == some .battlefield || z.sort == some .graveyard ||
          ((destination.sort == some .hand || destination.sort == some .library) &&
            (z.sort == some .exile || z.sort == some .stack || z.sort == some .command)))
    | _ => false
  .does .any (.move subject (from_.getD .anywhere) (some destination)
    (if before then .departure else .arrival) none [])

end Semantics.Macros.Primitives.GameEvent

namespace Semantics.Macros.Primitives.StaticSpec

semantic_template partScope (part : TurnPart) (whose : Option NounPhrase) (spec : Semantics.StaticSpec) :
    Semantics.StaticSpec := .conditional spec (.duringPart part whose) .asLongAs

end Semantics.Macros.Primitives.StaticSpec
