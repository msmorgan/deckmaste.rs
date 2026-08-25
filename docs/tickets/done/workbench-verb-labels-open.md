---
needs: []
---
# Keyword-action verbs are labels over expanded bodies

**Ruling (user, 2026-08-25), reversing `workbench-effect-basis-realign`'s
verdict:** the workbench mirrors semantics_v2's structure, and v1's
keyword-action shape is the model — `Composite { name, body }` where the
MACRO expands to the keyword action's full body and the label only names
which keyword action is performed ("discard is just a named move from
hand to graveyard"). The meaning lives in the expansion; the label is
data; the vocabulary is open. Authority:
`docs/memory/rulings/workbench-mirrors-semantics-v2-structure.md`
(gitignored local ruling; quote it in the round's As-landed).

## The change

- `VerbName` stops being a closed 8-member enum with meaning-bearing
  total tables. It becomes an open label (the crate's shape: an atom
  gated by membership, not by a per-member type). Pick the Idris shape
  that keeps typo-safety at the gate without a core enum row per verb.
- The carrier is `Enact : (label) -> Effect bs -> Effect bs` (the
  de-enum'd `Composite`; `Does subj label body` stays for the agentive
  surface). **The label sits on the innermost action, not the whole
  expansion** — [CR#701.9] defines the discard AS the move; choosing is
  the instruction's business. Sketch:
  `discardN n = Repeated n (Sequentially [Choose a card from your hand,
  Enact "Discard" (Move that Hand->GY)])`. Consequences the round must
  keep: n labeled moves in one atomic batch = n occurrences
  [CR#603.2c]; the stamp lands on the move's object so the verbed
  anaphors key on exactly the acted-on mention; would-discard
  replacements and "at random" attach at the labeled move.
- `TagBody`'s per-verb typed expansions move to the MACRO layer: each
  keyword-action macro (`discards`, `sacrifices`, `mills`, `scries`, …)
  builds the full body and applies the label. A well-formedness guard on
  `Composite`/`Does` may survive if it can be stated label-generically;
  a per-label table in core may not.
- The four `VerbName`-keyed total tables (`verbAgentive`, `verbMoves`,
  `Eq VerbName`, `verbedMarkingOk`) are re-homed: whatever each row
  encoded either derives from the expanded body, moves to the macro, or
  is recorded data keyed by the label.
- `TheVerbed`/`ThoseVerbed` (the verbed anaphors) keep working, keyed on
  the label via the binding `Stamp` that `Enact`/`Does` applies to the
  bindings its body introduces.
- `Repeated n body`'s outgoing context is the body's delta PLURALIZED —
  each mention the body itself introduces becomes one `ManyOf` summary
  binding (same payload, same stamp; identity on deictics), plus a
  quantity outcome carrying `n` — so "the discarded cards"/"that many"
  read the batch, and the `ChooseQ` batch form yields the same summary.
- Every bench witness spelling is preserved (the macros' names and call
  shapes stay; their definitions change).
- The `Put` finding survives: `Put` is not a [CR#701] keyword action
  ([CR#701.1]); under labels this is unremarkable — a label needs no
  rules entry, its body speaks.
- Update the two docstrings the realign round wrote (`VerbName`,
  `Effect`) to the new verdict; the `Keyword` sibling statement stands
  (it was already the declaration candidate).

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Experimental.idr`, `Macros.idr`,
`Proofs*.idr`, `Cards.idr`. No Rust crate.

## Acceptance

- No closed verb enum remains; a new keyword action is demonstrably a
  new macro + label (show one added in a test/witness without touching
  any total table).
- The ruling is recorded where `VerbName`'s replacement is defined.
- No witness lost, no pin silently passing; `idris/scripts/build` PASS.

Standard constraints apply.

## Expansion-shape ruling (2026-08-25, CR-verified, amended)

- The expansion's shape is not rules content: a resolving instruction is
  atomic (nothing interleaves mid-`Sequentially`), so the engine sees
  `Repeated n [choose, move]` and a batch `ChooseQ (Exactly n) … + move`
  as the same batch. **Canonical form, pinned (user, 2026-08-25): the
  iterated singular** — `Repeated n (Sequentially [Choose one, Move
  that])` — which handles a shortfall structurally (each iteration's
  choose finds what it finds). `ChooseQ`-style batch forms are for cards
  whose printed English carries the cardinality itself ("choose two
  colors"). No per-macro discretion.
- Per-event granularity is the ENGINE's rule, keyed by the label: draws
  are individual events [CR#121.2]; a multi-card discard is one event
  with n occurrences [CR#603.2c] ("discard a card" fires n times; "one
  or more" once). The label is what licenses the engine to apply the
  right granularity — another job the label does; the term shape does
  not encode it.
- Shortfall ("discard 3 with 2 in hand") is the engine's [CR#609.3]
  do-as-much-as-possible (discard is that rule's own example); either
  expansion is honest as written.
- No printed card writes "one at a time" for discard (corpus, 2026-08-25).

## As-landed

The ruling this round implements, quoted from
`docs/memory/rulings/workbench-mirrors-semantics-v2-structure.md`:

> v1's `Action::Composite { name, body }` is right: a macro expands to the
> keyword action's FULL body plus a label naming which keyword action is
> performed ("discard is just a named move from hand to graveyard"). The
> meaning lives in the expansion, the label is data, and the vocabulary is
> open — a new keyword action is a new macro + label, never a core enum
> row + total-table re-decide.

**Label shape.** `VerbLabel = String`, gated at every site that writes one
by `KnownVerb v = So (knownVerb v)`, where `knownVerb` reads `verbFacts` —
a plain `List VerbFacts` of `(label, participle)` rows. Idris reduces
primitive string equality on literals, so `Enact "Discard" …` typechecks and
`Enact "Descry" …` fails with `So False`: typo-safety at the gate, no core
enum row and no coverage-checked clause per verb. `Stamp` carries the label
string, so `Eq VerbLabel` is `Eq String` and the hand-written `Eq VerbName`
is gone.

**Carrier.** `Composite v e` → `Enact v e`; `Does subj v e` keeps its shape
as the agentive surface. Both apply the label to ONE action and impose no
relation between label and body — the body is the meaning. The gates a
keyword action really imposes (`OnBattlefield` for destroy, `DiscardOk` for
discard, `SlicePossessor` for mill) already rode the macros and still do.

**Macro layers.** Two, per the round's refinement: the ATOM takes the
referent and is just the labeled action (`discard n = Enact "Discard"
(Move n graveyardZ noRiders)`; the existing agentive macros `discards`,
`sacrifice`, `mills`, `puts`, `playerScries`, `playerSurveils` are the same
atom with a subject); the counted form composes from it (`discardN amt =
Repeated amt (Sequentially [choose (a card in hand), discard that])`). Every
bench witness spelling is unchanged — only macro definitions and raw
constructor spellings moved.

**`Repeated`.** New `Effect` row, `Repeated : (n : Amount bs) -> (body :
Effect (amtIntro n)) -> Effect bs`, the canonical iterated-singular
expansion. Its outgoing context is `effDelta body` (the new `effIntro` minus
the context it read) pluralized one summary `ManyOf` mention at a time —
same payload, same stamp, identity on the deictic self — plus an
`outcomeB RepeatCount`. `RepeatCount` is a new `OutcomeSort` row: the count
the TEXT wrote, distinct from the batch's own size (`GroupSize` over the
summary), because [CR#609.3] lets a shortfall leave them different. No
`ChooseQ` was minted: nothing in the bench prints its own cardinality yet.

**Proof of openness.** Tap ([CR#701.26a]) joined as `MkVerbFacts "Tap"
(Just "tapped")` plus `Macros.tap n = Enact "Tap" (SetStatus Tapped n)` — one data
row and one macro, no constructor and no total-table clause anywhere.
Witness: `masterDecoy` ("{W}, {T}: Tap target creature."), beside the
untouched `icyManipulator`, which writes the same body bare. `carefulStudy`
("Draw two cards, then discard two cards.") witnesses `Repeated` through
`discardN`.

Note the brief's suggested rule number for tap/untap was wrong: it is
[CR#701.26], not [CR#701.27] (that is Transform).

### Ledger

Retired tables (all `VerbName`-keyed, all closed-enum machinery):

| retired | why |
|---|---|
| `data VerbName` (8 arms) | replaced by `VerbLabel`/`verbFacts` |
| `Eq VerbName` | `Eq String` |
| `verbAgentive` / `NonAgentive` | the pinned sketch requires a subjectless `Enact "Discard"`, so agentivity cannot gate the carrier; which surface a keyword action writes is the macro's |
| `verbMoves` | derived: a stamp is written only where a labeled action moved its patient, so `stampMoves (Just _) = True` and a participle read never carries counter memory |
| `data TagBody` (11 arms) | per-label expansion table; the ruling forbids a per-label table in core, and every gate it carried already rides the macro |
| `verbedMarkingOk` (per-verb rows) | re-homed as data: `isJust (participleOf v)`, one `participle` field per label row |

Pins:

| pin | fate |
|---|---|
| `badDiscardBattlefield` | survives unchanged — `DiscardOk` on `Macros.discards` [CR#701.9a] |
| `badDiscardedCreatureWord` | survives, label re-spelled |
| `badVerbedWrongVerb` / `badVerbedWrongNoun` / `badVerbedAmbig` | survive, labels re-spelled |
| `badCompositeDestroyGraveyard` | re-keyed to the macro as `badDestroyGraveyardCard` — [CR#701.8a] moves a permanent off the battlefield and only a card there is one [CR#110.1] |
| `badExileTapped` | survives, demoted from `Dependent.Unspellable` to plain — only the rider gate remains [CR#110.5,110.5b] |
| `badDistributedMillSingular`, `badAfterReflexiveReadsTrigger` | survive; the `{tb = MillB …}` scaffolding dropped |
| `badDestroyTaggedExile` | RETIRED — tag/body agreement is unstatable over an open vocabulary; a mislabeled body means what its body says |
| `badAgentlessSacrifice`, `badAgentlessScry`, `badSubjectlessPut` | RETIRED — enforced `verbAgentive` only |
| `badDoesDiscardBattlefield` | RETIRED — was `badDiscardBattlefield` through the tag relation |
| `badScryBottom`, `badScryReveals`, `badBottomMill` | RETIRED — the per-label body table is gone and the gate cannot be restated label-generically |
| `badUnknownVerbLabel` | NEW — a label outside the catalog; a SPELLING pin, since [CR#701.1] leaves an unkeyworded verb its standard English meaning |

### Tolerated overgeneration

`Enact`/`Does` are public and the bench writes them raw, so every refusal the
retired pins carried is now spellable — not merely unstated. Verified by
type-checking each retired term against the landed tree; each is accepted:

| now spellable | the rule it overgenerates against |
|---|---|
| `Enact "Sacrifice" (Move …)` — no actor | [CR#701.21a] has the permanent's CONTROLLER move it |
| `Enact "Scry" …` — no one scrying | [CR#701.22a] states the looker and the library's owner as one player |
| `Does You "Scry" (lookAt bottomCard)` | [CR#701.22a] fixes the slice at the top N |
| `Does You "Scry" (revealCards …)` | [CR#701.22a] says "look at"; a reveal shows every player [CR#701.20a] |
| `Does You "Mill" (Move (LibrarySlice OnBottom …) …)` | [CR#701.17a] mills from the TOP of the library |
| `Does You "Discard" (Move (a creature) …)` — not from hand | [CR#701.9a] moves the card from its owner's HAND |
| `Enact "Destroy" (Move … exileZ)` — label disagrees with body | [CR#701.8b] makes "destroyed" the word's own event; a mislabel is a spelling defect |
| `Enact "Put" (Move …)` — subjectless placement | not a [CR#701] keyword action; the loss is a second spelling of a bare `Move`, not a rules refusal |

Restating any of these needs a label-generic gate the carrier cannot carry, or
the macro layer becoming the only way to write a labeled action.

`playerScries`/`playerSurveils` state only the LOOK. Under the old shape the
label carried the rest; now the body is the meaning, so both under-state their
rule: [CR#701.22a] and [CR#701.25a] each continue "then put any number of them
… and the rest on top of your library in any order". The workbench has no term
for that split, so the expansions are knowingly partial.

Gates: `idris/scripts/build` 19/19 from clean; `Cards.idr` 0 `{x = …}` binds;
0 `{default` in the Experimental tree; `cite check --list-noncompliant` empty;
`cite check` 0 stale after blessing [CR#609.3]; `cite audit --diff` sites read.

### Attested-text over-refusals closed (reviewer findings 3, 1-residual)

1. **A counted keyword action is payable.** `costActionOk (Repeated _ body)`
   now goes through `costRepeatedOk`, which looks THROUGH the body's own
   `Sequentially` and judges each step. The coordination refusal targets a
   cost written as "do A, then B", whose components [CR#601.2h] may be paid
   in any order; "do A n times" is one instruction repeated, and the
   canonical iterated-singular expansion writes its choice and its act as
   two steps of that one action. Witness: `zombieInfestation`
   ("Discard two cards: Create a 2/2 black Zombie creature token.").

2. **A labeled status change stamps its patient.** `effIntro` gained
   `SetStatus` arms for `Enact`/`Does` through the new `stampIntro`
   (`moveIntro` with the zone left where it was), and `mkStamp`'s docstring
   now states the rule generally: a labeled action stamps the mention it
   acted on, whatever it did to it. Body shapes past move and status get a
   row when a printed line needs one. `"Tap"` accordingly carries the
   participle `"tapped"`. Witness: `harmonyOfNature` ("Tap any number of
   untapped creatures you control. You gain 4 life for each creature tapped
   this way."), reading `thoseVerbedThisWay "Tap"` exactly as `martyrsCry`
   reads an exile's.

   Burn at the Stake itself does not bench: its "As an additional cost to
   cast this spell" frame has no construction here, and "three times the
   number of creatures tapped this way" wants an amount reading the SIZE of
   a participle group, which the vocabulary lacks (`GroupSize` reads the
   unique plural mention, not a named one). Harmony of Nature is the same
   19-line family and exercises the same stamp.

Pin probe after both fixes: `badDestroyGraveyardCard`,
`badUnknownVerbLabel`, `badExileTapped` each perturbed individually — all
three rejected the `impossible` clause (3/3). Build 19/19 from clean; cite
gates clean, `cite audit --diff` 45 sites read.
