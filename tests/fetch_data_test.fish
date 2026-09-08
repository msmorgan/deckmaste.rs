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

function check_arg --argument-names label expected
    set -l args
    while read -l arg
        set -a args $arg
    end <$curl_log
    check $label 0 (contains -- $expected $args; echo $status)
end

set -g test_root (mktemp -d)
or exit 1

function cleanup --on-event fish_exit
    command rm -rf -- $test_root
end

set -g catalogs_dir $test_root/catalogs
set -g mtgjson_dir $test_root/mtgjson
set -g scryfall_dir $test_root/scryfall
set -g curl_log $test_root/curl.log
set -g sleep_log $test_root/sleep.log
set -g cargo_log $test_root/cargo.log
mkdir -p $catalogs_dir

source scripts/lib/functions/fetch/download_file.fish
source scripts/lib/functions/fetch/scryfall.fish
source scripts/lib/functions/fetch/mtgjson.fish

set -g fake_http_code 200
set -g fake_curl_status 0
set -g fake_xz_status 0

function curl
    printf '%s\n' $argv >$curl_log

    set -l output
    for index in (seq (count $argv))
        if test "$argv[$index]" = -o
            set output $argv[(math $index + 1)]
            break
        end
    end

    if test $fake_curl_status -eq 0; and test "$fake_http_code" = 200
        printf 'fresh response\n' >$output
    end

    printf '%s' $fake_http_code
    return $fake_curl_status
end

function sleep
    printf '%s\n' $argv[1] >$sleep_log
end

function xzcat
    printf 'decoded MTGJSON\n'
    return $fake_xz_status
end

function cargo
    printf '%s\n' $argv >$cargo_log
    if contains -- uri $argv
        echo https://data.scryfall.io/oracle-cards/oracle-cards-test.jsonl.gz
    end
end

function reset_fake --argument-names code curl_status
    set -g fake_http_code $code
    set -g fake_curl_status $curl_status
    printf '' >$curl_log
    printf '' >$sleep_log
end

reset_fake 200 0
scryfall download catalog/creature-types:creature-types.json
check '200 succeeds' 0 $status
check '200 replaces destination atomically' 'fresh response' \
    (string trim < $catalogs_dir/creature-types.json)
check_arg 'Scryfall sends an explicit Accept header' 'Accept: application/json'
check_arg 'Scryfall sends an explicit User-Agent' \
    'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)'
check_arg 'downloads retry transient upstream failures' '--retry'
check 'catalog request waits 100 ms' 0.1 (string trim <$sleep_log)

printf 'cached response\n' >$catalogs_dir/creature-types.json
reset_fake 304 0
scryfall download catalog/creature-types:creature-types.json
check '304 is a successful refresh' 0 $status
check '304 preserves cached destination' 'cached response' \
    (string trim <$catalogs_dir/creature-types.json)
check_arg '304 uses a conditional request' '-z'
check '304 still waits 100 ms' 0.1 (string trim <$sleep_log)

reset_fake 503 22
scryfall download catalog/creature-types:creature-types.json
check 'curl failure propagates' 22 $status
check 'failure preserves cached destination' 'cached response' \
    (string trim <$catalogs_dir/creature-types.json)
check 'failure still waits 100 ms' 0.1 (string trim <$sleep_log)

reset_fake 304 0
mkdir -p $catalogs_dir/cards
printf 'cached card\n' >$catalogs_dir/cards/test.json
scryfall download cards/test:cards/test.json
check 'card request succeeds' 0 $status
check 'card request uses the conservative delay' 0.5 (string trim <$sleep_log)

# The optional MTGJSON reference cache stores compressed inputs. A conditional
# 304 must still reconstruct the local input.
mkdir -p $mtgjson_dir/cache
printf 'cached xz\n' >$mtgjson_dir/cache/Meta.json.xz
reset_fake 304 0
mtgjson download Meta.json
check 'MTGJSON 304 still succeeds' 0 $status
check 'MTGJSON 304 reconstructs the uncached expansion' 'decoded MTGJSON' \
    (string trim <$mtgjson_dir/Meta.json)

printf 'usable old expansion\n' >$mtgjson_dir/Meta.json
set -g fake_xz_status 17
reset_fake 304 0
mtgjson download Meta.json
check 'MTGJSON extraction failure propagates' 17 $status
check 'MTGJSON extraction failure preserves the usable expansion' \
    'usable old expansion' (string trim <$mtgjson_dir/Meta.json)
set -g fake_xz_status 0

# Exercise the script-level option plumbing without reaching the network. The
# provider stubs also pin fail-fast behavior at the orchestration boundary.
set -l fetch_plan (fish -C '\
    function mtgjson; echo mtgjson:$argv; end; \
    function academyruins; echo academyruins:$argv; end; \
    function scryfall; echo scryfall:$argv; end\
' scripts/fetch_data --minimal --cr)
check 'minimal plus CR plan succeeds' 0 $status
check 'minimal plan refreshes Oracle Cards' 0 \
    (string match -q '*scryfall:oracle-cards*' -- $fetch_plan; echo $status)
check 'minimal plus CR includes cr.txt' 0 \
    (string match -q '*academyruins:file/cr:cr.txt*' -- $fetch_plan; echo $status)
# Flavor words are the one catalog the English loader reads from the Scryfall
# dump rather than a generated CR-derived file, so a minimal fetch that omits
# them leaves a clean checkout unable to run the suite.
check 'minimal plan includes the flavor-word catalog' 0 \
    (string match -q '*catalog/flavor-words:flavor-words.json*' -- $fetch_plan; echo $status)

set -l failed_plan (fish -C '\
    function mtgjson; echo unexpected-mtgjson; end; \
    function academyruins; echo unexpected-academyruins; end; \
    function scryfall; return 23; end\
' scripts/fetch_data --minimal --cr)
check 'provider failure propagates from fetch_data' 23 $status
check 'provider failure stops later hosts' '' "$failed_plan"

# Ubuntu's Fish 3.7 does not implement argparse's strict-longopts flag. Keep the
# CI-invoked script and every function on the fetch path source-compatible.
set -l strict_longopts
for file in \
        scripts/fetch_data \
        scripts/lib/functions/fetch/download_file.fish \
        scripts/lib/functions/fetch/mtgjson.fish \
        scripts/lib/functions/fetch/scryfall.fish
    while read -l line
        string match -rq '^\s*argparse\s+.*-[A-Za-z]*S' -- $line
        and set -a strict_longopts "$file: $line"
    end <$file
end
check 'CI Fish path avoids strict-longopts' 0 (count $strict_longopts)

test $failures -eq 0
