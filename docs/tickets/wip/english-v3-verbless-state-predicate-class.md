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
   No decrease originates in this ticket's pre-refresh change. Nevertheless,
   the post-refresh 25 decreases fail VW10's literal self-integration gate:
   STOP, pending a ruling. No retired wrong outer-focus route is restored.
   The other incoming implementations are `vszmnwup` / `ttzrlpkw`
   (comparatives), `mqyqzunv` (devotion) and `tsvuoyup` (spelled cardinals);
   refresh-incoming-changes.txt lists the complete ancestry. Census evidence:
   refresh-after.json, refresh-diff.json, refresh-only-diff.json. The only supported
   haunted face is Cecily, Haunted Mage: haunted occurs in its proper Name,
   so no covered-face state-Adjective increase is expected there.
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
   trigger control stays 1. All final probes enumerate completely with 0 issues.
3. **Spot checks** — No ticket-owned newly covered corpus faces exist in the
   pre-refresh final diff,
   so there are no ten gained faces to name. Independent hypothetical probes
   and role assertions exercise saddled, prepared, tapped, untapped, blocked,
   crewed, exiled, equipped, enchanted and suspended (plus haunted): each
   forms a clause-final Preposition Predicate containing while + Verbless
   Clause with its existing Adjective owner. This is the same one route as
   its control, with no subjectless passive alternative.
4. **Deviations and STOPs** — VW7–VW10 replace the superseded one-member
   adjective-class row, which is deleted. Native participial classes export
   VerblessPredicateUse with their existing AdjectiveOrigin marker; no member
   list changes. Keyword participial_adjective recipes and Designation
   declarations with Adjective grammar export the permission generically in
   the loader, without naming a word or changing owners. Haunt declares the
   existing participial-adjective recipe on its owning plugin: no schema
   change or core member row. The vocabulary-source STOP at `znzxkzuk` was
   resolved by VW10. A later native-source audit at `xxtorssp` found
   invariant core Adjectives unblocked and unspent with empty feature maps,
   no state/participial marker or declared state-class membership. Both is/remains
   controls read 1; both bare while phrases read 0, completely with 0
   issues. Integration is held for the VW7 unmarked-class STOP unless a
   ruling classifies these existing ordinary Adjective sources as VW10
   vocabulary residuals. No word list or new marker has been introduced.
   Evidence: marker-probe-*.json; core.ron owns both declarations. One extra blocked
   route was found by probes: the selected by-Complement frame was accepted
   without its Complement. The existing VerblessClause now separates native
   participial unframed/intransitive selections from ordinary licensed heads;
   the existing saddled/able forms remain stable. Two surface alternatives,
   no new Construction or lexical frame, are added to enforce VW8's one
   route. The VerblessPredicateUse glossary entry records the class ruling;
   no terminology gap remains. CGEL p. 1267 supports reducible Clause
   Complements only; class permission and distribution are project rulings.
5. **Tests** — At `yovlyztp`, cargo test -p deckmaste_english_v3 passes all
   945 tests, and cargo test -p deckmaste_lexical_source passes all 86 tests.
   Both cargo clippy -p deckmaste_english_v3 --tests -- -D warnings and
   cargo clippy -p deckmaste_lexical_source --tests -- -D warnings pass.
   cargo test -p xtask english_v3 passes 56 tests; cargo xtask lean check
   passes for 240 cards across two plugins.
   Restored 0 / re-spelled 1 / ignored 0 / added 3 / removed 0:
   independent_complement_keeps_owner_roundtrips_and_lexical_licences retains
   its subject and now asserts the licensed tapped Reading; the three new
   class-role tests are registered once in suite_4. No test deletion or
   weakening. cargo xtask cite check --list-noncompliant reports 0; cargo xtask cite
   check reports 0 stale; jj --no-pager diff --git | cargo xtask cite audit
   --diff audits 0 citation sites; cargo xtask facts check passes. The
   citation checker initially treated a decimal timing as a rule-number
   lookalike; integer seconds resolved that documentation-only failure.
   After refresh at `xxtorssp`, cargo build -p xtask and the same four full
   touched-crate test/clippy commands PASS again: 975 English tests and 86
   lexical-source tests. The once-required xtask English and Lean checks
   are retained; refresh changed no xtask code or plugin definitions.
6. **Timing** — Latest bounded census at `xxtorssp`: 956 s (rounded wall
   time, including Cargo's build-lock wait), six workers, zero samples per
   face. No full corpus census was run, per the binding verification budget.
   Evidence is under target/english-v3/english-v3-verbless-state-predicate-class/.
