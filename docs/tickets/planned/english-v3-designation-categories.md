---
needs: []
---
# Give designations their lexical categories: Commodore Guff can be your commander.

## Why

Designations are lexed as Keywords, so none reads where its word has an
ordinary category. On census `xnkuznpo` (docs-only descendant of
`vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369 unread; wave-4
recon of 2026-10-08), the recon buckets touch these unread faces (sole
cause in brackets):

- "*commander* (noun)": 181 (128).
- "*monarch* / *initiative* / *city's blessing*": 116 (77).
- "*prepared*": 71 (62).
- Together: 364 (269).

Faces with other designation words among their failing units: *saddled* 33,
*monstrous* 30, *suspected* 11, *Ring-bearer* 11, *enduring story* 9,
*goaded* 9 and *renowned* 8. These are regex counts over the recon's failing
units, not recon buckets. Counts are surface counts, not gain forecasts.

Probes (admitted roots, `--category Sentence --readings 1`), each against
a control: "Commodore Guff can be your commander." 0 / "… your creature." 1;
"Commander spells you cast have cascade." 0 / "Creature spells …" 1; "This
creature enters prepared." 0 / "… enters tapped." 1; "As long as this
creature is monstrous, it has trample." 0 / "… is tapped …" 1; "Whenever a
goaded creature attacks, draw a card." 0 / "… a tapped creature …" 1;
"When this enchantment enters, you become the monarch." 0 / "… you become a
creature." 1.

## Ruling

Owner ruling, 2026-10-08: designations lexed today as `FixedTerm` (category
Keyword) get their proper lexical categories.

- *commander* is a common count noun (*your commander*, *a commander*,
  *commanders you control*, *commander spells*).
- *monarch* and *initiative* are nouns. *the* is the ordinary Determiner,
  not part of the term: *if there is no monarch* is attested (M'Baku,
  Jabari Chieftain).
- *prepared*, *saddled*, *suspected* and the other participial or
  adjectival designations (*goaded*, *harnessed*, *monstrous*, *renowned*,
  *solved*) are adjectives or participles.
- *city's blessing* is a compound noun.

Retire the `FixedTerm` route for each re-categorised term on both sides, so
no string gets two labels.

## Mechanism

Declarations: `plugins_v2/builtin/macros/designations/*.ron`. Each carries
`grammar: FixedTerm(surface: …)`, and the surface includes the article for
*the monarch*, *the initiative* and *the city's blessing*.
`legacy/plugins.rs` (`grammar_lexeme`, near line 165) turns `FixedTerm` into
`Lexeme::invariant(…, Category::Keyword, …)`. The `grammar:` field already
offers the right recipes:

- `Grammar::Noun { singular, plural, … }`
  (`deckmaste_construction_core/src/macro_def.rs:380`) becomes
  `Lexeme::noun(…, Countability::Count, …)` with a derived or overridden
  plural. Subtypes use it, for example
  `subtypes/artifact/book.ron`: `Noun(singular: "Book", plural: Unavailable)`.
  Use it for *commander*, *monarch* and *initiative*. *city's blessing* goes
  through the declaration reader's compound-noun recipe, the route counter
  kinds use for *blood counter*. If that recipe cannot express a genitive
  stem, STOP for a ruling.
- No `Grammar::Adjective` variant exists. The adjective route that does exist
  is `FixedKeyword { participial_adjective: (...) }`
  (`participial_adjective_lexeme`, `legacy/plugins.rs:332`), which mints a
  `Category::Adjective` Lexeme; *kicked*, *enchanted* and *equipped* use it.
  Where a designation is the past participle of a declared verb, the
  participle comes from that verb: `keyword_actions/goad.ron`,
  `suspect.ron` and `harness.ron` are `Verb` declarations whose
  PastParticiple form already spells *goaded*, *suspected* and *harnessed*.
  So the designation's `FixedTerm` is a second label for the same word,
  and it is retired. Other adjectives (*prepared*, *monstrous*, *renowned*,
  *saddled*, *solved*) need a declared adjective recipe. Adding a `Grammar`
  variant or reusing the participial-adjective recipe on the designation is
  a declaration-reader change. Record which one in the landing record. Do
  not mint duplicate Adjective Lexemes for words a verb already owns
  (2026-10-04 ADR amendment: premodification is a function of the existing
  word).
- *solved* is also `keyword_abilities/solved.ron`
  (`FixedKeyword(surface: "solved")`, the *Solved —* label). The label
  keeps its keyword; only the designation's predicative use is re-categorised.
- Out of the ruling's list, leave as they are unless the corpus shows them
  failing for the same cause: *day or night*, *an enduring story*, *left/right
  half unlocked*, *level*, *Ring-bearer*. Note any such face.

Each re-categorised term must keep its designation identity as declaration
provenance. The Lexeme owner stays `lexeme:designation/<name>`.

## Witnesses

Each witness's failing units all match these buckets and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Commodore Guff: "Commodore Guff can be your commander." (control "… your
  creature." 1)
- Flamekin Herald: "Commander spells you cast have cascade." (control
  "Creature spells you cast have cascade." 1)
- Blazing Firesinger: "This creature enters prepared." (control "This
  creature enters tapped." 1)
- Colossus of Akros: "As long as this creature is monstrous, it has trample
  and can attack as though it didn't have defender." (control with *tapped*
  1)
- Court of Ire: "When this enchantment enters, you become the monarch."
  (control "… you become a creature." 1)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed: `deckmaste_english_v3` and
   `deckmaste_lexical_source` always; `deckmaste_construction_core` if
   the declaration reader changes. No xtask tests. A `grammar:` field edit
   leaves definition bodies unchanged. Run `cargo xtask facts check` once.
   Only if it reports drift, regenerate with `facts generate` and run one
   `cargo xtask lean check`, and say so. No post-refresh re-census
   unless refresh touched grammar, lexicon, core or `plugins_v2`. New
   tests go under tests/suite/ registered in tests/main.rs. Evidence lives
   under the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling. Faces covered today
   through a designation's Keyword Reading must keep reading through the
   new category.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A designation word still analysed as a Keyword, one string read
   both as Keyword and as noun or adjective, or *the* fused into a noun
   designation, is a defect. A wrong analysis that parses is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` fields in `plugins_v2` designation and keyword
   declarations and lexicon frames in
   `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies untouched.
   Lexicon additions must be attested in the supported corpus (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *take the initiative*: *take* is a vocabulary gap ("When this creature
  enters, you take the initiative." reports gap *take*). Note the faces.
- *while saddled* and other verbless *while* clauses: the control "…
  attacks while tapped, …" also reads 0.
- Engine semantics of designations.

## Landing record

The six items in CLAUDE.md. Also list each re-categorised designation with
its new category and recipe, and the Keyword route retired for it.
