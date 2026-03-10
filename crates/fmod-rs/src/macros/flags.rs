macro_rules! fmod_flags_ops {
    ($Name:ty: $($Op:ident)::+ $fn_op:ident $op:tt) => {
        #[allow(deprecated)]
        impl $($Op)::+ for $Name {
            type Output = $Name;
            fn $fn_op(self) -> $Name {
                let raw = $op <$Name>::into_raw(self);
                <$Name>::from_raw(raw)
            }
        }

        #[allow(deprecated)]
        impl $($Op)::+ for &'_ $Name {
            type Output = $Name;
            fn $fn_op(self) -> $Name {
                $op *self
            }
        }
    };
    ($Name:ty: $($Op:ident)::+ $fn_op:ident $op:tt $($OpAssign:ident)::+ $fn_op_assign:ident) => {
        #[allow(deprecated)]
        impl $($Op)::+ for $Name {
            type Output = $Name;
            fn $fn_op(self, rhs: $Name) -> $Name {
                let raw = <$Name>::into_raw(self) $op <$Name>::into_raw(rhs);
                <$Name>::from_raw(raw)
            }
        }

        #[allow(deprecated)]
        impl $($Op)::+<&$Name> for $Name {
            type Output = $Name;
            fn $fn_op(self, rhs: &$Name) -> $Name {
                self $op *rhs
            }
        }

        #[allow(deprecated)]
        impl $($Op)::+<$Name> for &$Name {
            type Output = $Name;
            fn $fn_op(self, rhs: $Name) -> $Name {
                *self $op rhs
            }
        }

        #[allow(deprecated)]
        impl $($Op)::+<&$Name> for &$Name {
            type Output = $Name;
            fn $fn_op(self, rhs: &$Name) -> $Name {
                *self $op *rhs
            }
        }

        #[allow(deprecated)]
        impl $($OpAssign)::+ for $Name {
            fn $fn_op_assign(&mut self, rhs: $Name) {
                *self = *self $op rhs;
            }
        }

        #[allow(deprecated)]
        impl $($OpAssign)::+<&$Name> for $Name {
            fn $fn_op_assign(&mut self, rhs: &$Name) {
                *self = *self $op *rhs;
            }
        }
    };
}

macro_rules! fmod_flags {
    {$(
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident: $Raw:ty {$(
            $(#[$($vmeta:tt)*])*
            $Variant:ident = $value:expr,
        )*}
    )*} => {$(
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(::zerocopy::FromBytes, ::zerocopy::IntoBytes)]
        #[derive(::zerocopy::KnownLayout, ::zerocopy::Immutable)]
        $vis struct $Name {
            raw: $Raw,
        }

        #[allow(deprecated)]
        impl $Name {
            $(
                fmod_flags! {@stripdefault
                    $(#[$($vmeta)*])*
                    #[allow(non_upper_case_globals)]
                    pub const $Variant: Self = Self::from_raw($value);
                }
            )*
        }

        #[allow(deprecated)]
        impl $Name {
            raw! {
                pub const fn zeroed() -> $Name {
                    Self::from_raw(0)
                }
            }
            raw! {
                pub const fn from_raw(raw: $Raw) -> $Name {
                    unsafe { ::std::mem::transmute(raw) }
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
                    unsafe { ::std::mem::transmute(self) }
                }
            }
            raw! {
                pub const fn as_raw(&self) -> &$Raw {
                    unsafe { &*(self as *const $Name as *const $Raw ) }
                }
            }
            raw! {
                pub fn as_raw_mut(&mut self) -> &mut $Raw {
                    unsafe { &mut *(self as *mut $Name as *mut $Raw ) }
                }
            }

            /// Check whether *all* flags of the argument are set.
            pub fn is_set(self, variant: Self) -> bool {
                (self & variant) == variant
            }
        }

        fmod_flags_ops!($Name: std::ops::BitAnd bitand & std::ops::BitAndAssign bitand_assign);
        fmod_flags_ops!($Name: std::ops::BitOr  bitor  | std::ops::BitOrAssign  bitor_assign );
        fmod_flags_ops!($Name: std::ops::BitXor bitxor ^ std::ops::BitXorAssign bitxor_assign);
        fmod_flags_ops!($Name: std::ops::Not    not    !);

        #[allow(deprecated)]
        impl ::std::fmt::Debug for $Name {
            #[allow(unreachable_patterns)]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $($Name::$Variant => f.debug_struct(stringify!($Variant)).finish(),)*
                    _ => f.debug_struct(stringify!($Name)).field("raw", &self.raw).finish(),
                }
            }
        }

        fmod_flags! {@default $Name {$(
            $(#[$($vmeta)*])*
            $Variant = $value,
        )*}}
    )*};

    {@default $Name:ident {}} => {};

    {@default $Name:ident {
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
        fmod_flags! { @default $Name { $($(#[$($vmeta)*])* $VVariant = $vvalue,)* } }
    };

    {@default $Name:ident {
        $(#[$meta:meta])*
        $Variant:ident = $value:expr,
        $(
            $(#[$($vmeta:tt)*])*
            $VVariant:ident = $vvalue:expr,
        )*
    }} => {
        fmod_flags! { @default $Name { $($(#[$($vmeta)*])* $VVariant = $vvalue,)* } }
    };

    {@stripdefault #[default] $($tt:tt)*} => { $($tt)* };
    {@stripdefault $($tt:tt)*} => { $($tt)* };
}
