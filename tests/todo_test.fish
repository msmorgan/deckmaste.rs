#!/usr/bin/env fish
set -g failures 0
function check --argument-names label expected actual
    if test "$expected" = "$actual"
        echo "ok - $label"
    else
        echo "NOT ok - $label"
        echo "  expected: $expected"
        echo "  actual:   $actual"
        set failures (math $failures + 1)
    end
end

set -l root (status dirname)/tickets

# ready: alpha (needs base=done). beta blocked, cycles blocked.
set -l ready (env TODO_ROOT=$root TODO_CENSUS=$root/census.md scripts/todo ready --slugs-only | sort | string join ',')
check "ready" "alpha" "$ready"

# blocked: beta, cycle-a, cycle-b (kw-enchant needs engine-attach which has no ticket → dangling, not blocked-ready)
set -l blocked (env TODO_ROOT=$root TODO_CENSUS=$root/census.md scripts/todo blocked --slugs-only | sort | string join ',')
check "blocked" "beta,cycle-a,cycle-b" "$blocked"

# check: reports the cycle and the dangling census need engine-attach
set -l checkout (env TODO_ROOT=$root TODO_CENSUS=$root/census.md scripts/todo check)
check "check finds cycle" 0 (string match -q '*cycle*' -- "$checkout"; echo $status)
check "check finds dangling" 0 (string match -q '*engine-attach*' -- "$checkout"; echo $status)

# A focused invalid graph pins check's failure status. If the checker regresses
# to accepting a dangling dependency, this assertion makes the harness fail.
set -l invalid_root "$root/invalid"
set -l invalid_out (env TODO_ROOT=$invalid_root TODO_CENSUS=$invalid_root/census.md scripts/todo check)
set -l invalid_status $status
check "check rejects focused dangling fixture" 1 "$invalid_status"
check "check identifies focused dangling fixture" 0 \
    (string match -q '*engine-dependent needs unknown engine-missing*' -- "$invalid_out"; echo $status)

# graph: alpha's upstream is base
set -l g (env TODO_ROOT=$root TODO_CENSUS=$root/census.md scripts/todo graph alpha)
check "graph upstream" 0 (string match -q '*base*' -- "$g"; echo $status)

test $failures -eq 0
