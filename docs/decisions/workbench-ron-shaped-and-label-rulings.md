# The workbench is RON-shaped

**Every construction in the Idris workbench must be producible by the RON
re-emitter from a RON node, and every macro must name a RON macro. The
workbench checks RON re-emitted into it; it is not a second authoring
surface.**

Decided 2026-09-03, from the fresh workbench review of the same date. Records
four rulings: the RON-shaped constraint, the label vocabularies, the single
object kind, and clause order.

## 1. RON-shaped (the standing constraint)

A core constructor is admissible only if the RON re-emitter can produce it
from a RON node, and a macro is admissible only if it names a RON macro. This
belongs in every future workbench brief, alongside "standard constraints
apply".

Its authoring corollary: the bench carries **no implicit handles** — no
`{name = …}` braces anywhere in `idris/src/Experimental/Cards/*.idr` — enforced
by a one-line lint in `idris/scripts/build`. This extends
[Card authoring binds no implicits](card-authoring-binds-no-implicits.md),
whose `grep -cE` binds only the `Cards.idr` re-export today.
`Experimental/Proofs*.idr` are exempt: a pin's whole subject is the proof term
it refuses.

**The direction of travel is RON → Idris.** RON is authored and lowered to the
engine ([Core is explicit regions](core-explicit-regions.md)); Idris checks RON
that has been re-emitted into it. The review's proposals to give `Words.Binding`
a stable binder id, and to write a RON *emitter* in Idris over the checked term
(its §5.1, §5.9 and §3 risk table), are therefore moot — they solve a lowering
problem the project does not have. A workbench shape that RON cannot express is
not a lowering problem to be solved later; it is a shape that does not land.

## 2. Labels stay open, over tables derived from the stubs

Open string labels over facts tables stay. Closed enums per sort are
**declined**.

The rows are exactly the labels the Comprehensive Rules define — keyword
abilities [CR#702], keyword actions [CR#701] — plus the counter kinds the CR
names and the printed ones the corpus attests. A label outside the tables is
refused, and the refusal names the label and the table rather than failing as
a bare `So False`.

Rows are **derived from the RON stubs** (`plugins/builtin_v2/macros/stubs/…`,
fields `params`, `spelling`, `grammar`, `body`) wherever the stub determines
the column — `bodied`, `paidCost`, `regime`, and the admitted parameter shapes
as a **list** read off `params`. A column that exists only for an Idris gate is
hand-written. An `xtask` check verifies the label sets match in both
directions: every stub has a row, every row has a stub.

The parameter column is a list per row, not a single shape, so CR-defined
variants of a keyword ("hexproof from [quality]", "[type]cycling [cost]") are
admitted by the same row that admits the plain form. Custom (non-CR,
non-corpus) label sets are a later bridge, not part of this ruling.

Sweep: `workbench-facts-from-ron`.

## 3. One object kind

`Words.Kind.Ability` folds into `Object` with an `AbilityP` payload and zone
`Stack`. An ability on the stack is in the stack zone [CR#405.1] and is an
object [CR#109.1]; the grammar therefore has two entities, not three, and
"spell or ability" is one kind rather than a join of two.

The join-reading family shrinks to the object case:
`Phrase.NounWord.JoinW`/`AbilityJoinW`/`CopyJoinW`/`UnionHalf` and
`Phrase.StackActOn`. An ability's immovability and lack of printed
characteristics become payload facts, not kind facts.

This overrides the review's own recommendation (its R2) to keep the split and
record the divergence. Sweep: `workbench-ability-kind-fold`.

## 4. Clause order is not a slot

`Effect.StaticSpec.Conditionally` and `Effect.Continuously` thread
static-first, fixed. The printed orders that put the condition first are
spelled by the word-order macros (`throughout`, `onlyWhile`), which re-thread
on the way in.

`Effect.StaticThreads`, `Effect.SpanStaticThreads` and their `%hint` witnesses
go, and with them the `{ts = StaticFirstDone}` annotations the bench carries at
127 sites. `Effect.OnlyIf`/`Effect.If` are already the constructor mechanism
and stay.

Sweep: `workbench-clause-order-collapse`.

## Rationale

The workbench earns its keep as a soundness gate over the authored surface
([Idris is a soundness gate](idris-is-a-soundness-gate.md)), which means the
authored surface — RON — bounds it. A construction the re-emitter cannot
produce is checking a language nothing writes; an implicit handle in a bench
term is elaboration leaking into authoring, and it is also a term RON has no
way to spell.

The label rulings follow the same boundary. The stubs already hold the
vocabulary and its parameters, and a hand-maintained Idris copy of that
vocabulary is a second source that drifts. Deriving the rows makes the stub the
authority and the check the tripwire, while keeping the property the review
rated highest: CR-facing knowledge lives in data rather than in constructor
shapes, so a new keyword is a row.

The kind and clause-order rulings both remove a mechanism that exists only
inside Idris. A separate `Ability` kind and a proof-search-chosen thread
witness are each invisible in RON, so each would have to be reconstructed at
the boundary; the CR gives one object and the sentence gives one order, so
neither reconstruction is worth its cost.

## Consequences

Every workbench brief carries the RON-shaped constraint by name. A round that
wants a shape RON cannot express opens the RON shape first, or stops.

`grep -E '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards/*.idr` must stay
empty, and the build script enforces it. A new brace there is a missing macro.

Facts rows are generated, so a keyword's parameter shapes and its `bodied` /
`paidCost` / `regime` columns are changed in the stub, not in `Words.idr`.

## Tracked references

- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md)
- [Core is explicit regions](core-explicit-regions.md)
- [Builtin-v2 macro spelling and grammar](builtin-v2-macro-spelling-and-grammar.md)
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md)

## Rulings 2026-09-04 (cleanroom review 3)

From the third cleanroom review of the `Experimental.*` workbench and the
grill session on it. Twelve rulings; settled, and a round that wants to reopen
one stops first.

- **Reads are one `Pro` over a window.** `Window = Whole | Top n | Below n`
  over a `Nat`, and `countReach r pl` across the window is the only anaphora
  gate. `Phrase.Noun.Own` and `ItOtherThan` are deleted, and no
  `bs = own ++ outer` proof survives anywhere. The offset is derived by the
  macro, never written on a card; `Macros.agentRef` re-reads the agent through
  the window instead of splitting on which reference it is.
  Sweep: `workbench-windowed-pro`.
- **Simultaneous moves publish the destination.** After
  `Simultaneously [Move …]` the moved object is readable where it went, as it
  is after `Sequentially`, so `Effect.instrProfile`'s `announced` row is the
  post-move stack. One profile table and one plural-agent function replace the
  two context tables and the four `does*Intro` case tables.
  Sweep: `workbench-profile-table-drift`.
- **Contradiction gates keep only rules-defined clashes** — token against
  card, permanent against spell, status, and whatever else the CR makes
  meaningless. `Phrase.predEq`, `noNegatedPair` and `noRepeatedPair` are
  deleted, not completed. Sweep: `workbench-contradiction-gates`.
- **Exchange is one core row.** `Exchange (what : Exchanged bs)`, with
  `Exchanged` covering life totals, control, and the zone and card exchanges
  of [CR#701.12d..701.12f]. `Effect.ExchangeLife` folds in and the macro of
  two simultaneous control grants goes; atomicity is the rule's own — if the
  entire exchange can't be completed, no part of it occurs [CR#701.12a].
  Sweep: `workbench-exchange-row`.
- **"Tapped for mana" is its own `EventName`** [CR#106.12a]. The
  `forMana : Bool` axis on `Triggers.VerbedEvent` and the single-verb
  `actForMana` column go with it. Sweep: `workbench-tapped-for-mana-event`.
- **The `Card` law bundles are kept.** `MkCharacteristicsLaws`,
  `MkLevelBandLaws`, `MkCardLine`, `MkSharedLineHalfLaws` and
  `MkPrototypeAltLaws` stay as the one exception to "no proof-bundle witness
  types"; every other such type is restated as an obligation on the
  constructor that needs it. No sweep — record the exception in `VERIFY.md`
  beside the `OptOk` note.
- **`ForEachOf` publishes its group binding**, pluralised, after the loop,
  together with the body delta; `ForEachKindOf` follows the same rule.
  Sweep: `workbench-foreach-group-survives`.
- **The zone possessor is derived, not a slot.** "Its owner's hand" and "your
  hand" read the same zone because an object that would go to a library,
  graveyard or hand other than its owner's goes to its owner's [CR#400.3].
  Zones get no possessor slot and `BareScope` stays the spelling. No sweep.
- **`Unless` is a macro, not a row.** It spells the deontic offer with a cost
  (`May offer (Pay …) …`), and the `Effect.Unless` constructor goes.
  Sweep: `workbench-unless-macro`.
- **One `Delta` for every numeric change.** `Words.Delta = Up a | Down a |
  Set a` is shared by all of them; `PtUp`, `PtDown`, `PtShift`, `CharOp`,
  `Gets`, `LifeUp` and `LifeDown` fold away. A characteristic change is one
  `StaticSpec` row `Modify (referent) (property) (delta)`, named for
  [CR#613.4c]; "gets +1/+1 until end of turn" is `Continuously (Modify …)
  span` and "+1/+1" is two `Modify`s combined by a macro; `Becomes` stays for
  non-numeric characteristics. Life is not a continuous effect but one
  `Instruction` row over the same `Delta`, with the engine classifying gain,
  loss and no-op from the resulting total — macros never do arithmetic.
  Counters, loyalty included, keep their own lane.
  Sweep: `workbench-delta-modify`.
- **Deed fit is permissive** wherever the described head type can be animated,
  sharing the `StatOf` predicate: "target land can't attack" is admitted
  [CR#508.1a], and so is a duration-less one-shot restriction. No pin refuses
  on absence. Sweep: `workbench-too-tight-pins`.
- **The standing constraints, restated.** RON-shaped (§1 above); positional
  parameters only; one row per meaning; references as written on the card; no
  case split on which reference a noun is; anaphora by counted uniqueness
  only; no proof-bundle witness type whose constructors carry only implicit
  proofs, the `Card` law bundles excepted; the workbench proves rules text and
  never reminder text [CR#207.2a]; a count of printed cards never refuses a
  rules-meaningful shape; every pin has a same-module positive twin; witnesses
  are VINTAGE-legal cards.

### Clarification 2026-09-04: pairs versus printed words

"One macro per lemma, no agreement or arity pairs" forbids macro pairs that
encode forms of one printed word (verb agreement, argument counts). Two
distinct printed words are two macros: `It` and `Them`, `That` and `Those`.
References are spelled as written on the card; a `Plurality` argument on a
macro is never a stand-in for a different word (`workbench-pronouns-as-written`).
