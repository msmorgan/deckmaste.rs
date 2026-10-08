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
- **Default realizations (decided by this landing, 2026-10-07).** A slot
  with no hint realizes its argument as:

  | Kind | Default realization | Example |
  |---|---|---|
  | Amount | arabic numeral (or `X`) | "scry 2", "crew 3" |
  | Cost | mana symbols | "kicker {2}{R}" |
  | Subtype | singular | "Goblin" |
  | Quality | bare singular (an adjectival quality is numberless) | "Goblin offering", "Swampwalk", "splice onto Arcane", "protection from red" |
  | Subject | the argument's own noun phrase, determiner and all as the argument carries it; on a keyword line a determinerless nominal | "destroy target creature", "enchant creature" |
  | Ability | the ability's own text | "boast — {1}{R}: …" |
  | Condition | a declarative sentence | "companion — Each nonland card …" |
  | Power, Toughness | arabic numeral | "prototype {1}{R} — 2/1" |

  TokenSpec, Instruction, Disclosure, Ballot, SearchScope, Quantity and
  String occur only in the eight declarations the survey could not derive
  (below); their defaults are decided with those templates, not here.
- **Hint vocabulary: `plural`**, three slots: amass's subtype
  (`amass <Param(0):plural> <Param(1)>`, the owner's example), affinity's
  quality (`affinity for <Param(0):plural>`, which the grammar already
  declares as `nominal_number: Plural`) and protection's quality
  (`protection from <Param(0):plural>`: a bare nominal there is plural,
  "protection from creatures", while an adjectival quality is numberless,
  "protection from red"; the grammar leaves that slot's number open). `plural`
  is valid only on a nominal kind (Subtype, Quality).
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

## Survey (2026-10-07)

Derived mechanically (scratchpad script `templates.py`, named in the
landing record) from the grammar head, `params`, the verb `frame_set` or
`FixedKeyword` parameter class, and `keyword_syntax`:

- **Keyword abilities, 192:** 87 nullary (template = head), 105
  parameterized. After-head payloads join with `head_separator` then
  `parameter_separator` written literally (Space " ", Dash "—", SpacedDash
  " — "): "boast — <Param(0)>", "suspend <Param(0)>—<Param(1)>". A
  `parameter: Quality(preposition: …)` writes its preposition:
  "protection from <Param(0):plural>", "splice onto <Param(0)> <Param(1)>".
  `BeforeHead` is "<Param(0)> offering"; `BoundSuffix` glues the slot to
  the bound suffix: "<Param(0)>walk". The CostPowerToughness class writes
  its size as a slash pair (the v3 `CostPowerToughnessKeyword`
  construction's `SlashPair`): "prototype <Param(0)> — <Param(1)>/<Param(2)>".
- **Keyword actions, 66 (65 plus `conniveN`):** 28 with no or empty
  `params` keep the head; 30 map their `params` slot for slot onto one
  frame of their `frame_set` (an object-noun-phrase slot takes a Subject
  or Subtype, an Amount slot an Amount, a `Lex` preposition is written):
  "scry <Param(0)>", "connive <Param(0)>", "attach <Param(0)> to <Param(1)>",
  "amass <Param(0):plural> <Param(1)>".
- **Not derivable, STOP (8):** no frame realizes the params slot for slot,
  so any template would be a guess. `behold` [Quality] (Transitive; CR
  writes "behold a [quality]", the article is in no frame), `create`
  [Amount, TokenSpec] (one object slot, two params; the count is a number
  word), `faceAVillainousChoice` [Instruction, Instruction] (Intransitive),
  `fight` [Subject, Subject] (the first is the clause subject, not a frame
  slot), `meld` [Subject, String] (no "into" slot), `mill` [Amount] (the
  object is "N cards"; "cards" is in no frame and the count is a number
  word), `search` [SearchScope, Quantity, Quality] (two slots, three
  params), `vote` [Disclosure, Ballot] (no slot for the disclosure). Their
  spellings stay the bare head pending a ruling.
- **Hints:** the three `plural` slots above; no other derived slot's
  default misrenders a card.
- **`keyword_syntax` stays** as the lexicon's parse hint: the v3 keyword
  constructions select on the `KeywordSeparator`, `KeywordParameterSeparator`
  and `KeywordPayloadOrder` features the lexicon exports from it, and
  nothing reads a template yet. The keyword-ability consistency test
  re-derives each template from `keyword_syntax` and fails on any
  disagreement, so the two cannot drift.

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

## Landing record

Measured on the refreshed tree (claim `xlkquwvx`, stages `kuwpwmvu`
survey, `szkkroqv` reader, `zvrkmqpx` corpus, `mtkmtkxz` lexicon export,
`zuqtlvou` docs). This landing changes no English grammar or corpus parse,
so no lock count was measured.

- **Templates.** 135 declarations templated: 30 keyword actions (with
  `conniveN` as "connive <Param(0)>") and 105 keyword abilities. 115 stay
  as the head because they have no parameters (28 actions, 87 abilities).
  The rewrite is the scratchpad script `templates.py --write`, which only
  derives templates. Its rules are in the Survey section.
- **Hint vocabulary:** `plural`, 3 uses (amass's Subtype; affinity's and
  protection's Quality). An unknown hint is an `InvalidSpelling` load error.
  A hint on a kind it does not apply to is the new
  `ValidationError::SpellingHintKindMismatch`. The default table is under
  Decided and in the ADR.
- **Head rule.** `spelling_head` and `GrammarSpellingMismatch` still hold,
  reading the template's head literal: either it equals the grammar head,
  or it starts with the grammar head at a word boundary. A literal glued
  after a leading slot must be the grammar's bound suffix.
- **STOP, open: 8 keyword actions have no template** (behold, create,
  faceAVillainousChoice, fight, meld, mill, search, vote). Their params
  match no grammar frame slot for slot (reasons in the Survey). They keep
  the bare head, and the consistency test lists them by name in
  `PENDING_RULING`. That list fails once any of them is templated.
- **Lexicon export.** The "parameterized-spelling" unmapped marking is
  gone. The export reads `grammar` alone and records nothing for a
  template. `keyword_syntax` stays as the lexicon's parse hint. Proof:
  `cargo xtask lexical --card-name "Black Lotus" --output … --export …`.
  The export RON is byte-identical before and after (sha256 prefix
  `5a273b80bd15daac`). The report's `unmapped_sources` holds 87 entries
  both before and after, identical. The intermediate corpus-only state had
  222 (135 marking entries).
- **Expansions.** `cargo xtask expansions` before and after: `diff -r`
  shows no difference.
- **Tests** (`cargo test -p … --no-fail-fast`, all exit 0):
  deckmaste_construction_core 463 passed; deckmaste_lexical_source 56;
  deckmaste_semantics_v2 96; deckmaste_english_v3 366; 0 failed and 0
  ignored in each. `cargo clippy -p deckmaste_construction_core -p
  deckmaste_lexical_source --tests -- -D warnings`: clean.
  `cargo xtask lean check`: canon 134/134, testing 7/7. No lean/ edits.
  The gate was not run: the owner removed `cargo xtask gate`.
- **Counts.** Restored 0. Re-spelled 4: the keyword-actions "one literal
  spelling part" stub and the destroy spelling pin
  (`builtin_v2_keyword_actions.rs`); the keyword-abilities single-literal
  assertion, which now checks the template derived from keyword syntax, and
  the infinity/More Than Meets the Eye/web-slinging spelling list
  (`builtin_v2_keyword_abilities.rs`). Ignored 0. Added 5: two reader unit
  tests (hints; the head rule), the per-declaration pin test and the
  consistency test (`tests/builtin_v2_keyword_spelling_templates.rs`), and
  the lexicon test that no template is unmapped. Removed 0.
- **Cites.** `cite check --list-noncompliant`: 0. `cite check`: 0 stale.
  `cite audit --diff` from the claim: 5 sites (701.47a, 702.142a, 702.47a,
  702.48a, 702.14a). Each was read against its claim.
  `kata kanban check`: OK.
- **Deviations and additions.** (1) The `plural` hint on protection was
  not named by the ticket. The survey found that a bare nominal there
  prints plural. (2) Prototype's "/" between power and toughness comes
  from the CostPowerToughness class (v3 `SlashPair`), not from
  `keyword_syntax`. (3) Eight declarations are pending, as listed above.
  (4) The head rule was widened to word-boundary and bound-suffix matching,
  because separators, prepositions, before-head payloads and bound
  suffixes could not otherwise pass. (5) `SpellingPart::Param` became
  `Param { index, hint }`.
- **Glossary gap:** none found.
