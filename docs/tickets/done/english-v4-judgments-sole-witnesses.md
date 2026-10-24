---
needs: []
---
# Judgments: resolve the `# kept` sole-witness entries

Pass 3 of `english-v4-judgments-source-citations` kept 42 synthetic `+` entries marked `# kept`, each the only judgment that exercised some Production (55 Productions in all). The owner ruled: measure which of those Productions any corpus unit uses; a Production a card uses is judged with that card's real text; a schema-generated cell no card uses is left unjudged (the schema stays, Principle 1); a hand-written named Production no card uses is deleted unless it completes a closed inflectional paradigm beside attested siblings; and the `# kept` rule leaves the README.

## Measure

A temporary probe (not committed) parsed every corpus unit and walked each Packed Forest from its roots, so a Production counts as used when any Derivation of the unit contains it, not only the cheapest Reading. It also listed, for every judgment, the Productions of its endorsed Readings. Only two Productions of the kept set occur in any corpus unit: `Coordination:VerbPhrase/shared-bareinf` and `Valence:VerbPhrase/Catenative(BareInf)/gap0`, both in Gilded Drake (*If you don't or can't make an exchange, ...*). The other 53 occur in no unit, read or unread.

## Resolution, per file

- `shared_heads.txt`: *may and can draw cards* replaced by `# from: Gilded Drake` `+VerbPhrase don't or can't make an exchange`, same `Object of` assertion, exercising both used Productions. *be and be drawn* (`shared-pred`) and *choose and choose to attack* (`shared-toinf`, `Catenative(ToInf)/gap0`) retired: schema cells, no card.
- `correlatives.txt`: 25 retired. They are the `Both`/`Either`/`Neither` correlative rows (binary-coord, serial-coord and the `serial-end`, `Nominal#Disjunctive` and `NounPhrase#Alternative` cells) expanded across the Coordination schemas for Clause, VerbPhrase, PrepositionPhrase, AdverbPhrase, KeywordPhrase and Nominal/NounPhrase. The mechanism stays witnessed by the three real entries in the file (*both blue and red*, *neither day nor night*, *both own and control this land*).
- `correlative_adjectives.txt`: all 4 entries retired (AdjectivePhrase binary-coord and serial-coord, `or`/`nor`); the file is deleted.
- `phrase_coordination.txt`: *again and again* and *again, again, and again* retired (AdverbPhrase `binary`, `serial`, `serial-end`).
- `counter_kind_specifiers.txt`: *a +1/+1 or flying counter* retired (`CompoundCoordination#Coordinator/binary`); the other entries witness Word-Part Coordination.
- `shared_preposition_heads.txt`: *over or under cards* retired (`PrepositionPhrase/shared`).
- `postposed_heavy_object.txt`: 2 retired (`TransitiveWithPP` shifted-object cells for `into` and for `to` with `prefix1/more`); the heavy-shift mechanism is witnessed by the other entries.
- Hand-written Productions in `english.ron`, with their judgments:
  - `PredeterminerMass` deleted (*all the damage*): no card prints a mass-noun predeterminer phrase; `PredeterminerCount` stays (50 units). The `*` entries on *both the damage* and *all the creatures gets* still hold.
  - `SingularOrQuantifiedDetermination` deleted (*another two creatures*): no card prints it. The `*Clause another two creatures attacks` entry is retired with it, because with no Reading it would hold vacuously, and the README sentences about it are removed. `SingularOrQuantified` stays a Determinative Distribution value in the lexicon.
  - `ProximateExistentialBe` deleted (*there is an instant card, a land card, and two sorcery cards*); `LocatedProximateExistentialBe` (used by a card) stays and its comment still describes it.
  - `StrandedPerfect` kept without a judgment: with `StrandedModal` (35 units) and `StrandedBe` (1) it completes the stranded-auxiliary paradigm (Plain, past participle, gerund or past participle).
- `judgments/README.md`: the `# kept` exception is removed.

## Acceptance

Deleting the three Productions changes no reading of any unit (census Readings and Surplus identical).

## Landing note

1. **Census delta** (rpyvvkst): faces read 32825 -> 32825 of 32828, Readings 340496 -> 340496, Surplus 144096 -> 144096; lost faces: none; no unit changes verdict.
2. **Judgment set** (rpyvvkst): all enforced judgments hold; `cargo test -p deckmaste_english_v4` and clippy pass.
