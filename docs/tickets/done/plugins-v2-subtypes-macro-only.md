---
needs: [semantics-v2-definition-bodies]
---
**A subtype in a card is macro-only.** Ruling (user, 2026-09-07). Canon
cards spell `subtypes: [Of(host: Creature, label: "Gargoyle")]`; the
declaration `subtypes/creature/gargoyle.ron` exists (324 creature subtypes,
seven categories) but the `Subtype` meta emits an empty body, so `gargoyle`
is not invocable, and `Subtype` is not among the reader's restricted
expression kinds, so the raw constructor is accepted in card text.

Three parts: (1) the `Subtype` meta derives the body from the declaration
— `Of(host: <category>, label: <spelling>)` for object subtypes,
`Spell(label: <spelling>)` for spell subtypes — so no declaration writes a
body by hand; (2) `Subtype` joins the restricted kinds in
`deckmaste_semantics_v2::ron` (card text refuses a raw `Of`/`Spell`,
listing the names); (3) every canon and testing card is rewritten to
invoke the declaration (`subtypes: [human, knight]`), by a scripted
name-directed rewrite over the `subtypes:` field only (multi-word
subtypes take the camelCase name under the case rule, e.g. `timeLord`),
with `lean/Generated` byte-identical, `lean-check` 118/118 and 2/2, and
`facts check` byte-identical as the oracles. Counter kinds
(`Named(label: "charge")`) are the same question and a candidate for the
same treatment; do not fold them in without a ruling. Standard
constraints apply.

## STOP (claimant, 2026-09-07) — nothing landed

The ticket's premise, "the `Subtype` meta emits an empty body", holds for 458
of the 462 subtype declarations. It does not hold for four:

| declaration | body it writes today |
| --- | --- |
| `subtypes/artifact/equipment.ron` | `Subtype(name: "Equipment", types: [Artifact], confers: [Static(May(attach(what: Ref(This), to: Type(Creature))))])` |
| `subtypes/artifact/fortification.ron` | the same shape, `to: Type(Land)` |
| `subtypes/enchantment/aura.ron` | `Subtype(… confers: [StateBased(condition: Not(LegallyAttached(This)), effect: Move(This, Graveyard))])` |
| `subtypes/enchantment/saga.ron` | `Subtype(… confers: [Ability(Static(Replacement(…))), TurnBased(…), StateBased(…)])` |

That body is not a `deckmaste_semantics_v2::words::Subtype` value at all — it
is an engine type-rule record (v1 core's subtype declaration: `name`, `types`,
`confers`). Deriving the body as the ticket asks destroys it.

**The contradiction.** Two recorded rulings say the subtype declaration's
`body` is registry data, not a syntax term:

- `docs/tickets/done/builtin-v2-noncreature-subtype-stubs.md`: "Existing
  rules-defined conferrals remain on the same declaration."
- `docs/decisions/semantics-v2.md` §5 (ruling 2026-09-07,
  `facts-generator-sheds-v1`): "A registry declaration whose columns are the
  ones Lean's checker reads carries THOSE columns as its body, typed by the
  `facts` mirror rather than by a semantics term."

That is not only history. `crates/xtask/src/facts/lean.rs::subtype_rows`
carries a live note — "Frame columns are not yet present in the subtype
declarations" — while it emits the `subtypeFacts` table from xtask-side
overlays for Saga, Adventure and Room. Making the body the syntax value
forecloses the registry-column direction that comment plans for.

Also pinned by a test:
`crates/deckmaste_construction_core/tests/builtin_v2_noncreature_subtypes.rs::rules_defined_conferrals_stay_on_their_subtype_declarations`
asserts the four bodies byte-for-byte AND that every other non-creature
subtype has `body().is_none()`. Both halves fail under a derived body; the
first half is a genuine data loss, not a re-spelling.

**Why it blocks steps 2 and 3 too, not just step 1.** Seven canon card sites
write those subtypes — `Arcanum Wings`, `Eldrazi Conscription`, `Boar Umbra`
(Aura), `Flayer Husk`, `Leech Gauntlet`, `Sylvok Battle-Chair`,
`Warrior's Sword` (Equipment). If `equipment` and `aura` keep their conferral
body they are not invocable, so those seven cannot be rewritten to the
declaration name, so `Subtype` cannot join the restricted kinds without
refusing seven cards that have no macro to write instead. A half-converted
corpus — 145 sites on macros, 7 on raw constructors — is worse than either
end state and would have to be unpicked on resume, so nothing was landed.

**Recommendation, for a ruling.** Give the rules-defined conferral record a
home in the declaration's `metadata`, alongside `spelling`, `grammar` and
`noun_class`, and leave `body` to mean the macro's expansion everywhere. The
metadata half is already the consumer-specific half — `deckmaste_semantics_v2`
reads it as `OpaqueMetadata` and discards it, `deckmaste_construction_core`
types it — which is exactly what the conferral record is: engine data on a
declaration whose macro expansion belongs to the reader. It preserves all four
records and their CR-cited comments verbatim, costs one field on
`macro_def::Metadata` plus a re-spelling of the test above, and generalizes to
`CounterKind` when that family faces the same question. The alternative —
moving them into `plugins_v2/builtin/rules/{grant,sba}/` — is semantically the
v2 home but is a larger migration and does not fit Saga's `TurnBased` record,
which has no v2 rules table.

### Step 4's question, answered: counter kinds are NOT a candidate

All 71 `macros/counter_kinds/*.ron` declarations carry a
`CounterFacts(label: …, holder: …)` body, and `cargo xtask facts generate`
reads exactly that body (`crates/xtask/src/facts/lean.rs::counter_rows`) to
write the `counterFacts` table in `lean/Semantics/Check/Facts.lean`. So the
`CounterKind` body is fully occupied by registry columns under the §5 ruling,
for every declaration rather than four of them. Making
`Named(label: "charge")` macro-only collides with that head-on and would break
`cargo xtask facts check` byte-identity. It needs the same ruling as the
subtypes, and it needs it first.

### Design facts established, so a resume need not re-derive them

- **The meta cannot branch.** `macro_ron` reserves exactly three body idents —
  `Param`, `Splice`, `Quote` — and has no conditional or match form, so one
  meta cannot emit `Of(host: …, label: …)` for six categories and
  `Spell(label: …)` for the seventh: `Spell` is a `SubtypeCategory` but not a
  `CardType`, so `Of(host: Param(category), …)` cannot read for the spell
  directory. Two metas are needed — `Subtype` keeping its `category` param,
  and a `SpellSubtype` that emits `category: Spell` and the `Spell` body
  itself, added to `macro_def.rs`'s hardcoded `DECLARATION_META_MACROS`. The
  path check in `expected_builtin_identity` makes the split safe: a file
  under `subtypes/spell/` invoking the wrong meta is refused at read.
- **Step 2 needs no dispatch set.** Adding `"Subtype"` to `EXPRESSION_KINDS`
  and dropping its `HAND_BUILT_NATIVE` row is enough. The refusal that names
  the constructor and the kind — "`Of` is not author vocabulary at `Subtype`;
  it names a `Subtype` variant with no macro of that name" — reads serde's own
  variant list at `deserialize_enum`, not the `Kind`'s dispatch set, so
  `words::Subtype` need not derive `SupportsMacros`.
- **Step 3's rewrite is mechanically clean.** 152 `Of(host: X, label: "Y")`
  sites in `plugins_v2/{canon,testing}` cards and tokens (150 canon over 91
  files, 2 testing), 68 distinct `(host, label)` pairs, every one resolving to
  an existing declaration file, none needing a multi-word camelCase fold. No
  `Spell(label: …)` site exists anywhere in the corpus. Note that the ticket's
  "the `subtypes:` field only" is too narrow for its own oracle: five of the
  150 canon sites stand at `hasSubtype(subtype: …)`, `amass(…)` and
  `changes: (subtypes: …)`, and a restricted read refuses a raw `Of` there
  too, so the rewrite must be over the `Of(…)`/`Spell(…)` value wherever it
  stands in a card.
- **Out of scope, and it can stay that way.** 26 further sites in 16
  `plugins_v2/builtin/macros/**` bodies are definition text, which a
  restricted read never sees, so they need no rewrite.
- **Baseline on `pkqyxotk`, `nproc` 24, load average 20.90 (sibling
  workspaces building):** `lean-check`
  `plugins_v2/canon` 118/118 (cold run, about two minutes) and
  `plugins_v2/testing` 2/2;
  `cargo xtask facts check` both files up to date; `cargo test -p
  deckmaste_semantics_v2 --test corpus` 6/6 green — 2414 declarations across
  31 kinds, 2215 source files, 124 nullary declarations expanding with 297 at
  untested kinds, 120 cards round-tripping.
- **An addition worth making on resume:** `every_nullary_helper_expands` in
  `tests/corpus.rs` has no `"Subtype"` arm, so all 462 subtype declarations
  sit in its 297 "untested kinds". Adding the arm exercises every derived body
  as a real `words::Subtype`.

## Handoff

Nothing landed; the tree is unchanged apart from this section. The workspace
is parked with `@` empty and is NOT integrated. Resume needs a ruling on where
the four rules-defined conferral records live, and the same ruling settles
whether `plugins-v2-counter-kinds-macro-only` is mintable at all.

Re-pinned (2026-09-07) after the STOP recorded above: the `Subtype` meta
does not derive a facts row; the body is the `Subtype(category, label,
rules)` definition node that `semantics-v2-definition-bodies` introduces,
and this ticket runs after it. The design facts in the STOP record stand
(two metas, no dispatch set needed, 152 sites plus five non-`subtypes:`
sites, no multi-word labels).

## What `semantics-v2-definition-bodies` changed, and what it did not (2026-09-08)

Landed: `Semantics.Definition` exists, and 458 of the 462 subtype declarations
now carry `body: Subtype(subtype: Of(host: <Category>, label: "<Spelling>"),
rules: [])`. `cargo xtask facts generate` reads the subtype identity from that
body, so `subtype_rows` no longer capitalises a declaration name. A guard,
`facts::lean::subtype_definitions_read`, refuses any subtype declaration whose
body is not a `Definition::Subtype` unless it is one of the four named below,
so the exception list can only shrink.

NOT landed: the four rules-defined conferrals. Each keeps the v1 core type-rule
record with a STOP at the head of its file, because each needs syntax v2 lacks:

| declaration | what it needs |
| --- | --- |
| `equipment`, `fortification` | the [CR#301.5,301.6] host rule is a deontic over the `Attach` deed [CR#701.3a], whose `actFacts` row declares no patient role, so "…to a creature" has no role to bind |
| `aura` | [CR#704.5m] fires on "attached to an illegal object or player, OR is not attached"; `Predicate.attachment` spells only the second half |
| `saga` | [CR#714.4] reads the final chapter number off the Saga's own chapter abilities [CR#714.2d]; `Amount` has no aggregate over a card's chapter marks |

So this ticket's step 1 is now: teach the syntax those three things, then write
the four `rules` lists. Steps 2 and 3 are unchanged and still blocked on step 1
for the seven canon sites the STOP above lists. The step-4 question (counter
kinds) is answered differently now: every counter declaration carries a
`Definition::Counter`, so a counter macro CAN expand to one, and the remaining
question is only whether a card's `CounterKind` position should read the
definition node or the `words::CounterKind` inside it.

## Landing record

Measured on change `umlvturs` (working copy over `rzqlynty a3c0206b`), `nproc`
24, load average 3.34.

**Coordinator rulings (2026-09-08) that this landing implements, and which
override the "step 1 is now: teach the syntax" note above.**

- **R1 — a definition's name denotes its term.** A subtype declaration's body
  stays the `Definition::Subtype(subtype:, rules:)` node; where the macro
  stands at a `Subtype` TERM position, the position takes the definition's
  `subtype`.
- **R2 — the four rule-bearing subtypes convert now, rules routed.**
  `equipment`, `fortification`, `aura`, `saga` carry an empty `rules` list, keep
  their v1 record verbatim on the declaration, and the three syntax gaps go to
  the newly minted `semantics-v2-subtype-rules`.

### PROVE

**No silent loss.** Nothing stopped being covered.

- The four v1 core type-rule records are preserved VERBATIM as a comment block
  on their own declaration files, under a rewritten STOP that names
  `semantics-v2-subtype-rules` as their owner. They are pinned by text:
  `rules_defined_conferrals_stay_on_their_subtype_declarations` reads each file
  and compares the preserved block against the same four strings it used to
  compare the body against.
- 152 corpus sites moved from a constructor to a declaration name. Every one
  resolved to an existing declaration file; the rewrite script exits non-zero
  and names any `(host, label)` pair with no declaration, and it named none.
- No card, token, declaration or rules row was deleted. Only two files were
  added (`macros/meta/SpellSubtype.ron`, the minted ticket); none removed.

**Structural laws.**

- `lean/Generated/{Canon,Testing}.lean` are **byte-identical** across the corpus
  rewrite (`diff -r` against the pre-rewrite copy: no output).
- `cargo xtask lean-check plugins_v2/canon plugins_v2/testing`: 118/118 and 2/2.
- `cargo xtask facts check`: `lean/Semantics/Check/Facts.lean` and
  `idris/src/Experimental/FactsGen.idr` both up to date — byte-identical, with
  the exception list emptied and the name-derived fallback gone.
- `every_card_writes_and_reads_back_to_the_same_value`: 120 cards round-trip.
- `every_nullary_helper_expands` now reads all 462 subtype declarations at a
  real `words::Subtype` position.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition or card identity. The one name-keyed list touched,
`SUBTYPE_DEFINITION_STOPS`, went from four declaration names to EMPTY; the
guard around it stays, so a future exception has to be written down to exist.

### DISCLOSE

**Newly covered.**

- All 462 subtype declarations derive their `Definition` body from the
  `Subtype`/`SpellSubtype` meta; none writes one by hand, and a subtype
  declaration that tries is now a read error (`DiagnosticSubtype` declares no
  `body` field).
- `Subtype` joined `EXPRESSION_KINDS` and left `HAND_BUILT_NATIVE`, so a card
  writing `Of(host: …, label: …)` — or the definition node itself — is refused
  by name at the `Subtype` kind.
- Lean gained `Semantics.Definition.subtypeTerm`; Rust mirrors it as
  `Definition::subtype_term`, wired through the `Subtype` position's serde shim.

**Corpus counts, before → after.**

| | before | after |
| --- | --- | --- |
| `plugins_v2/builtin` declarations | 1785 across 30 kinds | 1786 across 30 kinds |
| source files with no elided constructor | 1586 | 1587 |
| nullary declarations expanding | 215 | 677 |
| declarations at untested kinds | 666 | 204 |
| `Of(…)`/`Spell(…)` sites in canon+testing | 152 over 93 files | 0 |
| subtype declarations with a hand-written body | 462 | 0 |
| `subtype_definitions_read` exceptions | 4 | 0 |

The declaration and file deltas are exactly the added `SpellSubtype` meta. The
nullary delta is exactly the 462 subtype declarations, which the new `"Subtype"`
arm moved out of "untested kinds".

**Assurance counts.** restored 0; re-spelled 3; ignored 0; added 2 tests plus
one new arm in an existing test; removed 0.

- Re-spelled `rules_defined_conferrals_stay_on_their_subtype_declarations`
  (construction_core): same four declarations, same four record strings, now
  read off the preserved comment block instead of the body, plus the derived
  `Definition` body asserted on all 138 non-creature subtypes uniformly (the
  four are no longer skipped).
- Re-spelled `a_word_types_constructor_still_reads_under_restriction`
  (semantics_v2 reader): the colour half is untouched; the subtype half, whose
  subject this ticket deliberately retires, is replaced by a counter kind
  (`Named(label: "charge")`), still a hand-built native word type. The retired
  half is re-spelled as its own test — see below.
- Re-spelled the body assertion inside
  `builtin_v2_creature_type_nursery_matches_catalog_and_attested_morphology`:
  same 324 creature types, same asserted node, compared with whitespace
  normalized rather than byte-for-byte.
- Added `a_subtype_macro_denotes_the_subtype_its_definition_names` (R1's first
  pin: `gargoyle` → `Of(host: Creature, label: "Gargoyle")`, `adventure` →
  `Spell(label: "Adventure")`).
- Added `a_subtype_in_a_card_is_the_declarations_name_and_not_its_constructor`
  (R1's second pin: `goblin` reads under restriction; a raw
  `Of(host: Creature, label: "Goblin")` and the raw definition node are both
  refused with a message naming the constructor and `Subtype`).
- Added the `"Subtype"` arm to `every_nullary_helper_expands`.

**Deviations and additions.**

1. **Step order.** R2's conversion of the four rule-bearing declarations landed
   in the SAME commit as step 1's meta derivation, not as a separate step 3: the
   rewritten meta takes no `body` argument, so the four could not stay on the
   old form for even one commit.
2. **`construction_core::macro_def` gained `BodySource`.** A declaration's body
   was gated on the AUTHORED source position, so stripping the `body:` line
   would have made every subtype bodyless and broken `facts generate`. A body is
   now "whichever body the meta left", and the enum distinguishes an authored
   body (a position to report against, a signature to check `Param` holes
   against) from a derived one. Confined to the subtype metas, whose diagnostic
   schema declares no `body` field; `AbilityWord` and the rest keep the previous
   meaning, and `builtin_v2_ability_words`' "must have no body" assertions still
   pass unchanged. (A first attempt gated on the unit sentinel instead and broke
   those two assertions; it was replaced rather than shipped.)
3. **`DiagnosticSubtype` reshaped.** `category` became optional so
   `SpellSubtype` shares the struct, `body` was dropped and `rules` added, so
   the diagnostic schema mirrors the two metas' signatures. Effect: a subtype
   declaration writing `body:` by hand fails to read.
4. **`xtask::facts::lean` shed dead code.** `subtype_of`'s name-derived fallback
   (whose own doc comment said it retires with the four STOPs) and the
   now-unused `surface` helper were deleted; `subtype_of` refuses a subtype
   declaration whose body is not a `Definition::Subtype`.
5. **Lean defines only the subtype projection.** `Definition.subtypeTerm` names
   the counter's `kind` and the designation's `label` in its doc comment as the
   same rule, but defines neither, per the ruling's "wire and pin only
   `Subtype`".
6. **The Lean/Rust drift scan reads public declarations only.** The `Subtype`
   position's serde shim is a private enum in `words.rs`; the scan previously
   reported it as a Rust declaration Lean lacks. The guard does not weaken — a
   mirrored type that lost its `pub` is still reported from the Lean side.
7. **Whitespace in a derived body.** `macro_ron` eats the space before a
   DEFAULTED param's substituted value, so a derived body reads
   `…, rules:[])` rather than `…, rules: [])`. Observed, not fixed: it is
   pre-existing expander behaviour with no effect on meaning, and the two
   construction_core tests that compare a body now normalize whitespace (the
   established `compact_ron` idiom in that file, now doc-commented).
8. **Glossary amendment.** `docs/contexts/game-model/CONTEXT.md`'s **Registry
   Definition** entry gained one sentence: the declared name DENOTES what is
   defined, so a position wanting the defined term takes the term rather than
   the node. That is R1's concept, and it had no entry.
9. **Minted `docs/tickets/planned/semantics-v2-subtype-rules.md`**, carrying the
   three syntax gaps verbatim and the four records' file locations, and holding
   the "write the four rules lists" residue.

**STOPs.** None raised. R1's fence — "if the seam forces a textual hack
(string-matching the expansion), STOP" — was not reached: the projection is a
typed serde shim that reads the expansion as this crate's `Definition` mirror
and calls `Definition::subtype_term`, which mirrors Lean's `subtypeTerm`. It
lives in `deckmaste_semantics_v2` rather than `macro_ron` because the rule is
typed and `Definition` is this crate's type; the expander's seam knows only
text and kind names.

**Glossary gap.** One, amended above (deviation 8).

### REPORT

Provenance, stamped on change `umlvturs` over `rzqlynty a3c0206b`.

- `cargo xtask gate --changed` derived, and this landing ran:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_english_v3 -p deckmaste_construction_v3 -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p xtask` — green, no failures.
- `cargo fmt --all`; `cargo clippy --workspace --all-targets` — no warnings.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
  `cargo xtask cite check`: 15858 citations, 0 stale. No rule needed blessing.
  `jj diff --from plugins-v2-subtypes-macro-only --to @ --git | cargo xtask cite audit --diff`:
  16 sites, each read against its rule text.
- Performance advisory. `cargo xtask lean-check plugins_v2/canon plugins_v2/testing`:
  111 s cold on the pre-change tree at load average 20.90 (sibling
  workspaces building), 35 s warm on the post-rewrite tree at load average
  3.34, 24 workers both times. `cargo test -p deckmaste_semantics_v2 --test corpus`:
  under a second. The english coverage ceiling does not apply to this lane; no
  english_v2 corpus command was run.
- Scoped scripts, both in the session scratchpad and neither in version
  control: `strip_subtype_bodies.py` (458 declarations, 5 onto `SpellSubtype`)
  and `subtypes_to_macros.py` (152 sites over 93 files).
