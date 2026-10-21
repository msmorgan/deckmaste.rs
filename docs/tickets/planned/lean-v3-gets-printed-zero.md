---
needs: []
---
# "gets +X/+0" is one `getsPt` carrying both printed deltas

Standard constraints apply. This is finding 6 of the spelling-layer audit. The words "gets +P/+T"
are written in three ways today, and the checker accepts all three:
- one `getsPt` / `get`;
- a `conjunction` of two single-stat `modification`s;
- a lone power `modification` for "+N/+0".

## Ruling (owner, 2026-10-21)

- `getsPt` always carries both printed deltas, including a "+0". The printed "+0" is part of the
  encoding, so spelling must render it back. "+X/+0" is `getsPt s (up X) (up (.lit 0))`, the form
  the existing "+N/+0" `getsPt` / `get` sites already use (*Ahn-Crop Invader*, *Char-Rumbler*, …).
- ADR [semantics-v3](../../decisions/semantics-v3.md) §5 "Stat Changes" now says this. The
  single-stat `modification` template remains only for a change whose text names one stat.
- [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §3 no longer drops
  Berserk's "+0" toughness edit.

## Sites

Line numbers verified 2026-10-21:
- **battle cry** `Definitions/Keywords.lean:306-312` [CR#702.91a] ("each other attacking creature
  gets +1/+0") writes a lone power `modification` (`:310`). Rewrite it as
  `getsPt … (up (.lit 1)) (up (.lit 0))`.
- ***Berserk*** `Cards/Turn.lean:514-515` ("gets +X/+0") writes a power `modification` plus a
  toughness `modification it … (up (.lit 0))` inside `establishFor`. Rewrite it as one `getsPt`.
  The fidelity ticket's §3 item binds `target isCreature` once on the same lines. Whichever lands
  second rebases.
- ***Deadly Dancer*** `Cards/Cost.lean:1018-1023` ("This creature and another target creature each
  get +1/+0") writes a `conjunction` of two `modification`s. Rewrite it as one `getsPt` over
  `eachOf (both thisCreature (target …))`.
- ***Arlinn Kord*** `Cards/Faces.lean:296-297` ("up to one target creature gets +2/+2") and
  ***Arlinn, Embraced by the Moon*** `:312-313` ("Creatures you control get +1/+1") each write a
  power/toughness `modification` pair. Rewrite each as one `getsPt`.
- ***Ashmouth Blade*** (back face of *Neglected Heirloom*) `Cards/Faces.lean:340-341`
  ("Equipped creature gets +3/+3") is the same split. The ruling's "Faces.lean ~341" names this
  card, not Arlinn.

`rg 'modification' lean/Semantics` lists no other split site. The `modification` template itself
(`Templates.lean:1947`) stays.

Battle cry and *Deadly Dancer* also print "another"/"other", which
[lean-v3-another-contrast-anchor](lean-v3-another-contrast-anchor.md) rewrites. Whichever ticket
lands second rebases onto the other's encoding.

Update `lean/CONTRACTS.md` ("gets +P/+T", `:306` and `:779`) to say that a printed "+0" is
written.

## Tests

- The `ok…` theorems for *Berserk*, *Deadly Dancer*, *Arlinn Kord* (both faces), *Ashmouth Blade*
  and the battle cry pins stay green on the rewritten forms.
- Add one pin that battle cry's characteristic change carries both a power and a toughness edit,
  the toughness edit being `+0`. Assert it as that property, not as an `rfl` restatement of the
  whole expansion, which
  [lean-v3-proofs-and-docs-alignment](lean-v3-proofs-and-docs-alignment.md) retires ("Retire vacuous `rfl` restatements").
