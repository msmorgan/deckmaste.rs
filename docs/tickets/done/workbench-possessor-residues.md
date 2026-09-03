---
needs: []
---
**Give "each of your postcombat main phases" and "the chosen player's … that player" their own spellings.** Two residues of `workbench-possessor-noun` (2026-09-03).

- `Owner.EachYours` had no replacement: "each of your postcombat main phases"
  (Sphinx of the Second Sun, Brazen Cannonade) collapsed to `Macros.yours`,
  because the `each` quantifies the turn PART, not the possessor. The header
  needs a part quantifier (a `TurnPart` under `EachDet`-style distribution),
  not a possessor value.
- A choice binding and a definite description over it are two bindings for
  one referent: after "the chosen player's upkeep", Black Vise's "that
  player" is refused (`countReach (Word PlayerW) OneOf = 2`, since `choiceB
  PlayerC` and the possessor's own binding both reach `PlayerW`); the bench
  repeats `Macros.the ChosenPlayer`. This is `Described TheDet` over a choice
  generally: the definite description should re-read the choice binding, not
  mint a second one.

Size: S each. Done when: both cards read as printed (Sphinx of the Second
Sun's second main phase; Black Vise's "that player"), each with a witness,
and a pin refuses the double binding. Standard constraints apply.

## As landed

- **Part quantifier.** `Words.PartQuant = ThePart | EachPart` ([CR#500.8,505.1a]:
  a turn can hold more than one instance of a part), and `Triggers.BeginningOf`
  takes it as its first positional slot; `Macros.beginningOfPossessed` forwards
  it. `Cards.sphinxOfTheSecondSun` and `Cards.brazenCannonade` read
  `BeginningOf EachPart PostcombatMain Macros.yours` — "each of your postcombat
  main phases" as printed, the `each` on the part and `yours` still the
  possessor. Witness: `Cards.sphinxOfTheSecondSun`.
- **Choice re-read.** `Phrase.choiceRead` (true for `ChosenPlayer` and
  `TheLastChosenPlayer`) gates a new `detDelta TheDet` clause: a definite
  description over a choice yields `predDelta p` alone instead of minting a
  binding, because "the chosen player" refers to the choice rather than making
  one [CR#607.2d]. `Cards.blackVise` now reads "that player" and "their hand"
  as `Macros.They`, not a repeated `Macros.the ChosenPlayer`. Witness:
  `Cards.blackVise`. Pin: `ProofsG.badIndefiniteChosenPlayerRead` — "a chosen
  player … that player" is refused, since only the definite re-reads and the
  indefinite binds a second player.
- Undone: nothing in the ticket's letter. `PartQuant` carries no gate — see
  Deviations.

## Landing record

- Construction count: `Words.PartQuant` 0 → 2; `Triggers.BeginningOf` arity
  2 → 3; `Phrase.detDelta` clauses 3 → 4 and `Phrase.choiceRead` +1;
  `Macros.beginningOfPossessed` arity 2 → 3; `ProofsG` pins +1; bench cards
  unchanged in count (`sphinxOfTheSecondSun`, `blackVise` re-spelled in place).
- Bench: 84 header sites re-spelled (80 `BeginningOf`, 4
  `beginningOfPossessed`), of which 2 take `EachPart`; 0 witnesses removed.
- Coverage and lock state: `cr-citations.lock` +1 rule ([CR#505.1a]), registered
  by `cargo xtask cite bless` — it added exactly that line and pruned nothing
  (1572 rules blessed).
- Gates (all foreground): clean `rm -rf build && ./scripts/build`, last line
  `23/23: Building Cards (src/Cards.idr)`, 0 `Error` and 0 `Warning` lines;
  `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`; `cargo xtask cite check` → `checked 17764
  citations against cr.txt (eff. 2026-08-07); 0 stale`; `cargo xtask cite audit
  --diff` → `audited 3 citation site(s) — read each rule text against its
  claim`, all three read ([CR#607.2d] twice, [CR#500.8,505.1a] once).
- Probes (scratch module, deleted afterwards): the positive twins
  `BeginningOf EachPart PostcombatMain Macros.yours` and "the chosen player …
  that player" both typecheck as terms; the negative "a chosen player … that
  player" fails on `countReach (Word PlayerW) OneOf … = 1`. Re-stated as the
  pin and mis-stated once (`Macros.a` → `Macros.the`), it is refused
  `badIndefiniteChosenPlayerRead Refl is not a valid impossible case`, so the
  pin is non-vacuous.
- Assurance: restored 0; re-spelled 10 (the `ThePart` insertion in
  `Proofs`/`ProofsC`/`ProofsF`/`ProofsG` terms — every one still refutes or
  typechecks for its own reason); ignored 0; added 1
  (`badIndefiniteChosenPlayerRead`); removed 0.
- Deviations and additions:
  - `PartQuant` carries no gate. "each of each player's postcombat main
    phases" is refused by English, not by the Comprehensive Rules, and the
    house rule is that a pin refuses only what the CR makes meaningless — so
    no `partQuantOk` conjunct was added to `PartTriggerable`. `ByPlayer`'s
    gateless slot from `workbench-possessor-noun` is the precedent.
  - `choiceRead` covers `TheLastChosenPlayer` as well as `ChosenPlayer`;
    [CR#607.2d] names both forms as references to the linked choice.
  - The re-read is `TheDet`-specific rather than determiner-agnostic. Making
    every determiner re-read would leave the double binding unpinnable; the
    new pin is exactly the non-definite case.
  - The quantifier is a slot on `BeginningOf` only. `Timing.DuringPart`,
    `Phrase.StartOf`/`EndOf` and `Effect.OnlyDuring` keep their unquantified
    part, which the ticket's letter does not touch.
- STOPs: none.
