use std::collections::BTreeSet;


#[derive(Default)]
pub struct LinkingCtx {

    fns_to_link : BTreeSet<u128>,
    linked_fns  : BTreeSet<u128>

}

impl LinkingCtx {

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
