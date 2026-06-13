set -q __setup_done; and return
set -g __setup_done

set -l deps_ok
for program in curl $argv
    if not command -q $program
        echo >&2 "setup.fish: missing needed program $program"
        set -e deps_ok
    end
end
set -q deps_ok; or return 1


set -g project_dir (path resolve (status dirname)/../..)

set -g data_dir $project_dir/data
set -g mtgjson_dir $data_dir/mtgjson
set -g rules_dir $data_dir/rules
set -g catalogs_dir $data_dir/catalogs

set -g scripts_dir $project_dir/scripts
set -g lib_dir $scripts_dir/lib

set -p fish_function_path $lib_dir/functions/fetch
