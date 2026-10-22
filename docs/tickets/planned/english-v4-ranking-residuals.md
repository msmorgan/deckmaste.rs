---
needs: []
---
# Ranking Key residuals: units whose cheapest tier is not the endorsed Reading

This ticket tracks every known Ranking Key residual: a Corpus Unit or Judgment
whose cheapest Ranking Key tier holds an unendorsed Reading, either instead of
the endorsed one or tied with it (`docs/decisions/english-v4.md` §4.3).
Owner's direction (relayed 2026-10-22): residuals are tracked, never
"accepted". Admission keeps every Reading English admits (§4.4, item 1), so the
only fix for a row is a better Ranking Key or a grammar fix that removes a
Reading no English grammar admits. No row is closed by a suppression mechanism
(§4.4, item 2).

## How this list is used

- Any structural change to the Ranking Key, or to a Production Cost, is measured
  against every row below. A change that fixes rows without regressing the
  judgment set is welcome. A change that breaks a resolved row, or moves a
  judgment-set entry out of an all-endorsed cheapest tier, is a regression.
- Each row has one identifying entry in the block at the end, in judgment
  syntax, with `ranks first`. A row is resolved exactly when its entry holds:
  run the block through `cargo xtask english-v4 parse --limit 1` (one quoted
  argument per line). Today rows 16 and 17 hold and every other row fails with
  "an unendorsed Reading is in the cheapest tier". Without `ranks first`, all
  23 hold: the endorsed Reading is in every forest.
- When a row is resolved, its `ranks first` entry goes into the judgment set (a
  new judgment for a unit that has none), and the row is marked with the landing
  that resolved it.
- A newly found residual is added as a row, with its entry.

## The residuals

Measured on `tztrqzts` with `cargo xtask english-v4 parse`, with card scope
wherever the text names a card. A key is AttachmentClass/RightAssociationWords/
ProductionCost, as `parse` prints it. "Endorsed key" is the key of the cheapest
Reading that satisfies the row's entry; "ahead" counts the Readings ranked
before it.

| # | Unit | Recorded in | Cheapest key (tier) | Endorsed key | Status |
|---|---|---|---|---|---|
| 1 | Teferi, Master of Time, line 1 (*loyalty abilities of Teferi on any player's turn …*) | `english-v4-unextended-and-closures` | 0/5/34 (1) | 0/15/34 | open; 18 ahead |
| 2 | Ember Island Production, first mode (*except it's not legendary and it's a 4/4 Hero …*) | `english-v4-set-off-first-conjunct` | 0/22/48 (2) | 0/24/48 | open; 2 ahead |
| 3 | Ember Island Production, second mode (*… a 2/2 Coward …*) | `english-v4-set-off-first-conjunct` | 0/24/49 (2) | 0/25/49 | open; 2 ahead |
| 4 | Mishra, Eminent One (*except its name is Mishra's Warform and …*) | `english-v4-set-off-first-conjunct` | 0/34/87 (1) | 0/35/87 | open; 1 ahead |
| 5 | Nightmare Shepherd (*except it's 1/1 and it's a Nightmare …*) | `english-v4-set-off-first-conjunct` | 0/18/65 (1) | 0/22/65 | open; 1 ahead |
| 6 | Ratadrabik of Urborg (*except it's not legendary and it's a 2/2 black Zombie …*) | `english-v4-set-off-first-conjunct` | 0/18/57 (2) | 0/28/57 | open; 4 ahead |
| 7 | Spark Double (`enter_as_copy_except_clause.txt:23`) | `english-v4-set-off-first-conjunct` | 0/51/65 (3) | 0/51/65 | tied; 1 of 3 endorsed |
| 8 | Ray of Ruin (*target creature, Vehicle, or nonbasic land*) | `english-v4-attributive-participle-licence` | 0/0/20 (1) | 0/1/17 | open; 1 ahead |
| 9 | Rob the Hoard (*target artifact, enchantment, or legendary card*) | `english-v4-attributive-participle-licence` | 0/1/21 (1) | 0/5/24 | open; 1 ahead |
| 10 | Abuelo's Awakening (`noun_premodifiers.txt:23`) | `english-v4-compound-noun-premodifier-coordination` | 0/16/56 (1) | 0/20/56 | open; 3 ahead (Noun Phrase alone: 0/0/9 against 0/1/9) |
| 11 | Excava, the Risen Past (`noun_premodifiers.txt:24`) | `english-v4-compound-noun-premodifier-coordination` | 0/20/71 (1) | 0/25/71 | open; 3 ahead (Noun Phrase alone: 0/2/22 against 0/7/22) |
| 12 | Nameless Race, *plus* (`comparative_quantity_than.txt:16`) | `english-v4-fused-comparative-predicative` | 0/12/61 (1) | 0/16/61 | open; 2 ahead |
| 13 | `enter_as_copy_complement.txt:11` (*with … equal to this creature's power and as a Mutant*) | `english-v4-right-association-key` | 0/37/43 (1) | 0/43/40 | open; 5 ahead |
| 14 | `remove_source_attachment.txt:6` (*you may untap it and remove it from combat*) | `english-v4-right-association-key` | 0/0/22 (1) | 0/2/21 | open; 3 ahead |
| 15 | `gapped_coordination_shared_adjunct.txt:11` (*in any order*) | `english-v4-right-association-key` | 0/1/32 (1) | 0/8/31 | open; 2 ahead |
| 16 | `kind_of_counter.txt:9` (*counter on each Spider and legendary creature you control*) | `english-v4-right-association-key` | 0/-7/28 (1) | 0/-7/28 | resolved by `english-v4-attachment-classes` |
| 17 | Seal of the Guildpact (`predicative_gap_relatives.txt:5`) | `english-v4-attachment-classes` | 0/-2/35 (2) | 0/-2/35 | resolved by `usvxoorq` (the partitive Noun Phrase carries `DeterminativeDistribution`) |
| 18 | Xyris, the Writhing Storm (`pro_form_one.txt:4`) | partitive *each of* loss after `usvxoorq` | 0/-6/48 (1) | 0/2/48 | open; 5 ahead |
| 19 | Birgi, God of Storytelling (`clause_supplements_and_adjuncts.txt:5`) | partitive *each of* loss after `usvxoorq` | 0/7/27 (4) | 0/7/27 | tied; 1 of 4 endorsed |
| 20 | Temple of Cyclical Time, line 2 (`coordinated_focused_adjuncts.txt:4`) | `english-v4-excise-predicate-conjunct-hosts` | 0/7/37 (2) | 0/8/37 | open; 2 ahead |
| 21 | Hedron Alignment, line 2 (*in exile, in your hand, in your graveyard, and on the battlefield*) | `english-v4-excise-predicate-conjunct-hosts` | 0/7/58 (2) | 0/8/58 | open; 2 ahead |
| 22 | `no_more_than.txt:13` (*Activate no more than three times each turn.*) | `english-v4-frequency-np-direct-object` | 0/1/16 (1) | 0/6/14 | open; 1 ahead |
| 23 | Agency Outfitter (`coordination.txt:73`) | `english-v4-agency-outfitter-coordination-ranking` | 0/7/58 (1) | 0/18/58 | open; owned by that ticket |

## What the cheapest tier holds instead

- **Row 1.** The garden path *loyalty [abilities of Teferi on any player's]
  turn any time …*: a bare relative whose verb is *turn*. The endorsed Reading
  uses the face's Shortened Name. The same tree with the subtype *Teferi* costs
  0/15/32.
- **Rows 2–6.** [[C, except A] and B] ranks ahead of C, except [A and B]. The
  owner's round-5 ruling (item 7a) asked the Ranking Key to rank C, except
  [A and B] first. The round-6 ruling left these units un-tuned.
- **Row 7.** The tier also holds *if* [*it's a creature, it enters … , and it
  isn't legendary*] and [[C, except A], B, and C].
- **Rows 8–11.** These are coordinations of type words before a carrier noun
  (`docs/oracle-style-guide.md` §7, "Card types are common nouns and
  attributive modifiers"; the shortest useful checklist, item 6). Ray of Ruin
  names permanents. Its endorsed Reading is the Nominal coordination
  [*creature*], [*Vehicle*], or [*nonbasic land*], but the Attributes
  coordination [*creature, Vehicle, or nonbasic*] *land* ranks first. Rob the
  Hoard, Abuelo's Awakening and Excava name graveyard cards. Their endorsed
  Reading lets *card* distribute over the coordination, but the Nominal split
  with a bare *artifact* conjunct ranks first. The two directions conflict:
  whichever structure a change ranks first fixes one side and breaks the other.
- **Row 12.** *plus …* attaches inside the *of* Complement of the first
  *number* (*number of [white nontoken permanents your opponents control plus
  the total number of …]*). The endorsed Reading adds two Noun Phrases.
- **Row 13.** Conjunct balance coordinates *equal to this creature's power*
  with *as a Mutant* rather than the *with* phrase with the *as* phrase. The
  endorsed Reading also has *equal to …* postmodify *number*.
- **Row 14.** *from combat* becomes an Adjunct of the coordination [*untap it*]
  *and* [*remove it*] under *may*: the coordination host is free and the
  conjuncts balance. The endorsed Reading has *from combat* as *remove*'s
  Complement, and *may* over both conjuncts.
- **Row 15.** *in any order* postmodifies *library* rather than modifying
  *put*.
- **Rows 18–19.** Since the partitive *each of* is distributive, the
  distributive Attachment Class attaches *in each of their draw steps* to the
  matrix *draws*, outside the relative clause *they draw …*. It also attaches
  *during each of your turns* to *can* rather than *boast*.
- **Row 20.** The tier holds *only* [[*if …*] *and* [*only as a sorcery*]] and
  *has no time counters* [[*on it*] *and* [*only as a sorcery*]]. The judgment
  endorses [*only if …*] *and* [*only as a sorcery*]. It does not settle whether
  *on it* is on *counters* (0/8/37) or on *has* (0/10/37).
- **Row 21.** The tier puts the locative coordination on *named*, or closes an
  *if*-clause conjunct after *in exile*. Endorsed: the coordination on *card*
  (0/8/58) or on *own* (0/10/58).
- **Row 22.** *activate* takes *no more than three times each turn* as its
  Object. `english-v4-frequency-np-direct-object` records why Right
  Association prefers it, and the measured cost of reordering the components.
- **Row 23.** See `english-v4-agency-outfitter-coordination-ranking`.

## Done when

- Every row's entry holds, or the row is resolved by a grammar fix that
  removes only Readings no English grammar admits, with a CGEL or
  `docs/oracle-style-guide.md` citation (§4.3, last paragraph).
- Every fix is structural (§4.3): no component keyed on a word, a lexeme or a
  catalog entry, and no suppression mechanism (§4.4, item 2).
- No judgment-set entry leaves an all-endorsed cheapest tier, and no face is
  lost.
- Each resolved row has its `ranks first` judgment in the judgment set.

Standard constraints apply.

## Identifying entries

One per row, in row order.

```text
+ <card=Teferi, Master of Time> You may activate loyalty abilities of Teferi on any player's turn any time you could cast an instant. => NounPhrase "loyalty abilities of Teferi" as Object of activate; PrepositionPhrase "on any player's turn" as Adjunct of activate; Name "Teferi"; ranks first
+ <card=Ember Island Production> Create a token that's a copy of target creature you control, except it's not legendary and it's a 4/4 Hero in addition to its other types. => Clause "it's not legendary and it's a 4/4 Hero in addition to its other types" as Complement of except; ranks first
+ <card=Ember Island Production> Create a token that's a copy of target creature an opponent controls, except it's not legendary and it's a 2/2 Coward in addition to its other types. => Clause "it's not legendary and it's a 2/2 Coward in addition to its other types" as Complement of except; ranks first
+ <card=Mishra, Eminent One> At the beginning of combat on your turn, create a token that's a copy of target noncreature artifact you control, except its name is Mishra's Warform and it's a 4/4 Construct artifact creature in addition to its other types. It gains haste until end of turn. Sacrifice it at the beginning of the next end step. => Clause "its name is Mishra's Warform and it's a 4/4 Construct artifact creature in addition to its other types" as Complement of except; ranks first
+ <card=Nightmare Shepherd> Whenever another nontoken creature you control dies, you may exile it. If you do, create a token that's a copy of that creature, except it's 1/1 and it's a Nightmare in addition to its other types. => Clause "it's 1/1 and it's a Nightmare in addition to its other types" as Complement of except; ranks first
+ <card=Ratadrabik of Urborg> Whenever another legendary creature you control dies, create a token that's a copy of that creature, except it's not legendary and it's a 2/2 black Zombie in addition to its other colors and types. => Clause "it's not legendary and it's a 2/2 black Zombie in addition to its other colors and types" as Complement of except; ranks first
+ <card=Spark Double> You may have this creature enter as a copy of a creature or planeswalker you control, except it enters with an additional +1/+1 counter on it if it's a creature, it enters with an additional loyalty counter on it if it's a planeswalker, and it isn't legendary. => PrepositionPhrase "except it enters with an additional +1/+1 counter on it if it's a creature, it enters with an additional loyalty counter on it if it's a planeswalker, and it isn't legendary" as Adjunct of may; !PrepositionPhrase "except it enters with an additional +1/+1 counter on it if it's a creature, it enters with an additional loyalty counter on it if it's a planeswalker, and it isn't legendary" as Adjunct of control; !PrepositionPhrase "except it enters with an additional +1/+1 counter on it if it's a creature" as Adjunct of control; Clause "it enters with an additional +1/+1 counter on it if it's a creature"; PrepositionPhrase "if it's a creature" as Adjunct of enter; ranks first
+ <card=Ray of Ruin> Exile target creature, Vehicle, or nonbasic land. Scry 1. => Nominal "creature, Vehicle, or nonbasic land"; Nominal "nonbasic land"; ranks first
+ <card=Rob the Hoard> Return target artifact, enchantment, or legendary card from your graveyard to your hand. => Attributes "artifact, enchantment, or legendary" as Modifier of card; ranks first
+ <card=Abuelo's Awakening> Return target artifact or non-Aura enchantment card from your graveyard to the battlefield with X additional +1/+1 counters on it. It's a 1/1 Spirit creature with flying in addition to its other types. => Nominal "artifact or non-Aura enchantment" as Modifier of card; Nominal "non-Aura" as Modifier of enchantment; ranks first
+ <card=Excava, the Risen Past> Whenever Excava attacks, return up to one target artifact, creature, or non-Aura enchantment card with mana value 3 or less from your graveyard to the battlefield with a finality counter on it. It's a 1/1 Spirit creature with flying in addition to its other types. => Nominal "artifact, creature, or non-Aura enchantment" as Modifier of card; Nominal "non-Aura" as Modifier of enchantment; ranks first
+ <card=Nameless Race> As this creature enters, pay any amount of life. The amount you pay can't be more than the total number of white nontoken permanents your opponents control plus the total number of white cards in their graveyards. => NounPhrase "more than the total number of white nontoken permanents your opponents control plus the total number of white cards in their graveyards" as PredicativeComplement of be; NounPhrase "the total number of white nontoken permanents your opponents control plus the total number of white cards in their graveyards" as Complement of than; NounPhrase "the total number of white nontoken permanents your opponents control"; NounPhrase "the total number of white cards in their graveyards"; !DeterminerPhrase "more than the total"; !DeterminerPhrase "more than the total number of white nontoken permanents your opponents control plus the total number of white"; ranks first
+ Each other creature you control enters with a number of additional +1/+1 counters on it equal to this creature's power and as a Mutant in addition to its other types. => PrepositionPhrase "with a number of additional +1/+1 counters on it equal to this creature's power"; AdjectivePhrase "equal to this creature's power" as Modifier of number; ranks first
+ Whenever this creature becomes blocked, you may untap it and remove it from combat. => PrepositionPhrase "from combat" as Complement of remove; !PrepositionPhrase "from combat" as Adjunct of remove; VerbPhrase "untap it and remove it from combat" as Complement of may; ranks first
+ Put one of them on top of your library in any order and the rest on the bottom of your library. => NounPhrase "one of them" as Object of put; NounPhrase "the rest" as Object of put; PrepositionPhrase "on top of your library"; PrepositionPhrase "in any order"; PrepositionPhrase "on the bottom of your library"; PrepositionPhrase "in any order" as Adjunct of put; ranks first
+ Whenever you attack, double the number of each kind of counter on each Spider and legendary creature you control. => PrepositionPhrase "of counter" as Complement of kind; ranks first
+ <card=Seal of the Guildpact> Each spell you cast costs {1} less to cast for each of the chosen colors it is. => RelativeClause "it is" gap "chosen colors"; PrepositionPhrase "for each of the chosen colors it is" as Adjunct of cost; ranks first
+ <card=Xyris, the Writhing Storm> Whenever an opponent draws a card except the first one they draw in each of their draw steps, create a 1/1 green Snake creature token. => RelativeClause "they draw in each of their draw steps" gap "first one"; ranks first
+ <card=Birgi, God of Storytelling> Creatures you control can boast twice during each of your turns rather than once. => VerbPhrase "boast twice during each of your turns rather than once" as Complement of can; AdverbPhrase "twice" as Adjunct of boast; PrepositionPhrase "during each of your turns" as Adjunct of boast; ranks first
+ <card=Temple of Cyclical Time> {2}{U}, {T}: Transform this land. Activate only if it has no time counters on it and only as a sorcery. => PrepositionPhrase "only if it has no time counters on it"; PrepositionPhrase "only as a sorcery"; ranks first
+ <card=Hedron Alignment> At the beginning of your upkeep, you may reveal your hand. If you do, you win the game if you own a card named Hedron Alignment in exile, in your hand, in your graveyard, and on the battlefield. => PrepositionPhrase "in exile, in your hand, in your graveyard, and on the battlefield" as Modifier of card || PrepositionPhrase "in exile, in your hand, in your graveyard, and on the battlefield" as Adjunct of own; ranks first
+ Activate no more than three times each turn. => NounPhrase "three times" as Complement of than; ranks first
+ <card=Agency Outfitter> When this creature enters, you may search your graveyard, hand, and/or library for a card named Magnifying Glass and/or a card named Thinking Cap and put them onto the battlefield. If you search your library this way, shuffle. => NounPhrase "your graveyard, hand, and/or library" as Object of search; NounPhrase "a card named Magnifying Glass and/or a card named Thinking Cap" as Complement of for; VerbPhrase "put them onto the battlefield"; VerbPhrase "search your graveyard, hand, and/or library for a card named Magnifying Glass and/or a card named Thinking Cap and put them onto the battlefield" as Complement of may; ranks first

```

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
