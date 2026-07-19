---
needs: []
---
"Can't be countered" and split-second-style stack lockout. "Cast only …"
timing is already wired (`DeonticAction::Cast{window}`); this covers the
can't-be-countered static and stack-wide lockout consumption.

---

## Spec (design settled 2026-07-18)

Full design: `docs/superpowers/specs/2026-07-18-core-casting-restrictions-design.md`.
Standard constraints apply; deltas only below.

**Reconciled reality.** The can't-be-countered ENGINE is already done+tested by
`engine-cant-be-countered` (`DeonticAction::Counter` `deontic.rs:271`;
`counter_legal` `legal.rs:1204` ← `resolve/action.rs:107`; `cant_counter_rows`
gathers battlefield grants + a spell's OWN stack statics, [CR#113.6g,701.6a]);
render exists (`render/deontic.rs:131`). Split-second has no engine consumption —
a `Cant(Cast)` row panics at the LOUD `guard_deontic_seam` trip (`legal.rs:86`),
which `engine-durations-grants` explicitly assigned here. Decisions: (1) full
scope — split-second + can't-be-countered parse arm (self+grant) + graduate all
one-away cards; (2) generic `Cant(Cast)`/`Cant(Activate)` row eval (not a bespoke
split-second flag).

### A. Split-second lockout (engine)

Mirrors `cant_counter_rows`/`counter_legal` + the `may_cast_rows` timing loop
(`cast.rs:801`). No new enum: split second = `Static(Cant(Cast(())))` +
`Static(Cant(Activate(())))` (all-default slots) carried by the spell, gathered
while on the stack.

- **`cant_cast_rows` + `cant_cast`** (new, `legal.rs`): gather `Cant(Cast)` from
  battlefield permanents + **every card-backed stack object's own statics**;
  forbidden iff a row's `what`/`by` match the candidate/caster. Wire into
  `castable_cost_ignoring_mana` (`cast.rs:801`) after `timing_ok` (`return None`).
  Cant beats May → flash still locked ([CR#101.2]).
- **`Cant(Activate)` cost-scoped vs blanket.** `cost_predicate_holds(None,_)==true`,
  so a blanket split-second row would leak into the mana-tap gate and wrongly
  block mana abilities. Fix: cost-scoped rows (`cost: Some(_)`, summoning-sickness)
  apply to both arms; **blanket rows (`cost: None`) apply only at the non-mana
  `can_activate` gate** (`activate.rs:322`) — mana abilities exempt per
  [CR#702.61b] (also Linvala/Damping Matrix family). Extend `cant_activate_rows`
  (`legal.rs:594`) to scan card-backed stack objects; add a role param so the mana
  arm (`legal.rs:205`) passes "ignore blanket", `can_activate` passes "honor all".
- **Exemptions fall out structurally** (no special-casing): mana arm never sees
  blanket rows; `PlayLand`/special actions gated by `May(Play)` not `Cant(Cast)`;
  triggers aren't player Cast/Activate. [CR#702.61b].
- **Clear the seam**: `legal.rs:234` Cast arm returns `false` (don't trip) for
  `is_cant(d)` — convert, never delete the trip; still trips `Must`/`Gate(Cast)`
  + `May(Cast)` with from/cost. Combat seams (`:294`,`:446`) untouched.
- **Idris**: no new constructor (`Cast|Activate` relations + `cant` exist,
  `Core.idr:337`); stack-sourcing is engine-level (ForThisEvent precedent). Verify
  + run idris-check once foregrounded or skip-and-say.
- **Tests** (`deckmaste_engine`, mint lockout static inline like `cant_be_countered_*`):
  cast locked (+ lifts off-stack); flash still locked; non-mana activate locked;
  mana ability allowed; PlayLand allowed; triggers land; battlefield-grant
  generalization (Grand Abolisher / Linvala); seam no longer panics but still
  guards `Must(Cast)`. No new decision kinds → sim/decide untouched.

### B. can't-be-countered surface

Parse lives in `crates/deckmaste_migrations/src/parsers/static_ability.rs`.

- **Parse arm** `parse_cant_be_countered` (passive form, twin of
  `parse_cant_be_blocked` `:204`): after the `"be blocked"` guard in
  `parse_restriction` (`:171`), handle `"be countered"` → `Cant(Counter(on: {subj}))`.
  Self `~` → `Ref(This)` (byte-identical to the tested engine form + render input).
  "by …"/duration tails decline to `None` (stay `Unparsed`).
- **Grant form**: same arm, spell-subject filter; confirm/extend
  `subject_to_filter`/`parse_phrase` for spell subjects ("Creature spells",
  "Spells you control") — the one open impl risk.
- **Graduate** (sole-`Unparsed` one-away): **22 self-form** (Akroma Angel of Fury,
  Blurred Mongoose, Carnage Tyrant, Eject, Great Sable Stag, Heated Debate,
  Hit-Monkey, Inescapable Blaze, Last Word, Lightning Mare, Overwhelming Denial,
  Scragnoth, Skylasher, Tears of Valakut, Terra Stomper, Thrun the Last Troll,
  Toski Bearer of Secrets, Tyrranax Rex, Void Rend, Volcanic Fallout, Wilson
  Refined Grizzly, Wreak Havoc) + **Gaea's Herald** (only clean grant one-away).
  Wipe-first wizards regen; cards suite round-trip.
- Deferred (multi-`Unparsed`, need machinery elsewhere): conditional / "next
  spell" / cost-reduction-compound / agent-scoped forms (Chimil, Rhythm of the
  Wild, Autumn's Veil, Veil of Summer, Thryx, Cunning Nightbonder, …).

### Non-goals

`SplitSecond` keyword macro + its graduations → `kw-split-second` (downstream).
Blanket `Cant(Activate)` that must also block mana abilities. Prevention / combat
Gate seams.
