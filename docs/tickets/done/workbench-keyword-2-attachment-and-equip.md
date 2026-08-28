# keyword-2: the attachment relation and the equip axes

Sub-round 2 of [workbench-keyword-parameters-and-attachment](workbench-keyword-parameters-and-attachment.md)
(the umbrella — authoritative). May run parallel with sub-round 1; stay off
the keyword catalog tables it owns (`Keyword` rows, `keywordFacts`,
`KeywordParamShape` cells, the class term). Owns: **the ATTACH EFFECT
constructor** (none exists; 65 supported faces wait; `ItToken` and
`ItOtherThan` are positioned to consume it — the biggest single win on the
board); the REVERSE attachment phrase ("an Aura attached TO a creature" —
Kitsune Mystic whole); the attachment's OWN direction ("as long as this
Equipment is attached to a creature", 12 lines — a third direction, not a
re-spelling); the participle cells (the 18 prenominal occurrences — CHECK
whether the participle is the predicate in attributive position before
minting; the `Equipped PermanentW` cell flips ONLY together with the negated
type setting, Luxior benching on the pair); the Aura CARVE-OUT rider (15
lines, four spellings — its own construction, not a sign flip on the
extension; whole cards benched, the cheapest six-card win in the area); the
restricted equip COMPOUND (21 lines — minted only with the card that demands
it; [CR#702.6c] makes the quality a TARGET restriction that does not compose
with the attachment; "Equip planeswalker" covered by the same shape); the
equip COST-REDUCTION statics (9 + 7 lines — the cost-modification axis, not
an equip variant; Ghostfire Blade benches); the status event's SECOND READER
(the FaceDown duration endpoint admits Vesuvan Shapeshifter while the header
keeps its 0-header refusal — two tables, not a widened cell; `eventSpan`'s
StatusChange row answered in the same pass or explicitly left); and the
enchanted-conspiracy admittance note (a tightening rule belongs with the
attachment relation — decide or record).

Standard constraints apply.

## As landed (2026-08-28)

Gate: `idris/scripts/build` clean from scratch, **23/23, 0 errors, 0
warnings**. `cargo xtask cite check --list-noncompliant` empty; `cite
check` 0 stale after `cite bless` registered [CR#303.4j]; the whole diff
read through `cite audit --diff`, 50 sites.

### THE ATTACH EFFECT CONSTRUCTOR — landed

`AttachTo` and `Unattach` on `Effect` (`Effect.idr`), nine tables each.
Re-measured 2026-08-28 against `jq 'select(.supported)'`:

- **233 supported faces** write an attach clause outside reminder text
  (297 further lines write it inside reminder text — every one the equip
  or reconfigure expansion). The umbrella's 65 was low by a wide margin.
- **0 of the 233 elide the host**, so the host is an argument, not a
  slot; it is kind-indexed on [CR#701.3a]'s "object or player" and both
  seats are written.
- The verb family's real shape is **attach / unattach and nothing else**.
  "Move" is not in it: every supported "move … onto" line moves a
  COUNTER, 0 move an attachment.
- `Unattach` is **18 imperative occurrences over 18 faces** — 9 as an
  effect, 9 as an activation cost, which `costActionOk` admits on
  [CR#118.1]. The 4 "becomes unattached from a permanent" lines are the
  EVENT [CR#701.3d] names and were kept out.
- `Unattach` took **one slot**: the "from [host]" phrase 4 of the 9
  effect lines write narrows WHICH attachment, which is the object's own
  `AttachedTo` description.
- **No `verbFacts` label bought.** [CR#701.3a]'s act has no expansion
  below it here, and the corpus reads an attachment back by description
  at every site and by a stamp at none. Recorded at the row.

`Macros.attachToIt` spends the co-argument narrowing on the family it was
built for — **66 of the 233 clauses write the host as a bare pronoun**,
40 of them "attach this Equipment to it" — and the exclusion is the act's
own rule ([CR#301.5c] "can't equip itself", [CR#303.4d] "can't enchant
itself").

Benched: **Ancestral Blade** whole (the `ItToken` witness — the host
pronoun names the token the same clause minted), **Cloud, Ex-SOLDIER**'s
entry trigger (the `ItOtherThan` witness — the bare `It` counts the
target Equipment too and refuses), **Embercleave** whole, **Disarm**
whole, **Kitsune Mystic** whole, **Akiri, Fearless Voyager**'s two lines.

### The REVERSE phrase and the attachment's OWN direction — one row

`AttachedTo` on `Predicate` (`Phrase.idr`), the relation's third
direction, named from the attachment's side with the host described
([CR#303.4b], [CR#301.5a] each state all three directions in one
sentence). **127 supported faces** — 104 attributive ("all Equipment
attached to that creature"), 23 after a copula ("as long as this
Equipment is attached to a creature"). One row for both, on finding
275's test. **No `AttachWord` slot**: "enchanted to", "equipped to" and
"fortified to" are 0 anywhere.

The two ticket items are the SAME relation in two positions, and that is
the finding — not two constructions. Benched: **Kitsune Mystic** whole
(attributive, and the card's last blocker), **Summoning Materia**'s
second line (predicative), plus the phrase's third job carrying
`Unattach`'s "from".

### The participle's prenominal cells — NOTHING MINTED

Checked before minting, as the ticket demanded, and the answer is that
the participle prenominally is `IsAttached` in attributive position.
Re-measured: 43 faces after a copula, 12 in a relative clause ("creatures
you control that are enchanted"), **29 occurrences over 28 faces
prenominally** ("equipped creatures you control"). The three writings
take the same word with the same absent slots; nothing covaries with the
position, so the position is spelling. **Finding 369's "never
prenominally" record is corrected in place** — at `IsAttached`'s new
docstring and in the closure tables. Benched whole: **Stone Haven
Outfitter** (both positions on one card).

`attachHeadOk Equipped PermanentW` needed no flip — **it was already
True and cited to Luxior**; the closure tables' §7 record of it as a live
False discrepancy was stale and is corrected. What Luxior's line actually
wanted was the other half, which landed: **`LosesType`**, the negated
type setting, **28 supported lines** (21 the devotion cycle). Benched:
**`luxiorTypeSetting`**, a fragment — see remainders.

### The Aura CARVE-OUT — landed, six whole cards

`DoesntRemove` on `StaticEffect`, its own construction pointing the
opposite way from `AlsoForKeywords`. **15 supported lines in four
spellings** confirmed exactly. What "remove" names is the state-based
action [CR#702.16c] and [CR#702.16d] describe ([CR#704.5m],
[CR#704.5n]). It rides the STATEMENT, not the line, because Guardian
Beast's rider trails a condition over three coordinated statements where
the twelve protection lines trail one. Only gate: no nesting.

Benched WHOLE: **Black Ward, Cho-Manno's Blessing, Tattoo Ward, Pentarch
Ward, Benevolent Blessing, Floating Shield**.

### The equip COST-REDUCTION statics — already built; Ghostfire Blade benched

The umbrella's premise was stale: `CostsToCast` already takes an
ability-kind subject (`AbilityCostSubject`) and `AbilityHead
(KeywordClass "Equip")` already exists, with Bureau Headmaster and
Fervent Champion already on the bench. Re-measured: **10** supported
lines write "equip abilities you activate … cost {N} less to activate"
(6 with a target restrictor), not 9; the "appended sentence" family is
**5** equip carriers, not 7, and it is a slice of the general
activated-ability reduction (**46 faces**), no part of the equip row.
**Ghostfire Blade benches whole** on machinery that was already there.

### The status event's SECOND READER — split, two tables

The umbrella's premise was again stale in one direction: `statusEventOk
FaceDown` was already `True` and the pin was gone, i.e. the cell had been
**widened**, which is what the acceptance forbids. Repaired as a split:

- `statusEventOk` keeps the TRANSITION question ([CR#710.4] closes
  `Unflipped`), which is `StatusEvent`'s own gate and what the duration
  endpoint reads.
- `statusHeaderOk` (new) is the trigger header's own table, gated onto
  `Triggered`, `AltEvent` and `JoinedHeader` through `HeaderStatus`.
  FaceDown False — **0 headers against 132 "turned face up"
  occurrences**, pinned by `badTurnedFaceDownHeader`.
- Vesuvan Shapeshifter's endpoint — the corpus's **single** "turned face
  down" — writes: `Cards.vesuvanShapeshifterCopySpan`.
- **`spanEventOk`'s `StatusChange` row is answered**, not left: attested
  at that one line, with the comment recording it.

### The enchanted-conspiracy admittance — DECIDED, the catch-all stands

No tightening rule is warranted. [CR#702.5a] puts the restriction on the
ENCHANT ability ("the enchant ability restricts what an Aura spell can
target and what an Aura can enchant"), [CR#303.4] says the same in the
Aura's own rule, and [CR#109.1] makes a card in any zone an object —
which is why "enchanted creature card in a graveyard" is ordinary
printed text. A refusal here would need a rule about the PARTICIPLE and
the CR states none; a refusal by count is not written. Recorded at
`attachHeadOk` and in the closure tables.

## Remainders

- **The restricted equip COMPOUND is NOT minted — deliberate lane
  deviation.** Re-measured: **22 supported lines over 19 distinct
  shapes** (the umbrella's 21/20 was close but off), "Equip planeswalker
  {1}" among them. Minting it means a new `KeywordParamShape`
  constructor, a `paramShapeOf` cell and a changed `keywordFacts "Equip"`
  row — all inside the parallel sub-round 1's declared table ownership,
  which this round was fenced off. [CR#702.6c] stands recorded: the
  quality is a TARGET restriction that does not compose with the
  attachment ("additional restrictions for an equip ability don't
  restrict what the Equipment may be attached to"), so the shape is a
  pair on the keyword row and nothing else. **Route to whichever round
  next owns `keywordFacts`.** It holds Luxior, Steelclaw Lance,
  Commander's Plate and 19 other lines off the bench.
- **`luxiorTypeSetting` is a fragment** for that reason alone: its other
  three lines write.
- **"Becomes attached" / "becomes unattached" has no event row.** 11
  supported faces write "becomes attached to" as a trigger header and 4
  write "becomes unattached from a permanent" ([CR#701.3d]'s last
  sentence defines the latter). Neither is a `GameEvent`; `AttachChoice`
  covers only the as-clause. Cheap now that the relation exists.
- **Akiri, Fearless Voyager is a fragment by one pronoun.** "…tap that
  creature and it gains indestructible until end of turn": the "it"
  wants `ItPrior` over the tap clause's own delta, which is empty because
  that clause's subject is itself a readback. An anaphora-family residue,
  untouched by anything here.
- **Flickering Ward does not bench**: "Return this Aura to its owner's
  hand" wants a destination zone possessed by `OwnerOf <the moved
  object>`, which `DestOk` refuses. A zone-expression residue.
- **"In addition to its other types" has no retain-all arm.**
  `SetsType`'s `ret` names ONE retained type; [CR#205.1b]'s phrase
  retains all. `BecomesAlso` covers the addition surface, which is what
  Luxior's second half uses — recorded because the 268-face family sits
  across both rows and no round has asked the question.
- **Conqueror's Flail** wants a temporal window on a prohibition ("your
  opponents can't cast spells during your turn"); `Macros.deontic` has no
  window slot.
