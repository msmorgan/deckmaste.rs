---
needs: []
---
# Read a keyword verb's past participle as a state adjective: You may play that card for as long as it remains exiled.

Probably data-only: one class row in `lexicon/core.ron`. Scoped as a full
ticket because of the Reading-count increase it brings under *be*.

## Why

*remain* and *become* take a predicative Complement, and a keyword verb's
participle is an Adjective only where declared. *It remains tapped.* reads 1;
*It remains exiled.* reads 0. Blocker report at trunk `uoqyuxtnowpo`: key
`at:exiled .` 26 sole-blocker faces, but the construction is wider than the
key. `cargo xtask english-v3 grep -i '\b(remains?|becomes?)
(exiled|crewed|attached)\b'` finds **92** faces, all unread (*remain(s)
exiled* 77, *becomes crewed* 4, *become(s) / remain(s) attached* 13); 93
failing lines match, **85** of them the only failing line of their face.

## Goal

A keyword-action or keyword-ability verb can export its past participle as a
participial Adjective, so the predicative Complement of *remain* / *become*
reads: *it remains exiled*, *they remain exiled*, *this Vehicle becomes
crewed*, *becomes crewed for the first time each turn*.

## Analysis

CGEL Ch. 16 §10.1.3, p. 1436: "*They were married.* [ambiguous]"; "in the
verbal interpretation it is dynamic, describing an event, while in the
adjectival interpretation it is static, describing the state resulting from
some prior event". P. 1437: "Adjectival predicative complements are not
restricted to occurrence with *be* but are found also with such verbs as
*seem*, *look*, and *remain*"; "Replacement of *be* by *remain* also serves to
remove the ambiguity of [iii] in favour of an adjectival reading". Those back
an Adjective after *remain*, and two Readings after *be*. Extending the test
to *become* and choosing the declaration mechanism are project rulings.
Glossary: **Adjectival Passive**.

Existing mechanism: `participial_adjective_classes` in
`crates/deckmaste_lexical_source/lexicon/core.ron` (`features:
{"AdjectiveOrigin": "Participial"}, form: PastParticiple, members:
["lexeme:keyword_action/tap", "lexeme:keyword_action/untap",
"core-verb:Block"]`), loaded by `native::add_participial_adjectives`, which
derives `<owner>/adjective`. *remains tapped* reads through `core-verb:Remain`
(`Predicate([PredicativeComplement])`) → `AdjectivalComplement` →
`lexeme:keyword_action/tap/adjective`. Precedent for the *be* pair: "It is
tapped." reads **2** today (adjectival Complement and `PassivePredicate`),
"It is exiled." 1 (passive only), "This Vehicle is crewed." 1 (passive of
`lexeme:keyword_ability/crew/verb`). Attributive use is not doubled: NP "the
tapped creature" 1, "the exiled card" 1.

Orchestrator rulings 2026-10-10:

- KS1. One mechanism: class membership. Add the exile and crew verbs to the
  existing class (data). No new Construction, no frame on *remain* / *become*
  that takes a passive, no hand-written Adjective homograph. If the loader
  rejects the crew verb (it is a `FixedKeyword` `verb:` export, not a `Verb`
  declaration), extending the class to that owner kind is in scope; STOP if
  it needs more than that.
- KS2. The licence is the declared feature `AdjectiveOrigin = Participial` on
  the derived lexeme and the existing predicative-Complement frames. No test
  on a verb, participle or card.
- KS3. Exactly one Reading: *it remains exiled*, *they remain exiled*, *this
  Vehicle becomes crewed*, *this creature becomes saddled* (unchanged, 1).
  After *remain* / *become* only the adjectival analysis exists (CGEL
  p. 1437); a passive Reading there is a defect.
- KS4. Two Readings, both kept: *it is exiled*, *this Vehicle is crewed*
  (adjectival Complement + verbal passive), as "It is tapped." has today
  (CGEL p. 1436 [32iii]). This is the same pair the keyword-verb-export
  landing deferred for *is saddled* (1 → 2 on adding the saddle verb,
  `english-v3-systemic-residuals`, "saddle and *becomes crewed*"): it is the
  CGEL ambiguity, not a duplicate. With this ruling the saddle verb export is
  unblocked; it stays out of this ticket (KS7).
- KS5. One analysis per string, the twins to avoid: (a) attributive *the
  exiled card*, *exiled cards* keep exactly their Readings (216 faces match
  `\bexiled (card|cards|creature|permanent|spell)`); (b) postnominal *cards
  exiled with this creature*, *exiled this way* gain no Adjective-Phrase twin
  of the participial postmodifier; (c) *exile it face down* and the
  ResultativeComplement frame are untouched; (d) *was crewed by two
  creatures* keeps its passive Readings (3 today): an Adjective with a *by*
  Complement is a new analysis, reported if it appears.
- KS6. Newly covered faces are gains. Expected increases on covered faces,
  exactly one shape: finite *be* + *exiled* / *crewed* gains the adjectival
  Reading (34 covered faces match `\b(is|are|was|were|be|been|isn.t|aren.t|
  wasn.t|it.s|they.re) exiled\b`; 0 covered faces for *crewed*, whose one
  *was crewed* face is unread). Report the
  count; any other shape is a defect to fix before landing.
- KS7. Out: declaring *saddle* as a verb; *attached* (KS8).
- KS8. *becomes attached to …* / *remains attached* (13 faces) joins only if
  the class member reads it with no further declaration: the Adjective then
  needs the verb's *to* Complement. Otherwise list the count and leave it.

## Witnesses

Probed 2026-10-10 on the `uoqyuxtnowpo` binary (`--readings 3 --as-card`).
Each line is its face's only failing line and reads 0; the control is the
same line with *tapped* for the participle.

- Gustha's Scepter: "{T}: Exile a card from your hand face down. You may look
  at it for as long as it remains exiled." 0 (*remains tapped* 3)
- Fiend of the Shadows: "… that player exiles a card from their hand. You may
  play that card for as long as it remains exiled." 0 (3)
- Savvy Trader: "… exile target permanent card from your graveyard. You may
  play that card for as long as it remains exiled." 0 (3)
- Jester's Scepter: "… You may look at those cards for as long as they remain
  exiled." 0 (3)
- Mobilizer Mech: "Whenever this Vehicle becomes crewed, up to one other
  target Vehicle you control becomes an artifact creature until end of
  turn." 0 (*becomes tapped* 3)

Sentence probes: "It remains exiled." 0; "They remain exiled." 0; "This
Vehicle becomes crewed." 0; "This Vehicle becomes crewed for the first time
each turn." 0 (*becomes tapped …* inside a *whenever* clause 3). Controls
that must not change: "It remains tapped." 1; "It is tapped." 2; "This
creature becomes saddled." 1; "This creature is saddled." 1; "It remains on
the battlefield." 1; "You may play that card for as long as it remains in
exile." 2; NP "the exiled card" 1. Controls that change by KS4 only: "It is
exiled." 1 → 2; "This Vehicle is crewed." 1 → 2.

## Method

Standard constraints apply. Lexicon data loads at runtime: no rebuild for the
class row. Bounded census, never `--all`: every supported face matching
`\b(exiled|crewed)\b` (608 faces) plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; KS6 for
increases, each classified by shape. Full `cargo test -p` and `cargo clippy
-p … --tests -- -D warnings` for `deckmaste_lexical_source` and
`deckmaste_english_v3` once, at the end (a rebuild after a grammar edit takes
about 3 minutes, the test binary about 7). Tests assert the role (the
Complement of *remain* / *become* is the derived Adjective; the KS3 and KS5
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only. On landing, close the "saddle and
*becomes crewed*" entry of `english-v3-systemic-residuals` or rewrite it to
what remains.

## Out of scope

- *Whenever this creature attacks while saddled, …*: verbless *while* +
  Adjective. 26 faces, all unread; "… attacks while tapped, …" reads 0 too, so
  it is not an adjective gap. Own ticket.
- *becomes unattached* (4 faces), *stays* (unknown word), *becomes suspected*
  (1), *remains unchanged* (1).
- *it was crewed by exactly two creatures* (*exactly* is an unknown word).
- *becomes plotted* / *prepared* / *renowned*: declared adjectives, covered.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
