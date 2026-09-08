# Scryfall Oracle Cards ingestion contract

Verified against Scryfall's live API and official documentation on 2026-09-08.
Values under “Observed snapshot” describe that export, not constants to bake into
the downloader.

## Descriptor and transport

Refresh code should request [`GET /bulk-data/oracle-cards`](https://scryfall.com/docs/api/bulk-data/type)
and follow the returned `jsonl_download_uri`. Scryfall says timestamped bulk URLs
change daily; its July 2026 migration notice says `download_uri` was retired on
2026-07-20 and `jsonl_download_uri` is now the only download property.
([bulk-data documentation](https://scryfall.com/docs/api/bulk-data),
[migration notice](https://scryfall.com/blog/two-new-ways-to-sync-scryfall-data-236))

The [live Oracle Cards descriptor](https://api.scryfall.com/bulk-data/oracle-cards)
currently has exactly these fields:

```json
{
  "object": "bulk_data",
  "id": "27bf3214-1271-490b-bdfe-c0be6c23d02e",
  "type": "oracle_cards",
  "updated_at": "2026-09-08T09:01:57.053+00:00",
  "uri": "https://api.scryfall.com/bulk-data/27bf3214-1271-490b-bdfe-c0be6c23d02e",
  "name": "Oracle Cards",
  "description": "A JSON file containing one Scryfall card object for each Oracle ID on Scryfall. The chosen sets for the cards are an attempt to return the most up-to-date recognizable version of the card.",
  "jsonl_download_uri": "https://data.scryfall.io/oracle-cards/oracle-cards-20260908090157.jsonl.gz",
  "compressed_size": 24535940
}
```

There is no `download_uri`, `size`, `content_type`, `content_encoding`, or API
schema-version field. The documented metadata contract is `id`, `uri`, `type`,
`name`, `description`, `updated_at`, `jsonl_download_uri`, and
`compressed_size`; `object` is the live response discriminator.
([field reference](https://scryfall.com/docs/api/bulk-data),
[live descriptor](https://api.scryfall.com/bulk-data/oracle-cards))

The target is a bare gzip file containing JSON Lines: one object per line, with
no enclosing JSON array and no commas between records. It is not a tarball and
gzip is the stored representation, not HTTP `Content-Encoding`. Scryfall
explicitly recommends line/chunk streaming directly from the gzip archive to
avoid decompressing or loading the entire export in memory.
([bulk-data documentation](https://scryfall.com/docs/api/bulk-data),
[migration notice](https://scryfall.com/blog/two-new-ways-to-sync-scryfall-data-236))

Observed response headers agree: the descriptor was
`application/json; charset=utf-8`; the [dated export](https://data.scryfall.io/oracle-cards/oracle-cards-20260908090157.jsonl.gz)
was `application/gzip`, `Content-Length: 24535940`, `Last-Modified: Tue, 08 Sep
2026 09:01:58 GMT`, `ETag: "ac39b9c5682b32b4df599eef5a15f057"`, and had no
`Content-Encoding`. The downloaded gzip passed integrity validation. Its SHA-256
was `070e3b5d9f4e5c9ee0c667f887fcca6afb99d441f284ba2cb0ba85c897adf956a`;
the decompressed JSONL SHA-256 was
`ba4952fd7eae58b49f5dd8277968169902f79f8a289644c1ba2b4376cc53bb22`.

Scryfall generates bulk data once every 12–24 hours and says weekly downloads
or downloads after set releases are probably sufficient for gameplay-only data.
It asks clients to cache/process downloaded data locally for at least 24 hours.
([bulk guidance](https://scryfall.com/docs/api/bulk-data),
[rate-limit guidance](https://scryfall.com/docs/api/rate-limits))

## Request policy

Every request to `api.scryfall.com` must use HTTPS with TLS 1.2 or newer and
must send explicit `User-Agent` and `Accept` headers. The user agent should name
the application and relevant version rather than the HTTP library; Scryfall
accepts `Accept: */*` or `Accept: application/json;q=0.9,*/*;q=0.8`.
([API requirements](https://scryfall.com/docs/api))

The descriptor endpoint falls under “all other methods,” currently limited to
10 requests/second. Direct files on `*.scryfall.io` have no rate limit. An HTTP
429 must cause the client to reduce traffic rather than retry through the
limit; repeated overload can lead to a temporary or permanent block.
([rate limits](https://scryfall.com/docs/api/rate-limits))

## Card and face shape

The Oracle Cards file chooses one recognizable, up-to-date printing for each
Oracle identity. A Card `id` identifies that selected Scryfall printing;
`oracle_id` is stable across reprints and distinguishes different cards that
share a name. Therefore durable corpus identity should use `oracle_id`, while
`id` remains snapshot provenance and may change when Scryfall chooses a new
representative.
([Oracle Cards description](https://scryfall.com/docs/api/bulk-data),
[Card field reference](https://scryfall.com/docs/api/cards#core-card-fields))

Scryfall represents a multiface card as one Card object with a `card_faces`
array. Arrays are ordered, so retain source order and use the face ordinal as
the discriminator within an ordinary shared `oracle_id`; do not sort faces by
name or another field. The API promises at least two face objects, not exactly
two. Parent `name` joins face names with ` // `.
([Scryfall face model](https://scryfall.com/docs/api/cards#multiface-cards),
[Card Face fields](https://scryfall.com/docs/api/cards#card-face-objects),
[JSON array definition](https://www.rfc-editor.org/rfc/rfc8259#section-5))

Field placement is layout-sensitive:

- Single-face cards, including `meld`, carry `oracle_text`, `type_line`,
  `mana_cost`, `cmc`, colors, and any power/toughness at the root. `meld` uses
  root `all_parts`, not `card_faces`.
- `split`, `flip`, and `adventure` carry combined root `name`, `type_line`, and
  `mana_cost`; their individual `type_line`, `mana_cost`, `oracle_text`, and
  stats are in `card_faces`. In the observed export their face `colors` are
  omitted while overall `colors` are at the root.
- `transform` and `modal_dfc` carry combined root `name` and `type_line`, but
  root `oracle_text`, `mana_cost`, and `colors` are absent; each face carries
  its own values. Root `cmc` is still the overall card mana value.
- `mana_cost: ""` means no mana cost and differs from `{0}`. Optional Oracle
  text and combat stats may be missing; power/toughness/loyalty/defense are
  strings because values such as `*` and `X` occur. Preserve missing versus
  empty instead of inventing defaults.

These placements follow Scryfall's
[Card fields](https://scryfall.com/docs/api/cards#gameplay-fields),
[Card Face fields](https://scryfall.com/docs/api/cards#card-face-objects), and
[layout descriptions](https://scryfall.com/docs/api/layouts#card-faces), and
were checked across every `normal`, `meld`, `split`, `flip`, `adventure`,
`transform`, and `modal_dfc` record in the observed export. Every record with
`card_faces` lacked root `oracle_text`, while every face in those ticket-relevant
layouts had `oracle_text`, `type_line`, and `mana_cost` keys.

`type_line` is a single string at both card and face level; Scryfall does not
provide MTGJSON-style type/supertype/subtype arrays. Catalog extraction must
parse the declared type-line structure without tokenizing multiword subtype
names by whitespace. ([Card fields](https://scryfall.com/docs/api/cards#gameplay-fields))

`legalities` is a root object whose values are `legal`, `not_legal`,
`restricted`, or `banned`. `legalities.vintage` was present on every observed
record, so this ticket's supported scope is the explicit union of `legal` and
`restricted`, not “anything other than `not_legal`.”
([legality field](https://scryfall.com/docs/api/cards#gameplay-fields))

Meld relationships live in `all_parts`. A Related Card contains only `id`,
`component`, `name`, `type_line`, and `uri`; `component` is `meld_part` or
`meld_result` for meld membership, and the related object does not carry an
`oracle_id`. Resolve its `id` against full Card records before deriving durable
Oracle identities. Each member of the checked Mishra meld family was a separate
Oracle Cards record with its own root `oracle_id` and the shared relationship.
([Related Card fields](https://scryfall.com/docs/api/cards#related-card-objects),
[official related-card type](https://github.com/scryfall/api-types/blob/main/src/objects/Card/RelatedCard.ts))

Reversible cards are a special Card API shape: the parent lacks `oracle_id`,
and each face carries its own `oracle_id`, `layout`, `cmc`, and other card-level
fields because the sides are unrelated gameplay objects. The live example
shows that the two faces may even have the same Oracle identity.
([layout guidance](https://scryfall.com/docs/api/layouts#card-faces),
[Card field exception](https://scryfall.com/docs/api/cards#core-card-fields),
[live reversible example](https://api.scryfall.com/cards/3e3f0bcd-0796-494d-bf51-94b33c1671e9))

The observed Oracle Cards export contained no `reversible_card` records and no
face-level Oracle IDs: all 38,634 records had distinct root `oracle_id` values.
The decoder should nevertheless retain the documented reversible shape so
validation and future snapshots do not collapse two unrelated face identities.

## Schema policy implied by the sources

The descriptor exposes an update timestamp but no API/schema version. Scryfall's
official TypeScript package states that its semantic versions describe the
package, not the Scryfall API as a whole.
([descriptor](https://api.scryfall.com/bulk-data/oracle-cards),
[official API types README](https://github.com/scryfall/api-types))

The live export also demonstrates why a closed schema cannot be the only guard:
it contains 291 `front_card` records and the current
[layout documentation](https://scryfall.com/docs/api/layouts) lists that value,
while the official package's current
[`ScryfallLayout`](https://github.com/scryfall/api-types/blob/main/src/objects/Card/values/Layout.ts)
does not. Scryfall also warns that bulk exports include every product/card type,
including unsupported planar, scheme, Vanguard, token, and funny cards.
([bulk guidance](https://scryfall.com/docs/api/bulk-data))

Accordingly, the local snapshot contract should record the descriptor body,
`updated_at`, download URI, compressed size, response validators, compressed and
decompressed hashes, decoder/schema-policy revision, and record count. Decode
unknown object fields permissively, keep layout as an extensible value, then
validate all required identity/field-placement invariants and explicitly
classify supported versus unsupported layouts. Missing required fields, duplicate
durable identities, invalid JSON lines, truncated gzip, size/hash mismatches, and
unknown layouts that reach the supported-selection path should fail publication.
This paragraph is an implementation inference from the unversioned, changing
upstream contract rather than a promise made by Scryfall.

## Observed snapshot checks

- 24,535,940 compressed bytes, matching `compressed_size`; 38,634 JSONL records.
- 38,634 distinct root Oracle IDs; no duplicates; every record had
  `legalities.vintage`.
- Vintage status counts: 31,690 `legal`, 52 `restricted`, 101 `banned`, and
  6,791 `not_legal`.
- Ticket-relevant layout counts: 33,334 `normal`, 401 `transform`, 170
  `adventure`, 137 `split`, 100 `modal_dfc`, 26 `flip`, and 21 `meld`.
- The export also contained unsupported or special layouts, including
  `front_card`, planar, scheme, Vanguard, token, emblem, art-series, host, and
  augment records, consistent with Scryfall's warning that bulk data is broader
  than normal Magic.
