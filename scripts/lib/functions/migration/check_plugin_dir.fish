function check_plugin_dir
    argparse -Ss -N1 n/name= -- $argv
    or return

    set name (string collect -- $_flag_name (status function))[1]

    set plugin_dir (path resolve -- $argv[1])
    if not set -q plugin_dir[1]
        echo >&2 "usage: $name PLUGIN_DIR"
        return 1
    end
    if not path is -d $plugin_dir
        echo >&2 "$name: plugin dir $plugin_dir does not exist"
        return 1
    end
    echo $plugin_dir
end