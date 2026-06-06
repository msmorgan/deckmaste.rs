.data[] |
select(
  .cards |
  map(
    select(
      (.isReprint | not) and
      ((.legalities.vintage // "Banned") != "Banned")
    )
  ) |
  length > 0
) |
.code + "\t" + .type