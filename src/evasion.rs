use rand::Rng;

pub fn generate_polymorphic(base: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut payload = base.to_vec();

    for byte in payload.iter_mut() {
        if rng.gen_bool(0.6) {
            *byte = byte.wrapping_add(rng.gen_range(1..=64));
        }
        if rng.gen_bool(0.4) {
            *byte ^= 0xAA;
        }
    }
    payload
}

pub fn activate() {
    println!("[Evasion] Polymorphic layer activated.");
}
