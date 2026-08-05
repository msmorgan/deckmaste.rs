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

function scryfall
    switch $argv[1]
        case download
            __scryfall_download $argv[2..]
        case '*'
            echo >&2 "$(status function): unknown subcommand '$argv[1]'"
            return 1
    end
end
