use crate::core::marker::PointeeSized;


#[lang = "legacy_receiver"]
#[doc(hidden)]
pub trait LegacyReceiver : PointeeSized { }
impl<T : PointeeSized> LegacyReceiver for &T { }
impl<T : PointeeSized> LegacyReceiver for &mut T { }
