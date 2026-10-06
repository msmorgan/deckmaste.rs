---
needs: [english-v3-generic-frame-consumption]
---
# Admit adjuncts inside object relative clauses

Pinned witness: **Seedborn Muse**, "Untap all permanents you control during each
other player's untap step." No current Reading places the `during` Preposition
Phrase inside the Object Relative Clause `you control …`.

The user's three-Reading ruling (2026-10-05) licenses attachment to `Untap …`,
to `control` inside the relative clause, and to the Nominal `permanents you
control`. The first and third already exist and must remain admitted. CGEL
Ch. 5 §14.2, p. 446 licenses temporal PPs as post-head Modifiers and stacked
modification; §15, p. 454 describes their essentially labile order. The nominal
attachment's semantic oddness concerns Preference, not Admission.

The `needs:` edge is sequencing only: grammar tickets are worked one at a time.
Reuse the existing clause-level Adjunct machinery rather than adding a
relative-clause-specific recipe. Add no guard naming a word or card.

Acceptance:

- Independently construct Reading 2, with the PP inside `you control …`, and
  establish both roundtrip laws.
- Un-ignore `seedborn_muse_retains_relative_clause_attachment` in
  `tests/reading_support.rs` and merge its assertion into the single Seedborn
  Muse test, so that test asserts all three attachments.
- Compare corpus Reading identities before and after on this ticket's own
  implementation tree, accounting for every addition and loss.

Standard constraints apply.
