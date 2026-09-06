---
needs: []
---
# Fold event patterns and static applicability forms

Implement the concrete event and static-spec folds below, preserving their
existing scopes. Apply the
[campaign contract](../done/lean-core-bindings-and-composition.md#campaign-contract).
Scope narrowed by the user on 2026-09-06; no new provenance, relation-query,
or general as-event instruction mechanism is a prerequisite.

## Event patterns

- Replace `GameEvent.dies`, `leaves`, `enters`, and `putInto` with macros over
  a zone-transition pattern. Preserve origin/destination constraints and
  whether the subject is observed before or after the transition. Optional
  endpoints must not silently admit meaningless combinations.
- Share the damage-event pattern underlying `isDealtDamage` and `dealsDamage`.
  Active/passive wording and textual argument order belong to macros, not a
  semantic voice field. Preserve damage roles and the binding order exposed
  by those macros.

## Static specs

- Fold `entryChoice` and `attachmentChoice` into a single choice spec with
  an entry/attachment occasion enum, subject, choice sort/domain, and
  disclosure where applicable. Preserve the original event and the current
  publication of declared choices across ability lines. Keep the two
  authoring forms as macros. This is an occasion-enum fold, not a general
  construct for performing arbitrary instructions as events occur.
- Express turn-position applicability through ordinary conditions, eliminating
  `partScope` in favor of shared conditional applicability. Preserve whose
  turn/part is described.
- Expand `offBattlefieldScope` into explicit affected selections using the
  existing noun/predicate vocabulary. Preserve control on the battlefield
  versus ownership in the relevant other zones. Arcane Adaptation and
  Encroaching Mycosynth are witnesses; a general relation-query family is
  unnecessary for these explicit selections.

Keep `establish`. Numeric definition folding belongs to the composition ticket;
ability-removal edits belong to the characteristic ticket. Those are ownership
boundaries, not prerequisites for this ticket's changes.

## Completion evidence and deferred work

Re-spell affected event and static witnesses. Check transition observation
points, both damage phrasings, entry/attachment occasion and linked choice
publication (including Sanctuary Blade and Psychic Paper), and explicit
owner/controller domains. Run `lean/scripts/build`. Standard constraints apply.

Leave the current provenance reads and protection retention in place.
[Action provenance](../maybe/lean-core-action-provenance.md),
[protection retention](../maybe/lean-core-protection-retention.md), and the
[general as-event instruction construct](../maybe/lean-core-as-event-instructions.md)
are separate design tickets. The latter does not block the occasion-enum fold.

## Result

All event and applicability folds are complete. Damage shares a participant
pattern, entry/attachment choices share an occasion, and zone transitions share
explicit endpoints and an observation point. Named authoring forms remain macros.
Turn applicability is an ordinary condition. Off-battlefield applicability names
the actual controlled and owned collections directly.

The user clarified that the Lean prototype must be self-consistent and
rules-correct, with no compatibility obligation to obsolete checker behavior.
That authorization supersedes the earlier preservation questions. The affected
checks and witnesses were corrected rather than deferred.

## Landing record

### PROVE

Measured on `mqoxyntz` over the earlier choice/damage fold `moklsruy`, with
English lock covered count **20,254**.

- Full Lean gate: **75 jobs passed**, warnings fatal. Focused event, scope, and affected
  card checks pass (**33 jobs**); the independent reviewer reran all original
  repros and confirmed their fixes.
- Rust closure: `cargo xtask gate --changed` reports **no affected workspace
  crates**.
- Existing checked card declarations: **815 retained**, **0 removed**, and
  **1 added** (`Cards.Trigger.dread`), for **816**. Dread represents its printed
  Fear ability, damage trigger, and graveyard-arrival trigger with a Shuffle
  expansion. This is a Lean witness, not a new English parser identity.
- Named proof inventory: **1,598 → 1,638**. **40 new** event/scope proofs,
  **68 re-spelled or corrected** (including the two replacements below),
  **0 restored**, **0 ignored**, and **0 assertions dropped without replacement**.
- Citations: **0 noncompliant, 0 stale**. The combined feature diff contains
  **7 changed citation sites**, each read against the local rules text.
- Parser identity coverage, roundtrip, lexical ownership, construction/traversal
  identity, tie and failure counts, and licensing-checker totals were not
  remeasured; no parser or grammar implementation changed. No new checker guard
  names a card or surface lexeme.

### DISCLOSE

Two retired witness names have explicit replacements:

- `Anaphora.badPlacementIntoBattlefield` → `okPlacementIntoBattlefield`:
  battlefield arrival is a valid zone-change pattern. The old refusal depended
  on the spelling used for that pattern.
- `Static.badDoubleExtension` → `badContradictoryOutsideSelection`: nesting the
  removed wrapper no longer denotes a distinct semantic error. Its replacement
  refuses an actually contradictory affected selection. `okSingleExtension`
  now checks the explicit three-domain selection.

Five existing negative diagnostics change while retaining their rejection:
`Anaphora.badEntryOriginBattlefield` uses the shared endpoint contradiction;
`Zone.badDiesInGraveyard` and `badWouldDieInGraveyard` use the shared observed-zone
check; `Zone.badCardTokenTarget` reports the card/token contradiction without a
spurious missing-zone diagnostic; `Damage.badGetsSource` reports the missing
battlefield evidence for both characteristic modifications instead of inventing
battlefield evidence between them. The other re-spellings preserve their asserted
results.

Deviations and additions:

- `ObservationPoint` names the before/after distinction, documented in the Game
  Model glossary and workbench contract. No action-provenance state was added.
- Zone checking now preserves positive zone evidence on card descriptions and
  keeps placeless cards/sources placeless when publishing their bindings.
  Qualified exclusions remain conservative where player identity is unknown;
  repeated indefinite phrases are not treated as equal references merely
  because their syntax matches.
- Endpoint checking, historical mentions, and continuation publication share
  subject → origin → destination order. Later endpoints can read players
  introduced by earlier fields. Historical observations do not move references.
- Review found and fixed discarded departure evidence, false battlefield
  publication, overly broad player-zone exclusions, missing endpoint scope,
  impossible origin sets, and whole-turn rejection. Direct witnesses cover
  each; the reviewer reran the original repros successfully.
- Celestial Dawn joins Arcane Adaptation, Conspiracy, Maskwood Nexus, and
  Encroaching Mycosynth in the explicit-selection migration. Its controlled
  spell domain is broader than Mycosynth's permanent-spell domain.
- Colossal Grave-Reaver, Voracious Brood, and Syr Konrad now describe arriving
  cards in after-observation patterns. Dread supplies the compact additional
  "graveyard from anywhere" witness. Exact card text was verified locally.
- Three obsolete refusal constructors and two unused origin-spelling helpers
  are removed. Protection retention and all parked mechanisms remain deferred.

### REPORT

Core `GameEvent`: **34 → 30**. Core `StaticSpec`: **28 → 25**.
`Condition`: **17 → 18**, adding the turn-position condition. The helper enums
`ObservationPoint` and `ChoiceOccasion` each have **2** constructors. Instruction
stays **62** on this ticket. These helper counts are reported separately from the
core reductions.

English lock coverage is unchanged at **20,254**. Parser selection census,
construction count, homograph/form-overlap inventories, and performance were not
remeasured. No parser performance or runtime engine execution claim is made.
