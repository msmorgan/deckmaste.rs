---
needs: [english-v3-generic-frame-consumption]
---
# Distinguish By complements from independent By adjuncts

The attachment-domain audit establishes a function gap, not a safe global By
restriction. Express the internalised Complement of a Bare Passive independently
of scalar-change extent Complements and means Adjuncts. Preserve genuinely
ambiguous analyses when the surface and declared grammatical properties do not
choose a unique function; no Reading-count reduction is required.

Witnesses, with full source context retained:

- Avacyn, Guardian Angel: “Prevent all damage that would be dealt to another
  target creature this turn by sources of the color of your choice.” The
  intended internalised Complement belongs within the Bare Passive headed by
  *dealt*, not to an enclosing modal as its passive participant.
- Spikeshell Harrier: “If that opponent's speed is greater than each other
  player's speed, reduce that opponent's speed by 1.” The entire consequent
  supplies an active scalar-change extent Complement.
- Trusted Advisor: “Your maximum hand size is increased by two.” Passive host
  status does not turn extent into an internalised Complement.
- Noctis, Prince of Lucis: “You may cast artifact spells from your graveyard by
  paying 3 life in addition to paying their other costs.” Preserve the complete
  means Adjunct and its gerund-participial Complement.

The smallest justified design has a declared internalised-Complement marker
selected by Bare Passive hosts, declared lexical-frame licensing for extent,
and an independent adjunct route for means. Reuse existing construction schemas
where those relations have the same shape. Do not classify function solely by
By spelling, passive host status, NP-versus-clause complement, or presumed game
semantics. Replacing every passive By adjunct with an agent-labelled node would
misclassify extent and means. Internalised Complement is a syntactic function,
not an invariant agent semantic role.

Consult full CGEL Ch. 16 §10.1.1, pp. 1427–1431; Ch. 8 §2.2, pp. 673–675 and
§§5.2–5.3, pp. 691–693; Ch. 4 §1.2, pp. 219–228. Independently construct intended
and contrasting values, verify both roundtrip laws, classify each retired
Reading by function, and report exact host/function distributions and supported
identity-level changes. A raw source suffix is not a constituent fixture.
Standard constraints apply.

Sequencing: extent is licensed by a lexical frame, so this follows
`english-v3-generic-frame-consumption`. The findings this ticket rests on are in
the done ticket `english-v3-attachment-domain-audit`. Write the four witnesses
as tests before implementing. The glossary has no
entry for Internalised Complement (it is mentioned only under Bare Passive); add
one with its CGEL authority as part of this landing.

## STOP resolved: inherited scalar extent fixture

The inherited positive fixture “Creatures attack by 2 plus 2.” occurs in
`counts_and_measures_interact_with_nominals_frames_and_prepositions` and
`pp_adjuncts_and_selected_locatives_keep_their_readings`. It assigns scalar
extent to an intransitive host without lexical-frame licensing. CGEL Ch. 8
§§5.2–5.3, pp. 691–693 requires an appropriate lexical host, and the current
English contract excludes invented nonsense instructions as positive witnesses.
The assurance rule requires the same card and outcome when re-spelling a test,
but this fixture has no source card. Those obligations cannot both be discharged
by preserving this fixture's grammatical judgment.

Resolution (orchestrator, 2026-10-06): resolved by the orchestrator on the
user's standing CGEL-authority rule; user review pending. CGEL Ch. 8 §5.3,
p. 692, “Licensing by verb”, identifies the unsupported scalar-extent attachment
as ungrammatical. The lexical-analysis decision excludes invented nonsense as a
positive contract. This ticket deliberately retires that subject; the Assurance
re-spell clause preserves its asserted positive outcome with authentic extent
and arithmetic witnesses. “Same card” cannot apply to a fixture that had none.

Both existing test functions remain, with all other assertions unchanged.
Trusted Advisor's “Your maximum hand size is increased by two.” replaces the
invented positive; each test structurally asserts `SelectedExtentPassive`
containing `ScalarExtentComplement`, including its exact realized extent.
Vitalizing Cascade's full “You gain X plus 3 life.” retains arithmetic
composition. The invented sentence remains only as a Document-level negative,
asserting zero Readings; it is not attributed to any card.

## Landing record

The orchestrator resolution above authorizes the re-spelling. All original
test functions and other assertions are retained. The final gate and supporting checks pass as recorded below.

### PROVE: preservation and structural laws

The supported raw-Oracle census selects all 32,828 faces without a Reading cap.
Baseline and final input fingerprints are identical:
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
The baseline is the claim tree before implementation; both measurements were
made on working change `rkkokprpqkrnlvytuorurtwxsumnqolr`, stamped below with
covered counts and distinct lexical inventory fingerprints.

SHA-256 identities of the complete generated Reading values, including ordered
children and all lexical owner/form/frame/countability choices, establish that
all 162,352 baseline Readings survive unchanged. The final set contains 176,171
Readings: 13,819 additions on 119 face identities, zero losses and zero duplicate
identities. No normalization, spelling equivalence, or preferred-Reading pruning
was applied. The exact baseline declarations and lexical source were extracted
from the claim parent with `jj file show`; their complete independent export
also agrees with every one of the original export's 159,225 completed identities.
All exports and comparison tools remain in `/tmp`.

No supported identity stops being covered. Retiring `MeasuredPreposition`
removes the unlicensed free scalar-extent route, but its baseline supported
construction count is zero: no corpus Reading retirement requires routing.
The two inherited invented fixtures were re-spelled under the recorded STOP
resolution and are never counted as covered Oracle identities.

Every final corpus Reading passes lexical ownership and declaration admission,
byte-exact realization, construction traversal identity, and leaf traversal
identity against materialization traces. Validation issues, cyclic derivations,
duplicate materializations, and internal failures are all zero. Independently
authored values exercise the opposite roundtrip law and compare complete values
and traversal sequences. Four witness tests were written first and failed on
the baseline (zero passed, four failed); the final four pass. These include
valid alternative Avacyn and Noctis attachments and independently authored
invalid active internalised, duplicate internalised, extent-as-adjunct,
wrong-frame, and wrong-marker values.

Forbidden word/card/lexeme-named admission guards: zero added. New guards read
declared Participial Use, marker/complement eligibility, complement-presence,
and lexical Frame properties. Lexical environment loading succeeds with zero
load errors. The active stack has no `environment.rs`; its source-loading
authority is `deckmaste_lexical_source::load_workspace`.

### DISCLOSE: census and analyses

| Tree stamp / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| `rkkokprp` baseline / 13,596 | 19,232 | 6,641 | 6,955 | 162,352 |
| `rkkokprp` final / 13,622 | 19,206 | 6,653 | 6,969 | 176,171 |

Baseline lexical inventory fingerprint:
`3529edcda18b9a654fead1641ec76614ca3c449d2066a0147d337956b031be07`.
Final lexical inventory fingerprint:
`ab63eaed51a42b72ff4c5064b39163e5354f0699ea5d1b4d3d53cfbf1d4cb973`.
Specificity-resolved count remains zero: the current English contract retains
all grammatical Readings. The relevant new function contrast is
`InternalisedComplementPredicate` versus `PrepositionPredicate`;
`SelectedExtentPassive` and `SelectedPredicate` select
`ScalarExtentComplement` without also admitting it as a generic PP Adjunct.
Current v3 tooling emits neither the retired coverage lock nor a legacy
licensing-checker total; the all-face census above supplies covered counts.
There are zero new word-named licensing checkers.

All 26 newly covered face identities are listed below with their admitted
analysis. The gerund PP faces retain independent adjunct and nominal scopes;
these scope alternatives are grammatical ambiguity, not selected game meanings.
Their complete gerund-participial and nested `in addition to` constituents are
preserved. Hand-size gains use the declared compound noun, not a literal form.

| Identity | Card face | New analysis | Readings |
|---|---|---|---:|
| `19504e19-2886-4d28-a813-b798a9a446a2#card` | Alien Symbiosis | By + gerund-participial Complement; independent PP scopes | 475 |
| `d185c829-ef12-403a-a071-7520a0293d4a#card` | Demonic Embrace | By + gerund-participial Complement; independent PP scopes | 575 |
| `cfb5da72-9f00-4ae2-8d59-f997faab451b#card` | Gnat Miser | Bare Passive + selected scalar-change extent | 1 |
| `684b2026-1589-4f59-bd72-5795fae44c1c#card` | Graceful Adept | compound hand size NP in declared Have object frame | 1 |
| `4b216b4d-f9cb-4ce2-af70-c91eef3db083#card` | Helbrute | By + gerund-participial Complement; independent PP scopes | 595 |
| `eb23aed0-c450-4e57-96f2-2866dceca004#card` | Jin-Gitaxias, Core Augur | Bare Passive + selected scalar-change extent | 1 |
| `75a53a88-51a5-4c76-9f67-ee94c3065b98#card` | Locust Miser | Bare Passive + selected scalar-change extent | 1 |
| `9ad4f876-e923-4a09-8e70-eb826aacf89d#card` | Minamo Scrollkeeper | Bare Passive + selected scalar-change extent | 1 |
| `80ea3e65-946d-42c3-a26f-3d74e81d70b4#card` | Morska, Undersea Sleuth | compound hand size NP in declared Have object frame | 2 |
| `c45e96cb-2539-4e00-9fff-782a182c2f4b#card` | Nezahal, Primal Tide | compound hand size NP in declared Have object frame | 38 |
| `2d44bdd2-aa31-415a-be71-d9e98ef12334#card` | Noctis, Prince of Lucis | By + gerund-participial Complement; independent PP scopes | 1,872 |
| `791cbb7c-7935-4902-b050-9ea9d030e9fa#card` | Osteomancer Adept | By + gerund-participial Complement; independent PP scopes | 2,538 |
| `83d8eda7-b8d2-479a-b1ec-de56833a3182#card` | Praetor's Counsel | compound hand size NP in declared Have object frame | 7 |
| `c23e5b80-08d2-4e24-9908-fe2aa4f30f6f#card` | Reliquary Tower | compound hand size NP in declared Have object frame | 1 |
| `52e92296-7af9-4e68-8045-40b6d6b73910#card` | Rona, Sheoldred's Faithful | By + gerund-participial Complement; independent PP scopes | 380 |
| `83133f51-bfbd-4db5-9be0-f660e3b66435#card` | Spellbook | compound hand size NP in declared Have object frame | 1 |
| `d5f31713-d380-42ba-8052-4b8d9beb3958#face:1` | Steaming Sauna | compound hand size NP in declared Have object frame | 1 |
| `8a6eda31-2791-4def-b9e7-3adde155a4f0#card` | The Second Doctor | compound hand size NP in declared Have object frame | 5 |
| `e5106a2d-caca-4519-a1a1-eabaec4005df#card` | Thought Devourer | Bare Passive + selected scalar-change extent | 1 |
| `88e18dc9-8b11-4369-9860-a687925cdcb4#card` | Thought Eater | Bare Passive + selected scalar-change extent | 1 |
| `9a640695-fc17-4ccc-ac88-ce48b6c6ad53#card` | Thought Nibbler | Bare Passive + selected scalar-change extent | 1 |
| `9965d9c5-2ebf-4a6c-930e-55c5890979be#card` | Thought Vessel | compound hand size NP in declared Have object frame | 1 |
| `430e4e74-33ee-49f2-aef5-6d09079c5eb7#card` | Trusted Advisor | Bare Passive + selected scalar-change extent | 2 |
| `f7907327-4df3-4bb2-a274-03d9e266af2f#card` | Venser's Journal | compound hand size NP in declared Have object frame | 9 |
| `730d02c7-2f8a-47d3-99f0-e4ada7f6255c#card` | Wickerfolk Indomitable | By + gerund-participial Complement; independent PP scopes | 212 |
| `d9722ec3-aa5e-4c69-8ff0-cef1b1839fa4#card` | Wisdom of Ages | compound hand size NP in declared Have object frame | 14 |

The supported identity-level addition inventory follows. No identity-level
losses exist. Counts include combinations with existing scope and lexical
alternatives, rather than treating only the intended interpretation as coverage.
The full Avacyn card increases from 33,856 to 39,601 Readings; its retained
baseline set is byte-identical as values.

| Face identity | Card face | Added Readings |
|---|---|---:|
| `0f5b0c77-1e3d-46a1-ae0e-03ed79196cd9#card` | Al-abara's Carpet | 262 |
| `19504e19-2886-4d28-a813-b798a9a446a2#card` | Alien Symbiosis | 475 |
| `bbf183bc-d502-4432-8202-f29f60c08396#card` | Argothian Pixies | 44 |
| `0dbcc5d8-9533-48fd-a69d-bbc6c72aaac5#card` | Argothian Sprite | 1 |
| `f3aaef18-dc32-40d6-b48c-f957aa31247f#card` | Argothian Treefolk | 3 |
| `9bd3a4bb-cc12-4e5f-a33f-77ab0c7788db#card` | Artifact Ward | 220 |
| `e01b3d52-297d-42e3-9de9-4f45c2359bcc#card` | Audacious Infiltrator | 1 |
| `03f5946b-e000-4b39-a49f-1c31668b51bd#card` | Autumn's Veil | 6 |
| `6da0b6d2-3c5e-49ef-9264-076269c04744#card` | Avacyn, Guardian Angel | 5,745 |
| `c640350b-16a5-4227-87fa-8b24fdb367c7#card` | Axelrod Gunnarson | 2 |
| `fc728cd7-2b5b-45f3-896a-a45648bcb61d#card` | Baron Sengir | 1 |
| `969233f8-a644-4297-a2bf-b1ca7a58bb0b#card` | Barrenton Cragtreads | 1 |
| `031ddf1e-afcf-4b09-8859-4df0f0ab5dcf#card` | Blood Cultist | 1 |
| `035d61a9-5ca2-4275-b8df-fcfa9f5c1b31#card` | Bog Rats | 1 |
| `82959ca2-cd96-4cca-9ce0-afb8db209860#face:0` | Bushi Tenderfoot | 1 |
| `2681bfb5-127b-4046-85ac-72be584999cb#card` | Champion Lancer | 3 |
| `65ddd04c-7207-4f6c-ae1b-c5e02a520a51#card` | Chandra's Phoenix | 40 |
| `07c20708-c305-46e7-a98e-c99f54aeafec#card` | Dauthi Horror | 1 |
| `5a6c81b8-71f0-468b-85ad-d87e9a712ecf#card` | Defang | 3 |
| `d185c829-ef12-403a-a071-7520a0293d4a#card` | Demonic Embrace | 575 |
| `8110f092-41b0-4e53-a7d7-f7e3cbc4a52a#card` | Dread Slaver | 120 |
| `6283e5ca-9924-478b-89a5-36924accb9aa#card` | Duskwatch Hunter | 1 |
| `3f9ab49b-81fc-487e-8613-e30afcae164d#card` | Dust Corona | 6 |
| `35ba0e70-da7c-4ed9-b8ec-7dd5c5ce110e#card` | Eerie Interference | 6 |
| `363b2213-4c70-4cb0-b895-81393c1082a7#card` | Elder Spawn | 3 |
| `0cdbd9e7-4941-46ac-99d8-ea227181bdf8#card` | Ethereal Haze | 8 |
| `9911c2f8-fafb-4979-898e-e86b726e3538#card` | Excruciator | 3 |
| `0f909cfd-c643-48d2-983a-da2e38cd2bff#card` | Fen Hauler | 1 |
| `4e16f537-8c36-4bd5-9ae6-0877ccfd983e#card` | Field of Reality | 1 |
| `2d6a387f-7ad4-419e-951a-9dcd4c9ac823#card` | Flow of Maggots | 1 |
| `51e0a61d-69f0-475d-b40b-50d69252800d#card` | Frost Fair Lure Fish | 3 |
| `c5829bff-9687-42b2-8bba-1f036e592cdd#card` | Fylamarid | 2 |
| `17f930a9-64ee-46f9-8a7c-8877e532c94f#card` | Gnat Alley Creeper | 5 |
| `cfb5da72-9f00-4ae2-8d59-f997faab451b#card` | Gnat Miser | 1 |
| `684b2026-1589-4f59-bd72-5795fae44c1c#card` | Graceful Adept | 1 |
| `4b216b4d-f9cb-4ce2-af70-c91eef3db083#card` | Helbrute | 595 |
| `eb23aed0-c450-4e57-96f2-2866dceca004#card` | Jin-Gitaxias, Core Augur | 1 |
| `d3278ddb-c198-450d-946e-8547e5c96dd1#card` | Kick in the Door | 4 |
| `fbb9dafa-b1aa-401a-b26c-c0c7e8b7222f#card` | Kor Castigator | 1 |
| `85e1791f-f9a0-4e82-baf6-33cff2dcf60b#card` | Kothophed, Soul Hoarder | 6 |
| `8fa93c01-f9ea-4d48-a040-418d1ca6be19#card` | Krovikan Vampire | 10 |
| `c34cf404-729d-46ef-8661-1095e5581766#card` | Light of Sanction | 6 |
| `7c7948c6-5144-4df0-8e33-117710ae00af#card` | Lightning Mare | 2 |
| `a8a27a87-fc4c-44b5-9ba8-505bbf42164e#card` | Lithomancer's Focus | 6 |
| `75a53a88-51a5-4c76-9f67-ee94c3065b98#card` | Locust Miser | 1 |
| `e262ea6f-c8c7-45c7-a9a6-29dcbe6300a4#card` | Markov Enforcer | 2 |
| `6ab1bd6f-866b-4665-bd8b-627aafafb94d#card` | Metathran Transport | 2 |
| `9ad4f876-e923-4a09-8e70-eb826aacf89d#card` | Minamo Scrollkeeper | 1 |
| `80ea3e65-946d-42c3-a26f-3d74e81d70b4#card` | Morska, Undersea Sleuth | 2 |
| `d8863b64-110d-4f6b-9d0e-4662f2b399cb#card` | Mudbrawler Raiders | 1 |
| `bdf497b5-166f-46a7-8e18-ae0b8f97768c#card` | Muzzle | 3 |
| `c45e96cb-2539-4e00-9fff-782a182c2f4b#card` | Nezahal, Primal Tide | 38 |
| `2d44bdd2-aa31-415a-be71-d9e98ef12334#card` | Noctis, Prince of Lucis | 1,872 |
| `791cbb7c-7935-4902-b050-9ea9d030e9fa#card` | Osteomancer Adept | 2,538 |
| `70f26726-0373-4e50-a1de-37392ca94890#card` | Ox Drover | 2 |
| `4cd096ac-dd67-4208-a3ad-8e6061b442c6#card` | Plague Mare | 3 |
| `0361b9b7-9097-4aae-aa06-c63a5fb47a76#card` | Pompous Gadabout | 1 |
| `83d8eda7-b8d2-479a-b1ec-de56833a3182#card` | Praetor's Counsel | 7 |
| `a51ec70d-1356-4403-a858-b99445bac54a#card` | Predator Ooze | 1 |
| `1d9b8859-14e3-4bbe-ad92-916b25a37b31#card` | Raiding Party | 264 |
| `302c0400-043d-42fa-abbb-488e520116ba#card` | Rampart Crawler | 1 |
| `5ece1778-fb19-40d5-8bcc-f23033e1d5c0#card` | Rampart Smasher | 2 |
| `170182ed-af1d-442a-8868-d74a96e17f03#card` | Ramses, Assassin Lord | 2 |
| `e405800a-27e6-468a-938b-484346ee20cd#card` | Raven's Run Dragoon | 1 |
| `c23e5b80-08d2-4e24-9908-fe2aa4f30f6f#card` | Reliquary Tower | 1 |
| `ed198c68-c219-469f-b133-737b060cfbfc#card` | Repel the Abominable | 6 |
| `939a6815-bed1-43b0-bf4d-f0581ba0fed5#card` | River Darter | 1 |
| `52e92296-7af9-4e68-8045-40b6d6b73910#card` | Rona, Sheoldred's Faithful | 380 |
| `7409906d-7111-48fa-bfea-8b95e0fdb7dc#card` | Rot Wolf | 1 |
| `3bfa51dd-daed-473a-8377-7bb34a2c3371#card` | Rubblebelt Runner | 1 |
| `fe8f5cc0-5849-437f-8d1f-cb191f2fc638#card` | Sacred Knight | 1 |
| `58616e15-531f-4680-8294-35ab7e962c96#card` | Scarecrow | 17 |
| `9dc26b9f-3b02-46f2-88f8-eb6dffa04ee5#card` | Scythe of the Wretched | 4 |
| `ade8f21a-a7bb-4bd9-a7f2-9fa3060ff1e8#card` | Sengir Bats | 1 |
| `749141aa-f6c4-4ad8-b146-406e68ae9b0b#card` | Sengir Vampire | 1 |
| `a1c50bd1-120d-44ac-af20-13a90fc991e6#card` | Seraph | 80 |
| `e07b6988-9ee7-4f20-8aa5-7dfa87ff507b#card` | Shield Mare | 5 |
| `57188b3d-567c-4dff-8b93-7cc7a47894be#card` | Shieldmage Advocate | 12 |
| `3fea3d1a-067b-4eae-8433-f7e1e7b724a3#card` | Skyblinder Staff | 6 |
| `b7cc831c-c1a1-4ea4-ae78-5be2b6173407#card` | Skyscythe Engulfer | 5 |
| `1492e778-b85e-4675-b48c-bc0a2eb1d105#card` | Sootwalkers | 1 |
| `7b7b3a34-13ff-45d4-ad0d-ac4bad9d0581#card` | Soul Collector | 4 |
| `83133f51-bfbd-4db5-9be0-f660e3b66435#card` | Spellbook | 1 |
| `d5f31713-d380-42ba-8052-4b8d9beb3958#face:1` | Steaming Sauna | 1 |
| `63bc0a02-0d6c-4a00-8b17-1b024f830219#card` | Stone Spirit | 5 |
| `ecfa6791-938a-4159-92bf-ff2b0ce69523#card` | Stonewise Fortifier | 8 |
| `8342de86-5b11-486d-8012-f0f9d2ded8c4#card` | Surge Mare | 4 |
| `e705ec38-b8f7-4f99-8cb9-8448dc3e50ec#card` | Taoist Mystic | 5 |
| `493ecd11-e980-42da-a79e-cddd9a94cc7a#card` | Temple Thief | 3 |
| `cb5ab978-d9b1-4f2f-bf3a-5e28e96b9e2e#card` | Temporal Isolation | 3 |
| `8a6eda31-2791-4def-b9e7-3adde155a4f0#card` | The Second Doctor | 5 |
| `e5106a2d-caca-4519-a1a1-eabaec4005df#card` | Thought Devourer | 1 |
| `88e18dc9-8b11-4369-9860-a687925cdcb4#card` | Thought Eater | 1 |
| `9a640695-fc17-4ccc-ac88-ce48b6c6ad53#card` | Thought Nibbler | 1 |
| `9965d9c5-2ebf-4a6c-930e-55c5890979be#card` | Thought Vessel | 1 |
| `5799baed-3457-4bf2-adf3-239a41dcc1c8#card` | Thwart the Enemy | 12 |
| `7ce28a3c-5f93-4434-918e-1e74f9fc71d8#card` | Time to Reflect | 8 |
| `d3a6bff6-ea85-4e8a-b256-7427d15326b6#card` | Touch of Moonglove | 3 |
| `f9afad36-933f-4d18-acb7-32e5cb5c8161#card` | Tower of Coireall | 3 |
| `df344469-1177-4168-91af-3621b2a65182#card` | Tresserhorn Skyknight | 17 |
| `d2c68c15-761e-4aea-89d8-f7fe8f44905c#card` | Trophy Hunter | 3 |
| `430e4e74-33ee-49f2-aef5-6d09079c5eb7#card` | Trusted Advisor | 2 |
| `44e855a4-95f9-4515-a868-a8ad7a2db751#card` | Uncle Istvan | 3 |
| `d16976b8-4eda-4415-bc16-c20d958b6f59#card` | Unscythe, Killer of Kings | 1 |
| `349a18bf-5c61-4fe6-a5e5-5e7cad6b4ba6#card` | Uphill Battle | 1 |
| `e8941499-2b31-417b-b2d2-10a144826703#card` | Vampiric Dragon | 1 |
| `c7a50ece-4fe4-4e54-9319-b92a4144445d#card` | Vampiric Embrace | 1 |
| `f7907327-4df3-4bb2-a274-03d9e266af2f#card` | Venser's Journal | 9 |
| `af55096c-af6b-479f-8257-144e596e89d8#card` | Vindictive Mob | 1 |
| `499c7ee3-08a3-4f40-809a-3f7087ab9b24#card` | Vine Mare | 1 |
| `e31d4be7-cd24-4287-b8ca-66f8612731a6#card` | Wall of Putrid Flesh | 3 |
| `7e2e96db-e4df-41de-b5a7-712c4a8fe7be#card` | Wanderbrine Rootcutters | 1 |
| `50343947-d127-4479-9c1b-8f5040a97afb#card` | Well-Laid Plans | 17 |
| `1cc00abb-b130-48a2-95f9-f1f986cd75d4#card` | Wicked Akuba | 1 |
| `730d02c7-2f8a-47d3-99f0-e4ada7f6255c#card` | Wickerfolk Indomitable | 212 |
| `0a07a676-890f-49a2-9a2d-17d7b1dc6868#card` | Wight | 3 |
| `d9722ec3-aa5e-4c69-8ff0-cef1b1839fa4#card` | Wisdom of Ages | 14 |
| `8e94ccdb-5978-440b-b7cd-11c7ca6b2c98#card` | You Cannot Pass! | 8 |
| `ae8773a3-05f2-4074-9a53-033b0c127235#card` | Zuo Ci, the Mocking Sage | 5 |

### Exact witness host/function distributions

These totals are for the ticket's complete sentences or complete consequent,
not the complete multi-ability cards. Each witness Reading has one classified
outer By function; nested To in the Noctis means PP is preserved separately.

| Witness | Function / host | Readings |
|---|---|---:|
| Avacyn (199 total) | adjunct / Clause / Prevent | 22 |
| Avacyn (199 total) | adjunct / FinitePredicate / would | 15 |
| Avacyn (199 total) | adjunct / SecondaryVerbPhrase / Prevent | 22 |
| Avacyn (199 total) | adjunct / SecondaryVerbPhrase / be | 10 |
| Avacyn (199 total) | adjunct / SecondaryVerbPhrase / dealt | 15 |
| Avacyn (199 total) | internalised / SecondaryVerbPhrase / dealt | 15 |
| Avacyn (199 total) | postmodifier / Nominal / damage | 25 |
| Avacyn (199 total) | postmodifier / Nominal / turn | 75 |
| Spikeshell consequent (2 total) | extent / active / reduce | 2 |
| Trusted Advisor (1 total) | extent / bare-passive / increased | 1 |
| Noctis (104 total) | adjunct / Clause / You | 10 |
| Noctis (104 total) | adjunct / FinitePredicate / may | 12 |
| Noctis (104 total) | adjunct / SecondaryVerbPhrase / cast | 12 |
| Noctis (104 total) | postmodifier / Nominal / artifact | 14 |
| Noctis (104 total) | postmodifier / Nominal / graveyard | 45 |
| Noctis (104 total) | postmodifier / Nominal / spells | 11 |

Avacyn's 15 new sentence Readings internalise the Complement under lexical
Bare Passive *dealt*; the 184 previous sentence Readings remain. Higher modal
and expanded-passive PP Adjunct alternatives retain their distinct functions.
Spikeshell's two consequent Readings differ in the declared Count/Mass analysis
of Speed. Trusted Advisor's extent is not relabelled an internalised Complement.
Noctis's intended complete means Adjunct and nominal-postmodifier alternative
are independently constructed and survive roundtrip and identity traversal.

The full Spikeshell card remains uncovered: it also needs comparative and
`below` support outside this ticket. The ticket's attested entire consequent
has the required active extent analysis; no raw By suffix is a fixture.
Generic remaining construction and frame breadth stays with the live
`english-v3-systemic-residuals` and `english-v3-frame-coordination` tickets.

### Deviations and additions

- Added `InternalisedComplementPredicate`, carrying presence through secondary
  projections and coordination so its syntactic function belongs to a Bare
  Passive and cannot be duplicated as another internalised Complement.
- Added `GerundComplementPreposition` under declared By and To eligibility.
  To is needed for the complete Noctis witness. Existing Addition, In PP,
  noun postmodification and gerund VP structure express `in addition to`;
  no opaque complex-preposition shortcut is introduced (CGEL Ch. 7 §3.1,
  pp. 618–623).
- Added `ScalarExtentComplement`, `ScalarExtentMeasure`,
  `CardinalExtentMeasure` and `SelectedExtentPassive`; the existing generic
  selected-frame schema handles active extent. Cardinal support is local to
  extent, preserving existing scalar notation and negative fixtures.
- Retired `MeasuredPreposition` and its marker feature because it incorrectly
  admitted a scalar extent as a free PP Adjunct without lexical selection.
- Added ordinary Reduce and Increase paradigms with transitive and selected
  extent frames, Count/Mass Speed, and the declared HandSize compound noun:
  these supply the ticket witnesses through the authored lexical source.
- Added one integration-test file containing the four required tests; no
  per-task evidence, corpus exports or verifier code entered source history.
- Filled the glossary gaps Internalised Complement, Scalar Change Extent
  Complement and Means Adjunct with CGEL authority. No Game Model or
  Comprehensive Rules term or citation was added.

Early provisional implementation allowed scalar extents through generic PP
paths and broadened all scalar numerals to Cardinals. Those invalid paths were
fixed within this ticket before the final corpus run: the final extent category
is selected and Cardinal eligibility is local. They are not counted as baseline
retirements or final coverage gains.

Assurance counts: restored 0; re-spelled 2; added 4 test functions; removed 0;
ignored 0. The two re-spelled functions are
`counts_and_measures_interact_with_nominals_frames_and_prepositions`
(`grammar.rs`) and `pp_adjuncts_and_selected_locatives_keep_their_readings`
(`pp_admission.rs`). Their unsupported any-verb extent subject is deliberately
retired under CGEL Ch. 8 §5.3, p. 692, “Licensing by verb”, and the
lexical-analysis decision's ruling on invented witnesses. Authentic Trusted
Advisor preserves the positive extent outcome; Vitalizing Cascade preserves
arithmetic composition. Every other assertion remains unchanged. The original
invented fixture is retained only as a negative in both functions.
The STOP is resolved by the orchestrator on the user's standing CGEL-authority
rule; user review pending.

### REPORT: economy, inventories, performance and checks

Final `rkkokprp` / covered 13,622: 175 ordinary constructions plus 44 schemas
(219 declaration forms), 134 Categories, 2,758 declaration lines, 537 static
productions and 595 compiled productions. Baseline has 170 ordinary
constructions plus 44 schemas (214 declaration forms). Existing 19 unsupported
lexical frame declarations remain explicit residuals; this ticket does not
invent coverage for them. Form-literal/vocabulary overlap inventory: empty,
verified against all declared quoted literals and lexical surfaces.

Homograph inventory: 120 surfaces, listed by name (provenance, not a gate):

`'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.

Performance advisory, with eight workers in both full runs:

| Tree stamp / covered | Corpus wall nanoseconds | Checked-text thread CPU | Host load (1 / 5 / 15 min) |
|---|---:|---:|---|
| `rkkokprp` baseline / 13,596 | 380,732,937,300 ns | 1,283,286 ns/B | 26.80 / 20.85 / 21.46 |
| `rkkokprp` final / 13,622 | 591,878,934,114 ns | 1,499,803 ns/B | 27.76 / 33.85 / 29.79 |

Both exceed the `16.26s` quiet-host advisory ceiling on this busy host; these
measurements are disclosed and do not establish quiet-host compliance.
Final source bytes are 5,204,548; checked-text bytes are 1,093,492.
The selector iteration used Trusted Advisor, Noctis, Spikeshell and Vitalizing
Cascade: four supported faces, zero issues; Spikeshell remains the disclosed
full-card residual. Final verification used
`cargo xtask english-v3 --all --workers 8 --samples-per-face 0`.

The required derived gate is
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`,
followed by the same package closure's `cargo clippy --all-targets -- -D warnings`.
The full `cargo xtask gate --changed --run --clippy` exits zero after the
orchestrator's re-spelling resolution: 672 tests pass, zero fail, and one
pre-existing test is ignored. The gate includes all four inherited Lean gate
integration tests and the all-targets Clippy closure with warnings denied.
The unchanged ignored test is
`macros::templates::tests::macro_schema_census_count_matches_21`, whose existing
attribute states “cross-checks the live corpus against the census; run on
demand”; this landing introduces zero ignores. The final authored witness suite
passes all four tests. `cargo fmt --all -- --check` passes. `kata kanban check`
reports no duplicates, cycles or dangling needs.

Citation validation reports zero non-compliant strings and zero stale citations
across 16,047 sites. `jj diff --git | cargo xtask cite audit --diff` finds zero
changed Comprehensive Rules citation sites; no bless is required.
