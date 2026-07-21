use std::fs;

pub fn install() {
    println!("[Persistence] Installing system hooks...");

    match fs::write("/tmp/shadowforge.lock", b"active") {
        Ok(_) => println!("[+] Persistence layer installed successfully."),
        Err(e) => println!("[!] Warning: {}", e),
    }
}
