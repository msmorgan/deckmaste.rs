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

`cargo xtask lean-check` and `cargo xtask definition-check` (or their
regrouped names) pass; a card naming an undeclared deed or keyword is refused
at load; `cargo xtask expansions` shows the re-spelled sites and no other
change.
