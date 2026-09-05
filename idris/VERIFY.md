# Verifying a soundness-suspect card against the Idris oracle

The Idris model in `src/` is dependently typed: a card's effect tree typechecks
only if every anaphor read resolves against the ANTECEDENT STACK (`Semantics.idr`,
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

Builds and typechecks every module listed in `mtg.ipkg` under `%default
total` (`src/Bridge.idr` — the new⇄old translation pairs — is parked out of
the build for now and catches up when the lowering work resumes). `Spec.idr` is the
self-checking regression suite — every `failing "<message>"` block must fail
WITH its pinned message (the same soundness invariants the `cargo xtask
idris-check` re-emit gate enforces on the Rust corpus); `Cards.idr` is the
worked corpus. Exit 0 means the whole model — including every card term —
typechecks. (`./scripts/build` wraps this command; `./scripts/emit-tables`
regenerates `crates/deckmaste_plugin/tables/entailments.ron`, the one table the
Rust engine still loads independently — `deckmaste_engine::entail`; the twelve
other tables the old elaborator consumed, and the per-card resolution
fixtures, were deleted with it.)

## The `Experimental.*` workbench

`src/Experimental/` is the semantics-v2 grammar workbench: `Words`, `Events`,
`Phrase`, `Triggers`, `Effect` and `Card` (re-exported together as
`Experimental`), `Macros` (spellings over the core constructors), `Cards` (the
printed-card bench), `Unspellable`, and the `Proofs` pin modules. Every
module is `%default total`; a card term typechecks only if every
`{auto 0 ok : …}` obligation on its constructors is met, so a type error is a
refused sentence.

The bench and the pins are split by grammar family. `Experimental.Cards` is an
`import public` shim over `Experimental/Cards/<Family>.idr` — `Description`,
`Anaphora`, `Trigger`, `Damage`, `Keyword`, `Counters`, `Mana`, `Deontic`,
`Choice`, `Static`, `Cost`, `Faces`, `Turn`, `Copy`, `Piles`, in that
dependency order; a family module imports only families earlier in the list,
and each ends with the witnesses that carry no family signal. The pins live in
`Experimental.Proofs.<Family>` over the same vocabulary plus `Zone`
(`Anaphora`, `Description`, `Zone`, `Damage`, `Trigger`, `Static`, `Counters`,
`Mana`, `Keyword`, `Deontic`, `Choice`, `Turn`, `Faces`, `Piles`), so a pin is
found from the constructor it refutes. No pin module imports another.

A **pin** is a compiler-checked refusal: `Unspellable T (\ok => term)` states
that the term's open obligation has no proof, and its body (`Oh impossible`,
`Refl impossible`) makes the compiler confirm it. Table assertions are
checked proofs of the same kind, `So (…)` over a facts table with body `Oh`
(e.g. `Words.actLabelsDistinct`). A pin is evidence only once it is
**non-vacuous**: its positive twin — the same constructor at the same slot
with the obligation met — is an ordinary definition **kept beside the pin in
the same module**, so the build re-checks non-vacuity after every core change.
A pin whose twin also fails is refusing nothing. One twin covers the pins
that share its obligation; write it above the first of them, named for what
it admits. A table assertion is probed the same way, by making the table
wrong and watching the proof fail.

A pin's docstring names the **spelling** it refuses, not just the sentence:
where a sibling constructor spells the same printed sentence, the docstring
says which, so the pin is not read as evidence that the sentence itself is
unwritable. Where the refusal message carries the meaning, prefer a named
`data` witness (`OptOk`, `ZoneIs`) over a bare `So (…)`, which collapses to
`Can't find an implementation for So False.` before the message is raised.

The gates, from `idris/`:

    idris2 --build mtg-dev.ipkg    # inner loop: everything but Proofs
    ./scripts/build                # full gate: mtg.ipkg, every module,
                                   # no Error and no Warning lines

Both share `build/`, so the full gate after a dev build re-elaborates only
the `Proofs` modules and what you touched. From the workspace root, the
citation gates then cover everything in the diff that cites the
Comprehensive Rules: `cargo xtask cite check --list-noncompliant` (empty),
`cargo xtask cite check` (0 stale), `cargo xtask cite bless` for newly cited
rules, and `jj --no-pager diff --git | cargo xtask cite audit --diff`, reading
each rule's text against the claim that cites it.

## Check the whole corpus (the automated gate)

`cargo xtask idris-check <plugin>` (e.g. `plugins/canon`) re-emits every
finished card as an equivalent raw `Semantics.idr` term (via
`deckmaste_plugin::idris_emit`) and typechecks the batch with
`idris2 --find-ipkg --check`. Batch mode reads the checked-in
`<plugin>/idris-check-baseline.ron` ratchet and exits nonzero when a required
pass disappears, an emitter gap is not the exact recorded `(card, reason)`
pair, or any emitted card fails its Idris proof. Known emitter gaps stay
visible in both the command output and baseline. New passes and resolved gaps
also stop the gate until the reviewed improvement is recorded, so a card cannot
silently relapse behind a stale gap entry.

After reviewing a deliberate coverage change, regenerate the deterministic
classification with:

    cargo xtask idris-check <plugin> --bless

Blessing refuses to record proof failures. Commit the baseline change with the
emitter/card change that justified it. `cargo xtask idris-check <plugin>
<card>` remains the baseline-free single-card probe and prints the `idris2`
output on failure.

CI's separate **Idris mirror** job bootstraps the pinned Idris2 0.8.0 release,
checks its exact compiler revision, builds the complete `mtg.ipkg` model, runs
the canon baseline gate, and regenerates `entailments.ron`; a byte diff against
the committed table fails the job on model/table drift.

## Check one transcribed card by hand (the oracle loop)

1. Read the Rust card's RON (e.g. `plugins/canon/cards/<Name>.ron`).
2. Transcribe its effect into the Idris term, using the constructor / anaphor
   correspondence (Rust spelling ⇄ Idris spelling):

   | Rust (RON) | Idris |
   | --- | --- |
   | `Sequentially([...])` | `Sequentially [ ... ]` (the telescope — clause i+1 sees clause i's introductions) |
   | `Targeted(targets: [...], effect: e)` | `Targeted [ ... ] e` |
   | `TargetOne(f)` / `Target(Between(1,3), f)` | `Target (^1) f` / `Target (between (^1) (^3)) f` |
   | `Target(n)` (the nth announced target) | `Target n` (a `Reference` reading the nth target slot) |
   | `It` / `That(Card)` / `That(Creature)` | `It` / `That Card` / `That (OfType Creature)` |
   | `They` / `Them(Token)` | `They` / `Them Token` |
   | `Each { binder, effect }` | `Each <binder> (Act <effect>)` |
   | `Distribute { amount, binder, body }` | `Distribute <amount> <binder> (Act <body>)` |
   | `Choose(Exactly(n), filter)` | `Choose (^n) <filter>` (a Many-binder) |
   | `With(ChooseOne(filter), …It…)` | `With (ChooseOne <filter>) (Act …It…)` (the indefinite; the choice binds a frame read by `It`) |
   | `DealDamage(This, Allotment, It)` | `DealDamage This Allotment It` |
   | `Move(It, Library(FromTop(0)))` | `Move It (ToLibrary (FromTop (^0)))` |

3. Put the term in a scratch module at frame `Base`, then check it:

       cat > src/Scratch.idr <<'EOF'
       module Scratch
       import Semantics
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
