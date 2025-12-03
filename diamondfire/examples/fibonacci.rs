#![no_std]
#![no_main]


use diamondfire_bind::prelude::*;


#[unsafe(no_mangle)]
#[expect(non_snake_case)]
pub fn DIAMONDFIRE_PlayerEvent__Join(default : PlayerSel) {
    let mut a = 1usize;
    let mut b = 1usize;
    for _i in 0..10 {
        let c = a + b;
        default.send_actionbar(a.to_string());
        a = b;
        b = c;
    };
}
