---
needs: []
---
# Home the `AbilityWord` primitive, then transcribe the attested ability words

The shape is ruled — an ability word is `AbilityWord Name Ability`, a macro
over an ordinary ability, no engine change (`workbench-keyword-policy-audit-
and-ability-words`, which measured the inventory and stopped here). What is
not decided is **where the primitive lives**: no `AbilityWord` constructor
exists in `idris/src/**`, core `Ability` has no ability-word variant, and
`deckmaste_core::ron::kinds()` registers no such kind, so a `.ron` body
`AbilityWord(…)` cannot load. `aw-prefix-parsing` records the same gap. The
binder-unification log (ch. 36) gives it to the semantics-v2 Idris grammar;
`builtin-v2-macro-spelling-and-grammar` rules ability words out of
declaration-backed vocabulary. Decide the home, mint the primitive there, then
transcribe.

Measured inventory (supported corpus, lines/cards), all 61 [CR#207.2c] words
attested — top of the table: Landfall 129/180, Threshold 91/99, Delirium
69/73, Domain 51/54, Channel 38/37, Raid 36/45, Constellation 35/35, Heroic
34/44, Metalcraft 30/35, Morbid 28/29; the full table is in the done ticket's
"As landed" section. Attested but outside [CR#207.2c]: Corrupted 24/24, Will
of the Planeswalkers 1/5 — evidence for `ability-word-catalog-regen`'s
authority question. Scryfall labels with zero supported prints (do not add):
Covercast, Descend (bare), Hero's Reward, Kinfall, Landship, Legacy, Underdog;
"Start your engines!" is a keyword ability [CR#702.179], not an ability word.

## Acceptance

- The primitive's home is recorded (ADR or the rewrite ADR's deviation log),
  the primitive exists there, and every attested word is transcribed; none
  unattested is.
- Build/test gates of the chosen home PASS; `cargo xtask cite check` 0/0.

Standard constraints apply.

## As-landed (2026-08-22)

**Home: the semantics-v2 Idris grammar.** `AbilityWord` is a constructor of
`AbilityAt` in `idris/src/Experimental.idr` — the type enclosing every ability
kind, so one constructor covers the triggered, static, activated and spell
lines a word can prefix. Core is untouched: `deckmaste_core::ron::kinds()` and
`crates/deckmaste_core/src/ability.rs` gain nothing, because [CR#207.2c] gives
the word no rules meaning, so the engine has nothing to carry. No ADR
deviation is owed — `docs/decisions/english-v2-rewrite.md` scopes the Idris
workbench out of the rewrite ("handled separately") and names no crate this
round touches.

**Shape.** `AbilityWord : (word : AbilityWordName) -> (ab : AbilityAt bs) ->
{auto 0 nw : NotAbilityWorded ab} -> AbilityAt bs`. The `So` gate refuses
nesting one word inside another: [CR#207.2c] puts the word at the beginning of
an ability, and a wrapped worded ability spells no second beginning. Because
the word is the outermost wrapper, `lineKeyword` deliberately reads no keyword
through it, which keeps `AlsoForKeywords` inside the word rather than outside.
Everything rules-facing delegates to the inner ability (`grantableAb`,
`emblemAbilityOk`, `abRegime`, `abIntro`, `cardAbilityOk`, `chapterLineOk`,
and — via the new `abDefinesPt` — `textDefines`, so a worded
characteristic-defining line still demands a starred print).

**Vocabulary.** `AbilityWordName` in `idris/src/Experimental/Words.idr`, beside
`Keyword`: the closed [CR#207.2c] list, in the rule's order, 61 constructors.
Words.idr carries no textual spelling for `Keyword` (a bare sum plus
`keywordParamShape`), so `AbilityWordName` is a bare sum too — no "Landfall — "
string is minted here; the em-dash prefix is spelling-layer work.

**CR verdict on the two Scryfall-only labels.** Verified against
`data/rules/cr.txt` with the mtg-rules `rule` script: **Corrupted** and **Will
of the Planeswalkers** are absent from [CR#207.2c]'s enumeration and are therefore
**excluded**, notwithstanding their corpus attestation (24/24 and 1/5) — the CR
is the authority, Scryfall's `ability-words.json` is not. The exclusion is
recorded in the `AbilityWordName` docstring so it is not "fixed" later. The
remaining Scryfall-only labels with zero supported prints were likewise not
added. This is the knowing deviation from the parent ticket's "every attested
word is transcribed" acceptance clause.

**Witnesses** (`idris/src/Experimental/Cards.idr`, oracle text verified with
mtg-rules `card`):

- `steppeLynx` — Landfall over a `Triggered` ability.
- `nimbleMongoose` — Threshold over a `Static` ability.
- `ghorClanRampager` — Bloodrush over an `Activated` ability. (Channel was the
  first pick; it failed on the anaphora gap ledgered below, not on anything
  ability-word-shaped.)

**Chapter gate reads through both wrappers.** `chapterLineOk` matched
`Triggered _ (ChapterMark _) _` positionally under a catch-all `True`, so a
chapter line wrapped in `AbilityWord` — or, pre-existing, in
`AlsoForKeywords` — evaded the Saga check. Both wrappers now delegate; probed
in both directions (refused off a Saga, still accepted on one).

**Gates.** `idris/scripts/build` 18/18 from a clean `build/`;
`cargo xtask cite check --list-noncompliant` 0; `cargo xtask cite check` 0 stale
of 16040; `cite bless` registered no new rule ([CR#207.2c] was already locked);
`cite audit --diff` read all 14 new sites. `Experimental/Cards.idr` still binds
0 implicits.

### Ledger

- **`It` has no antecedent after a discard cost.** Twinshot Sniper's `Channel —
  {1}{R}, Discard this card: It deals 2 damage to any target.` cannot be
  written: `It` demands `countOnes Object bs = 1` and `Do (discards You This)`
  introduces no object binding, so the effect's context is empty. An anaphora
  gap, independent of ability words; a `Channel` witness waits on it.
