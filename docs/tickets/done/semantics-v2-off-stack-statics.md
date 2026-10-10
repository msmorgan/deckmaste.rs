---
needs: []
---
**Off-stack statics: "as [event], do" and "any time [condition] holds, do".**
Ruling 2026-10-09 ("An off-stack static is a special action's sibling",
`docs/decisions/semantics-v2.md` §7). Bodies twelve census residues: dash,
blitz, warp, rebound, ascend, storied, daybound, nightbound, devour, riot,
tribute, unleash. Standard constraints apply.

## Shape

- Two `StaticSpec` rows beside `specialAction (when) (cost) (does)`, each
  carrying an `Instruction` that never uses the stack
  [CR#604.1,603.7a,603.7g,614.1c]:
  - `asEvent (event : GameEvent) (does : Instruction)`: "as [event], do";
  - `whileHolds (condition : Condition) (does : Instruction)`: "any time
    [condition] holds, do".
- `NounPhrase.becomes (subject : NounPhrase)`: the object's successor after
  its next zone change [CR#400.7] ("the permanent this spell becomes").
- `deonticRule` gains `except : Option Cause`, `Cause = byKeyword (label :
  KeywordLabel) | byObjects (predicate : Predicate)`: "can't transform except
  due to its daybound ability" [CR#702.145b], "can't be blocked except by
  artifact creatures …" [CR#702.13b]. Fear, intimidate and the evasion
  family re-spell onto `byObjects` where they use a negated predicate today.
- Checker: kinds and references; `becomes` takes an object; `except` reads a
  keyword label or an object predicate. No zone or timing rules.
- Rust mirror, RON helpers `asEvent`, `whileHolds`, `becomes`, bench macros.

## Bodies

- dash, blitz, warp: `asEvent(paysCost(you, byKeyword(K)),
  delay(beginningOf(endStep, none, next), <return to hand | sacrifice |
  exile>(becomes(thisSpell))))` [CR#702.109a,702.152a,702.185a,603.7g], plus
  haste (dash, blitz) and blitz's draw trigger via `conditional(…,
  costWasPaid)`; warp's later-turn cast via `onALaterTurn`.
- rebound: a `replacement` of the resolve-to-graveyard zone change by "exile
  it; delay(beginningOf(upkeep, you, next), may(cast it withoutPaying))",
  under `castFrom(hand)` [CR#702.88a,603.7f]. No new node.
- ascend (permanent): `whileHolds(and([<you control ten or more
  permanents>, not(hasDesignation(citysBlessing, you))]),
  gainDesignation(you, citysBlessing, restOfGame))` [CR#702.131b]; ascend
  (spell): the same under the instant-or-sorcery class guard, as gift
  [CR#702.131a]. storied likewise over its permanents and designation
  [CR#702.195a].
- daybound: conditional `entersTransformed` if it is night;
  `asEvent(gameBecomes(night), doIf(frontFaceUp, transform(this)))`; the
  can't-transform deontic with `except byKeyword(daybound)` [CR#702.145b].
  nightbound mirrors it [CR#702.145e]. [CR#702.145c,702.145d,702.145g,502.2]
  are the engine's.
- devour: `asEvent(enters(this), sequentially([may(sacrifice(anyNumberOf(
  <quality>))), entersWithCounters(this, times(N, countOf(theVerbed(
  sacrifice, Many, …))), p1p1Counter)]))`, written as the general
  `devourQuality(creature, N)` [CR#702.82a,702.82c], as connive reads
  `conniveN(1)`.
- riot: `mayOrElse(entersWithCounters(this, 1, p1p1Counter, additional),
  gains(haste))` [CR#702.136a].
- tribute: `sequentially([choose(an(opponent)), act(that(Opponent),
  may(entersWithCounters(this, N, p1p1Counter, additional)))])`
  [CR#702.104a].
- unleash: the optional counter plus the existing conditional can't-block
  [CR#702.98a].
- Counter kinds are always written. "It devoured" [CR#702.82b] and "tribute
  wasn't paid" [CR#702.104b] read `semantics-v2-linked-readback`'s `objects`
  and `taken` facets; if that ticket has not landed, add no readback pins and
  name it in the landing record.
- amplify [CR#702.38a] is an entry clause too but needs
  `semantics-v2-sharing`'s `shares`; body it here only if sharing has landed.

## Witnesses (canon, non-keyword)

Armor of Thorns (`becomes`), a shockland ("pay 2 life; if you don't, it
enters tapped"), Game Trail (reveal land), Dracoplasm, Callous Oppressor, one
"it becomes day as this enters" card (`asEvent(enters(this),
setGameDesignation(day))`), and a canon card per keyword bodied. Pins:
`becomes` of a player is refused (a kind); each body and witness checks.

## Siblings

Cross-references `semantics-v2-linked-readback` (readbacks) and
`semantics-v2-sharing` (amplify, intimidate's `except`); neither blocks.
Refresh before integrate; `Words`/`Phrase`/`Abilities` conflicts expected.

## Landing record

Measured on `ywuxlpqk` (the canon commit), after `kata refresh` onto a trunk
on which `semantics-v2-sharing`, `semantics-v2-linked-readback` and
`semantics-v2-text-changes` have all landed.

1. **Census delta**: not applicable. No English grammar, lexicon or keyword
   spelling changed, so no english-v3 census was run. Covered faces and
   Readings are unchanged. Lost faces: none.
2. **Witnesses**: English Readings do not apply (semantics-v2 only). Lean:
   `cargo xtask lean check` proves 226/226 cards (218 canon, 8 testing).
   Among them are 19 new canon witnesses:
   - keywords: Mardu Scout (dash), Plasma Jockey (blitz), Bygone Colossus
     (warp), Staggershock (rebound), Skymarcher Aspirant (ascend), Ori, Keeper
     of Songs (storied), Hookhand Mariner // Riphook Raider (daybound and
     nightbound), Gorger Wurm (devour), Zhur-Taa Goblin (riot), Gore-House
     Chainwalker (unleash), Glowering Rogon (amplify);
   - readbacks: Skullmulcher ("it devoured") and Snake of the Golden Grove
     (tribute and "tribute wasn't paid");
   - non-keyword: Steam Vents (shockland), Game Trail, Dracoplasm, Callous
     Oppressor, Armor of Thorns (`becomes`), and Firmament Sage ("it becomes
     day as this creature enters").

   Bladetusk Boar (intimidate) is re-spelled through its body.
3. **Spot checks** (bodies as they expand in `Declared.lean`):
   - dash: a permission, `asEvent(paysCost(you, paid, thisSpell, dash),
     delay(beginningOf(endStep, none, next), return(becomes(thisSpell),
     hand)))`, and haste `asLongAs` the dash cost was paid [CR#702.109a].
   - blitz: the same, with `sacrifice` and a granted dies-draw trigger
     [CR#702.152a].
   - warp: a permission from your hand; at the next end step, exile
     `becomes(thisSpell)` and give its owner a cast permission `onALaterTurn`
     [CR#702.185a].
   - rebound: `onlyIfSo(insteadOf(zoneChange(thisSpell, from stack, to
     graveyard), exile it; delay(beginningOf(upkeep, you, next), may(cast this
     withoutPaying))), castFrom(yourHand))` [CR#702.88a].
   - ascend: `whileHolds(and([count of permanents you control ≥ 10, not(you
     have citysBlessing)]), gainDesignation(you, citysBlessing, restOfGame))`
     under the permanent class guard, plus the guarded spell form
     [CR#702.131a,702.131b].
   - storied: the same, over artifacts, Sagas and legendaries [CR#702.195a].
   - daybound: enters transformed if it is night; `asEvent(gameBecomes(night),
     doIf(front face up, transform))`; `cantExcept(transform, this,
     byKeyword(daybound))` [CR#702.145b].
   - devour N: `devourQualityExpansion(creature, N)` [CR#702.82a,702.82c].
   - riot: `asEvent(enters, mayOrElse(+1/+1 entry rider, gain haste))`
     [CR#702.136a].
   - tribute: `asEvent(enters, choose an opponent; act(the chosen player,
     may(+N/+N entry rider)))` [CR#702.104a].
   - amplify: `asEvent(enters, reveal any number of cards in your hand that
     share a creature type with it; enters with N × that many counters)`
     [CR#702.38a].
   - fear, intimidate and flying: `cantBeBlockedExceptBy(this, …)`, a
     `byObjects` exception [CR#702.36b,702.13b,702.9b].
4. **Deviations and STOPs**:
   - **STOP, resolved by routing.** The ruling's general `devourQuality`
     declaration [CR#702.82c] cannot land as a `KeywordAbility`. Four standing
     contracts refuse it:
     - `deckmaste_construction_core` has no `[Quality, Amount]` keyword
       parameter class;
     - the construction-core test
       `builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`
       derives a keyword's name from its surface and allows only bare gift to
       be unspelled;
     - English has no line codec for that class;
     - facts has no `Shape` for it, the frozen Idris table included.

     Devour's body is the general definition, written as the Ability helper
     `devourQualityExpansion(quality, N)`; devour N is that over creatures.
     The declaration, its spelling and Feasting Hobbit are routed to the new
     ticket `planned/semantics-v2-devour-quality-keyword`.
   - Readbacks. `semantics-v2-linked-readback` landed during the refresh and
     named this ticket's follow-up. "It devoured" is written as
     `linkedObjects(ByKeyword(devour), OfAbility(devour), …)` and "tribute
     wasn't paid" as `not(choiceWasTaken(ByKeyword(tribute), …))`
     [CR#702.82b,702.104b]. Three changes make this possible:
     - devour and tribute are registered as Link Sources (`paid_cost: true` in
       the overlay);
     - devour gains an ability-deed row in `Check/Words.lean`, a permanent
       devouring permanents;
     - `NounPhrase.groupMention` counts an `objects` facet as a group, so "for
       each creature it devoured" counts [CR#607.2a].
   - Amplify got a body because `semantics-v2-sharing` landed. The revealed
     cards are read back as `them`, because `theVerbed(reveal, …)` stamps no
     patient (see ripple's STOP). Intimidate moves from sharing's negated
     predicate onto `byObjects`.
   - Evasion family. Only flying, fear and intimidate move onto `byObjects`,
     because the CR writes "except by" only for those three. Shadow and
     horsemanship keep "by creatures without …", as [CR#702.28b,702.31b]
     write it.
   - Ascend's permanent half is guarded by `onlyIfSo(…, matches(this,
     permanentType))`, as gift's is. Ascend and storied keep the "you don't
     have …" conjunct (owner ruling, 2026-10-09).
   - Naming. `Cause` is the ruling's name for the glossary's Deontic
     Exception. `except` is the last argument of `deonticRule`, and every
     existing Lean and RON site gains `none`.
   - Checker:
     - `asEvent` reads its instruction in `interceptCtx`; `whileHolds` reads
       it in the condition's context;
     - both publish choices to later lines (Callous Oppressor's "the chosen
       type");
     - `becomes` carries no zone in its binding;
     - `Card.lean` admits a replacement of the spell's own move off the stack
       on an instant or sorcery card;
     - `regimeFits` and xtask's `static_regime` leave that replacement's
       regime open.
   - Facts:
     - transform's patient role admits a permanent of any type
       [CR#701.27a];
     - warp, dash, blitz and rebound take `open_regimes: [(0, AtCasting)]`,
       so each row's existing `AtCasting` is its components' summary;
     - ascend's regime becomes `AtResolution` [CR#702.131a].
   - Found and fixed in code: the first xtask run failed
     `the_stack_column_is_the_grant_gate_and_the_regime_column_is_the_body_axis`,
     where rebound's regime had become `None`.
   - Helpers added: `asEvent`, `whileHolds`, `becomes`, `cantExcept`,
     `cantBeBlockedExceptBy`, `devourQualityExpansion` and `totalOf`.
   - Bench:
     - `flyingExpansion` is re-spelled onto the exception;
     - cumulative upkeep reads `permanent` (owner ruling, 2026-10-09);
     - `lean macros align` lists neither.
   - Firmament Sage's "day becomes night or night becomes day" is read as
     `gameBecomes`, as the bench's `firmamentSage` does. This omits the
     prior-state reading of [CR#731.1a].
   - `NounPhrase.resolvedPermanent` overlaps `becomes` and is kept.
   - Glossary: no term is missing.
   - Process: an early `pkill -f "lake build --wfail"` from this workspace may
     have stopped sibling workspaces' Lean builds.
5. **Tests**: all of the following pass.
   - `cargo test -p deckmaste_semantics_v2`: 47 + 7 + 1 + 4 + 45.
   - `cargo test -p deckmaste_construction_core`: 434 + 34.
   - `cargo test -p deckmaste_lexical_source`: 28 + 36.
   - `cargo test -p xtask`: 407 + 4 + 14, with 1 pre-existing ignore.
   - `cargo clippy -p <each> --tests -- -D warnings`: clean.
   - `lake build --wfail`: clean.
   - `lean macros check` and `facts check`: up to date.
   - `cite check`: 0 stale, 0 non-compliant. `cite bless` registered no new
     rule.

   Counts:
   - Restored: 0.
   - Re-spelled: 14 pin sites gained the `none` exception, with the same
     statements. These are 11 theorems in `Proofs/Deontic.lean`
     (`okStateBasedLossGate`, `okStateBasedLossForPlayerGroup`,
     `badStateBasedLossOnCreature`,
     `okTargetedContinuousStateBasedLossGate`,
     `badStateBasedCauseWithoutDeed`, `badStateBasedLossPatientRole`,
     `okPlayFromGraveyard`, `okPlayLandFromGraveyard`,
     `okCastCreatureFromGraveyard` and the two at lines 295 and 302) and the
     defs `awakenDefinition`, `castPermission` and
     `playableFromIsThePlayPermission`. Nine bench card sites were also
     re-spelled.
   - Ignored: 0.
   - Added: 25 pins in `Proofs/OffStack.lean`: 7 on the nodes (including
     `badBecomesOfAPlayer` = `[kindMismatch object player]`), 16 on the
     bodies and 2 on the readbacks.
   - Removed: 0.
6. **Timing**: no census run. `cargo xtask lean check` took 431 s;
   `cargo xtask lean definitions` took 40 s (711 definitions).
