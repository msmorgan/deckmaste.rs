---
needs: []
---
# Read an attributive scalar comparative before a property noun: a spell with equal or lesser mana value

## Why

*mana value 3 or less* and *power less than X* read; the attributive order
*lesser mana value*, *greater power*, *equal or lesser power* does not.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
key `at:equal or` 10 sole-blocker faces is one corner. `cargo xtask
english-v3 grep -i '\b(equal or )?(lesser|greater|equal) (power|toughness|
mana value|mana values|life|loyalty)\b'` finds **45** faces (47 lines), all
unread; **43** faces have that line as their only failing line. By shape:
*with lesser mana value / power / toughness* about 20; *with greater power /
mana value* about 9; *with equal or lesser …* 10 (all sole); *with equal
toughness* 1; with an overt standard after the noun (*has greater power than
this creature*, *lesser mana value than it*, *greater power or toughness
than Hulkling*) about 7.

## Goal

*lesser*, *greater* and *equal* read as attributive Modifiers of a scalar
property noun with the standard left implicit (*a creature with lesser
power*), coordinate there (*equal or lesser mana value*), and take an overt
*than* standard after the noun through the existing discontinuous route
(*greater power than this creature*), with byte-exact realisation.

## Analysis

CGEL Ch. 13 §4.1.1, p. 1126, [14]: "i *They too had felt the influence of
Christianity to a greater or lesser extent.* ii *We think this is a lesser
risk than taking no action at all.*"; "*Lesser* is found only in attributive
function". That backs attributive *lesser* (and *greater*), their
coordination, and an overt *than* phrase after the noun. Ch. 5 §11(d), p.
432 n. 48: "*greater* is an adjective". That the implicit standard is
grammatical in Oracle text, and the routes below, are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PremodifiedNominal` (`[modifier: AdjectivePhrase, " ", head: Nominal]`,
`require modifier.AdjectiveStructure = Simple`, `require
modifier.AdjectivePremodifierUse = Yes`); schema
`DiscontinuousComparativeNominal` with the `(Nominal, Adjective, …,
DiscontinuousComparativeModifier)` instance, which today requires
`governor.frame = DiscontinuousEquality` and an Equality marker (*same name
as that land*); `ComparativeAdjectivePhrase` / `ComparativeScalarValue`
(*power less than X*, postpositive) and `AdjectivalScalarCoordination` (*4
or greater*). Lexicon (`lexicon/core.ron`): `vocab:ScalarDegree/Lesser`,
`/Greater`, `/Equal`, each with a single frame selecting a *than* / *to*
Comparative Complement, so none is a Simple Adjective Phrase. Glossary:
**Discontinuous Comparative Nominal**, **Scalar Comparative Use**.

Orchestrator rulings 2026-10-10:

- AC1. Data first: the three Adjectives declare a complement-free
  attributive use (the frame / feature `PremodifiedNominal` already reads
  through the Simple Adjective route). No new Construction. *equal or
  lesser* is then ordinary Adjective coordination.
- AC2. Overt standard: add the Ordering case to the existing
  `DiscontinuousComparativeModifier` policy (or a sibling policy row in the
  same schema instance) so an Adjective whose declared frame selects *than*
  can govern `[governor Nominal than NP]`. The licence is the governor's
  declared frame and the marker's `ExpandedComparisonMarker`, as for *same …
  as*. No new schema.
- AC3. The licence reads declared features only. Whether the noun is a
  scalar property noun is read from its declared `ScalarDenotation` if a
  restriction is needed at all; do not add a check whose only job is to
  block *lesser creature*. No test on the three words or on *power*.
- AC4. One analysis per string. The twins to avoid: (a) *power greater than
  3*, *mana value less than or equal to X* keep exactly their postpositive
  Readings (the complement-free use must not let *greater* premodify what
  follows *than*, nor read *greater than 3* as Modifier + stray PP);
  (b) *4 or greater*, *five or greater* keep the
  `AdjectivalScalarCoordination` Reading only; (c) *equal to X* keeps its
  Readings (*equal* complement-free must not strand *to X* as an Adjunct);
  (d) *equal or lesser mana value than that spell* (Saruman of Many Colors)
  has one governor coordination sharing the *than* standard, not *equal*
  alone with an implicit standard beside it.
- AC5. Newly covered faces are gains. Reading-count increases on covered
  faces are not expected; `(?i)\b(lesser|greater|equal)\b` matches 2,091
  faces, so every one is in the selector. An increase on a
  covered *N or greater* / *greater than* / *equal to* face is an AC4
  defect; report each with its shape before landing.

- AC6 (orchestrator, 2026-10-10). Scoped licence, not general mass uses:
  extend the existing bare-property licence so a scalar specification can be
  a comparative Adjective Premodifier. Admission reads the noun's declared
  `ScalarDenotation` and the modifier's declared complement-free attributive
  comparative use. No new Construction if the existing route can take the
  condition; one added form is the fallback. No word test. Every covered
  `(lesser|greater|equal)` face keeps its count.
- W9-A (standing ruling). One analysis per string means one route for this
  construction; independent ambiguity also present in the control is retained
  and recorded, not a STOP.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- Reinterpret: "Counter target spell. You may cast a spell with equal or
  lesser mana value from your hand without paying its mana cost." (control
  "You may cast a spell with mana value 3 or less from your hand without
  paying its mana cost." 3+)
- Body Launderer: "When this creature dies, return another target non-Rogue
  creature card with equal or lesser power from your graveyard to the
  battlefield." (control "… with power 2 or less …" 3+)
- Prehistoric Pet: "This creature can't be blocked by creatures with greater
  power."
- Rushed Rebirth, the clause "search your library for a creature card with
  lesser mana value, put it onto the battlefield …"
- Conformer Shuriken, the clause "If that creature has greater power than
  this creature, …" (AC2)

Also 0 today (`noun-phrase`): "a creature with lesser power"; "a creature
with greater power"; "a creature with equal power"; "a permanent card with
lesser mana value". Controls that must not change: "a permanent card with
mana value 3 or less" (`noun-phrase`) 2; "Its power is 5 or greater." 1;
"Its power is five or greater." 1; "the greatest power" (`noun-phrase`) 1;
"Its power is less than 5." 1. Prehistoric Pet, Rushed Rebirth and Conformer
Shuriken were confirmed as sole failing lines from the blocker report, not
re-probed (the trunk moved under the prebuilt binary).

## Method

Standard constraints apply. Bounded census: every supported face matching
`(?i)\b(lesser|greater|equal)\b` (2,091 faces; the Lexemes gain a declared
use) plus 300 random covered controls (`--random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; AC5 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` and `deckmaste_lexical_source`, once, at the end: a
rebuild after a grammar edit takes about 2–3 minutes and the six test
binaries about 75 seconds, so do AC1 (data, loaded at runtime) and measure
before touching AC2. Tests assert the role (the comparative Adjective as
Modifier of the property noun; the coordination; the *than* phrase as the
governor's Comparative Complement; the AC4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only. Glossary: extend **Discontinuous Comparative Nominal** with the
Ordering Adjective governor (CGEL p. 1126 [14ii]) in the landing.

## Out of scope

- *greater power or toughness than Hulkling* (2 faces): coordinated head
  nouns under one governor, unless AC2 reads it unaided.
- *the lesser of …*, *the greater of …* fused heads.
- Postpositive comparative Determinatives
  (`english-v3-postpositive-comparative-determinatives`).

## Landing record

1. **Census delta** (`vszmnwup`, before implementation at its empty claim-parent-equivalent tree, and after implementation): 1,986 → 2,026 covered faces; 162,369 → 165,756 per-line Readings. Lost faces, decreases, and covered-face increases: none. Gains: 40; wrong-analysis gains: none. Digest-only: the class of covered faces using `DiscontinuousComparativeNominal`, whose governor now uses a helper node wrapper; Heidegger, Shinra Executive (79 → 79) is the one representative in the original manifest. Selector: all 2,091 faces matching `(?i)\b(lesser|greater|equal)\b` plus 300 random covered controls, seed 20261008; 13 overlap, 2,378 identities. Review widening: all 639 supported faces matching `(?i)\bsame\b|\b(more|fewer|less)\b[^.\n]*\bthan\b`, claim-parent-equivalent `xqwovnzx` → head `ttzrlpkw`, retain every covered count: 395 → 397 covered and 101,889 → 103,026 Readings; 0 lost/decreases/increases, 170 digest-only, 467 unchanged, 2 gains (Bane and Saruman, already in the original 40). The 170 digest-only faces are exactly all covered BEFORE faces using `DiscontinuousComparativeNominal`; both its instances replace a lexical governor with a helper node, changing tree fingerprints without adding or removing Readings. No other covered face changes digest. Evidence: `wider-{manifest,before,after,diff}.json`. Accepted-review refresh (`lzryttwv`) imports `ztrnpkyvzntv`, `mzkvyunrqtrw`, `rtzolrtxrrru`, and `mpvousxzotns`. The original manifest re-census preserves every implemented count: 2,029 → 2,037 covered; 166,247 → 167,080 Readings; 0 lost/decreases/increases, 8 incoming gains, 49 digest-only. `ztrnpkyvzntv` accounts for the 49 modal header count-wrapper digest changes and five gains (Collective Effort, Desperate Plea, Go Ninja Go, Markov Retribution, Steal the Show); `mzkvyunrqtrw` accounts for three gains (Ojer Axonil, Purity, The Wanderer). This ticket still has 40 gains, with 11 total incoming gains over both refreshes in the original manifest (51 final gains). The wider re-check has 397 → 401 covered and 103,026 → 106,786 Readings, with 0 lost/decreases/increases, 3 modal-header digest-only changes (`ztrnpkyvzntv`), and four incoming gains: Desperate Plea (`ztrnpkyvzntv`), Ojer Axonil (`mzkvyunrqtrw`), Kotose and Prowl (`rtzolrtxrrru`, preposed *for as long as*). The last two are outside the original manifest: across both manifests there are 53 unique gains, comprising 40 from this ticket and 13 incoming. Evidence: `integrate-wider-{after,diff}.json`. Evidence: `integrate-after.json`, `integrate-diff.json`, and `review-refresh-incoming-changes.txt`. Claim parent: `kuylxzly`. Evidence: `target/english-v3/english-v3-attributive-scalar-comparative/{manifest,before,after,diff}.json`; comparison is the built-in `english-v3 diff --lines`. After `kata refresh`, bounded re-census at `wnvmzmmx` preserves every implemented face and Reading count: 2,026 → 2,029 covered; 165,756 → 166,247 Readings; 3 incoming gains, 0 lost/decreases/increases/digest-only. Ferrafor, Young Yew (432) is attributable to `wkooyukozlpw` (kind-of-counter); The Moment (24) and The War Doctor (35) to `roomopmrpkys` (phase-in/out). Their previously blocked lines contain the respective newly licensed constructions. Evidence: `refresh-after.json`, `refresh-diff.json`, and `refresh-incoming-changes.txt` in the same directory.
2. **Witnesses** (`vszmnwup`): all five full-card witnesses re-probed before implementation had 0 Readings. After: Reinterpret 19; Body Launderer's quoted death clause 12 (full card 24); Prehistoric Pet's quoted sentence 21 (full card 42); Rushed Rebirth 60. Conformer Shuriken's comparative finite clause has 1; its full face remains 0 because `difference` is undeclared, also a vocabulary gap in its BEFORE probe. The four NP witnesses changed 0 → 1, 1, 1, 2 respectively. All ten controls retain their counts: measured-property NP 2; greatest-property NP 1; six copular controls 1 each; Reinterpret numeric control 9; Body Launderer numeric control 4. W9-A preserves independent noun and modifier-attachment ambiguity: the ticket construction itself uses one route. Saruman's coordinated governor shares one standard (local NP 1; face census 1,122 per-line Readings).
3. **Spot checks** (`vszmnwup`, ten new faces): Reinterpret and Body Launderer — coordinated Adjective Premodifier over one property noun; Prehistoric Pet — greater/power; Rushed Rebirth and Orah, Skyclave Hierophant — lesser/mana value; V.A.T.S. — equal/toughness. Saruman of Many Colors — coordinated equal/lesser governor with shared *than that spell*; Triple Triad — lesser/mana value with *than it*; Death's Oasis — lesser/mana value with *than the creature that died*; Hulkling, Burgeoning Bruiser — greater over the existing power/toughness head coordination with *than Hulkling*. Samples `spot-0.json` through `spot-9.json` preserve these roles and report no validation issues.
4. **Deviations and STOPs** (`vszmnwup`): the bare-count-property blocker was resolved by AC6, extending `BareNounPhrase` with Scalar Denotation and comparative-modifier presence; noun countability is unchanged. Two helper Constructions, `DiscontinuousComparativeGovernorHead` and `ComparativeDeterminerHead`, replace the schema's direct Word governors so ordinary governor coordination can share a standard; no new comparative schema or implicit-modifier Construction. The original selected frames remain; the new empty frames supply the attributive use. The scalar conjunct now requires its declared Ordering frame. An initial census found 22 covered-face increases (157 spurious Readings): bare comparatives had entered predicative coordination. A propagated Attributive Adjective Restriction removes them, restoring every claim-parent count. Existing comparative tests were re-spelled for the wrappers and selected-frame role; the old assertion that bare *equal* has no Adjective Phrase was superseded by AC1, while selected-frame complement rejection remains tested in both directions. Hulkling's head coordination reads unaided through the existing route, as the ticket permits. Glossary gaps filled: Implicit Comparative Use, Comparative Modifier Presence, and Attributive Adjective Restriction. Conformer's ordinary-vocabulary residual is routed to [english-v3-difference-noun](../planned/english-v3-difference-noun.md), brought in by refresh; its DN1 declaration covers the missing noun. No noun addition is included in this construction ticket. `AdjectiveAttributiveOnlyUse` is derived from `ImplicitComparativeUse`, without a separate lexical declaration; `english-v3-hygiene-3` item A1 should reuse it. This is a project restriction on the complement-free use, introduced to retire the 157 spurious predicative-coordination Readings; CGEL p. 1126 supports attributive *lesser* only. Predicative *whichever is greater* on Willowdusk, Essence Seer, Triumphant Chomp, and Prophet of the Scarab is blocked on *whichever* today and is not covered; revisit the gate when that item lands so it cannot prune the correct predicative Reading. The unattested *equal power to X* reach (illustrated by `a creature with equal power to that creature`; 0 supported faces matching `(?i)\bequal power to\b`) changes 0 → 3 Readings, with *to X* an ordinary postmodifier rather than the comparative standard.
5. **Tests** (`vszmnwup`): `cargo test -p deckmaste_english_v3` passed, 900 tests; its first run exposed the superseded bare-*equal* assertion, re-spelled before the successful full rerun. `cargo test -p deckmaste_lexical_source` passed, 84 tests. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` both passed. After refresh (`wnvmzmmx`), `cargo build -p xtask` passed, followed by the same four full test/clippy commands: English 918 tests passed; lexical source 84 tests passed; both clippy checks passed with warnings denied. Incoming English tests account for the suite total increase. Refreshed citation checks report 0 noncompliant and 0 stale references; diff audit found no new CR sites. Restored 0; re-spelled 5; ignored 0; added 5 (one suite module, registered once in `suite_6`); removed 0. Review corrections at `ttzrlpkw`: full `cargo test -p deckmaste_english_v3` passed (919 tests), and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed. The exact bare-*equal* Adjective Phrase set is asserted, selected frame 0 still rejects admission without its complement, and both requested predicative negatives are empty. Accepted-review refresh at `lzryttwv`: `cargo build -p xtask` passed; full `cargo test -p deckmaste_english_v3` passed (937 tests); full `cargo test -p deckmaste_lexical_source` passed (84 tests); both `cargo clippy -p` commands above passed with warnings denied. The only conflict was concurrent registrations in `suite_6.rs`, repaired in `vszmnwup` with every module registered once; refresh retry succeeded. No Lean checks; no xtask code changes.
6. **Timing** (`vszmnwup`): final bounded census 5:42.18 wall under `/usr/bin/time -v`, 6 workers, 0 samples per face. Refreshed bounded re-census: 5:03.68 wall, same manifest, workers, and sample setting. Wider claim-parent census: 1:48.62 wall; wider head census: 3:24.79 wall including 1:49 build-lock wait, both under `/usr/bin/time -v` with 6 workers and 0 samples. Accepted-review refresh census: original manifest 6:31.18 wall (including build-lock wait), wider manifest 1:11.15 wall; 6 workers and 0 samples per face. No full-corpus run.
