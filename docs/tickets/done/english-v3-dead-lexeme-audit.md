---
needs: []
---
# Delete every lexeme with zero tokens in the supported corpus

## Why

The user ruled 2026-10-06 that the grammar should support all and only Magic
vocabulary. These lexemes "snuck in" and appear on no card.

## Goal

The lexicon (`crates/deckmaste_lexical_source/lexicon/core.ron`, `verbs.ron`
and every other lexicon file in that directory) declares only vocabulary that
occurs on at least one supported-corpus face. Known dead entries found
2026-10-06: the prepositions *against*, *because*, *within* and *through*; each
carries a `PrepositionFunctionLicence` but has zero supported tokens. Audit the
whole lexicon, every category (prepositions, verbs, nouns, adjectives,
determiners and the rest), not just those four. The governing rule is the
"Pruning rule for licensed functions and attachments" in
`docs/decisions/english-lexical-analysis.md`.

## Method

1. Measure before: full corpus `cargo xtask english-v3 --all --workers 12
   --samples-per-face 0 --output <gitignored or scratch path>`. Record the
   covered-face count and total Readings, stamped with the change id.
2. Determine supported tokens per lexeme. Use whatever the xtask already
   exposes (check `cargo xtask english-v3 --help` and
   `crates/xtask/src/english_v3/` for a token/lexeme census; the lexicon loader
   reports lexeme-to-token mappings). If no existing output gives per-lexeme
   token counts, a throwaway script run from scratch space is acceptable, but
   it must not be checked in: no new xtask subcommands, flags, fixtures or
   tooling (standing rule).
3. Delete every lexeme with zero supported tokens, including any frames,
   feature declarations and plugin references that only it used. A
   `plugins_v2` keyword-action `grammar:` frame field that references a deleted
   lexeme may be edited; plugin bodies are untouched.
4. Measure after: full corpus again. Covered faces must be unchanged (zero
   lost faces); Readings may only decrease. Any lost face is a defect: fix it
   within the ticket first; STOP and report only if the fix fails or needs a
   ruling.
5. Gate: `cargo xtask gate --changed --from <claim change id> --run`; clippy
   and fmt clean; `cargo xtask cite check --list-noncompliant` reports 0 from
   the workspace root.

## Landing requirements

Standard PROVE/DISCLOSE/REPORT (see `CLAUDE.md`), plus:

- every deleted lexeme, grouped by category, with its zero-token evidence;
- before/after covered-face and Readings counts, each stamped with its change
  id;
- test counts: restored, re-spelled, ignored, added, removed. A test that named
  a deleted lexeme is re-spelled against a live lexeme with the same asserted
  outcome, never deleted;
- timings as integers in nanoseconds (e.g. `30,000,000,000 ns`), never decimal
  seconds.

## Out of scope

Adding vocabulary, changing any licence on a surviving lexeme, and grammar
changes beyond removing dead references.

## Landing record

Measured 2026-10-06 on `wnotnlnlsqmvwzyzuotwozvtsumwttzz` / covered **13,716**, claimed at `vulpnokt`.
Baseline and final snapshots have the same change id and covered count; their
lexical inventory hashes distinguish the two trees:

- Before: `7e7d614939345927c3be26bba1e713c811bfe24134593ab68679707a8a0f17b7`.
- After: `d81c3e20a1cb37af46058a5bda5f1f75022cafc0e5984ae05e802e40d057ad9f`.

### PROVE

Full `cargo xtask english-v3 --all --workers 12 --samples-per-face 0`
censuses completed both before and after, over the identical 32,828 supported
face identities. Covered faces **13,716 → 13,716**; complete Readings
**150,324 → 150,324**. Each count is stamped with `wnotnlnlsqmvwzyzuotwozvtsumwttzz` / covered **13,716** and the
corresponding inventory hash above. The exact per-face census, Reading count,
construction occurrence counts and issue lists are identical for every face.
Lost faces, newly covered faces, retired wrong Readings and regressions:
**none**. This change only removes lexical alternatives and preserves all
surviving grammatical declarations, so equal complete per-face counts also
exclude silent Reading loss. No exclusion is deferred.

Both complete runs report **0 issues, 0 failed/limited/undetermined faces,
0 internal failures**. Every enumerated Reading passes admission, lexical
ownership, byte-exact realization and node/word traversal identity. The existing
independently constructed-value tests remain in the reverse-dependency gate;
the corpus command itself does not prove the converse roundtrip law.

The production loader and analyzer counted all declared inflections and
spelling variants, deduplicating `(Lexeme, start, end)` per face and field.
The census inspected all four lexicon files, 380 normalized Lexemes across
14 Categories, plus 28 authored vocabulary entries which the loader reports as
unmapped; none of those 28 has zero tokens. After pruning, 335 normalized
lexicon Lexemes remain, all with at least one supported occurrence.

Ordinary vocabulary tokens exclude complete declared name atoms. All forms
of each deleted Lexeme have **0 ordinary rules-text tokens and 0 Type Line
tokens**. Exact card-name identities remain present. Raw overlapping name/label
matches for six deleted ordinary Lexemes were individually inspected:

| Lexeme | Overlapping occurrences | Attestation belongs to |
|---|---:|---|
| against | 2 | Slime Against Humanity's full name, twice |
| through | 4 | Travel Through Caradhras; Joust Through; Peer Through Depths and Reach Through Mists in Sift Through Sands |
| within | 2 | Burn from Within; Worlds Within Worlds |
| away | 2 | Burn Away; Cast Away Doubt |
| together | 1 | Together as One |
| me | 3 | Me, the Immortal's nickname, twice; Jason Bright, Glowing Prophet's flavor heading “Come Fly With Me” |

The complete corpus lexical-alternative count falls from **1,693,752** to
**1,693,738**, exactly the 14 overlapping substrings listed above, on the two
stamped trees. No successful Reading uses those ordinary-word alternatives.

The flavor heading is an opaque label, not an ordinary pronoun use. All other
deleted entries have zero raw overlapping matches as well. Symbol counts cover
rules-text notation and Type Lines, the fields analyzed here; printed mana-cost
metadata is not parser input. Reminder text is stripped under the production
input policy. The scratch census uses lexical occurrences, including those on
uncovered faces, rather than counting only successful Readings.

Frankenstein's Monster retains the coordinated notation “a +2/+0, +1/+1, or
+0/+2 counter”. It has no contiguous `+2/+0 counter` Word Form occurrence;
only that zero-token compound Lexeme is removed. The generic notation and
`lexeme:CommonNoun/Counter` remain, and the face's census is unchanged.

The normalized declaration comparison removes exactly the 45 identities below
and adds **0 identities**. Every surviving Category, feature, frame, form,
capitalization, binding, surface structure and onset remains identical. The
sole provenance change is `vocab:Supertype/World` moving from `vocabulary.rs`
to `core.ron`; its ordinary Adjective analysis is unchanged. Removing World
from the negative formation class retires only the unattested `nonworld`.

Forbidden word/card/lexeme-named admission guards added: **0**. No runtime
checker or parser code changes. Lexical-source and GrammarEnvironment loading
succeed with **0 errors**. The active v3 stack has no legacy `environment.rs`
or emitted permitted-licensing-checker total; those retired-pipeline figures
are unavailable, not invented. No legacy coverage gate was run.

### DISCLOSE

| Tree, stamped with the change id above / covered 13,716 | No Reading | Unique | Multiple | Complete Readings |
|---|---:|---:|---:|---:|
| Before, baseline inventory hash above | 19,112 | 6,421 | 7,295 | 150,324 |
| After, final inventory hash above | 19,112 | 6,421 | 7,295 | 150,324 |

Specificity-resolved selections: **0** before and after. The parser retains
all Readings; no construction pair changes preference or admission. Newly
covered identities and selected analyses: **none**.

Every deleted Lexeme follows, grouped by Category. Each row has zero ordinary
rules-text and Type Line occurrences across all its declared Word Forms;
only the six name/label-overlap rows above have raw matches. Evidence is
`/tmp/english-v3-dead-lexeme-audit/dead.json` and
`before-tokens-names.json`, derived from the stamped baseline report.

**Adjective (2)**

- `vocab:AttributiveAdjective/Precombat` — `precombat`; 0 rules-text, 0 Type Line tokens.
- `vocab:Supertype/World/non` — `nonworld`; 0 rules-text, 0 Type Line tokens.

**Adverb (3)**

- `vocab:Adverb/Away` — `away`; 0 rules-text, 0 Type Line tokens.
- `vocab:Adverb/Here` — `here`; 0 rules-text, 0 Type Line tokens.
- `vocab:Adverb/Together` — `together`; 0 rules-text, 0 Type Line tokens.

**Catalog (1)**

- `vocab:Supertype/Ongoing` — `ongoing`; 0 rules-text, 0 Type Line tokens.

**Determinative (7)**

- `vocab:Determinative/Enough` — `enough`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/Few` — `few`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/Little` — `little`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/Several` — `several`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/Such` — `such`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/These` — `these`; 0 rules-text, 0 Type Line tokens.
- `vocab:Determinative/What` — `what`; 0 rules-text, 0 Type Line tokens.

**Noun (1)**

- `lexeme:counter_kind_numeric/P2P0/compound-noun` — `+2/+0 counter`; 0 rules-text, 0 Type Line tokens.

**Preposition (4)**

- `vocab:Preposition/Against` — `against`; 0 rules-text, 0 Type Line tokens.
- `vocab:Preposition/Because` — `because`; 0 rules-text, 0 Type Line tokens.
- `vocab:Preposition/Through` — `through`; 0 rules-text, 0 Type Line tokens.
- `vocab:Preposition/Within` — `within`; 0 rules-text, 0 Type Line tokens.

**Pronoun (11)**

- `vocab:IndefinitePronoun/Anything` — `anything`; 0 rules-text, 0 Type Line tokens.
- `vocab:IndefinitePronoun/Nothing` — `nothing`; 0 rules-text, 0 Type Line tokens.
- `vocab:IndefinitePronoun/Something` — `something`; 0 rules-text, 0 Type Line tokens.
- `vocab:ObjectPronoun/Me` — `me`; 0 rules-text, 0 Type Line tokens.
- `vocab:ObjectPronoun/Us` — `us`; 0 rules-text, 0 Type Line tokens.
- `vocab:PossessiveAbsolutePronoun/Hers` — `hers`; 0 rules-text, 0 Type Line tokens.
- `vocab:PossessiveAbsolutePronoun/Theirs` — `theirs`; 0 rules-text, 0 Type Line tokens.
- `vocab:ReflexivePronoun/Herself` — `herself`; 0 rules-text, 0 Type Line tokens.
- `vocab:ReflexivePronoun/Themself` — `themself`; 0 rules-text, 0 Type Line tokens.
- `vocab:ReflexivePronoun/Themselves` — `themselves`; 0 rules-text, 0 Type Line tokens.
- `vocab:SubjectPronoun/We` — `we`; 0 rules-text, 0 Type Line tokens.

**Symbol (15)**

- `vocab:FixedCostSymbol/ColorlessHybridBlack` — `C/B`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/ColorlessHybridBlue` — `C/U`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/ColorlessHybridGreen` — `C/G`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/ColorlessHybridRed` — `C/R`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/ColorlessHybridWhite` — `C/W`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianBlackGreen` — `B/G/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianBlackRed` — `B/R/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianBlueBlack` — `U/B/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianBlueRed` — `U/R/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianGreenBlue` — `G/U/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianGreenWhite` — `G/W/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianRedGreen` — `R/G/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianRedWhite` — `R/W/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianWhiteBlack` — `W/B/P`; 0 rules-text, 0 Type Line tokens.
- `vocab:FixedCostSymbol/HybridPhyrexianWhiteBlue` — `W/U/P`; 0 rules-text, 0 Type Line tokens.

**Verb (1)**

- `core-verb:WouldContracted` — `'d`; 0 rules-text, 0 Type Line tokens.

Deviations and additions:

- No Construction, schema, Category, runtime implementation, xtask interface,
  fixture or tracked audit tool was added or deleted. `verbs.ron` and
  `overrides.ron` need no change: every Lexeme they own is attested.
- Removed the dead Lexemes' feature additions and form replacements, their
  authored vocabulary members, the native `+2/+0 counter` compound member and
  World's negative class membership. No plugin frame refers to these owners,
  so no `plugins_v2` declaration or body changes.
- Relocated the live World declaration to retain precisely its original
  Adjective Category without generating a dead negative. Its source path now
  names the actual declaration file.
- Updated two grammar tests that named *because*: the positive clause-taking
  PP uses Psychosis Crawler's exact “whenever you draw a card” constituent, and
  the negative locative-frame witness uses the same live preposition.
- Re-spelled the compositional positive and two number-agreement negatives
  from *these* to live *those*.
- Re-spelled the unaffected-pronoun value test with live *his* and *himself*;
  it keeps complete case/person/number/capitalization value-set comparisons
  and keeps the three original plural-pronoun cases unchanged.
- Re-spelled the native-counter inventory test for the ten live compounds;
  every remaining exact singular/plural value, onset, realization and
  analysis assertion remains. The unattested `+2/+0 counter` member is retired.
- Re-spelled the supertype value test for all four positive adjectives and
  the three live negative adjectives. World retains its exact independent
  positive values; the zero-token negative is retired. All category, owner,
  capitalization, realization, analysis-set and Legendary uniqueness checks
  remain.

Test counts: **restored 0; re-spelled 8; newly ignored 0; added 0; removed 0**.
No test function or assertion is deleted or weakened. The gate retains the
one pre-existing ignored test,
`macros::templates::tests::macro_schema_census_count_matches_21`, with its
unchanged reason “cross-checks the live corpus against the census; run on
demand”; new ignored tests: 0. STOPs: **none**; glossary gaps: **none**. The current pruning
ruling supplies the authority to remove unattested Lexemes; no surviving
licence is narrowed.

### REPORT

The active v3 command has no legacy coverage lock. Its complete measured
covered count is **13,716** before and after, stamped with the change id and
inventory hashes above. Legacy tracked coverage data is unchanged and is not
v3 admission authority. On both stamped trees: **179 ordinary Constructions +
44 shared schemas = 223 constructor names**, **135 Categories**, **545 static
Productions**, **611 environment-compiled Productions**.

The declared Word Form homograph inventory excludes exact Catalog name/type
atoms and combines owners at declared capitalization. It includes notation,
so `X`'s Symbol and Numeral owners are reported. Before: **122 surfaces**;
after: **120 surfaces**, retiring only `'d` and `’d`. Full named owner lists
are in `before-inventory.json` and `after-inventory.json` in scratch space.
Final named surfaces:

`'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `X`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `instead`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’s`, `∞`.

Form-literal/vocabulary overlap inventories: **none**, before and after,
measured from actual environment-compiled Production literals and declared
Word Forms. These are inventories, never admission gates.

Performance advisory, on the two stamped trees above (12 workers each):

- Before: command wall **35,482,192,772 ns**; corpus wall **28,495,988,408 ns**; checked-text thread CPU **161,251 ns/B**; host load `[4.84814453125, 6.82763671875, 4.01904296875]`.
- After: command wall **39,007,916,037 ns**; corpus wall **31,821,396,571 ns**; checked-text thread CPU **176,977 ns/B**; host load `[1.25, 4.00634765625, 3.78515625]`.

Both exceed the quiet-host advisory ceiling of **16,260,000,000 ns**.
The baseline overlapped a scratch census process; the final corpus phase
started before the gate, whose compilation began near its end. Neither is a
quiet-host benchmark. Performance remains advisory, owned by
`english-v3-census-tractability`.

All audit programs and full evidence remain in
`/tmp/english-v3-dead-lexeme-audit/`; no process artifacts enter source or
version control. The reports include original source, analyzed input, durable
face identities, complete counts, traversal checks and timing telemetry.

Verification:

- Full baseline and final corpus commands exit 0 with the census above.
- `cargo fmt --all -- --check` exits 0.
- `cargo xtask gate --changed --from vulpnokt --run --clippy` exits 0.
  Derived test command:
  `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
  All selected suites, including xtask's real Lean card/definition integration
  checks, pass. Strict clippy for the same four packages with
  `--all-targets -- -D warnings` exits 0. Log: `gate-complete.log` in scratch.
- The built xtask's final `cite check` reports 16,059 checked citations,
  **0 stale**; `cite check --list-noncompliant` reports **0 non-compliant**.
- `cargo xtask cite check --list-noncompliant` exits 0 and reports
  0 non-compliant citation-looking strings.
