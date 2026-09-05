---
needs: []
---
**Keyword-action verbs contribute their full finite paradigm.** A keyword-action
verb declaration whose `third_person` is authored `Unavailable` contributes **no
verb reading at all** — not a bare one either. Verified in the
`english-v2-lexical-inventory-2026-09-05` review: `Goad target creature.` and
`Detain target creature.` fail at the verb span, while `Destroy target
creature.` (no `Unavailable`) selects; and with the `Exert` core-verb workaround
renamed away, `{T}, Exert this creature: Draw a card.` reports
`outcome status=parse_failure span_start=5 span_end=10`.

Minted under the coordinator ruling of 2026-09-05 (Q2). The ruling is that this
is not a data gap but a **ruling violation**: `Unavailable` withholds a
grammatical English inflection because the form is unattested in AtomicCards,
and that is attestation used as a filter. Per the rewrite ADR amendment of
2026-09-04, attestation is provenance, never a filter.

What to build:

- Every keyword-action verb declaration contributes the full finite paradigm
  derived by the compiler's morphology — third person, preterite, participle —
  with `DerivedSurface::Unavailable` **retired as a licence** on verb
  inflections.
- A form's absence from AtomicCards is recorded as provenance in the landing
  report, never as a gate and never as a reason to withhold the row.
- The seventeen stubs authored `third_person: Unavailable` today, all under
  `plugins/builtin_v2/macros/stubs/keyword_actions/`: `Abandon`, `Behold`,
  `Bolster`, `CollectEvidence`, `Detain`, `Exert`, `Fateseal`, `Forage`, `Goad`,
  `Harness`, `Learn`, `Monstrosity`, `Populate`, `Recruit`, `Support`,
  `Suspect`, `TimeTravel`. Each carries a morphology-audit comment recording the
  absence; those comments become provenance notes, not licences.
- No second owner for a lexeme a declaration already owns. In particular
  `exert` stays owned by `keyword_actions/Exert.ron`; the retired
  `CoreVerbIdentity::Exert` row must not come back.

Expected effect: the **6** corpus identities the retired `exert` lexical row was
covering (`{T}, Exert this creature:` activation-cost clauses on Hope Tender,
Basri Tomorrow's Champion, Oasis Ritualist, Steward of Solidarity, Pride
Sovereign, Fervent Paincaster) return through the declaration, plus whatever the
other sixteen stubs' verbs reach. Exert is `[CR#701.43a]` ("To exert a
permanent, you choose to have it not untap during your next untap step"), rule
number verified against `data/rules/cr.txt` at mint time.

Tier: **sol**. Standard constraints apply.
