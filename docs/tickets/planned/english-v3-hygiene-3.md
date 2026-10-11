---
needs: []
---
# Hygiene 3: review observations from waves 5–9

Priority: LOW.

## Why

Review-time observations from the wave 5–9 landings (2026-10-08 … 2026-10-10)
that were recorded only in ignored orchestration notes or in the landing
records of done tickets, with no live owner. No coverage gain is expected;
several items lower Reading counts or remove a duplicate label. Change ids and
counts are those of the notes; anything marked "from review notes, unverified"
has not been probed at the current trunk.

## Items

### A. Overgeneration and duplicate-Reading candidates

- **A1. Attributive-only adjectives read predicatively.** Ruling V9
  (`english-v3-vocabulary-batch-1`, `ukrwksourqxq`) deferred this:
  `vocab:AttributiveAdjective` members pass through the generic
  `AdjectivalComplement`, so "Your hand is opening." and "This damage is
  excess." read 1 each. Smallest fix: a declared attributive-only feature on
  the class, read by `AdjectivalComplement`; confirm zero attested predicative
  uses by corpus grep over every member surface first.
- **A2. Mass-*time* twin under *the*.** Every "the next/first time" face
  carries two Readings with the same tree, differing only in countability
  (`english-v3-next-time-relative-adjunct`, `szkxvvylquyv`; inherited from
  `NounPhrase`). The review correction `stymwqqo` removed the mass analysis of
  *each time* only. OWNER DECISION: should a temporal-location NP require
  Count?
- **A3. "Exile target card type."** Noted in the review of
  `english-v3-card-noun-premodifier` (`ktvsnxsr`): the *card* premodifier gate
  on heads type/name/pool admits this unattested sentence. From review notes,
  unverified. Probe; if it reads, restrict by a declared feature of the host
  position, not of the words.
- **A4. Clause-final predicative *as* on unattested hosts.**
  `english-v3-clause-final-predicative-as` (`qvurzynu`): `PredicativeAdjunctUse`
  is Yes only on type/sorcery and type/instant, yet "Destroy target creature
  as a sorcery." reads. Zero increases on covered faces. Open question: is a
  host restriction wanted, or is this accepted reach under the pruning rule?
  OWNER DECISION.
- **A5. Negative common nouns head an NP.** *nontokens* / *nonlands* inherit
  the full noun paradigm (`english-v3-negative-common-noun-premodifier`,
  `xlnqpvsq`); flagged to the owner as a possible "premodifier-only" property.
  Probe attestation of the head use; retire it under the pruning rule if zero.
- **A6. Quoted ability with the stop outside the quote.** Ruling QA7
  (`english-v3-quoted-ability-coordination`) left the unattested string
  `It has "{T}: Draw a card".` open. Decide whether it should read 0 and add
  the negative, or record it as accepted.
- **A7. *first* is two lexemes.** *first* reads as
  `vocab:AttributiveAdjective/First` and as an Ordinal numeral
  (`OrdinalPremodifier`): a ×2 lexical duplicate on Eluge and Saint Traft.
  Owner wish (2026-10-09): an ordinal is the numeral's ordinal form with
  adjective distribution, one lexeme each; retire the second label.
  `english-v3-fused-ordinal-head` and
  `english-v3-temporal-pp-and-premodifier-scope` both name the split as out of
  their scope. Note ruling R14e (`english-v3-last-turn-temporal-adjunct`):
  *last* is not a numeral and was rehomed as a Word.

- **A8. Woodcaller Automaton's toughness-only equality.** Pruning candidate: the pre-existing `MeasuredAttribute(head: Noun(Toughness))` Readings attach *equal to this creature's power and toughness* to *toughness* alone instead of the intended coordination *power and toughness* (12 full-ability Readings, 6 isolated-sentence Readings on `svlsnsmv`); retire this wrong attachment through a declared coordination-context feature, preserving the correct coordinated-head Readings, legitimate single-property equality, and independent scope ambiguity.

- **A9. Have admits a nominal predicative Complement.** `english-v3-independent-genitive-pronoun` (IG8) leaves the existing Object + nominal predicative Complement frame unchanged; its shape splits *the chosen creature type in addition to yours | other types*. CGEL Ch. 4 §5.4, p. 265 labels complex-transitive *have* adjective-only; verify and repair the general frame licensing separately.

- **A10. Dawnglow Infusion reads through the wrong coordination.** `english-v3-mana-symbol-passive-subject` (`onymknyk`) exposes 18 Readings whose *cast* Object includes *this spell and X life*, rather than the intended gapped *gain* coordination; not counted as a coverage gain. Supply the gapped gain analysis before considering pruning of the inherited nominal/purpose routes.

### B. Lexicon and feature economy

- **QS7 quantifier policies.** `english-v3-quantified-mana-symbol` preserves ordinary determiner-licensing rows in a separate number-neutral quantifier policy; table composition could share those rows without broadening ordinary noun licensing.

- ScalarLocationPredicate carries NominalAdjunctHost and ClauseComplementBoundary to reuse the auxiliary schema (english-v3-scalar-location-plain-be-and-become); its current clause assembly does not consume them. Consider category-specific boundary projections if the schema compiler gains them.

- **B1. Two spellings of definiteness.** The noun-side determiner selection
  of `english-v3-fused-head-noun-phrases` (ruling FH7) reads the
  Determinative's `NominalAdjunctDependent = Definite`; trunk also has a
  `Definiteness` feature from `english-v3-multiplier-predeterminer`. Unify the
  two and collapse the doubled `(…, No, _)` / `(…, Yes, _)` rows of
  `determiner_license`. That landing record routes this to
  `english-v3-licence-plumbing-economy`, which does not list it.
- **B2. `NounPhraseStructure = IndefiniteArticle` beside a
  `Definiteness = Nondefinite` check.** From the review of
  `english-v3-named-token-apposition` (in flight; new feature
  `NounPhraseStructure { Other, ProperName, IndefiniteArticle }`): the two
  checks look redundant. From review notes, unverified. After it lands, show
  one row where they differ or drop one.
- **B3. Supertype word under two owners.** From the review of
  `english-v3-supertype-premodified-bound-keyword` (in flight; ruling SB7
  extends `TypeLineRole` to the owning Adjectives): the Adjective and the
  `catalog:supertypes.txt/*` Type Line Catalog atom may be two lexemes for one
  word. From review notes, unverified. After it lands, decide whether one
  owner can serve both the Type Line and rules text.
- **B4. Perfect has no `SelectedGapLocativePredicate` counterpart.** "each
  +1/+1 counter you've put on …" (4 faces) stays unread after
  `english-v3-clitic-have-object-relative`. Extend the existing route to the
  perfect; no new Construction if the auxiliary row can be shared.
- **B5. `ObjectRelativeClause` hosts a non-Object gap; *turn* lacks the
  class.** Follow-ups of `english-v3-perfect-relative-adverbial-gap`
  (`otzuyrluxpsw`): rename the Construction to match what it hosts (glossary
  first), and declare `RelativisableAdjunctClass` on *turn* ("During any turn
  you attacked with …" is attested; only *time* declares it today).
- **B6. Champion's article is not in its spelling (CH3).**
  `english-v3-champion-keyword-line` (`qzrvqqol`): `champion.ron` keeps
  `spelling: "champion <Param(0)>"`; `SpellingHint` supports only `plural` and
  `word`, so the realised keyword line drops the article. Needs an article
  hint in the spelling machinery, or a recorded decision that grammar-side
  `KeywordIndefiniteArticle` is the only owner.

- **B7. Mana passive head carries unused selected-head exports.** `english-v3-mana-symbol-passive-subject` (`onymknyk`) reuses `SelectedVerbHead`, whose Category interface also requires `FrameUse`, `HeadCoordination`, `AuxiliaryEllipsisLicence` and `AuxiliaryProFormLicence`; the current Mana passive Subject route does not consume these four exports. Evaluate schema/interface economy without changing retained Readings.

### C. Tests and tooling

- **C1. Magic-number and Debug-tree assertions.** Owner complaint of
  2026-10-09 after `english-v3-per-line-readings`: tests pin numbers such as
  `total_cost == 19` (xtask `tests.rs`, `cheapest_frequency_sample_…`) and
  scan Debug-printed trees for substrings. Sweep `xtask` and
  `deckmaste_english_v3` tests; re-spell each as a relational assertion (the
  sample is the cheapest Reading) or a typed-value comparison. Same subject,
  same outcome; none deleted.
- **C2. Census memory on huge single lines.** After per-line Readings the
  remaining RSS is per-line materialisation (Necromentia 43,560 Readings;
  Tetsuo 4.3 GB at one worker). Suspected holder: the `Tracing` build memo,
  `crates/xtask/src/english_v3/validation.rs:143` (`MEMO_CAP 100_000`,
  deep-cloned child values at `:239`); secondary `Readings::sibling`,
  `readings.rs:150`. Unconfirmed (no heap profile). Measure first.
- **C3. Blocker report hides multi-line failures.** `cargo xtask english-v3
  blockers` ranks sole-blocker lines, so shapes that fail on several lines of
  one face (the modal headers were found by hand during wave-8 drafting) do
  not surface; end-of-sentence position keys (`. <end>`, `turn .`) dominate
  the `at:` table and say little. Open question: a touched-faces column, and a
  key that looks one token further back at a sentence end.
- **C4. Bounded manifests miss retired analyses.** `english-v3-named-modes`
  (`pxtwrtvu`) lost 10 faces outside its 640-face manifest; fixed by
  `lqmpuwls`. The lesson (a change that retires an analysis must select every
  face containing the surface form) lives only in the dispatch brief. Put it
  in `docs/decisions/english-lexical-analysis.md` or the census tool's help.

Chart-item budget in selected_complement_clusters::bounded_roundtrip: single-Reading fixtures grew from <5,000 (original comment) to 7,684–11,895 items by 2026-10-11; raised 12,000 -> 12,500 by english-v3-coordinated-participial-premodifier (+83..+122). Find which Constructions inflate the chart on these nine witnesses (heaviest: the artifact damage/gain fixture) and replace the magic ceiling with a relational guard. OWNER DECISION whether further raises are allowed.

2026-10-11, `english-v3-chart-dead-prediction-filter` (`swxynlzl` → `vlxnywvr`): the pre-refresh nine witnesses use 7,869–12,295 → 4,698–7,578 items and 8,051–12,618 → 4,880–7,901 families; predictions 6,843–10,716 → 3,672–5,999; `items - predicted_items` stays 1,026–1,579 and completed nodes stay 163–287. Diagnosis suggests a control ratio `items(text) <= 2.0 * items(text without the second cluster)`, or bounds on `items - predicted_items` and completed nodes; the existing 12,500 / 13,000 ceilings and assertions remain unchanged pending the owner decision. After refresh: maximum 7,581 items / 7,904 families, completed nodes unchanged.

- **C5. Selected Literal over an optional lexical field panics.** `english-v3-additional-slash-adjustment` (`zquvqlrm`, 2026-10-11): `separator article(present)` referenced by `article(modifier.present)` before `modifier: optional(lexical(Adjective), " ")` passes validation but proc-macro expansion hits `internal error: entered unreachable code`; diagnose the emitter and reject or implement this combination. The landing uses retained lexical articles instead.

### D. Scope and bookkeeping

- **D1. Duplicate ticket.** `planned/english-v3-comparative-property-premodifier`
  (wave-9 set B, `puwpvlrq`) duplicates
  `english-v3-attributive-scalar-comparative` (set A, in flight). When the
  latter lands, check the set-B ticket's 44 faces against it, move any
  remainder into a new ticket and retire the duplicate. Do not dispatch it.
- **D2. Planechase faces in the supported corpus.** Wave-9 set B drafting saw
  "Path of the …" faces with planeswalk / "chaos ensues" text among the
  unread faces (under the *tied* cluster). Planechase is permanently out of
  scope, so these should not count as supported. From review notes,
  unverified. Scope check: list them with `cargo xtask english-v3 grep`, find
  why the supported-face filter admits them, fix the filter and restate the
  denominator. OWNER DECISION if any is Vintage-legal.
- **D3. Undrafted clusters with no ticket.** From the wave-9 set C report and
  the wave-6 follow-up list, sole-blocker counts as noted, unverified:
  "except it's N/N" 13; "life total becomes N" 12; "each player votes for X or
  Y" 27; twice X / N times X + noun 13; "N or more times" 11; noun + *not* +
  participle 11; "each 1 life" 10; "returned this way" 10; "differently
  named" 10; keyword-line residue (Suspend X—, Splice onto Arcane—,
  cost-internal "Pay half your life, rounded up"). Route only: re-measure
  against a fresh blocker report and mint or discard; implement none here.

## Method

Standard constraints apply. Deltas:

1. Probe first. Each A and B item starts with a probe or corpus grep at the
   claim parent; an item whose observation does not reproduce is closed in the
   record with the probe, not implemented.
2. Split into separate landings by group (A, B, C, D), minting a ticket per
   group or per item when claimed; this file is the inventory. B2 and B3 wait
   for their in-flight tickets to land.
3. A and B items that change grammar or lexicon run a bounded census over
   every face containing the affected surface form plus 300 random covered
   faces: zero lost faces; every decrease attributed.
4. Items marked OWNER DECISION stop for the owner before any change.

## Landing record
