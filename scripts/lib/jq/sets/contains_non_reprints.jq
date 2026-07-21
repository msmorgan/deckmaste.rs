.data[] |
select(
  .cards |
  map(
    select(
      (.isReprint | not) and
      ((.legalities.vintage // "Banned") as $legality |
        $legality == "Legal" or $legality == "Restricted")
    )
  ) |
  length > 0
) |
.code + "\t" + .type
