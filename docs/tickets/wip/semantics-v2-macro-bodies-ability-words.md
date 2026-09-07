---
needs: [semantics-v2-crate, lean-card-soundness-gate]
---
**Give every `plugins_v2/builtin/macros/stubs/ability_words` declaration its semantic
`body`** (61 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

Every ability word has the same body, `ItalicHead(word: AbilityWord(label:
<name>), ability: Param(0))` with `params: [Ability]`; supply it once from
the `AbilityWord` meta-declaration via `Param(name)` rather than in 61
files, unless the meta cannot express it, in which case per-file.

## Landing record

**Approach.** The meta expresses this in full: `params`/`body` no longer
exist as per-file params at all — hardcoded directly in
`plugins_v2/builtin/macros/meta/AbilityWord.ron`'s body template as
`params: [Ability]` and `body: ItalicHead(word: AbilityWord(label:
Param(name)), ability: Quote(Param(0)))`. `Param(name)` is a normal
reference to the meta's own required `name` param, resolved during meta
expansion. `Quote(Param(0))` ([typed-holes delta 4], `macro_ron/src/
expand.rs`) defers that hole past meta-expansion so the PRODUCED
declaration's body carries a literal `Param(0)` for its own future
invocation to resolve — the mechanism the brief's `Param(name)` framing
implied but didn't name. All 61 stub files needed zero changes; each still
reads `AbilityWord(name: "…", spelling: "…", grammar: …)`.

**Declarations given bodies: 61/61** (every file under
`plugins_v2/builtin/macros/stubs/ability_words/`).

**Gate.**
- `cargo test -p xtask --test plugins_v2_declarations`: 1 passed (both
  readers accept every builtin declaration, including all 61 ability
  words).
- Manual expansion proof (throwaway test, not committed — added, run, and
  reverted before commit; `jj diff` on the test file was empty afterward):
  reading the produced `Adamant` declaration gives `params =
  Positional([Ability])` and `body = ItalicHead(word: AbilityWord(label:
  "Adamant"), ability:Param(0))`; substituting a literal `Ability` for
  `Param(0)` and re-parsing as `Ability` succeeds, yielding `ItalicHead {
  word: AbilityWord { label: "Adamant" }, ability: Keyword { keyword:
  "Flying", .. } }`. This is "the reader expands it" in the sense reachable
  from this ticket — see the STOP below for why `Adamant(<ability>)`
  call-syntax itself isn't reachable from a card.
- `cargo xtask lean-check --bless plugins_v2/canon`:
  `plugins_v2/canon: 3/3 card(s) prove Card.check = [] (Generated.Canon)`;
  `lean-check: 1 plugin(s), 3 card(s), 1.6s`. Per a mid-task coordinator
  amendment, cards were authored under `plugins_v2/canon/cards/` (not
  `plugins_v2/testing/`, which is untouched — confirmed via `jj st`), and
  `plugins_v2/canon/lean-check-baseline.ron` was deleted after blessing
  (the ratchet is being retired repo-wide in a parallel change), so no
  baseline is committed.
- `cargo xtask gate --changed --run`: derived line `cargo test -p
  deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask` — all suites
  passed, 0 failed (construction_core, construction, english_v2,
  semantics_v2, xtask unit/integration/doc tests, including
  `plugins_v2_declarations` and all four `lean_check` tests).
- `cargo fmt --all`; `cargo xtask cite check --list-noncompliant` empty;
  `cargo xtask cite check`: 15089 citations, 0 stale; `jj diff --git |
  cargo xtask cite audit --diff`: 1 site audited
  (`plugins_v2/builtin/macros/meta/AbilityWord.ron:2`, `[CR#207.2c]`, text
  read and on-topic — defines ability words and lists the family by name).

**Additions.**
- No new bodyless declarations; no new helper macros (the shape needed no
  helper — every construct used is a raw v2 constructor already in the
  mirror).
- Three fixture cards under `plugins_v2/canon/cards/` (real oracle text,
  verified via `jq` against `data/mtgjson/AtomicCards.json`):
  `Eumidian Terrabotanist.ron` (Landfall — gain 1 life on landfall),
  `Storm Fleet Spy.ron` (Raid — draw a card, ETB, if-you-attacked
  intervening condition), `Mystic Visionary.ron` (Threshold — static
  `AbilityGrant` of flying, `Conditional`/`AsLongAs` on a graveyard-count
  `CompareAmt`). All three spell `ItalicHead(word: AbilityWord(label:
  "<Word>"), ability: <the real ability>)` directly rather than through
  macro-call syntax — see STOP.

**STOP.** `Adamant(<ability>)`-style macro-call invocation of an ability
word at a card's `Ability` text position is unreachable, confirmed
empirically, not by inference: setting the produced declaration's `kinds`
to `[AbilityWord, Ability]` (so it would register at the `Ability`
position too) made `cargo test -p xtask --test plugins_v2_declarations`
fail with `InvalidDeclarationKind` from
`deckmaste_construction_core::macro_def::normalized_kind`, which requires
`let [kind] = definition.kinds.as_slice()` — exactly one kind, always. That
function lives in `crates/deckmaste_construction_core`, outside this
ticket's edit scope (`CLAUDE.md`: don't edit `crates/` unless the ticket
says so). Reverted to `kinds: [AbilityWord]`, matching every sibling
declaration family (`KeywordAbility`, `KeywordAction`, `Designation`,
`TurnPart`, `Subtype`, `CounterKind`, `Type`, `FlavorWord` all declare
exactly one kind too) and matching `ron.rs`'s `DECLARATION_KINDS` doc
comment, which already classifies `AbilityWord` as one of five
"name-erasing loader tags that open no card position at all" by design.
Giving the family a real body does not change this: the block is
structural (one-kind-only), not expressiveness. Widening it is a
`deckmaste_construction_core` change belonging to a separate ticket, not
disclosed here as a decision but flagged for routing. Left bodyless: none
— all 61 have bodies; this STOP is about invocation reachability, not body
coverage.

Substitution disclosed (fixture-card choice, not a body gap): the third
fixture card is a **Threshold** card, not an **Adamant** card as the
brief's "e.g." suggested. Every real Adamant card's condition is "if at
least N mana of the SAME color was spent to cast this spell/creature",
which needs a per-color-amount `PaidFacet`; `words.rs`'s `PaidFacet` has
only `ColorsSpent` (count of distinct colors) and `ManaValueSpent` (total
mana value), no per-color amount. Per "never approximate", substituted a
same-family, Lean-precedented example instead
(`lean/Semantics/Cards/Static.lean`'s `nimbleMongoose`/`Mystic
Visionary`-shaped card). This is a card-authoring gap in the wider v2
model, not specific to ability_words' own bodies, and not fixed here.

No glossary gap: `AbilityWord`/ability words are already covered by
`docs/contexts/oracle-english/CONTEXT.md`'s linguistic vocabulary and
`[CR#207.2c]`.
