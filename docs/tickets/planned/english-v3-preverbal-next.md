---
needs: []
---
# Read serial-order *next* before the verb: When you next cast an instant or sorcery spell this turn, copy that spell.

## Why

*next* lexes only as `vocab:AttributiveAdjective/Next` (*your next upkeep*,
*the next spell you cast*). Between Subject and verb it has no analysis.
Blocker report at trunk `uoqyuxtn` (`target/english-v3/blockers-wave7.txt`):
key `at:you next` 34. `cargo xtask english-v3 grep '\byou next [a-z]+'` finds
**42** faces (44 matches), all unread; **35** have that line as their only
failing line. Shapes: *When you next cast an instant or sorcery spell this
turn* 27, *… cast a creature / artifact / Lesson / Bird spell …* 10, *…
cast a spell …* 4, *When you next activate an exhaust ability / an ability
that isn't a mana ability …* 2, *When you next attack this turn* 1. The verb
is always finite present after the Subject *you*.

## Goal

*next* reads as a serial-order Adverb in pre-verbal central position (*you
next cast …*, *you next attack …*), with byte-exact realisation. This is a
data ticket: one lexeme declaration that an existing route reads.

## Analysis

CGEL Ch. 6 §7.1, p. 576, [5ix]: "SERIAL ORDER | *The play was next performed
in 1901.*", among VP-oriented AdvP adjuncts. P. 580, [19ii] marks *next* as
acceptable before the lexical verb, and: "The serial order adverbs *again*,
*first*, *last*, and *next* generally resist front position quite strongly".
Those back the Adverb Category, the serial-order function and central
position. CGEL's example is post-auxiliary; the pre-verbal site after a
Subject with no auxiliary is the project application.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`CentralAdverbHead`, first form `[modifier: lexical(Adverb), " ", head:
lexical(Verb)]` with `require head.CentralAdjunctPosition = Preverbal` and
`require central_preverbal(modifier.CentralAdjunctLicence) = Yes`, consumed
by `CentralAdverbPredicate`
(`docs/tickets/done/english-v3-central-adverb-position.md`). "You also draw
a card." reads 1 through it. `lexicon/core.ron` models:
`vocab:Adverb/Fully` (`VPFinalAdjunct: No, ClauseInitialAdjunct: No,
CentralAdjunctLicence: Preverbal`), `vocab:Adverb/Also`, `vocab:Adverb/Still`.

Orchestrator rulings 2026-10-10:

- PN1. Declare an Adverb lexeme *next* in `lexicon/core.ron` with
  `CentralAdjunctLicence: Preverbal`, `VPFinalAdjunct: No`,
  `ClauseInitialAdjunct: No`. No grammar edit. If the declaration alone does
  not read the witnesses, record why and STOP rather than add a Construction.
- PN2. The Adverb is a second lexeme beside the attributive Adjective (CGEL
  treats the two uses under different Categories); the Adjective is
  untouched. This is not a duplicate owner for one use: each lexeme has one
  function.
- PN3. Only the pre-verbal site is licensed: post-copula, post-auxiliary,
  imperative, VP-final and clause-initial *next* have 0 faces
  (`grep '\b(is|are|was|has|have|may|can) next\b'`, `'\bnext[.,]'` and
  `'(^|\. )Next\b'`: 0 each) and are not declared.
- PN4. One analysis per string. *you next cast a spell* has the Adverb as
  Adjunct of *cast*; no Reading takes *next* as a postmodifier of *you*.
  *your next upkeep*, *the next spell you cast this turn*, *the next end
  step*, *the next time …* keep exactly their Readings: the Adverb lexeme
  must not supply a second analysis of any attributive *next*.
- PN5. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is a defect under PN4 (1,143 faces contain *next*,
  nearly all attributive); report its shape before landing and narrow the
  declaration, never the grammar by word.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category sentence --readings 3`): witness 0;
control = the same sentence without *next* (3+ = enumeration capped).
Re-probe on the claim parent first.

- Galvanic Iteration: "When you next cast an instant or sorcery spell this
  turn, copy that spell." (control 3+)
- The Clone Saga (chapter body): "When you next cast a creature spell this
  turn, copy it, except the copy isn't legendary." (control 2)
- All-Out Assault: "When you next attack this turn, untap each creature you
  control." (control 2)
- Dual Strike (document): "When you next cast an instant or sorcery spell
  with mana value 4 or less this turn, copy that spell. You may choose new
  targets for the copy." (control 3+)

Controls that must not change: "At the beginning of your next upkeep, draw a
card." 1; "You also draw a card." 1; "It still has flying." 1; "The next time
you cast a spell this turn, copy it." 3+.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bnext\b` (1,143 faces, case-insensitive) plus 300 random
covered controls (`cargo xtask english-v3 diff --manifest … --random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; PN5 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3` (it
consumes the data), once, at the end. Lexicon data loads at runtime, so no
grammar rebuild is needed for the census; the test binary takes about 7
minutes. Tests assert the role (the Adverb as central Adjunct of the finite
verb inside the trigger condition; the PN4 negatives on an attributive
*next*), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- *When you next activate an exhaust ability …* (Pit Automaton and one
  more): also needs the keyword premodifier of
  `english-v3-keyword-mention-without-parameter`.
- *first* / *last* in the same position: 0 faces (`grep '\b(you|they|it)
  (first|last) [a-z]+'`).
- *by the next play* and other attributive uses.

## Landing record

Not landed.
