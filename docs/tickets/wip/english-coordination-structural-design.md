---
needs: [english-coordination-residue, english-ability-output-retrofit]
design: true
---
**Close the remaining heterogeneous and member-scoped coordination design.**
This resumes only after the construction-output retrofit. The English tree is
not a legacy compatibility surface: it is the exact, compiler-owned
construction tree consumed through a typed spelling projection, while RON
definitions supply the more Magic-specific semantic macro layer. Do not
preserve serde field paths, legacy variant layouts, or flattened output merely
to keep old tests green. PMCFG is not a prerequisite unless a genuinely
discontinuous surface constituent remains after outputs carry shared context
explicitly.

Close the following coordination decisions on that new boundary; do not solve
them with surface-string gates or card-specific exceptions.

- Mixed keyword plus quoted-ability `with` lists (`with trample and "..."`),
  about 130 resisting faces. Keyword-only `with X and Y` is already typed, but
  the mixed form needs heterogeneous nominal/quoted coordination in a
  prepositional-complement position. The existing heterogeneous coordination
  is limited to verb objects.
- Asyndetic Oxford lists of three or more predicates whose members each carry
  a trailing `as long as` condition (Tek), plus the related shared-subject
  `A, B, then C` chains (about five Chaos Mutation faces). Decide whether the
  condition scopes inside each coordinated member before adding VP-level
  Oxford productions. Dominaria's Judgment's gapped verb remains separate
  ellipsis work.
- Sycorax Commander's or-coordinated appositive body (population three): a
  `then` chain inside one member plus a `minus one` nominal postmodifier. The
  quantity postmodifier is the costly half; existing appositive structure is
  already demonstrated by Midnight Crusader Shuttle and The Master.
- Mixed head-list versus common-head selection remains ambiguous. Synthetic
  control: `Put a +1/+1 counter on an Elf, Orc, or enchantment creature you
  control.` is best read as `an [[Elf], [Orc], or [enchantment creature]] [you
  control]`, but the same surface also supports the common-head analysis
  `an [[Elf], [Orc], or [enchantment] creature] [you control]`. Group-level
  relatives are now representable; the remaining work is a principled
  selection rule that preserves real common-head controls such as `Plains,
  Swamp, or Forest card` and `artifact and/or creature card`.
- The same unresolved boundary appears with separately modified heads: `a
  basic land card or Gate card` has a strong shared-article reading, but broad
  costs intended to select it also displaced established common-head parses.

Any eventual implementation must preserve n-ary coordination, typed member
attachments, source-free rendering, and the existing anti-overreach gates.

## Resolution

[`english-coordination-is-structural`](../../decisions/english-coordination-is-structural.md)
records the decision and the exact implementation boundary.

- The remaining members are contiguous after shared context is represented by
  its actual nominal, prepositional, finite-clause, or sentence owner. C01 and
  F04 stay on `ConstructionBackend::Chart`; no tuple-yield or PMCFG backend is
  justified.
- A coordination is homogeneous in grammatical role. A role may use a closed
  typed member sum, so mixed `with` lists gain a dedicated nominal/quoted-
  ability sum admitted only under `with`; the broad verb-object sum is not
  reused.
- Predicate attachments are built before member coordination. Tek's five
  trailing conditions are member-local, with a construction-local parallel-
  attachment guard selecting that reading for the final member. Chaos Mutation
  is one shared-subject asyndetic/`then` predicate layer. Sycorax Commander is
  an outer complete-clause `or` containing an inner shared-subject `then`
  layer.
- Common-head eligibility requires every modifier to carry the same typed
  attributive class and the trailing head to admit that class. This preserves
  uniform card-type, subtype, land-type, keyword-name, and adjectival controls
  while selecting a shared-determiner head list for the mixed subtype/card-type
  synthetic control. Repeated overt heads remain a head list.
- `that many cards minus one` is generated P01 arithmetic, not coordination;
  Dominaria's Judgment remains separate ellipsis work.

Direct probes of conditioned shared-subject predicates and nested `then`/`or`
clauses complete with zero recovery on the current one-span chart, establishing
that the apparent backend gap was an output-model gap. Supported Oracle
evidence includes Tek, Chaos Mutation, Sycorax Commander, Alien Invasion,
Basilica Shepherd, Blink, Abzan Monument, Deceptive Landscape, Grassland
Crusader, Open the Gates, and District Guide. The live registry also corrects
the stale inventory: 13 C01 plus six F04 rows remain handwritten; the F02 and
R01 coordinated-adjective consumers are already generated.
