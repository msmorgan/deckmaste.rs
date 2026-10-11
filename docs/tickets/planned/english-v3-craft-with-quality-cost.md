---
needs: []
---
# [design] Read Craft with Quality and Cost after the owner decides its semantic signature

Standard constraints apply. Split from `english-v3-quality-cost-keyword-lines`
by orchestrator ruling QC9 (2026-10-11). This ticket is design-gated by the
OWNER DECISION below; do not implement before that decision is recorded.

## Action and settled decisions

Craft's existing declaration has `params: [Cost]`. Reading its printed
materials and Cost through `QualityCostKeyword` requires all three changes:

- `plugins_v2/builtin/macros/keyword_abilities/craft.ron`: `params: [Quality, Cost]`;
- `FixedKeyword` parameter `Quality(preposition: ("Preposition", "With"))`;
- `(With) => Yes` in `keyword_quality_marker`, and marker concord for the
  marked QualityCost form (`With, With`).

The unmarked QualityCost form and separate grammar-only EquipQuality recipe
land in the source ticket. No new Construction is needed here. Preserve the
same named Head, Quality and Cost fields. Every Quality admitted by the existing
Quality Constructions is admissible (QC4); no witness or lexeme list licences
it. One route for the construction preserves independent ambiguity (W9-A).
QC7's two fused materials need `english-v3-fused-cardinal-head`; that dependency
is a design fact, not permission to include an unresolved analysis.

QC9 reverses QC2's Craft signature instruction and defers QC6's retirement of
bare-Cost Craft. Until the owner decision, all 24 Craft-with faces remain unread
and the existing signature and generated facts remain unchanged. Engine bodies
remain outside this grammar ticket.

## Probe evidence

Measured on claim-parent runtime `wlslkrom`, feature change `vwntqpupwqyw`,
2026-10-11, with `cargo xtask english-v3 probe --text TEXT --category document
--readings 64 --json`, before implementation:

| Witness/control | Claim parent | Withdrawn Quality+Cost implementation | QC9 landing |
|---|---:|---:|---:|
| Braided Net: `Craft with artifact {1}{U}` | 0 | 1 | 0 |
| The Enigma Jewel: `Craft with four or more nonlands with activated abilities {8}{U}` | 0 | 1 | 0 |
| `Craft {1}{U}` | 1 | 0 | 1 |
| `Splice onto Arcane {1}{R}` | 1 | 1 | 1 |
| `Affinity for artifacts` | 2 | 2 | 2 |
| `Protection from legendary creatures` | 2 | 2 | 2 |

Supported-corpus attestation: `cargo xtask english-v3 grep '(?m)^Craft with'`
returns 24 faces and 24 matches. Bare Cost-only Craft is unattested:
`cargo xtask english-v3 grep '(?m)^Craft \{'` returns 0 faces. The withdrawn
bounded implementation covered 22 Craft faces; QC7's two fused-material faces
stayed unread. The final census after QC9 must retain all 24 as unread.

## The 24 supported faces

- Eye of Ojer Taq (`0b1557e8-3dd6-4276-8b8e-505e7ea944ec#face:0`)
- Kaslem's Stonetree (`1ac3e4bc-1678-4280-a071-3dbc8ef4a2bf#face:0`)
- Oteclan Landmark (`2420828e-b886-42d3-81ba-8ee81b6e1eb8#face:0`)
- Master's Guide-Mural (`2dfdd358-e3cd-4bf5-871b-37689b7951e9#face:0`)
- Waterlogged Hulk (`30820f71-9fd4-453c-b377-c2e0b01c07a7#face:0`)
- Inverted Iceberg (`3feedc63-173e-4713-8e96-8f1c9576055f#face:0`)
- Dire Flail (`5a043512-fa56-4361-bb92-0a0f229f34d3#face:0`)
- Visage of Dread (`7cad43df-31c9-47b5-bc05-4a0f6544b396#face:0`)
- Braided Net (`86ef06b3-2049-494d-9ecb-ca14766d3b68#face:0`)
- Tithing Blade (`883a4180-9ede-4249-a4b8-3a29c998fb63#face:0`)
- Clay-Fired Bricks (`90e1bcef-53af-4c08-b713-bf10dfcbd577#face:0`)
- Paleontologist's Pick-Axe (`9b0c6b77-69df-4330-86a7-8dc4666abd38#face:0`)
- Throne of the Grim Captain (`a7c96442-706b-4fd8-9b38-052872dafe58#face:0`)
- The Enigma Jewel (`c0d2bdd9-1e68-4d3a-83f8-ec2ba440a494#face:0`)
- Altar of the Wretched (`c551f007-9740-4a2a-8ef5-7be1a7afe14b#face:0`)
- Sunbird Standard (`c69742bf-f0f5-4216-a602-c21f9c5a6501#face:0`)
- Tetzin, Gnome Champion (`e12b38e5-621b-4b2c-8c04-c15c41d4fc70#face:0`)
- Ore-Rich Stalactite (`e17d8c0e-16e7-477b-830d-0818cedcb354#face:0`)
- Idol of the Deep King (`e7ba349e-b9c1-4373-b79f-755688138b7c#face:0`)
- Saheeli's Lattice (`ef8df503-6160-4e40-a689-49e97fd56cd0#face:0`)
- Lodestone Needle (`f429ca15-c578-4c1d-a604-fd5509c92dba#face:0`)
- Spring-Loaded Sawblades (`fa1ed9fa-5f77-4f7d-bab0-47c719cdc5bc#face:0`)
- Jade Seedstones (`fa2c984a-6bc1-4ec8-93d7-87f9f06b841e#face:0`)
- Unstable Glyphbridge (`fcd54631-ea47-49a7-ad5f-9a9a51a815ba#face:0`)

## OWNER DECISION

May a grammar ticket change an existing keyword's declared semantic signature
and regenerate its generated v1 facts row, or does the semantics-v2 line make
that change first? Authority: `docs/decisions/semantics-v2.md` §§10–16 and
AGENTS.md's "Never copy v2 work back into v1, and add no new v1 registries."
QC9 says the orchestrator cannot authorise this exception; the owner decides
with the semantics-v2 line. Record that decision and its ordering here before
changing the signature or generated facts.

`cargo xtask facts generate` writes both Lean and legacy Idris facts;
`cargo xtask facts check` rejects drift in either. The full xtask suite failed
`facts::tests::the_committed_module_matches_the_stubs` while the proposed Craft
signature was present. No generated facts source was changed in the Equip
landing. The exact proposed generated changes are preserved below because
`target/` evidence is untracked.

### Required Lean facts diff

```diff
--- lean/Semantics/Check/Facts.lean	2026-10-11 01:13:06.381459443 -0400
+++ target/english-v3/english-v3-quality-cost-keyword-lines/facts-preview/lean/Semantics/Check/Facts.lean	2026-10-11 08:43:28.392758363 -0400
@@ -127,7 +127,7 @@
     { word := .named "partnerWith", argumentSchemas := [[⟨.quality, none⟩]] },
     { word := .named "emerge", argumentSchemas := [[⟨.cost, none⟩]], functionsOnStack := true, onInstantOrSorceryCard := true, paidCost := true, definition := [⟨.static, none⟩, ⟨.static, some .atCasting⟩], regime := some .atCasting },
     { word := .named "emergeFrom", argumentSchemas := [[⟨.quality, some .object⟩, ⟨.cost, none⟩], [⟨.cost, none⟩]], functionsOnStack := true, onInstantOrSorceryCard := true, paidCost := true, definition := [⟨.static, none⟩, ⟨.static, some .atCasting⟩], regime := some .atCasting },
-    { word := .named "craft", argumentSchemas := [[⟨.cost, none⟩]], paidCost := true },
+    { word := .named "craft", argumentSchemas := [[⟨.quality, some .object⟩, ⟨.cost, none⟩], [⟨.cost, none⟩]], paidCost := true },
     { word := .named "madness", argumentSchemas := [[⟨.cost, none⟩]], onInstantOrSorceryCard := true, paidCost := true, definition := [⟨.static, none⟩, ⟨.triggered, none⟩] },
     { word := .named "prowl", argumentSchemas := [[⟨.cost, none⟩]], functionsOnStack := true, onInstantOrSorceryCard := true, paidCost := true, definition := [⟨.static, some .atCasting⟩], regime := some .atCasting },
     { word := .named "surge", argumentSchemas := [[⟨.cost, none⟩]], functionsOnStack := true, onInstantOrSorceryCard := true, paidCost := true, definition := [⟨.static, some .atCasting⟩], regime := some .atCasting },
```

### Required legacy Idris facts diff

```diff
--- idris/src/Experimental/FactsGen.idr	2026-10-10 20:22:22.565083101 -0400
+++ target/english-v3/english-v3-quality-cost-keyword-lines/facts-preview/idris/src/Experimental/FactsGen.idr	2026-10-11 08:43:28.393758390 -0400
@@ -64,7 +64,7 @@
   , { word := "PartnerWith", paramShapes := [QualityParam] } defaultKeywordFacts
   , { word := "Emerge", paramShapes := [CostParam], regime := Just AtCasting, functionsOnStack := True, onSpellCard := True, paidCost := True } defaultKeywordFacts
   , { word := "EmergeFrom", paramShapes := [CompoundParam QualityHead], regime := Just AtCasting, functionsOnStack := True, onSpellCard := True, paidCost := True } defaultKeywordFacts
-  , { word := "Craft", paramShapes := [CostParam], paidCost := True } defaultKeywordFacts
+  , { word := "Craft", paramShapes := [CompoundParam QualityHead], paidCost := True } defaultKeywordFacts
   , { word := "Madness", paramShapes := [CostParam], onSpellCard := True, paidCost := True } defaultKeywordFacts
   , { word := "Prowl", paramShapes := [CostParam], regime := Just AtCasting, functionsOnStack := True, onSpellCard := True, paidCost := True } defaultKeywordFacts
   , { word := "Surge", paramShapes := [CostParam], regime := Just AtCasting, functionsOnStack := True, onSpellCard := True, paidCost := True } defaultKeywordFacts
```

The derived Lean macro would also change from
`craft (arg0 : Semantics.Cost)` to
`craft (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost)`, emitting separate
Quality and Cost KeywordParams. That existing export is restored by QC9.

## Verification after the decision

Probe the witnesses before editing. Select every supported face containing
Craft anywhere, every affected declaration's claim-parent users, and 300 random
covered controls (seed 20261008). Run the bounded census with workers 6 and
samples per face 0 under `/usr/bin/time -v`, then the official `english-v3 diff`
with `--lines`.
