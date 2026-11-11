//! Builds `mixins/vastspawners_native/logic.rs` on its own, for `cargo test`. In the game it is
//! `horizon_world::vastspawners`, over horizon-world's NBT; here over a small stand-in with the same names.

pub mod nbt;

#[path = "../mixins/vastspawners_native/logic.rs"]
pub mod vastspawners;
