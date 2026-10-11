---
needs: []
---
# Read a relative clause introduced by genitive *whose*: Gain control of target creature whose controller controls an Island.

## Why

`vocab:RelativePronoun/Whose` is declared, but no Construction uses it: NP
"target creature whose controller controls an Island" 0 against "target
creature that controls an Island" 1; "Each player whose life total is 10 or
less sacrifices a creature." 0 against "Each player who controls an Island
sacrifices a creature." 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`): failing token `whose` 10 sole, 10
touched (keys `at:creature whose` 4, `at:opponent whose` 2, `at:player whose`
2, …). `cargo xtask english-v3 grep '\bwhose\b'` finds **13** faces, all
unread; all **13** have that line as their only failing line. In 11 the
*whose* phrase is the Subject of the relative clause (*whose controller
controls an Island*, *whose life total is less than …*, *whose power and
toughness aren't equal*, *whose coin comes up tails*); in 2 it is a fronted
predicative Complement (*the player whose turn it is*).

## Goal

A relative clause whose relative phrase is *whose* + nominal, functioning as
the Subject of the clause, modifies a noun: *target creature whose controller
controls an Island*. Byte-exact realisation.

## Analysis

CGEL Ch. 12 §3.2.5, p. 1043, "Type VI: from genitive *whose* to NP (*whose
essay*)": "Relative *whose* functions as subject-determiner in NP structure
and obligatorily triggers upward percolation"; [23ii] "*the student* [*whose
essay he plagiarised*]". That backs *whose* as determiner of an NP that is
the relative phrase. CGEL's example has the phrase as a fronted Object; the
Subject case attested here needs no fronting and is the simpler one. Glossary:
**Relative Clause**, **Relative Use**, **Determiner**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PronounSubjectRelative` (`[marker: lexical(Pronoun), clitic_separator(…),
predicate: FinitePredicate]`, `require marker.RelativeUse = Yes`,
`marker.case = Nominative`, agreement of marker and predicate) reads *who*;
`SubjectRelativeClause` reads *that*; `SubjectRelativeNominal` attaches
either to a Nominal. Genitive determiners of an NP are read by the genitive
determiner route (*its controller*, "a creature its controller controls" 1).
Lexicon `core.ron`: `Whose` has no features; `Who` has `RelativeUse: Yes` and
Nominative forms.

Orchestrator rulings 2026-10-11:

- WH1. Refine `PronounSubjectRelative` with a second form, no new
  Construction (R-budget): `[marker: lexical(Pronoun), " ", head: Nominal,
  separator, predicate: FinitePredicate]`, where the marker is the relative
  pronoun in its genitive case and the predicate agrees with the *nominal*
  (its person and number), not with the antecedent.
- WH2. Data: give `Whose` `RelativeUse = Yes` and a genitive form (`case:
  Genitive`), following English's real paradigm: *whose* is the genitive of
  relative *who* and serves personal and non-personal antecedents alike
  (*creature whose*, *permanent … whose*); do not add an antecedent-animacy
  check. The licence reads `RelativeUse` and `case`; nothing names *whose*
  (R-class: any relative pronoun with a genitive form gets the form; there is
  no genitive *that*, and none is minted).
- WH3. One analysis per string (R-one). *creature whose controller controls
  an Island* is head + one relative clause whose Subject is [*whose
  controller*]. It must not also read as a bare (zero) Object relative
  beginning at *controller*, nor with *whose* as an interrogative. The
  witness reads as many ways as its *that*-relative control.
- WH4. Attachment follows the existing relative-clause rules (the 2026-10-06
  licensing amendment and `english-v3-relative-clause-adjuncts`); add no new
  attachment site. A head with intervening postmodifiers (*each nonland
  permanent with mana value X whose controller was dealt combat damage …*,
  Steel Hellkite) reads if ordinary relatives do there; otherwise list it.
- WH5. Fronted non-Subject *whose* phrases are out: *the player whose turn it
  is* (2 faces) has a predicative gap and goes with
  `english-v3-predicative-gap-relative` (shelved; add these two to its
  evidence). *whose essay he plagiarised* (Object gap) is unattested: build
  nothing and block nothing.
- WH6. Newly covered faces are gains. No covered face contains *whose*, so
  no Reading-count change on a covered face is expected. If WH1 alters the
  exports or requirements of `PronounSubjectRelative`'s existing form,
  R-reach applies (Method).

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Seasinger: "{T}: Gain control of target creature whose controller controls
  an Island for as long as you control this creature and this creature remains
  tapped." ("Gain control of target creature whose controller controls an
  Island." 0)
- Gilt-Leaf Winnower: "When this creature enters, you may destroy target
  non-Elf creature whose power and toughness aren't equal." (sentence 0)
- Anya, Merciless Angel: "Anya gets +3/+3 for each opponent whose life total
  is less than half their starting life total."
- Lullmage's Domination: "This spell costs {3} less to cast if it targets a
  creature whose controller has eight or more cards in their graveyard."
- Steel Hellkite: "{X}: Destroy each nonland permanent with mana value X
  whose controller was dealt combat damage by this creature this turn.
  Activate only once each turn." (WH4)

Also 0: "Destroy each creature whose power is 2 or less."; "Each player whose
life total is 10 or less sacrifices a creature." Controls that must not
change: "Destroy target creature that has flying." 1; "Destroy each creature
that has power 2 or less." 1; "Each player who controls an Island sacrifices
a creature." 1; NP "each opponent who has more life than you" 1; NP "a
creature its controller controls" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(whose|who|whom)\b` (R-reach: the refined Construction reads
every *who* relative) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; WH6 for changes; digest-only changes by class;
name every re-spelled test. Full `cargo test -p deckmaste_lexical_source` and
`-p deckmaste_english_v3` with clippy `--tests -- -D warnings`, once, at the
end. Lexicon data loads at runtime; the grammar edit costs a 2–3 minute
rebuild and the six test binaries about 75 seconds. Tests assert the role
(*whose* as determiner of the relative phrase, that phrase as Subject, the
clause as Modifier of the antecedent, the WH3 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *the player whose turn it is* (Obeka, The Beamtown Bullies): WH5.
- *whose coin comes up tails* (Rakdos, the Showstopper; Goblin Assassin):
  also needs *come up*, `english-v3-small-frames-batch-3`.
- *whose mana cost could be paid by some amount of, or all of, the mana you
  spent on {X}* (Illusionary Mask); *whose graveyard has fewer creature cards
  in it than their graveyard does* (Oath of Ghouls: stranded-auxiliary
  comparative, recorded in `english-v3-systemic-residuals`).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
