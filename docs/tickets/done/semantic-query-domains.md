---
needs: [core-entity-object-classes]
---
**Separate singular reference, collection, criterion, and filtering concepts
throughout the core query languages.** Use the [`Reference`, `Selection`,
`Predicate`, `Filter`, and `Source` definitions](../../contexts/game-model/CONTEXT.md). A `Reference`
denotes one Object or Player; a `Selection` denotes a collection; a `Predicate`
states a truth criterion over candidates from a known domain; a `Filter` is the
operation that applies one.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Audit `deckmaste_core`'s ASTs and the Idris workbench
(`idris/src/Experimental/`), together with their core-facing RON, lowering
output, and evaluation paths, for values that violate those boundaries. Move
set-valued `Reference::Source` behavior into a Selection or a specially named
predicate, because Source is a contextual relation—source of an Ability,
damage, or mana—not an Object class. Preserve singular Object-or-Player
References so targeting Players, attaching Auras to Players, and placing
counters on Players do not require duplicate languages.

Make every query constructor's candidate domain explicit enough that a
Player-only predicate cannot silently run over Objects and an Object predicate
cannot silently admit Players. This ticket does not decide whether Object and
Player identities share physical storage; it consumes the public Entity
boundary from `core-entity-object-classes`.

Split the current broad `Sort` vocabulary as needed: `ReferentSort` applies to
Entity-valued References and Selections, while `Amount`, `Pile`, and pile-valued
Selections need an explicitly named value or collection domain. Do not broaden
Reference to amounts or piles merely to preserve the current enum. Acceptance
includes the existing `PilesOf` and `Them(Pile)` paths as well as Entity-valued
queries, with the workspace green and `cd idris && ./scripts/build` green at
its module count.

## As landed

### Constructor → candidate domain → what changed

`Reference` (SINGULAR, Entity-valued). New `Reference::referent_domain()`
states each constructor's domain; `Predicate::Ref(r)` now reads it, so
"the candidate IS r" narrows the region that carries it.

| Constructor | Domain | Change |
| --- | --- | --- |
| `Reg(RefId)` | `Entity` | domain now stated (the parameter's `Kind` is the real answer) |
| `Single(Selection)` | the selection's `element_domain()` | new |
| `OpponentOf` / `ControllerOf` / `OwnerOf` | `Player` | new ([CR#109.5,108.3,102.2]) |
| `AttachHostOf` | `Entity` | new, and NOT `Object`: [CR#303.4b] an Aura enchants "that object or player" |
| `Coalesce` | `meet` of the arms | new — disagreeing arms widen rather than lie |
| `Source` | — | **deleted**; moved to `StatePredicate::WasDealtDamageBy` |

`Selection` (a GROUP). New `Selection::collection_domain()` (which register
shape it reads) and `Selection::element_domain()` (what its members are).

| Constructor | Collection / element domain | Change |
| --- | --- | --- |
| `Reg(RefId)` | `Entities` / `Entity` | now an Entity-group read ONLY; a pile register is refused at load |
| `Pile(RefId)` | `Pile` / `Object` | **new variant** — the pile-domain register read ([CR#700.3a,700.3b]) |
| `SelectAll(Region<Predicate>)` | `Entities` / the region's declared domain | unchanged |
| `Random(Quantity, Region<Predicate>)` | `Entities` / the region's declared domain | the bare `Predicate` now rides a candidate region |
| `Union` | `Entities` / `meet` of members | stated |
| `InChosenOrder` | `Entities` / inner | stated |
| `TopOfLibrary`/`BottomOfLibrary`/`LibraryOf`/`TopOfGraveyard` | `Entities` / `Object` | stated |
| `ValidTargetsFor` | `Entities` / `Entity` | stated — [CR#707.10d] reads "each player or object" |
| `Pick { proj }` | `Entities` / `proj.of`'s | stated |

`Countable` (the count-side twin, new `Countable::element_domain()`):
`Objects` → the region's declared domain; `Players` → `Player` ([CR#102.1]);
`Singleton(r)` → `r.referent_domain()`; `ManaSymbols`/`ManaSpentMatching` →
`Entity` (not an Entity query at all).

`Predicate`: `Ref(r)` → `r.referent_domain()` (was always `Entity`);
`State(WasDealtDamageBy(f))` → subject `Object`, relatum `Object`
([CR#120.1]); `Relation(Attachment(_))` subject `Object` → `Entity` and
`Relation(AttachedTo(_))` relatum `Object` → `Entity`, both [CR#303.4b].

### The `Source` move

`Reference::Source` is gone from core. It was never one Entity: it named the
SET of an object's marked-damage sources, fizzled in every reference
evaluator (`resolve::query`, `target::resolve_frameless_reference`), and was
special-cased ahead of `Condition::Matches`'s own evaluator. [CR#120.1] — "an
object that deals damage is the source of that damage" — makes being a source
a contextual relation between two objects, so it is now the named predicate
of the DAMAGED object:

`StatePredicate::WasDealtDamageBy(Arc<Predicate>)` — existential over the
candidate's damage marks, read against each mark's DEAL-TIME abilities
([CR#120.3,702.2c]). It lives beside its `Targets`/`RelatedBy`/`WasPutFrom`
peers, so it works in `Exists`, in a filter, and in a target constraint, not
only inside `Matches`. The engine's live matcher (`target::matches_with…`)
answers it; the LKI snapshot matcher answers `false` (a snapshot holds a
damage TOTAL, not the marks). The lethal-damage SBA's deathtouch clause
([CR#704.5h]) is now `Matches(This, WasDealtDamageBy(Has(Deathtouch)))`.

`deckmaste_semantics::Reference::Source` and
`plugins/builtin/macros/identity/Source.ron` are untouched (scope ruling):
lowering rewrites `Matches(Source, F)` whole into the named relation, and
refuses a `Source` in any other position, naming [CR#120.1]. No RON data file
changed. (Both sentences are superseded by "Merged onto the landed rounds"
below: default had meanwhile deleted `Source.ron` and named the relation
`Condition::DealtDamageBy`, which the whole-condition rewrite now targets.)

### The `Sort` split

`deckmaste_core::Sort` had one flat vocabulary spanning three register
domains and no consumer at all — lowering produced it and nothing read it.
It is now domain-tagged, and it has a real consumer:

- `ReferentSort` = `Player | Card | Token | Spell | StackObject | Permanent |
  OfType(Type)` — the ENTITY-valued nouns, with `key()`, `noun()`,
  `domain()` (Player vs Object) and the `compat` relation `compatible_with()`.
- `Sort` = `Referent(ReferentSort) | Amount | Pile` — the three register
  domains. `Sort::register_kind()` names each explicitly (`Entity`,
  `Number`, `Pile`), `Sort::answered_by(Kind)` and `Sort::compatible_with`
  refuse every cross-domain pairing by construction.
- `deckmaste_lowering::region`'s hand-written `compatible` and
  `kind_compatible` tables are deleted; both now delegate to the core sort,
  so the anaphor resolver and core cannot disagree about which domain a
  mention reaches. Behaviour is identical (the old tables' arms are the
  new methods' arms).
- `Them(Pile)` lowers to `Selection::Pile`; `They`/`AmongNoted` route through
  a new `region::group_read`, which picks the collection domain from the
  register's own declared `Kind`. `validate_read`'s `expected == Entities &&
  found == Pile` escape hatch is deleted — the constructor says which domain
  it means, so no cross-domain allowance is needed.

### Residues owned by siblings

- **Workbench pile domain (not landed).** `Words.idr` still has
  `kindOfW PileW = Object` and `PileP : … -> Payload Object`, so a pile is
  typed exactly as a singular Object — the same violation core just fixed
  ([CR#700.3b]). The fix is `Pile : Kind` plus `PileOf`/`InPile`/
  `PileMention`/`Macros.onePile`/`Macros.pileOfChoice` retyped to
  `Noun bs Pile`. It cannot land in this round's region: `Effect.Move` and
  `Effect.SetStatus` take `Noun bs Object` and four cards plus `ProofsPiles`
  hand them pile mentions, so retyping forces kind-indexing on those two
  constructors and on `instrIntro`/`doesInstrIntro`/`doesPreIntro`/
  `riderIntro`/`doesRiderIntro`/`doesAnnIntro`/`costActionOk`/`heldUntilOk`
  — the loop intros this round's brief assigns to `workbench-loop-delta`.
  Recorded rather than edited, per the brief's region discipline. The Idris
  tree is untouched this round.
- **`Selection::PilesOf`** (a SET of piles, not one) still has no core
  spelling; `Selection::Pile` names one pile register. The existing refusal
  already names its owner, `engine-piles`.
- **`deckmaste_semantics`** keeps `Reference::Source` and the flat `Sort`;
  they converge at cutover when the RON re-emit path points at
  `Experimental` (scope ruling 2026-09-04).

### Merged onto the landed rounds

Rebasing onto default (`ability-kind-taxonomy`, `type-def-permanent-type-flag`,
`effect-instruction-taxonomy`, `idris-sba-not-a-static-ability`, the
`core-entity-object-classes` follow-up) collided on the `Source` move alone,
because `core-reference-source-query` had meanwhile deleted
`Reference::Source` from core too and named the relation
`Condition::DealtDamageBy(subject, F)` — a semantics variant, an authored
`DealtDamageBy(This, Has(Deathtouch))` in `plugins/builtin/rules/sba/
lethal-damage.ron`, an identity-registry entry and an Idris emitter. Both
deletions stand and both spellings survive: the authored/production path keeps
default's `Condition::DealtDamageBy` (so the legacy `Matches(Source, F)` and
the explicit spelling lower to one core shape), while this round's
`StatePredicate::WasDealtDamageBy` keeps carrying the same query into `Exists`,
filters and target constraints where a `Condition` cannot reach. The one
matcher, `source_abilities_match`, moved to `target.rs` as this round wrote it
and is now `pub(crate)`, shared by both evaluators instead of duplicated. The
lowering refusal for a bare `Source` names both [CR#120.1] and the
`DealtDamageBy` condition, satisfying both rounds' refusal tests, and the SBA
deathtouch test keeps default's name while asserting both spellings against the
same marks.

## Landing record

Gates (foreground, last line of each):

- `cargo check --workspace --all-targets` → ``Finished `dev` profile [unoptimized + debuginfo] target(s)``, no diagnostics
- `cargo fmt --check` → exit 0, no diff
- `cargo test --workspace` → 6097 passed, 0 failed, 6 ignored across 128 test binaries (the 6 ignores are pre-existing)
- `cargo clippy --workspace --all-targets` → only the 10 pre-existing `deckmaste_lowering/src/card.rs` `unneeded pattern` warnings remain; every warning this round introduced is fixed
- `cd idris && ./scripts/build` → exit 0, 0 Warning lines; the full run at round start printed `44/44: Building Cards (src/Cards.idr)`. The Idris tree is unchanged this round (`jj st` lists only `crates/`), so the incremental re-run rebuilt nothing.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`
- `cargo xtask cite check` → `checked 18247 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` → `blessed 1583 rules`, `cr-citations.lock` byte-identical (no new rules)
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 71 citation site(s)`, each read against its claim
- `plugins/wizards` needs no regeneration: nothing in the diff touches the semantics grammar or the generators.

Acceptance paths: `Them(Pile)` runs end-to-end through Do or Die
(`deckmaste_plugin` canon `do_or_die_lowers_pile_labels_to_registers`, now
asserting `Selection::Pile` and `CollectionDomain::Pile`); `PilesOf` keeps its
own lowering-refusal test unchanged and passing.

Rust test counts: 6097 passed / 0 failed / 6 ignored.

Assurance counts: restored 0; re-spelled 20; ignored-with-blocker 0 added;
added 8; removed 0.

Re-spelled (subject renamed or retired, same asserted outcome):
`deckmaste_core::region::pile_registers_validate_as_iterable_groups`;
`deckmaste_core::cost` ×3 (the `Random` region shape);
`deckmaste_lowering::sort::lowers_sort_*` ×7 (the tagged `Referent` shape);
`deckmaste_lowering::reference::lowers_reference_source` →
`source_is_not_a_core_reference`;
`deckmaste_lowering::selection::lowers_selection_random`;
`deckmaste_engine::sba::is_source_has_deathtouch_reads_deal_time_marks` →
`was_dealt_damage_by_deathtouch_reads_deal_time_marks`;
`deckmaste_engine::target::unresolvable_ref_in_filter_fizzles_instead_of_asserting`;
`deckmaste_engine::resolve::query` ×1; `deckmaste_engine::resolve::action` ×1;
`deckmaste_engine` `payment.rs` ×2; `deckmaste_plugin` `canon.rs` ×1.

Added: `core::sort::{each_sort_names_one_register_domain, domains_do_not_cross,
referent_compat_is_preserved, referent_sorts_name_their_entity_domain}`;
`core::region::{a_pile_register_is_not_an_entity_group,
a_damage_source_is_read_in_the_object_domain,
every_query_constructor_names_its_domain,
a_reference_predicate_narrows_to_its_referents_domain}`.

### Deviations and additions

- **`Reference::AttachHostOf` is `Domain::Entity`, not `Domain::Object`, and
  `RelationPredicate`'s attachment arms were corrected with it.** Drafting
  the reference table I cited [CR#301.5,303.4] for "only an object can host an
  attachment" and the audit's rule text refuted the claim: [CR#303.4b] — "the
  object OR PLAYER an Aura is attached to is called enchanted". The ticket's
  own letter names attaching Auras to Players as a case that must keep
  working, so an Object-narrowed host would have refused a legitimate filter
  at load. `RelationPredicate::Attachment`'s SUBJECT (the host) and
  `AttachedTo`'s RELATUM (also the host) both move `Object` → `Entity`; the
  attachment side of each stays `Object` ([CR#301.5,303.4] — an Aura or
  Equipment is a permanent). Both were landed by `core-entity-object-classes`
  as `Object`; no ruling asserts the narrower reading, so this is a
  correction, not a contradiction, and it is disclosed here rather than
  taken as a STOP.
- **`validate_read`'s pile→Entities allowance is deleted.** Once
  `Selection::Pile` exists, a pile register read is spelled as one; the
  allowance existed only because `Selection::Reg` had to cover both domains.
  `region::pile_registers_validate_as_iterable_groups` is re-spelled to the
  pile constructor and a new witness pins both cross-domain refusals.
- **`Predicate::Ref(r)` now narrows to `r.referent_domain()`.** Previously
  every `Ref` widened its region to `Entity`, which is the exact silent
  admission the ticket forbids. No corpus card is affected (workspace green).
- **`Countable::element_domain` added** beside `Selection::element_domain`, so
  `Selection::Pick`'s domain is not re-derived at its one call site.
- **Three stale doc references fixed** in `deckmaste_core`: two mentions of the
  retired `Reference::It` (`Predicate::Where`, `Projection`) and one
  `Kind(Spell)` left over from `core-entity-object-classes`
  (`Predicate::FromSource`). Comment-only, zero behaviour.
- **`Countable::Players`'s domain comment cites [CR#102.1], not [CR#119.1].**
  The inherited [CR#119.1] (starting life totals) does not support the claim
  "the fold ranges over players alone"; [CR#102.1] does.
- **One `#[allow(clippy::too_many_lines)]`** on
  `trigger::filter_matches_snapshot_with_activation`, whose one-arm-per-leaf
  match crossed the 150-line threshold when the new snapshot arm was added;
  the function already carried a matching `match_same_arms` allow.
- **`deckmaste_engine::target::unresolvable_ref_in_filter_fizzles_instead_of_asserting`
  lost one of its three cases** — the `Reference::Source` entry, whose subject
  this round deletes. Its other two cases (a `Single` selection and an
  unattached `AttachHostOf`) are unchanged, so the test keeps its subject and
  outcome; the deleted case is covered instead by
  `deckmaste_lowering::reference::source_is_not_a_core_reference`, which pins
  the refusal at the boundary that can now state it.

STOP: none. No shape contradicts a recorded ruling; the one place the drafted
design and the CR disagreed (the Aura host domain) was caught by the citation
audit and resolved toward the ticket's own letter, disclosed above.
