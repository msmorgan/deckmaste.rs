# Union spellings — measured tables for the declaration author

What English writes when a phrase spans an object and a player. Four
constructions: the **union head** ("target player or planeswalker"), the
**mixed group** ("you and permanents you control"), the **class word** ("any
target") [CR#115.4], and the **demonstrative readback** that points at one of
them ("that permanent or player").

None of the four is declared in `src/constructions.rs` yet. This file is where
their spelling-boundary knowledge lives until they are, and it is the input a
declaration for any of them is written from.

Why here and not in the semantics: the semantics layer admits anything the
rules make meaningful, and a joined kind carries no marking — attestation,
order, modifier discipline and the demonstrative's word choice are facts about
English, so they are declared at this boundary. Authority:
`docs/decisions/kind-index-joins-union-marking-is-spelling.md`,
`docs/decisions/english-grammar-is-derived.md`, and the ruling in
`docs/memory/rulings/measurements-live-in-pins.md`.

## Method

Every count below is **distinct supported oracle-text lines**, measured with
the `mtg-rules` skill's `scripts/corpus --match <regex>` (default scope:
vintage-legal, non-reversible printings), re-measured 2026-08-22. Regexes are
case-insensitive PCRE and are printed with each table; two abbreviations
expand as written:

~~~text
DET     = (?:a |an |the |target |each |another |any |one |single |up to one target |up to one |chosen )?
NOTDEM  = (?<!that )(?<!those )(?<!chosen )
~~~

`NOTDEM` is what separates a **head** from a **readback**: the same word pair
spells both, and the demonstrative determiner is the only local signal telling
them apart. A line bearing a construction twice counts once — the unit is the
line, not the occurrence.

The figures in `docs/tickets/wip/workbench-union-gate-spelling-rehome.md` were
quoted from Idris docstrings that no longer exist, so their regexes are
unrecoverable and the two sets are **not** reconcilable line by line. Where
mine differs, both are stated. The differences are not noise: they concentrate
in exactly the two places where head/readback and reminder-text inclusion are
judgment calls.

## 1. The union head

A player word and a class word coordinated by "or", not under a demonstrative.

~~~text
NOTDEM\b(player|opponent)s? or DET(planeswalker|permanent|battle|creature)s?\b
|NOTDEM\b(planeswalker|permanent|battle|creature)s? or DET(player|opponent)s?\b
~~~

**319 lines** (ticket: 326).

### The admissible-pair grid

Two parameters, not a flat enum of the combinations somebody happened to
write. All eight cells attested. Per cell, the regex above with the two
alternants fixed to that pair.

| | planeswalker | permanent | battle | creature | **total** |
|---|---|---|---|---|---|
| **player** | 194 | 48 | 13 | 3 | **258** |
| **opponent** | 42 | 15 | 3 | 1 | **61** |
| **total** | **236** | **63** | **16** | **4** | **319** |

Ticket: player 279 / opponent 45; planeswalker 205, permanent 100, battle 17,
creature 4; total 326. Creature (4) and battle (16 vs 17) agree. Planeswalker
(236 vs 205) and permanent (63 vs 100) do not, and the two errors run
opposite ways while the totals nearly agree — consistent with the ticket
counting some "that permanent or player" lines as heads and excluding some
planeswalker lines. 37 of the planeswalker lines are the trample reminder
"(It can deal excess combat damage to the player or planeswalker it's
attacking.)"; excluding them gives 199, which does not reach 205 either. The
ticket's own two subtotals are already inconsistent: 279 + 45 = 324, not 326.

The cross-kind head is admitted by [CR#115.1] — "The targets are object(s)
and/or player(s)". Its class inventory is **not** the rules': [CR#115.4] names
creatures, players, planeswalkers and battles, which is the class word's list,
and the head's second-largest cell is "permanent" (63 lines), a word that rule
does not use. The two heads do not draw on the same inventory.

### Order is free variation

Per class, player-half-first versus class-half-first (the two alternants of
the head regex, counted separately).

| class | player-first | object-first |
|---|---|---|
| planeswalker | 236 | 0 |
| permanent | 17 | 46 |
| battle | 15 | 1 |
| creature | 0 | 4 |

Ticket: permanent 92 object-first / 8 player-first; battle 16 player-first /
1 object-first. Battle agrees to within the one-line total gap. Permanent
does not (mine 46/17 of 63; the ticket's 92/8 of 100).

The covariance test still fails: order does not track the class. Permanent
prefers object-first, battle and planeswalker player-first, creature is
object-first only — the class word does not predict the order, and both
orders are attested for permanent and for battle.

### Modifier discipline

Head lines where a half carries a post-nominal modifier:

~~~text
(player|opponent)s? or (a |an |the |target |each |another |any |single )*(planeswalker|permanent|battle|creature)s? (an?|that|they|you|your|it) 
~~~

applied to the 319 head lines: **22** (ticket: 11 of 326). Distinct
spellings:

| spelling | lines |
|---|---|
| "an opponent or a permanent an opponent controls" | 13 |
| "the player or planeswalker that creature is attacking" | 2 |
| "which player or permanent that creature is attacking" | 1 |
| "one of your opponents or a planeswalker they control" | 1 |
| "one of your opponents or a planeswalker an opponent controls" | 1 |
| "enchanted opponent or a planeswalker they control" | 1 |
| "a player or planeswalker that opponent is attacking" | 1 |
| "attacking defending player or a planeswalker they control" | 1 |
| "a player or planeswalker with one or more creatures" | 1 |

The last is an attachment false positive — the PP attaches to "attack", not
to the head — so 21 are genuine. Two further lines carry a pre-nominal
modifier the regex above does not reach ("another target battle or opponent",
"another target permanent or opponent"), and two more spell "which player or
permanent target attacking creature is attacking". A declaration must admit
at least: a relative clause on the whole pair, a relative clause on the class
half alone, and "another".

### Not coordinable, not negatable

Both are corpus zeros, and both hold as declarations rather than as
refusals — a union head nested in a further disjunction would spell the same
union twice over, and a union of two classes is not a property a thing can
lack.

~~~text
(permanent|creature|planeswalker|battle)s? or (a |an |the |target )?(player|opponent)s?,? or 
~~~

**0 lines.**

~~~text
(isn't|aren't|non)[a-z']* (a |an )?(permanent|creature|planeswalker|battle)s? or (a |an )?(player|opponent)s?
|(player|opponent)s? or (a |an )?(permanent|creature|planeswalker|battle)s? (isn't|aren't)
~~~

**0 lines.**

## 2. The mixed group

"you" coordinated with an object noun phrase.

~~~text
\byou and(?:/or)? (?:(?:a|an|the|target|each|this|all|other|another|up to one target) )*(creature|permanent|planeswalker|artifact|enchantment|land|battle|token)s?\b
~~~

**36 lines** (ticket: 35). Splitting on the conjunction: **"and" 28**,
**"and/or" 8** (ticket: 23 and 12).

"and" versus "and/or" is free variation, and the crossing pair is exact
(verified by `scripts/card`):

- **Channel Harm** — "…would be dealt to you **and** permanents you control
  this turn…"
- **Refraction Trap** — "…would deal to you **and/or** permanents you control
  this turn."

### The player half

Fixed at "you" in all 36 — the deictic, resolving to the object's controller
[CR#109.5]. The nearest competing spelling is 0:

~~~text
target player and (the )?creatures (they|that player) controls?
~~~

**0 lines** — agreeing with the ticket.

**But the family is not fixed at "you" once the object half is
distributive.** A separate measurement:

~~~text
\b(target|that|each|an) (player|opponent)s? and (a |an |the |all |each |target )?(creature|permanent|planeswalker|artifact|land)s? (they|that player|he or she) controls?
~~~

**14 lines**, all of the form "deals N damage to target player and **each
creature that player controls**" / "to each opponent and each creature they
control". The zero above is a fact about the *bare plural* spelling
("creatures they control"), not about a non-"you" player half. A declaration
that hard-wires "you" will refuse those 14 printed lines.

### The object half

Attested right halves, all with "you control" or bare:

bare plural ("permanents you control", "creatures you control",
"planeswalkers you control"), distributive ("each creature you control",
"each permanent you control"), complement ("other permanents you control"),
indefinite ("a planeswalker you control"), the self ("this creature"), a
target mention ("target creature", "target permanent you control").

Not attested in these 36: a counted group, and the class word.

### It refuses itself

"you and you" appears on 3 lines: two are "deals 1 damage to you and you draw
a card" (a clause coordination) and one is "a creature is attacking you and
you control a Forest and a Plains" (a conditional). None is a nested mixed
group, and no line nests one mixed group in another.

## 3. The class word

One lexical item, nullary [CR#115.4].

~~~text
\bany target\b
~~~

**748 lines.** Plural "any targets": **1 line.**

### Is it the union head under another spelling? No.

Four measurements, all still holding:

- No cross-kind head joins two classes to a player as this word does. The
  only spelling that comes close is `creature, planeswalker, or player`:
  **2 lines**, and both are this word's own pre-battle spelling (a damage
  redirection and a damage-prevention grant), not a two-class union head.
  `creature, player, or planeswalker`: **0 lines.**
- The rules name it as one lexical item [CR#115.4].
- The two obey different modifier disciplines (below).
- They do not even draw on the same class inventory: the rule's list is
  creature, player, planeswalker, battle, while the head's second-largest cell
  is "permanent" (§1, 63 lines), which the rule never names.

### Modifier discipline

The ruling to record is: **the class word takes no modifier; the union head
does.** The head's count is §1's 22 of 319. The class word's is **not
zero**, and this is the one place the ticket's inventory is wrong on its own
terms rather than merely differently measured.

~~~text
\bany [a-z]+ target\b
~~~

**16 lines** — every one of them "any **other** target". That is the
complement, not a modifier: it subtracts a referent rather than naming a
member of the phrase's domain, and it is the same `OtherThan` the workbench
gave its own kind index (see `docs/tickets/done/workbench-join-is-a-constructor.md`).

~~~text
\bany target (you|an |a |that|other|with|in |they|your)
~~~

**5 lines:**

| spelling | count |
|---|---|
| "any target other than that permanent" | 1 |
| "any target that isn't a Dragon" | 1 |
| "any target that isn't a Dinosaur" | 1 |
| "any target that isn't a commander" | 1 |
| "any target that was dealt damage this turn" | 1 |

One is again the complement. **The other four are relative-clause modifiers
on the class word**, so the discipline is "no *pre*-nominal modifier, and a
restrictive relative clause only" — not "no modifier". 0 lines write "any
red target" or "any target you control", which is the gate that actually
matters and which the four attested clauses do not open: every one restricts
by a card characteristic or by a game-history predicate, never by a colour
adjective and never by control.

## 4. The demonstrative readback, and the collapse rule

This is the deliverable of
`docs/tickets/wip/workbench-unhomed-union-gates.md` §1: a spelling table
mapping antecedent to demonstrative words. It is a spelling decision, not a
lattice consequence — a powerset join gives the class word
`{creature, player, planeswalker, battle}` and the permanent head
`{permanent, player}`, two different sets, where the corpus writes one
surface.

Readback = a demonstrative over a union pair. Three surface forms:

~~~text
GENERIC = \b(that|those) (permanents? or players?|players? or permanents?)\b
ECHO    = \b(that|those) (player|opponent)s? or planeswalkers?\b
TWO     = \b(that|those) (player|opponent|planeswalker|permanent|battle|creature)s? or (that|those) 
~~~

`GENERIC|ECHO|TWO`: **68 lines** (ticket: 48). Cross-tabulated against what
else the line carries:

| antecedent on the line | lines | ticket |
|---|---|---|
| a union head (§1 regex) | 51 | 33 |
| the class word "any target" | **10** | **10** |
| neither (antecedent on an earlier line, or a paraphrase) | 7 | 5 |
| **total** | **68** | **48** |

The class-word row reproduces the ticket exactly. The union-head row is where
the two measurements part.

### The collapse table — antecedent → demonstrative words

| antecedent spelling | demonstrative words | lines |
|---|---|---|
| `target player or planeswalker` | "that player or that **planeswalker's controller**" | 20 |
| `target opponent or planeswalker` | "that opponent or that planeswalker's controller" | 1 |
| `target player or planeswalker` | "that player or planeswalker" (one demonstrative over the pair) | 4 |
| `target players or planeswalkers` (plural) | "those players or planeswalkers they control" | 1 |
| `a` / `target permanent or player` | "that permanent or player" | 28 |
| `any target` (class word) | "that permanent or player" | 8 |
| `any target` (class word) | "that player or that **permanent's controller**" | 2 |
| — (antecedent on the card's other line) | "that permanent or player" / "that player or planeswalker" | 5 |
| `<player> or battle` | *no readback attested* | 0 |
| `creature or player` | *no readback attested* | 0 |

The rows sum to 69 against 68 distinct lines: one line carries both forms.

Four rules fall out, and they are the collapse:

1. **The generic surface is "permanent or player", and it is what both the
   class word and the permanent-pair head spell.** 8 class-word lines and 28
   permanent-head lines write the identical string; **no line distinguishes
   the two antecedents**. Nor does the half-naming form distinguish them:
   Chain Lightning and Chain of Plasma announce "any target" and read it back
   as "that player or that **permanent's** controller", the same construction
   a permanent-pair head would use. A collapse rule must map the class word's
   four-element domain and the head's `{permanent, player}` onto this one
   surface, in both readback forms.
2. **A planeswalker pair is echoed, never genericised**, and the echo is
   exact in both directions. A planeswalker head paired with a generic
   readback: **0 lines**. A permanent head paired with a planeswalker-naming
   readback: **0 lines**. No crossing either way.
3. **Naming a half is spelled as a coordination of two demonstratives**, and
   the object half is reached through its controller: "that player or that
   planeswalker's **controller**" — 23 lines in all. This grammar has no noun
   disjunction, so the construction that spells it coordinates two full
   demonstrative noun phrases; it is not a demonstrative over a disjunction.
   Not one of the 68 is a bare "that player" or "that permanent" pointing at
   a union antecedent.
4. **One demonstrative word throughout.** All 68 use "that"/"those";
   `\bthose (players?|permanents?|planeswalkers?|creatures?|battles?|opponents?) or `
   is **1 line** — the plural row above — so the singular/plural choice is
   ordinary number concord_class and not a distinct anaphor.

Ticket figures for this section: 33 exact echoes, 15 generic with no pair to
echo, 43 sharing one demonstrative (33 + 10 class-word). Mine: 28 lines name
a pair or a half (5 single-demonstrative + 23 two-demonstrative, one line in
both), 41 are generic, and all 68 share the one demonstrative. The
class-word row (10 readbacks, 8 generic + 2 half-naming) and the "no card
distinguishes them" finding both reproduce; the union-headed total does not.

## 5. The measured zeros

### The seven verbs

No verb of the destroy family takes a union head or the class word as its
object. Per verb:

~~~text
\bVERB(s|es|ed|ing)? (?:up to (?:one|two|three|X|that many) )?(?:a |an |the |target |each |another |all |any number of )*((permanent|creature|planeswalker|battle)s? or (?:a |an |the |target )?(player|opponent)s?|(player|opponent)s? or (?:a |an |the |target )?(permanent|creature|planeswalker|battle)s?|any target)\b
~~~

| verb | lines |
|---|---|
| destroy | 0 |
| exile | 0 |
| tap | 0 |
| untap | 0 |
| return | 0 |
| counter | 0 |
| sacrifice | 0 |

Agrees with the ticket's 0. A looser window (allowing up to 30 characters
between verb and pair) returns 29 lines, every one of them a false positive:
the noun "counter", the card name "Rakdos's Return", "return target creature
or Vehicle" (object × object), and "exile up to that many target cards from a
player". A declaration author reproducing this measurement must anchor the
object NP to the verb.

**This zero is a spelling fact and nothing else.** The semantics refuses
these terms earlier and harder: each of those verbs names a `Noun bs Object`,
so a joined phrase is a type error before any gate is asked, and the 18 pins
that used to record the refusal were retired as unpinnable
(`docs/tickets/done/workbench-union-family-macros.md`). The zero is re-homed
here because a measured zero dropped in the move becomes a silent yes — but
it constrains the English declaration only.

### The rest

- Union head under a further disjunction: 0 (§1).
- Negated union head: 0 (§1).
- Mixed group nested in a mixed group: 0 (§2).
- "target player and creatures they control": 0 (§2) — but see the 14
  attested distributive lines there.

## What is deliberately absent

Facts a joined kind already proves are not written down here: the union's
placelessness, its absence of a card type, its admission as a damage
recipient. They follow from the joined kind having no zone and no type, and
writing a derivable fact down is the failure this direction exists to stop.
