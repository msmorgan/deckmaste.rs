---
needs: []
---
# Close out the copy family: the card verb, the entry row and the stack residues

Everything the copy constructions still owe, in one unit. The stack copy
([CR#707.10]) landed and left targeting and mention residues behind; the
copy-a-CARD verb ([CR#707.12]) is a different rule and a different verb but
shares the stack copy's mention word; and the copy-on-entry row ([CR#707.5])
carries the exception kinds that ride on `CopyExcept` alongside the stack copy's
own `ExceptColor`. They are one claimable unit because they share the mention
noun, the `CopyExcept` rider family, and the debt bookkeeping between the two
copy rows (the stack copy was checked for `ExceptPt`'s payer and found none, so
the entry row owes it).

## What the stack-copy round landed — do not re-buy

The stack copy landed the verb (`CopyStack`, 294 sentences), the mention
(`Origin.CopyOrigin` + `NounWord.CopyW`, 261 reads), the retarget verb
(`ChooseNewTargets`, 202 sentences), `CopyExcept.ExceptColor` and the
`Subtype.Bird` row (findings 693–701), with Twincast, Fork, Meletis Charlatan,
Tawnos the Toymaker whole and Echo Mage's level-4 ability. Everything below is
what it measured and did not take; the two targeting constructions are the
substance and the rest ride along.

## The stack copy's residues

- **The COULD-TARGET multiplication** [CR#707.10d], the largest residue, and TWO
  constructions rather than one: an AMOUNT counting the players or objects a
  named spell "could target" (19 supported for-each copy counts — "copy that
  spell for each other Golem that spell could target"), and a DISTRIBUTION
  sentence handing the copies out one target apiece (7 — "Each copy targets a
  different one of those Golems"). The second reads the landed copy mention
  distributively ("each copy" is 8 of the 261 mentions), so half the machinery
  stands. Precursor Golem, Radiate, Ink-Treader Nephilim, Mirrorwing Dragon,
  Zada Hedron Grinder, Agrus Kos and Radiant Performer are the whole family;
  Radiate is cheapest — its first sentence is an ordinary choice and it has no
  other lines. The rule supplies the failure case a gate would want: "if that
  player or object isn't a legal target for each instance of the word 'target',
  a copy isn't created for that player or object".
- **The copy's SPECIFIED target** [CR#707.10e], 6 supported sentences: "The copy
  targets Ivy", "The copy targets that token", "The copy targets the chosen
  creature". A statement whose SUBJECT is the copy mention and whose predicate
  is a targeting relation nothing in the grammar states — `AnyTarget` and the
  target quantities describe a phrase's own targeting, never one object
  targeting another after the fact. Beamsplitter Mage, Exterminator Magmarch,
  Feather Radiant Arbiter, Frontline Heroism, Ivy Gleeful Spellthief;
  [CR#707.10e]'s own worked example is Frontline Heroism. It pays for itself
  only beside the could-target amount, which writes the same relation in the
  plural.
- **The small recorded residues**: "the copy they control" (2, a possessive on
  the mention — Curse of Echoes, Tempt with Mayhem); "copy that spell an
  additional time" (2, an increment on a count some other clause set — Howl of
  the Horde, Tempt with Mayhem); the CONDITIONAL exception (1, Double Major's
  "except it isn't legendary if the spell is legendary"); the starting-loyalty
  readback (1, Ob Nixilis the Adversary — finding 639's unmodeled box a second
  time).
- **A measured ZERO to keep**: the participial read is unwritten — "the copied
  spell" and "copied this way" are zero lines, and copying mints no `VerbName`
  stamp. Do not let a widening make it spellable silently.
- **The once-each-turn rider**, and it is not a copy question. Iron Man Bleeding
  Edge and Donal Herald of Wings are both blocked on nothing else: both end "Do
  this only once each turn", which [CR#603.2h] defines as a restriction on the
  ACTION and which `Triggered`'s `limit` slot explicitly is not. That rider is
  32 lines by the slot's own comment and is the cheapest way to buy those two
  cards.

## The copy-a-CARD verb and its cast permission

[CR#707.12], 22 copy verbs over 20 cards plus 45 "you may cast the copy"
readbacks over 43 — the single largest thing the stack-copy round measured and
did not touch. A different rule and a different verb from [CR#707.10]'s stack
copy: the copy is created IN THE ZONE THE CARD IS IN and then cast while another
spell is resolving.

- The shape: "Exile the top card of your library. You may copy that card and
  cast the copy without paying its mana cost."
- It SHARES the stack copy's mention word ("the copy", "the copies") and nothing
  else, so the noun is already there. What is missing is the verb, the
  cast-the-copy permission, and the without-paying rider — which `MayPlay`'s
  neighbourhood may already carry. **Measure that before designing.**
- Carriers: Arcane Proxy, Elite Arcanist, Isochron Scepter, Wizard's Spellbook,
  Mnemonic Deluge, Spelltwine, Reversal of Fortune.
- [CR#707.12a] handles the plural's per-object choice ("may cast" applies
  individually to each copy).
- [CR#707.13] (Garth One-Eye) and [CR#707.14] (Magar of the Magic Strings) are
  one-card rules riding at the edge of this family. Garth's line additionally
  wants a chooser narrowed to a printed LIST of six card names, which is a
  chooser-domain question and not this verb's.

## The copy-on-entry row

Chapter 92 fenced "enters as a copy" on a PROBE rather than on a missing carrier
and refused four exception kinds beside it. Both halves are the copy clause's
payload and riders, and the entry row owes a landed row its first witness.

- 64 supported sentences over 63 cards. The composition `Intercepts (Enters …)`
  over a `Continuously (BecomesCopy …)` compiles today (probed, verified,
  removed) and it spells the sentence [CR#707.5] exists to distinguish: an
  object that enters "as a copy" "becomes a copy AS it enters. It doesn't enter
  the battlefield, and then become a copy". The corpus writes "enter as a copy"
  64 times and the would-enter-instead form ZERO times.
- So the ask is a copy-on-entry row, and the rule hands it two riders the
  composition could not carry either: [CR#707.5]'s "enters with" / "as … enters"
  abilities of the copied text take effect, and [CR#707.6] gives the copy's
  controller the as-enters choices fresh.
- **This row owes `ExceptPt` its witness.** That row landed with finding 481's
  bar declared unmet, its 10 bare-P/T lines all multi-clause or behind this
  fence. Cheapest payer is Quicksilver Gargantuan, [CR#707.9d]'s own worked
  example; Clone, Sculpting Steel and Sakashima's Student are the bare, the
  artifact and the type-adding witnesses. The stack copy was checked for a payer
  and found none (Donal, Herald of Wings writes its number inside a
  characteristics bundle), so the debt is this row's.

## The exception kinds

- **The NAME exception**, 16 sentences ("except his name is Absorbing Man",
  "except its name is Mishra"). It wants a name SETTING inside a copy clause — a
  rider on `CopyExcept` — and shares nothing with the name MATCH but the word.
  This is pinned rather than merely absent: `badAscribedName` refuses a name in
  the ascription rows' payload, because a name is MATCHED and never ascribed
  ("are/becomes/is the chosen name" is zero lines apiece), and
  `chosenQualityReadOk CardName = False` still holds with `badYourChoiceCardName`
  and `badChosenCardNameRead` behind it. Measure it with the other copy
  exception kinds, NOT with the chosen name.
- **The CHARACTERISTICS BUNDLE**, 36 sentences writing a number inside a whole
  bundle ("except it's a 4/4 black Zombie", the Scarab God family). The missing
  cell is NOT the colour — `ExceptColor` landed (one bare sentence, Fork,
  [CR#707.10]'s own worked example) and the bundle is still refused, because the
  bundle is ONE NOUN PHRASE setting three characteristics at once and not three
  exceptions joined: `[ExceptPt, ExceptColor, ExceptTypes]` would spell "except
  it's 4/4, it's black and it's a Zombie in addition to its other types", which
  is not the sentence. It wants a bundle-shaped exception carrying a
  `TokenChars`-like value; measure that against `SetsType`'s payload, the same
  phrase in the statement position. **Re-measure the split before design**: of
  the 27 tails matching a bare "except it's a N/M …", 16 end "in addition to its
  other types" and 11 do not, so the bundle is not uniformly a setting.
- **The four small ones**, recorded rather than queued: the enters-with
  exception (5, [CR#707.9e]'s "additional effect rather than a modification"),
  the type SETTING exception (4, "and it loses all other card types"), starting
  loyalty (1) and RETAIN (1, [CR#707.9c], Vesuvan Doppelganger).

## Consumption boundary

`idris/src/Experimental.idr` (`CopyStack`, `CopyExcept`, `AnyTarget` and the
target quantities, `Triggered`'s `limit`, the copy-a-card verb, the cast
permission and the without-paying rider, `MayPlay`, `BecomesCopy`, `ExceptPt`,
`ExceptColor`, `SetsType`/`TokenChars`, the ascription gates including
`badAscribedName`), `idris/src/Experimental/Words.idr` (`CopyW`, `CopyOrigin`,
the possessive and distributive reads, reuse of the existing copy mention),
`idris/src/Experimental/Events.idr` (the distribution sentence, the entry
replacement and its riders), evidence bench
`idris/src/Experimental/Cards.idr`.

## Acceptance

- Radiate benches; the 19 could-target counts, the 7 distribution sentences and
  the 6 specified-target sentences write.
- Iron Man Bleeding Edge and Donal Herald of Wings bench on the once-each-turn
  rider, which sits on the action and not on `limit`.
- The participial-read zero still refuses, pinned.
- The 22 copy verbs and the 45 cast-the-copy readbacks write; the zone the copy
  is created in is the card's, not the stack.
- `MayPlay`'s neighbourhood is measured before a second without-paying rider is
  minted; the verdict is recorded either way.
- The stack copy's verb and the copy-a-card verb stay distinct rows.
- The 64 entry sentences write through one copy-on-entry row that carries both
  rule-given riders; the would-enter-instead form stays at zero.
- Quicksilver Gargantuan benches, paying `ExceptPt`'s outstanding witness debt.
- The bundle exception is one payload, not a list of three; the 16/11 tail split
  is measured and the verdict recorded before the row is shaped.
- `badAscribedName` and the chosen-name refusals still hold after the name
  exception lands.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
