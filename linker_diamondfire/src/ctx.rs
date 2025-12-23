use bridgecg_diamondfire::bridge_items::BridgeItems;
use std::collections::BTreeSet;


pub struct LinkingCtx<'l> {

    bridge_items : &'l BridgeItems,

    fns_to_link : BTreeSet<u128>,
    linked_fns  : BTreeSet<u128>

}

impl<'l> LinkingCtx<'l> {
    pub fn new(bridge_items : &'l BridgeItems) -> Self { Self {

        bridge_items,

        fns_to_link : BTreeSet::new(),
        linked_fns  : BTreeSet::new()

    } }
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
