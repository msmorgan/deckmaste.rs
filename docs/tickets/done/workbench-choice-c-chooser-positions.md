# choice-C: the chooser positions and the container

Sub-round C of [workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md)
(the umbrella). Owns its section "The chooser positions the as-enters rider
does not reach" in full, plus these routed items: the board-read counter-kind
chooser (~13 lines, Bribe Taker, Crystalline Giant) with Grimdancer's plural
distinct pick; finding 1030's chooser-repeatability record on `ChoiceStands`;
and `CountedGroup`'s missing `ChoiceMode` slot (51 call sites — script the
mechanical bulk).

Pins:
- The attach timing is a DISTINCT entry-replacement row, never a relaxed
  `ZoneFits` battlefield gate. Sanctuary Blade benches whole.
- The non-entry choosers want the CONTAINER at their positions; the
  effect-level `Choose` clause already exists and is not re-minted.
- The object chooser's mention is an OBJECT, not a Quality;
  `countQuality`/`qualityB` are not bent to reach it.
- The compound chooser is SPELLING (two choices in one sentence) — a
  coordination question, no discourse machinery.
- The each-player distributive stays refused as a different reading.
- `badTwoChoosersOneSortRead` holds: no append, no re-read of `countQuality`.
- Chromatic Armor's `{X}` symbol and `sleight` counter kind stay deliberately
  unbought; record, don't build.

Acceptance: the umbrella's chooser lines. `idris/scripts/build` PASS.
Standard constraints apply.

Routed from choice-B (close, 2026-08-27): once the attach chooser lands, the
marked-read generalization (`OfLastChosenColor` → `OfLastChosen q`) is a
five-line change that buys Psychic Paper — take it with the attach position.

## As landed

Eight files, +389 / −31. `idris/scripts/build` PASS (23/23 from a clean
`build/`, 0 errors, 0 warnings). Cites: 0 non-compliant, 18,239 checked / 0
stale, 26 audited sites read against their rule text, 1 rule newly blessed
([CR#301.5b]). No design artifact: the docstrings plus this section are the
record.

### Premise corrections, up front

- **`CountedGroup`'s missing `ChoiceMode` slot is STALE.** The slot is
  already there — `CountedGroup : (q : Quantity bs) -> (mode : Maybe
  (ChoiceMode bs)) -> (p : Predicate bs k) -> …`, `ChoiceMode` at
  `Events.idr` with `Unmarked | TheirChoice | AtRandom | YourChoice` — and
  all 51 call sites already pass it. Nothing to script; the routed item is
  closed by inspection.
- **Dinosaur Headdress is a FACE, not a card.** It is the back of
  Paleontologist's Pick-Axe (craft), supported. So the attach chooser is 3
  supported lines over 3 faces, and the object chooser 4 carriers over 3
  cards.
- **The non-entry chooser is 10 cards, not 5**, and over more positions than
  the umbrella names. Measured (a chooser outside an as-enters rider whose
  read sits in a DIFFERENT ability): combat trigger 2 (Beckoning
  Will-o'-Wisp, Triarch Stalker), activated 2 (Koh, Chromatic Armor), upkeep
  or fused trigger 2 (Shapeshifter, Mystic Barrier), enters trigger 1
  (Stalking Leonin), **Saga chapter 1** (Medomai's Prophecy, chapter II
  chooses and chapter III reads), **additional cost 3** (Caller of the Hunt,
  Celestial Reunion, Liquid Fire). The last two positions are new to the
  measurement; the additional-cost one is blocked on its own missing row
  (`AltCost` is the ALTERNATIVE cost — Force of Will's, Crash's — and no row
  writes "as an additional cost to cast this spell"), so it is routed rather
  than reached.
- **Beckoning Will-o'-Wisp and Triarch Stalker carry TWO further blockers
  each**, neither of them the chooser: the flavor word ("Lure the Unwary",
  "Targeting Relay") is not one of [CR#207.2c]'s ability words and no row
  spells one; and both spend the read inside "creatures attacking the last
  chosen player", an attributive attacking-DEFENDER phrase that `Attacking`
  carries no slot for. That phrase is **94 supported lines**, so it is its
  own cell and was deliberately not built here.

### The attach-triggered chooser — `AttachChoice`, and Sanctuary Blade whole

A distinct `StaticEffect` row beside `EntersChoice`, never a relaxed gate.
The ground is measured rather than argued: [CR#614.1c] names the ENTERING
event, [CR#301.5b] says Equipment "enter the battlefield like other
artifacts. They don't enter the battlefield attached to a creature", and
[CR#701.3a]'s attaching takes the permanent "from where it currently is",
so the two riders are both replacement effects under [CR#614.1]'s general
definition watching DIFFERENT events. `staticKind` is `Replacement`, not
`EntryRider`, for that reason. The printed host phrase ("to a creature") is
not carried — [CR#301.5] and [CR#301.6] fix the host outright and
[CR#303.4] hands an Aura's to its enchant ability — and no gate asks whether
the subject can attach, recorded as vocabulary-shaped with all three
carriers writing an Equipment ascription.

**Benched:** `sanctuaryBlade` (whole card, first attempt) and
`psychicPaperChoiceAndReads` (an `AbilitySeq` fragment). Dinosaur Headdress
does not land: its chooser is an object one narrowed by a CRAFT-exile
linkage ("an exiled creature card used to craft this Equipment") that no row
writes, and its read is the object-sorted marked read below.

### The marked read generalized — `OfLastChosen q`

The routed five-line change, taken with the attach position as instructed.
`OfLastChosenColor` becomes `OfLastChosen : (q : QualitySort) -> …`, gated by
`ChoiceStands` where `OfChosen` asks `= 1`, and by the same
`ChosenQualityRead` — one row for both, since the two spellings match a
chosen value against a characteristic identically and differ only in which of
the chooser's choices they name. `qualityReadHost` gains its `SubtypeQ` arm
so `HostedRead` applies at the marked read too. Psychic Paper's
name-and-type SETTING clause is written here as the two `SetsChosenQuality`
halves it is made of; the three-way coordination of the printed line ("has
ward {1}, it can't be blocked, and its name and creature type are …") is the
coordination cell's and is not taken.

### The non-entry chooser container — `effChoiceDelta` on `abIntro`

The CONTAINER, not a new clause, exactly as pinned. `Choose` already spells
the choice and already leaves the binding; what was missing was the export
across the ability boundary. `abIntro` now reads
`effChoiceDelta eff ++ bs` at `Activated`, `Triggered` and `Spell` alike —
uniform because [CR#607.2d] links two abilities printed on one object and
says nothing about their kinds, and a keyword ability has no body to look at.
`effChoiceDelta` recurses only through the composition rows a printed chooser
sits inside: a coordination (`Sequentially`) and an optional one (`May`, on
`mayIntro`'s standing precedent that a may-body's announcements are what the
clause leaves). A chooser under a CONDITION exports nothing — undergeneration,
not a refusal, and no supported line writes one.

Two supporting facts made this cheap rather than architectural:

- **`Choose` at a quality noun already mints the choice binding.**
  `bindFor det plur PhQuality p = MkBinding det (Quality q) plur QualityP`
  and `chosenDet PhQuality AD = AD`, which is `qualityB q` verbatim, which is
  `choiceB (QSort q)`. Nothing was re-minted.
- **The player kind needed one line to agree.** `chosenDelta` now goes
  through a new `chosenBind`, identical to `bindFor` but for `PhPlayer`,
  where it mints `ChosenPlayerP` instead of `PlayerP` — exactly
  `choiceB PlayerC`. Without it an effect-level player chooser and the
  as-enters one would have left different bindings and only one of them would
  have been countable.

A PLURAL choice exports nothing: `effChoiceDelta` matches `Choose` at
`Indefinite` only, so `CountedGroup`'s plural pick is invisible. That is
deliberate — `countChoice` counts singular bindings and the plural chooser's
reads are B's declined cell.

**Benched:** `shapeshifter` (whole card, first attempt — the second chooser
at an upkeep trigger, the marked read two abilities later, and "7 minus that
number" on the existing `Minus`), `beckoningWillOWispChooser` (the
combat-trigger chooser; Triarch Stalker writes the same ability), and
`kohChooser` (the activated chooser, at an object).
**Zeros, each with its own cause:** Mystic Barrier (the direction sort,
declined in B with a written verdict); Chromatic Armor's second chooser
(the `{X}` symbol and the `sleight` counter kind, both deliberately unbought
and still unbought); Koh's fourth line (no row grants ANOTHER object's whole
activated-and-triggered ability set); Stalking Leonin ("secretly" and
"Reveal the player you chose" as an activation cost); Medomai's Prophecy
(a chapter-scoped delayed trigger over the chosen name).

### The compound chooser — the coordination, as pinned

SPELLING, and it fell out of the same delta. `staticChoiceIntro` is now
`staticChoiceDelta se ++ bs`, and `staticChoiceDelta` reaches `AndAlso`'s
parts through `partsChoiceDelta`, later parts binding nearer on
[CR#608.2c]'s reading order. No discourse machinery and no new clause.

**Benched:** `volrathsLaboratoryChoice` and `callToArmsChoice` (the second
showing the coordination spans SORTS — a quality beside a player), plus
Psychic Paper's line above, which is the same coordination at the attach
position. **Zeros:** neither Volrath's Laboratory nor Riptide Replicator
lands whole, and the chooser is not why — both spend the reads on a TOKEN
SPEC ("a 2/2 creature token of the chosen color and type") and `TokenChars`
carries a literal colour list and a literal type line with nowhere for a
read to sit. That is the token bundle's chosen-quality cell, routed.

**No new pin.** The coordination path is refused by the gate that already
stands: `AndAlso [choose a color, choose a color]` makes
`countChoice (QSort Color)` two, so `OfChosen Color`'s `= 1` fails, and a
coordination twin of `badTwoChoosersOneSortRead` would be the near-duplicate
B declined at the player sort. `badTwoChoosersOneSortRead`,
`badReaderBeforeChooser`, `badChosenReadWrongSort`,
`badChosenProtectionBeforeChoice`, `badNameMatchBeforeChooser`,
`badNameMatchWrongSort` and every other pin still fail as they must (clean
23/23).

### The object chooser — built to the witness, the rest recorded

`choiceSortAt` is the row that says it: `Quality q` and `Player` bind a
VALUE, and every other kind — the `Object` one included — answers `Nothing`.
An object choice announces a MENTION, which the ordinary anaphora reads, and
no chosen value for a linked ability to name. `countQuality`/`qualityB` were
not bent, as pinned.

**Benched:** `forgottenLoreChoice` ("Target opponent chooses a card in your
graveyard", the opponent-chooser at an object — writable with no new
machinery; Shrouded Lore writes the same sentence) and `kohChooser` above.
**The object-sorted MARKED read is not built, and it is a measured zero of
payers**: all four carriers ("the last chosen card") carry a second blocker
— Koh's ability-set grant, Dinosaur Headdress's craft linkage, and both
Lores' repeat-with-exclusion rider ("repeat this process except that opponent
can't choose a card already chosen for Forgotten Lore"), which wants a
chosen-so-far memory. Building the read would have been dead structure; it is
recorded instead. The repeat-process rider is the Lores' own remainder.

### The plural read — measured against the container, three zeros

As instructed, measured rather than assumed. None of the three configurations
is reached by anything landed here.

1. **Two choosers from one distributive clause** (Null Chamber, "you and an
   opponent each choose a card name … the chosen names"). The each-player
   distributive stays refused as a different reading; the coordination export
   spells "A chooses X and B chooses X" and that is not the printed sentence.
2. **One chooser making a plural choice** (Seal of the Guildpact, Tablet of
   the Guilds). `EntersChoice` still takes no `Quantity` and
   `effChoiceDelta` deliberately declines to export a plural pick. B's
   measurement stands unchanged — the slot is cheap, the reads
   (an intersect-count of the object's colours with the chosen set) are the
   blocker — and the container adds nothing to that cost, because all four
   plural choosers in the corpus are as-enters ones ("choose two colors" ×2,
   "choose two players" ×2, "choose two basic land types" ×1).
3. **Three choosers in three clauses with a union read** (Paliano the High
   City, Regicide). The container WOULD export three bindings from a
   three-clause `Sequentially`, so the chooser side is no longer what blocks
   them; what does is the draft-time trigger ("Reveal this card as you draft
   it"), the card-name-scoped cross-card memory ("chosen as you drafted cards
   named Regicide"), and a read that is a union over all three choices where
   every landed read names one.

### The board-read counter-kind chooser — `CounterKindOn`, and `BoundKind`'s first witness

A `Predicate bs (Quality CounterKindQ)` naming the permanent the kinds are
read off. ONE row for both printed spellings — "a counter on [n]" and "a kind
of counter on [n]" — and [CR#122.1] is what merges them: counters "with the
same name or description are interchangeable" and a counter "is not an object
and has no characteristics", so pointing at a counter picks out nothing but
its kind. It is a DESCRIPTION and not a `ChoiceDomain`, because a domain is a
bindingless narrowing of a sort while this one names a permanent — and naming
it is what makes the next sentence's "each OTHER creature you control"
readable, the anchor being the creature the kind came off. Probed both ways:
with the unrestricted chooser the same sentence pair fails on `It`.

**Benched:** `contractualSafeguardPass`, which is `BoundKind`'s FIRST benched
carrier — B landed that arm with a measured zero of them. **The other 12 of
the 13 are unchanged and their blockers are re-stated in `BoundKind`'s
docstring**: 4 union recipients, 2 pronoun-ambiguous "on it", 1
static-embedded pass, 1 mixed printed/bound menu (Bribe Taker), 1 partitive
holder, and 3 that are the board-read chooser's own remainder — a trailing
"if it doesn't have a counter of that kind on it" (Aven Courier), a
distributive chooser under a Saga chapter (The Caves of Androzani), and an
at-random pick from a printed counter-kind menu with an exclusion rider
(Crystalline Giant, which additionally wants a `ChoiceDomain` at
`CounterKindQ` that no line but its own asks for). Contractual Safeguard's
own Addendum paragraph is a cast-timing rider and is why the bench is a
paragraph rather than a whole card.

**Grimdancer benches WHOLE** on a new `CounterKindSource` arm,
`DistinctChosenKinds`: the same menu picked more than once with no kind
picked twice. Its own arm beside `ChosenKind` because the pick is a different
SHAPE — a subset of the menu where that one takes a member — and because
[CR#122.1]'s interchangeability is exactly why the printed "different" has to
be said at all. Recorded overgeneration: the count rides the clause's own
amount slot where the menu cannot see it, so a pick wider than its menu is
[CR#608.2d]'s impossible option and is not gated, the amount not needing to
be literal.

### Finding 1030 — the record stands, ungated

`ChoiceStands`' docstring already carries it (12 occurrences over 12 cards,
every one behind a chooser that can fire more than once) and it already named
"an attach rider that re-fires on re-equip" among the shapes. Both positions
it anticipated now write, the covariance is unchanged, and NO provenance
machinery was added — as pinned. The 12 lines were re-counted this round and
are still 12.

### Deviations

- **`CounterKindOn` and `DistinctChosenKinds` are two new rows in the counter
  vocabulary**, which the umbrella's chooser section does not name. Both are
  paid for by a bench in the round (Contractual Safeguard's paragraph,
  Grimdancer whole) and both are inside the routed counter-kind chooser item.
- **The attacking-DEFENDER predicate was NOT built**, though it is the last
  blocker on two of the umbrella's five named non-entry cards. It is a
  94-line family with its own design questions (a defender may be a player, a
  planeswalker, a battle, or a joined "you or a planeswalker you control"),
  so landing it as a side effect of this round would have pre-empted a
  designed treatment. Routed with its count.

### Routed forward

- The attacking-DEFENDER attributive phrase, 94 supported lines; it is what
  Beckoning Will-o'-Wisp and Triarch Stalker wait on, together with a flavor
  word row.
- The additional-cost chooser position (3 cards) and the missing "as an
  additional cost to cast this spell" row.
- The token spec's chosen-quality cell (`TokenChars` carries literal colours
  and a literal type line): Volrath's Laboratory, Riptide Replicator.
- The object-sorted marked read, and the Lores' repeat-with-exclusion memory.
- A grant of another object's whole activated-and-triggered ability set
  (Koh).
- Crystalline Giant's `ChoiceDomain` at `CounterKindQ` plus its exclusion
  rider; Bribe Taker's menu mixing a printed kind and a bound one.
- Chromatic Armor's `{X}` symbol and `sleight` counter kind, still
  deliberately unbought.
