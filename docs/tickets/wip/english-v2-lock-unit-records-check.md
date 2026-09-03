Compare the lock's per-unit normalization records in `--check`
(lock-drift landing review M1). `--bless` writes 32,641 per-unit records
(schema 4) but the Check arm never compares them against the freshly
built `replacement.normalization_units`: a lock with the vector deleted,
or with 100 falsified card names and flipped onsets, passes exit 0 and
then mis-localizes real drift (zero lines, or 50 spurious "context onset
changed" lines). Add the equality check in the Check arm (the replacement
is constructed one line earlier); a mismatch fails naming `--bless` and
the first differing unit; add regressions for the deleted-vector and
falsified-row cases. Zero identity change; standard constraints apply.
