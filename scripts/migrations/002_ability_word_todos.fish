#!/usr/bin/fish

source (path resolve (status dirname)/../lib/setup.fish)
or return

set -l plugin_dir (check_plugin_dir -n (status filename) $argv[1])
or return

set -l dest_dir $plugin_dir/ability_words
mkdir -p $dest_dir

create_keyword_todos ability_words $dest_dir