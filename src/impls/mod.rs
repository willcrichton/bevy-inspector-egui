mod bevy_impls;
mod list;
mod number;
mod primitives;
mod vec;
pub(crate) mod with_context;

pub use bevy_impls::ColorAttributes;
pub use number::NumberAttributes;
pub use primitives::StringAttributes;
pub use vec::Vec2dAttributes;

#[cfg(feature = "rapier")]
mod rapier;
