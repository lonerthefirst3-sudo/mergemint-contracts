# Event schema

## config_changed

Topic: `(Symbol("config_changed"), Symbol(<key>))`

Data: `(old_value, new_value)`

Emitted by every admin configuration mutation, so an indexer can build a
full audit trail of admin actions from chain data alone. The `key` symbol
identifies which setting changed; the data payload type depends on the
setting:

| key              | value type      | emitted by           |
|------------------|------------------|-----------------------|
| `paused`         | `bool`           | `set_paused`          |
| `fee_bps`        | `u32`            | `set_fee_bps`         |
| `min_reputation` | `u32`            | `set_min_reputation`  |
| `claim_window`   | `u64`            | `set_claim_window`    |
| `verifier`       | `Option<Address>` -> `Address` | `set_verifier` |
| `admin`          | `Option<Address>` -> `Address` | `accept_admin` (completes a `transfer_admin`/`accept_admin` two-step transfer) |
