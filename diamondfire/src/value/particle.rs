use diamondfire_sys::df_particle;


#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Particle {
    _opaque : df_particle
}

impl Particle {
    #[inline(always)]
    pub fn raw(self) -> df_particle { self._opaque }
    #[inline(always)]
    pub fn from_raw(raw : df_particle) -> Self { Self { _opaque : raw } }
}
