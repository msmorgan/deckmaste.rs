---
needs: [english-v3-lean-grammar-model]
---
# Make the v3 English model's claims nonvacuous

Audit the rebuilt English workbench before Rust treats it as design evidence.
Every named law must range over structures the modeled grammar can derive, use
the hypothesis it claims to test, and distinguish a real counterexample when
the law is weakened. Replace or retire the old proof-gap inventory according to
the new model rather than narrowing its statements until they become automatic.

The audit must account explicitly for Tense versus Finiteness and Word Form;
flat serial-comma Coordination versus genuine nested Coordination; head-owned
countability in genitives; correlated alternatives that are not a Cartesian
product; extraction and anchor projections; ordinary-word payload boundaries;
duplicate derivations; and previously unwitnessed adjective, mass-noun,
placement, sharing and auxiliary cases. Resolve the three unrelated public
`Reading` concepts and add the missing Oracle English glossary terms used by the
v3 model.

Acceptance gives every inherited proof-gap and residual-audit item a named
disposition: structurally repaired, superseded by the accepted v3 relation, or
transferred to a live implementation ticket with the obligation intact. No
theorem disappears without its replacement claim being named. The project
builds without `sorry`, the axiom audit is unchanged, and each central exclusion
fails under a deliberately weakened premise or has another non-circular
nonvacuity witness. Use Lean LSP MCP and `english/scripts/build`. Standard
constraints apply.

## Landing record

Every inherited proof-gap item, every residual-audit clause and every row of the
independent vacuity scan now carries a named disposition. The workbench builds
with warnings as errors and no `sorry`; the axiom audit is a command
(`english/scripts/axioms`) rather than a narrative. The
[workbench README](../../../english/README.md) records the model's limits, the
two-relation divergence, the host-invariant limitation and the preference-layer
instantiation; the
[declaration disposition](../../../english/DECLARATIONS.md) carries the refreshed
proof accounting and every renamed row.

**Workbench exception.** `docs/decisions/english-lean-design-workbench.md`
applies: this is a Lean grammar-model landing. No Rust, no declaration data and
no `plugins*/` file changed, so there is **no corpus pass, no coverage lock, no
selection census, no licensing-checker total and no per-byte performance
telemetry** — none is measured and none is claimed. The evidence is the named
structural proofs, the declaration dispositions, the LSP checks, the complete
Lake build and the axiom command's output.

**All numbers below are stamped on change `vlxykxku`** (this record's own
commit), with the per-packet figures stamped on the packet change ids named in
the disposition table's Landed column.

### Prove

**No silent loss.** No theorem was deleted. Exactly one theorem *name* stopped
existing across the whole ticket —
`FeatureInteractions.genitive_preserves_countability` — and it is a re-spelling,
not a loss: its replacement `FeatureInteractions.genitive_determiner_is_transparent`
is named in `english/DECLARATIONS.md` and states strictly more (head-owned
countability, plus the two discriminating conjuncts the old shape could not
express). Three non-theorem declarations were retired, each with its replacement
named in the same file: the `Features.DeterminerUse.genitive` wildcard
constructor (→ `Features.Transparent`), the `DocumentProduction.body`
constructor (→ `ParagraphProduction.body`) and `BoundaryInteractions.wordPayload`
(→ `Atom.WellFormed`). Classification: all four are *wrong analysis retired*;
none is a regression and none is re-coverage owed to another ticket.

**Structural laws.**

- `english/scripts/build` (`lake build --wfail`, warnings as errors):
  **Build completed successfully (46 jobs)**, no warnings, on the final tree.
- `english/scripts/axioms` (new, §3 of the plan) exits `0` and prints:

  ```
  English theorems checked: 2681
  disallowed axiom uses: 0
  ```

  It enumerates every compiled constant that `isTheorem` and whose
  `privateToUserName` begins `English.` — private and generated declarations
  included — collects transitive axioms with `Lean.collectAxioms`, and permits
  only `propext`, `Classical.choice` and `Quot.sound`. `sorryAx` is not
  permitted, so a proof hole anywhere in the closure fails it. The axiom audit
  is therefore **unchanged**: the same three axioms, zero disallowed uses, over a
  closure that grew from 2,400 to 2,681 theorems.
- No `sorry` in `english/English/`, `english/English.lean` or
  `english/scripts/`.
- Lean LSP diagnostics (`lean_diagnostic_messages`) were clean for every file
  edited in this packet, and were used throughout P1–P4;
  `lean_minimal_hypotheses` confirmed the eight criterion-(ii) candidates and the
  new hypothesis-bearing laws are load-bearing.

**No word-naming.** Every guard added in this ticket reads a declared feature or
a construction category, never a lexeme, spelling or card identity. The two
places where that was at issue are recorded: ruling **S4**'s repair reads
`NotationKind` (a declared document-notation category), not the numeral string,
and B4's `FrameInteractions.framePolicy` reads `RolePreference.roleCount` and
`RolePreference.identityCount` — both declared-feature counters. Fixture
declarations name words, as the rules permit.

### Disclose

#### Per-item disposition table

Dispositions are `repaired` (structurally repaired), `superseded` (superseded by
the accepted v3 relation) or `transferred` (moved to a live ticket with the
obligation intact). Landed column gives the packet and its change id:
P1 `syvllpvqw`, P2 `twxoykpmw`, P3 `xplmrzqxo`, P4 `lkqzzztmm`, P5 `vlxykxku`.

**Group A — the six numbered items of the retired proof-gap ticket.**

| # | item | disposition | witness / replacement theorems | landed |
|---|---|---|---|---|
| A1 | Tense vs Finiteness vs Word Form | repaired | `GrammarWitnesses.no_tense_on_nonfinite`, `finite_is_not_nonfinite_use`, `plain_form_both_finite_and_nonfinite`, `preterite_is_not_tense` (`finite_tense_is_past` kept and cited) | P1 |
| A2 | flat serial-comma vs nested Coordination | repaired | `Construction.serialCoordinate` / `Production.serialCoordinate` / `Surface.serial` / `Linearizes.serialCoordinate` (both relations); `FamilyWitnesses.serialThree_derives`, `serialThree_realizes`, `serial_admitted`, `nestedThree_derives`, `nestedThree_realizes`, `pairTwo_derives`, `nested_rejects_serial_surface`, `nested_admitted_without_commas` | P2 |
| A3 | head-owned countability in genitives | repaired | `Features.Transparent`; `FeatureInteractions.genitive_determiner_is_transparent` (replaces `genitive_preserves_countability`); `NominalWitnesses.genitive_mass_admitted`, `genitive_count_admitted` | P1 |
| A4 | correlated alternatives, not a Cartesian product | repaired | `Scope.joint_correlated_pack`; `JointInteractions.row_derives`, `row_conforms`, `all_four_admitted_without_dependencies`, `row_safe_iff`, `row_admitted_iff`, `joint_alternatives_grammatical` (`Scope.joint_alternatives_exact` kept, re-proved through the new lemma) | P3 |
| A5 | anchor projection | repaired | `FrameScope.projectAnchors`, `anchorsFrom`/`childAnchorsFrom`, `anchorCount_anchorsFrom`, `anchorCount_childAnchorsFrom`, `projectAnchors_flat_ne_nested`, `FrameScope.anchor_shape_separate`; `FamilyWitnesses.serial_anchor_shapes`, `serial_anchor_shape_separate`, `serial_anchor_count_coarser` (`Scope.anchor_shape_separate` kept, now called with inhabited premises) | P2 |
| A6 | ordinary-word payload boundaries | repaired | `Atom.WellFormed` / `Surface.WellFormed` (side condition on `LexicalAnalysis`, replacing `BoundaryInteractions.wordPayload`); `source_space_is_not_a_word`, `lexical_apostrophe_allowed`, `source_newline_is_not_a_word` re-spelled, `quote_delimiter_is_not_a_word` added; `Payload.both_analysed_without_payload`, `plain_licensed`, `spaced_not_licensed`, `plain_admitted`, `spaced_not_admitted`; `LexicalWitnesses.rows_well_formed` | P4 |

**Group B — the retired ticket's "Residual audit" paragraph, clause by clause.**

| # | clause | disposition | witness / replacement theorems | landed |
|---|---|---|---|---|
| B1 | `Analysis.no_feature_bypass` never uses its packing hypothesis | superseded | `Reading.Valid` / `Reading.Admitted` — no packing layer exists to bypass; the unused hypothesis no longer exists. `english/DECLARATIONS.md:117` already named the replacement. Verified, no action. | P5 (record) |
| B2 | six `Unit`-keyed packing witnesses are inert | repaired (3) / superseded (3) | repaired: `GrammaticalScope.nested_alternatives_packed`, `quoted_alternatives_packed` (key `scopeClass`), `JointInteractions.joint_alternatives_grammatical` (key `Dependencies.exposed`). superseded: `PreferenceWitnesses.modifier_scope_unique`, `modifier_package_retains_both`, `CrossHost.cross_host_scope` — survivors are one scope class, so every projection is constant; doc-noted and `DECLARATIONS.md`-noted. | P3 |
| B3 | no host invariant on `FrameScope.Related`; equal-text ruling only for `GrammaticalScope` | repaired (lexeme, anchor, equal-text) / **limitation recorded** (host) | lexeme `FrameScope.related_preserves_lexemes`; anchor `related_preserves_anchors`; equal-text `AmbiguityWitnesses.unrelated_readings_retained` + `control_unambiguous`. **Host: no such invariant exists and cannot** — `Step.determiner`, `Step.sharedHead` and `Step.boundary` are host-restructuring moves by construction. Landed as a proved finding, `FrameScope.step_can_change_hosts`, rather than prose; the invariant that does hold is `GrammaticalScope.related_preserves_hosts` over `ScopeRelated`. README records it. | P5 |
| B4 | the `Claims`/`Policy` layer is never instantiated on syntax | repaired (see **S3**) | `FrameInteractions.framePolicy` (`Preference.Policy (Syntax Lexeme) (List Lexeme)`, claims read `roleCount`/`identityCount`, region is the ordered lexical identities), `roleBlindPolicy`, `RolePreference.identityCount`; `frame_policy_claims_read_features`, `frame_policy_same_region`, `frame_policy_prefers_marked`, `frame_policy_selects_marked`, `frame_policy_does_not_revoke_admission`, `role_blind_policy_selects_postmodifier` | P5 |
| B5 | `FrameScope` (model) imports `FrameInteractions` (witnesses) | repaired | `English/FrameScopeWitnesses.lean` (new); `FrameScope.lean` now imports `English.RolePreference` only. Twelve declarations re-homed verbatim; no theorem changed. | P4 |
| B6 | `chapter` / `dieDashRow` are byte-identical productions | repaired (see **S4**) | `NotationKind` (`plain`/`chapter`/`dieResult`) on `DocumentCategory.notation`; `Documents.chapter_notation_is_not_a_die_row`, `die_notation_is_not_a_chapter`, `notation_words_do_not_cross`. Both families kept; the `chapter_text` and `die_dash_text` witness surfaces are unchanged byte-for-byte from before the repair. | P4 |
| B7 | `attributive`, `bareMass`, `Placement.before`, `ScopeMove.auxiliary` unwitnessed | `bareMass` superseded (already repaired by the grammar-model landing); other three repaired | `Features.attributive_does_not_hide_target`, `NominalWitnesses.nontoken_licensed`, `attributive_admitted`, `attributive_cannot_hide_target`; `very_licensed`, `adverb_derives`, `adverb_realizes`, `degree_adjunct_admitted`, `degree_adjunct_rejects_after`; `GrammaticalScope.CrossHost.auxiliary_scope_move`, `auxiliary_scope_related`, `auxiliary_move_keeps_hosts` | P4 |
| B8 | `SharingInteractions` states no exclusion | repaired | `Heads.ordinary_coordination_cannot_raise`, `Heads.raised_over_ordinary_coordination_excluded`, `Determiners.determiner_cannot_distribute_across_singular` | P4 |
| B9a | three unrelated public `Reading` concepts | superseded (verified resolved) | exactly one `Reading L` remains; the other two are `GrammaticalScope.SchemaWitness` and `Scope.AnchorPattern`. The two look-alikes — `namespace English.Reading` in `SurfaceRelations.lean` and `Syntax L` at a bare Lexeme type — are documented in `english/README.md` so the distinction need not be rediscovered. | P5 |
| B9b | `Complement` misnames a slot descriptor | repaired (fitted; **no transfer needed**) | `English.Complement` → `English.FrameSlot`, the `FrameItem.argument`/`.marked` field `complement` → `slot`; 62 sites, build green. Glossary entry **Frame Slot** added, `_Avoid_: Complement for the slot descriptor`. `DECLARATIONS.md:678` updated. | P5 |
| B9c | `quantity` / `quantify` naming | superseded (see **S2**) | the inventory's naming claim is withdrawn; the names match the owning glossary entry (**Quantity**, `docs/contexts/oracle-english/CONTEXT.md`). No rename. | P5 |
| B9d | missing Oracle English glossary vocabulary | repaired | 14 entries added — see the glossary subsection below. | P5 |
| B10 | `.document .body` reachable through two judgment constructors | repaired | `ParagraphProduction` (new; `DocumentProduction.body` retired with the replacement named); `EllipsisInteractions.body_has_no_node_production`, `retired_node_route_premises`, `paragraph_is_the_only_route`; `Documents.paragraphUnary`/`paragraphBinary`/`paragraphTernary`/`sentence_derives_in` | P4 |

**Group C — the live ticket's "must account explicitly for" list.**

| # | bullet | disposition | landed |
|---|---|---|---|
| C1 | extraction projections (the half A5 does not cover) | superseded (already repaired) — `Dependencies.exposed` is connected through `FrameDeclarations.retained_gap_keeps_object_relation`, `marked_gap_keeps_complement_relation`, `DependencyInteractions.object_accessible` and `RelativeLicense`'s positive/negative witnesses; A4 deepens it by making the Relation component load-bearing for a joint law | P3 (deepened) |

Every other Group C bullet is a pointer into Groups A and B: Tense/Finiteness/Word
Form → A1; flat vs nested Coordination → A2; head-owned genitive countability →
A3; correlated alternatives → A4; anchor projection → A5; word payload → A6;
duplicate derivations → B6 + B10 + V1; unwitnessed adjective / mass-noun /
placement / sharing / auxiliary → B7 + B8; three `Reading` concepts → B9a;
glossary → B9d.

**Vacuity scan (§2 of the plan) — independent of the ledger.**

| # | declaration | disposition | witness / replacement | landed |
|---|---|---|---|---|
| V1 | `Reading.duplicate_derivations` (`:= rfl`) | repaired | statement kept; `AmbiguityWitnesses.duplicate_derivations_witnessed`, `duplicate_derivations_premise_inhabited`. Structural multiplicity (the two judgment routes) is what B10 removes; proof multiplicity is not observable in `Prop`. | P4 |
| V2 | `FrameScope.flat_nested_differ` (premise no tree satisfies) | repaired | re-spelled over derivable trees, both `Derives` load-bearing; the old arithmetic kept verbatim as private `flat_nested_anchors` | P2 |
| V3 | `FeatureInteractions.genitive_preserves_countability` | repaired | `genitive_determiner_is_transparent` (see A3) | P1 |
| V4 | six `Unit`-keyed packing witnesses | repaired (3) / superseded (3) | see B2 | P3 |
| V5 | `PreferenceWitnesses.acyclic_tie` acyclicity conjunct | repaired | statement unchanged; its emptiness is now *stated* by `PreferenceWitnesses.neutral_prefers_nothing`, and the acyclicity claim is re-spelled over an inhabited preference as `FrameInteractions.role_acyclic_tie` | P3 |
| V6 | `Reading.preference_retains_both` | superseded (justified, not a defect) | it is a projection of `Preference.admitted`, which *is* the design contract; doc note added naming the inhabitant `AmbiguityWitnesses.nounPreference` + `preferred_and_nonpreferred_remain` over "I saw her duck" | P5 |
| V7 | `Reading.AnalysisRoundtrip` never instantiated | repaired | `AmbiguityWitnesses.analysis_roundtrip_has_teeth`, `analysis_roundtrip_rejects_silence`, `analysis_roundtrip_rejects_constant` | P3 |
| V8 | `Reading.ValueRoundtrip` vacuously satisfiable | repaired | `SpellingWitnesses.constructs`; `value_roundtrip_forces_variants`, `value_roundtrip_rejects_variant_blind`, `value_roundtrip_rejects_case_blind` | P3 |
| V9 | two parallel `Linearizes`/`Realizes` relations | superseded (documentation-grade; not vacuous) | no proof change. The divergence — schema copy **rewrites** sentence/cost-action case, v3 copy **requires** it — is now stated in `english/README.md` and in a doc comment on **both** relations, together with the "add every new constructor to both copies" rule. `FamilyWitnesses.document_admitted` carries the document-surface claim under the v3 relation. | P5 (P2 honoured the both-copies rule for `serialCoordinate`) |
| V10 | `Scope.anchor_shape_separate` premises hand-built | repaired | see A5 | P2 |
| V11 | `BoundaryInteractions.wordPayload` trio | repaired | see A6 | P4 |
| V12 | `Preference` generic algebra (7 declarations) | superseded (justified) | real content (`omega`/induction), and `RolePreference` supplies the inhabited syntax-level instance. No change. | — |
| V13 | `PreferenceWitnesses.qualification_sites_excluded`, `measure_postmodifier_excluded` | superseded (justified) | `AdjunctLicense` is a closed inductive; "no constructor covers this pair" is model content, and each has a positive twin. No change. | — |
| V14 | `Documents.type_line_order` duplicates `NotationWitnesses.type_line_order` | superseded (justified) | both kept; a reciprocal doc note on each says the duplication is deliberate — each fixture module states the ordering rule its own witnesses depend on, and neither imports the other | P5 |
| V15 | `Reading.HasTense` / `NonfiniteUse` have no negative witness | repaired | see A1 | P1 |

Criterion (ii) confirmation: `lean_minimal_hypotheses` over the eight
"not findings after inspection" declarations found **no unused named
hypothesis** (P1 records the per-declaration verdicts). `FrameDeclarations`'
three `optional_does_not_*` theorems have no explicit binders at all, so
criterion (ii) is inapplicable to them rather than passed.

#### The five STOPs and their coordinator rulings — flagged for user review

Each STOP was raised by the audit plan as a contradiction among the ticket, the
retired inventory, the grammar-model landing record and the recorded
ADRs/glossary. The coordinator ruled on each before implementation; the rulings
are reproduced with how they were applied. **These five rulings decided
questions the ticket text alone did not settle and are the items most in need of
user review.**

**S1 — Oracle English `Selection` vs the "rename the identifier, not the entry"
rule.**
*Ruling:* Rename the identifier, per the standing project rule. CONTEXT-MAP
*permits* shared spellings; it does not require one, and `English.Selection` is
not a linguistic term. Rename module + namespace `English.Selection` →
`English.Preference`; if `English.Reading.Preference` makes that resolve
ambiguously at any use site, use `English.Ranking` instead and say so. Do **not**
add an Oracle English "Selection" glossary entry. Add the other preference-view
terms (Preference, Packing, Scope Class) as planned. `SelectionWitnesses`
follows. Update `DECLARATIONS.md` rows.
*Applied:* `English/Selection.lean` → `English/Preference.lean`,
`namespace English.Selection` → `English.Preference`;
`English/SelectionWitnesses.lean` → `English/PreferenceWitnesses.lean`,
namespace likewise. **No ambiguity arose** — `English.Reading.Preference` and
`English.Preference` coexist without a single resolution conflict, so the
`English.Ranking` fallback was **not** taken. 100 reference sites updated across
14 modules plus `English.lean`; build green. `DECLARATIONS.md` keeps the old
names in its "Old declaration" column (the file's convention) with the rename in
each Disposition, plus a note under each of the two section headers. No
"Selection" glossary entry was added; **Preference**, **Packing** and
**Scope Class** were.

**S2 — `quantity` / `quantify` naming.**
*Ruling:* Superseded: the Oracle English glossary's **Quantity** entry sanctions
exactly these names. No rename. Disposition text: "the inventory's naming claim
is withdrawn; the names match the owning glossary entry".
*Applied:* verbatim. `Construction.quantify`, `Production.quantify` and
`WordCategory.quantity` are unchanged; row B9c above carries the ruling's
disposition text.

**S3 — `Claims`/`Policy`: the inventory says superseded, the ADR says
preserved.**
*Ruling:* The ADR governs: superseded *as admission*, preserved *as optional
preference*. Do B4's repair (instantiate `Policy` on syntax through declared
features so the layer is not inert), keep the declarations. Disposition:
"repaired; the inventory's 'superseded' premise was broader than the ADR's
ruling".
*Applied:* nothing in `Preference.Claims`/`Policy` was removed. B4's repair
landed in P5 as `FrameInteractions.framePolicy` — claims read
`RolePreference.roleCount` and the new `RolePreference.identityCount`, region is
the ordered lexical-identity sequence. `frame_policy_selects_marked` selects the
marked-frame analysis; `frame_policy_does_not_revoke_admission` proves the
non-selected analysis stays admitted (the ADR's non-destructive clause, on
syntax); `role_blind_policy_selects_postmodifier` is the weakened-premise
counterexample. P3 and P4 explicitly held this work for P5.

**S4 — `chapter` / `dieDashRow`: every honest repair breaks a recorded rule.**
*Ruling:* Repair by distinguishing, using the observable Oracle difference —
Saga chapter lines are `I — …`, `II — …`; die-dash rows are `2 — …`,
`1 or 2 — …`. Give the two rules distinct notation categories, keep BOTH
families, keep `die_dash_text` and the chapter witnesses with their own surfaces,
and land the probe-verified two-tree ambiguity's replacement as a uniqueness
theorem or the two exclusions. No merge. Fixture notation values may be literal;
the grammar guard reads the category, not the string.
*Applied:* `NotationKind` (`plain`/`chapter`/`dieResult`) added to
`DocumentCategory.notation` (default `.plain`, so `classLevel`, `levelBand` and
`station` are untouched). `DocumentProduction.chapter` takes
`.document (.notation .chapter)`; `dieRow`/`dieDashRow` take
`.document (.notation .dieResult)`. `chapter_text` (`"I — Attack."`) and
`die_dash_text` (`"2 — Attack."`) are unchanged, surfaces byte-identical. The
two-tree ambiguity is replaced by `chapter_notation_is_not_a_die_row`,
`die_notation_is_not_a_chapter` and `notation_words_do_not_cross`. One fixture
was added (`Documents.dieFace`) because `Documents.level` could no longer serve
both rules — disclosed as a deviation in P4.

**S5 — the flat-coordination premise vs what the model can state.**
*Ruling:* State it as a model property, exactly as the plan proposes: "under the
declared linearizations, the nested binary bracketing has no derivation on the
serial-comma surface". Do not claim it settles the linguistic question. The
workbench ADR governs. Add the README sentence that says so.
*Applied:* verbatim, in two places —
`FamilyWitnesses.nested_rejects_serial_surface`'s doc comment and a paragraph in
`english/README.md` that says the exclusion is a property of *this model*, not
evidence that the nested analysis is wrong for Oracle English.
`FamilyWitnesses.nested_admitted_without_commas` is the weakened-premise
contrast: remove the commas and the nested bracketing realizes the surface
again, so the exclusion rests on the commas and not on the bracketing.

#### Nonvacuity witnesses (weakened-premise counterexamples)

Each central exclusion has a proved sibling in which the specific hypothesis is
dropped and the wrong tree *is* admitted. Both halves are proved in every case;
none is prose.

| law | weakened-premise counterexample |
|---|---|
| A1 three-dimension separation | `plain_form_both_finite_and_nonfinite` — one spelling, one Inflectional Form, two Finiteness values, both `Licensed`; `preterite_is_not_tense` discriminates `.past` from `.present` on one admitted tree |
| A3 head-owned countability | `genitive_determiner_is_transparent` conjunct 3 — the genitive declares *no* countability of its own; the retired wildcard constructor directly refutes it |
| A2/S5 serial exclusion | `FamilyWitnesses.nested_admitted_without_commas` — drop the comma atoms and the nested bracketing is fully admitted |
| A4 joint law | `JointInteractions.all_four_admitted_without_dependencies` vs `row_admitted_iff` — without `Dependencies.Safe` all four rows are admitted; with it exactly the correlated two survive |
| A5 anchor shape | `FrameScope.projectAnchors_flat_ne_nested` / `FamilyWitnesses.serial_anchor_count_coarser` — two derivable trees with equal `anchorCount` and different ordered topologies |
| A6 word payload | `BoundaryInteractions.Payload.both_analysed_without_payload` (both spellings analyse with the side condition dropped) vs `spaced_not_licensed` / `spaced_not_admitted` |
| V7 analysis roundtrip | `analysis_roundtrip_rejects_silence` and `analysis_roundtrip_rejects_constant` — a concrete wrong renderer, not merely the empty one |
| V8 value roundtrip | `value_roundtrip_rejects_variant_blind`, `value_roundtrip_rejects_case_blind` — the case-blind operation answers correctly for both declared-case values and is refuted by the `.initial` one alone |
| B7 `Placement.before` | `NominalWitnesses.degree_adjunct_rejects_after` — no `AdjunctLicense` constructor covers the pair at `.after` |
| B8 sharing | `Heads.ordinary_coordination_cannot_raise`, `Determiners.determiner_cannot_distribute_across_singular` |
| B10 paragraph route | `retired_node_route_premises` (before: the second route's premises were all satisfied) vs `body_has_no_node_production` + `paragraph_is_the_only_route` (after) |
| B4 preference layer | `role_blind_policy_selects_postmodifier` vs `frame_policy_does_not_revoke_admission` |
| B3 host invariant | `FrameScope.step_can_change_hosts` — the finding itself, proved rather than asserted |

#### Assurance counts, summed across P1–P5

| count | total | by packet |
|---|---|---|
| theorems added | **98** (101 unique new names by census) | P1 7, P2 17, P3 36, P4 31, P5 7 |
| theorems re-spelled | **7** | P1 1, P2 1, P3 1, P4 4, P5 0 |
| theorems retired with replacement named | **0** | — |
| non-theorem declarations retired with replacement named | **3** | P1 1 (`DeterminerUse.genitive` → `Features.Transparent`), P4 2 (`DocumentProduction.body` → `ParagraphProduction.body`; `wordPayload` → `Atom.WellFormed`) |
| theorems removed | **0** | — |
| theorems restored | **0** (none needed restoring) | — |
| theorems ignored / `#[ignore]`-equivalent | **0** | — |

The census figure (101) exceeds the per-packet semantic sum (98) because the
census counts unique `theorem <name>` source tokens and three names now occur in
two namespaces each (e.g. `anchor_shape_separate` in both `Scope` and
`FrameScope`). No theorem was deleted in any packet.

#### Theorem count before and after

| measure | before | after |
|---|---|---|
| named source theorems | **472** in 41 modules | **573** in 43 modules |
| compiled `English.` theorems (axiom-audit closure) | **2,400** | **2,681** |
| Lake build jobs | 44 | **46** |

The named-source census is
`grep -ohE '\btheorem [A-Za-z_]' english/English/*.lean | wc -l`, which
reproduces the recorded 472 exactly on the tree that recorded it. The two new
modules are `English/JointInteractions.lean` (P3) and
`English/FrameScopeWitnesses.lean` (P4); `English/Selection.lean` and
`English/SelectionWitnesses.lean` were renamed, not added.
`english/DECLARATIONS.md`'s "Proof accounting" section is refreshed with all of
these figures.

#### Glossary gaps

**None remain.** Fourteen entries were added to
`docs/contexts/oracle-english/CONTEXT.md` in the file's existing format
(bold term, colon, definition, optional `_Avoid_`), each placed to keep related
terms adjacent: **Derivation**, **Spelling Variant**, **Provenance**,
**Word Payload**, **Admission**, **Preference**, **Packing**, **Scope Class**,
**Atom**, **Type Line**, **Frame Slot**, **Premodifier**, **Anchor**,
**Serial Comma**. Per ruling **S1** no Oracle English **Selection** entry was
added; per ruling **S2** the existing **Quantity** entry was left untouched.
The plan drafted fourteen entries including **Selection**; ruling S1 struck that
one and B9b's rename added **Frame Slot** in its place (see Deviations).

CR citations: the only new ones are in the **Type Line** entry.
`cargo xtask cite check --list-noncompliant` reports **0 non-compliant
citation-looking strings**; `cargo xtask cite check` reports **15,710 citations
checked against `cr.txt` (eff. 2026-08-07); 0 stale**. Both `[CR#205.1,205.4a]` and
`[CR#302.3]` were already in `cr-citations.lock`, so no `bless` was needed.
`jj diff --git | cargo xtask cite audit --diff` audited both sites and each rule
text was read against its claim: `[CR#205.1]` gives the type line's contents,
`[CR#205.4a]` gives supertypes-before-card-types, and `[CR#302.3]` is the
long-dash-before-subtypes convention. The plan's draft cited `[CR#205.1]` alone
for the em dash; **that would have been a right-number-wrong-topic cite** —
`[CR#205.1]` says nothing about a dash — so the claim was split across the three
rules
that actually carry its parts.

#### Deviations and additions beyond the packet's letter

- **`Frame Slot` glossary entry added** (not in the plan's fourteen). B9b's
  `Complement` → `FrameSlot` rename needs an owning glossary term, and the
  project rule forbids reusing the **Complement** entry for a slot descriptor.
- **The `Complement` rename was taken, not transferred.** The plan sized it as
  "~14 fixture modules" and offered a transfer to
  `docs/tickets/planned/english-v3-whole-grammar-activation.md` if it did
  not fit. The real footprint is 62 sites in 12 files; it fitted, so no transfer
  line was appended to any other ticket and **no other ticket was edited**.
  `Selected` was rejected as the new name because `Preference.Selected` already
  exists.
- **B3 landed as a proved finding, not prose.** The plan said "report it and
  record the host invariant as belonging to `ScopeStep` only". A prose
  limitation is weaker than the ticket's own standard, so
  `FrameScope.step_can_change_hosts` exhibits the failure over
  `Step.determiner`; the README states the limitation alongside it.
- **B4's policy region is `List Lexeme`, not `Surface`.** The plan proposed
  `Policy (Syntax Lexeme) Surface`, but the workbench has no total tree →
  surface function (`Realizes` is a relation), so a `Surface` region could only
  have been a constant — an inert comparison region. `GrammaticalScope.lexicalLeaves`
  gives a real, non-constant region: two analyses are compared only if they bind
  the same lexical identities in the same order, which is exactly the
  comparison-region notion `Prefers` encodes. `frame_policy_same_region` proves
  the two candidates share one.
- **B4's `identity` claim needed a new counter.** `Claims` has two fields and the
  plan named `identity := tree contains an .identity leaf`; no such projection
  existed, so `RolePreference.identityCount` was added beside `roleCount` (same
  shape, same mutual-recursion idiom, reads a declared constructor).
  `frame_policy_claims_read_features` proves the claims function is not constant.
- **Two doc comments were reworded** so the phrase "theorem proved" does not
  appear in source prose; it would otherwise inflate the `theorem` token census
  the proof accounting uses.
- **`english/DECLARATIONS.md` "Proof accounting" refreshed once, in P5**, as P1,
  P2, P3 and P4 each recorded they were deferring it.
- **`english/README.md`'s obligation paragraph rewritten** — it pointed at
  `docs/tickets/planned/english-v3-lean-proof-audit.md` (this ticket, now in
  `wip/`) as owning obligations this landing discharges.
- Packet-level deviations (P1's A3 justification correction, P2's
  `anchorCount_projectAnchors` restatement and anchor-identity threading, P3's
  right-node-raised rows and per-row surfaces, P4's B10 ordering-claim
  correction and `Placement.before` exclusion strengthening) are recorded in
  full in the per-packet progress notes and are not restated here.

#### STOPs encountered during implementation

**None.** P1, P2, P3 and P4 each report zero STOPs; P5 reports zero. Every
contradiction the plan found was raised as S1–S5 *before* implementation and
ruled on by the coordinator, which is why no packet had to stop. No
ticket-versus-ruling contradiction was resolved without a STOP.

### Report

Provenance only; never fitted to and never a gate.

- **Stamped on change `vlxykxku`** (this record's commit). Per-packet figures are
  stamped on `syvllpvqw` (P1), `twxoykpmw` (P2), `xplmrzqxo` (P3) and
  `lkqzzztmm` (P4).
- `english/scripts/build`: **Build completed successfully (46 jobs)**, warnings
  as errors, no warnings.
- `english/scripts/axioms`: exit `0`; `English theorems checked: 2681`,
  `disallowed axiom uses: 0`.
- Named source theorems 472 → 573; modules 41 → 43; compiled `English.`
  theorems 2,400 → 2,681.
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15,710 citations, 0 stale.
- **Not measured and not claimed** (workbench exception): coverage-lock
  `covered` count, construction count, selection census, homograph and
  form-literal/vocabulary overlap inventories, licensing-checker total, and the
  coverage-command wall time and per-byte thread-CPU telemetry. No corpus
  command was run, because no Rust code and no declaration data changed.
