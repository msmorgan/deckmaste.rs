function is_todo
    argparse -N1 -- $argv
    or return

    not path is $argv[1]
    or rg -q '^\s*Todo\(' $argv[1]
end