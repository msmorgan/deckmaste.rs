---
needs: []
---
# A Gap passes through a child its Production does not mark transparent

## The defect

The syntax engine's contract says Gaps pass from a child to its mother only
through children the Production marks `transparent`
(`crates/deckmaste_syntax/README.md`). `take_gap` in
`crates/deckmaste_syntax/src/step.rs` enforces this: a `Closed` child accepts
only `Gap::None`. Even so, a Constituent can carry a Gap that no child its
Production marks transparent passed up to it.

`GerundSubject` (`english.ron`: [subject `Gerund`, head `VerbPhrase`]) marks
no child transparent. Yet its Clause carries Gap NP:

- **Panharmonicon** and its siblings, on the parent of `lxzvuwqu`: *If an
  artifact or creature entering causes a triggered ability of a permanent you
  control to trigger, …*.
  - Some Readings had a bare relative *creature [entering causes a triggered
    ability of a permanent you control to trigger]*.
  - Its Clause is `GerundSubject` with Gap NP. Its head Verb Phrase *causes …
    to trigger* also has Gap NP, which starts at the Object of monotransitive
    *trigger* inside *to trigger*.
  - 17 units carried such Readings: Panharmonicon, Traveling Chocobo, Ancient
    Greenwarden, Krang, the All-Powerful, Starfield Vocalist, Drivnod, Carnage
    Dominus, Elesh Norn, Mother of Machines, Gandalf the White, Teysa Karlov,
    Windcrag Siege, Isshin, Two Heavens as One, Felix Five-Boots, Yarok, the
    Desecrated, Wulfgar of Icewind Dale, Naban, Dean of Iteration, Wayta,
    Trainer Prodigy and Virtue of Knowledge.
  - `lxzvuwqu` removed them only incidentally: a bare relative's Clause is now
    `SubjectInitialClause`, whose productions are the Gap-passing
    subject-predicate ones.
- **Marked relatives, on `lxzvuwqu`:** `cargo xtask english-v4 parse` still
  gives a `GerundSubject` Clause with Gap NP under *that*. Examples: *an
  ability that entering causes a creature to trigger* and *an ability that
  entering causes to trigger*. These are probes, not card text. They show that
  the leak is in the engine or the production, not the bare relative.

## Change

- Find where the Gap enters the mother past a `Closed` child. Candidates:
  - a compiled or derived variant of the Production that marks its head
    transparent;
  - implicit head agreement treating the Gap as a feature;
  - Packing merging the summaries of gapped and gapless nodes.
- Fix it in the engine so that the contract holds. Do not add a grammar
  restriction for it.
- After the fix, a Constituent carries a Gap only if a child its Production
  marks `transparent` passed it up, or the Production introduces it with
  `Require("$.Gap", …)`.

## Done when

- No Reading on real text has a Constituent whose Gap did not come up through
  a transparent child or from its own Production.
- A test in `deckmaste_syntax` pins the contract: a non-transparent child with
  a Gap yields no mother.
- **No new features in `english.ron`.** If the fix needs one, STOP and return
  to the owner.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
