---
needs: []
---
**Airbend's body reads the airbending stamp its own wrapper has not yet
made.** Found 2026-10-05 at `plugins-v2-keyword-helper-additions`. Standard
constraints apply.

The loader wraps a keyword action's body as `Enact(Action("Airbend"), body,
actor)`, and `keyword_actions/airbend.ron`'s body reads its objects back as
`themVerbed(Action("Airbend"))` three times (the owner, the permission's
counterpart, and the "for as long as" condition). Inside the wrapper that
stamp does not exist yet, so the Lean card check refuses every reading,
`[anaphor (stamped (action "Airbend")) many 0]` three times, for the body as
it stands today and for the handoff spelling alike. No canon card airbends,
so nothing caught it. The probe "Airbend target creature." (an instant) was
used to find this; it is kept outside the repo.

The handoff spelling the owner approved is ready once the body has a
referent: `act(ownerOf(<the exiled objects>), establish(mayCastFrom(<them>,
exileZone, PayingInstead(mana([2]))), forAsLongAs(…)))` [CR#701.65a]. The fix
is to read "them" as what the body itself exiled (for example an exile that
stamps, read back by its own deed), not to change the checker; "each exiled
card's owner" may also need the group handoff to distribute per card.

Proof: a card or probe that airbends proves under `cargo xtask lean-check`.
