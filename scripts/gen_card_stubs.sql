-- scripts/gen_card_stubs.sql — bootstrap the flat atomic card store from MTGJSON (`mtg` DB).
--
-- Emits one block per Vintage-legal/restricted atomic card (one per scryfallOracleId):
--
--     // ===>\t<oid>\t<name>      ← splitter delimiter (consumed; not written into the file)
--     // <name>                   ← header comment: full card name (with `//` for multi-face)
--     Todo(r#"<Scryfall-style rendering>"#)
--
-- The rendering, per face, is blank-line-separated sections:
--     <name> [<mana cost>]
--     [Color Indicator: <colors>\n]<type line>
--     <oracle text>            (verbatim MTGJSON `text`, incl. reminder text; omitted if none)
--     <stats>                  (<P/T> | "Loyalty: N" | "Defense: N"; omitted if none)
-- Multi-face cards aggregate their faces separated by a `---` line. Oracle text is atomic
-- (identical across printings), so DISTINCT ON (oracleId, side) yields one block per face.
--
-- Run through scripts/gen_card_stubs.sh, which writes the stubs to ron/cards-gen/ and symlinks
-- ron/cards/<oid>.ron into them (real, hand-encoded files are left as-is).
--
--   psql -X -A -t -P pager=off -d mtg -f scripts/gen_card_stubs.sql -o build/cards_todo.ron
\pset footer off
WITH vintage_oids AS (
    -- Oracle ids with at least one Vintage Legal or Restricted printing. Vintage is the
    -- eternal format, so this drops ante/banned cards, the deck-construction Conspiracies,
    -- un-cards, and digital-only (Alchemy) cards in one predicate. Legality is per-oracle,
    -- so we collect ids here and include ALL faces of each below.
    SELECT DISTINCT ci."scryfallOracleId" AS oid
    FROM cards c
    JOIN "cardIdentifiers" ci ON ci.uuid = c.uuid
    JOIN "cardLegalities"  cl ON cl.uuid = c.uuid
    WHERE cl.vintage IN ('Legal', 'Restricted')
      AND ci."scryfallOracleId" IS NOT NULL
),
faces AS (
    SELECT DISTINCT ON (ci."scryfallOracleId", c.side)
        ci."scryfallOracleId"                                   AS oid,
        c.name                                                  AS cardname,
        c.side                                                  AS side,
        concat_ws(E'\n\n',
          -- name, with mana cost when it has one
          coalesce(c."faceName", c.name) || coalesce(' ' || nullif(c."manaCost", ''), ''),
          -- optional "Color Indicator:" line (codes -> names) directly above the type line
          coalesce(
            'Color Indicator: '
              || replace(replace(replace(replace(replace(nullif(c."colorIndicator", ''),
                   'G', 'Green'), 'R', 'Red'), 'W', 'White'), 'B', 'Black'), 'U', 'Blue')
              || E'\n', '')
            || coalesce(c.type, '(no type line)'),
          -- oracle text, verbatim (concat_ws drops it when NULL/empty)
          nullif(c.text, ''),
          -- stats line
          CASE
            WHEN c.power   IS NOT NULL THEN c.power || '/' || coalesce(c.toughness, '?')
            WHEN c.loyalty IS NOT NULL THEN 'Loyalty: ' || c.loyalty
            WHEN c.defense IS NOT NULL THEN 'Defense: ' || c.defense
            ELSE NULL
          END
        )                                                       AS block
    FROM cards c
    JOIN "cardIdentifiers" ci ON ci.uuid = c.uuid
    WHERE ci."scryfallOracleId" IN (SELECT oid FROM vintage_oids)
    ORDER BY ci."scryfallOracleId", c.side NULLS FIRST, c.name
)
SELECT
     '// ===>' || E'\t' || oid || E'\t' || max(cardname) || E'\n'
  || '// ' || max(cardname) || E'\n'
  || 'Todo(r#"' || string_agg(block, E'\n\n---\n\n' ORDER BY side NULLS FIRST) || '"#)' || E'\n'
FROM faces
GROUP BY oid
ORDER BY oid;
