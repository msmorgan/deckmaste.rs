---
needs: []
---
**A target slot whose filter reads an EARLIER slot's announced register is
enumerated before that register exists.** Found by
`core-regions-witness-fixtures`; it is the blocker on two of its nineteen
witnesses. Standard constraints apply.

## The gap

`GameState::legal_targets_for_specs` maps each `TargetSpec` to
`legal_targets(spec, carrier, activation)` INDEPENDENTLY, and the announce
activation's `AnnouncedTarget(k)` registers are written only later, by
`ChooseTargets::resolve`. A filter that reads `Reg(AnnouncedTarget(0))`
therefore evaluates against an unavailable product at enumeration time, and
null packing decides the slot the wrong way in both directions:

- a POSITIVE cross-reference yields an EMPTY candidate set — Fiery
  Annihilation's "up to one target Equipment attached to that creature" can
  never be chosen at all;
- a NEGATED one yields EVERY candidate — Run Away Together's "two target
  creatures controlled by different players" accepts two creatures under one
  controller, which is not a legal announcement ([CR#601.2c]).

The ADR's law that targets are "a telescope over earlier params" is what the
enumeration must honour: slot `k`'s candidates are relative to the choices
already made for slots `< k`.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`fiery_annihilation_exiles_only_equipment_on_the_damaged_creature` and
`run_away_together_refuses_two_targets_under_one_controller` lose their
`#[ignore]` and pass unchanged. Whatever shape the fix takes — announcing
slots in order, or re-deriving later slots as earlier ones are picked — the
`ChooseTargets` prompt must still be answerable in one submission, since
`ChooseTargets::resolve` validates the whole set.

## Landing record

### What the rules say

[CR#601.2c] is the whole authority: "The player announces their choice of an
appropriate object or player for each target the spell requires." The CR gives
no order among a spell's target slots — announcement is one step, and
[CR#601.2e] then checks the proposal as a whole — so *appropriate* for a slot
whose criteria name another target of the same spell is a property of the
announcement, not of the slot alone. Both cards' official rulings read the
same way: Run Away Together's (2025-11-17) says that if both creatures are
under one controller "both targets are illegal", and Fiery Annihilation's
(2024-11-08) that the Equipment is exiled only while attached to the target
creature.

The declaration ORDER the ticket relies on is core's, not the CR's:
`deckmaste_core`'s telescope validation lets target slot `k`'s filter region
declare `Provenance::AnnouncedTarget(0..k)` parameters and nothing later. That
is what makes a slot-by-slot walk total, and it is the law this landing leans
on. The engine may fix slots in that order because it admits exactly the same
announcements the set-level reading does.

### The ticket's localization held; its acceptance did not

The diagnosis was right where it pointed: `legal_targets_for_specs` mapped each
spec independently, so `Reg(AnnouncedTarget(0))` read an empty register and
null packing decided the slot the wrong way in both directions (Fiery
Annihilation's Equipment slot came back `[]`, Run Away Together's negated slot
came back with every creature). Reproduced before touching anything; the dump
confirmed the second spec's region carries
`Param { def: DefId(3), kind: Objects, provenance: AnnouncedTarget(0) }` and
`legal: [[33v1, 38v1], []]`.

What did NOT hold is the acceptance's "pass unchanged". Fiery Annihilation
asserted `targeting.slot(1) == vec![worn]` — the candidate set offered for the
Equipment slot **in the first target prompt**. No engine can satisfy that. The
menu is built before the player announces anything, so the creature the
Equipment must be attached to is not yet chosen; and the fixture's own pick for
the creature slot can only reach the engine through a `Decision::Targets`
answering a `ChooseTargets` — which `Targeting::answer` records, making any
earlier prompt the one `slot()` reads. A prefix prompt of a different kind was
built and discarded: the shared `routine` answer takes the first candidate,
which in this fixture is `other` (33v1), not `equipped` (38v1), so the fixture's
stated pick would have been overridden.

### The shape taken

The ticket's second sanctioned shape — "re-deriving later slots as earlier ones
are picked", one prompt, one submission, `ChooseTargets::resolve` validating the
whole set:

- `resolve::announced_prefix_len` reads how many earlier slots a spec's filter
  declares, off the region's parameters.
- `GameState::slot_candidates` widens the MENU for a slot that reads an earlier
  slot while nothing is announced: the union over that earlier slot's own
  candidates — every object some legal earlier choice would admit. Plain
  enumeration otherwise, including once a real announcement is in the register
  file, and for `ActivationId::NONE` (no register file to write).
- `GameState::cross_target_choice_legal` enforces the cross-reference on the
  submitted set: walk the slots in declaration order, write each announced slot
  into the activation, re-enumerate the next against it, and refuse if a pick
  falls outside. Called from `ChooseTargets::resolve` after the count and
  `Distinct` checks. The register file is restored on the way out.

Numbers: no card in `plugins/canon` (0 of 372) or `plugins/wizards` (0 of
30,688) has a target slot whose filter reads another slot, so
`announced_prefix_len` is 0 for the whole corpus and every corpus path is the
single-prompt code unchanged. Only hand-spelled fixtures exercise the new
branch.

### Deviations and additions

- **`fiery_annihilation_exiles_only_equipment_on_the_damaged_creature` is
  re-spelled**, per the assurance rule (same card, same asserted outcome, new
  spelling), for the reason above. The offered-menu assertion becomes the
  refusal Run Away Together already spells: announcing the Equipment attached
  to the OTHER creature is refused as `DecisionError::Illegal`, and the one
  attached to the announced creature is accepted. Every outcome assertion
  (the creature dies to 5 damage, one Witness Gear in exile, the other
  creature's Equipment untouched) is byte-identical. The stale "BLOCKED in the
  engine" paragraphs on both tests were rewritten; no assertion in Run Away
  Together changed at all.
- **No `deckmaste_core` change**, so no wizards regeneration and no build-script
  touch was required; the diff is `deckmaste_engine` only.

### Residue

- The castability precheck (`announcement_effect_satisfiable`) runs with
  `ActivationId::NONE` and so still judges a cross-referencing slot on the
  null-packed set. It stays permissive, which is what it was before; making it
  honest needs a register file it does not have. Unreachable for every corpus
  card.
- `Retarget` ([CR#707.10c]) re-derives against the entry's CURRENT targets and
  does not re-check a cross-referencing slot when an earlier slot is changed.
  Also unreachable for every corpus card.

Both are routed to `docs/tickets/planned/engine-cross-target-retarget-and-precheck.md`.

### Test counts (assurance form)

- restored: 0
- re-spelled: 1 (Fiery Annihilation, above)
- added: 0
- removed: 0
- `#[ignore]` closed: 2 — `fiery_annihilation_exiles_only_equipment_on_the_damaged_creature`,
  `run_away_together_refuses_two_targets_under_one_controller`
- `#[ignore]` added: 0. This landing closed exactly two; after refreshing onto
  the default line (which had meanwhile closed Painful Quandary's blocker) the
  file's remaining ignores are the two grammar ones — Deadly Brew and
  Perforating Artist — each still naming its own blocker and owned by another
  ticket.

### Gates

Re-run after `kata refresh` moved the feature onto the default line; these are
the post-refresh numbers.

- `cargo test -p deckmaste_core -p deckmaste_lowering` — ok; 53/738/3/4/0/0
  passed, 0 failed, 0 ignored.
- `cargo test -p deckmaste_engine` — all suites ok, 0 failed. Ignored: 1 lib
  (`reconfigure_suppresses_creature`, engine-attach seam), 1 sim
  (`bears_vs_bolts_50k_game_stats`, release-only Monte-Carlo), 2
  `region_witnesses`. `region_witnesses`: 17 passed, 0 failed, 2 ignored.
  Lib suite: 740 passed, 0 failed, 1 ignored.
- `cargo test -p deckmaste_plugin` — all suites ok, 0 failed, 0 ignored.
- `cargo clippy -p deckmaste_core -p deckmaste_lowering -p deckmaste_engine -p
  deckmaste_plugin --all-targets` — clean.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17658 citations, 0 stale. No new rule number, so no
  `bless` and no lock change.
- `cargo xtask cite audit --diff` — 6 sites, all [CR#601.2c] but one unchanged
  [CR#603.7,603.12] line; each read against its claim.
- `cargo xtask idris-check plugins/canon --differential` — 68 agreed sound, 0
  agreed unsound, 12 skipped; differential OK: 0 disagreements.

### STOPs

One judgement call, disclosed rather than silently resolved: the ticket's
acceptance demands the two tests pass **unchanged**, and one of them cannot,
for the reason set out above — the assertion is about a menu built before the
information it asserts exists. The assurance rule's re-spelling clause is what
was applied: the same card, the same asserted outcome, a new spelling. Nothing
was deleted or weakened, and Run Away Together — which asserts the same law in
the refusal form — passes with its body untouched.
