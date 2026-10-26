#!/usr/bin/env bash
# Refuse the FIRST touch of a legacy crate that a pending cutover deletes or constrains.
#
# Session preamble (CLAUDE.md "Crate fates") decays under context pressure and is
# bypassed entirely when a search result sends a tool straight to a line number.
# Appended context does not help either: by the time it is read, the stale file is
# already in context and reads as current. So the first touch is DENIED with an
# explanation, and the caller must repeat the call deliberately.
#
# Acknowledgements are per crate, per session, in /tmp/<session-id>.old-shit.
# For a stricter per-file gate, set key="$path" instead of the crate name below.
#
# Authority for every claim below: CLAUDE.md (English grammar) and docs/decisions/english-lexical-analysis.md
set -u

input=$(cat)
path=$(printf '%s' "$input" | jq -r '.tool_input.file_path // .tool_input.path // empty')
[ -n "$path" ] || exit 0

case "$path" in
  */crates/deckmaste_legacy_render/*)
    key="deckmaste_legacy_render"
    why="LEGACY; deletion-bound at semantics-v1-cutover, and deckmaste_migrations must shed its dependency on it by then. Do not build new work on this crate." ;;
  */crates/deckmaste_migrations/*)
    key="deckmaste_migrations"
    why="SURVIVES the cutovers as a function (card extract/resolve/graduate, snapshot ingestion), but its oracle-text extraction is its own regex pipeline that does NOT consume the english-v4 parser, and it still depends on deletion-slated deckmaste_legacy_render. Do not home new English infrastructure here without recording the deviation in docs/decisions/english-v4.md. Re-pointing extraction at english-v4 is a separate, not-yet-scheduled decision." ;;
  *)
    exit 0 ;;
esac

session=$(printf '%s' "$input" | jq -r '.session_id // "unknown"' | tr -d '\n' | tr -c 'A-Za-z0-9._-' '_')
ledger="/tmp/${session}.old-shit"

if [ -f "$ledger" ] && grep -qxF "$key" "$ledger" 2>/dev/null; then
  # Already acknowledged this session — let it through, but keep the status in context.
  jq -n --arg k "$key" --arg w "$why" '{hookSpecificOutput:{hookEventName:"PreToolUse",
    additionalContext:("Reminder — \($k) is a pre-cutover legacy crate: \($w) Authority: CLAUDE.md (English grammar) and docs/decisions/english-lexical-analysis.md")}}'
  exit 0
fi

printf '%s\n' "$key" >> "$ledger"
jq -n --arg k "$key" --arg w "$why" --arg p "$path" '{hookSpecificOutput:{hookEventName:"PreToolUse",
  permissionDecision:"deny",
  permissionDecisionReason:("STOP — you are looking at old shit. \($p) is in \($k), which is \($w)\n\nAuthority: CLAUDE.md (English grammar) and docs/decisions/english-lexical-analysis.md\n\nIf the current implementation is what you actually wanted, go there instead. If you genuinely intended to read pre-cutover code, repeat this exact call — it will be allowed for the rest of this session.")}}'
