---
needs: []
---
# Read Planechase and draft wording on Vintage-legal faces: You may planeswalk. / Draft this card face up.

## Why

Owner ruling 2026-10-11, verbatim: "planchase/conspiracy stuff that's
vintage-legal should be supported for parsing." It resolves
`english-v3-hygiene-3` item D2: these faces stay in the supported corpus and
must read. **The grammar parses the wording only; the Planechase and
Conspiracy game modes are not modelled**, and nothing here gives a definition
a body. `core-retire-planechase-surface` (deleting the modelled Planechase
surface) is unaffected.

`cargo xtask english-v3 grep` over the supported corpus, unread counts against
`target/english-v3/baseline-wave9.json`:

| Pattern | Faces | Unread |
|---|---:|---:|
| `(?i)\bplaneswalks?\b` | 8 | 8 |
| `(?i)\bplanar\b` | 3 | 3 |
| `(?i)chaos ensues` | 6 | 6 |
| `(?i)\bdraft(ed\|s\|ing)?\b` | 27 | 27 |
| `(?i)as you draft` | 18 | 18 |
| `(?i)during the draft` | 3 | 3 |
| `(?i)\bbooster packs?\b` | 9 | 9 |
| `(?i)\bconspiracy\b` | 0 | 0 |
| `(?i)face up[^.\n]*before` | 0 | 0 |

The union of the first four is **38** faces, all unread: 11 with Planechase
wording (five *Path of the …*, Susan Foreman, Start the TARDIS, TARDIS, Ichor
Elixir, Fractured Powerstone, Missy) and 27 with draft wording (Cogwork
Librarian, Lore Seeker, Regicide, …). No supported face names *conspiracy*.

## Goal

Every cluster below reads through existing clause, NP and Adjunct routes once
its vocabulary is declared, with byte-exact realisation.

## Analysis

No CGEL claim is needed: the surrounding constructions already read with
other vocabulary (controls below), so this is a declaration ticket. Clusters,
by construction, with faces:

- A. Verb *planeswalk*, intransitive: *you may planeswalk*, *If you would
  planeswalk*, *then planeswalk*, imperative *planeswalk.* 8 faces.
- B. *planar* + *die* / *dice* / *deck*: *Roll the planar die.*, *one or
  more planar dice*, *your planar deck*. 3 faces.
- C. Clause *chaos ensues*: Subject noun + intransitive verb. 6 faces.
- D. Verb *draft*, transitive, all forms: *Draft this card face up.* 13;
  *Reveal this card as you draft it* 11; line-initial *As you draft a …* 7;
  participle *drafted* (*a card you drafted*, *the next card drafted by …*,
  *as you drafted cards named …*) 14; *drafting* 2. 27 faces in all.
- E. Noun *draft*: *during / after / from / to the draft* 7; *draft round* 5.
- F. Noun *booster pack(s)*. 9 faces.
- G. Routed elsewhere, counted here only: the labels *planeswalk* / *chaos*
  (*votes for planeswalk or chaos*, *If planeswalk gets more votes*, 5 faces)
  belong to `english-v3-option-word-choice-list` (OW4, OW8); *Will of the
  Planeswalkers —* (5 faces) is an undeclared Ability Word; *the vote is
  tied* is `english-v3-tied-for-adjective`; *note how many cards you've
  drafted* (4 faces) needs an interrogative content clause after *note*.

No owning definition exists under `plugins_v2/` for any of A–F. Neighbours:
`keyword_actions/vote.ron` and `ventureIntoTheDungeon.ron` declare `grammar:
Verb(…)`; `designations/` and `turn_parts/` declare `grammar: Noun(…)`;
`ability_words/willOfTheCouncil.ron` is the Ability Word model. Core:
`verbs.ron` has `Roll` and `Note`; *die* and *deck* are declared nouns; there
is no *ensue*.

Rulings (orchestrator 2026-10-11):

- PC1 (R-plugin). Game terms are declared through an owning plugin
  definition's `grammar:` row: *planeswalk* and *draft* as verbs
  (`KeywordAction`-kind definitions with `grammar: Verb(…)`), *chaos*,
  *draft*, *booster pack* and the *planar* terms as nouns. The definitions are
  bodiless and grammar-only; each says so in a comment and names this ruling.
  If no existing meta-macro kind fits a noun term, STOP with the term and the
  nearest kind rather than adding a core noun or a new kind.
- PC2 (R-class). Licence the natural class, never a member list. Each verb
  gets its frame once and every form follows from the paradigm; no frame per
  attested sentence. Cluster B is one declaration that lets *planar* compose
  with the declared nouns it modifies, not three fixed strings; if only a
  Compound Noun per head is possible, say so before building.
- PC3 (R-one). One route per construction. *chaos ensues* is an ordinary
  clause, with no fixed-clause Construction beside it. *ensue* is ordinary
  English (core verb lexicon); *chaos* is the game term.
- PC4 (R-wrong). A face read only through an unintended analysis is not a
  gain: *planeswalk* never reads as a Type Word, *draft* never as a name,
  *chaos* in cluster G never as this noun's bare NP (that is the option-word
  ticket's label route). Spot checks name the analysis per cluster.
- PC5 (R-budget). Prefer data, refinement or the compiler's optional lexical
  field over a new Construction. Expected new Constructions: **0**. *face up*
  after *draft* is the frame's predicative Complement on the model of
  *Exile this card face down.*; *as you draft it* is the existing *as*
  Adjunct. A needed Construction is a STOP with the faces it buys.
- PC6. No guard names a lexeme or card. Gains are newly covered faces; no
  Reading-count change is expected on any covered face (none contains the
  vocabulary); any change is a defect.

## Witnesses

Probed 2026-10-11 (`--category sentence --readings 8`, enumeration complete)
with the prebuilt binary of the in-flight `english-v3-slash-pair-predicative`
workspace; re-probe at the claim parent. All **0**:

- Start the TARDIS: "You may planeswalk."
- Fractured Powerstone: "Roll the planar die."
- Cogwork Librarian: "Draft this card face up."
- Lore Seeker: "Reveal this card as you draft it."
- Whispergear Sneak: "During the draft, you may turn this card face down."

Also 0: "You draw a card and chaos ensues."; NP "a booster pack"; NP "the
draft". Controls that must not change: "Roll a six-sided die." 1; "Reveal
this card as you put it into your hand." 2; "You may turn this card face
down." 1. Also read today: "Exile this card face down." 1; "Note its name." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(planeswalks?|planar|chaos|ensues?|draft(ed|s|ing)?|booster)\b`
plus 300 random covered controls (`--random 300 --seed 20261008`), `--workers
6 --samples-per-face 0`, compared with `cargo xtask english-v3 diff --lines`
against the claim parent. Zero lost; PC6 for changes. Because `plugins_v2/`
changes: the FULL `cargo test -p xtask`, `-p deckmaste_semantics_v2`, `-p
deckmaste_construction_core`, `-p deckmaste_english_v3` (and `-p
deckmaste_lexical_source` for *ensue*), clippy `--tests -- -D warnings` for
each, `cargo xtask lean check`, and a facts registration or exemption for
each new bodiless definition. No existing definition's declared signature and
no v1 / Idris facts row changes. Tests assert the role (the verb and its
Complement; the PC4 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Cluster G and every other blocker on these faces (many have several
  failing lines; measure the sole-blocker count at claim time).
- Any body, engine behaviour or game-mode rule; faces not legal in Vintage.

## Landing record
