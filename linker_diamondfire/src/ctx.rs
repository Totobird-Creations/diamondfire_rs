use bridgecg_diamondfire::{
    extern_names::{
        ExternNameMap,
        ExternName
    },
    bridge_items::BridgeItems
};
use std::collections::BTreeSet;


pub struct LinkingCtx<'l> {

    extern_names : &'l ExternNameMap,
    bridge_items : &'l BridgeItems,

    fns_to_link : BTreeSet<u128>,
    linked_fns  : BTreeSet<u128>

}

impl<'l> LinkingCtx<'l> {
    pub fn new(extern_names : &'l ExternNameMap, bridge_items : &'l BridgeItems) -> Self { Self {

        extern_names,
        bridge_items,

        fns_to_link : BTreeSet::new(),
        linked_fns  : BTreeSet::new()

    } }
}

impl<'l> LinkingCtx<'l> {
    pub fn lookup_extern(&self, extern_name : &str) -> &'l ExternName {
        let Some(entry) = self.extern_names.names.get(extern_name)
            else { panic!("No extern with name {:?}", extern_name); };
        entry
    }
}

impl LinkingCtx<'_> {

    pub fn queue_link_fn(&mut self, fn_id : u128) {
        if (! self.linked_fns.contains(&fn_id)) {
            self.fns_to_link.insert(fn_id);
        }
    }

    pub fn pop_queued_fn(&mut self) -> Option<u128> {
        let fn_id = self.fns_to_link.pop_last()?;
        assert!(self.linked_fns.insert(fn_id));
        Some(fn_id)
    }

}
