use diamondfire_sys::{
    df_opaque,
    df_string,
    DF_VAR__NewSession,
    DF_VAR__NameOf,
    DF_ACTION__setSPECIALSpace_variable__PurgeVars
};


static mut GLOBAL_ALLOC_COUNTER : u64 = 0;


#[inline(always)]
pub unsafe fn alloc() -> *mut u8 { unsafe {
    GLOBAL_ALLOC_COUNTER += 1;
    todo!("alloc can not be implemented until strings are")
    // DF_VAR__NewSession("dfrs.malloc.".to_string() + GLOBAL_ALLOC_COUNTER.to_string()) as *mut u8
} }

#[inline(always)]
pub unsafe fn dealloc(ptr : *mut u8) { unsafe {
    DF_ACTION__setSPECIALSpace_variable__PurgeVars(
        df_string::from_str("Entire name"),
        df_string::from_str("False"),
        DF_VAR__NameOf(ptr as *mut df_opaque)
    );
} }
