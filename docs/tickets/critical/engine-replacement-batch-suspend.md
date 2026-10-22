---
needs: []
---
**Engine: a batch whose members each need a replacement choice loses no events,
and the replacement choice offers only the first non-empty [CR#616.1] tier.**
Part 2 is reachable: floating shields are keyed per instance
(`ReplacementKey::Floating(InstanceId)`, `replace_registry.rs:255`), so two
creatures that each hold two regeneration shields, destroyed in one SBA batch
(`Occurrence::Batch`, `sba.rs:649`), lose events.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #9.

## What is wrong at head (crates/deckmaste_engine/src)

1. **No tiers.** `replace_registry.rs:676-683` (`replace_event`, the `_ =>` arm)
   calls `surface_choice(state, current, applied, &applicable)` with every
   applicable key. [CR#616.1a]: "If any of the replacement and/or prevention
   effects are self-replacement effects …, one of them must be
   chosen"; [CR#616.1b..616.1d] give the control-on-entry, enter-as-copy and
   back-face-up tiers; [CR#616.1e] only then "Any of the applicable … may be
   chosen". Latent: none of those kinds is implemented yet.
2. **Lost tail.** `surface_choice` writes `remaining: vec![]` (`:927`). In
   `resume_replacements`' tail loop (`:983-1004`), a second Suspend does `// we
   don't have the tail here, so just return … return;`: the rest of
   `rs.remaining` is dropped, and so are the just-resolved member's `facts`, which
   are emitted only after the loop (`:1007-1015`). Tail members that pass are each
   scheduled as their own `Occurrence::Single` via `schedule_front` (`:988-991`),
   in reversed order, and `step/mod.rs:899-927` commits the earlier members as a
   separate partial batch. [CR#603.2c]: one event may contain multiple
   occurrences, and an ability "can trigger repeatedly" for them; splitting the
   batch changes `OneOrMore` matching.

## Change

1. Before `surface_choice`, filter `applicable` to the first non-empty tier
   ([CR#616.1a]→[CR#616.1d], else [CR#616.1e]); auto-apply when one remains. Tag
   each replacement kind with its tier as those kinds land.
2. Make `ReplaceState` own the unprocessed tail, the facts already resolved and the
   members that passed. On a mid-tail Suspend, carry all of it into the new state.
   When the last choice resolves, commit every member as one `Occurrence::Batch`,
   in member order.

Not in scope here: [CR#616.1] asks simultaneous choosers to choose in APNAP order;
note it where the choice is surfaced, ticket it if it needs work.

## Tests

- Two creatures, each with two regeneration shields, destroyed by one destroy-all:
  both are regenerated (both choices surfaced) and a "whenever one or more
  creatures …" trigger sees one occurrence.
- A self-replacement and an ordinary replacement both apply: the self-replacement
  is applied without a choice.
