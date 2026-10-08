---
needs: []
---
# Read a negative-prefixed common noun premodifier: Destroy all nontoken creatures.

## Why

*nontoken* never reads. On census `xnkuznpo` (docs-only descendant of
`vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369 unread; wave-4
recon of 2026-10-08), the recon bucket "*nontoken*" touches **324** unread
faces and is the sole cause on **224**. No text the grammar reads contains
*nontoken*. Counts are unread faces *touched* / *sole* as defined in the
recon; surface counts, not gain forecasts.

Probes (admitted roots): "Destroy all nontoken creatures." 0, while "Destroy
all nonland creatures." 1; "Exile target nontoken creature." 0, while "Exile
target nonartifact creature." 1. The lexer finds `vocab:NegativePrefix/Non`
plus `lexeme:CommonNoun/Token`; no single negative Lexeme exists.

## Goal

*nontoken* reads as a negative noun premodifier of a following Nominal
(*nontoken creatures*, *another nontoken Spirit*, *nontoken permanent*),
realised byte-exactly, with the same lexical shape type words already have
(*nonland*, *nonartifact*).

## Analysis

CGEL, Ch. 19 §5.5.1, p. 1687 lists *non·* among the negative prefixes. CGEL,
Ch. 6 §2.4.1(a), p. 537: nouns function as attributive modifiers in a
nominal.

Existing route: negative noun Lexemes are minted by `negative_lexeme`
(`crates/deckmaste_lexical_source/src/native.rs:233`), called only for type
words with `NounPremodifier` (`legacy/plugins.rs:84`, per the 2026-10-04 ADR
amendment on type macros). `lexeme:CommonNoun/Token` (`vocabulary.rs:419`)
is a common noun, outside that recipe. The consumer is `NounPremodifier` /
`NounPremodifiedNominal` (`declarations.rs:2232,2275`). This ticket declares
the negative formation for the common noun *token* through a declared
property, not a spelling rule, and reuses that consumer. The 2026-10-04 ADR
amendment names type and subtype declaration classes as the sources of
negative noun Lexemes; a common noun is outside both. If declaring the
formation on *token* reads as contradicting that amendment rather than
extending it, STOP for a ruling.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Hour of Reckoning: "Destroy all nontoken creatures." (control "Destroy all
  nonland creatures." 1)
- Swift Spiral: "Exile target nontoken creature." (control "Exile target
  nonartifact creature." 1)
- Rhythm of the Wild: "Nontoken creatures you control have riot." (control
  "Nonland creatures you control have riot." 1)
- Isilu, Carrier of Twilight: "Each other nontoken creature you control has
  persist."
- Thraben Watcher: "Other nontoken creatures you control get +1/+1 and have
  vigilance."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A *non* read as a separate word, or *nontoken* read as the head noun of its phrase, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *permanent* as a noun premodifier (`english-v3-permanent-noun-premodifier`).
- Positive *token* as a premodifier (*token creature*) unless the corpus
  attests it on a face this ticket covers.

## Landing record

1. **Census delta.** Original claim parent `sktuzntk`, reconstructed from the source-equivalent full census `xnkuznpo` and refreshed subset `tvsnunmn`: **23,460 covered faces / 1,605,559 Readings**. The one feature full census `ozrxmvqo` yields **23,658 / 1,672,796**, gaining **198 faces / 67,237 Readings**. After punctuation refresh, the final reconciliation (`xnoqplzq`) has the same totals: trunk full census `xtxrnswz` plus the **324-face** refreshed selector `xnoqplzq`. Lexeme fingerprint comparison proves `Token/non` is the sole lexical difference from trunk and the selector covers every face containing its declared forms (`xnoqplzq`). Lost faces and removed Readings: **none**; every previously covered face retains its count (`xnoqplzq`). The **1,981** inherited digest changes are attributed to punctuation consolidation `xtxrnswz`; no unexplained digest changes or inherited decreases. Complete admission, exact realization and node/word traversal pass with **0 issues / 0 internal failures** (`ozrxmvqo`, `xnoqplzq`).

2. **Witnesses.** Final full-face Reading counts (`xnoqplzq`): Hour of Reckoning **1**; Swift Spiral **9**; Rhythm of the Wild **4**; Isilu, Carrier of Twilight **12**; Thraben Watcher **6**. Each failing-unit sentence also passes its structural and byte-exact regression test (`opkzlsyv`).

3. **Spot checks.** Ten newly covered faces, inspected again in retained refreshed samples (`xnoqplzq`); *nontoken* is a singular `NounPremodifier` with owner `lexeme:CommonNoun/Token/non` throughout:

   | Face | Selected analysis |
   | --- | --- |
   | Hour of Reckoning | Modifies plural *creatures* under *all*, the Object of *Destroy*. |
   | Infernal Tribute | Modifies singular *permanent* under *a*, the Object of the sacrifice cost. |
   | Incandescent Aria | Modifies singular *creature* under *each*, inside the *to* Complement. |
   | Anafenza, Kin-Tree Spirit | Modifies *creature* in the trigger Subject *another nontoken creature you control*. |
   | Metastatic Evangel | Modifies *creature* in the trigger Subject, with *you control* as a relative restrictor. |
   | Surge Conductor | Modifies *artifact* in the trigger Subject, with *you control* as a relative restrictor. |
   | Harvester of Souls | Modifies *creature* under *another* in the Subject of *dies*. |
   | Grim Haruspex | Modifies *creature* in the Subject of *dies*, retaining the *you control* relative. |
   | Rishadan Pawnshop | Modifies *permanent* in the target Object of *Shuffle*, retaining the relative restrictor. |
   | Wirewood Hivemaster | Modifies the subtype noun *Elf* under *another* in the trigger Subject. |

4. **Deviations and STOPs.** The native negative-formation declaration and noun/singular validation extend the type/subtype recipe under the [advance orchestrator ruling of 2026-10-08](../../decisions/english-lexical-analysis.md#lexical-analysis), now recorded beneath the 2026-10-04 amendment. This ticket adds or retires no Construction or frame: the existing noun premodifier consumer suffices, and no superseded affix route exists in the grammar. Added an independently constructed nominal test and a lexical paradigm/provenance test for both roundtrip laws and retained noun identity (`opkzlsyv`). No Analysis STOP or glossary gap. The final refresh inherited punctuation grammar/compiler/lexicon changes (`xtxrnswz`); its `tests/main.rs` conflict was resolved by retaining both test modules. Refreshed suites and selector evidence verify the composition (`xnoqplzq`), without another full run.

5. **Tests.** Initial `cargo test -p deckmaste_english_v3`: **501 passed**; `cargo test -p deckmaste_lexical_source`: **58 passed**; `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`: passes (all `opkzlsyv`). Refreshed `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source -p deckmaste_construction_v3_core`: **504 / 58 / 27 passed**, respectively; `cargo clippy -p deckmaste_construction_v3_core -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`: passes (all `xnoqplzq`). Own assurance counts (`xnoqplzq`): restored **0**, re-spelled **0**, ignored **0**, added **7**, removed **0**. English integration tests use `tests/main.rs` → `tests/suite/`. The lowercase/capitalized selectors (`opkzlsyv`), single full census (`ozrxmvqo`) and refreshed selector (`xnoqplzq`) pass. Evidence stays under ignored `target/english-v3/english-v3-negative-common-noun-premodifier/`.

6. **Timing.** The one full census (`ozrxmvqo`): **224.171 seconds**, **12 workers**.
