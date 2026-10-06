---
needs: []
---
# Read "instead" in replacement and alternative effects

## Why

*Instead* never reads in Oracle replacement text. On change `wlvwtnppyovn`
(32,828 supported faces, 13,716 covered, 19,112 unread) it occurs in a failing
unit of 979 unread faces and is the only recognised cause on 288. These are
surface-bucket counts, not gain forecasts. The *would* host is not the problem:
"If that creature would die this turn, exile it." has one admitted root, and
"… exile it instead." has none; "Exile it instead." alone has none.

Two lexemes spell *instead*: `vocab:Preposition/Instead` (core.ron) and
`vocab:ReplacementMarker/Instead` (vocabulary.rs). Deal's first frame in
`verbs.ron` ends `OptionalRole("ReplacementMarker")`. This ticket takes over
that ReplacementMarker slot from the Deal row of the unsupported-inventory
table (done `english-v3-generic-frame-consumption`, routed to
`english-v3-systemic-residuals`). The row's `MassNoun` and `DistributionPhrase`
slots stay with the residuals ticket. It also takes over Burn the Accursed, one
of the 42 inherited frame-coordination faces in that ticket, whose only failing
unit is an *instead* sentence.

## Goal

*Instead* reads clause-finally ("…, exile it instead."), clause-initially
("…, instead any number of target creatures you control gain indestructible
…"), and with its Complement (*instead of X*), with one lexical analysis. The
duplicate lexeme is either retired or justified by a distinct declared
function, recorded in the landing. Any Reading the corpus cannot disambiguate
stays: clause-final *instead* attaches to the main clause and may also attach
to a coordinated predicate.

## Analysis

*Instead* is a Compound Preposition: *in* + *stead* have coalesced into one
word that takes an *of* Complement (CGEL, Ch. 7, §3.1, pp. 622–623; glossary
Compound Preposition). Its Complement is optional, and when it is omitted,
*of* drops too (CGEL, Ch. 7, §2.4, p. 616, [32v], [33]). Bare *instead*
therefore heads a PP with no overt Complement, like *out*, not an Adverb. In
the replacement sentences it functions as a connective Adjunct of contrast:
CGEL lists *instead* among the connective adjuncts of addition and comparison
(CGEL, Ch. 8, §19, p. 778, [10]). Its Preposition Function Licence must admit
the clause Adjunct function. A lexical verb frame should select it only where
the corpus requires a Complement reading.

Ruling (user, relayed with this ticket's brief, 2026-10-06): stranded *be*,
*have* and modals are not attested Oracle style ("If it would be, instead …"
does not occur). Add no ellipsis licence for them. Do-support (*if you do*)
and *can't* are attested and keep their existing analyses. If the
implementation seems to need a stranded-auxiliary Reading, STOP and report.

## Witnesses

- Forbidden Crypt: "If you would draw a card, return a card from your
  graveyard to your hand instead." (its other unit, "If a card would be put
  into your graveyard from anywhere, exile that card instead.", also needs
  *from anywhere*)
- Burn the Accursed: "If that creature would die this turn, exile it instead."
- Soldevi Excavations: "If this land would enter, sacrifice an untapped Island
  instead."
- Increasing Savagery: "If this spell was cast from a graveyard, put ten
  +1/+1 counters on that creature instead." (non-*would* host)
- Divine Resilience: "If this spell was kicked, instead any number of target
  creatures you control gain indestructible until end of turn."
  (clause-initial)
- Reality Twist: "If tapped for mana, Plains produce {R}, Swamps produce {G},
  Mountains produce {W}, and Forests produce {B} instead of any other type."
  (*instead of*)

Except for Forbidden Crypt, each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/instead-before.json` on the claim parent,
   stamped with its change id.
2. Write the witnesses as tests first, each with its *instead*-less twin where
   one reads.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *instead* attached inside an
   NP, or any stranded-auxiliary gap, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule). Deleting the duplicate lexeme follows the same rule.
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Ellipsis recoverability in general (`english-v3-ellipsis-recoverability`,
  maybe).
- Host-clause causes such as *double that damage* (Fire Servant), *from
  anywhere*, *rather than* (189 touched / 70 sole).

## Landing record

In addition to the standard record: which *instead* lexeme survives and why;
whether Deal's ReplacementMarker slot is now consumed; the Reading counts of
each witness and its twin; timings as integer ns and ns/B with host load and
worker count.
