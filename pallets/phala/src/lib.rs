#![cfg_attr(not(feature = "std"), no_std)]

/// Phala Pallets
///
/// This is the central crate of Phala tightly-coupled pallets.
///
/// - `phala_legacy`: The legacy `pallet-phala`; will be retired gradually
/// - `mq`: The message queue to connect components in the network
/// - `registry`: Manages the public key of offchain components (i.e. workers and contracts)
///
/// # Status
///
/// Now `phala-pallet` is still functional, but it will be gradually deconstructed as separate
/// pallets.
///
/// The `mq` pallet has a basic implementation and it's supposed to work. The `registry` pallet
/// only has very basic API for testing.
pub mod mq;
pub mod phala_legacy;
pub mod registry;

// Alias
pub use mq as pallet_mq;
pub use phala_legacy as pallet_phala;
pub use registry as pallet_registry;
