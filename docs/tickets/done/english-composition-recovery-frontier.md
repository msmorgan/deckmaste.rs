---
needs: []
---
**32 synthesized compositions built entirely from individually-clean printed
fragments still fail to parse as a whole.** Every one clusters under a single
label, `SentenceBody::Recovered`. Each input piece to each composition parses
clean on its own — no recovered spans — before the composition operator
(`substitute`, `extend_coordination`, or `concatenate`) splices it in; only
the combination fails to parse. This is an unambiguous coverage gap: the
grammar accepts every part and cannot fully parse the whole.

Measured 2026-07-29 with `cargo xtask english adversarial`'s synthesized
sweep (the new adversarial-corpus tool, sibling `english-adversarial-corpus`
workspace, not yet integrated). Of 90 admitted compositions, 58 parsed clean
end to end and **32 recovered**, all 32 under `SentenceBody::Recovered` (20
distinct carriers by recipe).

Reproduce the whole census:
```
cargo run -q --release -p xtask -- english adversarial --per-rule 2
```
Reproduce a single case:
```
cargo run -q --release -p xtask -- english adversarial --text "<the text>"
```

Three verbatim examples, with recipe and donor cards from the census:

- **`Aang, Aang enters, and La attack`** — recipe: extend `and` coordination
  — donors: **Aang, Airbending Master**.
- **`you're dealt a graveyard damage`** — recipe: substitute `combat` with a
  rule-139 fragment — donors: **Melira, the Living Cure**.
- **`a 1/1 white Human creature token X 1/1 black Rat creature tokens`** —
  recipe: substitute `Create` with a rule-140 fragment — donors: **The
  Eleventh Hour**.

Every substring in each example is attested verbatim printed text; only the
arrangement is synthesized. Standard constraints apply.

## Completion

- Audited all 32 compositions and found every output grammatically malformed; no parser widening landed, and category/agreement-safe admission remains with the adversarial-instrument ticket.
