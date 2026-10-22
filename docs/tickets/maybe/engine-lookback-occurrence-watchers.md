---
needs: [engine-lki-robustness]
---
**[design] Engine: look-back triggers see the whole Occurrence's pre-event
watchers, so an Object that leaves alongside others triggers for each of them.**
A design change to how look-back watchers are assembled (settled 2026-10-21: not
a patch). Claiming this opens a design dialogue with the user.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #3.

## What is wrong at head

`crates/deckmaste_engine/src/trigger.rs`:

- `scan_triggers` (`:614-666`) loops over an `Occurrence::Batch` and calls
  `scan_event` once per fact.
- `scan_event` (`:971-1022`) rebuilds the watcher set per fact: live
  battlefield, graveyards, hands, command-zone emblems, plus one
  `Watcher::Leaving(snapshot)` for THIS fact's leaver only
  (`if let GameEvent::ZoneChange { snapshot: Some(snapshot), from:
  Some(Zone::Battlefield), .. }`). A co-leaver is by then a `Watcher::Live` in its
  graveyard, and the function-zone gate (`watcher_zone != Some(t.from…)`, around
  `:1074`) drops its battlefield trigger.

So when Blood Artist and two other creatures die to one destroy-all, Blood
Artist triggers once (for itself) instead of three times.

[CR#603.10]: look-back abilities use "the existence of those abilities and the
appearance of objects immediately prior to the event". The [CR#603.10a] example:
"The artifact's ability triggers twice, even though the artifact goes to its
owner's graveyard at the same time as the creatures." Blood Artist's ruling
(2016-06-08): "its ability will trigger for each of those creatures."

## Direction (verifier-corrected; the design dialogue settles the shape)

Collect a `Watcher::Leaving` snapshot once per Occurrence for every member that
left the battlefield. For each fact, scan the live watchers, the fact's own
leaver, and the co-leavers' snapshots, deduplicating the leaver's own snapshot.
A co-leaver's snapshot matches only the look-back trigger classes
([CR#603.10a..603.10f]): leaves-the-battlefield and dies, sacrifice, leaving a
graveyard, a visible object put into a hand or library, phasing out, becoming
unattached, control loss, a spell being countered, a player losing the game. It
must not match ordinary post-event triggers ("whenever a creature enters").

Questions for the dialogue:

1. Whether the scan should instead build one pre-event watcher image per
   Occurrence (abilities as they existed immediately prior, [CR#603.10]) and use
   it for every look-back class, which also covers an object that ENTERED in the
   same batch (it did not exist before the event, so it must not see a look-back
   event) and abilities removed by the event itself.
2. How the look-back class of a trigger pattern is decided: per `EventFilter`
   shape, or a declared flag on the trigger's frame.
3. How this interacts with `batch_fired` (`OneOrMore` once per Occurrence,
   [CR#603.2c]) and the `Blocked` dedup in the same loop
   (`engine-combat-block-status` changes that dedup).

Needs `engine-lki-robustness`: the co-leaver snapshots must carry derived
characteristics and abilities for the match to be right.

## Tests

- Blood Artist and two other creatures die to one destroy-all: three triggers.
- The [CR#603.10a] example: a "whenever a creature dies, you gain 1 life"
  artifact destroyed with two creatures triggers twice.
- A co-leaver's snapshot does not fire its ordinary (non-look-back) triggers,
  such as "whenever a creature enters", for another member of the batch.
