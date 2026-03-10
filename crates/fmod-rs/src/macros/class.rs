macro_rules! opaque_type {
    {
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident $(;)?
    } => {
        ::std::cfg_select! {
            feature = "unstable_extern_type" => {
                unsafe extern "C" {
                    $(#[$meta])*
                    $vis type $Name;
                }
            }
            _ => {
                $(#[$meta])*
                $vis struct $Name {
                    _data: ::std::cell::Cell<[u8; 0]>,
                    _marker: ::std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
                }
            }
        }
    };
}

macro_rules! fmod_class {
    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        class $prefix:literal $Name:ident {
            type Raw = $Raw:ty;
            fn release = $release:expr;
        }
    } => {
        opaque_type! {
            #[doc = $doc]
            $(#[$meta])*
            pub struct $Name;
        }

        unsafe impl ::std::marker::Send for $Name {}
        unsafe impl ::std::marker::Sync for $Name {}
        impl ::std::panic::UnwindSafe for $Name {}
        impl ::std::panic::RefUnwindSafe for $Name {}
        impl ::fmod::handle::Sealed for $Name {}
        unsafe impl ::fmod::Resource for $Name {
            type Raw = $Raw;

            #[inline(always)]
            fn cast_from_raw(this: *mut Self::Raw) -> *mut Self {
                this as *mut Self
            }

            #[inline]
            #[allow(clippy::redundant_closure_call)]
            unsafe fn release(this: *mut Self::Raw) -> fmod::Result {
                ::std::ptr::drop_in_place(Self::cast_from_raw(this));
                ffi!(($release)(this))?;
                Ok(())
            }
        }

        impl ::std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, concat!($prefix, stringify!($Name), "({:p})"), ::fmod::Resource::as_raw(self))
            }
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        class $prefix:literal $Name:ident {
            type Raw = $Raw:ty;
            fn release = $release:expr;
        }

        mod $($module:ident),+;
        $($rest:tt)*
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class $prefix $Name {
                type Raw = $Raw;
                fn release = $release;
            }
        }

        ::paste::paste! {
            #[doc = $doc]
            pub mod [<$Name:snake>] {
                $(mod $module;)+
                pub use super::$Name;
                #[allow(unused_imports)]
                pub use /*self::*/{
                    $($module::*,)+
                };
                $($rest:tt)*
            }
            #[allow(unused_imports)]
            pub use self::[<$Name:snake>]::*;
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        class $prefix:literal $Name:ident {
            type Raw = $Raw:ty;
            fn release = $release:expr;
        }

        mod;
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class $prefix $Name {
                type Raw = $Raw;
                fn release = $release;
            }
        }

        ::paste::paste! {
            #[doc = $doc]
            pub mod [<$Name:snake>];
            pub use self::[<$Name:snake>]::*;
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        class $Name:ident = $Raw:ident;
        $(mod $($module:ident),*;)?
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class "fmod::" $Name {
                type Raw = ::fmod::raw::$Raw;
                fn release = $Name::raw_release;
            }
            $(mod $($module),*;)?
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        weak class $Name:ident = $Raw:ident;
        $(mod $($module:ident),*;)?
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class "fmod::" $Name {
                type Raw = ::fmod::raw::$Raw;
                fn release = |_| ::fmod::raw::FMOD_OK;
            }
            $(mod $($module),*;)?
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        class studio::$Name:ident = $Raw:ident;
        $(mod $($module:ident),*;)?
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class "fmod::studio::" $Name {
                type Raw = ::fmod::raw::$Raw;
                fn release = $Name::raw_release;
            }
            $(mod $($module),*;)?
        }
    };

    {
        #[doc = $doc:expr]
        $(#[$meta:meta])*
        weak class studio::$Name:ident = $Raw:ident;
        $(mod $($module:ident),*;)?
    } => {
        fmod_class! {
            #[doc = $doc]
            $(#[$meta])*
            class "fmod::studio::" $Name {
                type Raw = ::fmod::raw::$Raw;
                fn release = |_| ::fmod::raw::FMOD_OK;
            }
            $(mod $($module),*;)?
        }
    };
}
