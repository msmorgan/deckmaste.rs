function derive_set_order -a mtgjson_dir output_file
    mkdir -p (path dirname $output_file)

    set set_list $mtgjson_dir/SetList.json

    set filters \
        '.data' \
        'sort_by(.releaseDate)' \
        '.[]' \
        'select([.type == $bad_types[]] | any | not)' \
        '.code'

    # set bad_set_types \
    #     alchemy arsenal from_the_vault memorabilia minigame \
    #     premium_deck spellbook token treasure_chest vanguard
    # set bad_set_types (jq -ncr '$ARGS.positional' --args $bad_set_types)
    set bad_set_types '[]'

    set sorted_sets (jq -cr (string join '|' -- $filters) $set_list \
        --argjson bad_types $bad_set_types)

    printf '%s\n' $sorted_sets >$output_file
end

function derive_new_keywords -a mtgjson_dir set_order_file out_dir
    mkdir -p $out_dir

    set all_file $out_dir/_all.txt
    echo >$all_file
    
    cat $set_order_file | while read set_code
        set this_set_json $mtgjson_dir/AllSetFiles/$set_code.json
        set this_set_keywords_file $out_dir/$set_code.txt
        jq -cr '.data.cards[] | select((.legalities.vintage // "Banned") != "Banned") | .keywords[]?' $this_set_json | sort -u >$this_set_keywords_file
        
        set new_keywords (comm -2 -3 $this_set_keywords_file $all_file)
        if test (count $new_keywords) -gt 0
            string join \n -- $new_keywords >$this_set_keywords_file
            cat $this_set_keywords_file $all_file | sort -u | sponge $all_file
        else
            rm $this_set_keywords_file
        end
    end
end

function derive_new_subtypes -a mtgjson_dir set_order_file out_dir
    mkdir -p $out_dir
end

set -l project_dir (path resolve (status dirname)/..)
set -l data_dir $project_dir/data
set -l json_dir $project_dir/json
set -l ron_dir $project_dir/ron

set -l mtgjson_dir $data_dir/mtgjson
set -l set_order_file $data_dir/derived/set_order.txt


derive_set_order \
    $mtgjson_dir \
    $set_order_file

derive_new_keywords \
    $mtgjson_dir \
    $set_order_file \
    $data_dir/derived/keywords

derive_new_subtypes \
    $mtgjson_dir \
    $set_order_file \
    $data_dir/derived/subtypes

return
#
#
# set -l ap_json_xz $data_dir/xz/AllPrintings.json.xz
# set -l ap_json $data_dir/raw/AllPrintings.json
#
# test $ap_json -ot $ap_json_xz
# and xzcat <$ap_json_xz >$ap_json
# and echo "extracted $ap_json"
#
# set -l asf_tar_xz $data_dir/xz/AllSetFiles.tar.xz
# set -l asf_dir $data_dir/raw/AllSetFiles
#
# test $asf_dir -ot $asf_tar_xz
# and tar -C (path dirname $asf_dir) -xJf $asf_tar_xz
# and echo "extracted $asf_dir"
#
# if test $asf_dir -nt $data_dir/sorted_set_codes.txt
#     jq -cr '[inputs.data] | sort_by(.releaseDate)[].code' $asf_dir/*.json >$data_dir/sorted
#
# for code in $sorted_codes
#     echo >&2 "Processing $code..."
#     set -l filters \
#         '.data.cards[]' \
#         'select(.isReprint | not)' \
#         'select((.legalities.vintage // "Banned") != "Banned")' \
#         $argv[1] \
#         'group_by(.number)' #\
#     #'{number, name, type, text, power, toughness, loyalty, defense, scryfallOracleId, keys: (. | keys | join(","))}'
#
#     jq -cr (string join ' | ' -- $filters) $asf_dir/$code.json
# end
#
