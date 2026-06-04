function create_keyword_todos
    argparse -Ss -N2 -- $argv
    set jq_func $argv[1]
    set dest_dir $argv[2]

    set jq_template \
'// CR \(.ruleSection[0].ruleNumber)
Todo(
    name: \"\(.name)\",
    template: \"\(.template)\",
    rule: r#\"
\(.ruleSection | rules::format_section)
\"#,
)'

    _jq -n '
import "rules" as rules;
import "util" as util;

rules::'$jq_func'[]
| {
    name: . | util::to_rust_ident,
    template: .,
    ruleSection: . | rules::get_keyword_cr}
| {
    filename: .name + ".ron",
    contents: @text "'$jq_template'"
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
