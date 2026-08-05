set -g __mtgjson_base_url 'https://mtgjson.com/api/v5'

function __mtgjson_download
    argparse c/compression= q/quiet -- $argv
    or return

    set -l ext ''
    set -l cache_dir $mtgjson_dir
    if set -q _flag_compression
        set ext ".$_flag_compression"
        set cache_dir $cache_dir/cache
    end

    mkdir -p $cache_dir

    for wanted in $argv
        set url $__mtgjson_base_url/$wanted$ext
        set cached $cache_dir/$wanted$ext

        download_file \
            --tag 'mtgjson' \
            --accept 'application/x-xz' \
            --user-agent 'deckmaste.rs/0.1 (+https://github.com/msmorgan/deckmaste.rs)' \
            $url $cached
        or return

        set -q _flag_compression
        and switch $cached
            case "*.tar$ext"
                tar >&2 -C $mtgjson_dir -xJf $cached
                or return
                echo >&2 "mtgjson: Extracted to $(path change-extension '' $mtgjson_dir/$wanted)"
            case "*$ext"
                set -l extracted $mtgjson_dir/$wanted
                set -l temporary (mktemp "$extracted.XXXXXX")
                or return

                xzcat $cached >$temporary
                set -l extract_status $status
                if test $extract_status -ne 0
                    command rm -f -- $temporary
                    return $extract_status
                end

                command mv -- $temporary $extracted
                or begin
                    command rm -f -- $temporary
                    return 1
                end
                echo >&2 "mtgjson: Extracted to $extracted"
        end
    end
end

function mtgjson
    switch $argv[1]
        case download
            __mtgjson_download -c xz $argv[2..]
        case '*'
            echo >&2 "$(status function): unknown subcommand '$argv[1]'"
            return 1
    end
end
