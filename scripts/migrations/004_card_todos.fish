#!/usr/bin/fish

source (path resolve (status dirname)/../lib/setup.fish)
or return

set -l plugin_dir (check_plugin_dir -n (status filename) $argv[1])
or return

set -l dest_dir $plugin_dir/cards
mkdir -p $dest_dir

set -l jq_template (cat (status dirname)/templates/todo_card.ron.text.jq_template | string collect)

set jq_filter '
import "card" as card;
import "util" as util;

card::atomic_card_characteristics[]
| {
    filename: (.name | util::to_filename) + ".ron",
    contents: (
        .layout as $layout
        | .faces
        | map(@text "'$jq_template'")
        | if length == 1
        then first
        else (($layout // "") | util::capitalize) + "(\n" + (join(",\n") | util::indent_ron("    ")) + ",\n)"
        end
    ),
}'

_jq $jq_filter $data_dir/mtgjson/AtomicCards.json  | while read line
    set parts (string split \t (echo "$line" | _jq '.filename + "\t" + .contents' | string collect))
    set -l filename $parts[1]
    set -l contents $parts[2]

    set -l dest $dest_dir/$filename
    if not is_todo $dest
        continue
    end

    echo "$contents" >$dest
    echo "wrote $dest"
end

