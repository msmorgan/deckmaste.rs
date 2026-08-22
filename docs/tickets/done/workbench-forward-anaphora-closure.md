---
needs: [workbench-conditional-and-coordination]
---
# Prove every anaphor resolves forward

The workbench exists to show that all of oracle text's anaphora can be authored
in a forward direction — a card is written in reading order and every
back-reference resolves from what was already bound — without old semantics'
lifting devices (the prenex `Targeted` channel and positional `Target n` reads,
the `With`/`WithChosenValue` prefix binders). Today that claim is prose in
`docs/decisions/semantics-v2.md` and `Bridge.idr`'s T-rules; no ticket
delivers it and nothing would fail if a constructor quietly broke it. This
ticket makes it falsifiable.

Law: **oracle text is strictly anaphoric.** Pronouns and demonstratives follow
their antecedents. No cataphora, no introduction channel above the clause, no
constructor that reads a mention introduced later in reading order. The
alternative — a full-endophora binder — was considered and rejected
(2026-08-21): miserable to build and true to nothing the corpus writes.

## The change

- Enumerate every anaphor constructor (`It`, `They`, `Them`, `Those`, `That`,
  `TheVerbed`, `ThoseVerbed` and any sibling in `Noun`) and state per
  constructor, as a proof or pin rather than a comment, that it resolves over
  `Bindings` by counted uniqueness to a mention introduced earlier in the
  term's reading order.
- Audit every binding-threading function (`nomIntro`, `amtIntro`, `effIntro`,
  `condIntro`/`condDelta`, `preIntro`, `annIntro`, the `Effects` telescope) for
  a site where the threaded context is not the reading-order prefix. Each hit
  is either fixed here or named with its owner.
- Standing exceptions at minting, owned by `workbench-conditional-and-
  coordination`: the `If`/`OnlyIf` split and `WhereLetter`/
  `WhereLetterStatic`'s postposed definition. After that ticket lands, the
  bench has no card authored out of reading order; confirm it.
- Promote the claim to an ADR in `docs/decisions/` — the law above, the
  retired lifting devices by name, and the forward-only binder as the contract
  every new constructor meets — linked from the decisions README.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Proofs*.idr`, `docs/decisions/`. No Rust crate is
touched.

## Acceptance

- Every anaphor constructor carries its forward-resolution proof or pin; no
  threading function hands a clause anything but its reading-order prefix, or
  the exception is owned by a named ticket.
- The ADR exists. `idris/scripts/build` PASS, no witness lost, no pin silently
  passing.

Standard constraints apply.

## As-landed

The law is now `docs/decisions/oracle-text-is-forward-anaphoric.md`, linked
from the decisions README. The proofs are a new module
`idris/src/Experimental/ProofsAnaphora.idr` (929 lines), 19th in `mtg.ipkg`.
No grammar constructor changed: the audit found no threading defect to force
one.

### The proof shape

Every gate in the grammar is one of two folds over `Bindings` — `countBy p`
for the counted reads, `anyBy p` for the marked existence read. Both closure
lemmas are proved once on those:

- `countBySplit` / `anyBySplit`: the gate at a threaded context `delta ++ bs`
  is this clause's own contribution plus the prefix's, with no third term.
  Every threading function has that shape, so this is the sense in which a
  gate is a function of the reading-order prefix.
- `countByWitness` / `anyByWitness`: a satisfied gate hands back the binding
  it resolved to, as an `Elem` of the context.

Twelve `…IsFold` identities tie the grammar's counters to `countBy`
(`countOnes`, `countManys`, `countManysAny`, `countOutcomes`, `countWord`,
`countManyWord`, `countVerbed`, `countManyVerbed`, `countGroups`,
`countParts`, and `countQuality`/`countLetter` via `countOnes` — those two
are `countOnes` at `Quality q` / `Letter w`, proved so rather than asserted).
`anyTargeted` gets the `anyBy` identity. No `believe_me`, no holes.

### Anaphor inventory and per-constructor split

Twenty-one gated anaphor constructors, found by grepping `Experimental.idr`
and `Experimental/Events.idr` for the counted-uniqueness and existence gates:

- `Noun` (8): `It`, `They`, `Them`, `That w`, `Those w`, `TheVerbed v w`,
  `ThoseVerbed v w`, `TheRest`.
- `Amount` (5): `ThatMuch`, `PreventedThisWay`, `GroupSize`, `TheDifference`,
  `DefinedLetter w`.
- `Predicate` (3): `OfChosen q`, `OfLastChosenColor`, `Other`.
- One each: `NameSource.ChosenName`, `TurnDeixis.TurnInScope`,
  `TokenSpec.TokenAsThose`, `ProducedMana.OfChosenColor`,
  `ChoiceMode.TheirChoice` (added at review — it lives in
  `Experimental/Events.idr`, outside the file the first grep covered; its
  gate `countChoosers` is `countOnes Player + countManys Player`, so
  resolution is the `Either` of the two folds' witnesses).

**20 of 21 carry both** a `…ReadsOnlyPrefix` structural witness (a total
function from a fact about `bs` alone to the term — it typechecks only
because the gate takes nothing but the context) **and** a
`…ResolvesInPrefix` proof (the gate names a binding that is an `Elem` of
`bs`). **1 is witness-only**: `TokenAsThose`, because `countTokenSpecs`
overlaps its patterns on determiner, plurality and payload at once, so
restating it as a fold is a 77-case enumeration. It carries the structural
witness plus `noTokenAsThoseWithoutAntecedent` (the empty prefix satisfies
the gate for no card).

Five ungated constructors are recorded as **deixis, not anaphora**, each with
a witness that it is writable at `[]`: `This`, `You`, `PlayerGroup w`,
`AttachHost w h`, `XVal`. `XVal` is the cost variable [CR#107.3a]; the letter
READ over a `WhereLetter` binding is `DefinedLetter`, which is gated.

### Threading audit

Verdicts: FORWARD-EXACT = hands `delta ++ bs`, everything textually earlier.
FORWARD-NARROWED = hands a filtered or restricted earlier set, deliberately.
FORWARD-REWRITTEN = hands earlier material with its determiners re-marked
(`settleTargets`, in `thisWayCtx`/`reflexCtx`/`delayedCtx`) — still only
earlier material, but not literally `delta ++ bs`. No site hands a later
mention.

| function | what it threads | verdict |
| --- | --- | --- |
| `nomIntro n` | `nounDelta n ++ bs` | FORWARD-EXACT |
| `amtIntro a` | `bs`, or `nomIntro`/`predDelta ++ bs` of the amount's own noun or predicate; `Plus`/`Minus` type the right operand at `amtIntro` of the left | FORWARD-EXACT |
| `effIntro e` | the clause's mints on the prefix; `OnlyIf`/`If`/`Unless` export `bs` | FORWARD-EXACT (narrowed for conditionals) |
| `preIntro e` | the clause's mentions before its deed | FORWARD-NARROWED |
| `annIntro e` | the clause's announced mentions; `Sequentially` exports `bs` | FORWARD-NARROWED |
| `condIntro c` | `condDelta c ++ bs` | FORWARD-EXACT |
| `condDelta c` | `[]` for `Exists`/`Happened`/`GameIs`/`NoHolder`/`NotCond`; margin + named phrases for `CompareAmt`; the self re-mention rows for `Matches`; `condDeltaAll` for `AndCond` | FORWARD |
| `otherwiseCtx e` | `outcomesOnly (deedDelta e) ++ annIntro e` — the then-branch's announced phrases and the quantity it wrote | FORWARD; orientation-asymmetric, see below |
| `deedDelta e` | the outcome referents the deed produced; `[]` for containers | FORWARD |
| `Effects` telescope | member *n+1* at `effIntro` of member *n* | FORWARD-EXACT |
| `SimEffects` telescope | member *n+1* at `annIntro` of member *n* | FORWARD-NARROWED |
| `staticIntro se` | `selfSubjIntro`/`nomIntro` of the statement's own subject; containers pass through | FORWARD-EXACT |
| `costIntro c` | `bs`; `Do e` → `effIntro e`; `LoyaltySymbol LoyaltyDownX` mints `letterB LetterX` from the symbol itself | FORWARD-EXACT |
| `thisWayCtx body ev` | `settleTargets (eventAfter ev)` — the event's after-discourse with every `TargetD` re-marked `TheD`; a rewrite of earlier material, not `delta ++ bs` | FORWARD-REWRITTEN |

Supporting threaders checked and clean: `replacedCtx`, `mayIntro`,
`reflexCtx`, `delayedCtx`, `moveIntro` (re-carriers in place over `bs`),
`eventIntro`/`eventAfter`, `exposedIntro`, `elemIntro`, `possessorIntro`,
`lifeIntro`, and the `preIntros`/`simPres`/`annSeqs`/`annSims`/`effsIntro`/
`costsIntro`/`partsIntro` list folds.

**No defect found; no constructor changed.** The structural reason is
recorded in the ADR: an Idris telescope cannot type an argument in a later
argument's context, so the only way to author cataphora is to mint a binding
into an earlier argument's context on behalf of a later one. A grep of
`Experimental.idr` for a constructor consing a fresh binding into a sibling's
context returns exactly three sites, all forward: `WhereLetter` and
`WhereLetterStatic` (`letterB w`, derived from the constructor's own first
argument), and the `also` arms of `Prevents`/`PreventsFrom` (`outcomeB
DamagePrevented`, the prevention the clause itself performs).

Two observations, neither a forwardness defect, neither fixed here:

- **`otherwiseCtx` is orientation-asymmetric.** Under leading `If c e oth`,
  `annIntro e` bottoms out at `condIntro c`, so the arm can read a target
  written inside the condition; under `OnlyIf e c oth` it bottoms out at
  `bs` and cannot. Ledgered on `docs/tickets/planned/workbench-coordination-
  family.md` ("Ledger from round 1 review"); cited, not decided here.
- **`amtDelta` and `amtIntro` order their members oppositely.** `amtIntro` is
  nearest-first (`complementDelta what ++ nomIntro who`; `amtIntro b` for
  `Plus a b`), `amtDelta` is textual order (`nounDelta who ++ complementDelta
  what`; `amtDelta a ++ amtDelta b`). Both are prefix-only and the counted
  gates are order-blind, so nothing here is unsound; it reaches the
  nearest-first readers (`zoneOfIt`, `tyOfIt`, `OfLastChosenColor`) only
  through `condDelta (CompareAmt …)`. Belongs with the `condDelta`-rows
  question already ledgered on `workbench-coordination-family`.

Deliberate narrowings confirmed as intended, not defects: `Search who sc p`
types the predicate at `nomIntro who` rather than at the scope's output;
`Gets n pow tou` types both shifts at `nomIntro n`; `Modal q modes` types
every mode at `bs`, so alternatives do not see each other; `May` types the
`ifNot` arm at `mayCtx offer` rather than at the body's output.

### Bench confirmation

- **`Effect.OnlyIf`: 7 sites**, all printing a trailing condition. Six read
  from printed oracle text this round via the mtg-rules `card` script:
  Overload, Flames of the Raze-Boar, Savage Swipe, Dream Thief, Caustic
  Bronco, Galvanic Blast. The seventh, Disrupting Shoal, prints "Counter
  target spell if its mana value is X." None prints a leading if. (`Macros.
  activatedOnlyIf`, 2 sites, is a different construction — an activation
  guard at `bs`, not `Effect.OnlyIf`.)
- **`WhereLetter`/`WhereLetterStatic`: 17 sites, 1 through the macro.**
  Krenko goes through `Macros.whereLetter`; 8 `WhereLetter` and 8
  `WhereLetterStatic` sites are authored in core binder-first order.
  `Macros.whereLetterStatic` has zero consumers (as
  `workbench-coordination-family` already ledgers). These are recorded
  exceptions to the printed order, **not** exceptions to the law — see the
  verdict below.
- `grep -cE '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards.idr` is still 0.

### `WhereLetter` verdict

**Scoped binder, not cataphora.** The body reads `letterB w`, which is
derived from the constructor's own first argument; `def` is typed at `bs` and
contributes nothing to the binding. Neither argument reads a binding the
other introduces, so binder-first core and body-first macro are both forward.
The departure is from [Semantics v2](../../decisions/semantics-v2.md) §2's
"constructor argument order IS textual order" — the only one in the grammar —
and it is restored by `Macros.whereLetter`.

The justification recorded when the pair was minted
(`docs/tickets/done/workbench-conditional-and-coordination.md`) — that "a
reading-order core would be the cataphoric binder the workbench rejects" —
**does not hold and is withdrawn in the ADR**. A body-first core would type
the body at `letterB w :: bs` exactly as the macro does, with no argument
reading a later argument's mint. The shape stands on the binder convention
(binder before scope, as a `let` is written), not on the law. X is a name an
ability defines [CR#107.3], not a pronoun with an antecedent, which is why
the two orders are informationally independent in the first place.

### Gates

`idris/scripts/build` PASS, `19/19: Building Cards (src/Cards.idr)` on a
clean build. `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
`cargo xtask cite check`: 16088 citations, 0 stale. `cargo xtask cite bless`
registered no new rule (1446 rules, lockfile unchanged) — every rule cited
this round was already in `cr-citations.lock`.
