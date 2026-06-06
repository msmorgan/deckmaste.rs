# Input: a possibly null value
def and_then(f): .
    | if . == null then null
    else f
    end
;

# Input: text (string)
def capitalize: .
    | (.[:1] | ascii_upcase) + .[1:]
;

# Input: text (string)
def strip_parentheticals: .
    | split("\n")
    | map(
        gsub(" ?[(][^)]+[)](?<s> ?)"; "\(.s)")
    )
    | join("\n")
;

# Input: text (string)
def to_rust_ident: .
    | split(" ")
    | map(capitalize)
    | join("")
    | gsub("[^a-zA-Z0-9]"; "")
;

# Input: text (string)
def to_card_filename: .
    | gsub("/"; "|")
;

# Input: text (string)
def to_filename: .
    | explode
    | map(
        if [. == ("<>:\"/\\|?*" | explode)[]] | any
        then . + ("\uff02" | explode)[0] - ("\"" | explode)[0]
        else .
        end
    )
    | implode
;

# Input: text (a RON string). Indent every line by `indent`, leaving the
# bodies of multi-line raw strings (r#" … "#) untouched.
def indent_ron($indent): .
    | reduce split("\n")[] as $line (
        { lines: [], inraw: false };
        .lines += [if .inraw or ($line == "") then $line else $indent + $line end]
        | if (.inraw | not) and ($line | test("r#\"$")) then .inraw = true
        elif .inraw and ($line | test("^\"#")) then .inraw = false
        else . end
    )
    | .lines
    | join("\n")
;