set -g __scryfall_base_url 'https://api.scryfall.com'

function __scryfall_download
    mkdir -p $catalogs_dir
    for entry in $argv
        string split ':' -- $entry | read -L src dest
        set url $__scryfall_base_url/$src
        set out $catalogs_dir/$dest

        download_file \
            --tag 'scryfall' \
            --accept 'application/json' \
            --user-agent 'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)' \
            $url $out
        set -l download_status $status

        set -l delay 0.1
        if string match -rq '^cards' -- $src
            set delay 0.5
        end
        sleep $delay
        set -l sleep_status $status

        # Scryfall asks clients to leave at least 100 ms between requests.
        # Pace every attempt, including 304 responses and failed requests.
        test $download_status -eq 0; or return $download_status
        test $sleep_status -eq 0; or return $sleep_status
    end
end

function __scryfall_oracle_cards
    set -l cache_dir $scryfall_dir/cache
    set -l descriptor $cache_dir/oracle-cards-descriptor.json
    mkdir -p $cache_dir
    or return

    download_file \
        --tag 'scryfall oracle descriptor' \
        --accept 'application/json' \
        --user-agent 'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)' \
        $__scryfall_base_url/bulk-data/oracle-cards $descriptor
    or return
    sleep 0.1
    or return

    set -l uri (cargo xtask scryfall-snapshot uri --descriptor $descriptor)
    or return
    set -l compressed $cache_dir/(path basename $uri)
    download_file \
        --tag 'scryfall oracle cards' \
        --accept 'application/gzip' \
        --user-agent 'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)' \
        $uri $compressed
    or return
    sleep 0.1
    or return

    cargo xtask scryfall-snapshot prepare \
        --descriptor $descriptor \
        --compressed $compressed \
        --output $scryfall_dir/oracle-cards.jsonl \
        --metadata $scryfall_dir/oracle-cards.metadata.json
end

function scryfall
    switch $argv[1]
        case download
            __scryfall_download $argv[2..]
        case oracle-cards
            __scryfall_oracle_cards
        case '*'
            echo >&2 "$(status function): unknown subcommand '$argv[1]'"
            return 1
    end
end
