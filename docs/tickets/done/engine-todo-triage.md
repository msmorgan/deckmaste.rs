---
needs: []
---
**Classify the engine's `todo!()`/`unimplemented!()` population and turn it
into an explicit failure policy.** There are 76 grep hits in
`crates/deckmaste_engine/src` as of 2026-08-03; regenerate at claim time and
separate production sites from inline-test fixtures. Sixteen are the old
`todo!("P0.…")` SEAM backlog named in the tickets README.

The inventory must distinguish four cases that require different behavior:

1. **Malformed semantic reference or missing semantic object.** Fizzle only
   when the site falls under `docs/decisions/invalid-semantic-input-fizzles.md`:
   resolve to no applicable object, emit no game facts, and retain a
   diagnostic suitable for validation/logging.
2. **Valid but unsupported Magic mechanic.** Keep a loud, mechanic-specific
   diagnostic until its named implementation ticket lands. Do not silently
   turn a legal card behavior into a no-op.
3. **Internal engine invariant.** Keep or strengthen the assertion/panic and
   state the invariant it protects; these are explicitly outside the fizzle
   decision.
4. **Small implementation.** Mint a focused ticket, or record why a genuinely
   atomic correction is better handled ad hoc. Do not let opportunistic
   implementation make this inventory unbounded.

Examples to classify include `legal.rs:89` (unevaluated deontic legality),
`target.rs:409` (phased-in status), and `replace.rs:142` (uninterpreted
enters-replacement effect). Existing tickets such as
`review-low-severity-followups` and `engine-unbound-eventobject-panics` retain
ownership of their named instances; link rather than duplicate them.

Deliverables:

- a complete table in this ticket's eventual done record: location,
  reachability, class, governing decision/rule, and owner ticket;
- focused child tickets for every class-2 site and every non-ad-hoc class-4
  site, with no anonymous `P0.Wn` owner left;
- one engine-crate policy paragraph distinguishing invalid-semantic-input fizzle,
  unsupported mechanics, and invariant failure; and
- a final grep proving every remaining production `todo!`/`unimplemented!`
  has a durable diagnostic and a ticket slug.

This ticket performs classification and policy work, not a bulk behavior
rewrite.

---

## Closeout (2026-08-06)

### Finding: the `P0.Wn` population is uniformly class 2

All 23 production sites carrying a `P0.Wn` tag classify as **class 2** —
valid Magic mechanic, not built yet. None was a class-1 invalid-semantic-input
fizzle, a class-3 invariant, or a class-4 atomic correction. That uniformity is
why a single anonymous wave tag survived across seven waves and eleven files:
it was never encoding a *kind* of failure, only a scheduling batch. Once the
waves stopped being tracked, the tag stopped carrying any information at all.

The naive inventory grep undercounts. `grep 'todo!\|unimplemented!' | grep
'P0\.W'` finds 15 of the 23 — the other eight carry the tag on the message's
continuation line, not the macro line.

### Classification table

Every row is class 2. "Governing rule" is blank where no specific rule governs
the seam (generic dispatch-shape gaps and strategy fallbacks) — a citation was
deliberately omitted rather than stretched.

| Site | Unbuilt mechanic | Governing rule | Owner ticket |
|---|---|---|---|
| `cast.rs` ×3 | non-mana cost components in a cost-change pass | [CR#601.2f] | `engine-cost-modification-residue` (new) |
| `legal.rs` | deontic legality outside the Cast arm | [CR#101.2,601.3] | `engine-deontic-legality-residue` (new) |
| `render.rs` | `Action::Special` render shape | [CR#116.2] | `engine-special-actions` (new) |
| `decide/mod.rs` | `Action::Special` submission | [CR#116.2] | `engine-special-actions` (new) |
| `decide/mod.rs` | multiplayer leave-game cleanup | [CR#800.4a] | `engine-multiplayer-leave-game` |
| `resolve/player_action.rs` | restarting the game | [CR#727.1] | `engine-restart-game` |
| `sim.rs` ×2 | strategy + player lookup for unwired decision kinds | — | `engine-shell-decision-strategies` (new) |
| `resolve/effect.rs` ×3 | `ForThisEvent` riders beyond `Cant(Regenerate)` | [CR#701.19c] | `engine-forthisevent-riders` (new) |
| `resolve/effect.rs` ×3 | granted `Continuously` static-row kinds | — | `engine-granted-static-rows` (new) |
| `resolve/effect.rs` | granted continuous prevention row | [CR#615.1] | `engine-granted-prevention-rows` (new) |
| `decide/pending/choice.rs` | dividing an effect among targets | [CR#601.2d] | `engine-divided-distribution-as-you-choose` |
| `decide/pending/choice.rs` | voting | [CR#701.38a] | `engine-voting-procedure` (new) |
| `decide/pending/choice.rs` | ordering applicable replacements | [CR#616.1] | `engine-order-replacements-decision` (new) |
| `decide/pending/choice.rs` | the pre-game procedure | [CR#103] | `engine-pregame-procedure` (new) |
| `target.rs`, `trigger.rs` ×2 | unevaluated permanent status | [CR#702.26a,708.1,710.3] | split: `engine-phasing` / `engine-face-down` / `engine-flipped-status` (new) |

Reachability note: `Division`, `Vote`, `OrderReplacements`, and `PreGame` are
*shaped but unbuilt* — `strategy.rs` can answer them and `decide/mod.rs`
dispatches them, but no code path constructs one. `Action::Special` is the same
shape: `legal_actions` never enumerates it. Implementing their handlers here
would have produced unreachable, untestable code, so they were classified and
ticketed rather than written.

### Discrepancy: an orphaned seam, not a closed one

`resolve/effect.rs`'s granted-`Continuously(Prevention)` arm claimed
`engine-prevention` owned it. That ticket closed 2026-07-16 covering only the
one-shot `PreventNext` / `PreventAll` / `CantPrevent` primitives that shipped in
`replace_registry.rs`; it never unblocked the granted-continuous arm. The seam
was orphaned. `engine-granted-prevention-rows` now owns it, and the pinned
`should_panic` test's expectation moved off the stale slug.

### Child tickets minted

`engine-cost-modification-residue`, `engine-deontic-legality-residue`,
`engine-flipped-status`, `engine-forthisevent-riders`,
`engine-granted-prevention-rows`, `engine-granted-static-rows`,
`engine-order-replacements-decision`, `engine-pregame-procedure`,
`engine-shell-decision-strategies`, `engine-special-actions`,
`engine-voting-procedure`.

`engine-cost-modification-residue` exists because `engine-alt-costs` does *not*
cover these sites: alt-costs owns choosing an alternative cost at announcement,
a different lane from modifying an already-determined total.

### Policy

The engine-crate failure policy is now a `# Failure policy` section in
`crates/deckmaste_engine/src/lib.rs`, distinguishing invalid-semantic-input
fizzle (per `docs/decisions/invalid-semantic-input-fizzles.md`), unsupported
mechanic (loud, mechanic-named, `owner: <slug>`), and internal invariant.

Diagnostic format, propagated from the convention already in
`resolve/action.rs` — `engine seam:`, the mechanic, a bracketed rule citation
where a specific rule governs, what is unbuilt, then `; owner:` and the slug.
A real instance, from `resolve/effect.rs`:

    todo!("engine seam: Continuously(Prevention) ([CR#615.1]) — granted \
           prevention shields/windows unbuilt; owner: engine-granted-prevention-rows")

The citation is omitted, not stretched, where no single rule governs the seam
(generic dispatch-shape gaps, strategy fallbacks).

### Second pass: owners for the rest of the panic population

The `P0.Wn` sites were the loudest but not the whole population. A second sweep
went over every other production `todo!`/`unimplemented!` in the crate and
appended `; owner: <slug>` wherever an EXISTING ticket clearly owned the site.
No tickets were minted in that pass — a site with no confident owner was left
untouched and listed below, deliberately, so a human scopes it rather than
inheriting an over-eager wrong owner.

Verified at close: 63 real macro invocations in the crate (excluding mentions
inside comments), of which 51 name an owner and 12 do not. Those 12 are the
`engine-panic-owner-residue` list.

**Owners that are `done/` tickets.** Three assignments point a live seam at a
closed ticket: `engine-filter-breadth` (several `target.rs`/`trigger.rs`
frameless-reference and snapshot arms), `engine-turn-modification`
(`resolve/action.rs` extra phases), and `engine-find-moved-object`
(`resolve/effect.rs` `Binder::Produce` over a non-`Move` action). That is the
same shape as the `engine-prevention` discrepancy above and deserves the same
check: confirm each done ticket documents the residue as intentional, or the
seam is orphaned and needs a fresh owner.

### Still unowned — handed to `engine-panic-owner-residue`

Each is a production panic with a durable diagnostic but no ticket. They are
tabulated with scoping notes in `engine-panic-owner-residue`, which also carries
the done-ticket-owner check above:

- `decide/pending/cast.rs` — a `Must(Target)` row matching a placing trigger's
  source; the `by` filter can't tell a spell from a triggered ability.
- `replace.rs` — the enters-replacement fold catch-all beyond self-count/`If`.
- `resolve/query.rs` — `Selection::PilesOf`; the code names `engine-piles`, but
  no such ticket exists.
- `condition.rs` — `Condition::Crossed` with no before/after channel in frame.
- `activate.rs`, `resolve/effect.rs` ×2 — `Binder::Produce`/`Search`/`SearchOne`
  unwired at runtime. `parse-tutor-search` (done) covered only the parser side.
- `sba.rs` — an SBA's own effect supports only `Act`/`Sequentially`.
- `resolve/effect.rs` ×2 — `Simultaneously` members restricted to pure verbs.
- `resolve/effect.rs`, `trigger.rs` — the stage-3 interpreter and snapshot-filter
  catch-alls; both span many variants and need per-variant scoping first.

### Not done here

`comment-discipline-sweep` keeps its remaining scope — `Task N` references, the
broad "seam" mentions, and other crates (two `P0.Wn` comments survive in
`crates/deckmaste_plugin/`). What this ticket did take: the 17 `P0.Wn` comment
references inside `deckmaste_engine`, plus two bare `W3`/`W5` wave references.
Leaving a retired scheduling tag in the same crate as the new failure policy
would have been incoherent. `crates/deckmaste_engine/src` is now free of wave
tags entirely.
