function create_subtypes
    argparse -N3 -- $argv
    or return

    set jq_func $argv[1]
    set dest_dir $argv[2]
    set prefix $argv[3]
#     set rule_ref $argv[4]

    set jq_template $prefix'Type(\"\(.)\")'

    _jq -n '
    import "rules" as rules;
    import "util" as util;

    rules::'$jq_func'[]
    | {
        filename: (. | util::capitalize | gsub("[^A-Za-z0-9]"; "")) + ".ron",
        contents: . | @text "'$jq_template'",
    }
    ' | while read line
        set -l filename (echo "$line" | _jq '.filename')
        set -l contents (echo "$line" | _jq '.contents' | string collect)

        set -l dest $dest_dir/$filename
        if not is_todo $dest
            continue
        end

        echo "$contents" >$dest
        echo >&2 "wrote $dest"
    end
end
