use shadowforge::init_shadowforge;

fn main() {
    init_shadowforge();
    shadowforge::networking::init_mesh_network();
    println!("[SHADOWFORGE] Full system compromise achieved. Enjoy the collapse.");
}
