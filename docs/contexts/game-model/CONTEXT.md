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

**Token**:
A marker used to represent a Permanent that is not represented by a card
([CR#111.1]). A Token is an Object but never a Card ([CR#108.2b]).

**Emblem**:
A marker in the command zone representing an Object that has one or more
Abilities and usually no other characteristics ([CR#114.1]).

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
A Characteristic of an Object governed by [CR#205.2]. Card
Types determine which Subtype families are applicable, while Supertypes form a
separate independent axis ([CR#205.3c,205.4b]).
_Avoid_: Type

**Supertype**:
A Characteristic of an Object governed by [CR#205.4], an axis independent of
Card Type and Subtype.

**Subtype**:
A Characteristic of an Object correlated with a Card Type as specified by
[CR#205.3].
_Avoid_: Type Line as a model concept; [CR#205.1] uses it only for the printed
region of a card, and [CR#109.3] lists Card Type, Subtype, and Supertype as three
separate Characteristics

**Spell Type**:
A Subtype shared by instants and sorceries ([CR#205.3k]). It does not mean
"instant or sorcery," castability, or whether an Object is currently a Spell.

**Characteristic**:
One of the properties enumerated by [CR#109.3], such as name, mana cost, color,
types, rules text, abilities, or power, toughness, loyalty, and defense. State
such as tappedness, targets, ownership, control, and attachments is not a
Characteristic.

**Color**:
One of white, blue, black, red, and green ([CR#105.1]). An Object is the Color
or Colors of the mana symbols in its mana cost unless a color indicator or a
characteristic-defining ability says otherwise ([CR#105.2]); colorless is not a
Color ([CR#105.4]).

**Status**:
A Permanent's physical state in four categories, each with two values:
tapped/untapped, flipped/unflipped, face up/face down, and phased in/phased out
([CR#110.5]). Status is not a Characteristic, and only Permanents have one
([CR#110.5a,110.5d]).

**Counter**:
A marker placed on an Object or Player that modifies its characteristics and/or
interacts with a rule, Ability, or Effect ([CR#122.1]). Counters are not Objects
and have no characteristics; a Counter is not a Token.

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

**Keyword Action**:
One of the specialized verbs whose meaning a game rule supplies rather than
ordinary English ([CR#701.1]). A Keyword Action names something done; a Keyword
Ability names a quality an Object has. The set is open: a set release adds one,
so a Keyword Action is named by its declared label, never by a constructor.

**Deed**:
Something a card's text says is done, from one of exactly three sources: the
core rules (a Core Deed), a Keyword Action ([CR#701.1]), or the verb a Keyword
Ability defines for itself — crewing, saddling, phasing in
([CR#702.122b,702.171a,702.26a]). Deed is what a rule about "what is done"
ranges over ("can't attack or block"); Action is the wider notion that also
covers what the game does with no text naming it.
_Avoid_: verb for the whole union — a verb is the word, a Deed is what it names

**Core Deed**:
A Deed the core rules define outside [CR#701.1]: attacking and blocking as
Turn-Based Actions ([CR#508.1,509.1]), unlocking and fully unlocking a Room's
door ([CR#709.5f,709.5i]), and drawing, targeting, copying, spending and the
rest, each defined in its own rules section. The set is closed — no set release
adds one — so a rule may match a Core Deed structurally instead of by name.

**Special Action**:
An Action a Player may take while they have priority that does not use the stack
([CR#116.1]). The game does not generate it, which is what separates it from a
Turn-Based Action and a State-Based Action.

**Turn-Based Action**:
An Action that happens automatically when a Step or Phase begins, or when one
ends, without using the stack ([CR#703.1]). An Ability that watches for the same
moment is a Triggered Ability instead ([CR#703.1a]).

**State-Based Action**:
An Action that happens automatically whenever one of the conditions listed by
[CR#704.1] is met, without using the stack. An Ability that watches for a game
state is a Triggered Ability instead ([CR#704.1a]).

**State-Based Cause**:
A condition whose satisfaction causes a State-Based Action ([CR#704.1]).
Having 0 or less life, for example, causes a Player to lose the game
([CR#704.5a]); an exemption limited to that cause does not exempt the Player
from losing for other reasons.

**Priority**:
The system determining which Player may cast spells, activate abilities, and
take Special Actions at a given time ([CR#117.1]).

**Turn**:
The unit of turn structure made of five Phases in order — beginning, precombat
main, combat, postcombat main, and ending — each of which takes place even if
nothing happens during it ([CR#500.1]).

**Phase**:
One of the five parts of a Turn; the beginning, combat, and ending Phases are
further broken down into Steps ([CR#500.1]).

**Step**:
An ordered subdivision of the beginning, combat, or ending Phase ([CR#500.1]). A
Phase or Step in which Players receive priority ends when the stack is empty and
all Players pass in succession ([CR#500.2]).

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

**Damage**:
Harm an Object deals to a battle, creature, planeswalker, or Player
([CR#120.1]). The Object that deals it is the Source of that Damage, and Damage
cannot be dealt to any other Object ([CR#120.1a]).

**Life Total**:
The number each Player begins the game with ([CR#119.1]), adjusted whenever an
Effect causes that Player to gain or lose life ([CR#119.3]). Setting a total to
a number resolves as the gain or loss needed to reach it ([CR#119.5]).

**Mana**:
The primary resource of the game, spent to pay costs ([CR#106.1]). There are
five colors of Mana and six types of it, colorless included
([CR#106.1a,106.1b]).

**Cost**:
An action or payment necessary to take another action, or to stop another action
from taking place ([CR#118.1]).

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

**Attachment**:
An Aura, Equipment, or Fortification in its relationship to the Object or
Player it is attached to ([CR#701.3a]).

**Attachment Host** (project term):
The Object or Player to which an Attachment is attached ([CR#701.3a]). An
Aura may enchant an Object or Player; Equipment and Fortifications have their
respective creature and land host restrictions ([CR#303.4b,301.5,301.6]).

**Card Face Side** (project term):
The front or back position of a double-faced permanent's face ([CR#701.27g]),
distinct from its face-up or face-down Status ([CR#110.5]).

**Copy**:
An Object that has acquired the copiable values of another Object's
characteristics and, for an Object on the stack, the choices made when casting
or activating it ([CR#707.2]). Other effects, status, counters, and stickers
are not copied.

## Engine semantic language

**Semantic Macro** (project term):
A named, trusted expansion into semantic expressions. Its name provides an
attachment point for spellings; its expansion supplies the modeled meaning.
Checking the expansion's admissibility does not establish its rules fidelity.
_Avoid_: treating a deed label alone as its meaning

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

**Reference Scope** (project term):
The mentions available to a Reference under its containing grammatical
construction. An ordinary pronoun requires a unique compatible antecedent in
that scope; an explicit shared-subject reference belongs to its owning clause.
A historical-event predicate binds the entity it describes locally; a nested
historical-event predicate establishes its own scope.
_Avoid_: author-chosen numeric stack depth

**Referent Sort** (project term):
The domain classification required of an Entity denoted by a Reference or
contained in a Selection. It is two axes, not one: the Entity domain — Player,
Object, or either ([CR#102.1,109.1]) — optionally narrowed by an Object Class
or a Card Type. A flat single list conflates them, because only an Object has
an Object Class.

**Selection** (project term):
A semantic expression that denotes a collection of values from an explicit
domain. An Entity Selection contains Objects and/or Players.
_Avoid_: Reference for a collection

**Predicate** (project term):
A truth criterion evaluated against a candidate from a known domain.

**Type Evidence** (project term):
Known Card Type facts retained for reference resolution. Multiple facts can
hold for one Object [CR#205.2b]; an alternative retains only shared facts.
Remembered facts are separate from the current Object Class and Zone used to
resolve a reference, and do not simulate changing characteristics.

**Filter** (project term):
The operation of retaining candidates that satisfy a Predicate.
_Avoid_: Predicate as an interchangeable noun

**Instruction** (project term):
Executable semantic syntax describing work to perform. Executing an
Instruction may cause Actions, Events, and Effects, but the Instruction is none
of those things ([CR#609.1] — text itself is never an Effect).
_Avoid_: Effect for executable syntax; One-Shot Effect as its name

**Static Spec** (project term):
Declarative semantic syntax stating what holds while its carrier is in force.
It is the payload a Static Ability states, and the part a one-shot puts in
force for a duration ([CR#611.2]); like an Instruction it is syntax, so it is
not itself an Effect — the Continuous Effect is what applying it establishes.
_Avoid_: Static Effect

**Amount** (project term):
A semantic expression denoting a number in context — a literal, a count over a
Selection, or a characteristic read. An Amount is not a Reference.
_Avoid_: Quantity for a denoted number

**Quantity** (project term):
The count constraint a determiner states over a Selection — "one", "up to two",
"one or more", "any number". A Quantity bounds how many members a Selection has;
it is not an Amount.
_Avoid_: Amount for a cardinality constraint

**Delta** (project term):
A change stated against a numeric property: up by an Amount, down by an Amount,
or set to an Amount. The Delta states the change; the engine derives the Event
it causes — [CR#613.4c] for a characteristic modification, [CR#119.3] for life.
_Avoid_: separate gain/lose and raise/lower vocabularies per property

**Duration** (project term):
The span a Continuous Effect created by a one-shot lasts, and the span for which
a Static Spec is put in force. A stated Duration is what ends the Effect; with
none stated it lasts until the game ends ([CR#611.2a]).
_Avoid_: span, window

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

**Designation** (project term):
A named marker an Object, a Player, or the game can have, which rules and
Effects identify without it being an Ability or a copiable value. The CR names
each Designation on its own — goaded ([CR#701.15b]), the city's blessing
([CR#702.131c]) — rather than defining the class.

**Designation Conferrer** (project term):
A Keyword Ability, Keyword Action, or Core Deed whose expansion gives a
Designation. One conferrer may give several Designations: space sculptor gives
sector designations ([CR#702.158a]), and unlocking gives the corresponding
door's unlocked designation ([CR#709.5f]).

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
