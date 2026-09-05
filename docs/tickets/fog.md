# Fog

Work that is **in scope but not yet sharp enough to be a ticket**. A fog entry
records the substance — the family, the shape it should take, the identities and
probes it owns, and the open decision it waits on — so that nothing is lost while
the shape is still unsettled.

This file is **not a graph source**. The Kanban driver reads only the status
folders (`critical/`, `planned/`, `maybe/`, `wip/`, `done/`); like `census.md`,
nothing here is machine-read and no `needs:` may name a fog entry.

**Graduation rule.** An entry becomes a ticket only when it can carry a
`Pinned shape:` — a named method a claimant can execute without re-deciding the
design. Write the ticket, then **delete the entry**; the two never coexist. An
entry's `Hangs on:` line names the live ticket(s) whose landing is expected to
sharpen it.

## english-v2

Seven entries below. Six are families demoted from `planned/` on 2026-09-03:
they were pinned as a nine-deep serial chain of near-identical tickets whose
bodies all deferred their design to the same audit, which is not a pinned shape.
The seventh records gaps neither grammar ever had.

The design source for the six is the Plan 09 v1/v2 taxonomy audit (its "v1
taxonomy decisions v2 should ADOPT" table, rows A6–A11). Each entry restates
that row's shape here so a fresh checkout needs nothing else. The probe set for
each is that audit's 25-sentence probe table filtered to the family.

### Gerund clauses and modal ellipsis (A9, A11)

Two nonfinite/reduced clause shapes v2 has no path for. **A11:** a gerund clause
category (`instead of putting it…`, `by replacing…`, `rather than paying…`) —
CGEL lists gerund-participials alongside infinitivals as a nonfinite clause type;
v1 had the category with few productions, and its replacement-effect corpus rows
were still recovered through it. v2 has zero gerund constructions. **A9:** a modal
with an optional inner predicate, licensing post-auxiliary VP-ellipsis (`If you
can't, …`); v1 carried it, v2 was not found to — verify with a probe before
pinning.

Re-coverage targets routed here from `english-v2-control-role-declared-valence`,
which retires them because their sole surviving reading is wrong: **Cavalier of
Thorns, Animal Magnetism, Genesis Ultimatum** — `Put X onto the battlefield and
the rest into your graveyard`, a gapped coordinated destination that is not
expressible today. They are re-covered when this entry graduates and lands.
The Cavalier of Thorns sentence is `Put a land card from among them onto the
battlefield and the rest into your graveyard.`
(3a05d72cc35c86f1b6bb8cfb74017af4c6c5b440e9b18475485ad034db4440e6).
The Animal Magnetism sentence is `Put that card onto the battlefield and the
rest into your graveyard.`
(83b1cc666255a03888d2654849cbdcd552c61d9336ce41cbe163a248c4f887ec).
The Genesis Ultimatum sentence is `Put any number of permanent cards from among
them onto the battlefield and the rest into your hand.`
(cc39ba9efc13bba3022d314e135707a35dbe17b515db2cbd7ca858a97cff00b2).

Hangs on: `english-v2-relative-clause` (and the general clause machinery under it).

### Predicative complement as one copular frame (A7)

Predicative complement is a **function filled by AdjP | NP | PP**, not a family of
game-carved subtypes: color, designation, status and face orientation are lexical
classes of adjective or noun. v2 partitions 8 of its 9 predicative-complement
subtypes by game meaning. Adopt one copular frame and delete the partition — the
frame-literal ruling's spirit applied to the copula. No ruling blocks it.

`english-v2-cost-family-lowering` names the predicative block as owned elsewhere;
that pointer is this entry until it graduates.

Hangs on: `english-v2-subordinate-clause` (the general clause and frame shape it
would be spelled against).

### Recipient / retained-object passive as frame data (A6)

The passive of `deal X damage to Y` promotes the recipient and retains the theme
(`were dealt damage`), plus its reduced form (`a creature dealt damage this way`).
That is a **valence fact of the verb** — frame data on the declared valence, on the
`Custom`/`Participle` axis — never a construction per verb. v1 carried it as a
`recipient_passive` verb property with a reduced-passive phrase; v2 fails the probe
at `dealt`. The audit expects it to become natural once `damage` is an ordinary NP.

Hangs on: `english-v2-frame-selected-prepositions` (same seam — lexical facts moved
into declared Verb Frame data).

### Arithmetic and fraction values (A10)

One general quantity grammar for arithmetic value expressions: `half their life,
rounded up`, `twice X`, `X plus Y`, `X minus Y`. v1 had it; v2 fails the probe at
`half`. No ruling and no mechanism blocks it.

Hangs on: **nothing structural.** This one is fogged only because its inventory has
never been measured — it needs a corpus census of the arithmetic surfaces and their
rounding forms to pin a shape, not another ticket's landing. It can graduate as soon
as someone does that census.

### Polarity as a derived feature (A8a)

`non-` is productive prefixation and `not`/`no` negation is a feature — both belong
on the general modifier/nominal/predicate constructions, not in per-polarity
construction families. v1 carried a `Polarity` axis and a negated-modifier lexical
slot; v2 spends 12 constructions on `non-`. Collapsing them deletes those families.
Mechanism note from the audit: v2's per-family `declaration_noun` codecs would need
one codec parameterised by family — a mechanism change, not a ruling.

Hangs on: the noun-feature work; `english-v2-number-feature-unification` is the
precedent for a feature collapse (categories duplicating a feature axis get deleted
in favour of the declared feature) and the shape this should be spelled against.

### Card type as a derived feature on nominal modifiers (A8b)

24 of v2's 34 nominal modifiers are per-card-type constructions (8 types × 3
layers); card type is a lexical subclass, so collapse them to **one modifier
construction with the type as a declared feature**. Scope fence, still binding: the
ADR-blessed 34-construction card-kind **noun** partition (`english-v2-rewrite.md`,
~:547-551) is a separate kept decision and stays out of it — this is the modifier
side only. Same `declaration_noun` codec-parameterisation note as A8a.

`english-v2-adjective-inventory` previously waited on this as a ticket; it no longer
does. 2026-09-04: its three form-literal re-routes
(`AttributiveAdjective::{Additional,Next,Other}`) are NOT handled by
`english-v2-closed-class-single-owner` (done; it did not touch them) — the
adjective ticket's own "Hard blocker this ticket owns" is accurate and owns
them.

Hangs on: the noun-feature work; `english-v2-number-feature-unification` as the
feature-collapse precedent.

### Tail families (the frontier register for `english-v2-stage-5-grammar-buildout-14-10`)

The umbrella ticket is never claimed directly; each iteration the coordinator
re-runs the failure census, takes the largest remaining family, mints one
`english-v2-tail-<family>` ticket sized to that family, and deletes the family
from this list when the ticket lands. Counts go stale with every landing:
re-measure before minting, and stamp the measurement. No xtask command exposes
a family key yet; the census below bucketed parse failures by first-failure
byte offset over the `coverage` rows.

Measured 2026-09-05 on change `osnrsxuvkrwo` (19,198 / 32,641 covered, 13,443
parse failures). Key: the first-failure byte offset carried in each
`parse_failure` row's `message` ("parse failed at bytes N..M"), bucketed by the
token at that offset and the token before it. 534 rows carry no offset (build
rejections) and are unbucketed. A unit is a whole card face, so a family's count
is the units it is the FIRST failure of, not the units that contain its surface.

**Reading a mid-word offset.** An offset that lands inside an orthographic word
(`color|less`, `Other|wise`, `a|ddition`, `land|cycling`) is an ARTIFACT, not a
scanner split. `has_lexical_boundary`
(`crates/deckmaste_english_v2/src/parser/scan.rs:1184`) does enforce a right
word boundary; a prefix reading survives only when an adjacency-marked
continuation suppresses it, and that branch then dies without affecting the
outcome. The real gap in every such case is that no lexeme spans the whole
word. Verified on `colorless` 2026-09-05; the previous register's
"scanner splits `color` + `less`" wording was a wrong diagnosis and is retired
with it. Do not mint a scanner ticket on this evidence.

Top ten by first-failure attribution:

- `as` not licensed as a general preposition or comparator: 437 — OWNED by
  `english-v2-remaining-prepositions`, which names `as` as its hardest member
  and splits the `as long as` / `as though` subordinator sites explicitly
- `colorless`: 387 — minted 2026-09-05 as
  `english-v2-tail-color-property-adjectives`. One absent member of the
  colour-property class ([CR#105.2c]); the class is complete at eight
- `if … was kicked` / kicker conditionals: 194 — UNOWNED. No `Kick` verb is
  declared anywhere (`core_verbs.ron` has no entry), while
  `predicative_declared_participle` (`verb(head)`, bare) is already the right
  standalone host — a lexeme-data gap, not a seam
- restrictive `only … each turn`: 191 — OWNED by
  `english-v2-frequency-adverbial-family`
- `in addition to its other types`: 169 — UNOWNED, and double-blocked:
  `addition` is absent from the closed `CommonNoun` inventory, and
  `nominal_preposition_is_licensed` refuses every `to`-postmodifier outright —
  the second half is `english-v2-locative-licence-set`'s pinned defect
- copula contraction + `still` + predicate (`It's still a land.`): 115 — UNOWNED
- elliptical `If you don't, …`: 109 — fogged above under "Gerund clauses and
  modal ellipsis (A9, A11)"; a fog entry is not a ticket
- sentence-initial `Otherwise,`: 84 — UNOWNED, named nowhere
- `until you <verb>` finite subordinate clause: 84 — OWNED by
  `english-v2-subordinate-clause`
- bare `X` as a card-count quantifier (`the top X cards`): 80 — UNOWNED

Not a family, do not mint: the bare `.` fragment accounts for 653 failures
spread over 101 distinct preceding tokens (`turn` 163, `time` 48, `top` 41,
`cards` 35, `card` 34, `control` 29, … down to singletons). It is the generic
end-of-sentence position, not one phenomenon.

Routed residues:

- `vocab Color` is misnamed: [CR#105.4] states that multicolored and colorless
  are not colours, and the class already holds *monocolored* and *multicolored*.
  `english-v2-tail-color-property-adjectives` sharpens this and routes the
  rename to be minted at its landing.
- coordination families generally: v1's coordination modules are a phenomenon
  checklist (coordinable categories, serial-list comma conventions, and/or/nor,
  scope, agreement), never code or vocabulary to import.

Hangs on: nothing; this is the live frontier.

### Shared gaps (neither v1 nor v2 covers)

Recorded for the record, attributed to neither grammar — no ticket owns these, and
`english-v2-relative-clause` explicitly does not require the relative ones:

- `which` / `whose`
- pied-piping
- non-restrictive relatives
- finite `that`-complement clauses (and wh-complement clauses)
- correlative coordination (`both…and`, `neither…nor`, `either…or`), and `but`/`nor`
- gapping / right-node raising
- `the former` / `the latter`
- reminder text
- AdvP with degree modification

Each becomes a ticket only on evidence — a corpus witness count plus a pinned
construction shape. Evidence decides WHEN a gap is scheduled, never whether a
general construction may admit it: attestation is provenance, not a filter
(rewrite ADR), so a landed construction admits its full linguistic domain
regardless of this register's counts (clarified 2026-09-04). Adding one to a general construction that already exists (a
relative-marker slot, a coordination) is the cheap path; none is a family of its own.
