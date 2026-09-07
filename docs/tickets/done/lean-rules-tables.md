---
needs: [semantics-v2-crate]
---
**Model the rules tables in Lean so they port to v2 with the cards.** The
six files under `plugins/builtin/rules` are semantics-language RON: four
state-based actions [CR#704.5f,704.5g,704.5h,704.5i,704.5w], one conferral
(planeswalker enters with loyalty, [CR#306.5b]), one damage result (damage to
a planeswalker removes loyalty, [CR#120.3c]). Lean models none of them:
`StateBasedCause` has one constructor used only as a deontic rider, and there
is no conferral, damage-result, or predefined-token catalog (`TokenSpec`
covers created tokens only; the seven predefined tokens under
`plugins/builtin/tokens` have no Lean shape). Idris never modelled them
either.

Add the three table shapes and the predefined-token catalog to the Lean
syntax with their checker reads and pins, then mirror them in
`deckmaste_semantics_v2` and its reader. Governing decisions:
`state-based-actions-are-data.md`, `conferrals-come-from-registries.md`,
`minimal-core-data-driven-rules.md`. Standard constraints apply.

## Landing record

Change: `mkllzpkzvwmq` (`rules tables: Lean shapes, checker reads and pins;
Rust mirror`). Measured on that tree.

### PROVE

- **The tables have Lean shapes, and the shapes carry laws.**
  `lean/Semantics/Rules.lean` declares `SbaRule`, `ConferralRule`,
  `DamageResultRule`, `PredefinedToken` and `RulesTables`;
  `lean/Semantics/Check/Rules.lean` gives each a `check` returning the exact
  refusal list, and `lean/Semantics/Proofs/Rules.lean` pins 25 theorems by
  `decide` — 16 positive, 9 negative. `lean/scripts/build` ends in success with
  no warnings (80 jobs).
- **No twins.** A conferral confers an ordinary `Ability` and reuses
  `Ability.grantable`, the read `CopyExcept.ability` already uses, so a spell
  ability is refused for a conferral for the same reason it is refused for a
  grant. A catalog entry is an ordinary `CharacteristicBundle` checked *through*
  `TokenSpec.check [] (.written …)` — the literal token laws, not a copy of
  them. A damage result's counter goes through `CounterKind.check` and
  `counterKindNamed`, the existing registry reads. `SbaRule`'s untargetedness is
  the existing `.nontarget` refusal over `anyTargetedAt`. Two refusals were
  added and one law is new: `.tokenNamed` (an unnamed catalog entry), and
  the shared scope bound `ruleScopeBound` reusing `.kindLte`.
- **Every law reads a declared feature.** The scope law reads `Predicate.kind?`;
  the damage-recipient law reads `damageableKind` over `Predicate.seedTys`; the
  counter law reads the generated counter facts' declared `holder`. No law names
  a lexeme; the only strings in `Proofs/Rules.lean` are registry keys
  (`"Loyalty"`, `"Defense"`) and printed subtype/keyword labels inside bench
  rows.
- **The mirror holds.** `crates/deckmaste_semantics_v2/src/rules.rs` mirrors
  `Rules.lean` declaration-for-declaration; `tests/lean_drift.rs` gained the
  seventh pair and passes, as does its round-trip over every Lean name (`then_`
  required adding `then` to `LEAN_ESCAPED`).
- **A real defect the new law caught.** `plugins_v2/testing/rules/{grant,damage}/
  planeswalker-loyalty.ron` wrote the counter as `Named(label: "loyalty")`. The
  counter registry declares `"Loyalty"`, so both rows would have refused
  `.knownCounter`. Both fixtures and the reader test are corrected, and
  `badDamageResultUndeclaredCounter` pins the lowercase spelling as a refusal so
  it cannot come back silently.

### Assurance counts

Restored 0, re-spelled 0, ignored 0, added 25 pins + 1 reader assertion
(`damage.remove`) + 1 renamed reader test
(`a_token_reads_as_the_bundle_an_effect_writes` →
`a_token_reads_as_a_named_catalog_entry`, same card, same asserted values, new
catalog shape), removed 0.

### DISCLOSE

**The two narrowings the crate ticket left, resolved.** Both are *confirmed*,
not widened, and `rules.rs`'s module doc records why:

- `ConferralRule::confer : Ability` (v1 wrote `Property`). Every rules-defined
  conferral gives an ability — [CR#306.5b], [CR#305.6] — and `Ability` already
  spans the static, activated, triggered and keyword forms `Property` split
  apart. A `Property` type would be a twin of `Ability::Static`.
- `DamageResultRule::remove : CounterKind` (v1 wrote `CounterRef`).
  `CounterKind::Named` *is* the registry key: the Lean checker reads its label
  against the generated counter facts and requires the declared holder to be the
  kind the recipient binds. No second lookup type is needed.

**Rows proved, by table.** SBA: toughness-zero [CR#704.5f], loyalty-zero
[CR#704.5i], non-Siege battle defense-zero [CR#704.5w,310.8]. Conferral:
planeswalker loyalty [CR#306.5b]. Damage result: planeswalker loyalty
[CR#120.3c], battle defense [CR#120.3h]. Catalog: all seven builtin predefined
tokens [CR#111.10a,111.10b,111.10c,111.10f,111.10g,111.10h,111.10w].

**Deviations and additions.**

1. *The battle SBA reads counters, not a `Defense` stat, and states the Siege
   exclusion.* v1's `battle-defense-zero.ron` wrote `StatOf(This, Defense)` with
   a "SEAM: subtype-aware suppression … deferred to engine-battles" note. A
   battle's defense on the battlefield is its defense-counter count [CR#310.4c],
   exactly as loyalty is [CR#306.5c] — v1 already made that correction for
   loyalty and not for defense. The Lean row reads the counters and scopes
   `.and [battle, .not (hasSubtype (.of .battle "Siege"))]`, so the seam is
   closed for [CR#310.8]; [CR#310.7]'s pending-trigger clause remains engine
   work, unchanged.
2. *`Plugin.tokens` changed from `BTreeMap<String, CharacteristicBundle>` to
   `Vec<PredefinedToken>`.* The catalog is now a mirrored type, and two
   representations of it in one crate would drift. File-stem order is preserved
   (`ron_files_recursive` sorts). One consumer existed (the reader test).
3. *Glossary amendments* (`docs/contexts/game-model/CONTEXT.md`): **Predefined
   Token** [CR#111.10], **Damage Result** [CR#120.3,120.3f], and the project term
   **Rules Table** with an `_Avoid_` line separating it from Registry. Three
   terms the landing needed that the glossary did not define.
4. *Docs*: `lean/README.md` gains a "Rules tables" section and now says seven
   syntax layers; `lean/CONTRACTS.md` gains a "Rules tables" section naming the
   scope-supplies-`this` contract, the two representation boundaries, and the
   gaps below.

**STOPs.**

1. **The `then` effect's choice-freedom is not checked.**
   `state-based-actions-are-data.md` says choice-requiring state-based actions
   stay imperative, and I drafted `.sbaAutomatic` over
   `Instruction.introducedChoices`. That read returns `[]` for
   `choose … (a creature)` — `choiceSortAt` has no sort at kind `.object` — so
   the law would have passed every object choice while reading as complete.
   v2 has no complete recursive choice read (`Instruction.namesThisDoor`, the
   nearest idiom, is itself shallow), and shipping a shallow one is counterfeit
   assurance. Resolution: the refusal and the law were **removed**, and
   `Rules.lean`'s docstring states the convention explicitly as unchecked.
   Routed nowhere new — it needs a complete `Instruction` traversal, which is
   its own piece of work; noted here and in `CONTRACTS.md`.
2. **[CR#704.5g] and [CR#704.5h] have no row.** `ProjAxis` projects no marked
   damage and `Lookback` has no "since state-based actions were last checked"
   window, so `plugins/builtin/rules/sba/lethal-damage.ron` is the one v1 row
   `builtinTables` cannot carry. Routed to the new
   `docs/tickets/planned/lean-marked-damage-projection.md`.
3. **`Stat` has no `defense` axis**, so a battle's *printed* defense is
   unreadable and the battle's intrinsic conferral [CR#310.4b] — the exact
   analogue of the planeswalker row this ticket proves — has no row. The
   battlefield value is the counter count [CR#310.4c], so the SBA and damage
   rows are unaffected. Routed to the new
   `docs/tickets/planned/lean-defense-projection.md`.

No contradiction with a recorded ruling arose.

### REPORT

- Gate (`cargo xtask gate --changed`):
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`, run with `--run`, all
  green.
- `cargo test -p deckmaste_semantics_v2`: 26 + 4 (drift) + 6 (reader) passed, 0
  failed.
- `cargo xtask facts check`: `Facts.lean` and `FactsGen.idr` up to date.
- `cargo xtask lean-check`: `plugins_v2/canon` 23/23, `plugins_v2/testing` 2/2,
  25 cards, 4.6 s.
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15,269 citations, 0 stale. `cite bless` newly registered
  [CR#113.1a,113.1b,310.4b,310.4c]; each read against its citing claim, as was
  every site in `jj diff --git | cargo xtask cite audit --diff` (66 sites). Two
  cites were corrected by that read: [CR#111.4] (which is about a creating
  effect setting name and subtypes) replaced by [CR#111.1,110.4] for "a token
  has a card type", and the `SbaRule` docstring's implied choice law reworded.
- `cargo fmt --all` clean; `cargo clippy -p deckmaste_semantics_v2
  --all-targets` clean.
- **Performance advisory**: `lean/scripts/build` 55.4 s and 68.0 s on two runs
  (24 cores, load average 19.7 and 16.2 — a busy host with sibling workspaces
  building). Pin suite `SemanticsProofs` builds in 62 jobs; `Semantics.Proofs.Rules`
  itself elaborates in 8.7 s, the largest single pin module after
  `Proofs.Zone`. No ceiling applies to the Lean build; stated for provenance.
