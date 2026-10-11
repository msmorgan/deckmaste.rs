---
needs: []
---
# Read *power and toughness each equal to …* inside a Noun Phrase: an artifact creature with power and toughness each equal to its mana value.

## Why

The clause form reads ("This creature's power and toughness are each equal to
the number of cards in your hand." 3). The NP-internal form, after *with* or
as Object of *have*, does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): hidden under `at:turn .` 4,
`at:control .`, `at:1 .`, `at:way .`. `cargo xtask english-v3 grep 'power
and (base )?toughness each equal to'` finds **19** faces, all unread; 19
failing lines match, **18** of them the only failing line of their face.
Hosts (sole lines): *with (base) power and toughness each equal to* 13; *has
/ have base power and (base) toughness each equal to* 5.

## Goal

A coordination of property nouns postmodified by *each equal to* + NP reads
as one NP (Complement of *with*, Object of *have*), with *each* as the
quantificational Adjunct of the reduced predication.

## Analysis

CGEL Ch. 5 §9.2, p. 413, [13]: fused-head partitives "function as
quantificational adjunct in clause structure" and "serve to quantify the
subject"; "Universal *all*, *both*, and *each* can occur as implicit
partitives". That backs *each* in *are each equal to*
(`english-v3-floated-each-quantifier`). Its use inside a postpositive AdjP
with no verb is a project ruling. Glossary: **Scalar Property Value**,
**Property Value Position**, **Predicand**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs` and the
landings `english-v3-floated-each-quantifier`, `english-v3-property-equal-to`,
`english-v3-coordinated-property-values`): single property + *equal to* reads
NP-internally ("Target noncreature artifact becomes an artifact creature with
power equal to its mana value until end of turn." 3; "Creatures you control
have power equal to the number of creatures you control." 3); coordinated
property + Slash Pair reads ("… with power and toughness 4/4." 3; "It has base
power and toughness 3/3." 2). Read 0: "It becomes an artifact creature with
power and toughness equal to its mana value." and "It has power and toughness
equal to its mana value." (no *each*), so the coordinated head is the gap and
*each* a second one.

Orchestrator rulings 2026-10-10:

- PE1. Refine the `english-v3-property-equal-to` route: let its property head
  be the coordination of property nouns the Slash Pair route already takes,
  and let its *equal to* AdjP carry the quantificational *each* that the
  clause route already licenses after *be*. No new Construction if both are
  instances of existing schemas.
- PE2. Licences read declared features: the nouns' property class, the
  Determinative's declared floating/quantificational licence, the head's
  plural (coordinated) number. No test on *power*, *toughness*, *each* or
  *equal*.
- PE3. *each* requires a plural predicand: "with power each equal to …" must
  not read. That follows from agreement with the coordination; add no
  separate check.
- PE4. Without *each* (*power and toughness equal to X*, 0 today; 2 faces, 1
  unread): in scope only as the same refinement minus
  the Adjunct; report its count. It must have one Reading, with *equal to*
  predicated of the coordination, not of *toughness* alone.
- PE5. One analysis per string: *each* is not a Determiner of a following NP
  and not a postposed quantifier of *toughness*; the AdjP is not attached to
  the outer noun (*creature*).
- PE6. Newly covered faces are gains. Reading-count increases are possible on
  the 144 covered `each equal to` faces (clause form) and on covered *power
  and toughness* + Slash Pair faces; none is expected, and each is reported
  with its shape.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Karn's Touch: "Target noncreature artifact becomes an artifact creature
  with power and toughness each equal to its mana value until end of turn."
  (probed without the duration: 0; with *power equal to* 3)
- Porcelain Gallery: "Creatures you control have base power and toughness each
  equal to the number of creatures you control." (0)
- Trench Gorger (last sentence): "If you do, this creature has base power and
  base toughness each equal to the number of cards exiled this way."
  (consequent probed alone: 0)
- Sydri, Galvanic Genius: "{U}: Target noncreature artifact becomes an
  artifact creature with power and toughness each equal to its mana value
  until end of turn." (same sentence as Karn's Touch)

Controls that must not change: "This creature's power and toughness are each
equal to the number of cards in your hand." 3; "It becomes an artifact
creature with power and toughness 4/4." 3; "It has base power and toughness
3/3." 2; "Creatures you control have power equal to the number of creatures
you control." 3.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(power|toughness)[^.]* equal to` (every property + *equal to*)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
PE6 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the coordination as head, *each* as quantificational Adjunct, the
*equal to* AdjP as its postmodifier, PE3 and PE5 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *… each equal to X plus 1* (Fractalize): *plus one* is a sibling ticket's.
- *power and toughness each equal to* on faces with another blocker (1).
- Semantics of the two-property assignment.

## Landing record

1. **Census delta** (`svlsnsmvwlup`, claim parent `onzmrvku`): bounded selector
   `(power|toughness)[^.]* equal to` (340 faces), plus 300 covered controls,
   seed 20261008; 636 distinct faces. Covered **584 → 601**; Readings
   **11,393 → 12,174**. Lost: none; decreases: none; digest-only: none;
   gains: 17, all through the intended property route. Woodcaller Automaton
   increases **285 → 293**: the eight additions are PE4's unquantified
   equality predicated of the complete property coordination. They combine
   two independent *base* scopes, two comparison-Complement scopes and two
   surrounding conditional scopes; the same eight shapes occur through one
   property route, under W9-A. No added equality attaches to toughness alone.
   Existing unrelated coordination scopes remain preserved. Evidence:
   `target/english-v3/english-v3-property-pair-each-equal-to/{before,after,diff}.json`;
   comparison made with `cargo xtask english-v3 diff --lines`.
2. **Witnesses** (`svlsnsmvwlup`): claim-parent probes were all 0. After:
   Karn's Touch sentence **3**; Porcelain Gallery sentence **6**; Trench
   Gorger's last sentence **18**; Sydri's first activated ability **3**.
   Matching controls omitting *each* give **3 / 6 / 18 / 3**, respectively:
   independent ambiguity is retained under W9-A. The four ticket controls
   remain **3 / 3 / 2 / 3**. PE4's two general sentence controls give
   **3 / 1** (the first retains the existing outer-NP ambiguity).
   The singular property + *each* control remains **0**.
3. **Spot checks** (`svlsnsmvwlup`): Animate Artifact; Fractalize; Gideon,
   Champion of Justice; Karn's Touch; Karn, Silver Golem; March of the
   Machines; Opalescence; Porcelain Gallery; Spark Rupture; Starfield of Nyx.
   Each selected Reading retains a coordinated property head in
   `MeasuredAttribute`, with the comparative AdjP in the existing scalar
   Property Value route and exactly one internal Quantificational Adjunct.
   *base* and surrounding scope ambiguity is retained. No wrong-analysis
   gains found. Probe evidence is under the same target directory.
4. **Deviations and STOPs**: one Construction added,
   `QuantifiedComparativeAdjectivePhrase`: the existing quantification
   schemas require an explicit Subject or auxiliary Verb and cannot express
   this prefix AdjP instance; the property coordination and scalar-value
   routes are reused. No Construction, frame, lexical route or test retired.
   Added glossary term **Property Equality Form**, extending **Property
   Value** and **Quantificational Adjunct** under PE1–PE5, explicitly as
   project rulings. Fractalize is an incidental gain using arithmetic already
   present on the claim parent; no *plus 1* implementation was added. The
   user's bounded verification and seed override apply. No linguistic STOP;
   the standing pre-integration review STOP applies.
5. **Tests** (`svlsnsmvwlup`): `cargo test -p deckmaste_english_v3` passed
   **1,048 tests**, once; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed after fixing its one `replace_box` warning in the new test.
   Restored **0** / re-spelled **0** / ignored **0** / added **5** / removed
   **0**. The module is registered only in `suite_4`. No other crate's code
   or consumed data, and no plugin definition, changed. Citation checks:
   zero stale and noncompliant citations; the diff adds no CR citation.
6. **Timing** (`svlsnsmvwlup`): bounded after census **21.12 s**, **6 workers**,
   `--samples-per-face 0`. No full-corpus census or Lean check was run.
