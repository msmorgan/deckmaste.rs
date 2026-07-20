---
needs: []
---
**An announced target is a positional slot: it must be read ONLY by
`Target(n)` (singular) / `Targets(n)` (plural), and those reads must name ONLY
target slots.** Today a single target answers to *five* spellings — `It`,
`That(Sort)`, `Target(n)`, `They`, `Them(Sort)` — because the target is dumped
onto the shared wildcard antecedent stack alongside loop elements, `With`
choices, event roles, and move products, where R1/R2 lets anything compatible
grab it by proximity. A target is not an anaphor over that stack; it is an
indexed entry in the announce list [CR#601.2c,115.3]. This is the root-cause
correction; it is a core-model change, not a parser tweak.

## What's actually wired (engine, `resolve/query.rs`)

- `Target(n)` (`:473`) — the correct read: indexes `frame.anaphora.targets[n]`.
- `It` (`:395..430`) — checks the loop binding (`anaphora.it`); if unbound,
  **falls back to the lone announced target** (`no_other_singular &&
  flat.len() == 1` at `:413..425`). A special case for the single-target ability.
- `They`/`Them` (`:211..217`) — read the announced targets as "the one Many
  antecedent" (Arc Lightning's 1–3).
- `That(Sort)` (`:438..467`) — reads a target only after a `Move` relabels it a
  product; the surface still advertises it as a target read (`target_spec.rs:6..10`).

So three code paths read `anaphora.targets` directly (`It`-fallback,
`They`/`Them`, `Target(n)`), and the surface claims a fourth. That is the
special-casing — one concept, five spellings, disambiguated by luck of the
antecedent stack.

## Why this is the root of the earlier band-aids

Because the target rides the shared stack, it *competes* with whatever else the
frame binds:

- A trigger also binds an event role (`that_object`), so bare `It` on a
  triggered targeted body would be ambiguous and fizzle — which is the ONLY
  reason `triggered_ability.rs` carries the `\bIt\b → Target(0)` regex rewrite
  (`target_slot_reads`, `:98..123`). Separate the channel and that rewrite is dead.
- Two same-sort announce slots (the fight family) can't be told apart by
  anaphor at all — which is why [[cards-fidelity-target-sunset]] had to grow the
  whole `As(label)`/`The(label)` announce-labelling apparatus and freeze an R2
  ambiguity gate. With positional target reads, `Target(0)`/`Target(1)` name the
  slots directly and none of that machinery is needed for targets.
- `parse_damage_and_damage` (`effect.rs:242..260`) already emits
  `Sequentially([DealDamage(This,N,It), DealDamage(This,M,It)])` for two target
  slots — both read `It`, the second is unnameable, R2 rejects it. Latently broken.

## The ruling

1. **Targets are a positional-only channel.** `Target(n)` reads the n-th slot;
   introduce `Targets(n)` (the plural twin the `core-anaphor-surface` sketch
   named but never built) to read a plural slot's full set. Nothing else reads
   `anaphora.targets`.
2. **Engine (`resolve/query.rs`):** delete the `It` lone-target fallback
   (`:405..425`) — `It` binds only `anaphora.it` (loop/`Where`/`Pick`), else it
   is an unbound read. Stop `They`/`Them` reading `anaphora.targets` (`:211..217`)
   — a plural target reads `Targets(n)`. After this, `It`/`That`/`They`/`Them`
   name only loop elements, `With` binders, and move/create products — never a
   raw target.
3. **Parser:** every targeted-body production emits `Target(n)`/`Targets(n)` at
   the source, indexed to the slot it declares (single-slot → `Target(0)`;
   multi-slot threads the index; `parse_damage_and_damage` → `Target(0)` /
   `Target(1)`). Delete `target_slot_reads`/`IT_READ` from
   `triggered_ability.rs` — with targets off the shared channel the trigger case
   needs no rewrite, so spell/activated/triggered/modal/loyalty all wrap the body
   verbatim.
4. **Surface/doc:** rewrite `target_spec.rs:6..15` — a target is read only
   positionally; drop the `It`/`That(Sort)`/`They` claims. Add a reject fixture:
   a bare `It`/`They` in a `Targeted` body with no loop/`With`/product antecedent
   is a load error, not a fizzle.
5. Mechanical canon/wizards rewrite: bare-`It` target reads → `Target(0)` (e.g.
   `plugins/wizards/cards/Whipcorder.ron.todo` `Tap(It)` → `Tap(Target(0))`),
   plural `They`/`Them` target reads → `Targets(0)`.

**Acceptance lens.** Each effect atom must resolve from its own arguments plus the
bound anaphora frame ALONE — no `if`/`match` that reads across its boundary
(target cardinality, whether a sibling binding exists, the parent frame's kind, or
the atom's own rendered output). The engine `It` fallback
(`no_other_singular && flat.len() == 1`, query.rs) and the parser's
`starts_with("SelectAll(")` re-parse are the archetype violations; the work is
done when no such cross-boundary branch decides a target read. See
[Effect atom independence](../../decisions/effect-atom-independence.md).

## Scope the first draft missed (review, each verified against the code)

Direction confirmed correct; these belong in the ruling before it is claimed:

6. **Idris model + soundness gate.** The antecedent-stack model also lives in
   `idris/src/Core.idr` (`bindTargets` pushes target slots onto the anaphor
   stack; `Site = TargetSlot`; R1/R2 candidacy includes them; `They`/`Them` are
   proof-carrying constructors over that stack), `idris/src/Spec.idr` pins the
   old behaviour as *positive* probes (an announced slot read back as an anaphor;
   the Arc-Trail shape; the Goblin-Medics ambiguity), and `idris/src/Cards.idr`
   reads targets as `It` in dozens of encodings. Do the engine ruling without the
   matching Idris rework — drop `TargetSlot` from anaphor candidacy, add
   `Targets n`, flip the Spec probes from positives to rejections, rewrite
   Cards.idr — and model and engine assert opposite things; the re-emit/idris
   gate then rejects the new `Target(0)` canon. Add `cargo xtask idris-check` to
   Verify. If a deliberate Rust/Idris divergence is ruled acceptable here, record
   that exception explicitly; the gate consequence must be
   resolved either way.)
7. **Renderer + emitter arms for `Targets(n)`.** `deckmaste_cards/src/render/effect.rs:1604`
   carries its own targets-as-`They` special case
   (`Binder::Existing(Selection::They) if ctx.targets.len() == 1`) that must
   migrate to `Targets(n)`, and `idris_emit.rs:1259` emits `They`/`Them` with no
   arm for the new variant. A new grammar needs its render and emit arms or the
   cards suite breaks — name them here,
   don't discover them mid-implementation.
8. **Decide: does `Target(n)` chase moves?** The `It` lone-target fallback chases
   the same-resolution move record (`chase_moved`, query.rs:424, [CR#400.7j]
   "exile IT … return IT"); `Reference::Target(n)` does *not* — it takes the
   first still-live slot member. So step 5's "mechanical `It → Target(0)`" is
   behaviour-changing for any body that moves its target and reads it again.
   Either give `Target(n)`/`Targets(n)` the chase (and spec `Targets(n)`'s
   per-slot live-filter / partial-fizzle, [CR#608.2b]) or route post-move reads
   through `That(Card)`. Pick one before the canon rewrite — this is a real fork,
   not a mechanical detail.
9. **Doc rewrite is wider than target_spec.rs.** `reference.rs:132-146` (`It`
   "reads its one announced target") and `:176-185` (`That` "pushed by target
   slots"), and `selection.rs:87-93` (`They` "pushed by a plural target slot")
   all teach the old model. Fix all three alongside `target_spec.rs`.
10. **Canon rewrite is scope-sensitive, not textual.** `plugins/canon/cards/Do or
    Die.ron` mixes a target-read `It` (`ControlledBy(Ref(It))`) with a loop-element
    `It` (`DestroyNoRegen(It)` inside a `Them(Pile)` iteration) in one ability —
    exactly the binder-scope analysis the `IT_READ` regex skips is what retiring
    it requires. Blast radius: ~1,529 of ~6,993 graduated
    `plugins/wizards/cards/*.ron` carry bare-`It` target reads (plus the todos);
    the engine deletion and the regen must land atomically or the corpus fizzles
    wholesale at runtime. Notes: `Targets` is kind-context-disambiguated from
    `Predicate::Targets` / `Count::TargetsOf` (no parse ambiguity); the reject
    fixture needs a home in `validate.rs`/the gate — today `unbound_ref`
    (query.rs:591) is a runtime null fizzle and nothing load-rejects a bare `It`.

**Critical** (this folder): a core resolution-model correction the whole anaphor
surface sits on — a live canon shape (Arc Lightning reads its targets as `They`),
a corruption-capable two-target trigger path (`parse_damage_and_damage`), and
every target read in the corpus depend on it. Overlaps
[[parse-target-hoisted-effect-macros]] (multi-reference emission). History: the
`Target(n)` sunset ([[cards-fidelity-target-sunset]]) that first pushed targets
onto anaphors was reversed; this finishes the reversal by giving targets their
own channel.

Verify: a two-single-target damage fixture ("~ deals 2 damage to target
creature and 1 damage to target creature") elaborates (was R2-rejected); an
Arc-Lightning-shape plural fixture reads `Targets(0)`; `cargo xtask generate
plugins/wizards` — Whipcorder emits `Target(0)`; `cargo xtask fidelity`
unchanged (positional read is the same object); `cargo xtask idris-check` green
(Spec probes flipped, Cards.idr reads `Target(0)`); the `deckmaste_cards` render
suite green (new `Targets(n)` arm); `cargo test --workspace`; `cargo xtask cite
check` — 0 stale, `--list-noncompliant` empty.
