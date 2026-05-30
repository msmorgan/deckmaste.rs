#!/usr/bin/env fish

set -l project_dir (path dirname (path dirname (status filename)))
set -l data_dir $project_dir/data
set -l xz_dir $data_dir/xz
set -l raw_dir $data_dir/raw

mkdir -p $xz_dir $raw_dir

# --- Comprehensive Rules ---

echo "Fetching comp rules download URL..."
set -l rules_page (curl -fsSL https://magic.wizards.com/en/rules)
set -l rules_url (string match -r 'https://media\.wizards\.com/\S+MagicCompRules[^"]+\.txt' -- $rules_page)
or begin
    echo >&2 "Could not find comp rules .txt URL on the rules page"
    exit 1
end

set -l rules_file (string replace -r '.*/' '' -- $rules_url | string unescape --style=url)
set -l rules_date (string match -r '\d{8}' -- $rules_file)
set -l dated_name cr-$rules_date.txt

if test -f $data_dir/$dated_name
    echo "Comp rules $dated_name already present, skipping"
else
    echo "Downloading $rules_file..."
    curl -fsSL -o $data_dir/$dated_name -- $rules_url
    or begin
        echo >&2 "Failed to download comp rules"
        exit 1
    end
end

ln -sf $dated_name $data_dir/cr.txt
echo "cr.txt -> $dated_name"

# --- MTGJSON ---

set -l mtgjson_base https://mtgjson.com/api/v5
set -l mtgjson_files \
    AllPrintings.json.xz \
    AllPrintings.psql.xz \
    AllSetFiles.tar.xz \
    AtomicCards.json.xz

for xz_file in $mtgjson_files
    set -l dest $xz_dir/$xz_file
    set -l http_code (curl -fsSL -z $dest -o $dest -w '%{http_code}' -- $mtgjson_base/$xz_file)
    or begin
        echo >&2 "Failed to download $xz_file"
        exit 1
    end

    if test "$http_code" = 304
        echo "$xz_file is up to date, skipping"
        continue
    end

    set -l base (path change-extension '' -- $xz_file)

    if string match -q '*.tar' -- $base
        set base (path change-extension '' -- $base)
        echo "Extracting $xz_file -> raw/$base/"
        rm -rf $raw_dir/$base
        mkdir -p $raw_dir/$base
        tar -xJf $xz_dir/$xz_file -C $raw_dir/$base --strip-components=1
    else
        echo "Extracting $xz_file -> raw/$base"
        xz -dkf $xz_dir/$xz_file
        mv $xz_dir/$base $raw_dir/$base
    end
end

echo "Done."
