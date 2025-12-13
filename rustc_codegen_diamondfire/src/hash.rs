use rustc_data_structures::stable_hasher::HashStable as _;
use rustc_middle::ty::{
    TyCtxt,
    GenericArg
};
use rustc_span::def_id::DefId;
use rustc_stable_hash::{
    FromStableHash,
    StableSipHasher128,
    SipHasher128Hash
};

pub struct HashingUtil(u128);

impl FromStableHash for HashingUtil {
    type Hash = SipHasher128Hash;

    fn from(hash : Self::Hash) -> Self {
        let upper = hash.0[0] as u128;
        let lower = hash.0[1] as u128;
        Self((upper << 64) | lower)
    }
}


impl HashingUtil {

    pub fn hash_fn_def<'tcx>(tcx : TyCtxt<'tcx>, def_id : DefId, args : impl IntoIterator<Item = GenericArg<'tcx>>) -> u128 {
        let mut hasher = StableSipHasher128::new();
        tcx.with_stable_hashing_context(|mut hcx| {
            def_id.hash_stable(&mut hcx, &mut hasher);
            for arg in args {
                arg.hash_stable(&mut hcx, &mut hasher);
            }
        });
        hasher.finish::<Self>().0
    }

}
