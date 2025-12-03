use crate::std::{
    string::{
        String,
        ToString
    },
    vec::Vec
};


#[repr(transparent)]
pub struct PlayerSel {
    uuids : Vec<String>
}

impl PlayerSel {
    pub fn send_actionbar<S : ToString>(&self, s : S) { unsafe {
        DIAMONDFIRE_SelectObject_PlayerName(&self.uuids);
        DIAMONDFIRE_PlayerAction_ActionBar_Selection(s);
        DIAMONDFIRE_SelectObject_Reset();
    } }
}


unsafe extern "C" {
    unsafe fn DIAMONDFIRE_SelectObject_PlayerName(...);
    unsafe fn DIAMONDFIRE_SelectObject_Reset();
    unsafe fn DIAMONDFIRE_PlayerAction_ActionBar_Selection(...);
}
