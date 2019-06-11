# Serde support for juniper::ID

This crate allows you to derive `Serialize` and `Deserialize` the `juniper::ID` type.

## Obsolete

This crate will become obsolete once a new version of Juniper is released, containing commit
[3456786](https://github.com/graphql-rust/juniper/commit/3456786463e6e24adaa601aa0dcf73e47dc09fcd).
As of version `0.12.0`, this supporting crate is still required.

## Example

```rust
use juniper::ID;
use serde::{Serialize, Deserialize};

#[Serialize, Deserialize]
struct Car {
    #[serde(with = "juniper_serde")]
    id: ID,

    model: String
}
```
