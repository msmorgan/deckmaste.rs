---
needs: [english-v4-corpus-stratification-and-judgment-harness]
---
# Stratum 0: Card frame, type lines, keyword-ability lines, and ability headers

Implement **Stratum 0** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§3, §6.1):

- **Type lines (`[CR#205.2a,205.3,205.4a]`):** `Supertype* CardType+ (— Subtype+)?`
  sourced exclusively from `types.ron` and `subtypes.ron`.
- **Keyword-ability lines (`[CR#702.1]`):** Standalone and comma/semicolon-listed
  keyword abilities sourced from `keyword_abilities.ron`, parameterized by their
  catalog-declared frames (cost, amount, quality, dash cost, and bound stem
  classes such as landwalk and typecycling).
- **Document and Ability Spines:**
  - Labelled abilities: `<AbilityWord | FlavorWord> — <Ability>` (`[CR#207.2c]`).
  - Modal headers and bulleted mode lists (`[CR#700.2]`), including costed modes.
  - Level headers (`Level N–M`, `Level N+`), Saga chapter headers (`I, II —`),
    and Class level headers.
  - Activated ability cost spines: `<Cost> : <Effect>` (`[CR#602.1]`), including
    mana/tap/untap/energy symbol costs and loyalty brackets (`[+N]`, `[-N]`,
    `[0]`).

## Gate

- 100% of Stratum 0 corpus units (under the Max-Stratum Composition Rule:
  non-clausal type lines, keyword-ability lines, and symbol/numeral headers)
  parse as **read** (0 `known-wrong-only`) and satisfy both round-trip laws
  (`v is an admitted Reading of s => render(v) = s` and
  `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Outer ability spines (`Cost : Effect`, `<Label> — <Ability>`, modal bullets)
  are verified on Stratum 0/seed payloads in the Stratum 0 judgment suite.
- Zero CR-cataloged words appear directly in the grammar data file.

Standard constraints apply.
