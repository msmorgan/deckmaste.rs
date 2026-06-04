#!/usr/bin/fish

source (path resolve (status dirname)/../lib/setup.fish)
or return

set -l plugin_dir (check_plugin_dir $argv[1])
or return

set -l dest_dir $plugin_dir/keyword_abilities
mkdir -p $dest_dir

create_keyword_todos "keyword_abilities" $dest_dir
