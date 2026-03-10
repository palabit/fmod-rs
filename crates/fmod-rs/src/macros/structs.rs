macro_rules! fmod_struct {
    {
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident = $Raw:ident {
            $($body:tt)*
        }
    } => {
        fmod_struct! {
            #![fmod_no_default]
            $(#[$meta])*
            #[derive(::smart_default::SmartDefault)]
            $vis struct $Name = $Raw {
                $($body)*
            }
        }
    };
    {
        #![fmod_no_default]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident = $Raw:ident {
            $($body:tt)*
        }
    } => {
        fmod_struct! {
            #![fmod_no_pod, fmod_no_default]
            $(#[$meta])*
            #[derive(::zerocopy::FromBytes, ::zerocopy::IntoBytes)]
            #[derive(::zerocopy::Immutable)]
            $vis struct $Name = $Raw {
                $($body)*
            }
        }
    };
    {
        #![fmod_no_pod]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident$(<$lt:lifetime>)? = $Raw:ident {
            $($body:tt)*
        }
    } => {
        fmod_struct! {
            #![fmod_no_pod, fmod_no_default]
            $(#[$meta])*
            #[derive(::zerocopy::FromZeros)]
            #[derive(::smart_default::SmartDefault)]
            $vis struct $Name$(<$lt>)? = $Raw {
                $($body)*
            }
        }
    };
    {
        #![fmod_no_pod, fmod_no_default]
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident$(<$lt:lifetime>)? = $Raw:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident: $field_ty:ty $(= $raw_field:ident)?,
            )*
        }
    } => {
        #[repr(C)]
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[derive(::zerocopy::KnownLayout)]
        pub struct $Name$(<$lt>)? {
            $(
                $(#[$field_meta])*
                $field_vis $field: $field_ty,
            )*
        }

        const _: () = {
            assert!(::std::mem::size_of::<$Name>() == ::std::mem::size_of::<$Raw>());
            assert!(::std::mem::align_of::<$Name>() == ::std::mem::align_of::<$Raw>());
            $($(assert!(::std::mem::offset_of!($Name, $field) == ::std::mem::offset_of!($Raw, $raw_field));)?)*
        };

        impl$(<$lt>)? $Name$(<$lt>)? {
            raw! {
                pub const fn from_raw(raw: $Raw) -> $Name$(<$lt>)? {
                    unsafe { ::std::mem::transmute(raw) }
                }
            }
            raw! {
                pub const fn from_raw_ref(raw: &$Raw) -> &$Name$(<$lt>)? {
                    unsafe { &*(raw as *const $Raw as *const $Name$(<$lt>)? ) }
                }
            }
            raw! {
                pub fn from_raw_mut(raw: &mut $Raw) -> &mut $Name$(<$lt>)? {
                    unsafe { &mut *(raw as *mut $Raw as *mut $Name$(<$lt>)? ) }
                }
            }
            raw! {
                pub const fn into_raw(self) -> $Raw {
                    unsafe { ::std::mem::transmute(self) }
                }
            }
            raw! {
                pub const fn as_raw(&self) -> &$Raw {
                    unsafe { &*(self as *const $Name$(<$lt>)? as *const $Raw ) }
                }
            }
            raw! {
                pub fn as_raw_mut(&mut self) -> &mut $Raw {
                    unsafe { &mut *(self as *mut $Name$(<$lt>)? as *mut $Raw ) }
                }
            }
        }
    };
}
