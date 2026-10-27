---
needs: [plugins-v3-builtin-keywords]
---
# Migrate `Definitions/Actions.lean`, leaf definitions, and rules into `plugins_v3/builtin/`

**Extract `Actions.lean`, `Counters.lean`, `Subtypes.lean`, `SubtypeRules.lean`,
`Designations.lean`, `DesignationRules.lean`, `Relations.lean`, `CardTypes.lean`,
`CardTypeRules.lean`, `Phrasings.lean`, and the `RegistryDefinition` conferral rows in
`Proofs/Rules.lean` into individual `.lean` files under `plugins_v3/builtin/`.** Third of four
atomic tickets migrating Lean declarations into `plugins_v3/builtin/`
([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5). Standard
constraints apply.

## Scope

1. **Extract CR §701 keyword actions (`Definitions/Actions.lean`, ~827 lines):**
   - Move each keyword action `Definition` into
     `plugins_v3/builtin/definitions/keyword_action/<name>.lean` using the `action` command macro.
   - Re-spell `earthbend` (`[CR#701.66a]`) to reference `haste` directly and `timeTravel`
     (`[CR#701.56a]`) to reference `suspend` directly instead of `.named` string literals.
   - Omit manual `.ownScope`, `inCaller`, parameter `firstMention`/`laterMention`, and `theActor`
     `firstMention`/`laterMention` in `fight`, `explore`, `connive`, and `endure`.
   - Move the templates in `Definitions/Actions.lean` (`convert`, `destroyCantRegenerate`,
     `exileUntilLeaves`, `mayCastExiled`, `controllerSacrifices`, `doesntUntap`,
     `mayDeclineUntap`, `putIntoLibraryNthFromTop`, `takeExtraTurnAfterThisOne`, etc.) and
     `Definitions/Phrasings.lean` (~153 lines) into `plugins_v3/builtin/templates/<cat>/<name>.lean`.

2. **Unify and extract counters, subtypes, designations, relations, card types, and rules:**
   - Combine each counter in `Definitions/Counters.lean` with its conferrals from `Proofs/Rules.lean`
     into `plugins_v3/builtin/definitions/counter/<name>.lean` (`counter` command macro).
   - Combine each subtype in `Definitions/Subtypes.lean` with its rules from
     `Definitions/SubtypeRules.lean` into
     `plugins_v3/builtin/definitions/subtype/{creature,artifact,enchantment,land,planeswalker,battle,spell}/<name>.lean`
     (`subtype` command macro).
   - Combine each designation in `Definitions/Designations.lean` with its rules from
     `Definitions/DesignationRules.lean` into `plugins_v3/builtin/definitions/designation/<name>.lean`
     (`designation` command macro).
   - Move `Definitions/Relations.lean` into `plugins_v3/builtin/definitions/relation/<name>.lean`
     (`relation` command macro).
   - Combine `Definitions/CardTypes.lean` and `Definitions/CardTypeRules.lean` into
     `plugins_v3/builtin/definitions/card_type/<name>.lean` (`card_type` command macro; keeping any
     checker-level `CardType` enum in `Words.lean`).
   - Move standalone state-based action and conferral rows into `plugins_v3/builtin/rules/{sba,conferral}/<name>.lean`.

3. **Delete migrated `lean/Semantics/Definitions/*.lean` submodules:**
   - Remove the migrated `Definitions/*.lean` files and their imports from `lean/Semantics/Definitions.lean`.

## Proof

- `Definitions/Actions.lean`, `Counters.lean`, `Subtypes.lean`, `SubtypeRules.lean`,
  `Designations.lean`, `DesignationRules.lean`, `Relations.lean`, `CardTypeRules.lean`, and
  `Phrasings.lean` are deleted and replaced by individual `.lean` files in `plugins_v3/builtin/`.
- `earthbend` references `haste` directly and `timeTravel` references `suspend` directly with zero
  `.named` workarounds.
- `cargo xtask lean check` (`lake build --wfail`) exits 0 with zero errors and zero warnings.
