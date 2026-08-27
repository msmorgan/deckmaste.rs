---
needs: []
---
# The verb-label mechanism's residues: the partition body, the raw carrier, and the labels waiting to join

`docs/tickets/done/workbench-verb-labels-open.md` turned keyword-action verbs
into labels over expanded bodies and opened the vocabulary. Four things it named
and left; they share the `Enact`/`Does` carrier and the two macro layers, so
they are one claimable unit.

## 1. Scry and surveil have no term for the two-pile partition

Quoted from that round's ledger:

> `playerScries`/`playerSurveils` state only the LOOK. Under the old shape the
> label carried the rest; now the body is the meaning, so both under-state their
> rule: [CR#701.22a] and [CR#701.25a] each continue "then put any number of them
> … and the rest on top of your library in any order". The workbench has no term
> for that split, so the expansions are knowingly partial.

The construction is a partition of a looked-at slice into two destinations with
a "any number / the rest" split and an order clause on the remainder. It is the
one thing standing between the scry and surveil labels and honest expansions.

## 2. Raw `Enact`/`Does` overgenerate every refusal the retired pins carried

Same round, "Tolerated overgeneration": `Enact` and `Does` are public and the
bench writes them raw, so eight terms that were refused before are now
type-correct — a subjectless sacrifice against [CR#701.21a], a scry with no one
scrying and a scry off the bottom against [CR#701.22a], a mill from the bottom
against [CR#701.17a], a discard not from hand against [CR#701.9a], a destroy
labelled over an exile body against [CR#701.8b], and a subjectless `Enact "Put"`.
Its own closing sentence is the decision this ticket owes:

> Restating any of these needs a label-generic gate the carrier cannot carry, or
> the macro layer becoming the only way to write a labeled action.

Decide it. "The macro layer is the only way" is a real answer with a real cost
(the bench stops writing raw constructors); so is "these stay overgenerated,
refused at the spelling boundary". Do not split the difference per verb.

## 3. The labels the open vocabulary now makes cheap

Each is one data row plus one macro under the landed mechanism, and each was
named as a blocker by a round that could not afford a core enum arm:

- **proliferate** — `docs/tickets/done/workbench-counter-family-residues.md`
  (Tromell, blocked whole).
- **manifest dread** — same ledger (Curator Beastie, blocked whole).
- **a search verb**, which `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`
  named as consolidated open gap 8: deleting `shuffledAway` admits more than
  [CR#701.24b] licenses, and the fix wanted a search row in the then-closed
  enum. Re-read that gap against the open vocabulary before designing anything.

Take them as the mechanism's proof under load, not as a card-benching exercise —
each carrier has other blockers.

## 4. Two smaller carries

- `Repeated` minted no `ChooseQ`, "nothing in the bench prints its own
  cardinality yet". Mint it against the first witness that does, not before.
- `mkStamp` has arms for move and status only: "Body shapes past move and status
  get a row when a printed line needs one." Same rule — a row per printed
  witness.

## Consumption boundary

`idris/src/Experimental.idr` (`Enact`, `Does`, `Repeated`, `mkStamp`/`stampIntro`,
the partition's new row), `idris/src/Experimental/Words.idr` (`verbFacts`,
`knownVerb`, `VerbFacts`), `idris/src/Experimental/Macros.idr` (the atom and
counted layers, `playerScries`/`playerSurveils`), the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Scry and surveil expand to their full rules body, or the shortfall is named at
  a rule rather than at "no term exists".
- §2 has one written verdict applied uniformly, with the eight terms re-probed
  against it.
- Each label joined in §3 costs one data row plus one macro and nothing else;
  if one costs more, that is the finding.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

### 1. The partition composes; no new row

The two-destination split needed no `Effect` row. `SomeOf` marks the pile
it takes as `PartD` and `TheRest` reads the complement off that mark
([CR#608.2d] has the player announce the choice while applying the
effect), which is the shape `impulse` and `anticipate` already write. So
the scry and surveil bodies are the same three-clause chain the bench
already had a witness for:

    Does You "Scry" (Sequentially
      [ lookAt (topSlice amt)
      , move (SomeOf anyNumber Them) (onBottomIn AnyOrder)
      , move TheRest (onTopIn AnyOrder) ])

and surveil's, with the chosen pile going to `graveyardZ` and no order
clause on it — [CR#701.25a] writes one only on the remainder.
`playerScries`/`playerSurveils` are the same body with `They` for the
looker, slice possessor and both destinations; `Sequentially`'s telescope
makes `Does`' label ride the whole expansion, which is what [CR#701.22a]
and [CR#701.25a] each define as the keyword action.

**Cost.** No constructor, no total-table arm. What it cost instead is
hypothesis plumbing: over a generic `bs` the anaphor gates do not reduce,
so each macro lifts them to auto-implicits stated over two named context
functions (`lookedTop`, `lookedRest`) — four for the plural macros, two
for the singular. That is the `discardN` idiom, and the call sites
discharge them by search.

**The one-card spelling is separate.** A one-card slice binds `OneOf`, so
"them" and its complement have no plural antecedent: at N=1 the split is
written as the offer ("you may put that card on the bottom of your
library"), which is the printed reminder text's own paraphrase of
[CR#701.22a]. Hence `scryOne`/`surveilOne` beside `scry`/`surveil`. The
offer names the card by its word (`That CardW`) rather than as "it", which
is what lets a scry stand in a clause that already named an object — a
cast spell is on the stack and no card word reaches it. Without that,
Gandalf, White Rider ("Whenever you cast a spell, … scry 1") loses its
scry to an ambiguous "it"; the fuller expansion is what exposed it.

Bench: 6 scry 1, 2 scry 2, 1 scry 3, 1 surveil 1, all expanding in full.

### 2. The verdict, and what it cost

**Ruling (user, 2026-08-26).** *The macro layer is the only SANCTIONED way
to reach `Enact`/`Does`.* The carrier would be private if Idris could hide
one constructor of a `public export` type; it cannot, so this is authoring
policy, recorded, not a compiler gate. The rationale: keyword actions
stack on top of the core rules without disturbing them; the label exists
because the game rules must OBSERVE that a specific keyword action took
place (triggers and replacements watch it), but every keyword action is a
composite of pre-existing building blocks — label = observability hook,
macro = sanctioned constructor, body = meaning.

Applied:

- **Policy recorded** on `Enact` in `Experimental/Effect.idr`, with
  `Does` pointing at it. It states the reason it is not a gate, and that
  pins write the carrier raw on purpose: a pin's business is the term the
  bench must not have.
- **Bench migrated.** 11 raw carrier sites in `Cards.idr` → 0. Ten went to
  `scry`/`scryOne`/`surveil`/`surveilOne`; the eleventh (Thought Lash's
  "that player exiles all cards from their library") to a newly minted
  `exiles`, the agentive surface of `exile`. Two pins that wrote a raw
  `Does … "Mill"` over a body `mills` already builds went through the
  macro too (`badDistributedMillSingular`,
  `badAfterReflexiveReadsTrigger`), leaving four raw carrier sites in the
  tree, all pins whose subject IS the carrier: `badUnknownVerbLabel` (the
  catalog gate) and `badExileTapped` (a rider gate `exile` cannot carry).
- Macros minted: `exiles`, `scry`, `scryOne`, `surveil`, `surveilOne`,
  plus the shared helpers `topSlice`, `lookedTop`, `lookedRest`,
  `theyLookAtTop`.

**The eight terms, re-probed.** Each was written against the landed tree
and typechecked. All eight are still accepted — as expected, since nothing
in the verdict is a gate:

| term | disposition |
|---|---|
| `Enact "Sacrifice" (Move …)` — no actor | type-correct, unsanctioned |
| `Enact "Scry" …` — no one scrying | type-correct, unsanctioned |
| `Does You "Scry" (lookAt bottomCard)` | type-correct, unsanctioned |
| `Does You "Scry" (revealCards …)` | type-correct, unsanctioned |
| `Does You "Mill" (Move (LibrarySlice OnBottom …) …)` | type-correct, unsanctioned |
| `Does You "Discard" (Move (a creature) …)` | type-correct, unsanctioned |
| `Enact "Destroy" (Move … exileZ)` | type-correct, unsanctioned |
| `Enact "Put" (Move …)` — subjectless | type-correct, unsanctioned |

None is pinnable and none is restated. Each body is rules-meaningful on
its own — a subjectless sacrifice is a move to a graveyard, a scry over
the bottom card is a look — and what is wrong with each is the LABEL it
carries, which the settled doctrine refuses at the spelling boundary, not
at semantics. Pins refuse only CR-meaningless terms; a mislabel is a
spelling defect. The policy is what keeps them out of the bench, and the
bench is now clean of them.

### 3. The three labels: one lands, two are the finding

**Search — landed, and it cost more than a row plus a macro.** Re-read
against the open vocabulary, consolidated open gap 8 does not want a
`VerbName` arm any more, but it does not reduce to a row plus a macro
either, because search's body is already a core row (`Search`) and what
the gap wanted was a STAMP, not a spelling. Landed as: one `verbFacts`
row (`MkVerbFacts "Search" Nothing` — no printed line names a search's
patient by participle, and "the searched card" is not what English would
spell); `effIntro (Search …)` stamping the found mention with that label;
`survivesShuffle`/`afterShuffle` in `Words.idr`; and
`effIntro (Shuffle whose) = afterShuffle (nomIntro whose)`. So the gate
[CR#701.24b] states is now stated: the cards a search found survive the
shuffle, and every other library mention does not — the pile is
randomized where no player knows its order [CR#701.24a], and a revealed
card becomes a new object outright [CR#701.20d]. Witness standing:
`mysticalTutor`, which reads its found card after the shuffle. New pin:
`badReadsShuffledLibraryCard` (a bare look, a shuffle, then a read).
**Four wiring changes plus the row — the finding is that the label was the
cheap half and the stamp was not.**

**Proliferate — not landed; the label is cheap and the body is not.**
[CR#701.34a] (not [CR#701.27a], which is Transform) reads "choose any
number of permanents and/or players that have a counter, then give each
one additional counter of each kind that permanent or player already
has." The chooser half is spellable TODAY, verified by typechecking:

    CountedGroup anyNumber (Joined (And [Permanent, HasCounters Nothing])
                                   (CounterCompare Nothing AtLeast (Lit 1)))

The second half is not. No counter row says "one more of each kind it
already has": `PutCounters` names a kind, `PutSameCounters` copies a
source's counts to a destination, and `PutCountersOfThoseKinds` /
`GetsCountersOfThoseKinds` read an ANNOUNCED batch's kinds and presuppose
one (`countOutcomes CountersPut bs = 1`), which proliferate has not got.
So proliferate costs a `verbFacts` row, a macro, AND a new kind-blind
self-reading counter row at kind `Object \/ Player` with its total-table
cascade. Recorded, not forced.

**Manifest dread — not landed; blocked one level down, not on the label.**
[CR#701.62a] (the [CR#701.35] the brief guessed is Detain) reads "Look at
the top two cards of your library. Manifest one of them, then put the
cards you looked at that were not manifested this way into your
graveyard." Its STRUCTURE composes from what §1 landed — a look, a
one-of-them partition, and `TheRest` to the graveyard — so the negated
participle read the phrasing suggests is not a gap. The single blocker is
manifest itself [CR#701.40a]: turn the card face down, "it becomes a 2/2
face-down creature card with no text, no name, no subtypes, and no mana
cost", then put it onto the battlefield face down. `TokenRider` has
`EntersTapped` and `EntersAttacking` and no face-down arm, and nothing in
the vocabulary LISTS the characteristics a face-down permanent then has,
which is where [CR#708.2] says they come from. That is a
permanent-characteristics gap, not a label-mechanism one.

### 4. Neither carry earned its mint

- **`ChooseQ`** — still nothing in the bench prints its own cardinality
  for a labeled action. The two counted choices that do bench
  (`CountedGroup anyNumber creature`, `CountedGroup (upTo 1) creature`)
  go through `Choose` over a counted mention and want no batch
  constructor. Unminted, same rule as last round.
- **A third `mkStamp` body shape** — the labeled bodies this round added
  are `Sequentially` chains under scry and surveil, and both labels carry
  no participle ([CR#701.22a] and [CR#701.25a] leave no patient a later
  clause names that way), so nothing wants a stamp past move and status.
  Unminted, same rule.

### Ledger

| item | needs |
|---|---|
| proliferate's counter row | "one additional counter of each kind [n] already has", per member, at `Object \/ Player`; unblocks Tromell |
| manifest [CR#701.40a] | face-down battlefield entry with set characteristics [CR#708.2]; unblocks Curator Beastie via manifest dread |
| `afterShuffle` is owner-blind | it drops every unstamped library mention whatever library was shuffled; [CR#701.24b] scopes to the shuffled one. The payload records a zone, not an owner |
| `ChooseQ`, third `mkStamp` shape | a printed witness, per the standing rule |

Gates: `idris/scripts/build` 23/23 from clean; `cite check
--list-noncompliant` empty; `cite check` 0 stale; `cite audit --diff` 13
sites read, one claim re-scoped on the reading ([CR#701.20d] covers
REVEALED cards, so the general case moved to [CR#701.24a]).
