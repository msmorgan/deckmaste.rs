Make coverage-lock drift a check failure. `coverage --check` fails only
on lost identities; unblessed `newly covered` identities are reported and
return Ok (coverage_lock.rs ~:361-378), which is why three of the last
six landings left the lock behind (328, 1,097, and 2 identities). Add-only
bless doctrine means every landing must bless; enforce it: `--check`
exits non-zero on any newly covered identity, with the message naming
`--bless`. Keep the retire path exactly as authenticated. Also restore
the two identities the parenthetical-guard landing retired if its review
confirms they still select on the integrated tree (they appear as newly
covered today). Standard constraints apply.
