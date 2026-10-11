---
needs: []
---
# Export verbless Predicate permission from the state-Adjective class

Replace the one-member *saddled* licence with the declared state-Adjective
class permission, preserving existing owners and one adjectival route.
Standard constraints apply.

## Analysis

Owner correction (user, 2026-10-10, verbatim): "hypothetical lines such as attacks while equipped/enchanted/haunted and shit should work too, not a specific construction for saddled".

- **VW7** (user, 2026-10-10): permission belongs to the state-predicate class
  by derivation, with no member list. Every existing state/participial
  Adjective exports `VerblessPredicateUse: Yes` through its declared class,
  origin or participial derivation: keyword states, status Adjectives, and
  adjectival past participles already predicative after *be/become/remain*.
  Delete the one-member class row. Each derivation exports the permission.
  If a class has no declared marker, STOP and report it; do not list words.
- **VW8** supersedes VW2's speculative-member exclusion and VW4 as needed:
  *while tapped/equipped/enchanted/haunted/blocked* must read even where
  unattested. Reuse the adjectival route of *is tapped*, with no subjectless
  passive twin. W9-A preserves independent ambiguity; record each count
  beside its *is <word>* control.
- **VW9**: before/after probes for *This creature gets +1/+1 while saddled.*,
  *This creature attacks while equipped/enchanted/haunted/tapped/blocked.*,
  plus previous witnesses and controls. Selector: every face containing
  *while*, *if able* or *as long as*, plus 300 random covered faces, seed
  20261008. Bounded official census/diff only. Zero covered-face changes
  expected; report any increase and its shape. Full suites and clippy for
  touched crates. Tests assert class behaviour with at least four members
  from different derivations, not list equality. Six-item landing record and
  standard refresh rules apply. If covered faces change or a class lacks
  a marker, STOP. Otherwise finish, refresh, integrate, drop, and report the
  change id, class markers and probe table without another review pause.

- **VW10** (orchestrator, 2026-10-10): finish the class independently of
  vocabulary gaps. Existing state Adjectives inherit permission through
  their derivations/class markers. Add haunted through the owning haunt
  definition's participial-adjective export, without a core member row.
  Its state-Adjective Reading beside the verbal passive under *be* is the
  accepted KS shape; report covered-face increases and counts. If that
  export needs a schema change, leave haunted as a vocabulary residual and
  finish the class. Other missing sources are residuals, not STOPs. Run
  `cargo test -p xtask english_v3` and `cargo xtask lean check` once before
  integrating. Self-integrate only with 0 lost, 0 decreases and increases
  restricted to the KS shape on faces containing the new state Adjective;
  otherwise STOP. This resolves the vocabulary-source STOP recorded below.

- **VW11** (orchestrator, 2026-10-10): trunk decreases introduced by
  refresh are attributed to their owning change, including the 25 from
  `qktrquumtlmk` authorized by CF7. They do not count against this ticket's
  self-integration gate: ticket-caused lost/decreases must be zero and
  increases must be only the KS shape on newly derived haunted faces.
- **VW12** (orchestrator, 2026-10-10): prefer the class of Adjectives
  admissible as predicative Complements, using the existing declared
  predicative restriction. Replace redundant per-derivation exports and
  include unblocked, unspent, red and monocolored. Adopt only if the same
  7,102-face census has zero ticket-caused lost/decreases and no increases
  on covered faces except the KS shape. Otherwise retain the state-class
  exports, mark negative participial Adjectives through their derivation or
  existing class, list remaining unmarked predicative residuals, and report
  the rejected trial's increases. No grammar member lists. Run full suites
  and clippy, apply refresh checks, integrate and drop without another STOP.

## Method

Inspect the existing state-Adjective derivations and declarations; re-probe
all witnesses and controls before implementation. Census selector
`\bwhile\b|\bif\b|\bas long as\b|\bhaunted\b` case-insensitively, union with the
300 official random covered controls. Use an identity manifest, six workers,
zero samples per face and `/usr/bin/time -v`; compare with the official diff.
The selector includes every *if* face because it shares the state-Predicate
permission and every *haunted* face to measure the newly derived Adjective
outside the while construction. It includes VW9's required faces.

## Resolved VW7 STOP: missing haunted state-Adjective declaration

Inspection and complete before probes at `znzxkzuk` establish:

- Native participial-Adjective classes declare `AdjectiveOrigin: Participial`
  for the existing tap/untap/block and exile/crew derivations.
- Keyword `participial_adjective` recipes derive equipped, enchanted and
  suspended through `legacy::plugins::participial_adjective_lexeme`.
- Designation Adjectives, including saddled, come from declared Designation
  identities with `grammar: Adjective`; they have a different export path.
- **Haunted has no declared state-Adjective derivation or class marker.**
  `plugins_v2/builtin/macros/keyword_abilities/haunt.ron` declares a Verb
  export only, with no `participial_adjective` recipe; neither native
  participial class includes it. The complete *This creature is haunted.*
  probe contains one `PassiveComplement` / `PassivePredicate` Reading over
  `lexeme:keyword_ability/haunt/verb`, not an Adjectival Complement.
  *This creature remains haunted.* reads 0. Deriving an Adjective from every
  verbal past participle would exceed the existing state-Adjective class;
  adding a word-specific member would contradict VW7. STOP for a ruling on
  the missing declared state-Adjective source before implementation.

Requested before probes (complete, 0 issues; *is* and *remains* controls use
*This creature* as Subject):

| Predicate | Requested while witness | is control | remains control |
|---|---:|---:|---:|
| saddled (*gets +1/+1*) | 1 | 2 | 1 |
| equipped (*attacks*) | 0 | 1 | 1 |
| enchanted (*attacks*) | 0 | 1 | 1 |
| haunted (*attacks*) | 0 | 1, passive only | 0 |
| tapped (*attacks*) | 0 | 2 | 1 |
| blocked (*attacks*) | 0 | 2 | 1 |

Additional state controls, *is/remains*: untapped 2/1, crewed 2/1,
exiled 2/1, suspended 1/1. Prior Ghoda, Jaguar, Mascot and Dracosaur witnesses
already read 1 each; their controls without *while saddled* also read 1
and are retained as controls, not failing witnesses. The local *attacks
while saddled* reads 1, *has flying while tapped* reads 0; finite *while
it's saddled* 2, ordinary attack-trigger control 1, finite *while it's
tapped* 4. Every probe enumerated completely with 0 issues.

Evidence: `target/english-v3/english-v3-verbless-state-predicate-class/`
contains `probe-inputs.json` and `before-probe-*.json`, produced with
`cargo xtask english-v3 probe --text … --category sentence --readings 64
--json > target/…/probe.json` (plus `--as-card` for the previous card witnesses and controls).
At that STOP, no implementation, test, lexicon, census, suite or clippy
change/check had been made. VW10 subsequently authorized the missing export
and continuation; the final checks are recorded below.

## Landing record

1. **Census delta** — On `yovlyztp`, before edits (the code tree of parent
   `znzxkzuk`), the bounded manifest covers 5,972 of 7,102 faces with 214,320
   Readings. The manifest unions 6,862 matches of
   `\bwhile\b|\bif\b|\bas long as\b|\bhaunted\b` with 300 official random
   covered controls, seed 20261008 (60 overlaps). The final frame-constrained
   census at `yovlyztp` has identical totals; its official line diff reports
   0 gained/lost/decreased/increased/digest-only faces and 0 issues.
   Evidence: before.json, after-final.json, diff-final.json. Lost faces: none.
   After refresh at `xxtorssp`: 5,972 → 6,045 covered, 214,320 → 216,140
   Readings; 73 trunk gains, 0 lost/increases, 25 decreases and 80 digest-only
   faces. The official after-final → refresh-after diff attributes every
   change to refresh, with the identical counts above. The 25 decreases are
   the 62 outer-focus Readings deliberately retired by `qktrquumtlmk`, CF7,
   in [coordinated-focused-adjuncts](../done/english-v3-coordinated-focused-adjuncts.md).
   Their face counts are: Arcum's Sleigh 2→1; Ashen Ghoul 3→2; Assassin's Blade 7→4; Champion's Victory 7→4; Coffin Puppets 2→1; Command of Unsummoning 7→4; Deep Wood 30→27; Defenders of Humanity 21→14; Defiant Stand 7→4; Eightfold Maze 7→4; Essence Anchor 3→2; Gutterbones 3→2; Hakim, Loreweaver 4→3; Harsh Justice 12→9; Heavy Fog 30→27; Just Fate 7→4; Kjeldoran Guard 2→1; Kongming's Contraptions 6→3; Kuldotha Phoenix 5→3; Nemesis Phoenix 3→2; Rally the Troops 7→4; Remove 7→4; Scorching Winds 7→4; Treetop Defense 7→4; Warrior's Stand 7→4.
   No decrease originates in this ticket. VW11 resolves the prior gate
   STOP: these owner-approved trunk decreases are attributed to CF7 and
   excluded from this ticket's gate. No retired wrong route is restored.
   The VW12 preferred-class trial compares refresh-after.json with
   vw12-preferred-after.json: all 7,102 faces unchanged, 0 gains/lost/
   decreases/increases/digest-only, 6,045 covered and 216,140 Readings.
   The corrected final census vw12-final-after.json has the same totals
   and its official vw12-final-only-diff.json again reports all 7,102
   faces unchanged. Its official parent comparison has the same 73 gains,
   25 decreases and 80 digest-only trunk changes, with 0 ticket-caused
   gains, lost faces, decreases, increases or digest changes.
   The other incoming implementations are `vszmnwup` / `ttzrlpkw`
   (comparatives), `mqyqzunv` (devotion) and `tsvuoyup` (spelled cardinals);
   refresh-incoming-changes.txt lists the complete ancestry. Census evidence:
   refresh-after.json, refresh-diff.json, refresh-only-diff.json. The only supported
   haunted face is Cecily, Haunted Mage: haunted occurs in its proper Name,
   so no covered-face state-Adjective increase is expected there.
   Final refresh brought ooumkosu (supertype-bound keywords) and
   wsskrpxl (subtype/plugin definitions). The bounded re-run
   vw12-post-refresh-after.json again has 6,045 covered faces, 216,140
   Readings and 0 issues; its official line diff against vw12-final-after
   reports all 7,102 faces unchanged. The parent diff still reports exactly
   the previously attributed 73 gains, 25 CF7 decreases and 80 digest-only
   changes. Ticket-caused gains/lost/decreases/increases/digest-only: all 0.
   Integration refresh imports oyqrxmwz (exactly-cardinal), zvoyutom
   (phrasal keywords) and pusrqwml (partitive quantities). Its official
   integration-refresh-only-diff.json adds 68 trunk gains, 84 digest-only
   changes and zero lost/decreased/increased faces; the partitive digest
   changes use its canonical CardinalDeterminer wrapper. The final parent
   diff reports 5,972→6,113 covered, 214,320→235,412 Readings, 141 trunk
   gains, 0 lost/increases, the same 25 CF7 decreases and 157 digest-only
   changes. There are 0 issues. No new material from this ticket changes
   a covered face; the preferred class satisfies VW11/VW12's gate.
   Waterbend refresh (ulrnwvuw) adds seven trunk gains and no other change:
   waterbend-refresh-only-diff.json retains every previously covered
   Reading and digest. Final parent comparison: 5,972→6,120 covered,
   214,320→235,479 Readings, 148 trunk gains, 0 lost/increases, the same
   25 CF7 decreases, 157 digest-only changes and 0 issues. Ticket-caused
   census changes remain zero. Evidence: waterbend-refresh-after.json,
   waterbend-refresh-only-diff.json, waterbend-refresh-parent-diff.json.
   Declaration-batch refresh (mlnortmq) adds 12 trunk gains and no covered
   count/digest change. Its final parent diff has 5,972→6,132 covered,
   214,320→239,562 Readings, 160 trunk gains, 0 lost/increases, the same
   25 CF7 decreases and 157 digest-only changes. All 7,102 faces complete
   with 0 issues. Ticket-caused gains/lost/decreases/increases/digests: zero.
   Evidence: batch-b-refresh-after.json, batch-b-refresh-only-diff.json,
   batch-b-refresh-parent-diff.json.
   Partitive-head refresh (qnvlmzuu) adds 15 trunk gains and no covered
   count/digest changes: 6,147 covered, 241,700 Readings; final parent diff
   has 175 trunk gains, 0 lost/increases, the same 25 CF7 decreases and
   157 digest-only changes. All 7,102 faces complete with 0 issues;
   ticket-caused changes remain zero. Evidence: partitive-head-refresh-
   after.json, partitive-head-refresh-only-diff.json and
   partitive-head-refresh-parent-diff.json.
2. **Witnesses** — At `yovlyztp`, the six requested while probes each read 1:
   gets +1/+1 while saddled; attacks while equipped, enchanted, haunted,
   tapped and blocked. Parent while counts are respectively 1/0/0/0/0/0;
   saddled was already covered and was retained as a control. Their is
   controls before → after are respectively 2→2, 1→1, 1→1, 1→2, 2→2, 2→2.
   Haunted's added Reading is the authorized copular Adjectival Complement
   beside its preserved verbal passive (VW10); remains haunted changes 0→1.
   Other remains controls remain 1. Crew/exile/suspend/prepared/untapped
   while siblings change 0→1; is crewed/exiled/untapped remain 2, is suspended
   remains 1. Prior Ghoda/Jaguar/Mascot/Dracosaur witnesses and controls read
   1 each; local while saddled stays 1, has flying while tapped changes 0→1,
   finite while saddled stays 2, finite while tapped stays 4, and the attack
   trigger control stays 1. VW12 unblocked/unspent/red/monocolored/legendary
   while probes change 0→1; their is/remains controls each stay 1. Corrected
   other/same while probes each read 1 beside their is controls of 1;
   while greater/less read 0 and if able stays 1. All final probes enumerate
   completely with 0 issues. Evidence: vw12-final-probe-*.json,
   vw12-before-*.json, vw12-preferred-while-*.json, vw12-corrected-*.json.
3. **Spot checks** — No ticket-owned newly covered corpus faces exist in the
   pre-refresh final diff,
   so there are no ten gained faces to name. Independent hypothetical probes
   and role assertions exercise saddled, prepared, tapped, untapped, blocked,
   crewed, exiled, equipped, enchanted and suspended (plus haunted): each
   forms a clause-final Preposition Predicate containing while + Verbless
   Clause with its existing Adjective owner. This is the same one route as
   its control, with no subjectless passive alternative.
4. **Deviations and STOPs** — VW7–VW12 supersede the one-member
   adjective-class row, which is deleted. The preferred predicative-class
   implementation is adopted: the loader derives VerblessPredicateUse for
   every existing Adjective with an unframed or empty Predicate realization,
   excluding ImplicitComparativeUse's existing attributive-only restriction.
   These are the declarations underlying Adjective/IntransitiveAdjective
   and AdjectiveAttributiveOnlyUse on AdjectivalComplement. No word guard,
   member list, duplicate lexeme or new owner is introduced. The now-redundant
   native participial-class and keyword/Designation exports are removed.
   Haunt declares its existing participial-adjective recipe in the owning
   plugin; no schema change or core member row. The vocabulary-source STOP
   at znzxkzuk was resolved by VW10; the unmarked unblocked/unspent STOP
   at xxtorssp is resolved by VW12. Both are members of the natural
   predicative class and now read under while, as do red/monocolored/legendary.
   No remaining unmarked predicative residual was found.
   Probes found wrong omitted-selected-Complement routes for blocked and,
   in the preferred trial, other. The latter initially read 2 although its
   bare is control has one adjectival route. VerblessClause restricts derived
   permission to unframed or Intransitive selections. Its ordinary framed
   alternative is added; the existing saddled/able form and native
   participial forms remain stable. VerblessComplementOmissionUse explicitly
   preserves the existing able omission decision without granting omission
   to the broader class. Both feature terms are recorded in the glossary.
   CGEL p. 1267 supports reducible Clause Complements only; class permission
   and distribution are project rulings. The synthetic loader test checks
   complete values, owner preservation, attributive/selected exclusions,
   idempotence and conflicting-value rejection. The single/extra lexical
   fixture is re-spelled to include the derived feature while retaining its
   complete feature-map comparison. The new legendary exclusion is
   re-spelled as the exact positive class Reading under VW12.
   An additional, optional facts check after the phrasal-keyword refresh
   fails: ChooseABackground has no gate-column overlay. That registry
   checker reads builtin declarations and xtask overlays, not the adjective
   loader; the inherited zvoyutom declaration is unrelated to this class.
   No facts overlay or exemption is added in this ticket. Required English,
   lexical-source, xtask-English and Lean gates all pass. The failed command
   is preserved in integration-refresh-facts.log rather than reported green.
5. **Tests** — On vsvvsrvs, cargo test -p deckmaste_english_v3 PASS:
   975 tests across all six binaries. cargo test -p deckmaste_lexical_source
   PASS: 87 tests (43 unit, 44 integration). cargo clippy -p
   deckmaste_english_v3 --tests -- -D warnings PASS; cargo clippy -p
   deckmaste_lexical_source --tests -- -D warnings PASS. cargo build -p
   xtask PASS. The earlier once-required cargo test -p xtask english_v3
   passed 56 tests, and cargo xtask lean check passed 240 cards across two
   plugins. Restored 0 / re-spelled 2 / ignored 0 / added 4 / removed 0:
   the existing tapped exclusion and single/extra lexical fixture are
   re-spelled against their authorized class; three role/roundtrip tests
   are registered once in suite_4, and one synthetic loader test is added.
   No test is removed or weakened. The initial preferred-suite failures
   exposed the missing fixture feature and the wrong omitted other-than
   frame; the complete suites pass after the fixture update and production
   frame fix. Initial and post-first-refresh suites/clippy also passed;
   their logs remain in target. cargo xtask cite check --list-noncompliant
   reports 0; cargo xtask cite check reports 0 stale; jj --no-pager diff
   --git | cargo xtask cite audit --diff audits 0 citation sites; cargo
   xtask facts check PASS. After the final refresh, cargo build -p xtask
   PASS; cargo test -p deckmaste_english_v3 PASS (979 tests); cargo test -p
   deckmaste_lexical_source PASS (87 tests); cargo clippy -p
   deckmaste_english_v3 --tests -- -D warnings PASS; cargo clippy -p
   deckmaste_lexical_source --tests -- -D warnings PASS; cargo test -p xtask
   english_v3 PASS (56 tests). The refreshed cargo xtask lean check PASS: all 242 cards across two
   plugins prove Card.check = []. Integration refresh checks also PASS:
   cargo build -p xtask; cargo test -p deckmaste_english_v3 (998 tests);
   cargo test -p deckmaste_lexical_source (87 tests); cargo clippy -p
   deckmaste_english_v3 --tests -- -D warnings; cargo clippy -p
   deckmaste_lexical_source --tests -- -D warnings; cargo test -p xtask
   english_v3 (56 tests); cargo xtask lean check (242 cards, two plugins).
   After waterbend refresh these same commands PASS again: 1,002 English
   tests, 87 lexical-source tests, 56 xtask-English tests, both clippy
   commands and 242 Lean card proofs across two plugins. The additional
   cargo xtask facts check fails on the inherited missing ChooseABackground
   overlay, as disclosed in item 4; its earlier pre-phrasal passes remain
   historical results. Declaration-batch refresh checks PASS: cargo build
   -p xtask; cargo test -p deckmaste_english_v3 (1,009 tests); cargo test -p
   deckmaste_lexical_source (88 tests); cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings; cargo clippy -p deckmaste_lexical_source --tests
   -- -D warnings. This refresh changes no plugin definitions or xtask
   code, so the passed xtask-English/Lean results are retained.
   Partitive-head refresh checks PASS: cargo build -p xtask; cargo test -p
   deckmaste_english_v3 (1,018 tests); cargo test -p deckmaste_lexical_source
   (88 tests); cargo clippy -p deckmaste_english_v3 --tests -- -D warnings;
   cargo clippy -p deckmaste_lexical_source --tests -- -D warnings. No plugin
   definitions or xtask code changed; their prior passed checks are retained.
6. **Timing** — Latest bounded census: 347 s (rounded wall time), six
   workers, zero samples per face. No full corpus census was run, per the
   binding verification budget. Evidence is under
   target/english-v3/english-v3-verbless-state-predicate-class/.
