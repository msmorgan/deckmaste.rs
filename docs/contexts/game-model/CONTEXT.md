# Game Model

The canonical vocabulary for Deckmaste's executable model of Magic. It combines
Magic rules terms with project-defined concepts needed to represent them.

## Authority

Magic rules terms inherit their meanings from the Comprehensive Rules (CR),
which controls if a compressed definition here ever disagrees with it.

## Magic rules language

**Player**:
One of the people in the game ([CR#102.1]). A Player is not an Object.

**Object**:
An ability on the stack, a card, a copy of a card, a token, a spell, a
permanent, or an emblem ([CR#109.1]).
_Avoid_: Entity when the specifically rules-defined Object is meant

**Card**:
A Magic card or an Object represented by a Magic card ([CR#108.2]); a Token is
not a Card ([CR#108.2b]).

**Spell**:
A card, copy of a card, or copy of a spell on the stack as specified by
[CR#112.1..112.1b]. Spell is a current rules role, not a Card Type.

**Permanent**:
A card or token on the battlefield ([CR#110.1]). Permanent is a current rules
role, not a Card Type.

**Permanent Type**:
Artifact, battle, creature, enchantment, land, or planeswalker
([CR#110.4]).

**Permanent Card**:
A card that could be put onto the battlefield: specifically an artifact,
battle, creature, enchantment, land, or planeswalker card ([CR#110.4a]).

**Permanent Spell**:
A spell that will enter the battlefield as a permanent as part of resolving:
specifically an artifact, battle, creature, enchantment, or planeswalker spell
([CR#110.4b]). A Land is therefore a Permanent Type but not a Permanent Spell.

**Card Type**:
A classification appearing on the Type Line and governed by [CR#205.2]. Card
Types determine which Subtype families are applicable, while Supertypes form a
separate independent axis ([CR#205.3c,205.4b]).
_Avoid_: Type

**Supertype**:
A classification printed before the Card Types on a Type Line and governed by
[CR#205.4].

**Subtype**:
A classification printed after the dash on a Type Line and correlated with a
Card Type as specified by [CR#205.3].

**Spell Type**:
A Subtype shared by instants and sorceries ([CR#205.3k]). It does not mean
"instant or sorcery," castability, or whether an Object is currently a Spell.

**Type Line**:
The line containing a card's Card Types and any Supertypes and Subtypes
([CR#205.1]).

**Characteristic**:
One of the properties enumerated by [CR#109.3], such as name, mana cost, color,
types, rules text, abilities, or power, toughness, loyalty, and defense. State
such as tappedness, targets, ownership, control, and attachments is not a
Characteristic.

**Ability**:
A rules-defined quality of an Object or Player, or an activated or triggered
ability on the stack, as distinguished by [CR#113.1]. Abilities generate
Effects; they are not themselves Effects.

**Spell Ability**:
An Ability followed as instructions while an instant or sorcery Spell resolves
([CR#113.3a]).

**Activated Ability**:
An Ability expressed as a cost followed by an effect and activated by a Player
([CR#113.3b]). It normally uses the stack; an Activated Mana Ability resolves
immediately instead ([CR#605.3b]).

**Triggered Ability**:
An Ability with a trigger condition that triggers when its Event occurs or, for
a state trigger, when its game-state condition becomes true
([CR#113.3c,603.8]). It normally uses the stack; a Triggered Mana Ability
resolves immediately instead ([CR#605.4a]).

**Mana Ability**:
An Activated Ability or Triggered Ability that meets the criteria in
[CR#605.1]. Mana is an orthogonal classification, not a fifth general Ability
category.

**Static Ability**:
An Ability written as a statement that is simply true and may create continuous
Effects while active ([CR#113.3d,604.1,609.1]).
_Avoid_: Static Effect

**Keyword Ability**:
An Ability represented in rules text by a keyword whose game rule supplies its
meaning ([CR#702.1]). One keyword name may represent multiple component
Abilities.

**Intrinsic Ability**:
An Ability that the CR itself calls intrinsic, notably the mana Ability an
Object has from a basic land type ([CR#305.6]).
_Avoid_: Intrinsic as a synonym for engine-primitive

**Effect**:
Something that happens in the game as a result of a Spell or Ability
([CR#609.1]). Rules text and executable syntax are not themselves Effects.

**One-Shot Effect**:
An Effect that does something once and has no duration ([CR#610.1]).

**Continuous Effect**:
An Effect that modifies characteristics or control, or affects Players or the
rules, for a fixed or indefinite period ([CR#611.1]).

**Replacement Effect**:
Usually a Continuous Effect that watches for an Event and replaces it wholly or
partly with another Event ([CR#614.1]). A self-replacement effect is the
explicit exception described by [CR#614.15].

**Prevention Effect**:
A Continuous Effect that watches for a damage Event and prevents some or all of
that damage ([CR#615.1]).

**Event**:
Anything that happens in a game ([CR#700.1]). Whether one happening is one
Event or several depends on the observer defined by the relevant rule or text.

**Action**:
Something a Player or the game does. Keyword Actions are the specialized verbs
defined by [CR#701]. Action names what is done; Event names its occurrence as
grouped by the relevant observer.

**Choice**:
A rules procedure in which an allowed option or value is selected. Choice names
the rules-level selection, not the engine boundary that waits for an agent.

**Target**:
An Object or Player a Spell or Ability has identified to affect as a Target.
Targets are initially chosen while putting that Spell or Ability on the stack
([CR#115.1]) and may later change only as [CR#115.7] permits.

**Zone**:
A rules-defined place where Objects can be during a game ([CR#400.1]). Players
are not in Zones.

**Owner**:
The Player determined for a card by [CR#108.3] and for other Objects by the
applicable CR rules. Ownership is not a Characteristic.

**Controller**:
The Player who currently controls an Object, Spell, or Ability
([CR#108.4,110.2,112.2,113.8]), or—when a rule permits it—another Player
([CR#723.1]). The contextual rules for "you" include [CR#109.5]; control is not
a Characteristic.

**Source**:
An Object's contextual relationship as the source of an Ability, damage, or
mana ([CR#109.2c]). Source is not an intrinsic Object class.

**Card Face**:
A set of printed characteristics that the CR calls a card face. Split cards
have two faces on one side ([CR#709.1]); a double-faced card has a Magic card
face on one side and a Magic card face or half an oversized face on the other
([CR#712.1]). Flip and adventurer cards have an ordinary Card Face plus
Alternative Characteristics rather than a second face ([CR#710.1,715.2]).

**Alternative Characteristics**:
A non-face set of characteristics used in a rules-defined situation, such as a
flip card's upside-down characteristics or an adventurer card's inset Adventure
characteristics ([CR#710.1,715.2]).

## Engine semantic language

**Entity** (project term):
An addressable game-state thing. Deckmaste's current Entity domain contains
exactly CR Objects and Players; the project term may expand if the game model
later requires another addressable kind, and an Entity is not necessarily a CR
Object.

**Combatant** (project term):
A Permanent to which the creature-like combat rules apply. Its power governs
combat and fight damage, marked damage is compared with its toughness, it is
in the rules domains for attacking, blocking, and fighting, and the
continuous-control rule informally called summoning sickness applies to its
attacks and activated abilities with the tap or untap symbol
([CR#120.3d,120.3e,302.6,508.1a,509.1a,510.1a,701.14a,704.5g]). Whether a
Combatant may perform a particular action is a separate legality question, so
a restriction on one action does not remove the role. An “as though” effect may
treat a Permanent as a Combatant for only its stated purpose without conferring
the role for other purposes ([CR#609.4]).
_Avoid_: defining Combatant as a Permanent that currently may attack or block

**Card Form** (project term):
The structural arrangement by which a physical card supplies Card Faces and,
where applicable, Alternative Characteristics.

**Object Class** (project term):
One of the overlapping ways [CR#109.1] classifies an Object, such as Card,
Token, Spell, Permanent, Emblem, or an Ability on the stack.
_Avoid_: Object Kind, substrate

**Reference** (project term):
A singular semantic expression that denotes one Object or Player in context.
Amounts and collections are not References.

**Referent Sort** (project term):
The domain classification required of an Entity denoted by a Reference or
contained in a Selection, such as Player, Card, Spell, Permanent, or an Object
of a particular Card Type.

**Selection** (project term):
A semantic expression that denotes a collection of values from an explicit
domain. An Entity Selection contains Objects and/or Players.
_Avoid_: Reference for a collection

**Predicate** (project term):
A truth criterion evaluated against a candidate from a known domain.

**Filter** (project term):
The operation of retaining candidates that satisfy a Predicate.
_Avoid_: Predicate as an interchangeable noun

**Instruction** (project term):
Executable semantic syntax describing work to perform. Executing an
Instruction may cause Actions, Events, and Effects, but the Instruction is none
of those things.
_Avoid_: Effect for executable syntax

**Decision Point** (project term):
An engine boundary at which progress waits for an external agent to supply a
Decision.

**Decision** (project term):
The response submitted at a Decision Point. A Decision may encode a rules
Choice or a proposed Action.

**Conferral** (project term):
The relationship by which a rule, Card Type, or Subtype supplies an Ability or
other rules property. A conferred Ability remains an ordinary Ability unless
the CR says otherwise.

**Primitive Keyword Ability** (project term):
A Keyword Ability whose semantics are represented directly rather than
decomposed into component Abilities.
_Avoid_: Intrinsic Keyword Ability

**Composite Keyword Ability** (project term):
A named Keyword Ability represented as an ordered collection of component
Abilities. Its components remain structural parts of the one named Keyword
Ability rather than independent entries in the visible ability list.

**Execution Frame** (project term):
The evaluation context that supplies bindings needed to interpret References
and Instructions.
_Avoid_: Frame when the qualification matters
