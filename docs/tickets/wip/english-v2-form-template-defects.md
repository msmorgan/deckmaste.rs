---
needs: []
---
# Four one-line narrowings in forms and roles

**R5 — Group R.** Four independent defects, one ticket because each is a single
line and none needs a design decision. Each is a value-keyed form or a role
optionality that froze one attested spelling as a rule.

1. **`edge_of_phrase` loses `the` on `Top`**
   (`crates/deckmaste_english_v2/src/constructions.rs:3615`):
   `form top when position is Top = lex(position) lex(relation) whole;` beside
   `form bottom otherwise = "the" lex(position) lex(relation) whole;`. So
   `from top of your library` parses, **`from the top of your library` does
   not**, and `on the bottom of your library` does. This is a transcription
   error frozen by a value-keyed form, not a grammar rule.
2. **`passive_movement_predicate` makes the source mandatory** (`:1785`):
   `source: FrameComplement` is required, so
   `Whenever a permanent an opponent controls is put into a graveyard, …` fails
   and only the attested `put into X from Y` shape is admitted. The role is
   `opt`.
3. **`bare_and_predicate_coordination` refuses a two-member `, and`** (`:1662`):
   the positional separator table sets `pair = " and "`, so
   `…exile them in a face-down pile, and shuffle that pile.` is refused. Oracle
   spells both; the pair position admits both surfaces.
4. **`codec EnterWithCountersVerb` confines a general frame to one verb**
   (`:715`): the `V with ‹obj› ‹prep› ‹obj›` tail is *enter*-only, so
   `…exile that card with a dream counter on it…` fails for every counter kind.
   The frame belongs to any verb that declares it.

Pinned shape. Fix each in place; each is independently landable and independently
measurable. Item 4 is the smallest instance of the pattern
`english-v2-lexeme-owned-verb-frames` generalizes — do it here only as far as
letting the existing codec serve more than one verb; do not start the frame
redesign in this ticket.

Fences. Adding a fifth form arm instead of removing a value key. A `checked by`
naming `top`, `bottom`, `enter`, or any card. Any census used as a gate.

Glossary: Form, Linearization, Complement, Verb Frame, Coordination. Record any
gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.

## Landing record

**STOP — no production change retained.** Start: `2026-09-04T15:03:46-07:00`.
The refreshed-tree coverage probe disproved the proposed `Top` replacement:
adding `"the"` to the `Top` form removes 128 previously covered corpus
identities and adds 11. This is a ticket-versus-ruling contradiction: the Group
R ruling in `rulings/no-relayering-during-group-r.md` says that grammatical or
near-grammatical material stays, and the lost identities include the live
`on top of` surface (for example Academy Ruins, Run Aground, Repel, Brainstone,
and Plow Under). The requested one-line replacement removes that surface rather
than making the article optional. The coverage decrease is independently a STOP.

Item 4 is also not representable by the stated one-line scope. `Exile` is a
keyword-action declaration, not `CoreVerbIdentity`; its custom tail uses
`CustomTailAtom`, which has no `Role`/`FrameComplement` atom. An attempted
`With ObjectNounPhrase On Role("FrameComplement")` declaration was rejected
because `Role` is not a `CustomTailAtom` variant. Supporting the requested
counter tail requires a declaration-model/frame redesign, which the ticket
expressly fences out. That attempted row was removed immediately.

Coverage probe on the refreshed tree, report mode: 16,951 selected / 16,951
covered; 15,690 parse failures; 0 unresolved ties; 0 round-trip mismatches;
128 lock losses and 11 gains. No lock was blessed because this is a STOP. The
gains, with selected analyses, were:

- Gloom Surgeon — “If combat damage would be dealt to this creature, prevent that damage and exile that many cards from the top of your library.”
- Virtue of Courage // Embereth Blaze (Virtue of Courage) — “Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library. You may play those cards this turn.”
- Doomskar Warrior — “Backup 1 … Whenever this creature deals combat damage to a player or battle, look at that many cards from the top of your library. …”
- Feldon, Ronom Excavator — “Haste … Whenever Feldon is dealt damage, exile that many cards from the top of your library. …”
- The Key to the Vault — “Whenever equipped creature deals combat damage to a player, look at that many cards from the top of your library. …”
- Expedited Inheritance — “Whenever a creature is dealt damage, its controller may exile that many cards from the top of their library. …”
- Shadow Urchin — “Whenever a creature you control with one or more counters on it dies, exile that many cards from the top of your library. …”
- Thor, Guardian of Midgard — “Flying … Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library. …”
- Crumbling Sanctuary — “If damage would be dealt to a player, that player exiles that many cards from the top of their library instead.”
- Chandra, Legacy of Fire — “At the beginning of your end step, Chandra deals X damage … Exile that many cards from the top of your library. …”
- Dream Pillager — “Flying … Whenever this creature deals combat damage to a player, exile that many cards from the top of your library. …”

The coverage gate's complete loss delta was observed (128 names); it is not
reproduced here because no change can land after the STOP. No ambiguity
before/after comparison, roundtrip, or coverage bless was run after the STOP.
There was no genuine selected-reading tie.

Positive gates before the STOP: `cargo fmt --all` completed; strict
`cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings` completed
(`Finished dev profile … in 18.46s`). The workspace suite was started, but no
final result artifact was retained before stopping; it is not claimed as a
passing gate. Coverage performance advisory: workers 8, 162.331702937 s,
191.386 ns/B accepted CPU, host load 22.31 / 24.52 / 17.93; concurrent-process
count unavailable to the sandbox.

Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0. Deviations
and additions: none retained. Glossary gap: none (all five ticket terms occur
in the Oracle English context). Decision wanted: split the `Top` surface into a
form that preserves both articles, and decide whether lexeme-owned verb frames
need a declaration-model extension before retrying item 4. End:
`2026-09-04T15:24:26-07:00`.
