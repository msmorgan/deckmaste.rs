---
needs: [english-v2-subordinate-clause, english-v2-lexeme-owned-verb-frames]
---
Relative clauses (A5): a relative-marker slot {that, who, zero} feeding ONE
general relative body (subject-gap and object-gap as feature values). "who"
has 143 corpus witnesses and no v2 path; delete hardcoded "that" from
predicate constructions. Recorded shared gaps, not required: which/whose,
pied-piping.

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.

## Re-coverage obligation (2026-09-04)

`english-v2-adjunct-licence-removal` retired **Absorbing Man and Titania**
(`72b69a69a8a4add9165cd6a8f5b062801ad06247f3c48e126189904cdbac155d`) from the
coverage lock. The sentence is:

> Double all damage that creature sources you control would deal.

It had been covered only by a wrong analysis: `fixed_duration_phrase` accepted
any noun phrase as a temporal endpoint, so `that creature sources you control`
was absorbed as a duration adjunct. Narrowing the endpoint to a declared
temporal head removed that reading and left the unit with no parse.

The analysis that must select when this ticket lands: `that creature sources
you control would deal` is an **object-gap relative clause on `all damage`**
whose body carries an auxiliary — subject `creature sources you control`,
auxiliary `would`, head verb `deal`, object gap. The general relative body
this ticket introduces has to admit an auxiliary between the relative subject
and the head verb.
