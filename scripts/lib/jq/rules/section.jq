# section.jq -- emit the full text of a comprehensive-rules section.
#
# Input : the parsed cr.pretty.json object (keyed by rule number).
# Arg   : --arg rule "<rule number>"   (e.g. 100.1, 702.6, 205.3)
# Output: the text of $rule followed by every one of its subsections,
#         formatted the way they appear in cr.txt.
#
#   jq -r --arg rule 100.1 -f scripts/jq/rules/section.jq cr.pretty.json
#
# Rather than scanning every key for the $rule prefix, we walk the document's
# own `navigation.nextRule` linked list with the builtin `while(cond; update)`:
# starting at $rule we keep stepping to the next rule and stop the moment we
# land on one that no longer `startswith($rule)`. Because the chain is in
# document order, this naturally stays inside the section and -- unlike a
# prefix scan over keys -- never mistakes "100.10" for a subsection of "100.1".
# (`while`, `startswith`, `test`, string interpolation and `join` are all jq
# builtins, no custom function required.)

. as $cr
| [ $rule
    | while(. != null and startswith($rule); $cr[.].navigation.nextRule)
    | $cr[.]
    | select(. != null)
  ]
| map(
    # numbered rules ("100.2") get a trailing dot; lettered subrules
    # ("100.2a") do not -- matching the cr.txt layout.
    ( if .ruleNumber | test("[a-z]$")
      then "\(.ruleNumber) "
      else "\(.ruleNumber). "
      end
    )
    + .ruleText
    + ( (.examples // []) | map("\nExample: " + .) | add // "" )
  )
| join("\n\n")
