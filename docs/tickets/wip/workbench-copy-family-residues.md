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

## From the v1 comparison (2026-08-24)

[The v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md)
found no copy-family gap beyond what this ticket already carries. One blocker is
now named: **Magar of the Magic Strings**, listed above as a one-card rule at
this family's edge, is blocked on the crate's `Ident`-keyed note channel — its
text notes a name and reads it back at the copy ("create a copy of the card with
the noted name"). That channel has no v2 shape and needs a design ruling first:
[workbench-named-memory-channels](workbench-named-memory-channels.md).

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

- **Routed from workbench-coordination-2-noun-and-phrase (close, 2026-08-27):**
  Repeated Reverberation's two blockers — the family wants a discourse UNION
  OF ALTERNATIVES (not a `Joined` payload; the join was REFUSED with
  [CR#115.1,603.7b] — one arm's event causes the trigger, so "that spell"
  must be able to name nothing) and `Effect.CopyStack` is not kind-indexed
  (its `what` is gated `OnStack` at Object only). Copy machinery, so both
  land here.

- **Design obligation from the recency audit (2026-08-27):** when the
  ENTERS-AS-COPY family ("have this enter as a copy of X, except…") gets its
  constructor — 23 measured lines have none today — it MUST follow
  `TokenCopyOf`'s precedent and leave the copy source out of its delta
  (`specDelta = []`; the source is a constructor argument, never a discourse
  mention). Announcing the source would double every "except it" pronoun's
  antecedents and turn all 23 lines into live misbinding hazards. Audit
  record: scratchpad recency-audit/opus-analysis.md (session-local).

- **Routed from workbench-static-frame (close, 2026-09-02):** the
  copy-an-ability widenings — 15 targeted-copy lines need `CopyStack` opened
  past Object-kinded `OnStack` and `ChooseNewTargets`/`CopyW` likewise; the
  ability-noun vocabulary (`AnyTriggered` etc.) is ready and waiting. Same
  slot family as the Reverberation kind-index item already here.

## As landed (2026-09-02)

Full gate green from scratch: `idris/scripts/build` → 23/23, exit 0, no
warnings. Five commits, one per coherent piece.

### The copy verb opened past `Object` (routed from workbench-static-frame)

- `CopyStack` and `ChooseNewTargets` are kind-indexed under a new
  `Copiable`, `Counterable`'s twin at [CR#707.10]. ONE gate for both
  verbs: [CR#707.10c] states the retarget as a thing done to a copy, so
  what may be retargeted is what may be copied.
- `ControlledBy` is kind-indexed under [CR#109.4] — "target triggered
  ability you control" needed it, and 38 supported lines write
  "abilit[y|ies] you control".
- `AbilityP` carries an origin, and three noun words part on it:
  `AbilityW` (the plain ability demonstrative, `SpellW`'s row at that
  kind), `AbilityCopyW` (`CopyW`'s), and `CopyJoinW` (`AbilityJoinW`'s,
  which gained the not-a-copy guard in turn).
- **Re-measured**: 14 lines write "copy target [class] ability you
  control" and 15 write "copy that ability" — the ledger's "15 at copy"
  had merged the two halves.
- Benches: **Strionic Resonator** (whole), **Mister Fantastic**'s plural
  ("twice… the copies"), **Rowan's Talent**'s anaphor.
- Deviation recorded: `effEq (ChooseNewTargets _)` gives up on kind
  comparison, joining `CounterSpell` and `CopyStack`, which loosens
  `distinctModes` for two identical retarget modes. No printed line
  writes one.

### The copy-on-entry row

- `EntersAsCopy` at `EntryRider`'s seat, [CR#707.5]'s own sentence.
  **Re-measured at 60 supported lines over 60 cards, not 64/63**; the
  would-enter-instead form is still ZERO.
- The DESIGN OBLIGATION is honoured and written into the row: the copy
  source is a constructor argument and is NOT announced (`staticIntro`
  is `selfSubjIntro n` alone), on `TokenCopyOf`'s `specDelta = []` law.
  38 of the 60 lines write "except it" and every one means the entering
  permanent; the exceptions are elaborated in the SUBJECT's discourse,
  not the source's.
- The two rule-given riders ([CR#707.5]'s enters-with/as-enters
  abilities of the copied text, [CR#707.6]'s fresh as-enters choices)
  are the row's consequences and are recorded on it, not slots: no
  printed line states either.
- The `optional` flag is the printed "you may have" — 56 of 60 write it.
- Benches: **Clone**, **Quicksilver Gargantuan** (paying `ExceptPt`'s
  outstanding witness debt, [CR#707.9d]'s own worked example),
  **Sculpting Steel**, **Sakashima's Student**.

### The exception kinds

- `ExceptName` — 16 lines, always its own clause. The ticket's pin prose
  was STALE: `badAscribedName`, `badYourChoiceCardName` and
  `badChosenCardNameRead` no longer exist and `chosenQualityReadOk
  CardName` is now `True`. The live pin is `badNamedAddition` ("a name
  is set and never added"), and it still holds — this row sets a name at
  a different seat, which [CR#707.9d] is the rule for.
- `ExceptChars` — the bundle as ONE payload, `SetsType`'s `TokenChars`.
  **Re-measured: 35 tails, 23 setting and 12 adding — not the ticket's
  27/16/11.** The `typesAdded` flag is carried because [CR#707.9d] makes
  it a RULES difference (a setting stops the copied type-defining
  ability coming across; an "in addition to" addition does not), and
  `ExceptTypes` stays as the type-only addition that rule names outright.
  `copyBundleSays` requires two stated characteristics, so the partition
  against the singleton arms is exact and no sentence has two spellings.
- Benches: **Croaking Counterpart** (bundle), **Chameleon, Master of
  Disguise** (name).

### The discourse union of alternatives (routed from coordination-2)

- `sharedCtx` gains a third outcome: where a coordination's arms
  announce different things it hands on their UNION, falling back to the
  bare outer discourse where no union exists. This is NOT the meet the
  old rule refused — that one took a shared PREFIX, dropping mentions
  and shifting indices; the union drops nothing, stands at every
  position, keeps agreed fields and pairs disagreeing sorts into `JoinP`.
  A union with no reading word behind it is refused in `unionPayload`.
- The refused join is NOT re-litigated: the `Joined` head is one phrase
  over ONE event, and this card's arms are three events with three
  verbs, which is exactly why the answer had to be at the discourse.
- **Re-measured: 5 supported lines**, all of them copy lines.
- Bench: **Repeated Reverberation**, whole card. `AbilityJoinW` reads
  the union, `CopyJoinW` reads the copy of it.
- The whole workbench (all pins, all benched cards) builds unchanged
  with the union in, which is the evidence that it widens nothing else.

### The copy-a-CARD verb and the copy's specified target

- `CopyCard`, [CR#707.12]'s verb, gated `isCardZone` where the stack
  copy is gated `OnStack`; the copy is announced in the CARD's zone.
  `CopyW` drops its stack demand and asks the origin alone — the copy
  clause is the only thing that stamps `CopyOrigin`, and a token copy
  carries `TokenOrigin`.
- **`MayPlay`'s neighbourhood was measured first and the verdict is: no
  second without-paying rider.** `PlayPayment.WithoutPaying` already
  carries [CR#118.9]'s alternative cost at a permission.
- **Re-measured: 19 copy verbs (not 22) and 39 cast readbacks (not 45).**
- `CopyTargets`, [CR#707.10e]'s specified target — `Targets`' relation at
  the statement seat, 6 lines.
- Benches: **Flawless Forgery** (verb + permission), **Frontline
  Heroism** ([CR#707.10e]'s own worked example).

### Zeros held

- The participial read stays at zero and is now PINNED:
  `copyParticipleUnwritten` in `ProofsG` proves
  `verbedMarkingOk "Copy" Attributive = False`. "The copied spell" and
  "copied this way" are 0 lines apiece; `verbedWordOk` answers `False`
  for the copy words at all three kinds and the copy verbs mint no
  `VerbName` stamp.
- The would-enter-instead form: still 0.
- `badCopyPermanent` and `badRetargetPermanent` were re-pointed from
  `{zn}` to `{cp}` when the gate moved; both still refuse.

### Not built, with counts

Recorded in `Cards.idr`'s ledger under "THE COPY FAMILY'S REMAINDER":
the could-target amount (5) and distribution (7, blocked on NOUN
vocabulary — "each copy" and "a different one of those X"); the
once-each-turn rider (32, [CR#603.2h], trigger machinery held by the
sibling round's lane); "the copy they control" (2); "an additional time"
(4); Double Major's conditional exception (1); the starting-loyalty
readback (1); the type-setting exception (4); the retain exception (1);
"with no mana cost" inside the bundle (12 of 35 tails, `TokenChars` has
no mana slot); "enter tapped as a copy" (3). Also the ABILITY PRONOUN —
`It` is `Noun bs Object` and 5 of the 15 "copy that ability" lines write
"if it isn't a mana ability" — which is the noun vocabulary's row, not
this family's.
