set -g __scryfall_base_url 'https://api.scryfall.com'

function __scryfall_download
    mkdir -p $catalogs_dir
    for entry in $argv
        string split ':' -- $entry | read -L src dest
        set url $__scryfall_base_url/$src
        set out $catalogs_dir/$dest

        download_file --tag 'scryfall' $url $out
        or continue

        if string match -rq '^cards' -- $src
            sleep 0.5
        else
            sleep 0.1
        end
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