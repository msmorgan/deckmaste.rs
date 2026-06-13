#!/usr/bin/env fish
# scripts/hooks/jj_guard.fish — PreToolUse(Bash) guard.
#
# Two rules, both about routing through the project's own tooling:
#   1. jj must go through scripts/jj — that wrapper pins the workspace (-R) and,
#      outside `default`, marks the default line immutable, so a stray rebase or
#      abandon can't rewrite shared history or another worktree. Bare/other `jj`
#      skips all of that, so it's refused.
#   2. git is banned — this is a jj repo; git mutations corrupt/confuse the state.
#
# Exit 2 + stderr blocks the tool call; exit 0 allows it. Anything we can't parse
# is allowed (don't interfere with non-Bash tools or malformed payloads).

set -l payload (cat | string collect)
test (printf '%s' $payload | jq -r '.tool_name // ""') = Bash; or exit 0
set -l cmd (printf '%s' $payload | jq -r '.tool_input.command // ""')

# Every jj invocation (bare `jj`, or `<path>/jj`, at a command position — start, or
# after ; & | && || newline ( or a command/exec/sudo/… wrapper) must be scripts/jj.
for hit in (string match -rga -- '(?:^|&&|\|\||[;&|\n(])\s*(?:(?:command|exec|builtin|env|sudo|time|nice|nohup|xargs)\s+)*((?:[^\s;&|()`]*/)?jj)(?:\s|$)' $cmd)
    string match -rq -- '(?:^|/)scripts/jj$' $hit; and continue
    echo >&2 "jj-guard: run jj through scripts/jj — it pins the workspace and the immutability guard. Refusing: $hit"
    exit 2
end

# git: refused outright.
if string match -rq -- '(?:^|&&|\|\||[;&|\n(])\s*(?:(?:command|exec|builtin|env|sudo|time|nice|nohup|xargs)\s+)*(?:[^\s;&|()`]*/)?git(?:\s|$)' $cmd
    echo >&2 "jj-guard: git is banned in this jj repo — use scripts/jj / the jj tooling, not git."
    exit 2
end

exit 0
