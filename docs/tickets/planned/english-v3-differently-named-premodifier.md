---
needs: []
---
# Read an adverb-modified participial premodifier: the number of differently named lands you control

## Why

*differently* is an unknown word, and it occurs only in one frame: before the
past participle *named* premodifying a noun. Blocker report at trunk
`qzprwuxz` (`target/english-v3/blockers-wave9.txt`): `word:differently` 10
sole, 10 touched. `cargo xtask english-v3 grep 'differently named'` finds
**10** faces, all unread; all **10** have that line as their only failing
line (*differently named lands* 5, *artifact tokens* 2, *tokens* 1, *creature
tokens* 1, *dungeon* 1). Two things are missing: the adverb, and the
participle *named* as a premodifier at all (NP "named lands you control" 0,
although "a land named Forest" 1). No other adverb + participle premodifier
is attested (`\b\w+ly (\w+ed|named|cast|put|chosen) (\w+ )?(creature|land|
card|token|permanent|artifact|dungeon|spell)s?`: the same 10 faces).

## Goal

A past-participial Premodifier may carry a preceding manner adverb:
*differently named lands*, with byte-exact realisation.

## Analysis

CGEL Ch. 6 §3.2, p. 547: "Modifiers in the structure of AdjPs may have the
form of AdvPs, determinatives, NPs, PPs, and, under very restricted
conditions, relative clauses"; [33i] "*She is* [*quite incredibly
generous*]". That backs an AdvP modifying an adjectival head. The existing
declaration cites CGEL Ch. 5 §14.2, p. 444 for participial VPs premodifying a
noun; a manner adverb inside that premodifier is a project ruling here
(re-check p. 444 before citing it for the adverb). Glossary: **Premodifier**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`VerbalPremodifier` (`form [head: lexical(Verb)]`, `require
verbal_premodifier(head.form, head.frame, head.PastParticipialPremodifier) =
Yes`) with its `Premodifier` instance, and `ParticipialPremodifier`
(`[modifier: VerbalPremodifier, " ", head: Nominal]`). Lexicon: verb
`lexeme:Verb/Name`; the per-lexeme feature `PastParticipialPremodifier`
(`core.ron`, e.g. `core-verb:Lock`, `lexeme:keyword_action/goad`); adverbs
with `VPFinalAdjunct`, `ClauseInitialAdjunct`, `CentralAdjunctLicence`.

Orchestrator rulings 2026-10-11:

- DN1. Refine `VerbalPremodifier` with the compiler's optional lexical field
  with a trailing separator (`optional(lexical(Adverb), " ")` before the
  head), so the adverb-free premodifier's Debug and digests do not move. No
  new Construction (R-budget).
- DN2. The licence reads a declared feature of the adverb (a manner adverb
  that may modify a participle; declare the feature on the adverb class and
  set it for the lexemes of that class whose meaning allows it, R-class) and
  the verb's existing `PastParticipialPremodifier`. Nothing names
  *differently* or *named*.
- DN3. Data: declare the adverb `Differently` (not VP-final, not
  clause-initial, no central position: none of those uses is attested) and
  set `PastParticipialPremodifier` for `Name`. With that feature alone,
  *named lands* becomes readable; that string is unattested but grammatical,
  and no check is added to block it (R-prune).
- DN4. One analysis per string (R-one). *differently named lands you
  control* is one Nominal: [*differently named*] premodifies *lands*. It must
  not also read with *named* as a postmodifier of a preceding noun (*number
  of differently* is not a nominal), with *lands* as the name Complement of
  *named* (the *a land named Forest* frame), or with *differently* as an
  Adjunct of the clause.
- DN5. Newly covered faces are gains. `Name` gaining the premodifier feature
  could add Readings on the 237 covered faces containing *named*: wherever a
  noun follows *named* (*a card named Forest …*), a twin with *named*
  premodifying that noun is the DN4 defect. Expected increase: none. Report
  any with its shape; if the twin is real, constrain it by the participle's
  declared frame (a participle whose Complement is present is not a bare
  premodifier), never by a word.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Awakened Amalgam: "Awakened Amalgam's power and toughness are each equal to
  the number of differently named lands you control."
- Fungal Colossus: "This spell costs {X} less to cast, where X is the number
  of differently named lands you control."
- All-Fates Scroll: "{7}, {T}, Sacrifice this artifact: Draw X cards, where X
  is the number of differently named lands you control."
- Audience with Trostani: "Create a 0/1 green Plant creature token, then draw
  cards equal to the number of differently named creature tokens you
  control."
- Ellywick Tumblestrum: "… get +2/+2 for each differently named dungeon
  you've completed."

NP probes: "the number of differently named lands you control" 0; "each
differently named dungeon you've completed" 0. Controls that must not change:
NP "the number of tapped lands you control" 3+ and NP "the number of nontoken
lands you control" 3+ (re-probe with a higher `--readings` and record exact
counts; the witness NP must read as many ways as the *tapped* control); NP "a
land named Forest" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `differently|\bnamed\b` (326) and, because DN1 alters
`VerbalPremodifier` (R-reach), every face whose baseline Readings use it:
select by the participial and gerund premodifiers the lexicon marks
(`\b(tapped|untapped|attacking|blocking|enchanted|equipped|exiled|chosen|
revealed|sacrificed|goaded|suspected)\b` is a sufficient surface superset;
state the count), plus 300 random covered controls (`cargo xtask english-v3
diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; DN5 for increases; digest-only changes by class;
name every re-spelled test. Full `cargo test -p deckmaste_lexical_source` and
`-p deckmaste_english_v3` with clippy `--tests -- -D warnings`, once, at the
end. Lexicon data loads at runtime; the grammar edit costs a 2–3 minute
rebuild and the six test binaries about 75 seconds. Tests assert the role
(the adverb as Modifier of the participle, the participle phrase as
Premodifier of the head noun, the DN4 negatives), never an incidental count
or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *with different names* (reads today).
- Other adverb + adjective premodifiers (none attested).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
