# Verifying a soundness-suspect card against the Idris oracle

The Idris model in `src/` is dependently typed: a card's effect tree typechecks
only if every anaphor read resolves against the ANTECEDENT STACK (`Core.idr`,
`Ctx = MkCtx stack …`, v2) at the right sort, kind, and cardinality — R1
nearest-compatible, the R2 uniqueness gate, R3 strictly-leftward (the telescope
`Sequence`). A type error therefore IS a soundness failure: an unbound anaphor,
an ambiguous one (two compatible antecedents), a wrong-sort read ("that card"
against a token), or a stale target in a `Delayed` body is rejected at
compile time. This makes the model an **oracle** for the Rust corpus: a
suspect Rust RON term can be transcribed into the corresponding Idris term and
typechecked. The Rust load-time elaborator is gone — soundness is now proven
POSITIVELY, by re-emitting each Rust card as Idris and typechecking it (see
"the oracle loop", below); at engine eval time the Rust runtime resolves
anaphora dynamically over the frame, degrading an unresolvable read to the null
object (never-crash), separate from this compile-time gate.

## Build / typecheck the whole model

From `idris/`, with `~/.local/bin` on `PATH` for `idris2`:

    idris2 --build mtg.ipkg

Builds and typechecks all six modules (Core, Macros, Cards, Spec,
Experimental, EmitTables) under `%default total`. `Spec.idr` is the
self-checking regression suite — every `failing "<message>"` block must fail
WITH its pinned message (the same soundness invariants the `cargo xtask
idris-check` re-emit gate enforces on the Rust corpus); `Cards.idr` is the
worked corpus. Exit 0 means the whole model — including every card term —
typechecks. (`./scripts/build` wraps this command; `./scripts/emit-tables`
regenerates `crates/deckmaste_cards/tables/entailments.ron`, the one table the
Rust engine still loads independently — `deckmaste_engine::entail`; the twelve
other tables the old elaborator consumed, and the per-card resolution
fixtures, were deleted with it.)

## Check the whole corpus (the automated gate)

`cargo xtask idris-check <plugin>` (e.g. `plugins/canon`) re-emits every
finished card as an equivalent raw `Core.idr` term (via
`deckmaste_cards::idris_emit`) and typechecks the batch with
`idris2 --find-ipkg --check`. It reports how many cards typecheck and, for the
rest, whether it's an emitter gap (no Idris text produced) or an Idris proof
failure (emitted but rejected — a genuinely unsound card, or an over-strict
Idris proof). `cargo xtask idris-check <plugin> <card>` checks one card and
prints the `idris2` output on failure.

## Check one transcribed card by hand (the oracle loop)

1. Read the Rust card's RON (e.g. `plugins/canon/cards/<Name>.ron`).
2. Transcribe its effect into the Idris term, using the constructor / anaphor
   correspondence (Rust spelling ⇄ Idris spelling):

   | Rust (RON) | Idris |
   | --- | --- |
   | `Sequence([...])` | `Sequence [ ... ]` (the telescope — clause i+1 sees clause i's introductions) |
   | `Targeted(targets: [...], effect: e)` | `Targeted [ ... ] e` |
   | `TargetOne(f)` / `Target(Between(1,3), f)` | `Target (^1) f` / `Target (between (^1) (^3)) f` |
   | `Target(n)` (the nth announced target) | `Target n` (a `Reference` reading the nth target slot) |
   | `It` / `That(Card)` / `That(Creature)` | `It` / `That Card` / `That (OfType Creature)` |
   | `They` / `Them(Token)` | `They` / `Them Token` |
   | `Each { binder, effect }` | `Each <binder> (Act <effect>)` |
   | `DivideAmong { amount, binder, body }` | `Distribute <amount> <binder> (Act <body>)` |
   | `Choose(Exactly(n), filter)` | `Choose (^n) <filter>` (a Many-binder) |
   | `With(ChooseOne(filter), …It…)` | `With (ChooseOne <filter>) (Act …It…)` (the indefinite; the choice binds a frame read by `It`) |
   | `DealDamage(It, Allotment)` | `DealDamage It Allotment` |
   | `Move(It, Library(FromTop(0)))` | `Move It (ToLibrary (FromTop (^0)))` |

3. Put the term in a scratch module at frame `Base`, then check it:

       cat > src/Scratch.idr <<'EOF'
       module Scratch
       import Core
       import Macros
       suspect : OneShotEffect Base
       suspect = <the transcribed term>
       EOF
       idris2 --check --source-dir src src/Scratch.idr
       rm -f src/Scratch.idr

   No `Error:` (exit 0) ⇒ the shape is sound. A type error names the failure —
   e.g. reading `It` with no antecedent reports
   `Can't find an implementation for (case innermostBinder (Base .stack) of …) = Bound ?k`
   (the unbound-anaphor failure; an AMBIGUOUS read fails on the same goal with
   two candidates in the displayed stack). (Equivalently: paste the def into
   `Spec.idr` and run `idris2 --build mtg.ipkg`.)

Note: a target-reading anaphor only typechecks under a `Targeted [Target …]`
frame — outside one it fails with an empty-stack resolution, the index
correctly demanding that a target slot was announced. A `That (OfType T)` in a
`Delayed` body fails when its only compatible antecedent was the dropped
target ([CR#603.7c]) — that stale-target rejection is the oracle, not noise.

## Worked results — the signature v2 shapes

Verified with `idris2 --build mtg.ipkg` (whole model, exit 0):

| Shape | Idris term | Result |
| --- | --- | --- |
| **Cloudshift** (exile → return that card, sentence order) | `card_Cloudshift` / `Spec.tTelescopeCloudshift` | **typechecks** |
| **Arc Lightning** (plural slot → `They` → divided damage) | `Spec.tPluralTarget` | **typechecks** |
| **Chandra [0]** (create two tokens — THEY gain haste — sacrifice them) | `Spec.tTelescopeTokens` | **typechecks** |
| **Through the Breach** (May + indefinite + that-creature survival) | `card_ThroughTheBreach` | **typechecks** |
| **negative control** (stale target: `That (OfType Creature)` after `Delayed`) | `Spec.tBadDelayedTarget` | **REJECTED**, message pinned `"unbindTargets"` |
