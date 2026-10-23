---
needs: [lean-v3-keyword-facts-deleted]
---
# Delete `keywordActionLabels` and `actFacts`: keyword actions are typed declarations

Standard constraints apply. Lean first (`lean/Semantics/`). Part 4 of 5 of ruling #1 of the owner's
batch 4 (2026-10-22). Supersedes step 2 of lean-v3-facts-from-declarations (WIP, owned elsewhere; that ticket's step 1 still lands first).

## Ruling (owner, 2026-10-22, batch 4): the Facts tables are deleted

This replaces step 2 of `lean-v3-facts-from-declarations`' A/B; the owner chose neither.

> Make CardType and all catalog items self-contained `semantic_definition`s whose values carry
> their own fields and whose raw constructors are marked `[semantic_expression, internal_expansion]`.
> `Authoring.inspect` already rejects raw `[internal_expansion]` constructors in cards, so every
> `known*` table check is redundant with the authoring boundary.

The whole ruling is split across five tickets that land one at a time, in this order:
`lean-v3-template-helpers-into-definitions` (B1), `lean-v3-catalog-items-self-contained` (B2),
`lean-v3-keyword-facts-deleted` (B3), `lean-v3-action-facts-deleted` (B4),
`lean-v3-card-type-composite-definition` (B5, which also deletes `Check/Facts.lean` and
`Check/FactTypes.lean`).

## The clause this ticket carries

> - **keywordActionLabels and actFacts:**
>   - `participleOf` and `actDestOf` have zero callers; delete them.
>   - `deedFeatureOf`/`actLociOf` only served search/shuffle, which have dedicated constructors;
>     delete them.
>   - `actPermanentActorOf` (adapt, connive, endure, explore, fight, harness, monstrosity) becomes
>     a field on `Deed.action` or is derived from the expansion body.

## Authoring boundary (VERIFY; full account in `lean-v3-catalog-items-self-contained`)

`Authoring.inspect` (`Authoring.lean:345,427`) returns `.raw` for a constructor of any
`semantic_expression` type that is not `semantic_literal`, and `spelled` (`Check/Card.lean:411-431`)
refuses a card holding one; `[internal_expansion]` only suppresses the generated primitive template
(`Authoring.lean:246-253`). `KeywordLabel`, `AbilityWordLabel` and `Deed` are not
`semantic_expression` today (`Words.lean:637` tags only `Delta` and `Subtype`), so this ticket tags
them. **STOP-1 (narrowed, awaiting the owner)**: terms outside `spelled` (about 750 fragment
definitions in `Cards/` and all of `Proofs/`) are not covered by the boundary; proposed: they are
fixtures and keep no `known*` check. Every step except the final deletion of the `known*` checks
proceeds either way.

## Verified (2026-10-22, paths under `lean/Semantics/`)

- **Row counts**: `keywordActionLabels` lists 65 labels (`Check/Facts.lean:6-71`); `actFacts` has 65
  rows (`Check/Words.lean:142-210`); `abilityDeedFacts` has 6 (crew, saddle, phasing, cycling,
  mutate, devour; all `{}`, `Check/Words.lean:133-140`), which the ruling does not mention. The
  only reader of `keywordActionLabels` is `Proofs/Tables.lean:27-34` (`everyDeclaredActionHasFacts`,
  `everyActRowIsDeclared`).
- **`participleOf` (`Check/Words.lean:227`): no production caller** (verified). One pin calls it:
  `copyParticipleUnwritten` (`Proofs/Anaphora.lean:443`). The `participle` column (8 rows:
  destroy, discard, exile, mill, regenerate, sacrifice, tap, untap) is dead with it.
- **`actDestOf` (`:228`): zero callers anywhere** (verified); the `dest` column is dead.
- **`deedFeatureOf` (`:226`) and `actLociOf` (`:229`) are live, and the "dedicated constructors"
  do not replace what they read.** Callers: `deedFeatureOf` at `Check/Words.lean:1191`
  (`Referent.stampedBy`) and `:1202` (`survivesShuffle`), reading `feature = librarySearch`,
  which only the `search` row sets (`:196-197`; `DeedFeature` has the one member); `actLociOf`
  at `Check/Phrase.lean:1292` (`lookbackLocus`, for `.enact verb _ _ (some zone)`), whose only
  non-empty rows are `search` (seven zones) and `shuffle` (`[.library]`). The ruling is right that
  only search and shuffle are involved, and `Action.search`/`Action.shuffle` exist
  (`Abilities.lean:1187,1189`). But the readers key on the **deed label** (`Stamp.verb : Option
  Deed`, `Check/Words.lean:246-252`, and the enact's `verb`), not on the body constructor. So
  deleting the columns means moving those three behaviors: `lookbackLocus` becomes a law on the
  `.search`/`.shuffle` body, and the stamp (or `Deed.action`) must record "library search". This
  is a correction to the plan, not a contradiction of the ruling; the implementer chooses between
  a stamp flag and a field on `Deed.action` (the same mechanism as `permanentActor`, below), and
  says which.
- **`actPermanentActorOf` (`:230`) covers exactly the seven actions named** (verified: `adapt`
  `:144`, `connive` `:156`, `endure` `:166`, `explore` `:170`, `fight` `:173`, `harness` `:176`,
  `monstrosity` `:185`). Callers: `Check/Phrase.lean:1202` (`causedPerformer`) and `:3201`
  (`defaultsToSource`).
- **`knownAct` is redundant only after `Deed.action` is typed.** `KeywordActionLabel` is
  `abbrev … := String` (`Words.lean:278`) and `Deed.action (label : KeywordActionLabel)`
  (`Words.lean:301`) is not `semantic_expression`, so any string is a valid action in a card today
  and `knownAct` (`Check/Words.lean:225`, raised at `Check/Phrase.lean:3208` as `.knownAct`,
  `Check/Refusal.lean:132`) is load-bearing. The checker cannot import the declarations
  (`Definitions/Actions.lean:1-2` imports `Check.Phrase`).

## Change

1. Make `Deed.action` carry the facts the checker reads: a typed action value in
   `Definitions/Actions.lean` whose `Deed` carries `permanentActor : Bool := false` (and, per the
   choice above, `librarySearch`). Tag `Deed.action` `[semantic_expression, internal_expansion]`
   so a raw `.action "descry"` in a card is refused. Keep `Deed.core` and `Deed.ofAbility`.
2. Delete `participleOf`, `actDestOf`, the `participle` and `dest` columns, then (after moving the
   three behaviors) `deedFeatureOf`, `actLociOf`, `DeedFeature`, `loci`, `feature`.
3. `actPermanentActorOf` reads the field (or the expansion body, if the implementer finds it
   derivable (the field's doc comment, `Check/Words.lean:~128`, cites explore [CR#701.44a] and
   endure [CR#701.63a], where a permanent is instructed to perform the action; read the other five
   rules before relying on that)).
4. Delete `keywordActionLabels`, `actFacts`, `abilityDeedFacts`, `deedFacts`, `ActFacts`,
   `knownAct`, `distinctActLabels`, `distinctAbilityDeedLabels` and the `.knownAct` refusal once
   nothing raises it (after STOP-1). `abilityDeedFacts` is the one place the ruling is silent:
   `Deed.ofAbility k` is a keyword ability (typed by B3), so its `knownAct` falls with it.

## Tests

- **Retired**: `Proofs/Tables.lean` `actLabelsDistinct`, `everyDeclaredActionHasFacts`,
  `everyActRowIsDeclared`, `abilityDeedLabelsDistinct` (one declaration per action: a duplicate is
  a Lean error; the cross-check compared two lists that no longer exist);
  `copyParticipleUnwritten` (`Proofs/Anaphora.lean:443`: it pinned `= none` for a column deleted
  as dead); `badUnknownKeywordAction` (`Proofs/Zone.lean:112`: replaced by the authoring pin).
- **Added** in `Proofs/Authoring.lean`: a raw `Deed.action "descry"` in a `spelled` card is refused.
- **Added**: the seven `permanentActor` actions, by name, read `true`, and a sample of others read
  `false`; the `lookbackLocus` and `survivesShuffle` behaviors keep their existing pins (find by
  grep `lookbackLocus`, `librarySearch` in `Proofs/`), re-pointed at the new mechanism.
- `lean/scripts/build`; `cargo xtask lean check` only if plugin definitions change.

## Other tickets that touch the same files

- `lean-v3-keyword-bodies-and-catalog-sync` (planned): adds `Empower Jace` to `keywordActionLabels`;
  moot after this lands, whichever lands second reconciles.
- `research-deed-subjects` (planned, port-required): surveys every `Words.actFacts` row; its
  findings land against whatever replaces the rows.
- `lean-v3-facts-from-declarations` (WIP): Step 1 rewrites the `Proofs/Tables.lean` header.
