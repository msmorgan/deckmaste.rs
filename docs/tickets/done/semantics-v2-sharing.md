---
needs: []
---
**Sharing a value on an axis, as a predicate and as a group agreement.**
Ruling 2026-10-09 ("Sharing is at least one value in common on an axis",
`docs/decisions/semantics-v2.md` §7). Bodies intimidate, conspire's cost,
amplify and prowl. Standard constraints apply.

## Shape

- `Predicate.shares (axis : QualitySort) (agreement : Agreement) (with :
  NounPhrase)`, `Agreement = share | differ`: the member and `with` have at
  least one value in common on the axis (`share`) or none (`differ`)
  [CR#201.2a,201.2c,105.2,205.3m]. On the name axis `share` is "at least one
  name in common", never equality: an object with two names has the same
  name as an object with either one [CR#201.2a].
- `NounPhrase.agree (axis : QualitySort) (agreement : Agreement) (group :
  NounPhrase)`: the group-mutual form, a constraint on a selection ("two
  creatures that share a creature type"), not a predicate on one member.
- Checker: kinds only (`with` an object; `agree` over a plural group). It
  does not check that the axis is defined for the member's class.
- Rust mirror, RON helpers `shares` and `agree`, bench macros.

## Retires (re-spelled, same asserted outcome)

- `named(sameAs(X))` → `shares(cardName, share, X)`: every site in
  `lean/Semantics/Cards/` (Anaphora, Counters, Trigger, Cost, Keyword),
  `lean/Semantics/Proofs/` (Counters, Keyword, Zone, Piles), the bench and
  canon; 87 sites by the recon count, a mechanical re-spell.
  `NameSource.sameAs` goes if nothing else reads it.
- `namesAgree` (Phrase, `plugins_v2/builtin/macros/nouns/namesAgree.ron`,
  `Proofs/Zone.lean`) → `agree(cardName, share, …)`.
- `cardsShare (scope) (axis)` (Abilities, `Cards/Keyword.lean`,
  `Proofs/Keyword.lean`) → `agree` over the deck-scoped group.

## Bodies

- intimidate: can't be blocked except by artifact creatures and/or
  creatures `shares(color, share, it)` [CR#702.13b]. The "except" is
  `semantics-v2-off-stack-statics`' deontic `except : Option Cause`; if that
  has not landed, write intimidate as today's negated-predicate deontic over
  `or([artifactCreature, shares(color, share, it)])` and name the sibling in
  the landing record.
- conspire's cost: tap two untapped creatures you control that each
  `shares(color, share, thisSpell)` [CR#702.78a]. The copy half is
  `semantics-v2-spell-copies`'; if that has not landed, conspire stays
  bodyless with its STOP naming it.
- amplify: reveal any number of cards from hand `shares(subtype(creature),
  share, it)` [CR#702.38a], inside `semantics-v2-off-stack-statics`' entry
  clause `asEvent(enters(this), …)`. Body it here only if that ticket has
  landed; otherwise leave the STOP naming it.
- prowl: `happened(dealtCombatDamage(a(and([shares(subtype(creature), share,
  thisSpell), youControl])), a(player)), within(thisTurn))` [CR#702.76a];
  the creature's types are last-known information, the engine's
  [CR#608.2h].

## Witnesses (canon)

A "shares a creature type" non-keyword card, a "different name" (`differ`)
card [CR#201.2c], a "two creatures that share a color" card (`agree`), and a
canon card for each keyword bodied. Pins: `shares` on the name axis is not
equality (a two-named object matches either name); `agree` over one object is
refused as a plurality error.

## Siblings

Cross-references `semantics-v2-off-stack-statics` (except, entry clause)
and `semantics-v2-spell-copies` (conspire's copy); neither blocks. Refresh
before integrate; `Words`/`Phrase`/`Abilities` conflicts are expected.

## Landing record

Measured on change `xknwtpxs` (series `zpsxwqyp`..`xknwtpxs`, after the refresh that brought in `semantics-v2-modal-choice`).

1. **Census delta**: no English grammar, lexicon or spelling change, so no
   English v3 census was run; covered faces and Readings are unchanged. Lost
   faces: none. Lean: `lean check` 191/191 cards prove `Card.check = []`
   (canon 183, six of them this landing's witnesses; testing 8).
2. **Witnesses**: Bladetusk Boar (intimidate), Latchkey Faerie (prowl),
   Wandering Graybeard (`shares(Subtype(Creature), Share, thisCreature)`),
   Weight of Conscience (`agree(Subtype(Creature), Share, …)`, "two creatures
   that share a creature type"), Illuminated Folio (`agree(Color, Share, …)`,
   "two cards that share a color"), Eerie Ultimatum (`withDifferentNames`, now
   `agree(CardName, Differ, …)`): each proves `Card.check = []`, one Reading
   each (semantic cards, not English census faces). Pins in
   `Semantics.Proofs.Sharing`: name sharing is not equality
   (`twoNamesShareEither`, `namelessSharesNoName`), `agree` over one object is
   `[.plural]` (`badOneCreatureAgrees`).
3. **Spot checks**: intimidate =
   `cantBeBlockedBy(thisCreature, and([creature, not(or([artifact, shares(Color, Share, thisCreature)]))]))`;
   prowl = `onlyWhile(mayCastFor(you, cost), happened(damage(CombatOnly, gap(Player), a(and([shares(Subtype(Creature), Share, thisSpell), youControl]))), a(anyPlayer), soFarThisTurn))`
   (control and types at damage time are LKI, the engine's [CR#608.2h]);
   Weight of Conscience cost = `perform(tap(agree(Subtype(Creature), Share, counted(exactly(2), and([untapped, creature, youControl])))))`;
   Illuminated Folio cost reveals `agree(Color, Share, counted(exactly(2), cardIn(yourHand)))`;
   Graybeard reveals only if `matches(it, shares(Subtype(Creature), Share, thisCreature))`;
   Eerie Ultimatum returns `agree(CardName, Differ, counted(anyNumber, …))`;
   Lutri = `groupAgrees(agree(cardName, differ, allOf(nonland card)))`;
   Umori = `groupAgrees(agree(cardType, share, allOf(nonland card)))`;
   the seven former `named(sameAs(X))` bench sites read `shares(cardName, share, X)`;
   conspire's cost pin `okConspireCost` checks clean. No wrong analysis found.
4. **Deviations and STOPs**:
   - The ticket's recon count of 87 `named(sameAs)` sites was wrong: there
     were 7 (Cards/Anaphora 4, Cards/Cost 1, Cards/Trigger 1, Proofs/Piles 1),
     all re-spelled. The Counters/Keyword hits are `CounterKindSource.sameAs`
     ("the same kind of counter"), a different node, left alone.
     `NameSource.sameAs` retired (nothing else read it).
   - `cardsDiffer` retired with `cardsShare` (its `differ` twin; leaving it
     would keep a second agreement spelling). Both become
     `DeckCondition.groupAgrees (group : NounPhrase)` carrying a
     `NounPhrase.agree` over the starting deck's cards; the checker reads the
     axis and description from that agreement (same `deckReadable` /
     `deckComparable` refusals as before) and refuses any other group as
     `deckReadable`.
   - `Agreement.holdsOn` added in `Words.lean`: the value-set reading of
     share/differ the pins evaluate.
   - Field `with_` (Lean) / `with` (Rust): `"with"` added to
     `LEAN_ESCAPED`.
   - `Predicate.qualityReadOk` accepts `shares`, as it accepted
     `named(sameAs …)`.
   - Facts: prowl's overlay gains `open_regimes: [(0, AtCasting)]` like
     surge/spectacle; regenerated rows give intimidate and prowl their
     static definitions.
   - Intimidate written with fear's negated-predicate form; its `except` slot
     waits on `semantics-v2-off-stack-statics`. Amplify stays bodyless,
     STOP now naming only `semantics-v2-off-stack-statics` (entry clause).
     Conspire stays bodyless per the ticket, STOP naming only
     `semantics-v2-spell-copies`; its cost half is written and pinned
     (`okConspireCost`).
   - Eerie Ultimatum is a group "different names" card [CR#201.2b], not a
     single-object 201.2c card: no single-object differ card was writable
     with current helpers.
   - The regenerated `Declared.lean` also picked up trunk's stale
     `typecycling` entry. No STOPs; no glossary gap (Sharing, Agreement and
     Deontic Exception already defined).
5. **Tests**: `cargo test -p deckmaste_semantics_v2` pass (45+7+1+4+44);
   `cargo test -p xtask` pass (398 passed, 1 ignored pre-existing, +4, +14);
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` and
   `cargo clippy -p xtask --tests -- -D warnings` clean; `lake build --wfail`
   clean; `cargo xtask lean check` 191/191; `cargo xtask lean definitions`
   708/708; `lean macros check`, `facts check` up to date; `cite check` 0
   stale, 0 non-compliant. Counts: restored 0; re-spelled 6 theorems
   (Zone `okPluralNameAgreement`, `badSingularNameAgreement`; Keyword
   `okCompanionSharedCardType`, `okCompanionSharedColor`,
   `badCompanionSharedCounterKind`; the Piles theorem over `.named (.sameAs
   .this)`) plus 8 bench definitions (Anaphora 4, Cost 1, Trigger 1, Lutri,
   Umori), same outcomes; ignored 0; added 8 theorems (`Proofs/Sharing.lean`)
   and 6 canon cards; removed 0.
6. **Timing**: no census run; `cargo xtask lean check` wall 184 s (post-refresh
   run, lake default workers).
