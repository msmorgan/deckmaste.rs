---
needs: []
---
# Checker rules that refuse or misplace what the CR allows

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#43, #44, #49, #50, #51, #53, #55. Verified against head 2026-10-21. Each is a checker gate
narrower than its cited rule (a false refusal) or a zone default the rule does not give. Latent:
no lowering consumes the checker yet. Fix the gate, never the card, and add a witness the old
gate refused.

Boundary: `docs/decisions/semantics-v3.md` §0.1 item 4 (the checker models text meaning and rule data; runtime state is the engine's); triage in
[lean-v3-checker-runtime-state-audit](lean-v3-checker-runtime-state-audit.md).

## 1. Text changes reach creature types

`QualitySort.replaceable` (`Check/TextChanges.lean:11-13`) admits only `.color | .subtype .land`,
so `replaceWord [.subtype .creature]` is refused and *Artificial Evolution* and *New Blood* cannot
be written. [CR#612.2] lists "a creature type word used as a creature type" among its examples
(the list is "for example", not complete); [CR#612.2a] covers creature types used in token names.
`TextWord.basicLandType` (`Abilities.lean:29-35`) already takes any `Subtype`, so no new
constructor is needed: add `.subtype .creature` to `replaceable`, rename `basicLandType` (it is not
limited to basic land types), and restate the docstring as "words used in their rules sense, such
as color words, land types and creature types [CR#612.2,612.2a]". Mirror the doc in
`crates/deckmaste_semantics_v2/src/abilities.rs:145-151` and the glossary **Text Change** entry
(`docs/contexts/game-model/CONTEXT.md:593`, via `domain-modeling`). The refusal constructor reused
at `:28-29` is [lean-v3-checker-and-drt-fixes](lean-v3-checker-and-drt-fixes.md) §3's; land either
first.

## 2. "Exiled with" is not face down by default

`Location.exiledWith`'s docstring (`Abilities.lean:102-103`, also `Templates.lean:242-244`) says
"The face-down cards exiled with `source` … [CR#406.4]", and `Location.face` returns `.faceDown`
for it (`Check/AbilityRules.lean:113`). "Exiled cards are, by default, kept face up" [CR#406.3];
"exiled with" is defined by the linked ability [CR#406.6,607.2a]. Face-up sites at head:
`Cards/Cost.lean:71,76,85,724,836`, `Cards/Mana.lean:1081,1099`, `Cards/Keyword.lean:281`,
`Cards/Counters.lean:796`, `Cards/Description.lean:402`. Return `none` for `.exiledWith _` (or take
the face from the linked instruction), reword both docstrings with [CR#406.6,607.2a], and fix
`zonePilesFixTheirFaces` (`Proofs/Piles.lean:134-138`).

## 3. A spell card's self-cast flash permission

`onInstantOrSorceryCardOk` (`Check/Card.lean:32-50`) ends `| _, _ => false`, so `.can`/`.gatedBy`
statics are refused on instant and sorcery cards, and `badCastWindowAsDeonticStatic`
(`Proofs/Faces.lean:80-90`) says a cast window is "not a permission a spell card's static ability
grants". *Ghitu Fire* is refused. An ability that modifies how its own spell is cast "functions in
any zone from which it could be … cast and also on the stack" [CR#113.6e]; flash conditioned on an
added cost is [CR#601.3c]; only "may be cast 'only [before/after]'" is [CR#506.8]. Admit a self-cast
`.can`/`.gatedBy` whose `asThough` is flash with an added-cost gate. No such Template exists: promote
the card-local `asThoughFlash` (`Cards/Choice.lean:536`) to one. Narrow the Faces.lean docstring to
[CR#506.8] restrictions and add a *Ghitu Fire* pin. This is opus-findings §2.10, which
lean-v3-checker-and-drt-fixes's header lists but its body does not carry.

## 4. Linked referents inside a created token's or granted text

`choiceContext` (`Check/Choices.lean:6-12`) carries only chosen qualities and chosen players into
embedded text (`Check/Phrase.lean:1410,1433,1785,1839`), so "the exiled card" in a token's text is
refused `participialDescription` and *Hofri Ghostforge* and *Timothar, Baron of Bats* cannot be
written. Abilities on a token and on "the source of the ability that … created the token" can be
linked [CR#607.1d]; a granted ability counts as printed on the object [CR#607.1a]; "the exiled
cards" is a linking phrase [CR#607.2a]. When stepping text that the creating ability puts on or
grants to its token or permanent, carry in that ability's linked object referents (not pronouns).
Pin *Hofri* and *Timothar*; check emblems the same way.

## 5. Keyword card-class gates that the CR doesn't impose

`Check/Facts.lean:161,166` set replicate and split second `onPermanentCard := false`; `:267-268`
leave web-slinging and wither at the default `onInstantOrSorceryCard := false`
(`Check/FactTypes.lean:70-71`). The source rows are `crates/xtask/src/facts.rs:747-751,797-800,
1322-1332`, whose comment at `:829-832` says the gates come "from the entry's own card-type
wording". [CR#702.56a,702.61a,702.188a,702.80a] impose no card class, and wither works "no matter
what zone" [CR#702.80c]. Oracle has *Sulfur Elemental*, *Hatchery Sliver*, *Changing Loyalty*,
*Puncture Blast*. Set `on_permanent_card: true` for SplitSecond and Replicate and `on_spell_card:
true` for WebSlinging and Wither, regenerate `Facts.lean`, and correct the comment: class gates are
corpus observations, not refusals, even where a CR "appears on" sentence exists.

Removing these gates agrees with the boundary: they police no text meaning.

## 6. A revealed cost result is readable in place

`publicCostResult` (`Check/Phrase.lean:1910-1914`) admits only referents in a public zone
(`Zone.isPublic` is false for `.hand`, `Check/Words.lean:948-957`), so "the revealed card's power"
fails for a card revealed from hand (*Charge of the Forever-Beast*, *Calamity of the Titans*,
*Disaster Radius*). Revealing "doesn't cause it to leave the zone it's in" [CR#701.20b] and the card
"remains revealed" while needed [CR#701.20a]; [CR#400.7j]'s public-zone condition is for costs that
move objects. Also admit referents acted on in place (`Stamp.moved = false`, `Words.lean:253`) and
keep the public-zone requirement for moved ones. Cite [CR#701.20a,701.20b,400.7j] and the *Charge of
the Forever-Beast* ruling (2020-04-17), not [CR#608.2h]. Pin one of the three cards.

**Superseded by Decision 3 (owner, 2026-10-22).** "Keep the public-zone requirement for moved ones"
no longer holds: [lean-v3-ninjutsu-defender-lki](lean-v3-ninjutsu-defender-lki.md) removes the
public-zone filter from `publicCostResult` and `linkedResult` altogether. Of this item only the
*Charge of the Forever-Beast* pin remains.

## 7. "Enchanted creature card" is not on the battlefield

`attachHostZone` (`Check/Words.lean:1618-1622`) maps `.card` and `.spell` heads to `some
.battlefield`; consumed at `Check/Phrase.lean:2561-2563`. The battlefield default applies only to
descriptions that don't "include the word 'card,' 'spell,' 'source,' or 'scheme'" [CR#109.2]; a
spell is on the stack [CR#109.2b]. *Animate Dead* and *Spellweaver Volute* enchant graveyard cards
(Volute ruling, 2007-05-01: "The enchanted card won't be on the battlefield"). Return `none` (or the
enchant ability's zone) for `.card` and `.spell` heads, and add an *Animate Dead* or *Volute*
witness.

## Verification

`./lean/scripts/build`; `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings`
(facts regeneration); `cargo test -p deckmaste_semantics_v2` if its doc mirror changes; the CR
citation gates.
