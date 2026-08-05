function download_file
    argparse -s -N2 -X2 \
        t/tag= q/quiet v/verbose a/accept= u/user-agent= -- $argv
    or return

    set -l url $argv[1]
    set -l out $argv[2]

    set -q _flag_verbose; and echo >&2 $_flag_tag: "Downloading $url to $out"

    # Download to a sibling temporary file so a failed refresh cannot destroy
    # a usable cached input. `-z` turns a restored cache entry into a
    # conditional request while still allowing a cache miss to fetch normally.
    set -l temporary (mktemp "$out.XXXXXX")
    or return

    set -l curl_args \
        -w '%{http_code}' -fsSL \
        --retry 3 --retry-max-time 60 \
        -o $temporary
    test -f $out; and set -a curl_args -z $out
    set -q _flag_accept; and set -a curl_args --header "Accept: $_flag_accept"
    set -q _flag_user_agent; and set -a curl_args --user-agent $_flag_user_agent

    set -l code (curl $curl_args $url)
    set -l curl_status $status
    if test $curl_status -ne 0
        command rm -f -- $temporary
        return $curl_status
    end

    switch $code
        case 200
            command mv -- $temporary $out
            or begin
                command rm -f -- $temporary
                return 1
            end
            set -q _flag_quiet; or echo >&2 $_flag_tag: "Downloaded to $out"
        case 304
            command rm -f -- $temporary
            set -q _flag_quiet; or echo >&2 $_flag_tag: "Skipped $out; already up-to-date"
        case '*'
            command rm -f -- $temporary
            set -q _flag_quiet
            or echo >&2 $_flag_tag: \
                "ERROR: HTTP $code when downloading $url; aborting..."
            return 1
    end
end
