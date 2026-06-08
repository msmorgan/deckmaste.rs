#!/usr/bin/env fish
# One-shot: migrate legacy `CR …` / `rule …` / en-dash & hyphen ranges to the
# canonical `[CR#…]` form. Tracked .rs/.md/.ron only.
#
# SCOPE / SAFETY
# --------------
# The wide-net detector (`cargo xtask cite check --list-noncompliant`) flags
# every citation-LOOKING string. But several files use rule numbers as *data*,
# not as prose citations — converting them would corrupt the citation tooling's
# own self-tests and the CR-text fixtures. Those files are excluded below and
# left untouched (reported separately as deliberate non-conversions):
#   - crates/xtask/src/{citations,cr,legacy,lockfile}.rs   (tool self-tests)
#   - crates/xtask/tests/check.rs                          (tool self-tests)
#   - crates/deckmaste_migrations/src/data/academyruins.rs (verbatim CR text)
#   - crates/deckmaste_migrations/src/migrations/keyword_todos.rs (CR-text fixtures)
#
# This script handles ONLY the unambiguous mechanical forms keyed off an
# explicit `CR `/`rule ` prefix or an en-dash/hyphen RANGE. Slash-lists and
# genuine *bare* citations are context-sensitive and are migrated by hand
# (see the commit message / task report), NOT here.

set -l excludes \
    crates/xtask/src/citations.rs \
    crates/xtask/src/cr.rs \
    crates/xtask/src/legacy.rs \
    crates/xtask/src/lockfile.rs \
    crates/xtask/tests/check.rs \
    crates/deckmaste_migrations/src/data/academyruins.rs \
    crates/deckmaste_migrations/src/migrations/keyword_todos.rs

for f in (jj file list)
    string match -qr '\.(rs|md|ron)$' -- $f; or continue
    contains -- $f $excludes; and continue
    test -f $f; or continue
    set -l tmp (mktemp)
    # Pass order matters: ranges first (they contain a letter+sep+letter that the
    # single-CR pattern would otherwise swallow as a lone member), then
    # comma-lists, then singles, then `rule N`, then collapse the `, ` spacing.
    #
    # `CR ?` makes the leading space optional (handles the one `CR107.4`).
    # The hyphen-range right side is a SINGLE lowercase letter with a negative
    # lookahead so `CR 603.4-style` / `CR 106.6-ish` stay `[CR#603.4]-style`.
    string replace -ra '(CR ?)?(\d{1,3}\.\d+)([a-z])\x{2013}([a-z])\b' '[CR#$2$3..$2$4]' <$f \
      | string replace -ra '(CR ?)?(\d{1,3}\.\d+)([a-z])-([a-z])(?![a-z])' '[CR#$2$3..$2$4]' \
      | string replace -ra 'CR ?(\d{1,3}(?:\.\d+[a-z]*)?(?:,\s*\d{1,3}(?:\.\d+[a-z]*)?)+)' '[CR#$1]' \
      | string replace -ra 'CR ?(\d{1,3}(?:\.\d+[a-z]*)?)' '[CR#$1]' \
      | string replace -ra '\brule (\d{1,3}(?:\.\d+[a-z]*)?)' '[CR#$1]' \
      | string replace -ra '(\[CR#[0-9.,a-z]*?), +' '$1,' \
      | string replace -ra '(\[CR#[0-9.,a-z]*?), +' '$1,' \
      | string replace -ra '(\[CR#[0-9.,a-z]*?), +' '$1,' \
      | string replace -ra '(\[CR#[0-9.,a-z]*?), +' '$1,' >$tmp
    mv $tmp $f
end
