---
needs: []
---
**Split the three roles `CatalogKind::KeywordAbility` atoms currently serve.**

One atom kind stands for three syntactically distinct things:

1. a keyword **declaration** on a keyword line (`Flying`, `Enchant creature`);
2. a keyword **reference** in running text, an ordinary nominal (`gains
   flying`, `has trample`, `if tribute wasn't paid` [CR#702.104b]);
3. accidental **proper-name interior** (`named Storm Crow`, `Partner with Proud
   Mentor`), where the atom is part of a card name and not a keyword at all.

The routing is otherwise disciplined — ordinary noun position uses a closed
eight-kind list that excludes keyword abilities, the adjective slot is
supertypes only, and `Noun::Catalog` is built only in `catalog.rs` — so this is
the one place the kind is overloaded.

## Evidence it is overloaded

- `CatalogSlot::AbilityItem | CatalogSlot::KeywordAbilityNoun` resolve through
  a **single match arm** in `Catalogs::matches`. The enum names the
  declaration/reference distinction and the resolver discards it; they diverge
  only later, where one is wrapped as `NounInstance::Mass(Noun::Catalog(_))`.
- Two repairs already exist downstream for role 3: `detach_keyword_noun`
  (re-labels the atom opaque inside `named …`) and the renderer's
  `subject_is_enchant_keyword` / `predicate_verb_is_enchant` pair, which
  patches the `Enchant tapped creature` misparse — and which branches on
  `atom.canonical() == "Enchant"`, the only surface-spelling render branch in
  the crate and a direct violation of *English productions ship their inverse*.
- `render_noun` needs a per-kind exception (`KeywordAbility => lowercase`)
  because a declaration prints capitalized and a reference does not.

## What the split needs

The `kwgrant` round replaced a keyword-name list with a catalog-membership
property, which is more principled and strictly wider: every keyword atom
became admissible wherever any is. The correct narrowing is a third thing —
**which keywords are referable** (their name is an ordinary nominal) versus
**declaration-only** (enchant [CR#702.5a], champion [CR#702.72a]).

Two rounds have now failed for want of exactly this datum; see
`english-keyword-ability-parameters` for both, including the census evidence
that `tribute` is a legitimate bare keyword-atom subject while `Enchant` is
not. It is the same property that entry needs to gate a bare noun-phrase
keyword argument, so the two should land together or the second should follow
the first.

Deriving it: the CR states a written form for only three keywords, so the rest
must come from the exemplar rules, as the `kwparam` round's shape survey did.
Do not solve it with a keyword-name list.

Standard constraints apply.
