# Rules taxonomy: the value kinds of the card language

2026-06-06

What kinds of value does a complete card-encoding language need? This is the
taxonomy of typed positions — the things `MacroKind` will eventually
enumerate, the enums `deckmaste_core` will grow, and the vocabularies plugins
will extend. Sources:

- the Comprehensive Rules snapshot at `data/rules/cr.json` (rule numbers
  cited throughout as e.g. CR 603.4);
- an empirical census of the card-todo corpus: 30,881 todo files under
  `plugins/wizards/cards/`, 58,689 ability-text lines (counts cited as e.g.
  "5,256 lines");
- the current model: `Ability { Static, Activated, Triggered, Spell, Keyword }`,
  `Target`, `Selector::{Target(usize), GameObject(usize)}`, `Effect`,
  and `MacroKind { Ability, CardFace, Subtype, Target }`.

The corpus splits (mutually exclusive classification): bare keyword lines
23.5%, triggered abilities 22.4%, activated abilities 16.2%, one-shot
imperatives 12.4%, static prose ~13%, modal bullets 2.8%, loyalty lines
1.5%, saga chapters 0.9%, residual 7.7%. Nearly every line decomposes into
the kinds below.

---

## 1. The unification: Filter is the center of the taxonomy

The prompt list — Target, "gameobject selector", evasion-ability selector,
protection selector — is indeed mostly one thing. The CR uses a single
predicate vocabulary in all of these positions:

- **targeting** — "target creature you control" (CR 115.1);
- **protection from [quality]** — CR 702.16a: a color, a card type, a
  subtype, a supertype, a card name, a player, "everything", or "any
  characteristic value or information";
- **evasion** — "can't be blocked except by creatures with flying and/or
  reach" (CR 702.9b): a predicate over would-be blockers;
- **hexproof from [quality]**, **affinity for [quality]**, **landwalk
  [type-expression]**, **equip [quality] creature**, **champion an
  [object]**, **enchant [object or player]**;
- **trigger-event participants** — "whenever a creature *you control*
  dies";
- **non-target selection** — "each opponent", "all creatures with flying",
  "a creature you control with the greatest power".

So the foundational kind is a **Filter**: a predicate over game objects and
players. Everything in the prompt's list is Filter plus a *binding strategy*
(§2) or Filter plugged into an engine hook (block-legality for evasion,
targeting legality for protection/hexproof).

### Filter's predicate vocabulary

Atoms, each citable to CR or corpus:

| Atom | Examples / cites |
|---|---|
| card type / supertype / subtype | "artifact creature", "legendary", "Aura" (CR 205); typal lines everywhere |
| color, colorlessness | "red creature"; color pairs (CR 105.5) |
| name | "with the same name" (100 lines), "named X", "with the chosen name" (44) |
| characteristic comparisons | "power 4 or greater" (231), "mana value 3 or less" (383), "base power and toughness" (87), toughness, loyalty, defense |
| has-ability / lacks-ability | "with flying" (578), "without flying" (~86) |
| has-counter | "with a +1/+1 counter on it", "with a finality counter" (34) |
| status (CR 110.5) | tapped/untapped, face up/down, flipped, phased out; "untapped creature" |
| designation (§8) | "goaded creature", "is your Ring-bearer" (CR 701.54e), "in the same sector" (CR 702.158e) |
| control / ownership | "you control" (7,908 lines, 13.5%), "an opponent controls" (1,041), "its owner" (1,228) |
| zone | "card in a graveyard" (67 target lines), "from your hand", "on the battlefield" |
| combat state | "attacking or blocking creature" (198 target lines), "unblocked", "attacks alone" (CR 506.5) |
| relations | "enchanted creature" (1,343), "equipped creature" (776), attached-to, paired-with (CR 702.95b), "blocked by", exiled-with (CR 607.2a) |
| identity / exclusion | "another" (397 "another target"), "other than ~", "nontoken", self-reference |
| derived designations (§8) | "modified" (CR 700.9), "historic" (CR 700.6), poisoned (CR 122.1f), suspended (CR 702.62b) |
| spell properties | "spell that targets this creature", "noncreature spell" (96 trigger lines), "with X in its mana cost", kicked (CR 702.33) |
| player properties | life-total comparisons ("player with the most life", dethrone CR 702.105), "poisoned", has-designation (monarch), speed (CR 702.179f) |

Combinators: and, or ("artifact or enchantment", 128 target lines), not
("non-Human"), quantified comparisons against other selections ("with power
greater than ~'s power", skulk CR 702.118), and enumeration over a
characteristic's value space ("protection from each color", CR 702.16h —
the Quality type must support "each [characteristic]").

Filter is the most-reused kind: Target, Event, ContinuousEffect's affected
set, Cost ("sacrifice *a creature*"), CardFilter for searches (CR 701.23),
protection/hexproof/evasion parameters, and Condition counting all embed it.
A `Filter` macro namespace gives names like `AnyTarget` (already exists),
and protection/evasion macros become Filter-parameterized Ability macros.

Open design question: players-as-objects. "Any target" (829 lines; CR
115.4) = creature ∪ player ∪ planeswalker ∪ battle, so the cleanest model
makes Filter range over both objects and players, with player-only atoms
(life total, designations) and object-only atoms (power) simply never both
matching.

### Binding strategies (how a Filter becomes a selection)

What the current `Target` enum gestures at generalizes to: Filter ×
quantifier:

| Quantifier | Corpus | Notes |
|---|---|---|
| target(n) | "target" in 12,843 lines (21.9%) | the targeting *mechanic*: chosen at announce, rechecked at resolution (CR 115, 608.2b) |
| up to n targets | "up to ... target" 921 | zero legal (CR 115.6) |
| any number of targets | 170 | count announced first (CR 601.2c) |
| each | "each" selectors 5,505 (9.4%) | no targeting; evaluated at resolution |
| all | 1,813 | "all creatures" 395 |
| one chosen (no target) | "chosen/of your choice" 843 | e.g. "a creature of their choice" — chooser may differ from controller |
| random | 463 | discard at random etc. |
| the unique one | — | "the exiled card", "equipped creature": relations that resolve to ≤1 object |
| divided among | 82 | division fixed at announce for targets (CR 601.2d), at resolution otherwise (608.2d) |
| superlative | — | "with the greatest power among", bolster's "least toughness" (CR 701.39) |

Targets additionally carry per-instance identity (a spell's first vs second
target, CR 115.3, 601.2c) — which is what §2's references index into.

---

## 2. Reference: "this ability's nth target" and all its relatives

The existing `Selector::Target(usize)` is the seed of a much bigger kind.
Card text is full of *bound variables* — values fixed earlier (at cast, at
trigger time, by a linked ability, by the rules of the zone the object sits
in) and referenced later. A `Reference` kind needs at least:

- **Self** — "this creature/spell/permanent": 16,468 lines (28.1%) contain a
  self-reference; plus cardname-leading lines ("~ deals N damage…", 682
  residual lines) which are the same thing in older templating.
- **Target(i)** — per instance of the word "target" (CR 115.3). Already
  modeled.
- **Event participants** — "that creature", "that player", "that spell",
  "it": the objects bound by a trigger's event pattern. Corpus: "that
  card/creature/spell/token" 2,640 (4.5%), "that player" 1,265 (2.2%).
  An `Event` (§3) must declare what it binds; the effect refers to those
  bindings positionally or by role name.
- **Linked-ability information** (CR 607 — the full list of linkage
  patterns):
  - cards exiled with this (607.2a/b/q; "the exiled card" — 184 lines);
  - objects put onto the battlefield / created by this (607.2c);
  - the chosen value — color, type, name, number (607.2d; "the chosen
    name" 44 lines, "choose a color" 102, "choose a creature type" 104);
  - noted information (607.2e);
  - anchor-word choice (607.2f, 614.12c);
  - cost paid as it entered / as it was cast, and whether an optional cost
    was paid — "if kicked" (607.2g/i/j; CR 702.33);
  - pre-game choices (607.2n/p).
- **Attachment relations** — enchanted/equipped/fortified object, "the
  permanent this Aura is attached to", attachments-of.
- **Derived objects of a reference** — controller-of (612 "its controller"
  lines), owner-of (1,228), source-of-damage (CR 609.7).
- **Magnitude anaphora** — "that much"/"that many" (587 lines), "damage
  dealt this way" ("this way": 1,014 lines), "equal to the exiled card's
  power". These are Quantity-valued references (§6).
- **The choice machinery's outputs** — mode chosen, X (2,054 lines), the
  result of a vote, "the sacrificed creature".

Design implication: every kind below that introduces objects or values
(Event, Target lists, choices, linked actions) is a *binder*, and Effect
bodies are written against the bound names/indices. This is exactly the
"variant that can stand in for something" from the prompt, and it stays
declarative — references are data (indices/roles), not computation, in
keeping with the macro language's no-control-flow rule.

---

## 3. Event: trigger selectors

22.4% of all ability lines are triggers (13,130: "When" 5,399 / "Whenever"
5,832 / "At" 1,899). An `Event` value = event kind + Filters over the
participants + the bindings it exports. The anatomy from CR 603:

**Structural features** (each is a slot in the Event/Trigger model):
- trigger word when/whenever/at (CR 603.1);
- **intervening if** (CR 603.4) — checked at trigger *and* at resolution;
  1,113 lines (8.5% of triggers);
- **usage limits** — "only once each turn" (CR 603.2h; 291 lines);
- **state triggers** (CR 603.8) vs event triggers;
- **delayed triggers** (CR 603.7) — created by effects; "at the beginning of
  the next end step…";
- **reflexive triggers** (CR 603.12) — "When you do, …" mid-resolution;
- **look-back-in-time** semantics for leave/die/lose-control triggers (CR
  603.10a–g) — engine concern, but the Event kind must know which events
  use prior state;
- "becomes" semantics — only on transition, not on entering-already-true
  (CR 603.2e);
- multi-condition events ("Whenever you attack" = one or more attackers;
  "enters or attacks" — 61 lines).

**Event-kind vocabulary** (with trigger-share from the corpus census):

| Family | Events (corpus % of triggers) |
|---|---|
| zone changes | enters (35.7%), dies (8.2%), leaves the battlefield (2.2%), put into a graveyard from … (1.2%), exiled, shuffled in; "dies" = battlefield→graveyard (CR 700.4) |
| time | beginning of upkeep (7.9%), end step (4.9%), beginning of combat (1.8%), draw step, precombat main, end of combat (CR 5xx turn-based structure) |
| combat | attacks (10.5%), blocks (1.5%), becomes blocked, deals combat damage to a player (4.4%), deals damage (10.7%), attacks alone, isn't blocked (CR 508.3, 509.3) |
| player actions | you cast a spell (6.1%), draws (9.5%), discards (4.3%), sacrifices (6.9%), gains life (1.2%), loses life, plays a land (landfall — 166 ability-word lines), taps a land for mana, searches, mills, scries |
| object state | becomes tapped, becomes the target (0.7%), is turned face up, transforms, becomes paired/crewed (CR 702.122d), counters put on / "Nth counter put on" (CR 122.7), unlocked / fully unlocked (CR 709.5h–i) |
| game state / designations | becomes monstrous/renowned, day becomes night (CR 731.1a), becomes the monarch / takes the initiative (CR 725.2, 726.2), you get the city's blessing, completes a dungeon |
| meta | "for the first time each turn", "if it's the Nth time this ability has resolved this turn" (CR 603.7h) |

Note the **action/event mirror**: nearly every keyword action and effect
verb (§5) has a corresponding event ("sacrifices", "is dealt damage",
"is countered"). Event patterns are essentially pattern-matches over the
engine's action log plus state transitions plus clock ticks. Designing the
Action vocabulary and the Event vocabulary together — same nouns, same
filters — is the single biggest coherence win available.

---

## 4. Cost

Costs appear in three positions: activated-ability costs (16.2% of lines;
CR 602.1a), spell costs and their modifiers (additional/alternative, CR
118.8/118.9), and resolution-time payments ("unless you pay…", CR 118.12).
A `Cost` is a *vector of components* (CR 601.2f computes a "total cost"):

| Component | Corpus (of 9,522 activated lines) | Cites |
|---|---|---|
| mana (incl. X, hybrid, Phyrexian, snow) | mana-only costs 4,642 | CR 118.3a, 107.3, 118.13 |
| {T} / {Q} | 5,122 / 18 | CR 602.5a (summoning sickness) |
| sacrifice [filter] | 1,743 | |
| discard [spec] | 364 | |
| pay N life | 150 | CR 118.3b |
| exile [from zone] | 243 | e.g. "Exile this card from your graveyard:" |
| remove counters | 341 | also loyalty: 858 loyalty lines, `[+N]`/`[−N]` (CR 606) |
| put counters | — | level up; "an Aura attached…" odd costs |
| energy {E} | 58 | CR 107.14 |
| reveal / show | — | forecast (CR 702.57) |
| tap-others-with-total-power ≥ N | — | crew/saddle (CR 702.122/171) |
| unpayable | — | no mana cost (CR 118.6) |

Notice: **cost components are the same verbs as one-shot effects** (§5) —
sacrifice, discard, exile, pay life, tap. A shared Action vocabulary with a
"cost-position" subset (only actions whose performer is the payer and whose
magnitude is fixed) avoids defining everything twice. The Treasure token
already wants this: `cost: [Tap, SacrificeThis]`.

Around Cost sit the **cost modifiers** (CastModifier territory, §11):
additional costs (CR 118.8; "as an additional cost" 308 lines), alternative
costs (CR 118.9, only-one rule 118.9a), reductions ("cost {N} less" 561
lines, "more" 128; CR 118.7), and per-mode costs (spree/tiered/escalate, CR
702.172/183/120). Keyword parameters are overwhelmingly Costs — equip,
cycling, flashback, kicker, ward, echo, etc. (~60 keywords take exactly one
Cost; several take "N—[cost]" pairs: suspend, awaken, reinforce, impending).

---

## 5. Effect and Action: the verb inventory

Two layers, matching the user's Effect-vs-Action split:

- **Action**: one verb applied to a selection — the unit the engine
  executes and the event log records. CR 701's keyword actions are the
  canonical inventory, plus unkeyworded primitives (deal damage, gain/lose
  life, draw, add mana, win/lose).
- **Effect** (one-shot): the *program* a resolving spell/ability runs —
  a sequence of actions with binding, optionality, conditionals, and
  iteration as **data** (the macro layer stays control-flow-free; the AST
  carries the structure).

### Primitive verbs (CR 701 + unkeyworded), by family

| Family | Verbs |
|---|---|
| zone change | destroy (701.8), exile (701.13), sacrifice (701.21), discard (701.9), mill (701.17), counter (701.6), return-to-hand/battlefield ("Return" leads 450 lines), put-onto-battlefield, put-into-graveyard/library-position, create token (701.7; 3,232 "create" lines), attach/unattach (701.3), meld (701.42) |
| library | search (701.23), shuffle (701.24), reveal (701.20), look-at, scry (701.22; 428 lines), surveil (701.25; 216), fateseal (701.29), clash (701.30) |
| counters | put/remove counters (2,808 / 576 lines), proliferate (701.34; 95), move counters |
| state | tap/untap (701.26), transform/convert (701.27/28), turn face up/down, regenerate (701.19), phase out, exert (701.43), lock/unlock (709.5f–g) |
| designation-granting | goad (701.15; 84 lines), detain (701.35), suspect (701.60), harness (701.64), monstrosity (701.37), the Ring tempts you (701.54), become saddled/solved/prepared — see §8 |
| player resources | draw (520 leading lines), gain/lose life, pay, add mana (605), get energy/experience/rad counters, set/increase speed (702.179) |
| combat | fight (701.14; 140 lines), remove from combat, "must be blocked" requirements |
| damage | deal damage (the dominant spell effect; "~ deals" hides 682 cardname-led lines), prevent damage (§7) |
| game structure | extra turn, extra phase/step, end the turn (CR 724), skip (614.1b), restart (727), subgame (729), win/lose the game |
| choice | vote (701.38), villainous choice (701.55), choose-a-[value] (binders, §2) |
| variant | venture (701.49; 39 lines), planeswalk, set in motion, open an attraction, roll to visit (701.52), roll a die (76 lines), flip a coin (72 lines) |

**Compound keyword actions are macro fodder** — the CR defines them in
terms of primitives: investigate = create a Clue (701.16); populate;
incubate; amass; explore; connive; discover; learn; forage; behold;
manifest (+ cloak = manifest + ward); endure; earthbend; monstrosity. These
become `Effect`-kind macros exactly like `AnyTarget` is a `Target` macro —
e.g. the Aang todo's "airbend another target creature" is an Effect macro
invocation `Airbend(...)` (CR 701.65) awaiting definition.

### Effect-AST structure (the connective tissue)

From the corpus, the structural combinators an Effect body needs:

| Structure | Corpus | Notes |
|---|---|---|
| sequencing ("…, then …") | pervasive | order matters (CR 608.2c) |
| optionality "you may" | 4,453 (7.6%) | with "If you do / don't" continuations (1,029 / 159) |
| conditional (if / otherwise) | "If" mid-effect 588+ lines; "otherwise" 143 | distinct from intervening-if |
| unless | 749 | = cost-or-consequence (CR 118.12a) |
| for-each iteration | "for each" 1,609 (2.7%) | usually Quantity (§6), sometimes per-object actions (CR 607.3) |
| division | 82 | CR 601.2d/608.2d |
| reflexive trigger "When you do…" | — | CR 603.12 |
| delayed trigger creation | "at the beginning of the next end step" etc. | CR 603.7, 610.2 |
| modal block | §10 | |
| restriction grants | "can't block this turn", "must attack" | continuous, §6 below |

---

## 6. ContinuousEffect, Duration, Quantity, Condition

### ContinuousEffect

What statics and "until end of turn" effects produce (CR 611). The value is
a tuple — **(affected set: Filter or locked selection, modification,
duration)** — where the modification determines its layer (CR 613):

| Layer | Modification | Corpus |
|---|---|---|
| 1 | copy effects (707), face-down characteristics | |
| 2 | control change | "gain control" |
| 3 | text change (612) | rare |
| 4 | type/subtype/supertype change | "becomes a 0/0 land creature…" (earthbend) |
| 5 | color change | |
| 6 | add/remove abilities; can't-have | "have/has [keyword]" 3,430 lines (5.8%) |
| 7a–d | P/T: CDA / set / modify / switch | "get +N/+N" 4,949 lines (8.4%); CDA "~'s power is equal to…" 259 residual lines |
| player | player-affecting (613.10) | "You can't lose the game", max hand size |
| rules | game-rule modifying (613.11) | cost mods (561+128 lines), "can't" restrictions (1,861 lines), "don't/doesn't" (858), play-permissions ("you may cast … from your graveyard"), requirements ("attacks each combat if able", 251 "if able" lines), "double" (432), "enters tapped" (737) |

The lock-in distinction matters to the data model: resolution-created
effects fix their affected set (CR 611.2c); static-ability effects float
(611.3). That's an engine semantic, but the data must distinguish "target
creature gets" from "creatures you control get".

CDAs (CR 604.3) are a flagged subspecies — they work in all zones and feed
layer 7a / color / subtype.

### Duration

A small closed kind (CR 611.2, 614, 615): until end of turn (5,256 lines —
9.0%!), this turn (2,425), while-condition "as long as" (1,094) and "for as
long as" (203, with never-started semantics CR 611.2b), until-your-next-turn
(108; also goad/detain's default), until-end-of-combat (CR 514.2 cleanup vs
511), until-[event] (242 "until ~ leaves" — implemented as paired one-shots,
CR 610.3), next-N-times counted durations (prevention shields, CR 615.7),
once-per-turn usage windows, and unstated = rest of game (CR 611.2a).

### Quantity

Numbers are rarely literal: literal N (number-word distribution: one 2,956,
two 2,697, three 1,340, then steep falloff; seven spikes to 286 — hand
size), X with where-X-is definitions (2,054 / 1,084 lines; CR 107.3),
count(Filter) — "for each" (1,609), equal-to(property of Reference) (1,576),
that-much/that-many anaphora (587), up-to caps (1,344), twice/half (rounding
direction!), sum-over ("total power of creatures you control"), and
devotion-style derived counts (CR 700.5). Quantity must compose with
Reference (§2) and Filter (§1).

### Condition

Boolean game-state predicates, used by intervening-ifs, "as long as"
durations, modal gates, activation restrictions, and **ability words** —
which are *named Condition (or Event) macros*: 2,116 corpus lines lead with
an ability word (landfall 166, threshold 98, delirium 73, domain 55, raid
46, heroic 44…). The repo already stubs these in
`plugins/wizards/ability_words/`. Atom families:

- count comparisons over Filters ("if you control three or more artifacts",
  "if an opponent has more cards in hand than you");
- history-of-this-turn predicates ("if a creature died this turn" — morbid,
  "if you've cast another spell" — surge CR 702.117, "if you gained life",
  descended CR 700.11, "was kicked" CR 702.33, "spell was warped this turn"
  CR 702.185c);
- state predicates (it's day/night, you're the monarch, "if this creature
  is monstrous", threshold/delirium/hellbent/metalcraft/fateful-hour);
- comparison predicates over Quantities;
- whose-turn / which-phase predicates ("during your turn", "only during
  combat").

The cast-history family implies the engine keeps per-turn and per-object
memory (what was paid, what happened this turn) — the Condition kind names
those memories declaratively.

---

## 7. Replacement and prevention shapes

A closed set of template shapes the data model should mirror 1:1 (CR 614,
615), since the CR itself enumerates them:

| Shape | Template | Corpus / cites |
|---|---|---|
| replace | "If [event] would [happen], [modified event] instead" | 537 would-instead lines; CR 614.1a |
| skip | "Skip [step/phase/event]" | 58; CR 614.1b |
| enters-with | "~ enters with N [kind] counters" | 561 "enters with"; CR 614.1c |
| as-enters | "As ~ enters, [choice/state]" | 370 "As" lines; CR 614.1c, choices CR 614.12a |
| enters-tapped | continuous "enter tapped" | 737; CR 614.1d |
| ETB under modification | "you may have ~ enter [as a copy / with…]" | 61; CR 614.12 |
| regeneration | destruction-replacement | CR 614.8 |
| redirection | damage moved between recipients | CR 614.9 |
| draw replacement | "If you would draw…" | 45 "would draw"; CR 614.11 |
| token/counter replacement | "If one or more tokens/counters would…" | CR 614.16 |
| self-replacement | part of the same spell's effect | CR 614.15 |
| prevention: next-N | "Prevent the next N damage … this turn" | CR 615.7 |
| prevention: next-instance | "the next time [source] would deal damage" | 81 "the next time"; CR 615.8 |
| prevention: static per-event | "Prevent all combat damage" | 533 "prevent" lines; CR 615.10 |
| can't | not a replacement but kindred ("can't" 1,861 lines) | CR 614.17 |

Specific "would" subjects in the corpus: would be dealt 442, would deal 199,
would be put into 99, would die 89, would draw 45, would gain/lose life 22.
Keywords that are sugar over these shapes: persist/undying (return-with),
umbra armor, madness's discard-to-exile, daybound/nightbound pieces,
bloodthirst/devour/fabricate/riot/sunburst/tribute (all enters-with /
as-enters family).

---

## 8. Designation — the scattered kind, cataloged

Designations are markers the rules attach to objects, players, or the game
itself. The CR never centralizes them; this is the full catalog
from a sweep of every rule mentioning "designation" plus the
designation-like statuses defined without the word. Uniform properties the
CR repeats each time: a designation is **not an ability, not a
characteristic, not copiable** (e.g. CR 701.37b, 702.112b), and most have
**no inherent rules meaning** — they're markers other abilities test.

### Object-scoped

Designations attach to *objects*. A zone change creates a new object (CR
400.7), so the dominant "until it leaves the battlefield" persistence is
not a rule about battlefields — it's object identity, and it covers
exile-zone objects (foretold, plotted) the same way: granted while the card
sits in exile, gone when casting it creates a new object on the stack.
The Ends column lists only expirations *earlier* than object identity.

| Designation | Cite | Payload | Shape | Granted by | Ends |
|---|---|---|---|---|---|
| monstrous | 701.37b | marker only | flag | monstrosity N | leaves the battlefield |
| renowned | 702.112b | marker only | flag | renown N | leaves |
| harnessed | 701.64b | marker only | flag | harness | leaves |
| solved | 719.3b | gates "Solved —" abilities (719.3c) | flag | Case-solving checks | leaves |
| suspected | 701.60b | has menace, can't block (701.60a) | flag | suspect | leaves, or "no longer suspected" |
| goaded | 701.15b | attack requirements | flag **per goading player** (701.15d: can be goaded by multiple players) | goad | the goading effect's duration (default "until your next turn") |
| saddled | 702.171b | marker only | flag | saddle N | **end of turn** or leaves (unique: self-expires) |
| prepared | 722.3a–c | copy-in-exile becomes castable | flag | "becomes prepared" / enters prepared | "unprepared" effects; lost when the copy is cast |
| paired | 702.95b | soulbond pairing | **relation** to another creature | soulbond | either leaves / stops being eligible |
| Ring-bearer | 701.54b | tested by "is your Ring-bearer" (701.54e) | flag, **unique per player** | the Ring tempts you (chooses/moves it) | replaced by next temptation choice |
| level N | 716.2b | gates Class abilities | **number** | "becomes level N" (Class) | persists even if it stops being a Class |
| sector (alpha/beta/gamma) | 702.158b | grouping ("in the same sector" 702.158e) | **enum(3)** | space sculptor SBA choice (704.5u) | while any space-sculptor source exists |
| left/right half unlocked | 709.5c–i | half's name/cost/text active (709.5) | **set of 2 flags** | cast that half; unlock cost (special action 116.2m); unlock effects | lock effects (709.5g) |
| foretold | 702.143d | may be cast for its foretell cost (which the granting effect may supply) | flag (+ optional attached Cost) | foretell special action (116.2h); "becomes foretold" effects | — (object identity: cast / leaves exile) |
| plotted | 702.170c | castable later without paying its mana cost | flag | plot special action (116.2k); "becomes plotted" effects | — (object identity) |

Suspended (702.62b) belongs to the vocabulary but not the table: it's a
**derived designation** — defined as a predicate, never granted or stored.
See the derived catalog below.

**Commander** (903.3) is the one true exception to object scoping: the CR
explicitly makes it "an attribute of the card itself", not the object,
retained across zone changes — commander tax and color identity depend on
it surviving recasts. Model it as a card attribute outside the designation
system, not as a designation with unusual persistence.

### Player-scoped

| Designation | Cite | Shape / payload |
|---|---|---|
| the monarch | 725.1 | unique-per-game; inherent triggers: end-step draw, transfers on combat damage (725.2) |
| the initiative | 726.1 | unique-per-game; inherent venture triggers, transfers on combat damage (726.2); re-taking by holder re-triggers but creates no second designation (726.5) |
| city's blessing | 702.131c | non-unique, permanent once gained (ascend) |
| speed | 702.179 | **number 1–4**; "start your engines!" SBA sets 1; inherent once-per-turn increase trigger (702.179d); "max speed" = 4 (702.179e); no-speed reads as 0 (702.179f) |
| attacking / defending player | 506.2 | combat-scoped roles; multiple defending players in multiplayer (506.2a, 508.5a) |
| active / starting player | 102.1, 103.1 | turn-structure roles |
| planar controller | 311.5, 901.6 | Planechase variant |
| archenemy; emperor/lieutenant | 904, 809 | variant roles |

Player-attached counters (poison 122.1f, energy 107.14, experience, rad
728, ticket 107.17) are **counters, not designations** — they go through
the CounterKind vocabulary (§9) — but conditions like "poisoned" derive
from them.

### Game-scoped

| Designation | Cite | Shape |
|---|---|---|
| day / night | 731.1 | one-of-two once set, never unset; "day becomes night" = lose first, gain second (731.1a); checked in untap step (502.2, 703.4b) |

### Derived designations (first-class, not edge cases)

Some named statuses the rules test are never granted — they're *defined* as
predicates over other state. Rather than scattering these between Filter
atoms and Conditions, the designation vocabulary supports a second
definition kind: a **derived designation** binds a name to a predicate,
evaluated on demand. Effects can test it but never grant or remove it — a
granting effect naming a derived designation is a load-time error.

| Name | Scope | Cite | Derivation |
|---|---|---|---|
| suspended | object | 702.62b | in exile ∧ has suspend ∧ has a time counter |
| modified | object | 700.9 | has a counter ∨ equipped ∨ enchanted by controller's Aura |
| poisoned | player | 122.1f | poison counters ≥ 1 |
| max speed | player | 702.179e | speed = 4 |
| fully unlocked | object | 709.5i | has both unlocked designations |
| unpaired | object | 702.95b | ¬paired |
| descended | player | 700.11 | a permanent card was put into your graveyard this turn (turn memory) |
| enchanted / equipped / fortified | object | 303.4, 301.5 | has an attachment of that kind |
| blocked / unblocked | object | 509.1h | combat-state relation |
| is your Ring-bearer | object×player | 701.54e | has the stored Ring-bearer flag ∧ on the battlefield ∧ under your control |

Stored and derived layer naturally: speed is stored (a number), max speed
derived from it; paired is stored (a relation), unpaired derived; even
stored flags get tested through derived wrappers (701.54e adds control and
battlefield checks on top of the Ring-bearer flag). At the fully stateless
end, historic (700.6 — artifact ∨ legendary ∨ Saga) is the same declaration
form derived purely from characteristics.

Mechanically, an object-scoped derived designation is a named Filter and a
player- or game-scoped one is a named Condition — the win is one namespace
spanning all three scopes, so "modified creature" and "if you're poisoned"
resolve uniformly with no caller knowing whether the status is stored or
computed. The mechanism generalizes well beyond the CR's designation-worded
statuses: ability-word conditions (threshold, delirium, hellbent, …) are
the same thing wearing a different name, and plugin mechanics can mint
derived predicates as freely as stored ones.

### Designation-adjacent (still not designations)

- **Cast/creation memory** — kicked (702.33), "warped this turn" (702.185c),
  bargained, "cost paid as it entered/was cast" (607.2g/j), mana spent
  (609.4b), crewed-this-turn (702.122d): per-object/per-turn memory
  surfaced through References (§2) and Conditions (§6), not markers.
  (Derived designations may *consult* such memory — descended does — but
  the memory itself is keyed to events and linked abilities, not names in
  the designation namespace.)

### Design requirements (per the prompt: definable in ability macros)

The corpus keeps minting these (suspected 2024, harnessed/saddled/solved
2024–25, prepared 2025+…), so Designation must be an **open `Ident`
vocabulary like subtypes and counter kinds — declarable by plugins**, with
the natural mechanism being the one subtypes already use: the macro that
introduces the mechanic declares its designation (the way `Forest.ron`'s
`LandType("Forest")` declaration joins the Subtype namespace, a
`Monstrosity` effect-macro's plugin declares designation `Monstrous`).
A declaration needs to carry:

- **definition**: stored (granted/removed by effects, carries persistence)
  | derived (bound to a Filter/Condition predicate; granting one is a
  load-time error) — see the derived catalog above;
- **scope**: Object | Player | Game — object covers permanents and
  exile-zone cards alike; commander, a card attribute (903.3), sits outside
  the system;
- **shape**: Flag | Enum(values) | Number | Relation(object) — flag is the
  default; sector/level/paired show the others are real;
- **uniqueness**: none | per-player (Ring-bearer) | per-game (monarch,
  initiative) | one-of-set (day/night);
- **default persistence**: object lifetime (the dominant pattern — free via
  CR 400.7) | until-end-of-turn (saddled) | effect-supplied (goaded) |
  permanent (city's blessing) | rule-scoped (sector);
- **payload**: marker-only (most) vs carried abilities/requirements
  (suspected = menace + can't block; goaded = attack requirements; the
  designation declaration should be able to reference an Ability/
  Requirement so the engine applies it while the designation holds);
- invariants the engine enforces uniformly: never copiable, never an
  ability; stored ones are lost with object identity on zone change (CR
  400.7), derived ones simply re-evaluate.

---

## 9. The vocabulary leaves

Smaller kinds that everything above references:

**Zone** (CR 400.1): library, hand, battlefield, graveyard, stack, exile,
command (+ ante; + "outside the game", which is not a zone, 400.11). Each
zone reference may carry: whose (owner-relative), position (top/bottom/Nth
of library — "top of your library" 1,187 lines), facing (face-down exile
piles, 406.4), and visibility. Zone-change verbs map pairs of these.
Corpus mentions: hand 3,813, graveyard 3,716, exile 3,043, library 2,809,
battlefield 2,802, command zone 47, stack 5.

**CounterKind** (CR 122): open `Ident` vocabulary. On permanents: +1/+1
(2,733 lines) and -1/-1 (199, annihilation rule 122.3), loyalty (122.1e),
defense (122.1g), shield (122.1c), stun (122.1d), finality (122.1h), lore
(107.15a), and **keyword counters** (122.1b — flying, first strike, …,
trample, vigilance: a closed CR list worth mirroring); on players: poison,
energy, rad, experience, ticket. Corpus top kinds beyond P/T: charge 251,
time 108, oil 104, stun 89, poison 68, quest 65, loyalty 57, storage 57.
Like designations: plugin-declarable, with optional payload (keyword
counters grant the keyword; stun replaces untap).

**ManaSpec**: produced mana for AddMana and cost-side symbols — specific
symbols, "one mana of any color", "of any one color" (different!), "any
type" (incl. colorless), conversion riders, and **restricted mana** ("spend
this mana only on…" — a rider the mana object carries; CR 106.6-ish).
Treasure's `AddMana(1, AnyColor)` already wants this kind.

**TokenSpec / Characteristics**: the payload of create (CR 701.7a: number +
characteristics) and of becomes/copy effects. Needs: full CardFace-shaped
characteristics, the predefined-token shorthand (CR 111.10: Treasure, Food,
Gold, Walker, Shard, Clue, Blood, Powerstone, Incubator, the seven Role
tokens, Map, Junk, Lander, Mutagen — the repo's `plugins/builtin/tokens/`
is this), "token copy of [reference] except…" (CR 707.2 copiable values +
exception list), and entering-state riders ("tapped and attacking").
Corpus: "create … token" 3,232 lines (5.5%); Treasure 321, Food 138, Blood
56, Clue 27, Map 12.

**Mode / Choice**: modal blocks — "Choose one —" 433, choose two 44, "one
or both" 46, "up to" 29, "any number" 15; bullets 1,640 lines; repeatable
modes (CR 700.2d), other-player choosers (700.2e), per-mode costs
(spree/tiered/escalate), pawprint-budget modes (700.2i), per-mode targets
(700.2c/115.8). Plus vote specs (701.38), villainous choices (701.55), and
pile-splitting (700.3). Sagas (523 chapter lines) and Classes/levelers (50
LEVEL lines) are *positionally* gated mode-lists — chapter/level number is
the gate, so they reuse Condition + Ability rather than needing a new kind.

**Restriction / Requirement** (combat and play): "can't attack/block",
"must attack [whom] if able", "can't be blocked except by…" (evasion!),
"attacks each combat if able" (goaded), activation restrictions ("Activate
only as a sorcery" — 921 "Activate only" lines; CR 602.1b), casting
restrictions/permissions ("can't cast", "cast only during…", "you may cast
from the graveyard"), "as though" permissions (CR 609.4). These are mostly
rules-layer ContinuousEffects, but they recur enough to deserve named
sub-vocabularies — evasion macros are Filter-parameterized values of the
block-restriction family.

---

## 10. Ability, revisited

What the current `Ability` enum grows into, slot-wise (CR 113.3):

| Variant | Slots |
|---|---|
| Spell | effect: Effect (+ targets declared by the Effect's binders) |
| Activated | cost: Cost, effect: Effect, restrictions: [ActivationRestriction], flags: mana-ability (605.1a), loyalty (606) |
| Triggered | event: Event, condition: Option<Condition> (intervening if), effect: Effect, limits (once-each-turn), flags: mana-ability (605.1b), delayed?, reflexive? |
| Static | continuous: [ContinuousEffect] \| Replacement \| Prevention \| CastModifier \| Requirement, condition: Option<Condition>, flags: CDA (604.3), functions-in-zones (113.6 exceptions) |
| Keyword | already modeled: name + expansion (`Expanded<Ability>`) — the macro layer |

Cross-cutting flags the CR forces: where the ability functions (113.6a–p:
graveyard-only like unearth, hand-only like cycling, stack, command,
"as … enters"), linked-ability pairing (607 — likely an index/key shared
between two abilities on a face), and granted-vs-printed provenance
(113.10).

**Keyword classification** (from the CR 702 sweep of ~191 keywords): the
overwhelming majority are **pure macro sugar** expanding to the kinds above
— all "[cost]: effect" activated keywords (equip, cycling, ninjutsu, crew…),
all templated trigger keywords (annihilator N, prowess, afterlife N…), all
cast-modifier keywords (flashback, kicker, convoke, affinity…), all
enters-with/as-enters keywords (riot, devour N, fabricate N…). A short list
needs **first-class engine support** and can't be expanded away: protection
(five sub-effects parameterized by one Quality, CR 702.16b–f), ward
(counter-unless-pays, 702.21), hexproof/shroud (targeting legality),
evasion + menace/skulk-style block-legality predicates, the combat-damage
primitives (first/double strike 510.4, trample, deathtouch, banding),
damage-result modifiers (lifelink, wither, infect, absorb), indestructible,
phasing, and the designation system (§8). Keyword *parameters* observed:
none, N, Cost, "N—[cost]", Quality/Filter, CardName (partner with), subtype,
enumerated label (gift); keyword names are an open set (the corpus residual
is full of set-specific Warp {2}{R}, Firebending 1, Exhaust, Station —
data-driven, never a closed Rust enum).

---

## 11. The kind list, consolidated

Candidate `MacroKind`s / core types, roughly in dependency order. ✓ = exists
today.

| Kind | What it is | Macro examples |
|---|---|---|
| Subtype ✓ | open vocabulary, declared | `LandType("Forest")`, `Forest` |
| CardFace ✓ | a face | `Vanilla(name:, cost:, …)` |
| Ability ✓ | the five variants, §10 | `Flying`, `Hexproof`, `Boast(cost:)` |
| Target ✓ → Filter + binding | §1–2 | `AnyTarget`, `TargetCreatureYouControl` |
| **Filter** | predicate over objects/players | `Historic`, `Modified`, protection qualities, evasion blocker-sets |
| **Reference** | bound variables: targets, event participants, linked info, self, attachments | `Target(0)` ✓ (as `Selector`), `That`, `ExiledWith` |
| **Event** | trigger selectors | `Dies(Filter)`, `Landfall`, `BeginningOfUpkeep(Whose)` |
| **Condition** | state/history predicates | `Threshold`, `Delirium`, `Morbid`, `Kicked` |
| **Cost** | component vector | `Tap`, `SacrificeThis` (Treasure.ron already writes these) |
| **Effect** | one-shot AST over Actions | `Airbend(target)`, `Investigate`, `Bolster(2)` |
| **ContinuousEffect** | (set, modification, duration) | `Anthem(+1,+1, Filter)` |
| **Replacement / Prevention** | the §7 template shapes | `EntersTapped`, `Regenerate` |
| **Duration** | closed-ish vocabulary | `UntilEndOfTurn` |
| **Quantity** | literal/X/count/equal-to/anaphora | `CountOf(Filter)`, `Devotion(Green)` |
| **Zone** | zone + whose + position + facing | `TopOfLibrary(You)` |
| **CounterKind** | open vocabulary, declared | `PlusOnePlusOne`, keyword counters |
| **Designation** | open vocabulary, declared, §8 | `Monstrous`, `Goaded`, `TheMonarch` |
| **TokenSpec** | characteristics / predefined / copy-except | `Treasure` ✓ (tokens dir), `Walker` |
| **ManaSpec** | produced mana | `AnyColor` (Treasure.ron writes this) |
| **Mode/Choice** | modal machinery | — |
| **CastModifier** | alt/additional costs, permissions, reductions | `Flashback(cost)`, `Affinity(Filter)` |
| **Restriction/Requirement** | combat/activation/casting rules text | `CantBlock`, `AttacksEachCombatIfAble` |

Not every row needs to be a `MacroKind` — only positions where bare *names*
should expand (Filter, Event, Condition, Effect, Cost, Designation,
CounterKind, TokenSpec clearly qualify; Duration or Mode may never need
named macros). But each needs a serde type with a stable position name, and
the open-vocabulary rows (Subtype ✓, CounterKind, Designation, keyword
names, ability words) need **declaration support** à la
`MacroSet::declare`, so plugins can mint new ones alongside the mechanic
that introduces them.

### Corpus-driven priority

By line coverage, the build order that pays off fastest:

1. **Filter + binding + Quantity** — underlies 21.9% target lines, 13.5%
   you-control, 9.4% each-selectors; nothing else can be expressed without it.
2. **Cost + Action verbs** — unlocks activated abilities (16.2%) and the
   shelved tokens migration (Treasure/Clue/Food already written against it).
3. **Event + Reference + Condition** — unlocks triggers (22.4%) incl.
   intervening-if and ability words.
4. **Effect AST** (sequencing, you-may, if-you-do, for-each) — unlocks
   spell one-shots (12.4%) and trigger/activated bodies.
5. **ContinuousEffect + Duration** — statics (~13%) and the until-EOT 9%.
6. **Replacement/Prevention, Mode, CastModifier** — the long middle tail.
7. **Designation, CounterKind, TokenSpec, ManaSpec, Zone** — small closed/
   open vocabularies; cheap to define early, needed by everything above;
   designation and counter declarations should land with the macro-
   declaration mechanism.

### Open questions

- One Filter over objects *and* players, or split ObjectFilter/PlayerFilter
  with a union at target positions? ("any target" argues for union.)
- Is `Target` a distinct kind, or `Binding { quantifier, filter }` with
  target as one quantifier? (Retargeting and per-instance identity, CR
  115.3/115.7, argue targets stay structurally special.)
- How are References keyed — positional indices (`Target(0)`, current
  `Selector`) vs named roles (`that_creature`)? Named survives editing;
  positional matches CR 607's "linked" precision. Likely both: indices for
  targets, roles for event bindings.
- How much engine semantics leaks into data? Protection/ward/evasion need
  first-class engine rules; the data just names them with a Filter
  parameter. The line: data declares *what*, the engine owns *how* (layers,
  timestamps, lock-in, look-back-in-time).
- Designation payload abilities (suspected's menace): reference an Ability
  inline in the declaration, or by name? Inline keeps declarations
  self-contained, matching how macros already inline bodies.
- The residual census says ~7.7% of lines resist the main buckets — mostly
  ability-worded triggers, cardname-led damage, CDAs, and set-specific
  keyword-with-cost mechanics. None broke the taxonomy; all decompose into
  the kinds above. The open-ended keyword tail confirms keywords must stay
  data (macros), never a Rust enum.
