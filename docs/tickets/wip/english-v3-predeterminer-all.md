---
needs: []
---
# Read *all* as a Predeterminer of a definite Noun Phrase: Discard all the cards in your hand, then draw that many cards.

## Why

*all cards in your hand* and *the cards in your hand* each read; *all the
cards in your hand* does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:all the` 17 sole-blocker
faces. `cargo xtask english-v3 grep -i '\ball the\b'` finds **24** faces,
all unread; **21** have that line as their only failing line. By shape:
*discard(s) all the cards in your/their hand* 19 (two inside modal bullets);
*exile / put all the cards from your hand / library / graveyard* 4; *win all
the flips* 1 (Squee's Revenge). 4 of the 21 have a second cause (*that many
cards plus / minus one*: `english-v3-degree-quantity-arithmetic`). *all* +
demonstrative or genitive is nearly absent (`\ball (those|these|your|their|
its|that)\b`: 1 face) and *both the* has 0.

## Goal

*all* before a definite Noun Phrase (*all the cards in your hand*) reads as
a Predeterminer, an external Modifier of that Noun Phrase, with byte-exact
realisation and plural agreement from the host.

## Analysis

CGEL Ch. 5 §12, pp. 433–434: predeterminers "enter into construction with an
NP, not a nominal"; [1i] "*She had lost* [*all her money*]" labelled
[universal quantifier]; [2i] "*all the books/sugar/morning*, *all these
books*, *all that work*, *all Kim's friends*"; "*All* and *both* occur before
the definite determiners: the definite article, the demonstratives, and
genitives." And p. 434: "in each pair *all* is determiner in the version
without *the* and predeterminer in the one with *the*." Those back *all* as
Predeterminer before a definite NP and as Determiner otherwise.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`PredeterminerNounPhrase` (`[modifier: lexical(Adverb), " ", head:
NounPhrase]` and `[modifier: lexical(Noun), " ", head: NounPhrase]`,
`require predeterminer_host(modifier.PredeterminerUse, head.Definiteness) =
Yes`), feature `PredeterminerUse { No, Multiplier, Fraction }`, table
`predeterminer_host` (`(Multiplier, Definite)`, `(Fraction, Definite)`).
*all* is `vocab:FloatedQuantifier/All` (`lexicon/core.ron`, `DeterminerUse:
PluralOrMass`), a Determinative. Glossary: **Predeterminer**,
**Predeterminer Use**.

Orchestrator rulings 2026-10-10:

- PA1. Refine `PredeterminerNounPhrase`: one more form with a Determinative
  modifier, and one more `PredeterminerUse` value for the universal
  quantifier with a `predeterminer_host` row for a Definite host. *all*
  declares that value (data). No new Construction.
- PA2. The licence is the declared `PredeterminerUse` and the host's
  `Definiteness`; no test on *all* or *the*. Demonstrative and genitive
  hosts follow from `Definiteness` if they export Definite; do not add a
  check to block them, and do not declare *both* (0 faces).
- PA3. Number: the host NP's number is exported, as for the other
  Predeterminers, and the host must be compatible with *all*'s declared
  `DeterminerUse` (plural or mass), read from the features both already
  carry. *all the card* need not read and is not blocked by a named guard.
- PA4. One analysis per string. The twins to avoid: (a) *all cards in your
  hand* keeps exactly its Determiner Reading (CGEL's "determiner in the
  version without *the*"); it gains no Predeterminer Reading over a bare
  plural; (b) *all the cards* does not also read with *all* as Determiner of
  a nominal *the cards*, or as a floated quantifier; (c) *all other
  creatures*, *all three* keep their Readings (CGEL p. 434 takes *all* as
  Determiner before a cardinal); (d) Multiplier and Fraction Predeterminers
  (*twice that many*, *half the …*) do not stack with *all*.
- PA5. *all of the cards* (partitive) is a different Construction and stays
  out.
- PA6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *all the*, and PA4(a) forbids a
  second Reading of *all* + bare plural (1,767 faces contain *all*). Report
  any increase with its shape before landing.

Owner amendment (2026-10-10): licence the natural universal-quantifier class
of existing Determinatives, *all* and *both*. This overrides PA2's exclusion
of *both* and the original scope below; no new Lexemes. CGEL Ch. 5 §12(a),
pp. 433–434 is the page-checked passage (the owner's §11 locator was not
the predeterminer section). W9-A means one route for this construction;
independent ambiguity shared with the without-predeterminer control stays.
PA4 and PA6 remain binding. The census selector includes every *all* and
*both* face and uses the owner's seed 20261008.

PA7 (orchestrator ruling, 2026-10-10) amends PA6: the eight increases
from bare-plural genitives are permitted. *all opponents' graveyards* has
both `[all [opponents' graveyards]]` and `[[all opponents]' graveyards]`;
both grammatical structures are retained. No host feature is introduced to
prune the genitive-determined definite Noun Phrase. This resolves the
verification STOP. Each increase must add exactly that structure, multiplied
only by independent ambiguity; no other increase shape is allowed.
Demonstrative and possessive-pronoun hosts use the same Definiteness licence.
CGEL Ch. 5 §12(a), pp. 433–434 explicitly places universal-quantifier
Predeterminers before the definite article, demonstratives and genitives;
its examples include *all her money*, *all these books* and *all Kim's
friends*. PA7's preservation of the two genitive attachments is the
orchestrator ruling, rather than a claim that those examples themselves
establish this attachment ambiguity.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Tolarian Winds: "Discard all the cards in your hand, then draw that many
  cards." (control "Discard all cards in your hand, then draw that many
  cards." 3+; "Discard the cards in your hand, …" 3+)
- Awaken the Erstwhile: "Each player discards all the cards in their hand,
  then creates that many 2/2 black Zombie creature tokens."
- Robber Fly: "Whenever this creature becomes blocked, defending player
  discards all the cards in their hand, then draws that many cards."
- Endurance: "When this creature enters, up to one target player puts all
  the cards from their graveyard on the bottom of their library in a random
  order." (may have a second cause; report)
- Forgotten Creation, first sentence: "At the beginning of your upkeep, you
  may discard all the cards in your hand." ("You may discard all the cards
  in your hand." 0)

Also 0 today (`noun-phrase`): "all the cards in your hand"; "all those
cards"; "both the cards". Controls that must not change (`noun-phrase`):
"all cards in your hand" 1; "the cards in your hand" 1; "all other
creatures" 1; sentence "Each player discards the cards in their hand, then
draws that many cards." 2. The without-*the* controls for Awaken the
Erstwhile, Robber Fly and Endurance were not probed (the trunk moved under
the prebuilt binary); probe them on the claim parent.

## Method

Standard constraints apply. Bounded census: every supported face matching
`(?i)\b(all|both)\b` (1,879 faces on claim parent `wytzznnu`; each gains a
declared use, so every face with either word is in the selector) plus 300
random covered controls (`--random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; PA6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, once,
at the end: a rebuild after a grammar edit takes about 2–3 minutes and the
six test binaries about 75 seconds, so batch edits; lexicon data loads at
runtime. Tests assert the role (*all* as Predeterminer of the definite NP,
the host's number exported, the PA4 negatives), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: extend **Predeterminer** and **Predeterminer Use** with the
universal quantifier (CGEL Ch. 5 §12(a), pp. 433–434) in the landing.

## Out of scope

- *that many cards plus one* / *minus one* after the discard
  (`english-v3-degree-quantity-arithmetic`).
- *all of the …* partitives.
- *win all the flips* beyond what PA1 gives (Squee's Revenge has other
  unread material).

## Landing record

1. **Census delta** (`xswzoyrx`; BEFORE measured while empty on claim parent
   `wytzznnu`, AFTER measured with this implementation): selector
   `(?i)\b(all|both)\b` finds 1,879 faces; the manifest unions them with
   300 covered controls selected by `english-v3 diff --manifest --random 300
   --seed 20261008` from `baseline-wave8.json`, giving 2,163 distinct faces.
   Covered 1,862 → 1,878; total Readings 58,675 → 58,992. Lost: none;
   decreases: none; digest-only: none; gained: 16; issues: zero. Eight
   covered-face increases are permitted by PA7:
   Ashiok, Nightmare Weaver 51 → 57; Author of Shadows 24 → 48;
   Kaya's Guile 7 → 13; Mnemonic Betrayal 16 → 31;
   Multani, Maro-Sorcerer 8 → 15; Phyrexian Scriptures 5 → 6;
   Realm Seekers 20 → 34; Werewolf of Ancient Hunger 9 → 16.
   Each adds `[all [opponents' graveyards]]` or `[all [players' hands]]`
   as an outer Predeterminer, alongside the existing route with *all*
   determining the possessor.
   Affected line counts double, with other lines unchanged: Ashiok 6 → 12,
   Author 24 → 48, Kaya 6 → 12, Mnemonic 15 → 30, Multani 7 → 14,
   Phyrexian 1 → 2, Realm Seekers 14 → 28, Werewolf 7 → 14.
   Complete enumeration confirms Author's 48 Document Readings are
   24 old possessor / 24 new outer-Predeterminer analyses, and Multani's
   14 Document Readings are 7 / 7 (the census also counts its Shroud line).
   Each has exactly one NP shape per route and identical surrounding
   Reading choices. Evidence: `pa7-author-of-shadows.json`,
   `pa7-multani.json`, `pa7-enumeration-shapes.json` under the census directory.
   No other increase shape occurs. Supported-corpus grep of *all* plus
   demonstrative/possessive-pronoun determiners finds zero covered matches
   in BEFORE; the sole corpus match is Demonic Covenant's *all their card
   types*, an unread → covered gain. There is therefore no previously
   covered face of those shapes whose count could change; *all those cards*
   and *all your creatures* each read once in direct probes and independent
   roundtrip tests via the same definite-host licence.
   The built-in diff, including `--lines`, is
   `target/english-v3/english-v3-predeterminer-all/diff.json`.
   Post-refresh census at `lmprkvko` reuses that manifest and has the same
   totals and deltas against BEFORE (`diff-refresh.json`). The built-in
   AFTER → refreshed-AFTER comparison (`refresh-diff.json`) reports all
   2,163 faces unchanged: zero lost/gained/decreases/increases/digest-only.
   Integration refresh at `lytultmw` brought degree arithmetic `lvsykyns`.
   Its bounded re-census retains 16 gains, eight PA7 increases, zero lost or
   decreased faces and the same coverage/Reading totals. Compared with the
   reviewed AFTER, it changes only 48 digests, attributed to `lvsykyns`'s
   Cardinal feature exports and optional ArithmeticModifier slot. Compared
   with claim-parent BEFORE, 34 of those are digest-only changes (the other
   14 are already listed as gains). Final reports:
   `diff-integrate-refresh.json` and `integrate-refresh-diff.json`.
   Integration was then refused because default moved. The next refresh at
   `rvknxrlq` brought property-pair grammar `svlsnsmv`: Gideon, Champion of
   Justice 0 → 202 and Spark Rupture 0 → 33 are its two gains. The built-in
   refresh comparison reports no other changes. Final claim-parent delta:
   18 gains (16 this ticket, two `svlsnsmv`), eight PA7 increases, zero lost
   or decreased faces, 34 digest-only changes attributed to `lvsykyns`;
   covered 1,862 → 1,880 and Readings 58,675 → 59,227. Final evidence is
   `diff-integrate-refresh-2.json` and `integrate-refresh-2-diff.json`.

2. **Witnesses** (`xswzoyrx`, before/after, complete direct-probe counts):
   Tolarian Winds 0 → 4 (bare/definite controls 4/4);
   Awaken the Erstwhile 0 → 2 (2/2); Robber Fly 0 → 2 (2/2);
   Endurance 0 → 1 (1/1, no second blocker in this sentence);
   Forgotten Creation's first sentence 0 → 3 (3/3).
   Auxiliary modal witness 0 → 3. Noun Phrase witnesses
   *all the cards in your hand*, *all those cards*, *both the cards*
   and owner-class *both the creatures* each 0 → 1.
   Controls *all cards in your hand*, *the cards in your hand*,
   *all other creatures* each stay at 1, and the Each-player sentence
   stays at 2. Bare incomplete *all three* is 0 on the claim parent;
   tests preserve the complete constituent *all three cards*.
   W9-A independent attachment ambiguity is retained.

3. **Spot checks** (`xswzoyrx`, ten gained faces): Awaken the Erstwhile,
   Barbed Shocker, Book Devourer, Collective Defiance, Decaying Time Loop,
   Endurance, Fateful Showdown, Forgotten Creation and Hex Magic each
   use *all* as an external Predeterminer of a *the cards …* Noun Phrase;
   Demonic Covenant uses it over possessive *their card types*.
   Up to three Reading trees per face were inspected in
   `target/english-v3/english-v3-predeterminer-all/spot-checks.json`.
   No unintended analysis found among those samples; the separate covered
   genitive increases in item 1 are not counted as gains.

4. **Deviations and STOPs** (`xswzoyrx`): owner's class ruling includes
   existing *both*, overriding PA2 and the original scope. No Construction,
   Lexeme or frame added/deleted; refine the existing Predeterminer form
   and feature tables, including host Number/Countability and structural
   stacking constraints for PA3/PA4. No superseded route was retired.
   Glossary extends Predeterminer and Predeterminer Use; CGEL page-check
   confirms Ch. 5 §12(a), pp. 433–434. The genitive STOP is resolved by PA7,
   which amends PA6 and preserves both grammatical attachments. No
   genitive-blocking feature added. Implementation committed as `xswzoyrx`;
   refresh succeeded after retrying a concurrent snapshot refusal. Refresh
   brought the Devour Quality landing: English grammar `vuyokrvx`,
   semantics/plugin/facts `ulkrnzyt`, and canon Feasting Hobbit `pknkolqq`.
   Incoming stat is saved in `refresh-stat.txt`; the refreshed base is
   `vwktxkol`. Dependent Rust and plugin changes triggered the standing
   post-refresh build, bounded census, full suites, clippy and Lean check.
   Before acceptance, integration and workspace removal were reserved for
   the orchestrator.
   Orchestrator accepted the landing and authorized integration and removal;
   the subsequent refresh brought only degree-arithmetic crate changes and
   docs/tickets changes, with no plugin changes.

5. **Tests** (`xswzoyrx`): `cargo test -p deckmaste_english_v3` passed
   (1,046 tests), `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed; `cargo test -p deckmaste_lexical_source` passed (88 tests),
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored 0 / re-spelled 0 / ignored 0 / added 9 / removed 0; no removals.
   New module registered exactly once in `tests/main.rs`; tests assert
   external Modifier role, original host Reading sets, independent
   roundtrips, declared Number/Countability, plural finite agreement and
   PA4 exclusion of stacking/twins. Citation check: 0 noncompliant,
   0 stale; diff audit contains no newly cited CR rules. No plugin changes
   in this implementation. Post-refresh verification at `lmprkvko` passed
   `cargo build -p xtask`, both crate suites and both clippy commands above
   again (English 1,047 tests, including the incoming Devour test; lexical
   source 88). Incoming plugin changes also required, and passed:
   `cargo test -p xtask`;
   `cargo clippy -p xtask --tests -- -D warnings`;
   `cargo test -p deckmaste_semantics_v2`;
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings`;
   `cargo test -p deckmaste_construction_core`;
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`;
   `cargo xtask lean check` (243/243 cards). All suites were full, without
   name filters. Exact commands, exit statuses and logs are recorded in
   `post-refresh-checks.json`; build and Lean logs are adjacent. Refreshed
   citation checks report 0 noncompliant and 0 stale among 17,763 citations.
   After acceptance, the integration refresh again passed `cargo build -p
   xtask`, full `cargo test -p deckmaste_english_v3` and `cargo test -p
   deckmaste_lexical_source`, and `cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source
   --tests -- -D warnings`. Exact commands and zero exit statuses are in
   `integrate-refresh-checks.json`, with adjacent logs (English 1,052 tests).
   After the second integration refresh, the same build, bounded census,
   full English and lexical-source suites, and both clippy commands passed
   again; exact commands and zero statuses: `integrate-refresh-2-checks.json`.

6. **Timing** (`xswzoyrx`): bounded AFTER census 49.42 s wall time under
   `/usr/bin/time -v`, six workers, zero samples per face; post-refresh
   bounded census 126.79 s wall time, and integration-refresh bounded census
   52.47 s, under the same settings. No full-corpus
   census was run, per the binding verification budget.
