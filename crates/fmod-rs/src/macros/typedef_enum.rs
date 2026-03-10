macro_rules! fmod_typedef {
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(::zerocopy::FromBytes, ::zerocopy::IntoBytes)]
        #[derive(::zerocopy::KnownLayout, ::zerocopy::Immutable)]
        $vis struct $Name {
            raw: $Raw,
        }

        impl $Name {
            $(
                strip_default_attr! {
                    $(#[$($vmeta)*])*
                    #[allow(non_upper_case_globals)]
                    pub const $Variant: Self = Self::from_raw($value);
                }
            )*
        }

        impl $Name {
            raw! {
                pub const fn zeroed() -> $Name {
                    Self::from_raw(0)
                }
            }
            raw! {
                pub const fn from_raw(raw: $Raw) -> $Name {
                    Self { raw }
                }
            }
            raw! {
                pub const fn from_raw_ref(raw: &$Raw) -> &$Name {
                    unsafe { &*(raw as *const $Raw as *const $Name ) }
                }
            }
            raw! {
                pub fn from_raw_mut(raw: &mut $Raw) -> &mut $Name {
                    unsafe { &mut *(raw as *mut $Raw as *mut $Name ) }
                }
            }
            raw! {
                pub const fn into_raw(self) -> $Raw {
                    self.raw
                }
            }
            raw! {
                pub const fn as_raw(&self) -> &$Raw {
                    &self.raw
                }
            }
            raw! {
                pub fn as_raw_mut(&mut self) -> &mut $Raw {
                    &mut self.raw
                }
            }
        }

        impl std::fmt::Debug for $Name {
            #[allow(deprecated, unreachable_patterns)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $($Name::$Variant => f.debug_struct(stringify!($Variant)).finish(),)*
                    _ => f.debug_struct(stringify!($Name)).field("raw", &self.raw).finish(),
                }
            }
        }

        fmod_typedef_default! {$Name {$(
            $(#[$($vmeta)*])*
            $Variant = $value,
        )*}}
    };
}

macro_rules! fmod_typedef_default {
    {$Name:ident {}} => {};

    {$Name:ident {
        #[default]
        $(#[$meta:meta])*
        $Variant:ident = $value:expr,
        $(
            $(#[$($vmeta:tt)*])*
            $VVariant:ident = $vvalue:expr,
        )*
    }} => {
        #[doc = concat!("[`", stringify!($Name), "::", stringify!($Variant), "`]")]
        impl Default for $Name {
            fn default() -> $Name {
                $Name::$Variant
            }
        }
        fmod_typedef_default! { $Name { $($(#[$($vmeta)*])* $VVariant = $vvalue,)* } }
    };

    {$Name:ident {
        $(#[$meta:meta])*
        $Variant:ident = $value:expr,
        $(
            $(#[$($vmeta:tt)*])*
            $VVariant:ident = $vvalue:expr,
        )*
    }} => {
        fmod_typedef_default! { $Name { $($(#[$($vmeta)*])* $VVariant = $vvalue,)* } }
    };
}

#[rustfmt::skip]
macro_rules! strip_default_attr {
    {#[default] $($tt:tt)*} => { $($tt)* };
    {           $($tt:tt)*} => { $($tt)* };
}
