#!/usr/bin/fish

source (path resolve (status dirname)/lib/setup.fish)
or return

set -l plugin_dir $argv[1]
if not path is -d $plugin_dir
    echo >&2 "usage: $(status filename) PLUGIN_DIR"
    return 1
end


for migration in (path sort -- $migrations_dir/*.fish)
    fish $migration $plugin_dir
end