---
needs: []
---
# Oracle editorial corrections for the English v4 census

Some current Oracle texts break a convention their printed cards keep. The
census should read a recorded correction for each such card instead of
relaxing the grammar for a single editorial slip.

## Change

- Keep a recorded list of Oracle editorial corrections next to the census
  inputs. Each entry names the card, the Oracle text being corrected, the
  corrected text, and the evidence for the correction.
- Make the census read the corrected text for every listed card. The grammar
  stays as it is: `docs/decisions/english-v4.md` §3 (item 3) requires the Oxford comma
  for coordinations of three or more, and that rule is not relaxed.
- Each correction must cite printed-card evidence. A sibling card with the same
  template, alone, is supporting evidence only.

## First entry: Agency Outfitter (MKM 0038)

- Current Oracle text (`data/scryfall/oracle-cards.jsonl`): "you may search
  your graveyard, hand and/or library for a card named Magnifying Glass …".
- Printed card: the Scryfall MKM image
  <https://cards.scryfall.io/large/front/8/1/8112f133-535e-4264-8357-9cbf97957710.jpg>
  reads "When Agency Outfitter enters the battlefield, you may search your
  graveyard, hand, and/or library for …", with the Oxford comma.
- Template siblings whose current Oracle text has "graveyard, hand, and/or
  library" (all checked in `data/scryfall/oracle-cards.jsonl`): Boonweaver Giant,
  Dark Supplicant, Mishra, Artificer Prodigy, Say Its Name.
- MTGJSON's `originalText` for this card (`data/mtgjson/AllSetFiles/MKM.json`)
  also lacks the comma, which contradicts the printed image. Do not cite that
  field as evidence for this card.
- Until this ticket lands, Agency Outfitter is listed in
  `crates/deckmaste_english_v4/deferred.txt`, owned by this ticket.

## Done when

- The census reads Agency Outfitter's corrected text, and the unit reads at its
  stratum.
- Its `deferred.txt` entry is removed.

Standard constraints apply.
