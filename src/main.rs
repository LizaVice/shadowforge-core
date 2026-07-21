use shadowforge::{evasion, persistence, networking};

fn main() {
    println!("=== ShadowForge Core v0.1 ===");
    shadowforge::initialize();

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "persist" => persistence::install(),
            "evade" => {
                evasion::activate();
                let payload = evasion::generate_polymorphic(b"test_payload");
                println!("[+] Generated payload: {} bytes", payload.len());
            }
            "mesh" => networking::init_mesh(),
            _ => println!("Usage: shadowforge [persist|evade|mesh]"),
        }
    }
}
