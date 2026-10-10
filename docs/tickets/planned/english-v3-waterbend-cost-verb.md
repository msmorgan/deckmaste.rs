---
needs: []
---
# Declare *waterbend* as a verb taking a cost: Waterbend {3}: This creature can't be blocked this turn.

A data ticket on one plugin definition. No `declarations.rs` edit is
expected.

## Why

*waterbend* is a keyword action whose Complement is a mana cost. Its owning
definition exists (`plugins_v2/builtin/macros/keyword_actions/waterbend.ron`,
`params: [Cost]`, `spelling: "waterbend <Param(0)>"`) but exports
`grammar: FixedTerm(surface: "waterbend")`, so no verb reads. Blocker report
at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:<start>
waterbend` 14, failing token `waterbend` 27.
`cargo xtask english-v3 grep -i '\bwaterbend'` finds **29** faces, all
unread, **29** with that line as their only failing line. By position:
cost of an activated ability *Waterbend {N}: …* 16 (2 after *Exhaust —*);
*As an additional cost to cast this spell, (you may) waterbend {N}.* 9;
*unless you waterbend {2}* 1; *Ward—Waterbend {4}.* 1; bare *Whenever you
waterbend, earthbend, firebend, or airbend* 1; gerund-participle *by
waterbending {X} rather than paying its mana cost* 1 (Hama, the
Bloodbender).

The sibling actions already export verbs: `earthbend.ron` (`Verb(bare:
"earthbend", frame_set: MeasureComplement)`, 35 covered lines) and
`airbend.ron` (`Verb(bare: "airbend", frame_set: Transitive)`).

## Goal

*waterbend* reads as a verb with a cost-symbol Complement wherever *pay
{N}* reads today: as a Cost component, as the imperative or *you may*
Predicate of an additional-cost sentence, under *unless you*, and after a
keyword dash.

## Analysis

No CGEL claim: a game term's verb export and its Complement class are
ownership and declaration (owner ruling V3 of
`docs/tickets/done/english-v3-vocabulary-batch-1.md`: game terms are declared
through the owning plugin definition's `grammar:` field, never in core).

Nearest existing declarations: `core-verb:Pay`
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`, frames
`Predicate([ObjectNounPhrase])` and `Predicate([Role("CostSymbols")])`), the
route that reads "Pay {3}: Draw a card." 1; plugin `Verb` rows with a
`Custom` frame set that names a Role (`keyword_actions/surveil.ron`,
`[Role("MeasurePhrase")]`; `keyword_actions/scry.ron`, `[[], [Amount],
[ObjectNounPhrase]]`). `ObjectNounPhrase` is not the Complement: "Explore
{3}: Draw a card." 0.

Orchestrator rulings 2026-10-10:

- WB1. Change only `waterbend.ron`'s `grammar:` to a `Verb` row with `frame_set:
  Custom(frames: [[], [Role("CostSymbols")]])` (the bare frame for *whenever
  you waterbend*). `name`, `params`, `spelling` and `body` are untouched. No
  core lexeme, no entry in `lexicon/verbs.ron`.
- WB2. If the plugin loader does not accept `Role("CostSymbols")` in a plugin
  frame, the fix is in the loader's frame translation
  (`crates/deckmaste_lexical_source/src/legacy/plugins.rs`), reading the
  declared Role as it does for core verbs; record it under Deviations. A
  grammar Construction naming the verb is a STOP.
- WB3. One analysis per string. *Waterbend {3}: …* reads as an action Cost
  component (`ActionCost`) with the symbols as the verb's Complement; it must
  not also read as a `FixedTerm` noun followed by a `SymbolCost`. Retiring
  the `FixedTerm` export is intended: confirm no covered face used it
  (0 of the 29 faces are covered).
- WB4. Inflected forms: the plain form (28 faces) and the gerund-participle
  *waterbending* (1) are attested; *waterbends*, *waterbent* and
  *waterbended* are not (0 by grep). Declare the unattested forms
  `Unavailable` where the row allows it, as `keyword_actions/exert.ron` does.
- WB5. Newly covered faces are gains; no Reading-count change on any covered
  face is expected (the word occurs on none).

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Control probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data. Re-probe on the claim parent first.

- Giant Koi: "Waterbend {3}: This creature can't be blocked this turn."
  (control "Pay {3}: Draw a card." 1)
- Katara, Bending Prodigy: "Waterbend {6}: Draw a card."
- Water Whip: "As an additional cost to cast this spell, waterbend {5}."
  (control "As an additional cost to cast this spell, pay {5}." 1)
- Spirit Water Revival: "As an additional cost to cast this spell, you may
  waterbend {6}." (control "… you may pay {2}." 1)
- Waterbending Lesson: "Draw three cards. Then discard a card unless you
  waterbend {2}." (control "… unless you pay {2}." 1)
- The Unagi of Kyoshi Island: "Ward—Waterbend {4}." (control "Ward—Pay {4}."
  1)

Controls that must not change: Earth Village Ruffians "When this creature
dies, earthbend 2." 1 (census); Appa, Steadfast Guardian "When Appa enters,
airbend any number of other target nonland permanents you control." 5
(census); "Pay 2 life: Draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\bwaterbend|\bearthbend|\bairbend|\bfirebend|\bpay \{

(847 faces) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost, zero changed among covered faces. `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, and `cargo xtask lean
check` (a plugin definition changes), once, at the end. Plugin `grammar:`
data loads at runtime, so WB1 needs no grammar rebuild. Tests assert the role
(the verb as head of a Cost component and of an additional-cost Predicate,
the symbols as its Complement), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Avatar Aang: "Whenever you waterbend, earthbend, firebend, or airbend, …"
  also needs a verb *firebend* (`keyword_abilities/firebending.ron` exports
  the keyword only) and four-way bare-verb coordination; the control
  "Whenever you scry, earthbend, or surveil, draw a card." probed 0.
- The semantics of the action (`body:`); semantics_v2 owns bodies.
- Other plugin-owned terms held with `english-v3-plugin-term-definitions`.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
