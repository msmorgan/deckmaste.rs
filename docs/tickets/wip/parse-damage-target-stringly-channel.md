---
needs: []
---
**`damage_target` returns a stringly-typed channel that conflates two different
node kinds, and its caller discriminates them by prefix-sniffing its own emitted
RON.** The parser writes a string, then re-parses that string a few lines later
to decide what it meant.

`crates/deckmaste_migrations/src/parsers/effect.rs:2293-2348`, `damage_target`,
returns `(Vec<String>, String)` where the second `String` is *either* a
`Reference` (`"It"` for a targeted/self patient, `"You"`) *or* a `Selection`
(`"SelectAll(<filter>)"` for a "to each / to all" mass patient):

```rust
"any target"   => (vec!["AnyTarget".to_owned()], "It".to_owned()),
"each creature" => (Vec::new(), "SelectAll(Creature)".to_owned()),
```

The caller, `parse_deal_damage` (`effect.rs:1908-1936`), then recovers that
distinction by string-sniffing:

```rust
if amount == "X" && !selection.starts_with("SelectAll(") { return None; }   // :1912
let effect = match selection.strip_prefix("SelectAll(").and_then(|s| s.strip_suffix(')')) {
    Some(filter) => format!("DealsDamageToEach({amount}, {filter})"),        // :1933 macro
    None         => format!("DealDamage(This, {amount}, {selection})"),      // :1934 verb
};
```

The kind (target-`Reference` vs each-`Selection`) is known at the point
`damage_target` produces it, then thrown away into a `String`, then reconstructed
by `starts_with("SelectAll(")` / `strip_prefix("SelectAll(")`. Same anti-pattern
as `type_noun_phrase` (`effect.rs:~1750`), which rejects by sniffing its own
output for `"Subtype("`.

## Fix

Return a small enum from `damage_target` instead of a bare `String`, so the
caller matches a variant and never re-parses:

```rust
enum DamagePatient {
    Target(Vec<String>),   // targeted / self: rides the verb as a Reference
    EachOf(String),        // filter: the DealsDamageToEach macro's recipient
    Ref(String),           // "you", etc.
}
```

Then `parse_deal_damage` matches: `EachOf(filter)` → `DealsDamageToEach`, the rest
→ `DealDamage`, and the X-damage gate keys on the variant, not `starts_with`. Fold
the sibling `type_noun_phrase` self-sniff into the same discipline (return the
bare-subtype-head signal as a flag/enum from `filter::parse_phrase`, not a
re-parsed string).

Note the interplay with [[parse-positional-target-reads]]: that ticket changes the
targeted patient's *spelling* (`It` → `Target(n)`); this ticket changes its
*channel* (String → variant). Compatible and mutually reinforcing — the enum makes
emitting `Target(n)` at the source a one-line arm.

Verify: the existing `damage_target`/`parse_deal_damage` unit tests
(`effect.rs:2351+`) pin the same emitted RON (`DealDamage(...)` for targeted/self,
`DealsDamageToEach(...)` for each-shapes, X gated to mass only); `cargo xtask
generate plugins/wizards` emits burn cards unchanged; `cargo test --workspace`
green.
