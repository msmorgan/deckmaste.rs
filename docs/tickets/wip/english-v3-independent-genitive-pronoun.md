---
needs: []
---
# Read the independent genitive pronoun as a Noun Phrase: During turns other than yours, creatures you control get +0/+2.

## Why

*yours* is a declared Lexeme with no route: it reads in no position. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key
`at:than yours` 10 sole-blocker faces. `cargo xtask english-v3 grep -i
'\byours\b'` finds **10** faces, all unread, all *During turns other than
yours, …*, each the only failing line of its face. Probes: "turns other than
yours" (`noun-phrase`) 0; "turns other than your turn" 1; "yours" 0; "That
permanent is yours." 0.

Small, but it is one mechanism and the sole blocker of all ten.

## Goal

An independent genitive pronoun heads a Noun Phrase with no overt noun
(*yours* = *your turn(s)*) where an accusative Noun Phrase is admitted, here
as the Complement of *than* after *other*, with byte-exact realisation.

## Analysis

CGEL Ch. 5 §9, p. 410, [3]: "DEPENDENT *my your her our their no* /
INDEPENDENT *mine yours hers ours theirs none*"; "The dependent forms are
used in (pure) determiner function, the independent ones in head function.
The independent genitive pronouns occur in fused heads and also pure heads".
That backs *yours* as a head-function form making an NP by itself. Which
positions Oracle English uses is a project ruling (one is attested).

Existing declarations: `vocab:PossessiveAbsolutePronoun/Yours` and `/His`
(`crates/deckmaste_lexical_source/lexicon/vocabulary.rs`; forms in
`lexicon/core.ron` with `person: Second, number: Singular, case: Genitive`).
Routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`AccusativePronoun` (`NounPhrase`, `[head: lexical(Pronoun)]`) for *you* /
*it* / *them*; `FusedDeterminerHeadNounPhrase` (`[head:
lexical(Determinative)]`, `require head.FusedHeadUse = Yes`) for *this* and
the landed *the other* / *the rest* fused heads; the *other than* route
(*turns other than your turn* 1, *creatures other than this creature* 1).
Glossary: **Fused Determiner-Head**, **Fused Head Use**, **Genitive**.

Orchestrator rulings 2026-10-10:

- IG1. Refine an existing pronoun-headed NP Construction: `AccusativePronoun`
  gains the independent genitive as a second admitted pronoun class, or
  `FusedDeterminerHeadNounPhrase` admits it, whichever keeps one
  Construction and reads the declared `case: Genitive` plus a declared
  independent-form feature. No new Construction.
- IG2. The licence reads the declared features of the form (independent,
  genitive); no test on *yours*. The dependent genitives (*your*, *their*)
  must not qualify: they lack the independent-form feature. *his* is both
  (CGEL lists no distinct independent form for it); its independent Lexeme
  `PossessiveAbsolutePronoun/His` qualifies by the same feature and its
  dependent Lexeme does not.
- IG3. Number and person of the resulting NP: third person, number
  unspecified by the pronoun (the understood head decides, and nothing in
  the attested position agrees with it). Do not export the pronoun's second
  person: *yours* is not an addressee NP. If an agreement site forces a
  choice, STOP with the site.
- IG4. One analysis per string. The twins to avoid: (a) *his* before a noun
  (*his controller*, if any covered face has it) keeps the dependent Reading
  only; the independent form takes no following Nominal; (b) *turns other
  than your turn* keeps its 1 Reading; (c) *yours* is not also a predicative
  Adjective.
- IG5. Position: any position an accusative NP fills comes for free and is
  not blocked. Predicative *is yours* (0 faces) need not read and gets no
  dedicated work.
- IG6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *yours*; covered faces with
  *his* (count them: `(?i)\bhis\b`) must keep their counts under IG4(a).
  Report any increase with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Mesa Lynx: "During turns other than yours, this creature gets +0/+2."
- Geyser Drake: "During turns other than yours, spells you cast cost {1}
  less to cast."
- Midnight Mangler: "During turns other than yours, this Vehicle is an
  artifact creature."
- Glory of Warfare: "During turns other than yours, creatures you control
  get +0/+2." (sentence; control "During turns other than your turn,
  creatures you control get +0/+2." 1; "During your turn, creatures you
  control get +0/+2." 1)

Controls that must not change (`noun-phrase`): "turns other than your turn"
1; "creatures other than this creature" 1; "your devotion" 1. The matching
main-clause controls for Mesa Lynx, Geyser Drake and Midnight Mangler were
not probed (the trunk moved under the prebuilt binary); probe them on the
claim parent.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(yours|his)\b|\bother than\b` (191 faces) plus 300
random covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; IG6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 2–3 minutes and the six test binaries about 75 seconds; lexicon
data loads at runtime. Tests assert the role (the independent genitive as
head of the NP that is the Complement of *than*, the IG4 negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only. Glossary: add **Independent Genitive** (CGEL Ch. 5
§9, p. 410) in the landing; the Lexeme class name `PossessiveAbsolute…`
follows the glossary, not the reverse.

## Out of scope

- *theirs*, *hers*, *ours*, *mine*: 0 faces as independent forms; declare
  none.
- Angry Mob (in the 10): its first sentence must also read.
- Resolving what noun the independent genitive stands for.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
