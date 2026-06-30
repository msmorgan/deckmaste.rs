# Verifying a soundness-suspect card against the Idris oracle

The Idris model in `src/` is dependently typed: a card's effect tree typechecks
only if every anaphor read is in scope at the right kind and cardinality. A type
error therefore IS a soundness failure — the dependent `Bindings` typestate
(`Core.idr`, `record Bindings`; rename to `Endophora` tracked by
`idris-naming-reconciliation`) rejects an ill-scoped, first-of-many, or wrong-kind
anaphor read. This makes the model an **oracle** for the Rust corpus: transcribe a
suspect Rust RON term into the corresponding Idris term and typecheck it.

## Build / typecheck the whole model

From `idris/`, with `~/.local/bin` on `PATH` for `idris2`:

    idris2 --build mtg.ipkg

Builds and typechecks all six modules (Core, Macros, Experimental, Spec, Cards,
Ron). `Spec.idr` is the self-checking regression suite; `Cards.idr` is the worked
corpus. Exit 0 means the whole model — including every card term — typechecks.
(`./scripts/build` wraps this command.)

## Check one transcribed card (the oracle loop)

1. Read the Rust card's RON (e.g. `plugins/canon/cards/<Name>.ron`).
2. Transcribe its effect into the Idris term, using the constructor / anaphor
   correspondence (Rust spelling ⇄ Idris spelling):

   | Rust (RON) | Idris |
   | --- | --- |
   | `Each { binder, effect }` | `Each <binder> (Act <effect>)` |
   | `DivideAmong { amount, binder, body }` | `Distribute <amount> <binder> (Act <body>)` |
   | `Choose(Exactly(n), filter)` | `Choose (^n) <filter>` (a Many-binder) |
   | `Existing(GetTargets(0))` | `Existing (GetTargets 0)` (needs a `Targeted` frame) |
   | `It` / `That` | `It` / `That` |
   | `DealDamage(It, Allotment)` | `DealDamage It Allotment` |
   | `Move(It, Library(FromTop(0)))` | `Move It (ToLibrary (FromTop (^0)))` |

3. Put the term in a scratch module at frame `Base` (or wrap it in `Targeted [...]`
   if it reads `GetTargets`), then check it:

       cat > src/Scratch.idr <<'EOF'
       module Scratch
       import Core
       import Macros
       suspect : OneShotEffect Base
       suspect = <the transcribed term>
       EOF
       idris2 --check -p elab-util --source-dir src src/Scratch.idr
       rm -f src/Scratch.idr

   No `Error:` (exit 0) ⇒ the shape is sound. A type error names the failure —
   e.g. reading `It` with no binder in scope reports
   `Can't find an implementation for Base .itKind = Just AnObject`, the
   unbound-anaphor soundness failure. (Equivalently: paste the def into `Spec.idr`
   and run `idris2 --build mtg.ipkg`.)

Note: `Existing (GetTargets 0)` only typechecks under a `Targeted [Target …]`
frame — outside one it fails `InBounds 0 (Base .targetKinds)`, the index correctly
demanding that a target slot was announced. This is the oracle, not noise.

## Worked results — the cards this initiative remodeled

Verified with `idris2 --build mtg.ipkg` (whole model, exit 0) and independent
`idris2 --check` of each transcribed term:

| Card | Rust shape (RON) | Idris term | Result |
| --- | --- | --- | --- |
| **Brainstorm** | `Each(binder: Choose(Exactly(2), AllOf([InZone(Hand), Owner(Ref(You))])), effect: Move(It, Library(FromTop(0))))` | `Each (Choose (^2) inHand) (Act (Move It (ToZone Library)))` — `Cards.idr:107` | **typechecks** |
| **Arc Lightning** | `Targeted(targets: [Target(Between(1,3), CreatureOrPlayer)], effect: DivideAmong(amount: 3, binder: Existing(GetTargets(0)), body: DealDamage(It, Allotment)))` | `Targeted [Target (between (^1) (^3)) (Or [creature, Anyone])] (Distribute (^3) (Existing (GetTargets 0)) (Act (DealDamage It Allotment)))` — same shape as `Cards.idr:517` (Electrolyze) | **typechecks** |
| **Scry / Surveil** | `Each` over `TopOfLibrary`, per-element top/bottom (Scry) or top/graveyard (Surveil) modal | `scry` / `surveil` macros, `Macros.idr:218–232` = `Each (Existing (TopOfLibrary n)) (Modal … Move It …)`; exercised by `Spec.idr:447` `scry (Literal 2)` | **typechecks** |
| **negative control** | reading `It` with no binder in scope | `Act (Move It (ToZone Library))` at `Base` | **REJECTED** — `Can't find an implementation for Base .itKind = Just AnObject` (oracle catches the unbound anaphor) |

No remodeled Rust shape failed to typecheck — i.e. the oracle found the sound Rust
shapes sound, and (negative control) still rejects the unsound read.
