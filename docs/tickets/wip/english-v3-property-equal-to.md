---
needs: []
---
# Read an equality property value: each nonland permanent with mana value equal to the number of charge counters on this artifact.

## Why

*with power less than X* reads since `english-v3-postpositive-comparative-property`
landed, but *with power equal to X* does not. Not a wave-5 recon bucket; the
comparative-property review counted it. `cargo xtask english-v3 grep -i
'\bwith (power|toughness|mana value) equal to\b'` finds **36** faces, all
unread on census `qsupksky` (32,828 supported faces, 25,474 covered, 7,354
unread). On 21 of them the face's only failing unit at `qsupksky` contains
the phrase and no *plus*; 10 faces have *equal to 1 plus …* and also need
`english-v3-arithmetic-nominal-operand`. Surface counts, not gain
forecasts.

Probes (`--readings 1`): "Its power is equal to its toughness." 1; "… with
mana value less than the number of charge counters on this artifact." 1;
"… with mana value equal to the number of charge counters on this
artifact." 0.

## Goal

After a property noun (*power*, *toughness*, *mana value*), the equality AdjP
*equal to X* reads as the property value, through the same route as the
ordering comparatives, with byte-exact realisation.

## Analysis

CGEL Ch. 6 §3.3(c), p. 552, [46ii] (*a leader younger than you*): postpositive
AdjPs follow an ordinary noun head; Ch. 6 §2.1, pp. 528–529, for the
postpositive function. That backs the function only (the citation the
ordering landing used).

Existing route: `MeasuredAttribute`
(`crates/deckmaste_english_v3/src/declarations.rs:4342`,
`[head: Nominal, quantity: PropertyValue]`) ← `ScalarMeasureValue` instance
row `(PropertyValue, ScalarValue, ScalarPropertyValueKind)` (`:4300`) ←
`ComparativeScalarValue` (`:4309`, `[value: ComparativeAdjectivePhrase]`),
which today requires `value.ComparisonMarker = Ordering` (`:4312`).
*equal to* is a `ComparativeGovernorHead` with `ComparisonMarker = Equality`
(*to* declares it, `lexicon/core.ron:225`); coordinated *equal to or greater
than* already reads because `coordinated_comparison_marker`
(`declarations.rs:4427`) yields `Ordering`.

Orchestrator rulings 2026-10-09:

- R13a. Same route as R5a (2026-10-08): admit the Equality marker through the
  existing `ComparativeScalarValue` / `PropertyValue` route by relaxing the
  `ComparisonMarker` requirement (a table over the declared feature, or the
  requirement dropped if both values are licensed); no new Construction, no
  new NP postmodifier.
- R13b. The predicative "… is equal to X" must keep its claim-parent Reading
  count; *damage equal to its power* and other non-property heads must not
  gain a Reading through `MeasuredAttribute` (its `PropertyValuePosition`
  licence already gates the head; verify by probe, do not add a word guard).
- R13c. *equal to or greater than* keeps exactly its current Readings; the
  Equality admission must not create a second bracketing of the coordinated
  governor.
- R13d. *with power equal to that card's power and toughness equal to its
  toughness* (Soul Separator) composes through the landed coordinated
  property-value route; record it in spot checks if it reads.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control
(*equal to* → *less than*) 1. (D) = `--category Document`.

- Master Symmetrist: "Whenever a creature you control with power equal to its
  toughness attacks, it gains trample until end of turn."
- Chalice of the Void: "Whenever a player casts a spell with mana value equal
  to the number of charge counters on this artifact, counter that spell."
- Sanctum Prelate: "Noncreature spells with mana value equal to the chosen
  number can't be cast."
- Ratchet Bomb (D): "{T}, Sacrifice this artifact: Destroy each nonland
  permanent with mana value equal to the number of charge counters on this
  artifact."
- Aether Vial (D): "{T}: You may put a creature card with mana value equal to
  the number of charge counters on this artifact from your hand onto the
  battlefield."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bequal to\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *equal to X* read as a postmodifier of the whole NP rather than as the property value, *than*/*to X* attached outside the AdjP, or a non-property head gaining the value, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *whose power is equal to X* (relative-clause gap).
- Implementing arithmetic operands (`english-v3-arithmetic-nominal-operand`,
  landed as `owrvksuw`); verify their composition with equality here.

## Landing record

1. **Census delta** (`wlstpkpu`; before is the original, unchanged claim-parent tree
   `prmkpxrr` before refresh): 1,906 faces, selected by supported analysed text matching
   case-insensitive `\bequal to\b` (1,606 faces) plus 300 covered controls,
   seed `20261008`, in `target/english-v3/english-v3-property-equal-to/manifest.json`.
   Covered 1,380 → 1,411; total Readings 193,360 → 201,762. Official
   `english-v3 diff --lines`: 31 gained, 0 lost, 0 decreases, 1 increase,
   0 digest-only, 0 changed sources. Lost faces: none. Arithmetic *equal to
   1 plus …*: 11 gains (including Birthing Pod and Neoform), separately from
   20 non-arithmetic gains. Woodcaller Automaton increases 273 → 285
   (line 1: 272 → 284): six new NP coordinations on the *becomes* sentence,
   doubled by its containing line's two trigger analyses. Their shape is
   `[a Treefolk creature with haste and base power] and [toughness equal to
   this creature's power and toughness]`, with three existing left-conjunct
   modifier scopes; three counterparts instead have the wide genitive
   `[[toughness equal to this creature]'s [power and toughness]]`. The right
   property value retains `MeasuredAttribute → ScalarMeasureValue(PropertyValue)
   → ComparativeScalarValue → ComparativeAdjectivePhrase`. These distinct
   grammatical scopes are retained under the ambiguity contract, not a
   duplicate predicative or coordinated-governor route. Evidence: `diff.json`
   and `woodcaller-sentence-complete.json` in the same target directory.
   After `kata refresh`, bounded recheck on `pqpstqts` (then-empty child of
   refreshed `wlstpkpu`) covers 1,419 faces with 202,946 Readings.
   `final-diff.json` against the original claim parent: 39 gained, 0 lost,
   0 decreases, 1 increase, 0 digest-only, 1,866 unchanged. The eight
   refresh-only gains are Awe Strike, Bone Mask, Cho-Arrim Alchemist,
   Dazzling Reflection, Delif's Cone, Dragonfly Swarm, Intervention Pact,
   and Reverse Damage; they are inherited from the asyndetic-premodifier
   (`xqtkuyqk`, landing `zlottuom`) and next-time temporal (`rxwmpvru`,
   correction `stymwqqo`, landing `szkxvvyl`) changes. `refresh-diff.json`
   reports 8 gained, no losses, decreases, increases or digest-only changes,
   and 1,898 unchanged. The ticket's own 31 gains (11 arithmetic / 20 other)
   and Woodcaller increase remain unchanged; no decreased faces to attribute.
2. **Witnesses** (`wlstpkpu`, complete probes): all five were 0 before;
   Master Symmetrist 6, Chalice of the Void 10, Sanctum Prelate 2,
   Ratchet Bomb 31, Aether Vial 42 after. Their ordering controls retain
   6 / 10 / 2 / 31 / 42. R13b controls: predicative equality 1 → 1;
   *damage equal to its power* 1 → 1, remaining a postmodifier rather than
   a property value. R13c: *power equal to or greater than its toughness*
   1 → 1; Ironclaw Curse's sentence 8 → 8 and face 10 → 10.
   Refreshed probes (`probes-refreshed.log`) retain all five witness counts,
   their ordering controls, and the R13b/R13c fragment counts above.
3. **Spot checks** (`wlstpkpu`; face / affected-line Reading counts):
   Master Symmetrist 7 / 6, Chalice of the Void 15 / 10, Sanctum Prelate
   3 / 2, Ratchet Bomb 32 / 31, Aether Vial 43 / 42, Blast Zone 38 / 31,
   Brain in a Jar 1,659 / 1,658, Engineered Explosives 32 / 31,
   Birthing Pod 52 / 52, Neoform 263 / 262. All ten are newly covered.
   Selected analysis: the property Nominal selects the equality AdjP as
   `PropertyValue`, retaining its governor and comparison Complement inside
   the AdjP. Master Symmetrist compares power with *its toughness*; Sanctum
   Prelate compares mana value with *the chosen number*; the counter-based
   cases retain *the number of charge counters on this artifact/land*;
   Birthing Pod and Neoform retain *1 plus the sacrificed creature's mana
   value* as an arithmetic Measure Phrase inside that Complement. R13d's
   Soul Separator property fragment reads 0 → 7 with coordinated property
   values; its full face remains unread because its unrelated copy-token
   *except* sentence still fails. Probe evidence: `spot-0.json` through
   `spot-10.json`, `property-shape.json`, `charge-property-shape.json` and
   `after-prebuilt-control-4.json` under the same target directory.
4. **Deviations and STOPs** (`wlstpkpu`): no Constructions or frames added
   or retired; remove only the ordering-only requirement under R13a.
   The six tests beyond the five witness tests cover R13b–R13d, landed
   arithmetic composition and the independently composed-value roundtrip
   law. Re-spell the existing equality exclusion in
   `shared_comparative_complement_stays_inside_the_value_slot`; no test
   deletion. Amend the Scalar Value glossary definition under R13a; no CGEL
   passage is claimed to license the project value-slot analysis. Seed and
   arithmetic scope follow the user's updated instructions. No design STOPs;
   standing review STOP before integration.
5. **Tests** (`wlstpkpu`): `cargo test -p deckmaste_english_v3` passed
   (698 tests, 0 failed, 0 ignored; doc tests passed);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   After grammar-bearing refresh, the same full test command passed on
   `pqpstqts` (716 tests, 0 failed, 0 ignored; doc tests passed); the same
   clippy command also passed on the refreshed tree. Evidence:
   `tests-refreshed.log` and `clippy-refreshed.log` under the target directory.
   Restored 0 / re-spelled 1 / ignored 0 / added 11 / removed 0;
   removals: none. This ticket changed no other crate's code or consumed data.
6. **Timing** (`wlstpkpu`): bounded after census 2m 34s 720ms wall, 6 workers,
   peak RSS 10,550,532 KiB (`/usr/bin/time -v`, prebuilt binary,
   `--samples-per-face 0`). Refreshed bounded census: 54s 370ms wall,
   6 workers, peak RSS 10,223,504 KiB (same prebuilt-binary invocation;
   `after-refresh.time`).
