use crate::ty::*;


unsafe extern "C" {

    /// Creates a new local (`LINE`) variable by name, returning a pointer to it.
    pub unsafe fn DF_VAR__NewLocal(name : *const df_string) -> *mut df_opaque;

    /// Creates a new thread-local (`LOCAL`) variable by name, returning a pointer to it.
    pub unsafe fn DF_VAR__NewThreadLocal(name : *const df_string) -> *mut df_opaque;

    /// Creates a new session (`GAME`) variable by name, returning a pointer to it.
    pub unsafe fn DF_VAR__NewSession(name : *const df_string) -> *mut df_opaque;

    /// Creates a new persistent (`SAVE`) variable by name, returning a pointer to it.
    pub unsafe fn DF_VAR__NewPersistent(name : *const df_string) -> *mut df_opaque;

    /// Returns the name of the variable behind a pointer.
    pub unsafe fn DF_VAR__NameOf(ptr : *mut df_opaque) -> *mut df_string;

}
