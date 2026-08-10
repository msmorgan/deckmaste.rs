---
needs: [engine-payment-obligation-window]
---
Replace the temporary monocolor autopayment shim with a client-facing payment
overlay that consumes the engine's complete protocol. Support exact floating-
mana-to-pip coverage (including riders and `PayPips` alternatives), nested mana
ability activation, payer-chosen legal cost order, whole-batch action-cost
choices, `RescindFulfillment`, `DeclinePayment`, `SubmitPayment`, and any
`ChooseManaReversals` decision produced on decline.

Keep convenience policy in the client: automatically collapse forced/nullary
steps and obvious payments, but let the player expand the overlay and override
every legal choice before submission. Show replay barriers and retained actions
without assigning tournament-policy penalties. Network privacy/redaction of an
unsubmitted announcement is a separate concern; this ticket covers the local
interaction model and strategy interface.

