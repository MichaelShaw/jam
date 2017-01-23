pub mod engine;
pub mod load;
pub mod context;

// blend speed for persistent sounds, in, out?

pub type Listener = self::context::Listener;

pub use self::context::SoundEvent;