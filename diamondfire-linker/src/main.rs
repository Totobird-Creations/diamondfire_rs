use std::env;


fn main() {
    eprintln!("TODO linker:");
    for arg in env::args() {
        eprintln!("  {}", arg);
    }
    todo!()
}
