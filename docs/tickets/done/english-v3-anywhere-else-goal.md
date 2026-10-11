---
needs: []
---
# Read *anywhere else* as a locative Complement: If that creature would leave the battlefield, exile it instead of putting it anywhere else.

## Why

*anywhere* reads only after *from* ("You may cast this card from anywhere."
3, from `docs/tickets/done/english-v3-from-anywhere.md`). As the goal of
*put* it does not, with or without *else*: "Put it anywhere." 0, "Put it
anywhere else." 0, "Exile it instead of putting it anywhere." 0, beside "Put
it there." 1, "Exile it instead of putting it there." 1 and "Exile it instead
of putting it into your graveyard." 1. *else* is not a word of the lexicon.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
`word:else` 16 sole, 16 touched. `cargo xtask english-v3 grep -i '\banywhere
else\b'` finds **16** faces, all unread; **16** have that line as their only
failing line. **15** are the replacement tail *exile it / them instead of
putting it / them anywhere else* (`'\b(put|putting) (it|them) anywhere
else'`); 1 is *but not from anywhere else* (Haakon, Stromgald Scourge).

## Goal

*anywhere else* is an NP (*anywhere* with its post-head dependent *else*)
functioning as the locative Complement of *put*: "… exile it instead of
putting it anywhere else.", with byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.6, p. 423: the compound determinatives include [41iv]
"*everywhere*, *somewhere*, *anywhere*, *nowhere*"; "The analysis of the
locatives as heads of NPs is motivated by their use as object in
constructions like *Have you got anywhere to spend the night?*"; and, under
"*Else* and *more*": "The compound determinatives share with the
determiner-heads *all*, *much*, and *little* the property of taking *else* …
as immediate post-head dependent: *everything else*". Those back *anywhere*
as head of an NP and *else* as its immediate post-head dependent. That such
an NP functions as a goal Complement of *put* with no Preposition is the
project ruling (the from-anywhere ticket's R6a re-categorised *anywhere* as a
Determinative in fused determiner-head use and asked for any attested
adverbial use to be reported: this is it).

Existing declarations: `vocab:CompoundDeterminative/Anywhere`
(`FusedHeadUse: Yes`, `DeterminerUse: Singular`, `ComparativePostmodifierUse:
Yes`; `lexicon/core.ron`), read by `FusedDeterminerHeadNounPhrase` (`[head:
lexical(Determinative)]`, `head.FusedHeadUse = Yes`); `core-verb:Put`'s frame
`Predicate([ObjectNounPhrase, Role("LocativeComplement")])`
(`lexicon/verbs.ron`); `LocativeComplement` (`[phrase: PrepositionPhrase]`,
`phrase.LocativeUse = Yes`), which is how *there* reads.

Orchestrator rulings 2026-10-10:

- AE1. *else* is a post-head dependent of the fused head: one added form on
  `FusedDeterminerHeadNounPhrase` (head + " " + the dependent), licensed by a
  feature the compound determinative declares, with *else* declared in
  `lexicon/core.ron` in the category the glossary gives a post-head
  dependent of this kind (amend the glossary in the landing if it has none).
  *else* gets no other function: 0 faces have it outside *anywhere else*
  (`'\belse\b'` 16 faces, the same 16).
- AE2. The locative function: one added form on `LocativeComplement` taking
  an NP whose head declares locative use (`LocativeUse` on the compound
  determinative, data), beside the PP form. No Preposition homograph of
  *anywhere*, and the NP does not become a general Object of *put* (the
  frame's first slot stays the thing moved).
- AE3. One analysis per string. *from anywhere* keeps exactly its Readings
  (136 covered lines); *putting it anywhere else* has one structure: Object
  *it*, locative Complement [*anywhere else*]. The twins to avoid:
  *anywhere* as a second Object, and *else* attached to the clause.
- AE4. Bare *put it anywhere* (no *else*) is unattested; it will read as a
  consequence of AE2. Do not add a check whose only job is to block it;
  report any covered face it changes.
- AE5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected; the 136 covered *from anywhere* lines and the locative
  *there* lines must keep their counts. Report any increase with its shape.

- AE6 (orchestrator, 2026-10-11): extend the compiler with the leading
  separator mirror of SB8's optional lexical field. Use one form on
  `FusedDeterminerHeadNounPhrase`; requirements apply only when present and
  absent optional fields remain omitted from Debug. No wrapper Construction.
  Verify compiler tests and clippy and disclose the compiler change. The
  manifest is the 4,434 faces using the touched Constructions, the ticket
  pattern, and 300 random covered controls. AE2–AE5 stand; stop before
  integration for serialized orchestrator review.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Gruesome Encore: "Put target creature card from an opponent's graveyard
  onto the battlefield under your control. It gains haste. Exile it at the
  beginning of the next end step. If that creature would leave the
  battlefield, exile it instead of putting it anywhere else."
- Nephalia Academy: "If a spell or ability an opponent controls causes you to
  discard a card, you may reveal that card and put it on top of your library
  instead of putting it anywhere else."
- Storm Herald: "When this creature enters, return any number of Aura cards
  from your graveyard to the battlefield attached to creatures you control.
  Exile those Auras at the beginning of your next end step. If those Auras
  would leave the battlefield, exile them instead of putting them anywhere
  else."
- From the Catacombs: "Put target creature card from a graveyard onto the
  battlefield under your control with a corpse counter on it. You take the
  initiative. If that creature would leave the battlefield, exile it instead
  of putting it anywhere else."

Probe the fragment first on the claim parent: "If that creature would leave
the battlefield, exile it instead of putting it anywhere else." Controls
that must not change: "Exile it instead of putting it into your graveyard."
1; "Exile it instead of putting it there." 1; "Put it there." 1; Dissipate
"Counter target spell. If that spell is countered this way, exile it instead
of putting it into its owner's graveyard." 2 (census); Luminous Phantom "If
Luminous Phantom would be put into a graveyard from anywhere, exile it
instead." 4 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(anywhere|else|there)\b|\binstead of putting\b` (the
first alternative alone is 624 faces; record the total) plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; AE5 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`, once, at
the end; a rebuild after a grammar edit takes about 2–3 minutes, the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
role (*else* as post-head dependent of the fused head; the NP as locative
Complement of *put*; the AE3 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Haakon, Stromgald Scourge: "… but not from anywhere else." (the *but not
  from* PP coordination is a separate blocker: "You may cast this card from
  your graveyard, but not from exile." probed 0).
- Faces whose only remaining blocker is elsewhere on the line: The Eighth
  Doctor, Kheru Lich Lord (*at random from your graveyard*,
  `english-v3-at-random-before-source`), Realmbreaker, Geth, Elemental
  Expressionist, Llanowar Greenwidow, Spirit-Sister's Call (quoted grants).
- *something else*, *someone else* (0 faces).

## Landing record

1. **Census delta (`oslovlpz`, claim-parent tree `ktusvmxm`):** on the
   AE6 4,434-face manifest, covered 4,348 → 4,361; Readings 209,959 →
   210,218. Lost/decreased/increased/digest-only: all zero (digest-only
   classes and line counts: none); 13 gains, 4,421 unchanged, zero issues.
   Selector: 184 fused-head users, 3,822 locative users, union with the
   660-face Method grep and 300 distinct random covered controls, seed
   20261008. `manifest-node-users.json` is the exact manifest. Official
   `cargo xtask english-v3 diff before.json after.json --json diff.json
   --lines` compares its intersection with the broader parent report;
   the 21,284 extra BEFORE faces are outside this comparison, not losses.
   Before refresh, no previously covered face changes count or digest,
   including *from anywhere* and *there*. Evidence: `target/english-v3/english-v3-anywhere-else-goal/`.
   Refresh verification (`qznlvuzl`, source `oslovlpz`): covered
   4,348 → 4,362; Readings 209,959 → 210,236; 14 gains, zero losses,
   decreases or increases, 80 digest-only faces / 80 lines. Official
   `diff-refresh-incoming.json` isolates Reckoner Bankbuster (0 → 18) and
   81 digest-only faces / 81 lines, all from trunk `ouykkpns`: 60 measured-cost
   schema/category lines, 21 scalar-predicate schema/category lines, no mixed
   or unclassified line. Combined claim-parent digest classes are 59 and 21
   faces/lines respectively: Llanowar Greenwidow's same-count cost-schema
   respelling is classified as a gain in that comparison. All 13 own gains
   retain their counts. Evidence: `after-refresh.json`, `diff-refresh.json`,
   `diff-refresh-incoming.json`, `refresh-digest-classes.json`.
   Authorized integration verification (`urrsqmrl`): original 4,434-face
   manifest, identical final totals and classes; `diff-integration.json`
   reports 14 gains, 80 inherited digest-only faces, zero lost/decreased/
   increased. Official reviewed-result → final comparison has all 4,434
   unchanged and zero changed lines (`diff-integration-incoming.json`).
2. **Witnesses (`oslovlpz`):** full ticket text / its *there* control:
   Gruesome Encore 8/8, Nephalia Academy 22/22, Storm Herald 6/6, From the
   Catacombs 28/28; each ticket witness was 0 on `ktusvmxm`. The replacement
   fragment is 0 → 1, its control remains 1. Graveyard control, *there* tail,
   and *Put it there* remain 1; Dissipate remains 2; Luminous Phantom remains
   4. Each bare *anywhere [else]* goal/tail is 0 → 1 (AE4). All enumerations
   complete with zero issues. Census counts sum per-line Readings, rather
   than multiplying full-document alternatives: the four witness faces
   have 8, 23, 7 and 31 respectively.
3. **Spot checks (`oslovlpz`):** all 13 gains were probed, complete and
   byte-exact, with zero issues. Ten named checks: Elemental Expressionist,
   From the Catacombs, Geth, Thane of Contracts, Gruesome Encore, Isareth the
   Awakener, Llanowar Greenwidow, Moira and Teshar, Nephalia Academy,
   Realmbreaker, the Invasion Tree, and Spirit-Sister's Call. Each selected
   analysis has *put*'s object followed by `LocativeComplement` form 1,
   whose phrase is fused-head *anywhere* with lexical *else* as its optional
   dependent. Storm Herald, The Eighth Doctor and Whip of Erebos have the
   same analysis. Wrong-analysis gains: zero. Several faces marked blocked
   elsewhere in the draft already have those surrounding routes on the
   claim parent; this change adds only the missing dependent/goal route.
   The incoming Reckoner Bankbuster gain (`qznlvuzl`) was separately probed:
   full Document 30, complete, byte-exact, zero issues; its quoted crew
   sentence has irrealis *Be* selecting the measured *2 greater* predicative
   Complement, the `ouykkpns` IS1 route.
4. **Deviations and STOPs:** the same-field compiler invariant rejected the
   attempted two-form AE1 implementation; the STOP was resolved by AE6.
   No wrapper or other Construction was added. Compiler syntax now supports
   `optional(lexical(Category), before("separator"))`, beside SB8's trailing
   separator. Its `present` feature permits a conditional host-license
   table; requirements on the optional word still apply only when present,
   and absent fields remain omitted from Debug. NP interfaces/policies
   now export locative permission (ordinary heads: No; NP modifiers pass
   through the head), required by AE2. No verb frame changed or route was
   retired. Glossary gaps filled: immediate post-head dependent and its
   head/dependent licenses, locative use, optional field presence. Nominal
   *else* adopts CGEL's proposed Preposition analysis (Ch. 7 §2.4, p. 615,
   n. 5) as an AE1 project choice; p. 423 backs its immediate dependent use.
   Refresh brought `ouykkpns` and `knlqxlok`; the only merge conflict was the
   two new `suite_5.rs` registrations. Both were retained exactly once;
   no test was deleted or suppressed. No plugin file changed.
5. **Tests (`oslovlpz`):** `cargo test -p deckmaste_english_v3` passed
   1,083 tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. `cargo test -p deckmaste_lexical_source` passed 44;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   `cargo test -p deckmaste_construction_v3_core` passed 41 (also repeated
   after the clippy cleanup); `cargo clippy -p deckmaste_construction_v3_core
   --tests -- -D warnings` initially found function-length and empty-assert
   style warnings, fixed by extracting separator emission and an exact
   empty-vector assertion; its final run passed. Added 6, restored 0,
   re-spelled 9, newly ignored 0, removed 0. Eight English tests retain their
   exact expected values with explicit absent `dependent: None`:
   `from_anywhere::{brinebound_gift, yawgmoths_agenda, planar_void,
   rest_in_peace, generous_soul,
   independently_constructed_fused_head_and_source_obey_both_laws,
   comparative_modifier_attaches_to_anywhere_and_retains_nominal_scope}`
   and `once_frequency::independent_fused_determiner_head_and_transitive_do_preserve_lexical_identity`.
   Compiler `optional_lexical_fields_apply_requirements_only_when_present`
   now expects the absence helper's explicit No presence export, retaining
   its exact present-word requirement comparison. The inherited ignored
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
   remains blocked by MS4's governed Mana Phrase Subject route. Citation
   audit selected zero new sites; check: zero stale, zero noncompliant.
   After refresh (`qznlvuzl`), `cargo build -p xtask` and all six listed
   full-suite/clippy commands passed again: English 1,092; lexical source 44;
   compiler 41; the same inherited ignore remains. Trunk `ouykkpns` also
   re-spelled four tests, with complete comparisons preserved:
   `copular_scalar_location::{independent_scalar_location_preserves_both_laws_and_traversal_identity,
   independent_non_scalar_subject_and_wrong_frame_are_rejected}` add only
   `Category::ScalarLocationPredicate` to their shared predicate value;
   `cost_scalar_complement::{independent_complements_preserve_both_laws_and_traversal_identity,
   ordinary_infinitivals_and_unlicensed_measure_heads_are_rejected}` add only
   `Category::ScalarLocationComplement` to their measured values. These are
   inherited, additional to this ticket's nine re-spelled tests.
   Authorized integration (`urrsqmrl`): `cargo build -p xtask` and all six
   full test/clippy commands passed again (English 1,092, lexical source 44,
   compiler 41, inherited ignore unchanged). The named
   `selected_complement_clusters::all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets`
   test passed; no chart ceiling was changed.
6. **Timing (`oslovlpz`):** no full census was run, per the binding bounded
   budget. The broader bounded parent census took 8m04.40s; the exact
   4,434-face AFTER run took 3m58.69s under `/usr/bin/time -v`, six workers,
   zero samples per face. Refresh (`qznlvuzl`) took 6m43.23s including the
   Cargo queue; its census corpus wall was 3m50.97s, again six workers and
   zero samples. No second full census ran. Authorized final bounded run
   (`urrsqmrl`): 2m11.51s, six workers, zero samples, zero issues.

## Claim-parent evidence (2026-10-11)

Measured on pristine `oslovlpz` (tree identical to claim parent `ktusvmxm`):
all four full card witnesses and the replacement fragment have 0 Readings.
The three locative controls each have 1, Dissipate has 2, and Luminous
Phantom has 4. The fragment with *there* replacing *anywhere else* has 1.
The bare *Put it anywhere.* / *Put it anywhere else.* and the two
*Exile it instead of putting it anywhere [else].* probes each have 0.
All enumerations completed without issues. Evidence is under
`target/english-v3/english-v3-anywhere-else-goal/before-probe-*.json`.

The Method grep selects 660 supported faces. An expanded declaration-surface
grep plus 300 distinct random covered controls (official diff manifest,
seed 20261008) produced a 25,718-face claim-parent census: 23,600 covered,
695,867 Readings, zero issues, 8m04.40s under `/usr/bin/time -v`, 6 workers,
0 samples. All 300 random controls remain covered on this parent. The census
identifies 184 faces using `FusedDeterminerHeadNounPhrase` and 3,822 using
`LocativeComplement`; their union with the 660 ticket faces and those same
300 controls is the exact, narrower manifest for further checks. Evidence:
`before.json`, `before.time`, `manifest.json`, `manifest-node-users.json`,
`ticket-selector.json` and `declaration-selector.json` in that directory.

The attempted two-form implementation was rejected by the compiler's same-field
invariant, prompting a STOP. AE6 resolves it with an optional post-head field
in the existing single form. The rejected draft remains evidence only
(`rejected-two-form.patch`); it is not a second route in the grammar.
