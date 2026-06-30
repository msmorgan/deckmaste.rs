---
needs: [cards-remodel-bindable]
---
Close the loop: make the Idris model the verification oracle for the corpus, and
correct the naming policy that let concept-drift hide (see
[[core-bindable-unification]] for the north star).

## Changes

- **Rewrite `docs/tickets/planned/idris-naming-reconciliation.md`.** Its current
  decision ("Rust naming is canonical, rename Idris to match") entrenched the
  fragmentation this initiative undoes. Replace with the corrected policy: **mirror
  Idris concepts; Rust spellings only where the concept is identical; never
  preserve a weaker Rust concept behind a matching name.** Record the
  `Bindable`/`It`/`That`-by-slot reconciliation as the worked example, and fix the
  imprecise `It`/`That` → `ThatObject`/`ThatPlayer` mapping (It is overloaded
  across iteration/projection; `That` → `That`).
- **Stand up the verification loop.** A documented procedure (and, if cheap, a
  small script) to transcribe a soundness-suspect Rust card into an Idris term and
  run `idris2 --check` against `idris/mtg.ipkg` — a type error IS the soundness
  failure. The model already typechecks (`idris2`/`pack` installed, `build/ttc`
  present), so this is "use it as the oracle," not "rebuild it."
- **Verify the re-modeled suspect cards** — the ones exercising nested `Each`/
  `Distribute`, many-binders, and `It`/`That` reads (Brainstorm, Arc Lightning,
  Scry/Surveil, the `Distribute` cards). Record which were checked.

## Done
- Naming ticket rewritten; the verification procedure documented and demonstrated
  on at least the suspect cards above (each either typechecks in Idris or the
  divergence is surfaced and fixed).

## Outcome (verification record)

- **Naming policy rewritten** — `docs/tickets/planned/idris-naming-reconciliation.md`
  now records the corrected policy ("mirror Idris *concepts*; keep a Rust *spelling*
  only where the concept is identical; never preserve a weaker Rust concept behind a
  matching name"), the `Binder`/`It`/`That`-by-slot worked example, the corrected
  `It`/`That` anaphor mapping (It is one overloaded iteration+projection anaphor,
  NOT split; That→That; Subject/ThatObject/ThatPlayer folded into It/EventObject/
  EventActor), and the new **`Bindings → Endophora`** rename (it tracks exactly the
  text-internal endophoric refs — anaphora + cataphora — while This/You/Opponent
  stay out as exophora; Rust side via [[endophora-consolidation]]).
- **Oracle stood up** — build command `idris2 --build mtg.ipkg` (from `idris/`):
  all 6 modules build, exit 0. Per-card `idris2 --check` loop and results are in
  `idris/VERIFY.md`.
- **Suspect cards verified** (all typecheck; correspond to sound Rust shapes):
  - Brainstorm — `Each (Choose (^2) inHand) (Act (Move It (ToZone Library)))`
    (`Cards.idr:107`). PASS.
  - Arc Lightning — `Targeted […] (Distribute (^3) (Existing (GetTargets 0))
    (Act (DealDamage It Allotment)))`, same shape as Electrolyze `Cards.idr:517`.
    PASS.
  - Scry/Surveil — `Each (Existing (TopOfLibrary n)) (Modal … Move It …)` macros
    (`Macros.idr:218–232`, exercised in `Spec.idr:447`). PASS.
  - Negative control — `It` at a frameless `Base` is REJECTED
    (`Can't find an implementation for Base .itKind = Just AnObject`), confirming a
    type error IS the soundness failure.
  - No remodeled Rust shape failed to typecheck.
