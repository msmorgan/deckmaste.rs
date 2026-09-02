# workbench-choice-tail

The choice umbrella's remainders with no natural bundle, minted at its close
(all five sub-rounds done 2026-08-27; details in
done/workbench-choice-{a,b,c,d,e}-*.md As-landed sections):

- **The joint-typing container for cross-line choices** (consolidated open gap
  15) — would move the five [CR#608.2c] pins together.
- **The `TokenChars` chosen-quality payload pair** — Mutavault's "with all
  creature types" cell (3 lines) and the token spec's chosen-quality cell (2
  lines) are the same payload question, priced in choice-D's As-landed: a 6th
  positional field breaks every `MkToken` site; the `AndAlso` workaround moves
  a printed with-clause out of its bundle. Decide the shape, then both fall.
- **Master Biomancer's entry-time type ascription** — `EntersRider` carries a
  `TokenRider` only; the entry-time ascription row is 1 measured line, plus
  its coordination with the counter clause.

- **Routed from workbench-coordination-2-noun-and-phrase (close, 2026-08-27):**
  the AMOUNT-PER-MENU-ARM — Inspirit / Flagship Vessel write menu arms
  carrying different counter amounts; a coordination of counter clauses on
  the menu shape, which is choice machinery.

- **Routed from workbench-keyword-2 (close, 2026-08-28):** the RETAIN-ALL arm
  on `SetsType` — "is a [type] in addition to its other types" as a
  type-SETTING that keeps everything (268-face family, never asked of the
  row); Luxior's `LosesType` landed the loss side. Ascription payload, so it
  lands here.

## As landed (2026-09-02)

Six files, +409 / -14, three commits. Gate: `idris/scripts/build` from a
deleted `build/` — **23/23, 0 errors, 0 warnings**. Cites: 0
non-compliant, 20,164 checked / 0 stale, `cite bless` registered nothing
new, and the whole feature diff read through `cite audit --diff`.

### THE `TokenChars` PAYLOAD DECISION — landed, and choice-D's price was wrong

`TokenChars` gains a sixth cell `quals : List (TokenQuality bs)`:

    data TokenQuality : Bindings -> Type where
      WithEveryType : (space : TypeSpace) -> TokenQuality bs
      WithQuality : (q : Predicate bs Object) ->
                    {auto 0 qr : QualityRead q} -> TokenQuality bs

ONE cell for both routed families, because both are post-modifiers of the
same noun phrase and each is the bundle-internal form of a statement that
already has a row outside a bundle — `AddsEveryType`'s quantifier and
`AddsChosenQuality`'s linked read. Gated by `TokenQualsFit`
(`tokenQualHosted`), which asks [CR#205.3d]'s host of the BUNDLE's own
type words where `HostedRead` asks it of a subject noun, and rides
`TokenWritten`, `SetsType`, `BecomesAlso` and `ExceptChars`.

**Choice-D's price does not exist.** The constructor is now
`MkTokenChars` (six cells) and `MkToken` is a five-argument wrapping
macro over it, so all 74 literal construction sites are untouched — the
house idiom `creatureTok`/`create`/`becomes` already use, and the one
the "positional params for translation" rule points at. The `AndAlso`
workaround was never needed and was not taken.

**Benched (3 whole cards, each first attempt):** `mutavault` (the
subtype quantifier inside a `SetsType` bundle), `soulstoneSanctuary`
(the same cell with a keyword riding the bundle beside the quantifier,
and the setting standing with no duration) and `volrathsLaboratory`
(BOTH reads inside one `create` bundle, on `volrathsLaboratoryChoice`'s
compound chooser).

**Re-measured, against choice-D's numbers:**

- The becomes-a-creature-WITH cell is **4 supported lines, not 3**:
  Mutavault, Faceless Haven and Soulstone Sanctuary (the last two write
  "with vigilance and all creature types", which the round's grep missed)
  plus Mutable Explorer's reminder-text token.
- The token spec's chosen-quality cell is **3 supported lines over 2
  cards, not 2**: Volrath's Laboratory writes it twice ({3} 1/1 and {5}
  2/2), Riptide Replicator once.
- **Riptide Replicator still does not bench**, and the payload is no
  longer why: its X stands in a body whose cost writes none. Faceless
  Haven writes the identical shape and wants `{S}`; Soulstone Sanctuary
  landed.

**Recorded overgeneration:** an empty `colors` beside a colour read is
the READ's colour and not [CR#105.2c]'s colorless, and a literal colour
written beside such a read is admitted though unprinted. No rule refuses
an object that is both white and the chosen colour, so there is no gate
to write — only the record, which sits on `TokenChars`.

### THE RETAIN-ALL ARM — NOT BUILT, and [CR#205.1b] is why, not a count

The routing's premise inverted on the rule text. [CR#205.1b] says an
effect that "specif[ies] that the object retains a prior card type"
retains **all** of them, and lists "in addition to its other types" and
"still a [type…]" as the SAME rule's two phrases. So:

- `SetsType`'s `ret` is the printed retention PHRASE, not a one-type
  carve-out — the row already means retain-all whenever `ret` is written.
  Its docstring said nothing at all before; it now states this.
- The 268-face family the routing named re-measures at **289 supported
  lines over 285 cards**, and every one is the ADDITION operation
  `BecomesAlso` already writes over this same bundle (Luxior, Giada's
  Gift writes a setting and an addition in one sentence,
  `luxiorTypeSetting`). Nothing there wants a `SetsType` arm.
- A third family fell out of the rule and is also already covered: the
  unmarked "becomes a [subtype] artifact creature" form — **232
  supported lines** — retains all its prior types by [CR#205.1b]'s last
  two sentences, and that retention is a function of the type line the
  row already carries, so recording it would put a derived fact in a
  printed cell.

**Recorded residue (2 lines):** "It's still a Shapeshifter" and "It's
still a Cave land" print a SUBTYPE where `ret : Maybe CardType` can only
spell a card type. 135 supported lines print a retention marker (118
"It's still a land", 12 "They're still lands", one each of enchantment,
artifact, Shapeshifter, Cave land). What those two want is a wider
retention payload, not a retain-all arm.

### THE JOINT-TYPING CONTAINER — DECLINED, shape and price recorded on `AbilitySeq`

And the five pins are **re-grounded, not moved**: [CR#608.2c] was the
wrong rule for all five and pointed the other way.

- [CR#608.2c] orders the instructions of ONE spell or ability. All five
  pins put a reading ABILITY above a choosing ABILITY on a card, which
  that rule does not govern; its own last sentence tells the reader that
  later text on the card may modify earlier text and to read the whole
  text — the opposite of the refusal it was cited for.
- What links a chooser to a reader is [CR#607.2d], and it links them by
  ROLE — the ability that causes the choice, the ability that refers to
  it — naming no printed order.
- So the refusals are the MODEL's: `AbilitySeq` threads a face's
  abilities left to right. `badReaderBeforeChooser`,
  `badChosenProtectionBeforeChoice`, `badAscribedQualityBeforeChoice`,
  `badNameMatchBeforeChooser` and `badLastChosenBeforeChooser` now say
  STRUCTURAL and cite [CR#607.2d] where they cited [CR#608.2c].

**The container's shape, written down:** it cannot be `AbilitySeq` typed
at the union of its own members' introductions — that index would be
computed from the sequence the index types. The face must DECLARE its
choices: `CardFace` gains `chosen : List QualitySort`, `text` is indexed
at the bindings those sorts introduce ahead of `costLetters cost`, and a
`So` gate holds the declaration to what the abilities choose. Only the
CHOICE bindings may be lifted — lifting targets or pronoun antecedents
would let one ability read another's target, which no rule licenses.
**Price:** `CardFace`, `AltFace`, `FaceLaws`, `AltFaceLaws`,
`Macros.card` and every card in the bench. **Witnesses: 0** — measured
2026-09-02, no supported card prints a chosen-quality read above its
chooser. Unbought.

`badUnlessAnaphoricPayer`, the sixth pin the pins-round said would move
with these, was not touched: it is the "unless" node's own and outside
the choice machinery.

### THE AMOUNT-PER-MENU-ARM — RECORDED, not built (1 line)

Re-measured 2026-09-02: **38 supported lines** write "your choice of"
over counter kinds, 26 distinct, and exactly **one** gives an arm a
number of its own — Inspirit, Flagship Vessel, "put your choice of a
+1/+1 counter or two charge counters on up to one other target
artifact". (Grimdancer's "two different counters" is the PICK's size,
`DistinctChosenKinds`, already written.)

The shape wanted is a menu of counter QUANTITIES — arms of (amount,
kind) with the clause's amount slot absent because the menu subsumed it
— and it cannot be a `CounterKindSource` arm: that type sits in
`PutCounters`' kind slot beside an `Amount` the row always writes, so an
arm with its own number would state the count twice. `Modal` is not the
spelling either: two modes mint a target apiece [CR#601.2c] where the
card writes one mention for both arms. So it is a row of its own plus
nine tables at one line. Recorded on `ChosenKind`. The card is blocked
well past this anyway: it writes Station and two threshold-framed
ability groups.

### Zeros and non-findings

- **0** supported cards print a chosen-quality read above its chooser.
- **0** supported lines want a `SetsType` retain-all arm.
- **0** supported addition or copy-except lines write a bundle quality,
  so `TokenQualsFit` rides `BecomesAlso` and `ExceptChars` for soundness
  and not for a carrier.

### Remainders

- **Master Biomancer's entry-time type ascription** (this ticket's third
  bullet) was NOT taken — it is a new `EntersRider` row at 1 line, not a
  payload cell, and this round's budget went to the payload decision and
  the two rule corrections. Still open.
- The **subtype retention payload** (2 lines, above) is new and open.
- The **joint-typing container** stays open with its shape and price now
  recorded; re-opening it needs a printed witness, and there is none.
- The **amount-per-menu-arm** stays open at 1 line.
