#[cfg(feature = "miniwasm")]
mod miniwasm;
#[cfg(feature = "miniwasm")]
pub use miniwasm::*;

#[cfg(not(feature = "miniwasm"))]
pub use osmosis::*;
#[cfg(not(feature = "miniwasm"))]
mod osmosis;
