Close the provider-helper namespace hole for all features
(compiler-onset landing review M1). The generated-name reservation for
latent provider helpers is guarded by `feature == Feature::Onset`, so the
other nine provider features keep the exact defect the new test pins: a
latent `NominalForm` provider plus an authored `nominal_form_for_source`
element generates with NO error where `onset_for_source` is now rejected.
Reserve for every provider feature (drive the reservation from the
feature list, not a match arm); extend the namespace test over all ten.
Also: five of six new latent helpers have no caller and are hidden by the
generated modules' blanket `#![allow(dead_code)]` — emit helpers only for
features some construction reads, or pin the latent-emission contract in
an english_v2 test (today reverting the arm passes the whole suite).
Zero grammar change; standard constraints apply.
