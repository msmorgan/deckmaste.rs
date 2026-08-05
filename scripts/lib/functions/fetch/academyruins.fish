set -g __academyruins_base_url 'https://api.academyruins.com'

function __academyruins_download
    mkdir -p $rules_dir
    for entry in $argv
        string split ':' -- $entry | read -L src dest
        set url $__academyruins_base_url/$src
        set out $rules_dir/$dest

        set -l accept 'application/json'
        string match -q '*.txt' -- $dest; and set accept 'text/plain'
        download_file \
            --tag 'academyruins' \
            --accept $accept \
            --user-agent 'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)' \
            $url $out
        or return
    end
end

function academyruins
    switch $argv[1]
        case download
            __academyruins_download $argv[2..]
        case '*'
            echo >&2 "$(status function): unknown subcommand '$argv[1]'"
            return 1
    end
end
