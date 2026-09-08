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
