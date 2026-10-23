---
needs: []
---
# Stratum 7: Named-token apposition

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read a supplementary appositive Noun Phrase after a token's Introduced Name (*create Voja, Friend to Elves, a legendary 3/3 ... token*). The name itself may contain a comma. The appositive keeps its *with* postmodifiers.

## Grounding

- CGEL, Ch. 15 §5, p. 1357 (Apposition).
- CGEL, Ch. 5 §14.3, pp. 447–448.
- `docs/oracle-style-guide.md` §12, "Tokens" (appositive form required for a named token).

## Witness cards

- Rinoa Heartilly
- Minsc, Beloved Ranger
- Stangg
- Tamiyo, Compleated Sage
- Tolsimir, Friend to Wolves

## Estimated faces

About 27 faces (an estimate from the 2026-10-23 census, not a measured figure).
12 more faces also need `english-v4-s7-quoted-abilities`.

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`named_token_apposition.txt:2–6`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Implementation notes

A trailing NP Supplement leaves the token name as the Object and retains the
Supplement's postmodifiers. Ordinary bare-plural projection now checks the
count heads' Conjunct Number, so a coordination of singular count nominals
cannot lose its shared determiner and become a Supplement (CGEL, Ch. 5 §8.5,
pp. 409–410; Ch. 15 §3.1, p. 1323; OSG §7, "Plurals and agreement"). Plural
count, mass, and independently determined NP coordinations remain available.

Thought Dissector, Willowdusk, Essence Seer, and Prophet of the Scarab leave
the deferral list with judgments for their intended alternatives and
*whichever* Supplements. The four class-list units remain unread. Hearth
Elemental and Frantic Firebolt remain deferred: the existing finite-clause /
imperative coordination schema admits a grammatical but unintended separate
imperative. Full-relative-clause judgments prevent that alternative from
counting as coverage until the intended coordination is implemented. No
coordinator or grammatical mixed-mood coordination is excluded.

Garth One-Eye's first sentence has four different coordination/Supplement
divisions, each with six attachment/state alternatives. Tainted Pact's final
sentence has 30 distinct scope, attachment, and Word Form analyses; its middle
sentence contributes 28 unchanged analyses. These are distinct analyses, not
duplicate trees. The measured growth is accepted by the task owner.

## Landing note

1. **Census delta** (ywosppnq): faces read 28,802 → 28,834; total Readings 250,544 → 252,372; Surplus 109,750 → 110,323; lost faces none. 73 units change; `strata --check` passes. The only unit above 4,096 Readings is Maralen, Fae Ascendant, line 2, unchanged at 7,354.
2. **Judgment set** (ywosppnq): `cargo test -p deckmaste_english_v4` passes (68 tests); all 12 named-token judgments and six other active Supplement judgments hold. Six new intended-analysis judgments remain deferred. Fourteen judgments added; no tests or judgments rewritten or retired. Clippy and citation checks pass.
