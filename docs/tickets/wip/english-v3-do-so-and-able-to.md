---
needs: []
---
# Read the pro-form *do so* and postpositive *able to* VP: All creatures able to block this creature do so.

Two mechanisms in one ticket, on evidence: every *able to* face also needs
*do so*, so neither half alone covers a single face.

## Why

Blocker report at trunk `uoqyuxtnowpo`: key `at:creatures able` 27
sole-blocker faces. `cargo xtask english-v3 grep -i '\bable to\b'` finds
**31** faces, all unread, every one *… able to block … do so*; 32 failing
lines, **29** of them the only failing line of their face. `grep -i
'\bdo(es)? so\b'` finds **32** faces (the same 31 plus Camouflage's *does
so*), all unread. Both halves fail independently:

- "All creatures do so." 0 (pro-form alone)
- "Creatures able to block it can't block." 0 (postpositive AdjP alone)
- "This creature is able to block." 0, although `vocab:Adjective/Able`
  already declares the frame `[to + BarePredicate]`
- "All creatures block this creature." 1; "Each creature blocks this creature
  if able." 1; "If you do, draw a card." 1

## Goal

1. *do so* / *does so* reads as a finite predicate whose content is supplied
   anaphorically: *all creatures … do so*, *each creature … does so*.
2. An Adjective Phrase with its own Complement reads as a postmodifier of a
   Nominal: *creatures able to block this creature*, *Walls able to block
   this creature*, *creatures with flying able to block this creature*,
   *creatures your opponents control able to block that creature this turn*.

## Analysis

CGEL Ch. 17 §7.5, p. 1529: "***do*** functions as head of complex pro-forms";
"The complex pro-form ***do*** *so* serves as an anaphoric VP which in many
cases is a slightly more formal alternant to the types of reduced VPs
considered in §§7.1–3". Ch. 5 §14, p. 445, "(b) Adjectives and AdjPs": "single adjectives and
phrases without their own post-head dependents occur in attributive position,
while others occur **postpositively**, i.e. after the head of the NP, as in
*members* [*dissatisfied with the board's decision*]". Ch. 6 §2.1, p. 528:
"**Postpositive** adjectives function as post-head internal modifier in NP
structure". Those back *do so* as a pro-form VP headed by lexical-like *do*,
and an AdjP with a Complement as postmodifier. Routes and licences are
project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`,
`lexicon/core.ron`): `BareAuxiliaryPredicate` + `BareEllipsis` /
`OmittedPlain` read stranded *do* (*If you do*; `core-verb:Do` declares
`AuxiliaryEllipsisLicence`); `vocab:Adverb/So` exists. `vocab:Adjective/Able`
declares `VerblessPredicateUse = Yes` (read by `VerblessClause` for *if
able*) and a `Predicate` frame `Marked(InfinitivalMarker To, Complement:
BarePredicate)` that nothing reads today. `PostmodifiedNominal` and
`ParticipialPostmodifiedNominal` are the nominal postmodifier routes;
`CorrelativePostpositiveNominal` is the only postpositive Adjective route.

Orchestrator rulings 2026-10-10:

- DS1. *do so*: refine the stranded-*do* route. *so* fills the Complement
  position that `BareEllipsis` leaves empty: a pro-form Complement of *do*,
  licensed by a feature the pro-form lexeme declares. No fixed-clause entry
  for the string, no new verb *do-so*.
- DS2. The licence reads declared features (the head's existing ellipsis
  licence, the pro-form's declared use). No test on *do*, *so*, *block* or a
  card.
- DS3. *able to* VP: an Adjective Phrase is the Adjective plus the Complement
  its declared frame selects; such a phrase may postmodify a Nominal. The
  licence is that the phrase has its own post-head dependent (CGEL p. 445),
  read from the structure, not from the lexeme. If that needs a feature on
  the AdjP Category (complemented or not), add it; do not key on *able*.
- DS4. Predicative *is able to block* is unattested (0 faces besides the
  postpositive use). If DS3's Adjective Phrase reads it through the existing
  AdjectivalComplement, that is acceptable and reported; do not add a check
  to block it and do not build anything to reach it.
- DS5. One analysis per string. The twins to avoid: (a) *if able* keeps
  exactly its Readings (212 covered faces): the bare Adjective in a Verbless
  Clause gains no postpositive or complemented twin; (b) *If you do, …* keeps
  exactly 1 Reading: no Reading with an empty pro-form; (c) *do so* must not
  also read as stranded *do* + a degree or manner *so* Adjunct; (d) a bare
  Adjective after a noun (*creatures able*, *creatures tapped*) is not
  licensed as a postmodifier by this ticket; (e) the trailing *this turn* in
  *able to block target creature this turn do so* attaches inside the
  infinitival; report it if a second attachment appears.
- DS6. Agreement: *do* / *does* agrees with the Subject as any finite verb
  (*all creatures … do so*, *each creature … does so*).
- DS7. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected: no covered face contains *do so* or
  *able to*. Any increase is reported with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0.

- Lure: "All creatures able to block enchanted creature do so."
- Mortipede: "{2}{G}: All creatures able to block this creature this turn do
  so."
- Gift of the Deity: "As long as enchanted creature is green, it gets +1/+1
  and all creatures able to block it do so."
- Marble Priest: "All Walls able to block this creature do so."
- Talruum Piper: "All creatures with flying able to block this creature do
  so."

Sentence probes, 0 today, one per mechanism so each can be tested alone:
"All creatures do so."; "That player does so."; "Creatures able to block it
can't block."; "All creatures able to block this creature block it."
Controls that must not change: "All creatures block this creature." 1; "Each
creature blocks this creature if able." 1; "Target creature attacks this turn
if able." 2; "If you do, draw a card." 1; "If a player does, draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(do|does|did|doing) so\b|\bable\b` (284 faces) plus 300
random covered controls, `--workers 6 --samples-per-face 0`, compared with
`cargo xtask english-v3 diff` against the claim parent. Zero lost; DS7 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, once, at
the end: a rebuild after a grammar edit takes about 3 minutes and the test
binary about 7, so batch edits. Tests assert the roles separately (*so* is
the pro-form Complement of *do*; the AdjP is the Nominal's postmodifier and
its infinitival is the Adjective's Complement; the DS5 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- Camouflage (*… that can block the creature that pile is assigned to does
  so*): also blocked by the unknown word *likewise*.
- You Look Upon the Tarrasque: a named-mode face (`english-v3-named-modes`).
- *While doing so, …* (Word of Command, 1 face): gerund-participial pro-form.
- *do it* / *do that* pro-forms; *unable to*.
- Postpositive AdjPs with a PP Complement (*full of …*; *full* is an unknown
  word today).

## Landing record

1. **Census delta** (`mqrlzszk` before, source-equivalent to the claim-parent
   grammar `mktpoqzr`; `zzwnrkot` after): bounded covered faces **510 → 541**,
   total Readings **7,268 → 7,368**; **31 gains**, lost **none**, decreases **0**,
   increases on previously covered faces **0**, digest-only **0**, validation
   issues **0**. Selector: `\b(do|does|did|doing) so\b|\bable\b`, case-insensitive,
   **284 faces**, plus **300** random covered controls from the census diff
   tool, seed **20261008**; two overlapping identities give **582 unique faces**.
   Evidence: `target/english-v3/english-v3-do-so-and-able-to/{manifest,before,after,diff}.json`;
   The feature alone gained **30** faces (**540** covered, **7,362** Readings);
   refresh added Torchling (**6** Readings), whose *targets only this creature*
   now uses the object-focus route from trunk `zqnqzmnu` / `ynoxmnun`.
   `refresh-diff.json` isolates that one gain, with no other changes.
   comparison used `cargo xtask english-v3 diff before.json after.json --json diff.json --lines`
   with those full evidence paths.
2. **Witnesses** (claim-parent grammar `mktpoqzr` → `zzwnrkot`, complete direct
   probes of the ticket's lines): Lure **0 → 1**; Mortipede **0 → 1**; Gift of
   the Deity **0 → 2**; Marble Priest **0 → 1**; Talruum Piper **0 → 1**. All
   four isolated mechanism probes **0 → 1**. Predicative *is able to block*
   **0 → 1**, as DS4 permits. The five controls retain **1 / 1 / 2 / 1 / 1**
   Readings. No already-reading witness was retained as a failure.
3. **Spot checks** (`zzwnrkot`): Alluring Scent and Bloodscent have Subject
   `creatures [able [to block target creature this turn]]` and Predicate
   `do [so]`; Breaker of Armies, Elvish Bard, Golgari Decoy, Nath's Elite and
   Nessian Boar have the same roles with `this creature`; Declare Dominance
   uses `it this turn`; Indrik Umbra uses `it`; Nemesis Mask uses `equipped
   creature`. In every checked Reading the AdjP is the Nominal's Postmodifier,
   the infinitival is the Adjective's Complement, and *so* is *do*'s pro-form
   Complement. No unintended analysis was found.
4. **Deviations and STOPs** (`zzwnrkot`): reused the existing complemented-AdjP
   `PostpositiveNominal` route; added `InfinitivalAdjective`,
   `AdjectivalInfinitive` and `ProFormComplement`, retiring Able's unread
   marked-Predicate frame in the lexicon. Added a declared
   `AuxiliaryProFormLicence`, alongside DS2's ellipsis check, to express DS1's
   *do* restriction without admitting negative *can*; coordination and
   quantified heads preserve this distinction. Gift's two Readings differ
   only in existing conditional scope (first conjunct versus the whole
   coordination); a control on the previous grammar has those same two
   scopes. *creatures tapped* retains its existing bare-passive analysis,
   without an adjectival twin. Added the glossary's Pro-form, Pro-form
   Complement Use and Auxiliary Pro-form Licence entries, and qualified the
   Ellipsis Licence definition under DS2. No unresolved glossary gap or STOP.
5. **Tests** (`zzwnrkot`): `cargo test -p deckmaste_english_v3` passed
   (**819 tests** after refresh; **797** before, including the new module registered once in `suite_6`);
   `cargo test -p deckmaste_lexical_source` passed (**82 tests**);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored **0**, re-spelled **0**, ignored **0**, added **7**, removed **0**
   (no removals to justify). After refresh, `cargo build -p xtask` and both
   complete crate suites and clippy commands passed again. The sole refresh
   conflict was overlapping `suite_6` registrations; all three were preserved.
   Citation checks: zero noncompliant strings, zero
   stale citations; bless changed no registrations; the diff audit selected
   no new citation sites. No Lean check or full-corpus census was run.
6. **Timing** (`zzwnrkot`): the bounded landing census took **6.01 s wall time** after refresh (**7.27 s** before)
   under `/usr/bin/time -v`, using the prebuilt binary, **6 workers** and
   `--samples-per-face 0`.
