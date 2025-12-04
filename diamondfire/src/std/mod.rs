use core::{
    intrinsics::abort,
    panic::PanicInfo
};


// TODO: collections::HashMap
// TODO: collections::LargeHashMap
// TODO: string::String
// TODO: string::ToString
// TODO: sync::Arc
// TODO: sync::Mutex
// TODO: sync::RwLock
// TODO: thread::sleep
// TODO: thread::spawn
// TODO: thread::yield_now
// TODO: time::Instant
// TODO: time::UNIX_EPOCH
// TODO: vec::Vec
// TODO: vec::LargeVec


pub mod prelude {
}


#[panic_handler]
fn handle_panics(_info : &PanicInfo) -> ! {
    abort();
}
