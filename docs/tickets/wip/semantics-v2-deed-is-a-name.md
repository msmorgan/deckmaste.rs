---
needs: [semantics-v2-macro-capture-and-plurality]
---
**A keyword action is named by its declaration name where a deed is wanted,
and a keyword ability by its declaration name where a keyword label is
wanted.** Decided 2026-10-06 (`docs/decisions/semantics-v2.md` §7, "Deeds and
keyword labels are declared names"), the shape of the 2026-10-05 rulings "a
counter's kind is a name" and "a designation is a name". Needs the capture
landing because both change the loader. Standard constraints apply.

## Decided 2026-10-06

- `theVerbed(exile, Many)` and `enact(discard, …)` read the keyword action
  declaration's name bare at a deed position; `grants(flying)` reads the
  keyword ability declaration's name bare at a keyword label position. The
  loader refuses an undeclared name at load time.
- Lean keeps `Deed.action name` and `Deed.ofAbility name`
  (`lean/Semantics/Words.lean:211-215`), as it keeps `CounterKind.named`. The
  fifteen core deeds (`CoreDeed`, Words.lean:197) stay a closed enum.
- **`theVerbed` gains an optional noun word**, the optional slots trailing:
  `theVerbed(verb, plurality, word?, marking?)`. `theVerbed(exile, Many)` is
  "them, the exiled ones"; `theVerbed(exile, One, card)` is "the exiled card".
  The Lean selectors `stamped (verb)` (Words.lean:295) and `verbed (verb)
  (word) (marking)` (Words.lean:299) merge into one with `word : Option
  NounWord`.
- **`itVerbed` and `themVerbed` retire** into `theVerbed`. `it` and `them`
  are not overloaded. Owner: "I don't want to overload it and them like that,
  it'd take new macro_ron features"; "themVerbed is a horrific name".
  "thoseVerbed is better than themVerbed" was the interim answer before the
  merge was accepted; no `thoseVerbed` is minted.

## Where it is written today (counted 2026-10-06)

- `Action("…")`: 36 sites in 27 files. `plugins_v2/builtin/macros`: 32 sites
  in 23 files. `plugins_v2/canon/cards`: 4 (Graf Rats, Glimpse of Freedom,
  Krosan Grip, Sublime Exhalation). `plugins_v2/testing`: none. Fifteen
  distinct labels: Exile 5, Discard 5, Cast 5, Tap 3, Destroy 3, Airbend 3,
  Activate 3, Regenerate 2, and Vote, Shuffle, Search, Play, Meld, Manifest
  and Attach once each.
- The loader writes one `Action("Label")` per keyword action
  (`crates/deckmaste_semantics_v2/src/keywords.rs:141`), its label from
  `keyword_action_label` (:61), the camelCase name read as title-case words
  (`collectEvidence` is "Collect Evidence"). The generated label becomes the
  declaration name, and the generated `keywordActionLabels`
  (`lean/Semantics/Check/Facts.lean`, from `action_labels` in
  `crates/xtask/src/facts/lean.rs`) and the hand-written `actFacts` keys
  (`lean/Semantics/Check/Words.lean:287`) change with it.
- `keyword("…")`: 22 sites. Fifteen are counter declarations' `confers:
  [grants(keyword("Flying"))]`; seven are canon card sites.
- Lean: `KeywordLabel := String` (Words.lean:185), `KeywordActionLabel :=
  String` (Words.lean:192). An unknown label today is refused only at check
  time, as `knownAct` (`lean/Semantics/Check/Refusal.lean:151`; `deedFacts`
  and `knownAct` in `Check/Words.lean`, ~L395-400).

## The work

- Mirror the counter-kind precedent: `CounterKind`'s
  `#[macro_ron(denoted_by(…))]`
  (`crates/deckmaste_semantics_v2/src/words.rs`, ~L870-885) and the
  declaration kinds in `crates/deckmaste_semantics_v2/src/ron.rs` (~L28-45).
  `KeywordAction` already registers two kinds (its family and `Instruction`),
  so a deed position needs a further kind registration. No keyword action
  name collides with any other macro name today (65 keyword actions against
  1,472 macro names, checked 2026-10-06); keep a test that says so.
- Re-spell every `Action("…")` and `keyword("…")` site to the bare name, and
  every `itVerbed`/`themVerbed` caller to `theVerbed`.
- Rename the generated labels to the declaration names in the facts
  generator, `keywordActionLabels`, `actFacts` and every Lean pin that spells
  a label.
- The English side renders a pronoun when `theVerbed`'s word is absent. Name
  the owner of that realization (`docs/contexts/oracle-english/CONTEXT.md`
  and the English contract) in the landing record and check it there.
- Glossary: the **Keyword Action** entry says it "is named by its declared
  label"; amend it to the declaration name.

## Proof

`cargo xtask lean check` and `cargo xtask lean definitions` (or their
regrouped names) pass; a card naming an undeclared deed or keyword is refused
at load; `cargo xtask expansions` shows the re-spelled sites and no other
change.

## Landing record

Series on claim `suprnuvn`, oldest first: `lpulmpkm` (Lean and mirror: the
`stamped` and `verbed` selectors merge into `verbed (verb) (word : Option
NounWord) (marking)`), `mykqutqm` (`theVerbed(verb, plurality, word?,
marking?)`; `itVerbed` and `themVerbed` retired in RON and Lean), `nzkqnwxo`
(deeds are declared names), `lozxzott` (glossary, ADR, this record). Stage (d),
`foundCard`'s general form, needed no commit of its own: `mykqutqm` moved
`itVerbed(Action("Search"))` to `theVerbed(Action("Search"), One)` and
`nzkqnwxo` re-spelled it `theVerbed(search, One)` (transmute, transfigure).

**STOP: the keyword-label half is not landed.** `grants(flying)` and
`keyword(flying)` need a keyword ability's name to read at a `KeywordLabel`
position, and `KeywordLabel` is a `String` (Lean `abbrev KeywordLabel :=
String`; Rust `pub type KeywordLabel = String`). `macro_ron` expands a macro
only at an enum position or at a struct position that hosts macros; a string
position never expands, and ron reads no string at an enum position, so a
bare name cannot read there without one of: (A) a new `macro_ron` reader rule
(a newtype position that hosts macros is intercepted, as a struct position
is) with a Rust newtype `KeywordLabel(String)` read through ron's
`unwrap_newtypes` — a macro-system feature, which
`docs/decisions/macros-are-declarative.md` sends to a separate decision, and a
mirror shape `lean_drift` would have to pair with Lean's `abbrev`; (B)
`KeywordLabel` as a one-constructor type in Lean and Rust, as
`DesignationLabel` is, re-spelling every written label (well beyond the 22
`keyword("…")` sites); (C) `grants(flying)` and bare `flying` at an `Ability`
position, which already read today but carry the keyword's definition body
(`flyingExpansion`), so the 15 counter conferrals and 7 canon cards would
change term, not spelling. The 22 `keyword("…")` sites are unchanged.

**PROVE.**
- Lean, full build (`lean/scripts/build`, `--wfail`): 83 jobs, no errors or
  warnings, after each stage.
- `cargo xtask lean check`: canon 134/134, testing 7/7 after each stage.
  `cargo xtask lean definitions`: 553/553. `cargo xtask facts generate` then
  `facts check`: up to date (`keywordActionLabels` now the 65 declaration
  names).
- Emitted card terms (`lean/Generated/`, diffed stage against stage): (a) four
  canon cards (Drift of Phantasms, Drudge Beetle, Fleshwrither, Graf Rats)
  change constructor only (`stamped v` → `verbed v none .attributive`,
  `verbed v w m` → `verbed v (some w) m`); (b) byte-identical; (c) 71 canon
  lines differ, every difference a `Deed.action` label (checked by
  normalizing labels: 0 other differences).
- Expansions (`cargo xtask expansions`, `diff -r` stage against stage): (a) 9
  files, the selector constructor only (scavenge, station, transfigure,
  transmute, airbend, recruit and the three pronoun helpers); (b) the moved
  callers byte-identical, `itVerbed`/`themVerbed` gone, `theVerbed`'s sample
  header and value changed, `_index.txt` 1509 → 1507 declarations; (c) 98
  files, 132 lines, every one a `label:` line (`"Exile"` → `"exile"`, …),
  nothing else.
- Gate: `cargo xtask gate --changed --run` derived `cargo test -p
  deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask`: exit 0 after each stage; (a) and (b) 1391 passed, (c) 1395 passed,
  0 failed, 1 ignored (the pre-existing xtask ignore).
- Clippy `-D warnings` on `deckmaste_semantics_v2` and `xtask`: clean.
  `cite check --list-noncompliant`: 0; `cite check`: 0 stale; the diff's
  `cite audit` read (4 sites, all [CR#701.1] keyword actions); nothing to
  bless.
- Lexicon export (`cargo xtask lexical --card-name "Black Lotus" --export`) on
  the claim and on the tip: byte-identical. The English side already keys its
  keyword-action lexemes by declaration name (`lexeme:keyword_action/exile`)
  and reads no `semantics_v2` term, so the title-case label had no English
  consumer; `keyword_action_label` is retired outright, not confined.
- No refusal asserting a game rule was added: an undeclared deed is refused by
  the loader as an unknown name (term shape).

**DISCLOSE.** The loader registers, beside each keyword action declaration,
an argument-free macro of the same name at the `Deed` position denoting
`Action(label: <name>)` (`keywords::deed_macro`, the designation members'
mechanism), and `Deed::Action` is no longer natively spellable at a restricted
read (`ron.rs` `DECLARED_NAME_ONLY`), so `Action("Exile")` in a card is
refused by name. 39 `Action("…")` sites in 28 files re-spelled to the bare
name (the ticket's 36 counted before `mykqutqm`). The English realization of
`theVerbed` with no word is a pronoun ("it"/"them"), an Anaphor in
`docs/contexts/oracle-english/CONTEXT.md`; no English code consumes the
selector today, so there is nothing to check there yet. Glossary: Game Model
**Keyword Action** amended to the declaration name; no Oracle English gap.

**Deviations and additions.** Pins for the merged selector added to
`Proofs/NounWords.lean` (7: the pronoun and description reaching the same
stamped object, the wrong deed refused, `tracksObject` and `Reach.kind` for
each form). Rust tests added (4): `a_keyword_action_name_denotes_its_deed`,
`an_undeclared_deed_is_refused_at_load`,
`keyword_action_names_collide_with_no_other_declaration` (`tests/reader.rs`),
`a_keyword_actions_deed_is_its_declaration_name` (`keywords.rs`). `theVerbed`'s
`word` defaults through `Default(Any, None)`, since `None` is no `NounWord`.

**Counts (Assurance).** Restored 0. Re-spelled: Lean 21 pin and bench sites
for the constructor plus the three helper bodies in `Macros.lean` (a), 59 for the helper (b: 22 `itVerbed`, 5 `themVerbed`,
32 `theVerbed`), 385 label sites including the 65 `actFacts` keys and the
`.knownAct (.action "descry")` pin (c); Rust/xtask 6 tests (`keywords.rs`
`a_keyword_action_enacts_its_deed`, `tests/reader.rs` amass,
`expansions.rs` amass, `plugins_v2_declarations.rs` destroy/heal,
`facts/lean.rs` `new_action_reaches_the_lean_label_list`); the three
`keyword_action_label` assertions in `labels_capitalize_the_name` re-spelled as
`a_keyword_actions_deed_is_its_declaration_name` (the function is retired).
Ignored 0. Added 11 (7 Lean, 4 Rust). Removed 0.

**REPORT.** ADR §12.1 recount: 412 = 312 declarations + the two `counters`
macros + 98. Not an English-coverage landing: the coverage lock and its
`covered` count are untouched, and no `coverage` run is owed.
