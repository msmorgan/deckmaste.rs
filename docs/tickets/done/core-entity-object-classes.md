---
needs: [core-copy-grammar]
---
**Replace the exclusive `ObjectKind` axis with the Entity/Object boundary and
nonexclusive CR Object classes.** The target vocabulary is defined in the
[Game Model glossary](../../contexts/game-model/CONTEXT.md): a Player and an Object are both project
Entities, but a Player is not a CR Object; Card, copy of a card, Token, Spell,
Permanent, Emblem, and ability on the stack are overlapping classifications
from [CR#109.1].

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Today `ObjectKind` conflates three questions: whether a candidate is a Player
or Object, what represents an Object, and which current rules roles it has. It
also makes the false claim that Players are Objects. An exclusive enum cannot
say that a card on the stack is both a Card and Spell or that a battlefield
Token is both a Token and Permanent.

Change core's predicate language to make its candidate domain explicit at the
Entity boundary. Replace `Predicate::Kind(ObjectKind)` with independently
testable Object classes plus a Player classification at the Entity level. Carry
the same distinction through the Idris workbench (`idris/src/Experimental/`),
lowering's core-facing output, candidate enumeration, snapshots, and the engine
classifier. The spell and permanent tests may be derived from the relevant
state, but their public names and behavior remain the CR classes rather than
storage tags.

This ticket deliberately does **not** choose whether Players and Objects occupy
one slotmap or separate stores. Existing player proxies may remain behind an
adapter until that representation is relitigated; they must no longer leak the
claim that a Player is an Object into public types, filters, or prose.

Overlaps `workbench-ability-kind-fold`, which folds the workbench's `Kind`
family so an ability on the stack is an `Object` with an `AbilityP` payload;
whichever runs second reconciles the names.

Acceptance: a single candidate can satisfy every applicable Object class;
Player-only predicates do not enter the Object domain; representative Card +
Spell and Token + Permanent conjunctions work in `deckmaste_core` and in the
workbench, with `cd idris && ./scripts/build` green at its module count; and
repository prose contains no affirmative "players are objects" claim.

## As landed

- **Core shape.** `Predicate::Kind(ObjectKind)` is gone. Two atoms replace it:
  `Predicate::Entity(EntityClass)` (`Player` | `Object`) is the Entity-level
  classification, and `Predicate::Class(ObjectClass)` is one independently
  testable CR object class per atom (`AbilityOnStack`, `Card`, `CopyOfACard`,
  `Emblem`, `Permanent`, `Spell`, `Token` — the glossary's spellings).
  Conjunction is ordinary `And`, so `And([Class(Card), Class(Spell)])` and
  `And([Class(Token), Class(Permanent)])` are representable.
- **Entity boundary representation.** A typed domain rider on the region's
  candidate parameter, per the core-explicit-regions contract law 2:
  `Provenance::Candidate(Domain)` with `Domain = Player | Object | Entity`.
  `Region::candidate_in(domain, body)` declares it, `Region::candidate(body)`
  keeps the widest (`Entity`), `Region::over(predicate)` declares the narrowest
  domain the predicate's own atoms admit (`Predicate::subject_domain`), and
  `Region::candidate_domain()` reads it back. `deckmaste_core::validate`
  refuses, at load, a predicate whose subject cannot inhabit the declared
  domain (`ValidationError::CandidateDomain`); the domain travels through
  `And`/`Or`/`Not` and switches at a relation boundary to the relatum's own
  domain (`RelationPredicate::relatum`). Lowering declares every predicate
  region through `region::predicate_region`, which retags the candidate from
  the lowered predicate, so no lowered card can violate the rule.
  `deckmaste_core::Kind::Object`/`Objects` are renamed `Entity`/`Entities`:
  the register shape names the Entity it holds instead of calling a player
  proxy an object. The proxy itself is untouched — one slotmap versus two
  stays unrelitigated.
- **Engine classifier.** `target::object_kind` is replaced by
  `target::entity_class` (player proxy → `Player`, card-backed → `Object`) and
  `target::is_object_class(state, id, class)`, which answers each class
  independently off the same state it always read: `AbilityOnStack` from the
  stack entry ([CR#602.2a,603.3]); `Card` = card-backed and not a token, emblem,
  or card-less copy ([CR#108.2,108.2b,114.5,707.10]); `CopyOfACard` from the
  `StackEntry.copy` marker; `Emblem` and `Token` from the card instance;
  `Permanent` = on the battlefield and not an emblem ([CR#110.1,114.5]);
  `Spell` = on the stack ([CR#112.1,112.1a]). A player answers `false` to every
  class. `matches_region_with_activation` gates the declared domain before the
  predicate runs, so candidate enumeration over an Object-domain region never
  offers a player and the reverse.
- **Workbench shape.** The `Kind` index already fixes the candidate domain
  (`Predicate bs Object` vs `Predicate bs Player`), so the Entity boundary
  needed no new mechanism; the Object classes gained the missing members:
  `IsSpell`, `IsEmblem`, `IsCopyOfACard` beside `IsCard`, `IsToken`,
  `Permanent`, all positional. `Macros.spell` stopped being the storage
  spelling `InZone (ZoneAt Stack Bare)` and is now the CR class `IsSpell`; new
  macros `emblem`, `copyOfACard`, `cardOnTheStack` (`And [IsCard, IsSpell]`) and
  `tokenOnTheBattlefield` (`And [IsToken, Permanent]`). `cardTokenClashOf`
  gained the emblem pairs ([CR#114.5]) and card/copy-of-a-card ([CR#707.10]);
  `seedZone` gained `IsSpell → Stack` ([CR#112.1]) and `IsEmblem → Command`
  ([CR#114.2]). `Kind.Ability` is untouched, and nothing here blocks the
  `workbench-ability-kind-fold` sweep from folding it into `Object`.
- **Witnesses.** Core: `a_card_on_the_stack_is_both_card_and_spell`,
  `a_battlefield_token_is_both_token_and_permanent`,
  `a_player_only_predicate_is_refused_in_the_object_domain`,
  `an_object_only_predicate_is_refused_in_the_player_domain`,
  `a_relation_switches_the_domain_of_its_relatum`. Engine: the ability-on-stack
  test now also asserts a card on the stack is Card AND Spell, and the token
  creation test asserts Token AND Permanent AND not Card. Workbench:
  `Cards.Description.targetCardOnTheStack`, `eachTokenOnTheBattlefield`,
  `eachEmblemYouOwn`, `aCopyOfACard`; pins `badPlayerReadInObjectDomain`,
  `badCardToken`, `badEmblemPermanent`, `badCardCopyOfACard`, with
  `okPlayerReadInPlayerDomain`, `okCardAndSpell`, `okTokenAndPermanent`.
- **Not done, and why.** `plugins/builtin_v2/` holds only macro stubs and meta
  descriptors — no core-facing filter RON exists there, so the ticket's RON
  scope was a no-op. `deckmaste_semantics` keeps its exclusive `ObjectKind`
  (deletion-bound, scope ruling): lowering now maps it with
  `filter::object_kind_predicate`, `Player` to `Entity(Player)` and each other
  member to its object class. Whether Players and Objects share one slotmap is
  untouched, as the ticket directs.

## Landing record

Gates (foreground, last line of each):

- `cargo check --workspace` → ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.92s``
- `cargo test -p deckmaste_core -p deckmaste_lowering -p deckmaste_engine -p deckmaste_plugin` → 2102 passed, 0 failed, 4 ignored (the four ignores are pre-existing)
- `cargo fmt --check` → exit 0, no diff
- `cd idris && ./scripts/build` → `44/44: Building Cards (src/Cards.idr)`, 0 Warning lines
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`
- `cargo xtask cite check` → `checked 18024 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` → `blessed 1583 rules`, lock file byte-identical (no new rules)
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 72 citation site(s)`, each read against its claim
- `grep -rniE "players are objects|player is an object" crates docs/contexts idris | grep -vi "not an object"` → empty

Rust test counts: 2102 passed / 0 failed / 4 ignored across the four crates
(gate scope plus `deckmaste_plugin`, whose `tokens` test names core filters).

Assurance counts: restored 0; re-spelled 169 (every test function whose body
named `ObjectKind`, `Predicate::Kind`, `Provenance::Candidate`, or
`Kind::Object`/`Objects` — the subjects and asserted outcomes are unchanged);
ignored-with-blocker 0 added; added 5 (the core domain tests listed above, plus
in-place class assertions inside two existing engine tests); removed 0.

Idris pin probes: each of the four new pins was mis-stated once and the build
reported `… is not a valid impossible case` for it, then restored.

Deviations and additions:

- **`Kind::Object`/`Kind::Objects` renamed to `Kind::Entity`/`Kind::Entities`
  (265 sites, mechanical).** The register-shape variant was a public type whose
  doc said it holds "a player proxy" — the exact leak the ticket forbids. Pure
  rename, compiler-checked.
- **`Macros.spell` repointed from `InZone (ZoneAt Stack Bare)` to `IsSpell`.**
  The ticket requires the spell test to keep its CR name as the public
  predicate rather than a storage/zone tag; the seed zone is unchanged
  (`seedZone IsSpell = Just Stack`).
- **`Predicate::player()` helper** added beside `Predicate::r#type`/`creature`
  for the common Entity-level filter.
- **Three prose fixes in `deckmaste_semantics`** (`count.rs`, `filter.rs`,
  `reference.rs`) which asserted "players are objects". The scope ruling keeps
  that crate's *shape* untouched; the acceptance criterion is repository-wide
  prose, and these are comment-only, zero-behaviour edits.
- **`ObjectClass::CopyOfACard` is now nonexclusive with `Spell`** where the
  engine's copy marker and the stack coincide: a card-less copy on the stack
  answers `true` to both ([CR#112.1a,707.10]). Under the old exclusive enum the
  stack arm won and `CardCopy` was unreachable there. No corpus filter reads
  `CopyOfACard`, and the two existing copy tests keep their outcomes.
- **`cardTokenClashOf` extended** with emblem×{card, permanent, token}
  ([CR#114.5]) and card×copy-of-a-card ([CR#707.10]). Additive: no existing
  bench term uses `IsEmblem` or `IsCopyOfACard`.

STOP: none. No shape choice contradicted a recorded ruling or the ticket's
letter; every printed card shape reachable before stays reachable, and the
`workbench-ability-kind-fold` overlap is left open as the ADR §3 sweep expects.

### Follow-up: candidate-domain drift (2026-09-04)

**Root cause.** The round narrowed the candidate domain only on the lowering
path (`region::predicate_region` retags from `Predicate::subject_domain`) and
left `deckmaste_core::Region::candidate` — the constructor every hand-built
region uses — declaring the widest `Domain::Entity`, so the Ascend gate's
Object-only census declared `Candidate(Object)` when lowered and
`Candidate(Entity)` everywhere else; `deckmaste_migrations` and
`deckmaste_noncanon` were outside the round's test gate, so neither resulting
breakage was seen before integrate.

**Fix.**

- `Region::candidate` no longer takes the widest domain: it reads the declared
  domain from the body through a new `deckmaste_core::CandidateBody` trait
  (`Predicate` answers `subject_domain()`; `Condition`, `Count`,
  `StaticEffect`, and `Block` answer `Domain::Entity`, since a value or effect
  computed per candidate says nothing about the candidate's Entity class).
  `Region::over` is now the predicate-named spelling of the same thing, so
  core, engine, and lowering cannot disagree by construction.
- The migrations drift guard's hand-written canonical carried a stale
  `Domain::Entity` where the gate's atoms (`InZone(Battlefield)`,
  `ControlledBy`) admit only objects; corrected to `Domain::Object`. The
  `ASCEND_GATE` constant and every other field of the canonical `Condition`
  are untouched.
- `crates/deckmaste_noncanon/strategies/sped_red.ron` still spelled the
  retired core `Kind(Player)`; re-spelled `Entity(Player)` (plus the matching
  prose in `strategy.rs`).

**New test.**
`deckmaste_lowering::region::tests::both_predicate_region_paths_declare_the_same_candidate_domain`
— lowers three semantic predicates and asserts lowering's `predicate_region`,
`Region::over`, and `Region::candidate` all declare the same domain, and that
it is `Object` for the Ascend gate's body, `Player` for `Kind(Player)`, and
`Entity` for `Any`.

Gates (foreground, last line of each):

- `cargo check --workspace` → ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s``
- `cargo fmt --check` → exit 0, no diff
- `cargo test --workspace` → 6085 passed, 0 failed, 6 ignored across 127 test
  binaries; `resolve::tests::ascend_gate_const_matches_canonical_condition ... ok`
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`
- `cargo xtask cite check` → `checked 18049 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 3 citation site(s)`, each read against its claim
- `idris/` unchanged, so its build was not re-run.

Assurance counts: restored 0; re-spelled 1; ignored-with-blocker 0; added 1;
removed 0. The re-spelling is `deckmaste_engine::condition::tests::compare_counts_stack_census`,
whose fixture minted the in-flight announce's occupant as `ObjectSource::Player`
— a player proxy standing in for a spell. `Objects(InZone(Stack))` now correctly
declares the Object domain and does not count a player, so the fixture mints a
card-backed occupant instead. Same subject, same asserted outcome.

**Deviations and additions.**

- **The brief's premise that both sides of the drift guard are lowerings is
  wrong**, and the correction changes one token in the guard. The `canonical`
  side is a hand-typed `Condition` literal inside the test; the landing round
  re-spelled its `Provenance::Candidate` → `Provenance::Candidate(Domain::Entity)`
  mechanically without applying the round's own new narrowing rule. Since
  `Objects(InZone(Battlefield) ∧ ControlledBy(You))` is an Object-domain count
  — which the brief itself names as the expected answer — the literal had to
  become `Domain::Object`. The guard's subject (the `ASCEND_GATE` RON string
  and the whole rest of the lowered `Condition`) is unchanged, so this restores
  the guard rather than weakening it.
- **`deckmaste_noncanon` fixed too**, beyond the brief's core/lowering scope: it
  carried a second, independent breakage from the same landing round's gate gap
  and blocked `cargo test --workspace`.
- **One test-helper signature gained a bound** (`resolve::effect::tests::candidate_region`
  now requires `T: CandidateBody`) — mechanical, compiler-driven.

STOP: none.
