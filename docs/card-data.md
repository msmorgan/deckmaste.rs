# Card data policy

2026-06-10. What MTG-derived data this repo commits, and what stays local.

## Committed: a curated canon (~100–200 cards)

`plugins/canon/` holds real cards — names plus structured behavior encodings —
and is tracked in version control. The target is roughly 100–200 cards total,
curated as a nearly complete slice of MTG mechanics: a card earns its slot by
exercising a mechanic, interaction, or grammar shape, not by being famous.
Canon is the durable, reviewable proof that the grammar and engine handle the
real game; bulk coverage lives in the local, generated pipeline output
(`plugins/wizards/`, gitignored).

Curation is deliberate: batch canon additions and run them by the user.
Growth should track mechanics landing.

## Hand-written entries are legitimate

Some cards are unparseable without a 1-of-1 parser production. Don't build
those productions — writing the encoding by hand is better than pretending
we're not. Mark hand-written canon entries with a comment in the `.ron` so
parser-coverage claims stay honest. The parsers' job is the broad corpus;
canon's job is correctness.

## Still never committed

- `/data/` — CR text (`cr.txt`/`cr.json`), MTGJSON snapshots, catalogs.
  Fetched locally via `scripts/fetch_data.fish`.
- `plugins/wizards/` — generated full-corpus pipeline output; rebuild with
  `cargo xtask generate plugins/wizards`.
- Oracle text, flavor text, and art, in any form. Canon encodes behavior,
  not text; tools persist only derived artifacts (rule numbers, checksums,
  URLs — see the CR citation lockfile).

Rationale: game rules and mechanics are not copyrightable; card names, text,
and art are WOTC's. A small, purposeful slice of name+behavior encodings
(cf. XMage, Forge) is the deliberate trade.

## Engine tests

Engine tests use real cards from `plugins/canon`. A fake belongs in
`plugins/testing` ONLY when the behavior under test exists in no real card —
verified against the full corpus (see that plugin's `cards/README.md` for the
current residents and why each combo cannot be real). When a mechanic lands
that makes a mock encodable with a real card, canonize the real card and
delete the mock.
