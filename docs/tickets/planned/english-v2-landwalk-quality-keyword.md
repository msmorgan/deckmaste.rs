---
needs: []
---
**Landwalk as one keyword ability with a land-type quality.** The canonical
keyword-ability catalog is the Comprehensive Rules section-702 headings and
carries `Landwalk` alone; fused surfaces such as *islandwalk* are legacy atomic
variants that the catalog deliberately excludes
(`crates/deckmaste_catalogs/src/lib.rs::canonical_keyword_abilities_exclude_all_legacy_atomic_variants`
pins `"Islandwalk"` out of it). So english-v2 must reach the printed walk
surfaces through the `Landwalk` declaration plus a quality, never through
per-surface `KeywordAbility` stubs.

Minted by the `english-v2-lexical-inventory-2026-09-05` review under the
coordinator ruling of 2026-09-05 (Q1 = keep the exclusion, drop the row).

`[CR#702.14a]`: landwalk "appears within an object's rules text as
'[type]walk,' where [type] is usually a land type, but it can also be the card
type land plus any combination of land types, card types, and/or supertypes".
`[CR#702.14c]` gives the separated readings by example — "artifact landwalk",
"nonbasic landwalk", "snow swampwalk". (`[CR#702.14b]` is only "Landwalk is an
evasion ability" and does not bear on the surface, so the separated forms cite
`[CR#702.14a,702.14c]`, correcting the sub-rule letters named at mint time.)

What to build:

- A keyword-ability construction reading the `Landwalk` declaration with a
  land-type quality, realizing **both** shapes: the fused surface `<Type>walk`
  (*islandwalk*, *desertwalk*) and the separated `<Quality> landwalk`
  (*Nonbasic landwalk*, *Legendary landwalk*).
- **The quality is read from the type declarations.** A hard-coded list of the
  five basic land words is ruled against; so is a `require` or `checked by`
  naming any walk surface, land type or card. The basic-land subset is not the
  printed set, which is why the row that motivated this ticket was
  misclassified as a data line.
- The realization must round-trip byte-exact in both shapes, including the
  capitalized line-item form.

Expected units (counted with `jq` over `data/mtgjson/AtomicCards.json` at mint
time): the **146** identities the retired lexical row was covering, plus
`Desertwalk` 2, `Nonbasic landwalk` 3, `Legendary landwalk` 2. Out of scope
unless they fall out for free: the joke/silver printings `townwalk` 1,
`Planeswalkerwalk` 1, `Denimwalk` 1.

Open question, deliberately not resolved here: `macro-keyword-templates` says
"typed landwalk stays bare keyword names in the bespoke parser". Read in
context (`docs/keyword-policy.md` §4, where Landwalk carries
`quality: Default(Predicate, Type(Land))`) that sentence governs the **v1**
macro-template layer and its slot-reader codec, not the v2 grammar — the v1
shape is in fact the shape this ticket asks for. If the claimant reads it as a
constraint on v2, STOP and say so rather than resolving it.

Tier: **sol**. Standard constraints apply.
