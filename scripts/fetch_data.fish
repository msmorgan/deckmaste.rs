#!/usr/bin/env fish
function download_file
    argparse -Ss -N2 -X2 t/tag= q/quiet -- $argv
    or return

    set tag $_flag_tag
    set -l url $argv[1]
    set -l out $argv[2]

    set -l code (curl -w '%{http_code}' -fsSL -z $out -o $out $url)
    switch $code
        case 200
            set -q _flag_quiet; or echo >&2 $tag "Downloaded to $out"
        case 304
            set -q _flag_quiet; or echo >&2 $tag "Skipped $out; already up-to-date"
        case '*'
            set -q _flag_quiet; or echo >&2 $tag "ERROR: HTTP $code when downloading $url; aborting..."
            return 1
    end
end

function fetch_mtgjson -a cache_dir extract_dir
    mkdir -p $cache_dir $extract_dir

    set base_url 'https://mtgjson.com/api/v5'
    set wanted_files \
        Meta.json \
        CompiledList.json \
        EnumValues.json \
        Keywords.json \
        CardTypes.json \
        SetList.json \
        AllPrintings.json \
        AllSetFiles.tar

    for file in $wanted_files
        set url $base_url/$file.xz
        set cached $cache_dir/$file.xz

        set prev_mtime (path mtime $cached) 0

        download_file -t '('(status function)')' $url $cached
        or return

        test (path mtime $cached) -gt $prev_mtime[1]
        or continue

        switch $cached
            case '*.json.xz'
                set -l extracted $extract_dir/$file
                xzcat $cached >$extracted
                echo >&2 $tag "Extracted to $extracted"
            case '*.tar.xz'
                tar >&2 -C $extract_dir -xJf $cached
                echo >&2 $tag "Extracted to $(path change-extension '' $extract_dir/$file)"
        end
    end
end

function fetch_rules -a cache_dir
    mkdir -p $cache_dir

    set base_url "https://api.academyruins.com"
    set wanted_files_map \
        link/cr:cr.txt \
        cr:cr.json \
        cr/keywords:keywords.json \
        cr/glossary:glossary.json \
        cr/unofficial-glossary:unofficial-glossary.json \
        mtr:mtr.json

    for entry in $wanted_files_map
        string split ':' -- $entry | read -L src dest
        set url $base_url/$src
        set cached $cache_dir/$dest

        set prev_mtime (path mtime $cached) 0
        download_file -t (status function) $url $cached
        or return

        test (path mtime $cached) -gt $prev_mtime[1]
        or continue

        # Nothing to do after downloading.
    end
end

set -l project_dir (path resolve (status dirname)/..)
set -l data_dir $project_dir/data

fetch_mtgjson $data_dir/mtgjson{/xz,}
fetch_rules $data_dir/rules
