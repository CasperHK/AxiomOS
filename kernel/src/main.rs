//! Kernel entry point (host build).

fn main() {
    println!("AxiomOS kernel starting (host build)...");
    kernel::init::boot();
}
