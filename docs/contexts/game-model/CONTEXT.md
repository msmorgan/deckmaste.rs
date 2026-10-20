# Game Model

The canonical vocabulary for Deckmaste's executable model of Magic. It combines
Magic rules terms with project-defined concepts needed to represent them.

## Authority

Magic rules terms inherit their meanings from the Comprehensive Rules (CR),
which controls if a compressed definition here ever disagrees with it.

## Magic rules language

**Player**:
One of the people in the game ([CR#102.1]). A Player is not an Object.

**Last Known Information (LKI)**:
Information about a specific Object as it last existed in the public Zone where an Effect
expected it to be. An Effect uses this information when the Object has left that Zone,
including when the Effect moved it from a public Zone to a hidden Zone ([CR#608.2h]).

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

**Predefined Token**:
A Token whose characteristics the rules themselves define, listed by name in
[CR#111.10] — Treasure, Food, Clue and the rest. An Effect creates one by that
name and may still modify or add to the defined characteristics.

**Emblem**:
A marker in the command zone representing an Object that has one or more
Abilities and usually no other characteristics ([CR#114.1]).

**Double-Faced Token**:
A Token with a front face and a back face, each a full characteristics set, as
the Incubator is ([CR#111.10i,701.53b]). It transforms as a double-faced
permanent does; it is not a Card.

**Face-Down**:
The state of a card or Object turned face down, given by an optional set of
characteristics. With none, it is a card exiled face down, which has no
characteristics and no status ([CR#406.3a,110.5d]). With a set, it is a
face-down Spell or Permanent whose listed characteristics are its copiable
values — for manifest and cloak a 2/2 creature with no name, subtypes or mana
cost ([CR#708.2,701.40a,701.58a]). Turning it face up restores its normal
copiable values ([CR#708.8]). A face-down Object is never a Token.
_Avoid_: Face-Down Characteristics, for the state; face-down token

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

**Characteristic Edit**:
A change that sets, adds, or removes a Characteristic or a value within it.
Leaving a Characteristic unchanged differs from explicitly setting an empty
value, such as making an Object colorless ([CR#109.3,105.4]).

**Copy Exception**:
A qualification of a copy effect that changes what is copied or adds an
ability or entering-the-battlefield consequence to the copy. Its characteristic
changes participate in determining copiable values, including whether a
characteristic-defining ability is copied ([CR#707.9d]).

**Ability Selection**:
The abilities an effect identifies individually, by an ability family, as
all abilities except a stated selection, or as the abilities printed below a
keyword's line (backup's "the non-backup abilities of this creature printed
below this one", [CR#702.165a,702.165c]).

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
and have no characteristics; a Counter is not a Token. Counters with the same
name are interchangeable ([CR#122.1]), so a Counter's kind is its name: the
name of the counter declaration that defines it, verbatim (`p1p1Counter`,
`chargeCounter`), for a +X/+Y counter and a keyword counter as much as for any
other ([CR#122.1a,122.1b]). What a Counter of the kind does is its Registry
Definition.

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

**Ability Category**:
Which of the four general kinds an Ability is: Spell Ability, Activated
Ability, Triggered Ability, or Static Ability ([CR#113.3]). A Keyword Ability's
definition is written in the latter three, and in a Spell Ability where the
keyword represents one on an instant or sorcery Spell, guarded by that Card
Class ([CR#702.131a,702.174b]).
_Avoid_: Ability Class, which names the families a grant or a removal selects

**Stack Regime** (project term):
When an Ability matters relative to a Spell on the stack: as the Spell is
cast, or as it resolves; an Ability with neither functions off the stack. Each
component Ability of a Keyword Ability's definition carries its own, the one
its keyword's rule gives it: kicker's additional cost is cast-time
([CR#702.33a]), offspring's second Ability triggers on the battlefield
([CR#702.175a]), casualty's on the stack ([CR#702.153a]), and an instant or
sorcery Spell's Spell Ability is followed as it resolves ([CR#113.3a]). A
keyword's own regime, read where it is granted to a Spell, is the first among
its components'.
_Avoid_: Regime alone where a number's clamped/signed regime could be meant

**Keyword Ability**:
An Ability represented in rules text by a keyword whose game rule supplies its
meaning ([CR#702.1]). One keyword name may represent multiple component
Abilities. A Keyword Ability is named by its declaration's name (`flying`,
`firstStrike`), never by the phrase a card prints ("first strike"): a card writes
that name bare wherever a keyword is wanted, and a name no declaration gives is
refused when the card loads. Where the printed keyword fixes part of its meaning
in its own words, each printed form is a declaration of its own that carries the
keyword's name: "Gift a card" is `giftACard`, which is gift with the effect "the
chosen player draws a card" ([CR#702.174a,702.174e]). Bare `gift` writes no
spelling, so no card line reads as it.

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

**Zone Change**:
An Object's transition from one Zone to another ([CR#400.7]). The Object in the
new Zone normally has no memory of or relation to its previous existence.

**Becomes** (project term):
The new Object an Object turns into at its next Zone Change, named before
the change happens: "the permanent this spell becomes" ([CR#400.7]). It is
how a Spell's text reaches the Permanent it will be.
_Avoid_: it, for the successor (the spell itself)

**Observation Point** (project term):
The moment immediately before or after an Event at which its matching Objects
and their characteristics are observed ([CR#603.10,603.10a]). A trigger watching
arrival from anywhere differs from one watching battlefield departure even
when both match the same occurrence ([CR#603.6c]).

**Action**:
Something a Player, a Permanent or the game does. Card text describes one
Action whether it has it carried out, pays it as a cost, watches for its
occurrence or forbids it; Keyword Actions are the specialized verbs it uses
([CR#701.1]). Carrying out an Action may cause Events and Effects, but the Action
is neither: Event names its occurrence as grouped by the relevant observer, and
text itself is never an Effect ([CR#609.1]).
_Avoid_: Instruction; Effect, or One-Shot Effect, for an Action text names

**Keyword Action**:
One of the specialized verbs whose meaning a game rule supplies rather than
ordinary English ([CR#701.1]). A Keyword Action names something done; a Keyword
Ability names a quality an Object has. The set is open: a set release adds one,
so a Keyword Action is named by its declaration's name (`exile`, `collectEvidence`),
never by a constructor: a card writes that name bare wherever a Deed is wanted, and a name
no declaration gives is refused when the card loads.

**Deed**:
Something a card's text says is done, from one of exactly three sources: the
core rules (a Core Deed), a Keyword Action ([CR#701.1]), or the verb a Keyword
Ability defines for itself — crewing, saddling, phasing in
([CR#702.122b,702.171c,702.26a]). Deed is what a rule about "what is done"
ranges over ("can't attack or block"); Action is the wider notion that also
covers what the game does with no text naming it.
_Avoid_: verb for the whole union — a verb is the word, a Deed is what it names

**Core Deed**:
A Deed the core rules define outside [CR#701.1]: attacking and blocking as
Turn-Based Actions ([CR#508.1,509.1]), unlocking a Room's door and fully
unlocking a Room ([CR#709.5f,709.5i]), and drawing, targeting, copying, spending and the
rest, each defined in its own rules section. The set is closed — no set release
adds one — so a rule may match a Core Deed structurally instead of by name.

**Crewing**:
A creature's participation in paying a Vehicle's crew cost by being tapped
([CR#702.122b]). A restriction against that creature crewing Vehicles prevents
using it for that payment ([CR#702.122d]).

**Saddling**:
A creature's participation in paying a Permanent's saddle cost by being tapped
([CR#702.171c]).

**Phasing In**:
A Permanent's status changing to phased in, after which the game treats the
Permanent as existing again ([CR#702.26c]).

**Fully Unlocking**:
A Permanent with a shared type line gaining its other unlocked designation
while it already has one, or gaining both while it has neither ([CR#709.5i]).

**Rules Procedure**:
A procedure the rules define for activating an Ability, casting a Spell or
playing a card ([CR#601.2,602.1,701.18b]). Card text may have a Player
carry one out during resolution ("you may cast that card without paying its
mana cost"); a permission to do so over a duration ("until end of turn, you
may cast that card") grants the procedure rather than carrying it out.
_Avoid_: action, for the procedure itself — the Action is the Deed it names

**Special Action**:
An Action a Player may take while they have priority that does not use the stack
([CR#116.1]). The game does not generate it, which is what separates it from a
Turn-Based Action and a State-Based Action. One that an ability or effect
grants is a cost paid and an Action carried out, any time its Player has
priority within an optional Timing — foretell's is paying {2} and exiling the
card face down during its owner's turn ([CR#116.2c,116.2h]).

**Off-Stack Static** (project term):
A Static Ability that carries out an Action without using the stack,
either as an Event happens ("as it becomes night, if this permanent is front
face up, transform it", [CR#702.145b]) or at any time a condition holds ("any
time you control ten or more permanents and you don't have the city's
blessing, you get the city's blessing", [CR#702.131b]). Text that acts as a
Permanent enters ("as this enters, you may sacrifice …") is a Replacement
Effect instead ([CR#614.1c]). A Special Action is its sibling, done by choice
with a cost; a delayed trigger an Off-Stack Static creates has that static's
object as its source ([CR#603.7g]).
_Avoid_: state trigger, since nothing goes on the stack ([CR#604.1])

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

**Rules Table** (project term):
A set of rules-defined rows a plugin authors as data — State-Based Actions,
Conferrals, Damage Results — each row scoped by a Predicate over the Objects or
Players it applies to, evaluated generically rather than recognized by name.
_Avoid_: Registry for a Rules Table; a registry declares a namespace's members,
a Rules Table states what the rules do to what it scopes.

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

**Linked Abilities**:
Two abilities printed on one Object where the second refers only to what the
first did, chose, or allowed to be paid, and not to any other ability's
([CR#607.1,607.2d,607.2i]). Kicker's cost and its "if it was kicked" ability are
linked ([CR#702.33e]), as are gift's "you may choose an opponent" cost and the
ability that hands its effect to the chosen Player ([CR#702.174a,702.174b]).
_Avoid_: reading "the chosen player" as any Choice in scope

**Linked Facet** (project term):
What a Linked Ability reads back about its Link Source: the colors or amount
of mana spent, the number of times a cost was paid, whether it was paid
([CR#702.33d,702.44a,601.2h]), the Player chosen as it was paid
([CR#702.174a,607.2d]), the Objects its deed acted on ("the exiled cards",
"creatures it devoured", [CR#607.2a,702.82b]), or whether an offered choice
was taken ("if tribute wasn't paid", [CR#702.104b]).
_Avoid_: paid-cost facet, since not every source is a cost; payment record
(the engine's store, not the reference)

**Link Source** (project term):
The printed ability or cost a Linked Facet reads back: a cost named by its
Keyword Ability, "the additional" or "the alternative" cost, or a printed
ability named by what it does ("the exiled card" names the ability that
exiled it, [CR#607.2a]).
_Avoid_: paid cost name

**Target**:
An Object or Player a Spell or Ability has identified to affect as a Target.
Targets are initially chosen while putting that Spell or Ability on the stack
([CR#115.1]) and may later change only as [CR#115.7] permits.

**Zone**:
A rules-defined place where Objects can be during a game ([CR#400.1]). Players
are not in Zones.

**Pile**:
An ordered group of cards grouped together in a zone, never itself an Object
([CR#700.3,700.3b]): a library (face down, [CR#401.2]), a graveyard (face up,
[CR#404.2]), or a pile an effect makes, such as face-down cards in exile
([CR#406.4]). Grouping does not move the cards or change their order
([CR#700.3c]). A Pile is face down or face up, and that face, not the zone,
decides who may look at it.
_Avoid_: Object, for the pile itself

**Slice**:
A number of Objects counted from one end of a Pile, "the top three cards of
your library": an ordered group of the pile's Objects, still in its zone
([CR#700.3c]).
_Avoid_: library slice, since a graveyard or a made pile has slices too

**Run**:
The Objects taken one at a time from one end of a Pile until the first that
matches a description, that one included ([CR#701.57a,702.85a]). An ordered
group, the stopping Object a part of it.
_Avoid_: library run

**Location** (project term):
Where Objects are, or where a move takes them from or puts them: a Zone,
possibly a Player's own ([CR#400.1]), a place counted from one end of a Pile
("seventh from the top", [CR#401.7]), the cards exiled with an Object, a Pile
an Effect made, anywhere, or the complement or union of Locations ("from
anywhere other than the battlefield").
_Avoid_: zone expression, event source, search scope, for the one notion

**Entry Rider**:
How a Permanent arrives on the battlefield, said with what puts it there:
tapped, attacking, transformed, under a Player's control, with counters, as a
copy, or face down ([CR#614.1c,708.3]). It rides a move to the battlefield, a
Token's creation, or a static ability ("this enters tapped"), and nothing
else; an arrival in any other zone is a Pile Placement.
_Avoid_: token rider, since most arrivals are not a Token's

**Pile Placement**:
How a moved card is placed in a zone other than the battlefield: its order and
depth in a Pile ("in any order", "second from the top", [CR#401.4,401.7]), its
face ("exile it face down", [CR#406.3]), and the counters it is exiled with
([CR#702.62a]). A move carries the Entry Riders or the Pile Placements its
destination allows, never the other kind.
_Avoid_: rider, for a placement; library position

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

**Damage Result**:
One of the outcomes Damage has, chosen by whether its recipient is a Player or
a Permanent and by the characteristics of its Source and recipient
([CR#120.3]). One Damage event may have several results at once — a Source with
lifelink adds life gain to whatever else the Damage does ([CR#120.3f]) — so a
result is additive, not a replacement of the Damage.

**Marked Damage**:
Damage retained on a Permanent until it is removed, including if the Permanent
stops being a Creature. Regeneration and cleanup remove all damage marked on
a Permanent ([CR#120.6]).

**Regeneration**:
The replacement of a Permanent's destruction by removing its marked Damage,
having its Controller tap it, and removing it from combat when applicable.
Creating a shield that may later perform this replacement is not itself
Regeneration ([CR#701.19a..701.19c]).

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

**Mana Substitute**:
An action on an Object a Player may take rather than pay one generic or colored
mana of a cost, the paid mana read from the acted-on Object — convoke, delve,
improvise and waterbend ([CR#702.51a,702.66a,702.126a,701.67a]). On a Spell it
applies after the total cost is locked and is neither an additional nor an
alternative cost ([CR#702.51b]); on a named cost it pays only that cost's
generic mana ([CR#701.67b]).
_Avoid_: alternative cost, additional cost

**Mana Cost Term** (project term):
A mana cost a cost change is stated in symbols of: a printed amount, or the
mana cost of an Object ("reduced by the sacrificed permanent's mana cost",
[CR#702.48a,702.48c]). Its symbols reduce or raise the matching symbols of the
cost ([CR#202.1a]).
_Avoid_: run, which names a part of a Pile

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

**Merge**:
To place an Object on top of or under a Permanent, which becomes one merged
Permanent represented by both ([CR#730.2,702.140c]). Only mutate merges
([CR#730.1]).
_Avoid_: placement rider, since a merge is no Pile Placement

**Relation**:
A named link the rules give between two Objects, made by one ability and
read by others: a card haunting a creature, a pair of soulbonded creatures, a
card encoded on a creature ([CR#702.55b,702.95b,702.99b]). A Relation is
declared by name with the kinds of its two sides.
_Avoid_: designation, which marks one thing, not a pair

**Text Change**:
A continuous effect that changes an Object's rules text, applied in layer 3
([CR#612.1,613.1c]): removing the bracketed words (cleave, [CR#702.148a]),
changing "target" to "each" (overload, [CR#702.96a]), adding another card's
rules text after the spell's own (splice, [CR#612.10]), replacing one
word of a named kind, a color word or a land type word, with another of
that kind ([CR#612.2]), or exchanging two
objects' rules text ([CR#612.5,701.12h]). It changes words used in their
rules sense only.
_Avoid_: readings, alternative readings (a card is written once)

**Bracketed** (project term):
Rules text a card prints inside square brackets, kept in place as a marker
so that a Text Change can find it ([CR#702.148a]). Outside a Text Change the
marker means what its contents mean.

## Engine semantic language

**Definition** (project term):
A named, trusted expansion into semantic expressions for vocabulary the rules
define: a keyword action ([CR#701.1]), a keyword ability ([CR#702.1]), a
Counter, a Token or a Designation. Its name provides an attachment point for
spellings; its expansion supplies the modeled meaning, and an action's
expansion reads its arguments in its own Reference Scope. Its named arguments
can capture a subject's identity or a numeric value, or supply syntax whose
references retain their caller's scope. Checking the expansion's admissibility
does not establish its rules fidelity.
_Avoid_: Semantic Macro, macro; treating a deed label alone as its meaning;
Registry Definition, which is what a registry declaration means

**Template** (project term):
A named, transparent expansion into semantic expressions for a phrasing the
rules do not define as a keyword ([CR#701.1,702.1]): a wording a card prints
("can't be blocked by more than one creature"), expanded into the same
expressions its parts would be written as. Its name provides an attachment
point for spellings and nothing more; it adds no rules meaning of its own.
_Avoid_: Semantic Macro, macro; Definition, for a phrasing the rules do not
define

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

**Card Class** (project term):
Whether a card is a Permanent Card or an instant or sorcery card, by its Card
Types ([CR#110.4,110.4a]). A Keyword Ability that represents different
abilities "on a permanent" and "on an instant or sorcery spell"
([CR#702.55a,702.131a,702.131b,702.174b]) guards each half of its one
definition by the Card Class the half applies to.
_Avoid_: Ability Category, which sorts abilities, not cards

**Object Class** (project term):
One of the overlapping ways [CR#109.1] classifies an Object, such as Card,
Token, Spell, Permanent, Emblem, or an Ability on the stack.
_Avoid_: Object Kind, substrate

**Head Noun** (project term):
The Object Class or Card Type [CR#109.1,205.2a] a Reference names its Object
by, as in "this creature", "this spell" or "the emblem that granted it". It
narrows what the Reference is known to be without introducing another Object.
_Avoid_: Marker Word

**Reference** (project term):
A singular semantic expression that denotes one Object or Player in context.
Amounts and collections are not References.

**Reference Scope** (project term):
The mentions accessible to a Reference under its containing grammatical
construction and discourse box ([CR#109.2,701.21a]). Within that accessible
set, a bare pronoun excludes clause-mates and selects the antecedent in the
nearest clause, breaking ties by grammatical role (subject over object over
other) and refusing a remaining tie; a demonstrative noun phrase excludes
clause-mates and what a bare pronoun in the same clause resolved to, then
selects the most recent compatible antecedent; a stamped participle phrase
requires a unique compatible antecedent. A Definition body establishes its own
Reference Scope over its bound parameters and active Actor; a historical-event
predicate binds the entity it describes (`which`) locally.
_Avoid_: author-chosen numeric stack depth

**Binding Identity** (project term):
The identity of one mention's binding in the checker's context, fixed when the
mention introduces it. In discourse-representation terms a binding is a
discourse referent and its Binding Identity is that referent; Reference Scope
is accessibility. A later Reference that names the binding by its identity
resolves to it without a pronoun search: a Definition's referent parameter
is bound at the body's first mention, and each later mention of the parameter
refers back by Binding Identity.
_Avoid_: stack position or depth for the identity — a position shifts as
bindings are pushed and removed; capture for the binding rule

**Binding Possessor** (project term):
The Player an Object binding's referent belongs to, as far as the text fixes it,
recorded relative to the Controller of the text ("you" [CR#109.5]): for a card
in a hand, graveyard or library, that zone's Owner ([CR#400.1,400.3]); for a
Permanent, its Controller ([CR#108.4]). It is "you", another Player than you,
or unknown. A move between one Player's own zones keeps it; a move to or from a
shared zone, or a change of control, leaves it unknown ([CR#110.2]).
_Avoid_: owner or controller alone for the field — which of the two it is
depends on the zone the referent is in

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

**Sharing** (project term):
A Predicate true of an Object that has at least one value in common with
another on one characteristic: a color, a creature type, a name
([CR#201.2a,105.2,205.3m]); its negation is having none in common
([CR#201.2c]). On names it is not equality: an Object with two names shares
a name with an Object that has either.
_Avoid_: same as, equal

**Agreement** (project term):
A constraint that the members of a group share, or do not share, a value
on one characteristic ("two creatures that share a creature type"): it
holds of the group, not of any one member.
_Avoid_: Sharing, for the group form

**Type Evidence** (project term):
Known Card Type facts retained for reference resolution. Multiple facts can
hold for one Object [CR#205.2b]; an alternative retains only shared facts.
Remembered facts are separate from the current Object Class and Zone used to
resolve a reference, and do not simulate changing characteristics.

**Filter** (project term):
The operation of retaining candidates that satisfy a Predicate.
_Avoid_: Predicate as an interchangeable noun

**Actor** (project term):
The Player or Permanent who performs an Action. Outside every Handoff the
Actor is the controller of the spell or ability whose text names the Action
([CR#109.5]); inside one it is the Player or Permanent the innermost Handoff
names. A keyword action a Permanent performs ([CR#701.44a,701.63a]) written
with no Handoff to a Permanent is performed by the source Permanent. An
ability that an Action grants or creates has its own controller as Actor
([CR#109.5]).
_Avoid_: agent for the performer — no model field records one (a choice's
chooser and an added turn's taker are the Actor too), and "agent" means the
external party a Decision Point waits on; "you" for the Actor inside a
Handoff, where "you" still means the controller

**Handoff** (project term):
An Action that makes a named Player or Permanent the Actor of the
Action it contains ("target player discards a card", "target creature
explores" [CR#701.44a]). Handed to a group, the contained Action is
performed in full by each member. The steps the rules give a Permanent's
controller are handed on to that controller inside it. A Handoff is the only
place an Action's performer is written.
_Avoid_: rebinding "you" — "you" keeps referring to the controller inside a
Handoff ([CR#109.5])

**Static Spec** (project term):
Declarative semantic syntax stating what holds while its carrier is in force.
It is the payload a Static Ability states, and the part a one-shot puts in
force for a duration ([CR#611.2]); like an Action that text names, it is
not itself an Effect — the Continuous Effect is what applying it establishes.
_Avoid_: Static Effect

**Mode Change** (project term):
A Static Spec that changes how its own Spell's modes are chosen: all modes
for a cost or under a condition ([CR#702.42a]), a cost for each mode beyond
the first ([CR#702.120a]), the number of modes ([CR#702.172a,702.183a]), the
same mode chosen more than once ([CR#700.2d]), or only modes not chosen
before. A mode may carry a weight, counted by "{P} worth of modes"
([CR#700.2i]).
_Avoid_: mode cost, for the whole change (one row of it is)

**Deontic Exception** (project term):
The one source a "can't" exempts: "can't transform except due to its
daybound ability" ([CR#702.145b]), named by a Keyword Ability, or "can't be
blocked except by artifact creatures and/or creatures that share a color with
it" ([CR#702.13b]), named by a description of Objects.
_Avoid_: negated predicate, for the exception

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
none stated it lasts until the game ends ([CR#611.2a]). It is either bounded by
two optional Events — "until end of turn" has only an end, "on a later turn"
only a start ([CR#702.170d]) — or lasts for as long as a condition holds
([CR#611.2b]). A history window is a Duration that ends now ("this turn", "this
game"), or at the end of the Turn it names when that Turn is over ("last turn").
"The cards exiled this way" and the event that triggered an ability name an
occurrence, not a span, and are not Durations.
_Avoid_: span, window

**Timing** (project term):
When a Player may act, or when a trigger or replacement applies: during a Phase
or Step of a Turn named by a Turn Reference, before or after an Event, while an
Event is underway, or only if a condition holds, combined with and, or and not.
"Only as a sorcery" is a Timing ([CR#307.5]); with no Timing written, a Player
may act any time they have priority ([CR#117.1a]).
_Avoid_: window

**Turn Reference** (project term):
Which Turn a Timing or Duration names: the current Turn, the previous one, a
Player's next Turn, each of a Player's Turns, a later Turn ([CR#702.170d]), or
the Turn an earlier phrase named.
_Avoid_: this turn, as a reference — "this" names an Object

**Usage Limit** (project term):
How many times something may be done per Turn, per one of its Player's Turns,
per Phase or Step, per game, or for as long as the Object exists ("activate
only once", [CR#602.5b,702.177a]). An Event's ordinal ("the second
spell each turn") is not a Usage Limit.
_Avoid_: rank, ordinal

**Decision Point** (project term):
An engine boundary at which progress waits for an external agent to supply a
Decision.

**Decision** (project term):
The response submitted at a Decision Point. A Decision may encode a rules
Choice or a proposed Action.

**Conferral** (project term):
The relationship by which a rule, Card Type, Subtype, or Counter supplies an
Ability or other rules property. A conferred Ability remains an ordinary
Ability unless the CR says otherwise. Four things can be conferred, and the CR
keeps them apart: an intrinsic Ability ([CR#305.6]); a continuous property with
no Ability behind it, which neither grants an Ability nor sets a characteristic
([CR#113.12]); a State-Based Action ([CR#704.1]); and a turn-based action
([CR#703.1]).

**Registry Definition** (project term):
What one registry declaration means — the rules content the declared name
stands for. A Counter's kind, holder and Conferrals: an Ability or property
Conferral is what ONE Counter of the kind confers, applied once per Counter
held (a +1/+1 counter confers +1/+1 [CR#122.1a]), and a State-Based Action
Conferral applies once per holder per check and may read counts, its own
kind's included (the -1/-1 counter's annihilation rule [CR#704.5q]); a Subtype and its
Conferrals; a Designation's holder and columns; a Card Type, whether it is a
Permanent Type, and its Conferrals (a creature is a Combatant [CR#508.1a]). A registry declares a
namespace's members, so a Registry Definition names what it defines, where a
Rules Table row names nothing and is scoped by a Predicate instead. The
declared name means the defined term everywhere, the Registry Definition's own
body included — a Subtype in a card's subtype list, a Counter's kind — and the
Registry Definition is a separate record the registry holds under that name; a
Designation's name is the Designation.
_Avoid_: Rules Table for a Registry Definition.

**Designation** (project term):
A named marker an Object, a Player, or the game can have, which rules and
Effects identify without it being an Ability or a copiable value. The CR names
each Designation on its own — goaded ([CR#701.15b]), the city's blessing
([CR#702.131c]) — rather than defining the class. A Designation is identified
by its name, the name the registry declares it under (`monarch`), not by the
phrase a card prints ("the monarch"); one declaration may declare several,
as day and night are two Designations [CR#731.1].

**The Game** (as a holder):
The game itself, as the holder of the Designations it can have: "it becomes
night" is the game gaining night, and "it's day" is the game having day
([CR#731.1]). It is neither an Object nor a Player, and text names it only as
that holder.
_Avoid_: Entity, which the game is not

**Primitive Keyword Ability** (project term):
A Keyword Ability whose semantics are represented directly rather than
decomposed into component Abilities.
_Avoid_: Intrinsic Keyword Ability

**Composite Keyword Ability** (project term):
A named Keyword Ability represented as an ordered collection of component
Abilities. Its components remain structural parts of the one named Keyword
Ability rather than independent entries in the visible ability list.

**Qualified Ability** (project term):
An ability printed with a keyword, together with the restriction the keyword
adds to it: when it may be activated, how often (once each turn, or once for
as long as the Object stays in its zone, [CR#602.5b,702.177a]), a condition
read as "as long as", "triggers only if" or "activate only if" by the
ability's category ([CR#702.169b..702.169d]), and the zone it functions from
([CR#113.6b]). Ability words do not qualify an ability ([CR#207.2c]).

**Execution Frame** (project term):
The evaluation context that supplies bindings needed to interpret References
and Actions.
_Avoid_: Frame when the qualification matters
