---
needs: [plugins-v2-connive-bare-and-n]
---
**Every keyword action and keyword ability declaration's `spelling` becomes
its full uninflected card-usage template, with `<Param(n)>` slots and a
`:hint` only where a parameter kind's default realization is wrong.**
Decided 2026-10-07 by the owner; the ruling is the dated amendment
"Keyword spellings are card-usage templates" in
[the spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
Needs the connive split so the `connive`/`conniveN` templates are written
once. Standard constraints apply.

## Decided 2026-10-07

- Spellings are templates: "the spelling crate will need a full uninflected
  template, not just a plain verb". Examples: amass
  `"amass <Param(0):plural> <Param(1)>"`, conniveN `"connive <Param(0)>"`,
  connive `"connive"`, kicker `"kicker <Param(0)>"`, boast
  `"boast — <Param(0)>"`.
- Hints are overrides only. Each parameter kind has a default realization
  (Amount: arabic numeral; Cost: mana symbols; Subtype: singular) and a
  `:hint` is written only where the default is wrong. Owner: "lots of things
  are defaultable and you'd be inventing a bunch of spurious hint types to do
  it that way."
- `grammar` stays, untouched, as the word's own definition: "there's two
  separate and distinct functions: here's a new word and how it parses, and
  here's how it's used on a card. They're often very similar but not
  completely. maybe can be unified later but that's not what I'm asking you
  for." `bare`, `frame_set`, `FixedKeyword`'s fields and the inflections all
  stay.
- All declarations at once ("yes please"): 65 keyword actions
  (`plugins_v2/builtin/macros/keyword_actions/`, plus `conniveN`) and 192
  keyword abilities (`keyword_abilities/`).

## The work

1. **Hint survey, first.** Derive every template from today's `frame_set`,
   `params` and `keyword_syntax`, and list each slot whose default would
   misrender a card. The hint vocabulary is the minimum that list needs
   (`plural` is expected, for amass); write it in the landing record.
   Parameter kinds in use (2026-10-07): Amount, Cost, Subject, Ability,
   Quality, Subtype, Power, Toughness, TokenSpec, Instruction, Condition,
   Disclosure, Ballot, SearchScope, Quantity, String.
2. **The default table is a decision the landing records**: one default
   realization per kind in use, Amount, Cost and Subtype as above, every
   other kind stated.
3. **Reader.** `parse_spelling`
   (`crates/deckmaste_construction_core/src/macro_def.rs`, ~L2999) gains
   `<Param(n):hint>`; `SpellingPart::Param` carries the optional hint; an
   unknown hint is a load error. `validate_spelling_params` and the
   `spelling_head` / `GrammarSpellingMismatch` rule stand.
4. **Templates.** A Sonnet-mechanical pass writes every template. The
   `keyword_syntax` separators (`head_separator`, `parameter_separator`,
   `payload_order`; 15 abilities, e.g. boast's `SpacedDash`, landwalk's
   `BoundSuffix`) appear literally in the template. Decide, against the
   lexicon export, whether `keyword_syntax` stays as the lexicon's parse
   hint or is derived from the template, and state it.
5. **Tests.** A consistency test: each template's slots match `params`
   (every position, in range, once) and its literal head equals the grammar
   head. A test per declaration pins its template. The "must have one
   literal spelling part" assertion
   (`crates/deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`,
   ~L140-144) is re-spelled to assert the template shape, not deleted.
6. **Lexicon export.** `export_metadata`
   (`crates/deckmaste_lexical_source/src/legacy/plugins.rs`, ~L86-98) marks
   every spelling with a `Param` part "parameterized-spelling" unmapped;
   with every keyword declaration templated that marking is noise. Replace
   it with what the export does with templates, and state that. Lexemes
   come from `grammar`, which is untouched, so
   `cargo xtask lexical --card-name "Black Lotus" --export` stays
   byte-identical apart from the unmapped list the marking wrote: that is
   the proof.
7. **ADR.** In `docs/decisions/builtin-v2-macro-spelling-and-grammar.md`,
   note under the 2026-10-07 amendment that the corpus now matches the
   Decision's `scry <Param(0)>` example and that `<Param(n):hint>` is added;
   it is still not v1's `template:` language (no `${n:card|cards}`-style
   codec per site, no `kinds:` registry).

## Hands to `builtin-v2-spelling-frame-consumer`

That ticket compiles each declaration's `spelling` into a frame over the
parsed English. After this one it reads the template and its hints: the
template's slots and literals are the frame, and a hint tells it which
surface form of the argument to expect (amass's subtype is plural).

## Proof

The consistency test and the per-declaration tests pass; the Black Lotus
lexicon export's lexemes are byte-identical (diff before and after, any
change outside the unmapped list is a defect); the English v3 pins (generic predicates
"amass Orcs 1", amount complements "Scry 3.", the selected-frame tests) are
unchanged; `cargo xtask expansions` shows no change (spelling is not in a
body); `cargo xtask gate --changed --run` passes; state the command.
