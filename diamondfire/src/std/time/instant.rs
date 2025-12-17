use diamondfire_sys::{
    df_number,
    gamevalue::DF_GAMEVALUE__Timestamp
};

pub struct Instant {
    seconds : *const df_number
}

impl Instant {
    pub fn now() -> Instant {
        unsafe {
            let ts = DF_GAMEVALUE__Timestamp() as (*const df_number);
            return Instant { seconds: ts };
        }
    }
}
