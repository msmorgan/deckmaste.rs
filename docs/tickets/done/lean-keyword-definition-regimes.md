---
needs: []
---
**A keyword definition's triggered parts need not all share the keyword's
stack regime.** Found by `semantics-v2-macro-bodies-keyword-abilities`
round three (2026-09-07): `keywordBodyPartFits` requires every triggered
body part to key on the keyword's own `regime`, but Offspring
[CR#702.175a] and Squad [CR#702.157a] are cast-time keywords (`AtCasting`) whose second
ability is an enters trigger, and Impending's landed body carries the same
mismatch latent (no canon card reads it yet; the first one will refuse).
Decide the law from the CR: a definition's parts may carry different
regimes when the keyword's entry defines abilities that function in
different places; likely the regime becomes a per-part declared fact or the
law checks only the part that carries the keyword's own regime. Re-spell
the pins, mirror if the facts row shape changes, then body Offspring and
Squad and add a canon card that reads Impending. Standard constraints
apply.

**Gift too** (stopped `plugins-v2-gift-variants` attempt, 2026-10-05). Gift's
row is `regime := some .atCasting` (`lean/Semantics/Check/Facts.lean`) because
its first ability is an additional cost that works on the stack
[CR#702.174a]; its permanent form's second ability is "When this permanent
enters, if its gift cost was paid, [effect]" [CR#702.174b].
`bodyEventRegime` (`lean/Semantics/Check/Abilities.lean` ~L454) gives an
enters event `none`, so `keywordStackRegime "Gift" == bodyEventRegime ev`
fails and `keywordBodyFits "Gift"` refuses the trigger. Same law, same fix;
add a gift-permanent pin with the others. Gift's spell form is a separate
refusal, owned by `semantics-v2-keyword-definition-by-card-class`.

## Decided 2026-10-06

**Shape (a): the regime is a per-part declared fact, and each part is
checked against its own.** Nothing is exempt: no part skips the law. Kicker's
definition is the one-part case (its body is only `static(addedCost(…))`,
`plugins_v2/builtin/macros/keyword_abilities/kicker.ron`) [CR#702.33a]
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords). The rules give each part's regime:

- offspring and squad: the second ability is an enters trigger
  [CR#702.175a,702.157a];
- casualty: the second ability is a cast trigger that functions on the stack
  [CR#702.153a];
- gift: the second ability is an enters trigger on a permanent and a spell
  ability on an instant or sorcery [CR#702.174b] (the spell form waits on
  `semantics-v2-keyword-definition-by-card-class`).

Impending's latent mismatch is caught by the per-part check, not skipped.

## Landing record

Measured on change `slktvwupsopv` (tip of the claim after `kata refresh`,
2026-10-08). No English grammar, lexicon or corpus input changed, so the lock
`covered` count and construction count are not moved by this landing and were
not re-measured; the lexicon export (`cargo xtask lexical --card-name "Black
Lotus" --export`) is byte-identical with and without this landing's plugin
change (comments only) on the refreshed tree.

### PROVE

- **No silent loss.** `lake build --wfail` (87 jobs); every existing pin keeps
  its outcome (two rows written in pin files re-spelled, below). `cargo xtask
  lean check` canon 134/134, testing 7/7; `cargo xtask lean definitions`
  553/553 (73 counters, 461 subtypes, 19 designations). No identity stopped
  being covered.
- **Structural laws.** `cargo xtask facts check`: `Facts.lean` and
  `FactsGen.idr` up to date; the Idris table is unchanged (every overlay
  regime already equals its components' summary). Rust/Lean drift test green
  with `ComponentFacts` mirrored.
- **No word-naming.** The law reads forms (`Ability.regimeFits`) and declared
  rows only; no guard names a keyword, card or lexeme.

### DISCLOSE

- **The device (shape (a)).** `KeywordFacts.definition : List
  ComponentFacts`, each a category and an `Option StackRegime` (`none`: off
  the stack). `KeywordFacts.bodyPartFits` admits a part only as a declared
  component of its category whose regime its form admits. Form-determined: a
  triggered part keys on `bodyEventRegime` (cast at casting; enters and every
  other event off the stack); a spell ability at resolution [CR#113.3a]; an
  `addedCost` or `costShift` at casting [CR#601.2f]; activated and other
  static parts off the stack. **Declared override where the form
  underdetermines it:** a permission to cast the card (`altCost`, a permitting
  `deonticRule` with a `play` rider) admits at casting or off the stack, never
  at resolution [CR#113.6b,113.6d]; the generator reads it from the overlay's
  `open_regimes`: surge, spectacle, web-slinging and impending component 0 at
  casting [CR#702.117a,702.137a,702.188a,702.176a]; flashback, escape, mayhem,
  retrace, evoke, bestow and aftermath default off the stack. An entry on a
  form-determined component is refused.
- **The row's `regime` becomes the derived summary** for every keyword whose
  definition is declared (first stack regime among the components); it stays
  declared only where no definition is (convoke, deathtouch, …), since grants
  read it (`grantSubjectFits`). The generator refuses a disagreeing overlay
  value; `Proofs/Tables` pins `everyDefinedKeywordRegimeIsItsComponentsSummary`.
  No row's regime moved.
- **Impending** now fits its own row (end-step trigger off the stack); its twin
  under the old single casting regime is refused (`badImpendingSingleRegime`).
- **Not a game-rule refusal:** a mismatched part is a term-shape refusal
  (`keywordBodyFits`, existing), charter §7.
- **Deviations and additions.** New `Proofs/KeywordDefinitionRegimes.lean`
  (15 pins: kicker one component and its off-stack twin; offspring, squad and
  their single-regime twin; casualty and an enters-trigger twin; gift
  permanent, gift spell, spell-at-casting twin, gift summary; impending and
  its twin; cast-permission regimes). Offspring, squad, casualty and gift rows
  are written in the pin file (no declared definition yet). New Tables pin.
  Three xtask tests. Offspring and squad `.ron` STOP comments updated
  (comments only). The ticket's opening paragraph asks to body offspring and
  squad and add a canon impending card; not done here (out of the decided
  scope), routed to `semantics-v2-keyword-ability-residues`.
- **Glossary.** Added **Stack Regime** (game-model). `lean/CONTRACTS.md`
  gains "Each component of a keyword definition carries its own regime".
  ADR §12 count unchanged (no macro added).
- **STOPs.** None.

### Assurance counts

Restored 0; re-spelled 3 (`KeywordByCardClass.bothClassesRow`,
`LinkedChoiceReadback.giftShapedRow` definitions as components; xtask
`a_class_guarded_spell_ability_declares_the_spell_category` expected row);
ignored 0; added 19 (15 Lean pins, 1 Tables pin, 3 xtask tests); removed 0.

### Tests

`cargo test -p deckmaste_semantics_v2` (all green), `cargo test -p xtask`
(374 passed, 1 ignored as before; 4 and 14 in the other suites); `cargo clippy
-p deckmaste_semantics_v2 --tests -- -D warnings` and `cargo clippy -p xtask
--tests -- -D warnings` clean; cite checks 0 non-compliant, 0 stale, the diff
from the claim audited (50 sites), `bless` added nothing; `kata kanban check`
OK.

### REPORT

`lean check` 106 seconds on a host with other sessions running; no coverage
command was run, as no English input changed.
