use core::{
    intrinsics::abort,
    panic::PanicInfo
};


pub mod string;
pub mod vec;


pub mod prelude {
    pub use super::string::{
        String,
        ToString
    };
    pub use super::vec::Vec;
}


#[panic_handler]
fn handle_panic(_info : &PanicInfo) -> ! {
    abort();
}
