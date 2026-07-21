pub mod evasion;
pub mod persistence;
pub mod networking;
pub mod evasion;
pub mod persistence;
pub mod networking;

pub use evasion::generate_polymorphic;
pub use persistence::install;
pub use networking::{init_mesh, status};
pub fn initialize() {
    println!("ShadowForge Core initialized.");
    println!("Resilience framework ready for operation.");
}
