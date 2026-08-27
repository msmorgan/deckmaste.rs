---
needs: []
---
# A dead-constructor audit for the Idris workbench

`cargo xtask map idris` (`crates/xtask/src/map.rs`) already inventories every
`data` declaration in an Idris source file — but only as a listing. It never
answers "which of these constructors does no bench card exercise?", and
nothing else in the repo does either: `idris/scripts/build` is a three-line
typecheck-only wrapper with no coverage reporting. That question is currently
unanswerable for any Idris module in the repo.

## The ask

Extend `crates/xtask/src/map.rs`'s `idris` subcommand (or add a sibling mode)
to cross-reference the constructor names `map idris` already finds in a
`data` declaration against occurrences in one or more given files — at
minimum `idris/src/Experimental/Cards.idr` (the evidence bench) and the pin
modules `idris/src/Experimental/Proofs*.idr` (a constructor a pin's *refusal*
exercises is exercised, not just one a bench witness's *success* exercises).
Report constructors named by neither as a plain list.

## What this tool is not, per standing rulings

- **Not a coverage percentage, and not a gate.** Per
  `docs/memory/rulings/measurements-live-in-pins.md`, "the workbench proves
  rules text is self-consistent... it is not a census of printed phrasings."
  This tool's output is a diagnostic list, never a ratio, never wired into
  `idris/scripts/build`'s exit code.
- **Not a pin generator.** An unexercised constructor is not evidence a pin
  is missing, and is not itself grounds to write one — a pin still needs a
  named CR rule making the *positive* term meaningless. An unexercised
  constructor is equally likely to mean "no round has needed it yet," which
  is a normal, un-alarming state for a workbench that (per the same ruling)
  grows by proof, not by corpus sampling.
- **Not a rendering or fidelity check.** Per
  `docs/memory/rulings/rendering-is-not-the-workbench.md`, this stays entirely
  inside constructor-name cross-referencing; it never touches printed Oracle
  text.

The Rust-side precedent for the shape (never the mechanism — no fixture
concept exists on the Idris side) is
`crates/deckmaste_plugin/tests/no_dead_grammar.rs`: every grammar constructor
is either exercised by a fixture or carries a named `DEFERRED` allowlist
entry with a reason. This ticket's tool answers the first half of that
question for Idris; it does not need to build the second half (an allowlist)
to be useful — a first run's findings can be triaged by hand.

## Consumption boundary

`crates/xtask/src/map.rs` (the tool). Reads `idris/src/Experimental/*.idr`
(at minimum `Cards.idr` and the `Proofs*.idr` modules) as plain text, the same
way `map idris` already does; writes no Idris source. No engine crate.

## Acceptance

- The tool reports, for a given Idris source file's `data` declarations,
  every constructor with zero occurrences across the given bench/pin files.
- Output is read-only and diagnostic: no build, gate, or CI step fails on its
  own from this tool's output.
- A first run against `idris/src/Experimental/Words.idr` (or another
  constructor-rich module the conductor picks) is triaged at least once —
  not every finding need be closed, but the findings are recorded here or
  routed to whichever ticket already owns each unexercised arm, so the first
  run demonstrates the tool is actionable rather than merely novel.

Standard constraints apply.

## As landed

### The tool

`cargo xtask map idris-dead [FILE] [--against FILE]…` — a sibling subcommand
in `crates/xtask/src/map.rs` reusing `map idris`'s `data`-declaration scanner
unchanged. FILE defaults to `idris/src/Experimental/Words.idr`; the witness set
defaults to the evidence bench (`Experimental/Cards.idr`) plus every
`Experimental/Proofs*.idr` pin module, so a constructor a pin's *refusal*
exercises is credited exactly like one a bench witness's *success* exercises.
`--against` is repeatable and replaces the default set.

Output is a plain list — declaration heading, then unexercised constructor
names and their lines. It prints no percentage, no ratio, and no "N of M";
declarations with nothing unexercised are omitted entirely. `idris/scripts/build`
is untouched and nothing consumes the output, so no gate can fail from it. The
output's own header says all of this, plus that an unexercised constructor is
not by itself grounds for a pin.

Matching is a whole-token search over witness *code*: `|||` docstring lines,
`--` comments, and string-literal contents are skipped, so a name occurring
only in prose or inside a card's printed name is not miscredited. A qualifier
is a separate token, so qualified references (`Macros.exile`) count. Operator
constructors (`(::)`) are never reported: they are spelled as operators, so a
name search can neither credit nor convict them. Four unit tests in `map.rs`
cover the scanner; `cargo test -p xtask --lib map::` runs them.

The scanner cannot tell a constructor from a same-named type or function, so it
errs towards crediting. That keeps the list free of false accusations at the
cost of some silent under-reporting — the right direction for a diagnostic
nobody should have to defend against.

### First run, triaged

`cargo xtask map idris-dead` over `Words.idr`: 144 of its 380 constructors are
named by no bench card and no pin. Re-running with `--against …/Macros.idr`
appended drops that to 124. (Measurements recorded here, on the deciding
ticket, not in any source docstring.) The findings sort into five classes, and
only the last is a gap anyone should act on.

**1. Reached through a macro body (20).** `AscendW`, `BoostCounter`,
`CardSlot`, `Command`, `Exile`, `Fewer`, `Generic`, `Hand`, `HeldBy`, `Hybrid`,
`InExpansionOf`, `LosesFlip`, `MonstrosityW`, `OnBottom`, `PermanentSlot`,
`SaddleW`, `SameName`, `Simple`, `Specific`, `StartOf`. A bench card writes
`Macros.exile`, not `Exile`, so the name search cannot see the use. This is the
Idris analogue of `no_dead_grammar.rs` crediting macro *bodies*, and it is
answered by a flag rather than a code change: add
`--against idris/src/Experimental/Macros.idr`. Not a gap.

**2. Structurally unnameable by a bench term.** A constructor that appears only
in a *type* — an index a term leaves implicit — can never be spelled by a
witness. `StatusCat`'s `TapC`/`FlipC`/`FaceC`/`PhaseC` occur only as
`StatusVal TapC`-style indices; likewise `DamageableTy`'s
`DamCreature`/`DamPlaneswalker`/`DamBattle`, `Phrasal`'s `PhObject`…`PhJoin`,
`HeadTy`'s `SoleTy`/`JoinTy`, `Targetable`, `TargetExtent`, `Payload`,
`KindAxis`, `SlotCarrier`, `TypeSpace`. This is `no_dead_grammar.rs`'s
`STRUCTURALLY_UNTAGGED` exemption arriving on the Idris side, and it is the
first thing an allowlist would need to carve out if this tool ever grew one.
Not a gap.

**3. Vocabulary spelled only in the declaring module's own fact tables.**
`KeywordParamShape`'s `NoParam`/`CostParam`/…, `StackRegime`'s
`AtCasting`/`AtResolution`, and `ConferringWord`'s `StoriedW`/`RenownW` are
written in `Words.idr`'s `MkKeywordFacts` rows and `conferredDesignation`
clauses — total functions whose coverage checking is itself the exercise.
`AbilityWordName`'s 54 unexercised arms are the same shape: an ability word has
no special rules meaning [CR#207.2c], so the bench witnesses the ability the
word labels, never the label. Not a gap; a witness per ability word would prove
nothing.

**4. Outside the corpus by construction.** `CardType`'s `Conspiracy`,
`Dungeon`, `Phenomenon`, `Plane`, `Scheme`, `Vanguard` and `Supertype`'s
`Ongoing` cannot be exercised by a vintage-playable bench: over
`data/derived/cards.jsonl` filtered to `.supported`, the only card types present
are Creature, Instant, Enchantment, Artifact, Sorcery, Land, Planeswalker,
Kindred, Battle, and the only supertypes are Basic, Legendary, Snow, World.
Permanently unexercisable, correctly modelled, and not a defect.

**5. Corpus-attested with no witness — the real finding.**
`ChapterNumber`'s `ChapterIV`, `ChapterV`, `ChapterVI`: among supported Sagas,
chapter markers `IV —`, `V …` and `VI —` all occur in printed text (35, 2 and 3
occurrences respectively), yet no bench card and no pin names those arms. Same
for `ConferringWord`'s `StoriedW` and `RenownW` once class 3 is discounted —
renown and the storied designation are both on supported cards. These want a
bench card each, not a pin: nothing about them is rules-meaningless. No ticket
currently owns them, so they are recorded here and flagged to the coordinator
rather than routed.

The rest of the 124 fall under "no round has needed a witness yet", which is the
normal state this tool exists to make visible rather than alarming.

### Deliberately not built

No allowlist and no `DEFERRED`-style exemption file — the ticket says the first
half is useful alone, and the triage above shows the two exemption classes an
allowlist would have to encode (2 and 3) before one is worth writing. No pin was
written and no constructor was deleted: every finding above is either explained
or wants a bench witness.
