macro_rules! fmod_enum {
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        $(where)?
        {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        ::paste::paste! {
            fmod_enum! {
                $(#[$meta])*
                $vis enum $Name: $Raw
                where
                    const { self < [<$Raw _MAX>] },
                    const { self >= 0 },
                {$(
                    $(#[$($vmeta)*])*
                    $Variant = $value,
                )*}
            }
        }
    };
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        where const { self <= $MAX:expr } $(,)?
        {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        fmod_enum! {
            $(#[$meta])*
            $vis enum $Name: $Raw
            where
                const { self < $MAX + 1 },
                const { self >= 0 },
            {$(
                $(#[$($vmeta)*])*
                $Variant = $value,
            )*}
        }
    };
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        where
            const { self < $MAX:expr } $(,)?
        {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        fmod_enum! {
            $(#[$meta])*
            $vis enum $Name: $Raw
            where
                const { self < $MAX },
                const { self >= 0 },
            {$(
                $(#[$($vmeta)*])*
                $Variant = $value,
            )*}
        }
    };
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        where
            const { self >= $MIN:expr } $(,)?
        {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        ::paste::paste! {
            fmod_enum! {
                $(#[$meta])*
                $vis enum $Name: $Raw
                where
                    const { self < [<$Raw _MAX>] },
                    const { self >= $MIN },
                {$(
                    $(#[$($vmeta)*])*
                    $Variant = $value,
                )*}
            }
        }
    };
    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        where
            const { self <= $MAX:expr },
            const { self >= $MIN:expr },
        {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        fmod_enum! {
            $(#[$meta])*
            $vis enum $Name: $Raw
            where
                const { self < $MAX + 1 },
                const { self >= $MIN },
            {$(
                $(#[$($vmeta)*])*
                $Variant = $value,
            )*}
        }
    };

    {
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident: $Raw:ty
        where
            const { self < $MAX:expr },
            const { self >= $MIN:expr },
        {$(
            $(#[*cfg($vcfg:meta)])?
            $(#[$vmeta:meta])*
            $Variant:ident = $value:expr,
        )*}
    } => {
        $(#[$meta])*
        #[repr(i32)]
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[derive(::zerocopy::TryFromBytes, ::zerocopy::IntoBytes)]
        #[derive(::zerocopy::KnownLayout, ::zerocopy::Immutable)]
        $vis enum $Name {
            $(
                $(#[cfg($vcfg)])?
                $(#[$vmeta])*
                $Variant = $value,
            )*
        }

        #[allow(clippy::manual_range_contains)]
        impl $Name {
            raw! {
                pub const fn zeroed() -> $Name {
                    unsafe { Self::from_raw(0) }
                }
            }
            raw! {
                pub const unsafe fn from_raw(raw: $Raw) -> $Name {
                    assert_unsafe_precondition!(
                        concat!("Raw `", stringify!($Name), "` value should be in ", stringify!($MIN), "..(", stringify!($MAX), ")"),
                        (raw: $Raw = raw) => $MIN <= raw && raw < $MAX,
                    );
                    unsafe { ::std::mem::transmute(raw) }
                }
            }
            raw! {
                pub const fn try_from_raw(raw: $Raw) -> Result<$Name> {
                    if $MIN <= raw && raw < $MAX {
                        Ok(unsafe { Self::from_raw(raw) })
                    } else {
                        Err(Error::InvalidParam)
                    }
                }
            }
            raw! {
                pub const unsafe fn from_raw_ref(raw: &$Raw) -> &$Name {
                    assert_unsafe_precondition!(
                        concat!("Raw `", stringify!($Name), "` value should be in ", stringify!($MIN), "..(", stringify!($MAX), ")"),
                        (raw: $Raw = *raw) => $MIN <= raw && raw < $MAX,
                    );
                    unsafe { &*(raw as *const $Raw as *const $Name ) }
                }
            }
            raw! {
                pub unsafe fn from_raw_mut(raw: &mut $Raw) -> &mut $Name {
                    assert_unsafe_precondition!(
                        concat!("Raw `", stringify!($Name), "` value should be in ", stringify!($MIN), "..(", stringify!($MAX), ")"),
                        (raw: $Raw = *raw) => $MIN <= raw && raw < $MAX,
                    );
                    unsafe { &mut *(raw as *mut $Raw as *mut $Name ) }
                }
            }
            raw! {
                pub const fn into_raw(self) -> $Raw {
                    unsafe { ::std::mem::transmute(self) }
                }
            }
            raw! {
                pub const fn as_raw(&self) -> &$Raw {
                    unsafe { &*(self as *const $Name as *const $Raw ) }
                }
            }
            raw! {
                pub unsafe fn as_raw_mut(&mut self) -> &mut $Raw {
                    unsafe { &mut *(self as *mut $Name as *mut $Raw ) }
                }
            }
        }

        $(
            $(#[cfg($vcfg)])?
            const _: () = assert!($MIN <= $Name::$Variant.into_raw() && $Name::$Variant.into_raw() < $MAX);
        )*

        const _: () = assert!($MIN <= 0 && 0 < $MAX);
        assert_type_eq!($Raw, i32);

        #[allow(deprecated)]
        const _: () = {
            const VARIANTS: &[$Name] = &[$( $(#[cfg($vcfg)])? $Name::$Variant, )*];
            const EXPECTED_VARIANT_COUNT: usize = ($MAX - $MIN) as usize;

            assert!(
                VARIANTS.len() >= EXPECTED_VARIANT_COUNT,
                concat!("fmod_enum! ", stringify!($Raw), " is missing some variant(s) in ", file!()),
            );
            assert!(
                VARIANTS.len() <= EXPECTED_VARIANT_COUNT,
                concat!("fmod_enum! ", stringify!($Raw), " has extraneous variant(s) in ", file!()),
            );
        };

        // SAFETY: zero is a valid value as proved by the contained assertion
        #[allow(trivial_bounds)]
        unsafe impl ::zerocopy::FromZeros for $Name
        where
            $Name: ::zerocopy::TryFromBytes,
        {
            fn only_derive_is_allowed_to_implement_this_trait() {
                const ZEROED: $Name = $Name::zeroed();
                const _: () = assert!(ZEROED as $Raw == 0);
            }
        }
    };
}
