---
needs: [lean-v3-drt-checker]
---
# Enforce the Definition vs. Template split in Lean declarations and macros

**Separate rules-defined `Definition`s (`ownScope` + `inCaller` + parameter
identity) from transparent phrasing `Template`s (caller scope + multi-mention
parameter identity) across `lean/Semantics/`, retire the 9 hand-coded window
macros, and land the `airbend` and `ripple` keyword bodies.** Third
implementation step of [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)
(§2). Standard constraints apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §2)

- **Explicit `Definition` vs. `Template` split:**
  - **`Definition`s:** Every rules-defined term — all CR §701 keyword actions
    (both atomic actions like `destroy` `[CR#701.8]`, `exile` `[CR#701.13]`,
    `sacrifice` `[CR#701.21]`, `tap` `[CR#701.26a]`, `untap` `[CR#701.26b]`,
    `attach` `[CR#701.3]`, `counter` `[CR#701.5]`, `create` `[CR#701.6]`,
    `discard` `[CR#701.9]`, `exchange` `[CR#701.12]`, `fight` `[CR#701.14]`,
    `goad` `[CR#701.15]`, `regenerate` `[CR#701.19]`, `reveal` `[CR#701.20]`,
    `search` `[CR#701.23]`, `shuffle` `[CR#701.24]`, `transform` `[CR#701.27]`,
    `convert` `[CR#701.28]`, `fateseal` `[CR#701.29]`, `clash` `[CR#701.30]`,
    `proliferate` `[CR#701.34]`, `detain` `[CR#701.35]`, `populate` `[CR#701.36]`,
    `monstrosity` `[CR#701.37]`, `vote` `[CR#701.38]`, `meld` `[CR#701.42]`,
    `exert` `[CR#701.43]`, `cloak` `[CR#701.58]`,
    `suspect` `[CR#701.60]`, `plot` `[CR#701.62]` and multi-step actions like
    `double*` `[CR#701.10]`, `triple*` `[CR#701.11]`, `investigate` `[CR#701.16]`,
    `mill` `[CR#701.17]`, `scry` `[CR#701.22]`, `surveil` `[CR#701.25]`,
    `bolster` `[CR#701.39]`, `manifest` `[CR#701.40]`, `support` `[CR#701.41]`,
    `explore` `[CR#701.44]`, `amass` `[CR#701.47]`, `learn` `[CR#701.48]`,
    `connive` `[CR#701.50]`, `incubate` `[CR#701.53]`, `theRingTemptsYou` `[CR#701.54]`,
    `timeTravel` `[CR#701.56]`, `discover` `[CR#701.57]`, `collectEvidence` `[CR#701.59]`,
    `forage` `[CR#701.61]`, `manifestDread` `[CR#701.63]`, `endure` `[CR#701.64]`,
    `airbend` `[CR#701.65]`, `earthbend` `[CR#701.66]`, `waterbend` `[CR#701.67]`,
    `firebend` `[CR#701.68]`), all CR §702 keyword abilities, and all registry
    definitions (`CounterKind`, `Subtype`, `Designation`, `Relation`,
    `AbilityWord`, `FlavorWord`, `TurnPart`, `Type`).
    - Evaluated in their own Reference Scope (`Instruction.ownScope (some (.action name)) body`)
      with caller-evaluated arguments (`inCaller`) and `firstMention`/`laterMention`
      parameter identity, preserving the keyword action's `Deed.action name`
      header so `does`/`would`/`did`/`cant` and `deckmaste_lowering_v3`
      (`core::Action::Composite` `[CR#701.22b]` and `core::EventFilter::Act`)
      read the keyword action's deed and caller-captured arguments directly from
      the `ownScope` header.
    - When checking the definition law of an atomic `Definition` whose body is a
      direct `.enact` of itself, the checker strips the outer `.ownScope` so its
      effect on the parameter referents is verified directly.
  - **`Template`s:** All other Oracle phrasing helpers and constructor aliases in
    `lean/Semantics/Macros.lean` / `Declared.lean`.
    - Transparent caller scope (`no` `ownScope`, `no` `inCaller`).
    - Any `Template` that mentions a `NounPhrase` parameter $>1$ time before a
      zone change binds `firstMention` on its first occurrence and `laterMention`
      on subsequent occurrences (`duringNextTurnOf`, `lookAndSortSlice`,
      `regenerationApplication`, `controllerSacrifices`, `dealDamageOwnPower`,
      `comparesOwnStat`) so targets and indefinites are never duplicated
      `[CR#115.1,608.2c]`.
- **Retire the 9 hand-coded window macros (`lean/Semantics/Macros.lean`):**
  - `itPrior`, `itCondSubject`, `attachToIt`, `requireBlockIt`, `agentRef`,
    `itsOther`, `ownSubject`, `controllerSacrifices`, and `lookedCards` drop
    their hand-built `Window` plumbing and use ordinary pronouns (`it`, `them`,
    `that <noun>`) resolved by `D′` and `AC>R1` (or parameter identity where the
    template mentions a parameter twice before a zone change).
- **Moved-parameter readback (`[CR#400.7,400.7j]`):**
  - `connive` / `discardThenForEachNonland` `[CR#701.50a]` loops over
    `"each nonland card discarded this way"`:
    `allOf(and([not(land), card, did(any, discard(which), thisWay)]))`.
  - `airbend` `[CR#701.65a]` (absorbing `semantics-v2-per-member-handoff`,
    `"For each card exiled this way, for as long as it remains exiled, its owner
    may cast it by paying {2} rather than paying its mana cost"`):
    - Each exiled card's own owner gets a permission for that card (`doForEach`
      over `allOf(and([card, did(any, exile(which), thisWay)]))` with
      `instruct(ownerOf(it), establish(may(ownerOf(it), cast(it, ...), ...),
      forAsLongAs(matches(it, inZone(exileZone)))))`).
    - Because `doForEach` introduces the singular loop referent at quantifier
      header clause `C` and advances `drs.currentClause` to `C + 1` before
      stepping the loop body (`semantics-v3.md` §1.3 Box 4), and `Refusal.unusedCapture`
      is retired (`§1.1`), each `it` inside `doForEach` resolves by `D′`
      (nearest clause `C > C_exile`) to the loop's card referent for both one
      exiled object (flipping `airbendByCardOneTiesUnderStrictRule` to `[]`) and
      two or more exiled objects (`okAirbendByCardUpToTwo` checks `[]`, and
      `airbendItRefusesSeveral` is re-spelled as an acceptance under the
      per-card loop).
  - `ripple` `[CR#702.60a]` (routed from `semantics-v2-piles`): `reveal` stamps
    its revealed slice with `.actedOn (.action "reveal")` (or `"those cards"`
    resolves via `AC>R1` after the cast step), giving `ripple` its full body.

## Proof

- Port `lean/Semantics/Macros.lean`, `lean/Semantics/Macros/Primitives.lean`,
  `lean/Semantics/Macros/Declared.lean`, and `lean/Semantics/Authoring.lean`
  from `lean/archive/SemanticsV2/` onto the `v3` AST and `DRS` checker, and add
  `"Semantics.Macros"` and `"Semantics.Macros.Declared"` back to `roots` in
  `lean/lakefile.toml`.
- `./lean/scripts/build` and `lake build --wfail Semantics.Authoring` pass
  cleanly with zero warnings.
- The 9 hand-coded window macros in `lean/Semantics/Macros.lean` are retired or
  reduced to ordinary pronoun/parameter templates with zero `Window` arguments,
  and `comparesOwnStat` / `dealDamageOwnPower` no longer use `Window.top`.
- `airbend` (`[CR#701.65a]`), `connive` (`[CR#701.50a]`), and `ripple`
  (`[CR#702.60a]`) check cleanly by `decide`.
