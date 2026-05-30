#!/usr/bin/env bash
# Bootstrap / refresh the flat atomic card store from the `mtg` Postgres DB.
#
#   1. the DB assembles every Vintage-legal/restricted atomic card into one delimited stream;
#   2. split_card_stubs.py writes the stubs into the ron/cards-gen/ submodule, points
#      ron/cards/<oracleId>.ron symlinks at them (leaving real, hand-encoded files), and
#      rebuilds ron/cards-by-name/;
#   3. the regeneration is committed inside the submodule (its history = templating/errata
#      drift), and the submodule pointer is bumped in the superproject.
#
# Re-runnable and idempotent: identical output -> no submodule commit, no pointer bump.
# Does NOT push — push main and the ron/cards-gen submodule yourself when ready.
set -euo pipefail
cd "$(dirname "$0")/.."

mkdir -p build
echo "generating stub stream from mtg DB ..."
psql -X -A -t -P pager=off -d mtg -f scripts/gen_card_stubs.sql -o build/cards_todo.ron
echo "intermediate: build/cards_todo.ron ($(wc -l < build/cards_todo.ron) lines)"
python3 scripts/split_card_stubs.py build/cards_todo.ron

ver=$(psql -X -t -A -d mtg -c "SELECT version || ' (' || date || ')' FROM meta LIMIT 1;")
[ -n "$ver" ] || ver="unknown MTGJSON version"

# 3a. commit the regeneration inside the submodule (the drift history)
if [ -n "$(git -C ron/cards-gen status --porcelain)" ]; then
  git -C ron/cards-gen add -A
  git -C ron/cards-gen commit -q -m "regen from MTGJSON $ver"
  echo "cards-gen: committed $(git -C ron/cards-gen rev-parse --short HEAD)"
else
  echo "cards-gen: no change since last regen"
fi

# 3b. bump the submodule pointer in the superproject (just the gitlink)
git add ron/cards-gen
if git diff --cached --quiet -- ron/cards-gen; then
  echo "main: submodule pointer unchanged"
else
  git commit -q -m "cards-gen: bump to $(git -C ron/cards-gen rev-parse --short HEAD) (MTGJSON $ver)"
  echo "main: pointer bumped to $(git -C ron/cards-gen rev-parse --short HEAD)"
fi

# New/dropped cards alter the symlinks in ron/cards/, which also holds your real encodings,
# so leave those for you to review and commit.
n=$(git status --porcelain -- ron/cards | wc -l)
if [ "$n" -gt 0 ]; then
  echo "note: ron/cards/ has $n change(s) (new/dropped cards and/or your edits) — review & commit separately."
fi
echo "(pushing main later also requires pushing the ron/cards-gen submodule)"
