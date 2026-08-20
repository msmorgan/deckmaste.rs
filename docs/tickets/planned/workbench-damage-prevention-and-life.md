---
needs: []
---
# Make damage events mentionable, mint their event rows, and close prevention and life suppression

The damage/prevention/life region taken as one claimable unit: the mention of a
damage event a previous clause described, the `GameEvent` rows the damage
announcement left standing (damage and life alike), the amounts the shield and
its cut refuse, the prevented damage's source and its trigger, and the life-gain
suppression measurement. They are one round because they read and write the same
declarations — `IsDealtDamage`, `DealsCombatDamage`, `Intercepts`, `Redirects`,
`Prevents`/`PreventCut`, `ThatMuch` and the outcome sorts — and the mention that
unblocks one unblocks most of the others.

## Making the replaced damage event mentionable

Three measured families are blocked on one thing: a replacement clause that
points back at the damage event it replaces — a portion of it, or its magnitude.
The bodies themselves are already writable; the mention is not. One mechanism
unblocks all three.

### The excess-damage redirect — 5 sentences over 5 cards

"Excess damage is dealt to that creature's controller instead" (Flame Spill,
Gandalf's Sanction, Pigment Storm, Ravenous Tyrannosaurus, Ram Through). Its
destination is `Redirects`' destination; its **subject** reads a *portion* of a
damage event the card's own previous sentence described.

### The redirect immunity — 2 sentences over 2 cards, verified

Lava Burst and Whippoorwill — **not** Arrow Storm, which an earlier recon named
and which writes the plain can't-be-prevented. A conjoined negation over
`CantPrevent` and `Redirects` **together**.

Both families are under the minting bar apiece; they are a round together with
the third.

### The other "instead" bodies — 16 sentences

Replacements of damage with something that is not damage: "put that many delay
counters on this enchantment instead" (Force Bubble, Delaying Shield), "that
player exiles that many cards from the top of their library" (Crumbling
Sanctuary), "put that many -1/-1 counters on that creature" (Soul-Scar Mage),
"sacrifice that many permanents" (Dralnu, Lich Lord).

These **are** `Intercepts` — a general `Effect` body is exactly right for them —
and every one is blocked on the same readback: the body's "that many" measuring
the replaced damage. This is the full audience for the magnitude read, so a
round that lands it should be checked against all 16.

## Making a damage event a noun the next clause can point at

Four families have now asked for the same thing from four sides: a mention of a
damage event a previous clause described. A damage clause leaves an `Outcome`
behind, which is a **magnitude** read by "that much" and never an event to point
at, so the ask is an event mention with an amount projection.

### The can't-be-prevented lines — 10

"The damage" / "that damage can't be prevented", reading a damage event their own
previous clause described: Combust, Banefire, Demonfire, Urza's Rage, Flames of
the Blood Hand, Volcano Hellion, Pinpoint Avalanche, Arrow Storm, Lightning
Surge, Lava Burst.

### The three redirection residues

The excess-damage redirect's portion-reading subject, the redirect-immunity pair,
and Flaming Gambit's and Megatron's anaphoric bodies. If the mention lands, the
can't-be-prevented row's anaphoric subject and all three fall out of it.

### What is not this, and must not be folded in

- **The redirection body did not need it.** The body was fixed to a redirection
  rather than left a general effect, so its "that damage" is the row's own
  constant and not an amount anything computes.
- **The event announcement is not this either.** A damage event now leaves an
  `Outcome` behind as well (`eventAfter`), so the magnitude is readable from both
  mouths — this asks for the event as a **noun**, and the distinction is
  worth keeping sharp.
- **The excess-damage redirect stays fenced**, re-measured against that
  announcement: its subject reads a **portion** of a damage event ([CR#120.10]
  computes it), not the event's whole magnitude, and no `Amount` row spells it.

## The event rows the damage announcement left standing

The two damage cells landed: `eventAfter` announces `outcomeB DamageDealt` at
`IsDealtDamage` (58 lines / 57 cards) and at `DealsCombatDamage` (77 / 77), and
the passive row mints its subject's mention besides, which is what lets a body say
"it" or "that creature" after the event. No sort was minted and no vocabulary.
The same sweep left five cells, **each blocked on a `GameEvent` row rather than
on the anaphor**, and two recorded prices.

### The active damage event with its source written — 43 lines over 42 cards

"Whenever a source you control deals damage to you, put that many +1/+1 counters
on this creature"; "Whenever enchanted creature deals damage, you gain that much
life". Plus Soul-Scar Mage on the replacement side. The biggest and the nearest:
it is the damage event with `DamageAgent`'s axis written, which the redirection
and prevention rows both carry already, so the vocabulary exists and only the
event row does not. It inherits the landed announcement for free.

### The replacement side of both damage events — 10 lines

Phytohydra, Lichenthrope, Delaying Shield, Force Bubble, Crumbling Sanctuary,
Nefarious Lich, Dralnu, Szadek, Undead Alchemist, Soul-Scar Mage. Blocked on the
**table cell**, not the announcement: `eventUse DamageTaken` and `eventUse
CombatDamage` are both `TriggeredOnly`, so `Intercepts` cannot take either event.
Flipping the cell is the whole ask and the `eventIntro` half follows it.

**Not this:** the 22 "deals that much damage plus N instead" lines. That phrase is
`DamageScale`'s own spelling and they need nothing.

### The life events — 14 gains, 8 losses or payments

"Whenever you gain life, target opponent loses that much life" (Sanguine Bond);
"Whenever an opponent loses life, you gain that much life" (Exquisite Blood);
Ageless Entity, Kavu Predator, Vilis, Mindcrank. No `GameEvent` row watches a
life change and [CR#119.3] is all the rules supply. The sorts they would announce
already exist (`LifeGained`, `LifeLost`) — an event row and nothing else.

### The batch-count events — the attack cell, and the rest below the bar

Twelve attack declarations read the attacking group's size ("Whenever one or more
Dragons you control attack, draw that many cards", The Ur-Dragon); six discards,
two exiles, two token entries, and one apiece of energy, mill and draw. Each is
the counter batch's shape at a different verb and each is under the round-minting
bar alone. **The attack cell is the only one worth its own round**, and it wants a
plural-subject group size where `eventSubjectPlur` records singularity.

### The damage kind on the event — 5 lines

"Is dealt combat damage" (Pious Warrior, Wall of Essence, Wall of Souls, Souls of
the Faultless) and "is dealt noncombat damage" (Smaug). `IsDealtDamage` has one
slot and no damage-kind axis — it is the shield row's adjective at an event
position. Small, and it lands beside the active-damage row.

### Recorded prices, not gaps

- **The double read — 3 lines.** "Whenever this creature is dealt combat damage,
  you gain that much life and attacking player loses that much life" (Souls of
  the Faultless; Kain, Neheb) reads one announcement twice, and the second read is
  refused because the first clause's own outcome joins the context. `ThatMuch`'s
  obligation is a uniqueness one and this is what it costs. Attested English, so
  not pinnable, and not fixable without a second reader.
- **The body-internal announcement — 60 lines over 59 cards**, deliberately not
  fenced. "Discard any number of cards, then draw that many cards" reads a
  quantity the **body** wrote, not the header's event; it belongs with
  `GroupSize`'s family and is the largest false friend in the sweep. Do not
  re-buy it as part of this one.

## The shield and its cut: the amounts they refuse

Three measured amounts sit on the same two prevention rows. The complement is one
value both rows would take, the half is the same row's other missing operation,
and the where-rider is the amount position the scaling family left behind. Opening
the cut answers all three.

### The complement amount, "all but [n]" — 9 supported sentences

Two constructions apart, which is why they are scheduled together: **4** are the
shield's determiner ("prevent all but 1 of the damage that would be dealt to you
this turn") and **5** the event-shaped row's cut ("the next time an unblocked
creature of your choice would deal combat damage to you this turn, prevent all
but 1 of that damage" — Forcefield; Ajani Steadfast's emblem, Hyperion, Temple
Altisaur). `Shield` and `PreventCut` both refuse it today and both would take the
same value — **build it once**.

### The halving cut — 2 sentences

"Prevent half that damage, rounded down" (Dark Sphere) and "rounded up" (Gisela,
Blade of Goldnight). Half of this is already built: `RoundMode` is the
vocabulary, and [CR#107.1a] is why the rounding word is obligatory. What is left
is a `PreventCut` row that **halves** rather than cutting a written count. The
two cards write the two rounding directions between them, so one row populates
both cells.

### The where-rider on a shift amount — 1 sentence

Hawkeye, Young Avenger's "it deals that much damage plus X, **where X is
Hawkeye's power**" — the only one of the scaling family's three variable shifts
that `Amount` cannot already spell, and blocked on `WhereLetterStatic` reaching
into an operation's amount rather than on anything of the scaling family's. Under
the round-minting bar on its own; `PreventCut` ledgers three more where-rider
positions, so take them together.

Re-measured and it does **not** fall into the trigger event's announcement:
Hawkeye's line is an `Intercepts` body and its "that much damage" is
`DamageScale`'s own spelling, so the announcement reaches neither half of it.

### Measured zero to keep

The scaling row's amount readback is **0 lines** — nothing writes a literal base
amount, so the operation slot absorbs it, which is the cut's trade at a third
site. Do not re-buy a readback here.

## Qualifying the prevented damage's source, and deciding the prevention trigger

The [CR#615.5] consequence rider landed as a field on the two prevention rows,
with the prevented amount announced in the field's own typing and read at the
named and anaphoric surfaces — **35 of its 36 sentences are readbacks**. What is
left of the area is three cells that all turn on the damage's *source*: which
prevented damage feeds the rider, a trigger the rule states separately, and the
source word itself.

### The source-qualified container — 6 sentences

The half of the rider's container clause that carries meaning rather than
spelling: "If damage from a **red source** is prevented this way" (Honorable
Passage), "from a creature source" and "from a noncreature source" (Comeuppance's
two), "from a black source" (Shadowbane), "from a creature" (Judgment of
Alexander), "from a black or red source" (Samite Ministration). It restricts
**which** prevented damage feeds the rider, which no spelling supplies.

Four of the six also want the headless "source" description. The source head
predicate is landed, with its placelessness gate a projection ([CR#120.7] exempts
a source from being in any zone, so every verb demanding one refuses it) — probe
the composition before scoping rather than assuming it composes.

### The [CR#615.13] prevention trigger — 4 sentences

A different construction from the rider beside it, and the rule says so outright:
some triggered abilities trigger when damage that would be dealt is prevented,
and such an ability triggers each time a prevention effect is applied to one or
more simultaneous damage events and prevents some or all of that damage. A
trigger uses the stack where the rider does not.

"When damage is prevented this way" (New Way Forward, Phyrexian Vindicator),
"Whenever …" (Judgment of Alexander, Samite Ministration). No `GameEvent` row
watches a prevention, and four lines is **under the bar for minting one** —
decide it deliberately. Phyrexian Vindicator is the cheapest whole card if it is
ever taken, and its "any other target" is a second small ask.

### The source read — 13 sentences, and the area's lowest-value cell

Recorded so this stays visible rather than being rediscovered. "That source" and
"the source's controller" (Deflecting Palm, New Way Forward, Honorable Passage,
Comeuppance, Archfiend of Spite, Belltower Sphinx, Phyrexian Obliterator, Crag
Saurian, Reaper of Sheoldred, Rona, Elesh Norn, Flameblade Angel, Bitter Feud)
want a source word in `NounWord`, which is a dozen tables wide — `wordReaches`,
`verbedWordOk`, `kindOfW`, `zoneOfThat`, `tyOfThat` and the rest, most of which
would answer the narrow way the copy word's do. The corpus wants the word as a
description **head** in 268 of its 281 occurrences and as a read in the other 13.

**Every one of the 13 is benchable today** through the generic pronoun with the
demonstrative re-sort elided, and Deflecting Palm is benched that way, so this
buys a spelling and not a sentence. Take it only when a card needs the re-sort to
disambiguate two object mentions — none of the 13 does.

## Measuring the "can't" statics, then channelling life-gain suppression

"Players can't gain life" (10 lines), "Your opponents can't gain life" (9), and
three singular-subject lines: 22 supported lines and no channel at all. Chapter
fifty-three declined it as a witness rather than mint a suppression subsystem to
carry one. The work is the measurement first and the row second — the family may
be one row over a suppressed EVENT rather than several.

- The 22 lines above are the whole life-gain mass; nothing in the grammar writes
  them today.
- It is NOT the outcome gate's shape — that gate is [CR#101.2] precedence over an
  outcome — and it is not `Prevents`, which is damage only.
- Measure whether it generalises with the other "can't" statics before
  scheduling a row: "Players can't draw cards", "Players can't search libraries".
  If one suppressed-event row covers them, the life-gain lines ride it.

## Consumption boundary

`idris/src/Experimental.idr` (`Redirects`, `RedirectsFrom`, `Intercepts`,
`CantPrevent`, the damage event's mention and its portion/magnitude reads;
`Outcome`, `ThatMuch`, `eventAfter`, `eventIntro`, `eventSubjectPlur`, `Amount`;
`GameEvent`, `IsDealtDamage`, `DealsCombatDamage`, `DamageAgent`, `DamageScale`;
`Shield`, `PreventCut`, `WhereLetterStatic`; `Prevents`, `PreventsFrom`,
`PreventedThisWay`, `IsSource`; the static rows and the outcome gate),
`idris/src/Experimental/Events.idr` (`eventUse` and its `TriggeredOnly` cells,
the shield's event side, the event table's prevention cell if one is minted, the
life-gain event the suppression would range over),
`idris/src/Experimental/Words.idr` (`NounWord`, the outcome sorts, `LifeGained`,
`LifeLost`, the damage-kind adjective, `RoundMode`, `wordReaches`,
`verbedWordOk`, `kindOfW`, `zoneOfThat`, `tyOfThat`), pins in
`idris/src/Experimental/ProofsF.idr` and `idris/src/Experimental/ProofsD.idr`,
evidence bench `idris/src/Experimental/Cards.idr`.

## Acceptance

- The excess subject reads a portion of a described damage event, not a fresh
  damage amount.
- The immunity elaborates as one conjoined negation over both refusals, and
  Arrow Storm still elaborates as the plain can't-be-prevented.
- All 16 "instead" bodies are re-checked against the magnitude readback once it
  exists.
- The mention is an event with an amount projection, not a second magnitude sort.
- The portion read stays refused unless the round mints its own `Amount` row for
  it deliberately.
- Every row minted announces through the existing mechanism; no second
  announcement path and no new outcome sort unless the round states why.
- The two recorded prices (the double read, the body-internal announcement) are
  still recorded as prices at the end of the round.
- One complement value serves both the shield's determiner and the cut; not two
  spellings.
- The halving row writes both rounding directions and Dark Sphere and Gisela
  bench against it.
- The container's restriction is a value the rider reads, not a second spelling
  of the rider.
- The prevention trigger is either minted with its four lines recorded, or
  declined with the count recorded — not left implied by the rider.
- If the source word is taken, it is taken for a card that needs the re-sort, and
  the 13 benchable-today sentences are not counted as its payoff.
- The cross-family measurement over the "can't" statics is recorded before any
  constructor is written.
- If a suppressed-event row is minted, all 22 life-gain lines land under it and
  the other "can't" statics are stated as in or out.
- The outcome gate and `Prevents` keep their scopes; neither absorbs suppression.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
