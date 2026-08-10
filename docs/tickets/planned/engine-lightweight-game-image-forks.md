---
needs: [engine-payment-obligation-window]
---
Replace the payment window's naïve full `GameImage` clones with lightweight
forks only after the complete clone-based protocol is correct and measured.
Preserve exactly the same runnable surface: nested frames must still support
all engine queries and mutations, replacements, triggers, layers, history,
RNG, pending decisions, zone-change reminting, and deterministic replay.

Start with profiles and benchmarks covering nested filter-land/KCI payments and
selective rescind/decline replay. Share immutable card/registry data directly;
evaluate copy-on-write or persistent storage for mutable object/zones/player
state and chunked history. In particular, preserve SlotMap's generational-key
and allocator behavior—do not trade correctness for an object store that can
restore stale IDs or resolve them to different objects. The optimized image
must remain behaviorally interchangeable with the full-clone implementation,
with differential tests over the payment-window acceptance scenarios.

