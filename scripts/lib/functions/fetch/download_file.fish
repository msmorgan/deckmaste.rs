function download_file
    argparse -Ss -N2 -X2 t/tag= q/quiet v/verbose -- $argv
    or return

    set tag $_flag_tag
    set -l url $argv[1]
    set -l out $argv[2]

    set -q _flag_verbose; and echo >&2 $_flag_tag: "Downloading $url to $out"

    set -l code (curl -w '%{http_code}' -fsSL -z $out -o $out $url)
    and switch $code
        case 200
            set -q _flag_quiet; or echo >&2 $_flag_tag: "Downloaded to $out"
        case 304
            set -q _flag_quiet; or echo >&2 $_flag_tag: "Skipped $out; already up-to-date"
            return 1
        case '*'
            set -q _flag_quiet; or echo >&2 $_flag_tag: "ERROR: HTTP $code when downloading $url; aborting..."
            return 1
    end
end
