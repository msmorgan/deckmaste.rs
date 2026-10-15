---
needs: [english-v4-corpus-stratification-and-judgment-harness]
---
# Stratum 0: Card frame, type lines, keyword-ability lines, and ability headers

First, bind the grammar to the full lexical environment
(`deckmaste_lexical_source_v4::LexicalEnvironment::load()`): until it binds,
the census reports every unit unread with reason `Unbound`. The first binding
error is that the frameless auxiliaries (perfect *have*, *'ve*, *haven't*) are
not named by the grammar; the catalogs' frames also need slot Categories the
seed grammar lacks (Amount, Cost, finite and verbless Clause, `ToInf`,
`BareInf`, Gerund, QuotedAbility). Declare what Stratum 0 needs; a slot Category
for a later Stratum may be declared without Productions until that Stratum.

Implement **Stratum 0** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§3, §6.1):

- **Type lines (`[CR#205.2a,205.3,205.4a]`):** `Supertype* CardType+ (— Subtype+)?`
  sourced exclusively from `types.ron` and `subtypes.ron`.
- **Keyword-ability lines (`[CR#702.1]`):** Standalone and comma/semicolon-listed
  keyword abilities sourced from `keyword_abilities.ron`, parameterized by their
  catalog-declared frames (cost, amount, quality, dash cost, and bound stem
  classes such as landwalk and typecycling). The separator between keywords is
  the one place Oracle does not determine its punctuation (commas and
  semicolons both occur), so a Reading keeps it; everywhere else punctuation
  follows from the construction and is never stored.
- **Document and Ability Spines:**
  - Labelled abilities: `<AbilityWord | FlavorWord> — <Ability>` (`[CR#207.2c]`).
  - Modal headers and bulleted mode lists (`[CR#700.2]`), including costed modes.
  - Level headers (`Level N–M`, `Level N+`), Saga chapter headers (`I, II —`),
    and Class level headers.
  - Activated ability cost spines: `<Cost> : <Effect>` (`[CR#602.1]`), including
    mana/tap/untap/energy symbol costs and loyalty brackets (`[+N]`, `[−N]`,
    `[0]`).

The pinned Vintage corpus spells negative loyalty with Unicode `−` (520 lines,
none with ASCII `-`) and omits display brackets. Accept its `+N:`, `−N:`, and
`0:` forms; the bracketed examples above denote the printed symbols.

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

Census delta (`vktuxpvo`): faces read 0 → 2,363; total Readings 0 → 52,591; lost faces: none. Stratum 0: 47,674/47,674 read, zero known-wrong-only, unread, or law failures.
Judgment set (`vktuxpvo`): 195 enforced pass, 2,695 higher-stratum entries pending; seed and Stratum 0 spine suites pass; 56 Stratum 0 judgments pass.
