use std::cfg_select;

cfg_select! {
    feature = "unstable_sized_hierarchy" => {
        pub(crate) use std::marker::{MetaSized, PointeeSized};
    }
    _ => {
        /// Shim for [`std::marker::PointeeSized`] bounds.
        pub trait PointeeSized {}
        /// Shim for [`std::marker::MetaSized`] bounds.
        pub trait MetaSized {}

        impl<T: ?Sized> PointeeSized for T {}
        impl<T: ?Sized> MetaSized for T {}
    }
}

/// The sized hierarchy bound required by `Deref::Target`.
pub(crate) use MetaSized as DerefSized;
